//! Safe Vulkan RAII wrappers for NGX (`NgxVulkan`, `FeatureHandle`).
//!
//! **What.** Manages the NGX Vulkan session lifecycle (`NVSDK_NGX_VULKAN_Init` /
//! `NVSDK_NGX_VULKAN_Init_with_ProjectID` / `NVSDK_NGX_VULKAN_Shutdown1`), parameter bag allocation,
//! scratch-buffer sizing, and feature creation/evaluation/release (`CreateFeature` /
//! `EvaluateFeature_C` / `ReleaseFeature`).
//! **Why.** Guarantees deterministic teardown order via `Arc<VulkanSession>`: every
//! [`Parameters`] and [`FeatureHandle`] holds a reference to the session so `Shutdown1` never
//! runs while features or parameter bags are still alive.
//! **How.** NGX FFI calls live behind `#[cfg(has_ngx_sdk)]` (Windows and Linux x86_64 when the
//! NGX SDK is present) and run under the process-wide NGX lock ([`crate::ngx_call`]); targets
//! without the SDK compile clean stubs returning [`Error::Unsupported`].
//! **Used by** `ngx` consumers (renderers / upscalers) and T2 GPU smoke tests.

use crate::error::{Error, Result, ResultCode};
use crate::params::{Parameters, keys};
use crate::types::{
    DlssDepthType, DlssFeatureFlags, DlssPerfQuality, DlssRoughnessMode, DlssRrPreset, Feature,
    ProjectId, RequiredExtensions,
};
use std::ffi::c_void;
use std::path::Path;

#[cfg(has_ngx_sdk)]
use crate::check_sys;
#[cfg(has_ngx_sdk)]
use crate::types::DlssDenoiseMode;
#[cfg(has_ngx_sdk)]
use std::ffi::CStr;
#[cfg(all(has_ngx_sdk, windows))]
use std::os::windows::ffi::OsStrExt;
#[cfg(has_ngx_sdk)]
use std::sync::Arc;

/// Raw Vulkan device handles and loader entry points required to initialize NGX on Vulkan.
#[derive(Clone, Copy, Debug)]
pub struct VulkanDeviceHandles {
    /// `VkInstance` handle.
    pub instance: *mut c_void,
    /// `VkPhysicalDevice` handle.
    pub physical_device: *mut c_void,
    /// `VkDevice` handle.
    pub device: *mut c_void,
    /// `PFN_vkGetInstanceProcAddr` function pointer.
    pub get_instance_proc_addr: *const c_void,
    /// `PFN_vkGetDeviceProcAddr` function pointer.
    pub get_device_proc_addr: *const c_void,
}

/// Vulkan image view resource descriptor passed to NGX feature evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VkImageResourceInfo {
    /// `VkImageView` handle.
    pub image_view: *mut c_void,
    /// Backing `VkImage` handle.
    pub image: *mut c_void,
    /// Vulkan `VkFormat` raw value.
    pub format: u32,
    /// Image width in pixels.
    pub width: u32,
    /// Image height in pixels.
    pub height: u32,
    /// Whether the image has storage (`VK_IMAGE_USAGE_STORAGE_BIT`) read/write access.
    pub read_write: bool,
}

#[cfg(has_ngx_sdk)]
impl VkImageResourceInfo {
    fn validate(&self, role: &'static str) -> Result<()> {
        if self.image_view.is_null() || self.image.is_null() {
            return Err(Error::InvalidArgument(match role {
                "color" => "color VkImageResourceInfo must not have null image_view or image",
                "depth" => "depth VkImageResourceInfo must not have null image_view or image",
                "motion" => "motion VkImageResourceInfo must not have null image_view or image",
                "diffuse_albedo" => {
                    "diffuse_albedo VkImageResourceInfo must not have null image_view or image"
                }
                "specular_albedo" => {
                    "specular_albedo VkImageResourceInfo must not have null image_view or image"
                }
                "normals" => "normals VkImageResourceInfo must not have null image_view or image",
                "roughness" => {
                    "roughness VkImageResourceInfo must not have null image_view or image"
                }
                _ => "output VkImageResourceInfo must not have null image_view or image",
            }));
        }
        Ok(())
    }

    /// Validates the `Output` resource: non-null handles and `read_write = true`, without which
    /// NGX fails the evaluation with `NVSDK_NGX_Result_FAIL_RWFlagMissing` (`nvsdk_ngx_defs.h:140-143`).
    fn validate_output(&self) -> Result<()> {
        self.validate("output")?;
        if !self.read_write {
            return Err(Error::InvalidArgument(
                "output VkImageResourceInfo must have read_write = true",
            ));
        }
        Ok(())
    }

    fn into_ngx_resource(self, aspect_mask: u32) -> ngx_sys::NVSDK_NGX_Resource_VK {
        ngx_sys::NVSDK_NGX_Resource_VK {
            Resource: ngx_sys::NVSDK_NGX_Resource_VK__bindgen_ty_1 {
                ImageViewInfo: ngx_sys::NVSDK_NGX_ImageViewInfo_VK {
                    ImageView: self.image_view.cast::<ngx_sys::VkImageView_T>(),
                    Image: self.image.cast::<ngx_sys::VkImage_T>(),
                    SubresourceRange: ngx_sys::VkImageSubresourceRange {
                        aspectMask: aspect_mask,
                        baseMipLevel: 0,
                        levelCount: 1,
                        baseArrayLayer: 0,
                        layerCount: 1,
                    },
                    Format: ngx_sys::VkFormat::from_ne_bytes(self.format.to_ne_bytes()),
                    Width: self.width,
                    Height: self.height,
                },
            },
            Type: ngx_sys::NVSDK_NGX_Resource_VK_Type_NVSDK_NGX_RESOURCE_VK_TYPE_VK_IMAGEVIEW,
            ReadWrite: self.read_write,
        }
    }
}

/// Rejects a render subrect (`DLSS.Render.Subrect.Dimensions`) larger than the `Color` input it
/// is read from.
#[cfg(has_ngx_sdk)]
fn validate_render_extent(
    color: &VkImageResourceInfo,
    render_width: u32,
    render_height: u32,
) -> Result<()> {
    if render_width > color.width || render_height > color.height {
        return Err(Error::InvalidArgument(
            "render subrect must fit inside the color input extent",
        ));
    }
    Ok(())
}

/// Overwrites resource pointer keys with null after an evaluation.
///
/// The `*_with_params` evaluators point these keys at stack locals that die when they return;
/// clearing them keeps a reused bag from carrying dangling pointers into a later raw
/// [`FeatureHandle::evaluate`].
#[cfg(has_ngx_sdk)]
fn clear_pointer_keys(params: &mut Parameters, names: &[&str]) -> Result<()> {
    for name in names {
        // SAFETY: null is the value the evaluators already pass for absent optional inputs
        // (`GBuffer.Roughness`, matrices); nothing dereferences it.
        unsafe { params.set_void_ptr(name, std::ptr::null_mut())? };
    }
    Ok(())
}

