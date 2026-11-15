//! Navigation modes ([`NavMode`]) and turn feel presets ([`NavFeel`]): the one switch over the
//! camera backends and the sharp / smooth presets of the inertia knobs.
//!
//! What: [`NavMode`] names the eight navigation styles the operator picks from; each maps to a
//! [`NavModeSpec`] (backend [`CameraKind`], turntable [`OrientationMode`], horizon lock,
//! [`MouseSteer`]) that [`crate::camera_core::CameraController::set_mode`] applies pose-preservingly and
//! [`crate::camera_core::CameraController::mode`] reads back. [`NavFeel`] writes the sharp / smooth turn
//! preset into [`InertiaSettings`] + [`CameraNavigationSettings`].
//!
//! Why: hosts offered the same style list with hand-rolled backend + flag combinations; one table
//! keeps every host's "mode" combo identical.
//!
//! How: plain `Copy` enums and a `const` table; no state. Uses `crate::camera_core::controller::CameraKind`,
//! `crate::camera_core::orientation`, `crate::camera_core::settings`, `crate::camera_core::convention::WORLD_UP`. Used by the controller
//! facade (`set_mode` / `mode`), `crate::camera_core::prefs`, host mode combos and the `input` layer.

use glam::Vec3;

use crate::camera_core::coast::CoastSettings;
use crate::camera_core::controller::CameraKind;
use crate::camera_core::convention::WORLD_UP;
use crate::camera_core::orientation::OrientationMode;
use crate::camera_core::settings::{CameraNavigationSettings, InertiaSettings, MouseSteer};

/// `coast` with a usable time: a positive finite time is the user's own and kept, anything else
/// (the 0 a Sharp preset wrote, a NaN) becomes the `CoastSettings` default time.
fn smooth_coast(coast: CoastSettings) -> CoastSettings {
    if coast.time.is_finite() && coast.time > 0.0 {
        coast
    } else {
        CoastSettings {
            time: CoastSettings::default().time,
            ..coast
        }
    }
}

/// One navigation style of the camera (the operator's mode list).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum NavMode {
    /// Horizon-plane turntable (the default): `HoudiniOrbit` +
    /// [`OrientationMode::HorizontalPlaneEuler`].
    #[default]
    Turntable,
    /// The 90s euler turntable: `HoudiniOrbit` + [`OrientationMode::Euler`].
    EulerTurntable,
    /// Gimbal-free quaternion orbit / trackball: `QuatOrbit`.
    Trackball,
    /// First-person fly (no roll): `FpsFly` with [`MouseSteer::Relative`] (pointer motion turns
    /// the head 1:1, the classic FPS look).
    FpsFly,
    /// Free 6-DoF space sim: `SpaceFlight`, no horizon lock, [`MouseSteer::Deflection`] (cursor
    /// offset from the centre sets the turn rate).
    SpaceSim,
    /// Horizon-locked space sim: `SpaceFlight` locked to world up ([`WORLD_UP`]),
    /// [`MouseSteer::Deflection`].
    SpaceSimLocked,
    /// Quaternion flight: `SpaceFlight`, no horizon lock, [`MouseSteer::Relative`] (pointer
    /// motion turns the ship 1:1, roll keys roll it).
    QuatFlight,
    /// Horizon-locked quaternion flight: `SpaceFlight` locked to world up ([`WORLD_UP`]),
    /// [`MouseSteer::Relative`] (pointer motion turns the ship 1:1 around the locked horizon).
    QuatFlightLocked,
}

/// What a [`NavMode`] sets on the controller. The mode is THE owner of these fields: hosts never
/// set them another way (`CameraController::apply_inertia` keeps the steering, the lock lives
/// only here). `None` / unused fields do not apply to the mode's backend.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavModeSpec {
    /// The backend the mode runs on.
    pub kind: CameraKind,
    /// The turntable orientation policy (`HoudiniOrbit` only).
    pub orientation: Option<OrientationMode>,
    /// The horizon-lock axis (`SpaceFlight` only; `None` = free 6-DoF there).
    pub horizon_lock: Option<Vec3>,
    /// The mouse steering model; written into the fly backends, [`MouseSteer::Relative`] (and
    /// unused) on the orbit modes.
    pub mouse_steer: MouseSteer,
}

