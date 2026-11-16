//! Single-source-of-truth (SSOT) Python `.pyi` stub and Starlight reference generator.
//!
//! **What.** `cargo xtask py-stubs generate` parses `python/nv-py/src/lib.rs`, every registered
//! `python/nv-py/src/<mod>.rs` module (via `syn`), and `python/nv-py/python/nv_rs/__init__.py`,
//! then deterministically renders:
//! - `python/nv-py/python/nv_rs/__init__.pyi` — PEP 561 type stub for the `nv_rs` package;
//! - `docs/src/content/docs/python/04-modules-reference.md` — Starlight API reference page.
//!
//! `cargo xtask py-stubs check` builds the exact same outputs in memory and fails if either file
//! on disk is missing or out of date, or if `python/nv-py/python/nv_rs/__init__.py` (`from ._nv_rs`
//! imports and `__all__`) has drifted from the Rust module registrations.
//!
//! **Why.** Hand-maintained `.pyi` and Markdown tables inevitably drift from PyO3 `#[pyclass]`,
//! `#[pymethods]`, and `#[pyfunction]` signatures (wrong parameter names, missing `@staticmethod`,
//! wrong `__exit__` return types, `list[int]` instead of `Sequence[int]`, omitted properties).
//! Extracting signatures directly from the Rust AST keeps stubs and docs 100% synchronized.
//!
//! **How.** A two-pass pipeline first collects raw submodule registrations, exceptions, classes,
//! merged `#[pymethods]` blocks, and `#[pyfunction]` items across all registered Rust modules to
//! populate a crate-wide `RustIdent -> (submod, PyClassName)` registry, then resolves parameter
//! defaults, doc overrides (`/// @py-return: ...` and `/// @py-type <arg>: ...`), and context-aware
//! Rust-to-Python types (`Sequence[T]` in input position vs `list[T]` in return position).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use syn::ext::IdentExt;
use syn::punctuated::Punctuated;

const PYI_REL_PATH: &str = "python/nv-py/python/nv_rs/__init__.pyi";
const DOCS_REL_PATH: &str = "docs/src/content/docs/python/04-modules-reference.md";

/// Thematic section layout for `04-modules-reference.md`.
const DOC_SECTIONS: &[(&str, &[&str])] = &[
    ("Core & Diagnostics", &["errors", "diag", "core"]),
    (
        "System, Compilation, Compression, Imaging",
        &["nvml", "nvrtc", "nvcomp", "nvjpeg"],
    ),
    ("Video & Optical Flow", &["nvenc", "nvdec", "nvofa"]),
    (
        "Dense/Sparse Math, FFT, RNG, AI",
        &["cublas", "cufft", "curand", "cusparse", "cusolver", "cudnn"],
    ),
    ("Graphics, Ray Tracing, NPP, DLSS", &["optix", "npp", "ngx"]),
    (
        "Profiling, Driver Extensions, WGPU Interop",
        &["cupti", "cuda_ext", "wgpu"],
    ),
    ("Platform-Specific", &["nccl", "cufile", "nvapi"]),
];

/// Generates `python/nv-py/python/nv_rs/__init__.pyi` and
/// `docs/src/content/docs/python/04-modules-reference.md` from the `nv-py` sources under `root`.
pub fn generate(root: &Path) -> Result<String, String> {
    let model = build_api_model(root)?;
    let pyi_text = render_pyi(&model);
    let docs_text = render_docs_md(&model);

    let pyi_path = root.join(PYI_REL_PATH);
    let docs_path = root.join(DOCS_REL_PATH);

    write_file_utf8(&pyi_path, &pyi_text)?;
    write_file_utf8(&docs_path, &docs_text)?;

    Ok(format!(
        "generated {} ({} submodules) and {}",
        pyi_path.display(),
        model.submodules.len(),
        docs_path.display()
    ))
}

/// Verifies that `__init__.py`, `__init__.pyi`, and `04-modules-reference.md` under `root` match
/// the `nv-py` Rust AST without modifying any files on disk.
pub fn check(root: &Path) -> Result<String, String> {
    let model = build_api_model(root)?;
    let expected_pyi = render_pyi(&model);
    let expected_docs = render_docs_md(&model);

    let mut problems = Vec::new();
    check_file_matches(
        &root.join(PYI_REL_PATH),
        PYI_REL_PATH,
        &expected_pyi,
        &mut problems,
    );
    check_file_matches(
        &root.join(DOCS_REL_PATH),
        DOCS_REL_PATH,
        &expected_docs,
        &mut problems,
    );

    if problems.is_empty() {
        Ok(format!(
            "py-stubs check: ok ({} submodules verified)",
            model.submodules.len()
        ))
    } else {
        Err(format!("py-stubs check failed:\n{}", problems.join("\n")))
    }
}

fn check_file_matches(path: &Path, label: &str, expected: &str, problems: &mut Vec<String>) {
    match std::fs::read_to_string(path) {
        Ok(on_disk) if normalize_newlines(&on_disk) == expected => {}
        Ok(_) => problems.push(format!(
            "{label} is stale: run `cargo xtask py-stubs generate`"
        )),
        Err(_) => problems.push(format!(
            "{label} is missing: run `cargo xtask py-stubs generate`"
        )),
    }
}

fn normalize_newlines(s: &str) -> String {
    if s.contains('\r') {
        s.replace("\r\n", "\n")
    } else {
        s.to_owned()
    }
}

