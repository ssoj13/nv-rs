# Gate report - root workspace, Waves 3 + 5 (2026-11-23)

Gate agent: Claude Code (opus). HEAD `7a62138`. All fixes UNCOMMITTED in the working tree. No git state changed.
Commands ran one at a time with `CARGO_TERM_COLOR=never` (git-bash via filesystem-mcp `run_command`).
While this ran, another session (csg-rs) used the same GPU in a separate worktree; the GPU was at 99% load at app start.
No test went red, so nothing was re-run.

## 1) Per crate: compile status and edits

`cargo check --workspace --all-targets --keep-going`: the whole workspace compiled after ONE fix (the edits committed in `7a62138` already compiled; the only error was in a test).

| File:line | What | Why |
| --- | --- | --- |
| `crates/nv-loader/src/nv_loader.rs:1839` (test `nothing_present_is_not_found_with_every_searched_path`) | `\|_\| Ok(())` -> `\|_\| Ok::<(), String>(())` | E0283: error type of the closure could not be inferred; same annotation as the neighbouring test at :1826 |
| `crates/cudnn-sys/src/cudnn_sys.rs` `preload_siblings` + new `reject_foreign_copy` | Before mapping anything, for every `cudnn_*` sibling and the main `cudnn64_*.dll`: if a module with that base name is already loaded in the process (`GetModuleHandleExW` by name via `libloading::os::windows::Library::open_already_loaded`) but not the one mapped from the candidate's own directory (same call with the full path; handles compared), the candidate is rejected with `a different copy of <name> is already loaded in this process; refusing to load <path> ...`. References taken by the probe are released (`into_raw`/`from_raw`, SAFETY comments). | Coordinator regression from the python gate (see below). |
| `COVERAGE.md` | Regenerated (`cargo run -p xtask -- coverage generate`) | `coverage check` reported stale. Change: `cufftGetSize` and `nvjpegCreate` lost their `wrapped by` crate (`cufft` 20->19 wrapped, `nvjpeg` 46->45): these two were only mentioned in comments, which no longer count after the xtask comment-stripping fix. |
| follow-ups (section 5) | `nvrtc/src/options.rs`, `nvrtc/src/program.rs`, `nvapi-sys/src/nvapi_sys.rs`, `nv-demo/src/camera_rail.rs`, `nv-demo/src/main.rs` | see section 5 |

Clippy (`--workspace --all-targets -- -D warnings`): clean on first run, no clippy edits needed.

### cuDNN preload regression (coordinator request) - analysis and status
- Siblings were already loaded from the directory of the chosen `cudnn64_9.dll` (full paths via `nv_loader::open_library`). A preload failure is NOT silent: the `exports_sys!` `preload` hook (`$preload(path)?`) turns it into a rejection of that candidate, reported in `LoadError::Rejected { failures }`; the loader then tries the next candidate.
- Difference to `05efa7e`: the old code loaded with `LOAD_WITH_ALTERED_SEARCH_PATH`; `open_library` now tries `LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS` first and falls back to altered search only on error 126.
- Most likely mechanism for error 127 (NOT reproduced here; no torch in the Rust tests): the Windows loader resolves bare-name imports (`cudnn_graph64_9.dll`, ...) to a module of that name that is already loaded, even when the importer was loaded by full path. With torch's cuDNN 9 in the process first, the SDK v9.23 `cudnn_heuristic64_9.dll` binds to torch's graph/ops copies -> procedure not found. The reverse order (nv-rs first, then `import torch`) makes torch bind to the v9.23 copies; that cannot be prevented from our side: two cuDNN 9 builds cannot share one process on Windows. This was inherent in the old code too; whether this wave changed which copy is chosen first (catalog search order) is not verified.
- Fix applied: the foreign-copy check above turns the torch-first case into a clear, typed rejection instead of a 127 deep in a sibling. It does not fix torch-after-nv-rs. A decision is needed for the python side: prefer an already-loaded cuDNN copy (load from its directory) or document "import torch first / use one cuDNN". Not decided here.
- Verified: `cargo clippy -p cudnn-sys -- -D warnings` clean; `cargo test -p cudnn-sys -p cudnn -- --include-ignored --test-threads=1` 17 passed / 0 failed. Python reproduction not run (python/ is out of scope).

