//! Zero-copy shared GPU textures (`wgpu::Texture` <-> `cuda_ext::MipmappedArray`).
//!
//! Why: Shares 2D/3D/layered/cubemap GPU images directly between `wgpu` rendering or
//! compute passes and CUDA arrays/surfaces/textures without copying pixels through RAM:
//! - On Vulkan: allocates an exportable dedicated `VkImage` + `VkDeviceMemory`, optionally
//!   transitions it from `UNDEFINED` to the layout matching `initial_state`, wraps it via
//!   `wgpu::hal::vulkan::Device::texture_from_raw(..., TextureMemory::External)` +
//!   `create_texture_from_hal`, and maps a `cuda_ext::MipmappedArray` over the imported memory.
//! - On DX12 (Windows): allocates a committed shared `ID3D12Resource` texture
//!   (`D3D12_HEAP_FLAG_SHARED` + `D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS`), wraps
//!   via `wgpu::hal::dx12::Device::texture_from_raw`, and imports via `D3d12Resource`.
//!
//! Drop order ensures CUDA arrays release before `ExternalMemory`, before `wgpu::Texture`,
//! before the underlying `VkImage`/`VkDeviceMemory` or `ID3D12Resource`.
//!
//! Used by callers and GPU interop tests.

use std::sync::Arc;

use ash::vk;
use cuda_core::{Context, IntoResult};
use cuda_ext::array::LevelRef;
use cuda_ext::{ArrayDesc, ExternalMemory, MipmappedArray, TextureDesc, TextureObject};
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D12;
use wgpu::hal::api::Vulkan;
#[cfg(windows)]
use wgpu::hal::api::Dx12;

use crate::device::{InteropBackend, verify_device_interop_support};
use crate::error::{Error, Result};
use crate::format::{FormatMapping, build_cuda_array_desc};
use crate::platform::{
    ExternalMemoryKind, VkBarrierRunner, export_vk_memory, find_vk_memory_type_index,
    map_hal_texture_uses, map_vk_image_usage, vk_external_memory_handle_type,
    vk_layout_for_texture_uses,
};
use crate::sync::InteropSemaphore;

#[cfg(windows)]
use crate::platform::export_d3d12_shared_handle;

/// Descriptor for creating a zero-copy shared texture between `wgpu` and CUDA.
#[derive(Debug, Clone)]
pub struct SharedTextureDescriptor<'a> {
    /// Optional debug label attached to the wrapped `wgpu::Texture`.
    pub label: Option<&'a str>,
    /// Texture pixel/voxel extent (`width`, `height`, `depth_or_array_layers`).
    pub size: wgpu::Extent3d,
    /// Number of mipmap levels (`>= 1`).
    pub mip_level_count: u32,
    /// Texture dimensionality (`D1`, `D2`, `D3`).
    pub dimension: wgpu::TextureDimension,
    /// `wgpu::TextureFormat` validated against [`crate::format::SUPPORTED_FORMATS`].
    pub format: wgpu::TextureFormat,
    /// `wgpu` texture usage flags.
    pub usage: wgpu::TextureUsages,
    /// Enables `CU_ARRAY3D_SURFACE_LDST` on the CUDA array so CUDA kernels can bind a
    /// [`SharedSurfaceObject`] (`surf2Dwrite`/`surf2Dread`).
    pub cuda_surface: bool,
    /// Marks a 2D texture array with `6 * N` square faces as a CUDA/Vulkan cubemap.
    pub cubemap: bool,
    /// Initial `wgpu` layout/usage state reported to `create_texture_from_hal`.
    ///
    /// When set to anything other than [`wgpu::TextureUses::UNINITIALIZED`] on Vulkan,
    /// creation submits a one-time transition from `VK_IMAGE_LAYOUT_UNDEFINED` to the
    /// layout corresponding to `initial_state` so first-use barriers never discard prior
    /// CUDA writes.
    pub initial_state: wgpu::TextureUses,
}

