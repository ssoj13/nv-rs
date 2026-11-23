//! Raw cuFFT bindings: types, constants and one panic-free `extern "C"` shim per entry point.
//!
//! **What.** Generated at build time by `nv-bindgen` from the installed toolkit's `cufft.h`. Each
//! shim loads the toolkit's versioned `cufft64_<N>.dll` / `libcufft.so.<N>` on first use and
//! returns `CUFFT_SETUP_FAILED` when the library is absent or `CUFFT_NOT_SUPPORTED` when a symbol
//! is missing. `cudaStream_t` is `cuda_bindings::CUstream_st`, so a `cuda-core` stream passes
//! straight in. **Used by** a future safe `cufft` crate.

use nv_loader::Rejection;

nv_loader::exports_sys! {
    lib: "cufft",
    api: CufftApi,
    loader: cufft_api,
    dll: nv_loader::catalog::CUFFT.spec,
    version: runtime_version,
}

/// Rejects a runtime whose major version is older than the `CUFFT_VER_MAJOR` of the headers these
/// bindings were generated from: within a major, NVIDIA keeps struct layouts and enum values
/// stable; across majors they may differ, which would be silent ABI mismatch.
fn runtime_version(lib: &api::CufftApi) -> Result<(), Rejection> {
    let get = *lib
        .cufftGetProperty
        .as_ref()
        .map_err(|e| format!("cufftGetProperty is unavailable: {e}"))?;
    let mut major = 0;
    // SAFETY: `cufftGetProperty(libraryPropertyType, int*)` (cufft.h) needs no handle or initialisation and
    // writes one int through the valid, exclusive pointer.
    let status = unsafe { get(libraryPropertyType_t_MAJOR_VERSION, &mut major) };
    if status != cufftResult_t_CUFFT_SUCCESS {
        return Err(format!("cufftGetProperty(MAJOR_VERSION) returned {status}").into());
    }
    let found = u32::try_from(major)
        .map_err(|_| format!("cufftGetProperty reported major version {major}"))?;
    nv_loader::check_version(CUFFT_VER_MAJOR, found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_lists_the_core_entry_points() {
        for f in [
            "cufftCreate",
            "cufftPlan1d",
            "cufftExecC2C",
            "cufftSetStream",
            "cufftGetVersion",
        ] {
            assert!(
                INVENTORY.contains(&format!("\"{f}\"")),
                "{f} missing from the inventory"
            );
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
        let status = unsafe { cufftGetVersion(&mut v) };
        assert_eq!(status, cufftResult_t_CUFFT_SUCCESS);
        assert!(v >= 11000, "{v}");
    }
}
