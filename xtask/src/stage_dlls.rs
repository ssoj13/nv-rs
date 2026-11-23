//! Stages NVIDIA runtime DLLs into a target directory for bundling or distribution.
//!
//! **What.** Implements `cargo xtask stage-dlls --out <dir> [--libs <csv>] [--include-driver]
//! [--dry-run]`. Uses [`nv_loader::candidates`] to locate the first existing runtime library on
//! disk for each selected `*-sys` [`DllSpec`], and copies it into `--out` atomically
//! ([`crate::atomic_io::copy`]), skipping files that already match in size and modification time.
//! A library named in `--libs` that is not found fails the command (non-zero exit, nothing
//! copied); without `--libs`, missing libraries are reported as warnings. Libraries that are
//! optional by definition stay warnings either way: driver-installed ones without
//! `--include-driver`, and ones with no file name on the current platform.
//!
//! **Why.** Applications and Python wheels built against `nv-rs` need redistributable CUDA
//! Toolkit and SDK DLLs next to their binaries or in a staging directory without hand-hunting
//! versioned filenames (`cublas64_13.dll`, `cufft64_12.dll`, ...).
//!
//! **How.** Holds the workspace's [`DllSpec`] catalog mirroring `crates/*-sys/src/*.rs`. Filters
//! to redistributable [`Group::Toolkit`] and [`Group::Sdk`] libraries by default, or includes
//! [`Group::Driver`] and [`Group::DriverStore`] when `--include-driver` is passed, with optional
//! `--libs` CSV filtering.
//!
//! **Used by** `xtask` (`xtask.rs`).

use nv_loader::{DllSpec, Group};
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const USAGE: &str = "\
usage: cargo xtask stage-dlls|d --out <dir> [--libs <csv>] [--include-driver] [--dry-run]
  --libs <csv>      stage only these libraries; one that is not found fails the command
  --include-driver  also stage driver-installed libraries (nvml, nvapi, optix, ...)
  --dry-run         print what would be copied without copying
example: cargo xtask stage-dlls --out dist --libs cufft,nvjpeg,nvrtc";

macro_rules! npp_dll {
    ($name:literal, $linux:literal) => {
        DllSpec {
            lib: $name,
            group: Group::Toolkit,
            windows: &[concat!($name, "64_*")],
            linux: &[$linux, concat!("lib", $name, ".so")],
        }
    };
}

