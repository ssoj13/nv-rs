//! Raw NVDEC bindings (`nvcuvid`): video parser and decoder types, one panic-free shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the Video Codec SDK's `nvcuvid.h` /
//! `cuviddec.h` (located by vcv-rs; headers never committed). The decoder ships with
//! the display driver (`nvcuvid.dll` / `libnvcuvid.so.1`). Shims return the CUDA driver's
//! `CUDA_ERROR_NOT_INITIALIZED` when the library is absent and `CUDA_ERROR_NOT_FOUND` when a
//! symbol is missing. `CUresult`, `CUcontext` and `CUstream` are `cuda-bindings` types, so a
//! `cuda-core` context passes straight in.
//! **Used by** a future safe `nvdec` crate and media decoders (ffmpeg-rs, media-rs).

use nv_loader::{DllSpec, Group};

nv_loader::exports_sys! {
    lib: "nvcuvid",
    api: NvcuvidApi,
    loader: nvcuvid_api,
    dll: DllSpec { lib: "nvcuvid", group: Group::Driver, windows: &["nvcuvid.dll"], linux: &["libnvcuvid.so.1"] },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in ["cuvidCreateDecoder", "cuvidDecodePicture", "cuvidCreateVideoParser", "cuvidGetDecoderCaps"] {
            assert!(INVENTORY.contains(&format!("\"{f}\"")), "{f} missing from the inventory");
        }
    }

    #[test]
    fn results_and_contexts_are_cuda_bindings_types() {
        let _: CUresult = cuda_bindings::cudaError_enum_CUDA_SUCCESS;
        let _: CUcontext = std::ptr::null_mut::<cuda_bindings::CUctx_st>();
    }

    #[test]
    #[ignore = "GPU: needs the NVIDIA driver; run with --ignored"]
    fn library_loads() {
        load().expect("nvcuvid loads");
    }
}
