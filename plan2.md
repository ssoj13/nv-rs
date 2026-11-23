# Workspace-wide Bug Hunt & Remediation Plan (`plan2.md`)

**Audit date**: 2026-11-23
**Baseline**: `nv-rs` `b235d78` (`main`), `csg-rs` `f7c8474a` (from `Cargo.lock`)
**Scope**: the whole repository: all `crates/*`, `xtask`, `bootstrap.py`, CI, and the excluded `device/` and `python/` workspaces. `plan1.md` (fully closed) covered `crates/nv-demo` only.
**Status**: findings only. Nothing in the source tree was changed. Remediation waits for operator approval.

## How this audit was done

- 12 independent read-only auditors, one per disjoint slice, each reading every file of its slice in full. Their complete ledgers (every item with `file:line`, failing scenario and fix direction) are stored in `audit/2026-11-23/`:

| Prefix | Slice | Report | P0 | P1 |
| --- | --- | --- | --- | --- |
| `OPX` | `crates/optix` | `01_optix.md` | 2 | 9 |
| `BLA` | `cublas`, `cufft`, `curand` | `02_cublas_cufft_curand.md` | 12 | 6 |
| `SOL` | `cusolver`, `cusparse`, `cudnn` | `03_cusolver_cusparse_cudnn.md` | 5 | 7 |
| `LDR` | `nv-loader`, `nv-bindgen`, all `*-sys` | `04_loader_bindgen_sys.md` | 0 | 6 |
| `MED` | `nvenc`, `nvdec`, `nvjpeg`, `nvofa`, `npp`, `nvcomp`, `cufile` | `05_media_io.md` | 7 | 19 |
| `IOP` | `cuda-core-ext`, `nv-wgpu` | `06_coreext_wgpu.md` | 7 | 12 |
| `SYS` | `ngx`, `nvapi`, `nvml`, `cupti`, `nvrtc`, `nccl`, `xtask`, `bootstrap.py`, CI | `07_system_xtask.md` | 1 | 20 |
| `CAM` | `nv-demo` camera, rail, anim, MP4 muxer | `08_demo_camera_misc.md` | 0 | 3 |
| `RND` | `nv-demo` `renderer.rs`, `csg_scene_host.rs` | `09_demo_renderer_host.md` | 2 | 8 |
| `GPU` | `nv-demo` shaders, crowd, bloom, export, profiler | `10_demo_gpu.md` | 1 | 6 |
| `APP` | `nv-demo` `main.rs`, `ui.rs`, `persist.rs`, buses | `11_demo_main_ui.md` | 1 | 13 |
| `DPY` | `device/`, `python/nv-py` | `12_device_python.md` | 16 | 25 |

  IDs below are `<prefix>:<id in that report>`. Counts are the auditors' own ratings.
- **Lead spot-check**: 10 of the highest-rated items were re-read in source by the lead. All 10 match the code; one is downgraded (`MED:DEC-1`, formal aliasing UB with low practical risk, P0 -> P1). They are marked **[verified]** below. Every other item is as reported by its auditor and must be confirmed with a failing test before it is fixed.
- **Not run**: `cargo test --workspace` (stopped at operator request), ignored GPU tests, anything on Linux. Items marked PLAUSIBLE in the reports depend on native-library or hardware behaviour.

## Warnings audit

> **Update 2026-11-23**: resolved by W4.1. `nv-demo` now depends on `camera-rs` `9109108`; the vendored copy and the blanket allow are gone and `cargo clippy -p nv-demo --all-targets` is clean without them. The text below describes the state at audit time.

- `cargo clippy --workspace --all-targets`: **0 warnings**.
- That result is produced by a blanket `#[allow(unexpected_cfgs, dead_code, unused_imports, clippy::all)]` on `mod camera_core` and `mod camera_egui` (`crates/nv-demo/src/main.rs:7,9`). Forcing the lints back on shows **133 distinct warnings** in `nv-demo`.
- Root cause: `camera_core/**` and `camera_egui.rs` are a vendored, diverged copy of `cglibs/camera-rs` (`crates/core`, `crates/egui`, upstream `ab89440`). 29 of 33 files differ from upstream.
  - `nv-demo` declares no `serde` feature, so all 36 `cfg(feature = "serde")` sites are dead. Five serde tests never compile (`mode.rs:405`, `prefs.rs:589,619`, `settings.rs:315,338`), and `persist.rs:313-374` / `ui.rs:145-164` carry hand-written string tables instead.
  - Upstream fixes are missing here: the `MAX_DT` clamp (`CAM:D03`), coast speed clamp (`CAM:D04`), look-inertia removal (`CAM:D20`).
  - Unused remnants, not unfinished features: `auto_orbit.rs`, the lens branch in `driver.rs:377-390,484-518`, `lens.rs:128-206`, `view.rs:306-343`, `viewport.rs:152-208`, `set_raw_motion_units` (`camera_egui.rs:227`).
- Other suppressions worth fixing: `cupti.rs:21` crate-wide allow, `optix.rs:25` Linux-wide cast allow, `cudnn_sys.rs:47-57` and `nv_bindgen_fixture.rs:156,247` stale `undocumented_unsafe_blocks` allows, `device/optix-device/src/optix_device.rs:55,65`, `python/nv-py` `errors.rs:230` and `dlpack.rs:14`. The `too_many_arguments` expects that mirror a C API are justified.
- `TODO` / `FIXME` / `todo!()`: none in source. `unimplemented!()` at `xtask/src/py_stubs.rs:2724` is text inside a test fixture; the other four hits are doc examples.

## Systemic defects (each found independently by several auditors)

