# `nv-demo` Comprehensive Bug Hunt & Radical Acceleration Plan (`plan1.md`)

**Audit Date**: 2026-11-23  
**Baselines Verified**: `nv-rs` `5a58bab` (`origin/main`), `csg-rs` `539f244` (`origin/main`)  
**Scope**: Full-stack audit of `crates/nv-demo/` (`shaders/raytracer.cu`, `src/renderer.rs`, `src/csg_scene_host.rs`, `src/cuda_crowd.rs`, `src/fft_bloom.rs`, `src/gpu_export.rs`, `src/sdg_export.rs`, `src/main.rs`, `src/ui.rs`, `src/app_events.rs`, `src/camera_rail.rs`, `src/demo_anim.rs`, `src/persist.rs`, `src/viewport_toolbar.rs`) and `csg-device/shaders/csg_eval.cuh` + `csg-core/src/eval.rs`.

---

## Executive Summary & PTX / Profiler Baseline Evidence

Direct static inspection of the compiled `raytracer.ptx` (`2.74 MB`, `63,759` lines) alongside a 3-track deep audit (GPU/OptiX 9, Host Renderer/Sync/Caching, UI/App Loop) revealed **10 P0 correctness bugs**, **12+ P1 high-impact performance & correctness defects**, and **a compounding 3x–8x+ FPS and 5x–20x scene-switch/export acceleration path**.

### Measured PTX & Kernel Evidence (`raytracer.ptx`)
- **Massive `.local` stack spills**:
  - `csg_trace_rpn_slow` (`csg_eval.cuh:4795`): `.local .align 16 .b8 __local_depot15[4224]` — **4.2 KiB per thread** allocated unconditionally even for small RPNs or shadow hits.
  - `__intersection__csg` (`raytracer.cu:2837`): `__local_depot28[112]` (`112 B` per thread — `CsgSceneView` 80 B + `CsgHit` 32 B spilled to local memory).
  - `__intersection__csg_prim` (`raytracer.cu:2879`): `__local_depot29[32]` (`32 B` per thread — even for single-primitive SBT-5 `BOX`!).
  - `__raygen__render` (`raytracer.cu:1757`): `36,361` PTX lines, **`25,534` `.f32` virtual registers**, `80 B` `.local` spill.
  - `csg_intersect_road_terrain_direct` (`csg_eval.cuh:2732`): **`4,206` `.f32` virtual registers** due to `__forceinline__` of `csg_eval_road_cell_height` (`2294`) 6+1 times.
- **Read-only cache underutilization**: `282` non-nc `ld.global` vs only `30` `ld.global.nc` (`__ldg`) across CSG traversal; `csg_materials` (`5.4 KiB`) fetched via global memory instead of `__constant__`.
- **Hardware RT Core underutilization**: A 50-primitive `PURE_UNION` compound (buildings, balconies, trees, vehicles) currently submits **1 giant AABB** to OptiX GAS, forcing SMs to linearly scan all 50+ primitives in software instead of hardware BVH traversal (`OptixBuildInputCustomPrimitiveArray` multi-AABB GAS).
- **Hidden host/GPU stalls**:
  - `ias_state` (`renderer.rs:3679-3797`) checks `sat_angle`/`fractal_transform` even when `!show_classic`, triggering a **full 50k-instance CSG IAS rebuild every frame** when `sat_orbit` or `auto_rotate` is enabled.
  - `TrafficOnly` (`csg_scene_host.rs:1267-1350`) repacks and hashes **all 10k–100k static instances on CPU every frame**.
  - `step_gpu_direct` (`cuda_crowd.rs:649-656`) issues a **mid-frame pageable D2H + `stream.synchronize()`** before IAS/RT.
  - `Accel::custom` (`csg_scene_host.rs:779, 1549`) inherits `compact: true`, causing **~246 serial synchronous GPU compaction stalls** per scene/road switch.

