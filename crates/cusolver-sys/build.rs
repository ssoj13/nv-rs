//! Generates the cusolver bindings from the CUDA Toolkit (nv-bindgen, PLAN.md §3).
use nv_bindgen::{LibSpec, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "cusolver",
    headers: &["cusolverDn.h", "cusolverSp.h", "cusolverRf.h"],
    sdks: &[Sdk::CudaToolkit],
    allow_fn: &["^cusolver(Dn|Sp|Rf|GetProperty|GetVersion).*"],
    allow_type: &["^cusolver.*"],
    allow_var: &["^CUSOLVER_.*"],
    reuse_cuda_types: &["CUstream_st"],
    positive_enums: &["cusolverStatus_t"],
    shape: Shape::Exports {
        api_type: "CusolverApi",
        loader_fn: "cusolver_api",
        status_types: &["cusolverStatus_t"],
        not_loaded: "cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED",
        missing_symbol: "cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED",
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
