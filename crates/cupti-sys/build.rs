//! Generates the cupti bindings from the CUDA Toolkit (nv-bindgen, PLAN.md §3).
use nv_bindgen::{LibSpec, ReturnPolicy, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "cupti",
    headers: &["cupti.h"],
    sdks: &[Sdk::CudaToolkitSubdir("extras/CUPTI/include"), Sdk::CudaToolkit],
    allow_fn: &["^cupti.*"],
    allow_type: &["^CUpti.*"],
    allow_var: &["^CUPTI_.*"],
    reuse_cuda_types: &["CUctx_st", "CUstream_st"],
    positive_enums: &["CUptiResult"],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::Exports {
        api_type: "CuptiApi",
        loader_fn: "cupti_api",
        returns: &[ReturnPolicy {
            types: &["CUptiResult"],
            not_loaded: "CUptiResult_CUPTI_ERROR_NOT_INITIALIZED",
            missing_symbol: "CUptiResult_CUPTI_ERROR_NOT_SUPPORTED",
        }],
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