fn read_file_utf8(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn write_file_utf8(path: &Path, content: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    std::fs::write(path, content).map_err(|e| format!("{}: {e}", path.display()))
}

// ============================================================================
// Resolved API Model
// ============================================================================

#[derive(Debug)]
struct ApiModel {
    top_attrs: Vec<(String, String)>,
    submodules: Vec<SubmoduleSpec>,
    top_py_fns: Vec<PurePyFn>,
}

#[derive(Debug)]
struct SubmoduleSpec {
    name: String,
    summary: String,
    constants: Vec<(String, String)>,
    exceptions: Vec<ExceptionSpec>,
    classes: Vec<ClassSpec>,
    functions: Vec<FunctionSpec>,
}

#[derive(Debug)]
struct ExceptionSpec {
    name: String,
    base: String,
    doc: Option<String>,
}

#[derive(Debug)]
struct ClassSpec {
    py_name: String,
    doc: Option<String>,
    methods: Vec<MethodSpec>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum MethodKind {
    New,
    Instance,
    Static,
    Getter(String),
    Setter(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParamSpec {
    name: String,
    ty: Option<String>,
    default: Option<String>,
}

#[derive(Debug)]
struct MethodSpec {
    kind: MethodKind,
    py_name: String,
    params: Vec<ParamSpec>,
    ret_ty: String,
}

#[derive(Debug)]
struct FunctionSpec {
    py_name: String,
    params: Vec<ParamSpec>,
    raw_params: Option<String>,
    ret_ty: String,
}

#[derive(Clone, Debug)]
struct PurePyFn {
    name: String,
    params_raw: String,
    ret_ty: String,
    doc: Option<String>,
}

#[derive(Clone, Debug)]
struct MonkeyPatch {
    submod: String,
    attr_name: String,
    target_fn: String,
}

#[derive(Default, Debug)]
struct InitPyInfo {
    ext_imports: BTreeSet<String>,
    dunder_all: BTreeSet<String>,
    top_fns: Vec<PurePyFn>,
    monkey_patches: Vec<MonkeyPatch>,
}

// ============================================================================
// Raw AST Intermediate Structures (Pass 1)
// ============================================================================

struct LibSpecAst {
    top_attrs: Vec<(String, String)>,
    registered_mods: Vec<String>,
}

struct RawModule {
    file_doc: Option<String>,
    exceptions: BTreeMap<String, RawException>,
    classes: BTreeMap<String, RawClass>,
    impls: BTreeMap<String, Vec<RawMethod>>,
    functions: BTreeMap<String, RawFunction>,
    submodules: Vec<RawSubmodule>,
}

struct RawException {
    base: String,
    doc: Option<String>,
}

struct RawClass {
    py_name: String,
    explicit_submod: Option<String>,
    doc: Option<String>,
}

#[derive(Default)]
struct RawSubmodule {
    name: String,
    constants: Vec<(String, String)>,
    exceptions: Vec<(String, String)>, // (exposed_py_name, rust_exc_ident)
    classes: Vec<String>,              // rust_struct_ident in registration order
    functions: Vec<String>,            // rust_fn_ident in registration order
}

#[derive(Clone)]
enum SigItem {
    PosOnlySep,
    KwOnlySep,
    VarArgs(String),
    KwArgs(String),
    Param {
        name: String,
        default: Option<String>,
    },
}

#[derive(Default)]
struct ParsedItemAttrs {
    py_return_override: Option<String>,
    py_param_overrides: BTreeMap<String, String>,
    name_override: Option<String>,
    signature: Option<Vec<SigItem>>,
    is_new: bool,
    is_staticmethod: bool,
    getter: Option<Option<String>>,
    setter: Option<Option<String>>,
}

struct RawMethod {
    rust_name: String,
    attrs: ParsedItemAttrs,
    inputs: Vec<(String, syn::Type)>,
    output: Option<syn::Type>,
}

struct RawFunction {
    rust_name: String,
    attrs: ParsedItemAttrs,
    inputs: Vec<(String, syn::Type)>,
    output: Option<syn::Type>,
}

// ============================================================================
// Pipeline Orchestration
// ============================================================================

fn build_api_model(root: &Path) -> Result<ApiModel, String> {
    let nv_py_dir = root.join("python").join("nv-py");
    let src_dir = nv_py_dir.join("src");
    let lib_rs_path = src_dir.join("lib.rs");
    let init_py_path = nv_py_dir.join("python").join("nv_rs").join("__init__.py");

    let lib_src = read_file_utf8(&lib_rs_path)?;
    let lib_ast = parse_lib_rs(&lib_src, &lib_rs_path)?;

    let mut raw_modules: Vec<(String, RawModule)> =
        Vec::with_capacity(lib_ast.registered_mods.len());
    for mod_name in &lib_ast.registered_mods {
        let mod_path = src_dir.join(format!("{mod_name}.rs"));
        let mod_src = read_file_utf8(&mod_path)?;
        let raw_mod = parse_module_rs(&mod_src, &mod_path)?;
        raw_modules.push((mod_name.clone(), raw_mod));
    }

    let init_src = read_file_utf8(&init_py_path)?;
    let init_info = parse_init_py(&init_src);

    assemble_model(lib_ast, raw_modules, init_info)
}

fn assemble_model(
    lib_ast: LibSpecAst,
    raw_modules: Vec<(String, RawModule)>,
    init_info: InitPyInfo,
) -> Result<ApiModel, String> {
    // Pass 1b: Build global `RustStructIdent -> (submod_name, py_class_name)` registry.
    let mut class_registry: BTreeMap<String, (String, String)> = BTreeMap::new();
    for (mod_name, raw_mod) in &raw_modules {
        for submod in &raw_mod.submodules {
            for rust_cls in &submod.classes {
                let raw_cls = raw_mod
                    .classes
                    .get(rust_cls)
                    .or_else(|| {
                        raw_modules
                            .iter()
                            .find_map(|(_, m)| m.classes.get(rust_cls))
                    })
                    .ok_or_else(|| {
                        format!(
                            "module `{mod_name}` submodule `{}` registers unknown #[pyclass] `{rust_cls}`",
                            submod.name
                        )
                    })?;
                if let Some(ref exp_sub) = raw_cls.explicit_submod
                    && exp_sub != &submod.name
                {
                    return Err(format!(
                        "#[pyclass] `{rust_cls}` declares module `nv_rs.{exp_sub}`, but is registered in `{}`",
                        submod.name
                    ));
                }
                class_registry.insert(
                    rust_cls.clone(),
                    (submod.name.clone(), raw_cls.py_name.clone()),
                );
            }
        }
    }

    // Pass 2: Resolve all submodules, classes, methods, and functions.
    let pure_fn_map: BTreeMap<&str, &PurePyFn> = init_info
        .top_fns
        .iter()
        .map(|f| (f.name.as_str(), f))
        .collect();

    let mut submodules = Vec::new();
    for (mod_name, raw_mod) in &raw_modules {
        let multi_submod = raw_mod.submodules.len() > 1;
        for raw_sub in &raw_mod.submodules {
            let summary =
                resolve_submodule_summary(&raw_sub.name, raw_mod.file_doc.as_deref(), multi_submod);

            let mut exceptions = Vec::with_capacity(raw_sub.exceptions.len());
            for (py_exc_name, rust_exc_ident) in &raw_sub.exceptions {
                let raw_exc = raw_mod.exceptions.get(rust_exc_ident).ok_or_else(|| {
                    format!(
                        "module `{mod_name}` submodule `{}` registers unknown exception `{rust_exc_ident}`",
                        raw_sub.name
                    )
                })?;
                exceptions.push(ExceptionSpec {
                    name: py_exc_name.clone(),
                    base: raw_exc.base.clone(),
                    doc: raw_exc.doc.clone(),
                });
            }

            let mut classes = Vec::with_capacity(raw_sub.classes.len());
            for rust_cls in &raw_sub.classes {
                let raw_cls = raw_mod
                    .classes
                    .get(rust_cls)
                    .or_else(|| {
                        raw_modules
                            .iter()
                            .find_map(|(_, m)| m.classes.get(rust_cls))
                    })
                    .ok_or_else(|| format!("missing #[pyclass] `{rust_cls}`"))?;
                let raw_methods = raw_mod
                    .impls
                    .get(rust_cls)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]);
                let ctx = TypeCtx {
                    class_registry: &class_registry,
                    self_class: Some((&raw_sub.name, &raw_cls.py_name)),
                };
                let mut methods = Vec::new();
                for rm in raw_methods {
                    if let Some(ms) = resolve_method(rm, &ctx)? {
                        methods.push(ms);
                    }
                }
                classes.push(ClassSpec {
                    py_name: raw_cls.py_name.clone(),
                    doc: raw_cls.doc.clone(),
                    methods,
                });
            }

            let ctx = TypeCtx {
                class_registry: &class_registry,
                self_class: None,
            };
            let mut functions = Vec::with_capacity(raw_sub.functions.len());
            for rust_fn in &raw_sub.functions {
                let rf = raw_mod.functions.get(rust_fn).ok_or_else(|| {
                    format!(
                        "module `{mod_name}` submodule `{}` wraps unknown #[pyfunction] `{rust_fn}`",
                        raw_sub.name
                    )
                })?;
                functions.push(resolve_function(rf, &ctx)?);
            }

            for mp in &init_info.monkey_patches {
                if mp.submod == raw_sub.name
                    && let Some(pf) = pure_fn_map.get(mp.target_fn.as_str())
                {
                    functions.push(FunctionSpec {
                        py_name: mp.attr_name.clone(),
                        params: Vec::new(),
                        raw_params: Some(pf.params_raw.clone()),
                        ret_ty: pf.ret_ty.clone(),
                    });
                }
            }

            submodules.push(SubmoduleSpec {
                name: raw_sub.name.clone(),
                summary,
                constants: raw_sub.constants.clone(),
                exceptions,
                classes,
                functions,
            });
        }
    }

    // Verify `__init__.py` exports stay synchronized with Rust submodules and top-level attrs.
    verify_init_py_sync(&lib_ast.top_attrs, &submodules, &init_info)?;

    Ok(ApiModel {
        top_attrs: lib_ast.top_attrs,
        submodules,
        top_py_fns: init_info.top_fns,
    })
}

fn verify_init_py_sync(
    top_attrs: &[(String, String)],
    submodules: &[SubmoduleSpec],
    init_info: &InitPyInfo,
) -> Result<(), String> {
    let mut expected_ext: BTreeSet<String> = submodules.iter().map(|s| s.name.clone()).collect();
    for (attr, _) in top_attrs {
        expected_ext.insert(attr.clone());
    }

    let missing_imports: Vec<&String> = expected_ext.difference(&init_info.ext_imports).collect();
    let extra_imports: Vec<&String> = init_info.ext_imports.difference(&expected_ext).collect();
    if !missing_imports.is_empty() || !extra_imports.is_empty() {
        return Err(format!(
            "__init__.py `from ._nv_rs import (...)` out of sync: missing={missing_imports:?}, extra={extra_imports:?}"
        ));
    }

    let mut expected_all = expected_ext;
    for f in &init_info.top_fns {
        expected_all.insert(f.name.clone());
    }
    let missing_all: Vec<&String> = expected_all.difference(&init_info.dunder_all).collect();
    let extra_all: Vec<&String> = init_info.dunder_all.difference(&expected_all).collect();
    if !missing_all.is_empty() || !extra_all.is_empty() {
        return Err(format!(
            "__init__.py `__all__` out of sync: missing={missing_all:?}, extra={extra_all:?}"
        ));
    }

    Ok(())
}

// ============================================================================
// Rust AST Parsing (`lib.rs` and `<mod>.rs`)
// ============================================================================

fn parse_lib_rs(src: &str, path: &Path) -> Result<LibSpecAst, String> {
    let file = syn::parse_file(src).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut top_attrs = Vec::new();
    let mut registered_mods = Vec::new();

    for item in &file.items {
        let syn::Item::Fn(func) = item else {
            continue;
        };
        if !func.attrs.iter().any(|a| a.path().is_ident("pymodule")) {
            continue;
        }
        for stmt in &func.block.stmts {
            let syn::Stmt::Expr(expr, _) = stmt else {
                continue;
            };
            let inner = unwrap_try(expr);
            match inner {
                syn::Expr::MethodCall(mc) if mc.method == "add" && mc.args.len() == 2 => {
                    if let Some(attr_name) = extract_str_lit(&mc.args[0]) {
                        let ty = infer_const_py_type(&mc.args[1]);
                        top_attrs.push((attr_name, ty.to_owned()));
                    }
                }
                syn::Expr::Call(call) => {
                    if let syn::Expr::Path(ep) = &*call.func
                        && ep.path.segments.len() == 2
                        && ep.path.segments[1].ident == "register"
                    {
                        registered_mods.push(unraw_ident(&ep.path.segments[0].ident));
                    }
                }
                _ => {}
            }
        }
    }

    if registered_mods.is_empty() {
        return Err(format!(
            "{}: no `<mod>::register(...)` calls found inside #[pymodule]",
            path.display()
        ));
    }

    Ok(LibSpecAst {
        top_attrs,
        registered_mods,
    })
}

fn parse_module_rs(src: &str, path: &Path) -> Result<RawModule, String> {
    let file = syn::parse_file(src).map_err(|e| format!("{}: {e}", path.display()))?;
    let file_doc = extract_doc_summary(&parse_doc_lines(&file.attrs).0);

    let mut exceptions = BTreeMap::new();
    let mut classes = BTreeMap::new();
    let mut impls: BTreeMap<String, Vec<RawMethod>> = BTreeMap::new();
    let mut functions = BTreeMap::new();
    let mut helper_fns: BTreeMap<String, &syn::ItemFn> = BTreeMap::new();

    for item in &file.items {
        match item {
            syn::Item::Macro(item_mac) => {
                if path_ends_with(&item_mac.mac.path, "create_exception") {
                    let (exc_name, raw_exc) = parse_create_exception(item_mac, path)?;
                    exceptions.insert(exc_name, raw_exc);
                }
            }
            syn::Item::Struct(item_struct) => {
                if let Some(raw_cls) = parse_pyclass_struct(item_struct, path)? {
                    classes.insert(unraw_ident(&item_struct.ident), raw_cls);
                }
            }
            syn::Item::Impl(item_impl) => {
                if item_impl
                    .attrs
                    .iter()
                    .any(|a| a.path().is_ident("pymethods"))
                    && let Some(struct_ident) = extract_type_last_ident(&item_impl.self_ty)
                {
                    let entry = impls.entry(struct_ident.clone()).or_default();
                    for impl_item in &item_impl.items {
                        if let syn::ImplItem::Fn(method) = impl_item {
                            entry.push(parse_raw_method(method, &struct_ident, path)?);
                        }
                    }
                }
            }
            syn::Item::Fn(item_fn) => {
                let fn_name = unraw_ident(&item_fn.sig.ident);
                if item_fn
                    .attrs
                    .iter()
                    .any(|a| a.path().is_ident("pyfunction"))
                {
                    let rf = parse_raw_function(item_fn, path)?;
                    functions.insert(fn_name.clone(), rf);
                }
                helper_fns.insert(fn_name, item_fn);
            }
            _ => {}
        }
    }

    let register_fn = helper_fns.get("register").ok_or_else(|| {
        format!(
            "{}: missing `pub fn register(...)` entry point",
            path.display()
        )
    })?;

    let mut submodules = Vec::new();
    let mut visited = BTreeSet::new();
    collect_submodules_from_fn(
        register_fn,
        &helper_fns,
        &mut visited,
        &mut submodules,
        path,
    )?;

    if submodules.is_empty() {
        return Err(format!(
            "{}: no submodule registrations found in `register`",
            path.display()
        ));
    }

    Ok(RawModule {
        file_doc,
        exceptions,
        classes,
        impls,
        functions,
        submodules,
    })
}

fn collect_submodules_from_fn(
    func: &syn::ItemFn,
    helper_fns: &BTreeMap<String, &syn::ItemFn>,
    visited: &mut BTreeSet<String>,
    out: &mut Vec<RawSubmodule>,
    path: &Path,
) -> Result<(), String> {
    let fn_name = unraw_ident(&func.sig.ident);
    if !visited.insert(fn_name) {
        return Ok(());
    }
    collect_submodules_from_stmts(&func.block.stmts, helper_fns, visited, out, path)
}

fn collect_submodules_from_stmts(
    stmts: &[syn::Stmt],
    helper_fns: &BTreeMap<String, &syn::ItemFn>,
    visited: &mut BTreeSet<String>,
    out: &mut Vec<RawSubmodule>,
    path: &Path,
) -> Result<(), String> {
    // Supports both `py_register_submodule!(py, parent, "name", |m| { ... })` AND
    // direct `let m = PyModule::new(py, "name")?; m.add_*(...)?;`.
    let mut local_submods: BTreeMap<String, usize> = BTreeMap::new();

    for stmt in stmts {
        match stmt {
            syn::Stmt::Macro(stmt_mac) => {
                if path_ends_with(&stmt_mac.mac.path, "py_register_submodule") {
                    out.push(parse_py_register_submodule_macro(&stmt_mac.mac, path)?);
                }
            }
            syn::Stmt::Local(local) => {
                if let Some(var_name) = extract_pat_ident(&local.pat)
                    && let Some(init) = &local.init
                {
                    let inner = unwrap_try(&init.expr);
                    if let Some(submod_name) = extract_pymodule_new_name(inner) {
                        let idx = out.len();
                        out.push(RawSubmodule {
                            name: submod_name,
                            ..RawSubmodule::default()
                        });
                        local_submods.insert(var_name, idx);
                    }
                }
            }
            syn::Stmt::Expr(expr, _) => {
                let inner = unwrap_try(expr);
                match inner {
                    syn::Expr::Macro(expr_mac)
                        if path_ends_with(&expr_mac.mac.path, "py_register_submodule") =>
                    {
                        out.push(parse_py_register_submodule_macro(&expr_mac.mac, path)?);
                    }
                    syn::Expr::Call(call) => {
                        if let syn::Expr::Path(ep) = &*call.func
                            && ep.path.segments.len() == 1
                        {
                            let callee = unraw_ident(&ep.path.segments[0].ident);
                            if let Some(helper) = helper_fns.get(&callee) {
                                collect_submodules_from_fn(helper, helper_fns, visited, out, path)?;
                            }
                        }
                    }
                    syn::Expr::MethodCall(mc) => {
                        if let syn::Expr::Path(rp) = &*mc.receiver
                            && rp.path.segments.len() == 1
                        {
                            let recv_name = unraw_ident(&rp.path.segments[0].ident);
                            if let Some(&idx) = local_submods.get(&recv_name) {
                                record_submodule_method_call(&mut out[idx], mc, path)?;
                            }
                        }
                    }
                    syn::Expr::Block(eb) => {
                        collect_submodules_from_stmts(
                            &eb.block.stmts,
                            helper_fns,
                            visited,
                            out,
                            path,
                        )?;
                    }
                    _ => {}
                }
            }
            syn::Stmt::Item(_) => {}
        }
    }

    Ok(())
}

fn parse_py_register_submodule_macro(
    mac: &syn::Macro,
    path: &Path,
) -> Result<RawSubmodule, String> {
    let args = mac
        .parse_body_with(Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated)
        .map_err(|e| {
            format!(
                "{}: invalid py_register_submodule! invocation: {e}",
                path.display()
            )
        })?;
    if args.len() != 4 {
        return Err(format!(
            "{}: expected 4 arguments in py_register_submodule!, got {}",
            path.display(),
            args.len()
        ));
    }
    let submod_name = extract_str_lit(&args[2]).ok_or_else(|| {
        format!(
            "{}: third argument of py_register_submodule! must be a string literal",
            path.display()
        )
    })?;
    let syn::Expr::Closure(closure) = &args[3] else {
        return Err(format!(
            "{}: fourth argument of py_register_submodule! must be a `|m| {{ ... }}` closure",
            path.display()
        ));
    };
    let m_var = closure
        .inputs
        .first()
        .and_then(extract_pat_ident)
        .ok_or_else(|| {
            format!(
                "{}: could not extract module variable ident from py_register_submodule! closure",
                path.display()
            )
        })?;

    let mut submod = RawSubmodule {
        name: submod_name,
        ..RawSubmodule::default()
    };

    match &*closure.body {
        syn::Expr::Block(eb) => {
            for stmt in &eb.block.stmts {
                if let syn::Stmt::Expr(expr, _) = stmt
                    && let syn::Expr::MethodCall(mc) = unwrap_try(expr)
                    && is_ident_expr(&mc.receiver, &m_var)
                {
                    record_submodule_method_call(&mut submod, mc, path)?;
                }
            }
        }
        other => {
            if let syn::Expr::MethodCall(mc) = unwrap_try(other)
                && is_ident_expr(&mc.receiver, &m_var)
            {
                record_submodule_method_call(&mut submod, mc, path)?;
            }
        }
    }

    Ok(submod)
}

fn record_submodule_method_call(
    submod: &mut RawSubmodule,
    mc: &syn::ExprMethodCall,
    path: &Path,
) -> Result<(), String> {
    let method = mc.method.to_string();
    match method.as_str() {
        "add" if mc.args.len() == 2 => {
            if let Some(attr_name) = extract_str_lit(&mc.args[0]) {
                if let Some(exc_ident) = extract_get_type_turbofish(&mc.args[1]) {
                    submod.exceptions.push((attr_name, exc_ident));
                } else {
                    let py_ty = infer_const_py_type(&mc.args[1]);
                    submod.constants.push((attr_name, py_ty.to_owned()));
                }
            }
        }
        "add_function" if mc.args.len() == 1 => {
            let arg = unwrap_try(&mc.args[0]);
            if let syn::Expr::Macro(em) = arg
                && path_ends_with(&em.mac.path, "wrap_pyfunction")
            {
                let fn_args = em
                    .mac
                    .parse_body_with(Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated)
                    .map_err(|e| format!("{}: invalid wrap_pyfunction!: {e}", path.display()))?;
                if let Some(syn::Expr::Path(fp)) = fn_args.first()
                    && let Some(seg) = fp.path.segments.last()
                {
                    submod.functions.push(unraw_ident(&seg.ident));
                }
            }
        }
        "add_class" => {
            if let Some(tf) = &mc.turbofish {
                for ga in &tf.args {
                    if let syn::GenericArgument::Type(ty) = ga
                        && let Some(cls_ident) = extract_type_last_ident(ty)
                    {
                        submod.classes.push(cls_ident);
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn parse_create_exception(
    item_mac: &syn::ItemMacro,
    path: &Path,
) -> Result<(String, RawException), String> {
    let args = item_mac
        .mac
        .parse_body_with(Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated)
        .map_err(|e| format!("{}: invalid create_exception!: {e}", path.display()))?;
    if args.len() < 3 {
        return Err(format!(
            "{}: create_exception! requires at least 3 arguments",
            path.display()
        ));
    }
    let exc_ident = extract_expr_last_ident(&args[1]).ok_or_else(|| {
        format!(
            "{}: could not parse exception identifier in create_exception!",
            path.display()
        )
    })?;
    let base_raw = extract_expr_last_ident(&args[2]).unwrap_or_else(|| "Exception".to_owned());
    let base = map_exception_base(&base_raw);

    let (doc_lines, _, _) = parse_doc_lines(&item_mac.attrs);
    let doc = extract_doc_summary(&doc_lines).or_else(|| args.get(3).and_then(extract_str_lit));

    Ok((exc_ident, RawException { base, doc }))
}

fn map_exception_base(raw: &str) -> String {
    match raw {
        "PyException" => "Exception".to_owned(),
        "PyBaseException" => "BaseException".to_owned(),
        "PyRuntimeError" => "RuntimeError".to_owned(),
        "PyValueError" => "ValueError".to_owned(),
        "PyTypeError" => "TypeError".to_owned(),
        "PyOSError" | "PyIOError" => "OSError".to_owned(),
        "PyKeyError" => "KeyError".to_owned(),
        "PyIndexError" => "IndexError".to_owned(),
        other => other.to_owned(),
    }
}

fn parse_pyclass_struct(
    item_struct: &syn::ItemStruct,
    path: &Path,
) -> Result<Option<RawClass>, String> {
    let mut is_pyclass = false;
    let mut py_name: Option<String> = None;
    let mut module_name: Option<String> = None;

    for attr in &item_struct.attrs {
        if !attr.path().is_ident("pyclass") {
            continue;
        }
        is_pyclass = true;
        if matches!(attr.meta, syn::Meta::List(_)) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let lit: syn::LitStr = meta.value()?.parse()?;
                    py_name = Some(lit.value());
                } else if meta.path.is_ident("module") {
                    let lit: syn::LitStr = meta.value()?.parse()?;
                    module_name = Some(lit.value());
                } else {
                    consume_unknown_nested_meta(&meta)?;
                }
                Ok(())
            })
            .map_err(|e| format!("{}: invalid #[pyclass] attribute: {e}", path.display()))?;
        }
    }

    if !is_pyclass {
        return Ok(None);
    }

    let (doc_lines, _, _) = parse_doc_lines(&item_struct.attrs);
    let explicit_submod =
        module_name.map(|m| m.strip_prefix("nv_rs.").unwrap_or(m.as_str()).to_owned());

    Ok(Some(RawClass {
        py_name: py_name.unwrap_or_else(|| unraw_ident(&item_struct.ident)),
        explicit_submod,
        doc: extract_doc_summary(&doc_lines),
    }))
}

fn parse_raw_method(
    method: &syn::ImplItemFn,
    self_struct_ident: &str,
    path: &Path,
) -> Result<RawMethod, String> {
    let rust_name = unraw_ident(&method.sig.ident);
    let attrs = parse_callable_attrs(&method.attrs, path)?;
    let is_instance = !attrs.is_new && !attrs.is_staticmethod;
    let inputs = collect_callable_inputs(&method.sig.inputs, is_instance, Some(self_struct_ident));
    let output = extract_return_type(&method.sig.output);

    Ok(RawMethod {
        rust_name,
        attrs,
        inputs,
        output,
    })
}

fn parse_raw_function(func: &syn::ItemFn, path: &Path) -> Result<RawFunction, String> {
    let rust_name = unraw_ident(&func.sig.ident);
    let attrs = parse_callable_attrs(&func.attrs, path)?;
    let inputs = collect_callable_inputs(&func.sig.inputs, false, None);
    let output = extract_return_type(&func.sig.output);

    Ok(RawFunction {
        rust_name,
        attrs,
        inputs,
        output,
    })
}

fn collect_callable_inputs(
    fn_args: &Punctuated<syn::FnArg, syn::Token![,]>,
    expect_self_receiver: bool,
    self_struct_ident: Option<&str>,
) -> Vec<(String, syn::Type)> {
    let mut out = Vec::new();
    let mut consumed_self = false;

    for arg in fn_args {
        match arg {
            syn::FnArg::Receiver(_) => {
                consumed_self = true;
            }
            syn::FnArg::Typed(pat_ty) => {
                if is_python_gil_token(&pat_ty.ty) {
                    continue;
                }
                let param_name = extract_pat_ident(&pat_ty.pat).unwrap_or_else(|| "arg".to_owned());
                if expect_self_receiver
                    && !consumed_self
                    && is_typed_self_receiver(&param_name, &pat_ty.ty, self_struct_ident)
                {
                    consumed_self = true;
                    continue;
                }
                out.push((param_name, (*pat_ty.ty).clone()));
            }
        }
    }
    out
}

fn is_python_gil_token(ty: &syn::Type) -> bool {
    let inner = match ty {
        syn::Type::Reference(r) => &*r.elem,
        other => other,
    };
    extract_type_last_ident(inner).as_deref() == Some("Python")
}

fn is_typed_self_receiver(
    param_name: &str,
    ty: &syn::Type,
    self_struct_ident: Option<&str>,
) -> bool {
    if !matches!(param_name, "slf" | "_slf" | "self_") {
        return false;
    }
    let unwrapped = unwrap_pyo3_smart_ptr(ty);
    let Some(last) = extract_type_last_ident(unwrapped) else {
        return false;
    };
    last == "Self" || self_struct_ident.is_some_and(|s| s == last)
}

fn unwrap_pyo3_smart_ptr(mut ty: &syn::Type) -> &syn::Type {
    loop {
        match ty {
            syn::Type::Reference(r) => ty = &r.elem,
            syn::Type::Path(tp) => {
                let Some(seg) = tp.path.segments.last() else {
                    return ty;
                };
                let id = seg.ident.to_string();
                if matches!(
                    id.as_str(),
                    "PyRef" | "PyRefMut" | "Py" | "Bound" | "Borrowed"
                ) && let syn::PathArguments::AngleBracketed(ab) = &seg.arguments
                    && let Some(inner) = ab.args.iter().find_map(|ga| match ga {
                        syn::GenericArgument::Type(t) => Some(t),
                        _ => None,
                    })
                {
                    ty = inner;
                    continue;
                }
                return ty;
            }
            _ => return ty,
        }
    }
}

fn extract_return_type(ret: &syn::ReturnType) -> Option<syn::Type> {
    match ret {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, ty) => Some((**ty).clone()),
    }
}

fn parse_callable_attrs(attrs: &[syn::Attribute], path: &Path) -> Result<ParsedItemAttrs, String> {
    let (_, py_return_override, py_param_overrides) = parse_doc_lines(attrs);
    let mut out = ParsedItemAttrs {
        py_return_override,
        py_param_overrides,
        ..ParsedItemAttrs::default()
    };

    for attr in attrs {
        if attr.path().is_ident("new") {
            out.is_new = true;
        } else if attr.path().is_ident("staticmethod") {
            out.is_staticmethod = true;
        } else if attr.path().is_ident("getter") {
            let custom = parse_optional_ident_or_str_arg(attr, path, "getter")?;
            out.getter = Some(custom);
        } else if attr.path().is_ident("setter") {
            let custom = parse_optional_ident_or_str_arg(attr, path, "setter")?;
            out.setter = Some(custom);
        } else if (attr.path().is_ident("pyo3") || attr.path().is_ident("pyfunction"))
            && matches!(attr.meta, syn::Meta::List(_))
        {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let lit: syn::LitStr = meta.value()?.parse()?;
                    out.name_override = Some(lit.value());
                } else if meta.path.is_ident("signature") {
                    let _: syn::Token![=] = meta.input.parse()?;
                    let content;
                    syn::parenthesized!(content in meta.input);
                    out.signature = Some(parse_pyo3_signature(&content)?);
                } else {
                    consume_unknown_nested_meta(&meta)?;
                }
                Ok(())
            })
            .map_err(|e| format!("{}: invalid PyO3 attribute: {e}", path.display()))?;
        }
    }

    Ok(out)
}