1. **Async GPU work does not keep its buffers alive.** Reported by `BLA:SEAM-01`, `SOL:X1`, `MED:X-1`, `DPY:X1`/`G1`/`PE-02`. Lead verification against `cuda-core` `348895e` (the locked revision):
   - **Verified in source**: `Allocation::drop` (`cuda-core/src/allocation.rs:94-126`) retires the free on a worker without waiting: `cuMemFreeAsync` on the *allocation* stream for stream-ordered buffers, `cuMemFree` for synchronous ones. Its comment states the contract: "All safe users hold an independent access lease until completion", and the `DeviceBuffer` docs add "Unsafe raw-pointer users must retain storage through their own work" (`device_buffer.rs:147-153`).
   - **Verified in source**: across this repository the lease API (`storage().acquire(..)` + `retain_until_complete(..)`) is called in three files only: `crates/cupti/src/activity.rs`, `crates/cuda-core-ext/tests/interop.rs`, `device/optix-device-tests/src/cases/texture.rs`. No other wrapper takes a lease, so they do not meet that contract.
   - **Measured on GPU (2026-11-23, RTX 3080 Ti, `crates/curand/tests/lease.rs`, 20 repetitions per case, buffer dropped right after `Generator::generate_u32` queued behind a busy stream)**:

     | Buffer | Address reused by next allocation | Next allocation corrupted |
     | --- | --- | --- |
     | stream-ordered (`DeviceBuffer::uninitialized_async`), allocated on another stream than the work | 20/20 | **20/20** (`compute-sanitizer --tool memcheck`: 20 use-after-free errors) |
     | stream-ordered, allocated on the same stream as the work | 0/20 | 0/20 |
     | synchronous (`DeviceBuffer::zeroed`, `from_host`) | 16/20 | 0/20 |

   - **Consequence: the auditors' rating was too broad.** The defect is real only for stream-ordered buffers used across streams. `uninitialized_async` is the only stream-ordered constructor and it is `unsafe`; the safe constructors are synchronous and did not corrupt. The synchronous case is protected by `cuMemFree` waiting for device work, which is inferred from timing (the next allocation never completed while the stream was busy) and is not a documented guarantee. The `P0` ratings of `BLA:SEAM-01`, `SOL:X1`, `MED:X-1` therefore do not hold for callers that use only safe constructors.
   - **Where stream-ordered buffers exist in this repository** (`uninitialized_async` call sites, verified by grep): scratch buffers in `cublas`, `cufft`, `cudnn`, `cusolver`, `cusparse`, `nvcomp`; `optix` (`accel.rs`, `denoise.rs`, `cluster.rs`); `nv-demo` `fft_bloom.rs`; `python/nv-py` (`core.rs`, `video.rs`, `nvjpeg.rs`, `platform_ext.rs`). Not yet analysed per site: whether each is ever used on a different stream than its own. `python/nv-py` `DeviceBuffer.alloc` allocates user buffers this way on the caller's stream, so the corrupting case is probably reachable from Python with two streams and `del buf`; not tested.
   - **Fix prototype (uncommitted)**: `cuda_core_ext::lease::enqueue_leased` (acquire -> bind -> enqueue -> retain) applied to `curand::Generator::generate_u32` only. After it: 0/20 corruption in all three cases; cost about +2 us per call (median 16.3 -> 18.0 us for a 4-element call); new behaviour: `CUDA_ERROR_NOT_READY` when another stream still has unfinished work on the buffer, and a context check. `memcheck` still prints 3-4 "potentially accessed after free" stream-race warnings in some runs, with no corruption; a sanitizer-clean result would need `cuda-core` to order the free after the work with a stream wait.
2. **Context binding is inconsistent.** `cublas::Handle::set_stream` returns before `bind_to_thread` on a stream-cache hit (`cublas/src/handle.rs:313-315`) **[verified]**; `cusolver` Dn/Sp and `cusparse` handles are `Send` but never bind (`SOL:X2`); many `Drop` impls release CUDA resources unbound (`MED:ENC-6`, `MED:JPG-5`, `BLA:RNG-01`, `OPX:X2`).
3. **Unsound `Send`/`Sync`.** `cufft::Plan` is auto-`Sync` although its own comment says it must not be (`cufft/src/plan.rs:362-387`) **[verified]**; `ngx` `VulkanSession` is `Send + Sync` over a library documented as not thread-safe (`ngx/src/vulkan.rs:273-303`); `nccl` `GroupGuard` is auto-`Send` (`nccl/src/comm.rs:89-115`); also `MED:DEC-5`, `MED:OFA-1`, `MED:JPG-4`.
4. **Workspace memory handed to a native library without ownership.** `cublas::Handle::set_workspace` passes a raw `Scratch` pointer the handle does not retain (`cublas/src/handle.rs:397-405`) **[verified]**; `cufft` releases the scratch lock before exec (`plan.rs:663-700`); `cusparse` SpSV/SpSM analysis buffers come from caller scratch that may be reused before solve (`ops.rs:686-878`); `nvcomp` receipts are not tied to a staging generation (`batched.rs:37-118`).
5. **Teardown errors are replaced by an invented code.** About 20 sites map a real native status to `CUDA_ERROR_UNKNOWN` and park it in a sticky slot that surfaces in the next unrelated call (`BLA:ERR-01`, `SOL:X4`, `MED:ENC-5`, `SYS:CUPTI-5`, `SYS:NCCL-5`, `OPX:X1`).
6. **Silent fallbacks.** Prebuilt bindings replace a failed generation (`LDR:L-01..L-03`); denoiser Temporal -> Aov (`RND:R1`); pinned -> pageable staging (`OPX:A9`); bloom and AOV buffers via `.ok()` (`RND:R6`, `RND:R7`); NVENC 10-bit/444 downgrade in Python (`DPY:V7`); corrupt config -> defaults (`APP:F07`).
7. **Render-thread panics are invisible.** 23 `expect`/`assert` sites in `renderer.rs` (`RND:R5`) and no `is_finished()` check in the UI (`APP:F12`): a driver error looks like a frozen viewport.

## P0 ledger

### Wrapper crates

