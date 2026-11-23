# TODO - execution tracker for `plan2.md`

Source of truth for findings, IDs and verification status: [`plan2.md`](plan2.md) and the ledgers in `audit/2026-11-23/`. This file tracks execution only. An item is ticked when its fix is merged AND verified by the gate (targeted tests + `cargo clippy` on the touched crates), never on a worker's self-report.

Legend: `[ ]` open, `[~]` in progress (workers running or gate pending), `[x]` verified. A finding can also close as REFUTED (no change, reason recorded in `plan2.md`).

## Done
- [x] Audit: 12 slices, ledgers in `audit/2026-11-23/`, `plan2.md` (`8af9860`)
- [x] W4.1 `camera-rs` de-vendoring: `camera-rs` `9109108`, `nv-rs` `a4eb85e` (pushed)
- [x] W1.1 reproduction on GPU + `cuda_core_ext::lease::enqueue_leased` + `curand::generate_u32` (`8e84de4`)

## Wave 1 - wrapper crates (repair workers launched 2026-11-23, edit-only, disjoint crates)
Each worker re-verifies every finding (CONFIRMED / REFUTED / UNDECIDED) before changing code and adds a failing-before test.
- [x] `cublas`: BL-01..05, LT-01..08, LT-10, CB-01, CB-02
- [x] `cufft` + `curand`: FFT-01..06, RNG-01..03, lease the remaining `curand` device `generate_*`
- [x] `cusparse` + `cusolver`: P1..P11, S1..S4, S6, S7, X2 (context bind), scratch stream analysis
- [x] `cudnn`: N1..N4, N6..N13, workspace ownership check, scratch stream analysis
- [x] `nvjpeg` + `npp` + `nvofa`: JPG-1..6, NPP-1..4, OFA-1..4
- [x] `nvenc` + `nvdec`: ENC-1..4, ENC-6..11, DEC-1..9
- [x] `nvcomp` + `cufile` (CMP-4 fixed in the follow-up; `cufile` GDS paths unverified, no Linux host): CMP-1..12, CUF-1..5, CUF-7..10, `ssize_t` handling
- [x] `ngx` + `nvapi` + `nvml`: NGX-1..14, NVAPI-1..3, NVML-1..2
- [x] `cupti` + `nccl` + `nvrtc` (`nccl` compiled only; no Linux host): CUPTI-1..4, 6, 7, NVRTC-1..2, NCCL-1..4, 6..8, null `CStr` checks
- [x] Gate W1+W2 (clippy clean, 379 passed / 0 failed, GPU tests per crate; results in `plan2.md` "Gate result"): one serial pass - `cargo clippy` + tests for every touched crate, then `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` once
- [ ] Lead review W1: read the diffs of every P0 fix and check each new test analytically (does its assertion depend on the fixed line?); no revert-and-rerun mutation runs (operator decision 2026-11-23)
- [ ] Callers outside the wrapper crates reported by workers (`nv-demo`, `python/nv-py`): apply in one follow-up pass

## Wave 2 - OptiX and graphics interop (repair workers launched with Wave 1)
- [x] `optix`: A1..A10, D1..D3, C1, P1, X2, L1..L3, M1, PF1, PF2, DOC1, T1..T3, stream analysis
- [x] `nv-wgpu` + `cuda-core-ext`: CE-01..08, CE-10, CE-11, NW-01..19, NW-21..24
- [x] Gate W2 (with Gate W1) + GPU tests marked `#[ignore]` run explicitly on this machine
- [ ] Lead review W2: `nv-wgpu` encoder-based submit diff

## Wave 1-2 follow-up (done; gate: clippy clean, 375 passed / 0 failed / 203 ignored, ignored GPU tests of all touched crates pass)
- [x] `nvcomp` CMP-4: validate the bitcomp input before launching the kernel; the truncated-input test must be rejected before any GPU work
- [x] `nvjpeg`: a second `retrieve_bitstream` for the same encode returns a typed error instead of zeros
- [x] Deciding tests turned into regression tests of the observed behaviour or removed: `cufft` mixed NULL embed, `nvdec` DEC-3, `nv-wgpu` CE-06
- [x] `nvapi` `live_rtx_3080_ti_nvapi_queries` failure: environmental (0 connected displays reported by NVAPI on this machine); the test now skips only the HDR/VRR part in exactly that case
- [x] `nv-demo` GPU tests in `cuda_crowd.rs` marked `#[ignore = "GPU"]` (80 non-ignored / 4 ignored); the DEM/walkway test now really compares GPU against CPU
- [x] Out-of-manifest items (plus `set_epilogue` / `set_bias_batch_stride` made `unsafe fn`): `cublas` `set_emulation_desc` retention, `gemm_*_on_bound_stream` and `cudnn` `GraphConvPlan::build*` context bind, `optix` one-stream check for IAS children

