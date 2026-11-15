//! Orientation policies for orbit rigs (storage is always [`glam::Quat`]).

use glam::{Quat, Vec3};

use crate::camera_core::convention::{DEFAULT_OFFSET_AXIS, WORLD_UP};

/// How far short of straight up / down the default pitch limit stops (radians, ~0.06°): the
/// turntable's pitch axis (`forward × WORLD_UP`) stays defined, while a snapped Top / Bottom view
/// (exactly ±90°, [`crate::camera_core::OrbitRig::animate_to_view`]) lies only this much beyond the limit, so
/// the next orbit moves it by its delta instead of jumping down to a far lower limit.
pub const POLE_EPS: f32 = 1e-3;

/// Max |pitch| in radians for turntable / quaternion policies: π/2 − [`POLE_EPS`], close enough
/// to the pole that an exact Top / Bottom view orbits on by the drag amount, never jumping to a
/// lower limit first.
pub const DEFAULT_PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - POLE_EPS;

/// How mouse orbit deltas are applied to a unit quaternion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OrientationMode {
    /// Yaw around world +Y, then pitch around camera-local Z (legacy; can gimbal near poles).
    Quaternion,
    /// Turntable: yaw world Y, pitch around camera right (no gimbal at horizon).
    #[default]
    HorizontalPlaneEuler,
    /// Explicit yaw-then-pitch euler rebuild (documented order: Y then Z in offset space).
    Euler,
}

/// The part of the elevation step `delta` that keeps the elevation `pitch` within
/// `±pitch_limit`; a pitch already beyond the limit (a snapped Top / Bottom view at exactly ±90°)
/// may only move back toward the horizon. The ONE pitch-step rule of the orbit rigs
/// ([`apply_orbit_delta`]) and the free-pose orbit ([`crate::camera_core::orbit_about`]).
///
/// Limiting the STEP (not the result) is what keeps a rotation from crossing the pole: a turn by
/// `a` changes the elevation by at most `|a|`, so a path that ends at most at the limit
/// (< π/2) never passes over the pole, where the result would read back as `π − pitch` with the
/// yaw + π (a 180° flip that no clamp of the result can see). Never panics: the range always
/// contains `pitch`, so a negative / NaN limit just blocks the step.
pub(crate) fn limit_pitch_step(pitch: f32, delta: f32, pitch_limit: f32) -> f32 {
    crate::camera_core::num::clamp(
        pitch + delta,
        pitch.min(-pitch_limit),
        pitch.max(pitch_limit),
    ) - pitch
}

/// The horizontal turntable pitch axis of `orientation` (unit; positive turns raise the offset):
/// `offset × WORLD_UP`, or at an exact pole (a snapped Top / Bottom view), where that cross
/// vanishes, the flattened turntable right `orientation * Z` (`Ry(yaw) * Z` is unchanged by the
/// pitch, [`orientation_from_yaw_pitch`], so it still carries the heading). Zero only for an
/// orientation whose local Z is vertical (a rolled quaternion at a pole). Shared by
/// [`apply_orbit_delta`] and [`crate::camera_core::OrbitPose::view_plane_axes`] (screen right = −this), so the
/// orbit, the pan axes and `OrbitPose::view_matrix` (`orientation * WORLD_UP` at the pole) agree.
pub(crate) fn turntable_right(orientation: Quat) -> Vec3 {
    let offset = (orientation * DEFAULT_OFFSET_AXIS).normalize_or_zero();
    let axis = offset.cross(WORLD_UP);
    if axis.length_squared() > 1e-8 {
        axis.normalize()
    } else {
        let r = orientation * Vec3::Z;
        Vec3::new(r.x, 0.0, r.z).normalize_or_zero()
    }
}

