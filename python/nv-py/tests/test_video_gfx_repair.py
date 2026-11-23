"""Regression tests for the Wave 6 repairs of `nv_rs.nvenc` / `nvdec` / `nvofa` / `npp` / `ngx`.

Each test pins one audit item (`audit/2026-11-23/12_device_python.md`, IDs in the test names).
Tests that need a native runtime skip only on `NvLibraryNotFoundError`.
"""

import pytest

import nv_rs

InvalidArg = nv_rs.errors.NvInvalidArgumentError


def _skip_if_missing_runtime(exc: Exception, component: str) -> None:
    if isinstance(exc, nv_rs.errors.NvLibraryNotFoundError):
        pytest.skip(f"{component} runtime unavailable on host: {exc}")
    raise exc


def _npp_ctx(stream):
    try:
        nv_rs.npp.version()
        return nv_rs.npp.NppStreamContext(stream)
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "npp")


def _u8(stream, n):
    np = pytest.importorskip("numpy")
    return nv_rs.core.DeviceBuffer.from_buffer(stream, np.zeros(n, dtype=np.uint8), shape=[n], dtype="u8")


def _f32(stream, n):
    np = pytest.importorskip("numpy")
    return nv_rs.core.DeviceBuffer.from_buffer(stream, np.zeros(n, dtype=np.float32), shape=[n], dtype="f32")


# --- NPP ------------------------------------------------------------------------------------


def test_g3_npp_rejects_in_place_source_and_destination():
    stream = nv_rs.core.Stream(0)
    ctx = _npp_ctx(stream)
    w, h = 16, 8
    img = _u8(stream, w * h)
    with pytest.raises(InvalidArg, match="overlap"):
        nv_rs.npp.threshold_val_8u_c1(ctx, img, img, (w, h), 128, 255)
    with pytest.raises(InvalidArg, match="overlap"):
        nv_rs.npp.filter_gauss_8u(ctx, img, img, w, h)
    a = _f32(stream, 64)
    with pytest.raises(InvalidArg, match="overlap"):
        nv_rs.npp.add_32f(ctx, a, _f32(stream, 64), a)


def test_x3_npp_checks_dtype_and_element_count():
    stream = nv_rs.core.Stream(0)
    ctx = _npp_ctx(stream)
    w, h = 16, 8
    with pytest.raises(InvalidArg, match="dtype"):
        nv_rs.npp.threshold_val_8u_c1(ctx, _f32(stream, w * h), _u8(stream, w * h), (w, h), 1, 2)
    with pytest.raises(InvalidArg, match="too small"):
        nv_rs.npp.rgb_to_gray_8u(ctx, _u8(stream, w * h), _u8(stream, w * h), w, h)
    with pytest.raises(InvalidArg, match="same element count"):
        nv_rs.npp.mul_32f(ctx, _f32(stream, 8), _f32(stream, 9), _f32(stream, 8))


def test_g1_npp_result_survives_deleting_inputs_right_after_the_call():
    np = pytest.importorskip("numpy")
    stream = nv_rs.core.Stream(0)
    ctx = _npp_ctx(stream)
    n = 1 << 16
    a = nv_rs.core.DeviceBuffer.from_buffer(stream, np.full(n, 3.0, dtype=np.float32), shape=[n], dtype="f32")
    b = nv_rs.core.DeviceBuffer.from_buffer(stream, np.full(n, 4.0, dtype=np.float32), shape=[n], dtype="f32")
    out = _f32(stream, n)
    nv_rs.npp.mul_32f(ctx, a, b, out)
    del a, b
    stream.synchronize()
    np.testing.assert_array_equal(np.frombuffer(out.to_bytes(stream), dtype=np.float32), np.full(n, 12.0, np.float32))


# --- NGX / OptiX ----------------------------------------------------------------------------


def test_g2_g5_ngx_exposes_no_fake_support_flags():
    assert not hasattr(nv_rs.ngx, "is_dlss_supported")
    try:
        caps = nv_rs.ngx.NgxCapabilities()
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "ngx")
        return
    for name in ("supported", "status_message", "is_dlss_supported"):
        assert not hasattr(caps, name), name
    inst, dev = caps.required_extensions()
    assert isinstance(inst, list) and isinstance(dev, list)
    req = nv_rs.ngx.feature_requirements("dlss")
    assert req["feature"] == "super_sampling"
    assert "supported" not in req


# --- NVENC ----------------------------------------------------------------------------------


