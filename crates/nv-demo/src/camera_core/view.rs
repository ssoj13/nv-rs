//! View targets: THE one type for "look at the scene from this side".
//!
//! - **What:** [`ViewTarget`] is one of the 26 views of a cube around the pivot — 6 faces (one
//!   signed world axis), 12 edges (two), 8 corners (three). It names the side the CAMERA sits on,
//!   looking back at the pivot. [`Face`] gives the six faces their names under an [`UpAxis`]
//!   (Y-up: Maya / Houdini / USD; Z-up: Blender / Max / CAD).
//! - **Why:** a view cube's clicks, the numpad view keys and every rig must agree on one target
//!   type, so any view request reaches any rig.
//! - **How:** [`ViewTarget::frame`] turns a target into the camera's `(forward, up)` WITHOUT a
//!   flip: off the poles the screen up is the world up (upright, Autodesk "avoid upside-down");
//!   looking straight along the up axis the current up is carried by the shortest rotation from
//!   the current forward, so the heading is kept (no 180° spin). [`orbit_about`] / [`roll_about`]
//!   move a free [`CameraPose`] about a pivot (the nav cube drag and roll arrows).
//! - **Used by:** [`crate::camera_core::CameraIntent::View`] (every backend of [`crate::camera_core::CameraController`]),
//!   [`crate::camera_core::OrbitRig::animate_to_view`], host view cubes (26 cells map 1:1 through
//!   [`ViewTarget::axes`]) and view keys (1 / 3 / 7).

use std::f32::consts::PI;

use glam::{Quat, Vec3};

use crate::camera_core::num::clamp;
use crate::camera_core::pose::CameraPose;

/// Angle (radians) within which the camera "is at" a preset view ([`ViewTarget::matching`]): the
/// nav cube's highlight and current-view label, its second-click-is-the-opposite rule.
pub const VIEW_MATCH_RAD: f32 = 0.5_f32.to_radians();

/// Squared length below which a projected up vector is taken as degenerate (the view looks
/// along it).
const DEGENERATE2: f32 = 1e-6;
/// A forward change whose dot is below this is a half turn: `Quat::from_rotation_arc` picks an
/// arbitrary axis there, so the half turn is taken about the current screen right instead.
const HALF_TURN_DOT: f32 = -1.0 + 1e-6;

/// Which world axis points up on screen in an upright view.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UpAxis {
    /// +Y up, Front = camera on +Z (Maya, Houdini, USD; the crate's [`crate::camera_core::WORLD_UP`]).
    #[default]
    Y,
    /// +Z up, Front = camera on −Y (Blender, Max, CAD).
    Z,
}

impl UpAxis {
    /// The world up vector.
    pub const fn up(self) -> Vec3 {
        match self {
            UpAxis::Y => Vec3::Y,
            UpAxis::Z => Vec3::Z,
        }
    }

    /// The camera side of the Front view (Y-up: +Z, Z-up: −Y).
    pub const fn front(self) -> Vec3 {
        match self {
            UpAxis::Y => Vec3::Z,
            UpAxis::Z => Vec3::NEG_Y,
        }
    }
}

/// A named face view. Right is +X in both conventions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Face {
    /// Front (numpad 1).
    Front,
    /// Back (Ctrl/Cmd + numpad 1).
    Back,
    /// Right, +X (numpad 3).
    Right,
    /// Left, −X (Ctrl/Cmd + numpad 3).
    Left,
    /// Top, +up (numpad 7).
    Top,
    /// Bottom, −up (Ctrl/Cmd + numpad 7).
    Bottom,
}

impl Face {
    /// Every face.
    pub const ALL: [Face; 6] = [
        Face::Front,
        Face::Back,
        Face::Right,
        Face::Left,
        Face::Top,
        Face::Bottom,
    ];

    /// The face across the cube.
    pub const fn opposite(self) -> Face {
        match self {
            Face::Front => Face::Back,
            Face::Back => Face::Front,
            Face::Right => Face::Left,
            Face::Left => Face::Right,
            Face::Top => Face::Bottom,
            Face::Bottom => Face::Top,
        }
    }

    /// The unit direction from the pivot toward the camera of this face under `up`.
    pub fn dir(self, up: UpAxis) -> Vec3 {
        match self {
            Face::Front => up.front(),
            Face::Back => -up.front(),
            Face::Right => Vec3::X,
            Face::Left => Vec3::NEG_X,
            Face::Top => up.up(),
            Face::Bottom => -up.up(),
        }
    }
}