impl<'a> SharedTextureDescriptor<'a> {
    /// Convenience constructor for a single-mip 2D shared texture (`initial_state = STORAGE_READ_WRITE`
    /// when `cuda_surface` or `STORAGE_BINDING` is enabled, otherwise `UNINITIALIZED`).
    pub fn d2(
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsages,
        cuda_surface: bool,
    ) -> Self {
        let initial_state = if cuda_surface || usage.contains(wgpu::TextureUsages::STORAGE_BINDING) {
            wgpu::TextureUses::STORAGE_READ_WRITE
        } else {
            wgpu::TextureUses::UNINITIALIZED
        };
        Self {
            label: None,
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
            cuda_surface,
            cubemap: false,
            initial_state,
        }
    }
}

/// Validates [`SharedTextureDescriptor`] and computes its [`FormatMapping`] + [`ArrayDesc`].
pub fn validate_texture_descriptor(
    desc: &SharedTextureDescriptor<'_>,
) -> Result<(FormatMapping, ArrayDesc)> {
    if desc.usage.is_empty() {
        return Err(Error::InvalidInput("shared texture usage must be non-empty"));
    }
    if desc.size.depth_or_array_layers > u32::from(u16::MAX)
        || desc.mip_level_count > u32::from(u16::MAX)
    {
        return Err(Error::InvalidInput(
            "texture layers and mip_level_count must fit in u16",
        ));
    }
    build_cuda_array_desc(
        desc.size,
        desc.dimension,
        desc.format,
        desc.mip_level_count,
        desc.cuda_surface,
        desc.cubemap,
    )
}

/// CUDA surface object bound to a single mip level of a [`SharedTexture`].
///
/// Holds an [`Arc`] clone of the underlying [`MipmappedArray`] (which retains
/// [`ExternalMemory`]) so the backing allocation cannot be freed while this
/// surface view is alive.
pub struct SharedSurfaceObject {
    raw: cuda_bindings::CUsurfObject,
    ctx: Arc<Context>,
    _backing: Arc<MipmappedArray>,
}

// SAFETY: A `CUsurfObject` is an immutable 64-bit CUDA driver handle usable from
// any thread whose active context matches `ctx`.
unsafe impl Send for SharedSurfaceObject {}
// SAFETY: As above; `SharedSurfaceObject` has no interior mutability.
unsafe impl Sync for SharedSurfaceObject {}

impl SharedSurfaceObject {
    /// Returns the raw 64-bit `CUsurfObject` handle passed to CUDA kernels.
    #[inline]
    pub fn handle(&self) -> u64 {
        self.raw
    }
}

impl Drop for SharedSurfaceObject {
    fn drop(&mut self) {
        if let Err(e) = self.ctx.bind_to_thread() {
            self.ctx.record_err(Err::<(), _>(e));
            return;
        }
        // SAFETY: `self.raw` was created on `self.ctx` and has not been destroyed;
        // callers must ensure all CUDA stream work referencing it has completed.
        let res = unsafe { cuda_bindings::cuSurfObjectDestroy(self.raw) }.result();
        self.ctx.record_err(res);
    }
}

/// Refcounted Vulkan image + dedicated `VkDeviceMemory` owner shared between
/// [`SharedTexture`] and `wgpu-hal`'s `DropCallback` (zero `wgpu::Device` cycle).
struct VkImageBacking {
    device: ash::Device,
    image: vk::Image,
    memory: vk::DeviceMemory,
}

impl Drop for VkImageBacking {
    fn drop(&mut self) {
        // SAFETY: Destroyed only after CUDA `MipmappedArray` + `ExternalMemory` AND `wgpu::Texture`
        // (via HAL `DropCallback`) have released all references.
        unsafe {
            self.device.destroy_image(self.image, None);
            self.device.free_memory(self.memory, None);
        }
    }
}

enum TextureNativeBacking {
    Vulkan {
        vk: Arc<VkImageBacking>,
        barrier_runner: Box<VkBarrierRunner>,
        mip_level_count: u32,
        array_layers: u32,
    },
    #[cfg(windows)]
    Dx12 {
        _resource: Direct3D12::ID3D12Resource,
    },
}

