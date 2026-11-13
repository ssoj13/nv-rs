//! Hit objects and Shader Execution Reordering (SER): [`traverse`] a ray without running
//! closest-hit / miss, inspect the resulting *outgoing hit object*, [`reorder`] threads by it, and
//! [`invoke`] its program later; or build one directly with [`make_hit`], [`make_miss`] and
//! [`make_nop`].
//!
//! **Why free functions.** OptiX keeps exactly one outgoing hit object per thread, in registers it
//! owns; every `optixHitObject*` call reads or replaces that implicit state. A Rust value owning
//! it would lie about lifetimes (any `make_*` / `traverse` / `invoke` silently replaces it), so
//! this module mirrors the C++ API one function per entry point. The only data type is
//! [`TraverseData`], the opaque serialization [`make_hit`] consumes.
//!
//! **How.** Each function wraps the `_optix_hitobject_*` intrinsic `optix_device_impl.h` uses.
//! [`traverse`] and [`invoke`] pass the 32 payload registers the way [`crate::trace::trace`] does.
//! Matrix and transform queries build the transform-list matrix with [`crate::transform`] from
//! the hit object's list and ray time, with the header's identity shortcut for an empty list.
//! The per-primitive vertex getters (`optixHitObjectGet*VertexData`, `...SphereData`,
//! `...RibbonNormal`) live with their incoming-hit twins in [`crate::geometry`].
//!
//! **Used by** raygen programs that sort work before shading (path tracers with SER), and by
//! closest-hit / miss / callable programs that query or rebuild hit objects.

use crate::payload::{Payload, from_words, to_words};
use crate::trace::{Ray, TraceOptions};
use crate::transform::{
    Matrix3x4, object_to_world_from_list, transform_normal, transform_point, transform_vector, world_to_object_from_list,
};
use crate::{Float3, raw, sys};

/// A serialized hit object (`OptixTraverseData`): what [`traverse_data`] captures and
/// [`make_hit`] rebuilds a hit from. Opaque words; only meaningful to OptiX.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TraverseData(pub [u32; 20]);

/// Traverses `ray` into `handle` like [`crate::trace::trace`], but runs no closest-hit or miss
/// program: the result becomes the outgoing hit object, to be queried, [`reorder`]ed and
/// [`invoke`]d later. Intersection and any-hit programs see `payload`; it comes back as they left
/// it. `opts.payload_type` selects the typed overload (`OPTIX_PAYLOAD_TYPE_DEFAULT` is the
/// untyped `optixTraverse`).
///
/// # Safety
///
/// Valid in raygen, closest-hit, miss, continuation-callable and direct-callable programs;
/// `handle` must be a live traversable of this pipeline's context, and the reached programs must
/// agree on `P`.
#[inline(always)]
pub unsafe fn traverse<P: Payload>(handle: u64, ray: &Ray, opts: &TraceOptions, payload: &mut P) {
    let p = to_words(payload);
    let [ox, oy, oz] = ray.origin;
    let [dx, dy, dz] = ray.direction;
    // SAFETY: forwarded contract; argument order is the header's `optixTraverse`.
    let r = unsafe {
        raw::hitobject_traverse(
            opts.payload_type, handle, ox, oy, oz, dx, dy, dz, ray.tmin, ray.tmax, ray.time, opts.visibility_mask, opts.ray_flags,
            opts.sbt_offset, opts.sbt_stride, opts.miss_index, P::WORDS,
            p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11], p[12], p[13], p[14], p[15],
            p[16], p[17], p[18], p[19], p[20], p[21], p[22], p[23], p[24], p[25], p[26], p[27], p[28], p[29], p[30], p[31],
        )
    };
    *payload = from_words(&r);
}

/// Reorders this thread among the launch by its outgoing hit object, plus the low
/// `num_hint_bits` bits of `hint` (they take sort-key bits from the hit object, so keep them
/// few; nop hit objects can use more; the maximum is implementation defined).
///
/// # Safety
///
/// Valid in raygen programs.
#[inline(always)]
pub unsafe fn reorder_hint(hint: u32, num_hint_bits: u32) {
    // SAFETY: forwarded contract.
    unsafe { raw::hitobject_reorder(hint, num_hint_bits) }
}

