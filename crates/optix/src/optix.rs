//! Safe OptiX host API on top of `optix-sys` and `cuda-core`.
//!
//! **What.**
//! - [`context`] — [`DeviceContext`]: OptiX on one CUDA context, with an optional log sink.
//! - [`accel`] — [`Accel`]: geometry (triangle meshes) and instance acceleration structures,
//!   with compaction, a byte budget, refit in place and a caller-owned reusable [`Scratch`]. An
//!   instance structure keeps the structures it references alive.
//! - [`denoise`] — [`Denoiser`]: the AI denoiser (HDR / AOV models, albedo and normal guides)
//!   for RGBA `f32` images.
//! - [`module`] — [`Module`] from PTX or OptiX-IR, [`ProgramGroup`]s (raygen, miss, hit group,
//!   callables), [`PipelineOptions`] / [`ModuleOptions`].
//! - [`pipeline`] — [`Pipeline`] and [`Pipeline::launch`] with typed launch parameters.
//! - [`sbt`] — [`SbtRecord<T>`] (32-byte header + user data, 16-byte aligned) and
//!   [`ShaderBindingTable`], which owns the device copies of its records.
//!
//! **Conventions.** Every handle owns its OptiX object, holds an `Arc` of what it depends on
//! (a pipeline holds its program groups, a program group its modules, everything the
//! [`DeviceContext`]) and destroys it on drop; a failed OptiX destroy keeps its real
//! `OptixResult` in `cuda_core_ext::teardown` (`Library::Optix`), while CUDA driver failures in a
//! drop (bind, sync, free) are recorded on the CUDA context, as in `cuda-core`. Contexts and
//! streams are compared by value (`cuda_core_ext::same_context` / `same_stream`). Work is
//! stream-ordered. Build outputs, scratch, instance and argument arrays,
//! compaction and relocation buffers and denoiser state are stream-ordered allocations on the
//! stream of the call that creates them, freed with `cuMemFreeAsync` on that stream: ordered
//! after the work queued there, not after work on other streams (the `unsafe` contracts make
//! that the caller's job). SBT records ([`ShaderBindingTable`]) and transforms
//! ([`xform::Transform`]) are synchronous allocations (`cuMemAlloc` + a synchronized copy).
//! Streams are identified by their `CUstream` and CUDA context. Errors are [`Error`], carrying
//! the OptiX log when the call produced one.
//!
//! **Device code** is written in Rust with cuda-oxide; nv-rs spike S1b traces from a Rust
//! raygen. **Used by** render-rs and the nv-rs OptiX tests.
#![cfg_attr(target_os = "linux", allow(clippy::unnecessary_cast))]

pub mod accel;
pub mod cluster;
pub mod context;
pub mod coop_vec;
pub mod denoise;
pub mod micromap;
pub mod module;
pub mod pipeline;
pub mod sbt;
pub mod stack;
pub mod task;
pub mod xform;

pub use accel::{
    Aabb, Accel, BuildOptions, CurveType, Curves, CustomPrimitives, DeviceInstances, Emit,
    GeometryFlags, INSTANCE_FLAG_DISABLE_ANYHIT, INSTANCE_STAGING_BYTES, INSTANCE_STAGING_MAX_BYTES, Indices, Instance, InstanceBuffer, Keys, Radii, Relink,
    RelocationInfo, SbtOffsets, Scratch, Spheres, TriangleMesh, VertexFormat, Vertices,
};
pub use cluster::{
    ClusterAccel, ClusterArg, ClusterArgs, ClusterFlags, ClusterInput, ClusterKind, ClusterLimits,
    ClusterMode, ClusterOptions, ClusterRefs, ClusterSizes, GridLimits, TriangleLimits,
};
pub use context::{DeviceContext, LogFn, describe, error_name, error_string};
pub use denoise::{
    AlphaMode, AovType, Denoiser, DenoiserModel, DenoiserOptions, Guides, Image, Layer, Params,
    Pixel, PixelFormat, Tile, split_image,
};
pub use micromap::{
    MicromapCount, MicromapDesc, MicromapFormat, MicromapIndices, MicromapOptions, MicromapSizes,
    MicromapUse, OpacityMicromapArray, base_to_micro_bary, micro_to_base_bary,
};
pub use module::{
    BoundValue, BuiltinIs, DebugLevel, HitGroup, Module, ModuleOptions, OptLevel, PayloadSemantics,
    PayloadType, PipelineOptions, Primitive, ProgramGroup, Programs,
};
pub use pipeline::{LinkOptions, Pipeline};
pub use sbt::{SbtRecord, ShaderBindingTable};
pub use stack::{PipelineStack, StackSizes, simple_path_tracer};
pub use task::{CompileState, PendingModule, Task};
pub use xform::{Child, MotionOptions, Srt, Transform, TransformKind};

