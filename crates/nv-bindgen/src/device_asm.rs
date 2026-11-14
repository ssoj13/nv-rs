//! Device-side generator: turns the inline-PTX `asm` statements of an NVIDIA device header
//! (OptiX's `optix_device_impl*.h`) into Rust `ptx_asm!` wrappers for cuda-oxide.
//!
//! **What.** [`scan`] walks a header's text, finds every `asm(...)` / `asm volatile(...)`
//! statement, joins its template string literals (expanding `#param` stringification inside
//! `#define` bodies once per macro invocation), and reads its output and input constraint
//! letters. Statements that `call` an `_optix_*` function become an [`Intrinsic`]; the same
//! intrinsic reached from several C++ functions must have one shape, or scanning fails.
//! [`emit`] writes one `#[inline(always)] pub unsafe fn` per intrinsic that issues exactly the
//! header's template with the same operand order, so the PTX OptiX sees is the PTX `nvcc`
//! would produce. Volatile statements keep side effects; plain ones become
//! `options(register_only)`, as the header declares them.
//! **Why text, not clang.** The statements are uniform GCC-style inline asm; what matters is
//! the template and the constraint letters, never the C++ expressions bound to them.
//! **Used by** `device/optix-device/build.rs`; locked by the tests below on a synthetic header.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

/// A register class from an asm constraint letter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Reg {
    /// `h`: 16-bit.
    U16,
    /// `r`: 32-bit.
    U32,
    /// `l`: 64-bit.
    U64,
    /// `f`: 32-bit float.
    F32,
    /// `d`: 64-bit float.
    F64,
}

impl Reg {
    fn from_letter(c: char) -> Option<Self> {
        Some(match c {
            'h' => Reg::U16,
            'r' => Reg::U32,
            'l' => Reg::U64,
            'f' => Reg::F32,
            'd' => Reg::F64,
            _ => return None,
        })
    }

    /// The constraint letter.
    pub fn letter(self) -> char {
        match self {
            Reg::U16 => 'h',
            Reg::U32 => 'r',
            Reg::U64 => 'l',
            Reg::F32 => 'f',
            Reg::F64 => 'd',
        }
    }

    /// The Rust type.
    pub fn rust(self) -> &'static str {
        match self {
            Reg::U16 => "u16",
            Reg::U32 => "u32",
            Reg::U64 => "u64",
            Reg::F32 => "f32",
            Reg::F64 => "f64",
        }
    }
}

/// One `_optix_*` intrinsic and the exact inline PTX that reaches it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Intrinsic {
    /// The PTX function name (`_optix_get_launch_index_x`).
    pub name: String,
    /// The full asm template, whitespace inside literals preserved.
    pub template: String,
    /// Output registers in operand order.
    pub outputs: Vec<Reg>,
    /// Input registers in operand order (after the outputs).
    pub inputs: Vec<Reg>,
    /// `asm volatile` (side effects) or plain `asm` (pure).
    pub volatile: bool,
    /// How many asm statements reach this intrinsic.
    pub sites: usize,
}

/// What a header scan produced.
#[derive(Debug, Default)]
pub struct Scan {
    /// Intrinsics by name.
    pub intrinsics: BTreeMap<String, Intrinsic>,
    /// Templates of asm statements that call no `_optix_*` function (plain PTX helpers).
    pub plain_ptx: Vec<String>,
}

/// A malformed or ambiguous statement.
#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    /// The statement could not be parsed.
    #[error("{file}:{line}: {why}")]
    Parse {
        /// Header name.
        file: String,
        /// 1-based line.
        line: usize,
        /// What went wrong.
        why: String,
    },
    /// Two statements reach one intrinsic with different shapes.
    #[error("{name}: call sites disagree ({a} vs {b})")]
    Conflict {
        /// The intrinsic.
        name: String,
        /// One shape.
        a: String,
        /// The other.
        b: String,
    },
}

