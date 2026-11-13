//! Transforms: the OptiX 9.1 device transform API ported from `optix_device_impl_transformations.h`
//! and the transform functions of `optix_device_impl.h`.
//!
//! **What.** Transform-list access (size, handles), typed from-handle getters (transform type,
//! static / matrix-motion / SRT-motion transforms, instance id / child / transforms, IAS instance
//! lookup, GAS motion range), the composed object-to-world and world-to-object matrices of a
//! transform list at a ray time, and point / vector / normal transforms.
//!
//! **Why.** The C++ header does this math in inline device code rather than in intrinsics, so a Rust
//! program has to carry its own copy; it is ported line by line (same operation order, same key
//! resolution, same quaternion nlerp, same inverse) so results match what an `nvcc`-built program
//! computes.
//!
//! **How.** Matrices are [`Matrix3x4`], OptiX's row-major `float[12]` (rows `m0, m1, m2` of the
//! header). Transform structs are read with plain pointer reads from the device addresses the
//! intrinsics return (the header's `cvta.to.global` + `ld.global.v4.u32` is the same load). Motion
//! transforms resolve the key interval from [`sys::OptixMotionOptions`], lerp the two keys and, for
//! SRT, renormalise the quaternion (nlerp, as the header does; there is no slerp). A list is composed
//! front (world side) to back (object side). `sqrt`, `floor` and `__fdividef` are issued as the PTX
//! `nvcc` emits for them (`sqrt.rn`, `cvt.rmi`, `div.approx`), since `core` has no float math.
//!
//! **Used by** closest-hit / any-hit / intersection programs (current-hit functions) and by
//! `hitobject` (the `*_from_list` functions over an outgoing hit object's transform list).

use crate::sys::{
    OptixMotionOptions, OptixSRTData, OptixSRTMotionTransform, OptixMatrixMotionTransform,
    OptixStaticTransform, OptixTransformType,
    OptixTransformType_OPTIX_TRANSFORM_TYPE_INSTANCE as TY_INSTANCE,
    OptixTransformType_OPTIX_TRANSFORM_TYPE_MATRIX_MOTION_TRANSFORM as TY_MATRIX_MOTION,
    OptixTransformType_OPTIX_TRANSFORM_TYPE_SRT_MOTION_TRANSFORM as TY_SRT_MOTION,
    OptixTransformType_OPTIX_TRANSFORM_TYPE_STATIC_TRANSFORM as TY_STATIC,
};
use crate::{Float3, raw};

/// Row-major 3x4 matrix (OptiX's float[12] layout).
pub type Matrix3x4 = [f32; 12];

/// The identity [`Matrix3x4`] (what the header returns for an empty list or an unknown type).
pub const IDENTITY: Matrix3x4 = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];

// ---------------------------------------------------------------------------------------------
// PTX float ops `core` lacks (no_std).
// ---------------------------------------------------------------------------------------------

/// `sqrt(x)`, correctly rounded: what `nvcc` emits for `sqrt` with the default `-prec-sqrt=true`.
#[inline(always)]
fn sqrt(x: f32) -> f32 {
    let r: f32;
    // SAFETY: register-only arithmetic, no memory access, valid in every program type.
    unsafe { ::cuda_device::ptx_asm!("sqrt.rn.f32 %0, %1;", out("=f") r, in("f") x, options(register_only)) };
    r
}

/// `floorf(x)`.
#[inline(always)]
fn floor(x: f32) -> f32 {
    let r: f32;
    // SAFETY: register-only arithmetic, no memory access, valid in every program type.
    unsafe { ::cuda_device::ptx_asm!("cvt.rmi.f32.f32 %0, %1;", out("=f") r, in("f") x, options(register_only)) };
    r
}

/// `__fdividef(a, b)`: the header resolves motion keys with the fast division, so this does too.
#[inline(always)]
fn fdiv(a: f32, b: f32) -> f32 {
    let r: f32;
    // SAFETY: register-only arithmetic, no memory access, valid in every program type.
    unsafe { ::cuda_device::ptx_asm!("div.approx.f32 %0, %1, %2;", out("=f") r, in("f") a, in("f") b, options(register_only)) };
    r
}

// ---------------------------------------------------------------------------------------------
// Pure matrix math (optix_impl).
// ---------------------------------------------------------------------------------------------

