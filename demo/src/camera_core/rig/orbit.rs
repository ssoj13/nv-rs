//! Orbit rig: [`OrbitPose`] (Copy, GPU path) + [`OrbitRig`] (coasts / transitions).
//!
//! What: a Y-up turntable camera orbiting a target (yaw about [`WORLD_UP`], pitch about the
//! horizontal right), its view / projection / ray math, pan / zoom / frame, axis and plane snap,
//! release coasts and animated view transitions. Why: the pose is the one orbit state every
//! render pass and controller shares. How: orientation policy and the pitch-step rule live in
//! `crate::camera_core::orientation`, transitions in `crate::camera_core::dynamics`, fitting in `crate::camera_core::fit`. A release
//! coast is a closed-form [`OrbitCoast`] (`crate::camera_core::coast`: turn, log zoom, pan, dolly-zoom
//! channels seeded with the drag's release velocity, each ending EXACTLY at its time `T`);
//! [`OrbitRig::update_inertia`] applies one coast step per frame and never takes the pose over
//! (a coast step is not an edit, it must not cancel the transition or the snap gesture it
//! runs beside; the SEED does).
//! Used by `HoudiniOrbit` (controller), `CameraPose::from_orbit` and host render code.

use crate::camera_core::viewport::{NdcPoint, ViewportPoint, ViewportRay, ViewportSize};
use glam::{Mat4, Quat, Vec2, Vec3};

use crate::camera_core::OrbitSnapMode;
use crate::camera_core::coast::OrbitCoast;
use crate::camera_core::convention::{DEFAULT_OFFSET_AXIS, DEFAULT_ORBIT_DISTANCE, WORLD_UP};
use crate::camera_core::dynamics::TransitionState;
use crate::camera_core::fit::{aabb_center_radius, distance_to_frame_sphere};
use crate::camera_core::orientation::{
    DEFAULT_PITCH_LIMIT, OrientationMode, apply_orbit_delta, orientation_from_yaw_pitch,
    yaw_pitch_from_orientation,
};
use crate::camera_core::projection::PerspectiveSettings;
use crate::camera_core::settings::CameraNavigationSettings;

/// Default [`OrbitRig::transition_speed`] (1/s): the rate `TransitionState::step` approaches a
/// view transition's target with.
const DEFAULT_TRANSITION_SPEED: f32 = 8.0;

/// Horizontal length (of a unit direction) below which it is taken as straight up / down
/// (within ~0.006°): it has no heading, so the yaw comes from elsewhere (the camera's screen
/// right, the current yaw, or 0).
const POLE_HORIZ_EPS: f32 = 1e-4;

/// [`OrbitRig::align_seconds`] at or below this (seconds) means no smoothing: the smart snap
/// lands in one step.
const MIN_ALIGN_SECONDS: f32 = 1e-5;

/// Every input of a direct edit or coast is finite: a NaN / infinite delta or rate would
/// poison the pose (or start a coast that never ends), so such an input is ignored, as in
/// [`crate::camera_core::orbit_about`].
fn all_finite(values: &[f32]) -> bool {
    values.iter().all(|v| v.is_finite())
}

/// A frame step the dynamics can integrate: positive and finite. A NaN / infinite `dt` would
/// poison the pose, a negative one would make the brakes accelerate, zero is a no-op.
fn valid_dt(dt: f32) -> bool {
    dt > 0.0 && dt.is_finite()
}

/// Minimal orbit pose (Copy): the plain-data orbit state (orientation, distance, target) a host
/// can hand to its renderer each frame; [`OrbitRig`] adds settings, coasts and transitions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrbitPose {
    /// Turntable orientation mapping [`DEFAULT_OFFSET_AXIS`] onto the target-to-eye offset.
    pub orientation: Quat,
    /// Eye distance from `target` (world units).
    pub distance: f32,
    /// Orbit pivot the camera looks at (world space).
    pub target: Vec3,
}

impl Default for OrbitPose {
    fn default() -> Self {
        Self::from_yaw_pitch(0.6, 0.35, DEFAULT_ORBIT_DISTANCE, Vec3::ZERO)
    }
}

impl OrbitPose {
    /// The pose at turntable `yaw` / `pitch` (radians), `distance` from `target`.
    pub fn from_yaw_pitch(yaw: f32, pitch: f32, distance: f32, target: Vec3) -> Self {
        Self {
            orientation: orientation_from_yaw_pitch(yaw, pitch),
            distance,
            target,
        }
    }

    /// The orbit pose looking along `forward` at `target` from `distance`. The orbit
    /// orientation maps [`DEFAULT_OFFSET_AXIS`] onto the target-to-eye offset (`-forward`)
    /// using the Y-up turntable decomposition (`yaw` about `WORLD_UP`, `pitch` about `+Z`)
    /// so `OrbitRig` / `HoudiniOrbit` never accumulate roll twist or hit the 180-degree
    /// `from_rotation_arc` singularity at `offset = -Vec3::X` (`forward = +Vec3::X`).
    /// Looking straight up or down (no heading in the forward) the yaw is 0; use
    /// [`OrbitPose::from_view`] when the camera's orientation is known.
    pub fn looking(target: Vec3, forward: Vec3, distance: f32) -> Self {
        Self::turntable(target, forward, || 0.0, distance)
    }

    /// The orbit pose of a camera at `orientation` (a free [`crate::camera_core::CameraPose`] orientation:
    /// local −Z forward, +X screen right, no roll) looking at `target` from `distance`. Like
    /// [`OrbitPose::looking`], but at a pole the yaw comes from the camera's horizontal screen
    /// right, which is −`orientation::turntable_right` ([`OrbitPose::view_plane_axes`]): the
    /// round trip `CameraPose::from_orbit` → `from_view` keeps a Top / Bottom view's heading.
    pub fn from_view(target: Vec3, orientation: Quat, distance: f32) -> Self {
        let pole_yaw = || {
            // turntable right = Ry(yaw) * Z = (sin yaw, 0, cos yaw) = −screen right.
            let r = orientation * Vec3::X;
            (-r.x).atan2(-r.z)
        };
        Self::turntable(target, orientation * Vec3::NEG_Z, pole_yaw, distance)
    }

    /// THE forward → turntable decomposition of [`OrbitPose::looking`] / [`OrbitPose::from_view`]:
    /// yaw from the forward's horizontal part, `pole_yaw` when it has none
    /// ([`POLE_HORIZ_EPS`]).
    fn turntable(
        target: Vec3,
        forward: Vec3,
        pole_yaw: impl FnOnce() -> f32,
        distance: f32,
    ) -> Self {
        let offset = -forward.normalize_or_zero();
        let offset = if offset == Vec3::ZERO {
            DEFAULT_OFFSET_AXIS
        } else {
            offset
        };
        let horiz = Vec3::new(offset.x, 0.0, offset.z).length();
        let yaw = if horiz > POLE_HORIZ_EPS {
            (-offset.z).atan2(offset.x)
        } else {
            pole_yaw()
        };
        let pitch = crate::camera_core::num::clamp(offset.y, -1.0, 1.0).asin();
        Self {
            orientation: orientation_from_yaw_pitch(yaw, pitch),
            distance,
            target,
        }
    }

    /// The target-to-eye vector (length `distance`).
    pub fn offset(&self) -> Vec3 {
        self.orientation * (DEFAULT_OFFSET_AXIS * self.distance)
    }

    /// The eye position (world space).
    pub fn eye(&self) -> Vec3 {
        self.target + self.offset()
    }

    /// Right-handed view matrix looking from [`eye`](Self::eye) at `target`, upright about
    /// [`WORLD_UP`]; at a pole the orientation's own up keeps the heading.
    pub fn view_matrix(&self) -> Mat4 {
        let eye = self.eye();
        let forward = (self.target - eye).normalize_or_zero();
        glam::camera::rh::view::look_at_mat4(eye, self.target, self.up_reference(forward))
    }

    /// The `up` hint of [`view_matrix`](Self::view_matrix) (and of the rolled-pole fallback of
    /// [`view_plane_axes`](Self::view_plane_axes)): [`WORLD_UP`], except within ~0.8° of a pole
    /// where it is undefined and the orientation's own up (which keeps the heading) takes over.
    /// One rule, so the view matrix and the pan / zoom basis never disagree at the pole.
    fn up_reference(&self, forward: Vec3) -> Vec3 {
        if forward.dot(WORLD_UP).abs() > 0.9999 {
            self.orientation * WORLD_UP
        } else {
            WORLD_UP
        }
    }

