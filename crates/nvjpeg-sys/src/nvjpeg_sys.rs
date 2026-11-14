//! Raw nvjpeg bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed CUDA Toolkit
//! (nvjpeg.h; headers are never committed). Each shim loads the toolkit's
//! library on first use through `nv-loader` and returns `nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED` when it is absent or
//! `nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED` when a symbol is missing — never a panic.
//! **Used by** a future safe `nvjpeg` crate; call [`load`] for a diagnosable load error.

use nv_loader::{DllSpec, Group};

nv_loader::exports_sys! {
    lib: "nvjpeg",
    api: NvjpegApi,
    loader: nvjpeg_api,
    dll: DllSpec { lib: "nvjpeg", group: Group::Toolkit, windows: &["nvjpeg64_*"], linux: &["libnvjpeg.so.13", "libnvjpeg.so"] },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in ["nvjpegCreateSimple", "nvjpegDecode", "nvjpegGetImageInfo"] {
            assert!(
                INVENTORY.contains(&format!("\"{f}\"")),
                "{f} missing from the inventory"
            );
        }
    }

    #[test]
    fn streams_are_cuda_bindings_streams() {
        let _: cudaStream_t = std::ptr::null_mut::<cuda_bindings::CUstream_st>();
    }

    #[test]
    #[ignore = "GPU: needs the CUDA toolkit DLLs; run with --ignored"]
    fn library_loads() {
        load().expect("nvjpeg loads");
    }
}
