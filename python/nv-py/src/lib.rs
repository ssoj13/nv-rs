//! `nv_rs` Python extension module (`_nv_rs`) built with PyO3 + maturin.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::all)]
#![deny(clippy::undocumented_unsafe_blocks)]

mod compute;
mod core;
mod diag;
mod dlpack;
mod errors;
mod gfx_img;
mod nvcomp;
mod nvjpeg;
mod nvml;
mod nvrtc;
mod platform_ext;
mod sparse_ai;
mod sys_ext;
mod video;

use pyo3::prelude::*;

/// Top-level `_nv_rs` extension module re-exported by `nv_rs`.
#[pymodule]
fn _nv_rs(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    errors::register(py, m)?;
    diag::register(py, m)?;
    core::register(py, m)?;
    nvml::register(py, m)?;
    nvrtc::register(py, m)?;
    nvcomp::register(py, m)?;
    nvjpeg::register(py, m)?;
    video::register(py, m)?;
    compute::register(py, m)?;
    sparse_ai::register(py, m)?;
    gfx_img::register(py, m)?;
    platform_ext::register(py, m)?;
    sys_ext::register(py, m)?;
    Ok(())
}
