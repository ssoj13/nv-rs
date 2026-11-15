//! [`FpsFly`]: the first-person fly backend of [`super::CameraController`].
//!
//! What: a free-flying eye with yaw (about world up) + clamped pitch and no roll, like an FPS
//! camera; WASD-style thrust moves the eye in the camera basis, gravity-free.
//!
//! Why: game-style navigation distinct from the spaceship: the heading stays level, so the
//! horizon never tilts.
//!
//! How: `Thrust` / `LookRate` record HELD axes that `update_dynamics` integrates with the real
//! `dt` (frame-rate independent); `Look` turns the head directly. The held thrust is a
//! [`Drive`]: the velocity approaches `basis * axes * cruise_speed * speed_scale` with time
//! constant `thrust_response`, and a release glides to an exact stop in the coast time
//! (`crate::camera_core::coast`). The look has its own [`Coast`] (`LookInertia` from a released pointer, or
//! the held deflection rate when the steering is centred); any `Look` or non-zero `LookRate`
//! cancels it, `Stop` cancels every coast.
//!
//! Uses: `crate::camera_core::coast`, `super::thrust`, `crate::camera_core::intent`, `super::frame_sphere`. Used by:
//! `super::facade`.

use glam::{Quat, Vec2, Vec3};

use super::thrust::{Drive, LOOK_COAST_UNIT, clamp_speed_scale, live, non_negative};
use crate::camera_core::coast::Coast;
use crate::camera_core::convention::WORLD_UP;
use crate::camera_core::intent::CameraIntent;
use crate::camera_core::orientation::DEFAULT_PITCH_LIMIT;
use crate::camera_core::pose::CameraPose;
use crate::camera_core::projection::PerspectiveSettings;
use crate::camera_core::settings::InertiaSettings;
use crate::camera_core::viewport::ViewportSize;

/// Floor of the horizontal forward length in the pitch `atan2`, so a vertical forward still gives
/// a finite ±90 degree pitch (then clamped).
const HORIZ_FLOOR: f32 = 1e-6;

/// A `View` direction keeps the current heading when its horizontal part is shorter than this
/// (straight up / down has no heading of its own).
const HEADING_MIN: f32 = 1e-4;

/// First-person fly backend: yaw / pitch mouse-look + WASD-style thrust.
///
/// The eye flies freely; the orientation is yaw about world up + pitch (clamped to
/// ±[`DEFAULT_PITCH_LIMIT`]) with no roll. `Thrust` moves the eye relative to the camera basis.
#[derive(Debug, Clone)]
pub struct FpsFly {
    /// World-space eye position.
    pub eye: Vec3,
    /// Yaw about world up (radians).
    pub yaw: f32,
    /// Pitch about camera right (radians, clamped to ±[`DEFAULT_PITCH_LIMIT`]).
    pub pitch: f32,
    /// Lens (fov / near / far; the distance range is used by framing).
    pub projection: PerspectiveSettings,
    /// Cruise speed, thrust response, coast and steering tuning.
    pub inertia: InertiaSettings,
    /// Linear thrust: the held velocity ramp and the release coast (world units/s).
    linear: Drive<Vec3>,
    /// Look coast (yaw / pitch rates in rad/s) after a released pointer or a centred steer.
    look: Coast<Vec2>,
    /// Thrust scale from the held speed modifiers (`SpeedScale`).
    speed_scale: f32,
    /// Held thrust axes (x = forward, y = right, z = up), refreshed every frame by the `Thrust`
    /// intent; `update_dynamics` turns them into the commanded velocity of `linear`.
    thrust_input: Vec3,
    /// Held mouse-steer axes (yaw / pitch in [-1, 1], deflection from the screen centre),
    /// refreshed every frame by `LookRate` in `MouseSteer::Deflection` and integrated into
    /// yaw / pitch with the real dt; zero in `MouseSteer::Relative` (per-frame `Look` instead).
    look_input: Vec2,
}

impl Default for FpsFly {
    fn default() -> Self {
        Self::from_pose(CameraPose::default())
    }
}

