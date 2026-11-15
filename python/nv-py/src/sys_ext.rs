//! `nv_rs.cupti`, `nv_rs.cuda_ext`, and `nv_rs.wgpu` — CUPTI profiling, CUDA driver extensions,
//! and zero-copy CUDA <-> `wgpu` interop submodules.

use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context as TaskContext, Poll, Wake, Waker};
use std::thread::Thread;
use std::time::Instant;

use cuda_core::Context;
use cuda_core_ext::{
    GreenContext, HostRegistration, ManagedBuffer, MemoryHandle, device_luid as ext_device_luid,
    device_uuid as ext_device_uuid, sparse_arrays_supported as ext_sparse_arrays_supported,
    split_sms as ext_split_sms,
};
use cupti::{
    ActivityKind, ActivityProfiler, ActivityRecord, CallbackDomain, Subscriber,
    subscriber::callback_name as cupti_callback_name, version as cupti_version,
};
use nv_loader::diag::{self, Level};
use nv_wgpu::{
    InteropBackend, InteropContext, InteropSemaphore, SharedBuffer, SharedBufferDescriptor,
    aligned_row_pitch, aligned_staging_bytes, validate_buffer_descriptor,
};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

use crate::core::{PyContext, PyStream};
use crate::errors::{ErrorKind, record_and_err};

// ============================================================================
// Shared helpers
// ============================================================================

fn resolve_cuda_ctx(
    category: &str,
    op: &str,
    context: Option<&PyContext>,
    device: usize,
) -> PyResult<Arc<Context>> {
    match context {
        Some(ctx) => Ok(ctx.inner.clone()),
        None => Context::new(device).map_err(|e| record_and_err(category, op, e, ErrorKind::Cuda)),
    }
}

fn register_submodule(
    py: Python<'_>,
    parent: &Bound<'_, PyModule>,
    full_name: &str,
    m: &Bound<'_, PyModule>,
) -> PyResult<()> {
    parent.add_submodule(m)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item(full_name, m)?;
    Ok(())
}

// ============================================================================
// 1. nv_rs.cupti
// ============================================================================

fn map_cupti_err(op: &str, err: cupti::Error) -> PyErr {
    let kind = match &err {
        cupti::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
        cupti::Error::InvalidInput(_) => ErrorKind::InvalidArgument,
        _ => ErrorKind::Cuda,
    };
    record_and_err("cupti", op, err, kind)
}

fn parse_activity_kind(kind: &str) -> PyResult<ActivityKind> {
    match kind.trim().to_ascii_lowercase().as_str() {
        "memcpy" => Ok(ActivityKind::Memcpy),
        "memset" => Ok(ActivityKind::Memset),
        "kernel" => Ok(ActivityKind::Kernel),
        "concurrent_kernel" | "concurrentkernel" => Ok(ActivityKind::ConcurrentKernel),
        "driver" => Ok(ActivityKind::Driver),
        "runtime" => Ok(ActivityKind::Runtime),
        _ => Err(record_and_err(
            "cupti",
            "parse_activity_kind",
            format!("unknown CUPTI activity kind `{kind}`"),
            ErrorKind::InvalidArgument,
        )),
    }
}

fn parse_callback_domain(domain: &str) -> PyResult<CallbackDomain> {
    match domain.trim().to_ascii_lowercase().as_str() {
        "driver_api" | "driverapi" | "driver" => Ok(CallbackDomain::DriverApi),
        "runtime_api" | "runtimeapi" | "runtime" => Ok(CallbackDomain::RuntimeApi),
        "resource" => Ok(CallbackDomain::Resource),
        "synchronize" | "sync" => Ok(CallbackDomain::Synchronize),
        "nvtx" => Ok(CallbackDomain::Nvtx),
        "state" => Ok(CallbackDomain::State),
        _ => Err(record_and_err(
            "cupti",
            "parse_callback_domain",
            format!("unknown CUPTI callback domain `{domain}`"),
            ErrorKind::InvalidArgument,
        )),
    }
}

fn callback_domain_str(domain: CallbackDomain) -> &'static str {
    match domain {
        CallbackDomain::DriverApi => "driver_api",
        CallbackDomain::RuntimeApi => "runtime_api",
        CallbackDomain::Resource => "resource",
        CallbackDomain::Synchronize => "synchronize",
        CallbackDomain::Nvtx => "nvtx",
        CallbackDomain::State => "state",
    }
}

fn activity_record_to_py(py: Python<'_>, rec: ActivityRecord) -> PyResult<Py<PyDict>> {
    let d = PyDict::new(py);
    match rec {
        ActivityRecord::Kernel {
            name,
            start_ns,
            end_ns,
            device_id,
            context_id,
            stream_id,
            correlation_id,
        } => {
            d.set_item("kind", "kernel")?;
            d.set_item("name", name)?;
            d.set_item("start_ns", start_ns)?;
            d.set_item("end_ns", end_ns)?;
            d.set_item("device_id", device_id)?;
            d.set_item("context_id", context_id)?;
            d.set_item("stream_id", stream_id)?;
            d.set_item("correlation_id", correlation_id)?;
        }
        ActivityRecord::Memcpy {
            copy_kind,
            bytes,
            start_ns,
            end_ns,
            device_id,
            context_id,
            stream_id,
        } => {
            d.set_item("kind", "memcpy")?;
            d.set_item("copy_kind", copy_kind)?;
            d.set_item("bytes", bytes)?;
            d.set_item("start_ns", start_ns)?;
            d.set_item("end_ns", end_ns)?;
            d.set_item("device_id", device_id)?;
            d.set_item("context_id", context_id)?;
            d.set_item("stream_id", stream_id)?;
        }
        ActivityRecord::Api {
            cbid,
            thread_id,
            correlation_id,
            return_value,
        } => {
            d.set_item("kind", "api")?;
            d.set_item("cbid", cbid)?;
            d.set_item("thread_id", thread_id)?;
            d.set_item("correlation_id", correlation_id)?;
            d.set_item("return_value", return_value)?;
        }
        ActivityRecord::Other { kind } => {
            d.set_item("kind", "other")?;
            d.set_item("raw_kind", kind)?;
        }
    }
    Ok(d.unbind())
}

