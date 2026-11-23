# Audit: `device/` + `python/` (nv-rs, read-only), 2026-11-23

Scope: `device/` (optix-device, optix-device-macros, optix-device-tests, cuda-tex-device) and
`python/nv-py`. Both are excluded from the root workspace. No file in the repo was modified;
no cargo/maturin/pip/tests were run. The OptiX 9.1.0 SDK headers
(`C:\ProgramData\NVIDIA Corporation\OptiX SDK 9.1.0\include`) and the cuda-oxide checkout
`~/.cargo/git/checkouts/cuda-rust_b-*/42cc04e` (the revision `device/Cargo.lock` pins) were used
as reference. Python `video.rs`/`gfx_img.rs`, `sparse_ai.rs`/`compute.rs`/`nvjpeg.rs`/`nvcomp.rs`
and `platform_ext.rs`/`sys_ext.rs`/`nvml.rs` were read in full by three independent sub-auditors.
Their items carry IDs `V*`/`G*`, `X*`/`C*` and `PE*`/`SE*`/`NV*`. I re-checked the
highest-impact ones in source (V1, C1, PE-01, NV-01) and they hold.

Severity: P0 = UB / memory unsafety / crash / wrong results; P1 = leak, swallowed error, silent
fallback, significant perf; P2 = SSOT/dedup, dead code, doc drift.
Verdict: CONFIRMED = failing path traced in code; PLAUSIBLE = needs runtime/hardware.

---

## 1) Item-by-item ledger

### 1.1 device/

| ID | Sev | Verdict | file:line | Defect | Failing scenario | Fix direction |
|---|---|---|---|---|---|---|
| D1 | P1 | CONFIRMED (build) | device/Cargo.toml:16-18; device/Cargo.lock:171-196, 500-506, 529-540; optix-device-tests/src/cases/common.rs:98,138-142,167-191; texture.rs:149,409; symbols.rs:163,274-276; coopvec.rs:247,445 | Two different `cuda-core` crates end up in the device graph. The device workspace pins `cuda-core`/`cuda-host`/`cuda-device` to `ssoj13/cuda-rust_b`. The path deps `crates/optix` and `crates/cuda-core-ext` resolve `cuda-core` from `NVIDIA/cuda-rust`; the root switched in d6c6795. The test harness creates `Arc<cuda_core::Context>`, `Stream` and `DeviceBuffer` from the fork and passes them to `optix::DeviceContext::new`, `Pipeline::launch`, `read_symbol_into`, `cuda_core_ext::Array::new` and `coop_vec::convert`, which expect NVIDIA's types. | `cargo oxide run` in optix-device-tests fails with type mismatches (E0308: "expected `cuda_core::Context`, found `cuda_core::Context`"). None of the 15 GPU cases can run, so the whole device API ships untested. | Have the device workspace use the same `cuda-core` source as the root (or patch the root crates to the fork via `[patch]`). Add a CI check that the device workspace builds. |
| D2 | P1 | PLAUSIBLE (high) | optix-device-macros/src/optix_device_macros.rs:57,88-95 | `direct_call!`/`continuation_call!` advertise `u16`/`i16` and emit `.param .b16` in the caller's `.callprototype`. LLVM NVPTX widens integer params and returns narrower than 32 bits to `.param .b32` in function declarations (`promoteScalarArgumentSize`). So a `#[device] fn __direct_callable__x(a: u16) -> u16` is declared with `.b32`. The tests only cover u32/f32/u64/f64 (calls.rs:151-166; device/optix_device_tests.ptx:9525-10126 contains no `.b16`). | The prototype does not match the callee, so ptxas rejects the indirect call or the call reads an undefined high half. The documented feature is broken. | Map 16-bit types to `.b32` with `cvt`/zero-extend in the template (or reject them in `class()`). Add a test case for u16. |
| D3 | P2 | CONFIRMED | optix-device-macros/src/optix_device_macros.rs:122-130 | Hygiene: the expansion names `::optix_device::raw::…` and `::cuda_device::ptx_asm!` as absolute paths. `optix_device` does not re-export `cuda_device`. | A crate that depends only on `optix-device` and uses `direct_call!` gets "unresolved crate `cuda_device`". The macro is also unusable inside optix-device itself. | `pub use cuda_device` from optix_device (or `#[doc(hidden)] pub mod __private`) and expand through `::optix_device::__private`. |
| D4 | P2 | CONFIRMED | optix-device/src/optix_device.rs:55,65 | Blanket `#[allow(missing_docs, clippy::all)]` on `raw` and an `#[allow(...)]` with 7 items on `sys`. `emit` (crates/nv-bindgen/src/device_asm.rs:481-524) already writes docs, `# Safety` and `// SAFETY:` for every wrapper, so `missing_docs` hides nothing on `raw`. `clippy::all` silences any future lint in generated code, against the repo rule "no `#[allow]`, use `#[expect(.., reason)]`". On `sys` (bindgen output) the allows are justified but undocumented. | A future generator regression (for example a missing `# Safety`, or an undocumented unsafe block if the template changes) passes `clippy -D warnings` unnoticed. | Drop `missing_docs`/`clippy::all` on `raw` and fix the generator if anything shows up. On `sys`, switch to `#[expect(.., reason = "bindgen output")]`. |
| D5 | P2 | CONFIRMED | optix-device/API_MAP.toml:14-15 vs :220 and src/transform.rs:503-513 | Stale header comment: it says `optixGetGASPointerFromHandle` "has no typed wrapper yet, so it is deliberately absent". But line 220 maps it to `transform::gas_pointer`, which exists (commit cbe4abc). | The doc misleads anyone maintaining the coverage gate. | Delete lines 14-15. |
| D6 | P2 | CONFIRMED | optix-device/src/payload.rs:141-155 | `word::<I>`/`set_word::<I>` have no `const { assert!(I < 32) }`, while the sibling `state::attribute` (:174), `hitobject::attribute` (:331) and `exception::detail` (:354) all assert their limits. | `payload::word::<40>()` compiles and fails only at OptiX module creation, with a less precise error. | Add `const { assert!(I < 32, ..) }`. |
| D7 | P2 | PLAUSIBLE | optix-device/src/footprint.rs:158-161 | The doc says `out`/`single_mip_level` must not point at stack memory because cuda-oxide addresses `.local` through 32-bit registers. The crate now requires 64-bit pointers everywhere (optix_device.rs:26-32, checked by case `ptr64`), so the reason no longer applies. Generic stores to local memory are valid in PTX. | Stale contract; callers write awkward global-memory scratch for no reason. | Re-verify with a stack-local `Footprint2d` under `CUDA_OXIDE_NO_SHORT_PTR` and update the doc. |
| D8 | P2 | CONFIRMED | device/.optix_device_tests.oxide-pipeline.lock (tracked, 0 bytes) | A cargo-oxide build lock is committed. `.gitignore` covers `/*.ll` and `/*.ptx` but not this file. | Noise; a stale lock may confuse the tool. | `git rm --cached`; add `/.*.oxide-pipeline.lock` and `/.oxide-artifacts` to `device/.gitignore`. |
| D9 | P2 | CONFIRMED | optix-device-tests/src/cases/transform.rs:466-473 | Observed driver behaviour is pinned as expected output: on a motion GAS the current-hit `triangle_vertices()` returns key 0 at every time, although `optix_device.h:552` documents "at the current ray time". The wrapper matches the header exactly (geometry.rs:89 → `_optix_get_triangle_vertex_data_current_hit`, no inputs), and the pipeline sets `motion_blur = true` (transform.rs:395). | Not a wrapper bug. The test asserts a probable driver defect as correct, so a driver fix will make it fail. | Keep the pin but report to NVIDIA. Consider asserting "key 0 OR interpolated" so a driver fix does not break CI. |

