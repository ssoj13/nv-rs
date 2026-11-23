//! CUPTI Activity API with a reusable 8-byte-aligned buffer pool and `catch_unwind` callbacks.
//!
//! **What.**
//! - [`AlignedBufferPool`] — manages fixed-size 8-byte-aligned buffers (`Box<[u64]>`) so CUPTI
//!   activity callbacks reuse existing allocations with zero steady-state hot-path churn.
//! - [`ActivityProfiler`] — registers `cuptiActivityRegisterCallbacks`, enables/disables
//!   [`ActivityKind`]s, flushes records (`cuptiActivityFlushAll`), and drains typed
//!   [`ActivityRecord`]s.
//!
//! **Process-global state.** `cuptiActivityEnable` acts on the whole process, and CUPTI delivers
//! every record through one pair of buffer callbacks. Profilers therefore share one record queue
//! (records of all contexts, drained by whichever profiler asks first) and reference-count each
//! [`ActivityKind`]: CUPTI is asked to disable a kind only when the last profiler that enabled it
//! disables it or drops.
//!
//! **Decoding.** Kernel records are read field by field through raw pointers, never as a reference
//! to the whole `CUpti_ActivityKernelLatest`, because an older loaded CUPTI emits a smaller record
//! version. The fields read have the same offsets in every version (asserted at compile time).
//!
//! **Teardown safety.** `Drop` releases every [`ActivityKind`] this profiler enabled, forces a
//! flush, and records any failure on the owning `cuda_core::Context::record_err`. When the last
//! live profiler drops, undrained records are discarded so the next session starts empty.

use crate::{Error, Result, check, ensure_loaded, panic_payload_to_string, sys};
use cuda_core::{Context, DriverError};
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::{Arc, Mutex, OnceLock};

/// Activity record kinds (`CUpti_ActivityKind`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ActivityKind {
    /// Memory copies (`CUPTI_ACTIVITY_KIND_MEMCPY`).
    Memcpy,
    /// Memory sets (`CUPTI_ACTIVITY_KIND_MEMSET`).
    Memset,
    /// Serialized kernel execution (`CUPTI_ACTIVITY_KIND_KERNEL`).
    Kernel,
    /// Concurrent kernel execution (`CUPTI_ACTIVITY_KIND_CONCURRENT_KERNEL`).
    ConcurrentKernel,
    /// CUDA Driver API calls (`CUPTI_ACTIVITY_KIND_DRIVER`).
    Driver,
    /// CUDA Runtime API calls (`CUPTI_ACTIVITY_KIND_RUNTIME`).
    Runtime,
}

impl ActivityKind {
    pub(crate) fn to_sys(self) -> sys::CUpti_ActivityKind {
        match self {
            Self::Memcpy => sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_MEMCPY,
            Self::Memset => sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_MEMSET,
            Self::Kernel => sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_KERNEL,
            Self::ConcurrentKernel => sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_CONCURRENT_KERNEL,
            Self::Driver => sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_DRIVER,
            Self::Runtime => sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_RUNTIME,
        }
    }
}

/// Decoded CUPTI activity record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActivityRecord {
    /// GPU kernel execution (`CUpti_ActivityKernel12`).
    Kernel {
        /// Kernel name.
        name: String,
        /// Start timestamp (ns).
        start_ns: u64,
        /// End timestamp (ns).
        end_ns: u64,
        /// Device ID.
        device_id: u32,
        /// Context ID.
        context_id: u32,
        /// Stream ID.
        stream_id: u32,
        /// Correlation ID matching API record.
        correlation_id: u32,
    },
    /// GPU memory copy (`CUpti_ActivityMemcpy6`).
    Memcpy {
        /// Copy kind (`CUpti_ActivityMemcpyKind`).
        copy_kind: u8,
        /// Transferred byte count.
        bytes: u64,
        /// Start timestamp (ns).
        start_ns: u64,
        /// End timestamp (ns).
        end_ns: u64,
        /// Device ID.
        device_id: u32,
        /// Context ID.
        context_id: u32,
        /// Stream ID.
        stream_id: u32,
    },
    /// Driver/Runtime API call (`CUpti_ActivityAPI`).
    Api {
        /// Callback ID.
        cbid: u32,
        /// Thread ID.
        thread_id: u32,
        /// Correlation ID.
        correlation_id: u32,
        /// Return value.
        return_value: u32,
    },
    /// Any other activity kind not explicitly decoded.
    Other {
        /// Raw `CUpti_ActivityKind`.
        kind: u32,
    },
}

