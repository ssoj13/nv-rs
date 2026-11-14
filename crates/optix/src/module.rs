//! [`Module`]s (compiled device code) and [`ProgramGroup`]s (entry points bound for a pipeline).
//!
//! **What.** A module is compiled from PTX or OptiX-IR under [`ModuleOptions`] and
//! [`PipelineOptions`]; the same `PipelineOptions` must be used for every module of a pipeline
//! and for the pipeline itself (OptiX rejects a mismatch). [`ModuleOptions`] carries the
//! optimisation/debug levels, typed payload layouts ([`PayloadType`], one
//! [`PayloadSemantics`] per 32-bit word), launch-parameter values baked in as constants
//! ([`BoundValue`]) and an optional base module. [`Module::builtin_intersection`] returns
//! OptiX's intersection module for curves and spheres ([`BuiltinIs`]). Program groups name entry
//! points by their `__raygen__*`, `__miss__*`, `__closesthit__*`, `__anyhit__*`,
//! `__intersection__*`, `__direct_callable__*` and `__continuation_callable__*` symbols.
//! **How.** [`CompileOpts`] turns both option sets into the raw structs and owns copies of every
//! array they point at, so the pointers stay valid as long as it lives (the task-based compile in
//! `task.rs` keeps it alive across parallel task execution).
//! **Lifetimes.** A program group holds `Arc`s of its modules; a module holds the context and
//! its base module.

use crate::{DeviceContext, Error, LOG_BYTES, Result, check, check_log, entry, sys};
use cuda_core::bytemuck::{self, Pod};
use std::ffi::{CStr, CString, c_char};
use std::ops::BitOr;
use std::sync::Arc;

/// `OptixCompileOptimizationLevel`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OptLevel {
    /// OptiX's default (currently level 3).
    #[default]
    Default,
    /// No optimisation.
    L0,
    /// Level 1.
    L1,
    /// Level 2.
    L2,
    /// Level 3.
    L3,
}

impl OptLevel {
    fn to_sys(self) -> sys::OptixCompileOptimizationLevel {
        match self {
            Self::Default => sys::OptixCompileOptimizationLevel_OPTIX_COMPILE_OPTIMIZATION_DEFAULT,
            Self::L0 => sys::OptixCompileOptimizationLevel_OPTIX_COMPILE_OPTIMIZATION_LEVEL_0,
            Self::L1 => sys::OptixCompileOptimizationLevel_OPTIX_COMPILE_OPTIMIZATION_LEVEL_1,
            Self::L2 => sys::OptixCompileOptimizationLevel_OPTIX_COMPILE_OPTIMIZATION_LEVEL_2,
            Self::L3 => sys::OptixCompileOptimizationLevel_OPTIX_COMPILE_OPTIMIZATION_LEVEL_3,
        }
    }
}

/// `OptixCompileDebugLevel`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DebugLevel {
    /// OptiX's default (minimal).
    #[default]
    Default,
    /// No debug information.
    None,
    /// Line information only (profiling).
    Minimal,
    /// Line information plus some variables; keeps most optimisation.
    Moderate,
    /// Full debug information (for Nsight; pair with [`OptLevel::L0`]).
    Full,
}

impl DebugLevel {
    fn to_sys(self) -> sys::OptixCompileDebugLevel {
        match self {
            Self::Default => sys::OptixCompileDebugLevel_OPTIX_COMPILE_DEBUG_LEVEL_DEFAULT,
            Self::None => sys::OptixCompileDebugLevel_OPTIX_COMPILE_DEBUG_LEVEL_NONE,
            Self::Minimal => sys::OptixCompileDebugLevel_OPTIX_COMPILE_DEBUG_LEVEL_MINIMAL,
            Self::Moderate => sys::OptixCompileDebugLevel_OPTIX_COMPILE_DEBUG_LEVEL_MODERATE,
            Self::Full => sys::OptixCompileDebugLevel_OPTIX_COMPILE_DEBUG_LEVEL_FULL,
        }
    }
}

/// Who may read or write one payload word (`OptixPayloadSemantics`); combine with `|`.
///
/// A word must be writable by the trace caller or some program, and read after being written.
/// `repr(transparent)` over `u32`, so a slice of these is the array OptiX reads.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PayloadSemantics(pub u32);

