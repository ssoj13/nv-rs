//! Shared rig of the device test cases: one CUDA + OptiX context, the binary's PTX, pipeline
//! options every case starts from, a launch helper and comparisons.
//!
//! **Why.** Every case compiles the whole embedded PTX (all cases' programs live in one module),
//! so the options that constrain every program (payload and attribute register counts) must be
//! the same in every case; [`opts`] is their single source. **Used by** every `cases::*` module.

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
        Ok(Self { cuda, stream, ctx, ptx })
    }

    /// The binary's PTX compiled as one module with `mo` / `opts`.
    pub fn module(&self, mo: &ModuleOptions<'_>, opts: &PipelineOptions) -> R<Arc<Module>> {
        let (m, log) = Module::new(&self.ctx, mo, opts, &self.ptx).ctx("module")?;
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
