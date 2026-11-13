//! Cooperative vectors (OptiX 9.1 device side): small per-thread vectors that OptiX evaluates
//! cooperatively across the warp — vector load, activation functions and element-wise math,
//! matrix-vector multiply with optional bias, and the training-side atomic reductions
//! (reduce-sum and outer-product accumulate).
//!
//! **Why.** Neural shading / neural radiance caches run small MLPs inside OptiX programs; the
//! `optixCoopVec*` templates in `optix_device.h` are the only way to reach the tensor-core path.
//!
//! **How.** A port of `internal/optix_device_impl_coop_vec.h`. [`CoopVec<T, N>`] is the
//! `OptixCoopVec<T, N>` class (`#[repr(C)]` array). Each operation dispatches exactly as the
//! header's `*ASMGenerator` structs do, decided by the const sizes at compile time:
//! - more than 64 elements (or an element wider than 4 bytes) on any operand: the `_ptr`
//!   intrinsic, which receives the addresses of the operands and of the result;
//! - otherwise every element is widened to one `u32` register by zero-extending its bit pattern
//!   (the header's `*(const bitType*)&v[i]`) and the `_16xi32` intrinsic is called when every
//!   operand has at most 16 elements, the `_64xi32` one otherwise; results come back as `u32`
//!   registers truncated to the element's bit width and reinterpreted.
//!
//! Register slots past the vector's size are passed as `0` (the header leaves them
//! uninitialized; OptiX ignores them). Element types and layouts are the generated
//! `sys::OptixCoopVecElemType_*` / `sys::OptixCoopVecMatrixLayout_*` constants; the header's
//! template parameters that OptiX needs as immediates are const generics here.
//!
//! **Uses** [`crate::raw`] (`_optix_vector_*`, `_optix_matvecmul_*`,
//! `_optix_reduce_sum_accumulate_*`, `_optix_outer_product_accumulate_*`,
//! `_optix_coop_vec_get_matrix_size`) and [`crate::sys`].
//! **Used by** OptiX programs evaluating or training small networks.

use core::mem::MaybeUninit;

use crate::raw;
use crate::sys::{OptixCoopVecElemType, OptixCoopVecMatrixLayout};

/// `optix_internal::OptixCoopVecOp` (`internal/optix_device_impl_coop_vec.h`, lines 26-40).
///
/// The header keeps this enum in its internal namespace, so it is not part of `optix_types.h`
/// and not in the generated [`crate::sys`]; these values are the one hand-copied set in the
/// crate for that reason. `OPTIX_COOP_VEC_OP_UNKNOWN` (`0x2A20`) is never passed by the header
/// and is omitted.
mod op {
    pub const EXP2: u32 = 0x2A21;
    pub const LOG2: u32 = 0x2A22;
    pub const TANH: u32 = 0x2A23;
    pub const MAX: u32 = 0x2A24;
    pub const MIN: u32 = 0x2A25;
    pub const FFMA: u32 = 0x2A26;
    pub const MUL: u32 = 0x2A27;
    pub const ADD: u32 = 0x2A28;
    pub const SUB: u32 = 0x2A29;
    pub const CVT: u32 = 0x2A2A;
    pub const STEP: u32 = 0x2A2B;
}

mod sealed {
    /// Restricts [`super::CoopElem`] to the element types the header's type table maps.
    pub trait Sealed {}
}

/// An IEEE 754 binary16 value (`half` in the OptiX headers), carried as its bit pattern.
///
/// Rust's `f16` primitive is unstable on this toolchain and the crate enables no features, so
/// half-precision elements are this newtype; convert with [`F16::from_bits`] / [`F16::to_bits`]
/// or on the device with [`cvt`].
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct F16(pub u16);

impl F16 {
    /// The half whose bit pattern is `bits`.
    #[inline(always)]
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    /// This half's bit pattern.
    #[inline(always)]
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}

/// An element type of a [`CoopVec`]: the header's `OptixCoopVecElemTypeTrait` (element type
/// enum plus the bit type used to move the value through a 32-bit register).
///
/// Implemented for exactly the types the header's `TT` table maps: `i8`, `u8`, `i32`, `u32`,
/// `f32` and [`F16`]. Sealed.
pub trait CoopElem: Copy + sealed::Sealed {
    /// The `OPTIX_COOP_VEC_ELEM_TYPE_*` value of this type.
    const ELEM: OptixCoopVecElemType;
    /// The value's bit pattern zero-extended to a register (`*(const bitType*)&v`).
    fn to_reg(self) -> u32;
    /// The value whose bit pattern is the low bits of `r` (`bitType o = r; *(T*)&o`).
    fn from_reg(r: u32) -> Self;
}

