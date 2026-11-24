# nv-rs - single authoritative plan

Consolidated 2026-11-23 from the old `PLAN.md` (M1..M6), `plan1.md`, `plan2.md`, `TODO.md` and `HANDOFF.md`. Nothing was dropped: the full originals are archived verbatim (see [Archive](#archive)). Resume pointer for a new session: [`HANDOFF.md`](HANDOFF.md). Repository guide: [`AGENTS.md`](AGENTS.md); diagrams: [`DIAGRAMS.md`](DIAGRAMS.md).

Item IDs are stable. `<PREFIX>:<id>` points into the per-slice ledgers `audit/2026-11-23/NN_*.md` (prefixes: `OPX` 01, `BLA` 02, `SOL` 03, `LDR` 04, `MED` 05, `IOP` 06, `SYS` 07, `CAM` 08, `RND` 09, `GPU` 10, `APP` 11, `DPY` 12). `Wn.m` are remediation-wave items (full text: `audit/2026-11-23/plan2_full.md`). An item is closed only when its fix is merged AND verified by a gate, never on a worker's self-report.

## 1. Status

State as of `git log -1` = `ee08ade` (`fix(nv-demo): follow-up wave - classic motion (ABI 648 B), ...`), 2026-11-23. Hardware: RTX 3080 Ti (sm_86), CUDA 13.3, OptiX 9.1, Windows 11.

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

### In flight

- **Final wave** (lead-launched agent): will write `audit/2026-11-23/wave_final.md`. At the time of this consolidation that file did not exist and its scope was not read; treat its report, when present, as the source for what it closed.
- **csg-rs session** (chat name `Mullis-16-3934`): `R00` GPU crowd steering moved into a csg-rs-owned kernel; `R00b` GPU traffic. Branches from `origin/main`; they ping in chat for review, the lead reviews and merges.
- Another agent edits `CHANGELOG.md` and `crates/*/AGENTS.md` concurrently.

### Open

See section 2 (every unticked item, deduplicated).

## 2. Open items

Owner `lead` = this repository's session; `csg-rs` = the csg-rs session. Pointer = ledger or report holding the detail.

### Verification and review

| ID | Description | Owner | Pointer |
| --- | --- | --- | --- |
| `REV:W1` | Read the diffs of every P0 fix of Wave 1 and check each new test analytically (does its assertion depend on the fixed line?). No revert-and-rerun mutation runs (operator decision 2026-11-23). | lead | `plan2_full.md` Wave 1; `TODO_final.md` Open |
| `REV:W2` | Review the `nv-wgpu` encoder-based submit diff. | lead | `plan2_full.md` W2.2; `06_coreext_wgpu.md` |
| `GATE:FINAL` | Final full gate of both workspaces (root + `python/`; `device/` rebuild) after the final wave and the csg-rs R00/R00b work land. | lead | section 6 |
| `VIS:DEMO` | Interactive checks of `nv-demo` that no gate performed: AOVs (Depth, Motion, Albedo, Normal, WorldPos, Semantic, Instance), the four sensor projections, MP4 recording, SDG burst and cancel, Traffic toggle, day/night crossfade, cursor modes (upstream requests `Hidden` during RMB hold-to-fly), DLSS SR picture, camera feel after the `camera-rs` switch, plus classic object motion under DLSS / temporal denoiser with auto-rotate or sat orbit, parameterised-to-parameterised sky crossfade, SDG occlusion against the fractal. | lead (needs a person at the machine) | `plan2_full.md` Status; `wave_nvdemo_followups.md` Risks |
| `GPU:RERUN` | Ignored GPU tests of crates not touched by Wave 1b (green at `05efa7e`, not rerun since). | lead | `TODO_final.md` Not run yet |
| `LNX:ALL` | Linux-only code never compiled or run: `cufile` GDS, `nccl` (also needs 2+ GPUs), Linux loader search paths (needs globbing, `lib/Linux_x86_64/rel`, `extras/CUPTI/lib64`), Linux branches of `cudnn-sys` `loaded_copy` and `python/nv-py` `platform_ext.rs` (read-checked only), Linux CI. | lead (needs a Linux host) | `plan2_full.md` Status; `TODO_final.md` Not run yet |
| `SYS:NGX-HARNESS` | Vulkan test harness for `ngx` (`ash` as dev-dependency); `SYS:NGX-2`, `SYS:NGX-7`, `SYS:NGX-11` have documentation only. | lead | `07_system_xtask.md` |
| `HOF:PY-FOLLOW` | Follow-ups the `python/nv-py` platform worker asked for elsewhere (recorded in the old `HANDOFF.md`, not re-verified in this consolidation): `cuda-core-ext` `ManagedBuffer::as_slice` / `as_mut_slice` unsafe or synchronising, `zeroed` via `cuMemsetD8`, `HostRegistration::unregister(self) -> Result`; `cupti` `Subscriber::close` / `ActivityProfiler::close` returning `Result`; `python/nv-py` `errors.rs` allowing `General` on Windows so NVAPI errors stop mapping to `NvCudaError`. (The `nccl` `Arc::ptr_eq` part is closed: no `Arc::ptr_eq` remains in `crates/nccl/src`.) | lead | old `HANDOFF.md` (`git show ee08ade:HANDOFF.md`, python platform worker row) |

### Build, release, CI

| ID | Description | Owner | Pointer |
| --- | --- | --- | --- |
| `W5.2` | `xtask` consumes `nv_loader::catalog` and ships `nvngx_dlssd.dll` and companion DLLs (`SYS:X-1..X-3`); `nv-bindgen` writes an expected-files manifest and a prebuilt-fallback marker (today `xtask` infers both); `release.yml` Windows step checks `$LASTEXITCODE`. | lead | `07_system_xtask.md`, `04_loader_bindgen_sys.md` |
| `W5.4` | CI workflows (`a246a6a`) never executed and `cargo deny` never run: needs a green CI run. | lead | `07_system_xtask.md` `SYS:B-3..B-6` |
| `DEV:CI` | CI job that builds `device/`. | lead | `wave_6_device.md` |
| `DEV:LOCK` | `git rm --cached` of `device/.optix_device_tests.oxide-pipeline.lock`: `git ls-files device` at `ee08ade` lists no such file (`.gitignore` entry added in `c04d692`), so this looks closed; confirm once and drop. | lead | `wave_6_device.md` D8 |
| `DEV:FORK` | Refresh the `cuda-rust_b` fork when the root workspace needs an NVIDIA-only `cuda-core` API. | lead | ADR-12 |

### Single source of truth

| ID | Description | Owner | Pointer |
| --- | --- | --- | --- |
| `W4.3` | Remaining duplication: `OPX:S1..S4` (five `optixAccelBuild` copies, three primitive-type mappings), `SOL:S5`, `SOL:N5`, `BLA:LT-09`, `MED:X-2` (NV12/P016 plane math, `detail_suffix`, PSNR, 2D copies), `IOP:NW-20`, `IOP:CE-09`; `nv-demo` duplicates (FOV / AOV labels / DLSS quality table / scene rules `APP:F24`; the animation-range part was closed by `anim_range` sliders in `ee08ade`; SDK discovery is `GPU:D07`). | lead | `plan2_full.md` "Single source of truth" |
| `APP:F21` | `persist.rs:313-374` and `ui.rs:145-164` keep hand-written string tables although `serde` derives are available; left to keep the on-disk config format unchanged. | lead | `11_demo_main_ui.md` |

### `nv-demo`

| ID | Description | Owner | Pointer |
| --- | --- | --- | --- |
| `GPU:D07` / `GPU:D08` | Hash `optix.h` in the PTX cache key; share `build.rs` OptiX SDK discovery with the renderer (picks the first folder today). Not part of the `ee08ade` wave. | lead | `10_demo_gpu.md`; `wave_nvdemo_followups.md` |
| `APP:F27` | Unbounded waits: `device.poll(wait_indefinitely)` in `main.rs` and `platform::wait_for_submission` in `nv-wgpu` Drop paths (`VULKAN_SUBMIT_TIMEOUT` = 5 s covers only the submit). | lead | `11_demo_main_ui.md`; `wave_nvdemo_followups.md` |
| `RND:H12` | README and docs site still describe CSG host staging as 256 KiB (it is 2 MiB growing to 16 MiB). | lead | `09_demo_renderer_host.md`; `wave_nvdemo_followups.md` |
| `GPU:D04` | Cryptomatte id per pixel on road tiles needs the `csg-rs` `part_id` road-UV packing decision (recorded for that repository). | csg-rs | `10_demo_gpu.md` `E01`, `E02` |
| `CSG:RPN-FLAG` | `csg-rs` RPN stack overflow flag (recorded for that repository with the road-tile UV note). | csg-rs | `10_demo_gpu.md` `E01`, `E02` |
| `CSG:R00` | GPU crowd steering moves out of `nv-demo` `cuda_crowd.rs` into a csg-rs-owned CUDA kernel. | csg-rs | chat `Mullis-16-3934` |
| `CSG:R00b` | GPU traffic. | csg-rs | chat `Mullis-16-3934` |

### Python and docs

| ID | Description | Owner | Pointer |
| --- | --- | --- | --- |
| `DPY:C20` | `close()` / `__exit__` for the eight classes. | lead | `12_device_python.md` |
| `DPY:DLPACK-WAIT` | Precise `__dlpack__` wait needs a `cuda-core` API exposing pending write events (today a context-wide synchronize). | lead (blocked on external `cuda-core`) | `gate_python_w6.md` |
| `W7.1` | Documentation: `crates/nv-demo/AGENTS.md` (owned by the other agent right now), `optix` module docs (`optix.rs:19-20`, `accel.rs:25-26`, `OPX:DOC1`, `OPX:A7`), add the systemic defects to `## Traps`; every cited constant re-read in source. Links to the moved plan files in `CHANGELOG.md` and `crates/*/AGENTS.md` (section 7 of the consolidation report). | lead | `plan2_full.md` Documentation drift |

### Old backlog (from the former `PLAN.md`, not re-verified)

| ID | Description | Owner | Pointer |
| --- | --- | --- | --- |
| `BK:SANITIZER` | P2: `cargo test --workspace -- --ignored` plus `compute-sanitizer` on live GPU paths (`cufft` `ManyDesc` / `Scratch` growth, `cudnn`, `cufile`, `optix`). The probabilistic scratch-race tests in `cufft` and `cusparse` have not run under the sanitizer. | lead | `audit/history/PLAN_milestones_M1-M6.md` |
| `BK:SPAN-TRAIT` | P3: consolidate the sealed `DeviceSpan` / `DeviceSpanMut` contracts of `cudnn`, `nccl`, `cufile` into `cuda-core-ext`. | lead | same |
| `BK:PY-DTYPE` | P3: reuse `dlpack::find_dtype_entry` in `python/nv-py/src/core.rs` dtype inference. | lead | same |
| `BK:PROFILE` | P3: profile `nv-demo` frame time and wrapper hot paths (`samply` / `tracing-chrome` / `criterion`, A/B release). | lead | same |

Item count: 29 rows: 8 verification/review, 5 build/release/CI, 2 SSOT, 7 `nv-demo` (3 lead, 4 csg-rs), 3 python/docs, 4 old backlog. Closed since the old trackers and therefore absent here: `W4.1`, `W1.1..W1.6`, `W2.1..W2.3`, `W3.1..W3.6`, `W4.2`, `W5.1`, `W5.3`, `W6.1`, `W6.2`, GPU:D03, GPU:D21, RND:R10, RND:H13, APP:F19, `anim_range` sliders, async params upload, `nv-wgpu` submit timeout.

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
