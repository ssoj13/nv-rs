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
//! NGX SDK is present); targets without the SDK compile clean stubs returning [`Error::Unsupported`].
//! **Used by** `ngx` consumers (renderers / upscalers) and T2 GPU smoke tests.

use crate::error::{Error, Result};
use crate::params::{Parameters, keys};
use crate::types::{DlssFeatureFlags, DlssPerfQuality, Feature, ProjectId, RequiredExtensions};
use std::ffi::c_void;
use std::path::Path;

#[cfg(has_ngx_sdk)]
use crate::check_sys;
#[cfg(has_ngx_sdk)]
use crate::error::ResultCode;
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
                _ => "output VkImageResourceInfo must not have null image_view or image",
            }));
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

/// Optimal render dimensions and sharpness returned by `NGX_DLSS_GET_OPTIMAL_SETTINGS`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DlssOptimalSettings {
    /// Recommended render width in pixels.
    pub render_width: u32,
    /// Recommended render height in pixels.
    pub render_height: u32,
    /// Minimum dynamic render width in pixels.
    pub min_width: u32,
    /// Minimum dynamic render height in pixels.
    pub min_height: u32,
    /// Maximum dynamic render width in pixels.
    pub max_width: u32,
    /// Maximum dynamic render height in pixels.
    pub max_height: u32,
    /// Recommended sharpening factor.
    pub sharpness: f32,
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

// SAFETY: `VulkanSession` only stores the `VkDevice` handle for `NVSDK_NGX_VULKAN_Shutdown1`
// on the final `Arc::drop`, which is synchronized by `Arc`'s atomic reference count.
#[cfg(has_ngx_sdk)]
unsafe impl Send for VulkanSession {}
// SAFETY: `VulkanSession` exposes no interior mutability; `Shutdown1` runs exclusively on
// last `Arc::drop`.
#[cfg(has_ngx_sdk)]
unsafe impl Sync for VulkanSession {}

#[cfg(has_ngx_sdk)]
impl Drop for VulkanSession {
    fn drop(&mut self) {
        if !self.device.is_null() {
            // SAFETY: `self.device` was initialized by `NVSDK_NGX_VULKAN_Init_with_ProjectID`
            // and all `FeatureHandle` / `Parameters` children hold `Arc<VulkanSession>`, so this
            // runs only after every child has been dropped.
            let _ = unsafe { ngx_sys::NVSDK_NGX_VULKAN_Shutdown1(self.device) };
            self.device = std::ptr::null_mut();
        }
    }
}

/// RAII handle to an initialized NGX Vulkan session.
///
/// Cloning [`NgxVulkan`] shares the underlying `Arc` session; `NVSDK_NGX_VULKAN_Shutdown1` runs
/// once when the last [`NgxVulkan`], [`Parameters`], and [`FeatureHandle`] are dropped.
#[derive(Clone)]
pub struct NgxVulkan {
    #[cfg(has_ngx_sdk)]
    session: Arc<VulkanSession>,
}

