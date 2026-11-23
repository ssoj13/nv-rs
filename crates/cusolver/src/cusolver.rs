//! Safe cuSOLVER host API on top of `cusolver-sys`, `cusolvermg-sys`, and `cuda-core`.
//!
//! **What.**
//! - [`dn`] — [`DnHandle`]: dense linear algebra (`GETRF`/`GETRS`, `POTRF`/`POTRS`,
//!   `GEQRF`/`ORGQR`, `GESVD`, `SYEVD`) for `f32` and `f64` with two-phase `bufferSize` ->
//!   byte budget -> [`Scratch`] (the shared `cuda_core_ext::Scratch`, re-exported).
//! - [`sp`] — [`SpHandle`] and [`MatDescr`]: sparse QR linear solver (`csrlsvqr`) and
//!   shift-invert eigensolver (`csreigvsi`).
//! - [`mg`] — [`MgHandle`], [`MgGrid`], and [`MgMatrixDesc`]: multi-GPU dense `POTRF`/`POTRS`/
//!   `POTRI`, `GETRF`/`GETRS`, and `SYEVD`.
//!
//! **Conventions.** Every handle owns its native cuSOLVER object, holds an
//! `Arc<cuda_core::Context>`, and destroys it on `Drop` without panicking: a failed destroy status
//! is kept with its real code in `cuda_core_ext::teardown` ([`teardown::Library::Cusolver`];
//! `cusparseDestroyMatDescr` under `Library::Cusparse`), a failed context bind stays on the
//! `Context` (`Context::record_err`). Contexts are compared with `cuda_core_ext::same_context`.
//! Status codes are wrapped in [`Status`],
//! a `Copy` integer newtype so newer toolkit codes remain representable.
//!
//! **Used by** dense/sparse solver consumers and the nv-rs cuSOLVER tests.

pub mod dn;
pub mod mg;
pub mod sp;

pub use dn::{DnFloat, DnHandle, SvdJob};
pub use mg::{MgDataType, MgGrid, MgGridMapping, MgHandle, MgMatrixDesc};
pub use sp::{MatDescr, SpFloat, SpHandle};

pub use cusolver_sys as sys;
pub use cusolvermg_sys as mg_sys;

pub use cuda_core_ext::{Scratch, ScratchError};

use cuda_core::{Context, DriverError, Stream};
use cuda_core_ext::teardown;
use std::fmt;

/// Raw `cusolverStatus_t` code wrapped as a `Copy` integer newtype so newer toolkit codes
/// remain representable without breaking exhaustive matches.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status(pub u32);

impl Status {
    /// `CUSOLVER_STATUS_SUCCESS`.
    pub const SUCCESS: Self = Self(sys::cusolverStatus_t_CUSOLVER_STATUS_SUCCESS);
    /// `CUSOLVER_STATUS_NOT_INITIALIZED`.
    pub const NOT_INITIALIZED: Self = Self(sys::cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED);
    /// `CUSOLVER_STATUS_ALLOC_FAILED`.
    pub const ALLOC_FAILED: Self = Self(sys::cusolverStatus_t_CUSOLVER_STATUS_ALLOC_FAILED);
    /// `CUSOLVER_STATUS_INVALID_VALUE`.
    pub const INVALID_VALUE: Self = Self(sys::cusolverStatus_t_CUSOLVER_STATUS_INVALID_VALUE);
    /// `CUSOLVER_STATUS_ARCH_MISMATCH`.
    pub const ARCH_MISMATCH: Self = Self(sys::cusolverStatus_t_CUSOLVER_STATUS_ARCH_MISMATCH);
    /// `CUSOLVER_STATUS_MAPPING_ERROR`.
    pub const MAPPING_ERROR: Self = Self(sys::cusolverStatus_t_CUSOLVER_STATUS_MAPPING_ERROR);
    /// `CUSOLVER_STATUS_EXECUTION_FAILED`.
    pub const EXECUTION_FAILED: Self = Self(sys::cusolverStatus_t_CUSOLVER_STATUS_EXECUTION_FAILED);
    /// `CUSOLVER_STATUS_INTERNAL_ERROR`.
    pub const INTERNAL_ERROR: Self = Self(sys::cusolverStatus_t_CUSOLVER_STATUS_INTERNAL_ERROR);
    /// `CUSOLVER_STATUS_MATRIX_TYPE_NOT_SUPPORTED`.
    pub const MATRIX_TYPE_NOT_SUPPORTED: Self =
        Self(sys::cusolverStatus_t_CUSOLVER_STATUS_MATRIX_TYPE_NOT_SUPPORTED);
    /// `CUSOLVER_STATUS_NOT_SUPPORTED`.
    pub const NOT_SUPPORTED: Self = Self(sys::cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED);
    /// `CUSOLVER_STATUS_ZERO_PIVOT`.
    pub const ZERO_PIVOT: Self = Self(sys::cusolverStatus_t_CUSOLVER_STATUS_ZERO_PIVOT);
    /// `CUSOLVER_STATUS_INVALID_LICENSE`.
    pub const INVALID_LICENSE: Self = Self(sys::cusolverStatus_t_CUSOLVER_STATUS_INVALID_LICENSE);
    /// `CUSOLVER_STATUS_INVALID_WORKSPACE`.
    pub const INVALID_WORKSPACE: Self =
        Self(sys::cusolverStatus_t_CUSOLVER_STATUS_INVALID_WORKSPACE);