/// Reusable pool of 8-byte-aligned host buffers for CUPTI activity collection.
pub struct AlignedBufferPool {
    buf_bytes: usize,
    max_pooled: usize,
    free: Vec<Box<[u64]>>,
    in_flight: HashMap<usize, Box<[u64]>>,
}

impl AlignedBufferPool {
    /// Creates a pool whose buffers are `buf_bytes` bytes (rounded up to a multiple of 8 bytes,
    /// minimum 4096 bytes) and retains up to `max_pooled` free buffers.
    pub fn new(buf_bytes: usize, max_pooled: usize) -> Self {
        let aligned_bytes = buf_bytes.max(4096).div_ceil(8) * 8;
        Self {
            buf_bytes: aligned_bytes,
            max_pooled,
            free: Vec::new(),
            in_flight: HashMap::new(),
        }
    }

    /// Acquires an 8-byte-aligned buffer pointer and byte capacity, reusing a pooled buffer if
    /// available.
    pub fn acquire(&mut self) -> (*mut u8, usize) {
        let mut buf = self
            .free
            .pop()
            .unwrap_or_else(|| vec![0u64; self.buf_bytes / 8].into_boxed_slice());
        let ptr = buf.as_mut_ptr().cast::<u8>();
        let len = buf.len() * 8;
        self.in_flight.insert(ptr as usize, buf);
        (ptr, len)
    }

    /// Returns a buffer to the pool (or drops it if the pool already holds `max_pooled` buffers).
    /// Returns `true` if the pointer was tracked as in-flight.
    pub fn release(&mut self, ptr: *mut u8) -> bool {
        let Some(buf) = self.in_flight.remove(&(ptr as usize)) else {
            return false;
        };
        if self.free.len() < self.max_pooled {
            self.free.push(buf);
        }
        true
    }

    /// Number of currently idle buffers cached in the pool.
    pub fn free_count(&self) -> usize {
        self.free.len()
    }

    /// Number of buffers currently checked out to CUPTI.
    pub fn in_flight_count(&self) -> usize {
        self.in_flight.len()
    }
}

struct ActivityState {
    pool: AlignedBufferPool,
    records: Vec<ActivityRecord>,
    scratch: Vec<ActivityRecord>,
    dropped: usize,
    panic_msg: Option<String>,
    registered: bool,
}

static ACTIVITY_STATE: OnceLock<Mutex<ActivityState>> = OnceLock::new();

fn state() -> &'static Mutex<ActivityState> {
    ACTIVITY_STATE.get_or_init(|| {
        Mutex::new(ActivityState {
            pool: AlignedBufferPool::new(256 * 1024, 16),
            records: Vec::with_capacity(256),
            scratch: Vec::with_capacity(256),
            dropped: 0,
            panic_msg: None,
            registered: false,
        })
    })
}

/// Process-wide bookkeeping behind `cuptiActivityEnable` / `cuptiActivityDisable`, which act on the
/// whole process rather than on one profiler.
///
/// Kept apart from [`ACTIVITY_STATE`]: CUPTI may run the buffer callbacks (which lock that state)
/// from inside the enable/disable/flush calls made while this lock is held. Lock order is always
/// `KIND_REFS` before `ACTIVITY_STATE`.
struct KindRefs {
    /// Live [`ActivityProfiler`] instances.
    live: usize,
    /// One entry per (profiler, kind) enablement; a kind is enabled in CUPTI while it appears here.
    enabled: Vec<ActivityKind>,
}

