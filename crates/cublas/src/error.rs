//! Error types and status-code wrappers for `cublas` and `cublasLt`.
//!
//! **What.** Defines [`Status`] (a `Copy` integer newtype over `cublasStatus_t` so newer toolkit
//! codes remain valid), [`Error`] (`thiserror`), [`Result`], and internal status-checking /
//! context-teardown helpers.
//! **Why.** Preserves raw toolkit codes without closed enums and records `Drop` failures on the
//! owning `cuda_core::Context` instead of panicking.
//! **Used by** every module in `cublas`.

use crate::{lt_sys, sys};
use cuda_core::{Context, DriverError};
use std::ffi::CStr;
use std::fmt;

/// Raw `cublasStatus_t` status code.
///
/// Kept as a transparent integer newtype so status codes added in newer CUDA toolkits round-trip
/// without breaking exhaustive matches.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Status(pub sys::cublasStatus_t);

impl Status {
    /// Operation completed successfully (`CUBLAS_STATUS_SUCCESS`).
    pub const SUCCESS: Self = Self(sys::cublasStatus_t_CUBLAS_STATUS_SUCCESS);
    /// Library was not initialized (`CUBLAS_STATUS_NOT_INITIALIZED`).
    pub const NOT_INITIALIZED: Self = Self(sys::cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED);
    /// Resource allocation failed (`CUBLAS_STATUS_ALLOC_FAILED`).
    pub const ALLOC_FAILED: Self = Self(sys::cublasStatus_t_CUBLAS_STATUS_ALLOC_FAILED);
    /// Unsupported value or parameter (`CUBLAS_STATUS_INVALID_VALUE`).
    pub const INVALID_VALUE: Self = Self(sys::cublasStatus_t_CUBLAS_STATUS_INVALID_VALUE);
    /// Device architecture does not support the operation (`CUBLAS_STATUS_ARCH_MISMATCH`).
    pub const ARCH_MISMATCH: Self = Self(sys::cublasStatus_t_CUBLAS_STATUS_ARCH_MISMATCH);
    /// Access to GPU memory space failed (`CUBLAS_STATUS_MAPPING_ERROR`).
    pub const MAPPING_ERROR: Self = Self(sys::cublasStatus_t_CUBLAS_STATUS_MAPPING_ERROR);
    /// GPU program failed to execute (`CUBLAS_STATUS_EXECUTION_FAILED`).
    pub const EXECUTION_FAILED: Self = Self(sys::cublasStatus_t_CUBLAS_STATUS_EXECUTION_FAILED);
    /// Internal cuBLAS operation failed (`CUBLAS_STATUS_INTERNAL_ERROR`).
    pub const INTERNAL_ERROR: Self = Self(sys::cublasStatus_t_CUBLAS_STATUS_INTERNAL_ERROR);
    /// Functionality is not supported (`CUBLAS_STATUS_NOT_SUPPORTED`).
    pub const NOT_SUPPORTED: Self = Self(sys::cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED);
    /// Licensing error (`CUBLAS_STATUS_LICENSE_ERROR`).
    pub const LICENSE_ERROR: Self = Self(sys::cublasStatus_t_CUBLAS_STATUS_LICENSE_ERROR);

    /// Returns true when `self == Self::SUCCESS`.
    pub const fn is_success(self) -> bool {
        self.0 == Self::SUCCESS.0
    }