impl sealed::Sealed for i8 {}
impl CoopElem for i8 {
    const ELEM: OptixCoopVecElemType = crate::sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_INT8;
    #[inline(always)]
    fn to_reg(self) -> u32 {
        // Zero-extends the byte, as the header reads it through `unsigned char`.
        u32::from(self as u8)
    }
    #[inline(always)]
    fn from_reg(r: u32) -> Self {
        r as u8 as i8
    }
}

impl sealed::Sealed for u8 {}
impl CoopElem for u8 {
    const ELEM: OptixCoopVecElemType = crate::sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_UINT8;
    #[inline(always)]
    fn to_reg(self) -> u32 {
        u32::from(self)
    }
    #[inline(always)]
    fn from_reg(r: u32) -> Self {
        r as u8
    }
}

impl sealed::Sealed for i32 {}
impl CoopElem for i32 {
    const ELEM: OptixCoopVecElemType = crate::sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_INT32;
    #[inline(always)]
    fn to_reg(self) -> u32 {
        self as u32
    }
    #[inline(always)]
    fn from_reg(r: u32) -> Self {
        r as i32
    }
}

impl sealed::Sealed for u32 {}
impl CoopElem for u32 {
    const ELEM: OptixCoopVecElemType = crate::sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_UINT32;
    #[inline(always)]
    fn to_reg(self) -> u32 {
        self
    }
    #[inline(always)]
    fn from_reg(r: u32) -> Self {
        r
    }
}

impl sealed::Sealed for f32 {}
impl CoopElem for f32 {
    const ELEM: OptixCoopVecElemType = crate::sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_FLOAT32;
    #[inline(always)]
    fn to_reg(self) -> u32 {
        self.to_bits()
    }
    #[inline(always)]
    fn from_reg(r: u32) -> Self {
        f32::from_bits(r)
    }
}

impl sealed::Sealed for F16 {}
impl CoopElem for F16 {
    const ELEM: OptixCoopVecElemType = crate::sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_FLOAT16;
    #[inline(always)]
    fn to_reg(self) -> u32 {
        u32::from(self.0)
    }
    #[inline(always)]
    fn from_reg(r: u32) -> Self {
        F16(r as u16)
    }
}

/// A cooperative vector of `N` elements of type `T` (`OptixCoopVec<T, N>`): `T` is the
/// header's `value_type`, `N` its `size`, the array its `m_data`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CoopVec<T: CoopElem, const N: usize>(pub [T; N]);

impl<T: CoopElem, const N: usize> CoopVec<T, N> {
    /// Number of elements (`OptixCoopVec::size`).
    pub const SIZE: usize = N;

    /// A vector holding `a`.
    #[inline(always)]
    pub const fn new(a: [T; N]) -> Self {
        Self(a)
    }

    /// Every element set to `v` (`OptixCoopVec(const value_type&)`).
    #[inline(always)]
    pub const fn splat(v: T) -> Self {
        Self([v; N])
    }

    /// Generic address of the elements (`data()`), as the `_ptr` intrinsics take it.
    #[inline(always)]
    fn addr(&self) -> u64 {
        // A pointer-to-integer `as` cast exposes the provenance the intrinsic reads through.
        self.0.as_ptr() as u64
    }
}

impl<T: CoopElem, const N: usize> From<[T; N]> for CoopVec<T, N> {
    #[inline(always)]
    fn from(a: [T; N]) -> Self {
        Self(a)
    }
}

/// The header's fallback test: too many elements or elements too wide for register passing.
#[inline(always)]
const fn by_ptr<T>(n: usize) -> bool {
    n > 64 || size_of::<T>() > size_of::<u32>()
}

/// Runs `$body` with `$k` bound to every literal `0..64` below `$n` (the guards fold away).
///
/// Register packing indexes vectors only with constants this way: a loop over the arrays (even
/// a fixed-count one) made cuda-oxide keep them in a `.local` depot addressed through 32-bit
/// registers, which OptiX programs fault on.
macro_rules! each64 {
    ($n:expr, $k:ident => $body:expr) => {
        each64!(@ $n, $k => $body; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
            32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63)
    };
    (@ $n:expr, $k:ident => $body:expr; $($i:literal)*) => {
        $( if $i < $n { let $k: usize = $i; $body; } )*
    };
}