/// Returns the loaded CUPTI API version (`cuptiGetVersion`).
#[pyfunction(name = "version")]
fn cupti_version_py(py: Python<'_>) -> PyResult<u32> {
    py.detach(cupti_version)
        .map_err(|e| map_cupti_err("version", e))
}

/// Looks up the human-readable name of a callback ID (`cuptiGetCallbackName`).
#[pyfunction]
fn callback_name(py: Python<'_>, domain: &str, cbid: u32) -> PyResult<String> {
    let dom = parse_callback_domain(domain)?;
    py.detach(move || cupti_callback_name(dom, cbid))
        .map_err(|e| map_cupti_err("callback_name", e))
}

/// CUPTI Activity API profiler (`cupti::ActivityProfiler`).
#[pyclass(name = "CuptiActivity", module = "nv_rs.cupti")]
pub struct PyCuptiActivity {
    inner: Mutex<Option<ActivityProfiler>>,
}

#[pymethods]
impl PyCuptiActivity {
    /// Registers CUPTI activity callbacks and binds an `ActivityProfiler` to `context` (or `device`).
    #[new]
    #[pyo3(signature = (context=None, device=0))]
    fn new(py: Python<'_>, context: Option<&PyContext>, device: usize) -> PyResult<Self> {
        let t0 = Instant::now();
        let ctx = resolve_cuda_ctx("cupti", "CuptiActivity::new", context, device)?;
        let profiler = py
            .detach(move || ActivityProfiler::new(&ctx))
            .map_err(|e| map_cupti_err("CuptiActivity::new", e))?;
        diag::record_event(
            Level::Info,
            "cupti",
            "activity_new",
            format!("device={device}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(Self {
            inner: Mutex::new(Some(profiler)),
        })
    }

    /// Enables activity collection for `kind` (`"kernel"`, `"concurrent_kernel"`, `"memcpy"`,
    /// `"memset"`, `"driver"`, `"runtime"`).
    fn enable(&self, py: Python<'_>, kind: &str) -> PyResult<()> {
        let parsed = parse_activity_kind(kind)?;
        py.detach(|| {
            let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            let profiler = guard
                .as_mut()
                .ok_or(cupti::Error::InvalidInput("CuptiActivity is closed"))?;
            profiler.enable(parsed)
        })
        .map_err(|e| map_cupti_err("enable", e))
    }

    /// Disables activity collection for `kind`.
    fn disable(&self, py: Python<'_>, kind: &str) -> PyResult<()> {
        let parsed = parse_activity_kind(kind)?;
        py.detach(|| {
            let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            let profiler = guard
                .as_mut()
                .ok_or(cupti::Error::InvalidInput("CuptiActivity is closed"))?;
            profiler.disable(parsed)
        })
        .map_err(|e| map_cupti_err("disable", e))
    }

    /// Flushes all pending CUPTI activity records (`cuptiActivityFlushAll`).
    #[pyo3(signature = (forced=true))]
    fn flush(&self, py: Python<'_>, forced: bool) -> PyResult<()> {
        let t0 = Instant::now();
        py.detach(|| {
            let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            let profiler = guard
                .as_ref()
                .ok_or(cupti::Error::InvalidInput("CuptiActivity is closed"))?;
            profiler.flush(forced)
        })
        .map_err(|e| map_cupti_err("flush", e))?;
        diag::record_event(
            Level::Trace,
            "cupti",
            "activity_flush",
            format!("forced={forced}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(())
    }

    /// Drains all collected activity records, returning `(records, dropped_count)`.
    fn drain_records(&self, py: Python<'_>) -> PyResult<(Vec<Py<PyDict>>, usize)> {
        let (records, dropped) = py
            .detach(|| {
                let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
                let profiler = guard
                    .as_ref()
                    .ok_or(cupti::Error::InvalidInput("CuptiActivity is closed"))?;
                profiler.drain_records()
            })
            .map_err(|e| map_cupti_err("drain_records", e))?;

        let mut out = Vec::with_capacity(records.len());
        for rec in records {
            out.push(activity_record_to_py(py, rec)?);
        }
        Ok((out, dropped))
    }

    /// Explicitly tears down the profiler and flushes remaining activity buffers.
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        py.detach(|| {
            let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            drop(guard.take());
        });
        Ok(())
    }
}

/// CUPTI callback subscriber (`cupti::Subscriber`).
#[pyclass(name = "CuptiSubscriber", module = "nv_rs.cupti")]
pub struct PyCuptiSubscriber {
    inner: Mutex<Option<Subscriber>>,
    events: Arc<Mutex<Vec<(&'static str, u32)>>>,
    callback_err: Arc<Mutex<Option<String>>>,
}

#[pymethods]
impl PyCuptiSubscriber {
    /// Subscribes to CUPTI callbacks on `context` (or `device`).
    ///
    /// If `callback` is provided, it is invoked as `callback(domain: str, cbid: int)` and errors
    /// are captured for `check_errors()`. Observed `(domain, cbid)` pairs are also buffered for
    /// `drain_events()`.
    #[new]
    #[pyo3(signature = (context=None, device=0, callback=None))]
    fn new(
        py: Python<'_>,
        context: Option<&PyContext>,
        device: usize,
        callback: Option<Py<PyAny>>,
    ) -> PyResult<Self> {
        let t0 = Instant::now();
        let ctx = resolve_cuda_ctx("cupti", "CuptiSubscriber::new", context, device)?;
        let events = Arc::new(Mutex::new(Vec::new()));
        let callback_err = Arc::new(Mutex::new(None));

        let events_cb = events.clone();
        let err_cb = callback_err.clone();
        let cb: Box<cupti::CallbackFn> = Box::new(move |dom, cbid| {
            let dom_str = callback_domain_str(dom);
            {
                let mut ev = events_cb.lock().unwrap_or_else(|e| e.into_inner());
                ev.push((dom_str, cbid));
            }
            if let Some(ref py_cb) = callback {
                let res = Python::attach(|py| py_cb.call1(py, (dom_str, cbid)).map(|_| ()));
                if let Err(err) = res {
                    let mut slot = err_cb.lock().unwrap_or_else(|e| e.into_inner());
                    slot.get_or_insert_with(|| err.to_string());
                }
            }
        });

        let sub = py
            .detach(move || Subscriber::new(&ctx, cb))
            .map_err(|e| map_cupti_err("CuptiSubscriber::new", e))?;

        diag::record_event(
            Level::Info,
            "cupti",
            "subscriber_new",
            format!("device={device}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(Self {
            inner: Mutex::new(Some(sub)),
            events,
            callback_err,
        })
    }

