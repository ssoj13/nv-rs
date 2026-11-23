//! [`Scratch`]: the one caller-owned, reusable, budget-bounded device workspace of nv-rs.
//!
//! **What.** One stream-ordered `DeviceBuffer<u8>` bound to one [`Stream`] that grows on demand
//! and never shrinks until [`Scratch::clear`] or drop.
//! **Why.** `cublas`, `cufft`, `cudnn`, `cusolver`, `cusparse` and `nvcomp` each had their own copy
//! with slightly different budget and stream rules (budget on the growth vs. the total, `Arc`
//! stream identity vs. native handle). One owner gives one set of rules.
//! **How.**
//! - The budget bounds the whole workspace: a request above it is rejected even when the current
//!   buffer is large enough ([`ScratchError::Budget`]).
//! - Growth releases the old buffer before allocating the new one; both are stream-ordered on the
//!   bound stream (`cuMemFreeAsync` / `cuMemAllocAsync`), so the free is ordered after every
//!   queued use, provided all use is on that stream. Because the old free may still be pending
//!   in the pool when the new allocation is made, [`Scratch::growth`] charges the full new size.
//! - Streams are compared with [`crate::identity::same_stream`] (native handle + context), not by
//!   `Arc`, so a second `Context::default_stream()` call is the same stream.
//! - The buffer is held in an `Arc` so a native handle that keeps the pointer beyond the call
//!   (cuBLAS `cublasSetWorkspace`, cuFFT `cufftSetWorkArea`) can retain it
//!   ([`Scratch::shared_buffer`]); [`Scratch::generation`] changes on every reallocation so such a
//!   holder can tell that it must rebind.
//!
//! A raw pointer returned by [`Scratch::reserve`] must only be used by work queued on
//! [`Scratch::stream`]; it stays valid until the next growing `reserve`, [`Scratch::clear`] or
//! drop (or longer, for a retained [`Scratch::shared_buffer`]).
//!
//! **Used by** `cublas`, `cudnn`, `cusolver`, `cusparse`, `nvcomp` (re-exported as their
//! `Scratch`) and `cufft` (shared between plans behind its own newtype).

use std::sync::Arc;

use cuda_core::{Context, DeviceBuffer, DriverError, Stream};

use crate::identity::same_stream;

/// Why [`Scratch::reserve`] refused a request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScratchError {
    /// The request names another stream than the one the workspace is bound to.
    WrongStream,
    /// The request is larger than the caller's budget.
    Budget {
        /// Bytes requested.
        requested: usize,
        /// Bytes the budget admits.
        available: usize,
    },
    /// The allocator returned (or holds) memory below the requested alignment.
    Misaligned {
        /// Alignment requested, in bytes.
        align: usize,
    },
    /// The stream-ordered allocation failed.
    Driver(DriverError),
}

impl std::fmt::Display for ScratchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScratchError::WrongStream => {
                f.write_str("scratch used on a different stream than it was created for")
            }
            ScratchError::Budget {
                requested,
                available,
            } => write!(
                f,
                "needs {requested} device bytes, budget admits {available}"
            ),
            ScratchError::Misaligned { align } => {
                write!(f, "scratch workspace is not aligned to {align} bytes")
            }
            ScratchError::Driver(e) => write!(f, "CUDA: {e}"),
        }
    }
}

impl std::error::Error for ScratchError {}

impl From<DriverError> for ScratchError {
    fn from(e: DriverError) -> Self {
        ScratchError::Driver(e)
    }
}

/// Reusable stream-ordered device workspace bound to a single [`Stream`] (see the module docs).
pub struct Scratch {
    stream: Arc<Stream>,
    buf: Option<Arc<DeviceBuffer<u8>>>,
    generation: u64,
}

impl Scratch {
    /// Creates an empty workspace for work on `stream`; nothing is allocated.
    pub fn new(stream: &Arc<Stream>) -> Self {
        Self {
            stream: stream.clone(),
            buf: None,
            generation: 0,
        }
    }

    /// Creates a workspace pre-allocated to `bytes`, admitted against `budget`.
    pub fn with_capacity(
        stream: &Arc<Stream>,
        bytes: usize,
        budget: Option<usize>,
    ) -> Result<Self, ScratchError> {
        let mut scratch = Self::new(stream);
        scratch.reserve(stream, bytes, budget)?;
        Ok(scratch)
    }

    /// The bound stream.
    pub fn stream(&self) -> &Arc<Stream> {
        &self.stream
    }

    /// The bound stream's context.
    pub fn context(&self) -> &Arc<Context> {
        self.stream.context()
    }

    /// Bytes currently allocated.
    pub fn capacity(&self) -> usize {
        self.buf.as_ref().map_or(0, |b| b.num_bytes())
    }

    /// Changes on every reallocation (starts at `0`, before the first allocation).
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Device bytes a [`Self::reserve`] of `needed` bytes allocates: all of `needed` when the
    /// workspace must grow, else `0`. The full size is charged because the old buffer's
    /// stream-ordered free may still be pending when the new one is allocated.
    pub fn growth(&self, needed: usize) -> usize {
        growth_bytes(self.capacity(), needed)
    }

