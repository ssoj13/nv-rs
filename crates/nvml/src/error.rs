//! NVML status codes and error types.
//!
//! **What.** [`Status`] is a `Copy` integer newtype wrapping `nvmlReturn_t`, and [`Error`] is the
//! crate's `thiserror` error enum covering library load failures, NVML status codes, and input
//! validation.
//! **Why.** Wrapping `nvmlReturn_t` in a newtype preserves unknown status codes from newer drivers
//! while providing ergonomic classification and `nvmlErrorString` formatting.
//! **How.** [`check`] turns any non-`NVML_SUCCESS` return code into [`Error::Status`].
//! **Used by** [`crate::Nvml`] and [`crate::Device`].

use std::ffi::CStr;
use std::fmt;

use crate::sys;

/// Raw NVML return code (`nvmlReturn_t`), preserved as a `Copy` integer newtype so status codes
/// introduced by newer drivers remain representable.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Status(pub sys::nvmlReturn_t);

impl Status {
    /// `NVML_SUCCESS` (`0`).
    pub const SUCCESS: Self = Self(sys::nvmlReturn_enum_NVML_SUCCESS);
    /// `NVML_ERROR_UNINITIALIZED` (`1`).
    pub const UNINITIALIZED: Self = Self(sys::nvmlReturn_enum_NVML_ERROR_UNINITIALIZED);
    /// `NVML_ERROR_INVALID_ARGUMENT` (`2`).
    pub const INVALID_ARGUMENT: Self = Self(sys::nvmlReturn_enum_NVML_ERROR_INVALID_ARGUMENT);
    /// `NVML_ERROR_NOT_SUPPORTED` (`3`).
    pub const NOT_SUPPORTED: Self = Self(sys::nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED);
    /// `NVML_ERROR_NO_PERMISSION` (`4`).
    pub const NO_PERMISSION: Self = Self(sys::nvmlReturn_enum_NVML_ERROR_NO_PERMISSION);
    /// `NVML_ERROR_NOT_FOUND` (`6`).
    pub const NOT_FOUND: Self = Self(sys::nvmlReturn_enum_NVML_ERROR_NOT_FOUND);
    /// `NVML_ERROR_INSUFFICIENT_SIZE` (`7`).
    pub const INSUFFICIENT_SIZE: Self = Self(sys::nvmlReturn_enum_NVML_ERROR_INSUFFICIENT_SIZE);
    /// `NVML_ERROR_DRIVER_NOT_LOADED` (`9`).
    pub const DRIVER_NOT_LOADED: Self = Self(sys::nvmlReturn_enum_NVML_ERROR_DRIVER_NOT_LOADED);
    /// `NVML_ERROR_LIBRARY_NOT_FOUND` (`12`).
    pub const LIBRARY_NOT_FOUND: Self = Self(sys::nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND);
    /// `NVML_ERROR_FUNCTION_NOT_FOUND` (`13`).
    pub const FUNCTION_NOT_FOUND: Self = Self(sys::nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND);
    /// `NVML_ERROR_NO_DATA` (`21`).
    pub const NO_DATA: Self = Self(sys::nvmlReturn_enum_NVML_ERROR_NO_DATA);

