# Wave 6: `device/` (2026-11-23)

Scope: `device/` only (separate cargo workspace, toolchain 1.99.1, cuda-oxide fork `ssoj13/cuda-rust_b` `42cc04e`). RTX 3080 Ti (sm_86), CUDA 13.3, OptiX 9.1. Nothing committed; nothing outside `device/` edited except this file.

## 1. Duplicated crates (`cargo tree -d -e normal,build`, inside `device/`)

Before: four crates were in the graph twice, once from each repository:

| Crate | NVIDIA/cuda-rust `348895e` pulled by | ssoj13/cuda-rust_b `42cc04e` pulled by |
|---|---|---|
| `cuda-core` 0.5.0 | `crates/cuda-core-ext`, `crates/optix` | `cuda-host`, `optix-device-tests` |
| `cuda-bindings` 0.5.0 | `cuda-core` (NVIDIA), `cuda-core-ext`, `optix-sys` | `cuda-core` (fork) |
| `cuda-core-derive` 0.5.0 | `cuda-core` (NVIDIA) | `cuda-core` (fork) |
| `cuda-repr` 0.5.0 | `cuda-core` (NVIDIA) | `cuda-core` (fork), `cuda-device` |

After: none of them is duplicated. `cuda-core v0.5.0 (ssoj13/cuda-rust_b#42cc04ed)` is the only one, and `cuda-core-ext`, `cuda-host`, `optix` and `optix-device-tests` all use it. `device/Cargo.lock` no longer contains `NVIDIA/cuda-rust` (0 matches). The only duplicates left are `hashbrown` and `object` from crates.io, which have nothing to do with this.

Feasibility check (why the patch is safe): between NVIDIA `348895e` and the fork `42cc04e`:
- `cuda-core` differs only in `build.rs`, `src/embedded.rs` and two tests.
- `cuda-repr` and `cuda-core-derive` are identical.
- `cuda-bindings` differs in how it is built: the fork commits pre-generated bindings (`generated/`, CUDA 13.3, `cuda_version` 13030 in `meta.json`) and drops `cuda_toolkit_dir()`. No crate under `crates/` calls `cuda_toolkit_dir`.

None of the root crates needs an API that only NVIDIA's revision has.

## 2. Patch (`device/Cargo.toml`)

```toml
[patch."https://github.com/NVIDIA/cuda-rust.git"]
cuda-bindings = { git = "https://github.com/ssoj13/cuda-rust_b.git", branch = "main" }
cuda-core = { git = "https://github.com/ssoj13/cuda-rust_b.git", branch = "main" }
cuda-core-derive = { git = "https://github.com/ssoj13/cuda-rust_b.git", branch = "main" }
cuda-repr = { git = "https://github.com/ssoj13/cuda-rust_b.git", branch = "main" }
```

A comment above the section explains why. The root workspace stays on NVIDIA upstream.

## 3. Build steps and results

All commands ran in `device/optix-device-tests` unless noted.

- `cargo oxide build --arch sm_86`: exit 0, no warnings (first build 45 s, final 23 s).
- `cargo oxide run --arch sm_86`: exit 0, 15/15 PASS (first run, before any other change).
- `cargo clippy --workspace --all-targets`, run in `device/` (workspace lints: `clippy::all` and `undocumented_unsafe_blocks` deny, `missing_docs` warn): exit 0, no diagnostics.
- Final `cargo oxide build` + `cargo oxide run` after all changes: exit 0, 15/15 PASS.

## 4. GPU cases (final run, sm_86)

No case failed in any run, so nothing needed a re-run.

| # | Case | Initial run | Final run | Note |
|---|---|---|---|---|
| 1 | ptr64 | PASS | PASS | |
| 2 | shade | PASS | PASS | |
| 3 | geometry | PASS | PASS | |
| 4 | transform | PASS | PASS | Output: "14 probes ..."; still the key-0 driver behaviour (D9) |
| 5 | hitobject | PASS | PASS | |
| 6 | intersect | PASS | PASS | |
| 7 | exception | PASS | PASS | |
| 8 | callables | PASS | PASS | Now also covers `fn(u16, i16) -> u16` (D2); stack `continuation: 32` |
| 9 | payload_types | PASS | PASS | |
| 10 | symbols | PASS | PASS | |
| 11 | const_index | PASS | PASS | |
| 12 | coop_vec | PASS | PASS | |
| 13 | texture | PASS | PASS | |
| 14 | sparse | PASS | PASS | |
| 15 | footprint | PASS | PASS | Now writes to stack locals (D7) |

## 5. Ledger (D1..D9)

