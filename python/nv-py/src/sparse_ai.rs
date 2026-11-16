//! `nv_rs.cusparse`, `nv_rs.cusolver`, and `nv_rs.cudnn` — GIL-free sparse/dense linear algebra and DNN bindings over zero-copy `DeviceBuffer`.

use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use cuda_core::Context;
use cudnn::{
    ActivationDescriptor, ActivationMode, DataType as CudnnDataType, Handle as CudnnInnerHandle,
    NanPropagation, SoftmaxAlgo, SoftmaxMode, TensorDescriptor, TensorFormat,
};
use cusolver::dn::{validate_getrf, validate_getrs, validate_potrf, validate_potrs};
use cusolver::{DnFloat, DnHandle as CusolverDnInnerHandle, Scratch as CusolverScratch};
use cusparse::descr::validate_csr;
use cusparse::ops::validate_spmv_dims;
use cusparse::{
    DataType as CusparseDataType, Handle as CusparseInnerHandle, IndexType, Op as CusparseOp,
    Scratch as CusparseScratch,
};
use nv_loader::diag::{self, Level};
use pyo3::prelude::*;

use crate::buf_util::{buf_ptr_and_bytes, validate_dev_buf};
use crate::core::{PyDeviceBuffer, PyStream};
use crate::errors::{ErrorKind, record_and_err};

// ---------------------------------------------------------------------------
// nv_rs.cusparse
// ---------------------------------------------------------------------------

py_err_mapper!(map_cusparse_err, "cusparse", cusparse::Error, {
    cusparse::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    cusparse::Error::InvalidInput(_) | cusparse::Error::Budget { .. } => ErrorKind::InvalidArgument,
    cusparse::Error::Status { .. } | cusparse::Error::Driver(_) => ErrorKind::Cuda,
});

fn check_cusparse_status(
    call: &'static str,
    code: cusparse::sys::cusparseStatus_t,
) -> cusparse::Result<()> {
    if code == cusparse::sys::cusparseStatus_t_CUSPARSE_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(cusparse::Error::Status {
            call,
            code: cusparse::Status(code),
        })
    }
}

/// RAII guard for a raw `cusparseSpMatDescr_t` created from `PyDeviceBuffer` pointers.
struct RawSpMatGuard {
    raw: cusparse::sys::cusparseSpMatDescr_t,
}

impl Drop for RawSpMatGuard {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: `self.raw` was created by `cusparseCreateCsr` and is destroyed once.
            let _ = unsafe { cusparse::sys::cusparseDestroySpMat(self.raw.cast_const()) };
            self.raw = std::ptr::null_mut();
        }
    }
}

/// RAII guard for a raw `cusparseDnVecDescr_t` created from a `PyDeviceBuffer` pointer.
struct RawDnVecGuard {
    raw: cusparse::sys::cusparseDnVecDescr_t,
}

impl Drop for RawDnVecGuard {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: `self.raw` was created by `cusparseCreateDnVec` and is destroyed once.
            let _ = unsafe { cusparse::sys::cusparseDestroyDnVec(self.raw.cast_const()) };
            self.raw = std::ptr::null_mut();
        }
    }
}

