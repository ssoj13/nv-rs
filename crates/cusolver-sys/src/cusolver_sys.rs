//! Raw cusolver bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed CUDA Toolkit
//! (cusolverDn.h, cusolverSp.h, cusolverRf.h; headers are never committed). Each shim loads the toolkit's
//! library on first use through `nv-loader` and returns `cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED` when it is absent or
//! `cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED` when a symbol is missing — never a panic. The multi-GPU cusolverMg library ships as a separate DLL and is not bound here.
//! **Used by** a future safe `cusolver` crate; call [`load`] for a diagnosable load error.

use nv_loader::Rejection;

nv_loader::exports_sys! {
    lib: "cusolver",
    api: CusolverApi,
    loader: cusolver_api,
    dll: nv_loader::catalog::CUSOLVER.spec,
    version: runtime_version,
}

/// Rejects a runtime whose major version is older than the `CUSOLVER_VER_MAJOR` of the headers these
/// bindings were generated from: within a major, NVIDIA keeps struct layouts and enum values
/// stable; across majors they may differ, which would be silent ABI mismatch.
fn runtime_version(lib: &api::CusolverApi) -> Result<(), Rejection> {
    let get = *lib
        .cusolverGetProperty
        .as_ref()
        .map_err(|e| format!("cusolverGetProperty is unavailable: {e}"))?;
    let mut major = 0;
    // SAFETY: `cusolverGetProperty(libraryPropertyType, int*)` (cusolver_common.h) needs no handle or initialisation and
    // writes one int through the valid, exclusive pointer.
    let status = unsafe { get(libraryPropertyType_t_MAJOR_VERSION, &mut major) };
    if status != cusolverStatus_t_CUSOLVER_STATUS_SUCCESS {
        return Err(format!("cusolverGetProperty(MAJOR_VERSION) returned {status}").into());
    }
    let found = u32::try_from(major)
        .map_err(|_| format!("cusolverGetProperty reported major version {major}"))?;
    nv_loader::check_version(CUSOLVER_VER_MAJOR, found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in ["cusolverDnCreate", "cusolverSpCreate", "cusolverRfCreate"] {
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
    #[ignore = "GPU: needs the CUDA toolkit DLLs; run with --ignored"]
    fn library_loads() {
        load().expect("cusolver loads");
    }
}
