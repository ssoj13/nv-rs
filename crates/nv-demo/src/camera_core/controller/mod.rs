//! Pluggable camera backends behind one [`CameraController`] enum.
//!
//! What: four navigation styles that share a single [`CameraPose`] output, so render code is
//! backend-agnostic:
//!
//! | Variant | Backend | Style |
//! |---|---|---|
//! | [`CameraController::Houdini`] | [`HoudiniOrbit`] | classic euler tumble / dolly / pan, wraps [`crate::camera_core::rig::OrbitRig`] |
//! | [`CameraController::Quat`] | [`QuatOrbit`] | gimbal-free quaternion orbit around a target |
//! | [`CameraController::Fps`] | [`FpsFly`] | yaw / pitch mouse-look + WASD thrust, gravity-free, no roll |
//! | [`CameraController::Space`] | [`SpaceFlight`] | 6-DoF spaceship: thrust, roll, speed scale, inertial drift, optional horizon lock (crate-private `lock::HorizonLock`) |
//!
//! Why: hosts switch navigation style at runtime and keep view continuity; one enum facade
//! keeps every host on the same `apply_intent` / `update_dynamics` / `pose` contract.
//!
//! How: one file per backend (`orbit`, `quat`, `fps`, `space`), the horizon lock spring
//! (`lock`) and the dispatching facade (`facade`). This file holds the helpers every backend
//! shares: the lens restore of a pose (`projection_from_pose`), the sphere fit of an AABB
//! (`frame_sphere`), the dolly-zoom range limits of the orbit backends (`dolly_zoom`, over
//! `crate::camera_core::lens`) and the HUD look-lead gains; `thrust` holds the held-thrust velocity of the
//! fly backends (`Drive`: approach the commanded velocity, then a `crate::camera_core::coast::Coast`).
//!
//! Uses: `crate::camera_core::intent` (the intents), `crate::camera_core::rig`, `crate::camera_core::dynamics`, `crate::camera_core::coast`,
//! `crate::camera_core::fit`, `crate::camera_core::settings`, `crate::camera_core::pose`. Used by: the crate root re-exports, host viewports and a
//! host input layer.

mod facade;
mod fps;
mod lock;
mod orbit;
mod quat;
mod space;
mod thrust;

pub use facade::{CameraController, CameraKind};
pub use fps::FpsFly;
pub use orbit::HoudiniOrbit;
pub use quat::QuatOrbit;
pub use space::SpaceFlight;

use glam::Vec3;

use crate::camera_core::intent::CameraIntent;
use crate::camera_core::pose::CameraPose;
use crate::camera_core::projection::PerspectiveSettings;

/// HUD look lead: weight of the second (already low-passed) stage of the smoothed mouse-look
/// cascade. It dominates the lead so a reticle moves smoothly, without the per-poll steps the
/// first stage still carries.
const LOOK_LEAD_PENDING_GAIN: f32 = 1.6;

/// HUD look lead: weight of the first stage of the smoothed mouse-look cascade, a small share
/// so the reticle reacts on the very first frame of a turn.
const LOOK_LEAD_STAGE1_GAIN: f32 = 0.25;

/// HUD look lead (radians) per unit of held deflection steering (`LookRate`) when no smoothed
/// look is queued: the reticle leans toward the held turn.
const LOOK_LEAD_STEER_GAIN: f32 = 0.12;

/// Squared length below which a rotation axis (a camera up / right) is degenerate and a turn
/// about it is skipped instead of producing a NaN quaternion.
const AXIS_EPS_SQ: f32 = 1e-10;

/// A restored fov (radians) must stay this far inside `(0, pi)`, where `tan(fov / 2)` is finite.
const FOV_EDGE_EPS: f32 = 1e-4;

/// Smallest restored near plane (world units): a near at 0 makes the reverse-Z depth degenerate.
const NEAR_MIN: f32 = 1e-6;

/// Depth (world units) added beyond a restored near when the fallback far would not exceed it.
const FALLBACK_DEPTH: f32 = 1.0;

