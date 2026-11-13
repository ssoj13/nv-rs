//! Shared rig of the device test cases: one CUDA + OptiX context, the binary's PTX, pipeline
//! options every case starts from, a launch helper and comparisons.
//!
//! **Why.** All cases live in one PTX; each compiles only its own programs ([`subset_ptx`]), and
//! the register counts that constrain programs (payload, attributes) have one source, [`opts`].
//! **Used by** every `cases::*` module.

use bytemuck::Pod;
use cuda_core::{Context, DeviceBuffer, DeviceCopy, Stream};
use optix::{DeviceContext, LinkOptions, Module, ModuleOptions, Pipeline, PipelineOptions, ProgramGroup, ShaderBindingTable};
use std::fmt::Display;
use std::sync::Arc;

/// A case result: `Err` carries the failure message.
pub type R<T = ()> = Result<T, String>;

/// Attaches context to an error.
pub trait Ctx<T> {
    /// `Err(e)` becomes `Err("what: e")`.
    fn ctx(self, what: &str) -> R<T>;
}

impl<T, E: Display> Ctx<T> for Result<T, E> {
    fn ctx(self, what: &str) -> R<T> {
        self.map_err(|e| format!("{what}: {e}"))
    }
}

/// Payload registers of every pipeline: the most any program of this binary uses.
pub const PAYLOAD: u32 = 8;
/// Attribute registers of every pipeline: the most any intersection program reports.
pub const ATTRS: u32 = 8;

/// CUDA device 0, its default stream, an OptiX context on it and the binary's PTX.
pub struct Gpu {
    /// The CUDA context.
    pub cuda: Arc<Context>,
    /// Its default stream.
    pub stream: Arc<Stream>,
    /// The OptiX context.
    pub ctx: Arc<DeviceContext>,
    /// The PTX cuda-oxide embedded in this binary (every case's programs).
    pub ptx: Vec<u8>,
}

impl Gpu {
    /// Creates the contexts; OptiX messages of level <= 2 go to stderr.
    pub fn new(ptx: Vec<u8>) -> R<Self> {
        let cuda = Context::new(0).ctx("CUDA device 0")?;
        let stream = cuda.default_stream();
        // `ODT_VALIDATION=1` turns on OptiX validation mode (every exception enabled, checked launches).
        let validation = std::env::var("ODT_VALIDATION").is_ok_and(|v| v == "1");
        let ctx = DeviceContext::new(&cuda, Some(Box::new(|l, t, m| eprintln!("[optix {l} {t}] {m}"))), if validation { 4 } else { 2 }, validation).ctx("OptiX context")?;
        if let Some(v) = std::env::var("ODT_CUDA_STACK").ok().and_then(|s| s.parse().ok()) {
            cuda.set_limit(cuda_core::ContextLimit::StackSize, v).ctx("stack limit")?;
        }
        eprintln!("CUDA stack limit {:?}", cuda.limit(cuda_core::ContextLimit::StackSize));
        Ok(Self { cuda, stream, ctx, ptx })
    }

    /// The programs `entries` (and what they call) of the binary's PTX compiled as one module
    /// with `mo` / `opts` (see [`subset_ptx`]).
    pub fn module(&self, mo: &ModuleOptions<'_>, opts: &PipelineOptions, entries: &[&str]) -> R<Arc<Module>> {
        let ptx = subset_ptx(&self.ptx, entries)?;
        let (m, log) = Module::new(&self.ctx, mo, opts, &ptx).ctx("module")?;
        if !log.is_empty() {
            eprintln!("[module log] {log}");
        }
        Ok(m)
    }

    /// Links `groups` into a pipeline.
    pub fn pipeline(&self, opts: &PipelineOptions, link: LinkOptions, groups: &[Arc<ProgramGroup>]) -> R<Pipeline> {
        let (p, log) = Pipeline::new(&self.ctx, opts, link, groups).ctx("pipeline")?;
        if !log.is_empty() {
            eprintln!("[pipeline log] {log}");
        }
        Ok(p)
    }

