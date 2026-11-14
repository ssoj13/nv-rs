# D01 — remaining M2 sys crates

Updated: 2026-11-14 (UTC). Status: static Inquisitor Q1–Q7 & SDK-header audit completed; repairs written (`nvcomp-sys` `positive_enums` + `libnvcomp.so.6` + T1 ABI/enum test, `cufile-sys` T1 native ABI test); **no `cargo` or `git` commands executed by worker** — build, clippy, test, runtime and GPU gates remain pending root.
See the [domain index](../MASTERMIND_INDEX.md), [seam matrix](../SEAM_MATRIX.md) and [work queue](../../../TODO.md).

## Scope and ownership

D01 owns only these nine source files. Documentation is maintained by the documentation lane (with `D01_m2_sys.md` updated for the D01_D02 audit pass).
No `cargo` or `git` command belongs to a worker.

| Crate | Allowed source files |
|---|---|
| nvcomp-sys | `crates/nvcomp-sys/Cargo.toml`, `crates/nvcomp-sys/build.rs`, `crates/nvcomp-sys/src/nvcomp_sys.rs` |
| nccl-sys | `crates/nccl-sys/Cargo.toml`, `crates/nccl-sys/build.rs`, `crates/nccl-sys/src/nccl_sys.rs` |
| cufile-sys | `crates/cufile-sys/Cargo.toml`, `crates/cufile-sys/build.rs`, `crates/cufile-sys/src/cufile_sys.rs` |

The source manifest below was verified against source files and live SDK headers (`C:\projects\projects.rust.cg\nv\nvcomp_cuda13\nvcomp\include`, `C:\projects\projects.rust.cg\nv\nccl-2.31.2\nvidia\nccl\include`, `C:\projects\projects.rust.cg\nv\m2-linux-ref\include\cufile.h`, CUDA 13.3 toolkit, `cuda-bindings`, and `cuda-core`). Build/clippy/test execution remains pending root.

## Micro-unit symbol ledger