/// Reorders this thread by its outgoing hit object alone (no coherence hint).
///
/// # Safety
///
/// Valid in raygen programs.
#[inline(always)]
pub unsafe fn reorder() {
    // SAFETY: forwarded contract; zero hint bits is the header's hint-less overload.
    unsafe { raw::hitobject_reorder(0, 0) }
}

/// Runs closest-hit, miss or nothing for the outgoing hit object with `payload` (default payload
/// type), then resets the hit object to nop.
///
/// # Safety
///
/// Valid in raygen, closest-hit, miss and continuation-callable programs; the invoked program
/// must agree on `P`.
#[inline(always)]
pub unsafe fn invoke<P: Payload>(payload: &mut P) {
    // SAFETY: forwarded contract.
    unsafe { invoke_typed(sys::OptixPayloadTypeID_OPTIX_PAYLOAD_TYPE_DEFAULT as u32, payload) }
}

/// [`invoke`] with an explicit payload type (one `OPTIX_PAYLOAD_TYPE_ID_*`).
///
/// # Safety
///
/// As [`invoke`]; `payload_type` must be a type the module declares and the invoked program
/// accepts.
#[inline(always)]
pub unsafe fn invoke_typed<P: Payload>(payload_type: u32, payload: &mut P) {
    let p = to_words(payload);
    // SAFETY: forwarded contract; argument order is the header's `optixInvoke`.
    let r = unsafe {
        raw::hitobject_invoke(
            payload_type, P::WORDS,
            p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11], p[12], p[13], p[14], p[15],
            p[16], p[17], p[18], p[19], p[20], p[21], p[22], p[23], p[24], p[25], p[26], p[27], p[28], p[29], p[30], p[31],
        )
    };
    *payload = from_words(&r);
}

/// Rebuilds a hit as the outgoing hit object from a ray, the [`TraverseData`] of an earlier hit
/// object and its transform list (`optixMakeHitObject`). `ray_flags` is `OPTIX_RAY_FLAG_*` bits;
/// the ray's `tmax` is not used (the hit distance is part of `data`).
///
/// # Safety
///
/// Valid in raygen, closest-hit, miss and continuation-callable programs; `handle` must be the
/// traversable the data was traversed in, `data` must come from [`traverse_data`], and
/// `transforms` must be that hit's transform list ([`transform_list_handle`]), readable for as
/// long as this hit object is current.
#[inline(always)]
pub unsafe fn make_hit(handle: u64, ray: &Ray, ray_flags: u32, data: &TraverseData, transforms: &[u64]) {
    let [ox, oy, oz] = ray.origin;
    let [dx, dy, dz] = ray.direction;
    let (tmin, time) = (ray.tmin, ray.time);
    let d = &data.0;
    // A slice longer than `u32::MAX` handles cannot exist on the device (it would exceed memory).
    let n = transforms.len() as u32;
    // SAFETY: forwarded contract; argument order is the header's `optixMakeHitObject`.
    unsafe {
        raw::hitobject_make_with_traverse_data_v2(
            handle, ox, oy, oz, dx, dy, dz, tmin, time, ray_flags,
            d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7], d[8], d[9], d[10], d[11], d[12], d[13], d[14], d[15], d[16], d[17], d[18], d[19],
            transforms.as_ptr() as u64, n,
        )
    }
}

/// Makes a miss the outgoing hit object: [`invoke`] will run miss program `miss_index` for `ray`
/// (`optixMakeMissHitObject`). `ray_flags` is `OPTIX_RAY_FLAG_*` bits.
///
/// # Safety
///
/// Valid in raygen, closest-hit, miss and continuation-callable programs; `miss_index` must be a
/// miss record of the bound SBT.
#[inline(always)]
pub unsafe fn make_miss(miss_index: u32, ray: &Ray, ray_flags: u32) {
    let [ox, oy, oz] = ray.origin;
    let [dx, dy, dz] = ray.direction;
    // SAFETY: forwarded contract; argument order is the header's `optixMakeMissHitObject`.
    unsafe { raw::hitobject_make_miss_v2(miss_index, ox, oy, oz, dx, dy, dz, ray.tmin, ray.tmax, ray.time, ray_flags) }
}

