//! `nv_rs.diag` — runtime diagnostics, environment doctor, event ring-buffer, and leak tracing.

use nv_loader::diag::{self, Level};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

fn parse_level(raw: &str) -> PyResult<Level> {
    Level::parse(raw).ok_or_else(|| {
        PyValueError::new_err(format!(
            "invalid diag level `{raw}`; expected one of: off, error, warn, info, trace"
        ))
    })
}

/// Returns the current diagnostic verbosity level (`"OFF"`, `"ERROR"`, `"WARN"`, `"INFO"`, `"TRACE"`).
#[pyfunction]
fn get_level() -> &'static str {
    diag::level().as_str()
}

/// Sets the global diagnostic verbosity level (`"off"`, `"error"`, `"warn"`, `"info"`, `"trace"`).
#[pyfunction]
fn set_level(level: &str) -> PyResult<&'static str> {
    let lvl = parse_level(level)?;
    diag::set_level(lvl);
    Ok(lvl.as_str())
}

/// Returns cumulative diagnostic counters (library loads, GPU alloc/free bytes, errors).
#[pyfunction]
fn stats(py: Python<'_>) -> PyResult<Py<PyDict>> {
    let st = diag::stats();
    let d = PyDict::new(py);
    d.set_item("lib_loads", st.lib_loads)?;
    d.set_item("lib_load_failures", st.lib_load_failures)?;
    d.set_item("gpu_alloc_count", st.gpu_alloc_count)?;
    d.set_item("gpu_free_count", st.gpu_free_count)?;
    d.set_item("gpu_alloc_bytes", st.gpu_alloc_bytes)?;
    d.set_item("gpu_live_bytes", st.gpu_live_bytes)?;
    d.set_item("gpu_peak_bytes", st.gpu_peak_bytes)?;
    d.set_item("errors", st.errors)?;
    d.set_item("last_error", st.last_error)?;
    Ok(d.unbind())
}

/// Returns recorded events from the bounded diagnostic ring-buffer (`256` max).
#[pyfunction]
fn events(py: Python<'_>) -> PyResult<Py<PyList>> {
    let list = PyList::empty(py);
    for ev in diag::events() {
        let d = PyDict::new(py);
        d.set_item("seq", ev.seq)?;
        d.set_item("timestamp_ms", ev.timestamp_ms)?;
        d.set_item("level", ev.level.as_str())?;
        d.set_item("category", ev.category)?;
        d.set_item("op", ev.op)?;
        d.set_item("detail", ev.detail)?;
        d.set_item("elapsed_ms", ev.elapsed_ns as f64 / 1_000_000.0)?;
        d.set_item("ok", ev.ok)?;
        list.append(d)?;
    }
    Ok(list.unbind())
}

/// Clears the diagnostic event ring-buffer (counters remain intact).
#[pyfunction]
fn clear_events() {
    diag::clear_events();
}

/// Registers an additional directory to search when loading NVIDIA shared libraries.
#[pyfunction]
fn add_dll_directory(path: std::path::PathBuf) {
    nv_loader::add_search_dir(&path);
}

/// Probes all known NVIDIA libraries without loading them into the process.
#[pyfunction]
fn probe_libraries(py: Python<'_>) -> PyResult<Py<PyDict>> {
    let out = PyDict::new(py);
    for res in diag::probe_all() {
        let entry = PyDict::new(py);
        entry.set_item("present", res.first_present.is_some())?;
        entry.set_item(
            "path",
            res.first_present
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned()),
        )?;
        let cands: Vec<String> = res
            .candidates
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect();
        entry.set_item("candidates", cands)?;
        out.set_item(res.lib, entry)?;
    }
    Ok(out.unbind())
}

/// Collects a full environment diagnostic report (`os`, `arch`, `env`, `libraries`, `stats`, `python_interop`).
#[pyfunction]
fn doctor(py: Python<'_>) -> PyResult<Py<PyDict>> {
    let report = PyDict::new(py);
    report.set_item("version", env!("CARGO_PKG_VERSION"))?;
    report.set_item("os", std::env::consts::OS)?;
    report.set_item("arch", std::env::consts::ARCH)?;
    report.set_item("diag_level", diag::level().as_str())?;

    let env_dict = PyDict::new(py);
    for var in [
        "CUDA_PATH",
        "NV_SDK_DIRS",
        "NV_RS_DEBUG",
        "NV_RS_LOG",
        "VULKAN_SDK",
        "OPTIX_ROOT",
        "CUDNN_ROOT",
        "NVCOMP_ROOT",
        "VIDEO_CODEC_SDK_ROOT",
        "OPTICAL_FLOW_SDK_ROOT",
        "LD_LIBRARY_PATH",
    ] {
        env_dict.set_item(var, std::env::var(var).ok())?;
    }
    report.set_item("env", env_dict)?;
    report.set_item("libraries", probe_libraries(py)?)?;
    report.set_item("stats", stats(py)?)?;

    // Check optional Python ecosystem packages without importing heavy frameworks eagerly.
    let importlib_util = py.import("importlib.util")?;
    let interop = PyDict::new(py);
    for pkg in ["numpy", "torch", "cupy", "jax"] {
        let spec = importlib_util.call_method1("find_spec", (pkg,))?;
        interop.set_item(pkg, !spec.is_none())?;
    }
    report.set_item("python_interop", interop)?;
    Ok(report.unbind())
}

