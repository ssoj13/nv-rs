---
title: Modules Reference
description: Complete API reference for all nv_rs Python submodules, classes, methods, parameters, default values, and return types.
---

This reference documents every submodule, class, method, property, parameter, default value, and return type exposed by `nv_rs` (matching `python/nv-py/python/nv_rs/__init__.pyi`).

Top-level module attributes and convenience functions:

- `__version__: str`
- `trace(level: str = "trace", print_on_exit: bool = True) -> diag.TraceGuard`
- `format_doctor() -> str`

---

## Core & Diagnostics

### `nv_rs.errors`

Exception hierarchy for `nv_rs` operations.

- `NvError(Exception)` — base exception class.
- `NvLibraryNotFoundError(NvError)` — raised when a required NVIDIA shared library cannot be located or loaded.
- `NvCudaError(NvError)` — raised on CUDA driver or runtime API errors.
- `NvInvalidArgumentError(NvError)` — raised when invalid arguments or shapes are passed.
- `NvCompileError(NvError)` — raised when NVRTC or shader/module compilation fails.

### `nv_rs.diag`

Diagnostics, tracing, and library probing utilities.

#### `diag.TraceGuard`

Context manager returned by `diag.trace(...)` and `nv_rs.trace(...)`.

- `__init__(self, level: str = "trace", print_on_exit: bool = False) -> None`
- `__enter__(self) -> diag.TraceGuard`
- `__exit__(self, exc_type: Any = None, exc_val: Any = None, exc_tb: Any = None) -> bool`
- `@property events(self) -> list[dict[str, Any]]`
- `@property live_bytes_delta(self) -> int`
- `@property error_delta(self) -> int`

#### Static Methods

- `get_level() -> str`
- `set_level(level: Literal["off", "error", "warn", "info", "trace"]) -> str`
- `stats() -> dict[str, Any]`
- `events() -> list[dict[str, Any]]`
- `clear_events() -> None`
- `probe_libraries() -> dict[str, dict[str, Any]]`
- `doctor() -> dict[str, Any]`
- `trace(level: str = "trace", print_on_exit: bool = True) -> TraceGuard`
- `format_doctor() -> str`

### `nv_rs.core`

Core CUDA context, stream, and device buffer primitives.

#### `core.Context`

- `__init__(self, device: int = 0) -> None`
- `new_stream(self) -> core.Stream`
- `synchronize(self) -> None`
- `@property device_id(self) -> int`

#### `core.Stream`

- `__init__(self, device: int = 0) -> None`
- `synchronize(self) -> None`
- `@property raw_ptr(self) -> int`
- `@property device_id(self) -> int`

#### `core.DeviceBuffer`

- `@staticmethod alloc(stream: core.Stream, shape: list[int], dtype: str = "u8") -> core.DeviceBuffer`
- `@staticmethod from_buffer(stream: core.Stream, data: Any, shape: list[int] | None = None, dtype: str | None = None) -> core.DeviceBuffer`
- `@staticmethod from_dlpack(obj: Any, stream: core.Stream | None = None) -> core.DeviceBuffer`
- `__dlpack__(self, stream: int | None = None) -> Any`
- `__dlpack_device__(self) -> tuple[int, int]`
- `@property __cuda_array_interface__(self) -> dict[str, Any]`
- `to_bytes(self, stream: core.Stream) -> bytes`
- `@property ptr(self) -> int`
- `@property nbytes(self) -> int`
- `@property shape(self) -> list[int]`
- `@property dtype(self) -> str`
- `@property device_id(self) -> int`

---

## System, Compilation, Compression, Imaging

### `nv_rs.nvml`

NVIDIA Management Library (NVML) bindings for GPU telemetry and device enumeration.

#### `nvml.NvmlDevice`

- `index(self) -> int`
- `name(self) -> str`
- `uuid(self) -> str`
- `compute_capability(self) -> tuple[int, int]`
- `temperature_c(self) -> int`
- `power_usage_mw(self) -> int`
- `power_limit_mw(self) -> int`
- `memory_info(self) -> dict[str, int]`
- `utilization(self) -> dict[str, int]`
- `clocks_mhz(self) -> dict[str, int | None]`
- `summary(self) -> dict[str, Any]`

#### `nvml.Nvml`

- `__init__(self) -> None`
- `driver_version(self) -> str`
- `cuda_driver_version(self) -> tuple[int, int]`
- `device_count(self) -> int`
- `device(self, index: int) -> nvml.NvmlDevice`
- `device_by_uuid(self, uuid: str) -> nvml.NvmlDevice`

