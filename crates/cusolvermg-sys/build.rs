//! Generates the cusolvermg bindings from the CUDA Toolkit (nv-bindgen, PLAN.md §3).
use nv_bindgen::{LibSpec, ReturnPolicy, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "cusolvermg",
    headers: &["cusolverMg.h"],
    sdks: &[Sdk::CudaToolkit],
    allow_fn: &["^cusolverMg.*"],
    allow_type: &["^cusolverMg.*"],
    allow_var: &["^CUSOLVERMG_.*"],
    reuse_cuda_types: &[],
    positive_enums: &["cusolverStatus_t"],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::Exports {
        api_type: "CusolverMgApi",
        loader_fn: "cusolvermg_api",
        returns: &[ReturnPolicy {
            types: &["cusolverStatus_t"],
            not_loaded: "cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED",
            missing_symbol: "cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED",
        }],
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
