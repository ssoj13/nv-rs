//! `cargo xtask coverage optix`: a deterministic gate proving the OptiX API is fully covered.
//!
//! **What.** Three independent checks against the installed OptiX SDK (located through vcv-rs):
//! 1. *Device API* — every public function of `optix_device.h` (`static __forceinline__ __device__
//!    <ret> optixName(`) must have an entry in `device/optix-device/API_MAP.toml`, every entry must
//!    name an item that exists in the named module's source (`pub fn`, `macro_rules!`, `pub use`, or
//!    a row of a function-generating macro table), and no entry may be stale (absent from the header).
//! 2. *Host API* — every field of `OptixFunctionTable` (`optix_function_table.h`) must be used by the
//!    safe host crate: `entry!(<table>, <field>)` or a direct `.<field>` access (destructors and
//!    error-string helpers read the table without the erroring `entry!`), except [`HOST_ALLOW`]
//!    (reserved placeholders without public meaning).
//! 3. *Raw intrinsics* — every `_optix_*` intrinsic found by `nv_bindgen::device_asm::scan_optix`
//!    must be used as `raw::<name without _optix_>` in `device/optix-device/src` (or, for the macro
//!    crate, named by a string literal), except [`RAW_ONLY`].
//!
//! **Why.** "Fully covered" is a claim shown to NVIDIA; a list kept by hand drifts the first time the
//! SDK adds a function. Headers are parsed, never copied, so a new SDK fails the gate until mapped.
//!
//! **How.** The parsers are pure functions over header / source text so they are unit-tested on
//! small synthetic inputs; [`run`] wires them to the SDK and the repository.
//!
//! **Used by** `xtask.rs` (`coverage optix`).

use crate::{contains_word, read_tree};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// `OptixFunctionTable` fields that are reserved slots, not API: no public meaning to wrap.
const HOST_ALLOW: &[(&str, &str)] = &[
    (
        "optixStub",
        "placeholder slot inside the table; OptiX defines no function for it",
    ),
    ("stub1", "reserved slot; OptiX defines no function for it"),
    ("stub2", "reserved slot; OptiX defines no function for it"),
];

/// `_optix_*` intrinsics reachable only through `raw::`, with the reason (name without prefix).
/// Empty on purpose: every intrinsic must be used by the typed layer.
const RAW_ONLY: &[(&str, &str)] = &[];

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Source text with `//` line comments (doc comments included) removed, so prose never counts as use.
fn strip_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    for line in src.lines() {
        out.push_str(line.find("//").map_or(line, |i| &line[..i]));
        out.push('\n');
    }
    out
}

/// The trailing identifier of `s` (`"static void optixTrace"` -> `"optixTrace"`).
fn last_ident(s: &str) -> &str {
    let t = s.trim_end();
    let start = t
        .char_indices()
        .rev()
        .find(|(_, c)| !is_ident(*c))
        .map_or(0, |(i, c)| i + c.len_utf8());
    &t[start..]
}

/// A line opening a device declaration: its first three words are `static`, `__forceinline__` and
/// `__device__` in any order (the header spells both orders).
fn is_device_decl(line: &str) -> bool {
    let mut seen = [false; 3];
    for w in line.split_whitespace().take(3) {
        match w {
            "static" => seen[0] = true,
            "__forceinline__" => seen[1] = true,
            "__device__" => seen[2] = true,
            _ => return false,
        }
    }
    seen.iter().all(|s| *s)
}

/// Every public `optix*` function name declared in `optix_device.h` (overloads collapse to one name).
///
/// A declaration may continue on the next line (`... __device__ uint4` / `optixTexFootprint2DLod(`),
/// so up to three following lines are joined until the opening parenthesis appears.
pub fn device_api(header: &str) -> BTreeSet<String> {
    let lines: Vec<&str> = header.lines().collect();
    let mut out = BTreeSet::new();
    for (i, line) in lines.iter().enumerate() {
        if !is_device_decl(line) {
            continue;
        }
        let mut decl = line.trim().to_string();
        let mut j = i;
        while !decl.contains('(') && j + 1 < lines.len() && j - i < 3 {
            j += 1;
            decl.push(' ');
            decl.push_str(lines[j].trim());
        }
        let Some(open) = decl.find('(') else { continue };
        let name = last_ident(&decl[..open]);
        if name.starts_with("optix") {
            out.insert(name.to_string());
        }
    }
    out
}