/// Zero-copy GPU texture accessible simultaneously as [`wgpu::Texture`] and
/// [`cuda_ext::MipmappedArray`].
///
/// Field drop order is structural:
/// 1. `cuda_array` (`Arc<MipmappedArray>`)
/// 2. `ext_mem` (`Arc<ExternalMemory>`)
/// 3. `wgpu_tex` (`wgpu::Texture`)
/// 4. `backing` (`Arc<VkImageBacking>` / COM `ID3D12Resource`)
pub struct SharedTexture {
    cuda_array: Arc<MipmappedArray>,
    ext_mem: Arc<ExternalMemory>,
    wgpu_tex: wgpu::Texture,
    backing: TextureNativeBacking,
    mapping: FormatMapping,
    array_desc: ArrayDesc,
}

// SAFETY: `MipmappedArray`, `ExternalMemory`, `wgpu::Texture`, and Vulkan/DX12 handles are Send.
unsafe impl Send for SharedTexture {}
// SAFETY: Shared access only inspects handles or submits driver-synchronized commands.
unsafe impl Sync for SharedTexture {}

impl SharedTexture {
    /// Creates a new device-local shared texture visible to both `wgpu` and `cuda_ctx`.
    ///
    /// # Vulkan Queue Synchronization
    ///
    /// When `desc.initial_state != wgpu::TextureUses::UNINITIALIZED`, creation submits
    /// an initial Vulkan layout transition on `queue` guarded by `nv-wgpu`'s internal
    /// submission mutex (`VK_QUEUE_SUBMIT_LOCK`). Callers must not concurrently invoke
    /// `wgpu::Queue::submit` or `SurfaceTexture::present` on the same `queue`.
    pub fn create(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        cuda_ctx: &Arc<Context>,
        desc: &SharedTextureDescriptor<'_>,
    ) -> Result<Self> {
        let (mapping, array_desc) = validate_texture_descriptor(desc)?;
        match verify_device_interop_support(device)? {
            InteropBackend::Vulkan => {
                Self::create_vulkan(device, queue, cuda_ctx, desc, mapping, array_desc)
            }
            #[cfg(windows)]
            InteropBackend::Dx12 => Self::create_dx12(device, cuda_ctx, desc, mapping, array_desc),
        }
    }

