//! NCCL communicator (`Comm`), unique rendezvous identifier (`UniqueId`), collective/P2P
//! launches, async error inspection, group fusion, and abort-safe teardown.

use std::marker::PhantomData;
use std::sync::Arc;

use cuda_core::{Context, DeviceBuffer, Stream};
use cuda_core_ext::same_context;

#[cfg(target_os = "linux")]
use std::ffi::c_void;

use crate::{DataType, DeviceSpan, Error, NcclStatus, RedOp, Result, same_ctx};
#[cfg(target_os = "linux")]
use crate::{check, record_teardown};

/// Opaque 128-byte NCCL communicator rendezvous identifier (`ncclUniqueId`).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UniqueId(
    /// Raw 128-byte `ncclUniqueId` payload.
    pub [u8; 128],
);

impl UniqueId {
    /// Allocates a fresh NCCL unique identifier on the coordinator rank (`ncclGetUniqueId`).
    pub fn new() -> Result<Self> {
        #[cfg(target_os = "linux")]
        {
            nccl_sys::load()?;
            let mut raw = nccl_sys::ncclUniqueId::default();
            // SAFETY: `raw` is stack-allocated and matches `ncclUniqueId` C ABI layout.
            let status = unsafe { nccl_sys::ncclGetUniqueId(&mut raw) };
            check("ncclGetUniqueId", status)?;
            Ok(Self::from_sys(raw))
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(Error::Unsupported("ncclGetUniqueId requires Linux"))
        }
    }

    /// Constructs a `UniqueId` from serialized 128-byte wire payload.
    pub const fn from_bytes(bytes: [u8; 128]) -> Self {
        Self(bytes)
    }

    /// Returns the raw 128-byte payload for distribution across ranks.
    pub const fn as_bytes(&self) -> &[u8; 128] {
        &self.0
    }

    #[cfg(target_os = "linux")]
    fn to_sys(self) -> nccl_sys::ncclUniqueId {
        const _: () = assert!(size_of::<nccl_sys::ncclUniqueId>() == 128);
        const _: () = assert!(align_of::<nccl_sys::ncclUniqueId>() == 1);
        // SAFETY: identical 128-byte 1-aligned layout asserted at compile time; avoids signedness
        // differences in `c_char` across x86_64/aarch64.
        unsafe { std::mem::transmute::<[u8; 128], nccl_sys::ncclUniqueId>(self.0) }
    }

    #[cfg(target_os = "linux")]
    fn from_sys(raw: nccl_sys::ncclUniqueId) -> Self {
        const _: () = assert!(size_of::<nccl_sys::ncclUniqueId>() == 128);
        // SAFETY: inverse 128-byte POD transmute asserted above.
        Self(unsafe { std::mem::transmute::<nccl_sys::ncclUniqueId, [u8; 128]>(raw) })
    }
}

impl std::fmt::Debug for UniqueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UniqueId({:02x}{:02x}..)", self.0[0], self.0[1])
    }
}

/// Asynchronous communicator state reported by [`Comm::async_error`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AsyncState {
    /// No asynchronous error observed (`ncclSuccess`).
    Ok,
    /// Non-blocking communicator init/finalize or collective work is in progress (`ncclInProgress`).
    InProgress,
    /// Communicator encountered an asynchronous fault and should be aborted via [`Comm::abort`].
    Failed(NcclStatus),
}

/// RAII guard for a fused NCCL group (`ncclGroupStart` / `ncclGroupEnd`).
///
/// Dropping an unclosed guard automatically invokes `ncclGroupEnd` without panicking; its status
/// cannot be returned from `Drop` and is discarded. Call [`GroupGuard::finish`] to observe any
/// group-end error explicitly. Ending the group launches every operation enqueued in it so far,
/// so on an error path the peers must still post matching operations, or the communicators must
/// be aborted.
///
/// NCCL group nesting is per thread, so the guard is `!Send` and `!Sync`: it must end the group
/// on the thread that started it.
pub struct GroupGuard {
    active: bool,
    /// Pins the guard to the thread whose NCCL group state it owns.
    _thread_bound: PhantomData<*const ()>,
}

impl GroupGuard {
    /// Begins an NCCL group (`ncclGroupStart`) and returns a guard that ends it on [`Self::finish`]
    /// or `Drop`.
    pub fn start() -> Result<Self> {
        group_start()?;
        Ok(Self {
            active: true,
            _thread_bound: PhantomData,
        })
    }

