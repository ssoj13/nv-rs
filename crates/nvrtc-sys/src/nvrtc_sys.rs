//! Raw nvrtc bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed CUDA Toolkit
//! (nvrtc.h; headers are never committed). Each shim loads the toolkit's
//! library on first use through `nv-loader` and returns `nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR` when it is absent or
//! `nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR` when a symbol is missing — never a panic. NVRTC has no dedicated not-loaded or missing-symbol codes, so both report INTERNAL_ERROR; use load() to tell them apart.
//! **Used by** a future safe `nvrtc` crate; call [`load`] for a diagnosable load error.

nv_loader::exports_sys! {
    lib: "nvrtc",
    api: NvrtcApi,
    loader: nvrtc_api,
    dll: nv_loader::catalog::NVRTC.spec,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in [
            "nvrtcCreateProgram",
            "nvrtcCompileProgram",
            "nvrtcGetPTX",
            "nvrtcVersion",
        ] {
            assert!(
                INVENTORY.contains(&format!("\"{f}\"")),
                "{f} missing from the inventory"
            );
        }
    }

    #[test]
    #[ignore = "GPU: needs the CUDA toolkit DLLs; run with --ignored"]
    fn library_loads() {
        load().expect("nvrtc loads");
    }
}
