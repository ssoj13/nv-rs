//! Case `geometry`: vertex data of the hit primitive through `optix_device::geometry`.
//!
//! An IAS holds three GASes, each built with `random_vertex_access`: an indexed quad of two
//! triangles (one wound each way) under an instance translated by +0.1 in x, a sphere, and a
//! round cubic B-spline of three segments (spheres and curves through OptiX's built-in
//! intersection modules). A `[32, 16, 2]` launch shoots one orthographic ray per pixel, along +z
//! from z = -1 on layer 0 and along -z from z = +1 on layer 1, so every primitive is hit from
//! both sides. Each closest-hit writes a record: primitive kind, `hit_primitive_type`, the
//! front/back-face helper bits, primitive and SBT-GAS index, the current-hit vertex data, the
//! `_from_handle` vertex data and whether the deprecated `_from` overload agrees with it.
//! The host checks every pixel against the analytic scene (object-space data, exact), skipping
//! pixels within a margin of a boundary.

use super::common::{Ctx, Gpu, R, close, opts};
use bytemuck::{Pod, Zeroable};
use cuda_device::{ConstantMemory, constant, cuda_module, kernel};
use optix::{Accel, BuildOptions, BuiltinIs, CurveType, Curves, GeometryFlags, HitGroup, Indices, Instance, Keys, LinkOptions, Module, Primitive, ProgramGroup, Radii, SbtRecord, Scratch, ShaderBindingTable, Spheres, TriangleMesh, sys};
use optix_device::trace::{Ray, TraceOptions};
use optix_device::{geometry, state, trace};

const W: u32 = 32;
const H: u32 = 16;
/// Floats per pixel record.
const REC: usize = 40;

/// Launch parameters (`params_geo`).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    /// `REC` floats per launch index.
    pub out: u64,
    /// The IAS.
    pub handle: u64,
    /// Launch width.
    pub width: u32,
    /// Launch height.
    pub height: u32,
}

// SAFETY: plain integers; all-zero is valid.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

#[cuda_module]
mod kernels {
    use super::*;

    #[constant(export_name = "params_geo")]
    static PARAMS: ConstantMemory<Params> = ConstantMemory::UNINIT;

    /// This launch index's record.
    ///
    /// # Safety
    ///
    /// Any program of this case's launch; `out` holds `REC` floats per launch index.
    #[inline(always)]
    unsafe fn rec() -> *mut f32 {
        // SAFETY: forwarded contract.
        unsafe {
            let [x, y, z] = state::launch_index();
            let p = PARAMS.get();
            (p.out as *mut f32).add((((z * p.height + y) * p.width + x) as usize) * REC)
        }
    }

    /// Writes `v` at `r[at..]`.
    ///
    /// # Safety
    ///
    /// `r[at..at + N]` is writable.
    #[inline(always)]
    unsafe fn put<const N: usize>(r: *mut f32, at: usize, v: [f32; N]) {
        let mut i = 0;
        while i < N {
            // SAFETY: forwarded contract.
            unsafe { *r.add(at + i) = v[i] };
            i += 1;
        }
    }

    /// Kind, primitive type, face bits, primitive index and SBT-GAS index of the current hit.
    ///
    /// # Safety
    ///
    /// Closest-hit programs of this case.
    #[inline(always)]
    unsafe fn header(r: *mut f32, kind: f32) {
        // SAFETY: forwarded contract.
        unsafe {
            let hk = state::hit_kind();
            let bits = u32::from(geometry::is_front_face_hit())
                | u32::from(geometry::is_back_face_hit()) << 1
                | u32::from(geometry::is_triangle_front_face_hit()) << 2
                | u32::from(geometry::is_triangle_back_face_hit()) << 3
                | u32::from(geometry::is_triangle_hit()) << 4
                | u32::from(geometry::is_front_face(hk)) << 5
                | u32::from(state::is_back_face(hk)) << 6;
            put(r, 0, [kind, geometry::hit_primitive_type() as f32, bits as f32, state::primitive_index() as f32]);
            *r.add(38) = state::sbt_gas_index() as f32;
        }
    }