fn parse_optional_ident_or_str_arg(
    attr: &syn::Attribute,
    path: &Path,
    attr_label: &str,
) -> Result<Option<String>, String> {
    let syn::Meta::List(list) = &attr.meta else {
        return Ok(None);
    };
    let name = list
        .parse_args_with(parse_ident_or_str)
        .map_err(|e| format!("{}: invalid #[{attr_label}(...)]: {e}", path.display()))?;
    Ok(Some(name))
}

fn parse_ident_or_str(input: syn::parse::ParseStream<'_>) -> syn::Result<String> {
    if input.peek(syn::LitStr) {
        Ok(input.parse::<syn::LitStr>()?.value())
    } else {
        let id = syn::Ident::parse_any(input)?;
        Ok(unraw_ident(&id))
    }
}

fn consume_unknown_nested_meta(meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<()> {
    if meta.input.peek(syn::Token![=]) {
        let _: syn::Expr = meta.value()?.parse()?;
    } else if meta.input.peek(syn::token::Paren) {
        let content;
        syn::parenthesized!(content in meta.input);
        skip_all_tokens(&content)?;
    }
    Ok(())
}

fn skip_all_tokens(input: syn::parse::ParseStream<'_>) -> syn::Result<()> {
    input.step(|cursor| {
        let mut rest = *cursor;
        while let Some((_, next)) = rest.token_tree() {
            rest = next;
        }
        Ok(((), rest))
    })
}

fn parse_pyo3_signature(input: syn::parse::ParseStream<'_>) -> syn::Result<Vec<SigItem>> {
    let mut items = Vec::new();
    while !input.is_empty() {
        if input.peek(syn::Token![/]) {
            let _: syn::Token![/] = input.parse()?;
            items.push(SigItem::PosOnlySep);
        } else if input.peek(syn::Token![*]) {
            let _: syn::Token![*] = input.parse()?;
            if input.peek(syn::Token![*]) {
                let _: syn::Token![*] = input.parse()?;
                let id = syn::Ident::parse_any(input)?;
                items.push(SigItem::KwArgs(unraw_ident(&id)));
            } else if input.is_empty() || input.peek(syn::Token![,]) {
                items.push(SigItem::KwOnlySep);
            } else {
                let id = syn::Ident::parse_any(input)?;
                items.push(SigItem::VarArgs(unraw_ident(&id)));
            }
        } else {
            let id = syn::Ident::parse_any(input)?;
            let default = if input.peek(syn::Token![=]) {
                let _: syn::Token![=] = input.parse()?;
                let expr: syn::Expr = input.parse()?;
                Some(format_py_default(&expr))
            } else {
                None
            };
            items.push(SigItem::Param {
                name: unraw_ident(&id),
                default,
            });
        }

        if input.peek(syn::Token![,]) {
            let _: syn::Token![,] = input.parse()?;
        } else {
            break;
        }
    }
    Ok(items)
}

fn format_py_default(expr: &syn::Expr) -> String {
    match expr {
        syn::Expr::Lit(el) => match &el.lit {
            syn::Lit::Bool(b) => {
                if b.value {
                    "True".to_owned()
                } else {
                    "False".to_owned()
                }
            }
            syn::Lit::Str(s) => format!("{:?}", s.value()),
            syn::Lit::Int(i) => {
                let raw = i.to_string();
                raw.strip_suffix(i.suffix()).unwrap_or(&raw).to_owned()
            }
            syn::Lit::Float(f) => {
                let raw = f.to_string();
                raw.strip_suffix(f.suffix()).unwrap_or(&raw).to_owned()
            }
            _ => "None".to_owned(),
        },
        syn::Expr::Unary(u) if matches!(u.op, syn::UnOp::Neg(_)) => {
            format!("-{}", format_py_default(&u.expr))
        }
        syn::Expr::Path(ep) => {
            if ep.path.segments.last().is_some_and(|s| s.ident == "None") {
                "None".to_owned()
            } else {
                ep.path
                    .segments
                    .last()
                    .map_or_else(|| "None".to_owned(), |s| unraw_ident(&s.ident))
            }
        }
        syn::Expr::Call(call) => {
            if let syn::Expr::Path(ep) = &*call.func
                && ep.path.segments.last().is_some_and(|s| s.ident == "Some")
                && let Some(first) = call.args.first()
            {
                return format_py_default(first);
            }
            "None".to_owned()
        }
        _ => "None".to_owned(),
    }
}

type DocParseResult = (Vec<String>, Option<String>, BTreeMap<String, String>);

fn parse_doc_lines(attrs: &[syn::Attribute]) -> DocParseResult {
    let mut prose = Vec::new();
    let mut ret_override = None;
    let mut param_overrides = BTreeMap::new();

    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }
        let syn::Meta::NameValue(nv) = &attr.meta else {
            continue;
        };
        let Some(raw) = extract_str_lit(&nv.value) else {
            continue;
        };
        let trimmed = raw.trim();
        if let Some(rest) = trimmed.strip_prefix("@py-return:") {
            let ty = rest.trim();
            if !ty.is_empty() {
                ret_override = Some(ty.to_owned());
            }
        } else if let Some(rest) = trimmed
            .strip_prefix("@py-type ")
            .or_else(|| trimmed.strip_prefix("@py-param "))
        {
            if let Some((param, ty)) = rest.split_once(':') {
                let p = param.trim();
                let t = ty.trim();
                if !p.is_empty() && !t.is_empty() {
                    param_overrides.insert(p.to_owned(), t.to_owned());
                }
            }
        } else {
            prose.push(trimmed.to_owned());
        }
    }

    (prose, ret_override, param_overrides)
}

