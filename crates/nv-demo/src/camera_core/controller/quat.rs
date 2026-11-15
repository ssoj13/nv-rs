//! [`QuatOrbit`]: the gimbal-free quaternion orbit backend of [`super::CameraController`].
//!
//! What: an orbit around `target` at `distance` whose orientation is a free quaternion,
//! accumulated incrementally (yaw about the camera's current up, pitch about its current right).
//!
//! Why: the euler turntable ([`super::HoudiniOrbit`]) is clamped at the poles on purpose; this
//! backend trades that for pole-safe free tumbling (trackball style).
//!
//! How: `tumble` rotates the FORWARD (eye = target - forward * distance), so an `Orbit` intent
//! keeps its yaw sign and flips its pitch to move the eye the same way as the Houdini rig
//! (pinned by `quat_and_houdini_move_the_eye_the_same_way`). `Look` turns the forward like
//! the fly backends' look (`quat_look_turns_like_the_fly_backends`). Coasts: the SAME
//! `crate::camera_core::coast::OrbitCoast` as the Houdini rig (turn, log zoom, world pan, dolly zoom, seeded
//! with the release velocity, closed form, each ending exactly at `navigation.coast.time`), so
//! both orbits coast alike (pinned by `quat_and_houdini_coast_the_same_way`). The turn channel
//! holds `(yaw, pitch)` rates in the `Orbit` intent's convention and is applied as
//! `tumble(yaw, -pitch)`; a `LookInertia` is the same tumble in the `Look` sign convention, so
//! it is folded into that channel as `(yaw_rate, -pitch_rate)` (a look coast and a tumble coast
//! are one motion here: this backend has a single rotation about the target). A coast step
//! never cancels itself; a press (`Stop`) or any direct edit does.
//!
//! Uses: `crate::camera_core::coast`, `crate::camera_core::intent`, `super::frame_sphere`, `super::dolly_zoom`.
//! Used by: `super::facade`.

use glam::{Quat, Vec2, Vec3};

use crate::camera_core::coast::OrbitCoast;
use crate::camera_core::intent::CameraIntent;
use crate::camera_core::pose::CameraPose;
use crate::camera_core::projection::PerspectiveSettings;
use crate::camera_core::settings::CameraNavigationSettings;
use crate::camera_core::viewport::{ViewportPoint, ViewportSize};

/// Gimbal-free orbit around a target using quaternion accumulation.
///
/// Like the orbit rig it keeps a `target` + `distance`, but the orientation is a free quaternion
/// instead of a yaw / pitch euler pair, which removes the pole singularity of the turntable.
#[derive(Debug, Clone)]
pub struct QuatOrbit {
    /// Pivot the camera orbits.
    pub target: Vec3,
    /// Distance from `target` to the eye.
    pub distance: f32,
    /// Free camera orientation (local -Z = forward toward `target`).
    pub orientation: Quat,
    /// Lens and the zoom distance range.
    pub projection: PerspectiveSettings,
    /// Orbit tuning: the pan sensitivity and the coast settings (`coast`) are used.
    pub navigation: CameraNavigationSettings,
    /// Release coasts, shared with `OrbitRig`: `turn` = `(yaw, pitch)` rad/s in the `Orbit`
    /// convention, `zoom` = log-distance rate (1/s), `pan` = world velocity, `dolly_zoom` =
    /// fov rate (rad/s).
    pub coast: OrbitCoast,
}

impl Default for QuatOrbit {
    fn default() -> Self {
        Self::from_pose(CameraPose::default())
    }
}

impl QuatOrbit {
    /// Build from a free pose: the lens comes from the pose (`projection_from_pose`), and the
    /// target is placed `eye + forward * d` with `d` the default orbit distance clamped to the
    /// range.
    pub fn from_pose(pose: CameraPose) -> Self {
        let projection = super::projection_from_pose(pose, PerspectiveSettings::default());
        let distance = projection.clamp_distance(crate::camera_core::convention::DEFAULT_ORBIT_DISTANCE);
        let target = pose.eye + pose.forward() * distance;
        Self {
            target,
            distance,
            orientation: pose.orientation,
            projection,
            navigation: CameraNavigationSettings::default(),
            coast: OrbitCoast::default(),
        }
    }

    /// Move to `pose`, mutating only the pose: the orientation is taken as is, the target is
    /// re-placed at the current (clamped) distance ahead of the new eye and the coasts stop.
    /// Lens and navigation tuning are kept.
    pub fn set_pose(&mut self, pose: CameraPose) {
        self.distance = self.projection.clamp_distance(self.distance);
        self.target = pose.eye + pose.forward() * self.distance;
        self.orientation = pose.orientation;
        self.stop_coast();
    }