    /// Writes the rows of `v` back to back at `r[at..]`.
    ///
    /// # Safety
    ///
    /// `r[at..at + N * M]` is writable.
    #[inline(always)]
    unsafe fn put_rows<const N: usize, const M: usize>(r: *mut f32, at: usize, v: [[f32; N]; M]) {
        let mut j = 0;
        while j < M {
            // SAFETY: forwarded contract.
            unsafe { put(r, at + j * N, v[j]) };
            j += 1;
        }
    }

    /// 1.0 when `a == b` exactly.
    #[inline(always)]
    fn eq<const N: usize, const M: usize>(a: [[f32; N]; M], b: [[f32; N]; M]) -> f32 {
        let mut same = true;
        let mut j = 0;
        while j < M {
            let mut i = 0;
            while i < N {
                same &= a[j][i] == b[j][i];
                i += 1;
            }
            j += 1;
        }
        if same { 1.0 } else { 0.0 }
    }

    #[kernel]
    pub fn __raygen__geo() {
        // SAFETY: raygen; `params_geo` is set by the host, `out` sized to the launch.
        unsafe {
            let [x, y, z] = state::launch_index();
            let p = PARAMS.get();
            let s = if z == 0 { 1.0 } else { -1.0 };
            let origin = [(x as f32 + 0.5) / p.width as f32, (y as f32 + 0.5) / p.height as f32, -s];
            *rec() = -1.0;
            trace::trace_no_payload(p.handle, &Ray::new(origin, [0.0, 0.0, s], 0.0, 1.0e16), &TraceOptions::default());
        }
    }

    #[kernel]
    pub fn __miss__geo() {
        // SAFETY: miss program of this case.
        unsafe { *rec() = 0.0 }
    }

    #[kernel]
    pub fn __closesthit__geo_tri() {
        // SAFETY: closest-hit on triangles of a GAS built with random vertex access.
        unsafe {
            let r = rec();
            header(r, 1.0);
            let (gas, prim, sbt, t) = (state::gas_traversable_handle(), state::primitive_index(), state::sbt_gas_index(), state::ray_time());
            let h = geometry::triangle_vertices_from_handle(gas, prim, sbt, t);
            let d = geometry::triangle_vertices_from(gas, prim, sbt, t);
            put_rows(r, 4, geometry::triangle_vertices());
            put_rows(r, 20, h);
            *r.add(36) = eq(h, d);
        }
    }

    #[kernel]
    pub fn __closesthit__geo_sphere() {
        // SAFETY: closest-hit on spheres of a GAS built with random vertex access.
        unsafe {
            let r = rec();
            header(r, 2.0);
            let (gas, prim, sbt, t) = (state::gas_traversable_handle(), state::primitive_index(), state::sbt_gas_index(), state::ray_time());
            let h = geometry::sphere_data_from_handle(gas, prim, sbt, t);
            put(r, 4, geometry::sphere_data());
            put(r, 20, h);
            *r.add(36) = eq([h], [geometry::sphere_data_from(gas, prim, sbt, t)]);
        }
    }

    #[kernel]
    pub fn __closesthit__geo_curve() {
        // SAFETY: closest-hit on round cubic B-splines of a GAS built with random vertex access.
        unsafe {
            let r = rec();
            header(r, 3.0);
            let (gas, prim, sbt, t) = (state::gas_traversable_handle(), state::primitive_index(), state::sbt_gas_index(), state::ray_time());
            let h = geometry::cubic_bspline_vertices_from_handle(gas, prim, sbt, t);
            let d = geometry::cubic_bspline_vertices_from(gas, prim, sbt, t);
            put_rows(r, 4, geometry::cubic_bspline_vertices());
            put_rows(r, 20, h);
            *r.add(36) = eq(h, d);
            *r.add(37) = state::curve_parameter();
        }
    }
}

