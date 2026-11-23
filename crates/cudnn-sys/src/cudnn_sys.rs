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
//!
//! **Another cuDNN already in the process (e.g. PyTorch's).** On Windows, two cuDNN 9 builds cannot
//! share one process: each part imports its siblings by bare file name, and the loader binds such
//! an import to a module of that name that is already loaded, so parts of two builds get mixed
//! (`ERROR_PROC_NOT_FOUND`, 127). Therefore, when a `cudnn64_<major>.dll` is already loaded, the
//! `adopt` hook ([`loaded_copy`]) makes the loader use exactly that module (its siblings are then
//! mapped from its directory) instead of searching; the major-version gate still applies and the
//! adoption is reported as a loader diagnostic event (`cudnn` / `adopt`). Only when no copy is
//! loaded does the normal search run. The reverse order cannot be fixed from Rust: if nv-rs loads
//! cuDNN first, a later `import torch` that brings a different cuDNN build binds torch's parts to
//! ours and can fail (seen as `OSError: [WinError 127] ... cudnn_cnn64_9.dll`). Import torch (or
//! any other cuDNN user) first, or use one cuDNN build for the whole process. On Linux the dynamic
//! linker already shares one `libcudnn.so.<major>` per soname, so nothing is adopted there.
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
    adopt: loaded_copy,
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
    // Checked before anything is mapped: once a foreign copy is in the process, our siblings'
    // bare-name imports would bind to it (Windows error 127, two cuDNN builds mixed).
    for name in names.iter().map(String::as_str).chain([main_name]) {
        reject_foreign_copy(dir, name)?;
    }
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

/// Fails when a module named `name` is already loaded in this process from a directory other than
/// `dir` (for example the cuDNN copy shipped with PyTorch). The Windows loader resolves bare-name
/// imports to an already-loaded module of that name, so loading this candidate would mix two cuDNN
/// builds; rejecting it lets the loader report it instead of failing with error 127 later.
#[cfg(windows)]
fn reject_foreign_copy(dir: &Path, name: &str) -> Result<(), String> {
    use libloading::os::windows::Library as WinLibrary;

    // `GetModuleHandleExW` matches a bare name against every loaded module and a full path only
    // against the module mapped from that file; neither call loads anything new.
    let Ok(by_name) = WinLibrary::open_already_loaded(name) else {
        return Ok(());
    };
    let ours = dir.join(name);
    let by_name = by_name.into_raw();
    let same = match WinLibrary::open_already_loaded(&ours) {
        Ok(by_path) => {
            let by_path = by_path.into_raw();
            // SAFETY: `by_path` came from `into_raw` just above and is re-owned exactly once, so
            // dropping it releases the reference `GetModuleHandleExW` took.
            drop(unsafe { WinLibrary::from_raw(by_path) });
            by_path == by_name
        }
        Err(_) => false,
    };
    // SAFETY: as above, for the bare-name reference.
    drop(unsafe { WinLibrary::from_raw(by_name) });
    if same {
        Ok(())
    } else {
        Err(format!(
            "a different copy of {name} is already loaded in this process; refusing to load {} \
             next to it (two cuDNN builds cannot share one process on Windows)",
            ours.display()
        ))
    }
}

/// The full path of the `cudnn64_<CUDNN_MAJOR>.dll` already loaded in this process (for example
/// by PyTorch), or `None` when there is none: the loader then adopts that module instead of
/// searching (see the module docs).
#[cfg(windows)]
fn loaded_copy() -> Option<std::path::PathBuf> {
    use libloading::os::windows::Library as WinLibrary;

    let name = format!("cudnn64_{CUDNN_MAJOR}.dll");
    // `GetModuleHandleExW` by bare name: finds a loaded module, never loads one. An error means
    // no such module is loaded, which is the normal case.
    let module = WinLibrary::open_already_loaded(&name).ok()?.into_raw();
    let path = module_file_name(module);
    // SAFETY: `module` came from `into_raw` just above and is re-owned exactly once, so dropping it
    // releases the reference `GetModuleHandleExW` took; the module itself stays loaded by its
    // original owner.
    drop(unsafe { WinLibrary::from_raw(module) });
    match path {
        Ok(path) => Some(path),
        Err(e) => {
            // Without the path the copy cannot be adopted; the normal search then rejects every
            // candidate next to it with a clear message (`reject_foreign_copy`).
            nv_loader::diag::record_event(
                nv_loader::diag::Level::Warn,
                "cudnn",
                "adopt",
                format!("{name} is loaded but its path could not be read: {e}"),
                0,
                false,
            );
            None
        }
    }
}

/// `GetModuleFileNameW` for a module handle obtained from `libloading`.
#[cfg(windows)]
fn module_file_name(module: isize) -> Result<std::path::PathBuf, std::io::Error> {
    use std::os::windows::ffi::OsStringExt;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleFileNameW(module: isize, filename: *mut u16, size: u32) -> u32;
    }
    /// Longest path Windows supports (extended-length paths).
    const MAX_LEN: usize = 32_768;
    let mut buf = vec![0u16; 260];
    loop {
        let size = u32::try_from(buf.len()).unwrap_or(u32::MAX);
        // SAFETY: `module` is a live module handle held by the caller; `buf` is writable for
        // `size` UTF-16 units and the call writes at most that many.
        let len = unsafe { GetModuleFileNameW(module, buf.as_mut_ptr(), size) } as usize;
        if len == 0 {
            return Err(std::io::Error::last_os_error());
        }
        if len < buf.len() {
            buf.truncate(len);
            return Ok(std::ffi::OsString::from_wide(&buf).into());
        }
        // Truncated: the result filled the whole buffer.
        if buf.len() >= MAX_LEN {
            return Err(std::io::Error::other("module path longer than 32768 characters"));
        }
        buf.resize((buf.len() * 2).min(MAX_LEN), 0);
    }
}

/// Linux shares one loaded `libcudnn.so.<major>` per soname through the dynamic linker.
#[cfg(not(windows))]
fn loaded_copy() -> Option<std::path::PathBuf> {
    None
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

    #[cfg(windows)]
    #[test]
    #[ignore = "GPU: needs the CUDA toolkit DLLs; run with --ignored"]
    fn adopts_the_copy_already_loaded() {
        let path = load().expect("cudnn loads");
        // Now a `cudnn64_<major>.dll` is in the process: the adopt hook must name that same file.
        let adopted = loaded_copy().expect("loaded copy found");
        assert_eq!(
            std::fs::canonicalize(&adopted).expect("adopted path exists"),
            std::fs::canonicalize(path).expect("loaded path exists")
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
