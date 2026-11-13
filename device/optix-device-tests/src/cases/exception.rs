//! Case `exception`: user and built-in exceptions through `optix_device::exception`.
//!
//! The pipeline enables `OPTIX_EXCEPTION_FLAG_USER | OPTIX_EXCEPTION_FLAG_TRACE_DEPTH` and links
//! with `max_trace_depth = 1`. Launch row 0: each raygen thread throws a user exception with code
//! `100 + x` and 0, 2 or 8 detail words (by `x % 3`); row 1: each thread traces into a triangle
//! whose closest-hit traces again, exceeding the trace depth. The exception program writes the
//! code and the detail words it reads; the host checks every thread: user code and details, the
//! built-in `OPTIX_EXCEPTION_CODE_TRACE_DEPTH_EXCEEDED`, and that no thread ran past its throw.

use super::common::{Ctx, Gpu, R, opts, same};
use bytemuck::{Pod, Zeroable};
use cuda_device::{ConstantMemory, constant, cuda_module, kernel};
use optix::{Accel, BuildOptions, GeometryFlags, HitGroup, LinkOptions, ProgramGroup, SbtRecord, Scratch, ShaderBindingTable, TriangleMesh, sys};
use optix_device::trace::{Ray, TraceOptions};
use optix_device::{exception, state, trace};

const N: u32 = 24;
/// Words per record: marker, code, 8 details.
const REC: usize = 10;

/// Launch parameters (`params_exc`).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    /// `REC` words per launch index.
    pub out: u64,
    /// The GAS.
    pub handle: u64,
}

// SAFETY: plain integers; all-zero is valid.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

/// Detail word `k` of thread `x`'s user exception.
const fn detail(x: u32, k: u32) -> u32 {
    x * 1000 + k * 7 + 1
}

#[cuda_module]
mod kernels {
    use super::*;

    #[constant(export_name = "params_exc")]
    static PARAMS: ConstantMemory<Params> = ConstantMemory::UNINIT;

    /// This launch index's record.
    ///
    /// # Safety
    ///
    /// Programs of this case's launch.
    #[inline(always)]
    unsafe fn rec() -> *mut u32 {
        // SAFETY: forwarded contract; `out` holds `REC` words per launch index.
        unsafe {
            let [x, y, _] = state::launch_index();
            (PARAMS.get().out as *mut u32).add(((y * N + x) as usize) * REC)
        }
    }

    #[kernel]
    pub fn __raygen__exc() {
        // SAFETY: raygen of a pipeline with user and trace-depth exceptions.
        unsafe {
            let [x, y, _] = state::launch_index();
            *rec() = 1;
            if y == 0 {
                let code = 100 + x as i32;
                match x % 3 {
                    0 => exception::throw(code, []),
                    1 => exception::throw(code, [detail(x, 0), detail(x, 1)]),
                    _ => exception::throw(code, [detail(x, 0), detail(x, 1), detail(x, 2), detail(x, 3), detail(x, 4), detail(x, 5), detail(x, 6), detail(x, 7)]),
                }
            } else {
                trace::trace_no_payload(PARAMS.get().handle, &Ray::new([0.25, 0.25, -1.0], [0.0, 0.0, 1.0], 0.0, 1.0e16), &TraceOptions::default());
            }
            // Only reached when no exception fired.
            *rec() = 3;
        }
    }

    #[kernel]
    pub fn __closesthit__exc() {
        // SAFETY: closest-hit; tracing again exceeds `max_trace_depth = 1`.
        unsafe { trace::trace_no_payload(PARAMS.get().handle, &Ray::new([0.25, 0.25, -1.0], [0.0, 0.0, 1.0], 0.0, 1.0e16), &TraceOptions::default()) }
    }

    #[kernel]
    pub fn __miss__exc() {}

    #[kernel]
    pub fn __exception__exc() {
        // SAFETY: exception program of this case.
        unsafe {
            let r = rec();
            *r = 2;
            *r.add(1) = exception::code() as u32;
            *r.add(2) = exception::detail::<0>();
            *r.add(3) = exception::detail::<1>();
            *r.add(4) = exception::detail::<2>();
            *r.add(5) = exception::detail::<3>();
            *r.add(6) = exception::detail::<4>();
            *r.add(7) = exception::detail::<5>();
            *r.add(8) = exception::detail::<6>();
            *r.add(9) = exception::detail::<7>();
        }
    }
}

/// Runs the case.
pub fn run(g: &Gpu) -> R {
    let mut o = opts("params_exc", size_of::<Params>())?;
    o.single_gas = true;
    o.exceptions = (sys::OptixExceptionFlags_OPTIX_EXCEPTION_FLAG_USER | sys::OptixExceptionFlags_OPTIX_EXCEPTION_FLAG_TRACE_DEPTH) as u32;
    let module = g.module(&Default::default(), &o)?;
    let raygen = ProgramGroup::raygen(&module, "__raygen__exc").ctx("raygen")?;
    let miss = ProgramGroup::miss(&module, "__miss__exc").ctx("miss")?;
    let hit = ProgramGroup::hit_group(&g.ctx, HitGroup { closest_hit: Some((&module, "__closesthit__exc")), ..Default::default() }).ctx("hit group")?;
    let exc = ProgramGroup::exception(&module, "__exception__exc").ctx("exception")?;
    let pipeline = g.pipeline(&o, LinkOptions { max_trace_depth: 1, ..Default::default() }, &[raygen.clone(), miss.clone(), hit.clone(), exc.clone()])?;
    let sbt = ShaderBindingTable::new(&g.stream, &SbtRecord::new(&raygen, 0u32).ctx("rg")?, &[SbtRecord::new(&miss, 0u32).ctx("ms")?], &[SbtRecord::new(&hit, 0u32).ctx("hg")?])
        .and_then(|s| s.with_exception(&g.stream, &SbtRecord::new(&exc, 0u32)?))
        .ctx("SBT")?;
    let verts = g.up(&[[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]])?;
    let mut scratch = Scratch::new(&g.stream);
    let gas = Accel::triangles(&g.ctx, &g.stream, &mut scratch, &[TriangleMesh { flags: &[GeometryFlags::DISABLE_ANYHIT], ..TriangleMesh::new(&verts) }], BuildOptions::default()).ctx("GAS")?;

    let out = g.zeros::<u32>(2 * N as usize * REC)?;
    let params = Params { out: out.cu_deviceptr(), handle: gas.handle() };
    // SAFETY: `params` matches the device struct; `out` holds `REC` words per launch index; no
    // SBT data is read; the launch synchronizes.
    unsafe { g.launch(&pipeline, &sbt, &params, [N, 2, 1]) }?;
    let got = g.down(&out)?;
    let depth = sys::OptixExceptionCodes_OPTIX_EXCEPTION_CODE_TRACE_DEPTH_EXCEEDED as u32;
    for y in 0..2 {
        for x in 0..N {
            let r = &got[((y * N + x) as usize) * REC..][..REC];
            if y == 0 {
                let n = [0, 2, 8][(x % 3) as usize];
                let mut want = vec![2, 100 + x];
                want.extend((0..n).map(|k| detail(x, k)));
                same(&format!("user exception of thread {x} (marker, code, details)"), &r[..want.len()], &want, |j| format!("[{j}]"))?;
            } else {
                same(&format!("trace-depth exception of thread {x} (marker, code)"), &r[..2], &[2, depth], |j| format!("[{j}]"))?;
            }
        }
    }
    println!("exception: {N} user exceptions with 0/2/8 details, {N} trace-depth exceptions");
    Ok(())
}
