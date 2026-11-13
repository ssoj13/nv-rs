//! Primitive geometry queries: control vertices of triangles, curves, ribbons and spheres, ribbon
//! normals, and the hit-kind helpers built on top of [`state::hit_kind`].
//!
//! **Why.** OptiX exposes the object-space data of the primitive being hit (or of any primitive
//! of a GAS built with `OPTIX_BUILD_FLAG_ALLOW_RANDOM_VERTEX_ACCESS`) through `_optix_*`
//! intrinsics that return flat float lists. These wrappers regroup them into one array per control
//! vertex, exactly as the `data[N]` out-parameters of `optix_device.h` read.
//!
//! **How.** Four flavours per primitive, mirroring the header:
//! - `<prim>_vertices()` -- the currently hit primitive (any-hit / closest-hit); needs no build flag;
//! - `<prim>_vertices_from(gas, prim, sbt_gas_index, time)` -- the header's deprecated random-access
//!   overload (`optixGet*VertexData(gas, primIdx, sbtGASIndex, time, data)`);
//! - `<prim>_vertices_from_handle(..)` -- `optixGet*VertexDataFromHandle`, same arguments;
//! - `hitobject_<prim>_vertices()` -- the outgoing hit object (`optixHitObjectGet*`).
//!
//! Curves return `[[f32; 4]; N]` (xyz position + radius per control point); triangles
//! `[Float3; 3]`; spheres `[f32; 4]` (center + radius). Rocaps curve types have no `gas`
//! overload in the header, so they only have the other three flavours.
//!
//! **Uses** [`crate::raw`], [`crate::sys`] and [`crate::state`]. **Used by** OptiX programs that
//! shade or intersect built-in primitives and by custom curve/sphere shading.

use crate::{Float3, raw, state, sys};

fn t3(a: [f32; 9]) -> [Float3; 3] {
    [[a[0], a[1], a[2]], [a[3], a[4], a[5]], [a[6], a[7], a[8]]]
}

fn q2(a: [f32; 8]) -> [[f32; 4]; 2] {
    [[a[0], a[1], a[2], a[3]], [a[4], a[5], a[6], a[7]]]
}

fn q3(a: [f32; 12]) -> [[f32; 4]; 3] {
    [[a[0], a[1], a[2], a[3]], [a[4], a[5], a[6], a[7]], [a[8], a[9], a[10], a[11]]]
}

fn q4(a: [f32; 16]) -> [[f32; 4]; 4] {
    [
        [a[0], a[1], a[2], a[3]],
        [a[4], a[5], a[6], a[7]],
        [a[8], a[9], a[10], a[11]],
        [a[12], a[13], a[14], a[15]],
    ]
}

/// Identity regrouping for data that is already one `float4` (spheres).
fn same4(a: [f32; 4]) -> [f32; 4] {
    a
}

// Argument-free flavours (current hit, hit object): `name = raw, regroup -> ret; doc, valid;`.
macro_rules! cur_fns {
    ($($name:ident = $raw:ident, $conv:ident -> $ret:ty; $doc:literal, $valid:literal;)*) => {$(
        #[doc = $doc]
        ///
        /// # Safety
        ///
        #[doc = concat!("Valid in ", $valid, "; the primitive must be of the matching type.")]
        #[inline(always)]
        pub unsafe fn $name() -> $ret {
            // SAFETY: forwarded contract.
            unsafe { $conv(raw::$raw()) }
        }
    )*};
}

// Random-access flavours (`gas`, `primIdx`, `sbtGASIndex`, `time`): `name = raw, regroup -> ret; doc;`.
macro_rules! gas_fns {
    ($($name:ident = $raw:ident, $conv:ident -> $ret:ty; $doc:literal;)*) => {$(
        #[doc = $doc]
        ///
        /// `gas` is the GAS traversable handle, `prim` the primitive index in build input
        /// `sbt_gas_index`; `time` is ignored when motion is disabled or the GAS has none.
        ///
        /// # Safety
        ///
        /// Valid in every program type; the GAS must have been built with
        /// `OPTIX_BUILD_FLAG_ALLOW_RANDOM_VERTEX_ACCESS`, and `prim` / `sbt_gas_index` must address
        /// an existing primitive of the matching type.
        #[inline(always)]
        pub unsafe fn $name(gas: u64, prim: u32, sbt_gas_index: u32, time: f32) -> $ret {
            // SAFETY: forwarded contract.
            unsafe { $conv(raw::$raw(gas, prim, sbt_gas_index, time)) }
        }
    )*};
}

