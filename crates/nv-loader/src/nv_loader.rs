//! Runtime discovery and loading of NVIDIA libraries — the one loader every nv-rs sys crate uses.
//!
//! **What.** A sys crate describes its library once as a [`DllSpec`] (which [`Group`] it ships in,
//! its Windows file names or version-prefixes, its Linux sonames). [`open_first`] walks the
//! candidates in priority order and returns the first one the caller's constructor accepts, or a
//! [`LoadError`] that lists every path tried and why each was rejected. [`check_version`] turns a
//! too-old runtime into a typed error instead of a later crash.
//!
//! **Why.** Twenty-odd NVIDIA libraries live in four different places (System32, the display
//! driver's DriverStore folder, the CUDA Toolkit, separately installed SDKs) and toolkit DLLs
//! carry per-library version suffixes (`cufft64_12.dll`, `nvrtc64_130_0.dll`). Without one loader
//! every sys crate re-implements that search and they disagree.
//!
//! **How.** Discovery reads the environment, the filesystem and (Windows) the registry through
//! the [`Probe`] trait, so the search logic is tested without touching the machine. The real
//! probe is [`SystemProbe`]. Driver-store libraries are found the way NVIDIA's own
//! `optix_stubs.h` finds `nvoptix.dll`: System32 first, then the `OpenGLDriverName` value of each
//! display-class adapter, whose directory is that driver's DriverStore folder.
//!
//! **Used by** every `*-sys` crate in nv-rs; generated shims call the sys crate's loader function,
//! which calls [`open_first`] once and caches the result.

use std::fmt;
use std::path::{Path, PathBuf};

/// Where a library ships, which decides where it is searched for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    /// Installed by the display driver into System32 (Windows) or the system library path
    /// (`nvml`, `nvEncodeAPI64`, `nvcuvid`, `nvofapi64`, `nvapi64`).
    Driver,
    /// Installed by the display driver into its DriverStore folder only (`nvoptix`, `nvngx`).
    DriverStore,
    /// Part of the CUDA Toolkit, found under the toolkit root (`cufft64_12`, `nvrtc64_130_0`).
    Toolkit,
    /// A separately installed SDK (cuDNN, nvCOMP, ...). Searched in, in order: the directory next
    /// to the running executable (how applications ship these DLLs), `<root_var>/bin[/x64]` and
    /// `<root_var>/lib`, then `hint` — the SDK's DLL directory vcv-rs found when the sys crate was
    /// built (`env!`-baked by nv-bindgen, `None` when the SDK had none).
    Sdk {
        /// The environment variable naming the SDK root.
        root_var: &'static str,
        /// The build machine's SDK DLL directory, if known.
        hint: Option<&'static str>,
    },
}

/// File-name rules for one library.
///
/// A Windows entry ending in `*` is a prefix: every matching `.dll` in a searched directory is a
/// candidate, highest name first (toolkit DLLs differ per library: `cublas64_13`, `cufft64_12`).
#[derive(Clone, Copy, Debug)]
pub struct DllSpec {
    /// Short library name used in errors (`"cufft"`).
    pub lib: &'static str,
    /// Where it ships.
    pub group: Group,
    /// Windows file names or `prefix*` patterns.
    pub windows: &'static [&'static str],
    /// Linux sonames, tried in order through the dynamic linker's own search.
    pub linux: &'static [&'static str],
}

/// Why a library could not be loaded.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    /// No candidate file exists.
    #[error("{lib}: not found (searched {})", show_paths(.tried))]
    NotFound {
        /// The library.
        lib: &'static str,
        /// Every path that was looked for.
        tried: Vec<PathBuf>,
    },
    /// Candidates exist but every one was rejected by the loader or the constructor.
    #[error("{lib}: every candidate was rejected: {}", show_failures(.failures))]
    Rejected {
        /// The library.
        lib: &'static str,
        /// Each candidate and why it failed.
        failures: Vec<(PathBuf, String)>,
    },
    /// The library loaded but reports a version older than the bindings require.
    #[error("{lib}: runtime version {found} is older than the {required} these bindings need")]
    RuntimeTooOld {
        /// The library.
        lib: &'static str,
        /// The minimum version the bindings were generated for.
        required: u32,
        /// The version the loaded library reports.
        found: u32,
    },
}

