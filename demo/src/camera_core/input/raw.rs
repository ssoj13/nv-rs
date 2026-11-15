//! [`RawInput`]: one frame's host-agnostic input snapshot, plus its own [`Button`], [`Mods`],
//! [`Key`] vocabulary and the bit sets [`ButtonSet`] / [`KeySet`].
//!
//! What: everything camera navigation reads in one frame (pointer position and motion, buttons
//! down / pressed / released, modifiers, the keys camera navigation needs, wheel lines / points,
//! pinch, the host typing guard, hover, `dt`, viewport size), as plain `Copy` data.
//!
//! Why: the navigation rules (`super::Navigator`) must not depend on egui or winit; every host
//! (the `camera-egui` reader, the `camera-winit` collector, a test) fills this struct and the
//! rules run the same everywhere.
//!
//! How: buttons and keys are bit sets (no allocation, `Copy`); [`RawInput::sanitized`] drops
//! non-finite host values before any rule sees them.
//!
//! Used by: `super::navigator`, `super::bindings`, `super::fly`, the host adapters, tests.

use glam::Vec2;

/// A pointer button.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Button {
    /// Left mouse button (LMB).
    Primary,
    /// Right mouse button (RMB).
    Secondary,
    /// Middle mouse button / wheel click (MMB).
    Middle,
    /// The "back" extra button.
    Back,
    /// The "forward" extra button.
    Forward,
}

impl Button {
    /// Every button, in the order a drag start is resolved when several go down in one frame.
    pub const ALL: [Button; 5] = [
        Button::Primary,
        Button::Middle,
        Button::Secondary,
        Button::Back,
        Button::Forward,
    ];

    const fn bit(self) -> u8 {
        1 << (self as u8)
    }
}

/// A set of [`Button`]s (one bit each).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ButtonSet(u8);

impl ButtonSet {
    /// No button.
    pub const EMPTY: ButtonSet = ButtonSet(0);

    /// This set plus `button` (builder form).
    #[must_use]
    pub const fn with(self, button: Button) -> ButtonSet {
        ButtonSet(self.0 | button.bit())
    }

    /// Add `button`.
    pub fn insert(&mut self, button: Button) {
        self.0 |= button.bit();
    }

    /// Remove `button`.
    pub fn remove(&mut self, button: Button) {
        self.0 &= !button.bit();
    }

    /// Whether `button` is in the set.
    pub const fn contains(self, button: Button) -> bool {
        self.0 & button.bit() != 0
    }

    /// Whether the set is empty.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// Held modifier keys.
///
/// `ctrl` is the PHYSICAL Control key on every OS (the camera-navigation snap / align key, the
/// documented exception to the egui-widgets-rs gesture contract). `command` is the platform
/// command key: Control on Windows / Linux, Cmd on macOS (egui `Modifiers::command`); it picks the
/// opposite view of the view keys and Blender's dolly.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Mods {
    /// Physical Control.
    pub ctrl: bool,
    /// Alt / Option.
    pub alt: bool,
    /// Shift.
    pub shift: bool,
    /// Platform command (Control on Windows / Linux, Cmd on macOS).
    pub command: bool,
}

impl Mods {
    /// Nothing held.
    pub const NONE: Mods = Mods {
        ctrl: false,
        alt: false,
        shift: false,
        command: false,
    };
    /// Physical Control only.
    pub const CTRL: Mods = Mods {
        ctrl: true,
        ..Mods::NONE
    };
    /// Alt only.
    pub const ALT: Mods = Mods {
        alt: true,
        ..Mods::NONE
    };
    /// Shift only.
    pub const SHIFT: Mods = Mods {
        shift: true,
        ..Mods::NONE
    };
    /// Platform command only.
    pub const COMMAND: Mods = Mods {
        command: true,
        ..Mods::NONE
    };

    /// The union of both sets.
    #[must_use]
    pub const fn plus(self, other: Mods) -> Mods {
        Mods {
            ctrl: self.ctrl || other.ctrl,
            alt: self.alt || other.alt,
            shift: self.shift || other.shift,
            command: self.command || other.command,
        }
    }

    /// Whether every modifier of `required` is held in `self` (others may be held too).
    pub const fn contains(self, required: Mods) -> bool {
        (!required.ctrl || self.ctrl)
            && (!required.alt || self.alt)
            && (!required.shift || self.shift)
            && (!required.command || self.command)
    }
}

