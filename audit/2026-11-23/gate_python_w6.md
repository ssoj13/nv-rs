# Gate report: python/nv-py (Wave 6), 2026-11-23

HEAD `4dd4e2d`, RTX 3080 Ti, CUDA 13.3, Windows 11. Every cargo command ran inside `python/nv-py` with `CARGO_TARGET_DIR=python/nv-py/target`. Nothing was run in the root workspace and nothing is committed: all fixes are in the working tree.

## 1. Build status and edits
The build compiled on the first try. That covers the four workers' ungated edits and the path-dependent root crates as they stood around 14:25. No compile error came from the workers' code, so no compile fix was needed.

Edits made by this gate:

| File | What | Why |
| --- | --- | --- |
| `src/buf_util.rs` (top of file) | Moved `AnyBits` (with its 4 impls), `typed_view` and `poisoned` here from `compute.rs`. Deleted `buf_ptr_and_bytes` and `validate_dev_buf`. Updated the module `//!` docs. | Decided follow-up. The two deleted helpers had no callers left (grep). |
| `src/compute.rs:12-14, :23` | Removed the moved block and its doc note; `use crate::buf_util::{.., poisoned, typed_view}` | Same move |
| `src/sparse_ai.rs:33`, `src/nvjpeg.rs:31`, `src/nvcomp.rs:16` | Imports now come from `buf_util` | Same move |
| `src/core.rs:480-506` | New `PyDeviceBuffer::wait_for_foreign_writes()`, explained below | Decided follow-up (`__dlpack__` waits for leased writes) |
| `src/core.rs:~889` (`__dlpack__`) | Calls `wait_for_foreign_writes` inside `py.detach`, after `sync_for_consumer_stream`, unless `stream == -1` | Same |
| `src/core.rs` tests | New `gpu_dlpack_export_waits_for_leased_write_on_another_stream` (ignored GPU test). An idle buffer returns immediately; a buffer leased for writing behind a 200 ms busy stream waits at least 100 ms. | Proves the wait |
| `src/platform_ext.rs:1040` (Linux only) | `context.device_id()` changed to `context.inner.ordinal()` | Definite Linux compile error E0624: `PyContext::device_id` is a private `#[getter]` in `core.rs:86` |
| `tests/test_nv_rs_all.py:384-418` | NGX test moved to the new API, detailed below | Decided follow-up |
| `tests/test_sys_platform_nvml.py:32, :94` | `nv_rs.trace("nvml-optional")` and `nv_rs.trace("cupti-bound")` changed to `nv_rs.trace("trace")` | Test bug: the first argument is the diag level, so the tests raised `ValueError: invalid diag level` |

New NGX test (`tests/test_nv_rs_all.py`):
- It no longer calls `is_dlss_supported()` or reads `caps.supported` / `caps.status_message`.
- `required_extensions()` and `feature_requirements()` are wrapped in try/except; the test skips only on `NvLibraryNotFoundError`.
- It asserts that `feature_requirements("super_sampling")` agrees with `required_extensions()`.
- An unknown feature must raise `NvInvalidArgumentError`.
- The removed attributes must be absent from `NgxCapabilities`.

### How `wait_for_foreign_writes` works and where it is limited
`cuda-core` keeps the completion events of leased accesses private (`allocation.rs`: `Accesses` and `Access.completion` are not public). The only public probe is `RawDeviceBuffer::acquire(stream, write=false, ..)`, which returns `CUDA_ERROR_NOT_READY` when an unfinished write on another stream overlaps the buffer.

The new method works like this:
- It probes with the buffer's own stream, or with a fresh stream when the buffer has none (for example a DLPack import).
- When the probe reports a conflict, it calls `Context::synchronize()`, the only wait the public API offers.

The result is correct, but coarse: it blocks on the whole context, not on the one event. A precise wait (`cuStreamWaitEvent` on the consumer stream) needs `cuda-core` to expose the pending write events of an allocation, for example `RawDeviceBuffer::pending_write_events() -> Vec<Arc<Event>>`.

