//! Case `transform`: transform lists, from-handle getters and motion through
//! `optix_device::transform`.
//!
//! One IAS (built with random instance access) over a big triangle GAS, five instances 20 apart in
//! x, each a different graph:
//! 0. instance (rotate + scale) -> static transform (tilt + translate) -> GAS;
//! 1. instance (tilt about y) -> GAS;
//! 2. instance -> matrix-motion transform (identity -> rotate/scale/translate) -> GAS;
//! 3. instance -> SRT-motion transform (identity -> 90 degrees about z, scale 2, translate) -> GAS;
//! 4. instance -> motion GAS (two vertex keys).
//!
//! Every launch index is one probe ray (origin, direction, time) from a host table; static
//! instances are probed at one time, motion ones at times 0, 0.3, 0.5 and 1. The closest-hit
//! writes: transform-list size, instance id / index, object-to-world and world-to-object matrices,
//! point / vector / normal transforms both ways, hit distance and barycentrics, the vertices at
//! the ray time, and for the list entries their type, instance id / child / transforms, the
//! static or motion transform's child, `transform_from_handle` both ways,
//! `instance_traversable_from_ias` and the GAS motion range. The host recomputes everything
//! independently (matrix products, a general 3x3 inverse, `T * R * S` from the SRT key with an
//! nlerped quaternion, a ray-triangle intersection) and compares.

use super::common::{Ctx, Gpu, R, close, opts, same};
use bytemuck::{Pod, Zeroable};
use cuda_device::{ConstantMemory, constant, cuda_module, kernel};
use optix::{Accel, BuildOptions, GeometryFlags, HitGroup, Instance, Keys, LinkOptions, MotionOptions, ProgramGroup, SbtRecord, Scratch, ShaderBindingTable, Srt, Transform, TriangleMesh, Vertices, sys};
use optix_device::trace::{Ray, TraceOptions};
use optix_device::transform as xf;
use optix_device::{state, trace};

/// Words per probe record.
const REC: usize = 136;

/// Launch parameters (`params_xf`).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    /// `REC` words per probe.
    pub out: u64,
    /// `[f32; 8]` per probe: origin, direction, time, unused.
    pub probes: u64,
    /// The IAS.
    pub handle: u64,
}

// SAFETY: plain integers; all-zero is valid.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

/// The fixed point, vector and normal every closest-hit transforms both ways.
const P: [f32; 3] = [0.3, -0.2, 0.7];
const V: [f32; 3] = [1.0, 2.0, 3.0];
const N: [f32; 3] = [0.5, -1.0, 0.25];

#[cuda_module]
mod kernels {
    use super::*;
    use optix_device::sys as dsys;

    #[constant(export_name = "params_xf")]
    static PARAMS: ConstantMemory<Params> = ConstantMemory::UNINIT;

    /// This probe's record.
    ///
    /// # Safety
    ///
    /// Programs of this case's launch; `out` holds `REC` words per launch index.
    #[inline(always)]
    unsafe fn rec() -> *mut u32 {
        // SAFETY: forwarded contract.
        unsafe { (PARAMS.get().out as *mut u32).add(state::launch_index()[0] as usize * REC) }
    }

    /// Writes the bits of `v` at `r[at..]`.
    ///
    /// # Safety
    ///
    /// `r[at..at + K]` is writable.
    #[inline(always)]
    unsafe fn put<const K: usize>(r: *mut u32, at: usize, v: [f32; K]) {
        let mut i = 0;
        while i < K {
            // SAFETY: forwarded contract.
            unsafe { *r.add(at + i) = v[i].to_bits() };
            i += 1;
        }
    }

    /// Writes `h` as two words at `r[at..]` (low first).
    ///
    /// # Safety
    ///
    /// `r[at..at + 2]` is writable.
    #[inline(always)]
    unsafe fn put64(r: *mut u32, at: usize, h: u64) {
        // SAFETY: forwarded contract.
        unsafe {
            *r.add(at) = h as u32;
            *r.add(at + 1) = (h >> 32) as u32;
        }
    }

    #[kernel]
    pub fn __raygen__xf() {
        // SAFETY: raygen; `params_xf` is set by the host with one probe per launch index.
        unsafe {
            let i = state::launch_index()[0] as usize;
            let p = PARAMS.get();
            let q = (p.probes as *const [f32; 8]).add(i).read();
            *rec() = u32::MAX;
            let ray = Ray { origin: [q[0], q[1], q[2]], direction: [q[3], q[4], q[5]], tmin: 0.0, tmax: 1.0e16, time: q[6] };
            trace::trace_no_payload(p.handle, &ray, &TraceOptions::default());
        }
    }

