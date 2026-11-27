# HANDOFF - resume pointer (updated 2026-11-26)

Read [`PLAN.md`](PLAN.md) first (status, open items, ADRs, gate commands). The camera planner lives in the separate repo `cglibs/flight-navigator-rs` (`CAM_PLAN.md`, private GitHub `ssoj13/flight-navigator-rs`).

## State at handoff

- `nv-rs` `main`: Wave B+C+D committed & pushed (on top of Wave A `686942f`): `csg-rs` `3fbe5a0` (`crates/csg-traffic`, B1 smart boat LODs, C1/C2 recessed water, D1 `SignTextTable`), GPU `PlanKind::Traffic` v1 (`cuda_traffic.rs`) + zero-copy `Traffic -> Crowd -> IAS -> Camera`, `PossessToolbar` ADAS HUD pill (`THR/BRK` bar) + `OverlayKind::Adas`, `704 B` `LaunchParams` + `628 B` Highway Gothic SDF + 3-tier LOD in `raytracer.cu`, and MP4 recording active-rail preservation. Verified: `nv-demo` clippy 0 warnings, `238/0/9` release tests (`247` total), `1/1` OptiX 9 `704 B` pipeline compile/link (`css_rg: 352 B`), `1/1` real-Gundo Auto-Plan, `target/release/nv-demo.exe` built. Leave `device/*` (other session), `.agents/`, and untracked `NOTE.md` untouched.
- `csg-rs` pin in `nv-rs`: `3fbe5a0` (`crates/csg-traffic` + 4 deadlock fixes + `PlanKind::Traffic` v1 + VRAM Pedestrian AEB/Yield, B1 smart yacht/thin-mast LOD + auto-proxy guard, C1/C2 carved water basins/holes/Oxford Basin/`WaterMask`/recessed `+0.5 m` water, D1 `SignTextTable` & OSM street names, P41 `semantic_masks`, P42 OSM provenance, `RoadTerrainTile` v2 `u16` CSR).
- `flight-navigator-rs` `main` = `c5ad038` (pushed `origin/main` + `origin/dev`; 48/48 tests, 0 clippy & 0 rustdoc warnings): `fnav-core`, `fnav-interest`, `fnav-harmony`, `fnav-space`, `fnav-plan` (seed/mode exploration + beam dedup, smooth `cut_density` C2/cut scaling, quality/clearance floors, `PlanCache` limits invalidation), integrated into `nv-demo` `crates/nv-demo/src/flight/`.
- `camera-guide-rs` `main` = `6cffddf`: one speed/bank/timing planner, `f64` time.

## Done on 2026-11-25..26

| Commit | What |
| --- | --- |
| `flight-navigator-rs` `92d30bd`, `nv-rs` `f93e61d` | `flight-navigator-rs` crates implemented and integrated into `nv-demo` (`src/flight/`: Flight Lab + Flight Player + live R0+R1 `CsgSceneOracle`, replacing `camera_rail.rs`); `csg-rs` P41 adopted (`VEGETATION = 20`, `TERRAIN_FEATURE = 21`, `semantic_masks` fast path in `semantic_filter.rs`). Closes `RAIL:GEN`, `FILTER:FAST`. |
| `5324699` | Generic `GpuFramePlan` executor (`frame_plan_runner.rs`), `CrowdFramePlan` v2 (`cuda_crowd.rs`), `CameraFramePlan` v1 (`cuda_camera.rs` `GpuCameraComposer`); private `prepare_crowd_steering_and_grounds` deleted. Closes `CSG:R00`; narrows `GPU:FRAMEPLANS`. |
| `d70250e` | `nvrtc::MathMode::StrictF32` (`--ftz=false`, `--prec-div=true`, `--prec-sqrt=true`, `--fmad=false`) + PTX disk-cache keying on `MathMode` (`gpu_export.rs`, `frame_plan_runner.rs`). |
| `39c71d4` | `csg-rs` `bd5f73b` adopted: P42 per-instance OSM provenance (`OsmFeatureClass`/kind/`osm_id`) in GPU pick (`PickedObject::osm`) and category filter (`DemoSettings::hidden_osm_classes`, zero-IS-cost `0x80` hardware mask); `OBJECT_KEY_BIT` logical object selection; `ovl_axes` object-root pivot triads & whole-object selection bounds. Closes `DEMO:FILTER` and `DEMO:VIEWS` #4. |
| `686942f` (Wave A) | Wave 2 (`GpuVoxelBaker` `voxel_bake.cuh`, `OverlayKind::Skeletons`, `OverlayKind::FlightPath`), UI SSOT (`UiTheme::apply` zero hover-height jitter, `VAL_W = 68.0` + `slider_drag_value` + `allocate_clamped_ui`, deduplicated natural-height Contact Sheet, non-overlapping Playlist Queue/Library), `FractalPresetManager` (`sl-controls::preset`, atomic `~/.nv-rs/fractal_presets.json`), live `SceneBuildProgress` HUD overlay + status bar (`AppBus::build_progress`, `sync_with_progress`, 50 ms repaint), zero-readback `SimTelemetry`, `B2` thin-glass `glass_passes` (`0..6`) + `2 mm`-capped transmitted bias in `raytracer.cu`, `flight-navigator-rs` `c5ad038`, and `csg-rs` `99dc630`. Gate PASS (`230/0/9`). |
| Wave B+C+D (verified & committed) | Adopted `csg-rs` `3fbe5a0` (`crates/csg-traffic` + 4 deadlock fixes + `PlanKind::Traffic` v1 + VRAM Pedestrian AEB/Yield, B1 smart yacht/thin-mast LOD + auto-proxy guard, C1/C2 water basin carving/holes/Oxford Basin/`WaterMask`/recessed `+0.5 m` water, D1 `SignTextTable` & real OSM street names); wired `cuda_traffic.rs` (`PlanKind::Traffic` v1, zero-copy `Traffic -> Crowd -> IAS -> Camera`, `TrafficBackend::{Auto, Cpu, Cuda}`), `PossessToolbar` ADAS HUD pill (`THR/BRK` bar) + `OverlayKind::Adas` (`ovl_adas`), `704 B` `LaunchParams` + `628 B` `__constant__` Highway Gothic SDF + 3-tier LOD in `raytracer.cu`, and `FlightWorkspace::toggle_record_playlist` active-rail MP4 fix. Gate PASS: `nv-demo` clippy 0 warnings, `238/0/9` release tests (`247` total) + `1/1` OptiX 9 `704 B` pipeline compile/link + `1/1` real-Gundo Auto-Plan + `target/release/nv-demo.exe`. Closes `CSG:R00b` / `GPU:FRAMEPLANS`. |

