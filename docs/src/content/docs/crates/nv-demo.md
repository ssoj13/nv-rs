---
title: nv-demo (Interactive RTX Showcase)
description: Full-stack interactive OptiX 9 path tracer with pure-CSG worlds, 3-cascade cuFFT JONSWAP+TMA ocean, GPU traffic/crowd/camera FramePlans, Flight Lab/Player, 11 SDG AOVs + Motion + JSON export, SER, OMM, DLSS SR/RR, 60 Hz paced UI, cuFFT bloom, NVRTC hot-reload, NVENC/nvJPEG export, and NVML/CUPTI/NGX telemetry.
sidebar:
  order: 4
---

`nv-demo` (`crates/nv-demo/`, **262 unit tests** + **10 ignored GPU / OptiX / NVRTC / `cuFFT` Ocean / scene-census tests**) is the full-stack interactive RTX showcase for `nv-rs`. It wires `optix`, `nvrtc`, `cufft`, `nvjpeg`, `nvenc`, `nvml`, `cupti`, `ngx`, `cuda-core-ext`, `nv-wgpu`, `appbus-rs`, `camera-rs`, `flight-navigator-rs` (`e43bf05` + `fnav-csg` driving `csg_scene::RailRig`), and `csg-rs` (`csg-core`, `csg-actors`, `csg-traffic`, `csg-scene`, `csg-device`, `csg-scenes` at `f279da3`, `osm-rs` `301518e`) into a decoupled **60 Hz paced** `egui-wgpu` docked desktop studio with zero-copy CUDA ↔ `wgpu` presentation — **100% procedural / zero triangle-mesh assets**.

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

`CsgGpuHost` (`src/csg_scene_host.rs`) owns `Option<SceneBuilt>` directly from `csg_scenes::default_scene_registry()` (backed by the **27-crate** `csg-rs` workspace at `f279da3` with BLAKE3-256 `SceneDiskCache`, systemic 111-asset parametric `AssetRegistry`, analytical primitives incl. `RoadTerrainTile = 12` v2 `u16` CSR and `SWEEP_TILE = 14`, an **85-slot** `MaterialPalette`, **22** semantics, P42 per-instance OSM provenance incl. `FeatureTable::vegetation`, and `SceneModule::build_with_progress` streaming into `BuildEtaTracker` SMA-10 ETA):

- **Gundo & GundoOSM**: coastal El Segundo refinery, 3-tier `RivetedPipeBridge`, catenary power lines, Road Work zones, GPU `PlanKind::Traffic` v1 arterial traffic with VRAM Pedestrian AEB/Yield, Seawall Promenade & sidewalk biome crowds, analytical Highway Gothic OSM street-name signs, and Vista Del Mar beach.
- **Marina del Rey**: procedurally instanced 16-story Marina City Club crescent towers (split `glass_passes` thin glazing), furnished Marina Club balcony (`csg-kit-marina-furniture`), carved harbor basins & Oxford Retention Basin, smart-LOD yacht harbor, and promenade crowds.
- **San Francisco House of AI**: SoMa venue interior/exterior block with transmissive glazing and urban pedestrian crowds.
- **Studio Asset Inspector & Vendored DCC Attribute Editor (`111` parametric kit assets + Crowd Showcase)**: `sec_asset_inspector` pairs `csg-catalog`'s systemic `AssetRegistry` (`AssetParamSpec`/`AssetParamMap`/`AssetPreset`) with a vendored zero-new-dep DCC Attribute Editor (`src/attr_editor/`, adapted from `egui-widgets-rs/crates/sl/panels/src/attr_grid` — 3x3 category filter chips, search, presets, collapsible parameter groups, scrubby numeric labels, right-click `Reset`/`Copy`/`Paste`/`Reroll Seed`, `Ctrl+Click` reset, live parametric rebuild) alongside variant **`109` — `Actors: 4-Archetype Crowd Showcase`** (`8-GAS SINGLE_PRIM | IDENTITY` SBT 10 actor bank).
- **Robust BLAKE3 Disk Cache (`SceneDiskCache`)**: caches compiled analytical CSG bytecode + `WorldScene` + baked traffic/crowd/rails/viewpoints in `~/.cache/csg-rs/scenes` (framed BLAKE3-256 checksum, source-fingerprint auto-invalidation via `CSG_BUILD_FINGERPRINT`, atomic write + `2 GiB` LRU cap; excludes `wetness` and `time_of_day` so lighting/material changes stay `< 1 ms` via `refresh_materials`) with live `Disk Hit`/`Mem Hit`/`Built` status and `Clear Disk Cache` in `CSG World`.
- **Live `Road Mode`, `Ground Kerb` & `Smart Fill` Controls**: switch `Watertight3D`, `TangentPlaneC1`, `HybridC1Sdf`, and `FRepTile` (`RoadTerrainTile = 12`), toggle ground-level kerbs between `In-Tile F-Rep` and `Sweep Tile` (`GasClass::Sweep` / `SWEEP_TILE = 14`), and opt into 10-class `Smart Fill` OSM inference directly in `CSG World`.