    /// Stop every coast (tumble, zoom, pan, dolly zoom), like the Houdini rig's `halt`.
    fn stop_coast(&mut self) {
        self.coast.stop();
    }

    /// True while a coast runs (exact: a coast ends exactly at its time; one clamped at a
    /// limit keeps running, motionless, up to its end).
    pub fn has_motion(&self) -> bool {
        self.coast.is_active()
    }

    /// Current eye position (`target - forward * distance`).
    fn eye(&self) -> Vec3 {
        self.target - (self.orientation * -Vec3::Z).normalize_or_zero() * self.distance
    }

    /// Tumble by `(dyaw, dpitch)` radians about the camera's current up / right.
    fn tumble(&mut self, dyaw: f32, dpitch: f32) {
        if dyaw != 0.0 {
            let up = (self.orientation * Vec3::Y).normalize_or_zero();
            if up.length_squared() > super::AXIS_EPS_SQ {
                self.orientation = (Quat::from_axis_angle(up, dyaw) * self.orientation).normalize();
            }
        }
        if dpitch != 0.0 {
            let right = (self.orientation * Vec3::X).normalize_or_zero();
            if right.length_squared() > super::AXIS_EPS_SQ {
                self.orientation =
                    (Quat::from_axis_angle(right, dpitch) * self.orientation).normalize();
            }
        }
    }

