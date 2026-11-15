//! Coordinate conventions (documented SSOT for all rigs).
//!
//! Why: one place names the world up axis and the default orbit offset axis. How: plain `const`s.
//! Used by the rigs, orientation and the controller.

use glam::Vec3;

/// World up axis (right-handed, Y-up).
pub const WORLD_UP: Vec3 = Vec3::Y;

/// Default orbit offset axis before orientation is applied (+X in pivot space).
pub const DEFAULT_OFFSET_AXIS: Vec3 = Vec3::X;

/// Default orbit distance of a fresh rig (the default `OrbitPose`, the seed of a free-pose
/// backend switched to an orbit rig, the idle transition target): one value for every place that
/// needs a starting distance.
pub(crate) const DEFAULT_ORBIT_DISTANCE: f32 = 28.0;
