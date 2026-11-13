//! GPU tests of every host feature of the safe OptiX API: context properties, log sink and disk
//! cache; task-based compiles, payload types, bound values and base modules; stack sizing and
//! symbol copies; every geometry input (vertex / index formats, SBT offsets, pre-transforms,
//! curves, spheres, custom primitives, instance pointers, motion), emit, relocation and refit;
//! transforms; opacity micromaps; cluster builds; every denoiser model; cooperative-vector
//! matrices.
//!
//! Run with `cargo test -p optix --test gpu_features -- --include-ignored --test-threads=1` on a
//! machine with an NVIDIA RTX GPU and an OptiX 9.1 driver. Geometry is checked by tracing the
//! `fixtures/s1b.ptx` raygen over it and comparing the hit mask with an analytic one (see
//! `common/mod.rs`).

mod common;

use common::*;
use cuda_core::{DeviceBuffer, DeviceCopy, Stream};
use optix::coop_vec::{self, Batch, ElemType, Matrix, MatrixLayout, Network};
use optix::{
    Aabb, Accel, AovType, BoundValue, BuildOptions, BuiltinIs, ClusterAccel, ClusterArgs, ClusterFlags, ClusterInput, ClusterLimits, ClusterMode, ClusterOptions, ClusterRefs,
    CompileState, CurveType, Curves, CustomPrimitives, Denoiser, DenoiserModel, DenoiserOptions, DeviceContext, DeviceInstances, Emit, Error, GeometryFlags, GridLimits, Guides,
    Image, Indices, Instance, Keys, Layer, LinkOptions, MicromapCount, MicromapDesc, MicromapFormat, MicromapIndices, MicromapOptions, MicromapUse, Module, ModuleOptions,
    MotionOptions, OpacityMicromapArray, Params, PayloadSemantics, PayloadType, Pipeline, PipelineStack, PixelFormat, Primitive, ProgramGroup, Programs, Radii, Relink,
    SbtOffsets, Scratch, Spheres, StackSizes, Srt, Task, Transform, TriangleLimits, TriangleMesh, VertexFormat, Vertices, base_to_micro_bary, error_name, error_string,
    micro_to_base_bary, simple_path_tracer, sys,
};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------------------------

fn up<T: DeviceCopy>(stream: &Stream, v: &[T]) -> DeviceBuffer<T> {
    DeviceBuffer::from_host(stream, v).expect("upload")
}

fn invalid<T>(r: optix::Result<T>, what: &str) {
    match r {
        Err(Error::Invalid(_)) => {}
        Err(e) => panic!("{what}: expected Error::Invalid, got {e}"),
        Ok(_) => panic!("{what}: expected Error::Invalid, got Ok"),
    }
}

fn optix_err<T>(r: optix::Result<T>, want: sys::OptixResult, what: &str) {
    match r {
        Err(Error::Optix { code, .. }) if code == want => {}
        Err(e) => panic!("{what}: expected OptixResult {want}, got {e}"),
        Ok(_) => panic!("{what}: expected OptixResult {want}, got Ok"),
    }
}

fn opaque(verts: &DeviceBuffer<[f32; 3]>) -> TriangleMesh<'_> {
    TriangleMesh { flags: &[GeometryFlags::DISABLE_ANYHIT], ..TriangleMesh::new(verts) }
}

