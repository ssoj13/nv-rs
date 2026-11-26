# HANDOFF - resume pointer (updated 2026-11-26)

Read [`PLAN.md`](PLAN.md) first (status, open items, ADRs, gate commands). The camera planner lives in the separate repo `cglibs/flight-navigator-rs` (`CAM_PLAN.md`, private GitHub `ssoj13/flight-navigator-rs`).

## State at handoff

- `nv-rs` `main`: Wave 1 commits through `39c71d4` + Wave 2 (`GpuVoxelBaker` / `voxel_bake.cuh`, `OverlayKind::Skeletons`, `OverlayKind::FlightPath`) + Wave 3 docs sync (pending final gate before commit/push). Leave any line-ending-only diff in `crates/cudnn-sys/prebuilt/x86_64-pc-windows-msvc/cudnn_api.rs` unstaged if present.
- `csg-rs` pin in `nv-rs`: `bd5f73b` (P41 `semantic_masks`, P42 OSM provenance & logical object grouping, `FramePlan` `PlanKind::Crowd` v2 + `PlanKind::Camera` v1, R0+R1 `VoxelVolume` / `voxel_bake.cuh` / `WorldRayQuery`).
- `flight-navigator-rs` `main` = `92d30bd` (pushed): `fnav-core`, `fnav-interest`, `fnav-harmony`, `fnav-space`, `fnav-plan`, integrated into `nv-demo` `crates/nv-demo/src/flight/`.
- `camera-guide-rs` `main` = `6cffddf`: one speed/bank/timing planner, `f64` time.

## Done on 2026-11-25..26

| Commit | What |
| --- | --- |
| `flight-navigator-rs` `92d30bd`, `nv-rs` `f93e61d` | `flight-navigator-rs` crates implemented and integrated into `nv-demo` (`src/flight/`: Flight Lab + Flight Player + live R0+R1 `CsgSceneOracle`, replacing `camera_rail.rs`); `csg-rs` P41 adopted (`VEGETATION = 20`, `TERRAIN_FEATURE = 21`, `semantic_masks` fast path in `semantic_filter.rs`). Closes `RAIL:GEN`, `FILTER:FAST`. |
| `5324699` | Generic `GpuFramePlan` executor (`frame_plan_runner.rs`), `CrowdFramePlan` v2 (`cuda_crowd.rs`), `CameraFramePlan` v1 (`cuda_camera.rs` `GpuCameraComposer`); private `prepare_crowd_steering_and_grounds` deleted. Closes `CSG:R00`; narrows `GPU:FRAMEPLANS`. |
| `d70250e` | `nvrtc::MathMode::StrictF32` (`--ftz=false`, `--prec-div=true`, `--prec-sqrt=true`, `--fmad=false`) + PTX disk-cache keying on `MathMode` (`gpu_export.rs`, `frame_plan_runner.rs`). |
| `39c71d4` | `csg-rs` `bd5f73b` adopted: P42 per-instance OSM provenance (`OsmFeatureClass`/kind/`osm_id`) in GPU pick (`PickedObject::osm`) and category filter (`DemoSettings::hidden_osm_classes`, zero-IS-cost `0x80` hardware mask); `OBJECT_KEY_BIT` logical object selection; `ovl_axes` object-root pivot triads & whole-object selection bounds. Closes `DEMO:FILTER` and `DEMO:VIEWS` #4. |
| Wave 2 (pending final gate) | CUDA `GpuVoxelBaker` (`flight/voxel_gpu.rs`, compiling `voxel_bake_cuh_source(CSG_EVAL_CUH)` `voxel_bake_trace_columns` / `voxel_bake_merge_columns`, cutting R0 `VoxelVolume` bake from ~2 s CPU to ~265 ms GPU with overflow check & exact CPU fallback); `OverlayKind::Skeletons` (zero-readback GPU `ovl_skeletons` on `CrowdWorldJoints` + static `[u32; 4]` `d_skeleton_edges`); `OverlayKind::FlightPath` (`FlightPathOverlayData` cached spline + cut/C2 handover markers). Narrows `OVERLAY:NEXT`. |
| Wave 3 (pending final gate) | Docs sync across `PLAN.md`, `HANDOFF.md`, `AGENTS.md`, `crates/nv-demo/AGENTS.md`. |

## Exact resume point

- [ ] **Final gate for Wave 2 + Wave 3**: run `cargo clippy --workspace --all-targets --message-format=short -- -D warnings`, `cargo test --workspace --quiet`, ignored GPU tests for touched crates (`nv-demo`, `nvrtc`), and a `cargo run -p nv-demo --release` smoke test (verify `raytracer.cu` + `tonemap.cu` + `voxel_bake.cuh` compile cleanly, Flight Lab auto-plan runs GPU voxel bake, and `Contours` / `Axes` / `Skeletons` / `FlightPath` overlays render cleanly); commit and push to `main`.
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
