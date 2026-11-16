//! `nv_rs.optix`, `nv_rs.npp`, and `nv_rs.ngx` — ray tracing, image/signal primitives, and DLSS/NGX capability queries.
//!
//! **What.** Registers three Python submodules on `_nv_rs` and in `sys.modules`:
//! - `nv_rs.optix`: [`PyOptixContext`] (`OptixContext`), [`PyOptixModule`] (`OptixModule`), and `abi_version()`.
//! - `nv_rs.npp`: [`PyNppStreamContext`] (`NppStreamContext`), `version()`, image ops (`rgb_to_gray_8u`,
//!   `rgb_to_ycbcr_8u`, `ycbcr_to_rgb_8u`, `resize_8u_c1`, `resize_8u_c3`, `resize_8u_c4`,
//!   `resize_32f_c3`, `filter_gauss_8u`, `threshold_val_8u_c1`, `threshold_val_32f_c1`), and 1D signal
//!   ops (`set_32f`, `add_32f`, `mul_32f`, `sum_32f_scratch_bytes`, `sum_32f`) over [`PyDeviceBuffer`].
//! - `nv_rs.ngx`: [`PyNgxCapabilities`] (`NgxCapabilities`), `is_dlss_supported()`,
//!   `required_extensions()`, `feature_requirements()`, and `validate_project_id()`.
//!
//! **Used by** `nv_rs` Python extension module (`python/nv-py/src/lib.rs`).

use std::sync::Arc;
use std::time::Instant;

use ngx::{DlssPerfQuality, Feature, NgxVulkan, ProjectId};
use npp::{Interpolation, MaskSize, NppContext, NppStatus, Rect, Size};
use nv_loader::diag::{self, Level};
use optix::{DebugLevel, DeviceContext, Module, ModuleOptions, OptLevel, PipelineOptions};
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::buf_util::{buf_ptr_and_bytes, validate_dev_buf};
use crate::core::{PyContext, PyDeviceBuffer, PyStream};
use crate::errors::{ErrorKind, record_and_err};

// ---------------------------------------------------------------------------
// nv_rs.optix
// ---------------------------------------------------------------------------

py_err_mapper!(map_optix_err, "optix", optix::Error, {
    optix::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    optix::Error::Invalid(_) | optix::Error::Budget { .. } => ErrorKind::InvalidArgument,
    optix::Error::Optix { log, .. } => {
        if log.is_empty() {
            ErrorKind::Cuda
        } else {
            ErrorKind::Compile
        }
    },
    optix::Error::Driver(_) => ErrorKind::Cuda,
});

py_parse_enum!(parse_opt_level, "optix", "parse_opt_level", OptLevel, "OptiX opt_level (expected default, l0, l1, l2, or l3)", {
    "default" => OptLevel::Default,
    "0" | "l0" => OptLevel::L0,
    "1" | "l1" => OptLevel::L1,
    "2" | "l2" => OptLevel::L2,
    "3" | "l3" => OptLevel::L3,
});

/// Returns the OptiX ABI version compiled into `optix-sys`.
#[pyfunction]
fn abi_version() -> u32 {
    optix::sys::OPTIX_ABI_VERSION
}

/// Compiled OptiX device module (`optix::Module`).
#[pyclass(name = "OptixModule", module = "nv_rs.optix")]
pub struct PyOptixModule {
    inner: Arc<Module>,
    log: String,
}

#[pymethods]
impl PyOptixModule {
    /// Compiler log emitted during `optixModuleCreate`.
    #[getter]
    fn log(&self) -> &str {
        &self.log
    }

    fn __repr__(&self) -> String {
        let _keep_alive = &self.inner;
        format!("OptixModule(log_len={})", self.log.len())
    }
}

/// OptiX device context bound to a CUDA context (`optix::DeviceContext`).
#[pyclass(name = "OptixContext", module = "nv_rs.optix")]
pub struct PyOptixContext {
    inner: Arc<DeviceContext>,
}

