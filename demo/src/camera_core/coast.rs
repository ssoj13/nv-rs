//! Finite-time coast: the one release / thrust-off glide of every camera backend, and the
//! estimator that measures the release velocity from a gesture.
//!
//! What: [`Coast`] glides a velocity (`f32`, `Vec2`, `Vec3`) to EXACT rest at a fixed time `T`
//! after it started, with a closed-form displacement per frame; [`OrbitCoast`] bundles the four
//! orbit channels (turn, zoom, pan, dolly-zoom); [`ReleaseVelocity`] is the heap-free ring that
//! turns the last [`RELEASE_WINDOW_S`] of a pointer drag into the release velocity;
//! [`CoastSettings`] is the user tuning (`time`, `softness`, `min_speed`).
//!
//! Why: the old friction + brake model reached rest only asymptotically (or by a threshold) and
//! its feel depended on the speed. Here the glide always lasts `time` seconds, the travel is
//! `v0 * time * R(1)` (a closed form), and the result does not depend on the frame rate: two
//! half steps equal one full step.
//!
//! How: with `u = (T - t) / T` the remaining fraction and `s = softness` (`kT`), the speed is
//! `v0 * (e^(s u) - 1) / (e^s - 1)` (`v0 * u` for `s -> 0`): it falls to exactly 0 at `u = 0`
//! with a finite deceleration, so there is no threshold anywhere. The distance still to go is
//! its integral [`coast_remaining_factor`]; a step is the difference of that at the two ends of
//! the frame. Softness 0 is a linear brake (travel `0.5 v0 T`); larger values glide longer at
//! the start and stop more gently (travel ~`0.281 v0 T` at 3).
//!
//! Uses: `glam`. Used by: `settings`, `prefs`, `mode` (the tuning), the orbit and fly rigs in
//! `rig` / `controller` (the glide itself) and the host input layer (the estimator).

use std::ops::{Add, Mul, Sub};

use glam::{Vec2, Vec3};

/// Length (s) of the pointer history the release velocity is measured over. A pause at least this
/// long before the release measures exactly zero: no coast.
pub const RELEASE_WINDOW_S: f32 = 0.08;

/// Default coast duration (s).
pub const DEFAULT_COAST_TIME: f32 = 0.6;

/// Default coast softness (`kT`, dimensionless): the glide's exponential curvature.
pub const DEFAULT_COAST_SOFTNESS: f32 = 3.0;

/// Default `min_speed`: a release slower than this (in the channel's normalised unit per second,
/// see [`Coast::start`]) does not coast.
pub const DEFAULT_COAST_MIN_SPEED: f32 = 0.02;

/// Largest coast time (s) [`CoastSettings::sanitized`] allows.
const TIME_MAX: f32 = 10.0;

/// Largest softness [`CoastSettings::sanitized`] allows (`e^20` is still a finite f32).
const SOFTNESS_MAX: f32 = 20.0;

/// Largest `min_speed` [`CoastSettings::sanitized`] allows.
const MIN_SPEED_MAX: f32 = 1.0e3;

/// Below this softness the speed curve is the linear limit `u` (the exponential form would
/// divide by a vanishing `e^s - 1`).
const SOFTNESS_LINEAR_EPS: f32 = 1e-4;

/// A coast counts as finished when its elapsed time is within this fraction of `T`: the f32 sum
/// of many frame steps may fall a few ulps short of `T`, and the speed there is ~0 (the skipped
/// tail travels `~1e-10` of the total), so it ends on the expected frame instead of one later.
const COMPLETE_EPS: f32 = 1.0e-5;

/// Smallest span (s) the release velocity divides by: a single very short sample is not a
/// measurable speed (it would amplify one noisy pointer delta).
const RELEASE_SPAN_MIN_S: f32 = 1.0 / 250.0;

/// Samples the [`ReleaseVelocity`] ring keeps (the newest win on overflow).
const RELEASE_RING: usize = 32;

