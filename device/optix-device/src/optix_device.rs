//! OptiX device API for Rust programs compiled to PTX by cuda-oxide.
//!
//! **What.** [`raw`] is one `unsafe fn` per `_optix_*` intrinsic of the installed OptiX SDK,
//! generated at build time from its device headers (`nv_bindgen::device_asm`): each issues the
//! header's own inline PTX, so OptiX sees exactly what `nvcc` would emit. The build fails if the
//! headers contain an intrinsic without a wrapper. The typed layer on top is written by hand.
//! **Used by** OptiX programs (`__raygen__*`, `__miss__*`, `__closesthit__*`, ...) written with
//! `cuda_device::kernel`; the host side is the `optix` crate.

#![no_std]

/// One wrapper per `_optix_*` intrinsic, named without the `_optix_` prefix (generated).
#[allow(missing_docs, clippy::all)]
pub mod raw {
    include!(concat!(env!("OUT_DIR"), "/optix_raw.rs"));
}

/// Intrinsic names and header hashes the raw layer was generated from (coverage evidence).
pub const INVENTORY: &str = include_str!(concat!(env!("OUT_DIR"), "/optix_device_inventory.json"));

/// OptiX types and constants (`optix_types.h`, generated, `no_std`): transform structs, enums,
/// flags — the same definitions the host `optix-sys` crate uses.
#[allow(missing_docs, non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code, clippy::all)]
pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/optix_types.rs"));
}
