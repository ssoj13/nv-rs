//! Raw cusolver bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed CUDA Toolkit
//! (cusolverDn.h, cusolverSp.h, cusolverRf.h; headers are never committed). Each shim loads the toolkit's
//! library on first use through `nv-loader` and returns `cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED` when it is absent or
//! `cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED` when a symbol is missing — never a panic. The multi-GPU cusolverMg library ships as a separate DLL and is not bound here.
//! **Used by** a future safe `cusolver` crate; call [`load`] for a diagnosable load error.

use nv_loader::{DllSpec, Group};

nv_loader::exports_sys! {
    lib: "cusolver",
    api: CusolverApi,
    loader: cusolver_api,
    dll: DllSpec { lib: "cusolver", group: Group::Toolkit, windows: &["cusolver64_*"], linux: &["libcusolver.so.12", "libcusolver.so"] },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in ["cusolverDnCreate", "cusolverSpCreate", "cusolverRfCreate"] {
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
        load().expect("cusolver loads");
    }
}
