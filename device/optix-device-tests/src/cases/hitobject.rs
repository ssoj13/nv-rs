//! Case `hitobject`: hit objects and shader execution reordering (`optix_device::hitobject`).
//!
//! An IAS holds two instances (ids 5 and 9, the second translated by +2 in x) of a unit quad of
//! two triangles; the SBT has two hit-group records (closest-hits A and B) and two miss records.
//! Each of 64 raygen threads shoots one +z ray (some hit instance 0, some instance 1, some miss)
//! and records, in order:
//! 1. a normal `trace` (closest-hit A / miss `i % 2` fill an 8-word payload);
//! 2. `traverse` + every getter (`is_hit` / `is_miss` / `is_nop`, instance id and index,
//!    primitive, SBT GAS index, hit kind, ray tmin / tmax / time / flags, SBT record index,
//!    transform-list size, world ray, barycentrics, attributes, GAS handle), then
//!    `reorder_hint` + `invoke`: the payload must equal the normal trace's;
//! 3. `traverse_data` -> `make_nop` (`is_nop`) -> `make_hit` from the data and the saved
//!    transform list -> the same getters -> `invoke`: equal payload again;
//! 4. `traverse` + `set_sbt_record_index(1)` (hits) or `make_miss(1, ..)` (misses) + `invoke`:
//!    closest-hit B / miss 1 runs instead;
//! 5. `traverse` + `reorder` + `invoke`, and `is_nop` after an invoke.
//! The host checks the normal trace against the analytic scene and every hit-object result
//! against the normal trace.

use super::common::{Ctx, Gpu, R, close, opts, same};
use bytemuck::{Pod, Zeroable};
use cuda_device::{ConstantMemory, constant, cuda_module, kernel};
use optix::{Accel, BuildOptions, GeometryFlags, HitGroup, Indices, Instance, LinkOptions, ProgramGroup, SbtRecord, Scratch, ShaderBindingTable, TriangleMesh, sys};
use optix_device::hitobject as ho;
use optix_device::trace::{Ray, TraceOptions};
use optix_device::{payload, state, trace};

const N: u32 = 64;
/// Words per ray record.
const REC: usize = 96;
/// Ray flags every ray carries (checked through `ray_flags`).
const FLAGS: u32 = sys::OptixRayFlags_OPTIX_RAY_FLAG_DISABLE_ANYHIT as u32;

/// Launch parameters (`params_ho`).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    /// `REC` words per ray.
    pub out: u64,
    /// One `u64` per ray: the saved transform list (global memory OptiX reads during `make_hit`).
    pub lists: u64,
    /// The IAS.
    pub handle: u64,
}

// SAFETY: plain integers; all-zero is valid.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

/// What every closest-hit / miss writes: program tag, primitive, instance id, hit distance bits,
/// barycentrics bits, SBT GAS index, hit kind.
type Pay = [u32; 8];

/// The ray of thread `i`.
fn ray(i: u32) -> Ray {
    Ray { origin: [-0.5 + (i as f32 + 0.5) * 0.0625, 0.37, -1.0], direction: [0.0, 0.0, 1.0], tmin: 0.0, tmax: 1.0e16, time: 0.0 }
}

#[cuda_module]
mod kernels {
    use super::*;

    #[constant(export_name = "params_ho")]
    static PARAMS: ConstantMemory<Params> = ConstantMemory::UNINIT;

    /// Writes `v` at `r[at..]`.
    ///
    /// # Safety
    ///
    /// `r[at..at + K]` is writable.
    #[inline(always)]
    unsafe fn put<const K: usize>(r: *mut u32, at: usize, v: [u32; K]) {
        let mut i = 0;
        while i < K {
            // SAFETY: forwarded contract.
            unsafe { *r.add(at + i) = v[i] };
            i += 1;
        }
    }