/// If `b[open]` starts a function-pointer declarator `( * name ) (`, returns the name and the index
/// of the second `(`.
fn fn_ptr_at(b: &[u8], open: usize) -> Option<(String, usize)> {
    let skip = |mut i: usize| {
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        i
    };
    let mut i = skip(open + 1);
    if b.get(i) != Some(&b'*') {
        return None;
    }
    i = skip(i + 1);
    let start = i;
    while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
        i += 1;
    }
    if i == start {
        return None;
    }
    // ASCII by the loop above, so the lossy conversion never substitutes anything.
    let name = String::from_utf8_lossy(&b[start..i]).into_owned();
    i = skip(i);
    if b.get(i) != Some(&b')') {
        return None;
    }
    i = skip(i + 1);
    if b.get(i) != Some(&b'(') {
        return None;
    }
    Some((name, i))
}

/// The fields of `OptixFunctionTable` in declaration order (every `( *name )(` of the struct body).
pub fn table_fields(header: &str) -> Result<Vec<String>, String> {
    let start = header
        .find("typedef struct OptixFunctionTable")
        .ok_or("`typedef struct OptixFunctionTable` not found")?;
    let rest = &header[start..];
    let end = rest
        .find("} OptixFunctionTable;")
        .ok_or("end of `OptixFunctionTable` not found")?;
    let body = strip_comments(&rest[..end]);
    let b = body.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'('
            && let Some((name, next)) = fn_ptr_at(b, i)
        {
            out.push(name);
            i = next;
        } else {
            i += 1;
        }
    }
    Ok(out)
}

/// The last top-level argument of every `entry!(...)` call (the table field), whatever the table
/// expression is (`t`, `ctx.table()`, `self.ctx.table()`).
fn entry_args(src: &str) -> BTreeSet<String> {
    let b = src.as_bytes();
    let mut out = BTreeSet::new();
    for (at, _) in src.match_indices("entry!(") {
        let mut depth = 1usize;
        let mut i = at + "entry!(".len();
        let mut last = i;
        while i < b.len() && depth > 0 {
            match b[i] {
                b'(' | b'[' | b'{' => depth += 1,
                b')' | b']' | b'}' => depth -= 1,
                b',' if depth == 1 => last = i + 1,
                _ => {}
            }
            i += 1;
        }
        if depth == 0 {
            out.insert(src[last..i - 1].trim().to_string());
        }
    }
    out
}

/// `field` occurs as `.field` (a member access) and not as a longer identifier.
fn dot_access(src: &str, field: &str) -> bool {
    src.match_indices(field).any(|(i, _)| {
        src[..i].ends_with('.') && !src[i + field.len()..].chars().next().is_some_and(is_ident)
    })
}

/// Which of `fields` the host sources use: through `entry!` or as a direct table member access
/// (comments are ignored).
pub fn host_refs(src: &str, fields: &[String]) -> BTreeSet<String> {
    let src = strip_comments(src);
    let args = entry_args(&src);
    fields
        .iter()
        .filter(|f| args.contains(f.as_str()) || dot_access(&src, f))
        .cloned()
        .collect()
}

/// `API_MAP.toml` value: one path or several.
#[derive(Deserialize)]
#[serde(untagged)]
enum OneOrMany {
    One(String),
    Many(Vec<String>),
}

/// Parses `API_MAP.toml` into header name -> Rust item paths.
pub fn parse_map(text: &str) -> Result<BTreeMap<String, Vec<String>>, String> {
    let raw: BTreeMap<String, OneOrMany> = toml::from_str(text).map_err(|e| e.to_string())?;
    Ok(raw
        .into_iter()
        .map(|(k, v)| {
            let paths = match v {
                OneOrMany::One(s) => vec![s],
                OneOrMany::Many(v) => v,
            };
            (k, paths)
        })
        .collect())
}

/// Splits `module::item` (a trailing `::<N>` generic argument is dropped) into its module and item.
fn split_path(p: &str) -> Result<(&str, &str), String> {
    let base = p.find("::<").map_or(p, |i| &p[..i]);
    let module = base.split("::").next().filter(|s| !s.is_empty());
    let item = base.rsplit("::").next().filter(|s| !s.is_empty());
    match (module, item) {
        (Some(m), Some(i)) if base.contains("::") => Ok((m, i)),
        _ => Err(format!("`{p}` is not `<module>::<item>`")),
    }
}

