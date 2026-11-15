import sys
import pytest

import nv_rs


def _skip_if_missing_runtime(exc: Exception, component: str) -> None:
    """Skip only when the host lacks the required optional runtime DLL/SO or GPU capability."""
    msg = str(exc).lower()
    if isinstance(exc, nv_rs.errors.NvLibraryNotFoundError) or any(
        needle in msg
        for needle in (
            "not found",
            "failed to load",
            "could not load",
            "unsupported",
            "not supported",
            "no such file",
            "unavailable",
        )
    ):
        pytest.skip(f"{component} runtime unavailable on host: {exc}")
    raise exc


# ---------------------------------------------------------------------------
# 1. nv_rs.cusparse
# ---------------------------------------------------------------------------


def test_cusparse_spmv_f32_matches_numpy():
    np = pytest.importorskip("numpy")
    try:
        ver = nv_rs.cusparse.version()
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "cusparse")
        return

    assert isinstance(ver, int) and ver > 0

    stream = nv_rs.core.Stream(0)
    handle = nv_rs.cusparse.CusparseHandle(stream)
    assert handle.version() == ver
    handle.set_stream(stream)

    # Non-symmetric 3x3 CSR matrix:
    # [[1, 0, 2],
    #  [0, 3, 0],
    #  [4, 5, 6]]
    a_dense = np.array(
        [
            [1.0, 0.0, 2.0],
            [0.0, 3.0, 0.0],
            [4.0, 5.0, 6.0],
        ],
        dtype=np.float32,
    )
    row_offsets = np.array([0, 2, 3, 6], dtype=np.int32)
    col_ind = np.array([0, 2, 1, 0, 1, 2], dtype=np.int32)
    values = np.array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0], dtype=np.float32)
    x_np = np.array([1.5, -2.0, 0.5], dtype=np.float32)
    y0_np = np.array([10.0, -4.0, 2.0], dtype=np.float32)

    alpha, beta = 2.0, 0.5
    rows, cols, nnz = 3, 3, 6

    with nv_rs.trace("trace", print_on_exit=False) as tr:
        d_row = nv_rs.core.DeviceBuffer.from_buffer(stream, row_offsets, shape=[4], dtype="i32")
        d_col = nv_rs.core.DeviceBuffer.from_buffer(stream, col_ind, shape=[nnz], dtype="i32")
        d_val = nv_rs.core.DeviceBuffer.from_buffer(stream, values, shape=[nnz], dtype="f32")
        d_x = nv_rs.core.DeviceBuffer.from_buffer(stream, x_np, shape=[cols], dtype="f32")
        d_y = nv_rs.core.DeviceBuffer.from_buffer(stream, y0_np, shape=[rows], dtype="f32")

        # Non-transpose: y = alpha * A * x + beta * y
        handle.spmv_f32(
            d_row, d_col, d_val, d_x, d_y, rows, cols, nnz, alpha=alpha, beta=beta, transpose=False
        )
        stream.synchronize()
        got = np.frombuffer(d_y.to_bytes(stream), dtype=np.float32)
        expected = alpha * (a_dense @ x_np) + beta * y0_np
        np.testing.assert_allclose(got, expected, rtol=1e-5, atol=1e-5)

        # Transpose: y2 = alpha * A^T * x + beta * y0
        d_y2 = nv_rs.core.DeviceBuffer.from_buffer(stream, y0_np, shape=[cols], dtype="f32")
        handle.spmv_f32(
            d_row, d_col, d_val, d_x, d_y2, rows, cols, nnz, alpha=alpha, beta=beta, transpose=True
        )
        stream.synchronize()
        got_t = np.frombuffer(d_y2.to_bytes(stream), dtype=np.float32)
        expected_t = alpha * (a_dense.T @ x_np) + beta * y0_np
        np.testing.assert_allclose(got_t, expected_t, rtol=1e-5, atol=1e-5)

        del d_row, d_col, d_val, d_x, d_y, d_y2, handle

    assert tr.live_bytes_delta == 0


# ---------------------------------------------------------------------------
# 2. nv_rs.cusolver
# ---------------------------------------------------------------------------