    /// Reverse-Z perspective: `near` / `far` are passed SWAPPED so the near plane maps to NDC
    /// depth 1 and far to 0. Contract for the host renderer: use a `Greater` depth test and
    /// clear depth to 0; float-depth precision then concentrates far from the camera (no
    /// distant z-fighting on large depth spans).
    ///
    /// `directx` is glam 0.33's name for the [0, 1] depth-clip convention (wgpu's), i.e.
    /// exactly what the deprecated `Mat4::perspective_rh` produced.
    pub fn projection_matrix(&self, aspect: f32, settings: PerspectiveSettings) -> Mat4 {
        glam::camera::rh::proj::directx::perspective(
            settings.fov_y,
            aspect,
            settings.far,
            settings.near,
        )
    }

    /// Projection × view for a `width` × `height` pixel viewport.
    pub fn view_proj(&self, width: u32, height: u32, settings: PerspectiveSettings) -> Mat4 {
        let aspect = width as f32 / height.max(1) as f32;
        self.projection_matrix(aspect, settings) * self.view_matrix()
    }

    /// Orbit by `yaw_delta` / `pitch_delta` radians under `mode`; the pitch step is limited by
    /// `pitch_limit` ([`apply_orbit_delta`]).
    pub fn orbit(
        &mut self,
        yaw_delta: f32,
        pitch_delta: f32,
        mode: OrientationMode,
        pitch_limit: f32,
    ) {
        apply_orbit_delta(
            mode,
            &mut self.orientation,
            yaw_delta,
            pitch_delta,
            pitch_limit,
        );
    }

    /// World-space basis vectors for the camera's screen plane.
    ///
    /// **What:** returns `(right, up)` — the world directions of the
    /// screen-right and screen-up axes derived from `eye → target` and
    /// `WORLD_UP`. `right` and `up` are unit vectors; `right × up` aligns with
    /// the forward vector (away from the eye).
    ///
    /// **Why:** the one screen basis of the orbit: pan, pan coast and cursor zoom move along
    /// it, and hosts may project world axes onto the screen with `dot(axis, right)` /
    /// `dot(axis, up)` (e.g. an axis gizmo); it always matches
    /// [`view_matrix`](Self::view_matrix) rows 0 / 1.
    ///
    /// **Where:** [`pan`](Self::pan), [`zoom_at_cursor`](Self::zoom_at_cursor),
    /// [`OrbitRig::pan_inertia`], `CameraPose::from_orbit`; hosts via [`OrbitRig::pose`].
    ///
    /// At an exact pole (a snapped Top / Bottom view, which the rig may rest at) the right
    /// is the turntable right (`orientation::turntable_right`), the same axis
    /// [`view_matrix`](Self::view_matrix) implies through `orientation * WORLD_UP`, so the kept
    /// heading shows on the [`CameraPose::from_orbit`](crate::camera_core::CameraPose::from_orbit) path and
    /// the pan moves along the displayed screen axes.
    ///
    /// Zero-safe: a zero distance or a degenerate basis never yields NaN; the
    /// fallbacks are `Vec3::X` / [`WORLD_UP`].
    pub fn view_plane_axes(&self) -> (Vec3, Vec3) {
        let eye = self.eye();
        let forward = (self.target - eye).normalize_or_zero();
        // forward = −offset, so forward × WORLD_UP = −(offset × WORLD_UP) = −turntable right.
        let mut right = -crate::camera_core::orientation::turntable_right(self.orientation);
        if right.length_squared() < 1e-8 {
            // Only a rolled (non-turntable) orientation at a pole: the view matrix's up hint
            // (the orientation's own up there) gives the right it displays.
            right = forward.cross(self.up_reference(forward));
        }
        let right = right.try_normalize().unwrap_or(Vec3::X);
        let up = right.cross(forward).try_normalize().unwrap_or(WORLD_UP);
        (right, up)
    }

    /// Pan the target by a pointer delta in pixels along the screen axes, scaled by
    /// `distance * pan_sensitivity` (drag-the-scene convention).
    pub fn pan(&mut self, dx_px: f32, dy_px: f32, pan_sensitivity: f32) {
        let (right, up) = self.view_plane_axes();
        let scale = self.distance * pan_sensitivity;
        self.target += right * (-dx_px * scale) + up * (dy_px * scale);
    }

    /// Dolly: multiply the distance by `factor`, clamped to the settings' distance band.
    pub fn zoom(&mut self, factor: f32, settings: PerspectiveSettings) {
        self.distance = settings.clamp_distance(self.distance * factor);
    }

    /// [`zoom`](Self::zoom) keeping the world point under `point` (on the target plane) fixed
    /// on screen.
    pub fn zoom_at_cursor(
        &mut self,
        point: ViewportPoint,
        size: ViewportSize,
        factor: f32,
        settings: PerspectiveSettings,
    ) {
        let ndc = point.to_ndc(size);
        let (right, up) = self.view_plane_axes();
        let half_tan = (settings.fov_y * 0.5).tan();
        let half_w = self.distance * half_tan * size.aspect();
        let half_h = self.distance * half_tan;
        let focus = self.target + right * (ndc.x * half_w) + up * (ndc.y * half_h);
        self.zoom(factor, settings);
        let (right2, up2) = self.view_plane_axes();
        let half_w2 = self.distance * half_tan * size.aspect();
        let half_h2 = self.distance * half_tan;
        self.target = focus - (right2 * (ndc.x * half_w2) + up2 * (ndc.y * half_h2));
    }

    /// Aim at the centre of the AABB `min`..`max` from the distance that fits its bounding
    /// sphere (times `margin`) into the viewport; non-finite input is sanitised.
    pub fn frame_bounds(
        &mut self,
        min: Vec3,
        max: Vec3,
        viewport_width: u32,
        viewport_height: u32,
        margin: f32,
        settings: PerspectiveSettings,
    ) {
        // `crate::camera_core::fit` owns the guards: a finite centre (origin for a non-finite AABB) and a
        // finite distance clamped to the manual-zoom band, so Fit-All never lands farther
        // than a hand zoom could.
        let (center, radius) = aabb_center_radius(min, max);
        let aspect = viewport_width as f32 / viewport_height.max(1) as f32;
        self.target = center;
        self.distance = distance_to_frame_sphere(radius, aspect, settings.fov_y, margin, settings);
    }

    /// NDC `[x, y]` of a viewport point.
    pub fn viewport_ndc(&self, point: ViewportPoint, size: ViewportSize) -> [f32; 2] {
        NdcPoint::from_viewport(point, size).as_array()
    }

    /// NDC `[x, y]` of pixel `px`, `py` (clamped into the `width` × `height` viewport).
    pub fn screen_ndc(&self, px: f32, py: f32, width: u32, height: u32) -> [f32; 2] {
        let size = ViewportSize::new(width, height);
        let point = ViewportPoint::new_clamped(px, py, size);
        NdcPoint::from_viewport(point, size).as_array()
    }

    /// The inverse of [`view_proj`](Self::view_proj) as column arrays (GPU uniform layout).
    pub fn inv_view_proj_cols(
        &self,
        width: u32,
        height: u32,
        settings: PerspectiveSettings,
    ) -> [[f32; 4]; 4] {
        self.view_proj(width, height, settings)
            .inverse()
            .to_cols_array_2d()
    }

    /// The eye position as an array (GPU uniform layout).
    pub fn eye_pos(&self) -> [f32; 3] {
        self.eye().to_array()
    }

    /// The world ray through a viewport point (picking).
    pub fn viewport_ray(
        &self,
        point: ViewportPoint,
        size: ViewportSize,
        settings: PerspectiveSettings,
    ) -> ViewportRay {
        ViewportRay::from_inv_view_proj(
            point,
            size,
            self.inv_view_proj_cols(size.width, size.height, settings),
            self.eye_pos(),
        )
    }

    /// The world ray `(origin, dir)` through pixel `px`, `py` (clamped into the viewport).
    pub fn screen_ray(
        &self,
        px: f32,
        py: f32,
        width: u32,
        height: u32,
        settings: PerspectiveSettings,
    ) -> ([f32; 3], [f32; 3]) {
        let size = ViewportSize::new(width, height);
        let ray = self.viewport_ray(ViewportPoint::new_clamped(px, py, size), size, settings);
        (ray.origin, ray.dir)
    }
}