### Expected Cumulative Gains
| Optimization Pillar | Target Bottleneck | Expected Impact |
| --- | --- | --- |
| Multi-AABB Custom Primitive GAS for `PURE_UNION` | SM linear scan of 10–120 prim compounds | **2x–6x RT speedup** on dense buildings/marina/vehicles (`0` extra GAS count) |
| Eliminate `.local` spills + `__ldg` + `__constant__` palette + AABB pre-check | Local memory thrash & low SM occupancy | **1.3x–2.0x RT speedup** |
| Dedicated Shadow SBT + Typed Payloads + `DISABLE_ANYHIT` + SER `mat_id` fix | Shadow ray overhead & warp divergence | **1.25x–1.55x RT speedup** |
| AOV Mode Early-Out (skip shading/shadows/16-bounce loop) | `aov_mode != 0` wasted shading | **2x–4x AOV FPS** |
| Half-Res 7-Smooth `FftBloomPass` + Pre-Normalized PSF | Full-res `2048²`/`4096²` cuFFT (`~302 MiB` @ 4K) | **4x–7x bloom speedup** (`1.15x–1.35x` total FPS @ 4K) |
| Fix `!show_classic` IAS bug + Static-Tail Split + Async GPU Crowd | Full 50k IAS rebuild & mid-frame GPU sync | **Eliminates 10–30 ms/frame CPU+sync stalls** |
| `compact: false` on GAS + `SceneBuilt` LRU + Disk PTX / `Nvjpeg` Cache | Scene switches, cold boot, JPEG export | **5x–20x faster scene switches**, **2x–4x cold boot**, **10x–30x JPEG save** |

---

## Findings Ledger (Verified `file:line`)

### P0 — Critical Correctness & Stability Bugs (10 Items)

1. **`csg_eval.cuh:3069-3076` — `RoadTerrainTile` Tier 3B missing `t_box_in` guard**
   - Tier 3B (`n_sub == 0` / fallback) skips the `first_cell && t_box_in >= tmin - 1e-6f` entry check, causing false self-intersection and shadow acne on road curbs where `raytracer.cu:2132` uses shadow bias `<= 1.5e-3f`.
2. **`csg_eval.cuh:600-611` + `csg-core/src/eval.rs:506-515` — Double-negated normals on mirrored (`NEG_DET`) primitives**
   - `csg_prim_normal_to_compound` negates the normal twice on negative-determinant transforms, inverting `entering` and Beer-Lambert absorption on transmissive CSG glass (`raytracer.cu:2065-2112`) and breaking boolean winding parity.
3. **`fft_bloom.rs:50-62, 373` — `+Inf` -> `NaN` screen-wide FFT corruption**
   - A single `+Inf` radiance pixel produces `Inf / Inf = NaN` in the bright-pass kernel, which global 2D cuFFT spreads across the entire frame. On the host side (`fft_bloom.rs:373`), `intensity <= 0.0` evaluates `false` for `f32::NAN`, allowing NaN intensity into the pass.
4. **`main.rs:1770, 2464, 2656, 2688, 3055, 3468-3470` — `rr_last_exposure = f32::NAN` sentinel trap**
   - `(exposure - rr_last_exposure).abs() > 1e-5` is always `false` when `rr_last_exposure` is `NaN`, leaving DLSS-RR `display_hdr`/`display_ldr` stale or black whenever `ran_rr == false`.
5. **`main.rs:2471, 2996, 4028-4036` — Stalled `Viewport` `PQ10` video recording under `dlss_rr_on && aov_mode != 0 && !hdr_display`**
   - Both `shared_hdr` and `copy_buf` updates are skipped in this combination, hitting `return;` every frame and freezing the recorder indefinitely.
6. **`main.rs:2181-2186, 3144-3198` + `ui.rs:978-986` + `app_events.rs:194-200` — `Smooth 60` lockstep input & shutter state loss**
   - Lockstep recording passes `RawInput::default()` to `cam_driver.update` (canceling active drags/WASD flight and dropping `altitude_speed_multiplier`), skips `publish_render_state`, and `RenderShutterSubframe` omits `possess_free_yaw/pitch/auto_level/zoom_offset` and `reset_seq`.
