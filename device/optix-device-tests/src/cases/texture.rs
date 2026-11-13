//! Case `texture`: `cuda-tex-device` fetches and surface accesses from plain CUDA kernels
//! (no OptiX), launched with cuda-core on objects made by `cuda-ext`.
//!
//! - `tex2d` point and linear on an 8x8 RGBA f32 array (unnormalized coordinates, clamp) and
//!   `tex2d_gather` (channel 0, CUDA's texel order) against a CPU bilinear reference;
//! - `tex2d_lod` / `tex2d_grad` on a 5-level mipmapped array whose level k holds k, with linear
//!   mip filtering: the fetch returns the level of detail;
//! - `tex2d_layered` (3 layers) and `tex_cube` (6 faces, one direction per face);
//! - an RGBA8 array read as integers (`read_as_integer`) and as normalized floats;
//! - `tex1d_fetch` on a linear buffer (`TextureObject::from_linear`);
//! - surfaces of every element width (1, 2, 4, 8, 16 bytes): writes in every boundary mode,
//!   dropped out-of-range `Zero` writes, reads in `Trap` / `Clamp` / `Zero` (in and out of range),
//!   plus the array contents downloaded;
//! - sparse residency (case `sparse`): a sparse 2D array with two of four tiles mapped from a
//!   `TilePool`; `tex2d_sparse`, `tex2d_lod_sparse` and `tex2d_gather_sparse` report resident per
//!   tile and fetch zeros from unmapped tiles.
//!
//! The kernels are compiled out of the binary's PTX alone ([`super::common::subset_ptx`]): the
//! rest of it calls `_optix_*` intrinsics `cuModuleLoadData` cannot resolve.

use super::common::{Ctx, Gpu, R, close, same, subset_ptx};
use cuda_core::{Function, Module, launch_kernel_on_stream};
use cuda_device::{cuda_module, kernel, thread};
use cuda_ext::{Array, ArrayDesc, ArrayFormat, FilterMode, MipmappedArray, SurfaceObject, TextureDesc, TextureObject, TilePool};
use cuda_tex_device as tx;
use std::ffi::c_void;
use std::sync::Arc;

/// Builds a surface element from a seed word (device and host agree on it).
pub trait Mk: Copy {
    /// The element for seed `s`.
    fn mk(s: u32) -> Self;
}

impl Mk for u8 {
    fn mk(s: u32) -> Self {
        s as u8
    }
}
impl Mk for u16 {
    fn mk(s: u32) -> Self {
        s as u16
    }
}
impl Mk for u32 {
    fn mk(s: u32) -> Self {
        s
    }
}
impl Mk for [u32; 2] {
    fn mk(s: u32) -> Self {
        [s, s ^ 0xA5A5_0000]
    }
}
impl Mk for [u32; 4] {
    fn mk(s: u32) -> Self {
        [s, s + 1, s + 2, s ^ 0xFFFF_0000]
    }
}

/// Surface array size in elements, and the launch grid (two columns and rows out of range).
const SW: i32 = 8;
const SH: i32 = 4;
const GW: i32 = 10;
const GH: i32 = 6;

/// Seed of in-range element (x, y).
const fn seed(x: i32, y: i32) -> u32 {
    0x1234 + (y * 16 + x) as u32 * 0x0101
}

#[cuda_module]
mod tex_kernels {
    use super::*;

    #[kernel]
    pub fn odt_tex2d(tp: u64, tl: u64, coords: u64, out: u64, n: u32) {
        let i = thread::index_1d().get();
        if i < n as usize {
            // SAFETY: live texture objects over a 2D RGBA f32 array; `coords` holds n [f32; 2],
            // `out` 12 f32 per sample.
            unsafe {
                let [x, y] = (coords as *const [f32; 2]).add(i).read();
                let o = (out as *mut [f32; 4]).add(3 * i);
                o.write(tx::tex2d::<[f32; 4]>(tp, x, y));
                o.add(1).write(tx::tex2d::<[f32; 4]>(tl, x, y));
                o.add(2).write(tx::tex2d_gather::<[f32; 4], 0>(tp, x, y));
            }
        }
    }