    /// Every getter of the outgoing hit object, 22 words at `r[at..]`.
    ///
    /// # Safety
    ///
    /// Raygen; `r[at..at + 22]` is writable.
    #[inline(always)]
    unsafe fn getters(r: *mut u32, at: usize) {
        // SAFETY: forwarded contract; triangle getters only on hits.
        unsafe {
            let hit = ho::is_hit();
            let o = ho::world_ray_origin();
            let d = ho::world_ray_direction();
            put(r, at, [u32::from(hit), u32::from(ho::is_miss()), u32::from(ho::is_nop()), ho::sbt_record_index(), ho::transform_list_size()]);
            put(r, at + 5, [ho::ray_tmin().to_bits(), ho::ray_tmax().to_bits(), ho::ray_time().to_bits(), ho::ray_flags()]);
            put(r, at + 9, [o[0].to_bits(), o[1].to_bits(), o[2].to_bits(), d[0].to_bits(), d[1].to_bits(), d[2].to_bits()]);
            if hit {
                let [u, v] = ho::triangle_barycentrics();
                let a = ho::attributes::<[f32; 2]>();
                put(r, at + 15, [ho::instance_id(), ho::instance_index(), ho::primitive_index(), ho::sbt_gas_index(), ho::hit_kind()]);
                put(r, at + 20, [u.to_bits(), v.to_bits()]);
                // Attributes of a built-in triangle are its barycentrics; the GAS handle is set.
                *r.add(at + 4) |= (u32::from(a[0] == u && a[1] == v) << 8) | (u32::from(ho::gas_traversable_handle() != 0) << 9);
            }
        }
    }

    #[kernel]
    pub fn __raygen__ho() {
        // SAFETY: raygen; `params_ho` is set by the host with `REC` words and one list slot per
        // ray; every program reached reads and writes a `Pay`.
        unsafe {
            let i = state::launch_index()[0];
            let p = PARAMS.get();
            let r = (p.out as *mut u32).add(i as usize * REC);
            let ray = ray(i);
            let opts = TraceOptions { ray_flags: FLAGS, miss_index: i % 2, ..TraceOptions::default() };

            let mut a: Pay = [0; 8];
            trace::trace(p.handle, &ray, &opts, &mut a);
            put(r, 0, a);

            let mut b: Pay = [0; 8];
            ho::traverse(p.handle, &ray, &opts, &mut b);
            getters(r, 32);
            ho::reorder_hint(i & 3, 2);
            ho::invoke(&mut b);
            put(r, 8, b);
            *r.add(80) = u32::from(ho::is_nop());

            ho::traverse(p.handle, &ray, &opts, &mut b);
            let hit = ho::is_hit();
            let list = (p.lists as *mut u64).add(i as usize);
            let data = ho::traverse_data();
            if hit {
                *list = ho::transform_list_handle(0);
            }
            ho::make_nop();
            *r.add(81) = u32::from(ho::is_nop());
            *r.add(82) = u32::from(ho::is_hit() || ho::is_miss());
            if hit {
                ho::make_hit(p.handle, &ray, FLAGS, &data, core::slice::from_raw_parts(list, 1));
            } else {
                ho::make_miss(i % 2, &ray, FLAGS);
            }
            getters(r, 54);
            let mut c: Pay = [0; 8];
            ho::invoke(&mut c);
            put(r, 16, c);

            ho::traverse(p.handle, &ray, &opts, &mut b);
            if ho::is_hit() {
                ho::set_sbt_record_index(1);
            } else {
                ho::make_miss(1, &ray, FLAGS);
            }
            *r.add(83) = ho::sbt_record_index();
            *r.add(84) = u32::from(ho::is_miss());
            let mut d: Pay = [0; 8];
            ho::invoke(&mut d);
            put(r, 24, d);

            ho::traverse(p.handle, &ray, &opts, &mut b);
            ho::reorder();
            let mut e: Pay = [0; 8];
            ho::invoke(&mut e);
            put(r, 86, e);
        }
    }

    /// The payload a closest-hit with tag `tag` writes.
    ///
    /// # Safety
    ///
    /// Closest-hit on triangles.
    #[inline(always)]
    unsafe fn hit_payload(tag: u32) -> Pay {
        // SAFETY: forwarded contract.
        unsafe {
            let [u, v] = state::triangle_barycentrics();
            [tag, state::primitive_index(), state::instance_id(), state::ray_tmax().to_bits(), u.to_bits(), v.to_bits(), state::sbt_gas_index(), state::hit_kind()]
        }
    }

    #[kernel]
    pub fn __closesthit__ho_a() {
        // SAFETY: closest-hit on triangles; the payload is a `Pay`.
        unsafe { payload::set(&hit_payload(1)) }
    }

    #[kernel]
    pub fn __closesthit__ho_b() {
        // SAFETY: closest-hit on triangles; the payload is a `Pay`.
        unsafe { payload::set(&hit_payload(2)) }
    }

    #[kernel]
    pub fn __miss__ho0() {
        // SAFETY: miss; the payload is a `Pay`.
        unsafe { payload::set(&[10u32, 0, 0, state::ray_tmax().to_bits(), state::ray_flags(), 0, 0, 0]) }
    }

