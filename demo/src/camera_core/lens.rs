//! Lens math and the flat / perspective lens ease.
//!
//! What: pure lens conversions (focal length <-> angle of view on a film back, horizontal <->
//! vertical FOV, the After Effects "zoom in pixels" model, focal presets), the dolly-zoom distance
//! factor (`distance * tan(fov / 2)` invariant), the FOV-consistent orbit step scale, the
//! top-down gate predicate, the one viewport FOV range ([`FOV_MIN`]..=[`FOV_MAX`], shared with
//! the dolly-zoom intent), [`LensMode`] (`Flat` near-orthographic vs `Perspective`) and
//! [`LensEase`] (the per-frame, dt-based ease of the FOV toward the mode's target, adopting the
//! controller's FOV, the FOV's one owner, before every step).
//!
//! Why: every viewport needs the same lens numbers, and the flat-map / 3D-tumble lens switch of
//! node-graph style hosts was carried per host. One copy here; egui controls (the linked lens
//! fields) stay in their widget crates and call these functions.
//!
//! How: all angles are radians, all functions are total: out-of-range or NaN inputs are clamped
//! into documented ranges (`crate::camera_core::num::clamp`, NaN -> lower bound) so the result is always finite;
//! the stateful [`LensEase`] ignores non-finite inputs instead. The ease is a frame-rate
//! independent exponential approach (`gap *= exp(-rate * dt)`) computed on the gap to the target
//! (never `fov += delta`, which can stall in f32) and lands EXACTLY on the target once the gap is
//! below [`LAND_EPS_RAD`].
//!
//! Decision, no `ProjectionKind`: [`crate::camera_core::PerspectiveSettings`] is perspective-only and the
//! pose / matrix path has no orthographic branch. An enum here would be a flag no matrix honours.
//! The orthographic look is [`LensMode::Flat`]: a [`FOV_FLAT`] (2°) perspective with the distance
//! compensated by [`dolly_zoom_factor`], which keeps ONE projection codepath (picking, rays,
//! reverse-Z) for both looks. A true orthographic matrix, if a host ever needs one, belongs in
//! `projection` / `pose`, not here.
//!
//! Uses: `crate::camera_core::num`, glam. Used by: the dolly-zoom intent, the navigation layer and hosts
//! (lens widgets, node-graph viewports).

use std::f32::consts::{FRAC_PI_3, PI};

use glam::Vec3;

use crate::camera_core::num::clamp;

/// One degree in radians (`PI / 180`), for the documented degree constants below.
const DEG: f32 = PI / 180.0;

/// Width of the reference film back in millimetres (full-frame 35 mm still, measured
/// horizontally). The focal presets and the pixel-zoom model assume it.
pub const FILM_WIDTH_MM: f32 = 36.0;

/// Height of the reference film back in millimetres (full-frame 35 mm still). Pass it to
/// [`angle_of_view`] / [`focal_for_angle`] to convert a focal length to / from a VERTICAL FOV
/// (`CameraPose::fov_y`, `PerspectiveSettings::fov_y`).
pub const FILM_HEIGHT_MM: f32 = 24.0;

/// The default lens focal length in millimetres (a "normal" lens).
pub const DEFAULT_FOCAL_MM: f32 = 50.0;

/// Focal-length presets (After Effects' camera presets): display name, millimetres.
pub const PRESETS: [(&str, f32); 9] = [
    ("15mm", 15.0),
    ("20mm", 20.0),
    ("24mm", 24.0),
    ("28mm", 28.0),
    ("35mm", 35.0),
    ("50mm", 50.0),
    ("80mm", 80.0),
    ("135mm", 135.0),
    ("200mm", 200.0),
];

/// Smallest focal length (mm) the conversions accept.
pub const FOCAL_MIN_MM: f32 = 0.1;
/// Largest focal length (mm) the conversions accept.
pub const FOCAL_MAX_MM: f32 = 10_000.0;
/// Smallest angle of view (0.1°, radians) the conversions accept or return.
pub const ANGLE_MIN_RAD: f32 = 0.1 * DEG;
/// Largest angle of view (179°, radians) the conversions accept or return; below 180° so
/// `tan(angle / 2)` stays finite.
pub const ANGLE_MAX_RAD: f32 = 179.0 * DEG;