| # | Unit | Symbol | Source | Current evidence / verdict |
|---|---|---|---|---|
| 1 | U01 nvCOMP | SPEC | crates/nvcomp-sys/build.rs:7 | Static SDK audit pass; 16 C headers verified; 15 non-negative enums pinned in `positive_enums` (`bitcompResult_t` kept signed); build/test pending |
| 2 | U01 nvCOMP | main | crates/nvcomp-sys/build.rs:70 | Static audit pass (`cargo::error=` + exit 1); build pending |
| 3 | U01 nvCOMP | exports_sys! invocation | crates/nvcomp-sys/src/nvcomp_sys.rs:11 | Static audit pass; `DLL.linux` updated to `["libnvcomp.so.6", "libnvcomp.so"]`; build/test pending |
| 4 | U01 nvCOMP | inventory_covers_public_families | crates/nvcomp-sys/src/nvcomp_sys.rs:32 | Static audit pass (all public families + `bitcompMaxBuflen` checked); not run |
| 5 | U01 nvCOMP | streams_are_cuda_bindings_streams | crates/nvcomp-sys/src/nvcomp_sys.rs:60 | Static audit pass; not run |
| 6 | U01 nvCOMP | checked_native_and_enum_contracts_match_header_abis | crates/nvcomp-sys/src/nvcomp_sys.rs:65 | Added T1 ABI/enum/soname check; not run |
| 7 | U01 nvCOMP | native_buffer_bound_is_reported | crates/nvcomp-sys/src/nvcomp_sys.rs:78 | Static audit pass (`#[ignore]`); not run |
| 8 | U01 nvCOMP | properties_are_reported | crates/nvcomp-sys/src/nvcomp_sys.rs:89 | Static audit pass (`#[ignore]`); not run |
| 9 | U02 NCCL | SPEC | crates/nccl-sys/build.rs:7 | Static SDK audit pass; 22 headers cover `nccl.h` + Linux host `nccl_device/*` (`gin_win_stub.h` Windows-only, `barrier.h`/`cft.h` `__CUDACC__`-only); build pending |
| 10 | U02 NCCL | main | crates/nccl-sys/build.rs:52 | Static audit pass (`CARGO_CFG_TARGET_OS == "linux"` gate); build pending |
| 11 | U02 NCCL | exports_sys! invocation | crates/nccl-sys/src/nccl_sys.rs:12 | Static audit pass; `["libnccl.so.2", "libnccl.so"]`; build pending |
| 12 | U02 NCCL | inventory_covers_public_host_api | crates/nccl-sys/src/nccl_sys.rs:34 | Static audit pass; all 12 non-policy/non-pointer host fns match `nccl.h` + `nccl_device/*`; not run |
| 13 | U02 NCCL | cuda_types_are_shared | crates/nccl-sys/src/nccl_sys.rs:69 | Static audit pass (`CUstream_st`, `CUmemLocationType` verified in `cuda-bindings`); not run |
| 14 | U02 NCCL | host_queries_are_checked | crates/nccl-sys/src/nccl_sys.rs:77 | Static audit pass (matches `nccl_device/core.h`, `ll_a2a.h`, `nccl.h`); not run |
| 15 | U02 NCCL | version_is_reported | crates/nccl-sys/src/nccl_sys.rs:94 | Static audit pass (`#[ignore]`); not run |
| 16 | U02 NCCL | single_gpu_all_reduce_round_trips | crates/nccl-sys/src/nccl_sys.rs:204 | Static audit pass (`cuda_core` `Context::bind_to_thread`, `DeviceBuffer::from_host`, `Stream::synchronize` verified; `#[ignore]`); not run |
| 17 | U03 cuFile | SPEC | crates/cufile-sys/build.rs:7 | Static SDK audit pass against `m2-linux-ref/include/cufile.h` (no Windows cuFile in CUDA 13.3); build pending |
| 18 | U03 cuFile | main | crates/cufile-sys/build.rs:36 | Static audit pass (`CARGO_CFG_TARGET_OS == "linux"` gate); build pending |
| 19 | U03 cuFile | exports_sys! invocation | crates/cufile-sys/src/cufile_sys.rs:12 | Static audit pass; `["libcufile.so.0", "libcufile.so"]`; build pending |
| 20 | U03 cuFile | inventory_covers_public_api | crates/cufile-sys/src/cufile_sys.rs:30 | Static audit pass (`cuFileDriverClose` + `_v2` macro and 3 `unshimmed` fns match `cufile.h`); not run |
| 21 | U03 cuFile | cuda_types_are_shared | crates/cufile-sys/src/cufile_sys.rs:69 | Static audit pass (`CUstream_st`, `CUresult`, `cudaError_enum` verified in `cuda-bindings`); not run |
| 22 | U03 cuFile | use_count_is_checked | crates/cufile-sys/src/cufile_sys.rs:77 | Static audit pass (`cuFileUseCount() -> long` in `cufile.h:496`); not run |
| 23 | U03 cuFile | void_functions_are_checked | crates/cufile-sys/src/cufile_sys.rs:82 | Static audit pass (`cuFileHandleDeregister`, `cuFileBatchIODestroy` void in `cufile.h`); not run |
| 24 | U03 cuFile | return_policies_preserve_native_abi | crates/cufile-sys/src/cufile_sys.rs:88 | Added T1 `cuFileDriverOpen` native ABI check; not run |
| 25 | U03 cuFile | version_is_reported | crates/cufile-sys/src/cufile_sys.rs:94 | Static audit pass (`#[ignore]`); not run |
| 26 | U02R NCCL cleanup | Cleanup | crates/nccl-sys/src/nccl_sys.rs:112 | Static audit pass; explicit completion/destroy/abort results; not run |
| 27 | U02R NCCL cleanup | Cleanup::succeeded | crates/nccl-sys/src/nccl_sys.rs:119 | Static audit pass; not run |
| 28 | U02R NCCL cleanup | abort_partial_init | crates/nccl-sys/src/nccl_sys.rs:126 | Static audit pass; non-null failed-init cleanup; not run |
| 29 | U02R NCCL cleanup | finish_comm | crates/nccl-sys/src/nccl_sys.rs:134 | Static audit pass; abort-before-wait and checked destroy; not run |
| 30 | U02R NCCL cleanup | cleanup_orders_abort_wait_and_destroy_without_hiding_failures | crates/nccl-sys/src/nccl_sys.rs:157 | Static audit pass (6 sequences); not run |
| 31 | U02R NCCL cleanup | failed_initialization_aborts_only_a_non_null_partial_communicator | crates/nccl-sys/src/nccl_sys.rs:184 | Static audit pass (4 states); not run |

U01R repairs the same nvCOMP SPEC; U02R adds six cleanup/helper declarations to the original inventory, plus the added ABI/enum checks (`nvcomp_sys.rs:65`, `cufile_sys.rs:88`). D02 received bounded ownership for the NCCL/cuFile checked-call and cleanup repairs; no concurrent write grant is implied.

## Written behavior, not yet verified at runtime