/// Full orbit rig: pose + projection + orientation policy + release coasts + transitions.
#[derive(Debug, Clone)]
pub struct OrbitRig {
    /// The current orbit pose.
    pub pose: OrbitPose,
    /// Lens and distance band.
    pub projection: PerspectiveSettings,
    /// Sensitivities and the release-coast tuning (`navigation.coast`, latched per coast start).
    pub navigation: CameraNavigationSettings,
    /// How orbit deltas rotate the orientation.
    pub orientation_mode: OrientationMode,
    /// Max |pitch| of an orbit step (radians; [`DEFAULT_PITCH_LIMIT`]).
    pub pitch_limit: f32,
    /// Release coasts: turn / zoom / pan / dolly-zoom ([`OrbitRig::update_inertia`]).
    pub coast: OrbitCoast,
    /// Animated move toward a target pose.
    pub transition: TransitionState,
    /// Speed of [`OrbitRig::update_animation`] transitions (1/s).
    pub transition_speed: f32,
    /// Time constant (seconds) of the [`OrbitSnapMode::Smart`] ease, run as a transition by
    /// [`OrbitRig::update_dynamics`]; `0.0` snaps in one frame.
    pub align_seconds: f32,
    /// Free yaw / pitch of an axis-snap gesture ([`OrbitRig::orbit_snapped`]); cleared by
    /// every other edit of the pose, so a later gesture never resumes a stale direction.
    snap: Option<(f32, f32)>,
    /// Speed (1/s) of the running transition when it is a smart-snap ease
    /// (`1 / align_seconds`); `None` for a view transition ([`OrbitRig::transition_speed`]).
    ease_speed: Option<f32>,
}

impl Default for OrbitRig {
    fn default() -> Self {
        Self::from_yaw_pitch(0.6, 0.35, DEFAULT_ORBIT_DISTANCE, Vec3::ZERO)
    }
}

impl OrbitRig {
    /// A rig at turntable `yaw` / `pitch`, `distance` (clamped) from `target`, default settings.
    pub fn from_yaw_pitch(yaw: f32, pitch: f32, distance: f32, target: Vec3) -> Self {
        let projection = PerspectiveSettings::default();
        Self {
            pose: OrbitPose {
                orientation: orientation_from_yaw_pitch(yaw, pitch),
                distance: projection.clamp_distance(distance),
                target,
            },
            projection,
            navigation: CameraNavigationSettings::default(),
            orientation_mode: OrientationMode::HorizontalPlaneEuler,
            pitch_limit: DEFAULT_PITCH_LIMIT,
            coast: OrbitCoast::default(),
            transition: TransitionState::default(),
            transition_speed: DEFAULT_TRANSITION_SPEED,
            align_seconds: 0.0,
            snap: None,
            ease_speed: None,
        }
    }

    /// A rig with default settings at `pose`.
    pub fn from_pose(pose: OrbitPose) -> Self {
        Self {
            pose,
            ..Self::default()
        }
    }

    // ─── Pose accessors ──────────────────────────────────────────────────────

    /// The pose orientation.
    pub fn orientation(&self) -> Quat {
        self.pose.orientation
    }

    /// The pose orientation, mutable.
    pub fn orientation_mut(&mut self) -> &mut Quat {
        &mut self.pose.orientation
    }

    /// The eye distance from the target.
    pub fn distance(&self) -> f32 {
        self.pose.distance
    }

    /// The eye distance, mutable (not clamped).
    pub fn distance_mut(&mut self) -> &mut f32 {
        &mut self.pose.distance
    }

    /// The orbit pivot.
    pub fn target(&self) -> Vec3 {
        self.pose.target
    }

    /// The orbit pivot, mutable.
    pub fn target_mut(&mut self) -> &mut Vec3 {
        &mut self.pose.target
    }

    /// See [`OrbitPose::offset`].
    pub fn offset(&self) -> Vec3 {
        self.pose.offset()
    }

    /// See [`OrbitPose::eye`].
    pub fn eye(&self) -> Vec3 {
        self.pose.eye()
    }

    /// See [`OrbitPose::view_proj`] (with the rig's projection).
    pub fn view_proj(&self, width: u32, height: u32) -> Mat4 {
        self.pose.view_proj(width, height, self.projection)
    }

    /// Stop every coast and cancel a running transition, which rebuilds the whole pose each
    /// frame and would overwrite a direct edit.
    fn halt(&mut self) {
        self.coast.stop();
        self.transition.cancel();
    }

    /// A direct (non-snap) edit takes the pose over: [`halt`](Self::halt) and end any snap
    /// gesture, whose free direction no longer describes the pose. Also called by
    /// `HoudiniOrbit::set_pose` when a host restores a pose.
    pub(crate) fn take_over(&mut self) {
        self.halt();
        self.snap = None;
    }

    /// Orbit by the deltas (radians) under the rig's mode and pitch limit; stops any coast,
    /// transition and snap gesture. Non-finite deltas are ignored.
    pub fn orbit(&mut self, yaw_delta: f32, pitch_delta: f32) {
        if !all_finite(&[yaw_delta, pitch_delta]) {
            return;
        }
        self.take_over();
        self.pose.orbit(
            yaw_delta,
            pitch_delta,
            self.orientation_mode,
            self.pitch_limit,
        );
    }

    /// Orbit snapped to the nearest axis view (Blender's Alt + drag, Shift + tumble here):
    /// the drag accumulates in a free yaw / pitch (from the current view when `start`), and
    /// the view is the axis-aligned one closest to that free direction: the axis of its
    /// largest component, with the yaw in quarter turns so a top / bottom view is square to
    /// the world too. Exactly straight up / down is allowed (no pitch limit).
    pub fn orbit_snapped(&mut self, yaw_delta: f32, pitch_delta: f32, start: bool) {
        self.orbit_snapped_in(yaw_delta, pitch_delta, start, OrbitSnapMode::Axis);
    }

    /// Project the gesture's free direction onto the nearest world plane or axis.
    /// The free direction is retained across frames so constrained motion never sticks.
    /// When `mode == OrbitSnapMode::Smart` and `align_seconds > 0`, the view eases toward the
    /// snapped target as a transition with time constant `align_seconds`, advanced by
    /// [`update_dynamics`](Self::update_dynamics) with the real frame time, so it finishes
    /// after the gesture is released. Non-finite deltas are ignored.
    pub fn orbit_snapped_in(
        &mut self,
        yaw_delta: f32,
        pitch_delta: f32,
        start: bool,
        mode: OrbitSnapMode,
    ) {
        use std::f32::consts::FRAC_PI_2;
        if !all_finite(&[yaw_delta, pitch_delta]) {
            return;
        }
        self.halt();
        if start || self.snap.is_none() {
            self.snap = Some(yaw_pitch_from_orientation(self.pose.orientation));
        }
        let Some((yaw, pitch)) = &mut self.snap else {
            return;
        };
        *yaw += yaw_delta;
        *pitch = crate::camera_core::num::clamp(*pitch + pitch_delta, -FRAC_PI_2, FRAC_PI_2);
        let (target_yaw, target_pitch) = crate::camera_core::orientation::snapped_yaw_pitch(*yaw, *pitch, mode);
        let snapped = orientation_from_yaw_pitch(target_yaw, target_pitch);
        if mode == OrbitSnapMode::Smart && self.align_seconds > MIN_ALIGN_SECONDS {
            // `TransitionState::step` decays the remaining way as `exp(-speed * dt)`, so a time
            // constant of `align_seconds` is the speed `1 / align_seconds`. A retarget restarts
            // from the current pose (no jump). `animate_to` does not touch the coasts: the
            // `halt` above already stopped them.
            self.transition
                .animate_to(snapped, self.pose.distance, self.pose.target);
            self.ease_speed = Some(self.align_seconds.recip());
        } else {
            self.pose.orientation = snapped;
        }
    }

    /// See [`OrbitPose::pan`] (with the rig's pan sensitivity); stops any coast, transition and
    /// snap gesture. Non-finite deltas are ignored.
    pub fn pan(&mut self, dx_px: f32, dy_px: f32) {
        if !all_finite(&[dx_px, dy_px]) {
            return;
        }
        self.take_over();
        self.pose.pan(dx_px, dy_px, self.navigation.pan_sensitivity);
    }

