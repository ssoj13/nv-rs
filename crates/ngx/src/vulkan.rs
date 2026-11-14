//! Safe Vulkan RAII wrappers for NGX (`NgxVulkan`, `FeatureHandle`).
//!
//! **What.** Manages the NGX Vulkan session lifecycle (`NVSDK_NGX_VULKAN_Init` /
//! `NVSDK_NGX_VULKAN_Init_with_ProjectID` / `NVSDK_NGX_VULKAN_Shutdown1`), parameter bag allocation,
//! scratch-buffer sizing, and feature creation/evaluation/release (`CreateFeature` /
//! `EvaluateFeature_C` / `ReleaseFeature`).
//! **Why.** Guarantees deterministic teardown order via `Arc<VulkanSession>`: every
//! [`Parameters`] and [`FeatureHandle`] holds a reference to the session so `Shutdown1` never
//! runs while features or parameter bags are still alive.
//! **How.** Windows FFI calls live behind `#[cfg(target_os = "windows")]`; non-Windows targets
//! compile clean stubs returning [`Error::Unsupported`].
//! **Used by** `ngx` consumers (renderers / upscalers) and T2 GPU smoke tests.

use crate::error::{Error, Result};
use crate::params::Parameters;
use crate::types::{Feature, ProjectId, RequiredExtensions};
use std::ffi::c_void;
use std::path::Path;

#[cfg(target_os = "windows")]
use crate::error::check;
#[cfg(target_os = "windows")]
use std::ffi::CStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "windows")]
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

#[cfg(target_os = "windows")]
pub(crate) struct VulkanSession {
    device: ngx_sys::VkDevice,
}

// SAFETY: `VulkanSession` only stores the `VkDevice` handle for `NVSDK_NGX_VULKAN_Shutdown1`
// on the final `Arc::drop`, which is synchronized by `Arc`'s atomic reference count.
#[cfg(target_os = "windows")]
unsafe impl Send for VulkanSession {}
// SAFETY: `VulkanSession` exposes no interior mutability; `Shutdown1` runs exclusively on
// last `Arc::drop`.
#[cfg(target_os = "windows")]
unsafe impl Sync for VulkanSession {}

#[cfg(target_os = "windows")]
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
    #[cfg(target_os = "windows")]
    session: Arc<VulkanSession>,
}

