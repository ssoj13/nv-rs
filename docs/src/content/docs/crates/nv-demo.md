---
title: nv-demo (Interactive RTX Showcase)
description: Full-stack interactive OptiX 9 path tracer with pure-CSG worlds, articulated csg-actors crowds, 11 SDG AOVs + dynamic Motion optical flow + JSON export, SER, OMM, DLSS SR/RR, 60 Hz paced UI, cuFFT bloom, NVRTC hot-reload, NVTX3 markers, NVENC/nvJPEG export, and NVML/CUPTI/NGX telemetry.
sidebar:
  order: 4
---

`nv-demo` (`crates/nv-demo/`, **146 unit tests** (plus 5 ignored GPU/NVRTC tests)) is the full-stack interactive RTX showcase for `nv-rs`. It wires `optix`, `nvrtc`, `cufft`, `nvjpeg`, `nvenc`, `nvml`, `cupti`, `ngx`, `cuda-core-ext`, `nv-wgpu`, `appbus-rs`, and `csg-rs` (`csg-core`, `csg-actors`, `csg-scene`, `csg-device`, `csg-scenes` at `24b22f5`) into a decoupled **60 Hz paced** `egui-wgpu` docked desktop studio with zero-copy CUDA ↔ `wgpu` presentation.

## Building & Running

```powershell
$env:NV_SDK_DIRS = "$PWD\.sdk"   # or set OPTIX_ROOT / CUDA_PATH / NGX_SDK_ROOT
python bootstrap.py b -p nv-demo
cargo run -p nv-demo --release
```

On Windows, `ngx` (`ngx-sys`) is enabled by default (`default = ["ngx"]`) and automatically probes `NV_SDK_DIRS`, `NGX_SDK_ROOT`, or `C:\Program Files\NVIDIA\Streamline\external\ngx-sdk`. `init_optix` attaches a live `optix::LogFn` callback (`log_level = 3`, overridable via `NV_DEMO_OPTIX_LOG_LEVEL`) and reports an explicit `%LOCALAPPDATA%\NVIDIA\OptixCache` / `OPTIX_CACHE_MAXSIZE=0` hint if `OPTIX_ERROR_INTERNAL_ERROR (7999)` occurs.

---

## Pure-CSG Worlds, Articulated Crowds & OptiX 9 Pipeline

### Scene-Agnostic `csg-rs` World Host (`CsgGpuHost`)

`CsgGpuHost` (`src/csg_scene_host.rs`) owns `Option<SceneBuilt>` directly from `csg_scenes::default_scene_registry()` (backed by the **23-crate** `csg-rs` workspace at `24b22f5` with **13** analytical primitives `0..12`, an **84-slot** `MaterialPalette`, and a **15-class** `GasLedger` capped at **`246 / 256`** GASes):

- **Gundo & GundoOSM**: coastal El Segundo refinery, 3-tier `RivetedPipeBridge`, catenary power lines, Road Work zones, `TrafficStateSoA` arterial traffic, Seawall Promenade & sidewalk biome crowds, and Vista Del Mar beach.
- **Marina del Rey**: procedurally instanced 16-story Marina City Club crescent towers, furnished `4316 Marina City Dr — 3F Balcony` (`csg-kit-marina-furniture`), yacht harbor, and promenade crowds.
- **San Francisco House of AI**: 40 Boardman Pl interior/exterior SoMa block with transmissive glazing and urban pedestrian crowds.
- **Studio Asset Inspector (`110` variants)**: variants `0..108` (`VehicleKit`, `Robotaxi`, `RivetedPipeBridge`, Road Work, Marina furniture, foliage) plus variant **`109` — `Actors: 4-Archetype Crowd Showcase`** featuring Urban Human, comedic `RoadSignStickman`, `QuadrupedDog`, and 5-finger `SdgMannequin` compiled onto a shared **11-GAS `SINGLE_PRIM | IDENTITY`** SBT 10 actor bank.
- **Live 4-Way `Road Mode`**: switch `Watertight3D`, `TangentPlaneC1`, `HybridC1Sdf`, and `FRepTile` (`PrimKind::RoadTerrainTile = 12`) live in `CSG World` with quantized `(s, lat)` road micro-shading.

