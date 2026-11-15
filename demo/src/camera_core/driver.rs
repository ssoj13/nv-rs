//! [`CameraDriver`]: THE one-call host API of camera-core. One frame of [`RawInput`] in, one
//! [`DriverFrame`] (pose, repaint hint, cursor request, host requests) out.
//!
//! What: owns the [`CameraController`], the [`Navigator`], the [`Bindings`], the user's
//! [`NavPrefs`], an optional [`LensEase`], the hold-to-fly state (`Flight`: orbiting, held,
//! releasing) and the host's view roll, and runs the documented per-frame protocol in the one
//! correct order, so no host glues these by hand.
//!
//! Why: hosts that wired the navigator, the controller, the preferences and the lens themselves
//! got the order wrong (prefs re-applied every frame, a fly drag applied to an orbit rig, a lens
//! flattening mid-snap, the orbit pivot lost after a flight). One driver, one protocol.
//!
//! How, per [`CameraDriver::update`]:
//! 1. sanitise the input; on a falling `keys_allowed` edge release everything (the navigator's
//!    release frame + `CameraController::release_inputs`, leaving a hold-to-fly);
//! 2. apply the preferences for the effective mode, ONLY when they changed (a cached key of the
//!    sanitised prefs + effective mode, scene radius, viewport height and live FOV, which the
//!    pan scale depends on); the navigator tuning is cached and rebuilt in `set_prefs`;
//! 3. `Navigator::update` (with the hold-fly tuning only while the fly drag is `Held`);
//! 4. hold-to-fly, a three-state flight (`Flight`):
//!    - `Releasing` and this frame carries input (a fly engage, any intent, a request): the
//!      coasts stop and the flight ends now (pivot restored); the frame continues on the orbit
//!      backend, where a new RMB press begins a fresh `Held` from the restored pivot;
//!    - `Orbiting` and a fly drag (`fly_engaged` while the prefs' mode is an orbit mode):
//!      switch to `NavPrefs::hold_fly_mode` BEFORE the intents are applied, remembering the
//!      orbit distance (`Held`);
//!    - `Held` and the drag ended: `Releasing` WITHOUT leaving the fly backend, so this frame's
//!      `LookInertia` and zeroed thrust reach it and the camera glides on;
//! 5. apply the intents (un-rolled by the host's view roll when it is not 0); pass `fly_align`
//!    to `CameraController::set_align` with `prefs.align_seconds`; `update_dynamics(dt)`;
//! 6. `Releasing` and no coast left: the flight ends: back to `prefs.mode`, eye and orientation
//!    from the flight, the pivot the remembered distance ahead of the eye (the very same pivot
//!    when the flight did not move; a still release restores on its release frame);
//! 7. lens (when set): only while the camera is at rest (so a Top snap lands first, then
//!    flattens), flat only when looking straight down (`lens::is_top_down` about the bindings'
//!    up axis), the target clamped to `CameraController::fov_range` (an unreachable target
//!    counts as reached), each step pushed through the FOV's one owner (a `DollyZoom` intent on
//!    the orbit backends, keeping the subject's size; the projection's fov on the fly backends);
//!    it waits during the whole flight (`Held` and `Releasing`);
//! 8. `wants_repaint` = the navigator's || the controller still moving || a lens ease that can
//!    progress.
//!
//! Host edits that must not fight a flight ([`CameraDriver::apply_intent`],
//! [`CameraDriver::set_pose`]) end it first; [`CameraDriver::stop`] freezes the coasts (a press
//! on the view), [`CameraDriver::release`] ends everything at once (focus loss).
//!
//! Uses: `crate::camera_core::controller`, `crate::camera_core::input`, `crate::camera_core::lens`, `crate::camera_core::prefs`. Used by: hosts
//! (through the `camera-egui` / `camera-winit` readers that build the [`RawInput`]).

use glam::Vec2;

use crate::camera_core::controller::CameraController;
use crate::camera_core::input::{
    Bindings, CursorRequest, NavRequests, NavScheme, NavTuning, Navigator, RawInput,
};
use crate::camera_core::intent::CameraIntent;
use crate::camera_core::lens::{LAND_EPS_RAD, LensEase, LensMode, is_top_down};
use crate::camera_core::mode::NavMode;
use crate::camera_core::pose::CameraPose;
use crate::camera_core::prefs::NavPrefs;
use crate::camera_core::projection::PerspectiveSettings;
use crate::camera_core::viewport::ViewportSize;

/// One frame of driver output.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DriverFrame {
    /// The camera pose to render with.
    pub pose: CameraPose,
    /// Something still moves (input, a coast, a transition, a lens ease): keep painting. An idle
    /// driver asks for nothing.
    pub wants_repaint: bool,
    /// What to do with the OS cursor (the adapter applies it).
    pub cursor: CursorRequest,
    /// Frame-selected / home commands the host resolves (it answers with
    /// `CameraIntent::FrameBounds` through [`CameraDriver::controller_mut`] or its home pose).
    pub requests: NavRequests,
}

/// A running hold-to-fly flight: what to restore when it ends.
#[derive(Debug, Clone, Copy, PartialEq)]
struct HoldFly {
    /// The orbit distance at the start of the flight (`None` if the controller was not an orbit
    /// rig then).
    distance: Option<f32>,
}

