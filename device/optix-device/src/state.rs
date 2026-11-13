//! Launch and ray state: launch index and size, the current ray in world and object space, and
//! the hit being processed (primitive, instance, SBT indices, hit kind, triangle barycentrics,
//! attributes, SBT record data), plus the program-control intrinsics (report / ignore an
//! intersection, terminate the ray).

use crate::{Float3, raw};

/// This invocation's launch index `[x, y, z]`.
///
/// # Safety
///
/// Valid in every program type.
#[inline(always)]
pub unsafe fn launch_index() -> [u32; 3] {
    // SAFETY: forwarded contract.
    unsafe { [raw::get_launch_index_x(), raw::get_launch_index_y(), raw::get_launch_index_z()] }
}

/// The launch size `[width, height, depth]`.
///
/// # Safety
///
/// Valid in every program type.
#[inline(always)]
pub unsafe fn launch_dimensions() -> [u32; 3] {
    // SAFETY: forwarded contract.
    unsafe { [raw::get_launch_dimension_x(), raw::get_launch_dimension_y(), raw::get_launch_dimension_z()] }
}

/// The current ray's world-space origin.
///
/// # Safety
///
/// Valid in intersection, any-hit, closest-hit and miss programs.
#[inline(always)]
pub unsafe fn world_ray_origin() -> Float3 {
    // SAFETY: forwarded contract.
    unsafe { [raw::get_world_ray_origin_x(), raw::get_world_ray_origin_y(), raw::get_world_ray_origin_z()] }
}

/// The current ray's world-space direction.
///
/// # Safety
///
/// As [`world_ray_origin`].
#[inline(always)]
pub unsafe fn world_ray_direction() -> Float3 {
    // SAFETY: forwarded contract.
    unsafe { [raw::get_world_ray_direction_x(), raw::get_world_ray_direction_y(), raw::get_world_ray_direction_z()] }
}

/// The current ray's origin in the space of the geometry being intersected.
///
/// # Safety
///
/// Valid in intersection and any-hit programs.
#[inline(always)]
pub unsafe fn object_ray_origin() -> Float3 {
    // SAFETY: forwarded contract.
    unsafe { [raw::get_object_ray_origin_x(), raw::get_object_ray_origin_y(), raw::get_object_ray_origin_z()] }
}

/// The current ray's direction in object space.
///
/// # Safety
///
/// As [`object_ray_origin`].
#[inline(always)]
pub unsafe fn object_ray_direction() -> Float3 {
    // SAFETY: forwarded contract.
    unsafe { [raw::get_object_ray_direction_x(), raw::get_object_ray_direction_y(), raw::get_object_ray_direction_z()] }
}

