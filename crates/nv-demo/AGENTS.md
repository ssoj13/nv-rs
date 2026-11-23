# `nv-demo` Agent Guide

## What & Why
`crates/nv-demo/` is the flagship interactive RTX showcase binary (`nv-demo`) of `nv-rs` (`294+` unit tests). It exercises OptiX 9 hardware ray tracing (RT Cores), OptiX AI Denoiser & NGX DLSS SR/RR (Tensor Cores), cuFFT 2D diffraction bloom + CUDA tonemapping + zero-churn IAS/traffic/crowd streaming (CUDA Compute SMs), nvJPEG/nvENC export, atomic SDG JSON ground-truth export, and NVML/CUPTI telemetry inside a decoupled **60 Hz paced** `egui` / `egui-dock` / `wgpu` studio UI. See [plan1.md](../../plan1.md) for the active bug-hunt and acceleration roadmap.

## Callers & Dependencies
- **Callers**: standalone desktop binary (`cargo run -p nv-demo --release`).
- **Workspace crates**: `optix`, `nv-wgpu`, `nv-loader`, `nvrtc`, `nvml`, `cufft`, `nvjpeg`, `nvenc`, `cupti`, `ngx`, `cuda-core-ext`.
- **External CG crates**: `cuda-core`, `cuda-bindings`, `appbus`, `csg-core`, `csg-actors`, `csg-scene` (`wgpu`), `csg-device`, `csg-scenes` (`csg-rs` `7b8a085`, 23 crates, 84-slot palette, 15-class `GasLedger` `246/256`), `egui`/`egui-wgpu`/`egui-winit`/`egui_dock`/`egui-phosphor`, `wgpu`, `glam`, `bytemuck`, `serde`, `serde_json`.

## How It Works
```text
winit + 60 Hz paced egui UI (main.rs: PresentMode::Fifo + WaitUntil 16.667 ms, ui.rs)
  |-- appbus::AppBus (StateSlot<RenderStateSnapshot>, UiCommand, RenderControl, RenderEvent)
  v
Async OptiX Render Worker (renderer.rs, rig.stream, live optix::LogFn)
  1. GpuPassProfiler::begin_frame (3-slot CUevent ring, non-stalling cuEventQuery)
  2. CsgGpuHost::step_dynamic (TrafficStateSoA + SceneBuilt::crowd, Freeze Pose zero-motion snap)
  3. CsgGpuHost::sync -> TrafficOnly (dual pinned 256 KiB DMA + uncompacted Accel::instances_into UPDATE)
                       | FullRepack (persistent GrowDeviceBuffer + InstanceBuffer + in-place IAS BUILD)
  4. Uninterrupted GPU stream burst (ONE end-of-frame fence):
     [IAS/Traffic/Crowd] -> [OptiX RT (SBT 0..5, 640 B Params)] -> [OptiX Denoiser] -> [cuFFT Bloom] -> [Tonemap / HDR / DLSS-RR Pack]
  5. Single rig.stream.synchronize() -> GpuPassProfiler::poll_completed() -> FrameReady -> wgpu/DLSS/egui
```
- `src/csg_scene_host.rs`: 100% scene-agnostic `csg-scenes` host owning `Option<SceneBuilt>` (`Gundo`, `GundoOSM`, `Marina del Rey`, `SF House of AI`, **110-variant** `Inspector` incl. variant `109` 4-Archetype Crowd Showcase: Urban Human, `RoadSignStickman`, `QuadrupedDog`, 5-finger `SdgMannequin`), per-compound GAS cache, split SBT 4 (`hit_csg`) / SBT 5 (`hit_csg_prim`, 11-GAS actor bank) routing, persistent `GrowDeviceBuffer<T>` pools (incl. `prev_xforms_buf` for `Motion` AOV), dual `256 KiB` pinned `cuMemAllocHost_v2` staging buffers (`pinned_staging` + `pinned_prev_staging`), and atomic `export_sdg_json` via `csg_actors::extract_sdg_frame` + `CsgRayOccluder`.
- `src/profiler.rs`: lock-free fixed-capacity `[f32; 120]` `RingBuffer`s (`FrameTimings`), non-stalling 3-slot `GpuPassProfiler` (`cuEventQuery` only, never `cuEventSynchronize`) reporting **RT Cores (OptiX)**, **Tensor Cores (Denoiser / DLSS)**, and **CUDA Compute (IAS / cuFFT / Tonemap)** ms, plus NVML/CUPTI/NGX probing.
- `src/ui.rs`: narrow-dock-safe `dock_scroll` + adaptive `prop_row_widths`/`slider_control_widths` + truncated `equal_grid`/`ComboBox` layout, Apple Maps clickable WGS84 HUD pill, live 4-way `Road Mode` (`3D | C1 | Hybrid | F-Rep`), `Freeze Pose` & `Save SDG JSON` controls, `EnginePhase` / `FROZEN` status badges, master `Fractal` enable toggle, Hero Fractal full TRS (`T * R_z * R_y * R_x * S`), and per-engine `Performance` sparklines.

