//! Shared internal helpers for Vulkan/DX12 handles, usage mapping, and barriers.
//!
//! Why: `buffer`, `texture`, and `sync` all need identical Vulkan memory-type
//! selection, `wgpu::*Usages -> *Uses` conversion, Win32 `HANDLE` / POSIX `fd`
//! RAII ownership across CUDA import boundaries, and queue-family ownership
//! barriers. Centralizing them here removes duplication across modules.
//!
//! Used internally by `device`, `sync`, `buffer`, and `texture`.

#[cfg(unix)]
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::sync::{Arc, Mutex};

use ash::vk;
use cuda_core::Context;
use cuda_core_ext::{ExternalMemory, ExternalSemaphore, MemoryHandle, SemaphoreHandle};
#[cfg(windows)]
use wgpu::hal::api::Dx12;
use wgpu::hal::api::Vulkan;
#[cfg(windows)]
use windows::{
    Win32::{
        Foundation::{GENERIC_ALL, HANDLE},
        Graphics::Direct3D12,
    },
    core::PCWSTR,
};

use crate::error::{Error, Result};

/// Guarded OS handle produced when exporting Vulkan or DX12 external objects.
///
/// On Windows, `OwnedHandle` closes the duplicated NT `HANDLE` automatically on drop
/// (CUDA `cuImportExternalMemory` / `cuImportExternalSemaphore` does *not* consume
/// Win32 handles). On Unix, CUDA consumes the `fd` *only* if `cuImportExternal*`
/// succeeds, so [`into_cuda_memory`](Self::into_cuda_memory) /
/// [`into_cuda_semaphore`](Self::into_cuda_semaphore) leak the `OwnedFd` strictly
/// on `Ok` and let `OwnedFd::drop` close it on `Err`.
pub(crate) struct ExportedHandle {
    #[cfg(windows)]
    inner: OwnedHandle,
    #[cfg(unix)]
    inner: OwnedFd,
}

// SAFETY: NT kernel handles and POSIX file descriptors are kernel object references
// safe to transfer and drop across threads.
unsafe impl Send for ExportedHandle {}
// SAFETY: Shared read access only inspects the raw numeric handle/fd without mutation.
unsafe impl Sync for ExportedHandle {}

impl ExportedHandle {
    /// Wraps a freshly exported Windows handle (`vk::HANDLE` = `isize` or Win32 `HANDLE`).
    #[cfg(windows)]
    pub(crate) fn from_raw_win32(
        raw: *mut core::ffi::c_void,
        source_api: &'static str,
    ) -> Result<Self> {
        if raw.is_null() || raw == (-1isize as *mut core::ffi::c_void) {
            return Err(Error::InvalidHandle { source_api });
        }
        // SAFETY: `raw` was just returned non-null and non-INVALID_HANDLE_VALUE by
        // `source_api` (`vkGet*Win32HandleKHR` or `ID3D12Device::CreateSharedHandle`),
        // transferring a freshly allocated NT handle ownership to us.
        let inner = unsafe { OwnedHandle::from_raw_handle(raw) };
        Ok(Self { inner })
    }

    /// Wraps a freshly exported POSIX file descriptor (`vkGetMemoryFdKHR` / `vkGetSemaphoreFdKHR`).
    #[cfg(unix)]
    pub(crate) fn from_raw_fd(fd: i32, source_api: &'static str) -> Result<Self> {
        if fd < 0 {
            return Err(Error::InvalidHandle { source_api });
        }
        // SAFETY: `fd` is >= 0 and was freshly created for us by `source_api`.
        let inner = unsafe { OwnedFd::from_raw_fd(fd) };
        Ok(Self { inner })
    }