macro_rules! scalar_getters {
    ($($(#[$doc:meta])* $name:ident -> $t:ty = $raw:ident; valid: $valid:literal)*) => {$(
        $(#[$doc])*
        ///
        /// # Safety
        ///
        #[doc = concat!("Valid in ", $valid, ".")]
        #[inline(always)]
        pub unsafe fn $name() -> $t {
            // SAFETY: forwarded contract.
            unsafe { raw::$raw() }
        }
    )*};
}

scalar_getters! {
    /// The current ray's `tmin`.
    ray_tmin -> f32 = get_ray_tmin; valid: "intersection, any-hit, closest-hit and miss programs"
    /// The current ray's `tmax` (the closest hit so far in intersection and any-hit).
    ray_tmax -> f32 = get_ray_tmax; valid: "intersection, any-hit, closest-hit and miss programs"
    /// The current ray's time.
    ray_time -> f32 = get_ray_time; valid: "intersection, any-hit, closest-hit and miss programs"
    /// The current ray's `OPTIX_RAY_FLAG_*` bits.
    ray_flags -> u32 = get_ray_flags; valid: "intersection, any-hit, closest-hit and miss programs"
    /// The current ray's visibility mask.
    ray_visibility_mask -> u32 = get_ray_visibility_mask; valid: "intersection, any-hit, closest-hit and miss programs"
    /// Trace depth left before `maxTraceDepth`.
    remaining_trace_depth -> u32 = get_remaining_trace_depth; valid: "raygen, closest-hit, miss and callable programs"
    /// Index of the primitive hit within its build input.
    primitive_index -> u32 = read_primitive_idx; valid: "intersection, any-hit, closest-hit and exception programs"
    /// Index of the build input (geometry) hit within its GAS.
    sbt_gas_index -> u32 = read_sbt_gas_idx; valid: "intersection, any-hit, closest-hit and exception programs"
    /// The `instanceId` of the instance hit (0 without instances).
    instance_id -> u32 = read_instance_id; valid: "intersection, any-hit and closest-hit programs"
    /// Index of the instance hit in its instance array.
    instance_index -> u32 = read_instance_idx; valid: "intersection, any-hit and closest-hit programs"
    /// The hit kind (`OPTIX_HIT_KIND_*` for triangles, the reported kind for custom primitives).
    hit_kind -> u32 = get_hit_kind; valid: "any-hit and closest-hit programs"
    /// The cluster id of the cluster hit (cluster acceleration structures).
    cluster_id -> u32 = get_cluster_id; valid: "intersection, any-hit and closest-hit programs"
    /// The GAS that holds the hit primitive.
    gas_traversable_handle -> u64 = get_gas_traversable_handle; valid: "intersection, any-hit, closest-hit and exception programs"
    /// The curve parameter of a curve hit.
    curve_parameter -> f32 = get_curve_parameter; valid: "any-hit and closest-hit programs on curves"
    /// An undefined value: lets OptiX skip moving a payload word that is not set on this path.
    undefined -> u32 = undef_value; valid: "every program type"
}

/// Barycentrics `(u, v)` of a triangle hit (weights of vertices 1 and 2).
///
/// # Safety
///
/// Valid in any-hit and closest-hit programs for triangle hits.
#[inline(always)]
pub unsafe fn triangle_barycentrics() -> [f32; 2] {
    // SAFETY: forwarded contract.
    unsafe { raw::get_triangle_barycentrics() }
}

/// The ribbon `(u, v)` surface parameters of a flat-curve hit.
///
/// # Safety
///
/// Valid in any-hit and closest-hit programs for flat quadratic B-spline hits.
#[inline(always)]
pub unsafe fn ribbon_parameters() -> [f32; 2] {
    // SAFETY: forwarded contract.
    unsafe { raw::get_ribbon_parameters() }
}

/// The `OPTIX_PRIMITIVE_TYPE_*` a hit kind belongs to.
///
/// # Safety
///
/// Valid in every program type.
#[inline(always)]
pub unsafe fn primitive_type(hit_kind: u32) -> u32 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_primitive_type_from_hit_kind(hit_kind) }
}

/// Whether a hit kind is a back-face hit.
///
/// # Safety
///
/// Valid in every program type.
#[inline(always)]
pub unsafe fn is_back_face(hit_kind: u32) -> bool {
    // SAFETY: forwarded contract.
    unsafe { raw::get_backface_from_hit_kind(hit_kind) != 0 }
}

/// Attribute register `I` (0..8) reported by the intersection program (built-in triangles
/// report the barycentrics in 0 and 1).
///
/// # Safety
///
/// Valid in any-hit and closest-hit programs; `I` below the pipeline's `numAttributeValues`.
#[inline(always)]
pub unsafe fn attribute<const I: u32>() -> u32 {
    // SAFETY: forwarded contract; `I` is a constant.
    unsafe {
        match I {
            0 => raw::get_attribute_0(),
            1 => raw::get_attribute_1(),
            2 => raw::get_attribute_2(),
            3 => raw::get_attribute_3(),
            4 => raw::get_attribute_4(),
            5 => raw::get_attribute_5(),
            6 => raw::get_attribute_6(),
            7 => raw::get_attribute_7(),
            _ => const { panic!("OptiX has 8 attribute registers") },
        }
    }
}

