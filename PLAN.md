# nv-rs Master Plan & Status

Consolidated from `TODO.md`, `plan1.md`, `BUG_HUNT_REPORT.md`, `FFI_AUDIT.md`, and `INFRA_PYTHON_AUDIT.md`.
Maintainer: Alex Khalyavin `<joss13@gmail.com>`.

---

## Completed Milestones (Verified & Pushed to `main`)

### M1 — `nv-demo` Stability & Error Handling (`03021de`, `033f1e1`)
- [x] Fixed `DriverError(700)` (`optixGetTriangleVertexData` on GAS without `OPTIX_BUILD_FLAG_ALLOW_RANDOM_VERTEX_ACCESS`) by setting `random_vertex_access: true` on triangle GAS in `crates/nv-demo/src/renderer.rs`.
- [x] Verified `retired_buffers` resize lifetime and replaced OOM resize / `save_front_png` `unwrap()` panics with typed error propagation in `crates/nv-demo/src/main.rs`.

### M2 — Safe Wrapper Coverage & CI (`033f1e1`)
- [x] Wrapped all 48 public `cublasLt*` entry points in `crates/cublas/src/lt.rs` and `crates/cublas/src/error.rs` (plus 6 internal C stack-init helpers documented).
- [x] Wrapped all 19/19 `cusolverMg*` entry points in `crates/cusolver/src/mg.rs` and `crates/cusolver/src/cusolver.rs`.
- [x] Added `wrapper_src_dir` mapping (`cublaslt -> cublas`, `cusolvermg -> cusolver`, `nvcuvid -> nvdec`) in `xtask/src/xtask.rs`, updated `coverage/*.toml` and `COVERAGE.md`, and added `-p xtask` to `.github/workflows/ci.yml`.

### M3 — FFI, Python/DLPack & Infrastructure Hardening (`ae7c323`)
- [x] **FFI-1** (`crates/cufft/src/{plan.rs,scratch.rs}`): Retained `Option<Arc<ScratchInner>>` in `Plan` with generation/pointer tracking and stream validation, preventing dangling `cufftSetWorkArea` pointers.
- [x] **FFI-2** (`crates/cufft/src/plan.rs`): Normalized `ManyDesc::min_io_elems` for `None`-embed batching per cuFFT semantics + unit tests.
- [x] **FFI-3** (`crates/cudnn/src/cudnn.rs`, `crates/nccl/src/nccl.rs`, `crates/cufile/src/cufile.rs`): Sealed `DeviceSpan` / `DeviceSpanMut` via private `mod sealed` with `compile_fail` doctests.
- [x] **FFI-4** (`crates/cufft/src/plan.rs`, `crates/nv-demo/src/fft_bloom.rs`, `crates/cufft/src/cufft.rs`): Updated `exec_c2r` / `exec_z2d` to require `&mut DeviceBuffer` for destructive cuFFT inputs and rebound `FftBloomPass::scratch` to the render stream.
- [x] **PY-1..PY-4** (`python/nv-py/src/{core.rs,dlpack.rs,compute.rs,gfx_img.rs,sparse_ai.rs}`, `python/nv-py/tests/test_dlpack_safety.py`): Added `DlManagedGuard` (zero UAF on rejection), centralized `validate_shape_and_bytes`, exported byte `strides` and `"stream"` in `__cuda_array_interface__`, enforced `is_c_contiguous()` in `to_bytes`, honored `stream` (including `-1` and `PyStream`) in `__dlpack__`, unified `DTYPE_TABLE` and `buf_ptr_and_bytes`, and removed dead `CapsuleStatus`.
- [x] **INFRA-1** (`xtask/src/prebuilt.rs`): Isolated foreign-target prebuilt staging from host builds, added `CARGO_TARGET_DIR` resolution and unique test temp dirs.
- [x] **INFRA-2** (`bootstrap.py`): Added two-pass tar fallback extraction preserving symlinks/hardlinks with traversal validation.
- [x] **DLL version sort** (`crates/nv-loader/src/nv_loader.rs`): Switched wildcard DLL ordering to numeric version components (`12 > 11 > 9`).

### M4 — Nightly Verification, Manifest Cleanup & Documentation (`60f9bb4`)
- [x] Cleaned unused `cuda-async` (`Cargo.toml`), `cuda-bindings` (`crates/nvenc-sys/Cargo.toml`), and unused `[workspace.package]` fields; inherited `[lints]` and metadata in `crates/nv-demo/Cargo.toml`; added author/URL metadata in `python/nv-py/pyproject.toml`.
- [x] Fixed `clippy::new_instead_of_clear` in `crates/optix/tests/gpu_features.rs` via `wave.drain(..)`.
- [x] Verified `device/optix-device` and `device/cuda-tex-device` (`cargo +nightly check`), `cargo +nightly clippy --workspace --all-targets -- -D warnings`, `cargo +nightly test --workspace --quiet`, `cargo clippy` (stable), `cargo test --workspace`, `pytest`, WSL2 `Ubuntu` tests, and `bun run build` in `docs/`.
- [x] Updated `README.md`, `CHANGELOG.md`, and `docs/src/content/docs/**` with all changes and author attribution (`Alex Khalyavin <joss13@gmail.com>`).

