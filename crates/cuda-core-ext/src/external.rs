//! Memory and semaphores imported from another API (Vulkan, D3D12) or process.
//!
//! **What.** [`ExternalMemory::import`] adopts an exported allocation by OS handle; a byte range
//! of it becomes a `cuda_core::RawDeviceBuffer` ([`ExternalMemory::mapped_buffer`]) or a
//! mipmapped array ([`ExternalMemory::mapped_mipmapped_array`]). [`ExternalSemaphore`] imports a
//! binary or timeline semaphore / D3D12 fence and signals or waits on it from a CUDA stream.
//! **Why.** This is the CUDA half of zero-copy interop: wgpu (through gpu-info-rs) allocates and
//! exports, CUDA imports, a timeline semaphore orders the two queues (proven in nv-rs spike S2).
//! **Rules the driver sets, kept here once.** A Win32 handle is *not* consumed by the import (the
//! exporter still closes it); a POSIX fd *is* consumed. D3D12 resources must be imported as
//! dedicated. Mapped buffers are freed with `cuMemFree`, mapped mipmapped arrays with
//! `cuMipmappedArrayDestroy`, and both must go before the external memory object — the views
//! here hold an `Arc` of their [`ExternalMemory`] to guarantee that order.

use crate::array::{ArrayDesc, MipmappedArray};
use crate::{invalid_value, teardown};
use cuda_core::sys;
use cuda_core::{Context, DeviceAllocation, DriverError, IntoResult, RawDeviceBuffer, Stream};
use std::ffi::c_void;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// An exported memory object, by the handle kind its exporter produced.
#[derive(Clone, Copy, Debug)]
pub enum MemoryHandle {
    /// Vulkan `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT` (an NT handle; not consumed).
    OpaqueWin32(*mut c_void),
    /// Vulkan `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT` (consumed by a successful import).
    OpaqueFd(i32),
    /// An NT handle from `ID3D12Device::CreateSharedHandle` on an `ID3D12Resource`.
    D3d12Resource(*mut c_void),
    /// An NT handle from `ID3D12Device::CreateSharedHandle` on an `ID3D12Heap`.
    D3d12Heap(*mut c_void),
}

/// An imported external memory object.
pub struct ExternalMemory {
    raw: sys::CUexternalMemory,
    size: u64,
    ctx: Arc<Context>,
}

// SAFETY: a CUexternalMemory is a driver handle usable from any thread with its context current;
// this type only reads it.
unsafe impl Send for ExternalMemory {}
// SAFETY: as above.
unsafe impl Sync for ExternalMemory {}

impl ExternalMemory {
    /// Imports `size` bytes of an exported allocation. `dedicated` must match how the exporter
    /// allocated it (Vulkan dedicated allocation, or always for D3D12 resources).
    ///
    /// # Safety
    /// `handle` must refer to a live memory object of at least `size` bytes on `ctx`'s device,
    /// exported with the matching handle type.
    pub unsafe fn import(
        ctx: &Arc<Context>,
        handle: MemoryHandle,
        size: u64,
        dedicated: bool,
    ) -> Result<Arc<Self>, DriverError> {
        ctx.bind_to_thread()?;
        // SAFETY: an all-zero descriptor is the documented starting point; fields are set below.
        let mut desc: sys::CUDA_EXTERNAL_MEMORY_HANDLE_DESC = unsafe { std::mem::zeroed() };
        match handle {
            MemoryHandle::OpaqueWin32(h) => {
                desc.type_ = sys::CUexternalMemoryHandleType_enum_CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32;
                desc.handle.win32.handle = h;
            }
            MemoryHandle::OpaqueFd(fd) => {
                desc.type_ =
                    sys::CUexternalMemoryHandleType_enum_CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD;
                desc.handle.fd = fd;
            }
            MemoryHandle::D3d12Resource(h) => {
                if !dedicated {
                    return Err(invalid_value()); // the driver requires DEDICATED for D3D12 resources
                }
                desc.type_ = sys::CUexternalMemoryHandleType_enum_CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE;
                desc.handle.win32.handle = h;
            }
            MemoryHandle::D3d12Heap(h) => {
                desc.type_ =
                    sys::CUexternalMemoryHandleType_enum_CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP;
                desc.handle.win32.handle = h;
            }
        }
        desc.size = size;
        desc.flags = if dedicated {
            sys::CUDA_EXTERNAL_MEMORY_DEDICATED
        } else {
            0
        };
        let mut raw: sys::CUexternalMemory = std::ptr::null_mut();
        // SAFETY: valid out-pointer and descriptor; the caller vouches for the handle.
        unsafe { sys::cuImportExternalMemory(&mut raw, &desc) }.result()?;
        Ok(Arc::new(Self {
            raw,
            size,
            ctx: ctx.clone(),
        }))
    }

    /// The imported size in bytes.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// The raw driver handle.
    pub fn cu_external_memory(&self) -> sys::CUexternalMemory {
        self.raw
    }

