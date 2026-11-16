//! Unified Python exception hierarchy for `nv_rs` with automatic diagnostic error recording.

use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyRuntimeError, PyValueError};
use pyo3::prelude::*;

create_exception!(nv_rs.errors, NvError, PyException);
create_exception!(nv_rs.errors, NvLibraryNotFoundError, NvError);
create_exception!(nv_rs.errors, NvCudaError, NvError);
create_exception!(nv_rs.errors, NvInvalidArgumentError, NvError);
create_exception!(nv_rs.errors, NvCompileError, NvError);

/// Records an error into `nv_loader::diag` and converts it into the specified Python exception.
pub fn record_and_err<E: std::fmt::Display>(
    category: &str,
    op: &str,
    err: E,
    kind: ErrorKind,
) -> PyErr {
    let msg = err.to_string();
    nv_loader::diag::record_error(category, op, &msg);
    match kind {
        ErrorKind::LibraryNotFound => NvLibraryNotFoundError::new_err(msg),
        ErrorKind::Cuda => NvCudaError::new_err(msg),
        ErrorKind::InvalidArgument => NvInvalidArgumentError::new_err(msg),
        ErrorKind::Compile => NvCompileError::new_err(msg),
        ErrorKind::General => NvError::new_err(msg),
        ErrorKind::Value => PyValueError::new_err(msg),
        ErrorKind::Runtime => PyRuntimeError::new_err(msg),
    }
}

/// Classification of `nv_rs` errors mapped to Python exception subclasses.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    /// Missing or rejected NVIDIA DLL / shared object.
    LibraryNotFound,
    /// CUDA driver / runtime / library status error.
    Cuda,
    /// Invalid user argument or shape mismatch.
    InvalidArgument,
    /// NVRTC compilation failure (includes compiler build log).
    Compile,
    /// General `NvError`.
    General,
    /// Standard `ValueError` when appropriate.
    Value,
    /// Standard `RuntimeError` fallback.
    Runtime,
}

/// Registers the `nv_rs.errors` submodule.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    py_register_submodule!(py, parent, "errors", |m| {
        m.add("NvError", py.get_type::<NvError>())?;
        m.add(
            "NvLibraryNotFoundError",
            py.get_type::<NvLibraryNotFoundError>(),
        )?;
        m.add("NvCudaError", py.get_type::<NvCudaError>())?;
        m.add(
            "NvInvalidArgumentError",
            py.get_type::<NvInvalidArgumentError>(),
        )?;
        m.add("NvCompileError", py.get_type::<NvCompileError>())?;
    })
}
