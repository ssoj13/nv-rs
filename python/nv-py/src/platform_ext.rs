//! `nv_rs.nccl`, `nv_rs.cufile`, and `nv_rs.nvapi` — cross-platform Python bindings for
//! platform-specific NVIDIA subsystems.
//!
//! Every submodule registers unconditionally on all operating systems with a `SUPPORTED: bool`
//! constant (`True` on its target OS, `False` elsewhere) so `import nv_rs` never fails on
//! Windows or Linux. Runtime entry points raise `NvInvalidArgumentError` on unsupported hosts.
//!
//! **Buffers (Linux).** NCCL and cuFile take Python buffers (nv_rs-owned or DLPack imports) as a
//! byte-typed `DeviceBuffer` over the buffer's own storage ([`byte_span`]: same allocation and
//! access tracker, no copy); a completion lease (`buf_util::enqueue_leased`) makes each access
//! exclusive and keeps the memory allocated until the work completes.
//! **NCCL groups.** Inside `group_start()` / `group_end()` NCCL defers every collective to the
//! outermost `group_end`, after the call returned. The binding mirrors the per-thread group depth
//! ([`NCCL_GROUP`]) and holds the leases of each grouped collective until that `group_end` has
//! queued it, then retains them until the work completes ([`settle_group`]).
//! **Mutexes** recover from poisoning (as in `sys_ext`): each guards one handle's ownership.

use std::time::Instant;

use nv_loader::diag::{self, Level};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

use crate::core::{PyContext, PyDeviceBuffer, PyStream};
use crate::errors::{ErrorKind, record_and_err};

#[cfg(target_os = "linux")]
use std::cell::RefCell;
#[cfg(target_os = "linux")]
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(target_os = "linux")]
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

#[cfg(target_os = "linux")]
use cuda_core::{AllocationLease, DeviceBuffer as CudaDeviceBuffer, Stream};

#[cfg(target_os = "linux")]
use crate::buf_util::{Access, check_buf, enqueue_leased};
#[cfg(target_os = "linux")]
use crate::sys_ext::without_gil;

#[cfg(not(target_os = "linux"))]
fn unsupported_linux(category: &str, op: &str) -> PyErr {
    record_and_err(
        category,
        op,
        format!("nv_rs.{category} is only supported on Linux"),
        ErrorKind::InvalidArgument,
    )
}

#[cfg(not(target_os = "windows"))]
fn unsupported_windows(op: &str) -> PyErr {
    record_and_err(
        "nvapi",
        op,
        "nv_rs.nvapi is only supported on Windows",
        ErrorKind::InvalidArgument,
    )
}

// ---------------------------------------------------------------------------
// nv_rs.nccl (Linux-only runtime)
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
py_err_mapper!(map_nccl_err, "nccl", nccl::Error, {
    nccl::Error::Load(_) => ErrorKind::LibraryNotFound,
    nccl::Error::Invalid(_) | nccl::Error::Unsupported(_) => ErrorKind::InvalidArgument,
    nccl::Error::Driver(_) | nccl::Error::Status { .. } | nccl::Error::Call(_) => {
        ErrorKind::Cuda
    }
});

#[cfg(target_os = "linux")]
py_parse_enum!(parse_nccl_dtype, "nccl", "parse_dtype", nccl::DataType, "NCCL dtype", {
    "i8" | "int8" => nccl::DataType::I8,
    "u8" | "uint8" => nccl::DataType::U8,
    "i32" | "int32" => nccl::DataType::I32,
    "u32" | "uint32" => nccl::DataType::U32,
    "i64" | "int64" => nccl::DataType::I64,
    "u64" | "uint64" => nccl::DataType::U64,
    "f16" | "float16" => nccl::DataType::F16,
    "f32" | "float32" => nccl::DataType::F32,
    "f64" | "float64" => nccl::DataType::F64,
    "bf16" | "bfloat16" => nccl::DataType::Bf16,
    "f8e4m3" | "fp8_e4m3" => nccl::DataType::F8E4M3,
    "f8e5m2" | "fp8_e5m2" => nccl::DataType::F8E5M2,
});

#[cfg(target_os = "linux")]
py_parse_enum!(parse_nccl_redop, "nccl", "parse_redop", nccl::RedOp, "NCCL reduction op (expected sum, prod, max, min, or avg)", {
    "sum" => nccl::RedOp::Sum,
    "prod" => nccl::RedOp::Prod,
    "max" => nccl::RedOp::Max,
    "min" => nccl::RedOp::Min,
    "avg" => nccl::RedOp::Avg,
});

#[cfg(target_os = "linux")]
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Validates `buf` for native call `category`/`op` on `stream` and returns a byte-typed
/// `DeviceBuffer` over its own storage (same allocation and access tracker; nothing is copied or
/// allocated). The NCCL and cuFile wrappers take such owned spans; exclusivity of the access comes
/// from the completion lease the caller takes on `buf` (`enqueue_leased`).
#[cfg(target_os = "linux")]
fn byte_span(
    category: &str,
    op: &str,
    name: &str,
    buf: &PyDeviceBuffer,
    stream: &Stream,
) -> PyResult<CudaDeviceBuffer<u8>> {
    check_buf(category, op, name, buf, &[], 0, stream.context())?;
    let storage = buf.storage().clone();
    let len = storage.len_bytes();
    // SAFETY: every bit pattern is a valid `u8`. The view is handed only to NCCL / cuFile, which
    // move bytes with kernels, DMA or driver copies and never form a Rust reference to device
    // memory, and every access through it runs under a completion lease on `storage`.
    unsafe { CudaDeviceBuffer::<u8>::from_storage(storage, len) }
        .map_err(|e| record_and_err(category, op, e, ErrorKind::InvalidArgument))
}

/// Leases of one collective issued inside an open NCCL group, held until the outermost
/// `group_end` queues the collective (see the module docs).
#[cfg(target_os = "linux")]
struct GroupedOp {
    stream: Arc<Stream>,
    leases: Vec<AllocationLease>,
}

#[cfg(target_os = "linux")]
struct GroupState {
    depth: usize,
    ops: Vec<GroupedOp>,
}

#[cfg(target_os = "linux")]
thread_local! {
    /// Mirror of NCCL's per-thread group nesting (`ncclGroupStart` / `ncclGroupEnd`) as entered
    /// through `nv_rs.nccl.group_start` / `group_end`.
    static NCCL_GROUP: RefCell<GroupState> = const {
        RefCell::new(GroupState {
            depth: 0,
            ops: Vec::new(),
        })
    };
}

#[cfg(target_os = "linux")]
fn live_comm<'a>(slot: &'a Option<nccl::Comm>, op: &str) -> PyResult<&'a nccl::Comm> {
    slot.as_ref().ok_or_else(|| {
        map_nccl_err(
            op,
            nccl::Error::Invalid("communicator already finalized"),
        )
    })
}

