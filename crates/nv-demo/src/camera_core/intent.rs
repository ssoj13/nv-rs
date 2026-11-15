//! THE camera intent: the one host-agnostic request vocabulary of camera-core.
//!
//! What: [`CameraIntent`] (orbit / pan / zoom / dolly-zoom / frame / view snaps for the orbit rigs;
//! look, steer rate, roll, thrust, speed scale for the fly / spaceflight backends) and [`OrbitSnapMode`] (the
//! constraint of an aligned orbit gesture).
//!
//! Why: a host input layer and the host adapters (`camera-egui`, `camera-winit`) translate raw
//! events into intents, and [`crate::camera_core::CameraController::apply_intent`] is the single place that turns an
//! intent into rig motion, so every host navigates with the same rules.
//!
//! Used by: `controller`, `rig::orbit`, `orientation` and every host.

use glam::Vec3;

/// Constraint applied to the free direction accumulated during an orbit gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OrbitSnapMode {
    /// Project onto the nearest world coordinate plane.
    Plane,
    /// Project onto the nearest world axis.
    Axis,
    /// Project onto the nearest coordinate plane (zeroing the smallest direction component),
    /// and when within [`crate::camera_core::SMART_AXIS_SNAP_RAD`] (15°) of a second
    /// coordinate plane also project onto their intersection (the nearest world axis).
    #[default]
    Smart,
}

