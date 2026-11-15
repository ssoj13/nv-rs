---
title: AI, Storage, Comms & Management
description: Safe Rust wrappers and raw sys bindings for cuDNN, nvCOMP, cuFile (GDS), NCCL, NVRTC, CUPTI, NVML, and NVAPI.
sidebar:
  order: 3
---

This layer covers deep learning primitives, GPU compression, GPUDirect Storage, multi-GPU collectives, runtime kernel compilation, profiling, and system/driver telemetry.

## Crate Overview

| Safe Crate | Raw Bindings | Platform Notes | Scope |
|---|---|---|---|
| `cudnn` | `cudnn-sys` | Windows & Linux | Deep neural network forward primitives |
| `nvcomp` | `nvcomp-sys` | Windows & Linux | Batched GPU compression and native Bitcomp |
| `cufile` | `cufile-sys` | Linux (`cufile-sys`); `Error::Unsupported` on non-Linux | GPUDirect Storage (`cuFile`) |
| `nccl` | `nccl-sys` | Linux (`nccl-sys`); `Error::Unsupported` on non-Linux | Collective and point-to-point GPU comms |
| `nvrtc` | `nvrtc-sys` | Windows & Linux | Runtime CUDA C++ compilation |
| `cupti` | `cupti-sys` | Windows & Linux | CUDA Profiling Tools Interface |
| `nvml` | `nvml-sys` | Windows & Linux | NVIDIA Management Library telemetry |
| `nvapi` | `nvapi-sys` | Windows (`#[cfg(windows)]`); `Error::UnsupportedPlatform` elsewhere | Windows driver, display, HDR/VRR, and DRS |

---

## `cudnn` (`cudnn-sys`)

Safe wrappers for cuDNN handles, tensor/layer descriptors, workspace budgeting, and forward neural-network primitives.

- **Handle & descriptors (`handle`, `descriptor`)**: `Handle`, `TensorDescriptor`, `FilterDescriptor`, `ConvolutionDescriptor`, `ActivationDescriptor`, `PoolingDescriptor`, `ConvFwdAlgo`, `ConvMode`, `ActivationMode`, `PoolingMode`, and `MathType`.
- **Forward operations (`ops`, `scratch`)**: `conv_forward` (`conv_forward_workspace_size`), `activation_forward`, `pooling_forward`, `softmax_forward` (`SoftmaxAlgo`, `SoftmaxMode`), and `batch_norm_forward_inference` (`BatchNormMode`, `derive_bn_tensor_desc`). Workspaces are sized up front against a caller-supplied byte budget before growing a reusable `Scratch` buffer.

---

## `nvcomp` (`nvcomp-sys`)

Safe stream-ordered wrappers for NVIDIA nvCOMP 6 batched compression/decompression and native Bitcomp plans.

- **Batched async API (`batched`, `format`, `scratch`)**: `compress_batched_async`, `query_decompress_sizes_async`, and `decompress_batched_async` across `LZ4`, `Snappy`, `Zstd`, `Cascaded`, `Gdeflate`, and `Bitcomp`, with two-phase `temp_bytes` sizing, byte budgets, `Scratch`, and event-fenced `BatchStaging` tables.
- **Native Bitcomp (`bitcomp`)**: `BitcompPlan` (`BitcompAlgo`, `BitcompDataType`) for single-buffer lossless/lossy compression.
- **Stateful manager (`manager`)**: `CompressionManager` and self-describing `CompressedBuffer` / `CompressedLayout` built on top of the C batched APIs (since the upstream `nvcompManager` API is C++-only).

---

## `cufile` (`cufile-sys`)

Safe Rust wrappers for NVIDIA GPUDirect Storage (`cuFile`).

- **Platform constraint**: `cufile-sys` exists only on Linux (`cfg(target_os = "linux")`). The `cufile` crate compiles on both Windows and Linux; GDS-only entry points return `Error::Unsupported` on non-Linux hosts while types, status decoding (`CufileStatus`), and explicit `IoMode::PosixFallback` remain usable everywhere.
- **I/O policy & RAII (`handle`)**: `DriverSession` (`cuFileDriverOpen` / `cuFileDriverClose_v2`), `FileHandle`, `RegisteredBuffer`, `RegisteredStream`, and `AsyncIoSlot` for zero-steady-state-alloc stream-ordered async transfers. `IoMode::GdsOnly` never silently downgrades to host-staged POSIX I/O; `IoMode::PosixFallback` must be selected explicitly.