    /// Imports this exported handle into CUDA [`ExternalMemory`].
    ///
    /// On Unix this consumes `self` so that `OwnedFd` closes automatically if
    /// CUDA rejects the import and does *not* double-close when CUDA succeeds.
    pub(crate) fn import_cuda_memory(
        self,
        cuda_ctx: &Arc<Context>,
        kind: ExternalMemoryKind,
        size: u64,
        dedicated: bool,
    ) -> Result<Arc<ExternalMemory>> {
        #[cfg(windows)]
        {
            let raw = self.inner.as_raw_handle();
            let handle = match kind {
                ExternalMemoryKind::VulkanOpaque => MemoryHandle::OpaqueWin32(raw),
                ExternalMemoryKind::D3d12Resource => MemoryHandle::D3d12Resource(raw),
            };
            // SAFETY: `raw` is a live NT handle whose backing Vulkan/D3D12 allocation
            // matches `size` and `dedicated`, and `self` (`OwnedHandle`) keeps the handle valid.
            let mem = unsafe { ExternalMemory::import(cuda_ctx, handle, size, dedicated) }?;
            // Keep NT handle closed cleanly now that CUDA duplicated its own reference?
            // Note: keeping `OwnedHandle` only until import returns is valid for Vulkan/DX12,
            // and callers also keep the underlying VkDeviceMemory / ID3D12Resource alive.
            drop(self);
            Ok(mem)
        }
        #[cfg(unix)]
        {
            let _ = kind;
            let fd = self.inner.as_raw_fd();
            // SAFETY: `fd` is an open OPAQUE_FD for a Vulkan device memory allocation
            // of at least `size` bytes. CUDA takes ownership of `fd` only upon `Ok`.
            let res = unsafe {
                ExternalMemory::import(cuda_ctx, MemoryHandle::OpaqueFd(fd), size, dedicated)
            };
            match res {
                Ok(mem) => {
                    // CUDA now owns `fd`; prevent `OwnedFd::drop` from closing it twice.
                    std::mem::forget(self.inner);
                    Ok(mem)
                }
                Err(err) => Err(Error::Driver(err)),
            }
        }
    }

    /// Imports this exported handle into CUDA [`ExternalSemaphore`] as a valued
    /// timeline semaphore or DX12 fence.
    pub(crate) fn import_cuda_semaphore(
        self,
        cuda_ctx: &Arc<Context>,
        kind: ExternalSemaphoreKind,
    ) -> Result<Arc<ExternalSemaphore>> {
        #[cfg(windows)]
        {
            let raw = self.inner.as_raw_handle();
            let handle = match kind {
                ExternalSemaphoreKind::VulkanTimeline => SemaphoreHandle::TimelineWin32(raw),
                ExternalSemaphoreKind::D3d12Fence => SemaphoreHandle::D3d12Fence(raw),
            };
            // SAFETY: `raw` is a live NT handle for a timeline VkSemaphore or shared ID3D12Fence.
            let sem = unsafe { ExternalSemaphore::import(cuda_ctx, handle) }?;
            drop(self);
            Ok(sem)
        }
        #[cfg(unix)]
        {
            let _ = kind;
            let fd = self.inner.as_raw_fd();
            // SAFETY: `fd` is an open OPAQUE_FD for a Vulkan timeline semaphore.
            // CUDA takes ownership of `fd` only upon `Ok`.
            let res =
                unsafe { ExternalSemaphore::import(cuda_ctx, SemaphoreHandle::TimelineFd(fd)) };
            match res {
                Ok(sem) => {
                    std::mem::forget(self.inner);
                    Ok(sem)
                }
                Err(err) => Err(Error::Driver(err)),
            }
        }
    }
}

/// Discriminates the backing external memory type for CUDA import.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExternalMemoryKind {
    VulkanOpaque,
    #[cfg(windows)]
    D3d12Resource,
}

/// Discriminates the backing external semaphore type for CUDA import.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExternalSemaphoreKind {
    VulkanTimeline,
    #[cfg(windows)]
    D3d12Fence,
}

/// Platform-native Vulkan external memory handle bit.
#[inline]
pub(crate) const fn vk_external_memory_handle_type() -> vk::ExternalMemoryHandleTypeFlags {
    #[cfg(windows)]
    {
        vk::ExternalMemoryHandleTypeFlags::OPAQUE_WIN32
    }
    #[cfg(unix)]
    {
        vk::ExternalMemoryHandleTypeFlags::OPAQUE_FD
    }
}