    #[kernel]
    pub fn odt_tex_mip(tm: u64, lods: u64, out: u64, n: u32) {
        let i = thread::index_1d().get();
        if i < n as usize {
            // SAFETY: a live texture over a 16x16 mipmapped array; `lods` holds n f32, `out` 2 per sample.
            unsafe {
                let l = (lods as *const f32).add(i).read();
                let d = cuda_device::float::ex2_approx_f32(l) / 16.0;
                let o = (out as *mut f32).add(2 * i);
                o.write(tx::tex2d_lod::<f32>(tm, 0.5, 0.5, l));
                o.add(1).write(tx::tex2d_grad::<f32>(tm, 0.5, 0.5, [d, 0.0], [0.0, d]));
            }
        }
    }

    #[kernel]
    pub fn odt_tex_misc(layered: u64, cube: u64, int: u64, norm: u64, lin: u64, out: u64) {
        let i = thread::index_1d().get();
        if i < 48 {
            // SAFETY: live texture objects as the host built them; `out` holds 12 words per thread.
            unsafe {
                let o = (out as *mut u32).add(12 * i);
                let (x, y) = ((i % 4) as f32 + 0.5, ((i / 4) % 4) as f32 + 0.5);
                o.write(tx::tex2d_layered::<f32>(layered, x, y, (i / 16) as u32).to_bits());
                let s = if i % 2 == 0 { 1.0 } else { -1.0 };
                let dir = match (i / 2) % 3 {
                    0 => [s, 0.25, -0.125],
                    1 => [0.125, s, 0.25],
                    _ => [-0.25, 0.125, s],
                };
                o.add(1).write(tx::tex_cube::<f32>(cube, dir[0], dir[1], dir[2]).to_bits());
                let xi = (i % 4) as f32 + 0.5;
                let yi = ((i / 4) % 4) as f32 + 0.5;
                let v = tx::tex2d::<[u32; 4]>(int, xi, yi);
                o.add(2).write(v[0]);
                o.add(3).write(v[1]);
                o.add(4).write(v[2]);
                o.add(5).write(v[3]);
                let f = tx::tex2d::<[f32; 4]>(norm, xi, yi);
                o.add(6).write(f[0].to_bits());
                o.add(7).write(f[1].to_bits());
                o.add(8).write(f[2].to_bits());
                o.add(9).write(f[3].to_bits());
                o.add(10).write(tx::tex1d_fetch::<f32>(lin, i as i32).to_bits());
            }
        }
    }

    /// Phase 0 writes every in-range element (boundary mode by column) and drops `Zero`
    /// writes out of range; phase 1 reads every grid cell in `Zero` and `Clamp` mode (and `Trap`
    /// in range) into 3 x 16-byte slots.
    ///
    /// # Safety
    ///
    /// `surf` is a live 8x4 surface of `T`; `out` holds `GW * GH * 48` bytes.
    #[inline(always)]
    unsafe fn surf_body<T: tx::SurfElem + Mk>(i: usize, surf: u64, out: u64, phase: u32) {
        if i >= (GW * GH) as usize {
            return;
        }
        let (x, y) = ((i as i32) % GW, (i as i32) / GW);
        let w = size_of::<T>() as i32;
        let inside = x < SW && y < SH;
        // SAFETY: forwarded contract; `Trap` is only used in range.
        unsafe {
            if phase == 0 {
                if inside {
                    let mode = match x % 3 {
                        0 => tx::Boundary::Trap,
                        1 => tx::Boundary::Clamp,
                        _ => tx::Boundary::Zero,
                    };
                    tx::surf2d_write_mode(T::mk(seed(x, y)), surf, x * w, y, mode);
                } else {
                    tx::surf2d_write_mode(T::mk(0xEEEE_EEEE), surf, x * w, y, tx::Boundary::Zero);
                }
            } else {
                let o = (out as *mut u8).add(48 * i);
                o.cast::<T>().write_unaligned(tx::surf2d_read_mode::<T>(surf, x * w, y, tx::Boundary::Zero));
                o.add(16).cast::<T>().write_unaligned(tx::surf2d_read_mode::<T>(surf, x * w, y, tx::Boundary::Clamp));
                if inside {
                    o.add(32).cast::<T>().write_unaligned(tx::surf2d_read_mode::<T>(surf, x * w, y, tx::Boundary::Trap));
                }
            }
        }
    }

