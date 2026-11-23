"""LEASE-API: plan2 systemic defect 1 reproduced through the Python surface.

**What.** `DeviceBuffer.alloc(b, ...)` allocates stream-ordered memory on stream B. A cuRAND
generation that writes the buffer is queued on stream A behind other work, then Python drops the
buffer (`del x`) and allocates a new one on B. If the binding keeps no completion lease on `x`,
`cuda-core` frees it on B right away, the new allocation receives the same memory, and the
generation later writes into it.
**How.** Stream A is kept busy with large cuRAND fills sized from a measured fill time, so the
generation into `x` cannot have run when `x` is dropped. The replacement `y` is read on B once
while A is still busy and once after A finished; any difference is a write into memory that
Python freed. Modelled on `crates/curand/tests/lease.rs`.
**Fix.** `compute.rs` `CurandGenerator.fill_uniform_f32` runs its enqueue through
`buf_util::enqueue_leased`, which keeps `x` allocated until the generation completes.
"""

import time

import pytest

import nv_rs

N = 1 << 20  # f32 elements per victim buffer (4 MiB)
BUSY_N = 1 << 27  # f32 elements of the scratch buffer that keeps stream A busy (512 MiB)
BUSY_S = 0.25  # target time stream A stays busy per repetition
RETIRE_S = 0.03  # time given to cuda-core's retirement worker to queue the deferred free
REPS = 10


def _setup():
    try:
        ctx = nv_rs.core.Context(0)
    except nv_rs.errors.NvError as exc:
        pytest.skip(f"CUDA unavailable: {exc}")
    a, b = ctx.new_stream(), ctx.new_stream()
    try:
        gen = nv_rs.curand.CurandGenerator(a, seed=7)
    except nv_rs.errors.NvLibraryNotFoundError as exc:
        pytest.skip(f"cuRAND unavailable: {exc}")
    return a, b, gen


def _busy_fills(gen, a, busy):
    """Number of scratch fills that keeps stream A busy for about `BUSY_S`."""
    gen.fill_uniform_f32(busy)
    a.synchronize()
    t0 = time.perf_counter()
    for _ in range(10):
        gen.fill_uniform_f32(busy)
    a.synchronize()
    per_fill = (time.perf_counter() - t0) / 10
    return max(1, int(BUSY_S / per_fill))


def test_lease_buffer_dropped_while_other_stream_writes_is_not_reused():
    a, b, gen = _setup()
    busy = nv_rs.core.DeviceBuffer.alloc(a, [BUSY_N], dtype="f32")
    fills = _busy_fills(gen, a, busy)

    reused = corrupt = read_while_busy = 0
    for _ in range(REPS):
        x = nv_rs.core.DeviceBuffer.alloc(b, [N], dtype="f32")
        x_lo, x_hi = x.ptr, x.ptr + x.nbytes
        b.synchronize()  # the stream-ordered allocation is complete before stream A uses it

        t_busy = time.perf_counter()
        for _ in range(fills):
            gen.fill_uniform_f32(busy)
        gen.fill_uniform_f32(x)  # queued on A behind the busy fills
        del x  # Python drop: cuMemFreeAsync on B, without waiting for stream A
        time.sleep(RETIRE_S)

        y = nv_rs.core.DeviceBuffer.alloc(b, [N], dtype="f32")
        before = y.to_bytes(b)  # y's producer stream is B, so this does not wait for A
        if time.perf_counter() - t_busy < BUSY_S:
            read_while_busy += 1
        a.synchronize()
        after = y.to_bytes(b)

        if y.ptr < x_hi and x_lo < y.ptr + y.nbytes:
            reused += 1
        if before != after:
            corrupt += 1
        del y
        a.synchronize()
        b.synchronize()

    print(
        f"\nLEASE repro: fills/rep={fills} reused={reused}/{REPS} corrupt={corrupt}/{REPS} "
        f"read-while-busy={read_while_busy}/{REPS}"
    )
    if corrupt == 0 and read_while_busy < REPS:
        pytest.skip("stream A drained before the first read; the reproduction proves nothing")
    assert corrupt == 0, f"queued generation wrote into freed memory in {corrupt}/{REPS} runs"