    /// Explicitly ends the NCCL group (`ncclGroupEnd`) and returns its status.
    pub fn finish(mut self) -> Result<()> {
        self.active = false;
        group_end()
    }
}

impl Drop for GroupGuard {
    fn drop(&mut self) {
        if self.active {
            self.active = false;
            // `Drop` cannot report the status; `finish` is the observable path (documented above).
            let _ = group_end();
        }
    }
}

/// Begins a fused NCCL operation group (`ncclGroupStart`).
pub fn group_start() -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        nccl_sys::load()?;
        // SAFETY: `ncclGroupStart` takes no arguments and toggles thread-local group nesting.
        check("ncclGroupStart", unsafe { nccl_sys::ncclGroupStart() })
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(Error::Unsupported("ncclGroupStart requires Linux"))
    }
}

/// Ends a fused NCCL operation group (`ncclGroupEnd`).
pub fn group_end() -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        nccl_sys::load()?;
        // SAFETY: `ncclGroupEnd` ends the innermost thread-local NCCL group.
        check("ncclGroupEnd", unsafe { nccl_sys::ncclGroupEnd() })
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(Error::Unsupported("ncclGroupEnd requires Linux"))
    }
}

/// Executes `f` inside a `ncclGroupStart` / `ncclGroupEnd` scope, ensuring `ncclGroupEnd` always
/// runs even if `f` returns an error.
///
/// NCCL has no way to discard a group: when `f` fails after enqueuing some operations, ending
/// the group still launches them, and `f`'s error is returned (a group-end error is then not
/// reported). Peers must post the matching operations, or the communicators must be aborted
/// ([`Comm::abort`]) to avoid a hang.
pub fn group<T>(f: impl FnOnce() -> Result<T>) -> Result<T> {
    let guard = GroupGuard::start()?;
    let result = f();
    match (result, guard.finish()) {
        (Ok(val), Ok(())) => Ok(val),
        (Err(e), _) => Err(e),
        (Ok(_), Err(end_err)) => Err(end_err),
    }
}

/// Owned NCCL communicator (`ncclComm_t`) bound to one [`Arc<Context>`].
///
/// On `Drop`, `Comm` queries `ncclCommGetAsyncError`: if it reports an async error or an
/// in-progress non-blocking operation, it calls `ncclCommAbort`; otherwise it calls
/// `ncclCommDestroy`. Any NCCL teardown failure (including the async state that forced the
/// abort) is kept with its real `ncclResult_t` in `cuda_core_ext::teardown` under
/// `Library::Nccl`; a failed context bind stays on [`Context`] and skips the native calls.
///
/// `ncclCommDestroy` finalizes the communicator implicitly, which waits for every operation
/// already enqueued on it, so **`Drop` can block** while a peer has not posted matching work
/// (`ncclCommGetAsyncError` does not report outstanding collectives). Call [`Comm::abort`] for a
/// teardown that never waits. A failed `ncclCommDestroy` is not followed by `ncclCommAbort`: the
/// handle must not be used after destroy returns, whatever its status.
pub struct Comm {
    #[cfg(target_os = "linux")]
    raw: Option<nccl_sys::ncclComm_t>,
    ctx: Arc<Context>,
    nranks: usize,
    rank: usize,
    device: i32,
}

// SAFETY: `Comm` exposes thread-safe send when moved between threads while callers uphold
// CUDA context/stream rules (`bind_to_thread` is called prior to FFI).
unsafe impl Send for Comm {}

/// CUDA device ordinal of `ctx` as the C `int` NCCL expects. `Context::cu_device` is a driver
/// handle, which the CUDA API does not promise to equal the ordinal.
fn device_ordinal(ctx: &Context) -> Result<i32> {
    i32::try_from(ctx.ordinal()).map_err(|_| Error::Invalid("CUDA device ordinal exceeds i32"))
}

fn check_elem_span(count: usize, dtype: DataType, bytes: usize, label: &'static str) -> Result<()> {
    let needed = count
        .checked_mul(dtype.elem_bytes())
        .ok_or(Error::Invalid("element count overflow"))?;
    if bytes < needed {
        return Err(Error::Invalid(label));
    }
    Ok(())
}