    #[kernel]
    pub fn __miss__xf() {
        // SAFETY: miss program of this case.
        unsafe { *rec() = 0 }
    }

    #[kernel]
    pub fn __closesthit__xf() {
        // SAFETY: closest-hit on triangles under an instance; list entries are read by their type.
        unsafe {
            let r = rec();
            let n = xf::transform_list_size();
            let time = state::ray_time();
            *r = 1;
            *r.add(1) = n;
            *r.add(2) = state::instance_id();
            *r.add(3) = state::instance_index();
            put(r, 4, xf::object_to_world_matrix());
            put(r, 16, xf::world_to_object_matrix());
            put(r, 28, xf::transform_point_from_object_to_world(P));
            put(r, 31, xf::transform_vector_from_object_to_world(V));
            put(r, 34, xf::transform_normal_from_object_to_world(N));
            put(r, 37, xf::transform_point_from_world_to_object(P));
            put(r, 40, xf::transform_vector_from_world_to_object(V));
            put(r, 43, xf::transform_normal_from_world_to_object(N));
            put(r, 46, [state::ray_tmax()]);
            put(r, 47, state::triangle_barycentrics());
            let v = optix_device::geometry::triangle_vertices();
            put(r, 49, v[0]);
            put(r, 52, v[1]);
            put(r, 55, v[2]);
            let h0 = xf::transform_list_handle(0);
            *r.add(58) = xf::transform_type(h0) as u32;
            *r.add(59) = xf::instance_id(h0);
            put64(r, 60, xf::instance_child(h0));
            put(r, 62, xf::instance_transform(h0));
            put(r, 74, xf::instance_inverse_transform(h0));
            *r.add(86) = u32::MAX;
            if n > 1 {
                let h1 = xf::transform_list_handle(1);
                let ty = xf::transform_type(h1);
                *r.add(86) = ty as u32;
                let child = if ty == dsys::OptixTransformType_OPTIX_TRANSFORM_TYPE_STATIC_TRANSFORM {
                    let s = xf::static_transform(h1);
                    put(r, 120, s.transform);
                    s.child
                } else if ty == dsys::OptixTransformType_OPTIX_TRANSFORM_TYPE_MATRIX_MOTION_TRANSFORM {
                    (&raw const (*xf::matrix_motion_transform_ptr(h1)).child).read()
                } else {
                    (&raw const (*xf::srt_motion_transform_ptr(h1)).child).read()
                };
                put64(r, 87, child);
                put(r, 96, xf::transform_from_handle(h1, time, true));
                put(r, 108, xf::transform_from_handle(h1, time, false));
            }
            *r.add(89) = u32::from(xf::instance_traversable_from_ias(PARAMS.get().handle, state::instance_index()) == h0);
            let gas = state::gas_traversable_handle();
            let steps = xf::gas_motion_step_count(gas);
            *r.add(90) = steps;
            put(r, 91, [xf::gas_motion_time_begin(gas), xf::gas_motion_time_end(gas), time]);
            if steps > 1 {
                let d = optix_device::geometry::triangle_vertices_from_handle(gas, state::primitive_index(), state::sbt_gas_index(), time);
                put(r, 120, d[0]);
                put(r, 123, d[1]);
                put(r, 126, d[2]);
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Host math (independent of the device port).
// ---------------------------------------------------------------------------------------------

type M = [f32; 12];
const ID: M = Instance::IDENTITY;

fn mul(a: &M, b: &M) -> M {
    let mut r = [0.0; 12];
    for i in 0..3 {
        for j in 0..4 {
            r[i * 4 + j] = (0..3).map(|k| a[i * 4 + k] * b[k * 4 + j]).sum::<f32>() + if j == 3 { a[i * 4 + 3] } else { 0.0 };
        }
    }
    r
}

/// General affine inverse in f64 (adjugate of the 3x3 part).
fn inv(m: &M) -> M {
    let a = |i: usize, j: usize| f64::from(m[i * 4 + j]);
    let det = a(0, 0) * (a(1, 1) * a(2, 2) - a(1, 2) * a(2, 1)) - a(0, 1) * (a(1, 0) * a(2, 2) - a(1, 2) * a(2, 0)) + a(0, 2) * (a(1, 0) * a(2, 1) - a(1, 1) * a(2, 0));
    let c = |i: usize, j: usize| {
        let (r0, r1) = ([0, 1, 2].into_iter().filter(|x| *x != j).collect::<Vec<_>>(), [0, 1, 2].into_iter().filter(|x| *x != i).collect::<Vec<_>>());
        let s = if (i + j).is_multiple_of(2) { 1.0 } else { -1.0 };
        s * (a(r0[0], r1[0]) * a(r0[1], r1[1]) - a(r0[0], r1[1]) * a(r0[1], r1[0])) / det
    };
    let mut r = [0.0f32; 12];
    for i in 0..3 {
        for j in 0..3 {
            r[i * 4 + j] = c(i, j) as f32;
        }
        r[i * 4 + 3] = -(0..3).map(|k| c(i, k) * a(k, 3)).sum::<f64>() as f32;
    }
    r
}

fn point(m: &M, p: [f32; 3]) -> [f32; 3] {
    [0, 1, 2].map(|i| m[i * 4] * p[0] + m[i * 4 + 1] * p[1] + m[i * 4 + 2] * p[2] + m[i * 4 + 3])
}

fn vector(m: &M, v: [f32; 3]) -> [f32; 3] {
    [0, 1, 2].map(|i| m[i * 4] * v[0] + m[i * 4 + 1] * v[1] + m[i * 4 + 2] * v[2])
}

/// Normal under the point transform `m`: inverse-transpose of its linear part.
fn normal(m: &M, n: [f32; 3]) -> [f32; 3] {
    let i = inv(m);
    [0, 1, 2].map(|c| i[c] * n[0] + i[4 + c] * n[1] + i[8 + c] * n[2])
}

fn translate(t: [f32; 3]) -> M {
    [1.0, 0.0, 0.0, t[0], 0.0, 1.0, 0.0, t[1], 0.0, 0.0, 1.0, t[2]]
}

fn scale(s: f32) -> M {
    [s, 0.0, 0.0, 0.0, 0.0, s, 0.0, 0.0, 0.0, 0.0, s, 0.0]
}

fn rot(axis: usize, deg: f32) -> M {
    let (s, c) = deg.to_radians().sin_cos();
    let (a, b) = ((axis + 1) % 3, (axis + 2) % 3);
    let mut m = ID;
    m[a * 4 + a] = c;
    m[a * 4 + b] = -s;
    m[b * 4 + a] = s;
    m[b * 4 + b] = c;
    m
}

fn lerp<const K: usize>(a: &[f32; K], b: &[f32; K], t: f32) -> [f32; K] {
    std::array::from_fn(|i| a[i] * (1.0 - t) + b[i] * t)
}

/// `T * R * S` of an SRT key: S is upper-triangular scale/shear with the pivot as translation,
/// R the rotation of the (normalized) quaternion, T the translation.
fn srt_matrix(k: &Srt) -> M {
    let s = [k.sx, k.a, k.b, k.pvx, 0.0, k.sy, k.c, k.pvy, 0.0, 0.0, k.sz, k.pvz];
    let (x, y, z, w) = (k.qx, k.qy, k.qz, k.qw);
    let r = [
        1.0 - 2.0 * (y * y + z * z),
        2.0 * (x * y - z * w),
        2.0 * (x * z + y * w),
        0.0,
        2.0 * (x * y + z * w),
        1.0 - 2.0 * (x * x + z * z),
        2.0 * (y * z - x * w),
        0.0,
        2.0 * (x * z - y * w),
        2.0 * (y * z + x * w),
        1.0 - 2.0 * (x * x + y * y),
        0.0,
    ];
    mul(&translate([k.tx, k.ty, k.tz]), &mul(&r, &s))
}

/// The SRT key at local time `t`: componentwise lerp, quaternion renormalized (nlerp).
fn srt_at(a: &Srt, b: &Srt, t: f32) -> Srt {
    let l = |x: f32, y: f32| x * (1.0 - t) + y * t;
    let q = [l(a.qx, b.qx), l(a.qy, b.qy), l(a.qz, b.qz), l(a.qw, b.qw)];
    let n = q.iter().map(|v| v * v).sum::<f32>().sqrt();
    Srt {
        sx: l(a.sx, b.sx),
        a: l(a.a, b.a),
        b: l(a.b, b.b),
        pvx: l(a.pvx, b.pvx),
        sy: l(a.sy, b.sy),
        c: l(a.c, b.c),
        pvy: l(a.pvy, b.pvy),
        sz: l(a.sz, b.sz),
        pvz: l(a.pvz, b.pvz),
        qx: q[0] / n,
        qy: q[1] / n,
        qz: q[2] / n,
        qw: q[3] / n,
        tx: l(a.tx, b.tx),
        ty: l(a.ty, b.ty),
        tz: l(a.tz, b.tz),
    }
}

/// Ray-triangle intersection (Moller-Trumbore): `(t, u, v)`.
fn intersect(o: [f32; 3], d: [f32; 3], v: [[f32; 3]; 3]) -> Option<(f32, f32, f32)> {
    let sub = |a: [f32; 3], b: [f32; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let cross = |a: [f32; 3], b: [f32; 3]| [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let (e1, e2) = (sub(v[1], v[0]), sub(v[2], v[0]));
    let p = cross(d, e2);
    let det = dot(e1, p);
    let s = sub(o, v[0]);
    let u = dot(s, p) / det;
    let q = cross(s, e1);
    let w = dot(d, q) / det;
    let t = dot(e2, q) / det;
    (u >= 0.0 && w >= 0.0 && u + w <= 1.0 && t > 0.0).then_some((t, u, w))
}

// ---------------------------------------------------------------------------------------------
// Scene.
// ---------------------------------------------------------------------------------------------

/// The object-space triangle (big enough that every probe hits its own instance).
const TRI: [[f32; 3]; 3] = [[-3.0, -3.0, 0.0], [3.0, -3.0, 0.0], [0.0, 4.0, 0.0]];
/// Second key of the motion GAS.
const TRI1: [[f32; 3]; 3] = [[-2.5, -3.5, 0.0], [3.5, -2.0, 0.0], [0.5, 4.5, 0.0]];
const SPACING: f32 = 20.0;
const IDS: [u32; 5] = [7, 11, 13, 17, 19];

/// The object-to-world matrix of a list entry at a ray time.
type EntryFn = Box<dyn Fn(f32) -> M>;

/// What the host knows about one instance.
struct Inst {
    /// Instance object-to-world.
    m: M,
    /// Second list entry: its object-to-world at a time, and its kind.
    entry: Option<(EntryFn, sys::OptixTransformType)>,
    /// Vertices at a time.
    verts: Box<dyn Fn(f32) -> [[f32; 3]; 3]>,
}

fn srt_keys() -> [Srt; 2] {
    let h = std::f32::consts::FRAC_1_SQRT_2;
    [Srt::IDENTITY, Srt { sx: 2.0, sy: 2.0, sz: 2.0, qz: h, qw: h, tx: 1.0, ty: 0.5, ..Srt::IDENTITY }]
}

fn mm_keys() -> [M; 2] {
    [ID, mul(&translate([0.3, 0.2, 0.0]), &mul(&rot(2, 40.0), &scale(1.2)))]
}

fn static_m() -> M {
    mul(&translate([0.2, 0.1, 0.3]), &rot(0, 20.0))
}

fn scene() -> Vec<Inst> {
    let base = |k: usize| translate([SPACING * k as f32, 0.0, 0.0]);
    let fixed = |_: f32| TRI;
    vec![
        Inst { m: mul(&base(0), &mul(&rot(2, 30.0), &scale(1.5))), entry: Some((Box::new(|_| static_m()), sys::OptixTransformType_OPTIX_TRANSFORM_TYPE_STATIC_TRANSFORM)), verts: Box::new(fixed) },
        Inst { m: mul(&base(1), &rot(1, 15.0)), entry: None, verts: Box::new(fixed) },
        Inst {
            m: base(2),
            entry: Some((Box::new(|t| lerp(&mm_keys()[0], &mm_keys()[1], t)), sys::OptixTransformType_OPTIX_TRANSFORM_TYPE_MATRIX_MOTION_TRANSFORM)),
            verts: Box::new(fixed),
        },
        Inst { m: base(3), entry: Some((Box::new(|t| srt_matrix(&srt_at(&srt_keys()[0], &srt_keys()[1], t))), sys::OptixTransformType_OPTIX_TRANSFORM_TYPE_SRT_MOTION_TRANSFORM)), verts: Box::new(fixed) },
        Inst {
            m: base(4),
            entry: None,
            verts: Box::new(|t| std::array::from_fn(|i| lerp(&TRI[i], &TRI1[i], t))),
        },
    ]
}

/// The probes: `(instance, time)` with an origin near the instance's origin.
fn probes() -> Vec<(usize, [f32; 8])> {
    let mut v = Vec::new();
    for k in 0..5 {
        let times: &[f32] = if k < 2 { &[0.7] } else { &[0.0, 0.3, 0.5, 1.0] };
        for &t in times {
            v.push((k, [SPACING * k as f32 + 0.13, 0.07 + 0.1 * t, -5.0, 0.02, -0.01, 1.0, t, 0.0]));
        }
    }
    v
}

/// The programs this case links.
const ENTRIES: &[&str] = &["__raygen__xf", "__miss__xf", "__closesthit__xf"];

/// Runs the case.
pub fn run(g: &Gpu) -> R {
    let mut o = opts("params_xf", size_of::<Params>())?;
    o.motion_blur = true;
    let module = g.module(&Default::default(), &o, ENTRIES)?;
    let raygen = ProgramGroup::raygen(&module, "__raygen__xf").ctx("raygen")?;
    let miss = ProgramGroup::miss(&module, "__miss__xf").ctx("miss")?;
    let hit = ProgramGroup::hit_group(&g.ctx, HitGroup { closest_hit: Some((&module, "__closesthit__xf")), ..Default::default() }).ctx("hit group")?;
    let pipeline = g.pipeline(&o, LinkOptions { max_graph_depth: 3, ..Default::default() }, &[raygen.clone(), miss.clone(), hit.clone()])?;
    let sbt = ShaderBindingTable::new(&g.stream, &SbtRecord::new(&raygen, 0u32).ctx("rg")?, &[SbtRecord::new(&miss, 0u32).ctx("ms")?], &[SbtRecord::new(&hit, 0u32).ctx("hg")?]).ctx("SBT")?;

    let mut scratch = Scratch::new(&g.stream);
    let flags = [GeometryFlags::DISABLE_ANYHIT];
    let v0 = g.up(&TRI)?;
    let gas = Accel::triangles(&g.ctx, &g.stream, &mut scratch, &[TriangleMesh { flags: &flags, ..TriangleMesh::new(&v0) }], BuildOptions::default()).ctx("GAS")?;
    let v1 = g.up(&TRI1)?;
    let keys = [&v0, &v1];
    let mgas_opts = BuildOptions { motion_keys: 2, motion: MotionOptions::default(), random_vertex_access: true, ..Default::default() };
    let mgas = Accel::triangles(&g.ctx, &g.stream, &mut scratch, &[TriangleMesh { flags: &flags, ..TriangleMesh::new(Vertices::Float3(Keys::Motion(&keys))) }], mgas_opts).ctx("motion GAS")?;
    let st = Transform::fixed(&g.ctx, &g.stream, &gas, static_m()).ctx("static transform")?;
    let mm = Transform::matrix_motion(&g.ctx, &g.stream, &gas, MotionOptions::default(), &mm_keys()).ctx("matrix motion transform")?;
    let sm = Transform::srt_motion(&g.ctx, &g.stream, &gas, MotionOptions::default(), &srt_keys()).ctx("SRT motion transform")?;
    let sc = scene();
    let children: [optix::Child; 5] = [st.clone().into(), gas.clone().into(), mm.clone().into(), sm.clone().into(), mgas.clone().into()];
    let insts: Vec<Instance> = children.iter().zip(&sc).zip(IDS).map(|((c, s), id)| Instance { transform: s.m, id, ..Instance::new(c.clone()) }).collect();
    let ias = Accel::instances(&g.ctx, &g.stream, &mut scratch, &insts, BuildOptions { random_instance_access: true, ..Default::default() }).ctx("IAS")?;
    // The child each list entry must report: entry 0's instance child, entry 1's transform child.
    let child0 = [st.handle(), gas.handle(), mm.handle(), sm.handle(), mgas.handle()];

    let pr = probes();
    let table = g.up(&pr.iter().map(|p| p.1).collect::<Vec<_>>())?;
    let out = g.zeros::<u32>(pr.len() * REC)?;
    let params = Params { out: out.cu_deviceptr(), probes: table.cu_deviceptr(), handle: ias.handle() };
    // SAFETY: `params` matches the device struct; `out` holds `REC` words per probe; no SBT data
    // is read; everything outlives the launch (it synchronizes).
    unsafe { g.launch(&pipeline, &sbt, &params, [pr.len() as u32, 1, 1]) }?;
    let got = g.down(&out)?;

    let f = |w: &[u32]| w.iter().map(|x| f32::from_bits(*x)).collect::<Vec<f32>>();
    let h64 = |w: &[u32]| u64::from(w[0]) | u64::from(w[1]) << 32;
    for (i, (k, q)) in pr.iter().enumerate() {
        let r = &got[i * REC..(i + 1) * REC];
        let s = &sc[*k];
        let t = q[6];
        let label = |w: &str| format!("probe {i} (instance {k}, t {t}) {w}");
        let entry_m = s.entry.as_ref().map(|(m, _)| m(t));
        let o2w = entry_m.map_or(s.m, |e| mul(&s.m, &e));
        let w2o = inv(&o2w);
        let ints = [r[0], r[1], r[2], r[3], r[58], r[59], r[86], r[89]];
        let list = 1 + u32::from(s.entry.is_some());
        let ty1 = s.entry.as_ref().map_or(u32::MAX, |e| e.1 as u32);
        let want = [1, list, IDS[*k], *k as u32, sys::OptixTransformType_OPTIX_TRANSFORM_TYPE_INSTANCE as u32, IDS[*k], ty1, 1];
        same(&label("(hit, list size, id, index, type0, id0, type1, ias lookup)"), &ints, &want, |j| format!("word {j}"))?;
        same(&label("instance child"), &[h64(&r[60..62])], &[child0[*k]], |_| String::new())?;
        if s.entry.is_some() {
            same(&label("entry-1 child"), &[h64(&r[87..89])], &[gas.handle()], |_| String::new())?;
        }
        let tol = |w: &[f32]| w.iter().fold(0.0f32, |a, x| a.max(x.abs())).max(1.0) * 2e-5;
        let chk = |what: &str, at: usize, want: &[f32]| close(&label(what), &f(&r[at..at + want.len()]), want, tol(want) * 4.0, |j| format!("[{j}]"));
        chk("object_to_world", 4, &o2w)?;
        chk("world_to_object", 16, &w2o)?;
        chk("point o2w", 28, &point(&o2w, P))?;
        chk("vector o2w", 31, &vector(&o2w, V))?;
        chk("normal o2w", 34, &normal(&o2w, N))?;
        chk("point w2o", 37, &point(&w2o, P))?;
        chk("vector w2o", 40, &vector(&w2o, V))?;
        chk("normal w2o", 43, &normal(&w2o, N))?;
        chk("instance transform", 62, &s.m)?;
        chk("instance inverse", 74, &inv(&s.m))?;
        let verts = (s.verts)(t);
        let world = verts.map(|v| point(&o2w, v));
        let (th, u, v) = intersect([q[0], q[1], q[2]], [q[3], q[4], q[5]], world).ok_or_else(|| label("misses on the CPU"))?;
        close(&label("t, barycentrics"), &f(&r[46..49]), &[th, u, v], 2e-4, |j| format!("[{j}]"))?;
        if *k == 4 {
            chk("motion GAS vertices from handle at the ray time", 120, verts.as_flattened())?;
        }
        if *k == 4 {
            // Observed with OptiX 9.1 / driver 621.24: the current-hit query on a motion GAS
            // returns key 0 at every ray time, although optix_device.h documents "at the current
            // ray time" (the from-handle query above interpolates). Pinned so a driver change shows.
            chk("current-hit vertices on a motion GAS (driver returns key 0)", 49, TRI.as_flattened())?;
        } else {
            chk("vertices at the ray time", 49, verts.as_flattened())?;
        }
        if let Some(e) = entry_m {
            chk("transform_from_handle o2w", 96, &e)?;
            chk("transform_from_handle w2o", 108, &inv(&e))?;
            if *k == 0 {
                chk("static transform", 120, &e)?;
            }
        }
        if *k == 4 {
            same(&label("motion GAS steps"), &[r[90]], &[2], |_| String::new())?;
            close(&label("motion GAS range, ray time"), &f(&r[91..94]), &[0.0, 1.0, t], 0.0, |j| format!("[{j}]"))?;
        }
    }
    println!("transform: {} probes over static, instance, matrix-motion, SRT-motion and motion-GAS graphs", pr.len());
    Ok(())
}
