//! GPU tests (T2) for the safe OptiX API: run with `cargo test -p optix -- --include-ignored` on a
//! machine with an NVIDIA GPU and driver.
//!
//! Device code is `fixtures/s1b.ptx`: the PTX cuda-oxide produced from Rust in spike
//! `spikes/s1b-optix-trace` (raygen `__raygen__mask` traces one +z ray per pixel and writes the
//! hit flag; `__closesthit__hit` / `__miss__none` set payload 0 to 1 / 0). Its launch params are
//! `{ out: u64, handle: u64, width: u32, height: u32 }` under the symbol `params`.

use cuda_core::{Context, DeviceBuffer, Stream};
use optix::{Accel, BuildOptions, DeviceContext, Error, GeometryFlags, HitGroup, Instance, LinkOptions, Module, ModuleOptions, Pipeline, PipelineOptions, ProgramGroup, SbtRecord, ShaderBindingTable, TriangleMesh};
use std::sync::Arc;

const PTX: &[u8] = include_bytes!("fixtures/s1b.ptx");
const W: u32 = 16;
const H: u32 = 16;
/// Triangle legs, as in the spike: no pixel centre lies within 0.03 of an edge.
const LEG: f32 = 1.03;
const PARAMS_SIZE: usize = 24;

struct Rig {
    stream: Arc<Stream>,
    ctx: Arc<DeviceContext>,
    pipeline: Pipeline,
    sbt: ShaderBindingTable,
}

fn rig(single_gas: bool) -> Rig {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let ctx = DeviceContext::new(&cuda, None, 0, false).expect("OptiX device context");
    let mut opts = PipelineOptions::new("params", PARAMS_SIZE, 1).expect("options");
    opts.single_gas = single_gas;
    let (module, _log) = Module::new(&ctx, &ModuleOptions::default(), &opts, PTX).expect("module");
    let raygen = ProgramGroup::raygen(&module, "__raygen__mask").expect("raygen");
    let miss = ProgramGroup::miss(&module, "__miss__none").expect("miss");
    let hit = ProgramGroup::hit_group(&ctx, HitGroup { closest_hit: Some((&module, "__closesthit__hit")), ..Default::default() }).expect("hit group");
    let (pipeline, _log) = Pipeline::new(&ctx, &opts, LinkOptions::default(), &[raygen.clone(), miss.clone(), hit.clone()]).expect("pipeline");
    let sbt = ShaderBindingTable::new(
        &stream,
        &SbtRecord::new(&raygen, 0u32).expect("rg record"),
        &[SbtRecord::new(&miss, 0u32).expect("miss record")],
        &[SbtRecord::new(&hit, 0u32).expect("hit record")],
    )
    .expect("sbt");
    Rig { stream, ctx, pipeline, sbt }
}

fn triangle(rig: &Rig) -> Arc<Accel> {
    let verts = DeviceBuffer::from_host(&rig.stream, &[[0.0f32, 0.0, 0.0], [LEG, 0.0, 0.0], [0.0, LEG, 0.0]]).expect("vertices");
    let mesh = TriangleMesh { vertices: &verts, indices: None, flags: GeometryFlags::DISABLE_ANYHIT };
    Accel::triangles(&rig.ctx, &rig.stream, &[mesh], BuildOptions::default()).expect("GAS")
}

fn params(out: &DeviceBuffer<u32>, handle: u64) -> Vec<u8> {
    [out.cu_deviceptr().to_le_bytes().as_slice(), &handle.to_le_bytes(), &W.to_le_bytes(), &H.to_le_bytes()].concat()
}

/// Launches the fixture over `handle` and returns the per-pixel hit flags.
fn trace(rig: &Rig, handle: u64) -> Vec<u32> {
    let out = DeviceBuffer::<u32>::from_host(&rig.stream, &[7; (W * H) as usize]).expect("out");
    let params = DeviceBuffer::from_host(&rig.stream, &params(&out, handle)).expect("params");
    // SAFETY: `params` matches the fixture's launch-parameter struct; `out` holds W*H u32; the
    // SBT records carry no data the programs read; everything outlives the sync below.
    unsafe { rig.pipeline.launch(&rig.stream, &params, &rig.sbt, [W, H, 1]) }.expect("launch");
    rig.stream.synchronize().expect("sync");
    out.to_host_vec(&rig.stream).expect("read back")
}

/// Hit flags expected for the triangle translated by `dx` along x.
fn mask(dx: f32) -> Vec<u32> {
    (0..H)
        .flat_map(|y| {
            (0..W).map(move |x| {
                let (u, v) = ((x as f32 + 0.5) / W as f32 - dx, (y as f32 + 0.5) / H as f32);
                u32::from(u >= 0.0 && u + v < LEG)
            })
        })
        .collect()
}

#[test]
#[ignore = "GPU"]
fn traces_a_compacted_triangle_gas() {
    let rig = rig(true);
    let gas = triangle(&rig);
    assert!(gas.size_bytes() > 0);
    assert_eq!(trace(&rig, gas.handle()), mask(0.0));
}

#[test]
#[ignore = "GPU"]
fn traces_through_an_instance_structure() {
    let rig = rig(false);
    let gas = triangle(&rig);
    // The x offset is a multiple of the pixel pitch (1/16) so no centre lands on an edge.
    let dx = 0.25;
    let mut inst = Instance::new(&gas);
    inst.transform[3] = dx;
    let ias = Accel::instances(&rig.ctx, &rig.stream, &[inst], BuildOptions { compact: false, ..Default::default() }).expect("IAS");
    drop(gas); // the IAS keeps the GAS alive
    assert_eq!(trace(&rig, ias.handle()), mask(dx));
}

#[test]
#[ignore = "GPU"]
fn builds_over_budget_are_refused_before_allocating() {
    let rig = rig(true);
    let verts = DeviceBuffer::from_host(&rig.stream, &[[0.0f32, 0.0, 0.0], [LEG, 0.0, 0.0], [0.0, LEG, 0.0]]).expect("vertices");
    let mesh = TriangleMesh { vertices: &verts, indices: None, flags: GeometryFlags::NONE };
    let opts = BuildOptions { budget: Some(64), ..Default::default() };
    let Err(Error::Budget { requested, available }) = Accel::triangles(&rig.ctx, &rig.stream, &[mesh], opts) else {
        panic!("a 64-byte budget cannot hold a GAS");
    };
    assert_eq!(available, 64);
    assert!(requested > 64);
}

#[test]
#[ignore = "GPU"]
fn rejects_launch_parameters_of_the_wrong_size() {
    let rig = rig(true);
    let params = DeviceBuffer::<u8>::zeroed(&rig.stream, PARAMS_SIZE + 8).expect("params");
    // SAFETY: rejected before reaching OptiX.
    let err = unsafe { rig.pipeline.launch(&rig.stream, &params, &rig.sbt, [W, H, 1]) }.expect_err("size mismatch");
    assert!(matches!(err, Error::Invalid(_)), "{err}");
}

#[test]
#[ignore = "GPU"]
fn compile_errors_carry_the_log() {
    let rig = rig(true);
    let opts = PipelineOptions::new("params", PARAMS_SIZE, 1).expect("options");
    let Err(Error::Optix { call, log, .. }) = Module::new(&rig.ctx, &ModuleOptions::default(), &opts, b"not ptx at all") else {
        panic!("garbage PTX must fail to compile");
    };
    assert_eq!(call, "optixModuleCreate");
    assert!(!log.is_empty(), "OptiX explains why");
}
