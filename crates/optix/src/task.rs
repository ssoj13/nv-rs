//! Parallel module compilation: [`Module::with_tasks`], [`Task`] and [`PendingModule`].
//!
//! **What.** `optixModuleCreateWithTasks` splits a compile into [`Task`]s. Executing one
//! ([`Task::execute`]) may yield more; every task can run on any thread, in parallel and in any
//! order, until none is left and [`PendingModule::finish`] returns the module. A task can also be
//! fed from an external cache: [`Task::key`] names its output, [`Task::serialize`] stores it after
//! execution, and [`Task::deserialize`] replays it instead of compiling.
//! **Why the shared block.** OptiX reads the input code and writes the log buffer (and its length)
//! during every task, long after the create call returns, and forbids destroying the module while
//! a task runs. A shared block owns copies of the code and compile options, the log buffer and the
//! module; every task and the pending module hold it, so none of that is freed or destroyed while
//! a task can still use it. A read-write gate lets tasks run concurrently (read side) while the
//! log is only read with no task running (write side), so the host never races OptiX's writes.
//! **Thread safety (optix_host.h).** Task execution, `optixModuleGetCompilationState` and the
//! cancel calls are documented safe from any thread until the module is destroyed.
//! **Used by** applications compiling large modules on a thread pool.

use crate::context::creation_flags;
use crate::module::{CompileOpts, log_text};
use crate::{DeviceContext, Error, LOG_BYTES, Module, ModuleOptions, PipelineOptions, Result, check, entry, sys};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, RwLock};

/// Where a task-based compile stands (`OptixModuleCompileState`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompileState {
    /// No task has run yet.
    NotStarted,
    /// Tasks are running or waiting.
    Started,
    /// A task failed (or the compile was cancelled) and returned tasks are still outstanding;
    /// they need not be executed.
    ImpendingFailure,
    /// The compile failed; no task is outstanding.
    Failed,
    /// The module is ready.
    Completed,
}

/// The log buffer and its length cell, at fixed heap addresses OptiX keeps writing to.
struct LogBuf {
    ptr: *mut u8,
    cap: usize,
    len: *mut usize,
}

// SAFETY: the buffer is written only by OptiX inside create/execute calls (which run under the
// gate's read side) and read by the host only under the write side.
unsafe impl Send for LogBuf {}
// SAFETY: as above.
unsafe impl Sync for LogBuf {}

impl LogBuf {
    fn new() -> Self {
        let cap = LOG_BYTES;
        let ptr = Box::into_raw(vec![0u8; cap].into_boxed_slice()).cast::<u8>();
        let len = Box::into_raw(Box::new(cap));
        Self { ptr, cap, len }
    }

    /// The log text.
    ///
    /// # Safety
    ///
    /// No OptiX call that may write the log runs concurrently (hold the gate's write side, or
    /// call before the buffer is shared).
    unsafe fn text(&self) -> String {
        // SAFETY: `ptr`/`len` come from live boxes of `cap` bytes / one usize; the caller excludes
        // concurrent writers.
        let (bytes, len) = unsafe { (std::slice::from_raw_parts(self.ptr, self.cap), *self.len) };
        log_text(bytes, len)
    }
}

impl Drop for LogBuf {
    fn drop(&mut self) {
        // SAFETY: both pointers come from `Box::into_raw` in `new` with these exact types and
        // sizes and are released only here.
        unsafe {
            drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(self.ptr, self.cap)));
            drop(Box::from_raw(self.len));
        }
    }
}

/// What every task of one compile shares. Fields drop in order: the module (if this is its last
/// owner) is destroyed before the buffers OptiX may still reference are freed.
struct Shared {
    module: Arc<Module>,
    gate: RwLock<()>,
    failure: AtomicI32,
    log: LogBuf,
    _code: Box<[u8]>,
    _opts: CompileOpts,
}

impl Shared {
    /// Remembers the first failure code (later ones are consequences).
    fn fail(&self, code: sys::OptixResult) {
        // A lost exchange means an earlier failure is already recorded, which is the one to keep.
        let _first_kept = self.failure.compare_exchange(sys::OptixResult_OPTIX_SUCCESS, code, Ordering::AcqRel, Ordering::Acquire);
    }

    fn ctx(&self) -> &Arc<DeviceContext> {
        self.module.context()
    }
}

/// A module whose compile runs as [`Task`]s.
pub struct PendingModule {
    shared: Arc<Shared>,
}

