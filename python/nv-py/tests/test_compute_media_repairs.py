"""Regression tests for the plan2 Wave 6 repairs of `compute.rs`, `sparse_ai.rs`, `nvjpeg.rs` and
`nvcomp.rs` (audit ledger `12_device_python.md`, items X*/C*).

Every test needs a CUDA GPU; a missing optional NVIDIA library skips only on the typed
`NvLibraryNotFoundError`, so argument errors still fail.
"""

import pytest

import nv_rs


def _stream():
    try:
        return nv_rs.core.Stream(0)
    except nv_rs.errors.NvError as exc:
        pytest.skip(f"CUDA unavailable: {exc}")


def _or_skip(factory, component):
    try:
        return factory()
    except nv_rs.errors.NvLibraryNotFoundError as exc:
        pytest.skip(f"{component} unavailable: {exc}")


def _f32(stream, values):
    np = pytest.importorskip("numpy")
    arr = np.ascontiguousarray(values, dtype=np.float32)
    return nv_rs.core.DeviceBuffer.from_buffer(stream, arr, shape=list(arr.shape), dtype="f32")


# ---------------------------------------------------------------------------
# X3: dtype is checked per argument
# ---------------------------------------------------------------------------


def test_x3_gemm_rejects_a_non_f32_buffer():
    np = pytest.importorskip("numpy")
    stream = _stream()
    handle = _or_skip(lambda: nv_rs.cublas.CublasHandle(stream), "cuBLAS")
    a = nv_rs.core.DeviceBuffer.from_buffer(stream, np.zeros(4, dtype=np.int32), dtype="i32")
    b = _f32(stream, np.zeros(4))
    c = _f32(stream, np.zeros(4))
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError, match="dtype"):
        handle.gemm_f32(a, b, c, 2, 2, 2)


def test_x3_spmv_rejects_float_column_indices():
    np = pytest.importorskip("numpy")
    stream = _stream()
    handle = _or_skip(lambda: nv_rs.cusparse.CusparseHandle(stream), "cuSPARSE")
    rows = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.array([0, 1, 2], dtype=np.int32), dtype="i32"
    )
    cols = _f32(stream, [0.0, 1.0])  # must be i32
    vals = _f32(stream, [1.0, 2.0])
    x = _f32(stream, [1.0, 1.0])
    y = _f32(stream, [0.0, 0.0])
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError, match="dtype"):
        handle.spmv_f32(rows, cols, vals, x, y, 2, 2, 2)


# ---------------------------------------------------------------------------
# X2: a write while another stream still writes the buffer is rejected, not raced
# ---------------------------------------------------------------------------


def test_x2_cross_stream_write_conflict_is_rejected_until_synchronized():
    np = pytest.importorskip("numpy")
    a = _stream()
    ctx = nv_rs.core.Context(a.device_id)
    b = ctx.new_stream()
    gen = _or_skip(lambda: nv_rs.curand.CurandGenerator(a, seed=5), "cuRAND")
    handle = _or_skip(lambda: nv_rs.cublas.CublasHandle(b), "cuBLAS")
    n = 4
    x = _f32(a, np.zeros(n * n))
    m1 = _f32(b, np.eye(n))
    busy = nv_rs.core.DeviceBuffer.alloc(a, [1 << 27], dtype="f32")
    for _ in range(40):  # keep stream A busy for tens of milliseconds
        gen.fill_uniform_f32(busy)
    gen.fill_uniform_f32(x)  # unfinished write on A
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError, match="another stream"):
        handle.gemm_f32(m1, m1, x, n, n, n)  # writes x on B
    a.synchronize()
    handle.gemm_f32(m1, m1, x, n, n, n)
    b.synchronize()
    got = np.frombuffer(x.to_bytes(b), dtype=np.float32).reshape(n, n)
    np.testing.assert_allclose(got, np.eye(n, dtype=np.float32))


# ---------------------------------------------------------------------------
# C18: r2c/c2r direction is fixed by the plan
# ---------------------------------------------------------------------------