/// The hold-to-fly state machine (see the module docs, step 4).
#[derive(Debug, Clone, Copy, PartialEq)]
enum Flight {
    /// The preferences' orbit mode runs; no flight.
    Orbiting,
    /// The fly drag is held: the controller runs `NavPrefs::hold_fly_mode`.
    Held(HoldFly),
    /// The drag ended but the fly backend still coasts; the flight ends with the coast.
    Releasing(HoldFly),
}

impl Flight {
    /// The remembered pivot of a running flight (`Held` or `Releasing`), `None` while orbiting.
    fn hold(self) -> Option<HoldFly> {
        match self {
            Flight::Orbiting => None,
            Flight::Held(h) | Flight::Releasing(h) => Some(h),
        }
    }
}

/// `intent` un-rolled by the host's view `roll` (radians); a roll of exactly 0 is the identity.
fn roll_intent(intent: CameraIntent, roll: f32, vp: ViewportSize) -> CameraIntent {
    if roll == 0.0 {
        intent
    } else {
        intent.unrolled(roll, vp)
    }
}

/// What the controller was last configured with (`CameraController::apply_prefs`); a change
/// of any part re-applies. Floats compare by bits (the prefs are sanitised, so finite).
#[derive(Debug, Clone, Copy, PartialEq)]
struct Applied {
    prefs: NavPrefs,
    scene_radius: u32,
    viewport_height: u32,
    fov_y: u32,
}

/// The navigator tunings of `prefs`: the mode's, and the hold-to-fly mode's (fly input only
/// while the fly drag is held, so its persistent fly mode is off).
fn tunings(prefs: &NavPrefs) -> (NavTuning, NavTuning) {
    let tuning = NavTuning::from_prefs(prefs);
    let mut hold = NavTuning::from_prefs(&NavPrefs {
        mode: prefs.sanitized().hold_fly_mode,
        ..*prefs
    });
    hold.fly_mode = false;
    (tuning, hold)
}

/// The viewport size (whole units) of a sanitised viewport vector; at least 1 x 1.
fn viewport_size(viewport: Vec2) -> ViewportSize {
    ViewportSize::new(viewport.x.round() as u32, viewport.y.round() as u32)
}

/// THE one-call camera host API (see the module docs for the per-frame protocol).
#[derive(Debug, Clone)]
pub struct CameraDriver {
    controller: CameraController,
    navigator: Navigator,
    bindings: Bindings,
    prefs: NavPrefs,
    lens: Option<LensEase>,
    /// Navigator tuning of `prefs` (rebuilt in `set_prefs`).
    tuning: NavTuning,
    /// Navigator tuning while a hold-to-fly drag flies.
    hold_tuning: NavTuning,
    flight: Flight,
    /// The scheme the bindings were built from (`set_scheme`).
    scheme: NavScheme,
    /// The host overlay's view roll in radians (pushed with `set_view_roll`; 0 = none).
    view_roll: f32,
    applied: Option<Applied>,
    /// This frame's intents, copied out of the navigator (reused: no steady-state allocation).
    intents: Vec<CameraIntent>,
    /// The previous frame's typing guard, for the focus-loss edge.
    keys_allowed: bool,
    /// The last frame's scene radius and (sanitised) viewport, for `release`.
    scene_radius: f32,
    viewport: Vec2,
}

impl Default for CameraDriver {
    /// Default preferences, [`NavScheme::Default`], no lens.
    fn default() -> Self {
        Self::new(NavPrefs::default(), NavScheme::Default)
    }
}

impl CameraDriver {
    /// A driver at rest on the default controller; the preferences are applied on the first
    /// [`CameraDriver::update`].
    pub fn new(prefs: NavPrefs, scheme: NavScheme) -> Self {
        let (tuning, hold_tuning) = tunings(&prefs);
        Self {
            controller: CameraController::default(),
            navigator: Navigator::new(),
            bindings: scheme.bindings(),
            prefs,
            lens: None,
            tuning,
            hold_tuning,
            flight: Flight::Orbiting,
            scheme,
            view_roll: 0.0,
            applied: None,
            intents: Vec::new(),
            keys_allowed: false,
            scene_radius: 1.0,
            viewport: Vec2::ONE,
        }
    }

