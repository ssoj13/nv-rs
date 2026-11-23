//! nv-rs repository tasks: coverage matrices, prebuilt staging, DLL deployment, and Python stubs.
//!
//! **What.** `cargo xtask coverage generate` writes `COVERAGE.md` from
//! - `coverage/<lib>.toml` — hand-written overlay: per entry point, its safe wrapper and test id;
//! - `coverage/evidence/<lib>/inventory.json` — the symbol inventory `nv-bindgen` emits
//!   (entry-point names + header sha256, never header text);
//! - `coverage/evidence/<lib>/gpu-tests.json` — GPU test records (GPU, driver, platform, result).
//!
//! `cargo xtask coverage check` regenerates in memory and fails when `COVERAGE.md` differs, an
//! overlay names a symbol the inventory lacks, or a test id cited as evidence does not exist in
//! the tree. Statuses are derived, never typed by hand:
//! - **bound** — the symbol is in the inventory;
//! - **wrapped** — the overlay names a wrapper and the symbol is referenced in code (comments
//!   stripped) under `crates/<lib>/src`;
//! - **tested-on-gpu** — a passing GPU record exists for the overlay's test id.
//!
//! **Why.** The matrix is the evidence shown to NVIDIA; a table edited by hand drifts from the
//! code the first week. Modelled on cuda-oxide's intrinsics catalog + overlay + evidence.
//!
//! `cargo xtask coverage optix` is a separate gate (module `optix_cov`) proving the OptiX API is
//! fully covered: every `optix_device.h` function is mapped in `device/optix-device/API_MAP.toml`
//! to an existing Rust item, every `OptixFunctionTable` field is used by `crates/optix`, and every
//! `_optix_*` intrinsic is used by the typed device layer. It prints counts and exits non-zero,
//! listing the missing names, on any gap.
//!
//! `cargo xtask py-stubs <generate|check>` (module `py_stubs`) parses `python/nv-py/src/*.rs` and
//! `python/nv-py/python/nv_rs/__init__.py` to deterministically generate or verify
//! `python/nv-py/python/nv_rs/__init__.pyi` and `docs/src/content/docs/python/04-modules-reference.md`.
//!
//! Every file xtask writes goes through `atomic_io` (temp file in the same directory, `sync_all`,
//! rename), so an interrupted run never leaves a truncated artifact.

mod atomic_io;
mod optix_cov;
mod prebuilt;
mod py_stubs;
mod stage_dlls;

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("{path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{path}: {msg}")]
    Parse { path: PathBuf, msg: String },
    #[error("coverage check failed:\n{0}")]
    Check(String),
    #[error("optix coverage failed:\n{0}")]
    Optix(String),
    #[error("{0}")]
    Prebuilt(String),
    #[error("{0}")]
    PyStubs(String),
    #[error("{0}")]
    StageDlls(String),
    #[error("{0}")]
    Write(String),
    /// A command or subcommand was missing or unknown; carries the help text to print.
    #[error("{0}")]
    Usage(&'static str),
}

const HELP: &str = "\
usage: cargo xtask <command> [args]     (each command and subcommand: long name | 1-letter alias)

commands:
  coverage|c <collect|l, generate|g, check|c, optix|o>   Generate or verify API coverage matrices
  py-stubs|y <generate|g, check|c>                       Generate or verify Python .pyi stubs and docs
  prebuilt|p [--out <dir>] [--target <triple>]           Stage prebuilt sys bindings (alias: stage-prebuilt)
  stage-dlls|d --out <dir> [--libs <csv>] [--include-driver] [--dry-run]   Stage runtime DLLs

examples:
  cargo xtask coverage check
  cargo xtask c g
  cargo xtask py-stubs generate
  cargo xtask prebuilt --target x86_64-pc-windows-msvc
  cargo xtask stage-dlls --out dist --libs cufft,nvrtc";

const COVERAGE_HELP: &str = "\
usage: cargo xtask coverage|c <subcommand>
  collect|l    copy the newest <lib>_inventory.json of each built sys crate into coverage/evidence/
  generate|g   write COVERAGE.md from coverage/
  check|c      fail when COVERAGE.md is stale or the overlays are inconsistent
  optix|o      prove the OptiX device, host and intrinsic APIs are fully covered
