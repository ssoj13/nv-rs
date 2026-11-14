//! Generates the cupti bindings from the CUDA Toolkit (nv-bindgen, PLAN.md §3).
use nv_bindgen::{LibSpec, ReturnPolicy, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "cupti",
    headers: &["cupti.h"],
    sdks: &[
        Sdk::CudaToolkitSubdir("extras/CUPTI/include"),
        Sdk::CudaToolkit,
    ],
    allow_fn: &["^cupti.*"],
    allow_type: &["^CUpti.*"],
    allow_var: &["^CUPTI_.*"],
    reuse_cuda_types: &["CUctx_st", "CUstream_st"],
    positive_enums: &["CUptiResult"],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::Exports {
        api_type: "CuptiApi",
        loader_fn: "cupti_api",
        returns: &[ReturnPolicy {
            types: &["CUptiResult"],
            not_loaded: "CUptiResult_CUPTI_ERROR_NOT_INITIALIZED",
            missing_symbol: "CUptiResult_CUPTI_ERROR_NOT_SUPPORTED",
        }],
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
    if let Err(e) = append_latest_kernel_alias() {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}

fn append_latest_kernel_alias() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or("OUT_DIR unset")?);
    let types_path = out_dir.join("cupti_types.rs");
    let src = std::fs::read_to_string(&types_path)?;
    if src.contains("pub type CUpti_ActivityKernelLatest") {
        return Ok(());
    }
    let mut best_ver = 0u32;
    for line in src.lines() {
        if let Some(rest) = line.strip_prefix("pub struct CUpti_ActivityKernel") {
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(v) = digits.parse::<u32>() {
                best_ver = best_ver.max(v);
            }
        }
    }
    if best_ver == 0 {
        return Err("no CUpti_ActivityKernelN struct found in cupti_types.rs".into());
    }
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new().append(true).open(&types_path)?;
    writeln!(
        f,
        "\n/// Latest `CUpti_ActivityKernel*` struct present in the bound CUPTI headers.\npub type CUpti_ActivityKernelLatest = CUpti_ActivityKernel{best_ver};"
    )?;
    Ok(())
}
