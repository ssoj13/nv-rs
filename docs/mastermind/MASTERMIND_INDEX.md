# Mastermind Index — nv-rs (M2 Audit + M6 + M7 + M8 + M9 Wave 1)

Updated: 2026-11-13.
`vcv-rs` is published on `main` at `31c54282807fe2b0705ffde543fc8dfef4c2b543`; `Cargo.lock` and `device/Cargo.lock` lock `31c54282`.

## Wave 1 Completion Status

All 8 domains passed independent **Inquisitor** static audits (7 Mandatory Questions), **`cargo +1.98.0 clippy --all-targets -j 2 -- -D warnings`** (0 warnings, 0 errors), and **T0 `--lib` unit tests** (0 failures; GPU T2/T3 tests are `#[ignore]` pending serialized GPU run):

| Domain | Milestone | Crates | Inquisitor | Clippy `-D warnings` | T0 `--lib` Tests |
|---|---|---|---|---|---|
| `D01_D02` | M2 | `nvcomp-sys`, `nccl-sys`, `cufile-sys`, `nv-bindgen`, `nv-loader`, `nv-bindgen-fixture` | PASS | PASS | `nv-bindgen` 22 pass (1 ign), `nv-loader` 11 pass, `nv-bindgen-fixture` 8 pass |
| `D03` | M6 | `nv-wgpu` | PASS | PASS | 10 pass |
| `D04` | M7a | `cublas`, `cufft`, `curand` | PASS | PASS | `cublas` 5 pass (2 ign), `cufft` 3 pass (1 ign), `curand` 3 pass (1 ign) |
| `D05` | M7b | `cusparse`, `cusolver` | PASS | PASS | `cusparse` 6 pass (1 ign), `cusolver` 6 pass |
| `D06` | M7c | `npp`, `nvjpeg`, `nvrtc`, `cupti` | PASS | PASS | `npp` 4 pass, `nvjpeg` 2 pass, `nvrtc` 3 pass, `cupti` 6 pass |
| `D07` | M7d | `nvml`, `nvapi` | PASS | PASS | `nvml` 5 pass (1 ign), `nvapi` 7 pass (1 ign) |
| `D08` | M8 | `nvenc`, `nvdec`, `nvofa` | PASS | PASS | `nvenc` 4 pass, `nvdec` 4 pass, `nvofa` 4 pass |
| `D09` | M9 | `cudnn`, `nvcomp`, `nccl`, `cufile` (`ngx` held per D13) | PASS | PASS | `cudnn` 5 pass (2 ign), `nvcomp` 3 pass (2 ign), `nccl` 9 pass, `cufile` 3 pass (2 ign) |

See [SEAM_MATRIX.md](SEAM_MATRIX.md) for cross-crate contract verification.