### Zero-Churn Persistent IAS & Split CSG SBT Hit Groups

- **Split CSG SBT offsets 8 / 10**: multi-op RPN compounds dispatch to `hit_csg` (`CSG_SBT_OFFSET = 8`, +1 shadow), while `compound_flags::SINGLE_PRIM` compounds (including all 11 actor bone GASes and F-Rep road tiles) dispatch to `hit_csg_prim` (`CSG_SBT_PRIM_OFFSET = 10`, +1 shadow, 26 registers / `48 B` stack — a **679× stack reduction**, zero FP64 across all 13 analytical primitives, `SKIP_IF_EMPTY`, `PRIM_FLAG_IDENTITY`).
- **Persistent Uncompacted IAS (`Accel::instances_into`)**: `ias_build_opts()` explicitly sets `allow_update: true` and `compact: false` (locked by unit test), enabling in-place `OPTIX_BUILD_OPERATION_UPDATE` on dynamic-only frames via dual reusable pinned `cuMemAllocHost` staging buffers (`instances` and `prev_xforms`; `2 MiB`, growing on demand to `16 MiB`) and grow-only `GrowDeviceBuffer<T>` VRAM pools (**0 steady-state `cuMemAlloc`/`cuMemFree` per frame**).
- **Classic Multi-Instance Stage**: optional scratched metal floor (`TriangleMesh` with `random_vertex_access: true`), full **TRS** pixel-cone SDF fractals (**Mandelbulb**, **Mandelbox**, **KIFS Octahedron** with 4-tap tetrahedral normals, 3-tap micro-AO, 4-step interior bisection), orbiting dielectric glass satellites, emissive neon accent ring, and dual-GAS **Opacity Micro-Map (`OMM Halo`)** (`Off | OMM | Any-Hit`).
- **Shader Execution Reordering (OptiX SER) & Russian Roulette**: `trace_primary` uses `optixTraverse` → 3-bit hit/instance coherence hint `optixReorder(hint, 3)` → `optixInvoke` alongside unbiased Russian Roulette (`bounce >= 2`) and live `CameraView::perspective` / omnidirectional culling (`120 m` shadow-caster guard).

---

## Synthetic Data Generation (SDG), Optical Flow & Sensors

- **Strict `672 B` C ABI (`LaunchParams` / `Params`)**: compile-time locked on both Rust (`renderer.rs`) and CUDA (`raytracer.cu`) with `pick_px` at **`428`**, `pick_record` / `is_counters` / `hidden_semantics` at **`648`** / **`656`** / **`664`**, `csg_prev_xforms` at offset **`592`**, `sdg_ray_count` at offset **`632`**, the classic object-motion table `classic_motion_count` / `classic_motion` at **`636`** / **`640`**, the crossfade source sky block `env_from_lo` / `env_from_hi` at **`192`** / **`384`**, and `0 double` tokens.
- **True Dynamic Object Optical Flow (`Motion` AOV)**: uploads `PackedGpuScene::motion_prev_xforms` every dynamic frame and computes exact 3x3 affine inverse (`csg_inv_xform_pt`) → `prev_world_pos` in `__raygen__render` for moving `TrafficStateSoA` vehicles and walking crowd limbs.
- **11 Ground-Truth SDG AOVs**: `Beauty`, `Albedo`, `Normal`, `Depth`, `Motion`, `SemanticId` (`0..19` incl. `PEDESTRIAN = 18` Cityscapes crimson and `ANIMAL = 19` amber), `InstanceId`, `WorldPos`, Euclidean LiDAR `Range`, `Material` PBR, and canonical MurmurHash3-32 Psyop `Cryptomatte`.
- **`Freeze Pose` & Atomic SDG JSON Export (`ExportSdgJson`)**: toggling **`Freeze Pose`** pauses traffic and `SceneBuilt::crowd` stepping, snaps `prev_xform = xform` + `reset_crowd_motion_history()` + `traffic_dirty = true` for zero stale optical flow during clean multi-spp accumulation, and enables 1-click **`Save SDG JSON`** (plus automatic companion `.json` sidecar export on frozen PNG/JPEG captures) via `csg_actors::extract_sdg_frame` + `CsgRayOccluder` written atomically (temp file + `flush`/`sync_all` + rename).
- **Possess Body, 4 Optical Projections & Hardware `ViewScorer`**: `Ctrl + Left Click` Pick-to-Possess and 1-click `Possess Car` (0-lag moving `Robotaxi` lockstep with `[`/`]` mount cycling), **4 sensor projections** (`Pinhole`, `Fisheye 180°`, `360° Equirect`, `360° Slit Cam`), and OptiX 9 (`__raygen__score_views`) / WGPU / CPU `ViewProbeBackend` driving `Smart View` and `Smart Rail`.