example: cargo xtask coverage check";

const PY_STUBS_HELP: &str = "\
usage: cargo xtask py-stubs|y <subcommand>
  generate|g   write python/nv-py/python/nv_rs/__init__.pyi and the Python reference page
  check|c      fail when either file is stale
example: cargo xtask py-stubs check";

/// `coverage/<lib>.toml`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Overlay {
    lib: String,
    #[serde(default, rename = "fn")]
    fns: Vec<OverlayFn>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct OverlayFn {
    name: String,
    #[serde(default)]
    wrapper: Option<String>,
    #[serde(default)]
    test: Option<String>,
    #[serde(default)]
    note: Option<String>,
}

/// `coverage/evidence/<lib>/inventory.json` (nv-bindgen's `Inventory`).
#[derive(Debug, Deserialize)]
struct Inventory {
    lib: String,
    headers: Vec<HeaderHash>,
    symbols: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct HeaderHash {
    name: String,
    sha256: String,
}

/// One entry of `coverage/evidence/<lib>/gpu-tests.json`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GpuRecord {
    test: String,
    gpu: String,
    driver: String,
    platform: String,
    commit: String,
    date: String,
    passed: bool,
}

/// Everything known about one library.
struct Lib {
    name: String,
    overlay: Option<Overlay>,
    inventory: Option<Inventory>,
    gpu: Vec<GpuRecord>,
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = repo_root();
    let result = match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["help" | "--help" | "-h"] => Ok(HELP.to_string()),
        ["coverage" | "c", "generate" | "g"] => generate(&root),
        ["coverage" | "c", "check" | "c"] => check(&root),
        ["coverage" | "c", "collect" | "l"] => collect(&root),
        ["coverage" | "c", "optix" | "o"] => optix_cov::run(&root).map_err(Error::Optix),
        ["coverage" | "c", "help" | "--help" | "-h"] => Ok(COVERAGE_HELP.to_string()),
        ["coverage" | "c", ..] => Err(Error::Usage(COVERAGE_HELP)),
        ["py-stubs" | "y", "generate" | "g"] => py_stubs::generate(&root).map_err(Error::PyStubs),
        ["py-stubs" | "y", "check" | "c"] => py_stubs::check(&root).map_err(Error::PyStubs),
        ["py-stubs" | "y", "help" | "--help" | "-h"] => Ok(PY_STUBS_HELP.to_string()),
        ["py-stubs" | "y", ..] => Err(Error::Usage(PY_STUBS_HELP)),
        ["prebuilt" | "stage-prebuilt" | "p", rest @ ..] => {
            prebuilt::run(&root, rest).map_err(Error::Prebuilt)
        }
        ["stage-dlls" | "d", rest @ ..] => stage_dlls::run(rest).map_err(Error::StageDlls),
        _ => Err(Error::Usage(HELP)),
    };
    match result {
        Ok(msg) => {
            println!("{msg}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn generate(root: &Path) -> Result<String, Error> {
    let (text, _) = render(root)?;
    let path = root.join("COVERAGE.md");
    atomic_io::write(&path, text.as_bytes()).map_err(Error::Write)?;
    Ok(format!("wrote {}", path.display()))
}

/// Copies the newest `<lib>_inventory.json` each sys crate's build script wrote under
/// `<target dir>/<profile>/build/*/out/` or `<target dir>/<triple>/<profile>/build/*/out/` (the
/// target dir honours `CARGO_TARGET_DIR`) into `coverage/evidence/<lib>/inventory.json`. Run after
/// building the sys crates; the copied files are committed as evidence.
fn collect(root: &Path) -> Result<String, Error> {
    collect_from(root, &prebuilt::resolve_target_dir(root))
}

fn collect_from(root: &Path, target_dir: &Path) -> Result<String, Error> {
    let mut newest: BTreeMap<String, (std::time::SystemTime, PathBuf)> = BTreeMap::new();
    for build_dir in prebuilt::build_dirs(target_dir) {
        for build in list(&build_dir) {
            // nv-bindgen-fixture's synthetic libraries are a test harness, not NVIDIA evidence.
            if build
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("nv-bindgen-fixture-"))
            {
                continue;
            }
            for file in list(&build.join("out")) {
                let Some(name) = file.file_name().and_then(|n| n.to_str()) else {
                    continue;
                };
                let Some(lib) = name.strip_suffix("_inventory.json") else {
                    continue;
                };
                let modified = std::fs::metadata(&file)
                    .and_then(|m| m.modified())
                    .map_err(|source| Error::Io {
                        path: file.clone(),
                        source,
                    })?;
                if newest.get(lib).is_none_or(|(t, _)| modified > *t) {
                    newest.insert(lib.to_string(), (modified, file));
                }
            }
        }
    }
    let mut copied = Vec::new();
    for (lib, (_, src)) in &newest {
        let text = read(src)?;
        serde_json::from_str::<Inventory>(&text).map_err(|e| Error::Parse {
            path: src.clone(),
            msg: e.to_string(),
        })?;
        let dir = root.join("coverage").join("evidence").join(lib);
        std::fs::create_dir_all(&dir).map_err(|source| Error::Io {
            path: dir.clone(),
            source,
        })?;
        atomic_io::write(&dir.join("inventory.json"), text.as_bytes()).map_err(Error::Write)?;
        copied.push(lib.as_str());
    }
    Ok(format!(
        "collected inventories: {}",
        if copied.is_empty() {
            "none (build the sys crates first)".to_string()
        } else {
            copied.join(", ")
        }
    ))
}

fn check(root: &Path) -> Result<String, Error> {
    let (text, mut problems) = render(root)?;
    let path = root.join("COVERAGE.md");
    match std::fs::read_to_string(&path) {
        Ok(on_disk) if on_disk == text => {}
        Ok(_) => problems.push("COVERAGE.md is stale: run `cargo xtask coverage generate`".into()),
        Err(_) => {
            problems.push("COVERAGE.md is missing: run `cargo xtask coverage generate`".into())
        }
    }
    if problems.is_empty() {
        Ok("coverage check: ok".into())
    } else {
        Err(Error::Check(problems.join("\n")))
    }
}

/// The matrix text plus every consistency problem found while building it.
fn render(root: &Path) -> Result<(String, Vec<String>), Error> {
    let libs = load_libs(root)?;
    let mut problems = Vec::new();
    let mut out = String::from(
        "# Coverage\n\nGenerated by `cargo xtask coverage generate` from `coverage/`. Do not edit.\n\n\
         Status: **bound** (in the generated inventory), **wrapped** (a safe wrapper references it), \
         **tested-on-gpu** (a passing GPU test record exists).\n",
    );
    if libs.is_empty() {
        out.push_str("\nNo libraries yet.\n");
        return Ok((out, problems));
    }

    out.push_str(
        "\n| Library | Symbols | Bound | Wrapped | Tested on GPU |\n|---|---:|---:|---:|---:|\n",
    );
    let mut sections = String::new();
    // Read once for every library: test ids are looked up across the whole tree.
    let all_src = strip_comments(&read_tree(&root.join("crates")));
    for lib in &libs {
        let rows = rows(root, lib, &all_src, &mut problems);
        let count = |f: fn(&Row) -> bool| rows.iter().filter(|r| f(r)).count();
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            lib.name,
            rows.len(),
            count(|r| r.bound),
            count(|r| r.wrapped),
            count(|r| r.tested.is_some())
        ));

        sections.push_str(&format!("\n## {}\n\n", lib.name));
        if let Some(inv) = &lib.inventory {
            for h in &inv.headers {
                sections.push_str(&format!("- `{}` sha256 `{}`\n", h.name, h.sha256));
            }
            sections.push('\n');
        }
        sections.push_str(
            "| Symbol | Bound | Wrapper | Tested on GPU | Note |\n|---|---|---|---|---|\n",
        );
        for r in &rows {
            sections.push_str(&format!(
                "| `{}` | {} | {} | {} | {} |\n",
                r.name,
                mark(r.bound),
                r.wrapper
                    .as_deref()
                    .filter(|_| r.wrapped)
                    .map_or_else(String::new, |w| format!("`{w}`")),
                r.tested.as_deref().unwrap_or(""),
                r.note.as_deref().unwrap_or("")
            ));
        }
    }
    out.push_str(&sections);
    Ok((out, problems))
}

