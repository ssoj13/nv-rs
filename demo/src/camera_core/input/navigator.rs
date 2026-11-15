//! [`Navigator`]: the per-viewport navigation state machine, [`RawInput`] + [`Bindings`] +
//! [`NavTuning`] -> [`NavFrame`] (camera intents, repaint hint, cursor request, fly state, host
//! requests).
//!
//! What: drag roles latched at the press (orbit / pan / dolly / dolly-zoom / hold-to-fly), the
//! four release-velocity estimators ([`ReleaseVelocity`]: orbit, pan, dolly, dolly-zoom) that
//! seed the rigs' coasts, the orbit snap modes, wheel and pinch zoom, the frame / home / view /
//! orbit-step keys and the fly input ([`super::fly`]). [`NavTuning`] holds only the input-side
//! knobs (the rigs never read them: intents carry radians, factors and rates), built from
//! [`NavPrefs`] or from the settings structs.
//!
//! Why: the one implementation of the navigation rules of sl-viewport `camera_controls`,
//! `cam-controls-egui` (orbit + fly) and the nodes-rs gesture owners; every host feeds it a
//! [`RawInput`] and applies the intents to its `CameraController`.
//!
//! How: time-free (the host passes `dt` in [`RawInput`]); non-finite input is dropped
//! ([`RawInput::sanitized`]); the intent buffer is reused, so a steady-state `update` does not
//! allocate (the estimators are fixed rings). Every drag frame, zero motion included, is pushed
//! into its estimator, so a pause of `RELEASE_WINDOW_S` or more before the release leaves a zero
//! velocity and no `*Inertia` intent, while a shorter pause only dilutes the rate (both in
//! seconds, so the seed does not depend on the frame rate). A still RELEASE frame is not a
//! pause and is not pushed. The release rates are always emitted when non-zero: whether and how
//! long to coast is the rig's decision (its `CoastSettings`). A press over the view emits
//! `CameraIntent::Stop` first (the user grabs the view). The wheel zooms by
//! ONE curve, `exp(-notches * NavTuning::wheel_log_rate)`, over notch equivalents
//! ([`super::Wheel::notches`]); a frame applies at most a [`FACTOR_MAX`] step and carries the rest
//! to the next frames (spread, never dropped).
//!
//! Used by: hosts (through `camera-egui` / `camera-winit` readers), tests.

use glam::Vec2;

use super::bindings::{Bindings, DragRole};
use super::fly::{FlyState, push_release};
use super::raw::{Button, Key, Mods, POINTS_PER_NOTCH, RawInput};
use crate::camera_core::coast::{CoastValue, ReleaseVelocity};
use crate::camera_core::intent::{CameraIntent, OrbitSnapMode};
use crate::camera_core::prefs::NavPrefs;
use crate::camera_core::projection::PerspectiveSettings;
use crate::camera_core::settings::{CameraNavigationSettings, InertiaSettings, MouseSteer};
use crate::camera_core::{Face, ViewTarget};

/// One orbit-step key turn (radians): 15 degrees.
pub const ORBIT_STEP_RAD: f32 = std::f32::consts::PI / 12.0;
/// Field-of-view change (radians) per viewport unit of horizontal dolly-zoom drag (drag right
/// widens).
pub const DOLLY_ZOOM_RAD_PER_UNIT: f32 = 0.004;
/// Largest distance factor one frame applies (its inverse is the smallest): a larger wheel step
/// is spread over the next frames, a larger pinch is clamped (only a hostile host sends one).
pub const FACTOR_MAX: f32 = 100.0;
/// Most queued wheel notches: only an absurd host value (a 1e30 step) exceeds it.
const WHEEL_BACKLOG_MAX_NOTCHES: f32 = 1000.0;
/// The scene radius `NavTuning::from_prefs` passes to `NavPrefs::inertia`: it only scales the
/// thrust, which the navigator does not read.
const ANY_SCENE_RADIUS: f32 = 1.0;
/// The viewport height `NavTuning::from_prefs` passes to `NavPrefs::navigation`: it only scales
/// the pan, which the navigator does not read.
const ANY_VIEWPORT_HEIGHT: f32 = 1.0;

/// What the host should do with the OS cursor this frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CursorRequest {
    /// Visible and free.
    #[default]
    Default,
    /// Hidden, never grabbed or confined (deflection steering with `steer_capture_cursor`; a
    /// confine froze pointer input in a host backend).
    Hidden,
    /// Hidden and locked in place (relative mouse look): the adapter feeds raw device motion into
    /// `RawInput::pointer_delta`.
    Locked,
}