impl FpsFly {
    /// Build from a free pose: eye and lens (`projection_from_pose`) are kept, the forward is
    /// decomposed into yaw / pitch (roll is discarded: this rig cannot roll).
    pub fn from_pose(pose: CameraPose) -> Self {
        let (yaw, pitch) = Self::yaw_pitch(pose);
        Self {
            eye: pose.eye,
            yaw,
            pitch,
            projection: super::projection_from_pose(pose, PerspectiveSettings::default()),
            inertia: InertiaSettings::default(),
            linear: Drive::default(),
            look: Coast::default(),
            speed_scale: 1.0,
            thrust_input: Vec3::ZERO,
            look_input: Vec2::ZERO,
        }
    }

    /// Yaw / clamped pitch of the pose forward. Inverts `forward = Ry(yaw) * Rx(pitch) * -Z`,
    /// i.e. `forward = (-cos p sin y, sin p, -cos p cos y)`. Looking (almost) straight up or down
    /// the forward carries no heading, so the yaw comes from the screen right instead (a
    /// roll-free FPS right is `Ry(yaw) * X = (cos y, 0, -sin y)`): a Top / Bottom view switched to
    /// FPS keeps its image orientation instead of spinning by the rounding of a vertical forward.
    fn yaw_pitch(pose: CameraPose) -> (f32, f32) {
        let fwd = pose.forward();
        let horiz = Vec3::new(fwd.x, 0.0, fwd.z).length();
        let right = pose.right();
        let yaw = if horiz <= HEADING_MIN && Vec3::new(right.x, 0.0, right.z).length() > HEADING_MIN
        {
            (-right.z).atan2(right.x)
        } else {
            (-fwd.x).atan2(-fwd.z)
        };
        let pitch = fwd.y.atan2(horiz.max(HORIZ_FLOOR));
        (yaw, Self::clamp_pitch(pitch))
    }

    /// `pitch` within ±[`DEFAULT_PITCH_LIMIT`] (the panic-free `crate::camera_core::num::clamp`).
    fn clamp_pitch(pitch: f32) -> f32 {
        crate::camera_core::num::clamp(pitch, -DEFAULT_PITCH_LIMIT, DEFAULT_PITCH_LIMIT)
    }

    /// Move to `pose`, mutating only the pose (eye, yaw, pitch): the thrust velocity, the coasts
    /// and the held thrust / steer axes reset; lens, tuning and the speed scale are kept.
    pub fn set_pose(&mut self, pose: CameraPose) {
        let (yaw, pitch) = Self::yaw_pitch(pose);
        self.eye = pose.eye;
        self.yaw = yaw;
        self.pitch = pitch;
        self.linear.stop();
        self.look.stop();
        self.thrust_input = Vec3::ZERO;
        self.look_input = Vec2::ZERO;
    }

    /// Drop every held input (thrust and steer axes, speed modifier back to 1) as if all keys
    /// and the pointer were released. The host calls this on focus loss or whenever it stops
    /// emitting the per-frame held intents (a typing guard, a modal): the backend latches held
    /// axes until told otherwise, so without this call `has_motion` would never turn false.
    /// The linear velocity glides to rest in the coast time as after a key release; a steer axis
    /// is dropped without a look coast.
    pub fn release_inputs(&mut self) {
        self.thrust_input = Vec3::ZERO;
        self.look_input = Vec2::ZERO;
        self.speed_scale = 1.0;
    }

    /// Take over the held speed scale of the fly backend this one replaces (a mode switch,
    /// `CameraController::set_mode`); that backend only ever held a finite scale.
    pub(super) fn carry_speed_scale(&mut self, scale: f32) {
        self.speed_scale = clamp_speed_scale(scale);
    }

    /// Orientation from yaw / pitch: yaw about world up, then pitch about right.
    fn orientation(&self) -> Quat {
        Quat::from_axis_angle(WORLD_UP, self.yaw) * Quat::from_axis_angle(Vec3::X, self.pitch)
    }

    /// Forward direction implied by yaw / pitch.
    fn forward(&self) -> Vec3 {
        (self.orientation() * -Vec3::Z).normalize_or_zero()
    }