struct Row {
    name: String,
    bound: bool,
    wrapper: Option<String>,
    wrapped: bool,
    tested: Option<String>,
    note: Option<String>,
}

fn mark(b: bool) -> &'static str {
    if b { "yes" } else { "" }
}

/// `all_src` is the comment-stripped source of every crate under `crates/`.
fn rows(root: &Path, lib: &Lib, all_src: &str, problems: &mut Vec<String>) -> Vec<Row> {
    let symbols: Vec<String> = lib
        .inventory
        .as_ref()
        .map(|i| i.symbols.clone())
        .unwrap_or_default();
    let overlay: BTreeMap<&str, &OverlayFn> = lib
        .overlay
        .iter()
        .flat_map(|o| o.fns.iter())
        .map(|f| (f.name.as_str(), f))
        .collect();

    if let (Some(o), Some(inv)) = (&lib.overlay, &lib.inventory) {
        if o.lib != inv.lib {
            problems.push(format!(
                "{}: overlay lib `{}` != inventory lib `{}`",
                lib.name, o.lib, inv.lib
            ));
        }
        for name in overlay.keys() {
            if !symbols.iter().any(|s| s == name) {
                problems.push(format!(
                    "{}: overlay names `{name}`, which the inventory does not contain",
                    lib.name
                ));
            }
        }
    }

    // Comments are stripped: a wrapper that only mentions the C symbol in its docs does not wrap it.
    let safe_src = strip_comments(&read_tree(&wrapper_src_dir(root, &lib.name)));
    let mut names: Vec<String> = symbols.clone();
    for n in overlay.keys() {
        if !names.iter().any(|s| s == n) {
            names.push((*n).to_string());
        }
    }
    names.sort();

    names
        .into_iter()
        .map(|name| {
            let o = overlay.get(name.as_str());
            let wrapper = o.and_then(|f| f.wrapper.clone());
            let wrapped = wrapper.is_some() && contains_word(&safe_src, &name);
            let test = o.and_then(|f| f.test.clone());
            let tested = test.as_ref().and_then(|t| {
                if !contains_word(all_src, &format!("fn {t}")) {
                    problems.push(format!(
                        "{}: test `{t}` for `{name}` does not exist under crates/",
                        lib.name
                    ));
                    return None;
                }
                lib.gpu.iter().find(|g| &g.test == t && g.passed).map(|g| {
                    format!(
                        "{} {} ({}, {}, {})",
                        g.gpu, g.driver, g.platform, g.date, g.commit
                    )
                })
            });
            Row {
                bound: symbols.contains(&name),
                name,
                wrapper,
                wrapped,
                tested,
                note: o.and_then(|f| f.note.clone()),
            }
        })
        .collect()
}

