//! Case `footprint`: texture footprints (`optix_device::footprint`) from an OptiX raygen over a
//! 256x256, 9-level mipmapped texture (`cuda-core-ext`).
//!
//! Each raygen thread runs one probe from a host table: `tex_footprint_2d` (non-mipmapped
//! semantics: level 0, one level), `tex_footprint_2d_lod` at integer and fractional levels with
//! `coarse` false and true, and `tex_footprint_2d_grad` with derivatives selecting a level. The
//! host decodes the returned words with the same `Footprint2d` accessors and checks the level,
//! the single-mip-level flag (one level at integer lods, two at fractional ones with linear mip
//! filtering, where `coarse` picks the coarser), and a non-empty mask. `TexInfo::pack` is checked
//! bit by bit against the layout in `optix_device.h`, including its overflow rejection.

use super::common::{Ctx, Gpu, R, ensure, opts, same};
use bytemuck::{Pod, Zeroable};
use cuda_device::{ConstantMemory, constant, cuda_module, kernel};
use cuda_core_ext::{ArrayDesc, ArrayFormat, FilterMode, MipmappedArray, TextureDesc, TextureObject};
use optix::{HitGroup, LinkOptions, ProgramGroup, SbtRecord, ShaderBindingTable};
use optix_device::footprint::{self as fp, Footprint2d, TexInfo};
use optix_device::state;
use std::sync::Arc;

const SIZE: usize = 256;
const LEVELS: u32 = 9;

/// Launch parameters (`params_fp`).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    /// 8 words per probe: the `Footprint2d` (16 bytes), the single-mip-level flag, unused.
    pub out: u64,
    /// `[f32; 8]` per probe: kind (0 plain, 1 lod, 2 grad), u, v, lod, coarse, dpdx = dpdy scale.
    pub probes: u64,
    /// The texture.
    pub tex: u64,
    /// `TexInfo::pack` of the texture.
    pub info: u32,
    /// Padding.
    pub _pad: u32,
}

// SAFETY: plain integers; all-zero is valid.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

#[cuda_module]
mod kernels {
    use super::*;

    #[constant(export_name = "params_fp")]
    static PARAMS: ConstantMemory<Params> = ConstantMemory::UNINIT;

    #[kernel]
    pub fn __raygen__fp() {
        // SAFETY: raygen; `params_fp` holds a live texture, its info word and one probe and eight
        // output words (32 bytes, 16-byte aligned) per launch index. The footprint and the
        // single-mip-level flag go to stack locals first (a `.local` depot: the intrinsic stores
        // through their addresses), then to the global output.
        unsafe {
            let i = state::launch_index()[0] as usize;
            let p = PARAMS.get();
            let q = (p.probes as *const [f32; 8]).add(i).read();
            let o = (p.out as *mut u32).add(8 * i);
            let mut single = 0u32;
            let mut f = Footprint2d { mask: 0, word2: 0, word3: 0 };
            let coarse = q[4] != 0.0;
            if q[0] == 0.0 {
                fp::tex_footprint_2d(p.tex, p.info, q[1], q[2], &mut single, &mut f);
            } else if q[0] == 1.0 {
                fp::tex_footprint_2d_lod(p.tex, p.info, q[1], q[2], q[3], coarse, &mut single, &mut f);
            } else {
                fp::tex_footprint_2d_grad(p.tex, p.info, q[1], q[2], [q[5], 0.0], [0.0, q[5]], coarse, &mut single, &mut f);
            }
            o.cast::<Footprint2d>().write(f);
            *o.add(4) = single;
        }
    }

    #[kernel]
    pub fn __miss__fp() {}
}

