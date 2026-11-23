//! Typed NGX parameter bag (`NVSDK_NGX_Parameter`) with RAII cleanup.
//!
//! **What.** Wraps `NVSDK_NGX_Parameter` allocated via `AllocateParameters` or
//! `GetCapabilityParameters`, destroys it on `Drop` via `DestroyParameters`, and exposes typed
//! setters/getters (`u64`, `u32`, `i32`, `f32`, `f64`, raw pointers) plus DLSS convenience helpers.
//! **Why.** Prevents parameter-bag leaks and ensures the parent [`crate::NgxVulkan`] session
//! outlives every [`Parameters`] instance via `Arc`.
//! **How.** Short parameter names (under 128 bytes) are NUL-terminated on the stack to avoid
//! per-frame heap allocations during `EvaluateFeature`.
//! **Used by** [`crate::NgxVulkan`] and [`crate::FeatureHandle`].

use crate::error::{Error, Result};
use crate::types::{
    DlssDenoiseMode, DlssDepthType, DlssFeatureFlags, DlssPerfQuality, DlssRoughnessMode,
    DlssRrPreset,
};
use std::ffi::c_void;

#[cfg(has_ngx_sdk)]
use crate::check_sys;
#[cfg(has_ngx_sdk)]
use crate::vulkan::VulkanSession;
#[cfg(has_ngx_sdk)]
use std::ffi::{CString, c_char};
#[cfg(has_ngx_sdk)]
use std::sync::Arc;