fn load_libs(root: &Path) -> Result<Vec<Lib>, Error> {
    let cov = root.join("coverage");
    let mut libs: BTreeMap<String, Lib> = BTreeMap::new();
    for path in list(&cov) {
        if path.extension().is_some_and(|e| e == "toml")
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
        {
            let o: Overlay = toml::from_str(&read(&path)?).map_err(|e| Error::Parse {
                path: path.clone(),
                msg: e.to_string(),
            })?;
            lib_entry(&mut libs, stem).overlay = Some(o);
        }
    }
    for dir in list(&cov.join("evidence")) {
        let Some(name) = dir.file_name().and_then(|s| s.to_str()).map(str::to_string) else {
            continue;
        };
        if !dir.is_dir() {
            continue;
        }
        let inv_path = dir.join("inventory.json");
        let inventory = if inv_path.is_file() {
            Some(
                serde_json::from_str::<Inventory>(&read(&inv_path)?).map_err(|e| Error::Parse {
                    path: inv_path,
                    msg: e.to_string(),
                })?,
            )
        } else {
            None
        };
        let gpu_path = dir.join("gpu-tests.json");
        let gpu = if gpu_path.is_file() {
            serde_json::from_str::<Vec<GpuRecord>>(&read(&gpu_path)?).map_err(|e| Error::Parse {
                path: gpu_path,
                msg: e.to_string(),
            })?
        } else {
            Vec::new()
        };
        let lib = lib_entry(&mut libs, &name);
        lib.inventory = inventory;
        lib.gpu = gpu;
    }
    Ok(libs.into_values().collect())
}

