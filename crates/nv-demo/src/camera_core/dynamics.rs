//! Animated view transitions and the locked / free view rotation rule.
//!
//! What: [`TransitionState`] (the time-based orbit fly-to used by [`crate::camera_core::rig::OrbitRig`]),
//! [`rotate_view`] (the one yaw / pitch rule of the fly backends and their look coasts) and
//! [`LOCK_POLE_LIMIT`].
//!
//! Why: coasts live in [`crate::camera_core::coast`] (finite-time glides to exact rest); this module keeps
//! only what is not a coast. A fly-to is a pending state, not a velocity.
//!
//! How: everything is time-free: the caller passes `dt` (seconds) and every ease is the exact
//! exponential over that `dt`, so the motion does not depend on the frame rate.

use glam::{Quat, Vec3};

/// Squared length below which a rotation axis counts as degenerate (no rotation applied).
const MIN_AXIS_LEN_SQ: f32 = 1e-10;

/// Turn a view by `dyaw` / `dpitch` (radians): the one rotation rule of the fly backends.
///
/// Free (`lock_up` None): yaw about the view's own up, pitch about its right (6-DoF).
/// Locked: yaw about `lock_up` and pitch about the right axis; pitch input cannot carry the
/// forward vector's elevation past [`LOCK_POLE_LIMIT`] (i.e. closer than 1° = `pi/2 - limit`
/// to the lock axis), so the view never flips over the pole. It is a STEP rule: a view placed
/// past the limit (by the host) stays where it is; pitch only moves it back toward the limit.
pub fn rotate_view(orientation: Quat, dyaw: f32, dpitch: f32, lock_up: Option<Vec3>) -> Quat {
    let mut o = orientation;
    let yaw_axis = match lock_up {
        Some(up) => up,
        None => (o * Vec3::Y).normalize_or_zero(),
    };
    if dyaw != 0.0 && yaw_axis.length_squared() > MIN_AXIS_LEN_SQ {
        o = (Quat::from_axis_angle(yaw_axis, dyaw) * o).normalize();
    }
    if dpitch != 0.0 {
        let right = (o * Vec3::X).normalize_or_zero();
        let step = match lock_up {
            // Elevation of the forward vector above the lock plane; positive pitch raises it.
            // A STEP rule, not a clamp of the result: a view already past the limit is never
            // pulled back to it (no jump on the next tiny input); it may only move toward it.
            Some(up) => {
                let elevation = crate::camera_core::num::clamp((o * -Vec3::Z).dot(up), -1.0, 1.0).asin();
                crate::camera_core::num::clamp(
                    elevation + dpitch,
                    elevation.min(-LOCK_POLE_LIMIT),
                    elevation.max(LOCK_POLE_LIMIT),
                ) - elevation
            }
            None => dpitch,
        };
        if step != 0.0 && right.length_squared() > MIN_AXIS_LEN_SQ {
            o = (Quat::from_axis_angle(right, step) * o).normalize();
        }
    }
    o
}

/// Largest elevation (89°) pitch input may give a locked view's forward vector above / below
/// the lock plane: it stays at least `pi/2 - LOCK_POLE_LIMIT` (1°) off the lock axis. A step
/// limit ([`rotate_view`]), not a clamp: a view placed past it is not pulled back.
pub const LOCK_POLE_LIMIT: f32 = 89.0 * std::f32::consts::PI / 180.0;

/// Smooth fly-to of the orbit rig: eases orientation (slerp), distance and pivot toward targets.
#[derive(Debug, Clone)]
pub struct TransitionState {
    /// Orientation the transition eases toward.
    pub orientation_target: Quat,
    /// Orbit distance the transition eases toward.
    pub distance_target: f32,
    /// Pivot position the transition eases toward.
    pub target_target: Vec3,
    /// True while a transition is running (read through [`TransitionState::is_animating`]; only
    /// `animate_to` starts it and only `cancel` / landing end it, so the start state stays
    /// consistent).
    animating: bool,
    /// Pose `(orientation, distance, pivot)` the transition eases from; captured by the first
    /// `step` after [`TransitionState::animate_to`] (which does not receive the current pose).
    start: Option<(Quat, f32, Vec3)>,
    /// Fraction of the way still to go, `exp(-speed * elapsed)`; 1 at the start.
    remaining: f32,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self {
            orientation_target: Quat::IDENTITY,
            distance_target: crate::camera_core::convention::DEFAULT_ORBIT_DISTANCE,
            target_target: Vec3::ZERO,
            animating: false,
            start: None,
            remaining: 1.0,
        }
    }
}

