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
//!   the coverage matrix (names and hashes only, never header text);
//! - `<name>_manifest.json` — written last ([`Manifest`]): the exact files the library's shape
//!   promises (all verified to exist, otherwise a build error naming the missing one) and whether
//!   they were [`Origin::Generated`] or copied from the committed prebuilt set
//!   ([`Origin::Prebuilt`], which also prints a `cargo::warning`). `xtask prebuilt` stages from
//!   the manifest instead of inferring completeness, and the sys crates expose it as
//!   `BINDINGS_MANIFEST`.
//!
//! **Cross builds.** SDK discovery (vcv-rs) covers the host OS only. When the Cargo target OS
//! differs from the host, `Sdk::CudaToolkit*` and `Sdk::Nv` take the SDK root from the
//! discovery's root variables (`CUDA_PATH`, `NCCL_ROOT`, ...; `<root>/include`), and the MSVC
//! include directories are added only for `*-windows-msvc` targets: a Linux target gets its
//! sysroot from `BINDGEN_EXTRA_CLANG_ARGS` / `BINDGEN_EXTRA_CLANG_ARGS_<target>`, which bindgen
//! reads itself.
//!
//! **Optional-SDK policy (L-32).** A library that exists only on some targets and needs a
//! separately installed SDK (NCCL, cuFile: Linux) *requires* that SDK on its target: a missing SDK
//! is a build error naming the variable to set (there is no prebuilt set for them), and on other
//! targets the crate compiles to nothing without any SDK ([`generate_on_os`]). NGX is the one
//! deliberate exception: its sys crate gates every item on `cfg(has_ngx_sdk)` and the `ngx`
//! wrapper has a complete stub path, so a missing SDK degrades to a warning there.
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
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use syn::{
    Expr, Fields, GenericArgument, Item, ItemStruct, PathArguments, ReturnType, Type, TypeBareFn,
};

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
        /// `(name, C expression)` pairs evaluated by clang and emitted as
        /// `pub const NV_RS_PROBE_<name>: c_uint`. For values bindgen cannot compute from a header
        /// itself, such as the function-like version macro `NVENCAPI_STRUCT_VERSION(2)`, so a sys
        /// crate never hand-copies them. A probe clang cannot fold to a constant is a build error.
        probes: &'static [(&'static str, &'static str)],
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
    /// The SDK headers were found but libclang cannot be loaded, and no usable prebuilt bindings
    /// exist for this target.
    #[error(
        "{lib}: libclang is unavailable ({reason}) and no prebuilt bindings could be used ({prebuilt}); install LLVM and set LIBCLANG_PATH"
    )]
    Libclang {
        /// The library.
        lib: &'static str,
        /// Why libclang could not be loaded.
        reason: String,
        /// Why the prebuilt bindings could not be used.
        prebuilt: String,
    },
    /// A variable Cargo sets for build scripts is missing (the function runs outside `build.rs`).
    #[error("{lib}: {var} is unset: this runs from a build script")]
    CargoEnv {
        /// The library.
        lib: &'static str,
        /// The variable.
        var: &'static str,
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
    /// Every file libclang opened from the SDK include directories (the bound headers and the
    /// sub-headers they include), sorted: the rebuild inputs.
    pub includes: Vec<PathBuf>,
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

/// How the files in a build's `OUT_DIR` came to be.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    /// Written by the generator from the installed SDK headers.
    Generated,
    /// Copied from the committed prebuilt bindings (explicit request, absent SDK or no libclang).
    Prebuilt,
}

/// The record `generate` writes last into `OUT_DIR` as `<lib>_manifest.json`.
///
/// `files` is exactly the set of binding files the library's [`Shape`] promises (types, inventory,
/// api and shims), all of which existed when the manifest was written; the manifest itself is not
/// listed. `xtask prebuilt` stages `files` and nothing else, and skips any build whose `origin` is
/// [`Origin::Prebuilt`]. A missing manifest means the generation never finished.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Manifest {
    /// The library's short name.
    pub lib: String,
    /// Whether the files were generated or copied from the committed prebuilt set.
    pub origin: Origin,
    /// File names relative to `OUT_DIR`, in promise order.
    pub files: Vec<String>,
}

impl Manifest {
    /// The manifest's file name for library `lib`.
    pub fn file_name(lib: &str) -> String {
        format!("{lib}_manifest.json")
    }
}

/// A header and the sha256 of its contents.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HeaderHash {
    /// The `#include` name.
    pub name: String,
    /// Lower-case hex sha256.
    pub sha256: String,
}

