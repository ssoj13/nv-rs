//! Stages NVIDIA runtime DLLs into a target directory for bundling or distribution.
//!
//! **What.** Implements `cargo xtask stage-dlls --out <dir> [--libs <csv>] [--include-driver]
//! [--dry-run]`. For each selected library of [`nv_loader::catalog::ALL`] it locates the first
//! existing runtime file with [`nv_loader::candidates`] and copies it, together with the
//! library's companion files, into `--out` atomically ([`crate::atomic_io::copy`]), skipping files
//! that already match in size and modification time. A library named in `--libs` that is not
//! found fails the command (non-zero exit, nothing copied); without `--libs`, missing libraries
//! are reported as warnings. Libraries that are optional by definition stay warnings either way:
//! driver-installed ones without `--include-driver`, and ones with no file name on the current
//! platform. A library that is found but lacks a *required* companion file is always an error:
//! such a bundle fails at first use on a machine without the SDK.
//!
//! **Why.** Applications and Python wheels built against `nv-rs` need redistributable CUDA
//! Toolkit and SDK DLLs next to their binaries or in a staging directory without hand-hunting
//! versioned filenames (`cublas64_13.dll`, `cufft64_12.dll`, ...), and the files a library loads
//! by bare name at run time (cuDNN's sub-libraries, NVRTC's builtins, DLSS and DLSS-RR).
//!
//! **How.** There is no list here: the library table is [`nv_loader::catalog::ALL`], the same
//! one the sys crates and `nv_loader::diag::probe_all` read, including each entry's
//! [`nv_loader::catalog::Companion`] patterns (matched in the directory of the main file with
//! [`nv_loader::glob_match`]). Filters to redistributable [`Group::Toolkit`] and [`Group::Sdk`]
//! libraries by default, or includes [`Group::Driver`] and [`Group::DriverStore`] when
//! `--include-driver` is passed, with optional `--libs` CSV filtering. Failures are the typed
//! [`StageError`], every variant naming the library.
//!
//! **Used by** `xtask` (`xtask.rs`).

use nv_loader::catalog::{self, CatalogEntry};
use nv_loader::{DllSpec, Group};
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const USAGE: &str = "\
usage: cargo xtask stage-dlls|d --out <dir> [--libs <csv>] [--include-driver] [--dry-run]
  --libs <csv>      stage only these libraries; one that is not found fails the command
  --include-driver  also stage driver-installed libraries (nvml, nvapi, optix, ...)
  --dry-run         print what would be copied without copying
example: cargo xtask stage-dlls --out dist --libs cufft,nvjpeg,nvrtc,dlss,dlssd";

/// Why `stage-dlls` failed.
#[derive(Debug, thiserror::Error)]
enum StageError {
    /// `--libs` names something the catalog does not list.
    #[error("unknown library name(s): {names} (available: {available})")]
    UnknownLibs {
        /// The unknown names, comma-separated.
        names: String,
        /// Every catalog name, comma-separated.
        available: String,
    },
    /// Requested libraries were not found on disk.
    #[error(
        "stage-dlls: requested {noun} not found: {names}; nothing was copied to {out}{details}"
    )]
    MissingLibs {
        /// `library` or `libraries`.
        noun: &'static str,
        /// The missing library names, comma-separated.
        names: String,
        /// The destination that was left untouched.
        out: String,
        /// One `\n  missing <lib>: <why>` line per library.
        details: String,
    },
    /// A found library lacks a file it loads at run time.
    #[error(
        "stage-dlls: {lib}: required companion file `{pattern}` not found next to {main}; nothing was copied"
    )]
    MissingCompanion {
        /// The library.
        lib: &'static str,
        /// The companion pattern that matched nothing.
        pattern: &'static str,
        /// The main library file whose directory was searched.
        main: PathBuf,
    },
    /// A directory could not be listed or created, or a file could not be copied.
    #[error("stage-dlls: {lib}: {message}")]
    Io {
        /// The library (or `-` when none applies).
        lib: &'static str,
        /// The operation and the OS error.
        message: String,
    },
}

/// Runs `cargo xtask stage-dlls`.
pub fn run(args: &[&str]) -> Result<String, String> {
    if args.iter().any(|a| matches!(*a, "-h" | "--help")) {
        return Ok(USAGE.to_string());
    }
    let opts = parse_args(args)?;
    stage_with(&opts, catalog::ALL, find_first_existing).map_err(|e| e.to_string())
}

