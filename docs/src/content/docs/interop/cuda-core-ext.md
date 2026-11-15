---
title: cuda-core-ext
description: Safe Rust wrappers for CUDA driver features beyond cuda-core — external memory/semaphores, arrays, textures/surfaces, and green contexts.
sidebar:
  order: 1
---

`cuda-core-ext` (`crates/cuda-core-ext`) wraps CUDA driver APIs not yet exposed by `cuda-core`, following `cuda-core` conventions: every handle owns its driver resource and destroys it on `Drop`, recording any teardown error on the owning `cuda_core::Context` (`Context::record_err`) rather than panicking. Fallible operations return `cuda_core::DriverError`.

## External Memory and Semaphores (`external`)

The `external` module provides the CUDA side of zero-copy graphics interop:

- `MemoryHandle` — `OpaqueWin32(*mut c_void)`, `OpaqueFd(i32)`, `D3d12Resource(*mut c_void)`, and `D3d12Heap(*mut c_void)`. Win32 NT handles are not consumed on import (the exporter still closes them), whereas POSIX file descriptors (`OpaqueFd`) are consumed on a successful import; D3D12 resources must be imported as `dedicated: true`.
- `ExternalMemory::import(ctx, handle, size, dedicated)` — imports an exported allocation via `cuImportExternalMemory` and exposes:
  - `mapped_buffer(offset, size)` -> `cuda_core::RawDeviceBuffer`
  - `mapped_mipmapped_array(desc, levels, offset)` -> `MipmappedArray`
  - `RawDeviceBuffer` and `MipmappedArray` views retain an `Arc<ExternalMemory>` so mapped views are always destroyed before the underlying external memory object.
- `SemaphoreHandle` — `OpaqueWin32`, `OpaqueFd`, `TimelineOpaqueWin32`, `TimelineOpaqueFd`, and `D3d12Fence`.
- `ExternalSemaphore::import(ctx, handle)` — imports a binary or timeline semaphore / D3D12 fence and provides `signal(stream, value)`, `wait(stream, value)`, and `is_valued()`.

## Arrays and Sparse Arrays (`array`, `sparse`)

- `array`: `Array` and `MipmappedArray` wrap `CUarray` and `CUmipmappedArray` with `ArrayDesc` and `ArrayFormat`. `Array::upload` / `Array::download` transfer bytes on a `Stream`, and `MipmappedArray::level(level)` returns a `LevelRef<'_>` view of a single mip level.
- `sparse`: `sparse_arrays_supported(ctx)` queries device support; `TilePool` manages backing memory for sparse (tiled) arrays, and `MipmappedArray::sparse_properties`, `map_tiles`, and `unmap_tiles` manage virtual tile mappings.

## Textures and Surfaces (`texture`)

- `TextureObject` wraps `CUtexObject` for filtered hardware reads, constructed via `from_array`, `from_mipmapped`, `from_pitched`, or `from_linear` with `TextureDesc`, `AddressMode`, and `FilterMode`.
- `SurfaceObject` wraps `CUsurfObject` (`SurfaceObject::new(&array)`) for image writes.
- Both expose `.handle() -> u64`, matching `TexObject` and `SurfObject` in `cuda-tex-device` for device-side texture fetches and surface stores.

## Device Identity, Host Memory, and Green Contexts

- `device_id`: `device_uuid(ctx) -> Result<[u8; 16], DriverError>` and `device_luid(ctx) -> Result<Luid, DriverError>` match a CUDA context against a Vulkan or DXGI adapter.
- `host`: `HostRegistration` page-locks caller-owned slices, and `ManagedBuffer<T>` wraps unified (managed) memory.
- `green`: `split_sms(device, min_sms)` partitions a GPU's streaming multiprocessors into `GreenContext` instances (`cuDeviceGetDevResource` -> `cuDevSmResourceSplitByCount` -> `cuDevResourceGenerateDesc` -> `cuGreenCtxCreate`). Each `GreenContext` exposes `.context()`, `.sm_count()`, and `.stream()` for isolated execution without time-slicing the entire device.
