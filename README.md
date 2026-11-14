# nv-rs

Rust access to the NVIDIA library family — driver libraries (NVML, OptiX, video), CUDA Toolkit
libraries (cuBLAS, cuFFT, cuSPARSE, cuSOLVER, nvJPEG, NVRTC, ...), separately shipped SDKs — plus
OptiX device code written in Rust (via [cuda-oxide](https://github.com/NVIDIA/cuda-rust)) and
CUDA <-> wgpu interop. Built on the `cuda-rust` host crates (`cuda-bindings`, `cuda-core`).

Independent project; not affiliated with or endorsed by NVIDIA.

Status: early. The plan is in [PLAN.md](PLAN.md), recovered results in [HANDOFF.md](HANDOFF.md),
the active queue in [TODO.md](TODO.md), and binding coverage in [COVERAGE.md](COVERAGE.md).

Current M2 source adds `nvcomp-sys`, `nccl-sys` and `cufile-sys`, mixed/checked-return
generation and a target-aware vcv-rs adapter. Fresh independent review, Windows/Linux checks
and runtime/GPU gates for these additions remain pending. Earlier milestone results are not new checks of this source.

## Layout

| Crate | What it is |
|---|---|
| `nv-loader` | The one runtime loader: finds NVIDIA libraries in System32, the display driver's DriverStore folder, the CUDA Toolkit or an SDK root, returns typed errors. `exports_sys!` is the shared body of every exports-shaped sys crate. |
| `nv-bindgen` | Build-time generator: one declarative `LibSpec` per library; headers located by [vcv-rs](https://github.com/ssoj13/vcv-rs); emits types, native status/pointer shims, checked Rust calls where needed, and a symbol inventory. |
| `nv-bindgen-fixture` | Compiles generator output for a synthetic header and checks the shims' failure paths at runtime. |
| `*-sys` | Raw bindings per library: `nvml-sys`, `optix-sys`, `cufft-sys`, `cublas-sys`, `cublaslt-sys`, `cusparse-sys`, `cusolver-sys`, `nvjpeg-sys`, `nvrtc-sys`, `cupti-sys`, `nvcuvid-sys` (NVDEC), `nvenc-sys`, `npp-sys` (eleven DLLs, `split_exports_sys!`), `cusolvermg-sys`, `cudnn-sys`, `nvapi-sys` (by id through `nvapi_QueryInterface`, `query_interface_sys!`), `nvofa-sys` (Optical Flow), `ngx-sys` (DLSS; statically linked, PLAN D15). |
| `cuda-ext` | CUDA driver features cuda-core lacks: external memory and semaphores, arrays, texture and surface objects, host registration, managed memory, device UUID/LUID, green contexts. |
| `optix` | Safe host API for all of OptiX 9.1: context (properties, log, cache), modules (tasks, payload types, bound values, built-in IS), pipelines (stack sizes, symbols), typed SBT, every acceleration-structure input incl. curves, spheres, motion, opacity micromaps and cluster AS, transforms, relocation, every denoiser model, coop-vec matrices. |
| `device/optix-device` | OptiX device API for Rust programs (cuda-oxide): every `_optix_*` intrinsic generated from the SDK headers plus a typed layer (payloads, trace, SER, geometry, transforms, coop vectors, callables). `cargo xtask coverage optix` proves 248/248 device functions covered. |
| `device/cuda-tex-device` | CUDA texture, sparse-texture and surface reads/writes for cuda-oxide device code. |
| `xtask` | `cargo xtask coverage collect|generate|check`. |

Spikes that de-risked the plan live in `spikes/` (OptiX running a Rust raygen, a Rust raygen
calling `optixTrace`, shared Vulkan/CUDA memory with a timeline semaphore, the first generator
prototype).

## How a sys crate works

`build.rs` is one call:

```rust
use nv_bindgen::{LibSpec, ReturnPolicy, Sdk, Shape};

fn main() -> Result<(), nv_bindgen::Error> {
    nv_bindgen::generate(&LibSpec {
        name: "cufft",
        headers: &["cufft.h"],
        sdks: &[Sdk::CudaToolkit],
        allow_fn: &["^cufft.*"],
        allow_type: &["^cufft.*"],
        allow_var: &["^CUFFT_.*"],
        reuse_cuda_types: &["CUstream_st"],
        positive_enums: &["cufftResult_t"],
        clang_args: &[],
        opaque_types: &[],
        shape: Shape::Exports {
            api_type: "CufftApi",
            loader_fn: "cufft_api",
            returns: &[ReturnPolicy {
                types: &["cufftResult", "cufftResult_t"],
                not_loaded: "cufftResult_t_CUFFT_SETUP_FAILED",
                missing_symbol: "cufftResult_t_CUFFT_NOT_SUPPORTED",
            }],
        },
    })?;
    Ok(())
}
```

and the crate source uses `nv_loader::exports_sys! { .. }` to name where the library ships.
Configured return policies and pointer returns retain native C ABI failure behavior. A scalar
or void call without a legitimate native failure sentinel keeps its original name with a Rust
ABI and returns `Result<T, nv_loader::CallError>` (`Result<()>` for void). Missing libraries and
symbols remain explicit errors; generated native calls never use bindgen's panic-on-missing-symbol
accessors. `Inventory::unshimmed` means no native C ABI shim exists, not that the checked Rust
wrapper is absent. Headers are read from installed SDKs and are never committed.

The vcv-rs adapter takes one SDK snapshot for matching headers, target, runtime and link paths.
It validates the Cargo target architecture and path representation, including explicit
`bindings()` include/system paths before libclang. Discovery-owner root aliases are Cargo
rebuild inputs. Runtime-only sonames do not establish static/link artifacts. This integration
still needs the pushed vcv-rs revision and its execution receipts.

## Building

Follow the current [delivery queue](TODO.md): coherent source is committed/pushed to main before
compiler or test commands. Commands below document the gates, not a new pass receipt.

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
