//! [`AutoOrbit`]: the turntable auto-orbit (a camera revolving about an axis at a constant rate).
//!
//! What: one `step(controller, dt)` per frame turns the camera of any [`CameraController`]
//! backend by `rate * dt` radians about `axis` through a pivot.
//!
//! Why: product-shot / presentation turntables were written per host, frame-rate dependent
//! (a fixed angle per frame) and with a separate path per camera style. One dt-based rule here.
//!
//! How: the controller's [`CameraPose`] is rotated rigidly (eye about the pivot, orientation by
//! the same rotation) and written back with [`CameraController::set_pose`], so every backend works
//! through its public API and keeps its lens and tuning. Rotations about one fixed axis compose
//! additively, so the result is frame-rate independent. While it runs the turntable OWNS the
//! pose: `set_pose` ends coasts, view animations and held fly axes every frame (hosts stop the
//! turntable when the user navigates, as the auto-orbit toggles of the source viewers did).
//!
//! Per backend (pivot `None` = the backend's own pivot):
//! - `Houdini`: pivot = the rig target. The rig is re-seated through `HoudiniOrbit::set_pose`,
//!   which re-places the target at the same distance ahead of the rotated eye, i.e. exactly on
//!   the pivot again; a non-vertical axis tilts the view and the turntable drops the roll.
//! - `Quat`: pivot = its target, same re-seat; a free trackball keeps any tilt.
//! - `Fps`: no pivot of its own, so the camera turns in place about the eye (pass `pivot` to
//!   revolve around a point); the roll-free rig drops any roll of a tilted axis.
//! - `Space`: like `Fps`; a horizon lock re-seats on every `set_pose`.
//!
//! Uses: `crate::camera_core::controller`, `crate::camera_core::pose`, glam. Used by: hosts (turntable toggles) and the
//! navigation layer.

use std::f32::consts::TAU;

use glam::{Quat, Vec3};

use crate::camera_core::controller::CameraController;
use crate::camera_core::convention::WORLD_UP;
use crate::camera_core::pose::CameraPose;

/// Default turntable period in seconds per full revolution: a calm presentation spin.
pub const DEFAULT_PERIOD_SECS: f32 = 12.0;

/// Default turntable rate (rad/s): one revolution per [`DEFAULT_PERIOD_SECS`].
pub const DEFAULT_RATE: f32 = TAU / DEFAULT_PERIOD_SECS;

/// A constant-rate turntable about an axis.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AutoOrbit {
    /// Angular rate in rad/s; positive turns counter-clockwise seen from the tip of `axis`
    /// (right-hand rule: the eye moves counter-clockwise seen from above for `axis = +Y`). Zero
    /// stops the turntable.
    pub rate: f32,
    /// Rotation axis (world space, any length; normalised per step). Defaults to the world up.
    pub axis: Vec3,
    /// Point the axis runs through; `None` = the backend's own pivot (the orbit target of the
    /// orbit backends, the eye of the fly backends; see the module docs).
    pub pivot: Option<Vec3>,
}

impl Default for AutoOrbit {
    /// [`DEFAULT_RATE`] about the world up through the backend's own pivot.
    fn default() -> Self {
        Self {
            rate: DEFAULT_RATE,
            axis: WORLD_UP,
            pivot: None,
        }
    }
}

impl AutoOrbit {
    /// Turn `controller` by `rate * dt` radians about `axis` through the pivot. Returns `true`
    /// when the pose changed (the host keeps requesting frames while the turntable is on).
    ///
    /// A no-op returning `false` for a zero, negative or non-finite `dt`, a zero or non-finite
    /// `rate` (or a `rate * dt` overflowing to infinity), a zero or non-finite `axis`, a
    /// non-finite `pivot` or a non-finite current pose.
    pub fn step(&self, controller: &mut CameraController, dt: f32) -> bool {
        if dt <= 0.0 || !dt.is_finite() || self.rate == 0.0 || !self.rate.is_finite() {
            return false;
        }
        let angle = self.rate * dt;
        if !angle.is_finite() {
            return false;
        }
        // `normalize_or_zero` maps a zero or non-finite axis to zero.
        let axis = self.axis.normalize_or_zero();
        if axis == Vec3::ZERO {
            return false;
        }
        let pose = controller.pose();
        if !(pose.eye.is_finite() && pose.orientation.is_finite()) {
            return false;
        }
        let pivot = self.pivot.unwrap_or_else(|| own_pivot(controller, &pose));
        if !pivot.is_finite() {
            return false;
        }
        let rot = Quat::from_axis_angle(axis, angle);
        let turned = CameraPose {
            eye: pivot + rot * (pose.eye - pivot),
            orientation: (rot * pose.orientation).normalize(),
            ..pose
        };
        controller.set_pose(turned);
        true
    }
}

