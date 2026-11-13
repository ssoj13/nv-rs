//! Raw cudnn bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed cuDNN SDK
//! (cudnn.h; headers are never committed). Each shim loads the toolkit's
//! library on first use through `nv-loader` and returns `cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED` when it is absent or
//! `cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED` when a symbol is missing — never a panic. cudnn64_9.dll loads its sub-libraries (cudnn_ops, cudnn_cnn, ...) itself, so their folder must stay together.
//! **Used by** a future safe `cudnn` crate; call [`load`] for a diagnosable load error.

use nv_loader::{DllSpec, Group};

nv_loader::exports_sys! {
    lib: "cudnn",
    api: CudnnApi,
    loader: cudnn_api,
    dll: DllSpec { lib: "cudnn", group: Group::Sdk { root_var: "CUDNN_ROOT", hint: nv_loader::hint(env!("NV_RS_SDK_BIN")) }, windows: &["cudnn64_*"], linux: &["libcudnn.so.9", "libcudnn.so"] },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in ["cudnnCreate", "cudnnConvolutionForward", "cudnnBackendExecute", "cudnnGetVersion"] {
            assert!(INVENTORY.contains(&format!("\"{f}\"")), "{f} missing from the inventory");
        }
    }

    #[test]
    fn streams_are_cuda_bindings_streams() {
        let _: cudaStream_t = std::ptr::null_mut::<cuda_bindings::CUstream_st>();
    }

    #[test]
    #[ignore = "GPU: needs the CUDA toolkit DLLs; run with --ignored"]
    fn library_loads() {
        load().expect("cudnn loads");
    }
}
