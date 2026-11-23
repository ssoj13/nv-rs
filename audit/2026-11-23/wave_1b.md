# Wave 1b report - cross-crate items (2026-11-23)

Base: `main` `4f457d0`. Machine: RTX 3080 Ti, CUDA 13.3, Windows 11. Nothing is committed; everything is in the working tree.

Method: the lead wrote the three shared modules in `cuda-core-ext` and the cuDNN loader change, and built `cuda-core-ext` once. Five edit-only workers then changed disjoint crates and never ran cargo: W1 `cublas`/`cufft`/`curand`, W2 `cudnn`/`nvcomp`, W3 `cusolver`/`cusparse`, W4 `nccl`/`cufile`/`cupti`/`ngx`, W5 `nvenc`/`nvdec`/`nvofa`/`nvjpeg`/`optix`. The lead then made the `python/nv-py` edits and ran every build and test, one at a time.

The whole workspace compiled on the first `cargo check` after the workers finished. One ignored GPU test failed (cusparse P10, item 1); it was rewritten and re-run.

Concurrency with the csg-rs session: nothing went red, so nothing had to be re-run for timing. One message from the csg-rs session (#640) reached worker W5: it said `csg-rs-adoption` is rebased on `4f457d0` but not ready, because one nv-demo test still fails. W5 acknowledged it and did nothing else.

## Item 1 - Context identity: DONE

**Design.**
- `crates/cuda-core-ext/src/identity.rs` is new and holds the only two rules:
  - `same_context(a, b)` = `a == b`. `cuda_core::Context: PartialEq` compares the device, the `CUcontext` handle and the ordinal (`cuda-core/src/runtime.rs:154-160`).
  - `same_stream(a, b)` = same `cu_stream()` and `same_context`.
- Both are re-exported at the crate root. The workspace dependency `cuda-core-ext = { path = "crates/cuda-core-ext" }` is in the root `Cargo.toml`.
- Crate-local duplicates are deleted or now delegate to these: the `same_stream` copies in `cusparse`, `optix`, `cudnn` and `nvcomp`; the `same_ctx` wrappers in `cudnn`, `nccl`, `nvcomp` and `optix` stay but call `same_context`.

**Converted sites.** Line numbers are after the edit.
- **cublas**
  - `Arc::ptr_eq` contexts: `handle.rs:328, 435`; `lt.rs:442, 505, 582, 632, 797, 846, 894`.
  - `std::ptr::eq` on streams: `lt.rs:447, 510` -> `same_stream`.
- **cudnn**
  - `cudnn.rs:177` (`same_ctx`); the local `same_stream` (old `cudnn.rs:160`) is deleted.
  - `descriptor.rs:802` (x2), `:1052`; `graph.rs:759`.
  - `handle.rs:102`: handle-only stream comparison -> `same_stream`.
  - `ops.rs`: `:122` (x2), `:138` (x2), `:174-177`, `:230`, `:306-309`, `:362-365`, `:480-483`, `:537-540`, `:660-663`, `:1087`, `:1142`. 39 checks in total.
- **cufft**
  - `plan.rs:489, 668, 696` (contexts).
  - `plan.rs:702, 731`: handle-only stream comparison -> `same_stream`.
  - `scratch.rs:85` (old `Arc::ptr_eq` on streams) is removed; the shared `Scratch` does that check now.
- **cufile**: `handle.rs:782, 972, 975, 1078, 1081`. All are Linux-relevant and were not compiled for Linux.
- **curand**: `generator.rs:316`.
- **cusolver**
  - `dn.rs:790`, `sp.rs:430`.
  - `cusolver.rs:362` went away with the old `Scratch`.
- **cusparse**
  - `handle.rs:53`; `ops.rs:237` (`ensure_handle_context`).
  - `cusparse.rs:390` (local `same_stream`) is deleted.
- **nccl**
  - `nccl.rs:128` (`same_ctx`).
  - `comm.rs:522` (x2), `579, 640, 654, 728, 742`, `804` (x2), `865` (x2), `929, 983`. Mostly Linux-only code, not compiled for Linux.
- **nvcomp**
  - `nvcomp.rs:213` (`same_ctx`).
  - `nvcomp.rs:221` (`same_stream` was `Arc::ptr_eq` on streams; now the shared rule, signature `(&Stream, &Stream)`).
  - `batched.rs:209-210, 348, 431-432`; `bitcomp.rs:197` (x2), `248` (x2).
  - `bitcomp.rs:172`: handle-only stream comparison -> `same_stream`.
- **cuda-core-ext**: `sparse.rs:97` (`Arc::ptr_eq`), `sparse.rs:176`, `array.rs:285`, `external.rs:302` (raw `cu_ctx()` comparisons).
- **optix** compared CUDA contexts by value already; these now go through the shared helper:
  - `accel.rs:605` (`same_ctx`), `accel.rs:692`; the local `same_stream` is deleted and replaced by the shared one.
  - `cluster.rs:828`, `coop_vec.rs:327-328`, `pipeline.rs:240`, `sbt.rs:136`, `xform.rs:429`.

**Not converted, on purpose.**
- `optix` `Arc::ptr_eq` on OptiX `DeviceContext` / `Module` / `ProgramGroup` objects (`accel.rs:839, 1660, 1850, 1888, 2505, 2535, 2576`, `cluster.rs:508, 512`, `micromap.rs:587`, `module.rs:334, 875`, `pipeline.rs:95`, `stack.rs:195`, `xform.rs:424`): this is object identity.
- `nvjpeg/handle.rs:499, 527`: buffer identity.
- `nv-wgpu` had no `Arc::ptr_eq` on contexts.
- `python/nv-py/src/core.rs:373` already compares by value and was left as is (python edits are limited to call sites of changed APIs).

**nv-py cuDNN.** `python/nv-py/src/sparse_ai.rs` `activation_forward_f32` / `softmax_forward_f32` now call `cudnn::activation_forward` / `cudnn::softmax_forward` on `typed_view::<f32>` views inside `enqueue_leased`. The raw `cudnn_sys` calls, the manual size checks, the `c_void` import and the now-unused `cudnn-sys` dependency in `python/nv-py/Cargo.toml` are removed. The module doc is updated.

**Behaviour change and test.** Two `Context::new(0)` retain the same primary context, so they are now one context. `crates/cusparse/tests/gpu.rs` P10 is renamed `descriptors_from_another_arc_of_the_same_context_are_accepted` and asserts acceptance plus `same_context`. W2 made the same change in `cudnn/src/graph.rs:835-839`: the GPU test now asserts that `plan.bind(&other_handle)` succeeds.

## Item 2 - Typed teardown error sink: DONE

**Design.** `crates/cuda-core-ext/src/teardown.rs` is new.
- **API:**
  - `Library` is a `#[non_exhaustive]` enum with 16 values.
  - `TeardownError { library, op: &'static str, status: i64 }` keeps the real native status, unchanged.
  - `record(lib, op, status)`; `check(lib, op, status, success)`.
  - `take(Option<Library>) -> Drained { errors, lost }` is the explicit drain, for one library or for all.
  - `bind(ctx) -> bool`.
- **Storage:** a process-wide bounded queue (`CAPACITY = 256`). When full, the oldest entry is dropped and counted per library in `lost`. It never panics; a poisoned mutex is recovered, which is safe because every update is a single step. Unit test: `teardown::tests::records_drains_by_library_and_counts_overflow`.
- **Decision.** Wrappers no longer call `record_err` for library statuses. Real CUDA driver results (`cuMemFree`, event/stream sync, `bind_to_thread`, cuvid `CUresult`) stay on `Context::record_err`, which is `cuda-core`'s own convention for `Drop`.
  - `teardown::bind` replaces the invented `NOT_INITIALIZED` / `SETUP_FAILED` / `INITIALIZATION_FAILED` codes on bind failure. It puts the driver error back on the context, because `bind_to_thread` reads and clears a pending one, and tells the caller to skip the native call.
  - A few drops deliberately still call destroy after a failed bind, so one sticky error does not leak the native object: `nvenc` `EncodeSession`, `nvofa` `OfBuffer`, `nvjpeg` `bind_for_drop`, the `optix` destroys and the `cupti` process-global calls.

**Sites.** Before, every one of these was `record_err(CUDA_ERROR_UNKNOWN)` or another invented code; now each records `Library::X` with the native op name and status.
- **cublas**
  - `handle.rs:510` `cublasDestroy_v2`.
  - `lt.rs:956` `cublasLtDestroy`, `:1381` `cublasLtMatmulDescDestroy`, `:1824` `cublasLtMatrixLayoutDestroy`, `:2072` `cublasLtMatmulPreferenceDestroy`, `:2247` `cublasLtMatrixTransformDescDestroy`, `:2429` `cublasLtEmulationDescDestroy`.
  - Helpers are in `error.rs`.
- **cufft**: `plan.rs:400, 928` `cufftDestroy`.
- **curand**
  - `generator.rs:229` `curandDestroyDistribution`, `:801` `curandDestroyGenerator`.
  - The two context-less host branches used `eprintln!`; the lead routed them to the sink too.
- **cudnn**
  - `descriptor.rs`: `cudnnDestroyTensorDescriptor` at 381, 424, 445, 485; `cudnnDestroyFilterDescriptor` at 531, 544, 590, 603, 643; `cudnnDestroyConvolutionDescriptor` at 703, 763, 856; `cudnnDestroyActivationDescriptor` at 893, 921; `cudnnDestroyPoolingDescriptor` at 979, 1037, 1093.
  - `graph.rs:303` `cudnnBackendDestroyDescriptor`; `ops.rs:1239` `cudnnDestroyTensorDescriptor`; `handle.rs:114` `cudnnDestroy`.
- **nvcomp**: `bitcomp.rs:346` `bitcompDestroyPlan`. `scratch.rs` `BatchStaging` drop is an event sync and stays on `record_err`.
- **cusolver**
  - `dn.rs:1135` `cusolverDnDestroy`, `sp.rs:557` `cusolverSpDestroy`.
  - `mg.rs` `cusolverMgDestroy` / `cusolverMgDestroyGrid` / `cusolverMgDestroyMatrixDesc`.
  - `sp.rs` `cusparseDestroyMatDescr`, three sites, recorded as `Library::Cusparse` because it is a cuSPARSE object with a cuSPARSE status.
- **cusparse**: `descr.rs:872` `cusparseDestroySpMat`, `:959` `cusparseDestroyDnVec`, `:1073` `cusparseDestroyDnMat`; `handle.rs:142` `cusparseDestroy`; `ops.rs:750` `cusparseSpGEMM_destroyDescr`, `:911` `cusparseSpSV_destroyDescr`, `:1082` `cusparseSpSM_destroyDescr`.
- **nccl** (Linux-only code):
  - `comm.rs:250, 271, 473` `ncclCommAbort`.
  - Drop: `ncclCommGetAsyncError` (real status, including an in-progress state), `ncclCommDestroy`, `ncclCommAbort`.
  - `nccl.rs:136` helper.
- **cufile**
  - `cuFileDriverClose_v2`, `cuFileBufDeregister`, `cuFileStreamDeregister`, `cuFileHandleDeregister`. These are Linux-only; a real `cu_err` inside `CUfileError_t` goes to `record_err`.
  - The AsyncIoSlot deferred POSIX write failure is recorded as `pwrite` / `WriteFile` with the OS error code.
  - `TEARDOWN_NO_STATUS = -1` is used where no native status exists: an unresolved void entry point, or an OS error without a code.
- **cupti**: `activity.rs` `cuptiActivityDisable`, `cuptiActivityFlushAll`; `subscriber.rs` `cuptiUnsubscribe`.
- **nvenc**: `nvenc.rs:668, 938` `nvEncDestroyEncoder`, `~1049` `nvEncDestroyInputBuffer`, `~1103` `nvEncUnlockInputBuffer`, `~1289` `nvEncUnregisterResource`, `~1318` `nvEncUnmapInputResource`, `~1433` `nvEncDestroyBitstreamBuffer`, `~1474` `nvEncUnlockBitstream`.
- **nvofa**: `~780` `nvOFDestroy`, `~1030` `nvOFDestroyGPUBufferCuda`.
- **nvjpeg** (10 sites):
  - `handle.rs`: `nvjpegDestroy`, `nvjpegBufferPinnedDestroy`, `nvjpegBufferDeviceDestroy`, `nvjpegDecoderDestroy`, `nvjpegDecodeParamsDestroy`, `nvjpegJpegStateDestroy`, `nvjpegJpegStreamDestroy`.
  - `encode.rs`: `nvjpegEncoderParamsDestroy`, `nvjpegEncoderStateDestroy`.
- **optix**: `context.rs:405` `optixDeviceContextDestroy`, `pipeline.rs:386` `optixPipelineDestroy`, `module.rs:627` `optixModuleDestroy`, `module.rs:945` `optixProgramGroupDestroy`, `denoise.rs:1220, 1244` `optixDenoiserDestroy`.
- **ngx**
  - `NgxGlobal::record_teardown` now records `Library::Ngx` in the shared sink; the private `teardown_error` slot is removed.
  - Sites: `params.rs` `NVSDK_NGX_VULKAN_DestroyParameters`, `vulkan.rs` `NVSDK_NGX_VULKAN_Shutdown1`, `vulkan.rs` `NVSDK_NGX_VULKAN_ReleaseFeature`.
- **nvdec**: unchanged. Its cuvid calls return `CUresult`, which is a driver error, and no status is invented.

**Tests adapted.**
- `nvenc/tests/gpu.rs:178`, `nvjpeg/tests/gpu.rs:305-322` and `nvofa/tests/gpu.rs:213-218` now also assert that their library's sink is empty.
- The ngx unit test checks that a failure is kept exactly (`TeardownError { Ngx, "NVSDK_NGX_VULKAN_ReleaseFeature", status }`) and drained once.

## Item 3 - One `Scratch` owner: DONE

**Design.** `crates/cuda-core-ext/src/scratch.rs` is new: `Scratch` plus `ScratchError { WrongStream, Budget, Misaligned, Driver }`.
- **Budget:** it bounds the whole request, even when the existing buffer is reused.
- **Growth:**
  - The old buffer is released before the new one is allocated; both are stream-ordered on the bound stream.
  - `growth(needed)` charges the full new size.
- **Stream check:** `same_stream`.
- **Generation:** a counter that changes on every reallocation and on `clear`.
- **Retaining the buffer:** it is held as `Arc<DeviceBuffer<u8>>`, so `shared_buffer()` lets cuBLAS keep the memory its workspace pointer refers to.
- **Alignment:** `reserve_aligned` (nvCOMP).
- **Tests:** unit test `growth_charges_the_full_new_allocation` and GPU test `reserve_budget_stream_and_generation`.
- **Error mapping:** every crate has `impl From<ScratchError> for Error`.

**Per crate.**
- `cublas`, `cudnn`, `cusolver`, `cusparse`, `nvcomp`: `pub use cuda_core_ext::Scratch` (+ `ScratchError` in cusolver and cusparse). The crate files `cublas/src/scratch.rs` and `cudnn/src/scratch.rs` are deleted; the `Scratch` parts of `cusolver.rs`, `cusparse.rs` and `nvcomp/src/scratch.rs` are removed.
- `cufft`: a thin Clone newtype over `Arc<Shared { stream, scratch: Mutex<cuda_core_ext::Scratch> }>`.
  - Crate invariant: one workspace is shared by several plans, and the lock is held across `cufftExec*`.
  - The internal `reserve` returns the pointer and the generation under one lock (this was a race before).
  - The public API is unchanged.
- `nvcomp` `BatchStaging` keeps its own batch generation counter, which receipts need.
  - Lead change: `BatchStaging::prepare` (`scratch.rs:~200`) passes `None` to the inner `reserve_aligned`, because `prepare` already admits the combined table + scratch growth against the budget. Passing it again would reject reuse of an existing workspace that costs no new memory.

**Public API changes and callers.**
- `cublas::Scratch`:
  - `with_capacity(.., budget: Option<usize>)` (was `usize`); the `cublas::scratch` module is removed.
  - `reserve` / `shared_buffer` are now public on the shared type.
  - Callers needed no change: `cublas.rs:229, 256, 407`, `lt.rs:~3208`, `handle.rs` `set_workspace` (internal).
- `cudnn::Scratch`:
  - `reserve(&Stream, ..)` returns `ScratchError`; `growth` is now the full new size (was `needed - capacity`); the `cudnn::scratch` module is removed.
  - Callers: `ops.rs:245, 324, 499, 679` (unchanged via `From`).
- `cusolver::Scratch`:
  - `reserve_bytes(&Stream, usize, usize)` is removed; use `reserve(&Stream, usize, Option<usize>)`.
  - `reserve_elements` became the `pub(crate)` function `reserve_elements(scratch, ..)`.
  - Callers: `dn.rs:821, 898, 965, 1003, 1064, 1111`.
- `cusparse::Scratch`:
  - `reserve_with_budget(&Stream, usize, usize)` is removed; use `reserve(.., Option<usize>)`.
  - Callers: `ops.rs:342, 454`; `tests/gpu.rs` at the probe (~182), `clobber` (~395), pre-grow (~521, ~608) and the bench (~906).
- `nvcomp::Scratch`: `reserve(stream, needed, align, budget)` became `reserve_aligned(..)`, and `reserve` no longer takes `align`. Caller: `scratch.rs` `prepare`.
- `python/nv-py` needed no change for item 3 (it only uses `Scratch::new` of cufft, cusparse and cusolver).

## Item 4 - cusparse SpSV / SpSM / SpGEMM `scratch` parameter: DONE

- `SpGemm::work_estimation`, `SpGemm::compute`, `SpSv::prepare` and `SpSm::prepare` no longer take `scratch: &mut Scratch`, and their `stream` is now `&Arc<Stream>`. The descriptor-owned workspace is created with `Scratch::new(stream)`.
- `owned_workspace(slot, stream)` no longer returns `Result`.
- Callers updated in `crates/cusparse/tests/gpu.rs`: spgemm test (~162, ~196, ~218), spsv/spsm test (~304, ~348), `estimate_and_compute` helper (~434), P2 (~544), P3 (~614), P4 (~684).
- No callers in `python/`, `nv-demo`, `xtask` or `device/` (grep).

## Item 5 - Dead error variants: DONE

- Removed: `nvenc::Error::{Unsupported, Budget}`, `nvdec::Error::{Unsupported, Budget}`, `nvofa::Error::Budget`. Repo-wide grep: nothing constructs them.
- Kept: `nvofa::Error::Unsupported`, which is constructed at `nvofa.rs:714`.
- Matches removed in `python/nv-py/src/video.rs:53-73`.

## Item 6 - cuDNN adopts an already-loaded copy: DONE

**Loader changes** (`crates/nv-loader/src/nv_loader.rs`):
- `cached_loader!` and `exports_sys!` take an optional `adopt: fn() -> Option<PathBuf>` hook. It is placed after the accessor in `cached_loader!`, because placing it before the accessor was ambiguous for `macro_rules!`.
- New `pub fn open_adopted(spec, path, open)`:
  - Records diag event `<lib>`/`adopt` (Info) and `record_lib_load` with the adopted path.
  - Runs the same `open` (preload + version gate).
  - A rejection is final: there is no fallback to the search, because that would load the second copy that adoption avoids.

**cuDNN** (`crates/cudnn-sys/src/cudnn_sys.rs`):
- `adopt: loaded_copy`. On Windows it uses `GetModuleHandleExW("cudnn64_<CUDNN_MAJOR>.dll")` and `GetModuleFileNameW` (with SAFETY comments) to get the loaded module's full path. The existing preload then maps the siblings from that directory, and `reject_foreign_copy` stays as the guard.
- On Linux it returns `None`, because the dynamic linker already shares one soname.
- If the path cannot be read, a Warn diag is recorded and the normal search follows.

**Docs.** The module doc and `crates/AGENTS.md` (Review checklist) state that the reverse order cannot be prevented from Rust: nv-rs loads cuDNN first, then `import torch` brings a different build and fails with WinError 127.

**Verification.**
- `cargo test -p cudnn-sys -p nv-loader -- --include-ignored --test-threads=1`: 5 + 34 passed. This includes the new ignored test `adopts_the_copy_already_loaded`.
- Python, torch 2.8.0+cu129 (cuDNN 9.10.2) imported first: `test_nv_rs_all.py -k cudnn` passed. The process then held only torch's `cudnn64_9.dll`, `cudnn_graph64_9.dll` and `cudnn_cnn64_9.dll` (`GetModuleFileNameW`). Script: `scratchpad/torch_first.py`.
- The cuDNN Python test now runs and passes; at the w6 gate it was skipped.

## Item 7 - Stubs, rebuild, pytest: DONE, one expectation not met

**Commands.**
- `cargo run -p xtask -- py-stubs generate` rewrote `python/nv-py/python/nv_rs/__init__.pyi` and, from the same generator, `docs/src/content/docs/python/04-modules-reference.md`. `py-stubs check`: ok (25 submodules).
- `maturin develop` (debug, editable) into `C:\Programs\Ntutil\apps\prog\lang\a3\envs\main`, with `CARGO_TARGET_DIR=python/nv-py/target`.
- `python -m pytest python/nv-py/tests -q` with `NVCOMP_ROOT=.sdk\nvcomp`, all files in one process: **3 failed, 78 passed, 1 skipped**. At the w6 gate it was 4 failed, 77 passed.

**Remaining failures.**
- `test_nvapi_platform_behavior`: expected (NVAPI sees 0 displays on this machine).
- `test_nv_rs.py::test_dlpack_and_cuda_array_interface_interop` and `test_nv_rs_m3_m4.py::test_cublas_gemm_f32_dlpack_and_numpy`: `import torch` raises WinError 127 on `torch\lib\cudnn_cnn64_9.dll`.
  - Cause: an earlier test file (`test_compute_media_repairs.py`, which sorts first) loads cuDNN through nv-rs before torch is imported. This is exactly the reverse order that decision 6 says cannot be prevented from Rust.
  - Both tests pass when run on their own: `pytest test_nv_rs.py test_nv_rs_m3_m4.py`, 13 passed.
  - Fixing the test run needs a test-side change, such as importing torch first in a `conftest.py` or putting torch tests in their own process. That is outside this wave's scope (it is a python test change, not a call site), so it is not done.

**Stub parity test:** passes now.

## Item 8 - Final gate

**Root workspace.**
- `cargo clippy --workspace --all-targets --message-format=short -- -D warnings`: exit 0, `Finished`, no warnings. The only build-script output is the existing nvcomp-sys "using prebuilt bindings" notice when `NVCOMP_ROOT` is unset.
- `cargo test --workspace --quiet --no-fail-fast`: **467 passed, 0 failed, 205 ignored**.
- `cargo run -p xtask -- coverage check`: ok.

**Ignored GPU tests** (`cargo test -p <crate> --quiet -- --ignored --test-threads=1`, `NVCOMP_ROOT=$PWD/.sdk/nvcomp`):

| Crate | Passed | Failed |
| --- | --- | --- |
| cuda-core-ext | 11 | 0 |
| cublas | 9 | 0 |
| cufft | 3 | 0 |
| curand | 4 | 0 |
| cudnn | 5 | 0 |
| cudnn-sys | 2 | 0 |
| nvcomp | 7 | 0 |
| cusolver | 9 | 0 |
| cusparse | 11 | 0 (after the P10 rewrite; it was 10/1 before) |
| cupti | 5 | 0 |
| nvenc | 4 | 0 |
| nvdec | 2 | 0 |
| nvofa | 4 | 0 |
| nvjpeg | 7 | 0 |
| optix | 67 | 0 |
| ngx | 1 | 0 |

- `nccl` and `cufile` have no ignored tests on Windows. `nv-loader` has no ignored tests; its non-ignored tests pass (34).

**Python workspace** (inside `python/nv-py`, `CARGO_TARGET_DIR=python/nv-py/target`):
- `cargo clippy --all-targets --message-format=short -- -D warnings`: clean.
- `cargo test --lib --quiet`: 19 passed, 0 failed, 5 ignored.
- `cargo test --lib -- --ignored --test-threads=1`: 5 passed, 0 failed.
- pytest: 3 failed, 78 passed, 1 skipped (see item 7).

## Deviations

| Deviation | Reason |
| --- | --- |
| cuBLASLt constructors lost their `cuda` parameter (`MatmulDesc::new`, `MatrixLayout::new`, `MatrixLayout::new_grouped`, `Preference::new`, `MatrixTransformDesc::new`, `EmulationDesc::new`) | The stored `Arc<Context>` was read only by the old teardown mapping and became dead; these descriptors are host-only objects. Callers updated: `cublas.rs:396-401` and the `lt.rs` tests. No caller outside `cublas` (grep, including `nv-demo` and `python`). |
| Fields kept only for lifetime were renamed with a leading underscore: `cupti::Subscriber::_cuda`, `cudnn` `BackendDesc::_ctx` | They keep the context alive until the native object is destroyed and are no longer read. |
| `ngx::take_teardown_error() -> Option<Error>` became `ngx::take_teardown_errors() -> cuda_core_ext::teardown::Drained` | Now a user of the shared sink; no external caller. `CHANGELOG.md:41` still names the old function (not edited, per the rules). |
| `cusolver/src/sp.rs` `cusparseDestroyMatDescr` is recorded as `Library::Cusparse` | It returns a cuSPARSE status. |
| nvcomp `BatchStaging::prepare` passes `None` to the inner scratch reserve | Its combined budget check already ran; see item 3. |
| `docs/.../04-modules-reference.md` changed | It is output of `xtask py-stubs generate` (item 7). |
| `optix::Scratch` (`accel.rs:~729`) is not unified | Not in the item 3 list; it has OptiX-specific temp/output accounting. |

## Risks / not done

- **Linux-only code is not compiled:** NCCL, cuFile and the Linux branches of `cudnn-sys` `loaded_copy`. These edits were checked only by reading. A Linux build is required.
- **The teardown sink is process-wide.** The new "sink empty" assertions in the nvenc / nvjpeg / nvofa GPU tests assume that no other test in the same binary has a failing teardown for that library at the same moment.
- **Teardown errors are no longer visible through `Context::check_err`.** A user who relied on that must drain `cuda_core_ext::teardown::take`. No in-repo code did (grep).
- **Budget behaviour changes:**
  - `cusparse`, `cusolver`, `cublas`, `cufft`: unchanged.
  - `nvcomp` direct `Scratch` users: the budget now also bounds the request on reuse.
  - `cudnn::Scratch::growth`: now the full new size.
- **pytest is not fully green**, for the test-order reason in item 7.

## `git status --short`

```
 M Cargo.lock
 M Cargo.toml
 M crates/AGENTS.md
 M crates/cublas/Cargo.toml
 M crates/cublas/src/cublas.rs
 M crates/cublas/src/error.rs
 M crates/cublas/src/handle.rs
 M crates/cublas/src/lt.rs
 D crates/cublas/src/scratch.rs
 M crates/cuda-core-ext/src/array.rs
 M crates/cuda-core-ext/src/cuda_core_ext.rs
 M crates/cuda-core-ext/src/external.rs
 M crates/cuda-core-ext/src/sparse.rs
 M crates/cudnn-sys/src/cudnn_sys.rs
 M crates/cudnn/Cargo.toml
 M crates/cudnn/src/cudnn.rs
 M crates/cudnn/src/descriptor.rs
 M crates/cudnn/src/graph.rs
 M crates/cudnn/src/handle.rs
 M crates/cudnn/src/ops.rs
 D crates/cudnn/src/scratch.rs
 M crates/cufft/Cargo.toml
 M crates/cufft/src/cufft.rs
 M crates/cufft/src/error.rs
 M crates/cufft/src/plan.rs
 M crates/cufft/src/scratch.rs
 M crates/cufile/Cargo.toml
 M crates/cufile/src/cufile.rs
 M crates/cufile/src/handle.rs
 M crates/cupti/Cargo.toml
 M crates/cupti/src/activity.rs
 M crates/cupti/src/cupti.rs
 M crates/cupti/src/subscriber.rs
 M crates/curand/Cargo.toml
 M crates/curand/src/curand.rs
 M crates/curand/src/error.rs
 M crates/curand/src/generator.rs
 M crates/cusolver/Cargo.toml
 M crates/cusolver/src/cusolver.rs
 M crates/cusolver/src/dn.rs
 M crates/cusolver/src/mg.rs
 M crates/cusolver/src/sp.rs
 M crates/cusparse/Cargo.toml
 M crates/cusparse/src/cusparse.rs
 M crates/cusparse/src/descr.rs
 M crates/cusparse/src/handle.rs
 M crates/cusparse/src/ops.rs
 M crates/cusparse/tests/gpu.rs
 M crates/nccl/Cargo.toml
 M crates/nccl/src/comm.rs
 M crates/nccl/src/nccl.rs
 M crates/ngx/Cargo.toml
 M crates/ngx/src/ngx.rs
 M crates/ngx/src/params.rs
 M crates/ngx/src/vulkan.rs
 M crates/nv-loader/src/nv_loader.rs
 M crates/nvcomp/Cargo.toml
 M crates/nvcomp/src/batched.rs
 M crates/nvcomp/src/bitcomp.rs
 M crates/nvcomp/src/nvcomp.rs
 M crates/nvcomp/src/scratch.rs
 M crates/nvdec/src/nvdec.rs
 M crates/nvenc/Cargo.toml
 M crates/nvenc/src/nvenc.rs
 M crates/nvenc/tests/gpu.rs
 M crates/nvjpeg/Cargo.toml
 M crates/nvjpeg/src/encode.rs
 M crates/nvjpeg/src/handle.rs
 M crates/nvjpeg/tests/gpu.rs
 M crates/nvofa/Cargo.toml
 M crates/nvofa/src/nvofa.rs
 M crates/nvofa/tests/gpu.rs
 M crates/optix/Cargo.toml
 M crates/optix/src/accel.rs
 M crates/optix/src/cluster.rs
 M crates/optix/src/context.rs
 M crates/optix/src/coop_vec.rs
 M crates/optix/src/denoise.rs
 M crates/optix/src/module.rs
 M crates/optix/src/optix.rs
 M crates/optix/src/pipeline.rs
 M crates/optix/src/sbt.rs
 M crates/optix/src/xform.rs
 M docs/src/content/docs/python/04-modules-reference.md
 M python/nv-py/Cargo.toml
 M python/nv-py/python/nv_rs/__init__.pyi
 M python/nv-py/src/sparse_ai.rs
 M python/nv-py/src/video.rs
?? 0/
?? audit/2026-11-23/wave_1b.md
?? crates/cuda-core-ext/src/identity.rs
?? crates/cuda-core-ext/src/scratch.rs
?? crates/cuda-core-ext/src/teardown.rs
```
