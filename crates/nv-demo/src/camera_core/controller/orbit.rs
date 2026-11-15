//! [`HoudiniOrbit`]: the classic euler orbit backend of [`super::CameraController`].
//!
//! What: a thin wrapper that delegates every intent to [`OrbitRig`] and converts its
//! [`OrbitPose`] to the shared [`CameraPose`].
//!
//! Why: the orbit rig is the correctness anchor downstream; wrapping it (instead of
//! reimplementing) keeps the Houdini style identical to the plain rig (pinned by
//! `houdini_pose_matches_orbit`).
//!
//! How: `apply_intent` maps orbit-native intents onto rig calls (the release coasts
//! `OrbitInertia` / `PanInertia` / `ZoomInertia` / `DollyZoomInertia` seed the rig's
//! `OrbitCoast`, `Stop` ends it); fly / space intents (`Look*`, `Roll`, `Thrust`, `SpeedScale`)
//! are no-ops.
//! `from_pose` / `set_pose` reproject a free pose onto the orbit (target = eye + forward *
//! distance) through [`OrbitPose::from_view`], which keeps a Top / Bottom heading.
//!
//! Uses: `crate::camera_core::rig`, `crate::camera_core::intent`. Used by: `super::facade`.

use crate::camera_core::intent::CameraIntent;
use crate::camera_core::pose::CameraPose;
use crate::camera_core::projection::PerspectiveSettings;
use crate::camera_core::rig::{OrbitPose, OrbitRig};
use crate::camera_core::viewport::{ViewportPoint, ViewportSize};

/// Classic orbit backend: a thin wrapper over [`OrbitRig`].
///
/// Delegates every operation to [`OrbitRig`] so its tumble / dolly / pan feel and its inertia
/// are byte-for-byte the rig's; [`HoudiniOrbit::pose`] converts the rig's [`OrbitPose`] into the
/// shared [`CameraPose`].
#[derive(Debug, Clone, Default)]
pub struct HoudiniOrbit {
    /// The wrapped orbit rig (public so callers needing the full orbit API can reach it;
    /// [`HoudiniOrbit::pose`] is the backend-agnostic accessor).
    pub rig: OrbitRig,
}

impl HoudiniOrbit {
    /// Wrap an existing [`OrbitRig`].
    pub fn new(rig: OrbitRig) -> Self {
        Self { rig }
    }

    /// Build from a free pose: the pivot sits `DEFAULT_ORBIT_DISTANCE` (clamped) ahead of the
    /// eye and the rig takes the pose's lens (`projection_from_pose`). Roll is dropped (a
    /// turntable cannot roll).
    pub fn from_pose(pose: CameraPose) -> Self {
        let projection = super::projection_from_pose(pose, PerspectiveSettings::default());
        let distance =
            projection.clamp_distance(crate::camera_core::convention::DEFAULT_ORBIT_DISTANCE);
        let target = pose.eye + pose.forward() * distance;
        let mut rig = OrbitRig::from_pose(OrbitPose::from_view(target, pose.orientation, distance));
        rig.projection = projection;
        Self { rig }
    }

    /// Move the orbit to `pose`, mutating ONLY the rig pose: the pivot is re-placed at the
    /// current distance (clamped to the lens range, like [`super::QuatOrbit::set_pose`]) ahead
    /// of the new eye; the coast, any view animation and any snap gesture end (the restored pose
    /// is a direct edit). The rig's lens, pitch limit, inertia tuning and transition speed are
    /// kept (a rebuild through `OrbitRig::from_pose` would reset them on every restore).
    pub fn set_pose(&mut self, pose: CameraPose) {
        let distance = self.rig.projection.clamp_distance(self.rig.pose.distance);
        let target = pose.eye + pose.forward() * distance;
        self.rig.pose = OrbitPose::from_view(target, pose.orientation, distance);
        self.rig.take_over();
    }