## Wave 1b - cross-crate items (must not run in parallel with Wave 1)
- [ ] W1.6a Typed teardown error sink replacing the `CUDA_ERROR_UNKNOWN` mapping (BLA:ERR-01, SOL:X4, MED:ENC-5, MED:CUF-6, SYS:CUPTI-5, SYS:NCCL-5, OPX:X1)
- [ ] W1.6b One `Scratch` owner for `cublas`, `cufft`, `cudnn`, `cusolver`, `cusparse`, `nvcomp` (SOL:X3, SOL:N1, MED:CMP-6)
- [ ] Remaining duplication inside wrappers: OPX:S1..S4, SOL:S5, SOL:N5, BLA:LT-09, MED:X-2, IOP:NW-20, IOP:CE-09

## In flight right now (2026-11-23; edit-only workers, one gate per workspace after them)
- [~] Wave 3 `nv-demo`, four workers on disjoint files: renderer + scene host (running); main / ui / buses / persist (running); shaders + crowd + bloom + export + profiler (edits done, pre-gate); rail + anim + MP4 muxer (edits done, pre-gate)
- [~] Wave 5: `nv-loader` + `nv-bindgen` + all `*-sys` incl. the single `nv_loader::catalog` (running); `xtask` (edits done, pre-gate; catalog items X-2/X-3/X-4 wait for the loader worker)
- [x] Wave 5 `bootstrap.py`, CI, `deny.toml` (`a246a6a`; workflows not executed, `cargo deny` not run)
- [~] Wave 6 `python/nv-py`: foundation done (`072dd51`: DLPack validation, `check_buf`, `enqueue_leased`; corruption reproduced from Python 10/10 and 0/10 with the lease); three workers running on `video`/`gfx_img`, `compute`/`sparse_ai`/`nvjpeg`/`nvcomp`, `platform_ext`/`sys_ext`/`nvml`; the crate does not build until they land
- [x] Gate root workspace (Waves 3 + 5): clippy clean, 467 passed / 0 failed / 204 ignored, ignored tests of touched crates pass, `nv-demo` starts and renders (Old Gundo, DLSS RR, CUDA crowd); report `audit/2026-11-23/gate_root_w3_w5.md`. Not checked interactively: other AOVs, the four projections, MP4, SDG burst/cancel, Traffic toggle, day/night, cursor modes
- [x] Gate `python/` workspace: builds, clippy clean, 19 lib + 5 GPU tests, pytest 77/82 (remaining: NVAPI 0 displays, stale `.pyi`, cuDNN two-copies issue); report `audit/2026-11-23/gate_python_w6.md`
- [x] Wave 1b done (report `audit/2026-11-23/wave_1b.md`): context compare by value (`cuda_core_ext::same_context`), typed teardown sink (`cuda_core_ext::teardown`), one `Scratch` (`cuda_core_ext::scratch`), `cusparse` scratch parameter removed, dead `Budget`/`Unsupported` variants removed, cuDNN adopts an already-loaded copy, `.pyi` regenerated. Root: clippy clean, 467 passed / 0 failed / 205 ignored, ignored GPU tests pass. Python: clippy clean, 19 + 5 GPU tests, pytest 80 passed / 1 failed (NVAPI 0 displays, environment) / 1 skipped after `tests/conftest.py` imports torch first
- [x] Regenerate `.pyi` stubs (`cargo xtask py-stubs`) and rerun pytest
- [x] (superseded by Wave 1b) cuDNN in a process that already loaded another cuDNN 9 (torch): decision = adopt the already-loaded copy (use its directory, version-gated) instead of rejecting it; document that nv-rs loading cuDNN first breaks a later `import torch` (cannot be prevented from Rust). Today `cudnn-sys` rejects a foreign copy with a clear message (`reject_foreign_copy`)
- [x] (done in Wave 1b) Context identity: decision = compare `Context` by value (as `cuda-core` itself does in `RawDeviceBuffer::acquire`) in every wrapper instead of `Arc::ptr_eq` (about 40 sites; fixes nv-py cuDNN activation/softmax on DLPack imports)
- [ ] `ui.rs` animation sliders take their ranges from `demo_anim::anim_range` (UI-wide change, left by the gate)
- [ ] Follow-ups already known for the Wave 3 gate: rail `duration_s` survives scene rebuild like `looping` / `ease_io` (lead decision); `main.rs` reports video-finish errors in the status line; `ui.rs` takes animation ranges from `demo_anim::anim_range`; previous classic transforms for object motion (`GPU:D03`); fractal shape parameters in probe / SDG launches (`GPU:D21`); `prev_camera` during SDG bursts; `nvrtc` `dlto()` passes `--dlto`, which NVRTC 13.3 rejects
- [ ] Adopt csg-rs `main` (`0a73fca`, `125b870`; details in `csg-rs/HANDOFF.md`) AFTER the root gate is green: update csg-* in `Cargo.lock`; `cuda_crowd.rs` compiles `csg_actors::pod::actor_sim_cuh_source()` instead of pasting `ACTOR_SIM_CUH` (else NVRTC `#error "actor_sim.cuh needs the generated constant block"`); `ActorStatePod` 96 B (foot_lock), `ArchetypeMetaPod` 272 B (limb_mounts; unused chain index `u32::MAX`); `step_and_emit_actors_cuda` gains trailing `lod2_pawn_compound`, `archetypes` mandatory; `GpuInstance::flags` bits 16..24 = semantic override: use `csg_instance_semantic(inst.flags, hit.sem_id)` wherever `raytracer.cu` writes/compares a hit's semantic id and keep those bits when masking flags; removed from `csg_eval.cuh`: `csg_xform_pt/vec/normal_from_inv`, `csg_intersect_box_direct`, `csg_intersect_prim_direct`; evaluator overflow now returns a miss (closes `GPU:E01`); call `world.set_crowd_culling(false)` when stepping the crowd on the GPU. Verify: `cargo test -p nv-demo cuda_crowd -- --ignored` + app start. The csg-rs session held its own nv-demo agent (it changed no nv-rs file) and handed over the full change list with line numbers: `audit/2026-11-23/csg_rs_125b870_adoption.md` (also: `MAX_GPU_COMPOUNDS` instead of literal 256, log `world.library.take_issues()`, `InstanceId.0` is no longer a dynamic index, `sdg_export.rs` crowd tagging by `DynamicKind`, `CrowdSim::INVALID_ACTOR`).
- [ ] After the loader worker: `nv-bindgen` writes an expected-files manifest and a prebuilt-fallback marker for `xtask` (today `xtask` infers both); `xtask` consumes `nv_loader::catalog`, adds `dlssd` and companion DLLs; `release.yml` Windows step must check `$LASTEXITCODE`