fn tri_gas(rig: &mut Rig, meshes: &[TriangleMesh<'_>], opts: BuildOptions) -> Arc<Accel> {
    Accel::triangles(&rig.ctx, &rig.stream, &mut rig.scratch, meshes, opts).expect("triangle GAS")
}

fn try_tri(rig: &mut Rig, mesh: TriangleMesh<'_>) -> optix::Result<Arc<Accel>> {
    Accel::triangles(&rig.ctx, &rig.stream, &mut rig.scratch, &[mesh], BuildOptions::default())
}

fn try_curves(rig: &mut Rig, c: Curves<'_>) -> optix::Result<Arc<Accel>> {
    Accel::curves(&rig.ctx, &rig.stream, &mut rig.scratch, &[c], BuildOptions::default())
}

fn default_gas(rig: &mut Rig, dx: f32, opts: BuildOptions) -> Arc<Accel> {
    let verts = up(&rig.stream, &tri_verts(dx, LEG));
    tri_gas(rig, &[opaque(&verts)], opts)
}

fn updatable() -> BuildOptions {
    BuildOptions { allow_update: true, ..Default::default() }
}

/// `OptixBuildFlags` that `BuildOptions::default()` (plus `ALLOW_UPDATE` when `update`) builds
/// with; built-in intersectors are compiled for the same flags.
fn build_flags(update: bool) -> u32 {
    let mut f = (sys::OptixBuildFlags_OPTIX_BUILD_FLAG_PREFER_FAST_TRACE | sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_COMPACTION) as u32;
    if update {
        f |= sys::OptixBuildFlags_OPTIX_BUILD_FLAG_ALLOW_UPDATE as u32;
    }
    f
}

fn ias(rig: &mut Rig, instances: &[Instance], opts: BuildOptions) -> Arc<Accel> {
    Accel::instances(&rig.ctx, &rig.stream, &mut rig.scratch, instances, opts).expect("IAS")
}

fn translated(child: impl Into<optix::Child>, dx: f32) -> Instance {
    let mut i = Instance::new(child);
    i.transform[3] = dx;
    i
}

fn shift(dx: f32) -> [f32; 12] {
    let mut m = Instance::IDENTITY;
    m[3] = dx;
    m
}

fn misses() -> Vec<u32> {
    vec![0; (W * H) as usize]
}

// ---------------------------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------------------------

#[test]
#[ignore = "GPU"]
fn context_reports_every_property() {
    let (cuda, _stream) = cuda();
    let ctx = DeviceContext::new(&cuda, None, 0, false).expect("context");
    let limits = [
        ("max_trace_depth", ctx.max_trace_depth()),
        ("max_graph_depth", ctx.max_graph_depth()),
        ("max_prims_per_gas", ctx.max_prims_per_gas()),
        ("max_instances_per_ias", ctx.max_instances_per_ias()),
        ("rtcore_version", ctx.rtcore_version()),
        ("max_instance_id", ctx.max_instance_id()),
        ("max_sbt_records_per_gas", ctx.max_sbt_records_per_gas()),
        ("max_sbt_offset", ctx.max_sbt_offset()),
    ];
    for (name, v) in limits {
        let v = v.unwrap_or_else(|e| panic!("{name}: {e}"));
        println!("{name} = {v}");
        assert!(v > 0, "{name} is 0 on an RTX device");
    }
    assert_eq!(ctx.visibility_mask_bits().expect("visibility bits"), 8, "OptiX documents 8 visibility mask bits");
    assert!(ctx.max_trace_depth().expect("depth") >= 31, "OptiX guarantees trace depth 31");
    let ser = ctx.shader_reordering().expect("shader reordering");
    let coop = ctx.coop_vec().expect("coop vec");
    let clusters = ctx.cluster_accel().expect("cluster accel");
    println!("shader_reordering = {ser}, coop_vec = {coop}, cluster_accel = {clusters}");
    let cluster_limits = [
        ("max_cluster_vertices", ctx.max_cluster_vertices()),
        ("max_cluster_triangles", ctx.max_cluster_triangles()),
        ("max_grid_resolution", ctx.max_grid_resolution()),
        ("max_cluster_sbt_index", ctx.max_cluster_sbt_index()),
        ("max_clusters_per_gas", ctx.max_clusters_per_gas()),
    ];
    for (name, v) in cluster_limits {
        let v = v.unwrap_or_else(|e| panic!("{name}: {e}"));
        println!("{name} = {v}");
        if clusters {
            assert!(v > 0, "{name} is 0 although cluster builds are supported");
        }
    }
}

#[test]
#[ignore = "GPU"]
fn set_log_replaces_the_sink() {
    let (cuda, _stream) = cuda();
    let a = Arc::new(AtomicUsize::new(0));
    let b = Arc::new(Mutex::new(Vec::<String>::new()));
    let a2 = a.clone();
    let ctx = DeviceContext::new(&cuda, Some(Box::new(move |_: u32, _: &str, _: &str| {
        a2.fetch_add(1, Ordering::SeqCst);
    })), 4, false).expect("context");
    ctx.set_cache_enabled(false).expect("cache off");
    let a_before = a.load(Ordering::SeqCst);
    let b2 = b.clone();
    ctx.set_log(Some(Box::new(move |level: u32, tag: &str, msg: &str| b2.lock().expect("log lock").push(format!("[{level}][{tag}] {msg}")))), 4).expect("set_log");
    let opts = options(&Cfg::default(), 1);
    let err = Module::new(&ctx, &ModuleOptions::default(), &opts, b"not ptx at all").map(|_| ()).expect_err("garbage PTX");
    assert!(matches!(err, Error::Optix { call: "optixModuleCreate", .. }), "{err}");
    let got = b.lock().expect("log lock").clone();
    println!("sink B received {} messages, first: {:?}", got.len(), got.first());
    assert!(!got.is_empty(), "the new sink receives the failing compile's messages");
    assert_eq!(a.load(Ordering::SeqCst), a_before, "the replaced sink receives nothing after set_log");

    // Silenced: neither sink hears a second failing compile.
    ctx.set_log(None, 0).expect("silence");
    let n = got.len();
    Module::new(&ctx, &ModuleOptions::default(), &opts, b"still not ptx").map(|_| ()).expect_err("garbage PTX");
    assert_eq!(b.lock().expect("log lock").len(), n, "a silenced context calls no sink");
    assert_eq!(a.load(Ordering::SeqCst), a_before);
}

#[test]
#[ignore = "GPU"]
fn disk_cache_settings_round_trip() {
    let (cuda, _stream) = cuda();
    let ctx = DeviceContext::new(&cuda, None, 0, false).expect("context");
    let env_size = std::env::var("OPTIX_CACHE_MAXSIZE").ok();
    let env_path = std::env::var("OPTIX_CACHE_PATH").ok();
    println!("OPTIX_CACHE_MAXSIZE={env_size:?} OPTIX_CACHE_PATH={env_path:?}");
    assert!(env_size.is_none() && env_path.is_none(), "the cache environment overrides the settings under test; unset it");

    let dir = std::env::temp_dir().join(format!("nv-rs-optix-cache-{}", std::process::id()));
    ctx.set_cache_location(&dir).expect("set location");
    let loc = ctx.cache_location().expect("location");
    println!("cache location = {}", loc.display());
    assert_eq!(loc.file_name(), dir.file_name(), "the location reads back");
    assert!(dir.is_dir(), "OptiX creates the cache directory");

    ctx.set_cache_sizes(1 << 20, 1 << 24).expect("sizes");
    assert_eq!(ctx.cache_sizes().expect("sizes"), (1 << 20, 1 << 24));
    ctx.set_cache_sizes(0, 0).expect("collection off");
    assert_eq!(ctx.cache_sizes().expect("sizes"), (0, 0));
    optix_err(ctx.set_cache_sizes(1 << 24, 1 << 20), sys::OptixResult_OPTIX_ERROR_INVALID_VALUE, "high water mark below the low one");

    ctx.set_cache_enabled(false).expect("disable");
    assert!(!ctx.cache_enabled().expect("enabled"));
    ctx.set_cache_enabled(true).expect("enable");
    assert!(ctx.cache_enabled().expect("enabled"));
    drop(ctx);
    std::fs::remove_dir_all(&dir).expect("remove the cache directory");
}

#[test]
#[ignore = "GPU"]
fn error_names_and_display() {
    let code = sys::OptixResult_OPTIX_ERROR_INVALID_VALUE;
    assert_eq!(error_name(code).expect("name"), "OPTIX_ERROR_INVALID_VALUE");
    let text = error_string(code).expect("string");
    assert!(!text.is_empty() && !text.contains("OPTIX_"), "a description, not the name: {text}");
    assert_eq!(error_name(sys::OptixResult_OPTIX_ERROR_SYMBOL_NOT_FOUND).expect("name"), "OPTIX_ERROR_SYMBOL_NOT_FOUND");
    assert_eq!(error_name(12345).expect("unknown code"), "Unknown OptixResult code");
    let shown = Error::Optix { call: "optixFoo", code, log: "detail".into() }.to_string();
    assert!(shown.contains("optixFoo") && shown.contains("OPTIX_ERROR_INVALID_VALUE (7001)") && shown.contains(text) && shown.contains("detail"), "{shown}");
}

// ---------------------------------------------------------------------------------------------
// Modules and tasks
// ---------------------------------------------------------------------------------------------

/// Context with the disk cache off, so compiles really run their tasks.
fn uncached(validation: bool, log: Option<Box<optix::LogFn>>) -> (Arc<Stream>, Arc<DeviceContext>) {
    let (cuda, stream) = cuda();
    let ctx = DeviceContext::new(&cuda, log, 4, validation).expect("context");
    ctx.set_cache_enabled(false).expect("cache off");
    (stream, ctx)
}

/// Builds the default triangle GAS on `ctx`, links `module` and traces.
fn trace_module(stream: &Arc<Stream>, ctx: &Arc<DeviceContext>, opts: &optix::PipelineOptions, module: &Arc<Module>, payload: Option<PayloadType<'_>>) -> Vec<u32> {
    let groups = groups(ctx, module, None, payload);
    let (pipeline, _) = Pipeline::new(ctx, opts, LinkOptions::default(), &groups).expect("pipeline");
    let sbt = sbt(stream, &groups, 1);
    let mut scratch = Scratch::new(stream);
    let verts = up(stream, &tri_verts(0.0, LEG));
    let gas = Accel::triangles(ctx, stream, &mut scratch, &[opaque(&verts)], BuildOptions::default()).expect("GAS");
    launch(stream, &pipeline, &sbt, gas.handle())
}

/// Runs every task, wave by wave, each task of a wave on its own thread. `replay` maps
/// serialization keys to stored outputs (deserialized instead of executed); `store` collects the
/// outputs of executed serializable tasks. Returns (executed, deserialized, distinct threads).
fn run_tasks(first: Task, replay: &HashMap<Vec<u8>, Vec<u8>>, store: &Mutex<HashMap<Vec<u8>, Vec<u8>>>) -> (usize, usize, usize) {
    let (mut executed, mut replayed) = (0, 0);
    let mut threads = std::collections::HashSet::new();
    let mut wave = vec![first];
    while !wave.is_empty() {
        let results: Vec<(Vec<Task>, bool, std::thread::ThreadId)> = std::thread::scope(|s| {
            let handles: Vec<_> = wave
                .into_iter()
                .map(|mut t| {
                    s.spawn(move || {
                        let key = t.key().expect("key");
                        if let Some(data) = replay.get(&key).filter(|_| !key.is_empty()) {
                            (t.deserialize(data, 8).expect("deserialize"), true, std::thread::current().id())
                        } else {
                            let next = t.execute(8).expect("execute");
                            let out = t.serialize().expect("serialize");
                            if !key.is_empty() && !out.is_empty() {
                                store.lock().expect("store").insert(key, out);
                            }
                            (next, false, std::thread::current().id())
                        }
                    })
                })
                .collect();
            handles.into_iter().map(|h| h.join().expect("task thread")).collect()
        });
        wave = Vec::new();
        for (next, was_replay, id) in results {
            if was_replay {
                replayed += 1;
            } else {
                executed += 1;
            }
            threads.insert(id);
            wave.extend(next);
        }
    }
    (executed, replayed, threads.len())
}

#[test]
#[ignore = "GPU"]
fn task_compile_on_threads_links_and_traces() {
    let (stream, ctx) = uncached(false, None);
    let opts = options(&Cfg::default(), 1);
    let (pending, first) = Module::with_tasks(&ctx, &ModuleOptions::default(), &opts, PTX).expect("with_tasks");
    assert_eq!(pending.state().expect("state"), CompileState::NotStarted);
    invalid(pending.finish(), "finish before any task ran");
    let store = Mutex::new(HashMap::new());
    let (executed, replayed, threads) = run_tasks(first, &HashMap::new(), &store);
    println!("tasks executed = {executed}, threads = {threads}, serialized outputs = {}", store.lock().expect("store").len());
    assert_eq!(replayed, 0);
    assert!(executed >= 1);
    assert_eq!(pending.state().expect("state"), CompileState::Completed);
    let (module, _log) = pending.finish().expect("finish");
    assert_eq!(trace_module(&stream, &ctx, &opts, &module, None), mask(0.0));
}

#[test]
#[ignore = "GPU"]
fn task_outputs_serialize_and_replay() {
    let (stream, ctx) = uncached(false, None);
    let opts = options(&Cfg::default(), 1);
    let store = Mutex::new(HashMap::new());
    let (pending, first) = Module::with_tasks(&ctx, &ModuleOptions::default(), &opts, PTX).expect("with_tasks");
    let (executed, _, _) = run_tasks(first, &HashMap::new(), &store);
    pending.finish().expect("first compile");
    let stored = store.into_inner().expect("store");
    println!("first compile: {executed} tasks, {} serialized outputs", stored.len());
    assert!(!stored.is_empty(), "at least one task of a PTX compile is serializable");

    let (pending, first) = Module::with_tasks(&ctx, &ModuleOptions::default(), &opts, PTX).expect("with_tasks");
    let (executed, replayed, _) = run_tasks(first, &stored, &Mutex::new(HashMap::new()));
    println!("replay compile: {executed} executed, {replayed} deserialized");
    assert!(replayed >= 1, "stored outputs are replayed");
    let (module, _log) = pending.finish().expect("replayed compile");
    assert_eq!(trace_module(&stream, &ctx, &opts, &module, None), mask(0.0));

    // A deserialized task cannot be serialized; an executed one cannot run again.
    let (_pending, mut first) = Module::with_tasks(&ctx, &ModuleOptions::default(), &opts, PTX).expect("with_tasks");
    invalid(first.serialize(), "serialize before execute");
    let _rest = first.execute(8).expect("execute");
    invalid(first.execute(8), "second execute");
}

#[test]
#[ignore = "GPU"]
fn task_compile_cancels() {
    let (_stream, ctx) = uncached(false, None);
    let opts = options(&Cfg::default(), 1);

    let (pending, mut first) = Module::with_tasks(&ctx, &ModuleOptions::default(), &opts, PTX).expect("with_tasks");
    pending.cancel(true).expect("cancel");
    let r = first.execute(8);
    println!("execute after cancel: {:?}", r.as_ref().map(Vec::len).map_err(ToString::to_string));
    optix_err(pending.finish(), sys::OptixResult_OPTIX_ERROR_CREATION_CANCELED, "finish after PendingModule::cancel");

    let (pending, mut first) = Module::with_tasks(&ctx, &ModuleOptions::default(), &opts, PTX).expect("with_tasks");
    ctx.cancel_creations(true).expect("cancel_creations");
    let r = first.execute(8);
    println!("execute after cancel_creations: {:?}", r.as_ref().map(Vec::len).map_err(ToString::to_string));
    optix_err(pending.finish(), sys::OptixResult_OPTIX_ERROR_CREATION_CANCELED, "finish after DeviceContext::cancel_creations");
}

#[test]
#[ignore = "GPU"]
fn payload_types_compile_link_and_trace() {
    let (stream, ctx) = uncached(false, None);
    let words = [PayloadSemantics::CALLER_RW | PayloadSemantics::CH_WRITE | PayloadSemantics::MS_WRITE];
    let pt = PayloadType { words: &words };
    let types = [pt];
    let mo = ModuleOptions { payload_types: &types, ..Default::default() };
    let opts = options(&Cfg::default(), 0);
    let (module, _) = Module::new(&ctx, &mo, &opts, PTX).expect("module with a payload type");
    assert_eq!(trace_module(&stream, &ctx, &opts, &module, Some(pt)), mask(0.0));

    invalid(Module::new(&ctx, &mo, &options(&Cfg::default(), 1), PTX), "payload types with payload_values != 0");
    let empty = [PayloadType { words: &[] }];
    invalid(Module::new(&ctx, &ModuleOptions { payload_types: &empty, ..Default::default() }, &opts, PTX), "empty payload type");
    invalid(ProgramGroup::new(&ctx, Programs::Raygen(&module, "__raygen__mask"), Some(PayloadType { words: &[] })), "empty program-group payload type");
    let two = [PayloadSemantics::ALL_RW, PayloadSemantics::ALL_RW];
    // Miss and closest hit read payload words, so a type the module lacks is refused; raygen
    // only traces and accepts any type (observed).
    let mismatch = sys::OptixResult_OPTIX_ERROR_PAYLOAD_TYPE_MISMATCH;
    optix_err(ProgramGroup::new(&ctx, Programs::Miss(&module, "__miss__none"), Some(PayloadType { words: &two })), mismatch, "miss with a payload type the module lacks");
    let ch = optix::HitGroup { closest_hit: Some((&module, "__closesthit__hit")), ..Default::default() };
    optix_err(ProgramGroup::new(&ctx, Programs::HitGroup(ch), Some(PayloadType { words: &two })), mismatch, "hit group with a payload type the module lacks");
}

#[test]
#[ignore = "GPU"]
fn bound_values_specialize_and_are_validated() {
    let messages = Arc::new(Mutex::new(Vec::<String>::new()));
    let m2 = messages.clone();
    let (stream, ctx) = uncached(true, Some(Box::new(move |level: u32, tag: &str, msg: &str| m2.lock().expect("log").push(format!("[{level}][{tag}] {msg}")))));
    let opts = options(&Cfg::default(), 1);
    let width = W;
    let bounds = [BoundValue::new(16, &width)];
    let mo = ModuleOptions { bound_values: &bounds, ..Default::default() };
    let (module, _) = Module::new(&ctx, &mo, &opts, PTX).expect("module with a bound width");
    let groups = groups(&ctx, &module, None, None);
    let (pipeline, _) = Pipeline::new(&ctx, &opts, LinkOptions::default(), &groups).expect("pipeline");
    let sbt = sbt(&stream, &groups, 1);
    let mut scratch = Scratch::new(&stream);
    let verts = up(&stream, &tri_verts(0.0, LEG));
    let gas = Accel::triangles(&ctx, &stream, &mut scratch, &[opaque(&verts)], BuildOptions::default()).expect("GAS");
    assert_eq!(try_launch(&stream, &pipeline, &sbt, gas.handle(), W).expect("matching launch"), mask(0.0));

    let before = messages.lock().expect("log").len();
    let r = try_launch(&stream, &pipeline, &sbt, gas.handle(), W / 2);
    let logged: Vec<String> = messages.lock().expect("log")[before..].to_vec();
    println!("mismatched bound value: {:?}; log: {logged:?}", r.as_ref().map(|_| ()).map_err(ToString::to_string));
    assert!(r.is_err(), "validation mode rejects a launch whose parameters differ from a bound value");

    invalid(Module::new(&ctx, &ModuleOptions { bound_values: &[BoundValue::new(22, &width)], ..Default::default() }, &opts, PTX), "bound value past the parameters");
    invalid(Module::new(&ctx, &ModuleOptions { bound_values: &[BoundValue { offset: 0, bytes: &[], annotation: None }], ..Default::default() }, &opts, PTX), "empty bound value");
}

#[test]
#[ignore = "GPU"]
fn base_module_specializes() {
    let (stream, ctx) = uncached(false, None);
    let opts = options(&Cfg::default(), 1);
    let (base, _) = Module::new(&ctx, &ModuleOptions::default(), &opts, PTX).expect("base module");
    let (spec, _) = Module::new(&ctx, &ModuleOptions { base: Some(&base), ..Default::default() }, &opts, PTX).expect("module specialized from a base");
    drop(base); // the specialized module holds its base
    assert_eq!(trace_module(&stream, &ctx, &opts, &spec, None), mask(0.0));

    let (_s, other) = uncached(false, None);
    let (foreign, _) = Module::new(&other, &ModuleOptions::default(), &opts, PTX).expect("module on another context");
    invalid(Module::new(&ctx, &ModuleOptions { base: Some(&foreign), ..Default::default() }, &opts, PTX), "base module from another context");
}

#[test]
#[ignore = "GPU"]
fn builtin_intersection_rejects_mismatched_options() {
    let (_stream, ctx) = uncached(false, None);
    let opts = options(&Cfg::default(), 1);
    let sphere = BuiltinIs { primitive: Primitive::Sphere, motion_blur: false, build_flags: build_flags(false), endcaps: false };
    invalid(Module::builtin_intersection(&ctx, &ModuleOptions::default(), &opts, sphere), "sphere intersector in a triangle-only pipeline");
    let mut sphere_opts = opts.clone();
    sphere_opts.primitive_types = primitive_flag(Primitive::Sphere);
    invalid(Module::builtin_intersection(&ctx, &ModuleOptions::default(), &sphere_opts, BuiltinIs { endcaps: true, ..sphere }), "end caps on spheres");
    Module::builtin_intersection(&ctx, &ModuleOptions::default(), &sphere_opts, sphere).expect("sphere intersector");
}

// ---------------------------------------------------------------------------------------------
// Pipeline, stack sizes, symbols
// ---------------------------------------------------------------------------------------------

#[test]
#[ignore = "GPU"]
fn stack_sizes_follow_the_header_formulas() {
    let mut rig = rig(Cfg { single_gas: true, ..Default::default() });
    let gas = default_gas(&mut rig, 0.0, BuildOptions::default());
    let [rg, ms, hg] = &rig.groups;
    let p = Some(&rig.pipeline);
    let (s_rg, s_ms, s_hg) = (rg.stack_sizes(p).expect("rg"), ms.stack_sizes(p).expect("ms"), hg.stack_sizes(p).expect("hg"));
    println!("raygen {s_rg:?}\nmiss {s_ms:?}\nhit {s_hg:?}");
    assert_eq!(rg.stack_sizes(None).expect("rg without pipeline"), s_rg, "the fixture calls no external functions");
    let mut acc = StackSizes::default();
    for g in &rig.groups {
        acc.accumulate(g, p).expect("accumulate");
    }
    assert_eq!(acc, s_rg.max(s_ms).max(s_hg));

    // optix_stack_size.h, evaluated by hand for distinct values.
    let s = StackSizes { css_rg: 100, css_ms: 20, css_ch: 30, css_ah: 7, css_is: 50, css_cc: 11, dss_dc: 13 };
    // ccTree = 2 * 11 = 22; chOrMs = 30 + 22 = 52; 100 + 22 + (3 - 1) * 52 + max(52, 50 + 7).
    assert_eq!(s.compute(3, 2, 4).expect("compute"), PipelineStack { direct_from_traversal: 52, direct_from_state: 52, continuation: 283 });
    assert_eq!(s.compute(0, 0, 0).expect("compute"), PipelineStack { direct_from_traversal: 0, direct_from_state: 0, continuation: 100 });
    // ccTree = 11; chOrMs = 41; 100 + 11 + 41 + max(41, 57).
    assert_eq!(s.compute_dc_split(5, 6, 2, 1, 3, 4).expect("dc split"), PipelineStack { direct_from_traversal: 15, direct_from_state: 24, continuation: 209 });
    // chOrMs = 30 + 40 = 70; 100 + 40 + 0 + max(70, 57).
    assert_eq!(s.compute_css_cc_tree(40, 1, 2).expect("cc tree"), PipelineStack { direct_from_traversal: 26, direct_from_state: 26, continuation: 210 });
    invalid(StackSizes { css_rg: u32::MAX, css_ch: 1, ..Default::default() }.compute(1, 0, 0), "continuation stack overflow");
    invalid(s.compute(1, 0, u32::MAX), "direct stack overflow");

    let spt = simple_path_tracer(rg, ms, std::slice::from_ref(hg), ms, std::slice::from_ref(hg), p).expect("simple path tracer");
    let want = s_rg.css_rg + s_ms.css_ms.max(s_hg.css_ch + s_ms.css_ms.max(s_hg.css_ch));
    assert_eq!(spt, PipelineStack { direct_from_traversal: 0, direct_from_state: 0, continuation: want });

    let stack = acc.compute(1, 0, 0).expect("compute");
    rig.pipeline.set_stack(stack, 1).expect("set_stack");
    assert_eq!(rig.trace(gas.handle()), mask(0.0), "launch after set_stack");
    rig.pipeline.set_stack_from_depths(LinkOptions { max_trace_depth: 1, max_graph_depth: 1, ..Default::default() }).expect("set_stack_from_depths");
    assert_eq!(rig.trace(gas.handle()), mask(0.0), "launch after set_stack_from_depths");
}

#[test]
#[ignore = "GPU"]
fn symbol_copies() {
    let rig = rig(Cfg::default());
    // The fixture's only global is the launch-parameter block `params` (`.visible .const`);
    // OptiX owns it, so every copy direction is refused, and no other symbol exists.
    let invalid_op = sys::OptixResult_OPTIX_ERROR_INVALID_OPERATION;
    optix_err(rig.pipeline.read_symbol::<u64>(&rig.stream, "params", 0), invalid_op, "read the launch parameters");
    optix_err(rig.pipeline.write_symbol(&rig.stream, "params", 16, &W), invalid_op, "write the launch parameters");
    let src = up(&rig.stream, &W.to_le_bytes());
    // SAFETY: `src` outlives the synchronization below.
    optix_err(unsafe { rig.pipeline.write_symbol_from(&rig.stream, "params", 16, &src) }, invalid_op, "write the launch parameters from the device");
    let mut dst = DeviceBuffer::<u8>::zeroed(&rig.stream, 4).expect("dst");
    // SAFETY: `dst` is not read before the synchronization below.
    optix_err(unsafe { rig.pipeline.read_symbol_into(&rig.stream, "params", 16, &mut dst) }, invalid_op, "read the launch parameters to the device");
    rig.stream.synchronize().expect("sync");
    optix_err(rig.pipeline.write_symbol(&rig.stream, "no_such_symbol", 0, &1u32), sys::OptixResult_OPTIX_ERROR_SYMBOL_NOT_FOUND, "write unknown symbol");
    optix_err(rig.pipeline.read_symbol::<u32>(&rig.stream, "no_such_symbol", 0), sys::OptixResult_OPTIX_ERROR_SYMBOL_NOT_FOUND, "read unknown symbol");
    let empty = DeviceBuffer::<u8>::zeroed(&rig.stream, 0).expect("empty");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { rig.pipeline.write_symbol_from(&rig.stream, "params", 0, &empty) }, "zero-byte symbol copy");
    invalid(rig.pipeline.write_symbol(&rig.stream, "par\0ams", 0, &1u32), "symbol name with NUL");
}