impl PayloadSemantics {
    /// Nobody touches the word.
    pub const NONE: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_TRACE_CALLER_NONE as u32);
    /// The `optixTrace` caller reads the word after the call.
    pub const CALLER_READ: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_TRACE_CALLER_READ as u32);
    /// The `optixTrace` caller's value is consumed by programs.
    pub const CALLER_WRITE: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_TRACE_CALLER_WRITE as u32);
    /// Both caller directions.
    pub const CALLER_RW: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_TRACE_CALLER_READ_WRITE as u32);
    /// Closest hit reads.
    pub const CH_READ: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_CH_READ as u32);
    /// Closest hit writes.
    pub const CH_WRITE: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_CH_WRITE as u32);
    /// Closest hit reads and writes.
    pub const CH_RW: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_CH_READ_WRITE as u32);
    /// Miss reads.
    pub const MS_READ: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_MS_READ as u32);
    /// Miss writes.
    pub const MS_WRITE: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_MS_WRITE as u32);
    /// Miss reads and writes.
    pub const MS_RW: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_MS_READ_WRITE as u32);
    /// Any hit reads.
    pub const AH_READ: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_AH_READ as u32);
    /// Any hit writes.
    pub const AH_WRITE: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_AH_WRITE as u32);
    /// Any hit reads and writes.
    pub const AH_RW: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_AH_READ_WRITE as u32);
    /// Intersection reads.
    pub const IS_READ: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_IS_READ as u32);
    /// Intersection writes.
    pub const IS_WRITE: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_IS_WRITE as u32);
    /// Intersection reads and writes.
    pub const IS_RW: Self =
        Self(sys::OptixPayloadSemantics_OPTIX_PAYLOAD_SEMANTICS_IS_READ_WRITE as u32);
    /// Everyone reads and writes (the layout `numPayloadValues` implies).
    pub const ALL_RW: Self =
        Self(Self::CALLER_RW.0 | Self::CH_RW.0 | Self::MS_RW.0 | Self::AH_RW.0 | Self::IS_RW.0);
}

impl BitOr for PayloadSemantics {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// One payload layout (`OptixPayloadType`): the semantics of each 32-bit word, 1..=32 words.
/// Device code selects it with `optixSetPayloadTypes` / the payload-type argument of
/// `optixTrace` (`OPTIX_PAYLOAD_TYPE_ID_<index>`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PayloadType<'a> {
    /// Per-word semantics.
    pub words: &'a [PayloadSemantics],
}

/// A launch-parameter range compiled into the module as a constant
/// (`OptixModuleCompileBoundValueEntry`). Validation mode checks it against the real launch
/// parameters; overlapping bound values of modules in one pipeline must agree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundValue<'a> {
    /// Byte offset inside the launch-parameter struct.
    pub offset: usize,
    /// The value (non-empty).
    pub bytes: &'a [u8],
    /// Text OptiX shows when reporting a mismatch.
    pub annotation: Option<&'a CStr>,
}

impl<'a> BoundValue<'a> {
    /// Binds the field at `offset` to `value`.
    pub fn new<T: Pod>(offset: usize, value: &'a T) -> Self {
        Self {
            offset,
            bytes: bytemuck::bytes_of(value),
            annotation: None,
        }
    }
}

/// Per-module compile options.
#[derive(Clone, Copy, Default)]
pub struct ModuleOptions<'a> {
    /// Register limit (0 = OptiX default).
    pub max_registers: i32,
    /// Optimisation level.
    pub opt_level: OptLevel,
    /// Debug information.
    pub debug_level: DebugLevel,
    /// Payload layouts (at most `OPTIX_COMPILE_DEFAULT_MAX_PAYLOAD_TYPE_COUNT`); requires
    /// [`PipelineOptions::payload_values`] = 0.
    pub payload_types: &'a [PayloadType<'a>],
    /// Launch-parameter ranges baked in as constants.
    pub bound_values: &'a [BoundValue<'a>],
    /// Base module for specialisation (same device context); the new module holds it.
    pub base: Option<&'a Arc<Module>>,
}

