//! Caller-owned reusable GPU workspace buffer for cuDNN operations.
//!
//! Workspaces grow on demand only when the requested byte count exceeds the existing allocation,
//! avoiding per-invocation device allocations in steady-state inference or training loops. Every
//! growth request is checked against an optional caller-supplied byte budget before allocation.

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

    /// Ensures the workspace holds at least `needed` bytes, checking `budget` against the
    /// required growth before allocating.
    ///
    /// Returns `(device_ptr, available_bytes)`. When `needed == 0` and no buffer is allocated yet,
    /// returns `(0, 0)` with zero device allocations.
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
        let grow = self.growth(needed);
        if grow > 0 {
            within(budget, grow)?;
            // SAFETY: cuDNN workspace memory is written by cuDNN kernels before any read.
            let fresh = unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, needed) }?;
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

#[cfg(test)]
mod tests {
    #[test]
    fn t0_growth_math() {
        // Verify pure growth math invariant without touching a GPU context.
        let cap = 256usize;
        assert_eq!(128usize.saturating_sub(cap), 0);
        assert_eq!(512usize.saturating_sub(cap), 256);
    }
}
