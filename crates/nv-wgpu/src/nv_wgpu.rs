//! Zero-copy CUDA <-> `wgpu` 30 external memory and timeline semaphore interop.
//!
//! Why: Bridges `wgpu` rendering/compute pipelines (Vulkan on Windows & Linux,
//! DX12 on Windows) with CUDA (`cuda-core` + `cuda-ext`) so GPU buffers and
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
//! - [`texture`]: Zero-copy [`SharedTexture`] (`wgpu::Texture` <-> `cuda_ext::MipmappedArray`),
//!   exposing [`SharedSurfaceObject`] and [`cuda_ext::TextureObject`] views.
//! - [`sync`]: GPU-side timeline [`InteropSemaphore`] (Vulkan timeline semaphore
//!   & DX12 shared `ID3D12Fence`).
//! - [`format`]: Explicit `wgpu::TextureFormat` <-> CUDA/Vulkan/DXGI mapping table
//!   and 256-byte (`COPY_BYTES_PER_ROW_ALIGNMENT`) staging layout helpers.
//!
//! Uses `cuda-core`, `cuda-ext`, `gpu-info-rs`, `wgpu`, `ash`, and OS handle APIs
//! (`Win32` NT handles on Windows, `OPAQUE_FD` on Unix).

use std::sync::Arc;

use cuda_core::Context;

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
    GpuIdentity, InteropBackend, detect_backend, find_matching_cuda_device,
    open_vulkan_device_with_interop, pair_shared_gpu, required_vulkan_extensions,
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
};

/// Paired `wgpu::Device` + `wgpu::Queue` + matching `cuda_core::Context` verified
/// for zero-copy external memory and timeline semaphore interop.
#[derive(Clone)]
pub struct InteropContext {
    backend: InteropBackend,
    device: wgpu::Device,
    queue: wgpu::Queue,
    cuda_ctx: Arc<Context>,
}

impl std::fmt::Debug for InteropContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InteropContext")
            .field("backend", &self.backend)
            .field("cuda_ordinal", &self.cuda_ctx.ordinal())
            .finish_non_exhaustive()
    }
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
        Ok(Self {
            backend,
            device,
            queue,
            cuda_ctx,
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
        Ok(Self {
            backend,
            device,
            queue,
            cuda_ctx,
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
        let (device, queue) = open_vulkan_device_with_interop(adapter, features, limits, label)?;
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
        Ok(Self {
            backend,
            device: shared.device.clone(),
            queue: shared.queue.clone(),
            cuda_ctx,
        })
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
        SharedBuffer::create(&self.device, &self.cuda_ctx, desc)
    }

    /// Creates a zero-copy [`SharedTexture`] shared between `wgpu` and CUDA.
    pub fn create_texture(&self, desc: &SharedTextureDescriptor<'_>) -> Result<SharedTexture> {
        SharedTexture::create(&self.device, &self.queue, &self.cuda_ctx, desc)
    }
}