// ---------------------------------------------------------------------------------------------
// Geometry inputs
// ---------------------------------------------------------------------------------------------

#[test]
#[ignore = "GPU"]
fn every_vertex_format_traces() {
    let mut rig = rig(Cfg { single_gas: true, ..Default::default() });
    let s = rig.stream.clone();
    let t = tri_verts(0.0, LEG);
    // Snorm16 holds [-1, 1]: half-size triangle (no pixel centre within 0.015 of its edges).
    let small = 0.515;
    let ts = tri_verts(0.0, small);
    let snorm = |x: f32| (x * 32767.0).round() as i16;
    let f3 = up(&s, &t);
    let f2 = up(&s, &t.map(|v| [v[0], v[1]]));
    let h3 = up(&s, &t.map(|v| v.map(f16)));
    let h2 = up(&s, &t.map(|v| [f16(v[0]), f16(v[1])]));
    let s3 = up(&s, &ts.map(|v| v.map(snorm)));
    let s2 = up(&s, &ts.map(|v| [snorm(v[0]), snorm(v[1])]));
    let cases: [(Vertices<'_>, VertexFormat, Vec<u32>); 6] = [
        (Vertices::Float3(Keys::One(&f3)), VertexFormat::Float3, mask(0.0)),
        (Vertices::Float2(Keys::One(&f2)), VertexFormat::Float2, mask(0.0)),
        (Vertices::Half3(Keys::One(&h3)), VertexFormat::Half3, mask(0.0)),
        (Vertices::Half2(Keys::One(&h2)), VertexFormat::Half2, mask(0.0)),
        (Vertices::Snorm16x3(Keys::One(&s3)), VertexFormat::Snorm16x3, grid(tri(0.0, small))),
        (Vertices::Snorm16x2(Keys::One(&s2)), VertexFormat::Snorm16x2, grid(tri(0.0, small))),
    ];
    for (vertices, format, want) in cases {
        assert_eq!(vertices.format(), format);
        let mesh = TriangleMesh { vertices, flags: &[GeometryFlags::DISABLE_ANYHIT], ..TriangleMesh::new(&f3) };
        let gas = tri_gas(&mut rig, &[mesh], BuildOptions::default());
        assert_eq!(rig.trace(gas.handle()), want, "{format:?}");
    }
}

#[test]
#[ignore = "GPU"]
fn every_index_format_traces() {
    let mut rig = rig(Cfg { single_gas: true, ..Default::default() });
    let s = rig.stream.clone();
    let [a, b, c] = tri_verts(0.0, LEG);
    // Vertex 0 is far outside the launch and unused.
    let verts = up(&s, &[[5.0, 5.0, 0.0], a, b, c]);
    let i8s = up(&s, &[[1u8, 2, 3]]);
    let i16s = up(&s, &[[1u16, 2, 3]]);
    let i32s = up(&s, &[[1u32, 2, 3]]);
    for (indices, name) in [(Indices::U16(&i16s), "U16"), (Indices::U32(&i32s), "U32"), (Indices::U8(&i8s), "U8")] {
        let mesh = TriangleMesh { indices: Some(indices), ..opaque(&verts) };
        let r = Accel::triangles(&rig.ctx, &rig.stream, &mut rig.scratch, &[mesh], BuildOptions::default());
        match r {
            Ok(gas) => assert_eq!(rig.trace(gas.handle()), mask(0.0), "{name} indices"),
            Err(e) => panic!("{name} indices: {e}"),
        }
    }
}

#[test]
#[ignore = "GPU"]
fn per_primitive_sbt_offsets() {
    let mut rig = rig(Cfg { single_gas: true, records: 2, ..Default::default() });
    let s = rig.stream.clone();
    let leg = 0.53;
    let [a, b, c] = tri_verts(0.0, leg);
    let [d, e, f] = tri_verts(0.5, leg);
    let verts = up(&s, &[a, b, c, d, e, f]);
    let want = union(&grid(tri(0.0, leg)), &grid(tri(0.5, leg)));
    let two = [GeometryFlags::DISABLE_ANYHIT, GeometryFlags::DISABLE_ANYHIT];
    let o8 = up(&s, &[0u8, 1]);
    let o16 = up(&s, &[0u16, 1]);
    let o32 = up(&s, &[1u32, 0]);
    for offsets in [SbtOffsets::U8(&o8), SbtOffsets::U16(&o16), SbtOffsets::U32(&o32)] {
        let mesh = TriangleMesh { flags: &two, sbt_offsets: Some(offsets), ..TriangleMesh::new(&verts) };
        let gas = tri_gas(&mut rig, &[mesh], BuildOptions::default());
        assert_eq!(rig.trace(gas.handle()), want);
    }
    invalid(try_tri(&mut rig, TriangleMesh { flags: &two, ..TriangleMesh::new(&verts) }), "two records without offsets");
    let short = up(&s, &[0u8]);
    invalid(try_tri(&mut rig, TriangleMesh { flags: &two, sbt_offsets: Some(SbtOffsets::U8(&short)), ..TriangleMesh::new(&verts) }), "offsets shorter than the primitives");
    invalid(try_tri(&mut rig, TriangleMesh { flags: &[], ..TriangleMesh::new(&verts) }), "no SBT record");
}

#[test]
#[ignore = "GPU"]
fn pre_transform_moves_the_mesh() {
    let mut rig = rig(Cfg { single_gas: true, ..Default::default() });
    let s = rig.stream.clone();
    let verts = up(&s, &tri_verts(0.0, LEG));
    let xf = up(&s, &[shift(DX)]);
    let gas = tri_gas(&mut rig, &[TriangleMesh { pre_transform: Some(&xf), ..opaque(&verts) }], BuildOptions::default());
    assert_eq!(rig.trace(gas.handle()), mask(DX));
    let two = up(&s, &[shift(DX), shift(DX)]);
    invalid(Accel::triangles(&rig.ctx, &rig.stream, &mut rig.scratch, &[TriangleMesh { pre_transform: Some(&two), ..opaque(&verts) }], BuildOptions::default()), "two pre-transforms");
    invalid(Accel::triangles(&rig.ctx, &rig.stream, &mut rig.scratch, &[TriangleMesh::new(&up(&s, &[[0.0f32; 3]; 4]))], BuildOptions::default()), "4 non-indexed vertices");
}

const CURVES: [(CurveType, Primitive); 10] = [
    (CurveType::RoundLinear, Primitive::RoundLinear),
    (CurveType::RoundQuadraticBspline, Primitive::RoundQuadraticBspline),
    (CurveType::RoundCubicBspline, Primitive::RoundCubicBspline),
    (CurveType::RoundCatmullRom, Primitive::RoundCatmullRom),
    (CurveType::RoundCubicBezier, Primitive::RoundCubicBezier),
    (CurveType::RoundQuadraticBsplineRocaps, Primitive::RoundQuadraticBsplineRocaps),
    (CurveType::RoundCubicBsplineRocaps, Primitive::RoundCubicBsplineRocaps),
    (CurveType::RoundCatmullRomRocaps, Primitive::RoundCatmullRomRocaps),
    (CurveType::RoundCubicBezierRocaps, Primitive::RoundCubicBezierRocaps),
    (CurveType::FlatQuadraticBspline, Primitive::FlatQuadraticBspline),
];

/// Radius of the test curves: rows 7 and 8 (0.031 off the axis) are inside, 6 and 9 (0.094) out.
const CURVE_R: f32 = 0.07;

/// Seven collinear control points along x in [-1, 2] at height `y`: every basis then traces the
/// straight line, and its segments cover [-0.5, 1.5] at least, beyond the launch on both sides.
fn curve_points(y: f32) -> Vec<[f32; 3]> {
    (0..7).map(|i| [-1.0 + 0.5 * i as f32, y, 0.0]).collect()
}

/// First control point of every segment of `kind` over seven points.
fn curve_segments(kind: CurveType) -> Vec<u32> {
    match kind {
        CurveType::RoundCubicBezier | CurveType::RoundCubicBezierRocaps => vec![0, 3],
        CurveType::RoundLinear => (0..6).collect(),
        CurveType::RoundQuadraticBspline | CurveType::RoundQuadraticBsplineRocaps | CurveType::FlatQuadraticBspline => (0..5).collect(),
        CurveType::RoundCubicBspline | CurveType::RoundCatmullRom | CurveType::RoundCubicBsplineRocaps | CurveType::RoundCatmullRomRocaps => (0..4).collect(),
    }
}

fn band(y0: f32) -> Vec<u32> {
    grid(move |_, y| CURVE_R - (y - y0).abs())
}

fn curves<'a>(kind: CurveType, verts: &'a DeviceBuffer<[f32; 3]>, widths: &'a DeviceBuffer<f32>, segs: &'a DeviceBuffer<u32>) -> Curves<'a> {
    Curves {
        kind,
        vertices: Keys::One(verts),
        widths: Keys::One(widths),
        normals: None,
        segments: segs,
        flags: GeometryFlags::DISABLE_ANYHIT,
        endcaps: false,
        primitive_offset: 0,
    }
}

