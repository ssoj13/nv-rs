//! GPU tests for `optix-device`: OptiX programs written in Rust (compiled by cuda-oxide into this
//! binary's embedded PTX) driven by the host `optix` crate. Run with `cargo oxide run --arch
//! <sm>`; each case prints PASS or exits non-zero.
//!
//! Case `shade`: a 32x24 orthographic launch over a quad of two triangles. The raygen traces a
//! typed payload; the closest-hit reads its SBT data, the triangle barycentrics and the primitive
//! index and calls a direct callable through the pointer `optixDirectCall` resolves; the miss
//! program writes the background from its SBT data. Every pixel is compared with a CPU reference.

use bytemuck::{Pod, Zeroable};
use cuda_core::{Context, DeviceBuffer};
use cuda_core::embedded::{ArtifactPayloadKind, embedded_modules_from_current_exe};
use cuda_device::{ConstantMemory, constant, cuda_module, device, kernel, ptx_asm};
use optix_device::trace::{Ray, TraceOptions};
use optix_device::{payload, raw, state, trace};

const W: u32 = 32;
const H: u32 = 24;

/// Launch parameters (`params` in constant memory).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    pub out: u64,
    pub handle: u64,
    pub width: u32,
    pub height: u32,
}

// SAFETY: plain integers; all-zero is valid.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

/// The payload: shaded colour and the primitive hit (-1 on a miss).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Default)]
pub struct Shade {
    pub rgb: [f32; 3],
    pub prim: i32,
}

/// SBT data of the miss and hit-group records.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Color {
    pub rgb: [f32; 3],
    pub _pad: f32,
}

/// Calls direct callable `index` as `fn(f32) -> f32` (PTX indirect call; cuda-oxide does not
/// call through Rust function pointers).
#[inline(always)]
unsafe fn direct_call_f32(index: u32, x: f32) -> f32 {
    // SAFETY: valid where `optixDirectCall` is; the callable at `index` has this signature.
    let f = unsafe { raw::call_direct_callable(index) };
    let r: f32;
    // SAFETY: an indirect call through the address OptiX resolved, with the callee's prototype.
    unsafe {
        ptx_asm!(
            "{\n\t.param .b32 nvrs_rv;\n\t.param .b32 nvrs_a0;\n\tst.param.b32 [nvrs_a0], %2;\n\tnvrs_proto: .callprototype (.param .b32 _) _ (.param .b32 _);\n\tcall (nvrs_rv), %1, (nvrs_a0), nvrs_proto;\n\tld.param.b32 %0, [nvrs_rv];\n}",
            out("=f") r,
            in("l") f,
            in("f") x,
        )
    };
    r
}

/// The direct callable: doubles its argument.
#[device]
pub fn __direct_callable__double(x: f32) -> f32 {
    2.0 * x
}

#[cuda_module]
mod kernels {
    use super::*;

    #[constant(export_name = "params")]
    static PARAMS: ConstantMemory<Params> = ConstantMemory::UNINIT;

    #[kernel]
    pub fn __raygen__render() {
        // SAFETY: OptiX raygen program; the host sets `params` and sizes `out` to the launch.
        unsafe {
            let [x, y, _] = state::launch_index();
            let p = PARAMS.get();
            let origin = [(x as f32 + 0.5) / p.width as f32, (y as f32 + 0.5) / p.height as f32, -1.0];
            let ray = Ray::new(origin, [0.0, 0.0, 1.0], 0.0, 1.0e16);
            let mut shade = Shade { rgb: [0.0; 3], prim: -2 };
            trace::trace(p.handle, &ray, &TraceOptions::default(), &mut shade);
            let out = (p.out as *mut [f32; 4]).add((y * p.width + x) as usize);
            *out = [shade.rgb[0], shade.rgb[1], shade.rgb[2], shade.prim as f32];
        }
    }

    #[kernel]
    pub fn __miss__background() {
        // SAFETY: miss program; its SBT record holds a `Color`; the payload is a `Shade`.
        unsafe {
            let c = state::sbt_data::<Color>();
            payload::set(&Shade { rgb: c.rgb, prim: -1 });
        }
    }

    #[kernel]
    pub fn __closesthit__shade() {
        // SAFETY: closest-hit on triangles; its SBT record holds a `Color`; callable 0 doubles.
        unsafe {
            let c = state::sbt_data::<Color>();
            let [u, v] = state::triangle_barycentrics();
            let k = direct_call_f32(0, u);
            let rgb = [c.rgb[0] * k, c.rgb[1] * v, c.rgb[2] * (1.0 - u - v)];
            payload::set(&Shade { rgb, prim: state::primitive_index() as i32 });
        }
    }
}

const QUAD: [[f32; 3]; 4] = [[0.25, 0.25, 0.0], [0.75, 0.25, 0.0], [0.75, 0.75, 0.0], [0.25, 0.75, 0.0]];
const TRIS: [[u32; 3]; 2] = [[0, 1, 2], [0, 2, 3]];
const HIT: [f32; 3] = [0.9, 0.6, 0.3];
const BG: [f32; 3] = [0.1, 0.2, 0.3];