/// User tuning of a coast: how long it lasts, how it eases, and the slowest release that starts it.
///
/// Every consumer reads it through [`CoastSettings::sanitized`], so a hand-edited file cannot
/// feed a NaN or a negative time into the glide.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default)
)]
pub struct CoastSettings {
    /// Duration (s) of the glide; `0` disables coasting.
    pub time: f32,
    /// Curvature of the glide (`kT`): 0 is a linear brake, larger values glide longer at speed
    /// and stop more gently.
    pub softness: f32,
    /// Slowest release that coasts, in the channel's normalised unit per second (rad/s for turns
    /// and looks, 1/s of log distance for zoom, orbit distances/s for pan, a cruise fraction/s
    /// for fly thrust).
    pub min_speed: f32,
}

impl Default for CoastSettings {
    fn default() -> Self {
        Self {
            time: DEFAULT_COAST_TIME,
            softness: DEFAULT_COAST_SOFTNESS,
            min_speed: DEFAULT_COAST_MIN_SPEED,
        }
    }
}

impl CoastSettings {
    /// Coasting off (`time` 0); softness and `min_speed` keep their defaults so switching it
    /// back on is a single field.
    pub const NONE: Self = Self {
        time: 0.0,
        softness: DEFAULT_COAST_SOFTNESS,
        min_speed: DEFAULT_COAST_MIN_SPEED,
    };

    /// The settings with every field finite and in range: `time` in `[0, 10]`, `softness` in
    /// `[0, 20]`, `min_speed` in `[0, 1000]`; a non-finite field becomes its default.
    pub fn sanitized(&self) -> Self {
        let field = |v: f32, default: f32, max: f32| {
            if v.is_finite() {
                v.clamp(0.0, max)
            } else {
                default
            }
        };
        Self {
            time: field(self.time, DEFAULT_COAST_TIME, TIME_MAX),
            softness: field(self.softness, DEFAULT_COAST_SOFTNESS, SOFTNESS_MAX),
            min_speed: field(self.min_speed, DEFAULT_COAST_MIN_SPEED, MIN_SPEED_MAX),
        }
    }

    /// True when the sanitized `time` is positive.
    pub fn enabled(&self) -> bool {
        self.sanitized().time > 0.0
    }
}

/// A velocity a [`Coast`] can glide: `f32` (log zoom, fov rate), `Vec2` (turn / look rates) and
/// `Vec3` (pan, thrust). Angles are plain rates here; nothing wraps in velocity space.
pub trait CoastValue:
    Copy + PartialEq + Add<Output = Self> + Sub<Output = Self> + Mul<f32, Output = Self>
{
    /// The zero velocity (rest).
    const ZERO: Self;

    /// Length of the velocity (absolute value for a scalar).
    fn magnitude(self) -> f32;

    /// True when every component is finite.
    fn is_finite(self) -> bool;
}

impl CoastValue for f32 {
    const ZERO: Self = 0.0;

    fn magnitude(self) -> f32 {
        self.abs()
    }

    fn is_finite(self) -> bool {
        f32::is_finite(self)
    }
}

impl CoastValue for Vec2 {
    const ZERO: Self = Vec2::ZERO;

    fn magnitude(self) -> f32 {
        self.length()
    }

    fn is_finite(self) -> bool {
        Vec2::is_finite(self)
    }
}

impl CoastValue for Vec3 {
    const ZERO: Self = Vec3::ZERO;

    fn magnitude(self) -> f32 {
        self.length()
    }

    fn is_finite(self) -> bool {
        Vec3::is_finite(self)
    }
}

/// `(e^x - 1 - x) / x^2` without the cancellation of the direct form near 0 (`x >= 0`).
fn exprel2(x: f32) -> f32 {
    if x.abs() < 0.05 {
        0.5 + x * (1.0 / 6.0 + x * (1.0 / 24.0 + x / 120.0))
    } else {
        (x.exp_m1() - x) / (x * x)
    }
}

