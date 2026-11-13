//! Case `coop_vec`: cooperative vectors (`optix_device::coop_vec`) at N = 8 (the `_16xi32`
//! register path), 32 (`_64xi32`) and 80 (the pointer path).
//!
//! Each of 32 raygen threads loads its own f16 vectors `a`, `b` (multiples of 1/8 in
//! [-0.5, 0.5), so sums and products are exact in f16), an f32 copy of `a` and an i32 vector,
//! and stores `add`, `sub`, `mul`, `max`, `min`, `ffma(a, b, a)`, `step`, `cvt(i32 -> f16)`,
//! `matmul_bias(W, a, bias)` (W converted on the host from f32 row-major to f16
//! inferencing-optimal with `optix::coop_vec::convert`), `exp2` and `tanh`; it also adds its f32
//! `a` into one vector with `reduce_sum_acc` and `a * b^T` into one f16 training-optimal matrix
//! with `outer_product_acc` (converted back to f32 row-major on the host). Exact ops are checked
//! exactly, the matrix product within f16 accumulation error, the activations within f16
//! approximation error.

use super::common::{Ctx, Gpu, R, close, opts};
use bytemuck::{Pod, Zeroable};
use cuda_core::DeviceBuffer;
use cuda_device::{ConstantMemory, constant, cuda_module, kernel};
use optix::coop_vec::{self, Batch, ElemType, MatrixLayout, Network};
use optix::{HitGroup, LinkOptions, ProgramGroup, SbtRecord, ShaderBindingTable};
use optix_device::coop_vec::{self as cv, CoopVec, F16};
use optix_device::state;
use optix_device::sys as dsys;

const THREADS: u32 = 32;
/// Result vectors per thread.
const RES: usize = 11;

/// Launch parameters (`params_cv`); every pointer is 64-byte aligned (device allocations).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    /// `RES * N` f16 per thread.
    pub out: u64,
    /// `N` f16 per thread.
    pub a: u64,
    /// `N` f16 per thread.
    pub b: u64,
    /// `N` f32 per thread (the same values as `a`).
    pub af: u64,
    /// `N` i32 per thread.
    pub ints: u64,
    /// The N x N f16 matrix, inferencing-optimal.
    pub mat: u64,
    /// `N` f16.
    pub bias: u64,
    /// `N` f32, reduce-sum target.
    pub sum: u64,
    /// The N x N f16 training-optimal outer-product target.
    pub outer: u64,
}

// SAFETY: plain integers; all-zero is valid.
unsafe impl cuda_device::ConstantMemoryValue for Params {}

/// Runs `$body` with `$k` bound to every literal `0..80` below `$n` (the guards fold away), so
/// vector elements are only ever indexed by constants.
macro_rules! each {
    ($n:expr, $k:ident => $body:expr) => {
        each!(@ $n, $k => $body; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
            32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63
            64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79)
    };
    (@ $n:expr, $k:ident => $body:expr; $($i:literal)*) => {
        $( if $i < $n { let $k: usize = $i; $body; } )*
    };
}

#[cuda_module]
mod kernels {
    use super::*;

    #[constant(export_name = "params_cv")]
    static PARAMS: ConstantMemory<Params> = ConstantMemory::UNINIT;

    /// Writes the f16 bits of `v` at `dst[at * N..]`.
    ///
    /// # Safety
    ///
    /// `dst[at * N..(at + 1) * N]` is writable.
    #[inline(always)]
    unsafe fn store<const N: usize>(dst: *mut u16, at: usize, v: &CoopVec<F16, N>) {
        // SAFETY: forwarded contract; `k < N`.
        each!(N, k => unsafe { *dst.add(at * N + k) = v.0[k].to_bits() });
    }

