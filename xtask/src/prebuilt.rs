//! Stages already-built `*-sys` bindings from `target/` into `crates/*-sys/prebuilt/<target>/`.
//!
//! **What.** Implements `cargo xtask prebuilt [--out <dir>] [--target <triple>]` (aliases
//! `stage-prebuilt`, `p`). Scans `target/<target>/<profile>/build/*/out/` (and, for the host
//! target, `target/<profile>/build/*/out/`) for the `*_types.rs`, `*_api.rs`, `*_shims.rs` and
//! `*_inventory.json` files of each `crates/*-sys` crate and stages one complete set per crate
//! into `<out>/<crate>/prebuilt/<target>/` (defaulting `<out>` to `crates/`).
//!
//! **Why.** Enables `nv-bindgen` prebuilt fallback on machines without NVIDIA SDKs or `libclang`
//! without hand-copying hashed `OUT_DIR` paths. A committed set mixing files of two generations
//! binds mismatched layouts, so staging is chosen by completeness, never by mtime alone.
//!
//! **How.** `nv_bindgen::generate` writes `<lib>_manifest.json` ([`nv_bindgen::Manifest`]) into
//! `OUT_DIR` after every other file: the library name, the exact files its shape promises and
//! whether they were generated or copied from the committed prebuilt set. Nothing is inferred
//! here. An out dir is a complete generated set for library `<lib>` (the crate name without
//! `-sys`) when that manifest exists, names `<lib>`, lists the inventory, every listed file
//! exists and the inventory parses and names `<lib>`; the manifest's time is the generation time.
//! File times are not compared (a `build.rs` may post-process a generated file). Only the
//! manifest's files are staged, so a stale artifact an older recipe left in `OUT_DIR` is never
//! shipped. Builds whose manifest says `prebuilt` hold copies, not a generation, and are not
//! candidates. Incomplete candidates (no manifest included) are rejected with the reason; among
//! complete ones the most recently generated wins.
//! When a built crate has no complete candidate the command fails and stages nothing. Each set is
//! copied all-or-nothing (every file to a synced temp file first, then renamed, see
//! [`crate::atomic_io`]), and binding artifacts in the destination that the new set does not
//! contain are removed.
//!
//! **Used by** `xtask` (`xtask.rs`); [`resolve_target_dir`] and [`build_dirs`] are shared with
//! `coverage collect`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const USAGE: &str = "\
usage: cargo xtask prebuilt|p [--out <dir>] [--target <triple>]   (alias: stage-prebuilt)
  --out <dir>        destination root holding the *-sys crates (default: crates/)
  --target <triple>  target triple to stage (default: the rustc host)
example: cargo xtask prebuilt --target x86_64-pc-windows-msvc";

/// Runs `cargo xtask prebuilt` / `stage-prebuilt`.
pub fn run(root: &Path, args: &[&str]) -> Result<String, String> {
    if args.iter().any(|a| matches!(*a, "-h" | "--help")) {
        return Ok(USAGE.to_string());
    }
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
                    .ok_or_else(|| format!("--out requires a directory\n{USAGE}"))?;
                out = Some(PathBuf::from(val));
            }
            "--target" => {
                i += 1;
                let val = args
                    .get(i)
                    .ok_or_else(|| format!("--target requires a triple\n{USAGE}"))?;
                target = Some((*val).to_string());
            }
            other => {
                return Err(format!("unknown argument `{other}`\n{USAGE}"));
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
    let host = host_target()?;
    stage_dir_internal(root, &target_dir, out_root, target, Some(&host))
}

