//! Dense matrix multiplication (`GEMM`), batched, and strided-batched wrappers on [`Handle`].
//!
//! **What.** Safe wrappers for `cublasSgemm_v2`, `cublasDgemm_v2`, `cublasSgemmBatched`,
//! `cublasDgemmBatched`, `cublasSgemmStridedBatched`, `cublasDgemmStridedBatched`, and
//! mixed-precision `gemm_ex` / `gemm_strided_batched_ex`.
//! **Why.** Enforces column-major leading-dimension and buffer-length invariants before calling
//! cuBLAS, binds the caller's stream, and performs zero heap allocations on the hot path.
//! **Used by** `cublas` callers and T3 numerical verification tests.

use crate::error::check;
use crate::{ComputeType, DataType, Error, GemmAlgo, Handle, Operation, Result, sys};
use cuda_core::{DeviceBuffer, Stream};
use std::ffi::c_void;

/// Validates column-major GEMM dimensions and leading dimensions, returning the minimum required
/// element counts `(min_a, min_b, min_c)` for a single matrix of `A`, `B`, and `C`.
#[expect(
    clippy::too_many_arguments,
    reason = "validates m, n, k, transa, transb, lda, ldb, ldc"
)]
pub(crate) fn validate_gemm_dims(
    m: i32,
    n: i32,
    k: i32,
    transa: Operation,
    transb: Operation,
    lda: i32,
    ldb: i32,
    ldc: i32,
) -> Result<(usize, usize, usize)> {
    if m < 0 || n < 0 || k < 0 {
        return Err(Error::Invalid(
            "GEMM dimensions m, n, k must be non-negative",
        ));
    }
    if lda <= 0 || ldb <= 0 || ldc <= 0 {
        return Err(Error::Invalid(
            "leading dimensions lda, ldb, ldc must be positive",
        ));
    }
    let (rows_a, cols_a) = match transa {
        Operation::N => (m, k),
        Operation::T | Operation::C => (k, m),
    };
    let (rows_b, cols_b) = match transb {
        Operation::N => (k, n),
        Operation::T | Operation::C => (n, k),
    };
    if lda < rows_a.max(1) {
        return Err(Error::Invalid("lda is smaller than required rows of op(A)"));
    }
    if ldb < rows_b.max(1) {
        return Err(Error::Invalid("ldb is smaller than required rows of op(B)"));
    }
    if ldc < m.max(1) {
        return Err(Error::Invalid("ldc is smaller than m"));
    }
    let min_a = (lda as usize)
        .checked_mul(cols_a as usize)
        .ok_or(Error::Invalid("A size overflow"))?;
    let min_b = (ldb as usize)
        .checked_mul(cols_b as usize)
        .ok_or(Error::Invalid("B size overflow"))?;
    let min_c = (ldc as usize)
        .checked_mul(n as usize)
        .ok_or(Error::Invalid("C size overflow"))?;
    Ok((min_a, min_b, min_c))
}

/// Validates strided-batched buffer lengths, returning `(min_a, min_b, min_c)` in elements.
pub(crate) fn validate_strided_batch(
    min_a: usize,
    stride_a: i64,
    min_b: usize,
    stride_b: i64,
    min_c: usize,
    stride_c: i64,
    batch_count: i32,
) -> Result<(usize, usize, usize)> {
    if batch_count < 0 {
        return Err(Error::Invalid("batch_count must be non-negative"));
    }
    if stride_a < 0 || stride_b < 0 || stride_c < 0 {
        return Err(Error::Invalid("negative batch strides are not supported"));
    }
    if batch_count == 0 {
        return Ok((0, 0, 0));
    }
    let b_minus_1 = (batch_count - 1) as usize;
    let total = |base: usize, stride: i64| -> Result<usize> {
        let offset = b_minus_1
            .checked_mul(stride as usize)
            .ok_or(Error::Invalid("batch stride overflow"))?;
        offset
            .checked_add(base)
            .ok_or(Error::Invalid("batch buffer size overflow"))
    };
    Ok((
        total(min_a, stride_a)?,
        total(min_b, stride_b)?,
        total(min_c, stride_c)?,
    ))
}

#[inline]
fn checked_bytes(elems: usize, dtype: DataType) -> Result<usize> {
    elems
        .checked_mul(dtype.element_bytes())
        .ok_or(Error::Invalid("matrix byte size overflow"))
}