/// A frame step `update_dynamics` can integrate: positive and finite. A zero, negative, NaN or
/// infinite `dt` (a paused clock, a host bug) would freeze or poison the pose, so the free
/// backends (Quat, Fps, Space) return `false` and change nothing for it. (The orbit rig's view
/// transition handles such a step its own way and keeps reporting its animation.)
fn usable_dt(dt: f32) -> bool {
    dt > 0.0 && dt.is_finite()
}

/// Every scalar a delta / rate / factor intent carries is finite. A NaN or infinite value (a
/// host dividing by a zero dt, a broken device) would poison the pose or a latched rate for
/// good, so the free backends (Quat, Fps, Space) drop such an intent whole. `FrameBounds` is
/// always accepted (`frame_sphere` makes any bounds finite) and `View` carries no scalar.
fn finite_intent(intent: &CameraIntent) -> bool {
    let all = |v: &[f32]| v.iter().all(|x| x.is_finite());
    match *intent {
        CameraIntent::Orbit {
            yaw_delta,
            pitch_delta,
        }
        | CameraIntent::OrbitSnap {
            yaw_delta,
            pitch_delta,
            ..
        } => all(&[yaw_delta, pitch_delta]),
        CameraIntent::OrbitInertia {
            yaw_rate,
            pitch_rate,
        } => all(&[yaw_rate, pitch_rate]),
        CameraIntent::Pan { dx_px, dy_px } => all(&[dx_px, dy_px]),
        CameraIntent::PanInertia { dx_rate, dy_rate } => all(&[dx_rate, dy_rate]),
        CameraIntent::Zoom { factor } => factor.is_finite(),
        CameraIntent::ZoomInertia { rate } => rate.is_finite(),
        CameraIntent::DollyZoomInertia { fov_rate } => fov_rate.is_finite(),
        CameraIntent::LookInertia {
            yaw_rate,
            pitch_rate,
        } => all(&[yaw_rate, pitch_rate]),
        CameraIntent::ZoomAtCursor {
            cursor_x,
            cursor_y,
            factor,
        } => all(&[cursor_x, cursor_y, factor]),
        CameraIntent::Look { dyaw, dpitch } => all(&[dyaw, dpitch]),
        CameraIntent::LookRate { yaw, pitch } => all(&[yaw, pitch]),
        CameraIntent::Roll { d } => d.is_finite(),
        CameraIntent::Thrust { forward, right, up } => all(&[forward, right, up]),
        CameraIntent::SpeedScale(scale) => scale.is_finite(),
        CameraIntent::DollyZoom { dfov } => dfov.is_finite(),
        CameraIntent::FrameBounds { .. } | CameraIntent::View(_) | CameraIntent::Stop => true,
    }
}

/// The dolly-zoom step of the orbit backends: the new `(distance, fov_y)` after widening
/// `fov_y` by `dfov`. The invariant (`distance * tan(fov_y / 2)` constant, the subject's
/// on-screen size) is [`crate::camera_core::lens::dolly_zoom_factor`]'s; this function only limits the range.
///
/// The new FOV stays within [`dolly_fov_range`] (the viewport FOV range AND the FOVs the lens
/// distance range allows), so the distance never needs a clamp that would break the invariant.
/// `None` (nothing to do: no edit, no coast or transition cancelled) for a zero or non-finite
/// `dfov`, an unusable state (see [`dolly_fov_range`]), or a step the range clamps back onto
/// the current FOV (pushing against a limit).
pub(crate) fn dolly_zoom(
    distance: f32,
    fov_y: f32,
    dfov: f32,
    projection: PerspectiveSettings,
) -> Option<(f32, f32)> {
    if !(dfov.is_finite() && dfov != 0.0) {
        return None;
    }
    let (lo, hi) = dolly_fov_range(distance, fov_y, projection)?;
    let fov = crate::camera_core::num::clamp(fov_y + dfov, lo, hi);
    // Pushing against a limit: no edit at all, so no coast or transition is cancelled.
    if fov == fov_y {
        return None;
    }
    Some((
        distance * crate::camera_core::lens::dolly_zoom_factor(fov_y, fov),
        fov,
    ))
}

