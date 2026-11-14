//! Raw cudnn bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed cuDNN SDK
//! (cudnn.h; headers are never committed). Each shim loads the toolkit's
//! library on first use through `nv-loader` and returns `cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED` when it is absent or
//! `cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED` when a symbol is missing — never a panic.
//! **How.** On Windows, `cudnn64_9.dll` is a thin dispatcher that loads its sibling sub-libraries
//! (`cudnn_graph64_9.dll`, `cudnn_ops64_9.dll`, `cudnn_cnn64_9.dll`, ...) by bare filename via
//! `LoadLibraryA`. When the cuDNN `bin` directory is not on `PATH` (for example when located via
//! `CUDNN_ROOT` or `NV_RS_SDK_BIN`), preloading matching `cudnn_*` siblings from the same directory
//! with `LOAD_WITH_ALTERED_SEARCH_PATH` ensures those sub-libraries are already mapped in the process.
//! **Used by** the safe `cudnn` crate; call [`load`] for a diagnosable load error.

use std::path::Path;
use std::sync::OnceLock;

use nv_loader::{DllSpec, Group, LoadError, Loaded};

// Generated code: lint levels differ per header, so the generated modules opt out.
#[allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    dead_code,
    missing_docs,
    unnecessary_transmutes,
    clippy::all,
    clippy::undocumented_unsafe_blocks
)]
mod types {
    include!(concat!(env!("OUT_DIR"), "/cudnn_types.rs"));
}
#[allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    dead_code,
    missing_docs,
    unnecessary_transmutes,
    clippy::all,
    clippy::undocumented_unsafe_blocks
)]
mod api {
    use super::types::*;
    include!(concat!(env!("OUT_DIR"), "/cudnn_api.rs"));
}
// Shim bodies share one safety argument, documented once in nv-bindgen (`shims`);
// prettyplease drops comments, so the per-block SAFETY lint cannot be met here.
#[allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    missing_docs,
    unnecessary_transmutes,
    clippy::all,
    clippy::undocumented_unsafe_blocks
)]
mod shims {
    use super::cudnn_api;
    use super::types::*;
    include!(concat!(env!("OUT_DIR"), "/cudnn_shims.rs"));
}
pub use shims::*;
pub use types::*;

/// Where this library ships and what its files are called.
pub const DLL: DllSpec = DllSpec {
    lib: "cudnn",
    group: Group::Sdk {
        root_var: "CUDNN_ROOT",
        hint: nv_loader::hint(env!("NV_RS_SDK_BIN")),
    },
    windows: &["cudnn64_*"],
    linux: &["libcudnn.so.9", "libcudnn.so"],
};

/// Entry points and header sha256 of the generated bindings (coverage evidence).
pub const INVENTORY: &str = include_str!(concat!(env!("OUT_DIR"), "/cudnn_inventory.json"));

struct CudnnBundle {
    api: api::CudnnApi,
    #[cfg(windows)]
    _siblings: Vec<libloading::Library>,
}

/// Orders and filters sibling cuDNN sub-library filenames (`cudnn_<part><suffix>`) for `main_name`
/// (e.g. `cudnn64_9.dll` -> `cudnn_*64_9.dll`), placing leaf dependencies (`cudnn_graph`, `cudnn_ops`) first.
#[cfg(any(windows, test))]
fn sibling_dll_names(main_name: &str, entries: &[String]) -> Vec<String> {
    let lower_main = main_name.to_ascii_lowercase();
    let Some(suffix) = lower_main.strip_prefix("cudnn") else {
        return Vec::new();
    };
    if suffix.is_empty() || !suffix.ends_with(".dll") {
        return Vec::new();
    }
    let mut matches: Vec<String> = entries
        .iter()
        .filter(|name| {
            let lower = name.to_ascii_lowercase();
            lower.starts_with("cudnn_") && lower.ends_with(suffix)
        })
        .cloned()
        .collect();
    matches.sort_by_key(|name| {
        let lower = name.to_ascii_lowercase();
        let rank = if lower.starts_with("cudnn_graph") {
            0u8
        } else if lower.starts_with("cudnn_ops") {
            1
        } else if lower.starts_with("cudnn_heuristic") || lower.starts_with("cudnn_ext") {
            2
        } else {
            3
        };
        (rank, lower)
    });
    matches
}

#[cfg(windows)]
fn load_windows_dll(path: &Path) -> Result<libloading::Library, libloading::Error> {
    use libloading::os::windows::{LOAD_WITH_ALTERED_SEARCH_PATH, Library as WinLibrary};

    let abs = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    if abs.is_absolute() {
        // SAFETY: `abs` is an absolute path to a cuDNN DLL; loading runs only its DllMain
        // initializer and `LOAD_WITH_ALTERED_SEARCH_PATH` resolves static imports from `abs`'s folder.
        unsafe { WinLibrary::load_with_flags(&abs, LOAD_WITH_ALTERED_SEARCH_PATH) }.map(Into::into)
    } else {
        // SAFETY: loading the DLL runs only its DllMain initializer and the handle stays alive.
        unsafe { libloading::Library::new(path) }
    }
}

