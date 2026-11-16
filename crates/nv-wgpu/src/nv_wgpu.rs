//! Zero-copy CUDA <-> `wgpu` 30 external memory and timeline semaphore interop.
//!
//! Why: Bridges `wgpu` rendering/compute pipelines (Vulkan on Windows & Linux,
//! DX12 on Windows) with CUDA (`cuda-core` + `cuda-core-ext`) so GPU buffers and
//! textures can be produced and consumed in device-local VRAM with zero host
//! copies and strict GPU-side timeline synchronization.
//!
//! Architecture:
//! - [`device`] / [`InteropContext`]: Detects the `wgpu` backend (`Vulkan` or
//!   Windows `Dx12`), checks required external-memory / timeline-semaphore
//!   extensions without silent downgrades, opens interop-ready Vulkan devices via
//!   `wgpu_hal::vulkan::Adapter::open_with_callback`, matches physical GPUs by
//!   UUID/LUID against CUDA, and composes cleanly with `gpu_info::SharedGpu`.
//! - [`buffer`]: Zero-copy [`SharedBuffer`] (`wgpu::Buffer` <-> `cuda_core::RawDeviceBuffer`).
//! - [`texture`]: Zero-copy [`SharedTexture`] (`wgpu::Texture` <-> `cuda_core_ext::MipmappedArray`),
//!   exposing [`SharedSurfaceObject`] and [`cuda_core_ext::TextureObject`] views.
//! - [`sync`]: GPU-side timeline [`InteropSemaphore`] (Vulkan timeline semaphore
//!   & DX12 shared `ID3D12Fence`).
//! - [`format`]: Explicit `wgpu::TextureFormat` <-> CUDA/Vulkan/DXGI mapping table
//!   and 256-byte (`COPY_BYTES_PER_ROW_ALIGNMENT`) staging layout helpers.
//!
//! Uses `cuda-core`, `cuda-core-ext`, `gpu-info-rs`, `wgpu`, `ash`, and OS handle APIs
//! (`Win32` NT handles on Windows, `OPAQUE_FD` on Unix).

use std::ffi::c_void;
use std::sync::Arc;

use ash::vk::Handle as _;
use cuda_core::Context;
use wgpu::hal::api::Vulkan;

pub mod buffer;
pub mod device;
pub mod error;
pub mod format;
mod platform;
pub mod sync;
pub mod texture;

pub use buffer::{SharedBuffer, SharedBufferDescriptor, validate_buffer_descriptor};
#[cfg(windows)]
pub use device::wgpu_dx12_luid;
pub use device::{
    GpuIdentity, InteropBackend, create_vulkan_instance_with_extensions, detect_backend,
    find_matching_cuda_device, open_vulkan_device_with_interop,
    open_vulkan_device_with_interop_extensions, pair_shared_gpu, required_vulkan_extensions,
    verify_device_interop_support, verify_matching_cuda_device, wgpu_gpu_identity,
    wgpu_vulkan_uuid,
};
pub use error::{Error, Result};
pub use format::{
    COPY_BYTES_PER_ROW_ALIGNMENT, FormatMapping, SUPPORTED_FORMATS, align_up_u32, align_up_u64,
    aligned_row_pitch, aligned_staging_bytes, build_cuda_array_desc, format_mapping,
    validate_copy_layout,
};
pub use sync::InteropSemaphore;
pub use texture::{
    SharedSurfaceObject, SharedTexture, SharedTextureDescriptor, validate_texture_descriptor,
    vk_image_view_raw,
};

/// Vulkan image layout transitions recorded via [`InteropContext::vk_image_barrier`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkImageTransition {
    /// `TRANSFER_DST_OPTIMAL` -> `SHADER_READ_ONLY_OPTIMAL` (compute shader read).
    TransferDstToShaderRead,
    /// `SHADER_READ_ONLY_OPTIMAL` -> `TRANSFER_DST_OPTIMAL` (restore for `wgpu` copy).
    ShaderReadToTransferDst,
    /// `UNDEFINED` -> `GENERAL` (compute storage read/write).
    UndefinedToGeneral,
    /// `GENERAL` -> `SHADER_READ_ONLY_OPTIMAL` (fragment shader sample).
    GeneralToShaderRead,
}