7. **`ui.rs:503-507` — `Orbit` mode dropdown silently reverts**
   - Mutates `controller_mut().set_mode()` without updating `prefs.mode`, so `CameraDriver::sync_prefs()` immediately overwrites it on the next frame.
8. **`main.rs:1189-1348` — `CursorGrabController` misses one-shot `CursorGrab(Locked)`**
   - `camera_egui::apply_cursor` emits `ViewportCommand::CursorGrab(Locked)` once on frame 0, when `drag_accum_px < 4.0` suppresses it, and never re-emits it during the drag. In persistent fly modes (`buttons_held == 0`), `drag_accum_px` remains `0.0`, so OS cursor lock never engages.
9. **`renderer.rs:2890` — `SaveJpeg` resets accumulation when `max_accum == 0`**
   - `accum_count.min(settings.max_accum.saturating_sub(1))` evaluates to `0` when `max_accum == 0` (infinite accumulation), discarding converged samples and exporting a noisy 1-spp JPEG.
10. **`renderer.rs:3621-3647, 2204` (`csg_scene_host.rs:728, 862`) — CUDA crowd desync/teleport to frame-0 poses**
    - On `!will_camera_repack` `FullRepack`, `SmartView`/`SmartRail`, or `PokeCrowd` when `csg_traffic_speed <= 0.001`, host/GPU crowd transforms diverge and actors snap back to frame 0.

---

### P1 — Major Performance Bottlenecks & High-Impact Defects

#### Track A: GPU Shaders, OptiX 9 Pipeline & cuFFT (`raytracer.cu`, `csg_eval.cuh`, `fft_bloom.rs`)
- **A1 (Multi-AABB GAS)**: `csg_scene_host.rs` + `csg_eval.cuh` — `PURE_UNION` compounds with `M > 1` primitives pack 1 giant AABB per GAS instead of `M` tight `OptixAabb`s in one `OptixBuildInputCustomPrimitiveArray` (indexed via `optixGetPrimitiveIndex()`).
- **A2 (Stack Spills & Register Bloat)**:
  - `csg_eval.cuh:4795`: `csg_trace_rpn_slow` spills `4,224 B` unconditionally.
  - `raytracer.cu:2837, 2879`: `__intersection__csg` (`112 B`) and `__intersection__csg_prim` (`32 B`) spill `CsgSceneView` & `CsgHit` to `.local`.
  - `raytracer.cu:1757`: `__raygen__render` (`36,361` PTX lines, `25,534` `.f32` virtual regs, `80 B` spill) over-inlines `sky_color`, `eval_csg_procedural`, `floor_height`.
  - `csg_eval.cuh:2732`: `csg_intersect_road_terrain_direct` (`4,206` `.f32` regs) `__forceinline__`s `csg_eval_road_cell_height` (`2294`) 7 times.
- **A3 (SER Coherence Bug)**: `raytracer.cu:1644-1648` — `((instanceId & 7) + 1)` overflows 3 bits (`8 -> 0`, colliding every 8th instance with sky miss!) and sorts by `instanceId % 8` instead of `mat_id` (`optixHitObjectGetAttribute_3() & 0x3F`).
- **A4 (Shadow SBT & Traversal Flags)**: `renderer.rs:945, 1469-1563`, `csg_scene_host.rs:1431`, `raytracer.cu:1662` — Shadow rays reuse primary hit groups and 8-word payload; opaque instances omit `OPTIX_INSTANCE_FLAG_DISABLE_ANYHIT`; pipeline omits `OPTIX_TRAVERSABLE_GRAPH_FLAG_ALLOW_SINGLE_LEVEL_INSTANCING` + explicit `usesPrimitiveTypeFlags`.
- **A5 (Memory Read-Only Cache & AABB Redundancy)**: `csg_eval.cuh:125, 4440, 4703, 4894` — `282` non-nc `ld.global` vs `30` `__ldg`; `CsgGpuPrim::aabb_min/max` is never tested before `inv_xform`; `comp.aabb` is redundantly re-tested after OptiX BVH; `csg_materials` (`5.4 KiB`) is not in `__constant__`.
- **A6 (AOV Mode Wasted Shading)**: `raytracer.cu:2041-2174, 2200-2399` — `aov_mode != 0` still executes 4× `eval_csg_procedural`, `trace_shadow`, Blinn-Phong, and 16-bounce classic loop.
- **A7 (Full-Res PoT `FftBloomPass`)**: `fft_bloom.rs:87, 101, 122, 247` — Pads `1080p -> 2048²` and `4K -> 4096²` (`~302 MiB`), wraps halo/spikes on near-PoT widths, and divides by `psf_freq[0].x` per pixel every frame instead of half-res 7-smooth (`2^a 3^b 5^c 7^d`) + pre-normalized PSF.

