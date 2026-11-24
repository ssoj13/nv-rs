//! `nv_rs.cusparse`, `nv_rs.cusolver`, and `nv_rs.cudnn` — GIL-free sparse/dense linear algebra and DNN bindings over zero-copy `DeviceBuffer`.
//!
//! **How.** Each call validates its buffers with `buf_util::check_buf` and runs the native work
//! inside `buf_util::enqueue_leased`, so the Python buffers stay allocated until it completes.
//! cuSPARSE SpMV and the cuSOLVER Dn calls go through the safe wrappers (`cusparse::spmv`,
//! `cusolver::DnHandle::{getrf, getrs, potrf, potrs}`) on typed views (`compute::typed_view`).
//! cuDNN activation/softmax go through `cudnn::activation_forward` / `softmax_forward` on typed
//! views; the wrapper compares contexts by value (`cuda_core_ext::same_context`), so DLPack
//! imports and buffers of another `Arc<Context>` for the same CUDA context are accepted.

use std::sync::{Arc, Mutex};
use std::time::Instant;

use cuda_core::Context;
use cudnn::{
    ActivationDescriptor, ActivationMode, DataType as CudnnDataType, Handle as CudnnInnerHandle,
    NanPropagation, SoftmaxAlgo, SoftmaxMode, TensorDescriptor, TensorFormat,
};
use cusolver::dn::{validate_getrf, validate_getrs, validate_potrf, validate_potrs};
use cusolver::{
    DnHandle as CusolverDnInnerHandle, FillMode as CusolverFillMode, Op as CusolverOp,
    Scratch as CusolverScratch,
};
use cusparse::{
    DataType as CusparseDataType, DnVecDescr, Handle as CusparseInnerHandle, IndexBase,
    Op as CusparseOp, Scratch as CusparseScratch, SpMatDescr, SpMvAlg,
};
use nv_loader::diag::{self, Level};
use pyo3::prelude::*;

use crate::buf_util::{
    Access, check_buf, close_slot, closed as closed_err, drain_stream, enqueue_leased, is_closed,
    open, poisoned, typed_view,
};
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