impl NavMode {
    /// Every mode, in menu order.
    pub const ALL: [NavMode; 8] = [
        NavMode::Turntable,
        NavMode::EulerTurntable,
        NavMode::Trackball,
        NavMode::FpsFly,
        NavMode::SpaceSim,
        NavMode::SpaceSimLocked,
        NavMode::QuatFlight,
        NavMode::QuatFlightLocked,
    ];

    /// Human-readable name (menus, HUD).
    pub fn label(self) -> &'static str {
        match self {
            NavMode::Turntable => "Turntable",
            NavMode::EulerTurntable => "Euler turntable",
            NavMode::Trackball => "Trackball",
            NavMode::FpsFly => "FPS fly",
            NavMode::SpaceSim => "Space sim",
            NavMode::SpaceSimLocked => "Space sim (horizon lock)",
            NavMode::QuatFlight => "Quaternion flight",
            NavMode::QuatFlightLocked => "Flight (horizon lock)",
        }
    }

    /// The backend, orientation policy, horizon lock and mouse steering of this mode.
    pub fn spec(self) -> NavModeSpec {
        let orbit = |orientation| NavModeSpec {
            kind: CameraKind::Houdini,
            orientation: Some(orientation),
            horizon_lock: None,
            mouse_steer: MouseSteer::Relative,
        };
        let space = |horizon_lock, mouse_steer| NavModeSpec {
            kind: CameraKind::Space,
            orientation: None,
            horizon_lock,
            mouse_steer,
        };
        match self {
            NavMode::Turntable => orbit(OrientationMode::HorizontalPlaneEuler),
            NavMode::EulerTurntable => orbit(OrientationMode::Euler),
            NavMode::Trackball => NavModeSpec {
                kind: CameraKind::Quat,
                orientation: None,
                horizon_lock: None,
                mouse_steer: MouseSteer::Relative,
            },
            NavMode::FpsFly => NavModeSpec {
                kind: CameraKind::Fps,
                orientation: None,
                horizon_lock: None,
                mouse_steer: MouseSteer::Relative,
            },
            NavMode::SpaceSim => space(None, MouseSteer::Deflection),
            NavMode::SpaceSimLocked => space(Some(WORLD_UP), MouseSteer::Deflection),
            NavMode::QuatFlight => space(None, MouseSteer::Relative),
            NavMode::QuatFlightLocked => space(Some(WORLD_UP), MouseSteer::Relative),
        }
    }

    /// The backend this mode runs on.
    pub fn kind(self) -> CameraKind {
        self.spec().kind
    }

    /// The mouse steering model of this mode, THE owner of `InertiaSettings::mouse_steer`:
    /// FpsFly / QuatFlight / QuatFlightLocked [`MouseSteer::Relative`], SpaceSim / SpaceSimLocked
    /// [`MouseSteer::Deflection`], the orbit modes [`MouseSteer::Relative`] (unused there).
    pub fn mouse_steer(self) -> MouseSteer {
        self.spec().mouse_steer
    }

    /// True for the fly modes (FpsFly, SpaceSim, SpaceSimLocked, QuatFlight, QuatFlightLocked):
    /// the mode runs on a fly backend and its input is fly input.
    pub fn is_fly(self) -> bool {
        matches!(self.kind(), CameraKind::Fps | CameraKind::Space)
    }
}

/// Turn feel preset: how turns start and end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum NavFeel {
    /// Sharp turns: the fly look turns the view at once (`inertial_look = false`) and every
    /// release stops dead (`coast.time = 0`).
    Sharp,
    /// Smooth turns (the studio default): the fly look eases onto the pointer's turn
    /// (`inertial_look = true` with a positive `look_smoothing`) and releases glide to rest
    /// through the preference's coast (`coast.time > 0`).
    #[default]
    Smooth,
}

impl NavFeel {
    /// Both presets, in menu order.
    pub const ALL: [NavFeel; 2] = [NavFeel::Sharp, NavFeel::Smooth];