def test_cusolver_lu_and_cholesky_solve_match_numpy():
    np = pytest.importorskip("numpy")
    stream = nv_rs.core.Stream(0)
    try:
        handle = nv_rs.cusolver.CusolverDnHandle(stream)
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "cusolver")
        return

    handle.set_stream(stream)
    n = 4
    rng = np.random.default_rng(42)

    # 1. LU factorization & solve (getrf_f32 + getrs_f32)
    a_np = rng.standard_normal((n, n), dtype=np.float32) + 3.0 * np.eye(n, dtype=np.float32)
    b_np = rng.standard_normal((n, 2), dtype=np.float32)
    expected_x = np.linalg.solve(a_np, b_np)

    a_col = np.asfortranarray(a_np)
    b_col = np.asfortranarray(b_np)
    d_a = nv_rs.core.DeviceBuffer.from_buffer(stream, a_col.tobytes(order="F"), shape=[n * n], dtype="f32")
    d_b = nv_rs.core.DeviceBuffer.from_buffer(stream, b_col.tobytes(order="F"), shape=[n * 2], dtype="f32")
    d_ipiv = nv_rs.core.DeviceBuffer.from_buffer(stream, np.zeros(n, dtype=np.int32), shape=[n], dtype="i32")
    d_info = nv_rs.core.DeviceBuffer.from_buffer(stream, np.zeros(1, dtype=np.int32), shape=[1], dtype="i32")

    handle.getrf_f32(d_a, d_ipiv, d_info, n, n)
    stream.synchronize()
    info_lu = np.frombuffer(d_info.to_bytes(stream), dtype=np.int32)[0]
    assert info_lu == 0

    handle.getrs_f32(d_a, d_ipiv, d_b, d_info, n, nrhs=2)
    stream.synchronize()
    info_solve = np.frombuffer(d_info.to_bytes(stream), dtype=np.int32)[0]
    assert info_solve == 0
    got_x = np.frombuffer(d_b.to_bytes(stream), dtype=np.float32).reshape((n, 2), order="F")
    np.testing.assert_allclose(got_x, expected_x, rtol=1e-4, atol=1e-4)

    # 2. Cholesky factorization & solve (potrf_f32 + potrs_f32) on SPD matrix
    m_np = rng.standard_normal((n, n), dtype=np.float32)
    spd_np = (m_np.T @ m_np) + 2.0 * np.eye(n, dtype=np.float32)
    b_spd = rng.standard_normal((n, 1), dtype=np.float32)
    expected_spd = np.linalg.solve(spd_np, b_spd)

    d_spd = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.asfortranarray(spd_np).tobytes(order="F"), shape=[n * n], dtype="f32"
    )
    d_b_spd = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.asfortranarray(b_spd).tobytes(order="F"), shape=[n], dtype="f32"
    )
    handle.potrf_f32(d_spd, d_info, n, upper=False)
    stream.synchronize()
    assert np.frombuffer(d_info.to_bytes(stream), dtype=np.int32)[0] == 0

    handle.potrs_f32(d_spd, d_b_spd, d_info, n, nrhs=1, upper=False)
    stream.synchronize()
    assert np.frombuffer(d_info.to_bytes(stream), dtype=np.int32)[0] == 0
    got_spd = np.frombuffer(d_b_spd.to_bytes(stream), dtype=np.float32).reshape((n, 1), order="F")
    np.testing.assert_allclose(got_spd, expected_spd, rtol=1e-4, atol=1e-4)


# ---------------------------------------------------------------------------
# 3. nv_rs.cudnn
# ---------------------------------------------------------------------------


