# Changelog

All notable changes to `nv-rs` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this workspace adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **GitHub Actions CI** (`.github/workflows/ci.yml`): Automated `ubuntu-latest` and `windows-latest` checks running `cargo fmt --all -- --check`, `cargo clippy` (`-D warnings`), `cargo test` on SDK-free core crates (`nv-loader`, `nv-bindgen`, `nv-bindgen-fixture`), and Python stub synchronization tests.
- **Python stub parity tests** (`python/nv-py/tests/test_stub_sync.py`): Verifies that all 25 submodules and `__version__` exported by `python/nv-py/src/lib.rs` match `__init__.pyi`, `__init__.py`, and `pyproject.toml`.
- **Starlight documentation & GitHub Pages** (`docs/`, `.github/workflows/pages.yml`): Full documentation site covering architecture, `nv-loader`, `nv-bindgen`, `cuda-core-ext`, `nv-wgpu`, all wrapped NVIDIA crates, `nv-demo`, and the `nv_rs` Python API, published at <https://ssoj13.github.io/nv-rs/>.
- **`nv-demo` full-stack showcase** (`crates/nv-demo/`): Interactive OptiX 9 IAS ray tracer (fractal AABBs, dielectric glass, emissive neon, checkerboard floor) with `TemporalAov` AI denoising, A/B split view, live NVRTC `.cu` hot-reload, cuFFT diffraction bloom, zero-copy `nv-wgpu` presentation, `nvjpeg`/`nvenc` export, and `nvml`/`cupti`/`ngx` telemetry.
- **Streamline NGX SDK discovery**: Enabled `ngx` by default in `nv-demo` and integrated NVIDIA Streamline `ngx-sdk` detection via `vcv-rs` and `bootstrap.py`.

### Changed
- **Generated shim safety**: `nv-bindgen` now emits explicit `// SAFETY:` comments on every `unsafe` block in generated `*_shims.rs` files, and `nv-loader` macros (`exports_sys!`, `split_exports_sys!`, `query_interface_sys!`) no longer suppress `clippy::undocumented_unsafe_blocks`.
- **Workspace metadata**: Updated `workspace.package.authors` in `Cargo.toml` to `Alex Khalyavin <joss13@gmail.com>`.

### Fixed
- **`EncodeSession` `Sync` soundness** (`crates/nvenc/src/nvenc.rs`): Serialized all NVENC API calls and `Drop` paths touching the raw session handle behind an internal `Mutex<()>` (`lock_session()`), backing `unsafe impl Sync for EncodeSession`.
- **Documentation & links**: Fixed GitHub Pages `/nv-rs` base-path links, added the documentation badge/link to `README.md`, and corrected the `nv_rs` submodule count (`25`).

## [0.1.0] - Initial Workspace

### Added
- Core infrastructure crates: `nv-loader`, `nv-bindgen`, `nv-bindgen-fixture`, and `xtask` (`coverage`, `prebuilt`, `gpu-test`).
- Interop and device crates: `cuda-core-ext`, `nv-wgpu`, `device/optix-device`, `device/cuda-tex-device`.
- Safe wrappers and `*-sys` bindings across the NVIDIA stack: `cublas`, `cublaslt`, `cudnn`, `cufft`, `cufile`, `cupti`, `curand`, `cusolver`, `cusparse`, `nccl`, `ngx`, `npp`, `nvapi`, `nvcomp`, `nvdec`, `nvenc`, `nvjpeg`, `nvml`, `nvofa`, `nvrtc`, `optix`.
- Python bindings (`python/nv-py` / `nv_rs`) with DLPack, `__cuda_array_interface__`, Buffer Protocol, and `nv_rs.diag`.
