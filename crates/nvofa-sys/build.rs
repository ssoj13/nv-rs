//! Generates the Optical Flow types from the Optical Flow SDK's `NvOFInterface/nvOpticalFlowCuda.h`
//! (located by vcv-rs). Entry points come from the driver's `NvOFAPICreateInstanceCuda`.
use nv_bindgen::{Kind, LibSpec, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "nvofa",
    headers: &["nvOpticalFlowCuda.h"],
    sdks: &[Sdk::Nv(Kind::OpticalFlow), Sdk::CudaToolkit],
    allow_fn: &[],
    allow_type: &[
        "^_?NV_OF.*",
        "^NvOF.*",
        "^PFNNVOF.*",
        "^_?NV_OF_CUDA_API_FUNCTION_LIST$",
    ],
    allow_var: &["^NV_OF_.*"],
    reuse_cuda_types: &["CUctx_st", "CUstream_st", "CUarray_st"],
    positive_enums: &[],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::FunctionTable {
        table_type: "_NV_OF_CUDA_API_FUNCTION_LIST",
        probes: &[],
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
