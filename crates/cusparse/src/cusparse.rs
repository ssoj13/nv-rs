//! Safe cuSPARSE host API on top of `cusparse-sys` and `cuda-core`.
//!
//! **What.**
//! - [`handle`] — [`Handle`]: cuSPARSE library handle bound to one `cuda_core::Context`, with
//!   stream binding and pointer-mode control.
//! - [`descr`] — [`SpMatDescr`] (CSR, CSC, COO, Blocked-ELL), [`DnVecDescr`], and
//!   [`DnMatDescr`] with dimension/index validation and context-tracked teardown.
//! - [`ops`] — [`spmv`], [`spmm`], [`SpGemm`], [`SpSv`], and [`SpSm`] with two-phase workspace
//!   sizing, caller byte-budget enforcement, and reusable [`Scratch`] buffers.
//!
//! **Conventions.** Every handle and descriptor owns its native cuSPARSE object, holds an
//! `Arc<cuda_core::Context>`, and destroys it on `Drop`; teardown failures are recorded on the
//! `Context` (`Context::record_err`) rather than panicking. Status codes are wrapped in [`Status`],
//! a `Copy` integer newtype so newer toolkit codes remain representable.
//!
//! **Used by** sparse linear algebra consumers and the nv-rs cuSPARSE tests.

pub mod descr;
pub mod handle;
pub mod ops;

pub use descr::{DnMatDescr, DnVecDescr, SpMatDescr, SparseFormat};
pub use handle::Handle;
pub use ops::{
    SpGemm, SpGemmAlg, SpMmAlg, SpMvAlg, SpSm, SpSmAlg, SpSv, SpSvAlg, spmm, spmm_buffer_size,
    spmv, spmv_buffer_size,
};

pub use cusparse_sys as sys;

use cuda_core::{Context, DeviceBuffer, DriverError, Stream};
use std::ffi::{CStr, c_void};
use std::fmt;
use std::sync::Arc;

/// Raw `cusparseStatus_t` code wrapped as a `Copy` integer newtype so newer toolkit codes
/// remain representable without breaking exhaustive matches.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status(pub u32);

impl Status {
    /// `CUSPARSE_STATUS_SUCCESS`.
    pub const SUCCESS: Self = Self(sys::cusparseStatus_t_CUSPARSE_STATUS_SUCCESS);
    /// `CUSPARSE_STATUS_NOT_INITIALIZED`.
    pub const NOT_INITIALIZED: Self = Self(sys::cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED);
    /// `CUSPARSE_STATUS_ALLOC_FAILED`.
    pub const ALLOC_FAILED: Self = Self(sys::cusparseStatus_t_CUSPARSE_STATUS_ALLOC_FAILED);
    /// `CUSPARSE_STATUS_INVALID_VALUE`.
    pub const INVALID_VALUE: Self = Self(sys::cusparseStatus_t_CUSPARSE_STATUS_INVALID_VALUE);
    /// `CUSPARSE_STATUS_ARCH_MISMATCH`.
    pub const ARCH_MISMATCH: Self = Self(sys::cusparseStatus_t_CUSPARSE_STATUS_ARCH_MISMATCH);
    /// `CUSPARSE_STATUS_MAPPING_ERROR`.
    pub const MAPPING_ERROR: Self = Self(sys::cusparseStatus_t_CUSPARSE_STATUS_MAPPING_ERROR);
    /// `CUSPARSE_STATUS_EXECUTION_FAILED`.
    pub const EXECUTION_FAILED: Self = Self(sys::cusparseStatus_t_CUSPARSE_STATUS_EXECUTION_FAILED);
    /// `CUSPARSE_STATUS_INTERNAL_ERROR`.
    pub const INTERNAL_ERROR: Self = Self(sys::cusparseStatus_t_CUSPARSE_STATUS_INTERNAL_ERROR);
    /// `CUSPARSE_STATUS_MATRIX_TYPE_NOT_SUPPORTED`.
    pub const MATRIX_TYPE_NOT_SUPPORTED: Self =
        Self(sys::cusparseStatus_t_CUSPARSE_STATUS_MATRIX_TYPE_NOT_SUPPORTED);
    /// `CUSPARSE_STATUS_ZERO_PIVOT`.
    pub const ZERO_PIVOT: Self = Self(sys::cusparseStatus_t_CUSPARSE_STATUS_ZERO_PIVOT);
    /// `CUSPARSE_STATUS_NOT_SUPPORTED`.
    pub const NOT_SUPPORTED: Self = Self(sys::cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED);
    /// `CUSPARSE_STATUS_INSUFFICIENT_RESOURCES`.
    pub const INSUFFICIENT_RESOURCES: Self =
        Self(sys::cusparseStatus_t_CUSPARSE_STATUS_INSUFFICIENT_RESOURCES);