#[pymethods]
impl PyOptixContext {
    /// Creates an OptiX device context on `ctx`, GIL-free.
    #[new]
    #[pyo3(signature = (ctx, log_level=0, validation=false))]
    fn new(py: Python<'_>, ctx: &PyContext, log_level: u32, validation: bool) -> PyResult<Self> {
        let t0 = Instant::now();
        let cuda = ctx.inner.clone();
        let inner = py
            .detach(move || DeviceContext::new(&cuda, None, log_level, validation))
            .map_err(|e| map_optix_err("OptixContext::new", e))?;

        diag::record_event(
            Level::Info,
            "optix",
            "context_new",
            format!(
                "abi={} validation={validation}",
                optix::sys::OPTIX_ABI_VERSION
            ),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(Self { inner })
    }

    /// OptiX ABI version used by this context.
    #[getter]
    fn abi_version(&self) -> u32 {
        optix::sys::OPTIX_ABI_VERSION
    }

    /// RT core version (`0` = none, `10` = 1.0, ...).
    fn rtcore_version(&self, py: Python<'_>) -> PyResult<u32> {
        let ctx = self.inner.clone();
        py.detach(move || ctx.rtcore_version())
            .map_err(|e| map_optix_err("rtcore_version", e))
    }

    /// Maximum trace depth supported by the device.
    fn max_trace_depth(&self, py: Python<'_>) -> PyResult<u32> {
        let ctx = self.inner.clone();
        py.detach(move || ctx.max_trace_depth())
            .map_err(|e| map_optix_err("max_trace_depth", e))
    }

    /// Queries all OptiX device properties and limits as a dictionary.
    fn device_properties(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let t0 = Instant::now();
        let ctx = self.inner.clone();
        let props = py
            .detach(move || -> optix::Result<_> {
                Ok((
                    ctx.rtcore_version()?,
                    ctx.max_trace_depth()?,
                    ctx.max_graph_depth()?,
                    ctx.max_prims_per_gas()?,
                    ctx.max_instances_per_ias()?,
                    ctx.max_instance_id()?,
                    ctx.visibility_mask_bits()?,
                    ctx.max_sbt_records_per_gas()?,
                    ctx.max_sbt_offset()?,
                    ctx.shader_reordering()?,
                    ctx.coop_vec()?,
                    ctx.cluster_accel()?,
                    ctx.cache_enabled()?,
                ))
            })
            .map_err(|e| map_optix_err("device_properties", e))?;

        let d = PyDict::new(py);
        d.set_item("abi_version", optix::sys::OPTIX_ABI_VERSION)?;
        d.set_item("rtcore_version", props.0)?;
        d.set_item("max_trace_depth", props.1)?;
        d.set_item("max_graph_depth", props.2)?;
        d.set_item("max_prims_per_gas", props.3)?;
        d.set_item("max_instances_per_ias", props.4)?;
        d.set_item("max_instance_id", props.5)?;
        d.set_item("visibility_mask_bits", props.6)?;
        d.set_item("max_sbt_records_per_gas", props.7)?;
        d.set_item("max_sbt_offset", props.8)?;
        d.set_item("shader_reordering", props.9)?;
        d.set_item("coop_vec", props.10)?;
        d.set_item("cluster_accel", props.11)?;
        d.set_item("cache_enabled", props.12)?;

        diag::record_event(
            Level::Trace,
            "optix",
            "device_properties",
            format!("rtcore={} max_trace={}", props.0, props.1),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(d.unbind())
    }

    /// Compiles PTX text or OptiX-IR bytes into an `OptixModule`, GIL-free.
    #[pyo3(signature = (code, params_name="params", params_size=0, payload_values=2, opt_level="default"))]
    fn create_module(
        &self,
        py: Python<'_>,
        code: &[u8],
        params_name: &str,
        params_size: usize,
        payload_values: u32,
        opt_level: &str,
    ) -> PyResult<PyOptixModule> {
        let t0 = Instant::now();
        let opt = parse_opt_level(opt_level)?;
        let pipeline = PipelineOptions::new(params_name, params_size, payload_values)
            .map_err(|e| map_optix_err("create_module", e))?;
        let owned_code = code.to_vec();
        let ctx = self.inner.clone();

        let (module, log) = py
            .detach(move || {
                let mopts = ModuleOptions {
                    opt_level: opt,
                    debug_level: DebugLevel::Default,
                    ..ModuleOptions::default()
                };
                Module::new(&ctx, &mopts, &pipeline, &owned_code)
            })
            .map_err(|e| map_optix_err("create_module", e))?;

        diag::record_event(
            Level::Info,
            "optix",
            "create_module",
            format!("code_bytes={} log_bytes={}", code.len(), log.len()),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(PyOptixModule { inner: module, log })
    }

    fn __repr__(&self) -> String {
        format!("OptixContext(abi={})", optix::sys::OPTIX_ABI_VERSION)
    }
}

// ---------------------------------------------------------------------------
// nv_rs.npp
// ---------------------------------------------------------------------------

py_err_mapper!(map_npp_err, "npp", npp::Error, {
    npp::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    npp::Error::InvalidInput(_) | npp::Error::Budget { .. } => ErrorKind::InvalidArgument,
    npp::Error::Status { .. } | npp::Error::Driver(_) => ErrorKind::Cuda,
});

fn check_npp_status(call: &'static str, code: npp::sys::NppStatus) -> npp::Result<()> {
    let status = NppStatus(code);
    if status.is_error() {
        Err(npp::Error::Status { call, code: status })
    } else {
        Ok(())
    }
}

py_parse_enum!(parse_interp, "npp", "parse_interp", i32, "NPP interpolation (expected nearest, linear, cubic, super, or lanczos)", {
    "nn" | "nearest" => Interpolation::Nearest as i32,
    "linear" | "bilinear" => Interpolation::Linear as i32,
    "cubic" | "bicubic" => Interpolation::Cubic as i32,
    "super" => Interpolation::Super as i32,
    "lanczos" => Interpolation::Lanczos as i32,
});

fn interp_to_sys(interp_idx: i32) -> npp::Result<i32> {
    let raw = match interp_idx {
        0 => npp::sys::NppiInterpolationMode_NPPI_INTER_NN,
        1 => npp::sys::NppiInterpolationMode_NPPI_INTER_LINEAR,
        2 => npp::sys::NppiInterpolationMode_NPPI_INTER_CUBIC,
        3 => npp::sys::NppiInterpolationMode_NPPI_INTER_SUPER,
        _ => npp::sys::NppiInterpolationMode_NPPI_INTER_LANCZOS,
    };
    #[cfg_attr(
        target_os = "windows",
        expect(
            clippy::useless_conversion,
            reason = "NppiInterpolationMode is i32 on MSVC and u32 on Linux"
        )
    )]
    i32::try_from(raw).map_err(|_| npp::Error::InvalidInput("interpolation mode overflows i32"))
}

fn packed_step<T>(width: u32, channels: u32) -> PyResult<usize> {
    (width as usize)
        .checked_mul(channels as usize)
        .and_then(|n| n.checked_mul(size_of::<T>()))
        .ok_or_else(|| {
            record_and_err(
                "npp",
                "packed_step",
                "row byte stride overflow",
                ErrorKind::InvalidArgument,
            )
        })
}

fn validate_packed_view<T>(
    op: &str,
    name: &str,
    buf: &Bound<'_, PyDeviceBuffer>,
    size: Size,
    channels: u32,
) -> PyResult<(u64, i32, npp::sys::NppiSize)> {
    let (ptr, nbytes) = buf_ptr_and_bytes(buf);
    let elems = nbytes / size_of::<T>();
    let step = packed_step::<T>(size.width, channels)?;
    let step_i32 = npp::image::validate_image_layout::<T>(elems, size, channels, step)
        .map_err(|e| map_npp_err(op, e))?;
    if ptr == 0 {
        return Err(record_and_err(
            "npp",
            op,
            format!("{name} device pointer is null"),
            ErrorKind::InvalidArgument,
        ));
    }
    let roi = size.to_sys().map_err(|e| map_npp_err(op, e))?;
    Ok((ptr, step_i32, roi))
}

/// Returns the loaded `nppc` library version `(major, minor, build)`.
#[pyfunction]
fn version(py: Python<'_>) -> PyResult<(i32, i32, i32)> {
    let t0 = Instant::now();
    let v = py
        .detach(npp::lib_version)
        .map_err(|e| map_npp_err("version", e))?;
    diag::record_event(
        Level::Info,
        "npp",
        "version",
        format!("{}.{}.{}", v.major, v.minor, v.build),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok((v.major, v.minor, v.build))
}

/// Stream-bound NPP execution context (`npp::NppContext`).
#[pyclass(name = "NppStreamContext", module = "nv_rs.npp")]
#[derive(Clone)]
pub struct PyNppStreamContext {
    inner: Arc<NppContext>,
    device_id: i32,
}

#[pymethods]
impl PyNppStreamContext {
    /// Creates an `NppStreamContext` bound to `stream`, caching device properties once.
    #[new]
    fn new(py: Python<'_>, stream: &PyStream) -> PyResult<Self> {
        let t0 = Instant::now();
        let s = stream.inner.clone();
        let inner = py
            .detach(move || NppContext::new(&s))
            .map_err(|e| map_npp_err("NppStreamContext::new", e))?;
        diag::record_event(
            Level::Info,
            "npp",
            "context_new",
            format!("device={}", stream.device_id),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(Self {
            inner: Arc::new(inner),
            device_id: stream.device_id,
        })
    }

    #[getter]
    fn device_id(&self) -> i32 {
        self.device_id
    }

    fn __repr__(&self) -> String {
        format!("NppStreamContext(device_id={})", self.device_id)
    }
}

/// Converts packed 8-bit RGB (`C3`) to 8-bit grayscale (`C1`).
#[pyfunction]
fn rgb_to_gray_8u(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src: &Bound<'_, PyDeviceBuffer>,
    dst: &Bound<'_, PyDeviceBuffer>,
    width: u32,
    height: u32,
) -> PyResult<()> {
    let t0 = Instant::now();
    let size = Size { width, height };
    let (src_ptr, src_step, roi) =
        validate_packed_view::<u8>("rgb_to_gray_8u", "src", src, size, 3)?;
    let (dst_ptr, dst_step, _) = validate_packed_view::<u8>("rgb_to_gray_8u", "dst", dst, size, 1)?;
    let npp_ctx = ctx.inner.clone();

    py.detach(move || -> npp::Result<()> {
        npp::sys::nppicc::load().map_err(|e| npp::Error::Unavailable(e.to_string()))?;
        npp_ctx.stream().context().bind_to_thread()?;
        // SAFETY: `src_ptr` and `dst_ptr` are validated packed device image buffers for `roi`.
        check_npp_status("nppiRGBToGray_8u_C3C1R_Ctx", unsafe {
            npp::sys::nppiRGBToGray_8u_C3C1R_Ctx(
                src_ptr as *const u8,
                src_step,
                dst_ptr as *mut u8,
                dst_step,
                roi,
                npp_ctx.raw(),
            )
        })
    })
    .map_err(|e| map_npp_err("rgb_to_gray_8u", e))?;

    diag::record_event(
        Level::Trace,
        "npp",
        "rgb_to_gray_8u",
        format!("{width}x{height}"),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(())
}

/// Converts packed 8-bit RGB (`C3`) to YCbCr (`C3`).
#[pyfunction]
fn rgb_to_ycbcr_8u(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src: &Bound<'_, PyDeviceBuffer>,
    dst: &Bound<'_, PyDeviceBuffer>,
    width: u32,
    height: u32,
) -> PyResult<()> {
    let t0 = Instant::now();
    let size = Size { width, height };
    let (src_ptr, src_step, roi) =
        validate_packed_view::<u8>("rgb_to_ycbcr_8u", "src", src, size, 3)?;
    let (dst_ptr, dst_step, _) =
        validate_packed_view::<u8>("rgb_to_ycbcr_8u", "dst", dst, size, 3)?;
    let npp_ctx = ctx.inner.clone();

    py.detach(move || -> npp::Result<()> {
        npp::sys::nppicc::load().map_err(|e| npp::Error::Unavailable(e.to_string()))?;
        npp_ctx.stream().context().bind_to_thread()?;
        // SAFETY: `src_ptr` and `dst_ptr` are validated 3-channel packed device image buffers.
        check_npp_status("nppiRGBToYCbCr_8u_C3R_Ctx", unsafe {
            npp::sys::nppiRGBToYCbCr_8u_C3R_Ctx(
                src_ptr as *const u8,
                src_step,
                dst_ptr as *mut u8,
                dst_step,
                roi,
                npp_ctx.raw(),
            )
        })
    })
    .map_err(|e| map_npp_err("rgb_to_ycbcr_8u", e))?;

    diag::record_event(
        Level::Trace,
        "npp",
        "rgb_to_ycbcr_8u",
        format!("{width}x{height}"),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(())
}

/// Converts packed 8-bit YCbCr (`C3`) to RGB (`C3`).
#[pyfunction]
fn ycbcr_to_rgb_8u(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src: &Bound<'_, PyDeviceBuffer>,
    dst: &Bound<'_, PyDeviceBuffer>,
    width: u32,
    height: u32,
) -> PyResult<()> {
    let t0 = Instant::now();
    let size = Size { width, height };
    let (src_ptr, src_step, roi) =
        validate_packed_view::<u8>("ycbcr_to_rgb_8u", "src", src, size, 3)?;
    let (dst_ptr, dst_step, _) =
        validate_packed_view::<u8>("ycbcr_to_rgb_8u", "dst", dst, size, 3)?;
    let npp_ctx = ctx.inner.clone();

    py.detach(move || -> npp::Result<()> {
        npp::sys::nppicc::load().map_err(|e| npp::Error::Unavailable(e.to_string()))?;
        npp_ctx.stream().context().bind_to_thread()?;
        // SAFETY: `src_ptr` and `dst_ptr` are validated 3-channel packed device image buffers.
        check_npp_status("nppiYCbCrToRGB_8u_C3R_Ctx", unsafe {
            npp::sys::nppiYCbCrToRGB_8u_C3R_Ctx(
                src_ptr as *const u8,
                src_step,
                dst_ptr as *mut u8,
                dst_step,
                roi,
                npp_ctx.raw(),
            )
        })
    })
    .map_err(|e| map_npp_err("ycbcr_to_rgb_8u", e))?;

    diag::record_event(
        Level::Trace,
        "npp",
        "ycbcr_to_rgb_8u",
        format!("{width}x{height}"),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(())
}

fn resize_8u_impl(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src: &Bound<'_, PyDeviceBuffer>,
    dst: &Bound<'_, PyDeviceBuffer>,
    sizes: ((u32, u32), (u32, u32)),
    channels: u32,
    spec: (&'static str, &str),
) -> PyResult<()> {
    let (src_size, dst_size) = sizes;
    let (op, interp) = spec;
    let t0 = Instant::now();
    let s_size = Size {
        width: src_size.0,
        height: src_size.1,
    };
    let d_size = Size {
        width: dst_size.0,
        height: dst_size.1,
    };
    let interp_mode = interp_to_sys(parse_interp(interp)?).map_err(|e| map_npp_err(op, e))?;
    let (src_ptr, src_step, s_sz) = validate_packed_view::<u8>(op, "src", src, s_size, channels)?;
    let (dst_ptr, dst_step, d_sz) = validate_packed_view::<u8>(op, "dst", dst, d_size, channels)?;
    let s_roi = Rect::full(s_size)
        .validate_within(s_size)
        .map_err(|e| map_npp_err(op, e))?;
    let d_roi = Rect::full(d_size)
        .validate_within(d_size)
        .map_err(|e| map_npp_err(op, e))?;
    let npp_ctx = ctx.inner.clone();

    py.detach(move || -> npp::Result<()> {
        npp::sys::nppig::load().map_err(|e| npp::Error::Unavailable(e.to_string()))?;
        npp_ctx.stream().context().bind_to_thread()?;
        match channels {
            1 => {
                // SAFETY: validated 1-channel packed `u8` device buffers and full ROIs.
                check_npp_status("nppiResize_8u_C1R_Ctx", unsafe {
                    npp::sys::nppiResize_8u_C1R_Ctx(
                        src_ptr as *const u8,
                        src_step,
                        s_sz,
                        s_roi,
                        dst_ptr as *mut u8,
                        dst_step,
                        d_sz,
                        d_roi,
                        interp_mode,
                        npp_ctx.raw(),
                    )
                })
            }
            3 => {
                // SAFETY: validated 3-channel packed `u8` device buffers and full ROIs.
                check_npp_status("nppiResize_8u_C3R_Ctx", unsafe {
                    npp::sys::nppiResize_8u_C3R_Ctx(
                        src_ptr as *const u8,
                        src_step,
                        s_sz,
                        s_roi,
                        dst_ptr as *mut u8,
                        dst_step,
                        d_sz,
                        d_roi,
                        interp_mode,
                        npp_ctx.raw(),
                    )
                })
            }
            _ => {
                // SAFETY: validated 4-channel packed `u8` device buffers and full ROIs.
                check_npp_status("nppiResize_8u_C4R_Ctx", unsafe {
                    npp::sys::nppiResize_8u_C4R_Ctx(
                        src_ptr as *const u8,
                        src_step,
                        s_sz,
                        s_roi,
                        dst_ptr as *mut u8,
                        dst_step,
                        d_sz,
                        d_roi,
                        interp_mode,
                        npp_ctx.raw(),
                    )
                })
            }
        }
    })
    .map_err(|e| map_npp_err(op, e))?;

    diag::record_event(
        Level::Trace,
        "npp",
        op,
        format!(
            "{}x{}->{}x{}",
            src_size.0, src_size.1, dst_size.0, dst_size.1
        ),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(())
}

/// Resizes a packed 1-channel 8-bit image.
#[pyfunction]
#[pyo3(signature = (ctx, src, src_size, dst, dst_size, interp="linear"))]
fn resize_8u_c1(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src: &Bound<'_, PyDeviceBuffer>,
    src_size: (u32, u32),
    dst: &Bound<'_, PyDeviceBuffer>,
    dst_size: (u32, u32),
    interp: &str,
) -> PyResult<()> {
    resize_8u_impl(
        py,
        ctx,
        src,
        dst,
        (src_size, dst_size),
        1,
        ("resize_8u_c1", interp),
    )
}

/// Resizes a packed 3-channel 8-bit image.
#[pyfunction]
#[pyo3(signature = (ctx, src, src_size, dst, dst_size, interp="linear"))]
fn resize_8u_c3(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src: &Bound<'_, PyDeviceBuffer>,
    src_size: (u32, u32),
    dst: &Bound<'_, PyDeviceBuffer>,
    dst_size: (u32, u32),
    interp: &str,
) -> PyResult<()> {
    resize_8u_impl(
        py,
        ctx,
        src,
        dst,
        (src_size, dst_size),
        3,
        ("resize_8u_c3", interp),
    )
}

/// Resizes a packed 4-channel 8-bit image.
#[pyfunction]
#[pyo3(signature = (ctx, src, src_size, dst, dst_size, interp="linear"))]
fn resize_8u_c4(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src: &Bound<'_, PyDeviceBuffer>,
    src_size: (u32, u32),
    dst: &Bound<'_, PyDeviceBuffer>,
    dst_size: (u32, u32),
    interp: &str,
) -> PyResult<()> {
    resize_8u_impl(
        py,
        ctx,
        src,
        dst,
        (src_size, dst_size),
        4,
        ("resize_8u_c4", interp),
    )
}

/// Resizes a packed 3-channel 32-bit float image.
#[pyfunction]
#[pyo3(signature = (ctx, src, src_size, dst, dst_size, interp="linear"))]
fn resize_32f_c3(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src: &Bound<'_, PyDeviceBuffer>,
    src_size: (u32, u32),
    dst: &Bound<'_, PyDeviceBuffer>,
    dst_size: (u32, u32),
    interp: &str,
) -> PyResult<()> {
    let t0 = Instant::now();
    let s_size = Size {
        width: src_size.0,
        height: src_size.1,
    };
    let d_size = Size {
        width: dst_size.0,
        height: dst_size.1,
    };
    let interp_mode =
        interp_to_sys(parse_interp(interp)?).map_err(|e| map_npp_err("resize_32f_c3", e))?;
    let (src_ptr, src_step, s_sz) =
        validate_packed_view::<f32>("resize_32f_c3", "src", src, s_size, 3)?;
    let (dst_ptr, dst_step, d_sz) =
        validate_packed_view::<f32>("resize_32f_c3", "dst", dst, d_size, 3)?;
    let s_roi = Rect::full(s_size)
        .validate_within(s_size)
        .map_err(|e| map_npp_err("resize_32f_c3", e))?;
    let d_roi = Rect::full(d_size)
        .validate_within(d_size)
        .map_err(|e| map_npp_err("resize_32f_c3", e))?;
    let npp_ctx = ctx.inner.clone();

    py.detach(move || -> npp::Result<()> {
        npp::sys::nppig::load().map_err(|e| npp::Error::Unavailable(e.to_string()))?;
        npp_ctx.stream().context().bind_to_thread()?;
        // SAFETY: validated 3-channel packed `f32` device buffers and full ROIs.
        check_npp_status("nppiResize_32f_C3R_Ctx", unsafe {
            npp::sys::nppiResize_32f_C3R_Ctx(
                src_ptr as *const f32,
                src_step,
                s_sz,
                s_roi,
                dst_ptr as *mut f32,
                dst_step,
                d_sz,
                d_roi,
                interp_mode,
                npp_ctx.raw(),
            )
        })
    })
    .map_err(|e| map_npp_err("resize_32f_c3", e))?;

    diag::record_event(
        Level::Trace,
        "npp",
        "resize_32f_c3",
        format!(
            "{}x{}->{}x{}",
            src_size.0, src_size.1, dst_size.0, dst_size.1
        ),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(())
}

/// Applies a 3x3 Gaussian filter to the interior `(width - 2, height - 2)` ROI of a packed `u8` image (`C1` or `C3`).
#[pyfunction]
#[pyo3(signature = (ctx, src, dst, width, height, channels=1))]
fn filter_gauss_8u(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src: &Bound<'_, PyDeviceBuffer>,
    dst: &Bound<'_, PyDeviceBuffer>,
    width: u32,
    height: u32,
    channels: u32,
) -> PyResult<()> {
    let t0 = Instant::now();
    if channels != 1 && channels != 3 {
        return Err(record_and_err(
            "npp",
            "filter_gauss_8u",
            format!("filter_gauss_8u supports 1 or 3 channels, got {channels}"),
            ErrorKind::InvalidArgument,
        ));
    }
    if width < 3 || height < 3 {
        return Err(record_and_err(
            "npp",
            "filter_gauss_8u",
            "filter_gauss_8u requires width >= 3 and height >= 3 for 1px border margin",
            ErrorKind::InvalidArgument,
        ));
    }
    let full_size = Size { width, height };
    let (src_ptr, src_step, _) =
        validate_packed_view::<u8>("filter_gauss_8u", "src", src, full_size, channels)?;
    let (dst_ptr, dst_step, _) =
        validate_packed_view::<u8>("filter_gauss_8u", "dst", dst, full_size, channels)?;

    // Offset both pointers to (1, 1) so the 3x3 kernel never reads out-of-bounds.
    let byte_off = (src_step as u64) + (channels as u64);
    let dst_off = (dst_step as u64) + (channels as u64);
    let interior_roi = Size {
        width: width - 2,
        height: height - 2,
    }
    .to_sys()
    .map_err(|e| map_npp_err("filter_gauss_8u", e))?;
    let mask = match MaskSize::X3x3 {
        MaskSize::X3x3 => npp::sys::NppiMaskSize_NPP_MASK_SIZE_3_X_3,
        MaskSize::X5x5 => npp::sys::NppiMaskSize_NPP_MASK_SIZE_5_X_5,
        MaskSize::X7x7 => npp::sys::NppiMaskSize_NPP_MASK_SIZE_7_X_7,
    };
    let npp_ctx = ctx.inner.clone();

    py.detach(move || -> npp::Result<()> {
        npp::sys::nppif::load().map_err(|e| npp::Error::Unavailable(e.to_string()))?;
        npp_ctx.stream().context().bind_to_thread()?;
        let s_ptr = (src_ptr + byte_off) as *const u8;
        let d_ptr = (dst_ptr + dst_off) as *mut u8;
        if channels == 1 {
            // SAFETY: interior `(1, 1)` offset guarantees 1px valid border around `interior_roi`.
            check_npp_status("nppiFilterGauss_8u_C1R_Ctx", unsafe {
                npp::sys::nppiFilterGauss_8u_C1R_Ctx(
                    s_ptr,
                    src_step,
                    d_ptr,
                    dst_step,
                    interior_roi,
                    mask,
                    npp_ctx.raw(),
                )
            })
        } else {
            // SAFETY: interior `(1, 1)` offset guarantees 1px valid border around `interior_roi`.
            check_npp_status("nppiFilterGauss_8u_C3R_Ctx", unsafe {
                npp::sys::nppiFilterGauss_8u_C3R_Ctx(
                    s_ptr,
                    src_step,
                    d_ptr,
                    dst_step,
                    interior_roi,
                    mask,
                    npp_ctx.raw(),
                )
            })
        }
    })
    .map_err(|e| map_npp_err("filter_gauss_8u", e))?;

    diag::record_event(
        Level::Trace,
        "npp",
        "filter_gauss_8u",
        format!("{width}x{height}x{channels}"),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(())
}

/// Thresholds a 1-channel 8-bit image (`pixel > threshold -> value`).
#[pyfunction]
fn threshold_val_8u_c1(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src: &Bound<'_, PyDeviceBuffer>,
    dst: &Bound<'_, PyDeviceBuffer>,
    size: (u32, u32),
    threshold: u8,
    value: u8,
) -> PyResult<()> {
    let t0 = Instant::now();
    let img_sz = Size {
        width: size.0,
        height: size.1,
    };
    let (src_ptr, src_step, roi) =
        validate_packed_view::<u8>("threshold_val_8u_c1", "src", src, img_sz, 1)?;
    let (dst_ptr, dst_step, _) =
        validate_packed_view::<u8>("threshold_val_8u_c1", "dst", dst, img_sz, 1)?;
    let npp_ctx = ctx.inner.clone();

    py.detach(move || -> npp::Result<()> {
        npp::sys::nppitc::load().map_err(|e| npp::Error::Unavailable(e.to_string()))?;
        npp_ctx.stream().context().bind_to_thread()?;
        // SAFETY: validated 1-channel `u8` device buffers for `roi`.
        check_npp_status("nppiThreshold_Val_8u_C1R_Ctx", unsafe {
            npp::sys::nppiThreshold_Val_8u_C1R_Ctx(
                src_ptr as *const u8,
                src_step,
                dst_ptr as *mut u8,
                dst_step,
                roi,
                threshold,
                value,
                npp::sys::NppCmpOp_NPP_CMP_GREATER,
                npp_ctx.raw(),
            )
        })
    })
    .map_err(|e| map_npp_err("threshold_val_8u_c1", e))?;

    diag::record_event(
        Level::Trace,
        "npp",
        "threshold_val_8u_c1",
        format!("{}x{}", size.0, size.1),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(())
}

/// Thresholds a 1-channel 32-bit float image (`pixel > threshold -> value`).
#[pyfunction]
fn threshold_val_32f_c1(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src: &Bound<'_, PyDeviceBuffer>,
    dst: &Bound<'_, PyDeviceBuffer>,
    size: (u32, u32),
    threshold: f32,
    value: f32,
) -> PyResult<()> {
    let t0 = Instant::now();
    let img_sz = Size {
        width: size.0,
        height: size.1,
    };
    let (src_ptr, src_step, roi) =
        validate_packed_view::<f32>("threshold_val_32f_c1", "src", src, img_sz, 1)?;
    let (dst_ptr, dst_step, _) =
        validate_packed_view::<f32>("threshold_val_32f_c1", "dst", dst, img_sz, 1)?;
    let npp_ctx = ctx.inner.clone();

    py.detach(move || -> npp::Result<()> {
        npp::sys::nppitc::load().map_err(|e| npp::Error::Unavailable(e.to_string()))?;
        npp_ctx.stream().context().bind_to_thread()?;
        // SAFETY: validated 1-channel `f32` device buffers for `roi`.
        check_npp_status("nppiThreshold_Val_32f_C1R_Ctx", unsafe {
            npp::sys::nppiThreshold_Val_32f_C1R_Ctx(
                src_ptr as *const f32,
                src_step,
                dst_ptr as *mut f32,
                dst_step,
                roi,
                threshold,
                value,
                npp::sys::NppCmpOp_NPP_CMP_GREATER,
                npp_ctx.raw(),
            )
        })
    })
    .map_err(|e| map_npp_err("threshold_val_32f_c1", e))?;

    diag::record_event(
        Level::Trace,
        "npp",
        "threshold_val_32f_c1",
        format!("{}x{}", size.0, size.1),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(())
}

/// Fills a 1D `f32` signal buffer with `value` (`nppsSet_32f_Ctx`).
#[pyfunction]
fn set_32f(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    value: f32,
    dst: &Bound<'_, PyDeviceBuffer>,
) -> PyResult<()> {
    let t0 = Instant::now();
    let (dst_ptr, dst_bytes) = buf_ptr_and_bytes(dst);
    let len = dst_bytes / size_of::<f32>();
    if len == 0 || dst_bytes % size_of::<f32>() != 0 {
        return Err(record_and_err(
            "npp",
            "set_32f",
            "dst buffer must be non-empty and a multiple of 4 bytes",
            ErrorKind::InvalidArgument,
        ));
    }
    validate_dev_buf("npp", "set_32f", "dst", dst_ptr, dst_bytes, 4)?;
    let npp_ctx = ctx.inner.clone();

    py.detach(move || -> npp::Result<()> {
        npp::sys::npps::load().map_err(|e| npp::Error::Unavailable(e.to_string()))?;
        npp_ctx.stream().context().bind_to_thread()?;
        // SAFETY: `dst_ptr` points to `len` valid `f32` device elements.
        check_npp_status("nppsSet_32f_Ctx", unsafe {
            npp::sys::nppsSet_32f_Ctx(value, dst_ptr as *mut f32, len, npp_ctx.raw())
        })
    })
    .map_err(|e| map_npp_err("set_32f", e))?;

    diag::record_event(
        Level::Trace,
        "npp",
        "set_32f",
        format!("len={len}"),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(())
}

/// Elementwise `dst = src1 + src2` for 1D `f32` signal buffers (`nppsAdd_32f_Ctx`).
#[pyfunction]
fn add_32f(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src1: &Bound<'_, PyDeviceBuffer>,
    src2: &Bound<'_, PyDeviceBuffer>,
    dst: &Bound<'_, PyDeviceBuffer>,
) -> PyResult<()> {
    let t0 = Instant::now();
    let (p1, b1) = buf_ptr_and_bytes(src1);
    let (p2, b2) = buf_ptr_and_bytes(src2);
    let (pd, bd) = buf_ptr_and_bytes(dst);
    if b1 == 0 || b1 != b2 || b1 != bd || b1 % size_of::<f32>() != 0 {
        return Err(record_and_err(
            "npp",
            "add_32f",
            "src1, src2, dst must be non-empty, equal byte length, and 4-byte aligned",
            ErrorKind::InvalidArgument,
        ));
    }
    validate_dev_buf("npp", "add_32f", "src1", p1, b1, b1)?;
    validate_dev_buf("npp", "add_32f", "src2", p2, b2, b2)?;
    validate_dev_buf("npp", "add_32f", "dst", pd, bd, bd)?;
    let len = b1 / size_of::<f32>();
    let npp_ctx = ctx.inner.clone();

    py.detach(move || -> npp::Result<()> {
        npp::sys::npps::load().map_err(|e| npp::Error::Unavailable(e.to_string()))?;
        npp_ctx.stream().context().bind_to_thread()?;
        // SAFETY: all three device buffers have `len` valid `f32` elements.
        check_npp_status("nppsAdd_32f_Ctx", unsafe {
            npp::sys::nppsAdd_32f_Ctx(
                p1 as *const f32,
                p2 as *const f32,
                pd as *mut f32,
                len,
                npp_ctx.raw(),
            )
        })
    })
    .map_err(|e| map_npp_err("add_32f", e))?;

    diag::record_event(
        Level::Trace,
        "npp",
        "add_32f",
        format!("len={len}"),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(())
}

/// Elementwise `dst = src1 * src2` for 1D `f32` signal buffers (`nppsMul_32f_Ctx`).
#[pyfunction]
fn mul_32f(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src1: &Bound<'_, PyDeviceBuffer>,
    src2: &Bound<'_, PyDeviceBuffer>,
    dst: &Bound<'_, PyDeviceBuffer>,
) -> PyResult<()> {
    let t0 = Instant::now();
    let (p1, b1) = buf_ptr_and_bytes(src1);
    let (p2, b2) = buf_ptr_and_bytes(src2);
    let (pd, bd) = buf_ptr_and_bytes(dst);
    if b1 == 0 || b1 != b2 || b1 != bd || b1 % size_of::<f32>() != 0 {
        return Err(record_and_err(
            "npp",
            "mul_32f",
            "src1, src2, dst must be non-empty, equal byte length, and 4-byte aligned",
            ErrorKind::InvalidArgument,
        ));
    }
    validate_dev_buf("npp", "mul_32f", "src1", p1, b1, b1)?;
    validate_dev_buf("npp", "mul_32f", "src2", p2, b2, b2)?;
    validate_dev_buf("npp", "mul_32f", "dst", pd, bd, bd)?;
    let len = b1 / size_of::<f32>();
    let npp_ctx = ctx.inner.clone();

    py.detach(move || -> npp::Result<()> {
        npp::sys::npps::load().map_err(|e| npp::Error::Unavailable(e.to_string()))?;
        npp_ctx.stream().context().bind_to_thread()?;
        // SAFETY: all three device buffers have `len` valid `f32` elements.
        check_npp_status("nppsMul_32f_Ctx", unsafe {
            npp::sys::nppsMul_32f_Ctx(
                p1 as *const f32,
                p2 as *const f32,
                pd as *mut f32,
                len,
                npp_ctx.raw(),
            )
        })
    })
    .map_err(|e| map_npp_err("mul_32f", e))?;

    diag::record_event(
        Level::Trace,
        "npp",
        "mul_32f",
        format!("len={len}"),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(())
}

/// Queries the scratch buffer byte size required for `sum_32f` over `length` `f32` elements.
#[pyfunction]
fn sum_32f_scratch_bytes(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    length: usize,
) -> PyResult<usize> {
    if length == 0 {
        return Err(record_and_err(
            "npp",
            "sum_32f_scratch_bytes",
            "signal length must be non-zero",
            ErrorKind::InvalidArgument,
        ));
    }
    let npp_ctx = ctx.inner.clone();
    py.detach(move || -> npp::Result<usize> {
        npp::sys::npps::load().map_err(|e| npp::Error::Unavailable(e.to_string()))?;
        npp_ctx.stream().context().bind_to_thread()?;
        let mut bytes = 0usize;
        // SAFETY: `bytes` is a valid host out-pointer.
        check_npp_status("nppsSumGetBufferSize_32f_Ctx", unsafe {
            npp::sys::nppsSumGetBufferSize_32f_Ctx(length, &mut bytes, npp_ctx.raw())
        })?;
        Ok(bytes)
    })
    .map_err(|e| map_npp_err("sum_32f_scratch_bytes", e))
}

/// Computes the sum of `src` into `sum_out[0]` using caller-owned `scratch` (`nppsSum_32f_Ctx`).
#[pyfunction]
fn sum_32f(
    py: Python<'_>,
    ctx: &PyNppStreamContext,
    src: &Bound<'_, PyDeviceBuffer>,
    sum_out: &Bound<'_, PyDeviceBuffer>,
    scratch: &Bound<'_, PyDeviceBuffer>,
) -> PyResult<()> {
    let t0 = Instant::now();
    let (ps, bs) = buf_ptr_and_bytes(src);
    let (po, bo) = buf_ptr_and_bytes(sum_out);
    let (pw, bw) = buf_ptr_and_bytes(scratch);
    if bs == 0 || bs % size_of::<f32>() != 0 {
        return Err(record_and_err(
            "npp",
            "sum_32f",
            "src must be non-empty and a multiple of 4 bytes",
            ErrorKind::InvalidArgument,
        ));
    }
    validate_dev_buf("npp", "sum_32f", "src", ps, bs, bs)?;
    validate_dev_buf("npp", "sum_32f", "sum_out", po, bo, size_of::<f32>())?;
    let len = bs / size_of::<f32>();
    let npp_ctx = ctx.inner.clone();

    py.detach(move || -> npp::Result<()> {
        npp::sys::npps::load().map_err(|e| npp::Error::Unavailable(e.to_string()))?;
        npp_ctx.stream().context().bind_to_thread()?;
        let mut needed = 0usize;
        // SAFETY: `needed` is a valid host out-pointer.
        check_npp_status("nppsSumGetBufferSize_32f_Ctx", unsafe {
            npp::sys::nppsSumGetBufferSize_32f_Ctx(len, &mut needed, npp_ctx.raw())
        })?;
        if bw < needed {
            return Err(npp::Error::Budget {
                requested: needed,
                available: bw,
            });
        }
        if needed > 0 && pw == 0 {
            return Err(npp::Error::InvalidInput("scratch device pointer is null"));
        }
        // SAFETY: `src`, `sum_out`, and `scratch` (at least `needed` bytes) are valid device buffers.
        check_npp_status("nppsSum_32f_Ctx", unsafe {
            npp::sys::nppsSum_32f_Ctx(
                ps as *const f32,
                len,
                po as *mut f32,
                pw as *mut u8,
                npp_ctx.raw(),
            )
        })
    })
    .map_err(|e| map_npp_err("sum_32f", e))?;

    diag::record_event(
        Level::Trace,
        "npp",
        "sum_32f",
        format!("len={len}"),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// nv_rs.ngx
// ---------------------------------------------------------------------------

py_err_mapper!(map_ngx_err, "ngx", ngx::Error, {
    ngx::Error::Unsupported(_) => ErrorKind::LibraryNotFound,
    ngx::Error::InvalidArgument(_) => ErrorKind::InvalidArgument,
    ngx::Error::Ngx { .. } => ErrorKind::Cuda,
});

py_parse_enum!(parse_ngx_feature, "ngx", "parse_ngx_feature", (&'static str, Feature), "NGX feature", {
    "super_sampling" | "dlss" | "dlss_sr" => ("super_sampling", Feature::SuperSampling),
    "in_painting" => ("in_painting", Feature::InPainting),
    "image_super_resolution" => ("image_super_resolution", Feature::ImageSuperResolution),
    "slow_motion" => ("slow_motion", Feature::SlowMotion),
    "video_super_resolution" => ("video_super_resolution", Feature::VideoSuperResolution),
    "image_signal_processing" => ("image_signal_processing", Feature::ImageSignalProcessing),
    "deep_resolve" => ("deep_resolve", Feature::DeepResolve),
    "frame_generation" | "dlss_fg" => ("frame_generation", Feature::FrameGeneration),
    "deep_dvc" => ("deep_dvc", Feature::DeepDvc),
    "ray_reconstruction" | "dlss_rr" => ("ray_reconstruction", Feature::RayReconstruction),
});

/// NGX / DLSS runtime capabilities and Vulkan extension requirements.
#[pyclass(name = "NgxCapabilities", module = "nv_rs.ngx")]
#[derive(Clone)]
pub struct PyNgxCapabilities {
    supported: bool,
    instance_extensions: Vec<String>,
    device_extensions: Vec<String>,
    status_message: Option<String>,
}

#[pymethods]
impl PyNgxCapabilities {
    /// Queries NGX Vulkan extension requirements without requiring a live `VkDevice`.
    #[new]
    fn new(py: Python<'_>) -> Self {
        let t0 = Instant::now();
        let res = py.detach(NgxVulkan::required_extensions);
        let caps = match res {
            Ok(exts) => Self {
                supported: true,
                instance_extensions: exts.instance_extensions,
                device_extensions: exts.device_extensions,
                status_message: None,
            },
            Err(err) => Self {
                supported: false,
                instance_extensions: Vec::new(),
                device_extensions: Vec::new(),
                status_message: Some(err.to_string()),
            },
        };
        diag::record_event(
            Level::Info,
            "ngx",
            "capabilities",
            format!(
                "supported={} inst_exts={} dev_exts={}",
                caps.supported,
                caps.instance_extensions.len(),
                caps.device_extensions.len()
            ),
            t0.elapsed().as_nanos() as u64,
            caps.supported,
        );
        caps
    }

    /// Whether NGX Vulkan runtime is available on this host.
    #[getter]
    fn supported(&self) -> bool {
        self.supported
    }

    /// Required Vulkan instance extensions for NGX.
    #[getter]
    fn instance_extensions(&self) -> Vec<String> {
        self.instance_extensions.clone()
    }

    /// Required Vulkan device extensions for NGX.
    #[getter]
    fn device_extensions(&self) -> Vec<String> {
        self.device_extensions.clone()
    }

    /// Diagnostic message if NGX runtime query failed or is unsupported.
    #[getter]
    fn status_message(&self) -> Option<&str> {
        self.status_message.as_deref()
    }

    /// Returns `True` if NGX Vulkan runtime reports extension support on this host.
    fn is_dlss_supported(&self) -> bool {
        self.supported
    }

    /// Returns `(instance_extensions, device_extensions)` required by NGX on Vulkan.
    fn required_extensions(&self) -> (Vec<String>, Vec<String>) {
        (
            self.instance_extensions.clone(),
            self.device_extensions.clone(),
        )
    }

    /// Lists all `ngx::Feature` identifiers and their raw SDK enum values.
    fn supported_features(&self) -> Vec<(&'static str, i32)> {
        vec![
            ("super_sampling", Feature::SuperSampling.to_raw()),
            ("in_painting", Feature::InPainting.to_raw()),
            (
                "image_super_resolution",
                Feature::ImageSuperResolution.to_raw(),
            ),
            ("slow_motion", Feature::SlowMotion.to_raw()),
            (
                "video_super_resolution",
                Feature::VideoSuperResolution.to_raw(),
            ),
            (
                "image_signal_processing",
                Feature::ImageSignalProcessing.to_raw(),
            ),
            ("deep_resolve", Feature::DeepResolve.to_raw()),
            ("frame_generation", Feature::FrameGeneration.to_raw()),
            ("deep_dvc", Feature::DeepDvc.to_raw()),
            ("ray_reconstruction", Feature::RayReconstruction.to_raw()),
        ]
    }

    /// Lists all DLSS performance/quality presets and their raw SDK enum values.
    fn dlss_quality_presets(&self) -> Vec<(&'static str, i32)> {
        vec![
            ("max_perf", DlssPerfQuality::MaxPerf.to_raw()),
            ("balanced", DlssPerfQuality::Balanced.to_raw()),
            ("max_quality", DlssPerfQuality::MaxQuality.to_raw()),
            (
                "ultra_performance",
                DlssPerfQuality::UltraPerformance.to_raw(),
            ),
            ("ultra_quality", DlssPerfQuality::UltraQuality.to_raw()),
            ("dlaa", DlssPerfQuality::Dlaa.to_raw()),
        ]
    }

    fn __repr__(&self) -> String {
        format!(
            "NgxCapabilities(supported={}, inst_exts={}, dev_exts={})",
            self.supported,
            self.instance_extensions.len(),
            self.device_extensions.len()
        )
    }
}

/// Returns `True` if the NGX Vulkan runtime is available on this host.
#[pyfunction]
fn is_dlss_supported(py: Python<'_>) -> bool {
    PyNgxCapabilities::new(py).supported
}

/// Queries the Vulkan instance and device extensions required by NGX (`NVSDK_NGX_VULKAN_RequiredExtensions`).
#[pyfunction]
fn required_extensions(py: Python<'_>) -> PyResult<Py<PyDict>> {
    let t0 = Instant::now();
    let exts = py
        .detach(NgxVulkan::required_extensions)
        .map_err(|e| map_ngx_err("required_extensions", e))?;
    let d = PyDict::new(py);
    d.set_item("instance_extensions", &exts.instance_extensions)?;
    d.set_item("device_extensions", &exts.device_extensions)?;
    diag::record_event(
        Level::Info,
        "ngx",
        "required_extensions",
        format!(
            "inst={} dev={}",
            exts.instance_extensions.len(),
            exts.device_extensions.len()
        ),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(d.unbind())
}

/// Queries feature metadata and Vulkan extension requirements for an NGX feature name.
#[pyfunction]
#[pyo3(signature = (feature="super_sampling"))]
fn feature_requirements(py: Python<'_>, feature: &str) -> PyResult<Py<PyDict>> {
    let (canonical, feat) = parse_ngx_feature(feature)?;
    let caps = PyNgxCapabilities::new(py);
    let d = PyDict::new(py);
    d.set_item("feature", canonical)?;
    d.set_item("feature_id", feat.to_raw())?;
    d.set_item("supported", caps.supported)?;
    d.set_item("platform_windows", cfg!(target_os = "windows"))?;
    d.set_item("instance_extensions", caps.instance_extensions)?;
    d.set_item("device_extensions", caps.device_extensions)?;
    if let Some(msg) = caps.status_message {
        d.set_item("status_message", msg)?;
    }
    Ok(d.unbind())
}

/// Validates an NGX `ProjectId` GUID and optional engine metadata (`NVSDK_NGX_ENGINE_TYPE_CUSTOM`).
#[pyfunction]
#[pyo3(signature = (guid, engine_name=None, engine_version=None))]
fn validate_project_id(
    py: Python<'_>,
    guid: &str,
    engine_name: Option<&str>,
    engine_version: Option<&str>,
) -> PyResult<Py<PyDict>> {
    let mut pid =
        ProjectId::from_guid_str(guid).map_err(|e| map_ngx_err("validate_project_id", e))?;
    if let Some(name) = engine_name {
        pid = pid
            .with_engine_name(name)
            .map_err(|e| map_ngx_err("validate_project_id", e))?;
    }
    if let Some(ver) = engine_version {
        pid = pid
            .with_engine_version(ver)
            .map_err(|e| map_ngx_err("validate_project_id", e))?;
    }
    let d = PyDict::new(py);
    d.set_item("guid", pid.guid().to_string())?;
    d.set_item("engine_type", pid.engine_type())?;
    d.set_item("engine_name", pid.engine_name())?;
    d.set_item("engine_version", pid.engine_version())?;
    Ok(d.unbind())
}

// ---------------------------------------------------------------------------
// Module registration
// ---------------------------------------------------------------------------

/// Registers `nv_rs.optix`, `nv_rs.npp`, and `nv_rs.ngx` submodules.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    py_register_submodule!(py, parent, "optix", |m| {
        m.add_class::<PyOptixContext>()?;
        m.add_class::<PyOptixModule>()?;
        m.add_function(wrap_pyfunction!(abi_version, &m)?)?;
    })?;

    py_register_submodule!(py, parent, "npp", |m| {
        m.add_class::<PyNppStreamContext>()?;
        m.add_function(wrap_pyfunction!(version, &m)?)?;
        m.add_function(wrap_pyfunction!(rgb_to_gray_8u, &m)?)?;
        m.add_function(wrap_pyfunction!(rgb_to_ycbcr_8u, &m)?)?;
        m.add_function(wrap_pyfunction!(ycbcr_to_rgb_8u, &m)?)?;
        m.add_function(wrap_pyfunction!(resize_8u_c1, &m)?)?;
        m.add_function(wrap_pyfunction!(resize_8u_c3, &m)?)?;
        m.add_function(wrap_pyfunction!(resize_8u_c4, &m)?)?;
        m.add_function(wrap_pyfunction!(resize_32f_c3, &m)?)?;
        m.add_function(wrap_pyfunction!(filter_gauss_8u, &m)?)?;
        m.add_function(wrap_pyfunction!(threshold_val_8u_c1, &m)?)?;
        m.add_function(wrap_pyfunction!(threshold_val_32f_c1, &m)?)?;
        m.add_function(wrap_pyfunction!(set_32f, &m)?)?;
        m.add_function(wrap_pyfunction!(add_32f, &m)?)?;
        m.add_function(wrap_pyfunction!(mul_32f, &m)?)?;
        m.add_function(wrap_pyfunction!(sum_32f_scratch_bytes, &m)?)?;
        m.add_function(wrap_pyfunction!(sum_32f, &m)?)?;
    })?;

    py_register_submodule!(py, parent, "ngx", |m| {
        m.add_class::<PyNgxCapabilities>()?;
        m.add_function(wrap_pyfunction!(is_dlss_supported, &m)?)?;
        m.add_function(wrap_pyfunction!(required_extensions, &m)?)?;
        m.add_function(wrap_pyfunction!(feature_requirements, &m)?)?;
        m.add_function(wrap_pyfunction!(validate_project_id, &m)?)?;
    })?;

    Ok(())
}