    /// Enables or disables all callbacks in `domain`.
    fn enable_domain(&self, py: Python<'_>, domain: &str, enable: bool) -> PyResult<()> {
        let dom = parse_callback_domain(domain)?;
        py.detach(|| {
            let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            let sub = guard
                .as_ref()
                .ok_or(cupti::Error::InvalidInput("CuptiSubscriber is closed"))?;
            sub.enable_domain(dom, enable)
        })
        .map_err(|e| map_cupti_err("enable_domain", e))
    }

    /// Enables or disables a single callback `cbid` in `domain`.
    fn enable_callback(
        &self,
        py: Python<'_>,
        domain: &str,
        cbid: u32,
        enable: bool,
    ) -> PyResult<()> {
        let dom = parse_callback_domain(domain)?;
        py.detach(|| {
            let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            let sub = guard
                .as_ref()
                .ok_or(cupti::Error::InvalidInput("CuptiSubscriber is closed"))?;
            sub.enable_callback(dom, cbid, enable)
        })
        .map_err(|e| map_cupti_err("enable_callback", e))
    }

    /// Drains buffered `(domain, cbid)` callback events.
    fn drain_events(&self) -> Vec<(&'static str, u32)> {
        let mut ev = self.events.lock().unwrap_or_else(|e| e.into_inner());
        std::mem::take(&mut *ev)
    }

    /// Raises an error if any Rust panic or Python callback exception occurred inside CUPTI.
    fn check_errors(&self) -> PyResult<()> {
        {
            let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(sub) = guard.as_ref() {
                sub.check_panic()
                    .map_err(|e| map_cupti_err("check_errors", e))?;
            }
        }
        let mut slot = self.callback_err.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(msg) = slot.take() {
            return Err(record_and_err("cupti", "callback", msg, ErrorKind::Runtime));
        }
        Ok(())
    }

    /// Explicitly unsubscribes and releases the subscriber handle.
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        py.detach(|| {
            let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            drop(guard.take());
        });
        Ok(())
    }
}

fn register_cupti(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(py, "cupti")?;
    m.add_function(wrap_pyfunction!(cupti_version_py, &m)?)?;
    m.add_function(wrap_pyfunction!(callback_name, &m)?)?;
    m.add_class::<PyCuptiActivity>()?;
    m.add_class::<PyCuptiSubscriber>()?;
    register_submodule(py, parent, "nv_rs.cupti", &m)
}

// ============================================================================
// 2. nv_rs.cuda_ext
// ============================================================================

/// Returns `(luid_bytes, node_mask)` for `device` (Windows only; raises `NvCudaError` elsewhere).
#[pyfunction]
#[pyo3(signature = (device=0))]
fn device_luid<'py>(py: Python<'py>, device: usize) -> PyResult<(Bound<'py, PyBytes>, u32)> {
    let luid = py
        .detach(move || {
            let ctx = Context::new(device)?;
            ext_device_luid(&ctx)
        })
        .map_err(|e| record_and_err("cuda_ext", "device_luid", e, ErrorKind::Cuda))?;
    Ok((PyBytes::new(py, &luid.bytes), luid.node_mask))
}

/// Returns the 16-byte hardware UUID for `device` (`cuDeviceGetUuid_v2`).
#[pyfunction]
#[pyo3(signature = (device=0))]
fn device_uuid<'py>(py: Python<'py>, device: usize) -> PyResult<Bound<'py, PyBytes>> {
    let uuid = py
        .detach(move || {
            let ctx = Context::new(device)?;
            ext_device_uuid(&ctx)
        })
        .map_err(|e| record_and_err("cuda_ext", "device_uuid", e, ErrorKind::Cuda))?;
    Ok(PyBytes::new(py, &uuid))
}

/// Returns whether `device` supports sparse (tiled) CUDA arrays.
#[pyfunction]
#[pyo3(signature = (device=0))]
fn sparse_arrays_supported(py: Python<'_>, device: usize) -> PyResult<bool> {
    py.detach(move || {
        let ctx = Context::new(device)?;
        ext_sparse_arrays_supported(&ctx)
    })
    .map_err(|e| record_and_err("cuda_ext", "sparse_arrays_supported", e, ErrorKind::Cuda))
}

/// Unified/managed memory buffer (`cuda_core_ext::ManagedBuffer<u8>`).
#[pyclass(name = "ManagedBuffer", module = "nv_rs.cuda_ext")]
pub struct PyManagedBuffer {
    inner: Mutex<ManagedBuffer<u8>>,
}

