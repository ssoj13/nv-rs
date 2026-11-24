# csg-rs `fa5e170` adoption in `nv-demo` (2026-11-23/24)

Base `main` = `9bb2bae`. Code commit `9b6e178`; this report and the plan / changelog / AGENTS updates are in the docs commit that follows it. RTX 3080 Ti, CUDA 13.3, OptiX 9.1, Windows 11. Every csg-rs claim below was checked against the cargo checkout `~/.cargo/git/checkouts/csg-rs-3a6ed3e588aad103/fa5e170` (NOT the local `cglibs/csg-rs` tree, which carries uncommitted csg-rs work: there `csg_eval.cuh` is already split into `csg_prelude.cuh` + `csg_bk_spans.cuh` and `csg_device::CUDA_SOURCE_FILES` exists; at `fa5e170` neither does).

`cargo update -p csg-core -p csg-actors -p csg-scene -p csg-device -p csg-scenes`: all 23 csg-rs crates `24b22f53 -> fa5e1707`; new transitive deps through `osm-rs` (now pinned `rev=12ca832`): `ureq` 3.5.2, `rustls` 0.23.46, `ring` 0.17.14, `webpki-roots` 1.0.12, `http`, `httparse`, `png` 0.17.16, `subtle`, `untrusted`, `utf8-zero`, `zeroize`. `cargo deny` was not re-run.

## Change items

### 1. Evaluator caps, span window clipping, `FIRST_HIT_UNION`
- Verified at `fa5e170`: `csg_core::abi::MAX_SPANS = 10`, `MAX_STACK = 12`, `MAX_REPEAT_DEPTH = 2` (`abi.rs:26-32`), `csg_eval.cuh:30-32` defines the same; `compound_flags::FIRST_HIT_UNION = 1 << 4` (`abi.rs:161`); `compile.rs:495-498` sets EITHER `PURE_UNION` (parts disjoint) OR `FIRST_HIT_UNION` (parts overlap / touch), never both.
- GAS layout: the only place nv-demo chooses the multi-AABB GAS is `compound_gas_plan` (`crates/nv-demo/src/csg_scene_host.rs:139`); it already required `PURE_UNION` (`:156`), so a `FIRST_HIT_UNION` compound falls to `GeneralRpn` (SBT 8, `hit_csg`), where `csg_trace_compound_val` / `csg_trace_shadow` pick first-hit or the exact span path per ray. Doc comment added (`:135-138`). The old test built an OVERLAPPING box+sphere+cylinder union and expected `MultiAabbUnion`; under `fa5e170` that compound is `FIRST_HIT_UNION`. Test `csg_gas_build_opts_disables_compaction_and_multi_aabb_plans_pure_union` now builds the union twice: separated parts (`dx = 5`) -> `PURE_UNION` -> `MultiAabbUnion { count: 3 }`, SBT 10; overlapping parts (`dx = 0.5`) -> `FIRST_HIT_UNION`, no `PURE_UNION` bit -> `GeneralRpn`, SBT 8.
- Caps in `shaders/raytracer.cu`: it defines no caps of its own; it includes `csg_eval.cuh` and calls `csg_trace_compound_val` (`:3187`), `csg_trace_shadow` (`:3227`), `csg_intersect_prim_idx[_shadow]` (`:3250`, `:3294`); all exist with these signatures at `fa5e170`. Nothing to change.
- PTX cache key: `raytracer_ptx_inputs` (`renderer.rs:1432`) now hashes `csg_device::CSG_EVAL_CUH` (the string csg-rs documents as the device-source cache key) instead of reading `csg_eval.cuh` from disk. At `fa5e170` both are the same bytes; after csg-rs splits the header into fragments the umbrella file's bytes stop changing, the constant does not. `build.rs` still reruns on `csg_eval.cuh` only (`CUDA_SOURCE_FILES` does not exist at `fa5e170`; switch to it when csg-rs ships it).
- Stack sizes: see the section below.