/// Context manager that temporarily raises the diagnostic level and reports live GPU memory delta + events.
#[pyclass(name = "TraceGuard")]
pub struct TraceGuard {
    target_level: Level,
    prev_level: Level,
    start_live_bytes: u64,
    start_errors: u64,
    start_seq: u64,
    print_on_exit: bool,
    captured: Vec<Py<PyDict>>,
    live_bytes_delta: i64,
    error_delta: u64,
}

#[pymethods]
impl TraceGuard {
    #[new]
    #[pyo3(signature = (level="trace", print_on_exit=false))]
    fn new(level: &str, print_on_exit: bool) -> PyResult<Self> {
        let target_level = parse_level(level)?;
        Ok(Self {
            target_level,
            prev_level: diag::level(),
            start_live_bytes: 0,
            start_errors: 0,
            start_seq: 0,
            print_on_exit,
            captured: Vec::new(),
            live_bytes_delta: 0,
            error_delta: 0,
        })
    }

    fn __enter__(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf.prev_level = diag::level();
        let st = diag::stats();
        slf.start_live_bytes = st.gpu_live_bytes;
        slf.start_errors = st.errors;
        slf.start_seq = diag::events().last().map_or(0, |e| e.seq);
        slf.captured.clear();
        diag::set_level(slf.target_level);
        slf
    }

    #[pyo3(signature = (_exc_type=None, _exc_val=None, _exc_tb=None))]
    fn __exit__(
        &mut self,
        py: Python<'_>,
        _exc_type: Option<Py<PyAny>>,
        _exc_val: Option<Py<PyAny>>,
        _exc_tb: Option<Py<PyAny>>,
    ) -> PyResult<bool> {
        let st = diag::stats();
        self.live_bytes_delta = st.gpu_live_bytes as i64 - self.start_live_bytes as i64;
        self.error_delta = st.errors.saturating_sub(self.start_errors);

        for ev in diag::events() {
            if ev.seq > self.start_seq {
                let d = PyDict::new(py);
                d.set_item("seq", ev.seq)?;
                d.set_item("level", ev.level.as_str())?;
                d.set_item("category", ev.category)?;
                d.set_item("op", ev.op)?;
                d.set_item("detail", ev.detail)?;
                d.set_item("elapsed_ms", ev.elapsed_ns as f64 / 1_000_000.0)?;
                d.set_item("ok", ev.ok)?;
                self.captured.push(d.unbind());
            }
        }
        diag::set_level(self.prev_level);

        if self.print_on_exit {
            eprintln!(
                "[nv-rs TRACE SUMMARY] events={} gpu_live_delta={} B errors_delta={}",
                self.captured.len(),
                self.live_bytes_delta,
                self.error_delta
            );
        }
        Ok(false)
    }

    #[getter]
    fn events(&self, py: Python<'_>) -> Vec<Py<PyDict>> {
        self.captured.iter().map(|d| d.clone_ref(py)).collect()
    }

    #[getter]
    fn live_bytes_delta(&self) -> i64 {
        self.live_bytes_delta
    }

    #[getter]
    fn error_delta(&self) -> u64 {
        self.error_delta
    }
}

/// Registers the `nv_rs.diag` submodule.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(py, "diag")?;
    m.add_function(wrap_pyfunction!(get_level, &m)?)?;
    m.add_function(wrap_pyfunction!(set_level, &m)?)?;
    m.add_function(wrap_pyfunction!(stats, &m)?)?;
    m.add_function(wrap_pyfunction!(events, &m)?)?;
    m.add_function(wrap_pyfunction!(clear_events, &m)?)?;
    m.add_function(wrap_pyfunction!(add_dll_directory, &m)?)?;
    m.add_function(wrap_pyfunction!(probe_libraries, &m)?)?;
    m.add_function(wrap_pyfunction!(doctor, &m)?)?;
    m.add_class::<TraceGuard>()?;
    parent.add_submodule(&m)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("nv_rs.diag", m)?;
    Ok(())
}