    fn create_vulkan(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        cuda_ctx: &Arc<Context>,
        desc: &SharedTextureDescriptor<'_>,
        mapping: FormatMapping,
        array_desc: ArrayDesc,
    ) -> Result<Self> {
        // SAFETY: Read-only HAL access to `wgpu`'s Vulkan device handles.
        let hal_dev = unsafe { device.as_hal::<Vulkan>() }.ok_or_else(|| Error::BackendMismatch {
            expected: "Vulkan",
            found: device.adapter_info().backend,
        })?;
        let raw_dev = hal_dev.raw_device();
        let instance = hal_dev.shared_instance().raw_instance();
        let phys = hal_dev.raw_physical_device();
        let queue_family = hal_dev.queue_family_index();

        let (image_type, extent, array_layers) = match desc.dimension {
            wgpu::TextureDimension::D1 => (
                vk::ImageType::TYPE_1D,
                vk::Extent3D {
                    width: desc.size.width,
                    height: 1,
                    depth: 1,
                },
                1u32,
            ),
            wgpu::TextureDimension::D2 => (
                vk::ImageType::TYPE_2D,
                vk::Extent3D {
                    width: desc.size.width,
                    height: desc.size.height,
                    depth: 1,
                },
                desc.size.depth_or_array_layers,
            ),
            wgpu::TextureDimension::D3 => (
                vk::ImageType::TYPE_3D,
                vk::Extent3D {
                    width: desc.size.width,
                    height: desc.size.height,
                    depth: desc.size.depth_or_array_layers,
                },
                1u32,
            ),
        };

        let mut flags = vk::ImageCreateFlags::empty();
        if desc.cubemap {
            flags |= vk::ImageCreateFlags::CUBE_COMPATIBLE;
        }
        let handle_type = vk_external_memory_handle_type();
        let mut ext_img_info =
            vk::ExternalMemoryImageCreateInfo::default().handle_types(handle_type);
        let img_info = vk::ImageCreateInfo::default()
            .flags(flags)
            .image_type(image_type)
            .format(mapping.vk_format)
            .extent(extent)
            .mip_levels(desc.mip_level_count)
            .array_layers(array_layers)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(map_vk_image_usage(desc.usage))
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .push_next(&mut ext_img_info);

        // SAFETY: `raw_dev` is live and `img_info` uses validated format/extent/usage.
        let vk_img = unsafe { raw_dev.create_image(&img_info, None) }
            .map_err(|e| Error::vulkan("vkCreateImage", e))?;

        // SAFETY: `vk_img` was just created on `raw_dev`.
        let mem_reqs = unsafe { raw_dev.get_image_memory_requirements(vk_img) };
        let mem_type_idx = match find_vk_memory_type_index(
            instance,
            phys,
            mem_reqs.memory_type_bits,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        ) {
            Ok(idx) => idx,
            Err(err) => {
                // SAFETY: Clean up `vk_img` on early failure before allocation.
                unsafe { raw_dev.destroy_image(vk_img, None) };
                return Err(err);
            }
        };

        let mut export_info = vk::ExportMemoryAllocateInfo::default().handle_types(handle_type);
        let mut dedicated_info = vk::MemoryDedicatedAllocateInfo::default().image(vk_img);
        let alloc_info = vk::MemoryAllocateInfo::default()
            .allocation_size(mem_reqs.size)
            .memory_type_index(mem_type_idx)
            .push_next(&mut export_info)
            .push_next(&mut dedicated_info);

        // SAFETY: `mem_type_idx` and `mem_reqs.size` come directly from `get_image_memory_requirements`.
        let vk_mem = match unsafe { raw_dev.allocate_memory(&alloc_info, None) } {
            Ok(m) => m,
            Err(err) => {
                // SAFETY: Destroy unbacked image before propagating error.
                unsafe { raw_dev.destroy_image(vk_img, None) };
                return Err(Error::vulkan("vkAllocateMemory(Image)", err));
            }
        };

        // SAFETY: `vk_img` has not been bound yet and `vk_mem` matches `mem_reqs`.
        if let Err(err) = unsafe { raw_dev.bind_image_memory(vk_img, vk_mem, 0) } {
            // SAFETY: Destroy unbound image and free memory.
            unsafe {
                raw_dev.destroy_image(vk_img, None);
                raw_dev.free_memory(vk_mem, None);
            }
            return Err(Error::vulkan("vkBindImageMemory", err));
        }

        let vk_backing = Arc::new(VkImageBacking {
            device: raw_dev.clone(),
            image: vk_img,
            memory: vk_mem,
        });
        let barrier_runner = Box::new(VkBarrierRunner::new(raw_dev.clone(), queue_family)?);

        // Perform an initial layout transition from UNDEFINED -> target layout if caller
        // specified a concrete `initial_state` (for instance `STORAGE_READ_WRITE` -> GENERAL
        // before CUDA writes first).
        if desc.initial_state != wgpu::TextureUses::UNINITIALIZED {
            let dst_layout = vk_layout_for_texture_uses(desc.initial_state);
            let subresource = vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(desc.mip_level_count)
                .base_array_layer(0)
                .layer_count(array_layers);
            let barrier = vk::ImageMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::empty())
                .dst_access_mask(vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(dst_layout)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .image(vk_img)
                .subresource_range(subresource);
            crate::platform::with_vk_queue(queue, |raw_q| {
                barrier_runner.submit_image_barrier(
                    raw_q,
                    vk::PipelineStageFlags::TOP_OF_PIPE,
                    vk::PipelineStageFlags::ALL_COMMANDS,
                    &barrier,
                    None,
                    None,
                )
            })?;
        }

        let exported = export_vk_memory(instance, raw_dev, vk_mem)?;
        let ext_mem = exported.import_cuda_memory(
            cuda_ctx,
            ExternalMemoryKind::VulkanOpaque,
            mem_reqs.size,
            true,
        )?;
        let cuda_array =
            Arc::new(ext_mem.mapped_mipmapped_array(0, &array_desc, desc.mip_level_count)?);

        let hal_desc = wgpu::hal::TextureDescriptor {
            label: desc.label,
            size: desc.size,
            mip_level_count: desc.mip_level_count,
            sample_count: 1,
            dimension: desc.dimension,
            format: desc.format,
            usage: map_hal_texture_uses(desc.usage),
            memory_flags: wgpu::hal::MemoryFlags::empty(),
            view_formats: Vec::new(),
        };
        let drop_backing = Arc::clone(&vk_backing);
        // SAFETY: `vk_img` matches `hal_desc`, `view_formats` is empty, and `drop_backing`
        // keeps both `vk_img` and `vk_mem` alive until `DropCallback` runs.
        let hal_tex = unsafe {
            hal_dev.texture_from_raw(
                vk_img,
                &hal_desc,
                Some(Box::new(move || drop(drop_backing))),
                wgpu::hal::vulkan::TextureMemory::External,
            )
        };
        let wgpu_desc = wgpu::TextureDescriptor {
            label: desc.label,
            size: desc.size,
            mip_level_count: desc.mip_level_count,
            sample_count: 1,
            dimension: desc.dimension,
            format: desc.format,
            usage: desc.usage,
            view_formats: &[],
        };
        // SAFETY: `hal_tex` belongs to `device` and matches `wgpu_desc` + `desc.initial_state`.
        let wgpu_tex =
            unsafe { device.create_texture_from_hal::<Vulkan>(hal_tex, &wgpu_desc, desc.initial_state) };

        Ok(Self {
            cuda_array,
            ext_mem,
            wgpu_tex,
            backing: TextureNativeBacking::Vulkan {
                vk: vk_backing,
                barrier_runner,
                mip_level_count: desc.mip_level_count,
                array_layers,
            },
            mapping,
            array_desc,
        })
    }

