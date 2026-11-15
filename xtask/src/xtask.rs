//! nv-rs repository tasks. Today: the coverage matrix.
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
//! - **wrapped** — the overlay names a wrapper and the symbol is referenced in `crates/<lib>/src`;
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

mod optix_cov;
mod prebuilt;
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
    StageDlls(String),
    #[error(
        "usage: cargo xtask <coverage <collect|generate|check|optix> | prebuilt [--out <dir>] [--target <triple>] | stage-prebuilt [--out <dir>] [--target <triple>] | stage-dlls --out <dir> [--libs <csv>] [--include-driver] [--dry-run]>"
    )]
    Usage,
}

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
        ["coverage", "generate"] => generate(&root),
        ["coverage", "check"] => check(&root),
        ["coverage", "collect"] => collect(&root),
        ["coverage", "optix"] => optix_cov::run(&root).map_err(Error::Optix),
        ["prebuilt" | "stage-prebuilt", rest @ ..] => {
            let refs: Vec<&str> = rest.to_vec();
            prebuilt::run(&root, &refs).map_err(Error::Prebuilt)
        }
        ["stage-dlls", rest @ ..] => {
            let refs: Vec<&str> = rest.to_vec();
            stage_dlls::run(&refs).map_err(Error::StageDlls)
        }
        _ => Err(Error::Usage),
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
    std::fs::write(&path, &text).map_err(|source| Error::Io {
        path: path.clone(),
        source,
    })?;
    Ok(format!("wrote {}", path.display()))
}

/// Copies the newest `<lib>_inventory.json` each sys crate's build script wrote under
/// `target/*/build/*/out/` into `coverage/evidence/<lib>/inventory.json`. Run after building the
/// sys crates; the copied files are committed as evidence.
fn collect(root: &Path) -> Result<String, Error> {
    let mut newest: BTreeMap<String, (std::time::SystemTime, PathBuf)> = BTreeMap::new();
    for profile in list(&root.join("target")) {
        for build in list(&profile.join("build")) {
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
                    .unwrap_or(std::time::UNIX_EPOCH);
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
        let dst = dir.join("inventory.json");
        std::fs::write(&dst, text).map_err(|source| Error::Io { path: dst, source })?;
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
    for lib in &libs {
        let rows = rows(root, lib, &mut problems);
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

fn rows(root: &Path, lib: &Lib, problems: &mut Vec<String>) -> Vec<Row> {
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

    let safe_src = read_tree(&root.join("crates").join(&lib.name).join("src"));
    let all_src = read_tree(&root.join("crates"));
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
                if !contains_word(&all_src, &format!("fn {t}")) {
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
}
