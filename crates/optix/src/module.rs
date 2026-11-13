//! [`Module`]s (compiled device code) and [`ProgramGroup`]s (entry points bound for a pipeline).
//!
//! **What.** A module is compiled from PTX or OptiX-IR under [`ModuleOptions`] and
//! [`PipelineOptions`]; the same `PipelineOptions` must be used for every module of a pipeline
//! and for the pipeline itself (OptiX rejects a mismatch). Program groups name entry points by
//! their `__raygen__*`, `__miss__*`, `__closesthit__*`, `__anyhit__*`, `__intersection__*`,
//! `__direct_callable__*` and `__continuation_callable__*` symbols.
//! **Lifetimes.** A program group holds `Arc`s of its modules; a module holds the context.

use crate::{DeviceContext, Error, LOG_BYTES, Result, check, check_log, entry, sys};
use std::ffi::{CString, c_char};
use std::sync::Arc;

/// Per-module compile options.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModuleOptions {
    /// Register limit (0 = OptiX default).
    pub max_registers: i32,
    /// Keep line info and turn off optimisation (for Nsight).
    pub debug: bool,
}

impl Default for ModuleOptions {
    /// Optimised, no debug info, default register limit.
    fn default() -> Self {
        Self { max_registers: 0, debug: false }
    }
}

impl ModuleOptions {
    fn to_sys(self) -> sys::OptixModuleCompileOptions {
        sys::OptixModuleCompileOptions {
            maxRegisterCount: self.max_registers,
            optLevel: if self.debug {
                sys::OptixCompileOptimizationLevel_OPTIX_COMPILE_OPTIMIZATION_LEVEL_0
            } else {
                sys::OptixCompileOptimizationLevel_OPTIX_COMPILE_OPTIMIZATION_DEFAULT
            },
            debugLevel: if self.debug {
                sys::OptixCompileDebugLevel_OPTIX_COMPILE_DEBUG_LEVEL_FULL
            } else {
                sys::OptixCompileDebugLevel_OPTIX_COMPILE_DEBUG_LEVEL_DEFAULT
            },
            ..Default::default()
        }
    }
}

/// Options shared by every module of a pipeline and by the pipeline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineOptions {
    /// `.const` symbol holding the launch parameters (cuda-oxide:
    /// `#[constant(export_name = "...")]`).
    pub params_name: CString,
    /// `size_of` the launch-parameter struct.
    pub params_size: usize,
    /// Payload registers used by `optixTrace` (0..=32).
    pub payload_values: u32,
    /// Attribute registers reported by intersection programs (2 for built-in triangles).
    pub attribute_values: u32,
    /// Only a single GAS is traced (faster) instead of any traversable graph.
    pub single_gas: bool,
    /// `OptixPrimitiveTypeFlags` the pipeline traces (default: triangles).
    pub primitive_types: u32,
    /// `OptixExceptionFlags` (default: none).
    pub exceptions: u32,
}

impl PipelineOptions {
    /// Triangles, any traversable graph, no exceptions; `params` is the launch-parameter symbol.
    pub fn new(params_name: &str, params_size: usize, payload_values: u32) -> Result<Self> {
        Ok(Self {
            params_name: CString::new(params_name).map_err(|_| Error::Invalid("params_name contains NUL"))?,
            params_size,
            payload_values,
            attribute_values: 2,
            single_gas: false,
            primitive_types: sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_TRIANGLE as u32,
            exceptions: sys::OptixExceptionFlags_OPTIX_EXCEPTION_FLAG_NONE as u32,
        })
    }

    /// The raw struct; borrows `params_name`.
    pub(crate) fn to_sys(&self) -> Result<sys::OptixPipelineCompileOptions> {
        if self.payload_values > 32 || self.attribute_values > 8 {
            return Err(Error::Invalid("at most 32 payload and 8 attribute values"));
        }
        Ok(sys::OptixPipelineCompileOptions {
            usesMotionBlur: 0,
            traversableGraphFlags: if self.single_gas {
                sys::OptixTraversableGraphFlags_OPTIX_TRAVERSABLE_GRAPH_FLAG_ALLOW_SINGLE_GAS as u32
            } else {
                sys::OptixTraversableGraphFlags_OPTIX_TRAVERSABLE_GRAPH_FLAG_ALLOW_ANY as u32
            },
            numPayloadValues: self.payload_values as i32,
            numAttributeValues: self.attribute_values as i32,
            exceptionFlags: self.exceptions,
            pipelineLaunchParamsVariableName: self.params_name.as_ptr(),
            usesPrimitiveTypeFlags: self.primitive_types,
            pipelineLaunchParamsSizeInBytes: self.params_size,
            ..Default::default()
        })
    }
}