- nvCOMP SPEC names 16 public/native-bitcomp C headers (all C headers in `nvcomp_cuda13/nvcomp/include`; `gdeflate_cpu.h` and `*.hpp` are C++), separate nvCOMP/bitcomp status policies, and all 15 non-negative enums pinned in `positive_enums` (`bitcompResult_t` kept signed); Linux sonames try `"libnvcomp.so.6"` before `"libnvcomp.so"`.
- NCCL SPEC names 22 headers covering `nccl.h` and host-side `nccl_device/*` declarations (Windows-only `gin_win_stub.h` and pure `__CUDACC__` device headers excluded). Core non-status queries use the shared checked-return path.
- cuFile SPEC separates structured `CUfileError_t` status from negative `ssize_t` I/O returns; `cuFileUseCount`, `cuFileHandleDeregister`, and `cuFileBatchIODestroy` are checked (`Result<_, CallError>`).
- All three crates consume the shared generator/loader and reuse the specified `cuda-bindings` types.
- NCCL and cuFile have Linux target gates; absence of those APIs on another platform is not a Linux test pass.

These are static source and header observations. Generated code compilation, clippy, ABI/layout assertions at runtime, and GPU behavior still require root execution.

## Required repairs and Inquisitor ratchet

| Gate | Finding / required action | Status |
|---|---|---|
| nvCOMP cross-OS enums | `nvcomp-sys/build.rs:33` pinned all 15 non-negative nvCOMP/bitcomp enums (`bitcompResult_t` kept signed); added `nvcomp_sys.rs:65` check | repaired statically; compile/test pending |
| nvCOMP Linux soname | `nvcomp_sys.rs:22` updated to `["libnvcomp.so.6", "libnvcomp.so"]` matching `NVCOMP_VER_MAJOR = 6` | repaired statically; compile/test pending |
| cuFile native ABI check | Added `cufile_sys.rs:88` `return_policies_preserve_native_abi` | repaired statically; compile/test pending |
| NCCL initialization | `abort_partial_init` releases a non-null partial handle before the failing assertion | static audit pass; execution pending |
| NCCL completion | `finish_comm` aborts failed work before wait, skips wait after failed abort, and checks destroy/abort outcomes | static audit pass; execution pending |
| Shared void calls | D02 generates checked `Result<()>`; NCCL and cuFile call-shape fixtures consume it | static audit pass; execution pending |
| Q1 unfinished code | Inspected all 9 files in D01: zero stubs, `TODO`/`FIXME`, `todo!`/`unimplemented!`, or dead non-test code | static audit pass |
| Q2 live wiring | Traced `build.rs` SPEC -> `exports_sys!` -> `DLL` / `INVENTORY` / `load()` -> tests; all wired cleanly | static audit pass; compile/test pending |
| Q3 errors/panics | Zero non-test `unwrap`/`expect`/`panic!`; `nccl_sys.rs` tests use explicit `Cleanup`/`Result`; all non-test lints clean (`//!` headers, `// SAFETY:` comments in tests) | static audit pass |
| Q4 boundaries | Checked negative `bitcompResult_t`, `cuFileDriverClose`/`_v2`, Linux `cfg` gates, empty/null pointer guards | static audit pass |
| Q5 allocations | `exports_sys!` caches via `OnceLock`; zero hot-path allocations in sys shims | static audit pass |
| Q6 layout | Verified `positive_enums`, `CUstream_st`/`CUresult`/`cudaError_enum`/`CUmemLocationType` reuse, and C signatures vs installed SDK headers | static audit pass; T1 layout tests pending |
| Q7 assumptions | Verified `OnceLock` library lifetimes, `vcv-rs` `NV_SDK_DIRS` discovery, and platform gates (`#![cfg(target_os = "linux")]` + `Cargo.toml` target deps) | static audit pass; execution pending |

## Reference discovery evidence

Root reports an independent NCCL reference probe on Ubuntu 26.04 / glibc 2.43: runtime version
`23102` and all inspected ELF exports resolved. This was not a build or test of these crates.
The NVIDIA manylinux wheel SHA-256 is
`0bcaf0308854cb55fcc35af72e2c83143f3b71e65a4e865e2c586b1cdcdb5ae0`.
Ubuntu 24.04 debs were unused and have been deleted. Keep SDK/reference payloads outside this repository.

## Root gates and seams

- [ ] Finish canonical vcv-rs Windows/Linux discovery and consume its pushed main revision.
- [ ] Commit/push every involved repository to main before any build/check/test.
- [ ] Run fixtures, narrow checks, serialized workspace gates and a relevant seeded mutant.
- [ ] Generate/check coverage inventories against actual SDK versions and supported targets.
- [ ] Prove SDK discovery, CUDA type identity, error policies and library lifetime seams with D02.
- [ ] Run available runtime/GPU tests; record unavailable platforms explicitly.
- [ ] Obtain native Linux GPUDirect Storage and multi-GPU evidence separately where required.
- [ ] Refresh the final graph after source freeze; the reported incremental reindex is not the final gate.