/// High-level camera manipulation request (host-agnostic). `#[non_exhaustive]`: a host matches
/// with a catch-all arm, so a new intent (as `View` was) does not break it.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum CameraIntent {
    /// Orbit about the pivot by a yaw / pitch delta (radians).
    Orbit {
        /// Yaw delta (radians).
        yaw_delta: f32,
        /// Pitch delta (radians).
        pitch_delta: f32,
    },
    /// Coast after pointer release: the release angular velocity in rad/s, started as a
    /// finite-time [`crate::camera_core::coast::Coast`] by each orbit backend (`HoudiniOrbit` through
    /// [`crate::camera_core::OrbitRig::update_inertia`], `QuatOrbit` through its own
    /// [`crate::camera_core::coast::OrbitCoast`]); it ends at exact rest after the coast time.
    OrbitInertia {
        /// Yaw rate (rad/s).
        yaw_rate: f32,
        /// Pitch rate (rad/s).
        pitch_rate: f32,
    },
    /// Constrained orbit: accumulate free yaw / pitch, then show the nearest world
    /// coordinate plane or axis according to `mode`.
    /// `start` begins a gesture from the current view.
    OrbitSnap {
        /// Yaw delta (radians).
        yaw_delta: f32,
        /// Pitch delta (radians).
        pitch_delta: f32,
        /// True on the first frame of the gesture.
        start: bool,
        /// Plane / axis / smart constraint.
        mode: OrbitSnapMode,
    },
    /// Pan the pivot in the view plane by a pointer delta.
    Pan {
        /// Horizontal delta (viewport units, e.g. logical points).
        dx_px: f32,
        /// Vertical delta (viewport units).
        dy_px: f32,
    },
    /// Pan coast after pointer release: the release pointer velocity in px/s.
    PanInertia {
        /// Horizontal release velocity (viewport units/s).
        dx_rate: f32,
        /// Vertical release velocity (viewport units/s).
        dy_rate: f32,
    },
    /// Scale the orbit distance by `factor` (< 1 moves in).
    Zoom {
        /// Multiplicative distance factor.
        factor: f32,
    },
    /// Zoom coast after a wheel / dolly gesture: the log-distance rate in 1/s (the distance
    /// scales by `exp(rate * dt)`). Both orbit backends kick their zoom
    /// [`crate::camera_core::coast::Coast`] with it (a wheel notch adds to a running zoom coast); it ends at
    /// exact rest after the coast time.
    ZoomInertia {
        /// Log-distance rate (1/s); positive moves out.
        rate: f32,
    },
    /// Dolly-zoom coast after a gesture: the vertical-FOV rate in rad/s, started as a finite-time
    /// coast by the orbit backends and stepped through the same range limits as
    /// [`CameraIntent::DollyZoom`] (a channel pushing against a limit stops).
    DollyZoomInertia {
        /// Vertical-FOV rate (rad/s); positive widens the view.
        fov_rate: f32,
    },
    /// Look coast after a fly drag ends or a persistent mouse-look pauses: the release look rate
    /// in rad/s for the fly backends (`FpsFly`, `SpaceFlight`). Ignored by the orbit rigs, which
    /// coast through [`CameraIntent::OrbitInertia`].
    LookInertia {
        /// Yaw rate (rad/s).
        yaw_rate: f32,
        /// Pitch rate (rad/s).
        pitch_rate: f32,
    },
    /// Stop every coast now: emitted first on a pointer press so a grab freezes the camera. It
    /// is not a view edit (no pose change, a running view transition is untouched).
    Stop,
    /// Zoom toward the point under the cursor.
    ZoomAtCursor {
        /// Cursor x (viewport units, origin top-left).
        cursor_x: f32,
        /// Cursor y (viewport units, origin top-left).
        cursor_y: f32,
        /// Multiplicative distance factor.
        factor: f32,
    },
    /// Frame an axis-aligned box.
    FrameBounds {
        /// Box minimum corner (world).
        min: Vec3,
        /// Box maximum corner (world).
        max: Vec3,
        /// Framing-distance multiplier (values below 1.0 act as 1.0, a tight fit).
        margin: f32,
    },
    /// Mouse-look delta (radians applied as yaw / pitch) for the fly / spaceflight backends and
    /// `QuatOrbit`. Ignored by `HoudiniOrbit` (use [`CameraIntent::Orbit`] there).
    Look {
        /// Yaw delta (radians).
        dyaw: f32,
        /// Pitch delta (radians).
        dpitch: f32,
    },
    /// Held mouse-steer RATE axes (yaw/pitch ∈ [−1,1], deflection of the cursor
    /// from the viewport centre) for the space-sim steering mode. Integrated
    /// with real dt in the fly backends' `update_dynamics`, so the ship turns
    /// continuously while the cursor is held off-centre. No-op for both orbit backends.
    LookRate {
        /// Yaw rate axis in [-1, 1].
        yaw: f32,
        /// Pitch rate axis in [-1, 1].
        pitch: f32,
    },
    /// HELD roll axis for the spaceflight backend (refreshed every frame, like `Thrust`); it is
    /// integrated with the real dt as a roll rate of `d * roll_rate` rad/s approached with the
    /// thrust response ([`crate::camera_core::InertiaSettings::roll_rate`]), or steers the horizon-lock flip
    /// when locked. No-op for orbit / FPS.
    Roll {
        /// Held roll axis in [-1, 1] (e.g. -1 / 0 / +1 from the roll keys); 0 releases it.
        d: f32,
    },
    /// Body-relative thrust for the fly / spaceflight backends: `forward` /
    /// `right` / `up` are signed thrust magnitudes (e.g. WASD axes). No-op for
    /// the orbit rig.
    Thrust {
        /// Forward thrust axis (signed).
        forward: f32,
        /// Right thrust axis (signed).
        right: f32,
        /// Up thrust axis (signed).
        up: f32,
    },
    /// Thrust speed scale for the fly backends, from the held modifiers
    /// ([`crate::camera_core::InertiaSettings::speed_scale`]); 1.0 = normal. No-op for the orbit rig.
    SpeedScale(f32),
    /// Go to a view (a face, edge or corner: nav cube clicks, the 1 / 3 / 7 keys). The orbit
    /// rig animates there about its target ([`crate::camera_core::OrbitRig::animate_to_view`]); the free
    /// backends turn in place (Y-up framing, [`crate::camera_core::ViewTarget::frame`]).
    View(crate::camera_core::ViewTarget),
    /// Dolly-zoom (the "vertigo" shot): widen the vertical FOV by `dfov` while the eye dollies
    /// along the view axis about the pivot so `distance * tan(fov_y / 2)` stays constant, i.e.
    /// the subject at the pivot keeps its on-screen size while the perspective stretches.
    ///
    /// Handled by the orbit backends (`HoudiniOrbit`, `QuatOrbit`): the FOV stays within
    /// [`crate::camera_core::lens::FOV_MIN`]..=[`crate::camera_core::lens::FOV_MAX`] (the one viewport FOV range) and the
    /// distance within the lens distance range; the step is limited so both hold, so the
    /// invariant ([`crate::camera_core::lens::dolly_zoom_factor`]) is never broken. It is a direct edit (coasts and a view transition stop). A
    /// no-op on the fly backends (`FpsFly`, `SpaceFlight`): they have no pivot, so no subject
    /// distance to keep. A non-finite `dfov` is dropped.
    DollyZoom {
        /// FOV change in radians; positive widens the view (the eye dollies in).
        dfov: f32,
    },
}