/// Maps a capability `Get` that failed with an NGX status (key not published by this runtime) to
/// `None`; other errors propagate.
#[cfg(has_ngx_sdk)]
fn published<T>(res: Result<T>) -> Result<Option<T>> {
    match res {
        Ok(v) => Ok(Some(v)),
        Err(Error::Ngx { .. }) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Allocates a parameter bag owned by `session` (`NVSDK_NGX_VULKAN_AllocateParameters`).
#[cfg(has_ngx_sdk)]
fn allocate_session_parameters(session: &Arc<VulkanSession>) -> Result<Parameters> {
    let mut ptr: *mut ngx_sys::NVSDK_NGX_Parameter = std::ptr::null_mut();
    let rc = crate::ngx_call(|| {
        // SAFETY: `session` keeps the NGX Vulkan session active and `ptr` is writable.
        unsafe { ngx_sys::NVSDK_NGX_VULKAN_AllocateParameters(&mut ptr) }
    });
    check_sys("NVSDK_NGX_VULKAN_AllocateParameters", rc)?;
    Parameters::from_raw(ptr, Arc::clone(session))
}

/// Returns `VK_IMAGE_ASPECT_DEPTH_BIT` (`2`) for Vulkan depth/stencil formats (`124..=130`) and
/// `VK_IMAGE_ASPECT_COLOR_BIT` (`1`) for linear color formats such as `R32_SFLOAT` (`100`).
#[cfg(has_ngx_sdk)]
fn depth_aspect_mask(format: u32) -> u32 {
    const VK_FORMAT_D16_UNORM: u32 = 124;
    const VK_FORMAT_D32_SFLOAT_S8_UINT: u32 = 130;
    if (VK_FORMAT_D16_UNORM..=VK_FORMAT_D32_SFLOAT_S8_UINT).contains(&format) {
        2
    } else {
        1
    }
}

/// Optimal render dimensions and sharpness returned by `NGX_DLSS_GET_OPTIMAL_SETTINGS` and
/// `NGX_DLSSD_GET_OPTIMAL_SETTINGS`.
///
/// The dynamic min/max fields follow the SDK helper (`nvsdk_ngx_helpers.h:103-111`): runtimes
/// that do not publish the `DLSS.Get.Dynamic.*` keys report the optimal size, so
/// `min == max == optimal` means "no dynamic range reported", not necessarily "unsupported".
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DlssOptimalSettings {
    /// Recommended render width in pixels.
    pub render_width: u32,
    /// Recommended render height in pixels.
    pub render_height: u32,
    /// Minimum dynamic render width in pixels (`render_width` when the runtime reports none).
    pub min_width: u32,
    /// Minimum dynamic render height in pixels (`render_height` when the runtime reports none).
    pub min_height: u32,
    /// Maximum dynamic render width in pixels (`render_width` when the runtime reports none).
    pub max_width: u32,
    /// Maximum dynamic render height in pixels (`render_height` when the runtime reports none).
    pub max_height: u32,
    /// Sharpening factor reported by the runtime, `0.0` when it reports none. DLSS no longer
    /// supports sharpening (`nvsdk_ngx_defs.h:296`), so this is informational only.
    pub sharpness: f32,
}

/// DLSS Ray Reconstruction availability read from the capability parameters
/// (`SuperSamplingDenoising.*`, `nvsdk_ngx_defs_dlssd.h:151-155`).
///
/// A field is `None` when the runtime does not publish that key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DlssRrAvailability {
    /// `SuperSamplingDenoising.Available` is non-zero (`false` when the key is not published).
    pub available: bool,
    /// `SuperSamplingDenoising.NeedsUpdatedDriver`: the installed driver is too old for RR.
    pub needs_updated_driver: Option<bool>,
    /// Minimum driver version `(major, minor)` RR requires
    /// (`SuperSamplingDenoising.MinDriverVersionMajor` / `.MinDriverVersionMinor`).
    pub min_driver_version: Option<(u32, u32)>,
    /// `SuperSamplingDenoising.FeatureInitResult`: the runtime's feature initialization status.
    pub feature_init_result: Option<ResultCode>,
}

/// Creation configuration for [`NgxVulkan::create_dlss_rr`] (`NVSDK_NGX_DLSSD_Create_Params`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DlssRrCreateParams {
    /// Input render width in pixels.
    pub render_width: u32,
    /// Input render height in pixels.
    pub render_height: u32,
    /// Upscaled output width in pixels.
    pub out_width: u32,
    /// Upscaled output height in pixels.
    pub out_height: u32,
    /// Performance/quality preset.
    pub quality: DlssPerfQuality,
    /// Feature creation flags (typically `IS_HDR | MV_LOW_RES`).
    pub flags: DlssFeatureFlags,
    /// Roughness input packing (`Unpacked` vs `Packed` in `normals.w`).
    pub roughness_mode: DlssRoughnessMode,
    /// Depth buffer encoding (`Linear` vs `Hw`).
    pub depth_type: DlssDepthType,
    /// Optional Ray Reconstruction render preset hint.
    pub preset: DlssRrPreset,
}

/// Per-frame evaluation inputs for [`FeatureHandle::evaluate_dlss_rr_with_params`]
/// (`NVSDK_NGX_VK_DLSSD_Eval_Params`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DlssRrEvalParams {
    /// Noisy linear HDR input color resource (`Color`).
    pub color: VkImageResourceInfo,
    /// Depth resource (`Depth`, matching creation [`DlssDepthType`]).
    pub depth: VkImageResourceInfo,
    /// 2D motion vectors (`MotionVectors`).
    pub motion: VkImageResourceInfo,
    /// Diffuse albedo guide (`DLSS.Input.DiffuseAlbedo`).
    pub diffuse_albedo: VkImageResourceInfo,
    /// Specular albedo guide (`DLSS.Input.SpecularAlbedo`).
    pub specular_albedo: VkImageResourceInfo,
    /// Surface normals guide (`GBuffer.Normals`; `.w` holds roughness if [`DlssRoughnessMode::Packed`]).
    pub normals: VkImageResourceInfo,
    /// Optional separate roughness guide (`GBuffer.Roughness`; required for [`DlssRoughnessMode::Unpacked`]).
    pub roughness: Option<VkImageResourceInfo>,
    /// Output denoised and upscaled color resource (`Output`, requires `read_write = true`).
    pub output: VkImageResourceInfo,
    /// Active render subrect width in pixels.
    pub render_width: u32,
    /// Active render subrect height in pixels.
    pub render_height: u32,
    /// Camera jitter offset X in render pixel space.
    pub jitter_x: f32,
    /// Camera jitter offset Y in render pixel space.
    pub jitter_y: f32,
    /// Motion vector X scale (`0.0` defaults to `1.0`).
    pub mv_scale_x: f32,
    /// Motion vector Y scale (`0.0` defaults to `1.0`).
    pub mv_scale_y: f32,
    /// Pre-exposure scalar (`0.0` defaults to `1.0`).
    pub pre_exposure: f32,
    /// Exposure scale (`0.0` defaults to `1.0`).
    pub exposure_scale: f32,
    /// Reset temporal history (`Reset`).
    pub reset: bool,
    /// Optional 4x4 world-to-view matrix (`WorldToViewMatrix`).
    pub world_to_view: Option<[f32; 16]>,
    /// Optional 4x4 view-to-clip matrix (`ViewToClipMatrix`).
    pub view_to_clip: Option<[f32; 16]>,
}