impl Handle {
    /// Single-precision matrix multiply: `C = alpha * op(A) * op(B) + beta * C` (column-major).
    #[expect(
        clippy::too_many_arguments,
        reason = "BLAS GEMM signature requires trans, dims, scalars, and leading dims"
    )]
    pub fn sgemm(
        &self,
        stream: &Stream,
        transa: Operation,
        transb: Operation,
        m: i32,
        n: i32,
        k: i32,
        alpha: f32,
        a: &DeviceBuffer<f32>,
        lda: i32,
        b: &DeviceBuffer<f32>,
        ldb: i32,
        beta: f32,
        c: &mut DeviceBuffer<f32>,
        ldc: i32,
    ) -> Result<()> {
        let (min_a, min_b, min_c) = validate_gemm_dims(m, n, k, transa, transb, lda, ldb, ldc)?;
        self.require_host_pointer_mode()?;
        if a.len() < min_a || b.len() < min_b || c.len() < min_c {
            return Err(Error::Invalid(
                "device buffer smaller than required GEMM matrix extent",
            ));
        }
        self.set_stream(stream)?;
        // SAFETY: `self.raw` is bound to `stream`, host `alpha`/`beta` are valid for the call,
        // and device pointers are validated against column-major matrix extents.
        check("cublasSgemm_v2", unsafe {
            sys::cublasSgemm_v2(
                self.raw(),
                transa.raw(),
                transb.raw(),
                m,
                n,
                k,
                &alpha,
                a.cu_deviceptr() as usize as *const f32,
                lda,
                b.cu_deviceptr() as usize as *const f32,
                ldb,
                &beta,
                c.cu_deviceptr() as usize as *mut f32,
                ldc,
            )
        })
    }

    /// Double-precision matrix multiply: `C = alpha * op(A) * op(B) + beta * C` (column-major).
    #[expect(
        clippy::too_many_arguments,
        reason = "BLAS GEMM signature requires trans, dims, scalars, and leading dims"
    )]
    pub fn dgemm(
        &self,
        stream: &Stream,
        transa: Operation,
        transb: Operation,
        m: i32,
        n: i32,
        k: i32,
        alpha: f64,
        a: &DeviceBuffer<f64>,
        lda: i32,
        b: &DeviceBuffer<f64>,
        ldb: i32,
        beta: f64,
        c: &mut DeviceBuffer<f64>,
        ldc: i32,
    ) -> Result<()> {
        let (min_a, min_b, min_c) = validate_gemm_dims(m, n, k, transa, transb, lda, ldb, ldc)?;
        self.require_host_pointer_mode()?;
        if a.len() < min_a || b.len() < min_b || c.len() < min_c {
            return Err(Error::Invalid(
                "device buffer smaller than required GEMM matrix extent",
            ));
        }
        self.set_stream(stream)?;
        // SAFETY: `self.raw` is bound to `stream`, host `alpha`/`beta` are valid for the call,
        // and device pointers are validated against column-major matrix extents.
        check("cublasDgemm_v2", unsafe {
            sys::cublasDgemm_v2(
                self.raw(),
                transa.raw(),
                transb.raw(),
                m,
                n,
                k,
                &alpha,
                a.cu_deviceptr() as usize as *const f64,
                lda,
                b.cu_deviceptr() as usize as *const f64,
                ldb,
                &beta,
                c.cu_deviceptr() as usize as *mut f64,
                ldc,
            )
        })
    }

    /// Strided-batched single-precision GEMM (column-major).
    #[expect(
        clippy::too_many_arguments,
        reason = "BLAS strided-batched GEMM signature"
    )]
    pub fn sgemm_strided_batched(
        &self,
        stream: &Stream,
        transa: Operation,
        transb: Operation,
        m: i32,
        n: i32,
        k: i32,
        alpha: f32,
        a: &DeviceBuffer<f32>,
        lda: i32,
        stride_a: i64,
        b: &DeviceBuffer<f32>,
        ldb: i32,
        stride_b: i64,
        beta: f32,
        c: &mut DeviceBuffer<f32>,
        ldc: i32,
        stride_c: i64,
        batch_count: i32,
    ) -> Result<()> {
        let (min_a, min_b, min_c) = validate_gemm_dims(m, n, k, transa, transb, lda, ldb, ldc)?;
        let (tot_a, tot_b, tot_c) = validate_strided_batch(
            min_a,
            stride_a,
            min_b,
            stride_b,
            min_c,
            stride_c,
            batch_count,
        )?;
        self.require_host_pointer_mode()?;
        if a.len() < tot_a || b.len() < tot_b || c.len() < tot_c {
            return Err(Error::Invalid(
                "device buffer smaller than required strided-batched GEMM extent",
            ));
        }
        self.set_stream(stream)?;
        // SAFETY: `self.raw` is bound to `stream` and all strided batch extents are validated.
        check("cublasSgemmStridedBatched", unsafe {
            sys::cublasSgemmStridedBatched(
                self.raw(),
                transa.raw(),
                transb.raw(),
                m,
                n,
                k,
                &alpha,
                a.cu_deviceptr() as usize as *const f32,
                lda,
                stride_a,
                b.cu_deviceptr() as usize as *const f32,
                ldb,
                stride_b,
                &beta,
                c.cu_deviceptr() as usize as *mut f32,
                ldc,
                stride_c,
                batch_count,
            )
        })
    }

    /// Strided-batched double-precision GEMM (column-major).
    #[expect(
        clippy::too_many_arguments,
        reason = "BLAS strided-batched GEMM signature"
    )]
    pub fn dgemm_strided_batched(
        &self,
        stream: &Stream,
        transa: Operation,
        transb: Operation,
        m: i32,
        n: i32,
        k: i32,
        alpha: f64,
        a: &DeviceBuffer<f64>,
        lda: i32,
        stride_a: i64,
        b: &DeviceBuffer<f64>,
        ldb: i32,
        stride_b: i64,
        beta: f64,
        c: &mut DeviceBuffer<f64>,
        ldc: i32,
        stride_c: i64,
        batch_count: i32,
    ) -> Result<()> {
        let (min_a, min_b, min_c) = validate_gemm_dims(m, n, k, transa, transb, lda, ldb, ldc)?;
        let (tot_a, tot_b, tot_c) = validate_strided_batch(
            min_a,
            stride_a,
            min_b,
            stride_b,
            min_c,
            stride_c,
            batch_count,
        )?;
        self.require_host_pointer_mode()?;
        if a.len() < tot_a || b.len() < tot_b || c.len() < tot_c {
            return Err(Error::Invalid(
                "device buffer smaller than required strided-batched GEMM extent",
            ));
        }
        self.set_stream(stream)?;
        // SAFETY: `self.raw` is bound to `stream` and all strided batch extents are validated.
        check("cublasDgemmStridedBatched", unsafe {
            sys::cublasDgemmStridedBatched(
                self.raw(),
                transa.raw(),
                transb.raw(),
                m,
                n,
                k,
                &alpha,
                a.cu_deviceptr() as usize as *const f64,
                lda,
                stride_a,
                b.cu_deviceptr() as usize as *const f64,
                ldb,
                stride_b,
                &beta,
                c.cu_deviceptr() as usize as *mut f64,
                ldc,
                stride_c,
                batch_count,
            )
        })
    }

    /// Batched single-precision GEMM using device-resident pointer arrays (`a_ptrs`, `b_ptrs`, `c_ptrs`).
    ///
    /// # Safety
    ///
    /// Each pointer in `a_ptrs[..batch_count]`, `b_ptrs[..batch_count]`, and
    /// `c_ptrs[..batch_count]` must point to valid device memory of at least the matrix extent
    /// implied by `(m, n, k, lda, ldb, ldc)` on `stream`.
    #[expect(clippy::too_many_arguments, reason = "BLAS batched GEMM signature")]
    pub unsafe fn sgemm_batched(
        &self,
        stream: &Stream,
        transa: Operation,
        transb: Operation,
        m: i32,
        n: i32,
        k: i32,
        alpha: f32,
        a_ptrs: &DeviceBuffer<u64>,
        lda: i32,
        b_ptrs: &DeviceBuffer<u64>,
        ldb: i32,
        beta: f32,
        c_ptrs: &mut DeviceBuffer<u64>,
        ldc: i32,
        batch_count: i32,
    ) -> Result<()> {
        validate_gemm_dims(m, n, k, transa, transb, lda, ldb, ldc)?;
        self.require_host_pointer_mode()?;
        if batch_count < 0 {
            return Err(Error::Invalid("batch_count must be non-negative"));
        }
        let bc = batch_count as usize;
        if a_ptrs.len() < bc || b_ptrs.len() < bc || c_ptrs.len() < bc {
            return Err(Error::Invalid("pointer array smaller than batch_count"));
        }
        self.set_stream(stream)?;
        // SAFETY: caller guarantees each device pointer in the pointer arrays is valid for the
        // matrix extent and lifetime of the stream operation.
        check("cublasSgemmBatched", unsafe {
            sys::cublasSgemmBatched(
                self.raw(),
                transa.raw(),
                transb.raw(),
                m,
                n,
                k,
                &alpha,
                a_ptrs.cu_deviceptr() as usize as *const *const f32,
                lda,
                b_ptrs.cu_deviceptr() as usize as *const *const f32,
                ldb,
                &beta,
                c_ptrs.cu_deviceptr() as usize as *const *mut f32,
                ldc,
                batch_count,
            )
        })
    }

    /// Batched double-precision GEMM using device-resident pointer arrays.
    ///
    /// # Safety
    ///
    /// Same device-pointer validity contract as [`Self::sgemm_batched`].
    #[expect(clippy::too_many_arguments, reason = "BLAS batched GEMM signature")]
    pub unsafe fn dgemm_batched(
        &self,
        stream: &Stream,
        transa: Operation,
        transb: Operation,
        m: i32,
        n: i32,
        k: i32,
        alpha: f64,
        a_ptrs: &DeviceBuffer<u64>,
        lda: i32,
        b_ptrs: &DeviceBuffer<u64>,
        ldb: i32,
        beta: f64,
        c_ptrs: &mut DeviceBuffer<u64>,
        ldc: i32,
        batch_count: i32,
    ) -> Result<()> {
        validate_gemm_dims(m, n, k, transa, transb, lda, ldb, ldc)?;
        self.require_host_pointer_mode()?;
        if batch_count < 0 {
            return Err(Error::Invalid("batch_count must be non-negative"));
        }
        let bc = batch_count as usize;
        if a_ptrs.len() < bc || b_ptrs.len() < bc || c_ptrs.len() < bc {
            return Err(Error::Invalid("pointer array smaller than batch_count"));
        }
        self.set_stream(stream)?;
        // SAFETY: caller guarantees each device pointer in the pointer arrays is valid.
        check("cublasDgemmBatched", unsafe {
            sys::cublasDgemmBatched(
                self.raw(),
                transa.raw(),
                transb.raw(),
                m,
                n,
                k,
                &alpha,
                a_ptrs.cu_deviceptr() as usize as *const *const f64,
                lda,
                b_ptrs.cu_deviceptr() as usize as *const *const f64,
                ldb,
                &beta,
                c_ptrs.cu_deviceptr() as usize as *const *mut f64,
                ldc,
                batch_count,
            )
        })
    }

    /// Mixed-precision `cublasGemmEx` on raw byte buffers.
    ///
    /// # Safety
    ///
    /// `alpha` and `beta` must point to values of the compute scale type (in host or device memory
    /// matching [`Self::pointer_mode`]). `a`, `b`, `c` must hold at least the required matrix
    /// elements of `a_type`, `b_type`, `c_type`.
    #[expect(
        clippy::too_many_arguments,
        reason = "cublasGemmEx C API parameter list"
    )]
    pub unsafe fn gemm_ex(
        &self,
        stream: &Stream,
        transa: Operation,
        transb: Operation,
        m: i32,
        n: i32,
        k: i32,
        alpha: *const c_void,
        a: &DeviceBuffer<u8>,
        a_type: DataType,
        lda: i32,
        b: &DeviceBuffer<u8>,
        b_type: DataType,
        ldb: i32,
        beta: *const c_void,
        c: &mut DeviceBuffer<u8>,
        c_type: DataType,
        ldc: i32,
        compute_type: ComputeType,
        algo: GemmAlgo,
    ) -> Result<()> {
        let (min_a, min_b, min_c) = validate_gemm_dims(m, n, k, transa, transb, lda, ldb, ldc)?;
        if a.len() < checked_bytes(min_a, a_type)?
            || b.len() < checked_bytes(min_b, b_type)?
            || c.len() < checked_bytes(min_c, c_type)?
        {
            return Err(Error::Invalid(
                "byte buffer smaller than required GEMM matrix extent",
            ));
        }
        if alpha.is_null() || beta.is_null() {
            return Err(Error::Invalid("alpha and beta pointers must be non-null"));
        }
        self.set_stream(stream)?;
        // SAFETY: caller guarantees `alpha`/`beta` match pointer mode and scale type; buffer byte
        // sizes are validated above.
        check("cublasGemmEx", unsafe {
            sys::cublasGemmEx(
                self.raw(),
                transa.raw(),
                transb.raw(),
                m,
                n,
                k,
                alpha,
                a.cu_deviceptr() as usize as *const c_void,
                a_type.raw(),
                lda,
                b.cu_deviceptr() as usize as *const c_void,
                b_type.raw(),
                ldb,
                beta,
                c.cu_deviceptr() as usize as *mut c_void,
                c_type.raw(),
                ldc,
                compute_type.raw(),
                algo.raw(),
            )
        })
    }

    /// Mixed-precision `cublasGemmStridedBatchedEx` on raw byte buffers.
    ///
    /// # Safety
    ///
    /// Same scalar pointer contract as [`Self::gemm_ex`].
    #[expect(
        clippy::too_many_arguments,
        reason = "cublasGemmStridedBatchedEx C API parameter list"
    )]
    pub unsafe fn gemm_strided_batched_ex(
        &self,
        stream: &Stream,
        transa: Operation,
        transb: Operation,
        m: i32,
        n: i32,
        k: i32,
        alpha: *const c_void,
        a: &DeviceBuffer<u8>,
        a_type: DataType,
        lda: i32,
        stride_a: i64,
        b: &DeviceBuffer<u8>,
        b_type: DataType,
        ldb: i32,
        stride_b: i64,
        beta: *const c_void,
        c: &mut DeviceBuffer<u8>,
        c_type: DataType,
        ldc: i32,
        stride_c: i64,
        batch_count: i32,
        compute_type: ComputeType,
        algo: GemmAlgo,
    ) -> Result<()> {
        let (min_a, min_b, min_c) = validate_gemm_dims(m, n, k, transa, transb, lda, ldb, ldc)?;
        let (tot_a, tot_b, tot_c) = validate_strided_batch(
            min_a,
            stride_a,
            min_b,
            stride_b,
            min_c,
            stride_c,
            batch_count,
        )?;
        if a.len() < checked_bytes(tot_a, a_type)?
            || b.len() < checked_bytes(tot_b, b_type)?
            || c.len() < checked_bytes(tot_c, c_type)?
        {
            return Err(Error::Invalid(
                "byte buffer smaller than required strided-batched GEMM extent",
            ));
        }
        if alpha.is_null() || beta.is_null() {
            return Err(Error::Invalid("alpha and beta pointers must be non-null"));
        }
        self.set_stream(stream)?;
        // SAFETY: caller guarantees `alpha`/`beta` match pointer mode and scale type; buffer byte
        // sizes are validated above.
        check("cublasGemmStridedBatchedEx", unsafe {
            sys::cublasGemmStridedBatchedEx(
                self.raw(),
                transa.raw(),
                transb.raw(),
                m,
                n,
                k,
                alpha,
                a.cu_deviceptr() as usize as *const c_void,
                a_type.raw(),
                lda,
                stride_a,
                b.cu_deviceptr() as usize as *const c_void,
                b_type.raw(),
                ldb,
                stride_b,
                beta,
                c.cu_deviceptr() as usize as *mut c_void,
                c_type.raw(),
                ldc,
                stride_c,
                batch_count,
                compute_type.raw(),
                algo.raw(),
            )
        })
    }

    /// Zero-overhead mixed-precision `cublasGemmEx` on raw device pointers using the stream
    /// already bound to `self` (via [`Self::set_stream`] or [`Self::set_raw_stream`]).
    ///
    /// # Safety
    /// - `a_ptr`, `b_ptr`, and `c_ptr` must be valid device allocations in `self.cuda()` spanning
    ///   at least the required column-major matrix extents for `(m, n, k, lda, ldb, ldc)` and their
    ///   data types, and remain alive until the bound CUDA stream completes.
    /// - `alpha` and `beta` must point to valid scalars matching `compute_type` and `pointer_mode`.
    #[expect(
        clippy::too_many_arguments,
        reason = "cublasGemmEx C API parameter list"
    )]
    pub unsafe fn gemm_ex_on_bound_stream(
        &self,
        transa: Operation,
        transb: Operation,
        m: i32,
        n: i32,
        k: i32,
        alpha: *const c_void,
        a_ptr: u64,
        a_type: DataType,
        lda: i32,
        b_ptr: u64,
        b_type: DataType,
        ldb: i32,
        beta: *const c_void,
        c_ptr: u64,
        c_type: DataType,
        ldc: i32,
        compute_type: ComputeType,
        algo: GemmAlgo,
    ) -> Result<()> {
        // SAFETY: caller guarantees stream binding, scalar validity, and device buffer extents.
        check("cublasGemmEx", unsafe {
            sys::cublasGemmEx(
                self.raw(),
                transa.raw(),
                transb.raw(),
                m,
                n,
                k,
                alpha,
                a_ptr as usize as *const c_void,
                a_type.raw(),
                lda,
                b_ptr as usize as *const c_void,
                b_type.raw(),
                ldb,
                beta,
                c_ptr as usize as *mut c_void,
                c_type.raw(),
                ldc,
                compute_type.raw(),
                algo.raw(),
            )
        })
    }
}
