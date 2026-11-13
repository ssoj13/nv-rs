//! GPU tests (T2) for the safe OptiX API: run with `cargo test -p optix -- --include-ignored` on a
//! machine with an NVIDIA GPU and driver.
//!
//! Device code is `fixtures/s1b.ptx`: the PTX cuda-oxide produced from Rust in spike
//! `spikes/s1b-optix-trace` (raygen `__raygen__mask` traces one +z ray per pixel and writes the
//! hit flag; `__closesthit__hit` / `__miss__none` set payload 0 to 1 / 0). Its launch params are
//! `{ out: u64, handle: u64, width: u32, height: u32 }` under the symbol `params`.

use cuda_core::{Context, DeviceBuffer, Stream};
use optix::{
    Accel, BuildOptions, Denoiser, DenoiserModel, DenoiserOptions, DeviceContext, Error, GeometryFlags, HitGroup, Image, Instance, LinkOptions, Module, ModuleOptions, Pipeline,
    PipelineOptions, ProgramGroup, SbtRecord, Scratch, ShaderBindingTable, TriangleMesh,
};
use std::sync::Arc;

const PTX: &[u8] = include_bytes!("fixtures/s1b.ptx");
const W: u32 = 16;
const H: u32 = 16;
/// Triangle legs, as in the spike: no pixel centre lies within 0.03 of an edge.
const LEG: f32 = 1.03;
/// Translations used below are multiples of the pixel pitch (1/16) plus a quarter, so no pixel
/// centre lands on an edge after moving.
const DX: f32 = 0.25;
const PARAMS_SIZE: usize = 24;

struct Rig {
    stream: Arc<Stream>,
    ctx: Arc<DeviceContext>,
    scratch: Scratch,
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
    let scratch = Scratch::new(&stream);
    Rig { stream, ctx, scratch, pipeline, sbt }
}

/// The triangle's vertices, translated by `dx` along x.
fn vertices(rig: &Rig, dx: f32) -> DeviceBuffer<[f32; 3]> {
    DeviceBuffer::from_host(&rig.stream, &[[dx, 0.0, 0.0], [dx + LEG, 0.0, 0.0], [dx, LEG, 0.0]]).expect("vertices")
}

fn triangle(rig: &mut Rig, opts: BuildOptions) -> Arc<Accel> {
    let verts = vertices(rig, 0.0);
    let mesh = TriangleMesh { vertices: &verts, indices: None, flags: GeometryFlags::DISABLE_ANYHIT };
    Accel::triangles(&rig.ctx, &rig.stream, &mut rig.scratch, &[mesh], opts).expect("GAS")
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
    let mut rig = rig(true);
    let gas = triangle(&mut rig, BuildOptions::default());
    assert!(gas.size_bytes() > 0);
    assert_eq!(trace(&rig, gas.handle()), mask(0.0));
}

#[test]
#[ignore = "GPU"]
fn traces_through_an_instance_structure() {
    let mut rig = rig(false);
    let gas = triangle(&mut rig, BuildOptions::default());
    let mut inst = Instance::new(&gas);
    inst.transform[3] = DX;
    let ias = Accel::instances(&rig.ctx, &rig.stream, &mut rig.scratch, &[inst], BuildOptions { compact: false, ..Default::default() }).expect("IAS");
    drop(gas); // the IAS keeps the GAS alive
    assert_eq!(trace(&rig, ias.handle()), mask(DX));
}