/// CPU reference for one pixel: what the programs above must produce.
fn reference(x: u32, y: u32) -> [f32; 4] {
    let p = [(x as f32 + 0.5) / W as f32, (y as f32 + 0.5) / H as f32];
    for (prim, t) in TRIS.iter().enumerate() {
        let [a, b, c] = t.map(|i| QUAD[i as usize]);
        // Solve p = a + u (b - a) + v (c - a) in 2D.
        let (e1, e2, d) = ([b[0] - a[0], b[1] - a[1]], [c[0] - a[0], c[1] - a[1]], [p[0] - a[0], p[1] - a[1]]);
        let det = e1[0] * e2[1] - e1[1] * e2[0];
        let u = (d[0] * e2[1] - d[1] * e2[0]) / det;
        let v = (e1[0] * d[1] - e1[1] * d[0]) / det;
        if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
            return [HIT[0] * 2.0 * u, HIT[1] * v, HIT[2] * (1.0 - u - v), prim as f32];
        }
    }
    [BG[0], BG[1], BG[2], -1.0]
}

fn case_shade(ptx: &[u8]) -> Result<(), String> {
    use optix::{Accel, BuildOptions, DeviceContext, GeometryFlags, HitGroup, LinkOptions, Module, ModuleOptions, Pipeline, PipelineOptions, ProgramGroup, SbtRecord, Scratch, ShaderBindingTable, TriangleMesh};
    let e = |x: &dyn std::fmt::Display| x.to_string();
    let cuda = Context::new(0).map_err(|x| e(&x))?;
    let stream = cuda.default_stream();
    let ctx = DeviceContext::new(&cuda, Some(Box::new(|l, t, m| eprintln!("[optix {l} {t}] {m}"))), 2, false).map_err(|x| e(&x))?;
    let mut opts = PipelineOptions::new("params", size_of::<Params>(), 4).map_err(|x| e(&x))?;
    opts.single_gas = true;
    let (module, log) = Module::new(&ctx, &ModuleOptions::default(), &opts, ptx).map_err(|x| e(&x))?;
    if !log.is_empty() {
        eprintln!("[module log] {log}");
    }
    let raygen = ProgramGroup::raygen(&module, "__raygen__render").map_err(|x| e(&x))?;
    let miss = ProgramGroup::miss(&module, "__miss__background").map_err(|x| e(&x))?;
    let hit = ProgramGroup::hit_group(&ctx, HitGroup { closest_hit: Some((&module, "__closesthit__shade")), ..Default::default() }).map_err(|x| e(&x))?;
    let callable = ProgramGroup::callables(&ctx, Some((&module, "__direct_callable__double")), None).map_err(|x| e(&x))?;
    let (pipeline, _) = Pipeline::new(&ctx, &opts, LinkOptions::default(), &[raygen.clone(), miss.clone(), hit.clone(), callable.clone()]).map_err(|x| e(&x))?;
    let sbt = ShaderBindingTable::new(
        &stream,
        &SbtRecord::new(&raygen, 0u32).map_err(|x| e(&x))?,
        &[SbtRecord::new(&miss, Color { rgb: BG, _pad: 0.0 }).map_err(|x| e(&x))?],
        &[SbtRecord::new(&hit, Color { rgb: HIT, _pad: 0.0 }).map_err(|x| e(&x))?],
    )
    .and_then(|s| s.with_callables(&stream, &[SbtRecord::new(&callable, 0u32)?]))
    .map_err(|x| e(&x))?;

    let verts = DeviceBuffer::from_host(&stream, &QUAD).map_err(|x| e(&x))?;
    let index = DeviceBuffer::from_host(&stream, &TRIS).map_err(|x| e(&x))?;
    let mut scratch = Scratch::new(&stream);
    let mesh = TriangleMesh { vertices: &verts, indices: Some(&index), flags: GeometryFlags::DISABLE_ANYHIT };
    let gas = Accel::triangles(&ctx, &stream, &mut scratch, &[mesh], BuildOptions::default()).map_err(|x| e(&x))?;

    let out = DeviceBuffer::<[f32; 4]>::zeroed(&stream, (W * H) as usize).map_err(|x| e(&x))?;
    let params = Params { out: out.cu_deviceptr(), handle: gas.handle(), width: W, height: H };
    let params_dev = DeviceBuffer::from_host(&stream, bytemuck::bytes_of(&params)).map_err(|x| e(&x))?;
    // SAFETY: `params` matches the device struct; records match what each program reads;
    // everything outlives the synchronization below.
    unsafe { pipeline.launch(&stream, &params_dev, &sbt, [W, H, 1]) }.map_err(|x| e(&x))?;
    stream.synchronize().map_err(|x| e(&x))?;
    let got = out.to_host_vec(&stream).map_err(|x| e(&x))?;

    let mut bad = Vec::new();
    for y in 0..H {
        for x in 0..W {
            let (g, w) = (got[(y * W + x) as usize], reference(x, y));
            if g.iter().zip(&w).any(|(a, b)| (a - b).abs() > 1e-4) {
                bad.push((x, y, g, w));
            }
        }
    }
    if !bad.is_empty() {
        return Err(format!("shade: {} wrong pixels, first: {:?}", bad.len(), &bad[..bad.len().min(4)]));
    }
    println!("shade PASS: typed payload, SBT data, barycentrics, primitive index, direct callable, miss ({W}x{H})");
    Ok(())
}

fn main() {
    let modules = embedded_modules_from_current_exe().expect("embedded modules");
    let ptx = modules.iter().find_map(|m| m.payload(ArtifactPayloadKind::Ptx)).expect("PTX payload").to_vec();
    std::fs::create_dir_all("target").expect("target dir");
    std::fs::write("target/optix_device_tests.ptx", &ptx).expect("write PTX");
    let mut failed = false;
    for (name, case) in [("shade", case_shade as fn(&[u8]) -> Result<(), String>)] {
        if let Err(msg) = case(&ptx) {
            eprintln!("{name} FAIL: {msg}");
            failed = true;
        }
    }
    if failed {
        std::process::exit(1);
    }
}
