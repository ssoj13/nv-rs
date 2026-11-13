//! Case `local`: OptiX programs whose locals live in local memory (a `.local` depot), which
//! cuda-oxide addresses through 32-bit registers. A raygen fills an array indexed by a runtime
//! value (so it cannot stay in registers), calls a closest-hit that does the same, and writes
//! back elements picked at run time; the host recomputes them.
//!
//! **Why.** OptiX rewrites local memory of its programs onto its own stack; this case pins
//! whether cuda-oxide's local addressing survives that rewrite.

use super::common::{Ctx, Gpu, R, opts, same};
use bytemuck::{Pod, Zeroable};
use cuda_device::{ConstantMemory, constant, cuda_module, kernel};
use optix::{Accel, BuildOptions, GeometryFlags, HitGroup, LinkOptions, ProgramGroup, SbtRecord, Scratch, ShaderBindingTable, TriangleMesh};
use optix_device::trace::{Ray, TraceOptions};
use optix_device::{payload, state, trace};

const N: u32 = 64;

/// Launch parameters (`params_local`).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    /// Two words per launch index.
    pub out: u64,
    /// The GAS.
    pub handle: u64,
}

// SAFETY: plain integers; all-zero is valid.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

#[cuda_module]
mod kernels {
    use super::*;

    #[constant(export_name = "params_local")]
    static PARAMS: ConstantMemory<Params> = ConstantMemory::UNINIT;

    /// `a[i] = i * 3 + seed` over 16 words, then the word at a run-time index.
    #[inline(never)]
    fn pick(seed: u32, at: u32) -> u32 {
        let mut a = [0u32; 16];
        let mut i = 0;
        while i < 16 {
            a[i] = i as u32 * 3 + seed;
            i += 1;
        }
        let p = &mut a[(at % 16) as usize];
        *p ^= 0x55;
        a[((at * 7) % 16) as usize]
    }

    #[kernel]
    pub fn __raygen__local() {
        // SAFETY: raygen; `params_local` is set by the host, `out` holds two words per index.
        unsafe {
            let x = state::launch_index()[0];
            let p = PARAMS.get();
            let mut w = [x];
            trace::trace(p.handle, &Ray::new([0.25, 0.25, -1.0], [0.0, 0.0, 1.0], 0.0, 1.0e16), &TraceOptions::default(), &mut w);
            let out = (p.out as *mut u32).add(2 * x as usize);
            *out = pick(x, x + 1);
            *out.add(1) = w[0];
        }
    }

    #[kernel]
    pub fn __closesthit__local() {
        // SAFETY: closest-hit; the payload is one word.
        unsafe {
            let x = payload::get::<[u32; 1]>()[0];
            payload::set(&[pick(x + 100, x + 5)]);
        }
    }

    #[kernel]
    pub fn __miss__local() {
        // SAFETY: miss; the payload is one word.
        unsafe { payload::set(&[u32::MAX]) }
    }
}

/// CPU twin of the device `pick`.
fn pick(seed: u32, at: u32) -> u32 {
    let mut a: [u32; 16] = std::array::from_fn(|i| i as u32 * 3 + seed);
    a[(at % 16) as usize] ^= 0x55;
    a[((at * 7) % 16) as usize]
}

/// Runs the case.
pub fn run(g: &Gpu) -> R {
    let mut o = opts("params_local", size_of::<Params>())?;
    o.single_gas = true;
    let module = g.module(&Default::default(), &o)?;
    let raygen = ProgramGroup::raygen(&module, "__raygen__local").ctx("raygen")?;
    let miss = ProgramGroup::miss(&module, "__miss__local").ctx("miss")?;
    let hit = ProgramGroup::hit_group(&g.ctx, HitGroup { closest_hit: Some((&module, "__closesthit__local")), ..Default::default() }).ctx("hit group")?;
    let pipeline = g.pipeline(&o, LinkOptions::default(), &[raygen.clone(), miss.clone(), hit.clone()])?;
    let sbt = ShaderBindingTable::new(&g.stream, &SbtRecord::new(&raygen, 0u32).ctx("rg")?, &[SbtRecord::new(&miss, 0u32).ctx("ms")?], &[SbtRecord::new(&hit, 0u32).ctx("hg")?]).ctx("SBT")?;
    let verts = g.up(&[[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]])?;
    let mut scratch = Scratch::new(&g.stream);
    let gas = Accel::triangles(&g.ctx, &g.stream, &mut scratch, &[TriangleMesh { flags: &[GeometryFlags::DISABLE_ANYHIT], ..TriangleMesh::new(&verts) }], BuildOptions::default()).ctx("GAS")?;
    let out = g.zeros::<u32>(2 * N as usize)?;
    let params = Params { out: out.cu_deviceptr(), handle: gas.handle() };
    // SAFETY: `params` matches the device struct; `out` holds two words per index; no SBT data.
    unsafe { g.launch(&pipeline, &sbt, &params, [N, 1, 1]) }?;
    let got = g.down(&out)?;
    let want: Vec<u32> = (0..N).flat_map(|x| [pick(x, x + 1), pick(x + 100, x + 5)]).collect();
    same("local-memory arrays (raygen, closest-hit)", &got, &want, |i| format!("index {} word {}", i / 2, i % 2))
}
