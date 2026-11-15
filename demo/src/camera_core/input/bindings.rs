//! [`NavScheme`] presets and the editable [`Bindings`] table they produce.
//!
//! What: which button + modifiers (+ optionally a held key) starts which [`DragRole`]
//! ([`DragBinding`] rows, first match wins), the wheel behaviour, and the key bindings for
//! frame / home / view snaps / orbit steps ([`ViewKeys`], [`OrbitStepKeys`]) and for flying
//! ([`FlyKeys`]). The scheme table itself is in the `input` module docs.
//!
//! Why: navigation conventions differ per DCC (gesture contract P1 keeps each one); a host
//! picks a preset and may edit the table (drop a row, rebind a key) without touching the rules.
//!
//! How: plain data; [`NavScheme::bindings`] is the one place the presets are spelled.
//!
//! Used by: `super::navigator::Navigator::update`, hosts (scheme combo boxes, conflict checks
//! through [`FlyKeys::all`]).

use super::raw::{Button, Key, KeySet, Mods};
use crate::camera_core::UpAxis;

/// What a navigation drag does.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DragRole {
    /// Tumble about the pivot (`CameraIntent::Orbit` / `OrbitSnap`, release coast).
    Orbit,
    /// Pan the pivot in the view plane (`CameraIntent::Pan`, release coast).
    Pan,
    /// Dolly the distance with the vertical drag (`CameraIntent::Zoom`).
    Dolly,
    /// Dolly-zoom (vertigo) with the horizontal drag (`CameraIntent::DollyZoom`).
    DollyZoom,
    /// Hold to fly (Unreal style): mouse look + fly keys while the button is held.
    Fly,
}

/// One row of the drag table: `button` pressed with at least `mods` held (and `hold` held, when
/// set) starts `role`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DragBinding {
    /// The button whose press starts the drag (and whose release ends it).
    pub button: Button,
    /// Modifiers that must be held at the press (others may be held too: put the rows with
    /// more modifiers first).
    pub mods: Mods,
    /// A key that must be held at the press (Houdini's Space); it needs the typing guard.
    pub hold: Option<Key>,
    /// What the drag does.
    pub role: DragRole,
}

impl DragBinding {
    /// A row without a held key.
    pub const fn new(button: Button, mods: Mods, role: DragRole) -> DragBinding {
        DragBinding {
            button,
            mods,
            hold: None,
            role,
        }
    }

    /// The same row, also requiring `key` held.
    #[must_use]
    pub const fn holding(self, key: Key) -> DragBinding {
        DragBinding {
            hold: Some(key),
            ..self
        }
    }

    /// Whether a press of `button` with `mods` and `keys_down` held starts this row.
    /// `keys_allowed` is the host typing guard: a held-key requirement fails without it.
    pub fn matches(
        &self,
        button: Button,
        mods: Mods,
        keys_down: KeySet,
        keys_allowed: bool,
    ) -> bool {
        self.button == button
            && mods.contains(self.mods)
            && self
                .hold
                .is_none_or(|k| keys_allowed && keys_down.contains(k))
    }
}

/// The held fly keys (one source for the navigator and for host shortcut-conflict checks).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlyKeys {
    /// Thrust forward.
    pub forward: Key,
    /// Thrust back.
    pub back: Key,
    /// Strafe left.
    pub left: Key,
    /// Strafe right.
    pub right: Key,
    /// Thrust up.
    pub up: Key,
    /// Thrust down.
    pub down: Key,
    /// A second thrust-up key (Space); `None` where the host binds it itself.
    pub up_alt: Option<Key>,
    /// Roll left (counter-clockwise).
    pub roll_left: Key,
    /// Roll right (clockwise).
    pub roll_right: Key,
}

impl FlyKeys {
    /// W / S forward-back, A / D strafe, R / C up-down, Space up, Q / E roll.
    pub const WASD: FlyKeys = FlyKeys {
        forward: Key::W,
        back: Key::S,
        left: Key::A,
        right: Key::D,
        up: Key::R,
        down: Key::C,
        up_alt: Some(Key::Space),
        roll_left: Key::Q,
        roll_right: Key::E,
    };

