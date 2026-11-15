//! # camera-egui — egui input adapter for camera-core
//!
//! What: reads one egui frame of pointer / keyboard / wheel input over a viewport response into
//! camera-core's `RawInput` ([`read_input`]), applies the cursor requests the navigator makes
//! (hide / grab while flying, [`apply_cursor`]), and bundles both around
//! `CameraDriver::update` ([`drive`]). Nothing else: every navigation rule lives in
//! `crate::camera_core::input`.
//!
//! Why: one place where egui meets the camera; hosts (egui-widgets-rs, nodes-egui, apps) stop
//! re-implementing button / modifier / fly-key reading.
//!
//! How:
//! - **Read-only.** [`read_input`] never consumes or edits egui input. The one deliberate
//!   exception is [`consume_wheel`], a separate helper [`drive`] calls while the view is hovered:
//!   the camera owns the wheel over the view, and without it an enclosing `ScrollArea` (which
//!   reads `smooth_scroll_delta` after its children) would scroll the page while the camera zooms.
//! - **Keys by physical position.** egui's `keys_down` is logical (layout-dependent). The reader
//!   tracks `Event::Key::physical_key` (falling back to the logical key where the integration
//!   gives none, e.g. eframe web) in a small per-egui-viewport table kept in egui temp memory.
//!   Each entry remembers the logical key of its press and is dropped as soon as egui no longer
//!   holds that logical key, so a missed release (focus loss, a frame the view was not drawn)
//!   can never leave a fly key stuck: egui clears `keys_down` on `WindowFocused(false)`.
//! - **Wheel and pinch: one source per gesture.** Every raw `Event::MouseWheel` is counted,
//!   whatever its modifiers, unsmoothed: `Line` deltas go to `Wheel::lines`, `Point` deltas to
//!   `Wheel::points`, `Page` deltas to `Wheel::lines` times [`PAGE_NOTCHES`] (a fixed notch count:
//!   egui's own page size, the window height, would be ~15 notches of zoom per page step). So a
//!   Ctrl / Cmd + wheel notch is a plain notch, as in `camera-winit`. `RawInput::pinch` is the
//!   product of the raw `Event::Zoom` factors of the frame (finite and > 0 only) and nothing
//!   else: egui's `zoom_delta()` is never read, because egui also folds the wheel into it
//!   (Ctrl+wheel, latched wheel modifiers across smoothing and trackpad gestures:
//!   `input_state/wheel_state.rs`, `input_state/mod.rs`), which would count the same input twice
//!   or lose it. The sign is egui's (= winit's): `+y` is the wheel turned away from the user.
//!   Shift / Alt do NOT redirect the axis here (egui's horizontal / vertical scroll modifiers
//!   apply only to its smoothed delta): camera navigation keeps its own modifier meaning.
//! - **Cursor ownership.** The OS cursor is one per egui viewport (window) but a window can hold
//!   several camera views (a quad view, dock tabs). [`apply_cursor`] therefore gives a non-default
//!   cursor an OWNER, the view's `Response::id`: while a view owns it, other views' requests are
//!   ignored (their idle `Default` must not undo a flight next door); only the owner changes it,
//!   and its own `Default` releases it. An owner that was not drawn this frame (its dock tab was
//!   switched away, the panel closed) loses the cursor at the end of the frame: a [`egui::Plugin`]
//!   registered on the first capture checks the owner's last-drawn frame
//!   (`Context::cumulative_frame_nr`) in `on_end_pass` and frees the cursor. The state lives in
//!   egui temp memory per viewport and is mutated in place (no per-frame allocation);
//!   `ViewportCommand`s go out only on a change. The `CursorIcon::None` set while the owner hides
//!   the cursor is per-frame egui output (egui resets the icon every frame), not a command.
//! - **Locked mouse look.** While THIS view owns a `Locked` cursor the pointer position no longer
//!   moves, so [`read_input`] takes `pointer_delta` from the raw device motion
//!   (`PointerState::motion` = winit `DeviceEvent::MouseMotion`, forwarded unscaled by
//!   egui-winit), converted by [`RawMotionUnits::to_points`], the same rule `camera-winit` uses.
//!   Units per backend (winit 0.30.13 sources; table in `crate::camera_core::input::RawMotionUnits`):
//!
//!   | Backend | Unit | [`RawMotionUnits`] | Conversion to egui points |
//!   |---|---|---|---|
//!   | Windows, X11 | device counts ~ physical pixels | `Physical` | divided by `pixels_per_point` |
//!   | macOS | Cocoa points | `Logical` | divided by egui's `zoom_factor` |
//!   | Wayland | `zwp_relative_pointer_v1`: same dimension as `wl_pointer` motion = surface-local logical | `Logical` | divided by egui's `zoom_factor` |
//!
//!   egui points = native points / `zoom_factor` and egui-winit's `pixels_per_point` = zoom times
//!   the native scale, hence the two divisors. The default is
//!   [`RawMotionUnits::PLATFORM_DEFAULT`] (macOS `Logical`, else `Physical`); Wayland and X11 are
//!   both Linux builds, so a Wayland host calls [`set_raw_motion_units`] with
//!   [`RawMotionUnits::Logical`] once (winit `ActiveEventLoopExtWayland::is_wayland`). The
//!   override is stored per egui viewport in temp memory.
//! - **Confined-only platforms (X11).** winit refuses `Locked` there (the cursor stays only
//!   confined), so the OS pointer still moves and can leave the view rect while its hidden cursor
//!   steers. While THIS view owns a `Locked` request [`read_input`] therefore reports
//!   `hovered = true` (while the window has focus) and treats the view as the keyboard target for
//!   `keys_allowed` (the other-widget text-field guard still applies), so the navigator does not
//!   lose the steer and the driver does not release the capture. On web eframe ignores
//!   `CursorGrab` / `CursorVisible` and sends no `MouseMoved`: `Locked` degrades to a hidden icon
//!   on a free cursor (the pointer delta stays egui's).
//!
//! Uses: egui 0.36, camera-core. Used by: the `camera` umbrella (feature `egui`).

use crate::camera_core::input::{
    Button, ButtonSet, CursorRequest, Key, KeySet, Mods, RawInput, RawMotionUnits, Wheel,
};
use crate::camera_core::{CameraDriver, DriverFrame};
use egui::{CursorGrab, CursorIcon, Event, Id, MouseWheelUnit, PointerButton, ViewportCommand};

