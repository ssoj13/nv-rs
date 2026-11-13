//! Cases `payload_types` and `callables`.
//!
//! **`payload_types`.** The module declares two payload types (`ModuleOptions::payload_types`,
//! pipeline `payload_values = 0`): type 0 has 2 words, type 1 has 4. The raygen traces every
//! ray twice, once per type (`TraceOptions::payload_type`, ray type = SBT offset, miss index);
//! closest-hit and miss programs per type call `payload::set_types` and are grouped with their
//! type (`ProgramGroup::new(.., Some(type))`). The host checks every payload word of both traces
//! for hits and misses.
//!
//! **`callables`.** The raygen calls direct callables with `u32`, `f32`, `u64` (a pointer the
//! callee reads through) and `f64` arguments and an `f32` result, and a void one that stores
//! through a pointer; the closest-hit calls a continuation callable (`continuation_call!`) that
//! returns a value computed from its arguments. Stack sizes come from the stack helpers
//! (`StackSizes::accumulate` + `compute` + `Pipeline::set_stack`). The host checks every value.

use super::common::{Ctx, Gpu, R, close, opts, same};
use bytemuck::{Pod, Zeroable};
use cuda_device::{ConstantMemory, constant, cuda_module, device, kernel};
use optix::{
    Accel, BuildOptions, GeometryFlags, HitGroup, LinkOptions, ModuleOptions, PayloadSemantics, PayloadType, ProgramGroup, Programs, SbtRecord, Scratch, ShaderBindingTable, StackSizes,
    TriangleMesh,
};
use optix_device::sys as dsys;
use optix_device::trace::{Ray, TraceOptions};
use optix_device::{payload, state, trace};

const N: u32 = 32;

/// Launch parameters of both cases (`params_pt`, `params_call`).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    /// Output words.
    pub out: u64,
    /// The GAS.
    pub handle: u64,
}

// SAFETY: plain integers; all-zero is valid.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

/// Ray `x` of either case: the even ones hit the triangle.
fn ray(x: u32) -> Ray {
    let ox = if x % 2 == 0 { 0.25 } else { 2.0 };
    Ray::new([ox, 0.25 + x as f32 * 0.01, -1.0], [0.0, 0.0, 1.0], 0.0, 1.0e16)
}

const ID0: u32 = dsys::OptixPayloadTypeID_OPTIX_PAYLOAD_TYPE_ID_0 as u32;
const ID1: u32 = dsys::OptixPayloadTypeID_OPTIX_PAYLOAD_TYPE_ID_1 as u32;

/// `fn(u32, f32, u64, f64) -> f32`: `a + b + *p + d`.
#[device]
pub fn __direct_callable__mix(a: u32, b: f32, p: u64, d: f64) -> f32 {
    // SAFETY: the raygen passes a pointer to a live `f32`.
    let v = unsafe { (p as *const f32).read() };
    a as f32 + b + v + d as f32
}

/// `fn(u64, u32)`: stores `v * 3` at `p`.
#[device]
pub fn __direct_callable__store(p: u64, v: u32) {
    // SAFETY: the raygen passes a pointer to a live `u32`.
    unsafe { (p as *mut u32).write(v * 3) }
}

/// `fn(u32, f32) -> f32`: `x * 0.5 + y`.
#[device]
pub fn __continuation_callable__cc(x: u32, y: f32) -> f32 {
    x as f32 * 0.5 + y
}

#[cuda_module]
mod kernels {
    use super::*;

    #[constant(export_name = "params_pt")]
    static PT: ConstantMemory<Params> = ConstantMemory::UNINIT;

    #[constant(export_name = "params_call")]
    static CALL: ConstantMemory<Params> = ConstantMemory::UNINIT;

    #[kernel]
    pub fn __raygen__pt() {
        // SAFETY: raygen of the payload-type pipeline; `out` holds 6 words per thread.
        unsafe {
            let x = state::launch_index()[0];
            let p = PT.get();
            let mut a = [x, 0u32];
            trace::trace(p.handle, &ray(x), &TraceOptions { payload_type: ID0, sbt_offset: 0, sbt_stride: 2, miss_index: 0, ..TraceOptions::default() }, &mut a);
            let mut b = [x, 0u32, 0, 0];
            trace::trace(p.handle, &ray(x), &TraceOptions { payload_type: ID1, sbt_offset: 1, sbt_stride: 2, miss_index: 1, ..TraceOptions::default() }, &mut b);
            let o = (p.out as *mut u32).add(6 * x as usize);
            *o = a[0];
            *o.add(1) = a[1];
            *o.add(2) = b[0];
            *o.add(3) = b[1];
            *o.add(4) = b[2];
            *o.add(5) = b[3];
        }
    }