impl KindRefs {
    /// Records one more enablement of `kind`. Returns `true` when it is the first, so CUPTI must
    /// be asked to enable the kind.
    fn acquire(&mut self, kind: ActivityKind) -> bool {
        let first = !self.enabled.contains(&kind);
        self.enabled.push(kind);
        first
    }

    /// Removes one enablement of `kind`. Returns `true` when it was the last, so CUPTI must be
    /// asked to disable the kind; releasing a kind that is not held is a no-op returning `false`.
    fn release(&mut self, kind: ActivityKind) -> bool {
        match self.enabled.iter().position(|&k| k == kind) {
            Some(pos) => {
                self.enabled.swap_remove(pos);
                !self.enabled.contains(&kind)
            }
            None => false,
        }
    }
}

static KIND_REFS: Mutex<KindRefs> = Mutex::new(KindRefs {
    live: 0,
    enabled: Vec::new(),
});

fn kind_refs() -> std::sync::MutexGuard<'static, KindRefs> {
    KIND_REFS.lock().unwrap_or_else(|e| e.into_inner())
}

/// `CUPTI_ACTIVITY_FLAG_FLUSH_FORCED` as the `u32` taken by `cuptiActivityFlushAll`. The enum is
/// `c_int` with MSVC and `c_uint` with GCC, so a byte copy converts it without a cast that would
/// be redundant on Linux.
const FLUSH_FORCED: u32 =
    u32::from_ne_bytes(sys::CUpti_ActivityFlag_CUPTI_ACTIVITY_FLAG_FLUSH_FORCED.to_ne_bytes());

// The decoder reads these fields of `CUpti_ActivityKernelLatest` from records that an older loaded
// CUPTI may have emitted as a smaller version. `CUpti_ActivityKernel3` is the oldest layout still
// bound; every field read must sit at the same offset and inside that smallest record.
const _: () = {
    use std::mem::offset_of;
    assert!(
        offset_of!(sys::CUpti_ActivityKernelLatest, start)
            == offset_of!(sys::CUpti_ActivityKernel3, start)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityKernelLatest, end)
            == offset_of!(sys::CUpti_ActivityKernel3, end)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityKernelLatest, deviceId)
            == offset_of!(sys::CUpti_ActivityKernel3, deviceId)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityKernelLatest, contextId)
            == offset_of!(sys::CUpti_ActivityKernel3, contextId)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityKernelLatest, streamId)
            == offset_of!(sys::CUpti_ActivityKernel3, streamId)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityKernelLatest, correlationId)
            == offset_of!(sys::CUpti_ActivityKernel3, correlationId)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityKernelLatest, name)
            == offset_of!(sys::CUpti_ActivityKernel3, name)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityKernel3, name) + size_of::<*const std::ffi::c_char>()
            <= size_of::<sys::CUpti_ActivityKernel3>()
    );
    // Same rule for memcpy records: `CUpti_ActivityMemcpy` is the oldest (smallest) bound version.
    assert!(
        offset_of!(sys::CUpti_ActivityMemcpy6, copyKind)
            == offset_of!(sys::CUpti_ActivityMemcpy, copyKind)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityMemcpy6, bytes)
            == offset_of!(sys::CUpti_ActivityMemcpy, bytes)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityMemcpy6, start)
            == offset_of!(sys::CUpti_ActivityMemcpy, start)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityMemcpy6, end) == offset_of!(sys::CUpti_ActivityMemcpy, end)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityMemcpy6, deviceId)
            == offset_of!(sys::CUpti_ActivityMemcpy, deviceId)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityMemcpy6, contextId)
            == offset_of!(sys::CUpti_ActivityMemcpy, contextId)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityMemcpy6, streamId)
            == offset_of!(sys::CUpti_ActivityMemcpy, streamId)
    );
    assert!(
        offset_of!(sys::CUpti_ActivityMemcpy, streamId) + size_of::<u32>()
            <= size_of::<sys::CUpti_ActivityMemcpy>()
    );
};

