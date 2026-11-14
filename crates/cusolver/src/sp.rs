//! Sparse cuSOLVER (`cusolverSp`) handle, matrix descriptor, QR linear solver (`csrlsvqr`),
//! and shift-invert eigensolver (`csreigvsi`).
//!
//! **What.**
//! - [`SpHandle`] — owns `cusolverSpHandle_t`, binds a `cuda_core::Stream`, and destroys on
//!   `Drop` (recording teardown errors on `cuda_core::Context`).
//! - [`MatDescr`] — owns `cusparseMatDescr_t` for `cusolverSp` routines.
//! - [`SpFloat`] — sealed trait for `f32` and `f64` dispatching `S*` / `D*` sparse solvers.

use crate::{Error, Result, check, ensure_loaded, sys, teardown};
use cuda_core::{Context, DeviceBuffer, DriverError, Stream};
use std::sync::Arc;

/// Validates CSR sparse solver dimensions and buffer lengths.
pub fn validate_csr_solve(
    m: i32,
    nnz: i32,
    val_len: usize,
    row_ptr_len: usize,
    col_ind_len: usize,
    rhs_len: usize,
    sol_len: usize,
) -> Result<()> {
    if m < 0 || nnz < 0 {
        return Err(Error::InvalidInput("sparse CSR matrix dimensions m and nnz must be non-negative"));
    }
    let req_row = (m as usize)
        .checked_add(1)
        .ok_or(Error::InvalidInput("CSR row_ptr length overflow"))?;
    if row_ptr_len < req_row {
        return Err(Error::InvalidInput("CSR row_ptr buffer smaller than m + 1"));
    }
    if col_ind_len < nnz as usize || val_len < nnz as usize {
        return Err(Error::InvalidInput("CSR col_ind or values buffer smaller than nnz"));
    }
    if rhs_len < m as usize || sol_len < m as usize {
        return Err(Error::InvalidInput("CSR solver vector buffer smaller than m"));
    }
    Ok(())
}

/// Owning wrapper around `cusparseMatDescr_t` used by [`SpHandle`].
pub struct MatDescr {
    raw: cusparse_sys::cusparseMatDescr_t,
    cuda: Arc<Context>,
}

// SAFETY: `cusparseMatDescr_t` is metadata; thread safety is governed by caller sync.
unsafe impl Send for MatDescr {}

fn check_cusparse(call: &'static str, code: cusparse_sys::cusparseStatus_t) -> Result<()> {
    if code == cusparse_sys::cusparseStatus_t_CUSPARSE_STATUS_SUCCESS {
        return Ok(());
    }
    // SAFETY: `cusparseGetErrorName` returns a static NUL-terminated string when loaded.
    let ptr = unsafe { cusparse_sys::cusparseGetErrorName(code) };
    let name = if ptr.is_null() {
        "CUSPARSE_STATUS_UNKNOWN"
    } else {
        // SAFETY: `ptr` is non-null and points to a static C string in `cusparse`.
        unsafe { std::ffi::CStr::from_ptr(ptr) }.to_str().unwrap_or("CUSPARSE_STATUS_UNKNOWN")
    };
    Err(Error::Cusparse { call, code, name })
}

impl MatDescr {
    /// Creates a general, zero-based `cusparseMatDescr_t` on `cuda`.
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        cusparse_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        cuda.bind_to_thread()?;
        let mut raw: cusparse_sys::cusparseMatDescr_t = std::ptr::null_mut();
        // SAFETY: `cuda` is current and `raw` is a valid out-pointer.
        check_cusparse("cusparseCreateMatDescr", unsafe { cusparse_sys::cusparseCreateMatDescr(&mut raw) })?;
        // SAFETY: `raw` was just created by `cusparseCreateMatDescr`.
        if let Err(err) = check_cusparse("cusparseSetMatType", unsafe {
            cusparse_sys::cusparseSetMatType(raw, cusparse_sys::cusparseMatrixType_t_CUSPARSE_MATRIX_TYPE_GENERAL)
        }) {
            // SAFETY: clean up `raw` before returning error; record destroy failure on context.
            if check_cusparse("cusparseDestroyMatDescr", unsafe { cusparse_sys::cusparseDestroyMatDescr(raw) }).is_err() {
                cuda.record_err(Err::<(), _>(DriverError(cuda_core::sys::cudaError_enum_CUDA_ERROR_INVALID_VALUE)));
            }
            return Err(err);
        }
        // SAFETY: `raw` is valid.
        if let Err(err) = check_cusparse("cusparseSetMatIndexBase", unsafe {
            cusparse_sys::cusparseSetMatIndexBase(raw, cusparse_sys::cusparseIndexBase_t_CUSPARSE_INDEX_BASE_ZERO)
        }) {
            // SAFETY: clean up `raw` before returning error; record destroy failure on context.
            if check_cusparse("cusparseDestroyMatDescr", unsafe { cusparse_sys::cusparseDestroyMatDescr(raw) }).is_err() {
                cuda.record_err(Err::<(), _>(DriverError(cuda_core::sys::cudaError_enum_CUDA_ERROR_INVALID_VALUE)));
            }
            return Err(err);
        }
        Ok(Self { raw, cuda: cuda.clone() })
    }

    /// Casts to `cusolver_sys::cusparseMatDescr_t` (identical C ABI opaque pointer).
    pub fn as_cusolver_raw(&self) -> sys::cusparseMatDescr_t {
        self.raw.cast()
    }
}