    #[kernel]
    pub fn __miss__ho1() {
        // SAFETY: miss; the payload is a `Pay`.
        unsafe { payload::set(&[11u32, 0, 0, state::ray_tmax().to_bits(), state::ray_flags(), 0, 0, 0]) }
    }
}

/// Unit quad: triangle 0 below the diagonal (y < x), triangle 1 above.
const QUAD: [[f32; 3]; 4] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]];
const TRIS: [[u32; 3]; 2] = [[0, 1, 2], [0, 2, 3]];
const IDS: [u32; 2] = [5, 9];

/// The normal-trace payload of ray `i`, from the analytic scene.
fn expected(i: u32) -> Pay {
    let r = ray(i);
    let (x, y) = (r.origin[0], r.origin[1]);
    let inst = if (0.0..1.0).contains(&x) { 0 } else if (2.0..3.0).contains(&x) { 1 } else { return [10 + i % 2, 0, 0, r.tmax.to_bits(), FLAGS, 0, 0, 0] };
    let lx = x - 2.0 * inst as f32;
    let prim = u32::from(y > lx);
    // Barycentrics: weights of vertices 1 and 2 of the triangle.
    let [a, b, c] = TRIS[prim as usize].map(|k| QUAD[k as usize]);
    let det = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
    let u = ((lx - a[0]) * (c[1] - a[1]) - (y - a[1]) * (c[0] - a[0])) / det;
    let v = ((b[0] - a[0]) * (y - a[1]) - (b[1] - a[1]) * (lx - a[0])) / det;
    // Seen from -z, the quad's counter-clockwise triangles show their back face.
    [1, prim, IDS[inst], 1.0f32.to_bits(), u.to_bits(), v.to_bits(), 0, sys::OptixHitKind_OPTIX_HIT_KIND_TRIANGLE_BACK_FACE as u32]
}

/// The programs this case links.
const ENTRIES: &[&str] = &["__raygen__ho", "__closesthit__ho_a", "__closesthit__ho_b", "__miss__ho0", "__miss__ho1"];