def test_c18_r2c_rejects_inverse_direction_and_runs_without_one():
    np = pytest.importorskip("numpy")
    stream = _stream()
    nx = 16
    plan = _or_skip(lambda: nv_rs.cufft.CufftPlan1d(stream, nx, kind="r2c"), "cuFFT")
    x = np.cos(np.linspace(0.0, 2.0 * np.pi, nx, endpoint=False)).astype(np.float32)
    d_in = _f32(stream, x)
    d_out = _f32(stream, np.zeros(2 * (nx // 2 + 1)))
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
        plan.execute(d_in, d_out, direction="inverse")
    plan.execute(d_in, d_out)
    plan.execute(d_in, d_out, direction="forward")
    stream.synchronize()
    got = np.frombuffer(d_out.to_bytes(stream), dtype=np.complex64)
    np.testing.assert_allclose(got, np.fft.rfft(x).astype(np.complex64), rtol=1e-4, atol=1e-4)


# ---------------------------------------------------------------------------
# C9: cuDNN activation coef defaults per mode
# ---------------------------------------------------------------------------


def test_c9_clipped_relu_requires_coef_and_elu_defaults_to_alpha_one():
    np = pytest.importorskip("numpy")
    stream = _stream()
    handle = _or_skip(lambda: nv_rs.cudnn.CudnnHandle(stream), "cuDNN")
    shape = [1, 1, 2, 4]
    x_np = np.array([-2.0, -1.0, -0.5, 0.0, 0.5, 1.0, 2.0, 7.0], dtype=np.float32).reshape(shape)
    d_x = _f32(stream, x_np)
    d_y = _f32(stream, np.zeros(shape))
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError, match="coef"):
        handle.activation_forward_f32(d_x, d_y, shape, mode="clipped_relu")

    handle.activation_forward_f32(d_x, d_y, shape, mode="elu")
    stream.synchronize()
    got = np.frombuffer(d_y.to_bytes(stream), dtype=np.float32).reshape(shape)
    np.testing.assert_allclose(got, np.where(x_np > 0, x_np, np.expm1(x_np)), rtol=1e-5, atol=1e-5)

    handle.activation_forward_f32(d_x, d_y, shape, mode="clipped_relu", coef=6.0)
    stream.synchronize()
    got = np.frombuffer(d_y.to_bytes(stream), dtype=np.float32).reshape(shape)
    np.testing.assert_allclose(got, np.clip(x_np, 0.0, 6.0), rtol=1e-6, atol=1e-6)


# ---------------------------------------------------------------------------
# nvJPEG: C3, C5, C13, C4
# ---------------------------------------------------------------------------


def _image():
    np = pytest.importorskip("numpy")
    h, w = 48, 64
    yy, xx = np.meshgrid(np.arange(h), np.arange(w), indexing="ij")
    img = np.stack([xx * 4, yy * 5, (xx + yy) * 2], axis=-1).astype(np.uint8)
    return np.ascontiguousarray(img)


def test_c3_per_call_quality_does_not_change_the_encoder_default():
    stream = _stream()
    enc = _or_skip(lambda: nv_rs.nvjpeg.NvjpegEncoder(stream, quality=90), "nvJPEG")
    img = _image()
    default_q = enc.encode(img)
    overridden = enc.encode(img, quality=10, subsampling="444")
    assert overridden != default_q
    assert enc.encode(img) == default_q


def test_c13_width_without_height_is_rejected():
    stream = _stream()
    enc = _or_skip(lambda: nv_rs.nvjpeg.NvjpegEncoder(stream), "nvJPEG")
    img = _image()
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError, match="both width and height"):
        enc.encode(img, width=img.shape[1])
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError, match="width and height are required"):
        enc.encode(img.tobytes())


def test_c5_c4_decode_result_carries_its_stream_and_streams_can_alternate():
    np = pytest.importorskip("numpy")
    stream = _stream()
    other = nv_rs.core.Context(stream.device_id).new_stream()
    enc = _or_skip(lambda: nv_rs.nvjpeg.NvjpegEncoder(stream, quality=95), "nvJPEG")
    dec = nv_rs.nvjpeg.NvjpegDecoder(stream)
    jpeg = enc.encode(_image())

    first = dec.decode(jpeg)
    assert first.stream.raw_ptr == stream.raw_ptr
    reference = first.to_bytes(stream)
    for s in (other, stream, other, stream):
        out = dec.decode(jpeg, stream=s)
        assert out.stream.raw_ptr == s.raw_ptr
        assert out.to_bytes(s) == reference
    assert np.frombuffer(reference, dtype=np.uint8).size == 48 * 64 * 3


# ---------------------------------------------------------------------------
# nvCOMP: C2, C19
# ---------------------------------------------------------------------------


def test_c2_c19_compression_manager_has_no_dead_stream_parameter_and_canonical_format():
    stream = _stream()
    payload = b"nv-rs-wave6-nvcomp-" * 1024
    d_in = nv_rs.core.DeviceBuffer.from_buffer(stream, payload, dtype="u8")
    mgr = nv_rs.nvcomp.CompressionManager(stream, format=" LZ4 ", chunk_size=16384)
    comp = _or_skip(lambda: mgr.compress(d_in), "nvCOMP")
    assert comp.format == "lz4"
    with pytest.raises(TypeError):
        mgr.compress(d_in, stream=stream)
    out = mgr.decompress(comp)
    assert out.stream.raw_ptr == stream.raw_ptr
    assert out.to_bytes(stream) == payload
