//! Raw Linux cuFile C API, generated from the installed CUDA Toolkit by `nv-bindgen`.
//!
//! Structured status and negative byte-count failures preserve cuFile's C contracts.
//! `cuFileUseCount` has no documented failure sentinel, so its generated Rust function returns
//! `Result<c_long, nv_loader::CallError>`. The shared loader owns the library for every call.
//! CUDA result and stream types are reused from `cuda-bindings`.
//! Used by nv-rs storage consumers; loading or querying a version does not establish GDS support.
#![cfg(target_os = "linux")]

use nv_loader::{DllSpec, Group};

nv_loader::exports_sys! {
    lib: "cufile",
    api: CufileApi,
    loader: cufile_api,
    dll: DllSpec {
        lib: "cufile",
        group: Group::Toolkit,
        windows: &[],
        linux: &["libcufile.so.0", "libcufile.so"],
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn inventory_covers_public_api() {
        let inventory: Value = serde_json::from_str(INVENTORY).expect("valid generated inventory");
        let symbols = inventory
            .get("symbols")
            .and_then(Value::as_array)
            .expect("symbols array");
        for name in [
            "cuFileHandleRegister",
            "cuFileHandleDeregister",
            "cuFileBufRegister",
            "cuFileBufDeregister",
            "cuFileRead",
            "cuFileWrite",
            "cuFileDriverOpen",
            "cuFileDriverClose",
            "cuFileDriverClose_v2",
            "cuFileUseCount",
            "cuFileDriverGetProperties",
            "cuFileBatchIOSetUp",
            "cuFileBatchIOSubmit",
            "cuFileBatchIOGetStatus",
            "cuFileBatchIOCancel",
            "cuFileBatchIODestroy",
            "cuFileReadAsync",
            "cuFileWriteAsync",
            "cuFileStreamRegister",
            "cuFileStreamDeregister",
            "cuFileGetVersion",
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
        let names: Vec<&str> = unshimmed
            .iter()
            .map(|value| value.as_str().expect("entry point name"))
            .collect();
        assert_eq!(
            names,
            [
                "cuFileBatchIODestroy",
                "cuFileHandleDeregister",
                "cuFileRead",
                "cuFileUseCount",
                "cuFileWrite"
            ]
        );
    }

    #[test]
    fn cuda_types_are_shared() {
        let _: CUstream = std::ptr::null_mut::<cuda_bindings::CUstream_st>();
        let cuda_status: cuda_bindings::CUresult = cuda_bindings::cudaError_enum_CUDA_SUCCESS;
        let status = CUfileError_t {
            err: CUfileOpError_CU_FILE_SUCCESS,
            cu_err: cuda_status,
        };
        assert_eq!(status.cu_err, cuda_bindings::cudaError_enum_CUDA_SUCCESS);
    }

    #[test]
    fn use_count_is_checked() {
        let _: unsafe fn() -> Result<std::os::raw::c_long, nv_loader::CallError> = cuFileUseCount;
    }

    #[test]
    fn void_functions_are_checked() {
        let _: unsafe fn(CUfileHandle_t) -> Result<(), nv_loader::CallError> =
            cuFileHandleDeregister;
        let _: unsafe fn(CUfileBatchHandle_t) -> Result<(), nv_loader::CallError> =
            cuFileBatchIODestroy;
    }

    #[test]
    fn return_policies_preserve_native_abi() {
        let _: unsafe extern "C" fn() -> CUfileError_t = cuFileDriverOpen;
    }

    #[test]
    #[ignore = "runtime: needs the Linux cuFile library; run with --ignored"]
    fn version_is_reported() {
        load().expect("cufile loads");
        let mut version = 0;
        // SAFETY: version is a valid, aligned output integer for the duration of the call.
        let status = unsafe { cuFileGetVersion(&mut version) };
        assert_eq!(status.err, CUfileOpError_CU_FILE_SUCCESS);
        assert!(version > 0, "library version was not reported");
    }
}
