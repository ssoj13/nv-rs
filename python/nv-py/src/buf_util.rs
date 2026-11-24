//! Shared `PyDeviceBuffer` argument validation and completion leases for native calls.
//!
//! **What.** The entry points every binding that hands a `PyDeviceBuffer` to a native library
//! uses:
//! - [`check_buf`] validates one buffer argument (dtype, dense layout, element count, null
//!   pointer, CUDA context of the call) and returns its `(ptr, nbytes)` span;
//! - [`typed_view`] lends a checked buffer to a safe wrapper crate as a typed
//!   `cuda_core::DeviceBuffer<T>` over the same allocation (no copy; `T: AnyBits`);
//! - [`enqueue_leased`] runs the FFI enqueue with completion leases on the Python-owned buffers;
//! - [`poisoned`] is the error for a handle mutex poisoned by an earlier panic.
//! - [`open`], [`close_slot`], [`is_closed`], [`drain_stream`] and [`closed`] implement the
//!   `close()` / context-manager lifecycle of the handle classes (state kept in an
//!   `Arc<Mutex<Option<State>>>` slot that `close()` empties, so later use raises instead of
//!   touching released native objects).
//!
//! **Why.** A native library receives raw pointers, so nothing in `cuda-core` keeps the memory
//! alive or checks it: `del buf` right after an asynchronous call frees the allocation while
//! the queued work still uses it. For a stream-ordered buffer (`DeviceBuffer.alloc`) allocated on
//! another stream than the work, the next allocation then receives the same memory and is
//! corrupted (plan2 systemic defect 1, reproduced from Python by `tests/test_lease.py`). For a
//! DLPack import the producer's allocator (for example PyTorch's caching allocator) reuses the
//! block as soon as the tensor is released.
//!
//! **Lease contract** ([`enqueue_leased`]):
//! - every buffer the enqueue reads or writes is listed once per role with its [`Access`];
//! - `enqueue` queues its work on `stream` only (work on another stream is not covered) and
//!   performs no validation that could be done before it: on any error before `enqueue` runs,
//!   nothing is queued and no lease is kept;
//! - each buffer must belong to `stream`'s CUDA context, otherwise the call fails with
//!   `NvInvalidArgumentError` (`CUDA_ERROR_INVALID_CONTEXT`) before anything is queued;
//! - a write that overlaps unfinished work on another stream on the same buffer, or any access
//!   that overlaps an unfinished write there, fails with `NvInvalidArgumentError`
//!   (`CUDA_ERROR_NOT_READY`): order the streams first (for example `other.synchronize()`);
//! - the context of `stream` is bound to the calling thread before `enqueue` runs;
//! - once `enqueue` has run, every lease is kept until the work queued on `stream` completes,
//!   even when `enqueue` failed (a native call may have queued part of its work);
//! - the call may run inside `py.detach` (it neither needs nor takes the GIL); a lease released
//!   on `cuda-core`'s retirement worker may run a DLPack producer's deleter there.
//!
//! **How.** A thin adapter over `cuda_core_ext::enqueue_leased` (acquire -> bind -> enqueue ->
//! retain on `cuda_core::RawDeviceBuffer`): [`PyDeviceBuffer::storage`] supplies the allocation
//! for owned buffers and DLPack imports alike, and errors are mapped to Python exceptions.
//! **Used by** `core.rs` (`DeviceBuffer.to_bytes`) and the native bindings (`compute`,
//! `sparse_ai`, `gfx_img`, `video`, `nvjpeg`, `nvcomp`, `platform_ext`).

use std::sync::{Arc, Mutex, PoisonError, TryLockError};

use cuda_core::{Context, DeviceBuffer as CudaDeviceBuffer, DriverError, Stream};
pub(crate) use cuda_core_ext::Access;
use cufft::Complex32;
use pyo3::prelude::*;

use crate::core::PyDeviceBuffer;
use crate::errors::{ErrorKind, record_and_err};

/// Element types for which every bit pattern is a valid value, so a typed view over arbitrary
/// device bytes cannot produce an invalid `T` (`cuda_core::DeviceBuffer::from_storage` contract).
///
/// # Safety
/// Implementors must accept every bit pattern of `size_of::<Self>()` bytes.
pub(crate) unsafe trait AnyBits: Sized {}
// SAFETY: every bit pattern is a valid `u8`.
unsafe impl AnyBits for u8 {}
// SAFETY: every bit pattern is a valid `i32`.
unsafe impl AnyBits for i32 {}
// SAFETY: every bit pattern is a valid `f32` (NaN payloads included).
unsafe impl AnyBits for f32 {}
// SAFETY: `Complex32` is `#[repr(C, align(8))]` over `[f32; 2]`, valid for every bit pattern.
unsafe impl AnyBits for Complex32 {}

