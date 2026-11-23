//! GPU tests (T3) for buffer lifetime across asynchronous cuRAND generation (plan2 systemic
//! defect 1): run with `cargo test -p curand --test lease -- --ignored --nocapture`.
//!
//! **What.** `Generator::generate_u32` queues a kernel on the caller's stream that writes into the
//! caller's `DeviceBuffer`. The test drops that buffer right after the call, allocates a fresh
//! zeroed buffer of the same size, and checks whether the queued generation later writes into the
//! fresh buffer (write-after-free observed through safe code).
//! **How.** Stream A is held busy by a host function that sleeps, so the generation queued behind
//! it cannot have run when the buffer is dropped. Three allocation kinds are covered, because
//! `cuda-core` frees them differently (`Allocation::drop`): stream-ordered on another stream B
//! (`cuMemFreeAsync` on B), stream-ordered on A itself (`cuMemFreeAsync` on A) and synchronous
//! (`cuMemFree`). The replacement buffer is always zero-filled and synchronized on B.
//! **Used by** the W1.1 evidence in `plan2.md`; the overhead test measures the per-call cost of
//! the completion lease.

use cuda_core::{Context, DeviceBuffer, Stream};
use curand::{Generator, RngType};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Output elements per generation (4 MiB of `u32`).
const LEN: usize = 1 << 20;
/// Repetitions per allocation kind.
const REPS: usize = 20;
/// How long the host function keeps stream A busy.
const BUSY: Duration = Duration::from_millis(200);
/// Time given to the `cuda-resource-retirement` worker to run the deferred free.
const RETIRE: Duration = Duration::from_millis(30);

#[derive(Clone, Copy, Debug)]
enum Kind {
    /// `uninitialized_async` on stream B, the generation runs on stream A.
    OrderedOtherStream,
    /// `uninitialized_async` on stream A, the stream the generation runs on.
    OrderedSameStream,
    /// `DeviceBuffer::zeroed` (`cuMemAlloc` / `cuMemFree`).
    Synchronous,
}

/// Allocates a zero-filled buffer of `LEN` elements; the fill is complete on return.
fn zeroed(kind: Kind, alloc: &Arc<Stream>) -> DeviceBuffer<u32> {
    match kind {
        Kind::Synchronous => DeviceBuffer::zeroed(alloc, LEN).unwrap(),
        Kind::OrderedOtherStream | Kind::OrderedSameStream => {
            // SAFETY: the buffer is zero-filled on its allocation stream (ordered after the
            // allocation) and that stream is synchronized before any other use.
            let mut buf = unsafe { DeviceBuffer::<u32>::uninitialized_async(alloc, LEN) }.unwrap();
            buf.zero_async(alloc).unwrap();
            alloc.synchronize().unwrap();
            buf
        }
    }
}

#[derive(Default)]
struct Tally {
    reused: usize,
    corrupt: usize,
    max_nonzero: usize,
    y_ready_while_busy: usize,
}

fn run_case(kind: Kind, generator: &Generator, a: &Arc<Stream>, b: &Arc<Stream>) -> Tally {
    let mut tally = Tally::default();
    let x_stream = match kind {
        Kind::OrderedSameStream => a,
        Kind::OrderedOtherStream | Kind::Synchronous => b,
    };
    let bytes = (LEN * size_of::<u32>()) as u64;
    for _ in 0..REPS {
        let mut x = zeroed(kind, x_stream);
        let x_addr = x.cu_deviceptr();

        let busy_from = Instant::now();
        a.launch_host_function(|| std::thread::sleep(BUSY)).unwrap();
        generator.generate_u32(a, &mut x).unwrap();
        drop(x);
        std::thread::sleep(RETIRE);

        // The replacement is allocated like `x` but zero-filled and synchronized on B.
        let y = zeroed(kind, b);
        if busy_from.elapsed() < BUSY {
            tally.y_ready_while_busy += 1;
        }
        let y_addr = y.cu_deviceptr();
        a.synchronize().unwrap();
        let nonzero = y
            .to_host_vec(b)
            .unwrap()
            .iter()
            .filter(|v| **v != 0)
            .count();

        if y_addr < x_addr + bytes && x_addr < y_addr + bytes {
            tally.reused += 1;
        }
        if nonzero > 0 {
            tally.corrupt += 1;
        }
        tally.max_nonzero = tally.max_nonzero.max(nonzero);
        drop(y);
        a.synchronize().unwrap();
        b.synchronize().unwrap();
    }
    tally
}