impl NgxVulkan {
    /// Queries the Vulkan instance and device extensions required by NGX
    /// (`NVSDK_NGX_VULKAN_RequiredExtensions`).
    pub fn required_extensions() -> Result<RequiredExtensions> {
        #[cfg(has_ngx_sdk)]
        {
            let mut inst_count: u32 = 0;
            let mut inst_exts: *mut *const std::os::raw::c_char = std::ptr::null_mut();
            let mut dev_count: u32 = 0;
            let mut dev_exts: *mut *const std::os::raw::c_char = std::ptr::null_mut();
            // SAFETY: Output pointers are valid stack locals.
            let rc = unsafe {
                ngx_sys::NVSDK_NGX_VULKAN_RequiredExtensions(
                    &mut inst_count,
                    &mut inst_exts,
                    &mut dev_count,
                    &mut dev_exts,
                )
            };
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
    /// required.
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
            let mut ptr: *mut ngx_sys::NVSDK_NGX_Parameter = std::ptr::null_mut();
            // SAFETY: NGX Vulkan session is active (`self.session`) and `ptr` is writable.
            let rc = unsafe { ngx_sys::NVSDK_NGX_VULKAN_AllocateParameters(&mut ptr) };
            check_sys("NVSDK_NGX_VULKAN_AllocateParameters", rc)?;
            Parameters::from_raw(ptr, Arc::clone(&self.session))
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
            // SAFETY: NGX Vulkan session is active (`self.session`) and `ptr` is writable.
            let rc = unsafe { ngx_sys::NVSDK_NGX_VULKAN_GetCapabilityParameters(&mut ptr) };
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
            // SAFETY: `params.as_ptr()` is a valid `NVSDK_NGX_Parameter` and `size` is writable.
            let rc = unsafe {
                ngx_sys::NVSDK_NGX_VULKAN_GetScratchBufferSize(
                    ngx_sys::NVSDK_NGX_Feature::from_ne_bytes(feature.to_raw().to_ne_bytes()),
                    params.as_ptr(),
                    &mut size,
                )
            };
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
            // SAFETY: Caller guarantees `vk_cmd` is a valid recording `VkCommandBuffer`;
            // `params.as_mut_ptr()` and `handle` are valid.
            let rc = unsafe {
                ngx_sys::NVSDK_NGX_VULKAN_CreateFeature(
                    vk_cmd,
                    ngx_sys::NVSDK_NGX_Feature::from_ne_bytes(feature.to_raw().to_ne_bytes()),
                    params.as_mut_ptr(),
                    &mut handle,
                )
            };
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

    /// Queries recommended render dimensions and sharpness for DLSS Super Resolution
    /// (`NGX_DLSS_GET_OPTIMAL_SETTINGS`).
    pub fn query_dlss_optimal_settings(
        &self,
        out_width: u32,
        out_height: u32,
        quality: DlssPerfQuality,
    ) -> Result<DlssOptimalSettings> {
        #[cfg(has_ngx_sdk)]
        {
            let mut params = self.capability_parameters()?;
            let cb_ptr = params.get_void_ptr(keys::DLSS_OPTIMAL_SETTINGS_CALLBACK)?;
            if cb_ptr.is_null() {
                // Match `NGX_DLSS_GET_OPTIMAL_SETTINGS`: missing callback means DLSS is out of date.
                return Err(Error::Ngx {
                    call: "NGX_DLSS_GET_OPTIMAL_SETTINGS",
                    code: ResultCode::OUT_OF_DATE,
                });
            }

            params.set_u32(keys::WIDTH, out_width)?;
            params.set_u32(keys::HEIGHT, out_height)?;
            params.set_perf_quality(quality)?;
            params.set_i32(keys::RTX_VALUE, 0)?;

            // SAFETY: `cb_ptr` is non-null and returned by NGX under `DLSSOptimalSettingsCallback`
            // with signature `PFN_NVSDK_NGX_DLSS_GetOptimalSettingsCallback`.
            let callback: unsafe extern "C" fn(
                *mut ngx_sys::NVSDK_NGX_Parameter,
            ) -> ngx_sys::NVSDK_NGX_Result = unsafe { std::mem::transmute(cb_ptr) };
            // SAFETY: `params.as_mut_ptr()` is a valid capability `NVSDK_NGX_Parameter` bag.
            let rc = unsafe { callback(params.as_mut_ptr()) };
            check_sys("NVSDK_NGX_DLSS_GetOptimalSettingsCallback", rc)?;

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
            let _ = (out_width, out_height, quality);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Creates a DLSS Super Resolution feature handle on `cmd_buffer`
    /// (`NGX_VULKAN_CREATE_DLSS_EXT`).
    ///
    /// # Safety
    ///
    /// `cmd_buffer` must be a valid, recording `VkCommandBuffer` belonging to the `VkDevice` used
    /// to initialize this [`NgxVulkan`] session.
    #[allow(clippy::too_many_arguments)]
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
}

/// RAII handle to a created NGX feature (`NVSDK_NGX_Handle`).
///
/// Released automatically on [`Drop`] via `NVSDK_NGX_VULKAN_ReleaseFeature`.
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
            // SAFETY: `self.handle` was created by `NVSDK_NGX_VULKAN_CreateFeature` and
            // `_session` keeps the NGX Vulkan device alive.
            let _ = unsafe { ngx_sys::NVSDK_NGX_VULKAN_ReleaseFeature(self.handle) };
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
            // SAFETY: `self.handle` and `params.as_ptr()` are valid; caller guarantees `vk_cmd`
            // and referenced GPU resources are valid.
            let rc = unsafe {
                ngx_sys::NVSDK_NGX_VULKAN_EvaluateFeature_C(
                    vk_cmd,
                    self.handle,
                    params.as_ptr(),
                    None,
                )
            };
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
    #[allow(clippy::too_many_arguments)]
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
            output.validate("output")?;

            // `VK_IMAGE_ASPECT_COLOR_BIT = 1`, `VK_IMAGE_ASPECT_DEPTH_BIT = 2`.
            // Depth may be supplied either as a depth/stencil format (`124..=130`) or as a
            // linear `R32_SFLOAT` (`100`) color texture; using `DEPTH_BIT` on `R32_SFLOAT` is invalid.
            const VK_FORMAT_D16_UNORM: u32 = 124;
            const VK_FORMAT_D32_SFLOAT_S8_UINT: u32 = 130;
            let depth_aspect =
                if (VK_FORMAT_D16_UNORM..=VK_FORMAT_D32_SFLOAT_S8_UINT).contains(&depth.format) {
                    2
                } else {
                    1
                };
            let mut color_res = color.into_ngx_resource(1);
            let mut depth_res = depth.into_ngx_resource(depth_aspect);
            let mut motion_res = motion.into_ngx_resource(1);
            let mut output_res = output.into_ngx_resource(1);

            // SAFETY: Each stack resource outlives `self.evaluate(cmd_buffer, params)`.
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
            unsafe { self.evaluate(cmd_buffer, params) }
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
    #[allow(clippy::too_many_arguments)]
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
            let mut param_ptr: *mut ngx_sys::NVSDK_NGX_Parameter = std::ptr::null_mut();
            // SAFETY: `_session` keeps the NGX Vulkan device alive and `param_ptr` is writable.
            let rc = unsafe { ngx_sys::NVSDK_NGX_VULKAN_AllocateParameters(&mut param_ptr) };
            check_sys("NVSDK_NGX_VULKAN_AllocateParameters", rc)?;
            let mut params = Parameters::from_raw(param_ptr, Arc::clone(&self._session))?;

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

/// Validates `handles`, encodes `app_data_path` as NUL-terminated `wchar_t`, invokes `init_fn`,
/// and wraps the resulting `VkDevice` in an RAII [`NgxVulkan`] session.
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

    let rc = init_fn(wide_path.as_ptr(), &prepared);
    check_sys(call_name, rc)?;
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
