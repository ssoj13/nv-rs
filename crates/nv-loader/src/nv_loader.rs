//! Runtime discovery and loading of NVIDIA libraries — the one loader every nv-rs sys crate uses.
//!
//! **What.** Every library is described once, in [`catalog`], as a [`DllSpec`] (which [`Group`] it
//! ships in, its Windows file names or version-prefixes, its Linux sonames); sys crates,
//! [`diag::probe_all`] and `xtask stage-dlls` all read that one table. [`open_first`] walks the
//! candidates in priority order and returns the first one the caller's constructor accepts, or a
//! [`LoadError`] that lists every path tried and why each was rejected. A constructor reports a
//! runtime older than the headers it was bound against with [`check_version`]; when that is the
//! only reason every candidate failed, the result is the typed [`LoadError::RuntimeTooOld`].
//!
//! **Search order** (both OSes): directories from `NV_DLL_DIRS` and [`add_search_dir`] first for
//! every group, then the group's own locations (see [`Group`]). On Linux the group's directories are
//! followed by the bare sonames, i.e. the dynamic linker's own search.
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
//! which calls [`open_first`] once and caches the result ([`cached_loader!`]).

use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

pub mod catalog;
pub mod diag;

/// Where a library ships, which decides where it is searched for (after the `NV_DLL_DIRS` /
/// [`add_search_dir`] directories, which come first for every group).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    /// Installed by the display driver into System32 (Windows) or the system library path
    /// (`nvml`, `nvEncodeAPI64`, `nvcuvid`, `nvofapi64`, `nvapi64`).
    Driver,
    /// Installed by the display driver into its DriverStore folder (`nvoptix`, `nvngx`): System32
    /// first (older drivers copied it there), then each display adapter's driver folder.
    DriverStore,
    /// Part of the CUDA Toolkit (`cufft64_12`, `nvrtc64_130_0`). Windows: the executable's
    /// directory, `bin\x64`, `bin` and `extras\CUPTI\lib64` of each `CUDA_PATH`-style root and of
    /// every installed toolkit (newest first), then `PATH`. Linux: `lib64`/`lib` of each root, then
    /// the dynamic linker.
    Toolkit,
    /// A separately installed SDK (cuDNN, nvCOMP, ...). Windows: the directory next to the running
    /// executable (how applications ship these DLLs); `bin\x64`, `bin`, `lib\Windows_x86_64\rel`
    /// and `lib` under `<root_var>`; then `hint`; then `PATH`. Linux: `lib64`, `lib` and (x86_64)
    /// `lib/Linux_x86_64/rel` under `<root_var>`, then `hint`, then the dynamic linker.
    Sdk {
        /// The environment variable naming the SDK root.
        root_var: &'static str,
        /// The SDK's DLL directory vcv-rs found when the sys crate was built (see
        /// [`DllSpec::with_hint`]): only ever set in builds with `debug_assertions`, so tests and
        /// local runs find the build's SDK while a shipped binary never embeds or searches a
        /// build-machine path.
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

impl DllSpec {
    /// This spec with `dir` (nv-bindgen's `env!("NV_RS_SDK_BIN")`) as its [`Group::Sdk`] hint.
    ///
    /// The hint is kept only in builds with `debug_assertions` and only when `dir` is non-empty; a
    /// release build gets `None`, so the build machine's path is neither embedded nor searched.
    /// Specs of other groups are returned unchanged.
    pub const fn with_hint(self, dir: &'static str) -> Self {
        let hint = if cfg!(debug_assertions) && !dir.is_empty() {
            Some(dir)
        } else {
            None
        };
        match self.group {
            Group::Sdk { root_var, .. } => Self {
                group: Group::Sdk { root_var, hint },
                ..self
            },
            _ => self,
        }
    }
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
        /// Every path that was looked for; a version-prefix entry appears as its pattern
        /// (`<dir>\cufft64_*.dll`).
        tried: Vec<PathBuf>,
    },
    /// Candidates exist but every one was rejected by the loader or the constructor.
    #[error("{lib}: every candidate was rejected: {}", show_failures(.failures))]
    Rejected {
        /// The library.
        lib: &'static str,
        /// Each candidate and why it failed (including candidates that were too old).
        failures: Vec<(PathBuf, String)>,
    },
    /// Every candidate loaded but reports a version older than the bindings require.
    #[error(
        "{lib}: every candidate is older than the version {required} these bindings were generated for: {}",
        show_versions(.found)
    )]
    RuntimeTooOld {
        /// The library.
        lib: &'static str,
        /// The minimum version the bindings were generated for.
        required: u32,
        /// Each candidate and the version it reports, most preferred first.
        found: Vec<(PathBuf, u32)>,
    },
}

/// Why the constructor passed to [`open_first`] rejected one candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rejection {
    /// Loader failure, wrong ABI, missing entry point, ...
    Failed(String),
    /// The library loaded but reports a version older than the bindings require
    /// (see [`check_version`]).
    TooOld {
        /// The minimum version the bindings were generated for.
        required: u32,
        /// The version the library reports.
        found: u32,
    },
}

impl From<String> for Rejection {
    fn from(reason: String) -> Self {
        Self::Failed(reason)
    }
}