/// `CARGO_TARGET_DIR` (relative to `root` when relative) or `<root>/target`.
pub(crate) fn resolve_target_dir(root: &Path) -> PathBuf {
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
    let mut candidates: BTreeMap<String, Vec<PathBuf>> = BTreeMap::new();

    // 1. Target-qualified builds under <target_dir>/<target>/<profile>/build/
    let explicit_target_dir = target_dir.join(target);
    if explicit_target_dir.is_dir() {
        for profile in list_dirs(&explicit_target_dir) {
            scan_build_dir(&profile.join("build"), &sys_crates, &mut candidates)?;
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
                scan_build_dir(&entry.join("build"), &sys_crates, &mut candidates)?;
            }
        }
    }

    // Choose one complete set per crate before writing anything.
    let mut chosen: Vec<(&str, CompleteSet)> = Vec::new();
    let mut rejected: Vec<String> = Vec::new();
    let mut unusable: Vec<String> = Vec::new();
    for (crate_name, outs) in &candidates {
        let lib = crate_lib_name(crate_name);
        let mut best: Option<CompleteSet> = None;
        let mut reasons: Vec<String> = Vec::new();
        for out in outs {
            match complete_set(out, lib)? {
                Ok(set) => {
                    if best.as_ref().is_none_or(|b| set.generated > b.generated) {
                        best = Some(set);
                    }
                }
                Err(why) => reasons.push(format!("{crate_name}: {}: {why}", out.display())),
            }
        }
        match best {
            Some(set) => {
                rejected.extend(reasons);
                chosen.push((crate_name.as_str(), set));
            }
            None => unusable.extend(reasons),
        }
    }
    if !unusable.is_empty() {
        return Err(format!(
            "prebuilt: no complete generated binding set; nothing was staged (rebuild these crates):\n  {}",
            unusable.join("\n  ")
        ));
    }

    let mut staged = Vec::new();
    for (crate_name, set) in &chosen {
        let dst_dir = out_root.join(crate_name).join("prebuilt").join(target);
        let removed = stage_set(&set.files, &dst_dir)?;
        if removed.is_empty() {
            staged.push(format!("{crate_name} ({} files)", set.files.len()));
        } else {
            staged.push(format!(
                "{crate_name} ({} files, removed stale {})",
                set.files.len(),
                removed.join(", ")
            ));
        }
    }

    let mut msg = if staged.is_empty() {
        "staged prebuilt bindings: none (build the *-sys crates first)".to_string()
    } else {
        format!(
            "staged prebuilt bindings for {target}: {}",
            staged.join(", ")
        )
    };
    for r in &rejected {
        msg = format!("{msg}\n  rejected older/incomplete {r}");
    }
    Ok(msg)
}

/// The `nv-bindgen` library name of a sys crate: every `crates/<lib>-sys` names its `LibSpec`
/// `<lib>` (`nvcuvid-sys` -> `nvcuvid`), which prefixes every generated file.
fn crate_lib_name(crate_name: &str) -> &str {
    crate_name.strip_suffix("-sys").unwrap_or(crate_name)
}

/// One complete generated binding set.
struct CompleteSet {
    files: Vec<PathBuf>,
    /// Modification time of the inventory, the last file `nv_bindgen::generate` writes before the
    /// manifest.
    generated: SystemTime,
}

/// What an out dir's `<lib>_manifest.json` says.
enum ManifestState {
    /// No manifest: the generation never finished, or nv-bindgen predates the manifest.
    Absent,
    /// The file exists but is not a manifest.
    Invalid(String),
    Valid(nv_bindgen::Manifest),
}