- `OPX:A1` **[verified]** `optix/src/accel.rs:1074-1094`: `InstanceBuffer` rewrites its single pinned staging buffer without waiting for the previous async copy out of it. Two uploads before the stream drains give an IAS built from torn instance data. `nv-demo` is protected only by its per-frame `stream.synchronize()` (`renderer.rs:5112`).
- `OPX:D1` `optix/src/denoise.rs:967`: in-place denoise is allowed with tiling; later tiles read overlap already overwritten.
- `BLA:BL-01` **[verified]** `cublas/src/handle.rs:397-405`: workspace use-after-free (systemic 4).
- `BLA:BL-04` `cublas/src/gemm.rs:83-103`, `lt.rs:1621-1635`: output batch stride smaller than one matrix (including 0) is accepted.
- `BLA:BL-05` `cublas/src/gemm.rs:705-770`: scalars typed `&[f32]` while `compute_type` may be F64.
- `BLA:LT-01..LT-04` `cublas/src/lt.rs:1173, 1544, 1665, 2022, 2189`: safe raw-attribute setters bypass the cache that `matmul_f32` / `matrix_transform_f32` validate against; `set_batch_mode(PointerArray)` makes the library read an f32 buffer as pointers.
- `BLA:FFT-01` **[verified]** `cufft/src/plan.rs:362-387`: `Plan` is `Sync`.
- `BLA:FFT-02` `cufft/src/plan.rs:663-700`: scratch lock dropped before exec.
- `BLA:FFT-03`, `BLA:FFT-04` `cufft/src/plan.rs:217-282`: one-sided `inembed`/`onembed` validated against the smaller layout; `odist`/`ostride` overlap not rejected.
- `SOL:P1` `cusparse/src/ops.rs:214-878`: scalar type not checked against `compute_type` (8-byte read from `&f32`).
- `SOL:P2`, `SOL:P3` `cusparse/src/ops.rs:484-564, 686-878`: SpSV/SpSM and SpGEMM external buffers taken from reusable scratch.
- `SOL:P4` `cusparse/src/descr.rs:656-702`: `set_csr_pointers` does not check `nnz` against the native size.
- `MED:JPG-1` `nvjpeg/src/handle.rs:442,473`: decode does not check the output buffer against real JPEG dimensions and subsampling.
- `MED:JPG-2` **[verified]** `nvjpeg/src/encode.rs:207`: `encode_yuv` validates planes without the requested subsampling.
- `MED:JPG-3` `nvjpeg/src/handle.rs:525-540`: input lifetime not tied to the state when `save_stream = false`.
- `MED:CMP-1` `nvcomp/src/batched.rs:37-118`: stale receipt reads another batch's sizes.
- `MED:CMP-4` `nvcomp/src/bitcomp.rs:212-244`: decompression trusts the stream header.
- `IOP:NW-01` **[verified]** `nv-wgpu/src/sync.rs:72-86`: `vkDestroySemaphore` runs in `Drop::drop`, before the `cuda` field drops, with no wait for pending work; the SAFETY comment is wrong.
- `IOP:NW-02` `nv-wgpu/src/nv_wgpu.rs:228`, `platform.rs:434-446`: safe `with_vulkan_barrier_queue` calls `vkGetDeviceQueue` with arbitrary indices.
- `IOP:NW-04` `nv_wgpu.rs:98-105`, `buffer.rs:115`, `texture.rs:211`: interop objects do not retain `wgpu::Device`.
- `IOP:NW-06` `nv-wgpu/src/format.rs:469,485`: wrong CUDA array descriptors (2D becomes depth-1 3D; 1D import always fails); a unit test pins this behaviour.
- `IOP:NW-07` `nv-wgpu/src/texture.rs:431,574`: `RENDER_ATTACHMENT` textures mapped without `CUDA_ARRAY3D_COLOR_ATTACHMENT`.
- `IOP:CE-01` `cuda-core-ext/src/array.rs:334-342`: `MipmappedArray::level` halves the cubemap face count on lower mips.
- `SYS:NGX-1` `ngx/src/vulkan.rs:273-303`: `Send + Sync` over non-thread-safe NGX.

### `nv-demo`

- `RND:H1` **[verified]** `csg_scene_host.rs:547-561`: `GrowDeviceBuffer::upload` sets `len_elems` before the allocation; on failure the shader reads past the old buffer.
- `RND:R3` `renderer.rs:4243-4257, 4324-4333`: a failed CUDA crowd step leaves the IAS stale while instance buffers are already new.
- `GPU:D01` **[verified]** `shaders/raytracer.cu:2114`: depth is planar and clamped at 0, so Equirect360 / SlitCam export `depth.Z = 0` for the rear hemisphere. `Range` is computed but not exported.
- `APP:F01` `main.rs:3561-3593`, `renderer.rs:4926-4933`: Viewport PNG under DLSS-RR saves a stale or zero buffer and reports "Saved".

### `device/` and `python/`

- `DPY:P1` `python/nv-py/src/core.rs:641-707`, `buf_util.rs:274`: DLPack import ignores strides and dtype; a zero-stride tensor used as an output writes out of bounds. About 50 call sites.
- `DPY:X1`, `DPY:X2`, `DPY:X3`, `DPY:G1`, `DPY:V4`, `DPY:V5`, `DPY:PE-02`: systemic 1 reachable from Python; dtype never checked; producer stream never awaited.
- `DPY:V1` `video.rs:414,432`: NVENC keeps a raw stream handle without holding `Arc<Stream>`.
- `DPY:V2`, `DPY:V3` `video.rs:420-553, 1120`: B-frames/lookahead not disabled with per-frame unregister; NVOFA UV offset in bytes instead of rows.
- `DPY:C1` `compute.rs:184`: inherits the `cublas` bind bug.
- `DPY:C4`, `DPY:C5` `nvjpeg.rs:257-309`: one `JpegState` across streams; decode result returned with `stream = None`.
- `DPY:PE-01` `platform_ext.rs:94-128`: temp-buffer copy and free are queued before the NCCL collective deferred to `group_end`.
- `DPY:SE-01`, `DPY:SE-02` `sys_ext.rs:499-528, 640`: managed memory touched from host unsynchronised; `HostRegistration.close()` during a running kernel.