/// Smallest film dimension (mm); a zero film would make every angle zero.
const FILM_MIN_MM: f32 = 0.001;
/// Largest film dimension (mm); keeps an infinite input finite.
const FILM_MAX_MM: f32 = 1_000.0;
/// Smallest image width (pixels) of the pixel-zoom model (the divisor of `focal_for_zoom`).
const WIDTH_MIN_PX: f32 = 1.0;
/// Largest image width (pixels) of the pixel-zoom model; keeps an infinite input finite.
const WIDTH_MAX_PX: f32 = 1.0e6;
/// Smallest zoom (pixels) of the pixel-zoom model (the divisor of `angle_for_zoom`).
const ZOOM_MIN_PX: f32 = 1.0e-6;
/// Largest zoom (pixels) of the pixel-zoom model; keeps an infinite input finite.
const ZOOM_MAX_PX: f32 = 1.0e9;
/// Smallest aspect ratio (width / height) the FOV conversions accept.
const ASPECT_MIN: f32 = 1.0e-3;
/// Largest aspect ratio (width / height) the FOV conversions accept.
const ASPECT_MAX: f32 = 1.0e3;

/// Narrowest vertical FOV (1°, radians) of a viewport: THE lower bound of the one FOV range,
/// shared by the dolly-zoom intent (`CameraIntent::DollyZoom`), [`LensEase`] (current FOV and
/// perspective target) and every other place that limits a viewport's `fov_y`. Below
/// [`FOV_FLAT`], so the flat lens is inside the range.
pub const FOV_MIN: f32 = DEG;
/// Widest vertical FOV (120°, radians) of a viewport: THE upper bound of the one FOV range (see
/// [`FOV_MIN`]). Wider reads as a fisheye and `tan(fov / 2)` grows steeply toward 180°.
pub const FOV_MAX: f32 = 120.0 * DEG;
/// The resting near-orthographic lens of [`LensMode::Flat`] (2°, radians): a flat "2D map"
/// look that still uses the perspective codepath.
pub const FOV_FLAT: f32 = 2.0 * DEG;
/// The default lens of [`LensMode::Perspective`] (60°, radians): the opened 3D tumble view.
pub const FOV_PERSPECTIVE: f32 = FRAC_PI_3;
/// Default ease duration (seconds) of [`LensEase`]: the FOV covers ~98 % of the gap in it.
pub const DEFAULT_EASE_SECS: f32 = 0.35;
/// Remaining FOV gap (radians) below which [`LensEase`] lands exactly on its target. Far above
/// the f32 spacing of any FOV in `(0, PI)` (~2.4e-7), so the approach can never stall short of it.
pub const LAND_EPS_RAD: f32 = 1.0e-5;
/// `forward . plane_normal` below `-TOP_DOWN_COS` counts as looking straight down onto the
/// plane (about 2.6° off the normal), the gate of a flatten ([`is_top_down`]).
pub const TOP_DOWN_COS: f32 = 0.999;

/// Ease rate constant: `rate = EASE_K / ease_secs`, so `exp(-EASE_K)` (~1.8 %) of the gap is
/// left after one `ease_secs`.
const EASE_K: f32 = 4.0;

/// `angle` (radians) clamped into `[ANGLE_MIN_RAD, ANGLE_MAX_RAD]` (NaN -> the minimum).
fn clamp_angle(angle: f32) -> f32 {
    clamp(angle, ANGLE_MIN_RAD, ANGLE_MAX_RAD)
}

/// Angle of view (radians) of a `focal_mm` lens across a film dimension of `film_mm`
/// (`2 atan(film / 2f)`). Pass [`FILM_HEIGHT_MM`] for the vertical FOV, [`FILM_WIDTH_MM`] for the
/// horizontal one. Inputs are clamped (focal into `[FOCAL_MIN_MM, FOCAL_MAX_MM]`), the result
/// into `[ANGLE_MIN_RAD, ANGLE_MAX_RAD]`.
pub fn angle_of_view(focal_mm: f32, film_mm: f32) -> f32 {
    let f = clamp(focal_mm, FOCAL_MIN_MM, FOCAL_MAX_MM);
    let film = clamp(film_mm, FILM_MIN_MM, FILM_MAX_MM);
    clamp_angle(2.0 * (film * 0.5 / f).atan())
}

/// Focal length (mm) giving an angle of view of `angle` radians across `film_mm`; the inverse of
/// [`angle_of_view`]. Inputs and result are clamped into the documented ranges.
pub fn focal_for_angle(angle: f32, film_mm: f32) -> f32 {
    let film = clamp(film_mm, FILM_MIN_MM, FILM_MAX_MM);
    let half = clamp_angle(angle) * 0.5;
    clamp(film * 0.5 / half.tan(), FOCAL_MIN_MM, FOCAL_MAX_MM)
}

/// Vertical FOV (radians) of a horizontal FOV `fov_x` on a viewport of `aspect` (width / height):
/// `2 atan(tan(fov_x / 2) / aspect)`. Inputs are clamped; the result stays a valid angle.
pub fn vertical_fov(fov_x: f32, aspect: f32) -> f32 {
    let a = clamp(aspect, ASPECT_MIN, ASPECT_MAX);
    clamp_angle(2.0 * ((clamp_angle(fov_x) * 0.5).tan() / a).atan())
}

