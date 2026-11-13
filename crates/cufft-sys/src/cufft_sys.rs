//! Raw cuFFT bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed toolkit's `cufft.h`. Each
//! shim loads the toolkit's versioned `cufft64_<N>.dll` / `libcufft.so.<N>` on first use and
//! returns `CUFFT_SETUP_FAILED` when the library is absent or `CUFFT_NOT_SUPPORTED` when a symbol
//! is missing. `cudaStream_t` is `cuda_bindings::CUstream_st`, so a `cuda-core` stream passes
//! straight in. **Used by** a future safe `cufft` crate.

use nv_loader::{DllSpec, Group};

nv_loader::exports_sys! {
    lib: "cufft",
    api: CufftApi,
    loader: cufft_api,
    dll: DllSpec { lib: "cufft", group: Group::Toolkit, windows: &["cufft64_*"], linux: &["libcufft.so.12", "libcufft.so"] },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in ["cufftCreate", "cufftPlan1d", "cufftExecC2C", "cufftSetStream", "cufftGetVersion"] {
            assert!(INVENTORY.contains(&format!("\"{f}\"")), "{f} missing from the inventory");
        }
    }

    #[test]
    fn streams_are_cuda_bindings_streams() {
        // Compile-time proof: cuFFT's stream type is cuda-bindings' CUstream.
        let _: cudaStream_t = std::ptr::null_mut::<cuda_bindings::CUstream_st>();
    }

    #[test]
    #[ignore = "GPU: needs the CUDA toolkit DLLs; run with --ignored"]
    fn version_is_reported() {
        load().expect("cufft loads");
        let mut v = 0;
        // SAFETY: valid out-pointer.
        assert_eq!(unsafe { cufftGetVersion(&mut v) }, cufftResult_t_CUFFT_SUCCESS);
        assert!(v >= 11000, "{v}");
    }
}
