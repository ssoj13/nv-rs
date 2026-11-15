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
use std::time::Instant;

pub mod diag;

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
    #[error(
        "{lib}: not found (searched {}); place the library next to the executable, set NV_DLL_DIRS, or add its directory to PATH",
        show_paths(.tried)
    )]
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
#[derive(Debug)]
pub enum CallError {
    /// The library could not be loaded; preserves its original diagnostic.
    Load(&'static LoadError),
    /// The library loaded but the requested entry point was unavailable.
    Symbol(Box<SymbolError>),
}

/// Cold diagnostic for an unavailable native entry point.
#[derive(Debug)]
pub struct SymbolError {
    /// The library's binding name.
    pub lib: &'static str,
    /// The exact native entry-point name.
    pub symbol: &'static str,
    /// The dynamic loader's cause, or `None` when a query interface returned null.
    pub source: Option<&'static libloading::Error>,
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Load(err) => fmt::Display::fmt(err, f),
            Self::Symbol(diag) => fmt::Display::fmt(diag, f),
        }
    }
}

impl std::error::Error for CallError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            // Dereference `&&LoadError` to `&LoadError` so callers' `.is::<LoadError>()` matches.
            Self::Load(err) => Some(*err),
            Self::Symbol(diag) => std::error::Error::source(diag.as_ref()),
        }
    }
}

impl fmt::Display for SymbolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: symbol {} is unavailable", self.lib, self.symbol)
    }
}

impl std::error::Error for SymbolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Unsize `&libloading::Error` directly rather than via `thiserror`'s `&'static T` `AsDynError`,
        // which would otherwise report `TypeId` = `&'static libloading::Error` instead of `libloading::Error`.
        let err: &(dyn std::error::Error + 'static) = self.source?;
        Some(err)
    }
}

impl CallError {
    /// Constructs the boxed symbol diagnostic on a generated call's failure branch.
    pub fn symbol(
        lib: &'static str,
        symbol: &'static str,
        source: Option<&'static libloading::Error>,
    ) -> Self {
        Self::Symbol(Box::new(SymbolError {
            lib,
            symbol,
            source,
        }))
    }
}

fn show_paths(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn show_failures(failures: &[(PathBuf, String)]) -> String {
    failures
        .iter()
        .map(|(p, e)| format!("{}: {e}", p.display()))
        .collect::<Vec<_>>()
        .join("; ")
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
        entries
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect()
    }

    fn exe_dir(&self) -> Option<PathBuf> {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(Path::to_path_buf))
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
    let Ok(class) =
        RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(DISPLAY_CLASS, KEY_READ)
    else {
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
            .or_else(|_| {
                adapter
                    .get_value::<String, _>("OpenGLDriverName")
                    .map(|s| vec![s])
            })
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

static EXTRA_SEARCH_DIRS: std::sync::RwLock<Vec<PathBuf>> = std::sync::RwLock::new(Vec::new());

/// Registers an extra directory to search before standard toolkit, SDK, or `PATH` locations.
///
/// Empty paths and duplicates are ignored.
pub fn add_search_dir(dir: impl AsRef<Path>) {
    let path = dir.as_ref();
    if path.as_os_str().is_empty() {
        return;
    }
    let mut dirs = EXTRA_SEARCH_DIRS.write().unwrap_or_else(|e| e.into_inner());
    let owned = path.to_path_buf();
    if !dirs.contains(&owned) {
        dirs.push(owned);
    }
}

fn push_unique_dir(dirs: &mut Vec<PathBuf>, dir: PathBuf) {
    if !dir.as_os_str().is_empty() && !dirs.contains(&dir) {
        dirs.push(dir);
    }
}

fn extra_search_dirs(probe: &dyn Probe) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(env_dirs) = probe.env("NV_DLL_DIRS") {
        for dir in std::env::split_paths(&env_dirs) {
            push_unique_dir(&mut dirs, dir);
        }
    }
    let registered = EXTRA_SEARCH_DIRS.read().unwrap_or_else(|e| e.into_inner());
    for dir in registered.iter() {
        push_unique_dir(&mut dirs, dir.clone());
    }
    dirs
}

fn push_toolkit_root(dirs: &mut Vec<PathBuf>, root: &Path) {
    // CUDA 13 moved DLLs into `bin\x64` while 12.x keeps them in `bin`; CUPTI ships under
    // `extras\CUPTI\lib64`.
    for dir in [
        root.join("bin").join("x64"),
        root.join("bin"),
        root.join("extras").join("CUPTI").join("lib64"),
    ] {
        push_unique_dir(dirs, dir);
    }
}

fn cuda_version_dirs(probe: &dyn Probe, cuda_base: &Path) -> Vec<PathBuf> {
    let mut entries: Vec<(Vec<u32>, String)> = probe
        .list_dir(cuda_base)
        .into_iter()
        .filter_map(|name| {
            let rest = name.strip_prefix('v').or_else(|| name.strip_prefix('V'))?;
            let parts: Vec<u32> = rest
                .split('.')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
            Some((parts, name))
        })
        .collect();
    entries.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1)));
    entries
        .into_iter()
        .map(|(_, name)| cuda_base.join(name))
        .collect()
}

