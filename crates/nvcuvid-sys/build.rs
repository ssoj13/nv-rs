//! Generates the NVDEC (nvcuvid) bindings from the Video Codec SDK's `Interface/nvcuvid.h`.
//! The SDK is a zip with no installer, so its root comes from `VIDEO_CODEC_SDK_ROOT`. CUDA types
//! (`CUresult`, `CUcontext`, `CUstream`) are the `cuda-bindings` ones.
use nv_bindgen::{LibSpec, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "nvcuvid",
    headers: &["nvcuvid.h"],
    sdks: &[Sdk::EnvRootSubdir { var: "VIDEO_CODEC_SDK_ROOT", subdir: "Interface" }, Sdk::CudaToolkit],
    allow_fn: &["^cuvid.*"],
    allow_type: &["^CUVID.*", "^_CUVID.*", "^CUvideo.*", "^cudaVideo.*", "^cudaAudio.*"],
    allow_var: &["^CUVID_.*"],
    reuse_cuda_types: &["CUresult", "cudaError_enum", "CUctx_st", "CUstream_st", "CUarray_st"],
    positive_enums: &[],
    clang_args: &[],
    shape: Shape::Exports {
        api_type: "NvcuvidApi",
        loader_fn: "nvcuvid_api",
        status_types: &["CUresult"],
        not_loaded: "cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED",
        missing_symbol: "cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND",
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