    /// Apply one intent: look turns the head, thrust / steer record held axes, speed scale
    /// multiplies the thrust, `FrameBounds` frames the box in the real `viewport`, `View` turns
    /// in place. Roll and orbit-only intents are ignored.
    ///
    /// `Orbit` on a fly backend IS a look: its deltas turn the forward exactly like `Look`
    /// (+pitch raises the forward). There is no pivot to swing the eye about, so its pitch is
    /// the opposite of the orbit backends', where +pitch raises the EYE (and lowers the
    /// forward). Hosts that switch styles mid-drag keep sending `Orbit` and get the fly look.
    ///
    /// Coasts: `LookInertia` starts the look coast; `Look` / `Orbit`, a non-zero `LookRate`,
    /// `View`, `FrameBounds` and `set_pose` cancel it; `Stop` cancels every coast. A `LookRate`
    /// that returns to zero starts the look coast from the rate it was turning at.
    ///
    /// An intent carrying a NaN or infinite value is dropped whole (`super::finite_intent`).
    pub fn apply_intent(&mut self, intent: CameraIntent, viewport: ViewportSize) {
        if !super::finite_intent(&intent) {
            return;
        }
        match intent {
            // `Orbit` = look here (see the doc above).
            CameraIntent::Look { dyaw, dpitch }
            | CameraIntent::Orbit {
                yaw_delta: dyaw,
                pitch_delta: dpitch,
            } => {
                self.look.stop();
                self.yaw += dyaw;
                self.pitch = Self::clamp_pitch(self.pitch + dpitch);
            }
            CameraIntent::LookInertia {
                yaw_rate,
                pitch_rate,
            } => {
                self.look.start(
                    Vec2::new(yaw_rate, pitch_rate),
                    LOOK_COAST_UNIT,
                    &self.inertia.coast,
                );
            }
            CameraIntent::Thrust { forward, right, up } => {
                // The host emits this every frame (zero on release), so the held state always
                // tracks the keys; the commanded velocity is applied with the real dt.
                self.thrust_input = Vec3::new(forward, right, up);
            }
            CameraIntent::LookRate { yaw, pitch } => {
                // Held deflection -> turn rate, integrated with the real dt in `integrate_look`.
                let next = Vec2::new(yaw, pitch);
                if next != Vec2::ZERO {
                    self.look.stop();
                } else if self.look_input != Vec2::ZERO {
                    // The steer is centred: the head keeps turning at its last rate and glides
                    // to rest in the coast time.
                    self.look.start(
                        self.look_input * non_negative(self.inertia.steer_rate),
                        LOOK_COAST_UNIT,
                        &self.inertia.coast,
                    );
                }
                self.look_input = next;
            }
            CameraIntent::SpeedScale(scale) => self.speed_scale = clamp_speed_scale(scale),
            CameraIntent::Stop => self.stop_coasts(),
            CameraIntent::FrameBounds { min, max, margin } => {
                self.frame_aabb(min, max, viewport.width, viewport.height, margin);
            }
            CameraIntent::View(view) => {
                // Turn in place to look along the view (no pivot to orbit about); straight up
                // or down keeps the heading. Pitch stays within the FPS limit. The flight and
                // the look coast stop (a coasting turn would swing away from the snap).
                let f = -view.dir();
                if Vec3::new(f.x, 0.0, f.z).length() > HEADING_MIN {
                    self.yaw = (-f.x).atan2(-f.z);
                }
                self.pitch = Self::clamp_pitch(crate::camera_core::num::clamp(f.y, -1.0, 1.0).asin());
                self.linear.stop();
                self.look.stop();
            }
            // FPS cannot roll; orbit-specific intents do not apply (`DollyZoom` keeps the size of
            // the subject at a pivot, and a fly rig has none).
            CameraIntent::Roll { .. }
            | CameraIntent::DollyZoom { .. }
            | CameraIntent::DollyZoomInertia { .. }
            | CameraIntent::Pan { .. }
            | CameraIntent::PanInertia { .. }
            | CameraIntent::OrbitInertia { .. }
            | CameraIntent::OrbitSnap { .. }
            | CameraIntent::Zoom { .. }
            | CameraIntent::ZoomInertia { .. }
            | CameraIntent::ZoomAtCursor { .. } => {}
        }
    }

    /// The commanded velocity of the held thrust: the axes in the camera basis times the cruise
    /// speed and the speed scale. `None` when no key is held or the command is zero (a zero
    /// speed scale) or not finite: the thrust counts as released.
    fn thrust_command(&self) -> Option<Vec3> {
        if self.thrust_input == Vec3::ZERO {
            return None;
        }
        let o = self.orientation();
        let f = (o * -Vec3::Z).normalize_or_zero();
        let r = (o * Vec3::X).normalize_or_zero();
        let axes = self.thrust_input;
        live(
            (f * axes.x + r * axes.y + WORLD_UP * axes.z)
                * (self.cruise_speed() * self.speed_scale),
        )
    }