/// `optixMultiplyRowMatrix`: row `v` of an affine 3x4 (implicit `[0 0 0 1]` last row) times `b`.
#[inline(always)]
fn mul_row(v0: f32, v1: f32, v2: f32, v3: f32, b: &Matrix3x4) -> [f32; 4] {
    [
        v0 * b[0] + v1 * b[4] + v2 * b[8],
        v0 * b[1] + v1 * b[5] + v2 * b[9],
        v0 * b[2] + v1 * b[6] + v2 * b[10],
        v0 * b[3] + v1 * b[7] + v2 * b[11] + v3,
    ]
}

/// Affine product `a * b` (apply `b` first, then `a`), as the header's `m := trf * m`.
#[inline(always)]
pub fn mul(a: &Matrix3x4, b: &Matrix3x4) -> Matrix3x4 {
    let r0 = mul_row(a[0], a[1], a[2], a[3], b);
    let r1 = mul_row(a[4], a[5], a[6], a[7], b);
    let r2 = mul_row(a[8], a[9], a[10], a[11], b);
    [r0[0], r0[1], r0[2], r0[3], r1[0], r1[1], r1[2], r1[3], r2[0], r2[1], r2[2], r2[3]]
}

/// `optixInvertMatrix`: inverse of an affine 3x4 via the 3x3 cofactor inverse (no singularity
/// check, like the header: a singular matrix yields inf/NaN).
#[inline(always)]
pub fn invert(m: &Matrix3x4) -> Matrix3x4 {
    let (m0x, m0y, m0z, m0w) = (m[0], m[1], m[2], m[3]);
    let (m1x, m1y, m1z, m1w) = (m[4], m[5], m[6], m[7]);
    let (m2x, m2y, m2z, m2w) = (m[8], m[9], m[10], m[11]);

    let det3 = m0x * (m1y * m2z - m1z * m2y) - m0y * (m1x * m2z - m1z * m2x) + m0z * (m1x * m2y - m1y * m2x);
    let inv_det3 = 1.0 / det3;

    let i00 = inv_det3 * (m1y * m2z - m2y * m1z);
    let i01 = inv_det3 * (m0z * m2y - m2z * m0y);
    let i02 = inv_det3 * (m0y * m1z - m1y * m0z);

    let i10 = inv_det3 * (m1z * m2x - m2z * m1x);
    let i11 = inv_det3 * (m0x * m2z - m2x * m0z);
    let i12 = inv_det3 * (m0z * m1x - m1z * m0x);

    let i20 = inv_det3 * (m1x * m2y - m2x * m1y);
    let i21 = inv_det3 * (m0y * m2x - m2y * m0x);
    let i22 = inv_det3 * (m0x * m1y - m1x * m0y);

    let (b0, b1, b2) = (m0w, m1w, m2w);
    [
        i00, i01, i02, -i00 * b0 - i01 * b1 - i02 * b2,
        i10, i11, i12, -i10 * b0 - i11 * b1 - i12 * b2,
        i20, i21, i22, -i20 * b0 - i21 * b1 - i22 * b2,
    ]
}

/// `optixGetMatrixFromSrt`: the 3x4 matrix of an SRT key, `T * R * S` with the pivot folded into
/// the translation. The quaternion is assumed normalised (the header's assumption too).
#[inline(always)]
pub fn srt_to_matrix(srt: &OptixSRTData) -> Matrix3x4 {
    let (qx, qy, qz, qw) = (srt.qx, srt.qy, srt.qz, srt.qw);

    let sqw = qw * qw;
    let sqx = qx * qx;
    let sqy = qy * qy;
    let sqz = qz * qz;

    let xy = qx * qy;
    let zw = qz * qw;
    let xz = qx * qz;
    let yw = qy * qw;
    let yz = qy * qz;
    let xw = qx * qw;

    let mut m0x = sqx - sqy - sqz + sqw;
    let mut m0y = 2.0 * (xy - zw);
    let mut m0z = 2.0 * (xz + yw);

    let mut m1x = 2.0 * (xy + zw);
    let mut m1y = -sqx + sqy - sqz + sqw;
    let mut m1z = 2.0 * (yz - xw);

    let mut m2x = 2.0 * (xz - yw);
    let mut m2y = 2.0 * (yz + xw);
    let mut m2z = -sqx - sqy + sqz + sqw;

    // The statement order below is the header's: each step reads the columns the previous one left.
    let m0w = m0x * srt.pvx + m0y * srt.pvy + m0z * srt.pvz + srt.tx;
    let m1w = m1x * srt.pvx + m1y * srt.pvy + m1z * srt.pvz + srt.ty;
    let m2w = m2x * srt.pvx + m2y * srt.pvy + m2z * srt.pvz + srt.tz;

    m0z = m0x * srt.b + m0y * srt.c + m0z * srt.sz;
    m1z = m1x * srt.b + m1y * srt.c + m1z * srt.sz;
    m2z = m2x * srt.b + m2y * srt.c + m2z * srt.sz;

    m0y = m0x * srt.a + m0y * srt.sy;
    m1y = m1x * srt.a + m1y * srt.sy;
    m2y = m2x * srt.a + m2y * srt.sy;

    m0x *= srt.sx;
    m1x *= srt.sx;
    m2x *= srt.sx;

    [m0x, m0y, m0z, m0w, m1x, m1y, m1z, m1w, m2x, m2y, m2z, m2w]
}