#### Track B: Host Renderer, Sync, Memory & Caching (`renderer.rs`, `csg_scene_host.rs`, `cuda_crowd.rs`, `gpu_export.rs`)
- **B1 (Accidental 50k IAS Rebuild)**: `renderer.rs:3679-3797` — `ias_state` hashes `sat_angle` and `fractal_transform` even when `!show_classic`, forcing a full 50k-instance IAS rebuild every frame whenever `sat_orbit` or `auto_rotate` is active.
- **B2 (`TrafficOnly` Static Repack)**: `csg_scene_host.rs:1267-1350` — Repacks and hashes all 10k–100k static instances on CPU every dynamic frame instead of splitting `[0..D]` dynamic prefix from `[D..N]` cached static tail.
- **B3 (Mid-Frame Crowd Sync)**: `cuda_crowd.rs:649-656` — `step_gpu_direct` issues pageable `d_states` D2H + `stream.synchronize()` mid-frame before IAS/RT; DEM/walkway steering should move to GPU with a 1-word async atomic displacement flag.
- **B4 (~246 Serial GAS Compaction Stalls)**: `csg_scene_host.rs:779, 1549` — `Accel::custom` inherits `compact: true`, stalling the GPU ~246 times synchronously on every scene or road-mode switch.
- **B5 (Export & Compilation Cache Misses)**:
  - `gpu_export.rs`: recompiles NVRTC kernel and recreates `NvjpegHandle` on **every** JPEG save.
  - `renderer.rs` / `cuda_crowd.rs`: no content-addressed disk PTX cache for `raytracer.cu` / `actor_sim.cuh`, no isolated OptiX cache dir, and serial module compile instead of `Module::with_tasks`.
  - `csg_scene_host.rs`: no `SceneBuilt` LRU cache across scene/road switches.
- **B6 (Redundant Tonemap & 380 MiB Eager VRAM)**: `renderer.rs:4390` runs CUDA tonemap even when DLSS-RR owns display output; `renderer.rs:1894` eagerly allocates ~380 MiB of 4K auxiliary/denoiser/DLSS buffers even when unused.

#### Track C: UI, App Loop, Export Threading & Resource Hygiene (`main.rs`, `ui.rs`, `viewport_toolbar.rs`, `persist.rs`)
- **C1 (`FullWindow` Poll Stall)**: `main.rs:3667` — Runs `FullWindow` capture + `device.poll(WaitIndefinitely)` even when `frames_received == 0`.
- **C2 (`ViewportToolbar` Vertical Clipping)**: `viewport_toolbar.rs:102` — Clips controls to 30px when docked `Left` or `Right`.
- **C3 (UI-Thread Blocking I/O)**: `main.rs` / `persist.rs` — PNG/JPEG/MP4/EXR encoding and `persist::save` execute synchronously on the winit UI thread, causing frame drops/freezes.
- **C4 (Missing `bind_to_thread()` in `Drop`)**: `cuda_crowd.rs:251`, `renderer.rs:804` — CUDA resources dropped without binding context to current thread.
- **C5 (Swallowed Errors & Per-Frame UI Churn)**: `let _ =` on fallible channel/I/O ops and per-frame `format!`/`Vec` allocations in `ui.rs` hot panels.

