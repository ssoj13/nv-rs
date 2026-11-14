//! Reusable GPU workspace (`Scratch`) and pre-allocated host/device pointer/size tables
//! (`BatchStaging`) for zero-allocation steady-state nvCOMP batched launches.
//!
//! `BatchStaging` uses a CUDA event (`dma_fence`) around pinned host table copies so
//! consecutive async launches on a stream never race over pinned staging memory and never stall
//! unnecessarily when previous work has already finished.

use std::ffi::c_void;
use std::sync::Arc;

use cuda_core::{Context, DeviceBuffer, DeviceCopy, Event, PinnedHostBuffer, Stream};
use nvcomp_sys as sys;

use crate::{Error, Result, same_stream, within};

// C-ABI pointer and status arrays copied verbatim to/from device memory.
const _: () = assert!(size_of::<u64>() == size_of::<*const c_void>());
const _: () = assert!(size_of::<u32>() == size_of::<sys::nvcompStatus_t>());

/// Reusable stream-ordered temporary workspace buffer for nvCOMP kernels, bound to a single
/// [`Stream`].
pub struct Scratch {
    stream: Arc<Stream>,
    buf: Option<DeviceBuffer<u8>>,
}

impl Scratch {
    /// Creates an empty scratch workspace bound to `stream`.
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

    /// Additional bytes needed beyond [`Self::capacity`] to fit `needed` bytes.
    pub fn growth(&self, needed: usize) -> usize {
        needed.saturating_sub(self.capacity())
    }

    /// Ensures the workspace holds at least `needed` bytes and satisfies `align` byte alignment,
    /// enforcing `budget` against any allocation growth.
    pub fn reserve(
        &mut self,
        stream: &Arc<Stream>,
        needed: usize,
        align: usize,
        budget: Option<usize>,
    ) -> Result<(cuda_bindings::CUdeviceptr, usize)> {
        same_stream(stream, &self.stream)?;
        if needed == 0 {
            return Ok(self.as_raw_parts());
        }
        let grow = self.growth(needed);
        if grow > 0 {
            within(budget, grow)?;
            // SAFETY: nvCOMP temporary workspace is overwritten by device kernels before read.
            let fresh = unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, needed) }?;
            let req_align = u64::try_from(align.max(1)).unwrap_or(u64::MAX);
            if !fresh.cu_deviceptr().is_multiple_of(req_align) {
                return Err(Error::Invalid(
                    "CUDA allocator returned workspace below nvCOMP temp alignment",
                ));
            }
            self.buf = Some(fresh);
        } else if let Some(buf) = &self.buf {
            let req_align = u64::try_from(align.max(1)).unwrap_or(u64::MAX);
            if !buf.cu_deviceptr().is_multiple_of(req_align) {
                return Err(Error::Invalid(
                    "cached workspace below nvCOMP temp alignment",
                ));
            }
        }
        Ok(self.as_raw_parts())
    }

    /// Releases the GPU workspace buffer.
    pub fn clear(&mut self) {
        self.buf = None;
    }

    pub(crate) fn as_raw_parts(&self) -> (cuda_bindings::CUdeviceptr, usize) {
        match self.buf.as_ref() {
            Some(buf) => (buf.cu_deviceptr(), buf.num_bytes()),
            None => (0, 0),
        }
    }
}

pub(crate) struct TableSet {
    pub(crate) host_in_ptrs: PinnedHostBuffer<u64>,
    pub(crate) host_out_ptrs: PinnedHostBuffer<u64>,
    pub(crate) host_in_bytes: PinnedHostBuffer<usize>,
    pub(crate) host_out_bytes: PinnedHostBuffer<usize>,
    pub(crate) host_buf_bytes: PinnedHostBuffer<usize>,
    pub(crate) host_statuses: PinnedHostBuffer<u32>,
    pub(crate) dev_in_ptrs: DeviceBuffer<u64>,
    pub(crate) dev_out_ptrs: DeviceBuffer<u64>,
    pub(crate) dev_in_bytes: DeviceBuffer<usize>,
    pub(crate) dev_out_bytes: DeviceBuffer<usize>,
    pub(crate) dev_buf_bytes: DeviceBuffer<usize>,
    pub(crate) dev_statuses: DeviceBuffer<u32>,
}

