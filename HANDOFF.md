# Handoff — nv-rs (2026-11-13)

## State

- Spikes S1a (OptiX runs a Rust raygen from cuda-oxide), S2 (Vulkan buffer shared by wgpu and
  CUDA with a timeline semaphore), S3 (one generator + one loader for nvml/optix/cufft): all PASS.
  Details and findings in PLAN.md §8; code under `spikes/`.
- **M1 done**: `crates/nv-loader`, `crates/nv-bindgen` (on vcv-rs), `xtask coverage`.
- **M5 done** (whole OptiX 9.1 API, device + host, coverage gate `cargo xtask coverage optix`,
  GPU tests: 55 host + 15 device cases; PLAN.md §8). Device crates live in `device/` (toolchain
  1.99.1, `cargo oxide`); OptiX programs must build with `CUDA_OXIDE_NO_SHORT_PTR=1`
  (`device/optix-device-tests/.cargo/cuda-oxide.toml`).
- **S1b PASS** (Rust raygen calls `optixTrace`) and **M4 done**: `crates/optix` (context, GAS/IAS +
  compaction + byte budget + reusable scratch + refit, modules, program groups, pipeline, typed
  SBT, launch, HDR/AOV denoiser); 11 GPU tests on the S1b PTX. Details in PLAN.md §8.
- **M3 done**: `crates/cuda-ext` (external memory/semaphores, arrays, textures, surfaces, host
  registration, managed memory, UUID/LUID, green contexts); 7 GPU tests incl. a Vulkan round trip.
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
2. Fork `main` is `42cc04eda` (our OptiX patches + nvidia/main `7a7d160ea`); offer the patches
   upstream (FORK.md lists them).
3. M6 `nv-wgpu` on top of cuda-ext; M10 render-rs `pt-optix` on `optix` + `optix-device`.

## Building

- SDKs unpacked outside the standard folders: set `NV_SDK_DIRS` (here
  `C:\projects\projects.rust.cg\nv`, holding OptiX, Video Codec, NVAPI, Streamline/NGX).
- `ngx-sys` needs `VULKAN_SDK`.
- Commit and push before running tests; one build at a time (shared machine). The full workspace
  test run takes over 10 minutes here — run it in the background.

## Open items

- fork patch still to write: `#[device(export_name)]` (callables, M5);
- reconcile D6 with the fork's committed `cuda-bindings/generated/`;
- types shared between toolkit libraries (cuSOLVER pulls its own copy of cuBLAS/cuSPARSE handle
  types) should be re-used across sys crates, like CUDA types are from `cuda-bindings`;
- NVENC GUIDs are `static const` in the header and are not emitted by bindgen;
- NVOFA D3D11/D3D12/Vulkan variants and NGX's D3D11 helpers are not bound yet.

## Environment

- GPU: RTX 3080 Ti (sm_86), driver 621.24; CUDA 13.3; cuDNN 9.23 (CUDA 13.3 build). Host
  toolchain 1.98.0; spikes/device 1.99.1.

## Traps

- OptiX descriptors hold raw `*const c_char` entry names: the `CString`s must outlive the create
  call. `ProgramGroup::hit_group` once moved them into a list before the call (dangling names,
  "entryFunctionNameCH does not start with __closesthit__"); the GPU tests lock this.
- cuda-oxide device code: `ptx_asm!` `out` operands need declared variables (`_` is not a place).
- `cargo oxide` copies its backend into `~/.cargo/cuda-oxide`, which other sessions' builds keep
  open; set `CUDA_OXIDE_BACKEND` to a private copy instead of waiting or killing their processes.
- OptiX device code: index arrays with constants where possible; build OptiX programs with
  `CUDA_OXIDE_NO_SHORT_PTR=1` (the `ptr64` case guards it). Each test case compiles only its own
  programs out of the binary's PTX (`cases/common.rs` `subset_ptx`): OptiX validates every
  function of a module against that module's options.
- Device tests run with a private backend copy (`CUDA_OXIDE_BACKEND`), never the shared
  `~/.cargo/cuda-oxide`.