def test_v22_nvenc_rejects_odd_dimensions_for_420_formats():
    stream = nv_rs.core.Stream(0)
    with pytest.raises(InvalidArg, match="even width and height"):
        nv_rs.nvenc.NvencEncoder(stream, 255, 256, codec="h264", format="nv12")


def _h264_caps(stream):
    try:
        caps = nv_rs.nvenc.codec_caps(stream, "h264")
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "nvenc")
    if not caps["supported"]:
        pytest.skip("H.264 NVENC not supported on this host GPU")
    return caps


def test_v7_nvenc_10bit_request_is_honoured_or_rejected_by_name():
    stream = nv_rs.core.Stream(0)
    caps = _h264_caps(stream)
    if caps["support_10bit"]:
        with nv_rs.nvenc.NvencEncoder(stream, 256, 256, codec="h264", format="yuv420_10bit") as enc:
            assert enc.format == "yuv420_10bit"
    else:
        with pytest.raises(InvalidArg, match="10-bit"):
            nv_rs.nvenc.NvencEncoder(stream, 256, 256, codec="h264", format="yuv420_10bit")


def test_x3_nvenc_checks_frame_dtype_and_size():
    stream = nv_rs.core.Stream(0)
    _h264_caps(stream)
    w, h = 256, 256
    with nv_rs.nvenc.NvencEncoder(stream, w, h, codec="h264", format="nv12") as enc:
        with pytest.raises(InvalidArg, match="dtype"):
            enc.encode(_f32(stream, w * h * 3 // 2))
        with pytest.raises(InvalidArg, match="too small"):
            enc.encode(_u8(stream, w * h))


def test_v2_v8_nvenc_every_frame_comes_back_through_encode_or_flush():
    np = pytest.importorskip("numpy")
    stream = nv_rs.core.Stream(0)
    _h264_caps(stream)
    w, h, frames = 256, 256, 12
    stream_bytes = b""
    with nv_rs.nvenc.NvencEncoder(stream, w, h, codec="h264", preset="p7", format="nv12") as enc:
        for i in range(frames):
            img = np.full(w * h * 3 // 2, (i * 17) % 256, dtype=np.uint8)
            d = nv_rs.core.DeviceBuffer.from_buffer(stream, img, shape=[img.size], dtype="u8")
            stream_bytes += enc.encode(d, force_idr=(i == 0), output_sps_pps=(i == 0))
            del d  # a buffered frame must stay alive inside the encoder
        stream_bytes += b"".join(enc.flush())
    assert len(stream_bytes) > 0
    try:
        with nv_rs.nvdec.NvdecDecoder(stream, codec="h264") as dec:
            decoded = dec.decode(stream_bytes) + dec.flush()
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "nvdec")
        return
    assert len(decoded) == frames


# --- NVDEC / NVOFA --------------------------------------------------------------------------


def test_v16_nvdec_caps_reject_bit_depth_below_8():
    stream = nv_rs.core.Stream(0)
    with pytest.raises(InvalidArg, match="at least 8"):
        nv_rs.nvdec.decoder_caps(stream, codec="h264", chroma="420", bit_depth=7)


def test_v22_nvofa_rejects_odd_nv12_dimensions():
    stream = nv_rs.core.Stream(0)
    with pytest.raises(InvalidArg, match="even width and height"):
        nv_rs.nvofa.Nvofa(stream, 127, 128, input_format="nv12")


def test_v18_nvofa_result_records_its_producer_stream():
    np = pytest.importorskip("numpy")
    stream = nv_rs.core.Stream(0)
    w, h = 128, 128
    prev = nv_rs.core.DeviceBuffer.from_buffer(stream, np.zeros((h, w), np.uint8), shape=[h, w], dtype="u8")
    curr = nv_rs.core.DeviceBuffer.from_buffer(stream, np.zeros((h, w), np.uint8), shape=[h, w], dtype="u8")
    try:
        with nv_rs.nvofa.Nvofa(stream, w, h, grid_size=4) as ofa:
            with pytest.raises(InvalidArg, match="dtype"):
                ofa.execute(_f32(stream, w * h), curr)
            flow = ofa.execute(prev, curr)
            del prev, curr
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "nvofa")
        return
    assert flow.stream is not None
    vectors = np.frombuffer(flow.to_bytes(stream), dtype=np.int16)
    assert vectors.size == flow.shape[0] * flow.shape[1] * 2
