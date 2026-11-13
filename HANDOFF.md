# Handoff — nv-rs (2026-11-12)

## State

- Spikes S1a (OptiX runs a Rust raygen from cuda-oxide), S2 (Vulkan buffer shared by wgpu and
  CUDA with a timeline semaphore), S3 (one generator + one loader for nvml/optix/cufft): all PASS.
  Details and findings in PLAN.md §8; code under `spikes/`.
- **M1 done**: `crates/nv-loader`, `crates/nv-bindgen` (on vcv-rs), `xtask coverage`.
- **M2 in progress** — 12 sys crates build, load and pass their tests on this machine (counts in
  COVERAGE.md): nvml, optix, cufft, cublas, cublaslt, cusparse, cusolver, nvjpeg, nvrtc, cupti,
  nvcuvid, nvenc. `nv-bindgen-fixture` locks generator bug classes found in real headers.

## Next

1. **npp-sys** — NPP is eleven DLLs (nppc, nppial, nppicc, nppidei, nppif, nppig, nppim, nppist,
   nppisu, nppitc, npps). Needs one shared types module plus one loader/shim set per DLL; the
   `exports_sys!` macro assumes one DLL per crate, so nv-bindgen needs a "split by DLL" mode.
2. **cusolvermg-sys** (separate `cusolverMg64_*.dll`).
3. SDKs not installed on this machine (need the owner to install, then the same template):
   cuDNN, NCCL (Linux), nvCOMP, NVAPI, NGX/DLSS, Optical Flow (NVOFA), cuFile (Linux only).
4. Then M3 `cuda-ext` and M4 `optix` safe crates.

## Building

- CUDA Toolkit via `CUDA_PATH` (vcv-rs finds it otherwise); OptiX via `OPTIX_ROOT` or the default
  install; **`VIDEO_CODEC_SDK_ROOT` must point at the Video Codec SDK** for `nvcuvid-sys` and
  `nvenc-sys` (here: `C:\projects\projects.rust.cg\nv\Video_Codec_SDK_13.1.15`).
- Commit and push before running tests; one build at a time (shared machine).

## Open items

- fork patch list: `ptx_asm!` input arity (16 -> 64), `#[device(export_name)]`, honour
  `#[constant(export_name)]` inside `#[cuda_module]`;
- reconcile D6 with the fork's committed `cuda-bindings/generated/`;
- types shared between toolkit libraries (cuSOLVER pulls its own copy of cuBLAS/cuSPARSE handle
  types) should be re-used across sys crates, like CUDA types are from `cuda-bindings`;
- NVENC GUIDs are `static const` in the header and are not emitted by bindgen; the safe crate
  needs them generated another way.

## Environment

- OptiX headers: `C:\ProgramData\NVIDIA Corporation\OptiX SDK 9.1.0` and `nv\optix-dev` (both 9.1, ABI 118).
- GPU: RTX 3080 Ti (sm_86), driver 621.24; CUDA 13.3. Host toolchain 1.98.0; spikes/device 1.99.1.