impl ModuleOptions<'_> {
    /// No optimisation, full debug information (for Nsight).
    pub fn debug() -> Self {
        Self {
            opt_level: OptLevel::L0,
            debug_level: DebugLevel::Full,
            ..Self::default()
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
    /// Payload registers used by `optixTrace` (0..=32); 0 when modules declare payload types.
    pub payload_values: u32,
    /// Attribute registers reported by intersection programs (2 for built-in triangles).
    pub attribute_values: u32,
    /// Only a single GAS is traced (faster) (`OPTIX_TRAVERSABLE_GRAPH_FLAG_ALLOW_SINGLE_GAS`).
    pub single_gas: bool,
    /// Only one IAS level over GASes is traced
    /// (`OPTIX_TRAVERSABLE_GRAPH_FLAG_ALLOW_SINGLE_LEVEL_INSTANCING`); with `single_gas` both
    /// shapes are allowed; neither allows any graph.
    pub single_level_instancing: bool,
    /// Motion blur (motion transforms, motion structures) may be traced.
    pub motion_blur: bool,
    /// Opacity micromaps may be used.
    pub opacity_micromaps: bool,
    /// Cluster acceleration structures may be traced (required when present).
    pub clustered_geometry: bool,
    /// `OptixPrimitiveTypeFlags` the pipeline traces (default: triangles; 0 = custom + triangles).
    pub primitive_types: u32,
    /// `OptixExceptionFlags` (default: none).
    pub exceptions: u32,
}

impl PipelineOptions {
    /// Triangles, any traversable graph, no motion, no exceptions; `params` is the
    /// launch-parameter symbol.
    pub fn new(params_name: &str, params_size: usize, payload_values: u32) -> Result<Self> {
        Ok(Self {
            params_name: CString::new(params_name)
                .map_err(|_| Error::Invalid("params_name contains NUL"))?,
            params_size,
            payload_values,
            attribute_values: 2,
            single_gas: false,
            single_level_instancing: false,
            motion_blur: false,
            opacity_micromaps: false,
            clustered_geometry: false,
            primitive_types: sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_TRIANGLE
                as u32,
            exceptions: sys::OptixExceptionFlags_OPTIX_EXCEPTION_FLAG_NONE as u32,
        })
    }

    /// The raw struct; borrows `params_name`.
    pub(crate) fn to_sys(&self) -> Result<sys::OptixPipelineCompileOptions> {
        if self.payload_values > sys::OPTIX_COMPILE_DEFAULT_MAX_PAYLOAD_VALUE_COUNT
            || self.attribute_values > 8
        {
            return Err(Error::Invalid("at most 32 payload and 8 attribute values"));
        }
        let mut graph =
            sys::OptixTraversableGraphFlags_OPTIX_TRAVERSABLE_GRAPH_FLAG_ALLOW_ANY as u32;
        if self.single_gas {
            graph |= sys::OptixTraversableGraphFlags_OPTIX_TRAVERSABLE_GRAPH_FLAG_ALLOW_SINGLE_GAS
                as u32;
        }
        if self.single_level_instancing {
            graph |= sys::OptixTraversableGraphFlags_OPTIX_TRAVERSABLE_GRAPH_FLAG_ALLOW_SINGLE_LEVEL_INSTANCING as u32;
        }
        Ok(sys::OptixPipelineCompileOptions {
            usesMotionBlur: i32::from(self.motion_blur),
            traversableGraphFlags: graph,
            numPayloadValues: self.payload_values as i32,
            numAttributeValues: self.attribute_values as i32,
            exceptionFlags: self.exceptions,
            pipelineLaunchParamsVariableName: self.params_name.as_ptr(),
            pipelineLaunchParamsSizeInBytes: self.params_size,
            usesPrimitiveTypeFlags: self.primitive_types,
            allowOpacityMicromaps: i32::from(self.opacity_micromaps),
            allowClusteredGeometry: i32::from(self.clustered_geometry),
        })
    }
}

/// Raw module + pipeline compile options plus owned copies of everything they point at.
///
/// Heap storage does not move with the struct, so the raw pointers stay valid for its life.
pub(crate) struct CompileOpts {
    pub(crate) mco: sys::OptixModuleCompileOptions,
    pub(crate) pco: sys::OptixPipelineCompileOptions,
    pub(crate) base: Option<Arc<Module>>,
    _types: Vec<sys::OptixPayloadType>,
    _words: Vec<Box<[u32]>>,
    _bounds: Vec<sys::OptixModuleCompileBoundValueEntry>,
    _bytes: Vec<Box<[u8]>>,
    _notes: Vec<CString>,
    _pipeline: Box<PipelineOptions>,
}

// SAFETY: the raw pointers only reach heap data owned by this struct, which is never mutated
// after construction; OptiX only reads it.
unsafe impl Send for CompileOpts {}
// SAFETY: as above (shared reads only).
unsafe impl Sync for CompileOpts {}

impl CompileOpts {
    /// Validates and converts the options of a module compiled on `ctx`.
    pub(crate) fn new(
        ctx: &Arc<DeviceContext>,
        module: &ModuleOptions<'_>,
        pipeline: &PipelineOptions,
    ) -> Result<Self> {
        if module.payload_types.len() > sys::OPTIX_COMPILE_DEFAULT_MAX_PAYLOAD_TYPE_COUNT as usize {
            return Err(Error::Invalid("at most 8 payload types"));
        }
        if !module.payload_types.is_empty() && pipeline.payload_values != 0 {
            return Err(Error::Invalid(
                "payload types need PipelineOptions::payload_values = 0",
            ));
        }
        if let Some(base) = module.base
            && !Arc::ptr_eq(&base.ctx, ctx)
        {
            return Err(Error::Invalid("base module from another device context"));
        }
        let pipeline = Box::new(pipeline.clone());
        let pco = pipeline.to_sys()?;

        let mut words = Vec::with_capacity(module.payload_types.len());
        for p in module.payload_types {
            if p.words.is_empty()
                || p.words.len() > sys::OPTIX_COMPILE_DEFAULT_MAX_PAYLOAD_VALUE_COUNT as usize
            {
                return Err(Error::Invalid("a payload type has 1..=32 words"));
            }
            words.push(p.words.iter().map(|w| w.0).collect::<Box<[u32]>>());
        }
        let types: Vec<sys::OptixPayloadType> = words
            .iter()
            .map(|w| sys::OptixPayloadType {
                numPayloadValues: w.len() as u32,
                payloadSemantics: w.as_ptr(),
            })
            .collect();

        let mut bytes = Vec::with_capacity(module.bound_values.len());
        let mut notes = Vec::new();
        let mut bounds = Vec::with_capacity(module.bound_values.len());
        for b in module.bound_values {
            let end = b
                .offset
                .checked_add(b.bytes.len())
                .ok_or(Error::Invalid("bound value range overflows"))?;
            if b.bytes.is_empty() || end > pipeline.params_size {
                return Err(Error::Invalid(
                    "bound value must be non-empty and inside the launch parameters",
                ));
            }
            let copy: Box<[u8]> = b.bytes.into();
            let note = b.annotation.map(CString::from);
            bounds.push(sys::OptixModuleCompileBoundValueEntry {
                pipelineParamOffsetInBytes: b.offset,
                sizeInBytes: copy.len(),
                boundValuePtr: copy.as_ptr().cast(),
                annotation: note.as_ref().map_or(std::ptr::null(), |n| n.as_ptr()),
            });
            bytes.push(copy);
            notes.extend(note);
        }

        let base = module.base.cloned();
        let num_bounds =
            u32::try_from(bounds.len()).map_err(|_| Error::Invalid("too many bound values"))?;
        let mco = sys::OptixModuleCompileOptions {
            maxRegisterCount: module.max_registers,
            optLevel: module.opt_level.to_sys(),
            debugLevel: module.debug_level.to_sys(),
            boundValues: if bounds.is_empty() {
                std::ptr::null()
            } else {
                bounds.as_ptr()
            },
            numBoundValues: num_bounds,
            // At most 8, checked above.
            numPayloadTypes: types.len() as u32,
            payloadTypes: if types.is_empty() {
                std::ptr::null()
            } else {
                types.as_ptr()
            },
            baseModule: base.as_ref().map_or(std::ptr::null_mut(), |m| m.raw()),
        };
        Ok(Self {
            mco,
            pco,
            base,
            _types: types,
            _words: words,
            _bounds: bounds,
            _bytes: bytes,
            _notes: notes,
            _pipeline: pipeline,
        })
    }
}

/// A built-in primitive with an OptiX-provided intersection program (`OptixPrimitiveType`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Primitive {
    /// Round quadratic B-spline curves.
    RoundQuadraticBspline,
    /// Round cubic B-spline curves.
    RoundCubicBspline,
    /// Round linear curves.
    RoundLinear,
    /// Round Catmull-Rom curves.
    RoundCatmullRom,
    /// Flat (ribbon) quadratic B-spline curves.
    FlatQuadraticBspline,
    /// Spheres.
    Sphere,
    /// Round cubic Bezier curves.
    RoundCubicBezier,
    /// Round quadratic B-spline curves, rotation-orthogonal caps.
    RoundQuadraticBsplineRocaps,
    /// Round cubic B-spline curves, rotation-orthogonal caps.
    RoundCubicBsplineRocaps,
    /// Round Catmull-Rom curves, rotation-orthogonal caps.
    RoundCatmullRomRocaps,
    /// Round cubic Bezier curves, rotation-orthogonal caps.
    RoundCubicBezierRocaps,
    /// Triangles.
    Triangle,
}