/// Platform-native Vulkan external semaphore handle bit.
#[inline]
pub(crate) const fn vk_external_semaphore_handle_type() -> vk::ExternalSemaphoreHandleTypeFlags {
    #[cfg(windows)]
    {
        vk::ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32
    }
    #[cfg(unix)]
    {
        vk::ExternalSemaphoreHandleTypeFlags::OPAQUE_FD
    }
}

/// Exports a Vulkan `VkDeviceMemory` allocation as an OS-specific [`ExportedHandle`].
pub(crate) fn export_vk_memory(
    instance: &ash::Instance,
    device: &ash::Device,
    memory: vk::DeviceMemory,
) -> Result<ExportedHandle> {
    #[cfg(windows)]
    {
        let ext = ash::khr::external_memory_win32::Device::new(instance, device);
        let info = vk::MemoryGetWin32HandleInfoKHR::default()
            .memory(memory)
            .handle_type(vk::ExternalMemoryHandleTypeFlags::OPAQUE_WIN32);
        // SAFETY: `memory` was allocated with `ExportMemoryAllocateInfo` for `OPAQUE_WIN32`.
        let handle = unsafe { ext.get_memory_win32_handle(&info) }
            .map_err(|e| Error::vulkan("vkGetMemoryWin32HandleKHR", e))?;
        ExportedHandle::from_raw_win32(
            handle as *mut core::ffi::c_void,
            "vkGetMemoryWin32HandleKHR",
        )
    }
    #[cfg(unix)]
    {
        let ext = ash::khr::external_memory_fd::Device::new(instance, device);
        let info = vk::MemoryGetFdInfoKHR::default()
            .memory(memory)
            .handle_type(vk::ExternalMemoryHandleTypeFlags::OPAQUE_FD);
        // SAFETY: `memory` was allocated with `ExportMemoryAllocateInfo` for `OPAQUE_FD`.
        let fd = unsafe { ext.get_memory_fd(&info) }
            .map_err(|e| Error::vulkan("vkGetMemoryFdKHR", e))?;
        ExportedHandle::from_raw_fd(fd, "vkGetMemoryFdKHR")
    }
}

/// Exports a Vulkan `VkSemaphore` as an OS-specific [`ExportedHandle`].
pub(crate) fn export_vk_semaphore(
    instance: &ash::Instance,
    device: &ash::Device,
    semaphore: vk::Semaphore,
) -> Result<ExportedHandle> {
    #[cfg(windows)]
    {
        let ext = ash::khr::external_semaphore_win32::Device::new(instance, device);
        let info = vk::SemaphoreGetWin32HandleInfoKHR::default()
            .semaphore(semaphore)
            .handle_type(vk::ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32);
        // SAFETY: `semaphore` was created with `ExportSemaphoreCreateInfo` for `OPAQUE_WIN32`.
        let handle = unsafe { ext.get_semaphore_win32_handle(&info) }
            .map_err(|e| Error::vulkan("vkGetSemaphoreWin32HandleKHR", e))?;
        ExportedHandle::from_raw_win32(
            handle as *mut core::ffi::c_void,
            "vkGetSemaphoreWin32HandleKHR",
        )
    }
    #[cfg(unix)]
    {
        let ext = ash::khr::external_semaphore_fd::Device::new(instance, device);
        let info = vk::SemaphoreGetFdInfoKHR::default()
            .semaphore(semaphore)
            .handle_type(vk::ExternalSemaphoreHandleTypeFlags::OPAQUE_FD);
        // SAFETY: `semaphore` was created with `ExportSemaphoreCreateInfo` for `OPAQUE_FD`.
        let fd = unsafe { ext.get_semaphore_fd(&info) }
            .map_err(|e| Error::vulkan("vkGetSemaphoreFdKHR", e))?;
        ExportedHandle::from_raw_fd(fd, "vkGetSemaphoreFdKHR")
    }
}

