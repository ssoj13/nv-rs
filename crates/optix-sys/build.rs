//! Generates the OptiX host types from the OptiX SDK (`optix_function_table.h`, located by vcv-rs).
//! Entry points come from the driver at runtime through `optixQueryFunctionTable`.
use nv_bindgen::{LibSpec, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "optix",
    headers: &["optix_function_table.h"],
    sdks: &[Sdk::Optix, Sdk::CudaToolkit],
    allow_fn: &[],
    allow_type: &["^Optix.*"],
    allow_var: &["^OPTIX_.*"],
    reuse_cuda_types: &["CUctx_st", "CUstream_st"],
    positive_enums: &[],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::FunctionTable {
        table_type: "OptixFunctionTable",
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
