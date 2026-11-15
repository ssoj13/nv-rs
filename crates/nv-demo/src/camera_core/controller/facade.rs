//! [`CameraController`]: the one enum facade over the four camera backends, and [`CameraKind`].
//!
//! What: dispatches the uniform backend contract (`apply_intent`, `update_dynamics`,
//! `has_motion`, `is_coasting`, `stop_coasts`, `pose`, `set_pose`, framing, tuning) and the HUD accessors (horizon lock,
//! velocity, speed scale, cruise speed, look lead, thrust) to the active backend, and switches
//! navigation modes pose-preservingly (`set_mode` / `mode`, the table is `crate::camera_core::mode::NavMode`),
//! applies the user's preferences in one call (`apply_prefs`) and holds alignments (`set_align`).
//!
//! Why: hosts drive whatever style is selected and read one [`CameraPose`] back, never matching
//! on the variant; switching style keeps view continuity ([`CameraController::from_pose`]).
//!
//! How: every method is a `match` onto the backend's own method; accessors that only make sense
//! for some backends return a neutral value (or `None` / `false`) for the others.
//!
//! Uses: `super::{orbit, quat, fps, space}`. Used by: the crate root re-exports, the `input`
//! layer and host viewports.

use glam::{Vec2, Vec3};

use super::{FpsFly, HoudiniOrbit, QuatOrbit, SpaceFlight};
use crate::camera_core::coast::CoastSettings;
use crate::camera_core::intent::{CameraIntent, OrbitSnapMode};
use crate::camera_core::mode::NavMode;
use crate::camera_core::orientation::OrientationMode;
use crate::camera_core::pose::CameraPose;
use crate::camera_core::prefs::NavPrefs;
use crate::camera_core::projection::PerspectiveSettings;
use crate::camera_core::rig::OrbitPose;
use crate::camera_core::settings::{CameraNavigationSettings, InertiaSettings, MouseSteer};
use crate::camera_core::viewport::ViewportSize;

/// One of four pluggable camera navigation backends.
///
/// See the [module docs](super) for the per-variant behaviour table. The inherent methods
/// dispatch to the active backend, so callers never match on the variant directly.
#[derive(Debug, Clone)]
pub enum CameraController {
    /// Classic euler orbit (wraps [`crate::camera_core::rig::OrbitRig`]), see [`HoudiniOrbit`].
    Houdini(HoudiniOrbit),
    /// Gimbal-free quaternion orbit, see [`QuatOrbit`].
    Quat(QuatOrbit),
    /// FPS fly (mouse-look + thrust, no roll), see [`FpsFly`].
    Fps(FpsFly),
    /// 6-DoF spaceship (thrust + roll + speed scale + drift, optional horizon lock), see
    /// [`SpaceFlight`].
    Space(SpaceFlight),
}

impl Default for CameraController {
    /// The Houdini orbit, the default viewport behaviour.
    fn default() -> Self {
        Self::Houdini(HoudiniOrbit::default())
    }
}

impl CameraController {
    /// Apply one [`CameraIntent`] to the active backend.
    ///
    /// `dt` is the frame delta; the backends integrate held axes in `update_dynamics`, so it is
    /// unused here but part of the uniform signature hosts call. `viewport` carries the real
    /// pixel size: cursor-anchored zoom and `FrameBounds` need the true aspect.
    pub fn apply_intent(&mut self, intent: CameraIntent, _dt: f32, viewport: ViewportSize) {
        match self {
            CameraController::Houdini(b) => b.apply_intent(intent, viewport),
            CameraController::Quat(b) => b.apply_intent(intent, viewport),
            CameraController::Fps(b) => b.apply_intent(intent, viewport),
            CameraController::Space(b) => b.apply_intent(intent, viewport),
        }
    }

    /// Advance inertia / coast / held axes by `dt`. Returns `true` while still moving (the host
    /// should keep requesting frames).
    pub fn update_dynamics(&mut self, dt: f32) -> bool {
        match self {
            CameraController::Houdini(b) => b.rig.update_dynamics(dt),
            CameraController::Quat(b) => b.update_dynamics(dt),
            CameraController::Fps(b) => b.update_dynamics(dt),
            CameraController::Space(b) => b.update_dynamics(dt),
        }
    }

    /// True while the backend has residual motion (coast, a view animation, a held axis, a
    /// queued look, an alignment or an unsettled lock spring).
    pub fn has_motion(&self) -> bool {
        match self {
            CameraController::Houdini(b) => b.rig.has_inertia() || b.rig.is_animating(),
            CameraController::Quat(b) => b.has_motion(),
            CameraController::Fps(b) => b.has_motion(),
            CameraController::Space(b) => b.has_motion(),
        }
    }

    /// True while a release coast runs (an orbit / pan / zoom / dolly-zoom / look / thrust glide
    /// after the input ended); a view transition or a held axis alone is not a coast. A coast
    /// ends exactly at its time constant, so this is the "glide still running" flag hosts and
    /// `crate::camera_core::CameraDriver` use (the hold-to-fly flight lasts while it is true).
    pub fn is_coasting(&self) -> bool {
        match self {
            CameraController::Houdini(b) => b.rig.has_inertia(),
            CameraController::Quat(b) => b.coast.is_active(),
            CameraController::Fps(b) => b.is_coasting(),
            CameraController::Space(b) => b.is_coasting(),
        }
    }

    /// Stop every running coast of the active backend at once (the camera freezes where it is):
    /// what a press on the view does. Held axes, view transitions and the pose are untouched.
    pub fn stop_coasts(&mut self) {
        match self {
            CameraController::Houdini(b) => b.rig.stop_inertia(),
            CameraController::Quat(b) => b.coast.stop(),
            CameraController::Fps(b) => b.stop_coasts(),
            CameraController::Space(b) => b.stop_coasts(),
        }
    }

