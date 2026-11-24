# Final wave - all open plan2 / TODO items (2026-11-23)

Base `ee08ade`. RTX 3080 Ti, CUDA 13.3, OptiX 9.1, Windows 11. Five edit-only workers on disjoint files (loader/bindgen/xtask, nv-demo, wrappers + nv-wgpu, python, CI); the lead ran every cargo command, one at a time. Plan files (`plan2.md`, `TODO.md`, `HANDOFF.md`, root `AGENTS.md`, `DIAGRAMS.md`) are NOT edited by this wave (coordinator: the lead merges them into `PLAN.md`); their status changes are in the last section.

## Item ledger

Verdict | evidence | change | test. Line numbers are at this commit.

### W5.2 / SYS:X-2 / X-3 / X-4 - `xtask` uses the catalog
- CONFIRMED: `xtask/src/stage_dlls.rs` kept its own `SPECS` table; `release.yml` globbed the NVRTC builtins by hand; `dlssd` was in the catalog but not shipped.
- Change: `SPECS` and `npp_dll!` deleted, `stage_with(&[CatalogEntry])` over `nv_loader::catalog::ALL`, typed `StageError` (`MissingRequested`, `MissingCompanion { lib, pattern, main }`, `Io`). `crates/nv-loader/src/catalog.rs`: `Companion { pattern, required }`, `CatalogEntry::{companions_windows, companions_linux, companions()}`; data: cuDNN `cudnn_*64_*.dll` (required) + `zlibwapi.dll` (optional), NVRTC builtins, `nvJitLink` for cuSOLVER / cuSPARSE. `release.yml` stages `cufft,nvjpeg,nvrtc,cupti,dlss,dlssd` and the manual builtins globs are gone (both OSes).
- Tests: `dlss_and_dlssd_are_staged_from_the_catalog`, `missing_requested_library_fails_and_copies_nothing`, `optional_libraries_stay_warnings_when_requested`, `companions_ship_with_their_library_and_a_missing_required_one_fails`, `catalog::companions_match_siblings_and_never_the_main_file`.

### nv-bindgen manifest and prebuilt marker
- CONFIRMED (HANDOFF deviation "xtask prebuilt completeness inferred").
- Change: `nv_bindgen.rs` `Manifest { lib, origin, files }`, `Origin::{Generated, Prebuilt}`; `<lib>_manifest.json` is deleted when generation starts and written last; a promised file that is missing is a build error; a prebuilt-fallback build prints `cargo:warning` and records `origin: prebuilt`; `exports_sys!` / `query_interface_sys!` / `split_exports_sys!` expose `BINDINGS_MANIFEST`. `xtask/src/prebuilt.rs` stages exactly the files of a `generated` manifest and rejects out dirs without one. Lead fix during the gate: `prebuilt.rs` also rejected sets whose files were newer than the inventory; that inference rejected every `cupti-sys` build (its `build.rs` appends `CUpti_ActivityKernelLatest` after `generate`), so the manifest's time is now the generation time and file times are not compared.
- Tests: `manifest_requires_every_promised_file_and_records_the_origin`, fixture `the_manifest_lists_every_promised_file_and_says_generated`, `only_the_files_the_manifest_lists_are_staged`, `builds_that_used_the_prebuilt_fallback_are_not_candidates`, `a_promised_file_missing_from_the_out_dir_fails_and_stages_nothing` (replaces the mtime test).

