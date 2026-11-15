# D05 — sparse & solver safe wrappers (`cusparse`, `cusolver`)

Updated: 2026-11-13. Status: source and tests written; root `cargo` build/clippy/GPU verification pending (workers do not run `cargo` or `git`).
See the [domain index](../MASTERMIND_INDEX.md), [seam matrix](../SEAM_MATRIX.md), and [plan](../../PLAN.md).

## Scope and ownership

D05 (M7b) owns only:
- `crates/cusparse/Cargo.toml`, `crates/cusparse/src/cusparse.rs`, `crates/cusparse/src/handle.rs`, `crates/cusparse/src/descr.rs`, `crates/cusparse/src/ops.rs`, `crates/cusparse/tests/gpu.rs`
- `crates/cusolver/Cargo.toml`, `crates/cusolver/src/cusolver.rs`, `crates/cusolver/src/dn.rs`, `crates/cusolver/src/sp.rs`, `crates/cusolver/src/mg.rs`, `crates/cusolver/tests/gpu.rs`
- `docs/mastermind/domains/D05_sparse_solvers.md`

## Micro-unit symbol ledger

| # | Crate | Module / File | Key symbols | Status |
|---|---|---|---|---|
| 1 | `cusparse` | `src/cusparse.rs` | `Status`, `Error`, `DataType`, `IndexType`, `IndexBase`, `Op`, `Order`, `FillMode`, `DiagType`, `PointerMode`, `Scratch` | Written; T0 tests included |
| 2 | `cusparse` | `src/handle.rs` | `Handle` (`new`, `set_stream`, `raw_stream`, `set_pointer_mode`, `pointer_mode`, `version`, `Drop`) | Written |
| 3 | `cusparse` | `src/descr.rs` | `SpMatDescr` (CSR/CSC/COO/BlockedEll, `csr_empty`, `set_csr_pointers`, `query_size`, `set_fill_mode`, `set_diag_type`), `DnVecDescr`, `DnMatDescr`, `validate_*` | Written; T0 validation tests included |
| 4 | `cusparse` | `src/ops.rs` | `spmv_buffer_size`, `spmv`, `spmm_buffer_size`, `spmm`, `SpGemm` (`work_estimation`, `compute`, `copy`), `SpSv`, `SpSm` | Written; T0 dimension tests included |
| 5 | `cusparse` | `tests/gpu.rs` | `spmv_csr_f32_matches_reference`, `spmm_csr_f32_matches_reference`, `spgemm_csr_f32_matches_reference_and_enforces_budget`, `spsv_and_spsm_lower_triangular_solve` | `#[ignore]` T3 GPU tests written |
| 6 | `cusolver` | `src/cusolver.rs` | `Status`, `Error`, `FillMode`, `Op`, `EigMode`, `Scratch` (`reserve_bytes`, `reserve_elements`) | Written; T0 tests included |
| 7 | `cusolver` | `src/dn.rs` | `DnHandle`, `DnFloat` (`f32`/`f64`), `SvdJob`, `getrf`, `getrs`, `potrf`, `potrs`, `geqrf`, `orgqr`, `gesvd`, `syevd`, `validate_*` | Written; T0 validation tests included |
| 8 | `cusolver` | `src/sp.rs` | `SpHandle`, `MatDescr`, `SpFloat` (`f32`/`f64`), `csrlsvqr`, `csreigvsi`, `validate_csr_solve` | Written; T0 validation tests included |
| 9 | `cusolver` | `src/mg.rs` | `MgHandle`, `MgGrid`, `MgMatrixDesc`, `MgGridMapping`, `MgDataType`, `potrf*`, `potrs*`, `getrf*`, `getrs*`, `check_mg_work_budget` | Written; T0 validation & budget tests included |
| 10 | `cusolver` | `tests/gpu.rs` | `dn_lu_getrf_getrs_solves_and_checks_residual`, `dn_cholesky_potrf_potrs_solves_spd_system`, `dn_qr_geqrf_orgqr_checks_orthogonality`, `dn_svd_gesvd_reconstructs_and_checks_orthogonality`, `dn_syevd_symmetric_eigen_residual`, `sp_csrlsvqr_and_csreigvsi_numerical_checks` | `#[ignore]` T3 GPU tests written |

## Self-audit against the 7 Inquisitor Questions

| Gate | Finding / verification | Status |
|---|---|---|
| Q1 Unfinished code | Scanned `src/*.rs` for `TODO`/`FIXME`/`unimplemented!`/`todo!`; none present. | Pass (static) |
| Q2 Live wiring | Both crates wire `[lib] path = "src/<name>.rs"`, `[lints] workspace = true`, `*-sys` crates, `cuda-core`, and `tests/gpu.rs`. | Pass (static) |
| Q3 Errors / panics | Zero `.unwrap()`/`.expect()` in `src/*.rs`; all `Drop` impls record teardown failures on `cuda_core::Context::record_err`; `MatDescr::new` cleans up on partial failure. | Pass (static) |
| Q4 Boundaries | `checked_add`/`checked_mul` on all byte/element sizes; negative dimensions, `ld < minor_dim`, `m < n` in `gesvd`, `k > n \|\| n > m` in `orgqr`, and `requested > budget` rejected before FFI. | Pass (static) |
| Q5 Allocations | `Scratch` is stream-bound and reused across operations with zero steady-state allocations once at high-water mark. | Pass (static) |
| Q6 FFI & lints | Signatures and enum constants verified against CUDA 13.3 headers (`cusparse.h`, `cusolverDn.h`, `cusolverSp.h`, `cusolverMg.h`, `cusolver_common.h`, `library_types.h`). Every `unsafe` block has a `// SAFETY:` comment; every `pub` item is documented. | Pass (static) |
| Q7 Lifetimes | Child handles/descriptors hold `Arc<Context>` (or `Arc<MgGrid>`); `Scratch` and `set_stream` validate stream/context identity via `Arc::ptr_eq`. | Pass (static) |