#[test]
#[ignore = "GPU"]
fn every_curve_type_traces_and_refits() {
    for (kind, primitive) in CURVES {
        let builtin = BuiltinIs { primitive, motion_blur: false, build_flags: build_flags(true), endcaps: false };
        let mut rig = rig(Cfg { single_gas: true, builtin: Some(builtin), ..Default::default() });
        assert_eq!(rig.opts.primitive_types, kind.flag());
        let s = rig.stream.clone();
        let verts = up(&s, &curve_points(0.5));
        let widths = up(&s, &[CURVE_R; 7]);
        let segs = up(&s, &curve_segments(kind));
        let c = curves(kind, &verts, &widths, &segs);
        let gas = Accel::curves(&rig.ctx, &rig.stream, &mut rig.scratch, &[c], updatable()).unwrap_or_else(|e| panic!("{kind:?}: {e}"));
        assert_eq!(rig.trace(gas.handle()), band(0.5), "{kind:?}");
        // Move the curve up by four pixel rows and refit.
        let moved = up(&s, &curve_points(0.75));
        // SAFETY: no launch is in flight; the next launch is ordered after the refit.
        unsafe { gas.refit_curves(&rig.stream, &mut rig.scratch, &[curves(kind, &moved, &widths, &segs)]) }.unwrap_or_else(|e| panic!("{kind:?} refit: {e}"));
        assert_eq!(rig.trace(gas.handle()), band(0.75), "{kind:?} refitted");
    }
}

#[test]
#[ignore = "GPU"]
fn curve_inputs_are_checked() {
    let mut rig = rig(Cfg::default());
    let s = rig.stream.clone();
    let verts = up(&s, &curve_points(0.5));
    let widths = up(&s, &[CURVE_R; 7]);
    let segs = up(&s, &curve_segments(CurveType::RoundLinear));
    let short_w = up(&s, &[CURVE_R; 6]);
    let normals = up(&s, &[[0.0f32, 0.0, 1.0]; 7]);
    let no_segs = up(&s, &[0u32; 0]);
    let base = curves(CurveType::RoundLinear, &verts, &widths, &segs);
    invalid(try_curves(&mut rig, Curves { widths: Keys::One(&short_w), ..base }), "one width short");
    invalid(try_curves(&mut rig, Curves { normals: Some(Keys::One(&normals)), ..base }), "normals on round curves");
    invalid(try_curves(&mut rig, Curves { endcaps: true, ..base }), "end caps on linear curves");
    invalid(try_curves(&mut rig, Curves { segments: &no_segs, ..base }), "no segments");
}

#[test]
#[ignore = "GPU"]
fn spheres_trace_and_refit() {
    let builtin = BuiltinIs { primitive: Primitive::Sphere, motion_blur: false, build_flags: build_flags(true), endcaps: false };
    let mut rig = rig(Cfg { single_gas: true, builtin: Some(builtin), ..Default::default() });
    let s = rig.stream.clone();
    let disc = |cx: f32, cy: f32, r: f32| move |x: f32, y: f32| r - ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();

    let one = up(&s, &[[0.5f32, 0.5, 0.0]]);
    let r = up(&s, &[0.29f32]);
    let single = Spheres::new(&one, Radii::Single(Keys::One(&r)));
    let gas = Accel::spheres(&rig.ctx, &rig.stream, &mut rig.scratch, &[single], updatable()).expect("single-radius spheres");
    assert_eq!(rig.trace(gas.handle()), grid(disc(0.5, 0.5, 0.29)));
    let moved = up(&s, &[[0.25f32, 0.5, 0.0]]);
    // SAFETY: no launch is in flight; the next launch is ordered after the refit.
    unsafe { gas.refit_spheres(&rig.stream, &mut rig.scratch, &[Spheres::new(&moved, Radii::Single(Keys::One(&r)))]) }.expect("refit spheres");
    assert_eq!(rig.trace(gas.handle()), grid(disc(0.25, 0.5, 0.29)));

    let two = up(&s, &[[0.25f32, 0.25, 0.0], [0.75, 0.75, 0.0]]);
    let radii = up(&s, &[0.15f32, 0.2]);
    let gas = Accel::spheres(&rig.ctx, &rig.stream, &mut rig.scratch, &[Spheres::new(&two, Radii::PerSphere(Keys::One(&radii)))], BuildOptions::default()).expect("per-sphere radii");
    assert_eq!(rig.trace(gas.handle()), union(&grid(disc(0.25, 0.25, 0.15)), &grid(disc(0.75, 0.75, 0.2))));

    invalid(Accel::spheres(&rig.ctx, &rig.stream, &mut rig.scratch, &[Spheres::new(&two, Radii::Single(Keys::One(&radii)))], BuildOptions::default()), "single radius buffer of two");
    invalid(Accel::spheres(&rig.ctx, &rig.stream, &mut rig.scratch, &[Spheres::new(&two, Radii::PerSphere(Keys::One(&r)))], BuildOptions::default()), "one radius for two spheres");
}

fn emit_aabb(rig: &Rig, gas: &Accel) -> Aabb {
    let mut out = DeviceBuffer::<Aabb>::zeroed(&rig.stream, 1).expect("aabb out");
    // SAFETY: `gas` and `out` outlive the synchronization below.
    unsafe { gas.emit(&rig.stream, Emit::Aabbs(&mut out)) }.expect("emit AABBs");
    rig.stream.synchronize().expect("sync");
    out.to_host_vec(&rig.stream).expect("read back")[0]
}

fn assert_box(got: Aabb, want: Aabb) {
    let close = got.min.iter().zip(want.min).chain(got.max.iter().zip(want.max)).all(|(a, b)| (a - b).abs() <= 1e-4);
    assert!(close, "emitted {got:?}, expected {want:?}");
}

#[test]
#[ignore = "GPU"]
fn custom_primitives_build_emit_and_refit() {
    let mut rig = rig(Cfg { single_gas: true, records: 2, ..Default::default() });
    let s = rig.stream.clone();
    let boxes = [Aabb { min: [0.1, 0.2, -0.1], max: [0.4, 0.5, 0.1] }, Aabb { min: [0.3, 0.1, -0.2], max: [0.6, 0.7, 0.05] }];
    let union_box = Aabb { min: [0.1, 0.1, -0.2], max: [0.6, 0.7, 0.1] };
    let buf = up(&s, &boxes);
    let gas = Accel::custom(&rig.ctx, &rig.stream, &mut rig.scratch, &[CustomPrimitives::new(&buf)], updatable()).expect("custom GAS");
    assert_box(emit_aabb(&rig, &gas), union_box);

    let mut size = DeviceBuffer::<u64>::zeroed(&rig.stream, 1).expect("size out");
    // SAFETY: `gas` and `size` outlive the synchronization below.
    unsafe { gas.emit(&rig.stream, Emit::CompactedSize(&mut size)) }.expect("emit compacted size");
    rig.stream.synchronize().expect("sync");
    let compacted = size.to_host_vec(&rig.stream).expect("read back")[0];
    println!("custom GAS: {} bytes, emitted compacted size {compacted}", gas.size_bytes());
    assert!(compacted > 0 && compacted as usize <= gas.size_bytes());

    let moved = up(&s, &boxes.map(|b| Aabb { min: b.min.map(|v| v + 1.0), max: b.max.map(|v| v + 1.0) }));
    // SAFETY: nothing traces the structure.
    unsafe { gas.refit_custom(&rig.stream, &mut rig.scratch, &[CustomPrimitives::new(&moved)]) }.expect("refit custom");
    assert_box(emit_aabb(&rig, &gas), Aabb { min: union_box.min.map(|v| v + 1.0), max: union_box.max.map(|v| v + 1.0) });

    // Two SBT records with per-primitive offsets build; missing offsets do not.
    let two = [GeometryFlags::NONE, GeometryFlags::NONE];
    let offsets = up(&s, &[0u16, 1]);
    let gas2 = Accel::custom(&rig.ctx, &rig.stream, &mut rig.scratch, &[CustomPrimitives { flags: &two, sbt_offsets: Some(SbtOffsets::U16(&offsets)), ..CustomPrimitives::new(&buf) }], BuildOptions::default())
        .expect("custom with SBT offsets");
    assert_box(emit_aabb(&rig, &gas2), union_box);
    invalid(Accel::custom(&rig.ctx, &rig.stream, &mut rig.scratch, &[CustomPrimitives { flags: &two, ..CustomPrimitives::new(&buf) }], BuildOptions::default()), "two records without offsets");
    let none = up::<Aabb>(&s, &[]);
    invalid(Accel::custom(&rig.ctx, &rig.stream, &mut rig.scratch, &[CustomPrimitives::new(&none)], BuildOptions::default()), "no boxes");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { gas.refit_custom(&rig.stream, &mut rig.scratch, &[CustomPrimitives::new(&up(&s, &boxes[..1]))]) }, "refit with another count");
}

#[test]
#[ignore = "GPU"]
fn emit_checks_its_output() {
    let mut rig = rig(Cfg { single_gas: true, ..Default::default() });
    let plain = default_gas(&mut rig, 0.0, BuildOptions { compact: false, ..Default::default() });
    let compacted = default_gas(&mut rig, 0.0, BuildOptions::default());
    let mut size = DeviceBuffer::<u64>::zeroed(&rig.stream, 1).expect("size");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { plain.emit(&rig.stream, Emit::CompactedSize(&mut size)) }, "compacted size of an uncompactable structure");
    let mut two = DeviceBuffer::<Aabb>::zeroed(&rig.stream, 2).expect("boxes");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { compacted.emit(&rig.stream, Emit::Aabbs(&mut two)) }, "two boxes for a static structure");
    assert_box(emit_aabb(&rig, &compacted), Aabb { min: [0.0, 0.0, 0.0], max: [LEG, LEG, 0.0] });
    // SAFETY: `compacted` and `size` outlive the synchronization below.
    unsafe { compacted.emit(&rig.stream, Emit::CompactedSize(&mut size)) }.expect("emit");
    rig.stream.synchronize().expect("sync");
    let n = size.to_host_vec(&rig.stream).expect("read back")[0];
    assert_eq!(n as usize, compacted.size_bytes(), "a compacted structure reports its own size");
}

#[test]
#[ignore = "GPU"]
fn instance_pointers_select_instances_and_refit() {
    let mut rig = rig(Cfg::default());
    let gas = default_gas(&mut rig, 0.0, BuildOptions::default());
    let set = DeviceInstances::new(&rig.ctx, &rig.stream, &[translated(&gas, 0.0), translated(&gas, DX), translated(&gas, 0.5)]).expect("device instances");
    assert_eq!(set.len(), 3);
    assert!(!set.is_empty());
    let ias = Accel::instance_pointers(&rig.ctx, &rig.stream, &mut rig.scratch, &[(&set, 0), (&set, 2)], updatable()).expect("IAS over pointers");
    assert_eq!(rig.trace(ias.handle()), union(&mask(0.0), &mask(0.5)));
    // SAFETY: no launch is in flight; the next launch is ordered after the refit.
    unsafe { ias.refit_instance_pointers(&rig.stream, &mut rig.scratch, &[(&set, 1), (&set, 2)]) }.expect("refit pointers");
    assert_eq!(rig.trace(ias.handle()), union(&mask(DX), &mask(0.5)));

    invalid(Accel::instance_pointers(&rig.ctx, &rig.stream, &mut rig.scratch, &[(&set, 3)], BuildOptions::default()), "index past the set");
    invalid(Accel::instance_pointers(&rig.ctx, &rig.stream, &mut rig.scratch, &[], BuildOptions::default()), "no instances");
    let other = rig.ctx.cuda().new_stream().expect("stream");
    let mut other_scratch = Scratch::new(&other);
    invalid(Accel::instance_pointers(&rig.ctx, &other, &mut other_scratch, &[(&set, 0)], BuildOptions::default()), "set uploaded on another stream");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { ias.refit_instance_pointers(&rig.stream, &mut rig.scratch, &[(&set, 0)]) }, "refit with another count");
    invalid(DeviceInstances::new(&rig.ctx, &rig.stream, &[]), "empty set");
}

#[test]
#[ignore = "GPU"]
fn relocated_structures_trace_after_the_sources_are_gone() {
    let mut rig = rig(Cfg::default());
    let gas = default_gas(&mut rig, 0.0, BuildOptions::default());
    assert!(gas.relocation_info().expect("info").compatible(&rig.ctx).expect("compatible"), "a device is compatible with itself");
    // SAFETY: the build is ordered before the copy on the same stream; no refit runs.
    let gas2 = unsafe { gas.relocate(&rig.ctx, &rig.stream, Relink::Keep, None) }.expect("relocate GAS");
    assert_eq!(gas2.size_bytes(), gas.size_bytes());
    let ias = ias(&mut rig, &[translated(&gas, DX)], BuildOptions::default());
    // SAFETY: as above; the relinked child is a GAS of the same context.
    let ias2 = unsafe { ias.relocate(&rig.ctx, &rig.stream, Relink::Children(&[optix::Child::Accel(gas2.clone())]), None) }.expect("relocate IAS");
    drop(ias);
    drop(gas);
    rig.stream.synchronize().expect("sync");
    assert_eq!(rig.trace(gas2.handle()), mask(0.0), "relocated GAS");
    assert_eq!(rig.trace(ias2.handle()), mask(DX), "relocated IAS over the relocated GAS");

    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { ias2.relocate(&rig.ctx, &rig.stream, Relink::Children(&[]), None) }, "children count mismatch");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { gas2.relocate(&rig.ctx, &rig.stream, Relink::Children(&[optix::Child::Accel(gas2.clone())]), None) }, "children for a GAS");
    // SAFETY: rejected before allocating.
    let r = unsafe { gas2.relocate(&rig.ctx, &rig.stream, Relink::Keep, Some(16)) };
    assert!(matches!(r, Err(Error::Budget { available: 16, .. })), "budget");
}