impl TransitionState {
    /// Start (or retarget) a transition toward these targets. It eases from the pose the next
    /// [`TransitionState::step`] receives, so a retarget mid-transition continues from where
    /// the camera is (no jump back).
    ///
    /// It does NOT touch any coast: a coast keeps integrating over the transition and would
    /// fight it, so the caller stops its coasts first (`OrbitCoast::stop`).
    pub fn animate_to(&mut self, orientation: Quat, distance: f32, target: Vec3) {
        self.orientation_target = orientation;
        self.distance_target = distance;
        self.target_target = target;
        self.reset();
        self.animating = true;
    }

    /// True while a transition is running.
    pub fn is_animating(&self) -> bool {
        self.animating
    }

    /// Stop the transition where it is (the pose is left as the last step put it).
    pub fn cancel(&mut self) {
        self.reset();
    }

    /// End any transition and forget its start pose and progress, so nothing can resume from
    /// a stale start.
    fn reset(&mut self) {
        self.animating = false;
        self.start = None;
        self.remaining = 1.0;
    }

    /// Advance the ease by `dt`: the remaining fraction decays as `exp(-speed * elapsed)`
    /// (frame-rate independent: two half steps equal one full step) and the pose is the start
    /// pose interpolated toward the targets by `1 - remaining` (slerp / lerp, so no overshoot).
    ///
    /// Termination is TIME-based, independent of scale and f32 spacing: the transition lands
    /// exactly on the targets once `remaining < TRANSITION_REMAINING_EPS` (1e-4), i.e. after
    /// `ln(1e4) / speed` ≈ `9.2 / speed` seconds; the final jump is below 1e-4 of the travel.
    /// `speed <= 0` (or NaN) is an instant jump (lands, returns false).
    ///
    /// **`dt` contract:** a `dt <= 0` (or NaN) frame is a no-op that leaves the pose untouched
    /// and, while a transition is pending, returns **true** (it is still animating; the next
    /// positive `dt` continues it). This differs on purpose from the controller backends'
    /// `update_dynamics`, which treat an unusable `dt` as "nothing moved" and return false: a
    /// transition is a pending state, a coast is motion. Returns true while animating.
    pub fn step(
        &mut self,
        orientation: &mut Quat,
        distance: &mut f32,
        target: &mut Vec3,
        dt: f32,
        speed: f32,
    ) -> bool {
        if !self.animating {
            return false;
        }
        if speed.is_nan() || speed <= 0.0 {
            self.land(orientation, distance, target);
            return false;
        }
        if dt.is_nan() || dt <= 0.0 {
            return true;
        }
        let (q0, d0, p0) = *self.start.get_or_insert((*orientation, *distance, *target));
        self.remaining *= (-speed * dt).exp();
        if self.remaining < TRANSITION_REMAINING_EPS {
            self.land(orientation, distance, target);
            return false;
        }
        let t = 1.0 - self.remaining;
        *orientation = q0.slerp(self.orientation_target, t).normalize();
        *distance = d0 + (self.distance_target - d0) * t;
        *target = p0.lerp(self.target_target, t);
        true
    }

    /// Put the pose exactly on the targets and end the transition.
    fn land(&mut self, orientation: &mut Quat, distance: &mut f32, target: &mut Vec3) {
        *orientation = self.orientation_target;
        *distance = self.distance_target;
        *target = self.target_target;
        self.reset();
    }
}

/// Remaining fraction of a `TransitionState` ease below which it lands on its targets: the
/// final jump is under 1e-4 of the travel, reached after `ln(1e4) / speed` seconds.
const TRANSITION_REMAINING_EPS: f32 = 1.0e-4;

#[cfg(test)]
mod tests {
    use super::*;