    #[kernel]
    pub fn odt_surf_w1(surf: u64, out: u64, phase: u32) {
        let i = thread::index_1d().get();
        // SAFETY: an 8x4 u8 surface; `out` sized by the host.
        unsafe { surf_body::<u8>(i, surf, out, phase) }
    }

    #[kernel]
    pub fn odt_surf_w2(surf: u64, out: u64, phase: u32) {
        let i = thread::index_1d().get();
        // SAFETY: an 8x4 u16 surface; `out` sized by the host.
        unsafe { surf_body::<u16>(i, surf, out, phase) }
    }

    #[kernel]
    pub fn odt_surf_w4(surf: u64, out: u64, phase: u32) {
        let i = thread::index_1d().get();
        // SAFETY: an 8x4 u32 surface; `out` sized by the host.
        unsafe { surf_body::<u32>(i, surf, out, phase) }
    }

    #[kernel]
    pub fn odt_surf_w8(surf: u64, out: u64, phase: u32) {
        let i = thread::index_1d().get();
        // SAFETY: an 8x4 2 x u32 surface; `out` sized by the host.
        unsafe { surf_body::<[u32; 2]>(i, surf, out, phase) }
    }

    #[kernel]
    pub fn odt_surf_w16(surf: u64, out: u64, phase: u32) {
        let i = thread::index_1d().get();
        // SAFETY: an 8x4 4 x u32 surface; `out` sized by the host.
        unsafe { surf_body::<[u32; 4]>(i, surf, out, phase) }
    }

    #[kernel]
    pub fn odt_tex_sparse(t: u64, coords: u64, out: u64, n: u32) {
        let i = thread::index_1d().get();
        if i < n as usize {
            // SAFETY: a live texture over a sparse 2D f32 array; `coords` holds n [f32; 2], `out`
            // 6 words per sample.
            unsafe {
                let [x, y] = (coords as *const [f32; 2]).add(i).read();
                let o = (out as *mut u32).add(6 * i);
                let (v, r) = tx::tex2d_sparse::<f32>(t, x, y);
                o.write(v.to_bits());
                o.add(1).write(u32::from(r));
                let (v, r) = tx::tex2d_lod_sparse::<f32>(t, x, y, 0.0);
                o.add(2).write(v.to_bits());
                o.add(3).write(u32::from(r));
                let (v, r) = tx::tex2d_gather_sparse::<[f32; 4], 0>(t, x, y);
                o.add(4).write(v[0].to_bits());
                o.add(5).write(u32::from(r));
            }
        }
    }
}

/// The kernels `names` of the binary's PTX as one CUDA module.
fn load(g: &Gpu, names: &[&str]) -> R<Arc<Module>> {
    let ptx = subset_ptx(&g.ptx, names)?;
    g.cuda.load_module_from_image(&ptx).ctx("cuModuleLoadData")
}

/// Launches `f` over `n` threads (64 per block) and waits.
///
/// # Safety
///
/// `args` point to values matching the kernel's parameters; every pointer and handle in them
/// stays valid until the launch completes (it synchronizes).
unsafe fn run_kernel(g: &Gpu, f: &Function, n: u32, args: &mut [*mut c_void]) -> R {
    // SAFETY: forwarded contract.
    unsafe { launch_kernel_on_stream(f, (n.div_ceil(64), 1, 1), (64, 1, 1), 0, &g.stream, args) }.ctx("launch")?;
    g.stream.synchronize().ctx("synchronize after kernel")
}

/// A pointer to `v` as a kernel argument.
fn arg<T>(v: &mut T) -> *mut c_void {
    (v as *mut T).cast()
}

fn bytes<T: bytemuck::Pod>(v: &[T]) -> Vec<u8> {
    bytemuck::cast_slice(v).to_vec()
}

fn point_unnorm() -> TextureDesc {
    TextureDesc { filter: FilterMode::Point, mip_filter: FilterMode::Point, normalized_coords: false, ..TextureDesc::default() }
}

