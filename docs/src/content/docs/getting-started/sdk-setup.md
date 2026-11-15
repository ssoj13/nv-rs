---
title: SDK Setup
description: Provisioning NVIDIA SDKs and system dependencies with bootstrap.py and environment variables.
sidebar:
  order: 2
---

`bootstrap.py` is a stdlib-only Python 3 helper for provisioning system dependencies, downloading public NVIDIA SDKs into `.sdk/` (git-ignored), and inspecting detected SDK paths. Running `python bootstrap.py` with no arguments prints the full command reference and exits cleanly.

## Provisioning with `bootstrap.py`

```bash
python bootstrap.py          # show all commands and examples
python bootstrap.py i deps   # Linux sys pkgs + public SDKs into .sdk/ + manual SDK setup guide
python bootstrap.py d sys -y # install Linux build/runtime packages only (apt/dnf/pacman)
python bootstrap.py d s      # inspect detected SDKs (env vars, .sdk/, NV_SDK_DIRS, system paths)
python bootstrap.py d dl     # download & unpack public SDKs into .sdk/
python bootstrap.py s        # toolchain, git HEAD, and SDK summary
```

On a clean Ubuntu/Debian host, `d sys` / `i deps` installs `build-essential`, `clang`, `libclang-dev`, `pkg-config`, `libssl-dev`, `xz-utils`, `curl`, `ca-certificates`, and `libvulkan1`. Pass `--nccl` to also install `libnccl-dev`.

Useful `d dl` flags:
- `--dir <path>` — target SDK root (defaults to `<repo>/.sdk`)
- `--only <sdk>` — filter to specific SDK(s), comma-separated or repeatable
- `--dry-run` — preview downloads without writing files
- `--allow-manual` — skip auth-gated SDKs with a notice instead of failing

## Public vs. Manual SDKs

Public SDKs pull directly into `.sdk/` via `d dl`. Installer-based or NVIDIA Developer portal SDKs require a one-time manual download or installation.

| SDK | Provisioning | Source | Environment Variable / `.sdk/` Layout |
|---|---|---|---|
| **CUDA Toolkit** (`13.x`) | Manual | [CUDA Downloads](https://developer.download.nvidia.com/compute/cuda/13.3.1/network_installers/cuda_13.3.1_windows_network.exe) | `CUDA_PATH` (default `/usr/local/cuda` or `%ProgramFiles%\NVIDIA GPU Computing Toolkit\CUDA\v13.x`) |
| **OptiX** (`9.x`) | Auto (`d dl` via `optix-dev`) / Installer | [OptiX SDK](https://developer.nvidia.com/rtx/ray-tracing/optix) | `OPTIX_ROOT` or `.sdk/optix` (`include/optix.h`) |
| **Vulkan SDK** | Auto (Linux) / Installer (Windows) | [LunarG Win](https://sdk.lunarg.com/sdk/download/latest/windows/vulkan-sdk.exe) / [Linux](https://sdk.lunarg.com/sdk/download/latest/linux/vulkan-sdk.tar.xz) | `VULKAN_SDK` or `.sdk/vulkan` |
| **Video Codec SDK** (`nvenc`/`nvdec`) | Manual (portal login) | [Video Codec SDK](https://developer.nvidia.com/nvidia-video-codec-sdk/download) | `VIDEO_CODEC_SDK_ROOT` or unpack into `.sdk/videocodec/` (`Interface/nvEncodeAPI.h`, `Lib/`) |
| **Optical Flow SDK** (`nvofa`) | Manual (portal login) | [Optical Flow SDK](https://developer.nvidia.com/optical-flow-sdk) | `OPTICAL_FLOW_SDK_ROOT` or unpack into `.sdk/opticalflow/` (`NvOFInterface/nvOpticalFlowCommon.h`) |
| **nvCOMP** (`6.0+`) | Auto (`d dl`) | [nvCOMP redist](https://developer.download.nvidia.com/compute/nvcomp/redist/nvcomp/redistrib_6.0.0.json) | `NV_SDK_DIRS=.sdk` or `NVCOMP_ROOT` |
| **NVAPI** (Windows) | Auto (`d dl`) | [NVIDIA/nvapi](https://github.com/NVIDIA/nvapi) | `NV_SDK_DIRS=.sdk` or `NVAPI_ROOT` |
| **cuDNN** (`9.x`) | Auto (`d dl`) | [cuDNN redist](https://developer.download.nvidia.com/compute/cudnn/redist/cudnn/redistrib_9.28.1.json) | `NV_SDK_DIRS=.sdk` or `CUDNN_ROOT` |
| **NGX / DLSS** | Auto (`d dl`) / Streamline | [NVIDIA/DLSS](https://github.com/NVIDIA/DLSS) | `NV_SDK_DIRS=.sdk`, `NGX_SDK_ROOT`, or `C:\Program Files\NVIDIA\Streamline\external\ngx-sdk` |
| **NCCL** (Linux) | Auto (`d dl`) / `apt` | [NVIDIA/nccl](https://github.com/NVIDIA/nccl) or `libnccl-dev` (`>= 2.31` for `nccl_device/core.h`) | `NCCL_ROOT` or `/usr/include/nccl.h` |

## Environment Variables

Point `NV_SDK_DIRS` at `.sdk` (or your custom SDK root) when building crates that depend on unpacked SDKs.

### Windows (PowerShell)

```powershell
$env:NV_SDK_DIRS = "$PWD\.sdk"
```

### Linux / WSL2

```bash
export NV_SDK_DIRS="$PWD/.sdk"
export LD_LIBRARY_PATH="/usr/local/cuda/lib64:/usr/lib/wsl/lib:${LD_LIBRARY_PATH:-}"
```

If you prefer individual overrides instead of `NV_SDK_DIRS`, set the per-SDK variables (`CUDA_PATH`, `OPTIX_ROOT`, `VULKAN_SDK`, `VIDEO_CODEC_SDK_ROOT`, `OPTICAL_FLOW_SDK_ROOT`, `NVCOMP_ROOT`, `NVAPI_ROOT`, `CUDNN_ROOT`, `NGX_SDK_ROOT`, `NCCL_ROOT`) to their respective SDK directories, then verify detection with `python bootstrap.py d s`.

## Verifying the Workspace

`nv-rs` pins the host Rust toolchain to `1.99.1` and treats all warnings as errors:

```bash
python bootstrap.py c            # cargo fmt --all --check + clippy --workspace --all-targets -- -D warnings
python bootstrap.py t            # workspace tests (-d for debug, -p <crate> to scope, --gpu for #[ignore])
python bootstrap.py b            # workspace build (--release by default; -d for debug)
python bootstrap.py b -p nv-demo # build the full-stack nv-demo showcase
```

If a build host lacks SDK headers or `libclang`, see [Prebuilt Bindings](/getting-started/prebuilt-bindings/).