fn lib_entry<'a>(libs: &'a mut BTreeMap<String, Lib>, name: &str) -> &'a mut Lib {
    libs.entry(name.to_string()).or_insert_with(|| Lib {
        name: name.to_string(),
        overlay: None,
        inventory: None,
        gpu: Vec::new(),
    })
}

fn list(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .map(|it| it.filter_map(|e| e.ok()).map(|e| e.path()).collect())
        .unwrap_or_default();
    v.sort();
    v
}

fn read(path: &Path) -> Result<String, Error> {
    std::fs::read_to_string(path).map_err(|source| Error::Io {
        path: path.to_path_buf(),
        source,
    })
}

/// Resolves the crate `src/` directory that holds safe wrappers for `lib_name`.
fn wrapper_src_dir(root: &Path, lib_name: &str) -> PathBuf {
    let crate_name = match lib_name {
        "cublaslt" => "cublas",
        "cusolvermg" => "cusolver",
        "nvcuvid" => "nvdec",
        other => other,
    };
    root.join("crates").join(crate_name).join("src")
}

/// Concatenated `.rs` sources under `dir` (empty when it does not exist).
fn read_tree(dir: &Path) -> String {
    let mut out = String::new();
    for p in list(dir) {
        if p.is_dir() {
            if p.file_name().is_some_and(|n| n == "target") {
                continue;
            }
            out.push_str(&read_tree(&p));
        } else if p.extension().is_some_and(|e| e == "rs")
            && let Ok(s) = std::fs::read_to_string(&p)
        {
            out.push_str(&s);
            out.push('\n');
        }
    }
    out
}

/// Source text (Rust or C) with `//` line comments and `/* */` block comments removed, so prose
/// never counts as use. Doc comments are line comments too. Block comments nest as in Rust; in C a
/// nested `/*` is unusual, and over-stripping makes a gate fail loudly rather than pass silently.
/// String, raw-string and char literals are copied verbatim, so `"http://..."` does not cut the
/// line and `'"'` does not open a string. Newlines inside block comments are kept, so line-based
/// scanners see the same line structure.
fn strip_comments(src: &str) -> String {
    let b = src.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'/' if b.get(i + 1) == Some(&b'/') => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                let mut depth = 1usize;
                i += 2;
                while i < b.len() && depth > 0 {
                    if b[i] == b'/' && b.get(i + 1) == Some(&b'*') {
                        depth += 1;
                        i += 2;
                    } else if b[i] == b'*' && b.get(i + 1) == Some(&b'/') {
                        depth -= 1;
                        i += 2;
                    } else {
                        if b[i] == b'\n' {
                            out.push(b'\n');
                        }
                        i += 1;
                    }
                }
            }
            b'"' => {
                let end = quoted_end(b, i + 1);
                out.extend_from_slice(&b[i..end]);
                i = end;
            }
            b'r' => match raw_string_end(b, i) {
                Some(end) => {
                    out.extend_from_slice(&b[i..end]);
                    i = end;
                }
                None => {
                    out.push(b'r');
                    i += 1;
                }
            },
            b'\'' => {
                let end = char_literal_end(b, i).unwrap_or(i + 1);
                out.extend_from_slice(&b[i..end]);
                i = end;
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    // Cuts happen only at ASCII bytes, so the output is valid UTF-8 and nothing is substituted.
    String::from_utf8_lossy(&out).into_owned()
}

/// End (exclusive) of a `"..."` literal whose body starts at `start`, honouring `\` escapes.
fn quoted_end(b: &[u8], start: usize) -> usize {
    let mut i = start;
    while i < b.len() {
        match b[i] {
            b'\\' => i += 2,
            b'"' => return i + 1,
            _ => i += 1,
        }
    }
    b.len()
}