/// Why an entry point without a native failure sentinel could not be called.
///
/// Library causes are borrowed from the cached table. Symbol diagnostics are boxed only on
/// failure, keeping successful checked results compact and free of allocations.
#[derive(Debug, thiserror::Error)]
pub enum CallError {
    /// The library could not be loaded; preserves its original diagnostic.
    #[error("{0}")]
    Load(#[source] &'static LoadError),
    /// The library loaded but the requested entry point was unavailable.
    #[error(transparent)]
    Symbol(Box<SymbolError>),
}

/// Cold diagnostic for an unavailable native entry point.
#[derive(Debug, thiserror::Error)]
#[error("{lib}: symbol {symbol} is unavailable")]
pub struct SymbolError {
    /// The library's binding name.
    pub lib: &'static str,
    /// The exact native entry-point name.
    pub symbol: &'static str,
    /// The dynamic loader's cause, or `None` when a query interface returned null.
    #[source]
    pub source: Option<&'static libloading::Error>,
}

impl CallError {
    /// Constructs the boxed symbol diagnostic on a generated call's failure branch.
    pub fn symbol(lib: &'static str, symbol: &'static str, source: Option<&'static libloading::Error>) -> Self {
        Self::Symbol(Box::new(SymbolError { lib, symbol, source }))
    }
}

fn show_paths(paths: &[PathBuf]) -> String {
    paths.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ")
}

fn show_failures(failures: &[(PathBuf, String)]) -> String {
    failures.iter().map(|(p, e)| format!("{}: {e}", p.display())).collect::<Vec<_>>().join("; ")
}

/// Everything discovery reads from the machine. [`SystemProbe`] is the real one; tests supply a
/// fake so the search order is verified without the installed driver or toolkit.
pub trait Probe {
    /// An environment variable, `None` when unset or empty.
    fn env(&self, name: &str) -> Option<String>;
    /// Whether a file exists.
    fn is_file(&self, path: &Path) -> bool;
    /// The file names in a directory (empty when it does not exist).
    fn list_dir(&self, dir: &Path) -> Vec<String>;
    /// Directories of the display adapters' drivers (Windows: each display-class adapter's
    /// `OpenGLDriverName` parent; empty elsewhere).
    fn display_driver_dirs(&self) -> Vec<PathBuf>;
    /// The directory of the running executable, if known.
    fn exe_dir(&self) -> Option<PathBuf>;
}

/// The machine itself.
#[derive(Clone, Copy, Debug, Default)]
pub struct SystemProbe;

impl Probe for SystemProbe {
    fn env(&self, name: &str) -> Option<String> {
        std::env::var(name).ok().filter(|v| !v.trim().is_empty())
    }

    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn list_dir(&self, dir: &Path) -> Vec<String> {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return Vec::new();
        };
        entries.filter_map(|e| e.ok()).filter_map(|e| e.file_name().into_string().ok()).collect()
    }

    fn exe_dir(&self) -> Option<PathBuf> {
        std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf))
    }

    fn display_driver_dirs(&self) -> Vec<PathBuf> {
        #[cfg(windows)]
        {
            windows_display_driver_dirs()
        }
        #[cfg(not(windows))]
        {
            Vec::new()
        }
    }
}

/// Display-class adapters' driver folders, from `OpenGLDriverName` (REG_MULTI_SZ, the full path of
/// the adapter's OpenGL ICD inside its DriverStore folder). Same source `optix_stubs.h` uses.
#[cfg(windows)]
fn windows_display_driver_dirs() -> Vec<PathBuf> {
    use winreg::RegKey;
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

    const DISPLAY_CLASS: &str =
        r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}";
    let Ok(class) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(DISPLAY_CLASS, KEY_READ) else {
        return Vec::new();
    };
    let mut dirs: Vec<PathBuf> = Vec::new();
    for name in class.enum_keys().filter_map(|k| k.ok()) {
        // Adapter instances are the numbered subkeys (0000, 0001, ...); "Properties" is not one.
        if !name.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let Ok(adapter) = class.open_subkey_with_flags(&name, KEY_READ) else {
            continue;
        };
        let paths: Vec<String> = adapter
            .get_value::<Vec<String>, _>("OpenGLDriverName")
            .or_else(|_| adapter.get_value::<String, _>("OpenGLDriverName").map(|s| vec![s]))
            .unwrap_or_default();
        for p in paths {
            if let Some(dir) = Path::new(&p).parent().map(Path::to_path_buf)
                && !dirs.contains(&dir)
            {
                dirs.push(dir);
            }
        }
    }
    dirs
}

