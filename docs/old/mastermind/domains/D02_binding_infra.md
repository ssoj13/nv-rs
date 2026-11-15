# D02 — shared binding infrastructure

Updated: 2026-11-14 (UTC). Status: static Inquisitor Q1–Q7 audit completed across `nv-bindgen`, `nv-loader`, and `nv-bindgen-fixture`; repairs applied (`nv_bindgen.rs:344` QueryInterface `rerun-if-changed`, `nv_bindgen.rs:759-812` `WrapMode::CheckedVoid` eliminating `clippy::unit_arg`, `nv_bindgen.rs:991` cross-OS `normalize_positive_enums` + test `nv_bindgen.rs:1286`, `nv_loader.rs:100-150` `CallError`/`SymbolError` manual `Error::source` + test `nv_loader.rs:642`); **no `cargo` or `git` executed by worker** — root reported `clippy -p nv-bindgen-fixture` clean (`#117`), `nv-bindgen`/`nv-loader` test re-run pending root.
See the [domain index](../MASTERMIND_INDEX.md), [D01 ledger](D01_m2_sys.md), [seam matrix](../SEAM_MATRIX.md)
and [work queue](../../../TODO.md).

## Exact source boundary

D02's original boundary contains these 21 files (plus `crates/nv-bindgen/src/device_asm.rs`, `crates/nv-bindgen/tests/fixture/split*.h`, and `Cargo.toml`s audited in D01_D02). Root owns nv-bindgen's vcv-rs adapter integration. D02 also received bounded permission for NCCL/cuFile checked-call fixtures and NCCL cleanup repairs, recorded in the [D01 ledger](D01_m2_sys.md).
Workers do not run `cargo` or `git`.

| # | Allowed source file |
|---|---|
| 1 | crates/nv-bindgen/src/nv_bindgen.rs |
| 2 | crates/nv-loader/src/nv_loader.rs |
| 3 | crates/nv-bindgen-fixture/build.rs |
| 4 | crates/nv-bindgen-fixture/src/nv_bindgen_fixture.rs |
| 5 | crates/nv-bindgen/tests/fixture/fake.h |
| 6 | crates/nv-bindgen/tests/fixture/qi.h |
| 7 | crates/nv-bindgen/tests/fixture/qi_interface.h |
| 8 | crates/cublas-sys/build.rs |
| 9 | crates/cublaslt-sys/build.rs |
| 10 | crates/cudnn-sys/build.rs |
| 11 | crates/cufft-sys/build.rs |
| 12 | crates/cupti-sys/build.rs |
| 13 | crates/cusolver-sys/build.rs |
| 14 | crates/cusolvermg-sys/build.rs |
| 15 | crates/cusparse-sys/build.rs |
| 16 | crates/npp-sys/build.rs |
| 17 | crates/nvapi-sys/build.rs |
| 18 | crates/nvcuvid-sys/build.rs |
| 19 | crates/nvjpeg-sys/build.rs |
| 20 | crates/nvml-sys/build.rs |
| 21 | crates/nvrtc-sys/build.rs |

## Current source checkpoint

Static Inquisitor audit verified shared `ReturnPolicy` handling, boxed `CallError` diagnostics, mixed-return fixtures, Linux SDK load hints, `device_asm`, `vcv-rs` adapter integration, lint compliance (`unsafe_op_in_unsafe_fn = deny`, `clippy::undocumented_unsafe_blocks = deny`, `missing_docs = warn`, `//!` headers), and `unused_variables` safety (`__nv_error` underscore prefix). All execution gates remain pending root.

Read-only inspection verified these anchors at current line numbers:

| Anchor | Source at checkpoint | Gate |
|---|---|---|
| ReturnPolicy | crates/nv-bindgen/src/nv_bindgen.rs:689 | Static Q1–Q7 pass; documented & validated |
| parse_return_policies | crates/nv-bindgen/src/nv_bindgen.rs:705 | Static Q1–Q7 pass (empty/duplicate/invalid policy rejection); test execution pending |
| fallback | crates/nv-bindgen/src/nv_bindgen.rs:744 | Static Q1–Q7 pass (void/scalar -> `None` checked, ptr -> `Null`, path -> `Policy`); test pending |
| wrapper_plan | crates/nv-bindgen/src/nv_bindgen.rs:786 | Repaired with `WrapMode::CheckedVoid` (`nv_bindgen.rs:759-775`) emitting `{ #call; ::core::result::Result::Ok(()) }` to avoid `clippy::unit_arg` (`#115`); test pending |
| shims | crates/nv-bindgen/src/nv_bindgen.rs:828 | Static Q1–Q7 pass (`__nv_error` suppresses `unused_variables`, `#[allow]` & `// SAFETY:` verified, uses `WrapMode::wrap_call`); test pending |
| normalize_positive_enums | crates/nv-bindgen/src/nv_bindgen.rs:991 | Repaired to normalize both `c_int` (MSVC) and `c_uint` (GCC) to `::core::ffi::c_uint`; test `nv_bindgen.rs:1286` pending |
| generate (QI rerun) | crates/nv-bindgen/src/nv_bindgen.rs:344 | Repaired to emit `cargo:rerun-if-changed` for `Shape::QueryInterface` `interface_header`; build pending |
| CallError | crates/nv-loader/src/nv_loader.rs:100 | Repaired `Error::source` on `CallError`/`SymbolError` to unsize `&LoadError`/`&libloading::Error` directly (fixes `#111` downcast failure); tests `nv_loader.rs:608`, `nv_loader.rs:642` pending |
| mixed_return_policies_preserve_missing_symbol_errors | crates/nv-bindgen-fixture/src/nv_bindgen_fixture.rs:155 | Static Q1–Q7 pass (all 11 `fake.h` + 6 `qi.h` functions covered); re-run pending root |
| sdk_paths | crates/nv-bindgen/src/nv_bindgen.rs:1066 | Static Q1–Q7 pass (`vcv_rs::nvsdk::detect` + `sdk_target_arch` + `sdk_path_str`); build/test pending |
| sdk_rebuild_inputs_follow_discovery_aliases | crates/nv-bindgen/src/nv_bindgen.rs:1195 | Static Q1–Q7 pass; not run |