/// Candidate paths for `spec` as seen through `probe`, most preferred first.
///
/// On Linux an SDK's discovered runtime directory precedes bare sonames. The dynamic linker's
/// own search (`LD_LIBRARY_PATH`, `ld.so.cache`, `/usr/lib/wsl/lib` on WSL) follows that hint.
pub fn candidates_with(probe: &dyn Probe, spec: &DllSpec) -> Vec<PathBuf> {
    if !cfg!(windows) {
        return linux_candidates_with(probe, spec);
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

/// Linux candidate paths: extra search dirs, runtime env roots, build hints, then bare sonames.
fn linux_candidates_with(probe: &dyn Probe, spec: &DllSpec) -> Vec<PathBuf> {
    const TOOLKIT_VARS: [&str; 4] = [
        "CUDA_PATH",
        "CUDA_HOME",
        "CUDA_ROOT",
        "CUDA_TOOLKIT_ROOT_DIR",
    ];
    let mut out = Vec::new();
    for dir in extra_search_dirs(probe) {
        for soname in spec.linux {
            push_unique_dir(&mut out, dir.join(soname));
        }
    }
    match spec.group {
        Group::Sdk { root_var, hint } => {
            if let Some(root) = probe.env(root_var) {
                let root = Path::new(&root);
                for sub in ["lib64", "lib"] {
                    let dir = root.join(sub);
                    for soname in spec.linux {
                        push_unique_dir(&mut out, dir.join(soname));
                    }
                }
            }
            if let Some(dir) = hint {
                for soname in spec.linux {
                    push_unique_dir(&mut out, Path::new(dir).join(soname));
                }
            }
        }
        Group::Toolkit => {
            for var in TOOLKIT_VARS {
                if let Some(root) = probe.env(var) {
                    let root = Path::new(&root);
                    for sub in ["lib64", "lib"] {
                        let dir = root.join(sub);
                        for soname in spec.linux {
                            push_unique_dir(&mut out, dir.join(soname));
                        }
                    }
                }
            }
        }
        Group::Driver | Group::DriverStore => {}
    }
    for soname in spec.linux {
        push_unique_dir(&mut out, PathBuf::from(soname));
    }
    out
}

/// Directories searched for a group on Windows, most preferred first.
fn search_dirs(probe: &dyn Probe, group: Group) -> Vec<PathBuf> {
    let system32 = PathBuf::from(
        probe
            .env("SystemRoot")
            .unwrap_or_else(|| r"C:\Windows".to_string()),
    )
    .join("System32");
    match group {
        Group::Driver => vec![system32],
        // NVIDIA's own order: System32 first (older drivers copied it there), then the adapters,
        // then any user-supplied extra directories.
        Group::DriverStore => {
            let mut dirs = vec![system32];
            for dir in probe.display_driver_dirs() {
                push_unique_dir(&mut dirs, dir);
            }
            for dir in extra_search_dirs(probe) {
                push_unique_dir(&mut dirs, dir);
            }
            dirs
        }
        Group::Toolkit => {
            let mut dirs = extra_search_dirs(probe);
            if let Some(exe) = probe.exe_dir() {
                push_unique_dir(&mut dirs, exe);
            }
            // The variables vcv-rs and cudarc honour, in the same order.
            for var in [
                "CUDA_PATH",
                "CUDA_HOME",
                "CUDA_ROOT",
                "CUDA_TOOLKIT_ROOT_DIR",
            ] {
                if let Some(root) = probe.env(var) {
                    push_toolkit_root(&mut dirs, Path::new(&root));
                }
            }
            let program_files = probe
                .env("ProgramFiles")
                .unwrap_or_else(|| r"C:\Program Files".to_string());
            let cuda_base = PathBuf::from(program_files)
                .join("NVIDIA GPU Computing Toolkit")
                .join("CUDA");
            for ver_dir in cuda_version_dirs(probe, &cuda_base) {
                push_toolkit_root(&mut dirs, &ver_dir);
            }
            if let Some(path_var) = probe.env("PATH") {
                for dir in std::env::split_paths(&path_var) {
                    push_unique_dir(&mut dirs, dir);
                }
            }
            dirs
        }
        Group::Sdk { root_var, hint } => {
            let mut dirs = extra_search_dirs(probe);
            if let Some(exe) = probe.exe_dir() {
                push_unique_dir(&mut dirs, exe);
            }
            if let Some(root) = probe.env(root_var) {
                let root = PathBuf::from(root);
                for dir in [
                    root.join("bin").join("x64"),
                    root.join("bin"),
                    root.join("lib"),
                ] {
                    push_unique_dir(&mut dirs, dir);
                }
            }
            if let Some(dir) = hint {
                push_unique_dir(&mut dirs, PathBuf::from(dir));
            }
            if let Some(path_var) = probe.env("PATH") {
                for dir in std::env::split_paths(&path_var) {
                    push_unique_dir(&mut dirs, dir);
                }
            }
            dirs
        }
    }
}

/// Opens a dynamic library at `path`.
///
/// On Windows, tries `LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS`
/// (`0x0000_1100`) so transitive DLL dependencies next to `path` resolve without mutating
/// `PATH`, falling back to [`libloading::Library::new`] on error.
pub fn open_library(path: &Path) -> Result<libloading::Library, libloading::Error> {
    #[cfg(windows)]
    {
        // `LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS`
        const SEARCH_FLAGS: u32 = 0x0000_1100;
        if let Ok(lib) = unsafe {
            // SAFETY: loading a dynamic library executes only its loader initialisers; the
            // `LOAD_LIBRARY_SEARCH_*` flags restrict dependency resolution to safe directories.
            libloading::os::windows::Library::load_with_flags(path, SEARCH_FLAGS)
        } {
            return Ok(lib.into());
        }
    }
    // SAFETY: loading a dynamic library executes only its loader initialisers.
    unsafe { libloading::Library::new(path) }
}

/// Opens the first candidate for which `open` succeeds.
///
/// `open` receives a path (Windows: an existing file; Linux: an SDK path or linker soname) and
/// returns the loaded value or a reason it was rejected (wrong ABI, missing entry point, ...).
pub fn open_first<T>(
    spec: &DllSpec,
    open: impl FnMut(&Path) -> Result<T, String>,
) -> Result<Loaded<T>, LoadError> {
    open_first_with(&SystemProbe, spec, open)
}

/// [`open_first`] through an explicit [`Probe`].
pub fn open_first_with<T>(
    probe: &dyn Probe,
    spec: &DllSpec,
    mut open: impl FnMut(&Path) -> Result<T, String>,
) -> Result<Loaded<T>, LoadError> {
    let t0 = Instant::now();
    let all = candidates_with(probe, spec);
    let present: Vec<PathBuf> = if cfg!(windows) {
        all.iter().filter(|p| probe.is_file(p)).cloned().collect()
    } else {
        all.clone()
    };
    if present.is_empty() {
        let err = LoadError::NotFound {
            lib: spec.lib,
            tried: all,
        };
        diag::record_lib_load(
            spec.lib,
            None,
            Some(&err.to_string()),
            t0.elapsed().as_nanos() as u64,
        );
        return Err(err);
    }
    let mut failures = Vec::new();
    for path in present {
        match open(&path) {
            Ok(value) => {
                diag::record_lib_load(spec.lib, Some(&path), None, t0.elapsed().as_nanos() as u64);
                return Ok(Loaded { value, path });
            }
            Err(reason) => failures.push((path, reason)),
        }
    }
    let err = LoadError::Rejected {
        lib: spec.lib,
        failures,
    };
    diag::record_lib_load(
        spec.lib,
        None,
        Some(&err.to_string()),
        t0.elapsed().as_nanos() as u64,
    );
    Err(err)
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
        #[allow(
            non_camel_case_types,
            non_upper_case_globals,
            non_snake_case,
            dead_code,
            missing_docs,
            unnecessary_transmutes,
            clippy::all,
            clippy::undocumented_unsafe_blocks
        )]
        mod types {
            include!(concat!(env!("OUT_DIR"), "/", $lib, "_types.rs"));
        }
        #[allow(
            non_camel_case_types,
            non_upper_case_globals,
            non_snake_case,
            dead_code,
            missing_docs,
            unnecessary_transmutes,
            clippy::all,
            clippy::undocumented_unsafe_blocks
        )]
        mod api {
            use super::types::*;
            include!(concat!(env!("OUT_DIR"), "/", $lib, "_api.rs"));
        }
        // Generated shim unsafe blocks carry `// SAFETY:` comments emitted by `nv-bindgen`.
        #[allow(
            non_camel_case_types,
            non_upper_case_globals,
            non_snake_case,
            missing_docs,
            unnecessary_transmutes,
            clippy::all
        )]
        mod shims {
            use super::types::*;
            use super::$loader;
            include!(concat!(env!("OUT_DIR"), "/", $lib, "_shims.rs"));
        }
        pub use shims::*;
        pub use types::*;

        /// Where this library ships and what its files are called.
        pub const DLL: $crate::DllSpec = $dll;

        /// Entry points and header sha256 of the generated bindings (coverage evidence).
        pub const INVENTORY: &str =
            include_str!(concat!(env!("OUT_DIR"), "/", $lib, "_inventory.json"));

        fn loaded() -> &'static ::core::result::Result<$crate::Loaded<api::$api>, $crate::LoadError>
        {
            static API: ::std::sync::OnceLock<
                ::core::result::Result<$crate::Loaded<api::$api>, $crate::LoadError>,
            > = ::std::sync::OnceLock::new();
            API.get_or_init(|| {
                $crate::open_first(&DLL, |p| {
                    $crate::open_library(p)
                        .and_then(|lib| unsafe {
                            // SAFETY: opening the library runs only its loader initialisers; symbols
                            // are resolved lazily and a missing one becomes an `Err` field, not a crash.
                            api::$api::from_library(lib)
                        })
                        .map_err(|e| e.to_string())
                })
            })
        }

        /// The loaded library, for the shims.
        fn $loader() -> ::core::result::Result<&'static api::$api, &'static $crate::LoadError> {
            loaded().as_ref().map(|l| &l.value)
        }

        /// Loads the library now and reports which file was used, or why loading failed.
        pub fn load()
        -> ::core::result::Result<&'static ::std::path::Path, &'static $crate::LoadError> {
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
        Err(LoadError::RuntimeTooOld {
            lib,
            required,
            found,
        })
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
        let lib = open_library(path).map_err(|e| e.to_string())?;
        // SAFETY: the caller guarantees the symbol's signature.
        let query =
            unsafe { lib.get::<unsafe extern "C" fn(u32) -> *mut core::ffi::c_void>(symbol) }
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
        #[allow(
            non_camel_case_types,
            non_upper_case_globals,
            non_snake_case,
            dead_code,
            missing_docs,
            unnecessary_transmutes,
            clippy::all,
            clippy::undocumented_unsafe_blocks
        )]
        mod types {
            include!(concat!(env!("OUT_DIR"), "/", $lib, "_types.rs"));
        }
        // Generated shim unsafe blocks carry `// SAFETY:` comments emitted by `nv-bindgen`.
        #[allow(
            non_camel_case_types,
            non_upper_case_globals,
            non_snake_case,
            missing_docs,
            clippy::all
        )]
        mod shims {
            use super::types::*;
            use super::$loader;
            include!(concat!(env!("OUT_DIR"), "/", $lib, "_shims.rs"));
        }
        pub use shims::*;
        pub use types::*;

        /// Where this library ships and what its files are called.
        pub const DLL: $crate::DllSpec = $dll;

        /// Entry points and header sha256 of the generated bindings (coverage evidence).
        pub const INVENTORY: &str =
            include_str!(concat!(env!("OUT_DIR"), "/", $lib, "_inventory.json"));

        fn loaded() -> &'static ::core::result::Result<
            $crate::Loaded<$crate::QueryInterface>,
            $crate::LoadError,
        > {
            static API: ::std::sync::OnceLock<
                ::core::result::Result<$crate::Loaded<$crate::QueryInterface>, $crate::LoadError>,
            > = ::std::sync::OnceLock::new();
            // SAFETY: the query symbol named by the sys crate has the `void* (unsigned int)` signature.
            API.get_or_init(|| {
                $crate::open_first(&DLL, |p| unsafe { $crate::QueryInterface::open(p, $sym) })
            })
        }

        /// The loaded query, for the shims.
        fn $loader()
        -> ::core::result::Result<&'static $crate::QueryInterface, &'static $crate::LoadError> {
            loaded().as_ref().map(|l| &l.value)
        }

        /// Loads the library now and reports which file was used, or why loading failed.
        pub fn load()
        -> ::core::result::Result<&'static ::std::path::Path, &'static $crate::LoadError> {
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
                // Generated shim unsafe blocks carry `// SAFETY:` comments emitted by `nv-bindgen`.
                #[doc(hidden)]
                #[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, missing_docs, clippy::all)]
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
                        $crate::open_first(&DLL, |p| {
                            $crate::open_library(p)
                                .and_then(|lib| unsafe {
                                    // SAFETY: opening the library runs only its loader initialisers;
                                    // symbols are resolved lazily and a missing one becomes an `Err` field.
                                    api::$api::from_library(lib)
                                })
                                .map_err(|e| e.to_string())
                        })
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
mod test_support {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

    /// Serializes tests that mutate `EXTRA_SEARCH_DIRS` against tests that assert exact lists.
    static EXTRA_DIRS_GATE: RwLock<()> = RwLock::new(());

    pub(super) fn read_extra_dirs_gate() -> RwLockReadGuard<'static, ()> {
        EXTRA_DIRS_GATE.read().unwrap_or_else(|e| e.into_inner())
    }

    pub(super) struct ExtraDirsGuard {
        _lock: RwLockWriteGuard<'static, ()>,
    }

    impl ExtraDirsGuard {
        pub(super) fn new() -> Self {
            let lock = EXTRA_DIRS_GATE.write().unwrap_or_else(|e| e.into_inner());
            EXTRA_SEARCH_DIRS
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .clear();
            Self { _lock: lock }
        }
    }

    impl Drop for ExtraDirsGuard {
        fn drop(&mut self) {
            EXTRA_SEARCH_DIRS
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .clear();
        }
    }

    /// A machine described by maps instead of a real filesystem and registry.
    #[derive(Default)]
    pub(super) struct Fake {
        pub(super) env: HashMap<&'static str, &'static str>,
        pub(super) dirs: HashMap<PathBuf, Vec<&'static str>>,
        pub(super) drivers: Vec<PathBuf>,
        pub(super) exe: Option<PathBuf>,
    }

    impl Probe for Fake {
        fn exe_dir(&self) -> Option<PathBuf> {
            self.exe.clone()
        }
        fn env(&self, name: &str) -> Option<String> {
            self.env.get(name).map(|s| (*s).to_string())
        }
        fn is_file(&self, path: &Path) -> bool {
            let (Some(dir), Some(name)) = (path.parent(), path.file_name()) else {
                return false;
            };
            self.dirs
                .get(dir)
                .is_some_and(|files| files.iter().any(|f| *f == name.to_string_lossy()))
        }
        fn list_dir(&self, dir: &Path) -> Vec<String> {
            self.dirs
                .get(dir)
                .map(|f| f.iter().map(|s| (*s).to_string()).collect())
                .unwrap_or_default()
        }
        fn display_driver_dirs(&self) -> Vec<PathBuf> {
            self.drivers.clone()
        }
    }
}