/// Returns the cuSPARSE library version (`major * 1000 + minor * 100 + patch`), GIL-free.
#[pyfunction]
#[pyo3(name = "version")]
fn cusparse_version(py: Python<'_>) -> PyResult<i32> {
    let t0 = Instant::now();
    let ver = py
        .detach(|| -> cusparse::Result<i32> {
            let ctx = Context::new(0)?;
            let handle = CusparseInnerHandle::new(&ctx)?;
            handle.version()
        })
        .map_err(|e| map_cusparse_err("version", e))?;

    diag::record_event(
        Level::Info,
        "cusparse",
        "version",
        format!("ver={ver}"),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(ver)
}

struct CusparseState {
    handle: CusparseInnerHandle,
    scratch: CusparseScratch,
    stream: PyStream,
}

/// Owning cuSPARSE handle bound to a CUDA stream.
#[pyclass(name = "CusparseHandle", module = "nv_rs.cusparse")]
pub struct PyCusparseHandle {
    state: Arc<Mutex<CusparseState>>,
}

#[pymethods]
impl PyCusparseHandle {
    /// Creates a new cuSPARSE handle bound to `stream`, GIL-free.
    #[new]
    fn new(py: Python<'_>, stream: &PyStream) -> PyResult<Self> {
        let t0 = Instant::now();
        let s = stream.clone();
        let state = py
            .detach(move || -> cusparse::Result<CusparseState> {
                let handle = CusparseInnerHandle::new(s.inner.context())?;
                handle.set_stream(&s.inner)?;
                let scratch = CusparseScratch::new(&s.inner);
                Ok(CusparseState {
                    handle,
                    scratch,
                    stream: s,
                })
            })
            .map_err(|e| map_cusparse_err("CusparseHandle::new", e))?;

        diag::record_event(
            Level::Info,
            "cusparse",
            "handle_new",
            format!("device={}", stream.device_id),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(Self {
            state: Arc::new(Mutex::new(state)),
        })
    }

    /// Rebinds this cuSPARSE handle to `stream`.
    fn set_stream(&self, py: Python<'_>, stream: &PyStream) -> PyResult<()> {
        let t0 = Instant::now();
        let state_arc = self.state.clone();
        let s = stream.clone();
        py.detach(move || -> cusparse::Result<()> {
            let mut guard = state_arc
                .lock()
                .map_err(|_| cusparse::Error::InvalidInput("cusparse handle mutex poisoned"))?;
            guard.handle.set_stream(&s.inner)?;
            if guard.stream.inner.cu_stream() != s.inner.cu_stream() {
                guard.scratch = CusparseScratch::new(&s.inner);
            }
            guard.stream = s;
            Ok(())
        })
        .map_err(|e| map_cusparse_err("set_stream", e))?;

        diag::record_event(
            Level::Trace,
            "cusparse",
            "set_stream",
            format!("device={}", stream.device_id),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    /// Returns the cuSPARSE library version (`major * 1000 + minor * 100 + patch`).
    fn version(&self, py: Python<'_>) -> PyResult<i32> {
        let state_arc = self.state.clone();
        py.detach(move || -> cusparse::Result<i32> {
            let guard = state_arc
                .lock()
                .map_err(|_| cusparse::Error::InvalidInput("cusparse handle mutex poisoned"))?;
            guard.handle.version()
        })
        .map_err(|e| map_cusparse_err("version", e))
    }

    /// Executes CSR single-precision SpMV: `y = alpha * op(A) * x + beta * y`, GIL-free.
    #[pyo3(signature = (row_offsets, col_ind, values, x, y, rows, cols, nnz, alpha=1.0, beta=0.0, transpose=false))]
    #[expect(
        clippy::too_many_arguments,
        reason = "CSR SpMV signature requires CSR buffers, x/y vectors, dimensions, scalars, and transpose flag"
    )]
    fn spmv_f32(
        &self,
        py: Python<'_>,
        row_offsets: &Bound<'_, PyDeviceBuffer>,
        col_ind: &Bound<'_, PyDeviceBuffer>,
        values: &Bound<'_, PyDeviceBuffer>,
        x: &Bound<'_, PyDeviceBuffer>,
        y: &Bound<'_, PyDeviceBuffer>,
        rows: i64,
        cols: i64,
        nnz: i64,
        alpha: f32,
        beta: f32,
        transpose: bool,
    ) -> PyResult<()> {
        if !alpha.is_finite() || !beta.is_finite() {
            return Err(record_and_err(
                "cusparse",
                "spmv_f32",
                "alpha and beta must be finite",
                ErrorKind::InvalidArgument,
            ));
        }

        let (row_ptr, row_bytes) = buf_ptr_and_bytes(row_offsets);
        let (col_ptr, col_bytes) = buf_ptr_and_bytes(col_ind);
        let (val_ptr, val_bytes) = buf_ptr_and_bytes(values);
        let (x_ptr, x_bytes) = buf_ptr_and_bytes(x);
        let (y_ptr, y_bytes) = buf_ptr_and_bytes(y);

        validate_csr(
            rows,
            cols,
            nnz,
            row_bytes,
            col_bytes,
            val_bytes,
            IndexType::I32,
            IndexType::I32,
            CusparseDataType::F32,
        )
        .map_err(|e| map_cusparse_err("spmv_f32", e))?;

        let op_enum = if transpose {
            CusparseOp::Transpose
        } else {
            CusparseOp::NonTranspose
        };
        let x_elems = (x_bytes / size_of::<f32>()) as i64;
        let y_elems = (y_bytes / size_of::<f32>()) as i64;
        let (eff_rows, eff_cols) = if transpose {
            (cols, rows)
        } else {
            (rows, cols)
        };

        validate_spmv_dims(op_enum, rows, cols, eff_cols, eff_rows)
            .map_err(|e| map_cusparse_err("spmv_f32", e))?;
        if x_elems < eff_cols || y_elems < eff_rows {
            return Err(record_and_err(
                "cusparse",
                "spmv_f32",
                format!(
                    "vector buffer too small: x has {x_elems} f32s (need {eff_cols}), y has {y_elems} f32s (need {eff_rows})"
                ),
                ErrorKind::InvalidArgument,
            ));
        }

        let req_row = (rows as usize + 1) * size_of::<i32>();
        let req_col = (nnz as usize) * size_of::<i32>();
        let req_val = (nnz as usize) * size_of::<f32>();
        let req_x = (eff_cols as usize) * size_of::<f32>();
        let req_y = (eff_rows as usize) * size_of::<f32>();

        validate_dev_buf(
            "cusparse",
            "spmv_f32",
            "row_offsets",
            row_ptr,
            row_bytes,
            req_row,
        )?;
        validate_dev_buf(
            "cusparse", "spmv_f32", "col_ind", col_ptr, col_bytes, req_col,
        )?;
        validate_dev_buf(
            "cusparse", "spmv_f32", "values", val_ptr, val_bytes, req_val,
        )?;
        validate_dev_buf("cusparse", "spmv_f32", "x", x_ptr, x_bytes, req_x)?;
        validate_dev_buf("cusparse", "spmv_f32", "y", y_ptr, y_bytes, req_y)?;

        let op_sys = if transpose {
            cusparse::sys::cusparseOperation_t_CUSPARSE_OPERATION_TRANSPOSE
        } else {
            cusparse::sys::cusparseOperation_t_CUSPARSE_OPERATION_NON_TRANSPOSE
        };

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> cusparse::Result<()> {
            let mut guard = state_arc
                .lock()
                .map_err(|_| cusparse::Error::InvalidInput("cusparse handle mutex poisoned"))?;
            guard.handle.cuda().bind_to_thread()?;
            guard.handle.set_stream(&guard.stream.inner)?;

            let mut mat_raw: cusparse::sys::cusparseSpMatDescr_t = std::ptr::null_mut();
            // SAFETY: CSR buffer sizes and pointers were validated above on the active context.
            check_cusparse_status("cusparseCreateCsr", unsafe {
                cusparse::sys::cusparseCreateCsr(
                    &mut mat_raw,
                    rows,
                    cols,
                    nnz,
                    row_ptr as usize as *mut c_void,
                    col_ptr as usize as *mut c_void,
                    val_ptr as usize as *mut c_void,
                    cusparse::sys::cusparseIndexType_t_CUSPARSE_INDEX_32I,
                    cusparse::sys::cusparseIndexType_t_CUSPARSE_INDEX_32I,
                    cusparse::sys::cusparseIndexBase_t_CUSPARSE_INDEX_BASE_ZERO,
                    cusparse::sys::cudaDataType_t_CUDA_R_32F,
                )
            })?;
            let mat_guard = RawSpMatGuard { raw: mat_raw };

            let mut x_raw: cusparse::sys::cusparseDnVecDescr_t = std::ptr::null_mut();
            // SAFETY: `x_ptr` is validated for `eff_cols` `f32` elements.
            check_cusparse_status("cusparseCreateDnVec", unsafe {
                cusparse::sys::cusparseCreateDnVec(
                    &mut x_raw,
                    eff_cols,
                    x_ptr as usize as *mut c_void,
                    cusparse::sys::cudaDataType_t_CUDA_R_32F,
                )
            })?;
            let x_guard = RawDnVecGuard { raw: x_raw };

            let mut y_raw: cusparse::sys::cusparseDnVecDescr_t = std::ptr::null_mut();
            // SAFETY: `y_ptr` is validated for `eff_rows` `f32` elements.
            check_cusparse_status("cusparseCreateDnVec", unsafe {
                cusparse::sys::cusparseCreateDnVec(
                    &mut y_raw,
                    eff_rows,
                    y_ptr as usize as *mut c_void,
                    cusparse::sys::cudaDataType_t_CUDA_R_32F,
                )
            })?;
            let y_guard = RawDnVecGuard { raw: y_raw };

            let mut ws_size = 0usize;
            // SAFETY: Descriptors and host scalars `alpha`/`beta` are valid for this call.
            check_cusparse_status("cusparseSpMV_bufferSize", unsafe {
                cusparse::sys::cusparseSpMV_bufferSize(
                    guard.handle.raw(),
                    op_sys,
                    std::ptr::from_ref(&alpha).cast::<c_void>(),
                    mat_guard.raw.cast_const(),
                    x_guard.raw.cast_const(),
                    std::ptr::from_ref(&beta).cast::<c_void>(),
                    y_guard.raw,
                    cusparse::sys::cudaDataType_t_CUDA_R_32F,
                    cusparse::sys::cusparseSpMVAlg_t_CUSPARSE_SPMV_ALG_DEFAULT,
                    &mut ws_size,
                )
            })?;

            let stream = guard.stream.inner.clone();
            let (ws_ptr, _) = guard
                .scratch
                .reserve_with_budget(&stream, ws_size, usize::MAX)?;

            // SAFETY: `ws_ptr` spans at least `ws_size` bytes on `stream`; descriptors are live.
            check_cusparse_status("cusparseSpMV_preprocess", unsafe {
                cusparse::sys::cusparseSpMV_preprocess(
                    guard.handle.raw(),
                    op_sys,
                    std::ptr::from_ref(&alpha).cast::<c_void>(),
                    mat_guard.raw.cast_const(),
                    x_guard.raw.cast_const(),
                    std::ptr::from_ref(&beta).cast::<c_void>(),
                    y_guard.raw,
                    cusparse::sys::cudaDataType_t_CUDA_R_32F,
                    cusparse::sys::cusparseSpMVAlg_t_CUSPARSE_SPMV_ALG_DEFAULT,
                    ws_ptr,
                )
            })?;

            // SAFETY: `ws_ptr` spans at least `ws_size` bytes on `stream`; descriptors are live.
            check_cusparse_status("cusparseSpMV", unsafe {
                cusparse::sys::cusparseSpMV(
                    guard.handle.raw(),
                    op_sys,
                    std::ptr::from_ref(&alpha).cast::<c_void>(),
                    mat_guard.raw.cast_const(),
                    x_guard.raw.cast_const(),
                    std::ptr::from_ref(&beta).cast::<c_void>(),
                    y_guard.raw,
                    cusparse::sys::cudaDataType_t_CUDA_R_32F,
                    cusparse::sys::cusparseSpMVAlg_t_CUSPARSE_SPMV_ALG_DEFAULT,
                    ws_ptr,
                )
            })
        })
        .map_err(|e| map_cusparse_err("spmv_f32", e))?;

        diag::record_event(
            Level::Info,
            "cusparse",
            "spmv_f32",
            format!("rows={rows} cols={cols} nnz={nnz} trans={transpose}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    fn __repr__(&self) -> String {
        match self.state.lock() {
            Ok(g) => format!("CusparseHandle(device_id={})", g.stream.device_id),
            Err(_) => "CusparseHandle(<poisoned>)".to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// nv_rs.cusolver
// ---------------------------------------------------------------------------

py_err_mapper!(map_cusolver_err, "cusolver", cusolver::Error, {
    cusolver::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    cusolver::Error::InvalidInput(_) | cusolver::Error::Budget { .. } => ErrorKind::InvalidArgument,
    cusolver::Error::Status { .. }
    | cusolver::Error::Cusparse { .. }
    | cusolver::Error::Driver(_) => ErrorKind::Cuda,
});

struct CusolverDnState {
    handle: CusolverDnInnerHandle,
    scratch: CusolverScratch,
    stream: PyStream,
}

/// Owning dense cuSOLVER handle bound to a CUDA stream.
#[pyclass(name = "CusolverDnHandle", module = "nv_rs.cusolver")]
pub struct PyCusolverDnHandle {
    state: Arc<Mutex<CusolverDnState>>,
}

#[pymethods]
impl PyCusolverDnHandle {
    /// Creates a new dense cuSOLVER handle bound to `stream`, GIL-free.
    #[new]
    fn new(py: Python<'_>, stream: &PyStream) -> PyResult<Self> {
        let t0 = Instant::now();
        let s = stream.clone();
        let state = py
            .detach(move || -> cusolver::Result<CusolverDnState> {
                let handle = CusolverDnInnerHandle::new(s.inner.context())?;
                handle.set_stream(&s.inner)?;
                let scratch = CusolverScratch::new(&s.inner);
                Ok(CusolverDnState {
                    handle,
                    scratch,
                    stream: s,
                })
            })
            .map_err(|e| map_cusolver_err("CusolverDnHandle::new", e))?;

        diag::record_event(
            Level::Info,
            "cusolver",
            "dn_handle_new",
            format!("device={}", stream.device_id),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(Self {
            state: Arc::new(Mutex::new(state)),
        })
    }

    /// Rebinds this dense cuSOLVER handle to `stream`.
    fn set_stream(&self, py: Python<'_>, stream: &PyStream) -> PyResult<()> {
        let t0 = Instant::now();
        let state_arc = self.state.clone();
        let s = stream.clone();
        py.detach(move || -> cusolver::Result<()> {
            let mut guard = state_arc
                .lock()
                .map_err(|_| cusolver::Error::InvalidInput("cusolver handle mutex poisoned"))?;
            guard.handle.set_stream(&s.inner)?;
            if guard.stream.inner.cu_stream() != s.inner.cu_stream() {
                guard.scratch = CusolverScratch::new(&s.inner);
            }
            guard.stream = s;
            Ok(())
        })
        .map_err(|e| map_cusolver_err("set_stream", e))?;

        diag::record_event(
            Level::Trace,
            "cusolver",
            "set_stream",
            format!("device={}", stream.device_id),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    /// Computes in-place single-precision LU factorization `P * A = L * U` (column-major), GIL-free.
    #[pyo3(signature = (a, ipiv, dev_info, m, n, lda=None))]
    #[expect(
        clippy::too_many_arguments,
        reason = "cuSOLVER getrf requires A, ipiv, dev_info buffers and matrix dimensions"
    )]
    fn getrf_f32(
        &self,
        py: Python<'_>,
        a: &Bound<'_, PyDeviceBuffer>,
        ipiv: &Bound<'_, PyDeviceBuffer>,
        dev_info: &Bound<'_, PyDeviceBuffer>,
        m: i32,
        n: i32,
        lda: Option<i32>,
    ) -> PyResult<()> {
        let lda = lda.unwrap_or_else(|| m.max(1));
        let (a_ptr, a_bytes) = buf_ptr_and_bytes(a);
        let (ipiv_ptr, ipiv_bytes) = buf_ptr_and_bytes(ipiv);
        let (info_ptr, info_bytes) = buf_ptr_and_bytes(dev_info);

        validate_getrf(
            m,
            n,
            lda,
            a_bytes / size_of::<f32>(),
            ipiv_bytes / size_of::<i32>(),
            info_bytes / size_of::<i32>(),
        )
        .map_err(|e| map_cusolver_err("getrf_f32", e))?;

        let req_a = if n > 0 {
            (lda as usize) * (n as usize) * size_of::<f32>()
        } else {
            0
        };
        let req_ipiv = (m.min(n).max(0) as usize) * size_of::<i32>();
        validate_dev_buf("cusolver", "getrf_f32", "a", a_ptr, a_bytes, req_a)?;
        validate_dev_buf(
            "cusolver",
            "getrf_f32",
            "ipiv",
            ipiv_ptr,
            ipiv_bytes,
            req_ipiv,
        )?;
        validate_dev_buf(
            "cusolver",
            "getrf_f32",
            "dev_info",
            info_ptr,
            info_bytes,
            size_of::<i32>(),
        )?;

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> cusolver::Result<()> {
            let mut guard = state_arc
                .lock()
                .map_err(|_| cusolver::Error::InvalidInput("cusolver handle mutex poisoned"))?;
            guard.handle.cuda().bind_to_thread()?;
            guard.handle.set_stream(&guard.stream.inner)?;

            let a_dev = a_ptr as usize as *mut f32;
            // SAFETY: `guard.handle` is bound to `guard.stream` and `a_dev` spans `lda * n` `f32`s.
            let lwork = unsafe {
                <f32 as DnFloat>::getrf_buffer_size(guard.handle.raw(), m, n, a_dev, lda)
            }?;
            let stream = guard.stream.inner.clone();
            let (ws, _) = guard
                .scratch
                .reserve_elements::<f32>(&stream, lwork, usize::MAX)?;
            // SAFETY: All device pointers and workspace `ws` were validated for `getrf`.
            unsafe {
                <f32 as DnFloat>::getrf(
                    guard.handle.raw(),
                    m,
                    n,
                    a_dev,
                    lda,
                    ws,
                    ipiv_ptr as usize as *mut i32,
                    info_ptr as usize as *mut i32,
                )
            }
        })
        .map_err(|e| map_cusolver_err("getrf_f32", e))?;

        diag::record_event(
            Level::Info,
            "cusolver",
            "getrf_f32",
            format!("m={m} n={n} lda={lda}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    /// Solves `op(A) * X = B` in-place using LU factors from `getrf_f32`, GIL-free.
    #[pyo3(signature = (a, ipiv, b, dev_info, n, nrhs=1, lda=None, ldb=None, trans=false))]
    #[expect(
        clippy::too_many_arguments,
        reason = "cuSOLVER getrs requires A, ipiv, B, dev_info buffers and dimensions"
    )]
    fn getrs_f32(
        &self,
        py: Python<'_>,
        a: &Bound<'_, PyDeviceBuffer>,
        ipiv: &Bound<'_, PyDeviceBuffer>,
        b: &Bound<'_, PyDeviceBuffer>,
        dev_info: &Bound<'_, PyDeviceBuffer>,
        n: i32,
        nrhs: i32,
        lda: Option<i32>,
        ldb: Option<i32>,
        trans: bool,
    ) -> PyResult<()> {
        let lda = lda.unwrap_or_else(|| n.max(1));
        let ldb = ldb.unwrap_or_else(|| n.max(1));
        let (a_ptr, a_bytes) = buf_ptr_and_bytes(a);
        let (ipiv_ptr, ipiv_bytes) = buf_ptr_and_bytes(ipiv);
        let (b_ptr, b_bytes) = buf_ptr_and_bytes(b);
        let (info_ptr, info_bytes) = buf_ptr_and_bytes(dev_info);

        validate_getrs(
            n,
            nrhs,
            lda,
            ldb,
            a_bytes / size_of::<f32>(),
            ipiv_bytes / size_of::<i32>(),
            b_bytes / size_of::<f32>(),
            info_bytes / size_of::<i32>(),
        )
        .map_err(|e| map_cusolver_err("getrs_f32", e))?;

        let req_a = if n > 0 {
            (lda as usize) * (n as usize) * size_of::<f32>()
        } else {
            0
        };
        let req_b = if nrhs > 0 {
            (ldb as usize) * (nrhs as usize) * size_of::<f32>()
        } else {
            0
        };
        let req_ipiv = (n.max(0) as usize) * size_of::<i32>();
        validate_dev_buf("cusolver", "getrs_f32", "a", a_ptr, a_bytes, req_a)?;
        validate_dev_buf(
            "cusolver",
            "getrs_f32",
            "ipiv",
            ipiv_ptr,
            ipiv_bytes,
            req_ipiv,
        )?;
        validate_dev_buf("cusolver", "getrs_f32", "b", b_ptr, b_bytes, req_b)?;
        validate_dev_buf(
            "cusolver",
            "getrs_f32",
            "dev_info",
            info_ptr,
            info_bytes,
            size_of::<i32>(),
        )?;

        let op_sys = if trans {
            cusolver::sys::cublasOperation_t_CUBLAS_OP_T
        } else {
            cusolver::sys::cublasOperation_t_CUBLAS_OP_N
        };

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> cusolver::Result<()> {
            let guard = state_arc
                .lock()
                .map_err(|_| cusolver::Error::InvalidInput("cusolver handle mutex poisoned"))?;
            guard.handle.cuda().bind_to_thread()?;
            guard.handle.set_stream(&guard.stream.inner)?;
            // SAFETY: All device pointers and dimensions were validated above.
            unsafe {
                <f32 as DnFloat>::getrs(
                    guard.handle.raw(),
                    op_sys,
                    n,
                    nrhs,
                    a_ptr as usize as *const f32,
                    lda,
                    ipiv_ptr as usize as *const i32,
                    b_ptr as usize as *mut f32,
                    ldb,
                    info_ptr as usize as *mut i32,
                )
            }
        })
        .map_err(|e| map_cusolver_err("getrs_f32", e))?;

        diag::record_event(
            Level::Info,
            "cusolver",
            "getrs_f32",
            format!("n={n} nrhs={nrhs} trans={trans}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    /// Computes in-place single-precision Cholesky factorization `A = L * L^T` or `U^T * U`, GIL-free.
    #[pyo3(signature = (a, dev_info, n, lda=None, upper=false))]
    fn potrf_f32(
        &self,
        py: Python<'_>,
        a: &Bound<'_, PyDeviceBuffer>,
        dev_info: &Bound<'_, PyDeviceBuffer>,
        n: i32,
        lda: Option<i32>,
        upper: bool,
    ) -> PyResult<()> {
        let lda = lda.unwrap_or_else(|| n.max(1));
        let (a_ptr, a_bytes) = buf_ptr_and_bytes(a);
        let (info_ptr, info_bytes) = buf_ptr_and_bytes(dev_info);

        validate_potrf(
            n,
            lda,
            a_bytes / size_of::<f32>(),
            info_bytes / size_of::<i32>(),
        )
        .map_err(|e| map_cusolver_err("potrf_f32", e))?;

        let req_a = if n > 0 {
            (lda as usize) * (n as usize) * size_of::<f32>()
        } else {
            0
        };
        validate_dev_buf("cusolver", "potrf_f32", "a", a_ptr, a_bytes, req_a)?;
        validate_dev_buf(
            "cusolver",
            "potrf_f32",
            "dev_info",
            info_ptr,
            info_bytes,
            size_of::<i32>(),
        )?;

        let uplo_sys = if upper {
            cusolver::sys::cublasFillMode_t_CUBLAS_FILL_MODE_UPPER
        } else {
            cusolver::sys::cublasFillMode_t_CUBLAS_FILL_MODE_LOWER
        };

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> cusolver::Result<()> {
            let mut guard = state_arc
                .lock()
                .map_err(|_| cusolver::Error::InvalidInput("cusolver handle mutex poisoned"))?;
            guard.handle.cuda().bind_to_thread()?;
            guard.handle.set_stream(&guard.stream.inner)?;

            let a_dev = a_ptr as usize as *mut f32;
            // SAFETY: `guard.handle` is bound to `guard.stream` and `a_dev` spans `lda * n` `f32`s.
            let lwork = unsafe {
                <f32 as DnFloat>::potrf_buffer_size(guard.handle.raw(), uplo_sys, n, a_dev, lda)
            }?;
            let stream = guard.stream.inner.clone();
            let (ws, lwork) = guard
                .scratch
                .reserve_elements::<f32>(&stream, lwork, usize::MAX)?;
            // SAFETY: All device pointers and workspace `ws` were validated for `potrf`.
            unsafe {
                <f32 as DnFloat>::potrf(
                    guard.handle.raw(),
                    uplo_sys,
                    n,
                    a_dev,
                    lda,
                    ws,
                    lwork,
                    info_ptr as usize as *mut i32,
                )
            }
        })
        .map_err(|e| map_cusolver_err("potrf_f32", e))?;

        diag::record_event(
            Level::Info,
            "cusolver",
            "potrf_f32",
            format!("n={n} lda={lda} upper={upper}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    /// Solves `A * X = B` in-place using Cholesky factors from `potrf_f32`, GIL-free.
    #[pyo3(signature = (a, b, dev_info, n, nrhs=1, lda=None, ldb=None, upper=false))]
    #[expect(
        clippy::too_many_arguments,
        reason = "cuSOLVER potrs requires A, B, dev_info buffers and dimensions"
    )]
    fn potrs_f32(
        &self,
        py: Python<'_>,
        a: &Bound<'_, PyDeviceBuffer>,
        b: &Bound<'_, PyDeviceBuffer>,
        dev_info: &Bound<'_, PyDeviceBuffer>,
        n: i32,
        nrhs: i32,
        lda: Option<i32>,
        ldb: Option<i32>,
        upper: bool,
    ) -> PyResult<()> {
        let lda = lda.unwrap_or_else(|| n.max(1));
        let ldb = ldb.unwrap_or_else(|| n.max(1));
        let (a_ptr, a_bytes) = buf_ptr_and_bytes(a);
        let (b_ptr, b_bytes) = buf_ptr_and_bytes(b);
        let (info_ptr, info_bytes) = buf_ptr_and_bytes(dev_info);

        validate_potrs(
            n,
            nrhs,
            lda,
            ldb,
            a_bytes / size_of::<f32>(),
            b_bytes / size_of::<f32>(),
            info_bytes / size_of::<i32>(),
        )
        .map_err(|e| map_cusolver_err("potrs_f32", e))?;

        let req_a = if n > 0 {
            (lda as usize) * (n as usize) * size_of::<f32>()
        } else {
            0
        };
        let req_b = if nrhs > 0 {
            (ldb as usize) * (nrhs as usize) * size_of::<f32>()
        } else {
            0
        };
        validate_dev_buf("cusolver", "potrs_f32", "a", a_ptr, a_bytes, req_a)?;
        validate_dev_buf("cusolver", "potrs_f32", "b", b_ptr, b_bytes, req_b)?;
        validate_dev_buf(
            "cusolver",
            "potrs_f32",
            "dev_info",
            info_ptr,
            info_bytes,
            size_of::<i32>(),
        )?;

        let uplo_sys = if upper {
            cusolver::sys::cublasFillMode_t_CUBLAS_FILL_MODE_UPPER
        } else {
            cusolver::sys::cublasFillMode_t_CUBLAS_FILL_MODE_LOWER
        };

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> cusolver::Result<()> {
            let guard = state_arc
                .lock()
                .map_err(|_| cusolver::Error::InvalidInput("cusolver handle mutex poisoned"))?;
            guard.handle.cuda().bind_to_thread()?;
            guard.handle.set_stream(&guard.stream.inner)?;
            // SAFETY: All device pointers and dimensions were validated above.
            unsafe {
                <f32 as DnFloat>::potrs(
                    guard.handle.raw(),
                    uplo_sys,
                    n,
                    nrhs,
                    a_ptr as usize as *const f32,
                    lda,
                    b_ptr as usize as *mut f32,
                    ldb,
                    info_ptr as usize as *mut i32,
                )
            }
        })
        .map_err(|e| map_cusolver_err("potrs_f32", e))?;

        diag::record_event(
            Level::Info,
            "cusolver",
            "potrs_f32",
            format!("n={n} nrhs={nrhs} upper={upper}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    fn __repr__(&self) -> String {
        match self.state.lock() {
            Ok(g) => format!("CusolverDnHandle(device_id={})", g.stream.device_id),
            Err(_) => "CusolverDnHandle(<poisoned>)".to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// nv_rs.cudnn
// ---------------------------------------------------------------------------

py_err_mapper!(map_cudnn_err, "cudnn", cudnn::Error, {
    cudnn::Error::Load(_) | cudnn::Error::Call(_) => ErrorKind::LibraryNotFound,
    cudnn::Error::Invalid(_) | cudnn::Error::Budget { .. } | cudnn::Error::Unsupported(_) => {
        ErrorKind::InvalidArgument
    },
    cudnn::Error::Status { .. } | cudnn::Error::Driver(_) => ErrorKind::Cuda,
});

py_parse_enum!(parse_activation_mode, "cudnn", "activation_forward_f32", ActivationMode, "activation mode (expected sigmoid|relu|tanh|clipped_relu|elu|identity|swish)", {
    "sigmoid" => ActivationMode::Sigmoid,
    "relu" => ActivationMode::Relu,
    "tanh" => ActivationMode::Tanh,
    "clipped_relu" => ActivationMode::ClippedRelu,
    "elu" => ActivationMode::Elu,
    "identity" => ActivationMode::Identity,
    "swish" => ActivationMode::Swish,
});

py_parse_enum!(parse_softmax_algo, "cudnn", "softmax_forward_f32", SoftmaxAlgo, "softmax algo (expected fast|accurate|log)", {
    "fast" => SoftmaxAlgo::Fast,
    "accurate" => SoftmaxAlgo::Accurate,
    "log" => SoftmaxAlgo::Log,
});

py_parse_enum!(parse_softmax_mode, "cudnn", "softmax_forward_f32", SoftmaxMode, "softmax mode (expected instance|channel)", {
    "instance" => SoftmaxMode::Instance,
    "channel" => SoftmaxMode::Channel,
});

fn validate_4d_f32_bytes(op: &str, shape: [usize; 4]) -> PyResult<usize> {
    if shape.contains(&0) {
        return Err(record_and_err(
            "cudnn",
            op,
            "tensor 4D shape dimensions must all be non-zero",
            ErrorKind::InvalidArgument,
        ));
    }
    shape
        .iter()
        .try_fold(size_of::<f32>(), |acc, &dim| acc.checked_mul(dim))
        .ok_or_else(|| {
            record_and_err(
                "cudnn",
                op,
                "tensor byte size overflow",
                ErrorKind::InvalidArgument,
            )
        })
}

/// Returns the loaded cuDNN library version (`MAJOR * 10000 + MINOR * 100 + PATCH`), GIL-free.
#[pyfunction]
#[pyo3(name = "version")]
fn cudnn_version(py: Python<'_>) -> PyResult<usize> {
    let t0 = Instant::now();
    let ver = py
        .detach(cudnn::version)
        .map_err(|e| map_cudnn_err("version", e))?;

    diag::record_event(
        Level::Info,
        "cudnn",
        "version",
        format!("ver={ver}"),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(ver)
}

struct CudnnState {
    handle: CudnnInnerHandle,
    stream: PyStream,
}

/// Owning cuDNN handle bound to a CUDA stream.
#[pyclass(name = "CudnnHandle", module = "nv_rs.cudnn")]
pub struct PyCudnnHandle {
    state: Arc<Mutex<CudnnState>>,
}

#[pymethods]
impl PyCudnnHandle {
    /// Creates a new cuDNN handle bound to `stream`, GIL-free.
    #[new]
    fn new(py: Python<'_>, stream: &PyStream) -> PyResult<Self> {
        let t0 = Instant::now();
        let s = stream.clone();
        let state = py
            .detach(move || -> cudnn::Result<CudnnState> {
                let mut handle = CudnnInnerHandle::new(s.inner.context())?;
                handle.set_stream(&s.inner)?;
                Ok(CudnnState { handle, stream: s })
            })
            .map_err(|e| map_cudnn_err("CudnnHandle::new", e))?;

        diag::record_event(
            Level::Info,
            "cudnn",
            "handle_new",
            format!("device={}", stream.device_id),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(Self {
            state: Arc::new(Mutex::new(state)),
        })
    }

    /// Rebinds this cuDNN handle to `stream`.
    fn set_stream(&self, py: Python<'_>, stream: &PyStream) -> PyResult<()> {
        let t0 = Instant::now();
        let state_arc = self.state.clone();
        let s = stream.clone();
        py.detach(move || -> cudnn::Result<()> {
            let mut guard = state_arc
                .lock()
                .map_err(|_| cudnn::Error::Invalid("cudnn handle mutex poisoned"))?;
            guard.handle.set_stream(&s.inner)?;
            guard.stream = s;
            Ok(())
        })
        .map_err(|e| map_cudnn_err("set_stream", e))?;

        diag::record_event(
            Level::Trace,
            "cudnn",
            "set_stream",
            format!("device={}", stream.device_id),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    /// Executes 4D NCHW `f32` activation forward: `y = alpha * act(x) + beta * y`, GIL-free.
    #[pyo3(signature = (x, y, shape, mode="relu", alpha=1.0, beta=0.0, coef=0.0))]
    #[expect(
        clippy::too_many_arguments,
        reason = "cuDNN activation forward requires x/y buffers, NCHW shape, mode, alpha/beta, and coef"
    )]
    fn activation_forward_f32(
        &self,
        py: Python<'_>,
        x: &Bound<'_, PyDeviceBuffer>,
        y: &Bound<'_, PyDeviceBuffer>,
        shape: [usize; 4],
        mode: &str,
        alpha: f32,
        beta: f32,
        coef: f64,
    ) -> PyResult<()> {
        if !alpha.is_finite() || !beta.is_finite() || !coef.is_finite() {
            return Err(record_and_err(
                "cudnn",
                "activation_forward_f32",
                "alpha, beta, and coef must be finite",
                ErrorKind::InvalidArgument,
            ));
        }
        let act_mode = parse_activation_mode(mode)?;
        let req_bytes = validate_4d_f32_bytes("activation_forward_f32", shape)?;
        let (x_ptr, x_bytes) = buf_ptr_and_bytes(x);
        let (y_ptr, y_bytes) = buf_ptr_and_bytes(y);
        validate_dev_buf(
            "cudnn",
            "activation_forward_f32",
            "x",
            x_ptr,
            x_bytes,
            req_bytes,
        )?;
        validate_dev_buf(
            "cudnn",
            "activation_forward_f32",
            "y",
            y_ptr,
            y_bytes,
            req_bytes,
        )?;

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> cudnn::Result<()> {
            let mut guard = state_arc
                .lock()
                .map_err(|_| cudnn::Error::Invalid("cudnn handle mutex poisoned"))?;
            let ctx = guard.handle.context().clone();
            let stream = guard.stream.inner.clone();
            guard.handle.set_stream(&stream)?;

            let desc =
                TensorDescriptor::new_4d(&ctx, TensorFormat::Nchw, CudnnDataType::F32, shape)?;
            let act_desc =
                ActivationDescriptor::new(&ctx, act_mode, NanPropagation::NotPropagate, coef)?;
            if x_bytes < desc.size_in_bytes() || y_bytes < desc.size_in_bytes() {
                return Err(cudnn::Error::Invalid(
                    "tensor buffer smaller than cuDNN descriptor size_in_bytes",
                ));
            }

            // SAFETY: Context is bound, stream is set, tensor/activation descriptors are valid,
            // and `x_ptr`/`y_ptr` span at least `desc.size_in_bytes()` bytes.
            let status = unsafe {
                cudnn_sys::cudnnActivationForward(
                    guard.handle.as_raw(),
                    act_desc.as_raw(),
                    std::ptr::from_ref(&alpha).cast::<c_void>(),
                    desc.as_raw(),
                    x_ptr as usize as *const c_void,
                    std::ptr::from_ref(&beta).cast::<c_void>(),
                    desc.as_raw(),
                    y_ptr as usize as *mut c_void,
                )
            };
            cudnn::check("cudnnActivationForward", status)
        })
        .map_err(|e| map_cudnn_err("activation_forward_f32", e))?;

        diag::record_event(
            Level::Info,
            "cudnn",
            "activation_forward_f32",
            format!("shape={shape:?} mode={mode}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    /// Executes 4D NCHW `f32` softmax forward: `y = alpha * softmax(x) + beta * y`, GIL-free.
    #[pyo3(signature = (x, y, shape, algo="accurate", mode="channel", alpha=1.0, beta=0.0))]
    #[expect(
        clippy::too_many_arguments,
        reason = "cuDNN softmax forward requires x/y buffers, NCHW shape, algo/mode, and alpha/beta"
    )]
    fn softmax_forward_f32(
        &self,
        py: Python<'_>,
        x: &Bound<'_, PyDeviceBuffer>,
        y: &Bound<'_, PyDeviceBuffer>,
        shape: [usize; 4],
        algo: &str,
        mode: &str,
        alpha: f32,
        beta: f32,
    ) -> PyResult<()> {
        if !alpha.is_finite() || !beta.is_finite() {
            return Err(record_and_err(
                "cudnn",
                "softmax_forward_f32",
                "alpha and beta must be finite",
                ErrorKind::InvalidArgument,
            ));
        }
        let sm_algo = parse_softmax_algo(algo)?;
        let sm_mode = parse_softmax_mode(mode)?;
        let req_bytes = validate_4d_f32_bytes("softmax_forward_f32", shape)?;
        let (x_ptr, x_bytes) = buf_ptr_and_bytes(x);
        let (y_ptr, y_bytes) = buf_ptr_and_bytes(y);
        validate_dev_buf(
            "cudnn",
            "softmax_forward_f32",
            "x",
            x_ptr,
            x_bytes,
            req_bytes,
        )?;
        validate_dev_buf(
            "cudnn",
            "softmax_forward_f32",
            "y",
            y_ptr,
            y_bytes,
            req_bytes,
        )?;

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> cudnn::Result<()> {
            let mut guard = state_arc
                .lock()
                .map_err(|_| cudnn::Error::Invalid("cudnn handle mutex poisoned"))?;
            let ctx = guard.handle.context().clone();
            let stream = guard.stream.inner.clone();
            guard.handle.set_stream(&stream)?;

            let desc =
                TensorDescriptor::new_4d(&ctx, TensorFormat::Nchw, CudnnDataType::F32, shape)?;
            if x_bytes < desc.size_in_bytes() || y_bytes < desc.size_in_bytes() {
                return Err(cudnn::Error::Invalid(
                    "tensor buffer smaller than cuDNN descriptor size_in_bytes",
                ));
            }

            // SAFETY: Context is bound, stream is set, tensor descriptor is valid,
            // and `x_ptr`/`y_ptr` span at least `desc.size_in_bytes()` bytes.
            let status = unsafe {
                cudnn_sys::cudnnSoftmaxForward(
                    guard.handle.as_raw(),
                    sm_algo.to_sys(),
                    sm_mode.to_sys(),
                    std::ptr::from_ref(&alpha).cast::<c_void>(),
                    desc.as_raw(),
                    x_ptr as usize as *const c_void,
                    std::ptr::from_ref(&beta).cast::<c_void>(),
                    desc.as_raw(),
                    y_ptr as usize as *mut c_void,
                )
            };
            cudnn::check("cudnnSoftmaxForward", status)
        })
        .map_err(|e| map_cudnn_err("softmax_forward_f32", e))?;

        diag::record_event(
            Level::Info,
            "cudnn",
            "softmax_forward_f32",
            format!("shape={shape:?} algo={algo} mode={mode}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    fn __repr__(&self) -> String {
        match self.state.lock() {
            Ok(g) => format!("CudnnHandle(device_id={})", g.stream.device_id),
            Err(_) => "CudnnHandle(<poisoned>)".to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Module registration
// ---------------------------------------------------------------------------

/// Registers `nv_rs.cusparse`, `nv_rs.cusolver`, and `nv_rs.cudnn` submodules.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    py_register_submodule!(py, parent, "cusparse", |m| {
        m.add_function(wrap_pyfunction!(cusparse_version, &m)?)?;
        m.add_class::<PyCusparseHandle>()?;
    })?;

    py_register_submodule!(py, parent, "cusolver", |m| {
        m.add_class::<PyCusolverDnHandle>()?;
    })?;

    py_register_submodule!(py, parent, "cudnn", |m| {
        m.add_function(wrap_pyfunction!(cudnn_version, &m)?)?;
        m.add_class::<PyCudnnHandle>()?;
    })?;

    Ok(())
}