/// One of the 26 views of a cube around the pivot: a signed component (−1, 0 or 1) per world
/// axis, not all zero; the camera sits on that side looking at the pivot. The field is private,
/// so a target is always one of the 26 (build with [`ViewTarget::new`] / [`ViewTarget::face`]).
///
/// With the `serde` feature it is the array `[x, y, z]`; deserializing validates through
/// [`ViewTarget::new`], so an invalid array (a zero or out-of-range component) is an error, never
/// a ViewTarget outside the 26.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(try_from = "[i8; 3]", into = "[i8; 3]")
)]
pub struct ViewTarget([i8; 3]);

impl TryFrom<[i8; 3]> for ViewTarget {
    type Error = &'static str;

    /// The validated conversion ([`ViewTarget::new`]): each component −1, 0 or 1, not all zero.
    fn try_from([x, y, z]: [i8; 3]) -> Result<Self, Self::Error> {
        ViewTarget::new(x, y, z).ok_or("a view target needs components in -1..=1, not all zero")
    }
}

impl From<ViewTarget> for [i8; 3] {
    fn from(target: ViewTarget) -> Self {
        target.0
    }
}

impl ViewTarget {
    /// The target with these components; `None` unless each is −1, 0 or 1 and one is not 0.
    pub const fn new(x: i8, y: i8, z: i8) -> Option<ViewTarget> {
        let unit = matches!(x, -1..=1) && matches!(y, -1..=1) && matches!(z, -1..=1);
        if unit && (x != 0 || y != 0 || z != 0) {
            Some(ViewTarget([x, y, z]))
        } else {
            None
        }
    }

    /// The face view `face` under the `up` convention.
    pub fn face(face: Face, up: UpAxis) -> ViewTarget {
        let d = face.dir(up);
        // `d` is a signed unit axis: its components are exactly −1, 0 or 1.
        ViewTarget([d.x as i8, d.y as i8, d.z as i8])
    }

    /// All 26 targets in a fixed order.
    pub fn all() -> impl Iterator<Item = ViewTarget> {
        (0i8..27).filter_map(|i| ViewTarget::new(i % 3 - 1, (i / 3) % 3 - 1, i / 9 - 1))
    }

    /// The components `[x, y, z]` (the nav cube builds its `Target` from them).
    pub const fn axes(self) -> [i8; 3] {
        self.0
    }

    /// The components as a vector (not normalized).
    fn raw(self) -> Vec3 {
        let [x, y, z] = self.0;
        Vec3::new(f32::from(x), f32::from(y), f32::from(z))
    }

    /// The unit direction from the pivot toward the camera.
    pub fn dir(self) -> Vec3 {
        self.raw().normalize()
    }

    /// The view from the other side.
    pub fn opposite(self) -> ViewTarget {
        let [x, y, z] = self.0;
        ViewTarget([-x, -y, -z])
    }

    /// One signed axis only (a face view).
    pub fn is_face(self) -> bool {
        self.0.iter().filter(|c| **c != 0).count() == 1
    }

    /// The faces this target lies on, named under `up`, vertical first: `[Top/Bottom,
    /// Front/Back, Right/Left]` (`None` where the target has no component). A face view has one,
    /// an edge two, a corner three: the parts of its label ("Top Front Right").
    pub fn faces(self, up: UpAxis) -> [Option<Face>; 3] {
        let v = self.raw();
        let part = |(pos, neg): (Face, Face)| {
            // Components are integral, so the dot is exact: 1, 0 or −1.
            let c = v.dot(pos.dir(up));
            if c > 0.5 {
                Some(pos)
            } else if c < -0.5 {
                Some(neg)
            } else {
                None
            }
        };
        [
            part((Face::Top, Face::Bottom)),
            part((Face::Front, Face::Back)),
            part((Face::Right, Face::Left)),
        ]
    }

    /// The target whose direction is nearest `camera_side` (the direction from the pivot toward
    /// the camera). `None` for a zero or non-finite direction.
    pub fn nearest(camera_side: Vec3) -> Option<ViewTarget> {
        let s = camera_side.normalize_or_zero();
        if s == Vec3::ZERO || !s.is_finite() {
            return None;
        }
        ViewTarget::all().max_by(|a, b| a.dir().dot(s).total_cmp(&b.dir().dot(s)))
    }