pub use optix_sys as sys;

use cuda_core::DriverError;

/// Everything an OptiX call can fail with.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `nvoptix` could not be loaded or does not support this ABI.
    #[error("OptiX unavailable: {0}")]
    Unavailable(String),
    /// An OptiX call returned an error; `log` is the compiler/linker log, if any.
    #[error("{call} failed with {}{}", crate::context::describe(.code), log_suffix(.log))]
    Optix {
        /// The OptiX entry point.
        call: &'static str,
        /// The raw `OptixResult`.
        code: sys::OptixResult,
        /// Log text the call produced.
        log: String,
    },
    /// A CUDA driver call failed.
    #[error("CUDA: {0}")]
    Driver(#[from] DriverError),
    /// A build needs more device memory than the caller's budget admits.
    #[error("needs {requested} device bytes, budget admits {available}")]
    Budget {
        /// Peak bytes the operation would allocate.
        requested: usize,
        /// The caller's budget.
        available: usize,
    },
    /// An argument was rejected before reaching OptiX.
    #[error("invalid argument: {0}")]
    Invalid(&'static str),
}

fn log_suffix(log: &str) -> String {
    if log.is_empty() {
        String::new()
    } else {
        format!(":\n{log}")
    }
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Maps an `OptixResult` to [`Error::Optix`].
pub(crate) fn check(call: &'static str, code: sys::OptixResult) -> Result<()> {
    check_log(call, code, &[], 0)
}

/// Like [`check`], attaching the first `len` bytes of `log` (OptiX reports the size it wanted,
/// which may exceed the buffer).
pub(crate) fn check_log(
    call: &'static str,
    code: sys::OptixResult,
    log: &[u8],
    len: usize,
) -> Result<()> {
    if code == sys::OptixResult_OPTIX_SUCCESS {
        return Ok(());
    }
    Err(Error::Optix {
        call,
        code,
        log: log_text(log, len),
    })
}

/// The first `len` bytes of an OptiX log buffer (OptiX reports the size it wanted, which may
/// exceed the buffer) up to the first NUL, trimmed.
pub(crate) fn log_text(log: &[u8], len: usize) -> String {
    let text = &log[..len.min(log.len())];
    String::from_utf8_lossy(text.split(|b| *b == 0).next().unwrap_or_default())
        .trim()
        .to_owned()
}

/// Size of the log buffers handed to compile and link calls.
pub(crate) const LOG_BYTES: usize = 16 * 1024;

/// Returns the driver's OptiX function table.
pub(crate) fn table() -> Result<&'static sys::OptixFunctionTable> {
    sys::api()
        .map(|a| &a.table)
        .map_err(|e| Error::Unavailable(e.to_string()))
}

/// Unwraps a function-table entry; a null entry means the driver did not fill it for this ABI.
macro_rules! entry {
    ($t:expr, $name:ident) => {
        $t.$name.ok_or($crate::Error::Unavailable(
            concat!(stringify!($name), " missing from the OptiX function table").into(),
        ))?
    };
}
pub(crate) use entry;
