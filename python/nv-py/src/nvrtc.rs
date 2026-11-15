//! `nv_rs.nvrtc` — GIL-free CUDA C++ runtime compilation to PTX, CUBIN, LTO-IR, and OptiX-IR.

use std::collections::HashMap;
use std::time::Instant;

use nv_loader::diag::{self, Level};
use nvrtc::{CompileOptions, Program};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::errors::{ErrorKind, record_and_err};

fn map_nvrtc_err(op: &str, err: nvrtc::Error) -> PyErr {
    let kind = match &err {
        nvrtc::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
        nvrtc::Error::Compilation { .. } => ErrorKind::Compile,
        nvrtc::Error::InvalidInput(_) => ErrorKind::InvalidArgument,
        _ => ErrorKind::Cuda,
    };
    record_and_err("nvrtc", op, err, kind)
}

/// Returns `(major, minor)` NVRTC version.
#[pyfunction]
fn version() -> PyResult<(i32, i32)> {
    nvrtc::version().map_err(|e| map_nvrtc_err("version", e))
}

/// Returns list of supported GPU SM architectures (e.g., `[75, 80, 86, 89, 90]`).
#[pyfunction]
fn supported_archs() -> PyResult<Vec<i32>> {
    nvrtc::supported_archs().map_err(|e| map_nvrtc_err("supported_archs", e))
}

/// Compiled NVRTC artifact bundle (PTX / CUBIN / LTO-IR / OptiX-IR + build log + lowered names).
#[pyclass(name = "CompileResult", module = "nv_rs.nvrtc")]
pub struct PyCompileResult {
    ptx: String,
    cubin: Option<Vec<u8>>,
    ltoir: Option<Vec<u8>>,
    optixir: Option<Vec<u8>>,
    log: String,
    lowered: HashMap<String, String>,
}

#[pymethods]
impl PyCompileResult {
    #[getter]
    fn ptx(&self) -> &str {
        &self.ptx
    }

    #[getter]
    fn ptx_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, self.ptx.as_bytes())
    }

    #[getter]
    fn cubin<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.cubin.as_deref().map(|b| PyBytes::new(py, b))
    }

    #[getter]
    fn ltoir<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.ltoir.as_deref().map(|b| PyBytes::new(py, b))
    }

    #[getter]
    fn optixir<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.optixir.as_deref().map(|b| PyBytes::new(py, b))
    }

    #[getter]
    fn log(&self) -> &str {
        &self.log
    }

    fn lowered_name(&self, expr: &str) -> PyResult<String> {
        self.lowered.get(expr).cloned().ok_or_else(|| {
            record_and_err(
                "nvrtc",
                "lowered_name",
                format!("expression `{expr}` was not registered in `name_expressions`"),
                ErrorKind::InvalidArgument,
            )
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "CompileResult(ptx_bytes={}, has_cubin={}, has_ltoir={}, has_optixir={})",
            self.ptx.len(),
            self.cubin.is_some(),
            self.ltoir.is_some(),
            self.optixir.is_some()
        )
    }
}

/// Compiles CUDA C++ `source` into PTX/CUBIN/LTO-IR/OptiX-IR without holding the Python GIL.
#[pyfunction]
#[pyo3(signature = (
    source,
    name=None,
    arch=None,
    std=None,
    defines=None,
    include_dirs=None,
    headers=None,
    name_expressions=None,
    fast_math=false,
    line_info=false,
    device_debug=false,
    rdc=false,
    dlto=false,
    extra_args=None
))]
#[allow(clippy::too_many_arguments)]
fn compile(
    py: Python<'_>,
    source: String,
    name: Option<String>,
    arch: Option<String>,
    std: Option<String>,
    defines: Option<HashMap<String, Option<String>>>,
    include_dirs: Option<Vec<String>>,
    headers: Option<HashMap<String, String>>,
    name_expressions: Option<Vec<String>>,
    fast_math: bool,
    line_info: bool,
    device_debug: bool,
    rdc: bool,
    dlto: bool,
    extra_args: Option<Vec<String>>,
) -> PyResult<PyCompileResult> {
    let t0 = Instant::now();
    let res = py.detach(move || {
        let header_pairs: Vec<(String, String)> = headers.unwrap_or_default().into_iter().collect();
        let header_refs: Vec<(&str, &str)> = header_pairs
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        let mut prog = Program::new(&source, name.as_deref(), &header_refs)?;
        let exprs = name_expressions.unwrap_or_default();
        for expr in &exprs {
            prog.add_name_expression(expr)?;
        }

        let mut opts = CompileOptions::new();
        if let Some(a) = arch.as_deref() {
            opts = opts.arch(a);
        }
        if let Some(s) = std.as_deref() {
            opts = opts.std(s);
        }
        if let Some(defs) = defines {
            for (k, v) in defs {
                opts = opts.define(&k, v.as_deref());
            }
        }
        if let Some(dirs) = include_dirs {
            for d in dirs {
                opts = opts.include_dir(&d);
            }
        }
        if fast_math {
            opts = opts.fast_math();
        }
        if line_info {
            opts = opts.line_info();
        }
        if device_debug {
            opts = opts.device_debug();
        }
        if rdc {
            opts = opts.rdc(true);
        }
        if dlto {
            opts = opts.dlto();
        }
        if let Some(args) = extra_args {
            for a in args {
                opts = opts.arg(a);
            }
        }

        let log = prog.compile(&opts)?;
        let ptx = prog.ptx_string().unwrap_or_default();
        let cubin = prog.cubin()?;
        let ltoir = prog.ltoir()?;
        let optixir = prog.optixir()?;
        let mut lowered = HashMap::with_capacity(exprs.len());
        for expr in exprs {
            lowered.insert(expr.clone(), prog.lowered_name(&expr)?);
        }
        Ok::<_, nvrtc::Error>(PyCompileResult {
            ptx,
            cubin,
            ltoir,
            optixir,
            log,
            lowered,
        })
    });

    match res {
        Ok(out) => {
            diag::record_event(
                Level::Info,
                "nvrtc",
                "compile",
                format!("ptx={} B", out.ptx.len()),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(out)
        }
        Err(e) => Err(map_nvrtc_err("compile", e)),
    }
}

/// Registers the `nv_rs.nvrtc` submodule.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(py, "nvrtc")?;
    m.add_function(wrap_pyfunction!(version, &m)?)?;
    m.add_function(wrap_pyfunction!(supported_archs, &m)?)?;
    m.add_function(wrap_pyfunction!(compile, &m)?)?;
    m.add_class::<PyCompileResult>()?;
    parent.add_submodule(&m)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("nv_rs.nvrtc", m)?;
    Ok(())
}
