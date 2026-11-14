//! cuSPARSE library [`Handle`] bound to a `cuda_core::Context`.
//!
//! **What.** Owns a `cusparseHandle_t`, binds it to the current `cuda_core::Stream` before
//! launching work, configures host/device scalar pointer mode, and queries the toolkit version.
//! **Teardown.** `Drop` calls `cusparseDestroy` and records any error on the owning
//! `cuda_core::Context` rather than panicking.
//! **Used by** [`crate::ops`] and external sparse linear algebra callers.

use crate::{Error, PointerMode, Result, check, ensure_loaded, sys, teardown};
use cuda_core::{Context, Stream};
use std::sync::Arc;

/// Owning handle to a cuSPARSE library context (`cusparseHandle_t`).
pub struct Handle {
    raw: sys::cusparseHandle_t,
    cuda: Arc<Context>,
}

// SAFETY: `cusparseHandle_t` can be transferred across host threads when externally synchronized;
// this wrapper does not implement `Sync`, so concurrent calls on one handle are prevented by Rust.
unsafe impl Send for Handle {}

impl Handle {
    /// Creates a new cuSPARSE handle on `cuda`.
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::cusparseHandle_t = std::ptr::null_mut();
        // SAFETY: `cuda` is bound to this thread and `raw` is a valid out-pointer.
        check("cusparseCreate", unsafe { sys::cusparseCreate(&mut raw) })?;
        Ok(Self { raw, cuda: cuda.clone() })
    }

    /// Raw `cusparseHandle_t`.
    pub fn raw(&self) -> sys::cusparseHandle_t {
        self.raw
    }

    /// Owning CUDA context.
    pub fn cuda(&self) -> &Arc<Context> {
        &self.cuda
    }

    /// Binds `stream` to this handle so subsequent cuSPARSE calls execute on `stream`.
    pub fn set_stream(&self, stream: &Stream) -> Result<()> {
        if !Arc::ptr_eq(&self.cuda, stream.context()) {
            return Err(Error::InvalidInput("stream belongs to a different CUDA context than the cuSPARSE handle"));
        }
        // SAFETY: `self.raw` and `stream.cu_stream()` are valid handles on the same context.
        check("cusparseSetStream", unsafe { sys::cusparseSetStream(self.raw, stream.cu_stream()) })
    }

    /// Returns the raw `CUstream` currently bound to this handle.
    pub fn raw_stream(&self) -> Result<sys::cudaStream_t> {
        let mut stream: sys::cudaStream_t = std::ptr::null_mut();
        // SAFETY: `self.raw` is valid and `stream` is a valid out-pointer.
        check("cusparseGetStream", unsafe { sys::cusparseGetStream(self.raw, &mut stream) })?;
        Ok(stream)
    }

    /// Sets whether scalar arguments (`alpha`, `beta`) live in host or device memory.
    pub fn set_pointer_mode(&self, mode: PointerMode) -> Result<()> {
        // SAFETY: `self.raw` is valid.
        check("cusparseSetPointerMode", unsafe { sys::cusparseSetPointerMode(self.raw, mode.to_sys()) })
    }

    /// Returns the current scalar pointer mode.
    pub fn pointer_mode(&self) -> Result<PointerMode> {
        let mut raw = 0;
        // SAFETY: `self.raw` is valid and `raw` is a valid out-pointer.
        check("cusparseGetPointerMode", unsafe { sys::cusparseGetPointerMode(self.raw, &mut raw) })?;
        match raw {
            x if x == sys::cusparsePointerMode_t_CUSPARSE_POINTER_MODE_HOST => Ok(PointerMode::Host),
            x if x == sys::cusparsePointerMode_t_CUSPARSE_POINTER_MODE_DEVICE => Ok(PointerMode::Device),
            _ => Err(Error::InvalidInput("unrecognized cusparsePointerMode_t returned by driver")),
        }
    }

    /// Ensures the handle is in `PointerMode::Host` before passing host scalar pointers.
    pub(crate) fn ensure_host_pointer_mode(&self) -> Result<()> {
        if self.pointer_mode()? != PointerMode::Host {
            return Err(Error::InvalidInput("cuSPARSE handle pointer mode must be Host when passing host scalars"));
        }
        Ok(())
    }

    /// Queries the cuSPARSE library version (`major * 1000 + minor * 100 + patch`).
    pub fn version(&self) -> Result<i32> {
        let mut ver = 0i32;
        // SAFETY: `self.raw` is valid and `ver` is a valid out-pointer.
        check("cusparseGetVersion", unsafe { sys::cusparseGetVersion(self.raw, &mut ver) })?;
        Ok(ver)
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            teardown(&self.cuda, "cusparseDestroy", || {
                // SAFETY: `raw` is a valid `cusparseHandle_t` owned by `self` and destroyed once.
                unsafe { sys::cusparseDestroy(raw) }
            });
        }
    }
}