/// `optixTransformPoint`: `m * [p, 1]`.
#[inline(always)]
pub fn transform_point(m: &Matrix3x4, p: Float3) -> Float3 {
    [
        m[0] * p[0] + m[1] * p[1] + m[2] * p[2] + m[3],
        m[4] * p[0] + m[5] * p[1] + m[6] * p[2] + m[7],
        m[8] * p[0] + m[9] * p[1] + m[10] * p[2] + m[11],
    ]
}

/// `optixTransformVector`: the 3x3 linear part of `m` times `v` (no translation).
#[inline(always)]
pub fn transform_vector(m: &Matrix3x4, v: Float3) -> Float3 {
    [
        m[0] * v[0] + m[1] * v[1] + m[2] * v[2],
        m[4] * v[0] + m[5] * v[1] + m[6] * v[2],
        m[8] * v[0] + m[9] * v[1] + m[10] * v[2],
    ]
}

/// `optixTransformNormal`: the **transpose** of the 3x3 linear part of `m` times `n`.
///
/// `m` must be the *inverse* of the transform applied to points, so this is the inverse-transpose
/// rule: to take a normal object -> world pass the world-to-object matrix, and world -> object pass
/// the object-to-world matrix. The result is not normalised.
#[inline(always)]
pub fn transform_normal(m: &Matrix3x4, n: Float3) -> Float3 {
    [
        m[0] * n[0] + m[4] * n[1] + m[8] * n[2],
        m[1] * n[0] + m[5] * n[1] + m[9] * n[2],
        m[2] * n[0] + m[6] * n[1] + m[10] * n[2],
    ]
}

/// `a * t0 + b * t1` per component (the header's `optixAddFloat4(optixMulFloat4(..))` per float4).
#[inline(always)]
fn lerp<const N: usize>(mut a: [f32; N], b: &[f32; N], t0: f32, t1: f32) -> [f32; N] {
    for (x, y) in a.iter_mut().zip(b.iter()) {
        *x = *x * t0 + *y * t1;
    }
    a
}

/// `optixResolveMotionKey`: `(local time in [0, 1], left key index)` of `globalt` in `opts`.
///
/// Needs `numKeys >= 2` (OptiX requires it for motion transforms); the key is clamped to the last
/// interval so `key + 1` is always a valid key.
#[inline(always)]
fn resolve_key(opts: &OptixMotionOptions, globalt: f32) -> (f32, i32) {
    let begin = opts.timeBegin;
    let end = opts.timeEnd;
    // `(float)(numKeys - 1)`: every u16 is exact in f32, so subtracting in float is the same value.
    let intervals = opts.numKeys as f32 - 1.0;

    // `f32::min/max` are IEEE minNum/maxNum like PTX `min/max.f32`, matching the header's
    // `fminf`/`fmaxf` including their NaN handling.
    let mut time = 0.0f32.max(intervals.min(intervals * fdiv(globalt - begin, end - begin)));
    // Kept from the header for the begin == end case.
    if time.is_nan() {
        time = 0.0;
    }

    let flt_key = floor(time).min(intervals - 1.0);
    (time - flt_key, flt_key as i32)
}