/// Makes a nop the outgoing hit object: [`invoke`] runs nothing and the getters return zeros
/// (`optixMakeNopHitObject`).
///
/// # Safety
///
/// Valid in raygen, closest-hit, miss and continuation-callable programs.
#[inline(always)]
pub unsafe fn make_nop() {
    // SAFETY: forwarded contract.
    unsafe { raw::hitobject_make_nop() }
}

/// Serializes the outgoing hit object for a later [`make_hit`] (`optixHitObjectGetTraverseData`).
///
/// # Safety
///
/// Valid in raygen, closest-hit, miss, continuation-callable and direct-callable programs.
#[inline(always)]
pub unsafe fn traverse_data() -> TraverseData {
    // SAFETY: forwarded contract.
    TraverseData(unsafe { raw::hitobject_get_traverse_data() })
}

macro_rules! getters {
    ($($(#[$doc:meta])* $name:ident -> $t:ty = $raw:ident;)*) => {$(
        $(#[$doc])*
        ///
        /// # Safety
        ///
        /// Valid in raygen, closest-hit, miss, continuation-callable and direct-callable programs.
        #[inline(always)]
        pub unsafe fn $name() -> $t {
            // SAFETY: forwarded contract.
            unsafe { raw::$raw() }
        }
    )*};
}

getters! {
    /// The `instanceId` of the hit instance (`optixHitObjectGetInstanceId`); undefined for a miss.
    instance_id -> u32 = hitobject_get_instance_id;
    /// Index of the hit instance in its instance array (`optixHitObjectGetInstanceIndex`);
    /// undefined for a miss.
    instance_index -> u32 = hitobject_get_instance_idx;
    /// Index of the hit primitive in its build input (`optixHitObjectGetPrimitiveIndex`);
    /// undefined for a miss.
    primitive_index -> u32 = hitobject_get_primitive_idx;
    /// Number of transforms in the hit's transform list (`optixHitObjectGetTransformListSize`);
    /// 0 for a miss or nop.
    transform_list_size -> u32 = hitobject_get_transform_list_size;
    /// SBT GAS index (build input) of the hit primitive (`optixHitObjectGetSbtGASIndex`);
    /// undefined for a miss.
    sbt_gas_index -> u32 = hitobject_get_sbt_gas_idx;
    /// The 8-bit hit kind (`optixHitObjectGetHitKind`); undefined for a miss.
    hit_kind -> u32 = hitobject_get_hitkind;
    /// The ray's `tmin` (`optixHitObjectGetRayTmin`); 0 for nop.
    ray_tmin -> f32 = hitobject_get_ray_tmin;
    /// The closest hit distance for a hit, the ray's `tmax` for a miss, 0 for nop
    /// (`optixHitObjectGetRayTmax`).
    ray_tmax -> f32 = hitobject_get_ray_tmax;
    /// The ray's time (`optixHitObjectGetRayTime`); 0 for nop or without motion.
    ray_time -> f32 = hitobject_get_ray_time;
    /// The ray's `OPTIX_RAY_FLAG_*` bits (`optixHitObjectGetRayFlags`).
    ray_flags -> u32 = hitobject_get_ray_flags;
    /// SBT record index of the hit or miss program (`optixHitObjectGetSbtRecordIndex`); 0 for nop.
    sbt_record_index -> u32 = hitobject_get_sbt_record_index;
    /// Device address of the hit or miss program's SBT record data
    /// (`optixHitObjectGetSbtDataPointer`); 0 for nop.
    sbt_data_ptr -> u64 = hitobject_get_sbt_data_pointer;
    /// The GAS holding the hit primitive (`optixHitObjectGetGASTraversableHandle`); 0 unless a hit.
    gas_traversable_handle -> u64 = hitobject_get_gas_traversable_handle;
    /// The hit cluster's user id (`optixHitObjectGetClusterId`); `OPTIX_CLUSTER_ID_INVALID` for a
    /// non-cluster hit or a miss.
    cluster_id -> u32 = hitobject_get_cluster_id;
}

/// Whether the outgoing hit object is a hit (`optixHitObjectIsHit`).
///
/// # Safety
///
/// Valid in raygen, closest-hit, miss, continuation-callable and direct-callable programs.
#[inline(always)]
pub unsafe fn is_hit() -> bool {
    // SAFETY: forwarded contract.
    unsafe { raw::hitobject_is_hit() != 0 }
}

/// Whether the outgoing hit object is a miss (`optixHitObjectIsMiss`).
///
/// # Safety
///
/// As [`is_hit`].
#[inline(always)]
pub unsafe fn is_miss() -> bool {
    // SAFETY: forwarded contract.
    unsafe { raw::hitobject_is_miss() != 0 }
}

/// Whether the outgoing hit object is a nop, neither hit nor miss (`optixHitObjectIsNop`); a
/// thread that never made one has an implied nop.
///
/// # Safety
///
/// As [`is_hit`].
#[inline(always)]
pub unsafe fn is_nop() -> bool {
    // SAFETY: forwarded contract.
    unsafe { raw::hitobject_is_nop() != 0 }
}

/// Sets the SBT record index [`invoke`] will use (`optixHitObjectSetSbtRecordIndex`).
///
/// # Safety
///
/// As [`is_hit`]; the index must be a record of the bound SBT of the hit object's kind (hit group
/// for a hit, miss for a miss).
#[inline(always)]
pub unsafe fn set_sbt_record_index(index: u32) {
    // SAFETY: forwarded contract.
    unsafe { raw::hitobject_set_sbt_record_index(index) }
}

/// Handle `index` of the hit's transform list, outermost first
/// (`optixHitObjectGetTransformListHandle`); undefined for a miss.
///
/// # Safety
///
/// As [`is_hit`]; `index` below [`transform_list_size`].
#[inline(always)]
pub unsafe fn transform_list_handle(index: u32) -> u64 {
    // SAFETY: forwarded contract.
    unsafe { raw::hitobject_get_transform_list_handle(index) }
}

/// The ray origin given to [`traverse`], [`make_hit`] or [`make_miss`]
/// (`optixHitObjectGetWorldRayOrigin`); zero for nop.
///
/// # Safety
///
/// As [`is_hit`].
#[inline(always)]
pub unsafe fn world_ray_origin() -> Float3 {
    // SAFETY: forwarded contract.
    unsafe { [raw::hitobject_get_world_ray_origin_x(), raw::hitobject_get_world_ray_origin_y(), raw::hitobject_get_world_ray_origin_z()] }
}

/// The ray direction given to [`traverse`], [`make_hit`] or [`make_miss`]
/// (`optixHitObjectGetWorldRayDirection`); zero for nop.
///
/// # Safety
///
/// As [`is_hit`].
#[inline(always)]
pub unsafe fn world_ray_direction() -> Float3 {
    // SAFETY: forwarded contract.
    unsafe {
        [raw::hitobject_get_world_ray_direction_x(), raw::hitobject_get_world_ray_direction_y(), raw::hitobject_get_world_ray_direction_z()]
    }
}

/// Attribute register `I` (0..8) of the hit (`optixHitObjectGetAttribute_I`); undefined for a
/// miss.
///
/// # Safety
///
/// As [`is_hit`]; `I` below the pipeline's `numAttributeValues`.
#[inline(always)]
pub unsafe fn attribute<const I: u32>() -> u32 {
    const { assert!(I < 8, "OptiX has 8 attribute registers") };
    // SAFETY: forwarded contract; `I` is a compile-time slot, as the header's literal.
    unsafe { raw::hitobject_get_attribute(I) }
}

/// All attributes of the hit as one `Pod` value of up to 8 words.
///
/// # Safety
///
/// As [`attribute`]; the hit's intersection program must have reported an `A`.
#[inline(always)]
pub unsafe fn attributes<A: bytemuck::Pod>() -> A {
    const { assert!(size_of::<A>().is_multiple_of(4) && size_of::<A>() <= 32, "attributes are at most 8 words") };
    let n = size_of::<A>() / 4;
    // SAFETY: forwarded contract; each slot is a constant behind a folded guard.
    let w = unsafe {
        let mut w = [0u32; 8];
        if n > 0 { w[0] = attribute::<0>(); }
        if n > 1 { w[1] = attribute::<1>(); }
        if n > 2 { w[2] = attribute::<2>(); }
        if n > 3 { w[3] = attribute::<3>(); }
        if n > 4 { w[4] = attribute::<4>(); }
        if n > 5 { w[5] = attribute::<5>(); }
        if n > 6 { w[6] = attribute::<6>(); }
        if n > 7 { w[7] = attribute::<7>(); }
        w
    };
    let mut a = core::mem::MaybeUninit::<A>::uninit();
    // SAFETY: `A` is Pod and at most 32 bytes; all its bytes are written.
    unsafe {
        core::ptr::copy_nonoverlapping(w.as_ptr().cast::<u8>(), a.as_mut_ptr().cast::<u8>(), size_of::<A>());
        a.assume_init()
    }
}

/// Barycentrics `(u, v)` of a triangle hit (`optixHitObjectGetTriangleBarycentrics`).
///
/// # Safety
///
/// As [`is_hit`]; the hit's primitive type must be `OPTIX_PRIMITIVE_TYPE_TRIANGLE`.
#[inline(always)]
pub unsafe fn triangle_barycentrics() -> [f32; 2] {
    // SAFETY: forwarded contract.
    unsafe { raw::hitobject_get_triangle_barycentrics() }
}

/// The curve parameter of a curve hit (`optixHitObjectGetCurveParameter`).
///
/// # Safety
///
/// As [`is_hit`]; the hit's primitive type must be a curve type.
#[inline(always)]
pub unsafe fn curve_parameter() -> f32 {
    // SAFETY: forwarded contract.
    unsafe { raw::hitobject_get_curve_parameter() }
}

/// Ribbon `(u, v)` parameters (along the directrix, across the generator) of a flat-curve hit
/// (`optixHitObjectGetRibbonParameters`).
///
/// # Safety
///
/// As [`is_hit`]; the hit's primitive type must be `OPTIX_PRIMITIVE_TYPE_FLAT_QUADRATIC_BSPLINE`.
#[inline(always)]
pub unsafe fn ribbon_parameters() -> [f32; 2] {
    // SAFETY: forwarded contract.
    unsafe { raw::hitobject_get_ribbon_parameters() }
}

/// The 3x4 row-major identity (`optix_device_impl.h`'s result for an empty transform list).
const IDENTITY: Matrix3x4 = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];

/// Object-to-world matrix of a non-empty list of `n` transforms at the hit object's ray time.
///
/// # Safety
///
/// As [`is_hit`]; `n` is [`transform_list_size`] and nonzero.
#[inline(always)]
unsafe fn o2w(n: u32) -> Matrix3x4 {
    // SAFETY: forwarded contract; the list helper only asks for indices below `n`.
    let at = |i: u32| unsafe { raw::hitobject_get_transform_list_handle(i) };
    // SAFETY: forwarded contract; `n` is the list size and `at` reads that list.
    unsafe { object_to_world_from_list(n, at, ray_time()) }
}

/// World-to-object matrix of a non-empty list of `n` transforms at the hit object's ray time.
///
/// # Safety
///
/// As [`o2w`].
#[inline(always)]
unsafe fn w2o(n: u32) -> Matrix3x4 {
    // SAFETY: forwarded contract; the list helper only asks for indices below `n`.
    let at = |i: u32| unsafe { raw::hitobject_get_transform_list_handle(i) };
    // SAFETY: forwarded contract; `n` is the list size and `at` reads that list.
    unsafe { world_to_object_from_list(n, at, ray_time()) }
}

/// The world-to-object matrix (3x4, row-major) of the hit's transform list at its ray time;
/// identity for an empty list (`optixHitObjectGetWorldToObjectTransformMatrix`). Cost grows with
/// the list length.
///
/// # Safety
///
/// As [`is_hit`].
#[inline(always)]
pub unsafe fn world_to_object_matrix() -> Matrix3x4 {
    // SAFETY: forwarded contract.
    unsafe {
        match transform_list_size() {
            0 => IDENTITY,
            n => w2o(n),
        }
    }
}

/// The object-to-world matrix (3x4, row-major) of the hit's transform list at its ray time;
/// identity for an empty list (`optixHitObjectGetObjectToWorldTransformMatrix`).
///
/// # Safety
///
/// As [`is_hit`].
#[inline(always)]
pub unsafe fn object_to_world_matrix() -> Matrix3x4 {
    // SAFETY: forwarded contract.
    unsafe {
        match transform_list_size() {
            0 => IDENTITY,
            n => o2w(n),
        }
    }
}

/// `p` from world to the hit's object space
/// (`optixHitObjectTransformPointFromWorldToObjectSpace`).
///
/// # Safety
///
/// As [`is_hit`].
#[inline(always)]
pub unsafe fn point_to_object(p: Float3) -> Float3 {
    // SAFETY: forwarded contract.
    unsafe {
        match transform_list_size() {
            0 => p,
            n => transform_point(&w2o(n), p),
        }
    }
}

/// Vector `v` from world to object space (`optixHitObjectTransformVectorFromWorldToObjectSpace`).
///
/// # Safety
///
/// As [`is_hit`].
#[inline(always)]
pub unsafe fn vector_to_object(v: Float3) -> Float3 {
    // SAFETY: forwarded contract.
    unsafe {
        match transform_list_size() {
            0 => v,
            n => transform_vector(&w2o(n), v),
        }
    }
}

/// Normal `nrm` from world to object space
/// (`optixHitObjectTransformNormalFromWorldToObjectSpace`).
///
/// # Safety
///
/// As [`is_hit`].
#[inline(always)]
pub unsafe fn normal_to_object(nrm: Float3) -> Float3 {
    // SAFETY: forwarded contract.
    unsafe {
        match transform_list_size() {
            0 => nrm,
            // Normals take the inverse-transpose: the inverse of world-to-object is object-to-world,
            // which `transform_normal` multiplies transposed (the header passes the same matrix).
            n => transform_normal(&o2w(n), nrm),
        }
    }
}

/// `p` from the hit's object space to world (`optixHitObjectTransformPointFromObjectToWorldSpace`).
///
/// # Safety
///
/// As [`is_hit`].
#[inline(always)]
pub unsafe fn point_to_world(p: Float3) -> Float3 {
    // SAFETY: forwarded contract.
    unsafe {
        match transform_list_size() {
            0 => p,
            n => transform_point(&o2w(n), p),
        }
    }
}

/// Vector `v` from object space to world (`optixHitObjectTransformVectorFromObjectToWorldSpace`).
///
/// # Safety
///
/// As [`is_hit`].
#[inline(always)]
pub unsafe fn vector_to_world(v: Float3) -> Float3 {
    // SAFETY: forwarded contract.
    unsafe {
        match transform_list_size() {
            0 => v,
            n => transform_vector(&o2w(n), v),
        }
    }
}

/// Normal `nrm` from object space to world
/// (`optixHitObjectTransformNormalFromObjectToWorldSpace`).
///
/// # Safety
///
/// As [`is_hit`].
#[inline(always)]
pub unsafe fn normal_to_world(nrm: Float3) -> Float3 {
    // SAFETY: forwarded contract.
    unsafe {
        match transform_list_size() {
            0 => nrm,
            // The inverse of object-to-world is world-to-object, multiplied transposed.
            n => transform_normal(&w2o(n), nrm),
        }
    }
}