    /// Integrate the held steer axes into yaw / pitch over the real `dt`: `steer_rate` rad/s at
    /// full deflection, pitch clamped so the FPS horizon stays sane.
    fn integrate_look(&mut self, dt: f32) {
        if self.look_input == Vec2::ZERO {
            return;
        }
        let rate = self.inertia.steer_rate * dt;
        self.yaw += self.look_input.x * rate;
        self.pitch = Self::clamp_pitch(self.pitch + self.look_input.y * rate);
    }

    /// Advance the held thrust (or its coast) and the look coast over `dt`, after integrating
    /// the held steering. Returns `true` while moving, coasting or actively steering; an
    /// unusable `dt` (`super::usable_dt`) changes nothing and returns `false`.
    pub fn update_dynamics(&mut self, dt: f32) -> bool {
        if !super::usable_dt(dt) {
            return false;
        }
        let command = self.thrust_command();
        let cruise = self.cruise_speed();
        self.eye += self.linear.step(
            command,
            cruise,
            self.inertia.thrust_response,
            &self.inertia.coast,
            dt,
        );
        self.integrate_look(dt);
        let turn = self.look.step(dt);
        if turn != Vec2::ZERO {
            self.yaw += turn.x;
            self.pitch = Self::clamp_pitch(self.pitch + turn.y);
        }
        self.has_motion()
    }

    /// True while the eye moves or coasts, the look coasts, or a thrust / steer axis is held:
    /// hosts keep ticking `update_dynamics` while this holds (and call [`Self::release_inputs`]
    /// when they stop emitting the held intents). Exact: a held thrust with a zero command (a
    /// zero speed scale) is no motion.
    pub fn has_motion(&self) -> bool {
        self.linear.has_motion()
            || self.look.is_active()
            || self.thrust_command().is_some()
            || self.look_input != Vec2::ZERO
    }

    /// The current world velocity of the eye (units/s): the held thrust velocity or the release
    /// coast's.
    pub fn linear_velocity(&self) -> Vec3 {
        self.linear.velocity()
    }

    /// Cancel every running coast (the release glide of the thrust and the look coast) at once:
    /// the grab-to-stop of a press. A held thrust keeps its velocity; a thrust released earlier
    /// in the same frame does not start its glide.
    pub fn stop_coasts(&mut self) {
        let command = self.thrust_command();
        self.linear.stop_coast(command);
        self.look.stop();
    }

    /// True while a release coast (thrust or look) is gliding.
    pub fn is_coasting(&self) -> bool {
        self.linear.is_coasting() || self.look.is_active()
    }

    /// The held speed-scale multiplier (`SpeedScale`; 1 = normal).
    pub fn speed_scale(&self) -> f32 {
        self.speed_scale
    }

    /// The held body-space thrust axes `(forward, right, up)`.
    pub fn thrust_input(&self) -> Vec3 {
        self.thrust_input
    }

    /// Nominal cruise speed (units/s) at speed scale 1: `InertiaSettings::cruise_speed` (a
    /// negative or NaN value reads as 0).
    pub fn cruise_speed(&self) -> f32 {
        non_negative(self.inertia.cruise_speed)
    }

    /// Screen-space look lead `(right, down)` in radians for a HUD reticle: the held deflection
    /// steering scaled by `LOOK_LEAD_STEER_GAIN`; zero at rest.
    pub fn look_lead(&self) -> Vec2 {
        -self.look_input * super::LOOK_LEAD_STEER_GAIN
    }

    /// The shared pose.
    pub fn pose(&self) -> CameraPose {
        CameraPose {
            eye: self.eye,
            orientation: self.orientation(),
            fov_y: self.projection.fov_y,
            znear: self.projection.near,
            zfar: self.projection.far,
        }
    }

    /// Frame an AABB in a `width x height` viewport: the eye backs off from the box centre
    /// along the current forward so its sphere fits (`super::frame_sphere`, finite for any
    /// bounds); velocity, coasts and held axes reset.
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
        self.eye = center - self.forward() * distance;
        self.linear.stop();
        self.look.stop();
        self.thrust_input = Vec3::ZERO;
        self.look_input = Vec2::ZERO;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DT: f32 = 1.0 / 60.0;

