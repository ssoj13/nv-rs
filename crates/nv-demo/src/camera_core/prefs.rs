//! [`NavPrefs`]: the user-facing camera navigation preferences (a Settings > Controls page) and
//! their mapping onto the rig tuning.
//!
//! What: the camera half of the four `Controls` prefs copies (egui-widgets-rs sl-studio
//! `crates/sl/studio/src/app.rs:758-850`, colmap-rs `crates/colmap-ui/src/camera/nav.rs:26-103`,
//! vfx-view `crates/view/vfx-view/src/panels/camera_controls.rs:94-179`, and the vdb-view fork of
//! the vfx-view one): the navigation mode, the turn feel, look sensitivity, fly speed, the release
//! coast, look smoothing, horizon-lock flip, speed multipliers and the align time. App-UI fields
//! of those copies (gizmo size / stroke, selection bounds, slot-strip buttons) stay in the apps.
//! The copies' two decay times (translate / rotate) are gone: every release glides through one
//! [`CoastSettings`] (`coast`).
//!
//! Ownership (one owner per concern): `mode` owns the backend, the orientation policy, the mouse
//! steering and the horizon lock (the copies' `free_flight` flag is `NavMode::SpaceSim` vs
//! `NavMode::SpaceSimLocked`); `feel` owns the inertial look and the release coast (the copies'
//! `inertial_look` flag is `NavFeel::Smooth` vs `NavFeel::Sharp`).
//!
//! Why: the copies carried identical mappers; one struct keeps them identical and validates the
//! values (a NaN or negative preference never reaches a rig).
//!
//! How: [`NavPrefs::inertia`] / [`NavPrefs::navigation`] map the preferences onto the rig tuning
//! (look and orbit sensitivity, cruise speed from the scene radius and `fly_speed`, pan scale from
//! the fov and viewport height); the steering comes from the mode and the feel fields (inertial
//! look, coast time) from [`NavFeel`]. Both first run [`NavPrefs::sanitized`] (every number
//! clamped to a named safe range, non-finite -> the default) and validate their own arguments.
//! `CameraController::apply_prefs` applies everything in one call. Serde (feature `serde`) with
//! `#[serde(default)]` and no `deny_unknown_fields`, so an older file missing a field loads it at
//! its default and one still carrying the removed `translate_decay` / `rotate_decay` keys loads
//! (the keys are ignored).
//!
//! Uses: `crate::camera_core::settings`, `crate::camera_core::mode`, `crate::camera_core::coast`, `crate::camera_core::projection` (default fov).
//! Used by host settings pages and viewports (`CameraController::apply_prefs` each frame).

use crate::camera_core::coast::CoastSettings;
use crate::camera_core::mode::{NavFeel, NavMode};
use crate::camera_core::projection::PerspectiveSettings;
use crate::camera_core::settings::{CameraNavigationSettings, InertiaSettings, ZoomMode};

/// Mouse-look radians per pixel at `look_sensitivity = 1` (studio: "look 5 mrad per pixel").
const LOOK_RAD_PER_PX: f32 = 0.005;

/// Roll rate of the flight rig (rad/s of a fully held roll axis).
const ROLL_RATE: f32 = 2.0;

/// Horizon-lock levelling speed (1/s) the preferences give the flight rig.
const LOCK_RATE: f32 = 1.0;

/// Cruise speed in scene radii per second at `fly_speed = 1` (the old thrust 2 radii/s^2 over the
/// 0.96 s translate decay, kept so the default flight feels as before).
const CRUISE_RADII_PER_S: f32 = 1.92;

/// The horizon-lock hold angle sits this many degrees past the flip angle, so a held roll key
/// always flips.
const FLIP_HOLD_MARGIN_DEG: f32 = 15.0;

/// Orbit degrees per pixel of drag at `look_sensitivity = 1`.
const ORBIT_DEG_PER_PX: f32 = 0.1;

/// Exponential wheel-zoom rate of the orbit (per scroll unit).
const ZOOM_SCROLL_EXPONENTIAL: f32 = 0.0015;

/// Pan: the view spans `2 tan(fov / 2)` distances over its height, so the target moves exactly
/// with the cursor.
const PAN_VIEW_SPAN: f32 = 2.0;