/// Instance translation of the triangle GAS.
const TRI_DX: f32 = 0.1;
/// Object-space quad: world x in [0.0625, 0.3125], y in [0.125, 0.875].
const QUAD: [[f32; 3]; 4] = [[-0.0375, 0.125, 0.0], [0.2125, 0.125, 0.0], [0.2125, 0.875, 0.0], [-0.0375, 0.875, 0.0]];
/// Triangle 0 is counter-clockwise seen from +z (normal +z), triangle 1 clockwise.
const TRIS: [[u32; 3]; 2] = [[0, 1, 2], [0, 3, 2]];
const SPHERE: [f32; 4] = [0.5, 0.5, 0.0, 0.15];
const CURVE_X: f32 = 0.8;
const CURVE_R: f32 = 0.04;
const CURVE_Y: [f32; 6] = [0.1, 0.25, 0.4, 0.55, 0.7, 0.85];
/// Distance from a boundary below which a pixel is not checked.
const MARGIN: f32 = 0.004;

/// The expected record of one launch index, or `None` when the pixel is too close to a
/// boundary to call. Only the header and data slots are compared ([`checked`]).
fn expected(x: u32, y: u32, z: u32) -> Option<[f32; REC]> {
    let (px, py) = ((x as f32 + 0.5) / W as f32, (y as f32 + 0.5) / H as f32);
    let mut r = [0.0f32; REC];
    let hit_type = |t: u32| t as f32;
    // Triangles, in object space.
    let (ox, oy) = (px - TRI_DX, py);
    let (x0, x1, y0, y1) = (QUAD[0][0], QUAD[1][0], QUAD[0][1], QUAD[2][1]);
    let inside = (ox - x0).min(x1 - ox).min(oy - y0).min(y1 - oy);
    if inside.abs() < MARGIN {
        return None;
    }
    if inside > 0.0 {
        let (d, v) = ([QUAD[2][0] - x0, QUAD[2][1] - y0], [ox - x0, oy - y0]);
        let s = (d[0] * v[1] - d[1] * v[0]) / (d[0] * d[0] + d[1] * d[1]).sqrt();
        if s.abs() < MARGIN {
            return None;
        }
        let prim = if s < 0.0 { 0 } else { 1 };
        // Triangle 0's normal is +z: a +z ray (layer 0) sees its back.
        let front = (prim == 0) == (z == 1);
        let bits = if front { 1 | 4 | 16 | 32 } else { 2 | 8 | 16 | 64 };
        r[..4].copy_from_slice(&[1.0, hit_type(sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_TRIANGLE as u32), bits as f32, prim as f32]);
        let v: Vec<f32> = TRIS[prim].iter().flat_map(|i| QUAD[*i as usize]).collect();
        r[4..13].copy_from_slice(&v);
        r[20..29].copy_from_slice(&v);
        r[36] = 1.0;
        return Some(r);
    }
    let ds = SPHERE[3] - ((px - SPHERE[0]).powi(2) + (py - SPHERE[1]).powi(2)).sqrt();
    if ds.abs() < MARGIN {
        return None;
    }
    if ds > 0.0 {
        r[..4].copy_from_slice(&[2.0, hit_type(sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_SPHERE as u32), 33.0, 0.0]);
        r[4..8].copy_from_slice(&SPHERE);
        r[20..24].copy_from_slice(&SPHERE);
        r[36] = 1.0;
        return Some(r);
    }
    // Curve: segment k spans y in [CURVE_Y[k + 1], CURVE_Y[k + 2]] (uniform collinear points).
    let dx = CURVE_R - (px - CURVE_X).abs();
    let (ylo, yhi) = (CURVE_Y[1], CURVE_Y[4]);
    if dx.abs() < MARGIN || (dx > 0.0 && ((py - ylo).abs() < CURVE_R + MARGIN || (py - yhi).abs() < CURVE_R + MARGIN)) {
        return None;
    }
    if dx > 0.0 && py > ylo && py < yhi {
        let k = (1..4).find(|k| py < CURVE_Y[k + 1])? - 1;
        if (py - CURVE_Y[k + 1]).abs() < 0.01 || (py - CURVE_Y[k + 2]).abs() < 0.01 {
            return None;
        }
        r[..4].copy_from_slice(&[3.0, hit_type(sys::OptixPrimitiveType_OPTIX_PRIMITIVE_TYPE_ROUND_CUBIC_BSPLINE as u32), 33.0, k as f32]);
        let v: Vec<f32> = (k..k + 4).flat_map(|i| [CURVE_X, CURVE_Y[i], 0.0, CURVE_R]).collect();
        r[4..20].copy_from_slice(&v);
        r[20..36].copy_from_slice(&v);
        r[36] = 1.0;
        r[37] = (py - CURVE_Y[k + 1]) / (CURVE_Y[k + 2] - CURVE_Y[k + 1]);
        return Some(r);
    }
    Some(r)
}

