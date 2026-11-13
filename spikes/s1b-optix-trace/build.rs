//! Spike-only bindgen over the user's OptiX SDK (`OPTIX_ROOT`) and CUDA toolkit (`CUDA_PATH`).
//! Nothing generated here is committed (plan D6).

use std::{env, path::PathBuf};

fn main() {
    let optix = PathBuf::from(env::var("OPTIX_ROOT").expect("set OPTIX_ROOT to the OptiX SDK root"));
    let cuda = PathBuf::from(env::var("CUDA_PATH").expect("set CUDA_PATH to the CUDA toolkit root"));
    println!("cargo:rerun-if-env-changed=OPTIX_ROOT");
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", optix.join("include").display()))
        .clang_arg(format!("-I{}", cuda.join("include").display()))
        .allowlist_type("Optix.*")
        .allowlist_var("OPTIX_.*")
        .allowlist_function("optix.*")
        .default_enum_style(bindgen::EnumVariation::Consts)
        .derive_default(true)
        .layout_tests(false)
        .generate()
        .expect("bindgen over optix_function_table.h failed");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("optix.rs");
    bindings.write_to_file(out).expect("write optix.rs");
}