    fn vp() -> ViewportSize {
        ViewportSize::new(1, 1)
    }

    fn thrust(fps: &mut FpsFly, forward: f32) {
        fps.apply_intent(
            CameraIntent::Thrust {
                forward,
                right: 0.0,
                up: 0.0,
            },
            vp(),
        );
    }

    /// Run `seconds` of `update_dynamics` at `fps_rate`.
    fn run(fps: &mut FpsFly, seconds: f32, fps_rate: u32) {
        for _ in 0..(seconds * fps_rate as f32).round() as u32 {
            fps.update_dynamics(1.0 / fps_rate as f32);
        }
    }

    /// Frames until `update_dynamics` reports rest (bounded).
    fn frames_to_rest(fps: &mut FpsFly, dt: f32) -> u32 {
        let mut frames = 0;
        while fps.update_dynamics(dt) {
            frames += 1;
            assert!(frames < 10_000, "never came to rest");
        }
        frames
    }

    /// A Top / Bottom view (forward vertical, heading only in the up / right) switched to FPS
    /// keeps its image orientation from every heading: the yaw comes from the screen right, not
    /// from the rounding of a vertical forward.
    #[test]
    fn a_vertical_view_switched_to_fps_keeps_its_heading() {
        for &down in &[true, false] {
            for k in 0..8 {
                let heading = k as f32 * std::f32::consts::FRAC_PI_4 + 0.1;
                let up = Vec3::new(-heading.sin(), 0.0, -heading.cos());
                let forward = if down { -Vec3::Y } else { Vec3::Y };
                let right = forward.cross(up).normalize();
                let orientation =
                    glam::Quat::from_mat3(&glam::Mat3::from_cols(right, up, -forward)).normalize();
                let pose = CameraPose {
                    orientation,
                    ..CameraPose::default()
                };
                let fps = FpsFly::from_pose(pose);
                let got = fps.pose().up();
                // FPS pitch stops 1e-3 rad short of the pole, so the up tilts by that much.
                assert!(
                    got.angle_between(up) < 5e-3,
                    "down {down} heading {heading}: up {up} became {got}"
                );
            }
        }
    }

    /// A host that stops emitting the held intents (focus loss) calls `release_inputs`: the
    /// latched thrust / steer axes drop, the coast glides to rest and `has_motion` turns false.
    #[test]
    fn release_inputs_lets_a_latched_flight_come_to_rest() {
        let mut fps = FpsFly::default();
        fps.apply_intent(CameraIntent::SpeedScale(4.0), vp());
        thrust(&mut fps, 1.0);
        fps.apply_intent(
            CameraIntent::LookRate {
                yaw: 0.5,
                pitch: 0.0,
            },
            vp(),
        );
        for _ in 0..30 {
            fps.update_dynamics(DT);
        }
        assert!(fps.has_motion());
        // No further intents arrive: without the release the held axes would latch forever.
        fps.release_inputs();
        assert_eq!(fps.thrust_input(), Vec3::ZERO);
        assert_eq!(fps.speed_scale(), 1.0);
        frames_to_rest(&mut fps, DT);
        assert!(!fps.has_motion());
    }

    /// The released flight is at rest within the coast time (plus the frame that reaches it),
    /// the steer axis dropped by `release_inputs` starts no look coast.
    #[test]
    fn release_inputs_rests_within_the_coast_time() {
        let mut fps = FpsFly::default();
        thrust(&mut fps, 1.0);
        fps.apply_intent(
            CameraIntent::LookRate {
                yaw: 1.0,
                pitch: 0.0,
            },
            vp(),
        );
        run(&mut fps, 3.0, 60);
        let yaw = fps.yaw;
        fps.release_inputs();
        assert!(!fps.look.is_active(), "no look coast from a dropped steer");
        let frames = frames_to_rest(&mut fps, DT);
        let limit = (fps.inertia.coast.time / DT).ceil() as u32 + 1;
        assert!(frames <= limit, "{frames} frames, limit {limit}");
        assert_eq!(fps.yaw, yaw, "the dropped steer kept turning");
        assert_eq!(fps.linear_velocity(), Vec3::ZERO);
    }

