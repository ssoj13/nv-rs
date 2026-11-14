//! Generates the cuDNN bindings from the cuDNN SDK (located by vcv-rs; the `include/<cuda>` folder
//! matching the installed CUDA Toolkit) plus the toolkit headers it includes (nv-bindgen, PLAN.md §3).
use nv_bindgen::{Kind, LibSpec, ReturnPolicy, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "cudnn",
    headers: &["cudnn.h"],
    sdks: &[Sdk::Nv(Kind::Cudnn), Sdk::CudaToolkit],
    allow_fn: &["^cudnn.*"],
    allow_type: &["^cudnn.*"],
    allow_var: &["^CUDNN_.*"],
    reuse_cuda_types: &["CUstream_st"],
    positive_enums: &["cudnnStatus_t"],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::Exports {
        api_type: "CudnnApi",
        loader_fn: "cudnn_api",
        returns: &[ReturnPolicy {
            types: &["cudnnStatus_t"],
            not_loaded: "cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED",
            missing_symbol: "cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED",
        }],
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