/// Horizontal FOV (radians) of a vertical FOV `fov_y` on a viewport of `aspect` (width / height);
/// the inverse of [`vertical_fov`].
pub fn horizontal_fov(fov_y: f32, aspect: f32) -> f32 {
    let a = clamp(aspect, ASPECT_MIN, ASPECT_MAX);
    clamp_angle(2.0 * ((clamp_angle(fov_y) * 0.5).tan() * a).atan())
}

/// Zoom (pixels: the distance at which the image plane is 100 %) of a `focal_mm` lens on an
/// image `width_px` wide, on the [`FILM_WIDTH_MM`] film (`width * focal / FILM_WIDTH_MM`).
pub fn zoom_for_focal(width_px: f32, focal_mm: f32) -> f32 {
    let w = clamp(width_px, WIDTH_MIN_PX, WIDTH_MAX_PX);
    w * clamp(focal_mm, FOCAL_MIN_MM, FOCAL_MAX_MM) / FILM_WIDTH_MM
}

/// Focal length (mm) of a pixel `zoom` on an image `width_px` wide; the inverse of
/// [`zoom_for_focal`], clamped into the focal range.
pub fn focal_for_zoom(width_px: f32, zoom: f32) -> f32 {
    let w = clamp(width_px, WIDTH_MIN_PX, WIDTH_MAX_PX);
    let z = clamp(zoom, ZOOM_MIN_PX, ZOOM_MAX_PX);
    clamp(z * FILM_WIDTH_MM / w, FOCAL_MIN_MM, FOCAL_MAX_MM)
}

/// Horizontal angle of view (radians) of a pixel `zoom` on an image `width_px` wide
/// (`2 atan(width / 2 / zoom)`).
pub fn angle_for_zoom(width_px: f32, zoom: f32) -> f32 {
    let w = clamp(width_px, WIDTH_MIN_PX, WIDTH_MAX_PX);
    let z = clamp(zoom, ZOOM_MIN_PX, ZOOM_MAX_PX);
    clamp_angle(2.0 * (w * 0.5 / z).atan())
}

/// Pixel zoom giving a horizontal angle of view of `angle` radians on an image `width_px` wide;
/// the inverse of [`angle_for_zoom`] (the angle is clamped into the angle range first).
pub fn zoom_for_angle(width_px: f32, angle: f32) -> f32 {
    let w = clamp(width_px, WIDTH_MIN_PX, WIDTH_MAX_PX);
    w * 0.5 / (clamp_angle(angle) * 0.5).tan()
}

/// The distance multiplier that keeps the on-screen size of the subject at the pivot when the
/// FOV changes from `from_fov` to `to_fov` (radians): `tan(from / 2) / tan(to / 2)`, i.e. the
/// `distance * tan(fov / 2)` invariant of a dolly zoom (`new_distance = distance * factor`).
/// Finite angles are clamped into the angle range; a non-finite one returns `1.0` (no change).
pub fn dolly_zoom_factor(from_fov: f32, to_fov: f32) -> f32 {
    if !(from_fov.is_finite() && to_fov.is_finite()) {
        return 1.0;
    }
    (clamp_angle(from_fov) * 0.5).tan() / (clamp_angle(to_fov) * 0.5).tan()
}

/// Scale of an orbit's angular step at `fov_y` (radians) relative to [`FOV_PERSPECTIVE`]:
/// `tan(fov / 2) / tan(FOV_PERSPECTIVE / 2)`. Multiplying a pixel-drag orbit by it makes one
/// pixel sweep the same on-screen distance at any FOV, so a near-orthographic lens does not spin
/// wildly. A non-finite FOV returns `1.0`.
pub fn orbit_step_scale(fov_y: f32) -> f32 {
    if !fov_y.is_finite() {
        return 1.0;
    }
    (clamp_angle(fov_y) * 0.5).tan() / (FOV_PERSPECTIVE * 0.5).tan()
}

/// True when `forward` looks straight down onto a plane with normal `plane_normal` (within
/// [`TOP_DOWN_COS`]): the gate under which a [`LensMode::Flat`] flatten may run (a flat lens on a
/// tilted, edge-on plane would read as distorted). Zero or non-finite vectors return `false`.
pub fn is_top_down(forward: Vec3, plane_normal: Vec3) -> bool {
    let f = forward.normalize_or_zero();
    let n = plane_normal.normalize_or_zero();
    // `normalize_or_zero` maps non-finite input to zero, so a zero dot (never below -COS) is false.
    f.dot(n) < -TOP_DOWN_COS
}