/// Texel (x, y) of the 8x8 RGBA test image.
fn texel(x: i32, y: i32) -> [f32; 4] {
    let (x, y) = (x.clamp(0, 7), y.clamp(0, 7));
    [(x + 8 * y) as f32 / 64.0, x as f32 / 8.0, y as f32 / 8.0, 1.0]
}

/// Runs the texture case (everything but sparse residency).
pub fn run(g: &Gpu) -> R {
    let names = ["odt_tex2d", "odt_tex_mip", "odt_tex_misc", "odt_surf_w1", "odt_surf_w2", "odt_surf_w4", "odt_surf_w8", "odt_surf_w16"];
    let m = load(g, &names)?;
    let s = &g.stream;

    // 2D point / linear / gather.
    let img: Vec<f32> = (0..64).flat_map(|i| texel(i % 8, i / 8)).collect();
    let arr = Arc::new(Array::new(&g.cuda, ArrayDesc::d2(8, 8, ArrayFormat::F32, 4)).ctx("array")?);
    arr.upload(s, &bytes(&img)).ctx("upload")?;
    let tp = TextureObject::from_array(&arr, point_unnorm()).ctx("point texture")?;
    let tl = TextureObject::from_array(&arr, TextureDesc { filter: FilterMode::Linear, ..point_unnorm() }).ctx("linear texture")?;
    let coords: Vec<[f32; 2]> = (0..40).map(|i| [((i * 37) % 80) as f32 / 10.0 + 0.03, ((i * 53) % 80) as f32 / 10.0 + 0.07]).collect();
    let cd = g.up(&coords)?;
    let out = g.zeros::<[f32; 4]>(3 * coords.len())?;
    let (mut a0, mut a1, mut a2, mut a3, mut a4) = (tp.handle(), tl.handle(), cd.cu_deviceptr(), out.cu_deviceptr(), coords.len() as u32);
    let f = m.load_function("odt_tex2d").ctx("function")?;
    // SAFETY: arguments match `odt_tex2d`; every buffer and texture outlives the launch.
    unsafe { run_kernel(g, &f, coords.len() as u32, &mut [arg(&mut a0), arg(&mut a1), arg(&mut a2), arg(&mut a3), arg(&mut a4)]) }?;
    let got = g.down(&out)?;
    for (i, [x, y]) in coords.iter().enumerate() {
        let (xi, yi) = (x.floor() as i32, y.floor() as i32);
        close(&format!("tex2d point at ({x}, {y})"), &got[3 * i], &texel(xi, yi), 0.0, |c| format!("ch {c}"))?;
        let (bx, by) = (x - 0.5, y - 0.5);
        let (i0, j0) = (bx.floor() as i32, by.floor() as i32);
        let (fa, fb) = (bx - bx.floor(), by - by.floor());
        let lin: Vec<f32> = (0..4)
            .map(|c| {
                let t = |dx, dy| texel(i0 + dx, j0 + dy)[c];
                (1.0 - fa) * (1.0 - fb) * t(0, 0) + fa * (1.0 - fb) * t(1, 0) + (1.0 - fa) * fb * t(0, 1) + fa * fb * t(1, 1)
            })
            .collect();
        // Hardware weights carry 8 fractional bits.
        close(&format!("tex2d linear at ({x}, {y})"), &got[3 * i + 1], &lin, 2.0 / 256.0, |c| format!("ch {c}"))?;
        let gather = [texel(i0, j0 + 1)[0], texel(i0 + 1, j0 + 1)[0], texel(i0 + 1, j0)[0], texel(i0, j0)[0]];
        close(&format!("tex2d_gather r at ({x}, {y})"), &got[3 * i + 2], &gather, 0.0, |c| format!("texel {c}"))?;
    }

    // Mipmaps: level k holds k everywhere.
    let mip = Arc::new(MipmappedArray::new(&g.cuda, ArrayDesc::d2(16, 16, ArrayFormat::F32, 1), 5).ctx("mipmapped array")?);
    for k in 0..5u32 {
        let lv = mip.level(k).ctx("level")?;
        let n = lv.desc().width * lv.desc().height;
        lv.upload(s, &bytes(&vec![k as f32; n])).ctx("level upload")?;
    }
    let tm = TextureObject::from_mipmapped(&mip, TextureDesc { filter: FilterMode::Point, mip_filter: FilterMode::Linear, ..TextureDesc::default() }).ctx("mip texture")?;
    let lods = [0.0f32, 0.25, 1.0, 1.5, 2.75, 3.5, 4.0];
    let ld = g.up(&lods)?;
    let mout = g.zeros::<f32>(2 * lods.len())?;
    let (mut b0, mut b1, mut b2, mut b3) = (tm.handle(), ld.cu_deviceptr(), mout.cu_deviceptr(), lods.len() as u32);
    let f = m.load_function("odt_tex_mip").ctx("function")?;
    // SAFETY: arguments match `odt_tex_mip`; buffers and texture outlive the launch.
    unsafe { run_kernel(g, &f, lods.len() as u32, &mut [arg(&mut b0), arg(&mut b1), arg(&mut b2), arg(&mut b3)]) }?;
    let got = g.down(&mout)?;
    close("tex2d_lod returns the level of detail", &got.iter().step_by(2).copied().collect::<Vec<_>>(), &lods, 2.0 / 256.0 * 4.0, |i| format!("lod {}", lods[i]))?;
    close("tex2d_grad returns the level of detail", &got.iter().skip(1).step_by(2).copied().collect::<Vec<_>>(), &lods, 0.05, |i| format!("lod {}", lods[i]))?;

    // Layered, cubemap, read-as-integer, linear fetch.
    let layered = Arc::new(Array::new(&g.cuda, ArrayDesc { depth: 3, layered: true, ..ArrayDesc::d2(4, 4, ArrayFormat::F32, 1) }).ctx("layered array")?);
    layered.upload(s, &bytes(&(0..48).map(|i| ((i / 16) * 100 + i % 16) as f32).collect::<Vec<_>>())).ctx("layered upload")?;
    let tlay = TextureObject::from_array(&layered, point_unnorm()).ctx("layered texture")?;
    let cube = Arc::new(Array::new(&g.cuda, ArrayDesc { depth: 6, cubemap: true, ..ArrayDesc::d2(4, 4, ArrayFormat::F32, 1) }).ctx("cubemap array")?);
    cube.upload(s, &bytes(&(0..96).map(|i| (i / 16 + 10) as f32).collect::<Vec<_>>())).ctx("cubemap upload")?;
    let tcube = TextureObject::from_array(&cube, TextureDesc { filter: FilterMode::Point, ..TextureDesc::default() }).ctx("cubemap texture")?;
    let rgba8: Vec<[u8; 4]> = (0..16).map(|i| [(i % 4 * 16 + i / 4) as u8, 255 - (i % 4) as u8, (i / 4 * 3) as u8, 7]).collect();
    let i8a = Arc::new(Array::new(&g.cuda, ArrayDesc::d2(4, 4, ArrayFormat::U8, 4)).ctx("rgba8 array")?);
    i8a.upload(s, &bytes(&rgba8)).ctx("rgba8 upload")?;
    let tint = TextureObject::from_array(&i8a, TextureDesc { read_as_integer: true, ..point_unnorm() }).ctx("integer texture")?;
    let tnorm = TextureObject::from_array(&i8a, point_unnorm()).ctx("normalized texture")?;
    let linv: Vec<f32> = (0..48).map(|i| i as f32 * 1.5 - 3.0).collect();
    let lind = g.up(&linv)?;
    let tlin = TextureObject::from_linear(lind.storage(), ArrayFormat::F32, 1, point_unnorm()).ctx("linear texture")?;
    let xout = g.zeros::<u32>(48 * 12)?;
    let (mut c0, mut c1, mut c2, mut c3, mut c4, mut c5) = (tlay.handle(), tcube.handle(), tint.handle(), tnorm.handle(), tlin.handle(), xout.cu_deviceptr());
    let f = m.load_function("odt_tex_misc").ctx("function")?;
    // SAFETY: arguments match `odt_tex_misc`; buffers and textures outlive the launch.
    unsafe { run_kernel(g, &f, 48, &mut [arg(&mut c0), arg(&mut c1), arg(&mut c2), arg(&mut c3), arg(&mut c4), arg(&mut c5)]) }?;
    let got = g.down(&xout)?;
    for i in 0..48usize {
        let r = &got[12 * i..12 * i + 12];
        let fl = |w: u32| f32::from_bits(w);
        let face = (i / 2) % 3 * 2 + i % 2;
        let tx = (i % 4) + 4 * ((i / 4) % 4);
        let px = rgba8[tx];
        same(&format!("thread {i} (layered, cube face, linear fetch)"), &[fl(r[0]), fl(r[1]), fl(r[10])], &[((i / 16) * 100 + tx) as f32, (face + 10) as f32, linv[i]], |j| format!("[{j}]"))?;
        same(&format!("thread {i} read_as_integer"), &r[2..6], &px.map(u32::from), |j| format!("ch {j}"))?;
        close(&format!("thread {i} normalized read"), &r[6..10].iter().map(|w| fl(*w)).collect::<Vec<_>>(), &px.map(|v| f32::from(v) / 255.0), 1e-6, |j| format!("ch {j}"))?;
    }

    // Surfaces of every width.
    surfaces::<u8>(g, &m, "odt_surf_w1", ArrayFormat::U8, 1)?;
    surfaces::<u16>(g, &m, "odt_surf_w2", ArrayFormat::U16, 1)?;
    surfaces::<u32>(g, &m, "odt_surf_w4", ArrayFormat::U32, 1)?;
    surfaces::<[u32; 2]>(g, &m, "odt_surf_w8", ArrayFormat::U32, 2)?;
    surfaces::<[u32; 4]>(g, &m, "odt_surf_w16", ArrayFormat::U32, 4)?;
    println!("texture: 2D point/linear/gather, lod/grad, layered, cubemap, integer/normalized reads, linear fetch, surfaces 1/2/4/8/16 bytes x trap/clamp/zero");
    Ok(())
}