/// Common NGX parameter key names (from `nvsdk_ngx_defs.h`).
pub mod keys {
    /// Performance/quality value (`"PerfQualityValue"` / `NVSDK_NGX_Parameter_PerfQualityValue`).
    pub const PERF_QUALITY_VALUE: &str = "PerfQualityValue";
    /// Render width (`"Width"`).
    pub const WIDTH: &str = "Width";
    /// Render height (`"Height"`).
    pub const HEIGHT: &str = "Height";
    /// Output width (`"OutWidth"`).
    pub const OUT_WIDTH: &str = "OutWidth";
    /// Output height (`"OutHeight"`).
    pub const OUT_HEIGHT: &str = "OutHeight";
    /// DLSS feature creation flags (`"DLSS.Feature.Create.Flags"`).
    pub const DLSS_FEATURE_CREATE_FLAGS: &str = "DLSS.Feature.Create.Flags";
    /// Super-sampling availability flag (`"SuperSampling.Available"`).
    pub const SUPER_SAMPLING_AVAILABLE: &str = "SuperSampling.Available";
    /// Ray-reconstruction availability flag (`"SuperSamplingDenoising.Available"`).
    pub const RAY_RECONSTRUCTION_AVAILABLE: &str = "SuperSamplingDenoising.Available";
    /// Ray-reconstruction driver-update flag (`"SuperSamplingDenoising.NeedsUpdatedDriver"`).
    pub const RAY_RECONSTRUCTION_NEEDS_UPDATED_DRIVER: &str =
        "SuperSamplingDenoising.NeedsUpdatedDriver";
    /// Ray-reconstruction minimum driver major version
    /// (`"SuperSamplingDenoising.MinDriverVersionMajor"`).
    pub const RAY_RECONSTRUCTION_MIN_DRIVER_VERSION_MAJOR: &str =
        "SuperSamplingDenoising.MinDriverVersionMajor";
    /// Ray-reconstruction minimum driver minor version
    /// (`"SuperSamplingDenoising.MinDriverVersionMinor"`).
    pub const RAY_RECONSTRUCTION_MIN_DRIVER_VERSION_MINOR: &str =
        "SuperSamplingDenoising.MinDriverVersionMinor";
    /// Ray-reconstruction feature initialization result (`"SuperSamplingDenoising.FeatureInitResult"`).
    pub const RAY_RECONSTRUCTION_FEATURE_INIT_RESULT: &str =
        "SuperSamplingDenoising.FeatureInitResult";
    /// Ray-reconstruction optimal-settings callback pointer (`"DLSSDOptimalSettingsCallback"`).
    pub const DLSSD_OPTIMAL_SETTINGS_CALLBACK: &str = "DLSSDOptimalSettingsCallback";
    /// Ray-reconstruction denoise mode (`"DLSS.Denoise.Mode"`).
    pub const DLSS_DENOISE_MODE: &str = "DLSS.Denoise.Mode";
    /// Ray-reconstruction roughness layout mode (`"DLSS.Roughness.Mode"`).
    pub const DLSS_ROUGHNESS_MODE: &str = "DLSS.Roughness.Mode";
    /// Ray-reconstruction depth encoding flag (`"DLSS.Use.HW.Depth"`).
    pub const DLSS_USE_HW_DEPTH: &str = "DLSS.Use.HW.Depth";
    /// Diffuse albedo input resource (`"DLSS.Input.DiffuseAlbedo"`).
    pub const DLSS_INPUT_DIFFUSE_ALBEDO: &str = "DLSS.Input.DiffuseAlbedo";
    /// Specular albedo input resource (`"DLSS.Input.SpecularAlbedo"`).
    pub const DLSS_INPUT_SPECULAR_ALBEDO: &str = "DLSS.Input.SpecularAlbedo";
    /// G-buffer normals input resource (`"GBuffer.Normals"`).
    pub const GBUFFER_NORMALS: &str = "GBuffer.Normals";
    /// G-buffer roughness input resource (`"GBuffer.Roughness"`).
    pub const GBUFFER_ROUGHNESS: &str = "GBuffer.Roughness";
    /// World-to-view 4x4 matrix pointer (`"WorldToViewMatrix"`).
    pub const WORLD_TO_VIEW_MATRIX: &str = "WorldToViewMatrix";
    /// View-to-clip 4x4 matrix pointer (`"ViewToClipMatrix"`).
    pub const VIEW_TO_CLIP_MATRIX: &str = "ViewToClipMatrix";
    /// Tonemapper type (`"TonemapperType"`).
    pub const TONEMAPPER_TYPE: &str = "TonemapperType";
    /// Frame time delta in milliseconds (`"FrameTimeDeltaInMsec"`).
    pub const FRAME_TIME_DELTA_IN_MSEC: &str = "FrameTimeDeltaInMsec";
    /// Ray-tracing hit distance input resource (`"RayTracingHitDistance"`).
    pub const RAY_TRACING_HIT_DISTANCE: &str = "RayTracingHitDistance";
    /// Ray-reconstruction render preset hint for DLAA.
    pub const RR_HINT_RENDER_PRESET_DLAA: &str = "RayReconstruction.Hint.Render.Preset.DLAA";
    /// Ray-reconstruction render preset hint for Quality.
    pub const RR_HINT_RENDER_PRESET_QUALITY: &str = "RayReconstruction.Hint.Render.Preset.Quality";
    /// Ray-reconstruction render preset hint for Balanced.
    pub const RR_HINT_RENDER_PRESET_BALANCED: &str =
        "RayReconstruction.Hint.Render.Preset.Balanced";
    /// Ray-reconstruction render preset hint for Performance.
    pub const RR_HINT_RENDER_PRESET_PERFORMANCE: &str =
        "RayReconstruction.Hint.Render.Preset.Performance";
    /// Ray-reconstruction render preset hint for Ultra Performance.
    pub const RR_HINT_RENDER_PRESET_ULTRA_PERFORMANCE: &str =
        "RayReconstruction.Hint.Render.Preset.UltraPerformance";
    /// Ray-reconstruction render preset hint for Ultra Quality.
    pub const RR_HINT_RENDER_PRESET_ULTRA_QUALITY: &str =
        "RayReconstruction.Hint.Render.Preset.UltraQuality";
    /// Sharpness (`"Sharpness"`).
    pub const SHARPNESS: &str = "Sharpness";
    /// Legacy RTX value flag (`"RTXValue"`).
    pub const RTX_VALUE: &str = "RTXValue";
    /// DLSS optimal-settings callback pointer (`"DLSSOptimalSettingsCallback"`).
    pub const DLSS_OPTIMAL_SETTINGS_CALLBACK: &str = "DLSSOptimalSettingsCallback";
    /// Dynamic DLSS max render width (`"DLSS.Get.Dynamic.Max.Render.Width"`).
    pub const DLSS_DYNAMIC_MAX_RENDER_WIDTH: &str = "DLSS.Get.Dynamic.Max.Render.Width";
    /// Dynamic DLSS max render height (`"DLSS.Get.Dynamic.Max.Render.Height"`).
    pub const DLSS_DYNAMIC_MAX_RENDER_HEIGHT: &str = "DLSS.Get.Dynamic.Max.Render.Height";
    /// Dynamic DLSS min render width (`"DLSS.Get.Dynamic.Min.Render.Width"`).
    pub const DLSS_DYNAMIC_MIN_RENDER_WIDTH: &str = "DLSS.Get.Dynamic.Min.Render.Width";
    /// Dynamic DLSS min render height (`"DLSS.Get.Dynamic.Min.Render.Height"`).
    pub const DLSS_DYNAMIC_MIN_RENDER_HEIGHT: &str = "DLSS.Get.Dynamic.Min.Render.Height";
    /// Creation GPU node mask (`"CreationNodeMask"`).
    pub const CREATION_NODE_MASK: &str = "CreationNodeMask";
    /// Visibility GPU node mask (`"VisibilityNodeMask"`).
    pub const VISIBILITY_NODE_MASK: &str = "VisibilityNodeMask";
    /// Enable output subrects (`"DLSS.Enable.Output.Subrects"`).
    pub const DLSS_ENABLE_OUTPUT_SUBRECTS: &str = "DLSS.Enable.Output.Subrects";
    /// Color input resource (`"Color"`).
    pub const COLOR: &str = "Color";
    /// Output resource (`"Output"`).
    pub const OUTPUT: &str = "Output";
    /// Depth input resource (`"Depth"`).
    pub const DEPTH: &str = "Depth";
    /// Motion vectors input resource (`"MotionVectors"`).
    pub const MOTION_VECTORS: &str = "MotionVectors";
    /// Jitter offset X (`"Jitter.Offset.X"`).
    pub const JITTER_OFFSET_X: &str = "Jitter.Offset.X";
    /// Jitter offset Y (`"Jitter.Offset.Y"`).
    pub const JITTER_OFFSET_Y: &str = "Jitter.Offset.Y";
    /// Temporal history reset flag (`"Reset"`).
    pub const RESET: &str = "Reset";
    /// Motion vector X scale (`"MV.Scale.X"`).
    pub const MV_SCALE_X: &str = "MV.Scale.X";
    /// Motion vector Y scale (`"MV.Scale.Y"`).
    pub const MV_SCALE_Y: &str = "MV.Scale.Y";
    /// Render subrect width (`"DLSS.Render.Subrect.Dimensions.Width"`).
    pub const DLSS_RENDER_SUBRECT_WIDTH: &str = "DLSS.Render.Subrect.Dimensions.Width";
    /// Render subrect height (`"DLSS.Render.Subrect.Dimensions.Height"`).
    pub const DLSS_RENDER_SUBRECT_HEIGHT: &str = "DLSS.Render.Subrect.Dimensions.Height";
    /// Pre-exposure scalar (`"DLSS.Pre.Exposure"`).
    pub const DLSS_PRE_EXPOSURE: &str = "DLSS.Pre.Exposure";
    /// Exposure scale (`"DLSS.Exposure.Scale"`).
    pub const DLSS_EXPOSURE_SCALE: &str = "DLSS.Exposure.Scale";
}

