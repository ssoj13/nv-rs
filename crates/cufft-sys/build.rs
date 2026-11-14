//! Generates the cuFFT bindings from the CUDA Toolkit's `cufft.h` (nv-bindgen, PLAN.md §3).
//! `cudaStream_t` resolves to `cuda_bindings::CUstream_st`, so streams are one type across nv-rs.
use nv_bindgen::{LibSpec, ReturnPolicy, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "cufft",
    headers: &["cufft.h"],
    sdks: &[Sdk::CudaToolkit],
    allow_fn: &["^cufft.*"],
    allow_type: &["^cufft.*"],
    allow_var: &["^CUFFT_.*"],
    reuse_cuda_types: &["CUstream_st"],
    positive_enums: &["cufftResult_t"],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::Exports {
        api_type: "CufftApi",
        loader_fn: "cufft_api",
        returns: &[ReturnPolicy {
            types: &["cufftResult", "cufftResult_t"],
            not_loaded: "cufftResult_t_CUFFT_SETUP_FAILED",
            missing_symbol: "cufftResult_t_CUFFT_NOT_SUPPORTED",
        }],
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
