//! Generates the cublas bindings from the CUDA Toolkit (nv-bindgen, PLAN.md §3).
use nv_bindgen::{LibSpec, ReturnPolicy, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "cublas",
    headers: &["cublas_v2.h"],
    sdks: &[Sdk::CudaToolkit],
    allow_fn: &["^cublas.*"],
    allow_type: &["^cublas.*"],
    allow_var: &["^CUBLAS_.*"],
    reuse_cuda_types: &["CUstream_st"],
    positive_enums: &["cublasStatus_t"],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::Exports {
        api_type: "CublasApi",
        loader_fn: "cublas_api",
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