    /// See [`OrbitPose::zoom`]; stops any coast, transition and snap gesture. A non-finite
    /// factor is ignored.
    pub fn zoom(&mut self, factor: f32) {
        if !all_finite(&[factor]) {
            return;
        }
        self.take_over();
        self.pose.zoom(factor, self.projection);
    }

    /// See [`OrbitPose::zoom_at_cursor`]; stops any coast, transition and snap gesture. A
    /// non-finite factor or point is ignored.
    pub fn zoom_at_cursor(&mut self, point: ViewportPoint, size: ViewportSize, factor: f32) {
        if !all_finite(&[factor, point.x, point.y]) {
            return;
        }
        self.take_over();
        self.pose
            .zoom_at_cursor(point, size, factor, self.projection);
    }

    /// Replace the navigation settings. A running coast keeps the time / softness it latched
    /// at its start; the new `navigation.coast` applies to the next one.
    pub fn apply_navigation(&mut self, navigation: CameraNavigationSettings) {
        self.navigation = navigation;
    }

    /// See [`OrbitPose::frame_bounds`]; stops any coast, transition and snap gesture.
    pub fn frame_bounds(
        &mut self,
        min: Vec3,
        max: Vec3,
        viewport_width: u32,
        viewport_height: u32,
        margin: f32,
    ) {
        self.pose.frame_bounds(
            min,
            max,
            viewport_width,
            viewport_height,
            margin,
            self.projection,
        );
        self.take_over();
    }

    // ─── Coasts ──────────────────────────────────────────────────────────────

    /// Seed an orbit coast with the release velocity `yaw_rate` / `pitch_rate` (rad/s) of a
    /// drag (frame-rate independent; added to a running turn coast's current velocity),
    /// advanced by [`update_inertia`](Self::update_inertia) with `navigation.coast`. Cancels a
    /// running transition (it would overwrite the coast and the coast would pull off its
    /// target) and any snap gesture, but only when a coast really starts: a rate below
    /// `coast.min_speed` or `coast.time == 0` leaves the rig as it is. Non-finite rates are
    /// ignored.
    pub fn orbit_inertia(&mut self, yaw_rate: f32, pitch_rate: f32) {
        if !all_finite(&[yaw_rate, pitch_rate]) {
            return;
        }
        let dv = Vec2::new(yaw_rate, pitch_rate);
        if self.coast.turn.kick(dv, 1.0, &self.navigation.coast) {
            self.start_coast();
        }
    }

    /// A coast takes the pose over like a direct edit, but keeps the coasts already running.
    fn start_coast(&mut self) {
        self.transition.cancel();
        self.snap = None;
    }

    /// Seed a pan coast from a pointer velocity in px/s, through the same pixel-to-world
    /// scale as [`pan`](Self::pan): the coast starts at the speed of the drag (the world
    /// velocity is fixed at the seed; `coast.min_speed` is in distances per second). Cancels a
    /// running transition and any snap gesture when a coast starts. Non-finite rates are
    /// ignored.
    pub fn pan_inertia(&mut self, rate_x: f32, rate_y: f32) {
        if !all_finite(&[rate_x, rate_y]) {
            return;
        }
        let s = self.pose.distance * self.navigation.pan_sensitivity;
        let (right, up) = self.pose.view_plane_axes();
        let dv = right * (-rate_x * s) + up * (rate_y * s);
        if self
            .coast
            .pan
            .kick(dv, self.pose.distance, &self.navigation.coast)
        {
            self.start_coast();
        }
    }

    /// Seed a zoom coast with the log-distance `rate` (1/s, positive = farther):
    /// [`update_inertia`](Self::update_inertia) multiplies the distance by `exp(step)` over the
    /// coast. Cancels a running transition and any snap gesture when a coast starts. A
    /// non-finite rate is ignored.
    pub fn zoom_inertia(&mut self, rate: f32) {
        if !all_finite(&[rate]) {
            return;
        }
        if self.coast.zoom.kick(rate, 1.0, &self.navigation.coast) {
            self.start_coast();
        }
    }

    /// Seed a dolly-zoom coast with the fov rate `fov_rate` (rad/s): each step widens the
    /// lens by the coast's `dfov` through the dolly-zoom invariant (the subject keeps its
    /// size), and the channel stops where the lens range ends. Cancels a running transition
    /// and any snap gesture when a coast starts. A non-finite rate is ignored.
    pub fn dolly_zoom_inertia(&mut self, fov_rate: f32) {
        if !all_finite(&[fov_rate]) {
            return;
        }
        if self
            .coast
            .dolly_zoom
            .kick(fov_rate, 1.0, &self.navigation.coast)
        {
            self.start_coast();
        }
    }

    /// Stop every coast (the pose stays where it is).
    pub fn stop_inertia(&mut self) {
        self.coast.stop();
    }

    /// `true` while any coast runs ([`OrbitCoast::is_active`]: exact, a coast ends exactly at
    /// its time). A coast clamped at a limit (pitch step rule, distance band, lens range)
    /// keeps running, motionless, up to its end.
    pub fn has_inertia(&self) -> bool {
        self.coast.is_active()
    }

    /// Advance the coasts by `dt` seconds: one [`OrbitCoast::step`] applied to the pose. The
    /// turn goes through `pose.orbit` (the pitch STEP rule, so a coast at the top limit never
    /// crosses the pole), the distance multiplies by `exp(log_zoom)` clamped to the band, the
    /// target moves by the world pan, and the dolly-zoom step goes through the lens rule
    /// (`controller::dolly_zoom`; `None`, the lens range reached, stops that channel). Never
    /// calls `take_over`: a coast step is not an edit. Returns `true`
    /// while a coast still runs. A `dt` that is not positive and finite changes nothing and
    /// returns `false`.
    pub fn update_inertia(&mut self, dt: f32) -> bool {
        if !valid_dt(dt) || !self.has_inertia() {
            return false;
        }
        let step = self.coast.step(dt);
        self.pose.orbit(
            step.turn.x,
            step.turn.y,
            self.orientation_mode,
            self.pitch_limit,
        );
        if step.log_zoom != 0.0 {
            self.pose.distance = self
                .projection
                .clamp_distance(self.pose.distance * step.log_zoom.exp());
        }
        self.pose.target += step.pan;
        if step.dfov != 0.0 {
            match crate::camera_core::controller::dolly_zoom(
                self.pose.distance,
                self.projection.fov_y,
                step.dfov,
                self.projection,
            ) {
                Some((distance, fov_y)) => {
                    self.pose.distance = self.projection.clamp_distance(distance);
                    self.projection.fov_y = fov_y;
                }
                None => self.coast.dolly_zoom.stop(),
            }
        }
        self.has_inertia()
    }

    // ─── Transitions ─────────────────────────────────────────────────────────

    /// Animate to turntable `yaw` / `pitch`, `distance` (clamped) and `target` at
    /// [`transition_speed`](Self::transition_speed); stops any coast and snap gesture (the
    /// transition rebuilds the pose each frame and would fight a coast;
    /// `TransitionState::animate_to` leaves the coasts to this caller).
    pub fn animate_to(&mut self, yaw: f32, pitch: f32, distance: f32, target: Vec3) {
        let orientation = orientation_from_yaw_pitch(yaw, pitch);
        let distance = self.projection.clamp_distance(distance);
        self.coast.stop();
        self.snap = None;
        self.ease_speed = None;
        self.transition.animate_to(orientation, distance, target);
    }

    /// Animate to `view` about the current target at the current distance. The rig is Y-up
    /// (turntable about [`WORLD_UP`]), so the view's world direction is exact and its screen
    /// up is +Y; looking straight along ±Y (Top / Bottom) keeps the current heading (yaw), so
    /// the snap never spins 180° (no flip). The quaternion slerp of the transition takes the
    /// shortest way; a new call mid-transition retargets from the current pose.
    pub fn animate_to_view(&mut self, view: crate::camera_core::ViewTarget) {
        let d = view.dir();
        let (cur_yaw, _) = yaw_pitch_from_orientation(self.pose.orientation);
        let yaw = if Vec3::new(d.x, 0.0, d.z).length() > POLE_HORIZ_EPS {
            (-d.z).atan2(d.x)
        } else {
            cur_yaw
        };
        let pitch = crate::camera_core::num::clamp(d.y, -1.0, 1.0).asin();
        self.animate_to(yaw, pitch, self.pose.distance, self.pose.target);
    }

    /// Stop the transition where it is.
    pub fn cancel_animation(&mut self) {
        self.transition.cancel();
    }