### `nv_rs.nvrtc`

Runtime CUDA C++ compilation via NVRTC.

#### `nvrtc.CompileResult`

- `@property ptx(self) -> str`
- `@property ptx_bytes(self) -> bytes`
- `@property cubin(self) -> bytes | None`
- `@property ltoir(self) -> bytes | None`
- `@property optixir(self) -> bytes | None`
- `@property log(self) -> str`
- `lowered_name(self, expr: str) -> str`

#### Static Methods

- `version() -> tuple[int, int]`
- `supported_archs() -> list[int]`
- `compile(source: str, name: str | None = None, arch: str | None = None, std: str | None = None, defines: dict[str, str | None] | None = None, include_dirs: list[str] | None = None, headers: dict[str, str] | None = None, name_expressions: list[str] | None = None, fast_math: bool = False, line_info: bool = False, device_debug: bool = False, rdc: bool = False, dlto: bool = False, extra_args: list[str] | None = None) -> CompileResult`

### `nv_rs.nvcomp`

GPU-accelerated lossless compression via nvCOMP.

#### `nvcomp.CompressedBuffer`

- `@property compressed_bytes(self) -> int`
- `@property allocated_bytes(self) -> int`
- `@property uncompressed_bytes(self) -> int`
- `@property ratio(self) -> float`
- `@property format(self) -> str`
- `@property device_id(self) -> int`

#### `nvcomp.CompressionManager`

- `__init__(self, stream: core.Stream, format: str = "lz4", chunk_size: int = 65536, budget: int | None = None) -> None`
- `compress(self, input: core.DeviceBuffer, stream: core.Stream | None = None) -> nvcomp.CompressedBuffer`
- `decompress(self, compressed: nvcomp.CompressedBuffer, stream: core.Stream | None = None) -> core.DeviceBuffer`

### `nv_rs.nvjpeg`

GPU-accelerated JPEG decoding and encoding via nvJPEG.

#### `nvjpeg.NvjpegDecoder`

- `__init__(self, stream: core.Stream, backend: str = "default") -> None`
- `decode_info(self, data: Any) -> tuple[int, int, int, str]`
- `decode(self, data: Any, stream: core.Stream | None = None, output_format: str = "rgbi") -> core.DeviceBuffer`

#### `nvjpeg.NvjpegEncoder`

- `__init__(self, stream: core.Stream, quality: int = 90, subsampling: str | None = None) -> None`
- `encode(self, image: Any, width: int | None = None, height: int | None = None, quality: int | None = None, subsampling: str | None = None, input_format: str = "rgbi", stream: core.Stream | None = None) -> bytes`

---

## Video & Optical Flow

### `nv_rs.nvenc`

Hardware video encoding via NVENC.

#### Static Methods

- `is_codec_supported(stream: core.Stream, codec: str = "h264") -> bool`
- `codec_caps(stream: core.Stream, codec: str) -> dict[str, Any]`

#### `nvenc.NvencEncoder`

- `__init__(self, stream: core.Stream, width: int, height: int, codec: str = "h264", preset: str = "p4", tuning: str = "high_quality", fps_num: int = 30, fps_den: int = 1, format: str = "nv12", gop_length: int | None = None, bitrate: int | None = None) -> None`
- `encode(self, frame: core.DeviceBuffer, force_idr: bool = False, output_sps_pps: bool = False) -> bytes`
- `flush(self) -> list[bytes]`
- `close(self) -> None`
- `__enter__(self) -> nvenc.NvencEncoder`
- `__exit__(self, exc_type: Any = None, exc_val: Any = None, exc_tb: Any = None) -> None`
- `@property width(self) -> int`
- `@property height(self) -> int`
- `@property codec(self) -> str`
- `@property format(self) -> str`
- `@property is_closed(self) -> bool`

### `nv_rs.nvdec`

Hardware video decoding via NVDEC.

#### Static Methods

- `decoder_caps(stream: core.Stream, codec: str = "h264", chroma: str = "420", bit_depth: int = 8) -> dict[str, Any]`

#### `nvdec.DecodedFrame`

- `@property buffer(self) -> core.DeviceBuffer`
- `@property width(self) -> int`
- `@property height(self) -> int`
- `@property timestamp(self) -> int`
- `@property progressive(self) -> bool`
- `@property top_field_first(self) -> bool`
- `@property format(self) -> str`

#### `nvdec.NvdecDecoder`