/// Every `_optix_*` token in `text`, with `#param` macro names expanded (the coverage universe).
pub fn names(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let b = text.as_bytes();
    let mut i = 0;
    while let Some(p) = text[i..].find("_optix_") {
        let s = i + p;
        let mut e = s + 7;
        while e < b.len() && (b[e].is_ascii_alphanumeric() || b[e] == b'_') {
            e += 1;
        }
        out.insert(text[s..e].to_owned());
        i = e;
    }
    out
}

/// Scans every asm statement of `text` (`file` names it in errors).
pub fn scan(file: &str, text: &str) -> Result<Scan, ScanError> {
    let text = text.replace("\\\r\n", " ").replace("\\\n", " ");
    let mut scan = Scan::default();
    let mut from = 0;
    while let Some((start, body_start, volatile)) = next_asm(&text, from) {
        let line = text[..start].matches('\n').count() + 1;
        let err = |why: String| ScanError::Parse {
            file: file.to_owned(),
            line,
            why,
        };
        let end = matching_paren(&text, body_start - 1)
            .ok_or_else(|| err("unbalanced asm parentheses".into()))?;
        let body = &text[body_start..end];
        from = end;
        let sections = split_sections(body);
        let (pieces, macro_param) = template_pieces(sections[0]).map_err(err)?;
        let outputs = sections
            .get(1)
            .map_or(Ok(Vec::new()), |s| constraints(s, true))
            .map_err(err)?;
        let inputs = sections
            .get(2)
            .map_or(Ok(Vec::new()), |s| constraints(s, false))
            .map_err(err)?;
        let templates = match macro_param {
            None => vec![pieces.concat_with("")],
            Some(param) => {
                let args = macro_args(&text, start, &param)
                    .ok_or_else(|| err(format!("no invocations for macro parameter #{param}")))?;
                args.iter().map(|a| pieces.concat_with(a)).collect()
            }
        };
        for template in templates {
            let Some(name) = optix_name(&template) else {
                scan.plain_ptx.push(template);
                continue;
            };
            // GCC rule: an asm statement without outputs is implicitly volatile.
            let volatile = volatile || outputs.is_empty();
            let new = Intrinsic {
                name: name.clone(),
                template,
                outputs: outputs.clone(),
                inputs: inputs.clone(),
                volatile,
                sites: 1,
            };
            match scan.intrinsics.get_mut(&name) {
                None => {
                    scan.intrinsics.insert(name, new);
                }
                Some(old) => {
                    let shape = |i: &Intrinsic| {
                        format!(
                            "{:?} -> {:?} volatile={} `{}`",
                            i.inputs,
                            i.outputs,
                            i.volatile,
                            normalize(&i.template)
                        )
                    };
                    if shape(old) != shape(&new) {
                        return Err(ScanError::Conflict {
                            name,
                            a: shape(old),
                            b: shape(&new),
                        });
                    }
                    old.sites += 1;
                }
            }
        }
    }
    Ok(scan)
}

/// Template literal pieces; a `#param` splits them.
struct Pieces(Vec<String>);

impl Pieces {
    fn concat_with(&self, arg: &str) -> String {
        self.0.join(arg)
    }
}