    /// Ensures the workspace holds at least `needed` bytes for work on `stream`, rejecting
    /// `needed > budget` before allocating.
    ///
    /// Returns `(device_ptr, capacity)`; `(0, 0)` while nothing is allocated (a `needed == 0`
    /// request never allocates).
    pub fn reserve(
        &mut self,
        stream: &Stream,
        needed: usize,
        budget: Option<usize>,
    ) -> Result<(cuda_core::sys::CUdeviceptr, usize), ScratchError> {
        self.reserve_aligned(stream, needed, 1, budget)
    }

    /// [`Self::reserve`] that also requires the pointer to be a multiple of `align` bytes
    /// (`0` and `1` mean no requirement).
    pub fn reserve_aligned(
        &mut self,
        stream: &Stream,
        needed: usize,
        align: usize,
        budget: Option<usize>,
    ) -> Result<(cuda_core::sys::CUdeviceptr, usize), ScratchError> {
        if !same_stream(stream, &self.stream) {
            return Err(ScratchError::WrongStream);
        }
        if let Some(available) = budget
            && needed > available
        {
            return Err(ScratchError::Budget {
                requested: needed,
                available,
            });
        }
        if needed > self.capacity() {
            // Release the old buffer first so peak usage stays at `needed`; its stream-ordered
            // free runs on `self.stream`, after all prior work that used it.
            self.buf = None;
            // SAFETY: workspace memory is written by the native library before any read, and only
            // by work on `self.stream` (documented contract of `reserve`).
            let fresh = unsafe { DeviceBuffer::<u8>::uninitialized_async(&self.stream, needed) }?;
            self.buf = Some(Arc::new(fresh));
            self.generation = self.generation.wrapping_add(1);
        }
        let (ptr, cap) = self.as_raw_parts();
        let req = u64::try_from(align.max(1)).map_err(|_| ScratchError::Misaligned { align })?;
        if cap > 0 && !ptr.is_multiple_of(req) {
            return Err(ScratchError::Misaligned { align });
        }
        Ok((ptr, cap))
    }

    /// Releases the buffer (stream-ordered) back to the CUDA allocator.
    pub fn clear(&mut self) {
        if self.buf.take().is_some() {
            self.generation = self.generation.wrapping_add(1);
        }
    }

    /// Raw device pointer and byte capacity, `(0, 0)` while empty.
    pub fn as_raw_parts(&self) -> (cuda_core::sys::CUdeviceptr, usize) {
        match self.buf.as_ref() {
            Some(buf) => (buf.cu_deviceptr(), buf.num_bytes()),
            None => (0, 0),
        }
    }

    /// Shared reference to the current buffer, for a native handle that keeps its pointer beyond
    /// the call that reserved it: retaining it keeps the memory allocated across a later growth or
    /// drop of this `Scratch`.
    pub fn shared_buffer(&self) -> Option<Arc<DeviceBuffer<u8>>> {
        self.buf.clone()
    }
}

impl std::fmt::Debug for Scratch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scratch")
            .field("capacity", &self.capacity())
            .field("generation", &self.generation)
            .field("stream", &self.stream)
            .finish()
    }
}

/// Bytes a workspace of `capacity` bytes allocates to hold `needed` bytes.
const fn growth_bytes(capacity: usize, needed: usize) -> usize {
    if needed > capacity { needed } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn growth_charges_the_full_new_allocation() {
        assert_eq!(growth_bytes(600, 900), 900);
        assert_eq!(growth_bytes(900, 900), 0);
        assert_eq!(growth_bytes(900, 600), 0);
        assert_eq!(growth_bytes(0, 1), 1);
        assert_eq!(growth_bytes(0, 0), 0);
    }

    #[test]
    #[ignore = "GPU: needs a CUDA device"]
    fn reserve_budget_stream_and_generation() {
        let ctx = Context::new(0).expect("context");
        let mut scratch = Scratch::new(&ctx.default_stream());
        assert_eq!(scratch.reserve(&ctx.default_stream(), 0, Some(0)), Ok((0, 0)));
        assert_eq!(scratch.generation(), 0);
        // A second `default_stream()` is a different `Arc` for the same native stream.
        let (ptr, cap) = scratch
            .reserve(&ctx.default_stream(), 1024, Some(1024))
            .expect("reserve");
        assert_ne!(ptr, 0);
        assert_eq!(cap, 1024);
        assert_eq!(scratch.generation(), 1);
        // The budget bounds the request even when the buffer is already large enough.
        assert_eq!(
            scratch.reserve(&ctx.default_stream(), 512, Some(256)),
            Err(ScratchError::Budget {
                requested: 512,
                available: 256
            })
        );
        assert_eq!(scratch.reserve(&ctx.default_stream(), 512, None), Ok((ptr, 1024)));
        assert_eq!(scratch.generation(), 1, "reuse does not reallocate");
        let held = scratch.shared_buffer().expect("buffer");
        scratch
            .reserve(&ctx.default_stream(), 4096, None)
            .expect("grow");
        assert_eq!(scratch.generation(), 2);
        assert_eq!(held.num_bytes(), 1024, "a retained buffer survives growth");
        let other = ctx.new_stream().expect("stream");
        assert_eq!(
            scratch.reserve(&other, 16, None),
            Err(ScratchError::WrongStream)
        );
        scratch.clear();
        assert_eq!(scratch.capacity(), 0);
        assert_eq!(scratch.generation(), 3);
        ctx.default_stream().synchronize().expect("sync");
    }
}