/// The lens intent: where the FOV wants to be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum LensMode {
    /// The near-orthographic "2D map" look: [`FOV_FLAT`] (2°). Hosts gate it with
    /// [`LensEase::step_gated`] so it only flattens once the view is settled top-down.
    Flat,
    /// The opened 3D view at the adjustable perspective FOV ([`FOV_PERSPECTIVE`] = 60° by
    /// default, within [`FOV_MIN`]..=[`FOV_MAX`]).
    #[default]
    Perspective,
}

/// The per-frame FOV ease toward the [`LensMode`] target.
///
/// The FOV has ONE owner, the controller's projection (`CameraController::projection`): a
/// `DollyZoom` intent or a host edit changes it between frames. So every step first ADOPTS the
/// controller's current FOV ([`LensEase::step`] takes it, see [`LensEase::adopt`]) and returns the
/// new FOV for the host to push back (`CameraController::set_projection`); an external change
/// in [`LensMode::Perspective`] becomes the new perspective target, so it is never eased away.
///
/// Time-free: the caller passes `dt`. Frame-rate independent (`gap *= exp(-rate * dt)`, so two
/// half steps equal one full step) and lands exactly on the target, which is clamped into the
/// controller's reachable FOVs ([`LensEase::set_reach`]). To keep the subject's size while the
/// FOV moves, the host scales its pivot distance by [`dolly_zoom_factor`] of the FOV before and
/// after the step; `crate::camera_core::CameraDriver` does all of this (it pushes each step as a
/// `CameraIntent::DollyZoom` on the orbit backends).
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LensEase {
    /// Current lens intent.
    mode: LensMode,
    /// Current FOV (radians), always in `[FOV_MIN, FOV_MAX]`.
    fov: f32,
    /// The perspective target (radians) in `[FOV_MIN, FOV_MAX]`.
    perspective_fov: f32,
    /// Ease duration in seconds; 0 = land instantly.
    ease_secs: f32,
    /// The FOVs the controller can reach (`CameraController::fov_range`), within
    /// `[FOV_MIN, FOV_MAX]`: the target is clamped into it, so an unreachable target is reached.
    reach: (f32, f32),
}

impl Default for LensEase {
    /// A perspective lens resting at [`FOV_PERSPECTIVE`].
    fn default() -> Self {
        Self::new(LensMode::Perspective)
    }
}

impl LensEase {
    /// A lens resting at the target of `mode` (perspective target [`FOV_PERSPECTIVE`], ease
    /// [`DEFAULT_EASE_SECS`]).
    pub fn new(mode: LensMode) -> Self {
        let mut ease = Self {
            mode,
            fov: FOV_PERSPECTIVE,
            perspective_fov: FOV_PERSPECTIVE,
            ease_secs: DEFAULT_EASE_SECS,
            reach: (FOV_MIN, FOV_MAX),
        };
        ease.fov = ease.target();
        ease
    }

    /// Current lens intent.
    pub fn mode(&self) -> LensMode {
        self.mode
    }

    /// Change the lens intent; the FOV eases there in [`LensEase::step`].
    pub fn set_mode(&mut self, mode: LensMode) {
        self.mode = mode;
    }

    /// Current FOV (radians): the value the host renders with.
    pub fn fov(&self) -> f32 {
        self.fov
    }

    /// Set `fov` (radians) as the current FOV without easing and without touching the target;
    /// clamped into `[FOV_MIN, FOV_MAX]`, a non-finite value is ignored.
    pub fn set_fov(&mut self, fov: f32) {
        if fov.is_finite() {
            self.fov = clamp(fov, FOV_MIN, FOV_MAX);
        }
    }

    /// Adopt the controller's current FOV (radians, the FOV's owner): when it differs from the
    /// ease's (a `DollyZoom` or a host edit since the last step), it becomes the current FOV, and
    /// in [`LensMode::Perspective`] also the perspective target, so the change sticks instead of
    /// being eased back. In [`LensMode::Flat`] the ease then carries it back to [`FOV_FLAT`] (the
    /// flat intent wins). Clamped into `[FOV_MIN, FOV_MAX]`; a non-finite value is ignored.
    ///
    /// A difference of at most [`LAND_EPS_RAD`] is not a change: it is the rounding of the
    /// host's round trip through the controller (`a + (b - a)` is not always `b` in f32), and
    /// adopting it would retarget a running perspective ease by a rounding error.
    pub fn adopt(&mut self, current_fov: f32) {
        if !current_fov.is_finite() {
            return;
        }
        let fov = clamp(current_fov, FOV_MIN, FOV_MAX);
        if (fov - self.fov).abs() > LAND_EPS_RAD {
            self.fov = fov;
            if self.mode == LensMode::Perspective {
                self.perspective_fov = fov;
            }
        }
    }

