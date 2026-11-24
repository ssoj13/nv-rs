//! Zero-copy shared GPU buffers (`wgpu::Buffer` <-> `cuda_core::RawDeviceBuffer`).
//!
//! Why: Transfers large arrays, vertex/storage buffers, or compute results directly
//! in device-local VRAM between CUDA kernels and `wgpu` shaders without host staging:
//! - On Vulkan: allocates an exportable dedicated `VkBuffer` + `VkDeviceMemory`, wraps it
//!   into `wgpu::Buffer` via `wgpu::hal::vulkan::Buffer::from_raw_externally_owned` +
//!   `create_buffer_from_hal`, and imports it into CUDA via `ExternalMemory::mapped_buffer`.
//! - On DX12 (Windows): allocates a committed shared `ID3D12Resource` (`D3D12_HEAP_FLAG_SHARED`),
//!   wraps it via `wgpu::hal::dx12::Device::buffer_from_raw`, and imports via
//!   `MemoryHandle::D3d12Resource`.
//!
//! Drop order: `SharedBuffer::drop` first waits for its last ownership-barrier submission, then
//! `RawDeviceBuffer` -> `ExternalMemory` -> `wgpu::Buffer` -> native `VkBuffer`/`VkDeviceMemory`
//! (via a refcounted backing shared with `wgpu-hal`'s `DropCallback`) -> `wgpu::Device`, so the
//! native buffer is destroyed only after `SharedBuffer`'s own CUDA views and `wgpu::Buffer` are
//! gone and while the `VkDevice` is still alive. CUDA views cloned out of it
//! ([`SharedBuffer::cuda_buffer`], [`SharedBuffer::external_memory`]) keep the CUDA import alive
//! but not the native allocation: drop them first, or finish their CUDA work before dropping
//! the `SharedBuffer`.
//!
//! Used by callers and GPU interop tests.

use std::sync::{Arc, Mutex, PoisonError};

use ash::vk;
use cuda_core::{Context, RawDeviceBuffer};
use cuda_core_ext::ExternalMemory;
#[cfg(windows)]
use wgpu::hal::api::Dx12;
use wgpu::hal::api::Vulkan;
#[cfg(windows)]
use windows::Win32::Graphics::{Direct3D12, Dxgi};

use crate::device::{InteropBackend, verify_device_interop_support};
use crate::error::{Error, Result};
use crate::platform::{
    DedicatedResource, ExternalMemoryKind, alloc_bind_exportable_dedicated, export_vk_memory,
    map_vk_buffer_usage, ownership_transfer, submit_vulkan_raw, vk_external_memory_handle_type,
    wait_for_submission,
};
use crate::sync::InteropSemaphore;

#[cfg(windows)]
use crate::platform::{create_shared_committed_resource, export_d3d12_shared_handle};

/// Descriptor for creating a zero-copy shared buffer between `wgpu` and CUDA.
#[derive(Debug, Clone, Copy)]
pub struct SharedBufferDescriptor<'a> {
    /// Optional debug label attached to the wrapped `wgpu::Buffer`.
    pub label: Option<&'a str>,
    /// Requested logical buffer size in bytes (must be > 0).
    pub size: u64,
    /// `wgpu` usage flags (must not include `MAP_READ` or `MAP_WRITE`, because externally
    /// imported device-local buffers cannot be host-mapped through `wgpu`).
    pub usage: wgpu::BufferUsages,
}

/// Validates a [`SharedBufferDescriptor`] cleanly before touching any GPU driver APIs.
pub fn validate_buffer_descriptor(desc: &SharedBufferDescriptor<'_>) -> Result<()> {
    if desc.size == 0 {
        return Err(Error::InvalidInput("shared buffer size must be non-zero"));
    }
    if desc.usage.is_empty() {
        return Err(Error::InvalidInput("shared buffer usage must be non-empty"));
    }
    if desc
        .usage
        .intersects(wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::MAP_WRITE)
    {
        return Err(Error::InvalidInput(
            "externally shared buffers cannot combine MAP_READ or MAP_WRITE",
        ));
    }
    // Acceleration-structure inputs need device-address memory and AS build usage flags, which
    // this exporter does not allocate.
    if desc
        .usage
        .intersects(wgpu::BufferUsages::BLAS_INPUT | wgpu::BufferUsages::TLAS_INPUT)
    {
        return Err(Error::InvalidInput(
            "externally shared buffers cannot be acceleration-structure build inputs",
        ));
    }
    Ok(())
}

