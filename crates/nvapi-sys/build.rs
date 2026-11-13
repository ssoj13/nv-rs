//! Generates the NVAPI bindings from the NVAPI SDK (`nvapi.h`; root from `NVAPI_ROOT`, headers
//! at the root). Function ids come from the SDK's `nvapi_interface.h`.
use nv_bindgen::{LibSpec, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "nvapi",
    headers: &["nvapi.h"],
    sdks: &[Sdk::EnvRootSubdir { var: "NVAPI_ROOT", subdir: "" }],
    allow_fn: &["^NvAPI_.*"],
    allow_type: &["^_?NV.*", "^_?Nv.*"],
    allow_var: &["^NVAPI_.*", "^NV_.*"],
    reuse_cuda_types: &[],
    positive_enums: &[],
    // NVAPI annotates every entry point with `__success(return == NVAPI_OK)`; MSVC's SAL
    // definition of it is not C clang can parse, so it is emptied.
    clang_args: &["-D__success(x)="],
    shape: Shape::QueryInterface {
        interface_header: "nvapi_interface.h",
        loader_fn: "nvapi_api",
        status_types: &["NvAPI_Status"],
        not_loaded: "_NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND",
        missing_symbol: "_NvAPI_Status_NVAPI_NO_IMPLEMENTATION",
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