    /// The perspective target (radians).
    pub fn perspective_fov(&self) -> f32 {
        self.perspective_fov
    }

    /// Set the perspective target (radians), clamped into `[FOV_MIN, FOV_MAX]`; a non-finite
    /// value is ignored. Drives the ease TARGET.
    pub fn set_perspective_fov(&mut self, fov: f32) {
        if fov.is_finite() {
            self.perspective_fov = clamp(fov, FOV_MIN, FOV_MAX);
        }
    }

    /// Move the perspective target by `delta` radians (a dolly-zoom drag); a non-finite delta is
    /// ignored.
    pub fn nudge_perspective_fov(&mut self, delta: f32) {
        if delta.is_finite() {
            self.set_perspective_fov(self.perspective_fov + delta);
        }
    }

    /// Ease duration in seconds (0 = land instantly).
    pub fn ease_secs(&self) -> f32 {
        self.ease_secs
    }

    /// Set the ease duration in seconds; 0 lands instantly (animations off). A negative or
    /// non-finite value is ignored.
    pub fn set_ease_secs(&mut self, secs: f32) {
        if secs.is_finite() && secs >= 0.0 {
            self.ease_secs = secs;
        }
    }

    /// Set the FOVs (radians) the controller can reach (`CameraController::fov_range`), so the
    /// ease never chases a target the controller would clamp (it would never land and keep the
    /// host painting). Clamped into `[FOV_MIN, FOV_MAX]`; a non-finite or inverted range is
    /// ignored. Default: the whole FOV range.
    pub fn set_reach(&mut self, min: f32, max: f32) {
        if min.is_finite() && max.is_finite() && min <= max {
            self.reach = (clamp(min, FOV_MIN, FOV_MAX), clamp(max, FOV_MIN, FOV_MAX));
        }
    }

    /// The FOV (radians) the current intent wants: [`FOV_FLAT`] or the perspective target,
    /// clamped into the reachable range ([`LensEase::set_reach`]).
    pub fn target(&self) -> f32 {
        let wanted = match self.mode {
            LensMode::Flat => FOV_FLAT,
            LensMode::Perspective => self.perspective_fov,
        };
        clamp(wanted, self.reach.0, self.reach.1)
    }

    /// True while the FOV is not on its target (the ease lands exactly, so this is exact).
    pub fn is_easing(&self) -> bool {
        self.fov != self.target()
    }

    /// Advance the ease by `dt` seconds from the controller's `current_fov`; equal to
    /// [`LensEase::step_gated`] with the flatten allowed. Returns the new FOV.
    pub fn step(&mut self, dt: f32, current_fov: f32) -> f32 {
        self.step_gated(dt, current_fov, true)
    }

