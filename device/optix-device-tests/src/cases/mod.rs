//! The test cases, one module each; [`ALL`] lists them in run order.
//!
//! Each case builds its own pipeline over the shared module (all programs of the binary), runs
//! it and compares every output value with a CPU reference. **Used by** `main`.

pub mod calls;
pub mod common;
pub mod const_index;
pub mod coopvec;
pub mod exception;
pub mod footprint;
pub mod geometry;
pub mod hitobject;
pub mod intersect;
pub mod local;
pub mod shade;
pub mod symbols;
pub mod texture;
pub mod transform;

use common::{Gpu, R};

/// A case: name and entry point.
pub type Case = (&'static str, fn(&Gpu) -> R);

/// Every case, in run order (`ptr64` first: it only scans the PTX).
pub const ALL: &[Case] = &[
    ("ptr64", local::run),
    ("shade", shade::run),
    ("geometry", geometry::run),
    ("transform", transform::run),
    ("hitobject", hitobject::run),
    ("intersect", intersect::run),
    ("exception", exception::run),
    ("callables", calls::callables),
    ("payload_types", calls::payload_types),
    ("symbols", symbols::run),
    ("const_index", const_index::run),
    ("coop_vec", coopvec::run),
    ("texture", texture::run),
    ("sparse", texture::sparse),
    ("footprint", footprint::run),
];