    /// Apply one intent. Orbit-native intents drive the rig (`DollyZoom` dollies about the pivot
    /// and widens the rig's lens, `super::dolly_zoom`); fly / space intents are no-ops (this
    /// backend is not free-flying).
    pub fn apply_intent(&mut self, intent: CameraIntent, viewport: ViewportSize) {
        match intent {
            CameraIntent::Orbit {
                yaw_delta,
                pitch_delta,
            } => self.rig.orbit(yaw_delta, pitch_delta),
            CameraIntent::OrbitInertia {
                yaw_rate,
                pitch_rate,
            } => self.rig.orbit_inertia(yaw_rate, pitch_rate),
            CameraIntent::OrbitSnap {
                yaw_delta,
                pitch_delta,
                start,
                mode,
            } => self
                .rig
                .orbit_snapped_in(yaw_delta, pitch_delta, start, mode),
            CameraIntent::Pan { dx_px, dy_px } => self.rig.pan(dx_px, dy_px),
            CameraIntent::PanInertia { dx_rate, dy_rate } => self.rig.pan_inertia(dx_rate, dy_rate),
            CameraIntent::Zoom { factor } => self.rig.zoom(factor),
            CameraIntent::ZoomInertia { rate } => self.rig.zoom_inertia(rate),
            CameraIntent::DollyZoomInertia { fov_rate } => self.rig.dolly_zoom_inertia(fov_rate),
            CameraIntent::Stop => self.rig.stop_inertia(),
            CameraIntent::ZoomAtCursor {
                cursor_x,
                cursor_y,
                factor,
            } => {
                self.rig.zoom_at_cursor(
                    ViewportPoint {
                        x: cursor_x,
                        y: cursor_y,
                    },
                    viewport,
                    factor,
                );
            }
            CameraIntent::FrameBounds { min, max, margin } => {
                self.rig
                    .frame_bounds(min, max, viewport.width, viewport.height, margin);
            }
            CameraIntent::View(view) => self.rig.animate_to_view(view),
            CameraIntent::DollyZoom { dfov } => {
                if let Some((distance, fov_y)) = super::dolly_zoom(
                    self.rig.pose.distance,
                    self.rig.projection.fov_y,
                    dfov,
                    self.rig.projection,
                ) {
                    // A direct edit: a running view transition rebuilds the whole pose every
                    // frame and would overwrite it, so it ends here with the coasts (the
                    // coast step itself applies the same lens rule, `OrbitRig::update_inertia`).
                    self.rig.take_over();
                    self.rig.pose.distance = self.rig.projection.clamp_distance(distance);
                    self.rig.projection.fov_y = fov_y;
                }
            }
            CameraIntent::Look { .. }
            | CameraIntent::LookInertia { .. }
            | CameraIntent::LookRate { .. }
            | CameraIntent::Roll { .. }
            | CameraIntent::Thrust { .. }
            | CameraIntent::SpeedScale(_) => {}
        }
    }

    /// The orbit pose as the shared [`CameraPose`].
    pub fn pose(&self) -> CameraPose {
        CameraPose::from_orbit(&self.rig.pose, self.rig.projection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    #[test]
    fn houdini_pose_matches_orbit() {
        // The Houdini backend's pose().view_proj must equal the wrapped
        // OrbitRig.view_proj for the same state (view continuity anchor).
        let rig = OrbitRig::from_yaw_pitch(0.5, 0.25, 20.0, Vec3::new(2.0, 1.0, 0.0));
        let backend = HoudiniOrbit::new(rig.clone());
        let a = rig.view_proj(1280, 720);
        let b = backend.pose().view_proj(1280, 720, rig.projection);
        let mut max = 0.0_f32;
        for (x, y) in a.to_cols_array().iter().zip(b.to_cols_array().iter()) {
            max = max.max((x - y).abs());
        }
        assert!(max < 1e-3, "houdini pose mismatch, max delta {max}");
    }
}
