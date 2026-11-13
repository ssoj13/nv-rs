//! Build-time binding generator shared by every nv-rs sys crate.
//!
//! **What.** A sys crate's `build.rs` is one call: [`generate`] with a declarative [`LibSpec`]
//! (headers, which SDKs provide them, allowlists, CUDA types to re-use, and the library's
//! [`Shape`]). It writes into `$OUT_DIR`:
//! - `<name>_types.rs` — types, constants and layout assertions (functions excluded);
//! - for [`Shape::Exports`]: `<name>_api.rs` (a bindgen dynamic-loading struct) and
//!   `<name>_shims.rs` (one panic-free `extern "C"` wrapper per entry point, returning the
//!   library's own status code when the library or the symbol is missing);
//! - `<name>_inventory.json` — every entry-point name plus the sha256 of each header, the input of
//!   the coverage matrix (names and hashes only, never header text).
//!
//! **Why.** Twenty-odd NVIDIA libraries need the same pipeline; without one generator every sys
//! crate grows its own bindgen setup and loader and they drift. Bindgen's own dynamic-loading
//! methods `expect()` a symbol and panic when it is absent, which aborts across `extern "C"`; the
//! shims here never panic (the approach of the cuda-rust_b `xtask` bindings generator).
//!
//! **How.** Header directories come from vcv-rs (`detect_cuda`, `optix::detect_optix`, or an SDK
//! root variable), and on Windows the MSVC/UCRT include directories vcv-rs assembles are passed
//! to libclang with `-idirafter`, so a build works from any shell, not only a VS prompt. The pure
//! steps ([`bindings`], [`shims`], [`inventory`]) are separate from the `$OUT_DIR` writing so they
//! are tested on a synthetic header without any NVIDIA SDK.
//!
//! **Used by** every `*-sys` crate's `build.rs`. Generated headers are never committed (PLAN D6).

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use syn::{Expr, Fields, GenericArgument, Item, ItemStruct, PathArguments, ReturnType, Type, TypeBareFn};

/// Where a library's headers come from.
#[derive(Clone, Copy, Debug)]
pub enum Sdk {
    /// The CUDA Toolkit, located by vcv-rs (`CUDA_PATH`/`CUDA_HOME`/... then the install dir).
    CudaToolkit,
    /// The OptiX SDK, located by vcv-rs (`OPTIX_ROOT`/`OptiX_INSTALL_DIR`/... then ProgramData).
    Optix,
    /// A separately installed SDK whose root (containing `include/`) is named by a variable.
    EnvRoot(&'static str),
}

/// How a library exposes its entry points.
#[derive(Clone, Copy, Debug)]
pub enum Shape {
    /// Plain C exports. Generates a dynamic-loading struct and panic-free shims.
    Exports {
        /// Name of the generated loading struct (`"NvmlApi"`).
        api_type: &'static str,
        /// Name of the sys crate's loader function the shims call; it must return
        /// `Result<&'static <api_type>, _>` (`"nvml_api"`).
        loader_fn: &'static str,
        /// Expression returned when the library could not be loaded.
        not_loaded: &'static str,
        /// Expression returned when the library loaded but lacks this symbol.
        missing_symbol: &'static str,
    },
    /// Entry points come from a runtime query into a table struct (OptiX, NVENC); types only.
    FunctionTable {
        /// The table struct whose fields are the entry points (`"OptixFunctionTable"`).
        table_type: &'static str,
    },
}

/// One library's binding recipe.
#[derive(Clone, Copy, Debug)]
pub struct LibSpec {
    /// Short name, used for output file names (`"nvml"`).
    pub name: &'static str,
    /// Headers to bind, as `#include <...>` names.
    pub headers: &'static [&'static str],
    /// SDKs whose include directories are searched, in order.
    pub sdks: &'static [Sdk],
    /// Function allowlist regexes (entry points).
    pub allow_fn: &'static [&'static str],
    /// Type allowlist regexes.
    pub allow_type: &'static [&'static str],
    /// Constant allowlist regexes.
    pub allow_var: &'static [&'static str],
    /// Types defined by `cuda-bindings`: blocklisted here and re-exported, so `CUstream` etc. are
    /// one type across every sys crate. The sys crate must depend on `cuda-bindings`.
    pub reuse_cuda_types: &'static [&'static str],
    /// Enums that MSVC clang types as `c_int` and GCC as `c_uint`; pinned to `c_uint` so host code
    /// sees one type on every target (same normalisation as cuda-bindings).
    pub positive_enums: &'static [&'static str],
    /// The entry-point shape.
    pub shape: Shape,
}