    /// Apply one intent. Orbit / look intents tumble; `OrbitInertia` / `LookInertia` /
    /// `PanInertia` / `ZoomInertia` / `DollyZoomInertia` seed the coasts (kicked with the
    /// release velocity, `navigation.coast`; a seed below `min_speed` or with `time == 0`
    /// starts none) and `Stop` ends them; zoom changes the distance (`ZoomAtCursor` keeps the point under the cursor in
    /// place); pan slides the target on the view plane; `FrameBounds` frames the box in the
    /// real `viewport`; `View` snaps about the target; `DollyZoom` dollies about the target and
    /// widens the lens (`super::dolly_zoom`).
    ///
    /// `OrbitSnap` tumbles like a plain `Orbit` with its deltas: a free trackball has no
    /// turntable yaw / pitch to snap to a plane or axis, and dropping it froze the trackball
    /// while the snap modifiers (Shift / Ctrl) were held.
    ///
    /// Dropped (no-ops here): `LookRate` (fly deflection steering), `Thrust`, `Roll` and
    /// `SpeedScale` (fly-only). An intent carrying a
    /// NaN or infinite value is dropped whole (`super::finite_intent`).
    pub fn apply_intent(&mut self, intent: CameraIntent, viewport: ViewportSize) {
        if !super::finite_intent(&intent) {
            return;
        }
        match intent {
            // The trackball has no plane snap: `OrbitSnap` is a plain orbit (see the doc above).
            CameraIntent::Orbit {
                yaw_delta,
                pitch_delta,
            }
            | CameraIntent::OrbitSnap {
                yaw_delta,
                pitch_delta,
                ..
            } => {
                self.stop_coast();
                // A trackball: +yaw swings the eye about the CAMERA's own up axis, +pitch about
                // its own right (raising the eye). From a level view (no pitch, no roll) the
                // camera up is world up, so this is the Houdini turntable's direction there
                // (pinned by `quat_and_houdini_move_the_eye_the_same_way`); once pitched or
                // rolled the axes tilt with the camera instead of staying on world up.
                // `tumble` rotates the FORWARD (eye = target - forward * d), so yaw keeps its sign
                // and pitch flips.
                self.tumble(yaw_delta, -pitch_delta);
            }
            CameraIntent::Look { dyaw, dpitch } => {
                // A look turns the FORWARD exactly like `Look` on FpsFly / SpaceFlight (+yaw
                // toward the camera's left, +pitch up); the eye swings about the target.
                self.stop_coast();
                self.tumble(dyaw, dpitch);
            }
            CameraIntent::OrbitInertia {
                yaw_rate,
                pitch_rate,
            } => {
                // The coast continues the drag; the turn channel is in the `Orbit` convention
                // (`update_dynamics` applies it with the `Orbit` arm's signs).
                self.coast
                    .turn
                    .kick(Vec2::new(yaw_rate, pitch_rate), 1.0, &self.navigation.coast);
            }
            CameraIntent::LookInertia {
                yaw_rate,
                pitch_rate,
            } => {
                // A look is `tumble(dyaw, dpitch)`, an orbit is `tumble(yaw, -pitch)`: the look
                // rates enter the shared turn channel with the pitch sign flipped.
                self.coast.turn.kick(
                    Vec2::new(yaw_rate, -pitch_rate),
                    1.0,
                    &self.navigation.coast,
                );
            }
            CameraIntent::PanInertia { dx_rate, dy_rate } => {
                // The pointer velocity (px/s) through the `Pan` arm's pixel-to-world scale and
                // the screen axes of this moment; `min_speed` is in distances per second.
                let (right, up) = self.pose().view_plane_axes();
                let scale = self.distance * self.navigation.pan_sensitivity;
                let dv = right * (-dx_rate * scale) + up * (dy_rate * scale);
                self.coast
                    .pan
                    .kick(dv, self.distance, &self.navigation.coast);
            }
            CameraIntent::DollyZoomInertia { fov_rate } => {
                self.coast
                    .dolly_zoom
                    .kick(fov_rate, 1.0, &self.navigation.coast);
            }
            CameraIntent::Stop => self.stop_coast(),
            CameraIntent::Zoom { factor } => {
                // A direct edit cancels the coasts, as on the Houdini rig.
                self.stop_coast();
                self.distance = self.projection.clamp_distance(self.distance * factor);
            }
            CameraIntent::ZoomInertia { rate } => {
                // The log-distance rate of the release, like the Houdini rig's `zoom_inertia`,
                // integrated in `update_dynamics`.
                self.coast.zoom.kick(rate, 1.0, &self.navigation.coast);
            }
            CameraIntent::ZoomAtCursor {
                cursor_x,
                cursor_y,
                factor,
            } => {
                // The point under the cursor on the target plane stays under the cursor (the
                // rule of `OrbitPose::zoom_at_cursor`): the orientation does not change, so the
                // target slides by the cursor's view-plane offset times the distance change.
                self.stop_coast();
                let ndc = ViewportPoint {
                    x: cursor_x,
                    y: cursor_y,
                }
                .to_ndc(viewport);
                let (right, up) = self.pose().view_plane_axes();
                let half_tan = (self.projection.fov_y * 0.5).tan();
                let per_distance =
                    right * (ndc.x * half_tan * viewport.aspect()) + up * (ndc.y * half_tan);
                let before = self.distance;
                self.distance = self.projection.clamp_distance(before * factor);
                self.target += per_distance * (before - self.distance);
            }
            CameraIntent::Pan { dx_px, dy_px } => {
                self.stop_coast();
                let pose = self.pose();
                let (right, up) = pose.view_plane_axes();
                let scale = self.distance * self.navigation.pan_sensitivity;
                self.target += right * (-dx_px * scale) + up * (dy_px * scale);
            }
            CameraIntent::FrameBounds { min, max, margin } => {
                self.frame_aabb(min, max, viewport.width, viewport.height, margin);
            }
            CameraIntent::View(view) => {
                // Snap about the target (no transition state in this backend), Y-up framing.
                self.stop_coast();
                let (f, u) = view.frame(
                    crate::camera_core::UpAxis::Y,
                    self.orientation * -Vec3::Z,
                    self.orientation * Vec3::Y,
                );
                let p = CameraPose::from_eye_forward_up(self.eye(), f, u, 0.0, 0.0, 0.0);
                self.orientation = p.orientation;
            }
            CameraIntent::DollyZoom { dfov } => {
                if let Some((distance, fov_y)) =
                    super::dolly_zoom(self.distance, self.projection.fov_y, dfov, self.projection)
                {
                    // A direct edit cancels the coasts, as on the Houdini rig.
                    self.stop_coast();
                    self.distance = self.projection.clamp_distance(distance);
                    self.projection.fov_y = fov_y;
                }
            }
            CameraIntent::LookRate { .. }
            | CameraIntent::Roll { .. }
            | CameraIntent::Thrust { .. }
            | CameraIntent::SpeedScale(_) => {}
        }
    }

    /// Frame an AABB in a `width x height` viewport: the target moves to the box centre and the
    /// distance fits its sphere (`super::frame_sphere`, finite for any bounds); the coast stops.
    pub(super) fn frame_aabb(
        &mut self,
        min: Vec3,
        max: Vec3,
        width: u32,
        height: u32,
        margin: f32,
    ) {
        let (center, distance) =
            super::frame_sphere(min, max, width, height, margin, self.projection);
        self.target = center;
        self.distance = distance;
        self.stop_coast();
    }

