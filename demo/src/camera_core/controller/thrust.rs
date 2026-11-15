//! [`Drive`]: the held-thrust model of the fly rigs (`FpsFly`, `SpaceFlight`), one value channel.
//!
//! What: a held command `v_cmd` (a world velocity for the linear thrust, a rad/s rate for the
//! free roll) that the velocity approaches with time constant `thrust_response`, and a [`Coast`]
//! that takes over when the command goes away.
//!
//! Why: the old model integrated an acceleration against a linear damping and braked with a
//! threshold. Here the held speed is the commanded cruise speed (readable, frame-rate
//! independent) and the release glides to an EXACT stop at the coast time `T` like every other
//! coast of the crate (`crate::camera_core::coast`).
//!
//! How: `step` advances one frame in closed form. With a command the exact first-order
//! approach is used (`v = v_cmd + (v - v_cmd) e^(-dt/tau)`, displacement
//! `v_cmd dt + (v - v_cmd) tau (1 - e^(-dt/tau))`), so 30 fps and 240 fps give the same speed and
//! travel. On the held -> released edge the current velocity seeds the coast and the held
//! velocity is cleared; a command arriving during a coast stops it and continues from
//! `coast.velocity()`, so there is no speed jump.
//!
//! Uses: `crate::camera_core::coast`. Used by: `super::fps`, `super::space` (crate-private).

use crate::camera_core::coast::{Coast, CoastSettings, CoastValue};

/// Largest speed scale a fly rig accepts: keeps `cruise * scale * thrust` far from f32 overflow.
pub(super) const SPEED_SCALE_MAX: f32 = 1e6;

/// Unit of the look coasts' `min_speed` (rad/s per unit): a look coast speed is an angular rate.
pub(super) const LOOK_COAST_UNIT: f32 = 1.0;

/// A host `SpeedScale` limited to `[0, SPEED_SCALE_MAX]` (a NaN becomes 0). The scale is a
/// multiplier of the cruise speed: a negative one would reverse the thrust.
pub(super) fn clamp_speed_scale(scale: f32) -> f32 {
    crate::camera_core::num::clamp(scale, 0.0, SPEED_SCALE_MAX)
}

/// A host tuning value (cruise speed, roll rate) made usable: negative and NaN become 0, an
/// infinite one `f32::MAX`.
pub(super) fn non_negative(value: f32) -> f32 {
    crate::camera_core::num::clamp(value, 0.0, f32::MAX)
}

/// `value` as a drive command: `Some` only when it is finite and non-zero. A zero command (no
/// keys, or a zero speed scale) is a release, so a rig at `SpeedScale(0)` never counts as moving.
pub(super) fn live<V: CoastValue>(value: V) -> Option<V> {
    (value.is_finite() && value != V::ZERO).then_some(value)
}

/// One value channel of a held thrust: the ramped held velocity and the coast after the release.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Drive<V> {
    /// Held velocity (non-zero only while a command is applied).
    velocity: V,
    /// The release coast (idle while held).
    coast: Coast<V>,
}

impl<V: CoastValue> Default for Drive<V> {
    fn default() -> Self {
        Self {
            velocity: V::ZERO,
            coast: Coast::default(),
        }
    }
}

impl<V: CoastValue> Drive<V> {
    /// Advance one frame and return the displacement over `dt`.
    ///
    /// `command` is the held target velocity (`None` or a zero / non-finite value = released).
    /// `cruise_unit` is the speed the coast's `min_speed` fraction refers to; `response` is the
    /// approach time constant in seconds (`<= 0` or non-finite = the velocity jumps to the
    /// command). `dt` must be usable (`> 0`, finite); the callers check it.
    pub(super) fn step(
        &mut self,
        command: Option<V>,
        cruise_unit: f32,
        response: f32,
        settings: &CoastSettings,
        dt: f32,
    ) -> V {
        let Some(target) = command.and_then(live) else {
            if self.velocity != V::ZERO {
                // The held -> released edge: the velocity becomes the coast's start speed. A
                // coast that does not start (time 0, below `min_speed`) leaves the rig at rest.
                self.coast.start(self.velocity, cruise_unit, settings);
                self.velocity = V::ZERO;
            }
            return self.coast.step(dt);
        };
        if self.coast.is_active() {
            // A key during the coast continues from the coast's current speed (no jump).
            self.velocity = self.coast.velocity();
            self.coast.stop();
        }
        let (next, shift) = if response > 0.0 && response.is_finite() {
            let decay = (-dt / response).exp();
            let gap = self.velocity - target;
            (
                target + gap * decay,
                gap * (response * -(-dt / response).exp_m1()),
            )
        } else {
            (target, V::ZERO)
        };
        let travel = target * dt + shift;
        if next.is_finite() && travel.is_finite() {
            self.velocity = next;
            travel
        } else {
            self.velocity = V::ZERO;
            V::ZERO
        }
    }

