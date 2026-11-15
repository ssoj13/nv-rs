//! Host-agnostic input layer: one frame of raw input ([`RawInput`]) + a navigation table
//! ([`Bindings`], from a [`NavScheme`] preset) + the input-side knobs ([`NavTuning`]) -> camera
//! intents ([`Navigator::update`] -> [`NavFrame`]).
//!
//! What: the single home of the camera-navigation rules (which button / modifier does what,
//! release coast, orbit snap, wheel zoom, view keys, RMB hold-to-fly, fly keys) for every host.
//!
//! Why: these rules were written four times (sl-viewport `camera_controls`, `cam-controls-egui`,
//! nodes-egui and nodes-editor gestures). Adapters (`camera-egui`, `camera-winit`) now only READ
//! input into a [`RawInput`] and apply the [`CursorRequest`]; the rules live here, free of egui,
//! winit and any clock (the host passes `dt`).
//!
//! How (per frame): hosts use `crate::camera_core::CameraDriver::update(&raw, scene_radius)` — it runs this
//! navigator, switches to the hold-to-fly backend and back (restoring the pivot), applies the
//! intents, the fly alignment and the dynamics, eases the lens, and reports one `wants_repaint`.
//! The host only honours the cursor request, answers `requests` (frame selected / home) and
//! paints while asked. Driving [`Navigator::update`] by hand is the low-level path the driver is
//! built on (see its docs for the full ordering).
//!
//! Deviation from PLAN §2 ("`CameraIntent::{FrameBounds, FrameSelected, Home}`"): frame
//! selected and home are [`NavFrame::requests`] ([`NavRequests`]), not intents, because the
//! selection bounds and the home pose are host-owned; the host answers with
//! `CameraIntent::FrameBounds` / its home view.
//!
//! ## Schemes
//!
//! | Scheme | Orbit | Pan | Dolly | Dolly-zoom | Hold to fly | Home | Digit keys |
//! |---|---|---|---|---|---|---|---|
//! | `Default` | LMB | MMB | RMB | Ctrl+Alt+LMB | - | Home | 1/3/7 views, 4/6/8/2 steps |
//! | `Maya` | Alt+LMB | Alt+MMB | Alt+RMB | - | - | Home, A | - |
//! | `Houdini` | Alt or Space + LMB | Alt or Space + MMB | Alt or Space + RMB | - | - | Home, H | - |
//! | `Blender` | MMB | Shift+MMB | Ctrl/Cmd+MMB | - | - | Home | 1/3/7 views, 4/6/8/2 steps |
//! | `Unreal` | Alt+LMB | MMB (also Alt+MMB) | Alt+RMB | Ctrl+Alt+LMB | RMB | Home | - |
//!
//! Ctrl+Alt is AltGr on some keyboard layouts: where the dolly-zoom row collides, the host
//! rebinds it ([`Bindings::drags`]).
//!
//! Every scheme:
//! - **Drags** start on a press over the view (first matching [`DragBinding`] row; modifiers
//!   released mid-drag do not change the role) and end on that button's release anywhere. Esc
//!   cancels a drag without a coast.
//! - **Orbit snap** (camera-navigation modifier exception), live during an orbit drag: Shift =
//!   nearest plane, physical Ctrl = smart (plane, or axis within 15 degrees), Ctrl+Shift = nearest
//!   axis. A snap-modified orbit seeds no coast, whatever the backend (the navigator cannot know
//!   it, so a trackball's momentum is not seeded either). Only moving frames send `OrbitSnap`
//!   (a bare modifier change sends nothing).
//! - **Release coast:** an orbit / pan / dolly / dolly-zoom release emits `OrbitInertia` /
//!   `PanInertia` / `ZoomInertia` (log-distance rate) / `DollyZoomInertia` with the per-second
//!   release velocity of the last `RELEASE_WINDOW_S` of the drag (`coast::ReleaseVelocity`;
//!   zero-motion frames count, so a pause of the whole window leaves no intent at all). The
//!   navigator always emits the rates; the rig's `CoastSettings` decide whether and how long
//!   they coast. A fly drag ending in a `Relative` steering mode emits `LookInertia` the same
//!   way (a persistent `Relative` mode: after `fly::LOOK_PAUSE_S` of still steering frames).
//! - **Press:** a press of any button over the view emits `CameraIntent::Stop` first (every
//!   coast stops), before the drag it may start.
//! - **Wheel** (hovered, not flying): ONE curve, the distance scales by
//!   `exp(-notches * NavTuning::wheel_log_rate)` (the settings' `ZoomMode` rate); lines and
//!   smooth points are both notch equivalents ([`Wheel::notches`], [`POINTS_PER_NOTCH`]). A step
//!   beyond one frame's [`FACTOR_MAX`] is spread over the next frames, never dropped. Pinch
//!   divides the distance; toward the cursor with `zoom_to_cursor`; `smooth_scroll_pans` pans
//!   with the points instead. The dolly drag uses the same exponential curve.
//! - **Keys** (typing guard `RawInput::keys_allowed`, not flying, no modifier): F frame selected,
//!   Home (+ the scheme key) home, view keys (Ctrl/Cmd = opposite face), orbit-step keys
//!   ([`ORBIT_STEP_RAD`], like a drag that way).
//! - **Flying** (a hold-to-fly drag, or `NavTuning::fly_mode`): W/S forward / back, A/D strafe,
//!   R/C (and Space) up / down, Q/E roll; Shift fast / Alt slow times the wheel speed
//!   (`CameraIntent::SpeedScale`); wheel = flight speed; mouse look by `NavTuning::steer` (the
//!   mode's steering: `Look` deltas, or `LookRate` deflection from the steer anchor; a fly drag
//!   anchors at its press point, so a press never turns the camera). Physical Ctrl = align
//!   ([`NavFrame::fly_align`]), which only `SpaceFlight` honours. Thrust, roll, look rate AND the
//!   speed scale are sent every flying frame (a rebuilt backend gets them back at once) and
//!   zeroed once when flight ends. In the persistent fly mode mouse look and the cursor capture
//!   work only while the pointer is over the view (with the typing guard); Escape suspends the
//!   capture (free cursor, no look) until the pointer re-enters or clicks the view, and so does
//!   [`Navigator::release`]. A held fly drag keeps its capture while held. A fly drag ending
//!   inside a persistent deflection mode leaves the steer anchor at the cursor (no jump to the
//!   centre).
//! - **Repaint:** `NavFrame::wants_repaint` reports real input only (a drag, a turn, pointer
//!   motion while steering, a held axis, a speed change, the fly engage / disengage edge, a
//!   capture change, a wheel step or queued zoom, a key command). An idle flying frame still
//!   sends the hold intents but asks for no repaint; the camera's own coast is the controller's
//!   `has_motion`, which the host ORs in.
//!
//! Used by: hosts and the adapters; re-exported by the `camera` umbrella.

mod bindings;
mod fly;
mod navigator;
mod raw;

pub use bindings::{Bindings, DragBinding, DragRole, FlyKeys, NavScheme, OrbitStepKeys, ViewKeys};
pub use navigator::{
    CursorRequest, DOLLY_ZOOM_RAD_PER_UNIT, FACTOR_MAX, NavFrame, NavRequests, NavTuning,
    Navigator, ORBIT_STEP_RAD,
};
pub use raw::{
    Button, ButtonSet, Key, KeySet, Mods, POINTS_PER_NOTCH, RawInput, RawMotionUnits, Wheel,
};
