//! Generates the Linux cuFile C API from the installed CUDA Toolkit's `cufile.h`.
//!
//! The shared generator preserves structured `CUfileError_t` status errors; `cuFileRead` /
//! `cuFileWrite` (`ssize_t`, which bindgen spells `isize`) get checked Rust functions, so a missing
//! library is a `CallError` and a negative byte count stays cuFile's own I/O error. SDK discovery
//! belongs to vcv-rs. Other targets expose no cuFile API or SDK requirement.
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
        ],
    },
};

fn main() {
    let result = nv_bindgen::target_os(SPEC.name).and_then(|os| match os.as_str() {
        "linux" => nv_bindgen::generate(&SPEC).map(drop),
        _ => Ok(()),
    });
    if let Err(error) = result {
        println!("cargo::error={error}");
        std::process::exit(1);
    }
}
