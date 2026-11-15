//! Navigation tuning: [`CameraNavigationSettings`] (orbit rig: zoom mode, sensitivities, release
//! coast) and [`InertiaSettings`] (fly / spaceflight rigs: look, cruise thrust, roll rate, horizon
//! lock, release coast, mouse steering).
//!
//! What: plain data with defaults. Every release motion (an orbit / pan / zoom drag, a look drag,
//! released thrust) glides to rest through one [`CoastSettings`] (`crate::camera_core::coast`); held thrust is a
//! commanded velocity (`cruise_speed`) the rig's velocity approaches with `thrust_response`.
//! Why: one finite-time coast model replaces the old per-rig exponential damping and brake knobs.
//! How: serde derives behind the `serde` feature, every container `#[serde(default)]` so presets
//! missing a field (including files written before the coast model) load with its default.
//! Uses `crate::camera_core::coast::CoastSettings`. Used by the rigs, `crate::camera_core::prefs` and `crate::camera_core::mode`.

use crate::camera_core::coast::CoastSettings;

/// Gain on the RMB dolly exponent in [`ZoomMode::Exponential`]: gentler than the wheel's so a
/// drag-dolly feels like the wheel.
const DOLLY_EXPONENTIAL_GAIN: f32 = 0.15;

/// How wheel / dolly input maps to a multiplicative distance factor.
///
/// The enum's own `Default` is `Exponential`, but [`CameraNavigationSettings::default`] picks
/// `Linear` (the navigation default).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ZoomMode {
    /// `factor ≈ 1 - scroll * linear_rate` (clamped). The navigation settings' default.
    Linear,
    /// `factor ≈ exp(-scroll * exponential_rate)` (clamped): stable relative zoom.
    #[default]
    Exponential,
}

/// Single source of truth for tree-style orbit navigation (UI gather + GPU worker pan scale).
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))] // missing fields fall back to `Default`
pub struct CameraNavigationSettings {
    /// Wheel / dolly mapping (see [`ZoomMode`]).
    pub zoom_mode: ZoomMode,
    /// When `true`, the mouse wheel zooms toward the cursor (cursor-anchored).
    /// When `false` (default), it zooms straight toward the camera target /
    /// centre of aim — the steadier, less disorienting behaviour.
    pub zoom_to_cursor: bool,
    /// Radians of yaw / pitch per pixel of orbit drag.
    pub orbit_sensitivity: f32,
    /// Per unit of the host's (smoothed) vertical scroll delta when [`ZoomMode::Linear`].
    pub zoom_scroll_linear: f32,
    /// Per pixel of scroll when [`ZoomMode::Exponential`]: `exp(-scroll * rate)`.
    pub zoom_scroll_exponential: f32,
    /// Pan scale: `target += axis * distance * pan_sensitivity`.
    pub pan_sensitivity: f32,
    /// RMB dolly; uses the same [`ZoomMode`] as the wheel.
    pub dolly_sensitivity: f32,
    /// Release coast of an orbit / pan / zoom drag: the motion glides to rest from the drag's own
    /// release velocity in `coast.time` seconds. `coast.time == 0` stops dead
    /// (`NavFeel::Sharp`); the default is the `NavFeel::Smooth` coast. Owned by the feel preset.
    pub coast: CoastSettings,
    /// Invert the horizontal (yaw) orbit-drag axis (off by default, the historical behaviour).
    pub invert_orbit_x: bool,
    /// Invert the vertical (pitch) orbit-drag axis.
    pub invert_orbit_y: bool,
}

impl Default for CameraNavigationSettings {
    fn default() -> Self {
        Self {
            zoom_mode: ZoomMode::Linear,
            zoom_to_cursor: false,
            orbit_sensitivity: 0.008,
            zoom_scroll_linear: 0.0025,
            zoom_scroll_exponential: 0.0035,
            pan_sensitivity: 0.0025,
            dolly_sensitivity: 0.012,
            coast: CoastSettings::default(),
            invert_orbit_x: false,
            invert_orbit_y: false,
        }
    }
}

impl CameraNavigationSettings {
    // Per-FRAME zoom cap. Hosts with smooth scrolling smear one wheel notch across several
    // frames, so a loose cap (e.g. 0.85) compounds (0.85^6 ≈ 0.38 → ~62% of the distance
    // gone in one notch — the "whole cloud in one click" jump). A tight per-frame cap keeps
    // each notch smooth regardless of how many frames the host spreads it over.
    const ZOOM_CLAMP_MIN: f32 = 0.97;
    const ZOOM_CLAMP_MAX: f32 = 1.031;

