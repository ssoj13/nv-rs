# Architecture and runtime flows

Source map verified on 2026-11-23 (`5a58bab`, `csg-rs` `539f244`). See [repository guidance](AGENTS.md), [crate codepaths](crates/AGENTS.md), and [bughunt & acceleration plan](plan1.md).

## Build architecture

```mermaid
flowchart TD
  Spec["*-sys/build.rs: LibSpec"] --> Generate["nv-bindgen::generate"]
  SDK["vcv-rs SDK discovery"] --> Generate
  Headers["Installed SDK headers"] --> Bindgen["bindings / shims / inventory"]
  Generate --> Bindgen
  Prebuilt["Target-specific prebuilt artifacts"] --> Generate
  Bindgen --> Out["OUT_DIR generated Rust + inventory JSON"]
  Generate --> Out
  Out --> Sys["*-sys crate"]
  Sys --> Wrapper["Rust wrapper"]
  Cuda["cuda-core / cuda-bindings"] --> Wrapper
  Wrapper --> App["nv-demo / applications"]
  Wrapper --> Py["Separate python workspace"]
  Device["Separate device workspace"] --> PTX["PTX / device code"]
  PTX --> App
```

Prebuilt fallback depends on matching artifacts. Static-link shapes still require native link inputs; normal export shapes load libraries at runtime.

## First call and cached failures

```mermaid
sequenceDiagram
  participant Caller
  participant Shim as Generated shim
  participant Cache as Sys OnceLock
  participant Loader as nv-loader
  participant Native as Native library
  Caller->>Shim: Call entry point
  Shim->>Cache: Get API result
  alt First call
    Cache->>Loader: open_first(DllSpec)
    Loader->>Loader: Enumerate candidates and attempt load
    Loader-->>Cache: Loaded API or LoadError
  end
  Cache-->>Shim: Cached result
  alt API and symbol available
    Shim->>Native: Native call
    Native-->>Caller: Native result
  else Library or symbol unavailable
    Shim-->>Caller: Configured return policy / checked failure
  end
```

The shared macros retain a failed initialization for the process lifetime. Configure search paths before the first call.

## Graphics and CUDA interop

```mermaid
flowchart LR
  Adapter["wgpu adapter/device"] --> Verify["Backend support + UUID/LUID match"]
  CUDA["CUDA Context"] --> Verify
  Verify --> Context["InteropContext"]
  Context --> Shared["SharedBuffer / SharedTexture"]
  Shared --> Export["Export Vulkan or DX12 handle"]
  Export --> Import["cuda-core-ext import"]
  Import --> Work["CUDA device access"]
  Context --> Semaphore["InteropSemaphore"]
  Semaphore --> Work
  Semaphore --> Graphics["Graphics queue access"]
  Shared --> Graphics
```

External memory shares storage. Timeline synchronization controls when either API may access it; sharing alone does not order work.

## `nv-demo` decoupled 60 Hz UI, RTX burst & SDG pipeline

```mermaid
flowchart TD
  UI["60 Hz winit + egui UI (PresentMode::Fifo + WaitUntil 16.667ms)"] --> Bus["appbus::AppBus (StateSlot + UiCommand / RenderControl / RenderEvent)"]
  Bus --> Worker["Async OptiX Worker (renderer.rs, live optix::LogFn)"]
  Worker --> Step["CsgGpuHost::step_dynamic (TrafficStateSoA + SceneBuilt::crowd, Freeze Pose zero-motion snap)"]
  Step --> Sync["CsgGpuHost::sync (GrowDeviceBuffer + dual 256 KiB pinned DMA for instances & prev_xforms)"]
  Sync --> IAS["Persistent Uncompacted IAS (Accel::instances_into, compact=false, allow_update=true)"]
  IAS --> RT["OptiX 9 RT Cores (640 B Params, SBT 0..5, 11-GAS Crowd SBT 5, SER, OMM, Motion AOV 3x3 inv, 4x RGSS AA AOVs)"]
  RT --> Tensor["Tensor Cores (OptiX Aov/TemporalAov Denoiser or NGX DLSS SR/RR)"]
  Tensor --> Post["CUDA Compute (cuFFT Bloom -> Tonemap / HDR scRGB / DLSS-RR Pack)"]
  Post --> Fence["Single End-of-Frame stream.synchronize() + 3-slot cuEventQuery GpuPassProfiler"]
  Fence --> Present["Zero-Copy nv-wgpu SharedBuffer -> wgpu Display / MP4 NVENC / nvJPEG"]
  Worker --> SDG["ExportSdgJson (csg_actors::extract_sdg_frame + CsgRayOccluder -> atomic JSON)"]
```

The render worker submits `[IAS/Traffic/Crowd] -> [RT] -> [Denoiser] -> [cuFFT] -> [Tonemap]` in one uninterrupted stream burst with a single end-of-frame fence while `Runner::about_to_wait` paces UI redraws to 60 Hz.

## `nv-demo` audited hot-path bottlenecks vs target zero-stall architecture (`plan1.md`)

```mermaid
flowchart TD
  subgraph Current["Current Hot-Path Bottlenecks (Audited 2026-11-23)"]
    C1["TrafficOnly CPU Repack (10k-100k static + dynamic)"] --> C2["cuda_crowd Mid-Frame D2H + stream.synchronize()"]
    C2 --> C3["Accidental 50k IAS Rebuild (sat_angle when !show_classic)"]
    C3 --> C4["1 Giant AABB per PURE_UNION GAS -> SM Linear Scan + 4.2 KiB .local Spill"]
    C4 --> C5["Full-Res PoT cuFFT Bloom (2048^2 / 4096^2, ~302 MiB @ 4K)"]
    C5 --> C6["UI Thread Sync I/O (PNG/JPEG/MP4/persist) + FullWindow Poll Stall"]
  end

  subgraph Target["Target Zero-Stall Accelerated Pipeline (Waves 1-3)"]
    T1["Static-Tail Split (Repack Only [0..D] Dynamic Prefix)"] --> T2["100% GPU Crowd Steering + 1-Word Async Atomic Flag"]
    T2 --> T3["Guarded IAS State + Uncompacted Custom GAS (0 Compaction Stalls)"]
    T3 --> T4["Multi-AABB PURE_UNION GAS (RT Core BVH) + 0-Spill Registers + Shadow SBT + SER mat_id"]
    T4 --> T5["Half-Res 7-Smooth cuFFT Bloom + Pre-Normalized PSF + AOV Early-Out"]
    T5 --> T6["Async Export Worker + Disk PTX / Nvjpeg / SceneBuilt LRU Caches"]
  end
```