    /// `true` while a transition (a view animation or a smart-snap ease) runs.
    pub fn is_animating(&self) -> bool {
        self.transition.is_animating()
    }

    /// Advance the transition by `dt` seconds (at [`transition_speed`](Self::transition_speed),
    /// or `1 / align_seconds` for a smart-snap ease); `true` while still animating.
    pub fn update_animation(&mut self, dt: f32) -> bool {
        let speed = self.ease_speed.unwrap_or(self.transition_speed);
        self.transition.step(
            &mut self.pose.orientation,
            &mut self.pose.distance,
            &mut self.pose.target,
            dt,
            speed,
        )
    }

    /// Advance coasts and the transition by `dt` seconds; `true` while either moves. A `dt`
    /// that is not positive and finite changes nothing and returns `false`.
    pub fn update_dynamics(&mut self, dt: f32) -> bool {
        if !valid_dt(dt) {
            return false;
        }
        let a = self.update_inertia(dt);
        let b = self.update_animation(dt);
        a || b
    }

    // ─── Viewport helpers ────────────────────────────────────────────────────

    /// A [`ViewportSize`] of `width` × `height` pixels.
    pub fn viewport_size(&self, width: u32, height: u32) -> ViewportSize {
        ViewportSize::new(width, height)
    }

    /// See [`OrbitPose::viewport_ndc`].
    pub fn viewport_ndc(&self, point: ViewportPoint, size: ViewportSize) -> [f32; 2] {
        self.pose.viewport_ndc(point, size)
    }

    /// See [`OrbitPose::screen_ndc`].
    pub fn screen_ndc(&self, px: f32, py: f32, width: u32, height: u32) -> [f32; 2] {
        self.pose.screen_ndc(px, py, width, height)
    }

    /// See [`OrbitPose::inv_view_proj_cols`] (with the rig's projection).
    pub fn inv_view_proj_cols(&self, width: u32, height: u32) -> [[f32; 4]; 4] {
        self.pose.inv_view_proj_cols(width, height, self.projection)
    }

    /// See [`OrbitPose::eye_pos`].
    pub fn eye_pos(&self) -> [f32; 3] {
        self.pose.eye_pos()
    }

    /// See [`OrbitPose::viewport_ray`] (with the rig's projection).
    pub fn viewport_ray(&self, point: ViewportPoint, size: ViewportSize) -> ViewportRay {
        self.pose.viewport_ray(point, size, self.projection)
    }

