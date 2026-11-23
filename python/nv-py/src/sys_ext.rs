//! `nv_rs.cupti`, `nv_rs.cuda_ext`, and `nv_rs.wgpu` — CUPTI profiling, CUDA driver extensions,
//! and zero-copy CUDA <-> `wgpu` interop submodules.
//!
//! **GIL rules.** Every CUDA / CUPTI / `wgpu` call that can block runs with the GIL released
//! (`py.detach`), including teardown in `Drop` ([`without_gil`]): a CUPTI callback that calls into
//! Python takes the GIL on a driver thread, so holding the GIL while waiting for a driver lock, or
//! while dropping a `Subscriber` (which waits for running callbacks), can deadlock. Mutexes of
//! this module recover from poisoning (`PoisonError::into_inner`): they guard ownership of one
//! native handle (`Option` take), which a panic cannot leave half-updated.

use std::cell::Cell;
use std::collections::VecDeque;
use std::mem::ManuallyDrop;
use std::sync::{Arc, Mutex, PoisonError};
use std::task::{Context as TaskContext, Poll, Wake, Waker};
use std::thread::Thread;
use std::time::Instant;

use cuda_core::{Context, DriverError};
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
    aligned_row_pitch, aligned_staging_bytes,
};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

use crate::core::{PyContext, PyStream};
use crate::errors::{ErrorKind, record_and_err};

// ============================================================================
// Shared helpers
// ============================================================================

/// Runs `f` with the GIL released when the calling thread holds it (a pyclass `Drop` runs under
/// the GIL), or directly when the interpreter cannot be attached (shutdown). Used for teardown
/// that can block on the driver or on CUPTI callbacks.
pub(crate) fn without_gil(f: impl FnOnce() + Send) {
    let mut f = Some(f);
    Python::try_attach(|py| {
        py.detach(|| {
            if let Some(f) = f.take() {
                f();
            }
        })
    });
    if let Some(f) = f {
        f();
    }
}

fn resolve_cuda_ctx(
    py: Python<'_>,
    category: &str,
    op: &str,
    context: Option<&PyContext>,
    device: usize,
) -> PyResult<Arc<Context>> {
    match context {
        Some(ctx) => Ok(ctx.inner.clone()),
        // `Context::new` takes driver locks: never under the GIL (see the module docs).
        None => py
            .detach(move || Context::new(device))
            .map_err(|e| record_and_err(category, op, e, ErrorKind::Cuda)),
    }
}

// ============================================================================
// 1. nv_rs.cupti
// ============================================================================

py_err_mapper!(map_cupti_err, "cupti", cupti::Error, {
    cupti::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    cupti::Error::InvalidInput(_) => ErrorKind::InvalidArgument,
    cupti::Error::Status { .. } | cupti::Error::Driver(_) => ErrorKind::Cuda,
    cupti::Error::CallbackPanicked(_) => ErrorKind::Runtime,
});

/// Every kind `parse_activity_kind` accepts; `CuptiActivity.close` disables them explicitly so
/// CUPTI failures are raised instead of parked on the context by `ActivityProfiler::drop`.
const ACTIVITY_KINDS: [ActivityKind; 6] = [
    ActivityKind::Memcpy,
    ActivityKind::Memset,
    ActivityKind::Kernel,
    ActivityKind::ConcurrentKernel,
    ActivityKind::Driver,
    ActivityKind::Runtime,
];

/// Most `(domain, cbid)` events a `CuptiSubscriber` buffers between `drain_events` calls; older
/// events are dropped first and counted.
const MAX_BUFFERED_EVENTS: usize = 1 << 16;

thread_local! {
    /// Set on a thread while it runs a `CuptiSubscriber` Python callback: dropping a `Subscriber`
    /// waits for its running callbacks, so it must not happen on such a thread.
    static IN_CUPTI_CALLBACK: Cell<bool> = const { Cell::new(false) };
}

