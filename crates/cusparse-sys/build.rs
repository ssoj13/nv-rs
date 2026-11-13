//! Generates the cusparse bindings from the CUDA Toolkit (nv-bindgen, PLAN.md §3).
use nv_bindgen::{LibSpec, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "cusparse",
    headers: &["cusparse.h"],
    sdks: &[Sdk::CudaToolkit],
    allow_fn: &["^cusparse.*"],
    allow_type: &["^cusparse.*"],
    allow_var: &["^CUSPARSE_.*"],
    reuse_cuda_types: &["CUstream_st"],
    positive_enums: &["cusparseStatus_t"],
    shape: Shape::Exports {
        api_type: "CusparseApi",
        loader_fn: "cusparse_api",
        status_types: &["cusparseStatus_t"],
        not_loaded: "cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED",
        missing_symbol: "cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED",
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