**Device areas verified clean, line by line against the SDK 9.1 headers:**

- **trace / hitobject.** Argument order of `trace_typed_32`, `hitobject_traverse`, `hitobject_invoke`, `make_with_traverse_data_v2` and `make_miss_v2` matches. The `reorder()` overload is `(0, 0)`. Getters are volatile where the header is.
- **footprint.** `_v2` operand order matches. The `texInfo` and `Texture2DFootprint` bit layouts match `optix_device.h`.
- **transform.rs.** All formulas match `optix_device_impl_transformations.h`: `optixMultiplyRowMatrix`, `optixInvertMatrix`, `optixGetMatrixFromSrt` (same statement order), key resolution (`fminf`/`fmaxf` NaN parity, `__fdividef`), key strides (12 / 16 floats), nlerp, the composition order of o2w (back to front) and w2o (front to back), and the empty-list shortcuts.
- **coop_vec.rs.** OP codes 0x2A21–0x2A2B, the `by_ptr` rule (sizes and element widths of both operands), 16/64-register selection (outer product: `NB<=16 && NA<=16`), and the operand order of matvec, reduce and outer-product all match. Operand counts stay within the ptx_asm! limits (≤256 inputs, ≤64 outputs; the worst is `vector_op3_64xi32` with 197).
- **The `let _ = P::WORDS` sites (payload.rs:104,113).** Legitimate: they force the const size assertions to evaluate.
- **cuda-tex-device.** `gen/gen.py` was re-run into the scratchpad and rustfmt-ed; the result is **byte-identical** to `src/cuda_tex_device.rs`, so there is no drift. The generator was reviewed in full against the PTX ISA and clang's `__clang_cuda_texture_intrinsics.h`:
  - coordinate vector arity per shape: 1d `{x}`; 2d `{x,y}`; 3d/cube `{x,y,z,z}`; a1d `{layer,x}`; a2d `{layer,x,y,y}`; acube `{layer,x,y,z}`;
  - gradient vectors for 1d/2d/3d/cube/acube;
  - sparse `|%%p0` + `selp.u16` with the operand shift (`%%` is a valid escape in ptx_asm!; ptx_asm.rs:491);
  - `tld4.{r,g,b,a}`;
  - surface coordinate order (a1d `{layer,x}`, a2d `{layer,x,y,y}`, cube mapped to a2d);
  - b8 data in `h` registers; no `v4.b64`; `suld` deliberately not `register_only`.
  
  No 1D/2D/3D/cube sibling drifted from another (they come from one table).
- **Tests.** All cases are non-vacuous. Each compares every output element against a CPU reference; missing capabilities (sparse, coop_vec) are reported as FAIL, not skipped.

### 1.2 python/nv-py — core / DLPack / errors / misc (audited directly)

