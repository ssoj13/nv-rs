# D04 — Dense Linear Algebra, FFT & RNG (M7a)

Updated: 2026-11-14. Status: source written, self-audited, and all 5 Inquisitor-3 findings repaired; build, clippy, and T0/T3 execution pending root build gate.

## Scope and ownership

D04 owns `crates/cublas/**`, `crates/cufft/**`, `crates/curand/**`, and this report. Workspace `Cargo.toml` and other crates were not modified.

| Crate | Files | Wraps |
|---|---|---|
| `cublas` | `Cargo.toml`, `src/cublas.rs`, `src/error.rs`, `src/scratch.rs`, `src/handle.rs`, `src/gemm.rs`, `src/lt.rs` | `cublas-sys`, `cublaslt-sys`, `cuda-core` |
| `cufft` | `Cargo.toml`, `src/cufft.rs`, `src/error.rs`, `src/scratch.rs`, `src/plan.rs` | `cufft-sys`, `cuda-core` |
| `curand` | `Cargo.toml`, `src/curand.rs`, `src/error.rs`, `src/generator.rs` | `cuda-bindings` (cuRAND module), `cuda-core` |

## Implemented surface

- **`cublas`**:
  - `Handle` (`cublasCreate_v2`/`cublasDestroy_v2`), `set_stream`, `set_pointer_mode`/`pointer_mode`, `set_math_mode`/`math_mode`, `set_workspace`/`clear_workspace`, `version`.
  - `gemm`: `sgemm`, `dgemm`, `sgemm_strided_batched`, `dgemm_strided_batched`, `unsafe sgemm_batched`/`dgemm_batched`, `unsafe gemm_ex`/`gemm_strided_batched_ex` with column-major leading-dimension and extent validation.
  - `lt`: `LtHandle`, `MatmulDesc`, `MatrixLayout`, `Preference`, `HeuristicResult`, `matmul_heuristic`/`matmul_heuristics`, `matmul_f32`, `unsafe matmul_with_scratch` with caller-owned `Scratch` and byte-budget enforcement.
- **`cufft`**:
  - `Plan` (`plan_1d`, `plan_2d`, `plan_3d`, `plan_many`, `estimate_1d/2d/3d`) with `cufftSetAutoAllocation(0)` and RAII `RawHandle` guard during two-phase creation.
  - `Scratch` caller-owned workspace with byte-budget checks (`Error::Budget`) and `set_work_area`.
  - `exec_c2c`, `exec_r2c`, `exec_c2r`, `exec_z2z`, `exec_d2z`, `exec_z2d` with transform-type and minimum-element checks and `set_stream`.
- **`curand`**:
  - `Generator::new_device` and `Generator::new_host`, `RngType` (Philox4_32_10, MTGP32, MT19937, Xorwow, Mrg32k3a, Sobol32/64, ScrambledSobol32/64), `Ordering`, `PoissonDistribution`.
  - `set_stream`, `set_seed` (pseudo-only), `set_offset`, `set_ordering`, `set_dimensions` (quasi-only), `generate_seeds`.
  - Device and host generation for `u32`, `u64`, `uniform f32/f64`, `normal f32/f64`, `log_normal f32/f64`, `poisson u32`.

## Tests written (unexecuted — no cargo/git run by worker)

- **T0 unit tests**:
  - `cublas`: `Status` newtype copy/unknown-code formatting, `Error` formatting, `validate_gemm_dims`, `validate_strided_batch`, `DataType::element_bytes`.
  - `cufft`: `FftResult` newtype copy/unknown-code formatting, `Error` formatting, `ManyDesc::validate`, `dense_io_elems`.
  - `curand`: `CurandStatus` newtype copy/unknown-code formatting, `Error` formatting, `RngType` classification, `validate_normal_params`, `validate_poisson_lambda`.
- **T3 GPU tests (`#[ignore]`)**:
  - `cublas::tests::gpu_sgemm_and_dgemm_match_cpu_reference`, `cublas::tests::gpu_cublaslt_matmul_and_budget_rejection`.
  - `cufft::tests::gpu_1d_and_2d_fft_roundtrip_and_parseval`.
  - `curand::tests::gpu_philox_sobol_mtgp32_statistical_sanity_and_host_generator`.

## Self-audit against the 7 Inquisitor Questions

| Question | Result | Notes |
|---|---|---|
| Q1 Unfinished code / missing docs | Pass (repaired) | Zero `TODO`/stubs; `///` on all `pub` items; removed unused `half` dep from `cublas/Cargo.toml`. |
| Q2 Live wiring | Pass | All modules exported from crate roots (`cublas.rs`, `cufft.rs`, `curand.rs`). |
| Q3 Errors & panics | Pass | `Copy` integer newtype status codes (`Status`, `FftResult`, `CurandStatus`), `thiserror::Error`, `Drop` records teardown failures on `Context::record_err`. |
| Q4 Boundaries | Pass (repaired) | Checked overflow in GEMM/FFT extent math; fixed `lt::matmul_with_scratch` to validate stream match before `Scratch::reserve`; RAII `RawHandle` prevents `cufftHandle` leaks on budget rejection. |
| Q5 Allocations | Pass | Zero heap allocations on GEMM, cuBLASLt matmul, cuFFT exec/plan_many (`[i32; 3]` stack arrays), and cuRAND generation hot paths. |
| Q6 Layout / ABI | Pass | Compile-time `const` asserts for `cufftComplex == [f32; 2]` and `cufftDoubleComplex == [f64; 2]`; `#[repr(transparent)]` on status newtypes and `HeuristicResult`. |
| Q7 Assumptions / lifetimes | Pass | Handles are `Send + !Sync`; `Arc::ptr_eq` enforces context/stream ownership across handles and `Scratch`. |

## Inquisitor-3 findings repaired (#24 / #26)

1. Pointer-mode & `matmul_f32` validation: tracked `PointerMode` on `Handle` (`require_host_pointer_mode` in all safe GEMM wrappers) and `MatmulDesc` + `MatrixLayout` geometry/`DataType` validation in `LtHandle::matmul_f32`.
2. `gemm_ex` / `gemm_strided_batched_ex` overflow: replaced unchecked element-to-byte multiplications with `checked_bytes` (`checked_mul`).
3. `LtHandle::matmul_heuristic` / `matmul_heuristics`: added `self.cuda.bind_to_thread()?`, added `HeuristicResult::is_valid()`, and filtered `matmul_heuristics` to `CUBLAS_STATUS_SUCCESS`.
4. `cufft::Plan::plan_many`: updated `ManyDesc::validate` (`embed[i] >= req_dim[i]` for `i >= 1`) and `ManyDesc::min_io_elems()` to ignore `embed[0]` and compute the exact strided/batched span using checked arithmetic, with T0 tests.
5. `curand`: added `ctx.bind_to_thread()?` in `set_seed`, `set_offset`, `set_ordering`, `set_dimensions`; added `PoissonDistribution::new_on` + `destroy()` + checked `Drop` (and non-silent host `Drop` reporting).