    /// Static human-readable name for known cuSOLVER status codes.
    pub const fn name(self) -> &'static str {
        match self.0 {
            x if x == Self::SUCCESS.0 => "CUSOLVER_STATUS_SUCCESS",
            x if x == Self::NOT_INITIALIZED.0 => "CUSOLVER_STATUS_NOT_INITIALIZED",
            x if x == Self::ALLOC_FAILED.0 => "CUSOLVER_STATUS_ALLOC_FAILED",
            x if x == Self::INVALID_VALUE.0 => "CUSOLVER_STATUS_INVALID_VALUE",
            x if x == Self::ARCH_MISMATCH.0 => "CUSOLVER_STATUS_ARCH_MISMATCH",
            x if x == Self::MAPPING_ERROR.0 => "CUSOLVER_STATUS_MAPPING_ERROR",
            x if x == Self::EXECUTION_FAILED.0 => "CUSOLVER_STATUS_EXECUTION_FAILED",
            x if x == Self::INTERNAL_ERROR.0 => "CUSOLVER_STATUS_INTERNAL_ERROR",
            x if x == Self::MATRIX_TYPE_NOT_SUPPORTED.0 => {
                "CUSOLVER_STATUS_MATRIX_TYPE_NOT_SUPPORTED"
            }
            x if x == Self::NOT_SUPPORTED.0 => "CUSOLVER_STATUS_NOT_SUPPORTED",
            x if x == Self::ZERO_PIVOT.0 => "CUSOLVER_STATUS_ZERO_PIVOT",
            x if x == Self::INVALID_LICENSE.0 => "CUSOLVER_STATUS_INVALID_LICENSE",
            x if x == sys::cusolverStatus_t_CUSOLVER_STATUS_IRS_PARAMS_NOT_INITIALIZED => {
                "CUSOLVER_STATUS_IRS_PARAMS_NOT_INITIALIZED"
            }
            x if x == sys::cusolverStatus_t_CUSOLVER_STATUS_IRS_PARAMS_INVALID => {
                "CUSOLVER_STATUS_IRS_PARAMS_INVALID"
            }
            x if x == sys::cusolverStatus_t_CUSOLVER_STATUS_IRS_PARAMS_INVALID_PREC => {
                "CUSOLVER_STATUS_IRS_PARAMS_INVALID_PREC"
            }
            x if x == sys::cusolverStatus_t_CUSOLVER_STATUS_IRS_PARAMS_INVALID_REFINE => {
                "CUSOLVER_STATUS_IRS_PARAMS_INVALID_REFINE"
            }
            x if x == sys::cusolverStatus_t_CUSOLVER_STATUS_IRS_PARAMS_INVALID_MAXITER => {
                "CUSOLVER_STATUS_IRS_PARAMS_INVALID_MAXITER"
            }
            x if x == sys::cusolverStatus_t_CUSOLVER_STATUS_IRS_INTERNAL_ERROR => {
                "CUSOLVER_STATUS_IRS_INTERNAL_ERROR"
            }
            x if x == sys::cusolverStatus_t_CUSOLVER_STATUS_IRS_NOT_SUPPORTED => {
                "CUSOLVER_STATUS_IRS_NOT_SUPPORTED"
            }
            x if x == sys::cusolverStatus_t_CUSOLVER_STATUS_IRS_OUT_OF_RANGE => {
                "CUSOLVER_STATUS_IRS_OUT_OF_RANGE"
            }
            x if x
                == sys::cusolverStatus_t_CUSOLVER_STATUS_IRS_NRHS_NOT_SUPPORTED_FOR_REFINE_GMRES =>
            {
                "CUSOLVER_STATUS_IRS_NRHS_NOT_SUPPORTED_FOR_REFINE_GMRES"
            }
            x if x == sys::cusolverStatus_t_CUSOLVER_STATUS_IRS_INFOS_NOT_INITIALIZED => {
                "CUSOLVER_STATUS_IRS_INFOS_NOT_INITIALIZED"
            }
            x if x == sys::cusolverStatus_t_CUSOLVER_STATUS_IRS_INFOS_NOT_DESTROYED => {
                "CUSOLVER_STATUS_IRS_INFOS_NOT_DESTROYED"
            }
            x if x == sys::cusolverStatus_t_CUSOLVER_STATUS_IRS_MATRIX_SINGULAR => {
                "CUSOLVER_STATUS_IRS_MATRIX_SINGULAR"
            }
            x if x == Self::INVALID_WORKSPACE.0 => "CUSOLVER_STATUS_INVALID_WORKSPACE",
            _ => "CUSOLVER_STATUS_UNKNOWN",
        }
    }
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Status({}: {})", self.0, self.name())
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name(), self.0)
    }
}

