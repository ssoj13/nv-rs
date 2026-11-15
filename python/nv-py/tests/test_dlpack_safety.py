"""Regression tests for PY-1, PY-2, PY-3, and PY-4 DLPack & CUDA interop safety."""

import ctypes
import pytest

import nv_rs
import nv_rs.core


# ---------------------------------------------------------------------------
# DLPack C ABI definitions via ctypes for synthetic testing
# ---------------------------------------------------------------------------
class DLDevice(ctypes.Structure):
    _fields_ = [("device_type", ctypes.c_int), ("device_id", ctypes.c_int)]


class DLDataType(ctypes.Structure):
    _fields_ = [
        ("code", ctypes.c_uint8),
        ("bits", ctypes.c_uint8),
        ("lanes", ctypes.c_uint16),
    ]


class DLTensor(ctypes.Structure):
    _fields_ = [
        ("data", ctypes.c_void_p),
        ("device", DLDevice),
        ("ndim", ctypes.c_int),
        ("dtype", DLDataType),
        ("shape", ctypes.POINTER(ctypes.c_int64)),
        ("strides", ctypes.POINTER(ctypes.c_int64)),
        ("byte_offset", ctypes.c_uint64),
    ]


class DLManagedTensor(ctypes.Structure):
    pass


DELETER_FUNC = ctypes.CFUNCTYPE(None, ctypes.POINTER(DLManagedTensor))

DLManagedTensor._fields_ = [
    ("dl_tensor", DLTensor),
    ("manager_ctx", ctypes.c_void_p),
    ("deleter", DELETER_FUNC),
]

PyCapsule_New = ctypes.pythonapi.PyCapsule_New
PyCapsule_New.restype = ctypes.py_object
PyCapsule_New.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_void_p]


_CAPSULE_KEEPALIVE = []


def make_synthetic_dltensor_capsule(
    device_type=1,  # 1 = kDLCPU, 2 = kDLCUDA
    device_id=0,
    dtype_code=1,  # uint
    dtype_bits=8,
    ndim=1,
    shape_list=None,
    strides_list=None,
    data_ptr=0,
    byte_offset=0,
    on_delete=None,
):
    if shape_list is None:
        shape_list = [16]
    shape_arr = (ctypes.c_int64 * len(shape_list))(*shape_list)
    strides_arr = None
    if strides_list is not None:
        strides_arr = (ctypes.c_int64 * len(strides_list))(*strides_list)

    managed = DLManagedTensor()
    managed.dl_tensor.data = ctypes.c_void_p(data_ptr)
    managed.dl_tensor.device = DLDevice(device_type, device_id)
    managed.dl_tensor.ndim = ndim
    managed.dl_tensor.dtype = DLDataType(dtype_code, dtype_bits, 1)
    managed.dl_tensor.shape = shape_arr
    managed.dl_tensor.strides = (
        strides_arr if strides_arr is not None else ctypes.POINTER(ctypes.c_int64)()
    )
    managed.dl_tensor.byte_offset = byte_offset
    managed.manager_ctx = None

    # Keep references alive
    managed._shape_keepalive = shape_arr
    managed._strides_keepalive = strides_arr

    managed_ptr = ctypes.pointer(managed)

    @DELETER_FUNC
    def deleter_cb(self_ptr):
        if on_delete:
            on_delete()

    managed.deleter = deleter_cb
    managed._deleter_keepalive = deleter_cb

    cap = PyCapsule_New(
        ctypes.cast(managed_ptr, ctypes.c_void_p), b"dltensor", None
    )
    # Keep ctypes structures alive for the test duration (PyCapsule has no __dict__).
    _CAPSULE_KEEPALIVE.append(managed)
    return cap


class SyntheticProducer:
    def __init__(self, **kwargs):
        self.kwargs = kwargs

    def __dlpack__(self, stream=None):
        return make_synthetic_dltensor_capsule(**self.kwargs)

    def __dlpack_device__(self):
        dev_type = self.kwargs.get("device_type", 1)
        dev_id = self.kwargs.get("device_id", 0)
        return (dev_type, dev_id)


