# nv-rs

An armed engineer checks his tooling before he draws.

`nv-rs` is a multi-crate Rust workspace giving direct, honest access to the NVIDIA library
stack — CUDA Toolkit libraries, OptiX, display/management APIs, video, compression, DLSS/NGX,
CUDA <-> wgpu interop, and Rust device code via `cuda-oxide` — built on `cuda-bindings` and
`cuda-core`.

Independent project; not affiliated with or endorsed by NVIDIA.

TANSTAAFL: NVIDIA headers and proprietary redistributables do not live in this git tree.
Every `*-sys` crate reads headers from your installed SDKs at build time via
[vcv-rs](https://github.com/ssoj13/vcv-rs) and loads runtime libraries through one shared
loader (`nv-loader`) with explicit typed errors. If a DLL is missing, you get a clean `Err`,
never a surprise panic.

For architecture decisions, current state, and coverage numbers, read the ship's log:
[PLAN.md](PLAN.md), [HANDOFF.md](HANDOFF.md), [TODO.md](TODO.md), and [COVERAGE.md](COVERAGE.md).

---

## What Is in the Hangar

| Layer | Crates |
|---|---|
| **Infrastructure** | `nv-loader` (shared runtime loader, `exports_sys!` / `split_exports_sys!` / `query_interface_sys!`), `nv-bindgen` (declarative `LibSpec` generator), `nv-bindgen-fixture` |
| **Interop & Extensions** | `cuda-ext` (external memory/semaphores, arrays, textures/surfaces, green contexts), `nv-wgpu` (zero-copy CUDA <-> `wgpu` buffer/texture/semaphore interop) |
| **Linear Algebra & Math** | `cublas` (`cublas-sys`, `cublaslt-sys`), `cufft` (`cufft-sys`), `curand`, `cusolver` (`cusolver-sys`, `cusolvermg-sys`), `cusparse` (`cusparse-sys`) |
| **Imaging, Video & Graphics** | `npp` (`npp-sys`), `nvjpeg` (`nvjpeg-sys`), `nvdec` (`nvcuvid-sys`), `nvenc` (`nvenc-sys`), `nvofa` (`nvofa-sys`), `optix` (`optix-sys`), `ngx` (`ngx-sys` — statically linked per PLAN D15; DLSS-SR/RR safe wrapper via `Init_with_ProjectID` per D13) |
| **AI, Storage, Comms & Mgmt** | `cudnn` (`cudnn-sys`), `nvcomp` (`nvcomp-sys`), `cufile` (`cufile-sys`, Linux GPUDirect Storage), `nccl` (`nccl-sys`, Linux), `nvrtc` (`nvrtc-sys`), `cupti` (`cupti-sys`), `nvml` (`nvml-sys`), `nvapi` (`nvapi-sys`, Windows) |
| **Device & Tooling** | `device/optix-device`, `device/cuda-tex-device`, `xtask` (`coverage collect\|generate\|check`) |

---

## Provisioning SDKs (`bootstrap.py`)

A man who cannot provision his own bench cannot trust his own builds.
`bootstrap.py` is stdlib-only Python 3; running it bare prints help and exits cleanly (`0`):

```bash
python bootstrap.py          # show all commands and examples
python bootstrap.py d s      # inspect detected SDKs (env vars, .sdk/, NV_SDK_DIRS, system paths)
python bootstrap.py d dl     # download & unpack public SDKs into .sdk/
python bootstrap.py s        # toolchain, git HEAD, and SDK summary
```

Public SDKs (`nvcomp`, `nvapi`, `cudnn`, `ngx`, Linux `vulkan`) pull straight into `.sdk/`
(git-ignored). Auth-gated or installer SDKs (`cuda`, `opticalflow`, `videocodec`, Windows
`vulkan`) report their exact URLs and expected layout. Point `NV_SDK_DIRS` at `.sdk` (or your
own SDK directory) when building crates that use them.

---

## Building & Verification

Never ship an unverified build. Pin the host toolchain to `1.98.0` and treat warnings as
structural failures:

```bash
python bootstrap.py c        # cargo fmt --all --check + clippy --workspace --all-targets -- -D warnings
python bootstrap.py t        # workspace tests (-d for debug, -p <crate> to scope, --gpu for #[ignore])
python bootstrap.py b        # workspace build (--release by default; -d for debug)
```

Or invoke Cargo directly:

```bash
cargo +1.98.0 clippy --workspace --all-targets -- -D warnings
cargo +1.98.0 test --workspace --lib
```

GPU and runtime-driver tests are marked `#[ignore]`; run them with `-- --ignored` on a box
with an NVIDIA GPU.

---

## Conventions

- **License**: Apache-2.0.
- **NGX / DLSS Notice**: `ngx-sys` and `ngx` only bind the SDK. Applications that redistribute
  NVIDIA DLSS DLLs (`nvngx_dlss*.dll`) remain responsible for NVIDIA RTX SDK licence
  compliance and pre-release notification.
- **Commits**: DCO sign-off on every commit (`git commit -s`). Keep the tree clean, keep
  comments honest, and never leave a `#[allow(...)]` where a real fix belongs.