/// Candidate paths for `spec` on this machine, most preferred first.
pub fn candidates(spec: &DllSpec) -> Vec<PathBuf> {
    candidates_with(&SystemProbe, spec)
}

/// Candidate paths for `spec` as seen through `probe`, most preferred first.
///
/// On Linux an SDK's discovered runtime directory precedes bare sonames. The dynamic linker's
/// own search (`LD_LIBRARY_PATH`, `ld.so.cache`, `/usr/lib/wsl/lib` on WSL) follows that hint.
pub fn candidates_with(probe: &dyn Probe, spec: &DllSpec) -> Vec<PathBuf> {
    if !cfg!(windows) {
        return linux_candidates(spec);
    }
    let dirs = search_dirs(probe, spec.group);
    let mut out: Vec<PathBuf> = Vec::new();
    for dir in &dirs {
        for name in spec.windows {
            if let Some(prefix) = name.strip_suffix('*') {
                let mut matches: Vec<String> = probe
                    .list_dir(dir)
                    .into_iter()
                    .filter(|f| {
                        let lower = f.to_ascii_lowercase();
                        lower.starts_with(&prefix.to_ascii_lowercase()) && lower.ends_with(".dll")
                    })
                    .collect();
                // Highest version suffix first: `cufft64_12.dll` before `cufft64_11.dll`.
                matches.sort();
                matches.reverse();
                out.extend(matches.into_iter().map(|f| dir.join(f)));
            } else {
                out.push(dir.join(name));
            }
        }
    }
    out.dedup();
    out
}

/// Linux SDK hints come from the discovery owner; retain the dynamic linker's sonames after them.
fn linux_candidates(spec: &DllSpec) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Group::Sdk { hint: Some(dir), .. } = spec.group {
        out.extend(spec.linux.iter().map(|soname| Path::new(dir).join(soname)));
    }
    out.extend(spec.linux.iter().map(PathBuf::from));
    out
}

/// Directories searched for a group on Windows, most preferred first.
fn search_dirs(probe: &dyn Probe, group: Group) -> Vec<PathBuf> {
    let system32 = PathBuf::from(probe.env("SystemRoot").unwrap_or_else(|| r"C:\Windows".to_string())).join("System32");
    match group {
        Group::Driver => vec![system32],
        // NVIDIA's own order: System32 first (older drivers copied it there), then the adapters.
        Group::DriverStore => std::iter::once(system32).chain(probe.display_driver_dirs()).collect(),
        Group::Toolkit => {
            // The variables vcv-rs and cudarc honour, in the same order; CUDA 13 moved DLLs into
            // bin\x64 while 12.x keeps them in bin, so both are searched.
            let mut dirs = Vec::new();
            for var in ["CUDA_PATH", "CUDA_HOME", "CUDA_ROOT", "CUDA_TOOLKIT_ROOT_DIR"] {
                if let Some(root) = probe.env(var) {
                    let root = PathBuf::from(root);
                    // CUPTI ships its DLLs under extras\CUPTI\lib64, outside bin.
                    for dir in [root.join("bin").join("x64"), root.join("bin"), root.join("extras").join("CUPTI").join("lib64")] {
                        if !dirs.contains(&dir) {
                            dirs.push(dir);
                        }
                    }
                }
            }
            dirs
        }
        Group::Sdk { root_var, hint } => {
            let mut dirs: Vec<PathBuf> = probe.exe_dir().into_iter().collect();
            if let Some(root) = probe.env(root_var) {
                let root = PathBuf::from(root);
                dirs.extend([root.join("bin").join("x64"), root.join("bin"), root.join("lib")]);
            }
            dirs.extend(hint.map(PathBuf::from));
            dirs.dedup();
            dirs
        }
    }
}