| ID | Sev | Verdict | file:line | Defect | Failing scenario | Fix direction |
|---|---|---|---|---|---|---|
| P1 | P0 | CONFIRMED | core.rs:641-664,696-707 (`from_dlpack` keeps foreign strides); core.rs:177-181,324-331 (`byte_len` = numel*itemsize); buf_util.rs:274-277; no consumer outside core.rs calls `is_c_contiguous()`; tests/test_dlpack_safety.py:268-292 accepts a transposed import | `from_dlpack` accepts any strides (zero, negative, permuted) and reports `nbytes = numel*itemsize`. Every native binding then treats `(ptr, nbytes)` as a dense row-major span. Call sites: compute.rs:164-166,389-390,551,619; sparse_ai.rs:222-226,527-529,631-634,733-734,830-832,1084-1085,1180-1181; gfx_img.rs:294 (all NPP src/dst), 963,1007-1009,1061-1063,1144-1146; video.rs:470,1241-1242; platform_ext.rs:484-485,558-559,628,709-710,781-782,899-900. | (a) A transposed torch view passed to `CublasHandle.gemm_f32`, NPP or cuFFT computes on the wrong elements, with no error. (b) `torch.zeros(1,device='cuda').expand(1000)` (stride 0, 4-byte allocation) passed as an output to `curand.generate_uniform`, `nccl.all_reduce` recv or an NPP dst writes 4000 bytes: an out-of-bounds device write that corrupts neighbouring tensors. | In `from_dlpack`, reject non-C-contiguous strides (treat size-1 dimensions as wildcards) unless a caller opts in. Store the true span extent. Make `buf_ptr_and_bytes` return an error for non-contiguous buffers so every binding inherits the check. |
| P2 | P1 | CONFIRMED | core.rs:548-553 | The producer is called as `obj.__dlpack__(stream_ptr)`, with the stream positional. The Array API / DLPack ≥0.8 signature is `__dlpack__(self, /, *, stream=None, max_version=None, ...)`: `stream` is keyword-only (CuPy ≥13, PyTorch 2.x, JAX). The test producer (test_dlpack_safety.py:113) takes it positionally, which masks the problem. | `DeviceBuffer.from_dlpack(torch_tensor, stream=s)` raises `TypeError: __dlpack__() takes 1 positional argument but 2 were given`. The stream-ordered import path does not work with any modern producer. | Call `call_method("__dlpack__", (), Some(kwargs{stream}))` and fall back to the legacy form on TypeError. |
| P3 | P1 | PLAUSIBLE | core.rs:813-838 | `to_bytes(stream)` synchronizes and copies on the **caller's** stream and ignores `self.stream`, the producer stream recorded at allocation or upload. | The buffer is filled by work on stream A, and the user calls `buf.to_bytes(B)`: a host read that races the pending writes on A. | Wait on `self.stream` (event or synchronize) before the copy, or default to `self.stream`. |
| P4 | P1 | CONFIRMED | core.rs:271-317,343-350 (`from_owned_cuda` → `stream: None`), used at nvjpeg.rs:309, video.rs:831,1361, nvcomp.rs:214 | `from_owned_cuda` records no producer stream, so `__dlpack__` / `__cuda_array_interface__` skip synchronization entirely. nvjpeg.rs:309 wraps a decode that is still enqueued (C5). | `torch.from_dlpack(decoder.decode(jpg))` consumed on another stream reads half-decoded pixels. | Remove the stream-less constructor; require a stream (or an event) in every result wrapper. |
| P5 | P1 | PLAUSIBLE | core.rs:618-707 | `from_dlpack` never compares the tensor's `device_id` with the device of the `stream`/context it will be used with. No binding checks `buf.device_id` either (V13). | A tensor on GPU 1 is fed to a cuBLAS handle on GPU 0: illegal address without peer access, or silent cross-device traffic. The context is then poisoned. | Validate `device_id` at import when a stream is given, and in `buf_ptr_and_bytes` against the target context. |
| P6 | P2 | CONFIRMED | core.rs:288-301 | `from_owned_cuda_with_stream` uses `panic!`/`assert_eq!` that can be reached from Python (they become `PanicException`, which derives from BaseException). The sub-auditors confirmed the current callers compute consistent sizes, so it is latent. | A future binding with an off-by-one shape crashes the request with a non-`Exception` error. | Return `PyResult`. |
| P7 | P1 | CONFIRMED | diag.rs:305-308; python/nv_rs/__init__.py:41-46; nv-loader/src/nv_loader.rs:293-308 | `add_dll_directory` only appends to `EXTRA_SEARCH_DIRS`. The loader caches failures in `OnceLock<Result<..>>` (AGENTS.md trap), so calling it after a library has failed to load once does nothing, and it neither warns nor returns a status. | `import nv_rs; nv_rs.nvrtc.version()` fails (DLL missing); then `nv_rs.add_dll_directory(path)` followed by a retry fails the same way, silently. | Return or raise if any spec has already cached a failure, or let the loader retry on a new search dir. |
| P8 | P1 | CONFIRMED | nvrtc.rs:189 | `prog.ptx_string().unwrap_or_default()` swallows every `nvrtcGetPTX` failure (crates/nvrtc/src/program.rs:160-163 propagates errors) and returns `ptx == ""`. | A non-dlto compile whose PTX retrieval fails gives `CompileResult.ptx == ""`, and the caller then loads an empty module. | Make `ptx` an `Option<String>`: `None` only for dlto/LTO-IR output, otherwise propagate the error. |
| P9 | P2 | CONFIRMED | errors.rs:230 | `#[allow(dead_code)]` on `ErrorKind` is stale: every variant is used (LibraryNotFound 21, Cuda 43, InvalidArgument 119, Compile 3, General 2, Value 1, Runtime 2 sites). | Hides future dead variants. | Remove. |
| P10 | P2 | CONFIRMED | dlpack.rs:14-15 | `#[allow(dead_code)] DL_CPU` is used only by `#[cfg(test)]` (core.rs:995). | — | `#[cfg(test)]` or `#[expect(dead_code, reason)]`. |
| P11 | P2 | CONFIRMED | pyproject.toml:64,70 vs Cargo.toml:15 | `requires-python = ">=3.10"` and a 3.10 classifier, but PyO3 is built with `abi3-py311`, so the wheel is tagged cp311-abi3. | pip on 3.10 refuses the wheel; an sdist build on 3.10 fails. | Set `>=3.11`, or build with `abi3-py310`. |
| P12 | P2 | CONFIRMED | pyproject.toml:63 | The description advertises cuTENSOR, nvTIFF, nvImageCodec and NVTX bindings. None of them exist in nv-py (grep: no module or class; "nvtx" only as a CUPTI callback-domain string, sys_ext.rs:72). | Users install the package expecting features that are absent. | Fix the description, or derive it from the registered submodules (py-stubs SSOT). |
| P13 | P2 | CONFIRMED | core.rs:412-451 | `from_buffer` duplicates the typestr→dtype table, which already exists as `DTYPE_TABLE.numpy_typestr` (dlpack.rs:176). It also infers nothing for big-endian, complex and bf16. | Two tables drift apart. | Use `DTYPE_TABLE` lookup by `numpy_typestr`. |
| P14 | P2 | CONFIRMED | core.rs:712-716 | `__dlpack__` accepts neither `max_version`, `dl_device` nor `copy`, and never returns a versioned capsule. torch/numpy retry without `max_version` on TypeError, so it works, but every import pays an exception round-trip and there is no read-only flag. | Every `torch.from_dlpack(buf)` raises and catches a TypeError internally. | Accept the kwargs; emit `dltensor_versioned` when `max_version >= (1, 0)`. |
| P15 | P2 | CONFIRMED | tests/test_nv_rs_all.py:7-23; tests/test_nv_rs_m3_m4.py:7-21 | `_skip_if_missing_runtime` turns any exception whose message contains "unsupported", "not supported" or "unavailable" into a skip. Bindings use those words for argument errors (macros.rs:175 "unsupported <expected>: ..."; dlpack.rs:339 "unsupported dtype"). | A regression that raises `NvInvalidArgumentError("unsupported dtype ...")` makes the test skip instead of fail. | Skip only on `NvLibraryNotFoundError` or an explicit capability error type. |
| P16 | P2 | CONFIRMED | tests/test_dlpack_safety.py:303-331; tests/test_nv_rs_all.py:438-468,385-415 | Near-vacuous tests. `test_py4_dlpack_stream_synchronization` asserts only that the capsules are non-None and never checks that synchronization happened. The CUPTI activity test enables `memcpy` but only checks `isinstance(records, list)`, so an empty list passes. The NGX test asserts types only. | Regressions in DLPack stream sync or CUPTI record capture pass. | Assert observable effects: at least one memcpy record; a consumer reading data written by a pending kernel. |
| P17 | P2 | CONFIRMED | python/nv_rs/__init__.pyi (e.g. :9-16, `class diag:` with `@staticmethod` functions) | The stub models submodules as classes. `import nv_rs.core` / `from nv_rs.core import DeviceBuffer` fail type-checking (no such module), even though at runtime they are real modules in `sys.modules` (macros.rs:156-158). | mypy/pyright report "module not found" for documented import forms. | Generate `nv_rs/<mod>.pyi` files (or `ModuleType` stubs) from xtask `py_stubs.rs`. |