    /// Multiplicative distance factor from one frame of vertical scroll (host scroll units).
    pub fn scroll_zoom_factor(&self, scroll_y: f32) -> f32 {
        if scroll_y.abs() < f32::EPSILON {
            return 1.0;
        }
        let factor = match self.zoom_mode {
            ZoomMode::Linear => 1.0 - scroll_y * self.zoom_scroll_linear,
            ZoomMode::Exponential => (-scroll_y * self.zoom_scroll_exponential).exp(),
        };
        factor.clamp(Self::ZOOM_CLAMP_MIN, Self::ZOOM_CLAMP_MAX)
    }

    /// Multiplicative distance factor from RMB vertical drag (pixels this frame).
    pub fn dolly_zoom_factor(&self, drag_dy_px: f32) -> f32 {
        if drag_dy_px.abs() < f32::EPSILON {
            return 1.0;
        }
        let factor = match self.zoom_mode {
            ZoomMode::Linear => 1.0 - drag_dy_px * self.dolly_sensitivity,
            ZoomMode::Exponential => {
                (-drag_dy_px * self.dolly_sensitivity * DOLLY_EXPONENTIAL_GAIN).exp()
            }
        };
        factor.clamp(Self::ZOOM_CLAMP_MIN, Self::ZOOM_CLAMP_MAX)
    }
}

/// Mouse steering model for the fly backends (`FpsFly` / `SpaceFlight`).
///
/// `Relative` is the classic FPS look (per-frame pointer motion turns the view
/// 1:1). `Deflection` is space-sim rate steering: the cursor's offset from the
/// viewport centre sets a continuous yaw/pitch turn rate, so holding the cursor
/// off-centre keeps the ship turning. Both fly backends integrate the held deflection
/// (`CameraIntent::LookRate`) with the real `dt`; a host input layer chooses which intent it
/// emits (`Look` for `Relative`, `LookRate` for `Deflection`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseSteer {
    /// Per-frame pointer motion turns the view 1:1 (classic FPS look).
    #[default]
    Relative,
    /// Cursor offset from the viewport centre → continuous yaw/pitch turn rate.
    Deflection,
}