impl Comm {
    /// Initializes a single-process clique of communicators across `contexts` (`ncclCommInitAll`).
    ///
    /// Each context must represent a distinct CUDA device.
    pub fn init_all(contexts: &[Arc<Context>]) -> Result<Vec<Self>> {
        if contexts.is_empty() {
            return Err(Error::Invalid(
                "init_all requires at least one CUDA context",
            ));
        }
        let ndev = contexts.len();
        let _ = i32::try_from(ndev).map_err(|_| Error::Invalid("too many ranks for i32"))?;

        #[cfg(target_os = "linux")]
        {
            nccl_sys::load()?;
            let mut devs = Vec::with_capacity(ndev);
            for ctx in contexts {
                // `devlist` takes CUDA device ordinals, not driver `CUdevice` handles.
                let dev = device_ordinal(ctx)?;
                if devs.contains(&dev) {
                    return Err(Error::Invalid("init_all requires distinct CUDA devices"));
                }
                devs.push(dev);
            }
            let mut raw_comms = vec![std::ptr::null_mut(); ndev];
            // SAFETY: `raw_comms` and `devs` hold `ndev` elements; `ndev` fits `i32`.
            let status = unsafe {
                nccl_sys::ncclCommInitAll(raw_comms.as_mut_ptr(), ndev as i32, devs.as_ptr())
            };
            if let Err(err) = check("ncclCommInitAll", status) {
                for raw in raw_comms {
                    if !raw.is_null() {
                        // SAFETY: abort any partially initialized communicator on failure.
                        let abort_status = unsafe { nccl_sys::ncclCommAbort(raw) };
                        record_teardown("ncclCommAbort", abort_status);
                    }
                }
                return Err(err);
            }

            let mut out = Vec::with_capacity(ndev);
            for (idx, ((raw, ctx), &device)) in raw_comms
                .iter()
                .copied()
                .zip(contexts.iter())
                .zip(devs.iter())
                .enumerate()
            {
                let rank = match Self::query_rank(raw) {
                    Ok(r) => r,
                    Err(err) => {
                        for &rem in &raw_comms[idx..] {
                            if !rem.is_null() {
                                // SAFETY: abort remaining raw handles not yet wrapped in `Comm`.
                                let abort_status = unsafe { nccl_sys::ncclCommAbort(rem) };
                                record_teardown("ncclCommAbort", abort_status);
                            }
                        }
                        drop(out);
                        return Err(err);
                    }
                };
                out.push(Self {
                    raw: Some(raw),
                    ctx: ctx.clone(),
                    nranks: ndev,
                    rank,
                    device,
                });
            }
            Ok(out)
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(Error::Unsupported("ncclCommInitAll requires Linux"))
        }
    }

    /// Initializes one rank of a multi-thread / multi-process communicator (`ncclCommInitRank`).
    pub fn init_rank(
        ctx: &Arc<Context>,
        nranks: usize,
        unique_id: UniqueId,
        rank: usize,
    ) -> Result<Self> {
        if nranks == 0 || rank >= nranks {
            return Err(Error::Invalid("rank must satisfy 0 <= rank < nranks"));
        }
        let nranks_i = i32::try_from(nranks).map_err(|_| Error::Invalid("nranks exceeds i32"))?;
        let rank_i = i32::try_from(rank).map_err(|_| Error::Invalid("rank exceeds i32"))?;
        let device = device_ordinal(ctx)?;

        #[cfg(target_os = "linux")]
        {
            nccl_sys::load()?;
            ctx.bind_to_thread()?;
            let mut raw: nccl_sys::ncclComm_t = std::ptr::null_mut();
            // SAFETY: `ctx` is active on this thread, `raw` is a valid out-pointer, and `unique_id`
            // is copied by value matching `ncclUniqueId`.
            let status = unsafe {
                nccl_sys::ncclCommInitRank(&mut raw, nranks_i, unique_id.to_sys(), rank_i)
            };
            check("ncclCommInitRank", status)?;
            Ok(Self {
                raw: Some(raw),
                ctx: ctx.clone(),
                nranks,
                rank,
                device,
            })
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (ctx, unique_id, nranks_i, rank_i, device);
            Err(Error::Unsupported("ncclCommInitRank requires Linux"))
        }
    }

    #[cfg(target_os = "linux")]
    fn query_rank(raw: nccl_sys::ncclComm_t) -> Result<usize> {
        let mut r = 0i32;
        // SAFETY: `raw` is an initialized communicator.
        check("ncclCommUserRank", unsafe {
            nccl_sys::ncclCommUserRank(raw, &mut r)
        })?;
        usize::try_from(r).map_err(|_| Error::Invalid("negative rank returned by NCCL"))
    }