    #[kernel]
    pub fn __closesthit__pt0() {
        // SAFETY: closest-hit of payload type 0 (2 words).
        unsafe {
            payload::set_types(ID0);
            let p = payload::get::<[u32; 2]>();
            payload::set(&[p[0] * 2, 0xA0]);
        }
    }

    #[kernel]
    pub fn __closesthit__pt1() {
        // SAFETY: closest-hit of payload type 1 (4 words).
        unsafe {
            payload::set_types(ID1);
            let p = payload::get::<[u32; 4]>();
            payload::set(&[p[0] * 3, 0xB1, state::primitive_index() + 40, 7]);
        }
    }

    #[kernel]
    pub fn __miss__pt0() {
        // SAFETY: miss of payload type 0.
        unsafe {
            payload::set_types(ID0);
            let p = payload::get::<[u32; 2]>();
            payload::set(&[p[0] + 1000, 0xA9]);
        }
    }

    #[kernel]
    pub fn __miss__pt1() {
        // SAFETY: miss of payload type 1.
        unsafe {
            payload::set_types(ID1);
            let p = payload::get::<[u32; 4]>();
            payload::set(&[p[0] + 2000, 0xB9, 0, 9]);
        }
    }

    #[kernel]
    pub fn __raygen__call() {
        // SAFETY: raygen of the callables pipeline: direct callables 0 (mix) and 1 (store);
        // `out` holds 4 words per thread.
        unsafe {
            let x = state::launch_index()[0];
            let p = CALL.get();
            let o = (p.out as *mut u32).add(4 * x as usize);
            *o = (x as f32 * 0.25).to_bits();
            let m = optix_device::direct_call!(0, fn(u32, f32, u64, f64) -> f32, (x, 1.5f32, o as u64, 0.125f64 * x as f64));
            *o = m.to_bits();
            optix_device::direct_call!(1, fn(u64, u32), (o.add(1) as u64, x + 1));
            let mut w = [x, 0u32];
            trace::trace(p.handle, &ray(x), &TraceOptions::default(), &mut w);
            *o.add(2) = w[0];
            *o.add(3) = w[1];
        }
    }

    #[kernel]
    pub fn __closesthit__call() {
        // SAFETY: closest-hit; continuation callable 2 is `fn(u32, f32) -> f32`.
        unsafe {
            let x = payload::word::<0>();
            let r = optix_device::continuation_call!(2, fn(u32, f32) -> f32, (x, 0.75f32));
            payload::set_word::<1>(r.to_bits());
        }
    }

    #[kernel]
    pub fn __miss__call() {
        // SAFETY: miss; the payload is two words.
        unsafe { payload::set_word::<1>(0xDEAD) }
    }
}

fn tri_gas(g: &Gpu) -> R<std::sync::Arc<Accel>> {
    let verts = g.up(&[[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]])?;
    let mut scratch = Scratch::new(&g.stream);
    Accel::triangles(&g.ctx, &g.stream, &mut scratch, &[TriangleMesh { flags: &[GeometryFlags::DISABLE_ANYHIT], ..TriangleMesh::new(&verts) }], BuildOptions::default()).ctx("GAS")
}

/// Runs `payload_types`.
pub fn payload_types(g: &Gpu) -> R {
    let t0 = [PayloadSemantics::CALLER_RW | PayloadSemantics::CH_RW | PayloadSemantics::MS_RW; 2];
    let t1 = [PayloadSemantics::ALL_RW; 4];
    let (pt0, pt1) = (PayloadType { words: &t0 }, PayloadType { words: &t1 });
    let types = [pt0, pt1];
    let mut o = opts("params_pt", size_of::<Params>())?;
    o.payload_values = 0;
    o.single_gas = true;
    let module = g.module(&ModuleOptions { payload_types: &types, ..Default::default() }, &o)?;
    let raygen = ProgramGroup::new(&g.ctx, Programs::Raygen(&module, "__raygen__pt"), None).ctx("raygen")?;
    let ms0 = ProgramGroup::new(&g.ctx, Programs::Miss(&module, "__miss__pt0"), Some(pt0)).ctx("miss 0")?;
    let ms1 = ProgramGroup::new(&g.ctx, Programs::Miss(&module, "__miss__pt1"), Some(pt1)).ctx("miss 1")?;
    let ch0 = ProgramGroup::new(&g.ctx, Programs::HitGroup(HitGroup { closest_hit: Some((&module, "__closesthit__pt0")), ..Default::default() }), Some(pt0)).ctx("hit 0")?;
    let ch1 = ProgramGroup::new(&g.ctx, Programs::HitGroup(HitGroup { closest_hit: Some((&module, "__closesthit__pt1")), ..Default::default() }), Some(pt1)).ctx("hit 1")?;
    let pipeline = g.pipeline(&o, LinkOptions::default(), &[raygen.clone(), ms0.clone(), ms1.clone(), ch0.clone(), ch1.clone()])?;
    let misses = [SbtRecord::new(&ms0, 0u32), SbtRecord::new(&ms1, 0u32)].into_iter().collect::<optix::Result<Vec<_>>>().ctx("miss records")?;
    let hits = [SbtRecord::new(&ch0, 0u32), SbtRecord::new(&ch1, 0u32)].into_iter().collect::<optix::Result<Vec<_>>>().ctx("hit records")?;
    let sbt = ShaderBindingTable::new(&g.stream, &SbtRecord::new(&raygen, 0u32).ctx("rg")?, &misses, &hits).ctx("SBT")?;
    let gas = tri_gas(g)?;
    let out = g.zeros::<u32>(6 * N as usize)?;
    let params = Params { out: out.cu_deviceptr(), handle: gas.handle() };
    // SAFETY: `params` matches the device struct; `out` holds 6 words per thread; no SBT data.
    unsafe { g.launch(&pipeline, &sbt, &params, [N, 1, 1]) }?;
    let got = g.down(&out)?;
    let want: Vec<u32> = (0..N).flat_map(|x| if x % 2 == 0 { [x * 2, 0xA0, x * 3, 0xB1, 40, 7] } else { [x + 1000, 0xA9, x + 2000, 0xB9, 0, 9] }).collect();
    same("payloads of type 0 (2 words) and type 1 (4 words)", &got, &want, |i| format!("ray {} word {}", i / 6, i % 6))
}

