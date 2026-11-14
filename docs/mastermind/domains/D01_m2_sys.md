# D01 — remaining M2 sys crates

Updated: 2026-11-13. Status: source written; cleanup/checked-return repairs are present, independent review and execution remain pending.
See the [domain index](../MASTERMIND_INDEX.md), [seam matrix](../SEAM_MATRIX.md) and [work queue](../../../TODO.md).

## Scope and ownership

D01 owns only these nine source files. Documentation is maintained by the documentation lane.
No compiler or test command belongs to a worker.

| Crate | Allowed source files |
|---|---|
| nvcomp-sys | `crates/nvcomp-sys/Cargo.toml`, `crates/nvcomp-sys/build.rs`, `crates/nvcomp-sys/src/nvcomp_sys.rs` |
| nccl-sys | `crates/nccl-sys/Cargo.toml`, `crates/nccl-sys/build.rs`, `crates/nccl-sys/src/nccl_sys.rs` |
| cufile-sys | `crates/cufile-sys/Cargo.toml`, `crates/cufile-sys/build.rs`, `crates/cufile-sys/src/cufile_sys.rs` |

The source manifest below was checked against live declarations. This establishes inventory,
not functional parity or a review verdict. File/line references may move during required repairs.

## Micro-unit symbol ledger

| # | Unit | Symbol | Source | Current evidence / verdict |
|---|---|---|---|---|
| 1 | U01 nvCOMP | SPEC | crates/nvcomp-sys/build.rs:7 | Written; independent review pending |
| 2 | U01 nvCOMP | main | crates/nvcomp-sys/build.rs:53 | Written; independent review pending |
| 3 | U01 nvCOMP | exports_sys! invocation | crates/nvcomp-sys/src/nvcomp_sys.rs:11 | Written; independent review pending |
| 4 | U01 nvCOMP | inventory_covers_public_families | crates/nvcomp-sys/src/nvcomp_sys.rs:32 | Test written; not run |
| 5 | U01 nvCOMP | streams_are_cuda_bindings_streams | crates/nvcomp-sys/src/nvcomp_sys.rs:60 | Test written; not run |
| 6 | U01 nvCOMP | native_buffer_bound_is_reported | crates/nvcomp-sys/src/nvcomp_sys.rs:66 | Test written; not run |
| 7 | U01 nvCOMP | properties_are_reported | crates/nvcomp-sys/src/nvcomp_sys.rs:77 | Test written; not run |
| 8 | U02 NCCL | SPEC | crates/nccl-sys/build.rs:7 | Written; independent review pending |
| 9 | U02 NCCL | main | crates/nccl-sys/build.rs:52 | Written; independent review pending |
| 10 | U02 NCCL | exports_sys! invocation | crates/nccl-sys/src/nccl_sys.rs:12 | Written; independent review pending |
| 11 | U02 NCCL | inventory_covers_public_host_api | crates/nccl-sys/src/nccl_sys.rs:34 | Test written; not run |
| 12 | U02 NCCL | cuda_types_are_shared | crates/nccl-sys/src/nccl_sys.rs:69 | Test written; not run |
| 13 | U02 NCCL | host_queries_are_checked | crates/nccl-sys/src/nccl_sys.rs:77 | Includes checked void declarations; not run |
| 14 | U02 NCCL | version_is_reported | crates/nccl-sys/src/nccl_sys.rs:94 | Test written; not run |
| 15 | U02 NCCL | single_gpu_all_reduce_round_trips | crates/nccl-sys/src/nccl_sys.rs:204 | Counts 1/4 and cleanup repair written; not run |
| 16 | U03 cuFile | SPEC | crates/cufile-sys/build.rs:7 | Written; independent review pending |
| 17 | U03 cuFile | main | crates/cufile-sys/build.rs:36 | Written; independent review pending |
| 18 | U03 cuFile | exports_sys! invocation | crates/cufile-sys/src/cufile_sys.rs:12 | Written; independent review pending |
| 19 | U03 cuFile | inventory_covers_public_api | crates/cufile-sys/src/cufile_sys.rs:30 | Test written; not run |
| 20 | U03 cuFile | cuda_types_are_shared | crates/cufile-sys/src/cufile_sys.rs:69 | Test written; not run |
| 21 | U03 cuFile | use_count_is_checked | crates/cufile-sys/src/cufile_sys.rs:77 | Test written; not run |
| 22 | U03 cuFile | version_is_reported | crates/cufile-sys/src/cufile_sys.rs:89 | Test written; not run |
| 23 | U02R NCCL cleanup | Cleanup | crates/nccl-sys/src/nccl_sys.rs:112 | Explicit completion/destroy/abort results; review pending |
| 24 | U02R NCCL cleanup | Cleanup::succeeded | crates/nccl-sys/src/nccl_sys.rs:119 | Source predicate; not run |
| 25 | U02R NCCL cleanup | abort_partial_init | crates/nccl-sys/src/nccl_sys.rs:126 | Non-null failed-init handle cleanup written; review pending |
| 26 | U02R NCCL cleanup | finish_comm | crates/nccl-sys/src/nccl_sys.rs:134 | Abort-before-wait and checked destruction sequence written; review pending |
| 27 | U02R NCCL cleanup | cleanup_orders_abort_wait_and_destroy_without_hiding_failures | crates/nccl-sys/src/nccl_sys.rs:157 | Six fixture sequences written; not run |
| 28 | U02R NCCL cleanup | failed_initialization_aborts_only_a_non_null_partial_communicator | crates/nccl-sys/src/nccl_sys.rs:184 | Four initialization states written; not run |

