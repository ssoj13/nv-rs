//! Pre-allocated host/device pointer/size tables (`BatchStaging`) around the shared temporary
//! workspace ([`Scratch`], `cuda_core_ext::Scratch`) for zero-allocation steady-state nvCOMP
//! batched launches.
//!
//! `BatchStaging` uses a CUDA event (`dma_fence`) recorded after the pinned host table uploads so
//! consecutive async launches on a stream never race over pinned staging memory, while the host
//! only waits for those uploads, never for the previous kernel. Device tables and scratch are
//! allocated stream-ordered on the bound stream and every launch, readback, and release uses that
//! same stream, so reuse and stream-ordered frees are ordered after queued kernels without a host
//! wait. Each batch takes a fresh generation number so a receipt can only collect the results of
//! the latest batch launched on its staging.
//!
//! The scratch growth charged by [`BatchStaging::growth`] is the full new workspace size
//! ([`Scratch::growth`]): the old buffer's stream-ordered free may still be pending. The shared
//! `Scratch` also bounds `temp_bytes` itself by the budget, even when its buffer is reused.

use std::ffi::c_void;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use cuda_core::{Context, DeviceBuffer, DeviceCopy, Event, PinnedHostBuffer, Stream};
use nvcomp_sys as sys;

use crate::{Error, Result, Scratch, same_stream, within};

// C-ABI pointer and status arrays copied verbatim to/from device memory.
const _: () = assert!(size_of::<u64>() == size_of::<*const c_void>());
const _: () = assert!(size_of::<u32>() == size_of::<sys::nvcompStatus_t>());

/// Source of batch generation numbers, unique across every [`BatchStaging`] in the process.
static NEXT_BATCH_GENERATION: AtomicU64 = AtomicU64::new(1);

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
    generation: u64,
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
            generation: 0,
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

    /// Generation of the latest batch prepared on this staging; receipts compare against it.
    pub(crate) fn generation(&self) -> u64 {
        self.generation
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

    /// Marks that the pinned host tables are currently referenced by async HtoD uploads on `stream`.
    /// Recorded right after the uploads, so [`Self::sync_host_tables`] waits for the uploads only.
    pub(crate) fn arm_dma_fence(&mut self, stream: &Stream) -> Result<()> {
        self.dma_fence.record(stream)?;
        self.fence_armed = true;
        Ok(())
    }

    /// Ensures staging tables hold at least `chunks` entries and scratch workspace holds
    /// `temp_bytes` aligned to `temp_align`, enforcing `budget` against combined device growth
    /// before any allocation occurs. Starts a new batch generation, which invalidates receipts of
    /// earlier batches because their result tables may be overwritten or reallocated from here on.
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
        self.generation = NEXT_BATCH_GENERATION.fetch_add(1, Ordering::Relaxed);

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
        // `budget` was already admitted above against the combined growth of tables and scratch
        // (`Scratch::growth` charges the full new size); checking it again here would also reject
        // reuse of an existing workspace that costs no new memory.
        self.scratch
            .reserve_aligned(stream, temp_bytes, temp_align, None)?;
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
