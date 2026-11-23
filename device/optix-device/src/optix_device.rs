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
//! - [`coop_vec`] — cooperative vectors: [`coop_vec::CoopVec`], load, element-wise ops,
//!   matrix-vector products with bias, reductions and outer products for neural shading;
//! - [`exception`] — user exceptions, codes, details, line info.
//! - [`footprint`] — texture footprint queries (`optixTexFootprint2D*`) for sparse texturing;
//! - [`geometry`] — vertex data of the hit primitive (triangles, every curve kind, ribbons,
//!   spheres), current hit, by GAS index or by handle, and hit-kind helpers;
//! - [`hitobject`] — hit objects and shader execution reordering: traverse, make hit / miss /
//!   nop, invoke, reorder, every hit-object getter and transform;
//! - [`transform`] — transform lists, transforms by handle (static, matrix / SRT motion,
//!   instance), object <-> world matrices and point / vector / normal transforms, GAS motion;
//!
//! Every function is `unsafe`: OptiX restricts each intrinsic to certain program types, which the
//! type system cannot see; each documents where it is valid.
//!
//! **Build requirement: 64-bit pointers.** A crate whose OptiX programs use this API must be
//! compiled with 64-bit pointers in every address space: `CUDA_OXIDE_NO_SHORT_PTR=1`, e.g. as
//! `[env] CUDA_OXIDE_NO_SHORT_PTR = "1"` (or `short-pointers = false`) in the crate's
//! `.cargo/cuda-oxide.toml`. cuda-oxide's default `llc --nvptx-short-ptr` addresses a program's
//! `.local` depot through 32-bit registers, and OptiX faults (`CUDA_ERROR_ILLEGAL_ADDRESS`) on that
//! as soon as a program needs local memory (address-taken or run-time-indexed arrays, the
//! cooperative-vector pointer path for N > 64); nvcc never emits short pointers for OptiX code.
//!
//! **Used by** OptiX programs (`__raygen__*`, `__miss__*`, `__closesthit__*`, ...) written with
//! `cuda_device::kernel`; the host side is the `optix` crate.

#![no_std]

pub mod coop_vec;
pub mod exception;
pub mod footprint;
pub mod geometry;
pub mod hitobject;
pub mod payload;
pub mod state;
pub mod trace;
pub mod transform;

pub use payload::Payload;

/// A 3-vector (`float3` in the OptiX headers).
pub type Float3 = [f32; 3];

/// One wrapper per `_optix_*` intrinsic, named without the `_optix_` prefix (generated).
pub mod raw {
    include!(concat!(env!("OUT_DIR"), "/optix_raw.rs"));
}

/// Intrinsic names and header hashes the raw layer was generated from (coverage evidence).
pub const INVENTORY: &str = include_str!(concat!(env!("OUT_DIR"), "/optix_device_inventory.json"));

/// OptiX types and constants (`optix_types.h`, generated, `no_std`): transform structs, enums,
/// flags — the same definitions the host `optix-sys` crate uses.
#[expect(missing_docs, non_camel_case_types, non_upper_case_globals, non_snake_case, clippy::all, clippy::undocumented_unsafe_blocks, reason = "bindgen output")]
pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/optix_types.rs"));
}

/// `direct_call!(sbt_index, fn(T, ...) -> R, (args...))`: calls a direct callable program
/// (`optixDirectCall`) as an indirect PTX call; use inside `unsafe`.
pub use optix_device_macros::direct_call;
/// `continuation_call!(sbt_index, fn(T, ...) -> R, (args...))`: calls a continuation callable
/// program (`optixContinuationCall`); use inside `unsafe`.
pub use optix_device_macros::continuation_call;