/// The pivot a backend turns about by itself: the orbit target, or the eye of a fly backend
/// (which has no pivot, so it turns in place).
fn own_pivot(controller: &CameraController, pose: &CameraPose) -> Vec3 {
    match controller {
        CameraController::Houdini(b) => b.rig.pose.target,
        CameraController::Quat(b) => b.target,
        CameraController::Fps(_) | CameraController::Space(_) => pose.eye,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera_core::controller::{FpsFly, HoudiniOrbit, QuatOrbit, SpaceFlight};
    use crate::camera_core::rig::OrbitRig;

    const EPS: f32 = 1e-3;

    fn houdini() -> CameraController {
        let rig = OrbitRig::from_yaw_pitch(0.4, 0.3, 12.0, Vec3::new(1.0, 2.0, -3.0));
        CameraController::Houdini(HoudiniOrbit::new(rig))
    }

    fn quat() -> CameraController {
        CameraController::Quat(QuatOrbit {
            target: Vec3::new(-2.0, 0.5, 1.0),
            distance: 8.0,
            ..QuatOrbit::default()
        })
    }

    fn all() -> [CameraController; 4] {
        [
            houdini(),
            quat(),
            CameraController::Fps(FpsFly::default()),
            CameraController::Space(SpaceFlight::default()),
        ]
    }

    fn turntable(rate: f32) -> AutoOrbit {
        AutoOrbit {
            rate,
            ..AutoOrbit::default()
        }
    }

    /// The orbit backends revolve about their target: the target stays, the eye keeps its
    /// height and distance and swings by exactly `rate * dt` about +Y.
    #[test]
    fn orbit_backends_revolve_about_their_target() {
        for (mut ctrl, target) in [
            (houdini(), Vec3::new(1.0, 2.0, -3.0)),
            (quat(), Vec3::new(-2.0, 0.5, 1.0)),
        ] {
            let before = ctrl.pose();
            let (rate, dt) = (0.8, 0.25);
            assert!(turntable(rate).step(&mut ctrl, dt), "{}", ctrl.kind());
            let after = ctrl.pose();
            let expected = target + Quat::from_rotation_y(rate * dt) * (before.eye - target);
            assert!(
                (after.eye - expected).length() < EPS,
                "{}: {:?} vs {expected:?}",
                ctrl.kind(),
                after.eye
            );
            assert!((after.eye.y - before.eye.y).abs() < EPS, "{}", ctrl.kind());
            // Still looking at the target.
            let to_target = (target - after.eye).normalize();
            assert!(after.forward().dot(to_target) > 0.999_99, "{}", ctrl.kind());
            assert_eq!(
                after.fov_y,
                before.fov_y,
                "{}: the lens is kept",
                ctrl.kind()
            );
        }
        // The rig target itself is unchanged (not only the look direction).
        let mut ctrl = houdini();
        assert!(turntable(1.0).step(&mut ctrl, 0.5));
        let CameraController::Houdini(b) = &ctrl else {
            panic!("expected Houdini");
        };
        assert!((b.rig.pose.target - Vec3::new(1.0, 2.0, -3.0)).length() < EPS);
    }

    /// A positive rate moves the eye counter-clockwise seen from above (+X toward -Z).
    #[test]
    fn positive_rate_is_counter_clockwise_from_above() {
        let target = Vec3::ZERO;
        let rig = OrbitRig::from_yaw_pitch(0.0, 0.0, 10.0, target);
        let mut ctrl = CameraController::Houdini(HoudiniOrbit::new(rig));
        let e0 = ctrl.pose().eye;
        turntable(1.0).step(&mut ctrl, 0.1);
        let e1 = ctrl.pose().eye;
        // Counter-clockwise from +Y: the cross of the old and new offsets points up.
        assert!(e0.cross(e1).y > 0.0, "{e0:?} -> {e1:?}");
    }

    /// Fly backends turn in place by default and revolve around an explicit pivot.
    #[test]
    fn fly_backends_turn_in_place_or_about_the_pivot() {
        for mut ctrl in [
            CameraController::Fps(FpsFly::default()),
            CameraController::Space(SpaceFlight::default()),
        ] {
            let before = ctrl.pose();
            assert!(turntable(1.0).step(&mut ctrl, 0.2));
            let after = ctrl.pose();
            assert!((after.eye - before.eye).length() < EPS, "{}", ctrl.kind());
            let expected = Quat::from_rotation_y(0.2) * before.forward();
            assert!(
                after.forward().dot(expected) > 0.999_99,
                "{}: {:?} vs {expected:?}",
                ctrl.kind(),
                after.forward()
            );

            let pivot = Vec3::new(3.0, 0.0, -4.0);
            let orbit = AutoOrbit {
                rate: 1.0,
                axis: Vec3::Y,
                pivot: Some(pivot),
            };
            let before = ctrl.pose();
            assert!(orbit.step(&mut ctrl, 0.3));
            let after = ctrl.pose();
            let expected = pivot + Quat::from_rotation_y(0.3) * (before.eye - pivot);
            assert!((after.eye - expected).length() < EPS, "{}", ctrl.kind());
        }
    }

    /// The same elapsed time gives the same pose at 30 Hz and 240 Hz, on every backend.
    #[test]
    fn step_is_frame_rate_independent() {
        let orbit = turntable(1.3);
        for (mut slow, mut fast) in all().into_iter().zip(all()) {
            for _ in 0..30 {
                orbit.step(&mut slow, 1.0 / 30.0);
            }
            for _ in 0..240 {
                orbit.step(&mut fast, 1.0 / 240.0);
            }
            let (a, b) = (slow.pose(), fast.pose());
            assert!((a.eye - b.eye).length() < 1e-2, "{}", slow.kind());
            assert!(
                a.orientation.angle_between(b.orientation) < 1e-3,
                "{}",
                slow.kind()
            );
        }
    }

    /// A zero rate, bad dt, a degenerate axis or pivot change nothing.
    #[test]
    fn degenerate_inputs_are_no_ops() {
        let (nan, inf) = (f32::NAN, f32::INFINITY);
        let cases = [
            (turntable(0.0), 0.1),
            (turntable(nan), 0.1),
            (turntable(inf), 0.1),
            (turntable(f32::MAX), 1.0e3),
            (turntable(1.0), 0.0),
            (turntable(1.0), -0.1),
            (turntable(1.0), nan),
            (turntable(1.0), inf),
            (
                AutoOrbit {
                    axis: Vec3::ZERO,
                    ..AutoOrbit::default()
                },
                0.1,
            ),
            (
                AutoOrbit {
                    axis: Vec3::new(nan, 1.0, 0.0),
                    ..AutoOrbit::default()
                },
                0.1,
            ),
            (
                AutoOrbit {
                    pivot: Some(Vec3::splat(inf)),
                    ..AutoOrbit::default()
                },
                0.1,
            ),
        ];
        for mut ctrl in all() {
            let before = ctrl.pose();
            for (orbit, dt) in cases {
                assert!(
                    !orbit.step(&mut ctrl, dt),
                    "{}: {orbit:?} dt {dt}",
                    ctrl.kind()
                );
                assert_eq!(ctrl.pose(), before, "{}: {orbit:?} moved", ctrl.kind());
            }
        }
    }

    /// A tilted axis on the free trackball keeps the pivot and the distance.
    #[test]
    fn tilted_axis_keeps_pivot_and_distance_on_the_trackball() {
        let mut ctrl = quat();
        let target = Vec3::new(-2.0, 0.5, 1.0);
        let orbit = AutoOrbit {
            rate: 0.7,
            axis: Vec3::new(1.0, 1.0, 0.0),
            pivot: None,
        };
        let d0 = (ctrl.pose().eye - target).length();
        for _ in 0..50 {
            assert!(orbit.step(&mut ctrl, 1.0 / 60.0));
        }
        let d1 = (ctrl.pose().eye - target).length();
        assert!((d1 - d0).abs() < EPS, "{d0} -> {d1}");
        let CameraController::Quat(b) = &ctrl else {
            panic!("expected Quat");
        };
        assert!((b.target - target).length() < EPS);
    }
}