    /// The current velocity: the held one, or the running coast's.
    pub(super) fn velocity(&self) -> V {
        if self.coast.is_active() {
            self.coast.velocity()
        } else {
            self.velocity
        }
    }

    /// Stop everything at once (held velocity and coast).
    pub(super) fn stop(&mut self) {
        *self = Self::default();
    }

    /// Cancel the running coast, and the one the next `step` would start: a velocity whose
    /// `command` is already released (a zero thrust set applied earlier in the frame, then
    /// `Stop`) is dropped too. A velocity still commanded keeps ramping with its command.
    pub(super) fn stop_coast(&mut self, command: Option<V>) {
        self.coast.stop();
        if command.and_then(live).is_none() {
            self.velocity = V::ZERO;
        }
    }

    /// True while the release coast runs.
    pub(super) fn is_coasting(&self) -> bool {
        self.coast.is_active()
    }

    /// True while the channel has any velocity: held, or coasting (exact, no threshold).
    pub(super) fn has_motion(&self) -> bool {
        self.velocity != V::ZERO || self.coast.is_active()
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::*;

    const CRUISE: f32 = 10.0;
    const TAU: f32 = 0.25;

    fn settings() -> CoastSettings {
        CoastSettings::default()
    }

    /// Hold `command` for `seconds` at `fps`; returns the travelled distance.
    fn hold(d: &mut Drive<Vec3>, command: Vec3, seconds: f32, fps: u32) -> Vec3 {
        let dt = 1.0 / fps as f32;
        let mut total = Vec3::ZERO;
        for _ in 0..(seconds * fps as f32).round() as u32 {
            total += d.step(Some(command), CRUISE, TAU, &settings(), dt);
        }
        total
    }

    /// The held speed approaches the cruise speed with the same value at 30 and 240 fps, and
    /// equals the closed form `cruise (1 - e^(-t/tau))`.
    #[test]
    fn held_speed_approaches_cruise_equally_at_any_frame_rate() {
        let command = Vec3::new(0.0, 0.0, -CRUISE);
        let (mut slow, mut fast) = (Drive::default(), Drive::default());
        let travel_slow = hold(&mut slow, command, 1.0, 30);
        let travel_fast = hold(&mut fast, command, 1.0, 240);
        let expected = CRUISE * (1.0 - (-1.0 / TAU).exp());
        assert!((slow.velocity().length() - expected).abs() < 1e-3);
        assert!((fast.velocity().length() - expected).abs() < 1e-3);
        assert!((travel_slow - travel_fast).length() < 1e-3);
        // Distance of the closed form: cruise * (t - tau (1 - e^(-t/tau))).
        let distance = CRUISE * (1.0 - TAU * (1.0 - (-1.0 / TAU).exp()));
        assert!((travel_fast.length() - distance).abs() < 1e-3);
        // Long enough, the held speed IS the cruise speed.
        hold(&mut fast, command, 4.0, 60);
        assert!((fast.velocity().length() - CRUISE).abs() < 1e-3);
    }

    /// Without a response time the velocity is the command at once.
    #[test]
    fn zero_response_follows_the_command_at_once() {
        let mut d = Drive::<f32>::default();
        for response in [0.0, -1.0, f32::NAN] {
            d.stop();
            let travel = d.step(Some(3.0), CRUISE, response, &settings(), 0.5);
            assert_eq!(d.velocity(), 3.0, "response {response}");
            assert!((travel - 1.5).abs() < 1e-6);
        }
    }

    /// A release coasts and ends EXACTLY at the coast time: the velocity is zero from the
    /// frame that reaches `T`, never a threshold.
    #[test]
    fn release_stops_exactly_at_the_coast_time() {
        for fps in [30_u32, 60, 144, 240] {
            let dt = 1.0 / fps as f32;
            let mut d = Drive::<Vec3>::default();
            hold(&mut d, Vec3::X * CRUISE, 3.0, fps);
            let v0 = d.velocity().x;
            let mut frames = 0_u32;
            let mut travel = Vec3::ZERO;
            loop {
                travel += d.step(None, CRUISE, TAU, &settings(), dt);
                frames += 1;
                if !d.has_motion() {
                    break;
                }
                assert!(frames < 10_000, "never stopped at {fps} fps");
            }
            let expected = (settings().time * fps as f32).ceil() as u32;
            assert!(
                frames.abs_diff(expected) <= 1,
                "{fps} fps: stopped after {frames} frames, expected {expected}"
            );
            assert_eq!(d.velocity(), Vec3::ZERO);
            assert!(!d.is_coasting());
            assert!(
                travel.x > 0.0 && travel.x < v0 * settings().time,
                "{travel}"
            );
        }
    }

    /// A key during the coast continues from the coast's speed: no jump to the command and no
    /// restart from zero.
    #[test]
    fn a_key_during_the_coast_has_no_velocity_jump() {
        let dt = 1.0 / 60.0;
        let command = Vec3::Y * CRUISE;
        let mut d = Drive::<Vec3>::default();
        hold(&mut d, command, 3.0, 60);
        for _ in 0..10 {
            d.step(None, CRUISE, TAU, &settings(), dt);
        }
        assert!(d.is_coasting());
        let coasting = d.velocity().y;
        assert!(coasting > 0.0 && coasting < CRUISE);
        d.step(Some(command), CRUISE, TAU, &settings(), dt);
        assert!(!d.is_coasting(), "the key ended the coast");
        let after = d.velocity().y;
        assert!(after >= coasting, "{coasting} -> {after}: slowed down");
        assert!(
            after - coasting < 0.1 * CRUISE,
            "{coasting} -> {after}: jumped"
        );
    }

    /// A zero command from rest (a zero speed scale) starts nothing: no coast, no motion.
    #[test]
    fn a_zero_command_at_rest_stays_at_rest() {
        let mut d = Drive::<Vec3>::default();
        for command in [None, Some(Vec3::ZERO), Some(Vec3::NAN)] {
            let travel = d.step(command, CRUISE, TAU, &settings(), 1.0 / 60.0);
            assert_eq!(travel, Vec3::ZERO);
            assert!(!d.has_motion());
        }
    }

    /// With the coast disabled (Sharp feel) a release stops dead on the release frame.
    #[test]
    fn a_disabled_coast_stops_dead() {
        let mut d = Drive::<f32>::default();
        d.step(Some(2.0), CRUISE, 0.0, &CoastSettings::NONE, 0.1);
        assert!(d.has_motion());
        let travel = d.step(None, CRUISE, TAU, &CoastSettings::NONE, 0.1);
        assert_eq!(travel, 0.0);
        assert!(!d.has_motion());
    }

    /// `stop_coast` leaves a commanded velocity alone, drops a released one before its coast
    /// starts and cancels a running coast; `stop` clears everything.
    #[test]
    fn stop_variants() {
        let mut d = Drive::<f32>::default();
        d.step(Some(2.0), CRUISE, TAU, &settings(), 0.1);
        d.stop_coast(Some(2.0));
        assert!(d.has_motion());
        d.stop_coast(None);
        assert!(
            !d.has_motion(),
            "a released velocity must not start a coast after Stop"
        );
        assert_eq!(d.step(None, CRUISE, TAU, &settings(), 0.01), 0.0);
        d.step(Some(2.0), CRUISE, TAU, &settings(), 0.1);
        d.step(None, CRUISE, TAU, &settings(), 0.01);
        assert!(d.is_coasting());
        d.stop_coast(None);
        assert!(!d.has_motion());
        d.step(Some(2.0), CRUISE, TAU, &settings(), 0.1);
        d.stop();
        assert!(!d.has_motion());
    }

    /// The speed scale is limited to `[0, SPEED_SCALE_MAX]` and a NaN is a zero scale.
    #[test]
    fn speed_scale_is_clamped() {
        assert_eq!(clamp_speed_scale(4.0), 4.0);
        assert_eq!(clamp_speed_scale(-3.0), 0.0);
        assert_eq!(clamp_speed_scale(1e30), SPEED_SCALE_MAX);
        assert_eq!(clamp_speed_scale(f32::NAN), 0.0);
    }
}
