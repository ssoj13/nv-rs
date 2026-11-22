//! Fly input of the navigator: held thrust / roll axes from [`FlyKeys`], mouse look
//! (`Relative` look deltas or `Deflection` steer rates), the wheel flight-speed multiplier and the
//! steer anchor.
//!
//! What: [`FlyState`] (engaged, held by a fly drag, steer anchor, wheel speed) and pure helpers
//! that turn a [`RawInput`] into the fly intents.
//!
//! Why: one implementation of the fly rules that used to live in `cam-controls-egui::fly`,
//! sl-viewport `gather_fly_intents` and nodes-editor `step_camera` (RMB hold-to-fly, wheel =
//! speed), so FPS, spaceflight and RMB flight read the keys the same way everywhere.
//!
//! How: the held axes AND the speed scale are emitted EVERY engaged frame (zero / current when
//! nothing is held or the typing guard is down), so a release always reaches the rig and a
//! backend rebuilt mid-flight (`CameraController::set_mode`, `release_inputs`) gets the current
//! speed back on the next frame; a disengage emits the zero set once. A fly DRAG anchors the
//! deflection steer at its press point, so a press never turns the camera. Relative mouse look
//! also feeds a [`ReleaseVelocity`] every steering frame, zero deltas included (the pause): the
//! navigator turns it into `CameraIntent::LookInertia` when a fly drag ends, and here in the
//! persistent mode after `LOOK_PAUSE_S` of still steering frames (one still frame is often
//! just a display faster than the mouse polls).
//!
//! Used by: `super::navigator`.

use glam::Vec2;

use super::bindings::FlyKeys;
use super::navigator::NavTuning;
use super::raw::{Key, KeySet, RawInput};
use crate::camera_core::coast::{RELEASE_WINDOW_S, ReleaseVelocity};
use crate::camera_core::intent::CameraIntent;
use crate::camera_core::settings::MouseSteer;

/// Largest steering dead zone (fraction of the half viewport): the response must keep a range.
const DEADZONE_MAX: f32 = 0.95;
/// Smallest half-viewport extent (viewport units) the deflection is normalised against.
const HALF_VIEWPORT_MIN: f32 = 1.0;
/// Flight-speed factor per wheel notch while flying (`RMB + Mouse Wheel`, UE5/Blender standard).
const FLY_WHEEL_STEP: f32 = 1.25;
/// Lower bound of the wheel flight-speed multiplier.
const FLY_SPEED_MIN: f32 = 0.1;
/// Upper bound of the wheel flight-speed multiplier.
const FLY_SPEED_MAX: f32 = 64.0;

/// Progressive Shift-flight acceleration ramp for navigating large city scenes.
///
/// Stays at flat `1.0x` base Shift speed for the first `2.0 s`, then smoothly doubles every
/// `5.0 s` (`2^((t - 2.0) / 5.0)`), clamped to `[1.0, 64.0]`.
#[inline]
pub fn shift_fly_ramp(shift_fly_secs: f32) -> f32 {
    if !shift_fly_secs.is_finite() || shift_fly_secs <= 2.0 {
        1.0
    } else {
        2.0_f32.powf((shift_fly_secs - 2.0) / 5.0).clamp(1.0, 64.0)
    }
}

/// Cesium/UE5-style altitude-aware flight speed multiplier.
///
/// Keeps close-up ground/bolt inspection (`altitude <= 8 m`) at exact `1.0x`, scaling smoothly
/// up to `25.0x` at high aerial altitude (`>= 200 m`) so orbital/map sweeps feel effortless.
#[inline]
pub fn altitude_speed_multiplier(altitude_m: f32) -> f32 {
    if !altitude_m.is_finite() {
        return 1.0;
    }
    (altitude_m.max(2.0) / 8.0).clamp(1.0, 25.0)
}
/// Still steering time (seconds) after which the persistent mode's mouse look counts as
/// stopped and coasts. Longer than a slow mouse's polling gap (125 Hz = 8 ms) seen from a fast
/// display, shorter than the release window so the measured rate is still the sweep's.
pub(super) const LOOK_PAUSE_S: f32 = 0.03;
const _: () = assert!(LOOK_PAUSE_S < RELEASE_WINDOW_S);