/// The keys camera navigation reads. Letters and digits are PHYSICAL positions (US layout
/// names; the top-row digits and the numpad digits both map to `NumN` in the adapters).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Key {
    /// A.
    A,
    /// B.
    B,
    /// C.
    C,
    /// D.
    D,
    /// E.
    E,
    /// F.
    F,
    /// G.
    G,
    /// H.
    H,
    /// I.
    I,
    /// J.
    J,
    /// K.
    K,
    /// L.
    L,
    /// M.
    M,
    /// N.
    N,
    /// O.
    O,
    /// P.
    P,
    /// Q.
    Q,
    /// R.
    R,
    /// S.
    S,
    /// T.
    T,
    /// U.
    U,
    /// V.
    V,
    /// W.
    W,
    /// X.
    X,
    /// Y.
    Y,
    /// Z.
    Z,
    /// Digit 0 (top row or numpad).
    Num0,
    /// Digit 1 (top row or numpad).
    Num1,
    /// Digit 2 (top row or numpad).
    Num2,
    /// Digit 3 (top row or numpad).
    Num3,
    /// Digit 4 (top row or numpad).
    Num4,
    /// Digit 5 (top row or numpad).
    Num5,
    /// Digit 6 (top row or numpad).
    Num6,
    /// Digit 7 (top row or numpad).
    Num7,
    /// Digit 8 (top row or numpad).
    Num8,
    /// Digit 9 (top row or numpad).
    Num9,
    /// Space bar.
    Space,
    /// Home.
    Home,
    /// Escape.
    Escape,
    /// Left arrow.
    ArrowLeft,
    /// Right arrow.
    ArrowRight,
    /// Up arrow.
    ArrowUp,
    /// Down arrow.
    ArrowDown,
}

impl Key {
    const fn bit(self) -> u64 {
        1 << (self as u8)
    }
}

/// A set of [`Key`]s (one bit each; the enum has fewer than 64 variants).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct KeySet(u64);

impl KeySet {
    /// No key.
    pub const EMPTY: KeySet = KeySet(0);

    /// This set plus `key` (builder form).
    #[must_use]
    pub const fn with(self, key: Key) -> KeySet {
        KeySet(self.0 | key.bit())
    }

    /// Add `key`.
    pub fn insert(&mut self, key: Key) {
        self.0 |= key.bit();
    }

    /// Remove `key`.
    pub fn remove(&mut self, key: Key) {
        self.0 &= !key.bit();
    }

    /// Whether `key` is in the set.
    pub const fn contains(self, key: Key) -> bool {
        self.0 & key.bit() != 0
    }

    /// Whether the set is empty.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// One frame's wheel input. `+y` = wheel turned away from the user (scroll up: zoom in), `+x` =
/// scroll right, as egui and winit report it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Wheel {
    /// Notched wheel steps (lines). One notch = 1.0.
    pub lines: Vec2,
    /// Smooth scroll in viewport units (points): trackpads and smooth-scrolling wheels.
    pub points: Vec2,
}

/// Smooth-scroll viewport units that count as one wheel notch: egui's native line-scroll speed,
/// so a notch read as lines (winit) and the same notch read as points (egui) zoom alike. THE one
/// conversion of the wheel zoom and the fly-speed wheel ([`Wheel::notches`]).
pub const POINTS_PER_NOTCH: f32 = 40.0;

impl Wheel {
    /// The vertical scroll in notch equivalents: `lines.y + points.y / POINTS_PER_NOTCH`.
    pub fn notches(self) -> f32 {
        self.lines.y + self.points.y / POINTS_PER_NOTCH
    }
}

/// Smallest scale divisor [`RawMotionUnits::to_points`] uses (guards a zero / negative / NaN
/// scale from the host).
const MIN_MOTION_SCALE: f32 = 1e-3;