/// `optixLoadInterpolatedMatrixKey` + `optixGetInterpolatedTransformation(OptixMatrixMotionTransform)`.
///
/// # Safety
///
/// `p` must point to a valid matrix motion transform (`numKeys >= 2`) in device memory.
#[inline(always)]
unsafe fn matrix_motion_at(p: *const OptixMatrixMotionTransform, time: f32) -> Matrix3x4 {
    // SAFETY: `p` is a valid transform (caller contract); the field is plain data.
    let opts = unsafe { (&raw const (*p).motionOptions).read() };
    let (t1, key) = resolve_key(&opts, time);
    // SAFETY: the transform's key array really holds `numKeys` matrices (the struct declares 2 as
    // a minimum, the allocation is sized for all keys), and `key + 1 < numKeys`.
    let base = unsafe { (&raw const (*p).transform).cast::<Matrix3x4>().offset(key as isize) };
    // SAFETY: as above, `base` is in bounds.
    let m = unsafe { base.read() };
    // The branch avoids the second key's load when it is not needed, as the header does.
    if t1 > 0.0 {
        // SAFETY: `key + 1 < numKeys`, see `resolve_key`.
        let next = unsafe { base.add(1).read() };
        lerp(m, &next, 1.0 - t1, t1)
    } else {
        m
    }
}

/// `optixLoadInterpolatedSrtKey` + `optixGetInterpolatedTransformation(OptixSRTMotionTransform)`.
///
/// # Safety
///
/// `p` must point to a valid SRT motion transform (`numKeys >= 2`) in device memory.
#[inline(always)]
unsafe fn srt_motion_at(p: *const OptixSRTMotionTransform, time: f32) -> Matrix3x4 {
    // SAFETY: `p` is a valid transform (caller contract); the field is plain data.
    let opts = unsafe { (&raw const (*p).motionOptions).read() };
    let (t1, key) = resolve_key(&opts, time);
    // SAFETY: the key array holds `numKeys` `OptixSRTData` (16 f32 each, no padding), see
    // `matrix_motion_at`; `key + 1 < numKeys`.
    let base = unsafe { (&raw const (*p).srtData).cast::<[f32; 16]>().offset(key as isize) };
    // SAFETY: as above, `base` is in bounds.
    let mut d = unsafe { base.read() };
    if t1 > 0.0 {
        // SAFETY: `key + 1 < numKeys`.
        let next = unsafe { base.add(1).read() };
        d = lerp(d, &next, 1.0 - t1, t1);
        // nlerp: renormalise the lerped quaternion (qx, qy, qz, qw = words 9..12).
        let inv_len = 1.0 / sqrt(d[9] * d[9] + d[10] * d[10] + d[11] * d[11] + d[12] * d[12]);
        d[9] *= inv_len;
        d[10] *= inv_len;
        d[11] *= inv_len;
        d[12] *= inv_len;
    }
    let srt = OptixSRTData {
        sx: d[0],
        a: d[1],
        b: d[2],
        pvx: d[3],
        sy: d[4],
        c: d[5],
        pvy: d[6],
        sz: d[7],
        pvz: d[8],
        qx: d[9],
        qy: d[10],
        qz: d[11],
        qw: d[12],
        tx: d[13],
        ty: d[14],
        tz: d[15],
    };
    srt_to_matrix(&srt)
}

// ---------------------------------------------------------------------------------------------
// From-handle getters.
// ---------------------------------------------------------------------------------------------

/// `optixGetTransformTypeFromHandle`: the `OPTIX_TRANSFORM_TYPE_*` of a transform-list handle.
///
/// # Safety
///
/// Valid in all program types; `handle` must come from a transform list.
#[inline(always)]
pub unsafe fn transform_type(handle: u64) -> OptixTransformType {
    // SAFETY: forwarded contract.
    unsafe { raw::get_transform_type_from_handle(handle) as OptixTransformType }
}

/// `optixGetStaticTransformFromHandle`: pointer to the static transform, null if `handle` is not
/// an `OPTIX_TRANSFORM_TYPE_STATIC_TRANSFORM`.
///
/// # Safety
///
/// Valid in all program types.
#[inline(always)]
pub unsafe fn static_transform_ptr(handle: u64) -> *const OptixStaticTransform {
    // SAFETY: forwarded contract.
    unsafe { raw::get_static_transform_from_handle(handle) as *const OptixStaticTransform }
}

/// The static transform of `handle`, read by value.
///
/// # Safety
///
/// Valid in all program types; `handle` must be an `OPTIX_TRANSFORM_TYPE_STATIC_TRANSFORM`.
#[inline(always)]
pub unsafe fn static_transform(handle: u64) -> OptixStaticTransform {
    // SAFETY: non-null for a static-transform handle (caller contract); OptiX keeps it in device memory.
    unsafe { static_transform_ptr(handle).read() }
}