/// Salt of the per-egui-viewport cursor state ([`apply_cursor`]).
const CURSOR_STATE_SALT: &str = "camera_egui::cursor";
/// Name of the end-of-pass plugin that frees a cursor whose owner was not drawn.
const CURSOR_RELEASE_NAME: &str = "camera_egui::CursorRelease";
/// Wheel notches one `MouseWheelUnit::Page` step counts as (a page-scrolling wheel setting).
pub const PAGE_NOTCHES: f32 = 3.0;
/// Salt of the per-egui-viewport raw motion unit ([`set_raw_motion_units`]).
const MOTION_UNITS_SALT: &str = "camera_egui::raw_motion_units";
/// Salt of the per-egui-viewport physical key table ([`read_input`]).
const HELD_KEYS_SALT: &str = "camera_egui::held_keys";
/// Physical keys tracked at once. Keyboards roll over at 6-10 keys; a press beyond the table is
/// ignored (never panics, never evicts a held key).
const HELD_MAX: usize = 16;

/// egui pointer buttons and their camera-core names.
const BUTTONS: [(PointerButton, Button); 5] = [
    (PointerButton::Primary, Button::Primary),
    (PointerButton::Secondary, Button::Secondary),
    (PointerButton::Middle, Button::Middle),
    (PointerButton::Extra1, Button::Back),
    (PointerButton::Extra2, Button::Forward),
];

/// An egui vector as camera-core's vector type (inferred at the use site, so this crate does
/// not depend on glam directly).
fn to_core<T: From<[f32; 2]>>(v: egui::Vec2) -> T {
    T::from([v.x, v.y])
}

/// The camera-core key of an egui key, `None` for keys camera navigation does not read. egui has
/// no separate numpad digits: integrations report them as `Num0..Num9`, which is what the
/// camera-core vocabulary wants. Public so hosts can check their own egui shortcuts against the
/// navigator's keys (e.g. a binding on a `FlyKeys` key) without a second mapping table.
pub fn key_of(key: egui::Key) -> Option<Key> {
    use egui::Key as E;
    Some(match key {
        E::A => Key::A,
        E::B => Key::B,
        E::C => Key::C,
        E::D => Key::D,
        E::E => Key::E,
        E::F => Key::F,
        E::G => Key::G,
        E::H => Key::H,
        E::I => Key::I,
        E::J => Key::J,
        E::K => Key::K,
        E::L => Key::L,
        E::M => Key::M,
        E::N => Key::N,
        E::O => Key::O,
        E::P => Key::P,
        E::Q => Key::Q,
        E::R => Key::R,
        E::S => Key::S,
        E::T => Key::T,
        E::U => Key::U,
        E::V => Key::V,
        E::W => Key::W,
        E::X => Key::X,
        E::Y => Key::Y,
        E::Z => Key::Z,
        E::Num0 => Key::Num0,
        E::Num1 => Key::Num1,
        E::Num2 => Key::Num2,
        E::Num3 => Key::Num3,
        E::Num4 => Key::Num4,
        E::Num5 => Key::Num5,
        E::Num6 => Key::Num6,
        E::Num7 => Key::Num7,
        E::Num8 => Key::Num8,
        E::Num9 => Key::Num9,
        E::Space => Key::Space,
        E::Home => Key::Home,
        E::Escape => Key::Escape,
        E::ArrowLeft => Key::ArrowLeft,
        E::ArrowRight => Key::ArrowRight,
        E::ArrowUp => Key::ArrowUp,
        E::ArrowDown => Key::ArrowDown,
        _ => return None,
    })
}

/// camera-core modifiers of egui modifiers: `ctrl` is the physical Control key, `command` egui's
/// platform command (Ctrl on Windows / Linux, Cmd on macOS).
fn mods_of(m: egui::Modifiers) -> Mods {
    Mods {
        ctrl: m.ctrl,
        alt: m.alt,
        shift: m.shift,
        command: m.command,
    }
}

/// Physical keys held, each with the logical key of its press (see the module docs).
#[derive(Clone, Copy, Debug, Default)]
struct HeldKeys([Option<(Key, egui::Key)>; HELD_MAX]);

impl HeldKeys {
    /// `key` went down (or auto-repeats) as the logical `logical`.
    fn press(&mut self, key: Key, logical: egui::Key) {
        if let Some(entry) = self.0.iter_mut().flatten().find(|(k, _)| *k == key) {
            entry.1 = logical;
        } else if let Some(slot) = self.0.iter_mut().find(|s| s.is_none()) {
            *slot = Some((key, logical));
        }
    }

    /// A key went up: drop the entry of its physical key and any entry pressed as `logical`.
    fn release(&mut self, key: Option<Key>, logical: egui::Key) {
        for slot in &mut self.0 {
            if slot.is_some_and(|(k, l)| Some(k) == key || l == logical) {
                *slot = None;
            }
        }
    }

    /// Drop every entry whose logical key egui no longer holds.
    fn prune(&mut self, logical_down: impl Fn(egui::Key) -> bool) {
        for slot in &mut self.0 {
            if slot.is_some_and(|(_, l)| !logical_down(l)) {
                *slot = None;
            }
        }
    }

    /// The held physical keys.
    fn set(&self) -> KeySet {
        self.0
            .iter()
            .flatten()
            .fold(KeySet::EMPTY, |set, (k, _)| set.with(*k))
    }
}

/// The id of a per-egui-viewport temp value.
fn viewport_id(ctx: &egui::Context, salt: &str) -> Id {
    Id::new(salt).with(ctx.viewport_id())
}

/// Set the unit of raw relative mouse motion (`PointerState::motion`) for this egui viewport;
/// default [`RawMotionUnits::PLATFORM_DEFAULT`]. A host running on Wayland (winit
/// `ActiveEventLoopExtWayland::is_wayland`) passes [`RawMotionUnits::Logical`]; see the module
/// docs for the per-backend table. Stored in egui temp memory (`Copy`, updated in place, so
/// calling it every frame allocates nothing); [`read_input`] reads it while a view owns a
/// `Locked` cursor.
pub fn set_raw_motion_units(ctx: &egui::Context, units: RawMotionUnits) {
    let id = viewport_id(ctx, MOTION_UNITS_SALT);
    ctx.data_mut(|d| {
        *d.get_temp_mut_or_insert_with(id, || RawMotionUnits::PLATFORM_DEFAULT) = units;
    });
}

/// This viewport's raw motion unit ([`set_raw_motion_units`]).
fn raw_motion_units(ctx: &egui::Context) -> RawMotionUnits {
    let id = viewport_id(ctx, MOTION_UNITS_SALT);
    ctx.data(|d| d.get_temp::<RawMotionUnits>(id))
        .unwrap_or(RawMotionUnits::PLATFORM_DEFAULT)
}

/// The cursor of one egui viewport. Invariant: `owner.is_some()` exactly when `applied` is not
/// `CursorRequest::Default`.
#[derive(Clone, Copy, Debug, Default)]
struct CursorState {
    /// What the OS cursor was last told.
    applied: CursorRequest,
    /// The view (`Response::id`) holding a non-default cursor.
    owner: Option<Id>,
    /// `Context::cumulative_frame_nr` of the owner's last [`apply_cursor`].
    owner_frame: u64,
    /// [`CursorRelease`] is registered (only to skip `add_plugin` calls; it dedupes by type).
    watched: bool,
}