impl From<sys::cusolverStatus_t> for Status {
    fn from(code: sys::cusolverStatus_t) -> Self {
        Self(code)
    }
}

/// Everything a cuSOLVER call can fail with.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `cusolver` or `cusolvermg` could not be loaded or is missing a required symbol.
    #[error("cuSOLVER unavailable: {0}")]
    Unavailable(String),
    /// A cuSOLVER entry point returned a non-success status code.
    #[error("{call} failed with {code}")]
    Status {
        /// The cuSOLVER C entry point.
        call: &'static str,
        /// The raw status code.
        code: Status,
    },
    /// A cuSPARSE entry point (used by `cusolverSp` descriptors) returned a non-success status code.
    #[error("{call} failed with {name} ({code})")]
    Cusparse {
        /// The cuSPARSE C entry point.
        call: &'static str,
        /// The raw `cusparseStatus_t` code.
        code: u32,
        /// Human-readable status name.
        name: &'static str,
    },
    /// A CUDA driver call failed.
    #[error("CUDA: {0}")]
    Driver(#[from] DriverError),
    /// A workspace allocation would exceed the caller's byte budget.
    #[error("needs {requested} device bytes, budget admits {available}")]
    Budget {
        /// Bytes requested by cuSOLVER.
        requested: usize,
        /// Caller-supplied byte budget.
        available: usize,
    },
    /// An argument failed validation before reaching cuSOLVER.
    #[error("invalid argument: {0}")]
    InvalidInput(&'static str),
}

impl From<ScratchError> for Error {
    fn from(e: ScratchError) -> Self {
        match e {
            ScratchError::WrongStream => {
                Error::InvalidInput("scratch used on a different stream than it was created for")
            }
            ScratchError::Budget {
                requested,
                available,
            } => Error::Budget {
                requested,
                available,
            },
            ScratchError::Misaligned { .. } => {
                Error::InvalidInput("scratch workspace below the required alignment")
            }
            ScratchError::Driver(e) => Error::Driver(e),
        }
    }
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn ensure_loaded() -> Result<()> {
    sys::load()
        .map(|_| ())
        .map_err(|e| Error::Unavailable(e.to_string()))
}

pub(crate) fn ensure_mg_loaded() -> Result<()> {
    mg_sys::load()
        .map(|_| ())
        .map_err(|e| Error::Unavailable(e.to_string()))
}

pub(crate) fn check(call: &'static str, code: sys::cusolverStatus_t) -> Result<()> {
    if code == sys::cusolverStatus_t_CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(Error::Status {
            call,
            code: Status(code),
        })
    }
}

pub(crate) fn check_mg(call: &'static str, code: mg_sys::cusolverStatus_t) -> Result<()> {
    if code == mg_sys::cusolverStatus_t_CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(Error::Status {
            call,
            code: Status(code),
        })
    }
}