impl fmt::Display for Rejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failed(reason) => f.write_str(reason),
            Self::TooOld { required, found } => write!(
                f,
                "runtime version {found} is older than the {required} these bindings need"
            ),
        }
    }
}

/// How [`open_library`] asked the OS loader to resolve a library's dependencies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadMode {
    /// Windows: `LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS` (the library's
    /// own directory, the application directory, System32 and `AddDllDirectory` directories).
    SafeDirs,
    /// Windows: `LOAD_WITH_ALTERED_SEARCH_PATH` (the library's own directory, then the standard
    /// search including `PATH`). Used only after [`LoadMode::SafeDirs`] reported a missing
    /// dependency.
    AlteredSearchPath,
    /// Linux: `dlopen` with the dynamic linker's own search.
    Default,
}

impl fmt::Display for LoadMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::SafeDirs => "safe-directories load",
            Self::AlteredSearchPath => "altered-search-path load",
            Self::Default => "load",
        })
    }
}

/// Why [`open_library`] failed: every attempt it made, in order, with the loader's error.
#[derive(Debug)]
pub struct OpenError {
    /// The library file that was opened.
    pub path: PathBuf,
    /// Each attempt and the OS loader's error.
    pub attempts: Vec<(LoadMode, libloading::Error)>,
}

impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:", self.path.display())?;
        for (i, (mode, err)) in self.attempts.iter().enumerate() {
            let sep = if i == 0 { " " } else { "; " };
            // libloading's own message omits the OS cause (`LoadLibraryExW failed`), so add it.
            match std::error::Error::source(err) {
                Some(cause) => write!(f, "{sep}{mode}: {err}: {cause}")?,
                None => write!(f, "{sep}{mode}: {err}")?,
            }
        }
        Ok(())
    }
}

impl std::error::Error for OpenError {}

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

fn show_versions(found: &[(PathBuf, u32)]) -> String {
    found
        .iter()
        .map(|(p, v)| format!("{} reports {v}", p.display()))
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

/// Set by the first [`open_first`] call: from then on some library's result (success or failure)
/// is cached for the process and no longer sees directories added later.
static LOAD_ATTEMPTED: AtomicBool = AtomicBool::new(false);

/// What [`add_search_dir`] did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchDir {
    /// Registered before any library load was attempted: every library will search it.
    Registered,
    /// Registered, but at least one library load was already attempted in this process. Those
    /// results (including failures) are cached and do not see the new directory; libraries loaded
    /// later do. A `Warn` diagnostic event records it.
    RegisteredAfterLoad,
    /// Empty path or already registered: nothing changed.
    Unchanged,
}

