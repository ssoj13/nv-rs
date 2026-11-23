//! Error types and status-code wrappers for `curand`.
//!
//! **What.** Defines [`CurandStatus`] (a `Copy` integer newtype over `curandStatus_t`), [`Error`]
//! (`thiserror`), [`Result`], and internal status-checking / teardown helpers.
//! **Why.** Preserves raw cuRAND status codes without closed enums, surfaces `cuda-bindings`
//! dynamic-load failures clearly, and keeps `Drop` failures with their real status in the
//! `cuda_core_ext::teardown` sink (a failed context bind stays on the owning
//! `cuda_core::Context`).
//! **Used by** every module in `curand`.

use cuda_core::DriverError;
use std::fmt;

/// Raw `curandStatus_t` status code.
///
/// Kept as a transparent integer newtype so status codes added in newer CUDA toolkits round-trip
/// without breaking exhaustive matches.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CurandStatus(pub cuda_bindings::curandStatus_t);

impl CurandStatus {
    /// No errors (`CURAND_STATUS_SUCCESS`).
    pub const SUCCESS: Self = Self(cuda_bindings::curandStatus_CURAND_STATUS_SUCCESS);
    /// Header file and linked library version mismatch (`CURAND_STATUS_VERSION_MISMATCH`).
    pub const VERSION_MISMATCH: Self =
        Self(cuda_bindings::curandStatus_CURAND_STATUS_VERSION_MISMATCH);
    /// Generator not initialized (`CURAND_STATUS_NOT_INITIALIZED`).
    pub const NOT_INITIALIZED: Self =
        Self(cuda_bindings::curandStatus_CURAND_STATUS_NOT_INITIALIZED);
    /// Memory allocation failed (`CURAND_STATUS_ALLOCATION_FAILED`).
    pub const ALLOCATION_FAILED: Self =
        Self(cuda_bindings::curandStatus_CURAND_STATUS_ALLOCATION_FAILED);
    /// Generator is wrong type (`CURAND_STATUS_TYPE_ERROR`).
    pub const TYPE_ERROR: Self = Self(cuda_bindings::curandStatus_CURAND_STATUS_TYPE_ERROR);
    /// Argument out of range (`CURAND_STATUS_OUT_OF_RANGE`).
    pub const OUT_OF_RANGE: Self = Self(cuda_bindings::curandStatus_CURAND_STATUS_OUT_OF_RANGE);
    /// Length requested is not a multiple of dimension (`CURAND_STATUS_LENGTH_NOT_MULTIPLE`).
    pub const LENGTH_NOT_MULTIPLE: Self =
        Self(cuda_bindings::curandStatus_CURAND_STATUS_LENGTH_NOT_MULTIPLE);
    /// Double precision required (`CURAND_STATUS_DOUBLE_PRECISION_REQUIRED`).
    pub const DOUBLE_PRECISION_REQUIRED: Self =
        Self(cuda_bindings::curandStatus_CURAND_STATUS_DOUBLE_PRECISION_REQUIRED);
    /// Kernel launch failure (`CURAND_STATUS_LAUNCH_FAILURE`).
    pub const LAUNCH_FAILURE: Self = Self(cuda_bindings::curandStatus_CURAND_STATUS_LAUNCH_FAILURE);
    /// Preexisting failure on library entry (`CURAND_STATUS_PREEXISTING_FAILURE`).
    pub const PREEXISTING_FAILURE: Self =
        Self(cuda_bindings::curandStatus_CURAND_STATUS_PREEXISTING_FAILURE);
    /// Initialization of CUDA failed (`CURAND_STATUS_INITIALIZATION_FAILED`).
    pub const INITIALIZATION_FAILED: Self =
        Self(cuda_bindings::curandStatus_CURAND_STATUS_INITIALIZATION_FAILED);
    /// Architecture mismatch (`CURAND_STATUS_ARCH_MISMATCH`).
    pub const ARCH_MISMATCH: Self = Self(cuda_bindings::curandStatus_CURAND_STATUS_ARCH_MISMATCH);
    /// Internal library error (`CURAND_STATUS_INTERNAL_ERROR`).
    pub const INTERNAL_ERROR: Self = Self(cuda_bindings::curandStatus_CURAND_STATUS_INTERNAL_ERROR);

    /// Returns true when `self == Self::SUCCESS`.
    pub const fn is_success(self) -> bool {
        self.0 == Self::SUCCESS.0
    }