**Checked and clean.**

- **DLPack capsule protocol.** The names `dltensor`/`used_dltensor` are correct. The consumer renames the capsule before taking ownership (core.rs:580-587). `DlManagedGuard` calls the producer's deleter exactly once on every early exit after the rename, and the read-after-free path is covered by a test (PY-1). The unconsumed capsule destructor checks the name (dlpack.rs:129-155). The exporter frees `managed` if `PyCapsule_New` fails (core.rs:737-741). `byte_offset` is applied with overflow checking. ndim, shape-null and data-null are validated. The CAI v3 dict has `(ptr, False)`, byte strides with overflow checks, and a stream that is never 0 because PyStream is always a created stream.
- **GIL in core/nvrtc.** Sync, upload, download and compile all run in `py.detach`.
- **diag.** TraceGuard enter/exit restores the level and is re-entrant-safe.

### 1.3 python/nv-py — video.rs / gfx_img.rs (sub-auditor A)

| ID | Sev | Verdict | file:line | Defect / scenario | Fix |
|---|---|---|---|---|---|
| V1 | P0 | CONFIRMED (re-verified) | video.rs:414,432-438,354-359; crates/nvenc/src/nvenc.rs:713-727 | The `NvencEncoder` session gets the raw `CUstream` through `set_io_streams`, but neither `EncoderInner` nor `EncodeSession` keeps the `Arc<Stream>`. Once the Python `Stream` is collected, the handle goes back to cuda-core's pool or is destroyed while NVENC keeps using it. Encoder I/O then lands on a stranger's stream, or the driver uses a destroyed handle. | Keep the `Arc<Stream>` in `EncodeSession` (wrapper) or `EncoderInner`. |
| V2 | P0 | PLAUSIBLE | video.rs:420-431,526-553 | `frameIntervalP` (B-frames) and lookahead are not forced off. `NV_ENC_ERR_NEED_MORE_INPUT` becomes an error, but the frame stays queued inside NVENC while the binding unmaps and unregisters it. NVENC later reads unregistered memory. | Set `frameIntervalP=1` and lookahead off, or implement a real output queue that keeps resources registered. |
| V3 | P0 | PLAUSIBLE (high) | video.rs:1120 | The NV12 UV destination offset uses `strides.planes[0].1` as **bytes**. The NVOF SDK uses it as **rows** (`NvOFCuda::UploadData`). The UV plane is written into the Y plane, so every NV12 optical flow result is wrong. | `planes[0].1 * planes[0].0`; move this into the nvofa wrapper. |
| V4 | P0 | PLAUSIBLE | video.rs:470,515-554 | NVENC reads `frame` on its own stream without waiting for the frame's producer stream. | Make the encoder stream wait on an event recorded on the producer stream. |
| V5 | P0 | PLAUSIBLE | video.rs:1293-1310 | NVOFA copies of prev/curr have no producer-stream wait. | Same as V4. |
| G1 | P0 | PLAUSIBLE | gfx_img.rs:388-402,434-448,480-494,539-595,716-734,802-834,870-887,923-940,976-983,1024-1037,1078-1091,1160-1187 | Every NPP op is asynchronous on the NPP context stream. It does not wait for the inputs' producer streams, keeps no owner alive and sets no stream on `dst`. `del src` right after the call frees memory under the queued kernel; `dst.__dlpack__()` does not synchronize. | Record an event on the NPP stream, store it on dst, wait on src producers, keep owners alive until completion. |
| V6 | P1 | CONFIRMED | video.rs:526-536,551 | Register, map, unmap and unregister the resource on every frame. | Cache registrations by `(ptr, len)`. |
| V7 | P1 | PLAUSIBLE | video.rs:413,420-431 | The `yuv420_10bit` and `yuv444*` formats are accepted, but bit depth, chroma format and profile are never configured: the explicit request is silently downgraded or fails. | Configure them or reject the format. |
| V8 | P1 | CONFIRMED (stub) | video.rs:570-593 | `flush()` always returns `[]`, despite its doc ("remaining packets"). | Drain after EOS. |
| V9 | P1 | CONFIRMED | video.rs:165-168,888,811-815 | `output_format="yuv444"/"yuv444_16bit"` is accepted at construction, but every `on_display` errors. | Reject the format in `new`, or implement it. |
| V10 | P1 | PLAUSIBLE | video.rs:772 | A sequence change recreates the decoder without a flush or reconfigure. | Flush, or `cuvidReconfigureDecoder`. |
| V11 | P1 | CONFIRMED | video.rs:596-600,622-625,995-999,1006-1027,1370-1374,1401-1404 | `close`/getters/`__repr__` take the mutex while holding the GIL, so the interpreter stalls for the length of a concurrent encode or decode. `close()` also destroys native resources under the GIL. | Lock and drop inside `py.detach`; use atomics for getters. |
| V12 | P1 | CONFIRMED | video.rs:260,281 | Any `Error::Status` is treated as "codec unsupported", so OOM or driver errors become `False`. | Match only the unsupported status. |
| V13 | P1 | PLAUSIBLE | video.rs:470,1241-1242; gfx_img.rs:294,963,1007-1009,1061-1063,1144-1146 | `buf.device_id` is never checked against the session or context device. | Validate it. |
| G2 | P1 | CONFIRMED | gfx_img.rs:1238-1254,1296,1360,1398 | NGX `supported`/`is_dlss_supported` are derived from `RequiredExtensions` success (a static list, no GPU probe), and all 10 features get the same flag. | Rename to `sdk_available`, or do a real probe. |
| G3 | P1 | CONFIRMED (no check) | gfx_img.rs:781-806,529-530,704-707 | Overlapping or identical src/dst is not rejected (the Rust wrapper prevents it with borrows), so in-place Gaussian or resize gives wrong pixels. | Check for overlap. |
| V14 | P2 | CONFIRMED | video.rs:597,624,996,1008-1021,1026,1371,1403 | A poisoned mutex makes `close` a silent no-op, `is_closed` report True through `unwrap_or(true)`, and width become 0. | Use `into_inner`, or raise. |
| V15 | P2 | CONFIRMED | video.rs:947-948,977-978 | Frames from a packet that failed to parse leak into the next `decode()` result. | `take` the frames before propagating the error. |
| V16 | P2 | CONFIRMED | video.rs:672 | `bit_depth.saturating_sub(8)` silently turns <8 into 8-bit. | Reject. |
| V17 | P2 | CONFIRMED | video.rs:936 | Extra packet copy (`to_vec`) on every call. | Borrow the bytes. |
| V18 | P2 | CONFIRMED | video.rs:241,1293-1347 | 5 synchronizes and 1 allocation per NVOFA `execute`. | One sync at the end. |
| V19 | P2 | CONFIRMED | video.rs:212-243,796-816 | Duplicates nvdec `copy_nv12_to_host`/`copy_p016_to_host` and the nvofa `upload_plane0`/`download_plane0` helpers. | Add D2D helpers to the wrappers. |
| V20 | P2 | CONFIRMED | video.rs:1119,1125,1127,1133,1248-1250 | `saturating_*` in pointer arithmetic hides overflow; a dead arm carries a misleading message. | `checked_*`. |
| V21 | P2 | CONFIRMED/PLAUSIBLE | video.rs:46,284-306 | `CallbackPanic` is mapped to Cuda; one missing caps query (old driver) fails all of `codec_caps`. | Map per field. |
| V22 | P2 | PLAUSIBLE | video.rs:1130,1243-1247,479-488 | Odd width or height is not rejected for NV12. | Validate. |
| G4 | P2 | CONFIRMED | gfx_img.rs:238-271,371-1199,795-799 | All NPP ops are reimplemented instead of calling `crates/npp`. `interp_to_sys` relies on enum discriminant order (magic 0..3, `_ => LANCZOS`). | Raw-pointer `ImageView` constructors in npp; public `to_sys`. |
| G5 | P2 | CONFIRMED | gfx_img.rs:1248-1253 | The NgxCapabilities constructor stores the error in `status_message`; `required_extensions()` then silently returns empty lists. | Raise. |
| G6 | P2 | CONFIRMED (stub) | gfx_img.rs:59-78 | `OptixModule` cannot be used from Python (no pipeline or program-group API). `_keep_alive` is read in `__repr__` to dodge the `dead_code` lint. | Expose the pipeline, or drop the class; `#[expect]`. |
| G7 | P2 | CONFIRMED | gfx_img.rs:35-41 | Compile vs Cuda error kind is chosen by whether the log is empty, a heuristic instead of the error type. | Match on the error variant. |

