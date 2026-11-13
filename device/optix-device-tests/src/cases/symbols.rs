//! Case `symbols`: device globals copied with `Pipeline::write_symbol` / `read_symbol` and the
//! device-buffer variants `write_symbol_from` / `read_symbol_into`.
//!
//! [`ODT_SYM_IN`] and [`ODT_SYM_OUT`] are ordinary `static mut` arrays, which cuda-oxide emits as
//! `.visible .global` variables (checked in the PTX). The raygen copies `in[x]` to the output
//! buffer and writes `2 * in[x] + 1` to `out[x]`. The host checks the initializer through
//! `read_symbol`, writes the whole array and one word at an offset with `write_symbol`, launches,
//! checks the program read them and that `read_symbol` sees what the program wrote; then the
//! same with device buffers.

use super::common::{Ctx, Gpu, R, ensure, opts, same};
use bytemuck::{Pod, Zeroable};
use cuda_core::DeviceBuffer;
use cuda_device::{ConstantMemory, constant, cuda_module, kernel};
use optix::{LinkOptions, ProgramGroup, SbtRecord, ShaderBindingTable};
use optix_device::state;

const N: usize = 8;

/// Read by the raygen; written by the host.
#[unsafe(no_mangle)]
pub static mut ODT_SYM_IN: [u32; N] = [1, 2, 3, 4, 5, 6, 7, 8];
/// Written by the raygen; read by the host.
#[unsafe(no_mangle)]
pub static mut ODT_SYM_OUT: [u32; N] = [0; N];

/// Launch parameters (`params_sym`).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    /// One word per thread.
    pub out: u64,
}

// SAFETY: plain integers; all-zero is valid.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

#[cuda_module]
mod kernels {
    use super::*;

    #[constant(export_name = "params_sym")]
    static PARAMS: ConstantMemory<Params> = ConstantMemory::UNINIT;

    #[kernel]
    pub fn __raygen__sym() {
        // SAFETY: raygen over `N` threads; each touches only its own element of the globals and
        // of `out` (N words).
        unsafe {
            let x = state::launch_index()[0] as usize;
            let v = (&raw const ODT_SYM_IN).cast::<u32>().add(x).read_volatile();
            (PARAMS.get().out as *mut u32).add(x).write(v);
            (&raw mut ODT_SYM_OUT).cast::<u32>().add(x).write_volatile(2 * v + 1);
        }
    }
}

/// Runs the case.
pub fn run(g: &Gpu) -> R {
    for sym in ["ODT_SYM_IN", "ODT_SYM_OUT"] {
        let decl = String::from_utf8_lossy(&g.ptx).lines().find(|l| l.contains(sym) && l.trim_start().starts_with(".visible")).map(str::to_owned);
        ensure(decl.as_deref().is_some_and(|d| d.contains(".global")), || format!("{sym} is not a .global in the PTX: {decl:?}"))?;
    }
    let o = opts("params_sym", size_of::<Params>())?;
    let module = g.module(&Default::default(), &o, &["__raygen__sym"])?;
    let raygen = ProgramGroup::raygen(&module, "__raygen__sym").ctx("raygen")?;
    let pipeline = g.pipeline(&o, LinkOptions { max_trace_depth: 0, ..Default::default() }, std::slice::from_ref(&raygen))?;
    let sbt = ShaderBindingTable::new(&g.stream, &SbtRecord::new(&raygen, 0u32).ctx("rg")?, &[] as &[SbtRecord<u32>], &[] as &[SbtRecord<u32>]).ctx("SBT")?;
    let out = g.zeros::<u32>(N)?;
    let params = Params { out: out.cu_deviceptr() };
    let s = &g.stream;
    let label = |i: usize| format!("[{i}]");

    same("initializer via read_symbol", &pipeline.read_symbol::<[u32; N]>(s, "ODT_SYM_IN", 0).ctx("read_symbol")?, &[1, 2, 3, 4, 5, 6, 7, 8], label)?;
    let mut want: [u32; N] = std::array::from_fn(|i| 10 + i as u32);
    pipeline.write_symbol(s, "ODT_SYM_IN", 0, &want).ctx("write_symbol")?;
    pipeline.write_symbol(s, "ODT_SYM_IN", 4 * 5, &99u32).ctx("write_symbol at an offset")?;
    want[5] = 99;
    // SAFETY: `params` matches the device struct; `out` holds N words; no SBT data; the launch
    // synchronizes.
    unsafe { g.launch(&pipeline, &sbt, &params, [N as u32, 1, 1]) }?;
    same("program read of write_symbol data", &g.down(&out)?, &want, label)?;
    same("read_symbol of program writes", &pipeline.read_symbol::<[u32; N]>(s, "ODT_SYM_OUT", 0).ctx("read_symbol")?, &want.map(|v| 2 * v + 1), label)?;
    same("read_symbol at an offset", &[pipeline.read_symbol::<u32>(s, "ODT_SYM_OUT", 4 * 5).ctx("read_symbol at an offset")?], &[2 * 99 + 1], label)?;

    let src = g.up(&[7u8, 0, 0, 0, 8, 0, 0, 0])?;
    // SAFETY: `src` is alive and unmodified until the synchronization below.
    unsafe { pipeline.write_symbol_from(s, "ODT_SYM_IN", 4 * 2, &src) }.ctx("write_symbol_from")?;
    s.synchronize().ctx("sync")?;
    want[2] = 7;
    want[3] = 8;
    // SAFETY: as above.
    unsafe { g.launch(&pipeline, &sbt, &params, [N as u32, 1, 1]) }?;
    same("program read of write_symbol_from data", &g.down(&out)?, &want, label)?;
    let mut dst = DeviceBuffer::<u8>::zeroed(s, 4 * N).ctx("dst")?;
    // SAFETY: `dst` is not accessed before the synchronization below.
    unsafe { pipeline.read_symbol_into(s, "ODT_SYM_OUT", 0, &mut dst) }.ctx("read_symbol_into")?;
    s.synchronize().ctx("sync")?;
    let bytes = g.down(&dst)?;
    let words: Vec<u32> = bytes.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
    same("read_symbol_into of program writes", &words, &want.map(|v| 2 * v + 1), label)
}
