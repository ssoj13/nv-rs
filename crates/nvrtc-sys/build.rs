//! Generates the nvrtc bindings from the CUDA Toolkit (nv-bindgen, PLAN.md §3).
use nv_bindgen::{LibSpec, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "nvrtc",
    headers: &["nvrtc.h"],
    sdks: &[Sdk::CudaToolkit],
    allow_fn: &["^nvrtc.*"],
    allow_type: &["^nvrtc.*"],
    allow_var: &["^NVRTC_.*"],
    reuse_cuda_types: &[],
    positive_enums: &["nvrtcResult"],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::Exports {
        api_type: "NvrtcApi",
        loader_fn: "nvrtc_api",
        status_types: &["nvrtcResult"],
        not_loaded: "nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR",
        missing_symbol: "nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR",
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
