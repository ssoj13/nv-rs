//! Raw OptiX host bindings: the SDK's types plus the driver's ABI function table.
//!
//! **What.** Types are generated at build time by `nv-bindgen` from the installed OptiX SDK
//! (`optix_function_table.h`; headers are never committed). [`api`] opens the driver's
//! `nvoptix.dll` / `libnvoptix.so.1` once through `nv_loader::open_library` — System32, then each display adapter's DriverStore
//! folder, as NVIDIA's `optix_stubs.h` does — and fills [`OptixFunctionTable`] for the ABI these
//! headers describe ([`OPTIX_ABI_VERSION`]). A driver too old for that ABI is a typed
//! `LoadError::Rejected`, not a crash. `CUcontext`/`CUstream` are `cuda-bindings` types.
//! **Used by** the safe `optix` crate (M4).

use nv_loader::DllSpec;
use std::ffi::{c_int, c_uint, c_void};

// Generated code: lint levels differ per header.
#[allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    dead_code,
    missing_docs,
    unnecessary_transmutes,
    clippy::all,
    clippy::undocumented_unsafe_blocks
)]
mod types {
    include!(concat!(env!("OUT_DIR"), "/optix_types.rs"));
}
pub use types::*;

/// Where OptiX ships: inside the display driver.
pub const DLL: DllSpec = nv_loader::catalog::OPTIX.spec;

/// Entry points (function-table fields) and header sha256 (coverage evidence).
pub const INVENTORY: &str = include_str!(concat!(env!("OUT_DIR"), "/optix_inventory.json"));

/// The driver's OptiX function table plus the library that backs it (kept loaded for the process).
pub struct OptixApi {
    _lib: libloading::Library,
    /// Entry points for [`OPTIX_ABI_VERSION`].
    pub table: OptixFunctionTable,
}

type QueryFn = unsafe extern "C" fn(
    c_int,
    c_uint,
    *mut c_void,
    *mut *const c_void,
    *mut c_void,
    usize,
) -> OptixResult;

nv_loader::cached_loader! {
    dll: DLL,
    value: OptixApi,
    open: open,
    /// The OptiX function table for this process, loaded on first call.
    pub fn api
}

fn open(path: &std::path::Path) -> Result<OptixApi, String> {
    let lib = nv_loader::open_library(path).map_err(|e| e.to_string())?;
    // SAFETY: `optixQueryFunctionTable` has this signature in every OptiX ABI (optix_stubs.h).
    let query: QueryFn = unsafe { lib.get::<QueryFn>(b"optixQueryFunctionTable\0") }
        .map(|s| *s)
        .map_err(|e| e.to_string())?;
    let mut table = OptixFunctionTable::default();
    // SAFETY: `table` is a writable buffer of exactly the size passed; no options are given.
    let r = unsafe {
        query(
            OPTIX_ABI_VERSION as c_int,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            (&mut table as *mut OptixFunctionTable).cast(),
            size_of::<OptixFunctionTable>(),
        )
    };
    if r != OptixResult_OPTIX_SUCCESS {
        return Err(format!(
            "optixQueryFunctionTable(ABI {OPTIX_ABI_VERSION}) returned {r}"
        ));
    }
    Ok(OptixApi { _lib: lib, table })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_table_entry_points() {
        for f in [
            "optixDeviceContextCreate",
            "optixModuleCreate",
            "optixPipelineCreate",
            "optixLaunch",
            "optixAccelBuild",
        ] {
            assert!(
                INVENTORY.contains(&format!("\"{f}\"")),
                "{f} missing from the inventory"
            );
        }
    }

    #[test]
    fn contexts_and_streams_are_cuda_bindings_types() {
        let _: CUcontext = std::ptr::null_mut::<cuda_bindings::CUctx_st>();
        let _: CUstream = std::ptr::null_mut::<cuda_bindings::CUstream_st>();
    }

    #[test]
    #[ignore = "GPU: needs the NVIDIA driver; run with --ignored"]
    fn driver_fills_the_function_table() {
        let api = api().expect("nvoptix loads for this ABI");
        assert!(api.table.optixDeviceContextCreate.is_some());
        assert!(api.table.optixLaunch.is_some());
    }
}