/// Returns the cuSPARSE library version (`major * 1000 + minor * 100 + patch`), GIL-free.
///
/// Uses `cusparseGetProperty`, so no CUDA context is created or made current.
#[pyfunction]
#[pyo3(name = "version")]
fn cusparse_version(py: Python<'_>) -> PyResult<i32> {
    let t0 = Instant::now();
    let ver = py
        .detach(|| -> cusparse::Result<i32> {
            cusparse::sys::load().map_err(|e| cusparse::Error::Unavailable(e.to_string()))?;
            let property = |kind: cusparse::sys::libraryPropertyType| {
                let mut value = 0;
                // SAFETY: `value` is a valid out-pointer; the property query needs no handle.
                let code = unsafe { cusparse::sys::cusparseGetProperty(kind, &mut value) };
                check_cusparse_status("cusparseGetProperty", code).map(|()| value)
            };
            let major = property(cusparse::sys::libraryPropertyType_t_MAJOR_VERSION)?;
            let minor = property(cusparse::sys::libraryPropertyType_t_MINOR_VERSION)?;
            let patch = property(cusparse::sys::libraryPropertyType_t_PATCH_LEVEL)?;
            Ok(major * 1000 + minor * 100 + patch)
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
    state: Arc<Mutex<Option<CusparseState>>>,
    /// The handle's context; `set_stream` only accepts streams of it, so it never changes.
    ctx: Arc<Context>,
    device_id: i32,
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
            state: Arc::new(Mutex::new(Some(state))),
            ctx: stream.inner.context().clone(),
            device_id: stream.device_id,
        })
    }

    /// Rebinds this cuSPARSE handle to `stream`.
    fn set_stream(&self, py: Python<'_>, stream: &PyStream) -> PyResult<()> {
        let t0 = Instant::now();
        let state_arc = self.state.clone();
        let s = stream.clone();
        py.detach(move || -> PyResult<()> {
            let mut slot = state_arc
                .lock()
                .map_err(|_| poisoned("cusparse", "set_stream"))?;
            let guard = open(&mut slot, "cusparse", "set_stream")?;
            guard
                .handle
                .set_stream(&s.inner)
                .map_err(|e| map_cusparse_err("set_stream", e))?;
            if guard.stream.inner.cu_stream() != s.inner.cu_stream() {
                guard.scratch = CusparseScratch::new(&s.inner);
            }
            guard.stream = s;
            Ok(())
        })?;

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
        py.detach(move || -> PyResult<i32> {
            let mut slot = state_arc
                .lock()
                .map_err(|_| poisoned("cusparse", "version"))?;
            let guard = open(&mut slot, "cusparse", "version")?;
            guard
                .handle
                .version()
                .map_err(|e| map_cusparse_err("version", e))
        })
    }

    /// Executes CSR single-precision SpMV: `y = alpha * op(A) * x + beta * y`, GIL-free.
    ///
    /// `row_offsets`/`col_ind` are `i32`, `values`/`x`/`y` are `f32`; `x` and `y` may be longer
    /// than `op(A)` needs (only the leading elements are used).
    #[pyo3(signature = (row_offsets, col_ind, values, x, y, rows, cols, nnz, alpha=1.0, beta=0.0, transpose=false))]
    #[expect(
        clippy::too_many_arguments,
        reason = "CSR SpMV signature requires CSR buffers, x/y vectors, dimensions, scalars, and transpose flag"
    )]
    fn spmv_f32(
        &self,
        py: Python<'_>,
        row_offsets: &PyDeviceBuffer,
        col_ind: &PyDeviceBuffer,
        values: &PyDeviceBuffer,
        x: &PyDeviceBuffer,
        y: &PyDeviceBuffer,
        rows: i64,
        cols: i64,
        nnz: i64,
        alpha: f32,
        beta: f32,
        transpose: bool,
    ) -> PyResult<()> {
        let invalid =
            |msg: &str| record_and_err("cusparse", "spmv_f32", msg, ErrorKind::InvalidArgument);
        if !alpha.is_finite() || !beta.is_finite() {
            return Err(invalid("alpha and beta must be finite"));
        }
        let to_len = |v: i64| {
            usize::try_from(v).map_err(|_| invalid("CSR rows, cols, and nnz must be non-negative"))
        };
        let (n_rows, n_cols, n_nnz) = (to_len(rows)?, to_len(cols)?, to_len(nnz)?);
        let row_len = n_rows
            .checked_add(1)
            .ok_or_else(|| invalid("CSR row count overflow"))?;
        let (op, x_len, y_len) = if transpose {
            (CusparseOp::Transpose, n_rows, n_cols)
        } else {
            (CusparseOp::NonTranspose, n_cols, n_rows)
        };

        let ctx = &self.ctx;
        let (cat, op_name) = ("cusparse", "spmv_f32");
        check_buf(
            cat,
            op_name,
            "row_offsets",
            row_offsets,
            &["i32"],
            row_len,
            ctx,
        )?;
        check_buf(cat, op_name, "col_ind", col_ind, &["i32"], n_nnz, ctx)?;
        check_buf(cat, op_name, "values", values, &["f32"], n_nnz, ctx)?;
        check_buf(cat, op_name, "x", x, &["f32"], x_len, ctx)?;
        check_buf(cat, op_name, "y", y, &["f32"], y_len, ctx)?;
        let row_v = typed_view::<i32>(cat, op_name, "row_offsets", row_offsets, row_len)?;
        let col_v = typed_view::<i32>(cat, op_name, "col_ind", col_ind, n_nnz)?;
        let val_v = typed_view::<f32>(cat, op_name, "values", values, n_nnz)?;
        // Exactly the lengths `op(A)` needs: the wrapper requires vector length == op(A) extent.
        let x_v = typed_view::<f32>(cat, op_name, "x", x, x_len)?;
        let y_v = typed_view::<f32>(cat, op_name, "y", y, y_len)?;

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> PyResult<()> {
            let map = |e| map_cusparse_err("spmv_f32", e);
            let mut slot = state_arc.lock().map_err(|_| poisoned("cusparse", "spmv_f32"))?;
            let guard = open(&mut slot, "cusparse", "spmv_f32")?;
            let CusparseState {
                handle,
                scratch,
                stream,
            } = &mut *guard;
            let stream = &stream.inner;
            let cuda = handle.cuda().clone();
            // SAFETY: the views share the Python buffers' storage; the descriptors are used only
            // by the SpMV below, whose leases keep that storage allocated until it completes, and
            // are dropped at the end of this closure.
            let mat = unsafe {
                SpMatDescr::csr(
                    &cuda,
                    rows,
                    cols,
                    nnz,
                    &row_v,
                    &col_v,
                    &val_v,
                    IndexBase::Zero,
                )
            }
            .map_err(map)?;
            // SAFETY: as for `mat`.
            let vec_x = unsafe { DnVecDescr::new(&cuda, &x_v) }.map_err(map)?;
            // SAFETY: as for `mat`.
            let mut vec_y = unsafe { DnVecDescr::new(&cuda, &y_v) }.map_err(map)?;
            enqueue_leased(
                "cusparse",
                "spmv_f32",
                stream,
                [
                    (row_offsets, Access::Read),
                    (col_ind, Access::Read),
                    (values, Access::Read),
                    (x, Access::Read),
                    (y, Access::Write),
                ],
                || {
                    cusparse::spmv(
                        handle,
                        stream,
                        op,
                        &alpha,
                        &mat,
                        &vec_x,
                        &beta,
                        &mut vec_y,
                        CusparseDataType::F32,
                        SpMvAlg::Default,
                        scratch,
                        usize::MAX,
                    )
                    .map_err(map)
                },
            )
        })?;

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

    /// Releases the native cuSPARSE handle now instead of at garbage collection.
    ///
    /// Waits for in-flight calls and for work queued on the bound stream, then frees the native
    /// objects. Idempotent; every later call on this object raises `NvInvalidArgumentError`.
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        close_slot(py, &self.state, |st| drain_stream("cusparse", &st.stream.inner))
    }

    /// `True` once `close()` has released the native cuSPARSE handle.
    #[getter]
    fn is_closed(&self) -> bool {
        is_closed(&self.state)
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        if is_closed(&slf.state) {
            return Err(closed_err("cusparse", "__enter__"));
        }
        Ok(slf)
    }

    /// Closes the object; exceptions raised in the `with` body are not suppressed.
    #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
    fn __exit__(
        &self,
        py: Python<'_>,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<bool> {
        let _ = (exc_type, exc_val, exc_tb);
        self.close(py)?;
        Ok(false)
    }

    fn __repr__(&self) -> String {
        format!("CusparseHandle(device_id={})", self.device_id)
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
    state: Arc<Mutex<Option<CusolverDnState>>>,
    /// The handle's context; `set_stream` only accepts streams of it, so it never changes.
    ctx: Arc<Context>,
    device_id: i32,
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
            state: Arc::new(Mutex::new(Some(state))),
            ctx: stream.inner.context().clone(),
            device_id: stream.device_id,
        })
    }

    /// Rebinds this dense cuSOLVER handle to `stream`.
    fn set_stream(&self, py: Python<'_>, stream: &PyStream) -> PyResult<()> {
        let t0 = Instant::now();
        let state_arc = self.state.clone();
        let s = stream.clone();
        py.detach(move || -> PyResult<()> {
            let mut slot = state_arc
                .lock()
                .map_err(|_| poisoned("cusolver", "set_stream"))?;
            let guard = open(&mut slot, "cusolver", "set_stream")?;
            guard
                .handle
                .set_stream(&s.inner)
                .map_err(|e| map_cusolver_err("set_stream", e))?;
            if guard.stream.inner.cu_stream() != s.inner.cu_stream() {
                guard.scratch = CusolverScratch::new(&s.inner);
            }
            guard.stream = s;
            Ok(())
        })?;

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
    ///
    /// `a` is `f32`; `ipiv` and `dev_info` are `i32`.
    #[pyo3(signature = (a, ipiv, dev_info, m, n, lda=None))]
    #[expect(
        clippy::too_many_arguments,
        reason = "cuSOLVER getrf requires A, ipiv, dev_info buffers and matrix dimensions"
    )]
    fn getrf_f32(
        &self,
        py: Python<'_>,
        a: &PyDeviceBuffer,
        ipiv: &PyDeviceBuffer,
        dev_info: &PyDeviceBuffer,
        m: i32,
        n: i32,
        lda: Option<i32>,
    ) -> PyResult<()> {
        let lda = lda.unwrap_or_else(|| m.max(1));
        let (cat, op) = ("cusolver", "getrf_f32");
        check_buf(cat, op, "a", a, &["f32"], 0, &self.ctx)?;
        check_buf(cat, op, "ipiv", ipiv, &["i32"], 0, &self.ctx)?;
        check_buf(cat, op, "dev_info", dev_info, &["i32"], 0, &self.ctx)?;
        validate_getrf(m, n, lda, a.numel(), ipiv.numel(), dev_info.numel())
            .map_err(|e| map_cusolver_err(op, e))?;
        let mut a_v = typed_view::<f32>(cat, op, "a", a, a.numel())?;
        let mut ipiv_v = typed_view::<i32>(cat, op, "ipiv", ipiv, ipiv.numel())?;
        let mut info_v = typed_view::<i32>(cat, op, "dev_info", dev_info, dev_info.numel())?;

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> PyResult<()> {
            let mut slot = state_arc.lock().map_err(|_| poisoned(cat, op))?;
            let guard = open(&mut slot, cat, op)?;
            let CusolverDnState {
                handle,
                scratch,
                stream,
            } = &mut *guard;
            let stream = &stream.inner;
            enqueue_leased(
                cat,
                op,
                stream,
                [
                    (a, Access::Write),
                    (ipiv, Access::Write),
                    (dev_info, Access::Write),
                ],
                || {
                    handle
                        .getrf(
                            stream,
                            m,
                            n,
                            &mut a_v,
                            lda,
                            &mut ipiv_v,
                            &mut info_v,
                            scratch,
                            usize::MAX,
                        )
                        .map_err(|e| map_cusolver_err(op, e))
                },
            )
        })?;

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
        a: &PyDeviceBuffer,
        ipiv: &PyDeviceBuffer,
        b: &PyDeviceBuffer,
        dev_info: &PyDeviceBuffer,
        n: i32,
        nrhs: i32,
        lda: Option<i32>,
        ldb: Option<i32>,
        trans: bool,
    ) -> PyResult<()> {
        let lda = lda.unwrap_or_else(|| n.max(1));
        let ldb = ldb.unwrap_or_else(|| n.max(1));
        let (cat, op) = ("cusolver", "getrs_f32");
        check_buf(cat, op, "a", a, &["f32"], 0, &self.ctx)?;
        check_buf(cat, op, "ipiv", ipiv, &["i32"], 0, &self.ctx)?;
        check_buf(cat, op, "b", b, &["f32"], 0, &self.ctx)?;
        check_buf(cat, op, "dev_info", dev_info, &["i32"], 0, &self.ctx)?;
        validate_getrs(
            n,
            nrhs,
            lda,
            ldb,
            a.numel(),
            ipiv.numel(),
            b.numel(),
            dev_info.numel(),
        )
        .map_err(|e| map_cusolver_err(op, e))?;
        let a_v = typed_view::<f32>(cat, op, "a", a, a.numel())?;
        let ipiv_v = typed_view::<i32>(cat, op, "ipiv", ipiv, ipiv.numel())?;
        let mut b_v = typed_view::<f32>(cat, op, "b", b, b.numel())?;
        let mut info_v = typed_view::<i32>(cat, op, "dev_info", dev_info, dev_info.numel())?;
        let trans_op = if trans {
            CusolverOp::Transpose
        } else {
            CusolverOp::NonTranspose
        };

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> PyResult<()> {
            let mut slot = state_arc.lock().map_err(|_| poisoned(cat, op))?;
            let guard = open(&mut slot, cat, op)?;
            let stream = &guard.stream.inner;
            enqueue_leased(
                cat,
                op,
                stream,
                [
                    (a, Access::Read),
                    (ipiv, Access::Read),
                    (b, Access::Write),
                    (dev_info, Access::Write),
                ],
                || {
                    guard
                        .handle
                        .getrs(
                            stream,
                            trans_op,
                            n,
                            nrhs,
                            &a_v,
                            lda,
                            &ipiv_v,
                            &mut b_v,
                            ldb,
                            &mut info_v,
                        )
                        .map_err(|e| map_cusolver_err(op, e))
                },
            )
        })?;

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
        a: &PyDeviceBuffer,
        dev_info: &PyDeviceBuffer,
        n: i32,
        lda: Option<i32>,
        upper: bool,
    ) -> PyResult<()> {
        let lda = lda.unwrap_or_else(|| n.max(1));
        let (cat, op) = ("cusolver", "potrf_f32");
        check_buf(cat, op, "a", a, &["f32"], 0, &self.ctx)?;
        check_buf(cat, op, "dev_info", dev_info, &["i32"], 0, &self.ctx)?;
        validate_potrf(n, lda, a.numel(), dev_info.numel()).map_err(|e| map_cusolver_err(op, e))?;
        let mut a_v = typed_view::<f32>(cat, op, "a", a, a.numel())?;
        let mut info_v = typed_view::<i32>(cat, op, "dev_info", dev_info, dev_info.numel())?;
        let uplo = if upper {
            CusolverFillMode::Upper
        } else {
            CusolverFillMode::Lower
        };

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> PyResult<()> {
            let mut slot = state_arc.lock().map_err(|_| poisoned(cat, op))?;
            let guard = open(&mut slot, cat, op)?;
            let CusolverDnState {
                handle,
                scratch,
                stream,
            } = &mut *guard;
            let stream = &stream.inner;
            enqueue_leased(
                cat,
                op,
                stream,
                [(a, Access::Write), (dev_info, Access::Write)],
                || {
                    handle
                        .potrf(
                            stream,
                            uplo,
                            n,
                            &mut a_v,
                            lda,
                            &mut info_v,
                            scratch,
                            usize::MAX,
                        )
                        .map_err(|e| map_cusolver_err(op, e))
                },
            )
        })?;

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
        a: &PyDeviceBuffer,
        b: &PyDeviceBuffer,
        dev_info: &PyDeviceBuffer,
        n: i32,
        nrhs: i32,
        lda: Option<i32>,
        ldb: Option<i32>,
        upper: bool,
    ) -> PyResult<()> {
        let lda = lda.unwrap_or_else(|| n.max(1));
        let ldb = ldb.unwrap_or_else(|| n.max(1));
        let (cat, op) = ("cusolver", "potrs_f32");
        check_buf(cat, op, "a", a, &["f32"], 0, &self.ctx)?;
        check_buf(cat, op, "b", b, &["f32"], 0, &self.ctx)?;
        check_buf(cat, op, "dev_info", dev_info, &["i32"], 0, &self.ctx)?;
        validate_potrs(n, nrhs, lda, ldb, a.numel(), b.numel(), dev_info.numel())
            .map_err(|e| map_cusolver_err(op, e))?;
        let a_v = typed_view::<f32>(cat, op, "a", a, a.numel())?;
        let mut b_v = typed_view::<f32>(cat, op, "b", b, b.numel())?;
        let mut info_v = typed_view::<i32>(cat, op, "dev_info", dev_info, dev_info.numel())?;
        let uplo = if upper {
            CusolverFillMode::Upper
        } else {
            CusolverFillMode::Lower
        };

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> PyResult<()> {
            let mut slot = state_arc.lock().map_err(|_| poisoned(cat, op))?;
            let guard = open(&mut slot, cat, op)?;
            let stream = &guard.stream.inner;
            enqueue_leased(
                cat,
                op,
                stream,
                [
                    (a, Access::Read),
                    (b, Access::Write),
                    (dev_info, Access::Write),
                ],
                || {
                    guard
                        .handle
                        .potrs(stream, uplo, n, nrhs, &a_v, lda, &mut b_v, ldb, &mut info_v)
                        .map_err(|e| map_cusolver_err(op, e))
                },
            )
        })?;

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

    /// Releases the native cuSOLVER handle now instead of at garbage collection.
    ///
    /// Waits for in-flight calls and for work queued on the bound stream, then frees the native
    /// objects. Idempotent; every later call on this object raises `NvInvalidArgumentError`.
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        close_slot(py, &self.state, |st| drain_stream("cusolver", &st.stream.inner))
    }

    /// `True` once `close()` has released the native cuSOLVER handle.
    #[getter]
    fn is_closed(&self) -> bool {
        is_closed(&self.state)
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        if is_closed(&slf.state) {
            return Err(closed_err("cusolver", "__enter__"));
        }
        Ok(slf)
    }

    /// Closes the object; exceptions raised in the `with` body are not suppressed.
    #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
    fn __exit__(
        &self,
        py: Python<'_>,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<bool> {
        let _ = (exc_type, exc_val, exc_tb);
        self.close(py)?;
        Ok(false)
    }

    fn __repr__(&self) -> String {
        format!("CusolverDnHandle(device_id={})", self.device_id)
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

/// Resolves the `coef` of `cudnnSetActivationDescriptor` for `mode`.
///
/// `coef` is the clipping threshold of `clipped_relu` (no neutral default: `0` would clip every
/// output to zero, so it is required) and the `alpha` of `elu` (default `1.0`, standard ELU); the
/// other modes ignore it.
fn activation_coef(mode: ActivationMode, coef: Option<f64>) -> Result<f64, &'static str> {
    let coef = match (mode, coef) {
        (ActivationMode::ClippedRelu, None) => {
            return Err("clipped_relu requires coef (the clipping threshold)");
        }
        (ActivationMode::Elu, None) => 1.0,
        (_, Some(c)) => c,
        (_, None) => 0.0,
    };
    if coef.is_finite() {
        Ok(coef)
    } else {
        Err("coef must be finite")
    }
}