/// Record slots compared, with their tolerance (37: curve parameter, approximate).
fn checked(kind: f32) -> Vec<(usize, f32)> {
    let data = match kind as u32 {
        1 => 4..13,
        2 => 4..8,
        3 => 4..20,
        _ => 0..0,
    };
    let mut v: Vec<(usize, f32)> = (0..4).map(|i| (i, 0.0)).collect();
    v.extend(data.clone().map(|i| (i, 0.0)));
    v.extend(data.map(|i| (i + 16, 0.0)));
    if kind > 0.0 {
        v.extend([(36, 0.0), (38, 0.0)]);
    }
    if kind == 3.0 {
        v.push((37, 0.02));
    }
    v
}

/// `OptixBuildFlags` of `BuildOptions { random_vertex_access: true, ..Default::default() }`,
/// which the built-in intersection modules must be compiled for.
fn build_flags() -> u32 {
    (sys::OptixBuildFlags_OPTIX_BUILD_FLAG_PREFER_FAST_TRACE | sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_COMPACTION | sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_RANDOM_VERTEX_ACCESS) as u32
}

/// The programs this case links.
const ENTRIES: &[&str] = &["__raygen__geo", "__miss__geo", "__closesthit__geo_tri", "__closesthit__geo_sphere", "__closesthit__geo_curve"];