/// Apply orbit mouse deltas to `orientation`. The pitch step is limited first
/// (`limit_pitch_step`), so an upward orbit at the top limit (or from an exact Top snap) stops
/// there instead of crossing the pole and flipping the view 180°. A pose already beyond
/// `pitch_limit` moves back by the drag amount and is never snapped to the limit nor pushed
/// further out, in every mode.
pub fn apply_orbit_delta(
    mode: OrientationMode,
    orientation: &mut Quat,
    yaw_delta: f32,
    pitch_delta: f32,
    pitch_limit: f32,
) {
    match mode {
        OrientationMode::Quaternion | OrientationMode::HorizontalPlaneEuler => {
            if yaw_delta != 0.0 {
                *orientation = (Quat::from_rotation_y(yaw_delta) * *orientation).normalize();
            }
            let (_, pitch) = yaw_pitch_from_orientation(*orientation);
            let start_pitch = pitch;
            let pitch_delta = limit_pitch_step(pitch, pitch_delta, pitch_limit);
            if pitch_delta != 0.0 {
                let pitch_axis = if mode == OrientationMode::HorizontalPlaneEuler {
                    turntable_right(*orientation)
                } else {
                    (*orientation * Vec3::Z).normalize_or_zero()
                };
                if pitch_axis.length_squared() > 1e-8 {
                    *orientation =
                        (Quat::from_axis_angle(pitch_axis, pitch_delta) * *orientation).normalize();
                }
            }
            clamp_pitch_offset(orientation, start_pitch, pitch_limit);
        }
        OrientationMode::Euler => {
            let (yaw, pitch) = yaw_pitch_from_orientation(*orientation);
            let yaw = yaw + yaw_delta;
            // Rebuilt from yaw / pitch, so it cannot cross the pole; the same step rule as the
            // other modes keeps a pose beyond the limit where it is (no snap to the limit).
            let pitch = pitch + limit_pitch_step(pitch, pitch_delta, pitch_limit);
            *orientation = orientation_from_yaw_pitch(yaw, pitch);
        }
    }
}

/// Turntable orientation: yaw about world +Y, pitch about camera right after yaw.
pub fn orientation_from_yaw_pitch(yaw: f32, pitch: f32) -> Quat {
    let yaw_q = Quat::from_rotation_y(yaw);
    let offset = yaw_q * DEFAULT_OFFSET_AXIS;
    let mut right = offset.cross(WORLD_UP);
    if right.length_squared() < 1e-8 {
        right = Vec3::Z;
    } else {
        right = right.normalize();
    }
    (Quat::from_axis_angle(right, pitch) * yaw_q).normalize()
}

/// The turntable `(yaw, pitch)` of `orientation` (inverse of [`orientation_from_yaw_pitch`]):
/// the heading and elevation of its offset direction; straight up / down the yaw comes from the
/// horizontal right axis, so a Top / Bottom view keeps its heading.
pub fn yaw_pitch_from_orientation(orientation: Quat) -> (f32, f32) {
    let dir = orientation * DEFAULT_OFFSET_AXIS;
    let horiz = Vec3::new(dir.x, 0.0, dir.z).length();
    let pitch = dir.y.atan2(horiz.max(1e-6));
    // Straight up or down the offset has no heading; the horizontal right axis of a
    // turntable orientation ([`orientation_from_yaw_pitch`]: `Ry(yaw) * Z`, unchanged by the
    // pitch) still has it, so a top or bottom view keeps its yaw.
    let yaw = if horiz > 1e-4 {
        (-dir.z).atan2(dir.x)
    } else {
        let right = orientation * Vec3::Z;
        right.x.atan2(right.z)
    };
    (yaw, pitch)
}

/// Cone half-angle (radians, 15°) within which [`crate::camera_core::OrbitSnapMode::Smart`] also locks the
/// middle direction component, snapping to the intersection of two coordinate planes (a world
/// axis).
pub const SMART_AXIS_SNAP_RAD: f32 = 15.0_f32.to_radians();

