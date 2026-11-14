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
//! [`device_asm`] is the device-side counterpart: it turns the inline-PTX statements of a device
//! header (OptiX) into `ptx_asm!` wrappers for cuda-oxide.
//!
//! **Used by** every `*-sys` crate's `build.rs` and `device/optix-device`. Generated headers are
//! never committed (PLAN D6).

pub use vcv_rs::nvsdk::Kind;

pub mod device_asm;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::ffi::OsStr;
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
    /// A separately installed SDK whose headers sit in `<root>/<subdir>` (Video Codec SDK:
    /// `Interface`).
    EnvRootSubdir {
        /// The variable naming the SDK root.
        var: &'static str,
        /// The header directory under the root.
        subdir: &'static str,
    },
    /// A separately shipped NVIDIA SDK located by vcv-rs (`vcv_rs::nvsdk`: root variables, the
    /// installers' standard folders, `NV_SDK_DIRS`). If it ships DLLs, their directory is passed
    /// to the sys crate as `env!("NV_RS_SDK_BIN")` for `nv_loader::Group::Sdk::hint`.
    Nv(vcv_rs::nvsdk::Kind),
    /// A header directory inside the CUDA Toolkit other than `include/` (CUPTI:
    /// `extras/CUPTI/include`), relative to the toolkit root vcv-rs located.
    CudaToolkitSubdir(&'static str),
    /// A header directory inside the calling crate (relative to `CARGO_MANIFEST_DIR`); used by the
    /// generator's own fixture crate.
    CrateDir(&'static str),
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
        /// Return-type-specific expressions used when loading or symbol lookup fails.
        returns: &'static [ReturnPolicy<'static>],
    },
    /// One library split over several DLLs (NPP: `nppc`, `nppial`, ... `npps`): shared types, and
    /// per DLL its own loading struct and shims. Each part takes the functions declared in its
    /// header files.
    SplitExports {
        /// One entry per DLL.
        parts: &'static [ExportPart],
        /// Return-type-specific expressions used when loading or symbol lookup fails.
        returns: &'static [ReturnPolicy<'static>],
    },
    /// The DLL exports one query function returning entry points by numeric id (NVAPI:
    /// `nvapi_QueryInterface(id)`, ids listed in `nvapi_interface.h`). Shims resolve each function
    /// through the query and fall back like [`Shape::Exports`] shims.
    QueryInterface {
        /// Header holding `{ "Name", 0x1234abcd }` id entries.
        interface_header: &'static str,
        /// Name of the sys crate's loader function; it returns the loaded query entry point
        /// (`Result<&'static nv_loader::QueryInterface, _>`).
        loader_fn: &'static str,
        /// Return-type-specific expressions used when loading or symbol lookup fails.
        returns: &'static [ReturnPolicy<'static>],
    },
    /// The SDK ships only static libraries (NGX: `nvsdk_ngx_d.lib`), so entry points are plain
    /// `extern "C"` declarations linked at build time against the SDK's `lib` directory (from
    /// vcv-rs). The one deliberate exception to "nothing is linked at build time" (PLAN D15).
    StaticLink {
        /// Static libraries to link (`"nvsdk_ngx_d"`).
        link_libs: &'static [&'static str],
        /// OS libraries the static library itself depends on (`"advapi32"`), linked dynamically.
        system_libs: &'static [&'static str],
    },
    /// Entry points come from a runtime query into a table struct (OptiX, NVENC); types only.
    FunctionTable {
        /// The table struct whose fields are the entry points (`"OptixFunctionTable"`).
        table_type: &'static str,
    },
}

/// One DLL of a [`Shape::SplitExports`] library.
#[derive(Clone, Copy, Debug)]
pub struct ExportPart {
    /// Part name, used in file names and as the module name in the sys crate (`"nppist"`).
    pub name: &'static str,
    /// Name of this part's loading struct (`"NppistApi"`).
    pub api_type: &'static str,
    /// Name of this part's loader function the shims call (`"nppist_api"`).
    pub loader_fn: &'static str,
    /// Header file names whose functions this DLL exports (`"nppi_statistics_functions.h"`).
    pub header_files: &'static [&'static str],
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
    /// Extra libclang arguments, for headers that need a macro neutralised (NVAPI's
    /// `__success(...)` SAL annotation: `-D__success(x)=`).
    pub clang_args: &'static [&'static str],
    /// Types emitted as opaque byte blobs of the C size and alignment, for layouts bindgen gets
    /// wrong (MSVC bitfield unions in NVAPI's `NV_TIMING_FLAG`, caught by the layout assertions).
    pub opaque_types: &'static [&'static str],
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
    /// The requested Rust target cannot select a native SDK architecture.
    #[error("{lib}: unsupported SDK target architecture {arch}")]
    UnsupportedTarget {
        /// The library.
        lib: &'static str,
        /// The requested architecture.
        arch: String,
    },
    /// A discovered path cannot be represented in Cargo or libclang arguments.
    #[error("{lib}: SDK path is not Unicode or contains a line break/NUL: {path:?}")]
    InvalidSdkPath {
        /// The library.
        lib: &'static str,
        /// The unusable path.
        path: PathBuf,
    },
    /// A static-only SDK has no native link-library directory for the selected target.
    #[error("{lib}: the SDK provides no native link-library directory for this target")]
    MissingLinkDirectory {
        /// The library.
        lib: &'static str,
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
    /// Loading struct + shims per DLL: one for [`Shape::Exports`], one per part for
    /// [`Shape::SplitExports`], none for [`Shape::FunctionTable`].
    pub parts: Vec<GeneratedPart>,
    /// Entry points and header hashes.
    pub inventory: Inventory,
}

