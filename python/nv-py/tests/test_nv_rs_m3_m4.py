import importlib.util
import pytest

import nv_rs


def _skip_if_missing_runtime(exc: Exception, component: str) -> None:
    """Skip only when the host lacks the required NVIDIA runtime DLL or GPU capability."""
    msg = str(exc).lower()
    if isinstance(exc, nv_rs.errors.NvLibraryNotFoundError) or any(
        needle in msg
        for needle in (
            "not found",
            "failed to load",
            "could not load",
            "unsupported",
            "not supported",
        )
    ):
        pytest.skip(f"{component} runtime unavailable on host: {exc}")
    raise exc


def test_nvjpeg_roundtrip_numpy_device_buffer_and_leak_check():
    np = pytest.importorskip("numpy")
    stream = nv_rs.core.Stream(0)

    h, w = 64, 96
    yy, xx = np.meshgrid(
        np.linspace(0, 255, h, dtype=np.uint8),
        np.linspace(0, 255, w, dtype=np.uint8),
        indexing="ij",
    )
    img = np.stack([xx, yy, ((xx.astype(np.uint16) + yy.astype(np.uint16)) // 2).astype(np.uint8)], axis=-1)
    img = np.ascontiguousarray(img, dtype=np.uint8)

    with nv_rs.trace("trace", print_on_exit=False) as tr:
        enc = nv_rs.nvjpeg.NvjpegEncoder(stream, quality=95)
        dec = nv_rs.nvjpeg.NvjpegDecoder(stream, backend="default")

        # 1. Encode from numpy RGB array -> decode_info -> decode to DeviceBuffer
        jpeg_from_np = enc.encode(img)
        assert isinstance(jpeg_from_np, bytes)
        assert jpeg_from_np[:2] == b"\xff\xd8"

        dw, dh, comp, subsamp = dec.decode_info(jpeg_from_np)
        assert (dw, dh, comp) == (w, h, 3)
        assert isinstance(subsamp, str) and len(subsamp) > 0

        d_decoded_np = dec.decode(jpeg_from_np, output_format="rgbi")
        assert d_decoded_np.shape == [h, w, 3]
        assert d_decoded_np.dtype == "u8"
        out_np = np.frombuffer(d_decoded_np.to_bytes(stream), dtype=np.uint8).reshape(h, w, 3)
        mae_np = float(np.mean(np.abs(out_np.astype(np.float32) - img.astype(np.float32))))
        assert mae_np < 8.0

        # 2. Encode from owned PyDeviceBuffer -> decode roundtrip
        d_in = nv_rs.core.DeviceBuffer.from_buffer(stream, img, shape=[h, w, 3], dtype="u8")
        jpeg_from_dev = enc.encode(d_in)
        assert isinstance(jpeg_from_dev, bytes)
        assert jpeg_from_dev[:2] == b"\xff\xd8"

        d_decoded_dev = dec.decode(jpeg_from_dev, output_format="rgbi")
        assert d_decoded_dev.shape == [h, w, 3]
        out_dev = np.frombuffer(d_decoded_dev.to_bytes(stream), dtype=np.uint8).reshape(h, w, 3)
        mae_dev = float(np.mean(np.abs(out_dev.astype(np.float32) - img.astype(np.float32))))
        assert mae_dev < 8.0

        stream.synchronize()
        del d_decoded_np, d_in, d_decoded_dev, enc, dec

    assert tr.live_bytes_delta == 0


def test_cublas_gemm_f32_dlpack_and_numpy():
    np = pytest.importorskip("numpy")
    stream = nv_rs.core.Stream(0)
    handle = nv_rs.cublas.CublasHandle(stream)

    m, n, k = 32, 24, 16
    alpha, beta = 1.5, 0.5

    # NumPy reference (column-major layout for BLAS)
    rng = np.random.default_rng(123)
    a_np = rng.standard_normal((m, k), dtype=np.float32)
    b_np = rng.standard_normal((k, n), dtype=np.float32)
    c_np = rng.standard_normal((m, n), dtype=np.float32)
    expected_np = alpha * (a_np @ b_np) + beta * c_np

    a_col = np.asfortranarray(a_np)
    b_col = np.asfortranarray(b_np)
    c_col = np.asfortranarray(c_np)

    d_a = nv_rs.core.DeviceBuffer.from_buffer(stream, a_col.tobytes(order="F"), shape=[m * k], dtype="f32")
    d_b = nv_rs.core.DeviceBuffer.from_buffer(stream, b_col.tobytes(order="F"), shape=[k * n], dtype="f32")
    d_c = nv_rs.core.DeviceBuffer.from_buffer(stream, c_col.tobytes(order="F"), shape=[m * n], dtype="f32")

    handle.gemm_f32(d_a, d_b, d_c, m, n, k, alpha=alpha, beta=beta)
    stream.synchronize()
    got_np = np.frombuffer(d_c.to_bytes(stream), dtype=np.float32).reshape((m, n), order="F")
    np.testing.assert_allclose(got_np, expected_np, rtol=1e-4, atol=1e-4)

    # Zero-copy DLPack via PyTorch if available
    if importlib.util.find_spec("torch") is None:
        pytest.skip("torch not installed for DLPack cuBLAS check")
    import torch

    if not torch.cuda.is_available():
        pytest.skip("torch CUDA not available for DLPack cuBLAS check")

    # Row-major (k, m) in torch == column-major (m, k) in BLAS, so C_col.T = B_col.T @ A_col.T
    t_a = torch.randn((k, m), device="cuda", dtype=torch.float32)
    t_b = torch.randn((n, k), device="cuda", dtype=torch.float32)
    t_c = torch.randn((n, m), device="cuda", dtype=torch.float32)
    t_expected = alpha * (t_b @ t_a) + beta * t_c

    dl_a = nv_rs.core.DeviceBuffer.from_dlpack(t_a)
    dl_b = nv_rs.core.DeviceBuffer.from_dlpack(t_b)
    dl_c = nv_rs.core.DeviceBuffer.from_dlpack(t_c)
    assert dl_a.ptr == t_a.data_ptr()
    assert dl_c.ptr == t_c.data_ptr()

    handle.gemm_f32(dl_a, dl_b, dl_c, m, n, k, alpha=alpha, beta=beta)
    stream.synchronize()
    assert torch.allclose(t_c, t_expected, rtol=1e-4, atol=1e-4)
    np.testing.assert_allclose(t_c.cpu().numpy(), t_expected.cpu().numpy(), rtol=1e-4, atol=1e-4)


def test_cufft_1d_c2c_matches_numpy():
    np = pytest.importorskip("numpy")
    stream = nv_rs.core.Stream(0)

    nx = 256
    t = np.linspace(0.0, 2.0 * np.pi, nx, endpoint=False, dtype=np.float32)
    x = (np.cos(3.0 * t) + 0.5 * np.sin(7.0 * t) + 1j * np.sin(5.0 * t)).astype(np.complex64)

    d_in = nv_rs.core.DeviceBuffer.from_buffer(stream, x.tobytes(), shape=[nx * 2], dtype="f32")
    d_out = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros(nx * 2, dtype=np.float32), shape=[nx * 2], dtype="f32"
    )
    d_inv = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros(nx * 2, dtype=np.float32), shape=[nx * 2], dtype="f32"
    )

    plan = nv_rs.cufft.CufftPlan1d(stream, nx, kind="c2c", batch=1)
    plan.execute(d_in, d_out, direction="forward")
    stream.synchronize()

    out = np.frombuffer(d_out.to_bytes(stream), dtype=np.complex64)
    expected = np.fft.fft(x).astype(np.complex64)
    np.testing.assert_allclose(out, expected, rtol=1e-4, atol=1e-4)

    plan.execute(d_out, d_inv, direction="inverse")
    stream.synchronize()
    inv = np.frombuffer(d_inv.to_bytes(stream), dtype=np.complex64) / float(nx)
    np.testing.assert_allclose(inv, x, rtol=1e-4, atol=1e-4)


def test_curand_uniform_and_normal_f32():
    np = pytest.importorskip("numpy")
    stream = nv_rs.core.Stream(0)
    gen = nv_rs.curand.CurandGenerator(stream, seed=42)

    n = 8192
    d_u = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros(n, dtype=np.float32), shape=[n], dtype="f32"
    )
    gen.fill_uniform_f32(d_u)
    stream.synchronize()
    u = np.frombuffer(d_u.to_bytes(stream), dtype=np.float32)
    assert np.all(np.isfinite(u))
    assert float(np.min(u)) >= 0.0
    assert float(np.max(u)) <= 1.0
    assert 0.45 < float(np.mean(u)) < 0.55
    assert 0.24 < float(np.std(u)) < 0.34

    d_n = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros(n, dtype=np.float32), shape=[n], dtype="f32"
    )
    gen.fill_normal_f32(d_n, mean=2.5, stddev=1.5)
    stream.synchronize()
    normal = np.frombuffer(d_n.to_bytes(stream), dtype=np.float32)
    assert np.all(np.isfinite(normal))
    assert abs(float(np.mean(normal)) - 2.5) < 0.15
    assert abs(float(np.std(normal)) - 1.5) < 0.15