/// Exports an NT shared handle from a Direct3D 12 resource or shared fence.
#[cfg(windows)]
pub(crate) fn export_d3d12_shared_handle<T>(
    raw_device: &Direct3D12::ID3D12Device,
    object: &T,
    call_name: &'static str,
) -> Result<ExportedHandle>
where
    T: windows::core::Interface,
{
    let child: Direct3D12::ID3D12DeviceChild =
        object.cast().map_err(|e| Error::dx12(call_name, e))?;
    // SAFETY: `child` belongs to `raw_device` and was created with a `SHARED` heap/fence flag.
    let handle: HANDLE =
        unsafe { raw_device.CreateSharedHandle(&child, None, GENERIC_ALL.0, PCWSTR::null()) }
            .map_err(|e| Error::dx12("ID3D12Device::CreateSharedHandle", e))?;
    ExportedHandle::from_raw_win32(handle.0, "ID3D12Device::CreateSharedHandle")
}

/// Finds a Vulkan memory type index satisfying `type_filter` and `required_flags`.
pub(crate) fn find_vk_memory_type_index(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    type_filter: u32,
    required_flags: vk::MemoryPropertyFlags,
) -> Result<u32> {
    // SAFETY: `physical_device` is a valid physical device owned by `instance`.
    let mem_props = unsafe { instance.get_physical_device_memory_properties(physical_device) };
    let count = (mem_props.memory_type_count as usize).min(mem_props.memory_types.len());
    for (idx, mem_type) in mem_props.memory_types[..count].iter().enumerate() {
        let bit = 1u32.checked_shl(idx as u32).unwrap_or(0);
        if (type_filter & bit) != 0 && mem_type.property_flags.contains(required_flags) {
            return Ok(idx as u32);
        }
    }
    Err(Error::Unsupported {
        what: "vulkan memory type",
        required: "DEVICE_LOCAL memory type compatible with external handle export",
        found: format!("type_filter=0x{type_filter:x}"),
    })
}

/// Translates [`wgpu::BufferUsages`] into Vulkan buffer usage flags.
pub(crate) fn map_vk_buffer_usage(usage: wgpu::BufferUsages) -> vk::BufferUsageFlags {
    let mut flags = vk::BufferUsageFlags::empty();
    if usage.contains(wgpu::BufferUsages::COPY_SRC) {
        flags |= vk::BufferUsageFlags::TRANSFER_SRC;
    }
    if usage.contains(wgpu::BufferUsages::COPY_DST) {
        flags |= vk::BufferUsageFlags::TRANSFER_DST;
    }
    if usage.contains(wgpu::BufferUsages::INDEX) {
        flags |= vk::BufferUsageFlags::INDEX_BUFFER;
    }
    if usage.contains(wgpu::BufferUsages::VERTEX) {
        flags |= vk::BufferUsageFlags::VERTEX_BUFFER;
    }
    if usage.contains(wgpu::BufferUsages::UNIFORM) {
        flags |= vk::BufferUsageFlags::UNIFORM_BUFFER;
    }
    if usage.contains(wgpu::BufferUsages::STORAGE) {
        flags |= vk::BufferUsageFlags::STORAGE_BUFFER;
    }
    if usage.contains(wgpu::BufferUsages::INDIRECT) {
        flags |= vk::BufferUsageFlags::INDIRECT_BUFFER;
    }
    flags
}

/// Translates [`wgpu::TextureUsages`] into [`wgpu::TextureUses`] for HAL wrapping.
pub(crate) fn map_hal_texture_uses(usage: wgpu::TextureUsages) -> wgpu::TextureUses {
    let mut uses = wgpu::TextureUses::empty();
    uses.set(
        wgpu::TextureUses::COPY_SRC,
        usage.contains(wgpu::TextureUsages::COPY_SRC),
    );
    uses.set(
        wgpu::TextureUses::COPY_DST,
        usage.contains(wgpu::TextureUsages::COPY_DST),
    );
    uses.set(
        wgpu::TextureUses::RESOURCE,
        usage.contains(wgpu::TextureUsages::TEXTURE_BINDING),
    );
    if usage.contains(wgpu::TextureUsages::STORAGE_BINDING) {
        uses |= wgpu::TextureUses::STORAGE_READ_ONLY
            | wgpu::TextureUses::STORAGE_WRITE_ONLY
            | wgpu::TextureUses::STORAGE_READ_WRITE;
    }
    uses.set(
        wgpu::TextureUses::STORAGE_ATOMIC,
        usage.contains(wgpu::TextureUsages::STORAGE_ATOMIC),
    );
    uses.set(
        wgpu::TextureUses::COLOR_TARGET,
        usage.contains(wgpu::TextureUsages::RENDER_ATTACHMENT),
    );
    uses
}

