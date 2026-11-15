# nv-rs

An armed engineer checks his tooling before he draws.

`nv-rs` is a multi-crate Rust workspace (and Python package `nv_rs`) providing unified, safe,
and runtime-resilient access to the NVIDIA library stack — CUDA Toolkit libraries, OptiX,
display/management APIs, video, compression, DLSS/NGX, CUDA ↔ `wgpu` zero-copy interop, Rust
device code for OptiX via `cuda-oxide`, and zero-copy Python bindings — built directly on
[`NVIDIA/cuda-rust`](https://github.com/NVIDIA/cuda-rust) (`cuda-bindings`, `cuda-core`,
`cuda-async`).

Supported host targets: **Windows** (`x86_64`, `aarch64`) and **Linux** (`x86_64`, `aarch64`).

---

## Why `nv-rs` Exists

This project started while building `render-rs`, which needed direct **OptiX** access from Rust
to accelerate ray tracing. Instead of writing a narrow one-off wrapper, the goal became to solve
the NVIDIA-on-Rust gap once and for all across the entire stack.

### The Gap in the Ecosystem

- **`NVIDIA/cuda-rust` covers the core, not the stack:** `cuda-bindings`, `cuda-core`, and
  `cuda-async` provide a solid foundation for CUDA Driver/Runtime and `cutile`, but leave out
  OptiX, cuDNN, cuBLAS/cuBLASLt, cuSPARSE, cuSOLVER, cuFFT, cuRAND, NPP, nvJPEG, Video Codec
  (`nvenc`/`nvdec`), Optical Flow (`nvofa`), DLSS/NGX, `nvCOMP`, `cuFile` (GPUDirect Storage),
  `NCCL`, `NVRTC`, `CUPTI`, `NVML`, `NVAPI`, and CUDA ↔ `wgpu` interop.
- **Fragmented third-party crates don't compose:** standalone wrappers define incompatible
  `CUstream` / `CUcontext` types, hardcode SDK paths, or panic when a driver/DLL is absent.
- **OptiX device code in Rust needed compiler fixes:** compiling Rust kernels to OptiX PTX
  with `cuda-oxide` hit real gaps (`ptx_asm!` input arity for cooperative vectors,
  `#[export_name]` on device globals, SBT callable rooting, and 64-bit pointers across all
  address spaces).

### What `nv-rs` Solves

1. **One `CUstream` / `CUcontext` spine:** every crate builds directly on `NVIDIA/cuda-rust`,
   so buffers, streams, events, and contexts pass seamlessly between `cublas`, `optix`, `cudnn`,
   `nvenc`, `nv-wgpu`, and `cuda-core-ext`.
2. **Zero-copy graphics & Python interop:** `nv-wgpu` shares Vulkan (`OPAQUE_WIN32` /
   `OPAQUE_FD`) and DX12 resources with CUDA; `nv_rs` (PyO3 + `maturin`) exposes all modules to
   Python with DLPack, `__cuda_array_interface__`, and Buffer Protocol support.
3. **No committed proprietary headers (`nv-loader` + `nv-bindgen`):** every `*-sys` crate
   generates bindings at build time from your installed SDKs via
   [`vcv-rs`](https://github.com/ssoj13/vcv-rs) (or falls back to prebuilt bindings via
   `NV_RS_USE_PREBUILT=1`), and loads DLLs/SOs dynamically with typed `Result` errors — never a
   surprise panic.