#[test]
#[ignore = "GPU"]
fn motion_gas_traces_key_zero_emits_and_refits() {
    let mut rig = rig(Cfg { single_gas: true, motion: true, ..Default::default() });
    let s = rig.stream.clone();
    let k0 = up(&s, &tri_verts(0.0, LEG));
    let k1 = up(&s, &tri_verts(DX, LEG));
    let keys = [&k0, &k1];
    let mesh = TriangleMesh { vertices: Vertices::Float3(Keys::Motion(&keys)), flags: &[GeometryFlags::DISABLE_ANYHIT], ..TriangleMesh::new(&k0) };
    let opts = BuildOptions { motion_keys: 2, allow_update: true, ..Default::default() };
    let gas = tri_gas(&mut rig, &[mesh], opts);
    assert_eq!(rig.trace(gas.handle()), mask(0.0), "time 0 sees key 0");
    let mut boxes = DeviceBuffer::<Aabb>::zeroed(&s, 2).expect("boxes");
    // SAFETY: `gas` and `boxes` outlive the synchronization below.
    unsafe { gas.emit(&s, Emit::Aabbs(&mut boxes)) }.expect("emit per-key boxes");
    s.synchronize().expect("sync");
    let b = boxes.to_host_vec(&s).expect("read back");
    assert!(b[1].min[0] > b[0].min[0], "key 1 is shifted along x: {b:?}");

    // Refit: key 0 moves to DX.
    let k2 = up(&s, &tri_verts(2.0 * DX, LEG));
    let moved = [&k1, &k2];
    let mesh = TriangleMesh { vertices: Vertices::Float3(Keys::Motion(&moved)), flags: &[GeometryFlags::DISABLE_ANYHIT], ..TriangleMesh::new(&k0) };
    // SAFETY: no launch is in flight; the next launch is ordered after the refit.
    unsafe { gas.refit_triangles(&s, &mut rig.scratch, &[mesh]) }.expect("refit motion GAS");
    assert_eq!(rig.trace(gas.handle()), mask(DX), "time 0 sees the refitted key 0");

    let late = BuildOptions { motion_keys: 2, motion: MotionOptions { begin: 0.5, end: 1.0, start_vanish: false, end_vanish: false }, ..Default::default() };
    let gas = tri_gas(&mut rig, &[TriangleMesh { vertices: Vertices::Float3(Keys::Motion(&keys)), flags: &[GeometryFlags::DISABLE_ANYHIT], ..TriangleMesh::new(&k0) }], late);
    assert_eq!(rig.trace(gas.handle()), mask(0.0), "without vanish a ray before begin sees the first key");

    let one = [&k0];
    invalid(Accel::triangles(&rig.ctx, &s, &mut rig.scratch, &[TriangleMesh { vertices: Vertices::Float3(Keys::Motion(&one)), ..TriangleMesh::new(&k0) }], opts), "one buffer for two keys");
    invalid(Accel::triangles(&rig.ctx, &s, &mut rig.scratch, &[opaque(&k0)], BuildOptions { motion_keys: 0, ..Default::default() }), "zero motion keys");
    invalid(Accel::triangles(&rig.ctx, &s, &mut rig.scratch, &[opaque(&k0)], BuildOptions { motion_keys: 2, motion: MotionOptions { begin: 1.0, end: 1.0, ..Default::default() }, ..Default::default() }), "empty time range");
}

/// A two-key GAS over [0.5, 1] that vanishes before its first key, traced at time 0 through an
/// IAS. Traced as the top-level handle instead, the launch faults with an illegal address on the
/// OptiX 9.1 driver (single-GAS and any-graph pipelines alike), which would poison the CUDA
/// context of every later test; `MotionOptions` documents it.
#[test]
#[ignore = "GPU"]
fn motion_gas_start_vanish_misses_through_ias() {
    let mut rig = rig(Cfg { motion: true, ..Default::default() });
    let s = rig.stream.clone();
    let k0 = up(&s, &tri_verts(0.0, LEG));
    let k1 = up(&s, &tri_verts(DX, LEG));
    let keys = [&k0, &k1];
    let mesh = || TriangleMesh { vertices: Vertices::Float3(Keys::Motion(&keys)), flags: &[GeometryFlags::DISABLE_ANYHIT], ..TriangleMesh::new(&k0) };
    let late = BuildOptions { motion_keys: 2, motion: MotionOptions { begin: 0.5, end: 1.0, start_vanish: true, end_vanish: false }, ..Default::default() };
    let gas = tri_gas(&mut rig, &[mesh()], late);
    let top = ias(&mut rig, &[Instance::new(&gas)], BuildOptions::default());
    assert_eq!(rig.trace(top.handle()), misses(), "a ray before begin misses a start-vanishing GAS");
    let clamped = tri_gas(&mut rig, &[mesh()], BuildOptions { motion: MotionOptions { start_vanish: false, ..late.motion }, ..late });
    let top = ias(&mut rig, &[Instance::new(&clamped)], BuildOptions::default());
    assert_eq!(rig.trace(top.handle()), mask(0.0), "the same structure without vanish shows key 0");
}

// ---------------------------------------------------------------------------------------------
// Transforms
// ---------------------------------------------------------------------------------------------

#[test]
#[ignore = "GPU"]
fn transforms_under_an_instance_structure() {
    let link = LinkOptions { max_graph_depth: 3, ..Default::default() };
    let mut rig = rig(Cfg { motion: true, link, ..Default::default() });
    let gas = default_gas(&mut rig, 0.0, BuildOptions::default());
    let s = rig.stream.clone();

    let fixed = Transform::fixed(&rig.ctx, &s, &gas, shift(DX)).expect("static transform");
    assert_eq!(fixed.kind(), optix::TransformKind::Static);
    let top = ias(&mut rig, &[Instance::new(&fixed)], BuildOptions::default());
    drop(fixed); // the IAS holds the transform, the transform the GAS
    assert_eq!(rig.trace(top.handle()), mask(DX), "static transform");

    let span = MotionOptions::default();
    let mm = Transform::matrix_motion(&rig.ctx, &s, &gas, span, &[shift(0.0), shift(DX)]).expect("matrix motion");
    assert_eq!(mm.kind(), optix::TransformKind::MatrixMotion);
    let top = ias(&mut rig, &[Instance::new(&mm)], BuildOptions::default());
    assert_eq!(rig.trace(top.handle()), mask(0.0), "matrix motion at time 0 is key 0");

    let srt = |tx: f32| Srt { tx, ..Srt::IDENTITY };
    let sm = Transform::srt_motion(&rig.ctx, &s, &gas, span, &[srt(DX), srt(2.0 * DX)]).expect("SRT motion");
    assert_eq!(sm.kind(), optix::TransformKind::SrtMotion);
    let top = ias(&mut rig, &[Instance::new(&sm)], BuildOptions::default());
    assert_eq!(rig.trace(top.handle()), mask(DX), "SRT motion at time 0 is key 0");

    let late = MotionOptions { begin: 0.5, end: 1.0, start_vanish: true, end_vanish: false };
    let vm = Transform::matrix_motion(&rig.ctx, &s, &gas, late, &[shift(0.0), shift(DX)]).expect("vanishing matrix motion");
    let vs = Transform::srt_motion(&rig.ctx, &s, &gas, late, &[srt(0.0), srt(DX)]).expect("vanishing SRT motion");
    for t in [vm, vs] {
        let top = ias(&mut rig, &[Instance::new(&t)], BuildOptions::default());
        assert_eq!(rig.trace(top.handle()), misses(), "{:?} before begin with start_vanish", t.kind());
    }

    // A chain: static transform over a matrix motion transform (graph depth 4 = IAS + 2 + GAS).
    rig.pipeline.set_stack_from_depths(LinkOptions { max_graph_depth: 4, ..Default::default() }).expect("depth 4");
    let inner = Transform::matrix_motion(&rig.ctx, &s, &gas, span, &[shift(0.0), shift(DX)]).expect("inner");
    let outer = Transform::fixed(&rig.ctx, &s, &inner, shift(DX)).expect("outer");
    assert!(matches!(outer.child(), optix::Child::Transform(_)));
    let top = ias(&mut rig, &[Instance::new(&outer)], BuildOptions::default());
    assert_eq!(rig.trace(top.handle()), mask(DX), "static over matrix motion");

    invalid(Transform::matrix_motion(&rig.ctx, &s, &gas, span, &[shift(0.0)]), "one key");
    invalid(Transform::srt_motion(&rig.ctx, &s, &gas, MotionOptions { begin: 1.0, end: 0.0, ..span }, &[srt(0.0), srt(DX)]), "reversed time range");
    let mut singular = Instance::IDENTITY;
    singular[0] = 0.0;
    invalid(Transform::fixed(&rig.ctx, &s, &gas, singular), "singular matrix");
    let other = DeviceContext::new(rig.ctx.cuda(), None, 0, false).expect("second context");
    invalid(Transform::fixed(&other, &s, &gas, shift(0.0)), "child from another context");
}

// ---------------------------------------------------------------------------------------------
// Opacity micromaps
// ---------------------------------------------------------------------------------------------

const OMM_LEVEL: u32 = 3;

fn transparent(i: u32) -> bool {
    i.is_multiple_of(3)
}

/// 4-state data of one level-3 micromap: state 0 (transparent) or 1 (opaque), 2 bits each.
fn omm_bits() -> Vec<u8> {
    let mut bytes = vec![0u8; 16];
    for i in 0..64u32 {
        let state = u8::from(!transparent(i));
        bytes[(i / 4) as usize] |= state << ((i % 4) * 2);
    }
    bytes
}

/// CPU reference: the default triangle's mask with micro-triangles of `transparent` removed;
/// each covered pixel's micro-triangle is found with the barycentric helpers.
fn omm_mask() -> Vec<u32> {
    let base = mask(0.0);
    let micros: Vec<[[f32; 2]; 3]> = (0..1u32 << (2 * OMM_LEVEL)).map(|i| micro_to_base_bary(i, OMM_LEVEL).expect("micro bary")).collect();
    (0..H)
        .flat_map(|j| (0..W).map(move |i| ((i as f32 + 0.5) / W as f32, (j as f32 + 0.5) / H as f32)))
        .zip(base)
        .map(|((x, y), inside)| {
            if inside == 0 {
                return 0;
            }
            let bary = [x / LEG, y / LEG];
            let hits: Vec<(u32, f32)> = micros
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let [a, b] = base_to_micro_bary(bary, *m);
                    (i as u32, a.min(b).min(1.0 - a - b))
                })
                .filter(|(_, d)| *d > -1e-3)
                .collect();
            assert!(matches!(hits.as_slice(), [(_, d)] if *d > 1e-3), "pixel ({x}, {y}) is not strictly inside one micro-triangle: {hits:?}");
            u32::from(!transparent(hits[0].0))
        })
        .collect()
}

#[test]
#[ignore = "GPU"]
fn barycentric_helpers_match_hand_values() {
    assert_eq!(micro_to_base_bary(0, 0).expect("level 0"), [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]);
    // Level 1, evaluated by hand from optix_micromap_impl.h.
    let want = [
        [[0.0, 0.0], [0.5, 0.0], [0.0, 0.5]],
        [[0.0, 0.5], [0.5, 0.5], [0.5, 0.0]],
        [[0.5, 0.0], [1.0, 0.0], [0.5, 0.5]],
        [[0.5, 0.5], [0.0, 0.5], [0.0, 1.0]],
    ];
    for (i, w) in want.iter().enumerate() {
        assert_eq!(&micro_to_base_bary(i as u32, 1).expect("level 1"), w, "micro-triangle {i}");
    }
    assert_eq!(base_to_micro_bary([0.25, 0.25], want[1]), [0.0, 0.5]);
    assert_eq!(base_to_micro_bary([0.0, 0.5], want[1]), [0.0, 0.0]);
    // Every level-3 micro-triangle has area 1/64 of the base and they tile it.
    let area: f32 = (0..64).map(|i| {
        let [a, b, c] = micro_to_base_bary(i, 3).expect("level 3");
        ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])).abs() / 2.0
    }).sum();
    assert!((area - 0.5).abs() < 1e-6, "level-3 micro-triangles cover the base exactly: {area}");
    invalid(micro_to_base_bary(4, 1), "index past 4^level");
    invalid(micro_to_base_bary(0, sys::OPTIX_OPACITY_MICROMAP_MAX_SUBDIVISION_LEVEL + 1), "level past the maximum");
}