/// Compiled device code.
pub struct Module {
    raw: sys::OptixModule,
    ctx: Arc<DeviceContext>,
}

// SAFETY: an OptixModule is an immutable handle once created.
unsafe impl Send for Module {}
// SAFETY: as above.
unsafe impl Sync for Module {}

impl Module {
    /// Compiles PTX text or OptiX-IR bytes. Returns the module and the compiler log.
    pub fn new(ctx: &Arc<DeviceContext>, module: &ModuleOptions, pipeline: &PipelineOptions, code: &[u8]) -> Result<(Arc<Self>, String)> {
        let t = ctx.table();
        let mco = module.to_sys();
        let pco = pipeline.to_sys()?;
        let mut log = vec![0u8; LOG_BYTES];
        let mut log_len = log.len();
        let mut raw = std::ptr::null_mut();
        ctx.cuda().bind_to_thread()?;
        // SAFETY: options, code and log buffer outlive the call; `pco` borrows `pipeline`.
        let code_r = unsafe {
            entry!(t, optixModuleCreate)(ctx.raw(), &mco, &pco, code.as_ptr().cast(), code.len(), log.as_mut_ptr().cast(), &mut log_len, &mut raw)
        };
        check_log("optixModuleCreate", code_r, &log, log_len)?;
        Ok((Arc::new(Self { raw, ctx: ctx.clone() }), log_text(&log, log_len)))
    }

    /// The raw handle.
    pub fn raw(&self) -> sys::OptixModule {
        self.raw
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        if let Some(destroy) = self.ctx.table().optixModuleDestroy {
            // SAFETY: owned handle; program groups using it hold an `Arc` of `self`.
            let code = unsafe { destroy(self.raw) };
            self.ctx.record(code);
        }
    }
}

pub(crate) fn log_text(log: &[u8], len: usize) -> String {
    let text = &log[..len.min(log.len())];
    String::from_utf8_lossy(text.split(|b| *b == 0).next().unwrap_or_default()).trim().to_owned()
}

/// The programs of a hit group; any may be absent (built-in triangles need no intersection).
#[derive(Clone, Default)]
pub struct HitGroup<'a> {
    /// `__closesthit__*`.
    pub closest_hit: Option<(&'a Arc<Module>, &'a str)>,
    /// `__anyhit__*`.
    pub any_hit: Option<(&'a Arc<Module>, &'a str)>,
    /// `__intersection__*`.
    pub intersection: Option<(&'a Arc<Module>, &'a str)>,
}

/// One or more entry points bound for a pipeline.
pub struct ProgramGroup {
    raw: sys::OptixProgramGroup,
    ctx: Arc<DeviceContext>,
    _modules: Vec<Arc<Module>>,
}

// SAFETY: an OptixProgramGroup is an immutable handle once created.
unsafe impl Send for ProgramGroup {}
// SAFETY: as above.
unsafe impl Sync for ProgramGroup {}

/// Entry name kept as a C string next to its module.
fn named(m: Option<(&Arc<Module>, &str)>) -> Result<Option<(Arc<Module>, CString)>> {
    m.map(|(m, n)| Ok((m.clone(), CString::new(n).map_err(|_| Error::Invalid("entry name contains NUL"))?))).transpose()
}

fn raw_of(m: &Option<(Arc<Module>, CString)>) -> (sys::OptixModule, *const c_char) {
    m.as_ref().map_or((std::ptr::null_mut(), std::ptr::null()), |(m, n)| (m.raw(), n.as_ptr()))
}

impl ProgramGroup {
    /// A ray-generation program.
    pub fn raygen(module: &Arc<Module>, entry: &str) -> Result<Arc<Self>> {
        Self::single(module, entry, sys::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_RAYGEN)
    }

    /// A miss program.
    pub fn miss(module: &Arc<Module>, entry: &str) -> Result<Arc<Self>> {
        Self::single(module, entry, sys::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_MISS)
    }

    /// An exception program.
    pub fn exception(module: &Arc<Module>, entry: &str) -> Result<Arc<Self>> {
        Self::single(module, entry, sys::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_EXCEPTION)
    }

    fn single(module: &Arc<Module>, entry: &str, kind: sys::OptixProgramGroupKind) -> Result<Arc<Self>> {
        let name = CString::new(entry).map_err(|_| Error::Invalid("entry name contains NUL"))?;
        let mut desc = sys::OptixProgramGroupDesc { kind, ..Default::default() };
        desc.__bindgen_anon_1.raygen = sys::OptixProgramGroupSingleModule { module: module.raw(), entryFunctionName: name.as_ptr() };
        Self::create(&module.ctx, &desc, vec![module.clone()])
    }

