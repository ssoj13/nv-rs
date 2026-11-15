---
title: Linear Algebra & Math
description: Safe Rust wrappers and raw sys bindings for cuBLAS, cuBLASLt, cuFFT, cuRAND, cuSOLVER, and cuSPARSE.
sidebar:
  order: 1
---

The linear algebra and math layer provides safe RAII host wrappers on top of `cuda-core` and build-time-generated `*-sys` crates (or `cuda-bindings` for `curand`). Across these crates, handles own their native objects, hold `Arc<cuda_core::Context>`, record any `Drop` teardown failure on the CUDA context rather than panicking, wrap status codes in `Copy` integer newtypes, and enqueue stream-ordered work with zero heap allocations.

## Crate Overview

| Safe Crate | Raw Bindings | Scope |
|---|---|---|
| `cublas` | `cublas-sys`, `cublaslt-sys` | Dense BLAS and cuBLASLt matmul APIs |
| `cufft` | `cufft-sys` | 1D/2D/3D and batched Fast Fourier Transforms |
| `curand` | `cuda-bindings` | Host and device pseudo/quasi-random number generation |
| `cusolver` | `cusolver-sys`, `cusolvermg-sys`, `cusparse-sys` | Dense (`Dn`), sparse (`Sp`), and multi-GPU (`Mg`) solvers |
| `cusparse` | `cusparse-sys` | Sparse matrix/vector descriptors and SpMV/SpMM/SpGEMM/SpSV/SpSM |

---

## `cublas` (`cublas-sys`, `cublaslt-sys`)

Safe cuBLAS and cuBLASLt host API on top of `cublas-sys`, `cublaslt-sys`, and `cuda-core`.

- **Handles & configuration (`handle`)**: `Handle`, `Operation`, `PointerMode`, `MathMode`, `DataType`, `ComputeType`, and `GemmAlgo`.
- **GEMM operations (`gemm`)**: `sgemm`, `dgemm`, `sgemm_batched`, `dgemm_batched`, `sgemm_strided_batched`, `dgemm_strided_batched`, and `gemm_ex` / `gemm_strided_batched_ex`.
- **cuBLASLt (`lt`)**: `LtHandle`, `MatmulDesc`, `MatrixLayout`, `MatrixTransformDesc`, `EmulationDesc`, `Preference`, `MatmulAlgo`, `HeuristicResult`, `Order`, `Epilogue`, `BatchMode`, `ReductionScheme`, `SearchMode`, heuristics cache, and logger.
- **Workspace & errors (`scratch`, `error`)**: `Scratch` caller-owned reusable device workspace with byte-budget checks, plus `Status` (`Copy` integer newtype) and `Error` (including `Error::Unavailable`, `Error::Cublas`, `Error::Budget`, and `Error::Invalid`).

---

## `cufft` (`cufft-sys`)

Safe cuFFT host API on top of `cufft-sys` and `cuda-core`.

- **Plans & transforms (`plan`)**: `Plan` (`plan_1d`, `plan_2d`, `plan_3d`, `plan_many`, `estimate_*`), `ManyDesc` (where `ManyDesc::min_io_elems` normalizes `None`-embed dense layouts), `FftType`, `Direction`, `Complex32`, `Complex64`, and typed execution (`exec_c2c`, `exec_r2c`, `exec_c2r`, `exec_z2z`, `exec_d2z`, `exec_z2d`; `exec_c2r` and `exec_z2d` take `&mut DeviceBuffer` for destructive cuFFT inputs).
- **Workspace management**: Every `Plan` disables cuFFT auto-allocation (`cufftSetAutoAllocation(0)`), sizes its `Scratch` workspace against a caller-supplied byte budget, and retains `Arc<ScratchInner>` with generation tracking and stream validation.
- **Diagnostics (`version`, `error`)**: `version()` queries `cufftGetVersion`; `FftResult` (`Copy` integer newtype) and `Error` report failures cleanly.

---

## `curand`

Safe cuRAND host and device RNG API on top of `cuda-bindings` and `cuda-core`. Unlike the other math crates, `curand` uses `cuda-bindings` directly and has no separate `curand-sys` crate.

- **Generators & distributions (`generator`)**: `Generator` (host and device), `RngType` (`Philox4_32_10`, `Mtgp32`, `MT19937`, `Xorwow`, `Mrg32k3a`, `Sobol32`, `ScrambledSobol32`, `Sobol64`, `ScrambledSobol64`), `Ordering`, `PoissonDistribution`, seeds, offsets, dimensions, stream binding, and uniform/normal/log-normal/Poisson/integer generation.
- **Diagnostics (`version`, `error`)**: `version()` queries `curandGetVersion`; `CurandStatus` (`Copy` integer newtype) and `Error` surface typed status and validation errors.

---

## `cusolver` (`cusolver-sys`, `cusolvermg-sys`)

Safe cuSOLVER host API on top of `cusolver-sys`, `cusolvermg-sys`, `cusparse-sys`, and `cuda-core`.

- **Dense solvers (`dn`)**: `DnHandle`, `DnFloat`, and `SvdJob` covering `GETRF`/`GETRS`, `POTRF`/`POTRS`, `GEQRF`/`ORGQR`, `GESVD`, and `SYEVD` for `f32` and `f64` with two-phase `bufferSize` -> byte budget -> `Scratch`.
- **Sparse solvers (`sp`)**: `SpHandle` and `MatDescr` for sparse QR linear solving (`csrlsvqr`) and shift-invert eigensolving (`csreigvsi`), plus `SpFloat`.
- **Multi-GPU solvers (`mg`)**: `MgHandle`, `MgGrid`, `MgGridMapping`, `MgDataType`, `MgMatrixDesc`, `EigMode`, and `validate_mg_syevd_w_type` for multi-GPU dense `POTRF`/`POTRS`/`POTRI`, `GETRF`/`GETRS`, and `SYEVD`.

---

## `cusparse` (`cusparse-sys`)

Safe cuSPARSE host API on top of `cusparse-sys` and `cuda-core`.

- **Handle (`handle`)**: `Handle` bound to a `cuda_core::Context` with stream binding and pointer-mode control.
- **Descriptors (`descr`)**: `SpMatDescr` (supporting CSR, CSC, COO, and Blocked-ELL via `SparseFormat`), `DnVecDescr`, and `DnMatDescr` with dimension/index validation and context-tracked teardown.
- **Operations (`ops`)**: `spmv` (`spmv_buffer_size`, `SpMvAlg`), `spmm` (`spmm_buffer_size`, `SpMmAlg`), `SpGemm` (`SpGemmAlg`), `SpSv` (`SpSvAlg`), and `SpSm` (`SpSmAlg`) with two-phase workspace sizing, caller byte-budget enforcement, and reusable `Scratch` buffers.