impl TableSet {
    fn allocate(ctx: &Arc<Context>, stream: &Arc<Stream>, capacity: usize) -> Result<Self> {
        let cap = capacity.max(1);
        // SAFETY: every device staging array is written via async HtoD memcpy or nvCOMP kernels
        // before any consumer reads from it.
        unsafe {
            Ok(Self {
                host_in_ptrs: PinnedHostBuffer::zeroed(ctx, cap)?,
                host_out_ptrs: PinnedHostBuffer::zeroed(ctx, cap)?,
                host_in_bytes: PinnedHostBuffer::zeroed(ctx, cap)?,
                host_out_bytes: PinnedHostBuffer::zeroed(ctx, cap)?,
                host_buf_bytes: PinnedHostBuffer::zeroed(ctx, cap)?,
                host_statuses: PinnedHostBuffer::zeroed(ctx, cap)?,
                dev_in_ptrs: DeviceBuffer::uninitialized_async(stream, cap)?,
                dev_out_ptrs: DeviceBuffer::uninitialized_async(stream, cap)?,
                dev_in_bytes: DeviceBuffer::uninitialized_async(stream, cap)?,
                dev_out_bytes: DeviceBuffer::uninitialized_async(stream, cap)?,
                dev_buf_bytes: DeviceBuffer::uninitialized_async(stream, cap)?,
                dev_statuses: DeviceBuffer::uninitialized_async(stream, cap)?,
            })
        }
    }

    fn device_bytes_for(capacity: usize) -> usize {
        let cap = capacity.max(1);
        cap.saturating_mul(2 * size_of::<u64>() + 3 * size_of::<usize>() + size_of::<u32>())
    }
}

/// Reusable staging workspace combining device pointer/size/status tables, pinned host staging,
/// and the temporary [`Scratch`] buffer for zero-allocation steady-state batched compression on a
/// single bound [`Stream`].
pub struct BatchStaging {
    ctx: Arc<Context>,
    stream: Arc<Stream>,
    capacity: usize,
    tables: Option<TableSet>,
    scratch: Scratch,
    dma_fence: Event,
    fence_armed: bool,
}

impl BatchStaging {
    /// Creates an empty `BatchStaging` instance bound to `stream` with zero device memory allocated.
    pub fn new(stream: &Arc<Stream>) -> Result<Self> {
        let ctx = stream.context();
        let dma_fence = ctx.new_event_with_flags(None)?;
        Ok(Self {
            ctx: ctx.clone(),
            stream: stream.clone(),
            capacity: 0,
            tables: None,
            scratch: Scratch::new(stream),
            dma_fence,
            fence_armed: false,
        })
    }

