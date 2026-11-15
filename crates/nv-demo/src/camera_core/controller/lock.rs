//! [`HorizonLock`]: the horizon lock of [`super::SpaceFlight`].
//!
//! What: keeps the view right axis in a lock plane while yaw turns about the lock axis; roll is
//! a critically damped spring toward that plane, and a held roll key flips the lock to the next
//! plane of an orthonormal grid.
//!
//! Why: a spacesim with an "autolevel" needs a horizon that is level by default yet can be
//! rolled onto any wall of the world grid on purpose (a held Q / E), without a frame jump.
//!
//! How: the spring is stepped exactly (`x(t) = target + (a + b t) e^(-omega t)`, stable for any
//! dt); the flip is DIRECTIONAL: only grid planes ahead of the view in the held key's roll
//! direction qualify, so the lock never flips back against the key. The orientation is
//! continuous across a flip: only the reference plane changes and the roll is re-measured. The
//! requested axis is kept as `home`, so jumps of the view (`reseat`) return to the user's horizon.
//!
//! Uses: `crate::camera_core::settings::InertiaSettings` (`lock_rate`, `lock_step`, `lock_flip`,
//! `lock_hold`). Used by: `super::space` only (crate-private; `SpaceFlight` exposes the lock
//! through `set_horizon_lock` / `horizon_lock`).

use glam::{Mat3, Quat, Vec3};

use crate::camera_core::settings::InertiaSettings;

/// Spring natural frequency (1/s) floor while a roll key is held: a responsive roll drive
/// toward `lock_hold`, so crossing `lock_flip` to the next plane feels effortless even under a
/// soft `lock_rate` (a 2/s spring took seconds).
const HELD_OMEGA_MIN: f32 = 4.5;

/// Spring natural frequency (1/s) cap while a roll key is held, so a stiff `lock_rate`
/// does not snap the view onto the next plane.
const HELD_OMEGA_MAX: f32 = 6.0;

/// Spring natural frequency (1/s) floor with the key released (the autolevel return), so a
/// zero `lock_rate` still settles.
const RELEASED_OMEGA_MIN: f32 = 0.1;

/// A flip candidate must lie at least this far (radians) ahead of the view in the key's roll
/// direction, so a plane the view already sits on is never re-taken.
const FLIP_AHEAD_MIN: f32 = 0.1;

/// A flip candidate's roll must differ from the current spring roll by more than this
/// (radians), so the flip never re-selects the current plane through numeric noise.
const FLIP_MIN_JUMP: f32 = 0.25;

/// Below this roll and roll rate (radians, rad/s; 0.06 degrees) the spring is settled: no
/// visible tilt remains.
const SETTLED_EPS: f32 = 1e-3;

/// Squared length of the axis part perpendicular to the view forward below which the view looks
/// along the axis (within ~0.06 degrees) and no level plane exists.
const LEVEL_MIN_SQ: f32 = 1e-6;

/// Horizon lock of [`super::SpaceFlight`]: yaw turns about `up` and the view right axis is kept
/// in the lock plane. The lock always sits on a plane of an orthonormal **grid** (the world
/// planes XZ / XY / YZ for a world-axis lock): `up` is one of the six grid axes.
///
/// Roll is a damped spring toward the lock plane: the roll key adds torque against it, so a
/// short press tilts and springs back level, while a held key passes
/// `InertiaSettings::lock_flip` and moves the lock to the next grid plane in the roll direction,
/// where the spring settles the camera level.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct HorizonLock {
    /// The locked up axis (unit), one of the `±grid` columns; flips move it plane by plane.
    pub up: Vec3,
    /// The requested axis (unit, the grid's Y column): the user's horizon. Flips never change
    /// it; `reseat` returns to it when the view allows.
    home: Vec3,
    /// The plane grid (orthonormal columns).
    grid: Mat3,
    /// Roll of the view away from the lock plane (radians).
    pub(super) roll: f32,
    /// Roll rate of the spring (rad/s).
    pub(super) rate: f32,
}

