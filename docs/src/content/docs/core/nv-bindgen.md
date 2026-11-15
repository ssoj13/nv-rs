---
title: nv-bindgen
description: Declarative build-time binding generation, vcv-rs SDK discovery, prebuilt fallback, and the nv-bindgen-fixture test harness.
sidebar:
  order: 2
---

`nv-bindgen` (`crates/nv-bindgen`) is the shared build-time binding generator used by every `*-sys` crate's `build.rs` and by `device/optix-device`. Instead of duplicating `bindgen` configuration in twenty-plus crates, each sys crate declares a `LibSpec` and calls `nv_bindgen::generate(&spec)`.

## Declarative `LibSpec`

`LibSpec` describes a library's headers, SDK sources, allowlists, type normalization, and entry-point shape:

- `name`, `headers`, `sdks: &[Sdk]` — library short name, `#include <...>` headers, and ordered SDK search locations (`Sdk::CudaToolkit`, `Sdk::Optix`, `Sdk::EnvRoot`, `Sdk::EnvRootSubdir`, `Sdk::Nv(vcv_rs::nvsdk::Kind)`, `Sdk::CudaToolkitSubdir`, `Sdk::CrateDir`).
- `allow_fn`, `allow_type`, `allow_var` — regex allowlists passed to `bindgen`.
- `reuse_cuda_types` — types defined by `cuda-bindings` (such as `CUstream`) that are blocklisted during generation and re-exported from `cuda-bindings` so every sys crate shares one type.
- `positive_enums` — enums whose C underlying type differs between MSVC (`c_int`) and GCC (`c_uint`), pinned to `c_uint` across targets.
- `clang_args` and `opaque_types` — extra `libclang` flags (for example neutralising SAL annotations) and types emitted as opaque byte blobs with C size/alignment.
- `shape: Shape` — determines how entry points are wrapped:
  - `Shape::Exports` — generates `<name>_types.rs`, `<name>_api.rs`, `<name>_shims.rs`, and `<name>_inventory.json`.
  - `Shape::SplitExports` — shared `<name>_types.rs` plus per-DLL `ExportPart` `<name>_<part>_api.rs` and `<name>_<part>_shims.rs`.
  - `Shape::QueryInterface` — resolves entry points by numeric ID from an `interface_header` via a query function.
  - `Shape::StaticLink` — emits Cargo static/system link directives (`link_libs`, `system_libs`) for static-only SDKs such as NGX.
  - `Shape::FunctionTable` — types and constants only, for libraries whose entry points live in a runtime function table struct (OptiX, NVENC).

For `Exports`, `SplitExports`, and `QueryInterface`, `ReturnPolicy` defines the exact Rust expressions returned when a library is not loaded (`not_loaded`) or a symbol is missing (`missing_symbol`). Functions whose return type has no `ReturnPolicy` (including `void` functions) receive a Rust-ABI wrapper returning `Result<_, nv_loader::CallError>` and are recorded in `Inventory::unshimmed`.

`nv_bindgen::device_asm` provides the device-side counterpart: it scans OptiX device headers for inline PTX statements and emits `cuda_device::ptx_asm!` wrappers for `cuda-oxide`.

## `vcv-rs` Integration

`nv-bindgen` uses `vcv-rs` to locate both NVIDIA SDKs and Windows system headers:

- CUDA Toolkit (`detect_cuda`), OptiX (`vcv_rs::optix::detect_optix`), and separately shipped NVIDIA SDKs (`vcv_rs::nvsdk::Kind`) are resolved automatically without hard-coded machine paths.
- When `Sdk::Nv` discovers an SDK binary directory, `generate` exports `NV_RS_SDK_BIN` via `cargo:rustc-env=NV_RS_SDK_BIN=...` so `nv_loader::hint(env!("NV_RS_SDK_BIN"))` can feed it into `Group::Sdk`.
- On Windows, MSVC and UCRT include directories assembled by `vcv-rs` are passed to `libclang` with `-idirafter`, allowing builds to run from any shell rather than only a Visual Studio developer prompt.

## Prebuilt Fallback

`nv_bindgen::generate` supports building without local SDK headers or `libclang`:

1. If `NV_RS_USE_PREBUILT=1` (or `true`), `generate` copies pre-generated bindings directly into `$OUT_DIR`.
2. Otherwise, if SDK discovery fails with `Error::SdkNotFound` or `Error::MissingLinkDirectory` (for non-`StaticLink` crates), or if header lookup/`bindgen`/`libclang` fails or panics, `generate` falls back to prebuilt bindings if matching files exist for `TARGET` under `NV_RS_PREBUILT_DIR` or `<CARGO_MANIFEST_DIR>/prebuilt/<TARGET>/`.

See [Prebuilt Bindings](/getting-started/prebuilt-bindings/) for staging and artifact details.

## `nv-bindgen-fixture`

`crates/nv-bindgen-fixture` is a T0 compile-and-run test harness that requires no NVIDIA SDK and no GPU. Its `build.rs` runs `nv_bindgen::generate` on synthetic headers (`fake.h` for `Shape::Exports`, and `qi.h` + `qi_interface.h` for `Shape::QueryInterface`), and `src/nv_bindgen_fixture.rs` compiles the generated code through `nv_loader::exports_sys!` and `nv_loader::query_interface_sys!`.

Its unit tests verify:

- Missing libraries produce `LoadError::NotFound` or `LoadError::Rejected` and `CallError::Load`.
- Status, struct, integer, pointer, and `void` shims return their configured `not_loaded` / `missing_symbol` sentinels or `CallError::Symbol` without panicking.
- Present-library and query-interface mock tables forward calls cleanly and preserve `libloading::Error` causes and symbol diagnostics.