/// This viewport's cursor state (a copy; `Copy`, no allocation).
fn cursor_state(ctx: &egui::Context) -> CursorState {
    let id = viewport_id(ctx, CURSOR_STATE_SALT);
    ctx.data(|d| d.get_temp::<CursorState>(id))
        .unwrap_or_default()
}

/// Tell the OS cursor `request` (the two grabs of `Locked`: see [`apply_cursor`]).
fn send_cursor(ctx: &egui::Context, request: CursorRequest) {
    let commands: &[ViewportCommand] = match request {
        CursorRequest::Default => &[
            ViewportCommand::CursorGrab(CursorGrab::None),
            ViewportCommand::CursorVisible(true),
        ],
        CursorRequest::Hidden => &[
            ViewportCommand::CursorGrab(CursorGrab::None),
            ViewportCommand::CursorVisible(false),
        ],
        CursorRequest::Locked => &[
            ViewportCommand::CursorGrab(CursorGrab::Confined),
            ViewportCommand::CursorGrab(CursorGrab::Locked),
            ViewportCommand::CursorVisible(false),
        ],
    };
    for command in commands {
        ctx.send_viewport_cmd(command.clone());
    }
}

/// End-of-pass watchdog: frees a cursor whose owner did not call [`apply_cursor`] this frame
/// (hidden dock tab, closed panel), so the OS cursor never stays grabbed by an undrawn view.
/// Only runs inside `Context::run_ui` (egui calls plugins there; a host driving
/// `begin_pass` / `end_pass` by hand gets no automatic release).
struct CursorRelease;

impl egui::Plugin for CursorRelease {
    fn debug_name(&self) -> &'static str {
        CURSOR_RELEASE_NAME
    }

    fn on_end_pass(&mut self, ui: &mut egui::Ui) {
        release_stale(ui.ctx());
    }
}

/// Free this viewport's cursor if its owner was not drawn this frame.
fn release_stale(ctx: &egui::Context) {
    let frame = ctx.cumulative_frame_nr();
    let id = viewport_id(ctx, CURSOR_STATE_SALT);
    let stale = ctx.data_mut(|d| {
        let s = d.get_temp_mut_or_default::<CursorState>(id);
        let stale = s.owner.is_some() && s.owner_frame != frame;
        if stale {
            s.owner = None;
            s.applied = CursorRequest::Default;
        }
        stale
    });
    if stale {
        send_cursor(ctx, CursorRequest::Default);
    }
}

/// One frame of camera input over the viewport `response`, in viewport-local logical points.
///
/// - `pointer`: the latest pointer position minus `response.rect.min` (`None` when the pointer
///   left the window); `pointer_delta`: egui's pointer delta, or the raw device motion while
///   THIS view owns a `Locked` cursor ([`apply_cursor`]; units per platform in the module docs);
/// - buttons down / pressed / released (egui `Extra1` / `Extra2` = `Back` / `Forward`);
/// - `mods` from egui's modifiers (`ctrl` physical Control, `command` the platform command);
/// - keys down / pressed (no auto-repeat) by physical position, logical where egui gives none;
/// - wheel lines / points by `MouseWheelUnit` over EVERY raw wheel event, any modifiers (`Page` =
///   [`PAGE_NOTCHES`] lines); `pinch` = the product of the frame's raw `Event::Zoom` factors,
///   never egui's `zoom_delta()` (module docs);
/// - `keys_allowed`: the window has keyboard focus, the view is focused or hovered (or owns a
///   `Locked` cursor), and no other widget's text field holds the keyboard focus
///   (`Context::text_edit_focused`). Other focused widgets (a button reached with Tab) do not
///   block the camera keys;
/// - `hovered`: `response.hovered()` (false under a popup / modal or while another widget drags)
///   AND the window has a pointer (`PointerState::has_pointer`): egui keeps `interact_pos` after
///   `PointerGone`, so `hovered()` alone stays true once the pointer has left the window. Always
///   true while the window has focus and THIS view owns a `Locked` cursor ([`apply_cursor`]):
///   on X11 the pointer still moves under a Confined-only grab and may leave the view;
/// - `viewport`: `response.rect.size()`; `dt` as given (egui's `stable_dt` is the usual value).
///
/// Read-only: nothing is consumed (see [`consume_wheel`]).
pub fn read_input(ui: &egui::Ui, response: &egui::Response, dt: f32) -> RawInput {
    let ctx = ui.ctx();
    let rect = response.rect;
    let cursor = cursor_state(ctx);
    let locked = cursor.applied == CursorRequest::Locked && cursor.owner == Some(response.id);
    // Read outside `ctx.input`: its closure holds the context lock.
    let units = raw_motion_units(ctx);
    let zoom_factor = ctx.zoom_factor();
    let held_id = viewport_id(ctx, HELD_KEYS_SALT);
    let mut held = ctx
        .data(|d| d.get_temp::<HeldKeys>(held_id))
        .unwrap_or_default();

    let mut raw = ctx.input(|i| {
        let mut buttons_down = ButtonSet::EMPTY;
        let mut buttons_pressed = ButtonSet::EMPTY;
        let mut buttons_released = ButtonSet::EMPTY;
        for (egui_button, button) in BUTTONS {
            if i.pointer.button_down(egui_button) {
                buttons_down.insert(button);
            }
            if i.pointer.button_pressed(egui_button) {
                buttons_pressed.insert(button);
            }
            if i.pointer.button_released(egui_button) {
                buttons_released.insert(button);
            }
        }

        let mut keys_pressed = KeySet::EMPTY;
        let mut lines = egui::Vec2::ZERO;
        let mut points = egui::Vec2::ZERO;
        let mut pinch = 1.0_f32;
        for event in &i.events {
            match event {
                Event::Key {
                    key,
                    physical_key,
                    pressed,
                    repeat,
                    ..
                } => {
                    let camera_key = key_of(physical_key.unwrap_or(*key));
                    if *pressed {
                        if let Some(k) = camera_key {
                            held.press(k, *key);
                            if !*repeat {
                                keys_pressed.insert(k);
                            }
                        }
                    } else {
                        held.release(camera_key, *key);
                    }
                }
                // Every wheel event, any modifiers: egui's own zoom / scroll split is not used.
                Event::MouseWheel { unit, delta, .. } => match unit {
                    MouseWheelUnit::Line => lines += *delta,
                    MouseWheelUnit::Point => points += *delta,
                    MouseWheelUnit::Page => lines += *delta * PAGE_NOTCHES,
                },
                Event::Zoom(factor) if factor.is_finite() && *factor > 0.0 => pinch *= *factor,
                _ => {}
            }
        }
        held.prune(|k| i.keys_down.contains(&k));

        let pointer_delta = match i.pointer.motion() {
            Some(motion) if locked => {
                units.to_points(to_core(motion), i.pixels_per_point(), zoom_factor)
            }
            _ => to_core(i.pointer.delta()),
        };
        RawInput {
            pointer: i.pointer.latest_pos().map(|p| to_core(p - rect.min)),
            pointer_delta,
            buttons_down,
            buttons_pressed,
            buttons_released,
            mods: mods_of(i.modifiers),
            keys_down: held.set(),
            keys_pressed,
            wheel: Wheel {
                lines: to_core(lines),
                points: to_core(points),
            },
            pinch,
            dt,
            viewport: to_core(rect.size()),
            ..RawInput::default()
        }
    });
    // In place: `insert_temp` would box a new value every frame.
    ctx.data_mut(|d| *d.get_temp_mut_or_default::<HeldKeys>(held_id) = held);

    let window_focused = ctx.input(|i| i.focused);
    // While this view owns a `Locked` cursor the pointer may still move off the view (X11 only
    // confines): the owner stays hovered and is the keyboard target (see the module docs).
    let owns_lock = locked && window_focused;
    // A running drag stays this view's even when the frame ends on `PointerGone` (a drag across
    // the window edge on macOS / X11), or the driver's release edge would cancel it.
    raw.hovered = owns_lock
        || response.dragged()
        || (response.hovered() && ctx.input(|i| i.pointer.has_pointer()));
    let other_text_focused = ctx
        .memory(|m| m.focused())
        .is_some_and(|id| id != response.id)
        && ctx.text_edit_focused();
    raw.keys_allowed =
        window_focused && (response.has_focus() || raw.hovered) && !other_text_focused;
    raw
}

