---
title: nv-wgpu
description: Zero-copy CUDA <-> wgpu 30 external memory and timeline semaphore interop for Vulkan and DX12.
sidebar:
  order: 2
---

`nv-wgpu` (`crates/nv-wgpu`) bridges `wgpu` 30 rendering and compute pipelines (Vulkan on Windows and Linux, Direct3D 12 on Windows) with `cuda-core` and `cuda-core-ext`, allowing GPU buffers and textures to live in device-local VRAM with zero host copies and GPU-side timeline synchronization.

## `InteropContext` and Device Matching

`InteropContext` pairs a `wgpu::Device`, `wgpu::Queue`, and matching `Arc<cuda_core::Context>`:

- `InteropContext::from_wgpu(adapter, device, queue)` — verifies external memory/semaphore support via `verify_device_interop_support` and locates the matching CUDA device by UUID/LUID (`find_matching_cuda_device`).
- `InteropContext::from_wgpu_with_cuda(adapter, device, queue, cuda_ctx)` — verifies both interop support and that the caller-supplied `cuda_ctx` refers to the same physical GPU (`verify_matching_cuda_device`).
- `InteropContext::open_vulkan(adapter, features, limits, label)` — opens a Vulkan `wgpu::Device` with `required_vulkan_extensions` enabled via `wgpu_hal::vulkan::Adapter::open_with_callback` and pairs it with CUDA.
- `InteropContext::from_shared_gpu(shared)` — wraps a `gpu_info::SharedGpu`. Returns `Error::Unsupported` without silent downgrades if required extensions are missing.

Device identity helpers (`wgpu_gpu_identity`, `wgpu_vulkan_uuid`, `wgpu_dx12_luid`) ensure `wgpu` and CUDA never silently run on different GPUs (`Error::DeviceMismatch`, `Error::NoMatchingCudaDevice`).

## Shared Buffers (`buffer`)

`InteropContext::create_buffer(&SharedBufferDescriptor)` (or `SharedBuffer::create`) allocates an exportable `wgpu::Buffer` and imports it into CUDA as `ExternalMemory` + `cuda_core::RawDeviceBuffer`:

- Accessors: `wgpu_buffer()`, `cuda_buffer()`, `cu_deviceptr()`, `external_memory()`, and `size()`.
- Ownership handoff helpers: `release_to_cuda` and `acquire_from_cuda` coordinate queue/stream transitions.

## Shared Textures and Formats (`texture`, `format`)

`InteropContext::create_texture(&SharedTextureDescriptor)` (or `SharedTexture::create`) creates a `wgpu::Texture` backed by exportable memory and imports it into CUDA as a `cuda_core_ext::MipmappedArray`:

- Accessors: `wgpu_texture()`, `cuda_mipmapped_array()`, `cuda_level(level)`, `cuda_texture_object(tex_desc)` -> `TextureObject`, and `cuda_surface_object(level)` -> `cuda_core_ext::SurfaceObject` (built with `SurfaceObject::from_level`, keeps the mipmapped array alive).
- Ownership handoff helpers: `release_to_cuda` and `acquire_from_cuda`.
- `format` provides an explicit `wgpu::TextureFormat` <-> CUDA/Vulkan/DXGI mapping table (`SUPPORTED_FORMATS`, `format_mapping`, `build_cuda_array_desc`) and 256-byte (`COPY_BYTES_PER_ROW_ALIGNMENT`) staging layout helpers (`aligned_row_pitch`, `aligned_staging_bytes`, `validate_copy_layout`).

## Timeline Semaphores (`sync`)

`InteropContext::create_semaphore(initial_value)` (or `InteropSemaphore::new`) creates a shared GPU timeline semaphore (Vulkan timeline semaphore or DX12 shared `ID3D12Fence`):

- **CUDA side**: `signal_from_cuda(stream, value)` and `wait_in_cuda(stream, value)`.
- **wgpu side**: `stage_wait_in_wgpu(queue, value)`, `stage_signal_in_wgpu(queue, value)`, `unstage_wait_in_wgpu`, `unstage_signal_in_wgpu`, and `signal_from_wgpu(queue, value)`.
- **Host side**: `completed_value()`, `signal_from_host(value)`, and `wait_on_host(value, timeout_ns)`.