/// Raw Vulkan handles extracted from an [`InteropContext`] for external native
/// SDKs (such as NVIDIA NGX).
#[derive(Debug, Clone, Copy)]
pub struct RawVulkanHandles {
    /// Raw `VkInstance` handle.
    pub instance: *mut c_void,
    /// Raw `VkPhysicalDevice` handle.
    pub physical_device: *mut c_void,
    /// Raw `VkDevice` handle.
    pub device: *mut c_void,
    /// Raw `PFN_vkGetInstanceProcAddr` function pointer.
    pub get_instance_proc_addr: *const c_void,
    /// Raw `PFN_vkGetDeviceProcAddr` function pointer.
    pub get_device_proc_addr: *const c_void,
    /// Raw `VkQueue` handle (`wgpu` queue, or dedicated barrier queue if configured).
    pub queue: *mut c_void,
    /// Vulkan queue family index of `queue`.
    pub queue_family_index: u32,
}

/// Paired `wgpu::Device` + `wgpu::Queue` + matching `cuda_core::Context` verified
/// for zero-copy external memory and timeline semaphore interop.
#[derive(Clone)]
pub struct InteropContext {
    backend: InteropBackend,
    device: wgpu::Device,
    queue: wgpu::Queue,
    cuda_ctx: Arc<Context>,
    vulkan_barrier_queue: Option<ash::vk::Queue>,
    vulkan_cmd_runner: Option<Arc<platform::VkBarrierRunner>>,
}

impl std::fmt::Debug for InteropContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InteropContext")
            .field("backend", &self.backend)
            .field("cuda_ordinal", &self.cuda_ctx.ordinal())
            .finish_non_exhaustive()
    }
}

fn build_vulkan_cmd_runner(
    backend: InteropBackend,
    device: &wgpu::Device,
) -> Result<Option<Arc<platform::VkBarrierRunner>>> {
    if backend != InteropBackend::Vulkan {
        return Ok(None);
    }
    // SAFETY: Read-only inspection of Vulkan HAL device and queue family index.
    let hal_dev = unsafe { device.as_hal::<Vulkan>() }.ok_or_else(|| Error::BackendMismatch {
        expected: "Vulkan",
        found: device.adapter_info().backend,
    })?;
    let runner =
        platform::VkBarrierRunner::new(hal_dev.raw_device().clone(), hal_dev.queue_family_index())?;
    Ok(Some(Arc::new(runner)))
}

impl InteropContext {
    /// Wraps an existing `wgpu` adapter/device/queue after verifying external
    /// memory/semaphore support and matching it to a CUDA context by UUID/LUID.
    pub fn from_wgpu(
        adapter: &wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue,
    ) -> Result<Self> {
        let backend = verify_device_interop_support(&device)?;
        let cuda_ctx = find_matching_cuda_device(adapter, &device)?;
        let vulkan_cmd_runner = build_vulkan_cmd_runner(backend, &device)?;
        Ok(Self {
            backend,
            device,
            queue,
            cuda_ctx,
            vulkan_barrier_queue: None,
            vulkan_cmd_runner,
        })
    }

    /// Wraps an existing `wgpu` adapter/device/queue with a caller-supplied
    /// CUDA [`Context`], verifying both interop support and physical GPU match.
    pub fn from_wgpu_with_cuda(
        adapter: &wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue,
        cuda_ctx: Arc<Context>,
    ) -> Result<Self> {
        let backend = verify_device_interop_support(&device)?;
        verify_matching_cuda_device(adapter, &device, &cuda_ctx)?;
        let vulkan_cmd_runner = build_vulkan_cmd_runner(backend, &device)?;
        Ok(Self {
            backend,
            device,
            queue,
            cuda_ctx,
            vulkan_barrier_queue: None,
            vulkan_cmd_runner,
        })
    }