### 1.4 python/nv-py — sparse_ai.rs / compute.rs / nvjpeg.rs / nvcomp.rs (sub-auditor B)

| ID | Sev | Verdict | file:line | Defect / scenario | Fix |
|---|---|---|---|---|---|
| X1 | P0 | CONFIRMED (static) | compute.rs:187,424,437,449,572,648; sparse_ai.rs:387,585,694,785,882,1126,1220 | Async GEMM, FFT, cuRAND, SpMV, getrf/getrs/potrf/potrs and cuDNN launches use raw pointers and return while still queued. The buffer owner is not retained (cuda-core requires raw users to retain storage; drop runs `free_async` on the allocation stream). `del a` after `h.gemm_f32(a,..)` on another stream is a GPU use-after-free; for a DLPack import the torch caching allocator reuses the block immediately. | Retain `Arc<Backing>` or a lease until completion (`retain_until_complete`, or an event plus deferred drop). |
| X2 | P0 | PLAUSIBLE | same sites + nvjpeg.rs:386-398,433; nvcomp.rs:136-155 | `PyDeviceBuffer.stream` is never read; there is no producer→library stream join, so kernels can read unwritten inputs. | `lib_stream.join(producer)`. |
| C1 | P0 | CONFIRMED (re-verified) | compute.rs:184; crates/cublas/src/handle.rs:306-322 | `set_stream` returns early when the stream is cached and **skips `bind_to_thread()`**. `cublasSgemm_v2` then runs on whatever context is current on the OS thread. `sparse_ai.rs:88` (`cusparse.version()` → `Context::new(0)`) or a call from another Python thread switches that context. On multi-GPU, or with a handle used from another thread: errors, garbage or illegal address. | Bind the context explicitly before the raw call (as cusparse/cusolver bindings do at sparse_ai.rs:302,571), or remove the short-circuit from the wrapper. |
| C4 | P0 | PLAUSIBLE | nvjpeg.rs:257,285-288 | One `JpegState` is shared across per-call `stream` overrides with no ordering, so consecutive decodes on s1 and s2 clobber the state's internal buffers. | One state per stream, or join with the previous stream. |
| C5 | P0 | CONFIRMED | nvjpeg.rs:309 | The decode result is wrapped with `stream=None` while still enqueued (see P4). | `from_owned_cuda_with_stream(.., Some(s))`. |
| X3 | P0 | CONFIRMED | sparse_ai.rs:222-226,527-529,631-634,733-734,830-832,1084-1085,1180-1181; compute.rs:164-166,389-390,551,619 | Instance of P1: strides are ignored, and **the dtype string is not checked anywhere**. An i64 tensor passed as CSR indices, or f16 passed to `gemm_f32`, is reinterpreted. The cuRAND outputs at :551 and :619 are writes, so a stride-0 import gives an out-of-bounds write. | Check dtype per argument; contiguity per P1. |
| C2 | P1 | CONFIRMED | nvcomp.rs:135,188; crates/nvcomp/src/manager.rs:148,281 | `same_stream` uses `Arc::ptr_eq`, so any `stream=` other than the same object always errors. The documented feature cannot work. | Compare `cu_stream()` and the context, or drop the parameter. |
| C3 | P1 | CONFIRMED | nvjpeg.rs:426-431 | Per-call `quality`/`subsampling` permanently mutate the encoder defaults. | Restore the defaults or apply them every call. |
| C6 | P1 | CONFIRMED | sparse_ai.rs:305-400 | Descriptors are recreated and `SpMV_preprocess` runs on every call, then the result is discarded. | Cache, or drop the preprocess step. |
| C7 | P1 | CONFIRMED | nvcomp.rs:29-33 | Wildcard `_ => Cuda` maps Budget→Cuda and missing symbol→Cuda. | Exhaustive match. |
| C8 | P2 | CONFIRMED | sparse_ai.rs:254 | Tautological `validate_spmv_dims`: it is passed computed lengths, not the actual ones. | Pass actual lengths. |
| C9 | P2 | CONFIRMED | sparse_ai.rs:1058 | Default `coef=0` makes `clipped_relu` return 0 and `elu` behave as ReLU. | Per-mode default, or require `coef`. |
| C10 | P2 | PLAUSIBLE | nvjpeg.rs:39-42,203-210 | The `hardware` backend is wired to the legacy decode API, which does not support it. | Reject it or route to the batched API. |
| C11 | P2 | CONFIRMED | nvjpeg.rs:200-217,335-357; nvcomp.rs:110-125 | Library load, handle creation and pinned allocations happen while holding the GIL. | `py.detach`. |
| C12 | P2 | CONFIRMED | compute.rs:394,229,481; sparse_ai.rs:416,909,1248 | Mutex locked under the GIL in `__repr__`/plan getters. | Immutable metadata, or `try_lock`. |
| C13 | P2 | CONFIRMED | nvjpeg.rs:132-135,160-168 | `getattr("shape").ok()` swallows errors and can mix an explicit width with an inferred height. | Infer both or neither; propagate errors. |
| C14 | P2 | CONFIRMED | sparse_ai.rs:368,582,782,88; compute.rs:346 | `usize::MAX` byte budgets (limits disabled); device 0 hard-coded in `version()`, which also rebinds the thread's context. | Expose the budget; no `Context::new` in `version()`. |
| C15 | P2 | CONFIRMED | sparse_ai.rs:298-400,516-907,1063-1245; compute.rs:290-334,415-466 | The bindings reimplement the wrapper bodies (cusparse spmv, cusolver Dn ops, cudnn activation/softmax, cufft exec; the FFT path also skips `ensure_workspace`). | Span-based wrapper APIs. |
| C16 | P2 | CONFIRMED | nvcomp.rs:191-193 | A leftover thought-stream comment. | Rewrite it. |
| C17 | P2 | CONFIRMED | compute.rs:92-176 | GEMM is column-major while `shape` metadata is row-major, so NumPy users get a transposed result. | Document it, or derive the call from the shape. |
| C18 | P2 | CONFIRMED | compute.rs:388 | `direction` is silently ignored for r2c/c2r. | Reject mismatches. |
| C19 | P2 | CONFIRMED | nvcomp.rs:123 | `format_name` is not trimmed. | Take the name from `FormatKind`. |
| C20 | P2 | CONFIRMED | all 4 files | No `close`/context manager on any handle, plan or manager; release only through GC. | Add `close`/`__exit__`. |