    /// Advance the coasts by one [`OrbitCoast::step`]: the turn tumbles (`tumble(yaw, -pitch)`,
    /// the `Orbit` arm's signs), the distance multiplies by `exp(log_zoom)` clamped to the
    /// range, the target moves by the world pan, and the dolly-zoom step goes through
    /// `super::dolly_zoom` (`None`, the lens range reached, stops that channel). Exactly the
    /// Houdini rig's coast step, so both orbits coast alike; it never cancels a coast. Returns
    /// `true` while a coast runs; an unusable `dt` (`super::usable_dt`) changes nothing and
    /// returns `false`.
    pub fn update_dynamics(&mut self, dt: f32) -> bool {
        if !super::usable_dt(dt) || !self.has_motion() {
            return false;
        }
        let step = self.coast.step(dt);
        self.tumble(step.turn.x, -step.turn.y);
        if step.log_zoom != 0.0 {
            self.distance = self
                .projection
                .clamp_distance(self.distance * step.log_zoom.exp());
        }
        self.target += step.pan;
        if step.dfov != 0.0 {
            match super::dolly_zoom(
                self.distance,
                self.projection.fov_y,
                step.dfov,
                self.projection,
            ) {
                Some((distance, fov_y)) => {
                    self.distance = self.projection.clamp_distance(distance);
                    self.projection.fov_y = fov_y;
                }
                None => self.coast.dolly_zoom.stop(),
            }
        }
        self.has_motion()
    }

    /// The shared pose of the current orbit state.
    pub fn pose(&self) -> CameraPose {
        CameraPose {
            eye: self.eye(),
            orientation: self.orientation,
            fov_y: self.projection.fov_y,
            znear: self.projection.near,
            zfar: self.projection.far,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera_core::controller::HoudiniOrbit;

    use crate::camera_core::convention::DEFAULT_ORBIT_DISTANCE;

    /// The eye at +Z, the default orbit distance from the origin, looking at it (so both orbits
    /// built from it pivot on the origin).
    fn start_pose() -> CameraPose {
        CameraPose::from_eye_forward_up(
            Vec3::new(0.0, 0.0, DEFAULT_ORBIT_DISTANCE),
            Vec3::NEG_Z,
            Vec3::Y,
            45.0_f32.to_radians(),
            0.1,
            500.0,
        )
    }

    #[test]
    fn quat_orbit_no_nan_near_poles() {
        let mut q = QuatOrbit::default();
        // Pitch far past where an euler turntable would gimbal-lock.
        for _ in 0..200 {
            q.apply_intent(
                CameraIntent::Orbit {
                    yaw_delta: 0.0,
                    pitch_delta: 0.1,
                },
                ViewportSize::new(1, 1),
            );
            let p = q.pose();
            assert!(p.eye.is_finite(), "eye went non-finite: {:?}", p.eye);
            assert!(p.orientation.is_finite(), "orientation NaN");
            let f = p.forward();
            assert!(
                f.is_finite() && (f.length() - 1.0).abs() < 1e-3,
                "forward bad: {f:?}"
            );
        }
    }

    /// +yaw moves the eye toward +X (Houdini: offset = Ry(yaw) * X; from yaw = -pi/2 the
    /// offset (sin d, 0, cos d) has x > 0), +pitch raises it.
    #[test]
    fn quat_orbit_yaw_and_pitch_match_houdini_direction() {
        let vp = ViewportSize::new(1, 1);
        let mut yaw = QuatOrbit::from_pose(start_pose());
        yaw.apply_intent(
            CameraIntent::Orbit {
                yaw_delta: 0.1,
                pitch_delta: 0.0,
            },
            vp,
        );
        let e = yaw.pose().eye;
        assert!(
            e.x > 1.0 && (e.x - DEFAULT_ORBIT_DISTANCE * 0.1_f32.sin()).abs() < 1e-3,
            "yaw eye {e}"
        );

        let mut pitch = QuatOrbit::from_pose(start_pose());
        pitch.apply_intent(
            CameraIntent::Orbit {
                yaw_delta: 0.0,
                pitch_delta: 0.1,
            },
            vp,
        );
        assert!(pitch.pose().eye.y > 1.0, "pitch eye {}", pitch.pose().eye);
    }

    /// For the same `Orbit` intents from the same pose, the quaternion orbit and the Houdini
    /// turntable put the eye in the same place (yaw, pitch, and yaw followed by pitch).
    #[test]
    fn quat_and_houdini_move_the_eye_the_same_way() {
        let vp = ViewportSize::new(1, 1);
        let yaw: &[(f32, f32)] = &[(0.3, 0.0)];
        let pitch: &[(f32, f32)] = &[(0.0, 0.25)];
        let both: &[(f32, f32)] = &[(-0.4, 0.0), (0.0, -0.2)];
        for steps in [yaw, pitch, both] {
            let mut quat = QuatOrbit::from_pose(start_pose());
            let mut houdini = HoudiniOrbit::from_pose(start_pose());
            let start = quat.pose().eye.distance(houdini.pose().eye);
            assert!(start < 1e-3, "different start eyes, {start}");
            for &(yaw_delta, pitch_delta) in steps {
                let intent = CameraIntent::Orbit {
                    yaw_delta,
                    pitch_delta,
                };
                quat.apply_intent(intent, vp);
                houdini.apply_intent(intent, vp);
            }
            let (q, h) = (quat.pose().eye, houdini.pose().eye);
            assert!(
                q.distance(h) < 1e-3,
                "{steps:?}: quat eye {q} vs houdini eye {h}"
            );
        }
    }

    /// The two orbits from one pose (default `navigation.coast`), fed the same coast intent.
    fn coast_pair(intent: CameraIntent) -> (QuatOrbit, HoudiniOrbit) {
        let vp = ViewportSize::new(1, 1);
        let mut quat = QuatOrbit::from_pose(start_pose());
        let mut houdini = HoudiniOrbit::from_pose(start_pose());
        quat.apply_intent(intent, vp);
        houdini.apply_intent(intent, vp);
        (quat, houdini)
    }

    /// The coasts are the same motion on both orbits: an `OrbitInertia` puts the eye in the same
    /// place frame by frame (direction AND speed: same rate units, same coast), and both come
    /// to rest together, past the end of the coast.
    #[test]
    fn quat_and_houdini_coast_the_same_way() {
        let dt = 1.0 / 60.0;
        for (yaw_rate, pitch_rate) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 0.8), (0.0, -0.8)] {
            let intent = CameraIntent::OrbitInertia {
                yaw_rate,
                pitch_rate,
            };
            let (mut quat, mut houdini) = coast_pair(intent);
            let start = quat.pose().eye;
            for frame in 0..45 {
                let q_moving = quat.update_dynamics(dt);
                let h_moving = houdini.rig.update_dynamics(dt);
                assert_eq!(q_moving, h_moving, "frame {frame}: one coast stopped early");
                let (q, h) = (quat.pose().eye, houdini.pose().eye);
                assert!(
                    q.distance(h) < 1e-3,
                    "rates ({yaw_rate}, {pitch_rate}) frame {frame}: quat eye {q} vs houdini {h}"
                );
            }
            assert!(
                !quat.has_motion() && !houdini.rig.has_inertia(),
                "both at rest"
            );
            assert!(
                quat.pose().eye.distance(start) > 0.1,
                "the coast moved the eye"
            );
        }
    }

