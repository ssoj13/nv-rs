# Domain D03 (M6): `crates/nv-wgpu` — CUDA <-> `wgpu` 30 Zero-Copy Interop

> **Status**: Implementation & static/Inquisitor self-audit complete (`not compiled / not executed` by worker per swarm rule forbidding `cargo` / `git` invocation; ready for coordinator/verifier build & GPU test gate).

## 1. Scope & Files Owned

- [`crates/nv-wgpu/Cargo.toml`](../../crates/nv-wgpu/Cargo.toml) — Crate manifest with `[lib] name = "nv_wgpu"`, `path = "src/nv_wgpu.rs"`, `[lints] workspace = true`, workspace dependencies, `cuda-ext = { path = "../cuda-ext" }`, target-gated `windows = "0.62"` (Win32 + D3D12 + DXGI + Security + Threading) and `libc = "0.2"` (Unix).
- [`crates/nv-wgpu/src/nv_wgpu.rs`](../../crates/nv-wgpu/src/nv_wgpu.rs) — Crate root, public re-exports, and `InteropContext`.
- [`crates/nv-wgpu/src/error.rs`](../../crates/nv-wgpu/src/error.rs) — House `thiserror::Error` enum (`Error`) and `Result<T>`.
- [`crates/nv-wgpu/src/format.rs`](../../crates/nv-wgpu/src/format.rs) — `SUPPORTED_FORMATS` table (`wgpu::TextureFormat` <-> `cuda_ext::ArrayFormat` / `ash::vk::Format` / `DXGI_FORMAT`), 256-byte row-pitch alignment helpers, `build_cuda_array_desc`, `validate_copy_layout`.
- [`crates/nv-wgpu/src/platform.rs`](../../crates/nv-wgpu/src/platform.rs) — Internal `ExportedHandle` (Win32 `OwnedHandle` / Unix `OwnedFd` consumption tracking on CUDA import), Vulkan & DX12 handle export helpers, usage/layout translators, and `VkBarrierRunner` slot pool.
- [`crates/nv-wgpu/src/device.rs`](../../crates/nv-wgpu/src/device.rs) — Backend detection, required Vulkan extension enumeration (`OPAQUE_WIN32` / `OPAQUE_FD` + timeline semaphores), UUID/LUID device matching, `open_vulkan_device_with_interop`, and `gpu_info::SharedGpu` composition (`pair_shared_gpu`).
- [`crates/nv-wgpu/src/sync.rs`](../../crates/nv-wgpu/src/sync.rs) — `InteropSemaphore` (Vulkan timeline semaphore & Windows DX12 shared `ID3D12Fence`).
- [`crates/nv-wgpu/src/buffer.rs`](../../crates/nv-wgpu/src/buffer.rs) — `SharedBufferDescriptor` & `SharedBuffer` (`wgpu::Buffer` <-> `cuda_core::RawDeviceBuffer`).
- [`crates/nv-wgpu/src/texture.rs`](../../crates/nv-wgpu/src/texture.rs) — `SharedTextureDescriptor`, `SharedSurfaceObject`, & `SharedTexture` (`wgpu::Texture` <-> `cuda_ext::MipmappedArray`, `TextureObject`).
- [`crates/nv-wgpu/tests/gpu.rs`](../../crates/nv-wgpu/tests/gpu.rs) — `#[ignore = "GPU"]` T2/T3 integration tests (Vulkan & DX12 buffer/texture roundtrips, timeline semaphore sync, `gpu_info::shared_device()` composition, and negative validation/device-mismatch tests).

---

## 2. Architecture & Design Invariants

