//! Shared rig for the OptiX GPU feature tests (`gpu_features.rs`).
//!
//! **What.** Builds a CUDA + OptiX context, compiles `fixtures/s1b.ptx` (raygen `__raygen__mask`:
//! one +z ray per pixel of a 16x16 launch from z = -1 at time 0, tmin 0, payload word 0 = 7
//! before the trace; `__closesthit__hit` sets it to 1, `__miss__none` to 0), links a pipeline
//! for one geometry kind (triangles, a built-in curve or sphere intersector, motion, micromaps,
//! clusters), launches it over a traversable and returns the per-pixel hit flags. Analytic masks
//! ([`grid`]) are computed from a signed distance per pixel centre and refuse pixels closer than
//! [`MARGIN`] to an edge, so a mask mismatch is a real failure, never rounding.
//! **Why.** Every geometry feature can then be checked by comparing a traced mask with an
//! analytic one. **Used by** `tests/gpu_features.rs`.

use cuda_core::{Context, DeviceBuffer, Stream};
use optix::{
    BuiltinIs, DeviceContext, HitGroup, LinkOptions, LogFn, Module, ModuleOptions, PayloadType, Pipeline, PipelineOptions, ProgramGroup, Programs, SbtRecord, Scratch,
    ShaderBindingTable,
};
use std::sync::Arc;

/// The fixture's PTX.
pub const PTX: &[u8] = include_bytes!("../fixtures/s1b.ptx");
/// Launch width.
pub const W: u32 = 16;
/// Launch height.
pub const H: u32 = 16;
/// Triangle legs: no pixel centre lies within 0.03 of an edge.
pub const LEG: f32 = 1.03;
/// A translation that keeps pixel centres off triangle edges (pixel pitch multiple + a quarter).
pub const DX: f32 = 0.25;
/// `size_of` the fixture's launch parameters `{ out: u64, handle: u64, width: u32, height: u32 }`.
pub const PARAMS_SIZE: usize = 24;
/// Smallest distance of a pixel centre to an analytic edge that a mask accepts.
pub const MARGIN: f32 = 0.003;

/// CUDA device 0 and its default stream.
pub fn cuda() -> (Arc<Context>, Arc<Stream>) {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    (cuda, stream)
}

/// What the rig's pipeline traces.
pub struct Cfg {
    /// `PipelineOptions::single_gas`.
    pub single_gas: bool,
    /// `PipelineOptions::motion_blur`.
    pub motion: bool,
    /// `PipelineOptions::opacity_micromaps`.
    pub omm: bool,
    /// `PipelineOptions::clustered_geometry`.
    pub clusters: bool,
    /// Built-in intersector of the hit group (curves, spheres); `None` = triangles.
    pub builtin: Option<BuiltinIs>,
    /// Hit-group records in the SBT (all bound to the fixture's closest hit).
    pub records: usize,
    /// Link options (graph depth for transforms).
    pub link: LinkOptions,
    /// OptiX validation mode.
    pub validation: bool,
}

impl Default for Cfg {
    fn default() -> Self {
        Self { single_gas: false, motion: false, omm: false, clusters: false, builtin: None, records: 1, link: LinkOptions::default(), validation: false }
    }
}

/// A context, a pipeline over the fixture and an SBT.
pub struct Rig {
    /// Default stream of CUDA device 0.
    pub stream: Arc<Stream>,
    /// OptiX context.
    pub ctx: Arc<DeviceContext>,
    /// Build scratch on `stream`.
    pub scratch: Scratch,
    /// The options every module of the pipeline was compiled with.
    pub opts: PipelineOptions,
    /// Raygen, miss and hit groups.
    pub groups: [Arc<ProgramGroup>; 3],
    /// The linked pipeline.
    pub pipeline: Pipeline,
    /// Raygen, miss and `Cfg::records` hit records.
    pub sbt: ShaderBindingTable,
}

/// Pipeline options for `cfg` with `payload` payload values.
pub fn options(cfg: &Cfg, payload: u32) -> PipelineOptions {
    let mut opts = PipelineOptions::new("params", PARAMS_SIZE, payload).expect("options");
    opts.single_gas = cfg.single_gas;
    opts.motion_blur = cfg.motion;
    opts.opacity_micromaps = cfg.omm;
    opts.clustered_geometry = cfg.clusters;
    if let Some(is) = cfg.builtin {
        opts.primitive_types = primitive_flag(is.primitive);
    }
    opts
}

