# HANDOFF - resume pointer (updated 2026-11-26)

Read [`PLAN.md`](PLAN.md) first (status, open items, ADRs, gate commands). The camera planner lives in the separate repo `cglibs/flight-navigator-rs` (`CAM_PLAN.md`, private GitHub `ssoj13/flight-navigator-rs`).

## State at handoff

- `nv-rs` `main`: Wave A committed & pushed (Wave 2 `GpuVoxelBaker` + `Skeletons`/`FlightPath` overlays, UI SSOT, Fractal Presets, `SceneBuildProgress` HUD, `SimTelemetry`, `B2` Marina thin-glass fix in `raytracer.cu`, `flight-navigator-rs` `c5ad038`, `csg-rs` `99dc630`; verified: `nv-demo` clippy 0 warnings, `230/0/9` release tests, `1/1` OptiX 9 pipeline compile/link, `1/1` real-Gundo Auto-Plan, `target/release/nv-demo.exe` built). Leave any line-ending-only diff in `crates/cudnn-sys/prebuilt/x86_64-pc-windows-msvc/cudnn_api.rs` and untracked `NOTE.md` untouched.
- `csg-rs` pin in `nv-rs`: `99dc630` (P41 `semantic_masks`, P42 OSM provenance & logical object grouping, `FramePlan` `PlanKind::Crowd` v2 + `PlanKind::Camera` v1, R0+R1 `VoxelVolume` / `voxel_bake.cuh` / `WorldRayQuery`, `RoadTerrainTile` v2 `u16` CSR + `smin` height blend + zero embankment sawtooth + ~300x faster tile bake).
- `flight-navigator-rs` `main` = `c5ad038` (pushed `origin/main` + `origin/dev`; 48/48 tests, 0 clippy & 0 rustdoc warnings): `fnav-core`, `fnav-interest`, `fnav-harmony`, `fnav-space`, `fnav-plan` (seed/mode exploration + beam dedup, smooth `cut_density` C2/cut scaling, quality/clearance floors, `PlanCache` limits invalidation), integrated into `nv-demo` `crates/nv-demo/src/flight/`.
- `camera-guide-rs` `main` = `6cffddf`: one speed/bank/timing planner, `f64` time.

## Done on 2026-11-25..26

| Commit | What |
| --- | --- |
| `flight-navigator-rs` `92d30bd`, `nv-rs` `f93e61d` | `flight-navigator-rs` crates implemented and integrated into `nv-demo` (`src/flight/`: Flight Lab + Flight Player + live R0+R1 `CsgSceneOracle`, replacing `camera_rail.rs`); `csg-rs` P41 adopted (`VEGETATION = 20`, `TERRAIN_FEATURE = 21`, `semantic_masks` fast path in `semantic_filter.rs`). Closes `RAIL:GEN`, `FILTER:FAST`. |
| `5324699` | Generic `GpuFramePlan` executor (`frame_plan_runner.rs`), `CrowdFramePlan` v2 (`cuda_crowd.rs`), `CameraFramePlan` v1 (`cuda_camera.rs` `GpuCameraComposer`); private `prepare_crowd_steering_and_grounds` deleted. Closes `CSG:R00`; narrows `GPU:FRAMEPLANS`. |
| `d70250e` | `nvrtc::MathMode::StrictF32` (`--ftz=false`, `--prec-div=true`, `--prec-sqrt=true`, `--fmad=false`) + PTX disk-cache keying on `MathMode` (`gpu_export.rs`, `frame_plan_runner.rs`). |
| `39c71d4` | `csg-rs` `bd5f73b` adopted: P42 per-instance OSM provenance (`OsmFeatureClass`/kind/`osm_id`) in GPU pick (`PickedObject::osm`) and category filter (`DemoSettings::hidden_osm_classes`, zero-IS-cost `0x80` hardware mask); `OBJECT_KEY_BIT` logical object selection; `ovl_axes` object-root pivot triads & whole-object selection bounds. Closes `DEMO:FILTER` and `DEMO:VIEWS` #4. |
| Wave A (verified & committed) | Wave 2 (`GpuVoxelBaker` `voxel_bake.cuh`, `OverlayKind::Skeletons`, `OverlayKind::FlightPath`), UI SSOT (`UiTheme::apply` zero hover-height jitter, `VAL_W = 68.0` + `slider_drag_value` + `allocate_clamped_ui`, deduplicated natural-height Contact Sheet, non-overlapping Playlist Queue/Library), `FractalPresetManager` (`sl-controls::preset`, atomic `~/.nv-rs/fractal_presets.json`), live `SceneBuildProgress` HUD overlay + status bar (`AppBus::build_progress`, `sync_with_progress`, 50 ms repaint), zero-readback `SimTelemetry` (Crowd/Locomotion/Traffic in `Telemetry`), `B2` thin-glass `glass_passes` (`0..6`) + `2 mm`-capped transmitted bias in `raytracer.cu`, `flight-navigator-rs` `c5ad038`, and `csg-rs` `99dc630`. Gate PASS: `nv-demo` clippy 0 warnings, `230/0/9` release tests + `1/1` OptiX 9 pipeline compile/link + `1/1` real-Gundo Auto-Plan + `target/release/nv-demo.exe`. |