## P1 highlights (full lists in the reports)

- **Build / release**: prebuilt fallback hides generator bugs and ABI drift, and is sticky (`LDR:L-01..L-03`, `nv_bindgen.rs:366-384, 1479-1501`); NCCL device headers downgraded on a substring match (`LDR:L-04`); no runtime-vs-header version check (`LDR:L-06`); `stage-dlls` exits 0 with missing libs and omits `nvngx_dlssd.dll` (`SYS:X-1`, `SYS:X-2`); prebuilt directory picked by mtime (`SYS:X-5`); non-atomic artifact writes (`SYS:X-6`); `bootstrap.py` non-atomic unpack and unpinned downloads (`SYS:B-1`, `SYS:B-2`); CI lints and tests only `nv-loader`, `nv-bindgen`, `xtask`, and `deny.toml` is never run (`SYS:B-3`, `SYS:B-4`).
- **Codecs**: `NV_ENC_ERR_NEED_MORE_INPUT` returned as error, B-frames unusable (`MED:ENC-1`); NVDEC `display_area` unset, 1088 scaled to 1080 (`MED:DEC-2`); NVDEC parser user-data aliasing (`MED:DEC-1`, verified, downgraded); nvCOMP manager decompresses with its own options (`MED:CMP-3`); cuFile `DeviceSpanMut` for shared borrows (`MED:CUF-1`).
- **NGX / CUPTI / NCCL**: success tested as `== 1` instead of the SDK mask (`SYS:NGX-4`, `ngx/src/error.rs:34`, **[verified]**); CUPTI process-global state shared between profilers (`SYS:CUPTI-1`, `SYS:CUPTI-2`); `GroupGuard` `Send` (`SYS:NCCL-1`).
- **Interop**: direct `vkQueueSubmit` unsynchronised with wgpu (`IOP:NW-03`); global mutex held across closure and `vkWaitForFences(u64::MAX)` every DLSS frame (`IOP:NW-11`); barrier covers mip 0 / layer 0 only (`IOP:NW-09`); CUDA errors swallowed into `NoMatchingCudaDevice` (`IOP:NW-12`).
- **`nv-demo` renderer**: Traffic toggle ignored while the crowd moves (`RND:R4`); SDG Burst never completes under DLSS / auto-rotate (`RND:R2`); denoiser recreated every frame after Temporal fallback (`RND:R1`); `rebuild_scene` commits its key before fallible work (`RND:H2`); LOD history mismatch defeats the `TrafficOnly` fast path (`RND:H3`).
- **`nv-demo` app**: timestamp-per-second file names overwrite MP4/PNG/SDG output (`APP:F03`); corrupt config overwritten with defaults (`APP:F07`, `persist.rs:181-186`, **[verified]**); surface-lost early return drops the frame (`APP:F02`); Live RT video runs fast (`APP:F04`); time-of-day transition overrides user edits (`APP:F05`); EV range mismatch (`APP:F06`); DLSS render size aspect drift (`APP:F09`); captures differ from the screen (`APP:F10`); export pool never evicts (`APP:F11`).
- **`nv-demo` GPU / SDG**: motion vectors only for Pinhole (`GPU:D02`); no object motion for classic geometry (`GPU:D03`); Cryptomatte id per pixel on road tiles (`GPU:D04`); crowd scatter with stale topology (`GPU:D05`); OptiX SDK discovery picks the first folder (`GPU:D07`).
- **`nv-demo` misc**: MP4 written in place with no `moov` until `finish()` (`CAM:D01`, `CAM:D02`); unbounded `dt` (`CAM:D03`).
- **`device/`**: two different `cuda-core` crates in the test graph, so none of the 15 GPU cases can build (`DPY:D1`); `u16` PTX parameter width (`DPY:D2`).

## Single source of truth / duplication

- Vendored `camera-rs` (see Warnings audit).
- DLL spec catalog in three copies, already diverged: sys crates, `nv-loader/src/diag.rs:394-519`, `xtask/src/stage_dlls.rs:38-183` (`LDR:L-13`, `SYS:X-3`).
- `Scratch` implemented three times with different budget semantics: `cusolver.rs:287-374`, `cusparse.rs:384-456`, `cudnn/scratch.rs`; plus `nvcomp/src/scratch.rs` (`SOL:X3`, `SOL:N1`, `MED:CMP-6`).
- Function-table loaders hand-written three times: `nvenc_sys.rs`, `nvofa_sys.rs`, `optix_sys.rs` (`LDR:L-26`); `cudnn_sys.rs:19-78` copies the `exports_sys!` body (`LDR:L-25`).
- NV12/P016 plane math, `detail_suffix`, PSNR, 2D copies across media crates (`MED:X-2`).
- `optix`: five copies of the `optixAccelBuild` call, three primitive-type mappings (`OPX:S2`, `OPX:S3`).
- `nv-demo`: OptiX SDK discovery in `build.rs:23-40` and `renderer.rs:998-1018`; `SceneState` vs `IasClassicState` (`RND:R21`); camera-repack and freeze conditions duplicated (`RND:H5`, `RND:H6`); FOV range copied 5 times in `camera_rail.rs` with a different value than `lens::FOV_MIN` (`CAM:D09`); 87 animation ranges duplicating `ui.rs` sliders (`CAM:D12`); AOV labels, DLSS quality table, scene rules (`APP:F24`).
- Toolchain and CUDA versions hard-coded in several places with conflicting values (`SYS:B-5`, `SYS:B-6`).

## Documentation drift

- `crates/nv-demo/AGENTS.md`: SBT described as 4/5 (records are 8/9 and 10/11), staging 256 KiB (2 MiB), `csg-rs` `7b8a085` (`f7c8474a`), traps at lines 41 and 43 already fixed, link to `plan1.md` as the active plan (`RND:H12`, `GPU:D31`, `APP:F31`).
- `plan1.md:33` and root `AGENTS.md` claim typed payloads; none exist (`GPU:D31`).
- `optix.rs:19-20` and `accel.rs:25-26` describe stream ordering and sync counts that the code does not implement (`OPX:DOC1`, `OPX:A7`).