### 2. `SceneModule::build -> Result<SceneBuilt, SceneBuildError>`
- Verified: `scene_module.rs:3336` (trait), `SceneBuildError::{Compile, Crowd, Generate}` (`:3079`), every build ends in `SceneBuilt::checked()`.
- Production call site (only one): `csg_scene_host.rs:2401-2403` `rebuild_scene` maps the error to `"CSG scene '<name>' failed to build: <error>"` and returns it; `CsgGpuHost::sync` propagates it and `renderer.rs:4888-4893` emits `RenderEvent::StatusMessage("CSG sync error: ...")` once per distinct error and repacks without the scene (existing path, unchanged). No `expect` in non-test code.
- Test call sites (`.expect("scene builds")`): `csg_scene_host.rs` 13 sites (`:2595`, `:2671`, `:2711`, `:2755`, `:2800`, `:2945`, `:3007-3008`, `:3114-3115`, `:3293`, `:3383`, `:3507-3508`, `:3558`, `:3598`), `camera_rail.rs:947`, `:1031-1036`, `cuda_crowd.rs:1897-1899`. 16 in total.
- `build_gundo_osm_scene` / `build_marina_scene` / `build_sf_house_of_ai_scene`: nv-demo never called them (only `default_scene_registry()`), nothing to delete.
- Two further `fa5e170` API changes found by the compiler (test code only): `SceneTraffic::actors` and `soa_vehicle_count()` are gone ("one traffic path") -> `soa_city_traffic_detected_and_stepped_by_host` asserts `vehicle_count() > 0` (`csg_scene_host.rs:2946-2949`); `SceneDescriptor` gained `geo_origin` -> `main.rs:6439` test descriptor sets `geo_origin: None`.

### 3. APP:F24 `SceneDescriptor::geo_origin`
- Verified: `GeoAnchor { lat_deg, lon_deg, alt_m }` (`scene_module.rs:323`), `geo_origin: Option<GeoAnchor>` (`:351`); `gundo_osm`, `marina`, `sf_house_of_ai` carry `Some(...)` from their `bookmark.json`, `gundo` and `inspector` carry `None`.
- Deleted `ui::scene_wgs84_origin` (the hard-coded lat/lon table). New `active_geo_origin(descs, csg_mode)` (`ui.rs:86-94`) returns the selected descriptor's `geo_origin`; the HUD pill (`ui.rs:1399-1401`) shows coordinates only when it is `Some`: classic stage and synthetic scenes show none, no fallback. `world_to_wgs84` is unchanged; it keeps using world `y` as the altitude (the existing test pins `(0, 41.1, 0)` at gundo_osm, which is the anchor's `alt_m = 41.1`, so scene `y` is absolute; `alt_m` is therefore not added).
- Test `geo_origin_comes_from_the_selected_scene_descriptor_only` (`ui.rs:4706`) replaces the table test: classic and out-of-range -> `None`, every registry index returns exactly its descriptor's value, the three real-world scenes are `Some`, `gundo` / `inspector` are `None`. It does not hard-code coordinates or the scene count.

### 4. `SceneBuildConfig::smart_fill`
- Verified: default `SmartFill::OFF` (`scene_module.rs:313`). nv-demo has no such setting and builds with `..SceneBuildConfig::default()` (`csg_scene_host.rs:2384-2391`), so it stays off. No change.

### 5. Road tile header `aux[1][3]`
- `raytracer.cu` and the host never read road-tile `aux_planes`; the only road-tile decode in nv-demo is the hit `part_id` `(s, lat)` packing (`raytracer.cu:1295-1300`), which is produced inside `csg_eval.cuh`. No change.

### csg-rs request during the work (chat #688)
- `scene_registry_exposes_all_scenes_and_variants` (`csg_scene_host.rs:2543-2549`) now checks that the five scenes nv-demo relies on are present (set containment) instead of the exact id list, so the two data-only scenes csg-rs adds next do not break it. No test hard-codes viewpoint or rail counts (`:3363-3366` compares a restored scene with its own build).

## Pipeline stack sizes

Before: `Pipeline::new(.., LinkOptions::default(), ..)` and no explicit stack call, i.e. OptiX sized the stack at link time from `maxTraceDepth = 1`, graph depth from the pipeline flags. After: `build_optix_pipeline` links with `max_trace_depth = PIPELINE_MAX_TRACE_DEPTH (1)`, `max_graph_depth = PIPELINE_MAX_GRAPH_DEPTH (2)` and `size_pipeline_stack` (`renderer.rs:1578-1597`) accumulates `optixProgramGroupGetStackSize` over all 17 groups and applies `optixUtilComputeStackSizes(1, 0, 0)` with `optixPipelineSetStackSize(.., 2)`.