### 1.5 python/nv-py — platform_ext.rs / sys_ext.rs / nvml.rs (sub-auditor C)

| ID | Sev | Verdict | file:line | Defect / scenario | Fix |
|---|---|---|---|---|---|
| PE-01 | P0 | CONFIRMED (re-verified) | platform_ext.rs:94-128 (callers 501,572,642,726,798,913); groups 184-214 | `stage_and_copy_recv` enqueues `cuMemcpyDtoDAsync(recv←tmp)` and the `free_async(tmp)` immediately after the collective call. Inside `nccl.group_start()/group_end()` the collective is deferred until `group_end`, so the copy and the free run **before** it. recv gets garbage and NCCL then writes into freed memory. | Pass recv as a span (`DeviceSpanMut`) to the wrapper; refuse inside a group, or keep tmp alive until `group_end`. |
| PE-02 | P0 | PLAUSIBLE | platform_ext.rs:483-485,509,558,628,709,781,899; crates/nccl/src/comm.rs:521-522 | NCCL raw pointers carry no lease, so a send/recv buffer deleted right after the call is a GPU use-after-free if the streams differ. | Lease in the nccl wrapper. |
| SE-01 | P0 | PLAUSIBLE (Windows) | sys_ext.rs:499-502,506-528; cuda-core-ext/src/host.rs:147-157,118 | `ManagedBuffer.to_bytes`/`write_bytes`/zeroed touch managed memory from the host with no sync. On Windows (`concurrentManagedAccess=0`), doing so while any kernel is running is an access violation, i.e. a process crash. `as_slice` is a safe fn with an uncheckable precondition. | Make it `unsafe`, or take a stream and sync; check the attribute. |
| SE-02 | P0 | PLAUSIBLE | sys_ext.rs:609-615,640-646,550-556; host.rs:79-87 | `HostRegistration.close()`/GC unregisters while kernels may still read `device_ptr`. | Sync or require a stream in close. |
| PE-03 | P1 | PLAUSIBLE | platform_ext.rs:1202-1306; cufile handle.rs:742-861 | Synchronous cuFile read/write never synchronizes the buffer's (stream-ordered) allocation stream. | `stream` parameter plus sync, or expose `read_async`. |
| PE-04 | P1 | CONFIRMED | platform_ext.rs:111-126; comm.rs:483 | Every recv collective does an extra allocation and an extra D2D copy. | As PE-01. |
| SE-03 | P1 | CONFIRMED | sys_ext.rs:301-306,369-372 | The CUPTI subscriber event `Vec` grows without bound. | Bounded ring plus a drop counter. |
| SE-04 | P1 | PLAUSIBLE (deadlock) | sys_ext.rs:36-46 (callers 174,295,468,584,698),308 | `Context::new` runs under the GIL while the CUPTI callback calls `Python::attach` on a driver thread: GIL ↔ driver-lock inversion; also on GC drop against `cuptiUnsubscribe`. | `Context::new` in detach; callback pushes to a queue instead of taking the GIL. |
| SE-05 | P1 | CONFIRMED | sys_ext.rs:262-268,391-397,640-646; cupti activity.rs:407-430; subscriber.rs:152-167 | Explicit `close()` always returns Ok: disable, flush, unsubscribe and unregister errors go only to `record_err`. | Fallible `close`. |
| NV-01 | P1 | CONFIRMED (re-verified) | nvml.rs:139-141,152-156 | `.ok()` turns every NVML error (GPU lost, driver) into `None`, after `map_nvml_err` has already recorded a diag error, so each `summary()` adds phantom errors. | `None` only for NOT_SUPPORTED; otherwise propagate. |
| SE-06 | P1 | PLAUSIBLE | sys_ext.rs:1149-1176; nv-wgpu buffer.rs:261,362 | No wgpu error scope: a validation error (size > `max_buffer_size`, unchecked) goes to the default uncaptured handler, which panics → `PanicException`. | Error scope plus a limits check. |
| PE-05 | P1 | PLAUSIBLE | platform_ext.rs:996-1136 vs 1138-1375 | `CufileDriver.close()` while `CufileHandle`s are alive means deregistering after driver close (UB). | The handle holds the driver session. |
| SE-07 | P2 | CONFIRMED | sys_ext.rs:916-968,972-988 | Probe builds a full device and context per adapter; no short-circuit. | Light probe. |
| NV-02 | P2 | CONFIRMED | nvml.rs:26-158 | No `py.detach` on NVML calls (`nvmlInit` can take ~1 s). | detach. |
| SE-08 | P2 | CONFIRMED | cupti activity.rs:170; sys_ext.rs:162-269 | Global activity state: several `CuptiActivity` instances steal each other's records, `close` disables kinds for all, and the `context` parameter does not filter anything. | Singleton or refuse a second instance. |
| SE-09 | P2 | CONFIRMED | sys_ext.rs:721-793,764 | `ExternalMemoryDesc` is a validator with no consumer; `let _ = size_of::<MemoryHandle>()` is an empty check; there is no per-OS rejection. | Real import path via `TryFrom`. |
| SE-10 | P2 | CONFIRMED (latent UB) | sys_ext.rs:585-599 | `&'static mut` derived from a `Box` that is then moved (Stacked Borrows violation). The registration covers a private copy with no write API. | Use the raw ptr from `Box::into_raw`. |
| SE-11 | P2 | CONFIRMED | sys_ext.rs:890-891,1163; nv-wgpu buffer.rs:60-67 | `map_read`/`map_write` are parsed but always rejected (dead options); validation is duplicated. | Remove. |
| SE-12 | P2 | CONFIRMED | sys_ext.rs:916-968 vs 1069-1108,1102-1105 | Duplicated adapter loop and `block_on`; errors from all but the last adapter are lost. | `open_first` in nv-wgpu; aggregate errors. |
| SE-13 | P2 | CONFIRMED | sys_ext.rs:1189-1251 | No wgpu-side objects; `SharedTexture` not exported; no `close` on the interop classes. | — |
| PE-06 | P2 | CONFIRMED | platform_ext.rs:82-91 vs 484; 475-482 (×6); 9× lock/finalized boilerplate | Send/recv ownership asymmetry; duplicated `check_elem_span`. | `with_comm` helper. |
| PE-07 | P2 | CONFIRMED | platform_ext.rs:1387; sys_ext.rs:55,842; nvml.rs:14 | NVAPI status maps to `NvCudaError`; wildcard error matches. | Exhaustive match. |
| PE-08 | P2 | CONFIRMED | platform_ext.rs:1164-1197; cufile handle.rs:644-651 | The wrapper supports `PosixFallback` off Linux, but the binding disables cuFile on Windows; async I/O, `RegisteredBuffer` and `RegisteredStream` are not exposed. | — |
| X-01 | P2 | CONFIRMED | platform_ext (errors on poison) vs sys_ext (22× `into_inner`) | Inconsistent mutex-poison policy. | Pick one. |
| X-02 | P2 | PLAUSIBLE | NcclComm/CuptiActivity Drop; sys_ext.rs:499 | Heavy drop or copy under the GIL. | detach. |