## 2) Non-ignored tests
`cargo test --workspace --quiet --no-fail-fast`: **467 passed, 0 failed, 204 ignored** (before and after the follow-ups; same counts).

## 3) Ignored tests (`-- --ignored --test-threads=1`, `NVCOMP_ROOT=.sdk\nvcomp`)

| test crate | result (passed/failed) | class | evidence |
| --- | --- | --- | --- |
| nv-loader | 0/0 (no ignored tests) | - | |
| nv-bindgen | 1/0 | - | |
| nv-bindgen-fixture | 0/0 (none) | - | |
| xtask | 0/0 (none) | - | |
| cublas-sys, cublaslt-sys, cudnn-sys, cufft-sys, cupti-sys, cusolver-sys, cusolvermg-sys, cusparse-sys, npp-sys, nvapi-sys, nvcuvid-sys, nvenc-sys, nvjpeg-sys, nvml-sys, nvofa-sys, nvrtc-sys, optix-sys | 1/0 each | - | live load probes |
| nvcomp-sys | 2/0 | - | |
| cufile-sys, nccl-sys, ngx-sys | 0/0 (no ignored tests on Windows) | - | cufile/nccl Linux-only |
| nv-demo | 5/0 (incl. 4 `cuda_crowd` GPU tests vs pinned csg-rs `f7c8474a`) | - | |
| cudnn (after preload fix) | 17/0 incl. ignored | - | |
| nvrtc (after dlto fix) | 4/0 | - | |

Not run: ignored tests of the other wrapper crates (not touched by Waves 3/5; green at `05efa7e`).

## 4) Application start
- `cargo build -p nv-demo --release` OK (1m57s), then `target\release\nv-demo.exe` started detached (pid 28516), stdout/stderr captured to files; ran ~60 s; killed only that pid.
- stdout and stderr logs were EMPTY: no NVRTC/OptiX compile error, no `Fatal` message, nothing else printed.
- Window `nv-rs RTX Fractal Path Tracer (OptiX 9 + AI Denoiser + nv-wgpu)` came up; one screen capture after ~60 s showed: `Old Gundo` scene at `Sunset`, Beauty AOV, Pinhole lens, street with sidewalk, a parked white car, yellow pedestrian figures, street furniture, lamp bollard; status bar `Live (DEMO+ORBIT+TRAFFIC) | 832x640 | BEAUTY | UI 60 FPS | RT 10.3 FPS (84.7 ms)`; telemetry `Accum 1/256`, `DLSS RR Active`, `NGX Available`, `CUPTI Active`, `Crowd Sim CUDA 0.15 ms (180)`, `CSG Inst 3873/6967`. So the runtime-compiled `raytracer.cu` compiled and frames render.
- NOT observed: other AOVs (Depth/Motion/Albedo/Normal/WorldPos/Semantic/Instance), the 4 sensor projections, MP4 recording, SDG burst + cancel, Traffic toggle, Day->Night crossfade, cursor modes. No input was sent to the window. No claim about image correctness beyond the description above. RT FPS was measured while another session shared the GPU.
- The release binary started was built BEFORE the follow-up edits to `main.rs` / `camera_rail.rs` (both host-side, compile- and test-verified afterwards; no shader change by this gate).