---

## `nccl` (`nccl-sys`)

Safe Rust wrappers for NVIDIA NCCL collective and point-to-point GPU communication.

- **Platform constraint**: `nccl-sys` exists only on Linux (`cfg(target_os = "linux")`). The `nccl` crate compiles on both Windows and Linux; on non-Linux targets, runtime entry points return `Error::Unsupported` while types, serialization, and validation remain available.
- **Communicators & collectives (`comm`)**: `Comm` (`init_all`, `init_rank`), portable `UniqueId` tokens, `group` / `group_start` / `group_end` (`GroupGuard`) fusion, stream-ordered `all_reduce`, `broadcast`, `reduce`, `all_gather`, `reduce_scatter`, `send`, and `recv`, plus async-error-aware teardown (`async_error`, `finalize`, `abort`, and abort-on-fault in `Drop`).

---

## `nvrtc` (`nvrtc-sys`)

Safe wrapper over `nvrtc-sys` for host-side runtime compilation of CUDA C++ source without requiring a CUDA context.

- **Compiler options (`options`)**: `CompileOptions` builder for `--gpu-architecture`, `--std`, macros, include paths, fast math, debug/line-info, RDC, and LTO.
- **Programs & outputs (`program`)**: `Program` owns an `nvrtcProgram`, registers name expressions before compilation, compiles via `Program::compile` (capturing the build log on both success and failure), and retrieves PTX, CUBIN, LTO-IR, OptiX-IR, and lowered symbol names. Also exposes `version` and `supported_archs`.

---

## `cupti` (`cupti-sys`)

Safe wrapper over `cupti-sys` for the CUDA Profiling Tools Interface (CUPTI).

- **Activity API (`activity`)**: `ActivityProfiler`, `ActivityKind`, `ActivityRecord`, and `AlignedBufferPool` register `cuptiActivityRegisterCallbacks` with `catch_unwind` trampolines backed by a reusable 8-byte-aligned buffer pool.
- **Callback API (`subscriber`)**: `Subscriber`, `CallbackDomain`, and `CallbackFn` wrap `cuptiSubscribe` / `cuptiUnsubscribe` with `catch_unwind` trampolines and callback name lookup.
- **Header scope**: In CUDA 13.3, `cupti-sys` binds `cupti.h` (version, result strings, Activity API, Callback API); legacy `cupti_events.h` / `cupti_metrics.h` and `cupti_profiler_target.h` are not bound.

---

## `nvml` (`nvml-sys`)

Safe NVIDIA Management Library (NVML) wrapper over `nvml-sys` (used by `gpu-info-rs`).

- **Session (`session`)**: `Nvml` manages an `Arc`-shared NVML session (`nvmlInit_v2` / `nvmlShutdown` once when the last session and device drop), system version queries, and device enumeration by index, UUID, or `PciBusId`.
- **Telemetry (`device`, `types`)**: `Device` queries device identity, memory (`MemoryInfo`, `Bar1MemoryInfo`), utilization (`Utilization`, `CodecUtilization`), thermal/fan, power/energy, clocks (`ClockDomain`), ECC (`EccMode`, `EccCounterType`), running processes (`ProcessInfo`, `ProcessScratch`), PCIe link/throughput, and NVLink telemetry.

---

## `nvapi` (`nvapi-sys`)

Safe Windows NVAPI wrapper over `nvapi-sys` (`nvapi64.dll` via `nvapi_QueryInterface`).

- **Platform constraint**: Windows FFI calls live behind `#[cfg(windows)]`; on non-Windows targets, `NvApi::init()` and runtime queries return `Error::UnsupportedPlatform` while cross-platform types remain testable everywhere.
- **Session, GPUs & displays**: `NvApi` (`Arc`-shared `NvAPI_Initialize` / `NvAPI_Unload`), `PhysicalGpu` (WDDM and TCC GPUs, architecture, core count, VBIOS, memory, thermal sensors, tachometer, P-states, clocks, throttling reasons), `Display` (HDR capabilities and VRR / G-Sync Compatible status), Quadro Sync / G-Sync (`GSyncCapabilities`, `GSyncStatus`), and Driver Settings (`DrsSession`, `DrsProfileInfo`).