/// `s` starts with `ident` and the identifier does not continue.
fn starts_with_ident(s: &str, ident: &str) -> bool {
    s.strip_prefix(ident)
        .is_some_and(|r| !r.chars().next().is_some_and(is_ident))
}

/// A row of a function-generating macro table: `name -> ty = raw;` or `name = raw, conv -> ty;`.
fn macro_row(line: &str, item: &str) -> bool {
    let Some(rest) = line.strip_prefix(item) else {
        return false;
    };
    if rest.chars().next().is_some_and(is_ident) {
        return false;
    }
    let rest = rest.trim_start();
    if rest.starts_with("->") {
        return true;
    }
    let Some(r) = rest.strip_prefix('=') else {
        return false;
    };
    let r = r.trim_start();
    let end = r.find(|c: char| !is_ident(c)).unwrap_or(r.len());
    end > 0 && r[end..].trim_start().starts_with(',')
}

/// `item` is defined in comment-stripped module source `src`.
fn item_exists(src: &str, item: &str) -> bool {
    const KINDS: [&str; 8] = [
        "fn ", "mod ", "struct ", "enum ", "trait ", "type ", "const ", "static ",
    ];
    src.lines().map(str::trim_start).any(|line| {
        if let Some(rest) = line.strip_prefix("pub ") {
            let after = ["const unsafe ", "unsafe ", "const "]
                .iter()
                .find_map(|q| rest.strip_prefix(q))
                .unwrap_or(rest);
            if let Some(r) = rest.strip_prefix("use ") {
                return contains_word(r, item);
            }
            return [rest, after].iter().any(|cand| {
                KINDS.iter().any(|k| {
                    cand.strip_prefix(k)
                        .is_some_and(|r| starts_with_ident(r.trim_start(), item))
                })
            });
        }
        if let Some(r) = line.strip_prefix("macro_rules!") {
            return starts_with_ident(r.trim_start(), item);
        }
        macro_row(line, item)
    })
}

/// Result of the device-API check.
pub struct MapReport {
    /// Header functions whose every mapped path was verified.
    pub covered: usize,
    /// Header functions in total.
    pub total: usize,
    /// Everything that failed, one line each.
    pub problems: Vec<String>,
}

/// Checks `map` against the header function set `api`. `module_src` returns the comment-stripped
/// source of a module (None when it does not exist); each module is loaded once.
pub fn check_map(
    api: &BTreeSet<String>,
    map: &BTreeMap<String, Vec<String>>,
    module_src: &mut dyn FnMut(&str) -> Option<String>,
) -> MapReport {
    let mut cache: BTreeMap<String, Option<String>> = BTreeMap::new();
    let mut problems = Vec::new();
    let mut covered = 0;
    for name in api {
        let Some(paths) = map.get(name) else {
            problems.push(format!("device: no API_MAP entry for `{name}`"));
            continue;
        };
        if paths.is_empty() {
            problems.push(format!("device: `{name}` maps to an empty list"));
            continue;
        }
        let mut ok = true;
        for p in paths {
            let (module, item) = match split_path(p) {
                Ok(v) => v,
                Err(e) => {
                    problems.push(format!("device: `{name}`: {e}"));
                    ok = false;
                    continue;
                }
            };
            let src = cache
                .entry(module.to_string())
                .or_insert_with(|| module_src(module));
            match src.as_deref() {
                None => {
                    problems.push(format!(
                        "device: `{name}` -> `{p}`: module `{module}` has no source file"
                    ));
                    ok = false;
                }
                Some(s) if !item_exists(s, item) => {
                    problems.push(format!(
                        "device: `{name}` -> `{p}`: no item `{item}` in module `{module}`"
                    ));
                    ok = false;
                }
                Some(_) => {}
            }
        }
        if ok {
            covered += 1;
        }
    }
    for name in map.keys().filter(|n| !api.contains(*n)) {
        problems.push(format!(
            "device: stale API_MAP entry `{name}` (not in optix_device.h)"
        ));
    }
    MapReport {
        covered,
        total: api.len(),
        problems,
    }
}