/// Opens the first candidate for which `open` succeeds.
///
/// `open` receives a path (Windows: an existing file; Linux: an SDK path or linker soname) and
/// returns the loaded value or a reason it was rejected (wrong ABI, missing entry point, ...).
pub fn open_first<T>(spec: &DllSpec, open: impl FnMut(&Path) -> Result<T, String>) -> Result<Loaded<T>, LoadError> {
    open_first_with(&SystemProbe, spec, open)
}

/// [`open_first`] through an explicit [`Probe`].
pub fn open_first_with<T>(
    probe: &dyn Probe,
    spec: &DllSpec,
    mut open: impl FnMut(&Path) -> Result<T, String>,
) -> Result<Loaded<T>, LoadError> {
    let all = candidates_with(probe, spec);
    let present: Vec<PathBuf> = if cfg!(windows) {
        all.iter().filter(|p| probe.is_file(p)).cloned().collect()
    } else {
        all.clone()
    };
    if present.is_empty() {
        return Err(LoadError::NotFound { lib: spec.lib, tried: all });
    }
    let mut failures = Vec::new();
    for path in present {
        match open(&path) {
            Ok(value) => return Ok(Loaded { value, path }),
            Err(reason) => failures.push((path, reason)),
        }
    }
    Err(LoadError::Rejected { lib: spec.lib, failures })
}

/// A loaded library value and the file it came from (reported by diagnostics and evidence).
#[derive(Debug)]
pub struct Loaded<T> {
    /// The constructor's result.
    pub value: T,
    /// The file that produced it.
    pub path: PathBuf,
}

/// The body of an exports-shaped sys crate (one `nv-bindgen` [`Shape::Exports`] library).
///
/// Includes the generated `<lib>_types.rs`, `<lib>_api.rs` and `<lib>_shims.rs` from `OUT_DIR`,
/// re-exports types and shims, keeps the bindgen loading struct private (its methods `expect` a
/// symbol and would panic), and defines the cached loader the shims call plus a public
/// diagnostic `load()` and the `INVENTORY` evidence string. One definition for every sys crate.
///
/// [`Shape::Exports`]: https://docs.rs/nv-bindgen
#[macro_export]
macro_rules! exports_sys {
    (lib: $lib:literal, api: $api:ident, loader: $loader:ident, dll: $dll:expr $(,)?) => {
        // Generated code: lint levels differ per header, so the generated modules opt out.
        #[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code, missing_docs, unnecessary_transmutes, clippy::all, clippy::undocumented_unsafe_blocks)]
        mod types {
            include!(concat!(env!("OUT_DIR"), "/", $lib, "_types.rs"));
        }
        #[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code, missing_docs, unnecessary_transmutes, clippy::all, clippy::undocumented_unsafe_blocks)]
        mod api {
            use super::types::*;
            include!(concat!(env!("OUT_DIR"), "/", $lib, "_api.rs"));
        }
        // Shim bodies share one safety argument, documented once in nv-bindgen (`shims`);
        // prettyplease drops comments, so the per-block SAFETY lint cannot be met here.
        #[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, missing_docs, unnecessary_transmutes, clippy::all, clippy::undocumented_unsafe_blocks)]
        mod shims {
            use super::$loader;
            use super::types::*;
            include!(concat!(env!("OUT_DIR"), "/", $lib, "_shims.rs"));
        }
        pub use shims::*;
        pub use types::*;

        /// Where this library ships and what its files are called.
        pub const DLL: $crate::DllSpec = $dll;

        /// Entry points and header sha256 of the generated bindings (coverage evidence).
        pub const INVENTORY: &str = include_str!(concat!(env!("OUT_DIR"), "/", $lib, "_inventory.json"));

        fn loaded() -> &'static ::core::result::Result<$crate::Loaded<api::$api>, $crate::LoadError> {
            static API: ::std::sync::OnceLock<::core::result::Result<$crate::Loaded<api::$api>, $crate::LoadError>> =
                ::std::sync::OnceLock::new();
            API.get_or_init(|| {
                // SAFETY: opening the library runs only its loader initialisers; symbols are
                // resolved lazily and a missing one becomes an `Err` field, not a crash.
                $crate::open_first(&DLL, |p| unsafe { api::$api::new(p) }.map_err(|e| e.to_string()))
            })
        }

        /// The loaded library, for the shims.
        fn $loader() -> ::core::result::Result<&'static api::$api, &'static $crate::LoadError> {
            loaded().as_ref().map(|l| &l.value)
        }

        /// Loads the library now and reports which file was used, or why loading failed.
        pub fn load() -> ::core::result::Result<&'static ::std::path::Path, &'static $crate::LoadError> {
            loaded().as_ref().map(|l| l.path.as_path())
        }
    };
}