`let _ =` / `.ok()` / `unwrap_or*` classification in platform_ext.rs, sub-auditor C, 37 sites.
Classes: (a) legitimate cleanup-on-drop, (b) hides a real operational failure, (c) fine.

- **Unused-parameter suppression in cfg stubs, each followed by an explicit `Err`: (c).**
  152, 179, 195, 211, 270, 315, 380, 427, 450, 522, 593, 678, 747, 828, 872, 929, 991, 1029, 1044, 1092, 1115, 1195, 1249, 1303, 1327, 1437, 1452, 1485, 1502, 1520, 1551, 1603, 1676.
- **`__exit__` arguments: (c).** 1132, 1371.
- **`len.unwrap_or_else` defaults (the wrapper validates bounds): (c).** 1224, 1278.
- **The implicit drop of `tmp` in `stage_and_copy_recv`: (a),** but it is the root of PE-01.

None of the explicit sites hides a failure; the real swallowing in this file is implicit (PE-01 ordering, SE-05).

- **sys_ext.rs.**
  - Mutex-poison recovery: (c). 196, 209, 223, 246, 264, 340, 359, 370, 377, 383, 393, 489, 495, 500, 507, 531, 612, 621, 627, 642.
  - Inside the FFI callback: (c). 304, 310.
  - Empty check (SE-09): 764.
  - Loses errors from other adapters (SE-12): 1105.
  - `close()` teardown: (b), see SE-05. 265, 394, 643.
- **nvml.rs: all (b), see NV-01.** 139-141, 152-156.

---

## 2) Files inspected

| File | Lines | Read |
|---|---|---|
| device/Cargo.toml, rust-toolchain.toml, .cargo/config.toml, .gitignore, Cargo.lock (targeted) | 29/3/6/3/826 | full (lock: targeted) |
| device/optix-device/Cargo.toml, build.rs | 24/59 | full |
| device/optix-device/src/optix_device.rs | 75 | full |
| device/optix-device/src/payload.rs | 118 | full |
| device/optix-device/src/trace.rs | 97 | full |
| device/optix-device/src/exception.rs | 90 | full |
| device/optix-device/src/state.rs | 281 | full |
| device/optix-device/src/footprint.rs | 228 | full |
| device/optix-device/src/geometry.rs | 318 | full |
| device/optix-device/src/hitobject.rs | 564 | full |
| device/optix-device/src/transform.rs | 747 | full |
| device/optix-device/src/coop_vec.rs | 781 | full |
| device/optix-device/API_MAP.toml | 292 | head + targeted grep |
| device/optix-device-macros/Cargo.toml, src/optix_device_macros.rs | 20/149 | full |
| device/optix-device-tests/Cargo.toml, .cargo/cuda-oxide.toml | 25/9 | full |
| device/optix-device-tests/src/optix_device_tests.rs, cases/mod.rs, common.rs | 46/43/230 | full |
| cases/local.rs, const_index.rs, symbols.rs, footprint.rs, texture.rs, exception.rs, shade.rs, calls.rs, intersect.rs, coopvec.rs, hitobject.rs | 58/90/133/127/449/151/164/252/231/298/302 | full |
| cases/geometry.rs, transform.rs | 385/490 | header + all assertion/check sites (structure skimmed) |
| device/cuda-tex-device/Cargo.toml, gen/gen.py, gen/head.rs | 17/388/132 | full |
| device/cuda-tex-device/src/cuda_tex_device.rs | 6351 | verified byte-identical to regenerated output; sampled sections |
| python/nv-py/Cargo.toml, pyproject.toml, .gitignore | 55/36/9 | full |
| python/nv-py/src/lib.rs, macros.rs, errors.rs, buf_util.rs, dlpack.rs, core.rs | 46/50/68/41/431/1062 | full |
| python/nv-py/src/nvrtc.rs, diag.rs | 231/257 | full |
| python/nv-py/src/video.rs, gfx_img.rs | 1459/1478 | full (sub-auditor A); key lines re-checked |
| python/nv-py/src/sparse_ai.rs, compute.rs, nvjpeg.rs, nvcomp.rs | 1276/704/457/229 | full (sub-auditor B); key lines re-checked |
| python/nv-py/src/platform_ext.rs, sys_ext.rs, nvml.rs | 1721/1275/167 | full (sub-auditor C); key lines re-checked |
| python/nv-py/python/nv_rs/__init__.py | 106 | full |
| python/nv-py/python/nv_rs/__init__.pyi | 551 | targeted (signatures of core/diag/nvrtc/platform modules) |
| python/nv-py/tests/test_dlpack_safety.py | 331 | targeted (helpers, PY-3, PY-4) |
| python/nv-py/tests/test_stub_sync.py | 427 | targeted |
| python/nv-py/tests/test_nv_rs_all.py, test_nv_rs_m3_m4.py, test_nv_rs.py | 655/266/126 | targeted (skip helpers, assertions) |
| Reference (contract only): crates/nv-bindgen/src/device_asm.rs (722, full); crates/cublas/src/handle.rs:305-330; crates/nvenc/src/nvenc.rs:713-727; crates/nvrtc/src/program.rs:160-175; crates/nv-loader/src/nv_loader.rs:290-308; OptiX SDK 9.1 internal/optix_device_impl*.h; cuda-oxide cuda-macros/src/ptx_asm.rs and mir-importer asm.rs | — | targeted |