impl Primitive {
    /// `(OptixPrimitiveType, OptixPrimitiveTypeFlags bit)`.
    fn to_sys(self) -> (sys::OptixPrimitiveType, u32) {
        use Primitive::*;
        let (t, f) = match self {
            RoundQuadraticBspline => (
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_QUADRATIC_BSPLINE,
                sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_QUADRATIC_BSPLINE,
            ),
            RoundCubicBspline => (sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_CUBIC_BSPLINE, sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CUBIC_BSPLINE),
            RoundLinear => (sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_LINEAR, sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_LINEAR),
            RoundCatmullRom => (sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_CATMULLROM, sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CATMULLROM),
            FlatQuadraticBspline => (
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_FLAT_QUADRATIC_BSPLINE,
                sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_FLAT_QUADRATIC_BSPLINE,
            ),
            Sphere => (sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_SPHERE, sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_SPHERE),
            RoundCubicBezier => (sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_CUBIC_BEZIER, sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CUBIC_BEZIER),
            RoundQuadraticBsplineRocaps => (
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_QUADRATIC_BSPLINE_ROCAPS,
                sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_QUADRATIC_BSPLINE_ROCAPS,
            ),
            RoundCubicBsplineRocaps => (
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_CUBIC_BSPLINE_ROCAPS,
                sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CUBIC_BSPLINE_ROCAPS,
            ),
            RoundCatmullRomRocaps => (
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_CATMULLROM_ROCAPS,
                sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CATMULLROM_ROCAPS,
            ),
            RoundCubicBezierRocaps => (
                sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_CUBIC_BEZIER_ROCAPS,
                sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CUBIC_BEZIER_ROCAPS,
            ),
            Triangle => (sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_TRIANGLE, sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_TRIANGLE),
        };
        (t, f as u32)
    }