## Decisions needed from the operator before implementation

1. **Buffer lease API (systemic 1)**: resolved by reading `cuda-core` `348895e`. The lease API is already public: `DeviceBuffer::storage()`, `RawDeviceBuffer::acquire(stream, write, recording)`, `AllocationLease::retain_until_complete(stream)`, re-exported at `cuda-core/src/lib.rs:12`. No fork or upstream change is needed. The earlier statement that it is `pub(crate)` was wrong: only the internal `completion` module and `DeviceBuffer::enqueue_access` are private. Recommended: one helper in `cuda-core-ext` that mirrors `enqueue_access` (acquire -> bind -> enqueue -> retain) and is used by every wrapper. Note that `acquire` also rejects conflicting cross-stream accesses with `CUDA_ERROR_NOT_READY`, which is a behaviour change for callers that do this today.
2. **`camera-rs`**: decided and done (see W4.1): depend on the upstream crate, local changes ported there as opt-in `NavExtras`.
3. **`device/` test harness** (`DPY:D1`): facts verified by the lead. The fork `ssoj13/cuda-rust_b` (`42cc04e`, `FORK.md`) carries `cuda-oxide` patches that are not upstream and that the device code requires (`ptx_asm!` up to 256 inputs, OptiX callable rooting, 64-bit pointers, named device statics). The 15 GPU cases passed on 2026-11-13 while host crates also used the fork; commit `d6c6795` (2026-11-14) moved host crates to `NVIDIA/cuda-rust`, and `device/Cargo.lock` now holds both sources. `cuda-core` itself differs between `42cc04e` and `348895e` only in `build.rs`, `embedded.rs` and two tests. Recommended: keep NVIDIA upstream for the main workspace, keep the fork for `device/`, and add a `[patch."https://github.com/NVIDIA/cuda-rust.git"]` section in `device/Cargo.toml` that redirects the host crates to the fork inside that workspace only. Not verified: that the device workspace currently fails to build (it was not compiled in this audit).
4. **Priority of `python/nv-py`**: it holds the largest P0 count; most of it disappears once Wave 1 lands in the wrapper crates.

## Remediation plan

Every fix starts with a test that reproduces the defect. Items rated PLAUSIBLE are confirmed (ignored GPU test or `compute-sanitizer --tool memcheck`) before code changes.

### Wave 1 - Systemic memory safety in wrapper crates
- [ ] **W1.1** Buffer leases. Reproduction and helper prototype done (see systemic 1); scope of the rollout is an open operator decision because the measured exposure is narrower than reported: (a) lease every wrapper entry point (uniform, about +2 us per call, new `NOT_READY` errors), (b) lease only where stream-ordered buffers can cross streams (`python/nv-py`, plus any wrapper-internal scratch found to be used off its own stream), or (c) no leases in wrappers, document the contract and fix `nv-py` only | verify: `crates/curand/tests/lease.rs`-style test per leased entry point; `cargo test -p <crate> --quiet`
- [ ] **W1.2** Context rule: every FFI entry and every `Drop` binds its context; fix `cublas::Handle::set_stream` cache hit, `cusolver` Dn/Sp, `cusparse` handle, listed `Drop` impls | verify: cross-thread unit test per handle type
- [ ] **W1.3** `Send`/`Sync` soundness: `cufft::Plan`, `ngx::VulkanSession`, `nccl::GroupGuard`, `nvdec::VideoDecoder`, `nvofa`, `nvjpeg` states | verify: `static_assertions`-style compile tests (`assert_not_impl_any!`)
- [ ] **W1.4** Workspace ownership: `cublas::set_workspace`, `cufft` scratch lock across exec, `cusparse` SpSV/SpSM/SpGEMM buffers, `nvcomp` receipt generation | verify: targeted tests that grow or drop the scratch between phases
- [ ] **W1.5** Validation gaps: `nvjpeg` decode/encode sizes, `cusparse` scalar type and CSR `nnz`, cuBLASLt raw attribute setters, `cufft` embed/stride overlap, GEMM batch stride, `nvcomp` header trust | verify: negative unit tests, no GPU needed where the check is host-side
- [ ] **W1.6** Typed teardown error sink replacing the `CUDA_ERROR_UNKNOWN` mapping; unify the four `Scratch` implementations under one owner | verify: `cargo clippy --workspace --all-targets` + `cargo test --workspace`

### Wave 2 - OptiX and graphics interop
- [ ] **W2.1** `optix`: `InstanceBuffer` staging fence or slot ring (`OPX:A1`), reject in-place tiled denoise (`OPX:D1`), grow-failure state (`OPX:A2`), budget and context checks in `instances_into` (`OPX:A3..A5`), async compaction alloc (`OPX:A7`), report pinned fallback (`OPX:A9`) | verify: new `InstanceBuffer` / tiled-denoise tests in `crates/optix/tests`
- [ ] **W2.2** `nv-wgpu`: semaphore drop order and pending-work wait (`IOP:NW-01`), make `with_vulkan_barrier_queue` unsafe or validated (`IOP:NW-02`), retain `wgpu::Device` (`IOP:NW-04`, `IOP:NW-05`), array descriptors and colour-attachment flag (`IOP:NW-06`, `IOP:NW-07`, needs `IOP:CE-05`), full-subresource barrier (`IOP:NW-09`), queue synchronisation (`IOP:NW-03`, `IOP:NW-11`) | verify: kernel-level GPU tests (`tex2D` / `surf2Dwrite`, mips, cubemap, 1D)
- [ ] **W2.3** `cuda-core-ext`: cubemap mip levels (`IOP:CE-01`), mip count clamp (`IOP:CE-02`), semaphore drop contract (`IOP:CE-03`) | verify: `cargo test -p cuda-core-ext --quiet` + ignored GPU tests