- `__init__(self, stream: core.Stream, codec: str = "h264", output_format: str | None = None, max_display_delay: int = 0) -> None`
- `decode(self, data: bytes, timestamp: int | None = None) -> list[nvdec.DecodedFrame]`
- `flush(self) -> list[nvdec.DecodedFrame]`
- `close(self) -> None`
- `__enter__(self) -> nvdec.NvdecDecoder`
- `__exit__(self, exc_type: Any = None, exc_val: Any = None, exc_tb: Any = None) -> None`
- `@property codec(self) -> str`
- `@property width(self) -> int`
- `@property height(self) -> int`
- `@property is_closed(self) -> bool`

### `nv_rs.nvofa`

Hardware optical flow estimation via NVOFA.

#### `nvofa.Nvofa`

- `__init__(self, stream: core.Stream, width: int, height: int, grid_size: int = 4, input_format: str = "grayscale8", perf_level: str = "slow") -> None`
- `execute(self, prev_buf: core.DeviceBuffer, curr_buf: core.DeviceBuffer, disable_temporal_hints: bool = False) -> core.DeviceBuffer`
- `close(self) -> None`
- `__enter__(self) -> nvofa.Nvofa`
- `__exit__(self, exc_type: Any = None, exc_val: Any = None, exc_tb: Any = None) -> None`
- `@property width(self) -> int`
- `@property height(self) -> int`
- `@property grid_size(self) -> int`
- `@property grid_dims(self) -> tuple[int, int]`
- `@property input_format(self) -> str`
- `@property is_closed(self) -> bool`

---

## Dense/Sparse Math, FFT, RNG, AI

### `nv_rs.cublas`

Dense linear algebra via cuBLAS.

#### `cublas.CublasHandle`

- `__init__(self, stream: core.Stream) -> None`
- `set_stream(self, stream: core.Stream) -> None`
- `gemm_f32(self, a: core.DeviceBuffer, b: core.DeviceBuffer, c: core.DeviceBuffer, m: int, n: int, k: int, alpha: float = 1.0, beta: float = 0.0, trans_a: bool = False, trans_b: bool = False) -> None`

### `nv_rs.cufft`

Fast Fourier Transforms via cuFFT.

#### `cufft.CufftPlan1d`

- `__init__(self, stream: core.Stream, nx: int, kind: str = "c2c", batch: int = 1) -> None`
- `execute(self, in_buf: core.DeviceBuffer, out_buf: core.DeviceBuffer, direction: str = "forward") -> None`

### `nv_rs.curand`

GPU random number generation via cuRAND.

#### `curand.CurandGenerator`

- `__init__(self, stream: core.Stream, seed: int | None = None) -> None`
- `fill_uniform_f32(self, buf: core.DeviceBuffer) -> None`
- `fill_normal_f32(self, buf: core.DeviceBuffer, mean: float = 0.0, stddev: float = 1.0) -> None`

### `nv_rs.cusparse`

Sparse linear algebra via cuSPARSE.

#### Static Methods

- `version() -> int`

#### `cusparse.CusparseHandle`

- `__init__(self, stream: core.Stream) -> None`
- `set_stream(self, stream: core.Stream) -> None`
- `version(self) -> int`
- `spmv_f32(self, row_offsets: core.DeviceBuffer, col_ind: core.DeviceBuffer, values: core.DeviceBuffer, x: core.DeviceBuffer, y: core.DeviceBuffer, rows: int, cols: int, nnz: int, alpha: float = 1.0, beta: float = 0.0, transpose: bool = False) -> None`

### `nv_rs.cusolver`

Dense matrix factorizations and linear solvers via cuSOLVER.

#### `cusolver.CusolverDnHandle`

- `__init__(self, stream: core.Stream) -> None`
- `set_stream(self, stream: core.Stream) -> None`
- `getrf_f32(self, a: core.DeviceBuffer, ipiv: core.DeviceBuffer, dev_info: core.DeviceBuffer, m: int, n: int, lda: int | None = None) -> None`
- `getrs_f32(self, a: core.DeviceBuffer, ipiv: core.DeviceBuffer, b: core.DeviceBuffer, dev_info: core.DeviceBuffer, n: int, nrhs: int = 1, lda: int | None = None, ldb: int | None = None, trans: bool = False) -> None`
- `potrf_f32(self, a: core.DeviceBuffer, dev_info: core.DeviceBuffer, n: int, lda: int | None = None, upper: bool = False) -> None`
- `potrs_f32(self, a: core.DeviceBuffer, b: core.DeviceBuffer, dev_info: core.DeviceBuffer, n: int, nrhs: int = 1, lda: int | None = None, ldb: int | None = None, upper: bool = False) -> None`