/// Why generation failed.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A required SDK is not installed (or its root variable is unset).
    #[error("{lib}: {sdk} not found ({hint})")]
    SdkNotFound {
        /// The library.
        lib: &'static str,
        /// The SDK.
        sdk: String,
        /// What was checked.
        hint: String,
    },
    /// A header is in none of the include directories.
    #[error("{lib}: header {header} not found in {searched:?}")]
    HeaderNotFound {
        /// The library.
        lib: &'static str,
        /// The header.
        header: String,
        /// Directories searched.
        searched: Vec<PathBuf>,
    },
    /// bindgen rejected the input.
    #[error("{lib}: bindgen: {msg}")]
    Bindgen {
        /// The library.
        lib: &'static str,
        /// bindgen's message.
        msg: String,
    },
    /// The generated source could not be parsed or lacks an expected item.
    #[error("{lib}: generated source: {msg}")]
    Generated {
        /// The library.
        lib: &'static str,
        /// What was wrong.
        msg: String,
    },
    /// Writing the output failed.
    #[error("{lib}: writing {path}: {source}")]
    Io {
        /// The library.
        lib: &'static str,
        /// The file.
        path: PathBuf,
        /// The I/O error.
        source: std::io::Error,
    },
}

/// The generated sources for one library, before they are written anywhere.
#[derive(Debug)]
pub struct Output {
    /// Types, constants, layout assertions.
    pub types: String,
    /// The dynamic-loading struct ([`Shape::Exports`] only).
    pub api: Option<String>,
    /// The panic-free shims ([`Shape::Exports`] only).
    pub shims: Option<String>,
    /// Entry points and header hashes.
    pub inventory: Inventory,
}

/// What the coverage matrix needs to know about a generated library.
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Inventory {
    /// The library's short name.
    pub lib: String,
    /// Each bound header and its sha256.
    pub headers: Vec<HeaderHash>,
    /// Every entry point, sorted.
    pub symbols: Vec<String>,
}

/// A header and the sha256 of its contents.
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct HeaderHash {
    /// The `#include` name.
    pub name: String,
    /// Lower-case hex sha256.
    pub sha256: String,
}

