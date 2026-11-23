//! Caller-owned reusable workspace [`Scratch`] with byte-budget enforcement for cuFFT plans.
//!
//! **What.** A cheap-to-clone handle to one shared [`cuda_core_ext::Scratch`] (the nv-rs
//! workspace: stream-ordered growth, whole-request budget, native stream identity, generation
//! counter) behind a mutex, shared across cuFFT plans on the same stream.
//! **Why.** Disabling cuFFT auto-allocation (`cufftSetAutoAllocation(plan, 0)`) and supplying a
//! caller-owned [`Scratch`] keeps GPU memory bounded by an explicit byte budget. cuFFT keeps the
//! work-area pointer inside the plan (`cufftSetWorkArea`), so several plans share one workspace:
//! each [`crate::Plan`] retains the shared state, compares [`cuda_core_ext::Scratch::generation`]
//! and the pointer before every exec to rebind after a growth, and holds the mutex from that check
//! until the exec is queued, so no clone can replace the buffer in between.
//! **How.** The stream is stored next to the mutex (not only inside the shared workspace) so
//! [`Scratch::stream`] and [`crate::Plan::workspace_stream`] can hand out `&Arc<Stream>` without a
//! lock. All allocation, budget and stream rules are `cuda_core_ext::Scratch`'s.
//! **Used by** [`crate::Plan`] and cuFFT execution pipelines.

use crate::Result;
use cuda_core::Stream;
use std::sync::{Arc, Mutex, MutexGuard};

/// State shared by every clone of a [`Scratch`] and retained by the plans bound to it.
pub(crate) struct Shared {
    pub(crate) stream: Arc<Stream>,
    pub(crate) scratch: Mutex<cuda_core_ext::Scratch>,
}

impl Shared {
    /// Locks the workspace. Every update of `cuda_core_ext::Scratch` leaves it consistent, so a
    /// poisoned lock is recovered instead of panicking.
    pub(crate) fn lock(&self) -> MutexGuard<'_, cuda_core_ext::Scratch> {
        self.scratch.lock().unwrap_or_else(|p| p.into_inner())
    }
}

/// Reusable device workspace bound to a single CUDA [`Stream`].
///
/// Shared across plans on the same stream via [`Arc`] so that plans retain shared ownership
/// of the workspace allocation and never execute with dangling device pointers.
#[derive(Clone)]
pub struct Scratch {
    pub(crate) inner: Arc<Shared>,
}

impl Scratch {
    /// Creates an empty workspace for FFT plans on `stream`.
    pub fn new(stream: &Arc<Stream>) -> Self {
        Self::from_scratch(stream, cuda_core_ext::Scratch::new(stream))
    }

    /// Creates a workspace pre-allocated to `bytes` if `bytes <= budget`.
    pub fn with_capacity(stream: &Arc<Stream>, bytes: usize, budget: usize) -> Result<Self> {
        let scratch = cuda_core_ext::Scratch::with_capacity(stream, bytes, Some(budget))?;
        Ok(Self::from_scratch(stream, scratch))
    }

    fn from_scratch(stream: &Arc<Stream>, scratch: cuda_core_ext::Scratch) -> Self {
        Self {
            inner: Arc::new(Shared {
                stream: stream.clone(),
                scratch: Mutex::new(scratch),
            }),
        }
    }

    /// Bytes currently held by this workspace.
    pub fn capacity(&self) -> usize {
        self.inner.lock().capacity()
    }

    /// Stream this workspace is bound to.
    pub fn stream(&self) -> &Arc<Stream> {
        &self.inner.stream
    }

    /// Ensures at least `bytes` of workspace are available on `stream`, admitting the request
    /// against `budget`.
    ///
    /// Returns the device pointer (`0` while nothing is allocated) and the allocation generation,
    /// read under one lock so a concurrent growth through another clone cannot pair a pointer with
    /// another allocation's generation.
    pub(crate) fn reserve(
        &mut self,
        stream: &Stream,
        bytes: usize,
        budget: usize,
    ) -> Result<(cuda_core::sys::CUdeviceptr, u64)> {
        let mut scratch = self.inner.lock();
        let (ptr, _) = scratch.reserve(stream, bytes, Some(budget))?;
        Ok((ptr, scratch.generation()))
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