/// Reads `<lib>_manifest.json` from `out_dir`. The `Err` is an I/O failure other than "not found".
fn load_manifest(out_dir: &Path, lib: &str) -> Result<ManifestState, String> {
    let path = out_dir.join(nv_bindgen::Manifest::file_name(lib));
    match std::fs::read_to_string(&path) {
        Ok(text) => Ok(match serde_json::from_str::<nv_bindgen::Manifest>(&text) {
            Ok(manifest) => ManifestState::Valid(manifest),
            Err(e) => ManifestState::Invalid(format!("{} does not parse: {e}", path.display())),
        }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(ManifestState::Absent),
        Err(e) => Err(format!("reading {}: {e}", path.display())),
    }
}

/// Checks whether `out_dir` holds a complete generated set for `lib`. The outer `Err` is an I/O
/// failure (aborts the command); the inner `Err` explains why this candidate is rejected.
fn complete_set(out_dir: &Path, lib: &str) -> Result<Result<CompleteSet, String>, String> {
    let manifest_name = nv_bindgen::Manifest::file_name(lib);
    let manifest = match load_manifest(out_dir, lib)? {
        ManifestState::Valid(manifest) => manifest,
        ManifestState::Absent => {
            return Ok(Err(format!(
                "no {manifest_name} (nv-bindgen writes it last: the generation was interrupted or predates the manifest)"
            )));
        }
        ManifestState::Invalid(why) => return Ok(Err(why)),
    };
    if manifest.lib != lib {
        return Ok(Err(format!(
            "{manifest_name} names library `{}`, expected `{lib}`",
            manifest.lib
        )));
    }
    if manifest.origin != nv_bindgen::Origin::Generated {
        return Ok(Err(format!(
            "{manifest_name} says the files were copied from the prebuilt set, not generated"
        )));
    }
    let inventory = format!("{lib}_inventory.json");
    if !manifest.files.contains(&inventory) {
        return Ok(Err(format!("{manifest_name} does not list {inventory}")));
    }
    let mut files: Vec<PathBuf> = Vec::with_capacity(manifest.files.len());
    let mut missing: Vec<&str> = Vec::new();
    for name in &manifest.files {
        let path = out_dir.join(name);
        if path.is_file() {
            files.push(path);
        } else {
            missing.push(name);
        }
    }
    if !missing.is_empty() {
        return Ok(Err(format!(
            "incomplete: {manifest_name} lists missing file(s) {}",
            missing.join(", ")
        )));
    }

    let inv_path = out_dir.join(&inventory);
    let text = std::fs::read_to_string(&inv_path)
        .map_err(|e| format!("reading {}: {e}", inv_path.display()))?;
    match serde_json::from_str::<crate::Inventory>(&text) {
        Ok(inv) if inv.lib == lib => {}
        Ok(inv) => {
            return Ok(Err(format!(
                "{inventory} names library `{}`, expected `{lib}`",
                inv.lib
            )));
        }
        Err(e) => return Ok(Err(format!("{inventory} does not parse: {e}"))),
    }

    // The manifest is deleted when a generation starts and written after every other file, so
    // its time is the generation time. A `build.rs` may legitimately post-process a generated file
    // after `generate` returns (`cupti-sys` appends `CUpti_ActivityKernelLatest`), so file times
    // relative to each other prove nothing.
    let generated = modified(&out_dir.join(&manifest_name))?;
    Ok(Ok(CompleteSet { files, generated }))
}

fn modified(path: &Path) -> Result<SystemTime, String> {
    std::fs::metadata(path)
        .and_then(|m| m.modified())
        .map_err(|e| format!("reading modification time of {}: {e}", path.display()))
}

/// Copies `files` into `dst_dir` all-or-nothing and removes binding artifacts there that `files`
/// does not contain. Returns the names of the removed stale files.
fn stage_set(files: &[PathBuf], dst_dir: &Path) -> Result<Vec<String>, String> {
    std::fs::create_dir_all(dst_dir).map_err(|e| format!("creating {}: {e}", dst_dir.display()))?;

    let mut targets: Vec<(&Path, PathBuf)> = Vec::with_capacity(files.len());
    for src in files {
        let name = src
            .file_name()
            .ok_or_else(|| format!("{} has no file name", src.display()))?;
        let dst = dst_dir.join(name);
        if dst.exists() && !dst.is_file() {
            return Err(format!(
                "{} exists and is not a file; nothing was staged into {}",
                dst.display(),
                dst_dir.display()
            ));
        }
        targets.push((src.as_path(), dst));
    }

    // Phase 1: every file to a synced temp file next to its destination. A failure discards the
    // temps, so the destination keeps its previous complete set.
    let mut temps: Vec<PathBuf> = Vec::with_capacity(targets.len());
    for (src, dst) in &targets {
        match crate::atomic_io::copy_to_temp(src, dst) {
            Ok(tmp) => temps.push(tmp),
            Err(e) => return Err(crate::atomic_io::discard_all(&temps, e)),
        }
    }
    // Phase 2: rename into place.
    for (i, ((_, dst), tmp)) in targets.iter().zip(&temps).enumerate() {
        if let Err(e) = crate::atomic_io::commit(tmp, dst) {
            return Err(crate::atomic_io::discard_all(&temps[i + 1..], e));
        }
    }

    // Phase 3: remove artifacts of an older recipe that the new set no longer produces.
    let keep: BTreeSet<&std::ffi::OsStr> =
        targets.iter().filter_map(|(_, dst)| dst.file_name()).collect();
    let mut removed = Vec::new();
    for old in collect_binding_files(dst_dir)? {
        let Some(name) = old.file_name() else {
            continue;
        };
        if keep.contains(name) {
            continue;
        }
        std::fs::remove_file(&old)
            .map_err(|e| format!("removing stale {}: {e}", old.display()))?;
        removed.push(name.to_string_lossy().into_owned());
    }
    Ok(removed)
}

fn discover_sys_crates(crates_dir: &Path) -> BTreeSet<String> {
    list_dirs(crates_dir)
        .into_iter()
        .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(str::to_string))
        .filter(|name| name.ends_with("-sys"))
        .collect()
}