/// Commands the host resolves itself: the core does not know the selection or the home pose.
/// (Deviation from PLAN §2 "`CameraIntent::{FrameSelected, Home}`": both need host-owned data,
/// so they are requests, not intents.)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NavRequests {
    /// Frame the selection (`Bindings::frame_key`): the host emits
    /// `CameraIntent::FrameBounds` of its selection.
    pub frame_selected: bool,
    /// Go home (`Bindings::home_keys`): frame everything / restore the home view.
    pub home: bool,
}

/// The `-ln` of a distance factor for one input unit: the log rate of the one exponential curve.
/// `0` (no zoom) for a degenerate factor.
fn log_rate(factor_per_unit: f32) -> f32 {
    let k = -factor_per_unit.ln();
    if k.is_finite() && k > 0.0 { k } else { 0.0 }
}

/// The input-side knobs of the navigator. The rigs never read these (intents carry radians,
/// factors and rates), so nothing here mirrors a backend setting. Build with
/// [`NavTuning::from_prefs`] (the user preferences) or [`NavTuning::new`] (the settings structs).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavTuning {
    /// Persistent fly mode (the selected `NavMode` is a fly mode): fly input is live without
    /// holding a `DragRole::Fly` button, and orbit-style drags do not start. `false`: fly input
    /// only while a fly drag is held (a host may keep a fly backend coasting after it).
    pub fly_mode: bool,
    /// Which look intent flying emits: `Look` deltas or `LookRate` deflection.
    pub steer: MouseSteer,
    /// Orbit radians per viewport unit of drag.
    pub orbit_rad_per_unit: f32,
    /// Invert the horizontal orbit-drag axis.
    pub invert_orbit_x: bool,
    /// Invert the vertical orbit-drag axis.
    pub invert_orbit_y: bool,
    /// Wheel zoom log rate per notch: the distance scales by `exp(-notches * rate)`.
    pub wheel_log_rate: f32,
    /// Dolly log rate per viewport unit of downward drag: `exp(-dy * rate)`.
    pub dolly_log_rate: f32,
    /// The wheel zooms toward the cursor instead of the target.
    pub zoom_to_cursor: bool,
    /// Relative mouse-look radians per viewport unit.
    pub look_rad_per_unit: f32,
    /// Invert the horizontal look axis.
    pub invert_look_x: bool,
    /// Invert the vertical look axis.
    pub invert_look_y: bool,
    /// Deflection dead zone, a fraction of the half viewport.
    pub steer_deadzone: f32,
    /// Persistent fly modes anchor the deflection at the viewport centre (`false`: at the cursor
    /// when flight engages). A fly drag always anchors at its press point.
    pub steer_center_screen: bool,
    /// Relative anchors drift toward the cursor with this time constant (seconds); `None` = the
    /// deflection persists.
    pub steer_decay_secs: Option<f32>,
    /// Hide the cursor while deflection-steering.
    pub steer_capture_cursor: bool,
    /// Speed factor while Shift (fast) is held.
    pub fast_multiplier: f32,
    /// Speed factor while Alt (slow) is held.
    pub slow_multiplier: f32,
}

impl NavTuning {
    /// The input-side knobs of these settings. The log rates sample the settings' own zoom
    /// mappings at one input unit (`scroll_zoom_factor(1)`, `dolly_zoom_factor(1)`, so the
    /// `ZoomMode` and its gains are honoured; a rate above the settings' per-unit cap saturates
    /// there) and then run as one uncapped exponential curve.
    pub fn new(nav: &CameraNavigationSettings, inertia: &InertiaSettings, fly_mode: bool) -> Self {
        Self {
            fly_mode,
            steer: inertia.mouse_steer,
            orbit_rad_per_unit: nav.orbit_sensitivity,
            invert_orbit_x: nav.invert_orbit_x,
            invert_orbit_y: nav.invert_orbit_y,
            wheel_log_rate: log_rate(nav.scroll_zoom_factor(1.0)) * POINTS_PER_NOTCH,
            dolly_log_rate: log_rate(nav.dolly_zoom_factor(1.0)),
            zoom_to_cursor: nav.zoom_to_cursor,
            look_rad_per_unit: inertia.look_sensitivity,
            invert_look_x: inertia.invert_look_x,
            invert_look_y: inertia.invert_look_y,
            steer_deadzone: inertia.steer_deadzone,
            steer_center_screen: inertia.steer_center_screen,
            steer_decay_secs: inertia.steer_decay.then_some(inertia.steer_decay_secs),
            steer_capture_cursor: inertia.steer_capture_cursor,
            fast_multiplier: inertia.speed_scale(true, false),
            slow_multiplier: inertia.speed_scale(false, true),
        }
    }

