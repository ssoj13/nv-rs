---
title: Imaging, Video & Graphics
description: Safe Rust wrappers and raw sys bindings for OptiX, NGX (DLSS), NVENC, NVDEC, NVOFA, NPP, and nvJPEG.
sidebar:
  order: 2
---

The imaging, video, and graphics layer provides safe RAII wrappers over NVIDIA's ray tracing, AI upscaling, hardware video encode/decode, optical flow, image/signal primitives, and JPEG codec SDKs.

## Crate Overview

| Safe Crate | Raw Bindings | Platform Notes | Scope |
|---|---|---|---|
| `optix` | `optix-sys` | Windows & Linux | OptiX ray tracing, acceleration structures, and AI denoiser |
| `ngx` | `ngx-sys` | Windows (`ngx-sys` statically linked); clean stubs on non-Windows | NVIDIA NGX / DLSS Vulkan session and feature wrappers |
| `nvenc` | `nvenc-sys` | Windows & Linux | Hardware video encoder (H.264, HEVC, AV1) |
| `nvdec` | `nvcuvid-sys` | Windows & Linux | Hardware video parser and decoder |
| `nvofa` | `nvofa-sys` | Windows & Linux | Hardware optical flow estimation |
| `npp` | `npp-sys` | Windows & Linux | NVIDIA Performance Primitives (image & signal ops) |
| `nvjpeg` | `nvjpeg-sys` | Windows & Linux | GPU-accelerated JPEG decode, encode, and metadata |

---

## `optix` (`optix-sys`)

Safe OptiX host API on top of `optix-sys` and `cuda-core` (device raygen/shader code is written in Rust with `cuda-oxide`, as in `device/optix-device`).

- **Context & modules (`context`, `module`, `task`)**: `DeviceContext` (bound to one CUDA context with optional log sink), `Module` from PTX or OptiX-IR, `PendingModule` / `Task`, `ProgramGroup` (raygen, miss, hit group, callables), `PipelineOptions`, and `ModuleOptions`.
- **Pipelines & SBT (`pipeline`, `sbt`, `stack`)**: `Pipeline` and `Pipeline::launch` with typed launch parameters, `SbtRecord<T>` (32-byte header + 16-byte-aligned user data), `ShaderBindingTable`, and `PipelineStack` / `StackSizes`.
- **Acceleration structures & extensions (`accel`, `cluster`, `micromap`, `xform`, `coop_vec`)**: `Accel` (triangle meshes, curves, spheres, custom primitives, AABBs, and instances with compaction, byte budgets, in-place refit, and reusable `Scratch`), `ClusterAccel`, `OpacityMicromapArray`, `Transform` / `MotionOptions`, and cooperative vectors.
- **AI Denoiser (`denoise`)**: `Denoiser` supporting HDR and AOV models with albedo and normal `Guides` for RGBA `f32` images, plus tiled splitting (`split_image`).

---

## `ngx` (`ngx-sys`)

Safe NVIDIA NGX (DLSS) wrapper over `ngx-sys`.

- **Static linking & platform behavior**: `ngx-sys` is statically linked and included only on `cfg(target_os = "windows")`. Non-Windows targets compile clean stubs returning `Error::Unsupported`.
- **Project identity & enums (`types`)**: `ProjectId` / `Guid` (`NVSDK_NGX_ENGINE_TYPE_CUSTOM` with optional engine name and version), `DlssPerfQuality` (`DlssQuality`), `Feature` (`SuperSampling`, `FrameGeneration`, `RayReconstruction`), `BufferFormat`, `DlssFeatureFlags`, `RequiredExtensions`, and DLSS-RR enums (`DlssDenoiseMode`, `DlssRoughnessMode`, `DlssDepthType`, `DlssRrPreset`).
- **Parameters & Vulkan RAII (`params`, `vulkan`)**: `Parameters` (`AllocateParameters` / `GetCapabilityParameters` / `DestroyParameters`) and `NgxVulkan` / `FeatureHandle`, supporting both `NVSDK_NGX_VULKAN_Init_with_ProjectID` (via `ProjectId`) and `NVSDK_NGX_VULKAN_Init` (with an NVIDIA-issued `ApplicationId`), DLSS Super Resolution (`query_dlss_optimal_settings`, `create_dlss_sr`, `evaluate_dlss_sr_with_params`), and **DLSS Ray Reconstruction / DLSSD** (`is_dlss_rr_available`, `query_dlss_rr_optimal_settings`, `create_dlss_rr`, `evaluate_dlss_rr_with_params` with `DlssRrCreateParams` / `DlssRrEvalParams`).
- **Runtime DLL discovery**: `NgxVulkan::with_feature_common_info` populates `NVSDK_NGX_PathListInfo` with `app_data_path`, the current executable directory, `NGX_SDK_ROOT` / `DLSS_ROOT` / `NVSDK_NGX_ROOT`, and `ngx_sys::SDK_BIN` (`option_env!("NV_RS_SDK_BIN")`) so both `nvngx_dlss.dll` and `nvngx_dlssd.dll` resolve cleanly.

:::note[Redistribution]
`ngx-sys` and `ngx` only bind the NGX SDK. Applications that redistribute NVIDIA DLSS DLLs (`nvngx_dlss*.dll`) remain responsible for NVIDIA RTX SDK license compliance and pre-release notification.
:::

---

## `nvenc` (`nvenc-sys`)