    /// Every key the fly camera reads (`up_alt` only when set), for host conflict checks.
    pub fn all(&self) -> impl Iterator<Item = Key> + use<> {
        [
            Some(self.forward),
            Some(self.back),
            Some(self.left),
            Some(self.right),
            Some(self.up),
            Some(self.down),
            self.up_alt,
            Some(self.roll_left),
            Some(self.roll_right),
        ]
        .into_iter()
        .flatten()
    }
}

impl Default for FlyKeys {
    fn default() -> Self {
        Self::WASD
    }
}

/// Keys that snap to the front / right / top view under [`Bindings::up_axis`]; with the platform
/// command modifier they snap to the opposite face (back / left / bottom).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewKeys {
    /// Front view (Command: back).
    pub front: Key,
    /// Right view (Command: left).
    pub right: Key,
    /// Top view (Command: bottom).
    pub top: Key,
}

impl ViewKeys {
    /// Blender's numpad layout: 1 front, 3 right, 7 top.
    pub const NUMPAD: ViewKeys = ViewKeys {
        front: Key::Num1,
        right: Key::Num3,
        top: Key::Num7,
    };
}

/// Keys that orbit one [`super::ORBIT_STEP_RAD`] step, each as if the pointer were dragged that
/// way with an orbit drag.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrbitStepKeys {
    /// As a drag to the left.
    pub left: Key,
    /// As a drag to the right.
    pub right: Key,
    /// As a drag up.
    pub up: Key,
    /// As a drag down.
    pub down: Key,
}

impl OrbitStepKeys {
    /// Blender's numpad layout: 4 / 6 left / right, 8 / 2 up / down.
    pub const NUMPAD: OrbitStepKeys = OrbitStepKeys {
        left: Key::Num4,
        right: Key::Num6,
        up: Key::Num8,
        down: Key::Num2,
    };
}

/// The editable navigation table of one viewport (start from [`NavScheme::bindings`]).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bindings {
    /// Drag rows, first match wins.
    pub drags: Vec<DragBinding>,
    /// During an orbit drag: Shift = plane snap, physical Ctrl = smart snap, Ctrl+Shift = axis
    /// snap (`CameraIntent::OrbitSnap`; the camera-navigation modifier exception). Off where the
    /// host gives those modifiers another meaning mid-drag.
    pub orbit_snap: bool,
    /// The wheel / pinch zooms while hovered and not flying.
    pub wheel_zoom: bool,
    /// Smooth (points) scroll pans instead of zooming: the host classified it as a trackpad.
    pub smooth_scroll_pans: bool,
    /// While flying, the wheel scales the flight speed.
    pub fly_wheel_speed: bool,
    /// Frame the selection (`NavRequests::frame_selected`).
    pub frame_key: Option<Key>,
    /// Go home (`NavRequests::home`); two slots: Home plus a scheme key (Maya A, Houdini H).
    pub home_keys: [Option<Key>; 2],
    /// View snap keys, `None` where the scheme uses the digits for something else.
    pub view_keys: Option<ViewKeys>,
    /// Orbit step keys, `None` where the scheme uses the digits for something else.
    pub orbit_step_keys: Option<OrbitStepKeys>,
    /// Held fly keys.
    pub fly_keys: FlyKeys,
    /// Up convention that names the view-key faces.
    pub up_axis: UpAxis,
}

impl Bindings {
    /// The role a press of `button` starts with `mods` / `keys_down` held, if any (first
    /// matching row).
    pub fn role(
        &self,
        button: Button,
        mods: Mods,
        keys_down: KeySet,
        keys_allowed: bool,
    ) -> Option<DragRole> {
        self.drags
            .iter()
            .find(|d| d.matches(button, mods, keys_down, keys_allowed))
            .map(|d| d.role)
    }
}

impl Default for Bindings {
    /// [`NavScheme::Default`].
    fn default() -> Self {
        NavScheme::Default.bindings()
    }
}

impl From<NavScheme> for Bindings {
    fn from(scheme: NavScheme) -> Self {
        scheme.bindings()
    }
}

