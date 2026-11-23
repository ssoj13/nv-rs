//! Raw cudnn bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed cuDNN SDK
//! (cudnn.h; headers are never committed). Each shim loads the toolkit's
//! library on first use through `nv-loader` and returns `cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED` when it is absent or
//! `cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED` when a symbol is missing — never a panic.
//! **How.** On Windows, `cudnn64_9.dll` is a thin dispatcher that loads its sibling sub-libraries
//! (`cudnn_graph64_9.dll`, `cudnn_ops64_9.dll`, `cudnn_cnn64_9.dll`, ...) by bare filename via
//! `LoadLibraryA`. When the cuDNN `bin` directory is not on `PATH` (for example when located via
//! `CUDNN_ROOT` or the debug-build hint), the `preload` hook of [`nv_loader::exports_sys!`] maps the
//! matching `cudnn_*` siblings from the same directory first (through `nv_loader::open_library`), so
//! those sub-libraries are already in the process. A runtime whose major version is older than the
//! headers' `CUDNN_MAJOR` is a typed `LoadError::RuntimeTooOld`.
//! **Used by** the safe `cudnn` crate; call [`load`] for a diagnosable load error.

use std::path::Path;

use nv_loader::Rejection;

nv_loader::exports_sys! {
    lib: "cudnn",
    api: CudnnApi,
    loader: cudnn_api,
    dll: nv_loader::catalog::CUDNN.spec.with_hint(env!("NV_RS_SDK_BIN")),
    preload: preload_siblings,
    version: runtime_version,
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

/// Maps the dispatcher's sibling sub-libraries from `main_path`'s directory (Windows), leaf
/// dependencies first; they stay loaded with the dispatcher. Linux resolves them through the
/// dynamic linker, so nothing is preloaded there.
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
        let lib = nv_loader::open_library(&sibling)
            .map_err(|e| format!("failed to preload {e}"))?;
        libs.push(lib);
    }
    // Optional: older cuDNN builds import zlib from here; newer ones do not ship it.
    let zlib = dir.join("zlibwapi.dll");
    if zlib.is_file() {
        match nv_loader::open_library(&zlib) {
            Ok(lib) => libs.push(lib),
            Err(e) => nv_loader::diag::record_event(
                nv_loader::diag::Level::Warn,
                "cudnn",
                "preload",
                format!("optional zlibwapi.dll did not load: {e}"),
                0,
                false,
            ),
        }
    }
    Ok(libs)
}

/// See the Windows variant: the dynamic linker resolves cuDNN's sub-libraries itself.
#[cfg(not(windows))]
fn preload_siblings(_main_path: &Path) -> Result<Vec<libloading::Library>, String> {
    Ok(Vec::new())
}

/// Rejects a runtime whose major version is older than the headers' `CUDNN_MAJOR` (cuDNN keeps its
/// ABI within a major version).
fn runtime_version(lib: &api::CudnnApi) -> Result<(), Rejection> {
    let get = *lib
        .cudnnGetProperty
        .as_ref()
        .map_err(|e| format!("cudnnGetProperty is unavailable: {e}"))?;
    let mut major = 0;
    // SAFETY: `cudnnGetProperty(libraryPropertyType, int*)` (cudnn_graph.h) needs no handle and
    // writes one int through the valid, exclusive pointer.
    let status = unsafe { get(libraryPropertyType_t_MAJOR_VERSION, &mut major) };
    if status != cudnnStatus_t_CUDNN_STATUS_SUCCESS {
        return Err(format!("cudnnGetProperty(MAJOR_VERSION) returned {status}").into());
    }
    let found =
        u32::try_from(major).map_err(|_| format!("cudnnGetProperty reported major version {major}"))?;
    nv_loader::check_version(CUDNN_MAJOR, found)
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
