//! Shared internal helpers for Vulkan/DX12 handles, usage mapping, and raw command submission.
//!
//! Why: `buffer`, `texture`, and `sync` all need identical Vulkan memory-type
//! selection, `wgpu::*Usages -> *Uses` conversion, Win32 `HANDLE` / POSIX `fd`
//! RAII ownership across CUDA import boundaries, and a way to put raw Vulkan
//! commands (layout / queue-family ownership barriers, NGX evaluation) on `wgpu`'s
//! queue. Centralizing them here removes duplication across modules.
//!
//! Raw commands are recorded into a `wgpu` command encoder (`CommandEncoder::as_hal_mut`)
//! and submitted with `wgpu::Queue::submit` ([`submit_vulkan_raw`]): `vkQueueSubmit` requires
//! the queue to be externally synchronized, and only `wgpu-core` holds the lock that its own
//! submits and presents take.
//!
//! Every blocking `wgpu` poll goes through [`wait_submission_bounded`] (limit
//! [`VULKAN_SUBMIT_TIMEOUT`](crate::VULKAN_SUBMIT_TIMEOUT)). A `Drop` impl that times out or fails
//! reports it to `cuda_core_ext::teardown` under `Library::NvWgpu` ([`record_drop_failure`]; status
//! [`TEARDOWN_WAIT_TIMEOUT`] or [`TEARDOWN_STEP_FAILED`]) instead of waiting forever or
//! swallowing it; a timeout means the device must be treated as lost. The Vulkan dedicated
//! allocation sequence, ownership-transfer masks and the DX12 shared committed resource are
//! single sources here too ([`alloc_bind_exportable_dedicated`], [`ownership_transfer`],
//! `create_shared_committed_resource`).
//!
//! Used internally by `device`, `sync`, `buffer`, `texture`, and the crate root.

#[cfg(unix)]
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::sync::Arc;

use ash::vk;
use cuda_core::Context;
use cuda_core_ext::teardown::{self, Library};
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
use crate::sync::InteropSemaphore;

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
            // `cuImportExternalMemory` does not consume a Win32 handle (it takes its own
            // reference to the object), so the exported NT handle is closed right after import.
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

/// A freshly created Vulkan resource that gets a dedicated, exportable memory allocation.
#[derive(Debug, Clone, Copy)]
pub(crate) enum DedicatedResource {
    Buffer(vk::Buffer),
    Image(vk::Image),
}

impl DedicatedResource {
    /// Destroys the resource.
    ///
    /// # Safety
    /// The resource must belong to `device`, must not be destroyed already, and no GPU work may
    /// reference it.
    unsafe fn destroy(self, device: &ash::Device) {
        match self {
            // SAFETY: forwarded from this function's contract.
            Self::Buffer(b) => unsafe { device.destroy_buffer(b, None) },
            // SAFETY: forwarded from this function's contract.
            Self::Image(i) => unsafe { device.destroy_image(i, None) },
        }
    }

    /// The Vulkan entry point name for allocation errors.
    fn alloc_op(self) -> &'static str {
        match self {
            Self::Buffer(_) => "vkAllocateMemory(Buffer)",
            Self::Image(_) => "vkAllocateMemory(Image)",
        }
    }

    /// The Vulkan entry point name for bind errors.
    fn bind_op(self) -> &'static str {
        match self {
            Self::Buffer(_) => "vkBindBufferMemory",
            Self::Image(_) => "vkBindImageMemory",
        }
    }
}