/// Refcounted Vulkan buffer + dedicated `VkDeviceMemory` owner shared between
/// [`SharedBuffer`] and `wgpu-hal`'s `DropCallback` (zero `wgpu::Device` cycle).
struct VkBufferBacking {
    device: ash::Device,
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
}

impl Drop for VkBufferBacking {
    fn drop(&mut self) {
        // SAFETY: Destroyed only after both CUDA `ExternalMemory` and `wgpu::Buffer`'s
        // HAL `DropCallback` have dropped, guaranteeing zero active references.
        unsafe {
            self.device.destroy_buffer(self.buffer, None);
            self.device.free_memory(self.memory, None);
        }
    }
}

enum BufferNativeBacking {
    Vulkan {
        vk: Arc<VkBufferBacking>,
        /// `wgpu`'s queue family: the family side of ownership transfers.
        queue_family: u32,
    },
    #[cfg(windows)]
    Dx12 {
        _resource: Direct3D12::ID3D12Resource,
    },
}

/// Zero-copy GPU buffer accessible simultaneously as [`wgpu::Buffer`] and
/// [`cuda_core::RawDeviceBuffer`].
///
/// `Drop` first waits for the last ownership barrier submitted for this buffer (it references
/// the native buffer outside `wgpu`'s tracking), then fields drop in declaration order:
/// 1. `cuda_buf` (`RawDeviceBuffer`)
/// 2. `ext_mem` (`Arc<ExternalMemory>`)
/// 3. `wgpu_buf` (`wgpu::Buffer`)
/// 4. `backing` (`Arc<VkBufferBacking>` / COM `ID3D12Resource`)
/// 5. `device` (`wgpu::Device`), which keeps the `VkDevice` alive for step 4
pub struct SharedBuffer {
    cuda_buf: RawDeviceBuffer,
    ext_mem: Arc<ExternalMemory>,
    wgpu_buf: wgpu::Buffer,
    backing: BufferNativeBacking,
    size: u64,
    last_barrier: Mutex<Option<wgpu::SubmissionIndex>>,
    device: wgpu::Device,
}

impl Drop for SharedBuffer {
    fn drop(&mut self) {
        // Poisoning only means a panic elsewhere while storing an index; the value is intact.
        let last = self
            .last_barrier
            .get_mut()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        wait_for_submission(&self.device, last, "SharedBuffer::drop (wait for last ownership barrier)");
    }
}

// SAFETY: `RawDeviceBuffer`, `ExternalMemory`, `wgpu::Buffer`, and Vulkan/DX12 handles are Send.
unsafe impl Send for SharedBuffer {}
// SAFETY: Shared access only reads handles or enqueues driver-synchronized operations.
unsafe impl Sync for SharedBuffer {}

impl SharedBuffer {
    /// Creates a new device-local shared buffer visible to both `wgpu` and `cuda_ctx`.
    pub fn create(
        device: &wgpu::Device,
        cuda_ctx: &Arc<Context>,
        desc: &SharedBufferDescriptor<'_>,
    ) -> Result<Self> {
        validate_buffer_descriptor(desc)?;
        match verify_device_interop_support(device)? {
            InteropBackend::Vulkan => Self::create_vulkan(device, cuda_ctx, desc),
            #[cfg(windows)]
            InteropBackend::Dx12 => Self::create_dx12(device, cuda_ctx, desc),
        }
    }