### `nv_rs.cudnn`

Deep neural network primitives via cuDNN.

#### Static Methods

- `version() -> int`

#### `cudnn.CudnnHandle`

- `__init__(self, stream: core.Stream) -> None`
- `set_stream(self, stream: core.Stream) -> None`
- `activation_forward_f32(self, x: core.DeviceBuffer, y: core.DeviceBuffer, shape: list[int], mode: str = "relu", alpha: float = 1.0, beta: float = 0.0, coef: float = 0.0) -> None`
- `softmax_forward_f32(self, x: core.DeviceBuffer, y: core.DeviceBuffer, shape: list[int], algo: str = "accurate", mode: str = "channel", alpha: float = 1.0, beta: float = 0.0) -> None`

---

## Graphics, Ray Tracing, NPP, DLSS

### `nv_rs.optix`

Ray tracing context and module management via OptiX.

#### Static Methods

- `abi_version() -> int`

#### `optix.OptixModule`

- `@property log(self) -> str`

#### `optix.OptixContext`

- `__init__(self, ctx: core.Context, log_level: int = 0, validation: bool = False) -> None`
- `@property abi_version(self) -> int`
- `rtcore_version(self) -> int`
- `max_trace_depth(self) -> int`
- `device_properties(self) -> dict[str, Any]`
- `create_module(self, code: bytes, params_name: str = "params", params_size: int = 0, payload_values: int = 2, opt_level: str = "default") -> optix.OptixModule`

### `nv_rs.npp`

GPU image and signal processing primitives via NVIDIA Performance Primitives (NPP).

#### `npp.NppStreamContext`

- `__init__(self, stream: core.Stream) -> None`
- `@property device_id(self) -> int`

#### Static Methods

- `version() -> tuple[int, int, int]`
- `rgb_to_gray_8u(ctx: NppStreamContext, src: core.DeviceBuffer, dst: core.DeviceBuffer, width: int, height: int) -> None`
- `rgb_to_ycbcr_8u(ctx: NppStreamContext, src: core.DeviceBuffer, dst: core.DeviceBuffer, width: int, height: int) -> None`
- `ycbcr_to_rgb_8u(ctx: NppStreamContext, src: core.DeviceBuffer, dst: core.DeviceBuffer, width: int, height: int) -> None`
- `resize_8u_c1(ctx: NppStreamContext, src: core.DeviceBuffer, src_size: tuple[int, int], dst: core.DeviceBuffer, dst_size: tuple[int, int], interp: str = "linear") -> None`
- `resize_8u_c3(ctx: NppStreamContext, src: core.DeviceBuffer, src_size: tuple[int, int], dst: core.DeviceBuffer, dst_size: tuple[int, int], interp: str = "linear") -> None`
- `resize_8u_c4(ctx: NppStreamContext, src: core.DeviceBuffer, src_size: tuple[int, int], dst: core.DeviceBuffer, dst_size: tuple[int, int], interp: str = "linear") -> None`
- `resize_32f_c3(ctx: NppStreamContext, src: core.DeviceBuffer, src_size: tuple[int, int], dst: core.DeviceBuffer, dst_size: tuple[int, int], interp: str = "linear") -> None`
- `filter_gauss_8u(ctx: NppStreamContext, src: core.DeviceBuffer, dst: core.DeviceBuffer, width: int, height: int, channels: int = 1) -> None`
- `threshold_val_8u_c1(ctx: NppStreamContext, src: core.DeviceBuffer, dst: core.DeviceBuffer, size: tuple[int, int], threshold: int, value: int) -> None`
- `threshold_val_32f_c1(ctx: NppStreamContext, src: core.DeviceBuffer, dst: core.DeviceBuffer, size: tuple[int, int], threshold: float, value: float) -> None`
- `set_32f(ctx: NppStreamContext, value: float, dst: core.DeviceBuffer) -> None`
- `add_32f(ctx: NppStreamContext, src1: core.DeviceBuffer, src2: core.DeviceBuffer, dst: core.DeviceBuffer) -> None`
- `mul_32f(ctx: NppStreamContext, src1: core.DeviceBuffer, src2: core.DeviceBuffer, dst: core.DeviceBuffer) -> None`
- `sum_32f_scratch_bytes(ctx: NppStreamContext, length: int) -> int`
- `sum_32f(ctx: NppStreamContext, src: core.DeviceBuffer, sum_out: core.DeviceBuffer, scratch: core.DeviceBuffer) -> None`