/// Zero egui's smoothed scroll for the rest of this frame, so an enclosing `ScrollArea` does not
/// scroll while the camera uses the wheel. [`drive`] calls it while the view is hovered; a host
/// calling [`read_input`] directly decides itself. egui turns a wheel whose (latched) modifiers
/// match its zoom modifier (Ctrl / Cmd) into `zoom_delta` instead of a smoothed scroll
/// (`input_state/mod.rs`), so that gesture leaves nothing here for a `ScrollArea`; the private
/// `zoom_delta` state itself cannot be cleared, so a widget under the view that reads
/// `zoom_delta()` (a plot, a scene) still sees Ctrl+wheel: keep such widgets away from the view.
pub fn consume_wheel(ctx: &egui::Context) {
    ctx.input_mut(|i| i.smooth_scroll_delta = egui::Vec2::ZERO);
}

/// Apply the cursor request of the camera view `view` (its `Response::id`) to this egui
/// viewport. Call it every frame the view is drawn (a view that stops calling loses an owned
/// cursor at the end of that frame).
///
/// Ownership (module docs): a free cursor is taken by the first view asking for `Hidden` /
/// `Locked`; while owned, other views' requests are ignored; the owner's `Default` frees it.
/// `ViewportCommand`s go out only on a change of the applied request; the owner's hidden
/// `CursorIcon` is re-set each frame (egui resets it). The state is mutated in place in egui temp
/// memory (no per-frame allocation).
///
/// - `Default`: `CursorGrab::None`, visible.
/// - `Hidden`: `CursorGrab::None`, invisible (never confined: a confine froze pointer input in a
///   host backend).
/// - `Locked`: `CursorGrab::Confined` THEN `CursorGrab::Locked`, invisible. Support in winit
///   0.30.13: Windows and Wayland grant both (the later `Locked` wins); macOS grants `Locked`
///   only (the `Confined` request is logged as unsupported); X11 grants `Confined` only (the
///   `Locked` request is logged as unsupported): the cursor stays confined to the window but the
///   pointer position still moves, so while a view owns `Locked` [`read_input`] reports it
///   hovered / keyboard-targeted even when the hidden pointer leaves its rect, and raw motion
///   still steers. Web: eframe ignores `CursorGrab` and `CursorVisible` and sends no `MouseMoved`,
///   so `Locked` degrades to a hidden icon (`CursorIcon::None`) on a free cursor with egui's
///   ordinary pointer delta. Sending both grabs keeps one code path; the integration's warning
///   per capture is the accepted cost.
pub fn apply_cursor(ctx: &egui::Context, view: Id, request: CursorRequest) {
    let frame = ctx.cumulative_frame_nr();
    let id = viewport_id(ctx, CURSOR_STATE_SALT);
    let (send, owns, watch) = ctx.data_mut(|d| {
        let s = d.get_temp_mut_or_default::<CursorState>(id);
        if s.owner.is_some_and(|owner| owner != view) {
            return (None, false, false);
        }
        let send = (s.applied != request).then_some(request);
        s.applied = request;
        if request == CursorRequest::Default {
            s.owner = None;
        } else {
            s.owner = Some(view);
            s.owner_frame = frame;
        }
        let watch = s.owner.is_some() && !s.watched;
        s.watched |= watch;
        (send, s.owner.is_some(), watch)
    });
    if watch {
        ctx.add_plugin(CursorRelease);
    }
    if owns {
        ctx.set_cursor_icon(CursorIcon::None);
    }
    if let Some(request) = send {
        send_cursor(ctx, request);
    }
}

