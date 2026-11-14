# Handoff — nv-rs (2026-11-13)

## Wave 1 Completion Checkpoint — 2026-11-13 / 2026-11-14 (GPU Verified)

- **GPU T2/T3 Verification on RTX 3080 Ti (`2026-11-14`)**: All Windows-runnable GPU integration tests passed after fixing real hardware/driver/SDK findings (`cudnn-sys` Windows cuDNN 9.23 sibling DLL `LOAD_WITH_ALTERED_SEARCH_PATH` preloading, `cusolver::SpHandle::csreigvsi` device `d_mu` pointer, `nvjpeg` v2 `JpegDecoder` + `JpegState::with_decoder`, `npp` `CmpOp` threshold mode validation + BT.601 rounding tolerance, `nvenc`/`nvdec` 256x256 min frame dims + AV1 cap check, `nvofa` non-periodic pattern, `nvrtc` `.entry` assertion, `nvml` WDDM `reserved` memory invariant, `nvcomp` 512 MiB test budget):
  - `nv-wgpu --test gpu`: 5 passed; `nv-bindgen --lib -- --ignored`: 1 passed.
  - `cublas --lib`: 2 passed; `cufft --lib`: 1 passed; `curand --lib`: 1 passed.
  - `cusparse --test gpu`: 4 passed; `cusolver --test gpu`: 6 passed.
  - `npp --test gpu`: 2 passed; `nvjpeg --test gpu`: 1 passed; `nvrtc --test gpu`: 4 passed; `cupti --test gpu`: 2 passed.
  - `nvml --lib`: 1 passed; `nvapi --lib`: 1 passed.
  - `nvenc --test gpu`: 1 passed; `nvdec --test gpu`: 1 passed; `nvofa --test gpu`: 1 passed.
  - `cudnn-sys --lib`: 1 passed (`library_loads`); `cudnn --lib`: 2 passed; `nvcomp --lib`: 1 passed.
  - `nccl` / `cufile` remain Linux-only (`x86_64-unknown-linux-gnu` target not installed on this Windows host); `ngx` held per `PLAN.md` D13.

- **vcv-rs**: Completed Windows/Linux fixes, verified (`clippy -D warnings`, 117 tests + `pwsh`/`git-bash` roundtrips + mutant), and pushed to `main` at `31c54282807fe2b0705ffde543fc8dfef4c2b543`. `nv-rs/Cargo.lock` and `nv-rs/device/Cargo.lock` lock `31c54282`.
- **M2 Audit & Repair (`D01_D02`)**: Audited `nvcomp-sys`, `nccl-sys`, `cufile-sys`, `nv-bindgen`, `nv-loader`, `nv-bindgen-fixture` against installed SDK headers; fixed `nvcomp-sys` `positive_enums` + `libnvcomp.so.6`, `nv-bindgen` `WrapMode::CheckedVoid` (`clippy::unit_arg`-clean) + `QueryInterface` `rerun-if-changed` + GCC `c_uint`, and `nv-loader` `Error::source()` unwrapping across `thiserror` 2.0.
- **M6 (`D03`: `crates/nv-wgpu`)**: Implemented `wgpu 30` <-> CUDA interop (Vulkan Win `OPAQUE_WIN32` / Linux `OPAQUE_FD` + DX12 shared heaps/fences, `gpu-info-rs` `SharedGpu` composition, `SharedBuffer`, `SharedTexture`, `InteropSemaphore`, format table, `VkBarrierRunner` drop/sync hardening).
- **M7–M9 Safe Wrappers (`D04`–`D09`)**: Implemented `cublas` (+ `cublasLt`), `cufft`, `curand`, `cusparse`, `cusolver` (`Dn`/`Sp`/`Mg`), `npp`, `nvjpeg`, `nvrtc`, `cupti`, `nvml`, `nvapi`, `nvenc`, `nvdec`, `nvofa`, `cudnn`, `nvcomp`, `nccl` (Linux, `Unsupported` on Win), and `cufile` (Linux, explicit `PosixFallback`). `ngx` remains held per `PLAN.md` D13.
- **Verification**:
  - Independent Inquisitor static audits (7 Mandatory Questions): **PASS** across all 8 domains (`D01_D02`–`D09`).
  - `cargo +1.98.0 clippy --all-targets -j 2 -- -D warnings`: **PASS** (0 warnings, 0 errors) across all 22 target crates (`nv-bindgen`, `nv-loader`, `nv-bindgen-fixture`, `nv-wgpu`, `cublas`, `cufft`, `curand`, `cusparse`, `cusolver`, `npp`, `nvjpeg`, `nvrtc`, `cupti`, `nvml`, `nvapi`, `nvenc`, `nvdec`, `nvofa`, `cudnn`, `nvcomp`, `nccl`, `cufile`).
  - `cargo +1.98.0 test --lib -j 2 --quiet`: **PASS** (0 failures) across all 22 target crates (GPU T2/T3 tests `#[ignore]` pending serialized GPU run).
