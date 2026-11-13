//! Case `shade`: a 32x24 orthographic launch over a quad of two triangles. The raygen traces a
//! typed payload; the closest-hit reads its SBT data, the triangle barycentrics and the primitive
//! index and calls a direct callable (twice: distinct call-site prototypes); the miss program
//! writes the background from its SBT data. Every pixel is compared with a CPU reference.

use super::common::{Ctx, Gpu, R, close, opts};
use bytemuck::{Pod, Zeroable};
use cuda_device::{ConstantMemory, constant, cuda_module, device, kernel};
use optix::{Accel, BuildOptions, GeometryFlags, HitGroup, Indices, LinkOptions, ProgramGroup, SbtRecord, Scratch, ShaderBindingTable, TriangleMesh};
use optix_device::trace::{Ray, TraceOptions};
use optix_device::{payload, state, trace};

const W: u32 = 32;
const H: u32 = 24;

/// Launch parameters (`params_shade` in constant memory).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    /// `[f32; 4]` per pixel: rgb + primitive index.
    pub out: u64,
    /// The GAS.
    pub handle: u64,
    /// Launch width.
    pub width: u32,
    /// Launch height.
    pub height: u32,
}

// SAFETY: plain integers; all-zero is valid.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

/// The payload: shaded colour and the primitive hit (-1 on a miss).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Default)]
pub struct Shade {
    /// Colour.
    pub rgb: [f32; 3],
    /// Primitive index, -1 on a miss.
    pub prim: i32,
}

/// SBT data of the miss and hit-group records.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Color {
    /// Colour.
    pub rgb: [f32; 3],
    /// Padding to 16 bytes.
    pub _pad: f32,
}

/// The direct callable: doubles its argument.
#[device]
pub fn __direct_callable__double(x: f32) -> f32 {
    2.0 * x
}

#[cuda_module]
mod kernels {
    use super::*;

    #[constant(export_name = "params_shade")]
    static PARAMS: ConstantMemory<Params> = ConstantMemory::UNINIT;

    #[kernel]
    pub fn __raygen__render() {
        // SAFETY: OptiX raygen program; the host sets `params_shade` and sizes `out` to the launch.
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
            // Two call sites in one program: the PTX prototype labels must not collide.
            let k = optix_device::direct_call!(0, fn(f32) -> f32, (u));
            let k2 = optix_device::direct_call!(0, fn(f32) -> f32, (v));
            let rgb = [c.rgb[0] * k, c.rgb[1] * k2 * 0.5, c.rgb[2] * (1.0 - u - v)];
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

/// The programs this case links.
const ENTRIES: &[&str] = &["__raygen__render", "__miss__background", "__closesthit__shade", "__direct_callable__double"];

/// Runs the case.
pub fn run(g: &Gpu) -> R {
    let mut o = opts("params_shade", size_of::<Params>())?;
    o.single_gas = true;
    let module = g.module(&Default::default(), &o, ENTRIES)?;
    let raygen = ProgramGroup::raygen(&module, "__raygen__render").ctx("raygen")?;
    let miss = ProgramGroup::miss(&module, "__miss__background").ctx("miss")?;
    let hit = ProgramGroup::hit_group(&g.ctx, HitGroup { closest_hit: Some((&module, "__closesthit__shade")), ..Default::default() }).ctx("hit group")?;
    let callable = ProgramGroup::callables(&g.ctx, Some((&module, "__direct_callable__double")), None).ctx("callable")?;
    let link = LinkOptions { max_dc_depth_from_state: 1, ..Default::default() };
    let pipeline = g.pipeline(&o, link, &[raygen.clone(), miss.clone(), hit.clone(), callable.clone()])?;
    let sbt = ShaderBindingTable::new(
        &g.stream,
        &SbtRecord::new(&raygen, 0u32).ctx("raygen record")?,
        &[SbtRecord::new(&miss, Color { rgb: BG, _pad: 0.0 }).ctx("miss record")?],
        &[SbtRecord::new(&hit, Color { rgb: HIT, _pad: 0.0 }).ctx("hit record")?],
    )
    .and_then(|s| s.with_callables(&g.stream, &[SbtRecord::new(&callable, 0u32)?]))
    .ctx("SBT")?;

    let verts = g.up(&QUAD)?;
    let index = g.up(&TRIS)?;
    let mut scratch = Scratch::new(&g.stream);
    let mesh = TriangleMesh { indices: Some(Indices::U32(&index)), flags: &[GeometryFlags::DISABLE_ANYHIT], ..TriangleMesh::new(&verts) };
    let gas = Accel::triangles(&g.ctx, &g.stream, &mut scratch, &[mesh], BuildOptions::default()).ctx("GAS")?;

    let out = g.zeros::<[f32; 4]>((W * H) as usize)?;
    let params = Params { out: out.cu_deviceptr(), handle: gas.handle(), width: W, height: H };
    // SAFETY: `params` matches the device struct; records match what each program reads; `out`
    // holds one value per pixel; everything outlives the launch (it synchronizes).
    unsafe { g.launch(&pipeline, &sbt, &params, [W, H, 1]) }?;
    let got: Vec<f32> = g.down(&out)?.into_iter().flatten().collect();
    let want: Vec<f32> = (0..H).flat_map(|y| (0..W).map(move |x| (x, y))).flat_map(|(x, y)| reference(x, y)).collect();
    close("pixels (rgb, prim)", &got, &want, 1e-4, |i| format!("pixel ({}, {}) ch {}", (i / 4) as u32 % W, (i / 4) as u32 / W, i % 4))
}