    /// Human-readable status name if cuSPARSE is loaded, or a static fallback.
    pub fn name(self) -> &'static str {
        if sys::load().is_ok() {
            // SAFETY: `cusparseGetErrorName` returns a pointer to a static NUL-terminated string.
            let ptr = unsafe { sys::cusparseGetErrorName(self.0) };
            if !ptr.is_null() {
                // SAFETY: `ptr` is non-null and points to a static C string in `cusparse`.
                if let Ok(s) = unsafe { CStr::from_ptr(ptr) }.to_str() {
                    return s;
                }
            }
        }
        match self {
            Self::SUCCESS => "CUSPARSE_STATUS_SUCCESS",
            Self::NOT_INITIALIZED => "CUSPARSE_STATUS_NOT_INITIALIZED",
            Self::ALLOC_FAILED => "CUSPARSE_STATUS_ALLOC_FAILED",
            Self::INVALID_VALUE => "CUSPARSE_STATUS_INVALID_VALUE",
            Self::ARCH_MISMATCH => "CUSPARSE_STATUS_ARCH_MISMATCH",
            Self::MAPPING_ERROR => "CUSPARSE_STATUS_MAPPING_ERROR",
            Self::EXECUTION_FAILED => "CUSPARSE_STATUS_EXECUTION_FAILED",
            Self::INTERNAL_ERROR => "CUSPARSE_STATUS_INTERNAL_ERROR",
            Self::MATRIX_TYPE_NOT_SUPPORTED => "CUSPARSE_STATUS_MATRIX_TYPE_NOT_SUPPORTED",
            Self::ZERO_PIVOT => "CUSPARSE_STATUS_ZERO_PIVOT",
            Self::NOT_SUPPORTED => "CUSPARSE_STATUS_NOT_SUPPORTED",
            Self::INSUFFICIENT_RESOURCES => "CUSPARSE_STATUS_INSUFFICIENT_RESOURCES",
            _ => "CUSPARSE_STATUS_UNKNOWN",
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

impl From<sys::cusparseStatus_t> for Status {
    fn from(code: sys::cusparseStatus_t) -> Self {
        Self(code)
    }
}

/// Everything a cuSPARSE call can fail with.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `cusparse` could not be loaded or is missing a required symbol.
    #[error("cuSPARSE unavailable: {0}")]
    Unavailable(String),
    /// A cuSPARSE entry point returned a non-success status code.
    #[error("{call} failed with {code}")]
    Status {
        /// The cuSPARSE C entry point.
        call: &'static str,
        /// The raw status code.
        code: Status,
    },
    /// A CUDA driver call failed.
    #[error("CUDA: {0}")]
    Driver(#[from] DriverError),
    /// A workspace allocation would exceed the caller's byte budget.
    #[error("needs {requested} device bytes, budget admits {available}")]
    Budget {
        /// Bytes requested by cuSPARSE.
        requested: usize,
        /// Caller-supplied byte budget.
        available: usize,
    },
    /// An argument failed validation before reaching cuSPARSE.
    #[error("invalid argument: {0}")]
    InvalidInput(&'static str),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Ensures the cuSPARSE shared library is loaded, mapping load errors into [`Error::Unavailable`].
pub(crate) fn ensure_loaded() -> Result<()> {
    sys::load()
        .map(|_| ())
        .map_err(|e| Error::Unavailable(e.to_string()))
}

/// Maps a raw `cusparseStatus_t` into [`Result<()>`].
pub(crate) fn check(call: &'static str, code: sys::cusparseStatus_t) -> Result<()> {
    if code == sys::cusparseStatus_t_CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(Error::Status {
            call,
            code: Status(code),
        })
    }
}

/// Runs a cuSPARSE teardown call with `ctx` current; failures are recorded on `ctx` instead of
/// panicking in `Drop`.
pub(crate) fn teardown(
    ctx: &Context,
    call: &'static str,
    f: impl FnOnce() -> sys::cusparseStatus_t,
) {
    if let Err(e) = ctx.bind_to_thread() {
        ctx.record_err(Err::<(), _>(e));
        return;
    }
    if let Err(e) = check(call, f()) {
        // Record teardown failure as a CUDA invalid-value marker on the context without panicking.
        let _ = e;
        ctx.record_err(Err::<(), _>(DriverError(
            cuda_core::sys::cudaError_enum_CUDA_ERROR_INVALID_VALUE,
        )));
    }
}

/// Element scalar/compute data type (`cudaDataType`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
    /// IEEE 754 half precision (`CUDA_R_16F`).
    F16,
    /// Brain floating point 16 (`CUDA_R_16BF`).
    BF16,
    /// 32-bit float (`CUDA_R_32F`).
    F32,
    /// 64-bit float (`CUDA_R_64F`).
    F64,
    /// Complex 32-bit float (`CUDA_C_32F`).
    C32,
    /// Complex 64-bit float (`CUDA_C_64F`).
    C64,
    /// Signed 8-bit integer (`CUDA_R_8I`).
    I8,
    /// Unsigned 8-bit integer (`CUDA_R_8U`).
    U8,
    /// Signed 32-bit integer (`CUDA_R_32I`).
    I32,
    /// Unsigned 32-bit integer (`CUDA_R_32U`).
    U32,
}

impl DataType {
    /// Byte size of one element of this data type.
    pub const fn byte_size(self) -> usize {
        match self {
            Self::I8 | Self::U8 => 1,
            Self::F16 | Self::BF16 => 2,
            Self::F32 | Self::I32 | Self::U32 => 4,
            Self::F64 | Self::C32 => 8,
            Self::C64 => 16,
        }
    }

    pub(crate) fn to_sys(self) -> sys::cudaDataType {
        match self {
            Self::F16 => sys::cudaDataType_t_CUDA_R_16F,
            Self::BF16 => sys::cudaDataType_t_CUDA_R_16BF,
            Self::F32 => sys::cudaDataType_t_CUDA_R_32F,
            Self::F64 => sys::cudaDataType_t_CUDA_R_64F,
            Self::C32 => sys::cudaDataType_t_CUDA_C_32F,
            Self::C64 => sys::cudaDataType_t_CUDA_C_64F,
            Self::I8 => sys::cudaDataType_t_CUDA_R_8I,
            Self::U8 => sys::cudaDataType_t_CUDA_R_8U,
            Self::I32 => sys::cudaDataType_t_CUDA_R_32I,
            Self::U32 => sys::cudaDataType_t_CUDA_R_32U,
        }
    }
}

/// Sparse index element type (`cusparseIndexType_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IndexType {
    /// 16-bit unsigned integer (`CUSPARSE_INDEX_16U`).
    U16,
    /// 32-bit signed integer (`CUSPARSE_INDEX_32I`).
    I32,
    /// 64-bit signed integer (`CUSPARSE_INDEX_64I`).
    I64,
}

impl IndexType {
    /// Byte size of one index element.
    pub const fn byte_size(self) -> usize {
        match self {
            Self::U16 => 2,
            Self::I32 => 4,
            Self::I64 => 8,
        }
    }