## 2. Rust tests
- `cargo test --lib`: 19 passed, 0 failed, 5 ignored.
- `cargo test --lib -- --ignored --test-threads=1`: 5 passed, 0 failed:
  - `buf_util` unleased-corrupts: the hazard still reproduces;
  - `buf_util` leased-keeps-alive;
  - `buf_util` rejects conflicting cross-stream write;
  - `core` DLPack import deleter waits;
  - `core` DLPack export waits (new).

## 3. pytest (`maturin develop`, debug, editable install into `C:\Programs\Ntutil\apps\prog\lang\a3\envs\main`; `NVCOMP_ROOT=.sdk\nvcomp`)
All 10 test files in one process: **77 passed, 4 failed, 1 skipped**. Run file by file, the torch-related failures go away (see the table).

`tests/test_lease.py` passes as a normal test (it is part of the 28 passed in the run of `test_lease`, `test_foundation`, `test_dlpack_safety` and `test_compute_media_repairs`).

| Test | Class | Evidence |
| --- | --- | --- |
| `test_nv_rs_all.py::test_nvapi_platform_behavior` | (d) environment | `NvAPI_DISP_GetDisplayIdByDisplayName`: `NVAPI_EXPECTED_PHYSICAL_GPU_HANDLE (-101)`. NVAPI reports 0 connected displays on this machine; this was already known. |
| `test_stub_sync.py::TestStubSync::test_runtime_inspect_submodule_parity` | stale generated stub | `__init__.pyi` still declares `ngx.is_dlss_supported`. It needs `cargo xtask py-stubs` (root workspace), see section 5. |
| `test_nv_rs.py::test_dlpack_and_cuda_array_interface_interop`, `test_nv_rs_m3_m4.py::test_cublas_gemm_f32_dlpack_and_numpy` | Root `crates/` loader regression, not `python/`. Passes when its file runs alone. | Detailed below |
| skip: `test_foundation.py:44` | by design | Needs a host where nvCOMP cannot be found |

