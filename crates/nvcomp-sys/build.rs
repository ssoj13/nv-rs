//! Generates nvCOMP's public and native bitcomp C API from the installed SDK.
//!
//! SDK discovery belongs to vcv-rs; nv-bindgen emits every binding and loader-failure policy.
//! CUDA stream types are reused from cuda-bindings rather than regenerated.
use nv_bindgen::{Kind, LibSpec, ReturnPolicy, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "nvcomp",
    headers: &[
        "nvcomp.h",
        "nvcomp_export.h",
        "nvcomp/shared_types.h",
        "nvcomp/version.h",
        "nvcomp/ans.h",
        "nvcomp/bitcomp.h",
        "nvcomp/cascaded.h",
        "nvcomp/crc32.h",
        "nvcomp/deflate.h",
        "nvcomp/gdeflate.h",
        "nvcomp/gzip.h",
        "nvcomp/lz4.h",
        "nvcomp/snappy.h",
        "nvcomp/zstd.h",
        "nvcomp/native/bitcomp.h",
        "nvcomp/native/bitcomp_mode.h",
    ],
    sdks: &[Sdk::Nv(Kind::Nvcomp), Sdk::CudaToolkit],
    allow_fn: &["^nvcomp.*", "^bitcomp.*"],
    allow_type: &["^nvcomp.*", "^bitcomp.*"],
    allow_var: &["^NVCOMP_.*", "^nvcomp.*", "^BITCOMP_.*"],
    reuse_cuda_types: &["CUstream_st"],
    positive_enums: &["nvcompStatus_t"],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::Exports {
        api_type: "NvcompApi",
        loader_fn: "nvcomp_api",
        returns: &[
            ReturnPolicy {
                types: &["nvcompStatus_t"],
                not_loaded: "nvcompStatus_t_nvcompErrorInternal",
                missing_symbol: "nvcompStatus_t_nvcompErrorNotSupported",
            },
            ReturnPolicy {
                types: &["bitcompResult_t"],
                not_loaded: "bitcompResult_t_BITCOMP_UNKNOWN_ERROR",
                missing_symbol: "bitcompResult_t_BITCOMP_UNKNOWN_ERROR",
            },
        ],
    },
};

fn main() {
    if let Err(error) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={error}");
        std::process::exit(1);
    }
}