/// End (exclusive) of a raw string literal (`r"…"`, `r#"…"#`, `br"…"`) whose `r` is at `b[i]`,
/// or `None` when no raw string starts there (an identifier such as `for` or `r2`).
fn raw_string_end(b: &[u8], i: usize) -> Option<usize> {
    let ident = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let starts_token = match i.checked_sub(1).map(|p| b[p]) {
        None => true,
        Some(b'b') => i < 2 || !ident(b[i - 2]),
        Some(c) => !ident(c),
    };
    if !starts_token {
        return None;
    }
    let mut j = i + 1;
    while b.get(j) == Some(&b'#') {
        j += 1;
    }
    if b.get(j) != Some(&b'"') {
        return None;
    }
    let hashes = j - i - 1;
    j += 1;
    while j < b.len() {
        if b[j] == b'"'
            && b
                .get(j + 1..j + 1 + hashes)
                .is_some_and(|h| h.iter().all(|&c| c == b'#'))
        {
            return Some(j + 1 + hashes);
        }
        j += 1;
    }
    Some(b.len())
}

/// End (exclusive) of a char literal starting at `b[i] == '\''` (`'x'`, `'"'`, `'\''`, `'\u{..}'`),
/// or `None` for a lifetime / label (`'a`), which is copied as a plain character.
fn char_literal_end(b: &[u8], i: usize) -> Option<usize> {
    if b.get(i + 1) == Some(&b'\\') {
        let close = b.get(i + 3..)?.iter().take(10).position(|&c| c == b'\'')?;
        Some(i + 3 + close + 1)
    } else if b.get(i + 2) == Some(&b'\'') {
        Some(i + 3)
    } else {
        None
    }
}