    pub(crate) fn to_sys(self) -> sys::cusparseIndexType_t {
        match self {
            Self::U16 => sys::cusparseIndexType_t_CUSPARSE_INDEX_16U,
            Self::I32 => sys::cusparseIndexType_t_CUSPARSE_INDEX_32I,
            Self::I64 => sys::cusparseIndexType_t_CUSPARSE_INDEX_64I,
        }
    }
}

/// Index base for sparse indices (`cusparseIndexBase_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IndexBase {
    /// Zero-based indexing (`CUSPARSE_INDEX_BASE_ZERO`).
    Zero,
    /// One-based indexing (`CUSPARSE_INDEX_BASE_ONE`).
    One,
}

impl IndexBase {
    pub(crate) fn to_sys(self) -> sys::cusparseIndexBase_t {
        match self {
            Self::Zero => sys::cusparseIndexBase_t_CUSPARSE_INDEX_BASE_ZERO,
            Self::One => sys::cusparseIndexBase_t_CUSPARSE_INDEX_BASE_ONE,
        }
    }
}

/// Matrix transpose operation (`cusparseOperation_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Op {
    /// Non-transpose (`CUSPARSE_OPERATION_NON_TRANSPOSE`).
    NonTranspose,
    /// Transpose (`CUSPARSE_OPERATION_TRANSPOSE`).
    Transpose,
    /// Conjugate transpose (`CUSPARSE_OPERATION_CONJUGATE_TRANSPOSE`).
    ConjTranspose,
}