/// Translates [`wgpu::TextureUsages`] into Vulkan image usage flags.
pub(crate) fn map_vk_image_usage(usage: wgpu::TextureUsages) -> vk::ImageUsageFlags {
    let mut flags = vk::ImageUsageFlags::empty();
    if usage.contains(wgpu::TextureUsages::COPY_SRC) {
        flags |= vk::ImageUsageFlags::TRANSFER_SRC;
    }
    if usage.contains(wgpu::TextureUsages::COPY_DST) {
        flags |= vk::ImageUsageFlags::TRANSFER_DST;
    }
    if usage.contains(wgpu::TextureUsages::TEXTURE_BINDING) {
        flags |= vk::ImageUsageFlags::SAMPLED;
    }
    if usage.intersects(wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::STORAGE_ATOMIC)
    {
        flags |= vk::ImageUsageFlags::STORAGE;
    }
    if usage.contains(wgpu::TextureUsages::RENDER_ATTACHMENT) {
        flags |= vk::ImageUsageFlags::COLOR_ATTACHMENT;
    }
    flags
}

/// Maps a [`wgpu::TextureUses`] state to the [`vk::ImageLayout`] expected by `wgpu-hal`.
pub(crate) fn vk_layout_for_texture_uses(uses: wgpu::TextureUses) -> vk::ImageLayout {
    if uses == wgpu::TextureUses::UNINITIALIZED {
        vk::ImageLayout::UNDEFINED
    } else if uses == wgpu::TextureUses::COPY_SRC {
        vk::ImageLayout::TRANSFER_SRC_OPTIMAL
    } else if uses == wgpu::TextureUses::COPY_DST {
        vk::ImageLayout::TRANSFER_DST_OPTIMAL
    } else if uses == wgpu::TextureUses::RESOURCE {
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
    } else if uses == wgpu::TextureUses::COLOR_TARGET {
        vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL
    } else {
        vk::ImageLayout::GENERAL
    }
}

static VK_QUEUE_SUBMIT_LOCK: Mutex<()> = Mutex::new(());

/// Identifies the active `wgpu::Backend` behind a `wgpu::Queue` for diagnostics.
pub(crate) fn queue_backend_kind(queue: &wgpu::Queue) -> wgpu::Backend {
    // SAFETY: Read-only inspection of the queue's HAL backend tag without modifying state.
    if unsafe { queue.as_hal::<Vulkan>() }.is_some() {
        return wgpu::Backend::Vulkan;
    }
    #[cfg(windows)]
    // SAFETY: Read-only inspection of the queue's HAL backend tag without modifying state.
    if unsafe { queue.as_hal::<Dx12>() }.is_some() {
        return wgpu::Backend::Dx12;
    }
    wgpu::Backend::Noop
}

/// Resolves a dedicated Vulkan barrier `vk::Queue` `(family, index)` on `device`.
/// Submitting ownership/layout barriers on this queue (same family as `wgpu`, distinct
/// `VkQueue` handle) avoids `VUID-vkQueueSubmit-queue-06445` races against
/// `wgpu::Queue::submit` / `SurfaceTexture::present` on other threads.
pub(crate) fn resolve_vulkan_barrier_queue(
    device: &wgpu::Device,
    family: u32,
    index: u32,
) -> Result<vk::Queue> {
    // SAFETY: Read-only lookup of a queue created alongside `device`.
    let hal_dev = unsafe { device.as_hal::<Vulkan>() }.ok_or_else(|| Error::BackendMismatch {
        expected: "Vulkan",
        found: device.adapter_info().backend,
    })?;
    // SAFETY: Caller guarantees `(family, index)` was created with this logical Vulkan device.
    Ok(unsafe { hal_dev.raw_device().get_device_queue(family, index) })
}

