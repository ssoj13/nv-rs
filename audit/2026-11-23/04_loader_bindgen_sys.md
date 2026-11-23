# Audit: nv-loader, nv-bindgen (+fixture), 21 `*-sys` crates

Date: 2026-11-23. Read-only audit at HEAD `38ec140` (working tree has only untracked `0/`). No cargo or build was run.
Paths are relative to `C:\projects\projects.rust.cg\cglibs\nv-rs\crates\` unless stated.

Severity: P0 = UB/crash/wrong results, P1 = swallowed error/silent fallback/significant perf, P2 = SSOT/dead code/doc drift.
Verdict: CONFIRMED = the failing path was traced in code; PLAUSIBLE = needs runtime, hardware or a specific SDK to show.

---

## 1) Item-by-item ledger

### L-01 | P1 | CONFIRMED | `nv-bindgen/src/nv_bindgen.rs:366-384`
**Defect.** `generate()` wraps `bindings()` in `catch_unwind`. On **any** panic it falls back to prebuilt bindings and reports the fixed reason `"libclang is unavailable"`. The panic payload is thrown away whenever the fallback works. A panic from a generator bug (syn/quote, a bindgen internal panic, an index panic in our own helpers) cannot be told apart from a missing libclang.
**Failing scenario.** A regression in `shims()` or `signature_types()` panics on a new header. The crate has `prebuilt/x86_64-pc-windows-msvc/`, so the build succeeds on stale prebuilt output. The only output is one cargo warning that says libclang is missing. Cargo hides build-script stderr on success, so the panic message is never seen.
**Fix direction.** Fall back only when the panic is the clang-sys "couldn't find libclang" kind (check for libclang up front, e.g. `clang_sys::load()`/`is_loaded()`, before calling bindgen), and `resume_unwind` for anything else. When falling back, put the downcast payload message in the warning.

### L-02 | P1 | CONFIRMED | `nv-bindgen/src/nv_bindgen.rs:370-375` (also `:356-363`)
**Defect.** When the SDK **was found** but `bindings()` returns `HeaderNotFound` or `Bindgen`, the build silently switches to prebuilt bindings. Nothing compares the prebuilt inventory's header sha256 (`copy_prebuilt_bindings`, `:542-551`) with the installed headers. This path also runs for `Shape::StaticLink` (NGX), after `emit_static_link_directives(.., true)` has already linked the **installed** static library. The guard at `:344` only covers `SdkNotFound`.
**Failing scenario.** CUDA 13.x headers make bindgen fail (new syntax or macro) → prebuilt bindings generated from an older toolkit are compiled → struct layouts and enum values come from different headers than the DLL that runs. Same for NGX: prebuilt `ngx_types.rs` from SDK A is linked against `nvsdk_ngx_d.lib` from SDK B. The result is ABI mismatch, i.e. wrong results or UB at run time.
**Fix direction.** Fall back only when the toolchain is missing (libclang), never when installed headers fail to parse. If a fallback is kept, compare each `HeaderHash` in the prebuilt inventory with the installed headers and refuse on mismatch. Refuse the fallback for `StaticLink` whenever the SDK was discovered.

### L-03 | P1 | CONFIRMED | `nv-bindgen/src/nv_bindgen.rs:1479-1501`, `:414-425`, `:575-577`
**Defect.** The fallback sticks. After any fallback, the only `rerun-if-changed` lines are for the prebuilt files. Header lines are printed only after a successful generation. `rerun_vars` leaves out the libclang inputs (`LIBCLANG_PATH`, `LLVM_CONFIG_PATH`, `CLANG_PATH`), and bindgen 0.72 uses clang-sys `runtime` loading by default.
**Failing scenario.** First build without LLVM → prebuilt bindings. The user installs LLVM and sets `LIBCLANG_PATH` → cargo does not rerun the build script, so the stale prebuilt bindings stay until `cargo clean`.
**Fix direction.** Add the clang-sys variables to `rerun_vars`. On the fallback path, also print `rerun-if-changed` for the discovered header paths, when they exist.

### L-04 | P1 | CONFIRMED | `nccl-sys/build.rs:64-79`
**Defect.** Errors are classified by matching the message text: `msg.contains("nccl_device/")`. Any error that mentions a device header, including a real `Bindgen` parse error inside `nccl_device/*.h` from a newer NCCL, is treated as "NCCL < 2.28". The build then silently regenerates host-only bindings and does not set `nccl_has_device`. No `cargo::warning` is printed. Also: if `prebuilt/<target>` ever exists for nccl, `generate()` itself (L-02) would mask the `HeaderNotFound` and set `nccl_has_device` against an old library.
**Failing scenario.** With NCCL 2.29, `nccl_device/gin.h` uses a construct libclang rejects → host-only bindings, the device API disappears, and the device-API tests are compiled out under `#[cfg(nccl_has_device)]`. Nobody is told.
**Fix direction.** Match on `Error::HeaderNotFound { header, .. }` where `header.starts_with("nccl_device/")`, a typed check instead of a string. Print a `cargo::warning` on downgrade. Better: probe for `nccl_device/core.h` before calling `generate()`.

### L-05 | P2 | CONFIRMED | `cufile-sys/build.rs:27-31`, `cufile-sys/src/cufile_sys.rs:3`, `:72-81`
**Defect.** The `ssize_t` `ReturnPolicy` can never match. bindgen 0.72.1 maps `ssize_t` to `isize` when `size_t_is_usize` is set, which is the default (`bindgen-0.72.1/ir/context.rs:2383`, `codegen/mod.rs`). So `cuFileRead`/`cuFileWrite` get checked `Result<isize, CallError>` wrappers. The crate's own test asserts exactly this (they appear in the `unshimmed` list). The docs in `build.rs:3` ("preserves ... negative I/O errors") and `cufile_sys.rs:3` ("negative byte-count failures preserve cuFile's C contracts") are false. The generator does not complain when a policy matches nothing. The fixture has no `ssize_t`/`size_t` return, so this class of bug is not covered.
**Fix direction.** Key the policy on `isize`, or drop it and fix the docs. Make `parse_return_policies`/`shims` fail when a policy type matches zero entry points. Add a `ssize_t` return to `fake.h`.

### L-06 | P1 | PLAUSIBLE | `nv-loader/src/nv_loader.rs:389-420, 750-760`; specs in `*-sys/src/*.rs`
**Defect.** No toolkit or SDK library is version-gated against the header it was bound from. `check_version` and `LoadError::RuntimeTooOld` are called only from a unit test (see L-14). Several specs accept more than one major version on purpose: `nvjpeg` `libnvjpeg.so.13|.12`, `cupti` `.13|.12`, and every Windows `prefix*` pattern accepts any suffix. Windows candidates are ordered by directory first: the first directory with any match wins, whatever version it holds.
**Failing scenario.** An app ships `cublas64_12.dll` next to the exe (or it is first on `NV_DLL_DIRS`) while the bindings were generated from CUDA 13 headers. The exe directory comes before `CUDA_PATH`, so the v12 DLL is loaded. Struct layouts and enum values can differ, which gives silent wrong results or UB. On Linux, `libnvjpeg.so.12` is accepted for CUDA 13 bindings.
**Fix direction.** Have nv-bindgen emit the header's version macro (e.g. `CUBLAS_VER_MAJOR`, `NVJPEG_VER_MAJOR`, `CUDNN_MAJOR`) into the generated files. In the open closure, call the library's `*GetVersion`/`GetProperty` and run `check_version` (this gives L-14 a real caller). Or restrict the patterns to the bound major (`cublas64_13.dll`).

### L-07 | P2 | CONFIRMED | `nv-loader/src/nv_loader.rs:397-412`, `:599-608`, `:71-74`
**Defect.** For `prefix*` specs, `candidates_with` adds a path only if a directory **contains** a match. When nothing is installed, `all` is empty, so `LoadError::NotFound { tried: [] }` renders as `"cublas: not found (searched ); ..."`. The searched directories are lost. Unit test `an_unset_sdk_root_is_never_guessed` (`:1355-1367`) shows the empty list.
**Fix direction.** Record the searched directories, with the pattern (`dir\cublas64_*.dll`), in `NotFound.tried`. Or add a separate `searched_dirs` field.

### L-08 | P2 | CONFIRMED | `nv-loader/src/nv_loader.rs:483`, `:486-495`, `:295-297`, `:72`, `:431-435`
**Defect.** `add_search_dir` is documented as searched "before standard toolkit, SDK, or PATH locations", and the `NotFound` message tells every user to "set NV_DLL_DIRS". On Windows, `Group::Driver` ignores the extra directories entirely (System32 only) and `Group::DriverStore` searches them **last**. On Linux, extra directories come first for **every** group, Driver included. The two OSes behave differently.
**Failing scenario.** On Windows, a user puts a matched `nvml.dll`/`nvEncodeAPI64.dll` in a directory, sets `NV_DLL_DIRS`, gets `NotFound` advising `NV_DLL_DIRS`, and the setting has no effect.
**Fix direction.** Pick one documented policy per group and apply it on both OSes. Make the `NotFound` hint depend on the group.

### L-09 | P2 | CONFIRMED | `nv-loader/src/nv_loader.rs:535-543` vs `:437-446`, doc `:39-42`
**Defect.** The Windows `Group::Sdk` branch searches `root/lib/Linux_x86_64/rel`, a Linux directory that is dead on Windows. The Linux branch searches only `lib64`/`lib` and has no `lib/Linux_x86_64/rel`. The module doc lists neither directory.
**Fix direction.** Move the `Linux_x86_64/rel` entry to the Linux branch, or drop it, and update the doc.

### L-10 | P2 | CONFIRMED | `nv-loader/src/nv_loader.rs:563-578`
**Defect.** `open_library` discards the `load_with_flags(0x1100)` error and retries with `Library::new`. The retry uses the standard search order, which includes PATH and, for dependencies, the CWD. `Rejected` reports only the second error. `LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR` requires an absolute path, so a relative candidate (a relative entry in `PATH` or `NV_DLL_DIRS`; `split_paths` keeps it) always takes the fallback.
**Failing scenario.** A dependency of `cudnn64_9.dll` is missing from the safe directories. The flagged load fails, then the fallback succeeds by picking up a different `cublasLt64_*.dll` from PATH or CWD. No trace of the first failure or of the change in search policy is recorded, not even in `diag`.
**Fix direction.** Keep both errors and record the fallback in `diag`. Make the fallback explicit (e.g. only when `ERROR_MOD_NOT_FOUND`, and log it). Make relative candidates absolute or reject them.

### L-11 | P2 | CONFIRMED | `nvenc-sys/src/nvenc_sys.rs:80`, `nvofa-sys/src/nvofa_sys.rs:70`, `optix-sys/src/optix_sys.rs:65`, `cudnn-sys/src/cudnn_sys.rs:121-134`
**Defect.** Three Windows DLL search policies coexist. Shared `nv_loader::open_library` uses `SEARCH_DLL_LOAD_DIR|DEFAULT_DIRS` with a fallback. nvenc, nvofa and optix call raw `libloading::Library::new`. cudnn uses `LOAD_WITH_ALTERED_SEARCH_PATH`. AGENTS.md says "shared discovery in nv-loader; do not duplicate loaders".
**Fix direction.** Route all of them through `nv_loader::open_library`. Give it an optional flag set if cudnn really needs altered-search-path semantics.

### L-12 | P2 | CONFIRMED (ordering) / PLAUSIBLE (impact) | `nv-loader/src/nv_loader.rs:366-383`, `:407-411`, `:418`
**Defect.** (a) The version key splits on every non-digit, so `nvrtc64_*` matches both `nvrtc64_120_0.alt.dll` and `nvrtc64_120_0.dll` (CUDA 12.x ships both). Both get key `[120, 0]`. The tie-break is ascending lexical (`a.cmp(b)`) and `'.' < 'd'`, so the `.alt` variant sorts **first** and is tried first. (b) The tie-break direction (ascending) is the opposite of the version direction (descending). (c) `dll_version_key` returns `(Vec, &str)` but callers ignore the `&str`. (d) `out.dedup()` removes only consecutive duplicates; an overlap between an explicit name and a prefix (`diag.rs:463` `["cudnn64_9.dll","cudnn64_*"]`) can leave a non-adjacent duplicate.
**Fix direction.** Require that the suffix after the prefix be only digits and `_`. Explicitly rank or exclude `.alt`. Drop the unused tuple member. Dedup with a seen-set.

### L-13 | P2 | CONFIRMED | `nv-loader/src/diag.rs:394-519` (+ `xtask/src/stage_dlls.rs:39+`)
**Defect.** `probe_all` holds a hand-copied `DllSpec` catalog that has already drifted from the sys crates, which are the source of truth:
- Linux sonames: nvrtc diag `.13,.12` vs sys `.13`; cublas diag `.13,.12` vs sys `.13`; cufft diag `.12,.11` vs sys `.12`; cusolver diag `.12,.11` vs sys `.12`.
- Windows names: cudnn diag `["cudnn64_9.dll","cudnn64_*"]` vs sys `["cudnn64_*"]`; nvcomp diag adds `"nvcomp.dll"`.
- Hints: cudnn, nvcomp and nccl use `hint: None` in diag, but the sys crates use `hint(env!("NV_RS_SDK_BIN"))`. A probe can therefore report "missing" for an install the real loader finds through the hint.
- Names: `nvoptix`/`nvofapi` vs `optix`/`nvofa`.
- Missing entries: cublaslt, cupti, cusolvermg, the npp parts.
`xtask/src/stage_dlls.rs` holds a **third** copy ("mirroring crates/*-sys").
**Fix direction.** Use a single catalog. The sys crates already export `pub const DLL`. Have consumers (nv-py, xtask) assemble the list from `<crate>_sys::DLL`. Alternatively, keep one catalog in a small leaf crate that the sys crates also use.

### L-14 | P2 | CONFIRMED | `nv-loader/src/nv_loader.rs:89-98`, `:749-760`
**Defect.** `check_version` and `LoadError::RuntimeTooOld` have no production caller; grep finds them only in `version_gate_rejects_only_older_runtimes` and in docs. The feature was never finished (see L-06), but the module doc (`:6-7`) presents it as part of the loader.
**Fix direction.** Wire it in through L-06, or delete it together with its doc.

### L-15 | P2 | CONFIRMED | `nv-loader/src/nv_loader.rs:298-308`, `:712-727`
**Defect.** `add_search_dir`, called after a library's first load attempt, silently does nothing for that library, because the `OnceLock` caches failures. This is a documented trap (AGENTS.md) but nothing enforces or reports it. `python/nv-py/src/diag.rs:76` (`nv_rs.add_dll_directory`) can easily be called after an import has already touched a shim.
**Fix direction.** Keep a global "first load happened" flag. When `add_search_dir` runs after it, emit a `diag::record_event(Warn, ...)` and return a value or bool the caller can check. Or document a `reset`-free pattern in the Python API.

### L-16 | P2 | CONFIRMED (code) / PLAUSIBLE (impact) | `nv-bindgen/src/nv_bindgen.rs:649-665`, `:414-418`
**Defect.** The coverage `sha256` and `rerun-if-changed` cover only `spec.headers`, not the headers they include. Umbrella headers like `cudnn.h` (includes `cudnn_graph.h`, `cudnn_ops.h`, ...), `npp.h`, `nvapi.h` (`nvapi_lite_*.h`), `cusolverDn.h` and `nvEncodeAPI.h` hide most of the API. nccl and nvcomp list their sub-headers explicitly as a workaround (see `nccl-sys/build.rs:3-4`).
**Failing scenario.** An in-place SDK update that changes `cudnn_ops.h` but not `cudnn.h` → no regeneration and an unchanged inventory hash.
**Fix direction.** Hash and watch every file libclang opened. bindgen offers `parse_callbacks(Box::new(CargoCallbacks))` / `include_file` callbacks; collect those paths.

### L-17 | P2 | CONFIRMED | `nv-bindgen/src/nv_bindgen.rs:1485`, `:1495`
**Defect.** `PATH` is in `rerun_vars` for `Sdk::CudaToolkit`, `CudaToolkitSubdir` and `Nv`. Any change to PATH, such as building from a VS developer prompt and then from a plain shell, reruns bindgen for about 17 sys crates. That costs a lot of rebuild time.
**Fix direction.** Rerun on the discovered toolkit root (via `rerun-if-changed=<root>/version.json` or the include dir) instead of on all of PATH.

### L-18 | P2 | CONFIRMED | `nv-bindgen/src/nv_bindgen.rs:310-329`
**Defect.** In `NV_RS_USE_PREBUILT=1` mode, `sdk_paths(spec).ok()` swallows every discovery error (`InvalidSdkPath`, `UnsupportedTarget`, `SdkNotFound`). For `StaticLink`, it then emits `rustc-link-lib=static=nvsdk_ngx_d` with **no** `rustc-link-search` (`require_lib_dir=false`). This contradicts the explicit refusal at `:344` for the same situation without the env var.
**Failing scenario.** `NV_RS_USE_PREBUILT=1` on a machine without the NGX SDK → the linker fails with "could not find native static library nvsdk_ngx_d" instead of `MissingLinkDirectory`.
**Fix direction.** Propagate non-`SdkNotFound` errors. For `StaticLink` with non-empty `link_libs`, return `MissingLinkDirectory`/`SdkNotFound` in prebuilt mode too.

### L-19 | P2 | CONFIRMED | `nv-bindgen/src/nv_bindgen.rs:489-517` vs doc `:292-294`
**Defect.** When `NV_RS_PREBUILT_DIR` is set, `resolve_prebuilt_dir` always returns `<env>/<target>` even if the expected files are missing there, and never tries `<CARGO_MANIFEST_DIR>/prebuilt/<target>`. The doc says "under NV_RS_PREBUILT_DIR **or** <manifest>/prebuilt".
**Failing scenario.** A user sets `NV_RS_PREBUILT_DIR` for one library family → every other sys crate's fallback fails with "prebuilt file missing", even though its own `prebuilt/` has the files.
**Fix direction.** Fall through to the manifest directory when the env directory lacks the files, or fix the doc.

### L-20 | P2 | CONFIRMED | `nv-bindgen/src/nv_bindgen.rs:1451-1468`; `cupti-sys/build.rs:40-66`; `nvjpeg-sys/build.rs:37-78`
**Defect.** `normalize_positive_enums` works by exact line matching and silently does nothing for an entry that does not match (a typo, a renamed typedef, a bindgen formatting change). The post-processors in cupti and nvjpeg also depend on exact output text (`"pub struct CUpti_ActivityKernel"`, `"..._UNCHANGEDI:"`). All 13 current `positive_enums` entries do resolve to `c_uint` (checked in prebuilt output, see section 4). Nothing guards against drift.
**Fix direction.** Return `Error::Generated` when a `positive_enums` name was not rewritten. Do these edits on the syn AST instead of on text lines.

### L-21 | P2 | CONFIRMED | `nv-bindgen/src/nv_bindgen.rs:730-739`; `npp-sys/build.rs:7-102`
**Defect.** For `SplitExports`, `allow_fn` (`"^npp.*"`) is ignored; each part takes only its own `header_files`. Nothing checks that the parts together cover every function `npp.h` declares. A new NPP header gets no shims and appears in neither `symbols` nor `unshimmed`, so the coverage inventory cannot see the gap.
**Fix direction.** Run one more functions pass with `allow_fn` and fail, or list the names in `unshimmed`, for any function that no part claims.

### L-22 | P2 | CONFIRMED | `nv-bindgen/src/device_asm.rs:681-688`
**Defect.** `scan_optix` merges intrinsics across headers by comparing only `inputs`/`outputs`. Within one header, `scan()` (`:192-207`) also compares the template and `volatile`. If two headers have the same name with a different template or a different `volatile`, the first one wins silently and `sites` is not summed.
**Fix direction.** Use the same `shape()` comparison as `scan()`, and add `sites` together.

### L-23 | P2 | PLAUSIBLE | `nv-bindgen/src/device_asm.rs:422-451`
**Defect.** `macro_args` (a) finds the macro name with `find(name)` and no identifier-boundary check, so `BODY` matches inside `XBODY(`/`BODY_EXT(`; (b) searches to the end of the file, past `#undef`, so a later unrelated `#define` with the same name adds its arguments; (c) uses `rest.find(')')`, which cuts nested arguments short (`BODY( f(1) )` → `f(1`).
**Failing scenario.** An OptiX header update adds a same-named macro or an identifier that contains the name → a bogus `_optix_<name>` wrapper with a wrong template, or a spurious coverage pass.
**Fix direction.** Check identifier boundaries, stop at the matching `#undef`, and use `matching_paren` for the argument span.

### L-24 | P2 | PLAUSIBLE | `nv-bindgen/src/device_asm.rs:356`, `:140-144`
**Defect.** The error snippet `&section[i..(i + 20).min(section.len())]` slices by byte. With non-ASCII text in a template section it can split a UTF-8 character and panic in the build script instead of returning `ScanError::Parse`. Line numbers in errors are counted after line continuations are removed (`:140`), so they drift from the real header lines.
**Fix direction.** Use `section[i..].chars().take(20)`. Count lines on the original text, or map offsets back to it.

### L-25 | P2 | CONFIRMED | `cudnn-sys/src/cudnn_sys.rs:19-78`, `:187-200`, `:47-57`; `nv-bindgen-fixture/src/nv_bindgen_fixture.rs:156-157`, `:247-248`
**Defect.** cudnn-sys copies the whole `exports_sys!` body (the types/api/shims modules, `DLL`, `INVENTORY`, `loaded`, the loader and `load`) to add sibling preloading. Its shims module allows `clippy::undocumented_unsafe_blocks` with the comment "prettyplease drops comments, so the per-block SAFETY lint cannot be met here". That reason is stale: `document_unsafe_blocks` (`nv_bindgen.rs:1023-1039`) re-attaches the SAFETY comments, and `exports_sys!`'s shims module (`nv_loader.rs:689-696`) has no such allow while the workspace denies the lint (`Cargo.toml:54`). The fixture repeats the same stale comment and allow.
**Fix direction.** Give `exports_sys!` an optional `open: <fn>` hook (custom opener plus bundle type) so cudnn reuses the macro. Remove the stale allows and comments.

### L-26 | P2 | CONFIRMED | `nvenc-sys/src/nvenc_sys.rs:54-131`, `nvofa-sys/src/nvofa_sys.rs:44-115`, `optix-sys/src/optix_sys.rs:42-98`
**Defect.** The `Shape::FunctionTable` loaders are written by hand three times with the same structure: a struct holding `_lib` and the table, `unsafe impl Send/Sync`, `loaded`/`api`/`load`, a symbol lookup and a version check. Every Exports-shaped crate uses a shared macro instead.
**Fix direction.** Add a `function_table_sys!` macro to nv-loader that takes an `open` function. It also gives L-11 a single owner.

### L-27 | P2 | CONFIRMED | `nvenc-sys/src/nvenc_sys.rs:41-51`, `:152-159`
**Defect.** `struct_version_matches_the_header_macro` is vacuous: it asserts the formula against itself. `FUNCTION_LIST_VERSION = struct_version(2)` and the `0x7 << 28` constant are copied by hand from header macros (`NV_ENCODE_API_FUNCTION_LIST_VER`, `NVENCAPI_STRUCT_VERSION`), so an SDK change goes unnoticed.
**Fix direction.** Have nv-bindgen evaluate these macros (e.g. emit a small C wrapper with `const uint32_t NV_RS_FUNCTION_LIST_VER = NV_ENCODE_API_FUNCTION_LIST_VER;` that bindgen turns into a const). Assert against that value.

### L-28 | P2 | CONFIRMED | `nvapi-sys/src/nvapi_sys.rs:7-8` vs `:10`
**Defect.** The doc says "on other targets `load()` reports `NotFound`", but the crate is `#![cfg(target_os = "windows")]`, so on other targets there is no `load()` at all.
**Fix direction.** Fix the doc, or expose a non-Windows `load()` that returns `NotFound`.

### L-29 | P2 | CONFIRMED | `nvjpeg-sys/build.rs:37-78`
**Defect.** These are backward-compatibility shims of the kind the repository rules reject. They hand-copy enum values (`NVJPEG_OUTPUT_UNCHANGEDI = 10`, `NVJPEG_INPUT_YUV = 1`) into CUDA 12 builds, where those values are meaningless to the runtime. They also append a **Rust-ABI** `pub unsafe fn nvjpegEncoderParamsSetRestartInterval` that always returns `IMPLEMENTATION_NOT_SUPPORTED`, while the CUDA 13 build generates an `extern "C"` shim. So the function's type depends on the toolkit version, and the stub never asks the runtime even when the loaded DLL has the symbol.
**Fix direction.** Drop the compat layer and require the bound major. Or generate the binding from a declared signature so it goes through the normal missing-symbol path (`extern "C"`, `missing_symbol` code).

### L-30 | P2 | PLAUSIBLE (perf) | `nv-bindgen/src/nv_bindgen.rs:990-998`
**Defect.** Every generated query-interface (NVAPI) shim calls `nvapi_QueryInterface(id)` on **every** call. Resolved pointers are not cached, so each call pays a driver hash lookup. Per-frame NVAPI D3D-extension calls pay it repeatedly.
**Fix direction.** Cache per-id resolution in a generated `static OnceLock<Option<fn>>` (or an `AtomicPtr`) per shim.

### L-31 | P2 | CONFIRMED (code) / PLAUSIBLE (perf) | `cudnn-sys/src/cudnn_sys.rs:137-164`
**Defect.** `preload_siblings` eagerly maps **every** `cudnn_*64_9.dll` in the directory, including `cudnn_engines_precompiled64_9.dll`, which is hundreds of MB, plus adv, cnn and so on. The dispatcher would normally load these lazily, so the eager load costs memory and startup time. A failure to load `zlibwapi.dll` is discarded with `if let Ok`.
**Fix direction.** Preload only the dependency closure the dispatcher needs (`graph`, `ops`), or put the directory on the search path (`AddDllDirectory`) instead of mapping everything. Record the zlib failure in `diag`.

### L-32 | P2 | PLAUSIBLE | `nccl-sys/build.rs:60-83`, `cufile-sys/build.rs:44-50` vs `ngx-sys/build.rs:85-91`
**Defect.** Optional SDKs are handled inconsistently. On Linux, `cargo build --workspace` hard-fails when NCCL is not installed (`SdkNotFound`, and there is no prebuilt directory for nccl or cufile). NGX on Linux degrades with a warning. AGENTS.md documents `device/` and `python/` as the separately gated parts, not NCCL.
**Fix direction.** Choose one policy: a cfg-gated empty crate plus a warning, or Cargo features. Document it.

### L-33 | P2 | CONFIRMED | `nvapi-sys/build.rs:35`, `ngx-sys/build.rs:48`, `:94`, vs `cufile-sys/build.rs:37-43`, `nccl-sys/build.rs:53-59`
**Defect.** The target-OS gate is copied four ways. nvapi treats a missing `CARGO_CFG_TARGET_OS` as "not windows" without any error. ngx uses `unwrap_or_default()` for the arch and does nothing on Linux non-x86_64 without a warning. cufile and nccl fail with a cargo error.
**Fix direction.** Add one helper in nv-bindgen (`target_os()`/`skip_unless(os)`) that prints a consistent message.

### L-34 | P2 | CONFIRMED | `nv-loader/src/diag.rs:11-13`, `:44-53`, `:150-156`
**Defect.** The level parse is inconsistent: `"1"` means `Error` but `"true"` means `Trace`. The doc lists `1` among the values that enable logging, so a user who sets `NV_RS_DEBUG=1` to debug DLL discovery gets only errors and no `Info` load events. An unparsable `NV_RS_DEBUG` value does not fall through to `NV_RS_LOG` (the `or_else` only handles "unset").
**Fix direction.** Map `1` to `Info` (load events) or document the mapping. On a parse failure, fall through to `NV_RS_LOG` or warn once.

### L-35 | P2 | PLAUSIBLE | `nv-bindgen/src/nv_bindgen.rs:1627-1643`
**Defect.** `system_include_dirs` hard-codes `Arch::X64` for both host and target. `sdk_target_arch` (`:1519`) honours `CARGO_CFG_TARGET_ARCH`. An `aarch64`/`i686-pc-windows-msvc` build gets the include environment of the x64 VS toolset.
**Fix direction.** Derive both host and target arch the same way `sdk_target_arch` does.

### L-36 | P2 | PLAUSIBLE | `cudnn-sys/src/cudnn_sys.rs:71`, `nvcomp-sys/src/nvcomp_sys.rs:19`, `nccl-sys/src/nccl_sys.rs:20` (+ `nv_loader.rs:545-547`)
**Defect.** The SDK DLL directory found on the build machine is baked into shipped binaries through `env!("NV_RS_SDK_BIN")` and is searched on end-user machines before PATH. If that absolute path exists and is user-writable on the target machine (e.g. under a user profile, or `C:\SDK` created by the user), a planted `cudnn64_9.dll` is loaded.
**Fix direction.** Use the hint only in debug or dev builds (`cfg(debug_assertions)`), or behind an opt-in env var.

### L-37 | P2 | CONFIRMED | `nv-bindgen/src/nv_bindgen.rs:2241-2287`
**Defect.** The test uses a fixed temp directory, `std::env::temp_dir().join("nv_bindgen_prebuilt_test")`, and deletes it with `remove_dir_all`. Two concurrent test runs (two targets or two checkouts) race and fail.
**Fix direction.** Use a unique directory (pid plus a counter, or the `tempfile` crate).

### Clean / no defect found
- `nv-loader`: `QueryInterface::open` (lifetime correct, `_lib` keeps the symbol valid); the `exports_sys!`/`split_exports_sys!` caches (the handle is owned by a `'static` OnceLock value and never dropped); `CallError`/`SymbolError` source chains; `windows_display_driver_dirs`.
- `diag.rs` atomics: alloc/free accounting and the peak CAS are correct; the saturating free is intentional.
- `nv-bindgen`: the `shims`/`query_shims` codegen (parameter shadowing guarded by the fixture, `transmute` of `*mut c_void` to the exact header fn type), `interface_ids`, `regex_escape`, `split_sections`, `matching_paren`, `constraints`, `emit`.
- Sys crates: cublas, cublaslt, cufft, cusolver, cusolvermg, cusparse, nvcuvid, nvml, nvrtc and optix are thin recipes with no defect of their own beyond the cross-cutting items (L-06, L-11, L-13, L-16, L-26). The `QueryFn` signature in optix-sys matches `optixQueryFunctionTable`. The NVENC/NVOFA version arithmetic `major<<4|minor` matches the SDK docs.

---

## 2) Files inspected

| File | Lines | Read fully |
|---|---:|---|
| nv-loader/Cargo.toml | 21 | yes |
| nv-loader/src/nv_loader.rs | 1501 | yes |
| nv-loader/src/diag.rs | 519 | yes |
| nv-bindgen/Cargo.toml | 26 | yes |
| nv-bindgen/src/nv_bindgen.rs | 2304 | yes (4 pages) |
| nv-bindgen/src/device_asm.rs | 722 | yes |
| nv-bindgen/tests/fixture/{fake.h, qi.h, qi_interface.h, split.h, split_a.h, split_b.h, split_common.h} | 27/9/11/3/5/4/3 | yes |
| nv-bindgen-fixture/{Cargo.toml, build.rs, src/nv_bindgen_fixture.rs} | 21/70/295 | yes |
| cublas-sys/{Cargo.toml, build.rs, src/cublas_sys.rs} | 22/31/42 | yes |
| cublaslt-sys/{Cargo.toml, build.rs, src/cublaslt_sys.rs} | 22/31/46 | yes |
| cudnn-sys/{Cargo.toml, build.rs, src/cudnn_sys.rs} | 22/32/257 | yes |
| cufft-sys/{Cargo.toml, build.rs, src/cufft_sys.rs} | 22/32/54 | yes |
| cufile-sys/{Cargo.toml, build.rs, src/cufile_sys.rs} | 26/51/123 | yes |
| cupti-sys/{Cargo.toml, build.rs, src/cupti_sys.rs} | 22/66/42 | yes |
| cusolver-sys/{Cargo.toml, build.rs, src/cusolver_sys.rs} | 22/31/42 | yes |
| cusolvermg-sys/{Cargo.toml, build.rs, src/cusolvermg_sys.rs} | 22/31/41 | yes |
| cusparse-sys/{Cargo.toml, build.rs, src/cusparse_sys.rs} | 22/31/42 | yes |
| nccl-sys/{Cargo.toml, build.rs, src/nccl_sys.rs} | 29/84/419 | yes |
| ngx-sys/{Cargo.toml, build.rs, src/ngx_sys.rs} | 18/96/98 | yes |
| npp-sys/{Cargo.toml, build.rs, src/npp_sys.rs} | 22/109/82 | yes |
| nvapi-sys/{Cargo.toml, build.rs, src/nvapi_sys.rs} | 21/42/58 | yes |
| nvcomp-sys/{Cargo.toml, build.rs, src/nvcomp_sys.rs} | 26/75/131 | yes |
| nvcuvid-sys/{Cargo.toml, build.rs, src/nvcuvid_sys.rs} | 22/45/50 | yes |
| nvenc-sys/{Cargo.toml, build.rs, src/nvenc_sys.rs} | 21/32/168 | yes |
| nvjpeg-sys/{Cargo.toml, build.rs, src/nvjpeg_sys.rs} | 22/78/42 | yes |
| nvml-sys/{Cargo.toml, build.rs, src/nvml_sys.rs} | 22/31/51 | yes |
| nvofa-sys/{Cargo.toml, build.rs, src/nvofa_sys.rs} | 22/31/143 | yes |
| nvrtc-sys/{Cargo.toml, build.rs, src/nvrtc_sys.rs} | 22/31/42 | yes |
| optix-sys/{Cargo.toml, build.rs, src/optix_sys.rs} | 22/26/133 | yes |
| AGENTS.md (root), crates/AGENTS.md | n/a | yes |
| Root Cargo.toml (lints, bindgen version) | partial | grep only |
| xtask/src/stage_dlls.rs | partial | grep only (to confirm the third catalog copy) |
| `*/prebuilt/*/*_types.rs` | partial | grep only (positive-enum check) |
| `~/.cargo/registry/.../bindgen-0.72.1/ir/context.rs`, `codegen/mod.rs` | partial | grep only (ssize_t mapping) |

## 3) Files changed

None. No file in the repository was created, edited or deleted. The only file written is this report, in the session scratchpad.

## 4) Static/invariant verification performed

- **Caller tracing.** Traced the callers of `check_version`, `RuntimeTooOld`, `probe_all`, `add_search_dir`, `core_types`, `scan_optix` and `interface_ids` repo-wide (excluding `target/`). `check_version`/`RuntimeTooOld` are referenced only by a test and docs (L-14).
- **ssize_t mapping.** bindgen 0.72.1 is locked in `Cargo.lock:200-201`; its `ir/context.rs:2383` maps `"size_t" | "ssize_t"` under `size_t_is_usize`. This, together with the `cufile_sys.rs:72-81` unshimmed assertion, establishes L-05.
- **Positive enums.** Every `positive_enums` entry in 12 crates resolves to `::core::ffi::c_uint` in the committed prebuilt output (cublas, cublaslt, cudnn, cufft, cupti, cusolver, cusolvermg, cusparse, nvjpeg, nvml, nvrtc, nvcomp ×2). So L-20 is a missing guard, not a live mis-pin.
- **Workspace lints.** `Cargo.toml:48-54` sets `unsafe_op_in_unsafe_fn = deny`, `clippy::all = deny`, `undocumented_unsafe_blocks = deny`. Used for L-25.
- **Prebuilt directories** exist for 19 crates. There are none for `cufile-sys` and `nccl-sys`, which matters for L-04 and L-32.
- **`#[allow]` review.**
  - Justified: the generated-module allows in `exports_sys!`/`query_interface_sys!`/`split_exports_sys!`/`ngx_sys`/`nvenc`/`nvofa`/`optix` (header-dependent lint levels); `#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]` on generated shims (C names and arity); `#[allow(clippy::too_many_arguments)]` in `device_asm::emit`.
  - Not justified: `clippy::undocumented_unsafe_blocks` in cudnn-sys and in the fixture (L-25, stale reason).
- **`let _ =` review.** `device_asm.rs:481-519` uses `let _ = writeln!(String, ..)`, which cannot fail, so it is fine. No other `let _ =` on fallible operations in the slice.
- **Swallowed results.**
  - Problems: `nv_bindgen.rs:311` `.ok()` (L-18); `nv_loader.rs:568` `if let Ok` (L-10); `cudnn_sys.rs:158` `if let Ok` (L-31).
  - Acceptable: `nv_loader.rs:211/219/223/224` (probe semantics, documented "empty when absent"); `windows_display_driver_dirs` `unwrap_or_default` (a registry value that is absent is not an error); `cudnn_sys.rs:125` `absolute().unwrap_or_else` (falls back to a non-flagged load).
- **Unsafe review.** Every `unsafe` block in the slice has a SAFETY comment. The transmute in QI shims goes from `*mut c_void` to the exact header `unsafe extern "C" fn` type (same size). FunctionTable `Send/Sync` impls are justified as immutable after construction. Library handles outlive every cached pointer, because each lives in a never-dropped `'static OnceLock` value.
- **Panics reachable from build scripts.** Only L-24 (string slicing). `split_sections` `b.len() - 1` is reachable only when `b[i] == b'"'`, so `len >= 1`. `matching_paren` `depth -= 1` starts at the `(`, so `depth >= 1`.

## 5) Risks / open seam questions for other slices

1. **Null pointer fallbacks (wrapper crates).** Every generated shim that returns a pointer gives back `null`/`null_mut` when the library or symbol is missing (`Fallback::Null`, `nv_bindgen.rs:1187-1193`), e.g. `cudnnGetErrorString`, `nvmlErrorString`, `cublasGetStatusString`, `ncclGetErrorString`, `nppGetLibVersion`, `nvcompGetStatusString`. Do the safe wrappers (cudnn, nvml, npp, nvcomp, nccl, python/nv-py) null-check before `CStr::from_ptr`? If not, that is a P0 in the wrapper slice.
2. **Ambiguous status codes.** `missing_symbol` and `not_loaded` are the same code for nccl (`ncclSystemError`), nvrtc (`INTERNAL_ERROR`) and nvcomp bitcomp (`UNKNOWN_ERROR`). For cuBLAS/cuSPARSE/cuSOLVER `not_loaded` is `NOT_INITIALIZED`, which wrappers may confuse with "handle not created". Do the wrappers call `load()` first so they report a typed `LoadError` instead of a status code?
3. **Runtime/header version skew (L-06).** Does any wrapper (cuda-core, optix, ngx, nvenc wrapper, python) check `*GetVersion` against the bound header? The OptiX ABI and NVENC/NVOFA paths do (they reject an older driver). The toolkit and SDK libraries do not.
4. **Python DLL directory timing (L-15).** `nv_rs.add_dll_directory` → `nv_loader::add_search_dir`: does nv-py ever touch a sys shim during module init (for example `probe_all` or a version query)? If so, user-added directories never apply to that library.
5. **Three DLL catalogs (L-13).** nv-loader `diag::probe_all`, xtask `stage_dlls::SPECS` and the sys crates' `DLL` consts. The xtask slice should diff its catalog against the sys crates; `"dlss"` exists only in xtask.
6. **NGX static link and the DLSS DLLs.** ngx-sys links `nvsdk_ngx_d` statically. nv-demo `build.rs` stages `nvngx_dlss*.dll`. The loader's `Group::DriverStore` docs list `nvngx`, but no sys crate in this slice loads NGX through nv-loader. Who owns DLSS DLL discovery at run time?
7. **cuFile semantics change (L-05).** If wrapper code expects `cuFileRead`/`cuFileWrite` to return `ssize_t` with negative errors, it actually receives `Result<isize, CallError>`. Does the storage slice handle both a negative `Ok(isize)` and `Err(CallError)`?
8. **Prebuilt freshness (L-01..L-03).** The xtask prebuilt generator should store the source SDK version and header hashes, and `generate()` should verify them. Which slice owns `xtask prebuilt`?
9. **OptiX device header scan (L-22/L-23).** `device/optix-device` and `xtask optix_cov` both rely on `scan_optix`. Their slice should confirm that `optix_device_inventory.json` coverage would catch a template/volatile divergence between `optix_device_impl.h` and `optix_device_impl_coop_vec.h`.