impl Op {
    pub(crate) fn to_sys(self) -> sys::cusparseOperation_t {
        match self {
            Self::NonTranspose => sys::cusparseOperation_t_CUSPARSE_OPERATION_NON_TRANSPOSE,
            Self::Transpose => sys::cusparseOperation_t_CUSPARSE_OPERATION_TRANSPOSE,
            Self::ConjTranspose => sys::cusparseOperation_t_CUSPARSE_OPERATION_CONJUGATE_TRANSPOSE,
        }
    }
}

/// Dense matrix storage order (`cusparseOrder_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Order {
    /// Column-major (`CUSPARSE_ORDER_COL`).
    Col,
    /// Row-major (`CUSPARSE_ORDER_ROW`).
    Row,
}

impl Order {
    pub(crate) fn to_sys(self) -> sys::cusparseOrder_t {
        match self {
            Self::Col => sys::cusparseOrder_t_CUSPARSE_ORDER_COL,
            Self::Row => sys::cusparseOrder_t_CUSPARSE_ORDER_ROW,
        }
    }
}

/// Triangular fill mode (`cusparseFillMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FillMode {
    /// Lower triangular (`CUSPARSE_FILL_MODE_LOWER`).
    Lower,
    /// Upper triangular (`CUSPARSE_FILL_MODE_UPPER`).
    Upper,
}

impl FillMode {
    pub(crate) fn to_sys(self) -> sys::cusparseFillMode_t {
        match self {
            Self::Lower => sys::cusparseFillMode_t_CUSPARSE_FILL_MODE_LOWER,
            Self::Upper => sys::cusparseFillMode_t_CUSPARSE_FILL_MODE_UPPER,
        }
    }
}

/// Matrix diagonal unitness (`cusparseDiagType_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DiagType {
    /// Non-unit diagonal (`CUSPARSE_DIAG_TYPE_NON_UNIT`).
    NonUnit,
    /// Unit diagonal (`CUSPARSE_DIAG_TYPE_UNIT`).
    Unit,
}

impl DiagType {
    pub(crate) fn to_sys(self) -> sys::cusparseDiagType_t {
        match self {
            Self::NonUnit => sys::cusparseDiagType_t_CUSPARSE_DIAG_TYPE_NON_UNIT,
            Self::Unit => sys::cusparseDiagType_t_CUSPARSE_DIAG_TYPE_UNIT,
        }
    }
}

/// Scalar pointer mode (`cusparsePointerMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PointerMode {
    /// Scalars (`alpha`, `beta`) reside in host memory (`CUSPARSE_POINTER_MODE_HOST`).
    Host,
    /// Scalars (`alpha`, `beta`) reside in device memory (`CUSPARSE_POINTER_MODE_DEVICE`).
    Device,
}