/// `optixGetSRTMotionTransformFromHandle`: pointer to the SRT motion transform, null if `handle`
/// is not an `OPTIX_TRANSFORM_TYPE_SRT_MOTION_TRANSFORM`. The struct declares 2 keys; the real
/// `srtData` array holds `motionOptions.numKeys`.
///
/// # Safety
///
/// Valid in all program types.
#[inline(always)]
pub unsafe fn srt_motion_transform_ptr(handle: u64) -> *const OptixSRTMotionTransform {
    // SAFETY: forwarded contract.
    unsafe { raw::get_srt_motion_transform_from_handle(handle) as *const OptixSRTMotionTransform }
}

/// `optixGetMatrixMotionTransformFromHandle`: pointer to the matrix motion transform, null if
/// `handle` is not an `OPTIX_TRANSFORM_TYPE_MATRIX_MOTION_TRANSFORM`. The struct declares 2 keys;
/// the real `transform` array holds `motionOptions.numKeys`.
///
/// # Safety
///
/// Valid in all program types.
#[inline(always)]
pub unsafe fn matrix_motion_transform_ptr(handle: u64) -> *const OptixMatrixMotionTransform {
    // SAFETY: forwarded contract.
    unsafe { raw::get_matrix_motion_transform_from_handle(handle) as *const OptixMatrixMotionTransform }
}

/// `optixGetInstanceIdFromHandle`: the instance's `instanceId`, 0 if `handle` is not an instance.
///
/// # Safety
///
/// Valid in all program types.
#[inline(always)]
pub unsafe fn instance_id(handle: u64) -> u32 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_instance_id_from_handle(handle) }
}

/// `optixGetInstanceChildFromHandle`: the instance's child traversable, 0 if `handle` is not an
/// instance.
///
/// # Safety
///
/// Valid in all program types.
#[inline(always)]
pub unsafe fn instance_child(handle: u64) -> u64 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_instance_child_from_handle(handle) }
}

/// `optixGetInstanceTransformFromHandle`: pointer to the instance's object-to-world matrix, null
/// if `handle` is not an instance.
///
/// # Safety
///
/// Valid in all program types.
#[inline(always)]
pub unsafe fn instance_transform_ptr(handle: u64) -> *const Matrix3x4 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_instance_transform_from_handle(handle) as *const Matrix3x4 }
}

/// `optixGetInstanceInverseTransformFromHandle`: pointer to the instance's world-to-object matrix,
/// null if `handle` is not an instance.
///
/// # Safety
///
/// Valid in all program types.
#[inline(always)]
pub unsafe fn instance_inverse_transform_ptr(handle: u64) -> *const Matrix3x4 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_instance_inverse_transform_from_handle(handle) as *const Matrix3x4 }
}

/// The instance's object-to-world matrix, read by value.
///
/// # Safety
///
/// Valid in all program types; `handle` must be an `OPTIX_TRANSFORM_TYPE_INSTANCE`.
#[inline(always)]
pub unsafe fn instance_transform(handle: u64) -> Matrix3x4 {
    // SAFETY: non-null for an instance handle (caller contract).
    unsafe { instance_transform_ptr(handle).read() }
}

/// The instance's world-to-object matrix, read by value.
///
/// # Safety
///
/// As [`instance_transform`].
#[inline(always)]
pub unsafe fn instance_inverse_transform(handle: u64) -> Matrix3x4 {
    // SAFETY: non-null for an instance handle (caller contract).
    unsafe { instance_inverse_transform_ptr(handle).read() }
}

/// `optixGetInstanceTraversableFromIAS`: the traversable of instance `index` of `ias`.
///
/// # Safety
///
/// Valid in all program types; `ias` must be built with
/// `OPTIX_BUILD_FLAG_ALLOW_RANDOM_INSTANCE_ACCESS`.
#[inline(always)]
pub unsafe fn instance_traversable_from_ias(ias: u64, index: u32) -> u64 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_instance_traversable_from_ias(ias, index) }
}

/// `optixGetGASMotionTimeBegin`: the motion begin time of a GAS.
///
/// # Safety
///
/// Valid in all program types; `gas` must be a GAS handle.
#[inline(always)]
pub unsafe fn gas_motion_time_begin(gas: u64) -> f32 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_gas_motion_time_begin(gas) }
}

