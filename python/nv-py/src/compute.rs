//! `nv_rs.cublas`, `nv_rs.cufft`, and `nv_rs.curand` — GIL-free GPU compute bindings over zero-copy `DeviceBuffer`.

use std::sync::{Arc, Mutex};
use std::time::Instant;

use cublas::{Handle as CublasInnerHandle, Operation};
use cufft::{Complex32, Direction, FftType, Plan as CufftPlan, Scratch as CufftScratch};
use curand::{Generator as CurandInnerGenerator, RngType};
use nv_loader::diag::{self, Level};
use pyo3::prelude::*;

use crate::core::{PyDeviceBuffer, PyStream};
use crate::errors::{ErrorKind, record_and_err};

/// Extracts `(raw_device_ptr, nbytes)` from a `PyDeviceBuffer`, supporting both
/// `nv_rs`-owned buffers and borrowed DLPack tensors.
fn buf_ptr_and_bytes(buf: &Bound<'_, PyDeviceBuffer>) -> PyResult<(u64, usize)> {
    let borrowed = buf.borrow();
    let ptr = borrowed.raw_device_ptr();
    let nbytes = match borrowed.owned_cuda_buf() {
        Some(cuda_buf) => cuda_buf.len(),
        None => {
            drop(borrowed);
            buf.getattr("nbytes")?.extract::<usize>()?
        }
    };
    Ok((ptr, nbytes))
}

