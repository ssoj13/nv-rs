---
title: Core & Zero-Copy GPU Interop
description: Using nv_rs.core.Context, Stream, and DeviceBuffer with DLPack, __cuda_array_interface__ v3, and the Python Buffer Protocol.
---

`nv_rs.core` provides the fundamental GPU execution and memory primitives: `Context`, `Stream`, and `DeviceBuffer`.

## `Context` and `Stream`

```python
import nv_rs

# Create a CUDA context on device 0 and allocate a stream
ctx = nv_rs.core.Context(device=0)
stream = ctx.new_stream()
print(f"Device: {ctx.device_id}, Stream ptr: {stream.raw_ptr:#x}")

# Or create a stream directly on a device
stream = nv_rs.core.Stream(device=0)
stream.synchronize()
```

### API Summary

- `core.Context(device: int = 0)`
  - `new_stream() -> core.Stream`
  - `synchronize() -> None`
  - `device_id -> int`
- `core.Stream(device: int = 0)`
  - `synchronize() -> None`
  - `raw_ptr -> int`
  - `device_id -> int`

## `DeviceBuffer`

`core.DeviceBuffer` represents a typed, shaped allocation in GPU device memory.

```python
stream = nv_rs.core.Stream(0)

# Allocate an uninitialized buffer on the GPU
buf = nv_rs.core.DeviceBuffer.alloc(stream, shape=[1024], dtype="f32")
print(buf.ptr, buf.nbytes, buf.shape, buf.dtype, buf.device_id)
```

### Properties & Methods

- `ptr -> int` — raw CUDA device pointer.
- `nbytes -> int` — total size in bytes.
- `shape -> list[int]` — buffer dimensions.
- `dtype -> str` — element type string (e.g. `"u8"`, `"f32"`).
- `device_id -> int` — CUDA device ordinal.
- `is_contiguous -> bool` — `True` if the buffer has row-major (C-contiguous) strides (`is_c_contiguous()`).
- `to_bytes(stream: core.Stream) -> bytes` — copies device memory back to host `bytes`; checks `is_c_contiguous()` and raises an error on non-contiguous layouts.

## Buffer Protocol (`from_buffer` & `to_bytes`)

`DeviceBuffer.from_buffer` accepts any Python object implementing the buffer protocol (`bytes`, `bytearray`, `memoryview`, `numpy.ndarray`, etc.) and uploads it to the GPU:

```python
import numpy as np
import nv_rs

stream = nv_rs.core.Stream(0)
arr = np.arange(1024, dtype=np.float32)

d_buf = nv_rs.core.DeviceBuffer.from_buffer(
    stream, arr, shape=list(arr.shape), dtype="f32"
)

# Copy back to host and verify
raw = d_buf.to_bytes(stream)
out = np.frombuffer(raw, dtype=np.float32)
np.testing.assert_array_equal(out, arr)
```

You can also upload raw `bytes` or `memoryview` directly:

```python
payload = b"nv-rs-payload" * 128
d_in = nv_rs.core.DeviceBuffer.from_buffer(stream, payload, dtype="u8")
assert d_in.to_bytes(stream) == payload
```

## Zero-Copy DLPack & `__cuda_array_interface__` v3

`DeviceBuffer` implements both the **DLPack** protocol (`__dlpack__`, `__dlpack_device__`, `from_dlpack`) and **`__cuda_array_interface__` v3**, enabling zero-copy sharing with PyTorch, CuPy, JAX, and other CUDA-aware Python libraries.

```python
import torch
import cupy as cp
import nv_rs

stream = nv_rs.core.Stream(0)
t_in = torch.arange(256, device="cuda", dtype=torch.float32)

# Zero-copy import: PyTorch -> nv_rs.core.DeviceBuffer
buf = nv_rs.core.DeviceBuffer.from_dlpack(t_in)
assert buf.ptr == t_in.data_ptr()
assert buf.shape == [256]
assert buf.dtype == "f32"
assert buf.__cuda_array_interface__["data"][0] == t_in.data_ptr()

# Zero-copy export: nv_rs.core.DeviceBuffer -> PyTorch
t_out = torch.from_dlpack(buf)
assert t_out.data_ptr() == t_in.data_ptr()

# Zero-copy export: nv_rs.core.DeviceBuffer -> CuPy (via __cuda_array_interface__)
c_arr = cp.asarray(buf)
assert int(c_arr.data.ptr) == buf.ptr

stream.synchronize()
```

### Validation, Ownership & Stream Semantics

- **Checked shape & byte-size validation (`validate_shape_and_bytes`)**: validates non-negative dimensions, non-zero dtype `itemsize`, and checked element-count and byte-size multiplication, rejecting negative dimensions or `usize` overflows before allocating or importing memory.
- **Zero-UAF DLPack ownership (`DlManagedGuard`)**: `from_dlpack` wraps the imported `DLManagedTensor` in an RAII `DlManagedGuard`. If shape/dtype/device validation fails before ownership is transferred, dropping the guard invokes the producer's `deleter` exactly once without leaking or causing use-after-free.
- **`__dlpack__(stream=...)` synchronization**:
  - `stream=None` (default): synchronizes the buffer's producer stream before export.
  - `stream=-1`: DLPack no-sync sentinel — skips synchronization.
  - `stream=<int>` (CUDA stream handle `>= 0`) or `stream=<core.Stream>` (`PyStream`): skips synchronization when the consumer handle matches the producer stream; otherwise synchronizes the producer stream.
- **`__cuda_array_interface__` v3 exports**:
  - `"strides"`: exported in **bytes** (`element_stride * itemsize`, with checked multiplication) for non-empty shapes, or `None` for 0-D scalars.
  - `"stream"`: exported as the integer CUDA stream handle (`u64`) when a stream is associated with the buffer, or `None` otherwise.