    /// Drop every held input of the active backend (thrust / roll / steer axes, the speed
    /// modifier, a held alignment) as if all keys and the pointer were released; a no-op on the
    /// orbit backends, which hold no input between frames.
    ///
    /// THE focus-loss call: a host that stops emitting the per-frame held intents (the window
    /// lost focus, a typing guard took the keys, a modal opened) calls this once, otherwise the
    /// latched axes keep `has_motion` true and the camera moving forever. A held thrust ends in
    /// its coast, which runs to rest within the coast time.
    pub fn release_inputs(&mut self) {
        match self {
            CameraController::Houdini(_) | CameraController::Quat(_) => {}
            CameraController::Fps(b) => b.release_inputs(),
            CameraController::Space(b) => b.release_inputs(),
        }
    }

    /// Current pose (the uniform output for render code).
    pub fn pose(&self) -> CameraPose {
        match self {
            CameraController::Houdini(b) => b.pose(),
            CameraController::Quat(b) => b.pose(),
            CameraController::Fps(b) => b.pose(),
            CameraController::Space(b) => b.pose(),
        }
    }

    /// Frame the AABB `min..max` in a `width x height` viewport with the active backend.
    pub fn frame_bounds(&mut self, min: Vec3, max: Vec3, width: u32, height: u32, margin: f32) {
        match self {
            CameraController::Houdini(b) => b.rig.frame_bounds(min, max, width, height, margin),
            CameraController::Quat(b) => b.frame_aabb(min, max, width, height, margin),
            CameraController::Fps(b) => b.frame_aabb(min, max, width, height, margin),
            CameraController::Space(b) => b.frame_aabb(min, max, width, height, margin),
        }
    }

    /// Apply live fly tuning ([`InertiaSettings`]) to the active backend, keeping the fields the
    /// navigation mode owns: `mouse_steer` stays the backend's (set by
    /// [`CameraController::set_mode`] from [`NavMode::mouse_steer`]), so re-applying tuning every
    /// frame never flips the steering or [`CameraController::mode`]. The horizon lock is not
    /// part of the tuning and is untouched.
    ///
    /// Only the fly backends hold fly tuning (cruise speed, thrust response, roll rate, lock
    /// rate, coast, steering). The orbit backends coast with [`CameraNavigationSettings::coast`]
    /// (see [`CameraController::apply_navigation`]), so this is a deliberate no-op on both of
    /// them.
    pub fn apply_inertia(&mut self, inertia: InertiaSettings) {
        let keep_mode_fields = |current: &InertiaSettings| InertiaSettings {
            mouse_steer: current.mouse_steer,
            ..inertia
        };
        match self {
            // Orbit coasts are tuned through `apply_navigation`; no fly tuning to store.
            CameraController::Houdini(_) | CameraController::Quat(_) => {}
            CameraController::Fps(b) => b.inertia = keep_mode_fields(&b.inertia),
            CameraController::Space(b) => b.inertia = keep_mode_fields(&b.inertia),
        }
    }

    /// Apply the user's navigation preferences in one call, the way every host should: the
    /// mode ([`CameraController::set_mode`] of `prefs.mode`: backend, orientation policy,
    /// steering, horizon lock), the fly tuning ([`NavPrefs::inertia`] for `scene_radius`), the
    /// orbit tuning ([`NavPrefs::navigation`] for the current fov and `viewport_height`; both
    /// carry `prefs.feel`) and the align time constant (`OrbitRig::align_seconds` of the
    /// turntables; a fly host passes `prefs.align_seconds` in [`CameraController::set_align`]).
    /// The preferences are sanitised first ([`NavPrefs::sanitized`]). Idempotent (the backend is
    /// rebuilt only when the mode changes backend), but a host calls it only when the
    /// preferences, the mode, the scene radius, the viewport height or the FOV changed:
    /// `crate::camera_core::CameraDriver` does exactly that and is the recommended host API.
    pub fn apply_prefs(&mut self, prefs: &NavPrefs, scene_radius: f32, viewport_height: f32) {
        let p = prefs.sanitized();
        self.set_mode(p.mode);
        self.apply_inertia(p.inertia(scene_radius));
        let fov_y = self.projection().fov_y;
        self.apply_navigation(p.navigation(fov_y, viewport_height));
        if let CameraController::Houdini(b) = self {
            b.rig.align_seconds = p.align_seconds;
        }
    }

    /// Hold (`Some((snap mode, time constant in seconds))`) or release (`None`) a world
    /// alignment of the spaceflight backend ([`SpaceFlight::set_align`]); returns `true` there.
    /// The other backends hold no alignment: a documented no-op returning `false` (the
    /// turntables align through the `OrbitSnap` intent, the FPS rig and the trackball do not
    /// align). A non-finite or negative time constant is ignored (nothing changes, `false`).
    pub fn set_align(&mut self, align: Option<(OrbitSnapMode, f32)>) -> bool {
        if align.is_some_and(|(_, secs)| !(secs.is_finite() && secs >= 0.0)) {
            return false;
        }
        match self {
            CameraController::Space(b) => {
                b.set_align(align);
                true
            }
            CameraController::Houdini(_) | CameraController::Quat(_) | CameraController::Fps(_) => {
                false
            }
        }
    }

    /// Apply orbit navigation tuning ([`CameraNavigationSettings`]) to the active backend.
    ///
    /// Only the orbit backends consume it; the fly backends drive motion from
    /// [`InertiaSettings`], so this is a no-op there.
    pub fn apply_navigation(&mut self, nav: CameraNavigationSettings) {
        match self {
            CameraController::Houdini(b) => b.rig.apply_navigation(nav),
            CameraController::Quat(b) => b.navigation = nav,
            CameraController::Fps(_) | CameraController::Space(_) => {}
        }
    }