/// The generated sources for one DLL.
#[derive(Debug)]
pub struct GeneratedPart {
    /// File-name infix: empty for a single-DLL library, `_<part>` for a split one.
    pub stem: String,
    /// The dynamic-loading struct (`None` for [`Shape::QueryInterface`], whose shims resolve
    /// through the query instead).
    pub api: Option<String>,
    /// The panic-free shims.
    pub shims: String,
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
    /// Entry points without a native C ABI shim. Functions without a native failure value
    /// receive a checked Rust-ABI function returning `Result`; query functions without an id
    /// remain unreachable. Only configured return policies and pointers retain their C ABI;
    /// `void` functions return checked `Result<(), nv_loader::CallError>` values.
    pub unshimmed: Vec<String>,
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
    // One discovery snapshot supplies both the headers and their matching native libraries.
    let SdkPaths { includes: include_dirs, nvidia } = sdk_paths(spec)?;
    let runtime = match nvidia.iter().find_map(|sdk| sdk.runtime_dir.as_deref()) {
        Some(path) => sdk_path_str(spec.name, path)?,
        None => "",
    };
    println!("cargo:rustc-env=NV_RS_SDK_BIN={runtime}");
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
    if let Shape::StaticLink { link_libs, system_libs } = spec.shape {
        for l in system_libs {
            println!("cargo:rustc-link-lib=dylib={l}");
        }
        if !link_libs.is_empty() && nvidia.iter().all(|sdk| sdk.lib_dir.is_none()) {
            return Err(Error::MissingLinkDirectory { lib: spec.name });
        }
        for sdk in &nvidia {
            if let Some(lib_dir) = &sdk.lib_dir {
                println!("cargo:rustc-link-search=native={}", sdk_path_str(spec.name, lib_dir)?);
            }
        }
        for l in link_libs {
            println!("cargo:rustc-link-lib=static={l}");
        }
    }
    write("types.rs", &out.types)?;
    for part in &out.parts {
        // `<lib>_api.rs` for one DLL, `<lib>_<part>_api.rs` per DLL of a split library.
        let stem = part.stem.trim_start_matches('_');
        let prefix = if stem.is_empty() { String::new() } else { format!("{stem}_") };
        if let Some(api) = &part.api {
            write(&format!("{prefix}api.rs"), api)?;
        }
        write(&format!("{prefix}shims.rs"), &part.shims)?;
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

/// The types and constants of `spec` (its `allow_type` / `allow_var`) as `no_std` Rust (`core::`
/// paths, no layout tests, `Default` derived), for device crates compiled by cuda-oxide: the
/// same header-derived definitions the host sys crate uses, so host and device agree on layouts
/// and enum values without hand-copied constants. Headers are located as in [`generate`].
pub fn core_types(spec: &LibSpec) -> Result<String, Error> {
    for var in rerun_vars(spec) {
        println!("cargo:rerun-if-env-changed={var}");
    }
    let include_dirs = sdk_paths(spec)?.includes;
    let mut wrapper = String::new();
    for name in spec.headers {
        let path = find_header(&include_dirs, name).ok_or_else(|| Error::HeaderNotFound {
            lib: spec.name,
            header: (*name).to_string(),
            searched: include_dirs.clone(),
        })?;
        println!("cargo:rerun-if-changed={}", path.display());
        wrapper.push_str(&format!("#include <{name}>\n"));
    }
    let mut b = bindgen::Builder::default()
        .header_contents(&format!("nv_bindgen_{}_core.h", spec.name), &wrapper)
        .default_enum_style(bindgen::EnumVariation::Consts)
        .generate_comments(false)
        .use_core()
        .layout_tests(false)
        .derive_default(true)
        .with_codegen_config(bindgen::CodegenConfig::TYPES | bindgen::CodegenConfig::VARS);
    for d in &include_dirs {
        b = b.clang_arg(format!("-I{}", d.display()));
    }
    for d in system_include_dirs() {
        b = b.clang_arg(format!("-idirafter{}", sdk_path_str(spec.name, &d)?));
    }
    for a in spec.clang_args {
        b = b.clang_arg(*a);
    }
    for t in spec.allow_type {
        b = b.allowlist_type(*t);
    }
    for v in spec.allow_var {
        b = b.allowlist_var(*v);
    }
    b.generate().map(|g| g.to_string()).map_err(|e| Error::Bindgen { lib: spec.name, msg: e.to_string() })
}

/// Runs bindgen for `spec` against explicit include directories (`include_dirs` searched first,
/// `system_dirs` after libclang's own). Pure apart from reading headers.
pub fn bindings(spec: &LibSpec, include_dirs: &[PathBuf], system_dirs: &[PathBuf]) -> Result<Output, Error> {
    for path in include_dirs.iter().chain(system_dirs) {
        sdk_path_str(spec.name, path)?;
    }
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
        for a in spec.clang_args {
            b = b.clang_arg(*a);
        }
        for t in spec.opaque_types {
            b = b.opaque_type(*t);
        }
        b
    };
    let run = |b: bindgen::Builder| b.generate().map(|g| g.to_string()).map_err(|e| Error::Bindgen { lib: spec.name, msg: e.to_string() });

    // Functions pass first (exports only): its signatures name the types the types pass must
    // also emit. bindgen traces dependencies only of items it generates, so a function left out
    // of codegen would not pull in `cudaStream_t` / `libraryPropertyType`; they are allowlisted
    // by exact name instead.
    // (stem, api_type, loader_fn, header files or empty for "all allowlisted functions")
    let plan: Vec<(String, &str, &str, &[&str])> = match spec.shape {
        Shape::Exports { api_type, loader_fn, .. } => vec![(String::new(), api_type, loader_fn, &[])],
        // The signatures come from a throwaway dynamic-loading struct; the shims call the query.
        Shape::QueryInterface { loader_fn, .. } => vec![(String::new(), QI_SIGNATURES, loader_fn, &[])],
        Shape::SplitExports { parts, .. } => {
            parts.iter().map(|p| (format!("_{}", p.name), p.api_type, p.loader_fn, p.header_files)).collect()
        }
        Shape::FunctionTable { .. } | Shape::StaticLink { .. } => Vec::new(),
    };
    let mut apis: Vec<(String, &str, &str, String)> = Vec::new();
    for (stem, api_type, loader_fn, files) in plan {
        let mut ab = base()
            .dynamic_library_name(api_type)
            .dynamic_link_require_all(false)
            .with_codegen_config(bindgen::CodegenConfig::FUNCTIONS);
        // bindgen allowlists are a union, so a split part uses only its own headers (which
        // restricts it to the functions declared there); a single-DLL library uses `allow_fn`.
        if files.is_empty() {
            for f in spec.allow_fn {
                ab = ab.allowlist_function(*f);
            }
        }
        for file in files {
            ab = ab.allowlist_file(format!(r".*[/\\]{}$", regex_escape(file)));
        }
        apis.push((stem, api_type, loader_fn, run(ab)?));
    }
    // Statically linked libraries: plain `extern "C"` declarations instead of a loading struct.
    let externs = match spec.shape {
        Shape::StaticLink { .. } => {
            let mut eb = base().with_codegen_config(bindgen::CodegenConfig::FUNCTIONS);
            for f in spec.allow_fn {
                eb = eb.allowlist_function(*f);
            }
            Some(run(eb)?)
        }
        _ => None,
    };
    let mut signature_types_all = Vec::new();
    for (_, api_type, _, src) in &apis {
        signature_types_all.extend(signature_types(spec.name, src, api_type)?);
    }
    if let Some(src) = &externs {
        signature_types_all.extend(extern_fns(spec.name, src)?.1);
    }
    signature_types_all.sort();
    signature_types_all.dedup();
    let signature_types = signature_types_all;

    // Types pass: the allowlisted types and constants plus every type an entry point names.
    let mut tb = base().derive_default(true).blocklist_function(".*");
    for t in &signature_types {
        tb = tb.allowlist_type(format!("^{t}$"));
    }
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

    let mut parts = Vec::new();
    let mut symbols = Vec::new();
    let mut unshimmed = Vec::new();
    match spec.shape {
        Shape::Exports { returns, .. } | Shape::SplitExports { returns, .. } => {
            for (stem, api_type, loader_fn, api) in apis {
                let generated = shims(spec.name, &api, api_type, loader_fn, returns)?;
                symbols.extend(inventory(spec.name, &api, api_type)?);
                unshimmed.extend(generated.unshimmed);
                parts.push(GeneratedPart { stem, api: Some(api), shims: generated.source });
            }
        }
        Shape::QueryInterface { interface_header, returns, .. } => {
            let path = find_header(include_dirs, interface_header).ok_or_else(|| Error::HeaderNotFound {
                lib: spec.name,
                header: interface_header.to_string(),
                searched: include_dirs.to_vec(),
            })?;
            let text = std::fs::read_to_string(&path).map_err(|source| Error::Io { lib: spec.name, path, source })?;
            let ids = interface_ids(&text);
            for (stem, _, loader_fn, api) in apis {
                let generated = query_shims(spec.name, &api, loader_fn, &ids, returns)?;
                symbols.extend(generated.bound);
                unshimmed.extend(generated.unshimmed);
                parts.push(GeneratedPart { stem, api: None, shims: generated.source });
            }
        }
        Shape::FunctionTable { table_type } => symbols = inventory(spec.name, &types, table_type)?,
        Shape::StaticLink { .. } => {
            let src = externs.unwrap_or_default();
            symbols = extern_fns(spec.name, &src)?.0;
            // The `extern` block takes the place of shims: nothing is loaded at runtime.
            parts.push(GeneratedPart { stem: String::new(), api: None, shims: src });
        }
    }
    symbols.sort();
    symbols.dedup();
    unshimmed.sort();

    Ok(Output { types, parts, inventory: Inventory { lib: spec.name.to_string(), headers, symbols, unshimmed } })
}

/// Names of the functions declared in `extern` blocks of `source`, and every single-segment type
/// their signatures name (for the types pass).
fn extern_fns(lib: &'static str, source: &str) -> Result<(Vec<String>, Vec<String>), Error> {
    let file = syn::parse_file(source).map_err(|e| Error::Generated { lib, msg: e.to_string() })?;
    let mut names = Vec::new();
    let mut types = Vec::new();
    for item in &file.items {
        let Item::ForeignMod(block) = item else { continue };
        for f in &block.items {
            let syn::ForeignItem::Fn(f) = f else { continue };
            names.push(f.sig.ident.to_string());
            for arg in &f.sig.inputs {
                if let syn::FnArg::Typed(t) = arg {
                    collect_type_names(&t.ty, &mut types);
                }
            }
            if let ReturnType::Type(_, t) = &f.sig.output {
                collect_type_names(t, &mut types);
            }
        }
    }
    names.sort();
    Ok((names, types))
}

/// Internal name of the signature-only struct used by [`Shape::QueryInterface`].
const QI_SIGNATURES: &str = "NvBindgenQiSignatures";

/// `{ "Name", 0x1234abcd }` entries of a query-interface header, in header order.
pub fn interface_ids(text: &str) -> Vec<(String, u32)> {
    text.lines()
        .filter_map(|line| {
            let rest = line.trim().strip_prefix('{')?.trim_start().strip_prefix('"')?;
            let (name, rest) = rest.split_once('"')?;
            let hex = rest.trim_start().strip_prefix(',')?.trim().trim_end_matches(['}', ',', ' ']).trim();
            let id = u32::from_str_radix(hex.strip_prefix("0x").or_else(|| hex.strip_prefix("0X"))?, 16).ok()?;
            Some((name.to_string(), id))
        })
        .collect()
}

/// Query-interface shims plus what they cover.
#[derive(Debug)]
pub struct QueryShims {
    /// The generated function source.
    pub source: String,
    /// Reachable functions (declared in the headers and listed with an id), sorted.
    pub bound: Vec<String>,
    /// Functions without a native C ABI shim: no id or no native failure value.
    pub unshimmed: Vec<String>,
}

/// Functions that resolve each declared entry point through the library's query.
///
/// Configured return policies and pointers preserve the native C ABI. Other known-id
/// functions use their original name with a Rust ABI and return `Result<T, nv_loader::CallError>`.
/// Functions absent from the id table cannot be reached and remain only in `unshimmed`.
pub fn query_shims(
    lib: &'static str,
    signatures_source: &str,
    loader_fn: &str,
    ids: &[(String, u32)],
    returns: &[ReturnPolicy<'_>],
) -> Result<QueryShims, Error> {
    let gen_err = |msg: String| Error::Generated { lib, msg };
    let file = syn::parse_file(signatures_source).map_err(|e| gen_err(e.to_string()))?;
    let sigs = find_struct(&file, QI_SIGNATURES).ok_or_else(|| gen_err(format!("no struct {QI_SIGNATURES}")))?;
    let loader = format_ident!("{loader_fn}");
    let policies = parse_return_policies(lib, returns)?;
    let Fields::Named(fields) = &sigs.fields else {
        return Err(gen_err(format!("{QI_SIGNATURES} has no named fields")));
    };

    let mut bound = Vec::new();
    let mut unshimmed = Vec::new();
    let mut fns: Vec<TokenStream> = Vec::new();
    for field in &fields.named {
        let (Some(name), Some(bare)) = (field.ident.as_ref(), result_bare_fn(&field.ty)) else { continue };
        let Some(id) = ids.iter().find(|(n, _)| n == &name.to_string()).map(|(_, id)| *id) else {
            unshimmed.push(name.to_string());
            continue;
        };
        let fb = fallback(&bare.output, &policies);
        if fb.is_none() {
            unshimmed.push(name.to_string());
        }
        let plan = wrapper_plan(&bare.output, fb, lib, name, quote!(None));
        let WrapperPlan { abi, ret, on_missing, on_not_loaded, checked } = plan;
        let args = shim_args(bare);
        let names = args.iter().map(|(n, _)| n);
        let defs = args.iter().map(|(_, d)| d);
        let call = quote!(unsafe { __nv_fn(#(#names),*) });
        let result = if checked { quote!(::core::result::Result::Ok(#call)) } else { call };
        let fn_ty = bare;
        let id_lit = proc_macro2::Literal::u32_suffixed(id);
        fns.push(quote! {
            #[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
            #[inline]
            pub unsafe #abi fn #name(#(#defs),*) #ret {
                // SAFETY: the header declares this exact id's signature and the query's library
                // remains loaded for the process; the caller upholds the native call contract.
                match #loader() {
                    Ok(__nv_qi) => {
                        let __nv_ptr = unsafe { (__nv_qi.query)(#id_lit) };
                        if __nv_ptr.is_null() {
                            #on_missing
                        } else {
                            let __nv_fn = unsafe { ::core::mem::transmute::<*mut ::core::ffi::c_void, #fn_ty>(__nv_ptr) };
                            #result
                        }
                    }
                    Err(__nv_error) => #on_not_loaded,
                }
            }
        });
        bound.push(name.to_string());
    }
    let parsed: syn::File = syn::parse2(quote!(#(#fns)*)).map_err(|e| gen_err(format!("shims: {e}")))?;
    bound.sort();
    unshimmed.sort();
    Ok(QueryShims { source: prettyplease::unparse(&parsed), bound, unshimmed })
}

/// Escapes regex metacharacters in a file name (`.` in `nppcore.h`).
fn regex_escape(s: &str) -> String {
    s.chars().fold(String::new(), |mut out, c| {
        if r"\.+*?()|[]{}^$".contains(c) {
            out.push('\\');
        }
        out.push(c);
        out
    })
}

/// How one or more native return types report unavailable-library and missing-symbol errors.
#[derive(Clone, Copy, Debug)]
pub struct ReturnPolicy<'a> {
    /// Exact final type identifiers as they appear in generated signatures.
    pub types: &'a [&'a str],
    /// Rust expression returned when the library could not be loaded.
    pub not_loaded: &'a str,
    /// Rust expression returned when the symbol is missing.
    pub missing_symbol: &'a str,
}

struct ParsedReturnPolicy<'a> {
    types: &'a [&'a str],
    not_loaded: Expr,
    missing_symbol: Expr,
}

/// Validate and parse once, before visiting any entry points.
fn parse_return_policies<'a>(lib: &'static str, returns: &[ReturnPolicy<'a>]) -> Result<Vec<ParsedReturnPolicy<'a>>, Error> {
    let gen_err = |msg: String| Error::Generated { lib, msg };
    let mut names = Vec::new();
    let mut parsed = Vec::with_capacity(returns.len());
    for policy in returns {
        if policy.types.is_empty() {
            return Err(gen_err("return policy has no types".into()));
        }
        for name in policy.types {
            syn::parse_str::<proc_macro2::Ident>(name).map_err(|e| gen_err(format!("return type {name:?}: {e}")))?;
            if names.contains(name) {
                return Err(gen_err(format!("duplicate return policy for {name}")));
            }
            names.push(*name);
        }
        let not_loaded = syn::parse_str(policy.not_loaded).map_err(|e| gen_err(format!("not_loaded: {e}")))?;
        let missing_symbol = syn::parse_str(policy.missing_symbol).map_err(|e| gen_err(format!("missing_symbol: {e}")))?;
        parsed.push(ParsedReturnPolicy { types: policy.types, not_loaded, missing_symbol });
    }
    Ok(parsed)
}

/// Generated function source plus entries without native C ABI shims.
#[derive(Debug)]
pub struct Shims {
    /// Native shims and checked Rust-ABI functions.
    pub source: String,
    /// Sorted names of functions without a native failure value (see [`Inventory::unshimmed`]).
    pub unshimmed: Vec<String>,
}

/// Native failure values, when the return contract defines one.
enum Fallback<'a> {
    /// Expressions from the matching policy.
    Policy { not_loaded: &'a Expr, missing_symbol: &'a Expr },
    /// A null pointer (`*const` / `*mut`).
    Null { mutable: bool },
}

fn fallback<'a>(ret: &ReturnType, policies: &'a [ParsedReturnPolicy<'_>]) -> Option<Fallback<'a>> {
    match ret {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => match ty.as_ref() {
            Type::Ptr(p) => Some(Fallback::Null { mutable: p.mutability.is_some() }),
            Type::Path(p) => {
                let last = p.path.segments.last()?.ident.to_string();
                policies.iter().find(|policy| policy.types.contains(&last.as_str())).map(|policy| {
                    Fallback::Policy { not_loaded: &policy.not_loaded, missing_symbol: &policy.missing_symbol }
                })
            }
            _ => None,
        },
    }
}

struct WrapperPlan {
    abi: TokenStream,
    ret: TokenStream,
    on_missing: TokenStream,
    on_not_loaded: TokenStream,
    checked: bool,
}

/// The same ABI and failure-value decision for exports and query-interface functions.
fn wrapper_plan(ret: &ReturnType, fb: Option<Fallback<'_>>, lib: &'static str, name: &proc_macro2::Ident, source: TokenStream) -> WrapperPlan {
    let native_ret = match ret {
        ReturnType::Default => quote!(),
        ReturnType::Type(_, ty) => quote!(-> #ty),
    };
    let (on_missing, on_not_loaded) = match fb {
        Some(Fallback::Policy { not_loaded, missing_symbol }) => (quote!(#missing_symbol), quote!(#not_loaded)),
        Some(Fallback::Null { mutable: false }) => (quote!(::core::ptr::null()), quote!(::core::ptr::null())),
        Some(Fallback::Null { mutable: true }) => (quote!(::core::ptr::null_mut()), quote!(::core::ptr::null_mut())),
        None => {
            let ty = match ret {
                ReturnType::Default => quote!(()),
                ReturnType::Type(_, ty) => quote!(#ty),
            };
            return WrapperPlan {
                abi: quote!(),
                ret: quote!(-> ::core::result::Result<#ty, nv_loader::CallError>),
                on_missing: quote!(::core::result::Result::Err(nv_loader::CallError::symbol(
                    #lib, stringify!(#name), #source,
                ))),
                on_not_loaded: quote!(::core::result::Result::Err(nv_loader::CallError::Load(__nv_error))),
                checked: true,
            };
        }
    };
    WrapperPlan { abi: quote!(extern "C"), ret: native_ret, on_missing, on_not_loaded, checked: false }
}

/// Keep C parameter names intact; unnamed parameters receive deterministic private names.
fn shim_args(bare: &TypeBareFn) -> Vec<(proc_macro2::Ident, TokenStream)> {
    bare.inputs.iter().enumerate().map(|(i, arg)| {
        let name = arg.name.as_ref().map(|(id, _)| id.clone()).unwrap_or_else(|| format_ident!("arg_{i}"));
        let ty = &arg.ty;
        (name.clone(), quote!(#name: #ty))
    }).collect()
}

/// Panic-free functions for the function-pointer fields of `api_type`.
///
/// Policies and pointers preserve the native C ABI. Functions without a native failure
/// value use their original name with a Rust ABI and return `Result<T, nv_loader::CallError>`.
/// They remain in [`Shims::unshimmed`] because no native C ABI shim exists.
pub fn shims(lib: &'static str, api_source: &str, api_type: &str, loader_fn: &str, returns: &[ReturnPolicy<'_>]) -> Result<Shims, Error> {
    let gen_err = |msg: String| Error::Generated { lib, msg };
    let file = syn::parse_file(api_source).map_err(|e| gen_err(e.to_string()))?;
    let api = find_struct(&file, api_type).ok_or_else(|| gen_err(format!("no struct {api_type}")))?;
    let loader = format_ident!("{loader_fn}");
    let policies = parse_return_policies(lib, returns)?;

    let Fields::Named(fields) = &api.fields else {
        return Err(gen_err(format!("{api_type} has no named fields")));
    };
    let mut unshimmed = Vec::new();
    let fns: Vec<TokenStream> = fields.named.iter().filter_map(|field| {
        let name = field.ident.as_ref()?;
        let bare = result_bare_fn(&field.ty)?;
        let fb = fallback(&bare.output, &policies);
        if fb.is_none() {
            unshimmed.push(name.to_string());
        }
        let plan = wrapper_plan(&bare.output, fb, lib, name, quote!(Some(__nv_error)));
        let WrapperPlan { abi, ret, on_missing, on_not_loaded, checked } = plan;
        let args = shim_args(bare);
        let names = args.iter().map(|(n, _)| n);
        let defs = args.iter().map(|(_, d)| d);
        let call = quote!(unsafe { (*__nv_fn)(#(#names),*) });
        let result = if checked { quote!(::core::result::Result::Ok(#call)) } else { call };
        Some(quote! {
            #[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
            #[inline]
            pub unsafe #abi fn #name(#(#defs),*) #ret {
                match #loader() {
                    // SAFETY: the caller upholds the native contract and the pointer's library
                    // remains loaded for the process. Private names avoid C parameter shadowing.
                    Ok(__nv_api) => match &__nv_api.#name {
                        Ok(__nv_fn) => #result,
                        Err(__nv_error) => #on_missing,
                    },
                    Err(__nv_error) => #on_not_loaded,
                }
            }
        })
    }).collect();
    let tokens = quote!(#(#fns)*);
    let parsed: syn::File = syn::parse2(tokens).map_err(|e| gen_err(format!("shims: {e}")))?;
    unshimmed.sort();
    Ok(Shims { source: prettyplease::unparse(&parsed), unshimmed })
}

/// Sorted entry-point names: the function-pointer fields of `struct_name` in `source` (bindgen's
/// dynamic-loading struct wraps them in `Result<fn, _>`, a function table holds `Option<fn>`).
pub fn inventory(lib: &'static str, source: &str, struct_name: &str) -> Result<Vec<String>, Error> {
    let file = syn::parse_file(source).map_err(|e| Error::Generated { lib, msg: e.to_string() })?;
    let s = find_struct(&file, struct_name).ok_or_else(|| Error::Generated { lib, msg: format!("no struct {struct_name}") })?;
    let Fields::Named(fields) = &s.fields else {
        return Ok(Vec::new());
    };
    // Tables often name their fields through typedef'd pointer types (NVENC:
    // `PNVENCOPENENCODESESSIONEX nvEncOpenEncodeSessionEx;`), which bindgen emits as
    // `pub type PNVENC... = Option<unsafe extern "C" fn(..)>`; resolve those aliases.
    let fn_aliases: Vec<String> = file
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Type(t) if option_bare_fn(&t.ty).is_some() || result_bare_fn(&t.ty).is_some() => Some(t.ident.to_string()),
            _ => None,
        })
        .collect();
    let is_fn_alias = |ty: &Type| match ty {
        Type::Path(p) if p.path.segments.len() == 1 => fn_aliases.iter().any(|a| p.path.segments[0].ident == a.as_str()),
        _ => false,
    };
    let mut names: Vec<String> = fields
        .named
        .iter()
        .filter(|f| result_bare_fn(&f.ty).is_some() || option_bare_fn(&f.ty).is_some() || is_fn_alias(&f.ty))
        .filter_map(|f| f.ident.as_ref().map(ToString::to_string))
        .collect();
    names.sort();
    Ok(names)
}

/// Every single-segment type name used in the entry-point signatures of `api_type` (bindgen's
/// own aliases such as `cudaStream_t`; `::std::os::raw::*` paths have several segments and are
/// skipped), sorted and deduplicated.
pub fn signature_types(lib: &'static str, api_source: &str, api_type: &str) -> Result<Vec<String>, Error> {
    let file = syn::parse_file(api_source).map_err(|e| Error::Generated { lib, msg: e.to_string() })?;
    let s = find_struct(&file, api_type).ok_or_else(|| Error::Generated { lib, msg: format!("no struct {api_type}") })?;
    let mut names = Vec::new();
    if let Fields::Named(fields) = &s.fields {
        for f in fields.named.iter().filter_map(|f| result_bare_fn(&f.ty)) {
            for arg in &f.inputs {
                collect_type_names(&arg.ty, &mut names);
            }
            if let ReturnType::Type(_, ty) = &f.output {
                collect_type_names(ty, &mut names);
            }
        }
    }
    names.sort();
    names.dedup();
    Ok(names)
}

fn collect_type_names(ty: &Type, out: &mut Vec<String>) {
    match ty {
        Type::Path(p) => {
            if p.qself.is_none() && p.path.leading_colon.is_none() && p.path.segments.len() == 1 {
                out.push(p.path.segments[0].ident.to_string());
            }
            for seg in &p.path.segments {
                if let PathArguments::AngleBracketed(args) = &seg.arguments {
                    for a in &args.args {
                        if let GenericArgument::Type(t) = a {
                            collect_type_names(t, out);
                        }
                    }
                }
            }
        }
        Type::Ptr(p) => collect_type_names(&p.elem, out),
        Type::Reference(r) => collect_type_names(&r.elem, out),
        Type::Array(a) => collect_type_names(&a.elem, out),
        Type::Paren(p) => collect_type_names(&p.elem, out),
        Type::BareFn(f) => {
            for arg in &f.inputs {
                collect_type_names(&arg.ty, out);
            }
            if let ReturnType::Type(_, t) = &f.output {
                collect_type_names(t, out);
            }
        }
        _ => {}
    }
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
            Sdk::CudaToolkit | Sdk::CudaToolkitSubdir(_) => {
                vars.extend(vcv_rs::cuda::root_vars());
                vars.extend(["PATH", "ProgramFiles"]);
            }
            Sdk::Optix => {
                vars.extend(vcv_rs::optix::root_vars());
                vars.push("ProgramData");
            }
            Sdk::EnvRoot(v) | Sdk::EnvRootSubdir { var: v, .. } => vars.push(v),
            Sdk::Nv(kind) => {
                vars.extend(kind.root_vars());
                vars.extend(vcv_rs::cuda::root_vars());
                vars.extend(["NV_SDK_DIRS", "PATH", "ProgramFiles", "ProgramData"]);
            }
            Sdk::CrateDir(_) => {}
        }
    }
    vars
}

/// A single discovery result keeps a library's headers and runtime/link paths together.
struct SdkPaths {
    includes: Vec<PathBuf>,
    nvidia: Vec<vcv_rs::nvsdk::SdkInfo>,
}

fn sdk_path_str<'a>(lib: &'static str, path: &'a Path) -> Result<&'a str, Error> {
    match path.to_str() {
        Some(text) if !text.chars().any(|c| matches!(c, '\0' | '\r' | '\n')) => Ok(text),
        _ => Err(Error::InvalidSdkPath { lib, path: path.to_path_buf() }),
    }
}

fn sdk_target_arch(lib: &'static str, requested: Option<&OsStr>) -> Result<vcv_rs::Arch, Error> {
    match requested {
        Some(value) => value.to_str().and_then(vcv_rs::Arch::from_target_arch)
            .ok_or_else(|| Error::UnsupportedTarget { lib, arch: value.to_string_lossy().into_owned() }),
        None => vcv_rs::Arch::native()
            .ok_or_else(|| Error::UnsupportedTarget { lib, arch: std::env::consts::ARCH.to_string() }),
    }
}

/// Locate every SDK once; consumers must not rediscover a different runtime after binding.
fn sdk_paths(spec: &LibSpec) -> Result<SdkPaths, Error> {
    let mut dirs = Vec::new();
    let mut nvidia = Vec::new();
    let cargo_arch = std::env::var_os("CARGO_CFG_TARGET_ARCH");
    for sdk in spec.sdks {
        let dir = match sdk {
            Sdk::CudaToolkit => vcv_rs::detect_cuda().map(|c| c.include_dir()).ok_or_else(|| Error::SdkNotFound {
                lib: spec.name,
                sdk: "CUDA Toolkit".into(),
                hint: "set CUDA_PATH or install the toolkit".into(),
            })?,
            Sdk::Nv(kind) => {
                let target = sdk_target_arch(spec.name, cargo_arch.as_deref())?;
                let info = vcv_rs::nvsdk::detect(*kind, target).ok_or_else(|| Error::SdkNotFound {
                    lib: spec.name,
                    sdk: kind.name().into(),
                    hint: match kind.requested_root() {
                        Some(request) => format!("configured {}={:?} is not usable for {target}", request.variable, request.path),
                        None => format!("check {:?} or NV_SDK_DIRS for a native SDK on {}", kind.root_vars(), std::env::consts::OS),
                    },
                })?;
                for path in [&info.runtime_dir, &info.lib_dir].into_iter().flatten() {
                    sdk_path_str(spec.name, path)?;
                }
                let include = info.include_dir.clone();
                nvidia.push(info);
                include
            },
            Sdk::CudaToolkitSubdir(rel) => vcv_rs::detect_cuda().map(|c| c.root.join(rel)).ok_or_else(|| Error::SdkNotFound {
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
            Sdk::EnvRootSubdir { var, subdir } => std::env::var_os(var)
                .filter(|v| !v.is_empty())
                .map(|root| Path::new(&root).join(subdir))
                .ok_or_else(|| Error::SdkNotFound { lib: spec.name, sdk: (*var).to_string(), hint: format!("set {var}") })?,
            Sdk::CrateDir(rel) => std::env::var_os("CARGO_MANIFEST_DIR")
                .map(|m| Path::new(&m).join(rel))
                .ok_or_else(|| Error::SdkNotFound { lib: spec.name, sdk: (*rel).to_string(), hint: "CARGO_MANIFEST_DIR unset".into() })?,
        };
        sdk_path_str(spec.name, &dir)?;
        if !dirs.contains(&dir) {
            dirs.push(dir);
        }
    }
    Ok(SdkPaths { includes: dirs, nvidia })
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
        clang_args: &[],
        opaque_types: &[],
        shape: Shape::Exports {
            api_type: "FakeApi",
            loader_fn: "fake_api",
            returns: &[
                ReturnPolicy {
                    types: &["fakeStatus"],
                    not_loaded: "fakeStatus_enum_FAKE_ERROR_NOT_LOADED",
                    missing_symbol: "fakeStatus_enum_FAKE_ERROR_MISSING_SYMBOL",
                },
                ReturnPolicy {
                    types: &["fakeIoStatus"],
                    not_loaded: "fakeIoStatus { err: FAKE_IO_NOT_LOADED as i32, cuda: 0 }",
                    missing_symbol: "fakeIoStatus { err: FAKE_IO_MISSING as i32, cuda: 0 }",
                },
                ReturnPolicy {
                    types: &["fakeCount"],
                    not_loaded: "-(FAKE_IO_NOT_LOADED as fakeCount)",
                    missing_symbol: "-(FAKE_IO_MISSING as fakeCount)",
                },
            ],
        },
    };

    fn fixture() -> Vec<PathBuf> {
        vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixture")]
    }

    #[test]
    fn bindings_reject_paths_before_clang_or_header_lookup() {
        for bad in ["SDK\ninclude", "SDK\rinclude", "SDK\0include"] {
            let paths = [PathBuf::from(bad)];
            for (includes, system) in [(paths.as_slice(), &[][..]), (&[][..], paths.as_slice())] {
                assert!(matches!(bindings(&FAKE, includes, system), Err(Error::InvalidSdkPath { .. })));
            }
        }
    }

    #[test]
    fn sdk_rebuild_inputs_follow_discovery_aliases() {
        let spec = LibSpec {
            sdks: &[Sdk::CudaToolkit, Sdk::Optix, Sdk::Nv(Kind::Nvcomp)],
            ..FAKE
        };
        let vars = rerun_vars(&spec);
        for input in vcv_rs::cuda::root_vars().iter()
            .chain(vcv_rs::optix::root_vars())
            .chain(Kind::Nvcomp.root_vars())
            .copied()
            .chain(["NV_SDK_DIRS", "PATH", "ProgramFiles", "ProgramData"])
        {
            assert!(vars.contains(&input), "discovery input {input} must rerun generation");
        }
    }

    #[test]
    fn sdk_target_selection_is_explicit_and_checked() {
        for (name, expected) in [
            ("x86_64", vcv_rs::Arch::X64),
            ("x86", vcv_rs::Arch::X86),
            ("aarch64", vcv_rs::Arch::Arm64),
        ] {
            assert_eq!(sdk_target_arch("fake", Some(OsStr::new(name))).expect("supported target"), expected);
        }
        assert!(matches!(sdk_target_arch("fake", Some(OsStr::new("riscv64"))), Err(Error::UnsupportedTarget { .. })));
        assert_eq!(sdk_target_arch("fake", None).expect("native target"), vcv_rs::Arch::native().expect("supported test host"));
    }

    #[test]
    fn sdk_paths_cannot_inject_cargo_instructions() {
        assert_eq!(sdk_path_str("fake", Path::new("SDK with spaces/include")).expect("ordinary path"), "SDK with spaces/include");
        for bad in ["SDK\ninclude", "SDK\rinclude", "SDK\0include"] {
            assert!(matches!(sdk_path_str("fake", Path::new(bad)), Err(Error::InvalidSdkPath { .. })));
        }
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            assert!(matches!(sdk_path_str("fake", Path::new(OsStr::from_bytes(b"SDK\xff/include"))), Err(Error::InvalidSdkPath { .. })));
        }
    }

    #[test]
    fn exports_produce_shims_for_allowlisted_functions_only() {
        let out = bindings(&FAKE, &fixture(), &system_include_dirs()).expect("fixture binds");
        assert_eq!(
            out.inventory.symbols,
            ["fakeCtxCreate", "fakeErrorString", "fakeGetCount", "fakeInit", "fakeIo", "fakeMissingNumber", "fakeReset", "fakeShadow", "fakeStore", "fakeTransfer", "fakeVersionNumber"]
        );
        // These have checked Rust wrappers, never native C shims with made-up return values.
        assert_eq!(out.inventory.unshimmed, ["fakeMissingNumber", "fakeReset", "fakeStore", "fakeVersionNumber"]);
        assert_eq!(out.parts.len(), 1, "a single-DLL library has one part");
        let shims = &out.parts[0].shims;
        for f in ["fakeInit", "fakeCtxCreate", "fakeGetCount", "fakeErrorString"] {
            assert!(shims.contains(&format!("pub unsafe extern \"C\" fn {f}(")), "shim for {f}");
        }
        assert!(shims.contains("pub unsafe fn fakeVersionNumber("));
        assert!(shims.contains("pub unsafe fn fakeReset("));
        assert!(shims.contains("pub unsafe fn fakeStore("));
        assert!(!shims.contains("pub unsafe extern \"C\" fn fakeVersionNumber("));
        // A pointer return falls back to null, not to a status code of the wrong type.
        assert!(shims.contains("::core::ptr::null()"), "{shims}");
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
    fn split_exports_give_each_dll_only_its_own_headers_functions() {
        const PARTS: &[ExportPart] = &[
            ExportPart { name: "a", api_type: "SplitAApi", loader_fn: "split_a_api", header_files: &["split_a.h"] },
            ExportPart { name: "b", api_type: "SplitBApi", loader_fn: "split_b_api", header_files: &["split_b.h"] },
        ];
        let spec = LibSpec {
            name: "split",
            headers: &["split.h"],
            allow_fn: &["^split.*"],
            allow_type: &["^split.*"],
            allow_var: &[],
            positive_enums: &[],
            shape: Shape::SplitExports {
                parts: PARTS,
                returns: &[ReturnPolicy {
                    types: &["splitStatus"],
                    not_loaded: "splitStatus_enum_SPLIT_NOT_LOADED",
                    missing_symbol: "splitStatus_enum_SPLIT_MISSING",
                }],
            },
            ..FAKE
        };
        let out = bindings(&spec, &fixture(), &system_include_dirs()).expect("split fixture binds");
        assert_eq!(out.inventory.symbols, ["splitAlpha", "splitAlphaToo", "splitBeta"]);
        let stems: Vec<&str> = out.parts.iter().map(|p| p.stem.as_str()).collect();
        assert_eq!(stems, ["_a", "_b"]);
        // Mutation that reddens this: apply `allow_fn` to split parts (bindgen allowlists are a
        // union, so every part would get every function).
        assert!(out.parts[0].shims.contains("fn splitAlpha(") && !out.parts[0].shims.contains("fn splitBeta("));
        assert!(out.parts[1].shims.contains("fn splitBeta(") && !out.parts[1].shims.contains("fn splitAlpha("));
        assert!(out.parts[0].shims.contains("split_a_api()") && out.parts[1].shims.contains("split_b_api()"));
        assert!(out.types.contains("splitStatus_enum_SPLIT_SUCCESS"), "shared types are generated once");
    }

    #[test]
    fn query_interface_shims_call_by_id_and_report_functions_without_one() {
        let spec = LibSpec {
            name: "qi",
            headers: &["qi.h"],
            allow_fn: &["^qi.*"],
            allow_type: &["^qi.*", "^_qi.*"],
            allow_var: &[],
            positive_enums: &[],
            shape: Shape::QueryInterface {
                interface_header: "qi_interface.h",
                loader_fn: "qi_api",
                returns: &[ReturnPolicy {
                    types: &["qiStatus"],
                    not_loaded: "_qiStatus_QI_LIBRARY_NOT_FOUND",
                    missing_symbol: "_qiStatus_QI_NO_IMPLEMENTATION",
                }],
            },
            ..FAKE
        };
        let out = bindings(&spec, &fixture(), &system_include_dirs()).expect("qi fixture binds");
        assert_eq!(out.inventory.symbols, ["qiAbsentNumber", "qiGetValue", "qiInitialize", "qiMissingStore", "qiStore", "qiVersionNumber"]);
        // Checked numeric/void functions plus the declaration lacking a query id have no C shim.
        assert_eq!(out.inventory.unshimmed, ["qiAbsentNumber", "qiMissingStore", "qiNotInTable", "qiStore", "qiVersionNumber"]);
        let part = &out.parts[0];
        assert!(part.api.is_none(), "query-interface libraries need no loading struct");
        assert!(part.shims.contains("0x150e828u32") || part.shims.contains("22079528u32"), "{}", part.shims);
        assert!(part.shims.contains("_qiStatus_QI_NO_IMPLEMENTATION") && part.shims.contains("_qiStatus_QI_LIBRARY_NOT_FOUND"));
    }

    #[test]
    fn interface_ids_parse_the_table_form() {
        let text = "struct T t[] =\n{\n    { \"NvAPI_Initialize\", 0x0150e828 },\n    { \"NvAPI_Unload\", 0xd22bdd7e },\n};\n";
        assert_eq!(interface_ids(text), [("NvAPI_Initialize".to_string(), 0x0150e828), ("NvAPI_Unload".to_string(), 0xd22bdd7e)]);
    }

    #[test]
    fn signature_types_collect_aliases_and_skip_std_paths() {
        // The cuFFT failure that motivated this: `cudaStream_t` and `libraryPropertyType` appear
        // only in signatures and were missing from the types pass.
        let src = r#"
            pub struct Api {
                __library: ::libloading::Library,
                pub cufftSetStream: Result<unsafe extern "C" fn(plan: cufftHandle, s: cudaStream_t) -> cufftResult, ::libloading::Error>,
                pub cufftGetProperty: Result<unsafe extern "C" fn(t: libraryPropertyType, v: *mut ::std::os::raw::c_int) -> cufftResult, ::libloading::Error>,
            }
        "#;
        assert_eq!(
            signature_types("t", src, "Api").expect("parses"),
            ["cudaStream_t", "cufftHandle", "cufftResult", "libraryPropertyType"]
        );
    }

    fn assert_invalid_return_policies(returns: &[ReturnPolicy<'_>], expected: &str) {
        let api = r#"pub struct Api { pub value: Result<unsafe extern "C" fn() -> Status, ::libloading::Error> }"#;
        let error = shims("invalid", api, "Api", "loader", returns).expect_err("invalid recipe must be rejected");
        match error {
            Error::Generated { lib, msg } => {
                assert_eq!(lib, "invalid");
                assert!(msg.contains(expected), "{msg}");
            }
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn return_policies_reject_empty_type_lists() {
        assert_invalid_return_policies(&[ReturnPolicy { types: &[], not_loaded: "1", missing_symbol: "2" }], "no types");
    }

    #[test]
    fn return_policies_reject_duplicate_type_names() {
        let policy = ReturnPolicy { types: &["Status"], not_loaded: "1", missing_symbol: "2" };
        assert_invalid_return_policies(&[policy, policy], "duplicate return policy for Status");
        assert_invalid_return_policies(
            &[ReturnPolicy { types: &["Status", "Status"], ..policy }],
            "duplicate return policy for Status",
        );
    }

    #[test]
    fn return_policies_reject_invalid_type_identifiers() {
        assert_invalid_return_policies(
            &[ReturnPolicy { types: &["Not::AnIdentifier"], not_loaded: "1", missing_symbol: "2" }],
            "return type",
        );
    }

    #[test]
    fn return_policies_reject_malformed_failure_expressions() {
        assert_invalid_return_policies(
            &[ReturnPolicy { types: &["Status"], not_loaded: "{", missing_symbol: "2" }],
            "not_loaded",
        );
        assert_invalid_return_policies(
            &[ReturnPolicy { types: &["Status"], not_loaded: "1", missing_symbol: "{" }],
            "missing_symbol",
        );
    }

    #[test]
    fn function_table_inventory_reads_option_fn_fields() {
        // `PBAZOPEN` is how NVENC names its table fields: a typedef'd function-pointer type.
        let src = r#"
            pub type PBAZOPEN = Option<unsafe extern "C" fn(p: *mut u8) -> i32>;
            pub type NotAFn = u32;
            pub struct Table {
                pub fooCreate: Option<unsafe extern "C" fn(x: i32) -> i32>,
                pub barDestroy: Option<unsafe extern "C" fn()>,
                pub bazOpen: PBAZOPEN,
                pub version: NotAFn,
                pub reserved: u64,
            }
        "#;
        assert_eq!(inventory("t", src, "Table").expect("parses"), ["barDestroy", "bazOpen", "fooCreate"]);
    }
}
