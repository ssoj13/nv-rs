//! Case `intersect`: a custom-primitive intersection program and any-hit control.
//!
//! Four spheres (custom primitives, radius 0.3, centres on the z axis at z = 1..4) are
//! intersected by `__intersection__isect`, which reports the entry point with
//! `report_intersection`, user hit kind `10 + primitive` and typed attributes [`Attr`] (normal,
//! primitive, a magic word). Rays along +z at eight x offsets (|x| < 0.3 pierces all four)
//! run in four any-hit modes (launch y): 0 accept, 1 `ignore_intersection` on even primitives,
//! 2 ignore all, 3 `terminate_ray` on the first. The any-hit counts its calls and the
//! primitives it saw in the payload; the closest-hit reads `attributes::<Attr>()`, the hit
//! kind, the primitive type and the distance. The host checks the resulting hit per mode:
//! primitive 0, primitive 1, a miss with all four seen, and exactly one call whose primitive is
//! the one hit.

use super::common::{Ctx, Gpu, R, close, opts, same};
use bytemuck::{Pod, Zeroable};
use cuda_device::{ConstantMemory, constant, cuda_module, kernel};
use optix::{Aabb, Accel, BuildOptions, CustomPrimitives, GeometryFlags, HitGroup, LinkOptions, ProgramGroup, SbtRecord, Scratch, ShaderBindingTable, sys};
use optix_device::trace::{Ray, TraceOptions};
use optix_device::{geometry, payload, state, trace};

const RAYS: u32 = 8;
const MODES: u32 = 4;
/// Words per record.
const REC: usize = 16;
const RADIUS: f32 = 0.3;
const SPHERES: u32 = 4;
const MAGIC: u32 = 0xC0FF_EE00;

/// Launch parameters (`params_is`).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    /// `REC` words per launch index.
    pub out: u64,
    /// `[f32; 4]` per sphere: centre, radius.
    pub spheres: u64,
    /// The GAS.
    pub handle: u64,
}

// SAFETY: plain integers; all-zero is valid.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

/// Attributes the intersection program reports (5 of the 8 attribute registers).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Attr {
    /// Unit normal at the hit.
    pub n: [f32; 3],
    /// Primitive index.
    pub prim: u32,
    /// [`MAGIC`] xor the primitive.
    pub magic: u32,
}

/// The payload: any-hit mode, any-hit calls, mask of primitives the any-hit saw.
type Pay = [u32; 3];

#[cuda_module]
mod kernels {
    use super::*;

    #[constant(export_name = "params_is")]
    static PARAMS: ConstantMemory<Params> = ConstantMemory::UNINIT;

    /// This launch index's record.
    ///
    /// # Safety
    ///
    /// Programs of this case's launch.
    #[inline(always)]
    unsafe fn rec() -> *mut u32 {
        // SAFETY: forwarded contract; `out` holds `REC` words per launch index.
        unsafe {
            let [x, y, _] = state::launch_index();
            (PARAMS.get().out as *mut u32).add(((y * RAYS + x) as usize) * REC)
        }
    }

    #[kernel]
    pub fn __raygen__isect() {
        // SAFETY: raygen; `params_is` is set by the host.
        unsafe {
            let [x, y, _] = state::launch_index();
            let p = PARAMS.get();
            let ox = -0.45 + x as f32 * (0.9 / (RAYS - 1) as f32);
            let mut pay: Pay = [y, 0, 0];
            *rec() = 0;
            trace::trace(p.handle, &Ray::new([ox, 0.0, -1.0], [0.0, 0.0, 1.0], 0.0, 1.0e16), &TraceOptions::default(), &mut pay);
            *rec().add(1) = pay[1];
            *rec().add(2) = pay[2];
        }
    }

    #[kernel]
    pub fn __intersection__isect() {
        // SAFETY: intersection program over the spheres of `params_is`.
        unsafe {
            let prim = state::primitive_index();
            let s = (PARAMS.get().spheres as *const [f32; 4]).add(prim as usize).read();
            let o = state::object_ray_origin();
            let d = state::object_ray_direction();
            let oc = [o[0] - s[0], o[1] - s[1], o[2] - s[2]];
            let a = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
            let b = oc[0] * d[0] + oc[1] * d[1] + oc[2] * d[2];
            let c = oc[0] * oc[0] + oc[1] * oc[1] + oc[2] * oc[2] - s[3] * s[3];
            let disc = b * b - a * c;
            if disc > 0.0 {
                let t = (-b - cuda_device::float::sqrt_rn_f32(disc)) / a;
                if t > state::ray_tmin() && t < state::ray_tmax() {
                    let inv = 1.0 / s[3];
                    let n = [(oc[0] + t * d[0]) * inv, (oc[1] + t * d[1]) * inv, (oc[2] + t * d[2]) * inv];
                    state::report_intersection(t, 10 + prim, &Attr { n, prim, magic: MAGIC ^ prim });
                }
            }
        }
    }

    #[kernel]
    pub fn __anyhit__isect() {
        // SAFETY: any-hit on the spheres; the payload is a `Pay`.
        unsafe {
            let prim = state::primitive_index();
            let mode = payload::word::<0>();
            payload::set_word::<1>(payload::word::<1>() + 1);
            payload::set_word::<2>(payload::word::<2>() | 1 << prim);
            if (mode == 1 && prim % 2 == 0) || mode == 2 {
                state::ignore_intersection();
            } else if mode == 3 {
                state::terminate_ray();
            }
        }
    }