/// One unit of compile work; send it to any thread.
pub struct Task {
    raw: sys::OptixTask,
    state: TaskState,
    shared: Arc<Shared>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TaskState {
    Pending,
    Executed,
    Deserialized,
}

// SAFETY: OptiX allows executing a task on any thread; the handle is used through `&mut self`
// (execute, deserialize) or `&self` calls OptiX documents as read-only queries.
unsafe impl Send for Task {}

impl Module {
    /// Starts a task-based compile of PTX text or OptiX-IR bytes (copied; OptiX reads them while
    /// tasks run). Returns the pending module and the first task; execute it, and every task it
    /// returns, then call [`PendingModule::finish`].
    pub fn with_tasks(ctx: &Arc<DeviceContext>, module: &ModuleOptions<'_>, pipeline: &PipelineOptions, code: &[u8]) -> Result<(PendingModule, Task)> {
        let opts = CompileOpts::new(ctx, module, pipeline)?;
        let code: Box<[u8]> = code.into();
        let log = LogBuf::new();
        let mut raw = std::ptr::null_mut();
        let mut first = std::ptr::null_mut();
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `opts`, `code` and `log` own heap storage that ends up in `Shared`, which lives
        // until the module is destroyed; nothing else references the log yet.
        let r = unsafe {
            entry!(ctx.table(), optixModuleCreateWithTasks)(
                ctx.raw(),
                &opts.mco,
                &opts.pco,
                code.as_ptr().cast(),
                code.len(),
                log.ptr.cast(),
                log.len,
                &mut raw,
                &mut first,
            )
        };
        if r != sys::OptixResult_OPTIX_SUCCESS {
            // SAFETY: the create call returned; no task exists, so nothing writes the log.
            let text = unsafe { log.text() };
            return Err(Error::Optix { call: "optixModuleCreateWithTasks", code: r, log: text });
        }
        let module = Module::wrap(raw, ctx, opts.base.clone());
        let shared = Arc::new(Shared { module, gate: RwLock::new(()), failure: AtomicI32::new(sys::OptixResult_OPTIX_SUCCESS), log, _code: code, _opts: opts });
        let task = Task { raw: first, state: TaskState::Pending, shared: shared.clone() };
        Ok((PendingModule { shared }, task))
    }
}

impl PendingModule {
    /// The compile state (`optixModuleGetCompilationState`).
    pub fn state(&self) -> Result<CompileState> {
        let mut s = 0;
        // SAFETY: the module is alive (held by `shared`); documented thread safe.
        check("optixModuleGetCompilationState", unsafe {
            entry!(self.shared.ctx().table(), optixModuleGetCompilationState)(self.shared.module.raw(), &mut s)
        })?;
        Ok(match s {
            sys::OptixModuleCompileState_OPTIX_MODULE_COMPILE_STATE_NOT_STARTED => CompileState::NotStarted,
            sys::OptixModuleCompileState_OPTIX_MODULE_COMPILE_STATE_STARTED => CompileState::Started,
            sys::OptixModuleCompileState_OPTIX_MODULE_COMPILE_STATE_IMPENDING_FAILURE => CompileState::ImpendingFailure,
            sys::OptixModuleCompileState_OPTIX_MODULE_COMPILE_STATE_FAILED => CompileState::Failed,
            sys::OptixModuleCompileState_OPTIX_MODULE_COMPILE_STATE_COMPLETED => CompileState::Completed,
            _ => return Err(Error::Invalid("OptiX reported an unknown module compile state")),
        })
    }

    /// Cancels the compile (`optixModuleCancelCreation`): remaining tasks return
    /// `OPTIX_ERROR_CREATION_CANCELED` without work; `block` waits until that is in effect.
    /// Safe from any thread.
    pub fn cancel(&self, block: bool) -> Result<()> {
        let _running = self.shared.gate.read().map_err(|_| Error::Invalid("module task gate poisoned by a panic"))?;
        self.shared.fail(sys::OptixResult_OPTIX_ERROR_CREATION_CANCELED);
        // SAFETY: the module is alive; documented thread safe.
        check("optixModuleCancelCreation", unsafe {
            entry!(self.shared.ctx().table(), optixModuleCancelCreation)(self.shared.module.raw(), creation_flags(block))
        })
    }

    /// The compiler log so far; waits for running tasks to return first.
    pub fn log(&self) -> Result<String> {
        let _exclusive = self.shared.gate.write().map_err(|_| Error::Invalid("module task gate poisoned by a panic"))?;
        // SAFETY: the write side excludes every task call, so OptiX is not writing the log.
        Ok(unsafe { self.shared.log.text() })
    }

    /// The module and its compiler log once the state is [`CompileState::Completed`].
    /// A failed or cancelled compile (also [`CompileState::ImpendingFailure`]) is
    /// [`Error::Optix`] with the first failing task's code (`OPTIX_ERROR_CREATION_CANCELED` after
    /// a cancel, including [`DeviceContext::cancel_creations`]) and the log; a compile with tasks
    /// still to run is [`Error::Invalid`].
    pub fn finish(&self) -> Result<(Arc<Module>, String)> {
        match self.state()? {
            CompileState::Completed => Ok((self.shared.module.clone(), self.log()?)),
            CompileState::Failed | CompileState::ImpendingFailure => {
                let code = match self.shared.failure.load(Ordering::Acquire) {
                    // No task reported an error, so the context-wide cancel stopped it.
                    sys::OptixResult_OPTIX_SUCCESS => sys::OptixResult_OPTIX_ERROR_CREATION_CANCELED,
                    c => c,
                };
                Err(Error::Optix { call: "optixModuleCreateWithTasks", code, log: self.log()? })
            }
            CompileState::NotStarted | CompileState::Started => Err(Error::Invalid("module compile has tasks still to run")),
        }
    }
}

impl Task {
    fn require(&self, want: TaskState, msg: &'static str) -> Result<()> {
        if self.state == want { Ok(()) } else { Err(Error::Invalid(msg)) }
    }