/// `needle` occurs with no identifier character on either side.
fn contains_word(haystack: &str, needle: &str) -> bool {
    let is_ident = |c: char| c.is_alphanumeric() || c == '_';
    haystack.match_indices(needle).any(|(i, _)| {
        let before = haystack[..i].chars().next_back();
        let after = haystack[i + needle.len()..].chars().next();
        !before.is_some_and(is_ident) && !after.is_some_and(is_ident)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_match_respects_identifier_boundaries() {
        // `nvmlInit` must not count as wrapped because `nvmlInit_v2` appears.
        assert!(contains_word("x(nvmlInit_v2)", "nvmlInit_v2"));
        assert!(!contains_word("x(nvmlInit_v2)", "nvmlInit"));
        assert!(contains_word("fn init_reports() {}", "fn init_reports"));
    }

    #[test]
    fn an_empty_coverage_dir_renders_and_checks_clean() {
        let dir = std::env::temp_dir().join("nv_rs_xtask_empty");
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(dir.join("coverage").join("evidence")).expect("dirs");
        let (text, problems) = render(&dir).expect("renders");
        assert!(text.contains("No libraries yet."));
        assert!(problems.is_empty());
        std::fs::write(dir.join("COVERAGE.md"), &text).expect("write");
        assert!(check(&dir).is_ok());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn statuses_are_derived_and_inconsistencies_reported() {
        let dir = std::env::temp_dir().join("nv_rs_xtask_lib");
        std::fs::remove_dir_all(&dir).ok();
        let ev = dir.join("coverage").join("evidence").join("demo");
        std::fs::create_dir_all(&ev).expect("dirs");
        std::fs::create_dir_all(dir.join("crates").join("demo").join("src")).expect("src");
        std::fs::write(
            ev.join("inventory.json"),
            r#"{"lib":"demo","headers":[{"name":"demo.h","sha256":"ab"}],"symbols":["demoA","demoB"]}"#,
        )
        .expect("inventory");
        std::fs::write(
            ev.join("gpu-tests.json"),
            r#"[{"test":"a_works","gpu":"RTX","driver":"621","platform":"windows","commit":"abc","date":"2026-11-12","passed":true}]"#,
        )
        .expect("gpu");
        std::fs::write(
            dir.join("coverage").join("demo.toml"),
            "lib = \"demo\"\n[[fn]]\nname = \"demoA\"\nwrapper = \"Demo::a\"\ntest = \"a_works\"\n[[fn]]\nname = \"demoZ\"\n",
        )
        .expect("overlay");
        std::fs::write(
            dir.join("crates").join("demo").join("src").join("lib.rs"),
            "fn a() { demoA() }\n#[test] fn a_works() {}\n",
        )
        .expect("lib");

        let (text, problems) = render(&dir).expect("renders");
        assert!(text.contains("| demo | 3 | 2 | 1 | 1 |"), "{text}");
        assert_eq!(
            problems,
            ["demo: overlay names `demoZ`, which the inventory does not contain"]
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    fn unique_dir(label: &str) -> PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "nv_rs_xtask_cov_{label}_{}_{id}",
            std::process::id()
        ));
        std::fs::remove_dir_all(&dir).ok();
        dir
    }

    #[test]
    fn a_symbol_mentioned_only_in_comments_is_not_wrapped() {
        let dir = unique_dir("comments");
        let ev = dir.join("coverage").join("evidence").join("demo");
        std::fs::create_dir_all(&ev).expect("dirs");
        std::fs::create_dir_all(dir.join("crates").join("demo").join("src")).expect("src");
        std::fs::write(
            ev.join("inventory.json"),
            r#"{"lib":"demo","headers":[],"symbols":["demoA","demoB"]}"#,
        )
        .expect("inventory");
        std::fs::write(
            dir.join("coverage").join("demo.toml"),
            "lib = \"demo\"\n[[fn]]\nname = \"demoA\"\nwrapper = \"Demo::a\"\n[[fn]]\nname = \"demoB\"\nwrapper = \"Demo::b\"\n",
        )
        .expect("overlay");
        std::fs::write(
            dir.join("crates").join("demo").join("src").join("lib.rs"),
            "/// Wraps `demoB`.\nfn b() {}\n/* demoB */\nfn a() { sys::demoA() }\n",
        )
        .expect("lib");

        let (text, problems) = render(&dir).expect("renders");
        assert!(problems.is_empty(), "{problems:?}");
        assert!(text.contains("| demo | 2 | 2 | 1 | 0 |"), "{text}");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn collect_reads_triple_qualified_builds_and_writes_evidence() {
        let dir = unique_dir("collect");
        let target = dir.join("target");
        let out = target
            .join("aarch64-unknown-linux-gnu")
            .join("release")
            .join("build")
            .join("demo-sys-0123abcd")
            .join("out");
        std::fs::create_dir_all(&out).expect("out dir");
        let inv = r#"{"lib":"demo","headers":[],"symbols":["demoA"]}"#;
        std::fs::write(out.join("demo_inventory.json"), inv).expect("inventory");

        let msg = collect_from(&dir, &target).expect("collect");
        assert_eq!(msg, "collected inventories: demo");
        let evidence = dir
            .join("coverage")
            .join("evidence")
            .join("demo")
            .join("inventory.json");
        assert_eq!(std::fs::read_to_string(evidence).expect("evidence"), inv);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn comments_are_stripped_but_literals_are_kept() {
        let src = "a // x\nb /* y\n /* z */ w */ c\n\"//s\" '\"' r#\"/*\"# d // e\n'a: loop {}";
        assert_eq!(
            strip_comments(src),
            "a \nb \n c\n\"//s\" '\"' r#\"/*\"# d \n'a: loop {}"
        );
    }

    #[test]
    fn wrapper_src_dir_maps_shared_crate_libraries() {
        let root = Path::new("/repo");
        assert_eq!(
            wrapper_src_dir(root, "cublaslt"),
            root.join("crates").join("cublas").join("src")
        );
        assert_eq!(
            wrapper_src_dir(root, "cusolvermg"),
            root.join("crates").join("cusolver").join("src")
        );
        assert_eq!(
            wrapper_src_dir(root, "nvcuvid"),
            root.join("crates").join("nvdec").join("src")
        );
        assert_eq!(
            wrapper_src_dir(root, "nvenc"),
            root.join("crates").join("nvenc").join("src")
        );
    }
}