/// Runs a cuSOLVER teardown call with `ctx` current. A failed bind stays on `ctx`
/// ([`teardown::bind`]) and skips the call; a failed status goes to the shared teardown sink with
/// its real code. Never panics; used from `Drop`.
pub(crate) fn teardown_cusolver(
    ctx: &Context,
    call: &'static str,
    f: impl FnOnce() -> sys::cusolverStatus_t,
) {
    if teardown::bind(ctx) {
        teardown::check(
            teardown::Library::Cusolver,
            call,
            i64::from(f()),
            i64::from(sys::cusolverStatus_t_CUSOLVER_STATUS_SUCCESS),
        );
    }
}

/// Keeps a failed `cusolverMg` teardown status in the shared teardown sink with its real code.
pub(crate) fn teardown_check_mg(call: &'static str, status: mg_sys::cusolverStatus_t) {
    teardown::check(
        teardown::Library::Cusolver,
        call,
        i64::from(status),
        i64::from(mg_sys::cusolverStatus_t_CUSOLVER_STATUS_SUCCESS),
    );
}

/// Triangular fill mode (`cublasFillMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FillMode {
    /// Lower triangle (`CUBLAS_FILL_MODE_LOWER`).
    Lower,
    /// Upper triangle (`CUBLAS_FILL_MODE_UPPER`).
    Upper,
}

impl FillMode {
    pub(crate) fn to_sys(self) -> sys::cublasFillMode_t {
        match self {
            Self::Lower => sys::cublasFillMode_t_CUBLAS_FILL_MODE_LOWER,
            Self::Upper => sys::cublasFillMode_t_CUBLAS_FILL_MODE_UPPER,
        }
    }

    pub(crate) fn to_mg_sys(self) -> mg_sys::cublasFillMode_t {
        match self {
            Self::Lower => mg_sys::cublasFillMode_t_CUBLAS_FILL_MODE_LOWER,
            Self::Upper => mg_sys::cublasFillMode_t_CUBLAS_FILL_MODE_UPPER,
        }
    }
}

/// Matrix transpose operation (`cublasOperation_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Op {
    /// Non-transpose (`CUBLAS_OP_N`).
    NonTranspose,
    /// Transpose (`CUBLAS_OP_T`).
    Transpose,
    /// Conjugate transpose (`CUBLAS_OP_C`).
    ConjTranspose,
}

impl Op {
    pub(crate) fn to_sys(self) -> sys::cublasOperation_t {
        match self {
            Self::NonTranspose => sys::cublasOperation_t_CUBLAS_OP_N,
            Self::Transpose => sys::cublasOperation_t_CUBLAS_OP_T,
            Self::ConjTranspose => sys::cublasOperation_t_CUBLAS_OP_C,
        }
    }

    pub(crate) fn to_mg_sys(self) -> mg_sys::cublasOperation_t {
        match self {
            Self::NonTranspose => mg_sys::cublasOperation_t_CUBLAS_OP_N,
            Self::Transpose => mg_sys::cublasOperation_t_CUBLAS_OP_T,
            Self::ConjTranspose => mg_sys::cublasOperation_t_CUBLAS_OP_C,
        }
    }
}