    /// The whole per-thread test at vector size `N`.
    ///
    /// # Safety
    ///
    /// Raygen of a `THREADS`-wide launch with `params_cv` set up for `N`.
    #[inline(always)]
    unsafe fn body<const N: usize>() {
        use dsys::{
            OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_FLOAT16 as H, OptixCoopVecMatrixLayout_OPTIX_COOP_VEC_MATRIX_LAYOUT_INFERENCING_OPTIMAL as INF,
            OptixCoopVecMatrixLayout_OPTIX_COOP_VEC_MATRIX_LAYOUT_TRAINING_OPTIMAL as TRAIN,
        };
        // SAFETY: forwarded contract; every buffer holds this thread's slice, 16-byte aligned
        // (N * 2 and N * 4 bytes are multiples of 16 for N in {8, 32, 80}).
        unsafe {
            let t = state::launch_index()[0] as u64;
            let p = PARAMS.get();
            let a = cv::load::<F16, N>(p.a + t * N as u64 * 2);
            let b = cv::load::<F16, N>(p.b + t * N as u64 * 2);
            let af = cv::load::<f32, N>(p.af + t * N as u64 * 4);
            let iv = cv::load::<i32, N>(p.ints + t * N as u64 * 4);
            let o = (p.out as *mut u16).add(t as usize * RES * N);
            store(o, 0, &cv::add(&a, &b));
            store(o, 1, &cv::sub(&a, &b));
            store(o, 2, &cv::mul(&a, &b));
            store(o, 3, &cv::max(&a, &b));
            store(o, 4, &cv::min(&a, &b));
            store(o, 5, &cv::ffma(&a, &b, &a));
            store(o, 6, &cv::step(&a, &b));
            store(o, 7, &cv::cvt::<F16, N, i32, N>(&iv));
            store(o, 8, &cv::matmul_bias::<F16, F16, H, INF, false, N, N, H, H>(&a, p.mat, 0, p.bias, 0, 0));
            store(o, 9, &cv::exp2(&a));
            store(o, 10, &cv::tanh(&a));
            cv::reduce_sum_acc(&af, p.sum, 0);
            cv::outer_product_acc::<F16, N, F16, N, TRAIN>(&a, &b, p.outer, 0, 0);
        }
    }

    #[kernel]
    pub fn __raygen__cv8() {
        // SAFETY: raygen of the N = 8 pipeline.
        unsafe { body::<8>() }
    }

    #[kernel]
    pub fn __raygen__cv32() {
        // SAFETY: raygen of the N = 32 pipeline.
        unsafe { body::<32>() }
    }

    #[kernel]
    pub fn __raygen__cv80() {
        // SAFETY: raygen of the N = 80 pipeline.
        unsafe { body::<80>() }
    }

    #[kernel]
    pub fn __miss__cv() {}
}

/// IEEE binary16 bits of `x`, exact for the values used here (normal range or zero, at most
/// 11 significant bits).
fn h(x: f32) -> u16 {
    let bits = x.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    if x == 0.0 {
        return sign;
    }
    let exp = ((bits >> 23) & 0xff) as i32 - 127 + 15;
    let mant = bits & 0x7f_ffff;
    let mut v = ((exp as u32) << 10) | (mant >> 13);
    let rest = mant & 0x1fff;
    if rest > 0x1000 || (rest == 0x1000 && v & 1 == 1) {
        v += 1;
    }
    sign | v as u16
}

/// The value of binary16 bits `v` (normal numbers, zeros, subnormals).
fn f(v: u16) -> f32 {
    let sign = if v & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exp = i32::from((v >> 10) & 0x1f);
    let mant = f32::from(v & 0x3ff);
    sign * if exp == 0 { mant * 2f32.powi(-24) } else { (1.0 + mant / 1024.0) * 2f32.powi(exp - 15) }
}