### Backend Support & Zero Silent Downgrades
- **Vulkan (Windows & Linux)**:
  - `required_vulkan_extensions(device_api_version)` ([device.rs:70](../../crates/nv-wgpu/src/device.rs#L70)) selects platform handle extensions (`VK_KHR_external_memory_win32` + `VK_KHR_external_semaphore_win32` on Windows; `VK_KHR_external_memory_fd` + `VK_KHR_external_semaphore_fd` on Unix) plus `VK_KHR_external_memory`, `VK_KHR_external_semaphore`, and `VK_KHR_timeline_semaphore` when `device_api_version` precedes Vulkan 1.1/1.2.
  - `open_vulkan_device_with_interop` ([device.rs:285](../../crates/nv-wgpu/src/device.rs#L285)) checks `physical_device_capabilities().supports_extension(...)` upfront, failing loudly with `Error::Unsupported` if any extension is missing, then injects required extensions into `wgpu_hal::vulkan::Adapter::open_with_callback`.
  - `verify_device_interop_support` ([device.rs:95](../../crates/nv-wgpu/src/device.rs#L95)) verifies live extensions + `timeline_semaphore` support on any externally opened `wgpu::Device` (including `gpu_info::SharedGpu`).
- **Direct3D 12 (Windows)**:
  - Uses `D3D12_HEAP_FLAG_SHARED` committed resources (`D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS` for shared textures) and `D3D12_FENCE_FLAG_SHARED` fences exported via `ID3D12Device::CreateSharedHandle` (`GENERIC_ALL`).
  - LUID matching via `IDXGIAdapter3::GetDesc2` (`AdapterLuid`) compared byte-for-byte against `cuda_ext::device_luid(&ctx).bytes`.
- **Unsupported Backends & Formats**:
  - `detect_backend` ([device.rs:48](../../crates/nv-wgpu/src/device.rs#L48)) returns `Error::Unsupported` on Metal/GLES/WebGPU.
  - `format_mapping` ([format.rs:338](../../crates/nv-wgpu/src/format.rs#L338)) rejects depth/stencil, compressed BC/ETC2/ASTC, packed 10-bit, 3-channel, and signed 8/16-bit formats with `Error::Unsupported` without silent conversion.

### Ownership, Drop Order, and Cycle Avoidance
- **Zero `wgpu::Device` Arc Cycle**:
  - `SharedBuffer` ([buffer.rs:114](../../crates/nv-wgpu/src/buffer.rs#L114)) and `SharedTexture` ([texture.rs:165](../../crates/nv-wgpu/src/texture.rs#L165)) keep `VkBufferBacking` / `VkImageBacking` in an `Arc` holding `ash::Device` (no `wgpu::Device` back-reference) shared between the struct and `wgpu-hal`'s `DropCallback`.
  - Structural field order enforces teardown order:
    1. `SharedBuffer`: `cuda_buf (RawDeviceBuffer)` -> `ext_mem (Arc<ExternalMemory>)` -> `wgpu_buf (wgpu::Buffer)` -> `backing (Arc<VkBufferBacking> / ID3D12Resource)`.
    2. `SharedTexture`: `cuda_array (Arc<MipmappedArray>)` -> `ext_mem (Arc<ExternalMemory>)` -> `wgpu_tex (wgpu::Texture)` -> `backing (Arc<VkImageBacking> / ID3D12Resource)`.
    3. `InteropSemaphore`: `cuda (Arc<ExternalSemaphore>)` -> `native (VkSemaphore / ID3D12Fence)` -> `_device (wgpu::Device)`.
  - CUDA child drops record driver teardown errors onto `cuda_core::Context` (`Context::record_err`) via `cuda-ext` destructors without panicking.
- **POSIX FD Ownership**:
  - `ExportedHandle::import_cuda_memory` and `import_cuda_semaphore` ([platform.rs:83](../../crates/nv-wgpu/src/platform.rs#L83), [platform.rs:128](../../crates/nv-wgpu/src/platform.rs#L128)) leak `OwnedFd` via `std::mem::forget` *only* when `cuImportExternal*` returns `Ok`, letting `OwnedFd::drop` close the descriptor on error. On Windows, `OwnedHandle` closes the duplicated NT handle unconditionally on drop.

### Zero Hot-Path Allocations & Vulkan Barrier Safety
- `InteropSemaphore` methods (`signal_from_cuda`, `wait_in_cuda`, `stage_wait_in_wgpu`, `stage_signal_in_wgpu`, `unstage_*`, `signal_from_wgpu`, `completed_value`, `signal_from_host`) use stack arrays only — zero heap allocations. Both Vulkan and DX12 `signal_from_wgpu` / ownership barrier waits & signals route uniformly through `stage_wait_in_wgpu` / `stage_signal_in_wgpu` + `queue.submit(core::iter::empty())`, flushing `wgpu` `pending_writes` and preserving `wait`-before-`signal` order.
- `VkBarrierRunner` ([platform.rs:471](../../crates/nv-wgpu/src/platform.rs#L471)) pre-allocates 2 signaled `(VkCommandBuffer, VkFence)` slots under a `Mutex<Vec<BarrierSlot>>`. Each barrier submission inspects `vkGetFenceStatus` without CPU blocking, records the barrier before resetting `slot.fence` right before `vkQueueSubmit` (discarding & freeing the slot if `vkQueueSubmit` fails so no fence stays stuck unsignaled), and serializes raw `VkQueue` access across all `SharedBuffer` / `SharedTexture` instances under `VK_QUEUE_SUBMIT_LOCK` (`with_vk_queue` at [platform.rs:435](../../crates/nv-wgpu/src/platform.rs#L435)). `VkBarrierRunner::drop` waits (`vkWaitForFences`) on any in-flight barrier fence before destroying fences and the command pool (`VUID-vkDestroyFence-fence-01120` / `00041`).

---

## 3. Public API Symbol Ledger

| Module | Key Public Items (`file:line`) |
| --- | --- |
| `nv_wgpu` | `InteropContext` (`nv_wgpu.rs:60`), `from_wgpu` (`:79`), `from_wgpu_with_cuda` (`:96`), `open_vulkan` (`:114`), `from_shared_gpu` (`:131`), `backend` (`:142`), `device` (`:147`), `queue` (`:152`), `cuda_context` (`:157`), `verify` (`:162`), `create_semaphore` (`:167`), `create_buffer` (`:172`), `create_texture` (`:177`) |
| `error` | `Result<T>` (`error.rs:15`), `Error` (`error.rs:19`: `Driver`, `Unsupported`, `BackendMismatch`, `DeviceMismatch`, `NoMatchingCudaDevice`, `Vulkan`, `Dx12` [win], `InvalidInput`, `InvalidHandle`, `WgpuRequest`) |
| `format` | `COPY_BYTES_PER_ROW_ALIGNMENT` (`format.rs:19`), `FormatMapping` (`:24`), `FormatMapping::dxgi_format` (`:48` [win]), `SUPPORTED_FORMATS` (`:58`), `format_mapping` (`:338`), `aligned_row_pitch` (`:356`), `aligned_staging_bytes` (`:372`), `align_up_u32` (`:398`), `align_up_u64` (`:411`), `build_cuda_array_desc` (`:423`), `validate_copy_layout` (`:507`) |
| `device` | `InteropBackend` (`device.rs:29`), `GpuIdentity` (`:39`), `detect_backend` (`:48`), `required_vulkan_extensions` (`:70`), `verify_device_interop_support` (`:95`), `wgpu_gpu_identity` (`:145`), `wgpu_vulkan_uuid` (`:154`), `wgpu_dx12_luid` (`:190` [win]), `verify_matching_cuda_device` (`:213`), `find_matching_cuda_device` (`:244`), `open_vulkan_device_with_interop` (`:285`), `pair_shared_gpu` (`:352`) |
| `sync` | `InteropSemaphore` (`sync.rs:45`), `new` (`:94`), `signal_from_cuda` (`:202`), `wait_in_cuda` (`:211`), `stage_wait_in_wgpu` (`:220`), `stage_signal_in_wgpu` (`:256`), `unstage_wait_in_wgpu` (`:281`), `unstage_signal_in_wgpu` (`:306`), `signal_from_wgpu` (`:332`), `completed_value` (`:341`), `signal_from_host` (`:375`), `wait_on_host` (`:407`), `vulkan_handle` (`:472`), `cuda_semaphore` (`:481`) |
| `buffer` | `SharedBufferDescriptor<'a>` (`buffer.rs:42`), `validate_buffer_descriptor` (`:53`), `SharedBuffer` (`:114`), `create` (`:129`), `wgpu_buffer` (`:367`), `cuda_buffer` (`:373`), `cu_deviceptr` (`:379`), `external_memory` (`:385`), `size` (`:391`), `release_to_cuda` (`:406`), `acquire_from_cuda` (`:422`) |
| `texture` | `SharedTextureDescriptor<'a>` (`texture.rs:46`), `d2` (`:76`), `validate_texture_descriptor` (`:107`), `SharedSurfaceObject` (`:134`), `handle` (`:149`), `SharedTexture` (`:207`), `create` (`:230`), `wgpu_texture` (`:593`), `cuda_mipmapped_array` (`:599`), `cuda_level` (`:605`), `cuda_texture_object` (`:610`), `cuda_surface_object` (`:617`), `format_mapping` (`:643`), `array_desc` (`:649`), `external_memory` (`:655`), `release_to_cuda` (`:670`), `acquire_from_cuda` (`:693`) |

---

## 4. Inquisitor 7-Question Self-Audit & Re-Audit Repairs

Static audit conducted across all 9 `.rs` files in `crates/nv-wgpu`, plus all 4 defects + 1 nit from `#55`/`#56` fixed:

1. **Unfinished stubs / TODOs / placeholders?** — Pass. Automated scan found zero matches for `TODO`, `FIXME`, `todo!`, or `unimplemented!`.
2. **Dead code / unused functions or platform imports?** — Pass. `#[cfg(windows)]` / `#[cfg(unix)]` cleanly partition OS-specific handle imports (`OwnedHandle` vs `OwnedFd`), `D3d12` HAL types, `DXGI_FORMAT`, `Luid`, `flush_wgpu_wait_signal`, and DX12 `ExternalMemoryKind`/`ExternalSemaphoreKind` variants.
3. **Swallowed errors / `#[allow(...)]` / silent downgrades?** — Pass. Automated scan confirmed zero `#[allow(...)]` attributes. All `Error::BackendMismatch` sites now report actual `device.adapter_info().backend`, `queue_backend_kind(queue)`, or `sem.wgpu_backend()` instead of `wgpu::Backend::Noop`.
4. **Production `unwrap()` / `expect()` / unchecked indexing?** — Pass. Zero `.unwrap()` or `.expect()` calls outside `#[cfg(test)]` and `tests/gpu.rs`. Fixed `sync.rs` sub-millisecond WaitForSingleObject rounding and `nv_wgpu.rs:176` `&self.queue` pass-through.
5. **Boundary & descriptor validation?** — Pass. Fixed cubemap array `layered` flag (`format.rs:473`: `if cubemap { size.depth_or_array_layers > 6 } else { size.depth_or_array_layers > 1 }`) and honored `SharedTextureDescriptor::d2` documented `initial_state = STORAGE_READ_WRITE` when `cuda_surface || STORAGE_BINDING` (`texture.rs:83`).
6. **Memory layout, alignment, DX12 ordering & Vulkan barrier safety?** — Pass. Every `unsafe` block carries a preceding `// SAFETY:` comment. Fixed `VkBarrierRunner::drop` to `vkWaitForFences` on any live slot fences before destroy, moved `vkResetFences` to right before `vkQueueSubmit` with slot discard on failure, added `VK_QUEUE_SUBMIT_LOCK` + `# Vulkan Queue Synchronization` docs, and routed DX12 `signal_from_wgpu` / ownership barrier waits & signals through `queue.submit(core::iter::empty())`.
7. **Zero hot-path allocations?** — Pass. Per-frame `signal_*`, `wait_*`, `stage_*`, `unstage_*`, `release_to_cuda`, and `acquire_from_cuda` perform zero heap allocations in steady state.

---

## 5. Verification Status

- **Static checks executed**:
  - Cross-crate API verification against `wgpu 30.0.2`, `wgpu-hal 30.0.2`, `wgpu-types 30.0.2`, `ash 0.38.0`, `windows 0.62.2`, `cuda-core`, `cuda-ext`, and `gpu-info-rs` sources.
  - PowerShell automated scans confirming: all 9 `.rs` files start with `//!`, zero `TODO`/`FIXME`/`todo!`/`unimplemented!`/`#[allow`, zero `unwrap`/`expect` in library code, zero `unsafe` blocks without `// SAFETY:`, zero missing doc comments on public symbols, and `<= 7` args on every function.
- **Not yet executed (blocked by worker rule)**:
  - `cargo check` / `cargo clippy` / `cargo test` were **NOT** executed because workers are prohibited from invoking `cargo` or `git`.
  - Recommended verifier commands:
    - T0 unit + lint gate: `cargo clippy -p nv-wgpu --all-targets -- -D warnings` and `cargo test -p nv-wgpu --lib`
    - T2/T3 GPU interop gate (NVIDIA GPU host): `cargo test -p nv-wgpu --test gpu -- --include-ignored`
