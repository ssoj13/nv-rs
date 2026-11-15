import importlib.util
import pytest

import nv_rs


def test_diag_doctor_and_levels():
    prev = nv_rs.diag.get_level()
    try:
        assert nv_rs.diag.set_level("trace") == "TRACE"
        assert nv_rs.diag.get_level() == "TRACE"
        doc = nv_rs.diag.doctor()
        assert doc["version"] == nv_rs.__version__
        assert "libraries" in doc
        assert "nvcuda" in doc["libraries"]
        text = nv_rs.format_doctor()
        assert "nv_rs " in text
        assert "Libraries:" in text
    finally:
        nv_rs.diag.set_level(prev.lower())


def test_diag_trace_guard_and_errors():
    nv_rs.diag.clear_events()
    with nv_rs.trace("trace", print_on_exit=False) as tr:
        with pytest.raises(nv_rs.errors.NvCompileError):
            nv_rs.nvrtc.compile("__global__ void bad( {")
    assert tr.error_delta >= 1
    assert any(not ev["ok"] and ev["category"] == "nvrtc" for ev in tr.events)


def test_nvml_telemetry():
    session = nv_rs.nvml.Nvml()
    assert session.device_count() >= 1
    dev = session.device(0)
    summary = dev.summary()
    assert summary["index"] == 0
    assert isinstance(summary["name"], str) and len(summary["name"]) > 0
    assert summary["memory"]["total"] > 0


def test_nvrtc_compile_ptx():
    maj, _ = nv_rs.nvrtc.version()
    assert maj >= 11
    archs = nv_rs.nvrtc.supported_archs()
    assert len(archs) > 0
    res = nv_rs.nvrtc.compile(
        r"""
        extern "C" __global__ void add_one(float* x, int n) {
            int i = blockIdx.x * blockDim.x + threadIdx.x;
            if (i < n) x[i] += 1.0f;
        }
        """,
        name="add_one.cu",
        name_expressions=["add_one"],
        fast_math=True,
    )
    assert "add_one" in res.ptx
    assert res.lowered_name("add_one") == "add_one"


def test_core_buffer_numpy_and_leak_tracking():
    np = pytest.importorskip("numpy")
    stream = nv_rs.core.Stream(0)
    arr = np.arange(1024, dtype=np.float32)

    with nv_rs.trace("trace", print_on_exit=False) as tr:
        d_buf = nv_rs.core.DeviceBuffer.from_buffer(
            stream, arr, shape=list(arr.shape), dtype="f32"
        )
        assert d_buf.nbytes == arr.nbytes
        assert d_buf.shape == [1024]
        assert d_buf.dtype == "f32"
        raw = d_buf.to_bytes(stream)
        out = np.frombuffer(raw, dtype=np.float32)
        np.testing.assert_array_equal(out, arr)
        del d_buf

    assert tr.live_bytes_delta == 0


def test_dlpack_and_cuda_array_interface_interop():
    if importlib.util.find_spec("torch") is None:
        pytest.skip("torch not installed")
    import torch

    if not torch.cuda.is_available():
        pytest.skip("torch CUDA not available")

    stream = nv_rs.core.Stream(0)
    t_in = torch.arange(256, device="cuda", dtype=torch.float32)

    # Zero-copy import torch -> nv_rs -> torch
    buf = nv_rs.core.DeviceBuffer.from_dlpack(t_in)
    assert buf.ptr == t_in.data_ptr()
    assert buf.shape == [256]
    assert buf.dtype == "f32"
    assert buf.__cuda_array_interface__["data"][0] == t_in.data_ptr()

    t_out = torch.from_dlpack(buf)
    assert t_out.data_ptr() == t_in.data_ptr()
    assert torch.equal(t_in, t_out)

    # Also check CuPy if installed
    if importlib.util.find_spec("cupy") is not None:
        import cupy as cp

        c_arr = cp.asarray(buf)
        assert int(c_arr.data.ptr) == buf.ptr
        assert float(c_arr.sum()) == pytest.approx(float(t_in.sum().item()))

    stream.synchronize()


def test_nvcomp_roundtrip():
    stream = nv_rs.core.Stream(0)
    payload = (b"nv-rs-python-nvcomp-test-" * 2048)
    d_in = nv_rs.core.DeviceBuffer.from_buffer(stream, payload, dtype="u8")

    for fmt in ("lz4", "zstd"):
        mgr = nv_rs.nvcomp.CompressionManager(stream, format=fmt, chunk_size=16384)
        comp = mgr.compress(d_in)
        assert comp.compressed_bytes < comp.uncompressed_bytes
        assert comp.ratio > 1.0
        d_out = mgr.decompress(comp)
        assert d_out.to_bytes(stream) == payload
