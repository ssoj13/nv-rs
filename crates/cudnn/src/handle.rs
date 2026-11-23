//! RAII ownership of `cudnnHandle_t` with explicit stream binding; a failed `cudnnDestroy` is kept
//! in the `cuda_core_ext::teardown` sink with its real status.

use std::ptr;
use std::sync::Arc;

use cuda_core::{Context, Stream};
use cudnn_sys as sys;

use crate::{Result, check, record_teardown, same_ctx};

/// Owned cuDNN library handle tied to one [`Context`].
///
/// The handle stores the most recently bound [`Arc<Stream>`] so stream lifetime outlives any
/// queued cuDNN work. Dropping `Handle` binds the context and destroys the handle without
/// panicking: a failed bind stays on [`Context`], a failed `cudnnDestroy` goes to
/// `cuda_core_ext::teardown` (`Library::Cudnn`).
pub struct Handle {
    raw: sys::cudnnHandle_t,
    ctx: Arc<Context>,
    stream: Option<Arc<Stream>>,
    raw_stream: *mut std::ffi::c_void,
}

// SAFETY: `Handle` exposes mutation only via `&mut self` (and is `!Sync`), guaranteeing exclusive
// access whenever transferred across threads; cuDNN handles are bound to the `Context` via
// `bind_to_thread` before every FFI call.
unsafe impl Send for Handle {}

impl Handle {
    /// Creates a new cuDNN handle on `ctx`.
    pub fn new(ctx: &Arc<Context>) -> Result<Self> {
        sys::load()?;
        ctx.bind_to_thread()?;
        let mut raw: sys::cudnnHandle_t = ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer and `ctx` is bound on this thread.
        let status = unsafe { sys::cudnnCreate(&mut raw) };
        check("cudnnCreate", status)?;
        Ok(Self {
            raw,
            ctx: ctx.clone(),
            stream: None,
            raw_stream: ptr::null_mut(),
        })
    }

    /// Owning CUDA context.
    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }

    /// Currently bound CUDA stream, if any.
    pub fn stream(&self) -> Option<&Arc<Stream>> {
        self.stream.as_ref()
    }

    /// Raw underlying `cudnnHandle_t`.
    pub fn as_raw(&self) -> sys::cudnnHandle_t {
        self.raw
    }

    /// Explicitly binds `stream` to this cuDNN handle.
    pub fn set_stream(&mut self, stream: &Arc<Stream>) -> Result<()> {
        same_ctx(stream, &self.ctx)?;
        self.ctx.bind_to_thread()?;
        // SAFETY: `self.raw` is a live cuDNN handle and `stream.cu_stream()` is valid on `self.ctx`.
        let status = unsafe { sys::cudnnSetStream(self.raw, stream.cu_stream()) };
        check("cudnnSetStream", status)?;
        self.stream = Some(stream.clone());
        self.raw_stream = stream.cu_stream().cast();
        Ok(())
    }

    /// Binds an external raw `CUstream` (e.g. owned by CubeCL) to this cuDNN handle, skipping the
    /// `cudnnSetStream` FFI call when the exact same stream pointer is already bound.
    ///
    /// # Safety
    /// `raw_stream` must be a valid `CUstream` belonging to `self.context()` and must outlive all
    /// cuDNN work enqueued on this handle until another stream is bound.
    pub unsafe fn set_raw_stream(&mut self, raw_stream: cuda_bindings::CUstream) -> Result<()> {
        self.ctx.bind_to_thread()?;
        let raw_void = raw_stream.cast::<std::ffi::c_void>();
        if self.raw_stream == raw_void && !raw_void.is_null() {
            return Ok(());
        }
        // SAFETY: `self.raw` is a live cuDNN handle and caller guarantees `raw_stream` is valid on
        // `self.ctx`.
        let status = unsafe { sys::cudnnSetStream(self.raw, raw_stream) };
        check("cudnnSetStream", status)?;
        self.stream = None;
        self.raw_stream = raw_void;
        Ok(())
    }

    /// Ensures `stream` is bound to `self`, avoiding redundant `cudnnSetStream` FFI calls when the
    /// same stream is already bound.
    pub(crate) fn ensure_stream(&mut self, stream: &Arc<Stream>) -> Result<()> {
        same_ctx(stream, &self.ctx)?;
        if self
            .stream
            .as_ref()
            .is_some_and(|bound| cuda_core_ext::same_stream(bound, stream))
        {
            self.ctx.bind_to_thread()?;
            return Ok(());
        }
        self.set_stream(stream)
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        // A failed bind is put back on the context by `teardown::bind`; the handle then leaks.
        if !cuda_core_ext::teardown::bind(&self.ctx) {
            return;
        }
        // SAFETY: `self.raw` was initialized by `cudnnCreate` and has not been destroyed.
        let code = unsafe { sys::cudnnDestroy(self.raw) };
        record_teardown("cudnnDestroy", code);
    }
}