    /// Opens a Vulkan `wgpu::Device` from `adapter` with all required external
    /// interop extensions enabled, then pairs it with the matching CUDA device.
    pub fn open_vulkan(
        adapter: &wgpu::Adapter,
        features: wgpu::Features,
        limits: &wgpu::Limits,
        label: Option<&str>,
    ) -> Result<Self> {
        Self::open_vulkan_with_extensions(adapter, features, limits, label, &[])
    }

    /// Opens a Vulkan `wgpu::Device` from `adapter` with all required external
    /// interop extensions plus `extra_device_exts` enabled, then pairs it with
    /// the matching CUDA device.
    pub fn open_vulkan_with_extensions(
        adapter: &wgpu::Adapter,
        features: wgpu::Features,
        limits: &wgpu::Limits,
        label: Option<&str>,
        extra_device_exts: &[&str],
    ) -> Result<Self> {
        let (device, queue) = open_vulkan_device_with_interop_extensions(
            adapter,
            features,
            limits,
            label,
            extra_device_exts,
        )?;
        Self::from_wgpu(adapter, device, queue)
    }

    /// Wraps `gpu_info::SharedGpu` after verifying interop support and pairing
    /// its physical GPU with the matching CUDA [`Context`].
    ///
    /// Returns [`Error::Unsupported`] without downgrading if `shared.device` was
    /// opened without external-interop extensions (e.g. Vulkan without video
    /// decode on an adapter lacking external semaphore win32/fd); callers can
    /// open an interop-ready device from `&shared.adapter` via [`Self::open_vulkan`].
    pub fn from_shared_gpu(shared: &gpu_info::SharedGpu) -> Result<Self> {
        let (backend, cuda_ctx) = pair_shared_gpu(shared)?;
        let vulkan_cmd_runner = build_vulkan_cmd_runner(backend, &shared.device)?;
        Ok(Self {
            backend,
            device: shared.device.clone(),
            queue: shared.queue.clone(),
            cuda_ctx,
            vulkan_barrier_queue: None,
            vulkan_cmd_runner,
        })
    }

    /// Configures a dedicated Vulkan barrier queue `(family, index)` (in `wgpu`'s
    /// queue family, distinct from `wgpu`'s own queue) for ownership/layout barriers.
    pub fn with_vulkan_barrier_queue(mut self, family: u32, index: u32) -> Result<Self> {
        let vk_q = platform::resolve_vulkan_barrier_queue(&self.device, family, index)?;
        self.vulkan_barrier_queue = Some(vk_q);
        Ok(self)
    }

    /// Returns the verified graphics interop backend (`Vulkan` or Windows `Dx12`).
    pub fn backend(&self) -> InteropBackend {
        self.backend
    }

    /// Returns the paired `wgpu::Device`.
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// Returns the paired `wgpu::Queue`.
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    /// Returns the matched `cuda_core::Context`.
    pub fn cuda_context(&self) -> &Arc<Context> {
        &self.cuda_ctx
    }

    /// Re-verifies that the wrapped `wgpu::Device` still exposes interop support.
    pub fn verify(&self) -> Result<InteropBackend> {
        verify_device_interop_support(&self.device)
    }

    /// Creates a shared timeline [`InteropSemaphore`] initialized to `initial_value`.
    pub fn create_semaphore(&self, initial_value: u64) -> Result<InteropSemaphore> {
        InteropSemaphore::new(&self.device, &self.cuda_ctx, initial_value)
    }

    /// Creates a zero-copy [`SharedBuffer`] shared between `wgpu` and CUDA.
    pub fn create_buffer(&self, desc: &SharedBufferDescriptor<'_>) -> Result<SharedBuffer> {
        SharedBuffer::create_with_barrier_queue(
            &self.device,
            &self.cuda_ctx,
            desc,
            self.vulkan_barrier_queue,
        )
    }