| ID | Verdict | Evidence | Change | Test |
|---|---|---|---|---|
| D1 | CONFIRMED, FIXED | `cargo tree -d` showed 2x `cuda-core`/`cuda-bindings`/`cuda-core-derive`/`cuda-repr` (section 1) | `[patch]` in `device/Cargo.toml`; `device/Cargo.lock` updated | Device build + 15/15 GPU cases |
| D2 | CONFIRMED, FIXED | The new u16 callable compiled to `.visible .func (.param .b32 func_retval0) __direct_callable__narrow(.param .b32, .param .b32)`, but the caller emitted `.callprototype (.param .b16 _) _ (.param .b16 _, .param .b16 _)`. On the GPU: `thread 0 direct callable (u16, i16) -> u16: got 3060 want 65517` | `optix_device_macros.rs`: 16-bit types use the `r`/`.b32` class (new `is_16bit`); arguments are widened with `as u32` (zero-extends u16, sign-extends i16); the result comes back in a `u32` and is truncated to the declared type | `callables` adds `__direct_callable__narrow(u16, i16) -> u16` (SBT callable 3) and checks every thread. Passes; the PTX has no `.b16` prototype left |
| D3 | CONFIRMED, fix as proposed not possible; documented | The expansion uses the absolute paths `::optix_device::raw` / `::cuda_device::ptx_asm!`. But `ptx_asm!`'s own expansion (fork `cuda-macros/src/ptx_asm.rs:288,332,369,380`) names `cuda_device::ptx::...`, so re-exporting through `optix_device` would not remove the `cuda-device` dependency. Every OptiX program crate depends on `cuda-device` anyway (`#[kernel]`). | Module doc states the requirement and the reason | n/a (doc only) |
| D4 | CONFIRMED, FIXED | Removing `#[allow(missing_docs, clippy::all)]` from `raw` gives a clean clippy. On `sys`, `#[expect(...)]` showed `dead_code` as unfulfilled (`optix_device.rs:64:86`); the other six lints fire | `raw`: allow removed. `sys`: `#[allow(7 lints)]` replaced by `#[expect(missing_docs, non_camel_case_types, non_upper_case_globals, non_snake_case, clippy::all, clippy::undocumented_unsafe_blocks, reason = "bindgen output")]` | Clippy on the device workspace is clean; oxide build is clean |
| D5 | CONFIRMED, FIXED | `API_MAP.toml:14-15` said there was no wrapper, but `:220` maps `optixGetGASPointerFromHandle = "transform::gas_pointer"`, and `transform.rs:510` defines `pub unsafe fn gas_pointer` | Stale comment lines removed | `cargo xtask coverage optix` not run (root workspace, off limits) |
| D6 | CONFIRMED, FIXED | `payload::word`/`set_word` had no index check | `const { assert!(I < 32, "OptiX has 32 payload registers") }` in both | A temporary `payload::word::<40>()` failed the build with `E0080: evaluation panicked: OptiX has 32 payload registers`, then was reverted |
| D7 | CONFIRMED (stale doc), FIXED | `footprint` case changed to stack-local `Footprint2d` and flag. The PTX of `__raygen__fp` has `.local .align 16 .b8 __local_depot50[32]` and `cvta.local.u64 %SP` (64-bit generic address passed to the intrinsic). OptiX accepts it and every probe matches | `footprint.rs` doc: global memory or stack locals both work under the 64-bit pointer build | `footprint` PASS using locals |
| D8 | CONFIRMED, partly fixed | `git ls-files device` lists `device/.optix_device_tests.oxide-pipeline.lock`. `.oxide-artifacts/` was not ignored | `device/.gitignore`: `/.*.oxide-pipeline.lock`, `/.oxide-artifacts` (`git check-ignore` confirms both) | The tracked lock still needs `git rm --cached device/.optix_device_tests.oxide-pipeline.lock`. I did not run it because it stages an index change; the lead should decide |
| D9 | CONFIRMED, FIXED | `optix_device.h:552` says "at the current ray time". The wrapper (`geometry.rs:89` -> `_optix_get_triangle_vertex_data_current_hit`, no inputs) matches `optix_device_impl.h` exactly. The case pinned key 0 as the only correct answer | `transform.rs`: accepts key 0 (observed) or the vertices at the ray time (documented); prints a line when the documented behaviour appears | `transform` PASS; no "documented behaviour" line was printed, so driver 621.24 still returns key 0 |

## 6. `git status --short device`

```
 M device/.gitignore
 M device/Cargo.lock
 M device/Cargo.toml
 M device/optix-device-macros/src/optix_device_macros.rs
 M device/optix-device-tests/src/cases/calls.rs
 M device/optix-device-tests/src/cases/footprint.rs
 M device/optix-device-tests/src/cases/transform.rs
 M device/optix-device/API_MAP.toml
 M device/optix-device/src/footprint.rs
 M device/optix-device/src/optix_device.rs
 M device/optix-device/src/payload.rs
```

## 7. Risks

- **The fork now also builds `optix` and `cuda-core-ext` inside `device/`.** Its `cuda-bindings` uses committed CUDA 13.3 bindings instead of local bindgen, so the device harness exercises the host crates against slightly different `cuda-bindings`/`cuda-core` build plumbing than the root workspace does. They are API-identical at the two revisions. If the root later adopts an NVIDIA-only `cuda-core`/`cuda-bindings` API, the device build breaks and the fork must be rebased (or the patch dropped once the cuda-oxide patches are upstream).
- **The fork is tracked by `branch = "main"`.** `device/Cargo.lock` pins `42cc04e`, and `cargo update` would move it.
- **D2 changes the macro ABI for 16-bit types.** Any existing caller that relied on `.b16` was already broken (wrong values), so nothing that worked can regress.
- **D3 is not "fixed".** Calling crates must depend on `cuda-device` by that name. A renamed dependency (`package = "cuda-device"` under another name) breaks the expansion, as it does for `ptx_asm!` itself.
- **`#[expect(clippy::all)]` on `sys`.** If bindgen output changes so that no clippy lint fires, clippy reports an unfulfilled expectation (a warning). That is intended: it signals the expect can be narrowed.
- **D8 is incomplete** until the tracked lock file is removed from the index.
- **The D5 coverage gate (`cargo xtask coverage optix`) was not run** because it lives in the root workspace.
- **No CI check for the device build was added** (D1 fix direction). The device workspace has no CI config in scope.