/// `u` limited to `[0, 1]`; NaN is 0 (no speed, nothing left).
fn clamp_fraction(u: f32) -> f32 {
    if u > 0.0 { u.min(1.0) } else { 0.0 }
}

/// Speed as a fraction of the start speed at remaining fraction `u = (T - t) / T`
/// (clamped to `[0, 1]`): 1 at the start, exactly 0 at the end. `softness` is `kT`.
pub fn coast_speed_factor(u: f32, softness: f32) -> f32 {
    let u = clamp_fraction(u);
    let s = softness.max(0.0);
    if s < SOFTNESS_LINEAR_EPS {
        u
    } else {
        (s * u).exp_m1() / s.exp_m1()
    }
}

/// Distance still to go at remaining fraction `u`, as a fraction of `v0 * T`: the integral of
/// [`coast_speed_factor`] over `[0, u]`. `coast_remaining_factor(1, s)` is the whole travel.
pub fn coast_remaining_factor(u: f32, softness: f32) -> f32 {
    let u = clamp_fraction(u);
    let s = softness.max(0.0);
    if s < SOFTNESS_LINEAR_EPS {
        0.5 * u * u
    } else {
        s * u * u * exprel2(s * u) / s.exp_m1()
    }
}

/// One coasting velocity: glides `v0` to exact rest `time` seconds after [`Coast::start`].
///
/// `time` and `softness` are latched at the start, so retuning the settings mid-glide does not
/// bend a running coast. The idle state is `v0 == ZERO` ([`Coast::is_active`] is exact).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coast<V> {
    /// Start velocity; `ZERO` when idle.
    v0: V,
    /// Seconds since the start, `<= time`.
    elapsed: f32,
    /// Total duration (s), latched.
    time: f32,
    /// `kT`, latched.
    softness: f32,
}

impl<V: CoastValue> Default for Coast<V> {
    fn default() -> Self {
        Self {
            v0: V::ZERO,
            elapsed: 0.0,
            time: 0.0,
            softness: 0.0,
        }
    }
}

impl<V: CoastValue> Coast<V> {
    /// Start gliding from `v0`, replacing any running coast. `unit` converts the channel's
    /// `min_speed` (a normalised rate) into the units of `v0`: the coast starts only when
    /// `|v0| >= min_speed * unit`.
    ///
    /// Returns `false` and leaves the coast idle when coasting is off (`time` 0), `v0` or `unit`
    /// is non-finite, or `v0` is zero / below the minimum.
    pub fn start(&mut self, v0: V, unit: f32, s: &CoastSettings) -> bool {
        *self = Self::default();
        let s = s.sanitized();
        if s.time <= 0.0 || !v0.is_finite() || !unit.is_finite() {
            return false;
        }
        let speed = v0.magnitude();
        if !(speed.is_finite() && speed > 0.0 && speed >= s.min_speed * unit.max(0.0)) {
            return false;
        }
        *self = Self {
            v0,
            elapsed: 0.0,
            time: s.time,
            softness: s.softness,
        };
        true
    }

    /// Start from the current velocity plus `dv` (a wheel notch on top of a running zoom coast).
    /// Same return and units as [`Coast::start`].
    pub fn kick(&mut self, dv: V, unit: f32, s: &CoastSettings) -> bool {
        let v = self.velocity() + dv;
        self.start(v, unit, s)
    }

    /// End the coast at once (a press, a direct edit).
    pub fn stop(&mut self) {
        *self = Self::default();
    }

    /// True while gliding. Exact: the glide ends at exactly zero, so no threshold is involved.
    pub fn is_active(&self) -> bool {
        self.v0 != V::ZERO
    }

    /// Remaining fraction `u = (T - t) / T` in `[0, 1]`.
    fn fraction(&self) -> f32 {
        clamp_fraction(1.0 - self.elapsed / self.time)
    }

