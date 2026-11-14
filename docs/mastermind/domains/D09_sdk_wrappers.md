# Domain D09: Higher-Level SDK Safe Wrappers (`cudnn`, `nvcomp`, `nccl`, `cufile`)

- **Milestone**: M9
- **Owner**: `d09-sdk`
- **Date**: 2026-11-13
- **Verification Scope**: Static & manual self-audit completed across all 4 crates (`//!` headers, `///` docs, `// SAFETY:` comments, zero `#[allow(...)]`, zero `.unwrap()`/`.expect()` outside tests, cross-platform `cfg(target_os = "linux")` hygiene, C header ABI verification). **`cargo` and `git` were NOT run by this worker** per swarm boundary; workspace compilation, `clippy --all-targets -- -D warnings`, T0 unit tests, and `#[ignore]` T2/T3 GPU tests must be executed by `@coordinator`.

---

## 1. Crate & File Inventory

### `crates/cudnn` (wraps `cudnn-sys`)
| File | Lines | Core Types & Responsibilities |
|---|---:|---|
| [Cargo.toml](../../../crates/cudnn/Cargo.toml) | 20 | `[lib] path = "src/cudnn.rs"`, `[lints] workspace = true` |
| [src/cudnn.rs](../../../crates/cudnn/src/cudnn.rs) | 433 | `#[repr(transparent)] CudnnStatus(pub u32)`, `Error`, `check`, `DataType`, `TensorFormat`, `NanPropagation`, `DeviceSpan`, `version`, `max_device_version`, `cudart_version` |
| [src/handle.rs](../../../crates/cudnn/src/handle.rs) | 87 | RAII `Handle` (`cudnnCreate`/`cudnnDestroy`), `set_stream`/`ensure_stream`, context-recorded `Drop` |
| [src/scratch.rs](../../../crates/cudnn/src/scratch.rs) | 95 | `Scratch` stream-bound reusable device workspace (`Scratch::new(&stream)`, `capacity`, `growth`, `reserve` with stream affinity + byte-budget gate) |
| [src/descriptor.rs](../../../crates/cudnn/src/descriptor.rs) | 869 | `TensorDescriptor`, `FilterDescriptor`, `ConvolutionDescriptor`, `ActivationDescriptor`, `PoolingDescriptor`, `MAX_DIMS = 8` stack arrays, `record_teardown` on init error |
| [src/ops.rs](../../../crates/cudnn/src/ops.rs) | 590 | `conv_forward_workspace_size`, `conv_forward`, `activation_forward`, `pooling_forward`, `softmax_forward`, `derive_bn_tensor_desc`, `batch_norm_forward_inference`, `ScalePair` |

### `crates/nvcomp` (wraps `nvcomp-sys`)
| File | Lines | Core Types & Responsibilities |
|---|---:|---|
| [Cargo.toml](../../../crates/nvcomp/Cargo.toml) | 20 | `[lib] path = "src/nvcomp.rs"`, `[lints] workspace = true` |
| [src/nvcomp.rs](../../../crates/nvcomp/src/nvcomp.rs) | 259 | `#[repr(transparent)] NvcompStatus(pub u32)` & `BitcompStatus(pub i32)`, `Error`, `check`, `check_bitcomp` |
| [src/format.rs](../../../crates/nvcomp/src/format.rs) | 1186 | `FormatKind`, `Format` (`Lz4`, `Snappy`, `Zstd`, `Cascaded`, `Gdeflate`, `Bitcomp`), `DataType`, `BitshuffleMode`, `DecompressBackend`, `Alignments`, uniform C batched dispatch |
| [src/scratch.rs](../../../crates/nvcomp/src/scratch.rs) | 341 | Stream-bound `Scratch` + `BatchStaging` (`PinnedHostBuffer` + `DeviceBuffer` tables + `cuda_core::Event` DMA/kernel fence for zero steady-state heap allocs) |
| [src/batched.rs](../../../crates/nvcomp/src/batched.rs) | 479 | `compress_batched_async`, `query_decompress_sizes_async`, `decompress_batched_async` (post-kernel `arm_dma_fence` + stream check), receipts (`BatchedCompressReceipt`, `BatchedDecompressReceipt`) |
| [src/manager.rs](../../../crates/nvcomp/src/manager.rs) | 314 | Stream-bound stateful `CompressionManager` (`new(&stream, ...)`) and `CompressedBuffer` / `CompressedLayout` built cleanly over the C batched APIs |
| [src/bitcomp.rs](../../../crates/nvcomp/src/bitcomp.rs) | 250 | Native `BitcompPlan` (`bitcompCreatePlan`/`bitcompCompressLossless`/`Lossy`/`bitcompUncompress`), `BitcompDataType`, `BitcompAlgo` |

