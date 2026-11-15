---
title: Python Bindings Overview & Installation
description: Building, installing, and using the nv_rs Python bindings with maturin, bootstrap.py, and the 24-submodule NVIDIA Rust stack.
---

`nv_rs` provides Python bindings (built with PyO3 and `maturin`) for the `nv-rs` NVIDIA Rust stack, exposing zero-copy GPU buffers, CUDA runtime/driver primitives, NVML, NVRTC, nvCOMP, nvJPEG, NVENC/NVDEC, cuBLAS/cuDNN/cuFFT, and built-in diagnostics.

## Requirements

- **Python**: `>= 3.10` (`requires-python = ">=3.10"` in `pyproject.toml`; `abi3` wheels target Python `>= 3.11`)
- **Build backend**: `maturin >= 1.7, < 2.0`
- **Optional dependencies**:
  - `numpy >= 1.24` (`pip install nv-rs[numpy]`)
  - `pytest >= 8.0`, `numpy >= 1.24` (`pip install nv-rs[dev]`)
- **NVIDIA Driver / CUDA**: Required at runtime for GPU operations. Because `nv-rs` loads NVIDIA shared libraries dynamically, you can import `nv_rs` and run diagnostics (`nv_rs.diag.doctor()`) even on machines without a full CUDA toolkit installed.

## Building and Installing with `bootstrap.py`

The repository root `bootstrap.py` script orchestrates `maturin` builds and `pytest` runs:

```bash
# Build and install in editable/development mode into the active Python environment
python bootstrap.py py dev

# Build release wheel(s)
python bootstrap.py py build

# Run the Python test suite (pytest)
python bootstrap.py py test
```

You can also invoke `maturin` directly from `python/nv-py`:

```bash
cd python/nv-py
maturin develop --release
```

## Environment Variables

| Variable | Purpose |
| :--- | :--- |
| `NV_SDK_DIRS` | Semicolon- or colon-separated list of directories searched when probing for NVIDIA SDK shared libraries (e.g., `nvcuda`, `nvrtc`, `nvcomp`, `nvjpeg`). |
| `NV_RS_USE_PREBUILT=1` | Instructs the build to use prebuilt native artifacts instead of compiling from scratch when supported. |

## Submodule Overview

`nv_rs` exposes the following submodules from `nv_rs._nv_rs`:

| Category | Submodules |
| :--- | :--- |
| **Core & Diagnostics** | `core`, `diag`, `errors` |
| **Compute & Math** | `cublas`, `cuda_ext`, `cudnn`, `cufft`, `curand`, `cusolver`, `cusparse`, `npp` |
| **Compilation & Telemetry** | `nvrtc`, `nvml`, `cupti` |
| **Compression & I/O** | `nvcomp`, `cufile` |
| **Media & Graphics** | `nvjpeg`, `nvenc`, `nvdec`, `nvofa`, `optix`, `ngx`, `nvapi`, `wgpu` |
| **Multi-GPU** | `nccl` |

Top-level helpers are also exported directly on `nv_rs`:

- `nv_rs.__version__` — package version string (`"0.1.0"`).
- `nv_rs.trace(level="trace", print_on_exit=True)` — context manager returning `nv_rs.diag.TraceGuard` for tracing and GPU memory leak detection.
- `nv_rs.format_doctor()` — returns a formatted multi-line diagnostic report of the host environment and loaded libraries.