    fn is_curve(self) -> bool {
        !matches!(self, Self::Sphere | Self::Triangle)
    }
}

/// What [`Module::builtin_intersection`] returns a module for (`OptixBuiltinISOptions`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BuiltinIs {
    /// The primitive type.
    pub primitive: Primitive,
    /// The GAS uses vertex motion blur (not motion transforms).
    pub motion_blur: bool,
    /// `OptixBuildFlags` the GAS of these primitives is built with (they must match, e.g.
    /// `ALLOW_RANDOM_VERTEX_ACCESS`, `PREFER_FAST_TRACE`).
    pub build_flags: u32,
    /// Curves get end caps (`OPTIX_CURVE_ENDCAP_ON`); must be false for spheres and triangles.
    pub endcaps: bool,
}

/// Compiled device code.
pub struct Module {
    raw: sys::OptixModule,
    ctx: Arc<DeviceContext>,
    _base: Option<Arc<Module>>,
}

// SAFETY: an OptixModule is an immutable handle once created.
unsafe impl Send for Module {}
// SAFETY: as above.
unsafe impl Sync for Module {}

impl Module {
    /// Compiles PTX text or OptiX-IR bytes. Returns the module and the compiler log.
    pub fn new(
        ctx: &Arc<DeviceContext>,
        module: &ModuleOptions<'_>,
        pipeline: &PipelineOptions,
        code: &[u8],
    ) -> Result<(Arc<Self>, String)> {
        let t = ctx.table();
        let opts = CompileOpts::new(ctx, module, pipeline)?;
        let mut log = vec![0u8; LOG_BYTES];
        let mut log_len = log.len();
        let mut raw = std::ptr::null_mut();
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `opts` owns everything its raw structs point at; code and log buffer outlive
        // the call.
        let code_r = unsafe {
            entry!(t, optixModuleCreate)(
                ctx.raw(),
                &opts.mco,
                &opts.pco,
                code.as_ptr().cast(),
                code.len(),
                log.as_mut_ptr().cast(),
                &mut log_len,
                &mut raw,
            )
        };
        check_log("optixModuleCreate", code_r, &log, log_len)?;
        Ok((Self::wrap(raw, ctx, opts.base), log_text(&log, log_len)))
    }