### `nv_rs.ngx`

NVIDIA NGX / DLSS capability queries and project validation.

#### `ngx.NgxCapabilities`

- `__init__(self) -> None`
- `@property supported(self) -> bool`
- `@property instance_extensions(self) -> list[str]`
- `@property device_extensions(self) -> list[str]`
- `@property status_message(self) -> str | None`
- `is_dlss_supported(self) -> bool`
- `required_extensions(self) -> tuple[list[str], list[str]]`
- `supported_features(self) -> list[tuple[str, int]]`
- `dlss_quality_presets(self) -> list[tuple[str, int]]`

#### Static Methods

- `is_dlss_supported() -> bool`
- `required_extensions() -> dict[str, Any]`
- `feature_requirements(feature: str = "super_sampling") -> dict[str, Any]`
- `validate_project_id(guid: str, engine_name: str | None = None, engine_version: str | None = None) -> dict[str, Any]`

---

## Profiling, Driver Extensions, WGPU Interop

### `nv_rs.cupti`

CUDA Profiling Tools Interface (CUPTI) activity and callback tracing.

#### Static Methods

- `version() -> int`
- `callback_name(domain: str, cbid: int) -> str`

#### `cupti.CuptiActivity`

- `__init__(self, context: core.Context | None = None, device: int = 0) -> None`
- `enable(self, kind: str) -> None`
- `disable(self, kind: str) -> None`
- `flush(self, forced: bool = True) -> None`
- `drain_records(self) -> tuple[list[dict[str, Any]], int]`
- `close(self) -> None`

#### `cupti.CuptiSubscriber`

- `__init__(self, context: core.Context | None = None, device: int = 0, callback: Any = None) -> None`
- `enable_domain(self, domain: str, enable: bool) -> None`
- `enable_callback(self, domain: str, cbid: int, enable: bool) -> None`
- `drain_events(self) -> list[tuple[str, int]]`
- `check_errors(self) -> None`
- `close(self) -> None`

### `nv_rs.cuda_ext`

Extended CUDA driver features: device LUID/UUID, sparse arrays, green contexts, managed memory, host registration, and external memory descriptors.

#### Static Methods

- `device_luid(device: int = 0) -> tuple[bytes, int]`
- `device_uuid(device: int = 0) -> bytes`
- `sparse_arrays_supported(device: int = 0) -> bool`
- `split_sms(min_sms: int, context: core.Context | None = None, device: int = 0) -> list[cuda_ext.GreenContext]`

#### `cuda_ext.ManagedBuffer`

- `@staticmethod zeroed(nbytes: int, context: core.Context | None = None, device: int = 0) -> cuda_ext.ManagedBuffer`
- `@property ptr(self) -> int`
- `@property nbytes(self) -> int`
- `to_bytes(self) -> bytes`
- `write_bytes(self, data: bytes, offset: int = 0) -> None`

#### `cuda_ext.HostRegistration`

- `__init__(self, data: bytes, map_to_device: bool = True, context: core.Context | None = None, device: int = 0) -> None`
- `@property device_ptr(self) -> int | None`
- `@property nbytes(self) -> int`
- `to_bytes(self) -> bytes`
- `close(self) -> None`

#### `cuda_ext.GreenContext`

- `@property sm_count(self) -> int`
- `@property device_id(self) -> int`
- `new_stream(self) -> core.Stream`

#### `cuda_ext.ExternalMemoryDesc`

- `__init__(self, handle_kind: str, size: int, dedicated: bool = False) -> None`
- `@property handle_kind(self) -> str`
- `@property size(self) -> int`
- `@property dedicated(self) -> bool`

### `nv_rs.wgpu`

WebGPU (`wgpu`) / CUDA interoperability for shared buffers and timeline semaphores.

#### Static Methods

- `is_supported() -> bool`
- `probe_adapters() -> list[dict[str, Any]]`
- `aligned_row_pitch(format: str, width: int) -> int`
- `aligned_staging_bytes(format: str, width: int, height: int, depth: int = 1) -> int`

#### `wgpu.SharedBuffer`

- `@property ptr(self) -> int`
- `@property size(self) -> int`

#### `wgpu.InteropSemaphore`

- `signal_from_cuda(self, stream: core.Stream, value: int) -> None`
- `wait_in_cuda(self, stream: core.Stream, value: int) -> None`
- `completed_value(self) -> int`
- `signal_from_host(self, value: int) -> None`
- `wait_on_host(self, value: int, timeout_ns: int = 1_000_000_000) -> None`