## Wave 3 - `nv-demo` (after Waves 1-2: depends on `optix`, `nv-wgpu`, `ngx`, `nvenc`, `nvjpeg` APIs)
- [ ] W3.1 Render-thread failures as events + UI liveness check (RND:R5, APP:F12); remove `.ok()` on AOV buffers and bloom (RND:R6, RND:R7)
- [ ] W3.2 RND:H1, RND:R3, GPU:D05, RND:H2
- [ ] W3.3 RND:R4, RND:R2 (+ cancel command), RND:R1, RND:R10
- [ ] W3.4 SDG data: GPU:D01..D04 (D04 needs a `csg-rs` decision on `part_id`)
- [ ] W3.5 Capture and persistence: APP:F01, F10, F03, F07, F17; CAM:D01, D02
- [ ] W3.6 Frame loop and UI: APP:F02, F04, F05, F06, F09, F11, F08
- [ ] W3.7 Remaining `nv-demo` P2 items from ledgers 08-11
- [ ] Operator check on hardware: interactive camera feel after the `camera-rs` switch (RMB hold-to-fly cursor request is `Hidden` upstream)

## Wave 4 - single source of truth
- [x] W4.1 `camera-rs`
- [ ] W4.2 One DLL spec catalog (LDR:L-13, SYS:X-3)
- [ ] W4.3 Function-table loader macro (LDR:L-25, L-26), `nv-demo` duplicates (CAM:D09, CAM:D12, APP:F24, RND:R21, RND:H5, RND:H6, GPU:D07)

## Wave 5 - build, release, CI
- [ ] W5.1 `nv-bindgen` / `nv-loader`: LDR:L-01..L-12, L-14..L-24, L-27..L-37
- [ ] W5.2 `xtask`: SYS:X-1, X-2, X-4..X-8, X-10..X-12
- [ ] W5.3 `bootstrap.py`: SYS:B-1, B-2, B-7, B-8
- [ ] W5.4 CI and config: SYS:B-3..B-6, B-9; GPU:D26

## Wave 6 - `device/` and `python/`
- [x] W6.1 `device/` (done 2026-11-23, report `audit/2026-11-23/wave_6_device.md`): `[patch]` to the fork inside `device/` only, one `cuda-core` in the graph, `cargo oxide build/run --arch sm_86` green, all 15 GPU cases pass; D2 (u16/i16 params now passed as `.b32`), D4, D5, D6, D7, D8, D9 fixed, D3 documented (cannot be removed: `ptx_asm!` expands to `cuda_device::ptx`). Open: CI job that builds `device/`; refresh the fork when the root needs an NVIDIA-only API. Original item: DPY:D1 (`[patch]` to the fork inside `device/`, then build and run the 15 GPU cases), D2..D9
- [ ] W6.2 `python/nv-py`: DPY:P1..P17, V1..V22, G1..G7, X1..X3, C1..C20, PE-01..PE-08, SE-01..SE-13, NV-01

## Wave 7 - documentation
- [ ] W7.1 `crates/nv-demo/AGENTS.md`, root `AGENTS.md`, `plan1.md:33`, `optix` module docs; record every REFUTED finding in `plan2.md`

## Not run yet (must be run before the plan can be called complete)
- [ ] `cargo test --workspace` baseline on this machine
- [ ] Ignored GPU tests of every crate
- [ ] Any Linux-only branch (`cufile` GDS, `nccl`, Linux loader paths): no Linux host in this session
