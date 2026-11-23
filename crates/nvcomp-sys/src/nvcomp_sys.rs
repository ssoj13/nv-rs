//! Raw nvCOMP C API, generated from the installed SDK by `nv-bindgen`.
//!
//! The public umbrella and native bitcomp C headers supply the types and entry points;
//! proprietary headers and generated bindings are never committed. The shared `nv-loader`
//! keeps the runtime library alive, resolves calls lazily, and exposes [`load`] for diagnostics.
//! CUDA streams are the same `cuda_bindings::CUstream_st` used by `cuda-core`.
//! Used by nv-rs consumers needing batched compression, checksums, or native bitcomp.

use nv_loader::Rejection;

nv_loader::exports_sys! {
    lib: "nvcomp",
    api: NvcompApi,
    loader: nvcomp_api,
    dll: nv_loader::catalog::NVCOMP.spec.with_hint(env!("NV_RS_SDK_BIN")),
    version: runtime_version,
}

/// Rejects a runtime whose major version is older than the headers' `NVCOMP_VER_MAJOR`.
/// `nvcompProperties_t::version` is `MAKE_SEMANTIC_VERSION(major, minor, patch)` =
/// `major * 1000 + minor * 100 + patch` (`nvcomp/version.h`).
fn runtime_version(lib: &api::NvcompApi) -> Result<(), Rejection> {
    let get = *lib
        .nvcompGetProperties
        .as_ref()
        .map_err(|e| format!("nvcompGetProperties is unavailable: {e}"))?;
    let mut props = nvcompProperties_t::default();
    // SAFETY: `nvcompGetProperties(nvcompProperties_t*)` (nvcomp.h) writes the plain struct
    // through the valid, exclusive pointer and needs no initialisation.
    let status = unsafe { get(&mut props) };
    if status != nvcompStatus_t_nvcompSuccess {
        return Err(format!("nvcompGetProperties returned {status}").into());
    }
    nv_loader::check_version(NVCOMP_VER_MAJOR, props.version / 1000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn inventory_covers_public_families() {
        let inventory: Value = serde_json::from_str(INVENTORY).expect("valid generated inventory");
        let symbols = inventory
            .get("symbols")
            .and_then(Value::as_array)
            .expect("symbols array");
        for name in [
            "nvcompGetProperties",
            "nvcompGetStatusString",
            "nvcompBatchedANSCompressAsync",
            "nvcompBatchedBitcompCompressAsync",
            "nvcompBatchedCascadedCompressAsync",
            "nvcompBatchedCRC32Async",
            "nvcompBatchedDeflateCompressAsync",
            "nvcompBatchedGdeflateCompressAsync",
            "nvcompBatchedGzipCompressAsync",
            "nvcompBatchedLZ4CompressAsync",
            "nvcompBatchedSnappyCompressAsync",
            "nvcompBatchedZstdCompressAsync",
            "bitcompCreatePlan",
            "bitcompHostUncompress",
            "bitcompMaxBuflen",
        ] {
            assert!(
                symbols.iter().any(|symbol| symbol.as_str() == Some(name)),
                "{name} missing"
            );
        }
        let unshimmed = inventory
            .get("unshimmed")
            .and_then(Value::as_array)
            .expect("unshimmed array");
        assert_eq!(
            unshimmed.len(),
            1,
            "unexpected missing native shims: {unshimmed:?}"
        );
        assert_eq!(
            unshimmed.first().and_then(Value::as_str),
            Some("bitcompMaxBuflen")
        );
    }

    #[test]
    fn streams_are_cuda_bindings_streams() {
        let _: cudaStream_t = std::ptr::null_mut::<cuda_bindings::CUstream_st>();
    }

    #[test]
    fn checked_native_and_enum_contracts_match_header_abis() {
        let _: unsafe fn(usize) -> Result<usize, nv_loader::CallError> = bitcompMaxBuflen;
        let _: unsafe extern "C" fn(nvcompStatus_t) -> *const std::os::raw::c_char =
            nvcompGetStatusString;
        let _: unsafe extern "C" fn(*mut nvcompProperties_t) -> nvcompStatus_t =
            nvcompGetProperties;
        let _: u32 = nvcompStatus_t_nvcompSuccess;
        let _: u32 = nvcompType_t_NVCOMP_TYPE_FLOAT32;
        let _: u32 = bitcompAlgorithm_t_BITCOMP_DEFAULT_ALGO;
        let _: i32 = bitcompResult_t_BITCOMP_INVALID_PARAMETER;
        assert_eq!(DLL.linux, ["libnvcomp.so.6", "libnvcomp.so"]);
    }

    #[test]
    #[ignore = "runtime: needs the installed nvCOMP library; run with --ignored"]
    fn native_buffer_bound_is_reported() {
        let query: unsafe fn(usize) -> Result<usize, nv_loader::CallError> = bitcompMaxBuflen;
        for input_bytes in [0, 1, 4096] {
            // SAFETY: this size query takes no pointers and has no input-size precondition.
            let bound = unsafe { query(input_bytes) }.expect("native bitcomp bound is available");
            assert!(
                bound >= input_bytes,
                "bound {bound} is smaller than input {input_bytes}"
            );
        }
    }

    #[test]
    #[ignore = "runtime: needs the installed nvCOMP library; run with --ignored"]
    fn properties_are_reported() {
        load().expect("nvcomp loads");
        let mut properties = nvcompProperties_t::default();
        // SAFETY: properties is a valid, aligned output object for the duration of the call.
        let status = unsafe { nvcompGetProperties(&mut properties) };
        assert_eq!(status, nvcompStatus_t_nvcompSuccess);
        assert!(properties.version > 0, "library version was not reported");
        assert!(
            properties.cudart_version > 0,
            "CUDA runtime version was not reported"
        );
        // SAFETY: the header accepts a valid status value and returns a library-owned C string.
        let text = unsafe { nvcompGetStatusString(nvcompStatus_t_nvcompSuccess) };
        assert!(!text.is_null(), "success status string was not reported");
        // SAFETY: nvCOMP guarantees a NUL-terminated string, and load keeps its library alive.
        let text = unsafe { std::ffi::CStr::from_ptr(text) };
        assert!(
            !text.to_bytes().is_empty(),
            "success status string is empty"
        );
    }
}