    /// Replace the active backend's perspective projection (fov / near / far / distance range).
    ///
    /// A host that recomputes an adaptive projection every frame (e.g. from the live scene
    /// bounds) pushes it here, so the emitted pose carries the live near / far and every
    /// downstream `view_proj` / `viewport_ray` uses the same projection; it also re-targets the
    /// zoom clamp and the framing distance range.
    ///
    /// This WRITES `fov_y` too, and the controller is the FOV's one owner (a `DollyZoom` changes
    /// it): a host that only wants new clips / distances copies the live FOV first
    /// (`PerspectiveSettings { fov_y: ctrl.projection().fov_y, ..mine }`). `crate::camera_core::CameraDriver`
    /// does that and owns the lens ease's writes.
    pub fn set_projection(&mut self, projection: PerspectiveSettings) {
        match self {
            CameraController::Houdini(b) => b.rig.projection = projection,
            CameraController::Quat(b) => b.projection = projection,
            CameraController::Fps(b) => b.projection = projection,
            CameraController::Space(b) => b.projection = projection,
        }
    }

    /// Move the active backend to `pose`, keeping the backend kind and ALL its tuning.
    ///
    /// Only the pose changes: lens, inertia / navigation tuning, the horizon lock, the
    /// alignment and the speed scale survive; momentum, held axes and animations stop. A style
    /// switch builds a new backend with [`CameraController::from_pose`] instead.
    pub fn set_pose(&mut self, pose: CameraPose) {
        match self {
            CameraController::Houdini(b) => b.set_pose(pose),
            CameraController::Quat(b) => b.set_pose(pose),
            CameraController::Fps(b) => b.set_pose(pose),
            CameraController::Space(b) => b.set_pose(pose),
        }
    }

    /// Build a controller of `kind` seeded from `pose` (view continuity across a backend
    /// switch); the new backend takes the pose's lens.
    pub fn from_pose(kind: CameraKind, pose: CameraPose) -> Self {
        match kind {
            CameraKind::Houdini => CameraController::Houdini(HoudiniOrbit::from_pose(pose)),
            CameraKind::Quat => CameraController::Quat(QuatOrbit::from_pose(pose)),
            CameraKind::Fps => CameraController::Fps(FpsFly::from_pose(pose)),
            CameraKind::Space => CameraController::Space(SpaceFlight::from_pose(pose)),
        }
    }

    /// The active backend's lens and zoom distance range.
    pub fn projection(&self) -> PerspectiveSettings {
        match self {
            CameraController::Houdini(b) => b.rig.projection,
            CameraController::Quat(b) => b.projection,
            CameraController::Fps(b) => b.projection,
            CameraController::Space(b) => b.projection,
        }
    }

    /// The FOVs (radians, `(min, max)`) the controller can reach without breaking a dolly zoom:
    /// on the orbit backends the one viewport FOV range intersected with the FOVs the lens
    /// distance range allows at the current subject size (constant along a dolly zoom); a
    /// degenerate orbit state reports its current FOV only. The fly backends have no pivot to
    /// keep: the whole range [`crate::camera_core::lens::FOV_MIN`]..=[`crate::camera_core::lens::FOV_MAX`]. A lens ease
    /// clamps its target into this (`crate::camera_core::LensEase::set_reach`), so an unreachable target
    /// (e.g. the 2 degree flat lens beyond the distance range) counts as reached.
    pub fn fov_range(&self) -> (f32, f32) {
        use crate::camera_core::lens::{FOV_MAX, FOV_MIN};
        let orbit = |distance: f32, projection: PerspectiveSettings| {
            super::dolly_fov_range(distance, projection.fov_y, projection).unwrap_or_else(|| {
                let fov = crate::camera_core::num::clamp(projection.fov_y, FOV_MIN, FOV_MAX);
                (fov, fov)
            })
        };
        match self {
            CameraController::Houdini(b) => orbit(b.rig.pose.distance, b.rig.projection),
            CameraController::Quat(b) => orbit(b.distance, b.projection),
            CameraController::Fps(_) | CameraController::Space(_) => (FOV_MIN, FOV_MAX),
        }
    }

    /// The orbit pivot `(target, distance)` of the orbit backends; `None` on the fly backends.
    pub fn orbit_pivot(&self) -> Option<(Vec3, f32)> {
        self.orbit_carry()
            .map(|(target, distance, _)| (target, distance))
    }

    /// Re-place the orbit pivot `distance` (clamped to the lens range) ahead of the CURRENT eye
    /// along the current forward, keeping the eye and the orientation (a direct edit: coasts and
    /// a view transition stop). Returns `false` (nothing changes) on the fly backends or for a
    /// non-finite / non-positive distance. Used to bring back the remembered pivot after a
    /// hold-to-fly flight (`crate::camera_core::CameraDriver`).
    pub fn set_orbit_distance(&mut self, distance: f32) -> bool {
        if !(distance.is_finite() && distance > 0.0) {
            return false;
        }
        let pose = self.pose();
        match self {
            CameraController::Houdini(b) => {
                let d = b.rig.projection.clamp_distance(distance);
                let target = pose.eye + pose.forward() * d;
                b.rig.pose = OrbitPose::from_view(target, pose.orientation, d);
                b.rig.take_over();
                b.rig.stop_inertia();
                true
            }
            CameraController::Quat(b) => {
                b.distance = distance;
                b.set_pose(pose);
                b.coast.stop();
                true
            }
            CameraController::Fps(_) | CameraController::Space(_) => false,
        }
    }

    /// The active backend as a [`CameraKind`].
    pub fn camera_kind(&self) -> CameraKind {
        match self {
            CameraController::Houdini(_) => CameraKind::Houdini,
            CameraController::Quat(_) => CameraKind::Quat,
            CameraController::Fps(_) => CameraKind::Fps,
            CameraController::Space(_) => CameraKind::Space,
        }
    }