### Wave 3 - `nv-demo` correctness
- [ ] **W3.1** Render-thread failures become `RenderEvent::Fatal` and the UI polls `is_finished()` (`RND:R5`, `APP:F12`); remove `.ok()` on AOV buffers and bloom (`RND:R6`, `RND:R7`) | verify: unit test for the event path
- [ ] **W3.2** `GrowDeviceBuffer::upload` ordering (`RND:H1`), stale IAS after crowd-step failure (`RND:R3`), topology check in `step_gpu_direct` (`GPU:D05`), `rebuild_scene` key commit (`RND:H2`) | verify: `cargo test -p nv-demo --quiet` with injected failures
- [ ] **W3.3** Behaviour bugs: Traffic toggle (`RND:R4`), SDG Burst completion and cancel command (`RND:R2`), denoiser fallback loop (`RND:R1`), sky crossfade parameters (`RND:R10`) | verify: targeted unit tests
- [ ] **W3.4** SDG data: range/depth for non-pinhole sensors (`GPU:D01`), motion vectors for all projections and classic geometry (`GPU:D02`, `GPU:D03`), Cryptomatte ids (`GPU:D04`, needs a `csg-rs` decision on `part_id`) | verify: PTX compile + golden EXR check per sensor projection
- [ ] **W3.5** Capture and persistence: PNG under DLSS-RR (`APP:F01`, `APP:F10`), unique output names (`APP:F03`), atomic MP4 with finalisation on drop (`CAM:D01`, `CAM:D02`), config parse failure keeps the user file (`APP:F07`), scene persisted by descriptor id (`APP:F17`) | verify: unit tests for naming, persistence and muxer finalisation
- [ ] **W3.6** Frame loop and UI: surface-lost path (`APP:F02`), Live RT timing (`APP:F04`), time-of-day override (`APP:F05`), EV range (`APP:F06`), DLSS aspect (`APP:F09`), export pool eviction (`APP:F11`), `dt` clamp (`CAM:D03`) | verify: `cargo test -p nv-demo --quiet`

### Wave 4 - Single source of truth
- [x] **W4.1** `camera-rs` per decision 2; remove the blanket allow at `main.rs:7,9` | verify: `cargo clippy -p nv-demo --all-targets` with no module-level allow
  - Done 2026-11-23. The vendored copy was taken from `camera-rs` `db670f5`; its real local delta (about 240 lines: wheel step/floor, progressive Shift ramp, cursor lock on drags) went upstream as opt-in `NavExtras` with unchanged defaults (`camera-rs` `9109108`, 347 tests, clippy clean), because `camera-rs` has 8 other consumers. `nv-demo` enables them through `CameraDriver::set_nav_extras`; `altitude_speed_multiplier` stayed in `nv-demo` (`ui.rs`).
  - Verified: `cargo clippy -p nv-demo --all-targets` 0 warnings; `cargo test -p nv-demo` 84 passed (328 before, minus 245 tests that belonged to the vendored copy and now run in `camera-rs`, plus 1 moved test).
  - Inherited from upstream, closes `CAM:D03`, `CAM:D04`, `CAM:D20`: `MAX_DT = 0.1` clamp, coast speed clamp, no look-inertia coast after hold-fly. `CAM:D05`, `CAM:D06`, `CAM:D07` are closed by the removal.
  - Not verified (needs a run on the GPU machine): interactive feel. Upstream requests a `Hidden` cursor (not `Locked`) during RMB hold-to-fly, which touches the `CursorGrabController` path in `main.rs`; `NavScheme::Studio` is new and was added to the three scheme name tables (`persist.rs`, `ui.rs`).
  - Still open from `APP:F21`: `persist.rs:313-374` and `ui.rs:145-164` keep hand-written string tables although `serde` derives are now available; left as is to keep the on-disk config format unchanged.
- [ ] **W4.2** One DLL spec catalog consumed by sys crates, `nv-loader::diag` and `xtask stage-dlls` | verify: xtask test comparing the catalogs is deleted because only one remains
- [ ] **W4.3** Remaining duplicates listed above (function-table loader macro, media plane math, `optix` build-input packing, `nv-demo` FOV / animation ranges / AOV tables / SDK discovery) | verify: `cargo test --workspace`

### Wave 5 - Build, release and CI
- [ ] **W5.1** `nv-bindgen`: no prebuilt fallback after a generator panic or when the SDK is present; verify prebuilt header hashes; complete `rerun-if-*` (`LDR:L-01..L-03`, `LDR:L-16`); typed NCCL device-header detection (`LDR:L-04`); runtime version check (`LDR:L-06`, wires the unused `check_version`) | verify: `nv-bindgen-fixture` tests
- [ ] **W5.2** `xtask`: `stage-dlls` fails on missing requested libs and ships `nvngx_dlssd.dll` (`SYS:X-1`, `SYS:X-2`); prebuilt completeness instead of mtime (`SYS:X-5`); atomic writes (`SYS:X-6`) | verify: `cargo test -p xtask --quiet`
- [ ] **W5.3** `bootstrap.py`: atomic unpack, pinned revisions with checksums (`SYS:B-1`, `SYS:B-2`) | verify: interrupted-unpack test
- [ ] **W5.4** CI: clippy and tests for the whole workspace on Windows and Linux, `cargo deny`, one toolchain pin (`SYS:B-3..B-6`); mark GPU tests in `cuda_crowd.rs` `#[ignore]` (`GPU:D26`) | verify: green CI run

### Wave 6 - `device/` and `python/`
- [ ] **W6.1** `device/`: single `cuda-core` source (decision 3) so the 15 GPU cases build; `u16` PTX params (`DPY:D2`) | verify: `device` workspace build + test run on hardware
- [ ] **W6.2** `python/nv-py`: DLPack strides/dtype/contiguity/device checks (`DPY:P1`, `DPY:X3`, `DPY:P5`), stream handling (`DPY:P2..P4`, `DPY:X2`), NVENC stream ownership (`DPY:V1`), NCCL temp-buffer ordering (`DPY:PE-01`), swallowed errors (`DPY:P8`, `DPY:NV-01`) | verify: pytest negative cases