/// Smallest viewport height (points) the pan scale divides by.
const HEIGHT_MIN: f32 = 1.0;

/// Scene framing radius used when the host passes a non-finite one.
const SCENE_RADIUS_FALLBACK: f32 = 1.0;

/// Range of the scene framing radius (world units): positive (thrust never vanishes or flips)
/// and bounded (thrust never overflows).
const SCENE_RADIUS_RANGE: (f32, f32) = (1e-6, 1e12);

/// Sanitised range of `look_sensitivity` (also the orbit sensitivity multiplier).
const LOOK_SENSITIVITY_RANGE: (f32, f32) = (0.0, 100.0);

/// Sanitised range of `fly_speed`.
const FLY_SPEED_RANGE: (f32, f32) = (0.0, 1e4);

/// Sanitised range of `look_smoothing` (seconds; 0 = instant).
const LOOK_SMOOTHING_RANGE: (f32, f32) = (0.0, 10.0);

/// Sanitised range of `flip_degrees` (the widest UI range of the copies, vfx-view 20..85, with a
/// lower end that still flips).
const FLIP_DEGREES_RANGE: (f32, f32) = (1.0, 85.0);

/// Sanitised range of `fast_multiplier` (at least 1: "fast" never slows down).
const FAST_MULTIPLIER_RANGE: (f32, f32) = (1.0, 1e3);

/// Sanitised range of `slow_multiplier`: positive (a zero speed scale leaves a never-ending
/// exponential creep) and at most 1 ("slow" never speeds up).
const SLOW_MULTIPLIER_RANGE: (f32, f32) = (1e-3, 1.0);

/// Sanitised range of `align_seconds` (0 = snap at once).
const ALIGN_SECONDS_RANGE: (f32, f32) = (0.0, 60.0);

/// `v` clamped to `range`, or `fallback` when `v` is NaN or infinite.
fn finite_in(v: f32, range: (f32, f32), fallback: f32) -> f32 {
    if v.is_finite() {
        crate::camera_core::num::clamp(v, range.0, range.1)
    } else {
        fallback
    }
}

/// Camera navigation preferences (the camera half of a Settings > Controls page).
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default)
)]
pub struct NavPrefs {
    /// The navigation mode: THE owner of the backend, orientation policy, mouse steering and
    /// horizon lock (default [`NavMode::Turntable`]).
    pub mode: NavMode,
    /// The fly mode a hold-to-fly drag (Unreal RMB) flies in while `mode` is an orbit mode
    /// (default [`NavMode::FpsFly`]; a non-fly value sanitises to it). `crate::camera_core::CameraDriver`
    /// switches to it for the drag and back to `mode`, with the orbit pivot, on release.
    pub hold_fly_mode: NavMode,
    /// The turn feel: THE owner of the inertial look and whether releases coast (default
    /// [`NavFeel::Smooth`], the studio's `inertial_look = true`; Sharp maps `coast.time` to 0).
    pub feel: NavFeel,
    /// Mouse look and orbit sensitivity multiplier (1 = 5 mrad per pixel of look, 0.1 degree per
    /// pixel of orbit).
    pub look_sensitivity: f32,
    /// Flight cruise-speed multiplier (1 = 1.92 scene radii per second).
    pub fly_speed: f32,
    /// The release coast of every motion (orbit / pan / zoom drags, look, thrust, roll): glide
    /// time, softness and minimum speed, see [`CoastSettings`]. Sanitised; [`NavFeel::Sharp`]
    /// overrides its time with 0 in the mapped settings.
    pub coast: CoastSettings,
    /// Time constant (seconds) of the smoothed look of [`NavFeel::Smooth`] (0 there means the
    /// `InertiaSettings` default, see [`NavFeel::apply_inertia`]).
    pub look_smoothing: f32,
    /// Horizon lock: roll angle (degrees) past which the lock flips to the next plane.
    pub flip_degrees: f32,
    /// Thrust multiplier while the fast modifier (Shift) is held.
    pub fast_multiplier: f32,
    /// Thrust multiplier while the slow modifier (Alt) is held.
    pub slow_multiplier: f32,
    /// Holding the align modifier (Ctrl) during orbit or flight eases the view to the nearest
    /// world plane / axis with this time constant (seconds): `CameraController::apply_prefs`
    /// writes it into the turntable (`OrbitRig::align_seconds`); a fly host passes it in
    /// `CameraController::set_align`.
    pub align_seconds: f32,
}