    /// The target the camera is at: the [`ViewTarget::nearest`] one when it lies within
    /// `tol_rad` of `camera_side` ([`VIEW_MATCH_RAD`] is the shared tolerance).
    pub fn matching(camera_side: Vec3, tol_rad: f32) -> Option<ViewTarget> {
        let s = camera_side.normalize_or_zero();
        let t = ViewTarget::nearest(s)?;
        (t.dir().dot(s) >= tol_rad.cos()).then_some(t)
    }

    /// The camera `(forward, up)` of this view, coming from a camera with `from_forward` /
    /// `from_up`, under the `up` convention. No flip:
    /// - off the poles, screen up is the world up projected on the view plane (upright);
    /// - looking straight along ±world up (Top / Bottom), the current up carried by the shortest
    ///   rotation from the current forward to the new one, so the heading is kept; a half turn
    ///   (the opposite view) turns about the current screen right;
    /// - if that is degenerate too (a damaged pose, or the current up along the new forward), the
    ///   convention's default: Top shows Back up, Bottom shows Front up.
    ///
    /// Both returned vectors are unit and orthogonal.
    pub fn frame(self, up: UpAxis, from_forward: Vec3, from_up: Vec3) -> (Vec3, Vec3) {
        let f = -self.dir();
        let flat = |v: Vec3| {
            let p = v - f * v.dot(f);
            (p.is_finite() && p.length_squared() > DEGENERATE2).then(|| p.normalize())
        };
        if let Some(u) = flat(up.up()) {
            return (f, u);
        }
        let carried = carry_up(from_forward, from_up, f).and_then(flat);
        // Top looks down −up: Back at the top of the screen; Bottom: Front.
        let default = if f.dot(up.up()) < 0.0 {
            -up.front()
        } else {
            up.front()
        };
        (f, carried.unwrap_or(default))
    }

    /// The pose of this view about `pivot`: the camera at `from`'s distance to `pivot` (at least
    /// on the pivot), oriented by [`ViewTarget::frame`] from `from`, `from`'s lens kept.
    pub fn pose(self, up: UpAxis, from: &CameraPose, pivot: Vec3) -> CameraPose {
        let dist = (from.eye - pivot).length();
        let dist = if dist.is_finite() { dist } else { 0.0 };
        let (f, u) = self.frame(up, from.forward(), from.up());
        CameraPose::from_eye_forward_up(pivot - f * dist, f, u, from.fov_y, from.znear, from.zfar)
    }
}

/// `from_up` carried by the shortest rotation taking `from_forward` to `to` (a half turn about
/// the current screen right). `None` for a damaged input.
fn carry_up(from_forward: Vec3, from_up: Vec3, to: Vec3) -> Option<Vec3> {
    let f0 = from_forward.normalize_or_zero();
    let u0 = from_up.normalize_or_zero();
    if f0 == Vec3::ZERO || u0 == Vec3::ZERO || !f0.is_finite() || !u0.is_finite() {
        return None;
    }
    let q = if f0.dot(to) < HALF_TURN_DOT {
        let right = f0.cross(u0).normalize_or_zero();
        if right == Vec3::ZERO {
            return None;
        }
        Quat::from_axis_angle(right, PI)
    } else {
        Quat::from_rotation_arc(f0, to)
    };
    Some(q * u0)
}

/// Orbit the free `pose` about `pivot`: `yaw` radians about the world up of `up` (positive =
/// counter-clockwise seen from above), then `pitch` radians of elevation (positive = the camera
/// rises toward the up axis), about the camera's own screen right. The elevation stays within
/// `±pitch_limit` ([`crate::camera_core::DEFAULT_PITCH_LIMIT`]); a pose already beyond it (a snapped Top or
/// Bottom view at exactly ±90°) may only move back toward the horizon, so the first orbit after a
/// snap never jumps. Works at the exact pole (the screen right comes from the camera
/// basis, not from a cross product with the up axis). Non-finite deltas change nothing.
pub fn orbit_about(
    pose: &mut CameraPose,
    pivot: Vec3,
    up: UpAxis,
    yaw: f32,
    pitch: f32,
    pitch_limit: f32,
) {
    if !(yaw.is_finite() && pitch.is_finite()) {
        return;
    }
    let w = up.up();
    let side = -pose.forward();
    let el = clamp(side.dot(w), -1.0, 1.0).asin();
    // The one pitch-step rule of the orbit rigs: beyond the limit, only back; never over the pole.
    let applied = crate::camera_core::orientation::limit_pitch_step(el, pitch, pitch_limit);
    let right = pose.right();
    let tilt = if right == Vec3::ZERO || applied == 0.0 {
        Quat::IDENTITY
    } else {
        // A positive turn about the screen right raises the forward, i.e. lowers the camera.
        Quat::from_axis_angle(right, -applied)
    };
    let rot = Quat::from_axis_angle(w, yaw) * tilt;
    pose.orientation = (rot * pose.orientation).normalize();
    pose.eye = pivot + rot * (pose.eye - pivot);
}

