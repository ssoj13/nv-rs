# nv-demo OptiX RT pass: where the time goes (2026-11-24)

RTX 3080 Ti (sm_86), CUDA 13.3, OptiX 9.1, driver 621.24, Windows 11. Base `main` = `7e47707`.
Commits of this work: `d4f1098` (rail benchmark), `32a6ab1` (csg-rs `6e84ff6` P23 adoption),
the Flight-stats / bench-image commit and the docs commit that follow them.

## 1. Measurement method (changed)

The earlier numbers (`csg_fa5e170_adoption.md`, 44.1 vs 49.4 ms) came from free-running sessions:
wall-clock-paced camera, moving traffic and crowd, and no record of the camera state. The operator
saw slow runs in possession with the "360 Roof LiDAR" mount (Equirect360), fast runs in the free
Pinhole camera. Nothing in the persisted config starts possession or a 360 projection
(`camera_projection` in the saved file is `Pinhole`; `user_intent_settings` persists the
pre-possession projection; startup never possesses); possession needs a UI action (Possess Car,
Ctrl+Click, Pick mode), so a stray click or key on a benchmark window that appeared over the
operator's work is the plausible cause. Those A/B numbers are therefore not evidence.

New method, used for every number below:

- `nv-demo --bench-rail <rail> --scene <id> --out <csv> [--frames] [--warmup] [--image]`
  (`bench.rs`, `d4f1098`): frozen traffic/crowd, free Pinhole camera, no demo animation / TOD
  transition, UI commands dropped (stray input cannot possess), user config never written, rail
  from t = 0 stepped in lockstep with the renderer at `time_scale / 60 s` per frame (the Smooth
  60 FPS recording path), exits after the CSV.
- The perf CSV records per frame `possessed, projection, eye_x/y/z, width, height, spp, bounces`.
  Every run in this report: `possessed = 0`, `projection = 0` (Pinhole), 832x640 (viewport of the
  saved layout), 4 spp, 6 bounces, identical eye sequence in every run of every arm.
- `StartPerfLog` restarts the sample frame index, so two runs of one build render bit-identical
  frames (checked: max abs diff 0 on the PFM of frame 250); `--image` dumps the last frame's
  linear colour as PFM for pixel A/B.
- Rail: `gundo` `road_cruise` ("Street Navigation (24s)"), 60 warm-up + 1440 measured frames.
- The GPU was shared all day (a non-chat tars-rs-tails session ran GPU test suites continuously,
  csg-rs workers ran tests inside two freeze windows). Interleaved rounds, and the statistic is the
  per-frame minimum over rounds (frame i is the same work in every run; foreign load only adds
  time), then median / p95 / sum over the loop. Rounds inside the windows csg-rs named
  (17:47-18:23Z) were discarded. Scripts: scratchpad `rt/rail_ab.ps1`, `rt/rail_cmp.py`,
  `rt/pfm_diff.py`.

## 2. Tools

