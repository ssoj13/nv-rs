//! Green contexts: one GPU split into partitions of streaming multiprocessors (SMs).
//!
//! **What.** [`split_sms`] partitions a device's SMs into groups of at least `min_sms` SMs and
//! creates one green context per group; each [`GreenContext`] exposes a `cuda_core::Context` and
//! creates streams whose work runs only on that group's SMs.
//! **Why.** Isolating a latency-critical stream (interactive viewport) from a throughput stream
//! (background denoise, ML) on one GPU without time-slicing the whole device.
//! **How.** `cuDeviceGetDevResource(SM)` -> `cuDevSmResourceSplitByCount` ->
//! `cuDevResourceGenerateDesc` -> `cuGreenCtxCreate`; the green context becomes a `Context` through
//! `cuCtxFromGreenCtx` and `Context::borrow_with_owner`, whose owner destroys it last.

use crate::invalid_value;
use cuda_core::sys;
use cuda_core::{Context, DriverError, IntoResult, Stream};
use std::sync::Arc;

/// Owns a `CUgreenCtx`; dropped after the `Context` and every stream built on it.
struct GreenOwner {
    raw: sys::CUgreenCtx,
}

// SAFETY: a CUgreenCtx is a driver handle, destroyed only by this owner.
unsafe impl Send for GreenOwner {}
// SAFETY: as above.
unsafe impl Sync for GreenOwner {}

impl Drop for GreenOwner {
    fn drop(&mut self) {
        // SAFETY: owned handle; the Context and streams holding this owner are already gone.
        if let Err(e) = unsafe { sys::cuGreenCtxDestroy(self.raw) }.result() {
            // No Context remains to record on: the error is reported, not swallowed.
            eprintln!("cuda-core-ext: cuGreenCtxDestroy failed: {e}");
        }
    }
}

/// Owns a stream created on a green context.
struct StreamOwner {
    raw: sys::CUstream,
    ctx: Arc<Context>,
}

// SAFETY: a CUstream is a driver handle, destroyed only by this owner.
unsafe impl Send for StreamOwner {}
// SAFETY: as above.
unsafe impl Sync for StreamOwner {}

impl Drop for StreamOwner {
    fn drop(&mut self) {
        let raw = self.raw;
        // SAFETY: owned stream; the `Stream` borrowing it has been dropped.
        crate::teardown(&self.ctx, || unsafe { sys::cuStreamDestroy_v2(raw) });
    }
}

/// One SM partition of a GPU.
pub struct GreenContext {
    ctx: Arc<Context>,
    green: Arc<GreenOwner>,
    sm_count: u32,
}

impl GreenContext {
    /// The partition as a `cuda_core::Context` (modules, buffers, launches work as usual).
    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }

    /// SMs in this partition.
    pub fn sm_count(&self) -> u32 {
        self.sm_count
    }

    /// A non-blocking stream whose work runs on this partition's SMs.
    pub fn stream(&self) -> Result<Arc<Stream>, DriverError> {
        self.ctx.bind_to_thread()?;
        let mut raw: sys::CUstream = std::ptr::null_mut();
        // SAFETY: valid out-pointer and green context.
        unsafe {
            sys::cuGreenCtxStreamCreate(
                &mut raw,
                self.green.raw,
                sys::CUstream_flags_enum_CU_STREAM_NON_BLOCKING,
                0,
            )
        }
        .result()?;
        let owner = Arc::new(StreamOwner {
            raw,
            ctx: self.ctx.clone(),
        });
        // SAFETY: `raw` is a live stream of this context; `owner` destroys it when the last holder
        // drops, so it outlives the returned Stream.
        Ok(unsafe { Stream::borrow_with_owner(raw.cast(), &self.ctx, owner) })
    }
}

/// Splits `device`'s SMs into as many groups of at least `min_sms` SMs as fit and creates one
/// green context per group. SMs left over (the driver's remainder) are not used.
pub fn split_sms(device: &Context, min_sms: u32) -> Result<Vec<GreenContext>, DriverError> {
    if min_sms == 0 {
        return Err(invalid_value());
    }
    device.bind_to_thread()?;
    let dev = device.cu_device();
    // SAFETY: zeroed resource, filled by the driver.
    let mut whole: sys::CUdevResource = unsafe { std::mem::zeroed() };
    // SAFETY: valid out-pointer and device.
    unsafe {
        sys::cuDeviceGetDevResource(
            dev,
            &mut whole,
            sys::CUdevResourceType_CU_DEV_RESOURCE_TYPE_SM,
        )
    }
    .result()?;

    let mut groups = 0u32;
    // SAFETY: a null result pointer asks only for the group count.
    unsafe {
        sys::cuDevSmResourceSplitByCount(
            std::ptr::null_mut(),
            &mut groups,
            &whole,
            std::ptr::null_mut(),
            0,
            min_sms,
        )
    }
    .result()?;
    if groups == 0 {
        return Err(invalid_value());
    }
    // SAFETY: zeroed resources, filled by the driver.
    let mut parts: Vec<sys::CUdevResource> =
        (0..groups).map(|_| unsafe { std::mem::zeroed() }).collect();
    // SAFETY: zeroed remainder, filled by the driver.
    let mut remainder: sys::CUdevResource = unsafe { std::mem::zeroed() };
    // SAFETY: `parts` holds `groups` entries as the count query reported.
    unsafe {
        sys::cuDevSmResourceSplitByCount(
            parts.as_mut_ptr(),
            &mut groups,
            &whole,
            &mut remainder,
            0,
            min_sms,
        )
    }
    .result()?;

    let mut out = Vec::with_capacity(groups as usize);
    for part in parts.iter_mut().take(groups as usize) {
        // SAFETY: an SM resource from the split; the union's `sm` member is the active one.
        let sm_count = unsafe { part.__bindgen_anon_1.sm.smCount };
        let mut desc: sys::CUdevResourceDesc = std::ptr::null_mut();
        // SAFETY: one valid resource.
        unsafe { sys::cuDevResourceGenerateDesc(&mut desc, part, 1) }.result()?;
        let mut green: sys::CUgreenCtx = std::ptr::null_mut();
        #[cfg(windows)]
        let flags = sys::CUgreenCtxCreate_flags_CU_GREEN_CTX_DEFAULT_STREAM as u32;
        #[cfg(not(windows))]
        let flags = sys::CUgreenCtxCreate_flags_CU_GREEN_CTX_DEFAULT_STREAM;
        // SAFETY: descriptor generated above for this device.
        unsafe { sys::cuGreenCtxCreate(&mut green, desc, dev, flags) }.result()?;
        let owner = Arc::new(GreenOwner { raw: green });
        let mut cu_ctx: sys::CUcontext = std::ptr::null_mut();
        // SAFETY: live green context, valid out-pointer.
        unsafe { sys::cuCtxFromGreenCtx(&mut cu_ctx, green) }.result()?;
        // SAFETY: `cu_ctx` is the green context's context on `dev`; `owner` destroys the green
        // context only after this Context (and everything derived from it) is gone.
        let ctx = unsafe {
            Context::borrow_with_owner(cu_ctx.cast(), dev, device.ordinal(), owner.clone())
        };
        out.push(GreenContext {
            ctx,
            green: owner,
            sm_count,
        });
    }
    Ok(out)
}