    #[cfg(windows)]
    fn create_dx12(
        device: &wgpu::Device,
        cuda_ctx: &Arc<Context>,
        desc: &SharedTextureDescriptor<'_>,
        mapping: FormatMapping,
        array_desc: ArrayDesc,
    ) -> Result<Self> {
        // SAFETY: Read-only HAL access to `wgpu`'s DX12 device handle.
        let hal_dev = unsafe { device.as_hal::<Dx12>() }.ok_or_else(|| Error::BackendMismatch {
            expected: "Dx12",
            found: device.adapter_info().backend,
        })?;
        let raw_device = hal_dev.raw_device();

        let heap_props = Direct3D12::D3D12_HEAP_PROPERTIES {
            Type: Direct3D12::D3D12_HEAP_TYPE_DEFAULT,
            CPUPageProperty: Direct3D12::D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: Direct3D12::D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: 1,
            VisibleNodeMask: 1,
        };
        let d3d_dim = match desc.dimension {
            wgpu::TextureDimension::D1 => Direct3D12::D3D12_RESOURCE_DIMENSION_TEXTURE1D,
            wgpu::TextureDimension::D2 => Direct3D12::D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            wgpu::TextureDimension::D3 => Direct3D12::D3D12_RESOURCE_DIMENSION_TEXTURE3D,
        };
        let mut flags = Direct3D12::D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS;
        if desc.usage.contains(wgpu::TextureUsages::RENDER_ATTACHMENT) {
            flags |= Direct3D12::D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET;
        }
        if desc
            .usage
            .intersects(wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::STORAGE_ATOMIC)
        {
            flags |= Direct3D12::D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS;
        }
        let res_desc = Direct3D12::D3D12_RESOURCE_DESC {
            Dimension: d3d_dim,
            Alignment: 0,
            Width: u64::from(desc.size.width),
            Height: desc.size.height,
            DepthOrArraySize: desc.size.depth_or_array_layers as u16,
            MipLevels: desc.mip_level_count as u16,
            Format: mapping.dxgi_format(),
            SampleDesc: windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: Direct3D12::D3D12_TEXTURE_LAYOUT_UNKNOWN,
            Flags: flags,
        };

        // SAFETY: Queries committed texture allocation size for CUDA import.
        let alloc_info = unsafe {
            raw_device.GetResourceAllocationInfo(0, core::slice::from_ref(&res_desc))
        };
        if alloc_info.SizeInBytes == u64::MAX || alloc_info.SizeInBytes == 0 {
            return Err(Error::InvalidInput(
                "D3D12 rejected shared texture descriptor allocation info",
            ));
        }

        let mut resource: Option<Direct3D12::ID3D12Resource> = None;
        // SAFETY: Creates a committed D3D12 texture in a shared default heap.
        unsafe {
            raw_device.CreateCommittedResource(
                &heap_props,
                Direct3D12::D3D12_HEAP_FLAG_SHARED,
                &res_desc,
                Direct3D12::D3D12_RESOURCE_STATE_COMMON,
                None,
                &mut resource,
            )
        }
        .map_err(|e| Error::dx12("ID3D12Device::CreateCommittedResource(Texture)", e))?;
        let resource = resource.ok_or(Error::InvalidHandle {
            source_api: "ID3D12Device::CreateCommittedResource",
        })?;

        let exported =
            export_d3d12_shared_handle(raw_device, &resource, "ID3D12Resource->ID3D12DeviceChild")?;
        let ext_mem = exported.import_cuda_memory(
            cuda_ctx,
            ExternalMemoryKind::D3d12Resource,
            alloc_info.SizeInBytes,
            true,
        )?;
        let cuda_array =
            Arc::new(ext_mem.mapped_mipmapped_array(0, &array_desc, desc.mip_level_count)?);

        // SAFETY: `resource.clone()` increments COM refcount for `wgpu-hal::dx12::Texture`.
        let hal_tex = unsafe {
            wgpu::hal::dx12::Device::texture_from_raw(
                resource.clone(),
                desc.format,
                desc.dimension,
                desc.size,
                desc.mip_level_count,
                1,
            )
        };
        let wgpu_desc = wgpu::TextureDescriptor {
            label: desc.label,
            size: desc.size,
            mip_level_count: desc.mip_level_count,
            sample_count: 1,
            dimension: desc.dimension,
            format: desc.format,
            usage: desc.usage,
            view_formats: &[],
        };
        // SAFETY: `hal_tex` belongs to `device` and starts in `D3D12_RESOURCE_STATE_COMMON`.
        let wgpu_tex =
            unsafe { device.create_texture_from_hal::<Dx12>(hal_tex, &wgpu_desc, desc.initial_state) };

        Ok(Self {
            cuda_array,
            ext_mem,
            wgpu_tex,
            backing: TextureNativeBacking::Dx12 {
                _resource: resource,
            },
            mapping,
            array_desc,
        })
    }

