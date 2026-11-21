---
title: nv-demo (Interactive RTX Showcase)
description: Full-stack interactive OptiX 9 path tracer with SER, Opacity Micro-Maps, DLSS SR/RR, TemporalAov/Aov denoising, cuFFT bloom, NVRTC hot-reload, NVTX3 markers, NVENC/nvJPEG export, and NVML/CUPTI/NGX telemetry.
sidebar:
  order: 4
---

`nv-demo` (`crates/nv-demo/`) is the full-stack interactive RTX showcase for `nv-rs`. It wires `optix`, `nvrtc`, `cufft`, `nvjpeg`, `nvenc`, `nvml`, `cupti`, `ngx`, `cuda-core-ext`, and `nv-wgpu` into a docked `egui-wgpu` desktop application with zero-copy CUDA ↔ `wgpu` presentation.

## Building & Running

```powershell
$env:NV_SDK_DIRS = "$PWD\.sdk"   # or set OPTIX_ROOT / CUDA_PATH / NGX_SDK_ROOT
python bootstrap.py b -p nv-demo
cargo run -p nv-demo --release
```

On Windows, `ngx` (`ngx-sys`) is enabled by default (`default = ["ngx"]`) and automatically probes `NV_SDK_DIRS`, `NGX_SDK_ROOT`, or `C:\Program Files\NVIDIA\Streamline\external\ngx-sdk`.

---

## Scene & OptiX 9 Pipeline

### Multi-Instance IAS

The scene builds separate Geometry Acceleration Structures (GAS) combined into a multi-instance Top-Level Instance Acceleration Structure (`Accel::instances`):

- **Checkerboard Floor** (`TriangleMesh`): analytical PBR floor with configurable roughness, micro-scratch, scale, and metallic response.
- **Fractal Hero** (`CustomPrimitives` AABB `[-1.85, 1.85]^3`): pixel-cone ray-marched signed distance field supporting **Mandelbulb**, **Mandelbox**, and **KIFS Octahedron** with 4-tap tetrahedral normals, 3-tap micro-AO, Lipschitz step safety, 4-step interior bisection, and adaptive secondary/shadow ray bias.
- **Dielectric Glass Spheres** (`CustomPrimitives` analytic or `TriangleMesh`): orbiting refractive satellites with configurable radius, orbit radius, height, IOR, and Beer–Lambert absorption.
- **Emissive Neon Accent** (`TriangleMesh`): orbiting emissive ring illuminating the fractal and floor.
- **Perforated Filigree Halo (`OMM Halo`)** (`TriangleMesh` + `OpacityMicromapArray`): optional 48-triangle halo band backed by 2 shared level-5 `FourState` opacity micromaps, built into dual GASes (`omm_gas` and `omm_anyhit_gas`) so you can benchmark **`Off | OMM | Any-Hit`** live in the Scene panel.

### Shader Execution Reordering (OptiX SER)

- `trace_primary` supports native OptiX 9 **SER** (`optixTraverse` → 3-bit hit/instance coherence hint `optixReorder(hint, 3)` → `optixInvoke`), reducing warp divergence across fractal raymarching, dielectric refraction, procedural metal floor, filigree OMM, and sky misses.
- Hardware SER capability is queried via `DeviceContext::shader_reordering()` and surfaced in the Path Tracer panel (`OptiX SER` toggle) and Telemetry (`HW Active` / `HW Off` / `Unsupported`).

### Fractal Transform & Coefficients

- **Presets**: `Golden Bulb`, `Chrome Box`, `Obsidian KIFS`, and `Julia Coral`.
- **Formulas**:
  - **Mandelbulb**: `Power`, `Bailout`, `Iterations` (`4..=32`), `Theta Mult`, `Phi Mult`, `Theta Ph`, `Phi Ph`, and `Julia Mode` (`Julia C`).
  - **Mandelbox**: `Scale`, `Fold`, `Fold Mult`, `Min R`, `Fixed R`, `Iter Rot`, `Iterations` (`4..=32`), and `Bailout` (`4..=128`).
  - **KIFS Octahedron**: `Scale` (`1.3..=3.5`, default Menger `3.0`), `Rot Y`, `Rot X`, `Offset`, `Thickness`, `Z Bias`, `Iterations` (`4..=32`), and `Bailout`.
- **Transform & Detail**: `Obj Scale`, `Rot X/Y/Z`, `Detail Eps` (`0.00002..=0.005`), `Normal Eps`, `Fudge`, and `March Steps` (`32..=512`).

---

## Denoising, DLSS SR / Ray Reconstruction, AOVs & A/B Split