/// Generates `spec` into `$OUT_DIR` (see the module docs) and prints the `cargo:rerun-if-*` lines.
/// Returns the generated sources' inventory.
///
/// Pre-generated bindings (`<NV_RS_PREBUILT_DIR>[/<TARGET>]`, else
/// `<CARGO_MANIFEST_DIR>/prebuilt/<TARGET>/`) replace generation in exactly three cases, each named
/// in a cargo warning: `NV_RS_USE_PREBUILT=1` (explicit request), the SDK is not installed, or
/// libclang cannot be loaded. When the SDK headers were found, every other failure of the
/// generator (a `bindgen` error, a missing header, a panic) is a build error with its own cause.
/// Whenever prebuilt bindings are used although the SDK is installed, the header sha256 recorded in
/// the prebuilt inventory is compared with the installed headers and every difference is warned
/// about. A fallback is not sticky: the SDK root variables, the libclang inputs and the found
/// headers are rerun triggers.
pub fn generate(spec: &LibSpec) -> Result<Inventory, Error> {
    println!("cargo::rerun-if-env-changed=NV_RS_USE_PREBUILT");
    println!("cargo::rerun-if-env-changed=NV_RS_PREBUILT_DIR");
    for var in rerun_vars(spec, vcv_rs::cuda::requested_root().is_some()) {
        println!("cargo:rerun-if-env-changed={var}");
    }
    for var in LIBCLANG_VARS {
        println!("cargo:rerun-if-env-changed={var}");
    }
    let target = std::env::var("TARGET").unwrap_or_default();
    emit_target_clang_arg_reruns(&target);

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").ok_or_else(|| Error::Generated {
        lib: spec.name,
        msg: "OUT_DIR is unset: generate() runs from build.rs".into(),
    })?);
    // A manifest left by an earlier run must never vouch for this one.
    let stale = out_dir.join(Manifest::file_name(spec.name));
    match std::fs::remove_file(&stale) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(source) => {
            return Err(Error::Io {
                lib: spec.name,
                path: stale,
                source,
            });
        }
    }
    let use_prebuilt = std::env::var("NV_RS_USE_PREBUILT")
        .is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true"));
    let static_libs = match spec.shape {
        Shape::StaticLink { link_libs, .. } => link_libs,
        _ => &[],
    };

    if use_prebuilt {
        // Only an absent SDK is acceptable here; any other discovery failure is real.
        let (discovered, absent) = match sdk_paths(spec) {
            Ok(paths) => (Some(paths), None),
            Err(err @ Error::SdkNotFound { .. }) => (None, Some(err)),
            Err(err) => return Err(err),
        };
        // A static library is linked from an installed SDK; prebuilt bindings cannot supply it.
        if let Some(err) = absent
            && !static_libs.is_empty()
        {
            return Err(err);
        }
        let nvidia = discovered.as_ref().map_or(&[][..], |d| &d.nvidia);
        emit_runtime_env(spec.name, nvidia)?;
        if let Shape::StaticLink {
            link_libs,
            system_libs,
        } = spec.shape
        {
            emit_static_link_directives(spec.name, link_libs, system_libs, nvidia, true)?;
        }
        let inv = use_prebuilt_bindings(spec, &out_dir, &target, "NV_RS_USE_PREBUILT=1")?;
        if let Some(found) = &discovered {
            warn_header_drift(spec, &found.includes, &inv);
        }
        return Ok(inv);
    }

    // One discovery snapshot supplies both the headers and their matching native libraries.
    let SdkPaths {
        includes: include_dirs,
        nvidia,
    } = match sdk_paths(spec) {
        Ok(paths) => paths,
        Err(err @ Error::SdkNotFound { .. }) => {
            if !static_libs.is_empty() {
                return Err(err);
            }
            return match use_prebuilt_bindings(spec, &out_dir, &target, &format!("SDK absent: {err}"))
            {
                Ok(inv) => {
                    println!("cargo:rustc-env=NV_RS_SDK_BIN=");
                    Ok(inv)
                }
                Err(prebuilt) => {
                    println!(
                        "cargo::warning=nv-bindgen: {}: no usable prebuilt bindings either: {prebuilt}",
                        spec.name
                    );
                    Err(err)
                }
            };
        }
        Err(err) => return Err(err),
    };

    emit_runtime_env(spec.name, &nvidia)?;
    if let Shape::StaticLink {
        link_libs,
        system_libs,
    } = spec.shape
    {
        emit_static_link_directives(spec.name, link_libs, system_libs, &nvidia, true)?;
    }
    // Watch the found headers before anything can fail, so fixing the SDK or the toolchain reruns.
    let interface_header = match spec.shape {
        Shape::QueryInterface {
            interface_header, ..
        } => Some(interface_header),
        _ => None,
    };
    for name in spec.headers.iter().copied().chain(interface_header) {
        if let Some(path) = find_header(&include_dirs, name) {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    let system_dirs = system_include_dirs(spec.name)?;
    if let Err(reason) = libclang_available() {
        // The installed SDK cannot be parsed without libclang: the one toolchain condition that
        // permits prebuilt bindings. libclang is found through these, so a change reruns this.
        for var in ["PATH", "LD_LIBRARY_PATH", "LIBRARY_PATH"] {
            println!("cargo:rerun-if-env-changed={var}");
        }
        let inv = use_prebuilt_bindings(
            spec,
            &out_dir,
            &target,
            &format!("libclang unavailable: {reason}"),
        )
        .map_err(|prebuilt| Error::Libclang {
            lib: spec.name,
            reason,
            prebuilt: prebuilt.to_string(),
        })?;
        warn_header_drift(spec, &include_dirs, &inv);
        return Ok(inv);
    }
    // No fallback past this point: an error or a panic here is a generator or header problem.
    let out = bindings(spec, &include_dirs, &system_dirs)?;

    let write = |suffix: &str, text: &str| {
        let path = out_dir.join(format!("{}_{suffix}", spec.name));
        std::fs::write(&path, text).map_err(|source| Error::Io {
            lib: spec.name,
            path,
            source,
        })
    };
    write("types.rs", &out.types)?;
    for part in &out.parts {
        // `<lib>_api.rs` for one DLL, `<lib>_<part>_api.rs` per DLL of a split library.
        let stem = part.stem.trim_start_matches('_');
        let prefix = if stem.is_empty() {
            String::new()
        } else {
            format!("{stem}_")
        };
        if let Some(api) = &part.api {
            write(&format!("{prefix}api.rs"), api)?;
        }
        write(&format!("{prefix}shims.rs"), &part.shims)?;
    }
    let json = serde_json::to_string_pretty(&out.inventory).map_err(|e| Error::Generated {
        lib: spec.name,
        msg: e.to_string(),
    })?;
    write("inventory.json", &json)?;
    write_manifest(spec, &out_dir, Origin::Generated)?;
    // Umbrella headers (`cudnn.h`, `npp.h`, `nvapi.h`) hide most of the API in sub-headers.
    for path in &out.includes {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    Ok(out.inventory)
}

/// Inputs of libclang discovery (clang-sys `runtime` loading) and of bindgen's own arguments.
/// `CLANG_PATH` selects the clang whose builtin include directories bindgen adds.
const LIBCLANG_VARS: [&str; 4] = [
    "LIBCLANG_PATH",
    "LLVM_CONFIG_PATH",
    "CLANG_PATH",
    "BINDGEN_EXTRA_CLANG_ARGS",
];

/// Whether bindgen can load libclang, or why not.
///
/// bindgen 0.72 loads libclang at run time and panics when it cannot. Only that load is probed
/// here (through `bindgen::clang_version`), so every later panic of the generator surfaces as
/// itself instead of being mistaken for a missing libclang.
fn libclang_available() -> Result<(), String> {
    catch_quietly(|| {
        bindgen::clang_version();
    })
}

/// Runs `f`, turning a panic into its message without printing the default panic report.
fn catch_quietly(f: impl FnOnce() + std::panic::UnwindSafe) -> Result<(), String> {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(f);
    std::panic::set_hook(previous);
    result.map_err(|payload| {
        payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "a panic without a message".to_string())
    })
}

/// The bound headers whose installed bytes differ from the sha256 recorded in `inventory`, or that
/// are missing from `include_dirs`, as `"<name> (changed)"` / `"<name> (missing)"`.
fn header_drift(include_dirs: &[PathBuf], inventory: &Inventory) -> Vec<String> {
    inventory
        .headers
        .iter()
        .filter_map(|h| match find_header(include_dirs, &h.name) {
            None => Some(format!("{} (missing)", h.name)),
            Some(path) => match std::fs::read(&path) {
                Ok(bytes) if hex(&Sha256::digest(&bytes)) == h.sha256 => None,
                Ok(_) => Some(format!("{} (changed)", h.name)),
                Err(e) => Some(format!("{} (unreadable: {e})", h.name)),
            },
        })
        .collect()
}

/// Warns when prebuilt bindings were generated from other headers than the installed SDK's.
fn warn_header_drift(spec: &LibSpec, include_dirs: &[PathBuf], inventory: &Inventory) {
    let drift = header_drift(include_dirs, inventory);
    if !drift.is_empty() {
        println!(
            "cargo::warning=nv-bindgen: prebuilt bindings for {} were generated from other headers than the installed SDK: {}; layouts and constants may not match the installed library",
            spec.name,
            drift.join(", ")
        );
    }
}

/// [`generate`] only when the Cargo target OS is `os`, else `Ok(None)`: the one OS gate for sys
/// crates of single-OS libraries (NVAPI on Windows, cuFile on Linux), which compile to nothing
/// elsewhere and need no SDK there. An unset target OS is an error ([`Error::CargoEnv`]), never
/// "not this OS".
pub fn generate_on_os(spec: &LibSpec, os: &str) -> Result<Option<Inventory>, Error> {
    if target_os(spec.name)? == os {
        generate(spec).map(Some)
    } else {
        Ok(None)
    }
}

/// Whether `header` is in the include directories `spec`'s SDKs provide (e.g. an optional part of
/// an SDK that only newer releases ship). Fails like [`generate`] when an SDK is not installed.
pub fn has_header(spec: &LibSpec, header: &str) -> Result<bool, Error> {
    Ok(find_header(&sdk_paths(spec)?.includes, header).is_some())
}

/// `CARGO_CFG_TARGET_OS` of the build script's target (`"windows"`, `"linux"`, ...).
pub fn target_os(lib: &'static str) -> Result<String, Error> {
    cargo_cfg(lib, "CARGO_CFG_TARGET_OS")
}

/// `CARGO_CFG_TARGET_ARCH` of the build script's target (`"x86_64"`, `"aarch64"`, ...).
pub fn target_arch(lib: &'static str) -> Result<String, Error> {
    cargo_cfg(lib, "CARGO_CFG_TARGET_ARCH")
}

fn cargo_cfg(lib: &'static str, var: &'static str) -> Result<String, Error> {
    std::env::var(var).map_err(|_| Error::CargoEnv { lib, var })
}

fn emit_runtime_env(lib: &'static str, nvidia: &[vcv_rs::nvsdk::SdkInfo]) -> Result<(), Error> {
    let runtime = match nvidia.iter().find_map(|sdk| sdk.runtime_dir.as_deref()) {
        Some(path) => sdk_path_str(lib, path)?,
        None => "",
    };
    println!("cargo:rustc-env=NV_RS_SDK_BIN={runtime}");
    Ok(())
}

fn emit_static_link_directives(
    lib: &'static str,
    link_libs: &[&str],
    system_libs: &[&str],
    nvidia: &[vcv_rs::nvsdk::SdkInfo],
    require_lib_dir: bool,
) -> Result<(), Error> {
    for l in system_libs {
        println!("cargo:rustc-link-lib=dylib={l}");
    }
    if require_lib_dir && !link_libs.is_empty() && nvidia.iter().all(|sdk| sdk.lib_dir.is_none()) {
        return Err(Error::MissingLinkDirectory { lib });
    }
    for sdk in nvidia {
        if let Some(lib_dir) = &sdk.lib_dir {
            println!(
                "cargo:rustc-link-search=native={}",
                sdk_path_str(lib, lib_dir)?
            );
        }
    }
    for l in link_libs {
        println!("cargo:rustc-link-lib=static={l}");
    }
    Ok(())
}

fn expected_output_files(spec: &LibSpec) -> Vec<String> {
    let mut files = vec![
        format!("{}_types.rs", spec.name),
        format!("{}_inventory.json", spec.name),
    ];
    match spec.shape {
        Shape::Exports { .. } => {
            files.push(format!("{}_api.rs", spec.name));
            files.push(format!("{}_shims.rs", spec.name));
        }
        Shape::SplitExports { parts, .. } => {
            for part in parts {
                files.push(format!("{}_{}_api.rs", spec.name, part.name));
                files.push(format!("{}_{}_shims.rs", spec.name, part.name));
            }
        }
        Shape::QueryInterface { .. } | Shape::StaticLink { .. } => {
            files.push(format!("{}_shims.rs", spec.name));
        }
        Shape::FunctionTable { .. } => {}
    }
    files
}

/// Checks that every file `spec`'s shape promises exists in `out_dir`, then writes the
/// [`Manifest`] naming them. A missing file is a hard build error naming it, so a generator that
/// forgot to write a promised file fails here and not as an `include!` error in the sys crate.
fn write_manifest(spec: &LibSpec, out_dir: &Path, origin: Origin) -> Result<(), Error> {
    let files = expected_output_files(spec);
    for file in &files {
        let path = out_dir.join(file);
        if !path.is_file() {
            return Err(Error::Generated {
                lib: spec.name,
                msg: format!("promised output file is missing: {}", path.display()),
            });
        }
    }
    let manifest = Manifest {
        lib: spec.name.to_string(),
        origin,
        files,
    };
    let text = serde_json::to_string_pretty(&manifest).map_err(|e| Error::Generated {
        lib: spec.name,
        msg: e.to_string(),
    })?;
    let path = out_dir.join(Manifest::file_name(spec.name));
    std::fs::write(&path, text).map_err(|source| Error::Io {
        lib: spec.name,
        path,
        source,
    })
}

/// bindgen also reads `BINDGEN_EXTRA_CLANG_ARGS_<target>` (target with `-` or `_`), so a
/// cross-compiling caller can supply a sysroot per target; changing it reruns generation.
fn emit_target_clang_arg_reruns(target: &str) {
    if target.is_empty() {
        return;
    }
    println!(
        "cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_{}",
        target.replace('-', "_")
    );
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_{target}");
}

fn resolve_prebuilt_dir(
    manifest_dir: Option<&Path>,
    env_dir: Option<&OsStr>,
    target: &str,
    expected: &[String],
) -> Option<PathBuf> {
    let has_all = |dir: &Path| expected.iter().all(|f| dir.join(f).is_file());
    if let Some(raw) = env_dir.filter(|v| !v.is_empty()) {
        let base = Path::new(raw);
        if !target.is_empty() {
            let targeted = base.join(target);
            if has_all(&targeted) {
                return Some(targeted);
            }
        }
        if has_all(base) {
            return Some(base.to_path_buf());
        }
        // `NV_RS_PREBUILT_DIR` may hold only some libraries: fall through to the crate's own.
    }
    if target.is_empty() {
        return None;
    }
    manifest_dir.map(|m| m.join("prebuilt").join(target))
}

fn copy_prebuilt_bindings(
    spec: &LibSpec,
    prebuilt_dir: &Path,
    out_dir: &Path,
) -> Result<(Vec<PathBuf>, Inventory), Error> {
    let expected = expected_output_files(spec);
    let mut copied_sources = Vec::with_capacity(expected.len());
    for file in &expected {
        let src = prebuilt_dir.join(file);
        if !src.is_file() {
            return Err(Error::Generated {
                lib: spec.name,
                msg: format!("prebuilt file missing: {}", src.display()),
            });
        }
        let dst = out_dir.join(file);
        std::fs::copy(&src, &dst).map_err(|source| Error::Io {
            lib: spec.name,
            path: dst,
            source,
        })?;
        copied_sources.push(src);
    }
    let inv_path = out_dir.join(format!("{}_inventory.json", spec.name));
    let inv_text = std::fs::read_to_string(&inv_path).map_err(|source| Error::Io {
        lib: spec.name,
        path: inv_path,
        source,
    })?;
    let inventory: Inventory = serde_json::from_str(&inv_text).map_err(|e| Error::Generated {
        lib: spec.name,
        msg: format!("prebuilt inventory parse failed: {e}"),
    })?;
    Ok((copied_sources, inventory))
}

fn use_prebuilt_bindings(
    spec: &LibSpec,
    out_dir: &Path,
    target: &str,
    reason: &str,
) -> Result<Inventory, Error> {
    let expected = expected_output_files(spec);
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from);
    let env_dir = std::env::var_os("NV_RS_PREBUILT_DIR");
    let prebuilt_dir = resolve_prebuilt_dir(
        manifest_dir.as_deref(),
        env_dir.as_deref(),
        target,
        &expected,
    )
    .ok_or_else(|| Error::Generated {
        lib: spec.name,
        msg: "cannot resolve prebuilt directory (TARGET or CARGO_MANIFEST_DIR unset)".into(),
    })?;
    let (sources, inventory) = copy_prebuilt_bindings(spec, &prebuilt_dir, out_dir)?;
    write_manifest(spec, out_dir, Origin::Prebuilt)?;
    for src in &sources {
        println!("cargo::rerun-if-changed={}", src.display());
    }
    println!(
        "cargo::warning=nv-bindgen: using prebuilt bindings for {} ({}) from {} ({reason})",
        spec.name,
        target,
        prebuilt_dir.display()
    );
    Ok(inventory)
}