## Written repair and integration checkpoint

Void and scalar native calls without a native failure sentinel keep their original names with a Rust ABI and `Result<T, CallError>`; void uses `Result<()>`. Configured status returns and pointers retain native C ABI behavior. Exports and query-interface generation share the checked wrapper plan. Fixture source covers load failure, missing symbols, null query results and successful native calls; no fixture has executed in this wave.

Repairs added in this audit pass:
1. `crates/nv-bindgen/src/nv_bindgen.rs:344-348`: `generate()` now prints `cargo:rerun-if-changed` for `Shape::QueryInterface { interface_header, .. }`.
2. `crates/nv-bindgen/src/nv_bindgen.rs:991-1008` & `1286-1295`: `normalize_positive_enums` now matches both `::std::os::raw::c_int;` (Windows MSVC) and `::std::os::raw::c_uint;` (Linux GCC), preventing `types_exclude_functions_and_pin_positive_enums` from failing on Linux; added `positive_enums_normalize_both_msvc_and_gcc_underlying_types`.
3. `crates/nv-loader/src/nv_loader.rs:99-150` & `641-660`: replaced `#[derive(thiserror::Error)]` on `CallError` and `SymbolError` with manual `fmt::Display` + `std::error::Error` so `source()` unsizes `&'static LoadError` / `&'static libloading::Error` directly instead of `thiserror`'s `&'static T` `AsDynError` impl (which gave `TypeId` = `&'static E` and broke `.is::<libloading::Error>()` in `nv-bindgen-fixture`); added `call_error_preserves_load_and_symbol_error_causes`.
4. `crates/nv-bindgen/src/nv_bindgen.rs:640-644`, `759-812`, `847-851`, `1263-1267`, `1374-1378`: added `WrapMode::CheckedVoid` in `wrapper_plan` so both `query_shims` and `shims` emit `{ unsafe { ... }; ::core::result::Result::Ok(()) }` for `void` functions instead of `Result::Ok(unsafe { ... })`, fixing `clippy::unit_arg` (`#115`; scoped void-assertion helper `fn_slice` fixed `#117`).

## Review and verification queue

- [x] Static Inquisitor Q1–Q7 review of `nv-bindgen`, `nv-loader`, `nv-bindgen-fixture` (`//!` headers, `missing_docs`, `unsafe_op_in_unsafe_fn`, `undocumented_unsafe_blocks`, zero non-test panics/stubs/swallowed errors).
- [ ] Review all 14 migrated sys recipes outside D01_D02's write boundary against their SDK return types (`cublaslt`, `cupti`, `cusolvermg`, `nvofa`, etc. — read-only spot check done on `cublas`, `cudnn`, `cufft`, `cusolver`, `cusparse`, `npp`, `nvapi`, `nvcuvid`, `nvjpeg`, `nvml`, `nvrtc`).
- [ ] Commit/push all involved repositories to main before root starts any build/check/test.
- [ ] Run shared fixture/negative tests (`nv-bindgen`, `nv-loader`, `nv-bindgen-fixture`) and a relevant seeded mutant.
- [ ] Run serialized root check/clippy/full tests and affected Linux/Windows gates.
- [ ] Re-index after final source freeze and prove S01–S09 with D01 and vcv-rs.

## Cross-domain contracts

D01 depends on ReturnPolicy preserving each SDK's native status semantics while checked APIs expose
load/symbol failure. Loaded function pointers must retain the SDK ABI. Checked Rust entry points may
change our alpha API to expose errors; update every caller rather than retain a silent compatibility path.
Library handles must outlive all native pointers; CUDA handle types must remain shared.
SDK discovery and runtime hints must agree between vcv-rs CLI and nv-bindgen.

No compiler/test result, ABI parity execution, or runtime verdict is claimed by this worker.