/// Emit `LookInertia` for a non-zero, finite look velocity `v` (rad/s).
fn push_look_rate(v: Vec2, out: &mut Vec<CameraIntent>) {
    if v != Vec2::ZERO && v.is_finite() {
        out.push(CameraIntent::LookInertia {
            yaw_rate: v.x,
            pitch_rate: v.y,
        });
    }
}

/// Fly state carried between frames.
#[derive(Clone, Copy, Debug)]
pub(super) struct FlyState {
    /// Fly input was live last frame (a disengage edge emits the zero set).
    pub active: bool,
    /// A `DragRole::Fly` drag holds the flight (mouse look regardless of the typing guard; the
    /// steer anchor is the press point).
    pub held: bool,
    /// Steering neutral point (viewport units) for `MouseSteer::Deflection`.
    pub anchor: Vec2,
    /// The anchor stays where it was put (a fly drag's press point, or the cursor where a fly
    /// drag ended inside a persistent fly mode) instead of following the viewport centre.
    pub pinned: bool,
    /// Wheel flight-speed multiplier, folded into `CameraIntent::SpeedScale`.
    pub speed: f32,
    /// Continuous seconds spent flying with Shift held AND an active WASD/QE flight key.
    pub shift_fly_secs: f32,
    /// The speed scale of the previous engaged frame: a change is real input (repaint), the
    /// intent itself is sent every frame.
    pub last_scale: Option<f32>,
    /// Escape (or focus loss) suspended the persistent fly capture until the pointer re-enters
    /// or clicks the view.
    pub suspended: bool,
    /// Relative mouse-look steps (radians: yaw, pitch) of the steering frames: the release
    /// velocity of the turn.
    pub look: ReleaseVelocity<Vec2>,
    /// A look delta moved the view since the persistent mode last emitted `LookInertia` (it
    /// does after `LOOK_PAUSE_S` of still steering frames).
    pub look_moving: bool,
    /// Seconds of still steering frames since the last look delta.
    pub look_still: f32,
    /// The mouse-look release velocity measured on the last frame with a look delta: the
    /// persistent mode's coast speed (the still frames of the pause would dilute it).
    pub look_rate: Vec2,
}

impl Default for FlyState {
    fn default() -> Self {
        Self {
            active: false,
            held: false,
            anchor: Vec2::ZERO,
            pinned: false,
            speed: 1.0,
            shift_fly_secs: 0.0,
            last_scale: None,
            suspended: false,
            look: ReleaseVelocity::new(),
            look_moving: false,
            look_still: 0.0,
            look_rate: Vec2::ZERO,
        }
    }
}

impl FlyState {
    /// Forget the mouse-look velocity (a new engagement, a cancelled drag, focus loss).
    pub fn clear_look(&mut self) {
        self.look.clear();
        self.look_moving = false;
        self.look_still = 0.0;
        self.look_rate = Vec2::ZERO;
    }

    /// Emit `LookInertia` with the current mouse-look velocity, unless it is zero (the user
    /// paused) or not finite: whether it coasts is the rig's decision.
    pub fn push_look_inertia(&self, out: &mut Vec<CameraIntent>) {
        push_look_rate(self.look.velocity(), out);
    }

    /// Place the steering anchor: a fly DRAG anchors at its press point (never the screen
    /// centre, so pressing without moving turns nothing); a persistent fly mode anchors at the
    /// viewport centre, or at the cursor for a relative origin (`steer_center_screen` off).
    pub fn anchor_at(&mut self, raw: &RawInput, tuning: &NavTuning) {
        let centre = raw.viewport * 0.5;
        self.clear_look();
        self.pinned = self.held;
        self.anchor = if self.held || !tuning.steer_center_screen {
            raw.pointer.unwrap_or(centre)
        } else {
            centre
        };
    }

