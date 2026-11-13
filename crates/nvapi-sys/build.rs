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
    // NVAPI's headers define SAL macros (`__in`, `__success`, ...) only when absent and every
    // `nvapi_lite_*.h` undefines its own definitions again at its end (`nvapi_lite_salend.h`), so
    // the body of `nvapi.h` is left without them. Force-including the Windows SDK's `sal.h` first
    // makes them pre-existing: NVAPI then neither defines nor undefines them.
    clang_args: &["-include", "sal.h"],
    // The only bitfield union in NVAPI (`NV_TIMING_FLAG`'s anonymous member); bindgen sizes it 3
    // bytes where MSVC lays out 4. bindgen still emits nested anonymous types of an opaque parent,
    // so the pattern covers the union itself (patterns are whole-name matches).
    opaque_types: &["NV_TIMING_FLAG.*"],
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