Safe NVENC hardware video encoder wrapper over `nvenc-sys` and `cuda-core` (H.264, HEVC, AV1).

- **Session lifecycle**: `EncodeSession` opens an NVENC session bound to a `cuda_core::Context`, serializes all NVENC API calls and `Drop` paths through an internal `Mutex<()>` for sound `Sync` sharing, queries capabilities (`EncodeSession::caps`) and preset configs (`EncodeSession::preset_config`), initializes the encoder (`EncodeSession::initialize`), and encodes frames or flushes EOS with zero per-frame heap allocations.
- **Input & output buffers**: `InputBuffer` / `InputLock` (host-accessible driver-allocated buffers), `RegisteredResource` / `MappedResource` (zero-copy CUDA device-pointer or CUDA-array input registration), and `OutputBitstream` / `BitstreamLock` (yielding `&[u8]` encoded packets).
- **GUIDs & color VUI (`guid`, `EncodeConfig`)**: Authoritative H.264, HEVC, AV1, profile, and `P1`–`P7` preset GUIDs, plus `EncodeConfig::set_h264_bt709_vui` (H.264 High 8-bit BT.709) and `EncodeConfig::set_hevc_10bit_pq` (HEVC Main 10 10-bit BT.2020 SMPTE ST 2084 PQ).

---

## `nvdec` (`nvcuvid-sys`)

Safe NVDEC hardware video decoder and parser wrapper over `nvcuvid-sys` and `cuda-core`.

- **Parser**: `VideoParser` wraps `CUvideoparser` with a `ParserHandler` trait whose sequence, decode, and display callbacks run inside `std::panic::catch_unwind`; any Rust error or caught panic is recorded and surfaced on the next safe call (`parse_packet` / `flush_eos`) as `Error::CallbackPanic`.
- **Decoder & frame mapping**: `VideoDecoder` wraps `CUvideodecoder`, capability queries (`decoder_caps`), picture decoding (`VideoDecoder::decode_picture`), and frame mapping (`VideoDecoder::map_frame`). `MappedFrame` provides RAII `cuvidMapVideoFrame64` / `cuvidUnmapVideoFrame64` guards with zero-allocation NV12 and P016 surface readback into caller-owned host slices.

---

## `nvofa` (`nvofa-sys`)

Safe NVIDIA Optical Flow (NVOFA) CUDA wrapper over `nvofa-sys` and `cuda-core`.

- **Session & execution**: `OpticalFlowSession` opens a CUDA optical-flow handle (`nvCreateOpticalFlowCuda`), queries hardware capabilities (`nvOFGetCaps`), initializes the session (`nvOFInit` with output/hint grid sizes, cost output, and ROI support), binds input/output CUDA streams, and executes flow estimation (`nvOFExecute`) with zero per-frame allocations.
- **Buffers & types**: `OfBuffer` wraps `NvOFGPUBufferHandle` (`CUdeviceptr` or `CUarray`), queries plane strides (`nvOFGPUBufferGetStrideInfo`), and provides zero-allocation 2D upload/download and flow-vector readback. `FlowVector` represents S10.5 fixed-point `(flow_x, flow_y)` vectors with pixel conversion, and `RoiRect` validates hardware alignment.

---

## `npp` (`npp-sys`)

Safe wrapper over `npp-sys` for NVIDIA Performance Primitives (NPP).

- **Stream context (`context`)**: `NppContext` caches `NppStreamContext` (stream handle, device ID, SM/thread/shared memory properties, compute capability, stream flags) for a `cuda_core::Stream` so stream-ordered operations execute with zero per-call host allocations or driver queries. `lib_version()` queries `nppGetLibVersion`.
- **Image operations (`image`)**: `ImageView`, `ImageViewMut`, `Size`, `Rect`, `Interpolation`, `MaskSize`, `CmpOp`, ROI/stride validation, color conversion (`rgb_to_gray_8u`, `rgb_to_ycbcr_8u`, `ycbcr_to_rgb_8u`), resize (`resize_8u_c1`, `resize_8u_c3`, `resize_8u_c4`, `resize_32f_c3`), filtering (`filter_box_8u_c1`, `filter_box_8u_c3`, `filter_gauss_8u_c1`, `filter_gauss_8u_c3`), and thresholding (`threshold_val_8u_c1`, `threshold_val_32f_c1`).
- **Signal operations (`signal`)**: 1D signal operations (`set_32f`, `add_32f`, `mul_32f`, and two-phase `sum_32f_scratch_bytes` + `sum_32f`).

---

## `nvjpeg` (`nvjpeg-sys`)

Safe wrapper over `nvjpeg-sys` for GPU-accelerated JPEG decoding, encoding, and metadata parsing.

- **Handle & decode (`handle`)**: `Handle` (bound to `cuda_core::Context`), `Backend` (`Default`, `Hybrid`, `GpuHybrid`, `Hardware`), `JpegState`, `JpegDecoder`, `DecodeParams`, `PinnedBuffer`, and `NvjpegDeviceBuffer` for reusable scratch memory.
- **Metadata & encoding (`encode`)**: `JpegStream` parses JPEG headers/metadata on the host (dimensions, chroma subsampling, EXIF orientation, sample precision, encoding). `EncoderParams` and `EncoderState` encode from planar or interleaved `ImagePlanes` / `ImagePlanesMut` with zero-allocation bitstream retrieval (`retrieve_bitstream_into`).