fn extract_doc_summary(lines: &[String]) -> Option<String> {
    let mut paragraph = Vec::new();
    for line in lines {
        if line.is_empty() {
            if !paragraph.is_empty() {
                break;
            }
            continue;
        }
        paragraph.push(line.as_str());
    }
    if paragraph.is_empty() {
        None
    } else {
        Some(paragraph.join(" "))
    }
}

// ============================================================================
// Pure-Python `__init__.py` Parsing
// ============================================================================

fn parse_init_py(src: &str) -> InitPyInfo {
    let mut info = InitPyInfo::default();
    let lines: Vec<&str> = src.lines().collect();
    let mut i = 0usize;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = strip_py_comment(line).trim();

        // 1. `from ._nv_rs import (...)` or single-line `from ._nv_rs import a, b`
        if let Some(rest) = trimmed.strip_prefix("from ._nv_rs import") {
            let rest = rest.trim();
            if let Some(after_open) = rest.strip_prefix('(') {
                let mut buf = String::from(after_open);
                while !buf.contains(')') && i + 1 < lines.len() {
                    i += 1;
                    buf.push(' ');
                    buf.push_str(strip_py_comment(lines[i]).trim());
                }
                let inside = buf.split(')').next().unwrap_or("");
                for item in inside.split(',') {
                    let sym = item.trim();
                    if !sym.is_empty() {
                        info.ext_imports.insert(sym.to_owned());
                    }
                }
            } else {
                for item in rest.split(',') {
                    let sym = item.trim();
                    if !sym.is_empty() {
                        info.ext_imports.insert(sym.to_owned());
                    }
                }
            }
            i += 1;
            continue;
        }

        // 2. Top-level `def <name>(...) -> <ret>:` (column 0 only, public functions)
        if let Some(after_def) = line.strip_prefix("def ") {
            let mut header = String::from(after_def.trim());
            while !is_complete_py_def_header(&header) && i + 1 < lines.len() {
                i += 1;
                header.push(' ');
                header.push_str(strip_py_comment(lines[i]).trim());
            }
            if let Some(mut pf) = parse_py_def_header(&header)
                && !pf.name.starts_with('_')
            {
                // Extract optional docstring immediately following `def ...:`.
                if let Some(next_line) = lines.get(i + 1)
                    && let Some(after_triple) = next_line.trim().strip_prefix("\"\"\"")
                {
                    i += 1;
                    if let Some(single) = after_triple.strip_suffix("\"\"\"") {
                        let s = single.trim();
                        if !s.is_empty() {
                            pf.doc = Some(s.to_owned());
                        }
                    } else {
                        let mut doc_parts = Vec::new();
                        if !after_triple.trim().is_empty() {
                            doc_parts.push(after_triple.trim());
                        }
                        while i + 1 < lines.len() {
                            i += 1;
                            let dl = lines[i].trim();
                            if let Some(before_end) = dl.strip_suffix("\"\"\"") {
                                if !before_end.trim().is_empty() {
                                    doc_parts.push(before_end.trim());
                                }
                                break;
                            } else if !dl.is_empty() {
                                doc_parts.push(dl);
                            } else if !doc_parts.is_empty() {
                                // Stop summary at first blank line, advance past closing `"""`.
                                while i + 1 < lines.len() && !lines[i + 1].contains("\"\"\"") {
                                    i += 1;
                                }
                                if i + 1 < lines.len() {
                                    i += 1;
                                }
                                break;
                            }
                        }
                        if !doc_parts.is_empty() {
                            pf.doc = Some(doc_parts.join(" "));
                        }
                    }
                }
                info.top_fns.push(pf);
            }
            i += 1;
            continue;
        }

        // 3. `__all__ = [...]` at column 0
        if line.starts_with("__all__")
            && let Some((_, rhs)) = trimmed.split_once('=')
        {
            let mut buf = String::from(rhs.trim());
            while !buf.contains(']') && i + 1 < lines.len() {
                i += 1;
                buf.push(' ');
                buf.push_str(strip_py_comment(lines[i]).trim());
            }
            let inside = buf.trim_start_matches('[').split(']').next().unwrap_or("");
            for item in inside.split(',') {
                let s = item.trim().trim_matches('"').trim_matches('\'');
                if !s.is_empty() {
                    info.dunder_all.insert(s.to_owned());
                }
            }
            i += 1;
            continue;
        }

        // 4. Column-0 monkey-patches: `<submod>.<attr> = <fn_name>`
        if !line.starts_with(' ')
            && !line.starts_with('\t')
            && let Some((lhs, rhs)) = trimmed.split_once('=')
            && !lhs.ends_with('!')
            && !lhs.ends_with('=')
            && !rhs.starts_with('=')
        {
            let lhs = lhs.trim();
            let rhs = rhs.trim();
            if let Some((submod, attr_name)) = lhs.split_once('.')
                && is_valid_py_ident(submod)
                && is_valid_py_ident(attr_name)
                && is_valid_py_ident(rhs)
            {
                info.monkey_patches.push(MonkeyPatch {
                    submod: submod.to_owned(),
                    attr_name: attr_name.to_owned(),
                    target_fn: rhs.to_owned(),
                });
            }
        }

        i += 1;
    }

    info
}