    /// A zero, negative, NaN or infinite `dt` leaves the pose untouched and reports no step.
    #[test]
    fn unusable_dt_changes_nothing() {
        let mut fps = FpsFly::default();
        thrust(&mut fps, 1.0);
        fps.update_dynamics(DT);
        let before = fps.pose();
        for dt in [0.0, -DT, f32::NAN, f32::INFINITY] {
            assert!(!fps.update_dynamics(dt), "dt {dt} stepped");
            assert_eq!(fps.pose(), before, "dt {dt} moved the pose");
        }
    }

    /// The held speed approaches `cruise * (1 - e^(-t/response))` with the same value at 30 and
    /// 240 fps, and settles on the cruise speed.
    #[test]
    fn held_speed_approaches_cruise_equally_at_any_frame_rate() {
        let expected = |fps: &FpsFly, t: f32| {
            fps.cruise_speed() * (1.0 - (-t / fps.inertia.thrust_response).exp())
        };
        let mut slow = FpsFly::default();
        let mut fast = FpsFly::default();
        thrust(&mut slow, 1.0);
        thrust(&mut fast, 1.0);
        run(&mut slow, 1.0, 30);
        run(&mut fast, 1.0, 240);
        let want = expected(&slow, 1.0);
        assert!((slow.linear_velocity().length() - want).abs() < 1e-3);
        assert!((fast.linear_velocity().length() - want).abs() < 1e-3);
        assert!((slow.eye - fast.eye).length() < 1e-3, "travel differs");
        run(&mut fast, 5.0, 60);
        assert!((fast.linear_velocity().length() - fast.cruise_speed()).abs() < 1e-3);
    }

    /// A release glides and stops exactly at the coast time `T`, at every frame rate.
    #[test]
    fn release_stops_exactly_at_the_coast_time() {
        for rate in [30_u32, 60, 240] {
            let dt = 1.0 / rate as f32;
            let mut fps = FpsFly::default();
            thrust(&mut fps, 1.0);
            run(&mut fps, 3.0, rate);
            thrust(&mut fps, 0.0);
            let released_at = fps.eye;
            let frames = frames_to_rest(&mut fps, dt) + 1;
            let expected = (fps.inertia.coast.time * rate as f32).ceil() as u32;
            assert!(
                frames.abs_diff(expected) <= 1,
                "{rate} fps: {frames} frames, expected {expected}"
            );
            assert_eq!(fps.linear_velocity(), Vec3::ZERO);
            assert!(!fps.is_coasting() && !fps.has_motion());
            assert!(
                fps.eye.distance(released_at) > 0.1,
                "the release must glide"
            );
        }
    }

    /// A key during the coast continues from the coast speed: no jump.
    #[test]
    fn a_key_during_the_coast_has_no_velocity_jump() {
        let mut fps = FpsFly::default();
        thrust(&mut fps, 1.0);
        run(&mut fps, 3.0, 60);
        thrust(&mut fps, 0.0);
        run(&mut fps, 10.0 * DT, 60);
        assert!(fps.is_coasting());
        let before = fps.linear_velocity().length();
        assert!(before > 0.0 && before < fps.cruise_speed());
        thrust(&mut fps, 1.0);
        fps.update_dynamics(DT);
        let after = fps.linear_velocity().length();
        assert!(!fps.is_coasting(), "the key ended the coast");
        assert!(after >= before && after - before < 0.1 * fps.cruise_speed());
    }

    /// At `SpeedScale(0)` a held key commands nothing: the rig stays at rest, reports no motion,
    /// and the release starts no coast.
    #[test]
    fn speed_scale_zero_release_at_rest() {
        let mut fps = FpsFly::default();
        fps.apply_intent(CameraIntent::SpeedScale(0.0), vp());
        thrust(&mut fps, 1.0);
        let eye = fps.eye;
        assert!(!fps.update_dynamics(DT), "a zero command is no motion");
        assert!(!fps.has_motion());
        thrust(&mut fps, 0.0);
        assert!(!fps.update_dynamics(DT));
        assert!(!fps.is_coasting());
        assert_eq!(fps.eye, eye);
    }

    /// The speed scale is limited to a finite non-negative value.
    #[test]
    fn speed_scale_is_clamped() {
        let mut fps = FpsFly::default();
        fps.apply_intent(CameraIntent::SpeedScale(1e30), vp());
        assert_eq!(fps.speed_scale(), 1e6);
        fps.apply_intent(CameraIntent::SpeedScale(-2.0), vp());
        assert_eq!(fps.speed_scale(), 0.0);
        thrust(&mut fps, 1.0);
        fps.apply_intent(CameraIntent::SpeedScale(1e6), vp());
        for _ in 0..10 {
            fps.update_dynamics(DT);
            assert!(fps.eye.is_finite());
        }
    }

