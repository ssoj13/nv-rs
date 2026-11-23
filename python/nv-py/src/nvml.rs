//! `nv_rs.nvml` — safe NVML GPU telemetry and enumeration.
//!
//! Optional telemetry fields (`clocks_mhz`, `summary`) are `None` only when the device reports
//! `NVML_ERROR_NOT_SUPPORTED` (typed status, see [`optional`]); every other NVML error raises.

use std::sync::Arc;

use nvml::{ClockDomain, Device, MemoryInfo, Nvml, Utilization};
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::errors::ErrorKind;

py_err_mapper!(map_nvml_err, "nvml", nvml::Error, {
    nvml::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    nvml::Error::InvalidInput(_) => ErrorKind::InvalidArgument,
    nvml::Error::Status { .. } => ErrorKind::Cuda,
});

/// `Some(value)`, or `None` when the device does not support the query
/// (`NVML_ERROR_NOT_SUPPORTED`, which is not recorded as an error); any other failure (GPU lost,
/// driver error, ...) is raised and recorded in `nv_rs.diag`.
fn optional<T>(op: &str, res: nvml::Result<T>) -> PyResult<Option<T>> {
    match res {
        Ok(v) => Ok(Some(v)),
        Err(nvml::Error::Status { status, .. }) if status.is_not_supported() => Ok(None),
        Err(e) => Err(map_nvml_err(op, e)),
    }
}

fn memory_dict<'py>(py: Python<'py>, mem: &MemoryInfo) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("total", mem.total)?;
    d.set_item("free", mem.free)?;
    d.set_item("used", mem.used)?;
    Ok(d)
}

fn utilization_dict<'py>(py: Python<'py>, u: &Utilization) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("gpu", u.gpu)?;
    d.set_item("memory", u.memory)?;
    Ok(d)
}

/// NVML session handle.
#[pyclass(name = "Nvml", module = "nv_rs.nvml")]
pub struct PyNvml {
    inner: Arc<Nvml>,
}

#[pymethods]
impl PyNvml {
    #[new]
    fn new() -> PyResult<Self> {
        let inner = Nvml::init().map_err(|e| map_nvml_err("Nvml::init", e))?;
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    fn driver_version(&self) -> PyResult<String> {
        self.inner
            .driver_version()
            .map_err(|e| map_nvml_err("driver_version", e))
    }

    fn cuda_driver_version(&self) -> PyResult<(i32, i32)> {
        self.inner
            .cuda_driver_version()
            .map_err(|e| map_nvml_err("cuda_driver_version", e))
    }

    fn device_count(&self) -> PyResult<u32> {
        self.inner
            .device_count()
            .map_err(|e| map_nvml_err("device_count", e))
    }

    fn device(&self, index: u32) -> PyResult<PyNvmlDevice> {
        let dev = self
            .inner
            .device_by_index(index)
            .map_err(|e| map_nvml_err("device_by_index", e))?;
        Ok(PyNvmlDevice { inner: dev })
    }

    fn device_by_uuid(&self, uuid: &str) -> PyResult<PyNvmlDevice> {
        let dev = self
            .inner
            .device_by_uuid(uuid)
            .map_err(|e| map_nvml_err("device_by_uuid", e))?;
        Ok(PyNvmlDevice { inner: dev })
    }
}

/// Handle to one NVML GPU device.
#[pyclass(name = "NvmlDevice", module = "nv_rs.nvml")]
pub struct PyNvmlDevice {
    inner: Device,
}

#[pymethods]
impl PyNvmlDevice {
    fn index(&self) -> PyResult<u32> {
        self.inner.index().map_err(|e| map_nvml_err("index", e))
    }

    fn name(&self) -> PyResult<String> {
        self.inner.name().map_err(|e| map_nvml_err("name", e))
    }

    fn uuid(&self) -> PyResult<String> {
        self.inner.uuid().map_err(|e| map_nvml_err("uuid", e))
    }

    fn compute_capability(&self) -> PyResult<(i32, i32)> {
        self.inner
            .cuda_compute_capability()
            .map_err(|e| map_nvml_err("cuda_compute_capability", e))
    }

    fn temperature_c(&self) -> PyResult<u32> {
        self.inner
            .temperature_gpu()
            .map_err(|e| map_nvml_err("temperature_gpu", e))
    }

    fn power_usage_mw(&self) -> PyResult<u32> {
        self.inner
            .power_usage_mw()
            .map_err(|e| map_nvml_err("power_usage_mw", e))
    }

    fn power_limit_mw(&self) -> PyResult<u32> {
        self.inner
            .power_limit_mw()
            .map_err(|e| map_nvml_err("power_limit_mw", e))
    }

    /// @py-return: dict[str, int]
    fn memory_info(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let mem = self
            .inner
            .memory_info()
            .map_err(|e| map_nvml_err("memory_info", e))?;
        Ok(memory_dict(py, &mem)?.unbind())
    }

    /// @py-return: dict[str, int]
    fn utilization(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let u = self
            .inner
            .utilization_rates()
            .map_err(|e| map_nvml_err("utilization_rates", e))?;
        Ok(utilization_dict(py, &u)?.unbind())
    }

    /// Clock of each domain in MHz; `None` where the device does not support the query.
    /// @py-return: dict[str, int | None]
    fn clocks_mhz(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let d = PyDict::new(py);
        for (key, domain) in [
            ("graphics", ClockDomain::Graphics),
            ("sm", ClockDomain::Sm),
            ("memory", ClockDomain::Memory),
        ] {
            d.set_item(key, optional("clock_mhz", self.inner.clock_mhz(domain))?)?;
        }
        Ok(d.unbind())
    }

    /// Identity plus telemetry; a telemetry field is `None` only where the device does not
    /// support it, any other NVML error raises.
    /// @py-return: dict[str, Any]
    fn summary(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let d = PyDict::new(py);
        d.set_item("index", self.index()?)?;
        d.set_item("name", self.name()?)?;
        d.set_item("uuid", self.uuid()?)?;
        d.set_item(
            "compute_capability",
            optional(
                "cuda_compute_capability",
                self.inner.cuda_compute_capability(),
            )?,
        )?;
        d.set_item(
            "temperature_c",
            optional("temperature_gpu", self.inner.temperature_gpu())?,
        )?;
        d.set_item(
            "power_usage_mw",
            optional("power_usage_mw", self.inner.power_usage_mw())?,
        )?;
        let mem = optional("memory_info", self.inner.memory_info())?;
        d.set_item("memory", mem.map(|m| memory_dict(py, &m)).transpose()?)?;
        let util = optional("utilization_rates", self.inner.utilization_rates())?;
        d.set_item(
            "utilization",
            util.map(|u| utilization_dict(py, &u)).transpose()?,
        )?;
        Ok(d.unbind())
    }
}

/// Registers the `nv_rs.nvml` submodule.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    py_register_submodule!(py, parent, "nvml", |m| {
        m.add_class::<PyNvml>()?;
        m.add_class::<PyNvmlDevice>()?;
    })
}
