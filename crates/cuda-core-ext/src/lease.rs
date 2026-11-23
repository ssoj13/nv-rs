//! Completion leases for asynchronous work that a native library queues on raw device pointers.
//!
//! **What.** [`enqueue_leased`] runs one enqueue closure (typically a single FFI call) while each
//! listed buffer holds a `cuda_core::AllocationLease`, then hands every lease to `cuda-core`'s
//! completion-bound retirement on the same stream.
//! **Why.** Dropping a `cuda_core::DeviceBuffer` never waits for GPU work (`Allocation::drop`);
//! only an outstanding lease keeps its allocation alive. Without one, a stream-ordered buffer
//! allocated on stream B and written by work queued on stream A is freed on B while A still uses
//! it, and the next allocation can receive the same memory (reproduced by
//! `curand/tests/lease.rs` and `compute-sanitizer --tool memcheck`).
//! **How.** Same order as the private `DeviceBuffer::enqueue_access` in `cuda-core`: acquire ->
//! bind -> enqueue -> retain. `RawDeviceBuffer::acquire` checks the buffer's context against the
//! stream's and rejects a write that overlaps an unfinished access on another stream (or any
//! access overlapping an unfinished write there) with `CUDA_ERROR_NOT_READY`; on that or any other
//! failure before the closure runs, nothing is queued and the leases taken so far are released.
//! Once the closure has run, every lease is retained even when the closure failed, because a
//! native call can fail after queuing part of its work. `AllocationLease::retain_until_complete`
//! records one event per lease; a lease whose event cannot be recorded is leaked by `cuda-core`
//! (memory kept, never freed), which is safer than freeing memory that work may still use.
//! **Used by** `curand::Generator::generate_u32`.

use cuda_core::{AllocationLease, DriverError, RawDeviceBuffer, Stream};

/// How the queued work uses one buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    /// The work only reads the buffer; reads on different streams may overlap.
    Read,
    /// The work writes the buffer; excludes unfinished accesses on other streams.
    Write,
}

/// Runs `enqueue` with `buffers` leased on `stream` and keeps them allocated until the work that
/// `enqueue` queued on `stream` completes.
///
/// `enqueue` must queue its work on `stream` only; work queued elsewhere is not covered.
/// The same buffer may appear more than once (for example read and written in place).
pub fn enqueue_leased<const N: usize, E: From<DriverError>>(
    stream: &Stream,
    buffers: [(&RawDeviceBuffer, Access); N],
    enqueue: impl FnOnce() -> Result<(), E>,
) -> Result<(), E> {
    let mut leases: [Option<AllocationLease>; N] = std::array::from_fn(|_| None);
    for (slot, (buffer, access)) in leases.iter_mut().zip(buffers) {
        *slot = Some(buffer.acquire(stream, access == Access::Write, false)?);
    }
    stream.context().bind_to_thread()?;
    let submitted = enqueue();
    let mut retained = Ok(());
    for lease in leases.into_iter().flatten() {
        // Keep retaining the remaining leases after a failure: each one still guards its buffer.
        if let Err(error) = lease.retain_until_complete(stream) {
            retained = retained.and(Err(error));
        }
    }
    submitted.and(retained.map_err(E::from))
}