/// The `OptixPrimitiveTypeFlags` bit of a built-in primitive.
pub fn primitive_flag(p: optix::Primitive) -> u32 {
    use optix::Primitive::*;
    use optix::sys;
    (match p {
        RoundQuadraticBspline => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_QUADRATIC_BSPLINE,
        RoundCubicBspline => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CUBIC_BSPLINE,
        RoundLinear => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_LINEAR,
        RoundCatmullRom => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CATMULLROM,
        FlatQuadraticBspline => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_FLAT_QUADRATIC_BSPLINE,
        Sphere => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_SPHERE,
        RoundCubicBezier => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CUBIC_BEZIER,
        RoundQuadraticBsplineRocaps => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_QUADRATIC_BSPLINE_ROCAPS,
        RoundCubicBsplineRocaps => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CUBIC_BSPLINE_ROCAPS,
        RoundCatmullRomRocaps => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CATMULLROM_ROCAPS,
        RoundCubicBezierRocaps => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CUBIC_BEZIER_ROCAPS,
        Triangle => sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_TRIANGLE,
    }) as u32
}

/// Raygen, miss and hit groups over `module` (hit group: the fixture's closest hit plus the
/// built-in intersector `is`), each created with `payload` as its payload type.
pub fn groups(ctx: &Arc<DeviceContext>, module: &Arc<Module>, is: Option<&Arc<Module>>, payload: Option<PayloadType<'_>>) -> [Arc<ProgramGroup>; 3] {
    let raygen = ProgramGroup::new(ctx, Programs::Raygen(module, "__raygen__mask"), payload).expect("raygen");
    let miss = ProgramGroup::new(ctx, Programs::Miss(module, "__miss__none"), payload).expect("miss");
    let ch = Some((module, "__closesthit__hit"));
    let hit_group = match is {
        Some(is) => HitGroup::builtin(ch, is),
        None => HitGroup { closest_hit: ch, ..Default::default() },
    };
    let hit = ProgramGroup::new(ctx, Programs::HitGroup(hit_group), payload).expect("hit group");
    [raygen, miss, hit]
}

/// An SBT with one raygen and miss record and `records` hit records of `groups`.
pub fn sbt(stream: &Stream, groups: &[Arc<ProgramGroup>; 3], records: usize) -> ShaderBindingTable {
    let [raygen, miss, hit] = groups;
    let hits: Vec<SbtRecord<u32>> = (0..records).map(|_| SbtRecord::new(hit, 0u32).expect("hit record")).collect();
    ShaderBindingTable::new(stream, &SbtRecord::new(raygen, 0u32).expect("rg record"), &[SbtRecord::new(miss, 0u32).expect("miss record")], &hits).expect("sbt")
}

/// The rig for `cfg`, with an optional log sink on the context.
pub fn rig_with_log(cfg: Cfg, log: Option<Box<LogFn>>) -> Rig {
    let (cuda, stream) = cuda();
    let ctx = DeviceContext::new(&cuda, log, 4, cfg.validation).expect("OptiX device context");
    let opts = options(&cfg, 1);
    let (module, _log) = Module::new(&ctx, &ModuleOptions::default(), &opts, PTX).expect("module");
    let is = cfg.builtin.map(|b| Module::builtin_intersection(&ctx, &ModuleOptions::default(), &opts, b).expect("built-in intersection module"));
    let groups = groups(&ctx, &module, is.as_ref(), None);
    let (pipeline, _log) = Pipeline::new(&ctx, &opts, cfg.link, &groups).expect("pipeline");
    let sbt = sbt(&stream, &groups, cfg.records);
    let scratch = Scratch::new(&stream);
    Rig { stream, ctx, scratch, opts, groups, pipeline, sbt }
}

/// The rig for `cfg`.
pub fn rig(cfg: Cfg) -> Rig {
    rig_with_log(cfg, None)
}

/// The fixture's launch parameters for output `out` and traversable `handle`.
pub fn params(out: &DeviceBuffer<u32>, handle: u64, width: u32, height: u32) -> Vec<u8> {
    [out.cu_deviceptr().to_le_bytes().as_slice(), &handle.to_le_bytes(), &width.to_le_bytes(), &height.to_le_bytes()].concat()
}

/// Launches `pipeline` over `handle` (W x H) and returns the per-pixel hit flags.
pub fn launch(stream: &Arc<Stream>, pipeline: &Pipeline, sbt: &ShaderBindingTable, handle: u64) -> Vec<u32> {
    try_launch(stream, pipeline, sbt, handle, W).expect("launch")
}

