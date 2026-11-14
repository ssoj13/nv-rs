//! Generates the nvjpeg bindings from the CUDA Toolkit (nv-bindgen, PLAN.md §3).
use nv_bindgen::{LibSpec, ReturnPolicy, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "nvjpeg",
    headers: &["nvjpeg.h"],
    sdks: &[Sdk::CudaToolkit],
    allow_fn: &["^nvjpeg.*"],
    allow_type: &["^nvjpeg.*"],
    allow_var: &["^NVJPEG_.*"],
    reuse_cuda_types: &["CUstream_st"],
    positive_enums: &["nvjpegStatus_t"],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::Exports {
        api_type: "NvjpegApi",
        loader_fn: "nvjpeg_api",
        returns: &[ReturnPolicy {
            types: &["nvjpegStatus_t"],
            not_loaded: "nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED",
            missing_symbol: "nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED",
        }],
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
    if let Err(e) = append_cuda12_compat() {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}

fn append_cuda12_compat() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or("OUT_DIR unset")?);
    let types_path = out_dir.join("nvjpeg_types.rs");
    let types_src = std::fs::read_to_string(&types_path)?;
    let mut extra_types = String::new();
    if !types_src.contains("nvjpegOutputFormat_t_NVJPEG_OUTPUT_UNCHANGEDI:") {
        extra_types.push_str(
            "\npub const nvjpegOutputFormat_t_NVJPEG_OUTPUT_UNCHANGEDI: nvjpegOutputFormat_t = 10;\n",
        );
    }
    if !types_src.contains("nvjpegInputFormat_t_NVJPEG_INPUT_YUV:") {
        extra_types.push_str(
            "\npub const nvjpegInputFormat_t_NVJPEG_INPUT_YUV: nvjpegInputFormat_t = 1;\n",
        );
    }
    if !extra_types.is_empty() {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new().append(true).open(&types_path)?;
        f.write_all(extra_types.as_bytes())?;
    }

    let shims_path = out_dir.join("nvjpeg_shims.rs");
    let shims_src = std::fs::read_to_string(&shims_path)?;
    if !shims_src.contains("fn nvjpegEncoderParamsSetRestartInterval(") {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new().append(true).open(&shims_path)?;
        f.write_all(
            br#"
#[allow(clippy::missing_safety_doc)]
pub unsafe fn nvjpegEncoderParamsSetRestartInterval(
    _encoder_params: nvjpegEncoderParams_t,
    _restart_interval: std::os::raw::c_uint,
    _stream: cudaStream_t,
) -> nvjpegStatus_t {
    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
}
"#,
        )?;
    }
    Ok(())
}
