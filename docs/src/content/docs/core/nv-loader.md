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
- On **Linux**, `linux_candidates_with` emits paths from `extra_search_dirs` (`NV_DLL_DIRS` and `nv_loader::add_search_dir`), runtime `root_var` / `CUDA_PATH` (`CUDA_PATH`, `CUDA_HOME`, `CUDA_ROOT`, `CUDA_TOOLKIT_ROOT_DIR`) `lib64`/`lib` subdirectories, and any build-time `hint` (for `Group::Sdk`), followed by bare `linux` sonames resolved through the dynamic linker (`LD_LIBRARY_PATH`, `ld.so.cache`, `/usr/lib/wsl/lib` on WSL).

`Group` controls where `candidates` / `candidates_with` searches:

| Variant | Where it searches |
| --- | --- |
| `Group::Driver` | Display driver libraries in `System32` (Windows) or the system library path (`nvml`, `nvEncodeAPI64`, `nvcuvid`, `nvofapi64`, `nvapi64`). |
| `Group::DriverStore` | Driver-store-only libraries (`nvoptix`, `nvngx`): `System32` first, then each display-class adapter's `OpenGLDriverName` directory from the Windows registry (`HKLM\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}`), then `extra_search_dirs` (`NV_DLL_DIRS` and `nv_loader::add_search_dir`). |
| `Group::Toolkit` | CUDA Toolkit libraries (`cufft64_12`, `nvrtc64_130_0`): on Windows, `extra_search_dirs`, `exe_dir`, `CUDA_PATH` / `CUDA_HOME` / `CUDA_ROOT` / `CUDA_TOOLKIT_ROOT_DIR` (`bin\x64`, `bin`, `extras\CUPTI\lib64`), `%ProgramFiles%\NVIDIA GPU Computing Toolkit\CUDA\v*` (highest version first), then `PATH`; on Linux, `extra_search_dirs`, `CUDA_PATH` / `CUDA_HOME` / `CUDA_ROOT` / `CUDA_TOOLKIT_ROOT_DIR` (`lib64`, `lib`), then bare sonames. |
| `Group::Sdk { root_var, hint }` | Separately installed SDKs (`cuDNN`, `nvCOMP`, etc.): on Windows, `extra_search_dirs`, `exe_dir`, `<root_var>/bin[/x64]` and `<root_var>/lib`, `hint` (an `Option<&'static str>` baked at build time via `nv_loader::hint(env!("NV_RS_SDK_BIN"))`), then `PATH`; on Linux, `extra_search_dirs`, `<root_var>/lib64` and `<root_var>/lib`, `hint`, then bare sonames. |

Extra search directories (`extra_search_dirs`) can be supplied at runtime either via the `NV_DLL_DIRS` environment variable (parsed with `std::env::split_paths`) or programmatically via `nv_loader::add_search_dir(dir)` before libraries are loaded.

Discovery reads the environment, filesystem, registry, and executable path through the `Probe` trait (`SystemProbe` in production; mock implementations in unit tests).

## Loading and Typed Errors

`open_library(path)` opens a dynamic library at `path`. On Windows, it tries `LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS` (`0x0000_1100`) so transitive DLL dependencies next to `path` resolve without mutating `PATH`, falling back to `libloading::Library::new` on error; on non-Windows platforms it delegates to `libloading::Library::new`. Generated `exports_sys!`, `split_exports_sys!`, and `QueryInterface::open` loaders use `open_library`.

`open_first(spec, open)` (and `open_first_with(probe, spec, open)`) walks `candidates` in priority order and returns `Result<Loaded<T>, LoadError>`, where `Loaded<T>` pairs the constructed `value: T` with the `path: PathBuf` that produced it.

- `LoadError::NotFound { lib, tried }` — no candidate file existed; lists every searched `PathBuf` and includes an actionable hint to place the library next to the executable, set `NV_DLL_DIRS`, or add its directory to `PATH`.
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