/// Runs collective `run` on `stream` with completion leases on `buffers`.
///
/// Outside a group the leases cover the queued collective. Inside a group `run` only records the
/// collective, so a second set of leases is held until the outermost `group_end` (taken even when
/// `run` failed: NCCL may have recorded part of the work).
#[cfg(target_os = "linux")]
fn submit<const N: usize>(
    comm: &Mutex<Option<nccl::Comm>>,
    op: &'static str,
    stream: &Arc<Stream>,
    buffers: [(&PyDeviceBuffer, Access); N],
    run: impl FnOnce(&nccl::Comm) -> nccl::Result<()>,
) -> PyResult<()> {
    let guard = lock(comm);
    let comm = live_comm(&guard, op)?;
    let grouped = NCCL_GROUP.with(|g| g.borrow().depth > 0);
    let mut ran = false;
    let queued = enqueue_leased("nccl", op, stream, buffers, || {
        ran = true;
        run(comm).map_err(|e| map_nccl_err(op, e))
    });
    if !(grouped && ran) {
        return queued;
    }
    let mut held = Ok(());
    let mut leases = Vec::with_capacity(N);
    for (buf, access) in buffers {
        match buf.storage().acquire(stream, access == Access::Write, false) {
            Ok(lease) => leases.push(lease),
            Err(e) => {
                held = held.and(Err(record_and_err("nccl", op, e, ErrorKind::Cuda)));
            }
        }
    }
    NCCL_GROUP.with(|g| {
        g.borrow_mut().ops.push(GroupedOp {
            stream: stream.clone(),
            leases,
        })
    });
    queued.and(held)
}

/// Retains the leases of the collectives of a closed outermost group until the work queued on
/// their streams so far, which now includes those collectives, completes.
#[cfg(target_os = "linux")]
fn settle_group(ops: Vec<GroupedOp>) -> PyResult<()> {
    let mut settled = Ok(());
    for GroupedOp { stream, leases } in ops {
        for lease in leases {
            // On failure `cuda-core` leaks the lease: the memory stays allocated.
            if let Err(e) = lease.retain_until_complete(&stream) {
                settled = settled.and(Err(record_and_err(
                    "nccl",
                    "group_end",
                    e,
                    ErrorKind::Cuda,
                )));
            }
        }
    }
    settled
}

