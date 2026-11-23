//! Caller-owned reusable GPU workspace buffer for cuDNN operations.
//!
//! Workspaces grow on demand only when the requested byte count exceeds the existing allocation,
//! avoiding per-invocation device allocations in steady-state inference or training loops. Every
//! request is checked against an optional caller-supplied budget on the total workspace size, and
//! the old allocation is released (stream-ordered) before the larger one is allocated, so peak
//! device usage never exceeds the admitted size.
//!
//! The buffer is allocated stream-ordered on the bound stream and every crate operation that uses
//! it runs on that same stream, so its stream-ordered free is ordered after that work. A raw
//! pointer returned by [`Scratch::reserve`] must only be used by work on [`Scratch::stream`].

use std::sync::Arc;

use cuda_core::{Context, DeviceBuffer, Stream};

use crate::{Result, same_stream, within};

/// Reusable stream-ordered workspace buffer bound to a single [`Stream`].
pub struct Scratch {
    stream: Arc<Stream>,
    buf: Option<DeviceBuffer<u8>>,
}

impl Scratch {
    /// Creates an empty scratch workspace bound to `stream` with zero bytes allocated.
    pub fn new(stream: &Arc<Stream>) -> Self {
        Self {
            stream: stream.clone(),
            buf: None,
        }
    }

    /// Bound CUDA stream.
    pub fn stream(&self) -> &Arc<Stream> {
        &self.stream
    }

    /// Owning CUDA context.
    pub fn context(&self) -> &Arc<Context> {
        self.stream.context()
    }

    /// Current allocated capacity in bytes.
    pub fn capacity(&self) -> usize {
        self.buf.as_ref().map_or(0, DeviceBuffer::num_bytes)
    }

    /// Additional device bytes needed beyond [`Self::capacity`] to hold `needed` bytes.
    pub fn growth(&self, needed: usize) -> usize {
        needed.saturating_sub(self.capacity())
    }

    /// Ensures the workspace holds at least `needed` bytes, rejecting `needed > budget` before
    /// allocating.
    ///
    /// Returns `(device_ptr, available_bytes)`. When `needed == 0` and no buffer is allocated yet,
    /// returns `(0, 0)` with zero device allocations. The pointer stays valid until the next
    /// growing `reserve`, [`Self::clear`], or drop, and may only be used by work on
    /// [`Self::stream`].
    pub fn reserve(
        &mut self,
        stream: &Arc<Stream>,
        needed: usize,
        budget: Option<usize>,
    ) -> Result<(cuda_bindings::CUdeviceptr, usize)> {
        same_stream(stream, &self.stream)?;
        if needed == 0 {
            return Ok(self.as_raw_parts());
        }
        if let Some(bytes) = grow_to(self.capacity(), needed, budget)? {
            // Release the old buffer first so peak usage stays at `bytes`; its stream-ordered free
            // runs on `self.stream`, after all prior work that used it.
            self.buf = None;
            // SAFETY: cuDNN workspace memory is written by cuDNN kernels before any read.
            let fresh = unsafe { DeviceBuffer::<u8>::uninitialized_async(&self.stream, bytes) }?;
            self.buf = Some(fresh);
        }
        Ok(self.as_raw_parts())
    }

    /// Releases the underlying GPU buffer back to the CUDA allocator.
    pub fn clear(&mut self) {
        self.buf = None;
    }

    /// Raw device pointer and byte capacity, or `(0, 0)` when empty.
    pub(crate) fn as_raw_parts(&self) -> (cuda_bindings::CUdeviceptr, usize) {
        match self.buf.as_ref() {
            Some(buf) => (buf.cu_deviceptr(), buf.num_bytes()),
            None => (0, 0),
        }
    }
}

/// Byte size to allocate for a `needed`-byte request on a workspace of `capacity` bytes, or `None`
/// when the existing allocation suffices. `budget` bounds the total workspace size `needed`, not
/// the growth beyond `capacity`.
fn grow_to(capacity: usize, needed: usize, budget: Option<usize>) -> Result<Option<usize>> {
    within(budget, needed)?;
    Ok((needed > capacity).then_some(needed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    const MIB: usize = 1 << 20;

    #[test]
    fn t0_grow_to_budget_bounds_total_size() {
        // Growth 100 MiB fits a 200 MiB budget, but the workspace itself is 1000 MiB.
        let err = grow_to(900 * MIB, 1000 * MIB, Some(200 * MIB)).unwrap_err();
        assert!(matches!(
            err,
            Error::Budget { requested, available }
                if requested == 1000 * MIB && available == 200 * MIB
        ));
        // Reusing an existing larger buffer does not admit a request above the budget either.
        assert!(grow_to(1024, 512, Some(256)).is_err());

        assert_eq!(grow_to(256, 128, None).unwrap(), None);
        assert_eq!(grow_to(256, 512, None).unwrap(), Some(512));
        assert_eq!(grow_to(256, 512, Some(512)).unwrap(), Some(512));
        assert_eq!(grow_to(512, 512, Some(512)).unwrap(), None);
    }

    #[test]
    #[ignore = "requires CUDA GPU and cuDNN runtime"]
    fn t2_reserve_accepts_equal_default_stream() {
        let ctx = cuda_core::Context::new(0).unwrap();
        let mut scratch = Scratch::new(&ctx.default_stream());
        // A second `default_stream()` call is a different `Arc` for the same native stream.
        let (ptr, bytes) = scratch.reserve(&ctx.default_stream(), 256, None).unwrap();
        assert_ne!(ptr, 0);
        assert!(bytes >= 256);

        let other = ctx.new_stream().unwrap();
        assert!(matches!(
            scratch.reserve(&other, 256, None),
            Err(Error::Invalid(_))
        ));
    }
}