/// `name` is called as `raw::name` or passed to a macro table as `... = name;` / `... = name,`.
fn raw_used(src: &str, name: &str) -> bool {
    contains_word(src, &format!("raw::{name}"))
        || src.match_indices(name).any(|(i, _)| {
            let before = src[..i].trim_end();
            let rest = &src[i + name.len()..];
            let operator_char = before
                .chars()
                .rev()
                .nth(1)
                .is_some_and(|c| "=!<>+-*/%&|^".contains(c));
            before.ends_with('=')
                && !operator_char
                && !rest.chars().next().is_some_and(is_ident)
                && rest.trim_start().starts_with([',', ';'])
        })
}

/// Result of the raw-intrinsic check.
pub struct RawReport {
    /// Intrinsics in the SDK.
    pub total: usize,
    /// Intrinsics the typed layer uses.
    pub used: usize,
    /// Unused but allowlisted in [`RAW_ONLY`].
    pub allowed: Vec<String>,
    /// Unused and not allowlisted: failures.
    pub missing: Vec<String>,
}

/// Which of the `_optix_*` intrinsics `names` the typed layer uses. `dev_src` is the comment-stripped
/// `optix-device` source, `macros_src` the macro crate's (which names its intrinsics as string
/// literals, e.g. `"call_direct_callable"`).
pub fn raw_report(names: &BTreeSet<String>, dev_src: &str, macros_src: &str) -> RawReport {
    let mut rep = RawReport {
        total: names.len(),
        used: 0,
        allowed: Vec::new(),
        missing: Vec::new(),
    };
    for full in names {
        let short = full.strip_prefix("_optix_").unwrap_or(full);
        if raw_used(dev_src, short) || contains_word(macros_src, &format!("\"{short}\"")) {
            rep.used += 1;
        } else if RAW_ONLY.iter().any(|(n, _)| *n == short) {
            rep.allowed.push(short.to_string());
        } else {
            rep.missing.push(full.clone());
        }
    }
    rep
}

/// File of a module named in `API_MAP.toml` (`macros` and `cuda_tex` are the two special names).
fn module_file(root: &Path, module: &str) -> Option<PathBuf> {
    if !module.chars().all(is_ident) {
        return None;
    }
    let dev = root.join("device");
    Some(match module {
        "macros" => dev
            .join("optix-device-macros")
            .join("src")
            .join("optix_device_macros.rs"),
        "cuda_tex" => dev
            .join("cuda-tex-device")
            .join("src")
            .join("cuda_tex_device.rs"),
        m => dev.join("optix-device").join("src").join(format!("{m}.rs")),
    })
}

fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

