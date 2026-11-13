//! `optixTrace`: [`trace`] shoots a [`Ray`] into a traversable with a typed [`Payload`].
//!
//! **How.** One `_optix_trace_typed_32` call, the shape `optix_device_impl.h` uses: 17 scalar
//! inputs, then the 32 payload registers in and out; the payload size argument tells OptiX how
//! many of them are live.

use crate::payload::{Payload, from_words, to_words};
use crate::{Float3, raw, sys};

/// A ray: origin, direction and parametric interval.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ray {
    /// Origin.
    pub origin: Float3,
    /// Direction (need not be normalized; `t` is in its units).
    pub direction: Float3,
    /// Smallest accepted hit distance.
    pub tmin: f32,
    /// Largest accepted hit distance.
    pub tmax: f32,
    /// Time for motion blur.
    pub time: f32,
}

impl Ray {
    /// A ray from `origin` along `direction` over `[tmin, tmax]` at time 0.
    pub const fn new(origin: Float3, direction: Float3, tmin: f32, tmax: f32) -> Self {
        Self { origin, direction, tmin, tmax, time: 0.0 }
    }
}

/// Everything about a trace call besides the ray and payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraceOptions {
    /// Visibility mask (8 bits) tested against instance masks.
    pub visibility_mask: u32,
    /// `OPTIX_RAY_FLAG_*` bits.
    pub ray_flags: u32,
    /// SBT offset added to hit-group indices (ray type).
    pub sbt_offset: u32,
    /// SBT stride between geometries (number of ray types).
    pub sbt_stride: u32,
    /// Miss program index.
    pub miss_index: u32,
    /// Payload type (`OPTIX_PAYLOAD_TYPE_DEFAULT` or one `OPTIX_PAYLOAD_TYPE_ID_*`).
    pub payload_type: u32,
}

impl Default for TraceOptions {
    /// Visible to every instance, no flags, ray type 0 of 1, miss program 0, default payload type.
    fn default() -> Self {
        Self {
            visibility_mask: 0xFF,
            ray_flags: sys::OptixRayFlags_OPTIX_RAY_FLAG_NONE as u32,
            sbt_offset: 0,
            sbt_stride: 1,
            miss_index: 0,
            payload_type: sys::OptixPayloadTypeID_OPTIX_PAYLOAD_TYPE_DEFAULT as u32,
        }
    }
}

/// Traces `ray` into `handle`; `payload` goes in and comes back as the programs left it.
///
/// # Safety
///
/// Valid in raygen, closest-hit and miss programs (and callables called from them), within the
/// pipeline's `maxTraceDepth`; `handle` must be a live traversable of this pipeline's context, and
/// the reached programs must agree on `P`.
#[inline(always)]
pub unsafe fn trace<P: Payload>(handle: u64, ray: &Ray, opts: &TraceOptions, payload: &mut P) {
    let p = to_words(payload);
    let [ox, oy, oz] = ray.origin;
    let [dx, dy, dz] = ray.direction;
    // SAFETY: forwarded contract; argument order is the header's `optixTrace`.
    let r = unsafe {
        raw::trace_typed_32(
            opts.payload_type, handle, ox, oy, oz, dx, dy, dz, ray.tmin, ray.tmax, ray.time, opts.visibility_mask, opts.ray_flags,
            opts.sbt_offset, opts.sbt_stride, opts.miss_index, P::WORDS,
            p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11], p[12], p[13], p[14], p[15],
            p[16], p[17], p[18], p[19], p[20], p[21], p[22], p[23], p[24], p[25], p[26], p[27], p[28], p[29], p[30], p[31],
        )
    };
    *payload = from_words(&r);
}

/// Traces a ray that carries no payload (shadow rays whose programs only terminate or ignore).
///
/// # Safety
///
/// As [`trace`].
#[inline(always)]
pub unsafe fn trace_no_payload(handle: u64, ray: &Ray, opts: &TraceOptions) {
    let mut nothing = [0u32; 0];
    // SAFETY: forwarded contract; a zero-word payload.
    unsafe { trace(handle, ray, opts, &mut nothing) }
}