/// Writes then reads one 8x4 surface of `T` and checks reads and contents.
fn surfaces<T: bytemuck::Pod + Mk + PartialEq + std::fmt::Debug>(g: &Gpu, m: &Arc<Module>, name: &str, format: ArrayFormat, channels: u32) -> R {
    let s = &g.stream;
    let arr = Arc::new(Array::new(&g.cuda, ArrayDesc { surface: true, ..ArrayDesc::d2(SW as usize, SH as usize, format, channels) }).ctx("surface array")?);
    let surf = SurfaceObject::new(&arr).ctx("surface")?;
    let out = g.zeros::<u8>((GW * GH * 48) as usize)?;
    let f = m.load_function(name).ctx("function")?;
    for phase in 0..2u32 {
        let (mut a0, mut a1, mut a2) = (surf.handle(), out.cu_deviceptr(), phase);
        // SAFETY: arguments match the surface kernels; the surface and `out` outlive the launch.
        unsafe { run_kernel(g, &f, (GW * GH) as u32, &mut [arg(&mut a0), arg(&mut a1), arg(&mut a2)]) }?;
    }
    let got = g.down(&out)?;
    let read = |i: usize, slot: usize| -> T { bytemuck::pod_read_unaligned(&got[48 * i + 16 * slot..48 * i + 16 * slot + size_of::<T>()]) };
    let zero: T = bytemuck::Zeroable::zeroed();
    for y in 0..GH {
        for x in 0..GW {
            let i = (y * GW + x) as usize;
            let inside = x < SW && y < SH;
            let clamped = T::mk(seed(x.min(SW - 1), y.min(SH - 1)));
            let want_zero = if inside { clamped } else { zero };
            same(&format!("{name} ({x}, {y}) read zero / clamp"), &[read(i, 0), read(i, 1)], &[want_zero, clamped], |j| format!("[{j}]"))?;
            if inside {
                same(&format!("{name} ({x}, {y}) read trap"), &[read(i, 2)], &[clamped], |_| String::new())?;
            }
        }
    }
    let mut back = vec![0u8; arr.desc().total_bytes()];
    arr.download(s, &mut back).ctx("download")?;
    let want: Vec<T> = (0..SH).flat_map(|y| (0..SW).map(move |x| T::mk(seed(x, y)))).collect();
    let got: Vec<T> = back.chunks_exact(size_of::<T>()).map(bytemuck::pod_read_unaligned).collect();
    same(&format!("{name} contents after writes (out-of-range Zero writes dropped)"), &got, &want, |i| format!("({}, {})", i % SW as usize, i / SW as usize))
}

