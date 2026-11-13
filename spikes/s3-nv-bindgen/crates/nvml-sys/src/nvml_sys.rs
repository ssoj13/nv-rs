//! Raw NVML bindings (generated at build time) and the process-wide loader.
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, clippy::all)]
include!(concat!(env!("OUT_DIR"), "/nvml.rs"));

use nv_loader::{DllSpec, Group, LoadError, open_first};
use std::sync::OnceLock;

pub const DLL: DllSpec = DllSpec { lib: "nvml", group: Group::Driver, windows: &["nvml.dll"], linux: &["libnvidia-ml.so.1"] };

/// The loaded library, opened once per process.
pub fn load() -> Result<&'static NvmlLib, &'static LoadError> {
    static LIB: OnceLock<Result<NvmlLib, LoadError>> = OnceLock::new();
    LIB.get_or_init(|| {
        // SAFETY: loading the driver's NVML; bindgen resolves symbols lazily and tolerates misses.
        open_first(&DLL, |p| unsafe { NvmlLib::new(p) }.map_err(|e| e.to_string())).map(|(l, _)| l)
    })
    .as_ref()
}
