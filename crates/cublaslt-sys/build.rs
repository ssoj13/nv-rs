//! Generates the cublaslt bindings from the CUDA Toolkit (nv-bindgen, PLAN.md §3).
use nv_bindgen::{LibSpec, ReturnPolicy, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "cublaslt",
    headers: &["cublasLt.h"],
    sdks: &[Sdk::CudaToolkit],
    allow_fn: &["^cublasLt.*"],
    allow_type: &["^cublasLt.*"],
    allow_var: &["^CUBLASLT_.*"],
    reuse_cuda_types: &["CUstream_st"],
    positive_enums: &["cublasStatus_t"],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::Exports {
        api_type: "CublasLtApi",
        loader_fn: "cublaslt_api",
        returns: &[ReturnPolicy {
            types: &["cublasStatus_t"],
            not_loaded: "cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED",
            missing_symbol: "cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED",
        }],
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