impl Default for NavPrefs {
    /// The studio defaults (`app.rs:788-806`), shared verbatim by every copy; mode Turntable,
    /// feel Smooth (the studio's `inertial_look: true`).
    fn default() -> Self {
        Self {
            mode: NavMode::default(),
            hold_fly_mode: NavMode::FpsFly,
            feel: NavFeel::default(),
            look_sensitivity: 0.5,
            fly_speed: 1.0,
            coast: CoastSettings::default(),
            look_smoothing: 0.08,
            flip_degrees: 48.6,
            fast_multiplier: 4.0,
            slow_multiplier: 0.1,
            align_seconds: 0.5,
        }
    }
}

impl NavPrefs {
    /// These preferences with every number in its safe range (the `*_RANGE` constants of this
    /// module); a NaN or infinite value becomes the field's default. Valid values pass
    /// unchanged.
    pub fn sanitized(&self) -> Self {
        let d = Self::default();
        Self {
            mode: self.mode,
            hold_fly_mode: if self.hold_fly_mode.is_fly() {
                self.hold_fly_mode
            } else {
                d.hold_fly_mode
            },
            feel: self.feel,
            look_sensitivity: finite_in(
                self.look_sensitivity,
                LOOK_SENSITIVITY_RANGE,
                d.look_sensitivity,
            ),
            fly_speed: finite_in(self.fly_speed, FLY_SPEED_RANGE, d.fly_speed),
            coast: self.coast.sanitized(),
            look_smoothing: finite_in(self.look_smoothing, LOOK_SMOOTHING_RANGE, d.look_smoothing),
            flip_degrees: finite_in(self.flip_degrees, FLIP_DEGREES_RANGE, d.flip_degrees),
            fast_multiplier: finite_in(
                self.fast_multiplier,
                FAST_MULTIPLIER_RANGE,
                d.fast_multiplier,
            ),
            slow_multiplier: finite_in(
                self.slow_multiplier,
                SLOW_MULTIPLIER_RANGE,
                d.slow_multiplier,
            ),
            align_seconds: finite_in(self.align_seconds, ALIGN_SECONDS_RANGE, d.align_seconds),
        }
    }

    /// The flight rig tuning for a scene of framing radius `scene_radius` (the cruise speed
    /// scales with it): on the sanitised preferences, `cruise_speed` is 1.92 radii per second
    /// times `fly_speed`, `roll_rate` 2 rad/s, `lock_rate` 1/s, and `look_sensitivity`, the
    /// horizon-lock angles and the speed multipliers come from the preferences. Two fields are
    /// owned: `mouse_steer` is the mode's ([`NavMode::mouse_steer`]; the studio's fixed
    /// `Relative` is the FPS / quaternion-flight steering) and `inertial_look` /
    /// `look_smoothing` / `coast` are the feel's ([`NavFeel::apply_inertia`],
    /// [`NavFeel::coast`]). Every other field is the [`InertiaSettings`] default. A non-finite
    /// radius uses 1, any other is clamped to a positive bounded range.
    pub fn inertia(&self, scene_radius: f32) -> InertiaSettings {
        let p = self.sanitized();
        let radius = finite_in(scene_radius, SCENE_RADIUS_RANGE, SCENE_RADIUS_FALLBACK);
        let defaults = InertiaSettings::default();
        let mut inertia = InertiaSettings {
            mouse_steer: p.mode.mouse_steer(),
            look_sensitivity: LOOK_RAD_PER_PX * p.look_sensitivity,
            cruise_speed: CRUISE_RADII_PER_S * radius * p.fly_speed,
            roll_rate: ROLL_RATE,
            lock_rate: LOCK_RATE,
            look_smoothing: p.look_smoothing,
            lock_flip: p.flip_degrees.to_radians(),
            lock_hold: (p.flip_degrees + FLIP_HOLD_MARGIN_DEG).to_radians(),
            fast_multiplier: p.fast_multiplier,
            slow_multiplier: p.slow_multiplier,
            ..defaults
        };
        p.feel.apply_inertia(&mut inertia);
        // After `apply_inertia`: the preference's own coast time (even 0) wins over the preset's
        // "restore a usable time"; Sharp still maps it to 0.
        inertia.coast = p.feel.coast(p.coast);
        inertia
    }