cur_fns! {
    triangle_vertices = get_triangle_vertex_data_current_hit, t3 -> [Float3; 3];
        "Object-space vertices of the currently hit triangle at the current ray time (`optixGetTriangleVertexData(float3[3])`).", "any-hit and closest-hit programs";
    hitobject_triangle_vertices = hitobject_get_triangle_vertex_data, t3 -> [Float3; 3];
        "Object-space vertices of the triangle of the outgoing hit object (`optixHitObjectGetTriangleVertexData`).", "raygen, closest-hit, miss and callable programs";

    linear_curve_vertices = get_linear_curve_vertex_data_current_hit, q2 -> [[f32; 4]; 2];
        "Control points (xyz + radius) of the currently hit linear curve (`optixGetLinearCurveVertexData(float4[2])`).", "any-hit and closest-hit programs";
    hitobject_linear_curve_vertices = hitobject_get_linear_curve_vertex_data, q2 -> [[f32; 4]; 2];
        "Control points of the linear curve of the outgoing hit object (`optixHitObjectGetLinearCurveVertexData`).", "raygen, closest-hit, miss and callable programs";

    quadratic_bspline_vertices = get_quadratic_bspline_vertex_data_current_hit, q3 -> [[f32; 4]; 3];
        "Control points of the currently hit quadratic B-spline (`optixGetQuadraticBSplineVertexData(float4[3])`).", "any-hit and closest-hit programs";
    hitobject_quadratic_bspline_vertices = hitobject_get_quadratic_bspline_vertex_data, q3 -> [[f32; 4]; 3];
        "Control points of the quadratic B-spline of the outgoing hit object (`optixHitObjectGetQuadraticBSplineVertexData`).", "raygen, closest-hit, miss and callable programs";
    quadratic_bspline_rocaps_vertices = get_quadratic_bspline_rocaps_vertex_data_current_hit, q3 -> [[f32; 4]; 3];
        "Control points of the currently hit quadratic B-spline with rounded caps (`optixGetQuadraticBSplineRocapsVertexData`).", "any-hit and closest-hit programs";
    hitobject_quadratic_bspline_rocaps_vertices = hitobject_get_quadratic_bspline_rocaps_vertex_data, q3 -> [[f32; 4]; 3];
        "Control points of the rocaps quadratic B-spline of the outgoing hit object (`optixHitObjectGetQuadraticBSplineRocapsVertexData`).", "raygen, closest-hit, miss and callable programs";

    cubic_bspline_vertices = get_cubic_bspline_vertex_data_current_hit, q4 -> [[f32; 4]; 4];
        "Control points of the currently hit cubic B-spline (`optixGetCubicBSplineVertexData(float4[4])`).", "any-hit and closest-hit programs";
    hitobject_cubic_bspline_vertices = hitobject_get_cubic_bspline_vertex_data, q4 -> [[f32; 4]; 4];
        "Control points of the cubic B-spline of the outgoing hit object (`optixHitObjectGetCubicBSplineVertexData`).", "raygen, closest-hit, miss and callable programs";
    cubic_bspline_rocaps_vertices = get_cubic_bspline_rocaps_vertex_data_current_hit, q4 -> [[f32; 4]; 4];
        "Control points of the currently hit cubic B-spline with rounded caps (`optixGetCubicBSplineRocapsVertexData`).", "any-hit and closest-hit programs";
    hitobject_cubic_bspline_rocaps_vertices = hitobject_get_cubic_bspline_rocaps_vertex_data, q4 -> [[f32; 4]; 4];
        "Control points of the rocaps cubic B-spline of the outgoing hit object (`optixHitObjectGetCubicBSplineRocapsVertexData`).", "raygen, closest-hit, miss and callable programs";

    catmull_rom_vertices = get_catmullrom_vertex_data_current_hit, q4 -> [[f32; 4]; 4];
        "Control points of the currently hit Catmull-Rom spline (`optixGetCatmullRomVertexData(float4[4])`).", "any-hit and closest-hit programs";
    hitobject_catmull_rom_vertices = hitobject_get_catmullrom_vertex_data, q4 -> [[f32; 4]; 4];
        "Control points of the Catmull-Rom spline of the outgoing hit object (`optixHitObjectGetCatmullRomVertexData`).", "raygen, closest-hit, miss and callable programs";
    catmull_rom_rocaps_vertices = get_catmullrom_rocaps_vertex_data_current_hit, q4 -> [[f32; 4]; 4];
        "Control points of the currently hit Catmull-Rom spline with rounded caps (`optixGetCatmullRomRocapsVertexData`).", "any-hit and closest-hit programs";
    hitobject_catmull_rom_rocaps_vertices = hitobject_get_catmullrom_rocaps_vertex_data, q4 -> [[f32; 4]; 4];
        "Control points of the rocaps Catmull-Rom spline of the outgoing hit object (`optixHitObjectGetCatmullRomRocapsVertexData`).", "raygen, closest-hit, miss and callable programs";

    cubic_bezier_vertices = get_cubic_bezier_vertex_data_current_hit, q4 -> [[f32; 4]; 4];
        "Control points of the currently hit cubic Bezier curve (`optixGetCubicBezierVertexData(float4[4])`).", "any-hit and closest-hit programs";
    hitobject_cubic_bezier_vertices = hitobject_get_cubic_bezier_vertex_data, q4 -> [[f32; 4]; 4];
        "Control points of the cubic Bezier curve of the outgoing hit object (`optixHitObjectGetCubicBezierVertexData`).", "raygen, closest-hit, miss and callable programs";
    cubic_bezier_rocaps_vertices = get_cubic_bezier_rocaps_vertex_data_current_hit, q4 -> [[f32; 4]; 4];
        "Control points of the currently hit cubic Bezier curve with rounded caps (`optixGetCubicBezierRocapsVertexData`).", "any-hit and closest-hit programs";
    hitobject_cubic_bezier_rocaps_vertices = hitobject_get_cubic_bezier_rocaps_vertex_data, q4 -> [[f32; 4]; 4];
        "Control points of the rocaps cubic Bezier curve of the outgoing hit object (`optixHitObjectGetCubicBezierRocapsVertexData`).", "raygen, closest-hit, miss and callable programs";

    ribbon_vertices = get_ribbon_vertex_data_current_hit, q3 -> [[f32; 4]; 3];
        "Control points of the currently hit ribbon, a flat quadratic B-spline (`optixGetRibbonVertexData(float4[3])`).", "any-hit and closest-hit programs";
    hitobject_ribbon_vertices = hitobject_get_ribbon_vertex_data, q3 -> [[f32; 4]; 3];
        "Control points of the ribbon of the outgoing hit object (`optixHitObjectGetRibbonVertexData`).", "raygen, closest-hit, miss and callable programs";

    sphere_data = get_sphere_data_current_hit, same4 -> [f32; 4];
        "Center (xyz) and radius (w) of the currently hit sphere (`optixGetSphereData(float4[1])`).", "any-hit and closest-hit programs";
    hitobject_sphere_data = hitobject_get_sphere_data, same4 -> [f32; 4];
        "Center and radius of the sphere of the outgoing hit object (`optixHitObjectGetSphereData`).", "raygen, closest-hit, miss and callable programs";
}