/// `Some(dir)` unless `dir` is empty: turns the `env!("NV_RS_SDK_BIN")` nv-bindgen bakes into a sys
/// crate (empty when the SDK ships no DLLs) into a [`Group::Sdk`] hint.
pub const fn hint(dir: &'static str) -> Option<&'static str> {
    if dir.is_empty() { None } else { Some(dir) }
}

/// Fails with [`LoadError::RuntimeTooOld`] when `found < required`.
pub fn check_version(lib: &'static str, required: u32, found: u32) -> Result<(), LoadError> {
    if found < required {
        Err(LoadError::RuntimeTooOld { lib, required, found })
    } else {
        Ok(())
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Group::Driver => f.write_str("driver"),
            Group::DriverStore => f.write_str("driver store"),
            Group::Toolkit => f.write_str("CUDA toolkit"),
            Group::Sdk { root_var, .. } => write!(f, "SDK ({root_var})"),
        }
    }
}

/// A library whose only export is a query returning entry points by numeric id (NVAPI's
/// `nvapi_QueryInterface`). Kept loaded for the process.
pub struct QueryInterface {
    /// The query entry point: id -> function pointer, or null when the driver lacks it.
    pub query: unsafe extern "C" fn(u32) -> *mut core::ffi::c_void,
    _lib: libloading::Library,
}

impl QueryInterface {
    /// Opens `path` and resolves the query export `symbol` (NUL-terminated).
    ///
    /// # Safety
    /// `symbol` must name a function with the signature `void* (unsigned int)`.
    pub unsafe fn open(path: &Path, symbol: &[u8]) -> Result<Self, String> {
        // SAFETY: loading the library runs only its initialisers.
        let lib = unsafe { libloading::Library::new(path) }.map_err(|e| e.to_string())?;
        // SAFETY: the caller guarantees the symbol's signature.
        let query = unsafe { lib.get::<unsafe extern "C" fn(u32) -> *mut core::ffi::c_void>(symbol) }
            .map(|s| *s)
            .map_err(|e| e.to_string())?;
        Ok(Self { query, _lib: lib })
    }
}

/// The body of a query-interface sys crate (nv-bindgen `Shape::QueryInterface`, e.g. NVAPI):
/// generated types and shims, the cached loader the shims call, `DLL`, `load()` and `INVENTORY`.
#[macro_export]
macro_rules! query_interface_sys {
    (lib: $lib:literal, loader: $loader:ident, query_symbol: $sym:literal, dll: $dll:expr $(,)?) => {
        // Generated code: lint levels differ per header, so the generated modules opt out.
        #[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code, missing_docs, unnecessary_transmutes, clippy::all, clippy::undocumented_unsafe_blocks)]
        mod types {
            include!(concat!(env!("OUT_DIR"), "/", $lib, "_types.rs"));
        }
        // Shim bodies share one safety argument, documented once in nv-bindgen (`query_shims`).
        #[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, missing_docs, clippy::all, clippy::undocumented_unsafe_blocks)]
        mod shims {
            use super::$loader;
            use super::types::*;
            include!(concat!(env!("OUT_DIR"), "/", $lib, "_shims.rs"));
        }
        pub use shims::*;
        pub use types::*;

        /// Where this library ships and what its files are called.
        pub const DLL: $crate::DllSpec = $dll;

        /// Entry points and header sha256 of the generated bindings (coverage evidence).
        pub const INVENTORY: &str = include_str!(concat!(env!("OUT_DIR"), "/", $lib, "_inventory.json"));

        fn loaded() -> &'static ::core::result::Result<$crate::Loaded<$crate::QueryInterface>, $crate::LoadError> {
            static API: ::std::sync::OnceLock<::core::result::Result<$crate::Loaded<$crate::QueryInterface>, $crate::LoadError>> =
                ::std::sync::OnceLock::new();
            // SAFETY: the query symbol named by the sys crate has the `void* (unsigned int)` signature.
            API.get_or_init(|| $crate::open_first(&DLL, |p| unsafe { $crate::QueryInterface::open(p, $sym) }))
        }

        /// The loaded query, for the shims.
        fn $loader() -> ::core::result::Result<&'static $crate::QueryInterface, &'static $crate::LoadError> {
            loaded().as_ref().map(|l| &l.value)
        }

        /// Loads the library now and reports which file was used, or why loading failed.
        pub fn load() -> ::core::result::Result<&'static ::std::path::Path, &'static $crate::LoadError> {
            loaded().as_ref().map(|l| l.path.as_path())
        }
    };
}

