//! CUPTI Activity API with a reusable 8-byte-aligned buffer pool and `catch_unwind` callbacks.
//!
//! **What.**
//! - [`AlignedBufferPool`] — manages fixed-size 8-byte-aligned buffers (`Box<[u64]>`) so CUPTI
//!   activity callbacks reuse existing allocations with zero steady-state hot-path churn.
//! - [`ActivityProfiler`] — registers `cuptiActivityRegisterCallbacks`, enables/disables
//!   [`ActivityKind`]s, flushes records (`cuptiActivityFlushAll`), and drains typed
//!   [`ActivityRecord`]s.
//!
//! **Teardown safety.** `Drop` disables every enabled [`ActivityKind`], forces a flush, and
//! records any failure on the owning `cuda_core::Context::record_err`.

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
        Self { buf_bytes: aligned_bytes, max_pooled, free: Vec::new(), in_flight: HashMap::new() }
    }

    /// Acquires an 8-byte-aligned buffer pointer and byte capacity, reusing a pooled buffer if
    /// available.
    pub fn acquire(&mut self) -> (*mut u8, usize) {
        let mut buf = self.free.pop().unwrap_or_else(|| vec![0u64; self.buf_bytes / 8].into_boxed_slice());
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

unsafe extern "C" fn buffer_requested(buffer: *mut *mut u8, size: *mut usize, max_num_records: *mut usize) {
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
        st.panic_msg.get_or_insert_with(|| panic_payload_to_string(payload));
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
                let rc = unsafe { sys::cuptiActivityGetNextRecord(buffer, valid_size, &mut record_ptr) };
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
        st.panic_msg.get_or_insert_with(|| panic_payload_to_string(payload));
        if !buffer.is_null() {
            st.pool.release(buffer);
        }
    }
}

fn decode_record(record_ptr: *mut sys::CUpti_Activity, kind: sys::CUpti_ActivityKind) -> ActivityRecord {
    match kind {
        sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_KERNEL | sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_CONCURRENT_KERNEL => {
            // SAFETY: `kind` guarantees the record is `CUpti_ActivityKernel12`.
            let k = unsafe { &*record_ptr.cast::<sys::CUpti_ActivityKernel12>() };
            let name = if k.name.is_null() {
                String::new()
            } else {
                // SAFETY: `k.name` is a NUL-terminated static/interned string from CUPTI.
                unsafe { CStr::from_ptr(k.name) }.to_string_lossy().into_owned()
            };
            ActivityRecord::Kernel {
                name,
                start_ns: k.start,
                end_ns: k.end,
                device_id: k.deviceId,
                context_id: k.contextId,
                stream_id: k.streamId,
                correlation_id: k.correlationId,
            }
        }
        sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_MEMCPY => {
            // SAFETY: `kind` guarantees the record is `CUpti_ActivityMemcpy6`.
            let m = unsafe { &*record_ptr.cast::<sys::CUpti_ActivityMemcpy6>() };
            ActivityRecord::Memcpy {
                copy_kind: m.copyKind,
                bytes: m.bytes,
                start_ns: m.start,
                end_ns: m.end,
                device_id: m.deviceId,
                context_id: m.contextId,
                stream_id: m.streamId,
            }
        }
        sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_DRIVER | sys::CUpti_ActivityKind_CUPTI_ACTIVITY_KIND_RUNTIME => {
            // SAFETY: `kind` guarantees the record is `CUpti_ActivityAPI`.
            let a = unsafe { &*record_ptr.cast::<sys::CUpti_ActivityAPI>() };
            ActivityRecord::Api {
                cbid: a.cbid,
                thread_id: a.threadId,
                correlation_id: a.correlationId,
                return_value: a.returnValue,
            }
        }
        other => ActivityRecord::Other { kind: other as u32 },
    }
}

/// Manages CUPTI activity collection for a `cuda_core::Context`.
pub struct ActivityProfiler {
    cuda: Arc<Context>,
    enabled: Vec<ActivityKind>,
}

impl ActivityProfiler {
    /// Registers the global CUPTI activity buffer callbacks (idempotent across instances) and
    /// returns an [`ActivityProfiler`] bound to `cuda`.
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut st = state().lock().unwrap_or_else(|e| e.into_inner());
        if !st.registered {
            // SAFETY: `buffer_requested` and `buffer_completed` are `extern "C"` trampolines
            // guarded by `catch_unwind`.
            check("cuptiActivityRegisterCallbacks", unsafe {
                sys::cuptiActivityRegisterCallbacks(Some(buffer_requested), Some(buffer_completed))
            })?;
            st.registered = true;
        }
        Ok(Self { cuda: cuda.clone(), enabled: Vec::new() })
    }

    /// Enables collection for `kind` (`cuptiActivityEnable`).
    pub fn enable(&mut self, kind: ActivityKind) -> Result<()> {
        self.cuda.bind_to_thread()?;
        // SAFETY: `kind.to_sys()` is a valid `CUpti_ActivityKind`.
        check("cuptiActivityEnable", unsafe { sys::cuptiActivityEnable(kind.to_sys()) })?;
        if !self.enabled.contains(&kind) {
            self.enabled.push(kind);
        }
        Ok(())
    }

    /// Disables collection for `kind` (`cuptiActivityDisable`).
    pub fn disable(&mut self, kind: ActivityKind) -> Result<()> {
        self.cuda.bind_to_thread()?;
        // SAFETY: `kind.to_sys()` is a valid `CUpti_ActivityKind`.
        check("cuptiActivityDisable", unsafe { sys::cuptiActivityDisable(kind.to_sys()) })?;
        self.enabled.retain(|&k| k != kind);
        Ok(())
    }

    /// Flushes all pending activity records (`cuptiActivityFlushAll`), surfacing any callback panic.
    pub fn flush(&self, forced: bool) -> Result<()> {
        self.cuda.bind_to_thread()?;
        let flag = if forced { sys::CUpti_ActivityFlag_CUPTI_ACTIVITY_FLAG_FLUSH_FORCED as u32 } else { 0 };
        // SAFETY: `flag` is 0 or `CUPTI_ACTIVITY_FLAG_FLUSH_FORCED`.
        check("cuptiActivityFlushAll", unsafe { sys::cuptiActivityFlushAll(flag) })?;
        self.check_callback_panic()
    }

    /// Drains all collected [`ActivityRecord`]s into `out` (reusing `out`'s capacity) and returns
    /// the dropped record count.
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
        for kind in self.enabled.drain(..) {
            // SAFETY: `kind` was previously enabled by this profiler.
            let rc = unsafe { sys::cuptiActivityDisable(kind.to_sys()) };
            if rc != sys::CUptiResult_CUPTI_SUCCESS {
                self.cuda.record_err(Err::<(), _>(DriverError(cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN)));
            }
        }
        // SAFETY: forced flush on teardown ensures CUPTI returns all in-flight buffers to the pool.
        let rc = unsafe { sys::cuptiActivityFlushAll(sys::CUpti_ActivityFlag_CUPTI_ACTIVITY_FLAG_FLUSH_FORCED as u32) };
        if rc != sys::CUptiResult_CUPTI_SUCCESS {
            self.cuda.record_err(Err::<(), _>(DriverError(cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN)));
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
}