/// `optixGetGASMotionTimeEnd`: the motion end time of a GAS.
///
/// # Safety
///
/// As [`gas_motion_time_begin`].
#[inline(always)]
pub unsafe fn gas_motion_time_end(gas: u64) -> f32 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_gas_motion_time_end(gas) }
}

/// `optixGetGASMotionStepCount`: the number of motion steps of a GAS.
///
/// # Safety
///
/// As [`gas_motion_time_begin`].
#[inline(always)]
pub unsafe fn gas_motion_step_count(gas: u64) -> u32 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_gas_motion_step_count(gas) }
}

/// `optixGetInterpolatedTransformationFromHandle`: the matrix of one transform-list entry at
/// `time`, object-to-world if `o2w`, else world-to-object. Motion transforms are interpolated and,
/// for world-to-object, inverted; instances and static transforms use their stored inverse;
/// any other type gives [`IDENTITY`].
///
/// # Safety
///
/// Valid in all program types; `handle` must come from a transform list.
#[inline(always)]
pub unsafe fn transform_from_handle(handle: u64, time: f32, o2w: bool) -> Matrix3x4 {
    // SAFETY: forwarded contract.
    let ty = unsafe { transform_type(handle) };

    if ty == TY_MATRIX_MOTION || ty == TY_SRT_MOTION {
        // SAFETY: the type check makes the getter non-null and the struct valid.
        let m = unsafe {
            if ty == TY_MATRIX_MOTION {
                matrix_motion_at(matrix_motion_transform_ptr(handle), time)
            } else {
                srt_motion_at(srt_motion_transform_ptr(handle), time)
            }
        };
        if o2w { m } else { invert(&m) }
    } else if ty == TY_INSTANCE || ty == TY_STATIC {
        // SAFETY: the type check makes the getters non-null; the matrices are plain data.
        unsafe {
            let p: *const Matrix3x4 = if ty == TY_INSTANCE {
                if o2w { instance_transform_ptr(handle) } else { instance_inverse_transform_ptr(handle) }
            } else {
                let s = static_transform_ptr(handle);
                if o2w { &raw const (*s).transform } else { &raw const (*s).invTransform }
            };
            p.read()
        }
    } else {
        IDENTITY
    }
}

// ---------------------------------------------------------------------------------------------
// Transform-list composition.
// ---------------------------------------------------------------------------------------------

/// Object-to-world matrix composed over a transform list (`size` entries, `handle_at(i)` gives entry i), evaluated at `time`.
///
/// Entry 0 is the world side: the result is `T[0] * T[1] * ... * T[size-1]`, built from the back
/// as the header does. An empty list gives [`IDENTITY`] (the public `optixGet*TransformMatrix`
/// behaviour).
///
/// # Safety
///
/// Valid in all program types as far as this function goes; `handle_at` must return handles of
/// one transform list and is only called with `i < size` (its own validity applies: current-hit
/// lists in IS / AH / CH, outgoing hit-object lists in RG / CH / MS / CC / DC).
#[inline(always)]
pub unsafe fn object_to_world_from_list(size: u32, handle_at: impl Fn(u32) -> u64, time: f32) -> Matrix3x4 {
    let mut m = IDENTITY;
    let mut first = true;
    for i in (0..size).rev() {
        // SAFETY: forwarded contract.
        let t = unsafe { transform_from_handle(handle_at(i), time, true) };
        m = if first { t } else { mul(&t, &m) };
        first = false;
    }
    m
}

/// World-to-object matrix, same inputs.
///
/// The result is `inv(T[size-1]) * ... * inv(T[0])`, built from the front as the header does;
/// each entry's inverse is its stored inverse (instances, static transforms) or the inverted
/// interpolated key (motion transforms). An empty list gives [`IDENTITY`].
///
/// # Safety
///
/// As [`object_to_world_from_list`].
#[inline(always)]
pub unsafe fn world_to_object_from_list(size: u32, handle_at: impl Fn(u32) -> u64, time: f32) -> Matrix3x4 {
    let mut m = IDENTITY;
    let mut first = true;
    for i in 0..size {
        // SAFETY: forwarded contract.
        let t = unsafe { transform_from_handle(handle_at(i), time, false) };
        m = if first { t } else { mul(&t, &m) };
        first = false;
    }
    m
}

// ---------------------------------------------------------------------------------------------
// Current hit (OptixIncomingHitObject).
// ---------------------------------------------------------------------------------------------

