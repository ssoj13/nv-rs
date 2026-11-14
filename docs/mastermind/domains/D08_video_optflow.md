# D08 — Video Codec & Optical Flow (`nvenc`, `nvdec`, `nvofa`)

Updated: 2026-11-13. Status: source and tests written; not yet compiled or executed (workers do not run `cargo`).
See [MASTERMIND_INDEX.md](../MASTERMIND_INDEX.md) and [PLAN.md](../../../PLAN.md) (§2, §4, §6 M8, §8).

## Scope and ownership

| Crate | Allowed files | Purpose |
|---|---|---|
| `nvenc` | `crates/nvenc/Cargo.toml`, `crates/nvenc/src/nvenc.rs`, `crates/nvenc/src/guid.rs`, `crates/nvenc/tests/gpu.rs` | Safe NVENC encoder (H.264/HEVC/AV1), authoritative GUIDs, CUDA ptr/array registration, RAII input/bitstream locks |
| `nvdec` | `crates/nvdec/Cargo.toml`, `crates/nvdec/src/nvdec.rs`, `crates/nvdec/tests/gpu.rs` | Safe NVDEC parser (`catch_unwind` callbacks) & decoder, `MappedFrame` RAII guard, NV12/P016 readback, NVENC->NVDEC PSNR test |
| `nvofa` | `crates/nvofa/Cargo.toml`, `crates/nvofa/src/nvofa.rs`, `crates/nvofa/tests/gpu.rs` | Safe NVOFA CUDA optical flow session, `OfBuffer`, `ExecuteParams`, grid/hint/cost/ROI support, shifted-frame flow test |

## Key components implemented

- **`nvenc`**:
  - `guid` module: authoritative `CODEC_{H264,HEVC,AV1}`, `PROFILE_AUTOSELECT`, `H264_*`, `HEVC_*`, `AV1_MAIN`, and `PRESET_P1..P7` GUIDs verified against `Video_Codec_SDK_13.1.15/Interface/nvEncodeAPI.h`.
  - `EncodeSession` (`open`, `caps`, `preset_config`, `initialize`, `set_io_streams`, `encode_picture` taking typed `EncodeInput`, `flush_eos`), `RegisteredResource` (`from_cuda_ptr`, `from_cuda_array`, `map` -> `MappedResource`), `InputBuffer` (`lock` -> `InputLock` with bounded `as_mut_slice`), `OutputBitstream` (`lock` -> `BitstreamLock` yielding zero-copy `&[u8]`).
  - Struct version constants (`VER_CONFIG`, `VER_INITIALIZE_PARAMS`, `VER_PRESET_CONFIG`, `VER_PIC_PARAMS`, `VER_LOCK_BITSTREAM`, etc.) matching `nvEncodeAPI.h` including `(1 << 31)` high-bit flags.
- **`nvdec`**:
  - `decoder_caps` (`cuvidGetDecoderCaps`), `VideoParser<H: ParserHandler>` with `Box<ParserInner<H>>` stable `pUserData` and `std::panic::catch_unwind` in `on_sequence_cb`, `on_decode_cb`, `on_display_cb` surfacing errors/panics on the next `parse_packet` / `flush_eos`.
  - `VideoDecoder` (`new` enforcing even `target_width`/`target_height`, `unsafe decode_picture`, `map_frame`), `MappedFrame<'a>` (`cuvidMapVideoFrame64` / `cuvidUnmapVideoFrame64` RAII guard) with zero-alloc `copy_nv12_to_host` and `copy_p016_to_host` via `cuMemcpy2DAsync_v2`.
  - T3 GPU test (`nvenc_to_nvdec_h264_roundtrip_psnr`) encoding 4 synthetic 128x128 NV12 frames with NVENC, decoding with NVDEC, and asserting PSNR >= 30 dB.
- **`nvofa`**:
  - `OpticalFlowSession` (`open`, `caps`, `init` setting `inputBufferFormat`, `set_io_streams`, `execute` validating session ownership), `OfBuffer` (`new`, `stride_info`, `cu_deviceptr`, `cu_array`, bounds-checked `upload_plane0`/`download_plane0`, `download_flow_vectors`), `FlowVector` (S10.5 `to_pixels`), `RoiRect` (alignment + bounds validation).
  - T3 GPU test (`optical_flow_detects_synthetic_shift`) verifying mean flow vector direction and magnitude on a 256x256 synthetic texture shifted by `(+8, +4)`.

## 7-Question Inquisitor Self-Audit

1. **Unfinished stubs / TODOs?** None. All public APIs, error paths, and tests are complete.
2. **Dead / uncalled functions?** None in library internals; all helpers are exercised.
3. **Swallowed errors / panics in FFI?** None. All `Drop` impls record teardown errors via `cuda_core::Context::record_err`; all `nvdec` C callbacks run inside `catch_unwind`.
4. **Boundary & overflow bugs?** Checked arithmetic (`checked_mul`, `checked_add`, `is_multiple_of(4)`) guards frame byte calculations, slice lengths, pitches, and ROI alignment.
5. **Hot-path memory churn?** Zero heap allocations in `encode_picture`, `parse_packet`, `decode_picture`, `map_frame`, `copy_nv12_to_host`, `execute`, and `download_flow_vectors`.
6. **Memory layout / FFI ABI safety?** `StatusCode` is `#[repr(transparent)]`; `FlowVector` and `RoiRect` are `#[repr(C)]` with `size_of` unit tests; every `unsafe` block has a `// SAFETY:` proof; `unsafe_op_in_unsafe_fn` is respected.
7. **House conventions?** `[lib] path = "src/<name>.rs"`, `[lints] workspace = true`, `//!` headers on all files, `thiserror` `Error` with `Copy` `StatusCode`, T0 unit tests + `#[ignore]` T2/T3 GPU tests.