    /// `bytes` bytes at `offset` as a device buffer. The buffer keeps this object alive.
    pub fn mapped_buffer(
        self: &Arc<Self>,
        offset: u64,
        bytes: u64,
    ) -> Result<RawDeviceBuffer, DriverError> {
        if bytes == 0 || offset.checked_add(bytes).is_none_or(|end| end > self.size) {
            return Err(invalid_value());
        }
        self.ctx.bind_to_thread()?;
        // SAFETY: zeroed descriptor, then the requested range.
        let mut desc: sys::CUDA_EXTERNAL_MEMORY_BUFFER_DESC = unsafe { std::mem::zeroed() };
        desc.offset = offset;
        desc.size = bytes;
        let mut ptr: sys::CUdeviceptr = 0;
        // SAFETY: the range was checked against the imported size; valid out-pointer.
        unsafe { sys::cuExternalMemoryGetMappedBuffer(&mut ptr, self.raw, &desc) }.result()?;
        let len = usize::try_from(bytes).map_err(|_| invalid_value())?;
        Ok(RawDeviceBuffer::foreign(
            Arc::new(MappedBuffer {
                ptr,
                len,
                memory: self.clone(),
            }),
            len,
        ))
    }

    /// The memory at `offset` as a mipmapped array of `levels` levels described by `desc`
    /// (the layout must match how the exporter created its image; set
    /// [`ArrayDesc::color_attachment`] when the image is a graphics colour target).
    pub fn mapped_mipmapped_array(
        self: &Arc<Self>,
        offset: u64,
        desc: &ArrayDesc,
        levels: u32,
    ) -> Result<MipmappedArray, DriverError> {
        desc.validate()?;
        desc.check_levels(levels)?;
        if offset >= self.size {
            return Err(invalid_value());
        }
        self.ctx.bind_to_thread()?;
        // SAFETY: zeroed descriptor, then the layout.
        let mut d: sys::CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC = unsafe { std::mem::zeroed() };
        d.offset = offset;
        d.arrayDesc = desc.to_sys();
        d.numLevels = levels;
        let mut raw: sys::CUmipmappedArray = std::ptr::null_mut();
        // SAFETY: valid descriptor and out-pointer.
        unsafe { sys::cuExternalMemoryGetMappedMipmappedArray(&mut raw, self.raw, &d) }.result()?;
        Ok(MipmappedArray::from_owned_raw(
            raw,
            *desc,
            levels,
            &self.ctx,
            Some(self.clone()),
        ))
    }
}

impl Drop for ExternalMemory {
    fn drop(&mut self) {
        let raw = self.raw;
        // SAFETY: every mapped view holds an Arc of this object, so none outlives it.
        teardown(&self.ctx, || unsafe { sys::cuDestroyExternalMemory(raw) });
    }
}

/// A mapped range of an [`ExternalMemory`], owned by the `RawDeviceBuffer` built from it.
struct MappedBuffer {
    ptr: sys::CUdeviceptr,
    len: usize,
    memory: Arc<ExternalMemory>,
}

// SAFETY: `ptr` addresses `len` bytes mapped from `memory`, which this struct keeps alive with its
// context; the pointer is freed only in `Drop`, after every view of it has been dropped.
unsafe impl DeviceAllocation for MappedBuffer {
    fn device_ptr(&self) -> sys::CUdeviceptr {
        self.ptr
    }
    fn len_bytes(&self) -> usize {
        self.len
    }
    fn context(&self) -> Arc<Context> {
        self.memory.ctx.clone()
    }
}

impl Drop for MappedBuffer {
    fn drop(&mut self) {
        let ptr = self.ptr;
        // SAFETY: the driver documents cuMemFree for buffers from cuExternalMemoryGetMappedBuffer;
        // no view of the buffer remains (this is its owner).
        teardown(&self.memory.ctx, || unsafe { sys::cuMemFree_v2(ptr) });
    }
}

/// An exported semaphore, by the handle kind its exporter produced.
#[derive(Clone, Copy, Debug)]
pub enum SemaphoreHandle {
    /// A Vulkan binary semaphore exported as an NT handle.
    OpaqueWin32(*mut c_void),
    /// A Vulkan binary semaphore exported as a POSIX fd (consumed by a successful import).
    OpaqueFd(i32),
    /// A Vulkan timeline semaphore exported as an NT handle.
    TimelineWin32(*mut c_void),
    /// A Vulkan timeline semaphore exported as a POSIX fd (consumed by a successful import).
    TimelineFd(i32),
    /// An NT handle from `ID3D12Device::CreateSharedHandle` on an `ID3D12Fence`.
    D3d12Fence(*mut c_void),
}

/// An imported semaphore, signalled and waited on from CUDA streams.
///
/// cuda.h (`cuDestroyExternalSemaphore`): "Any outstanding signals or waits must have completed
/// before the semaphore is destroyed." Once a signal or wait has been enqueued, dropping
/// therefore synchronizes the owning context first; a wait whose signal is never submitted
/// blocks that drop forever.
pub struct ExternalSemaphore {
    raw: sys::CUexternalSemaphore,
    /// Timeline semaphores and D3D12 fences carry a 64-bit value; binary semaphores do not.
    valued: bool,
    /// Set before the first signal or wait is enqueued; `Drop` then waits for the context.
    enqueued: AtomicBool,
    ctx: Arc<Context>,
}

