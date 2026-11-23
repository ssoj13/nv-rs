//! Raw cublas bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed CUDA Toolkit
//! (cublas_v2.h; headers are never committed). Each shim loads the toolkit's
//! library on first use through `nv-loader` and returns `cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED` when it is absent or
//! `cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED` when a symbol is missing — never a panic.
//! **Used by** a future safe `cublas` crate; call [`load`] for a diagnosable load error.

use nv_loader::Rejection;

nv_loader::exports_sys! {
    lib: "cublas",
    api: CublasApi,
    loader: cublas_api,
    dll: nv_loader::catalog::CUBLAS.spec,
    version: runtime_version,
}

/// Rejects a runtime whose major version is older than the `CUBLAS_VER_MAJOR` of the headers these
/// bindings were generated from: within a major, NVIDIA keeps struct layouts and enum values
/// stable; across majors they may differ, which would be silent ABI mismatch.
fn runtime_version(lib: &api::CublasApi) -> Result<(), Rejection> {
    let get = *lib
        .cublasGetProperty
        .as_ref()
        .map_err(|e| format!("cublasGetProperty is unavailable: {e}"))?;
    let mut major = 0;
    // SAFETY: `cublasGetProperty(libraryPropertyType, int*)` (cublas_api.h) needs no handle or initialisation and
    // writes one int through the valid, exclusive pointer.
    let status = unsafe { get(libraryPropertyType_t_MAJOR_VERSION, &mut major) };
    if status != cublasStatus_t_CUBLAS_STATUS_SUCCESS {
        return Err(format!("cublasGetProperty(MAJOR_VERSION) returned {status}").into());
    }
    let found = u32::try_from(major)
        .map_err(|_| format!("cublasGetProperty reported major version {major}"))?;
    nv_loader::check_version(CUBLAS_VER_MAJOR, found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in ["cublasCreate_v2", "cublasSgemm_v2", "cublasSetStream_v2"] {
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
        load().expect("cublas loads");
    }
}
