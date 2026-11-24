# TODO - execution tracker for `plan2.md`

Source of truth for findings, IDs and verification status: [`plan2.md`](plan2.md) (see its Status block) and the ledgers in `audit/2026-11-23/`. This file tracks execution only. An item is ticked when its fix is merged AND verified by the gate (targeted tests + `cargo clippy` on the touched crates), never on a worker's self-report.

Legend: `[ ]` open, `[~]` in progress, `[x]` verified. A finding can also close as REFUTED (no change, reason recorded in `plan2.md`).

HEAD `1467778`. Nothing is running except one agent finishing the csg-rs adoption branch (`csg-rs-adoption`).

## Done
- [x] Audit: 12 slices, ledgers in `audit/2026-11-23/`, `plan2.md` (`8af9860`)
- [x] W4.1 `camera-rs` de-vendoring: `camera-rs` `9109108`, `nv-rs` `a4eb85e` (pushed)
- [x] W1.1 reproduction on GPU + `cuda_core_ext::lease::enqueue_leased` + `curand::generate_u32` (`8e84de4`)
- [x] Wave 1 wrapper crates (`cublas`, `cufft` + `curand`, `cusparse` + `cusolver`, `cudnn`, `nvjpeg` + `npp` + `nvofa`, `nvenc` + `nvdec`, `nvcomp` + `cufile`, `ngx` + `nvapi` + `nvml`, `cupti` + `nccl` + `nvrtc`) and Wave 2 (`optix`, `nv-wgpu` + `cuda-core-ext`): gate `05efa7e` (clippy clean, 379 passed / 0 failed; follow-up gate 375 passed / 0 failed / 203 ignored, ignored GPU tests of all touched crates pass). `cufile` GDS and `nccl` never ran (no Linux host); `nccl` only compiled.
- [x] Wave 1-2 follow-up: `nvcomp` CMP-4 header validation, `nvjpeg` second retrieve is a typed error, deciding tests turned into regression tests (`cufft` mixed NULL embed, `nvdec` DEC-3, `nv-wgpu` CE-06), `nvapi` hardware test skips only HDR/VRR when NVAPI sees 0 displays, `nv-demo` `cuda_crowd.rs` GPU tests `#[ignore = "GPU"]`, out-of-manifest items (`cublas` emulation desc retention and `unsafe` epilogue setters, `cudnn` `GraphConvPlan::build*` context bind, `optix` one-stream check)
- [x] Wave 3 `nv-demo` (renderer + host, main / ui / buses / persist, shaders + crowd + bloom + export + profiler, rail + anim + MP4 muxer) and Wave 5 (`nv-loader` single `nv_loader::catalog`, `nv-bindgen` no prebuilt fallback on generator failure, `*-sys`, `xtask`): root gate `4f457d0`, report `audit/2026-11-23/gate_root_w3_w5.md` (clippy clean, 467 passed / 0 failed / 204 ignored, `nv-demo` starts and renders Old Gundo with DLSS RR and the CUDA crowd)
- [x] Wave 5 `bootstrap.py`, CI, `deny.toml` (`a246a6a`; workflows never executed, `cargo deny` never run)
- [x] Wave 6 `python/nv-py`: gate `8f7af2f`, report `audit/2026-11-23/gate_python_w6.md` (clippy clean, 19 lib + 5 GPU tests)
- [x] Wave 6 `device/`: `c04d692`, report `audit/2026-11-23/wave_6_device.md` (`[patch]` to the `cuda-rust_b` fork inside `device/` only, one `cuda-core` in the graph, 15 of 15 GPU cases pass on sm_86; D2, D4..D9 fixed, D3 documented)
- [x] Wave 1b cross-crate: `1467778`, report `audit/2026-11-23/wave_1b.md` (clippy clean, 467 passed / 0 failed / 205 ignored, ignored GPU tests pass; pytest 80 passed / 1 failed (NVAPI 0 displays, environment) / 1 skipped)
  - [x] W1.6a typed teardown sink `cuda_core_ext::teardown` (BLA:ERR-01, SOL:X4, MED:ENC-5, MED:CUF-6, SYS:CUPTI-5, SYS:NCCL-5, OPX:X1)
  - [x] W1.6b one `Scratch` in `cuda_core_ext::scratch` for `cublas`, `cudnn`, `cusolver`, `cusparse`, `nvcomp` (`cufft` wraps it; SOL:X3, SOL:N1, MED:CMP-6)
  - [x] Context identity by value (`cuda_core_ext::same_context`), `cusparse` scratch parameter removed, dead `Budget` / `Unsupported` variants removed, cuDNN adopts an already-loaded copy (torch-first order documented; `tests/conftest.py` imports torch first), `.pyi` stubs regenerated
