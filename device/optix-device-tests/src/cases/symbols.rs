//! Case `symbols`: device globals copied with `Pipeline::write_symbol` / `read_symbol` and the
//! device-buffer variants `write_symbol_from` / `read_symbol_into`.
//!
//! [`ODT_SYM_IN`] and [`ODT_SYM_OUT`] are ordinary `static mut` arrays, which cuda-oxide emits as
//! `.visible .global` variables named `__device_global_*` (found and checked in the PTX). The
//! raygen copies `in[x]` to the output
//! buffer and writes `2 * in[x] + 1` to `out[x]`. The host checks the initializer through
//! `read_symbol`, writes the whole array and one word at an offset with `write_symbol`, launches,
//! checks the program read them and that `read_symbol` sees what the program wrote; then the
//! same with device buffers.

use super::common::{Ctx, Gpu, R, ensure, opts, same};
use bytemuck::{Pod, Zeroable};
use cuda_core::DeviceBuffer;
use cuda_device::{ConstantMemory, constant, cuda_module, kernel};
use optix::{HitGroup, LinkOptions, ProgramGroup, SbtRecord, ShaderBindingTable};
use optix_device::state;

const N: usize = 8;

/// Read by the raygen; written by the host. The length is a literal: cuda-oxide rejects a static
/// whose array length is a named constant ("Array length must be a value constant").
#[unsafe(no_mangle)]
pub static mut ODT_SYM_IN: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
/// Written by the raygen; read by the host.
#[unsafe(no_mangle)]
pub static mut ODT_SYM_OUT: [u32; 8] = [0; 8];

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

    #[constant(export_name = "odt_sym_const")]
    static CONSTS: ConstantMemory<[u32; 4]> = ConstantMemory::UNINIT;

    #[kernel]
    pub fn __raygen__sym() {
        // SAFETY: raygen over `N` threads; each touches only its own element of the globals and
        // of `out` (2 N words).
        unsafe {
            let x = state::launch_index()[0] as usize;
            let v = (&raw const ODT_SYM_IN).cast::<u32>().add(x).read_volatile();
            let o = PARAMS.get().out as *mut u32;
            o.add(x).write(v);
            // Thread 0 copies the constant array with constant indices only: a run-time index made
            // cuda-oxide select a 32-bit `.const` pointer through a `brx.idx` jump table.
            if x == 0 {
                let c = CONSTS.get();
                o.add(N).write(c[0]);
                o.add(N + 1).write(c[1]);
                o.add(N + 2).write(c[2]);
                o.add(N + 3).write(c[3]);
            }
            (&raw mut ODT_SYM_OUT).cast::<u32>().add(x).write_volatile(2 * v + 1);
        }
    }

    #[kernel]
    pub fn __miss__sym() {}
}