    /// Wraps the follow-up tasks OptiX wrote, or records the failure.
    fn follow_ups(&self, call: &'static str, code: sys::OptixResult, mut new: Vec<sys::OptixTask>, n: u32) -> Result<Vec<Task>> {
        if code != sys::OptixResult_OPTIX_SUCCESS {
            // Follow-ups of a failed task are left to `optixModuleDestroy`; OptiX does not
            // require running them once the compile is failing.
            self.shared.fail(code);
            // The per-task log is the module log; `PendingModule::finish` reports it.
            return Err(Error::Optix { call, code, log: String::new() });
        }
        new.truncate(n as usize);
        Ok(new.into_iter().map(|raw| Task { raw, state: TaskState::Pending, shared: self.shared.clone() }).collect())
    }

    /// Runs the task (`optixTaskExecute`); returns up to `max_new` follow-up tasks, each to be
    /// executed (or deserialized) on any thread. A task runs once.
    pub fn execute(&mut self, max_new: u32) -> Result<Vec<Task>> {
        self.require(TaskState::Pending, "task already executed or deserialized")?;
        let mut new = vec![std::ptr::null_mut(); max_new as usize];
        let mut n = 0;
        let code = {
            let _running = self.shared.gate.read().map_err(|_| Error::Invalid("module task gate poisoned by a panic"))?;
            self.shared.ctx().cuda().bind_to_thread()?;
            // SAFETY: the module (and the code/log OptiX uses) live in `shared`; `new` holds
            // `max_new` writable slots; the gate's read side keeps the host off the log.
            unsafe { entry!(self.shared.ctx().table(), optixTaskExecute)(self.raw, new.as_mut_ptr(), max_new, &mut n) }
        };
        self.state = TaskState::Executed;
        self.follow_ups("optixTaskExecute", code, new, n)
    }

    /// The task's cache key (`optixTaskGetSerializationKey`); empty when the task cannot be
    /// serialized and must be executed.
    pub fn key(&self) -> Result<Vec<u8>> {
        let f = entry!(self.shared.ctx().table(), optixTaskGetSerializationKey);
        self.two_call("optixTaskGetSerializationKey", |p, s| {
            // SAFETY: live task; `p` is null or `*s` writable bytes.
            unsafe { f(self.raw, p, s) }
        })
    }

    /// The executed task's output for an external cache (`optixTaskSerializeOutput`); empty for
    /// a non-serializable task. Only after [`Task::execute`], not after [`Task::deserialize`].
    pub fn serialize(&self) -> Result<Vec<u8>> {
        self.require(TaskState::Executed, "only an executed task can be serialized")?;
        let f = entry!(self.shared.ctx().table(), optixTaskSerializeOutput);
        self.two_call("optixTaskSerializeOutput", |p, s| {
            // SAFETY: live, executed task; `p` is null or `*s` writable bytes.
            unsafe { f(self.raw, p, s) }
        })
    }

    /// Size query then fill, as the serialization calls expect.
    fn two_call(&self, call: &'static str, f: impl Fn(*mut std::ffi::c_void, *mut usize) -> sys::OptixResult) -> Result<Vec<u8>> {
        let _running = self.shared.gate.read().map_err(|_| Error::Invalid("module task gate poisoned by a panic"))?;
        let mut size = 0usize;
        check(call, f(std::ptr::null_mut(), &mut size))?;
        let mut out = vec![0u8; size];
        if size == 0 {
            return Ok(out);
        }
        let mut written = size;
        check(call, f(out.as_mut_ptr().cast(), &mut written))?;
        out.truncate(written);
        Ok(out)
    }

    /// Replays output stored by [`Task::serialize`] under this task's [`Task::key`] instead of
    /// executing (`optixTaskDeserializeOutput`); returns up to `max_new` follow-up tasks.
    pub fn deserialize(&mut self, data: &[u8], max_new: u32) -> Result<Vec<Task>> {
        self.require(TaskState::Pending, "task already executed or deserialized")?;
        let mut new = vec![std::ptr::null_mut(); max_new as usize];
        let mut n = 0;
        let code = {
            let _running = self.shared.gate.read().map_err(|_| Error::Invalid("module task gate poisoned by a panic"))?;
            self.shared.ctx().cuda().bind_to_thread()?;
            // SAFETY: as in `execute`; `data` is readable for its length during the call.
            unsafe {
                entry!(self.shared.ctx().table(), optixTaskDeserializeOutput)(self.raw, data.as_ptr().cast(), data.len(), new.as_mut_ptr(), max_new, &mut n)
            }
        };
        self.state = TaskState::Deserialized;
        self.follow_ups("optixTaskDeserializeOutput", code, new, n)
    }
}
