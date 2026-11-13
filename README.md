# nv-rs

Rust access to the NVIDIA library family — driver libraries (NVML, OptiX, video), CUDA Toolkit
libraries (cuBLAS, cuFFT, cuSPARSE, cuSOLVER, nvJPEG, NVRTC, ...), separately shipped SDKs — plus
OptiX device code written in Rust (via [cuda-oxide](https://github.com/NVIDIA/cuda-rust)) and
CUDA <-> wgpu interop. Built on the `cuda-rust` host crates (`cuda-bindings`, `cuda-core`).

Independent project; not affiliated with or endorsed by NVIDIA.

Status: early. The plan is in [PLAN.md](PLAN.md), the current state in [HANDOFF.md](HANDOFF.md),
binding coverage in [COVERAGE.md](COVERAGE.md).

## Layout

| Crate | What it is |
|---|---|
| `nv-loader` | The one runtime loader: finds NVIDIA libraries in System32, the display driver's DriverStore folder, the CUDA Toolkit or an SDK root, returns typed errors. `exports_sys!` is the shared body of every exports-shaped sys crate. |
| `nv-bindgen` | Build-time generator: one declarative `LibSpec` per library; headers located by [vcv-rs](https://github.com/ssoj13/vcv-rs); emits types, panic-free `extern "C"` shims and a symbol inventory. |
| `nv-bindgen-fixture` | Compiles generator output for a synthetic header and checks the shims' failure paths at runtime. |
| `*-sys` | Raw bindings per library: `nvml-sys`, `optix-sys`, `cufft-sys`, `cublas-sys`, `cublaslt-sys`, `cusparse-sys`, `cusolver-sys`, `nvjpeg-sys`, `nvrtc-sys`, `cupti-sys`, `nvcuvid-sys` (NVDEC), `nvenc-sys`, `npp-sys` (eleven DLLs, `split_exports_sys!`), `cusolvermg-sys`, `cudnn-sys`, `nvapi-sys` (by id through `nvapi_QueryInterface`, `query_interface_sys!`), `nvofa-sys` (Optical Flow), `ngx-sys` (DLSS; statically linked, PLAN D15). |
| `cuda-ext` | CUDA driver features cuda-core lacks: external memory and semaphores, arrays, texture and surface objects, host registration, managed memory, device UUID/LUID, green contexts. |
| `optix` | Safe OptiX host API: device context with log sink, acceleration structures (triangle GAS, IAS, compaction, byte budget), modules and program groups, pipelines, typed `SbtRecord<T>` / `ShaderBindingTable`, launch. Device code is Rust compiled by cuda-oxide. |
| `xtask` | `cargo xtask coverage collect|generate|check`. |

Spikes that de-risked the plan live in `spikes/` (OptiX running a Rust raygen, a Rust raygen
calling `optixTrace`, shared Vulkan/CUDA memory with a timeline semaphore, the first generator
prototype).

## How a sys crate works

`build.rs` is one call:

```rust
nv_bindgen::generate(&LibSpec {
    name: "cufft",
    headers: &["cufft.h"],
    sdks: &[Sdk::CudaToolkit],
    allow_fn: &["^cufft.*"], allow_type: &["^cufft.*"], allow_var: &["^CUFFT_.*"],
    reuse_cuda_types: &["CUstream_st"],          // cudaStream_t == cuda_bindings::CUstream
    positive_enums: &["cufftResult_t"],
    shape: Shape::Exports { api_type: "CufftApi", loader_fn: "cufft_api",
        status_types: &["cufftResult"],
        not_loaded: "cufftResult_t_CUFFT_SETUP_FAILED",
        missing_symbol: "cufftResult_t_CUFFT_NOT_SUPPORTED" },
})
```

and `lib.rs` is one `nv_loader::exports_sys! { .. }` invocation naming where the DLL ships. Each
generated function loads the library on first call and returns the library's own error code when
the library or the symbol is missing; nothing panics across the C boundary. Headers are read from
the SDKs installed on the build machine and are never committed.

## Building

Needs the CUDA Toolkit (`CUDA_PATH` or the default install). Every other SDK is located by
[vcv-rs](https://github.com/ssoj13/vcv-rs):

- its root variable (`OPTIX_ROOT`, `CUDNN_ROOT`, `VIDEO_CODEC_SDK_ROOT`, `NVAPI_ROOT`, ...);
- the installers' folders `%ProgramFiles%\NVIDIA` and `%ProgramData%\NVIDIA Corporation`;
- any folder listed in `NV_SDK_DIRS`, for SDKs unpacked or cloned elsewhere.

`vcv-rs -l` shows what it finds. `ngx-sys` also needs the Vulkan SDK (`VULKAN_SDK`). On Windows
the MSVC include paths come from vcv-rs too, so any shell works.

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

```bash
cargo test --workspace
```

GPU tests are `#[ignore]`d; run them with `-- --include-ignored` on a machine with an NVIDIA GPU.

## Conventions

Apache-2.0, DCO sign-off on every commit (`git commit -s`), host toolchain pinned to the
`cuda-rust` host crates' version.