/// Generates `spec` into `$OUT_DIR` (see the module docs) and prints the `cargo:rerun-if-*` lines.
/// Returns the generated sources' inventory.
pub fn generate(spec: &LibSpec) -> Result<Inventory, Error> {
    for var in rerun_vars(spec) {
        println!("cargo:rerun-if-env-changed={var}");
    }
    let include_dirs = sdk_include_dirs(spec)?;
    let system_dirs = system_include_dirs();
    let out = bindings(spec, &include_dirs, &system_dirs)?;

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").ok_or_else(|| Error::Generated {
        lib: spec.name,
        msg: "OUT_DIR is unset: generate() runs from build.rs".into(),
    })?);
    let write = |suffix: &str, text: &str| {
        let path = out_dir.join(format!("{}_{suffix}", spec.name));
        std::fs::write(&path, text).map_err(|source| Error::Io { lib: spec.name, path, source })
    };
    write("types.rs", &out.types)?;
    if let (Some(api), Some(shims)) = (&out.api, &out.shims) {
        write("api.rs", api)?;
        write("shims.rs", shims)?;
    }
    let json = serde_json::to_string_pretty(&out.inventory).map_err(|e| Error::Generated { lib: spec.name, msg: e.to_string() })?;
    write("inventory.json", &json)?;
    for h in &out.inventory.headers {
        if let Some(path) = find_header(&include_dirs, &h.name) {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
    Ok(out.inventory)
}

/// Runs bindgen for `spec` against explicit include directories (`include_dirs` searched first,
/// `system_dirs` after libclang's own). Pure apart from reading headers.
pub fn bindings(spec: &LibSpec, include_dirs: &[PathBuf], system_dirs: &[PathBuf]) -> Result<Output, Error> {
    let mut headers = Vec::new();
    let mut wrapper = String::new();
    for name in spec.headers {
        let path = find_header(include_dirs, name).ok_or_else(|| Error::HeaderNotFound {
            lib: spec.name,
            header: (*name).to_string(),
            searched: include_dirs.to_vec(),
        })?;
        let bytes = std::fs::read(&path).map_err(|source| Error::Io { lib: spec.name, path: path.clone(), source })?;
        headers.push(HeaderHash { name: (*name).to_string(), sha256: hex(&Sha256::digest(&bytes)) });
        wrapper.push_str(&format!("#include <{name}>\n"));
    }

    let base = || {
        let mut b = bindgen::Builder::default()
            .header_contents(&format!("nv_bindgen_{}.h", spec.name), &wrapper)
            .default_enum_style(bindgen::EnumVariation::Consts)
            .generate_comments(false)
            .wrap_unsafe_ops(true);
        for d in include_dirs {
            b = b.clang_arg(format!("-I{}", d.display()));
        }
        for d in system_dirs {
            b = b.clang_arg(format!("-idirafter{}", d.display()));
        }
        b
    };
    let run = |b: bindgen::Builder| b.generate().map(|g| g.to_string()).map_err(|e| Error::Bindgen { lib: spec.name, msg: e.to_string() });

    // Types pass: everything the allowlists reach except functions.
    let mut tb = base().derive_default(true).blocklist_function(".*");
    for t in spec.allow_type {
        tb = tb.allowlist_type(*t);
    }
    for v in spec.allow_var {
        tb = tb.allowlist_var(*v);
    }
    for t in spec.reuse_cuda_types {
        tb = tb.blocklist_type(*t);
    }
    let mut types = normalize_positive_enums(&run(tb)?, spec.positive_enums);
    if !spec.reuse_cuda_types.is_empty() {
        types = format!("pub use cuda_bindings::{{{}}};\n{types}", spec.reuse_cuda_types.join(", "));
    }

    let (api, shims_src, symbols) = match spec.shape {
        Shape::Exports { api_type, loader_fn, not_loaded, missing_symbol } => {
            let mut ab = base()
                .dynamic_library_name(api_type)
                .dynamic_link_require_all(false)
                .with_codegen_config(bindgen::CodegenConfig::FUNCTIONS);
            for f in spec.allow_fn {
                ab = ab.allowlist_function(*f);
            }
            let api = run(ab)?;
            let shims_src = shims(spec.name, &api, api_type, loader_fn, not_loaded, missing_symbol)?;
            let symbols = inventory(spec.name, &api, api_type)?;
            (Some(api), Some(shims_src), symbols)
        }
        Shape::FunctionTable { table_type } => {
            let symbols = inventory(spec.name, &types, table_type)?;
            (None, None, symbols)
        }
    };

    Ok(Output { types, api, shims: shims_src, inventory: Inventory { lib: spec.name.to_string(), headers, symbols } })
}

/// Panic-free `extern "C"` shims for every function-pointer field of `api_type` in `api_source`.
///
/// Each shim calls `loader_fn()`; a failed load returns `not_loaded`, a missing symbol returns
/// `missing_symbol`. `extern "C"` (not `"C-unwind"`) so a shim coerces to a C function pointer;
/// unwinding out of it would abort, hence no panic path exists in the body.
pub fn shims(
    lib: &'static str,
    api_source: &str,
    api_type: &str,
    loader_fn: &str,
    not_loaded: &str,
    missing_symbol: &str,
) -> Result<String, Error> {
    let gen_err = |msg: String| Error::Generated { lib, msg };
    let file = syn::parse_file(api_source).map_err(|e| gen_err(e.to_string()))?;
    let api = find_struct(&file, api_type).ok_or_else(|| gen_err(format!("no struct {api_type}")))?;
    let loader = format_ident!("{loader_fn}");
    let not_loaded: Expr = syn::parse_str(not_loaded).map_err(|e| gen_err(format!("not_loaded: {e}")))?;
    let missing: Expr = syn::parse_str(missing_symbol).map_err(|e| gen_err(format!("missing_symbol: {e}")))?;

    let Fields::Named(fields) = &api.fields else {
        return Err(gen_err(format!("{api_type} has no named fields")));
    };
    let fns = fields.named.iter().filter_map(|field| {
        let name = field.ident.as_ref()?;
        let bare = result_bare_fn(&field.ty)?;
        let args: Vec<(proc_macro2::Ident, TokenStream)> = bare
            .inputs
            .iter()
            .enumerate()
            .map(|(i, arg)| {
                let n = arg.name.as_ref().map(|(id, _)| id.clone()).unwrap_or_else(|| format_ident!("arg_{i}"));
                let ty = &arg.ty;
                (n.clone(), quote!(#n: #ty))
            })
            .collect();
        let names = args.iter().map(|(n, _)| n);
        let defs = args.iter().map(|(_, d)| d);
        let ret = match &bare.output {
            ReturnType::Default => quote!(),
            ReturnType::Type(_, ty) => quote!(-> #ty),
        };
        Some(quote! {
            #[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
            #[inline]
            pub unsafe extern "C" fn #name(#(#defs),*) #ret {
                match #loader() {
                    Ok(api) => match &api.#name {
                        // SAFETY: the caller upholds the C function's contract; the pointer was
                        // resolved from the loaded library, which `api` keeps alive.
                        Ok(f) => unsafe { (*f)(#(#names),*) },
                        Err(_) => #missing,
                    },
                    Err(_) => #not_loaded,
                }
            }
        })
    });
    let tokens = quote!(#(#fns)*);
    let parsed: syn::File = syn::parse2(tokens).map_err(|e| gen_err(format!("shims: {e}")))?;
    Ok(prettyplease::unparse(&parsed))
}

/// Sorted entry-point names: the function-pointer fields of `struct_name` in `source` (bindgen's
/// dynamic-loading struct wraps them in `Result<fn, _>`, a function table holds `Option<fn>`).
pub fn inventory(lib: &'static str, source: &str, struct_name: &str) -> Result<Vec<String>, Error> {
    let file = syn::parse_file(source).map_err(|e| Error::Generated { lib, msg: e.to_string() })?;
    let s = find_struct(&file, struct_name).ok_or_else(|| Error::Generated { lib, msg: format!("no struct {struct_name}") })?;
    let Fields::Named(fields) = &s.fields else {
        return Ok(Vec::new());
    };
    let mut names: Vec<String> = fields
        .named
        .iter()
        .filter(|f| result_bare_fn(&f.ty).is_some() || option_bare_fn(&f.ty).is_some())
        .filter_map(|f| f.ident.as_ref().map(ToString::to_string))
        .collect();
    names.sort();
    Ok(names)
}

fn find_struct<'a>(file: &'a syn::File, name: &str) -> Option<&'a ItemStruct> {
    file.items.iter().find_map(|item| match item {
        Item::Struct(s) if s.ident == name => Some(s),
        _ => None,
    })
}

/// `Result<unsafe extern "C" fn(..), _>` -> the bare fn.
fn result_bare_fn(ty: &Type) -> Option<&TypeBareFn> {
    wrapped_bare_fn(ty, "Result")
}

/// `Option<unsafe extern "C" fn(..)>` -> the bare fn.
fn option_bare_fn(ty: &Type) -> Option<&TypeBareFn> {
    wrapped_bare_fn(ty, "Option")
}

fn wrapped_bare_fn<'a>(ty: &'a Type, wrapper: &str) -> Option<&'a TypeBareFn> {
    let Type::Path(p) = ty else { return None };
    let seg = p.path.segments.last()?;
    if seg.ident != wrapper {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &seg.arguments else { return None };
    match args.args.first() {
        Some(GenericArgument::Type(Type::BareFn(f))) => Some(f),
        _ => None,
    }
}

fn normalize_positive_enums(source: &str, enums: &[&str]) -> String {
    let mut out: String = source
        .lines()
        .map(|line| {
            let pinned = enums.iter().any(|e| line == format!("pub type {e} = ::std::os::raw::c_int;"));
            match line.split_once(" = ") {
                Some((lhs, _)) if pinned => format!("{lhs} = ::core::ffi::c_uint;"),
                _ => line.to_string(),
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    out.push('\n');
    out
}

fn find_header(dirs: &[PathBuf], name: &str) -> Option<PathBuf> {
    dirs.iter().map(|d| d.join(name)).find(|p| p.is_file())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Environment variables whose change must re-run generation.
fn rerun_vars(spec: &LibSpec) -> Vec<&'static str> {
    let mut vars = Vec::new();
    for sdk in spec.sdks {
        match sdk {
            Sdk::CudaToolkit => vars.extend(["CUDA_PATH", "CUDA_HOME", "CUDA_ROOT", "CUDA_TOOLKIT_ROOT_DIR"]),
            Sdk::Optix => vars.extend(["OPTIX_ROOT", "OptiX_INSTALL_DIR", "OPTIX_PATH"]),
            Sdk::EnvRoot(v) => vars.push(v),
        }
    }
    vars
}

/// The `include/` directory of every SDK in `spec`, located through vcv-rs.
fn sdk_include_dirs(spec: &LibSpec) -> Result<Vec<PathBuf>, Error> {
    let mut dirs = Vec::new();
    for sdk in spec.sdks {
        let dir = match sdk {
            Sdk::CudaToolkit => vcv_rs::detect_cuda().map(|c| c.include_dir()).ok_or_else(|| Error::SdkNotFound {
                lib: spec.name,
                sdk: "CUDA Toolkit".into(),
                hint: "set CUDA_PATH or install the toolkit".into(),
            })?,
            Sdk::Optix => vcv_rs::optix::detect_optix().map(|o| o.include_dir()).ok_or_else(|| Error::SdkNotFound {
                lib: spec.name,
                sdk: "OptiX SDK".into(),
                hint: "set OPTIX_ROOT or install the OptiX SDK".into(),
            })?,
            Sdk::EnvRoot(var) => std::env::var_os(var)
                .filter(|v| !v.is_empty())
                .map(|root| Path::new(&root).join("include"))
                .ok_or_else(|| Error::SdkNotFound { lib: spec.name, sdk: (*var).to_string(), hint: format!("set {var}") })?,
        };
        if !dirs.contains(&dir) {
            dirs.push(dir);
        }
    }
    Ok(dirs)
}

/// MSVC + Windows SDK + UCRT include directories from vcv-rs, so libclang finds `stddef.h` & co.
/// from any shell. Empty when no Visual Studio is installed (libclang then uses its own search).
fn system_include_dirs() -> Vec<PathBuf> {
    #[cfg(windows)]
    {
        let Some(vs) = vcv_rs::detect::detect_vs(None) else {
            return Vec::new();
        };
        let sdk = vcv_rs::detect::detect_sdk();
        let ucrt = vcv_rs::detect::detect_ucrt();
        vcv_rs::env::build_env(&vs, sdk.as_ref(), ucrt.as_ref(), vcv_rs::Arch::X64, vcv_rs::Arch::X64).include
    }
    #[cfg(not(windows))]
    {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FAKE: LibSpec = LibSpec {
        name: "fake",
        headers: &["fake.h"],
        sdks: &[],
        allow_fn: &["fake.*"],
        allow_type: &["fake.*"],
        allow_var: &["FAKE_.*"],
        reuse_cuda_types: &[],
        positive_enums: &["fakeStatus_enum"],
        shape: Shape::Exports {
            api_type: "FakeApi",
            loader_fn: "fake_api",
            not_loaded: "fakeStatus_enum_FAKE_ERROR_NOT_LOADED",
            missing_symbol: "fakeStatus_enum_FAKE_ERROR_MISSING_SYMBOL",
        },
    };

    fn fixture() -> Vec<PathBuf> {
        vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixture")]
    }

    #[test]
    fn exports_produce_shims_for_allowlisted_functions_only() {
        let out = bindings(&FAKE, &fixture(), &system_include_dirs()).expect("fixture binds");
        assert_eq!(out.inventory.symbols, ["fakeCtxCreate", "fakeGetCount", "fakeInit"]);
        let shims = out.shims.expect("exports have shims");
        for f in ["fakeInit", "fakeCtxCreate", "fakeGetCount"] {
            assert!(shims.contains(&format!("pub unsafe extern \"C\" fn {f}(")), "shim for {f}");
        }
        assert!(!shims.contains("notFakeHelper"), "allowlist must exclude other functions");
        // Mutation that reddens this: drop either error arm, or let a shim fall back to a panic.
        assert!(shims.contains("fakeStatus_enum_FAKE_ERROR_NOT_LOADED"));
        assert!(shims.contains("fakeStatus_enum_FAKE_ERROR_MISSING_SYMBOL"));
        assert!(!shims.contains("expect") && !shims.contains("unwrap") && !shims.contains("panic"));
    }

    #[test]
    fn types_exclude_functions_and_pin_positive_enums() {
        let out = bindings(&FAKE, &fixture(), &system_include_dirs()).expect("fixture binds");
        assert!(out.types.contains("pub type fakeStatus_enum = ::core::ffi::c_uint;"), "{}", out.types);
        assert!(out.types.contains("FAKE_VERSION"));
        assert!(!out.types.contains("pub fn fakeInit"), "types pass must not declare functions");
    }

    #[test]
    fn inventory_hashes_the_header_bytes() {
        let out = bindings(&FAKE, &fixture(), &system_include_dirs()).expect("fixture binds");
        let bytes = std::fs::read(fixture()[0].join("fake.h")).expect("fixture");
        assert_eq!(out.inventory.headers, vec![HeaderHash { name: "fake.h".into(), sha256: hex(&Sha256::digest(&bytes)) }]);
    }

    #[test]
    fn a_missing_header_names_every_searched_directory() {
        let spec = LibSpec { headers: &["absent.h"], ..FAKE };
        let err = bindings(&spec, &fixture(), &[]).expect_err("no such header");
        assert!(matches!(err, Error::HeaderNotFound { ref searched, .. } if searched == &fixture()), "{err}");
    }

    #[test]
    fn function_table_inventory_reads_option_fn_fields() {
        let src = r#"
            pub struct Table {
                pub fooCreate: Option<unsafe extern "C" fn(x: i32) -> i32>,
                pub barDestroy: Option<unsafe extern "C" fn()>,
                pub reserved: u64,
            }
        "#;
        assert_eq!(inventory("t", src, "Table").expect("parses"), ["barDestroy", "fooCreate"]);
    }
}