/// The types and constants of `spec` (its `allow_type` / `allow_var`) as `no_std` Rust (`core::`
/// paths, no layout tests, `Default` derived), for device crates compiled by cuda-oxide: the
/// same header-derived definitions the host sys crate uses, so host and device agree on layouts
/// and enum values without hand-copied constants. Headers are located as in [`generate`].
pub fn core_types(spec: &LibSpec) -> Result<String, Error> {
    for var in rerun_vars(spec, vcv_rs::cuda::requested_root().is_some()) {
        println!("cargo:rerun-if-env-changed={var}");
    }
    for var in LIBCLANG_VARS {
        println!("cargo:rerun-if-env-changed={var}");
    }
    emit_target_clang_arg_reruns(&std::env::var("TARGET").unwrap_or_default());
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
    for d in system_include_dirs(spec.name)? {
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
    b.generate()
        .map(|g| g.to_string())
        .map_err(|e| Error::Bindgen {
            lib: spec.name,
            msg: e.to_string(),
        })
}

/// Runs bindgen for `spec` against explicit include directories (`include_dirs` searched first,
/// `system_dirs` after libclang's own). Pure apart from reading headers.
pub fn bindings(
    spec: &LibSpec,
    include_dirs: &[PathBuf],
    system_dirs: &[PathBuf],
) -> Result<Output, Error> {
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
        let bytes = std::fs::read(&path).map_err(|source| Error::Io {
            lib: spec.name,
            path: path.clone(),
            source,
        })?;
        headers.push(HeaderHash {
            name: (*name).to_string(),
            sha256: hex(&Sha256::digest(&bytes)),
        });
        wrapper.push_str(&format!("#include <{name}>\n"));
    }
    let probes: &[(&str, &str)] = match spec.shape {
        Shape::FunctionTable { probes, .. } => probes,
        _ => &[],
    };
    for (name, expr) in probes {
        wrapper.push_str(&format!(
            "static const unsigned int NV_RS_PROBE_{name} = ({expr});\n"
        ));
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
    let run = |b: bindgen::Builder| {
        b.generate()
            .map(|g| g.to_string())
            .map_err(|e| Error::Bindgen {
                lib: spec.name,
                msg: e.to_string(),
            })
    };

    // Functions pass first (exports only): its signatures name the types the types pass must
    // also emit. bindgen traces dependencies only of items it generates, so a function left out
    // of codegen would not pull in `cudaStream_t` / `libraryPropertyType`; they are allowlisted
    // by exact name instead.
    // (stem, api_type, loader_fn, header files or empty for "all allowlisted functions")
    let plan: Vec<(String, &str, &str, &[&str])> = match spec.shape {
        Shape::Exports {
            api_type,
            loader_fn,
            ..
        } => vec![(String::new(), api_type, loader_fn, &[])],
        // The signatures come from a throwaway dynamic-loading struct; the shims call the query.
        Shape::QueryInterface { loader_fn, .. } => {
            vec![(String::new(), QI_SIGNATURES, loader_fn, &[])]
        }
        Shape::SplitExports { parts, .. } => parts
            .iter()
            .map(|p| {
                (
                    format!("_{}", p.name),
                    p.api_type,
                    p.loader_fn,
                    p.header_files,
                )
            })
            .collect(),
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
    let returns: &[ReturnPolicy<'_>] = match spec.shape {
        Shape::Exports { returns, .. }
        | Shape::SplitExports { returns, .. }
        | Shape::QueryInterface { returns, .. } => returns,
        Shape::FunctionTable { .. } | Shape::StaticLink { .. } => &[],
    };
    let api_structs: Vec<(&str, &str)> = apis
        .iter()
        .map(|(_, api_type, _, src)| (src.as_str(), *api_type))
        .collect();
    check_return_policies_used(spec.name, returns, &api_structs)?;
    // A split part takes only its own headers' functions, so a function declared elsewhere would
    // silently get no shim: list every allowlisted function no part claims.
    let all_split_fns = match spec.shape {
        Shape::SplitExports { .. } if !spec.allow_fn.is_empty() => {
            let mut ab = base()
                .dynamic_library_name(SPLIT_ALL_FNS)
                .dynamic_link_require_all(false)
                .with_codegen_config(bindgen::CodegenConfig::FUNCTIONS);
            for f in spec.allow_fn {
                ab = ab.allowlist_function(*f);
            }
            inventory(spec.name, &run(ab)?, SPLIT_ALL_FNS)?
        }
        _ => Vec::new(),
    };
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

    // Types pass: the allowlisted types and constants plus every type an entry point names. It
    // parses every header, so it also records which files libclang opened.
    let opened = IncludeLog::default();
    let mut tb = base()
        .derive_default(true)
        .blocklist_function(".*")
        .parse_callbacks(Box::new(opened.clone()));
    for t in &signature_types {
        tb = tb.allowlist_type(format!("^{t}$"));
    }
    for t in spec.allow_type {
        tb = tb.allowlist_type(*t);
    }
    for v in spec.allow_var {
        tb = tb.allowlist_var(*v);
    }
    if !probes.is_empty() {
        tb = tb.allowlist_var("^NV_RS_PROBE_.*");
    }
    for t in spec.reuse_cuda_types {
        tb = tb.blocklist_type(*t);
    }
    let mut types = normalize_positive_enums(spec.name, &run(tb)?, spec.positive_enums)?;
    for (name, expr) in probes {
        if !types.contains(&format!("pub const NV_RS_PROBE_{name}:")) {
            return Err(Error::Generated {
                lib: spec.name,
                msg: format!("probe `{name}` = `{expr}` did not fold to a constant"),
            });
        }
    }
    if !spec.reuse_cuda_types.is_empty() {
        types = format!(
            "pub use cuda_bindings::{{{}}};\n{types}",
            spec.reuse_cuda_types.join(", ")
        );
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
                parts.push(GeneratedPart {
                    stem,
                    api: Some(api),
                    shims: generated.source,
                });
            }
        }
        Shape::QueryInterface {
            interface_header,
            returns,
            ..
        } => {
            let path = find_header(include_dirs, interface_header).ok_or_else(|| {
                Error::HeaderNotFound {
                    lib: spec.name,
                    header: interface_header.to_string(),
                    searched: include_dirs.to_vec(),
                }
            })?;
            let text = std::fs::read_to_string(&path).map_err(|source| Error::Io {
                lib: spec.name,
                path,
                source,
            })?;
            let ids = interface_ids(&text);
            for (stem, _, loader_fn, api) in apis {
                let generated = query_shims(spec.name, &api, loader_fn, &ids, returns)?;
                symbols.extend(generated.bound);
                unshimmed.extend(generated.unshimmed);
                parts.push(GeneratedPart {
                    stem,
                    api: None,
                    shims: generated.source,
                });
            }
        }
        Shape::FunctionTable { table_type, .. } => {
            symbols = inventory(spec.name, &types, table_type)?;
        }
        Shape::StaticLink { .. } => {
            let src = externs.unwrap_or_default();
            symbols = extern_fns(spec.name, &src)?.0;
            // The `extern` block takes the place of shims: nothing is loaded at runtime.
            parts.push(GeneratedPart {
                stem: String::new(),
                api: None,
                shims: src,
            });
        }
    }
    symbols.sort();
    symbols.dedup();
    unshimmed.extend(
        all_split_fns
            .into_iter()
            .filter(|f| symbols.binary_search(f).is_err()),
    );
    unshimmed.sort();
    unshimmed.dedup();

    let mut includes: Vec<PathBuf> = opened
        .0
        .borrow()
        .iter()
        .map(PathBuf::from)
        .filter(|p| include_dirs.iter().any(|d| p.starts_with(d)))
        .collect();
    includes.sort();
    includes.dedup();

    Ok(Output {
        types,
        parts,
        inventory: Inventory {
            lib: spec.name.to_string(),
            headers,
            symbols,
            unshimmed,
        },
        includes,
    })
}

/// Internal name of the throwaway struct listing every allowlisted function of a split library.
const SPLIT_ALL_FNS: &str = "NvBindgenSplitAllFns";

/// Records every file libclang opens through an `#include` (bindgen's `include_file` callback).
#[derive(Debug, Default, Clone)]
struct IncludeLog(std::rc::Rc<std::cell::RefCell<Vec<String>>>);

impl bindgen::callbacks::ParseCallbacks for IncludeLog {
    fn include_file(&self, filename: &str) {
        self.0.borrow_mut().push(filename.to_string());
    }
}

