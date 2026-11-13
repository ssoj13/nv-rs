//! Raw cublas bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed CUDA Toolkit
//! (cublas_v2.h; headers are never committed). Each shim loads the toolkit's
//! library on first use through `nv-loader` and returns `cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED` when it is absent or
//! `cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED` when a symbol is missing — never a panic. 
//! **Used by** a future safe `cublas` crate; call [`load`] for a diagnosable load error.

use nv_loader::{DllSpec, Group};

nv_loader::exports_sys! {
    lib: "cublas",
    api: CublasApi,
    loader: cublas_api,
    dll: DllSpec { lib: "cublas", group: Group::Toolkit, windows: &["cublas64_*"], linux: &["libcublas.so.13", "libcublas.so"] },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in ["cublasCreate_v2", "cublasSgemm_v2", "cublasSetStream_v2"] {
            assert!(INVENTORY.contains(&format!("\"{f}\"")), "{f} missing from the inventory");
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
