//! Identity of CUDA contexts and streams: the one rule every nv-rs wrapper uses.
//!
//! **What.** [`same_context`] and [`same_stream`].
//! **Why.** `Arc<Context>` identity is not context identity: a DLPack import, `Context::new` on
//! the same ordinal, or a context borrowed from another library are different `Arc`s for the same
//! native `CUcontext`, and `Context::default_stream` returns a fresh `Arc<Stream>` per call.
//! Comparing `Arc` pointers rejected such buffers and streams although the driver sees one
//! context. `cuda-core` itself compares contexts by value (`RawDeviceBuffer::acquire`).
//! **How.** `Context: PartialEq` compares the device, the `CUcontext` handle and the ordinal. A
//! stream is the same when its `CUstream` and its context are; a held `Arc<Stream>` keeps its
//! handle alive, so equal handles cannot belong to two live streams.
//! **Used by** every wrapper crate that checks a buffer, stream or descriptor against its handle's
//! context (`cublas`, `cudnn`, `cufft`, `cufile`, `curand`, `cusolver`, `cusparse`, `nccl`,
//! `nvcomp`, `optix`), and by [`crate::scratch::Scratch`].

use cuda_core::{Context, Stream};

/// Whether `a` and `b` are the same CUDA context (same device, `CUcontext` and ordinal), whatever
/// `Arc` they are reached through.
#[inline]
pub fn same_context(a: &Context, b: &Context) -> bool {
    a == b
}

/// Whether `a` and `b` are the same CUDA stream: the same `CUstream` in the same context.
#[inline]
pub fn same_stream(a: &Stream, b: &Stream) -> bool {
    a.cu_stream() == b.cu_stream() && same_context(a.context(), b.context())
}