/// Fails when a return policy names no type any entry point returns: such a policy is a recipe
/// error that silently leaves its functions on the checked Rust ABI (bindgen spells `size_t` /
/// `ssize_t` as `usize` / `isize`, so a policy keyed on the C name never matches).
fn check_return_policies_used(
    lib: &'static str,
    returns: &[ReturnPolicy<'_>],
    apis: &[(&str, &str)],
) -> Result<(), Error> {
    let gen_err = |msg: String| Error::Generated { lib, msg };
    let mut used = vec![false; returns.len()];
    for (src, struct_name) in apis {
        let file = syn::parse_file(src).map_err(|e| gen_err(e.to_string()))?;
        let api = find_struct(&file, struct_name)
            .ok_or_else(|| gen_err(format!("no struct {struct_name}")))?;
        let Fields::Named(fields) = &api.fields else {
            continue;
        };
        for field in &fields.named {
            let Some(bare) = result_bare_fn(&field.ty) else {
                continue;
            };
            let ReturnType::Type(_, ty) = &bare.output else {
                continue;
            };
            let Type::Path(path) = ty.as_ref() else {
                continue;
            };
            let Some(last) = path.path.segments.last() else {
                continue;
            };
            for (flag, policy) in used.iter_mut().zip(returns) {
                if policy.types.iter().any(|t| last.ident == *t) {
                    *flag = true;
                }
            }
        }
    }
    let unused: Vec<String> = returns
        .iter()
        .zip(&used)
        .filter(|(_, used)| !**used)
        .map(|(policy, _)| policy.types.join("/"))
        .collect();
    if unused.is_empty() {
        Ok(())
    } else {
        Err(gen_err(format!(
            "return policy for {} matches no entry point's return type (bindgen spells size_t/ssize_t as usize/isize)",
            unused.join(", ")
        )))
    }
}

/// Names of the functions declared in `extern` blocks of `source`, and every single-segment type
/// their signatures name (for the types pass).
fn extern_fns(lib: &'static str, source: &str) -> Result<(Vec<String>, Vec<String>), Error> {
    let file = syn::parse_file(source).map_err(|e| Error::Generated {
        lib,
        msg: e.to_string(),
    })?;
    let mut names = Vec::new();
    let mut types = Vec::new();
    for item in &file.items {
        let Item::ForeignMod(block) = item else {
            continue;
        };
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
            let rest = line
                .trim()
                .strip_prefix('{')?
                .trim_start()
                .strip_prefix('"')?;
            let (name, rest) = rest.split_once('"')?;
            let hex = rest
                .trim_start()
                .strip_prefix(',')?
                .trim()
                .trim_end_matches(['}', ',', ' '])
                .trim();
            let id = u32::from_str_radix(
                hex.strip_prefix("0x").or_else(|| hex.strip_prefix("0X"))?,
                16,
            )
            .ok()?;
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
    let sigs = find_struct(&file, QI_SIGNATURES)
        .ok_or_else(|| gen_err(format!("no struct {QI_SIGNATURES}")))?;
    let loader = format_ident!("{loader_fn}");
    let policies = parse_return_policies(lib, returns)?;
    let Fields::Named(fields) = &sigs.fields else {
        return Err(gen_err(format!("{QI_SIGNATURES} has no named fields")));
    };

    let mut bound = Vec::new();
    let mut unshimmed = Vec::new();
    let mut fns: Vec<TokenStream> = Vec::new();
    for field in &fields.named {
        let (Some(name), Some(bare)) = (field.ident.as_ref(), result_bare_fn(&field.ty)) else {
            continue;
        };
        let Some(id) = ids
            .iter()
            .find(|(n, _)| n == &name.to_string())
            .map(|(_, id)| *id)
        else {
            unshimmed.push(name.to_string());
            continue;
        };
        let fb = fallback(&bare.output, &policies);
        if fb.is_none() {
            unshimmed.push(name.to_string());
        }
        let plan = wrapper_plan(&bare.output, fb, lib, name, quote!(None));
        let WrapperPlan {
            abi,
            ret,
            on_missing,
            on_not_loaded,
            mode,
        } = plan;
        let args = shim_args(bare);
        let names = args.iter().map(|(n, _)| n);
        let defs = args.iter().map(|(_, d)| d);
        let result = mode.wrap_call(quote!(unsafe { __nv_fn(#(#names),*) }));
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
                        // Resolved once per id (0 = the driver lacks it): the query is a hash
                        // lookup inside the driver and the loaded library never changes.
                        static __NV_ADDR: ::std::sync::OnceLock<usize> = ::std::sync::OnceLock::new();
                        let __nv_addr = *__NV_ADDR.get_or_init(|| unsafe { (__nv_qi.query)(#id_lit) } as usize);
                        if __nv_addr == 0 {
                            #on_missing
                        } else {
                            let __nv_fn = unsafe { ::core::mem::transmute::<usize, #fn_ty>(__nv_addr) };
                            #result
                        }
                    }
                    Err(__nv_error) => #on_not_loaded,
                }
            }
        });
        bound.push(name.to_string());
    }
    let parsed: syn::File =
        syn::parse2(quote!(#(#fns)*)).map_err(|e| gen_err(format!("shims: {e}")))?;
    bound.sort();
    unshimmed.sort();
    let source = document_unsafe_blocks(
        &prettyplease::unparse(&parsed),
        "the header declares this exact id's signature, the address was returned by the query for that id and its library remains loaded for the process; the caller upholds the native call contract.",
    );
    Ok(QueryShims {
        source,
        bound,
        unshimmed,
    })
}

/// Re-attaches `// SAFETY:` comments above every `unsafe {` block after `prettyplease::unparse`,
/// which strips comments from `quote!` tokens.
fn document_unsafe_blocks(src: &str, safety_comment: &str) -> String {
    let mut out = String::with_capacity(
        src.len() + src.matches("unsafe {").count() * (safety_comment.len() + 16),
    );
    for line in src.lines() {
        if line.contains("unsafe {") {
            let indent = &line[..line.len() - line.trim_start().len()];
            out.push_str(indent);
            out.push_str("// SAFETY: ");
            out.push_str(safety_comment);
            out.push('\n');
        }
        out.push_str(line);
        out.push('\n');
    }
    out
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
fn parse_return_policies<'a>(
    lib: &'static str,
    returns: &[ReturnPolicy<'a>],
) -> Result<Vec<ParsedReturnPolicy<'a>>, Error> {
    let gen_err = |msg: String| Error::Generated { lib, msg };
    let mut names = Vec::new();
    let mut parsed = Vec::with_capacity(returns.len());
    for policy in returns {
        if policy.types.is_empty() {
            return Err(gen_err("return policy has no types".into()));
        }
        for name in policy.types {
            syn::parse_str::<proc_macro2::Ident>(name)
                .map_err(|e| gen_err(format!("return type {name:?}: {e}")))?;
            if names.contains(name) {
                return Err(gen_err(format!("duplicate return policy for {name}")));
            }
            names.push(*name);
        }
        let not_loaded =
            syn::parse_str(policy.not_loaded).map_err(|e| gen_err(format!("not_loaded: {e}")))?;
        let missing_symbol = syn::parse_str(policy.missing_symbol)
            .map_err(|e| gen_err(format!("missing_symbol: {e}")))?;
        parsed.push(ParsedReturnPolicy {
            types: policy.types,
            not_loaded,
            missing_symbol,
        });
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
    Policy {
        not_loaded: &'a Expr,
        missing_symbol: &'a Expr,
    },
    /// A null pointer (`*const` / `*mut`).
    Null { mutable: bool },
}

fn fallback<'a>(ret: &ReturnType, policies: &'a [ParsedReturnPolicy<'_>]) -> Option<Fallback<'a>> {
    match ret {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => match ty.as_ref() {
            Type::Ptr(p) => Some(Fallback::Null {
                mutable: p.mutability.is_some(),
            }),
            Type::Path(p) => {
                let last = p.path.segments.last()?.ident.to_string();
                policies
                    .iter()
                    .find(|policy| policy.types.contains(&last.as_str()))
                    .map(|policy| Fallback::Policy {
                        not_loaded: &policy.not_loaded,
                        missing_symbol: &policy.missing_symbol,
                    })
            }
            _ => None,
        },
    }
}

#[derive(Clone, Copy)]
enum WrapMode {
    Native,
    CheckedScalar,
    CheckedVoid,
}

impl WrapMode {
    fn wrap_call(self, call: TokenStream) -> TokenStream {
        match self {
            Self::Native => call,
            Self::CheckedScalar => quote!(::core::result::Result::Ok(#call)),
            // Emit `{ unsafe { call }; Ok(()) }` so void shims do not trigger `clippy::unit_arg`.
            Self::CheckedVoid => quote!({ #call; ::core::result::Result::Ok(()) }),
        }
    }
}

struct WrapperPlan {
    abi: TokenStream,
    ret: TokenStream,
    on_missing: TokenStream,
    on_not_loaded: TokenStream,
    mode: WrapMode,
}

/// The same ABI and failure-value decision for exports and query-interface functions.
fn wrapper_plan(
    ret: &ReturnType,
    fb: Option<Fallback<'_>>,
    lib: &'static str,
    name: &proc_macro2::Ident,
    source: TokenStream,
) -> WrapperPlan {
    let native_ret = match ret {
        ReturnType::Default => quote!(),
        ReturnType::Type(_, ty) => quote!(-> #ty),
    };
    let (on_missing, on_not_loaded) = match fb {
        Some(Fallback::Policy {
            not_loaded,
            missing_symbol,
        }) => (quote!(#missing_symbol), quote!(#not_loaded)),
        Some(Fallback::Null { mutable: false }) => {
            (quote!(::core::ptr::null()), quote!(::core::ptr::null()))
        }
        Some(Fallback::Null { mutable: true }) => (
            quote!(::core::ptr::null_mut()),
            quote!(::core::ptr::null_mut()),
        ),
        None => {
            let (mode, ty) = match ret {
                ReturnType::Default => (WrapMode::CheckedVoid, quote!(())),
                ReturnType::Type(_, ty) => (WrapMode::CheckedScalar, quote!(#ty)),
            };
            return WrapperPlan {
                abi: quote!(),
                ret: quote!(-> ::core::result::Result<#ty, nv_loader::CallError>),
                on_missing: quote!(::core::result::Result::Err(nv_loader::CallError::symbol(
                    #lib, stringify!(#name), #source,
                ))),
                on_not_loaded: quote!(::core::result::Result::Err(nv_loader::CallError::Load(
                    __nv_error
                ))),
                mode,
            };
        }
    };
    WrapperPlan {
        abi: quote!(extern "C"),
        ret: native_ret,
        on_missing,
        on_not_loaded,
        mode: WrapMode::Native,
    }
}

/// Keep C parameter names intact; unnamed parameters receive deterministic private names.
fn shim_args(bare: &TypeBareFn) -> Vec<(proc_macro2::Ident, TokenStream)> {
    bare.inputs
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            let name = arg
                .name
                .as_ref()
                .map(|(id, _)| id.clone())
                .unwrap_or_else(|| format_ident!("arg_{i}"));
            let ty = &arg.ty;
            (name.clone(), quote!(#name: #ty))
        })
        .collect()
}

/// Panic-free functions for the function-pointer fields of `api_type`.
///
/// Policies and pointers preserve the native C ABI. Functions without a native failure
/// value use their original name with a Rust ABI and return `Result<T, nv_loader::CallError>`.
/// They remain in [`Shims::unshimmed`] because no native C ABI shim exists.
pub fn shims(
    lib: &'static str,
    api_source: &str,
    api_type: &str,
    loader_fn: &str,
    returns: &[ReturnPolicy<'_>],
) -> Result<Shims, Error> {
    let gen_err = |msg: String| Error::Generated { lib, msg };
    let file = syn::parse_file(api_source).map_err(|e| gen_err(e.to_string()))?;
    let api =
        find_struct(&file, api_type).ok_or_else(|| gen_err(format!("no struct {api_type}")))?;
    let loader = format_ident!("{loader_fn}");
    let policies = parse_return_policies(lib, returns)?;

    let Fields::Named(fields) = &api.fields else {
        return Err(gen_err(format!("{api_type} has no named fields")));
    };
    let mut unshimmed = Vec::new();
    let fns: Vec<TokenStream> = fields
        .named
        .iter()
        .filter_map(|field| {
            let name = field.ident.as_ref()?;
            let bare = result_bare_fn(&field.ty)?;
            let fb = fallback(&bare.output, &policies);
            if fb.is_none() {
                unshimmed.push(name.to_string());
            }
            let plan = wrapper_plan(&bare.output, fb, lib, name, quote!(Some(__nv_error)));
            let WrapperPlan {
                abi,
                ret,
                on_missing,
                on_not_loaded,
                mode,
            } = plan;
            let args = shim_args(bare);
            let names = args.iter().map(|(n, _)| n);
            let defs = args.iter().map(|(_, d)| d);
            let result = mode.wrap_call(quote!(unsafe { (*__nv_fn)(#(#names),*) }));
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
        })
        .collect();
    let tokens = quote!(#(#fns)*);
    let parsed: syn::File = syn::parse2(tokens).map_err(|e| gen_err(format!("shims: {e}")))?;
    unshimmed.sort();
    let source = document_unsafe_blocks(
        &prettyplease::unparse(&parsed),
        "the caller upholds the native contract and the pointer's library remains loaded for the process.",
    );
    Ok(Shims { source, unshimmed })
}

/// Sorted entry-point names: the function-pointer fields of `struct_name` in `source` (bindgen's
/// dynamic-loading struct wraps them in `Result<fn, _>`, a function table holds `Option<fn>`).
pub fn inventory(lib: &'static str, source: &str, struct_name: &str) -> Result<Vec<String>, Error> {
    let file = syn::parse_file(source).map_err(|e| Error::Generated {
        lib,
        msg: e.to_string(),
    })?;
    let s = find_struct(&file, struct_name).ok_or_else(|| Error::Generated {
        lib,
        msg: format!("no struct {struct_name}"),
    })?;
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
            Item::Type(t) if option_bare_fn(&t.ty).is_some() || result_bare_fn(&t.ty).is_some() => {
                Some(t.ident.to_string())
            }
            _ => None,
        })
        .collect();
    let is_fn_alias = |ty: &Type| match ty {
        Type::Path(p) if p.path.segments.len() == 1 => fn_aliases
            .iter()
            .any(|a| p.path.segments[0].ident == a.as_str()),
        _ => false,
    };
    let mut names: Vec<String> = fields
        .named
        .iter()
        .filter(|f| {
            result_bare_fn(&f.ty).is_some() || option_bare_fn(&f.ty).is_some() || is_fn_alias(&f.ty)
        })
        .filter_map(|f| f.ident.as_ref().map(ToString::to_string))
        .collect();
    names.sort();
    Ok(names)
}

/// Every single-segment type name used in the entry-point signatures of `api_type` (bindgen's
/// own aliases such as `cudaStream_t`; `::std::os::raw::*` paths have several segments and are
/// skipped), sorted and deduplicated.
pub fn signature_types(
    lib: &'static str,
    api_source: &str,
    api_type: &str,
) -> Result<Vec<String>, Error> {
    let file = syn::parse_file(api_source).map_err(|e| Error::Generated {
        lib,
        msg: e.to_string(),
    })?;
    let s = find_struct(&file, api_type).ok_or_else(|| Error::Generated {
        lib,
        msg: format!("no struct {api_type}"),
    })?;
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
    let PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };
    match args.args.first() {
        Some(GenericArgument::Type(Type::BareFn(f))) => Some(f),
        _ => None,
    }
}

/// Pins each of `enums` to `c_uint`; an entry that names no `c_int`/`c_uint` typedef in `source` is
/// a recipe error (a typo, a renamed typedef or a changed bindgen spelling), not a silent no-op.
fn normalize_positive_enums(
    lib: &'static str,
    source: &str,
    enums: &[&str],
) -> Result<String, Error> {
    let mut pinned_any = vec![false; enums.len()];
    let mut out: String = source
        .lines()
        .map(|line| {
            let pinned = enums.iter().position(|e| {
                line == format!("pub type {e} = ::std::os::raw::c_int;")
                    || line == format!("pub type {e} = ::std::os::raw::c_uint;")
            });
            match (pinned, line.split_once(" = ")) {
                (Some(i), Some((lhs, _))) => {
                    pinned_any[i] = true;
                    format!("{lhs} = ::core::ffi::c_uint;")
                }
                _ => line.to_string(),
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    out.push('\n');
    let missing: Vec<&str> = enums
        .iter()
        .zip(&pinned_any)
        .filter(|(_, pinned)| !**pinned)
        .map(|(e, _)| *e)
        .collect();
    if missing.is_empty() {
        Ok(out)
    } else {
        Err(Error::Generated {
            lib,
            msg: format!(
                "positive_enums {missing:?} name no `pub type X = c_int/c_uint` typedef in the generated types"
            ),
        })
    }
}

fn find_header(dirs: &[PathBuf], name: &str) -> Option<PathBuf> {
    dirs.iter().map(|d| d.join(name)).find(|p| p.is_file())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Environment variables whose change must re-run generation.
///
/// vcv-rs reads `PATH` (an `nvcc` on it) only when no CUDA root variable is set, so `PATH` is an
/// input only then: with `CUDA_PATH` set (the CUDA installer sets it), switching shells no longer
/// regenerates every CUDA-based sys crate. Where the selected toolkit's headers change, their
/// `rerun-if-changed` lines cover it.
fn rerun_vars(spec: &LibSpec, cuda_root_requested: bool) -> Vec<&'static str> {
    let mut vars = Vec::new();
    let cuda = |vars: &mut Vec<&'static str>| {
        vars.extend(vcv_rs::cuda::root_vars());
        vars.push("ProgramFiles");
        if !cuda_root_requested {
            vars.push("PATH");
        }
    };
    for sdk in spec.sdks {
        match sdk {
            Sdk::CudaToolkit | Sdk::CudaToolkitSubdir(_) => cuda(&mut vars),
            Sdk::Optix => {
                vars.extend(vcv_rs::optix::root_vars());
                vars.push("ProgramData");
            }
            Sdk::EnvRoot(v) | Sdk::EnvRootSubdir { var: v, .. } => vars.push(v),
            Sdk::Nv(kind) => {
                vars.extend(kind.root_vars());
                // vcv-rs picks an SDK build matching the detected CUDA toolkit.
                cuda(&mut vars);
                vars.extend(["NV_SDK_DIRS", "ProgramData"]);
            }
            Sdk::CrateDir(_) => {}
        }
    }
    vars.sort_unstable();
    vars.dedup();
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
        _ => Err(Error::InvalidSdkPath {
            lib,
            path: path.to_path_buf(),
        }),
    }
}

fn sdk_target_arch(lib: &'static str, requested: Option<&OsStr>) -> Result<vcv_rs::Arch, Error> {
    match requested {
        Some(value) => value
            .to_str()
            .and_then(vcv_rs::Arch::from_target_arch)
            .ok_or_else(|| Error::UnsupportedTarget {
                lib,
                arch: value.to_string_lossy().into_owned(),
            }),
        None => vcv_rs::Arch::native().ok_or_else(|| Error::UnsupportedTarget {
            lib,
            arch: std::env::consts::ARCH.to_string(),
        }),
    }
}

/// Locate every SDK once; consumers must not rediscover a different runtime after binding.
fn sdk_paths(spec: &LibSpec) -> Result<SdkPaths, Error> {
    let mut dirs = Vec::new();
    let mut nvidia = Vec::new();
    let cargo_arch = std::env::var_os("CARGO_CFG_TARGET_ARCH");
    let cross = is_cross_os();
    for sdk in spec.sdks {
        let dir = match sdk {
            Sdk::CudaToolkit if cross => {
                cross_root(spec.name, "CUDA Toolkit", vcv_rs::cuda::root_vars())?.join("include")
            }
            Sdk::CudaToolkitSubdir(rel) if cross => {
                cross_root(spec.name, "CUDA Toolkit", vcv_rs::cuda::root_vars())?.join(rel)
            }
            Sdk::Nv(kind) if cross => {
                cross_include_dir(&cross_root(spec.name, kind.name(), kind.root_vars())?, *kind)
            }
            Sdk::CudaToolkit => {
                vcv_rs::detect_cuda()
                    .map(|c| c.include_dir())
                    .ok_or_else(|| Error::SdkNotFound {
                        lib: spec.name,
                        sdk: "CUDA Toolkit".into(),
                        hint: "set CUDA_PATH or install the toolkit".into(),
                    })?
            }
            Sdk::Nv(kind) => {
                let target = sdk_target_arch(spec.name, cargo_arch.as_deref())?;
                let info =
                    vcv_rs::nvsdk::detect(*kind, target).ok_or_else(|| Error::SdkNotFound {
                        lib: spec.name,
                        sdk: kind.name().into(),
                        hint: match kind.requested_root() {
                            Some(request) => format!(
                                "configured {}={:?} is not usable for {target}",
                                request.variable, request.path
                            ),
                            None => format!(
                                "check {:?} or NV_SDK_DIRS for a native SDK on {}",
                                kind.root_vars(),
                                std::env::consts::OS
                            ),
                        },
                    })?;
                for path in [&info.runtime_dir, &info.lib_dir].into_iter().flatten() {
                    sdk_path_str(spec.name, path)?;
                }
                let include = info.include_dir.clone();
                nvidia.push(info);
                include
            }
            Sdk::CudaToolkitSubdir(rel) => vcv_rs::detect_cuda()
                .map(|c| c.root.join(rel))
                .ok_or_else(|| Error::SdkNotFound {
                    lib: spec.name,
                    sdk: "CUDA Toolkit".into(),
                    hint: "set CUDA_PATH or install the toolkit".into(),
                })?,
            Sdk::Optix => vcv_rs::optix::detect_optix()
                .map(|o| o.include_dir())
                .ok_or_else(|| Error::SdkNotFound {
                    lib: spec.name,
                    sdk: "OptiX SDK".into(),
                    hint: "set OPTIX_ROOT or install the OptiX SDK".into(),
                })?,
            Sdk::EnvRoot(var) => std::env::var_os(var)
                .filter(|v| !v.is_empty())
                .map(|root| Path::new(&root).join("include"))
                .ok_or_else(|| Error::SdkNotFound {
                    lib: spec.name,
                    sdk: (*var).to_string(),
                    hint: format!("set {var}"),
                })?,
            Sdk::EnvRootSubdir { var, subdir } => std::env::var_os(var)
                .filter(|v| !v.is_empty())
                .map(|root| Path::new(&root).join(subdir))
                .ok_or_else(|| Error::SdkNotFound {
                    lib: spec.name,
                    sdk: (*var).to_string(),
                    hint: format!("set {var}"),
                })?,
            Sdk::CrateDir(rel) => std::env::var_os("CARGO_MANIFEST_DIR")
                .map(|m| Path::new(&m).join(rel))
                .ok_or_else(|| Error::SdkNotFound {
                    lib: spec.name,
                    sdk: (*rel).to_string(),
                    hint: "CARGO_MANIFEST_DIR unset".into(),
                })?,
        };
        sdk_path_str(spec.name, &dir)?;
        if !dirs.contains(&dir) {
            dirs.push(dir);
        }
    }
    Ok(SdkPaths {
        includes: dirs,
        nvidia,
    })
}

/// Whether the Cargo target's OS differs from the OS running the build script (a cross build).
/// False outside a build script, where Cargo's target variables are unset.
fn is_cross_os() -> bool {
    std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os != std::env::consts::OS)
}

/// The SDK root a cross build names explicitly through one of `vars` (vcv-rs discovers SDKs of the
/// host OS only), as a `SdkNotFound` naming the variables when none is set.
fn cross_root(lib: &'static str, sdk: &str, vars: &[&str]) -> Result<PathBuf, Error> {
    vars.iter()
        .find_map(|v| std::env::var_os(v).filter(|x| !x.is_empty()))
        .map(PathBuf::from)
        .ok_or_else(|| Error::SdkNotFound {
            lib,
            sdk: sdk.to_string(),
            hint: format!(
                "cross build: SDK discovery covers the host OS only, set one of {vars:?} to the target SDK root"
            ),
        })
}

/// The header directory of a cross build's explicit SDK root: the first existing layout of that
/// SDK (vcv-rs keeps its own table private; these are the published SDK layouts: the Video Codec
/// SDK ships `Interface/`, the Optical Flow SDK `NvOFInterface/`, NVAPI headers at the root).
fn cross_include_dir(root: &Path, kind: vcv_rs::nvsdk::Kind) -> PathBuf {
    use vcv_rs::nvsdk::Kind;
    let subdirs: &[&str] = match kind {
        Kind::VideoCodec => &["Interface", "include"],
        Kind::OpticalFlow => &["NvOFInterface", "include"],
        Kind::Nvapi => &[""],
        _ => &["include"],
    };
    subdirs
        .iter()
        .map(|sub| root.join(sub))
        .find(|dir| dir.is_dir())
        .unwrap_or_else(|| root.join("include"))
}

/// Whether the Cargo target uses the MSVC/Windows SDK headers. Outside a build script (unit
/// tests) the host decides.
fn targets_msvc() -> bool {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| std::env::consts::OS.into());
    let env = std::env::var("CARGO_CFG_TARGET_ENV")
        .unwrap_or_else(|_| if cfg!(target_env = "msvc") { "msvc" } else { "" }.into());
    is_msvc_target(&os, &env)
}

fn is_msvc_target(os: &str, env: &str) -> bool {
    os == "windows" && env == "msvc"
}

/// MSVC + Windows SDK + UCRT include directories from vcv-rs, so libclang finds `stddef.h` & co.
/// from any shell, for the host running the build and the Cargo target's architecture. Only
/// MSVC targets get them: any other target gets none, and a cross build supplies its sysroot
/// through `BINDGEN_EXTRA_CLANG_ARGS` / `BINDGEN_EXTRA_CLANG_ARGS_<target>`, which bindgen reads
/// itself. Empty when no Visual Studio is installed (libclang then uses its own search).
fn system_include_dirs(lib: &'static str) -> Result<Vec<PathBuf>, Error> {
    if !targets_msvc() {
        return Ok(Vec::new());
    }
    #[cfg(windows)]
    {
        let (host, target) = msvc_arches(lib, std::env::var_os("CARGO_CFG_TARGET_ARCH").as_deref())?;
        let Some(vs) = vcv_rs::detect::detect_vs(None) else {
            return Ok(Vec::new());
        };
        let sdk = vcv_rs::detect::detect_sdk();
        let ucrt = vcv_rs::detect::detect_ucrt();
        Ok(vcv_rs::env::build_env(&vs, sdk.as_ref(), ucrt.as_ref(), host, target).include)
    }
    #[cfg(not(windows))]
    {
        // The MSVC toolset pair is a Windows-only input.
        let _ = lib;
        Ok(Vec::new())
    }
}

/// The MSVC toolset (host, target) pair: the build host's architecture and the Cargo target's
/// (`CARGO_CFG_TARGET_ARCH`, as [`sdk_target_arch`] reads it).
#[cfg(any(windows, test))]
fn msvc_arches(
    lib: &'static str,
    requested: Option<&OsStr>,
) -> Result<(vcv_rs::Arch, vcv_rs::Arch), Error> {
    let host = vcv_rs::Arch::native().ok_or_else(|| Error::UnsupportedTarget {
        lib,
        arch: std::env::consts::ARCH.to_string(),
    })?;
    Ok((host, sdk_target_arch(lib, requested)?))
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

    fn system_includes() -> Vec<PathBuf> {
        system_include_dirs("fake").expect("the test host is a supported architecture")
    }

    fn fixture() -> Vec<PathBuf> {
        vec![
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("fixture"),
        ]
    }

    #[test]
    fn bindings_reject_paths_before_clang_or_header_lookup() {
        for bad in ["SDK\ninclude", "SDK\rinclude", "SDK\0include"] {
            let paths = [PathBuf::from(bad)];
            for (includes, system) in [(paths.as_slice(), &[][..]), (&[][..], paths.as_slice())] {
                assert!(matches!(
                    bindings(&FAKE, includes, system),
                    Err(Error::InvalidSdkPath { .. })
                ));
            }
        }
    }

    #[test]
    fn sdk_rebuild_inputs_follow_discovery_aliases() {
        let spec = LibSpec {
            sdks: &[Sdk::CudaToolkit, Sdk::Optix, Sdk::Nv(Kind::Nvcomp)],
            ..FAKE
        };
        let vars = rerun_vars(&spec, false);
        for input in vcv_rs::cuda::root_vars()
            .iter()
            .chain(vcv_rs::optix::root_vars())
            .chain(Kind::Nvcomp.root_vars())
            .copied()
            .chain(["NV_SDK_DIRS", "PATH", "ProgramFiles", "ProgramData"])
        {
            assert!(
                vars.contains(&input),
                "discovery input {input} must rerun generation"
            );
        }
    }

    #[test]
    fn path_reruns_generation_only_when_no_cuda_root_is_set() {
        // L-17: with CUDA_PATH set, vcv-rs never reads PATH, so a PATH change must not regenerate.
        let spec = LibSpec {
            sdks: &[Sdk::CudaToolkit, Sdk::Nv(Kind::Cudnn)],
            ..FAKE
        };
        assert!(!rerun_vars(&spec, true).contains(&"PATH"));
        assert!(rerun_vars(&spec, true).contains(&"CUDA_PATH"));
        assert!(rerun_vars(&spec, false).contains(&"PATH"));
    }

    #[test]
    fn msvc_include_toolset_follows_the_cargo_target() {
        // L-35: the target half was hard-coded to x64.
        let (host, target) =
            msvc_arches("fake", Some(OsStr::new("aarch64"))).expect("supported target");
        assert_eq!(target, vcv_rs::Arch::Arm64);
        assert_eq!(host, vcv_rs::Arch::native().expect("supported test host"));
        assert!(matches!(
            msvc_arches("fake", Some(OsStr::new("riscv64"))),
            Err(Error::UnsupportedTarget { .. })
        ));
    }

    #[test]
    fn a_libclang_probe_panic_keeps_its_message() {
        // L-01: the payload used to be dropped and replaced by "libclang is unavailable".
        assert_eq!(catch_quietly(|| {}), Ok(()));
        let err = catch_quietly(|| panic!("Unable to find libclang: no DLL")).expect_err("panics");
        assert_eq!(err, "Unable to find libclang: no DLL");
        let err = catch_quietly(|| std::panic::panic_any(7_u8)).expect_err("panics");
        assert_eq!(err, "a panic without a message");
    }

    #[test]
    fn header_drift_names_changed_and_missing_headers() {
        // L-02: prebuilt bindings used beside an installed SDK are compared header by header.
        let bytes = std::fs::read(fixture()[0].join("fake.h")).expect("fixture");
        let inventory = |headers: Vec<HeaderHash>| Inventory {
            lib: "fake".into(),
            headers,
            symbols: Vec::new(),
            unshimmed: Vec::new(),
        };
        let same = inventory(vec![HeaderHash {
            name: "fake.h".into(),
            sha256: hex(&Sha256::digest(&bytes)),
        }]);
        assert!(header_drift(&fixture(), &same).is_empty());
        let drifted = inventory(vec![
            HeaderHash {
                name: "fake.h".into(),
                sha256: "0".repeat(64),
            },
            HeaderHash {
                name: "absent.h".into(),
                sha256: "0".repeat(64),
            },
        ]);
        assert_eq!(
            header_drift(&fixture(), &drifted),
            ["fake.h (changed)", "absent.h (missing)"]
        );
    }

    #[test]
    fn a_return_policy_that_matches_no_entry_point_is_rejected() {
        // L-05: cufile's `ssize_t` policy never matched (bindgen emits `isize`) and nothing said so.
        let api = r#"pub struct Api { pub read: Result<unsafe extern "C" fn() -> isize, ::libloading::Error> }"#;
        let policy = ReturnPolicy {
            types: &["ssize_t"],
            not_loaded: "-1",
            missing_symbol: "-2",
        };
        let err = check_return_policies_used("cufile", &[policy], &[(api, "Api")])
            .expect_err("ssize_t is never a generated return type");
        assert!(err.to_string().contains("ssize_t"), "{err}");
        let isize_policy = ReturnPolicy {
            types: &["isize"],
            ..policy
        };
        check_return_policies_used("cufile", &[isize_policy], &[(api, "Api")])
            .expect("isize matches");
    }

    #[test]
    fn split_functions_no_part_claims_are_listed_unshimmed() {
        // L-21: a header no part lists used to drop out of both `symbols` and `unshimmed`.
        const ONLY_A: &[ExportPart] = &[ExportPart {
            name: "a",
            api_type: "SplitAApi",
            loader_fn: "split_a_api",
            header_files: &["split_a.h"],
        }];
        let spec = LibSpec {
            name: "split",
            headers: &["split.h"],
            allow_fn: &["^split.*"],
            allow_type: &["^split.*"],
            allow_var: &[],
            positive_enums: &[],
            shape: Shape::SplitExports {
                parts: ONLY_A,
                returns: &[ReturnPolicy {
                    types: &["splitStatus"],
                    not_loaded: "splitStatus_enum_SPLIT_NOT_LOADED",
                    missing_symbol: "splitStatus_enum_SPLIT_MISSING",
                }],
            },
            ..FAKE
        };
        let out = bindings(&spec, &fixture(), &system_includes()).expect("split fixture binds");
        assert_eq!(out.inventory.symbols, ["splitAlpha", "splitAlphaToo"]);
        assert_eq!(out.inventory.unshimmed, ["splitBeta"]);
    }

    #[test]
    fn generation_reports_the_sub_headers_libclang_opened() {
        // L-16: only `split.h` was a rebuild input although the API lives in its sub-headers.
        const PARTS: &[ExportPart] = &[
            ExportPart {
                name: "a",
                api_type: "SplitAApi",
                loader_fn: "split_a_api",
                header_files: &["split_a.h"],
            },
            ExportPart {
                name: "b",
                api_type: "SplitBApi",
                loader_fn: "split_b_api",
                header_files: &["split_b.h"],
            },
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
        let out = bindings(&spec, &fixture(), &system_includes()).expect("split fixture binds");
        let names: Vec<String> = out
            .includes
            .iter()
            .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
            .collect();
        for header in ["split_a.h", "split_b.h", "split_common.h"] {
            assert!(names.iter().any(|n| n == header), "{header} missing from {names:?}");
        }
        assert!(
            out.includes.iter().all(|p| p.starts_with(&fixture()[0])),
            "only files under the SDK include directories are rebuild inputs: {:?}",
            out.includes
        );
        assert!(out.inventory.unshimmed.is_empty(), "every function has a part");
    }

    #[test]
    fn sdk_target_selection_is_explicit_and_checked() {
        for (name, expected) in [
            ("x86_64", vcv_rs::Arch::X64),
            ("x86", vcv_rs::Arch::X86),
            ("aarch64", vcv_rs::Arch::Arm64),
        ] {
            assert_eq!(
                sdk_target_arch("fake", Some(OsStr::new(name))).expect("supported target"),
                expected
            );
        }
        assert!(matches!(
            sdk_target_arch("fake", Some(OsStr::new("riscv64"))),
            Err(Error::UnsupportedTarget { .. })
        ));
        assert_eq!(
            sdk_target_arch("fake", None).expect("native target"),
            vcv_rs::Arch::native().expect("supported test host")
        );
    }

    #[test]
    fn sdk_paths_cannot_inject_cargo_instructions() {
        assert_eq!(
            sdk_path_str("fake", Path::new("SDK with spaces/include")).expect("ordinary path"),
            "SDK with spaces/include"
        );
        for bad in ["SDK\ninclude", "SDK\rinclude", "SDK\0include"] {
            assert!(matches!(
                sdk_path_str("fake", Path::new(bad)),
                Err(Error::InvalidSdkPath { .. })
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            assert!(matches!(
                sdk_path_str("fake", Path::new(OsStr::from_bytes(b"SDK\xff/include"))),
                Err(Error::InvalidSdkPath { .. })
            ));
        }
    }

    fn fn_slice<'a>(shims: &'a str, name: &str) -> &'a str {
        let marker = format!("pub unsafe fn {name}(");
        let start = shims
            .find(&marker)
            .unwrap_or_else(|| panic!("missing {marker}"));
        let tail = &shims[start..];
        let end = tail.find("#[allow(").unwrap_or(tail.len());
        &tail[..end]
    }

    #[test]
    fn exports_produce_shims_for_allowlisted_functions_only() {
        let out = bindings(&FAKE, &fixture(), &system_includes()).expect("fixture binds");
        assert_eq!(
            out.inventory.symbols,
            [
                "fakeCtxCreate",
                "fakeErrorString",
                "fakeGetCount",
                "fakeInit",
                "fakeIo",
                "fakeMissingNumber",
                "fakeReset",
                "fakeShadow",
                "fakeStore",
                "fakeTransfer",
                "fakeVersionNumber"
            ]
        );
        // These have checked Rust wrappers, never native C shims with made-up return values.
        assert_eq!(
            out.inventory.unshimmed,
            [
                "fakeMissingNumber",
                "fakeReset",
                "fakeStore",
                "fakeVersionNumber"
            ]
        );
        assert_eq!(out.parts.len(), 1, "a single-DLL library has one part");
        let shims = &out.parts[0].shims;
        for f in [
            "fakeInit",
            "fakeCtxCreate",
            "fakeGetCount",
            "fakeErrorString",
        ] {
            assert!(
                shims.contains(&format!("pub unsafe extern \"C\" fn {f}(")),
                "shim for {f}"
            );
        }
        assert!(shims.contains("pub unsafe fn fakeVersionNumber("));
        assert!(shims.contains("pub unsafe fn fakeReset("));
        assert!(shims.contains("pub unsafe fn fakeStore("));
        assert!(!shims.contains("pub unsafe extern \"C\" fn fakeVersionNumber("));
        for void_fn in ["fakeReset", "fakeStore"] {
            let body = fn_slice(shims, void_fn);
            assert!(
                !body.contains("::core::result::Result::Ok(unsafe"),
                "{void_fn} must not pass unit into Ok(...)"
            );
            assert!(body.contains("::core::result::Result::Ok(())"), "{body}");
        }
        // A pointer return falls back to null, not to a status code of the wrong type.
        assert!(shims.contains("::core::ptr::null()"), "{shims}");
        assert!(
            !shims.contains("notFakeHelper"),
            "allowlist must exclude other functions"
        );
        // Mutation that reddens this: drop either error arm, or let a shim fall back to a panic.
        assert!(shims.contains("fakeStatus_enum_FAKE_ERROR_NOT_LOADED"));
        assert!(shims.contains("fakeStatus_enum_FAKE_ERROR_MISSING_SYMBOL"));
        assert!(!shims.contains("expect") && !shims.contains("unwrap") && !shims.contains("panic"));
    }

    #[test]
    fn types_exclude_functions_and_pin_positive_enums() {
        let out = bindings(&FAKE, &fixture(), &system_includes()).expect("fixture binds");
        assert!(
            out.types
                .contains("pub type fakeStatus_enum = ::core::ffi::c_uint;"),
            "{}",
            out.types
        );
        assert!(out.types.contains("FAKE_VERSION"));
        assert!(
            !out.types.contains("pub fn fakeInit"),
            "types pass must not declare functions"
        );
    }

    #[test]
    fn positive_enums_normalize_both_msvc_and_gcc_underlying_types() {
        let raw = "\
pub type winPos = ::std::os::raw::c_int;\n\
pub type gccPos = ::std::os::raw::c_uint;\n\
pub type signedErr = ::std::os::raw::c_int;\n";
        let normalized =
            normalize_positive_enums("t", raw, &["winPos", "gccPos"]).expect("both are typedefs");
        assert!(normalized.contains("pub type winPos = ::core::ffi::c_uint;\n"));
        assert!(normalized.contains("pub type gccPos = ::core::ffi::c_uint;\n"));
        assert!(normalized.contains("pub type signedErr = ::std::os::raw::c_int;\n"));
    }

    #[test]
    fn a_positive_enum_that_matches_no_typedef_is_rejected() {
        // L-20: an entry that matched nothing used to be skipped silently.
        let raw = "pub type realEnum = ::std::os::raw::c_int;
";
        let err = normalize_positive_enums("t", raw, &["realEnum", "renamedEnum"])
            .expect_err("renamedEnum is not in the source");
        assert!(err.to_string().contains("renamedEnum"), "{err}");
        assert!(!err.to_string().contains("\"realEnum\""), "{err}");
    }

    #[test]
    fn inventory_hashes_the_header_bytes() {
        let out = bindings(&FAKE, &fixture(), &system_includes()).expect("fixture binds");
        let bytes = std::fs::read(fixture()[0].join("fake.h")).expect("fixture");
        assert_eq!(
            out.inventory.headers,
            vec![HeaderHash {
                name: "fake.h".into(),
                sha256: hex(&Sha256::digest(&bytes))
            }]
        );
    }

    #[test]
    fn a_missing_header_names_every_searched_directory() {
        let spec = LibSpec {
            headers: &["absent.h"],
            ..FAKE
        };
        let err = bindings(&spec, &fixture(), &[]).expect_err("no such header");
        assert!(
            matches!(err, Error::HeaderNotFound { ref searched, .. } if searched == &fixture()),
            "{err}"
        );
    }

    #[test]
    fn split_exports_give_each_dll_only_its_own_headers_functions() {
        const PARTS: &[ExportPart] = &[
            ExportPart {
                name: "a",
                api_type: "SplitAApi",
                loader_fn: "split_a_api",
                header_files: &["split_a.h"],
            },
            ExportPart {
                name: "b",
                api_type: "SplitBApi",
                loader_fn: "split_b_api",
                header_files: &["split_b.h"],
            },
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
        let out = bindings(&spec, &fixture(), &system_includes()).expect("split fixture binds");
        assert_eq!(
            out.inventory.symbols,
            ["splitAlpha", "splitAlphaToo", "splitBeta"]
        );
        let stems: Vec<&str> = out.parts.iter().map(|p| p.stem.as_str()).collect();
        assert_eq!(stems, ["_a", "_b"]);
        // Mutation that reddens this: apply `allow_fn` to split parts (bindgen allowlists are a
        // union, so every part would get every function).
        assert!(
            out.parts[0].shims.contains("fn splitAlpha(")
                && !out.parts[0].shims.contains("fn splitBeta(")
        );
        assert!(
            out.parts[1].shims.contains("fn splitBeta(")
                && !out.parts[1].shims.contains("fn splitAlpha(")
        );
        assert!(
            out.parts[0].shims.contains("split_a_api()")
                && out.parts[1].shims.contains("split_b_api()")
        );
        assert!(
            out.types.contains("splitStatus_enum_SPLIT_SUCCESS"),
            "shared types are generated once"
        );
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
        let out = bindings(&spec, &fixture(), &system_includes()).expect("qi fixture binds");
        assert_eq!(
            out.inventory.symbols,
            [
                "qiAbsentNumber",
                "qiGetValue",
                "qiInitialize",
                "qiMissingStore",
                "qiStore",
                "qiVersionNumber"
            ]
        );
        // Checked numeric/void functions plus the declaration lacking a query id have no C shim.
        assert_eq!(
            out.inventory.unshimmed,
            [
                "qiAbsentNumber",
                "qiMissingStore",
                "qiNotInTable",
                "qiStore",
                "qiVersionNumber"
            ]
        );
        let part = &out.parts[0];
        assert!(
            part.api.is_none(),
            "query-interface libraries need no loading struct"
        );
        assert!(
            part.shims.contains("0x150e828u32") || part.shims.contains("22079528u32"),
            "{}",
            part.shims
        );
        assert!(
            part.shims.contains("_qiStatus_QI_NO_IMPLEMENTATION")
                && part.shims.contains("_qiStatus_QI_LIBRARY_NOT_FOUND")
        );
        // L-30: the id is resolved once per shim, never per call.
        for f in ["qiStore", "qiMissingStore"] {
            let body = fn_slice(&part.shims, f);
            assert!(
                body.contains("OnceLock") && body.matches("(__nv_qi.query)").count() == 1,
                "{f} must cache its query result: {body}"
            );
        }
        for void_fn in ["qiStore", "qiMissingStore"] {
            let body = fn_slice(&part.shims, void_fn);
            assert!(
                !body.contains("::core::result::Result::Ok(unsafe"),
                "{void_fn} must not pass unit into Ok(...)"
            );
            assert!(body.contains("::core::result::Result::Ok(())"), "{body}");
        }
    }

    #[test]
    fn interface_ids_parse_the_table_form() {
        let text = "struct T t[] =\n{\n    { \"NvAPI_Initialize\", 0x0150e828 },\n    { \"NvAPI_Unload\", 0xd22bdd7e },\n};\n";
        assert_eq!(
            interface_ids(text),
            [
                ("NvAPI_Initialize".to_string(), 0x0150e828),
                ("NvAPI_Unload".to_string(), 0xd22bdd7e)
            ]
        );
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
            [
                "cudaStream_t",
                "cufftHandle",
                "cufftResult",
                "libraryPropertyType"
            ]
        );
    }

    fn assert_invalid_return_policies(returns: &[ReturnPolicy<'_>], expected: &str) {
        let api = r#"pub struct Api { pub value: Result<unsafe extern "C" fn() -> Status, ::libloading::Error> }"#;
        let error = shims("invalid", api, "Api", "loader", returns)
            .expect_err("invalid recipe must be rejected");
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
        assert_invalid_return_policies(
            &[ReturnPolicy {
                types: &[],
                not_loaded: "1",
                missing_symbol: "2",
            }],
            "no types",
        );
    }

    #[test]
    fn return_policies_reject_duplicate_type_names() {
        let policy = ReturnPolicy {
            types: &["Status"],
            not_loaded: "1",
            missing_symbol: "2",
        };
        assert_invalid_return_policies(&[policy, policy], "duplicate return policy for Status");
        assert_invalid_return_policies(
            &[ReturnPolicy {
                types: &["Status", "Status"],
                ..policy
            }],
            "duplicate return policy for Status",
        );
    }

    #[test]
    fn return_policies_reject_invalid_type_identifiers() {
        assert_invalid_return_policies(
            &[ReturnPolicy {
                types: &["Not::AnIdentifier"],
                not_loaded: "1",
                missing_symbol: "2",
            }],
            "return type",
        );
    }

    #[test]
    fn return_policies_reject_malformed_failure_expressions() {
        assert_invalid_return_policies(
            &[ReturnPolicy {
                types: &["Status"],
                not_loaded: "{",
                missing_symbol: "2",
            }],
            "not_loaded",
        );
        assert_invalid_return_policies(
            &[ReturnPolicy {
                types: &["Status"],
                not_loaded: "1",
                missing_symbol: "{",
            }],
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
        assert_eq!(
            inventory("t", src, "Table").expect("parses"),
            ["barDestroy", "bazOpen", "fooCreate"]
        );
    }

    #[test]
    fn expected_output_files_cover_every_shape() {
        assert_eq!(
            expected_output_files(&FAKE),
            [
                "fake_types.rs",
                "fake_inventory.json",
                "fake_api.rs",
                "fake_shims.rs"
            ]
        );
        const PARTS: &[ExportPart] = &[
            ExportPart {
                name: "a",
                api_type: "A",
                loader_fn: "a_api",
                header_files: &[],
            },
            ExportPart {
                name: "b",
                api_type: "B",
                loader_fn: "b_api",
                header_files: &[],
            },
        ];
        let split = LibSpec {
            name: "npp",
            shape: Shape::SplitExports {
                parts: PARTS,
                returns: &[],
            },
            ..FAKE
        };
        assert_eq!(
            expected_output_files(&split),
            [
                "npp_types.rs",
                "npp_inventory.json",
                "npp_a_api.rs",
                "npp_a_shims.rs",
                "npp_b_api.rs",
                "npp_b_shims.rs"
            ]
        );
        let qi = LibSpec {
            name: "nvapi",
            shape: Shape::QueryInterface {
                interface_header: "i.h",
                loader_fn: "q",
                returns: &[],
            },
            ..FAKE
        };
        assert_eq!(
            expected_output_files(&qi),
            ["nvapi_types.rs", "nvapi_inventory.json", "nvapi_shims.rs"]
        );
        let static_link = LibSpec {
            name: "ngx",
            shape: Shape::StaticLink {
                link_libs: &["ngx"],
                system_libs: &[],
            },
            ..FAKE
        };
        assert_eq!(
            expected_output_files(&static_link),
            ["ngx_types.rs", "ngx_inventory.json", "ngx_shims.rs"]
        );
        let table = LibSpec {
            name: "optix",
            shape: Shape::FunctionTable {
                table_type: "T",
                probes: &[],
            },
            ..FAKE
        };
        assert_eq!(
            expected_output_files(&table),
            ["optix_types.rs", "optix_inventory.json"]
        );
    }

    #[test]
    fn prebuilt_copy_and_resolution_work() {
        // L-37: a per-process directory, so concurrent runs (two targets, two checkouts) never race.
        let base = std::env::temp_dir().join(format!(
            "nv_bindgen_prebuilt_test_{}",
            std::process::id()
        ));
        std::fs::remove_dir_all(&base).ok();
        let target = "x86_64-pc-windows-msvc";
        let prebuilt = base.join("prebuilt").join(target);
        let out = base.join("out");
        std::fs::create_dir_all(&prebuilt).expect("prebuilt dir");
        std::fs::create_dir_all(&out).expect("out dir");

        let expected = expected_output_files(&FAKE);
        for f in &expected {
            let content = if f.ends_with("_inventory.json") {
                r#"{"lib":"fake","headers":[],"symbols":["fakeInit"],"unshimmed":[]}"#
            } else {
                "// prebuilt"
            };
            std::fs::write(prebuilt.join(f), content).expect("write prebuilt");
        }

        // Resolves <NV_RS_PREBUILT_DIR>/<target> when NV_RS_PREBUILT_DIR points to parent.
        let resolved = resolve_prebuilt_dir(
            Some(&base),
            Some(OsStr::new(prebuilt.parent().unwrap())),
            target,
            &expected,
        );
        assert_eq!(resolved.as_deref(), Some(prebuilt.as_path()));
        // Resolves <NV_RS_PREBUILT_DIR> directly when it already points to target dir.
        let resolved_direct =
            resolve_prebuilt_dir(Some(&base), Some(OsStr::new(&prebuilt)), target, &expected);
        assert_eq!(resolved_direct.as_deref(), Some(prebuilt.as_path()));
        // Resolves <CARGO_MANIFEST_DIR>/prebuilt/<target> when NV_RS_PREBUILT_DIR is unset.
        let resolved_default = resolve_prebuilt_dir(Some(&base), None, target, &expected);
        assert_eq!(resolved_default.as_deref(), Some(prebuilt.as_path()));
        // L-19: an NV_RS_PREBUILT_DIR without this library's files falls through to the crate's.
        let other_family = base.join("other_family");
        std::fs::create_dir_all(other_family.join(target)).expect("other prebuilt dir");
        let resolved_fallthrough = resolve_prebuilt_dir(
            Some(&base),
            Some(OsStr::new(&other_family)),
            target,
            &expected,
        );
        assert_eq!(resolved_fallthrough.as_deref(), Some(prebuilt.as_path()));

        let (sources, inv) = copy_prebuilt_bindings(&FAKE, &prebuilt, &out).expect("copy prebuilt");
        assert_eq!(sources.len(), expected.len());
        assert_eq!(inv.lib, "fake");
        assert_eq!(inv.symbols, ["fakeInit"]);
        for f in &expected {
            assert!(out.join(f).is_file(), "missing copied {f}");
        }

        // Missing file fails cleanly.
        std::fs::remove_file(prebuilt.join("fake_shims.rs")).expect("remove");
        assert!(copy_prebuilt_bindings(&FAKE, &prebuilt, &out).is_err());

        std::fs::remove_dir_all(&base).ok();
    }

    fn unique_dir(label: &str) -> PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "nv_bindgen_{label}_{}_{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).expect("temp dir");
        dir
    }

    #[test]
    fn manifest_requires_every_promised_file_and_records_the_origin() {
        let out = unique_dir("manifest");
        let promised = expected_output_files(&FAKE);
        // The generator forgot the last promised file: a hard error naming it, and no manifest.
        for f in &promised[..promised.len() - 1] {
            std::fs::write(out.join(f), "// x").expect("write");
        }
        let missing = promised.last().expect("promises files");
        let err = write_manifest(&FAKE, &out, Origin::Generated).expect_err("a file is missing");
        assert!(err.to_string().contains(missing.as_str()), "{err}");
        assert!(!out.join(Manifest::file_name("fake")).exists());

        std::fs::write(out.join(missing), "// x").expect("write");
        write_manifest(&FAKE, &out, Origin::Prebuilt).expect("complete set");
        let text = std::fs::read_to_string(out.join("fake_manifest.json")).expect("manifest");
        let manifest: Manifest = serde_json::from_str(&text).expect("manifest parses");
        assert_eq!(
            manifest,
            Manifest {
                lib: "fake".into(),
                origin: Origin::Prebuilt,
                files: promised,
            }
        );
        std::fs::remove_dir_all(&out).ok();
    }

    #[test]
    fn msvc_includes_belong_to_msvc_targets_only() {
        assert!(is_msvc_target("windows", "msvc"));
        assert!(!is_msvc_target("linux", "gnu"));
        assert!(!is_msvc_target("windows", "gnu"));
        assert!(!is_msvc_target("macos", ""));
    }

    #[test]
    fn a_cross_build_without_an_sdk_root_names_the_variables() {
        let err = cross_root("nccl", "NCCL", &["NV_RS_TEST_CROSS_ROOT_NEVER_SET"])
            .expect_err("the variable is unset");
        let Error::SdkNotFound { lib, hint, .. } = &err else {
            panic!("expected SdkNotFound, got {err}");
        };
        assert_eq!(*lib, "nccl");
        assert!(hint.contains("NV_RS_TEST_CROSS_ROOT_NEVER_SET"), "{hint}");
    }

    #[test]
    fn generated_shims_document_every_unsafe_block() {
        let sample = "fn a() {\n    let x = unsafe { foo() };\n    Ok(unsafe { bar() })\n}\n";
        let documented = document_unsafe_blocks(sample, "test invariant.");
        let lines: Vec<&str> = documented.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if line.contains("unsafe {") {
                assert!(
                    i > 0 && lines[i - 1].trim_start().starts_with("// SAFETY:"),
                    "missing SAFETY comment before line {i}: {line}"
                );
            }
        }
    }
}