## Exact resume point

- [ ] **Wave B+C+D adoption (`csg-rs` -> `nv-rs`)**: once `csg-rs` lands `B3+` (`crates/csg-traffic`, `PlanKind::Traffic` v1, VRAM Pedestrian AEB/Yield), `B1` (Smart Yacht/Thin-Mast LOD + auto-proxy guard), `C1/C2` (water basin carving, holes, Oxford Basin, `WaterMask`, recessed water), and `D1` (real OSM street names on procedural signs), bump `csg-*` in `nv-rs` and wire `nv-demo` `PlanKind::Traffic` + ADAS HUD/overlay + `CSG_PROC_SIGN_TEXT` shader.
- [ ] **`CSG:R00b` / `GPU:FRAMEPLANS`**: when `csg-rs` ships `TrafficFramePlan`, wire it onto `GpuFramePlan` (`frame_plan_runner.rs`), then delete the CPU traffic step and private `pack_visible` copy.
- [ ] **`OVERLAY:NEXT` / `DEMO:VIEWS`**: add camera frustum provider; switch to `csg-scene` `OverlayCurvePod` (160 B, P36 exact feature curves, K6 visibility raygen with `0x80` `LINES_OCCLUDER`) and delete the interim 64 B local record when P36 lands; wire optional R0 Voxel display mode view when prioritized.
- [ ] **`P37:SWEEPS`**: A/B every `csg-rs` sweep wave with `nv-demo --bench-views p37`; wire payload slot 9 = ray cone `half2` when the first `MESH` record lands.
- [ ] **`RAIL:ROLL` / `LOD:SIZE` / `GPU:D04` / `CSG:RPN-FLAG`**: adopt when `csg-rs` pushes fixes.

Out of scope: rewriting git history (operator: never); stripping `addr:*` tags of other buildings in the OSM extracts (operator: leave as is, don't raise again); further RT perf hunting (`PERF:RT` parked).

## Chat coordination

The `csg-rs` session is the chat peer `Mullis-16-3934` (filesystem MCP `chat_*`, mode auto). Coordinate directly, don't make the operator relay, and restart `chat-wait` in the background after each message. The `tars-rs` session (`tars-rs-30`) shares the machine: `CARGO_BUILD_JOBS=6`, and no simultaneous full suites; ping it for a quiet GPU.

## Operator rules

- Rustdocs everywhere explaining WHY (`//!` + `///`, CUDA block comments); put this in every agent brief and peer relay.
- Never put residential addresses in labels, code, docs or commits.
- Systemic, built-in, GPU-first features; nothing may slow the frame; no fakes or stubs; a disabled UI shows its exact blocking reason.
- Russian chat, English code/docs; no `Co-authored-by`; no `git reset`/`checkout --`/`stash`; never kill `bun`; push to `main` after green gates; the lead delegates cargo runs to subagents.
