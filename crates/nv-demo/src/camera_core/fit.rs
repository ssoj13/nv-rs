//! Frame / zoom-to-fit math (AABB in world space).
//!
//! Why: framing a selection needs one finite, panic-free answer for any bounds the host hands over.
//! How: the AABB collapses to a bounding sphere; the pivot distance is the tangent-cone distance
//! `r / sin(h)` for the binding half-angle, then [`PerspectiveSettings::clamp_distance`] applies
//! the host's zoom limits. Only a degenerate radius (zero or non-finite) falls back to
//! `DEGENERATE_RADIUS` (1.0); any positive finite radius, however tiny, frames itself.
//! Used by the rig `frame_*` / `FrameBounds` paths.

use glam::Vec3;

use crate::camera_core::projection::PerspectiveSettings;

/// Radius (world units) used in place of a degenerate one (a point AABB, or non-finite bounds).
const DEGENERATE_RADIUS: f32 = 1.0;
/// Smallest half field of view (radians) the fit divides by: keeps `sin(h)` nonzero.
const MIN_HALF_FOV: f32 = 1e-6;
/// Margin (radians) below a right angle for the half field of view: keeps `tan(h)` finite.
const HALF_FOV_RIGHT_ANGLE_MARGIN: f32 = 1e-4;
/// Smallest aspect ratio the fit uses: a zero / negative aspect still gives a finite width.
const MIN_ASPECT: f32 = 1e-6;

/// `radius` if it is positive and finite, else `DEGENERATE_RADIUS`.
fn usable_radius(radius: f32) -> f32 {
    if radius.is_finite() && radius > 0.0 {
        radius
    } else {
        DEGENERATE_RADIUS
    }
}

/// Union AABB → center and bounding sphere radius. A non-finite centre becomes the origin; a
/// zero or non-finite radius becomes 1.0 (`DEGENERATE_RADIUS`); a tiny valid radius is kept.
pub fn aabb_center_radius(min: Vec3, max: Vec3) -> (Vec3, f32) {
    let min_v = min.min(max);
    let max_v = min.max(max);
    let raw_center = (min_v + max_v) * 0.5;
    let center = if raw_center.is_finite() {
        raw_center
    } else {
        Vec3::ZERO
    };
    let radius = usable_radius((max_v - min_v).length() * 0.5);
    (center, radius)
}

/// Distance from pivot so a sphere of `radius` fits in a perspective view, clamped by
/// [`PerspectiveSettings::clamp_distance`] (a non-finite result becomes `distance_max`).
pub fn distance_to_frame_sphere(
    radius: f32,
    aspect: f32,
    fov_y: f32,
    margin: f32,
    settings: PerspectiveSettings,
) -> f32 {
    let safe_radius = usable_radius(radius);
    // A sphere is inside a view cone of half-angle `h` when the pivot distance is at least
    // `r / sin(h)` (the cone is tangent to the sphere), not `r / tan(h)`, which only places the
    // sphere's near silhouette on the cone axis plane and clips a wide fov.
    let half_y = crate::camera_core::num::clamp(
        fov_y * 0.5,
        MIN_HALF_FOV,
        std::f32::consts::FRAC_PI_2 - HALF_FOV_RIGHT_ANGLE_MARGIN,
    );
    let half_x = (half_y.tan() * aspect.max(MIN_ASPECT))
        .atan()
        .max(MIN_HALF_FOV);
    let dist_y = safe_radius / half_y.sin();
    let dist_x = safe_radius / half_x.sin();
    let d = dist_x.max(dist_y) * margin.max(1.0);
    if d.is_finite() {
        settings.clamp_distance(d)
    } else {
        settings.distance_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tangent-cone distance `r / sin(h)`. fov 90 deg, aspect 1, r = 2:
    /// h = 45 deg, sin = 0.70711 -> d = 2.8284 (the old `r / tan(h)` gave 2.0).
    #[test]
    fn sphere_fit_uses_sin_of_half_fov() {
        let s = PerspectiveSettings {
            distance_max: 1.0e6,
            ..PerspectiveSettings::default()
        };
        let d = distance_to_frame_sphere(2.0, 1.0, 90.0_f32.to_radians(), 1.0, s);
        assert!((d - 2.0 * std::f32::consts::SQRT_2).abs() < 1e-3, "d = {d}");
    }

    /// The sphere really fits: its centre-to-cone-axis tangent distance is `d * sin(h)` >= r
    /// for both axes, with equality on the binding one.
    #[test]
    fn sphere_fit_is_tangent_on_binding_axis() {
        let s = PerspectiveSettings {
            distance_max: 1.0e6,
            ..PerspectiveSettings::default()
        };
        let (fov, aspect, r) = (60.0_f32.to_radians(), 0.5, 3.0);
        let d = distance_to_frame_sphere(r, aspect, fov, 1.0, s);
        let half_x = ((fov * 0.5).tan() * aspect).atan();
        let binding = (fov * 0.5).min(half_x);
        assert!((d * binding.sin() - r).abs() < 1e-3, "d = {d}");
    }

    /// A tiny but valid selection keeps its own radius (no radius-1 fallback): with the
    /// default limits it frames at `distance_min`, and with no lower limit it frames at its
    /// own tangent distance, far closer than a radius-1 sphere would.
    #[test]
    fn tiny_aabb_frames_itself_at_the_clamp() {
        let (min, max) = (Vec3::splat(-1.0e-5), Vec3::splat(1.0e-5));
        let (center, r) = aabb_center_radius(min, max);
        assert_eq!(center, Vec3::ZERO);
        let expected_r = 3.0_f32.sqrt() * 1.0e-5;
        assert!((r - expected_r).abs() < 1e-9, "r = {r:e}");

        let fov = 90.0_f32.to_radians();
        let clamped = PerspectiveSettings::default();
        let d = distance_to_frame_sphere(r, 1.0, fov, 1.0, clamped);
        assert_eq!(
            d, clamped.distance_min,
            "tiny selection frames at the zoom-in limit"
        );

        let free = PerspectiveSettings {
            distance_min: 0.0,
            ..PerspectiveSettings::default()
        };
        let d = distance_to_frame_sphere(r, 1.0, fov, 1.0, free);
        let tangent = expected_r * std::f32::consts::SQRT_2;
        assert!(
            (d - tangent).abs() < 1e-8,
            "d = {d:e}, expected {tangent:e}"
        );
    }

    /// Only degenerate bounds fall back: a point AABB and non-finite bounds give the fallback
    /// radius (and a non-finite centre becomes the origin).
    #[test]
    fn degenerate_aabb_falls_back() {
        let p = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(aabb_center_radius(p, p), (p, DEGENERATE_RADIUS));
        let (c, r) = aabb_center_radius(Vec3::splat(-f32::INFINITY), Vec3::ZERO);
        assert_eq!((c, r), (Vec3::ZERO, DEGENERATE_RADIUS));
    }
}