/// Per-backend tuning for the fly / spaceflight rigs (sensitivity, cruise thrust, coast).
///
/// **What:** sensitivities convert raw input (pixels of mouse drag, normalized thrust axis
/// values) into angular / linear velocity. Held thrust commands the velocity
/// `axes * cruise_speed * speed_scale` (the rig's velocity approaches it with time constant
/// `thrust_response`); released thrust, a released look drag and a released roll glide to rest
/// through `coast`. `fast_multiplier` / `slow_multiplier` scale the thrust while the fast / slow
/// modifier is held ([`InertiaSettings::speed_scale`]).
///
/// **Why:** one struct is the single persisted source of truth for the thrust / coast /
/// sensitivity knobs; the orbit rig uses [`CameraNavigationSettings`].
///
/// **Where:** [`crate::camera_core::controller::FpsFly`] and [`crate::camera_core::controller::SpaceFlight`].
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))] // missing fields fall back to `Default`
pub struct InertiaSettings {
    /// Radians of yaw/pitch per pixel of mouse-look drag.
    pub look_sensitivity: f32,
    /// Cruise speed (world units/s) of a fully held thrust axis at speed scale 1: held thrust
    /// commands `axes * cruise_speed * speed_scale`. Also the unit of the coast's `min_speed`
    /// for released linear motion (a fraction of the cruise speed per second).
    pub cruise_speed: f32,
    /// Roll rate (rad/s) of a fully held roll axis (Q/E) of the free-flight spaceflight rig; the
    /// roll velocity approaches it with `thrust_response`. Never a roll coast under a horizon
    /// lock.
    pub roll_rate: f32,
    /// Time constant (seconds) with which the thrust / roll velocity approaches its held command
    /// (closed form, frame-rate independent); 0 = the velocity equals the command at once.
    pub thrust_response: f32,
    /// Horizon lock: the levelling spring's speed (1/s); `NavPrefs::inertia` sets 1. The lock's
    /// held-omega clamp bounds it.
    pub lock_rate: f32,
    /// Release coast of released thrust, a released look drag and a released free roll:
    /// the motion glides to rest from its release velocity in `coast.time` seconds
    /// (`coast.time == 0` = stops dead, `NavFeel::Sharp`).
    pub coast: CoastSettings,
    /// Thrust multiplier while the "fast" modifier is held (Shift).
    pub fast_multiplier: f32,
    /// Thrust multiplier while the "slow" modifier is held (Alt).
    pub slow_multiplier: f32,
    /// Mouse look is smoothed instead of turning the view at once: the view chases the
    /// pointer's total turn with time constant `look_smoothing` and stops where the pointer
    /// stopped (no coast past it). Spaceflight only.
    pub inertial_look: bool,
    /// Time constant (seconds) of the smoothed mouse look (`inertial_look`): 0.05..0.1 reads
    /// as smooth yet immediate; longer lags the hand.
    pub look_smoothing: f32,
    /// Horizon lock: the roll (radians) to the next grid plane a flip aims for (the grid
    /// planes are a quarter turn apart, so pi/2); <= 0 disables flips.
    pub lock_step: f32,
    /// Horizon lock: roll angle (radians) beyond which the lock flips to the next plane.
    pub lock_flip: f32,
    /// Horizon lock: roll angle (radians) a fully held roll key reaches against the levelling
    /// spring (whose speed is `lock_rate`). Above `lock_flip`, so holding the key flips
    /// and a short press springs back.
    pub lock_hold: f32,
    /// Invert the horizontal (yaw) mouse-look axis for the fly backends. Defaults
    /// ON — most users expect cursor-right to turn the ship's nose right in
    /// deflection steering. Filled from the container `#[serde(default)]`.
    pub invert_look_x: bool,
    /// Invert the vertical (pitch) mouse-look axis for the fly backends.
    pub invert_look_y: bool,
    /// Mouse steering model (see [`MouseSteer`]) of both fly backends.
    pub mouse_steer: MouseSteer,
    /// Turn rate (rad/s) at FULL cursor deflection in `MouseSteer::Deflection`.
    pub steer_rate: f32,
    /// Dead-zone as a fraction (0..1) of the half-viewport: deflection magnitudes
    /// below this map to zero turn, so a near-centred cursor holds heading steady.
    pub steer_deadzone: f32,
    /// Draw the steering reticle overlay (neutral cross + deflection dot) in the
    /// viewport while steering in `MouseSteer::Deflection` — on-screen feedback of
    /// the current deflection (essential when the cursor is hidden via capture).
    pub steer_reticle: bool,
    /// Steering neutral-point ORIGIN: `true` = fixed at the viewport CENTRE
    /// (absolute — the reticle sits in the middle of the screen); `false` =
    /// captured at the cursor position when fly engages (relative — no snap on
    /// entry, but the neutral point can sit anywhere). `MouseSteer::Deflection` only.
    pub steer_center_screen: bool,
    /// Steer decay (relative origin only): when set, the steering anchor drifts
    /// toward the cursor, so holding the mouse still eases the turn back to neutral
    /// (the ship turns only while the mouse is *moving*). When clear, deflection
    /// persists — hold off the neutral point to keep turning.
    pub steer_decay: bool,
    /// Time constant (seconds) of the steer-decay drift — larger = slower return to
    /// neutral. Only used when `steer_decay` is set.
    pub steer_decay_secs: f32,
    /// Hide the OS cursor while flying (game-engine style capture). Default OFF.
    /// A request to HIDE the cursor only, never to grab / confine it (confining froze pointer
    /// input in a host windowing backend). Deflection reads the cursor's live absolute
    /// position regardless.
    pub steer_capture_cursor: bool,
}

impl Default for InertiaSettings {
    fn default() -> Self {
        Self {
            look_sensitivity: 0.005,
            cruise_speed: 10.0,
            roll_rate: 0.75,
            thrust_response: 0.25,
            lock_rate: 8.0,
            coast: CoastSettings::default(),
            fast_multiplier: 4.0,
            slow_multiplier: 0.1,
            inertial_look: false,
            look_smoothing: 0.08,
            lock_step: std::f32::consts::FRAC_PI_2,
            // Tuned together with the directional HorizonLock flip.
            lock_flip: 38.0_f32.to_radians(),
            lock_hold: 85.0_f32.to_radians(),
            invert_look_x: true,
            invert_look_y: false,
            mouse_steer: MouseSteer::Deflection,
            steer_rate: 2.0,
            steer_deadzone: 0.06,
            steer_reticle: true,
            steer_center_screen: true,
            steer_decay: false,
            steer_decay_secs: 0.4,
            steer_capture_cursor: false,
        }
    }
}

