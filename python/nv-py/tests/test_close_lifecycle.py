"""`close()` / context-manager lifecycle of the nine native handle classes (ledger item `DPY:C20`).

Covers `CublasHandle`, `CufftPlan1d`, `CurandGenerator`, `CusparseHandle`, `CusolverDnHandle`,
`CudnnHandle`, `CompressionManager`, `NvjpegDecoder` and `NvjpegEncoder`: `close()` is idempotent,
any later use raises `NvInvalidArgumentError` ("closed") instead of touching released native
objects, and `with` closes on exit without suppressing the body's exception.

Every test needs a CUDA GPU; a missing optional NVIDIA library skips only on the typed
`NvLibraryNotFoundError`, so a lifecycle failure still fails.
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


def _f32(stream, count):
    np = pytest.importorskip("numpy")
    return nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros(count, dtype=np.float32), shape=[count], dtype="f32"
    )


def _u8(stream, count):
    np = pytest.importorskip("numpy")
    return nv_rs.core.DeviceBuffer.from_buffer(
        stream, np.zeros(count, dtype=np.uint8), shape=[count], dtype="u8"
    )


def _jpeg(stream):
    """A valid 16x16 JPEG, so `decode_info` succeeds while the decoder is open."""
    with nv_rs.nvjpeg.NvjpegEncoder(stream) as enc:
        return enc.encode(bytes(16 * 16 * 3), width=16, height=16)


# (id, library name, constructor, a call that reaches the native state and so must fail once the
# object is closed). Calls use valid arguments so that argument validation cannot mask the error.
CASES = [
    (
        "cublas",
        "cuBLAS",
        lambda s: nv_rs.cublas.CublasHandle(s),
        lambda obj, s: obj.gemm_f32(_f32(s, 1), _f32(s, 1), _f32(s, 1), 1, 1, 1),
    ),
    (
        "cufft",
        "cuFFT",
        lambda s: nv_rs.cufft.CufftPlan1d(s, 4, kind="c2c"),
        lambda obj, s: obj.execute(_f32(s, 8), _f32(s, 8)),
    ),
    (
        "curand",
        "cuRAND",
        lambda s: nv_rs.curand.CurandGenerator(s, seed=1),
        lambda obj, s: obj.fill_uniform_f32(_f32(s, 4)),
    ),
    (
        "cusparse",
        "cuSPARSE",
        lambda s: nv_rs.cusparse.CusparseHandle(s),
        lambda obj, s: obj.set_stream(s),
    ),
    (
        "cusolver",
        "cuSOLVER",
        lambda s: nv_rs.cusolver.CusolverDnHandle(s),
        lambda obj, s: obj.set_stream(s),
    ),
    (
        "cudnn",
        "cuDNN",
        lambda s: nv_rs.cudnn.CudnnHandle(s),
        lambda obj, s: obj.set_stream(s),
    ),
    (
        "nvcomp",
        "nvCOMP",
        lambda s: nv_rs.nvcomp.CompressionManager(s, format="lz4", chunk_size=16384),
        lambda obj, s: obj.compress(_u8(s, 16)),
    ),
    (
        "nvjpeg_decoder",
        "nvJPEG",
        lambda s: nv_rs.nvjpeg.NvjpegDecoder(s),
        lambda obj, s: obj.decode_info(_jpeg(s)),
    ),
    (
        "nvjpeg_encoder",
        "nvJPEG",
        lambda s: nv_rs.nvjpeg.NvjpegEncoder(s),
        lambda obj, s: obj.encode(bytes(16 * 16 * 3), width=16, height=16),
    ),
]

PARAMS = [pytest.param(c[1], c[2], c[3], id=c[0]) for c in CASES]


@pytest.mark.parametrize("component,make,use", PARAMS)
def test_close_is_idempotent(component, make, use):
    stream = _stream()
    obj = _or_skip(lambda: make(stream), component)
    assert obj.is_closed is False
    obj.close()
    assert obj.is_closed is True
    obj.close()  # second close is a no-op, not an error
    assert obj.is_closed is True


@pytest.mark.parametrize("component,make,use", PARAMS)
def test_use_after_close_raises_a_clear_error(component, make, use):
    stream = _stream()
    obj = _or_skip(lambda: make(stream), component)
    use(obj, stream)  # the call is valid while the object is open
    obj.close()
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError, match="closed"):
        use(obj, stream)


@pytest.mark.parametrize("component,make,use", PARAMS)
def test_with_block_closes_on_exit(component, make, use):
    stream = _stream()
    obj = _or_skip(lambda: make(stream), component)
    with obj as entered:
        assert entered is obj
        assert obj.is_closed is False
        use(entered, stream)
    assert obj.is_closed is True
    with pytest.raises(nv_rs.errors.NvInvalidArgumentError, match="closed"):
        with obj:  # re-entering a closed object is rejected
            pass


@pytest.mark.parametrize("component,make,use", PARAMS)
def test_with_block_propagates_the_body_exception_and_still_closes(component, make, use):
    stream = _stream()
    obj = _or_skip(lambda: make(stream), component)
    with pytest.raises(ZeroDivisionError):
        with obj:
            1 / 0
    assert obj.is_closed is True


def test_close_after_queued_work_waits_for_it():
    # `close()` drains the bound stream before freeing the generator, so work queued by the last
    # call cannot run against a destroyed handle; the output stays readable afterwards.
    np = pytest.importorskip("numpy")
    stream = _stream()
    gen = _or_skip(lambda: nv_rs.curand.CurandGenerator(stream, seed=3), "cuRAND")
    n = 1 << 20
    buf = nv_rs.core.DeviceBuffer.alloc(stream, [n], dtype="f32")
    gen.fill_uniform_f32(buf)
    gen.close()
    stream.synchronize()
    values = np.frombuffer(buf.to_bytes(stream), dtype=np.float32)
    assert values.shape == (n,)
    assert float(values.min()) > 0.0 and float(values.max()) <= 1.0


def test_nvenc_and_nvofa_close_are_idempotent_and_exit_propagates_nothing_on_success():
    # `close()` of the video classes drains their stream before releasing the session; with no
    # pending work it succeeds, twice, and `with` closes on exit.
    stream = _stream()
    try:
        enc = nv_rs.nvenc.NvencEncoder(stream, 64, 64)
    except nv_rs.errors.NvError as exc:
        pytest.skip(f"NVENC unavailable: {exc}")
    with enc:
        assert enc.is_closed is False
    assert enc.is_closed is True
    enc.close()
    try:
        ofa = nv_rs.nvofa.Nvofa(stream, 64, 64)
    except nv_rs.errors.NvError as exc:
        pytest.skip(f"NVOFA unavailable: {exc}")
    ofa.close()
    ofa.close()
    assert ofa.is_closed is True
