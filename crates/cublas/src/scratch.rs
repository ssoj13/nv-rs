//! Caller-owned reusable workspace [`Scratch`] with byte-budget enforcement.
//!
//! **What.** Holds one stream-ordered `DeviceBuffer<u8>` that grows on demand up to a caller's
//! byte budget and is reused across cuBLAS and cuBLASLt calls on the same stream.
//! **Why.** Avoids per-matmul `cudaMalloc`/`cudaFree` churn and keeps workspace memory bounded by
//! explicit byte budgets.
//! **Used by** [`crate::Handle::set_workspace`] and [`crate::lt::LtHandle::matmul_with_scratch`].

use crate::{Error, Result};
use cuda_core::{DeviceBuffer, Stream};
use std::ffi::c_void;
use std::sync::Arc;

/// Reusable device workspace bound to a single CUDA [`Stream`].
///
/// Grows to the largest admitted request and never shrinks until dropped.
pub struct Scratch {
    stream: Arc<Stream>,
    buffer: Option<DeviceBuffer<u8>>,
}

impl Scratch {
    /// Creates an empty workspace for operations on `stream`.
    pub fn new(stream: &Arc<Stream>) -> Self {
        Self {
            stream: stream.clone(),
            buffer: None,
        }
    }

    /// Creates a workspace pre-allocated to `bytes` if `bytes <= budget`.
    pub fn with_capacity(stream: &Arc<Stream>, bytes: usize, budget: usize) -> Result<Self> {
        let mut scratch = Self::new(stream);
        if bytes > 0 {
            scratch.reserve(stream, bytes, budget)?;
        }
        Ok(scratch)
    }

    /// Bytes currently held by this workspace.
    pub fn capacity(&self) -> usize {
        self.buffer.as_ref().map_or(0, |b| b.len())
    }

    /// Stream this workspace is bound to.
    pub fn stream(&self) -> &Arc<Stream> {
        &self.stream
    }

    /// Ensures at least `bytes` of workspace are available on `stream`, admitting the request
    /// against `budget`.
    ///
    /// Returns `(device_ptr_as_void, allocated_bytes)`. When `bytes == 0`, returns `(null, 0)`
    /// without allocating.
    pub(crate) fn reserve(
        &mut self,
        stream: &Stream,
        bytes: usize,
        budget: usize,
    ) -> Result<(*mut c_void, usize)> {
        if !std::ptr::eq(stream, &*self.stream) {
            return Err(Error::Invalid(
                "scratch used on a different stream than it was created for",
            ));
        }
        if bytes > budget {
            return Err(Error::Budget {
                requested: bytes,
                available: budget,
            });
        }
        if bytes == 0 {
            return Ok((std::ptr::null_mut(), 0));
        }
        if self.capacity() < bytes {
            // Drop the old buffer first; its stream-ordered free executes after prior work on `stream`.
            self.buffer = None;
            // SAFETY: cuBLAS/cuBLASLt workspace is scratch memory written before being read.
            self.buffer =
                Some(unsafe { DeviceBuffer::<u8>::uninitialized_async(&self.stream, bytes)? });
        }
        let buf = self
            .buffer
            .as_ref()
            .ok_or(Error::Invalid("scratch buffer missing after reserve"))?;
        Ok(((buf.cu_deviceptr() as usize) as *mut c_void, buf.len()))
    }
}