    /// The pan coasts match: the same world velocity (same pixel-to-world scale and screen axes
    /// from the level start pose), so the eye and target move together on both orbits.
    #[test]
    fn quat_and_houdini_pan_coasts_match() {
        let dt = 1.0 / 60.0;
        for (dx_rate, dy_rate) in [(300.0, 0.0), (-300.0, 120.0), (0.0, -250.0)] {
            let (mut quat, mut houdini) = coast_pair(CameraIntent::PanInertia { dx_rate, dy_rate });
            assert!(
                quat.has_motion() && houdini.rig.has_inertia(),
                "a pan coast starts"
            );
            let start = quat.target;
            for frame in 0..45 {
                assert_eq!(
                    quat.update_dynamics(dt),
                    houdini.rig.update_dynamics(dt),
                    "frame {frame}: one pan coast stopped early"
                );
                assert!(
                    quat.target.distance(houdini.rig.target()) < 1e-3,
                    "({dx_rate}, {dy_rate}) frame {frame}: {} vs {}",
                    quat.target,
                    houdini.rig.target()
                );
                assert!(quat.pose().eye.distance(houdini.pose().eye) < 1e-3);
            }
            assert!(quat.target.distance(start) > 0.05, "the pan coast moved");
        }
    }

    /// The zoom coasts match: after every frame the Quat and Houdini distances agree within
    /// 1e-4 relative (pins the log-rate units, the `exp(step)` model and the shared coast).
    #[test]
    fn quat_and_houdini_zoom_coasts_match() {
        let dt = 1.0 / 60.0;
        for rate in [0.1_f32, -0.1, 0.6] {
            let (mut quat, mut houdini) = coast_pair(CameraIntent::ZoomInertia { rate });
            let start = quat.distance;
            assert!(quat.has_motion(), "a zoom coast is motion");
            for frame in 0..60 {
                quat.update_dynamics(dt);
                houdini.rig.update_dynamics(dt);
                let (q, h) = (quat.distance, houdini.rig.distance());
                assert!(
                    (q - h).abs() <= 1e-4 * h,
                    "rate {rate} frame {frame}: quat {q} vs houdini {h}"
                );
            }
            assert!(
                (quat.distance - start) * rate > 0.0,
                "rate {rate} zoomed the wrong way"
            );
        }
    }