/// Acquires `nv-wgpu`'s internal Vulkan queue-submission serializer and invokes `f`
/// with `dedicated` if provided, otherwise the raw `vk::Queue` extracted from `queue`.
///
/// This prevents concurrent `VkBarrierRunner` submissions across `SharedBuffer` /
/// `SharedTexture` instances from racing on `vkQueueSubmit` (`VUID-vkQueueSubmit-queue-06445`).
pub(crate) fn with_barrier_vk_queue<F, R>(
    queue: &wgpu::Queue,
    dedicated: Option<vk::Queue>,
    f: F,
) -> Result<R>
where
    F: FnOnce(vk::Queue) -> Result<R>,
{
    let _guard = VK_QUEUE_SUBMIT_LOCK
        .lock()
        .map_err(|_| Error::InvalidHandle {
            source_api: "VK_QUEUE_SUBMIT_LOCK mutex poisoned",
        })?;
    if let Some(raw_q) = dedicated {
        return f(raw_q);
    }
    // SAFETY: Inspects Vulkan HAL queue handle; caller serializes against external `wgpu::Queue::submit`.
    let hal_q = unsafe { queue.as_hal::<Vulkan>() }.ok_or_else(|| Error::BackendMismatch {
        expected: "Vulkan",
        found: queue_backend_kind(queue),
    })?;
    f(hal_q.as_raw())
}

/// Flushes optional staged wait and signal operations through `wgpu::Queue::submit(empty)`
/// so `wgpu`'s pending writes flush first and `wait` executes before `signal`.
pub(crate) fn flush_wgpu_wait_signal(
    queue: &wgpu::Queue,
    wait: Option<(&crate::sync::InteropSemaphore, u64)>,
    signal: Option<(&crate::sync::InteropSemaphore, u64)>,
) -> Result<()> {
    if let Some((sem, val)) = wait {
        sem.stage_wait_in_wgpu(queue, val)?;
    }
    if let Some((sem, val)) = signal {
        sem.stage_signal_in_wgpu(queue, val)?;
    }
    if wait.is_some() || signal.is_some() {
        queue.submit(core::iter::empty());
    }
    Ok(())
}

struct BarrierSlot {
    cmd: vk::CommandBuffer,
    fence: vk::Fence,
}

/// Reusable command-buffer + fence pool for zero-allocation steady-state Vulkan
/// layout and `VK_QUEUE_FAMILY_EXTERNAL` ownership transitions.
///
/// Pre-allocates two signaled `(VkCommandBuffer, VkFence)` slots so back-to-back
/// acquire/release barriers never reset an in-flight command buffer and never
/// stall the CPU in `vkWaitForFences`.
pub(crate) struct VkBarrierRunner {
    device: ash::Device,
    pool: vk::CommandPool,
    slots: Mutex<Vec<BarrierSlot>>,
    queue_family: u32,
}

impl VkBarrierRunner {
    const INITIAL_SLOTS: usize = 2;

    /// Allocates one command pool and pre-seeds reusable signaled `(cmd, fence)` slots.
    pub(crate) fn new(device: ash::Device, queue_family: u32) -> Result<Self> {
        let pool_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(queue_family)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        // SAFETY: `device` is a live logical Vulkan device and `queue_family` belongs to it.
        let pool = unsafe { device.create_command_pool(&pool_info, None) }
            .map_err(|e| Error::vulkan("vkCreateCommandPool", e))?;

        let mut slots = Vec::with_capacity(Self::INITIAL_SLOTS);
        for _ in 0..Self::INITIAL_SLOTS {
            match alloc_barrier_slot(&device, pool) {
                Ok(slot) => slots.push(slot),
                Err(err) => {
                    for s in slots {
                        // SAFETY: Each `s.fence` was created by `device` and never submitted.
                        unsafe { device.destroy_fence(s.fence, None) };
                    }
                    // SAFETY: `pool` was created above and no command buffers escaped.
                    unsafe { device.destroy_command_pool(pool, None) };
                    return Err(err);
                }
            }
        }

        Ok(Self {
            device,
            pool,
            slots: Mutex::new(slots),
            queue_family,
        })
    }

    /// Returns the wgpu queue family index this runner targets.
    #[inline]
    pub(crate) fn queue_family(&self) -> u32 {
        self.queue_family
    }