/// A navigation preset (see the scheme table in the `input` module docs).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum NavScheme {
    /// LMB orbit, MMB pan, RMB dolly, Ctrl+Alt+LMB dolly-zoom; digit view / step keys.
    ///
    /// Ctrl+Alt is AltGr on some keyboard layouts (German, Polish, ...): the host may rebind
    /// the dolly-zoom row (edit [`Bindings::drags`]) where that collides. The same holds for
    /// [`NavScheme::Unreal`].
    #[default]
    Default,
    /// Alt + LMB orbit / MMB pan / RMB dolly; plain clicks stay the host's. Home key A.
    Maya,
    /// Alt or Space + LMB orbit / MMB pan / RMB dolly. Home key H.
    Houdini,
    /// MMB orbit, Shift+MMB pan, Ctrl/Cmd+MMB dolly; digit view / step keys.
    Blender,
    /// RMB hold-to-fly (WASD), Alt+LMB orbit, Alt+RMB dolly, MMB / Alt+MMB pan, Ctrl+Alt+LMB
    /// dolly-zoom.
    Unreal,
}

impl NavScheme {
    /// Every scheme.
    pub const ALL: [NavScheme; 5] = [
        NavScheme::Default,
        NavScheme::Maya,
        NavScheme::Houdini,
        NavScheme::Blender,
        NavScheme::Unreal,
    ];

    /// The preset's editable table.
    pub fn bindings(self) -> Bindings {
        use Button::{Middle as M, Primary as L, Secondary as R};
        use DragRole::{Dolly, DollyZoom, Fly, Orbit, Pan};
        let ctrl_alt = Mods::CTRL.plus(Mods::ALT);
        let row = DragBinding::new;
        let (drags, home2, digits) = match self {
            NavScheme::Default => (
                vec![
                    row(L, ctrl_alt, DollyZoom),
                    row(L, Mods::NONE, Orbit),
                    row(M, Mods::NONE, Pan),
                    row(R, Mods::NONE, Dolly),
                ],
                None,
                true,
            ),
            NavScheme::Maya => (
                vec![
                    row(L, Mods::ALT, Orbit),
                    row(M, Mods::ALT, Pan),
                    row(R, Mods::ALT, Dolly),
                ],
                Some(Key::A),
                false,
            ),
            NavScheme::Houdini => (
                vec![
                    row(L, Mods::ALT, Orbit),
                    row(L, Mods::NONE, Orbit).holding(Key::Space),
                    row(M, Mods::ALT, Pan),
                    row(M, Mods::NONE, Pan).holding(Key::Space),
                    row(R, Mods::ALT, Dolly),
                    row(R, Mods::NONE, Dolly).holding(Key::Space),
                ],
                Some(Key::H),
                false,
            ),
            NavScheme::Blender => (
                vec![
                    row(M, Mods::SHIFT, Pan),
                    // "Ctrl/Cmd": the platform command key OR the physical Ctrl (on macOS
                    // they differ; on Windows / Linux the adapters set both for Ctrl).
                    row(M, Mods::COMMAND, Dolly),
                    row(M, Mods::CTRL, Dolly),
                    row(M, Mods::NONE, Orbit),
                ],
                None,
                true,
            ),
            NavScheme::Unreal => (
                vec![
                    row(L, ctrl_alt, DollyZoom),
                    row(L, Mods::ALT, Orbit),
                    row(M, Mods::NONE, Pan),
                    row(R, Mods::ALT, Dolly),
                    row(R, Mods::NONE, Fly),
                ],
                None,
                false,
            ),
        };
        Bindings {
            drags,
            orbit_snap: true,
            wheel_zoom: true,
            smooth_scroll_pans: false,
            fly_wheel_speed: true,
            frame_key: Some(Key::F),
            home_keys: [Some(Key::Home), home2],
            view_keys: digits.then_some(ViewKeys::NUMPAD),
            orbit_step_keys: digits.then_some(OrbitStepKeys::NUMPAD),
            fly_keys: FlyKeys::WASD,
            up_axis: UpAxis::Y,
        }
    }
}