// SAFETY: a CUexternalSemaphore is a driver handle usable from any thread with its context current.
unsafe impl Send for ExternalSemaphore {}
// SAFETY: as above.
unsafe impl Sync for ExternalSemaphore {}

impl ExternalSemaphore {
    /// Imports an exported semaphore.
    ///
    /// # Safety
    /// `handle` must refer to a live semaphore / fence of the stated kind on `ctx`'s device.
    pub unsafe fn import(
        ctx: &Arc<Context>,
        handle: SemaphoreHandle,
    ) -> Result<Arc<Self>, DriverError> {
        ctx.bind_to_thread()?;
        // SAFETY: zeroed descriptor, then the handle.
        let mut desc: sys::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC = unsafe { std::mem::zeroed() };
        let valued = match handle {
            SemaphoreHandle::OpaqueWin32(h) => {
                desc.type_ = sys::CUexternalSemaphoreHandleType_enum_CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32;
                desc.handle.win32.handle = h;
                false
            }
            SemaphoreHandle::OpaqueFd(fd) => {
                desc.type_ = sys::CUexternalSemaphoreHandleType_enum_CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD;
                desc.handle.fd = fd;
                false
            }
            SemaphoreHandle::TimelineWin32(h) => {
                desc.type_ = sys::CUexternalSemaphoreHandleType_enum_CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_WIN32;
                desc.handle.win32.handle = h;
                true
            }
            SemaphoreHandle::TimelineFd(fd) => {
                desc.type_ = sys::CUexternalSemaphoreHandleType_enum_CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_FD;
                desc.handle.fd = fd;
                true
            }
            SemaphoreHandle::D3d12Fence(h) => {
                desc.type_ = sys::CUexternalSemaphoreHandleType_enum_CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE;
                desc.handle.win32.handle = h;
                true
            }
        };
        let mut raw: sys::CUexternalSemaphore = std::ptr::null_mut();
        // SAFETY: valid descriptor and out-pointer; the caller vouches for the handle.
        unsafe { sys::cuImportExternalSemaphore(&mut raw, &desc) }.result()?;
        Ok(Arc::new(Self {
            raw,
            valued,
            enqueued: AtomicBool::new(false),
            ctx: ctx.clone(),
        }))
    }

    /// Rejects a stream of another context and records that work will reference the semaphore.
    fn prepare_enqueue(&self, stream: &Stream, value: u64) -> Result<(), DriverError> {
        if (!self.valued && value != 0) || stream.context().cu_ctx() != self.ctx.cu_ctx() {
            return Err(invalid_value());
        }
        self.enqueued.store(true, Ordering::Release);
        self.ctx.bind_to_thread()
    }

    /// Whether the semaphore carries a value (timeline / fence) rather than a binary state.
    pub fn is_valued(&self) -> bool {
        self.valued
    }

    /// Enqueues a signal on `stream` (a stream of this semaphore's context): sets a timeline /
    /// fence to `value`, or signals a binary semaphore (`value` must then be 0).
    pub fn signal(&self, stream: &Stream, value: u64) -> Result<(), DriverError> {
        self.prepare_enqueue(stream, value)?;
        // SAFETY: zeroed params, then the fence value.
        let mut p: sys::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS = unsafe { std::mem::zeroed() };
        p.params.fence.value = value;
        // SAFETY: one semaphore, one params entry, a stream of this context.
        unsafe { sys::cuSignalExternalSemaphoresAsync(&self.raw, &p, 1, stream.cu_stream()) }
            .result()
    }

    /// Enqueues a wait on `stream` (a stream of this semaphore's context) until a timeline /
    /// fence reaches `value`, or a binary semaphore is signalled (`value` must then be 0).
    pub fn wait(&self, stream: &Stream, value: u64) -> Result<(), DriverError> {
        self.prepare_enqueue(stream, value)?;
        // SAFETY: zeroed params, then the fence value.
        let mut p: sys::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS = unsafe { std::mem::zeroed() };
        p.params.fence.value = value;
        // SAFETY: one semaphore, one params entry, a stream of this context.
        unsafe { sys::cuWaitExternalSemaphoresAsync(&self.raw, &p, 1, stream.cu_stream()) }.result()
    }
}

impl Drop for ExternalSemaphore {
    fn drop(&mut self) {
        // Every signal and wait ran on a stream of `ctx` (checked when enqueued), so one context
        // synchronize retires all of them, as `cuDestroyExternalSemaphore` requires.
        if *self.enqueued.get_mut() {
            self.ctx.record_err(self.ctx.synchronize());
        }
        let raw = self.raw;
        // SAFETY: the semaphore is owned here, and every signal or wait enqueued through it has
        // completed (context synchronized above, or nothing was ever enqueued).
        teardown(&self.ctx, || unsafe {
            sys::cuDestroyExternalSemaphore(raw)
        });
    }
}