/// Callback events buffered by a `CuptiSubscriber`, bounded by [`MAX_BUFFERED_EVENTS`].
struct EventQueue {
    events: VecDeque<(&'static str, u32)>,
    dropped: usize,
}

py_parse_enum!(parse_activity_kind, "cupti", "parse_activity_kind", ActivityKind, "CUPTI activity kind", {
    "memcpy" => ActivityKind::Memcpy,
    "memset" => ActivityKind::Memset,
    "kernel" => ActivityKind::Kernel,
    "concurrent_kernel" | "concurrentkernel" => ActivityKind::ConcurrentKernel,
    "driver" => ActivityKind::Driver,
    "runtime" => ActivityKind::Runtime,
});

py_parse_enum!(parse_callback_domain, "cupti", "parse_callback_domain", CallbackDomain, "CUPTI callback domain", {
    "driver_api" | "driverapi" | "driver" => CallbackDomain::DriverApi,
    "runtime_api" | "runtimeapi" | "runtime" => CallbackDomain::RuntimeApi,
    "resource" => CallbackDomain::Resource,
    "synchronize" | "sync" => CallbackDomain::Synchronize,
    "nvtx" => CallbackDomain::Nvtx,
    "state" => CallbackDomain::State,
});

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
///
/// CUPTI activity collection is process-wide: an enabled kind records every context, all
/// profilers share one record queue (so one profiler's `drain_records` also takes the records
/// another profiler expects), and `context` / `device` only select the context CUPTI calls bind.
/// Enabled kinds are reference-counted, so closing one profiler never disables a kind another
/// profiler still uses.
#[pyclass(name = "CuptiActivity", module = "nv_rs.cupti")]
pub struct PyCuptiActivity {
    inner: Mutex<Option<ActivityProfiler>>,
}

impl Drop for PyCuptiActivity {
    fn drop(&mut self) {
        let profiler = self
            .inner
            .get_mut()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        if let Some(profiler) = profiler {
            // The teardown flush can wait on the driver (see the module docs).
            without_gil(move || drop(profiler));
        }
    }
}

/// Disables every kind and flushes, returning the first CUPTI failure.
fn teardown_activity(profiler: &mut ActivityProfiler) -> cupti::Result<()> {
    let mut first = Ok(());
    for kind in ACTIVITY_KINDS {
        first = first.and(profiler.disable(kind));
    }
    first.and(profiler.flush(true))
}

#[pymethods]
impl PyCuptiActivity {
    /// Registers CUPTI activity callbacks and binds an `ActivityProfiler` to `context` (or `device`).
    #[new]
    #[pyo3(signature = (context=None, device=0))]
    fn new(py: Python<'_>, context: Option<&PyContext>, device: usize) -> PyResult<Self> {
        let t0 = Instant::now();
        let ctx = resolve_cuda_ctx(py, "cupti", "CuptiActivity::new", context, device)?;
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
    /// @py-return: tuple[list[dict[str, Any]], int]
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

    /// Disables this profiler's kinds, flushes remaining activity buffers and releases the
    /// profiler, raising the first CUPTI failure. Closing again is a no-op.
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        py.detach(|| {
            let taken = self
                .inner
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .take();
            let Some(mut profiler) = taken else {
                return Ok(());
            };
            let res = teardown_activity(&mut profiler);
            drop(profiler);
            res
        })
        .map_err(|e| map_cupti_err("close", e))
    }
}

/// CUPTI callback subscriber (`cupti::Subscriber`).
#[pyclass(name = "CuptiSubscriber", module = "nv_rs.cupti")]
pub struct PyCuptiSubscriber {
    inner: Mutex<Option<Subscriber>>,
    events: Arc<Mutex<EventQueue>>,
    callback_err: Arc<Mutex<Option<String>>>,
}

impl Drop for PyCuptiSubscriber {
    fn drop(&mut self) {
        let sub = self
            .inner
            .get_mut()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        let Some(sub) = sub else {
            return;
        };
        if !IN_CUPTI_CALLBACK.with(Cell::get) {
            // Dropping waits for callbacks running on other threads, which may need the GIL.
            without_gil(move || drop(sub));
            return;
        }
        // Collected inside a callback: dropping here would wait for this very callback, so
        // unsubscribe from another thread once it has returned.
        let slot = Arc::new(Mutex::new(Some(sub)));
        let moved = slot.clone();
        let spawned = std::thread::Builder::new()
            .name("nv-rs-cupti-unsubscribe".to_owned())
            .spawn(move || {
                let sub = moved.lock().unwrap_or_else(PoisonError::into_inner).take();
                drop(sub);
            });
        if let Err(e) = spawned {
            // No thread to unsubscribe from: keep the subscription (leaked) rather than deadlock.
            diag::record_error(
                "cupti",
                "CuptiSubscriber::drop",
                &format!("cannot spawn the unsubscribe thread ({e}); subscriber leaked"),
            );
            std::mem::forget(slot.lock().unwrap_or_else(PoisonError::into_inner).take());
        }
    }
}

#[pymethods]
impl PyCuptiSubscriber {
    /// Subscribes to CUPTI callbacks on `context` (or `device`).
    ///
    /// If `callback` is provided, it is invoked as `callback(domain: str, cbid: int)` and errors
    /// are captured for `check_errors()`; it must not close this subscriber. Observed
    /// `(domain, cbid)` pairs are also buffered for `drain_events()`, at most 65536 between
    /// drains (older events are dropped and reported in `nv_rs.diag` by `drain_events`).
    #[new]
    #[pyo3(signature = (context=None, device=0, callback=None))]
    fn new(
        py: Python<'_>,
        context: Option<&PyContext>,
        device: usize,
        callback: Option<Py<PyAny>>,
    ) -> PyResult<Self> {
        let t0 = Instant::now();
        let ctx = resolve_cuda_ctx(py, "cupti", "CuptiSubscriber::new", context, device)?;
        let events = Arc::new(Mutex::new(EventQueue {
            events: VecDeque::new(),
            dropped: 0,
        }));
        let callback_err = Arc::new(Mutex::new(None));

        let events_cb = events.clone();
        let err_cb = callback_err.clone();
        let cb: Box<cupti::CallbackFn> = Box::new(move |dom, cbid| {
            let dom_str = callback_domain_str(dom);
            {
                let mut q = events_cb.lock().unwrap_or_else(PoisonError::into_inner);
                if q.events.len() >= MAX_BUFFERED_EVENTS {
                    q.events.pop_front();
                    q.dropped += 1;
                }
                q.events.push_back((dom_str, cbid));
            }
            if let Some(ref py_cb) = callback {
                let outer = IN_CUPTI_CALLBACK.with(|f| f.replace(true));
                // `try_attach` skips the call while the interpreter shuts down instead of
                // blocking the driver thread that dispatched the callback.
                let res = Python::try_attach(|py| py_cb.call1(py, (dom_str, cbid)).map(|_| ()));
                IN_CUPTI_CALLBACK.with(|f| f.set(outer));
                if let Some(Err(err)) = res {
                    let mut slot = err_cb.lock().unwrap_or_else(PoisonError::into_inner);
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

    /// Drains buffered `(domain, cbid)` callback events (oldest first). Events dropped because
    /// the buffer was full since the last drain are reported as one `nv_rs.diag` warning.
    fn drain_events(&self) -> Vec<(&'static str, u32)> {
        let (events, dropped) = {
            let mut q = self.events.lock().unwrap_or_else(PoisonError::into_inner);
            let events: Vec<_> = q.events.drain(..).collect();
            (events, std::mem::take(&mut q.dropped))
        };
        if dropped > 0 {
            diag::record_event(
                Level::Warn,
                "cupti",
                "drain_events",
                format!("{dropped} callback events dropped (buffer holds {MAX_BUFFERED_EVENTS})"),
                0,
                false,
            );
        }
        events
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

    /// Unsubscribes and releases the subscriber handle, raising a panic of the Rust callback
    /// trampoline that was not yet reported. Closing again is a no-op; closing from inside a
    /// CUPTI callback raises `NvInvalidArgumentError` (unsubscribing waits for running callbacks).
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        if IN_CUPTI_CALLBACK.with(Cell::get) {
            return Err(record_and_err(
                "cupti",
                "close",
                "CuptiSubscriber cannot be closed from inside a CUPTI callback",
                ErrorKind::InvalidArgument,
            ));
        }
        py.detach(|| {
            // Released before dropping: `check_errors` must not wait behind a blocking drop.
            let taken = self
                .inner
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .take();
            let Some(sub) = taken else {
                return Ok(());
            };
            let panicked = sub.check_panic();
            drop(sub);
            panicked
        })
        .map_err(|e| map_cupti_err("close", e))
    }
}

fn register_cupti(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    py_register_submodule!(py, parent, "cupti", |m| {
        m.add_function(wrap_pyfunction!(cupti_version_py, &m)?)?;
        m.add_function(wrap_pyfunction!(callback_name, &m)?)?;
        m.add_class::<PyCuptiActivity>()?;
        m.add_class::<PyCuptiSubscriber>()?;
    })
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
///
/// Host access synchronizes the buffer's CUDA context first: without concurrent managed access
/// (every Windows device) touching managed memory from the host while any kernel of the context
/// runs is an access violation. Work queued from other threads during the host access itself is
/// not excluded.
#[pyclass(name = "ManagedBuffer", module = "nv_rs.cuda_ext")]
pub struct PyManagedBuffer {
    inner: Mutex<ManagedBuffer<u8>>,
    ctx: Arc<Context>,
    ptr: u64,
    nbytes: usize,
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
        let ctx = resolve_cuda_ctx(py, "cuda_ext", "ManagedBuffer::zeroed", context, device)?;
        let alloc_ctx = ctx.clone();
        let buf = py
            .detach(move || {
                // `ManagedBuffer::zeroed` clears the new memory from the host.
                alloc_ctx.synchronize()?;
                ManagedBuffer::<u8>::zeroed(&alloc_ctx, nbytes)
            })
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
            ptr: buf.cu_deviceptr(),
            nbytes: buf.len(),
            inner: Mutex::new(buf),
            ctx,
        })
    }

    #[getter]
    fn ptr(&self) -> u64 {
        self.ptr
    }

    #[getter]
    fn nbytes(&self) -> usize {
        self.nbytes
    }

    /// Reads the entire managed buffer as host `bytes` after synchronizing its CUDA context.
    fn to_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let host = py.detach(|| -> PyResult<Vec<u8>> {
            self.ctx.synchronize().map_err(|e| {
                record_and_err("cuda_ext", "ManagedBuffer::to_bytes", e, ErrorKind::Cuda)
            })?;
            let guard = self.inner.lock().unwrap_or_else(PoisonError::into_inner);
            Ok(guard.as_slice().to_vec())
        })?;
        Ok(PyBytes::new(py, &host))
    }

    /// Writes `data` into the managed buffer starting at `offset` after synchronizing its CUDA
    /// context.
    #[pyo3(signature = (data, offset=0))]
    fn write_bytes(&self, py: Python<'_>, data: &[u8], offset: usize) -> PyResult<()> {
        let invalid = |msg: String| {
            record_and_err(
                "cuda_ext",
                "ManagedBuffer::write_bytes",
                msg,
                ErrorKind::InvalidArgument,
            )
        };
        let end = offset
            .checked_add(data.len())
            .ok_or_else(|| invalid("offset + data length overflow".to_owned()))?;
        let range_err = || {
            invalid(format!(
                "write range {offset}..{end} exceeds buffer size {}",
                self.nbytes
            ))
        };
        if end > self.nbytes {
            return Err(range_err());
        }
        py.detach(|| {
            self.ctx.synchronize().map_err(|e| {
                record_and_err("cuda_ext", "ManagedBuffer::write_bytes", e, ErrorKind::Cuda)
            })?;
            let mut guard = self.inner.lock().unwrap_or_else(PoisonError::into_inner);
            let dst = guard
                .as_mut_slice()
                .get_mut(offset..end)
                .ok_or_else(range_err)?;
            dst.copy_from_slice(data);
            Ok(())
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "ManagedBuffer(ptr=0x{:x}, nbytes={})",
            self.ptr, self.nbytes
        )
    }
}

/// The registered private copy of a `HostRegistration`.
struct HostRegInner {
    reg: ManuallyDrop<HostRegistration<'static, u8>>,
    /// `Box::into_raw` of the registered bytes. Kept raw until [`Self::teardown`] frees it, so
    /// moving `HostRegInner` never retags the allocation `reg` points into (a moved `Box` would).
    backing: *mut [u8],
    ctx: Arc<Context>,
    released: bool,
}

// SAFETY: `backing` is owned exclusively by this value and only read through `reg` while the
// value is borrowed (behind the `Mutex` of `PyHostRegistration`); `HostRegistration` binds its
// owning `Context` itself, so unregistering and freeing may run on any thread.
unsafe impl Send for HostRegInner {}

impl HostRegInner {
    /// Unregisters and frees the copy once no device work can access it. Idempotent.
    ///
    /// Any stream of the context may read the mapped device pointer, so the whole context is
    /// synchronized first. If that fails, completion is unproven and both the registration and
    /// the memory are leaked rather than freed under the GPU.
    fn teardown(&mut self) -> Result<(), DriverError> {
        if self.released {
            return Ok(());
        }
        self.released = true;
        self.ctx.synchronize()?;
        // SAFETY: `reg` is dropped exactly once (guarded by `released`), before the memory it
        // registers is freed below.
        unsafe { ManuallyDrop::drop(&mut self.reg) };
        // SAFETY: `backing` came from `Box::into_raw` in `PyHostRegistration::new`, is freed
        // exactly once (guarded by `released`), and was unregistered just above.
        drop(unsafe { Box::from_raw(self.backing) });
        Ok(())
    }
}

impl Drop for HostRegInner {
    fn drop(&mut self) {
        if let Err(e) = self.teardown() {
            diag::record_error(
                "cuda_ext",
                "HostRegistration::drop",
                &format!("{e}; registered memory leaked"),
            );
        }
    }
}

/// Page-locked host buffer registered with CUDA (`cuda_core_ext::HostRegistration`).
#[pyclass(name = "HostRegistration", module = "nv_rs.cuda_ext")]
pub struct PyHostRegistration {
    inner: Mutex<Option<HostRegInner>>,
    /// Synchronized without holding `inner`: a CUPTI callback dispatched during the wait may
    /// need the GIL, which a getter blocked on `inner` would hold.
    ctx: Arc<Context>,
}

impl Drop for PyHostRegistration {
    fn drop(&mut self) {
        let inner = self
            .inner
            .get_mut()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        if let Some(inner) = inner {
            // Teardown synchronizes the context.
            without_gil(move || drop(inner));
        }
    }
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
        let ctx = resolve_cuda_ctx(py, "cuda_ext", "HostRegistration::new", context, device)?;
        let copy = data.to_vec();
        let reg_ctx = ctx.clone();

        let inner = py
            .detach(move || {
                let ctx = reg_ctx;
                let backing: *mut [u8] = Box::into_raw(copy.into_boxed_slice());
                // SAFETY: `backing` is a live heap allocation that no other reference accesses;
                // it is owned by the `HostRegInner` built below, which unregisters `reg` before
                // freeing it (or freed right here if registration fails), so the `'static`
                // borrow never outlives the memory.
                let slice_static: &'static mut [u8] = unsafe { &mut *backing };
                match HostRegistration::new(&ctx, slice_static, map_to_device) {
                    Ok(reg) => Ok(HostRegInner {
                        reg: ManuallyDrop::new(reg),
                        backing,
                        ctx,
                        released: false,
                    }),
                    Err(e) => {
                        // SAFETY: registration failed, so nothing refers to `backing` any more.
                        drop(unsafe { Box::from_raw(backing) });
                        Err(e)
                    }
                }
            })
            .map_err(|e| record_and_err("cuda_ext", "HostRegistration::new", e, ErrorKind::Cuda))?;

        Ok(Self {
            inner: Mutex::new(Some(inner)),
            ctx,
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

    /// Reads the registered bytes after synchronizing the CUDA context (device work may write
    /// them through `device_ptr`).
    fn to_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let host = py.detach(|| -> PyResult<Vec<u8>> {
            self.ctx.synchronize().map_err(|e| {
                record_and_err("cuda_ext", "HostRegistration::to_bytes", e, ErrorKind::Cuda)
            })?;
            let guard = self.inner.lock().unwrap_or_else(PoisonError::into_inner);
            let inner = guard.as_ref().ok_or_else(|| {
                record_and_err(
                    "cuda_ext",
                    "HostRegistration::to_bytes",
                    "HostRegistration is closed",
                    ErrorKind::InvalidArgument,
                )
            })?;
            Ok(inner.reg.as_slice().to_vec())
        })?;
        Ok(PyBytes::new(py, &host))
    }

    /// Unregisters and frees the page-locked buffer once the CUDA context has finished all
    /// queued work (any stream may read `device_ptr`). Raises if that synchronization fails; the
    /// memory is then leaked instead of freed. Closing again is a no-op.
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        py.detach(|| {
            // Released before the teardown waits on the GPU.
            let taken = self
                .inner
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .take();
            match taken {
                Some(mut inner) => inner.teardown(),
                None => Ok(()),
            }
        })
        .map_err(|e| record_and_err("cuda_ext", "HostRegistration::close", e, ErrorKind::Cuda))
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
    let ctx = resolve_cuda_ctx(py, "cuda_ext", "split_sms", context, device)?;
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

/// The `MemoryHandle` variant named by a normalized `handle_kind` (with a placeholder handle),
/// so the accepted names and the dedicated rule below follow `cuda-core-ext`'s variants.
fn memory_handle_kind(norm: &str) -> Option<MemoryHandle> {
    let none = std::ptr::null_mut();
    match norm {
        "opaque_win32" => Some(MemoryHandle::OpaqueWin32(none)),
        "opaque_fd" => Some(MemoryHandle::OpaqueFd(-1)),
        "d3d12_resource" => Some(MemoryHandle::D3d12Resource(none)),
        "d3d12_heap" => Some(MemoryHandle::D3d12Heap(none)),
        _ => None,
    }
}

#[pymethods]
impl PyExternalMemoryDesc {
    #[new]
    #[pyo3(signature = (handle_kind, size, dedicated=false))]
    fn new(handle_kind: &str, size: u64, dedicated: bool) -> PyResult<Self> {
        let norm = handle_kind.trim().to_ascii_lowercase();
        let Some(kind) = memory_handle_kind(&norm) else {
            return Err(record_and_err(
                "cuda_ext",
                "ExternalMemoryDesc::new",
                format!("unknown external memory handle_kind `{handle_kind}`"),
                ErrorKind::InvalidArgument,
            ));
        };
        // Exhaustive: a new `MemoryHandle` variant must decide its dedicated-allocation rule.
        let needs_dedicated = match kind {
            MemoryHandle::D3d12Resource(_) => true,
            MemoryHandle::OpaqueWin32(_) | MemoryHandle::OpaqueFd(_) | MemoryHandle::D3d12Heap(_) => {
                false
            }
        };
        if size == 0 {
            return Err(record_and_err(
                "cuda_ext",
                "ExternalMemoryDesc::new",
                "external memory size must be non-zero",
                ErrorKind::InvalidArgument,
            ));
        }
        if needs_dedicated && !dedicated {
            return Err(record_and_err(
                "cuda_ext",
                "ExternalMemoryDesc::new",
                format!("{norm} external memory requires dedicated=True"),
                ErrorKind::InvalidArgument,
            ));
        }
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
    py_register_submodule!(py, parent, "cuda_ext", |m| {
        m.add_function(wrap_pyfunction!(device_luid, &m)?)?;
        m.add_function(wrap_pyfunction!(device_uuid, &m)?)?;
        m.add_function(wrap_pyfunction!(sparse_arrays_supported, &m)?)?;
        m.add_function(wrap_pyfunction!(split_sms, &m)?)?;
        m.add_class::<PyManagedBuffer>()?;
        m.add_class::<PyHostRegistration>()?;
        m.add_class::<PyGreenContext>()?;
        m.add_class::<PyExternalMemoryDesc>()?;
    })
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

/// Exhaustive (a new `nv_wgpu::Error` variant must be classified here).
fn wgpu_err_kind(err: &nv_wgpu::Error) -> ErrorKind {
    match err {
        nv_wgpu::Error::InvalidInput(_)
        | nv_wgpu::Error::Unsupported { .. }
        | nv_wgpu::Error::BackendMismatch { .. }
        | nv_wgpu::Error::DeviceMismatch { .. }
        | nv_wgpu::Error::NoMatchingCudaDevice { .. } => ErrorKind::InvalidArgument,
        nv_wgpu::Error::Driver(_)
        | nv_wgpu::Error::Vulkan { .. }
        | nv_wgpu::Error::InvalidHandle { .. }
        | nv_wgpu::Error::WgpuRequest(_)
        | nv_wgpu::Error::Timeout { .. }
        | nv_wgpu::Error::Poll(_) => ErrorKind::Cuda,
        #[cfg(windows)]
        nv_wgpu::Error::Dx12 { .. } => ErrorKind::Cuda,
        nv_wgpu::Error::Poisoned(_) => ErrorKind::Runtime,
    }
}

fn map_wgpu_err(op: &str, err: nv_wgpu::Error) -> PyErr {
    let kind = wgpu_err_kind(&err);
    record_and_err("wgpu", op, err, kind)
}

/// A `wgpu` error captured by an error scope (the default uncaptured handler panics).
fn map_wgpu_scope_err(op: &str, err: wgpu::Error) -> PyErr {
    let kind = match &err {
        wgpu::Error::Validation { .. } => ErrorKind::InvalidArgument,
        wgpu::Error::OutOfMemory { .. } | wgpu::Error::Internal { .. } => ErrorKind::Cuda,
    };
    record_and_err("wgpu", op, err, kind)
}

py_parse_enum!(parse_texture_format, "wgpu", "parse_texture_format", wgpu::TextureFormat, "wgpu texture format", {
    "r8unorm" | "r8_unorm" => wgpu::TextureFormat::R8Unorm,
    "r8uint" | "r8_uint" => wgpu::TextureFormat::R8Uint,
    "rg8unorm" | "rg8_unorm" => wgpu::TextureFormat::Rg8Unorm,
    "rg8uint" | "rg8_uint" => wgpu::TextureFormat::Rg8Uint,
    "rgba8unorm" | "rgba8_unorm" => wgpu::TextureFormat::Rgba8Unorm,
    "rgba8unormsrgb" | "rgba8_unorm_srgb" => wgpu::TextureFormat::Rgba8UnormSrgb,
    "rgba8uint" | "rgba8_uint" => wgpu::TextureFormat::Rgba8Uint,
    "bgra8unorm" | "bgra8_unorm" => wgpu::TextureFormat::Bgra8Unorm,
    "bgra8unormsrgb" | "bgra8_unorm_srgb" => wgpu::TextureFormat::Bgra8UnormSrgb,
    "r16unorm" | "r16_unorm" => wgpu::TextureFormat::R16Unorm,
    "r16uint" | "r16_uint" => wgpu::TextureFormat::R16Uint,
    "r16float" | "r16_float" => wgpu::TextureFormat::R16Float,
    "rg16unorm" | "rg16_unorm" => wgpu::TextureFormat::Rg16Unorm,
    "rg16uint" | "rg16_uint" => wgpu::TextureFormat::Rg16Uint,
    "rg16float" | "rg16_float" => wgpu::TextureFormat::Rg16Float,
    "rgba16unorm" | "rgba16_unorm" => wgpu::TextureFormat::Rgba16Unorm,
    "rgba16uint" | "rgba16_uint" => wgpu::TextureFormat::Rgba16Uint,
    "rgba16float" | "rgba16_float" => wgpu::TextureFormat::Rgba16Float,
    "r32uint" | "r32_uint" => wgpu::TextureFormat::R32Uint,
    "r32sint" | "r32_sint" => wgpu::TextureFormat::R32Sint,
    "r32float" | "r32_float" => wgpu::TextureFormat::R32Float,
    "rg32uint" | "rg32_uint" => wgpu::TextureFormat::Rg32Uint,
    "rg32sint" | "rg32_sint" => wgpu::TextureFormat::Rg32Sint,
    "rg32float" | "rg32_float" => wgpu::TextureFormat::Rg32Float,
    "rgba32uint" | "rgba32_uint" => wgpu::TextureFormat::Rgba32Uint,
    "rgba32sint" | "rgba32_sint" => wgpu::TextureFormat::Rgba32Sint,
    "rgba32float" | "rgba32_float" => wgpu::TextureFormat::Rgba32Float,
});

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
            // No `map_read` / `map_write`: an externally shared buffer cannot be host-mapped
            // (`nv_wgpu::validate_buffer_descriptor` rejects both).
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

/// Backends that can share memory with CUDA on this OS.
fn interop_backends() -> wgpu::Backends {
    if cfg!(windows) {
        wgpu::Backends::VULKAN | wgpu::Backends::DX12
    } else {
        wgpu::Backends::VULKAN
    }
}

/// Opens an interop device on `adapter` with the constructor of its `backend`.
fn open_interop(
    adapter: &wgpu::Adapter,
    backend: wgpu::Backend,
    label: Option<&str>,
) -> nv_wgpu::Result<InteropContext> {
    match backend {
        wgpu::Backend::Vulkan => InteropContext::open_vulkan(
            adapter,
            wgpu::Features::empty(),
            &wgpu::Limits::default(),
            label,
        ),
        #[cfg(windows)]
        wgpu::Backend::Dx12 => {
            match block_on(adapter.request_device(&wgpu::DeviceDescriptor {
                label,
                ..Default::default()
            })) {
                Ok((device, queue)) => InteropContext::from_wgpu(adapter, device, queue),
                Err(e) => Err(nv_wgpu::Error::WgpuRequest(e.to_string())),
            }
        }
        other => Err(nv_wgpu::Error::Unsupported {
            what: "wgpu backend",
            required: "Vulkan or DX12",
            found: format!("{other:?}"),
        }),
    }
}

/// The one adapter loop of `is_supported`, `probe_adapters` and `InteropContext()`: tries to
/// open interop on every adapter of `backends` in enumeration order, hands each attempt to
/// `visit`, and stops at the first `Some` it returns.
fn find_interop_adapter<T>(
    backends: wgpu::Backends,
    label: Option<&str>,
    mut visit: impl FnMut(wgpu::AdapterInfo, nv_wgpu::Result<InteropContext>) -> Option<T>,
) -> Option<T> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    for adapter in block_on(instance.enumerate_adapters(backends)) {
        let info = adapter.get_info();
        let opened = open_interop(&adapter, info.backend, label);
        if let Some(found) = visit(info, opened) {
            return Some(found);
        }
    }
    None
}

fn probe_wgpu_adapters() -> Vec<AdapterProbe> {
    let mut out = Vec::new();
    find_interop_adapter::<()>(interop_backends(), Some("nv-rs-probe"), |info, opened| {
        let error = opened.err().map(|e| e.to_string());
        out.push(AdapterProbe {
            name: info.name,
            backend: format!("{:?}", info.backend),
            device_type: format!("{:?}", info.device_type),
            vendor: info.vendor,
            device: info.device,
            cuda_interop_supported: error.is_none(),
            error,
        });
        None
    });
    out
}

/// One error for a failed `InteropContext()` that names every adapter's failure, not only the
/// last one's.
fn open_failure(op: &str, mut failures: Vec<(String, wgpu::Backend, nv_wgpu::Error)>) -> PyErr {
    if failures.len() <= 1 {
        let err = failures.pop().map_or_else(
            || nv_wgpu::Error::WgpuRequest("no interop-capable wgpu adapter found".to_owned()),
            |(_, _, e)| e,
        );
        return map_wgpu_err(op, err);
    }
    let kind = if failures
        .iter()
        .all(|(_, _, e)| matches!(wgpu_err_kind(e), ErrorKind::InvalidArgument))
    {
        ErrorKind::InvalidArgument
    } else {
        ErrorKind::Cuda
    };
    let detail = failures
        .iter()
        .map(|(name, backend, e)| format!("{name} ({backend:?}): {e}"))
        .collect::<Vec<_>>()
        .join("; ");
    record_and_err(
        "wgpu",
        op,
        format!("no wgpu adapter could open CUDA interop: {detail}"),
        kind,
    )
}

/// Returns `True` if at least one `wgpu` adapter can open CUDA <-> `wgpu` zero-copy interop
/// (stops at the first one that can).
#[pyfunction]
fn is_supported(py: Python<'_>) -> bool {
    let t0 = Instant::now();
    let ok = py.detach(|| {
        find_interop_adapter(interop_backends(), Some("nv-rs-probe"), |_, opened| {
            opened.ok().map(drop)
        })
        .is_some()
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
/// @py-return: list[dict[str, Any]]
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
            "auto" => interop_backends(),
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
        let (found, failures) = py.detach(move || {
            let mut failures = Vec::new();
            let found =
                find_interop_adapter(backends, label_owned.as_deref(), |info, opened| match opened {
                    Ok(ctx) => Some(ctx),
                    Err(e) => {
                        failures.push((info.name, info.backend, e));
                        None
                    }
                });
            (found, failures)
        });
        let ctx = found.ok_or_else(|| open_failure("InteropContext::new", failures))?;

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

    /// Creates a zero-copy shared GPU buffer (`SharedBuffer`). A size above the device's
    /// `max_buffer_size` or any `wgpu` validation error raises (never a panic).
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
        let ctx = self.inner.clone();
        let buf = py.detach(move || -> PyResult<SharedBuffer> {
            let device = ctx.device();
            let max = device.limits().max_buffer_size;
            if size > max {
                return Err(record_and_err(
                    "wgpu",
                    "create_buffer",
                    format!("size {size} exceeds the device's max_buffer_size {max}"),
                    ErrorKind::InvalidArgument,
                ));
            }
            let desc = SharedBufferDescriptor {
                label: label_owned.as_deref(),
                size,
                usage: flags,
            };
            // Without scopes a `wgpu` error goes to the default uncaptured handler, which panics.
            let validation = device.push_error_scope(wgpu::ErrorFilter::Validation);
            let oom = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
            let internal = device.push_error_scope(wgpu::ErrorFilter::Internal);
            let created = ctx.create_buffer(&desc);
            // Popped in reverse order of their pushes.
            let internal_err = block_on(internal.pop());
            let oom_err = block_on(oom.pop());
            let validation_err = block_on(validation.pop());
            let buf = created.map_err(|e| map_wgpu_err("create_buffer", e))?;
            match validation_err.or(oom_err).or(internal_err) {
                None => Ok(buf),
                Some(e) => Err(map_wgpu_scope_err("create_buffer", e)),
            }
        })?;
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
    py_register_submodule!(py, parent, "wgpu", |m| {
        m.add_function(wrap_pyfunction!(is_supported, &m)?)?;
        m.add_function(wrap_pyfunction!(probe_adapters, &m)?)?;
        m.add_function(wrap_pyfunction!(wgpu_aligned_row_pitch, &m)?)?;
        m.add_function(wrap_pyfunction!(wgpu_aligned_staging_bytes, &m)?)?;
        m.add_class::<PyInteropContext>()?;
        m.add_class::<PySharedBuffer>()?;
        m.add_class::<PyInteropSemaphore>()?;
    })
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