/// Element count of a non-empty 4D NCHW tensor.
fn tensor_4d_elems(op: &str, shape: [usize; 4]) -> PyResult<usize> {
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
        .try_fold(1usize, |acc, &dim| acc.checked_mul(dim))
        .ok_or_else(|| {
            record_and_err(
                "cudnn",
                op,
                "tensor element count overflow",
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
    state: Arc<Mutex<Option<CudnnState>>>,
    /// The handle's context; `set_stream` only accepts streams of it, so it never changes.
    ctx: Arc<Context>,
    device_id: i32,
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
            state: Arc::new(Mutex::new(Some(state))),
            ctx: stream.inner.context().clone(),
            device_id: stream.device_id,
        })
    }

    /// Rebinds this cuDNN handle to `stream`.
    fn set_stream(&self, py: Python<'_>, stream: &PyStream) -> PyResult<()> {
        let t0 = Instant::now();
        let state_arc = self.state.clone();
        let s = stream.clone();
        py.detach(move || -> PyResult<()> {
            let mut slot = state_arc
                .lock()
                .map_err(|_| poisoned("cudnn", "set_stream"))?;
            let guard = open(&mut slot, "cudnn", "set_stream")?;
            guard
                .handle
                .set_stream(&s.inner)
                .map_err(|e| map_cudnn_err("set_stream", e))?;
            guard.stream = s;
            Ok(())
        })?;

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
    ///
    /// `coef` is the clipping threshold of `clipped_relu` (required) and the `alpha` of `elu`
    /// (default `1.0`); the other modes ignore it.
    #[pyo3(signature = (x, y, shape, mode="relu", alpha=1.0, beta=0.0, coef=None))]
    #[expect(
        clippy::too_many_arguments,
        reason = "cuDNN activation forward requires x/y buffers, NCHW shape, mode, alpha/beta, and coef"
    )]
    fn activation_forward_f32(
        &self,
        py: Python<'_>,
        x: &PyDeviceBuffer,
        y: &PyDeviceBuffer,
        shape: [usize; 4],
        mode: &str,
        alpha: f32,
        beta: f32,
        coef: Option<f64>,
    ) -> PyResult<()> {
        let op = "activation_forward_f32";
        if !alpha.is_finite() || !beta.is_finite() {
            return Err(record_and_err(
                "cudnn",
                op,
                "alpha and beta must be finite",
                ErrorKind::InvalidArgument,
            ));
        }
        let act_mode = parse_activation_mode(mode)?;
        let coef = activation_coef(act_mode, coef)
            .map_err(|msg| record_and_err("cudnn", op, msg, ErrorKind::InvalidArgument))?;
        let elems = tensor_4d_elems(op, shape)?;
        check_buf("cudnn", op, "x", x, &["f32"], elems, &self.ctx)?;
        check_buf("cudnn", op, "y", y, &["f32"], elems, &self.ctx)?;
        let xv = typed_view::<f32>("cudnn", op, "x", x, elems)?;
        let mut yv = typed_view::<f32>("cudnn", op, "y", y, elems)?;

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> PyResult<()> {
            let map = |e| map_cudnn_err(op, e);
            let mut slot = state_arc.lock().map_err(|_| poisoned("cudnn", op))?;
            let guard = open(&mut slot, "cudnn", op)?;
            let ctx = guard.handle.context().clone();
            let stream = guard.stream.inner.clone();
            guard.handle.set_stream(&stream).map_err(map)?;

            let desc =
                TensorDescriptor::new_4d(&ctx, TensorFormat::Nchw, CudnnDataType::F32, shape)
                    .map_err(map)?;
            let act_desc =
                ActivationDescriptor::new(&ctx, act_mode, NanPropagation::NotPropagate, coef)
                    .map_err(map)?;

            enqueue_leased(
                "cudnn",
                op,
                &stream,
                [(x, Access::Read), (y, Access::Write)],
                || {
                    cudnn::activation_forward(
                        &mut guard.handle,
                        &stream,
                        &act_desc,
                        f64::from(alpha),
                        &desc,
                        &xv,
                        f64::from(beta),
                        &desc,
                        &mut yv,
                    )
                    .map_err(map)
                },
            )
        })?;

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
        x: &PyDeviceBuffer,
        y: &PyDeviceBuffer,
        shape: [usize; 4],
        algo: &str,
        mode: &str,
        alpha: f32,
        beta: f32,
    ) -> PyResult<()> {
        let op = "softmax_forward_f32";
        if !alpha.is_finite() || !beta.is_finite() {
            return Err(record_and_err(
                "cudnn",
                op,
                "alpha and beta must be finite",
                ErrorKind::InvalidArgument,
            ));
        }
        let sm_algo = parse_softmax_algo(algo)?;
        let sm_mode = parse_softmax_mode(mode)?;
        let elems = tensor_4d_elems(op, shape)?;
        check_buf("cudnn", op, "x", x, &["f32"], elems, &self.ctx)?;
        check_buf("cudnn", op, "y", y, &["f32"], elems, &self.ctx)?;
        let xv = typed_view::<f32>("cudnn", op, "x", x, elems)?;
        let mut yv = typed_view::<f32>("cudnn", op, "y", y, elems)?;

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> PyResult<()> {
            let map = |e| map_cudnn_err(op, e);
            let mut slot = state_arc.lock().map_err(|_| poisoned("cudnn", op))?;
            let guard = open(&mut slot, "cudnn", op)?;
            let ctx = guard.handle.context().clone();
            let stream = guard.stream.inner.clone();
            guard.handle.set_stream(&stream).map_err(map)?;

            let desc =
                TensorDescriptor::new_4d(&ctx, TensorFormat::Nchw, CudnnDataType::F32, shape)
                    .map_err(map)?;

            enqueue_leased(
                "cudnn",
                op,
                &stream,
                [(x, Access::Read), (y, Access::Write)],
                || {
                    cudnn::softmax_forward(
                        &mut guard.handle,
                        &stream,
                        sm_algo,
                        sm_mode,
                        f64::from(alpha),
                        &desc,
                        &xv,
                        f64::from(beta),
                        &desc,
                        &mut yv,
                    )
                    .map_err(map)
                },
            )
        })?;

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

    /// Releases the native cuDNN handle now instead of at garbage collection.
    ///
    /// Waits for in-flight calls and for work queued on the bound stream, then frees the native
    /// objects. Idempotent; every later call on this object raises `NvInvalidArgumentError`.
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        close_slot(py, &self.state, |st| drain_stream("cudnn", &st.stream.inner))
    }

    /// `True` once `close()` has released the native cuDNN handle.
    #[getter]
    fn is_closed(&self) -> bool {
        is_closed(&self.state)
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        if is_closed(&slf.state) {
            return Err(closed_err("cudnn", "__enter__"));
        }
        Ok(slf)
    }

    /// Closes the object; exceptions raised in the `with` body are not suppressed.
    #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
    fn __exit__(
        &self,
        py: Python<'_>,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<bool> {
        let _ = (exc_type, exc_val, exc_tb);
        self.close(py)?;
        Ok(false)
    }

    fn __repr__(&self) -> String {
        format!("CudnnHandle(device_id={})", self.device_id)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activation_coef_defaults_per_mode() {
        assert!(activation_coef(ActivationMode::ClippedRelu, None).is_err());
        assert_eq!(
            activation_coef(ActivationMode::ClippedRelu, Some(6.0)),
            Ok(6.0)
        );
        assert_eq!(activation_coef(ActivationMode::Elu, None), Ok(1.0));
        assert_eq!(activation_coef(ActivationMode::Elu, Some(0.5)), Ok(0.5));
        assert_eq!(activation_coef(ActivationMode::Relu, None), Ok(0.0));
        assert!(activation_coef(ActivationMode::Relu, Some(f64::NAN)).is_err());
    }
}
