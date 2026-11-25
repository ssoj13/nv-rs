# nv-rs - single authoritative plan

Consolidated 2026-11-23 from the old `PLAN.md` (M1..M6), `plan1.md`, `plan2.md`, `TODO.md` and `HANDOFF.md`. Nothing was dropped: the full originals are archived verbatim (see [Archive](#archive)). Resume pointer for a new session: [`HANDOFF.md`](HANDOFF.md). Repository guide: [`AGENTS.md`](AGENTS.md); diagrams: [`DIAGRAMS.md`](DIAGRAMS.md).

Item IDs are stable. `<PREFIX>:<id>` points into the per-slice ledgers `audit/2026-11-23/NN_*.md` (prefixes: `OPX` 01, `BLA` 02, `SOL` 03, `LDR` 04, `MED` 05, `IOP` 06, `SYS` 07, `CAM` 08, `RND` 09, `GPU` 10, `APP` 11, `DPY` 12). `Wn.m` are remediation-wave items (full text: `audit/2026-11-23/plan2_full.md`). An item is closed only when its fix is merged AND verified by a gate, never on a worker's self-report.

## 1. Status

State as of `a33d21f` (`docs: final wave report and changelog`), 2026-11-23; the docs commit that applies the final wave to this file follows it. Hardware: RTX 3080 Ti (sm_86), CUDA 13.3, OptiX 9.1, Windows 11.

### Done (by wave; details in section 4)

| Wave | What | Commits / report |
| --- | --- | --- |
| Audit | 12 read-only slices, 10 lead spot-checks | `8af9860`; ledgers `audit/2026-11-23/01..12_*.md` |
| W4.1 | `camera-rs` de-vendored | `camera-rs` `9109108`, `nv-rs` `a4eb85e` |
| W1 + W2 | wrapper crates, `optix`, `nv-wgpu`, `cuda-core-ext`; lease helper | `8e84de4`, gate `05efa7e` (clippy clean, 379 then 375 passed / 0 failed) |
| W3 + W5 | `nv-demo` correctness; loader / bindgen / sys / `xtask` | `7a62138`, root gate `4f457d0`, `gate_root_w3_w5.md` (467 passed / 0 failed / 204 ignored) |
| W5.3 / W5.4 (written) | `bootstrap.py`, CI, `deny.toml` | `a246a6a` (never executed) |
| W6.2 | `python/nv-py` | gate `8f7af2f`, `gate_python_w6.md` (pytest 77/82 at that gate) |
| W6.1 | `device/` | `c04d692`, `wave_6_device.md` (15/15 GPU cases) |
| W1b | context by value, typed teardown sink, one `Scratch`, cuDNN adopts loaded copy | `1467778`, `wave_1b.md` (467 passed / 0 failed / 205 ignored; pytest 80 passed / 1 failed (NVAPI sees 0 displays, environment) / 1 skipped) |
| W7 (partial) | docs sync | `d8dce5f`, `c441ac3`, `3bdc5a6` |
| csg-rs adoption | `csg-rs` `24b22f5` merged | `ea2161d`, `a807624`, `csg_rs_125b870_adoption.md`; gate: clippy clean, 467 / 0 / 205, nv-demo ignored GPU tests 5/5, release app renders Marina with CUDA crowd 180 and DLSS RR |
| nv-demo follow-ups | GPU:D03 (ABI 648 B), GPU:D21, RND:R10, `anim_range` sliders, async launch upload, APP:F19, RND:H13, `nv-wgpu` submit timeout | `ee08ade`, `wave_nvdemo_followups.md` |
| csg-rs bump | `csg-rs` `fa5e170` adopted: typed `SceneBuildError` to the status line, `SceneDescriptor::geo_origin` (closes `APP:F24`), `FIRST_HIT_UNION` stays on RPN, explicit pipeline stack sizing | `9b6e178`, `csg_fa5e170_adoption.md`; gate: clippy clean, 500 / 0 / 208, nv-demo ignored GPU tests 6/6; RT pass +12 % (see `PERF:RT`) |
| Final wave | `W5.2` catalog-driven `xtask` + bindings manifest + loader leftovers + Linux loader search + cross-build fixes (`208e628`); `W4.3` wrapper duplicates + bounded `nv-wgpu` waits (`0508d75`); `nv-demo` OptiX SDK discovery, PTX cache key, `perf_log`, single-source tables, `main()` `ExitCode` (`0714f65`); `DPY:C20` `close()` / context managers (`473e1fe`); device CI job, `deny.toml`, `url` 2.5.9 / RUSTSEC-2026-0592 (`a7b7b49`); report and changelog (`a33d21f`) | `wave_final.md`; gate: clippy clean, **500 passed / 0 failed / 207 ignored** |

Final wave GPU tests (`-- --ignored --test-threads=1`, passed / failed): cuda-core-ext 12/0, nv-wgpu 13/0, optix 68/0, cublas 9/0, cudnn 5/0, cusolver 9/0, cusparse 11/0, nvenc 4/0, nvdec 2/0, nvofa 4/0, nvjpeg 7/0, nvenc-sys 1/0, nvapi-sys 1/0, nv-demo 5/0. `python/nv-py`: clippy clean, `cargo test --lib` 19 / 0 / 5 ignored (+ 5 ignored passed), **pytest 118 passed / 2 skipped**. `device/`: **15/15** cases. Linux cross: `cargo clippy --target x86_64-unknown-linux-gnu --workspace --all-targets -- -D warnings` clean except `nvcomp-sys` / `nvcomp` (compile only, nothing ran on Linux). `cargo deny check`: advisories, bans, sources ok; licenses FAILED (`DENY:LIC`).

Final wave performance (release, persisted default scene, 4 spp, 6 bounces, 150 warm-up + 600 measured frames, 5 runs; median of run medians): frame work 51.17 ms (p95 59.70), OptiX RT pass 50.12 ms (98 %), host prep 0.74 ms, CUDA compute 0.68 ms, tonemap 0.089 ms, crowd step 0.175 ms. Runs 1-3 shared the GPU with other processes (20-34 %); the clean figure is about 45.6 ms. No speed-up was made, so there is no before/after pair (`PERF:RT`).

### In flight

- Final wave: done (`wave_final.md`, applied here).
- **csg-rs session** (chat name `Mullis-16-3934`): `R00` GPU crowd steering moved into a csg-rs-owned kernel; `R00b` GPU traffic. Branches from `origin/main`; they ping in chat for review, the lead reviews and merges.

### Open

See section 2 (every unticked item, deduplicated).

## 2. Open items

Owner `lead` = this repository's session; `csg-rs` = the csg-rs session. Pointer = ledger or report holding the detail. Updated after the final wave (`wave_final.md`, `ee08ade..a33d21f`).

### Verification and review

| ID | Description | Owner | Pointer |
| --- | --- | --- | --- |
| `REV:W1` | Read the diffs of every P0 fix of Wave 1 and check each new test analytically (does its assertion depend on the fixed line?). No revert-and-rerun mutation runs (operator decision 2026-11-23). The final wave did not repeat it. | lead | `plan2_full.md` Wave 1; `wave_final.md` "Not done" |
| `REV:W2` | Review the `nv-wgpu` encoder-based submit diff. | lead | `plan2_full.md` W2.2; `06_coreext_wgpu.md` |
| `GATE:FINAL` | Narrowed: the final wave gated root (clippy clean, 500 / 0 / 207), `python/` (pytest 118 passed / 2 skipped) and `device/` (15/15). Remaining: one more full gate after the csg-rs `R00` / `R00b` work lands. | lead | section 6; `wave_final.md` Gate results |
| `VIS:DEMO` | Interactive checks of `nv-demo` that no gate performed: AOVs (Depth, Motion, Albedo, Normal, WorldPos, Semantic, Instance), the four sensor projections, MP4 recording, SDG burst and cancel, DLSS picture, classic object motion under DLSS / temporal denoiser, sky crossfade, camera feel after the `camera-rs` switch, cursor modes. The final wave did none of them. | lead (needs a person at the machine) | `plan2_full.md` Status; `wave_final.md` "Not done" |
| `GPU:RERUN` | Narrowed: ignored GPU tests were rerun in the final wave for cuda-core-ext 12, nv-wgpu 13, optix 68, cublas 9, cudnn 5, cusolver 9, cusparse 11, nvenc 4, nvdec 2, nvofa 4, nvjpeg 7, nvenc-sys 1, nvapi-sys 1, nv-demo 5 (all 0 failed). Crates not in that list (for example cufft, curand, cupti, nvcomp) were not rerun. | lead | `wave_final.md` Gate results |
| `LNX:ALL` | Narrowed: Linux code now compiles and `cargo clippy --target x86_64-unknown-linux-gnu --workspace --all-targets -- -D warnings` is clean except `nvcomp-sys` / `nvcomp` (no Linux nvCOMP headers); Linux loader search (globbing, versioned sonames, `extras/CUPTI/lib64`) done in `208e628`. Still open: nothing ever ran on Linux (NCCL needs 2+ GPUs, cuFile needs GDS), `python/` not cross-compiled (PyO3 needs a Linux interpreter config), `nvcomp` on Linux needs Linux headers. | lead (needs a Linux host) | `wave_final.md` Linux-only code, Not done |
| `SYS:NGX-HARNESS` | Vulkan test harness for `ngx` (`ash` as dev-dependency); `SYS:NGX-2`, `SYS:NGX-7`, `SYS:NGX-11` have documentation only. | lead | `07_system_xtask.md` |
| `HOF:PY-FOLLOW` | Follow-ups the `python/nv-py` platform worker asked for elsewhere (not re-verified): `cuda-core-ext` `ManagedBuffer::as_slice` / `as_mut_slice` unsafe or synchronising, `zeroed` via `cuMemsetD8`, `HostRegistration::unregister(self) -> Result`; `cupti` `Subscriber::close` / `ActivityProfiler::close` returning `Result`; `python/nv-py` `errors.rs` allowing `General` on Windows so NVAPI errors stop mapping to `NvCudaError`. | lead | old `HANDOFF.md` (`git show ee08ade:HANDOFF.md`) |

### Build, release, CI

| ID | Description | Owner | Pointer |
| --- | --- | --- | --- |
| `CI:EXEC` | Replaces `W5.4`. Workflows (`a246a6a`, fixed in `a7b7b49`) are written and `actionlint` v1.7.12 is clean (only its unknown `ubuntu-26.04` label), but CI was never executed. The new `device` job needs the secret `CUDA_RUST_B_TOKEN` (the `cuda-rust_b` fork is private); its 15 GPU cases stay a hardware run. | lead | `wave_final.md` CI / W5.4 |
| `DENY:LIC` (closed: `deny.toml` declares the three private crates `LicenseRef-Proprietary` via `licenses.clarify` + per-crate exceptions; `cargo deny check licenses` ok on cargo-deny 0.22.0) | `cargo deny check`: advisories ok, bans ok, sources ok, licenses FAILED for `flex-geom`, `nodes-core`, `ui-theme` 0.1.0 (git crates of the operator's own repositories without a `license` field). Fix: add `license` upstream or a `[[licenses.clarify]]` entry. Operator decision; no license is invented here. | operator | `wave_final.md` Addendum |
| `PREBUILT:REGEN` | Committed prebuilt sets (NVAPI, cuDNN, NGX, OptiX) come from newer SDKs than this machine has (NVAPI 62115 vs local 61683), so regenerating here would downgrade them; only `nvenc_types.rs` was regenerated. The committed NVAPI prebuilt shims still resolve the id on every call (L-30). Run `cargo xtask prebuilt` on a machine with the matching SDKs. | lead (needs newer SDKs) | `wave_final.md` Not done |
| `LDR:L-31a` | cuDNN still preloads every `cudnn_*` sibling eagerly. The alternatives (`AddDllDirectory` / `SetDllDirectory`) change the process-wide DLL search and no test covers the effect on other DLLs. | lead | `wave_final.md` L-31 |
| `DEV:FORK` | Refresh the `cuda-rust_b` fork when the root workspace needs an NVIDIA-only `cuda-core` API. | lead | ADR-3 |

### Single source of truth

| ID | Description | Owner | Pointer |
| --- | --- | --- | --- |
| `APP:F21` | `persist.rs:313-374` and `ui.rs:145-164` keep hand-written string tables although `serde` derives are available; left to keep the on-disk config format unchanged. | lead | `11_demo_main_ui.md` |

### `nv-demo`

| ID | Description | Owner | Pointer |
| --- | --- | --- | --- |
| `PERF:RT` | **Parked (operator, 2026-11-24): regression accepted.** Measure only with the rail benchmark (`nv-demo --bench-rail road_cruise --scene gundo`; frozen pose, free Pinhole camera, lockstep 1/60 s steps, state columns in the CSV); the earlier free-running numbers (44.1 / 49.4 ms) did not control camera state and are void. Rail, per-frame minimum over interleaved rounds: csg `24b22f5` 58.26 ms median vs `fa5e170` 77.01 ms (loop +19.8 %); P23 (`6e84ff6`, `32a6ab1`) recovered 6.7 % vs `fa5e170` (81.97 -> 75.88 ms). Attribution (P23): primary + its shadow 34.5 %, second bounce 38.4 %, bounces 3-6 27.1 %; shadow rays 21.5 %; SER off +1.5 %. Optional candidates: shadow-only visibility mask for non-`CAST_SHADOW` CSG instances; the 10 km AABB of unbounded (ground) compounds that every ray enters. ncu cannot collect counters inside nv-demo ("performance metric collection layer"). | parked | `audit/2026-11-24/perf_rt.md` |
| `RAIL:GEN` | Build a proper camera-rail generator (operator, 2026-11-24). Today on `gundo` the rail flies along the shore, turns sharply and flies back along the shore: dull after the first loop; the auto generator picks points in the OCEAN. Do not copy old trajectories (the old hand-picked points were better, nothing more). Requirements: control points only over valid, interesting content (land / streets / landmarks, never open water or inside geometry, clearance from geometry), a closed loop that visits distinct areas without retracing, smooth C1 path with sane turn rates and altitude, look-at targets on content, deterministic seed (the rail is also the A/B benchmark path). Operator decisions 2026-11-24: covered water under bridges within 40 m of land is allowed; duration is a user parameter 15-120 s and the generator chooses the number of shots and their timing by searching for the most harmonious transitions; a cinematic vocabulary of moves (pans H/V, tilt, top-down, orbit, push-in/fly-forward, dolly, crane, reveal, establishing) and lenses (200-400 / 80 / 35 mm, wide) sequenced by film grammar; interestingness as an isolated crate, harmony as a separate block, node-based selection pipeline; camera state (incl. optics) as a first-class value with one smooth transition engine A->B over N s; continuous chained shots preferred, cuts only when meaningful; street runs at asphalt / 2nd-floor / above-roof heights; the system lives in a new separate cglibs repository `flight-navigator-rs` (plan: `CAM_PLAN.md`); the planner works on a csg-scene voxel volume (sparse bricks, 0.25-0.5 m, semantics water/ground/walkable/solid) + ESDF provided by csg-rs. | lead (+ csg-rs for scene data: walkable/land masks, landmarks) | operator note; design `audit/2026-11-24/rail_gen_design.md` |
| `RAIL:ROLL` | 1-2 frame ~0.5 deg roll twitch on `gundo` over the tanks toward the shore. Root cause: csg-scene `rail.rs:569-611`, `:646-674` bank from pointwise second differences (0.2 m window, taps 9.5 m apart) passes C1 Catmull-Rom curvature spikes from short segments created by `relax_rail_against_world` straight into `bank_rad` (= screen roll, `rail.rs:771-781`). Fix: bank from heading turn rate over a fixed +-12 m arc window with a dead zone; regression tests in the report. | csg-rs | `audit/2026-11-24/rail_roll_jitter.md` |
| `DEMO:VIEWS` | nv-demo display modes (operator, via csg-rs #744). **Done (2026-11-24)**: CSG and hidden-line Wireframe (native resolution, GPU contours, flat grey + AO), drawn by the generic GPU overlay renderer (`overlay.rs`, record contract `shaders/overlay_segment.cuh`) with line LOD and temporal stabilisation. **Open**: (1) Voxel view: csg-scene at `0fb1087` has no voxel / ESDF API; add the selector entry when csg-rs ships it. (2) Line LOD uses the instance compound diameter; the operator asked for the hit part's diameter (needs a per-part bound on the device; small details of large objects, e.g. windows, still change shape far away). (3) Done (2026-11-24): the Selected scope reads the GPU pick selection (device selection table). (4) Object axes and selection bounds done (2026-11-24); next providers: actor skeletons, rail path, camera frustum; csg-rs exact feature curves (`OverlayCurvePod`, replaces the local record). (5) Text labels are not implemented (no provider needs them yet). | lead (voxel / exact curves: csg-rs) | chat #744; `crates/nv-demo/src/overlay.rs` |
| `GPU:FRAMEPLANS` | Run csg-scene `CrowdFramePlan` / `TrafficFramePlan` / `CameraFramePlan` with one generic runner on one stream, so nv-demo stops running its own private copies of the simulation. Why: nv-demo launches 1 of 22 csg-rs sim kernels; its private `prepare_crowd_steering_and_grounds` (hardcoded foot offsets 0.22, DEM ground) causes feet sunk in sidewalks and one-frame leg jumps; traffic steps on the CPU; csg_scene_host keeps a private pack_visible copy. Runner contract (csg-rs #811-#825): typed `Arg` enum, buffer table (elem size, count rule, class incl. `External`), `GridRule`, plan version + kernel source hash checked against our PTX, device-resident buffers, uploads only dirty host tables, zero per-frame allocation, no host sync mid-frame; readback on demand (SDG export, Freeze Pose); camera composed on the device (`camera_compose`, `CameraRecordPod` 64 B, right = fwd x up, read back async every frame for culling/HUD). Then delete the private crowd prep, CPU traffic step and pack_visible copy. | lead (runner) + csg-rs (plans) | chat #809-#825 |
| `OVERLAY:NEXT` | Overlay providers on the GPU, built into the overlay layer rather than added as one-offs: object axes from the device instance xforms (`WorldScene::instances()` / `dynamic_instances()`), actor skeletons from `WorldJoints` + `ArchetypeSkeleton { joints: JointDef, world_joint_index }`, rail path, bounds, frustum; picking through a 32 B pick record written by the primary launch. **Done (2026-11-24)**: object axes provider (`ovl_axes`, device instance records, constant 40 px, filter / cull, selection highlight with bounds) and GPU picking (`pick.rs`, 32 B record, async readback, click / Shift / Ctrl model shared with warpbro-rs / squarebob-rs, host CPU picking deleted). **Open**: skeletons (csg-rs `ArchetypeSkeleton`), rail path, frustum, `OverlayCurvePod`. Switch to csg-scene `OverlayCurvePod` (160 B, P36 exact feature curves, common per-pixel K7 draw) when it lands and delete the local record; K6 visibility = our OptiX raygen; visibility bit 0x80 = `LINES_OCCLUDER`, render rays use 0x7F. Why: the operator wants axes and skeletons for all objects, systemic and GPU-first. | lead + csg-rs (P36 g-lines) | chat #820-#839, #851-#854 |
| `P37:SWEEPS` | csg-rs CSG-native curves + `SweepTile` (line / arc / exact `ARC_HELIX`, upright-frame sweeps, analytic chain-link). Why: kerbs, sidewalks, walls, seawalls, cornices, pipes and rails are hand-assembled box chains today (about 750-1500 instances/km). nv-demo side: (1) **done (2026-11-24)**: per-hit-group IS-invocation counters + RT ms behind a debug flag (`--is-counters`, Flight stats toggle) and the fixed camera set `nv-demo --bench-views p37 --out views.csv` (gundo_osm bookmark + refinery, street grazing, top-down, Marina street level + grazing; one CSV row per view, see `crates/nv-demo/AGENTS.md` Benchmark), giving an A/B per migration wave; (2) payload slot 9 = packed half2 ray cone {width m, spread rad} from the shared header; non-opaque SBT entry only for MESH records (`sbtIndexOffsetBuffer`), shadow transmittance (1-c). | csg-rs + lead | chat #845-#857 |
| `DEMO:FILTER` | Viewport category filter (operator). **Done (2026-11-24)**: semantic groups in the toolbar eye dropdown, `hidden_semantics` u64 rejected in every CSG intersection program via `csg_instance_semantic` (all ray types, per part), classic instances masked on the host by the same rule. **Open**: Vegetation / Terrain features rows need csg-rs P41 (`VEGETATION = 20`, `TERRAIN_FEATURE = 21`; trees and scrub are FURNITURE today); the OSM class section (osm-rs `RoadKind` / `BuildingKind` / ...) and the OSM id / kind of a pick need csg-scene per-instance OSM provenance (csg-rs P42: `{OsmFeatureClass, kind u16, osm_id}` with a device copy) and show "needs csg-scene OSM provenance" until then. Semantic census (packed parts carrying each id): gundo ids 1-19 (no 0); gundo_osm ids 1-6, 12-19 (no 0, 7-11: no vehicles packed at build time); marina ids 1-6, 9-12, 15-19; no vegetation class anywhere (trees are FURNITURE, id 9, absent from gundo_osm). | lead + csg-rs (P41, P42) | `src/semantic_filter.rs` |
| `FILTER:FAST` | Category-filter fast path (agreed with csg-rs #870-#875). **Done (2026-11-24)**: per instance an effective semantic mask M (`semantic_filter::instance_semantic_mask`, the single source of M) is classified against the hidden mask H (`classify`): `M & !H == 0` = hidden in hardware (visibility mask keeps only `0x80` = `LINES_OCCLUDER`; render rays trace with `0x7F`), `M & H == 0` = unaffected (bit clear in the per-frame device bitset `LaunchParams::csg_sem_partial`, the intersection program skips the reject test), else partial (per-hit reject as before). Classic instances use the same function. State is recomputed only when H or the packed instances change; a filter-only change re-emits the IAS instances as one `UPDATE`. **Open (csg-rs P41)**: the pinned csg-rs `0fb1087` has no `CompiledLibrary::semantic_masks`, so M is exact only for instances with a semantic override (flags bits 16..24, i.e. actors) and all bits (unknown, always partial) for every other instance; crowd bones stay unknown because the GPU crowd scatter drops the override. Hence no speedup yet on city scenes (see `CHANGELOG.md` numbers). When P41 lands, replace the body of `instance_semantic_mask` with `csg_scene::instance_semantic_mask(flags, library.semantic_masks[compound_id])` (pass the compound mask) and re-run `--bench-views p37` / `--bench-rail road_cruise` with H = {Vehicles, Industrial}. | lead (+ csg-rs P41) | `crates/nv-demo/src/semantic_filter.rs` |
| `LOD:SIZE` | Size-based LOD/culling (operator idea, 2026-11-24): distant SMALL objects that cover almost nothing on screen should drop first — select LOD / cull by projected screen-space size (bounding-sphere radius / distance vs pixel threshold at the current FOV and resolution), not by distance alone. Check what the existing "Distance & SSE LOD" path in csg-scene actually computes, then measure the RT-pass effect on the rail benchmark. | csg-rs (LOD selection lives in csg-scene packing) + lead (measure) | operator note |
| `GPU:D04` | Cryptomatte id per pixel on road tiles needs the `csg-rs` `part_id` road-UV packing decision. | csg-rs | `10_demo_gpu.md` `E01`, `E02` |
| `CSG:RPN-FLAG` | `csg-rs` RPN stack overflow flag. | csg-rs | `10_demo_gpu.md` `E01`, `E02` |
| `CSG:R00` | GPU crowd steering moves out of `nv-demo` `cuda_crowd.rs` into a csg-rs-owned CUDA kernel; branch review by the lead pending. | csg-rs | chat `Mullis-16-3934` |
| `CSG:R00b` | GPU traffic; branch review by the lead pending. | csg-rs | chat `Mullis-16-3934` |

### Python and docs

| ID | Description | Owner | Pointer |
| --- | --- | --- | --- |
| `DPY:DLPACK-WAIT` | Exact `__dlpack__` wait: `cuda-core` keeps pending write events private (`allocation.rs:33` at `348895e`); today a context-wide synchronize. | lead (blocked on external `cuda-core`) | `wave_final.md` DPY |
| `DPY:P17` | Stubs model submodules as classes; needs an `xtask py-stubs` generator change. | lead | `wave_final.md` DPY |
| `W7.1` | Narrowed: `crates/nv-demo/AGENTS.md`, README and the docs-site pages are done (`0714f65`, `a33d21f`); stale plan links fixed in the commit that applies this report. Not covered by any report: `optix` module docs (`optix.rs:19-20`, `accel.rs:25-26`, `OPX:DOC1`, `OPX:A7`), systemic defects in `## Traps`, re-reading every cited constant. | lead | `plan2_full.md` Documentation drift |

### Old backlog (from the former `PLAN.md`, not re-verified)

| ID | Description | Owner | Pointer |
| --- | --- | --- | --- |
| `BK:SANITIZER` | P2: `cargo test --workspace -- --ignored` plus `compute-sanitizer` on live GPU paths (`cufft` `ManyDesc` / `Scratch` growth, `cudnn`, `cufile`, `optix`). The probabilistic scratch-race tests in `cufft` and `cusparse` have not run under the sanitizer. | lead | `audit/history/PLAN_milestones_M1-M6.md` |
| `BK:SPAN-TRAIT` | P3: consolidate the sealed `DeviceSpan` / `DeviceSpanMut` contracts of `cudnn`, `nccl`, `cufile` into `cuda-core-ext`. | lead | same |
| `BK:PY-DTYPE` | P3: reuse `dlpack::find_dtype_entry` in `python/nv-py/src/core.rs` dtype inference. | lead | same |
| `BK:PROFILE` | Narrowed: `nv-demo` frame time is profiled (`perf_log.rs`, `NV_DEMO_PERF_CSV`, see `PERF:RT`). Wrapper hot paths (`criterion`, A/B release) are not. | lead | same; `wave_final.md` Performance |

Item count: 27 open rows: 8 verification/review, 5 build/release/CI, 1 SSOT, 6 `nv-demo` (2 lead, 4 csg-rs), 3 python/docs, 4 old backlog.

Closed by the csg-rs `fa5e170` adoption (`9b6e178`): `APP:F24` (the WGS84 HUD reads `SceneDescriptor::geo_origin`; the per-scene table in `ui.rs` is deleted; synthetic scenes show no coordinates).

Closed by the final wave (`ee08ade..a33d21f`, 7 items, plus `W5.4` split into `CI:EXEC` and `DENY:LIC`): `W5.2` (`208e628`; catalog-driven `xtask`, `dlssd`, companions, manifest and prebuilt marker, `release.yml` exit codes), `W4.3` (`0508d75`, `0714f65`; geo-origin part stays as `APP:F24`), `DEV:CI` (`a7b7b49`, written, not executed), `DPY:C20` (`473e1fe`, nine classes), `GPU:D07` and `GPU:D08` (`0714f65`), `APP:F27` (`0508d75`, `0714f65`). Closed earlier and absent here: `W4.1`, `W1.1..W1.6`, `W2.1..W2.3`, `W3.1..W3.6`, `W4.2`, `W5.1`, `W5.3`, `W6.1`, `W6.2`, GPU:D03, GPU:D21, RND:R10, RND:H13, APP:F19, `anim_range` sliders, async params upload, `nv-wgpu` submit timeout.

## 3. Decisions (ADR)

Newest decisions last. Operator delegated design decisions to the lead on 2026-11-23: decide, record, push after each green gate.

| # | Context | Decision | Rejected alternatives and why |
| --- | --- | --- | --- |
| 1 | Systemic 1: async GPU work frees buffers early. Measured on GPU: only a stream-ordered buffer (`uninitialized_async`) freed while ANOTHER stream still writes it is corrupted (20/20; synchronous and same-stream buffers 0/20). | Targeted: `curand` (caller buffers) leased via `cuda_core_ext::lease::enqueue_leased`; everything else fixed by ownership or same-stream checks; every `python/nv-py` binding leases. | Lease every wrapper entry point: about +2 us per call and new `NOT_READY` errors for a hazard that only exists for stream-ordered buffers crossing streams. Fork or upstream change to `cuda-core`: unnecessary, the lease API is already public. |
| 2 | Vendored, diverged copy of `camera-rs` in `nv-demo` hid 133 warnings behind a blanket allow. | Depend on upstream `camera-rs` (`9109108`); nv-demo-only feel opt-in through `NavExtras` / `CameraDriver::set_nav_extras`. `altitude_speed_multiplier` stays in `nv-demo` (`ui.rs`). | Keep the vendored copy: 29 of 33 files diverged, upstream fixes missing. Change defaults upstream: `camera-rs` has 8 other consumers. |
| 3 | `device/` test harness: two different `cuda-core` crates in the graph, none of the 15 GPU cases could build. | Keep NVIDIA upstream for the main workspace; add `[patch."https://github.com/NVIDIA/cuda-rust.git"]` in `device/Cargo.toml` redirecting to fork `ssoj13/cuda-rust_b` (`42cc04e`) inside `device/` only. Result: one `cuda-core`, 15/15 pass. | Re-export `cuda_device` from `optix_device` (`DPY:D3`): impossible, `ptx_asm!` expands to `cuda_device::ptx::...`; documented instead. Move everything to the fork or to NVIDIA only: the device code needs unmerged `cuda-oxide` patches (`ptx_asm!` up to 256 inputs, callable rooting, 64-bit pointers, named statics). |
| 4 | `python/nv-py` holds the largest P0 count. | Fix after Wave 1 in the wrapper crates (most of it disappears), then Wave 6. | Fix first: duplicates work that Wave 1 removes. |
| 5 | `nv-wgpu` raw Vulkan work could not be synchronised with wgpu. | Record into a `wgpu` command encoder and submit through `wgpu::Queue::submit`; own command runner, global submit mutex and dedicated barrier queue removed (`with_vulkan_barrier_queue` removed). | Keep raw `vkQueueSubmit` with a private lock: cannot be synchronised with wgpu's own submissions, undefined behaviour under the Vulkan spec. |
| 6 | `optix` IAS children built on another stream. | Require and check one stream (`Accel` records its build stream, `check_children`); document. | Lease every child in `instances_into`: per-child cost on the per-frame path, new dependency, `nv-demo` uses one stream. |
| 7 | `ngx` thread safety. | One process-wide mutex around every NGX call, `Send`/`Sync` kept. | Make `NgxVulkan` `!Sync`: does not cover the static `required_extensions` path Python calls from several threads. |
| 8 | `cublas` workspace dangling pointer and silent reset by `cublasSetStream`. | The handle owns the workspace buffer while set and refuses to switch streams; `clear_workspace` restores the default pool. | Two separate patches for the pointer and the reset. |
| 9 | cuBLASLt / descriptor raw attribute setters bypass typed validation. | `unsafe fn` (also `set_epilogue`, `set_bias_batch_stride`). | Re-validate every raw attribute: the attribute set is open-ended. |
| 10 | `nvenc` B-frames: `NV_ENC_ERR_NEED_MORE_INPUT` was an error. | `encode_picture` returns `EncodeOutcome { Ready, NeedMoreInput }`. | Keep status 17 as `Err`: the header calls it non-fatal and forbids locking the bitstream after it. |
| 11 | `cusparse` SpSV/SpSM/SpGEMM `scratch` parameter and dead `Budget` / `Unsupported` variants. | Scratch parameter reduced to a stream check, then removed in Wave 1b; dead variants removed together with the `python/nv-py` matches in Wave 6. | Keep a parameter that no longer supplies memory; remove variants early (breaks `python/nv-py/src/video.rs`). |
| 12 | Context identity: crates compared contexts with `Arc::ptr_eq`; `default_stream()` returns a fresh `Arc`; two `Context::new(0)` retain one primary context; DLPack imports were rejected. | Compare by value: `cuda_core_ext::same_context` / `same_stream` (native handle + context); crate-local copies deleted or delegating. | `Arc::ptr_eq`: rejects valid buffers and identical contexts behind different `Arc`s. |
| 13 | Teardown errors were mapped to an invented `CUDA_ERROR_UNKNOWN` in a sticky slot (about 20 sites); four `Scratch` implementations. | Typed teardown sink `cuda_core_ext::teardown`; one `Scratch` in `cuda_core_ext::scratch` (`cufft` wraps it; `optix::Scratch` kept separate for its temp/output accounting). | Keep per-crate sinks and scratch types: already diverged budget semantics. |
| 14 | Two cuDNN 9 builds cannot share a Windows process; the `import torch` `WinError 127` cause was not established. | cuDNN adopts an already-loaded copy; documented order: import `torch` before `nv_rs` (`python/nv-py/tests/conftest.py` imports torch first). | `reject_foreign_copy` from the root gate: superseded; the pytest failure was test order, not the loader. |
| 15 | Prebuilt bindings silently replaced failed generation (`LDR:L-01..L-03`). | No prebuilt fallback after a generator failure or when the SDK is present; header-hash warning; runtime version gate; single `nv_loader::catalog`. | Sticky prebuilt fallback: hides generator bugs and ABI drift. |
| 16 | `csg-rs` `24b22f5` derives a crowd-bone flag from the world's `DynamicKind` tag on every pack. | `nv-demo` identifies packed crowd bones by `inst_flags::CROWD_BONE`, owned by `csg-scene` (`a637df6`); GPU scatter, incremental repack and prefix packer test that bit. | Identify crowd bones by a `user_id` range (the previous host-side convention). |
| 17 | Classic geometry had camera-only motion vectors (`GPU:D03`); SDG fractal params missing in probe launches (`GPU:D21`). | New launch fields (`classic_motion_count` at 636 and the previous-transform pointer): `LaunchParams` / `Params` ABI grows 640 B to 648 B, checked by compile-time Rust `assert!` and CUDA `static_assert`. | Not recorded in `wave_nvdemo_followups.md` beyond the layout; none stated. |
| 18 | Alpha code of this repository. | No backward-compatibility shims, adapters or old codepaths for our own alpha code (global rule); removed variants and APIs are replaced, callers adapted. | Transitional shims kept "for now". |
| 19 | Operator process rules. | Lead model does not run clippy / tests / log triage (cheap subagent does); new tests are verified by reading them, no revert-and-rerun mutation runs; one gate per wave, workers edit disjoint files and never run cargo; push after each green gate. | Mutation reruns and per-worker builds: cost and `.cargo-lock` contention. |
| 20 | Prebuilt staging inferred completeness from file times and the inventory; that rejected every `cupti-sys` build (its `build.rs` appends a type after `generate`). | `nv-bindgen` writes `<lib>_manifest.json` last (`Origin::{Generated, Prebuilt}`, promised files must exist); `xtask prebuilt` stages exactly the files of a `generated` manifest and rejects out dirs without one; `xtask stage-dlls` uses `nv_loader::catalog::ALL` with typed `StageError` and companion DLLs. | Keep mtime comparison and the hand-kept `SPECS` table: false rejections and a second DLL list that drifted (`dlssd` was not shipped). |
| 21 | Missing optional SDKs on Linux targets (`L-32`). | One policy: NCCL and cuFile are required on Linux targets; NGX stays a warning (gated `cfg(has_ngx_sdk)`, the wrapper has a stub path). | Warn everywhere: hides a missing library until runtime. |
| 22 | C enum constants are `i32` under MSVC and `u32` under GCC (`unnecessary_cast` on the Linux target). | Bit-preserving `u32::from_ne_bytes(x.to_ne_bytes())`; one `optix::INSTANCE_FLAG_DISABLE_ANYHIT` for `nv-demo`. | `as u32` with `#[allow]`: breaks the clippy gate on one of the targets. |
| 23 | `nv-wgpu` waits polled with `timeout: None`; failures were printed. | `wait_submission_bounded` (`VULKAN_SUBMIT_TIMEOUT`), typed errors, drop failures recorded in `cuda_core_ext::teardown` under `Library::NvWgpu`; `nv-demo` prints the teardown sink once at exit. `ExternalSemaphore` drop stays a driver-side context sync (cannot be bounded from Rust). | Keep indefinite waits in Drop paths: a stalled driver hangs process exit. |
| 24 | Python handles had no deterministic release. | `close()`, `is_closed`, `__enter__` / `__exit__` on nine classes; state in `Arc<Mutex<Option<State>>>`; close waits for in-flight calls, drains the stream, still releases on sync failure and returns the error; idempotent; use after close raises `NvInvalidArgumentError`. | Rely on `Drop` only: not deterministic from Python. |
| 25 | RT pass is 98 % of the frame; the only available lever is the shader, which needs Nsight Compute and can change the image. | No speculative shader change; recorded as `PERF:RT`. | Host-side micro-optimisation: under 3 % of the frame, no measurable gain. |
| 26 | `cargo deny` licenses: crates without a `license` field. | Do not invent a license; per-crate exceptions only for real third-party licenses (`clipboard-win`, `error-code`, `tiny-keccak`, `smartstring`, `epaint_default_fonts`); the three operator crates are `DENY:LIC`. | A blanket `clarify` for the operator's crates: asserts a license nobody chose. |

## 4. Done history

One line per wave or milestone; the full text of each is in the archive.

### Audit waves (`audit/2026-11-23/plan2_full.md`)

- Audit (`8af9860`): 12 slices, 7 systemic defects, P0 / P1 ledgers; lead re-read 10 top items, 10 matched, `MED:DEC-1` downgraded P0 to P1.
- W1.1 (`8e84de4`, `05efa7e`) leases targeted (ADR-1). W1.2 context binding. W1.3 `Send`/`Sync` soundness (`cufft::Plan`, `ngx::VulkanSession`, `nccl::GroupGuard`, `nvdec`, `nvofa`, `nvjpeg`). W1.4 workspace ownership. W1.5 validation gaps. All `05efa7e`.
- W1.6 teardown sink and one `Scratch`: Wave 1b `1467778`.
- W2.1 `optix` (`InstanceBuffer` fence, tiled in-place denoise, grow failure, one-stream rule). W2.2 `nv-wgpu` encoder-based submit (ADR-5). W2.3 `cuda-core-ext` cubemap mips, mip clamp, semaphore drop. All `05efa7e`.
- W3.1..W3.6 `nv-demo` correctness (Fatal events, GrowDeviceBuffer ordering, Traffic toggle, SDG burst, denoiser fallback loop, depth/motion per projection, capture and persistence, frame loop): `4f457d0`, `gate_root_w3_w5.md`. `GPU:D04` blocked on csg-rs.
- W4.1 `camera-rs` (ADR-2): `a4eb85e`. W4.2 single `nv_loader::catalog` (`7a62138`, gated `4f457d0`).
- W5.1 `nv-bindgen` rules, runtime version gate (`7a62138`, `4f457d0`). W5.2 mostly (`stage-dlls` fails on missing libs, completeness not mtime, atomic writes). W5.3 `bootstrap.py` (`a246a6a`).
- W6.1 `device/` (`c04d692`). W6.2 `python/nv-py` (`8f7af2f`, `1467778`).
- Gate counts: `05efa7e` clippy clean, 379 passed (follow-up gate 375 / 0 / 203 ignored); `4f457d0` 467 / 0 / 204; `1467778` 467 / 0 / 205.
- Behaviour changes callers notice (per crate, Waves 1-2): see `plan2_full.md` "Behaviour changes callers will notice".

### Follow-ups and adoptions

- Wave 1-2 follow-up: `MED:CMP-4` bitcomp header validation (magic `NVZ\0`, 32-byte header, shortest stream 56 bytes), `nvjpeg` single retrieve, hardware-decided tests turned into regression tests, `cublas` emulation-desc retention, `optix` one-stream check.
- csg-rs adoption (`csg-rs` `24b22f5`): `ff1b178`, `0a96f69`, `76c02d9`, `dee0d00`, `a637df6`, merge `ea2161d`.
- nv-demo follow-up wave (`ee08ade`): see Status table.
- Final wave (`208e628`, `0508d75`, `0714f65`, `473e1fe`, `a7b7b49`, `a33d21f`; `wave_final.md`): see Status table and ADR 20-26.

### Old `PLAN.md` milestones (full text `audit/history/PLAN_milestones_M1-M6.md`)

- M1 (`03021de`, `033f1e1`): `DriverError(700)` via `random_vertex_access`, typed errors instead of `unwrap()` panics in `nv-demo`.
- M2 (`033f1e1`): 48 `cublasLt*` and 19/19 `cusolverMg*` entry points wrapped; coverage mapping and `-p xtask` in CI.
- M3 (`ae7c323`): FFI-1..4 (cuFFT scratch retention, `ManyDesc`, sealed `DeviceSpan`, `exec_c2r` `&mut`), PY-1..4 DLPack safety, INFRA-1/2, numeric DLL version sort.
- M4 (`60f9bb4`): manifest cleanup, nightly verification, README / CHANGELOG / docs sync.
- M5 (`py-stubs`): `cargo xtask py-stubs <generate|check>`, PyO3 macros, stub-sync test wired into CI and `bootstrap.py`.
- M6 (`5b4fcf8`, `2c557a6`, `d3ac3f1`, `8e55b4d`): pure-CSG worlds, articulated crowds, 110-variant Inspector, `Motion` AOV, atomic SDG JSON export, 60 Hz paced UI, OptiX `7999` diagnostics.

### `plan1.md` (nv-demo bughunt and acceleration, closed; `audit/2026-11-23/plan1_nvdemo.md`)

- 2026-11-23, baselines `nv-rs` `5a58bab`, `csg-rs` `539f244`: 10 P0 correctness bugs (road tile `t_box_in`, double-negated normals, `+Inf` bloom NaN, `rr_last_exposure` NaN sentinel, PQ10 recording stall, lockstep input loss, Orbit dropdown, cursor grab, `SaveJpeg`, crowd desync) fixed in Wave 1 (traps kept in root `AGENTS.md`, fixed in `02c1cd9`).
- Waves 2-3 (multi-AABB GAS, spill elimination, SER and shadow SBT, AOV early-out, half-res bloom, static-tail split, GPU crowd steering, caches, UI hygiene) all ticked in that file; the "typed payloads" statement in it is wrong (payloads are plain 8-word, `GPU:D31`).

## 5. Refuted and narrowed findings

No code change for the refuted part. Per-item loader verdicts (`LDR:L-01..L-37`) and the `nv-demo` / `xtask` ledgers exist only in worker notes and the commit message of `7a62138`; no per-finding refuted list was recorded for them.

- `BLA:LT-10`: `cublasLtMatmul` itself rejects an undersized workspace (`cublasLt.h:1017-1019`).
- `SOL:S2`: Mg `lwork` is `unsafe fn` with an explicit contract. `SOL:S4`: the module makes no zero-allocation promise.
- cuDNN workspace ownership: cuDNN takes the workspace per call, `Handle` holds none.
- `SYS:NGX-5`: `DlssOptimalSettings` fallbacks mirror the SDK helper. `SYS:NVAPI-1`, `SYS:NVML-2` (no init needed for error strings), `SYS:NVML-1` (literal correct and intentional).
- `SYS:NVRTC-1`: log-fetch error is typed. Null `CStr` checks in `cupti`/`nccl`/`nvrtc`: all seven sites already check.
- `MED:DEC-5` mechanism: `vidLock` is unrelated to the decoder (`cuviddec.h:306`). `MED:ENC-8` for CUDA arrays: pitch is defined differently.
- `MED:CMP-9` compression-level part: levels above the maximum are accepted by design (`cascaded.h:157-165`); nvCOMP internal scratch never crosses streams.
- `OPX:PF1`: SBTs are built only at pipeline creation; stream-ordering them would create the measured cross-stream hazard.
- `IOP:NW-13`: unreachable once the dedicated barrier queue is removed.
- Systemic 1 inside wrappers: scratch buffers in `cublas`, `cudnn`, `cusolver`, `nvcomp`, `optix` are allocated and used on one checked stream; real cross-stream cases were `cusparse` SpSV/SpSM/SpGEMM (fixed by ownership) and caller buffers (`curand`, leased).
- Hardware-decided: `IOP:CE-06` REFUTED (driver accepts DEDICATED for a D3D12 heap import); `MED:DEC-3` not reproduced (24 frames, min PSNR 57.55 dB); `OPX:D2` no corruption on this driver (still outside the OptiX contract); `BLA:FFT-03` cuFFT itself rejects mixed NULL / non-NULL embeds, wrapper matches; `MED:CMP-4` CONFIRMED and fixed; `DPY:D2` PLAUSIBLE then confirmed (`got 3060 want 65517`) and fixed.
- A worker fix reverted by the gate: `MED:JPG-6` (`reserve` + `unsafe set_len` returned uninitialised heap because a second retrieve succeeds without writing); `resize` + `truncate` restored.
- Narrowed: `SYS:CUPTI-3` (field offsets identical across record versions; the reference to a larger struct was formal UB, fixed). `BLA:SEAM-01` / `SOL:X1` / `MED:X-1` P0 ratings do not hold for callers using only safe constructors.
- Wave 3 / 5 / 6 / 1b: `DPY:D3` fix direction impossible (ADR-3); `DPY:D7` doc-only (stack-local `Footprint2d` works under 64-bit pointers); `DPY:D9` not a wrapper bug (driver 621.24 returns key 0 for current-hit vertices, test accepts both); "`spin_speed` unreachable in `ui.rs`" refuted (reachable at `ui.rs:805`); "`cudnn` is the only crate using `Arc::ptr_eq`" refuted (`same_stream` and the `cublas` handle did too); `probe_libraries` key changes and `NV_RS_DEBUG` meaning: nothing in `python/` depends on them; the loader was not established as the cause of the `import torch` `WinError 127` (ADR-14).

- Final wave (`wave_final.md`), refuted as open: `L-31(b)` (zlib failure already in `diag::record_event`), `L-33`, `L-24`, `L-25`, `L-28`, `L-29`, `L-34`, `L-35`, `L-37` (already fixed in Wave 5), `APP:F27` for `nv-demo` (every `device.poll` already waits one submission with `GPU_COPY_WAIT_TIMEOUT`), `CAM:D09`, `CAM:D12`, `RND:H5`, `RND:H6` (already fixed), `IOP:NW-20` part (a) (no runner left), `MED:X-2` entry-macro and nvdec "UNKNOWN" parts, `SE-05` (already fixed in `sys_ext.rs`); `P14` accepted as valid (legacy capsule allowed); `device/.optix_device_tests.oxide-pipeline.lock` already untracked and ignored. Partly confirmed: `GPU:D07`, `GPU:D08`, `APP:F24`. Confirmed by measurement: Linux-target builds got MSVC `vadefs.h` / `vcruntime.h` (fixed: `system_include_dirs` only for windows-msvc, `cross_include_dir`).

## 6. Verification rules and gates

Rules: every audit finding is a claim until the code and the native header are re-read, and, where reading cannot decide, measured on hardware. A test that asserts a hazard must be backed by a hardware reproduction (three such tests were removed or inverted). No `#[allow]` to silence warnings, no swallowed errors, no shims. Preserve CUDA context ownership and stream ordering in wrappers. GPU tests marked `#[ignore]` need explicit execution; compile success does not verify driver behaviour, stream ordering, callbacks or interop. A successful root workspace check does not validate `device/` or `python/`.

Gates (one agent per workspace, one build at a time, never concurrent with other cargo runs):

| Workspace | Commands |
| --- | --- |
| Root | `cargo clippy --workspace --all-targets --message-format=short -- -D warnings`; `cargo test --workspace --quiet`; ignored GPU tests of touched crates: `cargo test -p <crate> -- --ignored --test-threads=1` (`nvcomp` needs `NVCOMP_ROOT=.sdk\nvcomp`); `coverage check` (regenerate `COVERAGE.md` if stale) |
| `nv-demo` run | `cargo run -p nv-demo --release`: the shader compiles at app start, so a green build is not enough; check it starts and renders (Gundo / Marina with DLSS RR and the CUDA crowd) with empty logs |
| `python/` | inside `python/`: `cargo clippy --all-targets --message-format=short -- -D warnings`; `cargo test --lib`; `cargo test --lib -- --ignored --test-threads=1`; `maturin develop` then `pytest python/nv-py/tests` with `NVCOMP_ROOT=.sdk\nvcomp` (tests import `torch` first); `cargo run -p xtask -- py-stubs check` |
| `device/` | inside `device/`: `cargo oxide build --arch sm_86`; `cargo oxide run --arch sm_86` (15/15 GPU cases); `cargo clippy --workspace --all-targets` |

Known environment results: NVAPI sees 0 connected displays on this machine (one pytest and the `nvapi` hardware test skip HDR/VRR only there). Nothing was run on Linux.

## Archive

- `audit/2026-11-23/plan2_full.md`: full workspace bughunt and remediation plan (findings, P0 ledger, decisions table, execution record, refuted findings).
- `audit/2026-11-23/plan1_nvdemo.md`: closed `nv-demo` bughunt and acceleration plan.
- `audit/2026-11-23/TODO_final.md`: last execution tracker.
- `audit/history/PLAN_milestones_M1-M6.md`: old master plan with milestones M1..M6 and backlog.
- `audit/2026-11-23/*.md`: slice ledgers `01..12`, gate and wave reports.
