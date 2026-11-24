//! Zero-copy shared GPU textures (`wgpu::Texture` <-> `cuda_core_ext::MipmappedArray`).
//!
//! Why: Shares 2D/3D/layered/cubemap GPU images directly between `wgpu` rendering or
//! compute passes and CUDA arrays/surfaces/textures without copying pixels through RAM:
//! - On Vulkan: allocates an exportable dedicated `VkImage` + `VkDeviceMemory`, optionally
//!   transitions it from `UNDEFINED` to the layout matching `initial_state`, wraps it via
//!   `wgpu::hal::vulkan::Device::texture_from_raw(..., TextureMemory::External)` +
//!   `create_texture_from_hal`, and maps a `cuda_core_ext::MipmappedArray` over the imported memory.
//! - On DX12 (Windows): allocates a committed shared `ID3D12Resource` texture
//!   (`D3D12_HEAP_FLAG_SHARED` + `D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS`), wraps
//!   via `wgpu::hal::dx12::Device::texture_from_raw`, and imports via `D3d12Resource`.
//!
//! Drop order: `SharedTexture::drop` first waits for its last raw barrier submission, then CUDA
//! arrays release before `ExternalMemory`, before `wgpu::Texture`, before the underlying
//! `VkImage`/`VkDeviceMemory` or `ID3D12Resource`, before the `wgpu::Device` that keeps the
//! `VkDevice` alive. CUDA views cloned out of a `SharedTexture` (mipmapped array, texture and
//! surface objects, `ExternalMemory`) keep the CUDA import alive but not the native image:
//! drop them first, or finish their CUDA work before dropping the `SharedTexture`.
//!
//! Used by callers and GPU interop tests.

use std::sync::{Arc, Mutex, PoisonError};

use ash::vk;
use cuda_core::Context;
use cuda_core_ext::array::LevelRef;
use cuda_core_ext::{
    ArrayDesc, ExternalMemory, MipmappedArray, SurfaceObject, TextureDesc, TextureObject,
};
#[cfg(windows)]
use wgpu::hal::api::Dx12;
use wgpu::hal::api::Vulkan;
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D12;

use crate::device::{InteropBackend, verify_device_interop_support};
use crate::error::{Error, Result};
use crate::format::{FormatMapping, build_cuda_array_desc};
use crate::platform::{
    DedicatedResource, ExternalMemoryKind, alloc_bind_exportable_dedicated, export_vk_memory,
    map_hal_texture_uses, map_vk_image_usage, ownership_transfer, submit_vulkan_raw,
    vk_external_memory_handle_type, vk_layout_for_texture_uses, wait_for_submission,
};
use crate::sync::InteropSemaphore;

#[cfg(windows)]
use crate::platform::{create_shared_committed_resource, export_d3d12_shared_handle};

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
    /// [`SurfaceObject`] (`surf2Dwrite`/`surf2Dread`).
    pub cuda_surface: bool,
    /// Marks a 2D texture array with `6 * N` square faces as a CUDA/Vulkan cubemap.
    pub cubemap: bool,
    /// Initial `wgpu` layout/usage state reported to `create_texture_from_hal` (Vulkan).
    ///
    /// When set to anything other than [`wgpu::TextureUses::UNINITIALIZED`] on Vulkan,
    /// creation submits a one-time transition from `VK_IMAGE_LAYOUT_UNDEFINED` to the
    /// layout corresponding to `initial_state` so first-use barriers never discard prior
    /// CUDA writes. On DX12 the shared resource is created in `D3D12_RESOURCE_STATE_COMMON`,
    /// which `wgpu` tracks as `UNINITIALIZED`, so this field is not used there.
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
        let initial_state = if cuda_surface || usage.contains(wgpu::TextureUsages::STORAGE_BINDING)
        {
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
        return Err(Error::InvalidInput(
            "shared texture usage must be non-empty",
        ));
    }
    if desc.size.depth_or_array_layers > u32::from(u16::MAX)
        || desc.mip_level_count > u32::from(u16::MAX)
    {
        return Err(Error::InvalidInput(
            "texture layers and mip_level_count must fit in u16",
        ));
    }
    let (mapping, mut array_desc) = build_cuda_array_desc(
        desc.size,
        desc.dimension,
        desc.format,
        desc.mip_level_count,
        desc.cuda_surface,
        desc.cubemap,
    )?;
    // cuda.h (`cuExternalMemoryGetMappedMipmappedArray`): a mipmapped array bound as a colour
    // target in the graphics API must be mapped with `CUDA_ARRAY3D_COLOR_ATTACHMENT`.
    array_desc.color_attachment = desc.usage.contains(wgpu::TextureUsages::RENDER_ATTACHMENT);
    Ok((mapping, array_desc))
}