fn strip_py_comment(line: &str) -> &str {
    let mut in_single = false;
    let mut in_double = false;
    let mut escaped = false;
    for (idx, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && (in_single || in_double) {
            escaped = true;
            continue;
        }
        if ch == '\'' && !in_double {
            in_single = !in_single;
        } else if ch == '"' && !in_single {
            in_double = !in_double;
        } else if ch == '#' && !in_single && !in_double {
            return &line[..idx];
        }
    }
    line
}

fn is_complete_py_def_header(header: &str) -> bool {
    let mut depth = 0i32;
    for ch in header.chars() {
        match ch {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            _ => {}
        }
    }
    depth == 0 && header.trim_end().ends_with(':')
}

fn parse_py_def_header(header: &str) -> Option<PurePyFn> {
    let open = header.find('(')?;
    let name = header[..open].trim();
    if !is_valid_py_ident(name) {
        return None;
    }
    let mut depth = 0i32;
    let mut close = None;
    for (idx, ch) in header[open..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    close = Some(open + idx);
                    break;
                }
            }
            _ => {}
        }
    }
    let close_idx = close?;
    let params_raw = collapse_ws(header[open + 1..close_idx].trim());
    let tail = header[close_idx + 1..].trim().trim_end_matches(':').trim();
    let ret_ty = if let Some(arrow) = tail.strip_prefix("->") {
        arrow.trim().to_owned()
    } else {
        "Any".to_owned()
    };

    Some(PurePyFn {
        name: name.to_owned(),
        params_raw,
        ret_ty,
        doc: None,
    })
}

fn collapse_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_valid_py_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

// ============================================================================
// Signature & Type Resolution (Pass 2)
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq)]
enum TypePos {
    Param,
    Return,
}

struct TypeCtx<'a> {
    class_registry: &'a BTreeMap<String, (String, String)>,
    self_class: Option<(&'a str, &'a str)>,
}

fn resolve_method(rm: &RawMethod, ctx: &TypeCtx<'_>) -> Result<Option<MethodSpec>, String> {
    let base_name = rm
        .attrs
        .name_override
        .clone()
        .unwrap_or_else(|| rm.rust_name.clone());

    // Python `object` already provides `__repr__`; stubs and docs omit display-only `__repr__`.
    if base_name == "__repr__" {
        return Ok(None);
    }

    if rm.attrs.is_new {
        let params = resolve_params(
            &rm.inputs,
            rm.attrs.signature.as_deref(),
            &rm.attrs.py_param_overrides,
            ctx,
        )?;
        return Ok(Some(MethodSpec {
            kind: MethodKind::New,
            py_name: "__init__".to_owned(),
            params,
            ret_ty: "None".to_owned(),
        }));
    }

    if let Some(ref custom_prop) = rm.attrs.getter {
        let prop_name = custom_prop.clone().unwrap_or(base_name);
        let ret_ty = resolve_return_type(
            rm.output.as_ref(),
            rm.attrs.py_return_override.as_deref(),
            ctx,
        );
        return Ok(Some(MethodSpec {
            kind: MethodKind::Getter(prop_name.clone()),
            py_name: prop_name,
            params: Vec::new(),
            ret_ty,
        }));
    }

    if let Some(ref custom_prop) = rm.attrs.setter {
        let prop_name = custom_prop.clone().unwrap_or_else(|| {
            base_name
                .strip_prefix("set_")
                .unwrap_or(&base_name)
                .to_owned()
        });
        let params = resolve_params(
            &rm.inputs,
            rm.attrs.signature.as_deref(),
            &rm.attrs.py_param_overrides,
            ctx,
        )?;
        return Ok(Some(MethodSpec {
            kind: MethodKind::Setter(prop_name.clone()),
            py_name: prop_name,
            params,
            ret_ty: "None".to_owned(),
        }));
    }

    let kind = if rm.attrs.is_staticmethod {
        MethodKind::Static
    } else {
        MethodKind::Instance
    };
    let params = resolve_params(
        &rm.inputs,
        rm.attrs.signature.as_deref(),
        &rm.attrs.py_param_overrides,
        ctx,
    )?;
    let ret_ty = resolve_return_type(
        rm.output.as_ref(),
        rm.attrs.py_return_override.as_deref(),
        ctx,
    );

    Ok(Some(MethodSpec {
        kind,
        py_name: base_name,
        params,
        ret_ty,
    }))
}

fn resolve_function(rf: &RawFunction, ctx: &TypeCtx<'_>) -> Result<FunctionSpec, String> {
    let py_name = rf
        .attrs
        .name_override
        .clone()
        .unwrap_or_else(|| rf.rust_name.clone());
    let params = resolve_params(
        &rf.inputs,
        rf.attrs.signature.as_deref(),
        &rf.attrs.py_param_overrides,
        ctx,
    )?;
    let ret_ty = resolve_return_type(
        rf.output.as_ref(),
        rf.attrs.py_return_override.as_deref(),
        ctx,
    );

    Ok(FunctionSpec {
        py_name,
        params,
        raw_params: None,
        ret_ty,
    })
}

fn resolve_params(
    rust_inputs: &[(String, syn::Type)],
    sig_spec: Option<&[SigItem]>,
    param_overrides: &BTreeMap<String, String>,
    ctx: &TypeCtx<'_>,
) -> Result<Vec<ParamSpec>, String> {
    let mut ty_by_name: BTreeMap<&str, &syn::Type> = BTreeMap::new();
    for (name, ty) in rust_inputs {
        ty_by_name.insert(name.as_str(), ty);
        if let Some(stripped) = name.strip_prefix('_')
            && !stripped.is_empty()
        {
            ty_by_name.entry(stripped).or_insert(ty);
        }
    }

    if let Some(items) = sig_spec {
        let mut out = Vec::with_capacity(items.len());
        for item in items {
            match item {
                SigItem::PosOnlySep => out.push(ParamSpec {
                    name: "/".to_owned(),
                    ty: None,
                    default: None,
                }),
                SigItem::KwOnlySep => out.push(ParamSpec {
                    name: "*".to_owned(),
                    ty: None,
                    default: None,
                }),
                SigItem::VarArgs(n) => out.push(ParamSpec {
                    name: format!("*{n}"),
                    ty: Some(
                        param_overrides
                            .get(n)
                            .cloned()
                            .unwrap_or_else(|| "Any".to_owned()),
                    ),
                    default: None,
                }),
                SigItem::KwArgs(n) => out.push(ParamSpec {
                    name: format!("**{n}"),
                    ty: Some(
                        param_overrides
                            .get(n)
                            .cloned()
                            .unwrap_or_else(|| "Any".to_owned()),
                    ),
                    default: None,
                }),
                SigItem::Param { name, default } => {
                    let py_ty = if let Some(ov) = param_overrides.get(name) {
                        ov.clone()
                    } else if let Some(rty) = ty_by_name.get(name.as_str()) {
                        map_rust_type(rty, TypePos::Param, ctx)
                    } else {
                        return Err(format!(
                            "#[pyo3(signature = (...))] names parameter `{name}`, which is not in Rust fn inputs"
                        ));
                    };
                    out.push(ParamSpec {
                        name: name.clone(),
                        ty: Some(py_ty),
                        default: default.clone(),
                    });
                }
            }
        }
        return Ok(out);
    }

    // When `#[pyo3(signature = ...)]` is absent, PyO3 treats trailing `Option<T>` parameters
    // (those with no required parameter after them) as defaulting to `None`.
    let first_trailing_opt = rust_inputs
        .iter()
        .rposition(|(_, ty)| !is_option_type(ty))
        .map_or(0, |idx| idx + 1);

    let mut out = Vec::with_capacity(rust_inputs.len());
    for (idx, (name, rty)) in rust_inputs.iter().enumerate() {
        let py_ty = if let Some(ov) = param_overrides.get(name) {
            ov.clone()
        } else {
            map_rust_type(rty, TypePos::Param, ctx)
        };
        let default = if idx >= first_trailing_opt && is_option_type(rty) {
            Some("None".to_owned())
        } else {
            None
        };
        out.push(ParamSpec {
            name: name.clone(),
            ty: Some(py_ty),
            default,
        });
    }
    Ok(out)
}

fn resolve_return_type(
    output: Option<&syn::Type>,
    override_ty: Option<&str>,
    ctx: &TypeCtx<'_>,
) -> String {
    if let Some(ov) = override_ty {
        return ov.to_owned();
    }
    match output {
        None => "None".to_owned(),
        Some(ty) => map_rust_type(ty, TypePos::Return, ctx),
    }
}

fn is_option_type(ty: &syn::Type) -> bool {
    extract_type_last_ident(ty).as_deref() == Some("Option")
}

fn is_u8_type(ty: &syn::Type) -> bool {
    extract_type_last_ident(ty).as_deref() == Some("u8")
}

fn map_rust_type(ty: &syn::Type, pos: TypePos, ctx: &TypeCtx<'_>) -> String {
    match ty {
        syn::Type::Tuple(tup) => {
            if tup.elems.is_empty() {
                "None".to_owned()
            } else {
                let parts: Vec<String> = tup
                    .elems
                    .iter()
                    .map(|e| map_rust_type(e, pos, ctx))
                    .collect();
                format!("tuple[{}]", parts.join(", "))
            }
        }
        syn::Type::Reference(r) => {
            if let syn::Type::Slice(slice) = &*r.elem {
                return map_slice_elem(&slice.elem, pos, ctx);
            }
            map_rust_type(&r.elem, pos, ctx)
        }
        syn::Type::Slice(slice) => map_slice_elem(&slice.elem, pos, ctx),
        syn::Type::Array(arr) => {
            let elem = map_rust_type(&arr.elem, pos, ctx);
            match pos {
                TypePos::Param => format!("Sequence[{elem}]"),
                TypePos::Return => format!("list[{elem}]"),
            }
        }
        syn::Type::Path(tp) => map_path_type(tp, pos, ctx),
        _ => "Any".to_owned(),
    }
}

fn map_slice_elem(elem_ty: &syn::Type, pos: TypePos, ctx: &TypeCtx<'_>) -> String {
    if is_u8_type(elem_ty) {
        return "bytes".to_owned();
    }
    let elem = map_rust_type(elem_ty, pos, ctx);
    match pos {
        TypePos::Param => format!("Sequence[{elem}]"),
        TypePos::Return => format!("list[{elem}]"),
    }
}

fn map_path_type(tp: &syn::TypePath, pos: TypePos, ctx: &TypeCtx<'_>) -> String {
    let Some(seg) = tp.path.segments.last() else {
        return "Any".to_owned();
    };
    let ident = unraw_ident(&seg.ident);
    let type_args = extract_angle_type_args(&seg.arguments);

    match ident.as_str() {
        "PyResult" | "Result" if !type_args.is_empty() => map_rust_type(type_args[0], pos, ctx),
        "Option" if !type_args.is_empty() => {
            let inner = map_rust_type(type_args[0], pos, ctx);
            if inner == "Any" || inner.ends_with("| None") {
                inner
            } else {
                format!("{inner} | None")
            }
        }
        "Bound" | "Borrowed" | "Py" | "PyRef" | "PyRefMut" if !type_args.is_empty() => {
            map_rust_type(type_args[0], pos, ctx)
        }
        "Vec" if !type_args.is_empty() => {
            let elem = map_rust_type(type_args[0], pos, ctx);
            match pos {
                TypePos::Param => format!("Sequence[{elem}]"),
                TypePos::Return => format!("list[{elem}]"),
            }
        }
        "HashMap" | "BTreeMap" if type_args.len() >= 2 => {
            let k = map_rust_type(type_args[0], pos, ctx);
            let v = map_rust_type(type_args[1], pos, ctx);
            match pos {
                TypePos::Param => format!("Mapping[{k}, {v}]"),
                TypePos::Return => format!("dict[{k}, {v}]"),
            }
        }
        "HashSet" | "BTreeSet" if !type_args.is_empty() => {
            let elem = map_rust_type(type_args[0], pos, ctx);
            format!("set[{elem}]")
        }
        "bool" => "bool".to_owned(),
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128"
        | "usize" => "int".to_owned(),
        "f32" | "f64" => "float".to_owned(),
        "str" | "String" | "PyString" => "str".to_owned(),
        "Path" | "PathBuf" => "str | os.PathLike[str]".to_owned(),
        "PyBytes" | "PyByteArray" => "bytes".to_owned(),
        "PyDict" => "dict[str, Any]".to_owned(),
        "PyList" => "list[Any]".to_owned(),
        "PyTuple" => "tuple[Any, ...]".to_owned(),
        "PyAny" | "PyObject" | "PyCapsule" => "Any".to_owned(),
        "Self" => {
            if let Some((submod, cls)) = ctx.self_class {
                format!("{submod}.{cls}")
            } else {
                "Any".to_owned()
            }
        }
        other => {
            if let Some((submod, cls)) = ctx.class_registry.get(other) {
                format!("{submod}.{cls}")
            } else {
                "Any".to_owned()
            }
        }
    }
}