    /// Adopt the controller's `current_fov` ([`LensEase::adopt`]), then advance the ease by `dt`
    /// seconds and return the new FOV (radians) for the host to push into the controller's
    /// projection. `flat_allowed: false` holds the FOV while the intent is [`LensMode::Flat`]
    /// (pass [`is_top_down`] of the camera: no flat lens on a tilted plane); it never holds a
    /// perspective ease. A zero, negative or non-finite `dt` only adopts. [`LensEase::is_easing`]
    /// tells whether later frames are still needed (exact: the ease lands on its target).
    pub fn step_gated(&mut self, dt: f32, current_fov: f32, flat_allowed: bool) -> f32 {
        self.adopt(current_fov);
        if dt <= 0.0 || !dt.is_finite() {
            return self.fov;
        }
        if self.mode == LensMode::Flat && !flat_allowed {
            return self.fov;
        }
        let target = self.target();
        let gap = self.fov - target;
        if gap == 0.0 {
            return self.fov;
        }
        // `EASE_K / 0` is +inf for an instant ease: `exp(-inf) = 0` lands this step.
        let keep = (-(EASE_K / self.ease_secs) * dt).exp();
        // `keep` is in [0, 1] (never NaN: the rate is positive or +inf, dt positive and finite).
        let gap = gap * keep;
        self.fov = if gap.abs() <= LAND_EPS_RAD {
            target
        } else {
            target + gap
        };
        self.fov
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 1e-4;

    /// 50 mm on a full-frame back: 39.598° horizontal, 26.991° vertical; round trips agree.
    #[test]
    fn focal_and_angle_round_trip() {
        let h = angle_of_view(50.0, FILM_WIDTH_MM);
        assert!(
            (h.to_degrees() - 39.597_75).abs() < 1e-3,
            "{}",
            h.to_degrees()
        );
        let v = angle_of_view(50.0, FILM_HEIGHT_MM);
        assert!(
            (v.to_degrees() - 26.991_45).abs() < 1e-3,
            "{}",
            v.to_degrees()
        );
        assert!((focal_for_angle(h, FILM_WIDTH_MM) - 50.0).abs() < 1e-3);
        assert!((focal_for_angle(v, FILM_HEIGHT_MM) - 50.0).abs() < 1e-3);
        for (_, mm) in PRESETS {
            let a = angle_of_view(mm, FILM_HEIGHT_MM);
            assert!(
                (focal_for_angle(a, FILM_HEIGHT_MM) - mm).abs() < mm * 1e-4,
                "{mm}"
            );
        }
        // Longer lens, narrower angle.
        assert!(angle_of_view(200.0, FILM_HEIGHT_MM) < angle_of_view(15.0, FILM_HEIGHT_MM));
    }

    /// The pixel-zoom model: 50 mm on 1920 px is 2666.67 px; its angle equals the film angle.
    #[test]
    fn pixel_zoom_model_agrees_with_film_angle() {
        let w = 1920.0;
        let z = zoom_for_focal(w, DEFAULT_FOCAL_MM);
        assert!((z - 2_666.666_7).abs() < 1e-2, "{z}");
        assert!((focal_for_zoom(w, z) - DEFAULT_FOCAL_MM).abs() < 1e-3);
        let a = angle_for_zoom(w, z);
        assert!((a - angle_of_view(DEFAULT_FOCAL_MM, FILM_WIDTH_MM)).abs() < 1e-5);
        assert!((zoom_for_angle(w, a) - z).abs() < 0.05);
    }

    /// Horizontal <-> vertical conversions invert each other; aspect 1 is the identity; a wide
    /// viewport has the wider horizontal angle.
    #[test]
    fn horizontal_and_vertical_fov_invert() {
        let fov_y = 45.0_f32.to_radians();
        let fov_x = horizontal_fov(fov_y, 16.0 / 9.0);
        assert!(fov_x > fov_y);
        assert!((vertical_fov(fov_x, 16.0 / 9.0) - fov_y).abs() < 1e-5);
        assert!((horizontal_fov(fov_y, 1.0) - fov_y).abs() < 1e-6);
    }

    /// Garbage in, finite and in-range out.
    #[test]
    fn degenerate_inputs_stay_finite() {
        let bad = [0.0, -5.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY];
        for x in bad {
            for y in bad {
                for v in [
                    angle_of_view(x, y),
                    focal_for_angle(x, y),
                    vertical_fov(x, y),
                    horizontal_fov(x, y),
                    angle_for_zoom(x, y),
                    zoom_for_angle(x, y),
                ] {
                    assert!(v.is_finite() && v > 0.0, "{x} {y} -> {v}");
                }
                assert!(zoom_for_focal(x, y).is_finite());
                assert!(focal_for_zoom(x, y).is_finite());
            }
        }
        assert!(angle_of_view(f32::NAN, FILM_HEIGHT_MM) <= ANGLE_MAX_RAD);
    }

    /// `distance * tan(fov / 2)` is invariant under the dolly-zoom factor; garbage is no change.
    #[test]
    fn dolly_zoom_factor_keeps_the_screen_size() {
        let d = 10.0;
        for (a, b) in [
            (FOV_PERSPECTIVE, FOV_FLAT),
            (FOV_FLAT, FOV_PERSPECTIVE),
            (0.5, 1.7),
        ] {
            let d2 = d * dolly_zoom_factor(a, b);
            let before = d * (a * 0.5).tan();
            let after = d2 * (b * 0.5).tan();
            assert!((before - after).abs() < before * 1e-5, "{a} -> {b}");
        }
        // Flattening pulls the camera ~33x back.
        assert!((dolly_zoom_factor(FOV_PERSPECTIVE, FOV_FLAT) - 33.07).abs() < 0.05);
        assert_eq!(dolly_zoom_factor(f32::NAN, 1.0), 1.0);
        assert_eq!(dolly_zoom_factor(1.0, f32::INFINITY), 1.0);
        assert_eq!(dolly_zoom_factor(0.7, 0.7), 1.0);
    }

    /// The orbit step is 1 at the perspective default and tiny at the flat lens.
    #[test]
    fn orbit_step_scale_follows_tan_half_fov() {
        assert!((orbit_step_scale(FOV_PERSPECTIVE) - 1.0).abs() < 1e-6);
        let flat = orbit_step_scale(FOV_FLAT);
        assert!((flat - (FOV_FLAT * 0.5).tan() / (FOV_PERSPECTIVE * 0.5).tan()).abs() < 1e-7);
        assert!(flat < 0.05, "{flat}");
        assert_eq!(orbit_step_scale(f32::NAN), 1.0);
    }

    #[test]
    fn top_down_gate() {
        assert!(is_top_down(-Vec3::Y, Vec3::Y));
        assert!(!is_top_down(Vec3::new(0.0, -1.0, 0.2), Vec3::Y));
        assert!(
            !is_top_down(Vec3::Y, Vec3::Y),
            "looking up from below is not top-down"
        );
        assert!(!is_top_down(Vec3::ZERO, Vec3::Y));
        assert!(!is_top_down(Vec3::splat(f32::NAN), Vec3::Y));
    }

    /// A fresh lens rests on its mode target.
    #[test]
    fn new_rests_on_the_mode_target() {
        let flat = LensEase::new(LensMode::Flat);
        assert_eq!(flat.fov(), FOV_FLAT);
        assert!(!flat.is_easing());
        let persp = LensEase::default();
        assert_eq!(persp.fov(), FOV_PERSPECTIVE);
        assert_eq!(persp.mode(), LensMode::Perspective);
    }

    /// One host frame: the controller's FOV goes in, the eased FOV comes back out and is pushed
    /// into the controller (here `fov`, standing in for its projection).
    fn frame(lens: &mut LensEase, fov: &mut f32, dt: f32) {
        *fov = lens.step(dt, *fov);
    }

    /// The ease moves, lands EXACTLY on the target, then rests.
    #[test]
    fn ease_lands_exactly() {
        let mut lens = LensEase::new(LensMode::Flat);
        let mut fov = lens.fov();
        lens.set_mode(LensMode::Perspective);
        assert!(lens.is_easing());
        let dt = 1.0 / 60.0;
        frame(&mut lens, &mut fov, dt);
        assert!(fov > FOV_FLAT && fov < FOV_PERSPECTIVE, "{fov}");
        assert_eq!(fov, lens.fov());
        let mut frames = 1;
        while lens.is_easing() {
            frame(&mut lens, &mut fov, dt);
            frames += 1;
            assert!(frames < 1_000, "the ease never landed");
        }
        assert_eq!(fov, FOV_PERSPECTIVE);
        frame(&mut lens, &mut fov, dt);
        assert_eq!(fov, FOV_PERSPECTIVE, "at rest it stays");
        // ~98 % of the gap within one ease duration (21 frames at 60 Hz).
        let mut l2 = LensEase::new(LensMode::Flat);
        let mut f2 = l2.fov();
        l2.set_mode(LensMode::Perspective);
        for _ in 0..21 {
            frame(&mut l2, &mut f2, dt);
        }
        let left = (FOV_PERSPECTIVE - f2) / (FOV_PERSPECTIVE - FOV_FLAT);
        assert!(left < 0.025, "{left}");
    }

    /// A dolly zoom between frames (the controller's FOV changed) is adopted: in perspective it
    /// becomes the target and is never eased back; in flat the ease carries it back to flat.
    #[test]
    fn an_external_fov_change_is_adopted() {
        let dt = 1.0 / 60.0;
        let mut lens = LensEase::default();
        let mut fov = lens.fov();
        fov += 0.3; // a DollyZoom on the controller
        frame(&mut lens, &mut fov, dt);
        assert_eq!(fov, FOV_PERSPECTIVE + 0.3, "the dolly zoom was not undone");
        assert_eq!(lens.perspective_fov(), FOV_PERSPECTIVE + 0.3);
        assert!(!lens.is_easing());
        for _ in 0..100 {
            frame(&mut lens, &mut fov, dt);
        }
        assert_eq!(fov, FOV_PERSPECTIVE + 0.3);

        let mut flat = LensEase::new(LensMode::Flat);
        let mut f = 0.5;
        frame(&mut flat, &mut f, dt);
        assert!(f < 0.5 && f > FOV_FLAT, "eases back toward flat: {f}");
        assert_eq!(
            flat.perspective_fov(),
            FOV_PERSPECTIVE,
            "flat keeps the target"
        );

        // Out of range and garbage: clamped into the one FOV range, NaN ignored.
        let mut l = LensEase::default();
        assert_eq!(l.step(dt, 3.0), FOV_MAX);
        assert_eq!(l.step(dt, f32::NAN), FOV_MAX);
        assert_eq!(l.step(dt, 0.0), FOV_MIN);
    }

    /// An unreachable target (the controller's reach excludes it) counts as reached: the ease
    /// lands on the reach limit and stops easing; a bad reach is ignored; a sub-epsilon round
    /// trip difference is not adopted.
    #[test]
    fn the_target_is_clamped_into_the_reach() {
        let dt = 1.0 / 60.0;
        let mut lens = LensEase::new(LensMode::Perspective);
        lens.set_reach(0.04, 1.5);
        lens.set_reach(f32::NAN, 1.0);
        lens.set_reach(1.0, 0.5);
        lens.set_mode(LensMode::Flat);
        assert_eq!(lens.target(), 0.04);
        let mut fov = lens.fov();
        let mut frames = 0;
        while lens.is_easing() {
            frame(&mut lens, &mut fov, dt);
            frames += 1;
            assert!(frames < 1_000, "an unreachable target never landed");
        }
        assert_eq!(fov, 0.04);
        // A rounding-level difference from the controller is not a change.
        lens.set_mode(LensMode::Perspective);
        let target = lens.perspective_fov();
        let back = lens.step(dt, lens.fov() + LAND_EPS_RAD * 0.5);
        assert!(back > 0.04, "the perspective ease still runs: {back}");
        assert_eq!(
            lens.perspective_fov(),
            target,
            "not retargeted by a rounding error"
        );
    }

    /// The same elapsed time gives the same FOV at 30 Hz and 120 Hz.
    #[test]
    fn ease_is_frame_rate_independent() {
        let run = |hz: u32, secs: f32| {
            let mut lens = LensEase::new(LensMode::Flat);
            let mut fov = lens.fov();
            lens.set_mode(LensMode::Perspective);
            let n = (secs * hz as f32).round() as u32;
            for _ in 0..n {
                frame(&mut lens, &mut fov, 1.0 / hz as f32);
            }
            fov
        };
        for secs in [0.1, 0.2, 0.3] {
            let a = run(30, secs);
            let b = run(120, secs);
            assert!((a - b).abs() < 1e-5, "{secs}s: {a} vs {b}");
            assert!(a < FOV_PERSPECTIVE, "still easing at {secs}s");
        }
    }

    /// Zero, negative and non-finite dt change nothing (the FOV is only adopted).
    #[test]
    fn bad_dt_is_ignored() {
        let mut lens = LensEase::new(LensMode::Flat);
        lens.set_mode(LensMode::Perspective);
        for dt in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            assert_eq!(lens.step(dt, FOV_FLAT), FOV_FLAT, "{dt}");
        }
        assert!(lens.is_easing());
    }