---

## Prioritized Remediation & Acceleration Plan

### Wave 1 — P0 Correctness Bugs & Immediate Stall Eliminations
- [x] **W1.1**: Fix `RoadTerrainTile` Tier 3B missing `first_cell && t_box_in >= tmin - 1e-6f` guard (`csg_eval.cuh:3069-3076`) and fix double-negated `NEG_DET` normal transform (`csg_eval.cuh:600-611`, `csg-core/src/eval.rs:506-515`) | verify: `cargo test -p csg-core --quiet` + unit test for mirrored primitive normal & curb self-intersection
- [x] **W1.2**: Guard `FftBloomPass` bright-pass against `NaN`/`+Inf` (`isfinite` clamp in CUDA kernel + `!intensity.is_finite() || intensity <= 0.0` on host, `fft_bloom.rs:50-62, 373`) | verify: `cargo test -p nv-demo fft_bloom --quiet`
- [x] **W1.3**: Fix `rr_last_exposure` NaN sentinel (`Option<f32>` or `!rr_last_exposure.is_finite()`, `main.rs:1770, 2464, 3468-3470`) and fix `dlss_rr_on && aov_mode != 0 && !hdr_display` `Viewport` `PQ10` recording stall (`main.rs:2471, 2996, 4028-4036`) | verify: `cargo test -p nv-demo --quiet` + targeted unit test for exposure change & PQ10 AOV fallback
- [x] **W1.4**: Fix `Smooth 60` lockstep input/shutter state preservation (`main.rs:2181-2186, 3144-3198`, `app_events.rs:194-200`), `Orbit` mode dropdown `prefs.mode` sync (`ui.rs:503-507`), and `CursorGrabController` persistent/deferred `CursorGrab(Locked)` re-emission (`main.rs:1189-1348`) | verify: `cargo test -p nv-demo cursor_grab --quiet` + new tests for fly-mode lock & orbit dropdown persistence
- [x] **W1.5**: Fix `SaveJpeg` `max_accum == 0` reset (`renderer.rs:2890`) and CUDA crowd desync/teleport on `!will_camera_repack` / `SmartView` / `PokeCrowd` (`renderer.rs:3621-3647, 2204`, `csg_scene_host.rs:728, 862`) | verify: `cargo test -p nv-demo --quiet` + unit test for `max_accum == 0` JPEG retention
- [x] **W1.6**: Guard `ias_state` `sat_angle`/`fractal_transform` behind `show_classic` (`renderer.rs:3679-3797`) and set `compact: false` on `Accel::custom` (`csg_scene_host.rs:779, 1549`) | verify: `cargo test -p nv-demo --quiet` + verify 0 IAS rebuilds when `!show_classic` and `sat_orbit == true`