---

## Denoising, DLSS SR / Ray Reconstruction, 60 Hz UI & Export

- **Decoupled 60 Hz UI (`appbus-rs`)**: `main.rs` locks UI redraws to **60 Hz** (`wgpu::PresentMode::Fifo` + `Runner::about_to_wait` `ControlFlow::WaitUntil(16.667 ms)` via `compute_next_ui_tick`), communicating with the async OptiX worker over typed `AppBus` queues (`UiCommand`, `RenderControl`, `RenderEvent`) and lock-free `StateSlot` coalescing, with automatic `1 spp` / `<= 2` bounce interactive fast-preview (`150 ms` settle window), clickable Apple Maps WGS84 coordinates, and live `EnginePhase` / `FROZEN` HUD badges.
- **4-Way `Upscale / AI` Selector**: switch live between **`Off`**, **`OptiX AI`** (`DenoiserModel::Aov` / `TemporalAov`), **`DLSS SR`** (NGX Vulkan Super Resolution), and **`DLSS RR`** (NGX Vulkan Ray Reconstruction / DLSSD via `pack_dlss_rr_aovs_f16` + WGSL ACES `HdrTonemapPipeline`), plus native **`Rgba16Float` scRGB HDR** viewport mode and interactive A/B split.
- **`nvrtc` Hot-Reload & `cufft` FFT Diffraction Bloom**: live `raytracer.cu` → PTX reload without restarting, and stream-bound batched 2D `R2C`/`C2R` N-blade aperture diffraction bloom.
- **`nvjpeg` & `nvenc` Zero-Copy Export (`Viewport` or `FullWindow`)**: GPU `nvjpeg` stills, lossless PNGs, and iPhone-compatible MP4 (`SDR` H.264 `avc1` BT.709 or `PQ10` HEVC Main10 `hvc1` BT.2020 SMPTE ST 2084 with `Live RT` 90 kHz `stts` or `Smooth 60` `time_scale` + `180°` subframe shutter motion blur) paired with scene-driven multi-cassette **`CameraRail`** & exact-CSG **`Random Rail`**.

---

## Telemetry & Profiler

- **Non-Stalling 3-Slot `GpuPassProfiler`**: uses `cuEventQuery` (zero mid-frame stalls, single end-of-frame `stream.synchronize()`) to measure hardware **RT Cores (OptiX)**, **Tensor Cores (Denoiser / DLSS)**, and **CUDA Compute (IAS / cuFFT / Tonemap)** durations inside stack-allocated `[f32; 120]` `RingBuffer`s.
- **`nvml` / `cupti` / `ngx` / `NVTX3`**: live thermals, utilization, VRAM, power, clocks, graceful CUPTI activity/subscriber probing, NGX capability probing, and optional header-only NVTX3 ranges (`nvtx_shim.c`, `cfg(has_nvtx3)`).

---

## Camera Controls

- **`RMB` (Hold + Drag / WASD / QE)**: free-fly `FpsFly` navigation with progressive `Shift` ramp (`<= 2 s` flat, doubling every `5 s` up to `64×`), altitude-aware base speed (`1×..=25×`), and `RMB + Wheel` speed multiplier (`0.1×..=64×`).
- **`Home`**: frames CSG viewpoint `[0]` above DEM (or classic home bounds).
- **`F` / `A`**: aim at origin (`F`) or fit bounds (`A`).
- **`Ctrl + Left Click` / `[` / `]`**: Pick-to-Possess and cycle camera mounts on vehicles or landmarks.
