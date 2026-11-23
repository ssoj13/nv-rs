# `nv-demo` Agent Guide

## What & Why
`crates/nv-demo/` is the flagship interactive RTX showcase binary (`nv-demo`) of `nv-rs`. It exercises OptiX 9 hardware ray tracing (RT Cores), OptiX AI Denoiser & NGX DLSS SR/RR (Tensor Cores), cuFFT 2D diffraction bloom + CUDA tonemapping + zero-churn IAS/traffic streaming (CUDA Compute SMs), nvJPEG/nvENC export, and NVML/CUPTI telemetry inside a decoupled 60+ FPS `egui` / `egui-dock` / `wgpu` studio UI.

## Callers & Dependencies
- **Callers**: standalone desktop binary (`cargo run -p nv-demo --release`).
- **Workspace crates**: `optix`, `nv-wgpu`, `nv-loader`, `nvrtc`, `nvml`, `cufft`, `nvjpeg`, `nvenc`, `cupti`, `ngx`, `cuda-core-ext`.
- **External CG crates**: `cuda-core`, `cuda-bindings`, `appbus`, `csg-core`, `csg-scene` (`wgpu`), `csg-device`, `csg-scenes`, `egui`/`egui-wgpu`/`egui-winit`/`egui_dock`/`egui-phosphor`, `wgpu`, `glam`, `bytemuck`, `serde`.

## How It Works
```text
winit + 60+ FPS egui UI (main.rs, ui.rs)
  |-- appbus::AppBus (StateSlot<RenderStateSnapshot>, UiCommand, RenderControl, RenderEvent)
  v
Async OptiX Render Worker (renderer.rs, rig.stream)
  1. GpuPassProfiler::begin_frame (3-slot CUevent ring, non-stalling cuEventQuery)
  2. CsgGpuHost::sync -> TrafficOnly (pinned 256 KiB DMA + Accel::instances_into UPDATE)
                       | FullRepack (persistent GrowDeviceBuffer + InstanceBuffer + in-place IAS BUILD)
  3. Uninterrupted GPU stream burst (ONE end-of-frame fence):
     [IAS/Traffic] -> [OptiX RT (SBT 0..5)] -> [OptiX Denoiser] -> [cuFFT Bloom] -> [Tonemap / HDR / DLSS-RR Pack]
  4. Single rig.stream.synchronize() -> GpuPassProfiler::poll_completed() -> FrameReady -> wgpu/DLSS/egui
```
- `src/csg_scene_host.rs`: 100% scene-agnostic `csg-scenes` host (`Gundo`, `GundoOSM`, `Marina del Rey`, `SF House of AI`, 91-asset `Inspector`), per-compound GAS cache, split SBT 4 (`hit_csg`) / SBT 5 (`hit_csg_prim`) routing, persistent `GrowDeviceBuffer<T>` pools for all static/dynamic buffers, and `256 KiB` pinned `cuMemAllocHost_v2` staging for `dynamic_instances[0..D]`.
- `src/profiler.rs`: lock-free fixed-capacity `[f32; 120]` `RingBuffer`s (`FrameTimings`), non-stalling 3-slot `GpuPassProfiler` (`cuEventQuery` only, never `cuEventSynchronize`) reporting **RT Cores (OptiX)**, **Tensor Cores (Denoiser / DLSS)**, and **CUDA Compute (IAS / cuFFT / Tonemap)** ms, plus NVML/CUPTI/NGX probing.
- `src/ui.rs`: narrow-dock-safe `dock_scroll` + adaptive `prop_row_widths`/`slider_control_widths` + truncated `equal_grid`/`ComboBox` layout, Apple Maps clickable WGS84 HUD pill, master `Fractal` enable toggle, Hero Fractal full TRS (`T * R_z * R_y * R_x * S`), and per-engine `Performance` sparklines.

## Optimizations & Invariants
- **Zero Per-Frame VRAM Alloc/Free Churn**: `optix::InstanceBuffer` (`d_optix_instances` + `256 KiB` pinned staging), `Accel::instances_into` (in-place `OPTIX_BUILD_OPERATION_UPDATE` on traffic-only frames and in-place buffer reuse on full repacks), and persistent `GrowDeviceBuffer<T>` pools across all CSG library & AABB buffers yield `0 cuMemAlloc`, `0 cuMemFree`, and `0 Vec` allocs in steady state.
- **Single End-of-Frame Fence**: zero mid-frame `stream.synchronize()` or `cuEventSynchronize()` stalls between RT, Denoiser, cuFFT Bloom, and Tonemap; double-buffered `free_bufs` overlaps Frame `N+1` CUDA/OptiX with Frame `N` Vulkan DLSS/UI presentation.
- **Strict 592-Byte C ABI**: `LaunchParams` (`renderer.rs`) and `Params` (`shaders/raytracer.cu`) are locked by compile-time `assert!` and `static_assert(sizeof(Params) == 592)`, with `0` `double` tokens in `raytracer.cu`.
- **Large-Map Flight & Responsive UI**: progressive Shift ramp (`1x` <= 2s, `2^((t-2)/5)` up to `64x`), altitude-aware speed (`1x..=25x`), `RMB + Wheel` speed multiplier (`0.1..=64x`), and interactive fast-preview (`1 spp`, `<= 2` bounces + `150 ms` settle).

## Traps & Future Improvements
- **Trap (`step_traffic` dirty flag)**: `CsgGpuHost::step_traffic` must set `traffic_dirty` (not `ias_dirty`), otherwise `sync()` forces a full 50k-instance repack instead of the `TrafficOnly` pinned-DMA + `OPTIX_BUILD_OPERATION_UPDATE` fast path.
- **Trap (egui narrow dock inflation)**: never place unbounded `add_sized([w, h], Button::new(long_text))` or untruncated `ComboBox`es inside a `ScrollArea` without locking `inner_w` via `dock_scroll` and `.truncate()`, or sibling sections will clip off the right edge.
- **Future Improvements**:
  - Wire zero-copy CUDA traffic kernel (`Pillar C` of `PLAN7_ROAD_FREP_TRAFFIC.md`) writing `xform[12]` directly into both `csg_instances[0..D]` and `d_optix_instances[0..D].transform` in VRAM (`0 B` PCIe H2D/frame).
  - Add a live `RoadMode` (`Watertight3D`, `TangentPlaneC1`, `HybridC1Sdf`, `FRepTile`) selector in `CSG World` once Wave 8 `csg-rs` lands.
  - Replace host `Instant` DLSS timing with Vulkan `VkQueryPool` timestamp queries for sub-microsecond DLSS GPU timing.