fn normalize(t: &str) -> String {
    t.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Finds the next `asm` / `asm volatile` statement at or after `from`: (statement start, index
/// after its opening parenthesis, volatile).
fn next_asm(text: &str, mut from: usize) -> Option<(usize, usize, bool)> {
    let b = text.as_bytes();
    while let Some(p) = text[from..].find("asm") {
        let s = from + p;
        from = s + 3;
        let word_start = s == 0 || !(b[s - 1].is_ascii_alphanumeric() || b[s - 1] == b'_');
        if !word_start
            || b.get(s + 3)
                .is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_')
        {
            continue;
        }
        let mut j = skip_ws(text, s + 3);
        let mut volatile = false;
        if text[j..].starts_with("volatile") {
            volatile = true;
            j = skip_ws(text, j + "volatile".len());
        }
        if b.get(j) == Some(&b'(') {
            return Some((s, j + 1, volatile));
        }
    }
    None
}

fn skip_ws(text: &str, mut i: usize) -> usize {
    let b = text.as_bytes();
    while i < b.len() && b[i].is_ascii_whitespace() {
        i += 1;
    }
    i
}

/// Index of the `)` matching the `(` at `open`, skipping string literals.
fn matching_paren(text: &str, open: usize) -> Option<usize> {
    let b = text.as_bytes();
    let mut depth = 0usize;
    let mut i = open;
    while i < b.len() {
        match b[i] {
            b'"' => i = end_of_string(b, i)?,
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// Index of the closing quote of the string literal starting at `i`.
fn end_of_string(b: &[u8], mut i: usize) -> Option<usize> {
    i += 1;
    while i < b.len() {
        match b[i] {
            b'\\' => i += 1,
            b'"' => return Some(i),
            _ => {}
        }
        i += 1;
    }
    None
}

/// Splits an asm body at top-level single colons (`::` belongs to C++ expressions).
fn split_sections(body: &str) -> Vec<&str> {
    let b = body.as_bytes();
    let (mut out, mut last, mut depth, mut i) = (Vec::new(), 0, 0usize, 0);
    while i < b.len() {
        match b[i] {
            b'"' => i = end_of_string(b, i).unwrap_or(b.len() - 1),
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            b':' if depth == 0 => {
                if b.get(i + 1) == Some(&b':') {
                    i += 2;
                    continue;
                }
                out.push(&body[last..i]);
                last = i + 1;
            }
            _ => {}
        }
        i += 1;
    }
    out.push(&body[last..]);
    out
}

/// The template section: string literals, optionally split by one `#param`.
fn template_pieces(section: &str) -> Result<(Pieces, Option<String>), String> {
    let b = section.as_bytes();
    let mut pieces = vec![String::new()];
    let mut param = None;
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c.is_ascii_whitespace() {
            i += 1;
        } else if c == b'"' {
            let e = end_of_string(b, i).ok_or("unterminated template literal")?;
            let lit = unescape(&section[i + 1..e])?;
            pieces.last_mut().ok_or("no template piece")?.push_str(&lit);
            i = e + 1;
        } else if c == b'#' {
            let s = i + 1;
            let mut e = s;
            while e < b.len() && (b[e].is_ascii_alphanumeric() || b[e] == b'_') {
                e += 1;
            }
            let name = &section[s..e];
            if param.as_deref().is_some_and(|p| p != name) {
                return Err("two different macro parameters in one template".into());
            }
            param = Some(name.to_owned());
            pieces.push(String::new());
            i = e;
        } else {
            return Err(format!(
                "unexpected `{}` in asm template",
                &section[i..(i + 20).min(section.len())]
            ));
        }
    }
    Ok((Pieces(pieces), param))
}

fn unescape(lit: &str) -> Result<String, String> {
    let mut out = String::with_capacity(lit.len());
    let mut chars = lit.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('"') => out.push('"'),
            Some('\\') => out.push('\\'),
            other => return Err(format!("unsupported escape \\{other:?} in asm template")),
        }
    }
    Ok(out)
}

/// Constraint letters of an operand list (`"=r"( x ), "f"( y )`).
fn constraints(section: &str, outputs: bool) -> Result<Vec<Reg>, String> {
    let b = section.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'"' {
            i += 1;
            continue;
        }
        let e = end_of_string(b, i).ok_or("unterminated constraint")?;
        let c = &section[i + 1..e];
        let letter = match (outputs, c.strip_prefix('=')) {
            (true, Some(l)) => l,
            (false, None) if !c.starts_with('+') => c,
            _ => {
                return Err(format!(
                    "unsupported constraint \"{c}\" in {} operands",
                    if outputs { "output" } else { "input" }
                ));
            }
        };
        let mut it = letter.chars();
        let reg = match (it.next(), it.next()) {
            (Some(l), None) => Reg::from_letter(l),
            _ => None,
        }
        .ok_or_else(|| format!("unsupported constraint \"{c}\""))?;
        out.push(reg);
        // Skip the bound expression `( ... )`.
        let open = skip_ws(section, e + 1);
        if b.get(open) != Some(&b'(') {
            return Err(format!("constraint \"{c}\" without an operand expression"));
        }
        i = matching_paren(section, open).ok_or("unbalanced operand expression")? + 1;
    }
    Ok(out)
}

/// The arguments the `#define` enclosing `at` is invoked with (`MACRO( 3 )` -> `3`).
fn macro_args(text: &str, at: usize, param: &str) -> Option<Vec<String>> {
    let def = text[..at].rfind("#define")?;
    let head = &text[def + "#define".len()..];
    let head = head.trim_start();
    let name_end = head.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))?;
    let name = &head[..name_end];
    let params = head[name_end..].trim_start().strip_prefix('(')?;
    let params = &params[..params.find(')')?];
    if params.split(',').map(str::trim).collect::<Vec<_>>() != [param] {
        return None;
    }
    let mut args = Vec::new();
    let mut from = def + 1;
    while let Some(p) = text[from..].find(name) {
        let s = from + p;
        from = s + name.len();
        let rest = text[from..].trim_start();
        let Some(rest) = rest.strip_prefix('(') else {
            continue;
        };
        let Some(close) = rest.find(')') else {
            continue;
        };
        let arg = rest[..close].trim();
        if arg != param && !arg.is_empty() {
            args.push(arg.to_owned());
        }
    }
    (!args.is_empty()).then_some(args)
}

