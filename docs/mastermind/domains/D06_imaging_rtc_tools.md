# Domain D06 (M7c): Imaging, Runtime Compilation & Profiling Tools

Owner: `d06-img-rtc`
Scope: `crates/npp`, `crates/nvjpeg`, `crates/nvrtc`, `crates/cupti`

## 1. Implemented Crates

| Crate | Sys Crate | Key Types & APIs | Status |
|---|---|---|---|
| `npp` | `npp-sys` | `NppContext` (cached `NppStreamContext`), `NppStatus(i32)`, `Size`, `Rect`, `ImageView`/`ImageViewMut` (ROI & stride validation), `rgb_to_gray_8u`, `rgb_to_ycbcr_8u`, `ycbcr_to_rgb_8u`, `resize_{8u_c1,8u_c3,8u_c4,32f_c3}`, `filter_{box,gauss}_8u_{c1,c3}`, `threshold_val_{8u,32f}_c1`, `signal::{set_32f,add_32f,mul_32f,sum_32f_scratch_bytes,sum_32f}` | Source complete (uncompiled; awaiting root `cargo` gate) |
| `nvjpeg` | `nvjpeg-sys` | `Handle` (`Drop` -> `Context::record_err`), `NvjpegStatus(u32)`, `JpegState`, `JpegStream` (dimensions, components, chroma subsampling, EXIF orientation, sample precision, SOF encoding), `PinnedBuffer`, `NvjpegDeviceBuffer`, `EncoderParams`, `EncoderState` (`encode_image`, `encode_yuv`, `retrieve_bitstream_into`), `ImagePlanes`/`ImagePlanesMut` | Source complete (uncompiled; awaiting root `cargo` gate) |
| `nvrtc` | `nvrtc-sys` | `Program`, `NvrtcStatus(u32)`, `CompileOptions`, `compile` (build log on success & `Error::Compilation`), `ptx`, `ptx_string`, `cubin`, `ltoir`, `optixir`, `add_name_expression`, `lowered_name`, `version`, `supported_archs` | Source complete (uncompiled; awaiting root `cargo` gate) |
| `cupti` | `cupti-sys` | `ActivityProfiler`, `CuptiStatus(u32)`, `AlignedBufferPool` (8-byte aligned `Box<[u64]>` reuse), `ActivityKind`, `ActivityRecord`, `Subscriber` (`cuptiSubscribe`/`cuptiUnsubscribe` + `catch_unwind` + `Drop` teardown on `Context::record_err`), `CallbackDomain`, `callback_name`, `version` | Source complete (uncompiled; awaiting root `cargo` gate) |

## 2. Architecture & House Conventions Notes

- **CUPTI Events / Metrics / Profiler Target APIs**: `cupti-sys` (`crates/cupti-sys/build.rs`) binds `cupti.h` only (version, result strings, Activity API, Callback API). Legacy `cupti_events.h`/`cupti_metrics.h` and `cupti_profiler_target.h` are not present in `cupti-sys`, so `cupti` wraps the Activity and Callback APIs honestly without stubbing unbound symbols.
- **Zero-Alloc Hot Paths**:
  - `npp::NppContext` queries CUDA device attributes and stream flags once at construction and passes `NppStreamContext` by value with zero per-call allocations.
  - `nvjpeg::EncoderState::retrieve_bitstream_into` reuses the caller's `Vec<u8>` capacity.
  - `cupti::AlignedBufferPool` caches 8-byte-aligned `Box<[u64]>` buffers across `buffer_requested` / `buffer_completed` callbacks.
- **FFI Callback Safety**: `cupti` activity and subscriber trampolines wrap Rust closures in `std::panic::catch_unwind` and surface recorded panic messages on the next safe call (`flush`, `drain_records`, `check_panic`).

## 3. Tests Written

- **T0 (no GPU)**:
  - `npp`: `NppStatus` error/warning distinction, `Size`/`Rect` bounds overflow/zero checks, `validate_image_layout` stride and buffer length checks.
  - `nvjpeg`: `validate_plane` pitch/buffer bounds, `ChromaSubsampling`/`JpegEncoding` unknown-encode rejection, `NvjpegStatus` formatting.
  - `nvrtc`: `CompileOptions` builder flags and interior NUL rejection, `NvrtcStatus`/`Error::Compilation` formatting, `trim_nul_utf8`.
  - `cupti`: `AlignedBufferPool` 8-byte alignment, pointer reuse, double-release rejection, `CuptiStatus` formatting.
- **T2/T3 (`#[ignore = "GPU"]`, in `tests/gpu.rs`)**:
  - `npp`: `color_convert_and_resize_roundtrip`, `filter_threshold_and_signal_ops`.
  - `nvjpeg`: `encode_decode_psnr_interleaved_and_planar` (PSNR > 35 dB + `JpegStream` metadata + `PinnedBuffer`/`NvjpegDeviceBuffer`).
  - `nvrtc`: `version_and_supported_archs`, `compile_ptx_inspect_and_launch_kernel` (template name expression + `cuModuleLoadData` + `cuLaunchKernel`), `compile_error_surfaces_build_log`, `compile_cubin_for_supported_sm`.
  - `cupti`: `activity_captures_memcpy_records`, `subscriber_captures_driver_api_callbacks`.

## 4. Inquisitor Self-Audit

- **Unfinished stubs / TODOs**: None.
- **Dead code / swallowed errors**: None; `Drop` paths in `nvjpeg` and `cupti` record teardown failures via `cuda_core::Context::record_err`.
- **Unsafe / lint compliance**: `[lib] path = "src/<name>.rs"`, `[lints] workspace = true`, `//!` headers on all files, `///` docs on all public items, `// SAFETY:` comments on every `unsafe` block, `unsafe { ... }` inside `unsafe fn`s.
- **Blocked / Not Run**: `cargo check`, `cargo clippy`, and `cargo test` were intentionally not run per swarm directive (root owns workspace `Cargo.toml` and serialized build/test gates).
- **Inquisitor-2 Fixes (#29)**:
  1. `nvjpeg::JpegState` retains `Arc<PinnedBuffer>` and `Arc<NvjpegDeviceBuffer>` (`Mutex`-guarded handles), eliminating safe-Rust UAF on attached scratch buffers.
  2. `nvjpeg::ImagePlanes` / `ImagePlanesMut` track `PlaneLayout` (`Interleaved`, `Planar3`, `SingleY`) and validate against `InputFormat` / `OutputFormat` (plus `ImagePlanesMut::single_y`).
  3. `npp::ImageView` / `ImageViewMut` add `subview` / `subview_mut` and `BorderPad` tracking; `filter_box_*` / `filter_gauss_*` require sufficient neighborhood border padding.
  4. Removed `.unwrap_or(0)` / `.unwrap_or_default()` error coercions in `npp/src/context.rs`, `nvjpeg/src/handle.rs`, and `nvrtc/src/program.rs`.
  5. `cupti/src/activity.rs` reuses a pre-allocated `scratch` `Vec<ActivityRecord>` in `buffer_completed` and adds `drain_records_into`; `nvrtc/tests/gpu.rs` asserts `!archs.is_empty()` before indexing `archs[0]`.