unsafe extern "C" fn buffer_requested(
    buffer: *mut *mut u8,
    size: *mut usize,
    max_num_records: *mut usize,
) {
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if buffer.is_null() || size.is_null() || max_num_records.is_null() {
            return;
        }
        let mut st = state().lock().unwrap_or_else(|e| e.into_inner());
        let (ptr, bytes) = st.pool.acquire();
        // SAFETY: CUPTI guarantees `buffer`, `size`, and `max_num_records` are valid out-pointers.
        unsafe {
            *buffer = ptr;
            *size = bytes;
            *max_num_records = 0;
        }
    }));
    if let Err(payload) = res {
        let mut st = state().lock().unwrap_or_else(|e| e.into_inner());
        st.panic_msg
            .get_or_insert_with(|| panic_payload_to_string(payload));
    }
}

unsafe extern "C" fn buffer_completed(
    ctx: sys::CUcontext,
    stream_id: u32,
    buffer: *mut u8,
    _size: usize,
    valid_size: usize,
) {
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut parsed = {
            let mut st = state().lock().unwrap_or_else(|e| e.into_inner());
            let mut v = std::mem::take(&mut st.scratch);
            v.clear();
            v
        };
        if !buffer.is_null() && valid_size > 0 {
            let mut record_ptr: *mut sys::CUpti_Activity = std::ptr::null_mut();
            loop {
                // SAFETY: `buffer` is valid for `valid_size` bytes until this callback returns.
                let rc =
                    unsafe { sys::cuptiActivityGetNextRecord(buffer, valid_size, &mut record_ptr) };
                if rc == sys::CUptiResult_CUPTI_SUCCESS && !record_ptr.is_null() {
                    // SAFETY: `record_ptr` points to a valid `CUpti_Activity` header.
                    let kind = unsafe { (*record_ptr).kind };
                    parsed.push(decode_record(record_ptr, kind));
                } else {
                    break;
                }
            }
        }
        let mut dropped = 0usize;
        // SAFETY: `dropped` is a valid out-pointer; `ctx` may be null for global records.
        let _ = unsafe { sys::cuptiActivityGetNumDroppedRecords(ctx, stream_id, &mut dropped) };

        let mut st = state().lock().unwrap_or_else(|e| e.into_inner());
        st.records.append(&mut parsed);
        st.scratch = parsed;
        st.dropped = st.dropped.saturating_add(dropped);
        if !buffer.is_null() {
            st.pool.release(buffer);
        }
    }));
    if let Err(payload) = res {
        let mut st = state().lock().unwrap_or_else(|e| e.into_inner());
        st.panic_msg
            .get_or_insert_with(|| panic_payload_to_string(payload));
        if !buffer.is_null() {
            st.pool.release(buffer);
        }
    }
}