/// The body of a sys crate for a library split over several DLLs (nv-bindgen
/// `Shape::SplitExports`, e.g. NPP).
///
/// Shared types come from `<lib>_types.rs`. Each part becomes a public module named after it
/// with its own private loading struct, cached loader, `DLL` spec and `load()`; its shims are
/// re-exported at the crate root (function names never collide across parts), so callers write
/// `npp_sys::nppiFilter_8u_C1R_Ctx(..)` and diagnose with `npp_sys::nppif::load()`.
#[macro_export]
macro_rules! split_exports_sys {
    (lib: $lib:literal, parts: [ $( { part: $part:ident, api: $api:ident, loader: $loader:ident, dll: $dll:expr $(,)? } ),+ $(,)? ] $(,)?) => {
        // Generated code: lint levels differ per header, so the generated modules opt out.
        #[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code, missing_docs, unnecessary_transmutes, clippy::all, clippy::undocumented_unsafe_blocks)]
        mod types {
            include!(concat!(env!("OUT_DIR"), "/", $lib, "_types.rs"));
        }
        pub use types::*;

        /// Entry points and header sha256 of the generated bindings (coverage evidence).
        pub const INVENTORY: &str = include_str!(concat!(env!("OUT_DIR"), "/", $lib, "_inventory.json"));

        $(
            #[doc = concat!("The `", stringify!($part), "` DLL: its loader, `DLL` spec and shims.")]
            pub mod $part {
                #[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code, missing_docs, unnecessary_transmutes, clippy::all, clippy::undocumented_unsafe_blocks)]
                mod api {
                    use super::super::types::*;
                    include!(concat!(env!("OUT_DIR"), "/", $lib, "_", stringify!($part), "_api.rs"));
                }
                // Shim bodies share one safety argument, documented once in nv-bindgen (`shims`).
                #[doc(hidden)]
                #[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, missing_docs, clippy::all, clippy::undocumented_unsafe_blocks)]
                pub mod shims {
                    use super::$loader;
                    use super::super::types::*;
                    include!(concat!(env!("OUT_DIR"), "/", $lib, "_", stringify!($part), "_shims.rs"));
                }

                /// Where this DLL ships and what it is called.
                pub const DLL: $crate::DllSpec = $dll;

                fn loaded() -> &'static ::core::result::Result<$crate::Loaded<api::$api>, $crate::LoadError> {
                    static API: ::std::sync::OnceLock<::core::result::Result<$crate::Loaded<api::$api>, $crate::LoadError>> =
                        ::std::sync::OnceLock::new();
                    API.get_or_init(|| {
                        // SAFETY: opening the library runs only its loader initialisers; symbols
                        // are resolved lazily and a missing one becomes an `Err` field.
                        $crate::open_first(&DLL, |p| unsafe { api::$api::new(p) }.map_err(|e| e.to_string()))
                    })
                }

                /// The loaded DLL, for the shims.
                fn $loader() -> ::core::result::Result<&'static api::$api, &'static $crate::LoadError> {
                    loaded().as_ref().map(|l| &l.value)
                }

                /// Loads this DLL now and reports which file was used, or why loading failed.
                pub fn load() -> ::core::result::Result<&'static ::std::path::Path, &'static $crate::LoadError> {
                    loaded().as_ref().map(|l| l.path.as_path())
                }
            }
            pub use $part::shims::*;
        )+
    };
}

#[cfg(test)]
mod linux_tests {
    use super::*;