#[cfg(test)]
mod linux_tests {
    use super::test_support::{Fake, read_extra_dirs_gate};
    use super::*;

    #[test]
    fn checked_error_layout_stays_within_pointer_word_budget() {
        let word = std::mem::size_of::<usize>();
        let error = std::mem::size_of::<CallError>();
        let result = std::mem::size_of::<Result<usize, CallError>>();
        let diagnostic = std::mem::size_of::<SymbolError>();
        println!(
            "pointer={word} CallError={error} Result<usize, CallError>={result} SymbolError={diagnostic}"
        );
        // Each error variant carries one pointer plus a discriminant; a checked usize result
        // needs at most one further word. The cold two-name diagnostic stays behind the box.
        assert!(error <= 2 * word, "CallError grew to {error} bytes");
        assert!(result <= 3 * word, "checked result grew to {result} bytes");
        assert!(
            diagnostic > error,
            "the diagnostic must remain outside the compact error"
        );
    }

    #[test]
    fn linux_sdk_hint_supports_a_versioned_soname_without_an_unversioned_link() {
        let _gate = read_extra_dirs_gate();
        let spec = DllSpec {
            lib: "nccl",
            group: Group::Sdk {
                root_var: "NCCL_ROOT",
                hint: Some("/sdk/nccl/lib"),
            },
            windows: &[],
            linux: &["libnccl.so.2"],
        };
        assert_eq!(
            linux_candidates_with(&Fake::default(), &spec),
            [
                PathBuf::from("/sdk/nccl/lib/libnccl.so.2"),
                PathBuf::from("libnccl.so.2")
            ]
        );
    }

