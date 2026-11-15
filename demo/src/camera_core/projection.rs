//! Perspective projection settings shared by rigs.
//!
//! Why: every rig needs the same fov / clip / distance-range record. How: a plain `Copy` struct
//! with a panic-free distance clamp. Used by `fit`, the rigs and the controller.

/// Perspective settings (WebGPU RH). The matrix builders render with **reverse-Z**
/// (near→NDC depth 1, far→0) by passing `near`/`far` swapped to `perspective_rh`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerspectiveSettings {
    /// Vertical field of view in radians.
    pub fov_y: f32,
    /// Near clip distance (world units).
    pub near: f32,
    /// Far clip distance (world units).
    pub far: f32,
    /// Smallest allowed pivot distance (zoom-in limit).
    pub distance_min: f32,
    /// Largest allowed pivot distance (zoom-out limit).
    pub distance_max: f32,
}

impl Default for PerspectiveSettings {
    fn default() -> Self {
        Self {
            fov_y: 45.0_f32.to_radians(),
            near: 0.1,
            // A wide default range: a large scene frames from up to 600 units away and is
            // inspected from 1 unit; `far` covers a scene of radius ~900 at the max distance.
            // Hosts with other scales set their own bounds.
            far: 1500.0,
            distance_min: 1.0,
            distance_max: 600.0,
        }
    }
}

impl PerspectiveSettings {
    /// `distance` within `[distance_min, distance_max]`; host-set bounds that are inverted or
    /// NaN never panic (`crate::camera_core::num::clamp`).
    pub fn clamp_distance(self, distance: f32) -> f32 {
        crate::camera_core::num::clamp(distance, self.distance_min, self.distance_max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_distance_survives_bad_host_bounds() {
        // Inverted or NaN host bounds: `f32::clamp` would panic on these.
        let inverted = PerspectiveSettings {
            distance_min: 10.0,
            distance_max: 1.0,
            ..PerspectiveSettings::default()
        };
        assert_eq!(inverted.clamp_distance(5.0), 10.0);
        let nan = PerspectiveSettings {
            distance_min: f32::NAN,
            ..PerspectiveSettings::default()
        };
        assert_eq!(nan.clamp_distance(5.0), 5.0);
        let s = PerspectiveSettings::default();
        assert_eq!(s.clamp_distance(0.0), s.distance_min);
        assert_eq!(s.clamp_distance(1e9), s.distance_max);
    }
}