    /// The orbit state an orbit <-> orbit switch carries: pivot, distance and the navigation
    /// tuning (including its coast settings); `None` on the fly backends.
    fn orbit_carry(&self) -> Option<(Vec3, f32, CameraNavigationSettings)> {
        match self {
            CameraController::Houdini(b) => {
                Some((b.rig.pose.target, b.rig.pose.distance, b.rig.navigation))
            }
            CameraController::Quat(b) => Some((b.target, b.distance, b.navigation)),
            CameraController::Fps(_) | CameraController::Space(_) => None,
        }
    }

    /// The coast settings of the active backend (the orbit navigation's or the fly tuning's):
    /// the one tuning that translates between an orbit and a fly backend.
    fn coast_settings(&self) -> CoastSettings {
        match self {
            CameraController::Houdini(b) => b.rig.navigation.coast,
            CameraController::Quat(b) => b.navigation.coast,
            CameraController::Fps(b) => b.inertia.coast,
            CameraController::Space(b) => b.inertia.coast,
        }
    }

    /// The fly state a fly <-> fly switch carries: the fly tuning and the held speed scale;
    /// `None` on the orbit backends.
    fn fly_carry(&self) -> Option<(InertiaSettings, f32)> {
        match self {
            CameraController::Fps(b) => Some((b.inertia, b.speed_scale())),
            CameraController::Space(b) => Some((b.inertia, b.speed_scale())),
            CameraController::Houdini(_) | CameraController::Quat(_) => None,
        }
    }

    /// Switch to the navigation `mode` keeping the view (see [`NavMode::spec`] for what each
    /// mode sets; the mode is THE owner of the orientation policy, the steering and the lock).
    ///
    /// When the mode runs on another backend, a new one is built from the current pose
    /// ([`CameraController::from_pose`]) and given the current projection back in full (fov,
    /// clips AND the distance range, which a pose does not carry), then:
    /// - orbit <-> orbit (turntable <-> trackball) carries the pivot (target + distance, so the
    ///   eye stays put and the orbit centre does not jump to the default distance ahead) and the
    ///   navigation tuning (incl. the coast settings);
    /// - fly <-> fly (FPS <-> space) carries the fly tuning (`InertiaSettings`) and the held
    ///   speed scale;
    /// - orbit <-> fly carries only the pose, the lens and the coast settings (the other tunings
    ///   do not translate); the new backend starts with its own defaults until the host
    ///   re-applies its preferences
    ///   ([`CameraController::apply_prefs`]). A fly backend has no pivot, so an orbit -> fly ->
    ///   orbit round trip through `set_mode` alone puts the pivot the default distance ahead of
    ///   the eye: remember [`CameraController::orbit_pivot`] and restore it with
    ///   [`CameraController::set_orbit_distance`], as `crate::camera_core::CameraDriver` does for hold-to-fly.
    ///
    /// A roll is dropped by the roll-free backends (turntables, FPS). Within one backend nothing
    /// is rebuilt and the pose is untouched: only the orientation policy, the mouse steering or
    /// the horizon lock changes (an engaged lock levels the view smoothly from where it is) and
    /// every tuning survives.
    pub fn set_mode(&mut self, mode: NavMode) {
        let spec = mode.spec();
        if self.camera_kind() != spec.kind {
            let (pose, projection) = (self.pose(), self.projection());
            let (orbit, fly) = (self.orbit_carry(), self.fly_carry());
            let coast = self.coast_settings();
            let mut next = Self::from_pose(spec.kind, pose);
            next.set_projection(projection);
            next.set_pose(pose);
            // Running coasts end with the old backend; only the coast TUNING survives a switch.
            match &mut next {
                CameraController::Houdini(b) => {
                    if let Some((target, distance, nav)) = orbit {
                        let distance = b.rig.projection.clamp_distance(distance);
                        b.rig.pose = OrbitPose::from_view(target, pose.orientation, distance);
                        b.rig.apply_navigation(nav);
                    } else {
                        b.rig.apply_navigation(CameraNavigationSettings {
                            coast,
                            ..b.rig.navigation
                        });
                    }
                }
                CameraController::Quat(b) => {
                    if let Some((target, distance, nav)) = orbit {
                        b.target = target;
                        b.distance = b.projection.clamp_distance(distance);
                        b.navigation = nav;
                    } else {
                        b.navigation.coast = coast;
                    }
                }
                CameraController::Fps(b) => {
                    if let Some((inertia, scale)) = fly {
                        b.inertia = inertia;
                        b.carry_speed_scale(scale);
                    } else {
                        b.inertia.coast = coast;
                    }
                }
                CameraController::Space(b) => {
                    if let Some((inertia, scale)) = fly {
                        b.inertia = inertia;
                        b.carry_speed_scale(scale);
                    } else {
                        b.inertia.coast = coast;
                    }
                }
            }
            *self = next;
        }
        match self {
            CameraController::Houdini(b) => {
                if let Some(orientation) = spec.orientation {
                    b.rig.orientation_mode = orientation;
                }
            }
            CameraController::Quat(_) => {}
            CameraController::Fps(b) => b.inertia.mouse_steer = spec.mouse_steer,
            CameraController::Space(b) => {
                b.inertia.mouse_steer = spec.mouse_steer;
                // The mode table only holds `None` or the non-zero world up, both of which
                // always take effect.
                let applied = b.set_horizon_lock(spec.horizon_lock);
                debug_assert!(applied, "{mode:?}: the mode's horizon lock was refused");
            }
        }
    }