    /// Returns `true` when this status represents `NVML_SUCCESS`.
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 == Self::SUCCESS.0
    }

    /// Returns `true` when the driver reported `NVML_ERROR_NOT_SUPPORTED`.
    #[inline]
    pub const fn is_not_supported(self) -> bool {
        self.0 == Self::NOT_SUPPORTED.0
    }

    /// Returns `true` when the driver reported `NVML_ERROR_NOT_FOUND`.
    #[inline]
    pub const fn is_not_found(self) -> bool {
        self.0 == Self::NOT_FOUND.0
    }

    /// Returns `true` when the driver reported `NVML_ERROR_INSUFFICIENT_SIZE`.
    #[inline]
    pub const fn is_insufficient_size(self) -> bool {
        self.0 == Self::INSUFFICIENT_SIZE.0
    }

    /// Returns `true` when the entry point is absent in the loaded NVML library.
    #[inline]
    pub const fn is_function_not_found(self) -> bool {
        self.0 == Self::FUNCTION_NOT_FOUND.0
    }

    /// Returns a static human-readable name for known NVML status codes, or `None` for newer
    /// driver codes.
    pub const fn known_name(self) -> Option<&'static str> {
        match self.0 {
            x if x == Self::SUCCESS.0 => Some("NVML_SUCCESS"),
            x if x == Self::UNINITIALIZED.0 => Some("NVML_ERROR_UNINITIALIZED"),
            x if x == Self::INVALID_ARGUMENT.0 => Some("NVML_ERROR_INVALID_ARGUMENT"),
            x if x == Self::NOT_SUPPORTED.0 => Some("NVML_ERROR_NOT_SUPPORTED"),
            x if x == Self::NO_PERMISSION.0 => Some("NVML_ERROR_NO_PERMISSION"),
            x if x == Self::NOT_FOUND.0 => Some("NVML_ERROR_NOT_FOUND"),
            x if x == Self::INSUFFICIENT_SIZE.0 => Some("NVML_ERROR_INSUFFICIENT_SIZE"),
            x if x == Self::DRIVER_NOT_LOADED.0 => Some("NVML_ERROR_DRIVER_NOT_LOADED"),
            x if x == Self::LIBRARY_NOT_FOUND.0 => Some("NVML_ERROR_LIBRARY_NOT_FOUND"),
            x if x == Self::FUNCTION_NOT_FOUND.0 => Some("NVML_ERROR_FUNCTION_NOT_FOUND"),
            x if x == Self::NO_DATA.0 => Some("NVML_ERROR_NO_DATA"),
            _ => None,
        }
    }

    /// Queries `nvmlErrorString` if `nvml` is loaded; returns `None` when the library is absent or
    /// the driver returns a null pointer.
    pub fn driver_message(self) -> Option<&'static str> {
        if sys::load().is_err() {
            return None;
        }
        // SAFETY: `nvmlErrorString` is a pure status-to-static-string lookup in NVML; when
        // non-null, NVML returns a NUL-terminated static string.
        let ptr = unsafe { sys::nvmlErrorString(self.0) };
        if ptr.is_null() {
            return None;
        }
        // SAFETY: `ptr` was just checked non-null and points to NVML's static NUL-terminated table.
        unsafe { CStr::from_ptr(ptr) }.to_str().ok()
    }
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.known_name() {
            Some(name) => write!(f, "Status({name} = {})", self.0),
            None => write!(f, "Status({})", self.0),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(msg) = self.driver_message() {
            write!(f, "{msg} (code {})", self.0)
        } else if let Some(name) = self.known_name() {
            write!(f, "{name} (code {})", self.0)
        } else {
            write!(f, "NVML error (code {})", self.0)
        }
    }
}

/// Everything an NVML call can fail with.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `nvml.dll` / `libnvidia-ml.so.1` could not be loaded.
    #[error("NVML unavailable: {0}")]
    Unavailable(#[from] &'static nv_loader::LoadError),
    /// An NVML call returned a non-success status code.
    #[error("{call} failed: {status}")]
    Status {
        /// The NVML C entry point that failed.
        call: &'static str,
        /// Raw NVML return code.
        status: Status,
    },
    /// An argument was rejected before reaching NVML.
    #[error("invalid argument: {0}")]
    InvalidInput(&'static str),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Maps an `nvmlReturn_t` to [`Result<()>`].
#[inline]
pub(crate) fn check(call: &'static str, code: sys::nvmlReturn_t) -> Result<()> {
    let status = Status(code);
    if status.is_ok() {
        Ok(())
    } else {
        Err(Error::Status { call, status })
    }
}