#### `wgpu.InteropContext`

- `__init__(self, backend: str = "auto", label: str | None = None) -> None`
- `@property backend(self) -> str`
- `@property cuda_device_id(self) -> int`
- `verify(self) -> str`
- `create_buffer(self, size: int, usage: str = "storage|copy_src|copy_dst", label: str | None = None) -> wgpu.SharedBuffer`
- `create_semaphore(self, initial_value: int = 0) -> wgpu.InteropSemaphore`

---

## Platform-Specific

### `nv_rs.nccl`

Multi-GPU collective communications via NCCL.

- `SUPPORTED: bool` — `True` on Linux when NCCL is available; `False` on unsupported platforms (such as Windows).

#### Static Methods

- `version() -> int`
- `unique_id() -> bytes`
- `group_start() -> None`
- `group_end() -> None`

#### `nccl.NcclComm`

- `@staticmethod init_all(contexts: list[core.Context]) -> list[nccl.NcclComm]`
- `@staticmethod init_rank(context: core.Context, nranks: int, unique_id: bytes, rank: int) -> nccl.NcclComm`
- `@property count(self) -> int`
- `@property rank(self) -> int`
- `@property device_id(self) -> int`
- `async_error(self) -> str`
- `last_error(self) -> str | None`
- `finalize(self) -> None`
- `abort(self) -> None`
- `all_reduce(self, send: core.DeviceBuffer, recv: core.DeviceBuffer, count: int, dtype: str, op: str, stream: core.Stream) -> None`
- `broadcast(self, send: core.DeviceBuffer | None, recv: core.DeviceBuffer, count: int, dtype: str, root: int, stream: core.Stream) -> None`
- `reduce(self, send: core.DeviceBuffer, recv: core.DeviceBuffer | None, count: int, dtype: str, op: str, root: int, stream: core.Stream) -> None`
- `all_gather(self, send: core.DeviceBuffer, recv: core.DeviceBuffer, send_count: int, dtype: str, stream: core.Stream) -> None`
- `reduce_scatter(self, send: core.DeviceBuffer, recv: core.DeviceBuffer, recv_count: int, dtype: str, op: str, stream: core.Stream) -> None`
- `send(self, send: core.DeviceBuffer, count: int, dtype: str, peer: int, stream: core.Stream) -> None`
- `recv(self, recv: core.DeviceBuffer, count: int, dtype: str, peer: int, stream: core.Stream) -> None`

### `nv_rs.cufile`

GPUDirect Storage (GDS) file I/O via cuFile.

- `SUPPORTED: bool` — `True` on Linux when cuFile/GDS is available; `False` on unsupported platforms (such as Windows).

#### Static Methods

- `version() -> int`

#### `cufile.CufileDriver`

- `__init__(self, context: core.Context) -> None`
- `use_count(self) -> int`
- `properties(self) -> dict[str, Any]`
- `close(self) -> None`
- `__enter__(self) -> cufile.CufileDriver`
- `__exit__(self, exc_type: Any = None, exc_val: Any = None, exc_tb: Any = None) -> None`

#### `cufile.CufileHandle`

- `__init__(self, context: core.Context, path: str, mode: str = "gds", create: bool = False, truncate: bool = False, read: bool = True, write: bool = True) -> None`
- `read(self, dst: core.DeviceBuffer, file_offset: int = 0, buf_offset: int = 0, len: int | None = None) -> int`
- `write(self, src: core.DeviceBuffer, file_offset: int = 0, buf_offset: int = 0, len: int | None = None) -> int`
- `close(self) -> None`
- `@property mode(self) -> str`
- `__enter__(self) -> cufile.CufileHandle`
- `__exit__(self, exc_type: Any = None, exc_val: Any = None, exc_tb: Any = None) -> None`

### `nv_rs.nvapi`

Windows-only NVIDIA driver and display APIs via NVAPI.

- `SUPPORTED: bool` — `True` on Windows when NVAPI is available; `False` on non-Windows platforms.

#### `nvapi.NvapiSession`

- `__init__(self) -> None`
- `interface_version(self) -> str`
- `driver_version(self) -> dict[str, Any]`
- `gpu_count(self) -> int`
- `gpu_names(self) -> list[str]`
- `thermal_info(self, index: int = 0) -> list[dict[str, Any]]`
- `gpu_info(self, index: int = 0) -> dict[str, Any]`
- `display_info(self) -> list[dict[str, Any]]`