    /// The navigator knobs of the user preferences, through the preferences' own mappers
    /// (`NavPrefs::navigation` / `inertia`): steering from `prefs.mode.mouse_steer()`, fly mode
    /// from `prefs.mode.is_fly()`, look / orbit sensitivity from `look_sensitivity`. The
    /// coast (and so the feel) is the rigs' business: the navigator always emits the release
    /// rates.
    pub fn from_prefs(prefs: &NavPrefs) -> Self {
        let nav = prefs.navigation(PerspectiveSettings::default().fov_y, ANY_VIEWPORT_HEIGHT);
        let mut inertia = prefs.inertia(ANY_SCENE_RADIUS);
        inertia.mouse_steer = prefs.mode.mouse_steer();
        Self::new(&nav, &inertia, prefs.mode.is_fly())
    }

    /// The thrust scale of the held speed modifiers (both held multiply, as
    /// `InertiaSettings::speed_scale`).
    pub fn speed_scale(&self, fast: bool, slow: bool) -> f32 {
        let f = if fast { self.fast_multiplier } else { 1.0 };
        let s = if slow { self.slow_multiplier } else { 1.0 };
        f * s
    }
}

impl Default for NavTuning {
    /// The default navigation and inertia settings, no persistent fly mode.
    fn default() -> Self {
        Self::new(
            &CameraNavigationSettings::default(),
            &InertiaSettings::default(),
            false,
        )
    }
}

/// One frame's navigation output.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavFrame<'a> {
    /// Intents for `CameraController::apply_intent`, in order.
    pub intents: &'a [CameraIntent],
    /// Real input this frame (a drag, a turn, pointer motion while steering, a held axis, a
    /// speed change, the fly engage / disengage edge, a capture change, a wheel step or queued
    /// zoom, a key command): keep painting. An idle frame (also an idle flying frame, which
    /// still sends the hold intents) asks for nothing; the host ORs in the controller's
    /// `has_motion` for the camera's own coast.
    pub wants_repaint: bool,
    /// What to do with the OS cursor.
    pub cursor: CursorRequest,
    /// Fly input is live. On a rising edge of a fly DRAG the host switches to a fly backend
    /// BEFORE applying this frame's intents (the orbit rigs ignore look / thrust).
    pub fly_engaged: bool,
    /// While flying, the held world alignment (physical Ctrl: `OrbitSnapMode::Smart`): the host
    /// passes it to `CameraController::set_align` with its time constant (`None` releases it).
    /// Only `SpaceFlight` aligns.
    pub fly_align: Option<OrbitSnapMode>,
    /// Frame / home commands for the host.
    pub requests: NavRequests,
}

/// The drag in progress, latched at its press.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ActiveDrag {
    button: Button,
    role: DragRole,
}

/// The release-velocity estimators of the drag in progress: each is fed the drag's per-frame step
/// (zero steps included: they are the pause) and answers the per-second velocity at the release.
#[derive(Clone, Copy, Debug, Default)]
struct Release {
    /// Orbit turn (radians: yaw, pitch).
    orbit: ReleaseVelocity<Vec2>,
    /// Pan (viewport units).
    pan: ReleaseVelocity<Vec2>,
    /// Dolly (`ln` of the distance factor).
    dolly: ReleaseVelocity<f32>,
    /// Dolly-zoom (field-of-view radians).
    dolly_zoom: ReleaseVelocity<f32>,
}

impl Release {
    /// Forget every sample (a new drag, a cancelled one).
    fn clear(&mut self) {
        self.orbit.clear();
        self.pan.clear();
        self.dolly.clear();
        self.dolly_zoom.clear();
    }
}

/// The release rate as a coast seed: `None` for a zero (the user paused) or non-finite velocity.
fn release_seed<V: CoastValue>(v: V) -> Option<V> {
    (v != V::ZERO && v.is_finite()).then_some(v)
}