    /// Creates a zero-copy [`SharedTexture`] shared between `wgpu` and CUDA.
    pub fn create_texture(&self, desc: &SharedTextureDescriptor<'_>) -> Result<SharedTexture> {
        SharedTexture::create_with_barrier_queue(
            &self.device,
            &self.queue,
            &self.cuda_ctx,
            desc,
            self.vulkan_barrier_queue,
        )
    }

    /// Returns `true` when this context has a dedicated Vulkan barrier queue
    /// configured so ownership barriers do not submit on `wgpu`'s queue.
    #[must_use]
    pub fn has_dedicated_vulkan_barrier_queue(&self) -> bool {
        self.vulkan_barrier_queue.is_some()
    }

    /// Flushes optional staged wait/signal semaphore operations on `wgpu`'s queue
    /// via `wgpu::Queue::submit(empty)` (thread-safe with other `wgpu::Queue::submit` callers).
    pub fn flush_wgpu_wait_signal(
        &self,
        wait: Option<(&InteropSemaphore, u64)>,
        signal: Option<(&InteropSemaphore, u64)>,
    ) -> Result<()> {
        platform::flush_wgpu_wait_signal(&self.queue, wait, signal)
    }

    /// Extracts raw Vulkan handles (`VkInstance`, `VkPhysicalDevice`, `VkDevice`,
    /// `VkQueue`, queue family index, and `vkGet*ProcAddr`) when backed by Vulkan,
    /// or returns `None` on DX12.
    pub fn raw_vulkan_handles(&self) -> Option<RawVulkanHandles> {
        if self.backend != InteropBackend::Vulkan {
            return None;
        }
        // SAFETY: Read-only HAL inspection of Vulkan device and queue handles.
        let hal_dev = unsafe { self.device.as_hal::<Vulkan>() }?;
        let shared_inst = hal_dev.shared_instance();
        let raw_inst = shared_inst.raw_instance();
        let raw_queue = if let Some(q) = self.vulkan_barrier_queue {
            q
        } else {
            // SAFETY: Read-only HAL inspection of `wgpu`'s Vulkan queue handle.
            let hal_q = unsafe { self.queue.as_hal::<Vulkan>() }?;
            hal_q.as_raw()
        };
        Some(RawVulkanHandles {
            instance: raw_inst.handle().as_raw() as usize as *mut c_void,
            physical_device: hal_dev.raw_physical_device().as_raw() as usize as *mut c_void,
            device: hal_dev.raw_device().handle().as_raw() as usize as *mut c_void,
            get_instance_proc_addr: shared_inst.entry().static_fn().get_instance_proc_addr
                as *const c_void,
            get_device_proc_addr: raw_inst.fp_v1_0().get_device_proc_addr as *const c_void,
            queue: raw_queue.as_raw() as usize as *mut c_void,
            queue_family_index: hal_dev.queue_family_index(),
        })
    }

    /// Allocates/reuses a primary `VkCommandBuffer`, lets `f` record commands into
    /// it (`*mut c_void` = `VkCommandBuffer`), submits it to the device's Vulkan
    /// queue, and waits for completion via fence.
    pub fn submit_vulkan_commands<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(*mut c_void) -> Result<R>,
    {
        if self.backend != InteropBackend::Vulkan {
            return Err(Error::BackendMismatch {
                expected: "Vulkan",
                found: self.device.adapter_info().backend,
            });
        }
        let runner = self
            .vulkan_cmd_runner
            .as_deref()
            .ok_or_else(|| Error::BackendMismatch {
                expected: "Vulkan",
                found: self.device.adapter_info().backend,
            })?;
        platform::with_barrier_vk_queue(&self.queue, self.vulkan_barrier_queue, |raw_q| {
            runner.record_and_submit_wait(raw_q, |cmd| f(cmd.as_raw() as usize as *mut c_void))
        })
    }