/// Project a unit view/offset direction `dir` using the unified 2-plane / axis lock-on rule:
/// 1. Always zero the smallest magnitude component (`|dx|, |dy|, |dz|`), locking to the
///    nearest coordinate plane (`XY`, `XZ`, or `YZ`).
/// 2. If the middle magnitude component is within [`SMART_AXIS_SNAP_RAD`] (15°, i.e.
///    `|d_mid| < sin(15°)`) of a second coordinate plane, also zero the middle component,
///    locking to the intersection of both planes (the nearest world axis `±X`, `±Y`, `±Z`).
pub fn smart_snap_direction(dir: Vec3) -> Vec3 {
    let d = dir.normalize_or_zero();
    if d == Vec3::ZERO {
        return DEFAULT_OFFSET_AXIS;
    }
    let mut a = d.to_array();
    let abs = [a[0].abs(), a[1].abs(), a[2].abs()];
    let mut order = [0usize, 1, 2];
    order.sort_by(|&i, &j| abs[i].total_cmp(&abs[j]));
    // Always project onto the nearest coordinate plane (zero smallest component).
    a[order[0]] = 0.0;
    // If within 15° of a second coordinate plane, lock onto their intersection (world axis).
    if abs[order[1]] < SMART_AXIS_SNAP_RAD.sin() {
        a[order[1]] = 0.0;
    }
    Vec3::from_array(a).normalize_or_zero()
}

/// Smart-snap a turntable `(yaw, pitch)` to the nearest coordinate plane, or to the nearest
/// world axis when within [`SMART_AXIS_SNAP_RAD`] (15°) of two coordinate planes. At the
/// `±Y` poles, `yaw` is snapped to the nearest quarter-turn so top/bottom views align with
/// world axes.
pub fn smart_snap_yaw_pitch(yaw: f32, pitch: f32) -> (f32, f32) {
    use std::f32::consts::FRAC_PI_2;
    let dir = orientation_from_yaw_pitch(yaw, pitch) * DEFAULT_OFFSET_AXIS;
    let projected = smart_snap_direction(dir);
    let snapped_yaw = if projected.x * projected.x + projected.z * projected.z > 1e-10 {
        (-projected.z).atan2(projected.x)
    } else {
        (yaw / FRAC_PI_2).round() * FRAC_PI_2
    };
    (snapped_yaw, projected.y.clamp(-1.0, 1.0).asin())
}

/// The turntable yaw / pitch of the world view nearest to the free `yaw` / `pitch` (of the
/// offset direction `orientation_from_yaw_pitch(yaw, pitch) * DEFAULT_OFFSET_AXIS`): the axis
/// of its largest component with the yaw in quarter turns (`Axis`, so a top / bottom view is
/// square to the world), the direction with its smallest component projected out (`Plane`,
/// continuous within the plane), or the unified 2-plane / 15°-axis cone rule (`Smart`).
pub fn snapped_yaw_pitch(
    yaw: f32,
    pitch: f32,
    mode: crate::camera_core::OrbitSnapMode,
) -> (f32, f32) {
    use crate::camera_core::OrbitSnapMode;
    use std::f32::consts::FRAC_PI_2;
    let dir = orientation_from_yaw_pitch(yaw, pitch) * DEFAULT_OFFSET_AXIS;
    match mode {
        OrbitSnapMode::Axis => {
            let quarter = (yaw / FRAC_PI_2).round() * FRAC_PI_2;
            let snapped = if dir.y.abs() > dir.x.abs().max(dir.z.abs()) {
                FRAC_PI_2.copysign(dir.y)
            } else {
                0.0
            };
            (quarter, snapped)
        }
        OrbitSnapMode::Plane => {
            // The smallest component measures the angular distance to its perpendicular
            // coordinate plane: project it out, keep the other two continuous.
            let mut projected = dir;
            let abs = dir.abs();
            if abs.y <= abs.x.min(abs.z) {
                projected.y = 0.0;
            } else if abs.x <= abs.z {
                projected.x = 0.0;
            } else {
                projected.z = 0.0;
            }
            // A zero `dir` (non-finite input upstream) must not turn into NaN here.
            projected = projected.normalize_or_zero();
            let yaw = if projected.x * projected.x + projected.z * projected.z > 1e-10 {
                (-projected.z).atan2(projected.x)
            } else {
                yaw
            };
            (yaw, projected.y.clamp(-1.0, 1.0).asin())
        }
        OrbitSnapMode::Smart => smart_snap_yaw_pitch(yaw, pitch),
    }
}