#[derive(Debug, PartialEq, Eq)]
struct Opts {
    out: PathBuf,
    libs: Option<BTreeSet<String>>,
    include_driver: bool,
    dry_run: bool,
}

fn parse_args(args: &[&str]) -> Result<Opts, String> {
    let mut out: Option<PathBuf> = None;
    let mut libs: Option<BTreeSet<String>> = None;
    let mut include_driver = false;
    let mut dry_run = false;

    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "--out" => {
                i += 1;
                let val = args
                    .get(i)
                    .ok_or_else(|| format!("--out requires a directory\n{USAGE}"))?;
                out = Some(PathBuf::from(val));
            }
            "--libs" => {
                i += 1;
                let val = args
                    .get(i)
                    .ok_or_else(|| format!("--libs requires a comma-separated list\n{USAGE}"))?;
                let parsed: BTreeSet<String> = val
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_ascii_lowercase)
                    .collect();
                if parsed.is_empty() {
                    return Err(format!("--libs cannot be empty\n{USAGE}"));
                }
                libs = Some(parsed);
            }
            "--include-driver" => include_driver = true,
            "--dry-run" => dry_run = true,
            other => {
                return Err(format!("unknown argument `{other}`\n{USAGE}"));
            }
        }
        i += 1;
    }

    let out = out.ok_or_else(|| format!("--out <dir> is required\n{USAGE}"))?;
    Ok(Opts {
        out,
        libs,
        include_driver,
        dry_run,
    })
}

fn is_selected_group(group: Group, include_driver: bool) -> bool {
    match group {
        Group::Toolkit | Group::Sdk { .. } => true,
        Group::Driver | Group::DriverStore => include_driver,
    }
}

/// The file names `spec` lists for the OS xtask runs on (the loader searches the same list).
fn platform_names(spec: &DllSpec) -> &'static [&'static str] {
    if cfg!(windows) {
        spec.windows
    } else {
        spec.linux
    }
}

enum CandidateOutcome {
    Found(PathBuf),
    Missing(String),
}

fn find_first_existing(entry: &CatalogEntry) -> CandidateOutcome {
    let spec = &entry.spec;
    let candidates = nv_loader::candidates(spec);
    if let Some(found) = candidates.iter().find(|p| p.is_file()) {
        return CandidateOutcome::Found(found.clone());
    }
    let reason = if candidates.is_empty() {
        match spec.group {
            Group::Toolkit => "no CUDA toolkit search dirs (check CUDA_PATH)".to_string(),
            Group::Sdk { root_var, .. } => format!("no search dirs (check {root_var})"),
            Group::Driver | Group::DriverStore => {
                "no driver candidates on this platform".to_string()
            }
        }
    } else {
        format!(
            "not found on disk (checked {} candidate(s))",
            candidates.len()
        )
    };
    CandidateOutcome::Missing(reason)
}

/// The files next to `main` that `entry` lists as companions for this OS, sorted. A required
/// pattern without a match is [`StageError::MissingCompanion`]; an optional one is skipped.
fn companion_files(entry: &CatalogEntry, main: &Path) -> Result<Vec<PathBuf>, StageError> {
    let companions = entry.companions();
    if companions.is_empty() {
        return Ok(Vec::new());
    }
    let lib = entry.name;
    let dir = main.parent().ok_or_else(|| StageError::Io {
        lib,
        message: format!("{} has no parent directory", main.display()),
    })?;
    let mut names: Vec<String> = Vec::new();
    let listing = std::fs::read_dir(dir).map_err(|e| StageError::Io {
        lib,
        message: format!("listing {}: {e}", dir.display()),
    })?;
    for item in listing {
        let item = item.map_err(|e| StageError::Io {
            lib,
            message: format!("listing {}: {e}", dir.display()),
        })?;
        // Companion names are ASCII library names; anything else cannot match a pattern.
        if let Ok(name) = item.file_name().into_string() {
            names.push(name);
        }
    }
    let mut files: Vec<PathBuf> = Vec::new();
    for companion in companions {
        let mut matched: Vec<&String> = names
            .iter()
            .filter(|n| nv_loader::glob_match(companion.pattern, n))
            .collect();
        matched.sort();
        if matched.is_empty() && companion.required {
            return Err(StageError::MissingCompanion {
                lib,
                pattern: companion.pattern,
                main: main.to_path_buf(),
            });
        }
        files.extend(matched.into_iter().map(|n| dir.join(n)));
    }
    Ok(files)
}

