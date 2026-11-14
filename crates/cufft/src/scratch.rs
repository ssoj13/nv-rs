//! Caller-owned reusable workspace [`Scratch`] with byte-budget enforcement for cuFFT plans.
//!
//! **What.** Holds one stream-ordered `DeviceBuffer<u8>` that grows on demand up to a caller's
//! byte budget and is shared across cuFFT plans on the same stream.
//! **Why.** Disabling cuFFT auto-allocation (`cufftSetAutoAllocation(plan, 0)`) and supplying a
//! caller-owned [`Scratch`] keeps GPU memory bounded by an explicit byte budget.
//! **Used by** [`crate::Plan`].

use crate::{Error, Result};
use cuda_core::{DeviceBuffer, Stream};
use std::ffi::c_void;
use std::sync::Arc;

/// Reusable device workspace bound to a single CUDA [`Stream`].
pub struct Scratch {
    stream: Arc<Stream>,
    buffer: Option<DeviceBuffer<u8>>,
}

impl Scratch {
    /// Creates an empty workspace for FFT plans on `stream`.
    pub fn new(stream: &Arc<Stream>) -> Self {
        Self { stream: stream.clone(), buffer: None }
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
    pub(crate) fn reserve(&mut self, stream: &Arc<Stream>, bytes: usize, budget: usize) -> Result<*mut c_void> {
        if !Arc::ptr_eq(stream, &self.stream) {
            return Err(Error::Invalid("scratch used on a different stream than it was created for"));
        }
        if bytes > budget {
            return Err(Error::Budget { requested: bytes, available: budget });
        }
        if bytes == 0 {
            return Ok(std::ptr::null_mut());
        }
        if self.capacity() < bytes {
            self.buffer = None;
            // SAFETY: cuFFT workspace is scratch memory written before being read.
            self.buffer = Some(unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, bytes)? });
        }
        let buf = self.buffer.as_ref().ok_or(Error::Invalid("scratch buffer missing after reserve"))?;
        Ok((buf.cu_deviceptr() as usize) as *mut c_void)
    }
}