impl NgxVulkan {
    /// Queries the Vulkan instance and device extensions required by NGX
    /// (`NVSDK_NGX_VULKAN_RequiredExtensions`).
    pub fn required_extensions() -> Result<RequiredExtensions> {
        #[cfg(target_os = "windows")]
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
            check("NVSDK_NGX_VULKAN_RequiredExtensions", rc)?;
            Ok(RequiredExtensions {
                // SAFETY: NGX populates `inst_exts` with `inst_count` valid C strings.
                instance_extensions: unsafe { read_cstr_array(inst_exts, inst_count) },
                // SAFETY: NGX populates `dev_exts` with `dev_count` valid C strings.
                device_extensions: unsafe { read_cstr_array(dev_exts, dev_count) },
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
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
        #[cfg(target_os = "windows")]
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
                            std::ptr::null(),
                            ngx_sys::NVSDK_NGX_Version_NVSDK_NGX_Version_API,
                        )
                    },
                )
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (project_id, app_data_path, handles);
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
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
        #[cfg(target_os = "windows")]
        {
            // SAFETY: Caller guarantees `handles` contains live Vulkan handles and valid loader
            // function pointers; `wide_path` outlives the FFI call.
            unsafe {
                init_vulkan_session(
                    app_data_path,
                    handles,
                    "NVSDK_NGX_VULKAN_Init",
                    |wide_path, h| {
                        ngx_sys::NVSDK_NGX_VULKAN_Init(
                            app_id as std::os::raw::c_ulonglong,
                            wide_path,
                            h.instance,
                            h.physical_device,
                            h.device,
                            h.gipa,
                            h.gdpa,
                            std::ptr::null(),
                            ngx_sys::NVSDK_NGX_Version_NVSDK_NGX_Version_API,
                        )
                    },
                )
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (app_id, app_data_path, handles);
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }

    /// Allocates a mutable NGX parameter bag (`NVSDK_NGX_VULKAN_AllocateParameters`).
    pub fn allocate_parameters(&self) -> Result<Parameters> {
        #[cfg(target_os = "windows")]
        {
            let mut ptr: *mut ngx_sys::NVSDK_NGX_Parameter = std::ptr::null_mut();
            // SAFETY: NGX Vulkan session is active (`self.session`) and `ptr` is writable.
            let rc = unsafe { ngx_sys::NVSDK_NGX_VULKAN_AllocateParameters(&mut ptr) };
            check("NVSDK_NGX_VULKAN_AllocateParameters", rc)?;
            Parameters::from_raw(ptr, Arc::clone(&self.session))
        }
        #[cfg(not(target_os = "windows"))]
        {
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }

    /// Retrieves NGX capability parameters (`NVSDK_NGX_VULKAN_GetCapabilityParameters`).
    pub fn capability_parameters(&self) -> Result<Parameters> {
        #[cfg(target_os = "windows")]
        {
            let mut ptr: *mut ngx_sys::NVSDK_NGX_Parameter = std::ptr::null_mut();
            // SAFETY: NGX Vulkan session is active (`self.session`) and `ptr` is writable.
            let rc = unsafe { ngx_sys::NVSDK_NGX_VULKAN_GetCapabilityParameters(&mut ptr) };
            check("NVSDK_NGX_VULKAN_GetCapabilityParameters", rc)?;
            Parameters::from_raw(ptr, Arc::clone(&self.session))
        }
        #[cfg(not(target_os = "windows"))]
        {
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }

    /// Queries the scratch buffer size in bytes for `feature` (`NVSDK_NGX_VULKAN_GetScratchBufferSize`).
    pub fn scratch_buffer_size(&self, feature: Feature, params: &Parameters) -> Result<usize> {
        #[cfg(target_os = "windows")]
        {
            let mut size = 0usize;
            // SAFETY: `params.as_ptr()` is a valid `NVSDK_NGX_Parameter` and `size` is writable.
            let rc = unsafe {
                ngx_sys::NVSDK_NGX_VULKAN_GetScratchBufferSize(
                    feature.to_raw(),
                    params.as_ptr(),
                    &mut size,
                )
            };
            check("NVSDK_NGX_VULKAN_GetScratchBufferSize", rc)?;
            Ok(size)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (feature, params);
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
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
        #[cfg(target_os = "windows")]
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
                    feature.to_raw(),
                    params.as_mut_ptr(),
                    &mut handle,
                )
            };
            check("NVSDK_NGX_VULKAN_CreateFeature", rc)?;
            if handle.is_null() {
                return Err(Error::InvalidArgument("NGX returned null NVSDK_NGX_Handle"));
            }
            Ok(FeatureHandle {
                handle,
                _session: Arc::clone(&self.session),
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (cmd_buffer, feature, params);
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }
}

/// RAII handle to a created NGX feature (`NVSDK_NGX_Handle`).
///
/// Released automatically on [`Drop`] via `NVSDK_NGX_VULKAN_ReleaseFeature`.
pub struct FeatureHandle {
    #[cfg(target_os = "windows")]
    handle: *mut ngx_sys::NVSDK_NGX_Handle,
    #[cfg(target_os = "windows")]
    _session: Arc<VulkanSession>,
}

#[cfg(target_os = "windows")]
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
        #[cfg(target_os = "windows")]
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
            check("NVSDK_NGX_VULKAN_EvaluateFeature_C", rc)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (cmd_buffer, params);
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }
}

/// Validated and typed Vulkan handles ready for NGX initialization FFI calls.
#[cfg(target_os = "windows")]
struct PreparedVulkanHandles {
    instance: ngx_sys::VkInstance,
    physical_device: ngx_sys::VkPhysicalDevice,
    device: ngx_sys::VkDevice,
    gipa: ngx_sys::PFN_vkGetInstanceProcAddr,
    gdpa: ngx_sys::PFN_vkGetDeviceProcAddr,
}

/// Validates `handles`, encodes `app_data_path` as NUL-terminated UTF-16, invokes `init_fn`,
/// and wraps the resulting `VkDevice` in an RAII [`NgxVulkan`] session.
///
/// # Safety
///
/// Caller must uphold the same Vulkan handle and loader pointer lifetime guarantees as
/// [`NgxVulkan::init`] and [`NgxVulkan::init_with_app_id`].
#[cfg(target_os = "windows")]
unsafe fn init_vulkan_session(
    app_data_path: &Path,
    handles: VulkanDeviceHandles,
    call_name: &'static str,
    init_fn: impl FnOnce(*const u16, &PreparedVulkanHandles) -> ngx_sys::NVSDK_NGX_Result,
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

    let wide_path: Vec<u16> = app_data_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

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
    check(call_name, rc)?;
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
#[cfg(target_os = "windows")]
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