- **Next**: Run `#[ignore]` T2/T3 GPU integration tests (`cargo +1.98.0 test -- --ignored`) in serialized `-j 2` batches on RTX 3080 Ti, run Linux-target checks for `nccl`/`cufile`, and commit/push `nv-rs` with DCO (`git commit -s`).

---

## Transfer checkpoint — 2026-11-13 (Codex)

**Status: paused by owner for an environment change.** All task agents have stopped/completed; do not resume implementation automatically. The owner explicitly requested updating progress and pushing the unfinished source checkpoint to main.

- vcv-rs checkpoint is published on main at `a8cab6688c1ef01f1067ea3d150ad7283e1367cb`.
- Both host and device Cargo.lock files now resolve that exact vcv-rs revision. Metadata-only commands succeeded: `cargo +1.98.0 update -p vcv-rs` and `cargo +1.99.1 update --manifest-path device/Cargo.toml -p vcv-rs`. They added object 0.40.0, removed unused optional CLI dependencies, and recorded the three written host sys crates; no compile/test result follows.
- nv-rs D01/D02 source remains **unreviewed and untested**. M2 is open. Historical M1/M3/M4/M5 and prior 18-library results below do not validate this checkpoint.
- vcv-rs D11 reached a 225-ID source-only static closure. Added ID226 (real COFF/ELF archive test) has not been formatted after insertion, independently reviewed or executed.
- vcv-rs D12 has 125 IDs; the full review is incomplete. **Known defect remains:** fmt_ps accepts oversized Windows/.NET environment keys/values/joined lists and may fail after earlier assignments. D13's 110-ID independent review remains pending.
- No Windows/Linux build, check, test, shell round trip, seeded mutant or GPU gate ran for this wave. Platform gates are 0/2.
- Durable transfer records, exact continuation, source hashes and unexecuted validation scripts are in sibling [vcv-rs HANDOFF.md](../vcv-rs/HANDOFF.md) and its `docs/mastermind/evidence/2026-11-13-transfer/`. SDK binaries and proprietary headers were excluded.
- Recovery documentation-agent OAuth failure produced no result; root wrote this checkpoint. No new agent or chat-wait loop was started after stop.

After the owner resumes: repair/review the vcv-rs length boundary, review ID226 and complete D12/D13/seam reviews; commit/push any new edits before every build/check/test. Root monitors CPU/RAM/GPU/other builds and serializes both platform gates under the existing build-lock override. Then verify nv-rs consumers and finish M2 NCCL/nvCOMP/cuFile reviews and supported-platform gates. Preserve separate NCCL multi-GPU and native-Linux cuFile/storage requirements.


## Historical checkpoint (superseded above)


## Recovery checkpoint (2026-11-13, 22:35–22:37 UTC)

The previous handoff was last modified at 19:40:44 UTC. Source edits continued through
21:58:52 UTC and TODO/domain-index updates through 22:24:17 UTC, so the checkpoint below
supersedes its next steps. Historical milestone results are retained separately.

- nv-rs: `main` at `8b7e13ccc90340bf7c1385878fd0b5e82c30afe0`; the snapshot before recovery
  documentation edits had 23 modified tracked files (805 insertions, 265 deletions), untracked
  TODO/domain docs and nine source files in `nvcomp-sys`, `nccl-sys` and `cufile-sys`.
- D01/D02 source and NCCL cleanup/checked-return fixtures are written; fresh review, publication
  and compiler/test/GPU gates remain pending. See [TODO.md](TODO.md) and the
  [domain index](docs/mastermind/MASTERMIND_INDEX.md).
- vcv-rs: `main` at `6eb28c0a4c78a5b34771dbfbb5b14742ecb696ca` with uncommitted changes.
  Its Windows/Linux D11/D12/D13 work is the immediate dependency.
- One nv-rs worktree and no `cargo`/`rustc` processes were observed. No build, check or test
  ran during recovery; inspected logs established no gate evidence for this wave.
- Queried workspace memory had no summary/items; queried OMH modes/recent-files were empty.
  GitNexus recommends `reanalyze_unstaged`: its 20:12:48 UTC index predates source edits.
- vcv-rs advisor artifact `.omh/artifacts/ask/claude-20261113-210117.md` records exit code 1
  from a weekly usage limit, so it supplies no independent review verdict. The older
  `target-run.log` success is historical evidence, not a current M2 gate.

## Historical milestone evidence

- Spikes S1a (OptiX runs a Rust raygen from cuda-oxide), S2 (Vulkan buffer shared by wgpu and
  CUDA with a timeline semaphore), S3 (one generator + one loader for nvml/optix/cufft): all PASS.
  Details and findings in PLAN.md §8; code under `spikes/`.
- **M1 done**: `crates/nv-loader`, `crates/nv-bindgen` (on vcv-rs), `xtask coverage`.
- **M5 done** (whole OptiX 9.1 API, device + host, coverage gate `cargo xtask coverage optix`,
  GPU tests: 55 host + 15 device cases; PLAN.md §8). Device crates live in `device/` (toolchain
  1.99.1, `cargo oxide`); OptiX programs must build with `CUDA_OXIDE_NO_SHORT_PTR=1`
  (`device/optix-device-tests/.cargo/cuda-oxide.toml`).