gas_fns! {
    triangle_vertices_from = get_triangle_vertex_data, t3 -> [Float3; 3];
        "Random-access triangle vertices (deprecated overload `optixGetTriangleVertexData(gas, primIdx, sbtGASIndex, time, data)`).";
    triangle_vertices_from_handle = get_triangle_vertex_data_from_handle, t3 -> [Float3; 3];
        "Random-access triangle vertices (`optixGetTriangleVertexDataFromHandle`).";

    linear_curve_vertices_from = get_linear_curve_vertex_data, q2 -> [[f32; 4]; 2];
        "Random-access linear-curve control points (deprecated overload `optixGetLinearCurveVertexData(gas, ..)`).";
    linear_curve_vertices_from_handle = get_linear_curve_vertex_data_from_handle, q2 -> [[f32; 4]; 2];
        "Random-access linear-curve control points (`optixGetLinearCurveVertexDataFromHandle`).";

    quadratic_bspline_vertices_from = get_quadratic_bspline_vertex_data, q3 -> [[f32; 4]; 3];
        "Random-access quadratic B-spline control points (deprecated overload `optixGetQuadraticBSplineVertexData(gas, ..)`).";
    quadratic_bspline_vertices_from_handle = get_quadratic_bspline_vertex_data_from_handle, q3 -> [[f32; 4]; 3];
        "Random-access quadratic B-spline control points (`optixGetQuadraticBSplineVertexDataFromHandle`).";
    quadratic_bspline_rocaps_vertices_from_handle = get_quadratic_bspline_rocaps_vertex_data_from_handle, q3 -> [[f32; 4]; 3];
        "Random-access rocaps quadratic B-spline control points (`optixGetQuadraticBSplineRocapsVertexDataFromHandle`).";

    cubic_bspline_vertices_from = get_cubic_bspline_vertex_data, q4 -> [[f32; 4]; 4];
        "Random-access cubic B-spline control points (deprecated overload `optixGetCubicBSplineVertexData(gas, ..)`).";
    cubic_bspline_vertices_from_handle = get_cubic_bspline_vertex_data_from_handle, q4 -> [[f32; 4]; 4];
        "Random-access cubic B-spline control points (`optixGetCubicBSplineVertexDataFromHandle`).";
    cubic_bspline_rocaps_vertices_from_handle = get_cubic_bspline_rocaps_vertex_data_from_handle, q4 -> [[f32; 4]; 4];
        "Random-access rocaps cubic B-spline control points (`optixGetCubicBSplineRocapsVertexDataFromHandle`).";

    catmull_rom_vertices_from = get_catmullrom_vertex_data, q4 -> [[f32; 4]; 4];
        "Random-access Catmull-Rom control points (deprecated overload `optixGetCatmullRomVertexData(gas, ..)`).";
    catmull_rom_vertices_from_handle = get_catmullrom_vertex_data_from_handle, q4 -> [[f32; 4]; 4];
        "Random-access Catmull-Rom control points (`optixGetCatmullRomVertexDataFromHandle`).";
    catmull_rom_rocaps_vertices_from_handle = get_catmullrom_rocaps_vertex_data_from_handle, q4 -> [[f32; 4]; 4];
        "Random-access rocaps Catmull-Rom control points (`optixGetCatmullRomRocapsVertexDataFromHandle`).";

    cubic_bezier_vertices_from = get_cubic_bezier_vertex_data, q4 -> [[f32; 4]; 4];
        "Random-access cubic Bezier control points (deprecated overload `optixGetCubicBezierVertexData(gas, ..)`).";
    cubic_bezier_vertices_from_handle = get_cubic_bezier_vertex_data_from_handle, q4 -> [[f32; 4]; 4];
        "Random-access cubic Bezier control points (`optixGetCubicBezierVertexDataFromHandle`).";
    cubic_bezier_rocaps_vertices_from_handle = get_cubic_bezier_rocaps_vertex_data_from_handle, q4 -> [[f32; 4]; 4];
        "Random-access rocaps cubic Bezier control points (`optixGetCubicBezierRocapsVertexDataFromHandle`).";

    ribbon_vertices_from = get_ribbon_vertex_data, q3 -> [[f32; 4]; 3];
        "Random-access ribbon control points (deprecated overload `optixGetRibbonVertexData(gas, ..)`).";
    ribbon_vertices_from_handle = get_ribbon_vertex_data_from_handle, q3 -> [[f32; 4]; 3];
        "Random-access ribbon control points (`optixGetRibbonVertexDataFromHandle`).";

    sphere_data_from = get_sphere_data, same4 -> [f32; 4];
        "Random-access sphere center and radius (deprecated overload `optixGetSphereData(gas, ..)`).";
    sphere_data_from_handle = get_sphere_data_from_handle, same4 -> [f32; 4];
        "Random-access sphere center and radius (`optixGetSphereDataFromHandle`).";
}