def test_cudnn_activation_and_softmax_match_numpy():
    np = pytest.importorskip("numpy")
    try:
        ver = nv_rs.cudnn.version()
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "cudnn")
        return

    assert isinstance(ver, int) and ver > 0

    stream = nv_rs.core.Stream(0)
    try:
        handle = nv_rs.cudnn.CudnnHandle(stream)
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "cudnn")
        return

    handle.set_stream(stream)

    shape = [2, 5, 3, 4]
    rng = np.random.default_rng(7)
    x_np = rng.standard_normal(shape, dtype=np.float32)

    d_x = nv_rs.core.DeviceBuffer.from_buffer(stream, x_np, shape=shape, dtype="f32")
    d_y = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros(shape, dtype=np.float32), shape=shape, dtype="f32"
    )

    # ReLU
    handle.activation_forward_f32(d_x, d_y, shape, mode="relu", alpha=1.0, beta=0.0)
    stream.synchronize()
    got_relu = np.frombuffer(d_y.to_bytes(stream), dtype=np.float32).reshape(shape)
    np.testing.assert_allclose(got_relu, np.maximum(0.0, x_np), rtol=1e-5, atol=1e-5)

    # Sigmoid
    handle.activation_forward_f32(d_x, d_y, shape, mode="sigmoid", alpha=1.0, beta=0.0)
    stream.synchronize()
    got_sig = np.frombuffer(d_y.to_bytes(stream), dtype=np.float32).reshape(shape)
    expected_sig = 1.0 / (1.0 + np.exp(-x_np))
    np.testing.assert_allclose(got_sig, expected_sig, rtol=1e-5, atol=1e-5)

    # Softmax (accurate, channel -> axis=1)
    handle.softmax_forward_f32(d_x, d_y, shape, algo="accurate", mode="channel", alpha=1.0, beta=0.0)
    stream.synchronize()
    got_sm = np.frombuffer(d_y.to_bytes(stream), dtype=np.float32).reshape(shape)
    x_shift = x_np - np.max(x_np, axis=1, keepdims=True)
    exp_x = np.exp(x_shift)
    expected_sm = exp_x / np.sum(exp_x, axis=1, keepdims=True)
    np.testing.assert_allclose(got_sm, expected_sm, rtol=1e-5, atol=1e-5)
    np.testing.assert_allclose(np.sum(got_sm, axis=1), np.ones((2, 3, 4), dtype=np.float32), rtol=1e-5, atol=1e-5)


# ---------------------------------------------------------------------------
# 4. nv_rs.optix
# ---------------------------------------------------------------------------


def test_optix_context_properties_and_module():
    abi = nv_rs.optix.abi_version()
    assert isinstance(abi, int) and abi > 0

    ctx = nv_rs.core.Context(0)
    try:
        optix_ctx = nv_rs.optix.OptixContext(ctx, log_level=0, validation=False)
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "optix")
        return

    assert optix_ctx.abi_version == abi
    assert optix_ctx.rtcore_version() > 0
    assert optix_ctx.max_trace_depth() >= 1

    props = optix_ctx.device_properties()
    assert isinstance(props, dict)
    for key in ("abi_version", "rtcore_version", "max_trace_depth", "max_graph_depth"):
        assert key in props

    ptx_res = nv_rs.nvrtc.compile(
        r"""
        extern "C" __global__ void __raygen__noop() {
        }
        """,
        name="noop_rg.cu",
    )
    mod = optix_ctx.create_module(
        ptx_res.ptx.encode("utf-8"),
        params_name="params",
        params_size=0,
        payload_values=2,
        opt_level="default",
    )
    assert isinstance(mod.log, str)


# ---------------------------------------------------------------------------
# 5. nv_rs.npp
# ---------------------------------------------------------------------------


