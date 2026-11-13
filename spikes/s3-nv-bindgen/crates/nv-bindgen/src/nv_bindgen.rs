//! Build-time binding generator shared by every nv-rs sys crate (spike S3 prototype).
//!
//! A sys crate's `build.rs` is one `generate(&LibSpec { .. })` call. The spec names the headers,
//! the SDK root environment variables whose `include/` dirs are searched, allowlists, the CUDA
//! types to re-use from `cuda-bindings` instead of regenerating them, and the library shape:
//! `Exports` libraries get a bindgen dynamic-loading struct (`dynamic_library_name`, missing
//! symbols tolerated), `FunctionTable` libraries (OptiX, NVENC) get plain types — their entry
//! points come from a query function the sys crate calls at runtime. Output: `$OUT_DIR/<name>.rs`.
//! Headers are read from the user's installed SDKs and never committed (PLAN.md D6).

use std::{env, path::PathBuf};

pub enum Shape {
    /// Plain C exports, loaded into a generated struct named `loader`.
    Exports { loader: &'static str },
    /// Entry points come from a runtime query (types only here).
    FunctionTable,
}

pub struct LibSpec {
    pub name: &'static str,
    pub headers: &'static [&'static str],
    /// `(env var, include subdir)` pairs, e.g. `("CUDA_PATH", "include")`.
    pub include_roots: &'static [(&'static str, &'static str)],
    pub allow_fn: &'static [&'static str],
    pub allow_type: &'static [&'static str],
    pub allow_var: &'static [&'static str],
    /// Types defined by `cuda-bindings`; blocklisted here and re-exported via `raw_lines`.
    pub reuse_cuda_types: &'static [&'static str],
    pub raw_lines: &'static [&'static str],
    pub shape: Shape,
}

pub fn generate(spec: &LibSpec) {
    let mut b = bindgen::Builder::default()
        .default_enum_style(bindgen::EnumVariation::Consts)
        .derive_default(true)
        .layout_tests(false)
        .generate_comments(false)
        // Edition 2024 denies unsafe ops in unsafe fns without an inner block (bindgen's loader code).
        .wrap_unsafe_ops(true);
    for h in spec.headers {
        b = b.header_contents(&format!("nv_{}_{h}", spec.name), &format!("#include <{h}>\n"));
    }
    for (var, sub) in spec.include_roots {
        println!("cargo:rerun-if-env-changed={var}");
        let root = env::var(var).unwrap_or_else(|_| panic!("{}: set {var} to the SDK root", spec.name));
        b = b.clang_arg(format!("-I{}", PathBuf::from(root).join(sub).display()));
    }
    for f in spec.allow_fn {
        b = b.allowlist_function(*f);
    }
    for t in spec.allow_type {
        b = b.allowlist_type(*t);
    }
    for v in spec.allow_var {
        b = b.allowlist_var(*v);
    }
    for t in spec.reuse_cuda_types {
        b = b.blocklist_type(*t);
    }
    for l in spec.raw_lines {
        b = b.raw_line(*l);
    }
    if let Shape::Exports { loader } = spec.shape {
        b = b.dynamic_library_name(loader).dynamic_link_require_all(false);
    }
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join(format!("{}.rs", spec.name));
    b.generate()
        .unwrap_or_else(|e| panic!("{}: bindgen failed: {e}", spec.name))
        .write_to_file(&out)
        .unwrap_or_else(|e| panic!("{}: write {}: {e}", spec.name, out.display()));
}