/// Adds every out dir under `build_dir` that holds binding artifacts of a known sys crate to
/// `candidates` (crate name -> out dirs).
fn scan_build_dir(
    build_dir: &Path,
    sys_crates: &BTreeSet<String>,
    candidates: &mut BTreeMap<String, Vec<PathBuf>>,
) -> Result<(), String> {
    for build in list_dirs(build_dir) {
        let Some(dir_name) = build.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let Some(crate_name) = match_sys_crate(dir_name, sys_crates) else {
            continue;
        };
        let out_dir = build.join("out");
        if collect_binding_files(&out_dir)?.is_empty() {
            continue;
        }
        // A build that fell back to the committed prebuilt bindings holds copies (with copy-time
        // mtimes), not a generation, so there is nothing to stage from it.
        if let ManifestState::Valid(manifest) = load_manifest(&out_dir, crate_lib_name(&crate_name))?
            && manifest.origin == nv_bindgen::Origin::Prebuilt
        {
            continue;
        }
        candidates.entry(crate_name).or_default().push(out_dir);
    }
    Ok(())
}

fn match_sys_crate(dir_name: &str, sys_crates: &BTreeSet<String>) -> Option<String> {
    let (pkg, hash) = dir_name.rsplit_once('-')?;
    if hash.is_empty() || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    sys_crates.get(pkg).cloned()
}