    /// Submits a single `vk::ImageMemoryBarrier` with zero steady-state allocations.
    pub(crate) fn submit_image_barrier(
        &self,
        queue: vk::Queue,
        src_stage: vk::PipelineStageFlags,
        dst_stage: vk::PipelineStageFlags,
        barrier: &vk::ImageMemoryBarrier<'_>,
        wait: Option<(vk::Semaphore, u64)>,
        signal: Option<(vk::Semaphore, u64)>,
    ) -> Result<()> {
        self.record_and_submit(queue, wait, signal, |device, cmd| {
            // SAFETY: `cmd` is freshly reset & begun; `barrier` borrows valid image handle.
            unsafe {
                device.cmd_pipeline_barrier(
                    cmd,
                    src_stage,
                    dst_stage,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    core::slice::from_ref(barrier),
                );
            }
        })
    }

    /// Submits a single `vk::BufferMemoryBarrier` with zero steady-state allocations.
    pub(crate) fn submit_buffer_barrier(
        &self,
        queue: vk::Queue,
        src_stage: vk::PipelineStageFlags,
        dst_stage: vk::PipelineStageFlags,
        barrier: &vk::BufferMemoryBarrier<'_>,
        wait: Option<(vk::Semaphore, u64)>,
        signal: Option<(vk::Semaphore, u64)>,
    ) -> Result<()> {
        self.record_and_submit(queue, wait, signal, |device, cmd| {
            // SAFETY: `cmd` is freshly reset & begun; `barrier` borrows valid buffer handle.
            unsafe {
                device.cmd_pipeline_barrier(
                    cmd,
                    src_stage,
                    dst_stage,
                    vk::DependencyFlags::empty(),
                    &[],
                    core::slice::from_ref(barrier),
                    &[],
                );
            }
        })
    }

    fn record_and_submit<F>(
        &self,
        queue: vk::Queue,
        wait: Option<(vk::Semaphore, u64)>,
        signal: Option<(vk::Semaphore, u64)>,
        record: F,
    ) -> Result<()>
    where
        F: FnOnce(&ash::Device, vk::CommandBuffer),
    {
        let mut guard = self.slots.lock().map_err(|_| Error::InvalidHandle {
            source_api: "VkBarrierRunner mutex poisoned",
        })?;

        let mut chosen = None;
        for (idx, slot) in guard.iter().enumerate() {
            // SAFETY: `slot.fence` is a valid `VkFence` owned by `self.device`.
            if unsafe { self.device.get_fence_status(slot.fence) } == Ok(true) {
                chosen = Some(idx);
                break;
            }
        }
        let idx = match chosen {
            Some(i) => i,
            None => {
                let extra = alloc_barrier_slot(&self.device, self.pool)?;
                guard.push(extra);
                guard.len() - 1
            }
        };
        let (slot_cmd, slot_fence) = (guard[idx].cmd, guard[idx].fence);

        let begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        // SAFETY: `slot_fence` is signaled (retired on GPU) and `guard` serializes pool/cmd access.
        unsafe {
            self.device
                .reset_command_buffer(slot_cmd, vk::CommandBufferResetFlags::empty())
                .map_err(|e| Error::vulkan("vkResetCommandBuffer", e))?;
            self.device
                .begin_command_buffer(slot_cmd, &begin_info)
                .map_err(|e| Error::vulkan("vkBeginCommandBuffer", e))?;
        }
        record(&self.device, slot_cmd);
        // SAFETY: `slot_cmd` is in recording state and only references stack barriers.
        unsafe {
            self.device
                .end_command_buffer(slot_cmd)
                .map_err(|e| Error::vulkan("vkEndCommandBuffer", e))?;
            // Reset fence only right before `vkQueueSubmit` so pre-submit errors never leave
            // a live slot fence stuck unsignaled.
            self.device
                .reset_fences(core::slice::from_ref(&slot_fence))
                .map_err(|e| Error::vulkan("vkResetFences", e))?;
        }
        if let Err(err) = submit_single_cmd(&self.device, queue, slot_cmd, slot_fence, wait, signal)
        {
            // Discard and destroy the slot because `slot_fence` was reset and never submitted.
            let dead = guard.swap_remove(idx);
            // SAFETY: `dead.fence` and `dead.cmd` were never submitted after reset.
            unsafe {
                self.device.destroy_fence(dead.fence, None);
                self.device
                    .free_command_buffers(self.pool, core::slice::from_ref(&dead.cmd));
            }
            return Err(err);
        }
        Ok(())
    }
}