### Wave 7 - Documentation
- [ ] **W7.1** Correct `crates/nv-demo/AGENTS.md`, root `AGENTS.md`, `plan1.md:33`, `optix` module docs as listed under Documentation drift; add the systemic defects to `## Traps` | verify: every cited constant re-read in source

## Execution record - Waves 1 and 2 (2026-11-23)

Eleven edit-only workers re-verified every finding in their crates before changing code; one serial gate then built and tested everything.

### Gate result (RTX 3080 Ti, CUDA 13.3)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean. `cargo test --workspace`: 379 passed, 0 failed, 197 ignored (110 binaries). `nv-demo` and `xtask` build without call-site changes.
- The commits `e1eed4c..d32d90f` are labelled "pre-gate" but already contain most of the gate's compile and test fixes, because they were committed while the gate was running; the label is inaccurate for them.
- Ignored GPU tests (`--ignored --test-threads=1`) pass for `cuda-core-ext`, `curand`, `cublas`, `cusolver`, `cusparse`, `cudnn`, `npp`, `nvjpeg`, `nvofa`, `nvrtc`, `cupti`, `nvenc`, `ngx` (smoke only), `optix`, except the deciding tests listed below.
- Fixes confirmed on hardware: buffer lease in `curand`; cubemap mip levels (`IOP:CE-01`); mip-count and foreign-stream rejection; B-frames give `NeedMoreInput` (`MED:ENC-1`); cropped 200x120 round trip (`MED:DEC-2`, `MED:ENC-3`); back-to-back instance uploads do not tear (`OPX:A1`); in-place denoise only untiled (`OPX:D1`); `InstanceBuffer` growth failure; `IOP:NW-01`, `NW-06`, `NW-07`; `SYS:CUPTI-1`, `CUPTI-2`; context binding in `cublas` / `cusolver` / `cusparse`; `MED:JPG-1`, `JPG-3`, `JPG-4`, `JPG-5`.
- A worker fix that was wrong and was reverted by the gate: `MED:JPG-6` replaced `resize(len, 0)` with `reserve` + `unsafe set_len`; a second `nvjpegEncodeRetrieveBitstream` for the same encode succeeds without writing, so safe code returned uninitialised heap. `resize` + `truncate` restored.
- Decided by hardware:
  - `MED:CMP-4` CONFIRMED and still open: `BitcompPlan::uncompress_async` accepts a truncated 8-byte input and the kernel faults with CUDA error 700, which then poisons the context for later tests in the process.
  - `IOP:CE-06` REFUTED: the driver accepts the DEDICATED flag for a D3D12 heap import.
  - `MED:DEC-3` not reproduced: 24 frames decoded with min PSNR 57.55 dB with the parser default DPB larger than the decoder's.
  - `OPX:D2`: destroying a denoiser or pipeline with queued work showed no corruption on this driver; still outside the documented OptiX contract.
  - `BLA:FFT-03`: cuFFT itself rejects `inembed = NULL` with `onembed` set (`CUFFT_INVALID_VALUE`), so the wrapper's rejection matches native behaviour.
  - nvJPEG hands out an encoded bitstream exactly once.
- Not run: `nccl` (Linux, 2+ GPUs), `cufile` GDS (Linux), `nvml`, the loader tests in `*-sys`; the probabilistic scratch-race tests in `cufft` and `cusparse` pass but were not run under `compute-sanitizer`, so they do not prove the absence of a use-after-free.
- Environment: `nvcomp` GPU tests need `NVCOMP_ROOT`; the loader does not look in `.sdk/nvcomp` by itself (`LDR` scope, Wave 5). The pre-existing `nvapi` hardware test `live_rtx_3080_ti_nvapi_queries` fails in `GetDisplayIdByDisplayName` (`EXPECTED_PHYSICAL_GPU_HANDLE`); cause not yet established.

### Findings refuted on re-verification (no code change)
- `BLA:LT-10` `HeuristicResult::from_algo`: `cublasLtMatmul` itself rejects an undersized workspace (`cublasLt.h:1017-1019`).
- `SOL:S2` Mg `lwork`: `unsafe fn` with an explicit contract. `SOL:S4` `csreigvsi` allocation: the module makes no zero-allocation promise.
- cuDNN workspace ownership (asked by the cuBLAS audit): cuDNN takes the workspace per call; `Handle` holds none.
- `SYS:NGX-5` (`DlssOptimalSettings` fallbacks mirror the SDK helper `NGX_DLSS_GET_OPTIMAL_SETTINGS`), `SYS:NVAPI-1`, `SYS:NVML-2` (no init requirement on the error-string calls), `SYS:NVML-1` (literal is correct and intentional).
- `SYS:NVRTC-1` (the log-fetch error is typed and names the call), null `CStr` checks in `cupti`/`nccl`/`nvrtc` (all seven sites already check).
- `MED:DEC-5` mechanism (`vidLock` is unrelated to the decoder, `cuviddec.h:306`); `MED:ENC-8` for CUDA arrays (pitch is defined differently there).
- `MED:CMP-9` compression-level part (levels above the maximum are accepted by design, `cascaded.h:157-165`); nvCOMP internal scratch never crosses streams.
- `OPX:PF1` (SBTs are built only at pipeline creation; making them stream-ordered would create the measured cross-stream hazard).
- `IOP:NW-13` (unreachable once the dedicated barrier queue is removed).
- Systemic 1 inside the wrappers: scratch buffers in `cublas`, `cudnn`, `cusolver`, `nvcomp`, `optix` are allocated and used on one checked stream. The only real cross-stream cases were `cusparse` SpSV/SpSM/SpGEMM (fixed by ownership, no lease) and caller-supplied buffers (`curand`, leased).
- Narrower than reported: `SYS:CUPTI-3` (field offsets are identical across record versions, so values were read correctly; the reference to a larger struct was formal UB and is fixed).

