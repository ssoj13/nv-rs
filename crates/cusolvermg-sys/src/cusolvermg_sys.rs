//! Raw cusolvermg bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed CUDA Toolkit
//! (cusolverMg.h; headers are never committed). Each shim loads the toolkit's
//! library on first use through `nv-loader` and returns `cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED` when it is absent or
//! `cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED` when a symbol is missing — never a panic. 
//! **Used by** a future safe `cusolvermg` crate; call [`load`] for a diagnosable load error.

use nv_loader::{DllSpec, Group};

nv_loader::exports_sys! {
    lib: "cusolvermg",
    api: CusolverMgApi,
    loader: cusolvermg_api,
    dll: DllSpec { lib: "cusolvermg", group: Group::Toolkit, windows: &["cusolverMg64_*"], linux: &["libcusolverMg.so.12", "libcusolverMg.so"] },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in ["cusolverMgCreate", "cusolverMgDeviceSelect", "cusolverMgPotrf"] {
            assert!(INVENTORY.contains(&format!("\"{f}\"")), "{f} missing from the inventory");
        }
    }

    #[test]
    #[ignore = "GPU: needs the CUDA toolkit DLLs; run with --ignored"]
    fn library_loads() {
        load().expect("cusolvermg loads");
    }
}