### Real-Time 3-Cascade `cuFFT` Ocean & OptiX 9 Dielectric Water Optics

- **`CudaOcean` (`src/cuda_ocean.rs`)**: runs **3 seamless `256×256` JONSWAP + TMA spectral cascades** (`220 m` Pacific swell, `48 m` wind sea, `9 m` capillary ripples) on GPU via `crates/cufft::Plan::plan_many` (`FftType::C2R`, `batch = 18`) and headerless NVRTC kernels in `rig.stream` (zero per-frame allocations), packing `(height_m, slope_x, slope_z, jacobian_foam)` into VRAM with clean fallback to a 12-octave analytical Gerstner synthesis (`OceanMode::Analytical`). Interactive controls (`Mode`, `Wind Speed`, `Wind Dir`, `Choppiness`, `Swell`, `Foam`, `Water Clarity`) live in `Render & Floor -> Ocean Water` and persist in `~/.nv-rs/nv-demo.json`.
- **True OptiX 9 Dielectric Water Transport (`raytracer.cu`)**: enables `IOR = 1.333` Schlick Fresnel reflection vs. Snell refraction to carved seabed, beach sand, pilings, and yacht hulls; real per-channel **Beer-Lambert depth absorption** (`clarity`-scaled), backlit emerald wave-crest **SSS**, Jacobian whitecaps + shallow shoreline/piling contact foam, shallow seabed caustics, Toksvig/LEADR distance roughness filtering (zero horizon shimmer), and `<60 m` 4-step parallax heightfield relief — zero triangle meshes.

### Zero-Churn Persistent IAS & Split CSG SBT Hit Groups

- **Split CSG SBT offsets 8 / 10**: multi-op RPN compounds dispatch to `hit_csg` (`CSG_SBT_OFFSET = 8`, +1 shadow), while `compound_flags::SINGLE_PRIM` compounds (including all 11 actor bone GASes and F-Rep road tiles) dispatch to `hit_csg_prim` (`CSG_SBT_PRIM_OFFSET = 10`, +1 shadow, 26 registers / `48 B` stack — a **679× stack reduction**, zero FP64 across all 13 analytical primitives, `SKIP_IF_EMPTY`, `PRIM_FLAG_IDENTITY`).
- **Persistent Uncompacted IAS (`Accel::instances_into`)**: `ias_build_opts()` explicitly sets `allow_update: true` and `compact: false` (locked by unit test), enabling in-place `OPTIX_BUILD_OPERATION_UPDATE` on dynamic-only frames via dual reusable pinned `cuMemAllocHost` staging buffers (`instances` and `prev_xforms`; `2 MiB`, growing on demand to `16 MiB`) and grow-only `GrowDeviceBuffer<T>` VRAM pools (**0 steady-state `cuMemAlloc`/`cuMemFree` per frame**).
- **Classic Multi-Instance Stage**: optional scratched metal floor (`TriangleMesh` with `random_vertex_access: true`), full **TRS** pixel-cone SDF fractals (**Mandelbulb**, **Mandelbox**, **KIFS Octahedron** with 4-tap tetrahedral normals, 3-tap micro-AO, 4-step interior bisection), orbiting dielectric glass satellites, emissive neon accent ring, and dual-GAS **Opacity Micro-Map (`OMM Halo`)** (`Off | OMM | Any-Hit`).
- **Shader Execution Reordering (OptiX SER) & Russian Roulette**: `trace_primary` uses `optixTraverse` → 3-bit hit/instance coherence hint `optixReorder(hint, 3)` → `optixInvoke` alongside unbiased Russian Roulette (`bounce >= 2`) and live `CameraView::perspective` / omnidirectional culling (`120 m` shadow-caster guard).

---

## Synthetic Data Generation (SDG), Optical Flow & Sensors

