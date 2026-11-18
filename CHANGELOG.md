# Changelog

All notable changes to `nv-rs` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this workspace adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **`nv-demo` interactive showcase & persistence** (`crates/nv-demo/`):
  - OptiX 9 IAS ray tracer with Mandelbulb / Mandelbox / KIFS Octahedron fractals, procedural scratched metal floor, emissive neon accent ring, `TemporalAov` AI denoising, A/B split view, live NVRTC `.cu` hot-reload, cuFFT diffraction bloom, zero-copy `nv-wgpu` presentation, `nvjpeg`/`nvenc` export, and `nvml`/`cupti`/`ngx` telemetry.
  - Analytic dielectric glass sphere (`__intersection__sphere`) with optional UV-mesh faceted mode (`U Segs` / `V Segs`), plus seeded multi-sphere planetary orbits (`0..=24` spheres, retrograde speed range, sin/cos vertical bobbing, and per-instance Beer–Lambert hue spread).
  - Top camera & animation toolbar (`Orbit`, `Fly (RMB)`, `Scheme`, `Feel`, `Home`, `Frame`, `Spin`, `Orbit`, `Speed`, `Reset`), compact collapsible property sections, and atomic persistence of window geometry, render settings, and camera pose in `~/.nv-rs/nv-demo.json` (`crates/nv-demo/src/persist.rs`).
- **`nv-wgpu` dedicated Vulkan barrier queue** (`crates/nv-wgpu/`): Added support for a dedicated Vulkan queue for CUDA↔wgpu interop barriers.
- **`nv-loader` DLL discovery & `open_library`** (`crates/nv-loader/src/nv_loader.rs`): Added `nv_loader::add_search_dir`, `NV_DLL_DIRS` support, expanded `Group::Toolkit` / `Group::Sdk` / `Group::DriverStore` search order, `nv_loader::open_library`, and actionable `LoadError::NotFound` hints.
- **`ngx` / `ngx-sys` NGX DLL path discovery** (`crates/ngx-sys/src/ngx_sys.rs`, `crates/ngx/src/vulkan.rs`): Exposed `ngx_sys::SDK_BIN` (`option_env!("NV_RS_SDK_BIN")`) and populated `NVSDK_NGX_PathListInfo` in `NgxVulkan::with_feature_common_info` (`app_data_path`, executable directory, `NGX_SDK_ROOT` / `DLSS_ROOT` / `NVSDK_NGX_ROOT`, and `ngx_sys::SDK_BIN`) so `nvngx_dlss*.dll` is found reliably without `0xBAD0000B`.
- **`nv_rs.add_dll_directory` & Python stub parity** (`python/nv-py/`, `xtask/src/py_stubs.rs`): Added runtime DLL search directory registration, `cargo xtask py-stubs <generate|check>`, and stub parity tests.
- **`cargo xtask stage-dlls` & `coverage`** (`xtask/src/`): Added redistributable NVIDIA DLL staging (`stage-dlls`) and wrapper coverage mapping (`cublaslt -> cublas`, `cusolvermg -> cusolver`, `nvcuvid -> nvdec`).
- **`cublaslt` & `cusolvermg` safe wrappers** (`crates/cublas/src/lt.rs`, `crates/cusolver/src/mg.rs`): Completed safe coverage of `cublasLt*` (48/48) and `cusolverMg*` (19/19) entry points.
- **Starlight documentation & GitHub Actions CI** (`docs/`, `.github/workflows/`): Published documentation at <https://ssoj13.github.io/nv-rs/> and automated formatting, clippy, core tests, and stub sync checks.

### Changed
- **`nv-demo` camera & bloom pipeline** (`crates/nv-demo/src/{main.rs,renderer.rs,ui.rs}`): Switched default RMB fly navigation from `SpaceSim` deflection to standard `FpsFly` (`NavFeel::Sharp`), wired `spin_speed` to fractal Y-rotation and sphere orbital velocity, defaulted `sat_orbit` to `false` for out-of-the-box accumulation, and isolated cuFFT bloom from `color_f32` accumulation and A/B Split Raw.
- **Generated shim safety**: `nv-bindgen` emits explicit `// SAFETY:` comments on every `unsafe` block in generated `*_shims.rs` files.

### Fixed
- **`EncodeSession` `Sync` soundness** (`crates/nvenc/src/nvenc.rs`): Serialized raw session handle access behind an internal `Mutex<()>` (`lock_session()`).
- **`cufft` workspace ownership & `ManyDesc`** (`crates/cufft/src/{plan.rs,scratch.rs}`): Retained `Arc<ScratchInner>` in `Plan` with generation tracking and stream validation, fixed `ManyDesc::min_io_elems`, and required `&mut DeviceBuffer` for destructive `exec_c2r` / `exec_z2d`.
- **Sealed `DeviceSpan` / `DeviceSpanMut` traits** (`crates/cudnn/`, `crates/nccl/`, `crates/cufile/`): Sealed unsafe device-span traits via private `mod sealed`.
- **`nv-py` DLPack & `__cuda_array_interface__` soundness** (`python/nv-py/src/`): Eliminated post-deleter UAF with `DlManagedGuard`, centralized checked `validate_shape_and_bytes`, exported byte `strides` and `"stream"`, and honored consumer `stream` in `__dlpack__`.
- **`nv-loader` numeric DLL version ordering** (`crates/nv-loader/src/nv_loader.rs`): Ordered wildcard DLL matches by numeric version components (`12 > 11 > 9`).
- **Prebuilt target/host isolation** (`xtask/src/prebuilt.rs`, `bootstrap.py`): Isolated host/target staging and added safe two-pass link extraction.

### Removed
- Obsolete `spikes/` prototype directory (`s1a-optix-ptx`, `s1b-optix-trace`, `s2-wgpu-cuda-share`, `s3-nv-bindgen`), superseded by the workspace crates.

## [0.1.0] - Initial Workspace

### Added
- Core infrastructure crates: `nv-loader`, `nv-bindgen`, `nv-bindgen-fixture`, and `xtask` (`coverage`, `prebuilt`, `gpu-test`).
- Interop and device crates: `cuda-core-ext`, `nv-wgpu`, `device/optix-device`, `device/cuda-tex-device`.
- Safe wrappers and `*-sys` bindings across the NVIDIA stack: `cublas`, `cublaslt`, `cudnn`, `cufft`, `cufile`, `cupti`, `curand`, `cusolver`, `cusparse`, `nccl`, `ngx`, `npp`, `nvapi`, `nvcomp`, `nvdec`, `nvenc`, `nvjpeg`, `nvml`, `nvofa`, `nvrtc`, `optix`.
- Python bindings (`python/nv-py` / `nv_rs`) with DLPack, `__cuda_array_interface__`, Buffer Protocol, and `nv_rs.diag`.