    /// The ease is exponential: two half steps land where one full step does, and the
    /// transition ends exactly on its targets (also at a large target distance).
    #[test]
    fn transition_eases_frame_rate_independent_and_lands_exactly() {
        let goal = Quat::from_rotation_y(1.2);
        let start = |tr: &mut TransitionState| {
            tr.animate_to(goal, 5000.0, Vec3::new(100.0, 0.0, 0.0));
        };

        let mut one = TransitionState::default();
        start(&mut one);
        let (mut q1, mut d1, mut p1) = (Quat::IDENTITY, 10.0_f32, Vec3::ZERO);
        one.step(&mut q1, &mut d1, &mut p1, 0.1, 8.0);

        let mut two = TransitionState::default();
        start(&mut two);
        let (mut q2, mut d2, mut p2) = (Quat::IDENTITY, 10.0_f32, Vec3::ZERO);
        two.step(&mut q2, &mut d2, &mut p2, 0.05, 8.0);
        two.step(&mut q2, &mut d2, &mut p2, 0.05, 8.0);

        assert!((d1 - d2).abs() < 1e-2, "{d1} vs {d2}");
        assert!(p1.distance(p2) < 1e-3, "{p1} vs {p2}");
        assert!(1.0 - q1.dot(q2).abs() < 1e-6, "{q1} vs {q2}");

        let mut frames = 0;
        while two.step(&mut q2, &mut d2, &mut p2, 1.0 / 60.0, 8.0) {
            frames += 1;
            assert!(frames < 1000, "transition never converged");
        }
        assert_eq!((q2, d2, p2), (goal, 5000.0, Vec3::new(100.0, 0.0, 0.0)));
    }

    /// A pivot far from the origin (f32 spacing ~0.008 at 1e5) still lands, in bounded frames
    /// (termination is time-based, not an epsilon on the pose).
    #[test]
    fn transition_far_pivot_lands() {
        let far = Vec3::new(1.0e5, 0.0, 0.0);
        let goal = Quat::from_rotation_y(0.5);
        let mut tr = TransitionState::default();
        tr.animate_to(goal, 10.0, far);
        let (mut q, mut d, mut p) = (Quat::IDENTITY, 10.0_f32, Vec3::ZERO);
        let mut frames = 0;
        while tr.step(&mut q, &mut d, &mut p, 1.0 / 60.0, 8.0) {
            frames += 1;
            assert!(frames < 400, "far pivot never converged: {p}");
        }
        assert_eq!((q, d, p), (goal, 10.0, far));
    }

    /// A slow ease (`speed * dt` far below 1e-3) at a near and a far pivot lands in the
    /// time-bounded frame count `ln(1e4) / (speed * dt)`, never overshoots, and its final
    /// landing jump is below 1e-4 of the travel (continuous to the eye).
    #[test]
    fn transition_slow_ease_lands_in_bounded_time_without_overshoot() {
        let (speed, dt) = (0.05_f32, 1.0 / 60.0);
        // ln(1e4) / (0.05 / 60) ≈ 11052 frames.
        let bound = 11_200;
        for goal_p in [Vec3::new(100.0, 0.0, 0.0), Vec3::new(1.0e5, 0.0, 0.0)] {
            let goal_q = Quat::from_rotation_y(1.0);
            let mut tr = TransitionState::default();
            tr.animate_to(goal_q, 40.0, goal_p);
            let (mut q, mut d, mut p) = (Quat::IDENTITY, 10.0_f32, Vec3::ZERO);
            let travel = goal_p.length();
            let mut gap = travel;
            let mut frames = 0;
            while tr.step(&mut q, &mut d, &mut p, dt, speed) {
                frames += 1;
                assert!(frames < bound, "{goal_p}: never landed");
                let g = (p - goal_p).length();
                assert!(g <= gap, "{goal_p}: moved away ({gap} -> {g})");
                assert!(p.x >= 0.0 && p.x <= goal_p.x, "{goal_p}: overshoot {p}");
                assert!(
                    (10.0..=40.0).contains(&d),
                    "{goal_p}: distance overshoot {d}"
                );
                gap = g;
            }
            assert!(frames > 10_000, "{goal_p}: landed early at {frames}");
            assert!(
                gap <= 1.01e-4 * travel,
                "{goal_p}: final jump {gap} vs travel {travel}"
            );
            assert_eq!((q, d, p), (goal_q, 40.0, goal_p));
        }
    }