/// The orbit snap of the held modifiers (camera-navigation exception: physical Ctrl snaps).
fn snap_mode(mods: Mods) -> Option<OrbitSnapMode> {
    match (mods.ctrl, mods.shift) {
        (true, true) => Some(OrbitSnapMode::Axis),
        (false, true) => Some(OrbitSnapMode::Plane),
        (true, false) => Some(OrbitSnapMode::Smart),
        (false, false) => None,
    }
}

/// Per-viewport navigation state. Create once per viewport and call [`Navigator::update`] every
/// frame.
#[derive(Clone, Debug, Default)]
pub struct Navigator {
    /// The reused intent buffer.
    intents: Vec<CameraIntent>,
    drag: Option<ActiveDrag>,
    release: Release,
    /// Active orbit constraint; a change begins a new snap gesture from the current view.
    snapping: Option<OrbitSnapMode>,
    fly: FlyState,
    /// Wheel notches accepted but not yet applied (a step larger than one frame's bound).
    wheel_backlog: f32,
    /// The pointer was over the view last frame (re-entry resumes a suspended fly capture).
    hovered_last: bool,
}

impl Navigator {
    /// A navigator at rest.
    pub fn new() -> Navigator {
        Navigator::default()
    }

    /// The role of the drag in progress, if any.
    pub fn active_role(&self) -> Option<DragRole> {
        self.drag.map(|d| d.role)
    }

    /// The wheel flight-speed multiplier (1.0 until the wheel changes it while flying).
    pub fn fly_speed(&self) -> f32 {
        self.fly.speed
    }

    /// Translate one frame of input. See the `input` module docs for the scheme table.
    pub fn update(
        &mut self,
        raw: &RawInput,
        bindings: &Bindings,
        tuning: &NavTuning,
    ) -> NavFrame<'_> {
        let raw = raw.sanitized();
        self.intents.clear();
        let mut requests = NavRequests::default();

        // A press over the view grabs it: every running coast stops before anything else of
        // this frame (a drag that starts now seeds a fresh one at its release).
        if raw.hovered && !raw.buttons_pressed.is_empty() {
            self.intents.push(CameraIntent::Stop);
        }

        // Esc cancels the drag in progress (no coast); a persistent fly mode drops orbit drags.
        let escape = raw.keys_pressed.contains(Key::Escape);
        let mut fly_cancelled = false;
        if let Some(drag) = self.drag
            && (escape || (tuning.fly_mode && drag.role != DragRole::Fly))
        {
            fly_cancelled = drag.role == DragRole::Fly;
            self.cancel_drag();
        }

        // A drag in progress: this frame's motion, then its release.
        let had_drag = self.drag.is_some();
        let was_held = self.fly.held;
        let mut fly_drag_ended = false;
        if let Some(drag) = self.drag {
            let down = raw.buttons_down.contains(drag.button);
            // A still release frame is not a pause: pushing its zero motion would dilute the
            // seed by one frame, which depends on the frame rate.
            if down || raw.pointer_delta != Vec2::ZERO {
                self.drag_motion(drag.role, &raw, bindings, tuning);
            }
            if !down {
                fly_drag_ended = self.finish_drag(drag.role);
            }
        }

        // A new drag starts on a press over the view (motion begins next frame).
        let fly_pressed =
            self.drag.is_none() && raw.hovered && self.start_drag(&raw, bindings, tuning);

        // Persistent capture: Escape suspends it (the pointer may then leave for the UI); the
        // pointer re-entering the view or a click on it resumes it.
        let mut capture_changed = false;
        let mut resumed = false;
        if tuning.fly_mode {
            let entered = raw.hovered && !self.hovered_last;
            let clicked = raw.hovered && !raw.buttons_pressed.is_empty();
            if escape {
                capture_changed = !self.fly.suspended;
                self.fly.suspended = true;
            } else if self.fly.suspended && (entered || clicked) {
                self.fly.suspended = false;
                capture_changed = true;
                resumed = true;
            }
        }

