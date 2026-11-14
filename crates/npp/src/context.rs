//! [`NppContext`]: cached `NppStreamContext` for zero-allocation stream-ordered NPP calls.
//!
//! **What.** Queries the device's multiprocessor count, max threads per MP/block, shared memory
//! per block, compute capability, and stream flags once via `cuda-bindings`, and stores the
//! resulting `NppStreamContext` alongside the owning `Arc<Stream>`.
//! **Why.** NPP's `*_Ctx` functions take `NppStreamContext` by value and require its device
//! fields to match the active device; caching them once per stream eliminates per-call driver
//! queries and allocations.

use crate::{Error, Result, sys};
use cuda_bindings as cu;
use cuda_core::{DriverError, IntoResult, Stream};
use std::sync::Arc;

/// NPP library version returned by [`lib_version`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LibVersion {
    /// Major version number.
    pub major: i32,
    /// Minor version number.
    pub minor: i32,
    /// Build number.
    pub build: i32,
}

/// Queries the loaded `nppc` library version (`nppGetLibVersion`).
pub fn lib_version() -> Result<LibVersion> {
    sys::nppc::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    // SAFETY: `nppGetLibVersion` takes no arguments and returns a pointer to a static struct.
    let ptr = unsafe { sys::nppGetLibVersion() };
    if ptr.is_null() {
        return Err(Error::Unavailable(
            "nppGetLibVersion returned null".to_owned(),
        ));
    }
    // SAFETY: `ptr` is non-null and points to a static `NppLibraryVersion` owned by `nppc`.
    let ver = unsafe { &*ptr };
    Ok(LibVersion {
        major: ver.major,
        minor: ver.minor,
        build: ver.build,
    })
}

/// Stream-bound NPP execution context (`NppStreamContext`).
pub struct NppContext {
    stream: Arc<Stream>,
    raw: sys::NppStreamContext,
}

// SAFETY: `NppStreamContext` is plain POD metadata for an `Arc<Stream>`; `Stream` is `Send + Sync`.
unsafe impl Send for NppContext {}
// SAFETY: as above.
unsafe impl Sync for NppContext {}

impl NppContext {
    /// Builds an [`NppContext`] for `stream` by querying the device attributes of its CUDA
    /// context once.
    pub fn new(stream: &Arc<Stream>) -> Result<Self> {
        sys::nppc::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let cuda = stream.context();
        cuda.bind_to_thread()?;

        let mut dev: cu::CUdevice = 0;
        // SAFETY: `cuda` is current on this thread and `dev` is a valid out-pointer.
        unsafe { cu::cuCtxGetDevice(&mut dev) }.result()?;

        let mp_count = dev_attr(
            dev,
            cu::CUdevice_attribute_enum_CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT,
        )?;
        let max_threads_mp = dev_attr(
            dev,
            cu::CUdevice_attribute_enum_CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR,
        )?;
        let max_threads_blk = dev_attr(
            dev,
            cu::CUdevice_attribute_enum_CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK,
        )?;
        let shared_mem = dev_attr(
            dev,
            cu::CUdevice_attribute_enum_CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK,
        )?;
        let cc_major = dev_attr(
            dev,
            cu::CUdevice_attribute_enum_CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR,
        )?;
        let cc_minor = dev_attr(
            dev,
            cu::CUdevice_attribute_enum_CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR,
        )?;

        let mut stream_flags = 0u32;
        // SAFETY: `stream.cu_stream()` is a valid `CUstream` on the bound context.
        unsafe { cu::cuStreamGetFlags(stream.cu_stream(), &mut stream_flags) }.result()?;

        let raw = sys::NppStreamContext {
            hStream: stream.cu_stream(),
            nCudaDeviceId: dev,
            nMultiProcessorCount: mp_count,
            nMaxThreadsPerMultiProcessor: max_threads_mp,
            nMaxThreadsPerBlock: max_threads_blk,
            nSharedMemPerBlock: usize::try_from(shared_mem)
                .map_err(|_| Error::InvalidInput("negative shared memory attribute"))?,
            nCudaDevAttrComputeCapabilityMajor: cc_major,
            nCudaDevAttrComputeCapabilityMinor: cc_minor,
            nStreamFlags: stream_flags,
            nReserved0: 0,
        };
        Ok(Self {
            stream: stream.clone(),
            raw,
        })
    }

    /// Returns the underlying `cuda_core::Stream`.
    pub fn stream(&self) -> &Arc<Stream> {
        &self.stream
    }

    /// Returns the raw `NppStreamContext` by value for FFI calls.
    #[inline]
    pub fn raw(&self) -> sys::NppStreamContext {
        self.raw
    }

    /// Binds the owning CUDA context to the calling thread before launching NPP work.
    #[inline]
    pub(crate) fn bind(&self) -> Result<()> {
        self.stream.context().bind_to_thread().map_err(Error::from)
    }
}

fn dev_attr(
    dev: cu::CUdevice,
    attr: cu::CUdevice_attribute,
) -> std::result::Result<i32, DriverError> {
    let mut val = 0i32;
    // SAFETY: `dev` is a valid `CUdevice` and `val` is a valid out-pointer.
    unsafe { cu::cuDeviceGetAttribute(&mut val, attr, dev) }.result()?;
    Ok(val)
}