    /// A fly drag ended inside a persistent fly mode: keep steering from where the cursor is
    /// (never jump the anchor to the centre, which would turn at once).
    pub fn pin_at_cursor(&mut self, raw: &RawInput) {
        self.anchor = raw.pointer.unwrap_or(self.anchor);
        self.pinned = true;
    }

    /// Scale the flight speed by this frame's wheel (`Wheel::notches`: lines plus smooth scroll
    /// through the one shared notch size). The speed range bounds hostile values; no per-frame
    /// cap drops input.
    pub fn wheel_speed(&mut self, raw: &RawInput) {
        let notches = raw.wheel.notches();
        if notches != 0.0 {
            self.speed =
                (self.speed * FLY_WHEEL_STEP.powf(notches)).clamp(FLY_SPEED_MIN, FLY_SPEED_MAX);
        }
    }

    /// Keep the anchor where the tuning puts it: on the (possibly resized) centre for an
    /// unpinned persistent fly mode with a centred origin, or drifting toward the cursor with a
    /// steer decay (relative origins and pinned anchors, which includes every fly drag).
    fn track_anchor(&mut self, raw: &RawInput, tuning: &NavTuning) {
        if !self.pinned && tuning.steer_center_screen {
            self.anchor = raw.viewport * 0.5;
        } else if let Some(secs) = tuning.steer_decay_secs
            && let Some(p) = raw.pointer
        {
            let blend = if secs.is_finite() && secs > 0.0 {
                1.0 - (-raw.dt / secs).exp()
            } else {
                1.0
            };
            self.anchor += (p - self.anchor) * blend;
        }
    }

    /// Push one engaged frame's fly intents and return whether it carried real input (a turn,
    /// pointer motion while steering, a held axis or a speed-scale change), the navigator's
    /// repaint signal. The hold / zero intents go out regardless. Keys need the typing guard
    /// (`raw.keys_allowed`); mouse look needs `steering` (decided by the navigator: a held fly
    /// drag, or the persistent capture over the view).
    pub fn push_frame(
        &mut self,
        raw: &RawInput,
        keys: &FlyKeys,
        tuning: &NavTuning,
        steering: bool,
        out: &mut Vec<CameraIntent>,
    ) -> bool {
        let live = raw.keys_allowed;
        let sx = if tuning.invert_look_x { -1.0 } else { 1.0 };
        let sy = if tuning.invert_look_y { -1.0 } else { 1.0 };
        self.track_anchor(raw, tuning);
        let mut moved = false;
        let (mut yaw, mut pitch) = (0.0, 0.0);
        match tuning.steer {
            MouseSteer::Relative => {
                let d = raw.pointer_delta;
                if steering {
                    let s = tuning.look_rad_per_unit;
                    let step = Vec2::new(d.x * s * sx, -d.y * s * sy);
                    // Zero steps are pushed too: they are the pause the estimator needs.
                    self.look.push(step, raw.dt);
                    if d != Vec2::ZERO {
                        out.push(CameraIntent::Look {
                            dyaw: step.x,
                            dpitch: step.y,
                        });
                        moved = true;
                        self.look_moving = true;
                        self.look_still = 0.0;
                        self.look_rate = self.look.velocity();
                    } else if self.look_moving && !self.held {
                        // Persistent mode: a frame without a pointer event is not a stop (a
                        // display faster than the mouse polls has them mid-sweep, and a coast
                        // started there adds a turn the next `Look` never takes back). Only a
                        // pause of `LOOK_PAUSE_S` is. A held fly drag coasts when it ends.
                        self.look_still += raw.dt;
                        if self.look_still >= LOOK_PAUSE_S {
                            self.look_moving = false;
                            push_look_rate(self.look_rate, out);
                        }
                    }
                } else {
                    self.clear_look();
                }
            }
            MouseSteer::Deflection => {
                if steering && let Some(p) = raw.pointer {
                    let v = deflection(p, self.anchor, raw.viewport, tuning.steer_deadzone);
                    yaw = v.x * sx;
                    pitch = -v.y * sy;
                    moved = v != Vec2::ZERO || raw.pointer_delta != Vec2::ZERO;
                }
            }
        }
        // Always refreshed: a recentred cursor (or Relative mode) stops a held turn.
        out.push(CameraIntent::LookRate { yaw, pitch });

        let keys_held = if live { raw.keys_down } else { KeySet::EMPTY };
        let (forward, right, up, roll) = axes(keys_held, keys);
        let thrust_moving = forward != 0.0 || right != 0.0 || up != 0.0 || roll != 0.0;
        moved |= thrust_moving;
        out.push(CameraIntent::Thrust { forward, right, up });
        out.push(CameraIntent::Roll { d: roll });
        // Progressive Shift-flight ramp: flat 1x for <= 2s, then doubles every 5s up to 64x;
        // resets immediately when Shift is released or movement stops.
        let m = raw.mods;
        let shift_held = live && m.shift;
        if shift_held && thrust_moving {
            let step_dt = if raw.dt.is_finite() && raw.dt > 0.0 {
                raw.dt
            } else {
                0.0
            };
            self.shift_fly_secs = (self.shift_fly_secs + step_dt).min(60.0);
        } else {
            self.shift_fly_secs = 0.0;
        }
        let ramp = if shift_held {
            shift_fly_ramp(self.shift_fly_secs)
        } else {
            1.0
        };
        let scale = tuning.speed_scale(shift_held, live && m.alt) * ramp * self.speed;
        moved |= self.last_scale != Some(scale);
        self.last_scale = Some(scale);
        out.push(CameraIntent::SpeedScale(scale));
        moved
    }
}