U01R repairs the same nvCOMP SPEC; U02R adds six cleanup/helper declarations to the original 22-row inventory. D02 received bounded ownership for the NCCL/cuFile checked-call and cleanup repairs; no concurrent write grant is implied.

## Written behavior, not yet verified

- nvCOMP SPEC names 16 public/native-bitcomp headers and separate nvCOMP/bitcomp status policies.
- NCCL SPEC names 22 headers, public host/profiling entry points and device-layout declarations. Core non-status queries use the shared checked-return path.
- cuFile SPEC separates structured status from negative I/O returns; the use-count query is checked.
- All three crates consume the shared generator/loader and reuse the specified CUDA types.
- NCCL and cuFile have Linux target gates; absence of those APIs on another platform is not a Linux test pass.

These are static source observations. Generated declarations, ABI/layout parity, full header coverage,
runtime errors and GPU behavior still need independent review and execution evidence.

## Required repairs and Inquisitor ratchet

| Gate | Finding / required action | Status |
|---|---|---|
| NCCL initialization | abort_partial_init releases a non-null partial handle before the failing assertion | written; review/execution pending |
| NCCL completion | finish_comm aborts failed work before wait, skips wait after failed abort, and checks destroy/abort outcomes | written; review/execution pending |
| Shared void calls | D02 generates checked Result<()>; NCCL and cuFile call-shape fixtures consume it | written; review/execution pending |
| Q1 unfinished code | Fresh read-only review of every manifest row and adjacent paths | pending |
| Q2 live wiring | Trace build SPEC, generated exports, checked calls and tests | pending |
| Q3 errors/panics | Review FFI failure paths and all resource cleanup | pending |
| Q4 boundaries | Review status/value semantics, empty/invalid inputs and overflow | pending |
| Q5 allocations | Review runtime boundaries for steady-state churn | pending |
| Q6 layout | Compare generated FFI types/alignment to authoritative SDK headers | pending |
| Q7 assumptions | Prove target, handle, library and stream lifetime contracts | pending |

No fresh independent Inquisitor pass or zero-defect verdict exists. Any finding must be repaired
and reviewed again before this domain is verified.

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