def _get_cuda_stream():
    try:
        return nv_rs.core.Stream(0)
    except Exception as exc:
        pytest.skip(f"CUDA context/stream unavailable on host: {exc}")


# ---------------------------------------------------------------------------
# PY-1: CPU DLPack rejection reads metadata after deleter (use-after-free)
# ---------------------------------------------------------------------------
def test_py1_cpu_dlpack_rejection_no_uaf():
    """PY-1: CPU DLPack rejection must invoke deleter cleanly with NO read-after-free."""
    deleter_called = [0]

    def on_delete():
        deleter_called[0] += 1

    producer = SyntheticProducer(
        device_type=1,  # kDLCPU = 1
        device_id=0,
        shape_list=[32],
        on_delete=on_delete,
    )

    with pytest.raises(Exception) as exc_info:
        nv_rs.core.DeviceBuffer.from_dlpack(producer)

    err_msg = str(exc_info.value)
    assert "expected CUDA DLPack device" in err_msg or "type=2" in err_msg
    assert "got 1" in err_msg
    assert (
        deleter_called[0] == 1
    ), "Producer deleter must be invoked exactly once on rejection"


def test_py1_unsupported_dtype_rejection_cleans_up():
    """PY-1: Unsupported dtype in DLPack must clean up via deleter exactly once."""
    deleter_called = [0]

    def on_delete():
        deleter_called[0] += 1

    producer = SyntheticProducer(
        device_type=2,  # kDLCUDA
        device_id=0,
        dtype_code=99,  # invalid/unsupported dtype
        dtype_bits=8,
        shape_list=[16],
        data_ptr=0x1000,
        on_delete=on_delete,
    )

    with pytest.raises(Exception) as exc_info:
        nv_rs.core.DeviceBuffer.from_dlpack(producer)

    assert "unsupported DLDataType" in str(exc_info.value)
    assert deleter_called[0] == 1


# ---------------------------------------------------------------------------
# PY-2: Shape / dtype / byte-size mismatch checks
# ---------------------------------------------------------------------------
def test_py2_shape_dtype_byte_mismatch_rejected():
    """PY-2: from_buffer must reject byte lengths that disagree with shape/dtype."""
    stream = _get_cuda_stream()

    # Case 1: 1 byte uploaded with shape [1024] and dtype 'f32' (needs 4096 bytes)
    with pytest.raises(Exception) as exc_info:
        nv_rs.core.DeviceBuffer.from_buffer(
            stream, b"X", shape=[1024], dtype="f32"
        )
    assert "does not match" in str(exc_info.value)

    # Case 2: 3 bytes uploaded with dtype 'f32' (not a multiple of itemsize 4)
    with pytest.raises(Exception) as exc_info:
        nv_rs.core.DeviceBuffer.from_buffer(stream, b"123", dtype="f32")
    assert "not a multiple of dtype" in str(exc_info.value)

    # Case 3: Negative dimension in shape
    with pytest.raises(Exception) as exc_info:
        nv_rs.core.DeviceBuffer.from_buffer(
            stream, b"1234", shape=[-1], dtype="u8"
        )
    assert "negative dimension" in str(exc_info.value)

    # Case 4: Element count multiplication overflow
    with pytest.raises(Exception) as exc_info:
        nv_rs.core.DeviceBuffer.from_buffer(
            stream, b"1234", shape=[2**62, 2**62], dtype="u8"
        )
    assert "overflow" in str(exc_info.value)

    # Case 5: alloc with negative dimension
    with pytest.raises(Exception) as exc_info:
        nv_rs.core.DeviceBuffer.alloc(stream, [-10], dtype="f32")
    assert "negative dimension" in str(exc_info.value)

    # Case 6: alloc with overflow
    with pytest.raises(Exception) as exc_info:
        nv_rs.core.DeviceBuffer.alloc(stream, [2**62, 2**62], dtype="f32")
    assert "overflow" in str(exc_info.value)


