//! Raw cusparse bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed CUDA Toolkit
//! (cusparse.h; headers are never committed). Each shim loads the toolkit's
//! library on first use through `nv-loader` and returns `cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED` when it is absent or
//! `cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED` when a symbol is missing — never a panic.
//! **Used by** a future safe `cusparse` crate; call [`load`] for a diagnosable load error.

use nv_loader::{DllSpec, Group};

nv_loader::exports_sys! {
    lib: "cusparse",
    api: CusparseApi,
    loader: cusparse_api,
    dll: DllSpec { lib: "cusparse", group: Group::Toolkit, windows: &["cusparse64_*"], linux: &["libcusparse.so.12", "libcusparse.so"] },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in ["cusparseCreate", "cusparseSpMV", "cusparseSetStream"] {
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
        load().expect("cusparse loads");
    }
}