    /// One frame: input in, pose / repaint / cursor / requests out (the protocol is in the
    /// module docs). `scene_radius` scales the flight speed (`NavPrefs::inertia`).
    pub fn update(&mut self, raw: &RawInput, scene_radius: f32) -> DriverFrame {
        let raw = raw.sanitized();
        self.viewport = raw.viewport;
        self.scene_radius = scene_radius;
        let vp = viewport_size(raw.viewport);
        let mut wants_repaint = false;
        if self.keys_allowed && !raw.keys_allowed {
            wants_repaint |= self.release_in(vp);
        }
        self.keys_allowed = raw.keys_allowed;
        self.sync_prefs();

        // Only a HELD fly drag uses the hold tuning: while Releasing the orbit tuning reads input
        // again, so a press during the glide is an orbit press (it ends the flight below).
        let tuning = if matches!(self.flight, Flight::Held(_)) {
            &self.hold_tuning
        } else {
            &self.tuning
        };
        let frame = self.navigator.update(&raw, &self.bindings, tuning);
        self.intents.clear();
        self.intents.extend_from_slice(frame.intents);
        let nav_repaint = frame.wants_repaint;
        let (cursor, requests) = (frame.cursor, frame.requests);
        let (fly_engaged, fly_align) = (frame.fly_engaged, frame.fly_align);

        // Hold-to-fly (module docs, step 4). Input during a release glide ends the flight first
        // (pivot restored, the frame continues on the orbit backend).
        let hold_now = fly_engaged && !self.prefs.mode.is_fly();
        if matches!(self.flight, Flight::Releasing(_))
            && (fly_engaged || !self.intents.is_empty() || requests != NavRequests::default())
        {
            self.controller.stop_coasts();
            self.end_hold();
        }
        match self.flight {
            // Switch BEFORE this frame's intents (the orbit rigs ignore fly input).
            Flight::Orbiting if hold_now => {
                let distance = self.controller.orbit_pivot().map(|(_, d)| d);
                self.flight = Flight::Held(HoldFly { distance });
                self.sync_prefs();
            }
            // The drag ended: stay on the fly backend so this frame's LookInertia and zeroed
            // thrust reach it; the flight ends when the coast does (below).
            Flight::Held(hold) if !hold_now => self.flight = Flight::Releasing(hold),
            Flight::Orbiting | Flight::Held(_) | Flight::Releasing(_) => {}
        }

        let roll = self.view_roll;
        for intent in &self.intents {
            self.controller
                .apply_intent(roll_intent(*intent, roll, vp), raw.dt, vp);
        }
        let align_seconds = self.prefs.sanitized().align_seconds;
        self.controller
            .set_align(fly_align.map(|mode| (mode, align_seconds)));
        self.controller.update_dynamics(raw.dt);
        if matches!(self.flight, Flight::Releasing(_)) && !self.controller.is_coasting() {
            self.end_hold();
        }
        let lens_busy = self.step_lens(raw.dt, vp);

        wants_repaint |= nav_repaint || self.controller.has_motion() || lens_busy;
        DriverFrame {
            pose: self.controller.pose(),
            wants_repaint,
            cursor,
            requests,
        }
    }

    /// Focus loss (the window lost focus, a modal opened): end every drag and held axis, stop
    /// every coast, drop a held alignment / queued look, and leave a hold-to-fly flight at once
    /// (pivot restored). [`CameraDriver::update`] does this itself on a falling `keys_allowed`
    /// edge.
    pub fn release(&mut self) {
        let vp = viewport_size(self.viewport);
        self.release_in(vp);
    }

    /// Freeze every running coast (what a press on the view does; the navigator emits the same
    /// `CameraIntent::Stop` itself). A hold-to-fly release glide ends with its coast: the pivot is
    /// restored now. A fly drag still held is not touched (the user is still flying).
    pub fn stop(&mut self) {
        self.controller.stop_coasts();
        if matches!(self.flight, Flight::Releasing(_)) {
            self.end_hold();
        }
    }

    /// Apply a host intent now (see [`CameraDriver::controller_mut`] for why not directly): any
    /// hold-to-fly flight ends first (coasts stop, pivot restored) so the edit lands on the
    /// orbit backend, the intent is un-rolled by the view roll ([`CameraDriver::set_view_roll`])
    /// and applied with the last frame's viewport.
    pub fn apply_intent(&mut self, intent: CameraIntent) {
        self.end_flight();
        let vp = viewport_size(self.viewport);
        self.controller
            .apply_intent(roll_intent(intent, self.view_roll, vp), 0.0, vp);
    }

    /// Move the camera to `pose` now, keeping the backend and its tuning
    /// ([`CameraController::set_pose`]); any hold-to-fly flight ends first.
    pub fn set_pose(&mut self, pose: CameraPose) {
        self.end_flight();
        self.controller.set_pose(pose);
    }

    /// Push the host overlay's view roll (radians; a non-finite value is 0). While it is not 0,
    /// screen-space intents (pan, cursor zoom) are rotated back by it
    /// ([`CameraIntent::unrolled`]) so a drag follows the rotated screen axes. The host owns
    /// the roll; the driver holds a copy, like the scene radius.
    pub fn set_view_roll(&mut self, radians: f32) {
        self.view_roll = if radians.is_finite() { radians } else { 0.0 };
    }

    /// The view roll last pushed with [`CameraDriver::set_view_roll`] (radians).
    pub fn view_roll(&self) -> f32 {
        self.view_roll
    }

    /// Replace the preferences: the navigator tuning is rebuilt now, the controller is
    /// re-configured on the next [`CameraDriver::update`] (only if anything changed).
    pub fn set_prefs(&mut self, prefs: NavPrefs) {
        (self.tuning, self.hold_tuning) = tunings(&prefs);
        self.prefs = prefs;
    }

    /// The preferences.
    pub fn prefs(&self) -> &NavPrefs {
        &self.prefs
    }

    /// Switch the navigation table to a scheme preset (edits of the previous table are lost).
    pub fn set_scheme(&mut self, scheme: NavScheme) {
        self.bindings = scheme.bindings();
        self.scheme = scheme;
    }

    /// The scheme the navigation table was last built from (edits through
    /// [`CameraDriver::bindings_mut`] do not change it).
    pub fn scheme(&self) -> NavScheme {
        self.scheme
    }