#[test]
#[ignore = "GPU"]
fn refits_moved_vertices_and_instances_in_place() {
    let mut rig = rig(false);
    let updatable = BuildOptions { allow_update: true, ..Default::default() };
    let gas = triangle(&mut rig, updatable);
    let ias = Accel::instances(&rig.ctx, &rig.stream, &mut rig.scratch, &[Instance::new(&gas)], updatable).expect("IAS");
    assert_eq!(trace(&rig, ias.handle()), mask(0.0));

    // Move the vertices, refit the GAS, then the IAS (its bounds are stale otherwise).
    let moved = vertices(&rig, DX);
    let mesh = TriangleMesh { vertices: &moved, indices: None, flags: GeometryFlags::DISABLE_ANYHIT };
    // SAFETY: no launch is in flight; launches below are ordered after the refits on this stream.
    unsafe { gas.refit_triangles(&rig.stream, &mut rig.scratch, &[mesh]) }.expect("refit GAS");
    // SAFETY: as above.
    unsafe { ias.refit_instances(&rig.stream, &mut rig.scratch, &[Instance::new(&gas)]) }.expect("refit IAS");
    assert_eq!(trace(&rig, ias.handle()), mask(DX));

    // Move the instance back by -DX: the vertex move and the transform cancel out.
    let mut inst = Instance::new(&gas);
    inst.transform[3] = -DX;
    // SAFETY: as above.
    unsafe { ias.refit_instances(&rig.stream, &mut rig.scratch, &[inst]) }.expect("refit IAS transform");
    assert_eq!(trace(&rig, ias.handle()), mask(0.0));
}

#[test]
#[ignore = "GPU"]
fn refit_rejects_static_structures_and_changed_topology() {
    let mut rig = rig(true);
    let fixed = triangle(&mut rig, BuildOptions::default());
    let updatable = triangle(&mut rig, BuildOptions { allow_update: true, ..Default::default() });
    let verts = vertices(&rig, 0.0);
    let mesh = || TriangleMesh { vertices: &verts, indices: None, flags: GeometryFlags::DISABLE_ANYHIT };
    // SAFETY: rejected before reaching OptiX.
    let err = unsafe { fixed.refit_triangles(&rig.stream, &mut rig.scratch, &[mesh()]) }.expect_err("static structure");
    assert!(matches!(err, Error::Invalid(_)), "{err}");
    // SAFETY: rejected before reaching OptiX.
    let err = unsafe { updatable.refit_triangles(&rig.stream, &mut rig.scratch, &[mesh(), mesh()]) }.expect_err("two meshes instead of one");
    assert!(matches!(err, Error::Invalid(_)), "{err}");
}

#[test]
#[ignore = "GPU"]
fn scratch_is_reused_across_builds() {
    let mut rig = rig(true);
    let _a = triangle(&mut rig, BuildOptions::default());
    let held = rig.scratch.capacity();
    assert!(held > 0);
    let _b = triangle(&mut rig, BuildOptions::default());
    assert_eq!(rig.scratch.capacity(), held, "an equal-sized build reuses the scratch");
    let other = rig.ctx.cuda().new_stream().expect("stream");
    let verts = vertices(&rig, 0.0);
    let mesh = TriangleMesh { vertices: &verts, indices: None, flags: GeometryFlags::NONE };
    let err = Accel::triangles(&rig.ctx, &other, &mut rig.scratch, &[mesh], BuildOptions::default()).expect_err("scratch bound to its stream");
    assert!(matches!(err, Error::Invalid(_)), "{err}");
}