- **S1b PASS** (Rust raygen calls `optixTrace`) and **M4 done**: `crates/optix` (context, GAS/IAS +
  compaction + byte budget + reusable scratch + refit, modules, program groups, pipeline, typed
  SBT, launch, HDR/AOV denoiser); 11 GPU tests on the S1b PTX. Details in PLAN.md §8.
- **M3 done**: `crates/cuda-ext` (external memory/semaphores, arrays, textures, surfaces, host
  registration, managed memory, UUID/LUID, green contexts); 7 GPU tests incl. a Vulkan round trip.
- **M2 prior baseline** — 18 sys crates built, loaded and passed their tests on this machine (counts in
  COVERAGE.md): nvml, optix, cufft, cublas, cublaslt, cusparse, cusolver, cusolvermg, nvjpeg,
  nvrtc, cupti, nvcuvid, nvenc, npp (11 DLLs), cudnn, nvapi, nvofa, ngx. Generator shapes:
  exports, split exports (NPP), query interface (NVAPI), function table (OptiX, NVENC, NVOFA),
  static link (NGX, PLAN D15). `nv-bindgen-fixture` locks generator bug classes.
- SDK discovery: `vcv_rs::nvsdk` (vcv-rs) finds cuDNN, Video Codec, Optical Flow, NVAPI, NGX and
  nvCOMP (PLAN D16); no per-SDK environment variables are needed.

## Next

1. Complete [vcv-rs Windows/Linux support](../vcv-rs/TODO.md). Repair/re-review the four open
   D11 findings reported by the work queue: OptiX fixture ownership, NGX runtime names/package
   paths, nested Windows NGX import libraries and Windows CUDA .LIB case handling. Obtain fresh
   D11/D12/D13 reviews.
2. Finish coherent source edits, then commit and push every involved repository to `main`
   with DCO sign-off before any build, check or test. Consume the pushed vcv-rs revision.
3. Root monitors CPU/RAM/GPU load and other builds, then runs serialized platform, fixture,
   compiler/test and mutant gates. The owner's build-lock override remains in force; no
   replacement lock is required. Refresh the source graph and record exact evidence.
4. Complete M2 **nvCOMP**, **NCCL** and **cuFile** reviews, coverage and supported-platform
   gates. Keep NCCL multi-GPU and native-Linux cuFile/storage validation separate.
5. Continue M6 `nv-wgpu`, then M10 render-rs `pt-optix` as planned. The historical fork revision
   is `42cc04eda` (nvidia/main `7a7d160ea`); D5 authorizes fork patches, not an upstream PR.

## Building

- SDKs unpacked outside the standard folders: set `NV_SDK_DIRS` (here
  `C:\projects\projects.rust.cg\nv`, holding OptiX, Video Codec, NVAPI, Streamline/NGX).
- `ngx-sys` needs `VULKAN_SDK`.
- Commit and push every involved repository to `main` before any build, check or test; one
  build at a time after root checks machine load and other builds. The prior full workspace
  test run took over 10 minutes here — run long gates in the background.

## Open items

- reconcile the prior pending `#[device(export_name)]` item with the M5 completion claim and actual fork API; recovery did not resolve this evidence conflict;
- reconcile D6 with the fork's committed `cuda-bindings/generated/`;
- types shared between toolkit libraries (cuSOLVER pulls its own copy of cuBLAS/cuSPARSE handle
  types) should be re-used across sys crates, like CUDA types are from `cuda-bindings`;
- NVENC GUIDs are `static const` in the header and are not emitted by bindgen;
- NVOFA D3D11/D3D12/Vulkan variants and NGX's D3D11 helpers are not bound yet.

## Environment

- GPU: RTX 3080 Ti (sm_86), driver 621.24; CUDA 13.3; cuDNN 9.23 (CUDA 13.3 build). Host
  toolchain 1.98.0; spikes/device 1.99.1.

## Traps

- OptiX descriptors hold raw `*const c_char` entry names: the `CString`s must outlive the create
  call. `ProgramGroup::hit_group` once moved them into a list before the call (dangling names,
  "entryFunctionNameCH does not start with __closesthit__"); the GPU tests lock this.
- cuda-oxide device code: `ptx_asm!` `out` operands need declared variables (`_` is not a place).
- `cargo oxide` copies its backend into `~/.cargo/cuda-oxide`, which other sessions' builds keep
  open; set `CUDA_OXIDE_BACKEND` to a private copy instead of waiting or killing their processes.
- OptiX device code: index arrays with constants where possible; build OptiX programs with
  `CUDA_OXIDE_NO_SHORT_PTR=1` (the `ptr64` case guards it). Each test case compiles only its own
  programs out of the binary's PTX (`cases/common.rs` `subset_ptx`): OptiX validates every
  function of a module against that module's options.
- Device tests run with a private backend copy (`CUDA_OXIDE_BACKEND`), never the shared
  `~/.cargo/cuda-oxide`.