## Exact resume point

- [ ] **Interactive visual check (`VIS:DEMO`)**: run `target/release/nv-demo.exe` and visually inspect Marina Crescent Tower glazing, moored yacht LODs, carved water basins/Oxford Basin, GPU Traffic AEB/Yield + Possess ADAS HUD + `OverlayKind::Adas`, Highway Gothic OSM street-name signs, and MP4 recording on an active Lab preview rail.
- [ ] **`OVERLAY:NEXT` / `DEMO:VIEWS`**: add camera frustum provider; switch to `csg-scene` `OverlayCurvePod` (160 B, P36 exact feature curves, K6 visibility raygen with `0x80` `LINES_OCCLUDER`) and delete the interim 64 B local record when P36 lands; wire optional R0 Voxel display mode view when prioritized.
- [ ] **`P37:SWEEPS`**: A/B every `csg-rs` sweep wave with `nv-demo --bench-views p37`; wire payload slot 9 = ray cone `half2` when the first `MESH` record lands.
- [ ] **`RAIL:ROLL` / `LOD:SIZE` / `GPU:D04` / `CSG:RPN-FLAG`**: adopt when `csg-rs` pushes fixes.

Out of scope: rewriting git history (operator: never); stripping `addr:*` tags of other buildings in the OSM extracts (operator: leave as is, don't raise again); further RT perf hunting (`PERF:RT` parked).

## Chat coordination

The `csg-rs` session is the chat peer `Mullis-16-3934` (filesystem MCP `chat_*`, mode auto). Coordinate directly, don't make the operator relay, and restart `chat-wait` in the background after each message. The `tars-rs` session (`tars-rs-30`) shares the machine: `CARGO_BUILD_JOBS=6`, and no simultaneous full suites; ping it for a quiet GPU.

## Operator rules

- **100% procedural / zero triangle meshes**: keep all scene geometry, terrain, vehicles, crowds, and signage/text purely procedural/analytical CSG/SDF (zero triangle-mesh assets or baked bitmap textures).
- **Minimal private git dependencies**: keep private git deps strictly minimal and isolated to `nv-demo` for clean future open-sourcing/vendoring of `nv-rs`.
- Rustdocs everywhere explaining WHY (`//!` + `///`, CUDA block comments); put this in every agent brief and peer relay.
- Never put residential addresses in labels, code, docs or commits.
- Systemic, built-in, GPU-first features; nothing may slow the frame; no fakes or stubs; a disabled UI shows its exact blocking reason.
- Russian chat, English code/docs; no `Co-authored-by`; no `git reset`/`checkout --`/`stash`; never kill `bun`; push to `main` after green gates; the lead delegates cargo runs to subagents.