    fn create_vulkan(
        device: &wgpu::Device,
        cuda_ctx: &Arc<Context>,
        desc: &SharedBufferDescriptor<'_>,
    ) -> Result<Self> {
        // SAFETY: Read-only HAL access to `wgpu`'s Vulkan device handles.
        let hal_dev =
            unsafe { device.as_hal::<Vulkan>() }.ok_or_else(|| Error::BackendMismatch {
                expected: "Vulkan",
                found: device.adapter_info().backend,
            })?;
        let raw_dev = hal_dev.raw_device();
        let instance = hal_dev.shared_instance().raw_instance();
        let phys = hal_dev.raw_physical_device();
        let queue_family = hal_dev.queue_family_index();

        let handle_type = vk_external_memory_handle_type();
        let vk_usage = map_vk_buffer_usage(desc.usage);
        let mut ext_buf_info =
            vk::ExternalMemoryBufferCreateInfo::default().handle_types(handle_type);
        let buf_info = vk::BufferCreateInfo::default()
            .size(desc.size)
            .usage(vk_usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .push_next(&mut ext_buf_info);

        // SAFETY: `raw_dev` is live and `buf_info` uses verified usage flags.
        let vk_buf = unsafe { raw_dev.create_buffer(&buf_info, None) }
            .map_err(|e| Error::vulkan("vkCreateBuffer", e))?;

        let (vk_mem, mem_reqs) = alloc_bind_exportable_dedicated(
            instance,
            raw_dev,
            phys,
            DedicatedResource::Buffer(vk_buf),
        )?;

        // Wrap into `VkBufferBacking` immediately so any subsequent error cleans up via Drop.
        let vk_backing = Arc::new(VkBufferBacking {
            device: raw_dev.clone(),
            buffer: vk_buf,
            memory: vk_mem,
        });

        let exported = export_vk_memory(instance, raw_dev, vk_mem)?;
        let ext_mem = exported.import_cuda_memory(
            cuda_ctx,
            ExternalMemoryKind::VulkanOpaque,
            mem_reqs.size,
            true,
        )?;
        let cuda_buf = ext_mem.mapped_buffer(0, desc.size)?;

        let drop_backing = Arc::clone(&vk_backing);
        // SAFETY: `vk_buf` is bound to valid `vk_mem`, outlives HAL `Buffer` until
        // `drop_backing` drops inside `wgpu_hal`'s `DropCallback`.
        let hal_buf = unsafe {
            wgpu::hal::vulkan::Buffer::from_raw_externally_owned(
                vk_buf,
                Box::new(move || drop(drop_backing)),
            )
        };
        let wgpu_desc = wgpu::BufferDescriptor {
            label: desc.label,
            size: desc.size,
            usage: desc.usage,
            mapped_at_creation: false,
        };
        // SAFETY: `hal_buf` matches `wgpu_desc` and belongs to `device`.
        let wgpu_buf = unsafe { device.create_buffer_from_hal::<Vulkan>(hal_buf, &wgpu_desc) };

        Ok(Self {
            cuda_buf,
            ext_mem,
            wgpu_buf,
            backing: BufferNativeBacking::Vulkan {
                vk: vk_backing,
                queue_family,
            },
            size: desc.size,
            last_barrier: Mutex::new(None),
            device: device.clone(),
        })
    }

    #[cfg(windows)]
    fn create_dx12(
        device: &wgpu::Device,
        cuda_ctx: &Arc<Context>,
        desc: &SharedBufferDescriptor<'_>,
    ) -> Result<Self> {
        // SAFETY: Read-only HAL access to `wgpu`'s DX12 device handle.
        let hal_dev = unsafe { device.as_hal::<Dx12>() }.ok_or_else(|| Error::BackendMismatch {
            expected: "Dx12",
            found: device.adapter_info().backend,
        })?;
        let raw_device = hal_dev.raw_device();

        let mut flags = Direct3D12::D3D12_RESOURCE_FLAG_NONE;
        // wgpu-core binds INDIRECT (validation) and QUERY_RESOLVE (timestamp normalization)
        // buffers as read-write storage too.
        if desc.usage.intersects(
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::INDIRECT
                | wgpu::BufferUsages::QUERY_RESOLVE,
        ) {
            flags |= Direct3D12::D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS;
        }
        let res_desc = Direct3D12::D3D12_RESOURCE_DESC {
            Dimension: Direct3D12::D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: desc.size,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: Dxgi::Common::DXGI_FORMAT_UNKNOWN,
            SampleDesc: Dxgi::Common::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: Direct3D12::D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: flags,
        };

        // SAFETY: Queries committed resource allocation size required by `cuImportExternalMemory`.
        let alloc_info =
            unsafe { raw_device.GetResourceAllocationInfo(0, core::slice::from_ref(&res_desc)) };
        if alloc_info.SizeInBytes == u64::MAX || alloc_info.SizeInBytes < desc.size {
            return Err(Error::InvalidInput(
                "D3D12 rejected buffer descriptor allocation info",
            ));
        }

        let resource = create_shared_committed_resource(
            raw_device,
            &res_desc,
            "ID3D12Device::CreateCommittedResource(Buffer)",
        )?;

        let exported =
            export_d3d12_shared_handle(raw_device, &resource, "ID3D12Resource->ID3D12DeviceChild")?;
        let ext_mem = exported.import_cuda_memory(
            cuda_ctx,
            ExternalMemoryKind::D3d12Resource,
            alloc_info.SizeInBytes,
            true,
        )?;
        let cuda_buf = ext_mem.mapped_buffer(0, desc.size)?;

        // SAFETY: `resource.clone()` increments COM refcount for `wgpu-hal::dx12::Buffer`.
        let hal_buf =
            unsafe { wgpu::hal::dx12::Device::buffer_from_raw(resource.clone(), desc.size) };
        let wgpu_desc = wgpu::BufferDescriptor {
            label: desc.label,
            size: desc.size,
            usage: desc.usage,
            mapped_at_creation: false,
        };
        // SAFETY: `hal_buf` matches `wgpu_desc` and belongs to `device`.
        let wgpu_buf = unsafe { device.create_buffer_from_hal::<Dx12>(hal_buf, &wgpu_desc) };

        Ok(Self {
            cuda_buf,
            ext_mem,
            wgpu_buf,
            backing: BufferNativeBacking::Dx12 {
                _resource: resource,
            },
            size: desc.size,
            last_barrier: Mutex::new(None),
            device: device.clone(),
        })
    }

    /// Returns the `wgpu::Buffer` view of this shared allocation.
    #[inline]
    pub fn wgpu_buffer(&self) -> &wgpu::Buffer {
        &self.wgpu_buf
    }

    /// Returns the CUDA [`RawDeviceBuffer`] view of this shared allocation.
    #[inline]
    pub fn cuda_buffer(&self) -> &RawDeviceBuffer {
        &self.cuda_buf
    }

    /// Returns the CUDA device pointer (`CUdeviceptr`) for kernel launches or async copies.
    #[inline]
    pub fn cu_deviceptr(&self) -> cuda_bindings::CUdeviceptr {
        self.cuda_buf.cu_deviceptr()
    }

    /// Returns the underlying `cuda_core_ext::ExternalMemory` handle.
    #[inline]
    pub fn external_memory(&self) -> &Arc<ExternalMemory> {
        &self.ext_mem
    }

    /// Returns the logical buffer byte size.
    #[inline]
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Submits an optional Vulkan queue-family ownership release barrier from `wgpu`'s
    /// queue family to `VK_QUEUE_FAMILY_EXTERNAL` while waiting on `wait` and signaling `signal`.
    ///
    /// On DX12 this routes `wait` and `signal` through `queue.submit(empty)` so pending
    /// `wgpu` writes flush and `wait` executes before `signal`.
    ///
    /// # Queue Synchronization
    ///
    /// On both backends the work goes through `wgpu::Queue::submit` (Vulkan: the barrier is
    /// recorded into a `wgpu` command encoder), so it is safe next to other threads' submits and
    /// presents, and `wgpu`'s pending `write_buffer` uploads are submitted before the barrier.
    /// `wait` / `signal` are staged on `queue` just before that submit; a submit from another
    /// thread in between would consume them, so serialize submits that use semaphores.
    pub fn release_to_cuda(
        &self,
        queue: &wgpu::Queue,
        wait: Option<(&InteropSemaphore, u64)>,
        signal: Option<(&InteropSemaphore, u64)>,
    ) -> Result<()> {
        self.submit_ownership_barrier(queue, false, wait, signal)
    }

    /// Submits an optional Vulkan queue-family ownership acquire barrier from
    /// `VK_QUEUE_FAMILY_EXTERNAL` back into `wgpu`'s queue family.
    ///
    /// # Queue Synchronization
    ///
    /// See [`Self::release_to_cuda`].
    pub fn acquire_from_cuda(
        &self,
        queue: &wgpu::Queue,
        wait: Option<(&InteropSemaphore, u64)>,
        signal: Option<(&InteropSemaphore, u64)>,
    ) -> Result<()> {
        self.submit_ownership_barrier(queue, true, wait, signal)
    }

    fn submit_ownership_barrier(
        &self,
        queue: &wgpu::Queue,
        acquire_to_wgpu: bool,
        wait: Option<(&InteropSemaphore, u64)>,
        signal: Option<(&InteropSemaphore, u64)>,
    ) -> Result<()> {
        match &self.backing {
            BufferNativeBacking::Vulkan { vk, queue_family } => {
                let xfer = ownership_transfer(acquire_to_wgpu, *queue_family);

                let barrier = vk::BufferMemoryBarrier::default()
                    .src_access_mask(xfer.src_access)
                    .dst_access_mask(xfer.dst_access)
                    .src_queue_family_index(xfer.src_family)
                    .dst_queue_family_index(xfer.dst_family)
                    .buffer(vk.buffer)
                    .offset(0)
                    .size(self.size);

                let ((), index) = submit_vulkan_raw(&self.device, queue, wait, signal, |cmd| {
                    // SAFETY: `cmd` is recording, and `barrier` names this backing's live buffer,
                    // which `Drop` keeps alive until this submission completes.
                    unsafe {
                        vk.device.cmd_pipeline_barrier(
                            cmd,
                            vk::PipelineStageFlags::ALL_COMMANDS,
                            vk::PipelineStageFlags::ALL_COMMANDS,
                            vk::DependencyFlags::empty(),
                            &[],
                            core::slice::from_ref(&barrier),
                            &[],
                        );
                    }
                    Ok(())
                })?;
                *self
                    .last_barrier
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner) = Some(index);
                Ok(())
            }
            #[cfg(windows)]
            BufferNativeBacking::Dx12 { .. } => {
                crate::platform::flush_wgpu_wait_signal(queue, wait, signal)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_descriptor_validation_rejects_zero_mappable_and_acceleration_inputs() {
        assert!(
            validate_buffer_descriptor(&SharedBufferDescriptor {
                label: None,
                size: 0,
                usage: wgpu::BufferUsages::STORAGE,
            })
            .is_err()
        );

        assert!(
            validate_buffer_descriptor(&SharedBufferDescriptor {
                label: None,
                size: 256,
                usage: wgpu::BufferUsages::empty(),
            })
            .is_err()
        );

        assert!(
            validate_buffer_descriptor(&SharedBufferDescriptor {
                label: None,
                size: 256,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::MAP_READ,
            })
            .is_err()
        );

        assert!(
            validate_buffer_descriptor(&SharedBufferDescriptor {
                label: None,
                size: 256,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::MAP_WRITE,
            })
            .is_err()
        );

        // Neither WebGPU nor Vulkan constrains a uniform buffer's size to 256 bytes (only
        // dynamic binding offsets are aligned).
        validate_buffer_descriptor(&SharedBufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM,
        })
        .unwrap();

        assert!(
            validate_buffer_descriptor(&SharedBufferDescriptor {
                label: None,
                size: 256,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::BLAS_INPUT,
            })
            .is_err()
        );

        validate_buffer_descriptor(&SharedBufferDescriptor {
            label: Some("valid"),
            size: 512,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::UNIFORM,
        })
        .unwrap();
    }
}