#[pymethods]
impl PyManagedBuffer {
    /// Allocates `nbytes` zeroed bytes of CUDA managed memory.
    #[staticmethod]
    #[pyo3(signature = (nbytes, context=None, device=0))]
    fn zeroed(
        py: Python<'_>,
        nbytes: usize,
        context: Option<&PyContext>,
        device: usize,
    ) -> PyResult<Self> {
        let t0 = Instant::now();
        let ctx = resolve_cuda_ctx("cuda_ext", "ManagedBuffer::zeroed", context, device)?;
        let buf = py
            .detach(move || ManagedBuffer::<u8>::zeroed(&ctx, nbytes))
            .map_err(|e| record_and_err("cuda_ext", "ManagedBuffer::zeroed", e, ErrorKind::Cuda))?;
        diag::record_event(
            Level::Trace,
            "cuda_ext",
            "managed_zeroed",
            format!("nbytes={nbytes}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(Self {
            inner: Mutex::new(buf),
        })
    }

    #[getter]
    fn ptr(&self) -> u64 {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .cu_deviceptr()
    }

    #[getter]
    fn nbytes(&self) -> usize {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).len()
    }

    /// Reads the entire managed buffer as host `bytes` (caller must synchronize GPU work first).
    fn to_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        PyBytes::new(py, guard.as_slice())
    }

    /// Writes `data` into the managed buffer starting at `offset` (caller must synchronize first).
    #[pyo3(signature = (data, offset=0))]
    fn write_bytes(&self, data: &[u8], offset: usize) -> PyResult<()> {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let slice = guard.as_mut_slice();
        let slice_len = slice.len();
        let end = offset.checked_add(data.len()).ok_or_else(|| {
            record_and_err(
                "cuda_ext",
                "ManagedBuffer::write_bytes",
                "offset + data length overflow",
                ErrorKind::InvalidArgument,
            )
        })?;
        let dst = slice.get_mut(offset..end).ok_or_else(|| {
            record_and_err(
                "cuda_ext",
                "ManagedBuffer::write_bytes",
                format!("write range {offset}..{end} exceeds buffer size {slice_len}"),
                ErrorKind::InvalidArgument,
            )
        })?;
        dst.copy_from_slice(data);
        Ok(())
    }

    fn __repr__(&self) -> String {
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        format!(
            "ManagedBuffer(ptr=0x{:x}, nbytes={})",
            guard.cu_deviceptr(),
            guard.len()
        )
    }
}

struct HostRegInner {
    reg: ManuallyDrop<HostRegistration<'static, u8>>,
    // Dropped after `reg` so `cuMemHostUnregister` executes while the pinned slice is live.
    _backing: Pin<Box<[u8]>>,
}

// SAFETY: `HostRegInner` is guarded by a `Mutex` in `PyHostRegistration`; `_backing` is pinned
// on the heap and never moved, and `HostRegistration` binds its owning `Context` on drop.
unsafe impl Send for HostRegInner {}

impl Drop for HostRegInner {
    fn drop(&mut self) {
        // SAFETY: `self.reg` was created from `self._backing` and is unregistered here before
        // `self._backing` is dropped.
        unsafe { ManuallyDrop::drop(&mut self.reg) };
    }
}

/// Page-locked host buffer registered with CUDA (`cuda_core_ext::HostRegistration`).
#[pyclass(name = "HostRegistration", module = "nv_rs.cuda_ext")]
pub struct PyHostRegistration {
    inner: Mutex<Option<HostRegInner>>,
}

#[pymethods]
impl PyHostRegistration {
    /// Copies `data` into a pinned heap allocation and page-locks it with `cuMemHostRegister`.
    #[new]
    #[pyo3(signature = (data, map_to_device=true, context=None, device=0))]
    fn new(
        py: Python<'_>,
        data: &[u8],
        map_to_device: bool,
        context: Option<&PyContext>,
        device: usize,
    ) -> PyResult<Self> {
        if data.is_empty() {
            return Err(record_and_err(
                "cuda_ext",
                "HostRegistration::new",
                "cannot register empty byte buffer",
                ErrorKind::InvalidArgument,
            ));
        }
        let ctx = resolve_cuda_ctx("cuda_ext", "HostRegistration::new", context, device)?;
        let mut backing: Pin<Box<[u8]>> = Pin::new(data.to_vec().into_boxed_slice());

        let inner = py
            .detach(move || {
                let ptr = backing.as_mut_ptr();
                let len = backing.len();
                // SAFETY: `backing` is a pinned heap allocation owned by `HostRegInner` and
                // dropped strictly after `reg` in `HostRegInner::drop`.
                let slice_static: &'static mut [u8] =
                    unsafe { std::slice::from_raw_parts_mut(ptr, len) };
                let reg = HostRegistration::new(&ctx, slice_static, map_to_device)?;
                Ok::<_, cuda_core::DriverError>(HostRegInner {
                    reg: ManuallyDrop::new(reg),
                    _backing: backing,
                })
            })
            .map_err(|e| record_and_err("cuda_ext", "HostRegistration::new", e, ErrorKind::Cuda))?;

        Ok(Self {
            inner: Mutex::new(Some(inner)),
        })
    }

    #[getter]
    fn device_ptr(&self) -> Option<u64> {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .and_then(|i| i.reg.device_ptr())
    }

    #[getter]
    fn nbytes(&self) -> usize {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .map_or(0, |i| i.reg.as_slice().len())
    }

    fn to_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let inner = guard.as_ref().ok_or_else(|| {
            record_and_err(
                "cuda_ext",
                "HostRegistration::to_bytes",
                "HostRegistration is closed",
                ErrorKind::InvalidArgument,
            )
        })?;
        Ok(PyBytes::new(py, inner.reg.as_slice()))
    }

    /// Unregisters and frees the page-locked buffer.
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        py.detach(|| {
            let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            drop(guard.take());
        });
        Ok(())
    }
}

/// Green context SM partition (`cuda_core_ext::GreenContext`).
#[pyclass(name = "GreenContext", module = "nv_rs.cuda_ext")]
pub struct PyGreenContext {
    inner: GreenContext,
    device_id: i32,
}