#[test]
#[ignore = "GPU"]
fn opacity_micromaps_cut_holes() {
    let mut rig = rig(Cfg { single_gas: true, omm: true, ..Default::default() });
    let s = rig.stream.clone();
    let data = up(&s, &omm_bits());
    let desc = [MicromapDesc::new(0, OMM_LEVEL as u16, MicromapFormat::FourState)];
    let array = OpacityMicromapArray::build(&rig.ctx, &rig.stream, &mut rig.scratch, &data, &desc, MicromapOptions::default()).expect("micromap array");
    assert_eq!(array.len(), 1);
    let usage = [MicromapCount { count: 1, level: OMM_LEVEL, format: MicromapFormat::FourState }];
    let verts = up(&s, &tri_verts(0.0, LEG));
    let want = omm_mask();
    assert_ne!(want, mask(0.0), "the pattern removes pixels");

    // SAFETY: the one triangle uses micromap 0, counted by `usage`.
    let linear = unsafe { MicromapUse::linear(&array, &usage) };
    let gas = tri_gas(&mut rig, &[TriangleMesh { micromap: Some(linear), ..TriangleMesh::new(&verts) }], BuildOptions::default());
    assert_eq!(rig.trace(gas.handle()), want, "linear micromap");

    let idx = up(&s, &[0i32]);
    // SAFETY: index 0 is the array's micromap, counted by `usage`.
    let indexed = unsafe { MicromapUse::indexed(&array, MicromapIndices::I32(&idx), 0, &usage) };
    let g = tri_gas(&mut rig, &[TriangleMesh { micromap: Some(indexed), ..TriangleMesh::new(&verts) }], BuildOptions::default());
    assert_eq!(rig.trace(g.handle()), want, "indexed micromap");

    for (special, expect, name) in [
        (sys::OPTIX_OPACITY_MICROMAP_PREDEFINED_INDEX_FULLY_TRANSPARENT, misses(), "fully transparent"),
        (sys::OPTIX_OPACITY_MICROMAP_PREDEFINED_INDEX_FULLY_OPAQUE, mask(0.0), "fully opaque"),
    ] {
        let idx = up(&s, &[special as i16]);
        // SAFETY: a predefined index references no micromap, so no usage is counted.
        let m = unsafe { MicromapUse::indexed(&array, MicromapIndices::I16(&idx), 0, &[]) };
        let g = tri_gas(&mut rig, &[TriangleMesh { micromap: Some(m), ..TriangleMesh::new(&verts) }], BuildOptions::default());
        assert_eq!(rig.trace(g.handle()), expect, "{name}");
    }

    // Relocate the array, then the GAS onto it; drop both sources.
    // SAFETY: the array's build is ordered before the copy on this stream.
    let array2 = unsafe { array.relocate(&rig.ctx, &rig.stream, None) }.expect("relocate array");
    assert_eq!(array2.size_bytes(), array.size_bytes());
    // SAFETY: the GAS build is ordered before the copy; the relinked array matches the mesh's.
    let gas2 = unsafe { gas.relocate(&rig.ctx, &rig.stream, Relink::Micromaps(&[Some(array2.clone())]), None) }.expect("relocate GAS");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { gas.relocate(&rig.ctx, &rig.stream, Relink::Micromaps(&[None]), None) }, "no array for a mesh built with one");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { gas.relocate(&rig.ctx, &rig.stream, Relink::Micromaps(&[]), None) }, "entry count mismatch");
    drop((gas, g, array));
    rig.stream.synchronize().expect("sync");
    assert_eq!(rig.trace(gas2.handle()), want, "relocated GAS over the relocated array");

    let short = up(&s, &[0u8; 8]);
    invalid(OpacityMicromapArray::build(&rig.ctx, &rig.stream, &mut rig.scratch, &short, &desc, MicromapOptions::default()), "descriptor past the data");
    let bad = [MicromapDesc::new(0, 13, MicromapFormat::TwoState)];
    invalid(OpacityMicromapArray::build(&rig.ctx, &rig.stream, &mut rig.scratch, &data, &bad, MicromapOptions::default()), "level 13");
    invalid(OpacityMicromapArray::build(&rig.ctx, &rig.stream, &mut rig.scratch, &data, &[], MicromapOptions::default()), "no micromap");
    let small = OpacityMicromapArray::build(&rig.ctx, &rig.stream, &mut rig.scratch, &data, &desc, MicromapOptions { budget: Some(8), ..Default::default() });
    assert!(matches!(small, Err(Error::Budget { available: 8, .. })), "micromap budget");
}

// ---------------------------------------------------------------------------------------------
// Cluster acceleration structures
// ---------------------------------------------------------------------------------------------

fn tri_args(index: &DeviceBuffer<u32>, verts: &DeviceBuffer<[f32; 3]>) -> sys::OptixClusterAccelBuildInputTrianglesArgs {
    let mut a = sys::OptixClusterAccelBuildInputTrianglesArgs { indexBuffer: index.cu_deviceptr(), vertexBuffer: verts.cu_deviceptr(), ..Default::default() };
    a.set_triangleCount(1);
    a.set_vertexCount(3);
    a.set_indexFormat(sys::OptixClusterAccelIndicesFormat_OPTIX_CLUSTER_ACCEL_INDICES_FORMAT_32BIT as u32);
    a
}

fn tri_limits() -> TriangleLimits {
    TriangleLimits { max_args: 1, max_triangles_per_arg: 1, max_vertices_per_arg: 3, ..Default::default() }
}

/// A GAS over the CLAS of `clas` (its first handle), built with implicit destinations or, when
/// `explicit`, with get-sizes + explicit destinations, traced through an IAS.
fn trace_clusters(rig: &mut Rig, clas: &Arc<ClusterAccel>, explicit: bool) -> Vec<u32> {
    let args = ClusterArgs::upload(&rig.stream, &[sys::OptixClusterAccelBuildInputClustersArgs {
        clusterHandlesCount: 1,
        clusterHandlesBufferStrideInBytes: size_of::<u64>() as u32,
        clusterHandlesBuffer: clas.handles().cu_deviceptr(),
    }])
    .expect("cluster args");
    let input = ClusterInput::GasesFromClusters(ClusterLimits { flags: ClusterFlags::NONE, max_args: 1, max_total_clusters: 1, max_clusters_per_arg: 1 });
    let opts = ClusterOptions { refs: ClusterRefs { clusters: std::slice::from_ref(clas), micromaps: &[] }, ..Default::default() };
    let gas = if explicit {
        // SAFETY: the handle buffer is the CLAS build's output on this stream, alive in `clas`.
        let sizes = unsafe { ClusterAccel::get_sizes(&rig.ctx, &rig.stream, &mut rig.scratch, &input, &args, None) }.expect("GAS sizes").to_host_vec(&rig.stream).expect("read sizes");
        // SAFETY: as above, kept alive by `refs`; `sizes` is the get-sizes result of these args.
        unsafe { ClusterAccel::build_explicit(&rig.ctx, &rig.stream, &mut rig.scratch, &input, &args, &sizes, opts) }.expect("explicit GAS from clusters")
    } else {
        // SAFETY: the handle buffer is the CLAS build's output on this stream, kept alive by `refs`.
        unsafe { ClusterAccel::build(&rig.ctx, &rig.stream, &mut rig.scratch, &input, &args, opts) }.expect("GAS from clusters")
    };
    assert_eq!(gas.kind(), optix::ClusterKind::GasesFromClusters);
    let child = gas.child(&rig.stream, 0).expect("cluster GAS child");
    invalid(gas.child(&rig.stream, 1), "child index past the build");
    invalid(clas.child(&rig.stream, 0), "a CLAS is not a traversable GAS");
    let top = ias(rig, &[Instance::new(child)], BuildOptions::default());
    rig.trace(top.handle())
}

#[test]
#[ignore = "GPU"]
fn cluster_builds_trace() {
    let mut rig = rig(Cfg { clusters: true, ..Default::default() });
    let supported = rig.ctx.cluster_accel().expect("cluster property");
    if !supported {
        println!("cluster acceleration structures are not supported on this device (OPTIX_DEVICE_PROPERTY_CLUSTER_ACCEL = 0)");
        let r = ClusterAccel::memory_usage(&rig.ctx, ClusterMode::Implicit, &ClusterInput::ClustersFromTriangles(tri_limits()));
        assert!(r.is_err(), "memory usage of an unsupported cluster build must fail");
        return;
    }
    let s = rig.stream.clone();
    let index = up(&s, &[0u32, 1, 2]);
    let verts = up(&s, &tri_verts(0.0, LEG));
    let input = ClusterInput::ClustersFromTriangles(tri_limits());
    let sizes = ClusterAccel::memory_usage(&rig.ctx, ClusterMode::Implicit, &input).expect("memory usage");
    println!("CLAS implicit sizes {sizes:?}");
    assert!(sizes.output > 0);
    let args = ClusterArgs::upload(&s, &[tri_args(&index, &verts)]).expect("triangle args");
    assert_eq!(args.len(), 1);

    // Implicit destinations.
    // SAFETY: index and vertex buffers outlive the build (synchronized by the readbacks below).
    let clas = unsafe { ClusterAccel::build(&rig.ctx, &s, &mut rig.scratch, &input, &args, ClusterOptions { sizes: true, ..Default::default() }) }.expect("CLAS");
    let out = clas.output_sizes().expect("sizes requested").to_host_vec(&s).expect("read sizes");
    assert!(out[0] > 0);
    assert_eq!(trace_clusters(&mut rig, &clas, false), mask(0.0), "implicit CLAS, implicit GAS");
    assert_eq!(trace_clusters(&mut rig, &clas, true), mask(0.0), "implicit CLAS, explicit GAS");

    // Get sizes, then explicit destinations.
    // SAFETY: as above.
    let exact = unsafe { ClusterAccel::get_sizes(&rig.ctx, &s, &mut rig.scratch, &input, &args, None) }.expect("get sizes").to_host_vec(&s).expect("read sizes");
    assert_eq!(exact, out, "get-sizes matches the implicit build's sizes");
    // SAFETY: `exact` is the get-sizes result of these arguments.
    let explicit = unsafe { ClusterAccel::build_explicit(&rig.ctx, &s, &mut rig.scratch, &input, &args, &exact, ClusterOptions { sizes: true, ..Default::default() }) }.expect("explicit CLAS");
    let eh = explicit.handles().to_host_vec(&s).expect("explicit handle")[0];
    let es = explicit.output_sizes().expect("sizes").to_host_vec(&s).expect("explicit sizes");
    println!("explicit CLAS handle {eh:#x}, sizes {es:?}, buffer {} bytes", explicit.size_bytes());
    assert_ne!(eh, 0, "the explicit build yields a CLAS address");
    assert_eq!(es, exact, "the explicit build reports the sizes get-sizes predicted");
    assert_eq!(explicit.size_bytes(), (exact[0] as usize).next_multiple_of(sys::OPTIX_ACCEL_BUFFER_BYTE_ALIGNMENT as usize), "one tight, aligned object");
    assert_eq!(trace_clusters(&mut rig, &explicit, false), mask(0.0), "explicit CLAS, implicit GAS");
    assert_eq!(trace_clusters(&mut rig, &explicit, true), mask(0.0), "explicit CLAS, explicit GAS");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { ClusterAccel::build_explicit(&rig.ctx, &s, &mut rig.scratch, &input, &args, &[], ClusterOptions::default()) }, "no sizes");

    // Templates from triangles, instantiated at moved vertices.
    let tinput = ClusterInput::TemplatesFromTriangles(tri_limits());
    // SAFETY: as above.
    let templates = unsafe { ClusterAccel::build(&rig.ctx, &s, &mut rig.scratch, &tinput, &args, ClusterOptions::default()) }.expect("templates");
    let tmpl = templates.handles().to_host_vec(&s).expect("template address")[0];
    assert_ne!(tmpl, 0);
    let moved = up(&s, &tri_verts(DX, LEG));
    let targs = ClusterArgs::upload(&s, &[sys::OptixClusterAccelBuildInputTemplatesArgs { clusterTemplate: tmpl, vertexBuffer: moved.cu_deviceptr(), ..Default::default() }]).expect("template args");
    let iinput = ClusterInput::ClustersFromTemplates(tri_limits());
    let refs = [templates.clone()];
    // SAFETY: the template is kept alive by `refs`; `moved` outlives the build.
    let inst = unsafe { ClusterAccel::build(&rig.ctx, &s, &mut rig.scratch, &iinput, &targs, ClusterOptions { refs: ClusterRefs { clusters: &refs, micromaps: &[] }, ..Default::default() }) }
        .expect("CLAS from templates");
    assert_eq!(trace_clusters(&mut rig, &inst, false), mask(DX), "CLAS instantiated from a template");

    // Templates from a 2x2 grid (smoke).
    let mut g = sys::OptixClusterAccelBuildInputGridsArgs { dimensions: [2, 2], ..Default::default() };
    g.set_positionTruncateBitCount(0);
    let gargs = ClusterArgs::upload(&s, &[g]).expect("grid args");
    let ginput = ClusterInput::TemplatesFromGrids(GridLimits { flags: ClusterFlags::NONE, max_args: 1, vertex_format: VertexFormat::Float3, max_sbt_index: 0, max_width: 2, max_height: 2 });
    // SAFETY: grid arguments reference no device memory.
    let grid_t = unsafe { ClusterAccel::build(&rig.ctx, &s, &mut rig.scratch, &ginput, &gargs, ClusterOptions::default()) }.expect("templates from grids");
    assert_ne!(grid_t.handles().to_host_vec(&s).expect("grid template")[0], 0);
    assert!(grid_t.size_bytes() > 0);

    // Invalid and budget paths.
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { ClusterAccel::build(&rig.ctx, &s, &mut rig.scratch, &ginput, &args, ClusterOptions::default()) }, "triangle args for a grid build");
    let zero = ClusterInput::ClustersFromTriangles(TriangleLimits { max_args: 0, ..tri_limits() });
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { ClusterAccel::build(&rig.ctx, &s, &mut rig.scratch, &zero, &args, ClusterOptions::default()) }, "max_args 0");
    let two = ClusterInput::ClustersFromTriangles(TriangleLimits { max_args: 2, ..tri_limits() });
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { ClusterAccel::build(&rig.ctx, &s, &mut rig.scratch, &two, &args, ClusterOptions::default()) }, "fewer args than max_args");
    // SAFETY: rejected before allocating.
    let r = unsafe { ClusterAccel::build(&rig.ctx, &s, &mut rig.scratch, &input, &args, ClusterOptions { budget: Some(1), ..Default::default() }) };
    assert!(matches!(r, Err(Error::Budget { available: 1, .. })), "cluster budget");
    let other = rig.ctx.cuda().new_stream().expect("stream");
    let mut other_scratch = Scratch::new(&other);
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { ClusterAccel::build(&rig.ctx, &other, &mut other_scratch, &input, &args, ClusterOptions::default()) }, "args uploaded on another stream");
}