/// Widens `v` into `R` registers; slots past `N` stay `0` (`N <= R <= 64` on every call path).
#[inline(always)]
fn pack<T: CoopElem, const N: usize, const R: usize>(v: &CoopVec<T, N>) -> [u32; R] {
    let mut r = [0u32; R];
    each64!(if N < R { N } else { R }, k => r[k] = v.0[k].to_reg());
    r
}

/// Narrows the first `N` of `R` registers into a vector (`N <= R <= 64` on every call path).
#[inline(always)]
fn unpack<T: CoopElem, const N: usize, const R: usize>(r: &[u32; R]) -> CoopVec<T, N> {
    let mut v = [T::from_reg(0); N];
    each64!(if N < R { N } else { R }, k => v[k] = T::from_reg(r[k]));
    CoopVec(v)
}

/// Spells out a `_16xi32` call: scalar operands, then 16 registers of each listed array.
macro_rules! regs16 {
    ($f:path; $($s:expr),*; $($r:ident),+) => {
        $f($($s,)* $(
            $r[0], $r[1], $r[2], $r[3], $r[4], $r[5], $r[6], $r[7],
            $r[8], $r[9], $r[10], $r[11], $r[12], $r[13], $r[14], $r[15],
        )+)
    };
}

/// Spells out a `_64xi32` call: scalar operands, then 64 registers of each listed array.
macro_rules! regs64 {
    ($f:path; $($s:expr),*; $($r:ident),+) => {
        $f($($s,)* $(
            $r[0], $r[1], $r[2], $r[3], $r[4], $r[5], $r[6], $r[7],
            $r[8], $r[9], $r[10], $r[11], $r[12], $r[13], $r[14], $r[15],
            $r[16], $r[17], $r[18], $r[19], $r[20], $r[21], $r[22], $r[23],
            $r[24], $r[25], $r[26], $r[27], $r[28], $r[29], $r[30], $r[31],
            $r[32], $r[33], $r[34], $r[35], $r[36], $r[37], $r[38], $r[39],
            $r[40], $r[41], $r[42], $r[43], $r[44], $r[45], $r[46], $r[47],
            $r[48], $r[49], $r[50], $r[51], $r[52], $r[53], $r[54], $r[55],
            $r[56], $r[57], $r[58], $r[59], $r[60], $r[61], $r[62], $r[63],
        )+)
    };
}

/// `OptixCoopVecASMGenerator<OP, VecTOut, VecTIn>::generateASM(vecA)`.
///
/// # Safety
///
/// The caller's contract (see the public wrappers).
#[inline(always)]
unsafe fn op1<const OP: u32, O: CoopElem, const NO: usize, I: CoopElem, const NI: usize>(
    a: &CoopVec<I, NI>,
) -> CoopVec<O, NO> {
    let (oe, no, ie, ni) = (O::ELEM as u32, NO as u32, I::ELEM as u32, NI as u32);
    if by_ptr::<I>(NI) || by_ptr::<O>(NO) {
        let mut out = MaybeUninit::<[O; NO]>::uninit();
        // SAFETY: caller's contract; `a` is live for the call and OptiX writes all `NO` result
        // elements through the result address before returning, so `out` is initialized.
        unsafe {
            raw::vector_op1_ptr(OP, oe, no, ie, ni, a.addr(), out.as_mut_ptr() as u64);
            CoopVec(out.assume_init())
        }
    } else if NO <= 16 && NI <= 16 {
        let ia: [u32; 16] = pack(a);
        // SAFETY: caller's contract.
        let o = unsafe { regs16!(raw::vector_op1_16xi32; OP, oe, no, ie, ni; ia) };
        unpack(&o)
    } else {
        let ia: [u32; 64] = pack(a);
        // SAFETY: caller's contract.
        let o = unsafe { regs64!(raw::vector_op1_64xi32; OP, oe, no, ie, ni; ia) };
        unpack(&o)
    }
}