/// The FOVs (radians) a dolly-zoom of an orbit at `distance` / `fov_y` can reach: the one
/// viewport FOV range ([`crate::camera_core::lens::FOV_MIN`]..=[`crate::camera_core::lens::FOV_MAX`]) intersected with
/// the FOVs the lens distance range allows (`fov = 2 atan(k / distance)`, `k = distance *
/// tan(fov_y / 2)`: the dolly-zoom invariant, so the range does not change along a dolly zoom).
/// `None` for an unusable distance / fov (outside the lens angle range, where
/// [`crate::camera_core::lens::dolly_zoom_factor`] would clamp) or a distance range admitting no FOV in range.
fn dolly_fov_range(
    distance: f32,
    fov_y: f32,
    projection: PerspectiveSettings,
) -> Option<(f32, f32)> {
    use crate::camera_core::lens::{ANGLE_MAX_RAD, ANGLE_MIN_RAD, FOV_MAX, FOV_MIN};
    let usable =
        distance.is_finite() && distance > 0.0 && (ANGLE_MIN_RAD..=ANGLE_MAX_RAD).contains(&fov_y);
    if !usable {
        return None;
    }
    let k = distance * (fov_y * 0.5).tan();
    if !k.is_finite() {
        return None;
    }
    let mut lo = FOV_MIN;
    let mut hi = FOV_MAX;
    if projection.distance_max.is_finite() && projection.distance_max > 0.0 {
        lo = lo.max(2.0 * (k / projection.distance_max).atan());
    }
    if projection.distance_min.is_finite() && projection.distance_min > 0.0 {
        hi = hi.min(2.0 * (k / projection.distance_min).atan());
    }
    (lo <= hi).then_some((lo, hi))
}

/// The lens `pose` carries, each field falling back to `base` when the pose value is unusable
/// (non-finite, a degenerate fov, a non-positive near, a far not beyond near). The distance
/// clamps always come from `base`: a pose has none.
///
/// Why: restoring a saved pose into a fresh backend must keep the lens it was saved with, and a
/// corrupt or default-constructed pose must never produce a NaN projection.
fn projection_from_pose(pose: CameraPose, base: PerspectiveSettings) -> PerspectiveSettings {
    let fov_y = if pose.fov_y.is_finite()
        && pose.fov_y > FOV_EDGE_EPS
        && pose.fov_y < std::f32::consts::PI - FOV_EDGE_EPS
    {
        pose.fov_y
    } else {
        base.fov_y
    };
    let near = if pose.znear.is_finite() && pose.znear > NEAR_MIN {
        pose.znear
    } else {
        base.near
    };
    let far = if pose.zfar.is_finite() && pose.zfar > near {
        pose.zfar
    } else {
        // `base.far` may sit below a restored near.
        base.far.max(near + FALLBACK_DEPTH)
    };
    PerspectiveSettings {
        fov_y,
        near,
        far,
        ..base
    }
}