/// Runs the case.
pub fn run(g: &Gpu) -> R {
    let mut o = opts("params_geo", size_of::<Params>())?;
    o.primitive_types = (sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_TRIANGLE
        | sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_SPHERE
        | sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_ROUND_CUBIC_BSPLINE) as u32;
    let module = g.module(&Default::default(), &o, ENTRIES)?;
    let is = |primitive| Module::builtin_intersection(&g.ctx, &Default::default(), &o, BuiltinIs { primitive, motion_blur: false, build_flags: build_flags(), endcaps: false });
    let sphere_is = is(Primitive::Sphere).ctx("sphere intersector")?;
    let curve_is = is(Primitive::RoundCubicBspline).ctx("curve intersector")?;
    let raygen = ProgramGroup::raygen(&module, "__raygen__geo").ctx("raygen")?;
    let miss = ProgramGroup::miss(&module, "__miss__geo").ctx("miss")?;
    let tri = ProgramGroup::hit_group(&g.ctx, HitGroup { closest_hit: Some((&module, "__closesthit__geo_tri")), ..Default::default() }).ctx("triangle hit group")?;
    let sph = ProgramGroup::hit_group(&g.ctx, HitGroup::builtin(Some((&module, "__closesthit__geo_sphere")), &sphere_is)).ctx("sphere hit group")?;
    let crv = ProgramGroup::hit_group(&g.ctx, HitGroup::builtin(Some((&module, "__closesthit__geo_curve")), &curve_is)).ctx("curve hit group")?;
    let pipeline = g.pipeline(&o, LinkOptions::default(), &[raygen.clone(), miss.clone(), tri.clone(), sph.clone(), crv.clone()])?;
    let hits = [SbtRecord::new(&tri, 0u32), SbtRecord::new(&sph, 0u32), SbtRecord::new(&crv, 0u32)].into_iter().collect::<optix::Result<Vec<_>>>().ctx("hit records")?;
    let sbt = ShaderBindingTable::new(&g.stream, &SbtRecord::new(&raygen, 0u32).ctx("raygen record")?, &[SbtRecord::new(&miss, 0u32).ctx("miss record")?], &hits).ctx("SBT")?;

    let bo = BuildOptions { random_vertex_access: true, ..Default::default() };
    let mut scratch = Scratch::new(&g.stream);
    let verts = g.up(&QUAD)?;
    let index = g.up(&TRIS)?;
    let mesh = TriangleMesh { indices: Some(Indices::U32(&index)), flags: &[GeometryFlags::DISABLE_ANYHIT], ..TriangleMesh::new(&verts) };
    let tri_gas = Accel::triangles(&g.ctx, &g.stream, &mut scratch, &[mesh], bo).ctx("triangle GAS")?;
    let centers = g.up(&[[SPHERE[0], SPHERE[1], SPHERE[2]]])?;
    let radius = g.up(&[SPHERE[3]])?;
    let spheres = Spheres { flags: &[GeometryFlags::DISABLE_ANYHIT], ..Spheres::new(&centers, Radii::Single(Keys::One(&radius))) };
    let sph_gas = Accel::spheres(&g.ctx, &g.stream, &mut scratch, &[spheres], bo).ctx("sphere GAS")?;
    let cps = g.up(&CURVE_Y.map(|y| [CURVE_X, y, 0.0]))?;
    let widths = g.up(&[CURVE_R; 6])?;
    let segs = g.up(&[0u32, 1, 2])?;
    let curves = Curves {
        kind: CurveType::RoundCubicBspline,
        vertices: Keys::One(&cps),
        widths: Keys::One(&widths),
        normals: None,
        segments: &segs,
        flags: GeometryFlags::DISABLE_ANYHIT,
        endcaps: false,
        primitive_offset: 0,
    };
    let crv_gas = Accel::curves(&g.ctx, &g.stream, &mut scratch, &[curves], bo).ctx("curve GAS")?;
    let mut ti = Instance::new(&tri_gas);
    ti.transform[3] = TRI_DX;
    let insts = [ti, Instance { sbt_offset: 1, ..Instance::new(&sph_gas) }, Instance { sbt_offset: 2, ..Instance::new(&crv_gas) }];
    let ias = Accel::instances(&g.ctx, &g.stream, &mut scratch, &insts, BuildOptions::default()).ctx("IAS")?;

    let n = (W * H * 2) as usize;
    let out = g.zeros::<f32>(n * REC)?;
    let params = Params { out: out.cu_deviceptr(), handle: ias.handle(), width: W, height: H };
    // SAFETY: `params` matches the device struct; no program reads SBT data; `out` holds `REC`
    // floats per launch index; everything outlives the launch (it synchronizes).
    unsafe { g.launch(&pipeline, &sbt, &params, [W, H, 2]) }?;
    let got = g.down(&out)?;

    let (mut gv, mut wv, mut labels) = (Vec::new(), Vec::new(), Vec::new());
    let mut kinds = [0usize; 4];
    for z in 0..2 {
        for y in 0..H {
            for x in 0..W {
                let Some(want) = expected(x, y, z) else { continue };
                let base = (((z * H + y) * W + x) as usize) * REC;
                kinds[want[0] as usize] += 1;
                for (slot, tol) in checked(want[0]) {
                    let g = got[base + slot];
                    if (g - want[slot]).abs() > tol {
                        gv.push(g);
                        wv.push(want[slot]);
                        labels.push(format!("({x},{y},{z}) slot {slot}"));
                    }
                }
            }
        }
    }
    close("geometry records", &gv, &wv, 0.0, |i| labels[i].clone())?;
    // Every kind must actually have been checked on both layers.
    if kinds.iter().any(|k| *k < 8) {
        return Err(format!("too few checked pixels per kind (miss, tri, sphere, curve): {kinds:?}"));
    }
    println!("geometry: checked pixels (miss, triangle, sphere, curve) = {kinds:?}");
    Ok(())
}
