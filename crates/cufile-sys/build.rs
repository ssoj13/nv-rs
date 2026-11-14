//! Generates the Linux cuFile C API from the installed CUDA Toolkit's `cufile.h`.
//!
//! The shared generator preserves structured status errors and negative I/O errors;
//! SDK discovery belongs to vcv-rs. Other targets expose no cuFile API or SDK requirement.
use nv_bindgen::{LibSpec, ReturnPolicy, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "cufile",
    headers: &["cufile.h"],
    sdks: &[Sdk::CudaToolkit],
    allow_fn: &["^cuFile.*"],
    allow_type: &["^CUfile.*", "^CUFile.*", "^cufile.*"],
    allow_var: &["^CU_FILE_.*", "^CUFILE.*"],
    reuse_cuda_types: &["CUresult", "cudaError_enum", "CUstream_st"],
    positive_enums: &["CUfileOpError"],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::Exports {
        api_type: "CufileApi",
        loader_fn: "cufile_api",
        returns: &[
            ReturnPolicy {
                types: &["CUfileError_t"],
                not_loaded: "CUfileError_t { err: CUfileOpError_CU_FILE_DRIVER_NOT_INITIALIZED, cu_err: cuda_bindings::cudaError_enum_CUDA_SUCCESS }",
                missing_symbol: "CUfileError_t { err: CUfileOpError_CU_FILE_PLATFORM_NOT_SUPPORTED, cu_err: cuda_bindings::cudaError_enum_CUDA_SUCCESS }",
            },
            ReturnPolicy {
                types: &["ssize_t"],
                not_loaded: "-(CUfileOpError_CU_FILE_DRIVER_NOT_INITIALIZED as ssize_t)",
                missing_symbol: "-(CUfileOpError_CU_FILE_PLATFORM_NOT_SUPPORTED as ssize_t)",
            },
        ],
    },
};

fn main() {
    let target_os = match std::env::var("CARGO_CFG_TARGET_OS") {
        Ok(target_os) => target_os,
        Err(error) => {
            println!("cargo::error=CARGO_CFG_TARGET_OS is unavailable: {error}");
            std::process::exit(1);
        }
    };
    if target_os != "linux" {
        return;
    }
    if let Err(error) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={error}");
        std::process::exit(1);
    }
}