/// Typed view of the first `len` elements of `buf`, sharing its allocation owner and access
/// tracker (no copy), so a safe wrapper crate can be called on a Python buffer.
///
/// Call [`check_buf`] first (dtype, layout, context). Fails with `NvInvalidArgumentError` when
/// `buf` holds fewer than `len * size_of::<T>()` bytes or is not aligned for `T`.
pub(crate) fn typed_view<T: AnyBits>(
    category: &str,
    op: &str,
    name: &str,
    buf: &PyDeviceBuffer,
    len: usize,
) -> PyResult<CudaDeviceBuffer<T>> {
    let invalid = |msg: String| record_and_err(category, op, msg, ErrorKind::InvalidArgument);
    let bytes = len
        .checked_mul(size_of::<T>())
        .ok_or_else(|| invalid(format!("{name} byte size overflow")))?;
    let storage = buf.storage().slice_bytes(0, bytes).map_err(|e| {
        invalid(format!(
            "{name} holds {} bytes, need {bytes} ({e})",
            buf.byte_len()
        ))
    })?;
    // SAFETY: `T: AnyBits` accepts every bit pattern, so bytes written through any alias of this
    // storage (including uninitialized device memory) are valid `T`s; the view only lends the span
    // to one wrapper call whose accesses are covered by `enqueue_leased`.
    unsafe { CudaDeviceBuffer::<T>::from_storage(storage, len) }.map_err(|e| {
        invalid(format!(
            "{name} is not aligned to {} bytes ({e})",
            align_of::<T>()
        ))
    })
}

/// Error for a handle whose mutex was poisoned by a panic in an earlier call.
pub(crate) fn poisoned(category: &str, op: &str) -> PyErr {
    record_and_err(
        category,
        op,
        format!("{category} handle mutex poisoned"),
        ErrorKind::InvalidArgument,
    )
}

/// Error for a call on a handle, plan or manager after `close()`.
pub(crate) fn closed(category: &str, op: &str) -> PyErr {
    record_and_err(
        category,
        op,
        format!("{category} object is closed"),
        ErrorKind::InvalidArgument,
    )
}

/// Borrows the native state of a handle locked from an `Arc<Mutex<Option<State>>>` slot; fails
/// with the "closed" error once `close()` has taken it.
pub(crate) fn open<'a, T>(
    slot: &'a mut Option<T>,
    category: &str,
    op: &str,
) -> PyResult<&'a mut T> {
    slot.as_mut().ok_or_else(|| closed(category, op))
}

/// `true` when `slot` was emptied by [`close_slot`].
///
/// Never blocks: a slot locked by an in-flight call is not closed yet (closing waits for that
/// call), and a poisoned slot still reports its content.
pub(crate) fn is_closed<T>(slot: &Mutex<Option<T>>) -> bool {
    match slot.try_lock() {
        Ok(guard) => guard.is_none(),
        Err(TryLockError::Poisoned(poison)) => poison.into_inner().is_none(),
        Err(TryLockError::WouldBlock) => false,
    }
}

/// Waits until every operation queued on `stream` has finished.
pub(crate) fn drain_stream(category: &str, stream: &Stream) -> PyResult<()> {
    stream
        .synchronize()
        .map_err(|e| record_and_err(category, "close", e, ErrorKind::Cuda))
}

/// Releases the native state of a handle: waits for an in-flight call (it holds the lock), runs
/// `drain` while the state is still alive, then drops the state, all without the GIL.
///
/// Idempotent: a second call finds the slot empty and returns `Ok`. The state is dropped even
/// when `drain` fails (the error is returned afterwards), and a slot poisoned by a panic in an
/// earlier call is still released, so `close()` never strands native resources.
pub(crate) fn close_slot<T: Send>(
    py: Python<'_>,
    slot: &Arc<Mutex<Option<T>>>,
    drain: impl FnOnce(&T) -> PyResult<()> + Send,
) -> PyResult<()> {
    let slot = Arc::clone(slot);
    py.detach(move || {
        let taken = slot.lock().unwrap_or_else(PoisonError::into_inner).take();
        match taken {
            Some(state) => {
                let drained = drain(&state);
                drop(state);
                drained
            },
            None => Ok(()),
        }
    })
}