/// Rejects a format or usage that `wgpu` does not allow on this device, before any native object
/// exists: `create_texture_from_hal` reports missing format features only through the device's
/// error sink, after the image and the CUDA import were already made, and does not check usages.
///
/// With `TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES` enabled, `wgpu` takes allowed usages from the
/// adapter, which a `wgpu::Device` does not expose; then only the required features are checked.
fn check_format_support(
    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsages,
    features: wgpu::Features,
) -> Result<()> {
    let required = format.required_features();
    if !features.contains(required) {
        return Err(Error::Unsupported {
            what: "texture format device feature",
            required: "the wgpu::Features the format requires",
            found: format!("{format:?} requires {required:?}, device has {features:?}"),
        });
    }
    if !features.contains(wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES) {
        let allowed = format.guaranteed_format_features(features).allowed_usages;
        if !allowed.contains(usage) {
            return Err(Error::Unsupported {
                what: "texture usage for format",
                required: "usages within the format's guaranteed wgpu usages",
                found: format!("{format:?} does not allow {:?}", usage.difference(allowed)),
            });
        }
    }
    Ok(())
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
        /// `wgpu`'s queue family: the family side of ownership transfers.
        queue_family: u32,
        mip_level_count: u32,
        array_layers: u32,
    },
    #[cfg(windows)]
    Dx12 {
        _resource: Direct3D12::ID3D12Resource,
    },
}

/// Zero-copy GPU texture accessible simultaneously as [`wgpu::Texture`] and
/// [`cuda_core_ext::MipmappedArray`].
///
/// `Drop` first waits for the last raw barrier submitted for this texture (it references the
/// native image outside `wgpu`'s tracking), then fields drop in declaration order:
/// 1. `cuda_array` (`Arc<MipmappedArray>`)
/// 2. `ext_mem` (`Arc<ExternalMemory>`)
/// 3. `wgpu_tex` (`wgpu::Texture`)
/// 4. `backing` (`Arc<VkImageBacking>` / COM `ID3D12Resource`)
/// 5. `device` (`wgpu::Device`), which keeps the `VkDevice` alive for step 4
pub struct SharedTexture {
    cuda_array: Arc<MipmappedArray>,
    ext_mem: Arc<ExternalMemory>,
    wgpu_tex: wgpu::Texture,
    backing: TextureNativeBacking,
    mapping: FormatMapping,
    array_desc: ArrayDesc,
    last_barrier: Mutex<Option<wgpu::SubmissionIndex>>,
    device: wgpu::Device,
}

impl Drop for SharedTexture {
    fn drop(&mut self) {
        // Poisoning only means a panic elsewhere while storing an index; the value is intact.
        let last = self
            .last_barrier
            .get_mut()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        wait_for_submission(&self.device, last, "SharedTexture::drop (wait for last ownership barrier)");
    }
}

// SAFETY: `MipmappedArray`, `ExternalMemory`, `wgpu::Texture`, and Vulkan/DX12 handles are Send.
unsafe impl Send for SharedTexture {}
// SAFETY: Shared access only inspects handles or submits driver-synchronized commands.
unsafe impl Sync for SharedTexture {}