/// RAII wrapper around an `NVSDK_NGX_Parameter` bag.
///
/// Destroyed automatically on [`Drop`] via `NVSDK_NGX_VULKAN_DestroyParameters`.
pub struct Parameters {
    #[cfg(has_ngx_sdk)]
    pub(crate) ptr: *mut ngx_sys::NVSDK_NGX_Parameter,
    #[cfg(has_ngx_sdk)]
    pub(crate) _session: Arc<VulkanSession>,
}

#[cfg(has_ngx_sdk)]
impl Drop for Parameters {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            let ptr = self.ptr;
            crate::ngx_call(|| {
                // SAFETY: `ptr` was allocated by `NVSDK_NGX_VULKAN_AllocateParameters` or
                // `GetCapabilityParameters` and `_session` keeps the NGX Vulkan device alive.
                let rc = unsafe { ngx_sys::NVSDK_NGX_VULKAN_DestroyParameters(ptr) };
                crate::NgxGlobal::record_teardown("NVSDK_NGX_VULKAN_DestroyParameters", rc);
            });
            self.ptr = std::ptr::null_mut();
        }
    }
}

#[cfg(has_ngx_sdk)]
impl Parameters {
    pub(crate) fn from_raw(
        ptr: *mut ngx_sys::NVSDK_NGX_Parameter,
        session: Arc<VulkanSession>,
    ) -> Result<Self> {
        if ptr.is_null() {
            return Err(Error::InvalidArgument(
                "NGX returned null NVSDK_NGX_Parameter",
            ));
        }
        Ok(Self {
            ptr,
            _session: session,
        })
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut ngx_sys::NVSDK_NGX_Parameter {
        self.ptr
    }

    pub(crate) fn as_ptr(&self) -> *const ngx_sys::NVSDK_NGX_Parameter {
        self.ptr
    }
}

impl Parameters {
    /// Sets an unsigned 64-bit integer parameter (`NVSDK_NGX_Parameter_SetULL`).
    pub fn set_u64(&mut self, name: &str, value: u64) -> Result<()> {
        #[cfg(has_ngx_sdk)]
        {
            with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is a valid `NVSDK_NGX_Parameter` and `cname` is NUL-terminated.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_SetULL(self.ptr, cname, value) };
            })
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (name, value);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Sets an unsigned 32-bit integer parameter (`NVSDK_NGX_Parameter_SetUI`).
    pub fn set_u32(&mut self, name: &str, value: u32) -> Result<()> {
        #[cfg(has_ngx_sdk)]
        {
            with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is a valid `NVSDK_NGX_Parameter` and `cname` is NUL-terminated.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_SetUI(self.ptr, cname, value) };
            })
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (name, value);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Sets a signed 32-bit integer parameter (`NVSDK_NGX_Parameter_SetI`).
    pub fn set_i32(&mut self, name: &str, value: i32) -> Result<()> {
        #[cfg(has_ngx_sdk)]
        {
            with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is a valid `NVSDK_NGX_Parameter` and `cname` is NUL-terminated.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_SetI(self.ptr, cname, value) };
            })
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (name, value);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Sets a 32-bit float parameter (`NVSDK_NGX_Parameter_SetF`).
    pub fn set_f32(&mut self, name: &str, value: f32) -> Result<()> {
        #[cfg(has_ngx_sdk)]
        {
            with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is a valid `NVSDK_NGX_Parameter` and `cname` is NUL-terminated.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_SetF(self.ptr, cname, value) };
            })
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (name, value);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Sets a 64-bit float parameter (`NVSDK_NGX_Parameter_SetD`).
    pub fn set_f64(&mut self, name: &str, value: f64) -> Result<()> {
        #[cfg(has_ngx_sdk)]
        {
            with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is a valid `NVSDK_NGX_Parameter` and `cname` is NUL-terminated.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_SetD(self.ptr, cname, value) };
            })
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (name, value);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Sets a raw pointer parameter (`NVSDK_NGX_Parameter_SetVoidPointer`).
    ///
    /// # Safety
    ///
    /// `value` must point to a valid object of the type expected by NGX for `name` (for example,
    /// `*mut NVSDK_NGX_Resource_VK`) and remain valid for as long as NGX reads it.
    pub unsafe fn set_void_ptr(&mut self, name: &str, value: *mut c_void) -> Result<()> {
        #[cfg(has_ngx_sdk)]
        {
            with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and caller guarantees `value`.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_SetVoidPointer(self.ptr, cname, value) };
            })
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = (name, value);
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Reads an unsigned 64-bit integer parameter (`NVSDK_NGX_Parameter_GetULL`).
    pub fn get_u64(&self, name: &str) -> Result<u64> {
        #[cfg(has_ngx_sdk)]
        {
            let mut out = 0u64;
            let rc = with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and `out` is writable.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_GetULL(self.ptr, cname, &mut out) }
            })?;
            check_sys("NVSDK_NGX_Parameter_GetULL", rc)?;
            Ok(out)
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = name;
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Reads an unsigned 32-bit integer parameter (`NVSDK_NGX_Parameter_GetUI`).
    pub fn get_u32(&self, name: &str) -> Result<u32> {
        #[cfg(has_ngx_sdk)]
        {
            let mut out = 0u32;
            let rc = with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and `out` is writable.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_GetUI(self.ptr, cname, &mut out) }
            })?;
            check_sys("NVSDK_NGX_Parameter_GetUI", rc)?;
            Ok(out)
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = name;
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Reads a signed 32-bit integer parameter (`NVSDK_NGX_Parameter_GetI`).
    pub fn get_i32(&self, name: &str) -> Result<i32> {
        #[cfg(has_ngx_sdk)]
        {
            let mut out = 0i32;
            let rc = with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and `out` is writable.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_GetI(self.ptr, cname, &mut out) }
            })?;
            check_sys("NVSDK_NGX_Parameter_GetI", rc)?;
            Ok(out)
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = name;
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Reads a 32-bit float parameter (`NVSDK_NGX_Parameter_GetF`).
    pub fn get_f32(&self, name: &str) -> Result<f32> {
        #[cfg(has_ngx_sdk)]
        {
            let mut out = 0.0f32;
            let rc = with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and `out` is writable.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_GetF(self.ptr, cname, &mut out) }
            })?;
            check_sys("NVSDK_NGX_Parameter_GetF", rc)?;
            Ok(out)
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = name;
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Reads a 64-bit float parameter (`NVSDK_NGX_Parameter_GetD`).
    pub fn get_f64(&self, name: &str) -> Result<f64> {
        #[cfg(has_ngx_sdk)]
        {
            let mut out = 0.0f64;
            let rc = with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and `out` is writable.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_GetD(self.ptr, cname, &mut out) }
            })?;
            check_sys("NVSDK_NGX_Parameter_GetD", rc)?;
            Ok(out)
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = name;
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Reads a raw pointer parameter (`NVSDK_NGX_Parameter_GetVoidPointer`).
    pub fn get_void_ptr(&self, name: &str) -> Result<*mut c_void> {
        #[cfg(has_ngx_sdk)]
        {
            let mut out: *mut c_void = std::ptr::null_mut();
            let rc = with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and `out` is writable.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_GetVoidPointer(self.ptr, cname, &mut out) }
            })?;
            check_sys("NVSDK_NGX_Parameter_GetVoidPointer", rc)?;
            Ok(out)
        }
        #[cfg(not(has_ngx_sdk))]
        {
            let _ = name;
            Err(Error::Unsupported(
                "NGX SDK is not available for this target".into(),
            ))
        }
    }

    /// Sets the DLSS performance/quality preset (`PerfQualityValue`).
    pub fn set_perf_quality(&mut self, quality: DlssPerfQuality) -> Result<()> {
        self.set_i32(keys::PERF_QUALITY_VALUE, quality.to_raw())
    }

    /// Reads the DLSS performance/quality preset (`PerfQualityValue`).
    pub fn get_perf_quality(&self) -> Result<Option<DlssPerfQuality>> {
        Ok(DlssPerfQuality::from_raw(
            self.get_i32(keys::PERF_QUALITY_VALUE)?,
        ))
    }

    /// Sets render input dimensions (`Width`, `Height`).
    pub fn set_render_dimensions(&mut self, width: u32, height: u32) -> Result<()> {
        self.set_u32(keys::WIDTH, width)?;
        self.set_u32(keys::HEIGHT, height)
    }

    /// Sets upscaled output dimensions (`OutWidth`, `OutHeight`).
    pub fn set_output_dimensions(&mut self, width: u32, height: u32) -> Result<()> {
        self.set_u32(keys::OUT_WIDTH, width)?;
        self.set_u32(keys::OUT_HEIGHT, height)
    }

    /// Sets DLSS feature creation flags (`DLSS.Feature.Create.Flags`).
    pub fn set_dlss_feature_flags(&mut self, flags: DlssFeatureFlags) -> Result<()> {
        self.set_i32(keys::DLSS_FEATURE_CREATE_FLAGS, flags.0)
    }

    /// Sets the DLSS Ray Reconstruction denoise mode (`DLSS.Denoise.Mode`).
    pub fn set_dlss_denoise_mode(&mut self, mode: DlssDenoiseMode) -> Result<()> {
        self.set_i32(keys::DLSS_DENOISE_MODE, mode.to_raw())
    }

    /// Reads the DLSS Ray Reconstruction denoise mode (`DLSS.Denoise.Mode`).
    pub fn get_dlss_denoise_mode(&self) -> Result<Option<DlssDenoiseMode>> {
        Ok(DlssDenoiseMode::from_raw(
            self.get_i32(keys::DLSS_DENOISE_MODE)?,
        ))
    }

    /// Sets the DLSS Ray Reconstruction roughness layout (`DLSS.Roughness.Mode`).
    pub fn set_dlss_roughness_mode(&mut self, mode: DlssRoughnessMode) -> Result<()> {
        self.set_u32(keys::DLSS_ROUGHNESS_MODE, mode.to_raw())
    }

    /// Reads the DLSS Ray Reconstruction roughness layout (`DLSS.Roughness.Mode`).
    pub fn get_dlss_roughness_mode(&self) -> Result<Option<DlssRoughnessMode>> {
        Ok(DlssRoughnessMode::from_raw(
            self.get_u32(keys::DLSS_ROUGHNESS_MODE)?,
        ))
    }

    /// Sets the DLSS Ray Reconstruction depth encoding (`DLSS.Use.HW.Depth`).
    pub fn set_dlss_depth_type(&mut self, depth_type: DlssDepthType) -> Result<()> {
        self.set_u32(keys::DLSS_USE_HW_DEPTH, depth_type.to_raw())
    }

    /// Reads the DLSS Ray Reconstruction depth encoding (`DLSS.Use.HW.Depth`).
    pub fn get_dlss_depth_type(&self) -> Result<Option<DlssDepthType>> {
        Ok(DlssDepthType::from_raw(
            self.get_u32(keys::DLSS_USE_HW_DEPTH)?,
        ))
    }

    /// Sets the DLSS Ray Reconstruction render preset hint for `quality`.
    pub fn set_dlss_rr_preset(
        &mut self,
        quality: DlssPerfQuality,
        preset: DlssRrPreset,
    ) -> Result<()> {
        let key = match quality {
            DlssPerfQuality::Dlaa => keys::RR_HINT_RENDER_PRESET_DLAA,
            DlssPerfQuality::MaxQuality => keys::RR_HINT_RENDER_PRESET_QUALITY,
            DlssPerfQuality::Balanced => keys::RR_HINT_RENDER_PRESET_BALANCED,
            DlssPerfQuality::MaxPerf => keys::RR_HINT_RENDER_PRESET_PERFORMANCE,
            DlssPerfQuality::UltraPerformance => keys::RR_HINT_RENDER_PRESET_ULTRA_PERFORMANCE,
            DlssPerfQuality::UltraQuality => keys::RR_HINT_RENDER_PRESET_ULTRA_QUALITY,
        };
        self.set_u32(key, preset.to_raw())
    }
}

/// Invokes `f` with a NUL-terminated C string for `name` without heap allocation for short keys.
///
/// `f` runs under the process-wide NGX lock ([`crate::ngx_call`]) because every caller passes the
/// string straight to an `NVSDK_NGX_Parameter_*` entry point.
#[cfg(has_ngx_sdk)]
pub(crate) fn with_cstr<R>(name: &str, f: impl FnOnce(*const c_char) -> R) -> Result<R> {
    let bytes = name.as_bytes();
    if bytes.contains(&0) {
        return Err(Error::InvalidArgument(
            "parameter name contains interior NUL",
        ));
    }
    if bytes.len() < 128 {
        let mut buf = [0u8; 128];
        buf[..bytes.len()].copy_from_slice(bytes);
        buf[bytes.len()] = 0;
        return Ok(crate::ngx_call(|| f(buf.as_ptr().cast::<c_char>())));
    }
    let owned = CString::new(name)
        .map_err(|_| Error::InvalidArgument("parameter name contains interior NUL"))?;
    Ok(crate::ngx_call(|| f(owned.as_ptr())))
}
