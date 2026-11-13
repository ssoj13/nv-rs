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
    /// A separately installed SDK whose root is named by an environment variable
    /// (`CUDNN_ROOT`, `NVCOMP_ROOT`, ...). Never guessed when the variable is unset.
    Sdk {
        /// The environment variable naming the SDK root.
        root_var: &'static str,
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
/// On Linux the sonames are returned as bare names: the dynamic linker's own search
/// (`LD_LIBRARY_PATH`, `ld.so.cache`, `/usr/lib/wsl/lib` on WSL) is the authority there.
pub fn candidates_with(probe: &dyn Probe, spec: &DllSpec) -> Vec<PathBuf> {
    if !cfg!(windows) {
        return spec.linux.iter().map(PathBuf::from).collect();
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
                    for dir in [root.join("bin").join("x64"), root.join("bin")] {
                        if !dirs.contains(&dir) {
                            dirs.push(dir);
                        }
                    }
                }
            }
            dirs
        }
        Group::Sdk { root_var } => probe
            .env(root_var)
            .map(|root| {
                let root = PathBuf::from(root);
                vec![root.join("bin").join("x64"), root.join("bin"), root.join("lib")]
            })
            .unwrap_or_default(),
    }
}

/// Opens the first candidate for which `open` succeeds.
///
/// `open` receives a path (Windows: an existing file; Linux: a soname for the dynamic linker) and
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
            Group::Sdk { root_var } => write!(f, "SDK ({root_var})"),
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
    }

    impl Probe for Fake {
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
        let spec = DllSpec { lib: "cudnn", group: Group::Sdk { root_var: "CUDNN_ROOT" }, windows: &["cudnn64_*"], linux: &[] };
        assert!(candidates_with(&Fake::default(), &spec).is_empty());
    }

    #[test]
    fn version_gate_rejects_only_older_runtimes() {
        assert!(check_version("nvml", 12000, 12000).is_ok());
        assert!(check_version("nvml", 12000, 12100).is_ok());
        assert!(matches!(check_version("nvml", 12000, 11800), Err(LoadError::RuntimeTooOld { found: 11800, .. })));
    }
}
