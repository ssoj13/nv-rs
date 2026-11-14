//! Error types and status-code wrappers for `cufft`.
//!
//! **What.** Defines [`FftResult`] (a `Copy` integer newtype over `cufftResult_t`), [`Error`]
//! (`thiserror`), [`Result`], and internal status-checking / context-teardown helpers.
//! **Why.** Keeps raw cuFFT status codes open-ended for future toolkits and records `Drop`
//! failures on the owning `cuda_core::Context` without panicking.
//! **Used by** every module in `cufft`.

use crate::sys;
use cuda_core::{Context, DriverError};
use std::fmt;

/// Raw `cufftResult_t` status code.
///
/// Kept as a transparent integer newtype so status codes added in newer CUDA toolkits round-trip
/// without breaking exhaustive matches.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FftResult(pub sys::cufftResult_t);

impl FftResult {
    /// Operation completed successfully (`CUFFT_SUCCESS`).
    pub const SUCCESS: Self = Self(sys::cufftResult_t_CUFFT_SUCCESS);
    /// Invalid plan handle (`CUFFT_INVALID_PLAN`).
    pub const INVALID_PLAN: Self = Self(sys::cufftResult_t_CUFFT_INVALID_PLAN);
    /// Memory allocation failed (`CUFFT_ALLOC_FAILED`).
    pub const ALLOC_FAILED: Self = Self(sys::cufftResult_t_CUFFT_ALLOC_FAILED);
    /// Invalid transform type (`CUFFT_INVALID_TYPE`).
    pub const INVALID_TYPE: Self = Self(sys::cufftResult_t_CUFFT_INVALID_TYPE);
    /// Invalid parameter value (`CUFFT_INVALID_VALUE`).
    pub const INVALID_VALUE: Self = Self(sys::cufftResult_t_CUFFT_INVALID_VALUE);
    /// Internal cuFFT error (`CUFFT_INTERNAL_ERROR`).
    pub const INTERNAL_ERROR: Self = Self(sys::cufftResult_t_CUFFT_INTERNAL_ERROR);
    /// Plan execution failed (`CUFFT_EXEC_FAILED`).
    pub const EXEC_FAILED: Self = Self(sys::cufftResult_t_CUFFT_EXEC_FAILED);
    /// Library initialization failed (`CUFFT_SETUP_FAILED`).
    pub const SETUP_FAILED: Self = Self(sys::cufftResult_t_CUFFT_SETUP_FAILED);
    /// Invalid transform size (`CUFFT_INVALID_SIZE`).
    pub const INVALID_SIZE: Self = Self(sys::cufftResult_t_CUFFT_INVALID_SIZE);
    /// Unaligned data pointer (`CUFFT_UNALIGNED_DATA`).
    pub const UNALIGNED_DATA: Self = Self(sys::cufftResult_t_CUFFT_UNALIGNED_DATA);
    /// Invalid CUDA device (`CUFFT_INVALID_DEVICE`).
    pub const INVALID_DEVICE: Self = Self(sys::cufftResult_t_CUFFT_INVALID_DEVICE);
    /// No workspace attached (`CUFFT_NO_WORKSPACE`).
    pub const NO_WORKSPACE: Self = Self(sys::cufftResult_t_CUFFT_NO_WORKSPACE);
    /// Feature not implemented (`CUFFT_NOT_IMPLEMENTED`).
    pub const NOT_IMPLEMENTED: Self = Self(sys::cufftResult_t_CUFFT_NOT_IMPLEMENTED);
    /// Operation not supported (`CUFFT_NOT_SUPPORTED`).
    pub const NOT_SUPPORTED: Self = Self(sys::cufftResult_t_CUFFT_NOT_SUPPORTED);

    /// Returns true when `self == Self::SUCCESS`.
    pub const fn is_success(self) -> bool {
        self.0 == Self::SUCCESS.0
    }

    /// Human-readable name for known cuFFT status codes.
    pub const fn name(self) -> &'static str {
        match self.0 {
            sys::cufftResult_t_CUFFT_SUCCESS => "CUFFT_SUCCESS",
            sys::cufftResult_t_CUFFT_INVALID_PLAN => "CUFFT_INVALID_PLAN",
            sys::cufftResult_t_CUFFT_ALLOC_FAILED => "CUFFT_ALLOC_FAILED",
            sys::cufftResult_t_CUFFT_INVALID_TYPE => "CUFFT_INVALID_TYPE",
            sys::cufftResult_t_CUFFT_INVALID_VALUE => "CUFFT_INVALID_VALUE",
            sys::cufftResult_t_CUFFT_INTERNAL_ERROR => "CUFFT_INTERNAL_ERROR",
            sys::cufftResult_t_CUFFT_EXEC_FAILED => "CUFFT_EXEC_FAILED",
            sys::cufftResult_t_CUFFT_SETUP_FAILED => "CUFFT_SETUP_FAILED",
            sys::cufftResult_t_CUFFT_INVALID_SIZE => "CUFFT_INVALID_SIZE",
            sys::cufftResult_t_CUFFT_UNALIGNED_DATA => "CUFFT_UNALIGNED_DATA",
            sys::cufftResult_t_CUFFT_INVALID_DEVICE => "CUFFT_INVALID_DEVICE",
            sys::cufftResult_t_CUFFT_NO_WORKSPACE => "CUFFT_NO_WORKSPACE",
            sys::cufftResult_t_CUFFT_NOT_IMPLEMENTED => "CUFFT_NOT_IMPLEMENTED",
            sys::cufftResult_t_CUFFT_NOT_SUPPORTED => "CUFFT_NOT_SUPPORTED",
            _ => "CUFFT_UNKNOWN",
        }
    }
}

impl fmt::Display for FftResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name(), self.0)
    }
}

/// Everything a cuFFT call can fail with.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `cufft` shared library could not be loaded or is missing a required symbol.
    #[error("cuFFT unavailable: {0}")]
    Unavailable(String),
    /// A cuFFT call returned a non-success status code.
    #[error("{call} failed with {code}")]
    Cufft {
        /// The cuFFT entry point.
        call: &'static str,
        /// The raw `cufftResult_t`.
        code: FftResult,
    },
    /// A CUDA driver call failed.
    #[error("CUDA: {0}")]
    Driver(#[from] DriverError),
    /// Plan workspace needs more device bytes than the caller's budget admits.
    #[error("needs {requested} device bytes, budget admits {available}")]
    Budget {
        /// Bytes required by the plan.
        requested: usize,
        /// Caller's byte budget.
        available: usize,
    },
    /// An argument or buffer extent was rejected before reaching cuFFT.
    #[error("invalid argument: {0}")]
    Invalid(&'static str),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Maps a `cufftResult_t` to [`Result<()>`].
#[inline]
pub(crate) fn check(call: &'static str, code: sys::cufftResult_t) -> Result<()> {
    if code == sys::cufftResult_t_CUFFT_SUCCESS {
        Ok(())
    } else {
        Err(Error::Cufft {
            call,
            code: FftResult(code),
        })
    }
}

/// Records a failed teardown call on the owning CUDA context.
#[inline]
pub(crate) fn teardown(ctx: &Context, code: sys::cufftResult_t) {
    if code != sys::cufftResult_t_CUFFT_SUCCESS {
        ctx.record_err(Err::<(), _>(DriverError(
            cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
        )));
    }
}