fn extract_angle_type_args(args: &syn::PathArguments) -> Vec<&syn::Type> {
    let syn::PathArguments::AngleBracketed(ab) = args else {
        return Vec::new();
    };
    ab.args
        .iter()
        .filter_map(|ga| match ga {
            syn::GenericArgument::Type(t) => Some(t),
            _ => None,
        })
        .collect()
}

// ============================================================================
// Rendering `.pyi` Stub (`python/nv-py/python/nv_rs/__init__.pyi`)
// ============================================================================

fn render_pyi(model: &ApiModel) -> String {
    let all_submods: BTreeSet<String> = model.submodules.iter().map(|s| s.name.clone()).collect();
    let mut defined_submods: BTreeSet<String> = BTreeSet::new();
    let mut body = String::new();

    for (attr, ty) in &model.top_attrs {
        body.push_str(&format!("{attr}: {ty}\n"));
    }
    if !model.top_attrs.is_empty() {
        body.push('\n');
    }

    for sub in &model.submodules {
        body.push_str(&format!("class {}:\n", sub.name));
        let is_empty = sub.constants.is_empty()
            && sub.exceptions.is_empty()
            && sub.classes.is_empty()
            && sub.functions.is_empty();
        if is_empty {
            body.push_str("    ...\n\n");
            defined_submods.insert(sub.name.clone());
            continue;
        }

        for (cname, cty) in &sub.constants {
            let q = quote_pyi_type(cty, &all_submods, &defined_submods);
            body.push_str(&format!("    {cname}: {q}\n"));
        }
        if !sub.constants.is_empty()
            && (!sub.exceptions.is_empty() || !sub.classes.is_empty() || !sub.functions.is_empty())
        {
            body.push('\n');
        }

        for exc in &sub.exceptions {
            body.push_str(&format!("    class {}({}): ...\n", exc.name, exc.base));
        }
        if !sub.exceptions.is_empty() && (!sub.classes.is_empty() || !sub.functions.is_empty()) {
            body.push('\n');
        }

        for (cls_idx, cls) in sub.classes.iter().enumerate() {
            if cls.methods.is_empty() {
                body.push_str(&format!("    class {}: ...\n", cls.py_name));
            } else {
                body.push_str(&format!("    class {}:\n", cls.py_name));
                for m in &cls.methods {
                    render_pyi_method(&mut body, m, &all_submods, &defined_submods);
                }
            }
            if cls_idx + 1 < sub.classes.len() || !sub.functions.is_empty() {
                body.push('\n');
            }
        }

        for func in &sub.functions {
            body.push_str("    @staticmethod\n");
            let params_str = match &func.raw_params {
                Some(raw) => raw.clone(),
                None => format_params_pyi(&func.params, &all_submods, &defined_submods),
            };
            let ret_str = quote_pyi_type(&func.ret_ty, &all_submods, &defined_submods);
            body.push_str(&format!(
                "    def {}({params_str}) -> {ret_str}: ...\n",
                func.py_name
            ));
        }

        body.push('\n');
        defined_submods.insert(sub.name.clone());
    }

    for pf in &model.top_py_fns {
        let ret_str = quote_pyi_type(&pf.ret_ty, &all_submods, &defined_submods);
        body.push_str(&format!(
            "def {}({}) -> {ret_str}: ...\n",
            pf.name, pf.params_raw
        ));
    }

    // Prepend deterministic minimal imports required by `body`.
    let mut header =
        String::from("# Generated by `cargo xtask py-stubs generate`. Do not edit by hand.\n\n");
    let mut has_imports = false;
    if body.contains("os.PathLike") {
        header.push_str("import os\n");
        has_imports = true;
    }
    let mut abc_imports = Vec::new();
    if contains_ident_token(&body, "Mapping") {
        abc_imports.push("Mapping");
    }
    if contains_ident_token(&body, "Sequence") {
        abc_imports.push("Sequence");
    }
    if !abc_imports.is_empty() {
        header.push_str(&format!(
            "from collections.abc import {}\n",
            abc_imports.join(", ")
        ));
        has_imports = true;
    }
    let mut typing_imports = Vec::new();
    if contains_ident_token(&body, "Any") {
        typing_imports.push("Any");
    }
    if contains_ident_token(&body, "Literal") {
        typing_imports.push("Literal");
    }
    if !typing_imports.is_empty() {
        header.push_str(&format!(
            "from typing import {}\n",
            typing_imports.join(", ")
        ));
        has_imports = true;
    }
    if has_imports {
        header.push('\n');
    }

    header.push_str(body.trim_end());
    header.push('\n');
    header
}

fn render_pyi_method(
    out: &mut String,
    m: &MethodSpec,
    all_submods: &BTreeSet<String>,
    defined_submods: &BTreeSet<String>,
) {
    let ret = quote_pyi_type(&m.ret_ty, all_submods, defined_submods);
    match &m.kind {
        MethodKind::New => {
            let params = format_self_and_params_pyi(&m.params, all_submods, defined_submods);
            out.push_str(&format!("        def __init__({params}) -> None: ...\n"));
        }
        MethodKind::Instance => {
            let params = format_self_and_params_pyi(&m.params, all_submods, defined_submods);
            out.push_str(&format!(
                "        def {}({params}) -> {ret}: ...\n",
                m.py_name
            ));
        }
        MethodKind::Static => {
            out.push_str("        @staticmethod\n");
            let params = format_params_pyi(&m.params, all_submods, defined_submods);
            out.push_str(&format!(
                "        def {}({params}) -> {ret}: ...\n",
                m.py_name
            ));
        }
        MethodKind::Getter(prop) => {
            out.push_str("        @property\n");
            out.push_str(&format!("        def {prop}(self) -> {ret}: ...\n"));
        }
        MethodKind::Setter(prop) => {
            out.push_str(&format!("        @{prop}.setter\n"));
            let params = format_self_and_params_pyi(&m.params, all_submods, defined_submods);
            out.push_str(&format!("        def {prop}({params}) -> None: ...\n"));
        }
    }
}

fn format_self_and_params_pyi(
    params: &[ParamSpec],
    all_submods: &BTreeSet<String>,
    defined_submods: &BTreeSet<String>,
) -> String {
    if params.is_empty() {
        "self".to_owned()
    } else {
        format!(
            "self, {}",
            format_params_pyi(params, all_submods, defined_submods)
        )
    }
}

fn format_params_pyi(
    params: &[ParamSpec],
    all_submods: &BTreeSet<String>,
    defined_submods: &BTreeSet<String>,
) -> String {
    params
        .iter()
        .map(|p| format_one_param(p, Some((all_submods, defined_submods))))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_one_param(
    p: &ParamSpec,
    quote_ctx: Option<(&BTreeSet<String>, &BTreeSet<String>)>,
) -> String {
    let Some(ref raw_ty) = p.ty else {
        return p.name.clone();
    };
    let ty = match quote_ctx {
        Some((all_submods, defined_submods)) => {
            quote_pyi_type(raw_ty, all_submods, defined_submods)
        }
        None => raw_ty.clone(),
    };
    match &p.default {
        Some(def) => format!("{}: {ty} = {def}", p.name),
        None => format!("{}: {ty}", p.name),
    }
}

/// Quotes a `.pyi` type annotation as `"..."` when it references a submodule not yet defined
/// above the current `class <submod>:` block (including self-references inside `class <submod>:`).
fn quote_pyi_type(
    ty: &str,
    all_submods: &BTreeSet<String>,
    defined_submods: &BTreeSet<String>,
) -> String {
    if references_unseen_submod(ty, all_submods, defined_submods) {
        if ty.contains('"') {
            format!("'{ty}'")
        } else {
            format!("\"{ty}\"")
        }
    } else {
        ty.to_owned()
    }
}

fn references_unseen_submod(
    ty: &str,
    all_submods: &BTreeSet<String>,
    defined_submods: &BTreeSet<String>,
) -> bool {
    let bytes = ty.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i].is_ascii_alphabetic() || bytes[i] == b'_' {
            let start = i;
            i += 1;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            if i < bytes.len() && bytes[i] == b'.' {
                let ident = &ty[start..i];
                if all_submods.contains(ident) && !defined_submods.contains(ident) {
                    return true;
                }
            }
        } else {
            i += 1;
        }
    }
    false
}

fn contains_ident_token(haystack: &str, needle: &str) -> bool {
    let is_ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    haystack.match_indices(needle).any(|(idx, _)| {
        let before = haystack[..idx].chars().next_back();
        let after = haystack[idx + needle.len()..].chars().next();
        !before.is_some_and(is_ident) && !after.is_some_and(is_ident)
    })
}

// ============================================================================
// Rendering Starlight Reference (`docs/src/content/docs/python/04-modules-reference.md`)
// ============================================================================

fn render_docs_md(model: &ApiModel) -> String {
    let mut out = String::from(
        "---\n\
         title: Modules Reference\n\
         description: Complete API reference for all nv_rs Python submodules, classes, methods, parameters, default values, and return types.\n\
         ---\n\n\
         Generated by `cargo xtask py-stubs generate` from `python/nv-py/`. Do not edit by hand.\n\n\
         This reference documents every submodule, class, method, property, parameter, default value, and return type exposed by `nv_rs` (matching `python/nv-py/python/nv_rs/__init__.pyi`).\n\n\
         Top-level module attributes and convenience functions:\n\n",
    );

    for (attr, ty) in &model.top_attrs {
        out.push_str(&format!("- `{attr}: {ty}`\n"));
    }
    for pf in &model.top_py_fns {
        match &pf.doc {
            Some(doc) => out.push_str(&format!(
                "- `{}({}) -> {}` — {}\n",
                pf.name,
                pf.params_raw,
                pf.ret_ty,
                sanitize_md_prose(doc)
            )),
            None => out.push_str(&format!(
                "- `{}({}) -> {}`\n",
                pf.name, pf.params_raw, pf.ret_ty
            )),
        }
    }

    let sub_by_name: BTreeMap<&str, &SubmoduleSpec> = model
        .submodules
        .iter()
        .map(|s| (s.name.as_str(), s))
        .collect();
    let mut emitted: BTreeSet<&str> = BTreeSet::new();

    for (section_title, members) in DOC_SECTIONS {
        let present: Vec<&SubmoduleSpec> = members
            .iter()
            .filter_map(|m| sub_by_name.get(m).copied())
            .collect();
        if present.is_empty() {
            continue;
        }
        out.push_str(&format!("\n---\n\n## {section_title}\n"));
        for sub in present {
            emitted.insert(sub.name.as_str());
            render_docs_submodule(&mut out, sub);
        }
    }

    let remaining: Vec<&SubmoduleSpec> = model
        .submodules
        .iter()
        .filter(|s| !emitted.contains(s.name.as_str()))
        .collect();
    if !remaining.is_empty() {
        let heading = if emitted.is_empty() {
            "Submodules"
        } else {
            "Additional Submodules"
        };
        out.push_str(&format!("\n---\n\n## {heading}\n"));
        for sub in remaining {
            render_docs_submodule(&mut out, sub);
        }
    }

    out
}

