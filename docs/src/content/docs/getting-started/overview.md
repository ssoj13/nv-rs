---
title: Overview
description: Why nv-rs exists, how it is architected, and how build-time bindings and runtime loading work together.
sidebar:
  order: 1
---

`nv-rs` is a multi-crate Rust workspace (Rust 2024 edition, Apache-2.0) that gives you direct, predictable access to the NVIDIA library stack — CUDA math and AI libraries, OptiX ray tracing, video encode/decode, optical flow, compression, DLSS/NGX, display/management APIs, zero-copy CUDA <-> `wgpu` interop, and Rust GPU device code via `cuda-oxide` — built on top of `cuda-bindings` and `cuda-core`.

If you are new to GPU/FFI crates in Rust, this page walks you through **why** `nv-rs` is built the way it is and **how** the pieces fit together. If you are already a seasoned Rust or CUDA engineer, think of this as the architectural map of the hangar.

---

## Why We Built `nv-rs`

Using NVIDIA's native ecosystem from Rust usually runs into four walls:

1. **You cannot ship proprietary SDKs in git.**
   CUDA, OptiX, Video Codec, Optical Flow, and NGX headers and binaries have different licenses and multi-gigabyte footprints. Checking them into a repository is a non-starter.
2. **Compile-time linking breaks portability.**
   If a `*-sys` crate hard-links against `cublas64_13.lib` or `libnvencodeapi.so` at build time, your binary fails to compile on machines without that exact SDK — and crashes at OS loader startup (`STATUS_DLL_NOT_FOUND` / `error while loading shared libraries`) before `main()` even runs if a user lacks a driver component.
3. **Hand-rolled `build.rs` + `bindgen` scripts do not scale.**
   Maintaining 20+ separate FFI crates with ad-hoc header search paths, custom `libclang` flags, and version-specific shims quickly turns into copy-pasted boilerplate.
4. **Raw FFI is noisy, but heavy wrappers hide the metal.**
   In graphics, compute, and video pipelines, you want RAII safety and typed `Result`s — without losing control over streams, memory lifetimes, or zero-copy interop with `wgpu`.

`nv-rs` solves all four with a clean split between **declarative build-time codegen**, **explicit runtime loading**, and **thin, honest safe wrappers**.

---

## How `nv-rs` Works (Build Time vs. Run Time)

Every library in `nv-rs` follows the same two-phase lifecycle:

```text
  BUILD TIME (cargo build)
  ┌─────────────────────────────────────────────────────────────────────┐
  │  *-sys/build.rs  ──►  nv-bindgen (LibSpec)                          │
  │                          │                                          │
  │          ┌───────────────┴───────────────┐                          │
  │          ▼ (SDK headers + libclang)      ▼ (NV_RS_USE_PREBUILT=1)   │
  │     vcv-rs finds SDKs              Prebuilt fallback                │
  │     bindgen emits FFI              (*_types.rs, *_shims.rs, ...)    │
  └─────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
  RUN TIME (your app)
  ┌─────────────────────────────────────────────────────────────────────┐
  │  Safe wrapper (e.g. cublas, optix, nvenc)                           │
  │       │  RAII handles, lifetimes, typed errors                      │
  │       ▼                                                             │
  │  *-sys crate (exports_sys! / split_exports_sys! / query_interface)  │
  │       │                                                             │
  │       ▼                                                             │
  │  nv-loader  ──►  LoadLibrary / dlopen  ──►  Err(Missing) or Ok(Api) │
  └─────────────────────────────────────────────────────────────────────┘
```

### 1. Build Time: `nv-bindgen` + `vcv-rs` (or Prebuilt Fallback)