    /// The dolly-zoom coasts match: same lens and distance frame by frame.
    #[test]
    fn quat_and_houdini_dolly_zoom_coasts_match() {
        let dt = 1.0 / 60.0;
        for fov_rate in [0.2_f32, -0.2] {
            let (mut quat, mut houdini) = coast_pair(CameraIntent::DollyZoomInertia { fov_rate });
            assert!(quat.has_motion() && houdini.rig.has_inertia());
            let fov0 = quat.projection.fov_y;
            for frame in 0..45 {
                quat.update_dynamics(dt);
                houdini.rig.update_dynamics(dt);
                assert!(
                    (quat.projection.fov_y - houdini.rig.projection.fov_y).abs() < 1e-5
                        && (quat.distance - houdini.rig.distance()).abs() < 1e-4,
                    "rate {fov_rate} frame {frame}"
                );
            }
            assert!(
                (quat.projection.fov_y - fov0) * fov_rate > 0.0,
                "the lens moved the seeded way"
            );
        }
    }

    /// Every coast ends EXACTLY at its time on both backends: the frame count is the time over
    /// `dt` (one frame of float slack), then the pose is bit-stable.
    #[test]
    fn coasts_stop_exactly_at_their_time_on_both_backends() {
        let dt = 1.0 / 60.0;
        let intents = [
            CameraIntent::OrbitInertia {
                yaw_rate: 2.0,
                pitch_rate: -0.5,
            },
            CameraIntent::PanInertia {
                dx_rate: 200.0,
                dy_rate: 50.0,
            },
            CameraIntent::ZoomInertia { rate: 0.5 },
        ];
        for intent in intents {
            let (mut quat, mut houdini) = coast_pair(intent);
            let t = quat.navigation.coast.time;
            assert_eq!(t, houdini.rig.navigation.coast.time, "one default");
            let expected = (t / dt).ceil() as u32;
            for (name, frames) in [
                ("quat", count_frames(|| quat.update_dynamics(dt))),
                ("houdini", count_frames(|| houdini.rig.update_dynamics(dt))),
            ] {
                assert!(
                    frames + 1 >= expected && frames <= expected,
                    "{intent:?} {name}: {frames} frames for T = {t} s ({expected})"
                );
            }
            let (q, h) = (quat.pose(), houdini.pose());
            for _ in 0..10 {
                assert!(!quat.update_dynamics(dt) && !houdini.rig.update_dynamics(dt));
                assert_eq!((quat.pose(), houdini.pose()), (q, h), "{intent:?} drifted");
            }
        }
    }

    /// Steps `f` (one frame each, `true` = still moving) until it reports rest; the number of
    /// moving frames. Bounded, so a coast that never ends fails instead of hanging.
    fn count_frames(mut f: impl FnMut() -> bool) -> u32 {
        let mut frames = 0;
        while f() {
            frames += 1;
            assert!(frames < 1000, "the coast did not stop");
        }
        frames
    }

    /// `Stop` (a press) freezes the pose on both orbits: every coast ends and ten more frames
    /// move nothing.
    #[test]
    fn stop_freezes_the_pose_on_both_backends() {
        let vp = ViewportSize::new(1, 1);
        let dt = 1.0 / 60.0;
        let mut quat = QuatOrbit::from_pose(start_pose());
        let mut houdini = HoudiniOrbit::from_pose(start_pose());
        for intent in [
            CameraIntent::OrbitInertia {
                yaw_rate: 2.0,
                pitch_rate: 0.7,
            },
            CameraIntent::PanInertia {
                dx_rate: 200.0,
                dy_rate: 50.0,
            },
            CameraIntent::ZoomInertia { rate: 0.5 },
            CameraIntent::DollyZoomInertia { fov_rate: 0.2 },
        ] {
            quat.apply_intent(intent, vp);
            houdini.apply_intent(intent, vp);
        }
        for _ in 0..5 {
            assert!(quat.update_dynamics(dt) && houdini.rig.update_dynamics(dt));
        }
        quat.apply_intent(CameraIntent::Stop, vp);
        houdini.apply_intent(CameraIntent::Stop, vp);
        assert!(!quat.has_motion() && !houdini.rig.has_inertia());
        let (q, h) = (quat.pose(), houdini.pose());
        let (qd, hd) = (quat.projection.fov_y, houdini.rig.projection.fov_y);
        for _ in 0..10 {
            assert!(!quat.update_dynamics(dt) && !houdini.rig.update_dynamics(dt));
            assert_eq!(quat.pose(), q, "quat moved after Stop");
            assert_eq!(houdini.pose(), h, "houdini moved after Stop");
        }
        assert_eq!(
            (quat.projection.fov_y, houdini.rig.projection.fov_y),
            (qd, hd)
        );
    }

