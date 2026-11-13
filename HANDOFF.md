# Handoff — nv-rs (2026-11-13)

## State

- Spikes S1a (OptiX runs a Rust raygen from cuda-oxide), S2 (Vulkan buffer shared by wgpu and
  CUDA with a timeline semaphore), S3 (one generator + one loader for nvml/optix/cufft): all PASS.
  Details and findings in PLAN.md §8; code under `spikes/`.
- **M1 done**: `crates/nv-loader`, `crates/nv-bindgen` (on vcv-rs), `xtask coverage`.
- **M2 nearly done** — 18 sys crates build, load and pass their tests on this machine (counts in
  COVERAGE.md): nvml, optix, cufft, cublas, cublaslt, cusparse, cusolver, cusolvermg, nvjpeg,
  nvrtc, cupti, nvcuvid, nvenc, npp (11 DLLs), cudnn, nvapi, nvofa, ngx. Generator shapes:
  exports, split exports (NPP), query interface (NVAPI), function table (OptiX, NVENC, NVOFA),
  static link (NGX, PLAN D15). `nv-bindgen-fixture` locks generator bug classes.
- SDK discovery: `vcv_rs::nvsdk` (vcv-rs) finds cuDNN, Video Codec, Optical Flow, NVAPI, NGX and
  nvCOMP (PLAN D16); no per-SDK environment variables are needed.

## Next

1. Remaining M2: **nvCOMP** (not installed on this machine), **NCCL** and **cuFile** (Linux only:
   WSL2 for NCCL, native Linux / RunPod for cuFile).
2. M3 `cuda-ext`, then S1b (fork patches) and M4/M5 OptiX.

## Building

- SDKs unpacked outside the standard folders: set `NV_SDK_DIRS` (here
  `C:\projects\projects.rust.cg\nv`, holding OptiX, Video Codec, NVAPI, Streamline/NGX).
- `ngx-sys` needs `VULKAN_SDK`.
- Commit and push before running tests; one build at a time (shared machine). The full workspace
  test run takes over 10 minutes here — run it in the background.

## Open items

- fork patch list: `ptx_asm!` input arity (16 -> 64), `#[device(export_name)]`, honour
  `#[constant(export_name)]` inside `#[cuda_module]`;
- reconcile D6 with the fork's committed `cuda-bindings/generated/`;
- types shared between toolkit libraries (cuSOLVER pulls its own copy of cuBLAS/cuSPARSE handle
  types) should be re-used across sys crates, like CUDA types are from `cuda-bindings`;
- NVENC GUIDs are `static const` in the header and are not emitted by bindgen;
- NVOFA D3D11/D3D12/Vulkan variants and NGX's D3D11 helpers are not bound yet.

## Environment

- GPU: RTX 3080 Ti (sm_86), driver 621.24; CUDA 13.3; cuDNN 9.23 (CUDA 13.3 build). Host
  toolchain 1.98.0; spikes/device 1.99.1.