/// Runs the three checks; `Ok` carries the summary, `Err` the summary plus every failure.
pub fn run(root: &Path) -> Result<String, String> {
    let optix = vcv_rs::optix::detect_optix().ok_or(
        "OptiX SDK not found (vcv-rs: OPTIX_ROOT / OptiX_INSTALL_DIR / default install dir)",
    )?;
    let inc = optix.include_dir();
    let mut problems: Vec<String> = Vec::new();

    // 1. Device API.
    let api = device_api(&read(&inc.join("optix_device.h"))?);
    let map = parse_map(&read(
        &root
            .join("device")
            .join("optix-device")
            .join("API_MAP.toml"),
    )?)?;
    let mut load = |m: &str| {
        module_file(root, m)
            .and_then(|p| std::fs::read_to_string(p).ok())
            .map(|s| strip_comments(&s))
    };
    let dev = check_map(&api, &map, &mut load);
    problems.extend(dev.problems);

    // 2. Host function table.
    let fields = table_fields(&read(&inc.join("optix_function_table.h"))?)?;
    let host_src = read_tree(&root.join("crates").join("optix").join("src"));
    let used = host_refs(&host_src, &fields);
    for (name, _) in HOST_ALLOW {
        if !fields.iter().any(|f| f == name) {
            problems.push(format!(
                "host: allowlisted `{name}` is not an OptixFunctionTable field"
            ));
        }
    }
    let wrappable: Vec<&String> = fields
        .iter()
        .filter(|f| !HOST_ALLOW.iter().any(|(n, _)| *n == f.as_str()))
        .collect();
    let mut host_used = 0;
    for f in &wrappable {
        if used.contains(f.as_str()) {
            host_used += 1;
        } else {
            problems.push(format!("host: OptixFunctionTable.{f} is not used by crates/optix (no `entry!` or member access)"));
        }
    }

    // 3. Raw intrinsics.
    let raw = match nv_bindgen::device_asm::scan_optix(&inc) {
        Ok((scan, _)) => {
            let names: BTreeSet<String> = scan.intrinsics.keys().cloned().collect();
            let dev_src = strip_comments(&read_tree(
                &root.join("device").join("optix-device").join("src"),
            ));
            let macros_src = strip_comments(&read_tree(
                &root.join("device").join("optix-device-macros").join("src"),
            ));
            let rep = raw_report(&names, &dev_src, &macros_src);
            for m in &rep.missing {
                problems.push(format!(
                    "raw: `{m}` is not used as `raw::{}` by optix-device (only reachable raw)",
                    m.trim_start_matches("_optix_")
                ));
            }
            Some(rep)
        }
        Err(e) => {
            problems.push(format!("raw: scan_optix failed: {e}"));
            None
        }
    };

    let mut summary = format!(
        "optix coverage\n  device API : {}/{} header functions mapped to existing items\n  host table : {}/{} OptixFunctionTable entries used ({} reserved slots allowlisted)\n",
        dev.covered,
        dev.total,
        host_used,
        wrappable.len(),
        HOST_ALLOW.len()
    );
    match &raw {
        Some(r) => summary.push_str(&format!(
            "  raw layer  : {}/{} _optix_* intrinsics used by the typed layer ({} raw-only allowlisted)\n",
            r.used,
            r.total,
            r.allowed.len()
        )),
        None => summary.push_str("  raw layer  : not scanned\n"),
    }
    if problems.is_empty() {
        Ok(summary.trim_end().to_string())
    } else {
        Err(format!("{summary}\n{}", problems.join("\n")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "\
/// docs mention optixDecoy( and static __forceinline__ __device__ void optixInDocs(
template <typename... Payload>
static __forceinline__ __device__ void optixTrace( int a, Payload&... p );
static __forceinline__ __device__ void optixTrace( int type, int a );
static __device__ __forceinline__ unsigned long long optixGetGASPointerFromHandle( unsigned long long h );
static __forceinline__ __device__ uint4
optixTexFootprint2DLod( float x );
static __forceinline__ __device__ unsigned int optixGetPayload_7();
static __forceinline__ __device__ helper_not_optix( int x );
__forceinline__ __device__ float getRayTime() const { return optixGetRayTime(); }
static inline int optixHostSide( int x );
";

    #[test]
    fn device_parser_handles_overloads_split_lines_and_either_qualifier_order() {
        let got: Vec<String> = device_api(HEADER).into_iter().collect();
        assert_eq!(
            got,
            [
                "optixGetGASPointerFromHandle",
                "optixGetPayload_7",
                "optixTexFootprint2DLod",
                "optixTrace"
            ]
        );
    }

    const TABLE: &str = "\
typedef struct OptixFunctionTable
{
    /// See ::optixGetErrorName().
    const char* ( *optixGetErrorName )( OptixResult result );
    OptixResult ( *optixStub )( void );
    OptixResult( *optixBuild )( OptixDeviceContext context,
                                const OptixOptions* o );
    // OptixResult ( *optixCommentedOut )( void );
    OptixResult ( *stub1 )( void );
} OptixFunctionTable;
OptixResult ( *optixOutsideTheStruct )( void );
";

    #[test]
    fn table_parser_reads_only_struct_fields_in_order() {
        assert_eq!(
            table_fields(TABLE).expect("parses"),
            ["optixGetErrorName", "optixStub", "optixBuild", "stub1"]
        );
        assert!(table_fields("nothing here").is_err());
        assert!(table_fields("typedef struct OptixFunctionTable { ").is_err());
    }

    #[test]
    fn host_refs_accept_entry_calls_and_member_access_but_not_comments() {
        let fields: Vec<String> = ["optixA", "optixB", "optixC", "optixD", "optixAB"]
            .map(String::from)
            .to_vec();
        let src = "\
unsafe { entry!(ctx.table(), optixA)(ctx.raw(), f(1, 2)) }
if let Some(d) = self.ctx.table().optixB { d(1) }
// entry!(t, optixC)
/// uses `.optixD` in prose
let x = entry!(t, optixAB);
";
        let got: Vec<String> = host_refs(src, &fields).into_iter().collect();
        assert_eq!(got, ["optixA", "optixAB", "optixB"]);
    }

    #[test]
    fn map_parses_strings_and_arrays_and_rejects_duplicates() {
        let m = parse_map("a = \"x::y\"\nb = [\"x::y\", \"z::w::<3>\"]\n").expect("parses");
        assert_eq!(m["a"], ["x::y"]);
        assert_eq!(m["b"], ["x::y", "z::w::<3>"]);
        assert!(parse_map("a = \"x::y\"\na = \"x::z\"\n").is_err());
        assert!(parse_map("a = 3\n").is_err());
    }

    #[test]
    fn paths_split_into_module_and_item() {
        assert_eq!(split_path("payload::word::<7>"), Ok(("payload", "word")));
        assert_eq!(split_path("trace::trace"), Ok(("trace", "trace")));
        assert!(split_path("trace").is_err());
        assert!(split_path("::x").is_err());
    }

    const MODULE: &str = "\
pub unsafe fn launch_index() -> [u32; 3] {}
pub const unsafe fn const_one() {}
pub use other::reexported;
macro_rules! getters { () => {} }
getters! {
    ray_tmin -> f32 = get_ray_tmin; valid: \"x\"
    triangle_vertices = get_tri, t3 -> [f32; 3];
}
fn private_one() {}
pub const IDENTITY: u32 = 1;
    m0z = m0x * 2.0;
";

    #[test]
    fn items_are_found_as_pub_fns_reexports_macros_and_macro_rows_only() {
        for ok in [
            "launch_index",
            "const_one",
            "reexported",
            "getters",
            "ray_tmin",
            "triangle_vertices",
            "IDENTITY",
        ] {
            assert!(item_exists(MODULE, ok), "{ok}");
        }
        for bad in ["private_one", "launch", "m0z", "get_ray_tmin", "nope"] {
            assert!(!item_exists(MODULE, bad), "{bad}");
        }
    }

    #[test]
    fn map_check_reports_missing_wrong_stale_and_bad_paths() {
        let api: BTreeSet<String> = ["optixA", "optixB", "optixC", "optixD", "optixE"]
            .map(String::from)
            .into();
        let mut map = BTreeMap::new();
        map.insert(
            "optixA".to_string(),
            vec!["state::launch_index".to_string()],
        );
        map.insert("optixB".to_string(), vec!["state::missing_fn".to_string()]);
        map.insert("optixC".to_string(), vec!["nomod::x".to_string()]);
        map.insert("optixD".to_string(), vec!["bad".to_string()]);
        map.insert(
            "optixStale".to_string(),
            vec!["state::launch_index".to_string()],
        );
        let mut load = |m: &str| (m == "state").then(|| MODULE.to_string());
        let rep = check_map(&api, &map, &mut load);
        assert_eq!((rep.covered, rep.total), (1, 5));
        assert_eq!(rep.problems.len(), 5, "{:?}", rep.problems);
        for needle in [
            "no item `missing_fn`",
            "module `nomod` has no source",
            "not `<module>::<item>`",
            "no API_MAP entry for `optixE`",
            "stale API_MAP entry `optixStale`",
        ] {
            assert!(
                rep.problems.iter().any(|p| p.contains(needle)),
                "{needle}: {:?}",
                rep.problems
            );
        }
    }

    #[test]
    fn raw_usage_sees_calls_macro_rows_and_macro_crate_literals() {
        let names: BTreeSet<String> = [
            "_optix_get_a",
            "_optix_get_b",
            "_optix_get_c",
            "_optix_call_x",
            "_optix_get_ab",
        ]
        .map(String::from)
        .into();
        let dev = "unsafe { raw::get_a(1) }\nx -> u32 = get_b; valid: \"y\"\nif q >= get_ab; \n";
        let mac = "expand(input, \"call_x\")";
        let rep = raw_report(&names, dev, mac);
        assert_eq!(rep.used, 3);
        assert_eq!(rep.missing, ["_optix_get_ab", "_optix_get_c"]);
        assert!(rep.allowed.is_empty());
    }
}