    #[test]
    fn linux_driver_toolkit_and_unhinted_sdk_keep_bare_sonames() {
        let _gate = read_extra_dirs_gate();
        let spec = DllSpec {
            lib: "test",
            group: Group::Driver,
            windows: &[],
            linux: &["libtest.so.2", "libtest.so"],
        };
        let expected = [PathBuf::from("libtest.so.2"), PathBuf::from("libtest.so")];
        let fake = Fake::default();
        for group in [
            Group::Driver,
            Group::DriverStore,
            Group::Toolkit,
            Group::Sdk {
                root_var: "TEST_ROOT",
                hint: None,
            },
        ] {
            assert_eq!(
                linux_candidates_with(&fake, &DllSpec { group, ..spec }),
                expected
            );
        }
    }

    #[test]
    fn linux_candidates_with_checks_extra_dirs_runtime_roots_and_hints() {
        let _gate = read_extra_dirs_gate();
        let mut fake = Fake::default();
        fake.env.insert("NV_DLL_DIRS", "/custom/lib");
        fake.env.insert("NCCL_ROOT", "/opt/nccl");
        fake.env.insert("CUDA_PATH", "/usr/local/cuda");

        let sdk_spec = DllSpec {
            lib: "nccl",
            group: Group::Sdk {
                root_var: "NCCL_ROOT",
                hint: Some("/build/nccl/lib"),
            },
            windows: &[],
            linux: &["libnccl.so.2"],
        };
        assert_eq!(
            linux_candidates_with(&fake, &sdk_spec),
            vec![
                PathBuf::from("/custom/lib/libnccl.so.2"),
                PathBuf::from("/opt/nccl/lib64/libnccl.so.2"),
                PathBuf::from("/opt/nccl/lib/libnccl.so.2"),
                PathBuf::from("/build/nccl/lib/libnccl.so.2"),
                PathBuf::from("libnccl.so.2"),
            ]
        );

        let toolkit_spec = DllSpec {
            lib: "cufft",
            group: Group::Toolkit,
            windows: &[],
            linux: &["libcufft.so.11"],
        };
        assert_eq!(
            linux_candidates_with(&fake, &toolkit_spec),
            vec![
                PathBuf::from("/custom/lib/libcufft.so.11"),
                PathBuf::from("/usr/local/cuda/lib64/libcufft.so.11"),
                PathBuf::from("/usr/local/cuda/lib/libcufft.so.11"),
                PathBuf::from("libcufft.so.11"),
            ]
        );
    }