/// Registers an extra directory, searched first for every [`Group`] on every OS (together with
/// `NV_DLL_DIRS`, which precedes it).
///
/// Call it before the first library use: loader results are cached per library for the process,
/// so a call after a load attempt cannot change that library's result, and is reported as
/// [`SearchDir::RegisteredAfterLoad`].
pub fn add_search_dir(dir: impl AsRef<Path>) -> SearchDir {
    let path = dir.as_ref();
    if path.as_os_str().is_empty() {
        return SearchDir::Unchanged;
    }
    let mut dirs = EXTRA_SEARCH_DIRS.write().unwrap_or_else(|e| e.into_inner());
    let owned = path.to_path_buf();
    if dirs.contains(&owned) {
        return SearchDir::Unchanged;
    }
    dirs.push(owned);
    drop(dirs);
    if LOAD_ATTEMPTED.load(Ordering::Acquire) {
        diag::record_event(
            diag::Level::Warn,
            "loader",
            "add_search_dir",
            format!(
                "{} registered after a library load was attempted; cached results do not search it",
                path.display()
            ),
            0,
            true,
        );
        SearchDir::RegisteredAfterLoad
    } else {
        SearchDir::Registered
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

/// The version of a `prefix*` match, or `None` when `name` is not one.
///
/// Strips `prefix` and `.dll` (case-insensitively); what remains must be a version: digits
/// separated by `_` or `.` (`cufft64_12.dll` -> `[12]`, `nvrtc64_130_0.dll` -> `[130, 0]`,
/// `cupti64_2026.2.1.dll` -> `[2026, 2, 1]`). Variants such as `nvrtc64_130_0.alt.dll` or another
/// library sharing the prefix are not matches.
pub(crate) fn dll_version_key(name: &str, prefix: &str) -> Option<Vec<u64>> {
    let lower_name = name.to_ascii_lowercase();
    let lower_prefix = prefix.to_ascii_lowercase();
    let suffix = lower_name
        .strip_prefix(&lower_prefix)?
        .strip_suffix(".dll")?;
    if !suffix.starts_with(|c: char| c.is_ascii_digit())
        || !suffix
            .chars()
            .all(|c| c.is_ascii_digit() || c == '_' || c == '.')
    {
        return None;
    }
    suffix
        .split(['_', '.'])
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().ok())
        .collect()
}

/// Candidate paths for `spec` as seen through `probe`, most preferred first.
///
/// On Linux an SDK's discovered runtime directory precedes bare sonames. The dynamic linker's
/// own search (`LD_LIBRARY_PATH`, `ld.so.cache`, `/usr/lib/wsl/lib` on WSL) follows that hint.
pub fn candidates_with(probe: &dyn Probe, spec: &DllSpec) -> Vec<PathBuf> {
    search_plan(probe, spec).candidates
}

/// Candidates plus everything that was looked for (the `NotFound` evidence).
struct SearchPlan {
    /// Paths to try, most preferred first.
    candidates: Vec<PathBuf>,
    /// Every path or `<dir>\<prefix>*.dll` pattern that was looked for.
    searched: Vec<PathBuf>,
}

fn search_plan(probe: &dyn Probe, spec: &DllSpec) -> SearchPlan {
    if !cfg!(windows) {
        let candidates = linux_candidates_with(probe, spec);
        return SearchPlan {
            searched: candidates.clone(),
            candidates,
        };
    }
    let dirs = search_dirs(probe, spec.group);
    let mut candidates: Vec<PathBuf> = Vec::new();
    let mut searched: Vec<PathBuf> = Vec::new();
    for dir in &dirs {
        for name in spec.windows {
            if let Some(prefix) = name.strip_suffix('*') {
                searched.push(dir.join(format!("{prefix}*.dll")));
                let mut matches: Vec<(Vec<u64>, String)> = probe
                    .list_dir(dir)
                    .into_iter()
                    .filter_map(|f| dll_version_key(&f, prefix).map(|v| (v, f)))
                    .collect();
                // Highest version first (`cufft64_12` before `cufft64_11` before `cufft64_9`);
                // equal versions in descending name order, the same direction.
                matches.sort_by(|a, b| b.cmp(a));
                candidates.extend(matches.into_iter().map(|(_, f)| dir.join(f)));
            } else {
                searched.push(dir.join(name));
                candidates.push(dir.join(name));
            }
        }
    }
    // An explicit name and a prefix can produce the same path from non-adjacent entries.
    let mut seen = std::collections::HashSet::new();
    candidates.retain(|p| seen.insert(p.clone()));
    SearchPlan {
        candidates,
        searched,
    }
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
                let mut dirs = vec![root.join("lib64"), root.join("lib")];
                // NGX ships its Linux runtime under `lib/Linux_x86_64/rel`.
                if cfg!(target_arch = "x86_64") {
                    dirs.push(root.join("lib").join("Linux_x86_64").join("rel"));
                }
                for dir in dirs {
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
        // User-supplied directories are an explicit request and come first for every group (the
        // same order the Linux branch uses), then the driver's own location.
        Group::Driver => {
            let mut dirs = extra_search_dirs(probe);
            push_unique_dir(&mut dirs, system32);
            dirs
        }
        // NVIDIA's own order after the user's: System32 first (older drivers copied it there),
        // then the adapters.
        Group::DriverStore => {
            let mut dirs = extra_search_dirs(probe);
            push_unique_dir(&mut dirs, system32);
            for dir in probe.display_driver_dirs() {
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
                // `lib\Windows_x86_64\rel` is where the NGX SDK ships its runtime DLLs.
                for dir in [
                    root.join("bin").join("x64"),
                    root.join("bin"),
                    root.join("lib").join("Windows_x86_64").join("rel"),
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

/// Opens a dynamic library at `path` (every sys crate's loader goes through this one function).
///
/// Windows: loads with `LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS`
/// ([`LoadMode::SafeDirs`]) so dependencies next to `path` resolve without mutating `PATH`. Only
/// when that reports a missing dependency (`ERROR_MOD_NOT_FOUND`) is it retried with
/// `LOAD_WITH_ALTERED_SEARCH_PATH` ([`LoadMode::AlteredSearchPath`]: the library's own directory,
/// then the standard search including `PATH`; needed e.g. by cuDNN's sub-libraries, which import
/// toolkit DLLs from `PATH`). The retry is recorded as a `Warn` diagnostic event, and a failure
/// reports both attempts. A relative `path` is made absolute first, because both flag sets
/// require an absolute path.
pub fn open_library(path: &Path) -> Result<libloading::Library, OpenError> {
    #[cfg(windows)]
    {
        use libloading::os::windows::{
            LOAD_LIBRARY_SEARCH_DEFAULT_DIRS, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR,
            LOAD_WITH_ALTERED_SEARCH_PATH, Library as WinLibrary,
        };
        /// `ERROR_MOD_NOT_FOUND`: the file exists (candidates are existing files) but one of its
        /// dependencies does not resolve.
        const ERROR_MOD_NOT_FOUND: i32 = 126;

        let abs = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
        let mut attempts = Vec::new();
        // SAFETY: loading a dynamic library executes only its loader initialisers; the
        // `LOAD_LIBRARY_SEARCH_*` flags restrict dependency resolution to safe directories.
        let safe = unsafe {
            WinLibrary::load_with_flags(
                &abs,
                LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
            )
        };
        let first = match safe {
            Ok(lib) => return Ok(lib.into()),
            Err(err) => err,
        };
        let missing_dependency = std::error::Error::source(&first)
            .and_then(|cause| cause.downcast_ref::<std::io::Error>())
            .and_then(std::io::Error::raw_os_error)
            == Some(ERROR_MOD_NOT_FOUND);
        attempts.push((LoadMode::SafeDirs, first));
        if missing_dependency {
            // SAFETY: as above; `abs` is absolute, so the library's own directory replaces the
            // application directory at the head of the standard search order.
            match unsafe { WinLibrary::load_with_flags(&abs, LOAD_WITH_ALTERED_SEARCH_PATH) } {
                Ok(lib) => {
                    diag::record_event(
                        diag::Level::Warn,
                        "loader",
                        "open_library",
                        format!(
                            "{}: a dependency is outside the safe directories; loaded with the standard search order (PATH)",
                            abs.display()
                        ),
                        0,
                        true,
                    );
                    return Ok(lib.into());
                }
                Err(err) => attempts.push((LoadMode::AlteredSearchPath, err)),
            }
        }
        Err(OpenError {
            path: abs,
            attempts,
        })
    }
    #[cfg(not(windows))]
    {
        // SAFETY: loading a dynamic library executes only its loader initialisers.
        unsafe { libloading::Library::new(path) }.map_err(|err| OpenError {
            path: path.to_path_buf(),
            attempts: vec![(LoadMode::Default, err)],
        })
    }
}

/// Opens the first candidate for which `open` succeeds.
///
/// `open` receives a path (Windows: an existing file; Linux: an SDK path or linker soname) and
/// returns the loaded value or why it was rejected: a `String` (wrong ABI, missing entry point,
/// ...) or a [`Rejection`] (also [`Rejection::TooOld`] from [`check_version`]).
///
/// The result is cached by every sys crate for the process, so this marks the process as having
/// attempted a load (see [`SearchDir::RegisteredAfterLoad`]).
pub fn open_first<T, E: Into<Rejection>>(
    spec: &DllSpec,
    open: impl FnMut(&Path) -> Result<T, E>,
) -> Result<Loaded<T>, LoadError> {
    LOAD_ATTEMPTED.store(true, Ordering::Release);
    open_first_with(&SystemProbe, spec, open)
}

/// [`open_first`] through an explicit [`Probe`].
pub fn open_first_with<T, E: Into<Rejection>>(
    probe: &dyn Probe,
    spec: &DllSpec,
    mut open: impl FnMut(&Path) -> Result<T, E>,
) -> Result<Loaded<T>, LoadError> {
    let t0 = Instant::now();
    let SearchPlan {
        candidates,
        searched,
    } = search_plan(probe, spec);
    let present: Vec<PathBuf> = if cfg!(windows) {
        candidates.into_iter().filter(|p| probe.is_file(p)).collect()
    } else {
        candidates
    };
    if present.is_empty() {
        let err = LoadError::NotFound {
            lib: spec.lib,
            tried: searched,
        };
        diag::record_lib_load(
            spec.lib,
            None,
            Some(&err.to_string()),
            t0.elapsed().as_nanos() as u64,
        );
        return Err(err);
    }
    let mut failures: Vec<(PathBuf, Rejection)> = Vec::new();
    for path in present {
        match open(&path) {
            Ok(value) => {
                diag::record_lib_load(spec.lib, Some(&path), None, t0.elapsed().as_nanos() as u64);
                return Ok(Loaded { value, path });
            }
            Err(reason) => failures.push((path, reason.into())),
        }
    }
    let err = rejected(spec.lib, failures);
    diag::record_lib_load(
        spec.lib,
        None,
        Some(&err.to_string()),
        t0.elapsed().as_nanos() as u64,
    );
    Err(err)
}

/// The error for a non-empty list of rejected candidates: [`LoadError::RuntimeTooOld`] when every
/// candidate was rejected only for its version, otherwise [`LoadError::Rejected`] with every reason.
fn rejected(lib: &'static str, failures: Vec<(PathBuf, Rejection)>) -> LoadError {
    // All candidates are checked by one sys crate's constructor, so `required` is one value.
    let mut required = None;
    let mut found = Vec::with_capacity(failures.len());
    for (path, reason) in &failures {
        match reason {
            Rejection::TooOld {
                required: r,
                found: v,
            } => {
                required.get_or_insert(*r);
                found.push((path.clone(), *v));
            }
            Rejection::Failed(_) => {
                required = None;
                break;
            }
        }
    }
    match required {
        Some(required) => LoadError::RuntimeTooOld {
            lib,
            required,
            found,
        },
        None => LoadError::Rejected {
            lib,
            failures: failures
                .into_iter()
                .map(|(path, reason)| (path, reason.to_string()))
                .collect(),
        },
    }
}

/// A loaded library value and the file it came from (reported by diagnostics and evidence).
#[derive(Debug)]
pub struct Loaded<T> {
    /// The constructor's result.
    pub value: T,
    /// The file that produced it.
    pub path: PathBuf,
}

/// The cached loader every sys crate uses: one [`open_first`] per process, its result (success or
/// failure) kept in a `OnceLock`.
///
/// Defines a private `loaded()`, the accessor `$vis fn $accessor()` returning the loaded value,
/// and a public `load()` reporting which file was used or why loading failed. `open` is the
/// constructor passed to [`open_first`]; it must open the file through [`open_library`]. Used by
/// [`exports_sys!`], [`split_exports_sys!`], [`query_interface_sys!`] and the function-table sys
/// crates (OptiX, NVENC, NVOFA), so every library goes through the same entry point.
#[macro_export]
macro_rules! cached_loader {
    (dll: $dll:expr, value: $ty:ty, open: $open:expr, $(#[$meta:meta])* $vis:vis fn $accessor:ident $(,)?) => {
        fn loaded() -> &'static ::core::result::Result<$crate::Loaded<$ty>, $crate::LoadError> {
            static VALUE: ::std::sync::OnceLock<
                ::core::result::Result<$crate::Loaded<$ty>, $crate::LoadError>,
            > = ::std::sync::OnceLock::new();
            VALUE.get_or_init(|| $crate::open_first(&$dll, $open))
        }

        $(#[$meta])*
        $vis fn $accessor() -> ::core::result::Result<&'static $ty, &'static $crate::LoadError> {
            loaded().as_ref().map(|l| &l.value)
        }

        /// Loads the library now and reports which file was used, or why loading failed.
        pub fn load()
        -> ::core::result::Result<&'static ::std::path::Path, &'static $crate::LoadError> {
            loaded().as_ref().map(|l| l.path.as_path())
        }
    };
}

/// Opens one exports-shaped library file for [`exports_sys!`]: optional `preload` (libraries that
/// must be mapped first and stay loaded), [`open_library`], the bindgen loading struct, then the
/// optional `version` check. Not public API.
#[doc(hidden)]
#[macro_export]
macro_rules! __exports_open {
    ($path:expr, $api:ty, [$($preload:path)?], [$($version:path)?]) => {{
        let path: &::std::path::Path = $path;
        #[allow(unused_mut)]
        let mut keep: ::std::vec::Vec<::libloading::Library> = ::std::vec::Vec::new();
        $( keep = $preload(path)?; )?
        let lib = $crate::open_library(path).map_err(|e| e.to_string())?;
        // SAFETY: opening the library runs only its loader initialisers; symbols are resolved
        // lazily and a missing one becomes an `Err` field, not a crash.
        let api = unsafe { <$api>::from_library(lib) }.map_err(|e| e.to_string())?;
        $( $version(&api)?; )?
        ::core::result::Result::Ok($crate::ExportsBundle { api, _keep: keep })
    }};
}

/// A loaded exports-shaped library: the bindgen loading struct plus libraries that must stay
/// mapped for it (cuDNN's sub-libraries; empty for every other library).
#[doc(hidden)]
pub struct ExportsBundle<A> {
    /// The bindgen loading struct.
    pub api: A,
    /// Libraries kept loaded for the process together with `api`.
    pub _keep: Vec<libloading::Library>,
}

/// The body of an exports-shaped sys crate (one `nv-bindgen` [`Shape::Exports`] library).
///
/// Includes the generated `<lib>_types.rs`, `<lib>_api.rs` and `<lib>_shims.rs` from `OUT_DIR`,
/// re-exports types and shims, keeps the bindgen loading struct private (its methods `expect` a
/// symbol and would panic), and defines the cached loader the shims call plus a public
/// diagnostic `load()` and the `INVENTORY` evidence string. One definition for every sys crate.
///
/// Optional hooks, in this order: `preload: fn(&Path) -> Result<Vec<libloading::Library>, String>`
/// maps libraries the main one needs first and keeps them loaded (cuDNN); `version:
/// fn(&api::<Api>) -> Result<(), nv_loader::Rejection>` rejects a runtime older than the bound
/// headers (see [`check_version`]).
///
/// [`Shape::Exports`]: https://docs.rs/nv-bindgen
#[macro_export]
macro_rules! exports_sys {
    (lib: $lib:literal, api: $api:ident, loader: $loader:ident, dll: $dll:expr
     $(, preload: $preload:path)? $(, version: $version:path)? $(,)?) => {
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

        $crate::cached_loader! {
            dll: DLL,
            value: $crate::ExportsBundle<api::$api>,
            open: |p: &::std::path::Path|
             -> ::core::result::Result<$crate::ExportsBundle<api::$api>, $crate::Rejection> {
                $crate::__exports_open!(p, api::$api, [$($preload)?], [$($version)?])
            },
            /// The loaded library and the libraries it keeps mapped.
            fn bundle
        }

        /// The loaded library, for the shims.
        fn $loader() -> ::core::result::Result<&'static api::$api, &'static $crate::LoadError> {
            bundle().map(|b| &b.api)
        }
    };
}

/// Rejects a library whose reported version is older than the version the bindings were
/// generated for: `Err(`[`Rejection::TooOld`]`)` when `found < required`. Constructors passed to
/// [`open_first`] return it, and [`open_first`] reports [`LoadError::RuntimeTooOld`] when every
/// candidate was too old.
pub fn check_version(required: u32, found: u32) -> Result<(), Rejection> {
    if found < required {
        Err(Rejection::TooOld { required, found })
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

        $crate::cached_loader! {
            dll: DLL,
            value: $crate::QueryInterface,
            open: |p: &::std::path::Path| {
                // SAFETY: the query symbol named by the sys crate has the `void* (unsigned int)`
                // signature.
                unsafe { $crate::QueryInterface::open(p, $sym) }
            },
            /// The loaded query, for the shims.
            fn $loader
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

                $crate::cached_loader! {
                    dll: DLL,
                    value: $crate::ExportsBundle<api::$api>,
                    open: |p: &::std::path::Path|
                     -> ::core::result::Result<$crate::ExportsBundle<api::$api>, $crate::Rejection> {
                        $crate::__exports_open!(p, api::$api, [], [])
                    },
                    /// This DLL's loading struct.
                    fn bundle
                }

                /// The loaded DLL, for the shims.
                fn $loader() -> ::core::result::Result<&'static api::$api, &'static $crate::LoadError> {
                    bundle().map(|b| &b.api)
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
    use super::test_support::{ExtraDirsGuard, Fake, read_extra_dirs_gate};
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
    fn linux_add_search_dir_and_nv_dll_dirs_precede_runtime_root_and_sonames() {
        let _guard = ExtraDirsGuard::new();
        add_search_dir("/opt/extra_nv");
        add_search_dir("/opt/extra_nv");
        let spec = DllSpec {
            lib: "cudnn",
            group: Group::Sdk {
                root_var: "CUDNN_ROOT",
                hint: Some("/opt/hint/lib"),
            },
            windows: &[],
            linux: &["libcudnn.so.9"],
        };
        let mut fake = Fake::default();
        fake.env.insert("NV_DLL_DIRS", "/opt/env_nv");
        fake.env.insert("CUDNN_ROOT", "/opt/cudnn");
        let got = linux_candidates_with(&fake, &spec);
        let mut expected = vec![
            PathBuf::from("/opt/env_nv/libcudnn.so.9"),
            PathBuf::from("/opt/extra_nv/libcudnn.so.9"),
            PathBuf::from("/opt/cudnn/lib64/libcudnn.so.9"),
            PathBuf::from("/opt/cudnn/lib/libcudnn.so.9"),
        ];
        if cfg!(target_arch = "x86_64") {
            expected.push(PathBuf::from("/opt/cudnn/lib/Linux_x86_64/rel/libcudnn.so.9"));
        }
        expected.extend([
            PathBuf::from("/opt/hint/lib/libcudnn.so.9"),
            PathBuf::from("libcudnn.so.9"),
        ]);
        assert_eq!(got, expected);
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
        let mut expected = vec![
            PathBuf::from("/custom/lib/libnccl.so.2"),
            PathBuf::from("/opt/nccl/lib64/libnccl.so.2"),
            PathBuf::from("/opt/nccl/lib/libnccl.so.2"),
        ];
        // Mutation that reddens this (L-09): drop the NGX runtime directory from the Linux branch.
        if cfg!(target_arch = "x86_64") {
            expected.push(PathBuf::from("/opt/nccl/lib/Linux_x86_64/rel/libnccl.so.2"));
        }
        expected.extend([
            PathBuf::from("/build/nccl/lib/libnccl.so.2"),
            PathBuf::from("libnccl.so.2"),
        ]);
        assert_eq!(linux_candidates_with(&fake, &sdk_spec), expected);

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

    /// A spec whose only candidates are fake paths the test's constructor decides about.
    fn versioned_spec() -> (Fake, DllSpec) {
        let mut fake = Fake::default();
        fake.env.insert("SystemRoot", r"C:\Windows");
        fake.drivers = vec![PathBuf::from(r"C:\DS\a"), PathBuf::from(r"C:\DS\b")];
        for dir in [r"C:\DS\a", r"C:\DS\b"] {
            fake.dirs.insert(PathBuf::from(dir), vec!["libver.dll"]);
        }
        let spec = DllSpec {
            lib: "ver",
            group: Group::DriverStore,
            windows: &["libver.dll"],
            linux: &["/a/libver.so.1", "/b/libver.so.1"],
        };
        (fake, spec)
    }

    #[test]
    fn every_candidate_too_old_is_a_typed_runtime_too_old_error() {
        let _gate = read_extra_dirs_gate();
        let (fake, spec) = versioned_spec();
        // Before L-06 nothing called `check_version`, so a too-old runtime was never typed.
        let err = open_first_with(&fake, &spec, |_| check_version(13, 12))
            .expect_err("every candidate is too old");
        match err {
            LoadError::RuntimeTooOld {
                lib,
                required,
                found,
            } => {
                assert_eq!((lib, required), ("ver", 13));
                assert_eq!(found.len(), 2, "every too-old candidate is reported");
                assert!(found.iter().all(|(_, v)| *v == 12));
            }
            other => panic!("expected RuntimeTooOld, got {other}"),
        }
    }

    #[test]
    fn a_mixed_rejection_keeps_every_reason_including_the_version() {
        let _gate = read_extra_dirs_gate();
        let (fake, spec) = versioned_spec();
        let mut first = true;
        let err = open_first_with(&fake, &spec, |_| -> Result<(), Rejection> {
            if std::mem::take(&mut first) {
                check_version(13, 12)
            } else {
                Err(String::from("missing entry point").into())
            }
        })
        .expect_err("nothing loads");
        let LoadError::Rejected { failures, .. } = &err else {
            panic!("expected Rejected, got {err}");
        };
        assert_eq!(failures.len(), 2);
        assert!(failures[0].1.contains("older than the 13"), "{err}");
        assert!(failures[1].1.contains("missing entry point"), "{err}");
    }

    #[test]
    fn a_newer_runtime_passes_the_version_gate() {
        let _gate = read_extra_dirs_gate();
        let (fake, spec) = versioned_spec();
        let loaded = open_first_with(&fake, &spec, |_| check_version(13, 14).map(|()| 5))
            .expect("13 <= 14");
        assert_eq!(loaded.value, 5);
    }

    #[test]
    fn open_library_reports_its_attempt_and_the_os_cause() {
        let dir = std::env::temp_dir().join(format!(
            "nv_loader_open_library_{}_{}",
            std::process::id(),
            line!()
        ));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let not_a_library = dir.join("not_a_library.dll");
        std::fs::write(&not_a_library, b"plain text, not a PE or ELF image").expect("write");
        let err = open_library(&not_a_library).expect_err("a text file is not a library");
        // Not a missing dependency, so the safe load is not retried with a wider search.
        assert_eq!(err.attempts.len(), 1, "{err}");
        let expected_mode = if cfg!(windows) {
            LoadMode::SafeDirs
        } else {
            LoadMode::Default
        };
        assert_eq!(err.attempts[0].0, expected_mode);
        assert!(err.path.is_absolute(), "{}", err.path.display());
        assert!(err.to_string().contains("not_a_library.dll"), "{err}");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn add_search_dir_after_a_load_attempt_is_reported() {
        let _guard = ExtraDirsGuard::new();
        let spec = DllSpec {
            lib: "absent",
            group: Group::Sdk {
                root_var: "NV_RS_TEST_ROOT_NEVER_SET",
                hint: None,
            },
            windows: &["nv-rs-test-absent-library.dll"],
            linux: &["libnv-rs-test-absent-library.so"],
        };
        // The real entry point marks the process; the constructor never opens anything.
        assert!(open_first(&spec, |_| Err::<(), String>("not opened by this test".into())).is_err());
        assert_eq!(
            add_search_dir("/nv-rs/test/late-dir"),
            SearchDir::RegisteredAfterLoad
        );
        assert_eq!(add_search_dir("/nv-rs/test/late-dir"), SearchDir::Unchanged);
        assert_eq!(add_search_dir(""), SearchDir::Unchanged);
    }

    #[test]
    fn the_build_hint_exists_only_in_debug_builds_and_only_for_sdks() {
        const SDK: DllSpec = DllSpec {
            lib: "s",
            group: Group::Sdk {
                root_var: "S_ROOT",
                hint: None,
            },
            windows: &[],
            linux: &[],
        };
        let expected = if cfg!(debug_assertions) {
            Some("/build/sdk/bin")
        } else {
            None
        };
        assert!(matches!(
            SDK.with_hint("/build/sdk/bin").group,
            Group::Sdk { root_var: "S_ROOT", hint } if hint == expected
        ));
        assert!(matches!(
            SDK.with_hint("").group,
            Group::Sdk { hint: None, .. }
        ));
        let driver = DllSpec {
            group: Group::Driver,
            ..SDK
        };
        assert_eq!(driver.with_hint("/build/sdk/bin").group, Group::Driver);
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
    fn toolkit_prefix_scan_orders_numeric_versions_correctly_over_lexical() {
        let _gate = read_extra_dirs_gate();
        // Lexical reverse sorting puts "cufft64_9.dll" ahead of "cufft64_11.dll"
        // because '9' > '1'. Numeric version ordering must guarantee 12 > 11 > 9.
        let mut fake = Fake::default();
        fake.env.insert("CUDA_PATH", r"C:\cuda");
        fake.dirs.insert(
            PathBuf::from(r"C:\cuda\bin\x64"),
            vec!["cufft64_9.dll", "cufft64_11.dll", "cufft64_12.dll"],
        );
        let got = candidates_with(&fake, &CUFFT);
        assert_eq!(
            got,
            vec![
                PathBuf::from(r"C:\cuda\bin\x64\cufft64_12.dll"),
                PathBuf::from(r"C:\cuda\bin\x64\cufft64_11.dll"),
                PathBuf::from(r"C:\cuda\bin\x64\cufft64_9.dll"),
            ]
        );
    }

    #[test]
    fn dll_version_key_accepts_only_version_suffixes() {
        assert_eq!(dll_version_key("cufft64_12.dll", "cufft64_"), Some(vec![12]));
        assert_eq!(dll_version_key("cufft64_9.dll", "cufft64_"), Some(vec![9]));
        assert_eq!(dll_version_key("CUFFT64_12.DLL", "cufft64_"), Some(vec![12]));
        assert_eq!(
            dll_version_key("nvrtc64_130_0.dll", "nvrtc64_"),
            Some(vec![130, 0])
        );
        assert_eq!(
            dll_version_key("cupti64_2026.2.1.dll", "cupti64_"),
            Some(vec![2026, 2, 1])
        );
        // L-12: the `.alt` NVRTC variant and neighbours sharing the prefix are not candidates.
        assert_eq!(dll_version_key("nvrtc64_130_0.alt.dll", "nvrtc64_"), None);
        assert_eq!(dll_version_key("cufft64_12.lib", "cufft64_"), None);
        assert_eq!(dll_version_key("random.dll", "cufft64_"), None);
        assert_eq!(dll_version_key("cufft64_.dll", "cufft64_"), None);
    }

    #[test]
    fn the_alt_variant_is_never_preferred_and_duplicates_are_removed() {
        let _gate = read_extra_dirs_gate();
        let spec = DllSpec {
            lib: "nvrtc",
            group: Group::Toolkit,
            windows: &["nvrtc64_130_0.dll", "nvrtc64_*"],
            linux: &[],
        };
        let mut fake = Fake::default();
        fake.env.insert("CUDA_PATH", r"C:\cuda");
        fake.dirs.insert(
            PathBuf::from(r"C:\cuda\bin\x64"),
            vec!["nvrtc64_130_0.alt.dll", "nvrtc64_130_0.dll", "nvrtc64_120_0.dll"],
        );
        let got: Vec<PathBuf> = candidates_with(&fake, &spec)
            .into_iter()
            .filter(|p| fake.is_file(p))
            .collect();
        assert_eq!(
            got,
            vec![
                PathBuf::from(r"C:\cuda\bin\x64\nvrtc64_130_0.dll"),
                PathBuf::from(r"C:\cuda\bin\x64\nvrtc64_120_0.dll"),
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
                Err(String::from("ABI 118 unsupported"))
            } else {
                Ok(7)
            }
        })
        .expect("second candidate opens");
        assert_eq!(
            (loaded.value, loaded.path),
            (7, PathBuf::from(r"C:\DS\new\nvoptix.dll"))
        );

        let err = open_first_with(&fake, &OPTIX, |_| Err::<(), String>("nope".into()))
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
        let err = open_first_with(&fake, &OPTIX, |_| Ok::<(), String>(()))
            .expect_err("nothing installed");
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
        assert!(check_version(12000, 12000).is_ok());
        assert!(check_version(12000, 12100).is_ok());
        assert_eq!(
            check_version(12000, 11800),
            Err(Rejection::TooOld {
                required: 12000,
                found: 11800
            })
        );
    }

    #[test]
    fn not_found_names_every_searched_pattern() {
        let _gate = read_extra_dirs_gate();
        // L-07: a prefix spec with nothing installed used to report `searched ` with no paths.
        let spec = DllSpec {
            lib: "cudnn",
            group: Group::Sdk {
                root_var: "CUDNN_ROOT",
                hint: None,
            },
            windows: &["cudnn64_*"],
            linux: &[],
        };
        let mut fake = Fake::default();
        fake.env.insert("CUDNN_ROOT", r"C:\cudnn");
        let err = open_first_with(&fake, &spec, |_| Ok::<(), String>(())).expect_err("absent");
        let LoadError::NotFound { tried, .. } = &err else {
            panic!("expected NotFound, got {err}");
        };
        assert_eq!(tried.first(), Some(&PathBuf::from(r"C:\cudnn\bin\x64\cudnn64_*.dll")));
        assert!(err.to_string().contains(r"C:\cudnn\bin\cudnn64_*.dll"), "{err}");
    }

    #[test]
    fn nv_dll_dirs_precede_the_driver_locations_like_on_linux() {
        let _gate = read_extra_dirs_gate();
        // L-08: Driver used to ignore NV_DLL_DIRS and DriverStore searched it last.
        let mut fake = Fake::default();
        fake.env.insert("SystemRoot", r"C:\Windows");
        fake.env.insert("NV_DLL_DIRS", r"C:\override");
        fake.drivers = vec![PathBuf::from(r"C:\DS\nv_a")];
        let nvml = DllSpec {
            lib: "nvml",
            group: Group::Driver,
            windows: &["nvml.dll"],
            linux: &[],
        };
        assert_eq!(
            candidates_with(&fake, &nvml),
            vec![
                PathBuf::from(r"C:\override\nvml.dll"),
                PathBuf::from(r"C:\Windows\System32\nvml.dll"),
            ]
        );
        assert_eq!(
            candidates_with(&fake, &OPTIX),
            vec![
                PathBuf::from(r"C:\override\nvoptix.dll"),
                PathBuf::from(r"C:\Windows\System32\nvoptix.dll"),
                PathBuf::from(r"C:\DS\nv_a\nvoptix.dll"),
            ]
        );
    }

    #[test]
    fn sdk_roots_on_windows_never_search_the_linux_runtime_directory() {
        let _gate = read_extra_dirs_gate();
        // L-09: `lib\Linux_x86_64\rel` used to be searched on Windows.
        let spec = DllSpec {
            lib: "dlss",
            group: Group::Sdk {
                root_var: "NGX_SDK_ROOT",
                hint: None,
            },
            windows: &["nvngx_dlss.dll"],
            linux: &[],
        };
        let mut fake = Fake::default();
        fake.env.insert("NGX_SDK_ROOT", r"C:\ngx");
        let got = candidates_with(&fake, &spec);
        assert!(got.contains(&PathBuf::from(r"C:\ngx\lib\Windows_x86_64\rel\nvngx_dlss.dll")));
        assert!(
            !got.iter().any(|p| p.to_string_lossy().contains("Linux_x86_64")),
            "{got:?}"
        );
    }
}