/// Runs the case.
pub fn run(g: &Gpu) -> R {
    // TexInfo::pack against the documented layout.
    let ti = TexInfo { levels: LEVELS, log2_tile_width: 6, log2_tile_height: 5, wrap_x: 1, wrap_y: 1, mip_filter: 1, max_anisotropy: 1 };
    let want = (LEVELS << 24) | (6 << 20) | (5 << 16) | (1 << 8) | (1 << 6) | (1 << 5) | 1;
    same("TexInfo::pack", &[ti.pack()], &[Some(want)], |_| String::new())?;
    same("TexInfo::pack rejects overflow", &[TexInfo { levels: 32, ..ti }.pack(), TexInfo { wrap_x: 4, ..ti }.pack(), TexInfo { max_anisotropy: 32, ..ti }.pack()], &[None; 3], |j| format!("[{j}]"))?;
    let info = ti.pack().ok_or("TexInfo::pack")?;

    let mip = Arc::new(MipmappedArray::new(&g.cuda, ArrayDesc::d2(SIZE, SIZE, ArrayFormat::F32, 1), LEVELS).ctx("mipmapped array")?);
    for k in 0..LEVELS {
        let lv = mip.level(k).ctx("level")?;
        let n = lv.desc().width * lv.desc().height;
        lv.upload(&g.stream, bytemuck::cast_slice(&vec![k as f32; n])).ctx("level upload")?;
    }
    let tex = TextureObject::from_mipmapped(&mip, TextureDesc { filter: FilterMode::Linear, mip_filter: FilterMode::Linear, ..TextureDesc::default() }).ctx("texture")?;

    let o = opts("params_fp", size_of::<Params>())?;
    let module = g.module(&Default::default(), &o, &["__raygen__fp", "__miss__fp"])?;
    let raygen = ProgramGroup::raygen(&module, "__raygen__fp").ctx("raygen")?;
    let miss = ProgramGroup::miss(&module, "__miss__fp").ctx("miss")?;
    let hit = ProgramGroup::hit_group(&g.ctx, HitGroup::default()).ctx("empty hit group")?;
    let pipeline = g.pipeline(&o, LinkOptions { max_trace_depth: 0, ..Default::default() }, &[raygen.clone(), miss.clone(), hit.clone()])?;
    let sbt = ShaderBindingTable::new(&g.stream, &SbtRecord::new(&raygen, 0u32).ctx("rg")?, &[SbtRecord::new(&miss, 0u32).ctx("ms")?], &[SbtRecord::new(&hit, 0u32).ctx("hg")?]).ctx("SBT")?;

    // (kind, u, v, lod, coarse, derivative) and the expected (level, single mip level). A coarse
    // query on a single-level footprint is not checked: OptiX returns level 0 for it (observed).
    let mut probes: Vec<([f32; 8], u32, u32)> = vec![([0.0, 0.3, 0.6, 0.0, 0.0, 0.0, 0.0, 0.0], 0, 1)];
    for (lod, coarse, level, single) in [(0.0, 0.0, 0, 1), (2.0, 0.0, 2, 1), (5.0, 0.0, 5, 1), (1.5, 0.0, 1, 0), (1.5, 1.0, 2, 0), (3.25, 1.0, 4, 0)] {
        probes.push(([1.0, 0.41, 0.17, lod, coarse, 0.0, 0.0, 0.0], level, single));
    }
    for (l, coarse, level) in [(2.0f32, 0.0, 2), (4.0, 0.0, 4)] {
        probes.push(([2.0, 0.55, 0.72, 0.0, coarse, 2f32.powf(l) / SIZE as f32, 0.0, 0.0], level, 1));
    }
    let table = g.up(&probes.iter().map(|p| p.0).collect::<Vec<_>>())?;
    let out = g.zeros::<u32>(8 * probes.len())?;
    let params = Params { out: out.cu_deviceptr(), probes: table.cu_deviceptr(), tex: tex.handle(), info, _pad: 0 };
    // SAFETY: `params` matches the device struct; texture, table and output outlive the launch.
    unsafe { g.launch(&pipeline, &sbt, &params, [probes.len() as u32, 1, 1]) }?;
    let got = g.down(&out)?;
    for (i, (q, level, single)) in probes.iter().enumerate() {
        let r = &got[8 * i..8 * i + 8];
        let f = Footprint2d { mask: u64::from(r[0]) | u64::from(r[1]) << 32, word2: r[2], word3: r[3] };
        let label = format!("probe {i} {q:?}");
        // The grad probes sit on exact powers of two: single level, `coarse` irrelevant.
        same(&format!("{label}: (level, single mip level)"), &[f.level(), u32::from(r[4] != 0)], &[*level, *single], |j| format!("[{j}]"))?;
        ensure(f.mask != 0, || format!("{label}: empty mask {f:?}"))?;
    }
    println!("footprint: {} probes (plain, lod fine/coarse, grad), TexInfo::pack", probes.len());
    Ok(())
}