#[pymethods]
impl PyGreenContext {
    #[getter]
    fn sm_count(&self) -> u32 {
        self.inner.sm_count()
    }

    #[getter]
    fn device_id(&self) -> i32 {
        self.device_id
    }

    /// Creates a non-blocking `nv_rs.core.Stream` bound to this SM partition.
    fn new_stream(&self, py: Python<'_>) -> PyResult<PyStream> {
        let stream = py.detach(|| self.inner.stream()).map_err(|e| {
            record_and_err("cuda_ext", "GreenContext::new_stream", e, ErrorKind::Cuda)
        })?;
        Ok(PyStream {
            inner: stream,
            device_id: self.device_id,
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "GreenContext(device_id={}, sm_count={})",
            self.device_id,
            self.inner.sm_count()
        )
    }
}

/// Splits a GPU's SMs into green contexts of at least `min_sms` SMs each.
#[pyfunction]
#[pyo3(signature = (min_sms, context=None, device=0))]
fn split_sms(
    py: Python<'_>,
    min_sms: u32,
    context: Option<&PyContext>,
    device: usize,
) -> PyResult<Vec<PyGreenContext>> {
    let t0 = Instant::now();
    let ctx = resolve_cuda_ctx("cuda_ext", "split_sms", context, device)?;
    let dev_id = ctx.ordinal() as i32;
    let groups = py
        .detach(move || ext_split_sms(&ctx, min_sms))
        .map_err(|e| record_and_err("cuda_ext", "split_sms", e, ErrorKind::Cuda))?;
    diag::record_event(
        Level::Info,
        "cuda_ext",
        "split_sms",
        format!("min_sms={min_sms} groups={}", groups.len()),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    Ok(groups
        .into_iter()
        .map(|inner| PyGreenContext {
            inner,
            device_id: dev_id,
        })
        .collect())
}

/// Validated descriptor for an external memory handle (`cuda_core_ext::MemoryHandle`).
#[pyclass(name = "ExternalMemoryDesc", module = "nv_rs.cuda_ext")]
#[derive(Clone)]
pub struct PyExternalMemoryDesc {
    handle_kind: String,
    size: u64,
    dedicated: bool,
}

#[pymethods]
impl PyExternalMemoryDesc {
    #[new]
    #[pyo3(signature = (handle_kind, size, dedicated=false))]
    fn new(handle_kind: &str, size: u64, dedicated: bool) -> PyResult<Self> {
        let norm = handle_kind.trim().to_ascii_lowercase();
        let valid = matches!(
            norm.as_str(),
            "opaque_win32" | "opaque_fd" | "d3d12_resource" | "d3d12_heap"
        );
        if !valid {
            return Err(record_and_err(
                "cuda_ext",
                "ExternalMemoryDesc::new",
                format!("unknown external memory handle_kind `{handle_kind}`"),
                ErrorKind::InvalidArgument,
            ));
        }
        if size == 0 {
            return Err(record_and_err(
                "cuda_ext",
                "ExternalMemoryDesc::new",
                "external memory size must be non-zero",
                ErrorKind::InvalidArgument,
            ));
        }
        if norm == "d3d12_resource" && !dedicated {
            return Err(record_and_err(
                "cuda_ext",
                "ExternalMemoryDesc::new",
                "d3d12_resource external memory requires dedicated=True",
                ErrorKind::InvalidArgument,
            ));
        }
        // Reference `MemoryHandle` variants so the mapping stays aligned with `cuda-core-ext`.
        let _ = std::mem::size_of::<MemoryHandle>();
        Ok(Self {
            handle_kind: norm,
            size,
            dedicated,
        })
    }

    #[getter]
    fn handle_kind(&self) -> &str {
        &self.handle_kind
    }

    #[getter]
    fn size(&self) -> u64 {
        self.size
    }

    #[getter]
    fn dedicated(&self) -> bool {
        self.dedicated
    }

    fn __repr__(&self) -> String {
        format!(
            "ExternalMemoryDesc(handle_kind='{}', size={}, dedicated={})",
            self.handle_kind, self.size, self.dedicated
        )
    }
}

fn register_cuda_ext(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(py, "cuda_ext")?;
    m.add_function(wrap_pyfunction!(device_luid, &m)?)?;
    m.add_function(wrap_pyfunction!(device_uuid, &m)?)?;
    m.add_function(wrap_pyfunction!(sparse_arrays_supported, &m)?)?;
    m.add_function(wrap_pyfunction!(split_sms, &m)?)?;
    m.add_class::<PyManagedBuffer>()?;
    m.add_class::<PyHostRegistration>()?;
    m.add_class::<PyGreenContext>()?;
    m.add_class::<PyExternalMemoryDesc>()?;
    register_submodule(py, parent, "nv_rs.cuda_ext", &m)
}

// ============================================================================
// 3. nv_rs.wgpu
// ============================================================================

struct ThreadWake(Thread);

impl Wake for ThreadWake {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: std::future::Future>(fut: F) -> F::Output {
    let waker = Waker::from(Arc::new(ThreadWake(std::thread::current())));
    let mut cx = TaskContext::from_waker(&waker);
    let mut fut = std::pin::pin!(fut);
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(out) => return out,
            Poll::Pending => std::thread::park(),
        }
    }
}

fn map_wgpu_err(op: &str, err: nv_wgpu::Error) -> PyErr {
    let kind = match &err {
        nv_wgpu::Error::InvalidInput(_) => ErrorKind::InvalidArgument,
        nv_wgpu::Error::Unsupported { .. }
        | nv_wgpu::Error::BackendMismatch { .. }
        | nv_wgpu::Error::DeviceMismatch { .. }
        | nv_wgpu::Error::NoMatchingCudaDevice { .. } => ErrorKind::InvalidArgument,
        _ => ErrorKind::Cuda,
    };
    record_and_err("wgpu", op, err, kind)
}

fn parse_texture_format(fmt: &str) -> PyResult<wgpu::TextureFormat> {
    match fmt.trim().to_ascii_lowercase().as_str() {
        "r8unorm" | "r8_unorm" => Ok(wgpu::TextureFormat::R8Unorm),
        "r8uint" | "r8_uint" => Ok(wgpu::TextureFormat::R8Uint),
        "rg8unorm" | "rg8_unorm" => Ok(wgpu::TextureFormat::Rg8Unorm),
        "rg8uint" | "rg8_uint" => Ok(wgpu::TextureFormat::Rg8Uint),
        "rgba8unorm" | "rgba8_unorm" => Ok(wgpu::TextureFormat::Rgba8Unorm),
        "rgba8unormsrgb" | "rgba8_unorm_srgb" => Ok(wgpu::TextureFormat::Rgba8UnormSrgb),
        "rgba8uint" | "rgba8_uint" => Ok(wgpu::TextureFormat::Rgba8Uint),
        "bgra8unorm" | "bgra8_unorm" => Ok(wgpu::TextureFormat::Bgra8Unorm),
        "bgra8unormsrgb" | "bgra8_unorm_srgb" => Ok(wgpu::TextureFormat::Bgra8UnormSrgb),
        "r16unorm" | "r16_unorm" => Ok(wgpu::TextureFormat::R16Unorm),
        "r16uint" | "r16_uint" => Ok(wgpu::TextureFormat::R16Uint),
        "r16float" | "r16_float" => Ok(wgpu::TextureFormat::R16Float),
        "rg16unorm" | "rg16_unorm" => Ok(wgpu::TextureFormat::Rg16Unorm),
        "rg16uint" | "rg16_uint" => Ok(wgpu::TextureFormat::Rg16Uint),
        "rg16float" | "rg16_float" => Ok(wgpu::TextureFormat::Rg16Float),
        "rgba16unorm" | "rgba16_unorm" => Ok(wgpu::TextureFormat::Rgba16Unorm),
        "rgba16uint" | "rgba16_uint" => Ok(wgpu::TextureFormat::Rgba16Uint),
        "rgba16float" | "rgba16_float" => Ok(wgpu::TextureFormat::Rgba16Float),
        "r32uint" | "r32_uint" => Ok(wgpu::TextureFormat::R32Uint),
        "r32sint" | "r32_sint" => Ok(wgpu::TextureFormat::R32Sint),
        "r32float" | "r32_float" => Ok(wgpu::TextureFormat::R32Float),
        "rg32uint" | "rg32_uint" => Ok(wgpu::TextureFormat::Rg32Uint),
        "rg32sint" | "rg32_sint" => Ok(wgpu::TextureFormat::Rg32Sint),
        "rg32float" | "rg32_float" => Ok(wgpu::TextureFormat::Rg32Float),
        "rgba32uint" | "rgba32_uint" => Ok(wgpu::TextureFormat::Rgba32Uint),
        "rgba32sint" | "rgba32_sint" => Ok(wgpu::TextureFormat::Rgba32Sint),
        "rgba32float" | "rgba32_float" => Ok(wgpu::TextureFormat::Rgba32Float),
        _ => Err(record_and_err(
            "wgpu",
            "parse_texture_format",
            format!("unsupported wgpu texture format `{fmt}`"),
            ErrorKind::InvalidArgument,
        )),
    }
}

fn parse_buffer_usage(usage: &str) -> PyResult<wgpu::BufferUsages> {
    let mut flags = wgpu::BufferUsages::empty();
    for part in usage.split(['|', ',']) {
        let tok = part.trim().to_ascii_lowercase();
        if tok.is_empty() {
            continue;
        }
        match tok.as_str() {
            "storage" => flags |= wgpu::BufferUsages::STORAGE,
            "uniform" => flags |= wgpu::BufferUsages::UNIFORM,
            "vertex" => flags |= wgpu::BufferUsages::VERTEX,
            "index" => flags |= wgpu::BufferUsages::INDEX,
            "indirect" => flags |= wgpu::BufferUsages::INDIRECT,
            "copy_src" => flags |= wgpu::BufferUsages::COPY_SRC,
            "copy_dst" => flags |= wgpu::BufferUsages::COPY_DST,
            "map_read" => flags |= wgpu::BufferUsages::MAP_READ,
            "map_write" => flags |= wgpu::BufferUsages::MAP_WRITE,
            _ => {
                return Err(record_and_err(
                    "wgpu",
                    "parse_buffer_usage",
                    format!("unknown wgpu BufferUsages flag `{tok}`"),
                    ErrorKind::InvalidArgument,
                ));
            }
        }
    }
    Ok(flags)
}

#[derive(Clone)]
struct AdapterProbe {
    name: String,
    backend: String,
    device_type: String,
    vendor: u32,
    device: u32,
    cuda_interop_supported: bool,
    error: Option<String>,
}

fn probe_wgpu_adapters() -> Vec<AdapterProbe> {
    let backends = if cfg!(windows) {
        wgpu::Backends::VULKAN | wgpu::Backends::DX12
    } else {
        wgpu::Backends::VULKAN
    };
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    let adapters = block_on(instance.enumerate_adapters(backends));
    let mut out = Vec::with_capacity(adapters.len());
    for adapter in adapters {
        let info = adapter.get_info();
        let res = match info.backend {
            wgpu::Backend::Vulkan => InteropContext::open_vulkan(
                &adapter,
                wgpu::Features::empty(),
                &wgpu::Limits::default(),
                Some("nv-rs-probe"),
            ),
            #[cfg(windows)]
            wgpu::Backend::Dx12 => {
                match block_on(adapter.request_device(&wgpu::DeviceDescriptor {
                    label: Some("nv-rs-probe"),
                    ..Default::default()
                })) {
                    Ok((device, queue)) => InteropContext::from_wgpu(&adapter, device, queue),
                    Err(e) => Err(nv_wgpu::Error::WgpuRequest(e.to_string())),
                }
            }
            other => Err(nv_wgpu::Error::Unsupported {
                what: "wgpu backend",
                required: "Vulkan or DX12",
                found: format!("{other:?}"),
            }),
        };
        let (cuda_interop_supported, error) = match res {
            Ok(_) => (true, None),
            Err(e) => (false, Some(e.to_string())),
        };
        out.push(AdapterProbe {
            name: info.name,
            backend: format!("{:?}", info.backend),
            device_type: format!("{:?}", info.device_type),
            vendor: info.vendor,
            device: info.device,
            cuda_interop_supported,
            error,
        });
    }
    out
}

/// Returns `True` if at least one `wgpu` adapter can open CUDA <-> `wgpu` zero-copy interop.
#[pyfunction]
fn is_supported(py: Python<'_>) -> bool {
    let t0 = Instant::now();
    let ok = py.detach(|| {
        probe_wgpu_adapters()
            .into_iter()
            .any(|p| p.cuda_interop_supported)
    });
    diag::record_event(
        Level::Info,
        "wgpu",
        "is_supported",
        format!("supported={ok}"),
        t0.elapsed().as_nanos() as u64,
        true,
    );
    ok
}

/// Probes all visible Vulkan/DX12 `wgpu` adapters and their CUDA interop capability.
#[pyfunction]
fn probe_adapters(py: Python<'_>) -> PyResult<Vec<Py<PyDict>>> {
    let probes = py.detach(probe_wgpu_adapters);
    let mut out = Vec::with_capacity(probes.len());
    for p in probes {
        let d = PyDict::new(py);
        d.set_item("name", p.name)?;
        d.set_item("backend", p.backend)?;
        d.set_item("device_type", p.device_type)?;
        d.set_item("vendor", p.vendor)?;
        d.set_item("device", p.device)?;
        d.set_item("cuda_interop_supported", p.cuda_interop_supported)?;
        d.set_item("error", p.error)?;
        out.push(d.unbind());
    }
    Ok(out)
}

/// Computes the 256-byte-aligned row pitch (`bytes_per_row`) for `format` and `width`.
#[pyfunction(name = "aligned_row_pitch")]
fn wgpu_aligned_row_pitch(format: &str, width: u32) -> PyResult<u32> {
    let fmt = parse_texture_format(format)?;
    aligned_row_pitch(fmt, width).map_err(|e| map_wgpu_err("aligned_row_pitch", e))
}

/// Computes the total aligned staging byte size for `format` and `(width, height, depth)`.
#[pyfunction(name = "aligned_staging_bytes")]
#[pyo3(signature = (format, width, height, depth=1))]
fn wgpu_aligned_staging_bytes(format: &str, width: u32, height: u32, depth: u32) -> PyResult<u64> {
    let fmt = parse_texture_format(format)?;
    aligned_staging_bytes(
        fmt,
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: depth,
        },
    )
    .map_err(|e| map_wgpu_err("aligned_staging_bytes", e))
}