#[cfg(has_ngx_sdk)]
fn with_feature_common_info<R>(
    wide_path: *const ngx_sys::wchar_t,
    f: impl FnOnce(*const ngx_sys::NVSDK_NGX_FeatureCommonInfo) -> R,
) -> R {
    let mut extra_wides: Vec<Vec<NgxWchar>> = Vec::with_capacity(4);
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        extra_wides.push(to_wchar_nul(dir));
    }
    for var in ["NGX_SDK_ROOT", "DLSS_ROOT", "NVSDK_NGX_ROOT"] {
        if let Some(root) = std::env::var_os(var) {
            let base = Path::new(&root);
            #[cfg(windows)]
            let rel = base.join("lib").join("Windows_x86_64").join("rel");
            #[cfg(not(windows))]
            let rel = base.join("lib").join("Linux_x86_64");
            if rel.is_dir() {
                extra_wides.push(to_wchar_nul(&rel));
            }
            if base.is_dir() {
                extra_wides.push(to_wchar_nul(base));
            }
        }
    }
    if let Some(sdk_bin) = ngx_sys::SDK_BIN {
        let p = Path::new(sdk_bin);
        if p.is_dir() {
            extra_wides.push(to_wchar_nul(p));
        }
    }
    let mut path_ptrs: Vec<*const ngx_sys::wchar_t> = Vec::with_capacity(1 + extra_wides.len());
    path_ptrs.push(wide_path);
    for w in &extra_wides {
        path_ptrs.push(w.as_ptr().cast::<ngx_sys::wchar_t>());
    }
    let feature_info = ngx_sys::NVSDK_NGX_FeatureCommonInfo {
        PathListInfo: ngx_sys::NVSDK_NGX_PathListInfo {
            Path: path_ptrs.as_ptr(),
            Length: u32::try_from(path_ptrs.len()).unwrap_or(1),
        },
        InternalData: std::ptr::null_mut(),
        LoggingInfo: ngx_sys::NVSDK_NGX_LoggingInfo {
            LoggingCallback: None,
            MinimumLoggingLevel: ngx_sys::NVSDK_NGX_Logging_Level_NVSDK_NGX_LOGGING_LEVEL_OFF,
            DisableOtherLoggingSinks: false,
        },
    };
    f(&feature_info)
}

#[cfg(has_ngx_sdk)]
pub(crate) struct VulkanSession {
    device: ngx_sys::VkDevice,
}

// SAFETY: NGX is not thread-safe (`nvsdk_ngx_vk.h:16-17`: "It is up to the client to ensure that
// thread safety is enforced as needed"). `VulkanSession` only stores the opaque `VkDevice` handle,
// and every NGX call made through any session, parameter bag or feature, including the final
// `Shutdown1` in `Drop`, runs under the process-wide `crate::ngx_call` / `crate::ngx_locked`
// lock, so moving the session to another thread cannot produce concurrent NGX calls.
#[cfg(has_ngx_sdk)]
unsafe impl Send for VulkanSession {}
// SAFETY: `VulkanSession` has no interior mutability, and every NGX call reachable through a
// shared reference is serialised by the process-wide NGX lock (see the `Send` impl above).
#[cfg(has_ngx_sdk)]
unsafe impl Sync for VulkanSession {}

#[cfg(has_ngx_sdk)]
impl Drop for VulkanSession {
    fn drop(&mut self) {
        if !self.device.is_null() {
            let device = self.device;
            crate::ngx_locked(|g| {
                // SAFETY: `device` was initialized by `NVSDK_NGX_VULKAN_Init*` and all
                // `FeatureHandle` / `Parameters` children hold `Arc<VulkanSession>`, so this runs
                // only after every child has been dropped.
                let rc = unsafe { ngx_sys::NVSDK_NGX_VULKAN_Shutdown1(device) };
                g.record_teardown("NVSDK_NGX_VULKAN_Shutdown1", rc);
                g.release_device(device.addr());
            });
            self.device = std::ptr::null_mut();
        }
    }
}

/// RAII handle to an initialized NGX Vulkan session.
///
/// Cloning [`NgxVulkan`] shares the underlying `Arc` session; `NVSDK_NGX_VULKAN_Shutdown1` runs
/// once when the last [`NgxVulkan`], [`Parameters`], and [`FeatureHandle`] are dropped. Only one
/// session per `VkDevice` may be live at a time; share it by cloning.
///
/// The session may be shared across threads: every NGX call is serialised by a process-wide lock
/// because NGX itself is not thread-safe.
///
/// Drop the last handle only after the GPU has finished every command buffer recorded with this
/// session (for example after `vkDeviceWaitIdle` or a fence wait): `Shutdown1` and
/// `ReleaseFeature` free NGX resources immediately (`nvsdk_ngx_vk.h:548-551`).
#[derive(Clone)]
pub struct NgxVulkan {
    #[cfg(has_ngx_sdk)]
    session: Arc<VulkanSession>,
}