### Decisions taken by the lead (operator delegated design decisions on 2026-11-23)
| Topic | Decision | Rejected alternative and why |
| --- | --- | --- |
| Buffer leases | Targeted: `curand` (caller buffers) is leased; everything else is fixed by ownership or same-stream checks | Lease every entry point: about +2 us per call and new `NOT_READY` errors for a hazard that only exists for stream-ordered buffers crossing streams |
| `nv-wgpu` raw Vulkan work | Record into a `wgpu` command encoder and submit through `wgpu::Queue::submit`; own command runner, global submit mutex and dedicated barrier queue removed | Keep raw `vkQueueSubmit` with a private lock: it cannot be synchronised with wgpu's own submissions, which is undefined behaviour under the Vulkan spec |
| `optix` IAS children built on another stream | Require and check one stream; document it | Lease every child in `instances_into`: per-child cost on the per-frame path, new dependency, and `nv-demo` uses one stream |
| `ngx` thread safety | One process-wide mutex around every NGX call, `Send`/`Sync` kept | Make `NgxVulkan` `!Sync`: does not cover the static `required_extensions` path that Python calls from several threads |
| `cublas` workspace | The handle owns the workspace buffer while set and refuses to switch streams; `clear_workspace` restores the default pool | Two separate patches for the dangling pointer and the silent reset by `cublasSetStream` |
| cuBLASLt / descriptor raw attribute setters | `unsafe fn` | Re-validating every raw attribute against the typed cache: the attribute set is open-ended |
| `nvenc` B-frames | `encode_picture` returns `EncodeOutcome { Ready, NeedMoreInput }` | Keep status 17 as `Err`: the header calls it non-fatal and forbids locking the bitstream after it |
| `cusparse` SpSV/SpSM/SpGEMM `scratch` parameter | Now only a stream check; remove it in Wave 1b | Keep a parameter that no longer supplies memory |
| Dead error variants (`Budget`, `Unsupported`) in media crates | Remove together with the `python/nv-py` matches in Wave 6 | Remove now: breaks `python/nv-py/src/video.rs` |

### Behaviour changes callers will notice
- `cublas`: `set_stream` to another stream fails while a workspace is set; overlapping output batches, undersized `ld`, pointer-array layouts in `min_elems` and non-F32 grouped GEMM are rejected.
- `cufft`: `Plan` is no longer `Sync`; mixed `inembed`/`onembed` and overlapping output batches are rejected.
- `cudnn`, `nvcomp`: scratch budgets bound the whole allocation, not the growth; `cudnn::HeurMode` is no longer public.
- `cusparse`: `set_pointer_mode(Device)` fails; SpSV/SpSM/SpGEMM phases must run in order on one stream; `set_csr_pointers` requires the native `nnz` and matching index/value types.
- `nvjpeg`: decode validates against the bitstream geometry; `JpegStream::parse(.., false)` and `encode_yuv` with `Yuv410v` fail; a scratch buffer attaches to one state.
- `npp`: "no operation" warnings are errors; row step must be a multiple of the element size.
- `nvofa`: `set_io_streams` takes `&Arc<Stream>`; readback waits for the I/O streams.
- `nvenc` / `nvdec`: `set_io_streams(&Arc<Stream>, ..)`; `DecoderConfig::display_area`; `map_frame` ties the frame to its stream.
- `cufile`: `FileHandle::read` takes `&mut D`; reusing a slot without harvesting and `try_complete` on an idle slot fail.
- `ngx`: success follows `NVSDK_NGX_SUCCEED`; a second `init` on a live `VkDevice` fails; outputs without `read_write` are rejected.
- `cupti`: activity kinds are reference-counted per process; dropping a `Subscriber` waits for running callbacks (dropping it inside its own callback deadlocks).
- `nccl`: `GroupGuard` is `!Send`; no `Abort` after a failed `Destroy`.
- `optix`: in-place tiled denoise, streams/SBTs of another CUDA context are rejected; compacting builds need about twice the output size in budget.
- `nv-wgpu` / `cuda-core-ext`: `with_vulkan_barrier_queue` removed; drops of semaphores and shared resources wait for pending work; 2D textures import as 2D CUDA arrays; mip counts and timeline values are validated.

### Open after Waves 1-2
- Needs a run of `nv-demo` on hardware: DLSS SR/RR after NGX commands moved into the wgpu encoder; camera feel after the `camera-rs` switch.
- Needs Linux / multi-GPU: `nccl` drop behaviour, `cufile` GDS session lifetime, every Linux cfg branch touched.
- Needs a Vulkan test harness for `ngx` (`ash` as a dev-dependency): `SYS:NGX-2`, `SYS:NGX-7`, `SYS:NGX-11` have documentation only.
- Callers to adapt in Wave 6 (`python/nv-py`): `video.rs:551-552` unmaps before `lock` (violates `nvEncodeAPI.h:4180`), `NeedMoreInput` handling and `flush`, `platform_ext.rs:1234` passes a shared buffer to `FileHandle::read`, `compute.rs:415-456` calls `cufftExec*` directly.
- Reported outside their manifests, not fixed yet: `cublas` `MatmulDesc::set_emulation_desc` does not retain the `EmulationDesc`, `gemm_*_on_bound_stream` do not bind the context, `set_epilogue` / `set_bias_batch_stride` remain safe; `cudnn` `GraphConvPlan::build*` does not bind; `nv-demo` empty CUPTI `Subscriber` occupies the single process-wide subscriber slot (blocks Nsight).

## Out of scope
- External dependencies: `cuda-core` / `cuda-bindings` (NVIDIA), `csg-rs`, `camera-rs`, `appbus-rs`. Two `csg-rs` notes are recorded in `10_demo_gpu.md` (`E01`, `E02`) for that repository.
- Generated `prebuilt/` bindings, the Astro site under `docs/`, untracked `0/`.
- Performance work beyond the stalls listed above; no benchmark was run in this audit.