    /// The navigation mode the controller is in, read back from the backend: `None` only for a
    /// Houdini rig on the legacy [`crate::camera_core::OrientationMode::Quaternion`] policy, which no mode
    /// uses. A `SpaceFlight` reads back from its two mode cells, the horizon lock (whatever its
    /// axis: after a flip the lock rides another grid plane) and the steering: locked + deflection
    /// is [`NavMode::SpaceSimLocked`], locked + relative [`NavMode::QuatFlightLocked`], unlocked +
    /// deflection [`NavMode::SpaceSim`], unlocked + relative [`NavMode::QuatFlight`]. An `FpsFly`
    /// is [`NavMode::FpsFly`] with either steering.
    pub fn mode(&self) -> Option<NavMode> {
        match self {
            CameraController::Houdini(b) => match b.rig.orientation_mode {
                OrientationMode::HorizontalPlaneEuler => Some(NavMode::Turntable),
                OrientationMode::Euler => Some(NavMode::EulerTurntable),
                OrientationMode::Quaternion => None,
            },
            CameraController::Quat(_) => Some(NavMode::Trackball),
            CameraController::Fps(_) => Some(NavMode::FpsFly),
            CameraController::Space(b) => {
                Some(match (b.horizon_lock().is_some(), b.inertia.mouse_steer) {
                    (true, MouseSteer::Deflection) => NavMode::SpaceSimLocked,
                    (true, MouseSteer::Relative) => NavMode::QuatFlightLocked,
                    (false, MouseSteer::Deflection) => NavMode::SpaceSim,
                    (false, MouseSteer::Relative) => NavMode::QuatFlight,
                })
            }
        }
    }

    /// Stable string name of the active backend (UI / logging).
    pub fn kind(&self) -> &'static str {
        match self {
            CameraController::Houdini(_) => "houdini",
            CameraController::Quat(_) => "quat",
            CameraController::Fps(_) => "fps",
            CameraController::Space(_) => "space",
        }
    }

    /// True for the free-flight backends (FPS / Spaceship), false for the orbit backends
    /// (Houdini / Quaternion); gates fly-only UI such as a flight HUD.
    pub fn is_fly(&self) -> bool {
        matches!(self, CameraController::Fps(_) | CameraController::Space(_))
    }

    /// Lock the spaceflight horizon to `up` (e.g. `Some(Vec3::Y)` to autolevel) or release it
    /// (`None`: free 6-DoF). Returns whether the request took effect: `false` for a zero axis.
    /// Only [`SpaceFlight`] has a horizon lock, so on every other backend a lock request
    /// returns `false` and a release (`None`) returns `true` (there is nothing locked).
    pub fn set_horizon_lock(&mut self, up: Option<Vec3>) -> bool {
        match self {
            CameraController::Space(b) => b.set_horizon_lock(up),
            CameraController::Houdini(_) | CameraController::Quat(_) | CameraController::Fps(_) => {
                up.is_none()
            }
        }
    }

    /// The active horizon-lock axis of the spaceflight backend; `None` in free flight and on
    /// every other backend.
    pub fn horizon_lock(&self) -> Option<Vec3> {
        match self {
            CameraController::Space(b) => b.horizon_lock(),
            CameraController::Houdini(_) | CameraController::Quat(_) | CameraController::Fps(_) => {
                None
            }
        }
    }

    /// World-space linear velocity (units/s) of a fly backend; zero for the orbit backends.
    pub fn linear_velocity(&self) -> Vec3 {
        match self {
            CameraController::Fps(b) => b.linear_velocity(),
            CameraController::Space(b) => b.linear_velocity(),
            CameraController::Houdini(_) | CameraController::Quat(_) => Vec3::ZERO,
        }
    }

    /// The held speed-scale multiplier of a fly backend (`SpeedScale`, e.g. the fast / slow
    /// modifiers); 1 for the orbit backends.
    pub fn speed_scale(&self) -> f32 {
        match self {
            CameraController::Fps(b) => b.speed_scale(),
            CameraController::Space(b) => b.speed_scale(),
            CameraController::Houdini(_) | CameraController::Quat(_) => 1.0,
        }
    }

    /// Nominal cruise speed (units/s, [`InertiaSettings::cruise_speed`]) of a fly backend at
    /// speed scale 1; `None` for the orbit backends (they do not fly).
    pub fn cruise_speed(&self) -> Option<f32> {
        match self {
            CameraController::Fps(b) => Some(b.inertia.cruise_speed),
            CameraController::Space(b) => Some(b.inertia.cruise_speed),
            CameraController::Houdini(_) | CameraController::Quat(_) => None,
        }
    }

    /// Screen-space look lead `(right, down)` in radians for a HUD reticle (see
    /// [`SpaceFlight::look_lead`], [`FpsFly::look_lead`]); zero for the orbit backends and at
    /// rest.
    pub fn look_lead(&self) -> Vec2 {
        match self {
            CameraController::Space(b) => b.look_lead(),
            CameraController::Fps(b) => b.look_lead(),
            CameraController::Houdini(_) | CameraController::Quat(_) => Vec2::ZERO,
        }
    }

    /// The held body-space thrust axes `(forward, right, up)` of a fly backend; zero for the
    /// orbit backends.
    pub fn thrust_input(&self) -> Vec3 {
        match self {
            CameraController::Fps(b) => b.thrust_input(),
            CameraController::Space(b) => b.thrust_input(),
            CameraController::Houdini(_) | CameraController::Quat(_) => Vec3::ZERO,
        }
    }
}