/// Runs the case at every size.
pub fn run(g: &Gpu) -> R {
    if !g.ctx.coop_vec().ctx("coop_vec property")? {
        return Err("OPTIX_DEVICE_PROPERTY_COOP_VEC is 0 on this device".into());
    }
    let o = opts("params_cv", size_of::<Params>())?;
    let module = g.module(&Default::default(), &o, &["__raygen__cv8", "__raygen__cv32", "__raygen__cv80", "__miss__cv"])?;
    let miss = ProgramGroup::miss(&module, "__miss__cv").ctx("miss")?;
    let hit = ProgramGroup::hit_group(&g.ctx, HitGroup::default()).ctx("empty hit group")?;
    // N = 80 is the pointer path: OptiX receives the addresses of operands in the program's
    // `.local` depot, which needs 64-bit local pointers (case `ptr64`).
    for (n, entry) in [(8usize, "__raygen__cv8"), (32, "__raygen__cv32"), (80, "__raygen__cv80")] {
        let raygen = ProgramGroup::raygen(&module, entry).ctx("raygen")?;
        let pipeline = g.pipeline(&o, LinkOptions { max_trace_depth: 0, ..Default::default() }, &[raygen.clone(), miss.clone(), hit.clone()])?;
        let sbt = ShaderBindingTable::new(&g.stream, &SbtRecord::new(&raygen, 0u32).ctx("rg")?, &[SbtRecord::new(&miss, 0u32).ctx("ms")?], &[SbtRecord::new(&hit, 0u32).ctx("hg")?]).ctx("SBT")?;
        size(g, n, &pipeline, &sbt).map_err(|e| format!("N = {n}: {e}"))?;
    }
    println!("coop_vec: N = 8, 32, 80 x {THREADS} threads: element-wise ops, cvt, matmul+bias, exp2, tanh, reduce-sum, outer product");
    Ok(())
}