The cuDNN loader regression behind the two torch failures:
- What fails: `import torch` raises `OSError: [WinError 127] ... torch\lib\cudnn_cnn64_9.dll`. This only happens after nv-rs's cuDNN loader has run in the same process.
- What nv-rs does: in a file-by-file run, `test_nv_rs_all.py::test_cudnn_activation_and_softmax_match_numpy` was skipped with `cudnn: every candidate was rejected: C:\Program Files\NVIDIA\CUDNN\v9.23\bin\13.3\x64\cudnn64_9.dll: failed to preload ...cudnn_heuristic64_9.dll: safe-directories load: LoadLibraryExW failed: The specified procedure could not be found (os error 127)`. `faulthandler` also printed `Windows fatal exception: code 0xc0000139` (entry point not found).
- Cause: the new `preload:` hook in `nv-loader` / `cudnn-sys` loads the cuDNN 9 parts by full path. Some of them bind to the other cuDNN 9 copy (torch's), or the reverse, so the process ends up with mismatched `cudnn_*64_9.dll` modules.
- Consequences: (1) the cuDNN Python test now SKIPS instead of running, which hides the regression; (2) torch fails to import later in the same process.
- Owner: the ROOT gate (`crates/nv-loader`, `crates/cudnn-sys`). The previous python run did not show this.

## 4. Follow-ups
- Done: NGX test updated to the new API, without restoring the removed flags.
- Done: `typed_view` / `AnyBits` / `poisoned` moved to `buf_util.rs`; `buf_ptr_and_bytes` and `validate_dev_buf` deleted.
- Done, coarse: `__dlpack__` waits for outstanding leased writes on other streams by synchronising the context (section 1). The precise per-event wait needs the `cuda-core` API named there.
- Nothing to change for `probe_libraries` keys: nv-py passes `nv_loader::diag::probe_all()` names through unchanged, and no file under `python/` names `nvoptix`, `nvofapi` or a debug-level meaning of `NV_RS_DEBUG` (grep). `doctor()` only echoes the variable.
- Not in this gate's list and not done: `close()` / `__exit__` for the eight classes (`DPY:C20`, HANDOFF step 4).

## 5. Stale `.pyi` entries (regenerate with `cargo xtask py-stubs` after the root gate)
Found by comparing runtime introspection (member names plus `__text_signature__`) against `python/nv-py/python/nv_rs/__init__.pyi`:
- `ngx.is_dlss_supported()` (module function, `.pyi:368`): removed at runtime.
- `ngx.NgxCapabilities.supported` (`:355`), `.status_message` (`:361`), `.is_dlss_supported()` (`:362`): removed at runtime.
- `cufile.CufileHandle.read(dst, file_offset, buf_offset, len)` (`:425`): runtime is `read(dst, stream, file_offset=0, buf_offset=0, len=None)`.
- `cufile.CufileHandle.write(src, file_offset, buf_offset, len)` (`:426`): runtime is `write(src, stream, ...)`.
- `nvcomp.CompressionManager.compress(input, stream)`: runtime is `compress(input)`.
- `nvcomp.CompressionManager.decompress(compressed, stream)`: runtime is `decompress(compressed)`. The stream argument was removed on purpose; `tests/test_compute_media_repairs.py:205` checks that `stream=` is rejected.
- Not stale (introspection noise): `errors.*` showing `add_note` / `args` / `with_traceback` inherited from `BaseException`.

## 6. Linux-only code (NCCL, cuFile in `platform_ext.rs`): checked by reading, not compiled
An independent read-only review compared every Linux-gated call site with the real signatures in `crates/nccl` (`comm.rs`, `nccl.rs`), `crates/cufile` (`handle.rs`, `cufile.rs`), `cuda-core` and `buf_util`. The calls checked:
- NCCL: `all_reduce`, `broadcast`, `reduce`, `all_gather`, `reduce_scatter`, `send`, `recv`, `init_all`, `init_rank`, `async_error`, `finalize`, `abort`, `UniqueId`;
- cuFile: `DriverSession`, `FileHandle::register`, `read` (with `&mut D: DeviceSpanMut`), `write`, `deregister`, `properties` fields;
- the `Send` bounds of the `py.detach` closures, and the cfg stubs and imports.

Results:
- One definite error, now fixed: `platform_ext.rs:1040`, the private `PyContext::device_id()`.
- Uncertain: the closure at `platform_ext.rs:1338-1349` uses `?` on `cufile::Result` with no return annotation. Its error type is only fixed by the `.map_err` outside it; this should infer. Adding `-> cufile::Result<()>` would remove the doubt.
- Uncertain: the let-chain at `:1341` needs Rust 1.88 or newer (edition 2024 is set).
- Not checked: Linux clippy lints (`clippy::all = deny`). Linux compile and clippy remain unverified until a Linux host builds them.

## 7. Final counts (after all edits)
- `cargo clippy --all-targets --message-format=short -- -D warnings`: `Finished`, 0 warnings, 0 errors.
- `cargo test --lib --quiet`: `test result: ok. 19 passed; 0 failed; 5 ignored`.
- `cargo test --lib -- --ignored --test-threads=1`: `test result: ok. 5 passed; 0 failed; 0 ignored`.
- pytest, all files in one process: `4 failed, 77 passed, 1 skipped`.

`git status --short python`:
```
 M python/nv-py/src/buf_util.rs
 M python/nv-py/src/compute.rs
 M python/nv-py/src/core.rs
 M python/nv-py/src/nvcomp.rs
 M python/nv-py/src/nvjpeg.rs
 M python/nv-py/src/platform_ext.rs
 M python/nv-py/src/sparse_ai.rs
 M python/nv-py/tests/test_nv_rs_all.py
 M python/nv-py/tests/test_sys_platform_nvml.py
```

Risks:
- The build depends on the root crates as they were at about 14:25. The root gate was still editing `crates/`, so rebuild nv-py after the root gate lands.
- The cuDNN preload regression (section 3) belongs to the root gate.
- The `__dlpack__` wait is context-wide, so an export can block on unrelated work in the same context.
- The installed `_nv_rs.pyd` is a debug build of this tree.
- Peer chat: a csg-rs session sent a change list for `crates/nv-demo` addressed to the root session (Halsted). It is out of scope here and was only acknowledged.
