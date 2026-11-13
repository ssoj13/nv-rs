//! Raw NVENC bindings: the Video Codec SDK's types plus the driver's encoder function list.
//!
//! **What.** Types are generated at build time by `nv-bindgen` from `nvEncodeAPI.h` (root from
//! `VIDEO_CODEC_SDK_ROOT`; never committed). [`api`] opens the driver's `nvEncodeAPI64.dll` /
//! `libnvidia-encode.so.1`, checks `NvEncodeAPIGetMaxSupportedVersion` against the header's
//! [`NVENCAPI_MAJOR_VERSION`]/[`NVENCAPI_MINOR_VERSION`] (an older driver is a typed load error),
//! and fills [`NV_ENCODE_API_FUNCTION_LIST`] through `NvEncodeAPICreateInstance`.
//! **Why.** NVENC exports only those two functions; every encoder call goes through the list.
//! **Used by** a future safe `nvenc` crate and media encoders.

use nv_loader::{DllSpec, Group, LoadError, Loaded, open_first};
use std::sync::OnceLock;

// Generated code: lint levels differ per header.
#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code, missing_docs, unnecessary_transmutes, clippy::all, clippy::undocumented_unsafe_blocks)]
mod types {
    include!(concat!(env!("OUT_DIR"), "/nvenc_types.rs"));
}
pub use types::*;

/// Where NVENC ships: the display driver.
pub const DLL: DllSpec = DllSpec {
    lib: "nvenc",
    group: Group::Driver,
    windows: &["nvEncodeAPI64.dll"],
    linux: &["libnvidia-encode.so.1"],
};

/// Entry points (function-list fields) and header sha256 (coverage evidence).
pub const INVENTORY: &str = include_str!(concat!(env!("OUT_DIR"), "/nvenc_inventory.json"));

/// `NVENCAPI_VERSION` from the header (`major | minor << 24`).
pub const API_VERSION: u32 = NVENCAPI_MAJOR_VERSION | (NVENCAPI_MINOR_VERSION << 24);

/// `NVENCAPI_STRUCT_VERSION(ver)`: the header macro bindgen cannot evaluate. Every NVENC struct
/// carries this in its `version` field; the driver rejects a mismatch with `NV_ENC_ERR_INVALID_VERSION`.
pub const fn struct_version(ver: u32) -> u32 {
    API_VERSION | (ver << 16) | (0x7 << 28)
}

/// `NV_ENCODE_API_FUNCTION_LIST_VER`.
pub const FUNCTION_LIST_VERSION: u32 = struct_version(2);

/// The driver's NVENC function list plus the library that backs it (kept loaded for the process).
pub struct NvencApi {
    _lib: libloading::Library,
    /// The driver's highest supported API as `major << 4 | minor`.
    pub driver_max_version: u32,
    /// Entry points for [`API_VERSION`].
    pub list: NV_ENCODE_API_FUNCTION_LIST,
}

// SAFETY: the function list is written once by `NvEncodeAPICreateInstance` and never mutated
// afterwards; its raw-pointer fields are function pointers into the loaded driver and reserved
// slots the driver leaves null. Sharing read-only references across threads is therefore sound;
// thread-safety of the calls themselves is the caller's contract with NVENC (per session).
unsafe impl Send for NvencApi {}
// SAFETY: see `Send` above — immutable after construction.
unsafe impl Sync for NvencApi {}

type MaxVersionFn = unsafe extern "C" fn(*mut u32) -> NVENCSTATUS;
type CreateInstanceFn = unsafe extern "C" fn(*mut NV_ENCODE_API_FUNCTION_LIST) -> NVENCSTATUS;

fn loaded() -> &'static Result<Loaded<NvencApi>, LoadError> {
    static API: OnceLock<Result<Loaded<NvencApi>, LoadError>> = OnceLock::new();
    API.get_or_init(|| open_first(&DLL, open))
}

fn open(path: &std::path::Path) -> Result<NvencApi, String> {
    // SAFETY: loading the driver's encoder library runs only its initialisers.
    let lib = unsafe { libloading::Library::new(path) }.map_err(|e| e.to_string())?;
    // SAFETY: both exports have these signatures in nvEncodeAPI.h.
    let max_version: MaxVersionFn = unsafe { lib.get::<MaxVersionFn>(b"NvEncodeAPIGetMaxSupportedVersion\0") }.map(|s| *s).map_err(|e| e.to_string())?;
    // SAFETY: as above.
    let create: CreateInstanceFn = unsafe { lib.get::<CreateInstanceFn>(b"NvEncodeAPICreateInstance\0") }.map(|s| *s).map_err(|e| e.to_string())?;

    let mut driver_max_version = 0u32;
    // SAFETY: valid out-pointer.
    let r = unsafe { max_version(&mut driver_max_version) };
    if r != _NVENCSTATUS_NV_ENC_SUCCESS {
        return Err(format!("NvEncodeAPIGetMaxSupportedVersion returned {r}"));
    }
    let required = (NVENCAPI_MAJOR_VERSION << 4) | NVENCAPI_MINOR_VERSION;
    if driver_max_version < required {
        return Err(format!(
            "driver supports NVENC API {}.{}, these bindings need {NVENCAPI_MAJOR_VERSION}.{NVENCAPI_MINOR_VERSION}",
            driver_max_version >> 4,
            driver_max_version & 0xf
        ));
    }

    let mut list = NV_ENCODE_API_FUNCTION_LIST { version: FUNCTION_LIST_VERSION, ..Default::default() };
    // SAFETY: `list` is a properly versioned, writable function list.
    let r = unsafe { create(&mut list) };
    if r != _NVENCSTATUS_NV_ENC_SUCCESS {
        return Err(format!("NvEncodeAPICreateInstance returned {r}"));
    }
    Ok(NvencApi { _lib: lib, driver_max_version, list })
}

/// The NVENC function list for this process, loaded on first call.
pub fn api() -> Result<&'static NvencApi, &'static LoadError> {
    loaded().as_ref().map(|l| &l.value)
}

/// Loads NVENC now and reports which file was used, or why loading failed.
pub fn load() -> Result<&'static std::path::Path, &'static LoadError> {
    loaded().as_ref().map(|l| l.path.as_path())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_function_list_entries() {
        for f in ["nvEncOpenEncodeSessionEx", "nvEncInitializeEncoder", "nvEncEncodePicture", "nvEncDestroyEncoder"] {
            assert!(INVENTORY.contains(&format!("\"{f}\"")), "{f} missing from the inventory");
        }
    }

    #[test]
    fn struct_version_matches_the_header_macro() {
        // NVENCAPI_STRUCT_VERSION(2) for API 13.1: 13 | 1<<24 | 2<<16 | 7<<28.
        assert_eq!(struct_version(2), NVENCAPI_MAJOR_VERSION | (NVENCAPI_MINOR_VERSION << 24) | (2 << 16) | (7 << 28));
    }

    #[test]
    #[ignore = "GPU: needs the NVIDIA driver; run with --ignored"]
    fn driver_fills_the_function_list() {
        let api = api().expect("nvEncodeAPI loads for this API version");
        assert!(api.list.nvEncOpenEncodeSessionEx.is_some());
        assert!(api.list.nvEncEncodePicture.is_some());
    }
}