/// Roll the free `pose` by `angle` radians about its viewing axis; positive turns the PICTURE
/// counter-clockwise (the camera turns clockwise). The eye stays (the axis runs through it and
/// a pivot on the view axis). A non-finite angle changes nothing.
pub fn roll_about(pose: &mut CameraPose, angle: f32) {
    let f = pose.forward();
    if !angle.is_finite() || f == Vec3::ZERO {
        return;
    }
    pose.orientation = (Quat::from_axis_angle(f, angle) * pose.orientation).normalize();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_PI_2;

    const EPS: f32 = 1e-5;

    fn pose_looking(forward: Vec3, up: Vec3) -> CameraPose {
        CameraPose::from_eye_forward_up(-forward * 10.0, forward, up, 0.8, 0.1, 100.0)
    }

    #[test]
    fn there_are_26_targets_and_opposites_pair_up() {
        let all: Vec<ViewTarget> = ViewTarget::all().collect();
        assert_eq!(all.len(), 26);
        assert_eq!(all.iter().filter(|t| t.is_face()).count(), 6);
        for t in &all {
            assert_ne!(t.opposite(), *t);
            assert_eq!(t.opposite().opposite(), *t);
            assert!((t.dir().length() - 1.0).abs() < EPS);
            assert_eq!(ViewTarget::nearest(t.dir()), Some(*t));
        }
        assert_eq!(ViewTarget::new(0, 0, 0), None);
        assert_eq!(ViewTarget::new(0, 2, 0), None);
    }

    /// Y-up: Front +Z, Top +Y; Z-up (Blender): Front −Y, Top +Z; Right is +X in both.
    #[test]
    fn faces_map_per_up_axis() {
        let y = |f| ViewTarget::face(f, UpAxis::Y).axes();
        let z = |f| ViewTarget::face(f, UpAxis::Z).axes();
        assert_eq!(y(Face::Front), [0, 0, 1]);
        assert_eq!(y(Face::Top), [0, 1, 0]);
        assert_eq!(y(Face::Right), [1, 0, 0]);
        assert_eq!(y(Face::Bottom), [0, -1, 0]);
        assert_eq!(z(Face::Front), [0, -1, 0]);
        assert_eq!(z(Face::Back), [0, 1, 0]);
        assert_eq!(z(Face::Top), [0, 0, 1]);
        assert_eq!(z(Face::Right), [1, 0, 0]);
        for up in [UpAxis::Y, UpAxis::Z] {
            for f in Face::ALL {
                let t = ViewTarget::face(f, up);
                assert!(t.is_face());
                assert_eq!(t.opposite(), ViewTarget::face(f.opposite(), up));
                assert_eq!(t.faces(up), {
                    let mut want = [None; 3];
                    let slot = match f {
                        Face::Top | Face::Bottom => 0,
                        Face::Front | Face::Back => 1,
                        Face::Right | Face::Left => 2,
                    };
                    if let Some(s) = want.get_mut(slot) {
                        *s = Some(f);
                    }
                    want
                });
            }
        }
        let Some(corner) = ViewTarget::new(1, 1, 1) else {
            panic!("a corner");
        };
        assert_eq!(
            corner.faces(UpAxis::Y),
            [Some(Face::Top), Some(Face::Front), Some(Face::Right)]
        );
        assert_eq!(
            corner.faces(UpAxis::Z),
            [Some(Face::Top), Some(Face::Back), Some(Face::Right)]
        );
    }

    /// Off the poles the view is upright: screen up has a positive world-up component and no
    /// roll (screen right is horizontal), in both conventions.
    #[test]
    fn side_edge_and_corner_views_are_upright() {
        for up in [UpAxis::Y, UpAxis::Z] {
            for t in ViewTarget::all() {
                let (f, u) = t.frame(up, Vec3::NEG_Z, Vec3::Y);
                assert!((f + t.dir()).length() < EPS, "{t:?} looks at the pivot");
                assert!((u.length() - 1.0).abs() < EPS && f.dot(u).abs() < EPS);
                if t.dir().dot(up.up()).abs() < 0.999 {
                    assert!(u.dot(up.up()) > 0.0, "{t:?} {up:?} upright");
                    assert!(f.cross(u).dot(up.up()).abs() < EPS, "{t:?} no roll");
                }
            }
        }
    }

    /// Top from Front (Y-up): the shortest rotation keeps the heading, Back at the screen top
    /// (Maya's top view), no spin; Bottom from Front: Front at the top.
    #[test]
    fn top_and_bottom_keep_the_heading() {
        let (f, u) = ViewTarget::face(Face::Top, UpAxis::Y).frame(UpAxis::Y, Vec3::NEG_Z, Vec3::Y);
        assert!((f - Vec3::NEG_Y).length() < EPS);
        assert!((u - Vec3::NEG_Z).length() < EPS, "{u}");
        let (_, u) =
            ViewTarget::face(Face::Bottom, UpAxis::Y).frame(UpAxis::Y, Vec3::NEG_Z, Vec3::Y);
        assert!((u - Vec3::Z).length() < EPS, "{u}");
        // From a side view facing +X (camera on −X): Top keeps that heading, +X at the top.
        let (_, u) = ViewTarget::face(Face::Top, UpAxis::Y).frame(UpAxis::Y, Vec3::X, Vec3::Y);
        assert!((u - Vec3::X).length() < EPS, "{u}");
        // Z-up from Front (camera on −Y looking +Y): Top shows +Y (Back) up, as Blender.
        let (f, u) = ViewTarget::face(Face::Top, UpAxis::Z).frame(UpAxis::Z, Vec3::Y, Vec3::Z);
        assert!((f - Vec3::NEG_Z).length() < EPS);
        assert!((u - Vec3::Y).length() < EPS, "{u}");
    }

    /// Degenerate up: already looking along the up axis (any heading), Top again and Bottom (a
    /// half turn) stay defined; a damaged pose falls back to the convention default.
    #[test]
    fn degenerate_up_is_defined() {
        let top = ViewTarget::face(Face::Top, UpAxis::Y);
        // Looking straight down with screen up +X.
        let (f, u) = top.frame(UpAxis::Y, Vec3::NEG_Y, Vec3::X);
        assert!((f - Vec3::NEG_Y).length() < EPS);
        assert!((u - Vec3::X).length() < EPS, "heading kept: {u}");
        let (f, u) = top.opposite().frame(UpAxis::Y, Vec3::NEG_Y, Vec3::X);
        assert!((f - Vec3::Y).length() < EPS);
        assert!(
            (u.length() - 1.0).abs() < EPS && f.dot(u).abs() < EPS,
            "{u}"
        );
        // Damaged pose: zero / NaN vectors -> the default (Top: Back up).
        let (_, u) = top.frame(UpAxis::Y, Vec3::ZERO, Vec3::NAN);
        assert!((u - Vec3::NEG_Z).length() < EPS);
        let (_, u) = top.opposite().frame(UpAxis::Y, Vec3::ZERO, Vec3::ZERO);
        assert!((u - Vec3::Z).length() < EPS);
        // Current up along the new forward (camera rolled 90° looking at the horizon).
        let (_, u) = top.frame(UpAxis::Y, Vec3::NEG_Z, Vec3::NEG_Y);
        assert!(u.is_finite() && (u.length() - 1.0).abs() < EPS);
    }

    #[test]
    fn matching_uses_the_tolerance() {
        let t = ViewTarget::face(Face::Front, UpAxis::Y);
        assert_eq!(ViewTarget::matching(Vec3::Z, VIEW_MATCH_RAD), Some(t));
        let off = Quat::from_rotation_y(VIEW_MATCH_RAD * 0.5) * Vec3::Z;
        assert_eq!(ViewTarget::matching(off, VIEW_MATCH_RAD), Some(t));
        let far = Quat::from_rotation_y(VIEW_MATCH_RAD * 2.0) * Vec3::Z;
        assert_eq!(ViewTarget::matching(far, VIEW_MATCH_RAD), None);
        assert_eq!(ViewTarget::matching(Vec3::NAN, VIEW_MATCH_RAD), None);
        assert_eq!(ViewTarget::nearest(Vec3::ZERO), None);
    }

    #[test]
    fn pose_keeps_the_distance_and_the_lens() {
        let from = pose_looking(Vec3::new(1.0, -0.5, -1.0).normalize(), Vec3::Y);
        let pivot = Vec3::new(1.0, 2.0, 3.0);
        let from = CameraPose {
            eye: pivot + Vec3::new(3.0, 4.0, 0.0),
            ..from
        };
        let Some(t) = ViewTarget::new(1, 1, 0) else {
            panic!("an edge");
        };
        let p = t.pose(UpAxis::Y, &from, pivot);
        assert!(((p.eye - pivot).length() - 5.0).abs() < 1e-4);
        assert!((p.forward() + t.dir()).length() < 1e-4);
        assert_eq!(
            (p.fov_y, p.znear, p.zfar),
            (from.fov_y, from.znear, from.zfar)
        );
    }

    /// From an exact Top view (elevation 90°, beyond the limit) a downward orbit moves by
    /// exactly the delta (no jump to the limit first), an upward one does not pass the pole.
    #[test]
    fn orbit_from_an_exact_pole_never_jumps() {
        let pivot = Vec3::ZERO;
        // Elevation of the camera side (a dot a hair past 1 would be a NaN asin).
        let elev = |p: &CameraPose| {
            crate::camera_core::num::clamp((-p.forward()).dot(Vec3::Y), -1.0, 1.0).asin()
        };
        let mut p = pose_looking(Vec3::NEG_Y, Vec3::NEG_Z);
        let lim = crate::camera_core::DEFAULT_PITCH_LIMIT;
        orbit_about(&mut p, pivot, UpAxis::Y, 0.0, -0.1, lim);
        let el = elev(&p);
        assert!((el - (FRAC_PI_2 - 0.1)).abs() < 1e-4, "{el}");
        assert!((p.eye.length() - 10.0).abs() < 1e-3, "distance kept");
        let mut q = pose_looking(Vec3::NEG_Y, Vec3::NEG_Z);
        orbit_about(&mut q, pivot, UpAxis::Y, 0.0, 0.3, lim);
        let el = elev(&q);
        assert!((FRAC_PI_2 - 1e-3..=FRAC_PI_2 + 1e-4).contains(&el), "{el}");
        // From the horizon, the limit holds.
        let mut h = pose_looking(Vec3::NEG_Z, Vec3::Y);
        orbit_about(&mut h, pivot, UpAxis::Y, 0.0, 3.0, lim);
        let el = elev(&h);
        assert!((el - lim).abs() < 1e-3, "{el}");
    }

    #[test]
    fn orbit_yaw_turns_about_the_up_axis_and_bad_input_is_ignored() {
        for up in [UpAxis::Y, UpAxis::Z] {
            let side = up.front();
            let mut p = pose_looking(-side, up.up());
            let before = p;
            orbit_about(&mut p, Vec3::ZERO, up, 0.5, 0.0, 1.0);
            let s = -p.forward();
            assert!(s.dot(up.up()).abs() < 1e-5, "stays level");
            assert!((s.dot(side) - 0.5_f32.cos()).abs() < 1e-4);
            let mut q = before;
            orbit_about(&mut q, Vec3::ZERO, up, f32::NAN, 0.0, 1.0);
            assert_eq!(q, before);
            orbit_about(&mut q, Vec3::ZERO, up, 0.0, 0.2, f32::NAN);
            assert!(q.orientation.is_finite() && q.eye.is_finite());
        }
    }

    #[test]
    fn roll_turns_the_picture_counter_clockwise() {
        let mut p = pose_looking(Vec3::NEG_Z, Vec3::Y);
        roll_about(&mut p, FRAC_PI_2);
        // Picture CCW = camera CW (seen by the viewer, who looks along −Z with +X on the right):
        // the camera's up leans over to world +X.
        assert!((p.up() - Vec3::X).length() < 1e-5, "{}", p.up());
        assert!((p.forward() - Vec3::NEG_Z).length() < 1e-5);
        let before = p;
        roll_about(&mut p, f32::INFINITY);
        assert_eq!(p, before);
    }

    /// The `[i8; 3]` conversion (the serde wire form) round-trips all 26 targets and rejects
    /// everything `ViewTarget::new` rejects.
    #[test]
    fn view_target_array_conversion_is_validated() {
        for t in ViewTarget::all() {
            let a: [i8; 3] = t.into();
            assert_eq!(ViewTarget::try_from(a), Ok(t));
        }
        for bad in [
            [0, 0, 0],
            [2, 0, 0],
            [0, -2, 1],
            [127, 0, 0],
            [0, 0, i8::MIN],
        ] {
            assert!(ViewTarget::try_from(bad).is_err(), "{bad:?}");
        }
    }
}