    /// An instant ease lands on the first valid step.
    #[test]
    fn zero_ease_secs_lands_instantly() {
        let mut lens = LensEase::new(LensMode::Perspective);
        lens.set_ease_secs(0.0);
        lens.set_mode(LensMode::Flat);
        assert_eq!(lens.step(1.0 / 60.0, FOV_PERSPECTIVE), FOV_FLAT);
        assert!(!lens.is_easing());
        lens.set_ease_secs(-1.0);
        lens.set_ease_secs(f32::NAN);
        assert_eq!(lens.ease_secs(), 0.0, "bad durations are ignored");
    }

    /// The gate holds a flatten, never a perspective ease.
    #[test]
    fn gate_holds_only_the_flatten() {
        let dt = 1.0 / 60.0;
        let mut lens = LensEase::new(LensMode::Perspective);
        lens.set_mode(LensMode::Flat);
        let held = lens.step_gated(dt, FOV_PERSPECTIVE, false);
        assert_eq!(held, FOV_PERSPECTIVE, "held while tilted");
        let moved = lens.step_gated(dt, held, true);
        assert!(moved < FOV_PERSPECTIVE && lens.is_easing());
        let mut open = LensEase::new(LensMode::Flat);
        open.set_mode(LensMode::Perspective);
        assert!(open.step_gated(dt, FOV_FLAT, false) > FOV_FLAT);
    }

