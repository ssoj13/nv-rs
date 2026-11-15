//! Caller-owned reusable workspace [`Scratch`] with byte-budget enforcement for cuFFT plans.
//!
//! **What.** Holds one stream-ordered `DeviceBuffer<u8>` wrapped in a thread-safe shared inner
//! state that grows on demand up to a caller's byte budget and is shared across cuFFT plans on the
//! same stream.
//! **Why.** Disabling cuFFT auto-allocation (`cufftSetAutoAllocation(plan, 0)`) and supplying a
//! caller-owned [`Scratch`] keeps GPU memory bounded by an explicit byte budget. Retaining shared
//! ownership in [`crate::Plan`] guarantees plans never execute with dangling or prematurely
//! deallocated workspace buffers even if the original [`Scratch`] handle is dropped or reallocated.
//! **Used by** [`crate::Plan`] and cuFFT execution pipelines.

use crate::{Error, Result};
use cuda_core::{DeviceBuffer, Stream};
use std::ffi::c_void;
use std::sync::{Arc, Mutex};

pub(crate) struct ScratchState {
    pub(crate) buffer: Option<DeviceBuffer<u8>>,
    pub(crate) generation: u64,
}

pub(crate) struct ScratchInner {
    pub(crate) stream: Arc<Stream>,
    pub(crate) state: Mutex<ScratchState>,
}

/// Reusable device workspace bound to a single CUDA [`Stream`].
///
/// Shared across plans on the same stream via [`Arc`] so that plans retain shared ownership
/// of the workspace allocation and never execute with dangling device pointers.
#[derive(Clone)]
pub struct Scratch {
    pub(crate) inner: Arc<ScratchInner>,
}

impl Scratch {
    /// Creates an empty workspace for FFT plans on `stream`.
    pub fn new(stream: &Arc<Stream>) -> Self {
        Self {
            inner: Arc::new(ScratchInner {
                stream: stream.clone(),
                state: Mutex::new(ScratchState {
                    buffer: None,
                    generation: 0,
                }),
            }),
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
        let state = self.inner.state.lock().unwrap_or_else(|p| p.into_inner());
        state.buffer.as_ref().map_or(0, |b| b.len())
    }

    /// Stream this workspace is bound to.
    pub fn stream(&self) -> &Arc<Stream> {
        &self.inner.stream
    }

    /// Returns the current allocation generation counter.
    #[inline]
    pub(crate) fn generation(&self) -> u64 {
        let state = self.inner.state.lock().unwrap_or_else(|p| p.into_inner());
        state.generation
    }

    /// Ensures at least `bytes` of workspace are available on `stream`, admitting the request
    /// against `budget`.
    pub(crate) fn reserve(
        &mut self,
        stream: &Arc<Stream>,
        bytes: usize,
        budget: usize,
    ) -> Result<*mut c_void> {
        if !Arc::ptr_eq(stream, &self.inner.stream) {
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
            return Ok(std::ptr::null_mut());
        }
        let mut state = self.inner.state.lock().unwrap_or_else(|p| p.into_inner());
        let cur_cap = state.buffer.as_ref().map_or(0, |b| b.len());
        if cur_cap < bytes {
            state.buffer = None;
            // SAFETY: cuFFT workspace is scratch memory written before being read on `stream`.
            state.buffer = Some(unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, bytes)? });
            state.generation = state.generation.wrapping_add(1);
        }
        let buf = state
            .buffer
            .as_ref()
            .ok_or(Error::Invalid("scratch buffer missing after reserve"))?;
        Ok((buf.cu_deviceptr() as usize) as *mut c_void)
    }
}

impl std::fmt::Debug for Scratch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scratch")
            .field("capacity", &self.capacity())
            .field("stream", &self.stream())
            .finish()
    }
}