    /// Human-readable name (menus).
    pub fn label(self) -> &'static str {
        match self {
            NavFeel::Sharp => "Sharp",
            NavFeel::Smooth => "Smooth",
        }
    }

    /// Write the preset into both settings ([`NavFeel::apply_inertia`] +
    /// [`NavFeel::apply_navigation`]). Exactly these fields change:
    /// - [`InertiaSettings::inertial_look`]: `false` (Sharp) / `true` (Smooth);
    /// - [`InertiaSettings::look_smoothing`]: Smooth only, and only when the current value is
    ///   not a positive finite time constant (it is then set to the `InertiaSettings` default);
    ///   a user's own smoothing is kept;
    /// - `coast.time` of both settings (`InertiaSettings::coast`,
    ///   `CameraNavigationSettings::coast`): `0` (Sharp); Smooth keeps a positive finite time
    ///   (the user's own) and otherwise sets the `CoastSettings` default time, so Sharp -> Smooth
    ///   round-trips. The coast's softness and minimum speed are tuning, not feel.
    ///
    /// Cruise speed, roll rate and thrust response are tuning, not feel, and stay as they are.
    pub fn apply(self, inertia: &mut InertiaSettings, nav: &mut CameraNavigationSettings) {
        self.apply_inertia(inertia);
        self.apply_navigation(nav);
    }

    /// The coast a preset runs with, given the user's coast `base`: Sharp is `base` with
    /// `time = 0` (every release stops dead), Smooth is `base` unchanged. The one mapping
    /// `NavPrefs::inertia` / `NavPrefs::navigation` and the two `apply_*` methods share.
    pub fn coast(self, base: CoastSettings) -> CoastSettings {
        match self {
            NavFeel::Sharp => CoastSettings { time: 0.0, ..base },
            NavFeel::Smooth => base,
        }
    }

    /// The fly half of [`NavFeel::apply`]: `inertial_look` (+ `look_smoothing` for Smooth) and,
    /// for Sharp, `coast.time = 0`.
    pub fn apply_inertia(self, inertia: &mut InertiaSettings) {
        match self {
            NavFeel::Sharp => {
                inertia.inertial_look = false;
                inertia.coast = self.coast(inertia.coast);
            }
            NavFeel::Smooth => {
                inertia.inertial_look = true;
                let usable = inertia.look_smoothing.is_finite() && inertia.look_smoothing > 0.0;
                if !usable {
                    inertia.look_smoothing = InertiaSettings::default().look_smoothing;
                }
                inertia.coast = smooth_coast(inertia.coast);
            }
        }
    }

    /// The orbit half of [`NavFeel::apply`]: the coast time (`0` for Sharp, kept or defaulted
    /// for Smooth).
    pub fn apply_navigation(self, nav: &mut CameraNavigationSettings) {
        nav.coast = match self {
            NavFeel::Sharp => self.coast(nav.coast),
            NavFeel::Smooth => smooth_coast(nav.coast),
        };
    }

    /// The preset the settings currently match: Sharp for no inertial look and both coast times
    /// 0, Smooth for an inertial look with a positive smoothing and an orbit coast time above 0,
    /// `None` for a mix.
    pub fn of(inertia: &InertiaSettings, nav: &CameraNavigationSettings) -> Option<NavFeel> {
        if !inertia.inertial_look && inertia.coast.time == 0.0 && nav.coast.time == 0.0 {
            Some(NavFeel::Sharp)
        } else if inertia.inertial_look && inertia.look_smoothing > 0.0 && nav.coast.time > 0.0 {
            Some(NavFeel::Smooth)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera_core::controller::{CameraController, SpaceFlight};
    use crate::camera_core::pose::CameraPose;
    use crate::camera_core::projection::PerspectiveSettings;

    /// A level (roll-free) pose looking down and to the side, with its own lens.
    fn level_pose(projection: PerspectiveSettings) -> CameraPose {
        CameraPose::from_eye_forward_up(
            Vec3::new(3.0, 4.0, 10.0),
            Vec3::new(-0.3, -0.4, -1.0),
            Vec3::Y,
            projection.fov_y,
            projection.near,
            projection.far,
        )
    }

    fn custom_projection() -> PerspectiveSettings {
        PerspectiveSettings {
            fov_y: 50.0_f32.to_radians(),
            near: 0.2,
            far: 900.0,
            distance_min: 2.0,
            distance_max: 2000.0,
        }
    }

    /// From every mode to every mode: the eye, forward, up and the whole lens (fov, clips AND
    /// the distance range) survive the switch, and `mode()` reports the new mode.
    #[test]
    fn set_mode_preserves_the_view_across_every_mode() {
        let projection = custom_projection();
        let pose = level_pose(projection);
        for from in NavMode::ALL {
            for to in NavMode::ALL {
                let mut ctrl = CameraController::from_pose(from.kind(), pose);
                ctrl.set_projection(projection);
                ctrl.set_pose(pose);
                ctrl.set_mode(from);
                assert_eq!(ctrl.mode(), Some(from), "{from:?}");
                ctrl.set_mode(to);
                assert_eq!(ctrl.mode(), Some(to), "{from:?} -> {to:?}");
                let got = ctrl.pose();
                assert!(
                    got.eye.distance(pose.eye) < 1e-3,
                    "{from:?} -> {to:?}: eye {} vs {}",
                    got.eye,
                    pose.eye
                );
                assert!(
                    got.forward().dot(pose.forward()) > 0.9999,
                    "{from:?} -> {to:?}: forward {} vs {}",
                    got.forward(),
                    pose.forward()
                );
                assert!(
                    got.up().dot(pose.up()) > 0.9999,
                    "{from:?} -> {to:?}: up {} vs {}",
                    got.up(),
                    pose.up()
                );
                assert_eq!(ctrl.projection(), projection, "{from:?} -> {to:?}");
                assert_eq!(ctrl.camera_kind(), to.kind(), "{from:?} -> {to:?}");
            }
        }
    }

    /// The mode table: backend, orientation, lock and steering of each mode; distinct labels.
    #[test]
    fn mode_spec_table() {
        use CameraKind::{Fps, Houdini, Quat, Space};
        use MouseSteer::{Deflection, Relative};
        use OrientationMode::{Euler, HorizontalPlaneEuler};
        let expected = [
            (
                NavMode::Turntable,
                Houdini,
                Some(HorizontalPlaneEuler),
                None,
                Relative,
            ),
            (
                NavMode::EulerTurntable,
                Houdini,
                Some(Euler),
                None,
                Relative,
            ),
            (NavMode::Trackball, Quat, None, None, Relative),
            (NavMode::FpsFly, Fps, None, None, Relative),
            (NavMode::SpaceSim, Space, None, None, Deflection),
            (
                NavMode::SpaceSimLocked,
                Space,
                None,
                Some(WORLD_UP),
                Deflection,
            ),
            (NavMode::QuatFlight, Space, None, None, Relative),
            (
                NavMode::QuatFlightLocked,
                Space,
                None,
                Some(WORLD_UP),
                Relative,
            ),
        ];
        assert_eq!(expected.len(), NavMode::ALL.len());
        for (i, (mode, kind, orientation, lock, steer)) in expected.into_iter().enumerate() {
            assert_eq!(mode, NavMode::ALL[i], "ALL order");
            assert_eq!(mode.mouse_steer(), steer, "{mode:?}");
            assert_eq!(mode.is_fly(), matches!(kind, Fps | Space), "{mode:?}");
            assert_eq!(
                mode.spec(),
                NavModeSpec {
                    kind,
                    orientation,
                    horizon_lock: lock,
                    mouse_steer: steer,
                },
                "{mode:?}"
            );
        }
        for (i, a) in NavMode::ALL.iter().enumerate() {
            for b in &NavMode::ALL[i + 1..] {
                assert_ne!(a.label(), b.label());
            }
        }
        assert_eq!(NavMode::default(), NavMode::Turntable);
        assert_eq!(NavMode::QuatFlightLocked.label(), "Flight (horizon lock)");
        assert_eq!(NavMode::ALL.len(), 8);
    }

    /// Every mode names itself in serde as its snake_case variant and reads back; the new mode
    /// is `quat_flight_locked`.
    #[cfg(feature = "serde")]
    #[test]
    fn mode_serde_round_trips_all_eight() {
        for mode in NavMode::ALL {
            let json = serde_json::to_string(&mode).expect("serialise");
            let back: NavMode = serde_json::from_str(&json).expect("deserialise");
            assert_eq!(back, mode, "{json}");
        }
        assert_eq!(
            serde_json::to_string(&NavMode::QuatFlightLocked).expect("serialise"),
            "\"quat_flight_locked\""
        );
    }

    /// The horizon-locked flight modes differ from their free twins only by the lock, and the
    /// two locked modes differ from each other only by the steering.
    #[test]
    fn locked_modes_differ_by_the_lock_and_the_steering() {
        let free = NavMode::QuatFlight.spec();
        let locked = NavMode::QuatFlightLocked.spec();
        assert_eq!(free.kind, locked.kind);
        assert_eq!(free.mouse_steer, locked.mouse_steer);
        assert_eq!(
            (free.horizon_lock, locked.horizon_lock),
            (None, Some(WORLD_UP))
        );
        let sim = NavMode::SpaceSimLocked.spec();
        assert_eq!(sim.horizon_lock, locked.horizon_lock);
        assert_ne!(sim.mouse_steer, locked.mouse_steer);
    }

    /// `mode()` cannot name the legacy quaternion orientation of the Houdini rig.
    #[test]
    fn mode_is_none_for_the_legacy_quaternion_orientation() {
        let mut ctrl = CameraController::default();
        assert_eq!(ctrl.mode(), Some(NavMode::Turntable));
        let CameraController::Houdini(b) = &mut ctrl else {
            panic!("the default controller is the Houdini orbit");
        };
        b.rig.orientation_mode = OrientationMode::Quaternion;
        assert_eq!(ctrl.mode(), None);
    }

    /// A switch within one backend keeps its tuning (no rebuild): the space sim's thrust and
    /// lens survive Space sim -> locked -> quaternion flight.
    #[test]
    fn same_backend_switch_keeps_tuning() {
        let mut space = SpaceFlight::default();
        space.inertia.cruise_speed = 999.0;
        let mut ctrl = CameraController::Space(space);
        ctrl.set_mode(NavMode::SpaceSimLocked);
        assert_eq!(ctrl.horizon_lock(), Some(WORLD_UP));
        ctrl.set_mode(NavMode::QuatFlight);
        assert_eq!(ctrl.horizon_lock(), None);
        let CameraController::Space(b) = &ctrl else {
            panic!("expected the Space backend");
        };
        assert_eq!(b.inertia.cruise_speed, 999.0);
        assert_eq!(b.inertia.mouse_steer, MouseSteer::Relative);
    }

    /// Orbit <-> orbit keeps the pivot: a turntable framed close (distance 3) switched to the
    /// trackball keeps its target and distance (not the default 28 ahead of the eye), keeps its
    /// navigation tuning, and comes back the same way.
    #[test]
    fn orbit_switch_carries_the_pivot_and_the_navigation() {
        let mut ctrl = CameraController::default();
        ctrl.frame_bounds(
            Vec3::new(4.0, 1.0, -2.0),
            Vec3::new(4.5, 1.5, -1.5),
            800,
            600,
            1.0,
        );
        let CameraController::Houdini(b) = &mut ctrl else {
            panic!("the default controller is the Houdini orbit");
        };
        b.rig.pose.distance = 3.0;
        let target = b.rig.pose.target;
        ctrl.apply_navigation(CameraNavigationSettings {
            pan_sensitivity: 0.042,
            coast: CoastSettings {
                time: 1.5,
                ..CoastSettings::default()
            },
            ..CameraNavigationSettings::default()
        });
        let eye = ctrl.pose().eye;

        ctrl.set_mode(NavMode::Trackball);
        let CameraController::Quat(q) = &ctrl else {
            panic!("expected the trackball");
        };
        assert!(
            q.target.distance(target) < 1e-4,
            "target {} vs {target}",
            q.target
        );
        assert!((q.distance - 3.0).abs() < 1e-5, "distance {}", q.distance);
        assert_eq!(q.navigation.pan_sensitivity, 0.042);
        assert_eq!(q.navigation.coast.time, 1.5);
        assert!(ctrl.pose().eye.distance(eye) < 1e-4);

        ctrl.set_mode(NavMode::EulerTurntable);
        let CameraController::Houdini(b) = &ctrl else {
            panic!("expected the turntable");
        };
        assert!(b.rig.pose.target.distance(target) < 1e-4);
        assert!((b.rig.pose.distance - 3.0).abs() < 1e-5);
        assert_eq!(b.rig.navigation.pan_sensitivity, 0.042);
        assert_eq!(b.rig.navigation.coast.time, 1.5);
        assert!(ctrl.pose().eye.distance(eye) < 1e-3);
    }

    /// Fly <-> fly keeps the fly tuning and the held speed scale: a fast FPS flight switched to
    /// the space sim keeps flying fast, with the mode's own steering.
    #[test]
    fn fly_switch_carries_the_inertia_and_the_speed_scale() {
        let mut ctrl = CameraController::default();
        ctrl.set_mode(NavMode::FpsFly);
        ctrl.apply_inertia(InertiaSettings {
            cruise_speed: 123.0,
            ..InertiaSettings::default()
        });
        let vp = crate::camera_core::viewport::ViewportSize::new(1, 1);
        ctrl.apply_intent(
            crate::camera_core::intent::CameraIntent::SpeedScale(4.0),
            0.0,
            vp,
        );

        ctrl.set_mode(NavMode::SpaceSim);
        assert_eq!(ctrl.speed_scale(), 4.0, "the held speed scale survived");
        let CameraController::Space(s) = &ctrl else {
            panic!("expected the space sim");
        };
        assert_eq!(s.inertia.cruise_speed, 123.0);
        assert_eq!(
            s.inertia.mouse_steer,
            MouseSteer::Deflection,
            "the mode's steering"
        );
    }

    /// Steering has one owner, the mode: re-applying fly tuning with another steering never
    /// flips the steering (so never `mode()`).
    #[test]
    fn apply_inertia_keeps_the_mode_steering() {
        for mode in [NavMode::SpaceSim, NavMode::QuatFlight, NavMode::FpsFly] {
            let mut ctrl = CameraController::default();
            ctrl.set_mode(mode);
            for steer in [MouseSteer::Relative, MouseSteer::Deflection] {
                let inertia = InertiaSettings {
                    mouse_steer: steer,
                    cruise_speed: 9.0,
                    ..InertiaSettings::default()
                };
                ctrl.apply_inertia(inertia);
                assert_eq!(ctrl.mode(), Some(mode), "{mode:?} after {steer:?}");
                let got = match &ctrl {
                    CameraController::Fps(b) => b.inertia,
                    CameraController::Space(b) => b.inertia,
                    other => panic!("{} is not a fly backend", other.kind()),
                };
                let kept = (got.mouse_steer, got.cruise_speed);
                assert_eq!(kept, (mode.mouse_steer(), 9.0), "{mode:?}");
            }
        }
    }

    /// The presets write exactly their documented fields and `of` reads them back.
    #[test]
    fn nav_feel_presets() {
        let mut inertia = InertiaSettings {
            look_smoothing: 0.2,
            cruise_speed: 7.0,
            coast: CoastSettings {
                time: 1.2,
                softness: 4.0,
                min_speed: 0.05,
            },
            ..InertiaSettings::default()
        };
        let mut nav = CameraNavigationSettings {
            coast: inertia.coast,
            ..CameraNavigationSettings::default()
        };

        NavFeel::Sharp.apply(&mut inertia, &mut nav);
        assert!(!inertia.inertial_look);
        assert_eq!(nav.coast.time, 0.0);
        assert_eq!(inertia.coast.time, 0.0);
        assert_eq!(
            (nav.coast.softness, nav.coast.min_speed),
            (4.0, 0.05),
            "only the time is feel"
        );
        assert_eq!(inertia.look_smoothing, 0.2, "Sharp keeps the smoothing");
        assert_eq!(inertia.cruise_speed, 7.0, "not a feel field");
        assert_eq!(NavFeel::of(&inertia, &nav), Some(NavFeel::Sharp));

        // Sharp -> Smooth: the time returns (the default; the user's 1.2 was overwritten).
        NavFeel::Smooth.apply(&mut inertia, &mut nav);
        assert!(inertia.inertial_look);
        assert_eq!(nav.coast.time, CoastSettings::default().time);
        assert_eq!(inertia.coast.time, CoastSettings::default().time);
        assert_eq!(inertia.look_smoothing, 0.2, "a user smoothing is kept");
        assert_eq!(NavFeel::of(&inertia, &nav), Some(NavFeel::Smooth));
        // Smooth over a user time keeps it.
        nav.coast.time = 2.5;
        NavFeel::Smooth.apply_navigation(&mut nav);
        assert_eq!(nav.coast.time, 2.5);

        for bad in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            inertia.look_smoothing = bad;
            NavFeel::Smooth.apply(&mut inertia, &mut nav);
            assert_eq!(
                inertia.look_smoothing,
                InertiaSettings::default().look_smoothing,
                "smoothing {bad}"
            );
        }

        inertia.inertial_look = false;
        assert_eq!(NavFeel::of(&inertia, &nav), None, "a mix is no preset");
        assert_eq!(NavFeel::default(), NavFeel::Smooth);
    }

    /// `NavFeel::coast` maps a base coast: Sharp zeroes the time only, Smooth is the identity.
    #[test]
    fn nav_feel_coast_mapping() {
        let base = CoastSettings {
            time: 0.9,
            softness: 2.0,
            min_speed: 0.3,
        };
        assert_eq!(NavFeel::Smooth.coast(base), base);
        assert_eq!(
            NavFeel::Sharp.coast(base),
            CoastSettings { time: 0.0, ..base }
        );
    }
}