    /// Human-readable name for known cuRAND status codes.
    pub const fn name(self) -> &'static str {
        match self.0 {
            cuda_bindings::curandStatus_CURAND_STATUS_SUCCESS => "CURAND_STATUS_SUCCESS",
            cuda_bindings::curandStatus_CURAND_STATUS_VERSION_MISMATCH => {
                "CURAND_STATUS_VERSION_MISMATCH"
            }
            cuda_bindings::curandStatus_CURAND_STATUS_NOT_INITIALIZED => {
                "CURAND_STATUS_NOT_INITIALIZED"
            }
            cuda_bindings::curandStatus_CURAND_STATUS_ALLOCATION_FAILED => {
                "CURAND_STATUS_ALLOCATION_FAILED"
            }
            cuda_bindings::curandStatus_CURAND_STATUS_TYPE_ERROR => "CURAND_STATUS_TYPE_ERROR",
            cuda_bindings::curandStatus_CURAND_STATUS_OUT_OF_RANGE => "CURAND_STATUS_OUT_OF_RANGE",
            cuda_bindings::curandStatus_CURAND_STATUS_LENGTH_NOT_MULTIPLE => {
                "CURAND_STATUS_LENGTH_NOT_MULTIPLE"
            }
            cuda_bindings::curandStatus_CURAND_STATUS_DOUBLE_PRECISION_REQUIRED => {
                "CURAND_STATUS_DOUBLE_PRECISION_REQUIRED"
            }
            cuda_bindings::curandStatus_CURAND_STATUS_LAUNCH_FAILURE => {
                "CURAND_STATUS_LAUNCH_FAILURE"
            }
            cuda_bindings::curandStatus_CURAND_STATUS_PREEXISTING_FAILURE => {
                "CURAND_STATUS_PREEXISTING_FAILURE"
            }
            cuda_bindings::curandStatus_CURAND_STATUS_INITIALIZATION_FAILED => {
                "CURAND_STATUS_INITIALIZATION_FAILED"
            }
            cuda_bindings::curandStatus_CURAND_STATUS_ARCH_MISMATCH => {
                "CURAND_STATUS_ARCH_MISMATCH"
            }
            cuda_bindings::curandStatus_CURAND_STATUS_INTERNAL_ERROR => {
                "CURAND_STATUS_INTERNAL_ERROR"
            }
            _ => "CURAND_STATUS_UNKNOWN",
        }
    }
}

impl fmt::Display for CurandStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name(), self.0)
    }
}

/// Everything a cuRAND call can fail with.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `curand` shared library could not be loaded.
    #[error("cuRAND unavailable: {0}")]
    Unavailable(String),
    /// A cuRAND call returned a non-success status code.
    #[error("{call} failed with {code}")]
    Curand {
        /// The cuRAND entry point.
        call: &'static str,
        /// The raw `curandStatus_t`.
        code: CurandStatus,
    },
    /// A CUDA driver call failed.
    #[error("CUDA: {0}")]
    Driver(#[from] DriverError),
    /// An argument was rejected before reaching cuRAND.
    #[error("invalid argument: {0}")]
    Invalid(&'static str),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Ensures `curand` shared library loaded cleanly via `cuda-bindings`.
#[inline]
pub(crate) fn ensure_loaded() -> Result<()> {
    if let Some(err) = cuda_bindings::curand_load_error() {
        return Err(Error::Unavailable(err.to_string()));
    }
    Ok(())
}

/// Maps a `curandStatus_t` to [`Result<()>`].
#[inline]
pub(crate) fn check(call: &'static str, code: cuda_bindings::curandStatus_t) -> Result<()> {
    if code == cuda_bindings::curandStatus_CURAND_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(Error::Curand {
            call,
            code: CurandStatus(code),
        })
    }
}

/// Keeps a failed `Drop` teardown call `op` in the process-wide `cuda_core_ext::teardown` sink,
/// with its real cuRAND status.
#[inline]
pub(crate) fn teardown(code: cuda_bindings::curandStatus_t, op: &'static str) {
    cuda_core_ext::teardown::check(
        cuda_core_ext::teardown::Library::Curand,
        op,
        i64::from(code),
        i64::from(cuda_bindings::curandStatus_CURAND_STATUS_SUCCESS),
    );
}
