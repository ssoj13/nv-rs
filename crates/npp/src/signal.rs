//! 1D signal operations (`npps`) with stream context and two-phase scratch sizing.
//!
//! **What.** Wraps `nppsSet_32f_Ctx`, `nppsAdd_32f_Ctx`, `nppsMul_32f_Ctx`, and two-phase
//! `nppsSumGetBufferSize_32f_Ctx` + `nppsSum_32f_Ctx` using caller-owned reusable scratch.

use crate::{Error, NppContext, Result, check, sys};
use cuda_core::DeviceBuffer;

/// Fills `dst` with `value` (`nppsSet_32f_Ctx`).
pub fn set_32f(ctx: &NppContext, value: f32, dst: &mut DeviceBuffer<f32>) -> Result<()> {
    sys::npps::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    if dst.is_empty() {
        return Err(Error::InvalidInput("signal buffer must be non-empty"));
    }
    ctx.bind()?;
    // SAFETY: `dst` is a live device buffer of `dst.len()` `f32` elements.
    check("nppsSet_32f_Ctx", unsafe {
        sys::nppsSet_32f_Ctx(value, dst.cu_deviceptr() as *mut f32, dst.len(), ctx.raw())
    })
}

/// Elementwise `dst[i] = src1[i] + src2[i]` (`nppsAdd_32f_Ctx`).
pub fn add_32f(ctx: &NppContext, src1: &DeviceBuffer<f32>, src2: &DeviceBuffer<f32>, dst: &mut DeviceBuffer<f32>) -> Result<()> {
    sys::npps::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let n = src1.len();
    if n == 0 || src2.len() != n || dst.len() != n {
        return Err(Error::InvalidInput("signal buffers must be non-empty and equal length"));
    }
    ctx.bind()?;
    // SAFETY: all three device buffers have `n` `f32` elements.
    check("nppsAdd_32f_Ctx", unsafe {
        sys::nppsAdd_32f_Ctx(src1.cu_deviceptr() as *const f32, src2.cu_deviceptr() as *const f32, dst.cu_deviceptr() as *mut f32, n, ctx.raw())
    })
}

/// Elementwise `dst[i] = src1[i] * src2[i]` (`nppsMul_32f_Ctx`).
pub fn mul_32f(ctx: &NppContext, src1: &DeviceBuffer<f32>, src2: &DeviceBuffer<f32>, dst: &mut DeviceBuffer<f32>) -> Result<()> {
    sys::npps::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let n = src1.len();
    if n == 0 || src2.len() != n || dst.len() != n {
        return Err(Error::InvalidInput("signal buffers must be non-empty and equal length"));
    }
    ctx.bind()?;
    // SAFETY: all three device buffers have `n` `f32` elements.
    check("nppsMul_32f_Ctx", unsafe {
        sys::nppsMul_32f_Ctx(src1.cu_deviceptr() as *const f32, src2.cu_deviceptr() as *const f32, dst.cu_deviceptr() as *mut f32, n, ctx.raw())
    })
}

/// Phase 1: queries the scratch buffer size in bytes required for [`sum_32f`] over `len` elements.
pub fn sum_32f_scratch_bytes(ctx: &NppContext, len: usize) -> Result<usize> {
    sys::npps::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    if len == 0 {
        return Err(Error::InvalidInput("signal length must be non-zero"));
    }
    ctx.bind()?;
    let mut bytes = 0usize;
    // SAFETY: `bytes` is a valid host out-pointer.
    check("nppsSumGetBufferSize_32f_Ctx", unsafe { sys::nppsSumGetBufferSize_32f_Ctx(len, &mut bytes, ctx.raw()) })?;
    Ok(bytes)
}

/// Phase 2: computes the sum of `src` into `sum_out[0]` using caller-owned `scratch`
/// (`nppsSum_32f_Ctx`).
pub fn sum_32f(ctx: &NppContext, src: &DeviceBuffer<f32>, sum_out: &mut DeviceBuffer<f32>, scratch: &mut DeviceBuffer<u8>) -> Result<()> {
    sys::npps::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    if src.is_empty() || sum_out.is_empty() {
        return Err(Error::InvalidInput("signal source and output must be non-empty"));
    }
    let needed = sum_32f_scratch_bytes(ctx, src.len())?;
    if scratch.len() < needed {
        return Err(Error::Budget { requested: needed, available: scratch.len() });
    }
    ctx.bind()?;
    // SAFETY: `src`, `sum_out`, and `scratch` (at least `needed` bytes) are valid device buffers.
    check("nppsSum_32f_Ctx", unsafe {
        sys::nppsSum_32f_Ctx(
            src.cu_deviceptr() as *const f32,
            src.len(),
            sum_out.cu_deviceptr() as *mut f32,
            scratch.cu_deviceptr() as *mut u8,
            ctx.raw(),
        )
    })
}