### M5 — Python (`nv_rs`) SSOT Stub & Docs Automation (`py-stubs`)
- [x] Fixed 6 stub/doc/annotation drift bugs across `python/nv-py/src/*.rs` (`__exit__` `(exc_type, exc_val, exc_tb)` kwargs, `/// @py-return:` structured return schemas for `Py<PyDict>`/`Py<PyAny>`, `#[expect(clippy::too_many_arguments)]` in `nvrtc.rs`, `pyproject.toml` description).
- [x] Consolidated PyO3 boilerplate into `python/nv-py/src/macros.rs` (`py_register_submodule!`, `py_parse_enum!`, `py_err_mapper!`) and `python/nv-py/src/buf_util.rs` (`buf_ptr_and_bytes`, `validate_dev_buf`), keeping all `#[pyclass]`/`#[pymethods]`/`#[pyfunction]` headers explicit.
- [x] Implemented `cargo xtask py-stubs <generate|check>` in `xtask/src/py_stubs.rs` (`syn`-based SSOT parser) to deterministically generate and drift-check `python/nv-py/python/nv_rs/__init__.pyi` and `docs/src/content/docs/python/04-modules-reference.md`.
- [x] Deepened `python/nv-py/tests/test_stub_sync.py` (full `ast` verification of all 25 submodules, classes, methods, functions, return annotations, and optional runtime `inspect` parity) and wired `cargo run -p xtask -- py-stubs check` into `.github/workflows/ci.yml` and `bootstrap.py`.

### M6 — `nv-demo` Pure-CSG Worlds, Articulated Crowds, Dynamic `Motion` AOV, SDG Export & 60 Hz Pacing (`5b4fcf8`)
- [x] **Zero-Churn RTX Pipelining & Per-Engine Telemetry (`2c557a6`, `d3ac3f1`, `8e55b4d`)**: Added persistent `optix::InstanceBuffer` + uncompacted `Accel::instances_into` (`compact: false`, `allow_update: true`), persistent `GrowDeviceBuffer<T>` CSG VRAM pools, single end-of-frame fence, non-stalling 3-slot `cuEventQuery` `GpuPassProfiler` (`RT Cores` / `Tensor Cores` / `CUDA Compute`), live 4-way `Road Mode` UI (`3D | C1 | Hybrid | F-Rep`), `(s, lat)` road micro-shading, `EnginePhase` HUD, and `appbus-rs` decoupled UI with `1 spp` fast-preview.
- [x] **Articulated `csg-actors` Crowds & 110-Variant `Inspector` (`5b4fcf8`, `csg-rs` `7b8a085`)**: Integrated `csg-actors` (23-crate `csg-rs`, 84-slot `MaterialPalette`, 15-class `GasLedger` `246/256`, 11-GAS `SINGLE_PRIM | IDENTITY` SBT 10 bank); `CsgGpuHost` owns `Option<SceneBuilt>` directly, stepping both `TrafficStateSoA`/legacy traffic (`!traffic.is_empty()`) and `SceneBuilt::crowd` across city walkways and `Inspector` variant `109` (Urban Human, `RoadSignStickman`, `QuadrupedDog`, 5-finger `SdgMannequin`).
- [x] **True Object Optical Flow (`608 B` ABI), `Freeze Pose` & Atomic SDG JSON Export (`5b4fcf8`)**: Extended `LaunchParams`/`Params` to `608 B` (`csg_prev_xforms` at offset `592`), uploaded `motion_prev_xforms` via `GrowDeviceBuffer<[f32; 12]>` + dedicated `pinned_prev_staging`, computed exact 3x3 affine-inverse `prev_world_pos` in `raytracer.cu` (`0 double`), added `PEDESTRIAN (18)` / `ANIMAL (19)` semantic AOV colors, wired `Freeze Pose` with zero-motion snap on `false -> true` transition, and added atomic `ExportSdgJson` (`csg_actors::extract_sdg_frame` + `CsgRayOccluder`).
- [x] **60 Hz UI Cap & OptiX `7999` Diagnostics (`5b4fcf8`)**: Switched swapchain to `PresentMode::Fifo` + `ControlFlow::WaitUntil(16.667 ms)` (`compute_next_ui_tick`), fixed `ias_build_opts().compact = false` with unit ratchet test, and installed live `optix::LogFn` callback (`log_level = 3`, explicit `OptixCache` hint on `7999`). Verified clean (`284/284` `nv-demo` tests, `523` workspace tests, `clippy -D warnings` clean).

---

## Backlog (Optional / Future Work, Sorted by Priority)

1. **[P2] Hardware GPU & Sanitizer Sweep**: Run `cargo test --workspace -- --ignored` and Compute Sanitizer (`compute-sanitizer`) on live GPU paths (`cufft` `ManyDesc` / `Scratch` growth, `cudnn`/`cufile`, `optix`).
2. **[P3] Shared Device-Span Trait Extraction**: Consolidate the sealed `DeviceSpan` / `DeviceSpanMut` contracts from `cudnn`, `nccl`, and `cufile` into `cuda-core-ext` once cross-crate mutability semantics are finalized.
3. **[P3] `nv-py` Dtype Inference Reuse**: Reuse `dlpack::find_dtype_entry` inside `python/nv-py/src/core.rs` dtype inference to eliminate the remaining local mapping table.
4. **[P3] Performance Profiling Pass**: Profile `nv-demo` frame-time and wrapper hot paths (`samply` / `tracing-chrome` / `criterion`) with A/B release benchmarks.
