//! Generates the nvjpeg bindings from the CUDA Toolkit (nv-bindgen, PLAN.md §3).
use nv_bindgen::{LibSpec, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "nvjpeg",
    headers: &["nvjpeg.h"],
    sdks: &[Sdk::CudaToolkit],
    allow_fn: &["^nvjpeg.*"],
    allow_type: &["^nvjpeg.*"],
    allow_var: &["^NVJPEG_.*"],
    reuse_cuda_types: &["CUstream_st"],
    positive_enums: &["nvjpegStatus_t"],
    shape: Shape::Exports {
        api_type: "NvjpegApi",
        loader_fn: "nvjpeg_api",
        status_types: &["nvjpegStatus_t"],
        not_loaded: "nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED",
        missing_symbol: "nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED",
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