/// Like [`launch`], with the `width` field of the launch parameters set to `width` (the launch
/// itself stays W x H; for bound-value checks) and the launch error returned.
pub fn try_launch(stream: &Arc<Stream>, pipeline: &Pipeline, sbt: &ShaderBindingTable, handle: u64, width: u32) -> optix::Result<Vec<u32>> {
    let out = DeviceBuffer::<u32>::from_host(stream, &[7; (W * H) as usize]).expect("out");
    let params = DeviceBuffer::from_host(stream, &params(&out, handle, width, H)).expect("params");
    // SAFETY: `params` matches the fixture's launch-parameter struct; `out` holds W*H u32 (a
    // smaller `width` only shrinks the rows the raygen indexes); the SBT records carry no data
    // the programs read; everything outlives the synchronization below.
    unsafe { pipeline.launch(stream, &params, sbt, [W, H, 1]) }?;
    stream.synchronize()?;
    Ok(out.to_host_vec(stream)?)
}

impl Rig {
    /// Launches the rig's pipeline over `handle`.
    pub fn trace(&self, handle: u64) -> Vec<u32> {
        launch(&self.stream, &self.pipeline, &self.sbt, handle)
    }
}

/// The hit mask of a shape given by a signed distance at pixel centres `(x, y)` in `[0, 1]^2`
/// (positive inside); panics when a pixel centre lies within [`MARGIN`] of the boundary.
pub fn grid(d: impl Fn(f32, f32) -> f32) -> Vec<u32> {
    (0..H)
        .flat_map(|j| (0..W).map(move |i| ((i as f32 + 0.5) / W as f32, (j as f32 + 0.5) / H as f32)))
        .map(|(x, y)| {
            let v = d(x, y);
            assert!(v.abs() > MARGIN, "pixel centre ({x}, {y}) lies {v} from the analytic boundary");
            u32::from(v > 0.0)
        })
        .collect()
}

/// Signed distance (in the sense of [`grid`]) of the right triangle with legs `leg` along +x and
/// +y and its right angle at `(dx, 0)`.
pub fn tri(dx: f32, leg: f32) -> impl Fn(f32, f32) -> f32 {
    move |x, y| {
        let u = x - dx;
        u.min(y).min(leg - u - y)
    }
}

/// The hit mask of the default triangle translated by `dx`.
pub fn mask(dx: f32) -> Vec<u32> {
    grid(tri(dx, LEG))
}

/// Pixel-wise OR of two masks.
pub fn union(a: &[u32], b: &[u32]) -> Vec<u32> {
    a.iter().zip(b).map(|(a, b)| a | b).collect()
}

/// The default triangle's vertices translated by `dx`.
pub fn tri_verts(dx: f32, leg: f32) -> [[f32; 3]; 3] {
    [[dx, 0.0, 0.0], [dx + leg, 0.0, 0.0], [dx, leg, 0.0]]
}

/// IEEE binary16 bits of `x` (round to nearest even), for normal-range values and zero.
pub fn f16(x: f32) -> u16 {
    let bits = x.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    if x == 0.0 {
        return sign;
    }
    let exp = ((bits >> 23) & 0xff) as i32 - 127 + 15;
    assert!((1..31).contains(&exp), "{x} is outside the normal half range");
    let mant = bits & 0x7f_ffff;
    let mut h = ((exp as u32) << 10) | (mant >> 13);
    let rest = mant & 0x1fff;
    if rest > 0x1000 || (rest == 0x1000 && h & 1 == 1) {
        h += 1;
    }
    sign | h as u16
}

/// The value of IEEE binary16 bits `h`.
pub fn f32_of(h: u16) -> f32 {
    let sign = if h & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exp = i32::from((h >> 10) & 0x1f);
    let mant = f32::from(h & 0x3ff);
    sign * match exp {
        0 => mant * 2f32.powi(-24),
        31 if mant == 0.0 => f32::INFINITY,
        31 => f32::NAN,
        e => (1.0 + mant / 1024.0) * 2f32.powi(e - 15),
    }
}

/// A flat image of colour `base` with deterministic per-channel noise of amplitude `amp`.
pub fn noisy(n: usize, base: f32, amp: f32, seed: u32) -> Vec<[f32; 4]> {
    let mut s = seed.max(1);
    let mut rnd = move || {
        s ^= s << 13;
        s ^= s >> 17;
        s ^= s << 5;
        (s as f32 / u32::MAX as f32) * 2.0 - 1.0
    };
    (0..n).map(|_| [base + amp * rnd(), base + amp * rnd(), base + amp * rnd(), 1.0]).collect()
}

/// Mean absolute deviation of the RGB channels from `base`.
pub fn deviation(px: &[[f32; 4]], base: f32) -> f32 {
    px.iter().flat_map(|p| p[..3].iter()).map(|c| (c - base).abs()).sum::<f32>() / (px.len() * 3) as f32
}