fn decode_record(
    record_ptr: *mut sys::CUpti_Activity,
    kind: sys::CUpti_ActivityKind,
) -> ActivityRecord {
    match kind {
        sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_KERNEL
        | sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_CONCURRENT_KERNEL => {
            let k = record_ptr.cast::<sys::CUpti_ActivityKernelLatest>();
            // SAFETY: `kind` says the record is some `CUpti_ActivityKernelN`, possibly an older and
            // smaller version than `CUpti_ActivityKernelLatest`. No reference to the whole struct
            // is formed; each field is read through a raw place whose offset is identical in every
            // bound version and lies inside the smallest one (compile-time asserts above), so all
            // reads stay inside the record CUPTI wrote. `read_unaligned` makes no alignment claim.
            let (start, end, device_id, context_id, stream_id, correlation_id, name_ptr) = unsafe {
                (
                    (&raw const (*k).start).read_unaligned(),
                    (&raw const (*k).end).read_unaligned(),
                    (&raw const (*k).deviceId).read_unaligned(),
                    (&raw const (*k).contextId).read_unaligned(),
                    (&raw const (*k).streamId).read_unaligned(),
                    (&raw const (*k).correlationId).read_unaligned(),
                    (&raw const (*k).name).read_unaligned(),
                )
            };
            let name = if name_ptr.is_null() {
                String::new()
            } else {
                // SAFETY: `name_ptr` is a non-null NUL-terminated static/interned string from CUPTI.
                unsafe { CStr::from_ptr(name_ptr) }
                    .to_string_lossy()
                    .into_owned()
            };
            ActivityRecord::Kernel {
                name,
                start_ns: start,
                end_ns: end,
                device_id,
                context_id,
                stream_id,
                correlation_id,
            }
        }
        sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_MEMCPY => {
            let m = record_ptr.cast::<sys::CUpti_ActivityMemcpy6>();
            // SAFETY: as for kernels: the record is some `CUpti_ActivityMemcpyN`, possibly smaller
            // than `CUpti_ActivityMemcpy6`; only fields with version-independent offsets inside the
            // smallest version are read through raw places (compile-time asserts above).
            unsafe {
                ActivityRecord::Memcpy {
                    copy_kind: (&raw const (*m).copyKind).read_unaligned(),
                    bytes: (&raw const (*m).bytes).read_unaligned(),
                    start_ns: (&raw const (*m).start).read_unaligned(),
                    end_ns: (&raw const (*m).end).read_unaligned(),
                    device_id: (&raw const (*m).deviceId).read_unaligned(),
                    context_id: (&raw const (*m).contextId).read_unaligned(),
                    stream_id: (&raw const (*m).streamId).read_unaligned(),
                }
            }
        }
        sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_DRIVER
        | sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_RUNTIME => {
            // SAFETY: `kind` guarantees the record is `CUpti_ActivityAPI`.
            let a = unsafe { &*record_ptr.cast::<sys::CUpti_ActivityAPI>() };
            ActivityRecord::Api {
                cbid: a.cbid,
                thread_id: a.threadId,
                correlation_id: a.correlationId,
                return_value: a.returnValue,
            }
        }
        // Byte copy instead of `as`: the enum is `c_int` with MSVC and `c_uint` with GCC.
        other => ActivityRecord::Other {
            kind: u32::from_ne_bytes(other.to_ne_bytes()),
        },
    }
}

/// Manages CUPTI activity collection on behalf of a `cuda_core::Context`.
///
/// CUPTI activity collection is process-global: an enabled kind produces records for every
/// context, and all profilers share one record queue, so [`Self::drain_records`] returns records
/// of all contexts and of other live profilers. Enabled kinds are reference-counted across
/// profilers; dropping one profiler never disables a kind another profiler still uses.
pub struct ActivityProfiler {
    cuda: Arc<Context>,
    enabled: Vec<ActivityKind>,
}