    #[kernel]
    pub fn __closesthit__isect() {
        // SAFETY: closest-hit on the spheres; the intersection program reported an `Attr`.
        unsafe {
            let r = rec();
            let a = state::attributes::<Attr>();
            let kind = state::hit_kind();
            *r = 1;
            *r.add(3) = state::primitive_index();
            *r.add(4) = kind;
            *r.add(5) = geometry::hit_primitive_type();
            *r.add(6) = state::ray_tmax().to_bits();
            *r.add(7) = a.n[0].to_bits();
            *r.add(8) = a.n[1].to_bits();
            *r.add(9) = a.n[2].to_bits();
            *r.add(10) = a.prim;
            *r.add(11) = a.magic;
        }
    }

    #[kernel]
    pub fn __miss__isect() {}
}

/// Runs the case.
pub fn run(g: &Gpu) -> R {
    let mut o = opts("params_is", size_of::<Params>())?;
    o.single_gas = true;
    o.primitive_types = (sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_CUSTOM | sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_TRIANGLE) as u32;
    let module = g.module(&Default::default(), &o)?;
    let raygen = ProgramGroup::raygen(&module, "__raygen__isect").ctx("raygen")?;
    let miss = ProgramGroup::miss(&module, "__miss__isect").ctx("miss")?;
    let hg = HitGroup { closest_hit: Some((&module, "__closesthit__isect")), any_hit: Some((&module, "__anyhit__isect")), intersection: Some((&module, "__intersection__isect")) };
    let hit = ProgramGroup::hit_group(&g.ctx, hg).ctx("hit group")?;
    let pipeline = g.pipeline(&o, LinkOptions::default(), &[raygen.clone(), miss.clone(), hit.clone()])?;
    let sbt = ShaderBindingTable::new(&g.stream, &SbtRecord::new(&raygen, 0u32).ctx("rg")?, &[SbtRecord::new(&miss, 0u32).ctx("ms")?], &[SbtRecord::new(&hit, 0u32).ctx("hg")?]).ctx("SBT")?;

    let spheres: Vec<[f32; 4]> = (0..SPHERES).map(|k| [0.0, 0.0, 1.0 + k as f32, RADIUS]).collect();
    let boxes: Vec<Aabb> = spheres.iter().map(|s| Aabb { min: [s[0] - s[3], s[1] - s[3], s[2] - s[3]], max: [s[0] + s[3], s[1] + s[3], s[2] + s[3]] }).collect();
    let sph = g.up(&spheres)?;
    let aabbs = g.up(&boxes)?;
    let mut scratch = Scratch::new(&g.stream);
    let prims = CustomPrimitives { flags: &[GeometryFlags::REQUIRE_SINGLE_ANYHIT_CALL], ..CustomPrimitives::new(&aabbs) };
    let gas = Accel::custom(&g.ctx, &g.stream, &mut scratch, &[prims], BuildOptions::default()).ctx("custom GAS")?;

    let out = g.zeros::<u32>((RAYS * MODES) as usize * REC)?;
    let params = Params { out: out.cu_deviceptr(), spheres: sph.cu_deviceptr(), handle: gas.handle() };
    // SAFETY: `params` matches the device struct; `out` holds `REC` words per launch index,
    // `spheres` one entry per primitive; no SBT data is read; the launch synchronizes.
    unsafe { g.launch(&pipeline, &sbt, &params, [RAYS, MODES, 1]) }?;
    let got = g.down(&out)?;

    let custom = sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_CUSTOM as u32;
    let mut pierced = 0;
    for mode in 0..MODES {
        for x in 0..RAYS {
            let r = &got[((mode * RAYS + x) as usize) * REC..][..REC];
            let ox = -0.45 + x as f32 * (0.9 / (RAYS - 1) as f32);
            let label = |w: &str| format!("ray x {ox} mode {mode}: {w}");
            let (hit, calls, mask) = (r[0], r[1], r[2]);
            if ox.abs() >= RADIUS {
                same(&label("miss (hit, calls, mask)"), &[hit, calls, mask], &[0, 0, 0], |j| format!("[{j}]"))?;
                continue;
            }
            pierced += 1;
            let want_prim = match mode {
                0 => Some(0),
                1 => Some(1),
                2 => None,
                _ => Some(mask.trailing_zeros()),
            };
            match mode {
                0 => same(&label("accept: prim 0 seen"), &[mask & 1], &[1], |_| String::new())?,
                1 => same(&label("ignore even: prims 0 and 1 seen"), &[mask & 3], &[3], |_| String::new())?,
                2 => same(&label("ignore all: (calls, mask)"), &[calls, mask], &[SPHERES, (1 << SPHERES) - 1], |j| format!("[{j}]"))?,
                _ => same(&label("terminate: (calls, one prim seen)"), &[calls, mask.count_ones()], &[1, 1], |j| format!("[{j}]"))?,
            }
            let Some(prim) = want_prim else {
                same(&label("ignore all misses"), &[hit], &[0], |_| String::new())?;
                continue;
            };
            let h = (RADIUS * RADIUS - ox * ox).sqrt();
            let t = 1.0 + prim as f32 - h + 1.0;
            same(&label("(hit, prim, kind, type, attr prim, attr magic)"), &[hit, r[3], r[4], r[5], r[10], r[11]], &[1, prim, 10 + prim, custom, prim, MAGIC ^ prim], |j| format!("[{j}]"))?;
            let f = |w: u32| f32::from_bits(w);
            close(&label("(t, normal)"), &[f(r[6]), f(r[7]), f(r[8]), f(r[9])], &[t, ox / RADIUS, 0.0, -h / RADIUS], 1e-4, |j| format!("[{j}]"))?;
        }
    }
    if pierced < 16 {
        return Err(format!("only {pierced} piercing rays checked"));
    }
    println!("intersect: {pierced} piercing rays over 4 any-hit modes, typed attributes, user hit kinds");
    Ok(())
}
