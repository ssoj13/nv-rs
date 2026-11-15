//! # camera-core — THE 3D camera navigation of the ecosystem
//!
//! What: every camera rig and mode (Houdini / Euler / horizon-plane turntable, quaternion orbit /
//! trackball, FPS fly, 6-DoF space flight, horizon-locked space flight, quaternion flight), their
//! inertia (sharp vs smooth turns), viewport pixel / NDC / ray math, fit and view snapping. Hosts
//! drive every rig through one intent enum (`CameraIntent`).
//!
//! Why: one implementation of camera navigation for every host with a 3D viewport (it replaces the
//! diverged copies those hosts used to carry): one crate, one codepath.
//!
//! How: pure Rust over glam (RH, Y-up by default), time-free (the caller passes `dt`), no UI or GPU
//! dependency, wasm-clean. Host adapters (`camera-egui`, `camera-winit`) only READ input.
//!
//! Uses: glam, serde (feature `serde`). Used by: `camera-egui`, `camera-winit`, the `camera`
//! umbrella and, through it, every app with a 3D viewport.

pub mod auto_orbit;
pub mod controller;
pub mod driver;
pub mod input;
pub mod intent;
pub mod lens;
pub mod mode;
pub mod pose;
pub mod prefs;
pub mod rig;
pub mod settings;
pub mod viewport;

pub mod coast;
mod convention;
mod dynamics;
mod fit;
mod num;
mod orientation;
mod projection;
mod view;

pub use auto_orbit::AutoOrbit;
pub use coast::{Coast, CoastSettings, CoastValue, OrbitCoast, ReleaseVelocity};
pub use controller::{CameraController, CameraKind, FpsFly, HoudiniOrbit, QuatOrbit, SpaceFlight};
pub use convention::{DEFAULT_OFFSET_AXIS, WORLD_UP};
pub use driver::{CameraDriver, DriverFrame};
pub use dynamics::{LOCK_POLE_LIMIT, TransitionState, rotate_view};
pub use fit::{aabb_center_radius, distance_to_frame_sphere};
pub use intent::{CameraIntent, OrbitSnapMode};
pub use lens::{FOV_MAX, FOV_MIN, LensEase, LensMode};
pub use mode::{NavFeel, NavMode, NavModeSpec};
pub use orientation::{
    DEFAULT_PITCH_LIMIT, OrientationMode, POLE_EPS, SMART_AXIS_SNAP_RAD, apply_orbit_delta,
    orientation_from_yaw_pitch, smart_snap_direction, smart_snap_yaw_pitch, snapped_yaw_pitch,
    yaw_pitch_from_orientation,
};
pub use pose::CameraPose;
pub use prefs::NavPrefs;
pub use projection::PerspectiveSettings;
pub use rig::{OrbitPose, OrbitRig};
pub use settings::{CameraNavigationSettings, InertiaSettings, MouseSteer, ZoomMode};
pub use view::{Face, UpAxis, VIEW_MATCH_RAD, ViewTarget, orbit_about, roll_about};
pub use viewport::{NdcPoint, ScreenRect, TextureUv, ViewportPoint, ViewportRay, ViewportSize};