- [x] Wave 3 follow-ups done by the root gate: rail `duration_s` survives a scene rebuild, video-finish errors go to the status line, `nvrtc` `dlto()` passes `-dlto`

## Open
- [ ] Lead review W1: read the diffs of every P0 fix and check each new test analytically (does its assertion depend on the fixed line?); no revert-and-rerun mutation runs (operator decision 2026-11-23)
- [ ] Lead review W2: `nv-wgpu` encoder-based submit diff
- [ ] `ui.rs` animation sliders take their ranges from `demo_anim::anim_range` (UI-wide change; `spin_speed` is already reachable)
- [ ] Remaining `nv-demo` follow-ups: previous classic transforms for object motion (`GPU:D03`); fractal shape parameters in probe / SDG launches (`GPU:D21`); `prev_camera` during SDG bursts; `params_buf.copy_from_host` synchronises every frame (make it async); hash `optix.h` in the PTX cache key and share `build.rs` OptiX discovery (`GPU:D07`, `D08`); general sky crossfade slots (`RND:R10`); persist the dock layout (`APP:F19`); `optix::InstanceBuffer` pinned staging grows on demand (`RND:H13`); `nv-wgpu` `submit_vulkan_commands` waits with `timeout: None`
- [ ] Adopt csg-rs `main` (`0a73fca`, `125b870`): branch `csg-rs-adoption` is being finished by a separate agent (rebased onto `4f457d0`, one `nv-demo` test was still failing when last reported). Change list with line numbers: `audit/2026-11-23/csg_rs_125b870_adoption.md`. Verify: `cargo test -p nv-demo cuda_crowd -- --ignored` + app start
- [ ] Interactive checks of `nv-demo` that no gate performed: other AOVs (Depth, Motion, Albedo, Normal, WorldPos, Semantic, Instance), the four sensor projections, MP4 recording, SDG burst and cancel, Traffic toggle, day/night crossfade, cursor modes, DLSS SR picture, camera feel after the `camera-rs` switch (RMB hold-to-fly cursor request is `Hidden` upstream)
- [ ] `xtask`: consume `nv_loader::catalog`, ship `nvngx_dlssd.dll` and companion DLLs; `nv-bindgen` writes an expected-files manifest and a prebuilt-fallback marker (today `xtask` infers both); `release.yml` Windows step checks `$LASTEXITCODE`
- [ ] Remaining duplication inside wrappers: OPX:S1..S4, SOL:S5, SOL:N5, BLA:LT-09, MED:X-2, IOP:NW-20, IOP:CE-09; `nv-demo` FOV / animation / AOV / SDK-discovery duplicates (W4.3)
- [ ] `python/nv-py`: `close()` / `__exit__` for the eight classes (`DPY:C20`); precise `__dlpack__` wait needs a `cuda-core` API exposing pending write events (today a context-wide synchronize)
- [ ] `device/`: CI job that builds it; `git rm --cached device/.optix_device_tests.oxide-pipeline.lock` (lead decision); refresh the fork when the root needs an NVIDIA-only `cuda-core` API
- [ ] Wave 7 documentation: `crates/nv-demo/AGENTS.md` (another agent is merging a branch there), `optix` module docs; root `AGENTS.md`, `crates/AGENTS.md`, `plan1.md`, `DIAGRAMS.md`, `plan2.md` and this file are updated. Every REFUTED finding recorded in `plan2.md` (per-item loader verdicts are not recorded as a list)
- [ ] Final full gate of both workspaces after the csg-rs adoption lands

## Not run yet (must be run before the plan can be called complete)
- [ ] Linux-only code never compiled or run: `cufile` GDS, `nccl`, Linux loader search paths, the Linux branches of `cudnn-sys` `loaded_copy` and of `python/nv-py` `platform_ext.rs` (checked by reading only), Linux CI
- [ ] CI workflows and `cargo deny` never executed
- [ ] Ignored GPU tests of crates not touched by Wave 1b (green at `05efa7e`, not rerun since)
- [x] `cargo test --workspace` baseline on this machine (467 passed / 0 failed / 205 ignored at `1467778`)
