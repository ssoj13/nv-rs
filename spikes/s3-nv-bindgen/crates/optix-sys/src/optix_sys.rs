//! Raw OptiX types (generated) and the function-table loader for the driver's nvoptix.dll.
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, clippy::all)]
include!(concat!(env!("OUT_DIR"), "/optix.rs"));

use nv_loader::{DllSpec, Group, LoadError, open_first};
use std::ffi::{c_int, c_uint, c_void};
use std::sync::OnceLock;

pub const DLL: DllSpec = DllSpec { lib: "optix", group: Group::DriverStore, windows: &["nvoptix.dll"], linux: &["libnvoptix.so.1"] };

/// The OptiX function table plus the library that backs it.
pub struct OptixApi {
    _lib: libloading::Library,
    pub table: OptixFunctionTable,
}

type QueryFn = unsafe extern "C" fn(c_int, c_uint, *mut c_void, *mut *const c_void, *mut c_void, usize) -> OptixResult;

/// Opens nvoptix.dll once and fills the ABI table this crate was generated against.
pub fn load() -> Result<&'static OptixApi, &'static LoadError> {
    static API: OnceLock<Result<OptixApi, LoadError>> = OnceLock::new();
    API.get_or_init(|| {
        open_first(&DLL, |p| unsafe {
            // SAFETY: the driver's OptiX library; the symbol type matches optix_stubs.h.
            let lib = libloading::Library::new(p).map_err(|e| e.to_string())?;
            let query: libloading::Symbol<QueryFn> = lib.get(b"optixQueryFunctionTable\0").map_err(|e| e.to_string())?;
            let mut table = OptixFunctionTable::default();
            let r = query(
                OPTIX_ABI_VERSION as c_int, 0, std::ptr::null_mut(), std::ptr::null_mut(),
                (&mut table as *mut OptixFunctionTable).cast(), size_of::<OptixFunctionTable>(),
            );
            if r != OptixResult_OPTIX_SUCCESS {
                return Err(format!("optixQueryFunctionTable(ABI {OPTIX_ABI_VERSION}) returned {r}"));
            }
            Ok(OptixApi { _lib: lib, table })
        })
        .map(|(a, _)| a)
    })
    .as_ref()
}
