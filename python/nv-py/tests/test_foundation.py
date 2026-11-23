"""Regression tests for the `core` / `diag` / `nvrtc` foundation items P3, P7 and P8 (plan2 Wave 6)."""

import pytest

import nv_rs


def _stream():
    try:
        return nv_rs.core.Stream(0)
    except nv_rs.errors.NvError as exc:
        pytest.skip(f"CUDA context/stream unavailable on host: {exc}")


def _generator(stream):
    try:
        return nv_rs.curand.CurandGenerator(stream, seed=11)
    except nv_rs.errors.NvLibraryNotFoundError as exc:
        pytest.skip(f"cuRAND unavailable: {exc}")


def test_p3_to_bytes_waits_for_the_producer_stream():
    """P3: `to_bytes(other)` must not read while the buffer's producer stream still writes it."""
    np = pytest.importorskip("numpy")
    producer = _stream()
    gen = _generator(producer)
    n = 1 << 22
    busy = nv_rs.core.DeviceBuffer.alloc(producer, [1 << 27], dtype="f32")
    buf = nv_rs.core.DeviceBuffer.from_buffer(producer, bytes(4 * n), dtype="f32")
    for _ in range(40):  # keep the producer stream busy for tens of milliseconds
        gen.fill_uniform_f32(busy)
    gen.fill_uniform_f32(buf)

    other = nv_rs.core.Context(producer.device_id).new_stream()
    data = np.frombuffer(buf.to_bytes(other), dtype=np.float32)
    zeros = int((data == 0.0).sum())
    assert zeros == 0, f"{zeros}/{n} elements read before the producer stream wrote them"


def test_p7_add_dll_directory_warns_when_a_load_failure_is_cached(tmp_path):
    """P7: load failures are cached per process, so a late search directory cannot help them."""
    libs = nv_rs.diag.probe_libraries()
    if libs.get("nvcomp", {}).get("present", True):
        pytest.skip("needs a host where nvCOMP is not discoverable (to cause a cached failure)")
    stream = _stream()
    data = nv_rs.core.DeviceBuffer.from_buffer(stream, b"nv-rs" * 64, dtype="u8")
    with pytest.raises(nv_rs.errors.NvLibraryNotFoundError):
        # The library loads lazily, on the first native call.
        nv_rs.nvcomp.CompressionManager(stream, format="lz4").compress(data)
    assert nv_rs.diag.stats()["lib_load_failures"] > 0
    with pytest.warns(RuntimeWarning, match="already failed"):
        nv_rs.diag.add_dll_directory(str(tmp_path))


def test_p7_add_dll_directory_is_silent_without_failures(tmp_path):
    """P7: no warning while nothing has failed to load (checked in a fresh interpreter)."""
    import subprocess
    import sys

    code = (
        "import warnings, nv_rs\n"
        "warnings.simplefilter('error')\n"
        f"nv_rs.diag.add_dll_directory({str(tmp_path)!r})\n"
        "assert nv_rs.diag.stats()['lib_load_failures'] == 0\n"
    )
    res = subprocess.run([sys.executable, "-c", code], capture_output=True, text=True)
    assert res.returncode == 0, res.stderr


KERNEL = 'extern "C" __global__ void k(float* x) { x[threadIdx.x] = 1.0f; }'


def test_p8_compile_ptx_is_text_and_lto_compile_has_no_ptx():
    """P8: PTX is `None` exactly for LTO-IR output; otherwise retrieval errors propagate."""
    res = nv_rs.nvrtc.compile(KERNEL, arch="compute_86")
    assert isinstance(res.ptx, str) and ".entry k" in res.ptx
    assert res.ptx_bytes == res.ptx.encode()
    assert res.ltoir is None

    # `-dlto` through `extra_args` (the `dlto=True` keyword emits `--dlto`, which NVRTC 13
    # rejects; tracked as a `crates/nvrtc` defect).
    lto = nv_rs.nvrtc.compile(KERNEL, arch="compute_86", extra_args=["-dlto"])
    assert lto.ltoir is not None and len(lto.ltoir) > 0
    assert lto.ptx is None
    assert lto.ptx_bytes is None