/// Paired `wgpu::Device` + `wgpu::Queue` + matching `cuda_core::Context` (`nv_wgpu::InteropContext`).
#[pyclass(name = "InteropContext", module = "nv_rs.wgpu")]
pub struct PyInteropContext {
    inner: InteropContext,
}

#[pymethods]
impl PyInteropContext {
    /// Opens an interop-capable Vulkan or DX12 `wgpu` device paired with a matching CUDA device.
    #[new]
    #[pyo3(signature = (backend="auto", label=None))]
    fn new(py: Python<'_>, backend: &str, label: Option<&str>) -> PyResult<Self> {
        let t0 = Instant::now();
        let norm = backend.trim().to_ascii_lowercase();
        let backends = match norm.as_str() {
            "auto" => {
                if cfg!(windows) {
                    wgpu::Backends::VULKAN | wgpu::Backends::DX12
                } else {
                    wgpu::Backends::VULKAN
                }
            }
            "vulkan" => wgpu::Backends::VULKAN,
            #[cfg(windows)]
            "dx12" => wgpu::Backends::DX12,
            _ => {
                return Err(record_and_err(
                    "wgpu",
                    "InteropContext::new",
                    format!("unsupported backend `{backend}`"),
                    ErrorKind::InvalidArgument,
                ));
            }
        };

        let label_owned = label.map(str::to_owned);
        let ctx = py
            .detach(move || {
                let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                    backends,
                    ..wgpu::InstanceDescriptor::new_without_display_handle()
                });
                let adapters = block_on(instance.enumerate_adapters(backends));
                let mut last_err: Option<nv_wgpu::Error> = None;
                for adapter in adapters {
                    let info = adapter.get_info();
                    let res = match info.backend {
                        wgpu::Backend::Vulkan => InteropContext::open_vulkan(
                            &adapter,
                            wgpu::Features::empty(),
                            &wgpu::Limits::default(),
                            label_owned.as_deref(),
                        ),
                        #[cfg(windows)]
                        wgpu::Backend::Dx12 => {
                            match block_on(adapter.request_device(&wgpu::DeviceDescriptor {
                                label: label_owned.as_deref(),
                                ..Default::default()
                            })) {
                                Ok((device, queue)) => {
                                    InteropContext::from_wgpu(&adapter, device, queue)
                                }
                                Err(e) => Err(nv_wgpu::Error::WgpuRequest(e.to_string())),
                            }
                        }
                        _ => continue,
                    };
                    match res {
                        Ok(c) => return Ok(c),
                        Err(e) => last_err = Some(e),
                    }
                }
                Err(last_err.unwrap_or_else(|| {
                    nv_wgpu::Error::WgpuRequest("no interop-capable wgpu adapter found".to_owned())
                }))
            })
            .map_err(|e| map_wgpu_err("InteropContext::new", e))?;

        diag::record_event(
            Level::Info,
            "wgpu",
            "interop_context_new",
            format!("backend={:?}", ctx.backend()),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(Self { inner: ctx })
    }

    #[getter]
    fn backend(&self) -> &'static str {
        match self.inner.backend() {
            InteropBackend::Vulkan => "vulkan",
            #[cfg(windows)]
            InteropBackend::Dx12 => "dx12",
        }
    }

    #[getter]
    fn cuda_device_id(&self) -> i32 {
        self.inner.cuda_context().ordinal() as i32
    }

    /// Re-verifies that the wrapped `wgpu::Device` still exposes external interop support.
    fn verify(&self) -> PyResult<&'static str> {
        let b = self.inner.verify().map_err(|e| map_wgpu_err("verify", e))?;
        Ok(match b {
            InteropBackend::Vulkan => "vulkan",
            #[cfg(windows)]
            InteropBackend::Dx12 => "dx12",
        })
    }

    /// Creates a zero-copy shared GPU buffer (`SharedBuffer`).
    #[pyo3(signature = (size, usage="storage|copy_src|copy_dst", label=None))]
    fn create_buffer(
        &self,
        py: Python<'_>,
        size: u64,
        usage: &str,
        label: Option<&str>,
    ) -> PyResult<PySharedBuffer> {
        let flags = parse_buffer_usage(usage)?;
        let label_owned = label.map(str::to_owned);
        let desc = SharedBufferDescriptor {
            label: label_owned.as_deref(),
            size,
            usage: flags,
        };
        validate_buffer_descriptor(&desc).map_err(|e| map_wgpu_err("create_buffer", e))?;
        let ctx = self.inner.clone();
        let buf = py
            .detach(move || {
                let d = SharedBufferDescriptor {
                    label: label_owned.as_deref(),
                    size,
                    usage: flags,
                };
                ctx.create_buffer(&d)
            })
            .map_err(|e| map_wgpu_err("create_buffer", e))?;
        Ok(PySharedBuffer { inner: buf })
    }

    /// Creates a shared GPU timeline semaphore (`InteropSemaphore`).
    #[pyo3(signature = (initial_value=0))]
    fn create_semaphore(&self, py: Python<'_>, initial_value: u64) -> PyResult<PyInteropSemaphore> {
        let ctx = self.inner.clone();
        let sem = py
            .detach(move || ctx.create_semaphore(initial_value))
            .map_err(|e| map_wgpu_err("create_semaphore", e))?;
        Ok(PyInteropSemaphore { inner: sem })
    }
}

