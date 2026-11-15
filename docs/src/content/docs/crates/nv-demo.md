---
title: nv-demo (Interactive RTX Showcase)
description: Full-stack interactive OptiX 9 path tracer, TemporalAov denoiser, cuFFT bloom, NVRTC hot-reload, NVENC/nvJPEG export, and NVML/CUPTI/NGX telemetry.
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

The scene builds separate Geometry Acceleration Structures (GAS) combined into a 4-instance Top-Level Instance Acceleration Structure (`Accel::instances`):

- **Checkerboard Floor** (`TriangleMesh`): analytical PBR floor with configurable roughness, micro-scratch, scale, and metallic response.
- **Fractal Hero** (`CustomPrimitives` AABB): ray-marched signed distance field supporting **Mandelbulb**, **Mandelbox**, and **KIFS Octahedron**.
- **Dielectric Glass Sphere** (`TriangleMesh`): orbiting refractive satellite with configurable radius, orbit radius, height, IOR, and absorption.
- **Emissive Neon Accent** (`TriangleMesh`): orbiting emissive ring illuminating the fractal and floor.

### Fractal Transform & Coefficients

- **Presets**: `Golden Bulb`, `Chrome Box`, `Obsidian KIFS`, and `Julia Coral`.
- **Formulas**:
  - **Mandelbulb**: `Power`, `Bailout`, `Iterations`, `Theta Mult`, `Phi Mult`, `Theta Ph`, `Phi Ph`, and `Julia Mode` (`Julia C`).
  - **Mandelbox**: `Scale`, `Fold`, `Fold Mult`, `Min R`, `Fixed R`, `Iter Rot`, `Iterations`, and `Bailout`.
  - **KIFS Octahedron**: `Scale`, `Rot Y`, `Rot X`, `Offset`, `Thickness`, `Z Bias`, `Iterations`, and `Bailout`.
- **Transform & Detail**: `Obj Scale`, `Rot X/Y/Z`, `Detail Eps`, `Normal Eps`, `Fudge`, and `March Steps`.

---

## Denoising, AOVs & A/B Split

- **`TemporalAov` Denoiser**: runs `optix::Denoiser` with `DenoiserModel::TemporalAov`, feeding color, albedo, normal, and motion vectors (`Guides`) with temporal history accumulation.
- **5 AOV Modes**: switchable live in the Render panel and reflected in the viewport HUD badge: `Beauty`, `Albedo`, `Normal`, `Depth`, and `Motion`.
- **Interactive A/B Split**: toggle `Split View` to drag a vertical divider handle directly in the viewport (or scrub `Split X`) comparing the active output on the left against `Raw`, `Albedo`, or `Normal` on the right.

---

## Post-Processing, Hot-Reload & Export

- **`nvrtc` Live Shader Hot-Reload**: clicking **Hot-Reload Shader (NVRTC)** reads `crates/nv-demo/shaders/raytracer.cu` from disk, compiles it to PTX via `nvrtc`, rebuilds the OptiX `Pipeline` and `ShaderBindingTable`, and resets accumulation without restarting the app.
- **`cufft` FFT Optical Bloom**: `FftBloomPass` extracts bright pixels above a soft-knee luminance threshold into a power-of-two padded 3-channel planar buffer, runs a batched 2D `R2C` plan (`cufft`), multiplies by a cached procedural N-blade (`3..=12`) aperture diffraction PSF spectrum, runs `C2R`, and additively composites halation and starbursts back onto the `float4` color buffer.
- **`nvjpeg` & `nvenc` Zero-Copy Export**:
  - **JPEG Snapshot**: converts the device `RGBA8` frame to `RGB8` via an NVRTC kernel and encodes on the GPU with `nvjpeg`.
  - **H.264 Recording**: converts each `RGBA8` frame to `NV12` (BT.709 limited range) via an NVRTC kernel and encodes zero-copy with `nvenc` (`RegisteredResource`) into an Annex-B `.h264` stream.

---

## Telemetry & Profiler

`Profiler` tracks per-stage timings (`rt_ms`, `denoise_ms`, `bloom_ms`, `tonemap_ms`, `fps`) in stack-allocated `[f32; 120]` `RingBuffer`s with zero per-frame heap allocations, alongside:

- **`nvml`**: GPU thermals, utilization, VRAM usage, power draw, and graphics/memory clocks.
- **`cupti`**: graceful `ActivityProfiler` and `Subscriber` probing for kernel execution time without panicking on non-admin Windows hosts.
- **`ngx`**: graceful `NgxVulkan::required_extensions` availability probing.

---

## Camera Controls

- **`RMB` (Hold + Drag / WASD)**: free-fly `SpaceSim` navigation while holding the right mouse button.
- **`F`**: aim camera at the scene origin while preserving current eye position and updating orbit distance.
- **`A`**: fit all (`FrameBounds`) around the scaled fractal bounds.
- **Orbit / Pan / Zoom**: standard viewport drag and scroll controls via `camera_core`.
