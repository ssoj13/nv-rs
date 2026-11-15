//! [`SpaceFlight`]: the 6-DoF spaceship backend of [`super::CameraController`].
//!
//! What: free eye + free orientation quaternion, body-relative thrust, roll, a speed scale,
//! a release glide, an optional horizon lock (`HorizonLock`), a held world alignment
//! (`set_align`) and an optional smoothed mouse look (`InertiaSettings::inertial_look`).
//!
//! Why: the most permissive style, for free exploration and flythroughs; the lock turns it into
//! a level spacesim without a second backend.
//!
//! How: held axes (thrust, roll, deflection steer) are recorded by `apply_intent` and applied
//! with the real `dt` in `update_dynamics`. The thrust and the free roll are [`Drive`]s: the
//! velocity approaches `axes * cruise * scale` (roll: `axes * roll_rate`) with time constant
//! `thrust_response`, and a release glides to an exact stop in the coast time (`crate::camera_core::coast`).
//! The look has its own [`Coast`] (`LookInertia`, or the held deflection rate when the steer
//! centres). Pointer look either turns 1:1 or enters a 2-stage cascaded exponential filter
//! (`look_stage1` -> `look_pending`) that ends exactly on the pointer's total turn (not a
//! coast); with `inertial_look` on, held deflection steering feeds the same filter, so a HUD
//! reticle can read the turn lead (`look_lead`). Every rotation goes through
//! `crate::camera_core::dynamics::rotate_view` (free, or yaw about the lock axis with the pole clamp). A
//! horizon lock never has a roll coast (the roll belongs to the lock spring); a held alignment
//! stops the look and roll coasts every frame.
//!
//! Uses: `crate::camera_core::coast`, `super::thrust`, `crate::camera_core::dynamics`, `crate::camera_core::orientation` (snap rule),
//! `super::lock`. Used by: `super::facade`.

use glam::{Mat3, Quat, Vec2, Vec3};

use super::lock::HorizonLock;
use super::thrust::{Drive, LOOK_COAST_UNIT, clamp_speed_scale, live, non_negative};
use crate::camera_core::coast::Coast;
use crate::camera_core::intent::{CameraIntent, OrbitSnapMode};
use crate::camera_core::pose::CameraPose;
use crate::camera_core::projection::PerspectiveSettings;
use crate::camera_core::settings::InertiaSettings;
use crate::camera_core::viewport::ViewportSize;

/// Below this time constant (seconds) the smoothed look is instantaneous: the queued turn is
/// applied in one step, and held steering bypasses the filter.
const LOOK_SMOOTHING_MIN: f32 = 1e-4;

/// The smoothed-look filter snaps to rest once the squared length of both stages is below this
/// (`1e-4` rad, ~0.0057 degrees: sub-pixel).
const LOOK_REST_EPS_SQ: f32 = 1e-8;

/// An alignment is reached when the residual rotation `orientation * target^-1` has a vector part
/// shorter than this: `|sin(angle / 2)|`, i.e. within ~2e-5 rad (~0.001 degrees, sub-pixel).
/// Measured on the vector part because `1 - |dot|` is below f32 resolution near 1 (a `1 - 1e-7`
/// threshold rounds to 1 and a converged Plane / Smart alignment never counted as reached).
const ALIGN_SIN_HALF_EPS: f32 = 1e-5;

/// An alignment time constant (seconds) at or below this snaps in one step.
const ALIGN_INSTANT_SECONDS: f32 = 1e-5;

/// Full 6-DoF "spaceship" backend.
///
/// A released thrust, a free roll and a released look glide to an exact stop in the coast time.
/// Free by default (the horizon rolls freely); with a horizon lock ([`Self::set_horizon_lock`])
/// it yaws about the lock axis and roll springs back to the lock plane.
#[derive(Debug, Clone)]
pub struct SpaceFlight {
    /// World-space eye position.
    pub eye: Vec3,
    /// Free camera orientation (local -Z forward, +Y up, +X right; roll allowed).
    pub orientation: Quat,
    /// Lens (fov / near / far; the distance range is used by framing).
    pub projection: PerspectiveSettings,
    /// Cruise speed, thrust response, coast, steering, smoothing and lock tuning.
    pub inertia: InertiaSettings,
    /// Linear thrust: the held velocity ramp and the release coast (world units/s).
    linear: Drive<Vec3>,
    /// Free roll: the held rate ramp and the release coast (rad/s). Idle under a horizon lock.
    roll: Drive<f32>,
    /// Look coast (yaw / pitch rates in rad/s) after a released pointer or a centred steer.
    look: Coast<Vec2>,
    /// Thrust scale from the held speed modifiers (`SpeedScale`).
    speed_scale: f32,
    /// Horizon lock (None = free 6-DoF).
    lock: Option<HorizonLock>,
    /// Held thrust axes (x = forward, y = right, z = up), refreshed every frame by `Thrust`.
    thrust_input: Vec3,
    /// Held roll axis (-1 = roll left / Q, +1 = roll right / E), refreshed every frame by
    /// `Roll`; commands the free roll rate or drives the lock spring (locked).
    roll_input: f32,
    /// Held mouse-steer axes (yaw / pitch in [-1, 1], deflection from the screen centre),
    /// refreshed every frame by `LookRate` in `MouseSteer::Deflection`.
    look_input: Vec2,
    /// First stage of the 2-stage cascaded exponential mouse-look filter: pointer deltas (and,
    /// with `inertial_look`, held steering) enter here and bleed into `look_pending`, which
    /// turns the view. The cascade gives a C1-continuous angular velocity (no velocity step on
    /// mouse start / stop) and ends on the exact total turn (no coast, no overshoot).
    look_stage1: Vec2,
    /// Second stage of the smoothed mouse-look filter.
    look_pending: Vec2,
    /// Held alignment ([`Self::set_align`]): snap mode and time constant (seconds).
    align: Option<(OrbitSnapMode, f32)>,
}

impl Default for SpaceFlight {
    fn default() -> Self {
        Self::from_pose(CameraPose::default())
    }
}

impl SpaceFlight {
    /// Build from a free pose: eye and orientation preserved exactly (including roll), lens from
    /// the pose (`projection_from_pose`).
    pub fn from_pose(pose: CameraPose) -> Self {
        Self {
            eye: pose.eye,
            orientation: pose.orientation,
            projection: super::projection_from_pose(pose, PerspectiveSettings::default()),
            inertia: InertiaSettings::default(),
            linear: Drive::default(),
            roll: Drive::default(),
            look: Coast::default(),
            speed_scale: 1.0,
            lock: None,
            thrust_input: Vec3::ZERO,
            roll_input: 0.0,
            look_input: Vec2::ZERO,
            look_stage1: Vec2::ZERO,
            look_pending: Vec2::ZERO,
            align: None,
        }
    }

    /// Move to `pose`, mutating only the pose (eye, orientation): the thrust / roll velocity,
    /// the coasts, held axes and the queued look reset. Lens, inertia tuning, speed scale, alignment and the horizon lock are
    /// kept; the lock re-seats (`HorizonLock::reseat`): on its requested axis when the new view
    /// is level to it or looks along it, on the current axis when the view looks along that, on
    /// another grid plane the view is level to, else back on the requested axis with the view's
    /// tilt as the spring roll, so a rolled restore levels back to the user's horizon.
    pub fn set_pose(&mut self, pose: CameraPose) {
        self.eye = pose.eye;
        self.orientation = pose.orientation;
        self.remeasure_lock();
        self.stop_motion();
        self.thrust_input = Vec3::ZERO;
        self.roll_input = 0.0;
        self.look_input = Vec2::ZERO;
        self.look_stage1 = Vec2::ZERO;
        self.look_pending = Vec2::ZERO;
    }