/// Eigenvector computation mode (`cusolverEigMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EigMode {
    /// Compute eigenvalues only (`CUSOLVER_EIG_MODE_NOVECTOR`).
    NoVector,
    /// Compute eigenvalues and eigenvectors (`CUSOLVER_EIG_MODE_VECTOR`).
    Vector,
}

impl EigMode {
    pub(crate) fn to_sys(self) -> sys::cusolverEigMode_t {
        match self {
            Self::NoVector => sys::cusolverEigMode_t_CUSOLVER_EIG_MODE_NOVECTOR,
            Self::Vector => sys::cusolverEigMode_t_CUSOLVER_EIG_MODE_VECTOR,
        }
    }

    pub(crate) fn to_mg_sys(self) -> mg_sys::cusolverEigMode_t {
        match self {
            Self::NoVector => mg_sys::cusolverEigMode_t_CUSOLVER_EIG_MODE_NOVECTOR,
            Self::Vector => mg_sys::cusolverEigMode_t_CUSOLVER_EIG_MODE_VECTOR,
        }
    }
}

/// Reserves `scratch` workspace for `lwork` elements of `T` on `stream` without exceeding
/// `budget` bytes; returns the typed device pointer and `lwork` (null while `lwork == 0` and
/// nothing is allocated).
pub(crate) fn reserve_elements<T>(
    scratch: &mut Scratch,
    stream: &Stream,
    lwork: i32,
    budget: usize,
) -> Result<(*mut T, i32)> {
    let elems = usize::try_from(lwork).map_err(|_| {
        Error::InvalidInput("cuSOLVER reported negative workspace element count")
    })?;
    let bytes = elems
        .checked_mul(std::mem::size_of::<T>())
        .ok_or(Error::InvalidInput("workspace byte size overflow"))?;
    let (ptr, _) = scratch.reserve(stream, bytes, Some(budget))?;
    Ok((ptr as usize as *mut T, lwork))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_newtype_is_copy_and_preserves_unknown_codes() {
        let s = Status(4242);
        let s2 = s;
        assert_eq!(s, s2);
        assert_eq!(s.name(), "CUSOLVER_STATUS_UNKNOWN");
        assert_eq!(Status::SUCCESS.name(), "CUSOLVER_STATUS_SUCCESS");
        assert_eq!(
            Status::INVALID_WORKSPACE.name(),
            "CUSOLVER_STATUS_INVALID_WORKSPACE"
        );
    }

    #[test]
    fn status_names_cover_irs_codes() {
        assert_eq!(Status(12).name(), "CUSOLVER_STATUS_IRS_PARAMS_NOT_INITIALIZED");
        assert_eq!(Status(16).name(), "CUSOLVER_STATUS_IRS_PARAMS_INVALID_MAXITER");
        assert_eq!(Status(20).name(), "CUSOLVER_STATUS_IRS_INTERNAL_ERROR");
        assert_eq!(
            Status(23).name(),
            "CUSOLVER_STATUS_IRS_NRHS_NOT_SUPPORTED_FOR_REFINE_GMRES"
        );
        assert_eq!(Status(26).name(), "CUSOLVER_STATUS_IRS_INFOS_NOT_DESTROYED");
        assert_eq!(Status(30).name(), "CUSOLVER_STATUS_IRS_MATRIX_SINGULAR");
        // Every code the bindings define has a name.
        for code in (0..=31).filter(|c| ![17, 18, 19, 24, 27, 28, 29].contains(c)) {
            assert_ne!(Status(code).name(), "CUSOLVER_STATUS_UNKNOWN", "code {code}");
        }
    }

    #[test]
    fn budget_error_formats_requested_and_available() {
        let err = Error::Budget {
            requested: 8192,
            available: 2048,
        };
        let msg = err.to_string();
        assert!(msg.contains("8192") && msg.contains("2048"), "{msg}");
    }
}