impl HorizonLock {
    /// Lock to the grid plane nearest to the current roll. The grid is the world grid turned so
    /// that its Y axis is `up` (the world planes themselves for a world axis). None for a zero
    /// axis.
    pub(super) fn nearest(orientation: Quat, up: Vec3) -> Option<Self> {
        let up = up.normalize_or_zero();
        if up == Vec3::ZERO {
            return None;
        }
        let grid = Mat3::from_quat(Quat::from_rotation_arc(Vec3::Y, up));
        let mut lock = Self {
            up,
            home: up,
            grid,
            roll: 0.0,
            rate: 0.0,
        };
        (lock.up, lock.roll) = lock.nearest_plane(orientation);
        Some(lock)
    }

    /// The requested axis (unit): the user's horizon, unchanged by flips.
    pub(super) fn home(&self) -> Vec3 {
        self.home
    }

    /// The rotation `G` taking the world grid onto the lock's grid (world Y onto `home`): the
    /// same rotation the grid was built from. Identity for a +Y lock.
    pub(super) fn grid_rotation(&self) -> Quat {
        Quat::from_rotation_arc(Vec3::Y, self.home)
    }

    /// The grid plane nearest to `orientation` (smallest roll) and that roll. At most two of the
    /// six grid axes can be along the view, so a plane is always found; the fallback (never
    /// reached) keeps `up` with no roll.
    fn nearest_plane(&self, orientation: Quat) -> (Vec3, f32) {
        self.candidates()
            .into_iter()
            .filter_map(|axis| Some((axis, Self::roll_from(axis, orientation)?)))
            .min_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
            .unwrap_or((self.up, 0.0))
    }

    /// Re-seat the lock after the orientation jumps (a pose restore, a `View` snap, the end of
    /// an alignment) so the next `update_dynamics` keeps the view exactly as it was put, on the
    /// user's horizon whenever the view allows; spring rate zeroed. First match wins:
    ///
    /// - (a) the view is level to `home`: back to `home` (a lock flipped onto a wall returns to
    ///   the user's horizon after a level snap);
    /// - (b) the view looks along the CURRENT lock axis: keep it with no roll (no plane of it is
    ///   level-able, `frame` leaves the view untouched, the spring has nothing to settle; a
    ///   Front-then-Top snap under a Y lock stays on Y although the carried up makes a wall
    ///   plane level too);
    /// - (c) the view looks along `home`: `home` with no roll, as in (b);
    /// - (d) another grid plane the view is level to (a level wall view stays put);
    /// - (e) otherwise `home` with the view's roll from it: `home` can always be levelled to
    ///   here (the view is not along it, (c)), so the spring returns the view to the user's
    ///   horizon. A view along another grid axis, rolled, levels back to `home` instead of
    ///   locking onto that axis (where the tilt would never be corrected and yaw would turn
    ///   about the wrong axis).
    ///
    /// Why (b) / (c) precede (e): measured against `home` (or a wall), the snapped heading of a
    /// view along the lock axis would read as a roll (up to 45 degrees), and the spring would
    /// roll the snapped view about its forward. `nearest` (a fresh lock request) differs on
    /// purpose: it levels the view to the nearest plane, so looking along the requested axis it
    /// takes a side plane.
    pub(super) fn reseat(&mut self, orientation: Quat) {
        let forward = (orientation * -Vec3::Z).normalize_or_zero();
        // None exactly when the view looks along `home` (no level plane of it).
        let home_roll = Self::roll_from(self.home, orientation);
        (self.up, self.roll) = match home_roll {
            Some(roll) if roll.abs() < SETTLED_EPS => (self.home, roll),
            _ if Self::level(self.up, forward).is_none() => (self.up, 0.0),
            None => (self.home, 0.0),
            Some(roll) => match self.nearest_plane(orientation) {
                (plane, plane_roll) if plane_roll.abs() < SETTLED_EPS => (plane, plane_roll),
                _ => (self.home, roll),
            },
        };
        self.rate = 0.0;
    }

    /// The six grid axes, each the normal of one oriented grid plane.
    fn candidates(&self) -> [Vec3; 6] {
        let [x, y, z] = [self.grid.x_axis, self.grid.y_axis, self.grid.z_axis];
        [y, -y, x, -x, z, -z]
    }

    /// `axis` made perpendicular to `forward`: the up of a view level to that plane. None when
    /// the view looks along the axis (no level plane exists).
    pub(super) fn level(axis: Vec3, forward: Vec3) -> Option<Vec3> {
        let level = axis - forward * forward.dot(axis);
        (level.length_squared() > LEVEL_MIN_SQ).then(|| level.normalize())
    }