/// Centre and pivot distance that frame the AABB `min..max` in a `width x height` viewport.
///
/// The one framing rule of the free backends ([`QuatOrbit`], [`FpsFly`], [`SpaceFlight`]):
/// `crate::camera_core::fit` collapses the box to a finite sphere (a non-finite centre becomes the origin, a
/// non-finite or empty radius becomes 1) and returns a finite distance clamped to the
/// projection range, so no host bounds can put a NaN into the pose.
fn frame_sphere(
    min: Vec3,
    max: Vec3,
    width: u32,
    height: u32,
    margin: f32,
    projection: PerspectiveSettings,
) -> (Vec3, f32) {
    let (center, radius) = crate::camera_core::fit::aabb_center_radius(min, max);
    let aspect = width as f32 / height.max(1) as f32;
    let distance = crate::camera_core::fit::distance_to_frame_sphere(
        radius,
        aspect,
        projection.fov_y,
        margin,
        projection,
    );
    (center, distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera_core::view::{Face, UpAxis, ViewTarget};
    use crate::camera_core::viewport::ViewportSize;

    const DT: f32 = 1.0 / 60.0;

    fn vp() -> ViewportSize {
        ViewportSize::new(800, 600)
    }

    /// `(pivot, distance, fov_y, forward)` of an orbit controller.
    fn orbit_state(ctrl: &CameraController) -> (Vec3, f32, f32, Vec3) {
        let forward = ctrl.pose().forward();
        match ctrl {
            CameraController::Houdini(b) => (
                b.rig.pose.target,
                b.rig.pose.distance,
                b.rig.projection.fov_y,
                forward,
            ),
            CameraController::Quat(b) => (b.target, b.distance, b.projection.fov_y, forward),
            CameraController::Fps(_) | CameraController::Space(_) => {
                panic!("{} is not an orbit backend", ctrl.kind())
            }
        }
    }

    fn orbits() -> [CameraController; 2] {
        [
            CameraController::Houdini(HoudiniOrbit::default()),
            CameraController::Quat(QuatOrbit::default()),
        ]
    }

    /// `distance * tan(fov / 2)`: the half-height of the view at the pivot.
    fn subject_half_height(distance: f32, fov_y: f32) -> f32 {
        distance * (fov_y * 0.5).tan()
    }

    /// Both orbits: the fov changes by `dfov`, the eye dollies along the unchanged forward about
    /// the unchanged pivot, and `distance * tan(fov / 2)` is kept; the pose carries the new fov.
    #[test]
    fn dolly_zoom_keeps_the_subject_size() {
        for mut ctrl in orbits() {
            for dfov in [0.2_f32, -0.3, 0.05] {
                let (p0, d0, f0, fw0) = orbit_state(&ctrl);
                ctrl.apply_intent(CameraIntent::DollyZoom { dfov }, DT, vp());
                let (p1, d1, f1, fw1) = orbit_state(&ctrl);
                let kind = ctrl.kind();
                assert!(
                    (f1 - (f0 + dfov)).abs() < 1e-5,
                    "{kind} {dfov}: fov {f0} -> {f1}"
                );
                let (k0, k1) = (subject_half_height(d0, f0), subject_half_height(d1, f1));
                assert!((k1 - k0).abs() <= 1e-5 * k0, "{kind} {dfov}: {k0} -> {k1}");
                assert!(p1.distance(p0) < 1e-4, "{kind} {dfov}: the pivot moved");
                assert!(fw1.dot(fw0) > 0.99999, "{kind} {dfov}: the view turned");
                assert_eq!(
                    ctrl.pose().fov_y,
                    f1,
                    "{kind}: the pose carries the new fov"
                );
                assert_eq!(ctrl.projection().fov_y, f1);
            }
        }
    }

    /// At the limits the step stops at the FOV range or at the distance range, whichever comes
    /// first, and the invariant still holds.
    #[test]
    fn dolly_zoom_limits_keep_the_invariant() {
        for mut ctrl in orbits() {
            let kind = ctrl.kind();
            let (_, d0, f0, _) = orbit_state(&ctrl);
            let k0 = subject_half_height(d0, f0);

            // Wide: the FOV range limits first (the distance stays above its minimum).
            ctrl.apply_intent(CameraIntent::DollyZoom { dfov: 10.0 }, DT, vp());
            let (_, d1, f1, _) = orbit_state(&ctrl);
            assert!(
                (f1 - crate::camera_core::lens::FOV_MAX).abs() < 1e-5,
                "{kind}: fov {f1}"
            );
            let k1 = subject_half_height(d1, f1);
            assert!((k1 - k0).abs() <= 1e-5 * k0, "{kind}: wide {k0} -> {k1}");

            // Narrow: 1 degree would need a distance past the range, so the distance range
            // limits first and the fov stays wider than the FOV minimum.
            ctrl.apply_intent(CameraIntent::DollyZoom { dfov: -10.0 }, DT, vp());
            let (_, d2, f2, _) = orbit_state(&ctrl);
            let max = ctrl.projection().distance_max;
            assert!(
                (d2 - max).abs() <= 1e-4 * max,
                "{kind}: distance {d2} vs max {max}"
            );
            assert!(f2 > crate::camera_core::lens::FOV_MIN, "{kind}: fov {f2}");
            let k2 = subject_half_height(d2, f2);
            assert!((k2 - k0).abs() <= 1e-4 * k0, "{kind}: narrow {k0} -> {k2}");
        }
    }

    /// A zero or non-finite `dfov` changes nothing on any backend; the fly backends ignore the
    /// intent altogether (they have no pivot).
    #[test]
    fn dolly_zoom_ignores_garbage_and_fly_backends() {
        for mut ctrl in [
            CameraController::Houdini(HoudiniOrbit::default()),
            CameraController::Quat(QuatOrbit::default()),
            CameraController::Fps(FpsFly::default()),
            CameraController::Space(SpaceFlight::default()),
        ] {
            let before = ctrl.pose();
            for dfov in [0.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
                ctrl.apply_intent(CameraIntent::DollyZoom { dfov }, DT, vp());
                assert_eq!(ctrl.pose(), before, "{} dfov {dfov}", ctrl.kind());
            }
            if ctrl.is_fly() {
                ctrl.apply_intent(CameraIntent::DollyZoom { dfov: 0.3 }, DT, vp());
                assert_eq!(ctrl.pose(), before, "{}: fly ignores it", ctrl.kind());
                assert!(!ctrl.has_motion());
            }
        }
    }

    /// A dolly-zoom is a direct edit: it ends the coasts of both orbits and the Houdini view
    /// transition, which would otherwise rebuild the pose over it on the next frames.
    #[test]
    fn dolly_zoom_cancels_coasts_and_the_view_transition() {
        let mut ctrl = CameraController::Houdini(HoudiniOrbit::default());
        ctrl.apply_intent(
            CameraIntent::View(ViewTarget::face(Face::Front, UpAxis::Y)),
            DT,
            vp(),
        );
        assert!(ctrl.has_motion(), "the view snap animates");
        ctrl.apply_intent(CameraIntent::DollyZoom { dfov: 0.2 }, DT, vp());
        assert!(!ctrl.has_motion(), "the transition ended");
        let after = ctrl.pose();
        for _ in 0..10 {
            ctrl.update_dynamics(DT);
        }
        assert_eq!(ctrl.pose(), after, "nothing overwrote the dolly-zoom");

        for mut ctrl in orbits() {
            ctrl.apply_intent(
                CameraIntent::OrbitInertia {
                    yaw_rate: 1.0,
                    pitch_rate: 0.0,
                },
                DT,
                vp(),
            );
            assert!(ctrl.has_motion(), "{}: coasting", ctrl.kind());
            ctrl.apply_intent(CameraIntent::DollyZoom { dfov: -0.1 }, DT, vp());
            assert!(!ctrl.has_motion(), "{}: the coast ended", ctrl.kind());
        }
    }

    /// Pushing a dolly-zoom against its limit is no edit: the FOV stays, and a running coast is
    /// NOT cancelled. `fov_range` reports the reachable FOVs (the narrow end is set by the
    /// distance range, above the 1 degree FOV minimum) and matches what the step reaches.
    #[test]
    fn dolly_zoom_at_the_limit_is_no_edit() {
        for mut ctrl in orbits() {
            let kind = ctrl.kind();
            let (lo, hi) = ctrl.fov_range();
            assert!(
                lo > crate::camera_core::lens::FOV_MIN && hi == crate::camera_core::lens::FOV_MAX,
                "{kind}"
            );
            ctrl.apply_intent(CameraIntent::DollyZoom { dfov: 10.0 }, DT, vp());
            assert_eq!(ctrl.projection().fov_y, hi, "{kind}: at the wide limit");
            ctrl.apply_intent(
                CameraIntent::OrbitInertia {
                    yaw_rate: 1.0,
                    pitch_rate: 0.0,
                },
                DT,
                vp(),
            );
            let before = ctrl.pose();
            ctrl.apply_intent(CameraIntent::DollyZoom { dfov: 0.1 }, DT, vp());
            assert!(ctrl.has_motion(), "{kind}: the coast survived");
            assert_eq!(ctrl.pose(), before, "{kind}: nothing changed");
            let (lo2, hi2) = ctrl.fov_range();
            assert!(
                (lo2 - lo).abs() < 1e-5 && hi2 == hi,
                "{kind}: the range is invariant"
            );
        }
        let fly = CameraController::Fps(FpsFly::default());
        assert_eq!(
            fly.fov_range(),
            (
                crate::camera_core::lens::FOV_MIN,
                crate::camera_core::lens::FOV_MAX
            )
        );
    }
}