/// Binding artifacts directly in `out_dir`, sorted; a directory that does not exist has none.
fn collect_binding_files(out_dir: &Path) -> Result<Vec<PathBuf>, String> {
    let entries = match std::fs::read_dir(out_dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(format!("listing {}: {e}", out_dir.display())),
    };
    let mut files = Vec::new();
    for entry in entries {
        let path = entry
            .map_err(|e| format!("listing {}: {e}", out_dir.display()))?
            .path();
        if path.is_file() && is_binding_artifact(&path) {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

/// Every `<target_dir>/<profile>/build` and `<target_dir>/<triple>/<profile>/build` directory.
pub(crate) fn build_dirs(target_dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for dir in list_dirs(target_dir) {
        let build = dir.join("build");
        if build.is_dir() {
            out.push(build);
            continue;
        }
        for profile in list_dirs(&dir) {
            let build = profile.join("build");
            if build.is_dir() {
                out.push(build);
            }
        }
    }
    out
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

    fn inventory_json(lib: &str) -> String {
        format!(r#"{{"lib":"{lib}","headers":[],"symbols":[]}}"#)
    }

    const NVML_FILES: [&str; 4] = [
        "nvml_types.rs",
        "nvml_inventory.json",
        "nvml_api.rs",
        "nvml_shims.rs",
    ];

    /// Writes `<lib>_manifest.json` the way `nv_bindgen::generate` does (after the other files).
    fn write_manifest(out_dir: &Path, lib: &str, origin: nv_bindgen::Origin, files: &[&str]) {
        let manifest = nv_bindgen::Manifest {
            lib: lib.to_string(),
            origin,
            files: files.iter().map(|f| (*f).to_string()).collect(),
        };
        std::fs::write(
            out_dir.join(nv_bindgen::Manifest::file_name(lib)),
            serde_json::to_string(&manifest).expect("serialize manifest"),
        )
        .expect("write manifest");
    }

    /// Writes a complete `Exports`-shape set: inventory, then the manifest, as
    /// `nv_bindgen::generate` does.
    fn write_complete_set(out_dir: &Path, tag: &str) {
        std::fs::create_dir_all(out_dir).expect("out dir");
        for name in ["nvml_types.rs", "nvml_api.rs", "nvml_shims.rs"] {
            std::fs::write(out_dir.join(name), format!("// {tag} {name}")).expect("write");
        }
        std::fs::write(out_dir.join("nvml_inventory.json"), inventory_json("nvml")).expect("write");
        write_manifest(out_dir, "nvml", nv_bindgen::Origin::Generated, &NVML_FILES);
    }

    fn set_mtime(path: &Path, secs: u64) {
        let t = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(secs);
        std::fs::File::options()
            .write(true)
            .open(path)
            .and_then(|f| f.set_modified(t))
            .expect("set mtime");
    }

    fn build_out(root: &Path, profile: &str, hash: &str) -> PathBuf {
        root.join("target")
            .join(profile)
            .join("build")
            .join(format!("nvml-sys-{hash}"))
            .join("out")
    }

    const HOST: &str = "x86_64-pc-windows-msvc";

    #[test]
    fn newer_incomplete_out_dir_is_rejected_for_an_older_complete_one() {
        let root = unique_test_dir("nv_rs_xtask_prebuilt_complete");
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(root.join("crates").join("nvml-sys")).expect("crate dir");
        let complete = build_out(&root, "debug", "aaaa0000aaaa0000");
        write_complete_set(&complete, "complete");
        set_mtime(&complete.join("nvml_manifest.json"), 1_700_000_000);
        // A newer, interrupted build that wrote only the types file.
        let partial = build_out(&root, "release", "bbbb1111bbbb1111");
        std::fs::create_dir_all(&partial).expect("partial dir");
        std::fs::write(partial.join("nvml_types.rs"), "// partial").expect("write");

        let staged_root = root.join("staged");
        let msg = stage_dir_internal(&root, &root.join("target"), &staged_root, HOST, Some(HOST))
            .expect("stage");
        assert!(msg.contains("nvml-sys (4 files)"), "{msg}");
        assert!(msg.contains("no nvml_manifest.json"), "{msg}");
        let dst = staged_root.join("nvml-sys").join("prebuilt").join(HOST);
        assert_eq!(
            std::fs::read_to_string(dst.join("nvml_types.rs")).expect("read"),
            "// complete nvml_types.rs"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_promised_file_missing_from_the_out_dir_fails_and_stages_nothing() {
        let root = unique_test_dir("nv_rs_xtask_prebuilt_interrupted");
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(root.join("crates").join("nvml-sys")).expect("crate dir");
        let out = build_out(&root, "debug", "cccc2222cccc2222");
        write_complete_set(&out, "partial");
        let staged_root = root.join("staged");
        // A file the manifest promises but the out dir lacks is incomplete too.
        std::fs::remove_file(out.join("nvml_shims.rs")).expect("remove shims");
        let err = stage_dir_internal(&root, &root.join("target"), &staged_root, HOST, Some(HOST))
            .expect_err("missing shims must fail");
        assert!(err.contains("lists missing file(s) nvml_shims.rs"), "{err}");
        assert!(!staged_root.join("nvml-sys").exists(), "{err}");
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn builds_that_used_the_prebuilt_fallback_are_not_candidates() {
        let root = unique_test_dir("nv_rs_xtask_prebuilt_fallback");
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(root.join("crates").join("nvml-sys")).expect("crate dir");
        let out = build_out(&root, "debug", "ffff5555ffff5555");
        write_complete_set(&out, "copied");
        write_manifest(&out, "nvml", nv_bindgen::Origin::Prebuilt, &NVML_FILES);

        let staged_root = root.join("staged");
        let msg = stage_dir_internal(&root, &root.join("target"), &staged_root, HOST, Some(HOST))
            .expect("fallback builds are skipped, not errors");
        assert!(msg.contains("none (build the *-sys crates first)"), "{msg}");
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn only_the_files_the_manifest_lists_are_staged() {
        let root = unique_test_dir("nv_rs_xtask_prebuilt_manifest_files");
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(root.join("crates").join("nvml-sys")).expect("crate dir");
        let out = build_out(&root, "debug", "1111aaaa1111aaaa");
        write_complete_set(&out, "current");
        // A leftover of an older recipe: a binding artifact the manifest does not promise.
        std::fs::write(out.join("nvml_old_api.rs"), "// stale").expect("write");
        std::fs::write(out.join("nvml_old_shims.rs"), "// stale").expect("write");

        let staged_root = root.join("staged");
        let msg = stage_dir_internal(&root, &root.join("target"), &staged_root, HOST, Some(HOST))
            .expect("stage");
        assert!(msg.contains("nvml-sys (4 files)"), "{msg}");
        let dst = staged_root.join("nvml-sys").join("prebuilt").join(HOST);
        assert!(!dst.join("nvml_old_api.rs").exists(), "{msg}");
        assert!(!dst.join("nvml_old_shims.rs").exists(), "{msg}");
        assert!(
            !dst.join("nvml_manifest.json").exists(),
            "the manifest describes one build and is not staged"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn staging_removes_stale_artifacts_and_keeps_other_files() {
        let root = unique_test_dir("nv_rs_xtask_prebuilt_stale");
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(root.join("crates").join("nvml-sys")).expect("crate dir");
        write_complete_set(&build_out(&root, "debug", "dddd3333dddd3333"), "new");
        let staged_root = root.join("staged");
        let dst = staged_root.join("nvml-sys").join("prebuilt").join(HOST);
        std::fs::create_dir_all(&dst).expect("dst");
        std::fs::write(dst.join("nvml_old_api.rs"), "// stale").expect("write");
        std::fs::write(dst.join("nvml_old_shims.rs"), "// stale").expect("write");
        std::fs::write(dst.join("README.md"), "keep").expect("write");

        let msg = stage_dir_internal(&root, &root.join("target"), &staged_root, HOST, Some(HOST))
            .expect("stage");
        assert!(
            msg.contains("removed stale nvml_old_api.rs, nvml_old_shims.rs"),
            "{msg}"
        );
        assert!(!dst.join("nvml_old_api.rs").exists());
        assert!(!dst.join("nvml_old_shims.rs").exists());
        assert!(dst.join("README.md").is_file());
        assert!(dst.join("nvml_api.rs").is_file());
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn staging_is_all_or_nothing() {
        let root = unique_test_dir("nv_rs_xtask_prebuilt_atomic");
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(root.join("crates").join("nvml-sys")).expect("crate dir");
        write_complete_set(&build_out(&root, "debug", "eeee4444eeee4444"), "new");
        let staged_root = root.join("staged");
        let dst = staged_root.join("nvml-sys").join("prebuilt").join(HOST);
        std::fs::create_dir_all(dst.join("nvml_shims.rs")).expect("blocking directory");
        // `nvml_api.rs` sorts before the blocked `nvml_shims.rs`, so a file-by-file copy would
        // already have replaced it.
        std::fs::write(dst.join("nvml_api.rs"), "// old").expect("write");

        let err = stage_dir_internal(&root, &root.join("target"), &staged_root, HOST, Some(HOST))
            .expect_err("a destination that is a directory must fail");
        assert!(err.contains("nvml_shims.rs"), "{err}");
        assert_eq!(
            std::fs::read_to_string(dst.join("nvml_api.rs")).expect("read"),
            "// old",
            "no file may be replaced when the set cannot be staged completely"
        );
        let mut left: Vec<String> = std::fs::read_dir(&dst)
            .expect("list dst")
            .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
            .collect();
        left.sort();
        assert_eq!(left, ["nvml_api.rs", "nvml_shims.rs"], "no temp files left");
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn build_dirs_cover_plain_and_triple_qualified_profiles() {
        let root = unique_test_dir("nv_rs_xtask_prebuilt_build_dirs");
        std::fs::remove_dir_all(&root).ok();
        let target = root.join("target");
        for d in [
            target.join("debug").join("build"),
            target.join("aarch64-unknown-linux-gnu").join("release").join("build"),
            target.join("doc"),
        ] {
            std::fs::create_dir_all(d).expect("dir");
        }
        assert_eq!(
            build_dirs(&target),
            [
                target.join("aarch64-unknown-linux-gnu").join("release").join("build"),
                target.join("debug").join("build"),
            ]
        );
        std::fs::remove_dir_all(&root).ok();
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
        write_complete_set(&out_dir, "host");
        std::fs::write(out_dir.join("unrelated.txt"), "unrelated").expect("write");

        // `stage_dir_internal` with an explicit target dir: `stage_dir` honours `CARGO_TARGET_DIR`,
        // which the test environment may set.
        let staged_root = root.join("staged");
        let msg = stage_dir_internal(&root, &root.join("target"), &staged_root, HOST, Some(HOST))
            .expect("stage");
        assert!(msg.contains("nvml-sys (4 files)"), "{msg}");

        let dst = staged_root.join("nvml-sys").join("prebuilt").join(HOST);
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
        std::fs::write(foreign_out.join("nvml_inventory.json"), inventory_json("nvml"))
            .expect("write");
        write_manifest(&foreign_out, "nvml", nv_bindgen::Origin::Generated, &NVML_FILES);

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
        std::fs::write(host_out.join("nvml_inventory.json"), inventory_json("nvml"))
            .expect("write");
        write_manifest(&host_out, "nvml", nv_bindgen::Origin::Generated, &NVML_FILES);

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
        std::fs::write(out_dir.join("nvml_inventory.json"), inventory_json("nvml"))
            .expect("write");
        write_manifest(&out_dir, "nvml", nv_bindgen::Origin::Generated, &NVML_FILES);

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