    /// Owning CUDA context.
    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }

    /// Number of ranks in this communicator clique.
    pub const fn count(&self) -> usize {
        self.nranks
    }

    /// CUDA device ordinal associated with this communicator.
    pub const fn cu_device(&self) -> i32 {
        self.device
    }

    /// Zero-based user rank of this communicator.
    pub const fn user_rank(&self) -> usize {
        self.rank
    }

    /// Queries the live rank count via `ncclCommCount`.
    pub fn query_count(&self) -> Result<usize> {
        #[cfg(target_os = "linux")]
        {
            let raw = self
                .raw
                .ok_or(Error::Invalid("communicator already destroyed"))?;
            let mut c = 0i32;
            // SAFETY: `raw` is live.
            check("ncclCommCount", unsafe {
                nccl_sys::ncclCommCount(raw, &mut c)
            })?;
            usize::try_from(c).map_err(|_| Error::Invalid("negative comm count"))
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(Error::Unsupported("ncclCommCount requires Linux"))
        }
    }

    /// Queries asynchronous communicator status (`ncclCommGetAsyncError`).
    pub fn async_error(&self) -> Result<AsyncState> {
        #[cfg(target_os = "linux")]
        {
            let raw = self
                .raw
                .ok_or(Error::Invalid("communicator already destroyed"))?;
            let mut async_err: nccl_sys::ncclResult_t = 0;
            // SAFETY: `raw` is live and `async_err` is a stack out-pointer.
            let status = unsafe { nccl_sys::ncclCommGetAsyncError(raw, &mut async_err) };
            check("ncclCommGetAsyncError", status)?;
            let code = NcclStatus(async_err);
            if code.is_success() {
                Ok(AsyncState::Ok)
            } else if code.is_in_progress() {
                Ok(AsyncState::InProgress)
            } else {
                Ok(AsyncState::Failed(code))
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(Error::Unsupported("ncclCommGetAsyncError requires Linux"))
        }
    }

    /// Returns NCCL's human-readable last error string for this communicator, if available.
    pub fn last_error(&self) -> Option<String> {
        #[cfg(target_os = "linux")]
        {
            let raw = self.raw?;
            // SAFETY: `ncclGetLastError` accepts a valid `ncclComm_t` and returns static C string or null.
            let ptr = unsafe { nccl_sys::ncclGetLastError(raw) };
            if ptr.is_null() {
                None
            } else {
                // SAFETY: non-null return is a valid NUL-terminated C string.
                Some(
                    unsafe { std::ffi::CStr::from_ptr(ptr) }
                        .to_string_lossy()
                        .into_owned(),
                )
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            None
        }
    }

    /// Gracefully finalizes pending communications and destroys the communicator (`ncclCommFinalize`
    /// + `ncclCommDestroy`), aborting via `ncclCommAbort` if finalization reports a failure.
    pub fn finalize(mut self) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            if self.raw.is_none() {
                return Ok(());
            }
            self.ctx.bind_to_thread()?;
            let Some(raw) = self.raw.take() else {
                return Ok(());
            };
            // SAFETY: `raw` is owned exclusively and taken once.
            let fin_status = unsafe { nccl_sys::ncclCommFinalize(raw) };
            let mut async_err = fin_status;
            if fin_status == 0 || NcclStatus(fin_status).is_in_progress() {
                loop {
                    let mut err_code = 0u32;
                    // SAFETY: polling async completion of `ncclCommFinalize`.
                    let poll = unsafe { nccl_sys::ncclCommGetAsyncError(raw, &mut err_code) };
                    if poll != 0 {
                        async_err = poll;
                        break;
                    }
                    async_err = err_code;
                    if !NcclStatus(err_code).is_in_progress() {
                        break;
                    }
                    std::thread::yield_now();
                }
            }
            if async_err == 0 {
                // SAFETY: `ncclCommFinalize` completed cleanly. `raw` is not used after this
                // call, whatever it returns (no abort on a handle destroy may have freed).
                let destroy = unsafe { nccl_sys::ncclCommDestroy(raw) };
                return check("ncclCommDestroy", destroy);
            }
            // SAFETY: abort failed communicator rather than blocking in destroy.
            let abort_status = unsafe { nccl_sys::ncclCommAbort(raw) };
            record_teardown("ncclCommAbort", abort_status);
            check("ncclCommFinalize", async_err)
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = &mut self;
            Err(Error::Unsupported("ncclCommFinalize requires Linux"))
        }
    }

    /// Immediately aborts all queued work and destroys the communicator (`ncclCommAbort`).
    pub fn abort(mut self) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            if self.raw.is_none() {
                return Ok(());
            }
            self.ctx.bind_to_thread()?;
            let Some(raw) = self.raw.take() else {
                return Ok(());
            };
            // SAFETY: `raw` is owned exclusively and taken once.
            check("ncclCommAbort", unsafe { nccl_sys::ncclCommAbort(raw) })
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = &mut self;
            Err(Error::Unsupported("ncclCommAbort requires Linux"))
        }
    }

    /// Launches `ncclAllReduce` on `stream`.
    pub fn all_reduce<S, R>(
        &self,
        send: &S,
        recv: &mut DeviceBuffer<R>,
        count: usize,
        dtype: DataType,
        op: RedOp,
        stream: &Stream,
    ) -> Result<()>
    where
        S: DeviceSpan + ?Sized,
        R: Sized,
    {
        same_ctx(stream, &self.ctx)?;
        if !same_context(send.context(), &self.ctx) || !same_context(recv.context(), &self.ctx) {
            return Err(Error::Invalid(
                "all_reduce buffers belong to another CUDA context",
            ));
        }
        check_elem_span(
            count,
            dtype,
            send.num_bytes(),
            "send buffer smaller than count * dtype",
        )?;
        check_elem_span(
            count,
            dtype,
            recv.num_bytes(),
            "recv buffer smaller than count * dtype",
        )?;

        #[cfg(target_os = "linux")]
        {
            let raw = self
                .raw
                .ok_or(Error::Invalid("communicator already destroyed"))?;
            self.ctx.bind_to_thread()?;
            // SAFETY: context is bound, buffers satisfy byte spans and belong to `self.ctx`.
            let status = unsafe {
                nccl_sys::ncclAllReduce(
                    send.cu_deviceptr() as *const c_void,
                    recv.cu_deviceptr() as *mut c_void,
                    count,
                    dtype.to_sys(),
                    op.to_sys(),
                    raw,
                    stream.cu_stream(),
                )
            };
            check("ncclAllReduce", status)
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = op;
            Err(Error::Unsupported("ncclAllReduce requires Linux"))
        }
    }

    /// Launches an in-place `ncclAllReduce` on `stream` (`sendbuff == recvbuff == buf`), which
    /// [`Self::all_reduce`] cannot express because one buffer cannot be borrowed shared and
    /// exclusive at once.
    pub fn all_reduce_in_place<T>(
        &self,
        buf: &mut DeviceBuffer<T>,
        count: usize,
        dtype: DataType,
        op: RedOp,
        stream: &Stream,
    ) -> Result<()> {
        same_ctx(stream, &self.ctx)?;
        if !same_context(buf.context(), &self.ctx) {
            return Err(Error::Invalid(
                "all_reduce_in_place buffer belongs to another CUDA context",
            ));
        }
        check_elem_span(
            count,
            dtype,
            buf.num_bytes(),
            "buffer smaller than count * dtype",
        )?;

        #[cfg(target_os = "linux")]
        {
            let raw = self
                .raw
                .ok_or(Error::Invalid("communicator already destroyed"))?;
            self.ctx.bind_to_thread()?;
            let ptr = buf.cu_deviceptr();
            // SAFETY: context is bound, `buf` spans `count * dtype` bytes and belongs to
            // `self.ctx`; NCCL documents `sendbuff == recvbuff` as in-place operation.
            let status = unsafe {
                nccl_sys::ncclAllReduce(
                    ptr as *const c_void,
                    ptr as *mut c_void,
                    count,
                    dtype.to_sys(),
                    op.to_sys(),
                    raw,
                    stream.cu_stream(),
                )
            };
            check("ncclAllReduce", status)
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = op;
            Err(Error::Unsupported("ncclAllReduce requires Linux"))
        }
    }

    /// Launches `ncclBroadcast` from `root` rank into `recv` on `stream`.
    ///
    /// On `self.user_rank() == root`, `send` must be `Some(...)` spanning at least `count` elements.
    pub fn broadcast<S, R>(
        &self,
        send: Option<&S>,
        recv: &mut DeviceBuffer<R>,
        count: usize,
        dtype: DataType,
        root: usize,
        stream: &Stream,
    ) -> Result<()>
    where
        S: DeviceSpan + ?Sized,
        R: Sized,
    {
        same_ctx(stream, &self.ctx)?;
        if root >= self.nranks {
            return Err(Error::Invalid("broadcast root out of range"));
        }
        if !same_context(recv.context(), &self.ctx) {
            return Err(Error::Invalid(
                "recv buffer belongs to another CUDA context",
            ));
        }
        check_elem_span(
            count,
            dtype,
            recv.num_bytes(),
            "recv buffer smaller than count * dtype",
        )?;

        let send_ptr = match send {
            Some(s) => {
                if !same_context(s.context(), &self.ctx) {
                    return Err(Error::Invalid(
                        "send buffer belongs to another CUDA context",
                    ));
                }
                check_elem_span(
                    count,
                    dtype,
                    s.num_bytes(),
                    "send buffer smaller than count * dtype",
                )?;
                s.cu_deviceptr()
            }
            None if self.rank == root => {
                return Err(Error::Invalid(
                    "broadcast root rank must provide send buffer",
                ));
            }
            None => 0,
        };

        #[cfg(target_os = "linux")]
        {
            let raw = self
                .raw
                .ok_or(Error::Invalid("communicator already destroyed"))?;
            let root_i = i32::try_from(root).map_err(|_| Error::Invalid("root exceeds i32"))?;
            self.ctx.bind_to_thread()?;
            // SAFETY: root send span and all-rank recv span validated above.
            let status = unsafe {
                nccl_sys::ncclBroadcast(
                    send_ptr as *const c_void,
                    recv.cu_deviceptr() as *mut c_void,
                    count,
                    dtype.to_sys(),
                    root_i,
                    raw,
                    stream.cu_stream(),
                )
            };
            check("ncclBroadcast", status)
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = send_ptr;
            Err(Error::Unsupported("ncclBroadcast requires Linux"))
        }
    }

    /// Launches `ncclReduce` onto `root` rank on `stream`.
    ///
    /// On `self.user_rank() == root`, `recv` must be `Some(...)` spanning at least `count` elements.
    #[expect(
        clippy::too_many_arguments,
        reason = "Direct stream-ordered wrapper over ncclReduce"
    )]
    pub fn reduce<S, R>(
        &self,
        send: &S,
        recv: Option<&mut DeviceBuffer<R>>,
        count: usize,
        dtype: DataType,
        op: RedOp,
        root: usize,
        stream: &Stream,
    ) -> Result<()>
    where
        S: DeviceSpan + ?Sized,
        R: Sized,
    {
        same_ctx(stream, &self.ctx)?;
        if root >= self.nranks {
            return Err(Error::Invalid("reduce root out of range"));
        }
        if !same_context(send.context(), &self.ctx) {
            return Err(Error::Invalid(
                "send buffer belongs to another CUDA context",
            ));
        }
        check_elem_span(
            count,
            dtype,
            send.num_bytes(),
            "send buffer smaller than count * dtype",
        )?;

        let recv_ptr = match recv {
            Some(r) => {
                if !same_context(r.context(), &self.ctx) {
                    return Err(Error::Invalid(
                        "recv buffer belongs to another CUDA context",
                    ));
                }
                check_elem_span(
                    count,
                    dtype,
                    r.num_bytes(),
                    "recv buffer smaller than count * dtype",
                )?;
                r.cu_deviceptr()
            }
            None if self.rank == root => {
                return Err(Error::Invalid("reduce root rank must provide recv buffer"));
            }
            None => 0,
        };

        #[cfg(target_os = "linux")]
        {
            let raw = self
                .raw
                .ok_or(Error::Invalid("communicator already destroyed"))?;
            let root_i = i32::try_from(root).map_err(|_| Error::Invalid("root exceeds i32"))?;
            self.ctx.bind_to_thread()?;
            // SAFETY: send span and root recv span validated above.
            let status = unsafe {
                nccl_sys::ncclReduce(
                    send.cu_deviceptr() as *const c_void,
                    recv_ptr as *mut c_void,
                    count,
                    dtype.to_sys(),
                    op.to_sys(),
                    root_i,
                    raw,
                    stream.cu_stream(),
                )
            };
            check("ncclReduce", status)
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (op, recv_ptr);
            Err(Error::Unsupported("ncclReduce requires Linux"))
        }
    }

    /// Launches `ncclAllGather` (`send_count` elements per rank -> `send_count * nranks` in `recv`).
    pub fn all_gather<S, R>(
        &self,
        send: &S,
        recv: &mut DeviceBuffer<R>,
        send_count: usize,
        dtype: DataType,
        stream: &Stream,
    ) -> Result<()>
    where
        S: DeviceSpan + ?Sized,
        R: Sized,
    {
        same_ctx(stream, &self.ctx)?;
        if !same_context(send.context(), &self.ctx) || !same_context(recv.context(), &self.ctx) {
            return Err(Error::Invalid(
                "all_gather buffers belong to another CUDA context",
            ));
        }
        let total_recv = send_count
            .checked_mul(self.nranks)
            .ok_or(Error::Invalid("recv count overflow"))?;
        check_elem_span(
            send_count,
            dtype,
            send.num_bytes(),
            "send buffer smaller than send_count * dtype",
        )?;
        check_elem_span(
            total_recv,
            dtype,
            recv.num_bytes(),
            "recv buffer smaller than send_count * nranks * dtype",
        )?;

        #[cfg(target_os = "linux")]
        {
            let raw = self
                .raw
                .ok_or(Error::Invalid("communicator already destroyed"))?;
            self.ctx.bind_to_thread()?;
            // SAFETY: send and `nranks`-scaled recv byte spans verified above.
            let status = unsafe {
                nccl_sys::ncclAllGather(
                    send.cu_deviceptr() as *const c_void,
                    recv.cu_deviceptr() as *mut c_void,
                    send_count,
                    dtype.to_sys(),
                    raw,
                    stream.cu_stream(),
                )
            };
            check("ncclAllGather", status)
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(Error::Unsupported("ncclAllGather requires Linux"))
        }
    }

    /// Launches `ncclReduceScatter` (`recv_count * nranks` in `send` -> `recv_count` in `recv`).
    pub fn reduce_scatter<S, R>(
        &self,
        send: &S,
        recv: &mut DeviceBuffer<R>,
        recv_count: usize,
        dtype: DataType,
        op: RedOp,
        stream: &Stream,
    ) -> Result<()>
    where
        S: DeviceSpan + ?Sized,
        R: Sized,
    {
        same_ctx(stream, &self.ctx)?;
        if !same_context(send.context(), &self.ctx) || !same_context(recv.context(), &self.ctx) {
            return Err(Error::Invalid(
                "reduce_scatter buffers belong to another CUDA context",
            ));
        }
        let total_send = recv_count
            .checked_mul(self.nranks)
            .ok_or(Error::Invalid("send count overflow"))?;
        check_elem_span(
            total_send,
            dtype,
            send.num_bytes(),
            "send buffer smaller than recv_count * nranks * dtype",
        )?;
        check_elem_span(
            recv_count,
            dtype,
            recv.num_bytes(),
            "recv buffer smaller than recv_count * dtype",
        )?;

        #[cfg(target_os = "linux")]
        {
            let raw = self
                .raw
                .ok_or(Error::Invalid("communicator already destroyed"))?;
            self.ctx.bind_to_thread()?;
            // SAFETY: `nranks`-scaled send and recv byte spans verified above.
            let status = unsafe {
                nccl_sys::ncclReduceScatter(
                    send.cu_deviceptr() as *const c_void,
                    recv.cu_deviceptr() as *mut c_void,
                    recv_count,
                    dtype.to_sys(),
                    op.to_sys(),
                    raw,
                    stream.cu_stream(),
                )
            };
            check("ncclReduceScatter", status)
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = op;
            Err(Error::Unsupported("ncclReduceScatter requires Linux"))
        }
    }

    /// Launches point-to-point `ncclSend` to `peer` on `stream`.
    pub fn send<S>(
        &self,
        send: &S,
        count: usize,
        dtype: DataType,
        peer: usize,
        stream: &Stream,
    ) -> Result<()>
    where
        S: DeviceSpan + ?Sized,
    {
        same_ctx(stream, &self.ctx)?;
        if peer >= self.nranks {
            return Err(Error::Invalid("send peer rank out of range"));
        }
        if !same_context(send.context(), &self.ctx) {
            return Err(Error::Invalid(
                "send buffer belongs to another CUDA context",
            ));
        }
        check_elem_span(
            count,
            dtype,
            send.num_bytes(),
            "send buffer smaller than count * dtype",
        )?;

        #[cfg(target_os = "linux")]
        {
            let raw = self
                .raw
                .ok_or(Error::Invalid("communicator already destroyed"))?;
            let peer_i = i32::try_from(peer).map_err(|_| Error::Invalid("peer exceeds i32"))?;
            self.ctx.bind_to_thread()?;
            // SAFETY: send span and peer validated above.
            let status = unsafe {
                nccl_sys::ncclSend(
                    send.cu_deviceptr() as *const c_void,
                    count,
                    dtype.to_sys(),
                    peer_i,
                    raw,
                    stream.cu_stream(),
                )
            };
            check("ncclSend", status)
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(Error::Unsupported("ncclSend requires Linux"))
        }
    }

    /// Launches point-to-point `ncclRecv` from `peer` on `stream`.
    pub fn recv<R>(
        &self,
        recv: &mut DeviceBuffer<R>,
        count: usize,
        dtype: DataType,
        peer: usize,
        stream: &Stream,
    ) -> Result<()>
    where
        R: Sized,
    {
        same_ctx(stream, &self.ctx)?;
        if peer >= self.nranks {
            return Err(Error::Invalid("recv peer rank out of range"));
        }
        if !same_context(recv.context(), &self.ctx) {
            return Err(Error::Invalid(
                "recv buffer belongs to another CUDA context",
            ));
        }
        check_elem_span(
            count,
            dtype,
            recv.num_bytes(),
            "recv buffer smaller than count * dtype",
        )?;

        #[cfg(target_os = "linux")]
        {
            let raw = self
                .raw
                .ok_or(Error::Invalid("communicator already destroyed"))?;
            let peer_i = i32::try_from(peer).map_err(|_| Error::Invalid("peer exceeds i32"))?;
            self.ctx.bind_to_thread()?;
            // SAFETY: recv span and peer validated above.
            let status = unsafe {
                nccl_sys::ncclRecv(
                    recv.cu_deviceptr() as *mut c_void,
                    count,
                    dtype.to_sys(),
                    peer_i,
                    raw,
                    stream.cu_stream(),
                )
            };
            check("ncclRecv", status)
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(Error::Unsupported("ncclRecv requires Linux"))
        }
    }
}