/// Ribbon normal at the ribbon `(u, v)` parameters `uv` reported by the intersection program
/// (`optixGetRibbonNormal(float2)`).
///
/// # Safety
///
/// Valid in any-hit and closest-hit programs for ribbon hits.
#[inline(always)]
pub unsafe fn ribbon_normal(uv: [f32; 2]) -> Float3 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_ribbon_normal_current_hit(uv[0], uv[1]) }
}

/// Ribbon normal of the outgoing hit object (`optixHitObjectGetRibbonNormal(float2)`).
///
/// # Safety
///
/// Valid in raygen, closest-hit, miss and callable programs; the hit object must hold a ribbon hit.
#[inline(always)]
pub unsafe fn hitobject_ribbon_normal(uv: [f32; 2]) -> Float3 {
    // SAFETY: forwarded contract.
    unsafe { raw::hitobject_get_ribbon_normal(uv[0], uv[1]) }
}

/// Ribbon normal of a random-access primitive (deprecated overload
/// `optixGetRibbonNormal(gas, primIdx, sbtGASIndex, time, ribbonParameters)`).
///
/// # Safety
///
/// Valid in every program type; `gas`, `prim` and `sbt_gas_index` must address an existing ribbon
/// of a GAS built with `OPTIX_BUILD_FLAG_ALLOW_RANDOM_VERTEX_ACCESS`.
#[inline(always)]
pub unsafe fn ribbon_normal_from(gas: u64, prim: u32, sbt_gas_index: u32, time: f32, uv: [f32; 2]) -> Float3 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_ribbon_normal(gas, prim, sbt_gas_index, time, uv[0], uv[1]) }
}