impl Drop for MatDescr {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            if let Err(e) = self.cuda.bind_to_thread() {
                self.cuda.record_err(Err::<(), _>(e));
                return;
            }
            // SAFETY: `raw` is owned by `self` and destroyed once.
            let code = unsafe { cusparse_sys::cusparseDestroyMatDescr(raw) };
            if code != cusparse_sys::cusparseStatus_t_CUSPARSE_STATUS_SUCCESS {
                self.cuda
                    .record_err(Err::<(), _>(DriverError(cuda_core::sys::cudaError_enum_CUDA_ERROR_INVALID_VALUE)));
            }
        }
    }
}

mod sealed {
    use super::*;

    pub trait Sealed: Sized {
        fn alloc_scalar(stream: &Stream, val: Self) -> Result<DeviceBuffer<Self>>;
        fn read_scalar(buf: &DeviceBuffer<Self>, stream: &Stream) -> Result<Self>;
    }

    impl Sealed for f32 {
        fn alloc_scalar(stream: &Stream, val: Self) -> Result<DeviceBuffer<Self>> {
            Ok(DeviceBuffer::from_host(stream, &[val])?)
        }

        fn read_scalar(buf: &DeviceBuffer<Self>, stream: &Stream) -> Result<Self> {
            let host = buf.to_host_vec(stream)?;
            Ok(host[0])
        }
    }

    impl Sealed for f64 {
        fn alloc_scalar(stream: &Stream, val: Self) -> Result<DeviceBuffer<Self>> {
            Ok(DeviceBuffer::from_host(stream, &[val])?)
        }

        fn read_scalar(buf: &DeviceBuffer<Self>, stream: &Stream) -> Result<Self> {
            let host = buf.to_host_vec(stream)?;
            Ok(host[0])
        }
    }
}

/// Real floating-point scalar supported by sparse cuSOLVER (`f32` and `f64`).
pub trait SpFloat: sealed::Sealed + Copy + 'static {
    /// Returns `true` if `self` is finite and `>= 0`.
    fn is_nonneg_finite(self) -> bool;

    /// Returns `true` if `self` is finite and `> 0`.
    fn is_pos_finite(self) -> bool;

    /// Solves `A * x = b` via sparse QR (`cusolverSp*csrlsvqr`), returning `singularity`.
    ///
    /// # Safety
    /// All device pointers and `descr` must be valid on `handle`'s stream.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    unsafe fn csrlsvqr(
        handle: sys::cusolverSpHandle_t,
        m: i32,
        nnz: i32,
        descr: sys::cusparseMatDescr_t,
        csr_val: *const Self,
        csr_row_ptr: *const i32,
        csr_col_ind: *const i32,
        b: *const Self,
        tol: Self,
        reorder: i32,
        x: *mut Self,
        singularity: *mut i32,
    ) -> Result<()>;

    /// Computes an eigenvalue/eigenvector pair via shift-invert (`cusolverSp*csreigvsi`).
    ///
    /// # Safety
    /// All device pointers and `descr` must be valid on `handle`'s stream.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    unsafe fn csreigvsi(
        handle: sys::cusolverSpHandle_t,
        m: i32,
        nnz: i32,
        descr: sys::cusparseMatDescr_t,
        csr_val: *const Self,
        csr_row_ptr: *const i32,
        csr_col_ind: *const i32,
        mu0: Self,
        x0: *const Self,
        maxite: i32,
        eps: Self,
        mu: *mut Self,
        x: *mut Self,
    ) -> Result<()>;
}

impl SpFloat for f32 {
    fn is_nonneg_finite(self) -> bool {
        self.is_finite() && self >= 0.0
    }

    fn is_pos_finite(self) -> bool {
        self.is_finite() && self > 0.0
    }