/// One camera frame over the viewport `response`: [`read_input`] (with egui's `stable_dt`),
/// [`consume_wheel`] while hovered, `CameraDriver::update`, [`apply_cursor`] owned by
/// `response.id`, and a repaint request while the driver still wants one. The host answers
/// `DriverFrame::requests` and renders with `DriverFrame::pose`.
///
/// The `response` must have a STABLE id (allocate it with `ui.interact(rect, id, sense)` or an
/// `id_salt`-ed widget): cursor ownership is keyed by `response.id`, so an id that changes
/// between frames makes the view lose its own `Locked` cursor (it reads as another view).
pub fn drive(
    ui: &egui::Ui,
    response: &egui::Response,
    driver: &mut CameraDriver,
    scene_radius: f32,
) -> DriverFrame {
    let dt = ui.input(|i| i.stable_dt);
    let raw = read_input(ui, response, dt);
    if raw.hovered {
        consume_wheel(ui.ctx());
    }
    let frame = driver.update(&raw, scene_radius);
    apply_cursor(ui.ctx(), response.id, frame.cursor);
    if frame.wants_repaint {
        ui.ctx().request_repaint();
    }
    frame
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera_core::NavPrefs;
    use crate::camera_core::input::NavScheme;
    use egui::{Context, Modifiers, Pos2, Rect, Sense, TouchPhase, Ui, Vec2, vec2};
    use std::cell::Cell;

    const DT: f32 = 1.0 / 60.0;
    const SCREEN: Vec2 = Vec2::new(800.0, 600.0);
    const VIEW: Vec2 = Vec2::new(400.0, 300.0);
    /// Frames a wheel zoom gets to show in the pose.
    const ZOOM_SETTLE_FRAMES: usize = 30;

    struct Frame {
        raw: RawInput,
        rect: Rect,
        out: egui::FullOutput,
    }

    /// One egui frame: `before` draws first, then the 400 x 300 view is read; `cursor` (if any)
    /// is applied after the read.
    fn run(
        ctx: &Context,
        events: Vec<Event>,
        before: &mut dyn FnMut(&mut Ui),
        cursor: Option<CursorRequest>,
    ) -> Frame {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SCREEN)),
            events,
            ..Default::default()
        };
        let mut got = None;
        let mut out = ctx.run_ui(input, |ui| {
            before(ui);
            let (rect, response) = ui.allocate_exact_size(VIEW, Sense::click_and_drag());
            got = Some((read_input(ui, &response, DT), rect));
            if let Some(request) = cursor {
                apply_cursor(ui.ctx(), response.id, request);
            }
        });
        out.textures_delta.clear();
        let Some((raw, rect)) = got else {
            panic!("the ui closure did not run");
        };
        Frame { raw, rect, out }
    }

    /// No widget before the view.
    fn nothing(_: &mut Ui) {}

    fn step(ctx: &Context, events: Vec<Event>) -> Frame {
        run(ctx, events, &mut nothing, None)
    }

    /// Put the pointer on the view centre and let egui's hit test see it. Returns the centre.
    fn hover(ctx: &Context, before: &mut dyn FnMut(&mut Ui)) -> Pos2 {
        let centre = run(ctx, Vec::new(), before, None).rect.center();
        run(ctx, vec![Event::PointerMoved(centre)], before, None);
        let f = run(ctx, Vec::new(), before, None);
        assert!(f.raw.hovered, "the view is hovered after the warm-up");
        centre
    }

    fn key(key: egui::Key, physical: Option<egui::Key>, pressed: bool) -> Event {
        Event::Key {
            key,
            physical_key: physical,
            pressed,
            repeat: false,
            modifiers: Modifiers::NONE,
        }
    }

    fn wheel(unit: MouseWheelUnit, delta: Vec2, modifiers: Modifiers) -> Event {
        Event::MouseWheel {
            unit,
            delta,
            phase: TouchPhase::Move,
            modifiers,
        }
    }

    fn cursor_commands(out: &egui::FullOutput) -> Vec<ViewportCommand> {
        out.viewport_output
            .get(&egui::ViewportId::ROOT)
            .map(|v| {
                v.commands
                    .iter()
                    .filter(|c| {
                        matches!(
                            c,
                            ViewportCommand::CursorGrab(_) | ViewportCommand::CursorVisible(_)
                        )
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Pointer position is viewport-local, the delta is egui's, the size is the response rect,
    /// `dt` is passed through, every egui button maps to its camera-core name.
    #[test]
    fn pointer_and_buttons_map() {
        let ctx = Context::default();
        let c = hover(&ctx, &mut nothing);
        let p = c + vec2(5.0, -3.0);
        let f = step(&ctx, vec![Event::PointerMoved(p)]);
        let local = p - f.rect.min;
        assert_eq!(f.raw.pointer.map(|q| (q.x, q.y)), Some((local.x, local.y)));
        assert_eq!((f.raw.pointer_delta.x, f.raw.pointer_delta.y), (5.0, -3.0));
        assert_eq!((f.raw.viewport.x, f.raw.viewport.y), (VIEW.x, VIEW.y));
        assert_eq!(f.raw.dt, DT);
        assert!(f.raw.hovered);

        for (egui_button, button) in BUTTONS {
            let press = Event::PointerButton {
                pos: p,
                button: egui_button,
                pressed: true,
                modifiers: Modifiers::NONE,
            };
            let f = step(&ctx, vec![press]);
            assert!(f.raw.buttons_pressed.contains(button), "{button:?} pressed");
            assert!(f.raw.buttons_down.contains(button), "{button:?} down");
            assert!(!f.raw.buttons_released.contains(button));
            let release = Event::PointerButton {
                pos: p,
                button: egui_button,
                pressed: false,
                modifiers: Modifiers::NONE,
            };
            let f = step(&ctx, vec![release]);
            assert!(
                f.raw.buttons_released.contains(button),
                "{button:?} released"
            );
            assert!(!f.raw.buttons_down.contains(button), "{button:?} up");
            assert!(!f.raw.buttons_pressed.contains(button));
        }

        let f = step(&ctx, vec![Event::PointerGone]);
        assert_eq!(f.raw.pointer, None);
        assert!(!f.raw.hovered);
    }

    /// `ctrl` is the physical Control key, `command` egui's platform command (a Mac Cmd sets
    /// `command` only).
    #[test]
    fn modifiers_map() {
        let ctx = Context::default();
        let win_ctrl = Modifiers {
            ctrl: true,
            command: true,
            ..Modifiers::NONE
        };
        let f = step(&ctx, vec![Event::ModifiersChanged(win_ctrl)]);
        assert_eq!(f.raw.mods, Mods::CTRL.plus(Mods::COMMAND));
        let mac_cmd = Modifiers {
            mac_cmd: true,
            command: true,
            ..Modifiers::NONE
        };
        let f = step(&ctx, vec![Event::ModifiersChanged(mac_cmd)]);
        assert_eq!(f.raw.mods, Mods::COMMAND);
        let f = step(
            &ctx,
            vec![Event::ModifiersChanged(Modifiers::ALT | Modifiers::SHIFT)],
        );
        assert_eq!(f.raw.mods, Mods::ALT.plus(Mods::SHIFT));
    }

    /// Keys are read by physical position (AZERTY: physical W types Z), held across frames, a
    /// repeat is no new press, a key without a physical code falls back to the logical one, and a
    /// window focus loss drops every held key.
    #[test]
    fn keys_map_by_physical_position() {
        let ctx = Context::default();
        let f = step(&ctx, vec![key(egui::Key::Z, Some(egui::Key::W), true)]);
        assert!(f.raw.keys_pressed.contains(Key::W));
        assert!(f.raw.keys_down.contains(Key::W));
        assert!(!f.raw.keys_down.contains(Key::Z));

        let f = step(&ctx, Vec::new());
        assert!(f.raw.keys_down.contains(Key::W), "held across frames");
        assert!(f.raw.keys_pressed.is_empty());

        // egui marks a second press without a release as a repeat.
        let f = step(&ctx, vec![key(egui::Key::Z, Some(egui::Key::W), true)]);
        assert!(f.raw.keys_pressed.is_empty(), "a repeat is not a press");
        assert!(f.raw.keys_down.contains(Key::W));

        let f = step(&ctx, vec![key(egui::Key::Z, Some(egui::Key::W), false)]);
        assert!(f.raw.keys_down.is_empty());

        let f = step(&ctx, vec![key(egui::Key::Num1, None, true)]);
        assert!(f.raw.keys_pressed.contains(Key::Num1), "logical fallback");
        let f = step(&ctx, vec![Event::WindowFocused(false)]);
        assert!(f.raw.keys_down.is_empty(), "focus loss drops held keys");

        let f = step(&ctx, vec![key(egui::Key::F1, None, true)]);
        assert!(f.raw.keys_down.is_empty(), "unmapped keys are ignored");
    }

    /// Lines, points and pages land in their own field; a Ctrl / Cmd + wheel notch is a plain
    /// notch (egui's own zoom of it is never read), pinch comes only from `Event::Zoom`.
    #[test]
    fn wheel_honours_the_unit() {
        let ctx = Context::default();
        let f = step(
            &ctx,
            vec![wheel(MouseWheelUnit::Line, vec2(0.0, 1.0), Modifiers::NONE)],
        );
        assert_eq!((f.raw.wheel.lines.x, f.raw.wheel.lines.y), (0.0, 1.0));
        assert_eq!(f.raw.wheel.points.y, 0.0);

        let f = step(
            &ctx,
            vec![wheel(
                MouseWheelUnit::Point,
                vec2(0.0, -12.0),
                Modifiers::NONE,
            )],
        );
        assert_eq!(f.raw.wheel.points.y, -12.0);
        assert_eq!(f.raw.wheel.lines.y, 0.0);

        let f = step(
            &ctx,
            vec![wheel(MouseWheelUnit::Page, vec2(0.0, 1.0), Modifiers::NONE)],
        );
        assert_eq!(
            f.raw.wheel.lines.y, PAGE_NOTCHES,
            "a page is a fixed notch count"
        );
        assert_eq!(f.raw.wheel.points.y, 0.0);

        // Ctrl (egui's zoom modifier): one notch, and egui's zoom_delta is not a pinch.
        let ctx = Context::default();
        let ctrl = Modifiers {
            ctrl: true,
            command: true,
            ..Modifiers::NONE
        };
        let f = step(
            &ctx,
            vec![wheel(MouseWheelUnit::Line, vec2(0.0, 1.0), ctrl)],
        );
        assert_eq!(f.raw.wheel.lines.y, 1.0, "Ctrl+wheel is one plain notch");
        assert_eq!(f.raw.pinch, 1.0, "no raw Zoom event: no pinch");
        // egui latches the wheel modifiers: the next frame's smoothing is still "zoom" for egui,
        // and a Ctrl-less notch after it still counts once, with no pinch.
        let f = step(
            &ctx,
            vec![wheel(MouseWheelUnit::Line, vec2(0.0, 1.0), Modifiers::NONE)],
        );
        assert_eq!(f.raw.wheel.lines.y, 1.0);
        assert_eq!(f.raw.pinch, 1.0);
    }

    /// Pinch is the product of the raw `Event::Zoom` factors (finite, > 0) and moves no wheel.
    #[test]
    fn pinch_comes_only_from_zoom_events() {
        let ctx = Context::default();
        let f = step(&ctx, vec![Event::Zoom(1.25)]);
        assert_eq!(f.raw.pinch, 1.25);
        assert_eq!(f.raw.wheel, Wheel::default());

        let f = step(
            &ctx,
            vec![
                Event::Zoom(2.0),
                Event::Zoom(1.5),
                Event::Zoom(f32::NAN),
                Event::Zoom(0.0),
                Event::Zoom(-1.0),
                Event::Zoom(f32::INFINITY),
            ],
        );
        assert_eq!(f.raw.pinch, 3.0, "product of the valid factors only");
        let f = step(&ctx, Vec::new());
        assert_eq!(f.raw.pinch, 1.0);
    }

    /// The typing guard: true over the hovered view, false while a TextEdit holds the focus,
    /// true again once it lets go, false when the pointer leaves an unfocused view.
    #[test]
    fn keys_allowed_respects_a_focused_text_edit() {
        let ctx = Context::default();
        let mut text = String::from("name");
        // Some(true) = request the focus this frame, Some(false) = surrender it.
        let focus = Cell::new(None::<bool>);
        let mut text_edit = |ui: &mut Ui| {
            let r = ui.text_edit_singleline(&mut text);
            match focus.get() {
                Some(true) => r.request_focus(),
                Some(false) => r.surrender_focus(),
                None => {}
            }
        };
        hover(&ctx, &mut text_edit);
        let f = run(&ctx, Vec::new(), &mut text_edit, None);
        assert!(f.raw.keys_allowed, "hovered, nothing focused");

        focus.set(Some(true));
        let f = run(&ctx, Vec::new(), &mut text_edit, None);
        assert!(f.raw.hovered);
        assert!(
            !f.raw.keys_allowed,
            "a focused text field owns the keyboard"
        );
        focus.set(None);
        let f = run(&ctx, Vec::new(), &mut text_edit, None);
        assert!(!f.raw.keys_allowed, "still focused");

        focus.set(Some(false));
        let f = run(&ctx, Vec::new(), &mut text_edit, None);
        assert!(f.raw.keys_allowed, "focus released");

        focus.set(None);
        let f = run(&ctx, vec![Event::PointerGone], &mut text_edit, None);
        assert!(!f.raw.keys_allowed, "not hovered, not focused");
    }

    /// Commands go out once per change; the hidden icon is set every hidden frame; while locked
    /// the pointer delta is the raw device motion.
    #[test]
    fn cursor_commands_only_on_change() {
        let ctx = Context::default();
        let f = run(&ctx, Vec::new(), &mut nothing, Some(CursorRequest::Default));
        assert!(cursor_commands(&f.out).is_empty(), "already free");

        let f = run(&ctx, Vec::new(), &mut nothing, Some(CursorRequest::Locked));
        assert_eq!(
            cursor_commands(&f.out),
            vec![
                ViewportCommand::CursorGrab(CursorGrab::Confined),
                ViewportCommand::CursorGrab(CursorGrab::Locked),
                ViewportCommand::CursorVisible(false),
            ]
        );
        assert_eq!(f.out.platform_output.cursor_icon, CursorIcon::None);

        let f = run(
            &ctx,
            vec![Event::MouseMoved(vec2(7.0, -2.0))],
            &mut nothing,
            Some(CursorRequest::Locked),
        );
        assert!(cursor_commands(&f.out).is_empty(), "no command spam");
        assert_eq!(f.out.platform_output.cursor_icon, CursorIcon::None);
        assert_eq!(
            (f.raw.pointer_delta.x, f.raw.pointer_delta.y),
            (7.0, -2.0),
            "locked: raw device motion"
        );

        let f = run(&ctx, Vec::new(), &mut nothing, Some(CursorRequest::Hidden));
        assert_eq!(
            cursor_commands(&f.out),
            vec![
                ViewportCommand::CursorGrab(CursorGrab::None),
                ViewportCommand::CursorVisible(false),
            ]
        );

        let f = run(&ctx, Vec::new(), &mut nothing, Some(CursorRequest::Default));
        assert_eq!(
            cursor_commands(&f.out),
            vec![
                ViewportCommand::CursorGrab(CursorGrab::None),
                ViewportCommand::CursorVisible(true),
            ]
        );
        assert_eq!(f.out.platform_output.cursor_icon, CursorIcon::Default);
        let f = run(&ctx, Vec::new(), &mut nothing, Some(CursorRequest::Default));
        assert!(cursor_commands(&f.out).is_empty());
    }

    /// The raw motion unit is a per-viewport runtime override (default: the platform's):
    /// `Physical` divides by `pixels_per_point` (native scale 2 x egui zoom 2 = 4), `Logical` only
    /// by egui's zoom factor (2).
    #[test]
    fn locked_motion_follows_the_unit_override() {
        let ctx = Context::default();
        assert_eq!(raw_motion_units(&ctx), RawMotionUnits::PLATFORM_DEFAULT);
        let frame = |events: Vec<Event>| {
            let native = egui::ViewportInfo {
                native_pixels_per_point: Some(2.0),
                ..Default::default()
            };
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SCREEN)),
                viewports: [(egui::ViewportId::ROOT, native)].into_iter().collect(),
                events,
                ..Default::default()
            };
            let mut got = None;
            let mut out = ctx.run_ui(input, |ui| {
                let (_, response) = ui.allocate_exact_size(VIEW, Sense::click_and_drag());
                got = Some(read_input(ui, &response, DT).pointer_delta);
                apply_cursor(ui.ctx(), response.id, CursorRequest::Locked);
            });
            // A new pixels_per_point rebuilds the font atlas: no renderer applies the delta here,
            // and epaint debug-asserts that an unapplied `TexturesDelta` is never dropped.
            out.textures_delta.clear();
            let Some(delta) = got else {
                panic!("the ui closure did not run");
            };
            (delta.x, delta.y)
        };
        ctx.set_zoom_factor(2.0);
        // Warm-up: the zoom becomes active, the view takes the lock.
        frame(Vec::new());
        frame(Vec::new());
        frame(Vec::new());
        assert_eq!(ctx.zoom_factor(), 2.0);

        let motion = || vec![Event::MouseMoved(vec2(8.0, -4.0))];
        set_raw_motion_units(&ctx, RawMotionUnits::Physical);
        assert_eq!(frame(motion()), (2.0, -1.0));
        set_raw_motion_units(&ctx, RawMotionUnits::Logical);
        assert_eq!(raw_motion_units(&ctx), RawMotionUnits::Logical);
        assert_eq!(frame(motion()), (4.0, -2.0));
    }

    /// X11 only confines: the hidden pointer may leave the view rect while the view owns the
    /// `Locked` cursor. It stays hovered and the keyboard target; a free cursor outside is not.
    #[test]
    fn locked_owner_stays_hovered_when_the_pointer_leaves_the_view() {
        let outside = Pos2::new(700.0, 550.0);
        let ctx = Context::default();
        hover(&ctx, &mut nothing);
        run(&ctx, Vec::new(), &mut nothing, Some(CursorRequest::Locked));
        let f = run(
            &ctx,
            vec![Event::PointerMoved(outside)],
            &mut nothing,
            Some(CursorRequest::Locked),
        );
        assert!(!f.rect.contains(outside));
        assert!(f.raw.hovered, "the lock owner is hovered off its rect");
        assert!(f.raw.keys_allowed, "and is the keyboard target");
        let f = run(&ctx, Vec::new(), &mut nothing, Some(CursorRequest::Default));
        assert!(f.raw.hovered);
        let f = run(&ctx, Vec::new(), &mut nothing, Some(CursorRequest::Default));
        assert!(!f.raw.hovered, "released: an off-view pointer is not hover");
        assert!(!f.raw.keys_allowed);

        // Another view's lock does not make THIS view hovered.
        let ctx = Context::default();
        let other = Id::new("some other view");
        cursor_frame(&ctx, &[(other, CursorRequest::Locked)]);
        let f = run(&ctx, vec![Event::PointerMoved(outside)], &mut nothing, None);
        assert!(!f.raw.hovered);
    }

    /// A drag that leaves the window (`PointerGone` ends the frame) keeps the view hovered, so the
    /// driver does not release (cancel) the running orbit / pan drag.
    #[test]
    fn a_drag_across_the_window_edge_stays_hovered() {
        let ctx = Context::default();
        let p = hover(&ctx, &mut nothing);
        let press = Event::PointerButton {
            pos: p,
            button: PointerButton::Primary,
            pressed: true,
            modifiers: Modifiers::NONE,
        };
        step(&ctx, vec![press]);
        step(&ctx, vec![Event::PointerMoved(p + vec2(30.0, 0.0))]);
        let f = step(&ctx, vec![Event::PointerGone]);
        assert!(f.raw.buttons_down.contains(Button::Primary));
        assert!(
            f.raw.hovered,
            "the dragged view stays hovered after PointerGone"
        );
        assert!(f.raw.keys_allowed);
    }

    /// The typing guard still applies to a lock owner: another widget's text field wins.
    #[test]
    fn locked_owner_respects_a_focused_text_edit() {
        let ctx = Context::default();
        let mut text = String::from("name");
        let focus = Cell::new(false);
        let mut text_edit = |ui: &mut Ui| {
            let r = ui.text_edit_singleline(&mut text);
            if focus.get() {
                r.request_focus();
            }
        };
        hover(&ctx, &mut text_edit);
        run(
            &ctx,
            Vec::new(),
            &mut text_edit,
            Some(CursorRequest::Locked),
        );
        let f = run(
            &ctx,
            Vec::new(),
            &mut text_edit,
            Some(CursorRequest::Locked),
        );
        assert!(f.raw.keys_allowed, "locked, no text field focused");
        focus.set(true);
        run(
            &ctx,
            Vec::new(),
            &mut text_edit,
            Some(CursorRequest::Locked),
        );
        let f = run(
            &ctx,
            Vec::new(),
            &mut text_edit,
            Some(CursorRequest::Locked),
        );
        assert!(f.raw.hovered);
        assert!(
            !f.raw.keys_allowed,
            "a focused text field owns the keyboard"
        );
    }

    const LOCK: [ViewportCommand; 3] = [
        ViewportCommand::CursorGrab(CursorGrab::Confined),
        ViewportCommand::CursorGrab(CursorGrab::Locked),
        ViewportCommand::CursorVisible(false),
    ];
    const FREE: [ViewportCommand; 2] = [
        ViewportCommand::CursorGrab(CursorGrab::None),
        ViewportCommand::CursorVisible(true),
    ];
    const HIDE: [ViewportCommand; 2] = [
        ViewportCommand::CursorGrab(CursorGrab::None),
        ViewportCommand::CursorVisible(false),
    ];

    /// One frame of bare `apply_cursor` calls, in order; returns the cursor commands.
    fn cursor_frame(ctx: &Context, calls: &[(Id, CursorRequest)]) -> Vec<ViewportCommand> {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SCREEN)),
            ..Default::default()
        };
        let mut out = ctx.run_ui(input, |ui| {
            for (view, request) in calls {
                apply_cursor(ui.ctx(), *view, *request);
            }
        });
        out.textures_delta.clear();
        cursor_commands(&out)
    }

    /// The cursor belongs to one view: another view's `Default` or `Hidden` never touches it,
    /// the owner's frames send nothing new, an owner that is not drawn loses it at the end of
    /// that frame, and then another view may take it.
    #[test]
    fn cursor_has_one_owner() {
        let ctx = Context::default();
        let (a, b) = (Id::new("view a"), Id::new("view b"));
        let lock_a = (a, CursorRequest::Locked);
        let idle_b = (b, CursorRequest::Default);
        assert_eq!(cursor_frame(&ctx, &[lock_a, idle_b]), LOCK.to_vec());
        for frame in 0..5 {
            assert!(
                cursor_frame(&ctx, &[lock_a, idle_b]).is_empty(),
                "frame {frame}: no toggling between the views"
            );
            assert!(
                cursor_frame(&ctx, &[idle_b, lock_a]).is_empty(),
                "frame {frame}: order does not matter"
            );
        }
        assert!(
            cursor_frame(&ctx, &[lock_a, (b, CursorRequest::Hidden)]).is_empty(),
            "another view cannot take an owned cursor"
        );
        assert_eq!(
            cursor_frame(&ctx, &[idle_b]),
            FREE.to_vec(),
            "the owner was not drawn: released at the end of the frame"
        );
        assert_eq!(
            cursor_frame(&ctx, &[(b, CursorRequest::Hidden)]),
            HIDE.to_vec(),
            "a free cursor can be taken by another view"
        );
        assert_eq!(
            cursor_frame(&ctx, &[(b, CursorRequest::Default)]),
            FREE.to_vec()
        );
        assert!(cursor_frame(&ctx, &[idle_b]).is_empty());
    }

    /// Two `drive` views in one window (Unreal scheme): an RMB flight in view A locks the cursor
    /// once, idle view B never toggles it while A flies, A's release frees it once; a hidden A
    /// (its tab switched away mid-flight) frees it at the end of that frame.
    #[test]
    fn two_drive_views_share_the_cursor() {
        const SMALL: Vec2 = Vec2::new(300.0, 200.0);
        let ctx = Context::default();
        let mut drivers = [
            CameraDriver::new(NavPrefs::default(), NavScheme::Unreal),
            CameraDriver::new(NavPrefs::default(), NavScheme::Unreal),
        ];
        let mut quad = |events: Vec<Event>, draw_a: bool| {
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SCREEN)),
                events,
                ..Default::default()
            };
            let mut rects = [Rect::NOTHING; 2];
            let mut out = ctx.run_ui(input, |ui| {
                for (i, driver) in drivers.iter_mut().enumerate() {
                    // The space is allocated even for a hidden view: B keeps its place.
                    let (rect, _) = ui.allocate_exact_size(SMALL, Sense::hover());
                    rects[i] = rect;
                    if i == 0 && !draw_a {
                        continue;
                    }
                    let id = Id::new(("quad view", i));
                    let response = ui.interact(rect, id, Sense::click_and_drag());
                    drive(ui, &response, driver, 10.0);
                }
            });
            out.textures_delta.clear();
            (cursor_commands(&out), rects)
        };
        let (_, rects) = quad(Vec::new(), true);
        let a = rects[0].center();
        quad(vec![Event::PointerMoved(a)], true);
        quad(Vec::new(), true);
        let rmb = |pressed| Event::PointerButton {
            pos: a,
            button: PointerButton::Secondary,
            pressed,
            modifiers: Modifiers::NONE,
        };

        assert_eq!(quad(vec![rmb(true)], true).0, LOCK.to_vec(), "A flies");
        for frame in 0..10 {
            assert!(
                quad(Vec::new(), true).0.is_empty(),
                "frame {frame}: idle B never toggles A's cursor"
            );
        }
        assert_eq!(quad(vec![rmb(false)], true).0, FREE.to_vec(), "A lands");
        assert!(quad(Vec::new(), true).0.is_empty());

        assert_eq!(
            quad(vec![rmb(true)], true).0,
            LOCK.to_vec(),
            "A flies again"
        );
        assert_eq!(
            quad(Vec::new(), false).0,
            FREE.to_vec(),
            "A hidden mid-flight: released"
        );
    }

    /// `drive`: an idle hovered view asks for nothing; a wheel notch over it zooms, asks for a
    /// repaint and leaves no smoothed scroll for an enclosing scroll area.
    #[test]
    fn drive_zooms_and_consumes_the_wheel() {
        let ctx = Context::default();
        hover(&ctx, &mut nothing);
        let mut driver = CameraDriver::default();
        let mut frame_of = |events: Vec<Event>| {
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SCREEN)),
                events,
                ..Default::default()
            };
            let mut got = None;
            let mut out = ctx.run_ui(input, |ui| {
                let (_, response) = ui.allocate_exact_size(VIEW, Sense::click_and_drag());
                let frame = drive(ui, &response, &mut driver, 10.0);
                got = Some((frame, ui.input(|i| i.smooth_scroll_delta)));
            });
            out.textures_delta.clear();
            let Some((frame, scroll)) = got else {
                panic!("the ui closure did not run");
            };
            (frame, scroll)
        };
        let (rest, _) = frame_of(Vec::new());
        let (idle, _) = frame_of(Vec::new());
        assert!(!idle.wants_repaint, "an idle view does not paint");
        assert_eq!(idle.pose, rest.pose);

        let (zoom, scroll) = frame_of(vec![wheel(
            MouseWheelUnit::Line,
            vec2(0.0, 1.0),
            Modifiers::NONE,
        )]);
        assert!(zoom.wants_repaint);
        assert_eq!(scroll, Vec2::ZERO, "the wheel was consumed");
        // The zoom may ease over a few frames (smooth zoom mode): settle, then compare.
        let mut last = zoom;
        for _ in 0..ZOOM_SETTLE_FRAMES {
            last = frame_of(Vec::new()).0;
        }
        assert_ne!(last.pose, rest.pose, "the wheel moved the camera");
    }
}