impl Drop for Comm {
    fn drop(&mut self) {
        #[cfg(target_os = "linux")]
        if let Some(raw) = self.raw.take() {
            // A failed bind puts the driver error back on the context; skip the native calls.
            if !cuda_core_ext::teardown::bind(&self.ctx) {
                return;
            }
            let mut async_err: nccl_sys::ncclResult_t = 0;
            // SAFETY: inspect async error state before deciding between `ncclCommDestroy` and
            // `ncclCommAbort` so Drop never blocks on a faulted communicator.
            let query = unsafe { nccl_sys::ncclCommGetAsyncError(raw, &mut async_err) };
            // A failed query, or a successful query reporting an async fault or an in-progress
            // operation (which the abort below cancels), is kept with its real status.
            record_teardown("ncclCommGetAsyncError", query);
            if query == 0 {
                record_teardown("ncclCommGetAsyncError", async_err);
            }
            if query == 0 && async_err == 0 {
                // SAFETY: communicator reported no async error or in-progress operation. `raw` is
                // not used after this call, whatever it returns (see the type docs).
                let status = unsafe { nccl_sys::ncclCommDestroy(raw) };
                record_teardown("ncclCommDestroy", status);
            } else {
                // SAFETY: async fault or in-progress state; abort immediately to prevent deadlock.
                let status = unsafe { nccl_sys::ncclCommAbort(raw) };
                record_teardown("ncclCommAbort", status);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_unique_id_roundtrip_and_span_checks() {
        let mut bytes = [0u8; 128];
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = i as u8;
        }
        let id = UniqueId::from_bytes(bytes);
        assert_eq!(id.as_bytes(), &bytes);
        assert!(format!("{id:?}").contains("UniqueId(0001..)"));

        assert!(check_elem_span(16, DataType::F32, 64, "buf").is_ok());
        assert!(check_elem_span(16, DataType::F32, 63, "buf").is_err());
        assert!(check_elem_span(usize::MAX, DataType::F64, 64, "buf").is_err());
    }

    #[test]
    fn group_guard_is_neither_send_nor_sync() {
        // NCCL-1 compile-time proof: `ncclGroupStart`/`ncclGroupEnd` nest per thread, so a guard
        // moved or shared across threads would end the group on the wrong thread. If
        // `GroupGuard` were `Send` (or `Sync`), both impls below would apply, the `_` would be
        // ambiguous and this test module would fail to compile.
        trait AmbiguousIfSend<A> {
            fn check() {}
        }
        impl<T: ?Sized> AmbiguousIfSend<()> for T {}
        struct IsSend;
        impl<T: ?Sized + Send> AmbiguousIfSend<IsSend> for T {}
        <GroupGuard as AmbiguousIfSend<_>>::check();

        trait AmbiguousIfSync<A> {
            fn check() {}
        }
        impl<T: ?Sized> AmbiguousIfSync<()> for T {}
        struct IsSync;
        impl<T: ?Sized + Sync> AmbiguousIfSync<IsSync> for T {}
        <GroupGuard as AmbiguousIfSync<_>>::check();
    }
}