    /// The velocity now (`ZERO` when idle).
    pub fn velocity(&self) -> V {
        if !self.is_active() {
            return V::ZERO;
        }
        self.v0 * coast_speed_factor(self.fraction(), self.softness)
    }

    /// The displacement from now until the glide ends (`ZERO` when idle).
    pub fn remaining(&self) -> V {
        if !self.is_active() {
            return V::ZERO;
        }
        self.v0 * (self.time * coast_remaining_factor(self.fraction(), self.softness))
    }

    /// Advance by `dt` seconds and return the displacement over the frame (closed form, so two
    /// half steps equal one full step). A step reaching `T` returns everything that was left and
    /// ends the coast at exactly rest. An unusable `dt` (not positive and finite) or an idle coast
    /// returns `ZERO` and changes nothing.
    pub fn step(&mut self, dt: f32) -> V {
        if !self.is_active() || !(dt > 0.0 && dt.is_finite()) {
            return V::ZERO;
        }
        let u0 = self.fraction();
        let r0 = coast_remaining_factor(u0, self.softness);
        self.elapsed = (self.elapsed + dt).min(self.time);
        if self.elapsed >= self.time * (1.0 - COMPLETE_EPS) {
            let d = self.v0 * (self.time * r0);
            *self = Self::default();
            return d;
        }
        let r1 = coast_remaining_factor(self.fraction(), self.softness);
        self.v0 * (self.time * (r0 - r1))
    }
}

/// The four coasting channels of an orbit camera, each in its own unit.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OrbitCoast {
    /// Turn rate (yaw, pitch) in rad/s.
    pub turn: Coast<Vec2>,
    /// Log-distance rate in 1/s (the distance scales by `exp(rate * dt)`).
    pub zoom: Coast<f32>,
    /// Pivot velocity in world units/s.
    pub pan: Coast<Vec3>,
    /// Vertical-FOV rate in rad/s (dolly-zoom).
    pub dolly_zoom: Coast<f32>,
}

/// What one [`OrbitCoast::step`] moves the camera by.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrbitCoastStep {
    /// Yaw / pitch turned over the frame (radians).
    pub turn: Vec2,
    /// Change of the log distance over the frame.
    pub log_zoom: f32,
    /// Pivot displacement over the frame (world units).
    pub pan: Vec3,
    /// Vertical-FOV change over the frame (radians).
    pub dfov: f32,
}

impl OrbitCoast {
    /// End every channel at once.
    pub fn stop(&mut self) {
        *self = Self::default();
    }

    /// True while any channel glides.
    pub fn is_active(&self) -> bool {
        self.turn.is_active()
            || self.zoom.is_active()
            || self.pan.is_active()
            || self.dolly_zoom.is_active()
    }

    /// Advance every channel by `dt` and return the displacements.
    pub fn step(&mut self, dt: f32) -> OrbitCoastStep {
        OrbitCoastStep {
            turn: self.turn.step(dt),
            log_zoom: self.zoom.step(dt),
            pan: self.pan.step(dt),
            dfov: self.dolly_zoom.step(dt),
        }
    }
}

/// Release-velocity estimator: a heap-free ring of the last pointer steps of a gesture.
///
/// Push every frame's `(step, dt)` while dragging, ZERO steps included: a pause is data and is
/// what makes a drag that stopped before the release measure exactly zero. [`ReleaseVelocity::velocity`]
/// then averages the last [`RELEASE_WINDOW_S`], frame-rate independent because the steps are
/// weighted by their time, not counted.
#[derive(Debug, Clone, Copy)]
pub struct ReleaseVelocity<V> {
    /// `(step, dt)` samples; `head` is the next write slot.
    ring: [(V, f32); RELEASE_RING],
    /// Next write index.
    head: usize,
    /// Valid samples, `<= RELEASE_RING`.
    len: usize,
}

impl<V: CoastValue> Default for ReleaseVelocity<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: CoastValue> ReleaseVelocity<V> {
    /// An empty estimator.
    pub fn new() -> Self {
        Self {
            ring: [(V::ZERO, 0.0); RELEASE_RING],
            head: 0,
            len: 0,
        }
    }