/// Pull the elevation of `orientation` back into the range [`limit_pitch_step`] allows from
/// `start_pitch` (the elevation before the step): `±pitch_limit`, widened to `start_pitch` when
/// that already lies beyond it, so a pose past the limit is never snapped onto it. Only a
/// rolled (non-turntable) quaternion, whose pitch axis is not horizontal, needs it.
fn clamp_pitch_offset(orientation: &mut Quat, start_pitch: f32, pitch_limit: f32) {
    let (yaw, pitch) = yaw_pitch_from_orientation(*orientation);
    // `pitch_limit` is a host setting: a negative / NaN one must not panic.
    let clamped = crate::camera_core::num::clamp(
        pitch,
        start_pitch.min(-pitch_limit),
        start_pitch.max(pitch_limit),
    );
    if (pitch - clamped).abs() < 1e-5 {
        return;
    }
    *orientation = orientation_from_yaw_pitch(yaw, clamped);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_orbit_delta_survives_bad_pitch_limits() {
        // `f32::clamp` panicked on a negative / NaN pitch limit.
        for mode in [
            OrientationMode::Quaternion,
            OrientationMode::HorizontalPlaneEuler,
            OrientationMode::Euler,
        ] {
            for limit in [-1.0, f32::NAN] {
                let mut q = orientation_from_yaw_pitch(0.2, 0.3);
                apply_orbit_delta(mode, &mut q, 0.1, 0.2, limit);
                assert!(q.is_finite(), "{mode:?} limit {limit}");
            }
        }
    }

    /// An exact Top / Bottom view (pitch ±90°, a view snap) orbits on without a jump: the
    /// first pitch step moves by the step (the pole has a pitch axis), the heading is kept, and
    /// a yaw-only orbit stays at the pole (beyond the limit: never snapped onto it).
    #[test]
    fn a_snapped_pole_orbits_without_a_jump() {
        use std::f32::consts::FRAC_PI_2;
        const { assert!(DEFAULT_PITCH_LIMIT > 1.5 && DEFAULT_PITCH_LIMIT < FRAC_PI_2) };
        for (pole, step) in [(FRAC_PI_2, -0.1), (-FRAC_PI_2, 0.1)] {
            let mut q = orientation_from_yaw_pitch(0.7, pole);
            apply_orbit_delta(
                OrientationMode::HorizontalPlaneEuler,
                &mut q,
                0.0,
                step,
                DEFAULT_PITCH_LIMIT,
            );
            let (yaw, pitch) = yaw_pitch_from_orientation(q);
            assert!((pitch - (pole + step)).abs() < 1e-3, "{pole}: {pitch}");
            assert!((yaw - 0.7).abs() < 1e-3, "heading kept: {yaw}");
            // A yaw-only orbit at the pole: at most POLE_EPS off it.
            let mut q = orientation_from_yaw_pitch(0.7, pole);
            apply_orbit_delta(
                OrientationMode::HorizontalPlaneEuler,
                &mut q,
                0.2,
                0.0,
                DEFAULT_PITCH_LIMIT,
            );
            let (_, pitch) = yaw_pitch_from_orientation(q);
            assert!(
                (pitch.abs() - FRAC_PI_2).abs() <= POLE_EPS + 1e-4,
                "{pitch}"
            );
        }
    }

    /// An upward step at the top limit, from an exact Top / Bottom snap, or a big step just
    /// below the limit never crosses the pole (no 180° flip), in every mode: the heading stays
    /// and the elevation keeps its sign and stays within ±π/2. Repeated, as a held drag is.
    #[test]
    fn an_upward_step_at_the_limit_never_crosses_the_pole() {
        use std::f32::consts::FRAC_PI_2;
        let lim = DEFAULT_PITCH_LIMIT;
        for mode in [
            OrientationMode::Quaternion,
            OrientationMode::HorizontalPlaneEuler,
            OrientationMode::Euler,
        ] {
            for (start, step) in [
                (lim, 0.01),
                (FRAC_PI_2, 0.01),
                (-lim, -0.01),
                (-FRAC_PI_2, -0.01),
                (lim - 0.01, 0.5),
            ] {
                let mut q = orientation_from_yaw_pitch(0.7, start);
                for frame in 0..3 {
                    apply_orbit_delta(mode, &mut q, 0.0, step, lim);
                    let (yaw, pitch) = yaw_pitch_from_orientation(q);
                    let at = format!("{mode:?} start {start} step {step} frame {frame}");
                    assert!((yaw - 0.7).abs() < 1e-3, "{at}: heading flipped to {yaw}");
                    assert!(pitch.abs() <= FRAC_PI_2, "{at}: {pitch}");
                    assert!(pitch * start > 0.0 && pitch.abs() > 1.5, "{at}: {pitch}");
                }
            }
        }
    }

    #[test]
    fn limit_pitch_step_only_moves_back_beyond_the_limit() {
        let lim = 1.0;
        assert_eq!(limit_pitch_step(0.0, 0.5, lim), 0.5);
        assert!((limit_pitch_step(0.9, 0.5, lim) - 0.1).abs() < 1e-6);
        assert_eq!(limit_pitch_step(1.2, 0.1, lim), 0.0, "beyond: no further");
        assert!(
            (limit_pitch_step(1.2, -0.1, lim) + 0.1).abs() < 1e-6,
            "beyond: back is free"
        );
        assert_eq!(limit_pitch_step(-1.2, -0.1, lim), 0.0);
        assert_eq!(
            limit_pitch_step(0.3, 0.2, f32::NAN),
            0.0,
            "NaN limit blocks"
        );
        assert_eq!(
            limit_pitch_step(0.3, 0.2, -1.0),
            0.0,
            "negative limit blocks"
        );
    }

    #[test]
    fn euler_round_trip() {
        let q = orientation_from_yaw_pitch(0.5, 0.2);
        let (y, p) = yaw_pitch_from_orientation(q);
        let q2 = orientation_from_yaw_pitch(y, p);
        let d = (q - q2).length();
        assert!(d < 1e-4, "recomposed quat delta {d}");
    }

    #[test]
    fn smart_snap_projects_to_nearest_plane_and_locks_axis_within_15_deg() {
        use std::f32::consts::FRAC_PI_2;
        // 30° yaw in XZ with small 5° pitch -> projects to XZ plane, keeps 30° yaw.
        let yaw_30 = 30.0_f32.to_radians();
        let (y, p) = smart_snap_yaw_pitch(yaw_30, 5.0_f32.to_radians());
        assert!(
            (y - yaw_30).abs() < 1e-4,
            "expected continuous yaw {yaw_30}, got {y}"
        );
        assert!(p.abs() < 1e-5, "expected zero pitch on XZ plane, got {p}");

        // 10° yaw in XZ (< 15°) with 5° pitch -> locks both Y and Z -> +X axis (yaw=0, pitch=0).
        let (y, p) = smart_snap_yaw_pitch(10.0_f32.to_radians(), 5.0_f32.to_radians());
        assert!(y.abs() < 1e-5, "expected +X axis yaw=0, got {y}");
        assert!(p.abs() < 1e-5, "expected +X axis pitch=0, got {p}");

        // Near +Y pole (82° pitch, 20° yaw) -> locks both X and Z -> +Y pole with quarter-turn yaw.
        let (y, p) = smart_snap_yaw_pitch(20.0_f32.to_radians(), 82.0_f32.to_radians());
        assert!(
            y.abs() < 1e-5,
            "expected quarter-turn yaw=0 at pole, got {y}"
        );
        assert!(
            (p - FRAC_PI_2).abs() < 1e-5,
            "expected +Y pole pitch, got {p}"
        );
    }

    const MODES: [OrientationMode; 3] = [
        OrientationMode::Quaternion,
        OrientationMode::HorizontalPlaneEuler,
        OrientationMode::Euler,
    ];

    /// A pitch limit well clear of the pole for the step-rule tests below, so a pose at 1.5 rad
    /// lies beyond it and the yaw reads back precisely at
    /// `-limit` (at our near-pole default the heading is ill-conditioned in f32).
    const STEP_TEST_LIMIT: f32 = 1.2;

    /// A pitch step far past the limit stops AT the limit and keeps the heading (clamping only
    /// after the rotation would flip the yaw by pi). Every mode.
    #[test]
    fn huge_pitch_step_clamps_without_yaw_flip() {
        for mode in MODES {
            let mut q = orientation_from_yaw_pitch(0.5, 1.0);
            apply_orbit_delta(mode, &mut q, 0.0, 5.0, STEP_TEST_LIMIT);
            let (y, p) = yaw_pitch_from_orientation(q);
            assert!((y - 0.5).abs() < 1e-4, "{mode:?}: yaw {y}");
            assert!((p - STEP_TEST_LIMIT).abs() < 1e-4, "{mode:?}: pitch {p}");
            apply_orbit_delta(mode, &mut q, 0.0, -10.0, STEP_TEST_LIMIT);
            let (y, p) = yaw_pitch_from_orientation(q);
            assert!(
                (y - 0.5).abs() < 1e-4 && (p + STEP_TEST_LIMIT).abs() < 1e-4,
                "{mode:?}: yaw {y} pitch {p}"
            );
        }
    }

    /// A pose already past the limit eases back by the drag amount instead of
    /// snapping onto the limit, and cannot be pushed further out. Every mode.
    #[test]
    fn pose_beyond_limit_moves_back_incrementally() {
        for mode in MODES {
            let mut q = orientation_from_yaw_pitch(0.2, 1.5);
            apply_orbit_delta(mode, &mut q, 0.0, -0.1, STEP_TEST_LIMIT);
            let p = yaw_pitch_from_orientation(q).1;
            assert!((p - 1.4).abs() < 1e-4, "{mode:?}: {p}");
            apply_orbit_delta(mode, &mut q, 0.0, 0.5, STEP_TEST_LIMIT);
            let p = yaw_pitch_from_orientation(q).1;
            assert!((p - 1.4).abs() < 1e-4, "{mode:?}: no outward push, {p}");
            // A yaw-only orbit beyond the limit keeps the elevation too.
            apply_orbit_delta(mode, &mut q, 0.3, 0.0, STEP_TEST_LIMIT);
            let p = yaw_pitch_from_orientation(q).1;
            assert!((p - 1.4).abs() < 1e-4, "{mode:?}: yaw-only kept, {p}");
        }
    }

    /// Yaw composes additively and leaves the pitch alone. Every mode.
    #[test]
    fn yaw_delta_adds() {
        for mode in MODES {
            let mut q = orientation_from_yaw_pitch(0.3, 0.2);
            apply_orbit_delta(mode, &mut q, 0.4, 0.0, DEFAULT_PITCH_LIMIT);
            let (y, p) = yaw_pitch_from_orientation(q);
            assert!(
                (y - 0.7).abs() < 1e-4 && (p - 0.2).abs() < 1e-4,
                "{mode:?}: yaw {y} pitch {p}"
            );
        }
    }
}
