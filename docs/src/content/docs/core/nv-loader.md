---
title: nv-loader
description: Runtime discovery and loading of NVIDIA shared libraries with typed errors and sys-crate macros.
sidebar:
  order: 1
---

`nv-loader` (`crates/nv-loader`) is the shared runtime loader used by every `*-sys` crate in `nv-rs`. It replaces per-crate `libloading` boilerplate with a single declarative [`DllSpec`](#dllspec-and-group), deterministic search order, typed diagnostics, and panic-free entry-point macros.

## `DllSpec` and `Group`

Each sys crate describes its native library once using `DllSpec`:

```rust
pub struct DllSpec {
    pub lib: &'static str,
    pub group: Group,
    pub windows: &'static [&'static str],
    pub linux: &'static [&'static str],
}
```

- On **Windows**, `windows` entries are exact file names or `prefix*` patterns (for example `cublas64_13*` or `cufft64_12*`); matching `.dll` files in a searched directory are ordered highest name first.
- On **Linux**, `linux` sonames are tried in order through the dynamic linker (`LD_LIBRARY_PATH`, `ld.so.cache`, `/usr/lib/wsl/lib` on WSL), preceded by any discovered SDK runtime directory.

`Group` controls where `candidates` / `candidates_with` searches:

| Variant | Where it searches |
| --- | --- |
| `Group::Driver` | Display driver libraries in `System32` (Windows) or the system library path (`nvml`, `nvEncodeAPI64`, `nvcuvid`, `nvofapi64`, `nvapi64`). |
| `Group::DriverStore` | Driver-store-only libraries (`nvoptix`, `nvngx`): `System32` first, then each display-class adapter's `OpenGLDriverName` directory from the Windows registry (`HKLM\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}`). |
| `Group::Toolkit` | CUDA Toolkit libraries under the toolkit root (`cufft64_12`, `nvrtc64_130_0`). |
| `Group::Sdk { root_var, hint }` | Separately installed SDKs (`cuDNN`, `nvCOMP`, etc.): executable directory (`exe_dir`) first, then `<root_var>/bin[/x64]` and `<root_var>/lib`, then `hint` (an `Option<&'static str>` baked at build time via `nv_loader::hint(env!("NV_RS_SDK_BIN"))`). |

Discovery reads the environment, filesystem, registry, and executable path through the `Probe` trait (`SystemProbe` in production; mock implementations in unit tests).

## Loading and Typed Errors

`open_first(spec, open)` (and `open_first_with(probe, spec, open)`) walks `candidates` in priority order and returns `Result<Loaded<T>, LoadError>`, where `Loaded<T>` pairs the constructed `value: T` with the `path: PathBuf` that produced it.

- `LoadError::NotFound { lib, tried }` — no candidate file existed; lists every searched `PathBuf`.
- `LoadError::Rejected { lib, failures }` — candidates existed, but every one was rejected by the dynamic linker or constructor; records each `(PathBuf, String)` failure.
- `LoadError::RuntimeTooOld { lib, required, found }` — returned by `check_version(lib, required, found)` when a loaded library reports an older version than the bindings require.

For entry points that lack a native C failure sentinel (such as `void` functions or integer-returning getters without an error code), generated shims return `Result<_, CallError>`:

- `CallError::Load(&'static LoadError)` — the library itself failed to load.
- `CallError::Symbol(Box<SymbolError>)` — the library loaded, but the symbol (`lib`, `symbol`, and `source: Option<&'static libloading::Error>`) was unavailable or a query interface returned null. `SymbolError` is boxed on the cold error path so `CallError` stays within two pointer words.

## Sys-Crate Macros

`nv-loader` provides three macros that wire `nv-bindgen` outputs from `$OUT_DIR` into a sys crate, caching the loaded library in a `std::sync::OnceLock` and exposing `DLL`, `INVENTORY`, and `pub fn load() -> Result<&'static Path, &'static LoadError>`:

- **`exports_sys!`** — for `nv_bindgen::Shape::Exports` libraries. Includes `<lib>_types.rs`, `<lib>_api.rs` (kept private so bindgen's panicking helpers are never exposed), and `<lib>_shims.rs`, re-exporting all types and panic-free shims at the crate root.
- **`split_exports_sys!`** — for `nv_bindgen::Shape::SplitExports` libraries distributed across multiple DLLs (such as NPP). Generates shared root types from `<lib>_types.rs` and one `pub mod <part>` per DLL (`DLL`, `load()`, private `api`, and `shims`), while re-exporting all part shims at the crate root.
- **`query_interface_sys!`** — for `nv_bindgen::Shape::QueryInterface` libraries (such as NVAPI). Opens the library via `QueryInterface::open(path, query_symbol)` (resolving a `unsafe extern "C" fn(u32) -> *mut c_void` entry point) and includes `<lib>_types.rs` and `<lib>_shims.rs`.