    /// A hit group (closest hit, any hit, intersection).
    pub fn hit_group(ctx: &Arc<DeviceContext>, programs: HitGroup<'_>) -> Result<Arc<Self>> {
        let ch = named(programs.closest_hit)?;
        let ah = named(programs.any_hit)?;
        let is = named(programs.intersection)?;
        let ((m_ch, n_ch), (m_ah, n_ah), (m_is, n_is)) = (raw_of(&ch), raw_of(&ah), raw_of(&is));
        let mut desc = sys::OptixProgramGroupDesc { kind: sys::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_HITGROUP, ..Default::default() };
        desc.__bindgen_anon_1.hitgroup = sys::OptixProgramGroupHitgroup {
            moduleCH: m_ch,
            entryFunctionNameCH: n_ch,
            moduleAH: m_ah,
            entryFunctionNameAH: n_ah,
            moduleIS: m_is,
            entryFunctionNameIS: n_is,
        };
        // Clone the modules: `ch`/`ah`/`is` own the entry-name strings `desc` points at and must
        // live until `create` returns.
        let modules = [&ch, &ah, &is].into_iter().flatten().map(|(m, _)| m.clone()).collect();
        Self::create(ctx, &desc, modules)
    }

    /// A callable program: direct (`__direct_callable__*`) and/or continuation
    /// (`__continuation_callable__*`).
    pub fn callables(ctx: &Arc<DeviceContext>, direct: Option<(&Arc<Module>, &str)>, continuation: Option<(&Arc<Module>, &str)>) -> Result<Arc<Self>> {
        let dc = named(direct)?;
        let cc = named(continuation)?;
        let ((m_dc, n_dc), (m_cc, n_cc)) = (raw_of(&dc), raw_of(&cc));
        let mut desc = sys::OptixProgramGroupDesc { kind: sys::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_CALLABLES, ..Default::default() };
        desc.__bindgen_anon_1.callables = sys::OptixProgramGroupCallables {
            moduleDC: m_dc,
            entryFunctionNameDC: n_dc,
            moduleCC: m_cc,
            entryFunctionNameCC: n_cc,
        };
        // As in `hit_group`: `dc`/`cc` own the entry names and must outlive `create`.
        let modules = [&dc, &cc].into_iter().flatten().map(|(m, _)| m.clone()).collect();
        Self::create(ctx, &desc, modules)
    }

    fn create(ctx: &Arc<DeviceContext>, desc: &sys::OptixProgramGroupDesc, modules: Vec<Arc<Module>>) -> Result<Arc<Self>> {
        if modules.iter().any(|m| !Arc::ptr_eq(&m.ctx, ctx)) {
            return Err(Error::Invalid("program group mixes device contexts"));
        }
        let t = ctx.table();
        let opts = sys::OptixProgramGroupOptions::default();
        let mut log = vec![0u8; LOG_BYTES];
        let mut log_len = log.len();
        let mut raw = std::ptr::null_mut();
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `desc` points at modules and C strings alive for the call (held by the caller's
        // locals and `modules`).
        let code = unsafe { entry!(t, optixProgramGroupCreate)(ctx.raw(), desc, 1, &opts, log.as_mut_ptr().cast(), &mut log_len, &mut raw) };
        check_log("optixProgramGroupCreate", code, &log, log_len)?;
        Ok(Arc::new(Self { raw, ctx: ctx.clone(), _modules: modules }))
    }

    /// The raw handle.
    pub fn raw(&self) -> sys::OptixProgramGroup {
        self.raw
    }

    /// The context this group was created on.
    pub fn context(&self) -> &Arc<DeviceContext> {
        &self.ctx
    }

    /// Writes this group's 32-byte SBT record header.
    pub(crate) fn pack_header(&self, header: &mut [u8; crate::sbt::HEADER]) -> Result<()> {
        // SAFETY: `header` is a writable 32-byte buffer, the size OptiX writes.
        check("optixSbtRecordPackHeader", unsafe { entry!(self.ctx.table(), optixSbtRecordPackHeader)(self.raw, header.as_mut_ptr().cast()) })
    }
}

impl Drop for ProgramGroup {
    fn drop(&mut self) {
        if let Some(destroy) = self.ctx.table().optixProgramGroupDestroy {
            // SAFETY: owned handle; pipelines using it hold an `Arc` of `self`.
            let code = unsafe { destroy(self.raw) };
            self.ctx.record(code);
        }
    }
}