        // Fly input (a held fly drag or the persistent fly mode).
        let engaged = self.fly.held || tuning.fly_mode;
        let edge = engaged != self.fly.active;
        // Mouse look and the cursor capture: a held fly drag keeps them while held; the
        // persistent mode only over the view, with the typing guard, and not suspended.
        let steering = self.fly.held
            || (tuning.fly_mode && raw.hovered && raw.keys_allowed && !self.fly.suspended);
        let mut fly_moved = false;
        let mut cursor = CursorRequest::Default;
        let mut fly_align = None;
        if engaged {
            if !self.fly.active || fly_pressed || resumed {
                self.fly.anchor_at(&raw, tuning);
            } else if was_held && !self.fly.held {
                self.fly.pin_at_cursor(&raw);
            }
            self.wheel_backlog = 0.0;
            if bindings.fly_wheel_speed && raw.hovered {
                self.fly.wheel_speed(&raw);
            }
            fly_moved = self.fly.push_frame(
                &raw,
                &bindings.fly_keys,
                tuning,
                steering,
                &mut self.intents,
            );
            if steering {
                cursor = match tuning.steer {
                    MouseSteer::Relative => CursorRequest::Locked,
                    MouseSteer::Deflection if tuning.steer_capture_cursor => CursorRequest::Hidden,
                    MouseSteer::Deflection => CursorRequest::Default,
                };
            }
            fly_align = (raw.keys_allowed && raw.mods.ctrl).then_some(OrbitSnapMode::Smart);
        } else {
            if self.fly.active {
                // The fly drag's mouse-look velocity coasts (the rig decides) before the zero
                // set, which would otherwise be the only thing it sees.
                if fly_drag_ended {
                    self.fly.push_look_inertia(&mut self.intents);
                }
                self.fly.clear_look();
                push_release(&mut self.intents);
                if fly_cancelled {
                    // Esc on a fly drag: no glide. `Stop` after the zero set drops the released
                    // thrust before the rig's step would turn it into a coast.
                    self.intents.push(CameraIntent::Stop);
                }
                self.fly.last_scale = None;
            }
            self.fly.suspended = false;
            if bindings.wheel_zoom {
                self.wheel_zoom(&raw, bindings, tuning);
            } else {
                self.wheel_backlog = 0.0;
            }
            if raw.keys_allowed {
                self.keys(&raw, bindings, &mut requests);
            }
        }
        self.fly.active = engaged;
        self.hovered_last = raw.hovered;