### `crates/nccl` (wraps `nccl-sys` on Linux; cross-platform types/T0 on Windows)
| File | Lines | Core Types & Responsibilities |
|---|---:|---|
| [Cargo.toml](../../../crates/nccl/Cargo.toml) | 22 | `[lib] path = "src/nccl.rs"`, `nccl-sys` under `[target.'cfg(target_os = "linux")'.dependencies]`, `[lints] workspace = true` |
| [src/nccl.rs](../../../crates/nccl/src/nccl.rs) | 395 | `#[repr(transparent)] NcclStatus(pub u32)`, `Error`, `check`, `record_teardown`, `DataType`, `RedOp`, `DeviceSpan`, `version` |
| [src/comm.rs](../../../crates/nccl/src/comm.rs) | 894 | `UniqueId` (`[u8; 128]`), `AsyncState`, `GroupGuard`, `group_start`/`group_end`/`group`, `Comm` (`init_all`, `init_rank`, `async_error`, `last_error`, `finalize`, `abort`, `all_reduce`, `broadcast`, `reduce`, `all_gather`, `reduce_scatter`, `send`, `recv`), abort-safe `Drop` |

### `crates/cufile` (wraps `cufile-sys` on Linux; cross-platform `IoMode::PosixFallback` + T0 on Windows)
| File | Lines | Core Types & Responsibilities |
|---|---:|---|
| [Cargo.toml](../../../crates/cufile/Cargo.toml) | 22 | `[lib] path = "src/cufile.rs"`, `cufile-sys` under `[target.'cfg(target_os = "linux")'.dependencies]`, `[lints] workspace = true` |
| [src/cufile.rs](../../../crates/cufile/src/cufile.rs) | 539 | `#[repr(transparent)] CufileStatus(pub u32)`, `IoMode` (`GdsOnly`, `PosixFallback`), `Error`, `decode_io_ret`, `DeviceSpan`/`DeviceSpanMut`, `version` |
| [src/handle.rs](../../../crates/cufile/src/handle.rs) | 1103 | `DriverSession`, `DriverProperties`, `BufRegisterFlags`, `StreamFlags`, `Arc`-pinned `RegisteredBuffer<B>` & `FileHandle`, `RegisteredStream`, `AsyncIoSlot` (stationary `Box<AsyncParams>` + `cuda_core::Event` fence + `Arc` pins), non-blocking sync/async I/O |

---

## 2. Key Architectural Decisions

1. **Two-Phase Workspace Sizing & Zero Steady-State Allocations**:
   - `cudnn::ops::conv_forward` queries `conv_forward_workspace_size` first, computes `scratch.growth(needed)`, enforces `budget` (`Error::Budget`) before any device allocation, and reuses `Scratch` across launches. Stack `[i32; MAX_DIMS]` arrays avoid heap allocations for descriptor creation.
   - `nvcomp::batched` + `CompressionManager` query `temp_bytes` and `decompress_temp_bytes` up front, account for `BatchStaging::growth(chunks, temp_bytes)` (device table growth + scratch growth) against `budget`, and synchronize via a `cuda_core::Event` DMA fence (`dma_fence`) so pinned host tables are reused across consecutive async launches with zero steady-state allocations.
   - `cufile::AsyncIoSlot` pre-allocates a stationary `Box<AsyncParams>` once in `AsyncIoSlot::new` and guards pointer lifetimes (`*size_p`, `*file_offset_p`, `*buf_offset_p`, `*bytes_transferred_p`) with a `cuda_core::Event` fence recorded at `cuFileReadAsync` / `cuFileWriteAsync` launch.
