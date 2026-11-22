# Repository guide

Verified against source on 2026-11-21. See [crate guidance](crates/AGENTS.md), [architecture diagrams](DIAGRAMS.md), and [build instructions](README.md#building--verification).

## Workspace boundaries

The root Cargo workspace contains `crates/*` and `xtask`. `device/` and `python/` are excluded and have separate build requirements. A successful root workspace check does not validate them.

## Dataflow

```text
SDK headers + LibSpec in each sys build.rs
  -> nv-bindgen::generate (vcv-rs discovery, bindgen, shims, inventory)
  -> OUT_DIR generated types/API/shims
  -> *-sys crate -> Rust wrapper -> application / Python extension
                         |
                         +-> cuda-core contexts, streams, device buffers

runtime DllSpec -> nv-loader::candidates -> open_first -> open_library
  -> cached API / cached LoadError -> native entry point / typed failure

wgpu Vulkan or DX12 allocation
  -> exported OS handle -> cuda-core-ext import
  -> nv-wgpu shared resource + timeline semaphore -> CUDA / graphics work
```

## Source owners

| Area | Responsibility |
| --- | --- |
| `crates/nv-bindgen` | Header discovery, generated bindings, return policies, prebuilt fallback, device PTX wrapper generation |
| `crates/nv-loader` | Runtime library search, shared loader macros, diagnostics |
| `crates/*-sys` | Per-library binding recipe and runtime entry-point adaptation |
| Wrapper crates | Validation, native status conversion, resource ownership |
| `crates/cuda-core-ext` | External memory/semaphores, arrays, textures, sparse and green-context extensions |
| `crates/nv-wgpu` | GPU identity matching, external memory and semaphore interop |
| `crates/nv-demo` | Interactive OptiX 9 renderer (SER, OMM, DLSS SR/RR, Aov/TemporalAov denoiser, analytical `csg-rs` `Gundo` & 28-asset `Inspector` scenes), camera controllers & multi-cassette `CameraRail`, `Viewport`/`FullWindow` MP4 SDR/PQ10 (180° shutter) & PNG/JPEG exports, and NVTX3/CUPTI profiling |
| `xtask` | Coverage inventories, prebuilt artifacts and runtime deployment tooling |
| `device/` | Separately compiled CUDA/OptiX device code |
| `python/` | Separate Python extension workspace |
| `docs/` | Astro/Starlight documentation site |

## Traps and verification

- Inspect each crate's Cargo `[lib].path`; entry points commonly use names such as `nv_loader.rs`, not `lib.rs`.
- Configure runtime search directories before the first sys call. The loader macros cache both success and failure in `OnceLock<Result<...>>`; later search-directory additions do not retry a failed initialization.
- Generated output belongs in `OUT_DIR`; change `LibSpec`, generator logic, or loader macros rather than patching generated files.
- NGX uses static linking; prebuilt Rust bindings do not supply its native libraries (`nvngx_dlss.dll` / `nvngx_dlssd.dll`). `crates/nv-demo/build.rs` stages both DLLs alongside the binary when present in `.sdk/ngx/`.
- `nv-demo` `LaunchParams` (`renderer.rs`) and `Params` (`raytracer.cu`) share a strict 576-byte C ABI checked by a compile-time `assert!`; keep both definitions synchronized when adding launch fields.
- `nv-demo` integrates `csg-rs` (`csg-core`, `csg-scene`, `csg-device`, `csg-urban`) at render time (`hit_csg` at SBT index 4, `#include "csg_eval.cuh"` via `csg_device::csg_eval_cuh_dir()` in `build.rs`, per-compound GAS caching in `csg_scene_host.rs`) for `Gundo` (El Segundo coastal refinery & highway) and 28-asset `Inspector` modes plus `SemanticId`/`InstanceId` AOVs.
- `nv-demo` supports `CaptureSource::{Viewport, FullWindow}` for MP4 (SDR H.264 & HDR10 PQ10) and PNG/JPEG export, plus cinematic video smoothness (`CameraDamper`, unified `1/60s * time_scale` fixed-step, toggleable multi-cassette `CameraRail` with C1 periodic Catmull-Rom + arc-length LUT + inward FPV banking, and 180° subframe shutter accumulation in `rig.color_f32`).
- `nv-demo` compiles `nvtx_shim.c` (`cfg(has_nvtx3)`) only when `<nvtx3/nvToolsExt.h>` is found under `CUDA_PATH`/`CUDA_HOME`/`CUDA_ROOT`, falling back to inline Rust no-ops otherwise.
- GPU integration tests marked `#[ignore]` need explicit execution on suitable hardware. Compile success alone does not verify driver behavior, stream ordering, callbacks, or interop.
- Check OS-gated branches separately: Windows-only NVAPI and Linux storage/collective paths cannot all execute on one host.
- Preserve CUDA context ownership and stream ordering when changing wrappers. Keep parent objects alive until dependent native objects and queued operations are finished.

Run `cargo clippy --workspace --all-targets` and `cargo test --workspace` for host changes. Inspect full diagnostics before suppressing warnings. Scope extra checks to the affected excluded workspace when relevant.