/// `OptixCoopVecASMGenerator<OP, VecTOut, VecTIn>::generateASM(vecA, vecB)`.
///
/// # Safety
///
/// The caller's contract (see the public wrappers).
#[inline(always)]
unsafe fn op2<const OP: u32, O: CoopElem, const NO: usize, I: CoopElem, const NI: usize>(
    a: &CoopVec<I, NI>,
    b: &CoopVec<I, NI>,
) -> CoopVec<O, NO> {
    let (oe, no, ie, ni) = (O::ELEM as u32, NO as u32, I::ELEM as u32, NI as u32);
    if by_ptr::<I>(NI) || by_ptr::<O>(NO) {
        let mut out = MaybeUninit::<[O; NO]>::uninit();
        // SAFETY: caller's contract; operands are live for the call and OptiX writes all `NO`
        // result elements before returning, so `out` is initialized.
        unsafe {
            raw::vector_op2_ptr(OP, oe, no, ie, ni, a.addr(), b.addr(), out.as_mut_ptr() as u64);
            CoopVec(out.assume_init())
        }
    } else if NO <= 16 && NI <= 16 {
        let (ia, ib): ([u32; 16], [u32; 16]) = (pack(a), pack(b));
        // SAFETY: caller's contract.
        let o = unsafe { regs16!(raw::vector_op2_16xi32; OP, oe, no, ie, ni; ia, ib) };
        unpack(&o)
    } else {
        let (ia, ib): ([u32; 64], [u32; 64]) = (pack(a), pack(b));
        // SAFETY: caller's contract.
        let o = unsafe { regs64!(raw::vector_op2_64xi32; OP, oe, no, ie, ni; ia, ib) };
        unpack(&o)
    }
}

/// `OptixCoopVecASMGenerator<OP, VecTOut, VecTIn>::generateASM(vecA, vecB, vecC)`.
///
/// # Safety
///
/// The caller's contract (see the public wrappers).
#[inline(always)]
unsafe fn op3<const OP: u32, O: CoopElem, const NO: usize, I: CoopElem, const NI: usize>(
    a: &CoopVec<I, NI>,
    b: &CoopVec<I, NI>,
    c: &CoopVec<I, NI>,
) -> CoopVec<O, NO> {
    let (oe, no, ie, ni) = (O::ELEM as u32, NO as u32, I::ELEM as u32, NI as u32);
    if by_ptr::<I>(NI) || by_ptr::<O>(NO) {
        let mut out = MaybeUninit::<[O; NO]>::uninit();
        // SAFETY: caller's contract; operands are live for the call and OptiX writes all `NO`
        // result elements before returning, so `out` is initialized.
        unsafe {
            raw::vector_op3_ptr(OP, oe, no, ie, ni, a.addr(), b.addr(), c.addr(), out.as_mut_ptr() as u64);
            CoopVec(out.assume_init())
        }
    } else if NO <= 16 && NI <= 16 {
        let (ia, ib, ic): ([u32; 16], [u32; 16], [u32; 16]) = (pack(a), pack(b), pack(c));
        // SAFETY: caller's contract.
        let o = unsafe { regs16!(raw::vector_op3_16xi32; OP, oe, no, ie, ni; ia, ib, ic) };
        unpack(&o)
    } else {
        let (ia, ib, ic): ([u32; 64], [u32; 64], [u32; 64]) = (pack(a), pack(b), pack(c));
        // SAFETY: caller's contract.
        let o = unsafe { regs64!(raw::vector_op3_64xi32; OP, oe, no, ie, ni; ia, ib, ic) };
        unpack(&o)
    }
}

/// Loads a vector from global memory (`optixCoopVecLoad<VecTOut>(CUdeviceptr)`).
///
/// # Safety
///
/// Valid in every program type ("available anywhere"). `ptr` must be a global-memory address,
/// 16-byte aligned regardless of `T` and `N`, with `N` readable elements of type `T`.
#[inline(always)]
pub unsafe fn load<T: CoopElem, const N: usize>(ptr: u64) -> CoopVec<T, N> {
    let (e, n) = (T::ELEM as u32, N as u32);
    if by_ptr::<T>(N) {
        let mut out = MaybeUninit::<[T; N]>::uninit();
        // SAFETY: forwarded contract; OptiX writes all `N` elements through the result address
        // before returning, so `out` is initialized.
        unsafe {
            raw::vector_load_ptr(e, n, ptr, out.as_mut_ptr() as u64);
            CoopVec(out.assume_init())
        }
    } else if N <= 16 {
        // SAFETY: forwarded contract.
        let o = unsafe { raw::vector_load_16xi32(e, n, ptr) };
        unpack(&o)
    } else {
        // SAFETY: forwarded contract.
        let o = unsafe { raw::vector_load_64xi32(e, n, ptr) };
        unpack(&o)
    }
}