fn stage_with(
    opts: &Opts,
    entries: &[CatalogEntry],
    mut find_candidate: impl FnMut(&CatalogEntry) -> CandidateOutcome,
) -> Result<String, StageError> {
    if let Some(filter) = &opts.libs {
        let known: BTreeSet<&str> = entries.iter().map(|e| e.name).collect();
        let unknown: Vec<&str> = filter
            .iter()
            .map(String::as_str)
            .filter(|name| !known.contains(name))
            .collect();
        if !unknown.is_empty() {
            let mut avail: Vec<&str> = known.into_iter().collect();
            avail.sort_unstable();
            return Err(StageError::UnknownLibs {
                names: unknown.join(", "),
                available: avail.join(", "),
            });
        }
    }

    // Resolve every selected library (and its companions) before copying anything, so a missing
    // requested library or companion fails the command without leaving a partial bundle behind.
    let mut found: Vec<(&'static str, PathBuf, PathBuf)> = Vec::new();
    let mut planned: BTreeSet<PathBuf> = BTreeSet::new();
    let mut skipped: Vec<(String, String)> = Vec::new();
    let mut missing: Vec<(String, String)> = Vec::new();

    for entry in entries {
        let spec = &entry.spec;
        if let Some(filter) = &opts.libs
            && !filter.contains(entry.name)
        {
            continue;
        }
        if !is_selected_group(spec.group, opts.include_driver) {
            // Only report driver-group skips when explicitly requested via `--libs`.
            if opts.libs.is_some() {
                skipped.push((
                    entry.name.to_string(),
                    format!("{} library (pass --include-driver to include)", spec.group),
                ));
            }
            continue;
        }
        if platform_names(spec).is_empty() {
            // The library does not exist on this OS (`cufile`/`nccl` on Windows, `nvapi` on
            // Linux): nothing to stage, so it is optional here even when requested.
            if opts.libs.is_some() {
                skipped.push((entry.name.to_string(), "not shipped on this platform".into()));
            }
            continue;
        }

        match find_candidate(entry) {
            CandidateOutcome::Missing(reason) => {
                missing.push((entry.name.to_string(), reason));
            }
            CandidateOutcome::Found(src) => {
                let Some(file_name) = src.file_name() else {
                    missing.push((
                        entry.name.to_string(),
                        format!("candidate {} has no file name", src.display()),
                    ));
                    continue;
                };
                let dst = opts.out.join(file_name);
                if planned.insert(dst.clone()) {
                    found.push((entry.name, src.clone(), dst));
                }
                for companion in companion_files(entry, &src)? {
                    let Some(name) = companion.file_name() else {
                        continue;
                    };
                    let dst = opts.out.join(name);
                    // Libraries share companions (`nvJitLink` for cusolver and cusparse).
                    if planned.insert(dst.clone()) {
                        found.push((entry.name, companion, dst));
                    }
                }
            }
        }
    }

    if opts.libs.is_some() && !missing.is_empty() {
        let names: Vec<&str> = missing.iter().map(|(lib, _)| lib.as_str()).collect();
        let details: String = missing
            .iter()
            .map(|(lib, reason)| format!("\n  missing {lib}: {reason}"))
            .collect();
        return Err(StageError::MissingLibs {
            noun: if names.len() == 1 {
                "library"
            } else {
                "libraries"
            },
            names: names.join(", "),
            out: opts.out.display().to_string(),
            details,
        });
    }

    if !opts.dry_run {
        std::fs::create_dir_all(&opts.out).map_err(|e| StageError::Io {
            lib: "-",
            message: format!("creating {}: {e}", opts.out.display()),
        })?;
    }

    let mut copied = 0usize;
    let mut log = String::new();
    for (lib, src, dst) in &found {
        if is_same_file(src, dst) {
            skipped.push((
                lib.to_string(),
                "up to date (identical size and mtime)".into(),
            ));
            let _ = writeln!(log, "skip  {:<12} {} (up to date)", lib, dst.display());
            continue;
        }
        if opts.dry_run {
            let verb = if dst.exists() { "overwrite" } else { "copy" };
            let _ = writeln!(
                log,
                "[dry-run] would {verb} {:<12} {} -> {}",
                lib,
                src.display(),
                dst.display()
            );
        } else {
            crate::atomic_io::copy(src, dst).map_err(|message| StageError::Io {
                lib,
                message,
            })?;
            let _ = writeln!(
                log,
                "copy  {:<12} {} -> {}",
                lib,
                src.display(),
                dst.display()
            );
        }
        copied += 1;
    }

    let action = if opts.dry_run { "would copy" } else { "copied" };
    let _ = write!(
        log,
        "stage-dlls summary ({}): {action} {copied}, skipped {}, missing {}",
        opts.out.display(),
        skipped.len(),
        missing.len()
    );
    for (lib, reason) in &skipped {
        let _ = write!(log, "\n  skipped {lib}: {reason}");
    }
    for (lib, reason) in &missing {
        let _ = write!(log, "\n  missing {lib}: {reason}");
    }
    Ok(log)
}

fn is_same_file(src: &Path, dst: &Path) -> bool {
    let Ok(src_meta) = std::fs::metadata(src) else {
        return false;
    };
    let Ok(dst_meta) = std::fs::metadata(dst) else {
        return false;
    };
    if src_meta.len() != dst_meta.len() {
        return false;
    }
    matches!(
        (src_meta.modified(), dst_meta.modified()),
        (Ok(s), Ok(d)) if s == d
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_validates_flags_and_required_out() {
        assert!(parse_args(&[]).is_err());
        assert!(parse_args(&["--out"]).is_err());
        assert!(parse_args(&["--out", "dist", "--libs"]).is_err());
        assert!(parse_args(&["--out", "dist", "--libs", " , "]).is_err());
        assert!(parse_args(&["--out", "dist", "--bogus"]).is_err());

        let opts = parse_args(&[
            "--out",
            "dist/bin",
            "--libs",
            "cublas, cufft",
            "--include-driver",
            "--dry-run",
        ])
        .expect("valid args");
        assert_eq!(opts.out, PathBuf::from("dist/bin"));
        assert_eq!(
            opts.libs,
            Some(BTreeSet::from(["cublas".to_string(), "cufft".to_string()]))
        );
        assert!(opts.include_driver);
        assert!(opts.dry_run);
    }

    #[test]
    fn group_filter_excludes_driver_unless_requested() {
        assert!(is_selected_group(Group::Toolkit, false));
        assert!(is_selected_group(
            Group::Sdk {
                root_var: "CUDNN_ROOT",
                hint: None
            },
            false
        ));
        assert!(!is_selected_group(Group::Driver, false));
        assert!(!is_selected_group(Group::DriverStore, false));
        assert!(is_selected_group(Group::Driver, true));
        assert!(is_selected_group(Group::DriverStore, true));
    }

    fn unique_test_dir(label: &str) -> PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "nv_rs_xtask_stage_dlls_{label}_{}_{id}",
            std::process::id()
        ));
        std::fs::remove_dir_all(&dir).ok();
        dir
    }

    /// A file name the catalog's `cublas` entry accepts on this OS.
    fn cublas_file() -> &'static str {
        if cfg!(windows) {
            "cublas64_13.dll"
        } else {
            "libcublas.so.13"
        }
    }

    #[test]
    fn stage_copies_skips_up_to_date_and_supports_dry_run() {
        let root = unique_test_dir("copy");
        let src_dir = root.join("src");
        let out_dir = root.join("out");
        std::fs::create_dir_all(&src_dir).expect("src dir");
        let cublas_dll = src_dir.join(cublas_file());
        std::fs::write(&cublas_dll, b"cublas-bytes").expect("write dll");
        let find = |entry: &CatalogEntry| match entry.name {
            "cublas" => CandidateOutcome::Found(cublas_dll.clone()),
            _ => CandidateOutcome::Missing("not installed".into()),
        };

        // Dry run should not create `out_dir` or copy the file.
        let dry_opts = Opts {
            out: out_dir.clone(),
            libs: Some(BTreeSet::from(["cublas".into()])),
            include_driver: false,
            dry_run: true,
        };
        let dry_msg = stage_with(&dry_opts, catalog::ALL, find).expect("dry run");
        assert!(!out_dir.exists());
        assert!(
            dry_msg.contains("would copy 1, skipped 0, missing 0"),
            "{dry_msg}"
        );

        // Real run copies the file.
        let real_opts = Opts {
            dry_run: false,
            ..dry_opts
        };
        let copy_msg = stage_with(&real_opts, catalog::ALL, find).expect("real run");
        assert!(out_dir.join(cublas_file()).is_file());
        assert!(
            copy_msg.contains("copied 1, skipped 0, missing 0"),
            "{copy_msg}"
        );

        // Second run skips because size and mtime match (the atomic copy keeps the mtime).
        let skip_msg = stage_with(&real_opts, catalog::ALL, find).expect("second run");
        assert!(
            skip_msg.contains("copied 0, skipped 1, missing 0"),
            "{skip_msg}"
        );
        assert!(
            skip_msg.contains("skipped cublas: up to date"),
            "{skip_msg}"
        );

        // Unknown library name fails.
        let bad_opts = Opts {
            libs: Some(BTreeSet::from(["not_a_real_lib".into()])),
            ..real_opts
        };
        let err = stage_with(&bad_opts, catalog::ALL, |_| CandidateOutcome::Missing("x".into()))
            .expect_err("unknown library");
        assert!(matches!(err, StageError::UnknownLibs { .. }), "{err}");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn missing_requested_library_fails_and_copies_nothing() {
        let root = unique_test_dir("missing");
        let src_dir = root.join("src");
        let out_dir = root.join("out");
        std::fs::create_dir_all(&src_dir).expect("src dir");
        let cublas_dll = src_dir.join(cublas_file());
        std::fs::write(&cublas_dll, b"cublas-bytes").expect("write dll");
        let find = |entry: &CatalogEntry| match entry.name {
            "cublas" => CandidateOutcome::Found(cublas_dll.clone()),
            _ => CandidateOutcome::Missing("not installed".into()),
        };

        for dry_run in [true, false] {
            let opts = Opts {
                out: out_dir.clone(),
                libs: Some(BTreeSet::from([
                    "cublas".into(),
                    "cufft".into(),
                    "nvrtc".into(),
                ])),
                include_driver: false,
                dry_run,
            };
            let err = stage_with(&opts, catalog::ALL, find).expect_err("requested libs are missing");
            let text = err.to_string();
            assert!(matches!(err, StageError::MissingLibs { .. }), "{text}");
            assert!(
                text.contains("requested libraries not found: cufft, nvrtc"),
                "{text}"
            );
            assert!(text.contains("missing cufft: not installed"), "{text}");
            assert!(text.contains("missing nvrtc: not installed"), "{text}");
            assert!(!out_dir.exists(), "nothing may be copied: {text}");
        }

        // Without `--libs` a missing library is only a warning.
        let opts = Opts {
            out: out_dir.clone(),
            libs: None,
            include_driver: false,
            dry_run: false,
        };
        let msg = stage_with(&opts, catalog::ALL, find)
            .expect("missing is a warning without --libs");
        assert!(msg.contains("missing cufft: not installed"), "{msg}");
        assert!(out_dir.join(cublas_file()).is_file());

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn dlss_and_dlssd_are_staged_from_the_catalog() {
        let root = unique_test_dir("ngx");
        let src_dir = root.join("src");
        let out_dir = root.join("out");
        std::fs::create_dir_all(&src_dir).expect("src dir");
        // The names the catalog lists for this OS; Linux ones are patterns, so use the versioned
        // files a real SDK ships.
        let (dlss, dlssd) = if cfg!(windows) {
            ("nvngx_dlss.dll", "nvngx_dlssd.dll")
        } else {
            ("libnvidia-ngx-dlss.so.310.4.0", "libnvidia-ngx-dlssd.so.310.4.0")
        };
        for name in [dlss, dlssd] {
            std::fs::write(src_dir.join(name), name.as_bytes()).expect("write runtime");
        }
        let find = |entry: &CatalogEntry| match entry.name {
            "dlss" => CandidateOutcome::Found(src_dir.join(dlss)),
            "dlssd" => CandidateOutcome::Found(src_dir.join(dlssd)),
            _ => CandidateOutcome::Missing("not installed".into()),
        };
        let opts = Opts {
            out: out_dir.clone(),
            libs: Some(BTreeSet::from(["dlss".into(), "dlssd".into()])),
            include_driver: false,
            dry_run: false,
        };
        let msg = stage_with(&opts, catalog::ALL, find).expect("both runtimes stage");
        assert!(msg.contains("copied 2,"), "{msg}");
        assert!(out_dir.join(dlss).is_file() && out_dir.join(dlssd).is_file());
        std::fs::remove_dir_all(&root).ok();
    }

    /// File names of a fake cuDNN install for this OS: the main file, its required siblings and
    /// (Windows) the optional zlib.
    fn cudnn_install() -> (&'static str, Vec<&'static str>) {
        if cfg!(windows) {
            (
                "cudnn64_9.dll",
                vec![
                    "cudnn_graph64_9.dll",
                    "cudnn_ops64_9.dll",
                    "cudnn_engines_precompiled64_9.dll",
                    "zlibwapi.dll",
                ],
            )
        } else {
            (
                "libcudnn.so.9",
                vec!["libcudnn_graph.so.9.1.0", "libcudnn_ops.so.9.1.0"],
            )
        }
    }

    #[test]
    fn companions_ship_with_their_library_and_a_missing_required_one_fails() {
        let root = unique_test_dir("companions");
        let src_dir = root.join("src");
        let out_dir = root.join("out");
        std::fs::create_dir_all(&src_dir).expect("src dir");
        let (main, siblings) = cudnn_install();
        std::fs::write(src_dir.join(main), b"main").expect("write main");
        let find = |entry: &CatalogEntry| match entry.name {
            "cudnn" => CandidateOutcome::Found(src_dir.join(main)),
            _ => CandidateOutcome::Missing("not installed".into()),
        };
        let opts = Opts {
            out: out_dir.clone(),
            libs: Some(BTreeSet::from(["cudnn".into()])),
            include_driver: false,
            dry_run: false,
        };

        // Only the main file exists: the required sub-libraries are missing, so nothing ships.
        let err = stage_with(&opts, catalog::ALL, find).expect_err("required companions missing");
        assert!(
            matches!(&err, StageError::MissingCompanion { lib: "cudnn", .. }),
            "{err}"
        );
        assert!(err.to_string().contains("cudnn"), "{err}");
        assert!(!out_dir.exists(), "nothing may be copied: {err}");

        // With the siblings present, every one is staged next to the main file.
        for name in &siblings {
            std::fs::write(src_dir.join(name), name.as_bytes()).expect("write sibling");
        }
        let msg = stage_with(&opts, catalog::ALL, find).expect("complete install stages");
        assert!(out_dir.join(main).is_file(), "{msg}");
        for name in &siblings {
            assert!(out_dir.join(name).is_file(), "{name} was not staged: {msg}");
        }
        assert!(msg.contains(&format!("copied {},", siblings.len() + 1)), "{msg}");
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn optional_libraries_stay_warnings_when_requested() {
        const ENTRIES: &[CatalogEntry] = &[
            CatalogEntry {
                name: "nowhere",
                spec: DllSpec {
                    lib: "nowhere",
                    group: Group::Toolkit,
                    windows: &[],
                    linux: &[],
                },
                companions_windows: &[],
                companions_linux: &[],
            },
            CatalogEntry {
                name: "drv",
                spec: DllSpec {
                    lib: "drv",
                    group: Group::Driver,
                    windows: &["drv.dll"],
                    linux: &["libdrv.so.1"],
                },
                companions_windows: &[],
                companions_linux: &[],
            },
        ];
        let root = unique_test_dir("optional");
        let opts = Opts {
            out: root.join("out"),
            libs: Some(BTreeSet::from(["nowhere".into(), "drv".into()])),
            include_driver: false,
            dry_run: true,
        };
        let msg = stage_with(&opts, ENTRIES, |entry| {
            panic!("{} must not be searched", entry.name)
        })
        .expect("optional libraries do not fail the command");
        assert!(
            msg.contains("skipped nowhere: not shipped on this platform"),
            "{msg}"
        );
        assert!(msg.contains("skipped drv: driver library"), "{msg}");
        std::fs::remove_dir_all(&root).ok();
    }
}