/// Validates that `nbytes >= min_bytes` and `ptr != 0` when `min_bytes > 0`.
fn validate_dev_buf(
    category: &str,
    op: &str,
    name: &str,
    ptr: u64,
    nbytes: usize,
    min_bytes: usize,
) -> PyResult<()> {
    if nbytes < min_bytes {
        return Err(record_and_err(
            category,
            op,
            format!("{name} buffer too small: got {nbytes} bytes, need at least {min_bytes} bytes"),
            ErrorKind::InvalidArgument,
        ));
    }
    if min_bytes > 0 && ptr == 0 {
        return Err(record_and_err(
            category,
            op,
            format!("{name} device pointer is null"),
            ErrorKind::InvalidArgument,
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// nv_rs.cublas
// ---------------------------------------------------------------------------

fn map_cublas_err(op: &str, err: cublas::Error) -> PyErr {
    let kind = match &err {
        cublas::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
        cublas::Error::Invalid(_) | cublas::Error::Budget { .. } => ErrorKind::InvalidArgument,
        cublas::Error::Cublas { .. } | cublas::Error::Driver(_) => ErrorKind::Cuda,
    };
    record_and_err("cublas", op, err, kind)
}

struct CublasState {
    handle: CublasInnerHandle,
    stream: PyStream,
}

/// Owning cuBLAS handle bound to a CUDA stream.
#[pyclass(name = "CublasHandle", module = "nv_rs.cublas")]
pub struct PyCublasHandle {
    state: Arc<Mutex<CublasState>>,
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

    /// Single-precision column-major GEMM: `C = alpha * op(A) * op(B) + beta * C`, GIL-free.
    #[pyo3(signature = (a, b, c, m, n, k, alpha=1.0, beta=0.0, trans_a=false, trans_b=false))]
    #[expect(
        clippy::too_many_arguments,
        reason = "BLAS GEMM signature requires A, B, C buffers, m/n/k, alpha/beta, and transpose flags"
    )]
    fn gemm_f32(
        &self,
        py: Python<'_>,
        a: &Bound<'_, PyDeviceBuffer>,
        b: &Bound<'_, PyDeviceBuffer>,
        c: &Bound<'_, PyDeviceBuffer>,
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

        let elem_bytes = size_of::<f32>();
        let req_a = (m as usize)
            .checked_mul(k as usize)
            .and_then(|elems| elems.checked_mul(elem_bytes))
            .ok_or_else(|| {
                record_and_err(
                    "cublas",
                    "gemm_f32",
                    "matrix A byte size overflow",
                    ErrorKind::InvalidArgument,
                )
            })?;
        let req_b = (k as usize)
            .checked_mul(n as usize)
            .and_then(|elems| elems.checked_mul(elem_bytes))
            .ok_or_else(|| {
                record_and_err(
                    "cublas",
                    "gemm_f32",
                    "matrix B byte size overflow",
                    ErrorKind::InvalidArgument,
                )
            })?;
        let req_c = (m as usize)
            .checked_mul(n as usize)
            .and_then(|elems| elems.checked_mul(elem_bytes))
            .ok_or_else(|| {
                record_and_err(
                    "cublas",
                    "gemm_f32",
                    "matrix C byte size overflow",
                    ErrorKind::InvalidArgument,
                )
            })?;

        let (a_ptr, a_bytes) = buf_ptr_and_bytes(a)?;
        let (b_ptr, b_bytes) = buf_ptr_and_bytes(b)?;
        let (c_ptr, c_bytes) = buf_ptr_and_bytes(c)?;

        validate_dev_buf("cublas", "gemm_f32", "a", a_ptr, a_bytes, req_a)?;
        validate_dev_buf("cublas", "gemm_f32", "b", b_ptr, b_bytes, req_b)?;
        validate_dev_buf("cublas", "gemm_f32", "c", c_ptr, c_bytes, req_c)?;

        let op_a = if trans_a { Operation::T } else { Operation::N };
        let op_b = if trans_b { Operation::T } else { Operation::N };
        let lda = if trans_a { k.max(1) } else { m.max(1) };
        let ldb = if trans_b { n.max(1) } else { k.max(1) };
        let ldc = m.max(1);

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> cublas::Result<()> {
            let guard = state_arc
                .lock()
                .map_err(|_| cublas::Error::Invalid("cublas handle mutex poisoned"))?;
            guard.handle.set_stream(&guard.stream.inner)?;
            // SAFETY: `guard.handle` is bound to `guard.stream`, `alpha`/`beta` are valid host
            // scalars for `PointerMode::Host`, and device pointers were validated for `req_a/b/c`.
            let code = unsafe {
                cublas::sys::cublasSgemm_v2(
                    guard.handle.raw(),
                    op_a.raw(),
                    op_b.raw(),
                    m,
                    n,
                    k,
                    &alpha,
                    a_ptr as usize as *const f32,
                    lda,
                    b_ptr as usize as *const f32,
                    ldb,
                    &beta,
                    c_ptr as usize as *mut f32,
                    ldc,
                )
            };
            let status = cublas::Status(code);
            if status.is_success() {
                Ok(())
            } else {
                Err(cublas::Error::Cublas {
                    call: "cublasSgemm_v2",
                    code: status,
                })
            }
        })
        .map_err(|e| map_cublas_err("gemm_f32", e))?;

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
        match self.state.lock() {
            Ok(g) => format!("CublasHandle(device_id={})", g.stream.device_id),
            Err(_) => "CublasHandle(<poisoned>)".to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// nv_rs.cufft
// ---------------------------------------------------------------------------

fn map_cufft_err(op: &str, err: cufft::Error) -> PyErr {
    let kind = match &err {
        cufft::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
        cufft::Error::Invalid(_) | cufft::Error::Budget { .. } => ErrorKind::InvalidArgument,
        cufft::Error::Cufft { .. } | cufft::Error::Driver(_) => ErrorKind::Cuda,
    };
    record_and_err("cufft", op, err, kind)
}

fn parse_fft_kind(kind: &str) -> PyResult<FftType> {
    match kind.trim().to_ascii_lowercase().as_str() {
        "c2c" => Ok(FftType::C2C),
        "r2c" => Ok(FftType::R2C),
        "c2r" => Ok(FftType::C2R),
        other => Err(record_and_err(
            "cufft",
            "parse_fft_kind",
            format!("unsupported 1D FFT kind `{other}`; expected 'c2c', 'r2c', or 'c2r'"),
            ErrorKind::InvalidArgument,
        )),
    }
}

fn parse_fft_direction(direction: &str) -> PyResult<Direction> {
    match direction.trim().to_ascii_lowercase().as_str() {
        "forward" | "fwd" => Ok(Direction::Forward),
        "inverse" | "inv" | "backward" => Ok(Direction::Inverse),
        other => Err(record_and_err(
            "cufft",
            "parse_fft_direction",
            format!("unsupported FFT direction `{other}`; expected 'forward' or 'inverse'"),
            ErrorKind::InvalidArgument,
        )),
    }
}

struct CufftState {
    plan: CufftPlan,
    _scratch: CufftScratch,
    stream: PyStream,
    nx: i32,
    batch: i32,
    fft_type: FftType,
    min_in_bytes: usize,
    min_out_bytes: usize,
}

/// 1D cuFFT plan (`c2c`, `r2c`, or `c2r`) with caller-stream workspace management.
#[pyclass(name = "CufftPlan1d", module = "nv_rs.cufft")]
pub struct PyCufftPlan1d {
    state: Arc<Mutex<CufftState>>,
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

        let real_elems = (nx as usize).checked_mul(batch as usize).ok_or_else(|| {
            record_and_err(
                "cufft",
                "CufftPlan1d::new",
                "FFT element count overflow",
                ErrorKind::InvalidArgument,
            )
        })?;
        let half_complex_elems = ((nx as usize) / 2 + 1)
            .checked_mul(batch as usize)
            .ok_or_else(|| {
                record_and_err(
                    "cufft",
                    "CufftPlan1d::new",
                    "FFT half-complex element count overflow",
                    ErrorKind::InvalidArgument,
                )
            })?;

        let (min_in_bytes, min_out_bytes) = match fft_type {
            FftType::C2C => {
                let b = real_elems.checked_mul(size_of::<Complex32>());
                (b, b)
            }
            FftType::R2C => (
                real_elems.checked_mul(size_of::<f32>()),
                half_complex_elems.checked_mul(size_of::<Complex32>()),
            ),
            FftType::C2R => (
                half_complex_elems.checked_mul(size_of::<Complex32>()),
                real_elems.checked_mul(size_of::<f32>()),
            ),
            _ => (None, None),
        };
        let (min_in_bytes, min_out_bytes) = match (min_in_bytes, min_out_bytes) {
            (Some(i), Some(o)) => (i, o),
            _ => {
                return Err(record_and_err(
                    "cufft",
                    "CufftPlan1d::new",
                    "FFT buffer byte size overflow",
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
                    nx,
                    batch,
                    fft_type,
                    min_in_bytes,
                    min_out_bytes,
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
        })
    }

    /// Executes the 1D FFT from `in_buf` into `out_buf`, GIL-free.
    ///
    /// `direction` (`"forward"` or `"inverse"`, default `"forward"`) applies to `"c2c"` plans.
    #[pyo3(signature = (in_buf, out_buf, direction="forward"))]
    fn execute(
        &self,
        py: Python<'_>,
        in_buf: &Bound<'_, PyDeviceBuffer>,
        out_buf: &Bound<'_, PyDeviceBuffer>,
        direction: &str,
    ) -> PyResult<()> {
        let dir = parse_fft_direction(direction)?;
        let (in_ptr, in_bytes) = buf_ptr_and_bytes(in_buf)?;
        let (out_ptr, out_bytes) = buf_ptr_and_bytes(out_buf)?;

        let state_arc = self.state.clone();
        let (min_in, min_out, nx, batch, fft_type) = {
            let guard = state_arc.lock().map_err(|_| {
                record_and_err(
                    "cufft",
                    "execute",
                    "cufft plan mutex poisoned",
                    ErrorKind::InvalidArgument,
                )
            })?;
            (
                guard.min_in_bytes,
                guard.min_out_bytes,
                guard.nx,
                guard.batch,
                guard.fft_type,
            )
        };

        validate_dev_buf("cufft", "execute", "in_buf", in_ptr, in_bytes, min_in)?;
        validate_dev_buf("cufft", "execute", "out_buf", out_ptr, out_bytes, min_out)?;

        let t0 = Instant::now();
        py.detach(move || -> cufft::Result<()> {
            let guard = state_arc
                .lock()
                .map_err(|_| cufft::Error::Invalid("cufft plan mutex poisoned"))?;
            guard.plan.set_stream(&guard.stream.inner)?;
            let (call, code) = match guard.fft_type {
                FftType::C2C => {
                    // SAFETY: `guard.plan` is a valid C2C plan bound to `guard.stream`, and
                    // `in_ptr`/`out_ptr` were validated against `min_in_bytes`/`min_out_bytes`.
                    let c = unsafe {
                        cufft::sys::cufftExecC2C(
                            guard.plan.raw(),
                            in_ptr as usize as *mut cufft::sys::cufftComplex,
                            out_ptr as usize as *mut cufft::sys::cufftComplex,
                            dir.raw(),
                        )
                    };
                    ("cufftExecC2C", c)
                }
                FftType::R2C => {
                    // SAFETY: `guard.plan` is a valid R2C plan bound to `guard.stream`, and
                    // `in_ptr`/`out_ptr` were validated against `min_in_bytes`/`min_out_bytes`.
                    let c = unsafe {
                        cufft::sys::cufftExecR2C(
                            guard.plan.raw(),
                            in_ptr as usize as *mut cufft::sys::cufftReal,
                            out_ptr as usize as *mut cufft::sys::cufftComplex,
                        )
                    };
                    ("cufftExecR2C", c)
                }
                FftType::C2R => {
                    // SAFETY: `guard.plan` is a valid C2R plan bound to `guard.stream`, and
                    // `in_ptr`/`out_ptr` were validated against `min_in_bytes`/`min_out_bytes`.
                    let c = unsafe {
                        cufft::sys::cufftExecC2R(
                            guard.plan.raw(),
                            in_ptr as usize as *mut cufft::sys::cufftComplex,
                            out_ptr as usize as *mut cufft::sys::cufftReal,
                        )
                    };
                    ("cufftExecC2R", c)
                }
                _ => return Err(cufft::Error::Invalid("unsupported 1D FFT plan type")),
            };
            let status = cufft::FftResult(code);
            if status.is_success() {
                Ok(())
            } else {
                Err(cufft::Error::Cufft { call, code: status })
            }
        })
        .map_err(|e| map_cufft_err("execute", e))?;

        diag::record_event(
            Level::Info,
            "cufft",
            "execute",
            format!("nx={nx} batch={batch} kind={fft_type:?}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    fn __repr__(&self) -> String {
        match self.state.lock() {
            Ok(g) => format!(
                "CufftPlan1d(nx={}, batch={}, kind={:?}, device_id={})",
                g.nx, g.batch, g.fft_type, g.stream.device_id
            ),
            Err(_) => "CufftPlan1d(<poisoned>)".to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// nv_rs.curand
// ---------------------------------------------------------------------------

fn map_curand_err(op: &str, err: curand::Error) -> PyErr {
    let kind = match &err {
        curand::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
        curand::Error::Invalid(_) => ErrorKind::InvalidArgument,
        curand::Error::Curand { .. } | curand::Error::Driver(_) => ErrorKind::Cuda,
    };
    record_and_err("curand", op, err, kind)
}

struct CurandState {
    generator: CurandInnerGenerator,
    stream: PyStream,
}

/// Device-side cuRAND Philox4_32_10 generator bound to a CUDA stream.
#[pyclass(name = "CurandGenerator", module = "nv_rs.curand")]
pub struct PyCurandGenerator {
    state: Arc<Mutex<CurandState>>,
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
        })
    }

    /// Fills `buf` with uniform `f32` values in `(0.0, 1.0]`, GIL-free.
    fn fill_uniform_f32(&self, py: Python<'_>, buf: &Bound<'_, PyDeviceBuffer>) -> PyResult<()> {
        let (ptr, nbytes) = buf_ptr_and_bytes(buf)?;
        if !nbytes.is_multiple_of(size_of::<f32>()) {
            return Err(record_and_err(
                "curand",
                "fill_uniform_f32",
                format!("buffer byte size ({nbytes}) must be a multiple of 4 for f32"),
                ErrorKind::InvalidArgument,
            ));
        }
        let len = nbytes / size_of::<f32>();
        validate_dev_buf("curand", "fill_uniform_f32", "buf", ptr, nbytes, nbytes)?;

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> curand::Result<()> {
            let guard = state_arc
                .lock()
                .map_err(|_| curand::Error::Invalid("curand generator mutex poisoned"))?;
            guard.generator.set_stream(&guard.stream.inner)?;
            // SAFETY: `guard.generator` is a live device generator bound to `guard.stream`, and
            // `ptr` points to `len` writable `f32` elements on device.
            let code = unsafe {
                curand::sys::curandGenerateUniform(
                    guard.generator.raw(),
                    ptr as usize as *mut f32,
                    len,
                )
            };
            let status = curand::CurandStatus(code);
            if status.is_success() {
                Ok(())
            } else {
                Err(curand::Error::Curand {
                    call: "curandGenerateUniform",
                    code: status,
                })
            }
        })
        .map_err(|e| map_curand_err("fill_uniform_f32", e))?;

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

    /// Fills `buf` with normally distributed `f32` values (`mean`, `stddev`), GIL-free.
    #[pyo3(signature = (buf, mean=0.0, stddev=1.0))]
    fn fill_normal_f32(
        &self,
        py: Python<'_>,
        buf: &Bound<'_, PyDeviceBuffer>,
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
        let (ptr, nbytes) = buf_ptr_and_bytes(buf)?;
        if !nbytes.is_multiple_of(size_of::<f32>()) {
            return Err(record_and_err(
                "curand",
                "fill_normal_f32",
                format!("buffer byte size ({nbytes}) must be a multiple of 4 for f32"),
                ErrorKind::InvalidArgument,
            ));
        }
        let len = nbytes / size_of::<f32>();
        if !len.is_multiple_of(2) {
            return Err(record_and_err(
                "curand",
                "fill_normal_f32",
                format!("pseudo-random normal generation requires even element count, got {len}"),
                ErrorKind::InvalidArgument,
            ));
        }
        validate_dev_buf("curand", "fill_normal_f32", "buf", ptr, nbytes, nbytes)?;

        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> curand::Result<()> {
            let guard = state_arc
                .lock()
                .map_err(|_| curand::Error::Invalid("curand generator mutex poisoned"))?;
            guard.generator.set_stream(&guard.stream.inner)?;
            // SAFETY: `guard.generator` is a live device generator bound to `guard.stream`, `len` is
            // even, `mean`/`stddev` are validated, and `ptr` points to `len` writable `f32`s.
            let code = unsafe {
                curand::sys::curandGenerateNormal(
                    guard.generator.raw(),
                    ptr as usize as *mut f32,
                    len,
                    mean,
                    stddev,
                )
            };
            let status = curand::CurandStatus(code);
            if status.is_success() {
                Ok(())
            } else {
                Err(curand::Error::Curand {
                    call: "curandGenerateNormal",
                    code: status,
                })
            }
        })
        .map_err(|e| map_curand_err("fill_normal_f32", e))?;

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
        match self.state.lock() {
            Ok(g) => format!("CurandGenerator(device_id={})", g.stream.device_id),
            Err(_) => "CurandGenerator(<poisoned>)".to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Module registration
// ---------------------------------------------------------------------------

/// Registers `nv_rs.cublas`, `nv_rs.cufft`, and `nv_rs.curand` submodules.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let sys_modules = py.import("sys")?.getattr("modules")?;

    let m_cublas = PyModule::new(py, "cublas")?;
    m_cublas.add_class::<PyCublasHandle>()?;
    parent.add_submodule(&m_cublas)?;
    sys_modules.set_item("nv_rs.cublas", m_cublas)?;

    let m_cufft = PyModule::new(py, "cufft")?;
    m_cufft.add_class::<PyCufftPlan1d>()?;
    parent.add_submodule(&m_cufft)?;
    sys_modules.set_item("nv_rs.cufft", m_cufft)?;

    let m_curand = PyModule::new(py, "curand")?;
    m_curand.add_class::<PyCurandGenerator>()?;
    parent.add_submodule(&m_curand)?;
    sys_modules.set_item("nv_rs.curand", m_curand)?;

    Ok(())
}