    #[test]
    fn checked_error_layout_stays_within_pointer_word_budget() {
        let word = std::mem::size_of::<usize>();
        let error = std::mem::size_of::<CallError>();
        let result = std::mem::size_of::<Result<usize, CallError>>();
        let diagnostic = std::mem::size_of::<SymbolError>();
        println!("pointer={word} CallError={error} Result<usize, CallError>={result} SymbolError={diagnostic}");
        // Each error variant carries one pointer plus a discriminant; a checked usize result
        // needs at most one further word. The cold two-name diagnostic stays behind the box.
        assert!(error <= 2 * word, "CallError grew to {error} bytes");
        assert!(result <= 3 * word, "checked result grew to {result} bytes");
        assert!(diagnostic > error, "the diagnostic must remain outside the compact error");
    }

    #[test]
    fn linux_sdk_hint_supports_a_versioned_soname_without_an_unversioned_link() {
        let spec = DllSpec {
            lib: "nccl",
            group: Group::Sdk { root_var: "NCCL_ROOT", hint: Some("/sdk/nccl/lib") },
            windows: &[],
            linux: &["libnccl.so.2"],
        };
        assert_eq!(linux_candidates(&spec), [PathBuf::from("/sdk/nccl/lib/libnccl.so.2"), PathBuf::from("libnccl.so.2")]);
    }

