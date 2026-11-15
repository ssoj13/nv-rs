# Changelog

All notable changes to `nv-rs` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this workspace adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **`nv-loader` DLL discovery & `open_library`** (`crates/nv-loader/src/nv_loader.rs`): Added `nv_loader::add_search_dir`, `NV_DLL_DIRS` support, expanded `Group::Toolkit` / `Group::Sdk` / `Group::DriverStore` search order (`exe_dir`, `%ProgramFiles%\NVIDIA GPU Computing Toolkit\CUDA\v*`, `PATH`, and Linux runtime `root_var` / `CUDA_PATH` resolution in `linux_candidates_with`), `nv_loader::open_library` (`LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS` with `Library::new` fallback), and actionable `LoadError::NotFound` hints.
- **`nv_rs.add_dll_directory`** (`python/nv-py/`): Added `nv_rs.add_dll_directory(path)` and `nv_rs.diag.add_dll_directory(path)` to register extra DLL search directories at runtime (also calling `os.add_dll_directory` on Windows).
- **`cargo xtask stage-dlls`** (`xtask/src/stage_dlls.rs`): Added `cargo xtask stage-dlls --out <dir> [--libs <csv>] [--include-driver] [--dry-run]` to stage redistributable NVIDIA runtime DLLs into a target directory.
- **GitHub Actions CI** (`.github/workflows/ci.yml`): Automated `ubuntu-latest` and `windows-latest` checks running `cargo fmt --all -- --check`, `cargo clippy` (`-D warnings`), `cargo test` on SDK-free core crates (`nv-loader`, `nv-bindgen`, `nv-bindgen-fixture`), and Python stub synchronization tests.
- **Python stub parity tests** (`python/nv-py/tests/test_stub_sync.py`): Verifies that all 25 submodules and `__version__` exported by `python/nv-py/src/lib.rs` match `__init__.pyi`, `__init__.py`, and `pyproject.toml`.
- **Starlight documentation & GitHub Pages** (`docs/`, `.github/workflows/pages.yml`): Full documentation site covering architecture, `nv-loader`, `nv-bindgen`, `cuda-core-ext`, `nv-wgpu`, all wrapped NVIDIA crates, `nv-demo`, and the `nv_rs` Python API, published at <https://ssoj13.github.io/nv-rs/>.
- **`nv-demo` full-stack showcase** (`crates/nv-demo/`): Interactive OptiX 9 IAS ray tracer (fractal AABBs, dielectric glass, emissive neon, checkerboard floor) with `TemporalAov` AI denoising, A/B split view, live NVRTC `.cu` hot-reload, cuFFT diffraction bloom, zero-copy `nv-wgpu` presentation, `nvjpeg`/`nvenc` export, and `nvml`/`cupti`/`ngx` telemetry.
- **Streamline NGX SDK discovery**: Enabled `ngx` by default in `nv-demo` and integrated NVIDIA Streamline `ngx-sdk` detection via `vcv-rs` and `bootstrap.py`.
- **`cublaslt` safe wrapper coverage** (`crates/cublas/src/lt.rs`, `crates/cublas/src/error.rs`): Wrapped all 48 public `cublasLt*` entry points, adding `MatrixTransformDesc`, `EmulationDesc`, `MatmulAlgo` config/capabilities, `BatchMode`, `ReductionScheme`, `SearchMode`, heuristics cache, logger helpers, and `Status::lt_name` / `Status::lt_string`.
- **`cusolvermg` safe wrapper coverage** (`crates/cusolver/src/mg.rs`, `crates/cusolver/src/cusolver.rs`): Completed all 19/19 `cusolverMg*` entry points with `potri_buffer_size`, `potri`, `syevd_buffer_size`, `syevd`, `EigMode::to_mg_sys`, and `validate_mg_syevd_w_type`.
- **`xtask coverage` wrapper mapping & CI** (`xtask/src/xtask.rs`, `coverage/`, `.github/workflows/ci.yml`): Added `wrapper_src_dir` mapping (`cublaslt -> cublas`, `cusolvermg -> cusolver`, `nvcuvid -> nvdec`), populated wrapper coverage manifests, and included `-p xtask` in CI.

### Changed
- **Generated shim safety**: `nv-bindgen` now emits explicit `// SAFETY:` comments on every `unsafe` block in generated `*_shims.rs` files, and `nv-loader` macros (`exports_sys!`, `split_exports_sys!`, `query_interface_sys!`) no longer suppress `clippy::undocumented_unsafe_blocks`.
- **Workspace metadata**: Updated `workspace.package.authors` in `Cargo.toml` to `Alex Khalyavin <joss13@gmail.com>`.

### Fixed
- **`EncodeSession` `Sync` soundness** (`crates/nvenc/src/nvenc.rs`): Serialized all NVENC API calls and `Drop` paths touching the raw session handle behind an internal `Mutex<()>` (`lock_session()`), backing `unsafe impl Sync for EncodeSession`.
- **Documentation & links**: Fixed GitHub Pages `/nv-rs` base-path links, added the documentation badge/link to `README.md`, and corrected the `nv_rs` submodule count (`25`).
- **`nv-demo` runtime resize & PNG export error handling** (`crates/nv-demo/src/main.rs`): Replaced panic-on-OOM render-target reallocation and `save_front_png` CUDA context binding `unwrap()` calls with error propagation and status reporting.

## [0.1.0] - Initial Workspace

### Added
- Core infrastructure crates: `nv-loader`, `nv-bindgen`, `nv-bindgen-fixture`, and `xtask` (`coverage`, `prebuilt`, `gpu-test`).
- Interop and device crates: `cuda-core-ext`, `nv-wgpu`, `device/optix-device`, `device/cuda-tex-device`.
- Safe wrappers and `*-sys` bindings across the NVIDIA stack: `cublas`, `cublaslt`, `cudnn`, `cufft`, `cufile`, `cupti`, `curand`, `cusolver`, `cusparse`, `nccl`, `ngx`, `npp`, `nvapi`, `nvcomp`, `nvdec`, `nvenc`, `nvjpeg`, `nvml`, `nvofa`, `nvrtc`, `optix`.
- Python bindings (`python/nv-py` / `nv_rs`) with DLPack, `__cuda_array_interface__`, Buffer Protocol, and `nv_rs.diag`.