/// Loads a vector from global memory through a typed pointer
/// (`optixCoopVecLoad<VecTOut, T>(T*)`).
///
/// # Safety
///
/// As [`load`], for the address of `ptr`.
#[inline(always)]
pub unsafe fn load_ptr<T: CoopElem, const N: usize, P>(ptr: *const P) -> CoopVec<T, N> {
    // SAFETY: forwarded contract; the `as` cast exposes the pointer's provenance to the load.
    unsafe { load(ptr as u64) }
}

/// Element-wise `2^v` (`optixCoopVecExp2`).
///
/// # Safety
///
/// Valid in every program type ("available anywhere"); `T` must be a type OptiX supports for
/// this operation.
#[inline(always)]
pub unsafe fn exp2<T: CoopElem, const N: usize>(v: &CoopVec<T, N>) -> CoopVec<T, N> {
    // SAFETY: forwarded contract.
    unsafe { op1::<{ op::EXP2 }, T, N, T, N>(v) }
}

/// Element-wise `log2(v)` (`optixCoopVecLog2`).
///
/// # Safety
///
/// As [`exp2`].
#[inline(always)]
pub unsafe fn log2<T: CoopElem, const N: usize>(v: &CoopVec<T, N>) -> CoopVec<T, N> {
    // SAFETY: forwarded contract.
    unsafe { op1::<{ op::LOG2 }, T, N, T, N>(v) }
}

/// Element-wise `tanh(v)` (`optixCoopVecTanh`).
///
/// # Safety
///
/// As [`exp2`].
#[inline(always)]
pub unsafe fn tanh<T: CoopElem, const N: usize>(v: &CoopVec<T, N>) -> CoopVec<T, N> {
    // SAFETY: forwarded contract.
    unsafe { op1::<{ op::TANH }, T, N, T, N>(v) }
}

/// Converts element types (`optixCoopVecCvt<VecTOut, VecTIn>`). OptiX supports only integral
/// to 16- or 32-bit floating point. Both sizes are passed to OptiX, as the header does.
///
/// # Safety
///
/// Valid in every program type ("available anywhere"); the `I` to `O` conversion must be one
/// OptiX supports.
#[inline(always)]
pub unsafe fn cvt<O: CoopElem, const NO: usize, I: CoopElem, const NI: usize>(
    v: &CoopVec<I, NI>,
) -> CoopVec<O, NO> {
    // SAFETY: forwarded contract.
    unsafe { op1::<{ op::CVT }, O, NO, I, NI>(v) }
}

/// Element-wise minimum (`optixCoopVecMin(vecA, vecB)`).
///
/// # Safety
///
/// As [`exp2`].
#[inline(always)]
pub unsafe fn min<T: CoopElem, const N: usize>(a: &CoopVec<T, N>, b: &CoopVec<T, N>) -> CoopVec<T, N> {
    // SAFETY: forwarded contract.
    unsafe { op2::<{ op::MIN }, T, N, T, N>(a, b) }
}

/// Element-wise minimum with a scalar (`optixCoopVecMin(vecA, value_type B)`).
///
/// # Safety
///
/// As [`exp2`].
#[inline(always)]
pub unsafe fn min_scalar<T: CoopElem, const N: usize>(a: &CoopVec<T, N>, b: T) -> CoopVec<T, N> {
    // SAFETY: forwarded contract.
    unsafe { min(a, &CoopVec::splat(b)) }
}

/// Element-wise maximum (`optixCoopVecMax(vecA, vecB)`).
///
/// # Safety
///
/// As [`exp2`].
#[inline(always)]
pub unsafe fn max<T: CoopElem, const N: usize>(a: &CoopVec<T, N>, b: &CoopVec<T, N>) -> CoopVec<T, N> {
    // SAFETY: forwarded contract.
    unsafe { op2::<{ op::MAX }, T, N, T, N>(a, b) }
}

/// Element-wise maximum with a scalar (`optixCoopVecMax(vecA, value_type B)`).
///
/// # Safety
///
/// As [`exp2`].
#[inline(always)]
pub unsafe fn max_scalar<T: CoopElem, const N: usize>(a: &CoopVec<T, N>, b: T) -> CoopVec<T, N> {
    // SAFETY: forwarded contract.
    unsafe { max(a, &CoopVec::splat(b)) }
}