2. **nvCOMP `CompressionManager` Over Batched C APIs**:
   - `nvcomp` 6 C headers (`nvcomp/*.h`) expose `nvcompBatched*` and `bitcomp*` C symbols, whereas `nvcompManager` is a C++ class template. `nvcomp::manager::CompressionManager` implements the manager workflow cleanly over `nvcompBatched*` without fragile C++ ABI shims, keeping chunk layout metadata explicit in `CompressedLayout` and validating against the caller's byte budget.
3. **Explicit `IoMode` Policy in `cufile` (Zero Silent Downgrade)**:
   - `IoMode::GdsOnly` invokes `cuFile*` strictly and propagates `Error::Status`, `Error::Load`, or `Error::Unsupported` (non-Linux) without ever falling back to POSIX.
   - `IoMode::PosixFallback` must be selected explicitly by the caller and uses `FileExt::read_at`/`write_at` (Unix) or `seek_read`/`seek_write` (Windows) + reusable `PinnedHostBuffer<u8>` staging and CUDA driver copies.
4. **Drop Ordering & Teardown Error Recording**:
   - `cufile::RegisteredBuffer::drop` calls `cuFileBufDeregister` before the wrapped device buffer drops. `cufile::FileHandle::drop` calls `cuFileHandleDeregister` before the owned `std::fs::File` closes.
   - `nccl::Comm::drop` checks `ncclCommGetAsyncError` / pending faults and calls `ncclCommAbort` when unhealthy instead of blocking indefinitely in `ncclCommDestroy`.
   - All `Drop` implementations record native teardown errors onto `cuda_core::Context::record_err` and never panic.
5. **Cross-Platform Windows + Linux Hygiene**:
   - `nccl-sys` and `cufile-sys` are `#![cfg(target_os = "linux")]`; both `nccl` and `cufile` place their `-sys` dependency under `[target.'cfg(target_os = "linux")'.dependencies]` and compile cleanly on Windows (returning explicit `Error::Unsupported` on GDS/NCCL OS-only entry points).

---

## 3. Test Coverage Matrix

### Tier 0 (Non-GPU Unit Tests)
- `cudnn`:
  - `cudnn.rs`: `t0_status_and_datatype_mapping`, `t0_budget_and_i32_bounds`
  - `scratch.rs`: `t0_growth_math`
  - `descriptor.rs`: `t0_rank_and_enum_validation`
  - `ops.rs`: `t0_scale_pair_abi_selection`
- `nvcomp`:
  - `nvcomp.rs`: `t0_status_and_budget_checks`
  - `format.rs`: `t0_format_defaults_and_validation`
  - `scratch.rs`: `t0_table_device_byte_accounting`
  - `batched.rs`: `t0_aligned_stride_and_range_validation`
  - `manager.rs`: `t0_chunk_planning_and_alignment`
  - `bitcomp.rs`: `t0_bitcomp_dtype_mapping`
- `nccl`:
  - `nccl.rs`: `t0_status_dtype_redop_invariants`, `t0_non_linux_explicit_unsupported`
  - `comm.rs`: `t0_unique_id_roundtrip_and_span_checks`
- `cufile`:
  - `cufile.rs`: `t0_status_and_io_ret_decoding`, `t0_non_linux_gds_explicit_unsupported_never_downgrades`
  - `handle.rs`: `t0_flags_and_posix_offset_io_invariants`

### Tier 2 / Tier 3 (`#[ignore]` GPU & Multi-Rank Integration Tests)
- `cudnn/src/cudnn.rs`: `t2_conv_and_relu_forward` (convolution + ReLU activation forward check)
- `cudnn/src/ops.rs`: `t2_pooling_softmax_batchnorm_forward` (pooling, softmax, batchnorm inference forward check)
- `nvcomp/src/nvcomp.rs`: `t2_all_formats_roundtrip_bit_exact` (LZ4, Snappy, Zstd, Cascaded, Gdeflate, Bitcomp compress+decompress bit-exactness)
- `nccl/src/nccl.rs`: `t2_single_rank_collectives` (single-rank `init_all` + `all_reduce` / `broadcast`), `t3_multi_rank_all_reduce` (2-GPU `init_all` + fused `group` `all_reduce` / `broadcast` / `send`+`recv`)
- `cufile/src/cufile.rs`: `t2_cufile_register_rw_roundtrip` (`PosixFallback` device sync/async roundtrip), `t2_gds_session_register_roundtrip` (`DriverSession` + `RegisteredBuffer` + `RegisteredStream` + `IoMode::GdsOnly` roundtrip)

