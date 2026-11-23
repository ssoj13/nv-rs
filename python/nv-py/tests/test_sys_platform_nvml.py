"""Regression tests for the `nvml`, `cuda_ext`, `cupti` and `wgpu` repairs (audit 12, Wave 6).

Each test pins one fixed defect through the public Python surface and skips when its runtime
(CUDA, NVML, CUPTI, wgpu interop) is missing on the host:
- NV-01: optional NVML telemetry no longer records phantom errors in `nv_rs.diag`;
- SE-02 / SE-05: `HostRegistration.close()` is idempotent and reports through exceptions;
- SE-01: managed-memory host access synchronizes the context (round trip after GPU work);
- SE-03: the CUPTI subscriber event buffer is bounded;
- SE-04: closing a `CuptiSubscriber` from inside its own callback raises instead of deadlocking;
- SE-06 / SE-11: `wgpu` buffer creation errors raise instead of panicking;
- SE-09: the dedicated-allocation rule follows the `MemoryHandle` kind.
"""

import pytest

import nv_rs


def _ctx():
    try:
        return nv_rs.core.Context(0)
    except nv_rs.errors.NvError as exc:
        pytest.skip(f"CUDA unavailable: {exc}")


def test_nvml_optional_telemetry_records_no_errors():
    try:
        session = nv_rs.nvml.Nvml()
    except nv_rs.errors.NvLibraryNotFoundError as exc:
        pytest.skip(f"NVML unavailable: {exc}")
    dev = session.device(0)
    with nv_rs.trace("trace", print_on_exit=False) as tr:
        clocks = dev.clocks_mhz()
        summary = dev.summary()
    assert set(clocks) == {"graphics", "sm", "memory"}
    assert all(v is None or isinstance(v, int) for v in clocks.values())
    assert summary["index"] == 0
    # Unsupported fields are `None` without a diag error; real errors would have raised.
    assert tr.error_delta == 0


def test_host_registration_close_is_idempotent():
    _ctx()
    payload = bytes(range(64))
    hreg = nv_rs.cuda_ext.HostRegistration(payload, map_to_device=True, device=0)
    assert hreg.to_bytes() == payload
    hreg.close()
    hreg.close()
    assert hreg.nbytes == 0
    assert hreg.device_ptr is None
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
        hreg.to_bytes()


def test_managed_buffer_host_access_after_gpu_work():
    ctx = _ctx()
    stream = ctx.new_stream()
    mbuf = nv_rs.cuda_ext.ManagedBuffer.zeroed(256, context=ctx)
    # Host access goes through the binding's context synchronize (no explicit one here); this
    # pins the round trip and the range check, not the race itself (that needs a running kernel).
    dev = nv_rs.core.DeviceBuffer.from_buffer(stream, bytes(1 << 20))
    _ = dev.to_bytes(stream)
    mbuf.write_bytes(b"\x07" * 16, offset=8)
    data = mbuf.to_bytes()
    assert data[8:24] == b"\x07" * 16
    assert data[:8] == b"\x00" * 8
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
        mbuf.write_bytes(b"\x00" * 16, offset=250)


def test_external_memory_desc_dedicated_rule():
    desc = nv_rs.cuda_ext.ExternalMemoryDesc("d3d12_heap", 4096)
    assert desc.handle_kind == "d3d12_heap"
    assert desc.dedicated is False
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
        nv_rs.cuda_ext.ExternalMemoryDesc("d3d12_resource", 4096)
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
        nv_rs.cuda_ext.ExternalMemoryDesc("opaque_fd", 0)


def _subscriber(**kwargs):
    try:
        return nv_rs.cupti.CuptiSubscriber(device=0, **kwargs)
    except nv_rs.errors.NvError as exc:
        pytest.skip(f"CUPTI subscriber unavailable: {exc}")


def test_cupti_subscriber_event_buffer_is_bounded():
    ctx = _ctx()
    stream = ctx.new_stream()
    sub = _subscriber(context=ctx)
    try:
        sub.enable_domain("driver_api", True)
        with nv_rs.trace("trace", print_on_exit=False) as tr:
            # Every synchronize issues at least one driver API call (the context bind check
            # included), so this overflows the 65536-event buffer.
            for _ in range(70_000):
                stream.synchronize()
            events = sub.drain_events()
        assert isinstance(events, list)
        assert len(events) <= 1 << 16
        assert any(
            ev["category"] == "cupti" and ev["op"] == "drain_events" for ev in tr.events
        ), "dropped events must be reported in nv_rs.diag"
        sub.check_errors()
    finally:
        sub.close()
        sub.close()


def test_cupti_subscriber_close_inside_callback_raises():
    ctx = _ctx()
    stream = ctx.new_stream()
    holder = {}

    def on_event(domain, cbid):
        holder["sub"].close()

    sub = _subscriber(context=ctx, callback=on_event)
    holder["sub"] = sub
    try:
        sub.enable_domain("driver_api", True)
        stream.synchronize()
        sub.enable_domain("driver_api", False)
        with pytest.raises(RuntimeError, match="inside a CUPTI callback"):
            sub.check_errors()
    finally:
        holder.clear()
        sub.close()


def test_wgpu_create_buffer_errors_raise_instead_of_panicking():
    if not nv_rs.wgpu.is_supported():
        pytest.skip("wgpu CUDA interop not supported on host")
    try:
        ictx = nv_rs.wgpu.InteropContext(backend="auto", label="nv-rs-repair-test")
    except nv_rs.errors.NvError as exc:
        pytest.skip(f"wgpu InteropContext unavailable: {exc}")
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
        ictx.create_buffer(1 << 62)
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
        ictx.create_buffer(4096, usage="storage|map_read")
    buf = ictx.create_buffer(4096)
    assert buf.size >= 4096