#[cfg(windows)]
fn preload_siblings(main_path: &Path) -> Result<Vec<libloading::Library>, String> {
    let Some(dir) = main_path.parent() else {
        return Ok(Vec::new());
    };
    let Some(main_name) = main_path.file_name().and_then(|n| n.to_str()) else {
        return Ok(Vec::new());
    };
    let entries: Vec<String> = std::fs::read_dir(dir)
        .map_err(|e| format!("failed to list {}: {e}", dir.display()))?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    let names = sibling_dll_names(main_name, &entries);
    let mut libs = Vec::with_capacity(names.len() + 1);
    for name in names {
        let sibling = dir.join(&name);
        let lib = load_windows_dll(&sibling)
            .map_err(|e| format!("failed to preload {}: {e}", sibling.display()))?;
        libs.push(lib);
    }
    let zlib = dir.join("zlibwapi.dll");
    if zlib.is_file()
        && let Ok(lib) = load_windows_dll(&zlib)
    {
        libs.push(lib);
    }
    Ok(libs)
}

#[cfg(windows)]
fn open_cudnn(path: &Path) -> Result<CudnnBundle, String> {
    let siblings = preload_siblings(path)?;
    let main_lib = load_windows_dll(path).map_err(|e| e.to_string())?;
    // SAFETY: `main_lib` is the opened `cudnn64_*.dll` with sibling sub-libraries already loaded;
    // symbols are resolved lazily and missing ones become `Err` fields in `CudnnApi`.
    let api = unsafe { api::CudnnApi::from_library(main_lib) }.map_err(|e| e.to_string())?;
    Ok(CudnnBundle {
        api,
        _siblings: siblings,
    })
}

#[cfg(not(windows))]
fn open_cudnn(path: &Path) -> Result<CudnnBundle, String> {
    // SAFETY: opening the library runs only its loader initialisers; symbols are resolved lazily
    // and a missing one becomes an `Err` field, not a crash.
    let api = unsafe { api::CudnnApi::new(path) }.map_err(|e| e.to_string())?;
    Ok(CudnnBundle { api })
}

fn loaded() -> &'static Result<Loaded<CudnnBundle>, LoadError> {
    static API: OnceLock<Result<Loaded<CudnnBundle>, LoadError>> = OnceLock::new();
    API.get_or_init(|| nv_loader::open_first(&DLL, open_cudnn))
}

/// The loaded library, for the shims.
fn cudnn_api() -> Result<&'static api::CudnnApi, &'static LoadError> {
    loaded().as_ref().map(|l| &l.value.api)
}

/// Loads the library and its sub-libraries now and reports which file was used, or why loading failed.
pub fn load() -> Result<&'static Path, &'static LoadError> {
    loaded().as_ref().map(|l| l.path.as_path())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in [
            "cudnnCreate",
            "cudnnConvolutionForward",
            "cudnnBackendExecute",
            "cudnnGetVersion",
        ] {
            assert!(
                INVENTORY.contains(&format!("\"{f}\"")),
                "{f} missing from the inventory"
            );
        }
    }

    #[test]
    fn streams_are_cuda_bindings_streams() {
        let _: cudaStream_t = std::ptr::null_mut::<cuda_bindings::CUstream_st>();
    }

    #[test]
    fn sibling_dll_names_orders_graph_and_ops_first() {
        let entries = vec![
            "cudnn64_9.dll".to_string(),
            "cudnn_cnn64_9.dll".to_string(),
            "cudnn_ops64_9.dll".to_string(),
            "cudnn_graph64_9.dll".to_string(),
            "cudnn_adv64_9.dll".to_string(),
            "cudnn_ops64_8.dll".to_string(),
            "unrelated.dll".to_string(),
        ];
        assert_eq!(
            sibling_dll_names("cudnn64_9.dll", &entries),
            vec![
                "cudnn_graph64_9.dll",
                "cudnn_ops64_9.dll",
                "cudnn_adv64_9.dll",
                "cudnn_cnn64_9.dll",
            ]
        );
    }

    #[test]
    #[ignore = "GPU: needs the CUDA toolkit DLLs; run with --ignored"]
    fn library_loads() {
        load().expect("cudnn loads");
        // Verify the dispatcher DLL can actually resolve exports from its sub-libraries.
        // SAFETY: `cudnnGetVersion` takes no arguments and queries the loaded library version.
        let ver = unsafe { cudnnGetVersion() }.expect("cudnnGetVersion resolves");
        assert!(ver > 0, "expected non-zero cuDNN version");
    }
}