---

## 4. Pre-Completion Self-Audit (7 Inquisitor Questions)

1. **Unfinished stubs, `TODO`/`FIXME`, or skipped tests?**
   - **Pass**: Static scan across all 15 `.rs` source files found zero `TODO`, `FIXME`, `todo!()`, `unimplemented!()`, or placeholder implementations. All GPU tests are explicitly marked `#[ignore = "requires ..."]`.
2. **Dead / uncalled functions or orphaned imports (`#[cfg(target_os = "linux")]` vs Windows)?**
   - **Pass after fix**: Caught and gated `cufile::record_teardown_driver_err` with `#[cfg(target_os = "linux")]` so Windows builds incur zero `dead_code` warnings; verified every `#[cfg(not(target_os = "linux"))]` branch binds unused parameters cleanly and all module re-exports/imports match across both OS targets.
3. **Swallowed errors, `#[allow(...)]`, silent fallbacks, or panics outside tests?**
   - **Pass after Inquisitor-3 fixes**:
     - `cufile` async `read_async`/`write_async` hold strong `Arc` pins on `FileHandle`, target device buffer (`RegisteredBuffer<B>` / `DeviceBuffer`), and `Stream` inside `AsyncIoSlot` alongside `Event` synchronization, preventing handle/buffer deregistration or drop UAF while in flight; `IoMode::PosixFallback` `write_async` enqueues non-blocking `memcpy_dtoh_async` and executes `posix_write_at` upon completion.
     - `cudnn::Scratch`, `nvcomp::Scratch`, and `nvcomp::BatchStaging` bind to a single `Stream` (`same_stream` check) and record `Event::record(stream)` after kernel launch so cross-stream races and premature buffer drops are prevented.
     - Removed all `let _ =` teardown/bind error swallowing across `cufile`, `cudnn`, and `nccl`, fixed `unused_mut` on Windows in `FileHandle::deregister`, moved `ctx.bind_to_thread()?` before `self.raw.take()` in `Comm::finalize`/`abort`, and added `#[repr(transparent)]` to all 5 status newtypes.
4. **Boundary / integer-overflow / null-pointer / ABI mismatches?**
   - **Pass**: All `usize -> i32`/`i64`/`off_t` conversions use `try_from`, all buffer/offset sums use `checked_add`, `cufile` `ssize_t` decode handles `>= 0`, `-1` (`errno`), and `< -1` (`-CUfileOpError`) safely, and `nccl` `DataType::to_sys` / `RedOp::to_sys` use direct `nccl_sys` constants instead of raw integer casts.
5. **Hot-path memory churn or missed buffer reuse?**
   - **Pass**: `cudnn::Scratch`, `nvcomp::BatchStaging` (`TableSet` + `Scratch` + `Event` fence), `cufile::FileHandle::posix_staging`, and `cufile::AsyncIoSlot` (`Box<AsyncParams>` + `Event` fence + `PinnedHostBuffer`) grow monotonically and perform zero steady-state heap/device allocations.
6. **Lint compliance (`unsafe_op_in_unsafe_fn = "deny"`, `undocumented_unsafe_blocks = "deny"`, `missing_docs = "warn"`, `clippy::all = "deny"`)?**
   - **Static pass**: Static scan verified every `//!` module header, `///` doc on all `pub` items/modules, `// SAFETY:` on every `unsafe` block and `unsafe impl`, and `unsafe { ... }` inside all `unsafe fn` bodies. Pending `@coordinator`'s `cargo clippy --workspace --all-targets -- -D warnings`.
7. **Remaining unverified build/runtime surface?**
   - Since `d09-sdk` cannot execute `cargo`, compile-time verification on Windows + Linux (`cargo check -p cudnn -p nvcomp -p nccl -p cufile --tests`, `cargo clippy --workspace --all-targets -- -D warnings`) and runtime T0 + GPU T2/T3 tests remain for `@coordinator`.