/// Allocates one dedicated `DEVICE_LOCAL` allocation exportable as an OS handle for `resource`
/// and binds it at offset 0: the single sequence behind `SharedBuffer` and `SharedTexture`.
///
/// Takes ownership of `resource`: on any failure it is destroyed (and a partial allocation freed)
/// before the error is returned. On success the caller owns both `resource` and the returned
/// memory (typically through a backing whose `Drop` destroys them), together with the memory
/// requirements the allocation was sized from (needed for the CUDA import size).
pub(crate) fn alloc_bind_exportable_dedicated(
    instance: &ash::Instance,
    device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    resource: DedicatedResource,
) -> Result<(vk::DeviceMemory, vk::MemoryRequirements)> {
    // SAFETY: `resource` was just created on `device`.
    let reqs = unsafe {
        match resource {
            DedicatedResource::Buffer(b) => device.get_buffer_memory_requirements(b),
            DedicatedResource::Image(i) => device.get_image_memory_requirements(i),
        }
    };
    let type_index = match find_vk_memory_type_index(
        instance,
        physical_device,
        reqs.memory_type_bits,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    ) {
        Ok(idx) => idx,
        Err(err) => {
            // SAFETY: `resource` is unbound and unused by any GPU work.
            unsafe { resource.destroy(device) };
            return Err(err);
        }
    };

    let mut export_info =
        vk::ExportMemoryAllocateInfo::default().handle_types(vk_external_memory_handle_type());
    let mut dedicated_info = match resource {
        DedicatedResource::Buffer(b) => vk::MemoryDedicatedAllocateInfo::default().buffer(b),
        DedicatedResource::Image(i) => vk::MemoryDedicatedAllocateInfo::default().image(i),
    };
    let alloc_info = vk::MemoryAllocateInfo::default()
        .allocation_size(reqs.size)
        .memory_type_index(type_index)
        .push_next(&mut export_info)
        .push_next(&mut dedicated_info);
    // SAFETY: `type_index` and `reqs.size` come from the resource's own memory requirements.
    let memory = match unsafe { device.allocate_memory(&alloc_info, None) } {
        Ok(m) => m,
        Err(err) => {
            // SAFETY: `resource` is unbound and unused by any GPU work.
            unsafe { resource.destroy(device) };
            return Err(Error::vulkan(resource.alloc_op(), err));
        }
    };

    // SAFETY: `resource` is unbound and `memory` matches its requirements.
    let bound = unsafe {
        match resource {
            DedicatedResource::Buffer(b) => device.bind_buffer_memory(b, memory, 0),
            DedicatedResource::Image(i) => device.bind_image_memory(i, memory, 0),
        }
    };
    if let Err(err) = bound {
        // SAFETY: nothing references the unbound resource or the unused memory.
        unsafe {
            resource.destroy(device);
            device.free_memory(memory, None);
        }
        return Err(Error::vulkan(resource.bind_op(), err));
    }
    Ok((memory, reqs))
}

/// Queue families and access masks of an ownership transfer between `wgpu`'s queue family and
/// `VK_QUEUE_FAMILY_EXTERNAL` (CUDA): the single source for buffer and image barriers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OwnershipTransfer {
    pub src_family: u32,
    pub dst_family: u32,
    pub src_access: vk::AccessFlags,
    pub dst_access: vk::AccessFlags,
}

/// The transfer that acquires a resource from CUDA into `wgpu_family` (`acquire_to_wgpu`), or
/// releases it from `wgpu_family` to CUDA.
pub(crate) fn ownership_transfer(acquire_to_wgpu: bool, wgpu_family: u32) -> OwnershipTransfer {
    if acquire_to_wgpu {
        OwnershipTransfer {
            src_family: vk::QUEUE_FAMILY_EXTERNAL,
            dst_family: wgpu_family,
            src_access: vk::AccessFlags::empty(),
            dst_access: vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE,
        }
    } else {
        OwnershipTransfer {
            src_family: wgpu_family,
            dst_family: vk::QUEUE_FAMILY_EXTERNAL,
            src_access: vk::AccessFlags::MEMORY_WRITE,
            dst_access: vk::AccessFlags::empty(),
        }
    }
}

