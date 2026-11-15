# nv-rs — Unified NVIDIA Stack for Rust & Python

*A driver panic is the mind-killer; check the `Result`, verify the SDK, and let the rays pass through.*

[![Documentation](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://ssoj13.github.io/nv-rs/) — **<https://ssoj13.github.io/nv-rs/>**

`nv-rs` is an independent, multi-crate **NVIDIA GPU stack for Rust** (with Python bindings `nv_rs`) built directly on top of [`NVIDIA/cuda-rust`](https://github.com/NVIDIA/cuda-rust) (`cuda-bindings`, `cuda-core`, `cuda-async`). Across **47 host crates**, **2 `cuda-oxide` device crates**, an interactive **OptiX 9 + `wgpu` showcase (`nv-demo`)**, and **25 Python submodules**, it provides panic-free runtime discovery, build-time binding generation without committed proprietary headers, safe Rust wrappers, and zero-copy GPU/graphics/Python interop.

Supported host targets: **Windows** (`x86_64`, `aarch64`) and **Linux** (`x86_64`, `aarch64`).

---

## Origin & Why `nv-rs` Exists

`nv-rs` started out of a practical bottleneck: making the **`render-rs`** renderer fast enough for **`squarebob`**. Achieving the required ray-tracing throughput meant using **NVIDIA OptiX**, which had no clean, composable Rust ecosystem story on top of modern CUDA Rust bindings.

Instead of writing another isolated, incompatible `*-sys` wrapper:

- **`NVIDIA/cuda-rust` provides the core, not the stack:** `cuda-bindings`, `cuda-core`, and `cuda-async` establish the foundation for CUDA Driver/Runtime and `cutile`, while leaving out OptiX, cuDNN, cuBLAS, cuFFT, cuSPARSE, cuSOLVER, cuRAND, NPP, nvJPEG, Video Codec (`nvenc`/`nvdec`), Optical Flow (`nvofa`), DLSS/NGX, `nvCOMP`, `cuFile` (GPUDirect Storage), `NCCL`, `NVRTC`, `CUPTI`, `NVML`, `NVAPI`, and CUDA ↔ `wgpu` interop.
- **Fragmented wrappers don't compose:** standalone crates define incompatible `CUstream` / `CUcontext` handles, hardcode SDK paths, commit vendor headers, or panic when a DLL/driver is absent.
- **Rust OptiX device code hit real compiler gaps:** compiling Rust kernels to OptiX PTX via `cuda-oxide` uncovered missing `ptx_asm!` input arity for cooperative vectors, `#[export_name]` on device globals, SBT callable rooting, and 64-bit pointer handling across address spaces.

### Architectural Pillars

1. **Single `CUstream` / `CUcontext` Spine:** every crate builds directly on `NVIDIA/cuda-rust`, so device buffers, streams, events, and contexts pass seamlessly between `optix`, `cublas`, `cudnn`, `cufft`, `nvenc`, `nv-wgpu`, and `cuda-core-ext`.
2. **No Committed Proprietary Headers (`nv-bindgen` + `vcv-rs`):** every `*-sys` crate generates bindings at build time from your installed SDKs via [`vcv-rs`](https://github.com/ssoj13/vcv-rs) (or falls back to prebuilt artifacts with `NV_RS_USE_PREBUILT=1`), emitting `// SAFETY:`-annotated shims that pass `#![deny(clippy::undocumented_unsafe_blocks)]`.
3. **Panic-Free Runtime Discovery & Transitive Loading (`nv-loader`):** twenty-plus NVIDIA libraries ship across `System32`, Windows `DriverStore`, CUDA Toolkit, and standalone SDKs. `nv-loader` replaces per-crate `libloading` boilerplate with deterministic discovery (`NV_DLL_DIRS`, `nv_loader::add_search_dir`, `exe_dir`, `CUDA_PATH`, standard Windows CUDA folders, `PATH`, and Linux runtime `root_var` resolution), Windows `LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS` transitive loading, and typed `LoadError` / `CallError` diagnostics — plus `cargo xtask stage-dlls` for deployment bundling.
4. **Zero-Copy Graphics & Python Interop:** `nv-wgpu` shares Vulkan (`OPAQUE_WIN32` / `OPAQUE_FD`) and DX12 resources with CUDA; `nv_rs` (PyO3 + `maturin`) exposes 25 submodules to Python with DLPack, `__cuda_array_interface__`, Buffer Protocol, `nv_rs.add_dll_directory`, and `nv_rs.diag`.
5. **Upstream-First:** all host crates and `nv_rs` already build directly against upstream [`NVIDIA/cuda-rust`](https://github.com/NVIDIA/cuda-rust). Only the `device/` workspace temporarily tracks [`cuda-rust_b`](https://github.com/ssoj13/cuda-rust_b) while upstream PRs [#1818](https://github.com/NVIDIA/cuda-rust/pull/1818), [#1819](https://github.com/NVIDIA/cuda-rust/pull/1819), [#1820](https://github.com/NVIDIA/cuda-rust/pull/1820), and [#1821](https://github.com/NVIDIA/cuda-rust/pull/1821) are under review.

---

## What Is in the Stack

| Layer | Crates |
|---|---|
| **Core Infrastructure** | `nv-loader` (shared runtime loader, `NV_DLL_DIRS`, `add_search_dir`, `open_library`, `exports_sys!` / `split_exports_sys!` / `query_interface_sys!`), `nv-bindgen` (declarative `LibSpec` + `// SAFETY:` shims + prebuilt fallback), `nv-bindgen-fixture` |
| **Interop & Extensions** | `cuda-core-ext` (external memory/semaphores, CUDA arrays, textures/surfaces, green contexts), `nv-wgpu` (zero-copy CUDA ↔ `wgpu` buffer/texture/semaphore interop) |
| **Math & Linear Algebra** | `cublas` (`cublas-sys`, `cublaslt-sys`), `cufft` (`cufft-sys`), `curand`, `cusolver` (`cusolver-sys`, `cusolvermg-sys`), `cusparse` (`cusparse-sys`) |
| **Ray Tracing, Imaging & Video** | `optix` (`optix-sys`), `ngx` (`ngx-sys`, DLSS-SR/RR via `Init_with_ProjectID` & `ApplicationId` + Streamline SDK discovery), `npp` (`npp-sys`), `nvjpeg` (`nvjpeg-sys`), `nvdec` (`nvcuvid-sys`), `nvenc` (`nvenc-sys`, `Mutex`-synchronized `EncodeSession`), `nvofa` (`nvofa-sys`) |
| **AI, Compression, Storage & Telemetry** | `cudnn` (`cudnn-sys`), `nvcomp` (`nvcomp-sys`), `cufile` (`cufile-sys`, Linux GPUDirect Storage), `nccl` (`nccl-sys`, Linux), `nvrtc` (`nvrtc-sys`), `cupti` (`cupti-sys`), `nvml` (`nvml-sys`), `nvapi` (`nvapi-sys`, Windows) |
| **Device, Demo & Tooling** | `device/optix-device`, `device/cuda-tex-device`, `crates/nv-demo` (OptiX 9 + `wgpu` + `egui` showcase), `xtask` (`coverage collect\|generate\|check\|optix`, `prebuilt`, `stage-dlls`) |
| **Python & Docs** | `python/nv-py` (`nv_rs`: 25 submodules, DLPack/`__cuda_array_interface__`/Buffer Protocol, `nv_rs.add_dll_directory`, `nv_rs.diag`), `docs/` (Astro + Starlight) |

See [PLAN.md](PLAN.md), [CHANGELOG.md](CHANGELOG.md), and [COVERAGE.md](COVERAGE.md) (generated by `cargo xtask coverage generate` from `coverage/` inventories and GPU test records) for architecture decisions and symbol-level coverage.

---

## Provisioning SDKs (`bootstrap.py`)

`bootstrap.py` is stdlib-only Python 3; running it bare prints help and exits cleanly (`0`):

```bash
python bootstrap.py          # show all commands and examples
python bootstrap.py i deps   # Linux sys pkgs + public SDKs into .sdk/ + manual SDK setup guide
python bootstrap.py d sys -y # install Linux build/runtime packages only (apt/dnf/pacman)
python bootstrap.py d s      # inspect detected SDKs (env vars, .sdk/, NV_SDK_DIRS, system paths)
python bootstrap.py d dl     # download & unpack public SDKs into .sdk/
python bootstrap.py s        # toolchain, git HEAD, and SDK summary
```

Public SDKs (`nvcomp`, `nvapi`, `cudnn`, `ngx`, `optix` headers, Linux `vulkan`, Linux `nccl`) unpack straight into `.sdk/` (git-ignored). Portal-gated SDKs require a one-time install or download:

| SDK | `d dl` | Source | Env var or `.sdk/` layout |
|---|---|---|---|
| **CUDA Toolkit** (`13.x`) | Manual | [CUDA Downloads](https://developer.download.nvidia.com/compute/cuda/13.3.1/network_installers/cuda_13.3.1_windows_network.exe) | `CUDA_PATH` (default `/usr/local/cuda` or `%ProgramFiles%\NVIDIA GPU Computing Toolkit\CUDA\v13.x`) |
| **OptiX** (`9.x`) | Auto (`optix-dev`) / Installer | [OptiX SDK](https://developer.nvidia.com/rtx/ray-tracing/optix) | `OPTIX_ROOT` or `.sdk/optix` (`include/optix.h`) |
| **Vulkan SDK** | Auto (Linux) / Installer (Win) | [LunarG Win](https://sdk.lunarg.com/sdk/download/latest/windows/vulkan-sdk.exe) / [Linux](https://sdk.lunarg.com/sdk/download/latest/linux/vulkan-sdk.tar.xz) | `VULKAN_SDK` or `.sdk/vulkan` |
| **Video Codec SDK** (`nvenc`/`nvdec`) | Manual (login) | [Video Codec SDK](https://developer.nvidia.com/nvidia-video-codec-sdk/download) | `VIDEO_CODEC_SDK_ROOT` or `.sdk/videocodec/` (`Interface/nvEncodeAPI.h`, `Lib/`) |
| **Optical Flow SDK** (`nvofa`) | Manual (login) | [Optical Flow SDK](https://developer.nvidia.com/optical-flow-sdk) | `OPTICAL_FLOW_SDK_ROOT` or `.sdk/opticalflow/` (`NvOFInterface/nvOpticalFlowCommon.h`) |
| **nvCOMP** (`6.0+`) | Auto (`d dl`) | [nvCOMP redist](https://developer.download.nvidia.com/compute/nvcomp/redist/nvcomp/redistrib_6.0.0.json) | `NV_SDK_DIRS=.sdk` or `NVCOMP_ROOT` |
| **NVAPI** (Windows) | Auto (`d dl`) | [NVIDIA/nvapi](https://github.com/NVIDIA/nvapi) | `NV_SDK_DIRS=.sdk` or `NVAPI_ROOT` |
| **cuDNN** (`9.x`) | Auto (`d dl`) | [cuDNN redist](https://developer.download.nvidia.com/compute/cudnn/redist/cudnn/redistrib_9.28.1.json) | `NV_SDK_DIRS=.sdk` or `CUDNN_ROOT` |
| **NGX / DLSS** | Auto (`d dl`) / Streamline | [NVIDIA/DLSS](https://github.com/NVIDIA/DLSS) | `NV_SDK_DIRS=.sdk`, `NGX_SDK_ROOT`, or `C:\Program Files\NVIDIA\Streamline\external\ngx-sdk` |
| **NCCL** (Linux) | Auto / `apt` | [NVIDIA/nccl](https://github.com/NVIDIA/nccl) or `libnccl-dev` (`>= 2.31`) | `NCCL_ROOT` or `/usr/include/nccl.h` |

Point `NV_SDK_DIRS` at `.sdk` when building:

```powershell
$env:NV_SDK_DIRS = "$PWD\.sdk"
```

```bash
export NV_SDK_DIRS="$PWD/.sdk"
export LD_LIBRARY_PATH="/usr/local/cuda/lib64:/usr/lib/wsl/lib:${LD_LIBRARY_PATH:-}"
```

### Runtime DLL / SO Deployment & Prebuilt Bindings

- **Runtime discovery:** set `NV_DLL_DIRS` (`;` on Windows, `:` on Linux) or call `nv_loader::add_search_dir(...)` / `nv_rs.add_dll_directory(...)` to add custom plugin or app directories.
- **Staging redistributables:** `cargo xtask stage-dlls --out <dir>` collects detected Toolkit and SDK shared libraries into a distribution directory.
- **Prebuilt bindings fallback:** set `NV_RS_USE_PREBUILT=1` (and optionally `NV_RS_PREBUILT_DIR`) or stage artifacts via `cargo xtask prebuilt` to build without local SDK headers or `libclang`.

---

## Building & Verification

Host toolchain is pinned in `rust-toolchain.toml` (`1.99.1`), with `#![deny(clippy::all)]`, `#![deny(clippy::undocumented_unsafe_blocks)]`, and `#![deny(unsafe_op_in_unsafe_fn)]`:

```bash
python bootstrap.py c            # cargo fmt --all --check + clippy --workspace --all-targets -- -D warnings
python bootstrap.py t            # workspace tests (-d for debug, -p <crate> to scope, --gpu for #[ignore])
python bootstrap.py b            # workspace build (--release by default; -d for debug)
python bootstrap.py b -p nv-demo # build the full-stack nv-demo showcase
```

GPU integration tests are marked `#[ignore]` and run via `-- --ignored` (or `python bootstrap.py t --gpu`) on an NVIDIA GPU host.

---

## Interactive RTX Demo (`crates/nv-demo/`)

A full-stack showcase (`crates/nv-demo/`, binary `nv-demo`) combining **OptiX 9** multi-instance IAS ray tracing (checkerboard floor, custom AABB fractal primitives — Mandelbulb, Mandelbox, KIFS Octahedron — dielectric glass sphere, and emissive neon accent), **`TemporalAov`** AI denoising with 5 AOVs and interactive A/B split, **`nvrtc`** live `.cu` → PTX shader hot-reload, **`cufft`** 2D FFT optical diffraction bloom, **`nvjpeg`/`nvenc`** zero-copy JPEG & H.264 export, zero-copy **`nv-wgpu`** (`SharedBuffer` → `wgpu::Texture`), an **`egui-wgpu`** docked viewport (`F` aim, `A` fit all, `RMB` fly), and **`nvml`/`cupti`/`ngx`** telemetry:

```powershell
$env:NV_SDK_DIRS = "$PWD\.sdk"   # or set OPTIX_ROOT / CUDA_PATH / NGX_SDK_ROOT
python bootstrap.py b -p nv-demo
cargo run -p nv-demo --release
```

---

## Conventions & Governance

- **Author**: Alex Khalyavin <joss13@gmail.com>
- **License**: Apache-2.0.
- **NGX / DLSS Notice**: `ngx-sys` and `ngx` only bind the SDK. Applications that redistribute NVIDIA DLSS DLLs (`nvngx_dlss*.dll`) remain responsible for NVIDIA RTX SDK licence compliance and pre-release notification.
- **Contributing**: See [CONTRIBUTING.md](CONTRIBUTING.md) for DCO sign-off (`git commit -s`), CLA requirements, and Windows/WSL2 verification gates.
- **Security**: See [SECURITY.md](SECURITY.md) for private vulnerability reporting.