/// One size: inputs, conversion, launch, checks.
fn size(g: &Gpu, n: usize, pipeline: &optix::Pipeline, sbt: &ShaderBindingTable) -> R {
    let t = THREADS as usize;
    // Values k / 8 for k in -4..4: sums, differences, products and 32-term sums of products are
    // exact in f16.
    let av: Vec<f32> = (0..t * n).map(|i| ((i * 7 + 3) % 8) as f32 / 8.0 - 0.5).collect();
    let bv: Vec<f32> = (0..t * n).map(|i| ((i * 5 + 1) % 8) as f32 / 8.0 - 0.5).collect();
    let iv: Vec<i32> = (0..t * n).map(|i| (i % 201) as i32 - 100).collect();
    let w: Vec<f32> = (0..n * n).map(|i| ((i * 37 + 11) % 64) as f32 / 64.0 - 0.5).collect();
    let bias: Vec<f32> = (0..n).map(|i| (i % 16) as f32 / 16.0 - 0.5).collect();

    let rm = Network::packed(&g.ctx, &[(n as u32, n as u32)], ElemType::F32, MatrixLayout::RowMajor).ctx("f32 row-major network")?;
    let inf = Network::packed(&g.ctx, &[(n as u32, n as u32)], ElemType::F16, MatrixLayout::InferencingOptimal).ctx("f16 inferencing network")?;
    let train = Network::packed(&g.ctx, &[(n as u32, n as u32)], ElemType::F16, MatrixLayout::TrainingOptimal).ctx("f16 training network")?;
    let mut host = vec![0f32; rm.bytes() / 4];
    let m = rm.matrices()[0];
    for r in 0..n {
        for c in 0..n {
            host[m.offset as usize / 4 + r * m.stride as usize / 4 + c] = w[r * n + c];
        }
    }
    let src = g.up(&host)?;
    let mut mat = DeviceBuffer::<u8>::zeroed(&g.stream, inf.bytes()).ctx("matrix")?;
    // SAFETY: both buffers outlive the synchronization in `down` below (same stream).
    unsafe { coop_vec::convert(&g.ctx, &g.stream, 1, Batch { network: &rm, buffer: &src, stride: 0 }, Batch { network: &inf, buffer: &mut mat, stride: 0 }) }.ctx("convert W")?;

    let a = g.up(&av.iter().map(|x| h(*x)).collect::<Vec<u16>>())?;
    let b = g.up(&bv.iter().map(|x| h(*x)).collect::<Vec<u16>>())?;
    let af = g.up(&av)?;
    let ints = g.up(&iv)?;
    let biasd = g.up(&bias.iter().map(|x| h(*x)).collect::<Vec<u16>>())?;
    let sum = g.zeros::<f32>(n)?;
    let outer = g.zeros::<u8>(train.bytes())?;
    let out = g.zeros::<u16>(t * RES * n)?;
    let params = Params {
        out: out.cu_deviceptr(),
        a: a.cu_deviceptr(),
        b: b.cu_deviceptr(),
        af: af.cu_deviceptr(),
        ints: ints.cu_deviceptr(),
        mat: mat.cu_deviceptr(),
        bias: biasd.cu_deviceptr(),
        sum: sum.cu_deviceptr(),
        outer: outer.cu_deviceptr(),
    };
    // SAFETY: `params` matches the device struct; every buffer is sized for THREADS threads at
    // this N; no SBT data is read; the launch synchronizes.
    unsafe { g.launch(pipeline, sbt, &params, [THREADS, 1, 1]) }?;
    let got: Vec<f32> = g.down(&out)?.into_iter().map(f).collect();

    let names = ["add", "sub", "mul", "max", "min", "ffma", "step", "cvt", "matmul_bias", "exp2", "tanh"];
    for (r, name) in names.iter().enumerate() {
        let (mut gv, mut wv, mut tol) = (Vec::new(), Vec::new(), 0.0f32);
        for th in 0..t {
            for i in 0..n {
                let (x, y) = (av[th * n + i], bv[th * n + i]);
                let want = match r {
                    0 => x + y,
                    1 => x - y,
                    2 => x * y,
                    3 => x.max(y),
                    4 => x.min(y),
                    5 => x * y + x,
                    6 => {
                        if x < y {
                            0.0
                        } else {
                            1.0
                        }
                    }
                    7 => iv[th * n + i] as f32,
                    8 => {
                        let row = (0..n).map(|k| w[i * n + k] * av[th * n + k]).sum::<f32>() + bias[i];
                        // f16 inputs are exact; products and sums are rounded to f16 by the
                        // hardware at most once per term.
                        tol = tol.max(2e-3 * (0..n).map(|k| (w[i * n + k] * av[th * n + k]).abs()).sum::<f32>() + 2e-3);
                        row
                    }
                    9 => x.exp2(),
                    _ => x.tanh(),
                };
                gv.push(got[(th * RES + r) * n + i]);
                wv.push(want);
            }
        }
        let tol = match r {
            8 => tol,
            9 | 10 => 4e-3,
            _ => 0.0,
        };
        close(name, &gv, &wv, tol, |j| format!("thread {} element {}", j / n, j % n))?;
    }
    let want_sum: Vec<f32> = (0..n).map(|i| (0..t).map(|th| av[th * n + i]).sum()).collect();
    close("reduce_sum_acc", &g.down(&sum)?, &want_sum, 0.0, |i| format!("element {i}"))?;
    let mut back = DeviceBuffer::<f32>::zeroed(&g.stream, rm.bytes() / 4).ctx("outer back")?;
    // SAFETY: both buffers outlive the synchronization in `down` below (same stream).
    unsafe { coop_vec::convert(&g.ctx, &g.stream, 1, Batch { network: &train, buffer: &outer, stride: 0 }, Batch { network: &rm, buffer: &mut back, stride: 0 }) }.ctx("convert outer product")?;
    let back = g.down(&back)?;
    let (mut gv, mut wv) = (Vec::new(), Vec::new());
    for r in 0..n {
        for c in 0..n {
            gv.push(back[m.offset as usize / 4 + r * m.stride as usize / 4 + c]);
            wv.push((0..t).map(|th| av[th * n + r] * bv[th * n + c]).sum());
        }
    }
    close("outer_product_acc", &gv, &wv, 0.0, |i| format!("({}, {})", i / n, i % n))
}