    /// A `LookInertia` on the trackball continues a `Look` drag: the eye ends where the same
    /// `Look` steps would have put it (same tumble, look sign convention); the Houdini orbit,
    /// which has no look, ignores it.
    #[test]
    fn look_inertia_continues_a_look_on_the_trackball() {
        let vp = ViewportSize::new(1, 1);
        let dt = 1.0 / 60.0;
        let mut coasting = QuatOrbit::from_pose(start_pose());
        coasting.apply_intent(
            CameraIntent::LookInertia {
                yaw_rate: 1.5,
                pitch_rate: 0.8,
            },
            vp,
        );
        assert!(coasting.has_motion());
        let mut looked = QuatOrbit::from_pose(start_pose());
        for _ in 0..200 {
            let before = coasting.orientation;
            if !coasting.update_dynamics(dt) && coasting.orientation == before {
                break;
            }
        }
        // The total tumble is v0 * T * R(1) per axis; replay it as one `Look` step.
        let c = coasting.navigation.coast.sanitized();
        let travel = c.time * crate::camera_core::coast::coast_remaining_factor(1.0, c.softness);
        looked.apply_intent(
            CameraIntent::Look {
                dyaw: 1.5 * travel,
                dpitch: 0.8 * travel,
            },
            vp,
        );
        // Interleaved small tumbles vs one big step differ at second order (yaw * pitch / 2,
        // ~0.02 rad here); a flipped sign would be ~0.3 rad off.
        assert!(
            coasting.pose().forward().dot(looked.pose().forward()) > 0.999,
            "the look coast ended off the look's own direction"
        );
        let mut houdini = HoudiniOrbit::from_pose(start_pose());
        houdini.apply_intent(
            CameraIntent::LookInertia {
                yaw_rate: 1.5,
                pitch_rate: 0.8,
            },
            vp,
        );
        assert!(!houdini.rig.has_inertia(), "the turntable has no look");
    }

    /// Direct edits (zoom, cursor zoom, pan) cancel a running coast, as on the Houdini rig.
    #[test]
    fn direct_edits_cancel_the_coasts() {
        let vp = ViewportSize::new(1, 1);
        let edits = [
            CameraIntent::Zoom { factor: 0.9 },
            CameraIntent::ZoomAtCursor {
                cursor_x: 0.5,
                cursor_y: 0.5,
                factor: 0.9,
            },
            CameraIntent::Pan {
                dx_px: 4.0,
                dy_px: 2.0,
            },
        ];
        for edit in edits {
            let (mut quat, _) = coast_pair(CameraIntent::OrbitInertia {
                yaw_rate: 1.0,
                pitch_rate: 0.0,
            });
            quat.apply_intent(CameraIntent::ZoomInertia { rate: 0.2 }, vp);
            quat.apply_intent(
                CameraIntent::PanInertia {
                    dx_rate: 100.0,
                    dy_rate: 0.0,
                },
                vp,
            );
            assert!(quat.has_motion());
            quat.apply_intent(edit, vp);
            assert!(!quat.has_motion(), "{edit:?} left a coast running");
        }
    }

    /// A zero, negative, NaN or infinite `dt` leaves the pose untouched and reports no step.
    #[test]
    fn unusable_dt_changes_nothing() {
        let (mut quat, _) = coast_pair(CameraIntent::OrbitInertia {
            yaw_rate: 1.0,
            pitch_rate: 0.5,
        });
        let before = quat.pose();
        for dt in [0.0, -1.0 / 60.0, f32::NAN, f32::INFINITY] {
            assert!(!quat.update_dynamics(dt), "dt {dt} stepped");
            assert_eq!(quat.pose(), before, "dt {dt} moved the pose");
        }
        assert!(quat.has_motion(), "the coast waits for a usable dt");
    }