/// Runs the case.
pub fn run(g: &Gpu) -> R {
    let o = opts("params_ho", size_of::<Params>())?;
    let module = g.module(&Default::default(), &o, ENTRIES)?;
    let raygen = ProgramGroup::raygen(&module, "__raygen__ho").ctx("raygen")?;
    let miss0 = ProgramGroup::miss(&module, "__miss__ho0").ctx("miss 0")?;
    let miss1 = ProgramGroup::miss(&module, "__miss__ho1").ctx("miss 1")?;
    let ha = ProgramGroup::hit_group(&g.ctx, HitGroup { closest_hit: Some((&module, "__closesthit__ho_a")), ..Default::default() }).ctx("hit group A")?;
    let hb = ProgramGroup::hit_group(&g.ctx, HitGroup { closest_hit: Some((&module, "__closesthit__ho_b")), ..Default::default() }).ctx("hit group B")?;
    let pipeline = g.pipeline(&o, LinkOptions::default(), &[raygen.clone(), miss0.clone(), miss1.clone(), ha.clone(), hb.clone()])?;
    let misses = [SbtRecord::new(&miss0, 0u32), SbtRecord::new(&miss1, 0u32)].into_iter().collect::<optix::Result<Vec<_>>>().ctx("miss records")?;
    let hits = [SbtRecord::new(&ha, 0u32), SbtRecord::new(&hb, 0u32)].into_iter().collect::<optix::Result<Vec<_>>>().ctx("hit records")?;
    let sbt = ShaderBindingTable::new(&g.stream, &SbtRecord::new(&raygen, 0u32).ctx("raygen record")?, &misses, &hits).ctx("SBT")?;

    let mut scratch = Scratch::new(&g.stream);
    let verts = g.up(&QUAD)?;
    let index = g.up(&TRIS)?;
    let mesh = TriangleMesh { indices: Some(Indices::U32(&index)), flags: &[GeometryFlags::DISABLE_ANYHIT], ..TriangleMesh::new(&verts) };
    let gas = Accel::triangles(&g.ctx, &g.stream, &mut scratch, &[mesh], BuildOptions::default()).ctx("GAS")?;
    let mut i1 = Instance { id: IDS[1], ..Instance::new(&gas) };
    i1.transform[3] = 2.0;
    let ias = Accel::instances(&g.ctx, &g.stream, &mut scratch, &[Instance { id: IDS[0], ..Instance::new(&gas) }, i1], BuildOptions::default()).ctx("IAS")?;

    let out = g.zeros::<u32>(N as usize * REC)?;
    let lists = g.zeros::<u64>(N as usize)?;
    let params = Params { out: out.cu_deviceptr(), lists: lists.cu_deviceptr(), handle: ias.handle() };
    // SAFETY: `params` matches the device struct; `out` / `lists` are sized per ray; no program
    // reads SBT data; everything outlives the launch (it synchronizes).
    unsafe { g.launch(&pipeline, &sbt, &params, [N, 1, 1]) }?;
    let got = g.down(&out)?;

    let (mut hits_seen, mut misses_seen) = (0, 0);
    for i in 0..N {
        let r = &got[i as usize * REC..(i as usize + 1) * REC];
        let w = expected(i);
        let label = |what: &str| format!("ray {i} {what}");
        let hit = w[0] == 1;
        if hit { hits_seen += 1 } else { misses_seen += 1 }
        let f = |x: u32| f32::from_bits(x);
        // Normal trace vs the analytic scene (floats with a tolerance, the rest exact).
        same(&label("trace (tag, prim, id, sbt gas, kind)"), &[r[0], r[1], r[2], r[6], r[7]], &[w[0], w[1], w[2], w[6], w[7]], |j| format!("[{j}]"))?;
        close(&label("trace (t, u, v)"), &[f(r[3]), f(r[4]), f(r[5])], &[f(w[3]), f(w[4]), f(w[5])], 1e-5, |j| format!("[{j}]"))?;
        // Every hit-object path reproduces the normal trace exactly.
        same(&label("traverse + reorder_hint + invoke"), &r[8..16], &r[0..8], |j| format!("[{j}]"))?;
        same(&label("make_hit(traverse_data) / make_miss + invoke"), &r[16..24], &r[0..8], |j| format!("[{j}]"))?;
        same(&label("traverse + reorder + invoke"), &r[86..94], &r[0..8], |j| format!("[{j}]"))?;
        let switched: Pay = if hit { [2, r[1], r[2], r[3], r[4], r[5], r[6], r[7]] } else { [11, 0, 0, r[3], FLAGS, 0, 0, 0] };
        same(&label("set_sbt_record_index(1) / make_miss(1) + invoke"), &r[24..32], &switched, |j| format!("[{j}]"))?;
        same(&label("(nop after invoke, nop after make_nop, nop is neither hit nor miss, record index 1, is_miss)"), &r[80..85], &[1, 1, 0, 1, u32::from(!hit)], |j| format!("[{j}]"))?;
        // Getters after traverse (32..54) and after make_hit / make_miss (54..76).
        let ray = ray(i);
        for (at, what) in [(32, "after traverse"), (54, "after make_hit / make_miss")] {
            let g = &r[at..at + 22];
            let list = if hit { 1 | 1 << 8 | 1 << 9 } else { 0 };
            let rec = if hit { 0 } else { i % 2 };
            let tmax = if hit { 1.0f32 } else { ray.tmax };
            same(&label(&format!("getters {what}: (hit, miss, nop, record, list|attr|gas)")), &g[0..5], &[u32::from(hit), u32::from(!hit), 0, rec, list], |j| format!("[{j}]"))?;
            same(&label(&format!("getters {what}: (tmin, time, flags)")), &[g[5], g[7], g[8]], &[0.0f32.to_bits(), 0.0f32.to_bits(), FLAGS], |j| format!("[{j}]"))?;
            close(&label(&format!("getters {what}: tmax, world ray")), &[g[6], g[9], g[10], g[11], g[12], g[13], g[14]].map(f), &[tmax, ray.origin[0], ray.origin[1], ray.origin[2], 0.0, 0.0, 1.0], 1e-5, |j| format!("[{j}]"))?;
            if hit {
                same(&label(&format!("getters {what}: (id, index, prim, sbt gas, kind, u, v)")), &g[15..22], &[r[2], (w[2] == IDS[1]) as u32, r[1], r[6], r[7], r[4], r[5]], |j| format!("[{j}]"))?;
            }
        }
    }
    if hits_seen < 16 || misses_seen < 16 {
        return Err(format!("scene degenerate: {hits_seen} hits, {misses_seen} misses"));
    }
    println!("hitobject: {hits_seen} hit rays, {misses_seen} miss rays through trace, traverse/invoke, make_hit, make_miss, nop, reorder, SBT index switch");
    Ok(())
}
