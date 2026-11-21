# Changelog

All notable changes to `nv-rs` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this workspace adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **`nv-demo` RTX Path Tracing, Fractal Detail & Profiling** (`crates/nv-demo/`):
  - **OptiX SER (Shader Execution Reordering)**: Added `optixTraverse` → 3-bit hit/instance coherence hint `optixReorder(hint, 3)` → `optixInvoke` in `trace_primary`, runtime `DeviceContext::shader_reordering()` probing, and an `OptiX SER` toggle + hardware status in UI/Telemetry.
  - **OptiX Opacity Micro-Maps (OMM)**: Enabled `PipelineOptions::opacity_micromaps`, added SBT hitgroup 3 (`__closesthit__hit` + `__anyhit__omm`), and built a procedural perforated filigree halo ring (`48` triangles, 2 shared level-5 `FourState` micromaps via `OpacityMicromapArray`) in dual GASes (`omm_gas` / `omm_anyhit_gas`) with a 3-way `OMM Halo` (`Off | OMM | Any-Hit`) benchmark toggle.
  - **DLSS Ray Reconstruction (DLSS-RR / DLSSD)**: Added `pack_dlss_rr_aovs_f16` (`tonemap.cu`), spec-compliant G-buffer AOV exports (linear HDR beauty, diffuse/specular albedo, normal + packed roughness, view-space linear depth, motion vectors, 64-phase Halton jitter), lazy `DlssRrSharedBuffers`, `DlssRrPass` + WGSL `HdrTonemapPipeline` (ACES + live exposure), 4-way `Upscale / AI` selector (`Off | OptiX AI | DLSS SR | DLSS RR`), and automatic `nvngx_dlssd.dll` / `libnvidia-ngx-dlssd.so*` staging in `build.rs`.
  - **Native HDR (`Rgba16Float` scRGB) Viewport & Match-View Video** (`crates/nv-demo/src/{main,renderer,ui}.rs`, `crates/nv-demo/shaders/tonemap.cu`): Added live `View: SDR | HDR` swapchain switching (`Rgba16Float` scRGB with parallel `egui_wgpu::Renderer`, 203-nit paper white and 750→1000-nit soft rolloff matching `PQ10` video for both OptiX and DLSS-RR) and `Video: Match | SDR | PQ10` auto-matching.
  - **iPhone-compatible MP4 Export (SDR H.264 & HDR10 PQ10 HEVC) + Timing Compensation** (`crates/nv-demo/src/{mp4_muxer,gpu_export,renderer,main,ui}.rs`, `crates/nvenc/src/nvenc.rs`): Replaced raw Annex-B `.h264` output with a zero-dependency ISO BMFF `.mp4` muxer supporting both **SDR** (`avc1` H.264 High 8-bit BT.709 `colr`) and **PQ10** (`hvc1` HEVC Main10 P010 BT.2020 SMPTE ST 2084 PQ via `scrgb_f16_to_p010_pq` with `colr`/`mdcv`/`clli` for iPhone Pro Max HDR10 playback), capturing the final post-DLSS-RR / post-DLSS-SR / post-OptiX viewport output at full resolution, plus **Live RT** (per-frame 90 kHz `stts` timestamps eliminating speedup on FPS dips) and **Smooth 60** (fixed `1/60s` per rendered frame) recording modes.
  - **Nsight Systems / NVTX3 markers**: Added optional header-only NVTX3 C shim (`nvtx_shim.c`, `src/nvtx.rs`, `cfg(has_nvtx3)`) naming `nv-demo-main`/`optix-render` threads and bracketing `OptiX RT`, `OptiX Denoise`, `cuFFT Bloom`, `Tonemap`, and `DLSS` / `DLSS-RR` stages with zero-overhead no-ops when CUDA headers are absent.
- **`ngx` DLSS Ray Reconstruction (DLSSD) API** (`crates/ngx/src/{types,params,vulkan,ngx}.rs`):
  - Added `DlssDenoiseMode`, `DlssRoughnessMode`, `DlssDepthType`, `DlssRrPreset`, DLSSD parameter keys/helpers, `NgxVulkan::is_dlss_rr_available`, `query_dlss_rr_optimal_settings`, `create_dlss_rr`, and `FeatureHandle::evaluate_dlss_rr_with_params` (with cross-platform stubs).
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
- **`nv-demo` default 3-column dock layout & panel ergonomics** (`crates/nv-demo/src/ui.rs`): Set 3-column default dock layout (`Camera & Stats` left, `Viewport` center, `Fractal` top-right, `Render & Floor` bottom-right) with curated initial open sections, made each section's leading title-bar icon collapse/expand all sections in its dock panel, and unified Telemetry/Performance typography to `Proportional 11.5` with truncation tooltips.
- **`nv-demo` camera & bloom pipeline** (`crates/nv-demo/src/{main.rs,renderer.rs,ui.rs}`): Switched default RMB fly navigation from `SpaceSim` deflection to standard `FpsFly` (`NavFeel::Sharp`), wired `spin_speed` to fractal Y-rotation and sphere orbital velocity, defaulted `sat_orbit` to `false` for out-of-the-box accumulation, and isolated cuFFT bloom from `color_f32` accumulation and A/B Split Raw.
- **Generated shim safety**: `nv-bindgen` emits explicit `// SAFETY:` comments on every `unsafe` block in generated `*_shims.rs` files.

### Fixed
- **`nv-demo` fractal micro-detail & OptiX Denoiser model** (`crates/nv-demo/shaders/raytracer.cu`, `crates/nv-demo/src/{renderer,ui}.rs`): Replaced constant additive hit epsilon with pixel-cone `fractal_hit_eps` (down to `2e-6`), switched to 4-tap tetrahedral normals + 3-tap micro-AO, added Lipschitz step scaling + 4-step interior bisection, eliminated the `0.005` secondary/shadow ray blind zone with adaptive `ray_bias`, corrected the Mandelbox DE formula, replaced KIFS sphere primitive with an invariant rounded-box SDF (default Menger `scale = 3.0`), expanded fractal AABB to `[-1.85, 1.85]^3` and step/iter caps (`512`/`32`), and upgraded non-temporal denoiser from deprecated `DenoiserModel::Hdr` to `DenoiserModel::Aov` matching 3D world-space normals.
- **`ngx` DLSS-RR availability key** (`crates/ngx/src/params.rs`): Corrected `keys::RAY_RECONSTRUCTION_AVAILABLE` to `"SuperSamplingDenoising.Available"` matching `nvsdk_ngx_defs_dlssd.h`.
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