    /// Forget the gesture (a press starts a new one).
    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }

    /// Record a pointer step taken over `dt` seconds. A zero step IS recorded (the pause); a
    /// `dt` that is not positive and finite, or a non-finite step, is ignored. The newest 32
    /// samples are kept.
    pub fn push(&mut self, step: V, dt: f32) {
        if !(dt > 0.0 && dt.is_finite()) || !step.is_finite() {
            return;
        }
        self.ring[self.head] = (step, dt);
        self.head = (self.head + 1) % RELEASE_RING;
        self.len = (self.len + 1).min(RELEASE_RING);
    }

    /// The release velocity: the steps of the newest [`RELEASE_WINDOW_S`] summed (the oldest,
    /// partly covered sample pro rata) over the time they cover, floored at 1/250 s so one short
    /// sample is not amplified. Exactly `ZERO` when the window holds only zero steps, or when
    /// nothing was recorded.
    pub fn velocity(&self) -> V {
        let mut left = RELEASE_WINDOW_S;
        let mut sum = V::ZERO;
        let mut covered = 0.0_f32;
        for back in 0..self.len {
            if left <= 0.0 {
                break;
            }
            let (step, dt) = self.ring[(self.head + RELEASE_RING - 1 - back) % RELEASE_RING];
            if dt <= left {
                sum = sum + step;
                left -= dt;
                covered += dt;
            } else {
                sum = sum + step * (left / dt);
                covered += left;
                left = 0.0;
            }
        }
        if covered <= 0.0 {
            return V::ZERO;
        }
        sum * (1.0 / covered.max(RELEASE_SPAN_MIN_S))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Settings without a minimum speed, so tiny test velocities start.
    fn cfg(time: f32, softness: f32) -> CoastSettings {
        CoastSettings {
            time,
            softness,
            min_speed: 0.0,
        }
    }

    /// Step until idle at fixed `dt`; returns `(total displacement, steps)`.
    fn drain(c: &mut Coast<f32>, dt: f32) -> (f64, usize) {
        let (mut total, mut steps) = (0.0_f64, 0);
        while c.is_active() {
            total += f64::from(c.step(dt));
            steps += 1;
            assert!(steps < 100_000, "coast never ended");
        }
        (total, steps)
    }

    /// The coast ends at EXACT rest on the first step at or past `T` (`ceil(T / dt)` steps of a
    /// fixed grid), with no threshold, for slow and fast starts, every softness and any step size.
    #[test]
    fn stops_exactly_at_t_on_the_frame_grid() {
        let t = 0.6_f32;
        for v0 in [1e-3_f32, 0.5, 7.0, 1e4] {
            for softness in [0.0_f32, 3.0, 4.0] {
                for dt in [1.0 / 30.0, 1.0 / 60.0, 1.0 / 144.0, 0.37 * t, 2.0 * t] {
                    let mut c = Coast::<f32>::default();
                    assert!(c.start(v0, 1.0, &cfg(t, softness)), "v0 {v0}");
                    let (_, steps) = drain(&mut c, dt);
                    // The 1e-3 absorbs the f32 rounding of `t / dt` on exact grids (36.000004).
                    let expect = (t / dt - 1e-3).ceil() as usize;
                    assert_eq!(steps, expect, "v0 {v0} s {softness} dt {dt}");
                    assert!(!c.is_active());
                    assert_eq!(c.velocity(), 0.0);
                    assert_eq!(c.remaining(), 0.0);
                    assert_eq!(c.step(dt), 0.0, "idle steps nothing");
                }
            }
        }
    }

    /// The total travel is `v0 * T * R(1)`: half `v0 T` for a linear brake, ~0.281 `v0 T` at
    /// softness 3; `remaining()` at the start is that travel, and `velocity()` follows the curve.
    #[test]
    fn travel_matches_the_closed_form() {
        let (v0, t) = (7.0_f32, 0.6_f32);
        assert!((coast_remaining_factor(1.0, 0.0) - 0.5).abs() < 1e-7);
        assert!((coast_remaining_factor(1.0, 3.0) - 0.2810).abs() < 1e-3);
        for softness in [0.0_f32, 1.0, 3.0, 4.0, 12.0] {
            let want = f64::from(v0 * t * coast_remaining_factor(1.0, softness));
            let mut c = Coast::<f32>::default();
            assert!(c.start(v0, 1.0, &cfg(t, softness)));
            assert!((f64::from(c.remaining()) - want).abs() <= 1e-5 * want);
            assert_eq!(c.velocity(), v0, "full speed at the start");
            let (got, _) = drain(&mut c, 1.0 / 60.0);
            assert!(
                (got - want).abs() <= 1e-5 * want,
                "s {softness}: {got} vs {want}"
            );
        }
        // Midway: remaining() is the distance x(t) still to go, velocity() the formula.
        let mut c = Coast::<f32>::default();
        assert!(c.start(v0, 1.0, &cfg(t, 3.0)));
        let before = c.remaining();
        let moved = c.step(t * 0.5);
        assert!((before - moved - c.remaining()).abs() < 1e-4);
        let want = v0 * coast_speed_factor(0.5, 3.0);
        assert!((c.velocity() - want).abs() <= 1e-5 * want);
    }

    /// exprel2 is continuous across its series / direct switch and matches the f64 reference.
    #[test]
    fn exprel2_matches_the_reference() {
        for x in [1e-6_f32, 0.01, 0.0499, 0.0501, 0.3, 2.0, 20.0] {
            let xd = f64::from(x);
            let want = (xd.exp_m1() - xd) / (xd * xd);
            assert!(
                (f64::from(exprel2(x)) - want).abs() <= 1e-4 * want,
                "x {x}: {} vs {want}",
                exprel2(x)
            );
        }
    }

    /// Frame-rate independence: two half steps equal one full step, and the totals at 30 / 60 /
    /// 144 / 240 fps agree.
    #[test]
    fn frame_rate_independent() {
        for softness in [0.0_f32, 3.0] {
            let mut one = Coast::<f32>::default();
            let mut two = Coast::<f32>::default();
            assert!(one.start(5.0, 1.0, &cfg(0.6, softness)));
            assert!(two.start(5.0, 1.0, &cfg(0.6, softness)));
            let a = one.step(0.1);
            let b = two.step(0.05) + two.step(0.05);
            assert!((a - b).abs() < 1e-5, "{a} vs {b}");
            assert!((one.velocity() - two.velocity()).abs() < 1e-5);

            let totals: Vec<f64> = [30.0_f32, 60.0, 144.0, 240.0]
                .iter()
                .map(|fps| {
                    let mut c = Coast::<f32>::default();
                    assert!(c.start(5.0, 1.0, &cfg(0.6, softness)));
                    drain(&mut c, 1.0 / fps).0
                })
                .collect();
            for t in &totals {
                assert!((t - totals[0]).abs() <= 1e-5 * totals[0], "{totals:?}");
            }
        }
    }

    /// Vector channels glide along the start direction and stop at exact zero too.
    #[test]
    fn vector_coast_keeps_its_direction() {
        let mut c = Coast::<Vec2>::default();
        let v0 = Vec2::new(3.0, -4.0);
        assert!(c.start(v0, 1.0, &CoastSettings::default()));
        let mut total = Vec2::ZERO;
        let mut steps = 0;
        while c.is_active() {
            total += c.step(1.0 / 60.0);
            steps += 1;
            assert!(steps < 1000);
        }
        assert!(total.normalize().dot(v0.normalize()) > 0.99999);
        let want = 5.0 * 0.6 * coast_remaining_factor(1.0, 3.0);
        assert!((total.length() - want).abs() <= 1e-4 * want);
    }

    /// `kick` adds to the running velocity (no jump), and a kick that cancels it ends the coast.
    #[test]
    fn kick_continues_from_the_current_velocity() {
        let s = cfg(0.6, 3.0);
        let mut c = Coast::<f32>::default();
        assert!(c.start(2.0, 1.0, &s));
        c.step(0.2);
        let v = c.velocity();
        assert!(c.kick(1.0, 1.0, &s));
        assert!((c.velocity() - (v + 1.0)).abs() < 1e-6);
        assert!(!c.kick(-(v + 1.0), 1.0, &s), "cancelled out: idle");
        assert!(!c.is_active());
        c.start(2.0, 1.0, &s);
        c.stop();
        assert!(!c.is_active());
    }

    /// Garbage never starts a coast or moves one: non-finite or slow starts, time 0, bad `dt`,
    /// NaN settings.
    #[test]
    fn garbage_inputs_are_refused() {
        let s = CoastSettings::default();
        let mut c = Coast::<f32>::default();
        for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 0.0, -0.0] {
            assert!(!c.start(bad, 1.0, &s), "{bad}");
            assert!(!c.is_active());
        }
        assert!(!c.start(1.0, f32::NAN, &s), "NaN unit");
        assert!(!c.start(0.01, 1.0, &s), "below min_speed * unit");
        assert!(c.start(0.01, 0.1, &s), "unit scales the minimum");
        assert!(!c.start(5.0, 1.0, &CoastSettings::NONE), "time 0");
        let mut v = Coast::<Vec3>::default();
        assert!(
            !v.start(Vec3::new(f32::MAX, f32::MAX, f32::MAX), 1.0, &s),
            "overflowing length"
        );
        assert!(!v.start(Vec3::new(1.0, f32::NAN, 0.0), 1.0, &s));

        assert!(c.start(5.0, 1.0, &s));
        let before = c;
        for dt in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            assert_eq!(c.step(dt), 0.0);
            assert_eq!(c, before, "dt {dt} changed the coast");
        }

        let nan = CoastSettings {
            time: f32::NAN,
            softness: f32::INFINITY,
            min_speed: -1.0,
        }
        .sanitized();
        assert_eq!(nan.time, DEFAULT_COAST_TIME);
        assert_eq!(nan.softness, DEFAULT_COAST_SOFTNESS);
        assert_eq!(nan.min_speed, 0.0);
        let big = CoastSettings {
            time: 99.0,
            softness: 99.0,
            min_speed: 1e9,
        }
        .sanitized();
        assert_eq!(
            (big.time, big.softness, big.min_speed),
            (10.0, 20.0, 1000.0)
        );
        assert!(!CoastSettings::NONE.enabled());
        assert!(CoastSettings::default().enabled());
        assert!(
            !CoastSettings {
                time: -3.0,
                ..CoastSettings::default()
            }
            .enabled()
        );
        // The curve helpers are total: NaN / out-of-range fractions and softness stay finite.
        assert_eq!(coast_speed_factor(f32::NAN, 3.0), 0.0);
        assert_eq!(coast_speed_factor(2.0, 3.0), 1.0);
        assert_eq!(coast_remaining_factor(-1.0, 3.0), 0.0);
        assert!(coast_speed_factor(0.5, f32::NAN).is_finite());
    }

    /// Settings latched at the start: the running glide ignores a later retune, and a coast
    /// started with softness 20 stays finite.
    #[test]
    fn latched_settings_and_extreme_softness() {
        let mut c = Coast::<f32>::default();
        assert!(c.start(1.0, 1.0, &cfg(0.6, 20.0)));
        assert!(c.step(0.01).is_finite());
        assert!(c.velocity().is_finite() && c.remaining().is_finite());
        let (rest, _) = drain(&mut c, 1.0 / 60.0);
        assert!(rest.is_finite());
    }

    /// A drag at constant speed measures that speed at 30..240 fps (within 1%).
    #[test]
    fn estimator_constant_speed() {
        for fps in [30.0_f32, 60.0, 144.0, 240.0] {
            let dt = 1.0 / fps;
            let mut r = ReleaseVelocity::<f32>::new();
            for _ in 0..(fps * 0.5) as usize {
                r.push(120.0 * dt, dt);
            }
            let v = r.velocity();
            assert!((v - 120.0).abs() <= 1.2, "{fps} fps: {v}");
        }
        let mut r = ReleaseVelocity::<Vec2>::default();
        for _ in 0..30 {
            r.push(Vec2::new(0.1, -0.2), 1.0 / 60.0);
        }
        assert!((r.velocity() - Vec2::new(6.0, -12.0)).length() < 0.1);
    }

    /// A pause of at least the window measures EXACTLY zero (no coast); a 40 ms pause measures
    /// about half the speed; before any samples or after `clear` it is zero.
    #[test]
    fn estimator_pause_gives_zero() {
        let dt = 1.0 / 60.0;
        let mut r = ReleaseVelocity::<f32>::new();
        assert_eq!(r.velocity(), 0.0);
        for _ in 0..20 {
            r.push(2.0 * dt, dt);
        }
        for _ in 0..6 {
            r.push(0.0, dt);
        }
        assert_eq!(r.velocity(), 0.0, "100 ms pause is exact zero");

        let mut half = ReleaseVelocity::<f32>::new();
        for _ in 0..20 {
            half.push(2.0 * dt, dt);
        }
        half.push(0.0, 0.04);
        let v = half.velocity();
        assert!((v - 1.0).abs() < 0.05, "40 ms pause ~ half: {v}");

        half.clear();
        assert_eq!(half.velocity(), 0.0);
    }

    /// Garbage pushes are ignored, a single short sample is not amplified past 1/250 s, and a
    /// flood past the ring size stays finite and still measures the newest window.
    #[test]
    fn estimator_overflow_and_garbage() {
        let mut r = ReleaseVelocity::<f32>::new();
        r.push(f32::NAN, 0.016);
        r.push(1.0, 0.0);
        r.push(1.0, -1.0);
        r.push(1.0, f32::NAN);
        r.push(f32::INFINITY, 0.016);
        assert_eq!(r.velocity(), 0.0, "all ignored");

        r.push(1.0, 0.001);
        // One 1 ms sample divides by the 4 ms floor: 1 / 0.004 = 250.
        assert!((r.velocity() - 1.0 / RELEASE_SPAN_MIN_S).abs() < 1e-2);

        let mut flood = ReleaseVelocity::<f32>::new();
        for _ in 0..1000 {
            flood.push(3.0 * 0.004, 0.004);
        }
        let v = flood.velocity();
        assert!(v.is_finite() && (v - 3.0).abs() < 0.03, "{v}");
        for _ in 0..40 {
            flood.push(0.0, 0.004);
        }
        assert_eq!(flood.velocity(), 0.0, "the pause wins after overflow");
    }

    /// `OrbitCoast` steps every channel and reports activity until all are at rest.
    #[test]
    fn orbit_coast_channels() {
        let s = cfg(0.3, 3.0);
        let mut o = OrbitCoast::default();
        assert!(!o.is_active());
        assert!(o.turn.start(Vec2::new(1.0, 0.5), 1.0, &s));
        assert!(o.zoom.start(0.4, 1.0, &s));
        assert!(o.pan.start(Vec3::X, 1.0, &s));
        assert!(o.dolly_zoom.start(-0.2, 1.0, &s));
        let first = o.step(1.0 / 60.0);
        assert!(
            first.turn.x > 0.0 && first.log_zoom > 0.0 && first.pan.x > 0.0 && first.dfov < 0.0
        );
        let mut n = 0;
        while o.is_active() {
            o.step(1.0 / 60.0);
            n += 1;
            assert!(n < 100);
        }
        o.turn.start(Vec2::X, 1.0, &s);
        o.stop();
        assert!(!o.is_active());
    }
}