/// Creates a committed resource in a shared `DEFAULT` heap (`D3D12_HEAP_FLAG_SHARED`, initial
/// state `COMMON`): the single DX12 creation path behind `SharedBuffer` and `SharedTexture`.
#[cfg(windows)]
pub(crate) fn create_shared_committed_resource(
    raw_device: &Direct3D12::ID3D12Device,
    res_desc: &Direct3D12::D3D12_RESOURCE_DESC,
    call_name: &'static str,
) -> Result<Direct3D12::ID3D12Resource> {
    let heap_props = Direct3D12::D3D12_HEAP_PROPERTIES {
        Type: Direct3D12::D3D12_HEAP_TYPE_DEFAULT,
        CPUPageProperty: Direct3D12::D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
        MemoryPoolPreference: Direct3D12::D3D12_MEMORY_POOL_UNKNOWN,
        CreationNodeMask: 1,
        VisibleNodeMask: 1,
    };
    let mut resource: Option<Direct3D12::ID3D12Resource> = None;
    // SAFETY: creates a committed resource described by `res_desc` in a shared default heap;
    // `resource` receives the new COM object.
    unsafe {
        raw_device.CreateCommittedResource(
            &heap_props,
            Direct3D12::D3D12_HEAP_FLAG_SHARED,
            res_desc,
            Direct3D12::D3D12_RESOURCE_STATE_COMMON,
            None,
            &mut resource,
        )
    }
    .map_err(|e| Error::dx12(call_name, e))?;
    resource.ok_or(Error::InvalidHandle {
        source_api: "ID3D12Device::CreateCommittedResource",
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
        // wgpu-core also binds indirect buffers as storage for indirect validation
        // (`Device::create_buffer`: INDIRECT adds STORAGE_READ_ONLY | STORAGE_READ_WRITE).
        flags |= vk::BufferUsageFlags::INDIRECT_BUFFER | vk::BufferUsageFlags::STORAGE_BUFFER;
    }
    if usage.contains(wgpu::BufferUsages::QUERY_RESOLVE) {
        // `resolve_query_set` is `vkCmdCopyQueryPoolResults` into this buffer, which needs
        // TRANSFER_DST (VUID-vkCmdCopyQueryPoolResults-dstBuffer-00825); wgpu-core also binds it
        // as storage for timestamp normalization (TIMESTAMP_NORMALIZATION_BUFFER_USES).
        flags |= vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::STORAGE_BUFFER;
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

/// Maps a [`wgpu::TextureUses`] state of a `format` texture to the [`vk::ImageLayout`] `wgpu-hal`
/// tracks for it. Delegates to `wgpu-hal` itself so the two cannot drift apart.
pub(crate) fn vk_layout_for_texture_uses(
    uses: wgpu::TextureUses,
    format: wgpu::TextureFormat,
) -> vk::ImageLayout {
    wgpu::hal::vulkan::conv::derive_image_layout(uses, format)
}

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

/// Flushes optional staged wait and signal operations through `wgpu::Queue::submit(empty)`
/// so `wgpu`'s pending writes flush first and `wait` executes before `signal`.
pub(crate) fn flush_wgpu_wait_signal(
    queue: &wgpu::Queue,
    wait: Option<(&InteropSemaphore, u64)>,
    signal: Option<(&InteropSemaphore, u64)>,
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

/// Records raw Vulkan commands with `record` into a fresh `wgpu` command encoder and submits it
/// with `wgpu::Queue::submit`, with `wait` staged before and `signal` after that submission.
///
/// Why through `wgpu`: `vkQueueSubmit` requires its queue to be externally synchronized, and only
/// `wgpu-core` holds the lock its own submits and presents take, so a raw submit on `wgpu`'s
/// `VkQueue` races any other thread's `wgpu::Queue::submit`. The same submission also carries
/// `wgpu`'s pending `write_buffer` / `write_texture` uploads ahead of the recorded commands.
///
/// Staging is not atomic with respect to other threads: a concurrent `submit` on the same queue
/// can pick up the staged `wait` / `signal`. Serialize submissions that use them.
///
/// Returns `record`'s value and the submission index (for `wgpu::Device::poll`). Nothing is
/// submitted or staged when `record` fails.
pub(crate) fn submit_vulkan_raw<R>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    wait: Option<(&InteropSemaphore, u64)>,
    signal: Option<(&InteropSemaphore, u64)>,
    record: impl FnOnce(vk::CommandBuffer) -> Result<R>,
) -> Result<(R, wgpu::SubmissionIndex)> {
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("nv-wgpu raw vulkan commands"),
    });
    let record_raw = |hal_enc: Option<&mut wgpu::hal::vulkan::CommandEncoder>| {
        hal_enc.map(|hal_enc| {
            // SAFETY: `wgpu-core` opens the encoder before handing it to this callback, so
            // `raw_handle` is its command buffer in the recording state.
            let cmd = unsafe { hal_enc.raw_handle() };
            record(cmd)
        })
    };
    // SAFETY: the callback only records commands into the active command buffer; it neither ends
    // nor frees it, and `encoder` is not used again until the callback has returned.
    let recorded = unsafe { encoder.as_hal_mut::<Vulkan, _, _>(record_raw) };
    let value = match recorded {
        Some(res) => res?,
        None => {
            return Err(Error::BackendMismatch {
                expected: "Vulkan",
                found: device.adapter_info().backend,
            });
        }
    };
    let commands = encoder.finish();
    if let Some((sem, val)) = wait {
        sem.stage_wait_in_wgpu(queue, val)?;
    }
    if let Some((sem, val)) = signal
        && let Err(err) = sem.stage_signal_in_wgpu(queue, val)
    {
        if let Some((waited, _)) = wait {
            waited.unstage_wait_in_wgpu(queue)?;
        }
        return Err(err);
    }
    let index = queue.submit([commands]);
    Ok((value, index))
}

/// Teardown status for a bounded wait that hit [`VULKAN_SUBMIT_TIMEOUT`](crate::VULKAN_SUBMIT_TIMEOUT).
pub(crate) const TEARDOWN_WAIT_TIMEOUT: i64 = 1;
/// Teardown status for a wait or unstage step that failed for any other reason.
pub(crate) const TEARDOWN_STEP_FAILED: i64 = 2;

/// The error a bounded submission wait reports: a timeout is [`Error::Timeout`] naming `what`,
/// everything else stays [`Error::Poll`].
pub(crate) fn poll_wait_error(what: &'static str, err: wgpu::PollError) -> Error {
    match err {
        wgpu::PollError::Timeout => Error::Timeout { what },
        other => Error::Poll(other),
    }
}

/// Waits for submission `index` of `device` (`None`: the most recent one), at most
/// [`VULKAN_SUBMIT_TIMEOUT`](crate::VULKAN_SUBMIT_TIMEOUT). The single bounded-wait path for every
/// blocking `wgpu` poll in this crate.
pub(crate) fn wait_submission_bounded(
    device: &wgpu::Device,
    index: Option<wgpu::SubmissionIndex>,
    what: &'static str,
) -> Result<()> {
    device
        .poll(wgpu::PollType::Wait {
            submission_index: index,
            timeout: Some(crate::VULKAN_SUBMIT_TIMEOUT),
        })
        .map(|_| ())
        .map_err(|err| poll_wait_error(what, err))
}

/// The teardown status [`record_drop_failure`] stores for `err`.
pub(crate) fn teardown_status(err: &Error) -> i64 {
    match err {
        Error::Timeout { .. } => TEARDOWN_WAIT_TIMEOUT,
        _ => TEARDOWN_STEP_FAILED,
    }
}

/// Reports a failed step of a `Drop` impl (`op`) to the typed teardown sink
/// (`cuda_core_ext::teardown`, [`Library::NvWgpu`]): `Drop` cannot return it, and it must not be
/// swallowed.
pub(crate) fn record_drop_failure(op: &'static str, err: &Error) {
    teardown::record(Library::NvWgpu, op, teardown_status(err));
}

/// Waits (bounded) until submission `index` (if any) of `device` completes. For `Drop` impls that
/// must not free native objects still referenced by in-flight raw commands. A timeout or poll
/// failure goes to the teardown sink under `op`; the caller then drops the native objects anyway
/// (a hung GPU never completes), so such an error means the device must be treated as lost.
pub(crate) fn wait_for_submission(
    device: &wgpu::Device,
    index: Option<wgpu::SubmissionIndex>,
    op: &'static str,
) {
    if index.is_none() {
        return;
    }
    if let Err(err) = wait_submission_bounded(device, index, op) {
        record_drop_failure(op, &err);
    }
}

/// Every mip level and array layer of a colour image.
pub(crate) fn full_color_subresource_range() -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange {
        aspect_mask: vk::ImageAspectFlags::COLOR,
        base_mip_level: 0,
        level_count: vk::REMAINING_MIP_LEVELS,
        base_array_layer: 0,
        layer_count: vk::REMAINING_ARRAY_LAYERS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_usage_mapping_covers_query_resolve_and_indirect() {
        let resolve = map_vk_buffer_usage(wgpu::BufferUsages::QUERY_RESOLVE);
        assert!(resolve.contains(vk::BufferUsageFlags::TRANSFER_DST));
        assert!(resolve.contains(vk::BufferUsageFlags::STORAGE_BUFFER));
        let indirect = map_vk_buffer_usage(wgpu::BufferUsages::INDIRECT);
        assert!(indirect.contains(vk::BufferUsageFlags::INDIRECT_BUFFER));
        assert!(indirect.contains(vk::BufferUsageFlags::STORAGE_BUFFER));
    }

    /// A bounded wait that hits the timeout is a typed `Timeout` carrying the wait name, every other
    /// poll failure stays `Poll`, and the teardown sink gets distinct codes for the two.
    #[test]
    fn bounded_wait_errors_are_typed_and_map_to_teardown_codes() {
        let timeout = poll_wait_error("SharedBuffer drop", wgpu::PollError::Timeout);
        assert!(matches!(timeout, Error::Timeout { what } if what == "SharedBuffer drop"));
        assert_eq!(teardown_status(&timeout), TEARDOWN_WAIT_TIMEOUT);

        let other = poll_wait_error("x", wgpu::PollError::WrongSubmissionIndex(3, 2));
        assert!(matches!(
            other,
            Error::Poll(wgpu::PollError::WrongSubmissionIndex(3, 2))
        ));
        assert_eq!(teardown_status(&other), TEARDOWN_STEP_FAILED);
        assert_ne!(TEARDOWN_WAIT_TIMEOUT, TEARDOWN_STEP_FAILED);

        // Only this test records `Library::NvWgpu` in this crate's test binary.
        let _ = teardown::take(Some(Library::NvWgpu));
        record_drop_failure("SharedBuffer::drop", &timeout);
        let drained = teardown::take(Some(Library::NvWgpu));
        assert_eq!(drained.errors.len(), 1);
        assert_eq!(drained.errors[0].op, "SharedBuffer::drop");
        assert_eq!(drained.errors[0].status, TEARDOWN_WAIT_TIMEOUT);
    }

    #[test]
    fn layout_mapping_follows_wgpu_hal() {
        let fmt = wgpu::TextureFormat::Rgba8Unorm;
        assert_eq!(
            vk_layout_for_texture_uses(wgpu::TextureUses::UNINITIALIZED, fmt),
            vk::ImageLayout::UNDEFINED
        );
        assert_eq!(
            vk_layout_for_texture_uses(wgpu::TextureUses::COPY_SRC, fmt),
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL
        );
        assert_eq!(
            vk_layout_for_texture_uses(wgpu::TextureUses::STORAGE_READ_WRITE, fmt),
            vk::ImageLayout::GENERAL
        );
    }

    #[test]
    fn ownership_transfer_release_and_acquire_are_mirror_images() {
        let release = ownership_transfer(false, 3);
        let acquire = ownership_transfer(true, 3);
        assert_eq!(release.src_family, 3);
        assert_eq!(release.dst_family, vk::QUEUE_FAMILY_EXTERNAL);
        assert_eq!(acquire.src_family, vk::QUEUE_FAMILY_EXTERNAL);
        assert_eq!(acquire.dst_family, 3);
        assert_eq!(release.src_access, vk::AccessFlags::MEMORY_WRITE);
        assert_eq!(release.dst_access, vk::AccessFlags::empty());
        assert_eq!(acquire.src_access, vk::AccessFlags::empty());
        assert!(acquire.dst_access.contains(vk::AccessFlags::MEMORY_READ));
    }

    #[test]
    fn full_color_range_covers_every_level_and_layer() {
        let range = full_color_subresource_range();
        assert_eq!(range.level_count, vk::REMAINING_MIP_LEVELS);
        assert_eq!(range.layer_count, vk::REMAINING_ARRAY_LAYERS);
        assert_eq!((range.base_mip_level, range.base_array_layer), (0, 0));
    }
}