/// Element-wise `a * b` (`optixCoopVecMul`).
///
/// # Safety
///
/// As [`exp2`].
#[inline(always)]
pub unsafe fn mul<T: CoopElem, const N: usize>(a: &CoopVec<T, N>, b: &CoopVec<T, N>) -> CoopVec<T, N> {
    // SAFETY: forwarded contract.
    unsafe { op2::<{ op::MUL }, T, N, T, N>(a, b) }
}

/// Element-wise `a + b` (`optixCoopVecAdd`).
///
/// # Safety
///
/// As [`exp2`].
#[inline(always)]
pub unsafe fn add<T: CoopElem, const N: usize>(a: &CoopVec<T, N>, b: &CoopVec<T, N>) -> CoopVec<T, N> {
    // SAFETY: forwarded contract.
    unsafe { op2::<{ op::ADD }, T, N, T, N>(a, b) }
}

/// Element-wise `a - b` (`optixCoopVecSub`).
///
/// # Safety
///
/// As [`exp2`].
#[inline(always)]
pub unsafe fn sub<T: CoopElem, const N: usize>(a: &CoopVec<T, N>, b: &CoopVec<T, N>) -> CoopVec<T, N> {
    // SAFETY: forwarded contract.
    unsafe { op2::<{ op::SUB }, T, N, T, N>(a, b) }
}

/// Element-wise step: `result[i] = if a[i] < b[i] { 0 } else { 1 }` (`optixCoopVecStep`).
///
/// # Safety
///
/// As [`exp2`].
#[inline(always)]
pub unsafe fn step<T: CoopElem, const N: usize>(a: &CoopVec<T, N>, b: &CoopVec<T, N>) -> CoopVec<T, N> {
    // SAFETY: forwarded contract.
    unsafe { op2::<{ op::STEP }, T, N, T, N>(a, b) }
}

/// Element-wise fused multiply-add `a * b + c` (`optixCoopVecFFMA`).
///
/// # Safety
///
/// As [`exp2`].
#[inline(always)]
pub unsafe fn ffma<T: CoopElem, const N: usize>(
    a: &CoopVec<T, N>,
    b: &CoopVec<T, N>,
    c: &CoopVec<T, N>,
) -> CoopVec<T, N> {
    // SAFETY: forwarded contract.
    unsafe { op3::<{ op::FFMA }, T, N, T, N>(a, b, c) }
}

/// `OptixCoopVecMatMulASMGenerator<...>::generateASM`: the shared body of [`matmul_bias`] and
/// [`matmul`]; the scalar operand order is the header's.
///
/// # Safety
///
/// The caller's contract (see the public wrappers).
#[inline(always)]
unsafe fn matvec<
    O: CoopElem,
    I: CoopElem,
    const INTERP: OptixCoopVecElemType,
    const LAYOUT: OptixCoopVecMatrixLayout,
    const TRANSPOSE: bool,
    const N: usize,
    const K: usize,
    const MAT: OptixCoopVecElemType,
    const BIAS: OptixCoopVecElemType,
>(
    v: &CoopVec<I, K>,
    mat: u64,
    mat_off: u32,
    stride: u32,
    bias: u64,
    bias_off: u32,
) -> CoopVec<O, N> {
    let (oe, no, ie, ni) = (O::ELEM as u32, N as u32, I::ELEM as u32, K as u32);
    let (interp, layout, tr, me, be) = (INTERP as u32, LAYOUT as u32, TRANSPOSE as u32, MAT as u32, BIAS as u32);
    if by_ptr::<I>(K) || by_ptr::<O>(N) {
        let mut out = MaybeUninit::<[O; N]>::uninit();
        // SAFETY: caller's contract; `v` is live for the call and OptiX writes all `N` result
        // elements before returning, so `out` is initialized.
        unsafe {
            raw::matvecmul_ptr(
                oe, no, ie, ni, interp, no, ni, mat, mat_off, stride, layout, tr, me, bias, bias_off, be,
                v.addr(), out.as_mut_ptr() as u64,
            );
            CoopVec(out.assume_init())
        }
    } else if N <= 16 && K <= 16 {
        let i: [u32; 16] = pack(v);
        // SAFETY: caller's contract.
        let o = unsafe {
            regs16!(raw::matvecmul_16xi32;
                oe, no, ie, ni, interp, no, ni, mat, mat_off, stride, layout, tr, me, bias, bias_off, be; i)
        };
        unpack(&o)
    } else {
        let i: [u32; 64] = pack(v);
        // SAFETY: caller's contract.
        let o = unsafe {
            regs64!(raw::matvecmul_64xi32;
                oe, no, ie, ni, interp, no, ni, mat, mat_off, stride, layout, tr, me, bias, bias_off, be; i)
        };
        unpack(&o)
    }
}