/// Runs the case.
pub fn run(g: &Gpu) -> R {
    // cuda-oxide names ordinary device globals `__device_global_<crate>_<key>` whatever the
    // Rust name or `#[no_mangle]` says, so the names come from the PTX: the globals the raygen
    // references, told apart by the initializer only `ODT_SYM_IN` has.
    let ptx = String::from_utf8_lossy(&g.ptx).into_owned();
    let body = String::from_utf8(super::common::subset_ptx(&g.ptx, &["__raygen__sym"])?).ctx("subset")?;
    let body = body.split(".entry __raygen__sym").nth(1).unwrap_or_default();
    let mut names: Vec<&str> = body.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).filter(|t| t.starts_with("__device_global_")).collect();
    names.dedup();
    let decl = |n: &str| ptx.lines().find(|l| l.trim_start().starts_with(".visible") && l.contains(&format!(" {n}["))).map(str::to_owned);
    let (mut sym_in, mut sym_out) = (None, None);
    for n in &names {
        let d = decl(n).ok_or_else(|| format!("no declaration of {n} in the PTX"))?;
        ensure(d.contains(".global"), || format!("{n} is not a .global: {d}"))?;
        if d.contains("= {1, 0, 0, 0, 2") {
            sym_in = Some(*n);
        } else {
            sym_out = Some(*n);
        }
    }
    let (Some(sym_in), Some(sym_out)) = (sym_in, sym_out) else {
        return Err(format!("the raygen does not reference both globals: {names:?}"));
    };
    let o = opts("params_sym", size_of::<Params>())?;
    let module = g.module(&Default::default(), &o, &["__raygen__sym", "__miss__sym"])?;
    let raygen = ProgramGroup::raygen(&module, "__raygen__sym").ctx("raygen")?;
    // The SBT needs one miss and one hit-group record even though nothing is traced.
    let miss = ProgramGroup::miss(&module, "__miss__sym").ctx("miss")?;
    let hit = ProgramGroup::hit_group(&g.ctx, HitGroup::default()).ctx("empty hit group")?;
    let pipeline = g.pipeline(&o, LinkOptions { max_trace_depth: 0, ..Default::default() }, &[raygen.clone(), miss.clone(), hit.clone()])?;
    let sbt = ShaderBindingTable::new(&g.stream, &SbtRecord::new(&raygen, 0u32).ctx("rg")?, &[SbtRecord::new(&miss, 0u32).ctx("ms")?], &[SbtRecord::new(&hit, 0u32).ctx("hg")?]).ctx("SBT")?;
    let out = g.zeros::<u32>(2 * N)?;
    // A constant-memory static keeps its `export_name`; OptiX copies into `.const` symbols too.
    pipeline.write_symbol(&g.stream, "odt_sym_const", 0, &[40u32, 41, 42, 43]).ctx("write_symbol into a .const symbol")?;
    let params = Params { out: out.cu_deviceptr() };
    let s = &g.stream;
    let label = |i: usize| format!("[{i}]");

    same("initializer via read_symbol", &pipeline.read_symbol::<[u32; N]>(s, sym_in, 0).ctx("read_symbol")?, &[1, 2, 3, 4, 5, 6, 7, 8], label)?;
    let mut want: [u32; N] = std::array::from_fn(|i| 10 + i as u32);
    pipeline.write_symbol(s, sym_in, 0, &want).ctx("write_symbol")?;
    pipeline.write_symbol(s, sym_in, 4 * 5, &99u32).ctx("write_symbol at an offset")?;
    want[5] = 99;
    // SAFETY: `params` matches the device struct; `out` holds 2 N words; no SBT data; the launch
    // synchronizes.
    unsafe { g.launch(&pipeline, &sbt, &params, [N as u32, 1, 1]) }?;
    let o1 = g.down(&out)?;
    same("program read of write_symbol data", &o1[..N], &want, label)?;
    same("program read of the named .const symbol", &o1[N..], &[40, 41, 42, 43, 0, 0, 0, 0], label)?;
    same("read_symbol of program writes", &pipeline.read_symbol::<[u32; N]>(s, sym_out, 0).ctx("read_symbol")?, &want.map(|v| 2 * v + 1), label)?;
    same("read_symbol at an offset", &[pipeline.read_symbol::<u32>(s, sym_out, 4 * 5).ctx("read_symbol at an offset")?], &[2 * 99 + 1], label)?;

    let src = g.up(&[7u8, 0, 0, 0, 8, 0, 0, 0])?;
    // SAFETY: `src` is alive and unmodified until the synchronization below.
    unsafe { pipeline.write_symbol_from(s, sym_in, 4 * 2, &src) }.ctx("write_symbol_from")?;
    s.synchronize().ctx("sync")?;
    want[2] = 7;
    want[3] = 8;
    // SAFETY: as above.
    unsafe { g.launch(&pipeline, &sbt, &params, [N as u32, 1, 1]) }?;
    same("program read of write_symbol_from data", &g.down(&out)?[..N], &want, label)?;
    let mut dst = DeviceBuffer::<u8>::zeroed(s, 4 * N).ctx("dst")?;
    // SAFETY: `dst` is not accessed before the synchronization below.
    unsafe { pipeline.read_symbol_into(s, sym_out, 0, &mut dst) }.ctx("read_symbol_into")?;
    s.synchronize().ctx("sync")?;
    let bytes = g.down(&dst)?;
    let words: Vec<u32> = bytes.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
    same("read_symbol_into of program writes", &words, &want.map(|v| 2 * v + 1), label)
}