/// All attributes as one `Pod` value of up to 8 words.
///
/// # Safety
///
/// As [`attribute`]; the intersection program must have reported an `A`.
#[inline(always)]
pub unsafe fn attributes<A: bytemuck::Pod>() -> A {
    const { assert!(size_of::<A>().is_multiple_of(4) && size_of::<A>() <= 32, "attributes are at most 8 words") };
    let n = size_of::<A>() / 4;
    // SAFETY: forwarded contract; each index is a constant behind a folded guard.
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
    let mut a = A::zeroed();
    // SAFETY: `A` is Pod and at most 32 bytes.
    unsafe { core::ptr::copy_nonoverlapping(w.as_ptr().cast::<u8>(), (&mut a as *mut A).cast::<u8>(), size_of::<A>()) };
    a
}

/// Reports an intersection at distance `t` with a user hit kind (0..=127) and attributes `A`
/// (up to 8 words); returns whether it was accepted.
///
/// # Safety
///
/// Valid in intersection programs; `A` must fit the pipeline's `numAttributeValues`.
#[inline(always)]
pub unsafe fn report_intersection<A: bytemuck::Pod>(t: f32, hit_kind: u32, attributes: &A) -> bool {
    const { assert!(size_of::<A>().is_multiple_of(4) && size_of::<A>() <= 32, "attributes are at most 8 words") };
    let mut a = [0u32; 8];
    // SAFETY: `A` is Pod and at most 32 bytes.
    unsafe { core::ptr::copy_nonoverlapping((attributes as *const A).cast::<u8>(), a.as_mut_ptr().cast::<u8>(), size_of::<A>()) };
    // SAFETY: forwarded contract; the attribute count is a compile-time constant.
    let r = unsafe {
        match size_of::<A>() / 4 {
            0 => raw::report_intersection_0(t, hit_kind),
            1 => raw::report_intersection_1(t, hit_kind, a[0]),
            2 => raw::report_intersection_2(t, hit_kind, a[0], a[1]),
            3 => raw::report_intersection_3(t, hit_kind, a[0], a[1], a[2]),
            4 => raw::report_intersection_4(t, hit_kind, a[0], a[1], a[2], a[3]),
            5 => raw::report_intersection_5(t, hit_kind, a[0], a[1], a[2], a[3], a[4]),
            6 => raw::report_intersection_6(t, hit_kind, a[0], a[1], a[2], a[3], a[4], a[5]),
            7 => raw::report_intersection_7(t, hit_kind, a[0], a[1], a[2], a[3], a[4], a[5], a[6]),
            _ => raw::report_intersection_8(t, hit_kind, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]),
        }
    };
    r != 0
}

/// Discards the current intersection and continues traversal.
///
/// # Safety
///
/// Valid in any-hit programs.
#[inline(always)]
pub unsafe fn ignore_intersection() {
    // SAFETY: forwarded contract.
    unsafe { raw::ignore_intersection() }
}

/// Accepts the current intersection and stops traversal (closest-hit runs on it).
///
/// # Safety
///
/// Valid in any-hit programs.
#[inline(always)]
pub unsafe fn terminate_ray() {
    // SAFETY: forwarded contract.
    unsafe { raw::terminate_ray() }
}

/// The user data of the current program's SBT record (the bytes after its 32-byte header).
///
/// # Safety
///
/// Valid in every program type except exception programs; the record must hold a `T` (the host
/// `optix::SbtRecord<T>`), and the reference must not outlive the launch.
#[inline(always)]
pub unsafe fn sbt_data<T>() -> &'static T {
    // SAFETY: forwarded contract; OptiX returns the record's data address.
    unsafe { &*(raw::get_sbt_data_ptr_64() as *const T) }
}