    /// Stop the thrust and roll velocities and the look coast at once (a jump of the pose).
    fn stop_motion(&mut self) {
        self.linear.stop();
        self.roll.stop();
        self.look.stop();
    }

    /// Take over the held speed scale of the fly backend this one replaces (a mode switch,
    /// `CameraController::set_mode`); that backend only ever held a finite scale.
    pub(super) fn carry_speed_scale(&mut self, scale: f32) {
        self.speed_scale = clamp_speed_scale(scale);
    }

    /// After a jump of the orientation (`set_pose`, a `View` snap): the lock re-selects the plane
    /// of its grid nearest the new orientation and re-measures the spring roll from it
    /// (`HorizonLock::reseat`), so the next `update_dynamics` keeps the view where it was put
    /// instead of rolling it toward a stale (possibly flipped) plane.
    fn remeasure_lock(&mut self) {
        if let Some(lock) = &mut self.lock {
            lock.reseat(self.orientation);
        }
    }

    /// Drop every held input as if all keys and the pointer were released: thrust, roll and
    /// steer axes, the speed modifier (back to 1) and a held alignment. The host calls this on
    /// focus loss or whenever it stops emitting the per-frame held intents (a typing guard, a
    /// modal): the backend latches held axes until told otherwise, so without this call
    /// `has_motion` would never turn false. The linear velocity glides to rest in the coast
    /// time as after a key release; a roll and a steer axis are dropped WITHOUT a coast (the
    /// roll velocity stops at once). A queued smoothed look and the lock spring finish on their
    /// own.
    pub fn release_inputs(&mut self) {
        self.thrust_input = Vec3::ZERO;
        self.roll_input = 0.0;
        self.look_input = Vec2::ZERO;
        self.speed_scale = 1.0;
        self.roll.stop();
        self.set_align(None);
    }

    /// The `(forward, up)` of a `View` snap. Free flight: [`crate::camera_core::ViewTarget::frame`] Y-up
    /// (upright off the poles, the heading carried along ±Y). Horizon-locked: upright to the
    /// lock's `home` axis instead, so the snap stays on the user's horizon for ANY lock axis
    /// (`ViewTarget::frame` only knows the Y-up / Z-up conventions): the screen up is `home`
    /// projected on the view plane (`HorizonLock::level`). Looking along `home` there is no such
    /// projection and the Y-up frame's up is used: for a ±Y home that is the carried current up
    /// (heading kept); for any other home it is world Y projected, and the lock keeps or re-seats
    /// its axis by `HorizonLock::reseat`.
    fn view_frame(&self, view: crate::camera_core::ViewTarget) -> (Vec3, Vec3) {
        let from_forward = self.orientation * -Vec3::Z;
        let from_up = self.orientation * Vec3::Y;
        let (f, y_up) = view.frame(crate::camera_core::UpAxis::Y, from_forward, from_up);
        let upright = self
            .lock
            .and_then(|lock| HorizonLock::level(lock.home(), f));
        (f, upright.unwrap_or(y_up))
    }