/// The `_optix_*` function a template calls.
fn optix_name(template: &str) -> Option<String> {
    names(template).into_iter().next()
}

/// Rust source of one wrapper per intrinsic: `pub unsafe fn <name without _optix_>(...)`.
/// `ptx_asm` is the path of the `ptx_asm!` macro at the include site.
pub fn emit(scan: &Scan, header: &str, ptx_asm: &str) -> String {
    let mut s = String::new();
    for i in scan.intrinsics.values() {
        let fname = i.name.trim_start_matches("_optix_");
        let params: Vec<String> = i
            .inputs
            .iter()
            .enumerate()
            .map(|(k, r)| format!("a{k}: {}", r.rust()))
            .collect();
        let ret = match i.outputs.as_slice() {
            [] => String::new(),
            [one] => format!(" -> {}", one.rust()),
            many if many.iter().all(|r| *r == many[0]) => {
                format!(" -> [{}; {}]", many[0].rust(), many.len())
            }
            many => format!(
                " -> ({})",
                many.iter().map(|r| r.rust()).collect::<Vec<_>>().join(", ")
            ),
        };
        let _ = writeln!(
            s,
            "/// `{}` ({header}, {} call site{}).",
            i.name,
            i.sites,
            if i.sites == 1 { "" } else { "s" }
        );
        let _ = writeln!(
            s,
            "///\n/// # Safety\n///\n/// Valid only in the OptiX program types the OptiX documentation lists for this intrinsic."
        );
        let _ = writeln!(
            s,
            "#[inline(always)]\n#[allow(clippy::too_many_arguments)]\npub unsafe fn {fname}({}){ret} {{",
            params.join(", ")
        );
        for (k, r) in i.outputs.iter().enumerate() {
            let _ = writeln!(s, "    let o{k}: {};", r.rust());
        }
        let mut ops: Vec<String> = i
            .outputs
            .iter()
            .enumerate()
            .map(|(k, r)| format!("out(\"={}\") o{k}", r.letter()))
            .collect();
        ops.extend(
            i.inputs
                .iter()
                .enumerate()
                .map(|(k, r)| format!("in(\"{}\") a{k}", r.letter())),
        );
        if !i.volatile {
            ops.push("options(register_only)".into());
        }
        let _ = writeln!(
            s,
            "    // SAFETY: the header's own inline PTX for this intrinsic; the caller upholds the program-type contract."
        );
        let _ = writeln!(
            s,
            "    unsafe {{ {ptx_asm}!({:?}, {}) }};",
            i.template,
            ops.join(", ")
        );
        match i.outputs.len() {
            0 => {}
            1 => s.push_str("    o0\n"),
            n => {
                let list = (0..n)
                    .map(|k| format!("o{k}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                let all_same = i.outputs.iter().all(|r| *r == i.outputs[0]);
                let _ = writeln!(
                    s,
                    "    {}{list}{}",
                    if all_same { "[" } else { "(" },
                    if all_same { "]" } else { ")" }
                );
            }
        }
        s.push_str("}\n\n");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = r#"
static __forceinline__ __device__ unsigned int optixGetLaunchIndexX()
{
    unsigned int u0;
    asm( "call (%0), _optix_get_launch_index_x, ();" : "=r"( u0 ) : );
    return u0;
}
static __forceinline__ __device__ void optixSetPayload_0( unsigned int p )
{
    asm volatile( "call _optix_set_payload, (%0, %1);" : : "r"( 0 ), "r"( p ) : );
}
static __forceinline__ __device__ void optixSetPayload_1( unsigned int p )
{
    asm volatile( "call _optix_set_payload, (%0, %1);" : : "r"( 1 ), "r"( p ) : );
}
static __forceinline__ __device__ void verts( OptixTraversableHandle gas, float time, float3 data[3] )
{
    asm( "call (%0, %1, %2), _optix_get_tri, "
         "(%3, %4);"
         : "=f"( data[0].x ), "=f"( data[0].y ), "=r"( n )
         : "l"( gas ), "f"( VecT::size ) );
}
#define BODY( which )                                     \
    unsigned int ret;                                     \
    asm( "call (%0), _optix_get_attribute_" #which ", ();" : "=r"( ret ) : ); \
    return ret;
unsigned int a0() { BODY( 0 ); }
unsigned int a1() { BODY( 1 ); }
#undef BODY
static void helper() { asm volatile( "cvta.to.global.u64 %0, %1;" : "=l"( ptr ) : "l"( addr ) ); }
static void terminate() { asm volatile( "call _optix_terminate_ray, ();" ); }
static void acc() { asm( "call (), _optix_reduce, (%0);" : : "l"( p ) ); }
"#;

    #[test]
    fn scans_shapes_macros_and_plain_ptx() {
        let s = scan("fake.h", HEADER).expect("scan");
        let names: Vec<_> = s.intrinsics.keys().cloned().collect();
        assert_eq!(
            names,
            [
                "_optix_get_attribute_0",
                "_optix_get_attribute_1",
                "_optix_get_launch_index_x",
                "_optix_get_tri",
                "_optix_reduce",
                "_optix_set_payload",
                "_optix_terminate_ray"
            ]
        );
        let tri = &s.intrinsics["_optix_get_tri"];
        assert_eq!(tri.template, "call (%0, %1, %2), _optix_get_tri, (%3, %4);");
        assert_eq!(tri.outputs, [Reg::F32, Reg::F32, Reg::U32]);
        assert_eq!(tri.inputs, [Reg::U64, Reg::F32]);
        assert!(!tri.volatile);
        let set = &s.intrinsics["_optix_set_payload"];
        assert!(set.volatile);
        assert_eq!(set.sites, 2);
        assert!(
            s.intrinsics["_optix_reduce"].volatile,
            "no outputs: implicitly volatile"
        );
        assert_eq!(
            s.intrinsics["_optix_get_attribute_1"].template,
            "call (%0), _optix_get_attribute_1, ();"
        );
        assert_eq!(s.plain_ptx, ["cvta.to.global.u64 %0, %1;"]);
    }

    #[test]
    fn conflicting_shapes_fail() {
        let h = r#"asm( "call (%0), _optix_x, ();" : "=r"( a ) : ); asm( "call (%0), _optix_x, ();" : "=f"( a ) : );"#;
        assert!(matches!(scan("c.h", h), Err(ScanError::Conflict { .. })));
    }

    #[test]
    fn names_cover_macro_bases_and_instances() {
        let n = names(HEADER);
        assert!(n.contains("_optix_get_attribute_"));
        assert!(n.contains("_optix_terminate_ray"));
    }

    #[test]
    fn emits_wrappers_with_matching_types() {
        let s = scan("fake.h", HEADER).expect("scan");
        let src = emit(&s, "fake.h", "ptx_asm");
        assert!(
            src.contains("pub unsafe fn get_tri(a0: u64, a1: f32) -> (f32, f32, u32) {"),
            "{src}"
        );
        assert!(src.contains("pub unsafe fn get_launch_index_x() -> u32 {"));
        assert!(src.contains("pub unsafe fn set_payload(a0: u32, a1: u32) {"));
        assert!(
            src.contains(r#"out("=r") o0, options(register_only)"#),
            "pure getters are register_only"
        );
        assert!(
            src.contains(r#"in("r") a1) }"#),
            "volatile statements keep side effects"
        );
        syn::parse_file(&src.replace("ptx_asm!", "dummy!")).expect("valid Rust");
    }
}

/// The OptiX device headers the wrappers come from, relative to the SDK's `include/`.
pub const OPTIX_DEVICE_HEADERS: &[&str] = &[
    "internal/optix_device_impl.h",
    "internal/optix_device_impl_transformations.h",
    "internal/optix_device_impl_coop_vec.h",
];

/// Scans the OptiX device headers under `include_dir` into one [`Scan`] and checks coverage:
/// every `_optix_*` name in them must have a wrapper (macro bases such as
/// `_optix_get_attribute_` count as covered by their instances).
pub fn scan_optix(include_dir: &std::path::Path) -> Result<(Scan, Vec<(String, String)>), String> {
    let mut all = Scan::default();
    let mut hashes = Vec::new();
    let mut universe = BTreeSet::new();
    for h in OPTIX_DEVICE_HEADERS {
        let path = include_dir.join(h);
        let text =
            std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        use sha2::Digest;
        hashes.push((
            h.to_string(),
            format!("{:x}", sha2::Sha256::digest(text.as_bytes())),
        ));
        universe.extend(names(&text));
        let s = scan(h, &text).map_err(|e| e.to_string())?;
        all.plain_ptx.extend(s.plain_ptx);
        for (name, i) in s.intrinsics {
            if let Some(old) = all.intrinsics.get(&name)
                && (old.inputs != i.inputs || old.outputs != i.outputs)
            {
                return Err(format!("{name}: headers disagree on its shape"));
            }
            all.intrinsics.entry(name).or_insert(i);
        }
    }
    let missing: Vec<&String> = universe
        .iter()
        .filter(|n| !all.intrinsics.contains_key(*n))
        .filter(|n| !(n.ends_with('_') && all.intrinsics.keys().any(|k| k.starts_with(n.as_str()))))
        .collect();
    if !missing.is_empty() {
        return Err(format!("OptiX intrinsics without a wrapper: {missing:?}"));
    }
    Ok((all, hashes))
}

#[cfg(test)]
mod optix_sdk {
    /// T1: the installed OptiX SDK's device headers scan cleanly and every intrinsic is wrapped.
    #[test]
    #[ignore = "needs the OptiX SDK"]
    fn every_optix_intrinsic_is_wrapped() {
        let optix = vcv_rs::optix::detect_optix().expect("OptiX SDK");
        let (scan, _) = super::scan_optix(&optix.include_dir()).expect("full coverage");
        let n = scan.intrinsics.len();
        eprintln!("{n} intrinsics; plain PTX helpers: {:?}", scan.plain_ptx);
        let widest = scan
            .intrinsics
            .values()
            .map(|i| i.inputs.len())
            .max()
            .unwrap_or(0);
        eprintln!("widest input list: {widest}");
        assert!(n >= 190, "{n}");
        syn::parse_file(&super::emit(&scan, "optix", "ptx_asm").replace("ptx_asm!", "dummy!"))
            .expect("valid Rust");
    }
}
