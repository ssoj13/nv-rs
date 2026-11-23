//! CUDA driver features that `cuda-core` does not wrap yet, written in `cuda-core`'s style so
//! they can be offered upstream.
//!
//! **What.**
//! - [`external`] — memory and semaphores imported from Vulkan, D3D12 or another process
//!   (`cuImportExternalMemory`, `cuImportExternalSemaphore`): the CUDA side of CUDA <-> wgpu
//!   interop. Imported buffers become ordinary `cuda_core::RawDeviceBuffer`s.
//! - [`array`] — CUDA arrays and mipmapped arrays, the storage textures and surfaces read from,
//!   with host <-> array copies.
//! - [`texture`] — texture objects (filtered reads; arrays, mipmapped arrays, pitched and linear
//!   buffers) and surface objects (writes).
//! - [`sparse`] — sparse (tiled) arrays: tile geometry, tile pools, mapping tiles.
//! - [`host`] — page-locking memory the caller already owns, and managed (unified) memory.
//! - [`device_id`] — device UUID and Windows LUID, to pair a CUDA device with a Vulkan or DXGI
//!   adapter.
//! - [`green`] — green contexts: one GPU split into SM partitions with their own streams.
//! - [`lease`] — keeps buffers allocated until work a native library queued on them completes.
//!
//! **Conventions (shared with `cuda-core`).** Every handle owns its driver object and destroys it
//! on drop; drop failures are recorded on the owning `Context` (`Context::record_err`) instead of
//! panicking. Fallible calls return `DriverError`. Everything takes `&Arc<Context>`.
//!
//! **Used by** `nv-wgpu` (interop), the safe `optix` crate (textures), `curand` (leases) and
//! render-rs.

pub mod array;
pub mod device_id;
pub mod external;
pub mod green;
pub mod host;
pub mod lease;
pub mod sparse;
pub mod texture;

pub use array::{Array, ArrayDesc, ArrayFormat, MipmappedArray};
pub use device_id::{Luid, device_luid, device_uuid};
pub use external::{ExternalMemory, ExternalSemaphore, MemoryHandle, SemaphoreHandle};
pub use green::{GreenContext, split_sms};
pub use host::{HostRegistration, ManagedBuffer};
pub use lease::{Access, enqueue_leased};
pub use sparse::{SparseProperties, TilePool, sparse_arrays_supported};
pub use texture::{AddressMode, FilterMode, SurfaceObject, TextureDesc, TextureObject};

use cuda_core::{Context, DriverError};

/// `CUDA_ERROR_INVALID_VALUE`, for arguments rejected before reaching the driver.
pub(crate) fn invalid_value() -> DriverError {
    DriverError(cuda_core::sys::cudaError_enum_CUDA_ERROR_INVALID_VALUE)
}

/// Runs a driver teardown call with `ctx` current; a failure is recorded on the context rather
/// than panicking in `Drop`.
pub(crate) fn teardown(ctx: &Context, f: impl FnOnce() -> cuda_core::sys::CUresult) {
    use cuda_core::IntoResult;
    if let Err(e) = ctx.bind_to_thread() {
        ctx.record_err(Err::<(), _>(e));
        return;
    }
    ctx.record_err(f().result());
}
