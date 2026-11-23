//! `nv_rs.cublas`, `nv_rs.cufft`, and `nv_rs.curand` — GIL-free GPU compute bindings over zero-copy `DeviceBuffer`.
//!
//! **What.** `CublasHandle.gemm_f32`, `CufftPlan1d.execute` and `CurandGenerator.fill_*` on
//! Python `DeviceBuffer`s (owned or DLPack imports).
//! **How.** Every call validates each buffer with `buf_util::check_buf`, builds a typed
//! `cuda_core::DeviceBuffer<T>` view over the buffer's storage ([`typed_view`], no copy) and calls
//! the safe wrapper crate (`cublas::Handle::sgemm`, `cufft::Plan::exec_*`,
//! `curand::Generator::generate_*`), so the wrappers' validation (dimensions, transform type,
//! workspace lock across exec, context bind, pointer mode, even-count rule) is the only codepath.
//! The wrapper call runs inside `buf_util::enqueue_leased`, which keeps the Python buffers alive
//! until the queued work completes.
//! **Also here** [`typed_view`] (used by `sparse_ai.rs`, `nvjpeg.rs`, `nvcomp.rs`) and
//! [`poisoned`] (used by `sparse_ai.rs`); both belong next to `check_buf` in `buf_util.rs`, which
//! this wave does not edit.

use std::sync::{Arc, Mutex};
use std::time::Instant;

use cublas::{Handle as CublasInnerHandle, Operation};
use cuda_core::{Context, DeviceBuffer as CudaDeviceBuffer};
use cufft::{Complex32, Direction, FftType, Plan as CufftPlan, Scratch as CufftScratch};
use curand::{Generator as CurandInnerGenerator, RngType};
use nv_loader::diag::{self, Level};
use pyo3::prelude::*;

use crate::buf_util::{Access, check_buf, enqueue_leased};
use crate::core::{PyDeviceBuffer, PyStream};
use crate::errors::{ErrorKind, record_and_err};

// ---------------------------------------------------------------------------
// Typed views over Python buffers
// ---------------------------------------------------------------------------

/// Element types for which every bit pattern is a valid value, so a typed view over arbitrary
/// device bytes cannot produce an invalid `T` (`cuda_core::DeviceBuffer::from_storage` contract).
///
/// # Safety
/// Implementors must accept every bit pattern of `size_of::<Self>()` bytes.
pub(crate) unsafe trait AnyBits: Sized {}
// SAFETY: every bit pattern is a valid `u8`.
unsafe impl AnyBits for u8 {}
// SAFETY: every bit pattern is a valid `i32`.
unsafe impl AnyBits for i32 {}
// SAFETY: every bit pattern is a valid `f32` (NaN payloads included).
unsafe impl AnyBits for f32 {}
// SAFETY: `Complex32` is `#[repr(C, align(8))]` over `[f32; 2]`, valid for every bit pattern.
unsafe impl AnyBits for Complex32 {}

/// Typed view of the first `len` elements of `buf`, sharing its allocation owner and access
/// tracker (no copy), so a safe wrapper crate can be called on a Python buffer.
///
/// Call [`check_buf`] first (dtype, layout, context). Fails with `NvInvalidArgumentError` when
/// `buf` holds fewer than `len * size_of::<T>()` bytes or is not aligned for `T`.
pub(crate) fn typed_view<T: AnyBits>(
    category: &str,
    op: &str,
    name: &str,
    buf: &PyDeviceBuffer,
    len: usize,
) -> PyResult<CudaDeviceBuffer<T>> {
    let invalid = |msg: String| record_and_err(category, op, msg, ErrorKind::InvalidArgument);
    let bytes = len
        .checked_mul(size_of::<T>())
        .ok_or_else(|| invalid(format!("{name} byte size overflow")))?;
    let storage = buf.storage().slice_bytes(0, bytes).map_err(|e| {
        invalid(format!(
            "{name} holds {} bytes, need {bytes} ({e})",
            buf.byte_len()
        ))
    })?;
    // SAFETY: `T: AnyBits` accepts every bit pattern, so bytes written through any alias of this
    // storage (including uninitialized device memory) are valid `T`s; the view only lends the span
    // to one wrapper call whose accesses are covered by `enqueue_leased`.
    unsafe { CudaDeviceBuffer::<T>::from_storage(storage, len) }.map_err(|e| {
        invalid(format!(
            "{name} is not aligned to {} bytes ({e})",
            align_of::<T>()
        ))
    })
}