def test_npp_image_and_signal_ops_match_numpy():
    np = pytest.importorskip("numpy")
    try:
        ver = nv_rs.npp.version()
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "npp")
        return

    assert isinstance(ver, tuple) and len(ver) == 3
    assert ver[0] >= 11

    stream = nv_rs.core.Stream(0)
    npp_ctx = nv_rs.npp.NppStreamContext(stream)
    assert npp_ctx.device_id == 0

    h, w = 32, 48
    rng = np.random.default_rng(11)
    rgb = rng.integers(0, 256, size=(h, w, 3), dtype=np.uint8)

    d_rgb = nv_rs.core.DeviceBuffer.from_buffer(stream, rgb, shape=[h, w, 3], dtype="u8")
    d_gray = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros((h, w), dtype=np.uint8), shape=[h, w], dtype="u8"
    )
    d_ycbcr = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros((h, w, 3), dtype=np.uint8), shape=[h, w, 3], dtype="u8"
    )
    d_rgb_rt = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros((h, w, 3), dtype=np.uint8), shape=[h, w, 3], dtype="u8"
    )

    # RGB -> Gray
    nv_rs.npp.rgb_to_gray_8u(npp_ctx, d_rgb, d_gray, w, h)
    stream.synchronize()
    got_gray = np.frombuffer(d_gray.to_bytes(stream), dtype=np.uint8).reshape(h, w)
    expected_gray = (
        0.299 * rgb[:, :, 0].astype(np.float32)
        + 0.587 * rgb[:, :, 1].astype(np.float32)
        + 0.114 * rgb[:, :, 2].astype(np.float32)
    )
    assert float(np.max(np.abs(got_gray.astype(np.float32) - expected_gray))) <= 2.0

    # RGB -> YCbCr -> RGB roundtrip
    nv_rs.npp.rgb_to_ycbcr_8u(npp_ctx, d_rgb, d_ycbcr, w, h)
    nv_rs.npp.ycbcr_to_rgb_8u(npp_ctx, d_ycbcr, d_rgb_rt, w, h)
    stream.synchronize()
    got_rt = np.frombuffer(d_rgb_rt.to_bytes(stream), dtype=np.uint8).reshape(h, w, 3)
    mae_rt = float(np.mean(np.abs(got_rt.astype(np.float32) - rgb.astype(np.float32))))
    assert mae_rt < 4.0

    # Resize 8u C1
    dh, dw = h // 2, w // 2
    d_small = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros((dh, dw), dtype=np.uint8), shape=[dh, dw], dtype="u8"
    )
    nv_rs.npp.resize_8u_c1(npp_ctx, d_gray, (w, h), d_small, (dw, dh), interp="linear")
    stream.synchronize()
    got_small = np.frombuffer(d_small.to_bytes(stream), dtype=np.uint8).reshape(dh, dw)
    assert got_small.shape == (dh, dw)

    # Threshold 8u C1
    thresh_in = np.arange(h * w, dtype=np.uint8).reshape(h, w)
    d_tin = nv_rs.core.DeviceBuffer.from_buffer(stream, thresh_in, shape=[h, w], dtype="u8")
    d_tout = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros((h, w), dtype=np.uint8), shape=[h, w], dtype="u8"
    )
    nv_rs.npp.threshold_val_8u_c1(npp_ctx, d_tin, d_tout, (w, h), 128, 255)
    stream.synchronize()
    got_thresh = np.frombuffer(d_tout.to_bytes(stream), dtype=np.uint8).reshape(h, w)
    expected_thresh = np.where(thresh_in > 128, np.uint8(255), thresh_in)
    np.testing.assert_array_equal(got_thresh, expected_thresh)

    # 1D signal ops: set_32f, add_32f, mul_32f, sum_32f
    n = 1024
    a_np = np.linspace(1.0, 10.0, n, dtype=np.float32)
    b_np = np.empty(n, dtype=np.float32)
    b_np.fill(2.5)
    d_a = nv_rs.core.DeviceBuffer.from_buffer(stream, a_np, shape=[n], dtype="f32")
    d_b = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros(n, dtype=np.float32), shape=[n], dtype="f32"
    )
    d_add = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros(n, dtype=np.float32), shape=[n], dtype="f32"
    )
    d_mul = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros(n, dtype=np.float32), shape=[n], dtype="f32"
    )

    nv_rs.npp.set_32f(npp_ctx, 2.5, d_b)
    nv_rs.npp.add_32f(npp_ctx, d_a, d_b, d_add)
    nv_rs.npp.mul_32f(npp_ctx, d_a, d_b, d_mul)

    scratch_bytes = nv_rs.npp.sum_32f_scratch_bytes(npp_ctx, n)
    d_scratch = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros(max(1, scratch_bytes), dtype=np.uint8), shape=[max(1, scratch_bytes)], dtype="u8"
    )
    d_sum = nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros(1, dtype=np.float32), shape=[1], dtype="f32"
    )
    nv_rs.npp.sum_32f(npp_ctx, d_mul, d_sum, d_scratch)
    stream.synchronize()

    np.testing.assert_allclose(
        np.frombuffer(d_add.to_bytes(stream), dtype=np.float32), a_np + b_np, rtol=1e-5, atol=1e-5
    )
    np.testing.assert_allclose(
        np.frombuffer(d_mul.to_bytes(stream), dtype=np.float32), a_np * b_np, rtol=1e-5, atol=1e-5
    )
    got_sum = float(np.frombuffer(d_sum.to_bytes(stream), dtype=np.float32)[0])
    assert got_sum == pytest.approx(float(np.sum(a_np * b_np)), rel=1e-4)


# ---------------------------------------------------------------------------
# 6. nv_rs.ngx
# ---------------------------------------------------------------------------


