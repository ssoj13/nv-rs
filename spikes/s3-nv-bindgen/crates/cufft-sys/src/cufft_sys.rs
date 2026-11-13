//! Raw cuFFT bindings (generated) and the loader for the toolkit's versioned cufft DLL.
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, clippy::all)]
include!(concat!(env!("OUT_DIR"), "/cufft.rs"));

use nv_loader::{DllSpec, Group, LoadError, open_first};
use std::sync::OnceLock;

pub const DLL: DllSpec = DllSpec { lib: "cufft", group: Group::Toolkit, windows: &["cufft64_*"], linux: &["libcufft.so.12", "libcufft.so"] };

pub fn load() -> Result<(&'static CufftLib, &'static std::path::Path), &'static LoadError> {
    static LIB: OnceLock<Result<(CufftLib, std::path::PathBuf), LoadError>> = OnceLock::new();
    LIB.get_or_init(|| open_first(&DLL, |p| unsafe { CufftLib::new(p) }.map_err(|e| e.to_string())))
        .as_ref()
        .map(|(l, p)| (l, p.as_path()))
}