def test_py2_valid_scalar_and_empty_buffers():
    """PY-2: Scalar (shape=[]) and zero-length buffers are accepted correctly."""
    stream = _get_cuda_stream()

    # Scalar buffer (4 bytes for f32)
    scalar_buf = nv_rs.core.DeviceBuffer.from_buffer(
        stream, b"\x00\x00\x80?", shape=[], dtype="f32"
    )
    assert scalar_buf.shape == []
    assert scalar_buf.nbytes == 4
    assert scalar_buf.dtype == "f32"
    assert scalar_buf.is_contiguous is True

    # Empty buffer (0 bytes)
    empty_buf = nv_rs.core.DeviceBuffer.from_buffer(stream, b"", dtype="f32")
    assert empty_buf.shape == [0]
    assert empty_buf.nbytes == 0


# ---------------------------------------------------------------------------
# PY-3: Strides preservation in DLPack and __cuda_array_interface__
# ---------------------------------------------------------------------------
def test_py3_strides_and_cuda_array_interface():
    """PY-3: __cuda_array_interface__ must export checked byte strides."""
    stream = _get_cuda_stream()

    # 2D contiguous buffer: shape [2, 3] of f32 (itemsize 4)
    # Row-major element strides: [3, 1] -> byte strides: [12, 4]
    data = b"\x00" * 24
    buf = nv_rs.core.DeviceBuffer.from_buffer(
        stream, data, shape=[2, 3], dtype="f32"
    )
    assert buf.strides == [3, 1]
    assert buf.is_contiguous is True

    cai = buf.__cuda_array_interface__
    assert "strides" in cai
    assert cai["strides"] == (12, 4)
    assert cai["shape"] == (2, 3)
    assert cai["typestr"] == "<f4"


def test_py3_non_contiguous_dlpack_import_and_to_bytes_rejection():
    """PY-3: Transposed non-contiguous DLPack tensor retains strides and rejects to_bytes."""
    stream = _get_cuda_stream()

    # Synthetic CUDA DLPack tensor with transposed layout:
    # shape [3, 2], strides [1, 3] (column-major / transposed)
    producer = SyntheticProducer(
        device_type=2,  # kDLCUDA
        device_id=stream.device_id,
        dtype_code=2,  # float
        dtype_bits=32,  # f32
        ndim=2,
        shape_list=[3, 2],
        strides_list=[1, 3],  # non-contiguous / transposed element strides
        data_ptr=0x2000,
    )

    buf = nv_rs.core.DeviceBuffer.from_dlpack(producer, stream=stream)
    assert buf.shape == [3, 2]
    assert buf.strides == [1, 3]
    assert buf.is_contiguous is False

    # CAI should export byte strides: [1 * 4, 3 * 4] = (4, 12)
    cai = buf.__cuda_array_interface__
    assert cai["strides"] == (4, 12)

    # to_bytes must reject non-contiguous buffers
    with pytest.raises(Exception) as exc_info:
        buf.to_bytes(stream)
    assert "contiguous" in str(exc_info.value).lower()


# ---------------------------------------------------------------------------
# PY-4: Asynchronous DLPack stream synchronization & metadata
# ---------------------------------------------------------------------------
def test_py4_dlpack_stream_synchronization():
    """PY-4: __dlpack__ honors stream argument including -1 sentinel and PyStream."""
    stream = _get_cuda_stream()
    buf = nv_rs.core.DeviceBuffer.alloc(stream, [64], dtype="f32")

    # stream=None
    cap_none = buf.__dlpack__(stream=None)
    assert cap_none is not None

    # stream=-1 (DLPack no-sync sentinel: must not fail with OverflowError on signed int)
    cap_nosync = buf.__dlpack__(stream=-1)
    assert cap_nosync is not None

    # stream integer handle
    cap_handle = buf.__dlpack__(stream=stream.raw_ptr)
    assert cap_handle is not None

    # stream PyStream instance
    cap_pystream = buf.__dlpack__(stream=stream)
    assert cap_pystream is not None

    # Verify stream getter
    assert buf.stream is not None
    assert buf.stream.raw_ptr == stream.raw_ptr

    # Verify __cuda_array_interface__ stream metadata
    cai = buf.__cuda_array_interface__
    assert "stream" in cai
    assert cai["stream"] == stream.raw_ptr