    /// The roll of `orientation` away from the plane of `axis`, signed about the view forward
    /// (the convention of `frame`: up = R(forward, roll) * level). None when the view looks
    /// along `axis`.
    pub(super) fn roll_from(axis: Vec3, orientation: Quat) -> Option<f32> {
        let forward = (orientation * -Vec3::Z).normalize_or_zero();
        let level = Self::level(axis, forward)?;
        let view_up = orientation * Vec3::Y;
        Some(forward.dot(level.cross(view_up)).atan2(level.dot(view_up)))
    }

    /// The roll spring has not settled (see `step`).
    pub(super) fn moving(&self) -> bool {
        self.roll != 0.0 || self.rate != 0.0
    }

    /// Advance the roll spring by `dt`: a critically damped spring toward the plane plus the
    /// held roll key (`input`) as torque sized so a full key holds at `lock_hold`. Released, the
    /// natural frequency is `lock_rate` (1/s, the spring's own knob: the lock is not a coast);
    /// held, it is clamped to `HELD_OMEGA_MIN..=HELD_OMEGA_MAX`. Past `lock_flip` IN THE KEY'S
    /// DIRECTION the lock moves to the next grid plane that way (`lock_step <= 0` disables
    /// flips); a key held on keeps ratcheting plane by plane. Returns `true` while the roll is
    /// still moving.
    pub(super) fn step(
        &mut self,
        orientation: Quat,
        input: f32,
        inertia: &InertiaSettings,
        dt: f32,
    ) -> bool {
        // `crate::camera_core::num::clamp` / `max` never panic or propagate a NaN rate (host setting).
        let omega = if input != 0.0 {
            crate::camera_core::num::clamp(inertia.lock_rate, HELD_OMEGA_MIN, HELD_OMEGA_MAX)
        } else {
            inertia.lock_rate.max(RELEASED_OMEGA_MIN)
        };
        // Exact step of the critically damped oscillator toward the key target.
        let target = input * inertia.lock_hold;
        let a = self.roll - target;
        let b = self.rate + omega * a;
        let decay = (-omega * dt).exp();
        self.roll = target + (a + b * dt) * decay;
        self.rate = (b - omega * (a + b * dt)) * decay;
        if inertia.lock_step > 0.0 && input != 0.0 && self.roll * input.signum() > inertia.lock_flip
        {
            self.flip(orientation, input.signum(), inertia.lock_step);
        }
        let settled =
            input == 0.0 && self.roll.abs() < SETTLED_EPS && self.rate.abs() < SETTLED_EPS;
        if settled {
            self.roll = 0.0;
            self.rate = 0.0;
        }
        !settled
    }

    /// Move the lock to the grid plane one `step` away in the `sign` roll direction: of the grid
    /// planes ahead of the current view in that direction (`roll * sign < -FLIP_AHEAD_MIN`) and
    /// not the current one, the one whose roll from the view is nearest to `roll - step`. The
    /// view itself does not move (continuous); the roll is re-measured from the new plane.
    fn flip(&mut self, orientation: Quat, sign: f32, step: f32) {
        // The view as `frame` will build it with the current spring roll.
        let view = self.frame(orientation);
        let goal = self.roll - step.copysign(sign);
        let next = self
            .candidates()
            .into_iter()
            .filter(|axis| axis.dot(self.up) < 0.5)
            .filter_map(|axis| Some((axis, Self::roll_from(axis, view)?)))
            .filter(|&(_, roll)| {
                roll * sign < -FLIP_AHEAD_MIN && (roll - self.roll).abs() > FLIP_MIN_JUMP
            })
            .min_by(|a, b| (a.1 - goal).abs().total_cmp(&(b.1 - goal).abs()));
        if let Some((axis, roll)) = next {
            self.up = axis;
            self.roll = roll;
        }
    }

    /// The view `orientation` rebuilt level to the lock plane, then rolled by the spring.
    /// Unchanged when the view looks along the lock axis.
    pub(super) fn frame(&self, orientation: Quat) -> Quat {
        let forward = (orientation * -Vec3::Z).normalize_or_zero();
        let Some(level) = Self::level(self.up, forward) else {
            return orientation;
        };
        let up = Quat::from_axis_angle(forward, self.roll) * level;
        let right = forward.cross(up).normalize();
        let up = right.cross(forward);
        Quat::from_mat3(&Mat3::from_cols(right, up, -forward)).normalize()
    }
}
