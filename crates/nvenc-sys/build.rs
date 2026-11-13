//! Generates the NVENC types from the Video Codec SDK's `Interface/nvEncodeAPI.h` (located by vcv-rs:
//! `VIDEO_CODEC_SDK_ROOT` or `NV_SDK_DIRS`). Entry points come from the driver's `NvEncodeAPICreateInstance`.
use nv_bindgen::{Kind, LibSpec, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "nvenc",
    headers: &["nvEncodeAPI.h"],
    sdks: &[Sdk::Nv(Kind::VideoCodec)],
    allow_fn: &[],
    allow_type: &["^_?NV_ENC.*", "^NVENC.*", "^PNVENC.*", "^_?NV_ENCODE_API_FUNCTION_LIST$", "^_?GUID$"],
    allow_var: &["^NVENC.*", "^NV_ENC.*"],
    reuse_cuda_types: &[],
    positive_enums: &[],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::FunctionTable { table_type: "_NV_ENCODE_API_FUNCTION_LIST" },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