/// The unit of the host's raw relative mouse motion (winit `DeviceEvent::MouseMotion`, egui
/// `PointerState::motion`), which winit documents as "unspecified" and which differs per backend
/// (winit 0.30.13 sources). THE one rule for both adapters (`camera-winit`, `camera-egui`).
///
/// | Backend | Source | Unit | Variant |
/// |---|---|---|---|
/// | Windows | raw-input mouse counts | device counts ~ physical pixels | `Physical` |
/// | X11 | XInput2 raw deltas | device counts ~ physical pixels | `Physical` |
/// | macOS | `NSEvent.deltaX/Y` | Cocoa points | `Logical` |
/// | Wayland | `zwp_relative_pointer_v1` | "same dimension as `wl_pointer` motion" = surface-local logical | `Logical` |
///
/// Windows / X11 / macOS are the compile-time default ([`RawMotionUnits::PLATFORM_DEFAULT`]).
/// Wayland and X11 are both `target_os = "linux"`, so only the host can tell them apart at run
/// time: on Wayland (winit `ActiveEventLoopExtWayland::is_wayland`) it sets `Logical` through
/// `camera_winit::InputCollector::set_raw_motion_units` / `camera_egui::set_raw_motion_units`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RawMotionUnits {
    /// Device counts ~ physical pixels (Windows, X11): divided by the pixels per point.
    Physical,
    /// Already logical points of the native window (macOS, Wayland): divided only by the host's
    /// own extra zoom (egui `zoom_factor`; 1 for a plain winit host).
    Logical,
}

impl RawMotionUnits {
    /// The compile-time default: `Logical` on macOS, else `Physical` (a Wayland host overrides it
    /// at run time).
    pub const PLATFORM_DEFAULT: RawMotionUnits = if cfg!(target_os = "macos") {
        RawMotionUnits::Logical
    } else {
        RawMotionUnits::Physical
    };

    /// One raw motion delta in the host's logical points. `Physical` divides by
    /// `pixels_per_point` (physical pixels per host point); `Logical` divides by `zoom` (host
    /// points are native points / zoom: egui's `zoom_factor`, 1.0 for a host without one).
    /// Scales below 1e-3 (zero, negative, NaN) count as 1e-3, so the result is never non-finite
    /// for a finite delta.
    pub fn to_points(self, delta: Vec2, pixels_per_point: f32, zoom: f32) -> Vec2 {
        let scale = match self {
            RawMotionUnits::Physical => pixels_per_point,
            RawMotionUnits::Logical => zoom,
        };
        delta / scale.max(MIN_MOTION_SCALE)
    }
}

/// One frame of host input, in viewport units (the unit of the `ViewportSize` the host passes to
/// `CameraController::apply_intent`; the egui adapter uses logical points).
///
/// While the host honours `CursorRequest::Locked` the pointer position stops moving: the adapter
/// then fills `pointer_delta` from the raw device motion (converted to the same units).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawInput {
    /// Pointer position relative to the viewport's top-left corner; `None` when unknown (the
    /// pointer left the window).
    pub pointer: Option<Vec2>,
    /// Pointer motion this frame.
    pub pointer_delta: Vec2,
    /// Buttons held at the end of the frame.
    pub buttons_down: ButtonSet,
    /// Buttons that went down this frame.
    pub buttons_pressed: ButtonSet,
    /// Buttons that went up this frame.
    pub buttons_released: ButtonSet,
    /// Held modifiers.
    pub mods: Mods,
    /// Keys held at the end of the frame.
    pub keys_down: KeySet,
    /// Keys that went down this frame (no auto-repeat).
    pub keys_pressed: KeySet,
    /// Wheel input this frame.
    pub wheel: Wheel,
    /// Multiplicative pinch zoom this frame (the product of the host's pinch / zoom-gesture
    /// events: egui `Event::Zoom`, winit `PinchGesture`); 1.0 = none, > 1 magnifies. A Ctrl+wheel
    /// notch is NOT a pinch: it is a plain wheel notch ([`Wheel`]).
    pub pinch: f32,
    /// The host's typing guard: `false` while a text field owns the keyboard, a modal is open or
    /// the viewport is not the keyboard target. Every key rule (and a key-held drag binding)
    /// needs it.
    pub keys_allowed: bool,
    /// The pointer is over the viewport (a drag may only START over it; a running drag keeps
    /// going off it).
    pub hovered: bool,
    /// Seconds since the previous frame.
    pub dt: f32,
    /// Viewport size.
    pub viewport: Vec2,
}

impl Default for RawInput {
    /// No input: pointer unknown, nothing held, `pinch` 1.0, `dt` 0.
    fn default() -> Self {
        Self {
            pointer: None,
            pointer_delta: Vec2::ZERO,
            buttons_down: ButtonSet::EMPTY,
            buttons_pressed: ButtonSet::EMPTY,
            buttons_released: ButtonSet::EMPTY,
            mods: Mods::NONE,
            keys_down: KeySet::EMPTY,
            keys_pressed: KeySet::EMPTY,
            wheel: Wheel::default(),
            pinch: 1.0,
            keys_allowed: false,
            hovered: false,
            dt: 0.0,
            viewport: Vec2::ZERO,
        }
    }
}