def test_ngx_capabilities_and_queries():
    dlss = nv_rs.ngx.is_dlss_supported()
    assert isinstance(dlss, bool)

    req_ext = nv_rs.ngx.required_extensions()
    assert isinstance(req_ext, dict)

    feat_req = nv_rs.ngx.feature_requirements("super_sampling")
    assert isinstance(feat_req, dict)

    proj = nv_rs.ngx.validate_project_id(
        "12345678-1234-1234-1234-123456789abc",
        engine_name="nv-rs-test",
        engine_version="1.0.0",
    )
    assert isinstance(proj, dict)

    try:
        caps = nv_rs.ngx.NgxCapabilities()
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "ngx")
        return

    assert isinstance(caps.supported, bool)
    assert isinstance(caps.instance_extensions, list)
    assert isinstance(caps.device_extensions, list)
    assert caps.status_message is None or isinstance(caps.status_message, str)
    assert isinstance(caps.is_dlss_supported(), bool)

    inst_ext, dev_ext = caps.required_extensions()
    assert isinstance(inst_ext, list) and isinstance(dev_ext, list)
    assert isinstance(caps.supported_features(), list)
    assert isinstance(caps.dlss_quality_presets(), list)


# ---------------------------------------------------------------------------
# 7. nv_rs.cupti
# ---------------------------------------------------------------------------


def test_cupti_activity_and_subscriber():
    try:
        ver = nv_rs.cupti.version()
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "cupti")
        return

    assert isinstance(ver, int) and ver > 0
    cb_name = nv_rs.cupti.callback_name("runtime_api", 1)
    assert isinstance(cb_name, str) and len(cb_name) > 0

    np = pytest.importorskip("numpy")
    stream = nv_rs.core.Stream(0)

    try:
        act = nv_rs.cupti.CuptiActivity(device=0)
        act.enable("memcpy")
        buf = nv_rs.core.DeviceBuffer.from_buffer(
            stream, np.arange(64, dtype=np.float32), shape=[64], dtype="f32"
        )
        _ = buf.to_bytes(stream)
        stream.synchronize()
        act.flush(forced=True)
        records, dropped = act.drain_records()
        assert isinstance(records, list)
        assert isinstance(dropped, int)
        act.disable("memcpy")
        act.close()
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "cupti-activity")

    try:
        sub = nv_rs.cupti.CuptiSubscriber(device=0)
        sub.enable_domain("runtime_api", True)
        buf2 = nv_rs.core.DeviceBuffer.from_buffer(
            stream, np.arange(32, dtype=np.float32), shape=[32], dtype="f32"
        )
        _ = buf2.to_bytes(stream)
        stream.synchronize()
        events = sub.drain_events()
        assert isinstance(events, list)
        sub.check_errors()
        sub.close()
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "cupti-subscriber")


# ---------------------------------------------------------------------------
# 8. nv_rs.cuda_ext
# ---------------------------------------------------------------------------


def test_cuda_ext_managed_host_reg_uuid_and_ext_mem():
    uuid = nv_rs.cuda_ext.device_uuid(0)
    assert isinstance(uuid, bytes) and len(uuid) == 16

    sparse_ok = nv_rs.cuda_ext.sparse_arrays_supported(0)
    assert isinstance(sparse_ok, bool)

    try:
        luid, node_mask = nv_rs.cuda_ext.device_luid(0)
        assert isinstance(luid, bytes) and len(luid) == 8
        assert isinstance(node_mask, int)
    except nv_rs.errors.NvError as exc:
        # TCC/headless Linux GPUs may not expose a WDDM LUID.
        _skip_if_missing_runtime(exc, "cuda_ext.device_luid")

    payload = b"nv-rs-cuda-ext-managed-memory-test" * 16
    mbuf = nv_rs.cuda_ext.ManagedBuffer.zeroed(len(payload), device=0)
    assert mbuf.ptr != 0
    assert mbuf.nbytes == len(payload)
    assert mbuf.to_bytes() == b"\x00" * len(payload)
    mbuf.write_bytes(payload, offset=0)
    assert mbuf.to_bytes() == payload

    hreg = nv_rs.cuda_ext.HostRegistration(payload, map_to_device=True, device=0)
    assert hreg.nbytes == len(payload)
    assert hreg.to_bytes() == payload
    hreg.close()

    desc = nv_rs.cuda_ext.ExternalMemoryDesc("opaque_win32", 4096, dedicated=True)
    assert desc.handle_kind == "opaque_win32"
    assert desc.size == 4096
    assert desc.dedicated is True

    with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
        nv_rs.cuda_ext.ExternalMemoryDesc("invalid_kind", 4096)