#[test]
#[ignore = "GPU"]
fn builds_over_budget_are_refused_before_allocating() {
    let mut rig = rig(true);
    let verts = vertices(&rig, 0.0);
    let mesh = TriangleMesh { vertices: &verts, indices: None, flags: GeometryFlags::NONE };
    let opts = BuildOptions { budget: Some(64), ..Default::default() };
    let Err(Error::Budget { requested, available }) = Accel::triangles(&rig.ctx, &rig.stream, &mut rig.scratch, &[mesh], opts) else {
        panic!("a 64-byte budget cannot hold a GAS");
    };
    assert_eq!(available, 64);
    assert!(requested > 64);
    assert_eq!(rig.scratch.capacity(), 0, "nothing was allocated");
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

/// A flat grey image with deterministic per-channel noise of amplitude `amp`.
fn noisy_grey(n: usize, amp: f32) -> Vec<[f32; 4]> {
    let mut s = 0x2545_F491_u32;
    let mut rnd = move || {
        s ^= s << 13;
        s ^= s >> 17;
        s ^= s << 5;
        (s as f32 / u32::MAX as f32) * 2.0 - 1.0
    };
    (0..n).map(|_| [0.5 + amp * rnd(), 0.5 + amp * rnd(), 0.5 + amp * rnd(), 1.0]).collect()
}

/// Mean absolute deviation of the RGB channels from 0.5.
fn grey_error(px: &[[f32; 4]]) -> f32 {
    px.iter().flat_map(|p| p[..3].iter()).map(|c| (c - 0.5).abs()).sum::<f32>() / (px.len() * 3) as f32
}

fn denoise(model: DenoiserModel, guided: bool) -> (f32, f32) {
    const S: u32 = 128;
    let rig = rig(true);
    let n = (S * S) as usize;
    let noisy = noisy_grey(n, 0.25);
    let input = DeviceBuffer::from_host(&rig.stream, &noisy).expect("input");
    let albedo = DeviceBuffer::from_host(&rig.stream, &vec![[0.5f32, 0.5, 0.5, 1.0]; n]).expect("albedo");
    let normal = DeviceBuffer::from_host(&rig.stream, &vec![[0.0f32, 0.0, 1.0, 0.0]; n]).expect("normal");
    let mut output = DeviceBuffer::<[f32; 4]>::zeroed(&rig.stream, n).expect("output");
    let opts = DenoiserOptions { model, albedo: guided, normal: guided, budget: None };
    let mut den = Denoiser::new(&rig.ctx, &rig.stream, opts, S, S).expect("denoiser");
    assert!(den.memory_bytes() > 0);
    let img = |b| Image::new(b, S, S).expect("image");
    let (a, nm) = if guided { (Some(img(&albedo)), Some(img(&normal))) } else { (None, None) };
    // SAFETY: every image outlives the synchronization below.
    unsafe { den.invoke(&rig.stream, img(&input), a, nm, &mut output, 0.0) }.expect("invoke");
    rig.stream.synchronize().expect("sync");
    let out = output.to_host_vec(&rig.stream).expect("read back");
    assert!(out.iter().flatten().all(|c| c.is_finite()), "finite output");
    (grey_error(&noisy), grey_error(&out))
}

#[test]
#[ignore = "GPU"]
fn hdr_denoiser_flattens_noise() {
    let (before, after) = denoise(DenoiserModel::Hdr, false);
    assert!(after < before * 0.5, "noise {before} -> {after}");
}

#[test]
#[ignore = "GPU"]
fn aov_denoiser_with_guides_flattens_noise() {
    let (before, after) = denoise(DenoiserModel::Aov, true);
    assert!(after < before * 0.5, "noise {before} -> {after}");
}

#[test]
#[ignore = "GPU"]
fn denoiser_rejects_mismatched_guides_and_sizes() {
    let rig = rig(true);
    let mut den = Denoiser::new(&rig.ctx, &rig.stream, DenoiserOptions::default(), 32, 32).expect("denoiser");
    let input = DeviceBuffer::<[f32; 4]>::zeroed(&rig.stream, 32 * 32).expect("input");
    let small = DeviceBuffer::<[f32; 4]>::zeroed(&rig.stream, 16 * 16).expect("small");
    let mut output = DeviceBuffer::<[f32; 4]>::zeroed(&rig.stream, 32 * 32).expect("output");
    let img = Image::new(&input, 32, 32).expect("image");
    // SAFETY: rejected before reaching OptiX.
    let err = unsafe { den.invoke(&rig.stream, img, Some(img), None, &mut output, 0.0) }.expect_err("guide not configured");
    assert!(matches!(err, Error::Invalid(_)), "{err}");
    let small_img = Image::new(&small, 16, 16).expect("small image");
    // SAFETY: rejected before reaching OptiX.
    let err = unsafe { den.invoke(&rig.stream, small_img, None, None, &mut output, 0.0) }.expect_err("size mismatch");
    assert!(matches!(err, Error::Invalid(_)), "{err}");
    assert!(Image::new(&input, 31, 32).is_err(), "length checked");
    let budget = DenoiserOptions { budget: Some(16), ..Default::default() };
    assert!(matches!(Denoiser::new(&rig.ctx, &rig.stream, budget, 32, 32), Err(Error::Budget { .. })));
}