/// Error for a handle whose mutex was poisoned by a panic in an earlier call.
pub(crate) fn poisoned(category: &str, op: &str) -> PyErr {
    record_and_err(
        category,
        op,
        format!("{category} handle mutex poisoned"),
        ErrorKind::InvalidArgument,
    )
}

// ---------------------------------------------------------------------------
// nv_rs.cublas
// ---------------------------------------------------------------------------

py_err_mapper!(map_cublas_err, "cublas", cublas::Error, {
    cublas::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    cublas::Error::Invalid(_) | cublas::Error::Budget { .. } => ErrorKind::InvalidArgument,
    cublas::Error::Cublas { .. } | cublas::Error::Driver(_) => ErrorKind::Cuda,
});

struct CublasState {
    handle: CublasInnerHandle,
    stream: PyStream,
}

/// Owning cuBLAS handle bound to a CUDA stream.
#[pyclass(name = "CublasHandle", module = "nv_rs.cublas")]
pub struct PyCublasHandle {
    state: Arc<Mutex<CublasState>>,
    /// The handle's context; `set_stream` only accepts streams of it, so it never changes.
    ctx: Arc<Context>,
    device_id: i32,
}

#[pymethods]
impl PyCublasHandle {
    /// Creates a new cuBLAS handle bound to `stream`, GIL-free.
    #[new]
    fn new(py: Python<'_>, stream: &PyStream) -> PyResult<Self> {
        let t0 = Instant::now();
        let s = stream.clone();
        let state = py
            .detach(move || -> cublas::Result<CublasState> {
                let handle = CublasInnerHandle::new(s.inner.context())?;
                handle.set_stream(&s.inner)?;
                Ok(CublasState { handle, stream: s })
            })
            .map_err(|e| map_cublas_err("CublasHandle::new", e))?;

        diag::record_event(
            Level::Info,
            "cublas",
            "handle_new",
            format!("device={}", stream.device_id),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(Self {
            state: Arc::new(Mutex::new(state)),
            ctx: stream.inner.context().clone(),
            device_id: stream.device_id,
        })
    }

    /// Rebinds this cuBLAS handle to `stream`.
    fn set_stream(&self, py: Python<'_>, stream: &PyStream) -> PyResult<()> {
        let t0 = Instant::now();
        let state_arc = self.state.clone();
        let s = stream.clone();
        py.detach(move || -> cublas::Result<()> {
            let mut guard = state_arc
                .lock()
                .map_err(|_| cublas::Error::Invalid("cublas handle mutex poisoned"))?;
            guard.handle.set_stream(&s.inner)?;
            guard.stream = s;
            Ok(())
        })
        .map_err(|e| map_cublas_err("set_stream", e))?;

        diag::record_event(
            Level::Trace,
            "cublas",
            "set_stream",
            format!("device={}", stream.device_id),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    /// Single-precision **column-major** GEMM: `C = alpha * op(A) * op(B) + beta * C`, GIL-free.
    ///
    /// `a` is `m x k` (`k x m` when `trans_a`), `b` is `k x n` (`n x k` when `trans_b`) and `c` is
    /// `m x n`, all column-major with leading dimension equal to their row count; the buffers'
    /// `shape` metadata is ignored. A row-major NumPy/PyTorch `(rows, cols)` array is the
    /// column-major transpose: pass `np.asfortranarray(x)` bytes, or compute
    /// `C^T = op(B)^T * op(A)^T` by swapping the operands and `m`/`n`.
    #[pyo3(signature = (a, b, c, m, n, k, alpha=1.0, beta=0.0, trans_a=false, trans_b=false))]
    #[expect(
        clippy::too_many_arguments,
        reason = "BLAS GEMM signature requires A, B, C buffers, m/n/k, alpha/beta, and transpose flags"
    )]
    fn gemm_f32(
        &self,
        py: Python<'_>,
        a: &PyDeviceBuffer,
        b: &PyDeviceBuffer,
        c: &PyDeviceBuffer,
        m: i32,
        n: i32,
        k: i32,
        alpha: f32,
        beta: f32,
        trans_a: bool,
        trans_b: bool,
    ) -> PyResult<()> {
        if m < 0 || n < 0 || k < 0 {
            return Err(record_and_err(
                "cublas",
                "gemm_f32",
                "GEMM dimensions m, n, k must be non-negative",
                ErrorKind::InvalidArgument,
            ));
        }
        if !alpha.is_finite() || !beta.is_finite() {
            return Err(record_and_err(
                "cublas",
                "gemm_f32",
                "alpha and beta must be finite",
                ErrorKind::InvalidArgument,
            ));
        }

        let op_a = if trans_a { Operation::T } else { Operation::N };
        let op_b = if trans_b { Operation::T } else { Operation::N };
        let lda = if trans_a { k.max(1) } else { m.max(1) };
        let ldb = if trans_b { n.max(1) } else { k.max(1) };
        let ldc = m.max(1);

        // Same element extents `cublas::Handle::sgemm` requires: `ld * cols` of each operand.
        let extent = |ld: i32, cols: i32, what: &str| {
            (ld as usize).checked_mul(cols as usize).ok_or_else(|| {
                record_and_err(
                    "cublas",
                    "gemm_f32",
                    format!("matrix {what} element count overflow"),
                    ErrorKind::InvalidArgument,
                )
            })
        };
        let req_a = extent(lda, if trans_a { m } else { k }, "A")?;
        let req_b = extent(ldb, if trans_b { k } else { n }, "B")?;
        let req_c = extent(ldc, n, "C")?;

        check_buf("cublas", "gemm_f32", "a", a, &["f32"], req_a, &self.ctx)?;
        check_buf("cublas", "gemm_f32", "b", b, &["f32"], req_b, &self.ctx)?;
        check_buf("cublas", "gemm_f32", "c", c, &["f32"], req_c, &self.ctx)?;
        let a_view = typed_view::<f32>("cublas", "gemm_f32", "a", a, a.numel())?;
        let b_view = typed_view::<f32>("cublas", "gemm_f32", "b", b, b.numel())?;
        let mut c_view = typed_view::<f32>("cublas", "gemm_f32", "c", c, c.numel())?;

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> PyResult<()> {
            let guard = state_arc
                .lock()
                .map_err(|_| poisoned("cublas", "gemm_f32"))?;
            let stream = &guard.stream.inner;
            enqueue_leased(
                "cublas",
                "gemm_f32",
                stream,
                [(a, Access::Read), (b, Access::Read), (c, Access::Write)],
                || {
                    guard
                        .handle
                        .sgemm(
                            stream,
                            op_a,
                            op_b,
                            m,
                            n,
                            k,
                            alpha,
                            &a_view,
                            lda,
                            &b_view,
                            ldb,
                            beta,
                            &mut c_view,
                            ldc,
                        )
                        .map_err(|e| map_cublas_err("gemm_f32", e))
                },
            )
        })?;

        diag::record_event(
            Level::Info,
            "cublas",
            "gemm_f32",
            format!("m={m} n={n} k={k} ta={trans_a} tb={trans_b}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    fn __repr__(&self) -> String {
        format!("CublasHandle(device_id={})", self.device_id)
    }
}

// ---------------------------------------------------------------------------
// nv_rs.cufft
// ---------------------------------------------------------------------------

py_err_mapper!(map_cufft_err, "cufft", cufft::Error, {
    cufft::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    cufft::Error::Invalid(_) | cufft::Error::Budget { .. } => ErrorKind::InvalidArgument,
    cufft::Error::Cufft { .. } | cufft::Error::Driver(_) => ErrorKind::Cuda,
});

py_parse_enum!(parse_fft_kind, "cufft", "parse_fft_kind", FftType, "1D FFT kind (expected 'c2c', 'r2c', or 'c2r')", {
    "c2c" => FftType::C2C,
    "r2c" => FftType::R2C,
    "c2r" => FftType::C2R,
});

py_parse_enum!(parse_fft_direction, "cufft", "parse_fft_direction", Direction, "FFT direction (expected 'forward' or 'inverse')", {
    "forward" | "fwd" => Direction::Forward,
    "inverse" | "inv" | "backward" => Direction::Inverse,
});

struct CufftState {
    plan: CufftPlan,
    /// Owner of the plan's workspace; the plan retains the buffer, this keeps the handle to it.
    _scratch: CufftScratch,
    /// The plan's workspace stream: every exec runs here (`Plan::set_stream` rejects others).
    stream: PyStream,
}

/// 1D cuFFT plan (`c2c`, `r2c`, or `c2r`) with caller-stream workspace management.
///
/// Complex data is interleaved `f32` pairs (`dtype="f32"`, two elements per complex value).
#[pyclass(name = "CufftPlan1d", module = "nv_rs.cufft")]
pub struct PyCufftPlan1d {
    state: Arc<Mutex<CufftState>>,
    ctx: Arc<Context>,
    device_id: i32,
    nx: i32,
    batch: i32,
    fft_type: FftType,
    /// Minimum `f32` element counts of the input and output buffers.
    min_in_f32: usize,
    min_out_f32: usize,
}

#[pymethods]
impl PyCufftPlan1d {
    /// Creates a 1D cuFFT plan on `stream` for `nx` points, `kind` (`"c2c"|"r2c"|"c2r"`), and `batch`.
    #[new]
    #[pyo3(signature = (stream, nx, kind="c2c", batch=1))]
    fn new(py: Python<'_>, stream: &PyStream, nx: i32, kind: &str, batch: i32) -> PyResult<Self> {
        let fft_type = parse_fft_kind(kind)?;
        if nx <= 0 || batch <= 0 {
            return Err(record_and_err(
                "cufft",
                "CufftPlan1d::new",
                "nx and batch must be positive",
                ErrorKind::InvalidArgument,
            ));
        }

        let overflow = || {
            record_and_err(
                "cufft",
                "CufftPlan1d::new",
                "FFT element count overflow",
                ErrorKind::InvalidArgument,
            )
        };
        // Real-domain values and interleaved half-spectrum floats (`2 * (nx / 2 + 1)` per batch).
        let real_f32 = (nx as usize)
            .checked_mul(batch as usize)
            .ok_or_else(overflow)?;
        let half_f32 = ((nx as usize) / 2 + 1)
            .checked_mul(2)
            .and_then(|v| v.checked_mul(batch as usize))
            .ok_or_else(overflow)?;
        let (min_in_f32, min_out_f32) = match fft_type {
            FftType::C2C => {
                let v = real_f32.checked_mul(2).ok_or_else(overflow)?;
                (v, v)
            }
            FftType::R2C => (real_f32, half_f32),
            FftType::C2R => (half_f32, real_f32),
            FftType::D2Z | FftType::Z2D | FftType::Z2Z => {
                return Err(record_and_err(
                    "cufft",
                    "CufftPlan1d::new",
                    "only single-precision 1D plans are exposed",
                    ErrorKind::InvalidArgument,
                ));
            }
        };

        let t0 = Instant::now();
        let s = stream.clone();
        let state = py
            .detach(move || -> cufft::Result<CufftState> {
                let mut scratch = CufftScratch::new(&s.inner);
                let plan = CufftPlan::plan_1d(
                    s.inner.context(),
                    nx,
                    fft_type,
                    batch,
                    usize::MAX,
                    &mut scratch,
                )?;
                plan.set_stream(&s.inner)?;
                Ok(CufftState {
                    plan,
                    _scratch: scratch,
                    stream: s,
                })
            })
            .map_err(|e| map_cufft_err("CufftPlan1d::new", e))?;

        diag::record_event(
            Level::Info,
            "cufft",
            "plan_1d",
            format!("nx={nx} batch={batch} kind={kind}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(Self {
            state: Arc::new(Mutex::new(state)),
            ctx: stream.inner.context().clone(),
            device_id: stream.device_id,
            nx,
            batch,
            fft_type,
            min_in_f32,
            min_out_f32,
        })
    }

    /// Executes the 1D FFT from `in_buf` into `out_buf` (both `f32`), GIL-free.
    ///
    /// `direction` (`"forward"` or `"inverse"`) defaults to `"forward"` for `"c2c"` plans; `"r2c"`
    /// plans are always forward and `"c2r"` plans always inverse, and a contradicting `direction`
    /// is rejected. A `"c2r"` transform overwrites `in_buf` (cuFFT semantics).
    #[pyo3(signature = (in_buf, out_buf, direction=None))]
    fn execute(
        &self,
        py: Python<'_>,
        in_buf: &PyDeviceBuffer,
        out_buf: &PyDeviceBuffer,
        direction: Option<&str>,
    ) -> PyResult<()> {
        let fft_type = self.fft_type;
        let implied = match fft_type {
            FftType::R2C => Some(Direction::Forward),
            FftType::C2R => Some(Direction::Inverse),
            _ => None,
        };
        let dir = match (implied, direction) {
            (None, d) => parse_fft_direction(d.unwrap_or("forward"))?,
            (Some(i), None) => i,
            (Some(i), Some(d)) => {
                if parse_fft_direction(d)? != i {
                    return Err(record_and_err(
                        "cufft",
                        "execute",
                        format!("a {fft_type:?} plan always runs {i:?}; got direction `{d}`"),
                        ErrorKind::InvalidArgument,
                    ));
                }
                i
            }
        };

        let (ctx, min_in, min_out) = (&self.ctx, self.min_in_f32, self.min_out_f32);
        check_buf("cufft", "execute", "in_buf", in_buf, &["f32"], min_in, ctx)?;
        check_buf(
            "cufft",
            "execute",
            "out_buf",
            out_buf,
            &["f32"],
            min_out,
            ctx,
        )?;

        /// Typed views per transform kind (complex = two interleaved `f32`).
        enum Views {
            C2c(CudaDeviceBuffer<Complex32>, CudaDeviceBuffer<Complex32>),
            R2c(CudaDeviceBuffer<f32>, CudaDeviceBuffer<Complex32>),
            C2r(CudaDeviceBuffer<Complex32>, CudaDeviceBuffer<f32>),
        }
        let complex = |name: &str, buf: &PyDeviceBuffer| {
            typed_view::<Complex32>("cufft", "execute", name, buf, buf.numel() / 2)
        };
        let real = |name: &str, buf: &PyDeviceBuffer| {
            typed_view::<f32>("cufft", "execute", name, buf, buf.numel())
        };
        let mut views = match fft_type {
            FftType::C2C => Views::C2c(complex("in_buf", in_buf)?, complex("out_buf", out_buf)?),
            FftType::R2C => Views::R2c(real("in_buf", in_buf)?, complex("out_buf", out_buf)?),
            FftType::C2R => Views::C2r(complex("in_buf", in_buf)?, real("out_buf", out_buf)?),
            FftType::D2Z | FftType::Z2D | FftType::Z2Z => {
                return Err(record_and_err(
                    "cufft",
                    "execute",
                    "unsupported 1D FFT plan type",
                    ErrorKind::InvalidArgument,
                ));
            }
        };
        // cuFFT overwrites the input of a complex-to-real transform.
        let in_access = if fft_type == FftType::C2R {
            Access::Write
        } else {
            Access::Read
        };

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> PyResult<()> {
            let guard = state_arc.lock().map_err(|_| poisoned("cufft", "execute"))?;
            let stream = &guard.stream.inner;
            enqueue_leased(
                "cufft",
                "execute",
                stream,
                [(in_buf, in_access), (out_buf, Access::Write)],
                || {
                    match &mut views {
                        Views::C2c(i, o) => guard.plan.exec_c2c(stream, i, o, dir),
                        Views::R2c(i, o) => guard.plan.exec_r2c(stream, i, o),
                        Views::C2r(i, o) => guard.plan.exec_c2r(stream, i, o),
                    }
                    .map_err(|e| map_cufft_err("execute", e))
                },
            )
        })?;

        diag::record_event(
            Level::Info,
            "cufft",
            "execute",
            format!("nx={} batch={} kind={fft_type:?}", self.nx, self.batch),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    fn __repr__(&self) -> String {
        format!(
            "CufftPlan1d(nx={}, batch={}, kind={:?}, device_id={})",
            self.nx, self.batch, self.fft_type, self.device_id
        )
    }
}

// ---------------------------------------------------------------------------
// nv_rs.curand
// ---------------------------------------------------------------------------

py_err_mapper!(map_curand_err, "curand", curand::Error, {
    curand::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    curand::Error::Invalid(_) => ErrorKind::InvalidArgument,
    curand::Error::Curand { .. } | curand::Error::Driver(_) => ErrorKind::Cuda,
});

struct CurandState {
    generator: CurandInnerGenerator,
    stream: PyStream,
}

/// Device-side cuRAND Philox4_32_10 generator bound to a CUDA stream.
#[pyclass(name = "CurandGenerator", module = "nv_rs.curand")]
pub struct PyCurandGenerator {
    state: Arc<Mutex<CurandState>>,
    ctx: Arc<Context>,
    device_id: i32,
}

#[pymethods]
impl PyCurandGenerator {
    /// Creates a device-side cuRAND generator on `stream` with optional `seed`, GIL-free.
    #[new]
    #[pyo3(signature = (stream, seed=None))]
    fn new(py: Python<'_>, stream: &PyStream, seed: Option<u64>) -> PyResult<Self> {
        let t0 = Instant::now();
        let s = stream.clone();
        let state = py
            .detach(move || -> curand::Result<CurandState> {
                let generator =
                    CurandInnerGenerator::new_device(s.inner.context(), RngType::Philox4_32_10)?;
                generator.set_stream(&s.inner)?;
                if let Some(seed_val) = seed {
                    generator.set_seed(seed_val)?;
                }
                Ok(CurandState {
                    generator,
                    stream: s,
                })
            })
            .map_err(|e| map_curand_err("CurandGenerator::new", e))?;

        diag::record_event(
            Level::Info,
            "curand",
            "generator_new",
            format!("device={} seed={seed:?}", stream.device_id),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(Self {
            state: Arc::new(Mutex::new(state)),
            ctx: stream.inner.context().clone(),
            device_id: stream.device_id,
        })
    }

    /// Fills `buf` (`f32`) with uniform values in `(0.0, 1.0]`, GIL-free.
    fn fill_uniform_f32(&self, py: Python<'_>, buf: &PyDeviceBuffer) -> PyResult<()> {
        check_buf(
            "curand",
            "fill_uniform_f32",
            "buf",
            buf,
            &["f32"],
            0,
            &self.ctx,
        )?;
        let len = buf.numel();
        let mut view = typed_view::<f32>("curand", "fill_uniform_f32", "buf", buf, len)?;

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> PyResult<()> {
            let guard = state_arc
                .lock()
                .map_err(|_| poisoned("curand", "fill_uniform_f32"))?;
            let stream = &guard.stream.inner;
            enqueue_leased(
                "curand",
                "fill_uniform_f32",
                stream,
                [(buf, Access::Write)],
                || {
                    guard
                        .generator
                        .generate_uniform_f32(stream, &mut view)
                        .map_err(|e| map_curand_err("fill_uniform_f32", e))
                },
            )
        })?;

        diag::record_event(
            Level::Info,
            "curand",
            "fill_uniform_f32",
            format!("n={len}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    /// Fills `buf` (`f32`, even element count) with normally distributed values (`mean`,
    /// `stddev`), GIL-free.
    #[pyo3(signature = (buf, mean=0.0, stddev=1.0))]
    fn fill_normal_f32(
        &self,
        py: Python<'_>,
        buf: &PyDeviceBuffer,
        mean: f32,
        stddev: f32,
    ) -> PyResult<()> {
        if !mean.is_finite() || !stddev.is_finite() || stddev <= 0.0 {
            return Err(record_and_err(
                "curand",
                "fill_normal_f32",
                "normal distribution requires finite mean and positive finite stddev",
                ErrorKind::InvalidArgument,
            ));
        }
        check_buf(
            "curand",
            "fill_normal_f32",
            "buf",
            buf,
            &["f32"],
            0,
            &self.ctx,
        )?;
        let len = buf.numel();
        if !len.is_multiple_of(2) {
            return Err(record_and_err(
                "curand",
                "fill_normal_f32",
                format!("pseudo-random normal generation requires even element count, got {len}"),
                ErrorKind::InvalidArgument,
            ));
        }
        let mut view = typed_view::<f32>("curand", "fill_normal_f32", "buf", buf, len)?;

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> PyResult<()> {
            let guard = state_arc
                .lock()
                .map_err(|_| poisoned("curand", "fill_normal_f32"))?;
            let stream = &guard.stream.inner;
            enqueue_leased(
                "curand",
                "fill_normal_f32",
                stream,
                [(buf, Access::Write)],
                || {
                    guard
                        .generator
                        .generate_normal_f32(stream, &mut view, mean, stddev)
                        .map_err(|e| map_curand_err("fill_normal_f32", e))
                },
            )
        })?;

        diag::record_event(
            Level::Info,
            "curand",
            "fill_normal_f32",
            format!("n={len} mean={mean} stddev={stddev}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    fn __repr__(&self) -> String {
        format!("CurandGenerator(device_id={})", self.device_id)
    }
}

// ---------------------------------------------------------------------------
// Module registration
// ---------------------------------------------------------------------------

/// Registers `nv_rs.cublas`, `nv_rs.cufft`, and `nv_rs.curand` submodules.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    py_register_submodule!(py, parent, "cublas", |m| {
        m.add_class::<PyCublasHandle>()?;
    })?;
    py_register_submodule!(py, parent, "cufft", |m| {
        m.add_class::<PyCufftPlan1d>()?;
    })?;
    py_register_submodule!(py, parent, "curand", |m| {
        m.add_class::<PyCurandGenerator>()?;
    })?;
    Ok(())
}