// ---------------------------------------------------------------------------------------------
// Denoiser
// ---------------------------------------------------------------------------------------------

const S: u32 = 64;

/// Nearest-neighbour upscale of an `S x S` image by `k`.
fn upscale(px: &[[f32; 4]], k: u32) -> Vec<[f32; 4]> {
    let w = S * k;
    (0..w * w).map(|i| px[((i / w / k) * S + (i % w) / k) as usize]).collect()
}

/// Denoises a noisy grey frame with `model` (two frames for temporal models, the second with the
/// first's output as previous) and checks the noise drops on each frame.
fn denoise_model(model: DenoiserModel) {
    let (cuda, stream) = cuda();
    let ctx = DeviceContext::new(&cuda, None, 0, false).expect("context");
    let n = (S * S) as usize;
    let k = model.scale();
    let noisy_px = noisy(n, 0.5, 0.25, 7);
    let before = deviation(&noisy_px, 0.5);
    let input = up(&stream, &noisy_px);
    let flow = DeviceBuffer::<[f32; 2]>::zeroed(&stream, n).expect("flow");
    let mut den = Denoiser::new(&ctx, &stream, DenoiserOptions { model, ..Default::default() }, S, S).unwrap_or_else(|e| panic!("{model:?}: {e}"));
    assert_eq!(den.size(), (S, S));
    assert_eq!(den.output_size(), (S * k, S * k));
    let temporal = model.is_temporal();
    let guides = Guides { flow: temporal.then(|| Image::new(&flow, S, S).expect("flow image")), ..Default::default() };
    let first_prev = up(&stream, &upscale(&noisy_px, k));
    let frames = if temporal { 2 } else { 1 };
    let mut prev_out: Option<DeviceBuffer<[f32; 4]>> = None;
    for frame in 0..frames {
        let out = DeviceBuffer::<[f32; 4]>::zeroed(&stream, n * (k * k) as usize).expect("output");
        let previous = temporal.then(|| Image::new(prev_out.as_ref().unwrap_or(&first_prev), S * k, S * k).expect("previous"));
        let layer = Layer { input: Image::new(&input, S, S).expect("input"), output: Image::new(&out, S * k, S * k).expect("output"), previous, aov: AovType::Beauty };
        // SAFETY: every image outlives the synchronization below.
        unsafe { den.invoke(&stream, &guides, &[layer], Params::default()) }.unwrap_or_else(|e| panic!("{model:?} frame {frame}: {e}"));
        stream.synchronize().expect("sync");
        let px = out.to_host_vec(&stream).expect("read back");
        assert!(px.iter().flatten().all(|c| c.is_finite()), "{model:?}: finite output");
        let after = deviation(&px, 0.5);
        println!("{model:?} frame {frame}: noise {before:.4} -> {after:.4} ({} px)", px.len());
        assert!(after < before * 0.5, "{model:?} frame {frame}: noise {before} -> {after}");
        prev_out = Some(out);
    }
}

#[test]
#[ignore = "GPU"]
fn denoiser_ldr() {
    denoise_model(DenoiserModel::Ldr);
}

#[test]
#[ignore = "GPU"]
fn denoiser_hdr() {
    denoise_model(DenoiserModel::Hdr);
}

#[test]
#[ignore = "GPU"]
fn denoiser_aov() {
    denoise_model(DenoiserModel::Aov);
}

#[test]
#[ignore = "GPU"]
fn denoiser_temporal() {
    denoise_model(DenoiserModel::Temporal);
}

#[test]
#[ignore = "GPU"]
fn denoiser_temporal_aov() {
    denoise_model(DenoiserModel::TemporalAov);
}

#[test]
#[ignore = "GPU"]
fn denoiser_upscale2x() {
    denoise_model(DenoiserModel::Upscale2x);
}

#[test]
#[ignore = "GPU"]
fn denoiser_temporal_upscale2x() {
    denoise_model(DenoiserModel::TemporalUpscale2x);
}

fn den_ctx() -> (Arc<Stream>, Arc<DeviceContext>) {
    let (cuda, stream) = cuda();
    (stream, DeviceContext::new(&cuda, None, 0, false).expect("context"))
}

#[test]
#[ignore = "GPU"]
fn denoiser_aov_layers() {
    let (stream, ctx) = den_ctx();
    let n = (S * S) as usize;
    let (beauty, diffuse) = (noisy(n, 0.5, 0.25, 7), noisy(n, 0.3, 0.2, 99));
    let (bi, di) = (up(&stream, &beauty), up(&stream, &diffuse));
    let bo = DeviceBuffer::<[f32; 4]>::zeroed(&stream, n).expect("out");
    let do_ = DeviceBuffer::<[f32; 4]>::zeroed(&stream, n).expect("out");
    let albedo = up(&stream, &vec![[0.5f32, 0.5, 0.5, 1.0]; n]);
    let normal = up(&stream, &vec![[0.0f32, 0.0, 1.0, 0.0]; n]);
    let mut den = Denoiser::new(&ctx, &stream, DenoiserOptions { albedo: true, normal: true, ..Default::default() }, S, S).expect("AOV denoiser");
    let img = |b| Image::new(b, S, S).expect("image");
    let guides = Guides { albedo: Some(img(&albedo)), normal: Some(img(&normal)), ..Default::default() };
    let layers = [Layer { aov: AovType::Beauty, ..Layer::new(img(&bi), img(&bo)) }, Layer { aov: AovType::Diffuse, ..Layer::new(img(&di), img(&do_)) }];
    // SAFETY: every image outlives the synchronization below.
    unsafe { den.invoke(&stream, &guides, &layers, Params::default()) }.expect("invoke");
    stream.synchronize().expect("sync");
    let (b_after, d_after) = (deviation(&bo.to_host_vec(&stream).expect("beauty"), 0.5), deviation(&do_.to_host_vec(&stream).expect("diffuse"), 0.3));
    println!("beauty {:.4} -> {b_after:.4}, diffuse {:.4} -> {d_after:.4}", deviation(&beauty, 0.5), deviation(&diffuse, 0.3));
    assert!(b_after < deviation(&beauty, 0.5) * 0.5);
    assert!(d_after < deviation(&diffuse, 0.3) * 0.5);

    // Two outputs in one buffer region, and AOVs on a beauty-only model, are rejected.
    let clash = [Layer::new(img(&bi), img(&bo)), Layer::new(img(&di), img(&bo))];
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { den.invoke(&stream, &guides, &clash, Params::default()) }, "overlapping outputs");
    let mut hdr = Denoiser::new(&ctx, &stream, DenoiserOptions { model: DenoiserModel::Hdr, ..Default::default() }, S, S).expect("HDR");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { hdr.invoke(&stream, &Guides::default(), &layers, Params::default()) }, "AOV layers on the HDR model");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { hdr.invoke(&stream, &Guides::default(), &layers[..1], Params { blend: 2.0, ..Default::default() }) }, "blend outside [0, 1]");
}

#[test]
#[ignore = "GPU"]
fn denoiser_half_and_strided_images() {
    let (stream, ctx) = den_ctx();
    let n = (S * S) as usize;
    let px = noisy(n, 0.5, 0.25, 7);
    let before = deviation(&px, 0.5);
    let mut den = Denoiser::new(&ctx, &stream, DenoiserOptions::default(), S, S).expect("denoiser");

    // Half RGBA in and out.
    let hin = up(&stream, &px.iter().map(|p| p.map(f16)).collect::<Vec<[u16; 4]>>());
    let hout = DeviceBuffer::<[u16; 4]>::zeroed(&stream, n).expect("half out");
    let hi = Image::with_format(&hin, S, S, PixelFormat::Half4).expect("half image");
    let ho = Image::with_format(&hout, S, S, PixelFormat::Half4).expect("half image");
    // SAFETY: every image outlives the synchronization below.
    unsafe { den.invoke(&stream, &Guides::default(), &[Layer::new(hi, ho)], Params::default()) }.expect("half invoke");
    stream.synchronize().expect("sync");
    let back: Vec<[f32; 4]> = hout.to_host_vec(&stream).expect("read").iter().map(|p| p.map(f32_of)).collect();
    let after = deviation(&back, 0.5);
    println!("half: {before:.4} -> {after:.4}");
    assert!(after < before * 0.5);

    // RGB of an RGBA input; output rows padded by 4 pixels holding a sentinel.
    let rgba = up(&stream, &px);
    let pad = 4;
    let row = (S + pad) as usize;
    let mut out_host = vec![[9.0f32; 4]; row * S as usize];
    let sout = up(&stream, &out_host);
    let si = Image::strided(&rgba, 0, S, S, PixelFormat::Float3, S * 16, 16).expect("RGB view");
    let so = Image::strided(&sout, 0, S, S, PixelFormat::Float4, (S + pad) * 16, 16).expect("padded output");
    // SAFETY: as above.
    unsafe { den.invoke(&stream, &Guides::default(), &[Layer::new(si, so)], Params::default()) }.expect("strided invoke");
    stream.synchronize().expect("sync");
    out_host = sout.to_host_vec(&stream).expect("read");
    let image: Vec<[f32; 4]> = out_host.chunks(row).flat_map(|r| r[..S as usize].iter().copied()).collect();
    let padding_kept = out_host.chunks(row).all(|r| r[S as usize..].iter().all(|p| *p == [9.0; 4]));
    let after = deviation(&image, 0.5);
    println!("strided: {before:.4} -> {after:.4}, padding kept = {padding_kept}");
    assert!(after < before * 0.5);
    assert!(padding_kept, "row padding is not written");

    invalid(Image::strided(&rgba, 0, S, S, PixelFormat::Float4, S * 16 - 4, 16), "row stride below width * pixel stride");
    invalid(Image::strided(&rgba, 2, S, S, PixelFormat::Float1, S * 16, 16), "misaligned offset");
    invalid(Image::strided(&rgba, 16, S, S, PixelFormat::Float4, S * 16, 16), "past the end");
    invalid(Image::with_format(&hin, S, S, PixelFormat::Float4), "size mismatch");
}

#[test]
#[ignore = "GPU"]
fn denoiser_tiled_matches_untiled() {
    let (stream, ctx) = den_ctx();
    let s = 128u32;
    let n = (s * s) as usize;
    let input = up(&stream, &noisy(n, 0.5, 0.25, 7));
    let run = |tile: Option<[u32; 2]>| {
        let out = DeviceBuffer::<[f32; 4]>::zeroed(&stream, n).expect("out");
        let mut den = Denoiser::new(&ctx, &stream, DenoiserOptions { tile, ..Default::default() }, s, s).expect("denoiser");
        let layer = Layer::new(Image::new(&input, s, s).expect("in"), Image::new(&out, s, s).expect("out"));
        // SAFETY: every image outlives the synchronization below.
        unsafe { den.invoke(&stream, &Guides::default(), &[layer], Params::default()) }.expect("invoke");
        stream.synchronize().expect("sync");
        (out.to_host_vec(&stream).expect("read"), den.overlap(), den.memory_bytes())
    };
    let (whole, o0, m0) = run(None);
    let (tiled, o1, m1) = run(Some([48, 48]));
    assert_eq!(o0, 0);
    assert!(o1 > 0, "tiled denoisers report an overlap");
    let diffs: Vec<f32> = whole.iter().flatten().zip(tiled.iter().flatten()).map(|(a, b)| (a - b).abs()).collect();
    let mean = diffs.iter().sum::<f32>() / diffs.len() as f32;
    let max = diffs.iter().copied().fold(0.0, f32::max);
    println!("tiled vs untiled: overlap {o1}, memory {m0} -> {m1}, mean diff {mean:.6}, max diff {max:.6}");
    assert!(mean < 0.005, "tiled output differs from untiled: mean {mean}");
}