fn render_docs_submodule(out: &mut String, sub: &SubmoduleSpec) {
    out.push_str(&format!("\n### `nv_rs.{}`\n\n", sub.name));
    out.push_str(&format!("{}\n", sanitize_md_prose(&sub.summary)));

    if !sub.constants.is_empty() {
        out.push('\n');
        for (cname, cty) in &sub.constants {
            out.push_str(&format!("- `{cname}: {cty}`\n"));
        }
    }

    if !sub.exceptions.is_empty() {
        out.push('\n');
        for exc in &sub.exceptions {
            match &exc.doc {
                Some(doc) => out.push_str(&format!(
                    "- `{}({})` — {}\n",
                    exc.name,
                    exc.base,
                    sanitize_md_prose(doc)
                )),
                None => out.push_str(&format!("- `{}({})`\n", exc.name, exc.base)),
            }
        }
    }

    if !sub.functions.is_empty() {
        out.push_str("\n#### Static Methods\n\n");
        for func in &sub.functions {
            let params_str = match &func.raw_params {
                Some(raw) => raw.clone(),
                None => format_params_md(&func.params),
            };
            out.push_str(&format!(
                "- `{}({params_str}) -> {}`\n",
                func.py_name, func.ret_ty
            ));
        }
    }

    for cls in &sub.classes {
        out.push_str(&format!("\n#### `{}.{}`\n\n", sub.name, cls.py_name));
        if let Some(ref doc) = cls.doc {
            out.push_str(&format!("{}\n\n", sanitize_md_prose(doc)));
        }
        for m in &cls.methods {
            render_docs_method(out, m);
        }
    }
}

fn render_docs_method(out: &mut String, m: &MethodSpec) {
    match &m.kind {
        MethodKind::New => {
            let params = format_self_and_params_md(&m.params);
            out.push_str(&format!("- `__init__({params}) -> None`\n"));
        }
        MethodKind::Instance => {
            let params = format_self_and_params_md(&m.params);
            out.push_str(&format!("- `{}({params}) -> {}`\n", m.py_name, m.ret_ty));
        }
        MethodKind::Static => {
            let params = format_params_md(&m.params);
            out.push_str(&format!(
                "- `@staticmethod {}({params}) -> {}`\n",
                m.py_name, m.ret_ty
            ));
        }
        MethodKind::Getter(prop) => {
            out.push_str(&format!("- `@property {prop}(self) -> {}`\n", m.ret_ty));
        }
        MethodKind::Setter(prop) => {
            let params = format_self_and_params_md(&m.params);
            out.push_str(&format!("- `@{prop}.setter {prop}({params}) -> None`\n"));
        }
    }
}

fn format_self_and_params_md(params: &[ParamSpec]) -> String {
    if params.is_empty() {
        "self".to_owned()
    } else {
        format!("self, {}", format_params_md(params))
    }
}

fn format_params_md(params: &[ParamSpec]) -> String {
    params
        .iter()
        .map(|p| format_one_param(p, None))
        .collect::<Vec<_>>()
        .join(", ")
}

fn resolve_submodule_summary(
    submod_name: &str,
    file_doc: Option<&str>,
    multi_submod: bool,
) -> String {
    if let Some(curated) = curated_submodule_summary(submod_name) {
        return curated.to_owned();
    }
    if !multi_submod && let Some(doc) = file_doc {
        let prefix = format!("`nv_rs.{submod_name}`");
        let cleaned = doc
            .strip_prefix(&prefix)
            .map(|s| s.trim_start_matches([' ', '—', '-', ':']))
            .unwrap_or(doc)
            .trim();
        if !cleaned.is_empty() {
            let mut chars = cleaned.chars();
            if let Some(first) = chars.next() {
                let mut cap = first.to_uppercase().to_string();
                cap.push_str(chars.as_str());
                return cap;
            }
        }
    }
    format!("Bindings for the `nv_rs.{submod_name}` submodule.")
}

fn curated_submodule_summary(name: &str) -> Option<&'static str> {
    match name {
        "errors" => Some("Unified exception hierarchy for `nv_rs` operations."),
        "diag" => {
            Some("Runtime diagnostics, environment doctor, event ring-buffer, and leak tracing.")
        }
        "core" => Some(
            "CUDA Context, Stream, and zero-copy DeviceBuffer (DLPack, `__cuda_array_interface__` v3, and Buffer Protocol).",
        ),
        "nvml" => Some(
            "NVIDIA Management Library (NVML) bindings for GPU telemetry and device enumeration.",
        ),
        "nvrtc" => Some(
            "GIL-free runtime CUDA C++ compilation to PTX, CUBIN, LTO-IR, and OptiX-IR via NVRTC.",
        ),
        "nvcomp" => Some("GPU-accelerated lossless compression and decompression via nvCOMP."),
        "nvjpeg" => Some("GPU-accelerated JPEG decoding and encoding via nvJPEG."),
        "nvenc" => Some("Hardware video encoding (`h264`, `hevc`, `av1`) via NVENC."),
        "nvdec" => Some("Hardware video decoding producing zero-copy GPU frames via NVDEC."),
        "nvofa" => Some("Hardware optical flow estimation via NVIDIA Optical Flow (NVOFA)."),
        "cublas" => Some("Dense linear algebra via cuBLAS."),
        "cufft" => Some("Fast Fourier Transforms via cuFFT."),
        "curand" => Some("GPU random number generation via cuRAND."),
        "cusparse" => Some("Sparse linear algebra via cuSPARSE."),
        "cusolver" => Some("Dense matrix factorizations and linear solvers via cuSOLVER."),
        "cudnn" => Some("Deep neural network primitives via cuDNN."),
        "optix" => Some("Ray tracing context and module management via OptiX."),
        "npp" => Some(
            "GPU image and signal processing primitives via NVIDIA Performance Primitives (NPP).",
        ),
        "ngx" => Some("NVIDIA NGX / DLSS capability queries and project validation."),
        "cupti" => Some("CUDA Profiling Tools Interface (CUPTI) activity and callback tracing."),
        "cuda_ext" => Some(
            "Extended CUDA driver features: device LUID/UUID, sparse arrays, green contexts, managed memory, host registration, and external memory descriptors.",
        ),
        "wgpu" => Some(
            "WebGPU (`wgpu`) / CUDA interoperability for shared buffers and timeline semaphores.",
        ),
        "nccl" => Some(
            "Multi-GPU collective communications via NCCL (`SUPPORTED: bool` indicates host availability).",
        ),
        "cufile" => Some(
            "GPUDirect Storage (GDS) file I/O via cuFile (`SUPPORTED: bool` indicates host availability).",
        ),
        "nvapi" => Some(
            "Windows NVIDIA driver and display APIs via NVAPI (`SUPPORTED: bool` indicates host availability).",
        ),
        _ => None,
    }
}

/// Escapes bare `<` and `>` outside inline `` `...` `` spans so Markdown/Starlight never parses
/// prose type brackets as raw HTML tags.
fn sanitize_md_prose(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_code = false;
    for ch in s.chars() {
        match ch {
            '`' => {
                in_code = !in_code;
                out.push(ch);
            }
            '<' if !in_code => out.push_str("&lt;"),
            '>' if !in_code => out.push_str("&gt;"),
            _ => out.push(ch),
        }
    }
    out
}

// ============================================================================
// Small `syn` Inspection Helpers
// ============================================================================

fn unraw_ident(ident: &syn::Ident) -> String {
    ident.unraw().to_string()
}

fn path_ends_with(path: &syn::Path, target: &str) -> bool {
    path.segments.last().is_some_and(|s| s.ident == target)
}

fn unwrap_try(mut expr: &syn::Expr) -> &syn::Expr {
    while let syn::Expr::Try(et) = expr {
        expr = &et.expr;
    }
    expr
}

fn extract_str_lit(expr: &syn::Expr) -> Option<String> {
    if let syn::Expr::Lit(el) = expr
        && let syn::Lit::Str(s) = &el.lit
    {
        Some(s.value())
    } else {
        None
    }
}

fn infer_const_py_type(expr: &syn::Expr) -> &'static str {
    match expr {
        syn::Expr::Lit(el) => match &el.lit {
            syn::Lit::Str(_) => "str",
            syn::Lit::Bool(_) => "bool",
            syn::Lit::Int(_) => "int",
            syn::Lit::Float(_) => "float",
            _ => "Any",
        },
        syn::Expr::Macro(em) => {
            if path_ends_with(&em.mac.path, "cfg") {
                "bool"
            } else if path_ends_with(&em.mac.path, "env") || path_ends_with(&em.mac.path, "concat")
            {
                "str"
            } else {
                "Any"
            }
        }
        _ => "Any",
    }
}

fn extract_pat_ident(pat: &syn::Pat) -> Option<String> {
    match pat {
        syn::Pat::Ident(pi) => Some(unraw_ident(&pi.ident)),
        syn::Pat::Type(pt) => extract_pat_ident(&pt.pat),
        _ => None,
    }
}

fn is_ident_expr(expr: &syn::Expr, target: &str) -> bool {
    if let syn::Expr::Path(ep) = expr
        && ep.path.segments.len() == 1
    {
        unraw_ident(&ep.path.segments[0].ident) == target
    } else {
        false
    }
}

fn extract_pymodule_new_name(expr: &syn::Expr) -> Option<String> {
    let syn::Expr::Call(call) = expr else {
        return None;
    };
    let syn::Expr::Path(ep) = &*call.func else {
        return None;
    };
    let len = ep.path.segments.len();
    if len >= 2
        && ep.path.segments[len - 2].ident == "PyModule"
        && ep.path.segments[len - 1].ident == "new"
        && call.args.len() >= 2
    {
        extract_str_lit(&call.args[1])
    } else {
        None
    }
}

fn extract_get_type_turbofish(expr: &syn::Expr) -> Option<String> {
    let syn::Expr::MethodCall(mc) = expr else {
        return None;
    };
    if mc.method != "get_type" {
        return None;
    }
    let tf = mc.turbofish.as_ref()?;
    tf.args.iter().find_map(|ga| match ga {
        syn::GenericArgument::Type(ty) => extract_type_last_ident(ty),
        _ => None,
    })
}

fn extract_type_last_ident(ty: &syn::Type) -> Option<String> {
    if let syn::Type::Path(tp) = ty {
        tp.path.segments.last().map(|s| unraw_ident(&s.ident))
    } else {
        None
    }
}