#[test]
#[ignore = "GPU: needs CUDA driver + curand64_10.dll"]
fn gpu_generate_u32_output_dropped_before_completion_is_not_reused() {
    let ctx = Context::new(0).expect("cuda context");
    let a = ctx.new_stream().expect("stream A");
    let b = ctx.new_stream().expect("stream B");
    let generator = Generator::new_device(&ctx, RngType::Philox4_32_10).unwrap();
    generator.set_seed(7).unwrap();
    // Warm-up: first generation initializes cuRAND state outside the measured repetitions.
    let mut warm = DeviceBuffer::<u32>::zeroed(&a, LEN).unwrap();
    generator.generate_u32(&a, &mut warm).unwrap();
    a.synchronize().unwrap();
    drop(warm);

    let mut failures = Vec::new();
    eprintln!("case                 | addr reused | corrupt | max nonzero | Y ready while A busy");
    for kind in [
        Kind::OrderedOtherStream,
        Kind::OrderedSameStream,
        Kind::Synchronous,
    ] {
        let t = run_case(kind, &generator, &a, &b);
        eprintln!(
            "{:<20} | {:>5}/{REPS:<5} | {:>3}/{REPS:<3} | {:>11} | {:>3}/{REPS}",
            format!("{kind:?}"),
            t.reused,
            t.corrupt,
            t.max_nonzero,
            t.y_ready_while_busy
        );
        if t.corrupt > 0 {
            failures.push(kind);
        }
    }
    assert!(
        failures.is_empty(),
        "queued generation wrote into a buffer allocated after the output was dropped: {failures:?}"
    );
}

#[test]
#[ignore = "GPU: needs CUDA driver + curand64_10.dll; timing only, run in release"]
fn gpu_generate_u32_tiny_call_overhead() {
    // Small batches stay below the driver's launch-queue depth, so the enqueue time measures host
    // cost instead of back-pressure from GPU execution.
    const BATCH: usize = 200;
    const BATCHES: usize = 25;
    let ctx = Context::new(0).expect("cuda context");
    let stream = ctx.new_stream().expect("stream");
    let generator = Generator::new_device(&ctx, RngType::Philox4_32_10).unwrap();
    let mut out = DeviceBuffer::<u32>::zeroed(&stream, 4).unwrap();
    for _ in 0..BATCH {
        generator.generate_u32(&stream, &mut out).unwrap();
    }
    stream.synchronize().unwrap();

    let mut enqueue_ns = Vec::with_capacity(BATCHES);
    let mut total_ns = Vec::with_capacity(BATCHES);
    for _ in 0..BATCHES {
        let start = Instant::now();
        for _ in 0..BATCH {
            generator.generate_u32(&stream, &mut out).unwrap();
        }
        let enqueued = start.elapsed();
        stream.synchronize().unwrap();
        let total = start.elapsed();
        enqueue_ns.push(enqueued.as_nanos() as f64 / BATCH as f64);
        total_ns.push(total.as_nanos() as f64 / BATCH as f64);
        // Let completion-bound retirement drain so batches start from the same state.
        std::thread::sleep(Duration::from_millis(20));
    }
    enqueue_ns.sort_by(f64::total_cmp);
    total_ns.sort_by(f64::total_cmp);
    eprintln!(
        "generate_u32(len 4): median enqueue {:.0} ns/call, median enqueue+sync {:.0} ns/call",
        enqueue_ns[BATCHES / 2],
        total_ns[BATCHES / 2]
    );
}