    /// Apply one intent: look turns the ship 1:1, or (`InertiaSettings::inertial_look`) is
    /// queued and eased in by `update_dynamics`; roll / thrust / steer are held axes integrated
    /// in `update_dynamics`; `SpeedScale` scales the thrust; `FrameBounds` frames the box in the
    /// real `viewport`; `View` turns in place.
    ///
    /// `Orbit` on a fly backend IS a look: its deltas turn the forward exactly like `Look`
    /// (+pitch raises the forward). There is no pivot to swing the eye about, so its pitch is
    /// the opposite of the orbit backends', where +pitch raises the EYE (and lowers the
    /// forward). Hosts that switch styles mid-drag keep sending `Orbit` and get the fly look.
    ///
    /// Coasts: `LookInertia` starts the look coast; `Look` / `Orbit`, a non-zero `LookRate`,
    /// `View`, `FrameBounds` and `set_pose` cancel it; a `LookRate` that returns to zero starts
    /// it from the rate it was turning at; `Stop` cancels every coast. A released thrust or
    /// free roll glides on its own (see the module docs).
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
                if self.inertia.inertial_look {
                    // Not a coast: a coasting turn runs on after the hand stops, the hand
                    // corrects late and the nose swings to and fro. The view chases the
                    // pointer's own total instead (`integrate_look_pending`); the release
                    // coast, if any, is a separate `LookInertia`.
                    self.look_stage1 += Vec2::new(dyaw, dpitch);
                } else {
                    // 1:1 look, through the same rotation rule as the coast and held steer.
                    self.orientation = crate::camera_core::dynamics::rotate_view(
                        self.orientation,
                        dyaw,
                        dpitch,
                        self.lock_up(),
                    );
                }
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
            CameraIntent::Roll { d } => {
                // Held roll axis (-1 / 0 / +1 from Q / E), refreshed every frame like thrust
                // and applied with the real dt, so a held key ramps frame-rate independently.
                self.roll_input = d;
            }
            CameraIntent::Thrust { forward, right, up } => {
                self.thrust_input = Vec3::new(forward, right, up);
            }
            CameraIntent::LookRate { yaw, pitch } => {
                // Held deflection -> turn rate, refreshed every frame (`integrate_look`).
                let next = Vec2::new(yaw, pitch);
                if next != Vec2::ZERO {
                    self.look.stop();
                } else if self.look_input != Vec2::ZERO {
                    // The steer is centred: the ship keeps turning at its last rate and glides
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
                // Turn in place; the drift stops. A queued smoothed look would turn away from
                // the snap and a stale lock roll would tilt it: both are dropped / re-seated,
                // exactly as `set_pose` does.
                let (f, u) = self.view_frame(view);
                let p = CameraPose::from_eye_forward_up(self.eye, f, u, 0.0, 0.0, 0.0);
                self.orientation = p.orientation;
                self.remeasure_lock();
                self.stop_motion();
                self.look_stage1 = Vec2::ZERO;
                self.look_pending = Vec2::ZERO;
            }
            // Orbit-only intents (`DollyZoom` keeps the size of the subject at a pivot, and a fly
            // rig has none).
            CameraIntent::OrbitInertia { .. }
            | CameraIntent::OrbitSnap { .. }
            | CameraIntent::Pan { .. }
            | CameraIntent::PanInertia { .. }
            | CameraIntent::Zoom { .. }
            | CameraIntent::ZoomInertia { .. }
            | CameraIntent::ZoomAtCursor { .. }
            | CameraIntent::DollyZoom { .. }
            | CameraIntent::DollyZoomInertia { .. } => {}
        }
    }

    /// The yaw axis handed to `rotate_view`: None in free flight; the lock axis when locked, or
    /// the lock's `home` while a held alignment (upright to `home`, see `aligned`) owns the frame.
    fn lock_up(&self) -> Option<Vec3> {
        self.lock
            .map(|l| if self.align.is_some() { l.home() } else { l.up })
    }

    /// The commanded velocity of the held thrust: the axes in the ship basis times the cruise
    /// speed and the speed scale. `None` when no key is held or the command is zero (a zero
    /// speed scale) or not finite: the thrust counts as released.
    fn thrust_command(&self) -> Option<Vec3> {
        if self.thrust_input == Vec3::ZERO {
            return None;
        }
        let f = (self.orientation * -Vec3::Z).normalize_or_zero();
        let r = (self.orientation * Vec3::X).normalize_or_zero();
        let u = (self.orientation * Vec3::Y).normalize_or_zero();
        let axes = self.thrust_input;
        live((f * axes.x + r * axes.y + u * axes.z) * (self.cruise_speed() * self.speed_scale))
    }

    /// The commanded roll rate (rad/s) of the held roll axis: `roll_input * roll_rate`. `None`
    /// under a horizon lock (the roll drives the lock spring there), with no key held or a zero
    /// roll rate.
    fn roll_command(&self) -> Option<f32> {
        if self.lock.is_some() {
            return None;
        }
        live(self.roll_input * non_negative(self.inertia.roll_rate))
    }

    /// Turn the view about its forward axis by `angle` radians (a free roll step).
    fn roll_by(&mut self, angle: f32) {
        if angle == 0.0 {
            return;
        }
        let fwd = (self.orientation * -Vec3::Z).normalize_or_zero();
        if fwd.length_squared() > super::AXIS_EPS_SQ {
            self.orientation = (Quat::from_axis_angle(fwd, angle) * self.orientation).normalize();
        }
    }

    /// Integrate the held steer axes (deflection -> `steer_rate` rad/s at full deflection) over
    /// the real `dt`. With `inertial_look` (and a non-zero `look_smoothing`) the turn feeds
    /// `look_stage1`, so `integrate_look_pending` smooths it and `look_lead` sees it; otherwise
    /// it rotates the view directly through the shared rotation rule. Zero axes = no-op.
    fn integrate_look(&mut self, dt: f32) {
        if self.look_input == Vec2::ZERO {
            return;
        }
        let rate = self.inertia.steer_rate * dt;
        if self.inertia.inertial_look && self.inertia.look_smoothing > LOOK_SMOOTHING_MIN {
            self.look_stage1 += self.look_input * rate;
        } else {
            self.orientation = crate::camera_core::dynamics::rotate_view(
                self.orientation,
                self.look_input.x * rate,
                self.look_input.y * rate,
                self.lock_up(),
            );
        }
    }

    /// Advance the 2-stage cascaded exponential mouse-look filter (`tau_stage = look_smoothing /
    /// 2`) over `dt`, snapping to rest once both stages are sub-pixel (`LOOK_REST_EPS_SQ`).
    /// Frame-rate independent, C1-continuous angular velocity, ends on the exact pointer total.
    /// Returns `true` while a turn is still queued.
    fn integrate_look_pending(&mut self, dt: f32) -> bool {
        if self.look_stage1 == Vec2::ZERO && self.look_pending == Vec2::ZERO {
            return false;
        }
        let tau = self.inertia.look_smoothing.max(0.0);
        let step = if tau <= LOOK_SMOOTHING_MIN {
            let total = self.look_stage1 + self.look_pending;
            self.look_stage1 = Vec2::ZERO;
            self.look_pending = Vec2::ZERO;
            total
        } else {
            let tau_stage = (tau * 0.5).max(LOOK_SMOOTHING_MIN);
            let f = 1.0 - (-dt / tau_stage).exp();
            let transfer = self.look_stage1 * f;
            self.look_stage1 -= transfer;
            self.look_pending += transfer;
            let mut step = self.look_pending * f;
            self.look_pending -= step;
            if self.look_stage1.length_squared() + self.look_pending.length_squared()
                < LOOK_REST_EPS_SQ
            {
                step += self.look_stage1 + self.look_pending;
                self.look_stage1 = Vec2::ZERO;
                self.look_pending = Vec2::ZERO;
            }
            step
        };
        self.orientation =
            crate::camera_core::dynamics::rotate_view(self.orientation, step.x, step.y, self.lock_up());
        self.look_stage1 != Vec2::ZERO || self.look_pending != Vec2::ZERO
    }

    /// Advance the thrust, the free roll and the look coast over the real `dt`, after the held
    /// steering. Free: the roll turns about the view forward; locked: yaw turns about the lock
    /// axis and roll is the lock's levelling spring (`HorizonLock`), after which the frame is
    /// rebuilt level to the lock plane. A held alignment stops the look and roll coasts. Returns
    /// `true` while still moving (including a glide, active steering, a queued look, an
    /// alignment and an unsettled roll); an unusable `dt` (`super::usable_dt`) changes nothing
    /// and returns `false`.
    pub fn update_dynamics(&mut self, dt: f32) -> bool {
        if !super::usable_dt(dt) {
            return false;
        }
        let thrust = self.thrust_command();
        let travel = self.linear.step(
            thrust,
            self.cruise_speed(),
            self.inertia.thrust_response,
            &self.inertia.coast,
            dt,
        );
        self.integrate_look(dt);
        let looking = self.integrate_look_pending(dt);
        let aligning = self.integrate_align(dt);
        // After the alignment: it stops the look and roll motion before they are stepped.
        let turn = self.look.step(dt);
        if turn != Vec2::ZERO {
            self.orientation =
                crate::camera_core::dynamics::rotate_view(self.orientation, turn.x, turn.y, self.lock_up());
        }
        let spin = if self.align.is_none() {
            self.roll.step(
                self.roll_command(),
                non_negative(self.inertia.roll_rate),
                self.inertia.thrust_response,
                &self.inertia.coast,
                dt,
            )
        } else {
            0.0
        };
        self.roll_by(spin);
        self.eye += travel;
        let mut moving =
            self.linear.has_motion() || self.roll.has_motion() || self.look.is_active();
        // A held alignment owns the frame (upright to the lock's `home`): the lock is idle
        // meanwhile (no spring, no reframe, its axis untouched) and re-seats when it ends.
        if self.align.is_none()
            && let Some(lock) = &mut self.lock
        {
            moving |= lock.step(self.orientation, self.roll_input, &self.inertia, dt);
            self.orientation = lock.frame(self.orientation);
        }
        moving |= thrust.is_some();
        moving || aligning || looking || self.look_input != Vec2::ZERO
    }

    /// Cancel every running coast (the release glide of the thrust and of the free roll, and
    /// the look coast) at once: the grab-to-stop of a press. A held thrust or roll keeps its
    /// velocity; a thrust or roll released earlier in the same frame does not start its glide.
    pub fn stop_coasts(&mut self) {
        let (thrust, roll) = (self.thrust_command(), self.roll_command());
        self.linear.stop_coast(thrust);
        self.roll.stop_coast(roll);
        self.look.stop();
    }

    /// True while a release coast (thrust, free roll or look) is gliding.
    pub fn is_coasting(&self) -> bool {
        self.linear.is_coasting() || self.roll.is_coasting() || self.look.is_active()
    }

    /// The current world velocity of the eye (units/s): the held thrust velocity or the release
    /// coast's.
    pub fn linear_velocity(&self) -> Vec3 {
        self.linear.velocity()
    }

    /// Hold (Some) or release (None) an alignment: while held, `update_dynamics` eases the view
    /// toward the nearest world plane (`Plane`), axis (`Axis`), or smart plane / axis (`Smart`)
    /// view with time constant `seconds`, upright to world up (free flight) or to the lock's
    /// requested axis (horizon-locked: the planes and axes of the lock's grid). The target is
    /// the orbit's snap rule ([`crate::camera_core::orientation::snapped_yaw_pitch`]) on the current view
    /// direction, taken in that grid's frame. A host
    /// sets it every frame from its own binding. A horizon lock is idle while it is held (its
    /// axis kept) and re-seats on the aligned view when it is released.
    pub fn set_align(&mut self, align: Option<(OrbitSnapMode, f32)>) {
        let ending = self.align.is_some() && align.is_none();
        self.align = align;
        if ending {
            self.remeasure_lock();
        }
    }

    /// The camera orientation of the grid view nearest the current one (`mode`), upright to the
    /// grid's up. Free flight: the world grid, upright to world Y. Horizon-locked: the lock's
    /// grid (Y turned onto `home`): the view is mapped into the grid frame by `G^-1`, snapped
    /// there and mapped back by `G`, so the target is upright to the user's lock axis and the
    /// lock re-seats on `home` when the alignment ends.
    fn aligned(&self, mode: OrbitSnapMode) -> Quat {
        let grid = self
            .lock
            .map_or(Quat::IDENTITY, |lock| lock.grid_rotation());
        (grid * Self::aligned_y_up(grid.conjugate() * self.orientation, mode)).normalize()
    }

    /// The world view nearest `orientation` (`mode`), level to world Y: the view direction in the
    /// orbit's turntable terms (the offset is the camera's back), snapped, and mapped directly
    /// back to the camera basis (preserving the quarter-turn yaw even at the `±Y` poles).
    fn aligned_y_up(orientation: Quat, mode: OrbitSnapMode) -> Quat {
        use crate::camera_core::orientation::{
            orientation_from_yaw_pitch, snapped_yaw_pitch, yaw_pitch_from_orientation,
        };
        let (right, up, back) = (
            orientation * Vec3::X,
            orientation * Vec3::Y,
            orientation * Vec3::Z,
        );
        // The turntable frame whose offset axis is the camera's back (its right is -right).
        let turntable = Quat::from_mat3(&Mat3::from_cols(back, up, -right)).normalize();
        let (yaw, pitch) = yaw_pitch_from_orientation(turntable);
        let (yaw, pitch) = snapped_yaw_pitch(yaw, pitch, mode);
        let snapped = orientation_from_yaw_pitch(yaw, pitch);
        Quat::from_mat3(&Mat3::from_cols(
            -(snapped * Vec3::Z),
            snapped * Vec3::Y,
            snapped * Vec3::X,
        ))
        .normalize()
    }

    /// Ease toward the held alignment; `true` while still turning. A held alignment owns the
    /// orientation: it stops the look and roll coasts every frame (before they are stepped).
    fn integrate_align(&mut self, dt: f32) -> bool {
        let Some((mode, seconds)) = self.align else {
            return false;
        };
        let target = self.aligned(mode);
        self.look.stop();
        self.roll.stop();
        if self.reached(target) {
            self.orientation = target;
            return false;
        }
        // A non-finite or tiny time constant snaps in one step (NaN would make the slerp NaN for
        // good, +inf would hold the alignment forever without moving); same rule as the orbit
        // rig's `align_seconds`.
        let t = if seconds.is_finite() && seconds > ALIGN_INSTANT_SECONDS {
            1.0 - (-dt / seconds).exp()
        } else {
            1.0
        };
        self.orientation = self.orientation.slerp(target, t).normalize();
        if self.reached(target) {
            self.orientation = target;
            return false;
        }
        true
    }

    /// The view is on `target` (within `ALIGN_SIN_HALF_EPS`). The vector part of the residual
    /// rotation is `±sin(angle / 2) * axis` for either quaternion sign, so `q` and `-q` (the same
    /// rotation in the other hemisphere) compare equal without a sign test.
    fn reached(&self, target: Quat) -> bool {
        (self.orientation * target.conjugate()).xyz().length() < ALIGN_SIN_HALF_EPS
    }

    /// A held alignment that has not yet reached its target view (a converged one is at rest).
    fn aligning(&self) -> bool {
        self.align
            .is_some_and(|(mode, _)| !self.reached(self.aligned(mode)))
    }

    /// The horizon lock axis, or None in free 6-DoF flight.
    pub fn horizon_lock(&self) -> Option<Vec3> {
        self.lock.map(|l| l.up)
    }

    /// Lock the horizon to `up` (any axis; its grid of planes is the world grid turned to it),
    /// or release it (None: free flight keeps the current orientation). The lock takes the grid
    /// plane nearest to the current roll, so the camera levels smoothly from where it is
    /// instead of snapping. Returns whether the request took effect: a zero axis leaves the
    /// view free and returns `false`; a view along the axis locks to the nearest side plane.
    ///
    /// Idempotent: repeating the current request (the same axis while locked, `None` while
    /// free) returns `true` and touches nothing (no spring reset, no roll velocity cleared), so
    /// a host may push its setting every frame. A change stops the free roll (velocity and
    /// coast): under a lock the roll belongs to the spring, and a freed view starts rolling
    /// from rest.
    pub fn set_horizon_lock(&mut self, up: Option<Vec3>) -> bool {
        let unchanged = match (up, self.lock) {
            (None, None) => true,
            (Some(up), Some(lock)) => up.normalize_or_zero() == lock.home(),
            _ => false,
        };
        if unchanged {
            return true;
        }
        self.lock = up.and_then(|up| HorizonLock::nearest(self.orientation, up));
        self.roll.stop();
        up.is_none() || self.lock.is_some()
    }

    /// Residual motion of any kind: a thrust / roll velocity or glide, the look coast, a held
    /// thrust / roll / steer axis that commands motion, a queued smoothed look, a held
    /// alignment still turning, or an unsettled lock spring (the spring is idle while an
    /// alignment owns the frame). Hosts keep ticking `update_dynamics` while this holds, and
    /// call [`Self::release_inputs`] when they stop emitting the held intents. Exact: a held
    /// thrust with a zero command (a zero speed scale) is no motion.
    pub fn has_motion(&self) -> bool {
        self.linear.has_motion()
            || self.roll.has_motion()
            || self.look.is_active()
            || self.thrust_command().is_some()
            || (self.align.is_none()
                && (self.roll_command().is_some()
                    || (self.lock.is_some() && self.roll_input != 0.0)))
            || self.look_input != Vec2::ZERO
            || self.look_stage1 != Vec2::ZERO
            || self.look_pending != Vec2::ZERO
            || self.aligning()
            || (self.align.is_none() && self.lock.is_some_and(|l| l.moving()))
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

    /// Screen-space look lead `(right, down)` in radians for a HUD reticle, zero at rest. Driven
    /// mainly by the second stage of the smoothed look (already low-passed against the discrete
    /// mouse-poll steps of the first), so a reticle moves smoothly; with no smoothed turn
    /// queued, the held deflection steering leads instead.
    pub fn look_lead(&self) -> Vec2 {
        let smooth = self.look_pending * super::LOOK_LEAD_PENDING_GAIN
            + self.look_stage1 * super::LOOK_LEAD_STAGE1_GAIN;
        if smooth != Vec2::ZERO {
            -smooth
        } else {
            -self.look_input * super::LOOK_LEAD_STEER_GAIN
        }
    }

    /// The shared pose.
    pub fn pose(&self) -> CameraPose {
        CameraPose {
            eye: self.eye,
            orientation: self.orientation,
            fov_y: self.projection.fov_y,
            znear: self.projection.near,
            zfar: self.projection.far,
        }
    }

    /// Frame an AABB in a `width x height` viewport: the eye backs off from the box centre
    /// along the current forward so its sphere fits (`super::frame_sphere`, finite for any
    /// bounds), keeping the orientation (roll included); velocities, coasts, held axes and the
    /// queued look reset.
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
        let fwd = (self.orientation * -Vec3::Z).normalize_or_zero();
        self.eye = center - fwd * distance;
        self.stop_motion();
        self.thrust_input = Vec3::ZERO;
        self.roll_input = 0.0;
        self.look_input = Vec2::ZERO;
        self.look_stage1 = Vec2::ZERO;
        self.look_pending = Vec2::ZERO;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DT: f32 = 1.0 / 60.0;

    fn vp() -> ViewportSize {
        ViewportSize::new(1, 1)
    }

    fn thrust(s: &mut SpaceFlight, forward: f32) {
        s.apply_intent(
            CameraIntent::Thrust {
                forward,
                right: 0.0,
                up: 0.0,
            },
            vp(),
        );
    }

    /// Run `seconds` of `update_dynamics` at `rate` fps.
    fn run(s: &mut SpaceFlight, seconds: f32, rate: u32) {
        for _ in 0..(seconds * rate as f32).round() as u32 {
            s.update_dynamics(1.0 / rate as f32);
        }
    }

    #[test]
    fn space_thrust_and_roll_move_then_glide_to_rest() {
        let mut s = SpaceFlight::default();
        let eye0 = s.eye;
        let orient0 = s.orientation;
        s.apply_intent(
            CameraIntent::Thrust {
                forward: 1.0,
                right: 0.0,
                up: 0.0,
            },
            ViewportSize::new(1, 1),
        );
        // Roll is a HELD axis (Q/E), integrated with dt like thrust: it stays applied every
        // frame until released, so set-once + coast turns the ship.
        s.apply_intent(CameraIntent::Roll { d: 1.0 }, ViewportSize::new(1, 1));
        let mut moved = false;
        for _ in 0..5 {
            moved |= s.update_dynamics(1.0 / 60.0);
        }
        assert!(moved, "spaceflight should report motion while coasting");
        assert!(
            (s.eye - eye0).length() > 1e-3,
            "thrust did not move the eye"
        );
        assert!(
            orient0.angle_between(s.orientation) > 1e-4,
            "roll did not turn the ship"
        );
        // Release thrust AND roll (the host emits zero on key-up): both glide to rest.
        s.apply_intent(
            CameraIntent::Thrust {
                forward: 0.0,
                right: 0.0,
                up: 0.0,
            },
            ViewportSize::new(1, 1),
        );
        s.apply_intent(CameraIntent::Roll { d: 0.0 }, ViewportSize::new(1, 1));
        let mut frames = 0;
        while s.update_dynamics(1.0 / 60.0) {
            frames += 1;
            assert!(frames <= 100_000, "spaceflight never came to rest");
        }
        assert_eq!(
            s.linear_velocity(),
            Vec3::ZERO,
            "the thrust should be at rest"
        );
        assert!(!s.is_coasting(), "the glides ended");
        assert!(!s.has_motion(), "released axes leave no motion");
    }

    #[test]
    fn space_mouse_steer_turns_then_glides_to_rest() {
        let mut s = SpaceFlight::default();
        let o0 = s.orientation;
        // Held deflection (yaw right) turns the ship continuously over dt.
        s.apply_intent(
            CameraIntent::LookRate {
                yaw: 1.0,
                pitch: 0.0,
            },
            ViewportSize::new(1, 1),
        );
        for _ in 0..5 {
            s.update_dynamics(1.0 / 60.0);
        }
        assert!(
            o0.angle_between(s.orientation) > 1e-3,
            "deflection steer did not turn the ship"
        );
        // Recentre (zero axis): the ship keeps turning at its last rate and glides to rest.
        let o1 = s.orientation;
        s.apply_intent(
            CameraIntent::LookRate {
                yaw: 0.0,
                pitch: 0.0,
            },
            ViewportSize::new(1, 1),
        );
        assert!(s.is_coasting());
        s.update_dynamics(1.0 / 60.0);
        assert!(
            o1.angle_between(s.orientation) > 1e-4,
            "the steer stopped dead after the cursor recentred"
        );
        settle(&mut s);
        assert!(!s.has_motion() && !s.is_coasting());
        // A deflection that comes back cancels the glide.
        s.apply_intent(
            CameraIntent::LookInertia {
                yaw_rate: 1.0,
                pitch_rate: 0.0,
            },
            ViewportSize::new(1, 1),
        );
        s.apply_intent(
            CameraIntent::LookRate {
                yaw: 0.3,
                pitch: 0.0,
            },
            ViewportSize::new(1, 1),
        );
        assert!(!s.look.is_active());
    }

    /// Run `update_dynamics` until it reports rest (bounded).
    fn settle(s: &mut SpaceFlight) {
        let mut frames = 0;
        while s.update_dynamics(1.0 / 60.0) {
            frames += 1;
            assert!(frames < 100_000, "spaceflight never came to rest");
        }
    }

    /// Focus loss: the host stops emitting held intents and calls `release_inputs`; the latched
    /// thrust / roll / steer axes and a held alignment drop and the ship comes to rest.
    #[test]
    fn release_inputs_lets_a_latched_flight_come_to_rest() {
        let mut s = SpaceFlight::default();
        let vp = ViewportSize::new(1, 1);
        s.apply_intent(CameraIntent::SpeedScale(4.0), vp);
        s.apply_intent(
            CameraIntent::Thrust {
                forward: 1.0,
                right: 0.5,
                up: 0.0,
            },
            vp,
        );
        s.apply_intent(CameraIntent::Roll { d: 1.0 }, vp);
        s.apply_intent(
            CameraIntent::LookRate {
                yaw: 0.3,
                pitch: 0.2,
            },
            vp,
        );
        s.set_align(Some((OrbitSnapMode::Axis, 0.05)));
        for _ in 0..30 {
            s.update_dynamics(1.0 / 60.0);
        }
        assert!(s.has_motion());
        s.release_inputs();
        assert_eq!(s.thrust_input(), Vec3::ZERO);
        assert_eq!(s.speed_scale(), 1.0);
        settle(&mut s);
        assert!(!s.has_motion(), "released inputs leave no motion");
    }

    /// A held alignment that has converged is at rest: after settling, neither `has_motion` nor
    /// `update_dynamics` reports motion, for every snap mode, free and horizon-locked, over a
    /// sweep of 50 view directions (10 yaws x 5 pitches).
    #[test]
    fn a_converged_held_alignment_is_at_rest() {
        let modes = [
            OrbitSnapMode::Axis,
            OrbitSnapMode::Plane,
            OrbitSnapMode::Smart,
        ];
        let pitches = [-1.2_f32, -0.55, 0.07, 0.62, 1.25];
        for mode in modes {
            for lock in [None, Some(Vec3::Y)] {
                for i in 0..10 {
                    let yaw = -3.0 + i as f32 * 0.61 + 0.13;
                    for pitch in pitches {
                        let forward = Vec3::new(
                            pitch.cos() * yaw.sin(),
                            pitch.sin(),
                            pitch.cos() * yaw.cos(),
                        );
                        let pose = CameraPose::from_eye_forward_up(
                            Vec3::ZERO,
                            forward,
                            Vec3::Y,
                            0.8,
                            0.1,
                            500.0,
                        );
                        let mut s = SpaceFlight::from_pose(pose);
                        assert!(s.set_horizon_lock(lock));
                        s.set_align(Some((mode, 0.05)));
                        settle(&mut s);
                        assert!(
                            !s.has_motion(),
                            "{mode:?} lock {lock:?} yaw {yaw} pitch {pitch}: converged but moving"
                        );
                        assert!(
                            !s.update_dynamics(1.0 / 60.0),
                            "{mode:?} lock {lock:?} yaw {yaw} pitch {pitch}: still stepping"
                        );
                    }
                }
            }
        }
    }

    /// A `View` snap after a held roll flipped the lock onto a wall plane re-seats the lock on
    /// the plane the snapped view is level to, so the view stays put instead of rolling toward
    /// the stale wall plane.
    #[test]
    fn view_snap_after_a_lock_flip_reseats_the_lock_and_stays() {
        let vp = ViewportSize::new(1, 1);
        let Some(front) = crate::camera_core::ViewTarget::new(0, 0, 1) else {
            panic!("(0, 0, 1) is a view target");
        };
        let mut s = SpaceFlight::default();
        assert!(s.set_horizon_lock(Some(Vec3::Y)));
        s.apply_intent(CameraIntent::Roll { d: 1.0 }, vp);
        let mut frames = 0;
        while s.horizon_lock() == Some(Vec3::Y) {
            s.update_dynamics(1.0 / 60.0);
            frames += 1;
            assert!(frames < 600, "a held roll must flip the lock");
        }
        s.apply_intent(CameraIntent::Roll { d: 0.0 }, vp);
        s.apply_intent(CameraIntent::View(front), vp);
        let snapped = s.orientation;
        let Some(up) = s.horizon_lock() else {
            panic!("the lock survives the snap");
        };
        assert!(up.distance(Vec3::Y) < 1e-5, "re-seated on {up}, not Y");
        for _ in 0..120 {
            s.update_dynamics(1.0 / 60.0);
        }
        let drift = snapped.angle_between(s.orientation);
        assert!(drift < 1e-3, "the snapped view rolled away by {drift} rad");
    }

    /// A zero, negative, NaN or infinite `dt` leaves the pose untouched and reports no step.
    #[test]
    fn unusable_dt_changes_nothing() {
        let vp = ViewportSize::new(1, 1);
        let mut s = SpaceFlight::default();
        s.inertia.inertial_look = true;
        s.apply_intent(
            CameraIntent::Thrust {
                forward: 1.0,
                right: 0.0,
                up: 0.0,
            },
            vp,
        );
        s.apply_intent(
            CameraIntent::Look {
                dyaw: 0.3,
                dpitch: 0.1,
            },
            vp,
        );
        s.update_dynamics(1.0 / 60.0);
        let before = s.pose();
        for dt in [0.0, -1.0 / 60.0, f32::NAN, f32::INFINITY] {
            assert!(!s.update_dynamics(dt), "dt {dt} stepped");
            assert_eq!(s.pose(), before, "dt {dt} moved the pose");
        }
    }

    /// A `View` snap on a horizon-locked ship with a queued smoothed look and a tilted lock
    /// spring stays on the snapped view: the queued look is dropped and the lock re-seats on the
    /// snap (side view: the Y plane it is level to; top view: the Y axis it looks along, with no
    /// spring roll).
    #[test]
    fn view_snap_on_a_locked_ship_stays_on_the_snapped_view() {
        let vp = ViewportSize::new(1, 1);
        for (x, y, z) in [(0, 0, 1), (0, 1, 0)] {
            let Some(view) = crate::camera_core::ViewTarget::new(x, y, z) else {
                panic!("({x}, {y}, {z}) is a view target");
            };
            let mut s = SpaceFlight::default();
            s.inertia.inertial_look = true;
            assert!(s.set_horizon_lock(Some(Vec3::Y)));
            // Tilt the lock spring with a short roll press, then release the key.
            s.apply_intent(CameraIntent::Roll { d: 1.0 }, vp);
            for _ in 0..6 {
                s.update_dynamics(1.0 / 60.0);
            }
            s.apply_intent(CameraIntent::Roll { d: 0.0 }, vp);
            // Queue a smoothed look, then snap before it has been eased in.
            s.apply_intent(
                CameraIntent::Look {
                    dyaw: 0.4,
                    dpitch: 0.2,
                },
                vp,
            );
            s.apply_intent(CameraIntent::View(view), vp);
            let snapped = s.orientation;
            assert!(
                s.pose().forward().distance(-view.dir()) < 1e-4,
                "the snap looks along the view"
            );
            // The lock stays on Y: level to the side view, and for the top view the axis the
            // view looks along (a wall plane would measure the snapped heading as a roll and the
            // spring would roll the view to level it).
            assert_eq!(s.horizon_lock(), Some(Vec3::Y), "view ({x}, {y}, {z})");
            if y != 0 {
                // Along the lock axis there is no plane to level to: nothing to settle at all.
                assert!(!s.has_motion(), "the top snap left the lock spring moving");
            }
            for _ in 0..120 {
                s.update_dynamics(1.0 / 60.0);
            }
            let drift = snapped.angle_between(s.orientation);
            assert!(drift < 1e-3, "view ({x}, {y}, {z}) drifted by {drift} rad");
        }
    }

    /// Snap to `(x, y, z)`, check the lock axis is `expect`, run two seconds and return the drift
    /// from the snapped orientation.
    fn snap_and_drift(s: &mut SpaceFlight, (x, y, z): (i8, i8, i8), expect: Vec3) -> f32 {
        let Some(view) = crate::camera_core::ViewTarget::new(x, y, z) else {
            panic!("({x}, {y}, {z}) is a view target");
        };
        s.apply_intent(CameraIntent::View(view), ViewportSize::new(1, 1));
        let snapped = s.orientation;
        let Some(up) = s.horizon_lock() else {
            panic!("the lock survives the snap");
        };
        assert!(
            up.distance(expect) < 1e-5,
            "view ({x}, {y}, {z}): lock on {up}, expected {expect}"
        );
        for _ in 0..120 {
            s.update_dynamics(1.0 / 60.0);
        }
        snapped.angle_between(s.orientation)
    }

    /// An untilted Front snap then a Top (or Bottom) snap under a Y lock: `View` carries the up
    /// to exactly -Z (+Z), which makes that wall plane level too, yet the lock stays on Y (the
    /// view looks along the current lock axis) and the view does not drift.
    #[test]
    fn front_then_top_or_bottom_under_a_y_lock_keeps_y() {
        for second in [(0, 1, 0), (0, -1, 0)] {
            let mut s = SpaceFlight::default();
            assert!(s.set_horizon_lock(Some(Vec3::Y)));
            let front = snap_and_drift(&mut s, (0, 0, 1), Vec3::Y);
            assert!(front < 1e-3, "front drifted by {front} rad");
            let pole = snap_and_drift(&mut s, second, Vec3::Y);
            assert!(pole < 1e-3, "{second:?} drifted by {pole} rad");
        }
    }

    /// A `View` snap under a non-Y lock is framed upright to the lock's own axis (here Z), so
    /// the lock stays on Z instead of being reset to Y, and the view does not drift.
    #[test]
    fn view_snap_under_a_z_lock_is_upright_to_z() {
        let pose =
            CameraPose::from_eye_forward_up(Vec3::ZERO, Vec3::NEG_X, Vec3::Z, 0.8, 0.1, 500.0);
        let mut s = SpaceFlight::from_pose(pose);
        assert!(s.set_horizon_lock(Some(Vec3::Z)));
        // (0, 1, 0): the camera on +Y looking at the pivot, a side view for a Z horizon.
        let drift = snap_and_drift(&mut s, (0, 1, 0), Vec3::Z);
        assert!(
            s.pose().up().distance(Vec3::Z) < 1e-4,
            "the snap is upright to the lock axis, up {}",
            s.pose().up()
        );
        assert!(drift < 1e-3, "drifted by {drift} rad");
    }

    /// A held alignment never overwrites the user's lock axis: with the lock on X, an alignment
    /// onto the +X axis view and its release leave the lock on X (the view looks along it).
    #[test]
    fn alignment_keeps_the_lock_axis() {
        let pose =
            CameraPose::from_eye_forward_up(Vec3::ZERO, Vec3::NEG_Z, Vec3::X, 0.8, 0.1, 500.0);
        let mut s = SpaceFlight::from_pose(pose);
        assert!(s.set_horizon_lock(Some(Vec3::X)));
        let is_x = |s: &SpaceFlight| {
            s.horizon_lock()
                .is_some_and(|up| up.distance(Vec3::X) < 1e-5)
        };
        assert!(is_x(&s), "locked on X");
        // Pitch up toward the lock axis: the pole clamp stops 1 degree short of +X.
        for _ in 0..50 {
            s.apply_intent(
                CameraIntent::Look {
                    dyaw: 0.0,
                    dpitch: 0.2,
                },
                ViewportSize::new(1, 1),
            );
        }
        assert!(s.pose().forward().dot(Vec3::X) > 0.99, "near +X");
        s.set_align(Some((OrbitSnapMode::Axis, 0.05)));
        settle(&mut s);
        assert!(
            is_x(&s),
            "the alignment rewrote the lock to {:?}",
            s.horizon_lock()
        );
        s.set_align(None);
        assert!(is_x(&s), "released onto {:?}", s.horizon_lock());
        let aligned = s.orientation;
        for _ in 0..60 {
            s.update_dynamics(1.0 / 60.0);
        }
        assert!(
            aligned.angle_between(s.orientation) < 1e-3,
            "the aligned view drifted"
        );
        assert!(s.pose().forward().distance(Vec3::X) < 1e-4);
    }

    /// Repeating the current horizon-lock request touches nothing: free flight keeps its roll
    /// velocity, a locked flight keeps its spring state exactly (twin comparison).
    #[test]
    fn set_horizon_lock_is_idempotent() {
        let vp = ViewportSize::new(1, 1);
        let mut free = SpaceFlight::default();
        free.apply_intent(CameraIntent::Roll { d: 1.0 }, vp);
        for _ in 0..5 {
            free.update_dynamics(1.0 / 60.0);
        }
        let spin = free.roll.velocity();
        assert!(spin != 0.0);
        assert!(free.set_horizon_lock(None));
        assert_eq!(
            free.roll.velocity(),
            spin,
            "the free roll velocity was cleared"
        );

        let mut locked = SpaceFlight::default();
        assert!(locked.set_horizon_lock(Some(Vec3::Y)));
        locked.apply_intent(CameraIntent::Roll { d: 1.0 }, vp);
        for _ in 0..6 {
            locked.update_dynamics(1.0 / 60.0);
        }
        locked.apply_intent(CameraIntent::Roll { d: 0.0 }, vp);
        let mut twin = locked.clone();
        assert!(locked.set_horizon_lock(Some(Vec3::Y)));
        assert!(
            locked.set_horizon_lock(Some(Vec3::Y * 2.0)),
            "the same axis, unnormalized"
        );
        for _ in 0..30 {
            locked.update_dynamics(1.0 / 60.0);
            twin.update_dynamics(1.0 / 60.0);
            assert_eq!(locked.orientation, twin.orientation, "the spring was reset");
        }
    }

    /// The view right axis lies in the plane of `axis` and the view up leans toward it.
    fn upright_to(s: &SpaceFlight, axis: Vec3) -> bool {
        (s.orientation * Vec3::X).dot(axis).abs() < 2e-3
            && (s.orientation * Vec3::Y).dot(axis) > 0.0
    }

    /// A rolled restore looking along a grid axis that is neither the lock axis nor home
    /// (Y lock, forward -X rolled 30 degrees) keeps the lock on Y and levels the view back to
    /// it, instead of locking onto X with the tilt never corrected.
    #[test]
    fn rolled_restore_along_a_wall_axis_levels_back_to_home() {
        let mut s = SpaceFlight::default();
        assert!(s.set_horizon_lock(Some(Vec3::Y)));
        let up = Quat::from_axis_angle(Vec3::NEG_X, 30.0_f32.to_radians()) * Vec3::Y;
        let pose = CameraPose::from_eye_forward_up(Vec3::ZERO, Vec3::NEG_X, up, 0.8, 0.1, 500.0);
        s.set_pose(pose);
        assert_eq!(s.horizon_lock(), Some(Vec3::Y), "the lock moved off home");
        assert!(
            s.has_motion(),
            "the 30 degree tilt is a spring roll to settle"
        );
        settle(&mut s);
        assert_eq!(s.horizon_lock(), Some(Vec3::Y));
        assert!(upright_to(&s, Vec3::Y), "the view did not level back to Y");
        assert!(
            s.pose().forward().distance(Vec3::NEG_X) < 1e-4,
            "levelling moved the forward"
        );
    }

    /// A held alignment under an X lock snaps within the lock's grid, upright to X, so its
    /// release re-seats on X (not world Y) and the view stays upright to X.
    #[test]
    fn alignment_under_an_x_lock_is_upright_to_x() {
        let forward = Vec3::new(-0.2, 0.3, -1.0).normalize();
        let pose = CameraPose::from_eye_forward_up(Vec3::ZERO, forward, Vec3::X, 0.8, 0.1, 500.0);
        let mut s = SpaceFlight::from_pose(pose);
        assert!(s.set_horizon_lock(Some(Vec3::X)));
        s.set_align(Some((OrbitSnapMode::Axis, 0.05)));
        settle(&mut s);
        s.set_align(None);
        let Some(up) = s.horizon_lock() else {
            panic!("the lock survives the alignment");
        };
        assert!(up.distance(Vec3::X) < 1e-5, "released onto {up}, not X");
        assert!(
            upright_to(&s, Vec3::X),
            "the aligned view is not upright to X"
        );
        assert!(
            s.pose().forward().distance(Vec3::NEG_Z) < 1e-4,
            "snapped to the nearest grid axis view, forward {}",
            s.pose().forward()
        );
        let aligned = s.orientation;
        for _ in 0..60 {
            s.update_dynamics(1.0 / 60.0);
        }
        assert!(
            aligned.angle_between(s.orientation) < 1e-3,
            "the released view drifted"
        );
    }

    /// A NaN or infinite alignment time constant snaps in one step instead of poisoning the
    /// orientation (NaN) or holding the alignment forever without moving (+inf).
    #[test]
    fn non_finite_align_seconds_snaps_and_stays_finite() {
        for seconds in [f32::NAN, f32::INFINITY] {
            let mut s = SpaceFlight::default();
            s.set_align(Some((crate::camera_core::OrbitSnapMode::Axis, seconds)));
            s.update_dynamics(1.0 / 60.0);
            let q = s.pose().orientation;
            assert!(q.is_finite(), "{seconds}: orientation {q}");
            assert!(
                !s.update_dynamics(1.0 / 60.0),
                "{seconds}: the snap landed at once"
            );
        }
    }

    /// Frames until `update_dynamics` reports rest (bounded).
    fn frames_to_rest(s: &mut SpaceFlight, dt: f32) -> u32 {
        let mut frames = 0;
        while s.update_dynamics(dt) {
            frames += 1;
            assert!(frames < 10_000, "never came to rest");
        }
        frames
    }

    /// The held speed approaches `cruise * (1 - e^(-t/response))` with the same value at 30 and
    /// 240 fps, and settles on the cruise speed.
    #[test]
    fn held_speed_approaches_cruise_equally_at_any_frame_rate() {
        let mut slow = SpaceFlight::default();
        let mut fast = SpaceFlight::default();
        thrust(&mut slow, 1.0);
        thrust(&mut fast, 1.0);
        run(&mut slow, 1.0, 30);
        run(&mut fast, 1.0, 240);
        let want = slow.cruise_speed() * (1.0 - (-1.0 / slow.inertia.thrust_response).exp());
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
            let mut s = SpaceFlight::default();
            thrust(&mut s, 1.0);
            run(&mut s, 3.0, rate);
            thrust(&mut s, 0.0);
            let released_at = s.eye;
            let frames = frames_to_rest(&mut s, dt) + 1;
            let expected = (s.inertia.coast.time * rate as f32).ceil() as u32;
            assert!(
                frames.abs_diff(expected) <= 1,
                "{rate} fps: {frames} frames, expected {expected}"
            );
            assert_eq!(s.linear_velocity(), Vec3::ZERO);
            assert!(!s.is_coasting() && !s.has_motion());
            assert!(s.eye.distance(released_at) > 0.1, "the release must glide");
        }
    }

    /// A key during the coast continues from the coast speed: no jump.
    #[test]
    fn a_key_during_the_coast_has_no_velocity_jump() {
        let mut s = SpaceFlight::default();
        thrust(&mut s, 1.0);
        run(&mut s, 3.0, 60);
        thrust(&mut s, 0.0);
        run(&mut s, 10.0 * DT, 60);
        assert!(s.is_coasting());
        let before = s.linear_velocity().length();
        assert!(before > 0.0 && before < s.cruise_speed());
        thrust(&mut s, 1.0);
        s.update_dynamics(DT);
        let after = s.linear_velocity().length();
        assert!(!s.is_coasting(), "the key ended the coast");
        assert!(after >= before && after - before < 0.1 * s.cruise_speed());
    }

    /// At `SpeedScale(0)` a held key commands nothing: rest, no reported motion, no coast.
    #[test]
    fn speed_scale_zero_release_at_rest() {
        let mut s = SpaceFlight::default();
        s.apply_intent(CameraIntent::SpeedScale(0.0), vp());
        thrust(&mut s, 1.0);
        let eye = s.eye;
        assert!(!s.update_dynamics(DT), "a zero command is no motion");
        assert!(!s.has_motion());
        thrust(&mut s, 0.0);
        assert!(!s.update_dynamics(DT));
        assert!(!s.is_coasting());
        assert_eq!(s.eye, eye);
        s.apply_intent(CameraIntent::SpeedScale(1e30), vp());
        assert_eq!(s.speed_scale(), 1e6);
    }

    /// `release_inputs` of a flight with thrust, roll and steer held: at rest within the coast
    /// time, the roll and the steer stop without a coast.
    #[test]
    fn release_inputs_rests_within_the_coast_time() {
        let mut s = SpaceFlight::default();
        thrust(&mut s, 1.0);
        s.apply_intent(CameraIntent::Roll { d: 1.0 }, vp());
        s.apply_intent(
            CameraIntent::LookRate {
                yaw: 1.0,
                pitch: 0.0,
            },
            vp(),
        );
        run(&mut s, 3.0, 60);
        s.release_inputs();
        assert!(!s.look.is_active() && !s.roll.has_motion());
        let o = s.orientation;
        let frames = frames_to_rest(&mut s, DT);
        let limit = (s.inertia.coast.time / DT).ceil() as u32 + 1;
        assert!(frames <= limit, "{frames} frames, limit {limit}");
        assert_eq!(s.orientation, o, "the dropped roll / steer kept turning");
        assert_eq!(s.linear_velocity(), Vec3::ZERO);
    }

    /// A released free roll glides and ends exactly; a lock never has a roll coast: setting it
    /// stops the glide, and under it a held roll key drives only the spring.
    #[test]
    fn a_free_roll_coasts_and_a_lock_has_no_roll_coast() {
        let mut s = SpaceFlight::default();
        s.apply_intent(CameraIntent::Roll { d: 1.0 }, vp());
        run(&mut s, 0.5, 60);
        s.apply_intent(CameraIntent::Roll { d: 0.0 }, vp());
        s.update_dynamics(DT);
        assert!(s.is_coasting(), "the released roll glides");
        let frames = frames_to_rest(&mut s, DT);
        assert!(frames <= (s.inertia.coast.time / DT).ceil() as u32 + 1);
        assert!(!s.is_coasting());

        s.apply_intent(CameraIntent::Roll { d: 1.0 }, vp());
        run(&mut s, 0.5, 60);
        s.apply_intent(CameraIntent::Roll { d: 0.0 }, vp());
        s.update_dynamics(DT);
        assert!(s.is_coasting());
        assert!(s.set_horizon_lock(Some(Vec3::Y)));
        assert!(!s.is_coasting(), "the lock stopped the roll glide");
        assert_eq!(s.roll.velocity(), 0.0);

        s.apply_intent(CameraIntent::Roll { d: 1.0 }, vp());
        run(&mut s, 0.3, 60);
        assert!(!s.roll.has_motion(), "the roll key drives the spring only");
        s.apply_intent(CameraIntent::Roll { d: 0.0 }, vp());
        s.update_dynamics(DT);
        assert!(!s.roll.has_motion(), "no roll coast under a lock");
    }

    /// A held alignment stops the look and roll coasts (it owns the orientation).
    #[test]
    fn an_alignment_stops_the_look_and_roll_coasts() {
        let mut s = SpaceFlight::default();
        s.apply_intent(CameraIntent::Roll { d: 1.0 }, vp());
        run(&mut s, 0.5, 60);
        s.apply_intent(CameraIntent::Roll { d: 0.0 }, vp());
        s.update_dynamics(DT);
        s.apply_intent(
            CameraIntent::LookInertia {
                yaw_rate: 1.0,
                pitch_rate: 0.5,
            },
            vp(),
        );
        assert!(s.look.is_active() && s.roll.is_coasting());
        s.set_align(Some((OrbitSnapMode::Axis, 0.5)));
        s.update_dynamics(DT);
        assert!(!s.look.is_active() && !s.roll.has_motion());
    }

    /// `Stop` ends every coast; `View` stops the flight and the coasts.
    #[test]
    fn stop_and_view_end_the_glides() {
        let mut s = SpaceFlight::default();
        thrust(&mut s, 1.0);
        run(&mut s, 3.0, 60);
        thrust(&mut s, 0.0);
        s.apply_intent(
            CameraIntent::LookInertia {
                yaw_rate: 1.0,
                pitch_rate: 0.0,
            },
            vp(),
        );
        s.update_dynamics(DT);
        assert!(s.is_coasting());
        s.apply_intent(CameraIntent::Stop, vp());
        assert!(!s.is_coasting() && !s.has_motion());

        thrust(&mut s, 1.0);
        run(&mut s, 1.0, 60);
        let held = s.linear_velocity();
        s.apply_intent(CameraIntent::Stop, vp());
        assert_eq!(s.linear_velocity(), held, "a held thrust is not a coast");
        let Some(view) = crate::camera_core::ViewTarget::new(0, 0, 1) else {
            panic!("(0, 0, 1) is a view target");
        };
        s.apply_intent(CameraIntent::View(view), vp());
        assert_eq!(s.linear_velocity(), Vec3::ZERO, "View stops the flight");
    }
}