4. **Upstream-first:** host crates and `nv_rs` already build directly against upstream
   [`NVIDIA/cuda-rust`](https://github.com/NVIDIA/cuda-rust). The `device/` workspace
   temporarily tracks [`cuda-rust_b`](https://github.com/ssoj13/cuda-rust_b) while upstream PRs
   [#1818](https://github.com/NVIDIA/cuda-rust/pull/1818),
   [#1819](https://github.com/NVIDIA/cuda-rust/pull/1819),
   [#1820](https://github.com/NVIDIA/cuda-rust/pull/1820), and
   [#1821](https://github.com/NVIDIA/cuda-rust/pull/1821) are under review.

For architecture decisions, current state, and symbol/GPU-test coverage numbers, see
[PLAN.md](PLAN.md) and [COVERAGE.md](COVERAGE.md).

---

## What Is in the Hangar

| Layer | Crates |
|---|---|
| **Infrastructure** | `nv-loader` (shared runtime loader, `exports_sys!` / `split_exports_sys!` / `query_interface_sys!`), `nv-bindgen` (declarative `LibSpec` generator + prebuilt fallback), `nv-bindgen-fixture` |
| **Interop & Extensions** | `cuda-core-ext` (external memory/semaphores, arrays, textures/surfaces, green contexts), `nv-wgpu` (zero-copy CUDA ↔ `wgpu` buffer/texture/semaphore interop) |
| **Linear Algebra & Math** | `cublas` (`cublas-sys`, `cublaslt-sys`), `cufft` (`cufft-sys`), `curand`, `cusolver` (`cusolver-sys`, `cusolvermg-sys`), `cusparse` (`cusparse-sys`) |
| **Imaging, Video & Graphics** | `npp` (`npp-sys`), `nvjpeg` (`nvjpeg-sys`), `nvdec` (`nvcuvid-sys`), `nvenc` (`nvenc-sys`), `nvofa` (`nvofa-sys`), `optix` (`optix-sys`), `ngx` (`ngx-sys` — statically linked per PLAN D15; DLSS-SR/RR safe wrapper supporting both `Init_with_ProjectID` and `ApplicationId` per D13) |
| **AI, Storage, Comms & Mgmt** | `cudnn` (`cudnn-sys`), `nvcomp` (`nvcomp-sys`), `cufile` (`cufile-sys`, Linux GPUDirect Storage), `nccl` (`nccl-sys`, Linux), `nvrtc` (`nvrtc-sys`), `cupti` (`cupti-sys`), `nvml` (`nvml-sys`), `nvapi` (`nvapi-sys`, Windows) |
| **Device & Tooling** | `device/optix-device`, `device/cuda-tex-device`, `xtask` (`coverage collect\|generate\|check`, `prebuilt`) |
| **Python & Docs** | `python/nv-py` (`nv_rs` via PyO3/`maturin`: 24 submodules, DLPack/`__cuda_array_interface__`/Buffer Protocol, `nv_rs.diag`), `docs/` (Astro + Starlight) |

---

## Provisioning SDKs (`bootstrap.py`)

A man who cannot provision his own bench cannot trust his own builds.
`bootstrap.py` is stdlib-only Python 3; running it bare prints help and exits cleanly (`0`):

```bash
python bootstrap.py          # show all commands and examples
python bootstrap.py i deps   # Linux sys pkgs + public SDKs into .sdk/ + manual SDK setup guide
python bootstrap.py d sys -y # install Linux build/runtime packages only (apt/dnf/pacman)
python bootstrap.py d s      # inspect detected SDKs (env vars, .sdk/, NV_SDK_DIRS, system paths)
python bootstrap.py d dl     # download & unpack public SDKs into .sdk/
python bootstrap.py s        # toolchain, git HEAD, and SDK summary
```

On a clean Ubuntu/Debian host, `d sys` / `i deps` installs `build-essential`, `clang`,
`libclang-dev`, `pkg-config`, `libssl-dev`, `xz-utils`, `curl`, `ca-certificates`, and
`libvulkan1` (pass `--nccl` to include `libnccl-dev`).

Public SDKs (`nvcomp`, `nvapi`, `cudnn`, `ngx`, `optix` headers, Linux `vulkan`, Linux `nccl`)
pull straight into `.sdk/` (git-ignored). Installer or NVIDIA Developer portal SDKs require
a one-time manual download/install:

| SDK | `d dl` | Where to get it | Env var or `.sdk/` layout |
|---|---|---|---|
| **CUDA Toolkit** (`13.x`) | Manual | [CUDA Downloads](https://developer.download.nvidia.com/compute/cuda/13.3.1/network_installers/cuda_13.3.1_windows_network.exe) | `CUDA_PATH` (default `/usr/local/cuda` or `%ProgramFiles%\NVIDIA GPU Computing Toolkit\CUDA\v13.x`) |
| **OptiX** (`9.x`) | Auto (`optix-dev`) / Installer | [OptiX SDK](https://developer.nvidia.com/rtx/ray-tracing/optix) | `OPTIX_ROOT` or `.sdk/optix` (`include/optix.h`) |
| **Vulkan SDK** | Auto (Linux) / Installer (Win) | [LunarG Win](https://sdk.lunarg.com/sdk/download/latest/windows/vulkan-sdk.exe) / [Linux](https://sdk.lunarg.com/sdk/download/latest/linux/vulkan-sdk.tar.xz) | `VULKAN_SDK` or `.sdk/vulkan` |
| **Video Codec SDK** (`nvenc`/`nvdec`) | Manual (login) | [Video Codec SDK](https://developer.nvidia.com/nvidia-video-codec-sdk/download) | `VIDEO_CODEC_SDK_ROOT` or unpack into `.sdk/videocodec/` (`Interface/nvEncodeAPI.h`, `Lib/`) |
| **Optical Flow SDK** (`nvofa`) | Manual (login) | [Optical Flow SDK](https://developer.nvidia.com/optical-flow-sdk) | `OPTICAL_FLOW_SDK_ROOT` or unpack into `.sdk/opticalflow/` (`NvOFInterface/nvOpticalFlowCommon.h`) |
| **nvCOMP** (`6.0+`) | Auto (`d dl`) | [nvCOMP redist](https://developer.download.nvidia.com/compute/nvcomp/redist/nvcomp/redistrib_6.0.0.json) | `NV_SDK_DIRS=.sdk` or `NVCOMP_ROOT` |
| **NVAPI** (Windows) | Auto (`d dl`) | [NVIDIA/nvapi](https://github.com/NVIDIA/nvapi) | `NV_SDK_DIRS=.sdk` or `NVAPI_ROOT` |
| **cuDNN** (`9.x`) | Auto (`d dl`) | [cuDNN redist](https://developer.download.nvidia.com/compute/cudnn/redist/cudnn/redistrib_9.28.1.json) | `NV_SDK_DIRS=.sdk` or `CUDNN_ROOT` |
| **NGX / DLSS** | Auto (`d dl`) | [NVIDIA/DLSS](https://github.com/NVIDIA/DLSS) | `NV_SDK_DIRS=.sdk` or `NGX_SDK_ROOT` |
| **NCCL** (Linux) | Auto / `apt` | [NVIDIA/nccl](https://github.com/NVIDIA/nccl) or `libnccl-dev` (`>= 2.31` for `nccl_device/core.h`) | `NCCL_ROOT` or `/usr/include/nccl.h` |

Point `NV_SDK_DIRS` at `.sdk` (or your own SDK root) when building crates that depend on
unpacked SDKs:

```powershell
$env:NV_SDK_DIRS = "$PWD\.sdk"
```

```bash
# Linux / WSL2 (use a separate target dir on ext4 for speed)
export NV_SDK_DIRS="$PWD/.sdk"
export LD_LIBRARY_PATH="/usr/local/cuda/lib64:/usr/lib/wsl/lib:${LD_LIBRARY_PATH:-}"
```

### Prebuilt Bindings Fallback

When SDK headers or `libclang` are not installed on a build host, `nv-bindgen` can fall back to
pre-generated bindings (`{name}_types.rs`, `{name}_shims.rs`, `{name}_inventory.json`):

- Set `NV_RS_USE_PREBUILT=1` to force prebuilt bindings.
- Set `NV_RS_PREBUILT_DIR` to point at a directory of prebuilt artifacts, or place them under
  `crates/<crate>/prebuilt/<target_triple>/`.
- Stage prebuilt artifacts from a machine with SDK headers installed via `cargo xtask prebuilt`.

---

## Building & Verification

Never ship an unverified build. Pin the host toolchain to `1.99.1` and treat warnings as
structural failures:

```bash
python bootstrap.py c        # cargo fmt --all --check + clippy --workspace --all-targets -- -D warnings
python bootstrap.py t        # workspace tests (-d for debug, -p <crate> to scope, --gpu for #[ignore])
python bootstrap.py b        # workspace build (--release by default; -d for debug)
```

Or invoke Cargo directly:

```bash
cargo +1.99.1 clippy --workspace --all-targets -- -D warnings
cargo +1.99.1 test --workspace --lib
```

GPU and runtime-driver tests are marked `#[ignore]`; run them with `-- --ignored` on a box
with an NVIDIA GPU.

---

## Conventions & Governance

- **License**: Apache-2.0.
- **NGX / DLSS Notice**: `ngx-sys` and `ngx` only bind the SDK. Applications that redistribute
  NVIDIA DLSS DLLs (`nvngx_dlss*.dll`) remain responsible for NVIDIA RTX SDK licence
  compliance and pre-release notification.
- **Contributing**: See [CONTRIBUTING.md](CONTRIBUTING.md) for DCO sign-off (`git commit -s`),
  CLA requirements, and Windows/WSL2 verification gates.
- **Security**: See [SECURITY.md](SECURITY.md) for vulnerability reporting.