    unsafe fn csrlsvqr(
        handle: sys::cusolverSpHandle_t,
        m: i32,
        nnz: i32,
        descr: sys::cusparseMatDescr_t,
        csr_val: *const Self,
        csr_row_ptr: *const i32,
        csr_col_ind: *const i32,
        b: *const Self,
        tol: Self,
        reorder: i32,
        x: *mut Self,
        singularity: *mut i32,
    ) -> Result<()> {
        // SAFETY: caller guarantees valid handle, descriptor, and buffers.
        check("cusolverSpScsrlsvqr", unsafe {
            sys::cusolverSpScsrlsvqr(handle, m, nnz, descr, csr_val, csr_row_ptr, csr_col_ind, b, tol, reorder, x, singularity)
        })
    }

    unsafe fn csreigvsi(
        handle: sys::cusolverSpHandle_t,
        m: i32,
        nnz: i32,
        descr: sys::cusparseMatDescr_t,
        csr_val: *const Self,
        csr_row_ptr: *const i32,
        csr_col_ind: *const i32,
        mu0: Self,
        x0: *const Self,
        maxite: i32,
        eps: Self,
        mu: *mut Self,
        x: *mut Self,
    ) -> Result<()> {
        // SAFETY: caller guarantees valid handle, descriptor, and buffers.
        check("cusolverSpScsreigvsi", unsafe {
            sys::cusolverSpScsreigvsi(handle, m, nnz, descr, csr_val, csr_row_ptr, csr_col_ind, mu0, x0, maxite, eps, mu, x)
        })
    }
}

impl SpFloat for f64 {
    fn is_nonneg_finite(self) -> bool {
        self.is_finite() && self >= 0.0
    }

    fn is_pos_finite(self) -> bool {
        self.is_finite() && self > 0.0
    }

    unsafe fn csrlsvqr(
        handle: sys::cusolverSpHandle_t,
        m: i32,
        nnz: i32,
        descr: sys::cusparseMatDescr_t,
        csr_val: *const Self,
        csr_row_ptr: *const i32,
        csr_col_ind: *const i32,
        b: *const Self,
        tol: Self,
        reorder: i32,
        x: *mut Self,
        singularity: *mut i32,
    ) -> Result<()> {
        // SAFETY: caller guarantees valid handle, descriptor, and buffers.
        check("cusolverSpDcsrlsvqr", unsafe {
            sys::cusolverSpDcsrlsvqr(handle, m, nnz, descr, csr_val, csr_row_ptr, csr_col_ind, b, tol, reorder, x, singularity)
        })
    }

    unsafe fn csreigvsi(
        handle: sys::cusolverSpHandle_t,
        m: i32,
        nnz: i32,
        descr: sys::cusparseMatDescr_t,
        csr_val: *const Self,
        csr_row_ptr: *const i32,
        csr_col_ind: *const i32,
        mu0: Self,
        x0: *const Self,
        maxite: i32,
        eps: Self,
        mu: *mut Self,
        x: *mut Self,
    ) -> Result<()> {
        // SAFETY: caller guarantees valid handle, descriptor, and buffers.
        check("cusolverSpDcsreigvsi", unsafe {
            sys::cusolverSpDcsreigvsi(handle, m, nnz, descr, csr_val, csr_row_ptr, csr_col_ind, mu0, x0, maxite, eps, mu, x)
        })
    }
}

/// Owning handle to a sparse cuSOLVER context (`cusolverSpHandle_t`).
pub struct SpHandle {
    raw: sys::cusolverSpHandle_t,
    cuda: Arc<Context>,
}

// SAFETY: `cusolverSpHandle_t` can be moved across host threads when externally synchronized;
// `!Sync` prevents concurrent un-synchronized calls on one handle.
unsafe impl Send for SpHandle {}