/// The zero set a disengaged flight leaves behind: no thrust, roll or steer, speed scale 1.
pub(super) fn push_release(out: &mut Vec<CameraIntent>) {
    out.push(CameraIntent::Thrust {
        forward: 0.0,
        right: 0.0,
        up: 0.0,
    });
    out.push(CameraIntent::Roll { d: 0.0 });
    out.push(CameraIntent::LookRate {
        yaw: 0.0,
        pitch: 0.0,
    });
    out.push(CameraIntent::SpeedScale(1.0));
}

/// `1.0` when `key` is held, else `0.0`.
fn held(keys: KeySet, key: Key) -> f32 {
    if keys.contains(key) { 1.0 } else { 0.0 }
}

/// Held `(forward, right, up, roll)` axes in `[-1, 1]` from `keys`.
pub(super) fn axes(keys: KeySet, fly: &FlyKeys) -> (f32, f32, f32, f32) {
    let up_alt = fly.up_alt.map_or(0.0, |k| held(keys, k));
    (
        held(keys, fly.forward) - held(keys, fly.back),
        held(keys, fly.right) - held(keys, fly.left),
        held(keys, fly.up).max(up_alt) - held(keys, fly.down),
        held(keys, fly.roll_right) - held(keys, fly.roll_left),
    )
}