### Loader ledger leftovers (`04_loader_bindgen_sys.md`)
| ID | Verdict | Evidence / change |
| --- | --- | --- |
| L-27 | CONFIRMED (vacuous test already gone, constants still hand-copied) | `Shape::FunctionTable { probes }`: the generator emits `static const unsigned int NV_RS_PROBE_<name> = (expr);` and requires a `pub const` back; `nvenc-sys` probes `NVENCAPI_VERSION`, `NVENCAPI_STRUCT_VERSION(0)`, `NV_ENCODE_API_FUNCTION_LIST_VER`. Test `probed_versions_agree_with_the_header_constants`. `nvenc-sys/prebuilt/.../nvenc_types.rs` regenerated (adds only the three probes; same SDK). |
| L-30 | CONFIRMED | generated NVAPI query shims resolve each id once into a `static OnceLock<usize>`; test `query_interface_shims_call_by_id_and_report_functions_without_one` extended. The committed NVAPI prebuilt shims were NOT regenerated (see "Not done"). |
| L-31 | (b) REFUTED as open (zlib failure already in `diag::record_event`, `cudnn_sys.rs:110-119`); (a) CONFIRMED, not changed | see "Not done" |
| L-32 | CONFIRMED | one policy, documented in `nv-bindgen` module docs and `nccl-sys/build.rs`: NCCL and cuFile are required on Linux targets; NGX stays a warning (gated `cfg(has_ngx_sdk)`, wrapper has a stub path) |
| L-33 | REFUTED as open (all four use `target_os()` / `target_arch()`) | `generate_on_os(spec, os)` added and used by nvapi / cufile |
| L-36 / `ngx_sys::SDK_BIN` | CONFIRMED for `SDK_BIN` | `option_env!` only under `debug_assertions`, `None` in release (`ngx_sys.rs`) |
| L-24, L-25, L-28, L-29, L-34, L-35, L-37 | REFUTED as open | already fixed in Wave 5 (`device_asm.rs:350`, `exports_sys!` `preload:`, `nvapi_sys.rs:7-8`, no nvjpeg shims, `diag.rs:46-52`, `msvc_arches`, pid-unique test dir) |
| L-35 family (lead finding) | CONFIRMED by measurement | a Linux-target build on this host got MSVC `vadefs.h` / `vcruntime.h`. `system_include_dirs` returns nothing unless the target is windows-msvc; reruns on `BINDGEN_EXTRA_CLANG_ARGS_<target>`; cross builds take SDK roots from the discovery variables; lead fix `cross_include_dir` (`nv_bindgen.rs:~2100`) uses the published layouts (`Interface/`, `NvOFInterface/`). Tests `msvc_includes_belong_to_msvc_targets_only`, `a_cross_build_without_an_sdk_root_names_the_variables`. |
| Linux loader search (HANDOFF step 5) | CONFIRMED (no globbing, no CUPTI dir; `lib/Linux_x86_64/rel` already present) | `glob_match`, versioned sonames `libfoo.so.N.*` newest first, `extras/CUPTI/lib64`, NGX runtime names as patterns. Tests `glob_match_handles_stars_anywhere_and_ignores_case`, `linux_versioned_files_follow_a_missing_soname_link_newest_first`, `linux_pattern_names_expand_in_the_ngx_runtime_dir_and_never_reach_dlopen`. |