/// Zero-copy shared buffer (`wgpu::Buffer` <-> CUDA device pointer).
#[pyclass(name = "SharedBuffer", module = "nv_rs.wgpu")]
pub struct PySharedBuffer {
    inner: SharedBuffer,
}

#[pymethods]
impl PySharedBuffer {
    #[getter]
    fn ptr(&self) -> u64 {
        self.inner.cu_deviceptr()
    }

    #[getter]
    fn size(&self) -> u64 {
        self.inner.size()
    }

    fn __repr__(&self) -> String {
        format!(
            "SharedBuffer(ptr=0x{:x}, size={})",
            self.inner.cu_deviceptr(),
            self.inner.size()
        )
    }
}

/// Cross-API GPU timeline semaphore (`nv_wgpu::InteropSemaphore`).
#[pyclass(name = "InteropSemaphore", module = "nv_rs.wgpu")]
pub struct PyInteropSemaphore {
    inner: InteropSemaphore,
}

#[pymethods]
impl PyInteropSemaphore {
    fn signal_from_cuda(&self, py: Python<'_>, stream: &PyStream, value: u64) -> PyResult<()> {
        let s = stream.inner.clone();
        py.detach(move || self.inner.signal_from_cuda(&s, value))
            .map_err(|e| map_wgpu_err("signal_from_cuda", e))
    }

