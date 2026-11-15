---
title: Diagnostics & Error Handling
description: Inspecting the host environment with nv_rs.diag, tracking GPU memory leaks with TraceGuard, and catching nv_rs.errors.NvError exceptions.
---

`nv_rs.diag` provides runtime diagnostics, library probing, event tracing, and GPU memory leak detection. `nv_rs.errors` defines the exception hierarchy raised by all `nv_rs` submodules.

## Environment Inspection (`doctor` & `probe_libraries`)

Use `diag.doctor()` (or the formatted `nv_rs.format_doctor()` / `diag.format_doctor()`) to inspect the OS, architecture, active diagnostic level, Python interop capabilities, probed NVIDIA shared libraries, and global counters:

```python
import nv_rs

print(nv_rs.format_doctor())

info = nv_rs.diag.doctor()
print("Version:", info["version"])
print("Libraries:", list(info["libraries"].keys()))

libs = nv_rs.diag.probe_libraries()
for name, entry in libs.items():
    print(name, entry["present"], entry.get("path"))
```

## Custom DLL Search Directories (`add_dll_directory`)

If CUDA Toolkit or SDK libraries live outside standard system locations, register extra search directories before loading any `nv_rs` submodule:

```python
import nv_rs

nv_rs.add_dll_directory(r"C:\path\to\cuda\bin\x64")
# Also exposed on the diagnostics submodule:
nv_rs.diag.add_dll_directory(r"C:\path\to\cudnn\bin")
```

- `nv_rs.add_dll_directory(path)` accepts `str | os.PathLike[str]`, forwards the path to `nv_rs.diag.add_dll_directory` (which calls `nv_loader::add_search_dir`), and on Windows also calls `os.add_dll_directory` (retaining the handle) when the directory exists so transitive DLL dependencies resolve.
- `nv_rs.diag.add_dll_directory(path)` registers the directory directly in `nv_loader` without calling `os.add_dll_directory`.

## Diagnostic Levels, Stats & Events

Control verbosity and inspect recorded diagnostic events and counters:

```python
prev = nv_rs.diag.get_level()
nv_rs.diag.set_level("trace")  # "off" | "error" | "warn" | "info" | "trace"

stats = nv_rs.diag.stats()
# Contains: lib_loads, lib_load_failures, gpu_live_bytes, gpu_peak_bytes, errors

events = nv_rs.diag.events()
nv_rs.diag.clear_events()

nv_rs.diag.set_level(prev.lower())
```

## GPU Leak Detection with `nv_rs.trace()` / `TraceGuard`

`nv_rs.trace(level="trace", print_on_exit=True)` (also available as `nv_rs.diag.TraceGuard`) is a context manager that records events, errors, and GPU live-byte deltas across a block:

```python
import numpy as np
import nv_rs

stream = nv_rs.core.Stream(0)
arr = np.arange(1024, dtype=np.float32)

with nv_rs.trace("trace", print_on_exit=False) as tr:
    d_buf = nv_rs.core.DeviceBuffer.from_buffer(
        stream, arr, shape=list(arr.shape), dtype="f32"
    )
    del d_buf

assert tr.live_bytes_delta == 0
assert tr.error_delta == 0
```

`TraceGuard` exposes:

- `events -> list[dict[str, Any]]` — diagnostic events captured while the guard was active.
- `live_bytes_delta -> int` — net change in `gpu_live_bytes` between enter and exit (non-zero indicates a GPU allocation is still live).
- `error_delta -> int` — number of errors recorded while the guard was active.

## Error Hierarchy (`nv_rs.errors`)

All errors raised by `nv_rs` inherit from `nv_rs.errors.NvError`:

- `NvError(Exception)` — base exception for all `nv_rs` errors.
  - `NvLibraryNotFoundError(NvError)` — a required NVIDIA shared library could not be located or loaded.
  - `NvCudaError(NvError)` — a CUDA driver or runtime API call returned an error.
  - `NvInvalidArgumentError(NvError)` — invalid argument, shape, dtype, or state passed from Python.
  - `NvCompileError(NvError)` — NVRTC kernel compilation failed.

```python
import nv_rs

try:
    nv_rs.nvrtc.compile("__global__ void bad( {")
except nv_rs.errors.NvCompileError as exc:
    print("Compilation failed:", exc)
except nv_rs.errors.NvError as exc:
    print("Other nv_rs error:", exc)
```