/// Workspace runtime library specifications, matching `crates/*-sys/src/*.rs`.
const SPECS: &[DllSpec] = &[
    DllSpec {
        lib: "cublas",
        group: Group::Toolkit,
        windows: &["cublas64_*"],
        linux: &["libcublas.so.13", "libcublas.so"],
    },
    DllSpec {
        lib: "cublaslt",
        group: Group::Toolkit,
        windows: &["cublasLt64_*"],
        linux: &["libcublasLt.so.13", "libcublasLt.so"],
    },
    DllSpec {
        lib: "cudnn",
        group: Group::Sdk {
            root_var: "CUDNN_ROOT",
            hint: None,
        },
        windows: &["cudnn64_*"],
        linux: &["libcudnn.so.9", "libcudnn.so"],
    },
    DllSpec {
        lib: "cufft",
        group: Group::Toolkit,
        windows: &["cufft64_*"],
        linux: &["libcufft.so.12", "libcufft.so"],
    },
    DllSpec {
        lib: "cufile",
        group: Group::Toolkit,
        windows: &[],
        linux: &["libcufile.so.0", "libcufile.so"],
    },
    DllSpec {
        lib: "cupti",
        group: Group::Toolkit,
        windows: &["cupti64_*"],
        linux: &["libcupti.so.13", "libcupti.so.12", "libcupti.so"],
    },
    DllSpec {
        lib: "cusolver",
        group: Group::Toolkit,
        windows: &["cusolver64_*"],
        linux: &["libcusolver.so.12", "libcusolver.so"],
    },
    DllSpec {
        lib: "cusolvermg",
        group: Group::Toolkit,
        windows: &["cusolverMg64_*"],
        linux: &["libcusolverMg.so.12", "libcusolverMg.so"],
    },
    DllSpec {
        lib: "cusparse",
        group: Group::Toolkit,
        windows: &["cusparse64_*"],
        linux: &["libcusparse.so.12", "libcusparse.so"],
    },
    DllSpec {
        lib: "nccl",
        group: Group::Sdk {
            root_var: "NCCL_ROOT",
            hint: None,
        },
        windows: &[],
        linux: &["libnccl.so.2", "libnccl.so"],
    },
    npp_dll!("nppc", "libnppc.so.13"),
    npp_dll!("npps", "libnpps.so.13"),
    npp_dll!("nppisu", "libnppisu.so.13"),
    npp_dll!("nppidei", "libnppidei.so.13"),
    npp_dll!("nppial", "libnppial.so.13"),
    npp_dll!("nppicc", "libnppicc.so.13"),
    npp_dll!("nppitc", "libnppitc.so.13"),
    npp_dll!("nppim", "libnppim.so.13"),
    npp_dll!("nppif", "libnppif.so.13"),
    npp_dll!("nppist", "libnppist.so.13"),
    npp_dll!("nppig", "libnppig.so.13"),
    DllSpec {
        lib: "nvapi",
        group: Group::Driver,
        windows: &["nvapi64.dll"],
        linux: &[],
    },
    DllSpec {
        lib: "nvcomp",
        group: Group::Sdk {
            root_var: "NVCOMP_ROOT",
            hint: None,
        },
        windows: &["nvcomp64_*"],
        linux: &["libnvcomp.so.6", "libnvcomp.so"],
    },
    DllSpec {
        lib: "nvcuvid",
        group: Group::Driver,
        windows: &["nvcuvid.dll"],
        linux: &["libnvcuvid.so.1"],
    },
    DllSpec {
        lib: "nvenc",
        group: Group::Driver,
        windows: &["nvEncodeAPI64.dll"],
        linux: &["libnvidia-encode.so.1"],
    },
    DllSpec {
        lib: "dlss",
        group: Group::Sdk {
            root_var: "NGX_SDK_ROOT",
            hint: None,
        },
        windows: &["nvngx_dlss.dll"],
        linux: &["libnvidia-ngx-dlss.so*"],
    },
    DllSpec {
        lib: "nvjpeg",
        group: Group::Toolkit,
        windows: &["nvjpeg64_*"],
        linux: &["libnvjpeg.so.13", "libnvjpeg.so.12", "libnvjpeg.so"],
    },
    DllSpec {
        lib: "nvml",
        group: Group::Driver,
        windows: &["nvml.dll"],
        linux: &["libnvidia-ml.so.1"],
    },
    DllSpec {
        lib: "nvofa",
        group: Group::Driver,
        windows: &["nvofapi64.dll"],
        linux: &["libnvidia-opticalflow.so.1"],
    },
    DllSpec {
        lib: "nvrtc",
        group: Group::Toolkit,
        windows: &["nvrtc64_*"],
        linux: &["libnvrtc.so.13", "libnvrtc.so"],
    },
    DllSpec {
        lib: "optix",
        group: Group::DriverStore,
        windows: &["nvoptix.dll"],
        linux: &["libnvoptix.so.1"],
    },
];

