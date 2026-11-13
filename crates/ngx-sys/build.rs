//! Generates the NGX (DLSS) bindings from the NGX SDK (`nvsdk_ngx.h` + `nvsdk_ngx_vk.h`, located by
//! vcv-rs) and links its static library. Vulkan types come from the Vulkan SDK (`VULKAN_SDK`),
//! which `nvsdk_ngx_vk.h` expects to be included first.
use nv_bindgen::{Kind, LibSpec, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "ngx",
    headers: &["vulkan/vulkan.h", "nvsdk_ngx.h", "nvsdk_ngx_vk.h"],
    sdks: &[Sdk::Nv(Kind::Ngx), Sdk::EnvRootSubdir { var: "VULKAN_SDK", subdir: "Include" }],
    allow_fn: &["^NVSDK_NGX_.*"],
    allow_type: &["^NVSDK_NGX.*", "^_?NVSDK.*"],
    allow_var: &["^NVSDK_NGX.*"],
    reuse_cuda_types: &[],
    positive_enums: &[],
    clang_args: &[],
    opaque_types: &[],
    // NGX ships only static libraries; `_d` is the /MD (dynamic CRT) build Rust's MSVC target uses.
    // nvsdk_ngx_d reads the registry (advapi32) and window ownership (user32).
    shape: Shape::StaticLink { link_libs: &["nvsdk_ngx_d"], system_libs: &["advapi32", "user32"] },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