    /// `LookInertia` glides the head and stops it exactly at `T`; `Look` cancels it; `Stop`
    /// cancels it too.
    #[test]
    fn look_inertia_glides_then_stops_and_is_cancelled_by_look_and_stop() {
        let mut fps = FpsFly::default();
        fps.apply_intent(
            CameraIntent::LookInertia {
                yaw_rate: 1.0,
                pitch_rate: 0.0,
            },
            vp(),
        );
        assert!(fps.is_coasting() && fps.has_motion());
        // The default pose already has a yaw: measure the glide, not the heading.
        let start = fps.yaw;
        let mut last = fps.yaw;
        let mut frames = 0;
        while fps.update_dynamics(DT) {
            assert!(fps.yaw >= last);
            last = fps.yaw;
            frames += 1;
            assert!(frames <= 37, "the look coast outran its time");
        }
        let turned = fps.yaw - start;
        assert!(turned > 0.0 && turned < 0.6, "turned {turned}");
        assert!(!fps.is_coasting());

        for cancel in [
            CameraIntent::Look {
                dyaw: 0.0,
                dpitch: 0.0,
            },
            CameraIntent::Stop,
        ] {
            let mut fps = FpsFly::default();
            fps.apply_intent(
                CameraIntent::LookInertia {
                    yaw_rate: 1.0,
                    pitch_rate: 0.0,
                },
                vp(),
            );
            fps.update_dynamics(DT);
            fps.apply_intent(cancel, vp());
            let yaw = fps.yaw;
            assert!(!fps.update_dynamics(DT) && !fps.is_coasting());
            assert_eq!(fps.yaw, yaw, "{cancel:?} left the head turning");
        }
    }

    /// A centred steer (`LookRate` back to zero) glides on from its last rate; a non-zero rate
    /// cancels that glide.
    #[test]
    fn a_centred_steer_glides_and_a_new_rate_cancels_it() {
        let mut fps = FpsFly::default();
        fps.apply_intent(
            CameraIntent::LookRate {
                yaw: 1.0,
                pitch: 0.0,
            },
            vp(),
        );
        run(&mut fps, 5.0 * DT, 60);
        fps.apply_intent(
            CameraIntent::LookRate {
                yaw: 0.0,
                pitch: 0.0,
            },
            vp(),
        );
        assert!(fps.is_coasting());
        let yaw = fps.yaw;
        fps.update_dynamics(DT);
        assert!(fps.yaw > yaw, "the head stopped dead");
        frames_to_rest(&mut fps, DT);
        // A glide cancelled by a new non-zero rate.
        fps.apply_intent(
            CameraIntent::LookInertia {
                yaw_rate: 1.0,
                pitch_rate: 0.0,
            },
            vp(),
        );
        fps.apply_intent(
            CameraIntent::LookRate {
                yaw: 0.2,
                pitch: 0.0,
            },
            vp(),
        );
        assert!(!fps.look.is_active());
    }

    /// `Stop` ends the thrust glide and keeps a held thrust ramping; `View` stops the flight.
    #[test]
    fn stop_and_view_end_the_glide() {
        let mut fps = FpsFly::default();
        thrust(&mut fps, 1.0);
        run(&mut fps, 3.0, 60);
        thrust(&mut fps, 0.0);
        fps.update_dynamics(DT);
        assert!(fps.is_coasting());
        fps.apply_intent(CameraIntent::Stop, vp());
        assert!(!fps.is_coasting() && !fps.has_motion());

        thrust(&mut fps, 1.0);
        run(&mut fps, 1.0, 60);
        let held = fps.linear_velocity();
        fps.apply_intent(CameraIntent::Stop, vp());
        assert_eq!(fps.linear_velocity(), held, "a held thrust is not a coast");
        let Some(view) = crate::camera_core::ViewTarget::new(0, 0, 1) else {
            panic!("(0, 0, 1) is a view target");
        };
        fps.apply_intent(CameraIntent::View(view), vp());
        assert_eq!(
            fps.linear_velocity(),
            Vec3::ZERO,
            "View stops the linear motion"
        );
    }
}