    /// `ZoomAtCursor` keeps the world point under the cursor (on the target plane) under the
    /// cursor; at the viewport centre it is a plain zoom (the target does not move).
    #[test]
    fn zoom_at_cursor_keeps_the_point_under_the_cursor() {
        let size = ViewportSize::new(800, 600);
        // Where the ray through `point` meets the plane through the target facing the camera.
        let focus = |q: &QuatOrbit, point: ViewportPoint| {
            let pose = q.pose();
            let ray = pose.viewport_ray(point, size, q.projection);
            let (origin, dir) = (Vec3::from(ray.origin), Vec3::from(ray.dir));
            let f = pose.forward();
            origin + dir * ((q.target - origin).dot(f) / dir.dot(f))
        };
        let cursor = ViewportPoint { x: 600.0, y: 150.0 };
        let mut q = QuatOrbit::from_pose(start_pose());
        let before = focus(&q, cursor);
        q.apply_intent(
            CameraIntent::ZoomAtCursor {
                cursor_x: cursor.x,
                cursor_y: cursor.y,
                factor: 0.8,
            },
            size,
        );
        assert!((q.distance - 0.8 * DEFAULT_ORBIT_DISTANCE).abs() < 1e-3);
        let after = focus(&q, cursor);
        assert!(
            before.distance(after) < 1e-3,
            "the point under the cursor moved: {before} -> {after}"
        );

        let mut centred = QuatOrbit::from_pose(start_pose());
        let target = centred.target;
        centred.apply_intent(
            CameraIntent::ZoomAtCursor {
                cursor_x: 400.0,
                cursor_y: 300.0,
                factor: 0.8,
            },
            size,
        );
        assert!(
            centred.target.distance(target) < 1e-4,
            "a centred zoom moved the target"
        );
    }

    /// With a snap modifier held the input layer sends `OrbitSnap`: the trackball tumbles exactly
    /// as for `Orbit` (no plane snap here) instead of freezing, for every snap mode and on the
    /// gesture's first and later frames.
    #[test]
    fn orbit_snap_tumbles_like_orbit() {
        use crate::camera_core::intent::OrbitSnapMode;
        let vp = ViewportSize::new(1, 1);
        for mode in [
            OrbitSnapMode::Plane,
            OrbitSnapMode::Axis,
            OrbitSnapMode::Smart,
        ] {
            let mut snap = QuatOrbit::from_pose(start_pose());
            let mut plain = QuatOrbit::from_pose(start_pose());
            let steps = [(0.2_f32, 0.1_f32), (-0.05, 0.3)];
            for (i, (yaw_delta, pitch_delta)) in steps.into_iter().enumerate() {
                snap.apply_intent(
                    CameraIntent::OrbitSnap {
                        yaw_delta,
                        pitch_delta,
                        start: i == 0,
                        mode,
                    },
                    vp,
                );
                plain.apply_intent(
                    CameraIntent::Orbit {
                        yaw_delta,
                        pitch_delta,
                    },
                    vp,
                );
            }
            assert_eq!(snap.pose(), plain.pose(), "{mode:?}");
            assert!(
                snap.pose().eye.distance(start_pose().eye) > 1.0,
                "{mode:?}: the trackball froze"
            );
        }
    }

    /// `Look` turns the view the same way on every backend that takes it: the quaternion orbit's
    /// forward follows the forward of FpsFly and SpaceFlight for the same yaw and pitch deltas.
    #[test]
    fn quat_look_turns_like_the_fly_backends() {
        use crate::camera_core::controller::{FpsFly, SpaceFlight};
        let vp = ViewportSize::new(1, 1);
        for (dyaw, dpitch) in [(0.2, 0.0), (-0.2, 0.0), (0.0, 0.15), (0.0, -0.15)] {
            let look = CameraIntent::Look { dyaw, dpitch };
            let mut quat = QuatOrbit::from_pose(start_pose());
            let mut fps = FpsFly::from_pose(start_pose());
            let mut space = SpaceFlight::from_pose(start_pose());
            space.inertia.inertial_look = false; // the 1:1 look, compared in one step
            quat.apply_intent(look, vp);
            fps.apply_intent(look, vp);
            space.apply_intent(look, vp);
            let q = quat.pose().forward();
            for (name, f) in [
                ("fps", fps.pose().forward()),
                ("space", space.pose().forward()),
            ] {
                assert!(
                    q.dot(f) > 0.9999,
                    "look ({dyaw}, {dpitch}): quat forward {q} vs {name} forward {f}"
                );
            }
        }
    }
}
