# HANDOFF - nv-rs plan2 repair waves (written 2026-11-23, lead session hit its model limit)

- **Timestamp / agent**: 2026-11-23T14:15-08:00, Claude Code (lead session `nv-rs-ee`). HEAD when written: `dcab6c7`. If `git status` shows edits newer than that commit, they come from the one worker still running (see below) or from a later session.
- **Goal**: operator asked for `/bughunt` over the whole repository, then to fix everything in the resulting `plan2.md` autonomously.
- **Active plan**: `plan2.md` (waves 1-7); execution tracker `TODO.md`.
- **Out of scope**: external repos except where the operator ordered it (`camera-rs` was changed and pushed: `9109108`); `csg-rs` (two items recorded for it: road-tile UV in `part_id`, RPN stack overflow flag), NVIDIA `cuda-rust`, generated `prebuilt/` bindings, the `docs/` site, untracked `0/`.
- **Completed and verified** (commands run by gate agents on this machine, RTX 3080 Ti): `cargo clippy --workspace --all-targets --message-format=short -- -D warnings` clean; `cargo test --workspace --quiet` 375 passed / 0 failed / 203 ignored; `cargo test -p <crate> -- --ignored --test-threads=1` green for `cuda-core-ext`, `curand`, `cufft`, `cublas`, `cusolver`, `cusparse`, `cudnn`, `npp`, `nvjpeg`, `nvofa`, `nvcomp` (with `NVCOMP_ROOT=.sdk\nvcomp`), `nvrtc`, `cupti`, `nvenc`, `nvdec`, `nv-wgpu`, `optix`, `nvapi`, `nv-demo`. Last commit in that state: `05efa7e`. `camera-rs` `9109108`: clippy clean, 347 tests.
- **Deviations from the plan**: `Deviation | Reason | Plan updated?`
  - Buffer leases targeted instead of everywhere | measured: only stream-ordered buffers crossing streams corrupt | yes (`plan2.md` systemic 1, decisions table)
  - `nv-wgpu` rewritten to submit through the wgpu encoder (architecture change, not pre-approved; operator later delegated decisions) | raw `vkQueueSubmit` cannot be synchronised with wgpu | yes
  - `xtask` prebuilt completeness inferred from generator guarantees, not from the crate recipe | the recipe lives in each `build.rs` | yes (`TODO.md` follow-up: manifest from `nv-bindgen`)
  - Commits `e1eed4c..d32d90f` are labelled "pre-gate" but contain gate fixes | committed while the gate ran | yes
  - `main` currently holds ungated, non-compiling work | operator asked to save everything before the model limit | this file
- **Exact resume point**:
  - `- [x] M1: loader worker finished (14:35) and its edits are committed (ungated). All 37 L-items have a verdict; nv_loader::catalog exists (CatalogEntry { name, spec }, ALL, one const per library incl. DLSS, DLSSD, 11 NPP parts); version gate wired for cublas, cufft, cusolver, cusparse, nvjpeg, cudnn, nvcomp, nvenc, nvofa; build-time SDK hint is debug-only; add_search_dir returns SearchDir; exports_sys! has preload:/version: hooks; cached_loader! replaces three hand-written loaders. xtask prebuilt rerun not needed (NPP inventory 'unshimmed' may grow on a fresh generation). Open from it: L-28 doc line in nvapi_sys.rs:7-8, L-27, L-30, L-31, L-32; xtask must switch SPECS to nv_loader::catalog::ALL and stage DLSSD; nv-py probe_libraries keys change (nvoptix -> optix, nvofapi -> nvofa) and NV_RS_DEBUG=1 now means Info; ngx_sys::SDK_BIN still embeds the build path.`
  - `- [~] M2/M3 were LAUNCHED at 14:36 as two background gate agents (root workspace; python/). Each writes its report to audit/2026-11-23/gate_root_w3_w5.md and audit/2026-11-23/gate_python_w6.md when it finishes and leaves its fixes UNCOMMITTED in the working tree. If those files exist, read them, review the diff, commit and push; if they do not exist and no agent is alive, run the gates again.`
  - `- [ ] M2: root gate | verify: cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --quiet; ignored GPU tests of touched crates; cargo run -p nv-demo --release starts and renders`
  - `- [ ] M3: python gate | verify: (in python/) cargo clippy --all-targets; cargo test; pytest python/nv-py/tests`
  - then the numbered "Next steps" below.