## 3) Files changed

None. Only the scratchpad was written: this report, plus `scratchpad/texgen/out.rs` for the
generator-drift check.

## 4) Static / invariant verification performed

- **OptiX header parity.** Every intrinsic call in trace, hitobject, footprint and geometry (spot) was compared argument by argument with `optix_device_impl.h` (SDK 9.1.0).
  - transform.rs was compared formula by formula with `optix_device_impl_transformations.h`.
  - coop_vec.rs was compared on dispatch conditions and operand order with `optix_device_impl_coop_vec.h`.
  - `.sdk/optix` and `ProgramData/.../OptiX SDK 9.1.0` report the same `OPTIX_VERSION 90100`, but their internal headers differ byte-wise (risk R3).
- **cuda-tex-device drift.** Ran `python -I gen/gen.py <scratch>/out.rs gen/head.rs` plus `rustfmt --edition 2024` (no repo writes); the output is byte-identical to `src/cuda_tex_device.rs`.
- **ptx_asm! contract.** Checked against the cuda-oxide source at the pinned revision: input limit 256 and output limit 64 (the largest OptiX wrapper fits); `%%` escape supported; `register_only` = no side effect, else side effect + convergent; no implicit memory clobber.
- **Dependency graph.** `device/Cargo.lock` was inspected to prove the dual `cuda-core` (D1).
- **Existing PTX artifact.** `device/optix_device_tests.ptx` was inspected for callable prototypes and `.b16` params (D2).
- **ErrorKind usage.** Counted per variant (P9). Grepped for `is_c_contiguous` consumers (P1) and for the libraries claimed in pyproject (P12).
- **Re-verification of sub-auditor items.**
  - V1: no `Arc<Stream>` kept; the wrapper does not keep it either.
  - C1: `set_stream` early return before `bind_to_thread`.
  - PE-01: memcpy + free enqueued right after a possibly deferred group collective.
  - NV-01: `.ok()` after an error-recording mapper.

## 5) Risks / open seam questions for other slices

1. **crates/nvenc (V1).** `EncodeSession::set_io_streams` should own `Arc<Stream>`s. Any caller, including nv-demo's MP4 export, can leave NVENC with a dangling or reused stream. A typed `NeedMoreInput` is needed (V2), and `SessionConfig` lacks bit depth, chroma and profile (V7).
2. **crates/cublas (C1).** The `set_stream` cache skips `bind_to_thread()`, so every raw `handle.raw()` user (nv-demo or others) depends on the current context being right. Decide whether the wrapper must always bind.
3. **Lease API for raw pointers.** No wrapper (nccl, nvjpeg ImagePlanes, NPP, cuBLAS/cuFFT raw paths) retains storage until completion, and cuda-core's `completion` is `pub(crate)` (X1, PE-02, G1). A public `retain_until_complete(span, &Stream)` is the SSOT fix for all bindings.
4. **crates/nvofa (V3).** `StrideInfo.planes[i].1` is named like bytes but the SDK uses it as rows. Confirm on hardware and document it in the wrapper.
5. **cuda-core / cuda-core-ext.** `ManagedBuffer::as_slice`/`as_mut_slice`/`zeroed` are safe fns with a precondition that is unsound on Windows (SE-01). The cuda-core revision `348895e` pinned by nv-py's lock is not in the local checkout cache, so sub-auditor C read `f8b99f3` for the Drop/`free_async` semantics; confirm on `348895e`.
6. **crates/nccl.** Recv requires `&mut DeviceBuffer` (forcing staging, PE-01/PE-04). Context comparison via `Arc::ptr_eq` gives false rejections for distinct `PyContext` objects on the same primary context; the same holds for the nvcomp `same_stream` check (C2).
7. **crates/nv-wgpu.** No error scope or uncaptured handler, so validation errors panic (SE-06). An `open_first` helper would deduplicate the bindings' adapter loop.
8. **crates/cupti.** Does `Subscriber::drop` (cuptiUnsubscribe) wait for an in-flight callback on another thread before freeing the `Box<SubscriberInner>` that holds `Py<PyAny>`?
9. **crates/nv-loader (P7).** Should `add_search_dir` invalidate cached load failures? The AGENTS.md trap documents the behaviour, but Python users have no way to recover.
10. **Root vs device dependency source (D1).** Pick one cuda-rust source (NVIDIA vs the ssoj13 fork) for the host crates the device test harness links. Today the device workspace cannot build its own GPU tests.
11. **xtask `py_stubs.rs` (P17).** Stubs model submodules as classes; consider per-module `.pyi`. The stub check also cannot see that pyproject metadata (P11, P12) is out of sync with the build.
12. **nv-bindgen `device_asm::scan_optix` (not in this slice).** Cross-header merging compares only input and output register lists, not `volatile` or the template (`or_insert`), so two headers that disagree on volatility would be silently merged.