    #[test]
    fn call_error_preserves_load_and_symbol_error_causes() {
        static LOAD_ERR: LoadError = LoadError::NotFound {
            lib: "fake",
            tried: Vec::new(),
        };
        static DL_ERR: libloading::Error = libloading::Error::DlSymUnknown;

        let load_err = CallError::Load(&LOAD_ERR);
        assert!(load_err.to_string().starts_with("fake: not found"));
        let msg = load_err.to_string();
        assert!(msg.contains("NV_DLL_DIRS"), "{msg}");
        assert!(msg.contains("PATH"), "{msg}");
        let load_cause = std::error::Error::source(&load_err).expect("load cause");
        assert!(
            load_cause.is::<LoadError>(),
            "CallError::Load source must downcast to LoadError"
        );

        let sym_err = CallError::symbol("fake", "fakeSym", Some(&DL_ERR));
        assert_eq!(sym_err.to_string(), "fake: symbol fakeSym is unavailable");
        let sym_cause = std::error::Error::source(&sym_err).expect("loader cause");
        assert!(
            sym_cause.is::<libloading::Error>(),
            "CallError::Symbol source must downcast to libloading::Error"
        );
        let CallError::Symbol(diag) = sym_err else {
            unreachable!()
        };
        assert!(
            std::error::Error::source(diag.as_ref())
                .expect("symbol cause")
                .is::<libloading::Error>()
        );

        let qi_err = CallError::symbol("fake", "fakeQi", None);
        assert!(std::error::Error::source(&qi_err).is_none());
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::test_support::{ExtraDirsGuard, Fake, read_extra_dirs_gate};
    use super::*;

    const CUFFT: DllSpec = DllSpec {
        lib: "cufft",
        group: Group::Toolkit,
        windows: &["cufft64_*"],
        linux: &[],
    };
    const OPTIX: DllSpec = DllSpec {
        lib: "optix",
        group: Group::DriverStore,
        windows: &["nvoptix.dll"],
        linux: &[],
    };

    #[test]
    fn toolkit_prefix_scan_prefers_the_highest_suffix_and_skips_other_libraries() {
        let _gate = read_extra_dirs_gate();
        // Mutation that reddens this: drop the reverse sort (11 before 12) or the `.dll` filter
        // (the `.lib`/`cufftw` neighbours become candidates).
        let mut fake = Fake::default();
        fake.env.insert("CUDA_PATH", r"C:\cuda");
        fake.dirs.insert(
            PathBuf::from(r"C:\cuda\bin\x64"),
            vec![
                "cufft64_11.dll",
                "cufft64_12.dll",
                "cufft64_12.lib",
                "cublas64_13.dll",
            ],
        );
        let got = candidates_with(&fake, &CUFFT);
        assert_eq!(
            got,
            vec![
                PathBuf::from(r"C:\cuda\bin\x64\cufft64_12.dll"),
                PathBuf::from(r"C:\cuda\bin\x64\cufft64_11.dll")
            ]
        );
    }

    #[test]
    fn driver_store_searches_system32_before_the_adapter_folders() {
        let _gate = read_extra_dirs_gate();
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
        let _gate = read_extra_dirs_gate();
        let mut fake = Fake::default();
        fake.env.insert("SystemRoot", r"C:\Windows");
        fake.drivers = vec![PathBuf::from(r"C:\DS\old"), PathBuf::from(r"C:\DS\new")];
        fake.dirs
            .insert(PathBuf::from(r"C:\DS\old"), vec!["nvoptix.dll"]);
        fake.dirs
            .insert(PathBuf::from(r"C:\DS\new"), vec!["nvoptix.dll"]);

        // The first present candidate is rejected (say, an ABI it does not offer); the second wins.
        let loaded = open_first_with(&fake, &OPTIX, |p| {
            if p.starts_with(r"C:\DS\old") {
                Err("ABI 118 unsupported".into())
            } else {
                Ok(7)
            }
        })
        .expect("second candidate opens");
        assert_eq!(
            (loaded.value, loaded.path),
            (7, PathBuf::from(r"C:\DS\new\nvoptix.dll"))
        );

        let err = open_first_with(&fake, &OPTIX, |_| Err::<(), _>("nope".into()))
            .expect_err("all rejected");
        assert!(
            matches!(&err, LoadError::Rejected { failures, .. } if failures.len() == 2),
            "{err}"
        );
    }

    #[test]
    fn nothing_present_is_not_found_with_every_searched_path() {
        let _gate = read_extra_dirs_gate();
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
        let _gate = read_extra_dirs_gate();
        let spec = DllSpec {
            lib: "cudnn",
            group: Group::Sdk {
                root_var: "CUDNN_ROOT",
                hint: None,
            },
            windows: &["cudnn64_*"],
            linux: &[],
        };
        assert!(candidates_with(&Fake::default(), &spec).is_empty());
    }

    #[test]
    fn sdk_dlls_are_searched_next_to_the_exe_then_root_then_build_hint() {
        let _gate = read_extra_dirs_gate();
        let spec = DllSpec {
            lib: "cudnn",
            group: Group::Sdk {
                root_var: "CUDNN_ROOT",
                hint: Some(r"C:\hint\bin"),
            },
            windows: &["cudnn64_*"],
            linux: &[],
        };
        let mut fake = Fake {
            exe: Some(PathBuf::from(r"C:\app")),
            ..Fake::default()
        };
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
    fn nv_dll_dirs_env_is_searched_before_exe_and_toolkit_root() {
        let _gate = read_extra_dirs_gate();
        let mut fake = Fake {
            exe: Some(PathBuf::from(r"C:\app")),
            ..Fake::default()
        };
        fake.env
            .insert("NV_DLL_DIRS", r"C:\override_a;C:\override_b");
        fake.env.insert("CUDA_PATH", r"C:\cuda");
        for dir in [
            r"C:\override_a",
            r"C:\override_b",
            r"C:\app",
            r"C:\cuda\bin\x64",
        ] {
            fake.dirs.insert(PathBuf::from(dir), vec!["cufft64_12.dll"]);
        }
        let got = candidates_with(&fake, &CUFFT);
        assert_eq!(
            got,
            vec![
                PathBuf::from(r"C:\override_a\cufft64_12.dll"),
                PathBuf::from(r"C:\override_b\cufft64_12.dll"),
                PathBuf::from(r"C:\app\cufft64_12.dll"),
                PathBuf::from(r"C:\cuda\bin\x64\cufft64_12.dll"),
            ]
        );
    }

    #[test]
    fn add_search_dir_deduplicates_and_ignores_empty_paths() {
        let _guard = ExtraDirsGuard::new();
        add_search_dir("");
        add_search_dir(r"C:\prog_extra");
        add_search_dir(r"C:\prog_extra");

        let mut fake = Fake {
            exe: Some(PathBuf::from(r"C:\app")),
            ..Fake::default()
        };
        fake.dirs
            .insert(PathBuf::from(r"C:\prog_extra"), vec!["cufft64_12.dll"]);
        fake.dirs
            .insert(PathBuf::from(r"C:\app"), vec!["cufft64_12.dll"]);
        let got = candidates_with(&fake, &CUFFT);
        assert_eq!(
            got,
            vec![
                PathBuf::from(r"C:\prog_extra\cufft64_12.dll"),
                PathBuf::from(r"C:\app\cufft64_12.dll"),
            ]
        );
    }

    #[test]
    fn toolkit_searches_exe_dir_program_files_descending_and_path() {
        let _gate = read_extra_dirs_gate();
        let mut fake = Fake {
            exe: Some(PathBuf::from(r"C:\app")),
            ..Fake::default()
        };
        fake.env.insert("ProgramFiles", r"C:\Program Files");
        fake.env.insert("PATH", r"C:\from_path");
        fake.dirs.insert(
            PathBuf::from(r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA"),
            vec!["v12.6", "v13.0", "other"],
        );
        for dir in [
            r"C:\app",
            r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0\bin\x64",
            r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.6\bin",
            r"C:\from_path",
        ] {
            fake.dirs.insert(PathBuf::from(dir), vec!["cufft64_12.dll"]);
        }
        let got = candidates_with(&fake, &CUFFT);
        assert_eq!(
            got,
            vec![
                PathBuf::from(r"C:\app\cufft64_12.dll"),
                PathBuf::from(
                    r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0\bin\x64\cufft64_12.dll"
                ),
                PathBuf::from(
                    r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.6\bin\cufft64_12.dll"
                ),
                PathBuf::from(r"C:\from_path\cufft64_12.dll"),
            ]
        );
    }

    #[test]
    fn version_gate_rejects_only_older_runtimes() {
        assert!(check_version("nvml", 12000, 12000).is_ok());
        assert!(check_version("nvml", 12000, 12100).is_ok());
        assert!(matches!(
            check_version("nvml", 12000, 11800),
            Err(LoadError::RuntimeTooOld { found: 11800, .. })
        ));
    }
}