    #[test]
    fn linux_driver_toolkit_and_unhinted_sdk_keep_bare_sonames() {
        let spec = DllSpec { lib: "test", group: Group::Driver, windows: &[], linux: &["libtest.so.2", "libtest.so"] };
        let expected = [PathBuf::from("libtest.so.2"), PathBuf::from("libtest.so")];
        for group in [Group::Driver, Group::DriverStore, Group::Toolkit, Group::Sdk { root_var: "TEST_ROOT", hint: None }] {
            assert_eq!(linux_candidates(&DllSpec { group, ..spec }), expected);
        }
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// A machine described by maps instead of a real filesystem and registry.
    #[derive(Default)]
    struct Fake {
        env: HashMap<&'static str, &'static str>,
        dirs: HashMap<PathBuf, Vec<&'static str>>,
        drivers: Vec<PathBuf>,
        exe: Option<PathBuf>,
    }

    impl Probe for Fake {
        fn exe_dir(&self) -> Option<PathBuf> {
            self.exe.clone()
        }
        fn env(&self, name: &str) -> Option<String> {
            self.env.get(name).map(|s| (*s).to_string())
        }
        fn is_file(&self, path: &Path) -> bool {
            let (Some(dir), Some(name)) = (path.parent(), path.file_name()) else { return false };
            self.dirs.get(dir).is_some_and(|files| files.iter().any(|f| *f == name.to_string_lossy()))
        }
        fn list_dir(&self, dir: &Path) -> Vec<String> {
            self.dirs.get(dir).map(|f| f.iter().map(|s| (*s).to_string()).collect()).unwrap_or_default()
        }
        fn display_driver_dirs(&self) -> Vec<PathBuf> {
            self.drivers.clone()
        }
    }

    const CUFFT: DllSpec = DllSpec { lib: "cufft", group: Group::Toolkit, windows: &["cufft64_*"], linux: &[] };
    const OPTIX: DllSpec = DllSpec { lib: "optix", group: Group::DriverStore, windows: &["nvoptix.dll"], linux: &[] };

    #[test]
    fn toolkit_prefix_scan_prefers_the_highest_suffix_and_skips_other_libraries() {
        // Mutation that reddens this: drop the reverse sort (11 before 12) or the `.dll` filter
        // (the `.lib`/`cufftw` neighbours become candidates).
        let mut fake = Fake::default();
        fake.env.insert("CUDA_PATH", r"C:\cuda");
        fake.dirs.insert(
            PathBuf::from(r"C:\cuda\bin\x64"),
            vec!["cufft64_11.dll", "cufft64_12.dll", "cufft64_12.lib", "cublas64_13.dll"],
        );
        let got = candidates_with(&fake, &CUFFT);
        assert_eq!(
            got,
            vec![PathBuf::from(r"C:\cuda\bin\x64\cufft64_12.dll"), PathBuf::from(r"C:\cuda\bin\x64\cufft64_11.dll")]
        );
    }

    #[test]
    fn driver_store_searches_system32_before_the_adapter_folders() {
        let mut fake = Fake::default();
        fake.env.insert("SystemRoot", r"C:\Windows");
        fake.drivers = vec![PathBuf::from(r"C:\DS\nv_a"), PathBuf::from(r"C:\DS\nv_b")];
        let got = candidates_with(&fake, &OPTIX);
        assert_eq!(
            got,
            vec![
                PathBuf::from(r"C:\Windows\System32\nvoptix.dll"),
                PathBuf::from(r"C:\DS\nv_a\nvoptix.dll"),
                PathBuf::from(r"C:\DS\nv_b\nvoptix.dll"),
            ]
        );
    }

    #[test]
    fn open_first_skips_missing_files_and_reports_rejections() {
        let mut fake = Fake::default();
        fake.env.insert("SystemRoot", r"C:\Windows");
        fake.drivers = vec![PathBuf::from(r"C:\DS\old"), PathBuf::from(r"C:\DS\new")];
        fake.dirs.insert(PathBuf::from(r"C:\DS\old"), vec!["nvoptix.dll"]);
        fake.dirs.insert(PathBuf::from(r"C:\DS\new"), vec!["nvoptix.dll"]);

        // The first present candidate is rejected (say, an ABI it does not offer); the second wins.
        let loaded = open_first_with(&fake, &OPTIX, |p| {
            if p.starts_with(r"C:\DS\old") { Err("ABI 118 unsupported".into()) } else { Ok(7) }
        })
        .expect("second candidate opens");
        assert_eq!((loaded.value, loaded.path), (7, PathBuf::from(r"C:\DS\new\nvoptix.dll")));

        let err = open_first_with(&fake, &OPTIX, |_| Err::<(), _>("nope".into())).expect_err("all rejected");
        assert!(matches!(&err, LoadError::Rejected { failures, .. } if failures.len() == 2), "{err}");
    }

    #[test]
    fn nothing_present_is_not_found_with_every_searched_path() {
        let mut fake = Fake::default();
        fake.env.insert("SystemRoot", r"C:\Windows");
        let err = open_first_with(&fake, &OPTIX, |_| Ok(())).expect_err("nothing installed");
        assert!(
            matches!(&err, LoadError::NotFound { tried, .. } if tried == &vec![PathBuf::from(r"C:\Windows\System32\nvoptix.dll")]),
            "{err}"
        );
    }

    #[test]
    fn an_unset_sdk_root_is_never_guessed() {
        let spec = DllSpec { lib: "cudnn", group: Group::Sdk { root_var: "CUDNN_ROOT", hint: None }, windows: &["cudnn64_*"], linux: &[] };
        assert!(candidates_with(&Fake::default(), &spec).is_empty());
    }

    #[test]
    fn sdk_dlls_are_searched_next_to_the_exe_then_root_then_build_hint() {
        let spec = DllSpec {
            lib: "cudnn",
            group: Group::Sdk { root_var: "CUDNN_ROOT", hint: Some(r"C:\hint\bin") },
            windows: &["cudnn64_*"],
            linux: &[],
        };
        let mut fake = Fake { exe: Some(PathBuf::from(r"C:\app")), ..Fake::default() };
        fake.env.insert("CUDNN_ROOT", r"C:\cudnn");
        for dir in [r"C:\app", r"C:\cudnn\bin\x64", r"C:\hint\bin"] {
            fake.dirs.insert(PathBuf::from(dir), vec!["cudnn64_9.dll"]);
        }
        let got = candidates_with(&fake, &spec);
        assert_eq!(
            got,
            vec![
                PathBuf::from(r"C:\app\cudnn64_9.dll"),
                PathBuf::from(r"C:\cudnn\bin\x64\cudnn64_9.dll"),
                PathBuf::from(r"C:\hint\bin\cudnn64_9.dll"),
            ]
        );
    }

    #[test]
    fn version_gate_rejects_only_older_runtimes() {
        assert!(check_version("nvml", 12000, 12000).is_ok());
        assert!(check_version("nvml", 12000, 12100).is_ok());
        assert!(matches!(check_version("nvml", 12000, 11800), Err(LoadError::RuntimeTooOld { found: 11800, .. })));
    }
}