/// Runs `cargo xtask stage-dlls`.
pub fn run(args: &[&str]) -> Result<String, String> {
    if args.iter().any(|a| matches!(*a, "-h" | "--help")) {
        return Ok(USAGE.to_string());
    }
    let opts = parse_args(args)?;
    stage_with(&opts, SPECS, find_first_existing)
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

fn find_first_existing(spec: &DllSpec) -> CandidateOutcome {
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

fn stage_with(
    opts: &Opts,
    specs: &[DllSpec],
    mut find_candidate: impl FnMut(&DllSpec) -> CandidateOutcome,
) -> Result<String, String> {
    if let Some(filter) = &opts.libs {
        let known: BTreeSet<&str> = specs.iter().map(|s| s.lib).collect();
        let unknown: Vec<&str> = filter
            .iter()
            .map(String::as_str)
            .filter(|name| !known.contains(name))
            .collect();
        if !unknown.is_empty() {
            let mut avail: Vec<&str> = known.into_iter().collect();
            avail.sort_unstable();
            return Err(format!(
                "unknown library name(s): {} (available: {})",
                unknown.join(", "),
                avail.join(", ")
            ));
        }
    }

    // Resolve every selected library before copying anything, so a missing requested library
    // fails the command without leaving a partial bundle behind.
    let mut found: Vec<(&str, PathBuf, PathBuf)> = Vec::new();
    let mut skipped: Vec<(String, String)> = Vec::new();
    let mut missing: Vec<(String, String)> = Vec::new();

    for spec in specs {
        if let Some(filter) = &opts.libs
            && !filter.contains(spec.lib)
        {
            continue;
        }
        if !is_selected_group(spec.group, opts.include_driver) {
            // Only report driver-group skips when explicitly requested via `--libs`.
            if opts.libs.is_some() {
                skipped.push((
                    spec.lib.to_string(),
                    format!("{} library (pass --include-driver to include)", spec.group),
                ));
            }
            continue;
        }
        if platform_names(spec).is_empty() {
            // The library does not exist on this OS (`cufile`/`nccl` on Windows, `nvapi` on
            // Linux): nothing to stage, so it is optional here even when requested.
            if opts.libs.is_some() {
                skipped.push((spec.lib.to_string(), "not shipped on this platform".into()));
            }
            continue;
        }

        match find_candidate(spec) {
            CandidateOutcome::Missing(reason) => {
                missing.push((spec.lib.to_string(), reason));
            }
            CandidateOutcome::Found(src) => {
                let dst = src.file_name().map(|name| opts.out.join(name));
                match dst {
                    Some(dst) => found.push((spec.lib, src, dst)),
                    None => missing.push((
                        spec.lib.to_string(),
                        format!("candidate {} has no file name", src.display()),
                    )),
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
        return Err(format!(
            "stage-dlls: requested {} not found: {}; nothing was copied to {}{details}",
            if names.len() == 1 { "library" } else { "libraries" },
            names.join(", "),
            opts.out.display()
        ));
    }

    if !opts.dry_run {
        std::fs::create_dir_all(&opts.out)
            .map_err(|e| format!("creating {}: {e}", opts.out.display()))?;
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
            crate::atomic_io::copy(src, dst)?;
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

    #[test]
    fn stage_copies_skips_up_to_date_and_supports_dry_run() {
        let root = unique_test_dir("copy");
        let src_dir = root.join("src");
        let out_dir = root.join("out");
        std::fs::create_dir_all(&src_dir).expect("src dir");
        let cublas_dll = src_dir.join("cublas64_13.dll");
        std::fs::write(&cublas_dll, b"cublas-bytes").expect("write dll");
        let find = |spec: &DllSpec| match spec.lib {
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
        let dry_msg = stage_with(&dry_opts, SPECS, find).expect("dry run");
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
        let copy_msg = stage_with(&real_opts, SPECS, find).expect("real run");
        assert!(out_dir.join("cublas64_13.dll").is_file());
        assert!(
            copy_msg.contains("copied 1, skipped 0, missing 0"),
            "{copy_msg}"
        );

        // Second run skips because size and mtime match (the atomic copy keeps the mtime).
        let skip_msg = stage_with(&real_opts, SPECS, find).expect("second run");
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
        assert!(stage_with(&bad_opts, SPECS, |_| CandidateOutcome::Missing("x".into())).is_err());

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn missing_requested_library_fails_and_copies_nothing() {
        let root = unique_test_dir("missing");
        let src_dir = root.join("src");
        let out_dir = root.join("out");
        std::fs::create_dir_all(&src_dir).expect("src dir");
        let cublas_dll = src_dir.join("cublas64_13.dll");
        std::fs::write(&cublas_dll, b"cublas-bytes").expect("write dll");
        let find = |spec: &DllSpec| match spec.lib {
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
            let err = stage_with(&opts, SPECS, find).expect_err("requested libs are missing");
            assert!(
                err.contains("requested libraries not found: cufft, nvrtc"),
                "{err}"
            );
            assert!(err.contains("missing cufft: not installed"), "{err}");
            assert!(err.contains("missing nvrtc: not installed"), "{err}");
            assert!(!out_dir.exists(), "nothing may be copied: {err}");
        }

        // Without `--libs` a missing library is only a warning.
        let opts = Opts {
            out: out_dir.clone(),
            libs: None,
            include_driver: false,
            dry_run: false,
        };
        let msg = stage_with(&opts, SPECS, find).expect("missing is a warning without --libs");
        assert!(msg.contains("missing cufft: not installed"), "{msg}");
        assert!(out_dir.join("cublas64_13.dll").is_file());

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn optional_libraries_stay_warnings_when_requested() {
        const SPECS_WITH_OPTIONAL: &[DllSpec] = &[
            DllSpec {
                lib: "nowhere",
                group: Group::Toolkit,
                windows: &[],
                linux: &[],
            },
            DllSpec {
                lib: "drv",
                group: Group::Driver,
                windows: &["drv.dll"],
                linux: &["libdrv.so.1"],
            },
        ];
        let root = unique_test_dir("optional");
        let opts = Opts {
            out: root.join("out"),
            libs: Some(BTreeSet::from(["nowhere".into(), "drv".into()])),
            include_driver: false,
            dry_run: true,
        };
        let msg = stage_with(&opts, SPECS_WITH_OPTIONAL, |spec| {
            panic!("{} must not be searched", spec.lib)
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