    /// OptiX's intersection module for a built-in primitive: use it as the hit group's
    /// intersection module with no entry name ([`HitGroup::builtin`]). `module` and `pipeline`
    /// must be the options of the pipeline's other modules.
    pub fn builtin_intersection(
        ctx: &Arc<DeviceContext>,
        module: &ModuleOptions<'_>,
        pipeline: &PipelineOptions,
        is: BuiltinIs,
    ) -> Result<Arc<Self>> {
        let (kind, flag) = is.primitive.to_sys();
        if pipeline.primitive_types != 0 && pipeline.primitive_types & flag == 0 {
            return Err(Error::Invalid(
                "PipelineOptions::primitive_types lacks the built-in primitive",
            ));
        }
        if is.endcaps && !is.primitive.is_curve() {
            return Err(Error::Invalid("end caps apply to curves only"));
        }
        let opts = CompileOpts::new(ctx, module, pipeline)?;
        let bis = sys::OptixBuiltinISOptions {
            builtinISModuleType: kind,
            usesMotionBlur: i32::from(is.motion_blur),
            buildFlags: is.build_flags,
            curveEndcapFlags: if is.endcaps {
                sys::OptixCurveEndcapFlags_OPTIX_CURVE_ENDCAP_ON as u32
            } else {
                sys::OptixCurveEndcapFlags_OPTIX_CURVE_ENDCAP_DEFAULT as u32
            },
        };
        let mut raw = std::ptr::null_mut();
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `opts` owns what its raw structs point at; `bis` outlives the call.
        check("optixBuiltinISModuleGet", unsafe {
            entry!(ctx.table(), optixBuiltinISModuleGet)(
                ctx.raw(),
                &opts.mco,
                &opts.pco,
                &bis,
                &mut raw,
            )
        })?;
        Ok(Self::wrap(raw, ctx, opts.base))
    }

    /// Takes ownership of a created module handle.
    pub(crate) fn wrap(
        raw: sys::OptixModule,
        ctx: &Arc<DeviceContext>,
        base: Option<Arc<Module>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            raw,
            ctx: ctx.clone(),
            _base: base,
        })
    }

    /// The raw handle.
    pub fn raw(&self) -> sys::OptixModule {
        self.raw
    }

    /// The context this module was compiled on.
    pub fn context(&self) -> &Arc<DeviceContext> {
        &self.ctx
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        if let Some(destroy) = self.ctx.table().optixModuleDestroy {
            // SAFETY: owned handle; program groups and modules specialised from it hold an `Arc`
            // of `self`, and task-based compiles hold one until their tasks are gone.
            let code = unsafe { destroy(self.raw) };
            self.ctx.record(code);
        }
    }
}

pub(crate) fn log_text(log: &[u8], len: usize) -> String {
    let text = &log[..len.min(log.len())];
    String::from_utf8_lossy(text.split(|b| *b == 0).next().unwrap_or_default())
        .trim()
        .to_owned()
}

/// The programs of a hit group; any may be absent (built-in triangles need no intersection).
#[derive(Clone, Default)]
pub struct HitGroup<'a> {
    /// `__closesthit__*`.
    pub closest_hit: Option<(&'a Arc<Module>, &'a str)>,
    /// `__anyhit__*`.
    pub any_hit: Option<(&'a Arc<Module>, &'a str)>,
    /// `__intersection__*`, or the empty name for a [`Module::builtin_intersection`] module.
    pub intersection: Option<(&'a Arc<Module>, &'a str)>,
}