    /// The navigation table.
    pub fn bindings(&self) -> &Bindings {
        &self.bindings
    }

    /// The navigation table, editable (rebind a row; takes effect on the next frame).
    pub fn bindings_mut(&mut self) -> &mut Bindings {
        &mut self.bindings
    }

    /// The camera controller.
    pub fn controller(&self) -> &CameraController {
        &self.controller
    }

    /// The camera controller, for host edits the driver cannot make (answer
    /// [`DriverFrame::requests`] with `CameraIntent::FrameBounds`, restore a saved pose, push
    /// clips with the live FOV kept). Change the mode and the tuning through
    /// [`CameraDriver::set_prefs`], never here: the driver would not know.
    pub fn controller_mut(&mut self) -> &mut CameraController {
        &mut self.controller
    }

    /// The navigator.
    pub fn navigator(&self) -> &Navigator {
        &self.navigator
    }

    /// The lens ease, if the host enabled one.
    pub fn lens(&self) -> Option<&LensEase> {
        self.lens.as_ref()
    }

    /// The lens ease, editable (switch `LensMode`, retarget, change the ease time).
    pub fn lens_mut(&mut self) -> Option<&mut LensEase> {
        self.lens.as_mut()
    }

    /// Enable (`Some`) or disable (`None`) the lens ease.
    pub fn set_lens(&mut self, lens: Option<LensEase>) {
        self.lens = lens;
    }

    /// True while a hold-to-fly drag is held (the controller runs `NavPrefs::hold_fly_mode` and
    /// the fly input is live). The release glide after it is [`CameraDriver::is_release_coasting`].
    pub fn is_hold_flying(&self) -> bool {
        matches!(self.flight, Flight::Held(_))
    }

    /// True while a hold-to-fly flight glides on after its drag ended (the controller still runs
    /// the fly backend; the flight ends with the coast or at the next input).
    pub fn is_release_coasting(&self) -> bool {
        matches!(self.flight, Flight::Releasing(_))
    }

    /// True while the controller has a release coast running ([`CameraController::is_coasting`]).
    pub fn is_coasting(&self) -> bool {
        self.controller.is_coasting()
    }

    /// The mode the controller should run now: the hold-to-fly mode during a flight (held or
    /// releasing), else the preferences' mode.
    fn effective_mode(&self) -> NavMode {
        if self.flight.hold().is_some() {
            self.prefs.sanitized().hold_fly_mode
        } else {
            self.prefs.mode
        }
    }

    /// Configure the controller for the effective mode when the preferences, the mode, the scene
    /// radius, the viewport height or the live FOV changed since the last time (never
    /// unconditionally: `apply_prefs` is idempotent, but the work is skipped).
    fn sync_prefs(&mut self) {
        let prefs = NavPrefs {
            mode: self.effective_mode(),
            ..self.prefs.sanitized()
        };
        let key = Applied {
            prefs,
            scene_radius: self.scene_radius.to_bits(),
            viewport_height: self.viewport.y.to_bits(),
            fov_y: self.controller.projection().fov_y.to_bits(),
        };
        if self.applied != Some(key) {
            self.controller
                .apply_prefs(&prefs, self.scene_radius, self.viewport.y);
            self.applied = Some(key);
        }
    }

    /// End a hold-to-fly flight (held or releasing): back to the preferences' mode (eye and
    /// orientation from the flight), the pivot the remembered distance ahead of the eye. A no-op
    /// while orbiting.
    fn end_hold(&mut self) {
        let flight = std::mem::replace(&mut self.flight, Flight::Orbiting);
        if let Some(hold) = flight.hold() {
            self.sync_prefs();
            if let Some(distance) = hold.distance {
                self.controller.set_orbit_distance(distance);
            }
        }
    }

    /// End a running flight now, coasts included (a host edit must not land on the fly backend
    /// or be undone by the pivot restore later). A no-op while orbiting: orbit coasts are the
    /// edit's own business.
    fn end_flight(&mut self) {
        if self.flight.hold().is_some() {
            self.controller.stop_coasts();
            self.end_hold();
        }
    }

    /// The release protocol: the navigator's release frame, `release_inputs`, every coast
    /// stopped, the end of a hold-to-fly flight at once. Returns whether the release emitted
    /// anything.
    fn release_in(&mut self, vp: ViewportSize) -> bool {
        let frame = self.navigator.release();
        let emitted = frame.wants_repaint;
        self.intents.clear();
        self.intents.extend_from_slice(frame.intents);
        let roll = self.view_roll;
        for intent in &self.intents {
            self.controller
                .apply_intent(roll_intent(*intent, roll, vp), 0.0, vp);
        }
        self.controller.release_inputs();
        self.controller.stop_coasts();
        self.end_hold();
        emitted
    }