    /// Static fallback name for well-known cuBLAS status codes.
    pub const fn static_name(self) -> &'static str {
        match self.0 {
            sys::cublasStatus_t_CUBLAS_STATUS_SUCCESS => "CUBLAS_STATUS_SUCCESS",
            sys::cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED => "CUBLAS_STATUS_NOT_INITIALIZED",
            sys::cublasStatus_t_CUBLAS_STATUS_ALLOC_FAILED => "CUBLAS_STATUS_ALLOC_FAILED",
            sys::cublasStatus_t_CUBLAS_STATUS_INVALID_VALUE => "CUBLAS_STATUS_INVALID_VALUE",
            sys::cublasStatus_t_CUBLAS_STATUS_ARCH_MISMATCH => "CUBLAS_STATUS_ARCH_MISMATCH",
            sys::cublasStatus_t_CUBLAS_STATUS_MAPPING_ERROR => "CUBLAS_STATUS_MAPPING_ERROR",
            sys::cublasStatus_t_CUBLAS_STATUS_EXECUTION_FAILED => "CUBLAS_STATUS_EXECUTION_FAILED",
            sys::cublasStatus_t_CUBLAS_STATUS_INTERNAL_ERROR => "CUBLAS_STATUS_INTERNAL_ERROR",
            sys::cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED => "CUBLAS_STATUS_NOT_SUPPORTED",
            sys::cublasStatus_t_CUBLAS_STATUS_LICENSE_ERROR => "CUBLAS_STATUS_LICENSE_ERROR",
            _ => "CUBLAS_STATUS_UNKNOWN",
        }
    }

    /// Queries `cublasGetStatusName` if the library is loaded, falling back to [`Self::static_name`].
    pub fn name(self) -> &'static str {
        if sys::load().is_ok() {
            // SAFETY: `cublasGetStatusName` returns a static NUL-terminated string or null.
            let ptr = unsafe { sys::cublasGetStatusName(self.0) };
            if !ptr.is_null() {
                // SAFETY: `ptr` is non-null and points to a static C string inside cublas.
                if let Ok(s) = unsafe { CStr::from_ptr(ptr) }.to_str() {
                    return s;
                }
            }
        }
        self.static_name()
    }

    /// Queries `cublasLtGetStatusName` if `cublasLt` is loaded, falling back to [`Self::static_name`].
    pub fn lt_name(self) -> &'static str {
        if lt_sys::load().is_ok() {
            // SAFETY: `cublasLtGetStatusName` returns a static NUL-terminated string or null.
            let ptr = unsafe { lt_sys::cublasLtGetStatusName(self.0 as lt_sys::cublasStatus_t) };
            if !ptr.is_null() {
                // SAFETY: `ptr` is non-null and points to a static C string inside cublasLt.
                if let Ok(s) = unsafe { CStr::from_ptr(ptr) }.to_str() {
                    return s;
                }
            }
        }
        self.static_name()
    }

    /// Queries `cublasLtGetStatusString` if `cublasLt` is loaded, returning `None` if unavailable.
    pub fn lt_string(self) -> Option<&'static str> {
        if lt_sys::load().is_ok() {
            // SAFETY: `cublasLtGetStatusString` returns a static NUL-terminated string or null.
            let ptr = unsafe { lt_sys::cublasLtGetStatusString(self.0 as lt_sys::cublasStatus_t) };
            if !ptr.is_null() {
                // SAFETY: `ptr` is non-null and points to a static C string inside cublasLt.
                return unsafe { CStr::from_ptr(ptr) }.to_str().ok();
            }
        }
        None
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name(), self.0)
    }
}

/// Everything a cuBLAS or cuBLASLt call can fail with.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `cublas` or `cublasLt` shared library could not be loaded or is missing a required symbol.
    #[error("cuBLAS unavailable: {0}")]
    Unavailable(String),
    /// A cuBLAS or cuBLASLt call returned a non-success status code.
    #[error("{call} failed with {code}")]
    Cublas {
        /// The cuBLAS / cuBLASLt entry point.
        call: &'static str,
        /// The raw `cublasStatus_t`.
        code: Status,
    },
    /// A CUDA driver call failed.
    #[error("CUDA: {0}")]
    Driver(#[from] DriverError),
    /// Workspace or operation needs more device bytes than the caller's budget admits.
    #[error("needs {requested} device bytes, budget admits {available}")]
    Budget {
        /// Bytes requested.
        requested: usize,
        /// Caller's byte budget.
        available: usize,
    },
    /// An argument was rejected before reaching cuBLAS.
    #[error("invalid argument: {0}")]
    Invalid(&'static str),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Maps a `cublas_sys::cublasStatus_t` to [`Result<()>`].
#[inline]
pub(crate) fn check(call: &'static str, code: sys::cublasStatus_t) -> Result<()> {
    if code == sys::cublasStatus_t_CUBLAS_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(Error::Cublas {
            call,
            code: Status(code),
        })
    }
}

/// Maps a `cublaslt_sys::cublasStatus_t` to [`Result<()>`].
#[inline]
pub(crate) fn check_lt(call: &'static str, code: lt_sys::cublasStatus_t) -> Result<()> {
    check(call, code as sys::cublasStatus_t)
}

/// Records a failed teardown call on the owning CUDA context (drop paths cannot return errors).
#[inline]
pub(crate) fn teardown(ctx: &Context, code: sys::cublasStatus_t) {
    if code != sys::cublasStatus_t_CUBLAS_STATUS_SUCCESS {
        ctx.record_err(Err::<(), _>(DriverError(
            cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
        )));
    }
}

/// Records a failed `cublasLt` teardown call on the owning CUDA context.
#[inline]
pub(crate) fn teardown_lt(ctx: &Context, code: lt_sys::cublasStatus_t) {
    teardown(ctx, code as sys::cublasStatus_t);
}