## 5) Follow-ups
Done:
- `crates/nv-demo/src/camera_rail.rs:~465`: when the selected rail still exists after a scene rebuild, `duration_s` is no longer reset from the spec (now survives like `looping` / `ease_io`). The fall-back-to-first-rail branch still resets it (new rail).
- `crates/nv-demo/src/main.rs:~1877, ~1913`: previous-video finish error and finish-after-encode-error now go to the status line (`RenderEvent::StatusMessage`); the shutdown-time finish error (`~1950`) stays on `eprintln` (UI is closing). Note: the previous-video message is immediately followed by the `Recording to ...` message on the same status line.
- `crates/nvrtc/src/options.rs:82-86`: `dlto()` pushes `-dlto` (nvrtc.h 13.3 documents `--dlink-time-opt (-dlto)`; matches `python/nv-py/tests/test_foundation.py:80-82`); unit-test expectation updated; doc in `program.rs:180`.
- `crates/nvapi-sys/src/nvapi_sys.rs:7-8`: doc now says the crate compiles to nothing off Windows (no `load()`), instead of claiming `NotFound`.

Not done:
- `ui.rs` animation sliders from `demo_anim::anim_range`: `spin_speed` is already reachable (`ui.rs:805`, range `0.0..=2.0`, same as the `demo_anim` table); switching every animated slider to `anim_range` is a UI-wide change, not small/local. Left for the next nv-demo pass.
- `crates/cudnn` buffer context check by value: the premise does not hold - `same_stream` (`cudnn.rs:160`) and the cublas handle (`handle.rs:327,434`, `lt.rs`) ALSO use `Arc::ptr_eq`. Comparing by value needs a decision on what identity is (raw `CUcontext` handle?) across ~40 call sites in `cudnn` (and cublas for consistency). Needs a lead decision.
- Larger items left alone as instructed.

## 6) Final gate (after all edits)
- `cargo clippy --workspace --all-targets --message-format=short -- -D warnings`: exit 0, no warnings/errors.
- `cargo test --workspace --quiet --no-fail-fast`: 467 passed, 0 failed, 204 ignored.
- `git status --short`:
```
 M COVERAGE.md
 M crates/cudnn-sys/src/cudnn_sys.rs
 M crates/nv-demo/src/camera_rail.rs
 M crates/nv-demo/src/main.rs
 M crates/nv-loader/src/nv_loader.rs
 M crates/nvapi-sys/src/nvapi_sys.rs
 M crates/nvrtc/src/options.rs
 M crates/nvrtc/src/program.rs
?? 0/
```
(plus this report file)

## Risks / open
- cuDNN two-copies-in-one-process: mitigated for the torch-first order only; root cause not reproduced; decision needed (see section 1).
- Interactive features of nv-demo listed in section 4 remain unverified.
- Peer message received from the csg-rs session (chat #629, csg-rs main `125b870`): adopting it needs these nv-demo changes - (1) `cuda_crowd.rs` compile `csg_actors::pod::actor_sim_cuh_source()` instead of `ACTOR_SIM_CUH`; (2) `step_and_emit_actors_cuda` has 19 params (append `uint32_t lod2_pawn_compound` after `cull_dist_m`, 0xFFFFFFFF keeps root compound; `archetypes` non-null), both launch sites; (3) `ActorStatePod` 96 B (`foot_lock`), `ArchetypeMetaPod` 272 B (`limb_mounts`), check kernels at cuda_crowd.rs:343/361/458/467; (4) `raytracer.cu` `__intersection__csg` / `__intersection__csg_prim` pack `csg_instance_semantic(flags, hit.sem_id)`, keep flag bits 16..24 on host; (5) `csg_scene_host.rs` use `MAX_GPU_COMPOUNDS`, log `world.library.take_issues()`, `world.set_crowd_culling(false)`, `InstanceId.0` no longer an index (use `dynamic_index`), `sdg_export.rs:365` crowd bones tagged by `DynamicKind`; (6) `CrowdSim::spawn_actor` returns `INVALID_ACTOR` (tests at cuda_crowd.rs:1640, :2004). Not adopted by this gate (pinned `f7c8474a`, out of scope).
