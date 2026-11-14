//! Generates NCCL's Linux host C API and layouts from the installed SDK.
//!
//! Explicit input headers keep coverage hashes and rebuild triggers tied to all public
//! declarations. CUDA device implementations remain in NCCL's device library.
use nv_bindgen::{Kind, LibSpec, ReturnPolicy, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "nccl",
    headers: &[
        "nccl.h",
        "nccl_device/core.h",
        "nccl_device/coop.h",
        "nccl_device/utility.h",
        "nccl_device/comm.h",
        "nccl_device/net_device.h",
        "nccl_device/impl/core__types.h",
        "nccl_device/impl/comm__types.h",
        "nccl_device/impl/ll_a2a__types.h",
        "nccl_device/ll_a2a.h",
        "nccl_device/impl/lsa_barrier__types.h",
        "nccl_device/lsa_barrier.h",
        "nccl_device/impl/gin_barrier__types.h",
        "nccl_device/gin_barrier.h",
        "nccl_device/impl/gin__types.h",
        "nccl_device/gin.h",
        "nccl_device/gin/gin_device_common.h",
        "nccl_device/gin/gin_device_host_common.h",
        "nccl_device/impl/cft_barrier__types.h",
        "nccl_device/cft_barrier.h",
        "nccl_device/impl/cft__types.h",
        "nccl_device/cft.h",
    ],
    sdks: &[Sdk::Nv(Kind::Nccl), Sdk::CudaToolkit],
    allow_fn: &["^nccl.*", "^pnccl.*"],
    allow_type: &["^nccl.*"],
    allow_var: &["^NCCL_.*", "^nccl.*"],
    reuse_cuda_types: &["CUstream_st", "CUmemLocationType", "CUmemLocationType_enum"],
    positive_enums: &["ncclResult_t"],
    clang_args: &["-DNCCL_OS_LINUX=1"],
    opaque_types: &[],
    shape: Shape::Exports {
        api_type: "NcclApi",
        loader_fn: "nccl_api",
        returns: &[ReturnPolicy {
            types: &["ncclResult_t"],
            not_loaded: "ncclResult_t_ncclSystemError",
            missing_symbol: "ncclResult_t_ncclSystemError",
        }],
    },
};

fn main() {
    let target_os = match std::env::var("CARGO_CFG_TARGET_OS") {
        Ok(target_os) => target_os,
        Err(error) => {
            println!("cargo::error=CARGO_CFG_TARGET_OS is unavailable: {error}");
            std::process::exit(1);
        }
    };
    if target_os != "linux" {
        return;
    }
    println!("cargo::rustc-check-cfg=cfg(nccl_has_device)");
    match nv_bindgen::generate(&SPEC) {
        Ok(_) => println!("cargo::rustc-cfg=nccl_has_device"),
        Err(error) => {
            let msg = error.to_string();
            if msg.contains("nccl_device/") {
                // NCCL < 2.28 (e.g. distro system packages) ships only `nccl.h` without `nccl_device/`.
                let host_only = LibSpec {
                    headers: &["nccl.h"],
                    ..SPEC
                };
                if let Err(host_err) = nv_bindgen::generate(&host_only) {
                    println!("cargo::error={host_err}");
                    std::process::exit(1);
                }
                return;
            }
            println!("cargo::error={error}");
            std::process::exit(1);
        }
    }
}
