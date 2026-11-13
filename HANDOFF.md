# Handoff — spikes S1a / S2 / S3 (2026-11-12, paused for shell restart)

## State

- **S1a** (`spikes/s1a-optix-ptx`): **PASS** (see PLAN.md §8).
  - Device: `__raygen__ids` reads `_optix_get_launch_index_{x,y}` via `ptx_asm!` `call`, writes
    `index + 1` into the buffer from `.const params` (`#[constant(export_name = "params")]`).
    `__miss__noop` is an empty miss program.
  - Host: `nvoptix.dll` from DriverStore, `optixQueryFunctionTable(118)`, bindgen over
    `optix_function_table.h` (build.rs, needs `OPTIX_ROOT` + `CUDA_PATH`), module from the
    embedded PTX payload, raygen + miss pipeline, SBT, 4x4 launch, per-cell check. Dumps the PTX
    to `target/s1a.ptx`.
  - Build: `cd spikes/s1a-optix-ptx && OPTIX_ROOT=C:/projects/projects.rust.cg/nv/optix-dev cargo oxide build`
    then `cargo oxide run`.
- **S2**: designed, no code. Build on gpu-info-rs: `gpu_info::shared_device()` (Vulkan device
  already has `VK_KHR_external_memory_win32` + `VK_KHR_external_semaphore_win32`),
  `gpu_info::external::SharedBuffer::export(&device, size)` -> `Export { handle, allocation,
  memory_type }`, `acquire`/`release` ownership command buffers, `device_uuid`. Steps:
  1. force `WGPU_BACKEND=vulkan`; compare `device_uuid` with `cuDeviceGetUuid`;
  2. `cuImportExternalMemory(OPAQUE_WIN32, handle, allocation, DEDICATED)` +
     `cuExternalMemoryGetMappedBuffer`;
  3. timeline `VkSemaphore` exported as `OPAQUE_WIN32`, imported as
     `TIMELINE_SEMAPHORE_WIN32`;
  4. CUDA writes pattern A, signals 1; wgpu `Queue::as_hal` -> `add_wait_semaphore(sem, Some(1))`,
     submits acquire + copy to readback, checks A (also proves no lazy clear);
  5. wgpu writes pattern B, release, `add_signal_semaphore(sem, Some(2))`; CUDA waits 2, reads B.
- **S3**: not started.

## Blockers found

1. **Disk C full** (0.59 GB free at 20:45). Builds fail or misbehave. Owner is freeing space.
2. **Stale shared cuda-oxide backend cache**: `C:\Users\joss1\.cargo\cuda-oxide\rustc_codegen_cuda.dll`
   (17:43) links `rustc_driver-9cc664dca72b379e.dll` from toolchain **1.99.0**; the pinned
   toolchain is **1.99.1** (`rustc_driver-58995fd3f1c8ecc6.dll`). Result: `couldn't load codegen
   backend ... os error 126`. `cargo oxide` claimed a rebuild but finished in 9 s and kept the old
   file (likely the full disk). Shared with render-rs sessions — do not delete without the owner.
   Retry `cargo oxide build` once space is free; if the error persists, ask the owner to remove
   the cached DLL so it rebuilds.

## Environment facts

- OptiX headers: `C:\projects\projects.rust.cg\nv\optix-dev` (public NVIDIA/optix-dev clone,
  OptiX 9.1.0, ABI 118). Never copied into this repo.
- GPU: RTX 3080 Ti (sm_86), driver 621.14. CUDA 13.3 at `C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.3`.
- Fork checkout `nv\cuda-oxide-windows` is on `feat/committed-bindings` with the owner's
  uncommitted work — read only. Spikes take cuda-rust_b `main` by git URL.
- render-rs `953c889` removed `pt_integrator::trace_path_gpu` (now `trace_path_with_options`);
  relevant for M10.

## Next

1. Retry S1a build; on success run it and record the gate result in PLAN.md §8.
2. If OptiX rejects the PTX: save `target/s1a.ptx` + the OptiX log, stop, reassess (PLAN.md §8 fallback).
3. Write and run S2, then S3. One build at a time; the machine is shared.