def test_cuda_ext_green_contexts():
    try:
        groups = nv_rs.cuda_ext.split_sms(2, device=0)
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "cuda_ext.split_sms")
        return

    assert isinstance(groups, list) and len(groups) >= 1
    assert groups[0].sm_count >= 2
    assert groups[0].device_id == 0
    s = groups[0].new_stream()
    s.synchronize()


# ---------------------------------------------------------------------------
# 9. nv_rs.wgpu
# ---------------------------------------------------------------------------


def test_wgpu_probe_alignment_and_interop():
    supported = nv_rs.wgpu.is_supported()
    assert isinstance(supported, bool)

    adapters = nv_rs.wgpu.probe_adapters()
    assert isinstance(adapters, list)

    pitch = nv_rs.wgpu.aligned_row_pitch("rgba8unorm", 100)
    assert pitch >= 400 and pitch % 256 == 0
    total = nv_rs.wgpu.aligned_staging_bytes("rgba8unorm", 100, 10)
    assert total == pitch * 9 + 400

    if not supported:
        pytest.skip("wgpu Vulkan/CUDA interop not supported on host")

    try:
        ictx = nv_rs.wgpu.InteropContext(backend="auto", label="nv-rs-test")
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "wgpu.InteropContext")
        return

    assert isinstance(ictx.backend, str) and len(ictx.backend) > 0
    assert isinstance(ictx.cuda_device_id, int)
    ictx.verify()

    sbuf = ictx.create_buffer(1024, usage="storage|copy_src|copy_dst", label="shared")
    assert sbuf.size >= 1024

    sem = ictx.create_semaphore(initial_value=0)
    sem.signal_from_host(1)
    assert sem.completed_value() >= 1
    sem.wait_on_host(1, timeout_ns=1_000_000_000)


# ---------------------------------------------------------------------------
# 10. nv_rs.nccl (Linux-only)
# ---------------------------------------------------------------------------


def test_nccl_platform_behavior():
    is_linux = sys.platform.startswith("linux")
    assert nv_rs.nccl.SUPPORTED is is_linux

    if not is_linux:
        with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
            nv_rs.nccl.version()
        with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
            nv_rs.nccl.unique_id()
        with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
            nv_rs.nccl.group_start()
        return

    try:
        ver = nv_rs.nccl.version()
        uid = nv_rs.nccl.unique_id()
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "nccl")
        return

    assert isinstance(ver, int) and ver > 0
    assert isinstance(uid, bytes) and len(uid) > 0


# ---------------------------------------------------------------------------
# 11. nv_rs.cufile (Linux-only)
# ---------------------------------------------------------------------------


def test_cufile_platform_behavior(tmp_path):
    is_linux = sys.platform.startswith("linux")
    assert nv_rs.cufile.SUPPORTED is is_linux

    ctx = nv_rs.core.Context(0)
    if not is_linux:
        with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
            nv_rs.cufile.version()
        with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
            nv_rs.cufile.CufileDriver(ctx)
        with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
            nv_rs.cufile.CufileHandle(ctx, str(tmp_path / "dummy.bin"))
        return

    try:
        ver = nv_rs.cufile.version()
        drv = nv_rs.cufile.CufileDriver(ctx)
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "cufile")
        return

    assert isinstance(ver, int) and ver > 0
    assert isinstance(drv.properties(), dict)
    drv.close()


# ---------------------------------------------------------------------------
# 12. nv_rs.nvapi (Windows-only)
# ---------------------------------------------------------------------------


def test_nvapi_platform_behavior():
    is_windows = sys.platform == "win32"
    assert nv_rs.nvapi.SUPPORTED is is_windows

    if not is_windows:
        with pytest.raises(nv_rs.errors.NvInvalidArgumentError):
            nv_rs.nvapi.NvapiSession()
        return

    try:
        sess = nv_rs.nvapi.NvapiSession()
    except nv_rs.errors.NvError as exc:
        _skip_if_missing_runtime(exc, "nvapi")
        return

    assert isinstance(sess.interface_version(), str) and len(sess.interface_version()) > 0
    drv = sess.driver_version()
    assert isinstance(drv, dict)
    assert isinstance(drv["version"], int) and drv["version"] > 0
    assert isinstance(drv["branch"], str)
    assert sess.gpu_count() >= 1
    names = sess.gpu_names()
    assert isinstance(names, list) and len(names) >= 1
    assert isinstance(sess.gpu_info(0), dict)
    assert isinstance(sess.thermal_info(0), list)
    assert isinstance(sess.display_info(), list)