impl ActivityProfiler {
    /// Registers the global CUPTI activity buffer callbacks (idempotent across instances) and
    /// returns an [`ActivityProfiler`] whose CUPTI calls bind `cuda` first.
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        {
            let mut st = state().lock().unwrap_or_else(|e| e.into_inner());
            if !st.registered {
                // SAFETY: `buffer_requested` and `buffer_completed` are `extern "C"` trampolines
                // guarded by `catch_unwind`.
                check("cuptiActivityRegisterCallbacks", unsafe {
                    sys::cuptiActivityRegisterCallbacks(
                        Some(buffer_requested),
                        Some(buffer_completed),
                    )
                })?;
                st.registered = true;
            }
        }
        // Counted only once construction cannot fail, so every increment has a matching `Drop`.
        kind_refs().live += 1;
        Ok(Self {
            cuda: cuda.clone(),
            enabled: Vec::new(),
        })
    }

    /// Enables collection for `kind`. CUPTI (`cuptiActivityEnable`) is called only when no other
    /// live profiler has the kind enabled; enabling a kind twice on one profiler is a no-op.
    pub fn enable(&mut self, kind: ActivityKind) -> Result<()> {
        if self.enabled.contains(&kind) {
            return Ok(());
        }
        self.cuda.bind_to_thread()?;
        let mut refs = kind_refs();
        if refs.acquire(kind) {
            // SAFETY: `kind.to_sys()` is a valid `CUpti_ActivityKind`.
            let rc = unsafe { sys::cuptiActivityEnable(kind.to_sys()) };
            if let Err(err) = check("cuptiActivityEnable", rc) {
                // Undo the reference taken above; CUPTI did not enable the kind.
                refs.release(kind);
                return Err(err);
            }
        }
        self.enabled.push(kind);
        Ok(())
    }

    /// Disables collection for `kind` on this profiler. CUPTI (`cuptiActivityDisable`) is called
    /// only when this was the last live profiler with the kind enabled; disabling a kind this
    /// profiler did not enable is a no-op.
    pub fn disable(&mut self, kind: ActivityKind) -> Result<()> {
        let Some(pos) = self.enabled.iter().position(|&k| k == kind) else {
            return Ok(());
        };
        self.cuda.bind_to_thread()?;
        let mut refs = kind_refs();
        if refs.release(kind) {
            // SAFETY: `kind.to_sys()` is a valid `CUpti_ActivityKind`.
            let rc = unsafe { sys::cuptiActivityDisable(kind.to_sys()) };
            if let Err(err) = check("cuptiActivityDisable", rc) {
                // CUPTI still collects the kind, so this profiler keeps its reference.
                refs.acquire(kind);
                return Err(err);
            }
        }
        self.enabled.swap_remove(pos);
        Ok(())
    }

    /// Flushes all pending activity records (`cuptiActivityFlushAll`), surfacing any callback panic.
    pub fn flush(&self, forced: bool) -> Result<()> {
        self.cuda.bind_to_thread()?;
        let flag = if forced { FLUSH_FORCED } else { 0 };
        // SAFETY: `flag` is 0 or `CUPTI_ACTIVITY_FLAG_FLUSH_FORCED`.
        check("cuptiActivityFlushAll", unsafe {
            sys::cuptiActivityFlushAll(flag)
        })?;
        self.check_callback_panic()
    }

    /// Drains all collected [`ActivityRecord`]s into `out` (reusing `out`'s capacity) and returns
    /// the dropped record count. The queue is process-global: it holds records of every context
    /// and is shared with any other live profiler.
    pub fn drain_records_into(&self, out: &mut Vec<ActivityRecord>) -> Result<usize> {
        self.check_callback_panic()?;
        let mut st = state().lock().unwrap_or_else(|e| e.into_inner());
        out.append(&mut st.records);
        Ok(std::mem::take(&mut st.dropped))
    }

    /// Drains all collected [`ActivityRecord`]s and returns `(records, dropped_count)`.
    pub fn drain_records(&self) -> Result<(Vec<ActivityRecord>, usize)> {
        let mut records = Vec::new();
        let dropped = self.drain_records_into(&mut records)?;
        Ok((records, dropped))
    }

    fn check_callback_panic(&self) -> Result<()> {
        let mut st = state().lock().unwrap_or_else(|e| e.into_inner());
        if let Some(msg) = st.panic_msg.take() {
            return Err(Error::CallbackPanicked(msg));
        }
        Ok(())
    }
}