- **Strict `752 B` C ABI (`LaunchParams` / `Params`)**: compile-time locked on both Rust (`renderer.rs`) and CUDA (`raytracer.cu`) with `pick_px` at **`428`**, `csg_prev_xforms` at **`592`**, `sdg_ray_count` at **`632`**, `classic_motion_count` / `classic_motion` at **`636`** / **`640`**, `pick_record` / `is_counters` / `hidden_semantics` / `csg_sem_partial` at **`648`** / **`656`** / **`664`** / **`672`**, `csg_instance_text` / `csg_sign_text_bytes` / `csg_sign_text_len` at **`680`** / **`688`** / **`696`**, `ocean_cascades_ptr` / `ocean_params_a` / `ocean_params_b` at **`704`** / **`720`** / **`736`**, crossfade source sky `env_from_lo` / `env_from_hi` at **`192`** / **`384`**, and `0 double` tokens.
- **True Dynamic Object Optical Flow (`Motion` AOV)**: uploads `PackedGpuScene::motion_prev_xforms` every dynamic frame (and updates in-VRAM via `cuda_traffic.rs` / `cuda_crowd.rs` scatter kernels) and computes exact 3x3 affine inverse (`csg_inv_xform_pt`) → `prev_world_pos` in `__raygen__render` for moving vehicles and walking crowd limbs.
- **11 Ground-Truth SDG AOVs**: `Beauty`, `Albedo`, `Normal`, `Depth`, `Motion`, `SemanticId` (`0..21` incl. `PEDESTRIAN = 18`, `ANIMAL = 19`, `VEGETATION = 20`, `TERRAIN_FEATURE = 21`), `InstanceId`, `WorldPos`, Euclidean LiDAR `Range`, `Material` PBR, and canonical MurmurHash3-32 Psyop `Cryptomatte`.
- **`Freeze Pose` & Atomic SDG JSON Export (`ExportSdgJson`)**: toggling **`Freeze Pose`** pauses traffic and `SceneBuilt::crowd` stepping, snaps `prev_xform = xform` + `reset_crowd_motion_history()` + `traffic_dirty = true` for zero stale optical flow during clean multi-spp accumulation, and enables 1-click **`Save SDG JSON`** (plus automatic companion `.json` sidecar export on frozen PNG/JPEG captures) via `csg_actors::extract_sdg_frame` + `CsgRayOccluder` written atomically (temp file + `flush`/`sync_all` + rename).
- **Possess Body, 4 Optical Projections & Hardware `ViewScorer`**: `Ctrl + Left Click` Pick-to-Possess and 1-click `Possess Car` (0-lag moving `Robotaxi` lockstep with `[`/`]` mount cycling), **4 sensor projections** (`Pinhole`, `Fisheye 180°`, `360° Equirect`, `360° Slit Cam`), and OptiX 9 (`__raygen__score_views`) / WGPU / CPU `ViewProbeBackend` driving `Smart View` and `Smart Rail`.

---

## Denoising, DLSS SR / Ray Reconstruction, 60 Hz UI & Export

- **Decoupled 60 Hz UI (`appbus-rs`) & Determinate Build ETA**: `main.rs` locks UI redraws to **60 Hz** (`wgpu::PresentMode::Fifo` + `Runner::about_to_wait` `ControlFlow::WaitUntil(16.667 ms)` via `compute_next_ui_tick`), communicating with the async OptiX worker over typed `AppBus` queues (`UiCommand`, `RenderControl`, `RenderEvent`) and lock-free `StateSlot` coalescing (`RenderStateSnapshot` + `SceneBuildProgress` with `BuildEtaTracker` **10-step moving average ETA**), automatic `1 spp` / `<= 2` bounce interactive fast-preview (`150 ms` settle window), 7-section `Camera & Stats` with horizontal bars, filled-area sparklines, and plain-English hover tooltips, clickable Apple Maps WGS84 coordinates, and live `EnginePhase` / `FROZEN` / `PossessToolbar` ADAS HUD badges.
- **4-Way `Upscale / AI` Selector**: switch live between **`Off`**, **`OptiX AI`** (`DenoiserModel::Aov` / `TemporalAov`), **`DLSS SR`** (NGX Vulkan Super Resolution), and **`DLSS RR`** (NGX Vulkan Ray Reconstruction / DLSSD via `pack_dlss_rr_aovs_f16` + WGSL ACES `HdrTonemapPipeline`), plus native **`Rgba16Float` scRGB HDR** viewport mode and interactive A/B split.
- **`nvrtc` Hot-Reload & `cufft` FFT Diffraction Bloom + Ocean**: live `raytracer.cu` → PTX reload without restarting, stream-bound 3-cascade `cuFFT` ocean synthesis (`cuda_ocean.rs`), and stream-bound batched 2D `R2C`/`C2R` N-blade aperture diffraction bloom (`fft_bloom.rs`).
- **`nvjpeg` & `nvenc` Zero-Copy Export (`Viewport` or `FullWindow`)**: GPU `nvjpeg` stills, lossless PNGs, and iPhone-compatible MP4 (`SDR` H.264 `avc1` BT.709 or `PQ10` HEVC Main10 `hvc1` BT.2020 SMPTE ST 2084 with `Live RT` 90 kHz `stts` or `Smooth 60` `time_scale` + `180°` subframe shutter motion blur) paired with unified **`flight-navigator-rs` Flight Lab & Flight Player** (curated rails, GPU-voxelized R0+R1 auto-planning via `GpuVoxelBaker`, natural-height Contact Sheet, Winamp-style playlists, and active-rail-preserving MP4 record).

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