/// Returns the loaded NCCL library version integer (`MAJOR * 10000 + MINOR * 100 + PATCH`).
#[pyfunction]
#[pyo3(name = "version")]
fn nccl_version(py: Python<'_>) -> PyResult<i32> {
    #[cfg(target_os = "linux")]
    {
        let t0 = Instant::now();
        let ver = py
            .detach(nccl::version)
            .map_err(|e| map_nccl_err("version", e))?;
        diag::record_event(
            Level::Info,
            "nccl",
            "version",
            format!("version={ver}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(ver)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = py;
        Err(unsupported_linux("nccl", "version"))
    }
}

/// Allocates a fresh 128-byte NCCL unique rendezvous ID (`ncclGetUniqueId`).
#[pyfunction]
#[pyo3(name = "unique_id")]
fn nccl_unique_id(py: Python<'_>) -> PyResult<Py<PyBytes>> {
    #[cfg(target_os = "linux")]
    {
        let t0 = Instant::now();
        let id = py
            .detach(nccl::UniqueId::new)
            .map_err(|e| map_nccl_err("unique_id", e))?;
        diag::record_event(
            Level::Info,
            "nccl",
            "unique_id",
            "128 B".to_owned(),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(PyBytes::new(py, id.as_bytes()).unbind())
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = py;
        Err(unsupported_linux("nccl", "unique_id"))
    }
}

/// Begins a fused NCCL operation group (`ncclGroupStart`) on the calling thread.
#[pyfunction]
#[pyo3(name = "group_start")]
fn nccl_group_start(py: Python<'_>) -> PyResult<()> {
    #[cfg(target_os = "linux")]
    {
        py.detach(|| {
            nccl::group_start().map_err(|e| map_nccl_err("group_start", e))?;
            NCCL_GROUP.with(|g| g.borrow_mut().depth += 1);
            Ok(())
        })
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = py;
        Err(unsupported_linux("nccl", "group_start"))
    }
}

/// Ends a fused NCCL operation group (`ncclGroupEnd`) on the calling thread. The outermost end
/// queues the group's collectives and keeps their buffers leased until they complete.
#[pyfunction]
#[pyo3(name = "group_end")]
fn nccl_group_end(py: Python<'_>) -> PyResult<()> {
    #[cfg(target_os = "linux")]
    {
        py.detach(|| {
            let ended = nccl::group_end().map_err(|e| map_nccl_err("group_end", e));
            let ops = NCCL_GROUP.with(|g| {
                let mut g = g.borrow_mut();
                // `ncclGroupEnd` leaves the group even when launching fails; only a call outside
                // any group changes nothing.
                g.depth = g.depth.saturating_sub(1);
                if g.depth == 0 {
                    std::mem::take(&mut g.ops)
                } else {
                    Vec::new()
                }
            });
            // Settled whatever `ncclGroupEnd` returned: part of the group may have been queued.
            let settled = settle_group(ops);
            ended.and(settled)
        })
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = py;
        Err(unsupported_linux("nccl", "group_end"))
    }
}

/// RAII NCCL communicator (`ncclComm_t`).
#[pyclass(name = "NcclComm", module = "nv_rs.nccl")]
pub struct PyNcclComm {
    #[cfg(target_os = "linux")]
    inner: Arc<Mutex<Option<nccl::Comm>>>,
    #[cfg(target_os = "linux")]
    nranks: usize,
    #[cfg(target_os = "linux")]
    rank: usize,
    #[cfg(target_os = "linux")]
    device_id: i32,
}

#[cfg(target_os = "linux")]
impl PyNcclComm {
    fn from_comm(comm: nccl::Comm) -> Self {
        let nranks = comm.count();
        let rank = comm.user_rank();
        let device_id = comm.cu_device();
        Self {
            inner: Arc::new(Mutex::new(Some(comm))),
            nranks,
            rank,
            device_id,
        }
    }
}

#[cfg(target_os = "linux")]
impl Drop for PyNcclComm {
    fn drop(&mut self) {
        let comm = lock(&self.inner).take();
        if let Some(comm) = comm {
            // `ncclCommDestroy` can block until peers post matching work: never under the GIL.
            without_gil(move || drop(comm));
        }
    }
}

#[pymethods]
impl PyNcclComm {
    /// Initializes a single-process clique of communicators across `contexts` (`ncclCommInitAll`).
    #[staticmethod]
    fn init_all(py: Python<'_>, contexts: Vec<PyRef<'_, PyContext>>) -> PyResult<Vec<Self>> {
        #[cfg(target_os = "linux")]
        {
            let t0 = Instant::now();
            let ctxs: Vec<Arc<cuda_core::Context>> =
                contexts.into_iter().map(|c| c.inner.clone()).collect();
            let n = ctxs.len();
            let comms = py
                .detach(move || nccl::Comm::init_all(&ctxs))
                .map_err(|e| map_nccl_err("init_all", e))?;
            diag::record_event(
                Level::Info,
                "nccl",
                "init_all",
                format!("nranks={n}"),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(comms.into_iter().map(Self::from_comm).collect())
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (py, contexts);
            Err(unsupported_linux("nccl", "NcclComm::init_all"))
        }
    }

    /// Initializes one rank of a multi-process/multi-thread communicator (`ncclCommInitRank`).
    #[staticmethod]
    fn init_rank(
        py: Python<'_>,
        context: &PyContext,
        nranks: usize,
        unique_id: &[u8],
        rank: usize,
    ) -> PyResult<Self> {
        #[cfg(target_os = "linux")]
        {
            let t0 = Instant::now();
            let uid_bytes: [u8; 128] = unique_id.try_into().map_err(|_| {
                record_and_err(
                    "nccl",
                    "init_rank",
                    format!(
                        "unique_id must be exactly 128 bytes, got {}",
                        unique_id.len()
                    ),
                    ErrorKind::InvalidArgument,
                )
            })?;
            let uid = nccl::UniqueId::from_bytes(uid_bytes);
            let ctx = context.inner.clone();
            let comm = py
                .detach(move || nccl::Comm::init_rank(&ctx, nranks, uid, rank))
                .map_err(|e| map_nccl_err("init_rank", e))?;
            diag::record_event(
                Level::Info,
                "nccl",
                "init_rank",
                format!("rank={rank}/{nranks}"),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(Self::from_comm(comm))
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (py, context, nranks, unique_id, rank);
            Err(unsupported_linux("nccl", "NcclComm::init_rank"))
        }
    }

    #[getter]
    fn count(&self) -> PyResult<usize> {
        #[cfg(target_os = "linux")]
        {
            Ok(self.nranks)
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(unsupported_linux("nccl", "NcclComm::count"))
        }
    }

    #[getter]
    fn rank(&self) -> PyResult<usize> {
        #[cfg(target_os = "linux")]
        {
            Ok(self.rank)
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(unsupported_linux("nccl", "NcclComm::rank"))
        }
    }

    #[getter]
    fn device_id(&self) -> PyResult<i32> {
        #[cfg(target_os = "linux")]
        {
            Ok(self.device_id)
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(unsupported_linux("nccl", "NcclComm::device_id"))
        }
    }

    /// Queries asynchronous communicator status (`"ok"`, `"in_progress"`, or `"failed(...)"`).
    fn async_error(&self, py: Python<'_>) -> PyResult<String> {
        #[cfg(target_os = "linux")]
        {
            let state = py.detach(|| {
                let guard = lock(&self.inner);
                live_comm(&guard, "async_error")?
                    .async_error()
                    .map_err(|e| map_nccl_err("async_error", e))
            })?;
            Ok(match state {
                nccl::AsyncState::Ok => "ok".to_owned(),
                nccl::AsyncState::InProgress => "in_progress".to_owned(),
                nccl::AsyncState::Failed(code) => format!("failed({code})"),
            })
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = py;
            Err(unsupported_linux("nccl", "NcclComm::async_error"))
        }
    }

    /// Returns NCCL's human-readable last error string for this communicator, if available.
    fn last_error(&self) -> PyResult<Option<String>> {
        #[cfg(target_os = "linux")]
        {
            let guard = lock(&self.inner);
            Ok(live_comm(&guard, "last_error")?.last_error())
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(unsupported_linux("nccl", "NcclComm::last_error"))
        }
    }

    /// Gracefully finalizes pending communications and destroys the communicator. Calling it
    /// again (or after `abort`) is a no-op.
    fn finalize(&self, py: Python<'_>) -> PyResult<()> {
        #[cfg(target_os = "linux")]
        {
            py.detach(|| {
                let taken = lock(&self.inner).take();
                taken.map_or(Ok(()), nccl::Comm::finalize)
            })
            .map_err(|e| map_nccl_err("finalize", e))
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = py;
            Err(unsupported_linux("nccl", "NcclComm::finalize"))
        }
    }

    /// Immediately aborts all queued work and destroys the communicator. Calling it again (or
    /// after `finalize`) is a no-op.
    fn abort(&self, py: Python<'_>) -> PyResult<()> {
        #[cfg(target_os = "linux")]
        {
            py.detach(|| {
                let taken = lock(&self.inner).take();
                taken.map_or(Ok(()), nccl::Comm::abort)
            })
            .map_err(|e| map_nccl_err("abort", e))
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = py;
            Err(unsupported_linux("nccl", "NcclComm::abort"))
        }
    }

    /// Launches `ncclAllReduce` on `stream`, writing into `recv`.
    #[expect(
        clippy::too_many_arguments,
        reason = "Direct Python binding matching nccl::Comm::all_reduce"
    )]
    fn all_reduce(
        &self,
        py: Python<'_>,
        send: &PyDeviceBuffer,
        recv: &PyDeviceBuffer,
        count: usize,
        dtype: &str,
        op: &str,
        stream: &PyStream,
    ) -> PyResult<()> {
        #[cfg(target_os = "linux")]
        {
            let t0 = Instant::now();
            let dt = parse_nccl_dtype(dtype)?;
            let red_op = parse_nccl_redop(op)?;
            let s = &stream.inner;
            let send_span = byte_span("nccl", "all_reduce", "send", send, s)?;
            let mut recv_span = byte_span("nccl", "all_reduce", "recv", recv, s)?;
            py.detach(|| {
                submit(
                    &self.inner,
                    "all_reduce",
                    s,
                    [(send, Access::Read), (recv, Access::Write)],
                    |c| c.all_reduce(&send_span, &mut recv_span, count, dt, red_op, s),
                )
            })?;
            diag::record_event(
                Level::Info,
                "nccl",
                "all_reduce",
                format!("count={count} dtype={dtype} op={op}"),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(())
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (py, send, recv, count, dtype, op, stream);
            Err(unsupported_linux("nccl", "NcclComm::all_reduce"))
        }
    }

    /// Launches `ncclBroadcast` from `root` rank into `recv` on `stream`.
    #[expect(
        clippy::too_many_arguments,
        reason = "Direct Python binding matching nccl::Comm::broadcast"
    )]
    fn broadcast(
        &self,
        py: Python<'_>,
        send: Option<&PyDeviceBuffer>,
        recv: &PyDeviceBuffer,
        count: usize,
        dtype: &str,
        root: usize,
        stream: &PyStream,
    ) -> PyResult<()> {
        #[cfg(target_os = "linux")]
        {
            let t0 = Instant::now();
            let dt = parse_nccl_dtype(dtype)?;
            let s = &stream.inner;
            let send_span = send
                .map(|b| byte_span("nccl", "broadcast", "send", b, s))
                .transpose()?;
            let mut recv_span = byte_span("nccl", "broadcast", "recv", recv, s)?;
            py.detach(|| {
                let run = |c: &nccl::Comm| {
                    c.broadcast(send_span.as_ref(), &mut recv_span, count, dt, root, s)
                };
                match send {
                    Some(b) => submit(
                        &self.inner,
                        "broadcast",
                        s,
                        [(b, Access::Read), (recv, Access::Write)],
                        run,
                    ),
                    None => submit(&self.inner, "broadcast", s, [(recv, Access::Write)], run),
                }
            })?;
            diag::record_event(
                Level::Info,
                "nccl",
                "broadcast",
                format!("count={count} dtype={dtype} root={root}"),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(())
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (py, send, recv, count, dtype, root, stream);
            Err(unsupported_linux("nccl", "NcclComm::broadcast"))
        }
    }

    /// Launches `ncclReduce` onto `root` rank on `stream`.
    #[expect(
        clippy::too_many_arguments,
        reason = "Direct Python binding matching nccl::Comm::reduce"
    )]
    fn reduce(
        &self,
        py: Python<'_>,
        send: &PyDeviceBuffer,
        recv: Option<&PyDeviceBuffer>,
        count: usize,
        dtype: &str,
        op: &str,
        root: usize,
        stream: &PyStream,
    ) -> PyResult<()> {
        #[cfg(target_os = "linux")]
        {
            let t0 = Instant::now();
            let dt = parse_nccl_dtype(dtype)?;
            let red_op = parse_nccl_redop(op)?;
            let s = &stream.inner;
            let send_span = byte_span("nccl", "reduce", "send", send, s)?;
            let mut recv_span = recv
                .map(|b| byte_span("nccl", "reduce", "recv", b, s))
                .transpose()?;
            py.detach(|| {
                let run = |c: &nccl::Comm| {
                    c.reduce(
                        &send_span,
                        recv_span.as_mut(),
                        count,
                        dt,
                        red_op,
                        root,
                        s,
                    )
                };
                match recv {
                    Some(b) => submit(
                        &self.inner,
                        "reduce",
                        s,
                        [(send, Access::Read), (b, Access::Write)],
                        run,
                    ),
                    None => submit(&self.inner, "reduce", s, [(send, Access::Read)], run),
                }
            })?;
            diag::record_event(
                Level::Info,
                "nccl",
                "reduce",
                format!("count={count} dtype={dtype} op={op} root={root}"),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(())
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (py, send, recv, count, dtype, op, root, stream);
            Err(unsupported_linux("nccl", "NcclComm::reduce"))
        }
    }

    /// Launches `ncclAllGather` (`send_count` elements per rank -> `send_count * nranks` in `recv`).
    fn all_gather(
        &self,
        py: Python<'_>,
        send: &PyDeviceBuffer,
        recv: &PyDeviceBuffer,
        send_count: usize,
        dtype: &str,
        stream: &PyStream,
    ) -> PyResult<()> {
        #[cfg(target_os = "linux")]
        {
            let t0 = Instant::now();
            let dt = parse_nccl_dtype(dtype)?;
            let s = &stream.inner;
            let send_span = byte_span("nccl", "all_gather", "send", send, s)?;
            let mut recv_span = byte_span("nccl", "all_gather", "recv", recv, s)?;
            py.detach(|| {
                submit(
                    &self.inner,
                    "all_gather",
                    s,
                    [(send, Access::Read), (recv, Access::Write)],
                    |c| c.all_gather(&send_span, &mut recv_span, send_count, dt, s),
                )
            })?;
            diag::record_event(
                Level::Info,
                "nccl",
                "all_gather",
                format!("send_count={send_count} dtype={dtype}"),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(())
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (py, send, recv, send_count, dtype, stream);
            Err(unsupported_linux("nccl", "NcclComm::all_gather"))
        }
    }

    /// Launches `ncclReduceScatter` (`recv_count * nranks` in `send` -> `recv_count` in `recv`).
    #[expect(
        clippy::too_many_arguments,
        reason = "Direct Python binding matching nccl::Comm::reduce_scatter"
    )]
    fn reduce_scatter(
        &self,
        py: Python<'_>,
        send: &PyDeviceBuffer,
        recv: &PyDeviceBuffer,
        recv_count: usize,
        dtype: &str,
        op: &str,
        stream: &PyStream,
    ) -> PyResult<()> {
        #[cfg(target_os = "linux")]
        {
            let t0 = Instant::now();
            let dt = parse_nccl_dtype(dtype)?;
            let red_op = parse_nccl_redop(op)?;
            let s = &stream.inner;
            let send_span = byte_span("nccl", "reduce_scatter", "send", send, s)?;
            let mut recv_span = byte_span("nccl", "reduce_scatter", "recv", recv, s)?;
            py.detach(|| {
                submit(
                    &self.inner,
                    "reduce_scatter",
                    s,
                    [(send, Access::Read), (recv, Access::Write)],
                    |c| c.reduce_scatter(&send_span, &mut recv_span, recv_count, dt, red_op, s),
                )
            })?;
            diag::record_event(
                Level::Info,
                "nccl",
                "reduce_scatter",
                format!("recv_count={recv_count} dtype={dtype} op={op}"),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(())
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (py, send, recv, recv_count, dtype, op, stream);
            Err(unsupported_linux("nccl", "NcclComm::reduce_scatter"))
        }
    }

    /// Launches point-to-point `ncclSend` to `peer` on `stream`.
    fn send(
        &self,
        py: Python<'_>,
        send: &PyDeviceBuffer,
        count: usize,
        dtype: &str,
        peer: usize,
        stream: &PyStream,
    ) -> PyResult<()> {
        #[cfg(target_os = "linux")]
        {
            let t0 = Instant::now();
            let dt = parse_nccl_dtype(dtype)?;
            let s = &stream.inner;
            let send_span = byte_span("nccl", "send", "send", send, s)?;
            py.detach(|| {
                submit(
                    &self.inner,
                    "send",
                    s,
                    [(send, Access::Read)],
                    |c| c.send(&send_span, count, dt, peer, s),
                )
            })?;
            diag::record_event(
                Level::Info,
                "nccl",
                "send",
                format!("count={count} dtype={dtype} peer={peer}"),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(())
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (py, send, count, dtype, peer, stream);
            Err(unsupported_linux("nccl", "NcclComm::send"))
        }
    }

    /// Launches point-to-point `ncclRecv` from `peer` on `stream`.
    fn recv(
        &self,
        py: Python<'_>,
        recv: &PyDeviceBuffer,
        count: usize,
        dtype: &str,
        peer: usize,
        stream: &PyStream,
    ) -> PyResult<()> {
        #[cfg(target_os = "linux")]
        {
            let t0 = Instant::now();
            let dt = parse_nccl_dtype(dtype)?;
            let s = &stream.inner;
            let mut recv_span = byte_span("nccl", "recv", "recv", recv, s)?;
            py.detach(|| {
                submit(
                    &self.inner,
                    "recv",
                    s,
                    [(recv, Access::Write)],
                    |c| c.recv(&mut recv_span, count, dt, peer, s),
                )
            })?;
            diag::record_event(
                Level::Info,
                "nccl",
                "recv",
                format!("count={count} dtype={dtype} peer={peer}"),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(())
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (py, recv, count, dtype, peer, stream);
            Err(unsupported_linux("nccl", "NcclComm::recv"))
        }
    }

    fn __repr__(&self) -> String {
        #[cfg(target_os = "linux")]
        {
            format!(
                "NcclComm(rank={}, count={}, device_id={})",
                self.rank, self.nranks, self.device_id
            )
        }
        #[cfg(not(target_os = "linux"))]
        {
            "NcclComm(unsupported)".to_owned()
        }
    }
}

// ---------------------------------------------------------------------------
// nv_rs.cufile (Linux-only GDS runtime)
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
py_err_mapper!(map_cufile_err, "cufile", cufile::Error, {
    cufile::Error::Load(_) => ErrorKind::LibraryNotFound,
    cufile::Error::Invalid(_) | cufile::Error::Unsupported(_) => ErrorKind::InvalidArgument,
    cufile::Error::Io(_) => ErrorKind::General,
    cufile::Error::Driver(_) | cufile::Error::Status { .. } | cufile::Error::Call(_) => {
        ErrorKind::Cuda
    }
});

#[cfg(target_os = "linux")]
py_parse_enum!(parse_io_mode, "cufile", "parse_mode", cufile::IoMode, "cuFile mode (expected 'gds' or 'posix')", {
    "gds" | "gds_only" => cufile::IoMode::GdsOnly,
    "posix" | "posix_fallback" => cufile::IoMode::PosixFallback,
});

/// Returns the loaded `libcufile` library version integer.
#[pyfunction]
#[pyo3(name = "version")]
fn cufile_version(py: Python<'_>) -> PyResult<i32> {
    #[cfg(target_os = "linux")]
    {
        let t0 = Instant::now();
        let ver = py
            .detach(cufile::version)
            .map_err(|e| map_cufile_err("version", e))?;
        diag::record_event(
            Level::Info,
            "cufile",
            "version",
            format!("version={ver}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(ver)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = py;
        Err(unsupported_linux("cufile", "version"))
    }
}

/// `CufileHandle`s registered in this process and not yet deregistered. Whether a registered
/// handle keeps the cuFile driver open by itself is not established, so the driver session is
/// never closed under one: `CufileDriver.close` raises and a collected driver leaves the session
/// open.
#[cfg(target_os = "linux")]
static LIVE_CUFILE_HANDLES: AtomicUsize = AtomicUsize::new(0);

/// A registered handle counted in [`LIVE_CUFILE_HANDLES`] until it is deregistered (`close`) or
/// dropped (the count falls only after the handle itself is gone).
#[cfg(target_os = "linux")]
struct LiveHandle(Option<Arc<cufile::FileHandle>>);

#[cfg(target_os = "linux")]
impl LiveHandle {
    fn new(fh: Arc<cufile::FileHandle>) -> Self {
        LIVE_CUFILE_HANDLES.fetch_add(1, Ordering::SeqCst);
        Self(Some(fh))
    }
}

#[cfg(target_os = "linux")]
impl Drop for LiveHandle {
    fn drop(&mut self) {
        drop(self.0.take());
        LIVE_CUFILE_HANDLES.fetch_sub(1, Ordering::SeqCst);
    }
}

#[cfg(target_os = "linux")]
fn live_handle<'a>(slot: &'a Option<LiveHandle>, op: &str) -> PyResult<&'a cufile::FileHandle> {
    slot.as_ref()
        .and_then(|h| h.0.as_deref())
        .ok_or_else(|| map_cufile_err(op, cufile::Error::Invalid("cuFile handle already closed")))
}

/// RAII GPUDirect Storage driver session (`cuFileDriverOpen` / `cuFileDriverClose_v2`).
#[pyclass(name = "CufileDriver", module = "nv_rs.cufile")]
pub struct PyCufileDriver {
    #[cfg(target_os = "linux")]
    inner: Arc<Mutex<Option<cufile::DriverSession>>>,
}

#[cfg(target_os = "linux")]
impl Drop for PyCufileDriver {
    fn drop(&mut self) {
        let open = LIVE_CUFILE_HANDLES.load(Ordering::SeqCst);
        if open == 0 {
            return;
        }
        if let Some(sess) = lock(&self.inner).take() {
            diag::record_error(
                "cufile",
                "CufileDriver::drop",
                &format!("{open} CufileHandle(s) still registered: driver session left open"),
            );
            std::mem::forget(sess);
        }
    }
}

#[pymethods]
impl PyCufileDriver {
    /// Opens a GPUDirect Storage driver session bound to `context`.
    #[new]
    fn new(py: Python<'_>, context: &PyContext) -> PyResult<Self> {
        #[cfg(target_os = "linux")]
        {
            let t0 = Instant::now();
            let ctx = context.inner.clone();
            let session = py
                .detach(move || cufile::DriverSession::open(&ctx))
                .map_err(|e| map_cufile_err("CufileDriver::new", e))?;
            diag::record_event(
                Level::Info,
                "cufile",
                "driver_open",
                format!("device={}", context.device_id()),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(Self {
                inner: Arc::new(Mutex::new(Some(session))),
            })
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (py, context);
            Err(unsupported_linux("cufile", "CufileDriver::new"))
        }
    }

    /// Returns the process-wide cuFile driver reference count (`cuFileUseCount`).
    #[staticmethod]
    fn use_count(py: Python<'_>) -> PyResult<i64> {
        #[cfg(target_os = "linux")]
        {
            py.detach(cufile::DriverSession::use_count)
                .map_err(|e| map_cufile_err("use_count", e))
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = py;
            Err(unsupported_linux("cufile", "CufileDriver::use_count"))
        }
    }

    /// Queries active cuFile driver and NVFS kernel properties.
    /// @py-return: dict[str, Any]
    fn properties(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        #[cfg(target_os = "linux")]
        {
            let inner = self.inner.clone();
            let props = py
                .detach(move || {
                    let guard = lock(&inner);
                    let sess = guard.as_ref().ok_or(cufile::Error::Invalid(
                        "cuFile driver session already closed",
                    ))?;
                    sess.properties()
                })
                .map_err(|e| map_cufile_err("properties", e))?;
            let nvfs = PyDict::new(py);
            nvfs.set_item("major_version", props.nvfs.major_version)?;
            nvfs.set_item("minor_version", props.nvfs.minor_version)?;
            nvfs.set_item("poll_thresh_size_kb", props.nvfs.poll_thresh_size_kb)?;
            nvfs.set_item("max_direct_io_size_kb", props.nvfs.max_direct_io_size_kb)?;
            nvfs.set_item("driver_status_flags", props.nvfs.driver_status_flags)?;
            nvfs.set_item("driver_control_flags", props.nvfs.driver_control_flags)?;

            let d = PyDict::new(py);
            d.set_item("nvfs", nvfs)?;
            d.set_item("feature_flags", props.feature_flags)?;
            d.set_item("max_device_cache_size_kb", props.max_device_cache_size_kb)?;
            d.set_item("per_buffer_cache_size_kb", props.per_buffer_cache_size_kb)?;
            d.set_item(
                "max_device_pinned_mem_size_kb",
                props.max_device_pinned_mem_size_kb,
            )?;
            d.set_item("max_batch_io_size", props.max_batch_io_size)?;
            d.set_item(
                "max_batch_io_timeout_msecs",
                props.max_batch_io_timeout_msecs,
            )?;
            Ok(d.unbind())
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = py;
            Err(unsupported_linux("cufile", "CufileDriver::properties"))
        }
    }

    /// Explicitly closes the cuFile driver session. Raises `NvInvalidArgumentError` while any
    /// `CufileHandle` is still registered (close those first). Closing again is a no-op.
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        #[cfg(target_os = "linux")]
        {
            let inner = self.inner.clone();
            py.detach(move || {
                let mut guard = lock(&inner);
                if guard.is_none() {
                    return Ok(());
                }
                let open = LIVE_CUFILE_HANDLES.load(Ordering::SeqCst);
                if open > 0 {
                    return Err(record_and_err(
                        "cufile",
                        "close",
                        format!("{open} CufileHandle(s) still registered; close them first"),
                        ErrorKind::InvalidArgument,
                    ));
                }
                guard
                    .take()
                    .map_or(Ok(()), cufile::DriverSession::close)
                    .map_err(|e| map_cufile_err("close", e))
            })
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = py;
            Err(unsupported_linux("cufile", "CufileDriver::close"))
        }
    }

    fn __enter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
    fn __exit__(
        &self,
        py: Python<'_>,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<bool> {
        let _ = (exc_type, exc_val, exc_tb);
        self.close(py)?;
        Ok(false)
    }
}

/// Registered cuFile handle for synchronous GPU direct reads/writes on `DeviceBuffer`
/// (nv_rs-owned buffers and DLPack imports).
#[pyclass(name = "CufileHandle", module = "nv_rs.cufile")]
pub struct PyCufileHandle {
    #[cfg(target_os = "linux")]
    inner: Arc<Mutex<Option<LiveHandle>>>,
}

#[pymethods]
impl PyCufileHandle {
    /// Opens `path` and registers it with cuFile (`mode="gds"` or `"posix"`).
    #[new]
    #[pyo3(signature = (context, path, mode="gds", create=false, truncate=false, read=true, write=true))]
    #[expect(
        clippy::too_many_arguments,
        reason = "Python file-open constructor exposes mode and OpenOptions flags"
    )]
    fn new(
        py: Python<'_>,
        context: &PyContext,
        path: &str,
        mode: &str,
        create: bool,
        truncate: bool,
        read: bool,
        write: bool,
    ) -> PyResult<Self> {
        #[cfg(target_os = "linux")]
        {
            let t0 = Instant::now();
            let io_mode = parse_io_mode(mode)?;
            let ctx = context.inner.clone();
            let path_owned = path.to_owned();
            let fh = py
                .detach(move || -> cufile::Result<Arc<cufile::FileHandle>> {
                    let file = std::fs::OpenOptions::new()
                        .read(read)
                        .write(write)
                        .create(create)
                        .truncate(truncate)
                        .open(&path_owned)?;
                    cufile::FileHandle::register(&ctx, file, io_mode)
                })
                .map_err(|e| map_cufile_err("CufileHandle::new", e))?;
            diag::record_event(
                Level::Info,
                "cufile",
                "handle_register",
                format!("mode={mode} path={path}"),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(Self {
                inner: Arc::new(Mutex::new(Some(LiveHandle::new(fh)))),
            })
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (py, context, path, mode, create, truncate, read, write);
            Err(unsupported_linux("cufile", "CufileHandle::new"))
        }
    }

    /// Synchronously reads up to `len` bytes from `file_offset` into `dst` at `buf_offset`.
    ///
    /// `stream` must order `dst` (its producer stream, `dst.stream`, or one already synchronized
    /// with it): it is synchronized first, because the transfer itself is ordered with no stream
    /// and `dst` may be a stream-ordered allocation or still being written. Unfinished leased
    /// work on `dst` from another stream is rejected with `NvInvalidArgumentError`.
    #[pyo3(signature = (dst, stream, file_offset=0, buf_offset=0, len=None))]
    fn read(
        &self,
        py: Python<'_>,
        dst: &PyDeviceBuffer,
        stream: &PyStream,
        file_offset: u64,
        buf_offset: usize,
        len: Option<usize>,
    ) -> PyResult<usize> {
        #[cfg(target_os = "linux")]
        {
            let t0 = Instant::now();
            let s = &stream.inner;
            let mut span = byte_span("cufile", "read", "dst", dst, s)?;
            let transfer_len = len.unwrap_or_else(|| span.num_bytes().saturating_sub(buf_offset));
            let n = py.detach(|| -> PyResult<usize> {
                s.synchronize()
                    .map_err(|e| record_and_err("cufile", "read", e, ErrorKind::Cuda))?;
                let guard = lock(&self.inner);
                let fh = live_handle(&guard, "read")?;
                let mut n = 0;
                enqueue_leased("cufile", "read", s, [(dst, Access::Write)], || {
                    n = fh
                        .read(file_offset, &mut span, buf_offset, transfer_len)
                        .map_err(|e| map_cufile_err("read", e))?;
                    Ok(())
                })?;
                Ok(n)
            })?;
            diag::record_event(
                Level::Info,
                "cufile",
                "read",
                format!("offset={file_offset} bytes={n}"),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(n)
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (py, dst, stream, file_offset, buf_offset, len);
            Err(unsupported_linux("cufile", "CufileHandle::read"))
        }
    }

    /// Synchronously writes up to `len` bytes from `src` at `buf_offset` to `file_offset`.
    ///
    /// `stream` must order `src` as for `read`; it is synchronized before the transfer.
    #[pyo3(signature = (src, stream, file_offset=0, buf_offset=0, len=None))]
    fn write(
        &self,
        py: Python<'_>,
        src: &PyDeviceBuffer,
        stream: &PyStream,
        file_offset: u64,
        buf_offset: usize,
        len: Option<usize>,
    ) -> PyResult<usize> {
        #[cfg(target_os = "linux")]
        {
            let t0 = Instant::now();
            let s = &stream.inner;
            let span = byte_span("cufile", "write", "src", src, s)?;
            let transfer_len = len.unwrap_or_else(|| span.num_bytes().saturating_sub(buf_offset));
            let n = py.detach(|| -> PyResult<usize> {
                s.synchronize()
                    .map_err(|e| record_and_err("cufile", "write", e, ErrorKind::Cuda))?;
                let guard = lock(&self.inner);
                let fh = live_handle(&guard, "write")?;
                let mut n = 0;
                enqueue_leased("cufile", "write", s, [(src, Access::Read)], || {
                    n = fh
                        .write(file_offset, &span, buf_offset, transfer_len)
                        .map_err(|e| map_cufile_err("write", e))?;
                    Ok(())
                })?;
                Ok(n)
            })?;
            diag::record_event(
                Level::Info,
                "cufile",
                "write",
                format!("offset={file_offset} bytes={n}"),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(n)
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (py, src, stream, file_offset, buf_offset, len);
            Err(unsupported_linux("cufile", "CufileHandle::write"))
        }
    }

    /// Deregisters the cuFile handle and closes the underlying file. Closing again is a no-op.
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        #[cfg(target_os = "linux")]
        {
            let inner = self.inner.clone();
            py.detach(move || {
                let taken = lock(&inner).take();
                // `live` is dropped (and uncounted) only after the deregistration below.
                if let Some(mut live) = taken
                    && let Some(fh) = live.0.take()
                {
                    let file = cufile::FileHandle::deregister(fh)?;
                    drop(file);
                }
                Ok(())
            })
            .map_err(|e| map_cufile_err("close", e))
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = py;
            Err(unsupported_linux("cufile", "CufileHandle::close"))
        }
    }

    #[getter]
    fn mode(&self) -> PyResult<&'static str> {
        #[cfg(target_os = "linux")]
        {
            let guard = lock(&self.inner);
            Ok(match live_handle(&guard, "mode")?.mode() {
                cufile::IoMode::GdsOnly => "gds",
                cufile::IoMode::PosixFallback => "posix",
            })
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(unsupported_linux("cufile", "CufileHandle::mode"))
        }
    }

    fn __enter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
    fn __exit__(
        &self,
        py: Python<'_>,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<bool> {
        let _ = (exc_type, exc_val, exc_tb);
        self.close(py)?;
        Ok(false)
    }
}

// ---------------------------------------------------------------------------
// nv_rs.nvapi (Windows-only runtime)
// ---------------------------------------------------------------------------

#[cfg(target_os = "windows")]
py_err_mapper!(map_nvapi_err, "nvapi", nvapi::Error, {
    nvapi::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    nvapi::Error::UnsupportedPlatform | nvapi::Error::InvalidInput(_) => {
        ErrorKind::InvalidArgument
    },
    nvapi::Error::Status { .. } => ErrorKind::Cuda,
});

#[cfg(target_os = "windows")]
fn select_physical_gpu(
    sess: &nvapi::NvApi,
    index: usize,
    op: &str,
) -> PyResult<nvapi::PhysicalGpu> {
    let gpus = sess.physical_gpus().map_err(|e| map_nvapi_err(op, e))?;
    gpus.into_iter().nth(index).ok_or_else(|| {
        record_and_err(
            "nvapi",
            op,
            format!("NVAPI GPU index {index} out of range"),
            ErrorKind::InvalidArgument,
        )
    })
}

/// Handle to an initialized Windows NVAPI session.
#[pyclass(name = "NvapiSession", module = "nv_rs.nvapi")]
pub struct PyNvapiSession {
    #[cfg(target_os = "windows")]
    inner: nvapi::NvApi,
}

#[pymethods]
impl PyNvapiSession {
    /// Loads `nvapi64.dll` and initializes an NVAPI session.
    #[new]
    fn new(py: Python<'_>) -> PyResult<Self> {
        #[cfg(target_os = "windows")]
        {
            let t0 = Instant::now();
            let inner = py
                .detach(nvapi::NvApi::init)
                .map_err(|e| map_nvapi_err("NvapiSession::new", e))?;
            diag::record_event(
                Level::Info,
                "nvapi",
                "init",
                "nvapi64.dll initialized".to_owned(),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            Ok(Self { inner })
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = py;
            Err(unsupported_windows("NvapiSession::new"))
        }
    }

    /// Queries the NVAPI interface version string.
    fn interface_version(&self, py: Python<'_>) -> PyResult<String> {
        #[cfg(target_os = "windows")]
        {
            let sess = self.inner.clone();
            py.detach(move || sess.interface_version())
                .map_err(|e| map_nvapi_err("interface_version", e))
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = py;
            Err(unsupported_windows("NvapiSession::interface_version"))
        }
    }

    /// Queries the NVIDIA display driver version and branch (`version`, `major`, `minor`, `branch`).
    /// @py-return: dict[str, Any]
    fn driver_version(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        #[cfg(target_os = "windows")]
        {
            let t0 = Instant::now();
            let sess = self.inner.clone();
            let drv = py
                .detach(move || sess.driver_and_branch_version())
                .map_err(|e| map_nvapi_err("driver_version", e))?;
            let (major, minor) = drv.major_minor();
            diag::record_event(
                Level::Info,
                "nvapi",
                "driver_version",
                format!("{major}.{minor} ({})", drv.branch),
                t0.elapsed().as_nanos() as u64,
                true,
            );
            let d = PyDict::new(py);
            d.set_item("version", drv.version)?;
            d.set_item("major", major)?;
            d.set_item("minor", minor)?;
            d.set_item("branch", drv.branch)?;
            Ok(d.unbind())
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = py;
            Err(unsupported_windows("NvapiSession::driver_version"))
        }
    }

    /// Returns the number of physical WDDM GPUs visible to NVAPI.
    fn gpu_count(&self, py: Python<'_>) -> PyResult<usize> {
        #[cfg(target_os = "windows")]
        {
            let sess = self.inner.clone();
            let gpus = py
                .detach(move || sess.physical_gpus())
                .map_err(|e| map_nvapi_err("gpu_count", e))?;
            Ok(gpus.len())
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = py;
            Err(unsupported_windows("NvapiSession::gpu_count"))
        }
    }

    /// Returns the full marketing names of all physical WDDM GPUs.
    fn gpu_names(&self, py: Python<'_>) -> PyResult<Vec<String>> {
        #[cfg(target_os = "windows")]
        {
            let sess = self.inner.clone();
            py.detach(move || -> nvapi::Result<Vec<String>> {
                let gpus = sess.physical_gpus()?;
                gpus.iter().map(|g| g.full_name()).collect()
            })
            .map_err(|e| map_nvapi_err("gpu_names", e))
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = py;
            Err(unsupported_windows("NvapiSession::gpu_names"))
        }
    }

    /// Queries thermal sensor readings for physical GPU `index`.
    /// @py-return: list[dict[str, Any]]
    #[pyo3(signature = (index=0))]
    fn thermal_info(&self, py: Python<'_>, index: usize) -> PyResult<Vec<Py<PyDict>>> {
        #[cfg(target_os = "windows")]
        {
            let sess = self.inner.clone();
            let sensors = py.detach(move || -> PyResult<Vec<nvapi::ThermalSensor>> {
                let gpu = select_physical_gpu(&sess, index, "thermal_info")?;
                gpu.thermal_settings()
                    .map_err(|e| map_nvapi_err("thermal_info", e))
            })?;
            let mut out = Vec::with_capacity(sensors.len());
            for s in sensors {
                let d = PyDict::new(py);
                d.set_item("controller", s.controller)?;
                d.set_item("target", s.target)?;
                d.set_item("current_temp_c", s.current_temp_c)?;
                d.set_item("default_min_c", s.default_min_c)?;
                d.set_item("default_max_c", s.default_max_c)?;
                out.push(d.unbind());
            }
            Ok(out)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (py, index);
            Err(unsupported_windows("NvapiSession::thermal_info"))
        }
    }

    /// Queries detailed hardware, memory, clock, and P-state telemetry for physical GPU `index`.
    /// @py-return: dict[str, Any]
    #[pyo3(signature = (index=0))]
    fn gpu_info(&self, py: Python<'_>, index: usize) -> PyResult<Py<PyDict>> {
        #[cfg(target_os = "windows")]
        {
            let sess = self.inner.clone();
            let (name, pci, arch, cores, vbios, mem, pstate, clocks) = py.detach(move || {
                let gpu = select_physical_gpu(&sess, index, "gpu_info")?;
                let name = gpu.full_name().map_err(|e| map_nvapi_err("gpu_info", e))?;
                let pci = gpu
                    .pci_identifiers()
                    .map_err(|e| map_nvapi_err("gpu_info", e))?;
                let arch = gpu.arch_info().map_err(|e| map_nvapi_err("gpu_info", e))?;
                let cores = gpu.core_count().map_err(|e| map_nvapi_err("gpu_info", e))?;
                let vbios = gpu
                    .vbios_version()
                    .map_err(|e| map_nvapi_err("gpu_info", e))?;
                let mem = gpu
                    .memory_info()
                    .map_err(|e| map_nvapi_err("gpu_info", e))?;
                let pstate = gpu
                    .current_pstate()
                    .map_err(|e| map_nvapi_err("gpu_info", e))?;
                let clocks = gpu
                    .clock_frequencies(nvapi::ClockKind::Current)
                    .map_err(|e| map_nvapi_err("gpu_info", e))?;
                Ok::<_, PyErr>((name, pci, arch, cores, vbios, mem, pstate, clocks))
            })?;

            let d = PyDict::new(py);
            d.set_item("name", name)?;
            d.set_item("device_id", pci.device_id)?;
            d.set_item("subsystem_id", pci.subsystem_id)?;
            d.set_item("revision_id", pci.revision_id)?;
            d.set_item("architecture", arch.architecture)?;
            d.set_item("core_count", cores)?;
            d.set_item("vbios_version", vbios)?;
            d.set_item("dedicated_bytes", mem.dedicated_bytes)?;
            d.set_item("available_dedicated_bytes", mem.available_dedicated_bytes)?;
            d.set_item("current_pstate", pstate)?;
            d.set_item("graphics_clock_khz", clocks.graphics_khz)?;
            d.set_item("memory_clock_khz", clocks.memory_khz)?;
            Ok(d.unbind())
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (py, index);
            Err(unsupported_windows("NvapiSession::gpu_info"))
        }
    }

    /// Enumerates attached NVIDIA displays along with HDR and VRR capabilities.
    /// @py-return: list[dict[str, Any]]
    fn display_info(&self, py: Python<'_>) -> PyResult<Vec<Py<PyDict>>> {
        #[cfg(target_os = "windows")]
        {
            let sess = self.inner.clone();
            let entries = py.detach(move || -> PyResult<Vec<_>> {
                let displays = sess
                    .displays()
                    .map_err(|e| map_nvapi_err("display_info", e))?;
                let mut rows = Vec::with_capacity(displays.len());
                for disp in displays {
                    let name = disp.name().map_err(|e| map_nvapi_err("display_info", e))?;
                    let display_id = disp
                        .display_id()
                        .map_err(|e| map_nvapi_err("display_info", e))?;
                    let hdr = match disp.hdr_capabilities() {
                        Ok(h) => Some(h),
                        Err(nvapi::Error::Status { status, .. }) if status.is_not_supported() => {
                            None
                        }
                        Err(e) => return Err(map_nvapi_err("display_info", e)),
                    };
                    let vrr = match disp.vrr_info() {
                        Ok(v) => Some(v),
                        Err(nvapi::Error::Status { status, .. }) if status.is_not_supported() => {
                            None
                        }
                        Err(e) => return Err(map_nvapi_err("display_info", e)),
                    };
                    rows.push((name, display_id, hdr, vrr));
                }
                Ok(rows)
            })?;

            let mut out = Vec::with_capacity(entries.len());
            for (name, display_id, hdr, vrr) in entries {
                let d = PyDict::new(py);
                d.set_item("name", name)?;
                d.set_item("display_id", display_id)?;
                if let Some(h) = hdr {
                    let hd = PyDict::new(py);
                    hd.set_item("st2084_eotf", h.st2084_eotf)?;
                    hd.set_item("edr", h.edr)?;
                    hd.set_item("dolby_vision", h.dolby_vision)?;
                    hd.set_item("hdr10_plus", h.hdr10_plus)?;
                    hd.set_item("nvidia_certified", h.nvidia_certified)?;
                    hd.set_item("max_luminance_nits", h.max_luminance_nits)?;
                    d.set_item("hdr", hd)?;
                } else {
                    d.set_item("hdr", py.None())?;
                }
                if let Some(v) = vrr {
                    let vd = PyDict::new(py);
                    vd.set_item("enabled", v.enabled)?;
                    vd.set_item("possible", v.possible)?;
                    vd.set_item("requested", v.requested)?;
                    vd.set_item("in_vrr_mode", v.in_vrr_mode)?;
                    d.set_item("vrr", vd)?;
                } else {
                    d.set_item("vrr", py.None())?;
                }
                out.push(d.unbind());
            }
            Ok(out)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = py;
            Err(unsupported_windows("NvapiSession::display_info"))
        }
    }

    fn __repr__(&self) -> &'static str {
        #[cfg(target_os = "windows")]
        {
            "NvapiSession(active=True)"
        }
        #[cfg(not(target_os = "windows"))]
        {
            "NvapiSession(unsupported)"
        }
    }
}

// ---------------------------------------------------------------------------
// Module registration
// ---------------------------------------------------------------------------

/// Registers `nv_rs.nccl`, `nv_rs.cufile`, and `nv_rs.nvapi` cross-platform.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    py_register_submodule!(py, parent, "nccl", |m| {
        m.add("SUPPORTED", cfg!(target_os = "linux"))?;
        m.add_function(wrap_pyfunction!(nccl_version, &m)?)?;
        m.add_function(wrap_pyfunction!(nccl_unique_id, &m)?)?;
        m.add_function(wrap_pyfunction!(nccl_group_start, &m)?)?;
        m.add_function(wrap_pyfunction!(nccl_group_end, &m)?)?;
        m.add_class::<PyNcclComm>()?;
    })?;

    py_register_submodule!(py, parent, "cufile", |m| {
        m.add("SUPPORTED", cfg!(target_os = "linux"))?;
        m.add_function(wrap_pyfunction!(cufile_version, &m)?)?;
        m.add_class::<PyCufileDriver>()?;
        m.add_class::<PyCufileHandle>()?;
    })?;

    py_register_submodule!(py, parent, "nvapi", |m| {
        m.add("SUPPORTED", cfg!(target_os = "windows"))?;
        m.add_class::<PyNvapiSession>()?;
    })?;

    Ok(())
}
