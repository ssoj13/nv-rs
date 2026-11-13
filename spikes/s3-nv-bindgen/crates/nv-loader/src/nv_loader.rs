//! The one runtime loader for every NVIDIA library in nv-rs (spike S3 prototype).
//!
//! A sys crate describes where its DLL lives ([`DllSpec`]: group + file names / prefixes) and
//! hands a constructor to [`open_first`]; this module walks the candidates in order, returns the
//! first that opens and resolves, and otherwise a [`LoadError`] listing every path it tried.
//! Discovery per group follows PLAN.md §3: driver DLLs in System32, DriverStore-only DLLs
//! (nvoptix) in the display driver's FileRepository folder, toolkit DLLs under `CUDA_PATH` with a
//! version-suffix prefix scan. Linux sonames are listed but untested here.

use std::path::{Path, PathBuf};

/// Where a library ships.
#[derive(Clone, Copy, Debug)]
pub enum Group {
    /// Installed by the display driver into System32 (`nvml.dll`, `nvEncodeAPI64.dll`).
    Driver,
    /// Installed by the display driver into DriverStore only (`nvoptix.dll`, `nvngx.dll`).
    DriverStore,
    /// Part of the CUDA Toolkit (`cufft64_12.dll`), found under `CUDA_PATH`.
    Toolkit,
}

/// File-name rules for one library. A Windows entry ending in `*` is a prefix: the first matching
/// `.dll` in the directory wins (toolkit DLLs carry per-library version suffixes).
#[derive(Clone, Copy, Debug)]
pub struct DllSpec {
    pub lib: &'static str,
    pub group: Group,
    pub windows: &'static [&'static str],
    pub linux: &'static [&'static str],
}

#[derive(Debug)]
pub enum LoadError {
    NotFound { lib: &'static str, tried: Vec<PathBuf> },
    Rejected { lib: &'static str, failures: Vec<(PathBuf, String)> },
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound { lib, tried } => write!(f, "{lib}: no candidate found (tried {tried:?})"),
            Self::Rejected { lib, failures } => write!(f, "{lib}: every candidate failed: {failures:?}"),
        }
    }
}

impl std::error::Error for LoadError {}

/// Candidate paths for `spec`, most specific first.
pub fn candidates(spec: &DllSpec) -> Vec<PathBuf> {
    if !cfg!(windows) {
        return spec.linux.iter().map(PathBuf::from).collect();
    }
    let mut dirs: Vec<PathBuf> = Vec::new();
    match spec.group {
        Group::Driver => dirs.push(system32()),
        Group::DriverStore => dirs.extend(driver_store_dirs()),
        Group::Toolkit => {
            for var in ["CUDA_PATH", "CUDA_HOME", "CUDA_TOOLKIT_PATH"] {
                if let Some(root) = std::env::var_os(var) {
                    let root = PathBuf::from(root);
                    dirs.push(root.join("bin").join("x64"));
                    dirs.push(root.join("bin"));
                }
            }
        }
    }
    let mut out = Vec::new();
    for dir in &dirs {
        for name in spec.windows {
            if let Some(prefix) = name.strip_suffix('*') {
                out.extend(prefix_matches(dir, prefix));
            } else {
                out.push(dir.join(name));
            }
        }
    }
    out
}

/// Opens the first candidate for which `open` succeeds.
pub fn open_first<T>(
    spec: &DllSpec,
    mut open: impl FnMut(&Path) -> Result<T, String>,
) -> Result<(T, PathBuf), LoadError> {
    let tried: Vec<PathBuf> = candidates(spec).into_iter().filter(|p| p.exists() || !cfg!(windows)).collect();
    if tried.is_empty() {
        return Err(LoadError::NotFound { lib: spec.lib, tried: candidates(spec) });
    }
    let mut failures = Vec::new();
    for path in tried {
        match open(&path) {
            Ok(v) => return Ok((v, path)),
            Err(e) => failures.push((path, e)),
        }
    }
    Err(LoadError::Rejected { lib: spec.lib, failures })
}

fn system32() -> PathBuf {
    let root = std::env::var_os("SystemRoot").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(r"C:\Windows"));
    root.join("System32")
}

/// Display-driver package folders. Spike shortcut: scan FileRepository; the real loader resolves
/// the active adapter's driver folder from the registry (as `optix_stubs.h` does).
fn driver_store_dirs() -> Vec<PathBuf> {
    let repo = system32().join("DriverStore").join("FileRepository");
    let Ok(entries) = std::fs::read_dir(&repo) else { return Vec::new() };
    let mut dirs: Vec<(std::time::SystemTime, PathBuf)> = entries
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().starts_with("nv_dispi.inf_amd64_"))
        .map(|e| (e.metadata().and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH), e.path()))
        .collect();
    dirs.sort_by(|a, b| b.0.cmp(&a.0)); // newest driver package first
    dirs.into_iter().map(|(_, p)| p).collect()
}

fn prefix_matches(dir: &Path, prefix: &str) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut found: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|x| x.eq_ignore_ascii_case("dll"))
                && p.file_name().is_some_and(|n| n.to_string_lossy().to_ascii_lowercase().starts_with(&prefix.to_ascii_lowercase()))
        })
        .collect();
    found.sort();
    found.reverse(); // highest version suffix first
    found
}
