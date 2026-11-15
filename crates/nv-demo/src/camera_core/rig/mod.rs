//! Camera rigs: the Y-up turntable orbit ([`OrbitPose`] / [`OrbitRig`]); the free-pose
//! controllers live in `crate::camera_core::controller`. The release coasts of every orbit-style rig are
//! `crate::camera_core::coast::OrbitCoast` (one model, no rig-local tuning constants).

mod orbit;

pub use orbit::{OrbitPose, OrbitRig};
