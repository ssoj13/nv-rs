# Cross-Module Seam Matrix — nv-rs (2026-11-13)

| Seam | Producer | Consumer | Contract & Verified Invariant |
|---|---|---|---|
| `vcv-rs` SDK discovery | `vcv-rs` (`31c54282`) | `nv-bindgen` (`crates/nv-bindgen`) | `vcv_rs::cuda`, `vcv_rs::nvsdk`, `vcv_rs::optix` resolved via `Cargo.lock` & `device/Cargo.lock`; `NV_SDK_DIRS` supported. |
| Checked void & query shims | `nv-bindgen` (`WrapMode`) | `*-sys` crates, `nv-bindgen-fixture` | Void shims emit `{ unsafe { ... }; Ok(()) }` (`clippy::unit_arg`-clean); `QueryInterface` emits `cargo:rerun-if-changed`. |
| Error cause chain | `nv-loader` (`CallError`, `SymbolError`) | `*-sys` crates, `nv-bindgen-fixture` | Manual `Error::source()` unwraps `&'static E` before unsizing so `.is::<libloading::Error>()` works across `thiserror` 2.0. |
| `cuda-core` Context & Stream | `cuda-core` (`0379bbf1`) | `nv-wgpu`, `cublas`, `cufft`, `curand`, `cusparse`, `cusolver`, `npp`, `nvjpeg`, `nvrtc`, `cupti`, `nvenc`, `nvdec`, `nvofa`, `cudnn`, `nvcomp`, `nccl`, `cufile` | Constructors take `&Arc<Context>`; stream ops take `&Stream` / `&Arc<Stream>`; `Drop` records teardown errors on `Context` via `record_err`; `Scratch` / `BatchStaging` validate owning stream. |
| `wgpu 30` <-> CUDA interop | `gpu-info-rs` (`657c3570`), `cuda-ext` | `nv-wgpu` | Vulkan Win/Linux (`OPAQUE_WIN32`/`OPAQUE_FD`) + DX12 (`D3D12_RESOURCE`/`ID3D12Fence`); `VkBarrierRunner` waits on live fences in `Drop`, `VK_QUEUE_SUBMIT_LOCK` serializes `VkQueue`; DX12 wait+signal flushed via `queue.submit(empty)`. |
| Video encode -> decode | `nvenc` | `nvdec` (`tests/gpu.rs`) | Typed `EncodeInput` (`Buffer` \| `Mapped`); H.264/HEVC/AV1 bitstream -> `VideoParser` (`catch_unwind`) -> `VideoDecoder` (even dims enforced) -> `MappedFrame`. |
| Linux-only SDKs | `nccl-sys`, `cufile-sys` | `nccl`, `cufile` | Target-gated `[target.'cfg(target_os = "linux")'.dependencies]`; Windows compiles cleanly and returns `Error::Unsupported` (`nccl`) / explicit `IoMode::PosixFallback` (`cufile`). |
| NGX hold | `ngx-sys` | — | Deferred per `PLAN.md` D13 pending owner confirmation. |
