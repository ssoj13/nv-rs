# nv-rs — Unified NVIDIA Stack for Rust & Python

*How trying to reach hardware RT cores from Rust turned into `nv-rs` — a 40+ crate ecosystem for CUDA, OptiX, DLSS, Video Codec, and zero-copy `wgpu` interop.*

It started as a weekend project. I wanted `render-rs` to be fast enough to be genuinely useful, so I sat down with my AI agents to figure out how to squeeze out more ray-tracing throughput. They took one look at the numbers and basically went: *"Shiver me timbers, matey, NVIDIA's hardware RT cores are the only way — anything else is a fool's voyage."* The catch? Those RT cores only speak OptiX, which lives firmly in C++/CUDA territory. From Rust, we didn't even have a boarding plank.

So I asked the obvious follow-up: *what would it actually take to bring OptiX to Rust properly?*

One wrapper wasn't going to cut it. A clean OptiX binding needs a solid CUDA foundation underneath it. Once you build that foundation, you realize that DLSS, hardware video encoding, math libraries, and graphics interop all run into the exact same problems: incompatible handles, brittle DLL discovery, and scattered FFI boilerplate.

But with AI agents in the crew, we fear no storm (or we're simply too stubborn to heed the warnings) — and so that weekend experiment grew into **`nv-rs`**: a modular Rust workspace of more than 40 crates, a `device` workspace for Rust GPU kernels, a `wgpu` interop layer, and a Python extension (`nv_rs`). With that foundation in place, I ported `render-rs` to OptiX, with `warpbro` next in line.

---

## What’s Inside the Stack

Instead of wrapping one API in isolation, `nv-rs` covers the full NVIDIA ecosystem under a single roof:

* **Ray Tracing & AI Upscaling:** OptiX 9, NGX (DLSS Super Resolution & Ray Reconstruction)
* **Math & Linear Algebra:** cuBLAS, cuFFT, cuSOLVER, cuSPARSE, cuRAND
* **AI & Compute:** cuDNN, NVRTC, NCCL
* **Imaging & Video:** NPP, nvJPEG, NVENC, NVDEC, Optical Flow (NVOFA)
* **Storage, Compression & Telemetry:** nvCOMP, cuFile, NVML, CUPTI, NVAPI
* **Graphics Interop & Demo:** Zero-copy `CUDA ↔ wgpu` (Vulkan & DX12 external memory/semaphores via `nv-wgpu` and `cuda-core-ext`), plus `nv-demo` — an interactive OptiX + `wgpu` + `egui` renderer where you can watch the RT cores work in real time.

---

## Why It’s Not Just Another `*-sys` Wrapper

Most Rust bindings to C/C++ GPU libraries stop at `bindgen` output. That works for a single library, but falls apart when you try to combine them in a real renderer or compute pipeline. Here is what `nv-rs` does differently:

### 1. One CUDA Context and Stream Across the Entire Stack
Standalone `*-sys` crates almost always define their own incompatible context and stream handles. In `nv-rs`, everything sits on top of `cuda-core` (from `NVIDIA/cuda-rust`) and `cuda-core-ext`. Buffers and streams move between OptiX, cuBLAS, NVENC, and `wgpu` without copies, unsafe handle transmutes, or type conflicts.

### 2. Zero Proprietary Headers in the Repository
NVIDIA’s SDK headers stay where they belong — on your machine. At build time, `nv-bindgen` discovers your local SDKs (via `vcv-rs`), generates clean bindings, shims, and return policies into `OUT_DIR`, and falls back to prebuilt bindings if an SDK isn’t installed.

### 3. Deterministic Runtime Loading (No Panics on Missing DLLs)
On Windows especially, NVIDIA libraries are scattered across `System32`, the driver store, and the CUDA Toolkit. Instead of crashing at startup or panicking on a missing symbol, `nv-loader` searches candidate paths deterministically, caches the result, and returns typed errors when a driver or library isn’t available.

### 4. Strict `unsafe` Discipline
FFI is inherently unsafe, so the workspace enforces strict guardrails: `unsafe_op_in_unsafe_fn` and `clippy::undocumented_unsafe_blocks` are set to `deny`. Every `unsafe` block must carry a `// SAFETY:` invariant proof, and the safe wrapper crates handle validation, native status conversion, and resource lifetimes.

### 5. Rust on the Device Side Too
Writing OptiX kernels shouldn’t force you back into C++ if you don’t want it to. `nv-rs` includes a dedicated `device/` workspace for compiling CUDA/OptiX device code in Rust via `cuda-oxide`. Pushing Rust into OptiX device code also uncovered real compiler gaps — such as cooperative vectors, device globals, and SBT callables — which led to upstream fixes submitted to `NVIDIA/cuda-rust` (PRs #1818–#1821).

### 6. Built for Real Workflows
The repository ships with `xtask` tooling for coverage inventories, prebuilt artifacts, and runtime DLL deployment, alongside a full Astro/Starlight documentation site. It runs on **Windows and Linux**, across both **`x86_64` and `aarch64`**.

📖 **Documentation:** [https://ssoj13.github.io/nv-rs/](https://ssoj13.github.io/nv-rs/)

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

A full-stack showcase (`crates/nv-demo/`, binary `nv-demo`, **278 unit tests** out of **517 passing workspace tests**) combining:

- **Pure Render-Time Analytical `csg-rs` Worlds & OptiX 9 Speedups**: dynamically discovers scenes via `csg-scenes` `SceneRegistry` (21-crate modular `csg-rs` Wave 7 architecture with `csg-ops`, socket-driven `csg-assemble`, 7 `kits/csg-kit-*` crates, and `kits/csg-catalog`) — **Gundo** (El Segundo coastal refinery & highway with 3D Frenet traffic), **GundoOSM** (real Chevron El Segundo Refinery with flange-snapped 3-tier `RivetedPipeBridge` hero pipeway, socket-wired catenary power lines, Road Work / Work Zone clusters, `RoadSurfaceMode::HybridC1Sdf` + grounded corner sidewalk aprons, Vista Del Mar + El Segundo Beach), **Marina del Rey** (Marina City Club crescent towers & yacht harbor), **San Francisco House of AI** (40 Boardman Pl with transmissive glass into the interior club/lofts/patio), and a **91-asset Studio Asset Inspector** (including showroom `VehicleKit` & `Robotaxi`, 3-tier `RivetedPipeBridge`, 4 Road Work / Work Zone assets, coastal `PineTree`/`DuneScrub`). Uses batched `DeviceBuffer<Aabb>` per-compound GAS caching, **split CSG SBT hit groups** (`hit_csg` at SBT 4 for general RPN vs 26-register / `48 B` stack `hit_csg_prim` at SBT 5 — a **679× stack reduction** from the original 32 KB, paired with zero-FP64 evaluation across all 12 analytical CSG primitives, `SKIP_IF_EMPTY`, and `PRIM_FLAG_IDENTITY`), unbiased **Russian Roulette** path termination (`bounce >= 2`), live **`CameraView::perspective`** / omnidirectional culling with a `120 m` shadow-caster guard in `CsgGpuHost`, and procedural micro-detail shading (corrugated steel cladding, welded tank plate courses, 3-octave ballast gravel, worn asphalt/concrete expansion joints, coastal dune sand ripples, wall-tangent bricks with flat tar/gravel roof membranes, planked wood, pool caustics/ocean waves, glass micro-warp, and warm interior glow) alongside the classic scratched metal floor, full **TRS (`Translate / Rotate / Scale`)** pixel-cone SDF fractals, dielectric glass satellites, emissive neon accent, and an optional **Opacity Micro-Map (OMM)** filigree halo benchmark.
- **Possess Body, Optical Sensor Projections, Hardware ViewScorer & 11 SDG AOVs**: `Ctrl + Left Click` **Pick-to-Possess** & **`Possess Car`** (live 0-lag moving `Robotaxi` lockstep, `[`/`]` mount cycling, free-look, `Auto-Level`, `Eject`), **OptiX 9 / WGPU / CPU `ViewProbeBackend`** (`Smart View` & `Smart Rail`), **4 optical sensor projections** (`Pinhole`, `Fisheye 180°`, `360° Equirect`, `360° Slit Cam`), **11 ground-truth SDG AOVs** (`Beauty`, `Albedo`, `Normal`, `Depth`, `Motion`, `SemanticId`, `InstanceId`, `WorldPos`, Euclidean `Range`, `Material` PBR, canonical MurmurHash3-32 `Cryptomatte`), plus large-map navigation (progressive `Shift` ramp doubling every 5 s up to `64×`, altitude-aware base speed, `RMB + Wheel` speed multiplier).
- **`appbus-rs` EventBus & Decoupled 60+ FPS UI**: typed `AppBus` (`UiCommand`, `RenderControl`, `RenderEvent`) with lock-free `StateSlot` latest-state coalescing and lost-wakeup-immune `RenderWake`, automatic **`1 spp` / `<= 2` bounce interactive fast-preview** during slider/camera drags with a `150 ms` settle reset to full-quality accumulation, zero redundant GPU buffer copies on intermediate cached UI frames (`viewport_tex_dirty`), and separate **`UI FPS`** vs **`RT FPS (RT ms)`** telemetry (`PREVIEW` badge when fast-preview is active).
- **AI Denoising, Upscaling, Capture & Cinema**: **Shader Execution Reordering (SER)** (`optixTraverse`/`optixReorder`/`optixInvoke`), **`Aov` / `TemporalAov`** AI denoising, **DLSS Super Resolution & Ray Reconstruction (DLSS-RR)** via `ngx`, interactive A/B split, **`nvrtc`** live `.cu` → PTX shader hot-reload, **`cufft`** 2D FFT optical diffraction bloom, **`nvjpeg`/`nvenc`** zero-copy PNG/JPEG & iPhone-compatible MP4 (`Viewport` or `FullWindow`, SDR H.264 & HDR10 PQ10 HEVC with fixed-60fps `time_scale` & 180° subframe shutter motion blur), a toggleable scene-driven multi-cassette **`CameraRail`** + on-demand exact-CSG **`Random Rail`** generator (periodic C1 Catmull-Rom / SQUAD, arc-length pacing, C1 cinema gimbal smoothing & `<= 14°` banking), zero-copy **`nv-wgpu`** (`SharedBuffer` → `wgpu::Texture`), an **`egui-wgpu`** docked viewport (`F` aim, `A` fit all, `RMB` fly), and **`nvml`/`cupti`/`ngx`/`NVTX3`** telemetry & Nsight markers:

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