impl<'a> HitGroup<'a> {
    /// A hit group over built-in primitives: `closest_hit` (if any) plus the built-in
    /// intersection module `is`.
    pub fn builtin(closest_hit: Option<(&'a Arc<Module>, &'a str)>, is: &'a Arc<Module>) -> Self {
        Self {
            closest_hit,
            any_hit: None,
            intersection: Some((is, "")),
        }
    }
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

/// Entry name kept as a C string next to its module; the empty name stays empty (built-in
/// intersection modules take a null entry name).
fn named(m: Option<(&Arc<Module>, &str)>) -> Result<Option<(Arc<Module>, Option<CString>)>> {
    m.map(|(m, n)| {
        let name = if n.is_empty() {
            None
        } else {
            Some(CString::new(n).map_err(|_| Error::Invalid("entry name contains NUL"))?)
        };
        Ok((m.clone(), name))
    })
    .transpose()
}

fn raw_of(m: &Option<(Arc<Module>, Option<CString>)>) -> (sys::OptixModule, *const c_char) {
    m.as_ref()
        .map_or((std::ptr::null_mut(), std::ptr::null()), |(m, n)| {
            (m.raw(), n.as_ref().map_or(std::ptr::null(), |n| n.as_ptr()))
        })
}

/// The programs of one group, for [`ProgramGroup::new`].
#[derive(Clone)]
pub enum Programs<'a> {
    /// A ray-generation program.
    Raygen(&'a Arc<Module>, &'a str),
    /// A miss program.
    Miss(&'a Arc<Module>, &'a str),
    /// An exception program.
    Exception(&'a Arc<Module>, &'a str),
    /// A hit group.
    HitGroup(HitGroup<'a>),
    /// Direct and / or continuation callables.
    Callables {
        /// `__direct_callable__*`.
        direct: Option<(&'a Arc<Module>, &'a str)>,
        /// `__continuation_callable__*`.
        continuation: Option<(&'a Arc<Module>, &'a str)>,
    },
}

impl ProgramGroup {
    /// A program group whose programs use `payload_type` (`OptixProgramGroupOptions::payloadType`):
    /// required when its modules declare several payload types and OptiX cannot deduce which one
    /// the group uses; `None` lets OptiX deduce it.
    pub fn new(
        ctx: &Arc<DeviceContext>,
        programs: Programs<'_>,
        payload_type: Option<PayloadType<'_>>,
    ) -> Result<Arc<Self>> {
        match programs {
            Programs::Raygen(m, e) => Self::single(
                ctx,
                m,
                e,
                sys::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_RAYGEN,
                payload_type,
            ),
            Programs::Miss(m, e) => Self::single(
                ctx,
                m,
                e,
                sys::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_MISS,
                payload_type,
            ),
            Programs::Exception(m, e) => Self::single(
                ctx,
                m,
                e,
                sys::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_EXCEPTION,
                payload_type,
            ),
            Programs::HitGroup(h) => Self::hits(ctx, h, payload_type),
            Programs::Callables {
                direct,
                continuation,
            } => Self::calls(ctx, direct, continuation, payload_type),
        }
    }

    /// A ray-generation program.
    pub fn raygen(module: &Arc<Module>, entry: &str) -> Result<Arc<Self>> {
        Self::new(&module.ctx, Programs::Raygen(module, entry), None)
    }

    /// A miss program.
    pub fn miss(module: &Arc<Module>, entry: &str) -> Result<Arc<Self>> {
        Self::new(&module.ctx, Programs::Miss(module, entry), None)
    }

    /// An exception program.
    pub fn exception(module: &Arc<Module>, entry: &str) -> Result<Arc<Self>> {
        Self::new(&module.ctx, Programs::Exception(module, entry), None)
    }

    fn single(
        ctx: &Arc<DeviceContext>,
        module: &Arc<Module>,
        entry: &str,
        kind: sys::OptixProgramGroupKind,
        payload: Option<PayloadType<'_>>,
    ) -> Result<Arc<Self>> {
        let name = CString::new(entry).map_err(|_| Error::Invalid("entry name contains NUL"))?;
        let mut desc = sys::OptixProgramGroupDesc {
            kind,
            ..Default::default()
        };
        desc.__bindgen_anon_1.raygen = sys::OptixProgramGroupSingleModule {
            module: module.raw(),
            entryFunctionName: name.as_ptr(),
        };
        Self::create(ctx, &desc, vec![module.clone()], payload)
    }

    /// A hit group (closest hit, any hit, intersection).
    pub fn hit_group(ctx: &Arc<DeviceContext>, programs: HitGroup<'_>) -> Result<Arc<Self>> {
        Self::new(ctx, Programs::HitGroup(programs), None)
    }

