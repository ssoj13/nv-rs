//! Stages already-built `*-sys` bindings from `target/` into `crates/*-sys/prebuilt/<target>/`.
//!
//! **What.** Implements `cargo xtask prebuilt [--out <dir>] [--target <triple>]` (alias
//! `stage-prebuilt`). Scans `target/*/build/*/out/` and `target/*/*/build/*/out/` for the newest
//! `*_types.rs`, `*_api.rs`, `*_shims.rs`, and `*_inventory.json` files of each `crates/*-sys`
//! crate and copies them into `<out>/<crate>/prebuilt/<target>/` (defaulting `<out>` to
//! `crates/`).
//!
//! **Why.** Enables `nv-bindgen` prebuilt fallback on machines without NVIDIA SDKs or `libclang`
//! without hand-copying hashed `OUT_DIR` paths.
//!
//! **Used by** `xtask` (`main.rs` / `xtask.rs`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Runs `cargo xtask prebuilt` / `stage-prebuilt`.
pub fn run(root: &Path, args: &[&str]) -> Result<String, String> {
    let opts = parse_args(args)?;
    let target = match opts.target {
        Some(t) => t,
        None => host_target()?,
    };
    let out_root = opts.out.unwrap_or_else(|| root.join("crates"));
    stage_dir(root, &out_root, &target)
}

struct Opts {
    out: Option<PathBuf>,
    target: Option<String>,
}

fn parse_args(args: &[&str]) -> Result<Opts, String> {
    let mut out = None;
    let mut target = None;
    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "--out" => {
                i += 1;
                let val = args
                    .get(i)
                    .ok_or_else(|| "--out requires a directory".to_string())?;
                out = Some(PathBuf::from(val));
            }
            "--target" => {
                i += 1;
                let val = args
                    .get(i)
                    .ok_or_else(|| "--target requires a triple".to_string())?;
                target = Some((*val).to_string());
            }
            other => {
                return Err(format!(
                    "unknown argument `{other}`; usage: cargo xtask prebuilt [--out <dir>] [--target <triple>]"
                ));
            }
        }
        i += 1;
    }
    Ok(Opts { out, target })
}

fn host_target() -> Result<String, String> {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = std::process::Command::new(rustc)
        .arg("-vV")
        .output()
        .map_err(|e| format!("failed to run `rustc -vV`: {e}"))?;
    if !output.status.success() {
        return Err("`rustc -vV` exited with non-zero status".into());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("host: ").map(|s| s.trim().to_string()))
        .ok_or_else(|| "could not parse `host:` from `rustc -vV`".into())
}

fn stage_dir(root: &Path, out_root: &Path, target: &str) -> Result<String, String> {
    let sys_crates = discover_sys_crates(&root.join("crates"));
    if sys_crates.is_empty() {
        return Ok("staged prebuilt bindings: no *-sys crates found".into());
    }

    let mut newest: BTreeMap<String, (SystemTime, PathBuf)> = BTreeMap::new();
    let target_dir = root.join("target");
    for entry in list_dirs(&target_dir) {
        let Some(name) = entry.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if name == target {
            for profile in list_dirs(&entry) {
                scan_build_dir(&profile.join("build"), &sys_crates, &mut newest);
            }
        } else if matches!(name, "debug" | "release") || entry.join("build").is_dir() {
            // Host build under target/<profile>/build/.
            scan_build_dir(&entry.join("build"), &sys_crates, &mut newest);
        }
    }

    let mut staged = Vec::new();
    for (crate_name, (_, src_out)) in &newest {
        let files = collect_binding_files(src_out);
        if files.is_empty() {
            continue;
        }
        let dst_dir = out_root.join(crate_name).join("prebuilt").join(target);
        std::fs::create_dir_all(&dst_dir)
            .map_err(|e| format!("creating {}: {e}", dst_dir.display()))?;
        for src in &files {
            let Some(file_name) = src.file_name() else {
                continue;
            };
            let dst = dst_dir.join(file_name);
            std::fs::copy(src, &dst)
                .map_err(|e| format!("copying {} -> {}: {e}", src.display(), dst.display()))?;
        }
        staged.push(format!("{crate_name} ({} files)", files.len()));
    }

    if staged.is_empty() {
        Ok("staged prebuilt bindings: none (build the *-sys crates first)".into())
    } else {
        Ok(format!(
            "staged prebuilt bindings for {target}: {}",
            staged.join(", ")
        ))
    }
}