    /// `speed <= 0` is an instant jump (no endless zero-progress transition); a `dt == 0`
    /// frame keeps animating without moving.
    #[test]
    fn transition_speed_zero_jumps_dt_zero_waits() {
        let goal = Quat::from_rotation_x(0.3);
        let tgt = Vec3::new(1.0, 2.0, 3.0);
        let mut tr = TransitionState::default();
        tr.animate_to(goal, 20.0, tgt);
        let (mut q, mut d, mut p) = (Quat::IDENTITY, 10.0_f32, Vec3::ZERO);
        assert!(
            tr.step(&mut q, &mut d, &mut p, 0.0, 8.0),
            "dt 0 keeps animating"
        );
        assert_eq!(
            (q, d, p),
            (Quat::IDENTITY, 10.0, Vec3::ZERO),
            "dt 0 does not move"
        );
        assert!(!tr.step(&mut q, &mut d, &mut p, 1.0 / 60.0, 0.0));
        assert_eq!((q, d, p), (goal, 20.0, tgt));
        assert!(!tr.is_animating());
    }

    /// `cancel` and landing forget the start pose and progress: a later `animate_to` eases
    /// from the pose its first step receives, never from a stale start.
    #[test]
    fn transition_cancel_and_land_reset_the_start() {
        let mut tr = TransitionState::default();
        let goal = Quat::from_rotation_y(1.0);
        tr.animate_to(goal, 20.0, Vec3::X);
        let (mut q, mut d, mut p) = (Quat::IDENTITY, 10.0_f32, Vec3::ZERO);
        assert!(tr.step(&mut q, &mut d, &mut p, 1.0 / 60.0, 8.0));
        assert!(tr.start.is_some() && tr.remaining < 1.0);

        tr.cancel();
        assert!(!tr.is_animating());
        assert!(tr.start.is_none() && tr.remaining == 1.0, "cancel resets");

        // Animate again from a different pose: the first step eases from THAT pose.
        let (mut q, mut d, mut p) = (Quat::from_rotation_x(0.5), 50.0_f32, Vec3::Y * 7.0);
        let from = (q, d, p);
        tr.animate_to(goal, 20.0, Vec3::X);
        assert!(tr.step(&mut q, &mut d, &mut p, 1.0 / 60.0, 8.0));
        assert_eq!(tr.start, Some(from), "fresh start, not the cancelled one");
        let t = 1.0 - (-8.0_f32 / 60.0).exp();
        assert!(
            (d - (50.0 + (20.0 - 50.0) * t)).abs() < 1e-4,
            "eased from the new pose: {d}"
        );

        // Landing resets too.
        assert!(!tr.step(&mut q, &mut d, &mut p, 1.0 / 60.0, 0.0));
        assert!(!tr.is_animating());
        assert!(tr.start.is_none() && tr.remaining == 1.0, "landing resets");
    }

    /// A locked view already past `LOCK_POLE_LIMIT` is not pulled back on a tiny pitch input:
    /// up-pitch is refused, down-pitch moves by exactly the step (no ~1 degree jump).
    #[test]
    fn locked_pitch_past_the_pole_limit_never_jumps() {
        let elevation = |o: Quat| (o * -Vec3::Z).dot(Vec3::Y).clamp(-1.0, 1.0).asin();
        let e0 = 89.5_f32.to_radians();
        let o = Quat::from_rotation_x(e0);
        assert!((elevation(o) - e0).abs() < 1e-4);

        let up = rotate_view(o, 0.0, 0.001, Some(Vec3::Y));
        assert!(
            (elevation(up) - e0).abs() < 1e-4,
            "up-pitch past the limit moved it"
        );

        let down = rotate_view(o, 0.0, -0.001, Some(Vec3::Y));
        assert!(
            (elevation(down) - (e0 - 0.001)).abs() < 1e-4,
            "down-pitch jumped: {} -> {}",
            e0.to_degrees(),
            elevation(down).to_degrees()
        );
    }
}