    /// Records a `vkCmdPipelineBarrier` image layout transition into `cmd_buffer`
    /// (`*mut c_void` = `VkCommandBuffer`) for `image` (`*mut c_void` = `VkImage`).
    ///
    /// # Safety
    ///
    /// `cmd_buffer` must be a valid recording `VkCommandBuffer` from
    /// [`Self::submit_vulkan_commands`], and `image` must be a live `VkImage` on `self.device`.
    pub unsafe fn vk_image_barrier(
        &self,
        cmd_buffer: *mut c_void,
        image: *mut c_void,
        transition: VkImageTransition,
    ) -> Result<()> {
        if self.backend != InteropBackend::Vulkan {
            return Err(Error::BackendMismatch {
                expected: "Vulkan",
                found: self.device.adapter_info().backend,
            });
        }
        let runner = self
            .vulkan_cmd_runner
            .as_deref()
            .ok_or_else(|| Error::BackendMismatch {
                expected: "Vulkan",
                found: self.device.adapter_info().backend,
            })?;
        let cmd = ash::vk::CommandBuffer::from_raw(cmd_buffer as usize as u64);
        let img = ash::vk::Image::from_raw(image as usize as u64);
        let (src_stage, dst_stage, src_access, dst_access, old_layout, new_layout) =
            match transition {
                VkImageTransition::TransferDstToShaderRead => (
                    ash::vk::PipelineStageFlags::TRANSFER,
                    ash::vk::PipelineStageFlags::COMPUTE_SHADER,
                    ash::vk::AccessFlags::TRANSFER_WRITE,
                    ash::vk::AccessFlags::SHADER_READ,
                    ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                ),
                VkImageTransition::ShaderReadToTransferDst => (
                    ash::vk::PipelineStageFlags::COMPUTE_SHADER,
                    ash::vk::PipelineStageFlags::TRANSFER,
                    ash::vk::AccessFlags::SHADER_READ,
                    ash::vk::AccessFlags::TRANSFER_WRITE,
                    ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                    ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                ),
                VkImageTransition::UndefinedToGeneral => (
                    ash::vk::PipelineStageFlags::TOP_OF_PIPE,
                    ash::vk::PipelineStageFlags::COMPUTE_SHADER,
                    ash::vk::AccessFlags::empty(),
                    ash::vk::AccessFlags::SHADER_WRITE | ash::vk::AccessFlags::SHADER_READ,
                    ash::vk::ImageLayout::UNDEFINED,
                    ash::vk::ImageLayout::GENERAL,
                ),
                VkImageTransition::GeneralToShaderRead => (
                    ash::vk::PipelineStageFlags::COMPUTE_SHADER,
                    ash::vk::PipelineStageFlags::FRAGMENT_SHADER,
                    ash::vk::AccessFlags::SHADER_WRITE,
                    ash::vk::AccessFlags::SHADER_READ,
                    ash::vk::ImageLayout::GENERAL,
                    ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                ),
            };
        let barrier = ash::vk::ImageMemoryBarrier::default()
            .src_access_mask(src_access)
            .dst_access_mask(dst_access)
            .old_layout(old_layout)
            .new_layout(new_layout)
            .src_queue_family_index(ash::vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(ash::vk::QUEUE_FAMILY_IGNORED)
            .image(img)
            .subresource_range(ash::vk::ImageSubresourceRange {
                aspect_mask: ash::vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });
        // SAFETY: Caller guarantees `cmd` is a valid recording `VkCommandBuffer` and `img` is live.
        unsafe {
            runner.vk_device().cmd_pipeline_barrier(
                cmd,
                src_stage,
                dst_stage,
                ash::vk::DependencyFlags::empty(),
                &[],
                &[],
                core::slice::from_ref(&barrier),
            );
        }
        Ok(())
    }
}