Measured with the new GPU test `pipeline_stack_follows_the_linked_programs` (`renderer.rs:6059`, `--nocapture`), after adoption:

```
raytracer.cu stack needs: StackSizes { css_rg: 320, css_ms: 0, css_ch: 0, css_ah: 0, css_is: 0, css_cc: 0, dss_dc: 0 }; continuation 320 B, direct 0 / 0 B
```

So the OptiX continuation stack is 320 B (raygen state across `optixTrace`), direct-callable stacks 0 / 0. `css_is` is 0: the CSG evaluator frame is NOT on the OptiX continuation stack; it is per-thread local memory of the non-inlined evaluator functions, which the driver sizes from the module. The pre-adoption OptiX value was not printed (the test did not exist); by the same formula over the same unchanged raygen it is the same 320 B. The evaluator frame itself, from the PTX `__local_depot` sizes (scratchpad `ptx_depots.ps1`):

| PTX function | before (`24b22f5`, release PTX of `9bb2bae`) | after (`fa5e170`) |
| --- | --- | --- |
| `csg_trace_rpn_slow_large_val` / `_shadow` | 4224 B | 4240 B |
| `csg_trace_rpn_slow_small_val` / `_shadow` | 1632 B | 1632 B |
| `csg_trace_pure_union_slow_val` / `_shadow` | 40 / 8 B | 40 / 8 B |
| `__raygen__render` | 32 B | 32 B |

The "~4.8 KB" figure from csg-rs is the large RPN frame plus callees; the large frame grew by 16 B. No stack constant in nv-demo depends on it.

## Performance

Method of `wave_final.md` Performance, unchanged: release, persisted default scene (config restored from `scratchpad\nv-demo.json.bak` before and after every run), 4 spp, 6 bounces, denoiser / bloom / DLSS off, `NV_DEMO_PERF_WARMUP=150`, `NV_DEMO_PERF_FRAMES=600`, statistic = median of the 5 per-run medians (`perf_stats.py`). Window 2207x996 as persisted.