    /// One lens step (see the module docs, step 6). Returns whether the ease can still
    /// progress (a flat ease held by a tilted view does not keep the host painting).
    fn step_lens(&mut self, dt: f32, vp: ViewportSize) -> bool {
        // A hold-to-fly is a temporary backend: its wider FOV reach would let a reach-clamped
        // lens target ease as a plain zoom (no pivot, no subject-size invariant) and the release
        // would then not restore the view. The lens waits and resumes on the orbit after the
        // hold (the release glide included).
        if self.flight.hold().is_some() {
            return false;
        }
        let Some(lens) = &mut self.lens else {
            return false;
        };
        let up = self.bindings.up_axis.up();
        let top_down = is_top_down(self.controller.pose().forward(), up);
        if !self.controller.has_motion() {
            let (min, max) = self.controller.fov_range();
            lens.set_reach(min, max);
            let current = self.controller.projection().fov_y;
            let next = lens.step_gated(dt, current, top_down);
            // A sub-epsilon step is the rounding of the previous push: not a change.
            if (next - current).abs() > LAND_EPS_RAD {
                if self.controller.is_fly() {
                    let projection = PerspectiveSettings {
                        fov_y: next,
                        ..self.controller.projection()
                    };
                    self.controller.set_projection(projection);
                } else {
                    let dfov = next - current;
                    self.controller
                        .apply_intent(CameraIntent::DollyZoom { dfov }, dt, vp);
                }
            }
        }
        lens.is_easing() && (lens.mode() == LensMode::Perspective || top_down)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera_core::input::{Button, CursorRequest, Key};
    use crate::camera_core::view::{Face, UpAxis, ViewTarget};
    use glam::Vec3;

    const DT: f32 = 1.0 / 60.0;

    /// A frame with the pointer resting over an 800 x 600 view, keys allowed, nothing pressed.
    fn idle() -> RawInput {
        RawInput {
            pointer: Some(Vec2::new(400.0, 300.0)),
            hovered: true,
            keys_allowed: true,
            dt: DT,
            viewport: Vec2::new(800.0, 600.0),
            ..RawInput::default()
        }
    }

    fn pivot(driver: &CameraDriver) -> (Vec3, f32) {
        let Some(pivot) = driver.controller().orbit_pivot() else {
            panic!("expected an orbit rig, got {}", driver.controller().kind());
        };
        pivot
    }

    /// Unreal: an RMB hold from the turntable flies (FPS) on every frame it is held, and its
    /// release returns to the turntable on the very same pivot.
    #[test]
    fn unreal_hold_to_fly_keeps_flying_and_restores_the_pivot() {
        let mut driver = CameraDriver::new(NavPrefs::default(), NavScheme::Unreal);
        driver.update(&idle(), 10.0);
        let (target, distance) = pivot(&driver);

        let mut press = idle();
        press.buttons_pressed.insert(Button::Secondary);
        press.buttons_down.insert(Button::Secondary);
        driver.update(&press, 10.0);
        assert!(driver.is_hold_flying());
        assert_eq!(driver.controller().mode(), Some(NavMode::FpsFly));

        let mut held = idle();
        held.buttons_down.insert(Button::Secondary);
        for frame in 0..30 {
            driver.update(&held, 10.0);
            assert!(driver.is_hold_flying(), "frame {frame}");
            assert_eq!(
                driver.controller().mode(),
                Some(NavMode::FpsFly),
                "frame {frame}"
            );
        }

        let mut up = idle();
        up.buttons_released.insert(Button::Secondary);
        driver.update(&up, 10.0);
        assert!(!driver.is_hold_flying());
        assert_eq!(driver.controller().mode(), Some(NavMode::Turntable));
        let (t2, d2) = pivot(&driver);
        assert!(t2.distance(target) < 1e-3, "pivot {t2} vs {target}");
        assert!((d2 - distance).abs() < 1e-4, "distance {d2} vs {distance}");
    }

    /// With a lens set, an idle RMB hold-to-fly (press, hold still, release) leaves the view and
    /// the pivot exactly as they were: the lens never eases during the temporary fly backend.
    #[test]
    fn an_idle_hold_with_a_lens_restores_the_view() {
        let mut driver = CameraDriver::new(NavPrefs::default(), NavScheme::Unreal);
        driver.set_lens(Some(LensEase::new(LensMode::Perspective)));
        let rest = RawInput {
            dt: DT,
            viewport: Vec2::new(800.0, 600.0),
            ..RawInput::default()
        };
        driver.update(&rest, 10.0);
        if let Some(lens) = driver.lens_mut() {
            lens.set_mode(LensMode::Flat);
        }
        let top = CameraIntent::View(ViewTarget::face(Face::Top, UpAxis::Y));
        driver
            .controller_mut()
            .apply_intent(top, DT, ViewportSize::new(800, 600));
        let mut frames = 0;
        while driver.update(&rest, 10.0).wants_repaint {
            frames += 1;
            assert!(frames < 3_000, "the driver never settled");
        }
        let before = driver.controller().pose();
        let (target, distance) = pivot(&driver);

        let mut press = rest;
        press.buttons_pressed.insert(Button::Secondary);
        press.buttons_down.insert(Button::Secondary);
        driver.update(&press, 10.0);
        let mut held = rest;
        held.buttons_down.insert(Button::Secondary);
        for _ in 0..30 {
            driver.update(&held, 10.0);
        }
        let fov_during = driver.controller().projection().fov_y;
        assert!(
            (fov_during - before.fov_y).abs() < 1e-6,
            "the lens eased during the hold"
        );
        let mut up = rest;
        up.buttons_released.insert(Button::Secondary);
        driver.update(&up, 10.0);
        for _ in 0..10 {
            driver.update(&rest, 10.0);
        }
        let after = driver.controller().pose();
        let (t2, d2) = pivot(&driver);
        assert!((after.fov_y - before.fov_y).abs() < 1e-5, "fov changed");
        assert!(after.eye.distance(before.eye) < 1e-3, "eye moved");
        assert!(t2.distance(target) < 1e-3, "pivot {t2} vs {target}");
        assert!((d2 - distance).abs() < 1e-3, "distance {d2} vs {distance}");
    }

    /// An idle driver asks for no repaint, never moves the camera and never rebuilds the
    /// controller (a marker on the rig survives 100 frames); a prefs change does switch it.
    #[test]
    fn idle_frames_are_idempotent() {
        let mut driver = CameraDriver::default();
        let rest = RawInput {
            dt: DT,
            viewport: Vec2::new(800.0, 600.0),
            ..RawInput::default()
        };
        let first = driver.update(&rest, 10.0);
        assert!(!first.wants_repaint, "an idle driver does not paint");
        let CameraController::Houdini(b) = driver.controller_mut() else {
            panic!("the default mode runs on the Houdini orbit");
        };
        b.rig.pitch_limit = 0.5;
        for frame in 0..100 {
            let out = driver.update(&rest, 10.0);
            assert!(!out.wants_repaint, "frame {frame}");
            assert_eq!(out.pose, first.pose, "frame {frame}: the camera moved");
        }
        let CameraController::Houdini(b) = driver.controller() else {
            panic!("the controller was replaced");
        };
        assert_eq!(b.rig.pitch_limit, 0.5, "the controller was rebuilt");

        driver.set_prefs(NavPrefs {
            mode: NavMode::Trackball,
            ..NavPrefs::default()
        });
        let out = driver.update(&rest, 10.0);
        assert_eq!(driver.controller().mode(), Some(NavMode::Trackball));
        assert!(
            out.pose.eye.distance(first.pose.eye) < 1e-3,
            "the view was kept"
        );
    }

    /// An Unreal driver 20 frames into an RMB fly drag with a steady mouse push (both the
    /// relative delta and a pointer offset, so either steering turns the view); returns it with
    /// the orbit distance remembered at the press.
    fn flying_driver() -> (CameraDriver, f32) {
        let mut driver = CameraDriver::new(NavPrefs::default(), NavScheme::Unreal);
        driver.update(&idle(), 10.0);
        let (_, distance) = pivot(&driver);
        let mut press = idle();
        press.buttons_pressed.insert(Button::Secondary);
        press.buttons_down.insert(Button::Secondary);
        driver.update(&press, 10.0);
        for _ in 0..20 {
            driver.update(&drag_frame(), 10.0);
        }
        assert!(driver.is_hold_flying());
        (driver, distance)
    }

    /// One frame of the steady fly-drag push (RMB down).
    fn drag_frame() -> RawInput {
        let mut raw = idle();
        raw.buttons_down.insert(Button::Secondary);
        raw.pointer = Some(Vec2::new(500.0, 300.0));
        raw.pointer_delta = Vec2::new(6.0, 0.0);
        raw
    }

    /// The frame the fly drag ends on, the push still going.
    fn release_frame() -> RawInput {
        let mut raw = idle();
        raw.buttons_released.insert(Button::Secondary);
        raw.pointer = Some(Vec2::new(500.0, 300.0));
        raw
    }

    /// The flight is over: orbit mode, and the pivot sits `distance` ahead of the eye.
    fn assert_restored(driver: &CameraDriver, distance: f32) {
        assert!(!driver.is_hold_flying());
        assert!(!driver.is_release_coasting());
        assert_eq!(driver.controller().mode(), Some(NavMode::Turntable));
        let pose = driver.controller().pose();
        let (target, d) = pivot(driver);
        assert!((d - distance).abs() < 1e-3, "distance {d} vs {distance}");
        assert!(
            target.distance(pose.eye + pose.forward() * distance) < 1e-3,
            "pivot {target} is not {distance} ahead of the eye"
        );
    }

    /// A fly drag that ends moving glides on (the fly backend keeps running, the cursor is
    /// free, the driver keeps asking for frames), then restores the pivot at eye + forward *
    /// the remembered distance within the coast time (+1 frame).
    #[test]
    fn fly_release_coasts_then_restores_the_pivot() {
        let (mut driver, distance) = flying_driver();
        let out = driver.update(&release_frame(), 10.0);
        assert!(
            driver.is_release_coasting(),
            "the glide starts on the release frame"
        );
        assert!(!driver.is_hold_flying());
        assert!(driver.is_coasting());
        assert!(out.wants_repaint);
        assert_eq!(out.cursor, CursorRequest::Default);
        assert_eq!(driver.controller().mode(), Some(NavMode::FpsFly));

        let limit = (driver.prefs().coast.time / DT).ceil() as usize + 1;
        let mut frames = 0;
        while driver.is_release_coasting() {
            driver.update(&idle(), 10.0);
            frames += 1;
            assert!(frames <= limit, "the glide outlived {limit} frames");
        }
        assert!(
            driver
                .controller()
                .pose()
                .orientation
                .angle_between(out.pose.orientation)
                > 1e-4,
            "the view kept turning during the glide"
        );
        assert_restored(&driver, distance);
        assert!(!driver.update(&idle(), 10.0).wants_repaint, "settled");
    }

    /// Escape on a moving fly drag cancels it: no glide, the pivot is restored on that frame and
    /// the camera stays where the Escape found it.
    #[test]
    fn escape_on_a_fly_drag_restores_without_a_glide() {
        let (mut driver, distance) = flying_driver();
        let mut esc = drag_frame();
        esc.pointer_delta = Vec2::ZERO;
        esc.keys_pressed = esc.keys_pressed.with(Key::Escape);
        let out = driver.update(&esc, 10.0);
        assert_restored(&driver, distance);
        assert!(!driver.is_coasting());
        let still = driver.update(&idle(), 10.0);
        assert_eq!(still.pose.eye, out.pose.eye, "no glide after Escape");
    }

    /// A press on the view (LMB) during the glide freezes it and restores the pivot on that very
    /// frame; the camera stays where the press found it.
    #[test]
    fn lmb_press_mid_coast_restores_the_same_frame() {
        let (mut driver, distance) = flying_driver();
        driver.update(&release_frame(), 10.0);
        for _ in 0..3 {
            driver.update(&idle(), 10.0);
        }
        assert!(driver.is_release_coasting());
        let mut press = idle();
        press.buttons_pressed.insert(Button::Primary);
        press.buttons_down.insert(Button::Primary);
        let out = driver.update(&press, 10.0);
        assert_restored(&driver, distance);
        assert!(!driver.is_coasting(), "the press froze the coast");
        let frozen = driver.update(&idle(), 10.0);
        assert!(
            frozen.pose.eye.distance(out.pose.eye) < 1e-5,
            "moved after the press"
        );
    }

    /// An RMB press during the glide ends it and begins a new hold-fly from the restored pivot:
    /// a still hold then restores the ORIGINAL distance on its release frame.
    #[test]
    fn rmb_press_mid_coast_begins_a_new_hold_with_the_original_distance() {
        let (mut driver, distance) = flying_driver();
        driver.update(&release_frame(), 10.0);
        for _ in 0..3 {
            driver.update(&idle(), 10.0);
        }
        assert!(driver.is_release_coasting());
        let mut press = idle();
        press.buttons_pressed.insert(Button::Secondary);
        press.buttons_down.insert(Button::Secondary);
        driver.update(&press, 10.0);
        assert!(driver.is_hold_flying(), "a new hold began");
        assert!(!driver.is_release_coasting());
        let mut held = idle();
        held.buttons_down.insert(Button::Secondary);
        for _ in 0..2 {
            driver.update(&held, 10.0);
        }
        let mut up = idle();
        up.buttons_released.insert(Button::Secondary);
        driver.update(&up, 10.0);
        assert_restored(&driver, distance);
    }

    /// A wheel notch or the frame key during the glide restores the pivot first: the wheel zoom
    /// lands on the orbit rig, the frame request reaches the host with the pivot restored.
    #[test]
    fn wheel_and_frame_key_during_release_restore_first() {
        let (mut wheel_driver, distance) = flying_driver();
        wheel_driver.update(&release_frame(), 10.0);
        assert!(wheel_driver.is_release_coasting());
        let mut wheel = idle();
        wheel.wheel.lines = Vec2::new(0.0, 1.0);
        wheel_driver.update(&wheel, 10.0);
        assert!(!wheel_driver.is_release_coasting());
        assert_eq!(wheel_driver.controller().mode(), Some(NavMode::Turntable));
        let (_, zoomed) = pivot(&wheel_driver);
        assert!(
            (zoomed - distance).abs() > 1e-4,
            "the wheel zoomed the restored orbit"
        );

        let (mut key_driver, distance) = flying_driver();
        key_driver.update(&release_frame(), 10.0);
        assert!(key_driver.is_release_coasting());
        let mut frame_key = idle();
        frame_key.keys_pressed.insert(Key::F);
        frame_key.keys_down.insert(Key::F);
        let out = key_driver.update(&frame_key, 10.0);
        assert!(out.requests.frame_selected, "the request reaches the host");
        assert_restored(&key_driver, distance);
    }

    /// `release()` (focus loss) and `stop()` end the glide at once, pivot restored; `stop` also
    /// leaves a still-held drag alone.
    #[test]
    fn release_and_stop_end_the_glide_at_once() {
        for use_release in [true, false] {
            let (mut driver, distance) = flying_driver();
            driver.update(&release_frame(), 10.0);
            assert!(driver.is_release_coasting());
            if use_release {
                driver.release();
            } else {
                driver.stop();
            }
            assert!(!driver.is_coasting());
            assert_restored(&driver, distance);
        }
        let (mut held, _) = flying_driver();
        held.stop();
        assert!(held.is_hold_flying(), "a held drag is still flying");
    }

    /// Host edits end a flight first: `set_pose` and `apply_intent` never land on the fly backend.
    #[test]
    fn host_edits_end_the_flight_first() {
        let (mut driver, distance) = flying_driver();
        driver.update(&release_frame(), 10.0);
        assert!(driver.is_release_coasting());
        let pose = driver.controller().pose();
        driver.set_pose(pose);
        assert_restored(&driver, distance);

        let (mut driver, _) = flying_driver();
        driver.apply_intent(CameraIntent::Zoom { factor: 0.9 });
        assert!(!driver.is_hold_flying());
        assert_eq!(driver.controller().mode(), Some(NavMode::Turntable));
    }

    /// The host's view roll rotates screen-space intents back: with a quarter-turn roll a
    /// rightward pan acts as the downward pan of the unrolled view; roll 0 (never set, set to 0,
    /// or a non-finite value) is bit-identical to no roll at all.
    #[test]
    fn set_view_roll_rotates_pan_and_zero_is_bit_identical() {
        let settled = || {
            let mut driver = CameraDriver::default();
            driver.update(&idle(), 10.0);
            driver
        };
        let pan_right = CameraIntent::Pan {
            dx_px: 10.0,
            dy_px: 0.0,
        };
        let pan_down = CameraIntent::Pan {
            dx_px: 0.0,
            dy_px: -10.0,
        };

        let mut rolled = settled();
        rolled.set_view_roll(std::f32::consts::FRAC_PI_2);
        assert_eq!(rolled.view_roll(), std::f32::consts::FRAC_PI_2);
        rolled.apply_intent(pan_right);
        let mut reference = settled();
        reference.apply_intent(pan_down);
        let (a, b) = (rolled.controller().pose(), reference.controller().pose());
        assert!(
            a.eye.distance(b.eye) < 1e-4,
            "roll pan {:?} vs {:?}",
            a.eye,
            b.eye
        );

        let mut plain = settled();
        plain.apply_intent(pan_right);
        assert_ne!(
            plain.controller().pose(),
            a,
            "the roll changed the pan direction"
        );

        let mut zero = settled();
        zero.set_view_roll(0.0);
        zero.apply_intent(pan_right);
        assert_eq!(zero.controller().pose(), plain.controller().pose());
        let mut nan = settled();
        nan.set_view_roll(f32::NAN);
        assert_eq!(nan.view_roll(), 0.0);
        nan.apply_intent(pan_right);
        assert_eq!(nan.controller().pose(), plain.controller().pose());
    }

    /// The scheme is stored at `new` and `set_scheme`.
    #[test]
    fn scheme_is_stored() {
        let mut driver = CameraDriver::new(NavPrefs::default(), NavScheme::Unreal);
        assert_eq!(driver.scheme(), NavScheme::Unreal);
        driver.set_scheme(NavScheme::Default);
        assert_eq!(driver.scheme(), NavScheme::Default);
    }

    /// At rest `update` allocates nothing: the intent buffer never grows over 1000 frames (the
    /// core tests carry no allocation counter, so this is the capacity check; the appbus crate's
    /// allocation-counter test covers the rest).
    #[test]
    fn update_at_rest_does_not_grow_its_buffers() {
        let mut driver = CameraDriver::default();
        let rest = RawInput {
            dt: DT,
            viewport: Vec2::new(800.0, 600.0),
            ..RawInput::default()
        };
        for _ in 0..5 {
            driver.update(&rest, 10.0);
        }
        let capacity = driver.intents.capacity();
        for frame in 0..1000 {
            let out = driver.update(&rest, 10.0);
            assert!(!out.wants_repaint, "frame {frame}");
        }
        assert_eq!(driver.intents.capacity(), capacity);
    }

    /// A Top snap with a flat lens: the snap lands first (the FOV does not move while the
    /// camera moves), then the lens flattens to the narrowest reachable FOV (the distance range
    /// stops it above the 2 degree flat lens) and the driver settles in bounded frames.
    #[test]
    fn top_snap_then_flat_lands_exactly_and_settles() {
        let mut driver = CameraDriver::default();
        driver.set_lens(Some(LensEase::new(LensMode::Perspective)));
        let rest = RawInput {
            dt: DT,
            viewport: Vec2::new(800.0, 600.0),
            ..RawInput::default()
        };
        driver.update(&rest, 10.0);
        if let Some(lens) = driver.lens_mut() {
            lens.set_mode(LensMode::Flat);
        }
        let top = CameraIntent::View(ViewTarget::face(Face::Top, UpAxis::Y));
        let vp = ViewportSize::new(800, 600);
        driver.controller_mut().apply_intent(top, DT, vp);

        let mut fov = driver.controller().projection().fov_y;
        let mut frames = 0;
        loop {
            let out = driver.update(&rest, 10.0);
            let now = driver.controller().projection().fov_y;
            if now != fov {
                assert!(
                    out.pose.forward().dot(-Vec3::Y) > 0.99999,
                    "frame {frames}: the lens moved before the snap landed"
                );
                fov = now;
            }
            if !out.wants_repaint {
                break;
            }
            frames += 1;
            assert!(frames < 3_000, "the driver never settled");
        }
        let pose = driver.controller().pose();
        assert!(
            pose.forward().dot(-Vec3::Y) > 0.99999,
            "landed exactly on Top"
        );
        let Some(lens) = driver.lens() else {
            panic!("the lens is set");
        };
        assert!(
            !lens.is_easing(),
            "the unreachable flat target counts as reached"
        );
        let (min, _) = driver.controller().fov_range();
        assert!(
            min > crate::camera_core::lens::FOV_FLAT,
            "the distance range limits the flatten"
        );
        assert!(
            (fov - min).abs() < 1e-4,
            "flattened to the reachable minimum: {fov} vs {min}"
        );
    }
}