    /// The perspective target clamps into the one FOV range and ignores non-finite input; a
    /// retarget mid-ease is followed.
    #[test]
    fn perspective_target_clamps_and_retargets() {
        let mut lens = LensEase::default();
        lens.set_perspective_fov(10.0);
        assert_eq!(lens.perspective_fov(), FOV_MAX);
        lens.set_perspective_fov(-1.0);
        assert_eq!(lens.perspective_fov(), FOV_MIN);
        lens.set_perspective_fov(f32::NAN);
        assert_eq!(lens.perspective_fov(), FOV_MIN);
        lens.nudge_perspective_fov(f32::INFINITY);
        assert_eq!(lens.perspective_fov(), FOV_MIN);
        lens.set_perspective_fov(1.0);
        lens.nudge_perspective_fov(0.25);
        assert!((lens.perspective_fov() - 1.25).abs() < EPS);
        let mut fov = lens.fov();
        let mut frames = 0;
        while lens.is_easing() {
            frame(&mut lens, &mut fov, 1.0 / 60.0);
            frames += 1;
            assert!(frames < 1_000);
        }
        assert_eq!(fov, lens.perspective_fov());
        lens.set_fov(f32::NAN);
        assert_eq!(lens.fov(), fov);
        lens.set_fov(7.0);
        assert_eq!(lens.fov(), FOV_MAX);
    }
}