/// `v` when both components are finite, else zero.
fn finite_or_zero(v: Vec2) -> Vec2 {
    if v.is_finite() { v } else { Vec2::ZERO }
}

impl RawInput {
    /// A copy with every non-finite host value dropped: an unknown pointer, zero motion / wheel /
    /// viewport, `pinch` 1.0 (also for `<= 0`), `dt` 0 (also for `< 0`). The navigation rules
    /// only ever see the sanitized copy.
    #[must_use]
    pub fn sanitized(&self) -> RawInput {
        RawInput {
            pointer: self.pointer.filter(|p| p.is_finite()),
            pointer_delta: finite_or_zero(self.pointer_delta),
            wheel: Wheel {
                lines: finite_or_zero(self.wheel.lines),
                points: finite_or_zero(self.wheel.points),
            },
            pinch: if self.pinch.is_finite() && self.pinch > 0.0 {
                self.pinch
            } else {
                1.0
            },
            dt: if self.dt.is_finite() && self.dt > 0.0 {
                self.dt
            } else {
                0.0
            },
            viewport: finite_or_zero(self.viewport).max(Vec2::ZERO),
            ..*self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_hold_every_member_independently() {
        let mut keys = KeySet::EMPTY;
        for k in [Key::A, Key::Z, Key::Num0, Key::Num9, Key::ArrowDown] {
            assert!(!keys.contains(k));
            keys.insert(k);
            assert!(keys.contains(k));
        }
        keys.remove(Key::Z);
        assert!(!keys.contains(Key::Z) && keys.contains(Key::ArrowDown));
        let mut b = ButtonSet::EMPTY.with(Button::Forward);
        assert!(b.contains(Button::Forward) && !b.contains(Button::Primary));
        b.remove(Button::Forward);
        assert!(b.is_empty());
    }

    #[test]
    fn raw_motion_units_rule() {
        let d = Vec2::new(8.0, -4.0);
        assert_eq!(
            RawMotionUnits::Physical.to_points(d, 2.0, 1.5),
            Vec2::new(4.0, -2.0)
        );
        assert_eq!(
            RawMotionUnits::Logical.to_points(d, 2.0, 2.0),
            Vec2::new(4.0, -2.0),
            "logical motion is divided by the zoom, not by pixels per point"
        );
        assert_eq!(RawMotionUnits::Logical.to_points(d, 2.0, 1.0), d);
        assert_eq!(
            RawMotionUnits::PLATFORM_DEFAULT == RawMotionUnits::Logical,
            cfg!(target_os = "macos")
        );
        for bad in [0.0, -1.0, f32::NAN] {
            assert!(RawMotionUnits::Physical.to_points(d, bad, 1.0).is_finite());
            assert!(RawMotionUnits::Logical.to_points(d, 1.0, bad).is_finite());
        }
    }

    #[test]
    fn mods_contains_is_a_subset_test() {
        let ca = Mods::CTRL.plus(Mods::ALT);
        assert!(ca.contains(Mods::ALT));
        assert!(ca.contains(Mods::NONE));
        assert!(!Mods::ALT.contains(ca));
    }

    #[test]
    fn sanitized_drops_every_non_finite_value() {
        let raw = RawInput {
            pointer: Some(Vec2::new(f32::NAN, 1.0)),
            pointer_delta: Vec2::new(f32::INFINITY, 0.0),
            wheel: Wheel {
                lines: Vec2::new(0.0, f32::NAN),
                points: Vec2::splat(f32::NEG_INFINITY),
            },
            pinch: 0.0,
            dt: f32::NAN,
            viewport: Vec2::new(-5.0, f32::NAN),
            ..RawInput::default()
        }
        .sanitized();
        assert_eq!(raw.pointer, None);
        assert_eq!(raw.pointer_delta, Vec2::ZERO);
        assert_eq!(raw.wheel, Wheel::default());
        assert_eq!(raw.pinch, 1.0);
        assert_eq!(raw.dt, 0.0);
        assert_eq!(raw.viewport, Vec2::ZERO);
    }
}
