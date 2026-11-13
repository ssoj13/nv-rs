//! Generates the NVML bindings from the CUDA Toolkit's `nvml.h` (nv-bindgen, PLAN.md §3).
use nv_bindgen::{LibSpec, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "nvml",
    headers: &["nvml.h"],
    sdks: &[Sdk::CudaToolkit],
    allow_fn: &["^nvml.*"],
    allow_type: &["^nvml.*"],
    allow_var: &["^NVML_.*"],
    reuse_cuda_types: &[],
    positive_enums: &["nvmlReturn_enum"],
    shape: Shape::Exports {
        api_type: "NvmlApi",
        loader_fn: "nvml_api",
        status_types: &["nvmlReturn_t"],
        not_loaded: "nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND",
        missing_symbol: "nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND",
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