#[test]
#[ignore = "GPU"]
fn denoiser_intensity_and_average_color() {
    let (stream, ctx) = den_ctx();
    let n = (S * S) as usize;
    let mut den = Denoiser::new(&ctx, &stream, DenoiserOptions::default(), S, S).expect("denoiser");
    let intensity = |den: &mut Denoiser, c: [f32; 4]| {
        let b = up(&stream, &vec![c; n]);
        let mut out = DeviceBuffer::<f32>::zeroed(&stream, 1).expect("out");
        // SAFETY: `b` and `out` outlive the synchronization below.
        unsafe { den.compute_intensity(&stream, Image::new(&b, S, S).expect("image"), &mut out) }.expect("intensity");
        stream.synchronize().expect("sync");
        out.to_host_vec(&stream).expect("read")[0]
    };
    let (i1, i2) = (intensity(&mut den, [0.25, 0.25, 0.25, 1.0]), intensity(&mut den, [0.5, 0.5, 0.5, 1.0]));
    println!("intensity(0.25) = {i1}, intensity(0.5) = {i2}");
    assert!(i1.is_finite() && i2.is_finite() && i1 > 0.0 && i2 > 0.0);
    assert!((i1 / i2 - 2.0).abs() < 0.1, "intensity is inversely proportional to brightness: {i1} / {i2}");

    let average = |den: &mut Denoiser, c: [f32; 4]| {
        let b = up(&stream, &vec![c; n]);
        let mut out = DeviceBuffer::<[f32; 3]>::zeroed(&stream, 1).expect("out");
        // SAFETY: as above.
        unsafe { den.compute_average_color(&stream, Image::new(&b, S, S).expect("image"), &mut out) }.expect("average colour");
        stream.synchronize().expect("sync");
        out.to_host_vec(&stream).expect("read")[0]
    };
    let grey = average(&mut den, [0.4, 0.4, 0.4, 1.0]);
    let col = average(&mut den, [0.2, 0.4, 0.8, 1.0]);
    println!("average colour: grey {grey:?}, colour {col:?}");
    assert!(grey.iter().chain(&col).all(|v| v.is_finite()));
    assert!((grey[0] - grey[1]).abs() < 1e-5 && (grey[1] - grey[2]).abs() < 1e-5, "equal channels give equal averages");
    let increasing = col[0] < col[1] && col[1] < col[2];
    let decreasing = col[0] > col[1] && col[1] > col[2];
    assert!(increasing || decreasing, "distinct channels give strictly ordered averages: {col:?}");

    let two = DeviceBuffer::<[f32; 2]>::zeroed(&stream, n).expect("two channels");
    let mut out = DeviceBuffer::<f32>::zeroed(&stream, 1).expect("out");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { den.compute_intensity(&stream, Image::new(&two, S, S).expect("image"), &mut out) }, "two-channel intensity");
    let mut empty = DeviceBuffer::<f32>::zeroed(&stream, 0).expect("empty");
    let four = up(&stream, &vec![[0.5f32; 4]; n]);
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { den.compute_intensity(&stream, Image::new(&four, S, S).expect("image"), &mut empty) }, "empty output");
    let other = ctx.cuda().new_stream().expect("stream");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { den.compute_intensity(&other, Image::new(&four, S, S).expect("image"), &mut out) }, "another stream");
}

#[test]
#[ignore = "GPU"]
fn denoiser_user_model_and_option_checks() {
    let (stream, ctx) = den_ctx();
    invalid(Denoiser::with_user_model(&ctx, &stream, &[], DenoiserOptions::default(), S, S), "empty weights");
    invalid(Denoiser::with_user_model(&ctx, &stream, &[1, 2, 3], DenoiserOptions { alpha: optix::AlphaMode::Denoise, ..Default::default() }, S, S), "alpha with a user model");
    let err = Denoiser::with_user_model(&ctx, &stream, &[0u8; 256], DenoiserOptions::default(), S, S).map(|_| ()).expect_err("garbage weights");
    println!("garbage user weights: {err}");
    assert!(matches!(err, Error::Optix { call: "optixDenoiserCreateWithUserModel", .. }), "{err}");
    invalid(Denoiser::new(&ctx, &stream, DenoiserOptions { normal: true, ..Default::default() }, S, S), "normal without albedo");
    invalid(Denoiser::new(&ctx, &stream, DenoiserOptions { tile: Some([0, 8]), ..Default::default() }, S, S), "zero tile");
    invalid(Denoiser::new(&ctx, &stream, DenoiserOptions::default(), 0, S), "zero width");

    // Temporal frames need a flow guide and previous outputs; others take neither.
    let n = (S * S) as usize;
    let buf = up(&stream, &noisy(n, 0.5, 0.1, 3));
    let out = DeviceBuffer::<[f32; 4]>::zeroed(&stream, n).expect("out");
    let img = |b| Image::new(b, S, S).expect("image");
    let mut t = Denoiser::new(&ctx, &stream, DenoiserOptions { model: DenoiserModel::TemporalAov, ..Default::default() }, S, S).expect("temporal");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { t.invoke(&stream, &Guides::default(), &[Layer { previous: Some(img(&buf)), ..Layer::new(img(&buf), img(&out)) }], Params::default()) }, "temporal without flow");
    let flow = DeviceBuffer::<[f32; 2]>::zeroed(&stream, n).expect("flow");
    let g = Guides { flow: Some(Image::new(&flow, S, S).expect("flow")), ..Default::default() };
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { t.invoke(&stream, &g, &[Layer::new(img(&buf), img(&out))], Params::default()) }, "temporal without previous output");
    let mut a = Denoiser::new(&ctx, &stream, DenoiserOptions::default(), S, S).expect("aov");
    // SAFETY: rejected before reaching OptiX.
    invalid(unsafe { a.invoke(&stream, &Guides::default(), &[Layer { previous: Some(img(&buf)), ..Layer::new(img(&buf), img(&out)) }], Params::default()) }, "previous on a non-temporal model");
}

#[test]
#[ignore = "GPU"]
fn denoiser_budget_is_exact() {
    let (stream, ctx) = den_ctx();
    for model in [DenoiserModel::Aov, DenoiserModel::TemporalAov] {
        let opts = DenoiserOptions { model, ..Default::default() };
        let held = Denoiser::new(&ctx, &stream, opts, S, S).expect("unbudgeted").memory_bytes();
        // Peak = state + scratch + internal guide layers (memory_bytes) + intensity (f32) and
        // average colour ([f32; 3]).
        let peak = held + size_of::<f32>() + size_of::<[f32; 3]>();
        let r = Denoiser::new(&ctx, &stream, DenoiserOptions { budget: Some(peak - 1), ..opts }, S, S);
        assert!(matches!(r, Err(Error::Budget { requested, available }) if requested == peak && available == peak - 1), "{model:?}: one byte short");
        Denoiser::new(&ctx, &stream, DenoiserOptions { budget: Some(peak), ..opts }, S, S).unwrap_or_else(|e| panic!("{model:?}: exact budget: {e}"));
    }
}

// ---------------------------------------------------------------------------------------------
// Cooperative vectors
// ---------------------------------------------------------------------------------------------

#[test]
#[ignore = "GPU"]
fn coop_vec_matrices_convert_round_trip() {
    let (stream, ctx) = den_ctx();
    if !ctx.coop_vec().expect("coop vec property") {
        println!("cooperative vectors are not supported on this device (OPTIX_DEVICE_PROPERTY_COOP_VEC = 0)");
        return;
    }
    let shapes = [(16u32, 32u32), (8, 16)];
    let f32_rm = Network::packed(&ctx, &shapes, ElemType::F32, MatrixLayout::RowMajor).expect("f32 row-major");
    let f16_opt = Network::packed(&ctx, &shapes, ElemType::F16, MatrixLayout::InferencingOptimal).expect("f16 optimal");
    for (m, &(n, k)) in f32_rm.matrices().iter().zip(&shapes) {
        assert_eq!((m.n, m.k, m.stride), (n, k, k * 4));
        assert_eq!(m.offset as usize % coop_vec::MATRIX_ALIGN, 0);
        assert!(m.size as usize >= (n * k * 4) as usize);
        assert_eq!(m.size as usize, coop_vec::matrix_size(&ctx, n, k, ElemType::F32, MatrixLayout::RowMajor, 0).expect("size"));
    }
    println!("f32 row-major {} bytes, f16 optimal {} bytes", f32_rm.bytes(), f16_opt.bytes());

    // Values exact in f16: multiples of 1/64 in [-2, 2).
    let value = |m: usize, r: u32, c: u32| ((m as i32 * 37 + r as i32 * 11 + c as i32 * 5) % 256 - 128) as f32 / 64.0;
    let mut host = vec![0f32; f32_rm.bytes() / 4];
    for (mi, m) in f32_rm.matrices().iter().enumerate() {
        for r in 0..m.n {
            for c in 0..m.k {
                host[(m.offset / 4 + r * m.k + c) as usize] = value(mi, r, c);
            }
        }
    }
    let src = up(&stream, &host);
    let mut mid = DeviceBuffer::<u8>::zeroed(&stream, f16_opt.bytes()).expect("f16 buffer");
    let mut back = DeviceBuffer::<f32>::zeroed(&stream, host.len()).expect("f32 buffer");
    // SAFETY: the buffers outlive the synchronization below.
    unsafe { coop_vec::convert(&ctx, &stream, 1, Batch { network: &f32_rm, buffer: &src, stride: 0 }, Batch { network: &f16_opt, buffer: &mut mid, stride: 0 }) }.expect("f32 -> f16 optimal");
    // SAFETY: as above; the conversion into `mid` is ordered before this one.
    unsafe { coop_vec::convert(&ctx, &stream, 1, Batch { network: &f16_opt, buffer: &mid, stride: 0 }, Batch { network: &f32_rm, buffer: &mut back, stride: 0 }) }.expect("f16 optimal -> f32");
    stream.synchronize().expect("sync");
    let got = back.to_host_vec(&stream).expect("read");
    for (mi, m) in f32_rm.matrices().iter().enumerate() {
        for r in 0..m.n {
            for c in 0..m.k {
                let i = (m.offset / 4 + r * m.k + c) as usize;
                assert_eq!(got[i], value(mi, r, c), "matrix {mi} ({r}, {c})");
            }
        }
    }

    // Invalid cases.
    invalid(coop_vec::matrix_size(&ctx, 0, 4, ElemType::F16, MatrixLayout::RowMajor, 0), "zero rows");
    invalid(coop_vec::matrix_size(&ctx, 4, 4, ElemType::F16, MatrixLayout::InferencingOptimal, 8), "stride on an optimal layout");
    invalid(coop_vec::matrix_size(&ctx, 4, 4, ElemType::F16, MatrixLayout::RowMajor, 4), "stride below a packed row");
    let m = Matrix { n: 4, k: 4, elem: ElemType::F16, layout: MatrixLayout::RowMajor, stride: 8, offset: 0, size: 128 };
    invalid(Network::new(vec![]), "empty network");
    invalid(Network::new(vec![m, Matrix { offset: 64, ..m }]), "overlapping matrices");
    invalid(Network::new(vec![Matrix { offset: 32, ..m }]), "unaligned offset");
    let f8_rm = Network::new(vec![Matrix { elem: ElemType::F8E4M3, stride: 4, ..m }]).expect("f8 row-major description");
    let f16_rm = Network::new(vec![m]).expect("f16 row-major");
    let i8_rm = Network::new(vec![Matrix { elem: ElemType::I8, stride: 4, ..m }]).expect("i8 row-major");
    let other = Network::new(vec![Matrix { n: 8, ..m }]).expect("other shape");
    let small = up(&stream, &[0u8; 256]);
    let mut dst = DeviceBuffer::<u8>::zeroed(&stream, 256).expect("dst");
    // SAFETY: every call below is rejected before reaching OptiX.
    unsafe {
        invalid(coop_vec::convert(&ctx, &stream, 0, Batch { network: &f16_rm, buffer: &small, stride: 0 }, Batch { network: &f16_rm, buffer: &mut dst, stride: 0 }), "zero networks");
        invalid(coop_vec::convert(&ctx, &stream, 1, Batch { network: &f16_rm, buffer: &small, stride: 0 }, Batch { network: &other, buffer: &mut dst, stride: 0 }), "shape mismatch");
        invalid(coop_vec::convert(&ctx, &stream, 1, Batch { network: &f16_rm, buffer: &small, stride: 0 }, Batch { network: &f8_rm, buffer: &mut dst, stride: 0 }), "f8 into row-major");
        invalid(coop_vec::convert(&ctx, &stream, 1, Batch { network: &i8_rm, buffer: &small, stride: 0 }, Batch { network: &f16_rm, buffer: &mut dst, stride: 0 }), "integer to float");
        invalid(coop_vec::convert(&ctx, &stream, 4, Batch { network: &f16_rm, buffer: &small, stride: 128 }, Batch { network: &f16_rm, buffer: &mut dst, stride: 128 }), "networks past the buffer");
        invalid(coop_vec::convert(&ctx, &stream, 2, Batch { network: &f16_rm, buffer: &small, stride: 96 }, Batch { network: &f16_rm, buffer: &mut dst, stride: 128 }), "unaligned network stride");
    }
}