## Optimizations & Invariants
- **Zero Per-Frame VRAM Alloc/Free Churn**: `optix::InstanceBuffer` (`d_optix_instances` + `256 KiB` pinned staging), uncompacted persistent `Accel::instances_into` (`compact: false`, `allow_update: true`, in-place `OPTIX_BUILD_OPERATION_UPDATE` on dynamic-only frames and in-place buffer reuse on full repacks), and persistent `GrowDeviceBuffer<T>` pools across all CSG library, instance, `prev_xforms`, and AABB buffers yield `0 cuMemAlloc`, `0 cuMemFree`, and `0 Vec` allocs in steady state.
- **Single End-of-Frame Fence**: zero mid-frame `stream.synchronize()` or `cuEventSynchronize()` stalls between RT, Denoiser, cuFFT Bloom, and Tonemap; double-buffered `free_bufs` overlaps Frame `N+1` CUDA/OptiX with Frame `N` Vulkan DLSS/UI presentation.
- **Strict 640-Byte C ABI**: `LaunchParams` (`renderer.rs`) and `Params` (`shaders/raytracer.cu`) are locked by compile-time `assert!` and `static_assert(sizeof(Params) == 640)` + `__builtin_offsetof(Params, csg_prev_xforms) == 592` + `sdg_ray_count` at offset `632`, with `0` `double` tokens in `raytracer.cu`, true per-instance object optical flow (`csg_prev_xforms` + exact 3x3 affine inverse `csg_inv_xform_pt`), deterministic 4x RGSS AA zero-jitter AOV passes, and semantic IDs `0..19` (`18 = PEDESTRIAN`, `19 = ANIMAL`).
- **60 Hz UI Pacing & Safe Persistent IAS**: `main.rs` caps UI redraws to `60 Hz` (`PresentMode::Fifo` + `ControlFlow::WaitUntil(16.667 ms)` via `compute_next_ui_tick`); `ias_build_opts()` explicitly sets `compact: false` (`allow_update: true`) locked by `ias_build_opts_enables_update_and_disables_compaction`; `init_optix` installs a live `optix::LogFn` callback (`log_level = 3`, `NV_DEMO_OPTIX_LOG_LEVEL` override) with explicit `%LOCALAPPDATA%\NVIDIA\OptixCache` hint on `7999`.
- **Articulated Crowds & SDG Capture**: `CsgGpuHost` owns `SceneBuilt` directly, steps both `TrafficStateSoA`/legacy traffic (`!traffic.is_empty()`) and `SceneBuilt::crowd`, snaps `prev_xform = Some(inst.xform)` + `reset_crowd_motion_history()` + `traffic_dirty = true` on `Freeze Pose` `false -> true` transition for zero stale optical flow during clean multi-spp accumulation, and atomically exports `csg_actors::extract_sdg_frame` JSON keypoints/BBoxes (`Save SDG JSON` or auto-sidecar on frozen PNG/JPEG capture).
- **Large-Map Flight & Responsive UI**: progressive Shift ramp (`1x` <= 2s, `2^((t-2)/5)` up to `64x`), altitude-aware speed (`1x..=25x`), `RMB + Wheel` speed multiplier (`0.1..=64x`), and interactive fast-preview (`1 spp`, `<= 2` bounces + `150 ms` settle).

## Traps & Future Improvements
- **Trap (`step_dynamic` dirty flag & SoA traffic)**: `CsgGpuHost::step_dynamic` must set `traffic_dirty` (not `ias_dirty`) and check `!traffic.is_empty()` (not `!traffic.actors.is_empty()`, because `csg-citygen` OSM cities store vehicles in `TrafficStateSoA`), otherwise `sync()` either misses OSM traffic or forces a full 50k-instance repack instead of the `TrafficOnly` fast path.
- **Trap (`ias_build_opts().compact` & `Accel::custom`)**: `optix::BuildOptions::default()` sets `compact: true`, which `Accel::instances_into` rejects at runtime and which causes ~246 serial synchronous GPU compaction stalls in `Accel::custom` during scene/road switches. Always keep `compact: false` for dynamic IAS and GAS builds unless compaction is explicitly batched asynchronously.
- **Trap (`ias_state` `!show_classic`)**: never hash `sat_angle` or `fractal_transform` into `ias_state` when `!show_classic`, or `sat_orbit`/`auto_rotate` will force a full 50k-instance CSG IAS rebuild every frame (`renderer.rs:3679-3797`).
- **Trap (`rr_last_exposure` NaN sentinel)**: `(exposure - f32::NAN).abs() > 1e-5` is always `false`; use `Option<f32>` or `.is_finite()` checks (`main.rs:1770, 3468-3470`).
- **Trap (`SaveJpeg` & `max_accum == 0`)**: `0u32.saturating_sub(1) == 0`; guard `max_accum > 0` before clamping `accum_count` (`renderer.rs:2890`).
- **Trap (`CursorGrabController` one-shot command)**: `camera_egui::apply_cursor` emits `ViewportCommand::CursorGrab(Locked)` once on frame 0; suppressing it on `drag_accum_px < 4.0` without re-emitting later drops OS cursor lock for the entire drag and in persistent fly modes (`main.rs:1189-1348`).
- **Trap (`RoadTerrainTile` Tier 3B & `NEG_DET` normals)**: Tier 3B in `csg_eval.cuh:3069-3076` requires the `first_cell && t_box_in >= tmin - 1e-6f` guard to prevent curb shadow acne; `csg_prim_normal_to_compound` (`csg_eval.cuh:600-611`) must not double-negate mirrored primitive normals.
- **Trap (`Drop` CUDA context binding)**: always call `bind_to_thread()` before releasing CUDA allocations in `Drop` (`cuda_crowd.rs:251`, `renderer.rs:804`).
- **Trap (egui narrow dock inflation)**: never place unbounded `add_sized([w, h], Button::new(long_text))` or untruncated `ComboBox`es inside a `ScrollArea` without locking `inner_w` via `dock_scroll` and `.truncate()`, or sibling sections will clip off the right edge.
- **Future Improvements**:
  - Wire zero-copy CUDA traffic/crowd kernel writing `xform[12]` directly into both `csg_instances[0..D]` and `d_optix_instances[0..D].transform` in VRAM (`0 B` PCIe H2D/frame).
  - Replace host `Instant` DLSS timing with Vulkan `VkQueryPool` timestamp queries for sub-microsecond DLSS GPU timing.
