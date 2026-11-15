//! `nv_rs.nvml` — safe NVML GPU telemetry and enumeration.

use std::sync::Arc;

use nvml::{ClockDomain, Device, Nvml};
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::errors::{ErrorKind, record_and_err};

fn map_nvml_err(op: &str, err: nvml::Error) -> PyErr {
    let kind = match &err {
        nvml::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
        nvml::Error::InvalidInput(_) => ErrorKind::InvalidArgument,
        _ => ErrorKind::Cuda,
    };
    record_and_err("nvml", op, err, kind)
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

    fn memory_info(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let mem = self
            .inner
            .memory_info()
            .map_err(|e| map_nvml_err("memory_info", e))?;
        let d = PyDict::new(py);
        d.set_item("total", mem.total)?;
        d.set_item("free", mem.free)?;
        d.set_item("used", mem.used)?;
        Ok(d.unbind())
    }

    fn utilization(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let u = self
            .inner
            .utilization_rates()
            .map_err(|e| map_nvml_err("utilization_rates", e))?;
        let d = PyDict::new(py);
        d.set_item("gpu", u.gpu)?;
        d.set_item("memory", u.memory)?;
        Ok(d.unbind())
    }

    fn clocks_mhz(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let d = PyDict::new(py);
        d.set_item("graphics", self.inner.clock_mhz(ClockDomain::Graphics).ok())?;
        d.set_item("sm", self.inner.clock_mhz(ClockDomain::Sm).ok())?;
        d.set_item("memory", self.inner.clock_mhz(ClockDomain::Memory).ok())?;
        Ok(d.unbind())
    }

    fn summary(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let d = PyDict::new(py);
        d.set_item("index", self.index()?)?;
        d.set_item("name", self.name()?)?;
        d.set_item("uuid", self.uuid()?)?;
        d.set_item("compute_capability", self.compute_capability().ok())?;
        d.set_item("temperature_c", self.temperature_c().ok())?;
        d.set_item("power_usage_mw", self.power_usage_mw().ok())?;
        d.set_item("memory", self.memory_info(py).ok())?;
        d.set_item("utilization", self.utilization(py).ok())?;
        Ok(d.unbind())
    }
}

/// Registers the `nv_rs.nvml` submodule.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(py, "nvml")?;
    m.add_class::<PyNvml>()?;
    m.add_class::<PyNvmlDevice>()?;
    parent.add_submodule(&m)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("nv_rs.nvml", m)?;
    Ok(())
}