        // Repaint only on real input; an idle engaged frame still sends the hold intents but asks
        // for nothing (the camera's own coast is the controller's `has_motion`).
        NavFrame {
            wants_repaint: had_drag
                || self.drag.is_some()
                || edge
                || capture_changed
                || fly_moved
                || self.wheel_backlog != 0.0
                || (!engaged && !self.intents.is_empty())
                || requests != NavRequests::default(),
            intents: &self.intents,
            cursor,
            fly_engaged: engaged,
            fly_align,
            requests,
        }
    }

    /// Focus loss (the window lost focus, a modal opened, the viewport went away): end the drag
    /// without a coast, drop queued wheel zoom and the fly state. Returns a frame whose intents
    /// release the rig's held axes (apply them like a frame's), with no alignment
    /// (`fly_align: None`), a free cursor and no fly. A persistent fly capture stays suspended
    /// until the pointer re-enters or clicks the view. The host ALSO calls
    /// `CameraController::release_inputs`, which drops what intents cannot (a held alignment, a
    /// queued look).
    pub fn release(&mut self) -> NavFrame<'_> {
        self.intents.clear();
        self.cancel_drag();
        self.fly.held = false;
        self.wheel_backlog = 0.0;
        if self.fly.active {
            push_release(&mut self.intents);
        }
        self.fly.clear_look();
        self.fly.active = false;
        self.fly.last_scale = None;
        self.fly.suspended = true;
        self.hovered_last = false;
        NavFrame {
            wants_repaint: !self.intents.is_empty(),
            intents: &self.intents,
            cursor: CursorRequest::Default,
            fly_engaged: false,
            fly_align: None,
            requests: NavRequests::default(),
        }
    }

    /// End the drag without a coast.
    fn cancel_drag(&mut self) {
        if self.drag.is_some_and(|d| d.role == DragRole::Fly) {
            self.fly.held = false;
        }
        self.drag = None;
        self.release.clear();
        self.snapping = None;
    }

    /// Start the first pressed button's drag that has a role (resolution order [`Button::ALL`]).
    /// Returns whether a fly drag started.
    fn start_drag(&mut self, raw: &RawInput, bindings: &Bindings, tuning: &NavTuning) -> bool {
        let started = Button::ALL
            .into_iter()
            .filter(|&b| raw.buttons_pressed.contains(b) && raw.buttons_down.contains(b))
            .find_map(|b| {
                bindings
                    .role(b, raw.mods, raw.keys_down, raw.keys_allowed)
                    .map(|role| ActiveDrag { button: b, role })
            })
            .filter(|d| !tuning.fly_mode || d.role == DragRole::Fly);
        let Some(drag) = started else {
            return false;
        };
        self.drag = Some(drag);
        self.release.clear();
        self.snapping = None;
        let fly = drag.role == DragRole::Fly;
        if fly {
            self.fly.held = true;
        }
        fly
    }

    /// One frame of pointer motion for the drag's role.
    fn drag_motion(&mut self, role: DragRole, raw: &RawInput, bindings: &Bindings, t: &NavTuning) {
        let d = raw.pointer_delta;
        let r = &mut self.release;
        match role {
            DragRole::Orbit => {
                let sx = if t.invert_orbit_x { -1.0 } else { 1.0 };
                let sy = if t.invert_orbit_y { -1.0 } else { 1.0 };
                // Base signs "grab the world" like pan; the invert options flip an axis.
                let turn = Vec2::new(
                    -d.x * t.orbit_rad_per_unit * sx,
                    d.y * t.orbit_rad_per_unit * sy,
                );
                let mode = if bindings.orbit_snap {
                    snap_mode(raw.mods)
                } else {
                    None
                };
                if let Some(mode) = mode {
                    // A snap-modified orbit seeds no coast (whatever the backend: the navigator
                    // cannot know it, so a trackball's own momentum is not seeded either).
                    r.orbit.clear();
                    // Only a moving frame emits (and so starts) a snap gesture: a bare modifier
                    // change would send a zero-delta OrbitSnap, which stops coasts for nothing.
                    if turn != Vec2::ZERO {
                        let start = self.snapping.replace(mode) != Some(mode);
                        self.intents.push(CameraIntent::OrbitSnap {
                            yaw_delta: turn.x,
                            pitch_delta: turn.y,
                            start,
                            mode,
                        });
                    }
                } else {
                    if turn != Vec2::ZERO {
                        self.snapping = None;
                    }
                    r.orbit.push(turn, raw.dt);
                    if turn != Vec2::ZERO {
                        self.intents.push(CameraIntent::Orbit {
                            yaw_delta: turn.x,
                            pitch_delta: turn.y,
                        });
                    }
                }
            }
            DragRole::Pan => {
                r.pan.push(d, raw.dt);
                if d != Vec2::ZERO {
                    self.intents.push(CameraIntent::Pan {
                        dx_px: d.x,
                        dy_px: d.y,
                    });
                }
            }
            DragRole::Dolly => {
                // The same exponential curve as the wheel: frames compose exactly, no cap.
                let log = -d.y * t.dolly_log_rate;
                r.dolly.push(log, raw.dt);
                let factor = log.exp();
                if factor != 1.0 && factor.is_finite() {
                    self.intents.push(CameraIntent::Zoom {
                        factor: factor.clamp(1.0 / FACTOR_MAX, FACTOR_MAX),
                    });
                }
            }
            DragRole::DollyZoom => {
                let dfov = d.x * DOLLY_ZOOM_RAD_PER_UNIT;
                r.dolly_zoom.push(dfov, raw.dt);
                if dfov != 0.0 {
                    self.intents.push(CameraIntent::DollyZoom { dfov });
                }
            }
            // Mouse look of a held flight is the fly frame's (`FlyState::push_frame`).
            DragRole::Fly => {}
        }
    }

    /// The drag's button went up: emit the release rates (the rig decides whether they coast),
    /// end the drag. Returns whether it was a fly drag (the caller emits its `LookInertia` once
    /// the flight state is settled).
    fn finish_drag(&mut self, role: DragRole) -> bool {
        let r = &self.release;
        match role {
            DragRole::Orbit if self.snapping.is_none() => {
                if let Some(v) = release_seed(r.orbit.velocity()) {
                    self.intents.push(CameraIntent::OrbitInertia {
                        yaw_rate: v.x,
                        pitch_rate: v.y,
                    });
                }
            }
            DragRole::Pan => {
                if let Some(v) = release_seed(r.pan.velocity()) {
                    self.intents.push(CameraIntent::PanInertia {
                        dx_rate: v.x,
                        dy_rate: v.y,
                    });
                }
            }
            DragRole::Dolly => {
                if let Some(rate) = release_seed(r.dolly.velocity()) {
                    self.intents.push(CameraIntent::ZoomInertia { rate });
                }
            }
            DragRole::DollyZoom => {
                if let Some(fov_rate) = release_seed(r.dolly_zoom.velocity()) {
                    self.intents
                        .push(CameraIntent::DollyZoomInertia { fov_rate });
                }
            }
            DragRole::Orbit | DragRole::Fly => {}
        }
        self.cancel_drag();
        role == DragRole::Fly
    }

    /// Wheel / smooth scroll / pinch -> one zoom (or a trackpad pan). New wheel input is taken
    /// only over the view; queued notches drain wherever the pointer is.
    fn wheel_zoom(&mut self, raw: &RawInput, bindings: &Bindings, t: &NavTuning) {
        let w = raw.wheel;
        if raw.hovered {
            let notches = if bindings.smooth_scroll_pans {
                if w.points != Vec2::ZERO {
                    self.intents.push(CameraIntent::Pan {
                        dx_px: w.points.x,
                        dy_px: w.points.y,
                    });
                }
                w.lines.y
            } else {
                w.notches()
            };
            self.wheel_backlog = (self.wheel_backlog + notches)
                .clamp(-WHEEL_BACKLOG_MAX_NOTCHES, WHEEL_BACKLOG_MAX_NOTCHES);
        }
        let k = t.wheel_log_rate;
        let mut log = 0.0;
        if k > 0.0 && k.is_finite() {
            // At most a FACTOR_MAX step this frame; the rest waits for the next frames.
            let bound = FACTOR_MAX.ln() / k;
            let n = self.wheel_backlog.clamp(-bound, bound);
            self.wheel_backlog -= n;
            log = -n * k;
        } else {
            self.wheel_backlog = 0.0;
        }
        // A pinch > 1 magnifies: the distance shrinks by its inverse.
        let pinch = if raw.hovered { raw.pinch } else { 1.0 };
        let factor = (log.exp() / pinch).clamp(1.0 / FACTOR_MAX, FACTOR_MAX);
        if factor == 1.0 {
            return;
        }
        let cursor = raw.pointer.filter(|_| t.zoom_to_cursor);
        self.intents.push(match cursor {
            Some(p) => CameraIntent::ZoomAtCursor {
                cursor_x: p.x,
                cursor_y: p.y,
                factor,
            },
            None => CameraIntent::Zoom { factor },
        });
    }

    /// Frame / home / view / orbit-step keys (the caller checked the typing guard). Plain keys
    /// only; the view keys also take the platform command modifier for the opposite face.
    fn keys(&mut self, raw: &RawInput, bindings: &Bindings, requests: &mut NavRequests) {
        let pressed = |k: Key| raw.keys_pressed.contains(k);
        let m = raw.mods;
        let plain = m == Mods::NONE;
        // Command alone: on Windows / Linux the adapters report Ctrl as both `ctrl` and `command`.
        let command = m.command && !m.alt && !m.shift;
        if plain && bindings.frame_key.is_some_and(pressed) {
            requests.frame_selected = true;
        }
        if plain && bindings.home_keys.iter().flatten().any(|&k| pressed(k)) {
            requests.home = true;
        }
        if let Some(v) = bindings.view_keys {
            for (key, face) in [
                (v.front, Face::Front),
                (v.right, Face::Right),
                (v.top, Face::Top),
            ] {
                if !pressed(key) {
                    continue;
                }
                let face = if plain {
                    Some(face)
                } else if command {
                    Some(face.opposite())
                } else {
                    None
                };
                if let Some(face) = face {
                    self.intents
                        .push(CameraIntent::View(ViewTarget::face(face, bindings.up_axis)));
                }
            }
        }
        if plain && let Some(s) = bindings.orbit_step_keys {
            // Each step turns like a drag that way: drag right = negative yaw, drag down =
            // positive pitch (see `drag_motion`).
            for (key, yaw, pitch) in [
                (s.left, ORBIT_STEP_RAD, 0.0),
                (s.right, -ORBIT_STEP_RAD, 0.0),
                (s.up, 0.0, -ORBIT_STEP_RAD),
                (s.down, 0.0, ORBIT_STEP_RAD),
            ] {
                if pressed(key) {
                    self.intents.push(CameraIntent::Orbit {
                        yaw_delta: yaw,
                        pitch_delta: pitch,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::camera_core::input::raw::ButtonSet;

    /// A frame over the view with the primary button held (or just released) and `delta` motion.
    fn frame(down: bool, pressed: bool, delta: Vec2, dt: f32) -> RawInput {
        let held = if down {
            ButtonSet::EMPTY.with(Button::Primary)
        } else {
            ButtonSet::EMPTY
        };
        RawInput {
            buttons_down: held,
            buttons_pressed: if pressed { held } else { ButtonSet::EMPTY },
            pointer_delta: delta,
            hovered: true,
            keys_allowed: true,
            dt,
            viewport: Vec2::new(800.0, 600.0),
            ..RawInput::default()
        }
    }

    #[test]
    fn release_seed_drops_zero_and_non_finite_velocities() {
        assert_eq!(release_seed(Vec2::ZERO), None);
        assert_eq!(release_seed(0.0_f32), None);
        assert_eq!(release_seed(Vec2::new(f32::NAN, 1.0)), None);
        assert_eq!(release_seed(f32::INFINITY), None);
        assert_eq!(
            release_seed(Vec2::new(0.0, -2.0)),
            Some(Vec2::new(0.0, -2.0))
        );
    }

    #[test]
    fn a_still_release_frame_is_not_a_pause() {
        // The release frame carries no motion: its `dt` must not dilute the seed, whatever it is.
        let seed = |release_dt: f32| {
            let (b, t) = (Bindings::default(), NavTuning::default());
            let mut nav = Navigator::new();
            nav.update(&frame(true, true, Vec2::ZERO, 0.016), &b, &t);
            for _ in 0..20 {
                nav.update(&frame(true, false, Vec2::new(5.0, 0.0), 0.016), &b, &t);
            }
            nav.update(&frame(false, false, Vec2::ZERO, release_dt), &b, &t)
                .intents
                .iter()
                .find_map(|i| match *i {
                    CameraIntent::OrbitInertia { yaw_rate, .. } => Some(yaw_rate),
                    _ => None,
                })
        };
        let quick = seed(0.001).expect("a moving release seeds the coast");
        assert_eq!(seed(0.5), Some(quick));
    }

    #[test]
    fn snap_modes_follow_the_camera_modifier_exception() {
        assert_eq!(snap_mode(Mods::NONE), None);
        assert_eq!(snap_mode(Mods::SHIFT), Some(OrbitSnapMode::Plane));
        assert_eq!(snap_mode(Mods::CTRL), Some(OrbitSnapMode::Smart));
        assert_eq!(
            snap_mode(Mods::CTRL.plus(Mods::SHIFT)),
            Some(OrbitSnapMode::Axis)
        );
        // The platform command key alone (Cmd on macOS) never snaps.
        assert_eq!(snap_mode(Mods::COMMAND), None);
    }

    #[test]
    fn log_rates_follow_the_settings_zoom_mapping() {
        use crate::camera_core::settings::ZoomMode;
        let exp = CameraNavigationSettings {
            zoom_mode: ZoomMode::Exponential,
            ..CameraNavigationSettings::default()
        };
        let t = NavTuning::new(&exp, &InertiaSettings::default(), false);
        let per_point = exp.zoom_scroll_exponential;
        assert!((t.wheel_log_rate - per_point * POINTS_PER_NOTCH).abs() < 1e-5);
        let lin = CameraNavigationSettings::default();
        let t = NavTuning::new(&lin, &InertiaSettings::default(), false);
        assert!(
            (t.dolly_log_rate + (1.0 - lin.dolly_sensitivity).ln()).abs() < 1e-6,
            "linear: one unit dollies by 1 - sensitivity"
        );
        assert_eq!(log_rate(1.0), 0.0);
        assert_eq!(log_rate(f32::NAN), 0.0);
        assert_eq!(log_rate(2.0), 0.0, "an inverted mapping zooms nothing");
    }

    #[test]
    fn steady_state_update_does_not_grow_the_buffer() {
        let mut nav = Navigator::new();
        let bindings = Bindings::default();
        let tuning = NavTuning {
            fly_mode: true,
            ..NavTuning::default()
        };
        let raw = RawInput {
            keys_allowed: true,
            hovered: true,
            dt: 1.0 / 60.0,
            viewport: Vec2::new(800.0, 600.0),
            ..RawInput::default()
        };
        nav.update(&raw, &bindings, &tuning);
        let cap = nav.intents.capacity();
        for _ in 0..100 {
            nav.update(&raw, &bindings, &tuning);
        }
        assert_eq!(nav.intents.capacity(), cap);
    }
}