impl Drop for ActivityProfiler {
    fn drop(&mut self) {
        let _ = self.cuda.bind_to_thread();
        let mut refs = kind_refs();
        for kind in self.enabled.drain(..) {
            if !refs.release(kind) {
                // Another live profiler still collects this kind.
                continue;
            }
            // SAFETY: `kind` was enabled through this profiler and no other profiler holds it.
            let rc = unsafe { sys::cuptiActivityDisable(kind.to_sys()) };
            if rc != sys::CUptiResult_CUPTI_SUCCESS {
                self.cuda.record_err(Err::<(), _>(DriverError(
                    cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
                )));
            }
        }
        // SAFETY: forced flush on teardown ensures CUPTI returns all in-flight buffers to the pool.
        let rc = unsafe { sys::cuptiActivityFlushAll(FLUSH_FORCED) };
        if rc != sys::CUptiResult_CUPTI_SUCCESS {
            self.cuda.record_err(Err::<(), _>(DriverError(
                cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
            )));
        }
        refs.live = refs.live.saturating_sub(1);
        if refs.live == 0 {
            // No profiler is left to drain what the flush above delivered; discard it so the next
            // profiler does not report records and drop counts of a finished session.
            let mut st = state().lock().unwrap_or_else(|e| e.into_inner());
            st.records.clear();
            st.dropped = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aligned_buffer_pool_reuses_8_byte_aligned_buffers() {
        let mut pool = AlignedBufferPool::new(4096, 2);
        let (p1, len1) = pool.acquire();
        assert_eq!(len1, 4096);
        assert_eq!((p1 as usize) % 8, 0);
        assert_eq!(pool.in_flight_count(), 1);
        assert!(pool.release(p1));
        assert_eq!(pool.in_flight_count(), 0);
        assert_eq!(pool.free_count(), 1);

        // Re-acquiring should return the exact same pooled buffer pointer.
        let (p2, len2) = pool.acquire();
        assert_eq!(p2, p1);
        assert_eq!(len2, 4096);
        assert_eq!(pool.free_count(), 0);
        assert!(pool.release(p2));
        assert!(!pool.release(p2), "double release must return false");
    }

    #[test]
    fn kind_refs_enable_once_and_disable_only_on_last_release() {
        // CUPTI-1: two profilers enabling the same kind must not let the first drop disable it.
        let mut refs = KindRefs {
            live: 0,
            enabled: Vec::new(),
        };
        assert!(refs.acquire(ActivityKind::Memcpy), "first enable reaches CUPTI");
        assert!(!refs.acquire(ActivityKind::Memcpy), "second enable is shared");
        assert!(refs.acquire(ActivityKind::Kernel));
        assert!(
            !refs.release(ActivityKind::Memcpy),
            "another profiler still holds Memcpy"
        );
        assert!(refs.release(ActivityKind::Memcpy), "last holder disables");
        assert!(
            !refs.release(ActivityKind::Memcpy),
            "release without a holder is a no-op"
        );
        assert!(refs.release(ActivityKind::Kernel));
        assert!(refs.enabled.is_empty());
    }

    #[test]
    fn decode_reads_an_older_smaller_kernel_record_in_bounds() {
        // CUPTI-3: an older CUPTI emits `CUpti_ActivityKernel3` (120 bytes), smaller than
        // `CUpti_ActivityKernelLatest`. Decoding must read only that record's bytes (Miri flags a
        // reference to the larger struct here).
        // SAFETY: the record is plain C data (integers, an integer union, raw pointers), for which
        // all-zero bytes are a valid value.
        let mut rec: sys::CUpti_ActivityKernel3 = unsafe { std::mem::zeroed() };
        rec.kind = sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_KERNEL;
        rec.start = 10;
        rec.end = 25;
        rec.deviceId = 1;
        rec.contextId = 2;
        rec.streamId = 3;
        rec.correlationId = 4;
        let kind = rec.kind;
        let decoded = decode_record((&raw mut rec).cast::<sys::CUpti_Activity>(), kind);
        assert_eq!(
            decoded,
            ActivityRecord::Kernel {
                name: String::new(),
                start_ns: 10,
                end_ns: 25,
                device_id: 1,
                context_id: 2,
                stream_id: 3,
                correlation_id: 4,
            }
        );
    }
}