def test_nvenc_and_nvdec_smoke():
    np = pytest.importorskip("numpy")
    stream = nv_rs.core.Stream(0)

    try:
        supported = nv_rs.nvenc.is_codec_supported(stream, codec="h264")
        caps = nv_rs.nvenc.codec_caps(stream, "h264")
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "nvenc")
        return

    assert isinstance(caps, dict)
    assert "supported" in caps
    if not supported or not caps["supported"]:
        pytest.skip("H.264 NVENC not supported on this host GPU")

    w = (max(256, int(caps.get("min_width") or 256)) + 3) & ~3
    h = (max(256, int(caps.get("min_height") or 256)) + 1) & ~1
    nv12_size = w * h * 3 // 2
    frame_bytes = np.full(nv12_size, 128, dtype=np.uint8)
    d_frame = nv_rs.core.DeviceBuffer.from_buffer(stream, frame_bytes, shape=[nv12_size], dtype="u8")

    with nv_rs.nvenc.NvencEncoder(stream, w, h, codec="h264", format="nv12") as enc:
        pkt = enc.encode(d_frame, force_idr=True, output_sps_pps=True)
        tail = enc.flush()
    assert enc.is_closed

    bitstream = pkt + b"".join(tail)
    assert len(bitstream) > 0

    try:
        dec_caps = nv_rs.nvdec.decoder_caps(stream, codec="h264", chroma="420", bit_depth=8)
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "nvdec")
        return

    assert isinstance(dec_caps, dict)
    if dec_caps.get("supported") is False:
        pytest.skip("H.264 NVDEC not supported on this host GPU")

    try:
        with nv_rs.nvdec.NvdecDecoder(stream, codec="h264") as dec:
            frames = dec.decode(bitstream) + dec.flush()
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "nvdec")
        return

    assert dec.is_closed
    assert len(frames) >= 1
    assert frames[0].width == w
    assert frames[0].height == h
    assert frames[0].buffer.nbytes > 0


def test_nvofa_optical_flow_smoke():
    np = pytest.importorskip("numpy")
    stream = nv_rs.core.Stream(0)

    w, h = 128, 128
    xx = np.tile(np.linspace(0, 255, w, dtype=np.uint8), (h, 1))
    prev_img = np.ascontiguousarray(xx)
    curr_img = np.ascontiguousarray(np.roll(xx, shift=4, axis=1))

    d_prev = nv_rs.core.DeviceBuffer.from_buffer(stream, prev_img, shape=[h, w], dtype="u8")
    d_curr = nv_rs.core.DeviceBuffer.from_buffer(stream, curr_img, shape=[h, w], dtype="u8")

    try:
        with nv_rs.nvofa.Nvofa(stream, w, h, grid_size=4, input_format="grayscale8") as ofa:
            flow = ofa.execute(d_prev, d_curr)
            stream.synchronize()
            gh, gw = ofa.grid_dims
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "nvofa")
        return

    assert ofa.is_closed
    assert flow.shape == [gh, gw, 2]
    assert flow.dtype == "i16"
    assert flow.nbytes == gh * gw * 2 * 2