/// Matrix-vector multiply plus bias: `matrix[N x K] * v[K] + bias[N]`
/// (`optixCoopVecMatMul` with bias).
///
/// Generic parameters follow the header's template order: output element `O`, input element
/// `I`, `INTERP` (`inputInterpretation`; the input is arithmetically cast to it when it differs
/// from `I`), `LAYOUT` (`matrixLayout`), `TRANSPOSE` (matrix stored as K x N), `N` (rows, the
/// output size), `K` (columns, the input size), `MAT` (`matrixElementType`) and `BIAS`
/// (`biasElementType`). The header's requirement "N must match VecTOut::size, K must match
/// VecTIn::size" is enforced by the types. `stride` is `rowColumnStrideInBytes` (the header's
/// default is `0`).
///
/// # Safety
///
/// Valid in all OptiX program types. The `I` / `INTERP` / `MAT` / `BIAS` / `O` combination must
/// be one of the supported configurations listed for `optixCoopVecMatMul` in `optix_device.h`
/// (signed input or matrix requires signed bias and output); FP8 matrices and `TRANSPOSE` require
/// an inferencing- or training-optimal layout, and `TRANSPOSE` only supports FP16 matrices.
/// `mat` is global memory holding the N x K matrix, 64-byte aligned, `mat_off` 64-byte aligned;
/// `bias` is global memory holding `N` elements (never null, it is dereferenced), 16-byte
/// aligned, `bias_off` 16-byte aligned; neither may be modified during the call. For row- or
/// column-major layouts `stride` must be 16-byte aligned, and tight packing is only assumed
/// when it is a compile-time constant `0`.
#[inline(always)]
pub unsafe fn matmul_bias<
    O: CoopElem,
    I: CoopElem,
    const INTERP: OptixCoopVecElemType,
    const LAYOUT: OptixCoopVecMatrixLayout,
    const TRANSPOSE: bool,
    const N: usize,
    const K: usize,
    const MAT: OptixCoopVecElemType,
    const BIAS: OptixCoopVecElemType,
>(
    v: &CoopVec<I, K>,
    mat: u64,
    mat_off: u32,
    bias: u64,
    bias_off: u32,
    stride: u32,
) -> CoopVec<O, N> {
    // SAFETY: forwarded contract.
    unsafe { matvec::<O, I, INTERP, LAYOUT, TRANSPOSE, N, K, MAT, BIAS>(v, mat, mat_off, stride, bias, bias_off) }
}

/// Matrix-vector multiply without bias: `matrix[N x K] * v[K]` (`optixCoopVecMatMul` without the
/// bias parameters; passes `OPTIX_COOP_VEC_ELEM_TYPE_UNKNOWN`, a null bias and offset `0`, as
/// the header does).
///
/// Generic parameters as [`matmul_bias`] without `BIAS`.
///
/// # Safety
///
/// As [`matmul_bias`], without the bias requirements.
#[inline(always)]
pub unsafe fn matmul<
    O: CoopElem,
    I: CoopElem,
    const INTERP: OptixCoopVecElemType,
    const LAYOUT: OptixCoopVecMatrixLayout,
    const TRANSPOSE: bool,
    const N: usize,
    const K: usize,
    const MAT: OptixCoopVecElemType,
>(
    v: &CoopVec<I, K>,
    mat: u64,
    mat_off: u32,
    stride: u32,
) -> CoopVec<O, N> {
    // SAFETY: forwarded contract.
    unsafe {
        matvec::<O, I, INTERP, LAYOUT, TRANSPOSE, N, K, MAT, { crate::sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_UNKNOWN }>(
            v, mat, mat_off, stride, 0, 0,
        )
    }
}

