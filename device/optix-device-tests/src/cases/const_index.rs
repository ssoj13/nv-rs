//! Case `const_index`: run-time indices into arrays read by value from constant memory.
//!
//! **Why.** Built with 32-bit (short) pointers, a raygen reading `CONSTS.get()[x]` crashed (then a
//! `brx.idx` jump table selecting a 32-bit `.const` address). This program, built that way,
//! copies both arrays into a `.local` depot addressed through 32-bit registers and faults with
//! `CUDA_ERROR_ILLEGAL_ADDRESS`; with 64-bit pointers in every address space
//! (`CUDA_OXIDE_NO_SHORT_PTR`, see case `ptr64`) it runs. Each of [`THREADS`] threads reads
//! `TABLE[x % 4]` (a `ConstantMemory<[u32; 4]>` written by the host by name) and
//! `PARAMS.get().scale[x % 8]` (an array field of the launch parameters), and writes
//! `TABLE[x % 4] * scale[x % 8] + x`.

use super::common::{Ctx, Gpu, R, opts, same};
use bytemuck::{Pod, Zeroable};
use cuda_device::{ConstantMemory, constant, cuda_module, kernel};
use optix::{HitGroup, LinkOptions, ProgramGroup, SbtRecord, ShaderBindingTable};
use optix_device::state;

/// Threads per launch.
const THREADS: usize = 16;
/// Entries of [`Params::scale`].
const SCALES: usize = 8;

/// Launch parameters (`params_ci`).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    /// One word per thread.
    pub out: u64,
    /// Indexed at run time after a by-value read of the whole struct.
    pub scale: [u32; SCALES],
}

// SAFETY: plain integers; all-zero is valid.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

#[cuda_module]
mod kernels {
    use super::*;

    #[constant(export_name = "params_ci")]
    static PARAMS: ConstantMemory<Params> = ConstantMemory::UNINIT;

    #[constant(export_name = "odt_ci_table")]
    static TABLE: ConstantMemory<[u32; 4]> = ConstantMemory::UNINIT;

    #[kernel]
    pub fn __raygen__ci() {
        // SAFETY: raygen over `THREADS` threads; each writes only its own word of `out`.
        unsafe {
            let x = state::launch_index()[0] as usize;
            let table = TABLE.get();
            let p = PARAMS.get();
            let v = table[x % 4] * p.scale[x % SCALES] + x as u32;
            (p.out as *mut u32).add(x).write(v);
        }
    }

    #[kernel]
    pub fn __miss__ci() {}
}

/// Runs the case.
pub fn run(g: &Gpu) -> R {
    let o = opts("params_ci", size_of::<Params>())?;
    let module = g.module(&Default::default(), &o, &["__raygen__ci", "__miss__ci"])?;
    let raygen = ProgramGroup::raygen(&module, "__raygen__ci").ctx("raygen")?;
    let miss = ProgramGroup::miss(&module, "__miss__ci").ctx("miss")?;
    let hit = ProgramGroup::hit_group(&g.ctx, HitGroup::default()).ctx("empty hit group")?;
    let pipeline = g.pipeline(&o, LinkOptions { max_trace_depth: 0, ..Default::default() }, &[raygen.clone(), miss.clone(), hit.clone()])?;
    let sbt = ShaderBindingTable::new(&g.stream, &SbtRecord::new(&raygen, 0u32).ctx("rg")?, &[SbtRecord::new(&miss, 0u32).ctx("ms")?], &[SbtRecord::new(&hit, 0u32).ctx("hg")?]).ctx("SBT")?;

    let table = [3u32, 5, 7, 11];
    pipeline.write_symbol(&g.stream, "odt_ci_table", 0, &table).ctx("write_symbol odt_ci_table")?;
    let out = g.zeros::<u32>(THREADS)?;
    let scale: [u32; SCALES] = std::array::from_fn(|i| 10 + i as u32);
    let params = Params { out: out.cu_deviceptr(), scale };
    // SAFETY: `params` matches the device struct; `out` holds `THREADS` words; no SBT data; the
    // launch synchronizes.
    unsafe { g.launch(&pipeline, &sbt, &params, [THREADS as u32, 1, 1]) }?;
    let want: Vec<u32> = (0..THREADS).map(|x| table[x % 4] * scale[x % SCALES] + x as u32).collect();
    same("run-time indexed constant arrays", &g.down(&out)?, &want, |i| format!("[{i}]"))?;

    let body = String::from_utf8(super::common::subset_ptx(&g.ptx, &["__raygen__ci"])?).ctx("subset")?;
    println!(
        "const_index: {THREADS} threads, raygen uses brx.idx: {}, .local depot: {}",
        body.contains("brx.idx"),
        body.contains("__local_depot")
    );
    Ok(())
}