impl SharedTexture {
    /// Creates a new device-local shared texture visible to both `wgpu` and `cuda_ctx`.
    ///
    /// Fails with [`Error::Unsupported`] before allocating anything when the format needs a
    /// `wgpu::Features` the device lacks, or `desc.usage` exceeds the format's allowed usages.
    ///
    /// # Queue Synchronization
    ///
    /// When `desc.initial_state != wgpu::TextureUses::UNINITIALIZED` on Vulkan, creation
    /// submits the initial layout transition through `wgpu::Queue::submit` on `queue`, which is
    /// safe next to other threads' submits and presents.
    pub fn create(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        cuda_ctx: &Arc<Context>,
        desc: &SharedTextureDescriptor<'_>,
    ) -> Result<Self> {
        let (mapping, array_desc) = validate_texture_descriptor(desc)?;
        check_format_support(desc.format, desc.usage, device.features())?;
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
        let hal_dev =
            unsafe { device.as_hal::<Vulkan>() }.ok_or_else(|| Error::BackendMismatch {
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

        let (vk_mem, mem_reqs) = alloc_bind_exportable_dedicated(
            instance,
            raw_dev,
            phys,
            DedicatedResource::Image(vk_img),
        )?;

        let vk_backing = Arc::new(VkImageBacking {
            device: raw_dev.clone(),
            image: vk_img,
            memory: vk_mem,
        });

        let exported = export_vk_memory(instance, raw_dev, vk_mem)?;
        let ext_mem = exported.import_cuda_memory(
            cuda_ctx,
            ExternalMemoryKind::VulkanOpaque,
            mem_reqs.size,
            true,
        )?;
        let cuda_array =
            Arc::new(ext_mem.mapped_mipmapped_array(0, &array_desc, desc.mip_level_count)?);

        // Initial layout transition UNDEFINED -> layout of `initial_state` (for instance
        // `STORAGE_READ_WRITE` -> GENERAL before CUDA writes first). Submitted after every
        // fallible step, so no early return can destroy the image while it is in flight; from
        // here on `SharedTexture::drop` waits for it.
        let mut initial_barrier = None;
        if desc.initial_state != wgpu::TextureUses::UNINITIALIZED {
            let dst_layout = vk_layout_for_texture_uses(desc.initial_state, desc.format);
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
            let ((), index) = submit_vulkan_raw(device, queue, None, None, |cmd| {
                // SAFETY: `cmd` is recording; `barrier` names `vk_img`, which stays alive until
                // the returned `SharedTexture` has waited for this submission.
                unsafe {
                    raw_dev.cmd_pipeline_barrier(
                        cmd,
                        vk::PipelineStageFlags::TOP_OF_PIPE,
                        vk::PipelineStageFlags::ALL_COMMANDS,
                        vk::DependencyFlags::empty(),
                        &[],
                        &[],
                        core::slice::from_ref(&barrier),
                    );
                }
                Ok(())
            })?;
            initial_barrier = Some(index);
        }

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
        let wgpu_tex = unsafe {
            device.create_texture_from_hal::<Vulkan>(hal_tex, &wgpu_desc, desc.initial_state)
        };

        Ok(Self {
            cuda_array,
            ext_mem,
            wgpu_tex,
            backing: TextureNativeBacking::Vulkan {
                vk: vk_backing,
                queue_family,
                mip_level_count: desc.mip_level_count,
                array_layers,
            },
            mapping,
            array_desc,
            last_barrier: Mutex::new(initial_barrier),
            device: device.clone(),
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
        let alloc_info =
            unsafe { raw_device.GetResourceAllocationInfo(0, core::slice::from_ref(&res_desc)) };
        if alloc_info.SizeInBytes == u64::MAX || alloc_info.SizeInBytes == 0 {
            return Err(Error::InvalidInput(
                "D3D12 rejected shared texture descriptor allocation info",
            ));
        }

        let resource = create_shared_committed_resource(
            raw_device,
            &res_desc,
            "ID3D12Device::CreateCommittedResource(Texture)",
        )?;

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
        // SAFETY: `hal_tex` belongs to `device` and starts in `D3D12_RESOURCE_STATE_COMMON`, the
        // state wgpu-hal DX12 maps `UNINITIALIZED` to (`conv::map_texture_usage_to_state`); any
        // other tracked state would make wgpu's first barrier start from a state the resource
        // is not in.
        let wgpu_tex = unsafe {
            device.create_texture_from_hal::<Dx12>(
                hal_tex,
                &wgpu_desc,
                wgpu::TextureUses::UNINITIALIZED,
            )
        };

        Ok(Self {
            cuda_array,
            ext_mem,
            wgpu_tex,
            backing: TextureNativeBacking::Dx12 {
                _resource: resource,
            },
            mapping,
            array_desc,
            last_barrier: Mutex::new(None),
            device: device.clone(),
        })
    }

    /// Returns the `wgpu::Texture` handle sharing this image memory.
    #[inline]
    pub fn wgpu_texture(&self) -> &wgpu::Texture {
        &self.wgpu_tex
    }

    /// Returns the imported [`cuda_core_ext::MipmappedArray`] handle.
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

    /// Creates a [`SurfaceObject`] bound to mip level `level`. It keeps the mipmapped array
    /// alive, not the native image: see the module docs for the drop rule.
    ///
    /// Requires [`SharedTextureDescriptor::cuda_surface`] = `true` at creation time.
    pub fn cuda_surface_object(&self, level: u32) -> Result<SurfaceObject> {
        if !self.array_desc.surface {
            return Err(Error::InvalidInput(
                "cuda_surface_object requires cuda_surface = true in SharedTextureDescriptor",
            ));
        }
        Ok(SurfaceObject::from_level(&self.cuda_array, level)?)
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

    /// Returns the underlying `cuda_core_ext::ExternalMemory` handle.
    #[inline]
    pub fn external_memory(&self) -> &Arc<ExternalMemory> {
        &self.ext_mem
    }

    /// Returns the raw `VkImage` handle (`*mut c_void`) when backed by Vulkan,
    /// or `None` on DX12.
    #[inline]
    pub fn vk_image_raw(&self) -> Option<*mut core::ffi::c_void> {
        use ash::vk::Handle as _;
        match &self.backing {
            TextureNativeBacking::Vulkan { vk, .. } => {
                Some(vk.image.as_raw() as usize as *mut core::ffi::c_void)
            }
            #[cfg(windows)]
            TextureNativeBacking::Dx12 { .. } => None,
        }
    }

    /// Returns the raw `VkFormat` integer (`u32`) when backed by Vulkan,
    /// or `None` on DX12.
    #[inline]
    pub fn vk_format_raw(&self) -> Option<u32> {
        match &self.backing {
            TextureNativeBacking::Vulkan { .. } => {
                Some(self.format_mapping().vk_format.as_raw() as u32)
            }
            #[cfg(windows)]
            TextureNativeBacking::Dx12 { .. } => None,
        }
    }
}

/// Returns the raw `VkImageView` handle (`*mut c_void`) for a [`wgpu::TextureView`]
/// when backed by Vulkan, or `None` on other backends.
#[inline]
pub fn vk_image_view_raw(view: &wgpu::TextureView) -> Option<*mut std::ffi::c_void> {
    use ash::vk::Handle as _;
    // SAFETY: Read-only HAL inspection of the Vulkan image view handle; `view` outlives the call.
    unsafe {
        view.as_hal::<Vulkan>()
            .map(|v| v.raw_handle().as_raw() as usize as *mut std::ffi::c_void)
    }
}

impl SharedTexture {
    /// Submits a Vulkan layout + queue-family ownership release barrier from `wgpu` (`from_uses`)
    /// to `VK_QUEUE_FAMILY_EXTERNAL` + `VK_IMAGE_LAYOUT_GENERAL` for CUDA consumption.
    ///
    /// On DX12 this routes `wait` and `signal` through `queue.submit(empty)` so pending
    /// `wgpu` writes flush and `wait` executes before `signal`.
    ///
    /// # Layout contract (Vulkan)
    ///
    /// The barrier changes the real `VkImage` layout, but `wgpu`'s resource tracker is not told.
    /// `from_uses` must be the state `wgpu` last tracked for this texture (the barrier's old
    /// layout), and after a later [`Self::acquire_from_cuda`] the texture must be in the state
    /// `wgpu` will assume next (its `to_uses`). A mismatch makes `wgpu`'s next barrier start from
    /// a wrong layout.
    ///
    /// # Queue Synchronization
    ///
    /// On both backends the work goes through `wgpu::Queue::submit` (Vulkan: the barrier is
    /// recorded into a `wgpu` command encoder), so it is safe next to other threads' submits and
    /// presents, and `wgpu`'s pending `write_texture` uploads are submitted before the barrier.
    /// `wait` / `signal` are staged on `queue` just before that submit; a submit from another
    /// thread in between would consume them, so serialize submits that use semaphores.
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
    /// `to_uses` must be the state `wgpu`'s tracker holds for this texture (see the layout
    /// contract and queue synchronization notes on [`Self::release_to_cuda`]).
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
                queue_family,
                mip_level_count,
                array_layers,
            } => {
                let xfer = ownership_transfer(acquire_to_wgpu, *queue_family);

                let subresource = vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(*mip_level_count)
                    .base_array_layer(0)
                    .layer_count(*array_layers);
                let barrier = vk::ImageMemoryBarrier::default()
                    .src_access_mask(xfer.src_access)
                    .dst_access_mask(xfer.dst_access)
                    .old_layout(vk_layout_for_texture_uses(from_uses, self.mapping.wgpu))
                    .new_layout(vk_layout_for_texture_uses(to_uses, self.mapping.wgpu))
                    .src_queue_family_index(xfer.src_family)
                    .dst_queue_family_index(xfer.dst_family)
                    .image(vk.image)
                    .subresource_range(subresource);

                let ((), index) = submit_vulkan_raw(&self.device, queue, wait, signal, |cmd| {
                    // SAFETY: `cmd` is recording, and `barrier` names this backing's live image,
                    // which `Drop` keeps alive until this submission completes.
                    unsafe {
                        vk.device.cmd_pipeline_barrier(
                            cmd,
                            vk::PipelineStageFlags::ALL_COMMANDS,
                            vk::PipelineStageFlags::ALL_COMMANDS,
                            vk::DependencyFlags::empty(),
                            &[],
                            &[],
                            core::slice::from_ref(&barrier),
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

        let render_target = SharedTextureDescriptor::d2(
            16,
            16,
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            false,
        );
        let (_, rt) = validate_texture_descriptor(&render_target).unwrap();
        assert!(
            rt.color_attachment,
            "colour targets map with COLOR_ATTACHMENT"
        );
        assert!(!array.color_attachment);
    }

    #[test]
    fn format_support_follows_wgpu_feature_and_usage_rules() {
        let none = wgpu::Features::empty();
        assert!(
            check_format_support(
                wgpu::TextureFormat::R16Unorm,
                wgpu::TextureUsages::TEXTURE_BINDING,
                none
            )
            .is_err(),
            "R16Unorm requires TEXTURE_FORMAT_16BIT_NORM"
        );
        check_format_support(
            wgpu::TextureFormat::R16Unorm,
            wgpu::TextureUsages::TEXTURE_BINDING,
            wgpu::Features::TEXTURE_FORMAT_16BIT_NORM,
        )
        .unwrap();
        assert!(
            check_format_support(
                wgpu::TextureFormat::Rgba8UnormSrgb,
                wgpu::TextureUsages::STORAGE_BINDING,
                none
            )
            .is_err(),
            "sRGB formats are never storage-capable"
        );
        check_format_support(
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            none,
        )
        .unwrap();
    }
}