impl NgxVulkan {
    /// Queries the Vulkan instance and device extensions required by NGX
    /// (`NVSDK_NGX_VULKAN_RequiredExtensions`).
    ///
    /// The SDK marks this query deprecated in favour of the per-feature
    /// `NVSDK_NGX_VULKAN_GetFeatureInstanceExtensionRequirements` /
    /// `NVSDK_NGX_VULKAN_GetFeatureDeviceExtensionRequirements` (`nvsdk_ngx_vk.h:87-96`), which
    /// are not wrapped yet.
    pub fn required_extensions() -> Result<RequiredExtensions> {
        #[cfg(has_ngx_sdk)]
        {
            let mut inst_count: u32 = 0;
            let mut inst_exts: *mut *const std::os::raw::c_char = std::ptr::null_mut();
            let mut dev_count: u32 = 0;
            let mut dev_exts: *mut *const std::os::raw::c_char = std::ptr::null_mut();
            let rc = crate::ngx_call(|| {
                // SAFETY: Output pointers are valid stack locals.
                unsafe {
                    ngx_sys::NVSDK_NGX_VULKAN_RequiredExtensions(
                        &mut inst_count,
                        &mut inst_exts,
                        &mut dev_count,
                        &mut dev_exts,
                    )
                }
            });
            check_sys("NVSDK_NGX_VULKAN_RequiredExtensions", rc)?;
            Ok(RequiredExtensions {
                // SAFETY: NGX populates `inst_exts` with `inst_count` valid C strings.
                instance_extensions: unsafe { read_cstr_array(inst_exts, inst_count) },
                // SAFETY: NGX populates `dev_exts` with `dev_count` valid C strings.
                device_extensions: unsafe { read_cstr_array(dev_exts, dev_count) },
            })
        }
        #[cfg(not(has_ngx_sdk))]
        {
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Initializes NGX on a Vulkan device using `NVSDK_NGX_VULKAN_Init_with_ProjectID` (PLAN D13).
    ///
    /// Returns [`Error::InvalidArgument`] while another [`NgxVulkan`] session for the same
    /// `VkDevice` is alive; clone that session instead.
    ///
    /// # Safety
    ///
    /// `handles` must contain live, valid Vulkan handles (`VkInstance`, `VkPhysicalDevice`,
    /// `VkDevice`) and valid `vkGetInstanceProcAddr` / `vkGetDeviceProcAddr` function pointers
    /// that outlive the returned [`NgxVulkan`] session and all derived handles.
    pub unsafe fn init(
        project_id: &ProjectId,
        app_data_path: &Path,
        handles: VulkanDeviceHandles,
    ) -> Result<Self> {
        #[cfg(has_ngx_sdk)]
        {
            let engine_version_ptr = project_id
                .engine_version_cstr()
                .map_or(c"".as_ptr(), |c| c.as_ptr());
            // SAFETY: Caller guarantees `handles` contains live Vulkan handles and valid loader
            // function pointers; `project_id` C strings and `wide_path` outlive the FFI call.
            // `engineType` is `NVSDK_NGX_ENGINE_TYPE_CUSTOM` per PLAN D13.
            unsafe {
                init_vulkan_session(
                    app_data_path,
                    handles,
                    "NVSDK_NGX_VULKAN_Init_with_ProjectID",
                    |wide_path, h| {
                        with_feature_common_info(wide_path, |feature_info| {
                            ngx_sys::NVSDK_NGX_VULKAN_Init_with_ProjectID(
                                project_id.guid_cstr().as_ptr(),
                                ngx_sys::NVSDK_NGX_EngineType_NVSDK_NGX_ENGINE_TYPE_CUSTOM,
                                engine_version_ptr,
                                wide_path,
                                h.instance,
                                h.physical_device,
                                h.device,
                                h.gipa,
                                h.gdpa,
                                feature_info,
                                ngx_sys::NVSDK_NGX_Version_NVSDK_NGX_Version_API,
                            )
                        })
                    },
                )
            }
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (project_id, app_data_path, handles);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Initializes NGX on a Vulkan device using an NVIDIA-issued `ApplicationId`
    /// (`NVSDK_NGX_VULKAN_Init`).
    ///
    /// Prefer [`NgxVulkan::init`] with a [`ProjectId`] when no NVIDIA-issued application ID is
    /// required. Like [`NgxVulkan::init`], rejects a second live session on the same `VkDevice`.
    ///
    /// # Safety
    ///
    /// `handles` must contain live, valid Vulkan handles (`VkInstance`, `VkPhysicalDevice`,
    /// `VkDevice`) and valid `vkGetInstanceProcAddr` / `vkGetDeviceProcAddr` function pointers
    /// that outlive the returned [`NgxVulkan`] session and all derived handles.
    pub unsafe fn init_with_app_id(
        app_id: u64,
        app_data_path: &Path,
        handles: VulkanDeviceHandles,
    ) -> Result<Self> {
        #[cfg(has_ngx_sdk)]
        {
            // SAFETY: Caller guarantees `handles` contains live Vulkan handles and valid loader
            // function pointers; `wide_path` outlives the FFI call.
            unsafe {
                init_vulkan_session(
                    app_data_path,
                    handles,
                    "NVSDK_NGX_VULKAN_Init",
                    |wide_path, h| {
                        with_feature_common_info(wide_path, |feature_info| {
                            ngx_sys::NVSDK_NGX_VULKAN_Init(
                                app_id as std::os::raw::c_ulonglong,
                                wide_path,
                                h.instance,
                                h.physical_device,
                                h.device,
                                h.gipa,
                                h.gdpa,
                                feature_info,
                                ngx_sys::NVSDK_NGX_Version_NVSDK_NGX_Version_API,
                            )
                        })
                    },
                )
            }
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (app_id, app_data_path, handles);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Allocates a mutable NGX parameter bag (`NVSDK_NGX_VULKAN_AllocateParameters`).
    pub fn allocate_parameters(&self) -> Result<Parameters> {
        #[cfg(has_ngx_sdk)]
        {
            allocate_session_parameters(&self.session)
        }
        #[cfg(not(has_ngx_sdk))]
        {
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Retrieves NGX capability parameters (`NVSDK_NGX_VULKAN_GetCapabilityParameters`).
    pub fn capability_parameters(&self) -> Result<Parameters> {
        #[cfg(has_ngx_sdk)]
        {
            let mut ptr: *mut ngx_sys::NVSDK_NGX_Parameter = std::ptr::null_mut();
            let rc = crate::ngx_call(|| {
                // SAFETY: NGX Vulkan session is active (`self.session`) and `ptr` is writable.
                unsafe { ngx_sys::NVSDK_NGX_VULKAN_GetCapabilityParameters(&mut ptr) }
            });
            check_sys("NVSDK_NGX_VULKAN_GetCapabilityParameters", rc)?;
            Parameters::from_raw(ptr, Arc::clone(&self.session))
        }
        #[cfg(not(has_ngx_sdk))]
        {
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Queries the scratch buffer size in bytes for `feature` (`NVSDK_NGX_VULKAN_GetScratchBufferSize`).
    pub fn scratch_buffer_size(&self, feature: Feature, params: &Parameters) -> Result<usize> {
        #[cfg(has_ngx_sdk)]
        {
            let mut size = 0usize;
            let rc = crate::ngx_call(|| {
                // SAFETY: `params.as_ptr()` is a valid `NVSDK_NGX_Parameter` and `size` is writable.
                unsafe {
                    ngx_sys::NVSDK_NGX_VULKAN_GetScratchBufferSize(
                        ngx_sys::NVSDK_NGX_Feature::from_ne_bytes(feature.to_raw().to_ne_bytes()),
                        params.as_ptr(),
                        &mut size,
                    )
                }
            });
            check_sys("NVSDK_NGX_VULKAN_GetScratchBufferSize", rc)?;
            Ok(size)
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (feature, params);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Creates an NGX feature instance on `cmd_buffer` (`NVSDK_NGX_VULKAN_CreateFeature`).
    ///
    /// # Safety
    ///
    /// `cmd_buffer` must be a valid, recording `VkCommandBuffer` belonging to the `VkDevice` used
    /// to initialize this [`NgxVulkan`] session.
    pub unsafe fn create_feature(
        &self,
        cmd_buffer: *mut c_void,
        feature: Feature,
        params: &mut Parameters,
    ) -> Result<FeatureHandle> {
        #[cfg(has_ngx_sdk)]
        {
            if cmd_buffer.is_null() {
                return Err(Error::InvalidArgument("cmd_buffer must not be null"));
            }
            let mut handle: *mut ngx_sys::NVSDK_NGX_Handle = std::ptr::null_mut();
            let vk_cmd = cmd_buffer.cast::<ngx_sys::VkCommandBuffer_T>();
            let params_ptr = params.as_mut_ptr();
            let rc = crate::ngx_call(|| {
                // SAFETY: Caller guarantees `vk_cmd` is a valid recording `VkCommandBuffer`;
                // `params_ptr` and `handle` are valid.
                unsafe {
                    ngx_sys::NVSDK_NGX_VULKAN_CreateFeature(
                        vk_cmd,
                        ngx_sys::NVSDK_NGX_Feature::from_ne_bytes(feature.to_raw().to_ne_bytes()),
                        params_ptr,
                        &mut handle,
                    )
                }
            });
            check_sys("NVSDK_NGX_VULKAN_CreateFeature", rc)?;
            if handle.is_null() {
                return Err(Error::InvalidArgument("NGX returned null NVSDK_NGX_Handle"));
            }
            Ok(FeatureHandle {
                handle,
                _session: Arc::clone(&self.session),
            })
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (cmd_buffer, feature, params);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Queries whether DLSS Ray Reconstruction (`SuperSamplingDenoising.Available`) is available
    /// on this device and runtime. See [`NgxVulkan::dlss_rr_availability`] for the reason when it
    /// is not.
    pub fn is_dlss_rr_available(&self) -> Result<bool> {
        Ok(self.dlss_rr_availability()?.available)
    }

    /// Queries DLSS Ray Reconstruction availability together with the reasons the runtime
    /// publishes when it is unavailable (driver too old, minimum driver version, feature
    /// initialization result).
    pub fn dlss_rr_availability(&self) -> Result<DlssRrAvailability> {
        #[cfg(has_ngx_sdk)]
        {
            let params = self.capability_parameters()?;
            let available = published(params.get_i32(keys::RAY_RECONSTRUCTION_AVAILABLE))?
                .is_some_and(|v| v != 0);
            let needs_updated_driver =
                published(params.get_i32(keys::RAY_RECONSTRUCTION_NEEDS_UPDATED_DRIVER))?
                    .map(|v| v != 0);
            let min_major =
                published(params.get_u32(keys::RAY_RECONSTRUCTION_MIN_DRIVER_VERSION_MAJOR))?;
            let min_minor =
                published(params.get_u32(keys::RAY_RECONSTRUCTION_MIN_DRIVER_VERSION_MINOR))?;
            let feature_init_result =
                published(params.get_i32(keys::RAY_RECONSTRUCTION_FEATURE_INIT_RESULT))?
                    .map(ResultCode);
            Ok(DlssRrAvailability {
                available,
                needs_updated_driver,
                min_driver_version: min_major.zip(min_minor),
                feature_init_result,
            })
        }
        #[cfg(not(has_ngx_sdk))]
        {
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    fn query_optimal_settings_with_key(
        &self,
        callback_key: &'static str,
        call_name: &'static str,
        out_width: u32,
        out_height: u32,
        quality: DlssPerfQuality,
    ) -> Result<DlssOptimalSettings> {
        #[cfg(has_ngx_sdk)]
        {
            let mut params = self.capability_parameters()?;
            let cb_ptr = params.get_void_ptr(callback_key)?;
            if cb_ptr.is_null() {
                return Err(Error::Ngx {
                    call: call_name,
                    code: ResultCode::OUT_OF_DATE,
                });
            }

            params.set_u32(keys::WIDTH, out_width)?;
            params.set_u32(keys::HEIGHT, out_height)?;
            params.set_perf_quality(quality)?;
            params.set_i32(keys::RTX_VALUE, 0)?;

            // SAFETY: `cb_ptr` is non-null and returned by NGX under the optimal-settings callback
            // key with signature `PFN_NVSDK_NGX_DLSS_GetOptimalSettingsCallback`.
            let callback: unsafe extern "C" fn(
                *mut ngx_sys::NVSDK_NGX_Parameter,
            ) -> ngx_sys::NVSDK_NGX_Result = unsafe { std::mem::transmute(cb_ptr) };
            let params_ptr = params.as_mut_ptr();
            let rc = crate::ngx_call(|| {
                // SAFETY: `params_ptr` is a valid capability `NVSDK_NGX_Parameter` bag.
                unsafe { callback(params_ptr) }
            });
            check_sys(call_name, rc)?;

            // A runtime that does not publish the dynamic-resolution keys reports the optimal
            // size, exactly as the SDK helper does (`nvsdk_ngx_helpers.h:103-111`).
            let render_width = params.get_u32(keys::OUT_WIDTH)?;
            let render_height = params.get_u32(keys::OUT_HEIGHT)?;
            let max_width = params
                .get_u32(keys::DLSS_DYNAMIC_MAX_RENDER_WIDTH)
                .unwrap_or(render_width);
            let max_height = params
                .get_u32(keys::DLSS_DYNAMIC_MAX_RENDER_HEIGHT)
                .unwrap_or(render_height);
            let min_width = params
                .get_u32(keys::DLSS_DYNAMIC_MIN_RENDER_WIDTH)
                .unwrap_or(render_width);
            let min_height = params
                .get_u32(keys::DLSS_DYNAMIC_MIN_RENDER_HEIGHT)
                .unwrap_or(render_height);
            let sharpness = params.get_f32(keys::SHARPNESS).unwrap_or(0.0);

            Ok(DlssOptimalSettings {
                render_width,
                render_height,
                min_width,
                min_height,
                max_width,
                max_height,
                sharpness,
            })
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (callback_key, call_name, out_width, out_height, quality);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Queries recommended render dimensions and sharpness for DLSS Super Resolution
    /// (`NGX_DLSS_GET_OPTIMAL_SETTINGS`).
    pub fn query_dlss_optimal_settings(
        &self,
        out_width: u32,
        out_height: u32,
        quality: DlssPerfQuality,
    ) -> Result<DlssOptimalSettings> {
        self.query_optimal_settings_with_key(
            keys::DLSS_OPTIMAL_SETTINGS_CALLBACK,
            "NGX_DLSS_GET_OPTIMAL_SETTINGS",
            out_width,
            out_height,
            quality,
        )
    }

    /// Queries recommended render dimensions and sharpness for DLSS Ray Reconstruction
    /// (`NGX_DLSSD_GET_OPTIMAL_SETTINGS`).
    pub fn query_dlss_rr_optimal_settings(
        &self,
        out_width: u32,
        out_height: u32,
        quality: DlssPerfQuality,
    ) -> Result<DlssOptimalSettings> {
        self.query_optimal_settings_with_key(
            keys::DLSSD_OPTIMAL_SETTINGS_CALLBACK,
            "NGX_DLSSD_GET_OPTIMAL_SETTINGS",
            out_width,
            out_height,
            quality,
        )
    }

    /// Creates a DLSS Super Resolution feature handle on `cmd_buffer`
    /// (`NGX_VULKAN_CREATE_DLSS_EXT`).
    ///
    /// # Safety
    ///
    /// `cmd_buffer` must be a valid, recording `VkCommandBuffer` belonging to the `VkDevice` used
    /// to initialize this [`NgxVulkan`] session.
    #[expect(
        clippy::too_many_arguments,
        reason = "mirrors the NGX_VULKAN_CREATE_DLSS_EXT helper arguments"
    )]
    pub unsafe fn create_dlss_sr(
        &self,
        cmd_buffer: *mut c_void,
        render_width: u32,
        render_height: u32,
        out_width: u32,
        out_height: u32,
        quality: DlssPerfQuality,
        flags: DlssFeatureFlags,
    ) -> Result<FeatureHandle> {
        #[cfg(has_ngx_sdk)]
        {
            let mut params = self.allocate_parameters()?;
            params.set_u32(keys::CREATION_NODE_MASK, 1)?;
            params.set_u32(keys::VISIBILITY_NODE_MASK, 1)?;
            params.set_render_dimensions(render_width, render_height)?;
            params.set_output_dimensions(out_width, out_height)?;
            params.set_perf_quality(quality)?;
            params.set_dlss_feature_flags(flags)?;
            params.set_i32(keys::DLSS_ENABLE_OUTPUT_SUBRECTS, 0)?;
            // SAFETY: Caller guarantees `cmd_buffer` is a valid recording `VkCommandBuffer`.
            unsafe { self.create_feature(cmd_buffer, Feature::SuperSampling, &mut params) }
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (
                cmd_buffer,
                render_width,
                render_height,
                out_width,
                out_height,
                quality,
                flags,
            );
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Creates a DLSS Ray Reconstruction feature handle on `cmd_buffer`
    /// (`NGX_VULKAN_CREATE_DLSSD_EXT1`).
    ///
    /// # Safety
    ///
    /// `cmd_buffer` must be a valid, recording `VkCommandBuffer` belonging to the `VkDevice` used
    /// to initialize this [`NgxVulkan`] session.
    pub unsafe fn create_dlss_rr(
        &self,
        cmd_buffer: *mut c_void,
        config: &DlssRrCreateParams,
    ) -> Result<FeatureHandle> {
        #[cfg(has_ngx_sdk)]
        {
            let mut params = self.allocate_parameters()?;
            params.set_u32(keys::CREATION_NODE_MASK, 1)?;
            params.set_u32(keys::VISIBILITY_NODE_MASK, 1)?;
            params.set_render_dimensions(config.render_width, config.render_height)?;
            params.set_output_dimensions(config.out_width, config.out_height)?;
            params.set_perf_quality(config.quality)?;
            params.set_dlss_feature_flags(config.flags)?;
            params.set_i32(keys::DLSS_ENABLE_OUTPUT_SUBRECTS, 0)?;
            params.set_dlss_denoise_mode(DlssDenoiseMode::DlUnified)?;
            params.set_dlss_roughness_mode(config.roughness_mode)?;
            params.set_dlss_depth_type(config.depth_type)?;
            if config.preset != DlssRrPreset::Default {
                params.set_dlss_rr_preset(config.quality, config.preset)?;
            }
            // SAFETY: Caller guarantees `cmd_buffer` is a valid recording `VkCommandBuffer`.
            unsafe { self.create_feature(cmd_buffer, Feature::RayReconstruction, &mut params) }
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (cmd_buffer, config);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }
}

/// RAII handle to a created NGX feature (`NVSDK_NGX_Handle`).
///
/// Released automatically on [`Drop`] via `NVSDK_NGX_VULKAN_ReleaseFeature`, which frees the
/// feature's memory immediately (`nvsdk_ngx_vk.h:548-551`): drop it only after the GPU has
/// finished every command buffer recorded by its `evaluate*` calls. A failed release is kept for
/// [`crate::take_teardown_error`].
pub struct FeatureHandle {
    #[cfg(has_ngx_sdk)]
    handle: *mut ngx_sys::NVSDK_NGX_Handle,
    #[cfg(has_ngx_sdk)]
    _session: Arc<VulkanSession>,
}

#[cfg(has_ngx_sdk)]
impl Drop for FeatureHandle {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            let handle = self.handle;
            crate::ngx_locked(|g| {
                // SAFETY: `handle` was created by `NVSDK_NGX_VULKAN_CreateFeature` and `_session`
                // keeps the NGX Vulkan device alive.
                let rc = unsafe { ngx_sys::NVSDK_NGX_VULKAN_ReleaseFeature(handle) };
                g.record_teardown("NVSDK_NGX_VULKAN_ReleaseFeature", rc);
            });
            self.handle = std::ptr::null_mut();
        }
    }
}

impl FeatureHandle {
    /// Evaluates the NGX feature on `cmd_buffer` (`NVSDK_NGX_VULKAN_EvaluateFeature_C`).
    ///
    /// # Safety
    ///
    /// `cmd_buffer` must be a valid, recording `VkCommandBuffer` and all resource pointers
    /// referenced inside `params` must remain valid until GPU execution completes.
    pub unsafe fn evaluate(&self, cmd_buffer: *mut c_void, params: &Parameters) -> Result<()> {
        #[cfg(has_ngx_sdk)]
        {
            if cmd_buffer.is_null() {
                return Err(Error::InvalidArgument("cmd_buffer must not be null"));
            }
            let vk_cmd = cmd_buffer.cast::<ngx_sys::VkCommandBuffer_T>();
            let rc = crate::ngx_call(|| {
                // SAFETY: `self.handle` and `params.as_ptr()` are valid; caller guarantees
                // `vk_cmd` and referenced GPU resources are valid.
                unsafe {
                    ngx_sys::NVSDK_NGX_VULKAN_EvaluateFeature_C(
                        vk_cmd,
                        self.handle,
                        params.as_ptr(),
                        None,
                    )
                }
            });
            check_sys("NVSDK_NGX_VULKAN_EvaluateFeature_C", rc)
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (cmd_buffer, params);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Evaluates a DLSS Super Resolution feature on `cmd_buffer` using a caller-supplied
    /// [`Parameters`] bag (`NGX_VULKAN_EVALUATE_DLSS_EXT`) to avoid per-frame allocations.
    ///
    /// # Safety
    ///
    /// `cmd_buffer` must be a valid, recording `VkCommandBuffer` and all `VkImage` / `VkImageView`
    /// handles in `color`, `depth`, `motion`, and `output` must remain valid until GPU execution
    /// completes.
    ///
    /// `output` must have `read_write = true` and the render subrect must fit inside `color`;
    /// otherwise this returns [`Error::InvalidArgument`]. `sharpness` is forwarded as `Sharpness`
    /// like the SDK helper (`nvsdk_ngx_helpers_vk.h:178`), but current DLSS ignores it
    /// (`nvsdk_ngx_defs.h:296`: "Sharpness is not supported"). The resource keys are reset to null
    /// before returning, so `params` never keeps pointers to this call's stack.
    #[expect(
        clippy::too_many_arguments,
        reason = "mirrors the NGX_VULKAN_EVALUATE_DLSS_EXT helper arguments"
    )]
    pub unsafe fn evaluate_dlss_sr_with_params(
        &self,
        cmd_buffer: *mut c_void,
        params: &mut Parameters,
        color: VkImageResourceInfo,
        depth: VkImageResourceInfo,
        motion: VkImageResourceInfo,
        output: VkImageResourceInfo,
        render_width: u32,
        render_height: u32,
        jitter_x: f32,
        jitter_y: f32,
        mv_scale_x: f32,
        mv_scale_y: f32,
        sharpness: f32,
        reset: bool,
    ) -> Result<()> {
        #[cfg(has_ngx_sdk)]
        {
            if cmd_buffer.is_null() {
                return Err(Error::InvalidArgument("cmd_buffer must not be null"));
            }
            color.validate("color")?;
            depth.validate("depth")?;
            motion.validate("motion")?;
            output.validate_output()?;
            validate_render_extent(&color, render_width, render_height)?;

            let depth_aspect = depth_aspect_mask(depth.format);
            let mut color_res = color.into_ngx_resource(1);
            let mut depth_res = depth.into_ngx_resource(depth_aspect);
            let mut motion_res = motion.into_ngx_resource(1);
            let mut output_res = output.into_ngx_resource(1);

            // SAFETY: Each stack resource outlives `self.evaluate(cmd_buffer, params)`, and
            // `clear_pointer_keys` nulls the keys before they go out of scope.
            unsafe {
                params.set_void_ptr(keys::COLOR, (&raw mut color_res).cast())?;
                params.set_void_ptr(keys::OUTPUT, (&raw mut output_res).cast())?;
                params.set_void_ptr(keys::DEPTH, (&raw mut depth_res).cast())?;
                params.set_void_ptr(keys::MOTION_VECTORS, (&raw mut motion_res).cast())?;
            }
            params.set_f32(keys::JITTER_OFFSET_X, jitter_x)?;
            params.set_f32(keys::JITTER_OFFSET_Y, jitter_y)?;
            params.set_f32(keys::SHARPNESS, sharpness)?;
            params.set_i32(keys::RESET, i32::from(reset))?;
            params.set_f32(
                keys::MV_SCALE_X,
                if mv_scale_x == 0.0 { 1.0 } else { mv_scale_x },
            )?;
            params.set_f32(
                keys::MV_SCALE_Y,
                if mv_scale_y == 0.0 { 1.0 } else { mv_scale_y },
            )?;
            params.set_u32(keys::DLSS_RENDER_SUBRECT_WIDTH, render_width)?;
            params.set_u32(keys::DLSS_RENDER_SUBRECT_HEIGHT, render_height)?;
            params.set_f32(keys::DLSS_PRE_EXPOSURE, 1.0)?;
            params.set_f32(keys::DLSS_EXPOSURE_SCALE, 1.0)?;

            // SAFETY: Caller guarantees `cmd_buffer` and referenced GPU resources are valid.
            let eval_res = unsafe { self.evaluate(cmd_buffer, params) };
            let clear_res = clear_pointer_keys(
                params,
                &[keys::COLOR, keys::OUTPUT, keys::DEPTH, keys::MOTION_VECTORS],
            );
            eval_res.and(clear_res)
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (
                cmd_buffer,
                params,
                color,
                depth,
                motion,
                output,
                render_width,
                render_height,
                jitter_x,
                jitter_y,
                mv_scale_x,
                mv_scale_y,
                sharpness,
                reset,
            );
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Evaluates a DLSS Super Resolution feature on `cmd_buffer` (`NGX_VULKAN_EVALUATE_DLSS_EXT`).
    ///
    /// # Safety
    ///
    /// `cmd_buffer` must be a valid, recording `VkCommandBuffer` and all `VkImage` / `VkImageView`
    /// handles in `color`, `depth`, `motion`, and `output` must remain valid until GPU execution
    /// completes.
    #[expect(
        clippy::too_many_arguments,
        reason = "mirrors the NGX_VULKAN_EVALUATE_DLSS_EXT helper arguments"
    )]
    pub unsafe fn evaluate_dlss_sr(
        &self,
        cmd_buffer: *mut c_void,
        color: VkImageResourceInfo,
        depth: VkImageResourceInfo,
        motion: VkImageResourceInfo,
        output: VkImageResourceInfo,
        render_width: u32,
        render_height: u32,
        jitter_x: f32,
        jitter_y: f32,
        mv_scale_x: f32,
        mv_scale_y: f32,
        sharpness: f32,
        reset: bool,
    ) -> Result<()> {
        #[cfg(has_ngx_sdk)]
        {
            let mut params = allocate_session_parameters(&self._session)?;

            // SAFETY: Caller guarantees `cmd_buffer` and referenced GPU resources are valid.
            unsafe {
                self.evaluate_dlss_sr_with_params(
                    cmd_buffer,
                    &mut params,
                    color,
                    depth,
                    motion,
                    output,
                    render_width,
                    render_height,
                    jitter_x,
                    jitter_y,
                    mv_scale_x,
                    mv_scale_y,
                    sharpness,
                    reset,
                )
            }
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (
                cmd_buffer,
                color,
                depth,
                motion,
                output,
                render_width,
                render_height,
                jitter_x,
                jitter_y,
                mv_scale_x,
                mv_scale_y,
                sharpness,
                reset,
            );
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Evaluates a DLSS Ray Reconstruction feature on `cmd_buffer` using a caller-supplied
    /// [`Parameters`] bag (`NGX_VULKAN_EVALUATE_DLSSD_EXT`) to avoid per-frame allocations.
    ///
    /// # Safety
    ///
    /// `cmd_buffer` must be a valid, recording `VkCommandBuffer` and all `VkImage` / `VkImageView`
    /// handles in `eval` must remain valid until GPU execution completes.
    ///
    /// `eval.output` must have `read_write = true` and the render subrect must fit inside
    /// `eval.color`; otherwise this returns [`Error::InvalidArgument`]. The resource and matrix
    /// keys are reset to null before returning, so `params` never keeps pointers to this call's
    /// stack.
    pub unsafe fn evaluate_dlss_rr_with_params(
        &self,
        cmd_buffer: *mut c_void,
        params: &mut Parameters,
        eval: &DlssRrEvalParams,
    ) -> Result<()> {
        #[cfg(has_ngx_sdk)]
        {
            if cmd_buffer.is_null() {
                return Err(Error::InvalidArgument("cmd_buffer must not be null"));
            }
            eval.color.validate("color")?;
            eval.depth.validate("depth")?;
            eval.motion.validate("motion")?;
            eval.diffuse_albedo.validate("diffuse_albedo")?;
            eval.specular_albedo.validate("specular_albedo")?;
            eval.normals.validate("normals")?;
            if let Some(roughness) = eval.roughness {
                roughness.validate("roughness")?;
            }
            eval.output.validate_output()?;
            validate_render_extent(&eval.color, eval.render_width, eval.render_height)?;

            let mut color_res = eval.color.into_ngx_resource(1);
            let mut depth_res = eval
                .depth
                .into_ngx_resource(depth_aspect_mask(eval.depth.format));
            let mut motion_res = eval.motion.into_ngx_resource(1);
            let mut diffuse_res = eval.diffuse_albedo.into_ngx_resource(1);
            let mut specular_res = eval.specular_albedo.into_ngx_resource(1);
            let mut normals_res = eval.normals.into_ngx_resource(1);
            let mut roughness_res = eval.roughness.map(|r| r.into_ngx_resource(1));
            let mut output_res = eval.output.into_ngx_resource(1);
            let mut world_to_view = eval.world_to_view;
            let mut view_to_clip = eval.view_to_clip;

            let roughness_ptr = roughness_res
                .as_mut()
                .map_or(std::ptr::null_mut(), |r| (&raw mut *r).cast());
            let w2v_ptr = world_to_view
                .as_mut()
                .map_or(std::ptr::null_mut(), |m| m.as_mut_ptr().cast());
            let v2c_ptr = view_to_clip
                .as_mut()
                .map_or(std::ptr::null_mut(), |m| m.as_mut_ptr().cast());

            // SAFETY: Every stack resource and optional matrix outlives
            // `self.evaluate(cmd_buffer, params)`, and `clear_pointer_keys` nulls the keys before
            // they go out of scope.
            unsafe {
                params.set_void_ptr(keys::COLOR, (&raw mut color_res).cast())?;
                params.set_void_ptr(keys::OUTPUT, (&raw mut output_res).cast())?;
                params.set_void_ptr(keys::DEPTH, (&raw mut depth_res).cast())?;
                params.set_void_ptr(keys::MOTION_VECTORS, (&raw mut motion_res).cast())?;
                params.set_void_ptr(
                    keys::DLSS_INPUT_DIFFUSE_ALBEDO,
                    (&raw mut diffuse_res).cast(),
                )?;
                params.set_void_ptr(
                    keys::DLSS_INPUT_SPECULAR_ALBEDO,
                    (&raw mut specular_res).cast(),
                )?;
                params.set_void_ptr(keys::GBUFFER_NORMALS, (&raw mut normals_res).cast())?;
                params.set_void_ptr(keys::GBUFFER_ROUGHNESS, roughness_ptr)?;
                params.set_void_ptr(keys::WORLD_TO_VIEW_MATRIX, w2v_ptr)?;
                params.set_void_ptr(keys::VIEW_TO_CLIP_MATRIX, v2c_ptr)?;
            }
            params.set_f32(keys::JITTER_OFFSET_X, eval.jitter_x)?;
            params.set_f32(keys::JITTER_OFFSET_Y, eval.jitter_y)?;
            params.set_i32(keys::RESET, i32::from(eval.reset))?;
            params.set_f32(
                keys::MV_SCALE_X,
                if eval.mv_scale_x == 0.0 {
                    1.0
                } else {
                    eval.mv_scale_x
                },
            )?;
            params.set_f32(
                keys::MV_SCALE_Y,
                if eval.mv_scale_y == 0.0 {
                    1.0
                } else {
                    eval.mv_scale_y
                },
            )?;
            params.set_u32(keys::DLSS_RENDER_SUBRECT_WIDTH, eval.render_width)?;
            params.set_u32(keys::DLSS_RENDER_SUBRECT_HEIGHT, eval.render_height)?;
            params.set_f32(
                keys::DLSS_PRE_EXPOSURE,
                if eval.pre_exposure == 0.0 {
                    1.0
                } else {
                    eval.pre_exposure
                },
            )?;
            params.set_f32(
                keys::DLSS_EXPOSURE_SCALE,
                if eval.exposure_scale == 0.0 {
                    1.0
                } else {
                    eval.exposure_scale
                },
            )?;

            // SAFETY: Caller guarantees `cmd_buffer` and referenced GPU resources are valid.
            let eval_res = unsafe { self.evaluate(cmd_buffer, params) };
            let clear_res = clear_pointer_keys(
                params,
                &[
                    keys::COLOR,
                    keys::OUTPUT,
                    keys::DEPTH,
                    keys::MOTION_VECTORS,
                    keys::DLSS_INPUT_DIFFUSE_ALBEDO,
                    keys::DLSS_INPUT_SPECULAR_ALBEDO,
                    keys::GBUFFER_NORMALS,
                    keys::GBUFFER_ROUGHNESS,
                    keys::WORLD_TO_VIEW_MATRIX,
                    keys::VIEW_TO_CLIP_MATRIX,
                ],
            );
            eval_res.and(clear_res)
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (cmd_buffer, params, eval);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }
}

/// Validated and typed Vulkan handles ready for NGX initialization FFI calls.
#[cfg(has_ngx_sdk)]
struct PreparedVulkanHandles {
    instance: ngx_sys::VkInstance,
    physical_device: ngx_sys::VkPhysicalDevice,
    device: ngx_sys::VkDevice,
    gipa: ngx_sys::PFN_vkGetInstanceProcAddr,
    gdpa: ngx_sys::PFN_vkGetDeviceProcAddr,
}

#[cfg(all(has_ngx_sdk, windows))]
type NgxWchar = u16;
#[cfg(all(has_ngx_sdk, not(windows)))]
type NgxWchar = i32;

#[cfg(has_ngx_sdk)]
pub(crate) fn to_wchar_nul(path: &Path) -> Vec<NgxWchar> {
    #[cfg(windows)]
    {
        path.as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }
    #[cfg(not(windows))]
    {
        path.to_string_lossy()
            .chars()
            .map(|c| c as i32)
            .chain(std::iter::once(0))
            .collect()
    }
}

/// Validates `handles`, encodes `app_data_path` as NUL-terminated `wchar_t`, invokes `init_fn`
/// under the NGX lock, and wraps the resulting `VkDevice` in an RAII [`NgxVulkan`] session.
/// Returns [`Error::InvalidArgument`] if a session for the same `VkDevice` is still alive.
///
/// # Safety
///
/// Caller must uphold the same Vulkan handle and loader pointer lifetime guarantees as
/// [`NgxVulkan::init`] and [`NgxVulkan::init_with_app_id`].
#[cfg(has_ngx_sdk)]
unsafe fn init_vulkan_session(
    app_data_path: &Path,
    handles: VulkanDeviceHandles,
    call_name: &'static str,
    init_fn: impl FnOnce(*const NgxWchar, &PreparedVulkanHandles) -> ngx_sys::NVSDK_NGX_Result,
) -> Result<NgxVulkan> {
    if handles.instance.is_null()
        || handles.physical_device.is_null()
        || handles.device.is_null()
        || handles.get_instance_proc_addr.is_null()
        || handles.get_device_proc_addr.is_null()
    {
        return Err(Error::InvalidArgument(
            "VulkanDeviceHandles must not contain null pointers",
        ));
    }

    let wide_path = to_wchar_nul(app_data_path);

    let prepared = PreparedVulkanHandles {
        instance: handles.instance.cast::<ngx_sys::VkInstance_T>(),
        physical_device: handles
            .physical_device
            .cast::<ngx_sys::VkPhysicalDevice_T>(),
        device: handles.device.cast::<ngx_sys::VkDevice_T>(),
        // SAFETY: Caller guarantees `get_instance_proc_addr` is a valid non-null
        // `PFN_vkGetInstanceProcAddr` function pointer.
        gipa: unsafe {
            std::mem::transmute::<*const c_void, ngx_sys::PFN_vkGetInstanceProcAddr>(
                handles.get_instance_proc_addr,
            )
        },
        // SAFETY: Caller guarantees `get_device_proc_addr` is a valid non-null
        // `PFN_vkGetDeviceProcAddr` function pointer.
        gdpa: unsafe {
            std::mem::transmute::<*const c_void, ngx_sys::PFN_vkGetDeviceProcAddr>(
                handles.get_device_proc_addr,
            )
        },
    };

    let device_addr = prepared.device.addr();
    crate::ngx_locked(|g| -> Result<()> {
        g.claim_device(device_addr)?;
        let res = check_sys(call_name, init_fn(wide_path.as_ptr(), &prepared));
        if res.is_err() {
            g.release_device(device_addr);
        }
        res
    })?;
    Ok(NgxVulkan {
        session: Arc::new(VulkanSession {
            device: prepared.device,
        }),
    })
}

/// Converts a C string pointer array `(ptr, count)` returned by NGX into `Vec<String>`.
///
/// # Safety
///
/// If `count > 0`, `ptr` must point to `count` valid NUL-terminated C string pointers.
#[cfg(has_ngx_sdk)]
unsafe fn read_cstr_array(ptr: *mut *const std::os::raw::c_char, count: u32) -> Vec<String> {
    if ptr.is_null() || count == 0 {
        return Vec::new();
    }
    // SAFETY: Caller guarantees `ptr` points to `count` valid `*const c_char` elements.
    let slice = unsafe { std::slice::from_raw_parts(ptr, count as usize) };
    slice
        .iter()
        .filter(|&&p| !p.is_null())
        // SAFETY: Each non-null element is a NUL-terminated C string owned by NGX.
        .map(|&p| unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned())
        .collect()
}

#[cfg(all(test, has_ngx_sdk))]
mod tests {
    use super::*;

    fn fake_image(width: u32, height: u32, read_write: bool) -> VkImageResourceInfo {
        // Non-null placeholders: these tests only exercise host-side validation, never NGX.
        VkImageResourceInfo {
            image_view: std::ptr::dangling_mut::<u8>().cast(),
            image: std::ptr::dangling_mut::<u8>().cast(),
            format: 97,
            width,
            height,
            read_write,
        }
    }

    #[test]
    fn output_without_read_write_is_rejected_before_ngx() {
        assert!(matches!(
            fake_image(1920, 1080, false).validate_output(),
            Err(Error::InvalidArgument(_))
        ));
        assert!(fake_image(1920, 1080, true).validate_output().is_ok());
        let null_output = VkImageResourceInfo {
            image_view: std::ptr::null_mut(),
            ..fake_image(1920, 1080, true)
        };
        assert!(matches!(
            null_output.validate_output(),
            Err(Error::InvalidArgument(_))
        ));
    }

    #[test]
    fn render_subrect_larger_than_color_is_rejected() {
        let color = fake_image(960, 540, false);
        assert!(validate_render_extent(&color, 960, 540).is_ok());
        assert!(validate_render_extent(&color, 640, 360).is_ok());
        assert!(matches!(
            validate_render_extent(&color, 961, 540),
            Err(Error::InvalidArgument(_))
        ));
        assert!(matches!(
            validate_render_extent(&color, 960, 541),
            Err(Error::InvalidArgument(_))
        ));
    }

    #[test]
    fn unpublished_capability_key_maps_to_none_only_for_ngx_status() {
        assert_eq!(published(Ok(7_i32)).ok(), Some(Some(7)));
        let missing: Result<i32> = Err(Error::Ngx {
            call: "NVSDK_NGX_Parameter_GetI",
            code: ResultCode::OUT_OF_DATE,
        });
        assert!(matches!(published(missing), Ok(None)));
        let invalid: Result<i32> = Err(Error::InvalidArgument("x"));
        assert!(matches!(published(invalid), Err(Error::InvalidArgument(_))));
    }
}