/// Atomically adds `v` component-wise into global memory at `out + off`
/// (`optixCoopVecReduceSumAccumulate`).
///
/// # Safety
///
/// Valid in all OptiX program types. `T` must be [`F16`] or `f32`; the memory at `out + off`
/// must hold `N` elements of type `T` (no conversion is performed) and `out + off` must be a
/// multiple of 4.
#[inline(always)]
pub unsafe fn reduce_sum_acc<T: CoopElem, const N: usize>(v: &CoopVec<T, N>, out: u64, off: u32) {
    let (e, n) = (T::ELEM as u32, N as u32);
    if by_ptr::<T>(N) {
        // SAFETY: forwarded contract; `v` is live for the call.
        unsafe { raw::reduce_sum_accumulate_ptr(e, n, out, off, v.addr()) }
    } else if N <= 16 {
        let ia: [u32; 16] = pack(v);
        // SAFETY: forwarded contract.
        unsafe { regs16!(raw::reduce_sum_accumulate_16xi32; e, n, out, off; ia) }
    } else {
        let ia: [u32; 64] = pack(v);
        // SAFETY: forwarded contract.
        unsafe { regs64!(raw::reduce_sum_accumulate_64xi32; e, n, out, off; ia) }
    }
}

/// Atomically adds the outer product `a * transpose(b)` (an `NA x NB` matrix in layout `LAYOUT`)
/// component-wise into global memory at `out + off` (`optixCoopVecOuterProductAccumulate`; the
/// header's default layout is `OPTIX_COOP_VEC_MATRIX_LAYOUT_TRAINING_OPTIMAL`, the only one OptiX
/// currently supports). `stride` is `rowColumnStrideInBytes` (`0`: natural stride; ignored for
/// optimal layouts).
///
/// # Safety
///
/// Valid in all OptiX program types. `A`, `B` and the matrix element type must all be [`F16`]
/// (no conversion is performed); `out + off` must be a multiple of 4 and hold the whole matrix.
#[inline(always)]
pub unsafe fn outer_product_acc<
    A: CoopElem,
    const NA: usize,
    B: CoopElem,
    const NB: usize,
    const LAYOUT: OptixCoopVecMatrixLayout,
>(
    a: &CoopVec<A, NA>,
    b: &CoopVec<B, NB>,
    out: u64,
    off: u32,
    stride: u32,
) {
    let (ae, an, be, bn, layout) = (A::ELEM as u32, NA as u32, B::ELEM as u32, NB as u32, LAYOUT as u32);
    if by_ptr::<A>(NA) || by_ptr::<B>(NB) {
        // SAFETY: forwarded contract; both operands are live for the call.
        unsafe { raw::outer_product_accumulate_ptr(ae, an, be, bn, out, off, layout, stride, a.addr(), b.addr()) }
    } else if NB <= 16 && NA <= 16 {
        let (ia, ib): ([u32; 16], [u32; 16]) = (pack(a), pack(b));
        // SAFETY: forwarded contract.
        unsafe { regs16!(raw::outer_product_accumulate_16xi32; ae, an, be, bn, out, off, layout, stride; ia, ib) }
    } else {
        let (ia, ib): ([u32; 64], [u32; 64]) = (pack(a), pack(b));
        // SAFETY: forwarded contract.
        unsafe { regs64!(raw::outer_product_accumulate_64xi32; ae, an, be, bn, out, off, layout, stride; ia, ib) }
    }
}

/// Size in bytes of an `N x K` matrix of `ELEM` in `LAYOUT`, rounded up to a multiple of 64
/// (`optixCoopVecGetMatrixSize`); matches the host's `optixCoopVecMatrixComputeSize`. Meant
/// for computing per-layer offsets on the device. `STRIDE` is `rowColumnStrideInBytes` for row-
/// and column-major layouts (`0`: tight packing). The header's defaults are
/// `LAYOUT = OPTIX_COOP_VEC_MATRIX_LAYOUT_INFERENCING_OPTIMAL` and `STRIDE = 0`.
///
/// # Safety
///
/// Valid in every program type ("available anywhere").
#[inline(always)]
pub unsafe fn matrix_size<
    const N: u32,
    const K: u32,
    const ELEM: OptixCoopVecElemType,
    const LAYOUT: OptixCoopVecMatrixLayout,
    const STRIDE: u32,
>() -> u32 {
    // SAFETY: forwarded contract.
    unsafe { raw::coop_vec_get_matrix_size(N, K, ELEM as u32, LAYOUT as u32, STRIDE) }
}