fn extract_expr_last_ident(expr: &syn::Expr) -> Option<String> {
    match expr {
        syn::Expr::Path(ep) => ep.path.segments.last().map(|s| unraw_ident(&s.ident)),
        syn::Expr::Field(ef) => match &ef.member {
            syn::Member::Named(id) => Some(unraw_ident(id)),
            syn::Member::Unnamed(_) => None,
        },
        _ => None,
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TMP_SEQ: AtomicUsize = AtomicUsize::new(0);

    fn temp_fixture_dir(label: &str) -> PathBuf {
        let id = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "nv_rs_py_stubs_{label}_{}_{id}",
            std::process::id()
        ));
        std::fs::remove_dir_all(&dir).ok();
        dir
    }

    fn build_synthetic_model(mod_rs: &str, init_py: &str) -> Result<ApiModel, String> {
        let lib_rs = r#"
            use pyo3::prelude::*;
            #[pymodule]
            fn _nv_rs(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
                m.add("__version__", env!("CARGO_PKG_VERSION"))?;
                demo::register(py, m)?;
                Ok(())
            }
        "#;
        let lib_ast = parse_lib_rs(lib_rs, Path::new("lib.rs"))?;
        let raw_mod = parse_module_rs(mod_rs, Path::new("demo.rs"))?;
        let init_info = parse_init_py(init_py);
        assemble_model(lib_ast, vec![("demo".to_owned(), raw_mod)], init_info)
    }

    #[test]
    fn parses_macro_and_raw_submodules_and_merges_pymethods() {
        let mod_rs = r#"
            //! `nv_rs.demo` — synthetic module.
            use pyo3::create_exception;
            use pyo3::exceptions::PyException;
            use pyo3::prelude::*;

            create_exception!(nv_rs.demo, DemoError, PyException);

            /// Demo stream handle.
            #[pyclass(name = "Stream", module = "nv_rs.demo")]
            pub struct PyStream;

            #[pymethods]
            impl PyStream {
                #[new]
                #[pyo3(signature = (device=0))]
                fn new(device: usize) -> PyResult<Self> {
                    let _ = device;
                    Ok(Self)
                }

                #[getter]
                fn id(&self) -> u32 { 0 }
            }

            #[pymethods]
            impl PyStream {
                #[getter(raw_handle)]
                fn handle_impl(&self) -> u64 { 0 }

                #[setter]
                fn set_label(&mut self, value: &str) { let _ = value; }

                fn __repr__(&self) -> String { String::new() }
            }

            #[pyfunction(name = "ping")]
            fn demo_ping(py: Python<'_>) -> PyResult<bool> {
                let _ = py;
                Ok(true)
            }

            fn register_legacy(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
                let m = PyModule::new(py, "legacy")?;
                m.add("SUPPORTED", cfg!(target_os = "linux"))?;
                m.add_function(wrap_pyfunction!(demo_ping, &m)?)?;
                parent.add_submodule(&m)?;
                Ok(())
            }

            pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
                py_register_submodule!(py, parent, "demo", |m| {
                    m.add("NvDemoError", py.get_type::<DemoError>())?;
                    m.add_class::<PyStream>()?;
                })?;
                register_legacy(py, parent)?;
                Ok(())
            }
        "#;

        let init_py = r#"
from ._nv_rs import (__version__, demo, legacy)
__all__ = ["__version__", "demo", "legacy"]
"#;

        let model = build_synthetic_model(mod_rs, init_py).expect("parses synthetic module");
        assert_eq!(model.submodules.len(), 2);
        assert_eq!(model.submodules[0].name, "demo");
        assert_eq!(model.submodules[1].name, "legacy");

        let pyi = render_pyi(&model);
        assert!(pyi.contains("class NvDemoError(Exception): ..."), "{pyi}");
        assert!(
            pyi.contains("def __init__(self, device: int = 0) -> None: ..."),
            "{pyi}"
        );
        assert!(pyi.contains("def id(self) -> int: ..."), "{pyi}");
        assert!(pyi.contains("def raw_handle(self) -> int: ..."), "{pyi}");
        assert!(
            pyi.contains("@label.setter\n        def label(self, value: str) -> None: ..."),
            "{pyi}"
        );
        assert!(!pyi.contains("__repr__"), "must omit __repr__: {pyi}");
        assert!(pyi.contains("SUPPORTED: bool"), "{pyi}");
        assert!(pyi.contains("def ping() -> bool: ..."), "{pyi}");
    }

    #[test]
    fn applies_py_return_and_py_type_overrides_and_strips_tags_from_docs() {
        let mod_rs = r#"
            use pyo3::prelude::*;
            use pyo3::types::PyDict;

            /// Queries structured telemetry counters.
            /// @py-type filter: str | None
            /// @py-return: dict[str, int]
            #[pyfunction]
            #[pyo3(signature = (filter=None))]
            fn telemetry(py: Python<'_>, filter: Option<&Bound<'_, PyAny>>) -> PyResult<Py<PyDict>> {
                let _ = (py, filter);
                unimplemented!()
            }

            pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
                py_register_submodule!(py, parent, "demo", |m| {
                    m.add_function(wrap_pyfunction!(telemetry, &m)?)?;
                })
            }
        "#;
        let init_py = r#"
from ._nv_rs import (__version__, demo)
__all__ = ["__version__", "demo"]
"#;

        let model = build_synthetic_model(mod_rs, init_py).expect("model builds");
        let pyi = render_pyi(&model);
        let md = render_docs_md(&model);

        assert!(
            pyi.contains("def telemetry(filter: str | None = None) -> dict[str, int]: ..."),
            "{pyi}"
        );
        assert!(
            md.contains("- `telemetry(filter: str | None = None) -> dict[str, int]`"),
            "{md}"
        );
        assert!(!md.contains("@py-return"), "raw tag leaked into docs: {md}");
        assert!(!md.contains("@py-type"), "raw tag leaked into docs: {md}");
    }

    #[test]
    fn renders_staticmethod_and_context_manager_exit_bool() {
        let mod_rs = r#"
            use pyo3::prelude::*;

            /// Scoped driver resource.
            #[pyclass(name = "DriverSession", module = "nv_rs.demo")]
            pub struct PyDriverSession;

            #[pymethods]
            impl PyDriverSession {
                #[staticmethod]
                fn use_count(py: Python<'_>) -> PyResult<i64> {
                    let _ = py;
                    Ok(1)
                }

                fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
                    slf
                }

                #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
                fn __exit__(
                    &self,
                    py: Python<'_>,
                    exc_type: Option<&Bound<'_, PyAny>>,
                    exc_val: Option<&Bound<'_, PyAny>>,
                    exc_tb: Option<&Bound<'_, PyAny>>,
                ) -> PyResult<bool> {
                    let _ = (py, exc_type, exc_val, exc_tb);
                    Ok(false)
                }
            }

            pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
                py_register_submodule!(py, parent, "demo", |m| {
                    m.add_class::<PyDriverSession>()?;
                })
            }
        "#;
        let init_py = r#"
from ._nv_rs import (__version__, demo)
__all__ = ["__version__", "demo"]
"#;

        let model = build_synthetic_model(mod_rs, init_py).expect("model builds");
        let pyi = render_pyi(&model);
        let md = render_docs_md(&model);

        assert!(
            pyi.contains("        @staticmethod\n        def use_count() -> int: ..."),
            "{pyi}"
        );
        assert!(
            pyi.contains("        def __enter__(self) -> \"demo.DriverSession\": ..."),
            "{pyi}"
        );
        assert!(
            pyi.contains(
                "        def __exit__(self, exc_type: Any = None, exc_val: Any = None, exc_tb: Any = None) -> bool: ..."
            ),
            "{pyi}"
        );
        assert!(md.contains("- `@staticmethod use_count() -> int`"), "{md}");
        assert!(
            md.contains(
                "- `__exit__(self, exc_type: Any = None, exc_val: Any = None, exc_tb: Any = None) -> bool`"
            ),
            "{md}"
        );
    }

    #[test]
    fn maps_input_sequences_and_output_lists_cleanly() {
        let mod_rs = r#"
            use std::collections::HashMap;
            use pyo3::prelude::*;

            #[pyfunction]
            #[pyo3(signature = (dims, fixed, view, payload, env=None))]
            fn run_op(
                dims: Vec<usize>,
                fixed: [usize; 4],
                view: &[i64],
                payload: &[u8],
                env: Option<HashMap<String, Option<String>>>,
            ) -> PyResult<Vec<usize>> {
                let _ = (dims, fixed, view, payload, env);
                Ok(vec![1, 2])
            }

            pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
                py_register_submodule!(py, parent, "demo", |m| {
                    m.add_function(wrap_pyfunction!(run_op, &m)?)?;
                })
            }
        "#;
        let init_py = r#"
from ._nv_rs import (__version__, demo)
__all__ = ["__version__", "demo"]
"#;

        let model = build_synthetic_model(mod_rs, init_py).expect("model builds");
        let pyi = render_pyi(&model);

        assert!(
            pyi.contains("from collections.abc import Mapping, Sequence"),
            "{pyi}"
        );
        assert!(
            pyi.contains(
                "def run_op(dims: Sequence[int], fixed: Sequence[int], view: Sequence[int], payload: bytes, env: Mapping[str, str | None] | None = None) -> list[int]: ..."
            ),
            "{pyi}"
        );
    }

    #[test]
    fn roundtrip_generate_and_check_detects_drift() {
        let root = temp_fixture_dir("roundtrip");
        let src_dir = root.join("python").join("nv-py").join("src");
        let pkg_dir = root
            .join("python")
            .join("nv-py")
            .join("python")
            .join("nv_rs");
        std::fs::create_dir_all(&src_dir).expect("create src");
        std::fs::create_dir_all(&pkg_dir).expect("create pkg");

        std::fs::write(
            src_dir.join("lib.rs"),
            r#"
            use pyo3::prelude::*;
            #[pymodule]
            fn _nv_rs(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
                m.add("__version__", env!("CARGO_PKG_VERSION"))?;
                diag::register(py, m)?;
                Ok(())
            }
            "#,
        )
        .expect("write lib.rs");

        std::fs::write(
            src_dir.join("diag.rs"),
            r#"
            use pyo3::prelude::*;
            #[pyclass(name = "TraceGuard")]
            pub struct TraceGuard;
            #[pymethods]
            impl TraceGuard {
                #[new]
                fn new() -> Self { Self }
            }
            pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
                py_register_submodule!(py, parent, "diag", |m| {
                    m.add_class::<TraceGuard>()?;
                })
            }
            "#,
        )
        .expect("write diag.rs");

        std::fs::write(
            pkg_dir.join("__init__.py"),
            r#"from ._nv_rs import (__version__, diag)

def trace(level: str = "trace") -> diag.TraceGuard:
    """Open a trace guard."""
    return diag.TraceGuard()

diag.trace = trace  # type: ignore[attr-defined]

__all__ = ["__version__", "diag", "trace"]
"#,
        )
        .expect("write __init__.py");

        generate(&root).expect("generate succeeds");
        check(&root).expect("check succeeds immediately after generate");

        let pyi_path = root.join(PYI_REL_PATH);
        let pyi_text = std::fs::read_to_string(&pyi_path).expect("read pyi");
        assert!(
            pyi_text.contains("def trace(level: str = \"trace\") -> \"diag.TraceGuard\": ..."),
            "monkey-patched diag.trace inside class diag must quote self-submod return: {pyi_text}"
        );
        assert!(
            pyi_text.contains("\ndef trace(level: str = \"trace\") -> diag.TraceGuard: ..."),
            "top-level trace after class diag must unquote diag.TraceGuard: {pyi_text}"
        );

        std::fs::write(&pyi_path, "# drifted\n").expect("corrupt pyi");
        let err = check(&root).expect_err("stale pyi must fail check");
        assert!(err.contains("__init__.pyi is stale"), "{err}");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn workspace_nv_py_builds_deterministic_model() {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repo root");
        let model = build_api_model(repo_root).expect("real python/nv-py parses cleanly");
        assert_eq!(
            model.submodules.len(),
            25,
            "expected all 25 nv_rs submodules"
        );

        let pyi_a = render_pyi(&model);
        let pyi_b = render_pyi(&model);
        assert_eq!(pyi_a, pyi_b, "pyi rendering must be deterministic");

        let md_a = render_docs_md(&model);
        let md_b = render_docs_md(&model);
        assert_eq!(md_a, md_b, "docs rendering must be deterministic");

        // Key audit regression checks:
        assert!(
            pyi_a.contains("def stream(self) -> \"core.Stream | None\": ..."),
            "DeviceBuffer.stream property must be present in stub"
        );
        assert!(
            pyi_a.contains("@staticmethod\n        def use_count() -> int: ..."),
            "CufileDriver.use_count must be @staticmethod without self"
        );
        assert!(
            !pyi_a.contains("_exc_type"),
            "__exit__ parameters must not have leading underscore"
        );
        assert!(
            pyi_a.contains("shape: Sequence[int]"),
            "Vec/array shape inputs must map to Sequence[int]"
        );
    }
}