- Each `*-sys` crate describes what it needs in a declarative `LibSpec` and calls `nv-bindgen` from its `build.rs`.
- `nv-bindgen` uses [`vcv-rs`](https://github.com/ssoj13/vcv-rs) to locate your locally installed NVIDIA SDKs (system paths, environment variables like `CUDA_PATH` / `NV_SDK_DIRS`, or `.sdk/` provisioned by `bootstrap.py`) and generates `*_types.rs`, `*_shims.rs`, `*_api.rs`, and `*_inventory.json`.
- **No headers or `libclang` on a CI runner?** Set `NV_RS_USE_PREBUILT=1` (or stage artifacts under `crates/<crate>/prebuilt/<target_triple>/` via `cargo xtask prebuilt`), and `nv-bindgen` uses pre-generated bindings instead.

### 2. Run Time: `nv-loader` (No Surprise Panics)

- Instead of forcing the OS dynamic linker to resolve every vendor DLL/SO when your executable starts, `*-sys` crates load libraries on demand through [`nv-loader`](/core/nv-loader/).
- If a DLL or `.so` is missing on the user's machine (for example, `nvenc` on a headless node without video drivers, or `nvapi` on a non-NVIDIA GPU), you get a clean, typed `Err` you can handle gracefully — never an unhandled startup crash or panic.
- **Intentional exception:** `ngx-sys` (DLSS / NGX) is statically linked against the NGX SDK library where required by the NGX integration model, while still exposing safe DLSS-SR / DLSS-RR wrappers.

---

## How the Workspace Is Layered

You never have to swallow the whole workspace at once. Pick the layer that matches your job:

| Layer | Purpose | Crates |
|---|---|---|
| **0. Core Engine** | Shared runtime loader & declarative `bindgen` pipeline | [`nv-loader`](/core/nv-loader/), [`nv-bindgen`](/core/nv-bindgen/), `nv-bindgen-fixture` |
| **1. Raw FFI (`*-sys`)** | Generated C-ABI types, shims, and `nv-loader` dispatch tables | `cublas-sys`, `cudnn-sys`, `optix-sys`, `nvenc-sys`, `nvml-sys`, etc. |
| **2. Safe Wrappers** | Idiomatic Rust types, RAII lifetimes, and typed errors | [Math](/crates/math/), [Graphics & Video](/crates/graphics-video/), [AI, Storage & Mgmt](/crates/ai-storage-mgmt/) |
| **3. Interop & Device** | Zero-copy CUDA <-> `wgpu`, `cuda-core` extensions, Rust GPU kernels | [`cuda-core-ext`](/interop/cuda-core-ext/), [`nv-wgpu`](/interop/nv-wgpu/), [`device/*`](/interop/device-code/) |
| **4. Tooling** | SDK provisioning, coverage tracking, and prebuilt staging | `bootstrap.py`, `xtask` (`coverage`, `prebuilt`) |

---

## Supported Host Targets

`nv-rs` targets 64-bit desktop and server hosts:

| OS | Architectures |
|---|---|
| **Windows** | `x86_64`, `aarch64` |
| **Linux** | `x86_64`, `aarch64` |

OS-specific libraries are scoped to their native platforms (for example, `nvapi` on Windows; `cufile` and `nccl` on Linux).

---

## Choose Your Starting Path

- **New to `nv-rs` and want to build locally?**
  Start with [SDK Setup](/getting-started/sdk-setup/) to inspect what is already on your machine (`python bootstrap.py d s`) and pull public SDKs into `.sdk/`.
- **Building in CI or without local SDK headers?**
  Jump straight to [Prebuilt Bindings](/getting-started/prebuilt-bindings/) (`NV_RS_USE_PREBUILT=1`).
- **Looking for a specific NVIDIA library?**
  Browse the domain guides under **Crates**: [Linear Algebra & Math](/crates/math/), [Imaging, Video & Graphics](/crates/graphics-video/), or [AI, Storage, Comms & Management](/crates/ai-storage-mgmt/).
- **Contributing or adding a new `*-sys` + wrapper crate?**
  Read [Core Infrastructure](/core/nv-loader/) and [Building & Testing](/contributing/building-testing/).