Read in this order: this file, `TODO.md`, `plan2.md` ("Execution record - Waves 1 and 2" and the remediation plan), root `AGENTS.md`. Per-finding ledgers: `audit/2026-11-23/NN_*.md`.

## Operator's standing instructions (this session)
- Answer in Russian; code, comments, docs, commits in English. No co-author footers.
- Fix everything in `plan2.md`. Take design decisions yourself ("best in class"), record decision + rejected alternative in `plan2.md`, commit and push to `main` as you go, keep docs in sync, report afterwards.
- Every audit finding is a claim: re-read the code and the native header before fixing; where reading cannot decide, measure on hardware (RTX 3080 Ti, CUDA 13.3 on this machine). Never guess.
- Do NOT burn the lead model on clippy / test runs / log triage: delegate builds to a subagent. Do NOT do revert-and-rerun mutation checks; check new tests by reading them.
- Workers that run in parallel edit disjoint files and never run cargo; ONE gate agent per workspace builds and tests afterwards (max 2 builds on the machine). Never `git reset` / `checkout --` / stash. Never kill `bun`.
- `/sysgo` rules are active (no shims, no `#[allow]`, no swallowed errors, SSOT, `TODO.md` maintained).

## State of `main` when this was written
- Gated and green (clippy `-D warnings` clean, `cargo test --workspace` 375 passed / 0 failed / 203 ignored, ignored GPU tests of touched crates pass): everything up to `05efa7e` = audit, `camera-rs` de-vendoring (`camera-rs` `9109108`), Waves 1-2 (all wrapper crates, `optix`, `nv-wgpu`, `cuda-core-ext`) and their follow-up.
- Pushed but NOT executed: `a246a6a` (`bootstrap.py`, CI workflows, `deny.toml`; `cargo deny` never run).
- Pushed, does NOT build: `072dd51` `python/nv-py` foundation (needs the three python workers' edits).
- The commit made together with this file is a WORK-IN-PROGRESS SNAPSHOT of ungated edits (see next section). `main` is expected NOT to compile at that commit. This is deliberate (operator asked to save everything before the limit).

## Ungated work in the snapshot commit
All of it was written WITHOUT compiling. Editor diagnostics already show compile errors in several of these files; that is expected until the gate.

| Area | Files | Worker status | Report highlights |
| --- | --- | --- | --- |
| `nv-demo` renderer + scene host | `renderer.rs`, `csg_scene_host.rs` | done | ~60 `expect`/`assert` on the render thread replaced by `RenderEvent::Fatal` or recoverable errors; denoiser fallback loop, SDG burst completion + cancel, Traffic toggle, stale IAS after crowd-step failure, `GrowDeviceBuffer::upload`, scene rebuild retry; 14 host tests. Two borrow-check spots unverified. |
| `nv-demo` main / ui / buses / persist | `main.rs`, `ui.rs`, `app_events.rs`, `persist.rs` | done | captures read the displayed texture (fixes stale PNG under DLSS-RR), unique output names, corrupt config never overwritten (session then saves nowhere), `Fatal` + `is_finished` handling, surface-lost path, Live RT timing, TOD cancel on user edit, DLSS render dims keep aspect, scene saved by id, cursor Hidden/Locked fix. |
| `nv-demo` shaders + GPU compute | `shaders/raytracer.cu`, `tonemap.cu`, `cuda_crowd.rs`, `fft_bloom.rs`, `gpu_export.rs`, `sdg_export.rs`, `profiler.rs`, `build.rs` | done | depth = range for non-pinhole sensors, motion vectors for all 4 projections, semantic/instance id schemes, PTX cache key, empty CUPTI subscriber removed, crowd topology check. `Params` layout unchanged. Shader compiles only at app start. |
| `nv-demo` rail / anim / MP4 | `camera_rail.rs`, `demo_anim.rs`, `mp4_muxer.rs` | done | atomic MP4 (`.partial` + rename, empty = error), 64-bit durations, avcC High trailer, FOV range from `camera`, one animation table. |
| `xtask` | `xtask/src/*` + new `atomic_io.rs` | done | `stage-dlls` fails on missing requested libs, prebuilt chosen by completeness, atomic writes, CLI aliases. Catalog items X-2/X-3/X-4 deferred. |
| loader / bindgen / sys | `crates/nv-loader`, `crates/nv-bindgen`, `crates/nv-bindgen-fixture`, all `crates/*-sys` (not `prebuilt/`) | STILL RUNNING at 14:15 (the only live worker); its files in the committed snapshot are half-edited and its later edits are uncommitted | Building the single `nv_loader::catalog` (`CatalogEntry { name, spec: DllSpec }`, `ALL`), prebuilt-fallback rules, version check, search order. No report yet. |
| `python/nv-py` video + NPP | `video.rs`, `gfx_img.rs`, `tests/test_video_gfx_repair.py` | done | NVENC B-frame queue, lock->unmap order, real `flush`, 10-bit/444, NVDEC 444 read, NVOFA zero-sync execute, NPP leases; removed fake NGX `supported` flags (existing `tests/test_nv_rs_all.py:385-411` must be updated: decision taken, fix the test). |
| `python/nv-py` compute / sparse / jpeg / comp | `compute.rs`, `sparse_ai.rs`, `nvjpeg.rs`, `nvcomp.rs`, tests | done | raw sys calls replaced by the wrapper crates via `compute::typed_view`; leases on all 14 async calls; `test_lease.py` xfail removed. cuDNN activation/softmax still raw (see follow-ups). |
| `python/nv-py` platform / sys / nvml | `platform_ext.rs`, `sys_ext.rs`, `nvml.rs`, `tests/test_sys_platform_nvml.py` | done (finished right after the snapshot; committed in the next commit) | NCCL: temp buffer removed, recv goes straight to NCCL, leases held until the outer `group_end` (PE-01/02/04); cuFile read/write take a mandatory `stream` argument and run under the lease (PE-03); `CufileDriver.close()` fails while handles are live; managed memory and `HostRegistration` synchronise the context before host access / unregister (SE-01/02); CUPTI event buffer bounded at 65536, `close` inside its own callback is an error (SE-03/04); wgpu errors become exceptions via error scopes (SE-06); NVML optional telemetry returns `None` only for `NOT_SUPPORTED` (NV-01). ALL NCCL and cuFile code is Linux-only and was neither compiled here nor will be by the Windows gate. Needs elsewhere: `cuda-core-ext` `ManagedBuffer::as_slice` / `as_mut_slice` should be `unsafe` or synchronise, `zeroed` via `cuMemsetD8`, `HostRegistration::unregister(self) -> Result`; `cupti` `Subscriber::close` / `ActivityProfiler::close` returning `Result`; `errors.rs` allow `General` on Windows so NVAPI errors stop mapping to `NvCudaError`; `nccl` compares contexts with `Arc::ptr_eq`. |

If a later session finds the two "still running" areas in a broken half-edited state and no agent is alive, re-run that worker from the manifest in `TODO.md` ("In flight") against the current tree; do not revert.

## Next steps, in order
1. Wait for / re-run the loader worker and the python platform worker (see table).
2. **Gate the root workspace** with ONE agent (opus), cargo allowed, one build at a time: `cargo check --workspace --all-targets`, fix compile errors minimally preserving intent (contract between nv-demo workers: `RenderEvent::Fatal(String)`, `RenderControl::CancelSdgBurst`, `UiCommand::CancelSdgBurst` in `app_events.rs`), `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --quiet`, then ignored GPU tests of touched crates (`nv-demo`, `nv-loader`, sys crates, `xtask`), then START `nv-demo` (`cargo run -p nv-demo --release`) because the shader compiles only at runtime, and check: app starts, Depth/Motion AOVs on all 4 sensor projections, Albedo/Normal/WorldPos/Semantic/Instance AOVs, MP4 recording, SDG burst with DLSS on + cancel, Traffic off while crowd walks, Day->Night crossfade, DLSS SR/RR picture (NGX commands now go through the wgpu encoder), cursor in hold-fly / orbit / persistent fly. If `coverage check` says `COVERAGE.md` is stale after the xtask comment-stripping fix, regenerate. Model the gate prompt on the Wave 1-2 gate (it is in the session log; essentials are in this paragraph).
3. **Follow-ups for that gate** (lead decisions already taken):
   - rail `duration_s` survives a scene rebuild like `looping` / `ease_io` (`camera_rail.rs:~465`);
   - `main.rs` video-finish errors go to the status line, not only `eprintln`; `ui.rs` animation sliders take ranges from `demo_anim::anim_range`, `spin_speed` reachable;
   - `renderer.rs`: hash `optix.h` bytes in the PTX cache key and share `build.rs` OptiX discovery (`GPU:D07`, `D08`); fractal shape params in probe / SDG launches (`GPU:D21`); previous classic transforms for object motion (`GPU:D03`); `prev_camera` during SDG bursts; `params_buf.copy_from_host` synchronises the stream every frame (breaks the one-sync invariant: make it an async upload);
   - general sky crossfade needs `env_from` in the dead `fractal_xform` / `sat_pos` slots on both sides (`RND:R10`);
   - enable `serde` on `egui_dock` and persist the dock layout (`APP:F19`);
   - `crates/nvrtc/src/options.rs:83` passes `--dlto`; NVRTC 13.3 accepts `-dlto` / `--dlink-time-opt`;
   - `crates/cudnn` compares buffer contexts with `Arc::ptr_eq` (rejects DLPack imports): compare by value, then route nv-py activation/softmax through the wrapper;
   - `optix::InstanceBuffer` pinned staging is fixed at 256 KiB: make it grow on demand (`RND:H13`);
   - `nv-wgpu` `submit_vulkan_commands` waits with `timeout: None` (`nv_wgpu.rs:~328`).
4. **Gate `python/`** with one agent (cargo only inside `python/`): build, clippy, `cargo test`, pytest; update `tests/test_nv_rs_all.py:385-411` for the removed NGX flags; regenerate `.pyi` stubs (`xtask py-stubs`); move `typed_view` / `AnyBits` / `poisoned` into `buf_util.rs` and delete `buf_ptr_and_bytes` / `validate_dev_buf` once unused; make `__dlpack__` wait for outstanding leased writes; add `close()` / `__exit__` to the eight classes (`DPY:C20`, decision: add).
5. After the loader worker: `nv-bindgen` writes an expected-files manifest and a prebuilt-fallback marker for `xtask`; `xtask` consumes `nv_loader::catalog`, adds `dlssd` and companion DLLs; `release.yml` Windows step must check `$LASTEXITCODE`; Linux loader search needs globbing and `lib/Linux_x86_64/rel`, `extras/CUPTI/lib64`.
6. **Wave 1b** (cross-crate, must not run in parallel with other wrapper edits): typed teardown error sink replacing `CUDA_ERROR_UNKNOWN`; one `Scratch` owner; remove the now-formal `scratch` parameter of `cusparse` SpSV/SpSM/SpGEMM; remove dead `Budget` / `Unsupported` variants together with the nv-py matches.
7. **Wave 6 `device/`**: `[patch."https://github.com/NVIDIA/cuda-rust.git"]` in `device/Cargo.toml` to the fork `ssoj13/cuda-rust_b` (fork carries unmerged `cuda-oxide` patches; `cuda-core` differs only in `build.rs`, `embedded.rs`, two tests), then build and run the 15 GPU cases; `DPY:D2..D9`.
8. **Wave 7**: record every refuted finding, close `TODO.md`, final full gate of both workspaces.

## Things only the operator / other hosts can verify
- Interactive feel of camera after the `camera-rs` switch and the DLSS picture, if the gate agent cannot judge them from screen capture.
- Linux-only branches (`cufile` GDS, `nccl`, Linux loader paths, Linux CI): no Linux host in this session. `python/nv-py/src/platform_ext.rs` read path will not compile on Linux until the platform worker's change lands.
- CI workflows have never been executed; branch-protection check names changed.

## Measured facts worth keeping (do not re-derive)
- Buffer lifetime (systemic 1): only a stream-ordered buffer (`DeviceBuffer::uninitialized_async`) freed while ANOTHER stream still writes it is corrupted (Rust 20/20, Python 10/10); same-stream and synchronous buffers are not. Lease helper `cuda_core_ext::lease::enqueue_leased`, about +2 us per call. `cuda-core` lease API is public (`storage().acquire`, `retain_until_complete`).
- cuFFT rejects mixed NULL / non-NULL embeds natively; nvJPEG hands an encoded bitstream out exactly once; bitcomp stream = magic `NVZ\0`, 32-byte header, shortest real stream 56 bytes; the D3D12 heap import accepts DEDICATED; NVAPI on this machine reports 0 connected displays (the hardware test skips only HDR/VRR there); nvCOMP tests need `NVCOMP_ROOT=.sdk\nvcomp`.
- `camera-rs` has 8 other consumers on `main`: nv-demo-only behaviour is opt-in through `NavExtras`.

## Memory
`filesystem-mcp-rs` memory, workspace `nv-rs`, actor `claude`: summaries of this session. File memory: `~/.claude/projects/C--projects-projects-rust-cg-cglibs-nv-rs/memory/` (three feedback notes: no mechanical runs on the lead model, verify tests analytically, decide and push autonomously).