| Tool | Result |
| --- | --- |
| Nsight Compute 2026.3.1 (`ncu`) | First failure: `CUPTI_ERROR_MULTIPLE_SUBSCRIBERS_NOT_SUPPORTED` because nv-demo enables CUPTI kernel activity in-process. Fixed: `profiler::probe_cupti` skips CUPTI when an Nsight tool is injected (`NV_NSIGHT_INJECTION_TRANSPORT_TYPE` on Windows, `CUDA_INJECTION64_PATH` on Linux; `d4f1098`). Second failure, not resolved: "Failed to initialize the performance metric collection layer" on the first kernel of the nv-demo process (elevated shell; the same `ncu` profiles nv-demo's own CUDA crowd test binary and other CUDA tests fine). Suspect the Vulkan/NGX/NVML side of the app; no per-program counters, registers or occupancy from ncu. |
| Nsight Systems 2026.1.3 | `nsys profile -t cuda,nvtx` of a 60 s run: report generation ran > 8 min at 100 % CPU and was stopped; nothing usable. |
| `ptxas -v` on the OptiX PTX | Fails (`_optix_*` calls need OptiX's prototypes). |
| csg-rs `ptxas` on their entry points (chat #764) | `csg_intersect_prim_idx` program 142 regs / 160 B local (shadow 108), `csg_union_part_hit` program 193-195 regs / 784 B (shadow 188-190), 0 spill. |
| PTX `__local_depot` (previous session, `ptx_depots.ps1`) | `csg_trace_rpn_slow_large_val/_shadow` 4240 B, `_small` 1632 B, `csg_trace_pure_union_slow_val/_shadow` 40 / 8 B, `__raygen__render` 32 B. OptiX continuation stack 320 B (raygen only). |
| Controlled A/B on the rail benchmark | Used for all attribution below. |

## 3. Time attribution (rail benchmark, P23 build `32a6ab1`)

Per-frame minimum over rounds, RT pass (OptiX launch, GPU events):

| Arm | rounds | median ms | p95 ms | loop sum vs full |
| --- | --- | --- | --- | --- |
| full (4 spp, 6 bounces, shadows, SER) | 5 | 75.88 | 165.93 | 100 % |
| shadows off | 3 | 59.17 | 127.47 | 78.5 % |
| 1 bounce (primary + its shadow ray) | 3 | 28.86 | 45.03 | 34.5 % |
| 2 bounces | 3 | 57.81 | 114.12 | 72.9 % |
| SER off | 3 | 76.41 | 167.44 | 101.5 % |

By ray type / path depth (loop sums): primary hit + its shadow 34.5 %; the second bounce (first
incoherent bounce + its shadow) 38.4 %; bounces 3-6 (Russian roulette from bounce 2) 27.1 %.
Shadow rays at all depths 21.5 %. So ~2/3 of the RT time is secondary rays; the second bounce
alone costs more than the camera rays. The heavy views (p95 166 ms, more than 2x the median) are
the same frames in every arm and every build: view content, not noise.

SER (`optixTraverse` + `optixReorder` + `optixInvoke`) on Ampere: reorder is a no-op on sm_86,
but turning it off is 1.5 % slower (within ~1-2 % noise; no evidence for removing it).

By program / geometry class: not measurable here without ncu. Register footprints above; the
evaluation-path split per frame (single primitive / pure union / first-hit union / RPN instance
counts) is now in `CsgStats::path_instances` and the Flight stats panel.

## 4. csg-rs versions on the same benchmark

| Arm | rounds | median ms | p95 ms | loop sum |
| --- | --- | --- | --- | --- |
| `9bb2bae` (csg `24b22f5`) | 6 | 58.26 | 161.61 | 1.000 |
| `fa5e170` (main before this work) | 6 | 77.01 | 161.87 | 1.198 |

(first window, 2 arms interleaved)

| Arm | rounds | median ms | p95 ms | loop sum vs fa5e170 |
| --- | --- | --- | --- | --- |
| `fa5e170` | 4 | 81.97 | 173.89 | 1.000 |
| csg `6e84ff6`, nv-demo hooks not applied | 5 | 82.13 | 175.29 | 1.004 |
| csg `6e84ff6` + P23 hooks (`32a6ab1`) | 5 | 75.88 | 165.93 | 0.933 |

(second window, 3 arms interleaved; the machine was busier, so absolute values are higher than in
the first window; compare within a table)

- The `fa5e170` regression is real under identical camera state: +19.8 % on the loop (median
  frame +32 %), heavy views unchanged.
- P23 recovers 6.7 % of the loop: about a third of the regression. The rest is elsewhere in
  `fa5e170` (candidates on the csg-rs side: span-window clipping / overflow re-evaluation in the
  RPN path, the `b2c0824` evaluator split), reported to csg-rs.
- Separate hit group for first-hit unions (csg-rs #764): rendered output bit-identical to the
  single-program variant (PFM diff 0).

## 5. Image checks (PFM, linear colour, frame 250 of `road_cruise`, frozen)

| Pair | max abs | mean abs (mean value) | pixels changed | > 0.05 |
| --- | --- | --- | --- | --- |
| P23 build, two runs | 0 | 0 | 0 | 0 |
| no-hooks vs P23, 6 bounces | 0.654 | 2.7e-5 (0.196) | 1.82 % | 122 (0.023 %) |
| no-hooks vs P23, 1 bounce, no shadows | 0.222 | 2.8e-6 (0.159) | 0.81 % | 12 (0.002 %) |
| P23 one program vs separate hit group | 0 | 0 | 0 | 0 |

The primary-hit differences are ulp-level hit `t` / normal changes on first-hit-union parts (the
exact per-part rule replaces the old first-hit walk) plus a dozen silhouette pixels; at 6 bounces
they decorrelate later random decisions, hence more changed pixels with the same tiny mean.

## 6. Findings

nv-demo side (`raytracer.cu`, pipeline, host):

1. Measurement: free-running A/B is not valid (camera state, traffic, pacing); fixed by the rail
   benchmark and the state columns. The Inspector crowd showcase index was hard-coded as 109; csg-rs
   moved it to 111 (three tests failed, and the UI "poke all" rule silently stopped working);
   now `csg_scenes::ACTOR_SHOWCASE_VARIANT_IDX`.
2. Shadow rays already use `TERMINATE_ON_FIRST_HIT | DISABLE_CLOSESTHIT`; every CSG / classic
   instance and GAS carries `DISABLE_ANYHIT` except the OMM halo (needs any-hit). Pipeline
   `maxTraceDepth = 1`, 8 payload / 5 attribute registers, continuation stack 320 B: no waste
   found there.
3. Candidate, not changed (would change the image or needs data): CSG instances without
   `CAST_SHADOW` are still traversed by shadow rays and rejected inside the intersection program;
   a visibility-mask bit (camera vs shadow) would skip them in hardware without changing the
   image. Count of such instances on `gundo` not measured yet; the Flight stats panel shows the evaluation-path split per frame.
4. Candidate: unbounded compounds (ground) get a 10 km AABB, so every ray of every type invokes
   their intersection program once; worth counting with csg-rs (their per-path counters).
5. No nv-demo shader change was proven faster in this session; nothing else was changed in the RT
   code.

csg-rs side (sent to Mullis-16-3934):

- Regression `24b22f5 -> fa5e170` = +19.8 % loop on `road_cruise`; P23 recovers 6.7 %, the rest
  is outside the union GAS layout.
- Union-part entry must stay in its own hit group (193 vs 142 registers); adopted that way.
- ~65 % of the RT time is secondary rays and 21.5 % is shadow rays, so the RPN / union paths are
  exercised mostly by incoherent rays; the heavy views (p95 > 2x median) are the place to look
  (the Flight stats "slowest positions" list gives rail times and eye positions).

## 7. Gate (`CARGO_BUILD_JOBS=6`)

- `cargo clippy --workspace --all-targets -- -D warnings`: `Finished dev profile ... in 6m 43s`, no warnings.
- `cargo test --workspace --quiet`: **508 passed / 0 failed / 208 ignored** (base 500 / 0 / 208; +8 new tests: bench args, perf CSV / PFM, flight stats, evaluation paths).
- `cargo test -p nv-demo -- --ignored --test-threads=1`: **6 passed / 0 failed** (crowd x4, PTX cache eviction, pipeline stack).
- P23-only tree (`32a6ab1`) gated separately: nv-demo clippy clean, 150 passed / 0 failed / 6 ignored.
- Release: every benchmark run above is a release run of the committed code (exit 0, empty stderr).

## 8. Status: parked (operator, 2026-11-24)

The remaining regression is accepted. Optional follow-ups: shadow-only visibility mask for
non-`CAST_SHADOW` instances, the 10 km AABB of unbounded compounds, a headless OptiX harness for
ncu, and the csg-rs side of the remaining ~13 %.