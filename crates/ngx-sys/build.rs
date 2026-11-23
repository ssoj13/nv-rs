//! Generates the NGX (DLSS) bindings from the NGX SDK (`nvsdk_ngx.h` + `nvsdk_ngx_vk.h`, located by
//! vcv-rs) and links its static library. Vulkan types come from the Vulkan SDK (`VULKAN_SDK`),
//! which `nvsdk_ngx_vk.h` expects to be included first.
use nv_bindgen::{Kind, LibSpec, Sdk, Shape};

const SDKS_WINDOWS: &[Sdk] = &[
    Sdk::Nv(Kind::Ngx),
    Sdk::EnvRootSubdir {
        var: "VULKAN_SDK",
        subdir: "Include",
    },
];

const SDKS_UNIX: &[Sdk] = &[
    Sdk::Nv(Kind::Ngx),
    Sdk::EnvRootSubdir {
        var: "VULKAN_SDK",
        subdir: "include",
    },
];

fn spec_for_shape(shape: Shape, sdks: &'static [Sdk]) -> LibSpec {
    LibSpec {
        name: "ngx",
        headers: &["vulkan/vulkan.h", "nvsdk_ngx.h", "nvsdk_ngx_vk.h"],
        sdks,
        allow_fn: &["^NVSDK_NGX_.*"],
        allow_type: &["^NVSDK_NGX.*", "^_?NVSDK.*"],
        allow_var: &["^NVSDK_NGX.*"],
        reuse_cuda_types: &[],
        positive_enums: &[],
        clang_args: &[],
        opaque_types: &[],
        shape,
    }
}

fn main() {
    println!("cargo::rustc-check-cfg=cfg(has_ngx_sdk)");

    let target = nv_bindgen::target_os("ngx")
        .and_then(|os| nv_bindgen::target_arch("ngx").map(|arch| (os, arch)));
    let (target_os, target_arch) = match target {
        Ok(target) => target,
        Err(e) => {
            println!("cargo::error={e}");
            std::process::exit(1);
        }
    };

    match (target_os.as_str(), target_arch.as_str()) {
        ("windows", _) => {
            // NGX ships only static libraries; `_d` is the /MD (dynamic CRT) build Rust's MSVC
            // target uses. `nvsdk_ngx_d` reads the registry (advapi32) and window ownership (user32).
            let spec = spec_for_shape(
                Shape::StaticLink {
                    link_libs: &["nvsdk_ngx_d"],
                    system_libs: &["advapi32", "user32"],
                },
                SDKS_WINDOWS,
            );
            match nv_bindgen::generate(&spec) {
                Ok(_) => {
                    println!("cargo::rustc-cfg=has_ngx_sdk");
                    println!("cargo::metadata=has_ngx_sdk=1");
                }
                Err(e) => {
                    println!("cargo::error={e}");
                    std::process::exit(1);
                }
            }
        }
        ("linux", "x86_64") => {
            let spec = spec_for_shape(
                Shape::StaticLink {
                    link_libs: &["nvsdk_ngx"],
                    system_libs: &["dl", "stdc++"],
                },
                SDKS_UNIX,
            );
            match nv_bindgen::generate(&spec) {
                Ok(_) => {
                    println!("cargo::rustc-cfg=has_ngx_sdk");
                    println!("cargo::metadata=has_ngx_sdk=1");
                }
                // Only an absent SDK degrades; a found SDK that fails to bind is a real error.
                Err(e @ nv_bindgen::Error::SdkNotFound { .. }) => {
                    println!("cargo::rerun-if-env-changed=NGX_SDK_ROOT");
                    println!("cargo::rerun-if-env-changed=NV_SDK_DIRS");
                    println!(
                        "cargo::warning=NGX SDK not found for Linux x86_64 ({e}); building ngx-sys without bindings"
                    );
                }
                Err(e) => {
                    println!("cargo::error={e}");
                    std::process::exit(1);
                }
            }
        }
        (os, arch) => println!(
            "cargo::warning=NGX ships no SDK for {os}/{arch}; building ngx-sys without bindings"
        ),
    }
}