/// Ribbon normal of a random-access primitive (`optixGetRibbonNormalFromHandle`).
///
/// # Safety
///
/// As [`ribbon_normal_from`].
#[inline(always)]
pub unsafe fn ribbon_normal_from_handle(gas: u64, prim: u32, sbt_gas_index: u32, time: f32, uv: [f32; 2]) -> Float3 {
    // SAFETY: forwarded contract.
    unsafe { raw::get_ribbon_normal_from_handle(gas, prim, sbt_gas_index, time, uv[0], uv[1]) }
}

/// The `OPTIX_PRIMITIVE_TYPE_*` of the current hit (`optixGetPrimitiveType()`).
///
/// # Safety
///
/// Valid in any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn hit_primitive_type() -> u32 {
    // SAFETY: forwarded contract; the hit kind is read from the current hit.
    unsafe { state::primitive_type(state::hit_kind()) }
}

/// Whether a hit kind is a front-face hit (`optixIsFrontFaceHit(hitKind)`).
///
/// # Safety
///
/// Valid in every program type.
#[inline(always)]
pub unsafe fn is_front_face(hit_kind: u32) -> bool {
    // SAFETY: forwarded contract.
    unsafe { !state::is_back_face(hit_kind) }
}

/// Whether the current hit is a back-face hit (`optixIsBackFaceHit()`).
///
/// # Safety
///
/// Valid in any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn is_back_face_hit() -> bool {
    // SAFETY: forwarded contract.
    unsafe { state::is_back_face(state::hit_kind()) }
}

/// Whether the current hit is a front-face hit (`optixIsFrontFaceHit()`).
///
/// # Safety
///
/// Valid in any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn is_front_face_hit() -> bool {
    // SAFETY: forwarded contract.
    unsafe { is_front_face(state::hit_kind()) }
}

/// Whether the current hit is a front-face built-in triangle hit (`optixIsTriangleFrontFaceHit()`).
///
/// # Safety
///
/// Valid in any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn is_triangle_front_face_hit() -> bool {
    // SAFETY: forwarded contract.
    unsafe { state::hit_kind() == sys::OptixHitKind_OPTIX_HIT_KIND_TRIANGLE_FRONT_FACE as u32 }
}

/// Whether the current hit is a back-face built-in triangle hit (`optixIsTriangleBackFaceHit()`).
///
/// # Safety
///
/// Valid in any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn is_triangle_back_face_hit() -> bool {
    // SAFETY: forwarded contract.
    unsafe { state::hit_kind() == sys::OptixHitKind_OPTIX_HIT_KIND_TRIANGLE_BACK_FACE as u32 }
}

/// Whether the current hit is a built-in triangle hit, front or back (`optixIsTriangleHit()`).
///
/// # Safety
///
/// Valid in any-hit and closest-hit programs.
#[inline(always)]
pub unsafe fn is_triangle_hit() -> bool {
    // SAFETY: forwarded contract.
    unsafe { is_triangle_front_face_hit() || is_triangle_back_face_hit() }
}
