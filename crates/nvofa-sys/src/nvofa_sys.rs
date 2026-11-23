//! Raw NVIDIA Optical Flow (NVOFA) bindings: SDK types plus the driver's CUDA function list.
//!
//! **What.** Types are generated at build time by `nv-bindgen` from the Optical Flow SDK's
//! `nvOpticalFlowCuda.h` (located by vcv-rs; never committed). [`api`] opens the driver's
//! `nvofapi64.dll` / `libnvidia-opticalflow.so.1` through `nv_loader::open_library`, checks
//! `NvOFGetMaxSupportedApiVersion` against the header's [`API_VERSION`] (an older driver is
//! `LoadError::RuntimeTooOld`) and fills
//! [`NV_OF_CUDA_API_FUNCTION_LIST`] through `NvOFAPICreateInstanceCuda`. CUDA types are the
//! `cuda-bindings` ones. The D3D11/D3D12/Vulkan variants of the interface are not bound yet.
//! **Used by** a future safe `nvofa` crate (motion vectors, frame interpolation).

use nv_loader::{DllSpec, Rejection};

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
    include!(concat!(env!("OUT_DIR"), "/nvofa_types.rs"));
}
pub use types::*;

/// Where NVOFA ships: the display driver.
pub const DLL: DllSpec = nv_loader::catalog::NVOFA.spec;

/// Entry points (function-list fields) and header sha256 (coverage evidence).
pub const INVENTORY: &str = include_str!(concat!(env!("OUT_DIR"), "/nvofa_inventory.json"));

/// `NV_OF_API_VERSION` (`major << 4 | minor`): a cast macro bindgen cannot evaluate.
pub const API_VERSION: u32 = (NV_OF_API_MAJOR_VERSION << 4) | NV_OF_API_MINOR_VERSION;

/// The driver's NVOFA CUDA function list plus the library that backs it (kept loaded).
pub struct NvofaApi {
    _lib: libloading::Library,
    /// The driver's highest supported API as `major << 4 | minor`.
    pub driver_max_version: u32,
    /// Entry points for [`API_VERSION`].
    pub list: NV_OF_CUDA_API_FUNCTION_LIST,
}

// SAFETY: the function list is written once by `NvOFAPICreateInstanceCuda` and never mutated; its
// fields are function pointers into the loaded driver. Read-only sharing across threads is sound;
// thread-safety of the calls is the caller's contract with NVOFA (per handle).
unsafe impl Send for NvofaApi {}
// SAFETY: see `Send` — immutable after construction.
unsafe impl Sync for NvofaApi {}

type MaxVersionFn = unsafe extern "C" fn(*mut u32) -> NV_OF_STATUS;
type CreateCudaFn = unsafe extern "C" fn(u32, *mut NV_OF_CUDA_API_FUNCTION_LIST) -> NV_OF_STATUS;

nv_loader::cached_loader! {
    dll: DLL,
    value: NvofaApi,
    open: open,
    /// The NVOFA CUDA function list for this process, loaded on first call.
    pub fn api
}

fn open(path: &std::path::Path) -> Result<NvofaApi, Rejection> {
    let lib = nv_loader::open_library(path).map_err(|e| e.to_string())?;
    // SAFETY: both exports have these signatures in nvOpticalFlowCommon.h / nvOpticalFlowCuda.h.
    let max_version: MaxVersionFn =
        unsafe { lib.get::<MaxVersionFn>(b"NvOFGetMaxSupportedApiVersion\0") }
            .map(|s| *s)
            .map_err(|e| e.to_string())?;
    // SAFETY: as above.
    let create: CreateCudaFn = unsafe { lib.get::<CreateCudaFn>(b"NvOFAPICreateInstanceCuda\0") }
        .map(|s| *s)
        .map_err(|e| e.to_string())?;

    let mut driver_max_version = 0u32;
    // SAFETY: valid out-pointer.
    let r = unsafe { max_version(&mut driver_max_version) };
    if r != _NV_OF_STATUS_NV_OF_SUCCESS {
        return Err(format!("NvOFGetMaxSupportedApiVersion returned {r}").into());
    }
    // Both sides are `major << 4 | minor`; an older driver is `LoadError::RuntimeTooOld`.
    nv_loader::check_version(API_VERSION, driver_max_version)?;
    let mut list = NV_OF_CUDA_API_FUNCTION_LIST::default();
    // SAFETY: `list` is a writable function list of the header's layout.
    let r = unsafe { create(API_VERSION, &mut list) };
    if r != _NV_OF_STATUS_NV_OF_SUCCESS {
        return Err(format!("NvOFAPICreateInstanceCuda returned {r}").into());
    }
    Ok(NvofaApi {
        _lib: lib,
        driver_max_version,
        list,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_function_list_entries() {
        for f in [
            "nvCreateOpticalFlowCuda",
            "nvOFInit",
            "nvOFExecute",
            "nvOFDestroy",
        ] {
            assert!(
                INVENTORY.contains(&format!("\"{f}\"")),
                "{f} missing from the inventory"
            );
        }
    }

    #[test]
    #[ignore = "GPU: needs the NVIDIA driver; run with --ignored"]
    fn driver_fills_the_function_list() {
        let api = api().expect("nvofapi64 loads for this API version");
        assert!(api.list.nvCreateOpticalFlowCuda.is_some());
        assert!(api.list.nvOFExecute.is_some());
    }
}