/// Validates buffer argument `name` of native call `category`/`op` and returns its dense
/// `(ptr, nbytes)` span.
///
/// - `dtypes`: accepted canonical dtype names (`"f32"`, `"i32"`, ... from `dlpack::DTYPE_TABLE`);
///   an empty slice accepts any dtype (byte-oriented APIs).
/// - `min_elems`: minimum element count, in elements of the buffer's own dtype.
/// - `ctx`: the CUDA context the call runs in (`stream.inner.context()`); the buffer's
///   allocation must belong to it, which also rejects buffers on another device.
///
/// Also rejects a non-dense layout and a null pointer on a non-empty buffer. Every failure is an
/// `NvInvalidArgumentError` recorded in `nv_rs.diag`.
pub(crate) fn check_buf(
    category: &str,
    op: &str,
    name: &str,
    buf: &PyDeviceBuffer,
    dtypes: &[&str],
    min_elems: usize,
    ctx: &Context,
) -> PyResult<(u64, usize)> {
    let invalid = |msg: String| record_and_err(category, op, msg, ErrorKind::InvalidArgument);
    let dtype = buf.dtype_name();
    if !dtypes.is_empty() && !dtypes.contains(&dtype) {
        return Err(invalid(format!(
            "{name} has dtype `{dtype}`, expected one of {dtypes:?}"
        )));
    }
    if !buf.is_c_contiguous() {
        return Err(invalid(format!("{name} is not a dense row-major buffer")));
    }
    let numel = buf.numel();
    if numel < min_elems {
        return Err(invalid(format!(
            "{name} too small: {numel} elements of `{dtype}`, need at least {min_elems}"
        )));
    }
    let (ptr, nbytes) = (buf.raw_device_ptr(), buf.byte_len());
    if ptr == 0 && nbytes > 0 {
        return Err(invalid(format!("{name} device pointer is null")));
    }
    let own = buf.storage().context();
    if own.as_ref() != ctx {
        return Err(invalid(format!(
            "{name} belongs to another CUDA context (device {}) than the call (device {})",
            own.ordinal(),
            ctx.ordinal()
        )));
    }
    Ok((ptr, nbytes))
}

/// Error of the enqueue closure or of the lease bookkeeping around it.
enum LeaseFailure {
    Driver(DriverError),
    Call(PyErr),
}

impl From<DriverError> for LeaseFailure {
    fn from(e: DriverError) -> Self {
        Self::Driver(e)
    }
}

/// Runs `enqueue` (the FFI call that queues work on `stream`) with completion leases on
/// `buffers`, so each buffer stays allocated until that work completes even if Python drops
/// it immediately. See the module docs for the full contract.
///
/// `category`/`op` name the call in errors and `nv_rs.diag`; `enqueue` returns its own
/// `PyResult` (map the native error inside it), which is passed through unchanged.
pub(crate) fn enqueue_leased<const N: usize>(
    category: &str,
    op: &str,
    stream: &Stream,
    buffers: [(&PyDeviceBuffer, Access); N],
    enqueue: impl FnOnce() -> PyResult<()>,
) -> PyResult<()> {
    let raw = buffers.map(|(buf, access)| (buf.storage(), access));
    cuda_core_ext::enqueue_leased(stream, raw, || enqueue().map_err(LeaseFailure::Call)).map_err(
        |failure| match failure {
            LeaseFailure::Call(e) => e,
            LeaseFailure::Driver(e) => lease_error(category, op, e),
        },
    )
}

/// Maps a lease bookkeeping failure to the Python exception the module docs promise.
fn lease_error(category: &str, op: &str, e: DriverError) -> PyErr {
    match e.0 {
        cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_READY => record_and_err(
            category,
            op,
            format!(
                "a buffer is still used by unfinished work on another stream ({e}); \
                 synchronize that stream first"
            ),
            ErrorKind::InvalidArgument,
        ),
        cuda_bindings::cudaError_enum_CUDA_ERROR_INVALID_CONTEXT => record_and_err(
            category,
            op,
            format!("a buffer belongs to another CUDA context than the stream ({e})"),
            ErrorKind::InvalidArgument,
        ),
        _ => record_and_err(category, op, e, ErrorKind::Cuda),
    }
}

#[cfg(test)]
mod tests {
    //! GPU tests for the lease adapter (plan2 systemic defect 1 through `PyDeviceBuffer`):
    //! `cargo test -- --ignored --test-threads=1` inside `python/nv-py`.
    //!
    //! Same reproduction as `crates/curand/tests/lease.rs`: stream A is held busy by a host
    //! function, a cuRAND generation queued behind it writes a buffer allocated stream-ordered on
    //! stream B, the buffer is dropped, and a replacement is allocated on B. Without a lease the
    //! replacement receives the same memory and the generation later writes into it.

    use super::*;
    use std::sync::Arc;
    use std::time::Duration;