    /// Returns the `wgpu::Texture` handle sharing this image memory.
    #[inline]
    pub fn wgpu_texture(&self) -> &wgpu::Texture {
        &self.wgpu_tex
    }

    /// Returns the imported [`cuda_ext::MipmappedArray`] handle.
    #[inline]
    pub fn cuda_mipmapped_array(&self) -> &Arc<MipmappedArray> {
        &self.cuda_array
    }

    /// Borrows mip level `level` as a [`LevelRef`].
    #[inline]
    pub fn cuda_level(&self, level: u32) -> Result<LevelRef<'_>> {
        Ok(self.cuda_array.level(level)?)
    }

    /// Creates a [`TextureObject`] bound to this mipmapped array.
    pub fn cuda_texture_object(&self, tex_desc: TextureDesc) -> Result<TextureObject> {
        Ok(TextureObject::from_mipmapped(&self.cuda_array, tex_desc)?)
    }

    /// Creates a [`SharedSurfaceObject`] bound to mip level `level`.
    ///
    /// Note: requires [`SharedTextureDescriptor::cuda_surface`] = `true` at creation time.
    pub fn cuda_surface_object(&self, level: u32) -> Result<SharedSurfaceObject> {
        if !self.array_desc.surface {
            return Err(Error::InvalidInput(
                "cuda_surface_object requires cuda_surface = true in SharedTextureDescriptor",
            ));
        }
        let lvl = self.cuda_array.level(level)?;
        let ctx = self.cuda_array.context();
        ctx.bind_to_thread()?;
        // SAFETY: zeroed descriptor is required by CUDA before populating `resType` and `res.array`.
        let mut res: cuda_bindings::CUDA_RESOURCE_DESC = unsafe { std::mem::zeroed() };
        res.resType = cuda_bindings::CUresourcetype_enum_CU_RESOURCE_TYPE_ARRAY;
        res.res.array.hArray = lvl.cu_array();
        let mut raw: cuda_bindings::CUsurfObject = 0;
        // SAFETY: `res` references the live mip level `CUarray` whose owning `MipmappedArray`
        // is retained inside the returned `SharedSurfaceObject`.
        unsafe { cuda_bindings::cuSurfObjectCreate(&mut raw, &res) }.result()?;
        Ok(SharedSurfaceObject {
            raw,
            ctx: Arc::clone(ctx),
            _backing: Arc::clone(&self.cuda_array),
        })
    }

    /// Returns the format mapping metadata for this texture.
    #[inline]
    pub fn format_mapping(&self) -> FormatMapping {
        self.mapping
    }

    /// Returns the CUDA array descriptor matching mip level 0.
    #[inline]
    pub fn array_desc(&self) -> &ArrayDesc {
        &self.array_desc
    }

    /// Returns the underlying `cuda_ext::ExternalMemory` handle.
    #[inline]
    pub fn external_memory(&self) -> &Arc<ExternalMemory> {
        &self.ext_mem
    }

    /// Submits a Vulkan layout + queue-family ownership release barrier from `wgpu` (`from_uses`)
    /// to `VK_QUEUE_FAMILY_EXTERNAL` + `VK_IMAGE_LAYOUT_GENERAL` for CUDA consumption.
    ///
    /// On DX12 this routes `wait` and `signal` through `queue.submit(empty)` so pending
    /// `wgpu` writes flush and `wait` executes before `signal`.
    ///
    /// # Vulkan Queue Synchronization
    ///
    /// Uses `nv-wgpu`'s internal submission mutex (`VK_QUEUE_SUBMIT_LOCK`), but callers
    /// must not concurrently call `wgpu::Queue::submit` or `SurfaceTexture::present` on the
    /// same `queue` from another thread while a Vulkan ownership barrier executes.
    pub fn release_to_cuda(
        &self,
        queue: &wgpu::Queue,
        from_uses: wgpu::TextureUses,
        wait: Option<(&InteropSemaphore, u64)>,
        signal: Option<(&InteropSemaphore, u64)>,
    ) -> Result<()> {
        self.submit_ownership_barrier(
            queue,
            false,
            from_uses,
            wgpu::TextureUses::STORAGE_READ_WRITE,
            wait,
            signal,
        )
    }

    /// Submits a Vulkan layout + queue-family ownership acquire barrier from
    /// `VK_QUEUE_FAMILY_EXTERNAL` + `VK_IMAGE_LAYOUT_GENERAL` into `wgpu`'s `to_uses`.
    ///
    /// # Vulkan Queue Synchronization
    ///
    /// See [`Self::release_to_cuda`] for external `wgpu::Queue` synchronization rules.
    pub fn acquire_from_cuda(
        &self,
        queue: &wgpu::Queue,
        to_uses: wgpu::TextureUses,
        wait: Option<(&InteropSemaphore, u64)>,
        signal: Option<(&InteropSemaphore, u64)>,
    ) -> Result<()> {
        self.submit_ownership_barrier(
            queue,
            true,
            wgpu::TextureUses::STORAGE_READ_WRITE,
            to_uses,
            wait,
            signal,
        )
    }

    fn submit_ownership_barrier(
        &self,
        queue: &wgpu::Queue,
        acquire_to_wgpu: bool,
        from_uses: wgpu::TextureUses,
        to_uses: wgpu::TextureUses,
        wait: Option<(&InteropSemaphore, u64)>,
        signal: Option<(&InteropSemaphore, u64)>,
    ) -> Result<()> {
        match &self.backing {
            TextureNativeBacking::Vulkan {
                vk,
                barrier_runner,
                mip_level_count,
                array_layers,
            } => {
                let wgpu_family = barrier_runner.queue_family();
                let (src_family, dst_family, src_access, dst_access) = if acquire_to_wgpu {
                    (
                        vk::QUEUE_FAMILY_EXTERNAL,
                        wgpu_family,
                        vk::AccessFlags::empty(),
                        vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE,
                    )
                } else {
                    (
                        wgpu_family,
                        vk::QUEUE_FAMILY_EXTERNAL,
                        vk::AccessFlags::MEMORY_WRITE,
                        vk::AccessFlags::empty(),
                    )
                };

                let subresource = vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(*mip_level_count)
                    .base_array_layer(0)
                    .layer_count(*array_layers);
                let barrier = vk::ImageMemoryBarrier::default()
                    .src_access_mask(src_access)
                    .dst_access_mask(dst_access)
                    .old_layout(vk_layout_for_texture_uses(from_uses))
                    .new_layout(vk_layout_for_texture_uses(to_uses))
                    .src_queue_family_index(src_family)
                    .dst_queue_family_index(dst_family)
                    .image(vk.image)
                    .subresource_range(subresource);

                let wait_pair = match wait {
                    Some((sem, val)) => Some((
                        sem.vulkan_handle().ok_or_else(|| Error::BackendMismatch {
                            expected: "Vulkan",
                            found: sem.wgpu_backend(),
                        })?,
                        val,
                    )),
                    None => None,
                };
                let signal_pair = match signal {
                    Some((sem, val)) => Some((
                        sem.vulkan_handle().ok_or_else(|| Error::BackendMismatch {
                            expected: "Vulkan",
                            found: sem.wgpu_backend(),
                        })?,
                        val,
                    )),
                    None => None,
                };

                crate::platform::with_vk_queue(queue, |raw_q| {
                    barrier_runner.submit_image_barrier(
                        raw_q,
                        vk::PipelineStageFlags::ALL_COMMANDS,
                        vk::PipelineStageFlags::ALL_COMMANDS,
                        &barrier,
                        wait_pair,
                        signal_pair,
                    )
                })
            }
            #[cfg(windows)]
            TextureNativeBacking::Dx12 { .. } => {
                crate::platform::flush_wgpu_wait_signal(queue, wait, signal)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texture_descriptor_validation_enforces_format_and_usage() {
        let valid = SharedTextureDescriptor::d2(
            128,
            64,
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            true,
        );
        assert_eq!(valid.initial_state, wgpu::TextureUses::STORAGE_READ_WRITE);
        let (map, array) = validate_texture_descriptor(&valid).unwrap();
        assert_eq!(map.bytes_per_pixel, 4);
        assert_eq!(array.width, 128);
        assert_eq!(array.height, 64);

        let storage_only = SharedTextureDescriptor::d2(
            32,
            32,
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureUsages::STORAGE_BINDING,
            false,
        );
        assert_eq!(
            storage_only.initial_state,
            wgpu::TextureUses::STORAGE_READ_WRITE
        );

        let uninit_plain = SharedTextureDescriptor::d2(
            32,
            32,
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureUsages::TEXTURE_BINDING,
            false,
        );
        assert_eq!(uninit_plain.initial_state, wgpu::TextureUses::UNINITIALIZED);

        let empty_usage = SharedTextureDescriptor::d2(
            16,
            16,
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureUsages::empty(),
            false,
        );
        assert!(validate_texture_descriptor(&empty_usage).is_err());

        let bad_format = SharedTextureDescriptor::d2(
            16,
            16,
            wgpu::TextureFormat::Depth32Float,
            wgpu::TextureUsages::TEXTURE_BINDING,
            false,
        );
        assert!(matches!(
            validate_texture_descriptor(&bad_format),
            Err(Error::Unsupported {
                what: "texture format",
                ..
            })
        ));
    }
}