    /// The orbit tuning for a view of vertical FOV `fov_y` (radians) that is `viewport_height`
    /// points tall: pan moves the target exactly with the cursor, the wheel zooms
    /// exponentially, a released drag coasts through the preferences' `coast` (Sharp = time 0,
    /// [`NavFeel::coast`]). A non-finite or
    /// degenerate fov uses the [`PerspectiveSettings`] default (a fov is clamped into the lens
    /// angle range); a non-finite height uses 1.
    pub fn navigation(&self, fov_y: f32, viewport_height: f32) -> CameraNavigationSettings {
        let p = self.sanitized();
        let default_fov = PerspectiveSettings::default().fov_y;
        let fov_y = finite_in(
            fov_y,
            (
                crate::camera_core::lens::ANGLE_MIN_RAD,
                crate::camera_core::lens::ANGLE_MAX_RAD,
            ),
            default_fov,
        );
        let height = if viewport_height.is_finite() {
            viewport_height
        } else {
            HEIGHT_MIN
        };
        CameraNavigationSettings {
            zoom_mode: ZoomMode::Exponential,
            zoom_to_cursor: false,
            orbit_sensitivity: ORBIT_DEG_PER_PX.to_radians() * p.look_sensitivity,
            zoom_scroll_exponential: ZOOM_SCROLL_EXPONENTIAL,
            pan_sensitivity: PAN_VIEW_SPAN * (fov_y * 0.5).tan() / height.max(HEIGHT_MIN),
            coast: p.feel.coast(p.coast),
            ..CameraNavigationSettings::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera_core::settings::MouseSteer;

    /// The expected flight mapping, transcribed literally with the coast-model constants (cruise
    /// 1.92 radii/s, roll 2 rad/s, lock 1/s); the studio's `inertial_look` flag is
    /// `feel == Smooth` and the coast is the preference's (Smooth feel).
    fn studio_inertia(p: &NavPrefs, radius: f32) -> InertiaSettings {
        let defaults = InertiaSettings::default();
        InertiaSettings {
            mouse_steer: MouseSteer::Relative,
            look_sensitivity: 0.005 * p.look_sensitivity,
            cruise_speed: 1.92 * radius * p.fly_speed,
            roll_rate: 2.0,
            lock_rate: 1.0,
            coast: p.coast,
            inertial_look: p.feel == NavFeel::Smooth,
            look_smoothing: p.look_smoothing,
            lock_flip: p.flip_degrees.to_radians(),
            lock_hold: (p.flip_degrees + 15.0).to_radians(),
            fast_multiplier: p.fast_multiplier,
            slow_multiplier: p.slow_multiplier,
            ..defaults
        }
    }

    /// The expected orbit mapping, transcribed literally (Smooth feel: the preference's coast).
    fn studio_navigation(p: &NavPrefs, fov_y: f32, height: f32) -> CameraNavigationSettings {
        CameraNavigationSettings {
            zoom_mode: ZoomMode::Exponential,
            zoom_to_cursor: false,
            orbit_sensitivity: 0.1_f32.to_radians() * p.look_sensitivity,
            zoom_scroll_exponential: 0.0015,
            pan_sensitivity: 2.0 * (fov_y * 0.5).tan() / height.max(1.0),
            coast: p.coast,
            ..CameraNavigationSettings::default()
        }
    }

    /// Every field of two navigation settings is bit-identical (`CameraNavigationSettings` has
    /// no `PartialEq`).
    fn nav_bits_eq(a: &CameraNavigationSettings, b: &CameraNavigationSettings) -> bool {
        let floats = |s: &CameraNavigationSettings| {
            [
                s.orbit_sensitivity,
                s.zoom_scroll_linear,
                s.zoom_scroll_exponential,
                s.pan_sensitivity,
                s.dolly_sensitivity,
                s.coast.time,
                s.coast.softness,
                s.coast.min_speed,
            ]
            .map(f32::to_bits)
        };
        a.zoom_mode == b.zoom_mode
            && a.zoom_to_cursor == b.zoom_to_cursor
            && a.invert_orbit_x == b.invert_orbit_x
            && a.invert_orbit_y == b.invert_orbit_y
            && floats(a) == floats(b)
    }

    /// Valid studio-style preferences (feel Smooth, positive smoothing) in modes whose steering
    /// is the studio's `Relative`.
    fn samples() -> [NavPrefs; 3] {
        [
            NavPrefs::default(),
            NavPrefs {
                mode: NavMode::QuatFlight,
                look_sensitivity: 2.5,
                fly_speed: 0.3,
                coast: CoastSettings {
                    time: 1.5,
                    softness: 0.0,
                    min_speed: 0.1,
                },
                look_smoothing: 0.2,
                flip_degrees: 30.0,
                fast_multiplier: 10.0,
                slow_multiplier: 0.05,
                align_seconds: 0.0,
                ..NavPrefs::default()
            },
            NavPrefs {
                mode: NavMode::FpsFly,
                coast: CoastSettings::NONE,
                flip_degrees: 85.0,
                ..NavPrefs::default()
            },
        ]
    }

    #[test]
    fn defaults_are_the_studio_defaults() {
        let p = NavPrefs::default();
        assert_eq!(p.mode, NavMode::Turntable);
        assert_eq!(p.hold_fly_mode, NavMode::FpsFly);
        assert_eq!(p.feel, NavFeel::Smooth);
        let orbit_hold = NavPrefs {
            hold_fly_mode: NavMode::Trackball,
            ..NavPrefs::default()
        };
        assert_eq!(orbit_hold.sanitized().hold_fly_mode, NavMode::FpsFly);
        assert_eq!((p.look_sensitivity, p.fly_speed), (0.5, 1.0));
        assert_eq!(p.coast, CoastSettings::default());
        assert_eq!(
            (
                p.look_smoothing,
                p.flip_degrees,
                p.fast_multiplier,
                p.slow_multiplier
            ),
            (0.08, 48.6, 4.0, 0.1)
        );
        assert_eq!(p.align_seconds, 0.5);
        assert_eq!(p.sanitized(), p, "the defaults are in range");
    }

    /// On valid Smooth-feel preferences both mappers are bit-identical to the literal formulas.
    #[test]
    fn mappers_match_the_studio_formulas() {
        for p in samples() {
            assert_eq!(p.sanitized(), p, "{p:?} is valid");
            for radius in [0.5_f32, 1.0, 37.25, 4000.0] {
                assert_eq!(
                    p.inertia(radius),
                    studio_inertia(&p, radius),
                    "{p:?} r {radius}"
                );
            }
            for (fov, height) in [(45.0_f32, 720.0_f32), (20.0, 1.0), (90.0, 1333.5)] {
                let fov = fov.to_radians();
                assert!(
                    nav_bits_eq(
                        &p.navigation(fov, height),
                        &studio_navigation(&p, fov, height)
                    ),
                    "{p:?} fov {fov} h {height}"
                );
            }
        }
        // The studio floors a sub-point height at 1 too.
        let p = NavPrefs::default();
        let fov = 45.0_f32.to_radians();
        assert!(nav_bits_eq(
            &p.navigation(fov, 0.25),
            &studio_navigation(&p, fov, 0.25)
        ));
    }

    /// The steering is the mode's (never a fixed value), for every mode.
    #[test]
    fn inertia_takes_the_steering_from_the_mode() {
        for mode in NavMode::ALL {
            let p = NavPrefs {
                mode,
                ..NavPrefs::default()
            };
            assert_eq!(p.inertia(10.0).mouse_steer, mode.mouse_steer(), "{mode:?}");
        }
    }

    /// The feel is honoured by both mappers: Sharp = no inertial look, coast time 0 in BOTH
    /// (softness and minimum speed kept); Smooth = inertial look with the user's smoothing (or
    /// the default for 0), the preference's coast.
    #[test]
    fn mappers_honour_the_feel() {
        let user_coast = CoastSettings {
            time: 1.1,
            softness: 2.5,
            min_speed: 0.07,
        };
        let sharp = NavPrefs {
            feel: NavFeel::Sharp,
            coast: user_coast,
            ..NavPrefs::default()
        };
        let (i, n) = (sharp.inertia(10.0), sharp.navigation(0.8, 600.0));
        assert!(!i.inertial_look);
        assert_eq!(i.coast.time, 0.0);
        assert_eq!(n.coast.time, 0.0);
        assert_eq!((n.coast.softness, n.coast.min_speed), (2.5, 0.07));
        assert_eq!((i.coast.softness, i.coast.min_speed), (2.5, 0.07));
        assert_eq!(NavFeel::of(&i, &n), Some(NavFeel::Sharp));

        let smooth = NavPrefs::default();
        let (i, n) = (smooth.inertia(10.0), smooth.navigation(0.8, 600.0));
        assert!(i.inertial_look);
        assert_eq!(i.look_smoothing, smooth.look_smoothing);
        assert_eq!(i.coast, smooth.coast);
        assert_eq!(n.coast, smooth.coast);
        assert_eq!(NavFeel::of(&i, &n), Some(NavFeel::Smooth));

        // A user coast time of 0 under Smooth stays 0 (the preset only restores it in `apply`).
        let none = NavPrefs {
            coast: CoastSettings::NONE,
            ..NavPrefs::default()
        };
        assert_eq!(none.inertia(10.0).coast.time, 0.0);
        assert_eq!(none.navigation(0.8, 600.0).coast.time, 0.0);

        let instant = NavPrefs {
            look_smoothing: 0.0,
            ..NavPrefs::default()
        };
        assert_eq!(
            instant.inertia(10.0).look_smoothing,
            InertiaSettings::default().look_smoothing,
            "Smooth never runs with a zero smoothing"
        );
    }

    /// NaN / infinite preferences fall back to the defaults, out-of-range ones clamp, and the
    /// mappers stay finite for garbage arguments.
    #[test]
    fn garbage_is_sanitized() {
        let (nan, inf) = (f32::NAN, f32::INFINITY);
        let garbage = NavPrefs {
            look_sensitivity: nan,
            fly_speed: -3.0,
            coast: CoastSettings {
                time: nan,
                softness: -4.0,
                min_speed: inf,
            },
            look_smoothing: -1.0,
            flip_degrees: 400.0,
            fast_multiplier: 0.0,
            slow_multiplier: -0.5,
            align_seconds: nan,
            ..NavPrefs::default()
        };
        let s = garbage.sanitized();
        let d = NavPrefs::default();
        assert_eq!(s.look_sensitivity, d.look_sensitivity);
        assert_eq!(s.fly_speed, 0.0);
        assert_eq!(
            s.coast,
            garbage.coast.sanitized(),
            "the coast sanitises itself"
        );
        assert_eq!(s.coast.time, d.coast.time, "NaN time -> default");
        assert_eq!(s.coast.softness, 0.0, "negative softness clamps");
        assert_eq!(s.coast.min_speed, d.coast.min_speed, "infinite -> default");
        assert_eq!(s.look_smoothing, 0.0);
        assert_eq!(s.flip_degrees, FLIP_DEGREES_RANGE.1);
        assert_eq!(s.fast_multiplier, FAST_MULTIPLIER_RANGE.0);
        assert_eq!(s.slow_multiplier, SLOW_MULTIPLIER_RANGE.0);
        assert_eq!(s.align_seconds, d.align_seconds);

        for radius in [nan, inf, -inf, -5.0, 0.0, 1e38] {
            let i = garbage.inertia(radius);
            let all = [
                i.look_sensitivity,
                i.cruise_speed,
                i.roll_rate,
                i.lock_rate,
                i.coast.time,
                i.coast.softness,
                i.coast.min_speed,
                i.look_smoothing,
                i.lock_flip,
                i.lock_hold,
                i.fast_multiplier,
                i.slow_multiplier,
            ];
            assert!(all.iter().all(|v| v.is_finite()), "radius {radius}: {i:?}");
            assert!(i.cruise_speed >= 0.0 && i.slow_multiplier > 0.0);
        }
        for fov in [nan, inf, 0.0, -1.0, 4.0] {
            for height in [nan, inf, -inf, 0.0] {
                let n = garbage.navigation(fov, height);
                assert!(
                    n.pan_sensitivity.is_finite() && n.pan_sensitivity > 0.0,
                    "fov {fov} h {height}: pan {}",
                    n.pan_sensitivity
                );
                assert!(
                    n.orbit_sensitivity.is_finite()
                        && n.coast.time.is_finite()
                        && n.coast.softness.is_finite()
                        && n.coast.min_speed.is_finite()
                );
            }
        }
        // An infinite height is a host bug: it falls back to 1 like NaN.
        let p = NavPrefs::default();
        let fov = 45.0_f32.to_radians();
        assert!(nav_bits_eq(
            &p.navigation(fov, inf),
            &studio_navigation(&p, fov, 1.0)
        ));
    }

    /// The inertia mapping of the coast model: cruise = 1.92 radii/s x fly speed, roll 2 rad/s,
    /// horizon-lock rate 1/s (the old 1 / rotate_decay), coast from the preferences.
    #[test]
    fn inertia_maps_the_coast_model() {
        let p = NavPrefs {
            fly_speed: 2.0,
            ..NavPrefs::default()
        };
        let i = p.inertia(5.0);
        assert!((i.cruise_speed - 1.92 * 5.0 * 2.0).abs() < 1e-5);
        assert_eq!((i.roll_rate, i.lock_rate), (2.0, 1.0));
        assert_eq!(i.coast, p.coast);
        // The default flight cruises at the old thrust / decay speed: 2 radii/s^2 x 0.96 s.
        let d = NavPrefs::default().inertia(1.0);
        assert!((d.cruise_speed - 2.0 * 0.96).abs() < 1e-6);
    }

    /// A preferences value written as JSON reads back equal (the coast included); a partial
    /// `coast` object takes the missing members from the coast defaults.
    #[cfg(feature = "serde")]
    #[test]
    fn prefs_serde_round_trip() {
        let p = NavPrefs {
            mode: NavMode::QuatFlightLocked,
            feel: NavFeel::Sharp,
            coast: CoastSettings {
                time: 1.75,
                softness: 6.5,
                min_speed: 0.125,
            },
            fly_speed: 3.25,
            ..NavPrefs::default()
        };
        let json = serde_json::to_string(&p).expect("serialise");
        assert!(!json.contains("decay"), "{json}");
        assert_eq!(
            serde_json::from_str::<NavPrefs>(&json).expect("round trip"),
            p
        );
        let partial: NavPrefs =
            serde_json::from_str(r#"{"coast":{"time":2.0}}"#).expect("partial coast");
        assert_eq!(partial.coast.time, 2.0);
        assert_eq!(partial.coast.softness, CoastSettings::default().softness);
        assert_eq!(partial.coast.min_speed, CoastSettings::default().min_speed);
    }

    /// A preferences file written before the coast model still carries `translate_decay` /
    /// `rotate_decay`: it loads (the keys are ignored), keeps its other values and takes the
    /// default coast.
    #[cfg(feature = "serde")]
    #[test]
    fn old_prefs_file_with_decay_keys_loads() {
        let old = r#"{"mode":"fps_fly","feel":"sharp","fly_speed":2.5,
            "translate_decay":0.5,"rotate_decay":2.0,"look_smoothing":0.1}"#;
        let p: NavPrefs = serde_json::from_str(old).expect("old file loads");
        assert_eq!(p.mode, NavMode::FpsFly);
        assert_eq!(p.feel, NavFeel::Sharp);
        assert_eq!((p.fly_speed, p.look_smoothing), (2.5, 0.1));
        assert_eq!(p.coast, CoastSettings::default());
        assert_eq!(p.align_seconds, NavPrefs::default().align_seconds);
        assert_eq!(p.sanitized().coast, CoastSettings::default());
    }
}