    /// Owning CUDA context.
    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }

    /// Bound CUDA stream.
    pub fn stream(&self) -> &Arc<Stream> {
        &self.stream
    }

    /// Number of chunks this staging area can currently hold without reallocating.
    pub fn chunk_capacity(&self) -> usize {
        self.capacity
    }

    /// Current temporary workspace capacity in bytes.
    pub fn temp_capacity(&self) -> usize {
        self.scratch.capacity()
    }

    /// Total additional device bytes required to grow both staging tables (to `chunks`) and
    /// temporary workspace (to `temp_bytes`).
    pub fn growth(&self, chunks: usize, temp_bytes: usize) -> usize {
        let table_grow = if chunks > self.capacity {
            TableSet::device_bytes_for(chunks)
        } else {
            0
        };
        table_grow.saturating_add(self.scratch.growth(temp_bytes))
    }

    /// Waits for any previous table DMA using `dma_fence` to complete so pinned host buffers can
    /// be safely mutated without overwriting an in-flight transfer.
    pub(crate) fn sync_host_tables(&mut self) -> Result<()> {
        if self.fence_armed {
            self.dma_fence.synchronize()?;
            self.fence_armed = false;
        }
        Ok(())
    }

    /// Marks that the pinned host tables are currently referenced by async DMA on `stream`.
    pub(crate) fn arm_dma_fence(&mut self, stream: &Stream) -> Result<()> {
        self.dma_fence.record(stream)?;
        self.fence_armed = true;
        Ok(())
    }

    /// Ensures staging tables hold at least `chunks` entries and scratch workspace holds
    /// `temp_bytes` aligned to `temp_align`, enforcing `budget` against combined device growth
    /// before any allocation occurs.
    pub(crate) fn prepare(
        &mut self,
        stream: &Arc<Stream>,
        chunks: usize,
        temp_bytes: usize,
        temp_align: usize,
        budget: Option<usize>,
    ) -> Result<()> {
        same_stream(stream, &self.stream)?;
        self.sync_host_tables()?;
        let total_grow = self.growth(chunks, temp_bytes);
        within(budget, total_grow)?;

        if chunks > self.capacity {
            let cap = chunks.max(self.capacity.saturating_mul(2)).max(16);
            // Re-check doubled capacity against budget; shrink to exact `chunks` if budget is tight.
            let target_cap = if let Some(avail) = budget {
                if TableSet::device_bytes_for(cap).saturating_add(self.scratch.growth(temp_bytes))
                    > avail
                {
                    chunks
                } else {
                    cap
                }
            } else {
                cap
            };
            self.tables = None;
            self.capacity = 0;
            self.tables = Some(TableSet::allocate(&self.ctx, stream, target_cap)?);
            self.capacity = target_cap;
        }
        let _ = self
            .scratch
            .reserve(stream, temp_bytes, temp_align, budget)?;
        Ok(())
    }

    pub(crate) fn tables_mut(&mut self) -> Result<(&mut TableSet, &Scratch)> {
        let tables = self
            .tables
            .as_mut()
            .ok_or(Error::Invalid("batch staging has not been prepared"))?;
        Ok((tables, &self.scratch))
    }

    /// Copies the first `count` items from a pinned host table to its paired device buffer on `stream`.
    pub(crate) fn upload_prefix<T: DeviceCopy>(
        host: &PinnedHostBuffer<T>,
        dev: &mut DeviceBuffer<T>,
        count: usize,
        stream: &Stream,
    ) -> Result<()> {
        if count == 0 {
            return Ok(());
        }
        let bytes = count.saturating_mul(size_of::<T>());
        // SAFETY: `host` holds at least `count` initialized elements, `dev` has `>= count` capacity,
        // and `BatchStaging::arm_dma_fence` keeps `host` immutable until DMA completes on `stream`.
        unsafe {
            cuda_core::memory::memcpy_htod_async(
                dev.cu_deviceptr(),
                host.as_ptr(),
                bytes,
                stream.cu_stream(),
            )?;
        }
        Ok(())
    }

    /// Copies the first `count` items from a device table into the paired pinned host table on `stream`.
    pub(crate) fn download_prefix<T: DeviceCopy>(
        host: &mut PinnedHostBuffer<T>,
        dev: &DeviceBuffer<T>,
        count: usize,
        stream: &Stream,
    ) -> Result<()> {
        if count == 0 {
            return Ok(());
        }
        let bytes = count.saturating_mul(size_of::<T>());
        // SAFETY: `dev` holds `>= count` elements, `host` has `>= count` capacity, and caller
        // synchronizes `stream` before reading from `host`.
        unsafe {
            cuda_core::memory::memcpy_dtoh_async(
                host.as_mut_ptr(),
                dev.cu_deviceptr(),
                bytes,
                stream.cu_stream(),
            )?;
        }
        Ok(())
    }
}

impl Drop for BatchStaging {
    fn drop(&mut self) {
        if self.fence_armed {
            // Prevent freeing pinned host buffers while an async HtoD table copy is active.
            if let Err(err) = self.dma_fence.synchronize() {
                self.ctx.record_err(Err::<(), _>(err));
            }
            self.fence_armed = false;
        }
    }
}

pub(crate) use TableSet as StagingTables;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_table_device_byte_accounting() {
        let single = TableSet::device_bytes_for(1);
        assert_eq!(single, 2 * 8 + 3 * size_of::<usize>() + 4);
        assert!(TableSet::device_bytes_for(64) >= 64 * single);
    }
}
