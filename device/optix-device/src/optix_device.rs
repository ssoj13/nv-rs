//! OptiX device API for Rust programs compiled to PTX by cuda-oxide.
//!
//! **What.** [`raw`] is one `unsafe fn` per `_optix_*` intrinsic of the installed OptiX SDK,
//! generated at build time from its device headers (`nv_bindgen::device_asm`): each issues the
//! header's own inline PTX, so OptiX sees exactly what `nvcc` would emit. The build fails if the
//! headers contain an intrinsic without a wrapper. [`sys`] holds the SDK's types and constants
//! (generated, `no_std`). The typed layer on top is written by hand:
//! - [`state`] — launch index/size, ray, hit (primitive, instance, SBT, hit kind, barycentrics,
//!   attributes, SBT data), report / ignore / terminate;
//! - [`payload`] — typed [`Payload`]s in payload registers;
//! - [`trace`] — [`trace::trace`] with a [`trace::Ray`] and [`trace::TraceOptions`];
//! - [`exception`] — user exceptions, codes, details, line info.
//!
//! Every function is `unsafe`: OptiX restricts each intrinsic to certain program types, which the
//! type system cannot see; each documents where it is valid.
//! **Used by** OptiX programs (`__raygen__*`, `__miss__*`, `__closesthit__*`, ...) written with
//! `cuda_device::kernel`; the host side is the `optix` crate.

#![no_std]

pub mod exception;
pub mod payload;
pub mod state;
pub mod trace;

pub use payload::Payload;

/// A 3-vector (`float3` in the OptiX headers).
pub type Float3 = [f32; 3];

/// One wrapper per `_optix_*` intrinsic, named without the `_optix_` prefix (generated).
#[allow(missing_docs, clippy::all)]
pub mod raw {
    include!(concat!(env!("OUT_DIR"), "/optix_raw.rs"));
}

/// Intrinsic names and header hashes the raw layer was generated from (coverage evidence).
pub const INVENTORY: &str = include_str!(concat!(env!("OUT_DIR"), "/optix_device_inventory.json"));

/// OptiX types and constants (`optix_types.h`, generated, `no_std`): transform structs, enums,
/// flags — the same definitions the host `optix-sys` crate uses.
#[allow(missing_docs, non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code, clippy::all, clippy::undocumented_unsafe_blocks)]
pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/optix_types.rs"));
}