/// `optixGetTransformListSize`: the number of transforms on the current transform list.
///
/// # Safety
///
/// Valid in intersection, any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn transform_list_size() -> u32 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_transform_list_size() }
}

/// `optixGetTransformListHandle`: entry `index` of the current transform list.
///
/// # Safety
///
/// Valid in intersection, any-hit and closest-hit programs; `index < transform_list_size()`.
#[inline(always)]
pub unsafe fn transform_list_handle(index: u32) -> u64 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_transform_list_handle(index) }
}

/// `optixGetObjectToWorldTransformMatrix`: the current hit's object-to-world matrix at the ray time.
///
/// # Safety
///
/// Valid in intersection, any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn object_to_world_matrix() -> Matrix3x4 {
    // SAFETY: forwarded contract; the list composition only passes indices below the list size.
    let at = |i| unsafe { raw::get_transform_list_handle(i) };
    // SAFETY: forwarded contract.
    unsafe { object_to_world_from_list(raw::get_transform_list_size(), at, raw::get_ray_time()) }
}

/// `optixGetWorldToObjectTransformMatrix`: the current hit's world-to-object matrix at the ray time.
///
/// # Safety
///
/// Valid in intersection, any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn world_to_object_matrix() -> Matrix3x4 {
    // SAFETY: forwarded contract; the list composition only passes indices below the list size.
    let at = |i| unsafe { raw::get_transform_list_handle(i) };
    // SAFETY: forwarded contract.
    unsafe { world_to_object_from_list(raw::get_transform_list_size(), at, raw::get_ray_time()) }
}

/// `optixTransformPointFromWorldToObjectSpace`.
///
/// # Safety
///
/// Valid in intersection, any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn transform_point_from_world_to_object(p: Float3) -> Float3 {
    // The header returns the input untouched for an empty list (identity math could turn inf into NaN).
    // SAFETY: forwarded contract.
    if unsafe { raw::get_transform_list_size() } == 0 {
        return p;
    }
    // SAFETY: forwarded contract.
    transform_point(&unsafe { world_to_object_matrix() }, p)
}

/// `optixTransformVectorFromWorldToObjectSpace`.
///
/// # Safety
///
/// Valid in intersection, any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn transform_vector_from_world_to_object(v: Float3) -> Float3 {
    // SAFETY: forwarded contract.
    if unsafe { raw::get_transform_list_size() } == 0 {
        return v;
    }
    // SAFETY: forwarded contract.
    transform_vector(&unsafe { world_to_object_matrix() }, v)
}

/// `optixTransformNormalFromWorldToObjectSpace`: uses the object-to-world matrix (the inverse of
/// world-to-object) with [`transform_normal`].
///
/// # Safety
///
/// Valid in intersection, any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn transform_normal_from_world_to_object(n: Float3) -> Float3 {
    // SAFETY: forwarded contract.
    if unsafe { raw::get_transform_list_size() } == 0 {
        return n;
    }
    // SAFETY: forwarded contract.
    transform_normal(&unsafe { object_to_world_matrix() }, n)
}

/// `optixTransformPointFromObjectToWorldSpace`.
///
/// # Safety
///
/// Valid in intersection, any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn transform_point_from_object_to_world(p: Float3) -> Float3 {
    // SAFETY: forwarded contract.
    if unsafe { raw::get_transform_list_size() } == 0 {
        return p;
    }
    // SAFETY: forwarded contract.
    transform_point(&unsafe { object_to_world_matrix() }, p)
}

/// `optixTransformVectorFromObjectToWorldSpace`.
///
/// # Safety
///
/// Valid in intersection, any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn transform_vector_from_object_to_world(v: Float3) -> Float3 {
    // SAFETY: forwarded contract.
    if unsafe { raw::get_transform_list_size() } == 0 {
        return v;
    }
    // SAFETY: forwarded contract.
    transform_vector(&unsafe { object_to_world_matrix() }, v)
}

/// `optixTransformNormalFromObjectToWorldSpace`: uses the world-to-object matrix (the inverse of
/// object-to-world) with [`transform_normal`].
///
/// # Safety
///
/// Valid in intersection, any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn transform_normal_from_object_to_world(n: Float3) -> Float3 {
    // SAFETY: forwarded contract.
    if unsafe { raw::get_transform_list_size() } == 0 {
        return n;
    }
    // SAFETY: forwarded contract.
    transform_normal(&unsafe { world_to_object_matrix() }, n)
}
