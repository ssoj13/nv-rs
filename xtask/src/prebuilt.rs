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
    let target_dir = resolve_target_dir(root);
    let host = host_target().ok();
    stage_dir_internal(root, &target_dir, out_root, target, host.as_deref())
}

fn resolve_target_dir(root: &Path) -> PathBuf {
    match std::env::var_os("CARGO_TARGET_DIR") {
        Some(val) if !val.is_empty() => {
            let p = PathBuf::from(val);
            if p.is_relative() { root.join(p) } else { p }
        }
        _ => root.join("target"),
    }
}

fn stage_dir_internal(
    root: &Path,
    target_dir: &Path,
    out_root: &Path,
    target: &str,
    host: Option<&str>,
) -> Result<String, String> {
    let sys_crates = discover_sys_crates(&root.join("crates"));
    if sys_crates.is_empty() {
        return Ok("staged prebuilt bindings: no *-sys crates found".into());
    }

    let is_host = host.is_some_and(|h| h == target);
    let mut newest: BTreeMap<String, (SystemTime, PathBuf)> = BTreeMap::new();

    // 1. Target-qualified builds under <target_dir>/<target>/<profile>/build/
    let explicit_target_dir = target_dir.join(target);
    if explicit_target_dir.is_dir() {
        for profile in list_dirs(&explicit_target_dir) {
            scan_build_dir(&profile.join("build"), &sys_crates, &mut newest);
        }
    }

    // 2. Unqualified host builds under <target_dir>/<profile>/build/
    // Allowed ONLY when requested target equals the detected rustc host.
    if is_host {
        for entry in list_dirs(target_dir) {
            let Some(name) = entry.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if name == target {
                continue;
            }
            if matches!(name, "debug" | "release") || entry.join("build").is_dir() {
                scan_build_dir(&entry.join("build"), &sys_crates, &mut newest);
            }
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

    fn unique_test_dir(prefix: &str) -> PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("{prefix}_{}_{id}", std::process::id()))
    }

    #[test]
    fn stage_dir_copies_binding_artifacts_and_skips_unrelated_files() {
        let root = unique_test_dir("nv_rs_xtask_prebuilt_test");
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
        let host = host_target().unwrap_or_else(|_| "x86_64-pc-windows-msvc".to_string());
        let msg = stage_dir(&root, &staged_root, &host).expect("stage");
        assert!(msg.contains("nvml-sys (4 files)"), "{msg}");

        let dst = staged_root.join("nvml-sys").join("prebuilt").join(&host);
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

    #[test]
    fn foreign_target_ignores_newer_host_artifacts() {
        let root = unique_test_dir("nv_rs_xtask_prebuilt_foreign_test");
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(root.join("crates").join("nvml-sys")).expect("crate dir");

        let host = "x86_64-pc-windows-msvc";
        let foreign = "aarch64-unknown-linux-gnu";

        // 1. Foreign target build under target/aarch64-unknown-linux-gnu/release/build/...
        let foreign_out = root
            .join("target")
            .join(foreign)
            .join("release")
            .join("build")
            .join("nvml-sys-aaaa1111bbbb2222")
            .join("out");
        std::fs::create_dir_all(&foreign_out).expect("foreign out dir");
        std::fs::write(
            foreign_out.join("nvml_types.rs"),
            "// aarch64 foreign content",
        )
        .expect("write");
        std::fs::write(foreign_out.join("nvml_api.rs"), "// aarch64 foreign api").expect("write");
        std::fs::write(
            foreign_out.join("nvml_shims.rs"),
            "// aarch64 foreign shims",
        )
        .expect("write");
        std::fs::write(foreign_out.join("nvml_inventory.json"), "{}").expect("write");

        // Ensure host build files have newer timestamps
        std::thread::sleep(std::time::Duration::from_millis(50));

        // 2. Newer host build under unqualified target/debug/build/...
        let host_out = root
            .join("target")
            .join("debug")
            .join("build")
            .join("nvml-sys-cccc3333dddd4444")
            .join("out");
        std::fs::create_dir_all(&host_out).expect("host out dir");
        std::fs::write(host_out.join("nvml_types.rs"), "// host content newer").expect("write");
        std::fs::write(host_out.join("nvml_api.rs"), "// host api newer").expect("write");
        std::fs::write(host_out.join("nvml_shims.rs"), "// host shims newer").expect("write");
        std::fs::write(host_out.join("nvml_inventory.json"), "{}").expect("write");

        let staged_root = root.join("staged");
        let msg = stage_dir_internal(
            &root,
            &root.join("target"),
            &staged_root,
            foreign,
            Some(host),
        )
        .expect("stage foreign");
        assert!(msg.contains("nvml-sys (4 files)"), "{msg}");

        let dst = staged_root.join("nvml-sys").join("prebuilt").join(foreign);
        let content = std::fs::read_to_string(dst.join("nvml_types.rs")).expect("read staged");
        assert_eq!(
            content, "// aarch64 foreign content",
            "must use foreign target artifacts, not host!"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn foreign_target_rejects_when_only_host_build_exists() {
        let root = unique_test_dir("nv_rs_xtask_prebuilt_missing_foreign_test");
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(root.join("crates").join("nvml-sys")).expect("crate dir");

        let host = "x86_64-pc-windows-msvc";
        let foreign = "riscv64gc-unknown-linux-gnu";

        let host_out = root
            .join("target")
            .join("debug")
            .join("build")
            .join("nvml-sys-1234567890abcdef")
            .join("out");
        std::fs::create_dir_all(&host_out).expect("host out dir");
        std::fs::write(host_out.join("nvml_types.rs"), "// host content").expect("write");

        let staged_root = root.join("staged");
        let msg = stage_dir_internal(
            &root,
            &root.join("target"),
            &staged_root,
            foreign,
            Some(host),
        )
        .expect("stage foreign");
        assert!(
            msg.contains("none (build the *-sys crates first)"),
            "expected 'none' when foreign target has no build, got: {msg}"
        );
        assert!(
            !staged_root
                .join("nvml-sys")
                .join("prebuilt")
                .join(foreign)
                .exists()
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn custom_target_dir_staged() {
        let root = unique_test_dir("nv_rs_xtask_prebuilt_custom_target_dir_test");
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(root.join("crates").join("nvml-sys")).expect("crate dir");

        let custom_target = root.join("custom_target_dir");
        let out_dir = custom_target
            .join("debug")
            .join("build")
            .join("nvml-sys-0123456789abcdef")
            .join("out");
        std::fs::create_dir_all(&out_dir).expect("out dir");
        std::fs::write(
            out_dir.join("nvml_types.rs"),
            "// custom target dir content",
        )
        .expect("write");
        std::fs::write(out_dir.join("nvml_api.rs"), "// api").expect("write");
        std::fs::write(out_dir.join("nvml_shims.rs"), "// shims").expect("write");
        std::fs::write(out_dir.join("nvml_inventory.json"), "{}").expect("write");

        let staged_root = root.join("staged");
        let host = "x86_64-pc-windows-msvc";
        let msg = stage_dir_internal(&root, &custom_target, &staged_root, host, Some(host))
            .expect("stage custom target dir");
        assert!(msg.contains("nvml-sys (4 files)"), "{msg}");

        let dst = staged_root.join("nvml-sys").join("prebuilt").join(host);
        let content = std::fs::read_to_string(dst.join("nvml_types.rs")).expect("read staged");
        assert_eq!(content, "// custom target dir content");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn resolve_target_dir_defaults_to_target() {
        let root = Path::new("/workspace");
        if std::env::var_os("CARGO_TARGET_DIR").is_none() {
            assert_eq!(resolve_target_dir(root), root.join("target"));
        }
    }
}