/// Runs `callables`.
pub fn callables(g: &Gpu) -> R {
    let mut o = opts("params_call", size_of::<Params>())?;
    o.single_gas = true;
    let module = g.module(&Default::default(), &o)?;
    let raygen = ProgramGroup::raygen(&module, "__raygen__call").ctx("raygen")?;
    let miss = ProgramGroup::miss(&module, "__miss__call").ctx("miss")?;
    let hit = ProgramGroup::hit_group(&g.ctx, HitGroup { closest_hit: Some((&module, "__closesthit__call")), ..Default::default() }).ctx("hit group")?;
    let mix = ProgramGroup::callables(&g.ctx, Some((&module, "__direct_callable__mix")), None).ctx("dc mix")?;
    let store = ProgramGroup::callables(&g.ctx, Some((&module, "__direct_callable__store")), None).ctx("dc store")?;
    let cc = ProgramGroup::callables(&g.ctx, None, Some((&module, "__continuation_callable__cc"))).ctx("cc")?;
    let groups = [raygen.clone(), miss.clone(), hit.clone(), mix.clone(), store.clone(), cc.clone()];
    let link = LinkOptions { max_trace_depth: 1, max_cc_depth: 1, max_dc_depth_from_state: 1, max_dc_depth_from_traversal: 0, max_graph_depth: 1 };
    let pipeline = g.pipeline(&o, link, &groups)?;
    let mut acc = StackSizes::default();
    for gr in &groups {
        acc.accumulate(gr, Some(&pipeline)).ctx("stack sizes")?;
    }
    let stack = acc.compute(1, 1, 1).ctx("compute stack")?;
    pipeline.set_stack(stack, 1).ctx("set_stack")?;
    let sbt = ShaderBindingTable::new(&g.stream, &SbtRecord::new(&raygen, 0u32).ctx("rg")?, &[SbtRecord::new(&miss, 0u32).ctx("ms")?], &[SbtRecord::new(&hit, 0u32).ctx("hg")?])
        .and_then(|s| s.with_callables(&g.stream, &[SbtRecord::new(&mix, 0u32)?, SbtRecord::new(&store, 0u32)?, SbtRecord::new(&cc, 0u32)?]))
        .ctx("SBT")?;
    let gas = tri_gas(g)?;
    let out = g.zeros::<u32>(4 * N as usize)?;
    let params = Params { out: out.cu_deviceptr(), handle: gas.handle() };
    // SAFETY: `params` matches the device struct; `out` holds 4 words per thread; no SBT data.
    unsafe { g.launch(&pipeline, &sbt, &params, [N, 1, 1]) }?;
    let got = g.down(&out)?;
    for x in 0..N {
        let r = &got[4 * x as usize..][..4];
        let f = |w: u32| f32::from_bits(w);
        let mix = x as f32 + 1.5 + x as f32 * 0.25 + (0.125f64 * f64::from(x)) as f32;
        close(&format!("thread {x} direct callable (u32, f32, u64, f64) -> f32"), &[f(r[0])], &[mix], 1e-5, |_| String::new())?;
        let cc = if x % 2 == 0 { (x as f32 * 0.5 + 0.75).to_bits() } else { 0xDEAD };
        same(&format!("thread {x} (void callable store, payload x, continuation result / miss)"), &r[1..4], &[(x + 1) * 3, x, cc], |j| format!("[{j}]"))?;
    }
    println!("callables: {N} threads, stack {stack:?}");
    Ok(())
}
