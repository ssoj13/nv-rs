# Handoff — nv-rs (2026-11-12)

## State

- Spikes S1a (OptiX runs a Rust raygen from cuda-oxide), S2 (Vulkan buffer shared by wgpu and
  CUDA with a timeline semaphore), S3 (one generator + one loader for nvml/optix/cufft): all PASS.
  Details and findings in PLAN.md §8; code under `spikes/`.
- **M2 in progress**: sys crates nvml, optix, cufft, cublas, cublaslt, cusparse, cusolver, nvjpeg, nvrtc built and loading (see COVERAGE.md). Next wave: cupti (extras/CUPTI), nvcuvid, nvenc (function table, Video Codec SDK), npp (multi-DLL design).
- **M1 done**: `crates/nv-loader`, `crates/nv-bindgen` (on vcv-rs), `xtask coverage`. Gate green
  (clippy `-D warnings`, T0 14/14, `cargo xtask coverage check`).
- vcv-rs detects the OptiX SDK (`vcv_rs::optix`, vcv-rs `18f6ed8`); nv-bindgen uses it.

## Next: M2 — sys crates

One crate per DLL family under `crates/`, each `build.rs` a single `nv_bindgen::generate(&SPEC)`
call and a loader function built on `nv_loader::open_first`. Start with the three S3 proved
(`nvml-sys`, `optix-sys`, `cufft-sys`), then fan out per library (disjoint directories, no
concurrent builds; one gate per wave). Each sys crate copies its `inventory.json` into
`coverage/evidence/<lib>/` and gets a `coverage/<lib>.toml` overlay.

Open items carried from the spikes:
- fork patch list: `ptx_asm!` input arity (16 -> 64), `#[device(export_name)]`, honour
  `#[constant(export_name)]` inside `#[cuda_module]`;
- reconcile D6 with the fork's committed `cuda-bindings/generated/`;
- `optix-sys` should re-use `cuda-bindings` for `CUcontext`/`CUstream`.

## Environment

- OptiX headers: `C:\ProgramData\NVIDIA Corporation\OptiX SDK 9.1.0` and `nv\optix-dev` (both 9.1, ABI 118).
- GPU: RTX 3080 Ti (sm_86), driver 621.24; CUDA 13.3. Host toolchain 1.98.0; spikes/device 1.99.1.
- The machine is shared: one build at a time, no looped test runs.