    /// A device buffer holding `v`.
    pub fn up<T: DeviceCopy>(&self, v: &[T]) -> R<DeviceBuffer<T>> {
        DeviceBuffer::from_host(&self.stream, v).ctx("upload")
    }

    /// A zeroed device buffer of `n` elements.
    pub fn zeros<T: DeviceCopy>(&self, n: usize) -> R<DeviceBuffer<T>> {
        DeviceBuffer::zeroed(&self.stream, n).ctx("allocate")
    }

    /// The contents of `b` (synchronizes the stream).
    pub fn down<T: DeviceCopy>(&self, b: &DeviceBuffer<T>) -> R<Vec<T>> {
        b.to_host_vec(&self.stream).ctx("download")
    }

    /// Launches `pipeline` over `dims` with launch parameters `params` and waits for it.
    ///
    /// # Safety
    ///
    /// As [`Pipeline::launch`]: `P` is the device-side parameter struct, the SBT records hold
    /// what the programs read, every pointer and handle in `params` is live.
    pub unsafe fn launch<P: Pod>(&self, pipeline: &Pipeline, sbt: &ShaderBindingTable, params: &P, dims: [u32; 3]) -> R {
        let dev = self.up(bytemuck::bytes_of(params))?;
        // SAFETY: forwarded contract; `dev` and `sbt` outlive the synchronization below.
        unsafe { pipeline.launch(&self.stream, &dev, sbt, dims) }.ctx("launch")?;
        self.stream.synchronize().ctx("synchronize after launch")
    }
}

/// The function name in a PTX function header (`.visible .entry name(`, `.func (.param ..)
/// name(`, ...): the identifier after the keywords and the optional return-parameter list.
fn func_name(header: &str) -> Option<&str> {
    let mut s = header.trim_start();
    for kw in [".visible", ".weak", ".extern", ".entry", ".func"] {
        s = s.trim_start().strip_prefix(kw).unwrap_or(s);
    }
    s = s.trim_start();
    if let Some(rest) = s.strip_prefix('(') {
        s = &rest[rest.find(')')? + 1..];
    }
    let s = s.trim_start();
    let end = s.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '$')).unwrap_or(s.len());
    (end > 0).then(|| &s[..end])
}