impl InertiaSettings {
    /// Thrust scale for the held speed modifiers: fast (Shift) x `fast_multiplier`, slow (Alt)
    /// x `slow_multiplier`, both at once multiply. The one mapping every host uses.
    pub fn speed_scale(&self, fast: bool, slow: bool) -> f32 {
        let mut scale = 1.0;
        if fast {
            scale *= self.fast_multiplier;
        }
        if slow {
            scale *= self.slow_multiplier;
        }
        scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_zoom_mode_is_linear() {
        assert_eq!(
            CameraNavigationSettings::default().zoom_mode,
            ZoomMode::Linear
        );
    }

    /// The horizon-lock defaults are 38 / 85 degrees, and a held roll key (lock_hold) lies
    /// past the flip angle so holding flips while a short press springs back.
    #[test]
    fn horizon_lock_defaults_are_38_85() {
        let s = InertiaSettings::default();
        assert!((s.lock_flip.to_degrees() - 38.0).abs() < 1e-4);
        assert!((s.lock_hold.to_degrees() - 85.0).abs() < 1e-4);
        assert!(s.lock_hold > s.lock_flip);
    }

    /// The coast-model defaults: 10 units/s cruise, 0.75 rad/s roll, 0.25 s thrust response,
    /// lock rate 8/s, and the default coast in both settings.
    #[test]
    fn coast_model_defaults() {
        let i = InertiaSettings::default();
        assert_eq!(
            (i.cruise_speed, i.roll_rate, i.thrust_response, i.lock_rate),
            (10.0, 0.75, 0.25, 8.0)
        );
        assert_eq!(i.coast, CoastSettings::default());
        assert_eq!(
            CameraNavigationSettings::default().coast,
            CoastSettings::default()
        );
    }

    /// A settings file written before the coast model (removed keys, no `coast`) loads: unknown
    /// keys are ignored and the new fields take their defaults.
    #[cfg(feature = "serde")]
    #[test]
    fn old_settings_files_load() {
        let old = r#"{"look_sensitivity":0.01,"linear_damping":4.0,"angular_damping":8.0,
            "thrust_sensitivity":40.0,"roll_sensitivity":6.0,"linear_stop":0.05,"angular_stop":0.05}"#;
        let i: InertiaSettings = serde_json::from_str(old).expect("old inertia file loads");
        assert_eq!(i.look_sensitivity, 0.01);
        assert_eq!(
            InertiaSettings {
                look_sensitivity: 0.005,
                ..i
            },
            InertiaSettings::default()
        );
        let old = r#"{"orbit_sensitivity":0.02,"orbit_release_inertia_scale":1.0,"inertia_friction":0.8}"#;
        let n: CameraNavigationSettings =
            serde_json::from_str(old).expect("old navigation file loads");
        assert_eq!(n.orbit_sensitivity, 0.02);
        assert_eq!(n.coast, CoastSettings::default());
    }

    /// A coast written to JSON reads back equal, and a partial `coast` object fills the rest
    /// from the coast defaults.
    #[cfg(feature = "serde")]
    #[test]
    fn coast_settings_serde_round_trip() {
        let i = InertiaSettings {
            coast: CoastSettings {
                time: 1.25,
                softness: 5.0,
                min_speed: 0.1,
            },
            ..InertiaSettings::default()
        };
        let json = serde_json::to_string(&i).expect("serialise");
        assert_eq!(
            serde_json::from_str::<InertiaSettings>(&json).expect("round trip"),
            i
        );
        let partial: InertiaSettings =
            serde_json::from_str(r#"{"coast":{"time":2.0}}"#).expect("partial coast");
        assert_eq!(partial.coast.time, 2.0);
        assert_eq!(partial.coast.softness, CoastSettings::default().softness);
    }

    #[test]
    fn exponential_scroll_zoom_moves_distance() {
        let s = CameraNavigationSettings {
            zoom_mode: ZoomMode::Exponential,
            ..CameraNavigationSettings::default()
        };
        let f = s.scroll_zoom_factor(10.0);
        assert!((CameraNavigationSettings::ZOOM_CLAMP_MIN..1.0).contains(&f));
    }
}