    /// See [`OrbitPose::screen_ray`] (with the rig's projection).
    pub fn screen_ray(&self, px: f32, py: f32, width: u32, height: u32) -> ([f32; 3], [f32; 3]) {
        self.pose.screen_ray(px, py, width, height, self.projection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_bounds_sets_target_and_distance() {
        let mut rig = OrbitRig::default();
        rig.frame_bounds(
            Vec3::new(-10.0, -5.0, -10.0),
            Vec3::new(10.0, 5.0, 10.0),
            800,
            600,
            1.2,
        );
        assert!(rig.pose.target.length() < 1e-3);
        assert!(rig.pose.distance > rig.projection.distance_min);
    }

    #[test]
    fn top_view_keeps_its_yaw() {
        use std::f32::consts::FRAC_PI_2;
        for pitch in [FRAC_PI_2, -FRAC_PI_2] {
            let (yaw, back) = yaw_pitch_from_orientation(orientation_from_yaw_pitch(0.7, pitch));
            assert!(
                (yaw - 0.7).abs() < 1e-4 && (back - pitch).abs() < 1e-4,
                "{yaw} {back}"
            );
        }
    }

    #[test]
    fn shift_orbit_snaps_to_the_nearest_axis_view() {
        use std::f32::consts::FRAC_PI_2;
        let mut rig = OrbitRig::from_yaw_pitch(0.3, 0.2, 10.0, Vec3::ZERO);
        let view = |rig: &OrbitRig| rig.pose.orientation * DEFAULT_OFFSET_AXIS;
        rig.orbit_snapped(0.0, 0.0, true);
        assert!(
            view(&rig).distance(Vec3::X) < 1e-5,
            "nearest axis +X: {}",
            view(&rig)
        );
        // A small drag stays on the axis; past half a quarter turn it moves to the next one.
        rig.orbit_snapped(0.4, 0.0, false);
        assert!(view(&rig).distance(Vec3::X) < 1e-5);
        rig.orbit_snapped(0.6, 0.0, false);
        let side = view(&rig);
        assert!(
            side.distance(Vec3::NEG_Z) < 1e-5,
            "next quarter turn: {side}"
        );
        // Up to straight above, square to the world (the yaw stays a quarter turn).
        rig.orbit_snapped(0.0, 1.2, false);
        assert!(view(&rig).distance(Vec3::Y) < 1e-5);
        let (yaw, pitch) = yaw_pitch_from_orientation(rig.pose.orientation);
        assert!((yaw - FRAC_PI_2).abs() < 1e-4 && (pitch - FRAC_PI_2).abs() < 1e-4);
        assert!(!rig.has_inertia(), "snapped views do not coast");
    }

    #[test]
    fn plane_snap_projects_to_all_three_nearest_planes_and_keeps_tumbling() {
        for source in [
            Vec3::new(0.1, 0.8, 0.6),
            Vec3::new(0.8, 0.1, 0.6),
            Vec3::new(0.8, 0.6, 0.1),
        ] {
            let direction = source.normalize();
            let yaw = (-direction.z).atan2(direction.x);
            let mut rig = OrbitRig::from_yaw_pitch(yaw, direction.y.asin(), 10.0, Vec3::ZERO);
            rig.orbit_inertia(2.0, 1.0);
            rig.orbit_snapped_in(0.0, 0.0, true, OrbitSnapMode::Plane);
            let projected = rig.pose.orientation * DEFAULT_OFFSET_AXIS;
            let mut expected = source;
            if source.x == 0.1 {
                expected.x = 0.0;
            }
            if source.y == 0.1 {
                expected.y = 0.0;
            }
            if source.z == 0.1 {
                expected.z = 0.0;
            }
            assert!(
                projected.distance(expected.normalize()) < 1e-5,
                "{projected} != {expected}"
            );
            assert!(!rig.has_inertia());
            rig.orbit_snapped_in(0.05, 0.05, false, OrbitSnapMode::Plane);
            let moved = rig.pose.orientation * DEFAULT_OFFSET_AXIS;
            assert!(
                moved.distance(projected) > 0.001,
                "plane constraint must allow continuous tumble"
            );
            rig.orbit_snapped_in(0.0, 0.0, true, OrbitSnapMode::Axis);
            let axis = rig.pose.orientation * DEFAULT_OFFSET_AXIS;
            assert!(
                [Vec3::X, Vec3::Y, Vec3::Z]
                    .iter()
                    .any(|a| axis.dot(*a).abs() > 0.99999)
            );
        }
    }

    /// A view intent animates the rig onto the view's direction about its target; Top keeps
    /// the heading; a second target mid-transition retargets from the current pose; the next
    /// orbit after an exact Top view does not jump.
    #[test]
    fn view_snaps_animate_keep_the_heading_and_retarget() {
        use crate::camera_core::{Face, UpAxis, ViewTarget};
        let mut rig = OrbitRig::from_yaw_pitch(0.4, 0.2, 10.0, Vec3::new(1.0, 0.0, 0.0));
        let side = |rig: &OrbitRig| rig.pose.orientation * DEFAULT_OFFSET_AXIS;
        let top = ViewTarget::face(Face::Top, UpAxis::Y);
        rig.animate_to_view(top);
        assert!(rig.is_animating());
        rig.update_animation(1.0 / 60.0);
        let mid = side(&rig);
        // Retarget mid-way: continues from `mid`, no jump back to the start.
        let right = ViewTarget::face(Face::Right, UpAxis::Y);
        rig.animate_to_view(right);
        rig.update_animation(1.0 / 60.0);
        assert!(side(&rig).distance(mid) < 0.3, "continuous retarget");
        for _ in 0..600 {
            if !rig.update_animation(1.0 / 60.0) {
                break;
            }
        }
        assert!(side(&rig).distance(Vec3::X) < 1e-3, "{}", side(&rig));
        assert!((rig.pose.distance - 10.0).abs() < 1e-3);
        // Top from here: exact, heading (yaw 0 of the Right view) kept.
        rig.animate_to_view(top);
        for _ in 0..600 {
            if !rig.update_animation(1.0 / 60.0) {
                break;
            }
        }
        assert!(side(&rig).distance(Vec3::Y) < 1e-3);
        let (yaw, _) = yaw_pitch_from_orientation(rig.pose.orientation);
        assert!(yaw.abs() < 1e-3, "heading kept: {yaw}");
        // The first orbit off the exact pole moves by the step, not down to a lower limit.
        rig.orbit(0.0, -0.05);
        let (_, pitch) = yaw_pitch_from_orientation(rig.pose.orientation);
        assert!(
            (pitch - (std::f32::consts::FRAC_PI_2 - 0.05)).abs() < 2e-3,
            "{pitch}"
        );
    }

    /// At the exact Top / Bottom a view snap rests on, the free pose (`CameraPose::from_orbit`,
    /// i.e. `HoudiniOrbit::pose`) shows the same image as `OrbitPose::view_matrix`, the kept
    /// heading included, and the pan axes are the displayed screen axes.
    #[test]
    fn a_snapped_pole_keeps_its_heading_on_the_pose_and_pan_path() {
        use crate::camera_core::{CameraPose, Face, UpAxis, ViewTarget};
        use std::f32::consts::FRAC_PI_2;
        for face in [Face::Top, Face::Bottom] {
            let mut rig = OrbitRig::from_yaw_pitch(0.7, 0.2, 10.0, Vec3::new(1.0, 2.0, 3.0));
            rig.animate_to_view(ViewTarget::face(face, UpAxis::Y));
            for _ in 0..600 {
                if !rig.update_animation(1.0 / 60.0) {
                    break;
                }
            }
            let (yaw, pitch) = yaw_pitch_from_orientation(rig.pose.orientation);
            assert!((pitch.abs() - FRAC_PI_2).abs() < 1e-3, "{face:?}: {pitch}");
            assert!(
                (yaw - 0.7).abs() < 1e-3,
                "{face:?}: heading kept by the snap: {yaw}"
            );
            let view = rig.pose.view_matrix();
            let free = CameraPose::from_orbit(&rig.pose, rig.projection).view_matrix();
            assert!(
                free.abs_diff_eq(view, 1e-4),
                "{face:?}: from_orbit {free} != orbit {view}"
            );
            // The view matrix rows are the screen right / up.
            let (right, up) = rig.pose.view_plane_axes();
            assert!(
                right.distance(view.row(0).truncate()) < 1e-4,
                "{face:?}: right {right}"
            );
            assert!(
                up.distance(view.row(1).truncate()) < 1e-4,
                "{face:?}: up {up}"
            );
            // The heading: the screen right is the yaw-0.7 turntable right, flipped.
            let turntable = Quat::from_rotation_y(0.7) * Vec3::Z;
            assert!(right.distance(-turntable) < 1e-4, "{face:?}: {right}");
        }
    }

    /// Straight above, the pan / pose basis keeps the heading and agrees with the view matrix.
    /// Sign check: the turntable
    /// right at yaw 0.7 is `Ry(0.7) * Z = (sin, 0, cos)`, the screen right is its negation and
    /// `forward = −Y`, so `up = right × forward = (−cos 0.7, 0, sin 0.7)` = `orientation * Y`,
    /// the up hint `view_matrix` uses at the pole; the matrix rows are the arbiter.
    #[test]
    fn pole_view_basis_keeps_heading_and_matches_view_matrix() {
        use std::f32::consts::FRAC_PI_2;
        let pose = OrbitPose::from_yaw_pitch(0.7, FRAC_PI_2, 10.0, Vec3::ZERO);
        let (right, up) = pose.view_plane_axes();
        let expect_up = Vec3::new(-0.7_f32.cos(), 0.0, 0.7_f32.sin());
        assert!(up.distance(expect_up) < 1e-4, "up {up}");
        assert!(right.dot(up).abs() < 1e-5);
        // View-matrix rows: camera right is row 0, up is row 1 of the rotation part.
        let v = pose.view_matrix();
        let m_right = Vec3::new(v.x_axis.x, v.y_axis.x, v.z_axis.x);
        let m_up = Vec3::new(v.x_axis.y, v.y_axis.y, v.z_axis.y);
        assert!(m_right.distance(right) < 1e-4 && m_up.distance(up) < 1e-4);
    }

    /// Zero-safe basis: a zero distance never yields NaN axes.
    #[test]
    fn view_plane_axes_survive_a_zero_distance() {
        let pose = OrbitPose::from_yaw_pitch(0.7, 0.3, 0.0, Vec3::ZERO);
        let (right, up) = pose.view_plane_axes();
        assert!(right.is_finite() && up.is_finite(), "{right} {up}");
    }

    /// An upward coast at (or from an exact snap to) the top limit stops there instead of
    /// crossing the pole every frame (a 180° flip each step).
    #[test]
    fn an_upward_coast_at_the_top_limit_never_crosses_the_pole() {
        use std::f32::consts::FRAC_PI_2;
        for start in [DEFAULT_PITCH_LIMIT, FRAC_PI_2, DEFAULT_PITCH_LIMIT - 0.05] {
            let mut rig = OrbitRig::from_yaw_pitch(0.7, start, 10.0, Vec3::ZERO);
            rig.orbit_inertia(0.0, 3.0);
            let mut frames = 0;
            while rig.update_inertia(1.0 / 60.0) && frames < 120 {
                frames += 1;
                let (yaw, pitch) = yaw_pitch_from_orientation(rig.pose.orientation);
                assert!(
                    (yaw - 0.7).abs() < 1e-3,
                    "{start} frame {frames}: yaw {yaw}"
                );
                assert!(
                    pitch > 1.5 && pitch < FRAC_PI_2,
                    "{start} frame {frames}: {pitch}"
                );
            }
            assert!(frames > 0, "the coast ran");
        }
    }

    #[test]
    fn inertia_coasts_multiple_frames() {
        let mut rig = OrbitRig::default();
        let q0 = rig.pose.orientation;
        rig.orbit_inertia(4.8, 0.0);
        assert!(rig.update_inertia(1.0 / 60.0));
        assert_ne!(q0, rig.pose.orientation);
        let mut frames = 0_u32;
        while rig.update_inertia(1.0 / 60.0) {
            frames += 1;
            if frames > 300 {
                break;
            }
        }
        assert!(frames > 8, "expected multi-frame coast, got {frames}");
    }

    /// A coast ends at its time `T` (`navigation.coast.time`): within one frame of `T / dt`
    /// frames it reports rest, and further frames leave the pose exactly alone.
    #[test]
    fn a_coast_stops_exactly_at_its_time() {
        let dt = 1.0 / 60.0;
        let mut rig = OrbitRig::default();
        let t = rig.navigation.coast.time;
        rig.orbit_inertia(3.0, 0.0);
        let mut frames = 0_u32;
        while rig.update_inertia(dt) {
            frames += 1;
            assert!(frames < 100, "the coast did not stop");
        }
        let expected = (t / dt).ceil() as u32;
        assert!(
            frames + 1 >= expected && frames < expected + 1,
            "{frames} frames for T = {t} s ({expected} expected)"
        );
        assert!(!rig.has_inertia());
        let rest = rig.pose;
        for _ in 0..10 {
            assert!(!rig.update_dynamics(dt));
            assert_eq!(rig.pose, rest, "the pose drifted after the coast ended");
        }
    }

    /// The total turn of a coast is the closed form `v0 * T * R(1)`, whatever the frame rate.
    #[test]
    fn a_coast_travels_its_closed_form_distance() {
        use crate::camera_core::coast::coast_remaining_factor;
        for dt in [1.0 / 30.0, 1.0 / 60.0, 1.0 / 144.0] {
            let mut rig = OrbitRig::from_yaw_pitch(0.4, 0.2, 10.0, Vec3::ZERO);
            let c = rig.navigation.coast.sanitized();
            let v0 = 2.0;
            rig.orbit_inertia(v0, 0.0);
            let (yaw0, _) = yaw_pitch_from_orientation(rig.pose.orientation);
            while rig.update_inertia(dt) {}
            let (yaw1, pitch1) = yaw_pitch_from_orientation(rig.pose.orientation);
            let expected = v0 * c.time * coast_remaining_factor(1.0, c.softness);
            assert!(
                (yaw1 - yaw0 - expected).abs() < 5e-4,
                "dt {dt}: turned {} expected {expected}",
                yaw1 - yaw0
            );
            assert!((pitch1 - 0.2).abs() < 1e-5, "dt {dt}: pitch {pitch1}");
        }
    }

    /// A pan coast moves the target along the screen axes by `v0 * T * R(1)` in the seed's
    /// direction; a zoom coast scales the distance by `exp(v0 * T * R(1))`.
    #[test]
    fn pan_and_zoom_coasts_travel_the_seeded_direction() {
        use crate::camera_core::coast::coast_remaining_factor;
        let dt = 1.0 / 60.0;
        let mut rig = OrbitRig::from_yaw_pitch(0.4, 0.2, 10.0, Vec3::ZERO);
        let c = rig.navigation.coast.sanitized();
        let r1 = coast_remaining_factor(1.0, c.softness);
        let (right, up) = rig.pose.view_plane_axes();
        let d0 = rig.pose.distance;
        let s = d0 * rig.navigation.pan_sensitivity;
        let v0 = right * (-300.0 * s) + up * (100.0 * s);
        rig.pan_inertia(300.0, 100.0);
        rig.zoom_inertia(0.5);
        while rig.update_inertia(dt) {}
        let moved = rig.pose.target;
        let expected = v0 * (c.time * r1);
        assert!(
            moved.distance(expected) < 1e-3 * expected.length().max(1.0),
            "pan {moved} expected {expected}"
        );
        let want = d0 * (0.5 * c.time * r1).exp();
        assert!(
            (rig.pose.distance - rig.projection.clamp_distance(want)).abs() < 1e-3 * want,
            "zoom {} expected {want}",
            rig.pose.distance
        );
    }

    /// A dolly-zoom coast widens the lens and keeps the subject size
    /// (`distance * tan(fov / 2)` constant); `stop_inertia` ends it.
    #[test]
    fn a_dolly_zoom_coast_keeps_the_subject_size() {
        let dt = 1.0 / 60.0;
        let mut rig = OrbitRig::from_yaw_pitch(0.4, 0.2, 10.0, Vec3::ZERO);
        let size = |r: &OrbitRig| r.pose.distance * (r.projection.fov_y * 0.5).tan();
        let (fov0, k0) = (rig.projection.fov_y, size(&rig));
        rig.dolly_zoom_inertia(0.3);
        assert!(rig.has_inertia());
        for _ in 0..10 {
            assert!(rig.update_inertia(dt));
        }
        assert!(rig.projection.fov_y > fov0, "the lens widened");
        assert!((size(&rig) - k0).abs() < 1e-3 * k0, "subject size kept");
        rig.stop_inertia();
        let rest = (rig.pose, rig.projection.fov_y);
        assert!(!rig.update_inertia(dt));
        assert_eq!((rig.pose, rig.projection.fov_y), rest);
    }

    /// `animate_to` ends every coast (the transition would fight it) and runs.
    #[test]
    fn animate_to_stops_the_coasts() {
        let mut rig = OrbitRig::from_yaw_pitch(0.4, 0.2, 10.0, Vec3::ZERO);
        rig.orbit_inertia(2.0, 0.5);
        rig.pan_inertia(30.0, 10.0);
        rig.zoom_inertia(0.4);
        assert!(rig.has_inertia());
        rig.animate_to(1.0, 0.1, 12.0, Vec3::ONE);
        assert!(!rig.has_inertia(), "the coasts stopped");
        assert!(rig.is_animating());
        let mut rig = OrbitRig::from_yaw_pitch(0.4, 0.2, 10.0, Vec3::ZERO);
        rig.orbit_inertia(2.0, 0.5);
        rig.animate_to_view(crate::camera_core::ViewTarget::face(crate::camera_core::Face::Top, crate::camera_core::UpAxis::Y));
        assert!(!rig.has_inertia() && rig.is_animating());
    }

    /// A coast seed below `coast.min_speed`, or with `coast.time == 0`, starts nothing and
    /// leaves a running transition alone.
    #[test]
    fn a_disabled_or_negligible_seed_starts_no_coast() {
        let mut rig = OrbitRig::from_yaw_pitch(0.4, 0.2, 10.0, Vec3::ZERO);
        rig.navigation.coast.time = 0.0;
        rig.animate_to(1.0, 0.1, 12.0, Vec3::ONE);
        rig.orbit_inertia(5.0, 0.0);
        rig.pan_inertia(500.0, 0.0);
        rig.zoom_inertia(1.0);
        assert!(!rig.has_inertia(), "time 0 = no coast");
        assert!(
            rig.is_animating(),
            "no coast started, the transition runs on"
        );
        rig.navigation.coast = Default::default();
        rig.orbit_inertia(rig.navigation.coast.min_speed * 0.5, 0.0);
        assert!(!rig.has_inertia(), "below min_speed");
    }

    #[test]
    fn orbit_pose_is_copy() {
        fn is_copy<T: Copy>() {}
        is_copy::<OrbitPose>();
    }

    /// A rig mid-way through a view transition (Top, one frame in).
    fn rig_mid_transition() -> OrbitRig {
        use crate::camera_core::{Face, UpAxis, ViewTarget};
        let mut rig = OrbitRig::from_yaw_pitch(0.4, 0.2, 10.0, Vec3::ZERO);
        rig.animate_to_view(ViewTarget::face(Face::Top, UpAxis::Y));
        rig.update_dynamics(1.0 / 60.0);
        assert!(rig.is_animating());
        rig
    }

    /// A direct edit during a transition cancels it: the next frame keeps the edit instead of
    /// rebuilding the pose from the transition's start / target.
    #[test]
    fn a_direct_edit_cancels_a_running_transition() {
        // Orbit: the yaw step survives the next frame.
        let mut rig = rig_mid_transition();
        let (yaw0, _) = yaw_pitch_from_orientation(rig.pose.orientation);
        rig.orbit(0.3, 0.0);
        assert!(!rig.is_animating());
        rig.update_dynamics(1.0 / 60.0);
        let (yaw, _) = yaw_pitch_from_orientation(rig.pose.orientation);
        assert!(
            (yaw - (yaw0 + 0.3)).abs() < 1e-4,
            "orbit kept: {yaw0} -> {yaw}"
        );
        // Pan: the target move survives.
        let mut rig = rig_mid_transition();
        let t0 = rig.pose.target;
        rig.pan(20.0, 0.0);
        assert!(!rig.is_animating());
        let moved = rig.pose.target;
        assert!(moved.distance(t0) > 1e-4);
        rig.update_dynamics(1.0 / 60.0);
        assert_eq!(rig.pose.target, moved, "pan kept");
        // Zoom: the distance survives.
        let mut rig = rig_mid_transition();
        rig.zoom(0.5);
        assert!(!rig.is_animating());
        let d = rig.pose.distance;
        rig.update_dynamics(1.0 / 60.0);
        assert_eq!(rig.pose.distance, d, "zoom kept");
        // A coast started mid-transition cancels it too, so the coast is not lost.
        let mut rig = rig_mid_transition();
        rig.orbit_inertia(1.0, 0.0);
        assert!(!rig.is_animating());
        assert!(rig.update_dynamics(1.0 / 60.0), "the coast runs");
    }

    /// NaN / infinite deltas and rates are ignored by every edit and coast: the pose stays
    /// finite and unchanged and the rig stays at rest.
    #[test]
    fn non_finite_inputs_are_ignored() {
        for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut rig = OrbitRig::from_yaw_pitch(0.4, 0.2, 10.0, Vec3::ZERO);
            let before = rig.pose;
            let size = ViewportSize::new(100, 100);
            rig.orbit(bad, 0.0);
            rig.orbit(0.0, bad);
            rig.orbit_snapped_in(bad, 0.0, true, OrbitSnapMode::Axis);
            rig.pan(bad, 1.0);
            rig.zoom(bad);
            rig.zoom_at_cursor(ViewportPoint { x: bad, y: 1.0 }, size, 0.5);
            rig.zoom_at_cursor(ViewportPoint::ZERO, size, bad);
            rig.orbit_inertia(bad, 0.0);
            rig.pan_inertia(0.0, bad);
            rig.zoom_inertia(bad);
            rig.dolly_zoom_inertia(bad);
            assert_eq!(rig.pose, before, "{bad}");
            assert!(!rig.has_inertia() && !rig.is_animating(), "{bad}: at rest");
            assert!(!rig.update_dynamics(1.0 / 60.0));
            assert!(rig.pose.orientation.is_finite() && rig.pose.target.is_finite());
        }
    }

    /// The smoothed smart snap runs as a transition on the real frame time: after a few snap
    /// intents and a release mid-ease, `update_dynamics` alone lands the view on the snap
    /// target (it no longer stops part-way).
    #[test]
    fn smoothed_smart_snap_finishes_after_release() {
        let mut rig = OrbitRig::from_yaw_pitch(
            10.0_f32.to_radians(),
            5.0_f32.to_radians(),
            10.0,
            Vec3::ZERO,
        );
        rig.align_seconds = 0.1;
        let side = |rig: &OrbitRig| rig.pose.orientation * DEFAULT_OFFSET_AXIS;
        // Within 15° of two planes: the smart target is the +X axis.
        for (i, d) in [0.0, 0.01, 0.01].into_iter().enumerate() {
            rig.orbit_snapped_in(d, 0.0, i == 0, OrbitSnapMode::Smart);
            rig.update_dynamics(1.0 / 60.0);
        }
        assert!(rig.is_animating(), "eases, not snaps");
        assert!(side(&rig).distance(Vec3::X) > 1e-3, "released part-way");
        let mut frames = 0;
        while rig.update_dynamics(1.0 / 60.0) && frames < 600 {
            frames += 1;
        }
        assert!(!rig.is_animating(), "the ease ended");
        assert!(
            side(&rig).distance(Vec3::X) < 1e-4,
            "landed: {}",
            side(&rig)
        );
    }

    /// A snap gesture's free direction does not outlive other edits: snapped at A, orbited to
    /// B, a continued snap (`start = false`) starts from B instead of jumping back to A.
    #[test]
    fn a_stale_snap_never_jumps_back() {
        let mut rig = OrbitRig::from_yaw_pitch(0.3, 0.2, 10.0, Vec3::ZERO);
        let side = |rig: &OrbitRig| rig.pose.orientation * DEFAULT_OFFSET_AXIS;
        rig.orbit_snapped(0.0, 0.0, true);
        assert!(side(&rig).distance(Vec3::X) < 1e-5, "A = +X");
        rig.orbit(1.2, 0.0);
        rig.orbit_snapped(0.0, 0.0, false);
        // From yaw ~1.2 the nearest axis view is a quarter turn: −Z, not A.
        assert!(side(&rig).distance(Vec3::NEG_Z) < 1e-5, "{}", side(&rig));
    }

    /// Every coast reaches EXACT rest through `update_inertia` (no threshold).
    #[test]
    fn coasts_reach_exact_rest() {
        let mut rig = OrbitRig::from_yaw_pitch(0.4, 0.2, 10.0, Vec3::ZERO);
        rig.orbit_inertia(3.0, -1.0);
        rig.pan_inertia(200.0, -50.0);
        rig.zoom_inertia(0.4);
        rig.dolly_zoom_inertia(0.1);
        let mut frames = 0;
        while rig.update_inertia(1.0 / 60.0) {
            frames += 1;
            assert!(frames < 2000, "the coast did not stop");
        }
        let c = rig.coast;
        assert!(
            !c.turn.is_active() && !c.zoom.is_active() && !c.pan.is_active(),
            "every channel idle"
        );
        assert!(!c.dolly_zoom.is_active());
        assert_eq!(c.turn.velocity(), Vec2::ZERO);
        assert_eq!(c.pan.velocity(), Vec3::ZERO);
        assert!(!rig.has_inertia());
    }

    /// A `dt` that is not positive and finite changes nothing and reports no motion, also with
    /// a coast running (NaN / INF would poison the pose, a negative one would accelerate).
    #[test]
    fn bad_dt_leaves_the_pose_alone() {
        for dt in [0.0, -1.0 / 60.0, f32::NAN, f32::INFINITY] {
            let mut rig = OrbitRig::from_yaw_pitch(0.4, 0.2, 10.0, Vec3::ZERO);
            rig.orbit_inertia(2.0, 0.5);
            rig.pan_inertia(30.0, 10.0);
            rig.zoom_inertia(0.3);
            let before = rig.pose;
            assert!(!rig.update_inertia(dt), "update_inertia({dt})");
            assert!(!rig.update_dynamics(dt), "update_dynamics({dt})");
            assert_eq!(rig.pose, before, "dt {dt}");
            assert!(
                rig.has_inertia(),
                "dt {dt}: the coast is kept for a valid frame"
            );
        }
    }

    #[test]
    fn looking_builds_twist_free_turntable_across_all_quadrants_and_anti_parallel_x() {
        let target = Vec3::new(3.0, -2.0, 5.0);
        let dist = 14.0;
        let dirs = [
            -Vec3::X,
            Vec3::X, // offset = -Vec3::X (180 deg opposite DEFAULT_OFFSET_AXIS)
            Vec3::Z,
            -Vec3::Z,
            Vec3::Y,
            -Vec3::Y,
            Vec3::new(1.0, 0.4, 1.0),
            Vec3::new(-1.0, -0.6, 0.8),
            Vec3::new(0.7, 0.5, -0.9),
            Vec3::new(-0.4, -0.3, -1.2),
        ];
        for raw_fwd in dirs {
            let fwd = raw_fwd.normalize();
            let pose = OrbitPose::looking(target, fwd, dist);
            let expected_eye = target - fwd * dist;
            assert!(
                (pose.eye() - expected_eye).length() < 1e-4,
                "eye mismatch for forward {fwd:?}: got {:?}, expected {expected_eye:?}",
                pose.eye()
            );
            assert!(
                !pose.view_matrix().is_nan(),
                "view_matrix produced NaN for forward {fwd:?}"
            );
            // Local pitch axis (`orientation * Vec3::Z`) must remain horizontal (zero Y-roll twist).
            let pitch_axis = pose.orientation * Vec3::Z;
            assert!(
                pitch_axis.y.abs() < 1e-5,
                "non-zero roll twist in OrbitPose::looking for forward {fwd:?}: pitch_axis={pitch_axis:?}"
            );
            // Round-trip through OrbitRig yaw/pitch extraction must preserve the eye position.
            let rig = OrbitRig::from_pose(pose);
            let (yaw, pitch) = yaw_pitch_from_orientation(rig.pose.orientation);
            let rebuilt = OrbitPose::from_yaw_pitch(yaw, pitch, dist, target);
            assert!(
                (rebuilt.eye() - expected_eye).length() < 1e-4,
                "yaw/pitch roundtrip mismatch for forward {fwd:?}"
            );
        }
    }

    /// `from_view` inverts `CameraPose::from_orbit`, also at both poles where the forward has no
    /// heading (the yaw comes from the camera's screen right), and a yaw-only turn of the camera
    /// about the world up turns the orbit's heading by exactly that step (a host view cube).
    #[test]
    fn from_view_keeps_the_heading_at_both_poles() {
        use std::f32::consts::{FRAC_PI_2, PI, TAU};
        let wrap = |a: f32| (a + PI).rem_euclid(TAU) - PI;
        let target = Vec3::new(1.0, 2.0, -3.0);
        let step = 0.25;
        for pitch in [FRAC_PI_2, -FRAC_PI_2, 0.3] {
            for yaw in [0.0_f32, 0.7, -2.4] {
                let orbit = OrbitPose::from_yaw_pitch(yaw, pitch, 5.0, target);
                let cam = crate::camera_core::CameraPose::from_orbit(&orbit, PerspectiveSettings::default());
                let back = OrbitPose::from_view(target, cam.orientation, 5.0);
                let (y, p) = yaw_pitch_from_orientation(back.orientation);
                assert!(
                    wrap(y - yaw).abs() < 1e-4,
                    "yaw {yaw} pitch {pitch}: got {y}"
                );
                assert!(
                    (p - pitch).abs() < 1e-4,
                    "yaw {yaw} pitch {pitch}: got pitch {p}"
                );
                assert!((back.eye() - orbit.eye()).length() < 1e-3);
                let turned = Quat::from_rotation_y(step) * cam.orientation;
                let back = OrbitPose::from_view(target, turned, 5.0);
                let (y, p) = yaw_pitch_from_orientation(back.orientation);
                assert!(
                    wrap(y - yaw - step).abs() < 1e-4,
                    "yaw {yaw} pitch {pitch}: turned to {y}"
                );
                assert!((p - pitch).abs() < 1e-4);
            }
        }
    }
}