/// `ptx` reduced to the entry functions `entries` and the functions they reach (directly or
/// through other kept functions), with every non-function line (header, globals) kept.
///
/// **Why.** OptiX compiles every function of a module and validates it against the module's
/// options: one program calling `optixSetPayloadTypes` breaks every module without payload
/// types, and plain CUDA kernels must not drag `_optix_*` calls into `cuModuleLoadData`. Each
/// case therefore compiles only its own programs out of the binary's single PTX.
pub fn subset_ptx(ptx: &[u8], entries: &[&str]) -> R<Vec<u8>> {
    let text = std::str::from_utf8(ptx).ctx("PTX is not UTF-8")?;
    // Top-level items: (function name or None, text, is a definition).
    let mut items: Vec<(Option<String>, String, bool)> = Vec::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        let t = line.trim_start();
        let is_func = [".visible .entry", ".entry", ".visible .func", ".func", ".weak .func", ".extern .func"].iter().any(|k| t.starts_with(k));
        if !is_func {
            items.push((None, format!("{line}\n"), false));
            continue;
        }
        let mut header = format!("{line}\n");
        // The header ends with `;` (a declaration) or is followed by the body's `{`.
        let mut is_def = false;
        loop {
            let last = header.lines().last().unwrap_or_default().trim();
            if last.ends_with(';') {
                break;
            }
            if last == "{" || last.ends_with('{') {
                is_def = true;
                break;
            }
            match lines.next() {
                Some(l) => header.push_str(&format!("{l}\n")),
                None => return Err("PTX ends inside a function header".into()),
            }
        }
        let name = func_name(&header).map(str::to_owned).ok_or_else(|| format!("no function name in PTX header {header:?}"))?;
        if is_def {
            // Count braces outside `//` comments until the body closes (inline asm brings its own
            // balanced braces).
            let count = |s: &str| s.split("//").next().unwrap_or_default().chars().fold(0i64, |d, c| d + i64::from(c == '{') - i64::from(c == '}'));
            let mut depth: i64 = header.lines().map(count).sum();
            while depth > 0 {
                let l = lines.next().ok_or_else(|| format!("PTX ends inside function {name}"))?;
                depth += count(l);
                header.push_str(&format!("{l}\n"));
            }
        }
        items.push((Some(name), header, is_def));
    }
    let defined: Vec<&str> = items.iter().filter(|i| i.2).filter_map(|i| i.0.as_deref()).collect();
    for e in entries {
        if !defined.contains(e) {
            return Err(format!("entry {e} not found in the PTX"));
        }
    }
    // Closure over references: a kept body naming another defined function keeps it.
    let mut keep: Vec<&str> = entries.to_vec();
    let mut i = 0;
    while i < keep.len() {
        let body = items.iter().find(|it| it.2 && it.0.as_deref() == Some(keep[i])).map(|it| it.1.as_str()).unwrap_or_default();
        for name in &defined {
            if !keep.contains(name) && body.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '$')).any(|tok| tok == *name) {
                keep.push(name);
            }
        }
        i += 1;
    }
    let out: String = items.iter().filter(|it| it.0.as_deref().is_none_or(|n| keep.contains(&n))).map(|it| it.1.as_str()).collect();
    Ok(out.into_bytes())
}

/// Pipeline options every case starts from: launch parameters `name` of `size` bytes,
/// [`PAYLOAD`] payload and [`ATTRS`] attribute registers, triangles, any graph.
pub fn opts(name: &str, size: usize) -> R<PipelineOptions> {
    let mut o = PipelineOptions::new(name, size, PAYLOAD).ctx("pipeline options")?;
    o.attribute_values = ATTRS;
    Ok(o)
}

/// `Err` listing the first mismatches when any `got[i]` differs from `want[i]` by more than
/// `tol` (absolute), labelled by `label(i)`.
pub fn close(what: &str, got: &[f32], want: &[f32], tol: f32, label: impl Fn(usize) -> String) -> R {
    if got.len() != want.len() {
        return Err(format!("{what}: {} values, expected {}", got.len(), want.len()));
    }
    let bad: Vec<String> = got
        .iter()
        .zip(want)
        .enumerate()
        .filter(|(_, (g, w))| !((*g - *w).abs() <= tol || (g.is_nan() && w.is_nan())))
        .map(|(i, (g, w))| format!("{}: got {g} want {w}", label(i)))
        .collect();
    if bad.is_empty() { Ok(()) } else { Err(format!("{what}: {} mismatches (tol {tol}); first: {:?}", bad.len(), &bad[..bad.len().min(6)])) }
}

/// `Err` listing the first mismatches when `got != want` element-wise.
pub fn same<T: PartialEq + std::fmt::Debug>(what: &str, got: &[T], want: &[T], label: impl Fn(usize) -> String) -> R {
    if got.len() != want.len() {
        return Err(format!("{what}: {} values, expected {}", got.len(), want.len()));
    }
    let bad: Vec<String> = got.iter().zip(want).enumerate().filter(|(_, (g, w))| g != w).map(|(i, (g, w))| format!("{}: got {g:?} want {w:?}", label(i))).collect();
    if bad.is_empty() { Ok(()) } else { Err(format!("{what}: {} mismatches; first: {:?}", bad.len(), &bad[..bad.len().min(6)])) }
}

/// `Err(what())` unless `ok`.
pub fn ensure(ok: bool, what: impl FnOnce() -> String) -> R {
    if ok { Ok(()) } else { Err(what()) }
}