### Wave 2 — GPU RT-Core Acceleration, Register/Spill Elimination & Half-Res Bloom
- [x] **W2.1**: Implement **Multi-AABB Custom Primitive GAS** for `PURE_UNION` compounds (`M` tight `OptixAabb`s in 1 `OptixBuildInputCustomPrimitiveArray` + `optixGetPrimitiveIndex()` in `hit_csg_prim` / `PURE_UNION` SBT, `csg_scene_host.rs`, `csg_eval.cuh`, `raytracer.cu`) | verify: `cargo test -p nv-demo --quiet` + PTX compile check + A/B FPS benchmark on `Gundo` & `Marina del Rey`
- [x] **W2.2**: Eliminate `.local` stack spills & virtual register bloat: split `csg_trace_rpn_slow` `<4>` vs `<12>`, read `CsgSceneView` from `__constant__ params` and return `CsgHit` in registers in `__intersection__csg*`, out-of-line `sky_color`/`eval_csg_procedural`/`floor_height`/`csg_eval_road_cell_height` | verify: inspect compiled `raytracer.ptx` confirming `__local_depot29 == 0 B`, `__local_depot28 == 0 B`, and reduced `.f32` reg count in `__raygen__render`
- [x] **W2.3**: OptiX 9 pipeline & traversal optimizations: fix SER coherence bits (`mat_id` `0..63`, no 3-bit overflow, `raytracer.cu:1644-1648`), add dedicated Shadow SBT + minimal payload + `OPTIX_INSTANCE_FLAG_DISABLE_ANYHIT` + `ALLOW_SINGLE_LEVEL_INSTANCING` (`renderer.rs:945, 1469-1563`, `csg_scene_host.rs:1431`), add `__ldg` + per-prim AABB pre-check + drop redundant `comp.aabb` + move `csg_materials` (`5.4 KiB`) to `__constant__` (`csg_eval.cuh`) | verify: PTX compile + `cargo test -p nv-demo --quiet` + bit-identical golden check on shadow/beauty
- [x] **W2.4**: Add `aov_mode != 0` early-out in `raytracer.cu:2041-2174, 2200-2399` (skip procedural shading, shadows, and classic 16-bounce loop when rendering non-beauty AOVs) | verify: `cargo test -p nv-demo --quiet` + AOV visual/FPS verification
- [x] **W2.5**: Upgrade `FftBloomPass` (`fft_bloom.rs:87, 101, 122, 247`) to half-resolution 7-smooth (`2^a 3^b 5^c 7^d`) with safe wrap padding and pre-normalized PSF | verify: `cargo test -p nv-demo fft_bloom --quiet` + VRAM & frame-time measurement @ 1080p/4K

### Wave 3 — Zero-Stall Host Pipeline, Multi-Tier Caching & UI Hygiene
- [x] **W3.1**: Split static-tail vs dynamic-prefix instances in `CsgGpuHost` (`csg_scene_host.rs:1267-1350`) so `TrafficOnly` never repacks/hashes static instances | verify: `cargo test -p nv-demo --quiet` + CPU prep time < 0.2 ms on 50k instances
- [x] **W3.2**: Move crowd DEM/walkway steering to GPU kernel with 1-word async atomic displacement flag, eliminating mid-frame pageable D2H + `stream.synchronize()` (`cuda_crowd.rs:649-656`) | verify: `cargo test -p nv-demo --quiet` + profiler check showing 0 mid-frame syncs
- [x] **W3.3**: Add multi-tier caching: `OnceLock` + disk PTX cache for `gpu_export.rs` NVRTC & `NvjpegHandle`, content-addressed disk PTX cache for `raytracer.cu` / `actor_sim.cuh`, isolated OptiX disk cache + `Module::with_tasks` parallel compile, and `SceneBuilt` LRU cache in `csg_scene_host.rs` | verify: `cargo test -p nv-demo --quiet` + warm-start & scene-switch timing check
- [x] **W3.4**: Skip redundant CUDA tonemap under DLSS-RR (`renderer.rs:4390`), lazy-allocate unused 4K auxiliary buffers (`renderer.rs:1894`), skip `FullWindow` capture + `poll(WaitIndefinitely)` when `frames_received == 0` (`main.rs:3667`), fix `ViewportToolbar` vertical `Left`/`Right` layout (`viewport_toolbar.rs:102`), offload PNG/JPEG/MP4/EXR/`persist::save` to background worker thread, add `bind_to_thread()` in CUDA `Drop` impls (`cuda_crowd.rs:251`, `renderer.rs:804`), and replace swallowed `let _ =` / per-frame UI `format!` churn | verify: `cargo clippy -p nv-demo --all-targets -- -D warnings` + `cargo test -p nv-demo --quiet`

---

## Out of Scope
- `device/` and `python/` excluded workspaces (separate build toolchains).
- Untracked `0/` directory in `nv-rs`.
- Public FFI/wire contract changes in `crates/*-sys` or `crates/nv-bindgen`.
- Non-`nv-demo` crates unless directly required by `csg-core`/`csg-device` P0 fixes (`W1.1`, `W2.1–W2.3`).
