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
- [~] `nvcomp` + `cufile` (CMP-4 confirmed on hardware and still open; `cufile` GDS paths unverified, no Linux host): CMP-1..12, CUF-1..5, CUF-7..10, `ssize_t` handling
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

## Wave 1-2 follow-up (in progress)
- [~] `nvcomp` CMP-4: validate the bitcomp input before launching the kernel; the truncated-input test must be rejected before any GPU work
- [~] `nvjpeg`: a second `retrieve_bitstream` for the same encode returns a typed error instead of zeros
- [~] Deciding tests turned into regression tests of the observed behaviour or removed: `cufft` mixed NULL embed, `nvdec` DEC-3, `nv-wgpu` CE-06
- [~] `nvapi` `live_rtx_3080_ti_nvapi_queries` failure: establish cause
- [~] Out-of-manifest items: `cublas` `set_emulation_desc` retention, `gemm_*_on_bound_stream` and `cudnn` `GraphConvPlan::build*` context bind, `optix` one-stream check for IAS children

## Wave 1b - cross-crate items (must not run in parallel with Wave 1)
- [ ] W1.6a Typed teardown error sink replacing the `CUDA_ERROR_UNKNOWN` mapping (BLA:ERR-01, SOL:X4, MED:ENC-5, MED:CUF-6, SYS:CUPTI-5, SYS:NCCL-5, OPX:X1)
- [ ] W1.6b One `Scratch` owner for `cublas`, `cufft`, `cudnn`, `cusolver`, `cusparse`, `nvcomp` (SOL:X3, SOL:N1, MED:CMP-6)
- [ ] Remaining duplication inside wrappers: OPX:S1..S4, SOL:S5, SOL:N5, BLA:LT-09, MED:X-2, IOP:NW-20, IOP:CE-09

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
- [ ] W6.1 `device/`: DPY:D1 (`[patch]` to the fork inside `device/`, then build and run the 15 GPU cases), D2..D9
- [ ] W6.2 `python/nv-py`: DPY:P1..P17, V1..V22, G1..G7, X1..X3, C1..C20, PE-01..PE-08, SE-01..SE-13, NV-01

## Wave 7 - documentation
- [ ] W7.1 `crates/nv-demo/AGENTS.md`, root `AGENTS.md`, `plan1.md:33`, `optix` module docs; record every REFUTED finding in `plan2.md`

## Not run yet (must be run before the plan can be called complete)
- [ ] `cargo test --workspace` baseline on this machine
- [ ] Ignored GPU tests of every crate
- [ ] Any Linux-only branch (`cufile` GDS, `nccl`, Linux loader paths): no Linux host in this session