impl SpHandle {
    /// Creates a new sparse cuSOLVER handle on `cuda`.
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::cusolverSpHandle_t = std::ptr::null_mut();
        // SAFETY: `cuda` is current and `raw` is a valid out-pointer.
        check("cusolverSpCreate", unsafe { sys::cusolverSpCreate(&mut raw) })?;
        Ok(Self { raw, cuda: cuda.clone() })
    }

    /// Raw `cusolverSpHandle_t`.
    pub fn raw(&self) -> sys::cusolverSpHandle_t {
        self.raw
    }

    /// Owning CUDA context.
    pub fn cuda(&self) -> &Arc<Context> {
        &self.cuda
    }

    /// Binds `stream` to this handle.
    pub fn set_stream(&self, stream: &Stream) -> Result<()> {
        if !Arc::ptr_eq(&self.cuda, stream.context()) {
            return Err(Error::InvalidInput("stream belongs to a different CUDA context than SpHandle"));
        }
        // SAFETY: `self.raw` and `stream.cu_stream()` are valid on `self.cuda`.
        check("cusolverSpSetStream", unsafe { sys::cusolverSpSetStream(self.raw, stream.cu_stream()) })
    }

    /// Solves sparse linear system `A * x = b` using QR factorization on `stream`.
    ///
    /// Returns `-1` if `A` is nonsingular within `tol`, or the zero-based index of the first
    /// singular row.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn csrlsvqr<T: SpFloat>(
        &self,
        stream: &Stream,
        m: i32,
        nnz: i32,
        descr: &MatDescr,
        csr_val: &DeviceBuffer<T>,
        csr_row_ptr: &DeviceBuffer<i32>,
        csr_col_ind: &DeviceBuffer<i32>,
        b: &DeviceBuffer<T>,
        tol: T,
        reorder: i32,
        x: &mut DeviceBuffer<T>,
    ) -> Result<i32> {
        if !tol.is_nonneg_finite() {
            return Err(Error::InvalidInput("csrlsvqr requires finite tol >= 0"));
        }
        validate_csr_solve(m, nnz, csr_val.len(), csr_row_ptr.len(), csr_col_ind.len(), b.len(), x.len())?;
        self.set_stream(stream)?;
        let mut singularity = -1i32;
        // SAFETY: buffer lengths were validated above; `singularity` is a valid host out-pointer.
        unsafe {
            T::csrlsvqr(
                self.raw,
                m,
                nnz,
                descr.as_cusolver_raw(),
                csr_val.cu_deviceptr() as *const T,
                csr_row_ptr.cu_deviceptr() as *const i32,
                csr_col_ind.cu_deviceptr() as *const i32,
                b.cu_deviceptr() as *const T,
                tol,
                reorder,
                x.cu_deviceptr() as *mut T,
                &mut singularity,
            )?;
        }
        Ok(singularity)
    }

    /// Computes an eigenvalue/eigenvector pair `(mu, x)` closest to `mu0` via shift-invert
    /// iteration on `stream`.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn csreigvsi<T: SpFloat>(
        &self,
        stream: &Stream,
        m: i32,
        nnz: i32,
        descr: &MatDescr,
        csr_val: &DeviceBuffer<T>,
        csr_row_ptr: &DeviceBuffer<i32>,
        csr_col_ind: &DeviceBuffer<i32>,
        mu0: T,
        x0: &DeviceBuffer<T>,
        maxite: i32,
        eps: T,
        x: &mut DeviceBuffer<T>,
    ) -> Result<T> {
        if maxite <= 0 {
            return Err(Error::InvalidInput("csreigvsi requires maxite > 0"));
        }
        if !eps.is_pos_finite() {
            return Err(Error::InvalidInput("csreigvsi requires finite eps > 0"));
        }
        validate_csr_solve(m, nnz, csr_val.len(), csr_row_ptr.len(), csr_col_ind.len(), x0.len(), x.len())?;
        self.set_stream(stream)?;
        let d_mu = T::alloc_scalar(stream, mu0)?;
        // SAFETY: buffer lengths were validated above; `d_mu` is a 1-element device buffer on `stream`.
        unsafe {
            T::csreigvsi(
                self.raw,
                m,
                nnz,
                descr.as_cusolver_raw(),
                csr_val.cu_deviceptr() as *const T,
                csr_row_ptr.cu_deviceptr() as *const i32,
                csr_col_ind.cu_deviceptr() as *const i32,
                mu0,
                x0.cu_deviceptr() as *const T,
                maxite,
                eps,
                d_mu.cu_deviceptr() as *mut T,
                x.cu_deviceptr() as *mut T,
            )?;
        }
        T::read_scalar(&d_mu, stream)
    }
}

impl Drop for SpHandle {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            teardown(&self.cuda, "cusolverSpDestroy", || {
                // SAFETY: `raw` is owned by `self` and destroyed once.
                unsafe { sys::cusolverSpDestroy(raw) }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csr_solve_validation_rejects_bad_lengths() {
        assert!(validate_csr_solve(-1, 2, 2, 2, 2, 1, 1).is_err());
        assert!(validate_csr_solve(3, 4, 4, 3, 4, 3, 3).is_err());
        assert!(validate_csr_solve(3, 4, 3, 4, 4, 3, 3).is_err());
        assert!(validate_csr_solve(3, 4, 4, 4, 3, 3, 3).is_err());
        assert!(validate_csr_solve(3, 4, 4, 4, 4, 2, 3).is_err());
        assert!(validate_csr_solve(3, 4, 4, 4, 4, 3, 2).is_err());
        assert!(validate_csr_solve(3, 4, 4, 4, 4, 3, 3).is_ok());
    }
}
