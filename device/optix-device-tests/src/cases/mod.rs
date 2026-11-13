//! The test cases, one module each; [`ALL`] lists them in run order.
//!
//! Each case builds its own pipeline over the shared module (all programs of the binary), runs
//! it and compares every output value with a CPU reference. **Used by** `main`.

pub mod common;
pub mod geometry;
pub mod hitobject;
pub mod local;
pub mod shade;
pub mod transform;

use common::{Gpu, R};

/// A case: name and entry point.
pub type Case = (&'static str, fn(&Gpu) -> R);

/// Every case, in run order (`no_local` first: it only scans the PTX).
pub const ALL: &[Case] = &[
    ("no_local", local::run),
    ("shade", shade::run),
    ("geometry", geometry::run),
    ("transform", transform::run),
    ("hitobject", hitobject::run),
];