fn alloc_barrier_slot(device: &ash::Device, pool: vk::CommandPool) -> Result<BarrierSlot> {
    let alloc_info = vk::CommandBufferAllocateInfo::default()
        .command_pool(pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);
    // SAFETY: `pool` belongs to `device` and allocates 1 primary command buffer.
    let cmd = unsafe { device.allocate_command_buffers(&alloc_info) }
        .map_err(|e| Error::vulkan("vkAllocateCommandBuffers", e))?[0];
    let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
    // SAFETY: `device` is live and `fence_info` has valid flags.
    let fence = match unsafe { device.create_fence(&fence_info, None) } {
        Ok(f) => f,
        Err(err) => {
            // SAFETY: `cmd` was just allocated from `pool` and never submitted.
            unsafe { device.free_command_buffers(pool, core::slice::from_ref(&cmd)) };
            return Err(Error::vulkan("vkCreateFence", err));
        }
    };
    Ok(BarrierSlot { cmd, fence })
}

impl Drop for VkBarrierRunner {
    fn drop(&mut self) {
        if let Ok(slots) = self.slots.get_mut() {
            for slot in slots.drain(..) {
                // SAFETY: Wait for any still-pending barrier submission before destroying
                // `slot.fence` or freeing `self.pool` (`VUID-vkDestroyFence-fence-01120` / `00041`).
                unsafe {
                    if self.device.get_fence_status(slot.fence) == Ok(false)
                        && let Err(err) = self.device.wait_for_fences(
                            core::slice::from_ref(&slot.fence),
                            true,
                            u64::MAX,
                        )
                    {
                        eprintln!(
                            "nv-wgpu: vkWaitForFences during VkBarrierRunner::drop failed: {err:?}"
                        );
                    }
                    self.device.destroy_fence(slot.fence, None);
                }
            }
        }
        // SAFETY: All submitted barrier fences have completed above and `pool` belongs to `self.device`.
        unsafe {
            self.device.destroy_command_pool(self.pool, None);
        }
    }
}

/// Submits one primary command buffer with optional stack-allocated timeline semaphore wait/signal.
fn submit_single_cmd(
    device: &ash::Device,
    queue: vk::Queue,
    cmd: vk::CommandBuffer,
    fence: vk::Fence,
    wait: Option<(vk::Semaphore, u64)>,
    signal: Option<(vk::Semaphore, u64)>,
) -> Result<()> {
    let wait_sems = [wait.map_or(vk::Semaphore::null(), |(s, _)| s)];
    let wait_vals = [wait.map_or(0, |(_, v)| v)];
    let wait_stages = [vk::PipelineStageFlags::ALL_COMMANDS];
    let signal_sems = [signal.map_or(vk::Semaphore::null(), |(s, _)| s)];
    let signal_vals = [signal.map_or(0, |(_, v)| v)];
    let cmds = [cmd];

    let wait_count = usize::from(wait.is_some());
    let signal_count = usize::from(signal.is_some());

    let mut timeline = vk::TimelineSemaphoreSubmitInfo::default()
        .wait_semaphore_values(&wait_vals[..wait_count])
        .signal_semaphore_values(&signal_vals[..signal_count]);

    let submit = vk::SubmitInfo::default()
        .command_buffers(&cmds)
        .wait_semaphores(&wait_sems[..wait_count])
        .wait_dst_stage_mask(&wait_stages[..wait_count])
        .signal_semaphores(&signal_sems[..signal_count])
        .push_next(&mut timeline);

    // SAFETY: `cmd` is ended executable primary command buffer, `fence` is unsignaled, and slices match bounds.
    unsafe { device.queue_submit(queue, core::slice::from_ref(&submit), fence) }
        .map_err(|e| Error::vulkan("vkQueueSubmit", e))
}