fn discover_sys_crates(crates_dir: &Path) -> BTreeSet<String> {
    list_dirs(crates_dir)
        .into_iter()
        .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(str::to_string))
        .filter(|name| name.ends_with("-sys"))
        .collect()
}

fn scan_build_dir(
    build_dir: &Path,
    sys_crates: &BTreeSet<String>,
    newest: &mut BTreeMap<String, (SystemTime, PathBuf)>,
) {
    for build in list_dirs(build_dir) {
        let Some(dir_name) = build.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let Some(crate_name) = match_sys_crate(dir_name, sys_crates) else {
            continue;
        };
        let out_dir = build.join("out");
        let files = collect_binding_files(&out_dir);
        if files.is_empty() {
            continue;
        }
        let modified = files
            .iter()
            .filter_map(|f| std::fs::metadata(f).and_then(|m| m.modified()).ok())
            .max()
            .unwrap_or(SystemTime::UNIX_EPOCH);
        if newest.get(&crate_name).is_none_or(|(t, _)| modified > *t) {
            newest.insert(crate_name, (modified, out_dir));
        }
    }
}

fn match_sys_crate(dir_name: &str, sys_crates: &BTreeSet<String>) -> Option<String> {
    let (pkg, hash) = dir_name.rsplit_once('-')?;
    if hash.is_empty() || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    sys_crates.get(pkg).cloned()
}

fn collect_binding_files(out_dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(out_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && is_binding_artifact(p))
        .collect();
    files.sort();
    files
}

fn is_binding_artifact(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    name.ends_with("_types.rs")
        || name.ends_with("_api.rs")
        || name.ends_with("_shims.rs")
        || name.ends_with("_inventory.json")
}

fn list_dirs(dir: &Path) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    dirs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_accepts_out_and_target() {
        let opts = parse_args(&["--out", "dist", "--target", "x86_64-unknown-linux-gnu"])
            .expect("valid args");
        assert_eq!(opts.out, Some(PathBuf::from("dist")));
        assert_eq!(opts.target.as_deref(), Some("x86_64-unknown-linux-gnu"));
        assert!(parse_args(&["--bogus"]).is_err());
        assert!(parse_args(&["--out"]).is_err());
    }

    #[test]
    fn stage_dir_copies_binding_artifacts_and_skips_unrelated_files() {
        let root = std::env::temp_dir().join("nv_rs_xtask_prebuilt_test");
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(root.join("crates").join("nvml-sys")).expect("crate dir");
        let out_dir = root
            .join("target")
            .join("debug")
            .join("build")
            .join("nvml-sys-0123456789abcdef")
            .join("out");
        std::fs::create_dir_all(&out_dir).expect("out dir");
        for name in [
            "nvml_types.rs",
            "nvml_api.rs",
            "nvml_shims.rs",
            "nvml_inventory.json",
            "unrelated.txt",
        ] {
            std::fs::write(out_dir.join(name), name).expect("write");
        }

        let staged_root = root.join("staged");
        let msg = stage_dir(&root, &staged_root, "x86_64-pc-windows-msvc").expect("stage");
        assert!(msg.contains("nvml-sys (4 files)"), "{msg}");

        let dst = staged_root
            .join("nvml-sys")
            .join("prebuilt")
            .join("x86_64-pc-windows-msvc");
        for name in [
            "nvml_types.rs",
            "nvml_api.rs",
            "nvml_shims.rs",
            "nvml_inventory.json",
        ] {
            assert!(dst.join(name).is_file(), "missing {name}");
        }
        assert!(!dst.join("unrelated.txt").exists());
        std::fs::remove_dir_all(&root).ok();
    }
}