    fn wait_in_cuda(&self, py: Python<'_>, stream: &PyStream, value: u64) -> PyResult<()> {
        let s = stream.inner.clone();
        py.detach(move || self.inner.wait_in_cuda(&s, value))
            .map_err(|e| map_wgpu_err("wait_in_cuda", e))
    }

    fn completed_value(&self, py: Python<'_>) -> PyResult<u64> {
        py.detach(|| self.inner.completed_value())
            .map_err(|e| map_wgpu_err("completed_value", e))
    }

    fn signal_from_host(&self, py: Python<'_>, value: u64) -> PyResult<()> {
        py.detach(|| self.inner.signal_from_host(value))
            .map_err(|e| map_wgpu_err("signal_from_host", e))
    }

    #[pyo3(signature = (value, timeout_ns=1_000_000_000))]
    fn wait_on_host(&self, py: Python<'_>, value: u64, timeout_ns: u64) -> PyResult<()> {
        py.detach(|| self.inner.wait_on_host(value, timeout_ns))
            .map_err(|e| map_wgpu_err("wait_on_host", e))
    }
}

fn register_wgpu(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(py, "wgpu")?;
    m.add_function(wrap_pyfunction!(is_supported, &m)?)?;
    m.add_function(wrap_pyfunction!(probe_adapters, &m)?)?;
    m.add_function(wrap_pyfunction!(wgpu_aligned_row_pitch, &m)?)?;
    m.add_function(wrap_pyfunction!(wgpu_aligned_staging_bytes, &m)?)?;
    m.add_class::<PyInteropContext>()?;
    m.add_class::<PySharedBuffer>()?;
    m.add_class::<PyInteropSemaphore>()?;
    register_submodule(py, parent, "nv_rs.wgpu", &m)
}

// ============================================================================
// Top-level registration
// ============================================================================

/// Registers `nv_rs.cupti`, `nv_rs.cuda_ext`, and `nv_rs.wgpu` submodules.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    register_cupti(py, parent)?;
    register_cuda_ext(py, parent)?;
    register_wgpu(py, parent)?;
    Ok(())
}
