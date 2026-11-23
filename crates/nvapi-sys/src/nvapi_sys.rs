//! Raw NVAPI bindings: types plus one panic-free shim per entry point, resolved by id.
//!
//! **What.** Generated at build time by `nv-bindgen` from the NVAPI SDK (`nvapi.h`, located by
//! vcv-rs; headers never committed). The driver's `nvapi64.dll` exports only
//! `nvapi_QueryInterface(id)`; each shim asks it for its function by the id listed in the SDK's
//! `nvapi_interface.h`, returns `NVAPI_LIBRARY_NOT_FOUND` when the DLL is absent and
//! `NVAPI_NO_IMPLEMENTATION` when this driver has no entry for the id. Windows only (NVAPI does not
//! exist elsewhere; on other targets `load()` reports `NotFound`).
//! **Used by** a future safe `nvapi` crate (driver settings, GPU topology, D3D extensions).
#![cfg(target_os = "windows")]

nv_loader::query_interface_sys! {
    lib: "nvapi",
    loader: nvapi_api,
    query_symbol: b"nvapi_QueryInterface\0",
    dll: nv_loader::catalog::NVAPI.spec,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in [
            "NvAPI_Initialize",
            "NvAPI_Unload",
            "NvAPI_EnumPhysicalGPUs",
            "NvAPI_GPU_GetFullName",
        ] {
            assert!(
                INVENTORY.contains(&format!("\"{f}\"")),
                "{f} missing from the inventory"
            );
        }
    }

    #[test]
    #[ignore = "GPU: needs the NVIDIA driver; run with --ignored"]
    fn initialize_and_enumerate_gpus() {
        load().expect("nvapi64 loads");
        // SAFETY: documented NVAPI sequence with valid out-pointers; Initialize precedes use.
        unsafe {
            assert_eq!(NvAPI_Initialize(), _NvAPI_Status_NVAPI_OK);
            let mut handles: [NvPhysicalGpuHandle; NVAPI_MAX_PHYSICAL_GPUS as usize] =
                [std::ptr::null_mut(); NVAPI_MAX_PHYSICAL_GPUS as usize];
            let mut count = 0u32;
            assert_eq!(
                NvAPI_EnumPhysicalGPUs(handles.as_mut_ptr(), &mut count),
                _NvAPI_Status_NVAPI_OK
            );
            assert!(count >= 1);
            assert_eq!(NvAPI_Unload(), _NvAPI_Status_NVAPI_OK);
        }
    }
}