/// Cursor offset from `anchor` over the half viewport, per axis clamped to `[-1, 1]`, with the
/// dead zone cut out and the rest rescaled so the response starts at 0 and reaches 1 at full
/// deflection. `+x` = cursor right, `+y` = cursor down (screen axes).
pub(super) fn deflection(pointer: Vec2, anchor: Vec2, viewport: Vec2, deadzone: f32) -> Vec2 {
    let half = (viewport * 0.5).max(Vec2::splat(HALF_VIEWPORT_MIN));
    let dz = if deadzone.is_finite() {
        deadzone.clamp(0.0, DEADZONE_MAX)
    } else {
        0.0
    };
    let shape = |raw: f32| {
        let v = raw.clamp(-1.0, 1.0);
        let a = v.abs();
        if a <= dz {
            0.0
        } else {
            v.signum() * ((a - dz) / (1.0 - dz))
        }
    };
    let d = (pointer - anchor) / half;
    Vec2::new(shape(d.x), shape(d.y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera_core::input::raw::{POINTS_PER_NOTCH, Wheel};

    #[test]
    fn deflection_has_a_dead_zone_and_reaches_one_at_the_edge() {
        let vp = Vec2::new(200.0, 100.0);
        let c = vp * 0.5;
        assert_eq!(deflection(c + Vec2::new(4.0, 0.0), c, vp, 0.05), Vec2::ZERO);
        let edge = deflection(c + Vec2::new(100.0, -50.0), c, vp, 0.05);
        assert!((edge.x - 1.0).abs() < 1e-6 && (edge.y + 1.0).abs() < 1e-6);
        let beyond = deflection(c + Vec2::new(1e6, 0.0), c, vp, 0.05);
        assert_eq!(beyond.x, 1.0);
        // A degenerate viewport and a NaN dead zone stay finite.
        assert!(deflection(Vec2::ONE, Vec2::ZERO, Vec2::ZERO, f32::NAN).is_finite());
    }

    #[test]
    fn axes_combine_opposite_keys_and_the_alt_up_key() {
        let keys = KeySet::EMPTY
            .with(Key::W)
            .with(Key::S)
            .with(Key::D)
            .with(Key::Space)
            .with(Key::Q);
        assert_eq!(axes(keys, &FlyKeys::WASD), (0.0, 1.0, 1.0, -1.0));
        let no_alt = FlyKeys {
            up_alt: None,
            ..FlyKeys::WASD
        };
        assert_eq!(axes(keys, &no_alt).2, 0.0);
    }

    #[test]
    fn wheel_speed_steps_by_notch_equivalents_and_clamps() {
        let wheel = |lines: f32, points: f32| RawInput {
            wheel: Wheel {
                lines: Vec2::new(0.0, lines),
                points: Vec2::new(0.0, points),
            },
            ..RawInput::default()
        };
        let mut a = FlyState::default();
        a.wheel_speed(&wheel(1.0, 0.0));
        let mut b = FlyState::default();
        b.wheel_speed(&wheel(0.0, POINTS_PER_NOTCH));
        assert!((a.speed - FLY_WHEEL_STEP).abs() < 1e-6);
        assert!((a.speed - b.speed).abs() < 1e-6, "lines and points agree");
        for _ in 0..1000 {
            a.wheel_speed(&wheel(1.0, 0.0));
        }
        assert_eq!(a.speed, FLY_SPEED_MAX);
        a.wheel_speed(&wheel(-1e30, 0.0));
        assert_eq!(a.speed, FLY_SPEED_MIN);
    }

    #[test]
    fn a_fly_drag_anchors_at_the_press_point() {
        let tuning = NavTuning::default();
        assert!(tuning.steer_center_screen);
        let raw = RawInput {
            pointer: Some(Vec2::new(100.0, 100.0)),
            viewport: Vec2::new(800.0, 600.0),
            ..RawInput::default()
        };
        let mut drag = FlyState {
            held: true,
            ..FlyState::default()
        };
        drag.anchor_at(&raw, &tuning);
        assert_eq!(drag.anchor, Vec2::new(100.0, 100.0));
        drag.track_anchor(&raw, &tuning);
        assert_eq!(drag.anchor, Vec2::new(100.0, 100.0), "no recentring");
        let mut mode = FlyState::default();
        mode.anchor_at(&raw, &tuning);
        assert_eq!(mode.anchor, Vec2::new(400.0, 300.0));
    }

    #[test]
    fn shift_fly_ramp_doubles_every_5s_after_2s_and_resets_on_release() {
        assert!((shift_fly_ramp(0.0) - 1.0).abs() < 1e-6);
        assert!((shift_fly_ramp(1.5) - 1.0).abs() < 1e-6);
        assert!((shift_fly_ramp(2.0) - 1.0).abs() < 1e-6);
        assert!((shift_fly_ramp(7.0) - 2.0).abs() < 1e-5);
        assert!((shift_fly_ramp(12.0) - 4.0).abs() < 1e-5);
        assert!((shift_fly_ramp(32.0) - 64.0).abs() < 1e-4);
        assert_eq!(shift_fly_ramp(100.0), 64.0);
        assert_eq!(shift_fly_ramp(f32::NAN), 1.0);

        // Stateful hold -> ramp -> release reset check through FlyState::push_frame.
        let tuning = NavTuning::default();
        let keys = FlyKeys::WASD;
        let mut st = FlyState::default();
        let mut out = Vec::new();
        let moving_shift = RawInput {
            dt: 1.0,
            keys_allowed: true,
            keys_down: KeySet::EMPTY.with(Key::W),
            mods: crate::camera_core::input::Mods::SHIFT,
            viewport: Vec2::new(800.0, 600.0),
            ..RawInput::default()
        };
        for _ in 0..7 {
            out.clear();
            st.push_frame(&moving_shift, &keys, &tuning, true, &mut out);
        }
        assert!((st.shift_fly_secs - 7.0).abs() < 1e-5);
        let expected_7s = tuning.speed_scale(true, false) * 2.0;
        let last_7s = out.iter().rev().find_map(|i| match *i {
            CameraIntent::SpeedScale(s) => Some(s),
            _ => None,
        });
        assert!((last_7s.expect("speed scale") - expected_7s).abs() < 1e-4);

        // Releasing Shift resets `shift_fly_secs` immediately to 0.
        let released_shift = RawInput {
            mods: crate::camera_core::input::Mods::NONE,
            ..moving_shift
        };
        out.clear();
        st.push_frame(&released_shift, &keys, &tuning, true, &mut out);
        assert_eq!(st.shift_fly_secs, 0.0);

        // Stopping movement while Shift is still held also resets `shift_fly_secs` to 0.
        st.shift_fly_secs = 9.0;
        let stopped_move = RawInput {
            keys_down: KeySet::EMPTY,
            ..moving_shift
        };
        out.clear();
        st.push_frame(&stopped_move, &keys, &tuning, true, &mut out);
        assert_eq!(st.shift_fly_secs, 0.0);
    }

    #[test]
    fn altitude_speed_multiplier_scales_smoothly_from_ground_to_aerial() {
        assert_eq!(altitude_speed_multiplier(0.0), 1.0);
        assert_eq!(altitude_speed_multiplier(2.0), 1.0);
        assert_eq!(altitude_speed_multiplier(8.0), 1.0);
        assert!((altitude_speed_multiplier(16.0) - 2.0).abs() < 1e-5);
        assert!((altitude_speed_multiplier(80.0) - 10.0).abs() < 1e-5);
        assert_eq!(altitude_speed_multiplier(200.0), 25.0);
        assert_eq!(altitude_speed_multiplier(500.0), 25.0);
        assert_eq!(altitude_speed_multiplier(f32::NAN), 1.0);
    }

    #[test]
    fn rmb_wheel_scales_fly_speed_by_1_25_clamped_to_0_1_through_64() {
        let wheel = |lines: f32| RawInput {
            wheel: Wheel {
                lines: Vec2::new(0.0, lines),
                points: Vec2::ZERO,
            },
            ..RawInput::default()
        };
        let mut st = FlyState::default();
        st.wheel_speed(&wheel(1.0));
        assert!((st.speed - 1.25).abs() < 1e-6);
        st.wheel_speed(&wheel(1.0));
        assert!((st.speed - 1.5625).abs() < 1e-5);
        st.wheel_speed(&wheel(-100.0));
        assert!((st.speed - 0.1).abs() < 1e-6);
        st.wheel_speed(&wheel(100.0));
        assert!((st.speed - 64.0).abs() < 1e-6);
    }
}