/// Runs the sparse-residency case.
pub fn sparse(g: &Gpu) -> R {
    if !cuda_ext::sparse_arrays_supported(&g.cuda).ctx("sparse attribute")? {
        return Err("CU_DEVICE_ATTRIBUTE_SPARSE_CUDA_ARRAY_SUPPORTED is 0 on this device".into());
    }
    let s = &g.stream;
    let probe = Array::new(&g.cuda, ArrayDesc { sparse: true, ..ArrayDesc::d2(256, 256, ArrayFormat::F32, 1) }).ctx("sparse array")?;
    let props = probe.sparse_properties().ctx("sparse properties")?;
    let [tw, th, _] = props.tile;
    let (w, h) = (2 * tw as usize, 2 * th as usize);
    let arr = Arc::new(Array::new(&g.cuda, ArrayDesc { sparse: true, ..ArrayDesc::d2(w, h, ArrayFormat::F32, 1) }).ctx("sparse 2x2-tile array")?);
    let pool = TilePool::new(&g.cuda, 2).ctx("tile pool")?;
    // Tiles (0, 0) and (1, 1) are mapped; (1, 0) and (0, 1) are not.
    arr.map_tiles(s, [0, 0], [tw, th], &pool, 0).ctx("map tile (0, 0)")?;
    arr.map_tiles(s, [tw, th], [tw, th], &pool, pool.tile_bytes()).ctx("map tile (1, 1)")?;
    let t = TextureObject::from_array(&arr, point_unnorm()).ctx("sparse texture")?;
    let tiles = [(0u32, 0u32, true), (1, 0, false), (0, 1, false), (1, 1, true)];
    let coords: Vec<[f32; 2]> = tiles.iter().flat_map(|&(tx, ty, _)| [[0.25, 0.25], [0.5, 0.5], [0.75, 0.75]].map(|[fx, fy]| [(tx as f32 + fx) * tw as f32, (ty as f32 + fy) * th as f32])).collect();
    let cd = g.up(&coords)?;
    let out = g.zeros::<u32>(6 * coords.len())?;
    let m = load(g, &["odt_tex_sparse"])?;
    let f = m.load_function("odt_tex_sparse").ctx("function")?;
    let (mut a0, mut a1, mut a2, mut a3) = (t.handle(), cd.cu_deviceptr(), out.cu_deviceptr(), coords.len() as u32);
    // SAFETY: arguments match `odt_tex_sparse`; texture, pool and buffers outlive the launch,
    // which is ordered after the tile mappings on the same stream.
    unsafe { run_kernel(g, &f, coords.len() as u32, &mut [arg(&mut a0), arg(&mut a1), arg(&mut a2), arg(&mut a3)]) }?;
    let got = g.down(&out)?;
    for (i, c) in coords.iter().enumerate() {
        let (_, _, resident) = tiles[i / 3];
        let r = &got[6 * i..6 * i + 6];
        let res = [r[1], r[3], r[5]];
        same(&format!("residency at {c:?} (tex2d, lod, gather)"), &res, &[u32::from(resident); 3], |j| format!("[{j}]"))?;
        if !resident {
            same(&format!("unmapped fetch at {c:?} is zero"), &[r[0], r[2], r[4]], &[0; 3], |j| format!("[{j}]"))?;
        }
    }
    arr.unmap_tiles(s, [0, 0], [tw, th]).ctx("unmap tile (0, 0)")?;
    arr.unmap_tiles(s, [tw, th], [tw, th]).ctx("unmap tile (1, 1)")?;
    s.synchronize().ctx("sync")?;
    println!("sparse: tile {tw}x{th}, 2 of 4 tiles mapped, residency via tex2d/lod/gather sparse");
    Ok(())
}