    use cuda_core::DeviceBuffer as CudaDeviceBuffer;
    use curand::{Generator, RngType};

    const LEN: usize = 1 << 20;
    const REPS: usize = 10;
    const BUSY: Duration = Duration::from_millis(200);
    const RETIRE: Duration = Duration::from_millis(30);

    /// Zero-filled stream-ordered buffer on `stream`, wrapped like `DeviceBuffer.alloc` does.
    fn ordered_zeroed(stream: &Arc<Stream>) -> PyDeviceBuffer {
        // SAFETY: zero-filled on its allocation stream, which is synchronized before any use.
        let mut buf =
            unsafe { CudaDeviceBuffer::<u8>::uninitialized_async(stream, LEN * 4) }.expect("alloc");
        buf.zero_async(stream).expect("zero");
        stream.synchronize().expect("sync");
        PyDeviceBuffer::from_owned_cuda(buf, "u32", vec![LEN as i64], stream).expect("wrap")
    }

    /// Returns how many repetitions corrupted the replacement buffer.
    fn corrupted_runs(leased: bool) -> usize {
        let ctx = Context::new(0).expect("cuda context");
        let a = ctx.new_stream().expect("stream A");
        let b = ctx.new_stream().expect("stream B");
        let generator = Generator::new_device(&ctx, RngType::Philox4_32_10).expect("generator");
        generator.set_stream(&a).expect("set_stream");
        let mut corrupt = 0;
        for _ in 0..REPS {
            let x = ordered_zeroed(&b);
            a.launch_host_function(|| std::thread::sleep(BUSY))
                .expect("busy");
            let ptr = x.raw_device_ptr();
            let generate = || {
                // SAFETY: `ptr` addresses `LEN` writable `u32` of `x`, which is alive here; the
                // generator is bound to stream A.
                let code = unsafe {
                    curand::sys::curandGenerate(generator.raw(), ptr as usize as *mut u32, LEN)
                };
                let status = curand::CurandStatus(code);
                if status.is_success() {
                    Ok(())
                } else {
                    Err(record_and_err(
                        "test",
                        "curandGenerate",
                        status,
                        ErrorKind::Cuda,
                    ))
                }
            };
            if leased {
                enqueue_leased("test", "generate", &a, [(&x, Access::Write)], generate)
                    .expect("leased generate");
            } else {
                generate().expect("generate");
            }
            drop(x);
            std::thread::sleep(RETIRE);
            let y = ordered_zeroed(&b);
            a.synchronize().expect("sync A");
            let host = y
                .owned_cuda_buf()
                .expect("owned")
                .to_host_vec(&b)
                .expect("download");
            if host.iter().any(|v| *v != 0) {
                corrupt += 1;
            }
        }
        corrupt
    }

    #[test]
    #[ignore = "GPU: needs CUDA driver + curand64_10.dll"]
    fn gpu_unleased_generation_corrupts_reused_buffer() {
        // Pins the hazard the lease exists for; if this stops reproducing, the deciding test
        // below no longer proves anything.
        let corrupt = corrupted_runs(false);
        eprintln!("unleased: {corrupt}/{REPS} replacement buffers corrupted");
        assert!(corrupt > 0, "reproduction no longer corrupts");
    }

    #[test]
    #[ignore = "GPU: needs CUDA driver + curand64_10.dll"]
    fn gpu_enqueue_leased_keeps_dropped_buffer_alive() {
        let corrupt = corrupted_runs(true);
        eprintln!("leased: {corrupt}/{REPS} replacement buffers corrupted");
        assert_eq!(corrupt, 0);
    }

    #[test]
    #[ignore = "GPU: needs a CUDA driver"]
    fn gpu_enqueue_leased_rejects_conflicting_write_from_another_stream() {
        let ctx = Context::new(0).expect("cuda context");
        let a = ctx.new_stream().expect("stream A");
        let b = ctx.new_stream().expect("stream B");
        let x = ordered_zeroed(&b);
        a.launch_host_function(|| std::thread::sleep(BUSY))
            .expect("busy");
        enqueue_leased("test", "first", &a, [(&x, Access::Write)], || Ok(())).expect("first");
        let mut ran = false;
        let second = enqueue_leased("test", "second", &b, [(&x, Access::Write)], || {
            ran = true;
            Ok(())
        });
        assert!(second.is_err(), "overlapping cross-stream write accepted");
        assert!(!ran, "enqueue ran despite the lease conflict");
        a.synchronize().expect("sync A");
        enqueue_leased("test", "after", &b, [(&x, Access::Write)], || Ok(())).expect("after sync");
    }
}
