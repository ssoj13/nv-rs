//! NVAPI status codes and error types.
//!
//! **What.** [`Status`] is a `Copy` integer newtype wrapping `NvAPI_Status` (`i32`), and [`Error`]
//! is the crate's `thiserror` error enum covering non-Windows platforms, library load failures,
//! NVAPI status codes, and input validation.
//! **Why.** Preserves unknown status codes from newer drivers while providing ergonomic
//! classification (`is_not_supported`, `is_end_enumeration`, `is_profile_not_found`) and
//! `NvAPI_GetErrorMessage` formatting on Windows.
//! **How.** [`check`] maps any non-`NVAPI_OK` status into [`Error::Status`].
//! **Used by** [`crate::NvApi`] and all Windows query modules.

use std::fmt;

#[cfg(windows)]
use crate::sys;

/// Raw NVAPI return code (`NvAPI_Status`), preserved as a `Copy` integer newtype.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Status(pub i32);

impl Status {
    /// `NVAPI_OK` (`0`).
    pub const OK: Self = Self(0);
    /// `NVAPI_ERROR` (`-1`).
    pub const ERROR: Self = Self(-1);
    /// `NVAPI_LIBRARY_NOT_FOUND` (`-2`).
    pub const LIBRARY_NOT_FOUND: Self = Self(-2);
    /// `NVAPI_NO_IMPLEMENTATION` (`-3`).
    pub const NO_IMPLEMENTATION: Self = Self(-3);
    /// `NVAPI_INVALID_ARGUMENT` (`-5`).
    pub const INVALID_ARGUMENT: Self = Self(-5);
    /// `NVAPI_NVIDIA_DEVICE_NOT_FOUND` (`-6`).
    pub const NVIDIA_DEVICE_NOT_FOUND: Self = Self(-6);
    /// `NVAPI_END_ENUMERATION` (`-7`).
    pub const END_ENUMERATION: Self = Self(-7);
    /// `NVAPI_INCOMPATIBLE_STRUCT_VERSION` (`-9`).
    pub const INCOMPATIBLE_STRUCT_VERSION: Self = Self(-9);
    /// `NVAPI_NOT_SUPPORTED` (`-104`).
    pub const NOT_SUPPORTED: Self = Self(-104);
    /// `NVAPI_PROFILE_NOT_FOUND` (`-163`).
    pub const PROFILE_NOT_FOUND: Self = Self(-163);

    /// Returns `true` when this status represents `NVAPI_OK`.
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 == Self::OK.0
    }

    /// Returns `true` when the driver reported `NVAPI_END_ENUMERATION`.
    #[inline]
    pub const fn is_end_enumeration(self) -> bool {
        self.0 == Self::END_ENUMERATION.0
    }

    /// Returns `true` when the driver reported `NVAPI_NOT_SUPPORTED` or `NVAPI_NO_IMPLEMENTATION`.
    #[inline]
    pub const fn is_not_supported(self) -> bool {
        self.0 == Self::NOT_SUPPORTED.0 || self.0 == Self::NO_IMPLEMENTATION.0
    }

    /// Returns `true` when the driver reported `NVAPI_NVIDIA_DEVICE_NOT_FOUND`.
    #[inline]
    pub const fn is_device_not_found(self) -> bool {
        self.0 == Self::NVIDIA_DEVICE_NOT_FOUND.0
    }

    /// Returns `true` when the driver reported `NVAPI_PROFILE_NOT_FOUND`.
    #[inline]
    pub const fn is_profile_not_found(self) -> bool {
        self.0 == Self::PROFILE_NOT_FOUND.0
    }

    /// Returns a static human-readable name for known NVAPI status codes.
    pub const fn known_name(self) -> Option<&'static str> {
        match self.0 {
            0 => Some("NVAPI_OK"),
            -1 => Some("NVAPI_ERROR"),
            -2 => Some("NVAPI_LIBRARY_NOT_FOUND"),
            -3 => Some("NVAPI_NO_IMPLEMENTATION"),
            -5 => Some("NVAPI_INVALID_ARGUMENT"),
            -6 => Some("NVAPI_NVIDIA_DEVICE_NOT_FOUND"),
            -7 => Some("NVAPI_END_ENUMERATION"),
            -9 => Some("NVAPI_INCOMPATIBLE_STRUCT_VERSION"),
            -104 => Some("NVAPI_NOT_SUPPORTED"),
            -163 => Some("NVAPI_PROFILE_NOT_FOUND"),
            _ => None,
        }
    }

    /// Queries `NvAPI_GetErrorMessage` on Windows when `nvapi64.dll` is loaded.
    #[cfg(windows)]
    pub fn driver_message(self) -> Option<String> {
        if sys::load().is_err() {
            return None;
        }
        let mut buf: sys::NvAPI_ShortString = [0; 64];
        // SAFETY: `NvAPI_GetErrorMessage` writes a NUL-terminated ASCII string into the 64-byte
        // `NvAPI_ShortString` buffer.
        let rc = unsafe { sys::NvAPI_GetErrorMessage(self.0, buf.as_mut_ptr()) };
        if rc != Self::OK.0 {
            return None;
        }
        let s = crate::types::decode_short_string(&buf);
        (!s.is_empty()).then_some(s)
    }

    /// Always `None` on non-Windows targets where NVAPI does not exist.
    #[cfg(not(windows))]
    pub fn driver_message(self) -> Option<String> {
        None
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
            write!(f, "NVAPI error (code {})", self.0)
        }
    }
}

/// Everything an NVAPI call can fail with.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// NVAPI is Windows-only; returned cleanly on non-Windows targets.
    #[error("NVAPI is only supported on Windows")]
    UnsupportedPlatform,
    /// `nvapi64.dll` could not be loaded.
    #[error("NVAPI unavailable: {0}")]
    Unavailable(#[from] &'static nv_loader::LoadError),
    /// An NVAPI call returned a non-OK status code.
    #[error("{call} failed: {status}")]
    Status {
        /// The NVAPI C entry point that failed.
        call: &'static str,
        /// Raw NVAPI return code.
        status: Status,
    },
    /// An argument was rejected before reaching NVAPI.
    #[error("invalid argument: {0}")]
    InvalidInput(&'static str),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Maps a raw `NvAPI_Status` (`i32`) to [`Result<()>`].
#[cfg_attr(not(windows), allow(dead_code))]
#[inline]
pub(crate) fn check(call: &'static str, code: i32) -> Result<()> {
    let status = Status(code);
    if status.is_ok() {
        Ok(())
    } else {
        Err(Error::Status { call, status })
    }
}