- **4-Way `Upscale / AI` Selector**: switch live between **`Off`**, **`OptiX AI`** (`DenoiserModel::Aov` or `DenoiserModel::TemporalAov` with albedo, 3D world-space normal, and motion guides), **`DLSS SR`** (NGX Vulkan Super Resolution with 16-phase Halton jitter), and **`DLSS RR`** (NGX Vulkan Ray Reconstruction / DLSSD).
- **DLSS Ray Reconstruction (`DLSS RR`)**: replaces OptiX Denoiser + DLSS SR with a single neural pass fed by `pack_dlss_rr_aovs_f16` (linear HDR noisy beauty, diffuse/specular albedo, normal + packed roughness, view-space linear depth, motion vectors, and 64-phase Halton jitter), followed by a WGSL ACES `HdrTonemapPipeline` with live exposure control. `build.rs` automatically stages both `nvngx_dlss.dll` and `nvngx_dlssd.dll`.
- **5 AOV Modes**: switchable live in the Render panel (including while DLSS-RR is active) and reflected in the viewport HUD badge: `Beauty`, `Albedo`, `Normal`, `Depth`, and `Motion`.
- **Interactive A/B Split**: toggle `Split View` to drag a vertical divider handle directly in the viewport (or scrub `Split X`) comparing the active output on the left against `Raw`, `Albedo`, or `Normal` on the right.

---

## Post-Processing, Hot-Reload & Export

- **`nvrtc` Live Shader Hot-Reload**: clicking **Hot-Reload Shader (NVRTC)** reads `crates/nv-demo/shaders/raytracer.cu` from disk, compiles it to PTX via `nvrtc`, rebuilds the OptiX `Pipeline` and `ShaderBindingTable`, and resets accumulation without restarting the app.
- **`cufft` FFT Optical Bloom**: `FftBloomPass` binds its `Scratch` workspace to the active render stream, extracts bright pixels above a soft-knee luminance threshold into a power-of-two padded 3-channel planar buffer, runs a batched 2D `R2C` plan (`cufft`), multiplies by a cached procedural N-blade (`3..=12`) aperture diffraction PSF spectrum, runs `C2R` (`&mut` destructive input), and additively composites halation and starbursts back onto the `float4` color buffer.
- **Triangle GAS & Resize Resilience**: triangle GAS builders enable `random_vertex_access: true` for `optixGetTriangleVertexData`, and runtime viewport resize / PNG export paths propagate `nv_wgpu` and CUDA context errors cleanly without panicking on OOM.
- **`nvjpeg` & `nvenc` Zero-Copy Export**:
  - **JPEG Snapshot**: converts the device `RGBA8` frame to `RGB8` via an NVRTC kernel and encodes on the GPU with `nvjpeg`.
  - **H.264 Recording**: converts each `RGBA8` frame to `NV12` (BT.709 limited range) via an NVRTC kernel and encodes zero-copy with `nvenc` (`RegisteredResource`) into an Annex-B `.h264` stream.

---

## Telemetry & Profiler

`Profiler` tracks per-stage timings (`rt_ms`, `denoise_ms`, `bloom_ms`, `tonemap_ms`, `fps`) in stack-allocated `[f32; 120]` `RingBuffer`s with zero per-frame heap allocations, alongside:

- **`nvml`**: GPU thermals, utilization, VRAM usage, power draw, and graphics/memory clocks.
- **`cupti`**: graceful `ActivityProfiler` and `Subscriber` probing for kernel execution time without panicking on non-admin Windows hosts.
- **`ngx`**: graceful `NgxVulkan::required_extensions`, DLSS Super Resolution, and DLSS Ray Reconstruction (`is_dlss_rr_available`) probing.
- **Nsight Systems / NVTX3 (`nvtx_shim.c`, `src/nvtx.rs`)**: optional header-only NVTX3 ranges (`cfg(has_nvtx3)`) naming `nv-demo-main` and `optix-render` threads and annotating `OptiX RT`, `OptiX Denoise`, `cuFFT Bloom`, `Tonemap`, and `DLSS` / `DLSS-RR` stages (compiles to inline no-ops when CUDA NVTX3 headers are absent).

---

## Camera Controls

- **`RMB` (Hold + Drag / WASD)**: free-fly `SpaceSim` navigation while holding the right mouse button.
- **`F`**: aim camera at the scene origin while preserving current eye position and updating orbit distance.
- **`A`**: fit all (`FrameBounds`) around the scaled fractal bounds.
- **Orbit / Pan / Zoom**: standard viewport drag and scroll controls via `camera_core`.
