//! Raw cupti bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed CUDA Toolkit
//! (cupti.h; headers are never committed). Each shim loads the toolkit's
//! library on first use through `nv-loader` and returns `CUptiResult_CUPTI_ERROR_NOT_INITIALIZED` when it is absent or
//! `CUptiResult_CUPTI_ERROR_NOT_SUPPORTED` when a symbol is missing — never a panic. CUPTI lives in extras/CUPTI: headers from extras/CUPTI/include, DLL from extras/CUPTI/lib64.
//! **Used by** a future safe `cupti` crate; call [`load`] for a diagnosable load error.

use nv_loader::{DllSpec, Group};

nv_loader::exports_sys! {
    lib: "cupti",
    api: CuptiApi,
    loader: cupti_api,
    dll: DllSpec { lib: "cupti", group: Group::Toolkit, windows: &["cupti64_*"], linux: &["libcupti.so.13", "libcupti.so"] },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in ["cuptiGetVersion", "cuptiSubscribe", "cuptiActivityEnable"] {
            assert!(INVENTORY.contains(&format!("\"{f}\"")), "{f} missing from the inventory");
        }
    }

    #[test]
    fn contexts_are_cuda_bindings_contexts() {
        let _: CUcontext = std::ptr::null_mut::<cuda_bindings::CUctx_st>();
    }

    #[test]
    #[ignore = "GPU: needs the CUDA toolkit DLLs; run with --ignored"]
    fn library_loads() {
        load().expect("cupti loads");
    }
}
