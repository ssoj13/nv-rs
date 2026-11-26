//! Safe wrapper over `nvrtc-sys` for runtime compilation of CUDA C++ source into PTX,
//! CUBIN, LTO-IR, and OptiX-IR.
//!
//! **What.**
//! - [`CompileOptions`] — builder for NVRTC compiler flags (`--gpu-architecture`, `--std`,
//!   macros, include paths, fast math, debug/line-info, RDC, LTO).
//! - [`Program`] — owns an `nvrtcProgram`, registers name expressions before compilation,
//!   compiles with [`Program::compile`] (capturing the build log on both success and failure),
//!   and retrieves PTX, CUBIN, LTO-IR, OptiX-IR, and mangled/lowered symbol names.
//! - [`version`] and [`supported_archs`] — query the loaded NVRTC library.
//!
//! **Why.** NVRTC is a host-side compiler library with no CUDA context requirement; wrapping it
//! cleanly lets callers compile kernels at runtime and hand the resulting PTX/CUBIN to
//! `cuda-core` or `optix`.
//! **Used by** `nvrtc` GPU tests and downstream runtime-compilation callers.

pub mod options;
pub mod program;

pub use options::{CompileOptions, MathMode};
pub use program::Program;

pub use nvrtc_sys as sys;

use std::ffi::CStr;
use std::fmt;

/// Raw `nvrtcResult` status code wrapped in a `Copy` newtype so newer toolkit codes remain valid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NvrtcStatus(pub u32);

impl NvrtcStatus {
    /// `NVRTC_SUCCESS` (`0`).
    pub const SUCCESS: Self = Self(sys::nvrtcResult_NVRTC_SUCCESS);

    /// Human-readable name from `nvrtcGetErrorString` (falls back to the numeric code if NVRTC is
    /// not loaded or returns null).
    pub fn describe(self) -> &'static str {
        if sys::load().is_err() {
            return "nvrtc unavailable";
        }
        // SAFETY: `nvrtcGetErrorString` takes a plain integer enum value and returns a pointer to
        // a static NUL-terminated string (or null for unknown codes).
        let ptr = unsafe { sys::nvrtcGetErrorString(self.0) };
        if ptr.is_null() {
            return "unknown nvrtcResult";
        }
        // SAFETY: `ptr` is non-null and points to a static NUL-terminated string from NVRTC.
        unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .unwrap_or("non-utf8 nvrtcResult")
    }
}

impl fmt::Display for NvrtcStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.describe(), self.0)
    }
}

/// Errors returned by `nvrtc`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `nvrtc` could not be loaded from the CUDA Toolkit.
    #[error("NVRTC unavailable: {0}")]
    Unavailable(String),
    /// An NVRTC API call returned a non-success status code.
    #[error("{call} failed with {code}")]
    Status {
        /// The NVRTC entry point.
        call: &'static str,
        /// The raw status code.
        code: NvrtcStatus,
    },
    /// `nvrtcCompileProgram` failed; includes the compiler build log when available.
    #[error("NVRTC compilation failed ({code}){}", log_suffix(.log))]
    Compilation {
        /// The raw status code.
        code: NvrtcStatus,
        /// The compiler diagnostic log.
        log: String,
    },
    /// An argument was rejected before reaching NVRTC.
    #[error("invalid input: {0}")]
    InvalidInput(&'static str),
}

fn log_suffix(log: &str) -> String {
    if log.is_empty() {
        String::new()
    } else {
        format!(":\n{log}")
    }
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Ensures `nvrtc` is loaded so missing-DLL failures report [`Error::Unavailable`] rather than
/// `NVRTC_ERROR_INTERNAL_ERROR`.
pub(crate) fn ensure_loaded() -> Result<()> {
    sys::load()
        .map(|_| ())
        .map_err(|e| Error::Unavailable(e.to_string()))
}

/// Maps a raw `nvrtcResult` to [`Error::Status`].
pub(crate) fn check(call: &'static str, code: sys::nvrtcResult) -> Result<()> {
    if code == sys::nvrtcResult_NVRTC_SUCCESS {
        Ok(())
    } else {
        Err(Error::Status {
            call,
            code: NvrtcStatus(code),
        })
    }
}

/// Returns the `(major, minor)` version of the loaded NVRTC library (`nvrtcVersion`).
pub fn version() -> Result<(i32, i32)> {
    ensure_loaded()?;
    let (mut major, mut minor) = (0i32, 0i32);
    // SAFETY: `major` and `minor` are valid stack out-pointers.
    check("nvrtcVersion", unsafe {
        sys::nvrtcVersion(&mut major, &mut minor)
    })?;
    Ok((major, minor))
}

/// Returns the sorted list of GPU architecture numbers (e.g. `75`, `86`, `89`, `90`) supported by
/// the loaded NVRTC library (`nvrtcGetNumSupportedArchs` + `nvrtcGetSupportedArchs`).
pub fn supported_archs() -> Result<Vec<i32>> {
    ensure_loaded()?;
    let mut count = 0i32;
    // SAFETY: `count` is a valid stack out-pointer.
    check("nvrtcGetNumSupportedArchs", unsafe {
        sys::nvrtcGetNumSupportedArchs(&mut count)
    })?;
    if count <= 0 {
        return Ok(Vec::new());
    }
    let mut archs = vec![0i32; count as usize];
    // SAFETY: `archs` has capacity `count` elements matching `nvrtcGetNumSupportedArchs`.
    check("nvrtcGetSupportedArchs", unsafe {
        sys::nvrtcGetSupportedArchs(archs.as_mut_ptr())
    })?;
    Ok(archs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_display_and_error_formatting() {
        let code = NvrtcStatus(6);
        let err = Error::Compilation {
            code,
            log: "error: expected ';'".to_owned(),
        };
        let msg = err.to_string();
        assert!(msg.contains("6"));
        assert!(msg.contains("error: expected ';'"));

        let empty_err = Error::Compilation {
            code,
            log: String::new(),
        };
        assert!(!empty_err.to_string().ends_with('\n'));
    }

    #[test]
    fn status_newtype_is_copy_and_eq() {
        let a = NvrtcStatus::SUCCESS;
        let b = a;
        assert_eq!(a, b);
        assert_eq!(a.0, 0);
    }
}