### CI / W5.4
- `release.yml`: the multi-command pwsh staging step only propagated the last exit code -> `$ErrorActionPreference = 'Stop'` + `$PSNativeCommandUseErrorActionPreference = $true` (pwsh 7.4+); workflow permissions `contents: read`, publish job `contents: write`; `cp ... || true` removed.
- New build-only `device` job (`ci.yml:92-171`): `device/rust-toolchain.toml`, CUDA via the repo's `setup-cuda` action, OptiX headers via `bootstrap.py`, cargo-oxide from the fork at the revision in `device/Cargo.lock`, `cargo oxide build --arch sm_86`, then clippy `-D warnings`. Needs secret `CUDA_RUST_B_TOKEN` (fork is private); the 15 GPU cases stay a hardware run.
- `deny.toml`: license exceptions for `clipboard-win`, `error-code` (BSL-1.0), `tiny-keccak` (CC0-1.0), `smartstring` (MPL-2.0+).
- Validation: `actionlint` v1.7.12 (downloaded to the scratchpad): only `label "ubuntu-26.04" is unknown` (actionlint's label list; the runner image exists); with that ignored, no output, exit 0. **CI was not executed.** `cargo deny check`: see gate results.

### Linux-only code
- `rustup target add x86_64-unknown-linux-gnu` worked without admin. A glibc sysroot came from the installed WSL Ubuntu (`\\wsl.localhost\Ubuntu\usr\include`, NCCL in `/usr`, CUDA 13.3 in `/usr/local/cuda-13.3/targets/x86_64-linux`); Windows copies of the platform-neutral cuDNN / Video Codec / Optical Flow headers.
- Result: `cargo clippy --target x86_64-unknown-linux-gnu --workspace --all-targets -- -D warnings` is clean for every crate except `nvcomp-sys` / `nvcomp` (the Windows nvCOMP package's `nvcomp_export.h` uses `__declspec`; no Linux nvCOMP package here). It found three Linux-only defects, fixed: MSVC headers in cross builds (above), and `unnecessary_cast` where a C enum constant is `i32` under MSVC and `u32` under GCC (`cublas/src/lt.rs` reduction scheme, `OPTIX_INSTANCE_FLAG_DISABLE_ANYHIT` copied in three nv-demo files) -> bit-preserving `u32::from_ne_bytes(x.to_ne_bytes())`, and one `optix::INSTANCE_FLAG_DISABLE_ANYHIT` (`optix/src/accel.rs`) used by `renderer.rs`, `csg_scene_host.rs:58`, `cuda_crowd.rs:40`.
- Compile only. Nothing ran on Linux (no Linux host, no GDS, one GPU). `python/` was not cross-checked (PyO3 needs a Linux interpreter config).

### APP:F27 - unbounded waits
- nv-wgpu: CONFIRMED (`platform.rs` `wait_for_submission` polled with `timeout: None`; `sync.rs` `wait_indefinitely()`; failures printed). Now `wait_submission_bounded` (`VULKAN_SUBMIT_TIMEOUT`), `poll_wait_error`, drop failures recorded in `cuda_core_ext::teardown` under new `Library::NvWgpu`. Test `bounded_wait_errors_are_typed_and_map_to_teardown_codes`.
- nv-demo: REFUTED as open; every `device.poll` in `main.rs` already waits one submission with `GPU_COPY_WAIT_TIMEOUT` (2 s): `main.rs:1532`, `5456-5465`, `5591`. Added: `main()` returns `ExitCode` (two `expect`s gone) and prints the teardown sink once at exit (`teardown_report`, test `teardown_report_lists_each_failure_and_the_overflow_count`).
- Residual: `cuda_core_ext::ExternalSemaphore` drop synchronises the context (a driver call, cannot be bounded from Rust).

### GPU:D07 / GPU:D08
- D07 PARTLY CONFIRMED: `build.rs` picked the highest version, `renderer.rs` the first folder. New `crates/nv-demo/src/optix_sdk.rs` (`include_dir()`: `OPTIX_ROOT` first, an invalid `OPTIX_ROOT` is an error; else the highest `OPTIX_VERSION`), included by `build.rs` via `#[path]` and used at runtime. Tests `header_version_reads_the_exact_macro_only`, `highest_version_wins_regardless_of_order`.
- D08 PARTLY CONFIRMED: the PTX cache key hashed the include path, not the headers. `gpu_export::dir_content_digest` hashes every file under the OptiX include dir plus `csg_eval.cuh` bytes; a cached image OptiX rejects is evicted and recompiled once (`load_ptx_cached`). `build.rs` reruns on `optix.h`. Test `dir_content_digest_tracks_header_bytes_not_just_paths`. Lead fix: `compile_ptx_cached` is `#[cfg(not(has_embedded_ptx))]` (dead in embedded builds).

### W4.3 duplicates
| ID | Verdict | Change |
| --- | --- | --- |
| OPX:S1 | CONFIRMED | one `log_text` in `optix.rs` |
| OPX:S2 | CONFIRMED | `Primitive` is the only primitive table; `CurveType::primitive()`; test copy deleted; test `curve_types_map_to_distinct_primitives_through_one_table` |
| OPX:S3 | CONFIRMED | one `accel_build` FFI site + `BuildCall`; shared `instance_array_input` |
| OPX:S4 | CONFIRMED | `pipeline.rs`, `xform.rs`, `cluster.rs` use `accel::same_ctx` |
| SOL:S5 | CONFIRMED | `cusolver` uses `cusparse::Status(code).name()` (new dep `cusolver -> cusparse`) |
| SOL:N5 | CONFIRMED | `cudnn/graph.rs` `make_conv_desc`, `ConvTensors`, `make_conv_op`, `make_op_graph`, `select_plan` |
| BLA:LT-09 | CONFIRMED | `lt.rs` `check_stream`, `heuristic_raw/best/list`, `HeuristicQuery` |
| MED:X-2 | CONFIRMED (entry macro and nvdec "UNKNOWN" parts REFUTED) | `cuda_core_ext::media` (`yuv420_rows`, `yuv420_frame_bytes`, `detail_suffix`, `psnr_u8`, `copy_2d_*`); copies in nvenc / nvdec / nvofa / nvjpeg tests deleted. Bug fixed on the way: odd heights lost their last chroma row (`ceil(h/2)` now). Tests `yuv420_geometry_keeps_the_odd_chroma_row`, `psnr_u8_behaves_correctly`, `detail_suffix_is_empty_or_parenthesised`, GPU `pitched_copies_round_trip` |
| IOP:CE-09 | CONFIRMED | `SurfaceObject::from_level(&Arc<MipmappedArray>, level)` |
| IOP:NW-20 | (a) REFUTED (no runner left); (b)-(f) CONFIRMED | `platform::ownership_transfer`, `alloc_bind_exportable_dedicated`, `create_shared_committed_resource`, `SharedSurfaceObject` deleted, layouts via `wgpu_hal::vulkan::conv::derive_image_layout`; tests `ownership_transfer_release_and_acquire_are_mirror_images`, `layout_mapping_follows_wgpu_hal` |
| CAM:D09, CAM:D12, RND:H5, RND:H6 | REFUTED (already fixed) | `camera_rail.rs:17-26`, `demo_anim::anim_range`, `csg_scene_host.rs:1366-1373`, `:1411` |
| RND:R21 | CONFIRMED | `SceneState` deleted; `IasClassicState::Visible` carries the build inputs; test `ias_classic_state_carries_every_build_input_and_zeroes_idle_orbit_phase` |
| APP:F24 | PARTLY CONFIRMED | `ui::AOV_PASSES`, `renderer::DLSS_QUALITY_MODES`, `TIME_SCALE_MIN/MAX`, `SHUTTER_SUBFRAMES_MAX`, `ui::SKY_MODES`, `is_crowd_showcase`; 3 tests. Scene geo-origin table stays (needs a `SceneDescriptor` field in csg-rs) |

### DPY:C20 and python leftovers
- DPY:C20 CONFIRMED for nine classes (ledger said eight): `close()`, `is_closed`, `__enter__`, `__exit__` on `CublasHandle`, `CufftPlan1d`, `CurandGenerator`, `CusparseHandle`, `CusolverDnHandle`, `CudnnHandle`, `CompressionManager`, `NvjpegDecoder`, `NvjpegEncoder` (state in `Arc<Mutex<Option<State>>>`, close waits for in-flight calls, drains the stream, still releases on sync failure and returns the error; idempotent; use after close raises `NvInvalidArgumentError`). `NvencEncoder.close` / `Nvofa.close` now return the stream-drain error. Stubs regenerated (`xtask py-stubs generate`, check ok, 25 submodules). Test file `tests/test_close_lifecycle.py`.
- SE-05 REFUTED as open (already fixed in `sys_ext.rs`). V11, V14, P3, P6-P13, P15, G3, V22 closed (read); P14 accepted as valid (legacy capsule is allowed). `__dlpack__` precise wait: blocked, `cuda-core` keeps pending write events private (`allocation.rs:33` at `348895e`). P17 (stubs model submodules as classes) open.

### W7.1 docs
- `crates/nv-demo/AGENTS.md`: stale traps rewritten; OptiX discovery, PTX cache key, single-source tables, bounded waits, teardown report, measured frame profile, test count 146.
- `README.md`, `docs/.../crates/nv-demo.md`: csg-rs `24b22f5`, test counts. `docs/.../interop/nv-wgpu.md` (`SurfaceObject`), `getting-started/prebuilt-bindings.md` (manifest), `contributing/building-testing.md` (`stage-dlls` from the catalog, companions, failure), `python/04-modules-reference.md` (generated).
- `device/.optix_device_tests.oxide-pipeline.lock`: already untracked and ignored (`git ls-files device` lists only `Cargo.lock`; `device/.gitignore` has `/.*.oxide-pipeline.lock`).

## Performance

Instrument: `crates/nv-demo/src/perf_log.rs` (new, opt-in `NV_DEMO_PERF_CSV`; buffer allocated once, CSV written once; zero work when unset). Release build, persisted default scene (`csg_mode = 1`, traffic + crowd moving, so it renders continuously), camera restored from the same saved config before every run, window 2207x996, 4 spp, 6 bounces, denoiser / bloom / DLSS off (the persisted settings). 150 warm-up + 600 measured frames per run, 5 runs, csg-rs GPU runs paused for the window (chat #665-#667).

| Metric (ms) | median of run medians | per-run medians | median p95 |
| --- | --- | --- | --- |
| frame work (frame start to end-of-frame fence) | 51.17 | 56.95 52.46 51.17 46.50 46.48 | 59.70 |
| OptiX RT pass (GPU) | 50.12 | 55.92 50.66 50.12 45.55 45.59 | 58.58 |
| host prep before launch | 0.74 | 0.77 0.88 0.74 0.66 0.66 | 1.65 |
| CUDA compute (IAS / crowd / traffic) | 0.68 | 0.68 0.77 0.69 0.65 0.65 | 1.51 |
| tonemap | 0.089 | | 0.096 |
| crowd step (host) | 0.175 | | 0.322 |

- Hotspot 1 (98 %): the OptiX RT pass. Runs 1-3 started with 20-34 % GPU utilisation from other processes (sampled before each run), runs 4-5 with 0-2 %, which explains the 56 -> 46 ms drift; the clean figure is ~45.6 ms.
- Hotspots 2-3 (host prep 0.74 ms, CUDA compute 0.68 ms) together are under 3 % of the frame. No per-frame allocation, sync or redundant upload showed up in the numbers, so no host change can give a measurable gain here.
- **No speed-up was made, so there is no before/after pair.** Making the RT pass faster means changing the raygen / CSG shader. That needs Nsight Compute analysis, and the result could change the image. Speculative shader changes were out of bounds, so the next step is recorded instead: profile `__raygen__` / `hit_csg*` with Nsight Compute on this scene.
- Release runs: 5 x ~46-56 s plus a 108 s trial, stdout/stderr logs empty (`scratchpad\perf_final_*_{out,err}.log`), every PID stopped by the script that started it, user config restored after each run.

## Breaking changes
See `CHANGELOG.md` [Unreleased] "Changed - Breaking (final wave)": `xtask` catalog / companions / manifest-only prebuilt staging; `Shape::FunctionTable { probes }`; `CatalogEntry` companion fields; Linux `dlss` / `dlssd` names are patterns; `ngx_sys::SDK_BIN` `None` in release; NVAPI shims cache resolution; `nv_wgpu::SharedSurfaceObject` removed; `nvdec::{nv12_frame_bytes, p016_frame_u16s, psnr_u8}` removed; new deps `cusolver -> cusparse`, `nvdec -> cuda-core-ext`; python close semantics (methods raise after `close()`).

## Gate results (verbatim summaries)
- Root `cargo clippy --workspace --all-targets --message-format=short -- -D warnings`: `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 2m 29s` (only the existing `nvcomp-sys` prebuilt-bindings build warning).
- Root `cargo test --workspace --quiet` (`NVCOMP_ROOT=.sdk\nvcomp`): **500 passed / 0 failed / 207 ignored** (baseline 475 / 0 / 206).
- Ignored GPU tests (`-- --ignored --test-threads=1`), passed/failed: cuda-core-ext 12/0, nv-wgpu 13/0, optix 68/0, cublas 9/0, cudnn 5/0, cusolver 9/0, cusparse 11/0, nvenc 4/0, nvdec 2/0, nvofa 4/0, nvjpeg 7/0, nvenc-sys 1/0, nvapi-sys 1/0, nv-demo 5/0; nv-loader, ngx-sys, nccl-sys, xtask have no ignored tests on Windows (0/0).
- Linux cross: `cargo clippy --target x86_64-unknown-linux-gnu --workspace --all-targets -- -D warnings`: clean except `nvcomp-sys` build script (no Linux nvCOMP headers).
- `python/nv-py` (`CARGO_TARGET_DIR=python/nv-py/target`): clippy `-D warnings` `Finished`; `cargo test --lib`: 19 passed / 0 failed / 5 ignored; `--ignored --test-threads=1`: 5 passed; `maturin develop` + `pytest python/nv-py/tests` (torch first via conftest): **118 passed, 2 skipped** (no failure; the NVAPI 0-display case did not fail this time).
- `device/optix-device-tests`: `cargo oxide build --arch sm_86` `Finished`; `cargo oxide run --arch sm_86`: all 15 cases PASS (ptr64, shade, geometry, transform, hitobject, intersect, exception, callables, payload_types, symbols, const_index, coop_vec, texture, sparse, footprint). First run failed to compile `device/optix-device/build.rs` (missing new `probes` field); fixed.
- `cargo deny check`: see the addendum at the end (installed for this gate).
- `nv-demo` release: runs above.

## Hands-off areas (csg-rs session)
Only `crates/nv-demo/src/cuda_crowd.rs:40` and `csg_scene_host.rs:58` changed (the `OPTIX_INST_DISABLE_ANYHIT` constant now reads `optix::INSTANCE_FLAG_DISABLE_ANYHIT`; value 4, `cuda_crowd.rs:42` asserts it). `step_dynamic`, the upload path, `pack_dynamic_prefix_only`, the traffic-only repack and possess/follow retargeting are untouched.

## Not done, and why
- **Performance**: no optimisation. The measured cost is 98 % RT shader. Host-side waste is under 3 % of the frame. Shader optimisation needs Nsight Compute work and may change the image.
- **L-31(a)**: cuDNN still preloads every `cudnn_*` sibling eagerly. The alternatives (`AddDllDirectory` / `SetDllDirectory`) change the process-wide DLL search, and no test here covers what that does to other DLLs the process loads.
- **NVAPI and other committed prebuilt sets**: they come from newer SDKs than this machine has (NVAPI 62115 vs local 61683; cuDNN, NGX and OptiX header hashes also differ). Regenerating them here would downgrade them, so only `nvenc_types.rs` was regenerated. The committed NVAPI prebuilt shims still resolve the id on every call. Regenerate them on a machine with the matching SDKs (`cargo xtask prebuilt`).
- **Linux runtime** was never run: NCCL needs 2+ GPUs, cuFile needs GDS. `nvcomp` on Linux needs Linux nvCOMP headers. `python/` was not cross-compiled.
- **CI** was never executed. The `device` job needs the `CUDA_RUST_B_TOKEN` secret. git dependencies without a license field (`flex-geom`, `nodes-core`, `ui-theme`) need a `clarify` entry or an upstream license.
- **`__dlpack__` precise wait**: needs `cuda-core` to expose pending write events. **P17** (stub submodules): needs an xtask stub-generator change.
- **APP:F24 geo-origin table**: needs a csg-rs `SceneDescriptor` field.
- **Interactive visual checks** were not done by any gate: AOVs, projections, MP4, SDG burst, DLSS picture, classic motion, sky crossfade, camera feel.
- **TODO "Lead review W1 / W2"**: analytic review of the P0 diffs is a lead task and was not repeated here.

## Status changes for PLAN.md (lead applies)
- W4.3 -> done (this report, sections W4.3 and D07/D08; APP:F24 geo-origin remains, csg-rs).
- W5.2 -> done (catalog, `dlssd`, companions, loud failure, `release.yml`).
- W5.4 -> written and statically validated (actionlint); still not executed; `device` job added.
- W7.1 -> done for `crates/nv-demo/AGENTS.md`, README, docs site pages listed above; root docs are the lead's.
- TODO "xtask consume catalog / manifest / marker / `$LASTEXITCODE`" -> done.
- TODO "Remaining duplication inside wrappers" -> done (see table).
- TODO "python close()/__exit__ (DPY:C20)" -> done; `__dlpack__` precise wait stays open (blocked upstream).
- TODO "device/: CI job; git rm --cached lock" -> CI job done (not executed); lock file already untracked.
- TODO "Linux-only code never compiled" -> compiled + clippy clean on `x86_64-unknown-linux-gnu` (except nvcomp); never run.
- TODO "Remaining nv-demo follow-ups" GPU:D07/D08 and APP:F27 -> done (F27 nv-demo part was already bounded).
- HANDOFF step 5 (Linux loader globbing, CUPTI dir) -> done.
- Commits: listed in the commit section of the final message / `git log` after `ee08ade`.

## Addendum - `cargo deny check` (cargo-deny installed for this gate)
- First run: `deny.toml` did not parse (`expected a WITH here` at `allow = ["MPL-2.0+"]`) -> `MPL-2.0`.
- Then: `advisories FAILED` with RUSTSEC-2026-0592 (`url` 2.5.8, stack overflow in `Url::origin()`, via `webbrowser` <- `egui-winit` <- `nv-demo`) -> `cargo update -p url` (2.5.9, plus `form_urlencoded` 1.2.3; lockfile only; `nv-demo` clippy clean after it). `epaint_default_fonts` (`OFL-1.1`, `Ubuntu-font-1.0`, egui's bundled fonts) -> per-crate exception.
- Final: `advisories ok, bans ok, licenses FAILED, sources ok`. The remaining errors are `flex-geom`, `nodes-core`, `ui-theme` = 0.1.0 "unlicensed" (git crates from the operator's own repositories with no `license` field). A license cannot be invented here: add `license` upstream or a `[[licenses.clarify]]` entry (operator decision). `bans` reports duplicate-version warnings only.
