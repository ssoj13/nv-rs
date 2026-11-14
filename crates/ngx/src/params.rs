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
use crate::types::{DlssFeatureFlags, DlssPerfQuality};
use std::ffi::c_void;

#[cfg(target_os = "windows")]
use crate::error::check;
#[cfg(target_os = "windows")]
use crate::vulkan::VulkanSession;
#[cfg(target_os = "windows")]
use std::ffi::{CString, c_char};
#[cfg(target_os = "windows")]
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
    /// Ray-reconstruction availability flag (`"RayReconstruction.Available"`).
    pub const RAY_RECONSTRUCTION_AVAILABLE: &str = "RayReconstruction.Available";
}

/// RAII wrapper around an `NVSDK_NGX_Parameter` bag.
///
/// Destroyed automatically on [`Drop`] via `NVSDK_NGX_VULKAN_DestroyParameters`.
pub struct Parameters {
    #[cfg(target_os = "windows")]
    pub(crate) ptr: *mut ngx_sys::NVSDK_NGX_Parameter,
    #[cfg(target_os = "windows")]
    pub(crate) _session: Arc<VulkanSession>,
}

#[cfg(target_os = "windows")]
impl Drop for Parameters {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: `self.ptr` was allocated by `NVSDK_NGX_VULKAN_AllocateParameters` or
            // `GetCapabilityParameters` and `_session` keeps the NGX Vulkan device alive.
            let _ = unsafe { ngx_sys::NVSDK_NGX_VULKAN_DestroyParameters(self.ptr) };
            self.ptr = std::ptr::null_mut();
        }
    }
}

#[cfg(target_os = "windows")]
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
        #[cfg(target_os = "windows")]
        {
            with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is a valid `NVSDK_NGX_Parameter` and `cname` is NUL-terminated.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_SetULL(self.ptr, cname, value) };
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (name, value);
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }

    /// Sets an unsigned 32-bit integer parameter (`NVSDK_NGX_Parameter_SetUI`).
    pub fn set_u32(&mut self, name: &str, value: u32) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is a valid `NVSDK_NGX_Parameter` and `cname` is NUL-terminated.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_SetUI(self.ptr, cname, value) };
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (name, value);
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }

    /// Sets a signed 32-bit integer parameter (`NVSDK_NGX_Parameter_SetI`).
    pub fn set_i32(&mut self, name: &str, value: i32) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is a valid `NVSDK_NGX_Parameter` and `cname` is NUL-terminated.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_SetI(self.ptr, cname, value) };
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (name, value);
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }

    /// Sets a 32-bit float parameter (`NVSDK_NGX_Parameter_SetF`).
    pub fn set_f32(&mut self, name: &str, value: f32) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is a valid `NVSDK_NGX_Parameter` and `cname` is NUL-terminated.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_SetF(self.ptr, cname, value) };
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (name, value);
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }

    /// Sets a 64-bit float parameter (`NVSDK_NGX_Parameter_SetD`).
    pub fn set_f64(&mut self, name: &str, value: f64) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is a valid `NVSDK_NGX_Parameter` and `cname` is NUL-terminated.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_SetD(self.ptr, cname, value) };
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (name, value);
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
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
        #[cfg(target_os = "windows")]
        {
            with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and caller guarantees `value`.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_SetVoidPointer(self.ptr, cname, value) };
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (name, value);
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }

    /// Reads an unsigned 64-bit integer parameter (`NVSDK_NGX_Parameter_GetULL`).
    pub fn get_u64(&self, name: &str) -> Result<u64> {
        #[cfg(target_os = "windows")]
        {
            let mut out = 0u64;
            let rc = with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and `out` is writable.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_GetULL(self.ptr, cname, &mut out) }
            })?;
            check("NVSDK_NGX_Parameter_GetULL", rc)?;
            Ok(out)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = name;
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }

    /// Reads an unsigned 32-bit integer parameter (`NVSDK_NGX_Parameter_GetUI`).
    pub fn get_u32(&self, name: &str) -> Result<u32> {
        #[cfg(target_os = "windows")]
        {
            let mut out = 0u32;
            let rc = with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and `out` is writable.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_GetUI(self.ptr, cname, &mut out) }
            })?;
            check("NVSDK_NGX_Parameter_GetUI", rc)?;
            Ok(out)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = name;
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }

    /// Reads a signed 32-bit integer parameter (`NVSDK_NGX_Parameter_GetI`).
    pub fn get_i32(&self, name: &str) -> Result<i32> {
        #[cfg(target_os = "windows")]
        {
            let mut out = 0i32;
            let rc = with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and `out` is writable.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_GetI(self.ptr, cname, &mut out) }
            })?;
            check("NVSDK_NGX_Parameter_GetI", rc)?;
            Ok(out)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = name;
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }

    /// Reads a 32-bit float parameter (`NVSDK_NGX_Parameter_GetF`).
    pub fn get_f32(&self, name: &str) -> Result<f32> {
        #[cfg(target_os = "windows")]
        {
            let mut out = 0.0f32;
            let rc = with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and `out` is writable.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_GetF(self.ptr, cname, &mut out) }
            })?;
            check("NVSDK_NGX_Parameter_GetF", rc)?;
            Ok(out)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = name;
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }

    /// Reads a 64-bit float parameter (`NVSDK_NGX_Parameter_GetD`).
    pub fn get_f64(&self, name: &str) -> Result<f64> {
        #[cfg(target_os = "windows")]
        {
            let mut out = 0.0f64;
            let rc = with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and `out` is writable.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_GetD(self.ptr, cname, &mut out) }
            })?;
            check("NVSDK_NGX_Parameter_GetD", rc)?;
            Ok(out)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = name;
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
            ))
        }
    }

    /// Reads a raw pointer parameter (`NVSDK_NGX_Parameter_GetVoidPointer`).
    pub fn get_void_ptr(&self, name: &str) -> Result<*mut c_void> {
        #[cfg(target_os = "windows")]
        {
            let mut out: *mut c_void = std::ptr::null_mut();
            let rc = with_cstr(name, |cname| {
                // SAFETY: `self.ptr` is valid, `cname` is NUL-terminated, and `out` is writable.
                unsafe { ngx_sys::NVSDK_NGX_Parameter_GetVoidPointer(self.ptr, cname, &mut out) }
            })?;
            check("NVSDK_NGX_Parameter_GetVoidPointer", rc)?;
            Ok(out)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = name;
            Err(Error::Unsupported(
                "NGX is only supported on Windows".into(),
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
}

/// Invokes `f` with a NUL-terminated C string for `name` without heap allocation for short keys.
#[cfg(target_os = "windows")]
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
        return Ok(f(buf.as_ptr().cast::<c_char>()));
    }
    let owned = CString::new(name)
        .map_err(|_| Error::InvalidArgument("parameter name contains interior NUL"))?;
    Ok(f(owned.as_ptr()))
}