The machine was not quiet (tars-rs-tails GPU tests, a foreign `trace_bench`, csg-rs tests; the operator also deleted `target\release` for disk space mid-session), so the decisive measurement is an alternating A/B: A = `9bb2bae` built in a scratch worktree (`scratchpad\nvrs_main`, `--locked`), B = this adoption, run A1 B1 A2 B2 ... with GPU-idle wait and a log of foreign executables per run (`perf_ab.ps1`). csg-rs paused GPU work for every window (chat #677/#678, #695/#696, #703/#704).

| Metric (ms), median of run medians | A `9bb2bae` | B adoption | delta |
| --- | --- | --- | --- |
| OptiX RT pass (GPU) | **44.11** (44.11 45.31 43.76 47.40 42.91) | **49.40** (52.54 52.41 49.40 49.23 49.39) | +5.29 (+12.0 %) |
| frame work | **45.31** (p95 50.67) | **50.12** (p95 58.57) | +4.81 (+10.6 %) |
| host prep | 0.513 | 0.543 | +0.03 |
| CUDA compute | 0.524 | 0.507 | -0.02 |
| tonemap | 0.087 | 0.087 | 0 |
| crowd step | 0.127 | 0.129 | 0 |

Pairs used: 3, 4, 5, 6, 7. Discarded and re-run: pairs 1 and 2 (csg-rs disclosed `cargo test` + clippy 08:22-08:27 UTC, overlapping B1 08:21:30, A2 08:24:02, B2 08:26:24; A1 additionally hit a foreign GPU load, RT median 201.7 ms) -> re-run as pairs 6 and 7. In every kept pair B is slower than A, and the slowest A run (47.40) is faster than the fastest B run (49.23). The two other series are consistent with the A/B: sequential baseline on `9bb2bae` before the bump (csg-rs paused, 06:45-06:49 UTC) RT 44.93 / frame 45.63 ms; a first sequential post-adoption series (07:34-07:41 UTC) was bimodal (RT 52-112 ms) under foreign load and was discarded entirely, as was one run of a third series (target deleted mid-series).

Result: **the adoption makes the RT pass ~5.3 ms (12 %) slower on the default scene.** Not attributed by measurement. Two changes are in B: the csg-rs `fa5e170` evaluator (window clipping, overflow re-evaluation, `FIRST_HIT_UNION` compounds leaving the multi-AABB GAS for the RPN path) and nv-demo's explicit stack sizing. The stack sizing applies the same formula OptiX uses at link (continuation 320 B either way), so the evaluator / GAS-layout change is the likely cause; an A/B of `fa5e170` without the stack change, or a count of compounds that moved from `MultiAabbUnion` to `GeneralRpn` on this scene, would settle it.

## Image changes

Screenshots of the same scene and config, ~40 s after start: `scratchpad\smoke_main_9bb2bae.png` (A) and `scratchpad\smoke_adopt_fa5e170.png` (B). Both render the scene (sidewalk, bench, lamps, crowd, sunset lighting) with no missing geometry, black or NaN pixels visible at this view. The camera orbits and traffic / crowd move, so the two images are not the same frame and a pixel diff is meaningless; crowd placement differs between them (the crowd is simulated). Any pixel-level change from the evaluator (window clipping, exact path for rays starting inside overlapping unions) was NOT checked and may exist; a frozen-pose, fixed-camera capture of both builds is needed for that.

Release smoke run (B, 2201x1028 window): ~60 s, stdout and stderr logs empty (`scratchpad\smoke_adopt_{out,err}.log`), RT FPS 18 (49.6 ms) in the HUD, CUDA crowd 180, NGX available. All 15 perf runs of B and 7 of A also left empty logs.

## Gate results (verbatim summaries)

- `cargo clippy --workspace --all-targets --message-format=short -- -D warnings` (`NVCOMP_ROOT=.sdk\nvcomp`): `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 5m 24s`, no warning lines. After the last nv-demo edits (test rename, doc comments, registry test) re-run scoped: `cargo clippy -p nv-demo --all-targets -- -D warnings`: `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 1m 03s`.
- `cargo test --workspace --quiet`: **500 passed / 0 failed / 208 ignored** (base 500 / 0 / 207; +1 ignored = the new GPU stack test). nv-demo alone after the last edits: `test result: ok. 146 passed; 0 failed; 6 ignored`.
- `cargo test -p nv-demo -- --ignored --test-threads=1`: `test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 146 filtered out; finished in 26.88s` (5 existing + `pipeline_stack_follows_the_linked_programs`). An earlier run failed that new test on its first assertion (`css_is > 0`, which was wrong: the evaluator frame is local memory, see above); the assertion was corrected, not weakened to nothing.
- Release run: above. Screenshot: above.

## Risks / open

- **RT regression +12 %** (see Performance), cause not isolated. Hand to csg-rs with the numbers; nv-demo cannot recover it without changing the GAS plan for overlapping unions, which would reintroduce the wrong-for-inside-origins case.
- Image equivalence not verified pixel-wise (moving scene). Needs a fixed-camera, frozen-pose A/B capture.
- New network / TLS dependencies (`ureq`, `rustls`, `ring`, `webpki-roots`) through `osm-rs`; `cargo deny check` not run: `webpki-roots` (CDLA-Permissive-2.0) and `ring` licences may need `deny.toml` entries (`DENY:LIC`).
- `build.rs` reruns only on `csg_eval.cuh`; when csg-rs commits the fragment split, switch it to `csg_device::CUDA_SOURCE_FILES` (the runtime PTX cache key already uses `CSG_EVAL_CUH`).
- `OptixPipelineSet`'s fifth element (`StackSizes`) is consumed only by the GPU test; production ignores it (`_stack_needs`).
- Scratch worktree `scratchpad\nvrs_main` (detached `9bb2bae`, with a `.sdk` junction to the repo) is left for further A/B runs; remove it with `git worktree remove`.
- `target\linux-check` (10.8 GB, cross-check build cache from the final wave) was deleted by this session when the disk hit 0 bytes free and an edit truncated `csg_scene_host.rs` to 0 bytes; the file was restored byte-exact from `HEAD` (`git cat-file blob`) and the edits re-applied before any build.