impl PointerMode {
    pub(crate) fn to_sys(self) -> sys::cusparsePointerMode_t {
        match self {
            Self::Host => sys::cusparsePointerMode_t_CUSPARSE_POINTER_MODE_HOST,
            Self::Device => sys::cusparsePointerMode_t_CUSPARSE_POINTER_MODE_DEVICE,
        }
    }
}

/// Caller-owned reusable device scratch workspace bound to one [`Stream`].
///
/// Grows to the high-water mark of admitted requests and never shrinks, eliminating steady-state
/// allocations across repeated sparse operations.
pub struct Scratch {
    stream: Arc<Stream>,
    buffer: Option<DeviceBuffer<u8>>,
}

impl Scratch {
    /// Creates an empty scratch workspace for operations on `stream`.
    pub fn new(stream: &Arc<Stream>) -> Self {
        Self {
            stream: stream.clone(),
            buffer: None,
        }
    }

    /// Current capacity in bytes.
    pub fn capacity(&self) -> usize {
        self.buffer.as_ref().map_or(0, |b| b.len())
    }

    /// Stream this scratch buffer is bound to.
    pub fn stream(&self) -> &Arc<Stream> {
        &self.stream
    }

    /// Ensures at least `required` bytes are available without exceeding `budget` total bytes.
    ///
    /// Verifies that `stream` matches the stream this scratch buffer was created for.
    /// Returns a device pointer (null if `required == 0` and no buffer is allocated yet) and the
    /// available byte capacity.
    pub fn reserve_with_budget(
        &mut self,
        stream: &Stream,
        required: usize,
        budget: usize,
    ) -> Result<(*mut c_void, usize)> {
        if stream.cu_stream() != self.stream.cu_stream()
            || !Arc::ptr_eq(stream.context(), self.stream.context())
        {
            return Err(Error::InvalidInput(
                "scratch used on a different stream than it was created for",
            ));
        }
        if required > budget {
            return Err(Error::Budget {
                requested: required,
                available: budget,
            });
        }
        if required == 0 {
            let ptr = self
                .buffer
                .as_ref()
                .map_or(std::ptr::null_mut(), |b| b.cu_deviceptr() as *mut c_void);
            return Ok((ptr, self.capacity()));
        }
        if self.capacity() < required {
            // Free previous buffer in stream order before allocating the larger one.
            self.buffer = None;
            // SAFETY: `required > 0`; `self.stream` matches `stream`; cuSPARSE writes scratch before reading it.
            self.buffer =
                Some(unsafe { DeviceBuffer::<u8>::uninitialized_async(&self.stream, required) }?);
        }
        let buf = self
            .buffer
            .as_ref()
            .ok_or(Error::InvalidInput("scratch buffer missing after reserve"))?;
        Ok((buf.cu_deviceptr() as *mut c_void, buf.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_newtype_is_copy_and_preserves_unknown_codes() {
        let s = Status(9999);
        let s2 = s;
        assert_eq!(s, s2);
        assert!(
            s.name() == "CUSPARSE_STATUS_UNKNOWN" || s.name() == "unrecognized error code",
            "unexpected unknown status name: {}",
            s.name()
        );
        assert_eq!(Status::SUCCESS.name(), "CUSPARSE_STATUS_SUCCESS");
        assert_eq!(
            Status::INVALID_VALUE.name(),
            "CUSPARSE_STATUS_INVALID_VALUE"
        );
    }

    #[test]
    fn budget_error_formats_requested_and_available() {
        let err = Error::Budget {
            requested: 4096,
            available: 1024,
        };
        let msg = err.to_string();
        assert!(msg.contains("4096") && msg.contains("1024"), "{msg}");
    }

    #[test]
    fn data_and_index_byte_sizes() {
        assert_eq!(DataType::F16.byte_size(), 2);
        assert_eq!(DataType::F32.byte_size(), 4);
        assert_eq!(DataType::F64.byte_size(), 8);
        assert_eq!(DataType::C64.byte_size(), 16);
        assert_eq!(IndexType::U16.byte_size(), 2);
        assert_eq!(IndexType::I32.byte_size(), 4);
        assert_eq!(IndexType::I64.byte_size(), 8);
    }
}