impl CameraIntent {
    /// This intent expressed for a view the host displays rolled by `view_roll` radians about
    /// the view axis (a host overlay roll; the rig's own orientation carries none).
    ///
    /// Screen-space intents are rotated so a drag follows the DISPLAYED screen axes:
    /// `Pan` / `PanInertia` `(x, y) -> (x cos + y sin, y cos - x sin)` and `ZoomAtCursor`'s
    /// cursor rotated the same way about the centre of `viewport`. Everything else, and a zero
    /// or non-finite roll, is returned unchanged (bit-identical).
    pub fn unrolled(
        self,
        view_roll: f32,
        viewport: crate::camera_core::viewport::ViewportSize,
    ) -> Self {
        if view_roll == 0.0 || !view_roll.is_finite() {
            return self;
        }
        let (sin, cos) = view_roll.sin_cos();
        let rot = |x: f32, y: f32| (x * cos + y * sin, y * cos - x * sin);
        match self {
            Self::Pan { dx_px, dy_px } => {
                let (dx_px, dy_px) = rot(dx_px, dy_px);
                Self::Pan { dx_px, dy_px }
            }
            Self::PanInertia { dx_rate, dy_rate } => {
                let (dx_rate, dy_rate) = rot(dx_rate, dy_rate);
                Self::PanInertia { dx_rate, dy_rate }
            }
            Self::ZoomAtCursor {
                cursor_x,
                cursor_y,
                factor,
            } => {
                let (cx, cy) = (viewport.width_f() * 0.5, viewport.height_f() * 0.5);
                let (ox, oy) = rot(cursor_x - cx, cursor_y - cy);
                Self::ZoomAtCursor {
                    cursor_x: cx + ox,
                    cursor_y: cy + oy,
                    factor,
                }
            }
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera_core::viewport::ViewportSize;

    const HALF_PI: f32 = std::f32::consts::FRAC_PI_2;

    fn vp() -> ViewportSize {
        ViewportSize::new(800, 600)
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    /// A quarter-turn roll maps a rightward drag onto the displayed screen's downward axis, for
    /// the drag and its release rate alike.
    #[test]
    fn unrolled_rotates_pan_with_the_roll() {
        let CameraIntent::Pan { dx_px, dy_px } = CameraIntent::Pan {
            dx_px: 10.0,
            dy_px: 0.0,
        }
        .unrolled(HALF_PI, vp()) else {
            panic!("Pan stays Pan");
        };
        assert!(
            close(dx_px, 0.0) && close(dy_px, -10.0),
            "({dx_px}, {dy_px})"
        );

        let CameraIntent::PanInertia { dx_rate, dy_rate } = (CameraIntent::PanInertia {
            dx_rate: 0.0,
            dy_rate: 30.0,
        })
        .unrolled(HALF_PI, vp()) else {
            panic!("PanInertia stays PanInertia");
        };
        assert!(
            close(dx_rate, 30.0) && close(dy_rate, 0.0),
            "({dx_rate}, {dy_rate})"
        );
    }

    /// Roll 0 (and a non-finite roll) is the identity, bit for bit, for every intent.
    #[test]
    fn unrolled_zero_roll_is_identity() {
        let intents = [
            CameraIntent::Pan {
                dx_px: 3.5,
                dy_px: -7.25,
            },
            CameraIntent::ZoomAtCursor {
                cursor_x: 123.0,
                cursor_y: 45.0,
                factor: 0.9,
            },
            CameraIntent::Orbit {
                yaw_delta: 0.1,
                pitch_delta: 0.2,
            },
            CameraIntent::Stop,
        ];
        for i in intents {
            assert_eq!(i.unrolled(0.0, vp()), i);
            assert_eq!(i.unrolled(-0.0, vp()), i);
            assert_eq!(i.unrolled(f32::NAN, vp()), i);
        }
    }

    /// The zoom cursor turns about the viewport centre; the centre itself and the factor stay;
    /// non-screen intents are untouched under a roll.
    #[test]
    fn unrolled_rotates_the_zoom_cursor_about_the_centre() {
        let CameraIntent::ZoomAtCursor {
            cursor_x,
            cursor_y,
            factor,
        } = (CameraIntent::ZoomAtCursor {
            cursor_x: 500.0,
            cursor_y: 300.0,
            factor: 0.8,
        })
        .unrolled(HALF_PI, vp())
        else {
            panic!("ZoomAtCursor stays ZoomAtCursor");
        };
        assert!(
            close(cursor_x, 400.0) && close(cursor_y, 200.0),
            "({cursor_x}, {cursor_y})"
        );
        assert_eq!(factor, 0.8);

        let centre = CameraIntent::ZoomAtCursor {
            cursor_x: 400.0,
            cursor_y: 300.0,
            factor: 1.1,
        };
        let CameraIntent::ZoomAtCursor {
            cursor_x, cursor_y, ..
        } = centre.unrolled(1.0, vp())
        else {
            panic!("ZoomAtCursor stays ZoomAtCursor");
        };
        assert!(close(cursor_x, 400.0) && close(cursor_y, 300.0));

        let orbit = CameraIntent::Orbit {
            yaw_delta: 0.3,
            pitch_delta: -0.1,
        };
        assert_eq!(orbit.unrolled(1.0, vp()), orbit);
    }
}
