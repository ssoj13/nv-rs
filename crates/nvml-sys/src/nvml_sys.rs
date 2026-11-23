//! Raw NVML bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Everything is generated at build time by `nv-bindgen` from the installed CUDA
//! Toolkit's `nvml.h` (never committed). Each shim loads `nvml.dll` / `libnvidia-ml.so.1` on first
//! use through `nv-loader` and returns `NVML_ERROR_LIBRARY_NOT_FOUND` when the library is absent or
//! `NVML_ERROR_FUNCTION_NOT_FOUND` when this driver lacks the symbol — never a panic.
//! **Why.** The raw layer under a future safe `nvml` crate and under gpu-info-rs's GPU stats.
//! **Used by** safe wrappers in nv-rs; call [`load`] for a diagnosable load error.

nv_loader::exports_sys! {
    lib: "nvml",
    api: NvmlApi,
    loader: nvml_api,
    dll: nv_loader::catalog::NVML.spec,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in [
            "nvmlInit_v2",
            "nvmlShutdown",
            "nvmlDeviceGetCount_v2",
            "nvmlDeviceGetName",
        ] {
            assert!(
                INVENTORY.contains(&format!("\"{f}\"")),
                "{f} missing from the inventory"
            );
        }
    }

    #[test]
    #[ignore = "GPU: needs the NVIDIA driver; run with --ignored"]
    fn init_reports_at_least_one_device() {
        load().expect("nvml loads");
        // SAFETY: plain NVML calls with valid out-pointers; init precedes use, shutdown ends it.
        unsafe {
            assert_eq!(nvmlInit_v2(), nvmlReturn_enum_NVML_SUCCESS);
            let mut n = 0u32;
            assert_eq!(nvmlDeviceGetCount_v2(&mut n), nvmlReturn_enum_NVML_SUCCESS);
            assert!(n >= 1);
            assert_eq!(nvmlShutdown(), nvmlReturn_enum_NVML_SUCCESS);
        }
    }
}