/// Backend selector for [`CameraController::from_pose`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum CameraKind {
    /// [`HoudiniOrbit`].
    #[default]
    Houdini,
    /// [`QuatOrbit`].
    Quat,
    /// [`FpsFly`].
    Fps,
    /// [`SpaceFlight`].
    Space,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera_core::rig::OrbitRig;

    #[test]
    fn set_pose_round_trips() {
        let pose = CameraPose::from_eye_forward_up(
            Vec3::new(5.0, 3.0, -2.0),
            Vec3::new(0.0, -0.2, -1.0),
            Vec3::Y,
            45.0_f32.to_radians(),
            0.1,
            500.0,
        );
        let mut ctrl = CameraController::Space(SpaceFlight::default());
        ctrl.set_pose(pose);
        let got = ctrl.pose();
        assert!((got.eye - pose.eye).length() < 1e-4, "eye not preserved");
        assert!(
            got.orientation.angle_between(pose.orientation) < 1e-3,
            "orientation not preserved"
        );

        // Fps round-trip preserves eye and (roll-free) heading.
        let mut fps = CameraController::Fps(FpsFly::default());
        fps.set_pose(pose);
        let g2 = fps.pose();
        assert!((g2.eye - pose.eye).length() < 1e-4, "fps eye not preserved");
        assert!(
            g2.forward().dot(pose.forward()) > 0.999,
            "fps forward not preserved"
        );
    }

    #[test]
    fn set_pose_preserves_horizon_lock_inertia_and_projection() {
        let custom_proj = PerspectiveSettings {
            fov_y: 60.0_f32.to_radians(),
            near: 0.25,
            far: 4000.0,
            distance_min: 2.0,
            distance_max: 2000.0,
        };
        let custom_inertia = InertiaSettings {
            cruise_speed: 999.0,
            ..InertiaSettings::default()
        };

        let mut space = SpaceFlight::default();
        assert!(space.set_horizon_lock(Some(Vec3::Y)));
        space.inertia = custom_inertia;
        space.projection = custom_proj;
        let mut ctrl = CameraController::Space(space);

        let pose = CameraPose::from_eye_forward_up(
            Vec3::new(10.0, 20.0, 30.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::Y,
            35.0_f32.to_radians(),
            0.05,
            800.0,
        );
        ctrl.set_pose(pose);
        assert_eq!(ctrl.horizon_lock(), Some(Vec3::Y));
        let CameraController::Space(b) = &ctrl else {
            panic!("expected Space variant");
        };
        assert_eq!(b.inertia.cruise_speed, 999.0);
        assert_eq!(b.projection, custom_proj);

        // The quaternion orbit's tuning is its navigation (pan, coast) and its lens.
        let mut quat = QuatOrbit {
            projection: custom_proj,
            ..Default::default()
        };
        quat.navigation.pan_sensitivity = 0.042;
        quat.navigation.coast.time = 1.7;
        let mut qctrl = CameraController::Quat(quat);
        qctrl.set_pose(pose);
        let CameraController::Quat(b) = &qctrl else {
            panic!("expected Quat variant");
        };
        assert_eq!(b.projection, custom_proj);
        assert!((b.navigation.pan_sensitivity - 0.042).abs() < 1e-6);
        assert_eq!(b.navigation.coast.time, 1.7);
        assert!((b.pose().eye - pose.eye).length() < 1e-4);
    }

    /// Restoring a pose into the Houdini backend keeps the rig tuning.
    #[test]
    fn houdini_set_pose_keeps_rig_tuning() {
        let mut rig = OrbitRig::from_pose(OrbitPose::default());
        rig.pitch_limit = 0.5;
        rig.navigation.coast.time = 1.5;
        rig.transition_speed = 2.0;
        let mut ctrl = CameraController::Houdini(HoudiniOrbit::new(rig));
        let pose = CameraPose::from_eye_forward_up(
            Vec3::new(4.0, 5.0, 6.0),
            Vec3::new(0.3, -0.4, -1.0),
            Vec3::Y,
            45.0_f32.to_radians(),
            0.1,
            500.0,
        );
        ctrl.set_pose(pose);
        let CameraController::Houdini(b) = &ctrl else {
            panic!("expected Houdini");
        };
        assert_eq!(b.rig.pitch_limit, 0.5);
        assert_eq!(b.rig.navigation.coast.time, 1.5);
        assert_eq!(b.rig.transition_speed, 2.0);
        assert!((ctrl.pose().eye - pose.eye).length() < 1e-3);
    }

    /// With `inertial_look`, held deflection steering runs through the smoothing
    /// cascade: it fills `look_lead`, keeps easing one frame past the recentre, then rests.
    #[test]
    fn space_mouse_steer_inertial_smooths_and_damps_to_rest() {
        let mut s = SpaceFlight::default();
        s.inertia.inertial_look = true;
        s.inertia.look_smoothing = 0.08;
        let mut ctrl = CameraController::Space(s);
        let o0 = ctrl.pose().orientation;
        let dt = 1.0 / 60.0;
        let vp = ViewportSize::new(1, 1);
        ctrl.apply_intent(
            CameraIntent::LookRate {
                yaw: 1.0,
                pitch: 0.25,
            },
            dt,
            vp,
        );
        for _ in 0..5 {
            assert!(ctrl.update_dynamics(dt));
        }
        let lead = ctrl.look_lead();
        assert!(
            lead.length() > 1e-3,
            "inertial LookRate must populate look_lead(), got {lead:?}"
        );
        assert!(
            o0.angle_between(ctrl.pose().orientation) > 1e-3,
            "inertial deflection steer did not turn the ship"
        );

        let o1 = ctrl.pose().orientation;
        ctrl.apply_intent(
            CameraIntent::LookRate {
                yaw: 0.0,
                pitch: 0.0,
            },
            dt,
            vp,
        );
        assert!(
            ctrl.update_dynamics(dt),
            "inertial LookRate should coast smoothly after recentre"
        );
        assert!(
            o1.angle_between(ctrl.pose().orientation) > 1e-4,
            "inertial LookRate should continue smoothing across the first post-release frame"
        );
        let mut frames = 0;
        while ctrl.update_dynamics(dt) {
            frames += 1;
            assert!(frames < 10_000, "inertial LookRate never damped to rest");
        }
        assert_eq!(ctrl.look_lead(), Vec2::ZERO);
    }

    /// Orbit backends answer the fly accessors neutrally: no lock (a lock request is refused, a
    /// release succeeds), no velocity, scale 1, no cruise speed, no lead, no thrust.
    #[test]
    fn fly_accessors_are_neutral_on_orbit_backends() {
        for mut ctrl in [
            CameraController::Houdini(HoudiniOrbit::default()),
            CameraController::Quat(QuatOrbit::default()),
        ] {
            assert!(!ctrl.set_horizon_lock(Some(Vec3::Y)), "{}", ctrl.kind());
            assert!(
                ctrl.set_horizon_lock(None),
                "{}: a release took effect",
                ctrl.kind()
            );
            assert_eq!(ctrl.horizon_lock(), None);
            assert_eq!(ctrl.linear_velocity(), Vec3::ZERO);
            assert_eq!(ctrl.speed_scale(), 1.0);
            assert_eq!(ctrl.cruise_speed(), None);
            assert_eq!(ctrl.look_lead(), Vec2::ZERO);
            assert_eq!(ctrl.thrust_input(), Vec3::ZERO);
            assert!(!ctrl.is_coasting(), "{}", ctrl.kind());
            ctrl.stop_coasts();
            assert!(!ctrl.has_motion(), "{}", ctrl.kind());
        }
        let fps = CameraController::Fps(FpsFly::default());
        assert_eq!(
            fps.cruise_speed(),
            Some(InertiaSettings::default().cruise_speed)
        );
        assert!(!fps.is_coasting());
        assert!(!CameraController::Fps(FpsFly::default()).set_horizon_lock(Some(Vec3::Y)));
        assert!(CameraController::Fps(FpsFly::default()).set_horizon_lock(None));
    }

    /// The facade's focus-loss call: held fly axes drop and the camera comes to rest; on an
    /// orbit backend it changes nothing.
    #[test]
    fn release_inputs_through_the_facade_brings_a_fly_camera_to_rest() {
        let dt = 1.0 / 60.0;
        let vp = ViewportSize::new(1, 1);
        for mut ctrl in [
            CameraController::Fps(FpsFly::default()),
            CameraController::Space(SpaceFlight::default()),
        ] {
            ctrl.apply_intent(
                CameraIntent::Thrust {
                    forward: 1.0,
                    right: 0.0,
                    up: 0.0,
                },
                dt,
                vp,
            );
            for _ in 0..20 {
                ctrl.update_dynamics(dt);
            }
            assert!(ctrl.has_motion());
            ctrl.release_inputs();
            assert_eq!(ctrl.thrust_input(), Vec3::ZERO, "{}", ctrl.kind());
            let mut frames = 0;
            while ctrl.update_dynamics(dt) {
                frames += 1;
                assert!(frames < 100_000, "{} never came to rest", ctrl.kind());
            }
            assert!(!ctrl.has_motion(), "{}", ctrl.kind());
        }
        let mut orbit = CameraController::Houdini(HoudiniOrbit::default());
        let before = orbit.pose();
        orbit.release_inputs();
        assert_eq!(orbit.pose(), before);
        assert!(!orbit.has_motion());
    }

    /// NaN / infinite intent values never reach the pose of the free backends: every such
    /// intent is dropped (a `FrameBounds` with garbage bounds frames a finite sphere), the pose
    /// stays finite and nothing is left moving.
    #[test]
    fn non_finite_intents_leave_the_pose_finite_and_at_rest() {
        let (nan, inf) = (f32::NAN, f32::INFINITY);
        let intents = [
            CameraIntent::Orbit {
                yaw_delta: nan,
                pitch_delta: 0.1,
            },
            CameraIntent::Look {
                dyaw: 0.1,
                dpitch: inf,
            },
            CameraIntent::LookRate {
                yaw: nan,
                pitch: 0.0,
            },
            CameraIntent::OrbitInertia {
                yaw_rate: inf,
                pitch_rate: 0.0,
            },
            CameraIntent::Pan {
                dx_px: -inf,
                dy_px: 1.0,
            },
            CameraIntent::Zoom { factor: nan },
            CameraIntent::ZoomInertia { rate: inf },
            CameraIntent::DollyZoomInertia { fov_rate: nan },
            CameraIntent::LookInertia {
                yaw_rate: inf,
                pitch_rate: 0.0,
            },
            CameraIntent::ZoomAtCursor {
                cursor_x: nan,
                cursor_y: 0.0,
                factor: 0.9,
            },
            CameraIntent::Thrust {
                forward: inf,
                right: 0.0,
                up: 0.0,
            },
            CameraIntent::Roll { d: nan },
            CameraIntent::SpeedScale(inf),
            CameraIntent::FrameBounds {
                min: Vec3::splat(nan),
                max: Vec3::splat(inf),
                margin: nan,
            },
        ];
        let dt = 1.0 / 60.0;
        let vp = ViewportSize::new(800, 600);
        for mut ctrl in [
            CameraController::Quat(QuatOrbit::default()),
            CameraController::Fps(FpsFly::default()),
            CameraController::Space(SpaceFlight::default()),
        ] {
            for intent in intents {
                ctrl.apply_intent(intent, dt, vp);
                for _ in 0..3 {
                    ctrl.update_dynamics(dt);
                }
                let pose = ctrl.pose();
                assert!(
                    pose.eye.is_finite() && pose.orientation.is_finite(),
                    "{}: {intent:?} poisoned the pose {pose:?}",
                    ctrl.kind()
                );
            }
            assert!(
                !ctrl.has_motion(),
                "{}: something is left moving",
                ctrl.kind()
            );
            assert_eq!(ctrl.speed_scale(), 1.0, "{}", ctrl.kind());
        }
    }

    /// `set_align` reaches the spaceflight alignment without a variant match (the view turns
    /// toward the grid and `has_motion` reports it); the other backends refuse it, and a
    /// non-finite time constant is ignored.
    #[test]
    fn set_align_reaches_the_space_backend_only() {
        let tilted = CameraPose::from_eye_forward_up(
            Vec3::ZERO,
            Vec3::new(0.3, -0.2, -1.0),
            Vec3::Y,
            45.0_f32.to_radians(),
            0.1,
            500.0,
        );
        let align = Some((OrbitSnapMode::Axis, 0.1));
        let mut space = CameraController::from_pose(CameraKind::Space, tilted);
        assert!(!space.set_align(Some((OrbitSnapMode::Axis, f32::NAN))));
        assert!(!space.has_motion(), "a NaN time constant changed nothing");
        assert!(space.set_align(align));
        assert!(space.has_motion(), "the alignment turns the view");
        assert!(space.set_align(None));
        for kind in [CameraKind::Houdini, CameraKind::Quat, CameraKind::Fps] {
            let mut ctrl = CameraController::from_pose(kind, tilted);
            assert!(!ctrl.set_align(align), "{kind:?}");
            assert!(!ctrl.set_align(None), "{kind:?}");
        }
    }

    /// A press on the view: `stop_coasts` freezes every backend where it is (the orbit release
    /// glides and a fly thrust coast alike) and `is_coasting` goes false at once.
    #[test]
    fn stop_coasts_freezes_every_backend() {
        let dt = 1.0 / 60.0;
        let vp = ViewportSize::new(800, 600);
        let orbit_glide = CameraIntent::OrbitInertia {
            yaw_rate: 1.0,
            pitch_rate: 0.0,
        };
        for mut ctrl in [
            CameraController::Houdini(HoudiniOrbit::default()),
            CameraController::Quat(QuatOrbit::default()),
        ] {
            ctrl.apply_intent(orbit_glide, dt, vp);
            ctrl.update_dynamics(dt);
            assert!(ctrl.is_coasting(), "{}: the glide runs", ctrl.kind());
            assert!(ctrl.has_motion(), "{}", ctrl.kind());
            ctrl.stop_coasts();
            assert!(!ctrl.is_coasting(), "{}", ctrl.kind());
            let frozen = ctrl.pose();
            for _ in 0..10 {
                ctrl.update_dynamics(dt);
            }
            assert_eq!(ctrl.pose(), frozen, "{}: moved after stop", ctrl.kind());
        }
        for mut ctrl in [
            CameraController::Fps(FpsFly::default()),
            CameraController::Space(SpaceFlight::default()),
        ] {
            ctrl.apply_intent(
                CameraIntent::Thrust {
                    forward: 1.0,
                    right: 0.0,
                    up: 0.0,
                },
                dt,
                vp,
            );
            for _ in 0..30 {
                ctrl.update_dynamics(dt);
            }
            ctrl.release_inputs();
            ctrl.update_dynamics(dt);
            assert!(ctrl.is_coasting(), "{}: the thrust coast runs", ctrl.kind());
            assert!(ctrl.linear_velocity().length() > 0.0, "{}", ctrl.kind());
            ctrl.stop_coasts();
            assert!(!ctrl.is_coasting(), "{}", ctrl.kind());
            assert_eq!(ctrl.linear_velocity(), Vec3::ZERO, "{}", ctrl.kind());
            let frozen = ctrl.pose();
            for _ in 0..10 {
                ctrl.update_dynamics(dt);
            }
            assert_eq!(ctrl.pose(), frozen, "{}: moved after stop", ctrl.kind());
            assert!(!ctrl.has_motion(), "{}", ctrl.kind());
        }
    }

    /// A backend switch keeps the user's coast tuning (orbit -> fly -> orbit) and the mode reads
    /// back through the four Space cells.
    #[test]
    fn set_mode_carries_the_coast_settings() {
        let mut ctrl = CameraController::default();
        let CameraController::Houdini(b) = &mut ctrl else {
            panic!("the default is the Houdini orbit");
        };
        b.rig.navigation.coast.time = 2.0;
        for mode in [
            NavMode::FpsFly,
            NavMode::QuatFlightLocked,
            NavMode::Trackball,
        ] {
            ctrl.set_mode(mode);
            assert_eq!(ctrl.mode(), Some(mode), "{mode:?}");
            assert_eq!(ctrl.coast_settings().time, 2.0, "{mode:?}: coast lost");
        }
        ctrl.set_mode(NavMode::Turntable);
        assert_eq!(ctrl.coast_settings().time, 2.0);
    }

    /// `apply_prefs` is the one call: mode, steering, lock, feel, tuning and the turntable's
    /// align time all follow the preferences, and re-applying it every frame changes nothing.
    #[test]
    fn apply_prefs_applies_every_owned_field() {
        use crate::camera_core::mode::NavFeel;
        let mut ctrl = CameraController::default();
        let prefs = NavPrefs {
            align_seconds: 0.25,
            feel: NavFeel::Sharp,
            ..NavPrefs::default()
        };
        ctrl.apply_prefs(&prefs, 10.0, 600.0);
        assert_eq!(ctrl.mode(), Some(NavMode::Turntable));
        let CameraController::Houdini(b) = &ctrl else {
            panic!("the turntable runs on the Houdini orbit");
        };
        assert_eq!(b.rig.align_seconds, 0.25);
        assert_eq!(b.rig.navigation.coast.time, 0.0, "Sharp: no coast");
        assert_eq!(b.rig.navigation.coast, prefs.navigation(0.8, 600.0).coast);

        for mode in NavMode::ALL {
            let prefs = NavPrefs {
                mode,
                ..NavPrefs::default()
            };
            ctrl.apply_prefs(&prefs, 10.0, 600.0);
            let pose = ctrl.pose();
            ctrl.apply_prefs(&prefs, 10.0, 600.0);
            assert_eq!(ctrl.mode(), Some(mode), "{mode:?}");
            assert_eq!(ctrl.pose(), pose, "{mode:?}: re-applying moved the view");
            let expected_lock = mode.spec().horizon_lock;
            assert_eq!(ctrl.horizon_lock(), expected_lock, "{mode:?}");
            match &ctrl {
                CameraController::Fps(b) => {
                    assert_eq!(b.inertia, prefs.inertia(10.0), "{mode:?}");
                }
                CameraController::Space(b) => {
                    assert_eq!(b.inertia, prefs.inertia(10.0), "{mode:?}");
                }
                CameraController::Houdini(_) | CameraController::Quat(_) => {}
            }
        }
    }
}