    fn hits(
        ctx: &Arc<DeviceContext>,
        programs: HitGroup<'_>,
        payload: Option<PayloadType<'_>>,
    ) -> Result<Arc<Self>> {
        let ch = named(programs.closest_hit)?;
        let ah = named(programs.any_hit)?;
        let is = named(programs.intersection)?;
        let ((m_ch, n_ch), (m_ah, n_ah), (m_is, n_is)) = (raw_of(&ch), raw_of(&ah), raw_of(&is));
        let mut desc = sys::OptixProgramGroupDesc {
            kind: sys::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_HITGROUP,
            ..Default::default()
        };
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
        let modules = [&ch, &ah, &is]
            .into_iter()
            .flatten()
            .map(|(m, _)| m.clone())
            .collect();
        Self::create(ctx, &desc, modules, payload)
    }

    /// A callable program: direct (`__direct_callable__*`) and/or continuation
    /// (`__continuation_callable__*`).
    pub fn callables(
        ctx: &Arc<DeviceContext>,
        direct: Option<(&Arc<Module>, &str)>,
        continuation: Option<(&Arc<Module>, &str)>,
    ) -> Result<Arc<Self>> {
        Self::new(
            ctx,
            Programs::Callables {
                direct,
                continuation,
            },
            None,
        )
    }

    fn calls(
        ctx: &Arc<DeviceContext>,
        direct: Option<(&Arc<Module>, &str)>,
        continuation: Option<(&Arc<Module>, &str)>,
        payload: Option<PayloadType<'_>>,
    ) -> Result<Arc<Self>> {
        let dc = named(direct)?;
        let cc = named(continuation)?;
        let ((m_dc, n_dc), (m_cc, n_cc)) = (raw_of(&dc), raw_of(&cc));
        let mut desc = sys::OptixProgramGroupDesc {
            kind: sys::OptixProgramGroupKind_OPTIX_PROGRAM_GROUP_KIND_CALLABLES,
            ..Default::default()
        };
        desc.__bindgen_anon_1.callables = sys::OptixProgramGroupCallables {
            moduleDC: m_dc,
            entryFunctionNameDC: n_dc,
            moduleCC: m_cc,
            entryFunctionNameCC: n_cc,
        };
        // As in `hit_group`: `dc`/`cc` own the entry names and must outlive `create`.
        let modules = [&dc, &cc]
            .into_iter()
            .flatten()
            .map(|(m, _)| m.clone())
            .collect();
        Self::create(ctx, &desc, modules, payload)
    }

    fn create(
        ctx: &Arc<DeviceContext>,
        desc: &sys::OptixProgramGroupDesc,
        modules: Vec<Arc<Module>>,
        payload: Option<PayloadType<'_>>,
    ) -> Result<Arc<Self>> {
        if modules.iter().any(|m| !Arc::ptr_eq(&m.ctx, ctx)) {
            return Err(Error::Invalid("program group mixes device contexts"));
        }
        let t = ctx.table();
        if payload.is_some_and(|p| p.words.is_empty() || p.words.len() > 32) {
            return Err(Error::Invalid("a payload type has 1..=32 words"));
        }
        // `PayloadSemantics` is `repr(transparent)` over the `u32` OptiX reads; `ty` outlives the call.
        let ty = payload.map(|p| sys::OptixPayloadType {
            numPayloadValues: p.words.len() as u32,
            payloadSemantics: p.words.as_ptr().cast(),
        });
        let opts = sys::OptixProgramGroupOptions {
            payloadType: ty.as_ref().map_or(std::ptr::null(), |t| t as *const _),
        };
        let mut log = vec![0u8; LOG_BYTES];
        let mut log_len = log.len();
        let mut raw = std::ptr::null_mut();
        ctx.cuda().bind_to_thread()?;
        // SAFETY: `desc` points at modules and C strings alive for the call (held by the caller's
        // locals and `modules`).
        let code = unsafe {
            entry!(t, optixProgramGroupCreate)(
                ctx.raw(),
                desc,
                1,
                &opts,
                log.as_mut_ptr().cast(),
                &mut log_len,
                &mut raw,
            )
        };
        check_log("optixProgramGroupCreate", code, &log, log_len)?;
        Ok(Arc::new(Self {
            raw,
            ctx: ctx.clone(),
            _modules: modules,
        }))
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
        check("optixSbtRecordPackHeader", unsafe {
            entry!(self.ctx.table(), optixSbtRecordPackHeader)(self.raw, header.as_mut_ptr().cast())
        })
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
