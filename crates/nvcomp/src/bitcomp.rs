//! Safe RAII wrapper over the native Bitcomp plan C API (`bitcompCreatePlan`,
//! `bitcompCompressLossless`, `bitcompUncompress`, `bitcompMaxBuflen`).
//!
//! Decompression validates the stream header (`bitcompGetCompressedInfo`) on the host before the
//! kernel is queued, because the native kernel trusts it and faults on a truncated input.
//!
//! Only lossless compression is exposed: a plan created with a lossy [`BitcompMode`] can be used
//! for `bitcompUncompress`, but [`BitcompPlan::compress_lossless_async`] rejects it with
//! [`Error::Unsupported`] instead of mixing a lossy plan with the lossless entry point.

use std::ffi::c_void;
use std::ptr;
use std::sync::Arc;

use cuda_core::{Context, DeviceBuffer, Stream};
use cuda_core_ext::same_context;
use cuda_core_ext::teardown::{self, Library};
use nvcomp_sys as sys;

use crate::{BitcompMode, Error, Result, check_bitcomp, same_ctx, within};

/// Element type for native Bitcomp plans (`bitcompDataType_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BitcompDataType {
    /// Unsigned 8-bit integer.
    U8,
    /// Signed 8-bit integer.
    I8,
    /// Unsigned 16-bit integer.
    U16,
    /// Signed 16-bit integer.
    I16,
    /// Unsigned 32-bit integer.
    U32,
    /// Signed 32-bit integer.
    I32,
    /// Unsigned 64-bit integer.
    U64,
    /// Signed 64-bit integer.
    I64,
    /// 16-bit IEEE float.
    F16,
    /// 32-bit IEEE float.
    F32,
    /// 64-bit IEEE float.
    F64,
}

impl BitcompDataType {
    /// Converts to `bitcompDataType_t`.
    pub const fn to_sys(self) -> sys::bitcompDataType_t {
        match self {
            Self::U8 => sys::bitcompDataType_t_BITCOMP_UNSIGNED_8BIT,
            Self::I8 => sys::bitcompDataType_t_BITCOMP_SIGNED_8BIT,
            Self::U16 => sys::bitcompDataType_t_BITCOMP_UNSIGNED_16BIT,
            Self::I16 => sys::bitcompDataType_t_BITCOMP_SIGNED_16BIT,
            Self::U32 => sys::bitcompDataType_t_BITCOMP_UNSIGNED_32BIT,
            Self::I32 => sys::bitcompDataType_t_BITCOMP_SIGNED_32BIT,
            Self::U64 => sys::bitcompDataType_t_BITCOMP_UNSIGNED_64BIT,
            Self::I64 => sys::bitcompDataType_t_BITCOMP_SIGNED_64BIT,
            Self::F16 => sys::bitcompDataType_t_BITCOMP_FP16_DATA,
            Self::F32 => sys::bitcompDataType_t_BITCOMP_FP32_DATA,
            Self::F64 => sys::bitcompDataType_t_BITCOMP_FP64_DATA,
        }
    }

    /// Element byte size.
    pub const fn elem_bytes(self) -> usize {
        match self {
            Self::U8 | Self::I8 => 1,
            Self::U16 | Self::I16 | Self::F16 => 2,
            Self::U32 | Self::I32 | Self::F32 => 4,
            Self::U64 | Self::I64 | Self::F64 => 8,
        }
    }
}

/// Algorithm selector for native Bitcomp plans (`bitcompAlgorithm_t`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum BitcompAlgo {
    /// Default balanced compression algorithm (`BITCOMP_DEFAULT_ALGO`).
    #[default]
    Default,
    /// Sparse-optimized algorithm (`BITCOMP_SPARSE_ALGO`).
    Sparse,
}

impl BitcompAlgo {
    /// Converts to `bitcompAlgorithm_t`.
    pub const fn to_sys(self) -> sys::bitcompAlgorithm_t {
        match self {
            Self::Default => sys::bitcompAlgorithm_t_BITCOMP_DEFAULT_ALGO,
            Self::Sparse => sys::bitcompAlgorithm_t_BITCOMP_SPARSE_ALGO,
        }
    }
}

/// Owned `bitcompHandle_t` plan tied to one [`Context`].
pub struct BitcompPlan {
    raw: sys::bitcompHandle_t,
    ctx: Arc<Context>,
    uncompressed_bytes: usize,
    mode: BitcompMode,
    stream: Option<Arc<Stream>>,
}

// SAFETY: `BitcompPlan` mutates its internal handle only via `&mut self` (and is `!Sync`), so
// external synchronization holds when moved across threads.
unsafe impl Send for BitcompPlan {}

impl BitcompPlan {
    /// Queries worst-case output buffer size in bytes for `nbytes` uncompressed bytes.
    pub fn max_buflen(nbytes: usize) -> Result<usize> {
        sys::load()?;
        // SAFETY: `bitcompMaxBuflen` is a pure integer query.
        Ok(unsafe { sys::bitcompMaxBuflen(nbytes)? })
    }

    /// Creates a native Bitcomp plan for `uncompressed_bytes` using explicit `dtype`, `mode`,
    /// and `algo`.
    pub fn new(
        ctx: &Arc<Context>,
        uncompressed_bytes: usize,
        dtype: BitcompDataType,
        mode: BitcompMode,
        algo: BitcompAlgo,
    ) -> Result<Self> {
        if uncompressed_bytes == 0 || !uncompressed_bytes.is_multiple_of(dtype.elem_bytes()) {
            return Err(Error::Invalid(
                "bitcomp input size must be non-zero and element-aligned",
            ));
        }
        sys::load()?;
        ctx.bind_to_thread()?;
        let mut raw: sys::bitcompHandle_t = ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer and `ctx` is active on this thread.
        let status = unsafe {
            sys::bitcompCreatePlan(
                &mut raw,
                uncompressed_bytes,
                dtype.to_sys(),
                mode.to_sys(),
                algo.to_sys(),
            )
        };
        check_bitcomp("bitcompCreatePlan", status)?;
        Ok(Self {
            raw,
            ctx: ctx.clone(),
            uncompressed_bytes,
            mode,
            stream: None,
        })
    }

    /// Binds `stream` to this Bitcomp plan.
    pub fn set_stream(&mut self, stream: &Arc<Stream>) -> Result<()> {
        same_ctx(stream, &self.ctx)?;
        self.ctx.bind_to_thread()?;
        // SAFETY: `self.raw` is a valid plan and `stream.cu_stream()` belongs to `self.ctx`.
        let status = unsafe { sys::bitcompSetStream(self.raw, stream.cu_stream()) };
        check_bitcomp("bitcompSetStream", status)?;
        self.stream = Some(stream.clone());
        Ok(())
    }

    fn ensure_stream(&mut self, stream: &Arc<Stream>) -> Result<()> {
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

    /// Compresses `input` losslessly into `output` on `stream`, checking `budget` against
    /// required output capacity.
    ///
    /// Fails with [`Error::Unsupported`] for a plan created with a lossy [`BitcompMode`].
    pub fn compress_lossless_async(
        &mut self,
        stream: &Arc<Stream>,
        input: &DeviceBuffer<u8>,
        output: &mut DeviceBuffer<u8>,
        budget: Option<usize>,
    ) -> Result<()> {
        same_ctx(stream, &self.ctx)?;
        if self.mode != BitcompMode::Lossless {
            return Err(Error::Unsupported(
                "lossy Bitcomp plans need the lossy compression entry points, which are not exposed",
            ));
        }
        if !same_context(input.context(), &self.ctx) || !same_context(output.context(), &self.ctx) {
            return Err(Error::Invalid(
                "bitcomp input/output belongs to another context",
            ));
        }
        if input.num_bytes() != self.uncompressed_bytes {
            return Err(Error::Invalid("bitcomp input size does not match plan"));
        }
        let max_out = Self::max_buflen(self.uncompressed_bytes)?;
        within(budget, max_out)?;
        if output.num_bytes() < max_out {
            return Err(Error::Invalid(
                "bitcomp output buffer smaller than bitcompMaxBuflen",
            ));
        }
        if !output.cu_deviceptr().is_multiple_of(8) {
            return Err(Error::Invalid(
                "bitcomp output buffer must be 64-bit aligned",
            ));
        }
        self.ensure_stream(stream)?;
        // SAFETY: context and stream are bound, input/output extents and 64-bit alignment checked.
        let status = unsafe {
            sys::bitcompCompressLossless(
                self.raw,
                input.cu_deviceptr() as *const c_void,
                output.cu_deviceptr() as *mut c_void,
            )
        };
        check_bitcomp("bitcompCompressLossless", status)
    }

    /// Decompresses `input` into `output` on `stream`.
    ///
    /// `bitcompUncompress` trusts the stream header: a truncated input makes the kernel read past
    /// the buffer and fault (CUDA error 700, which poisons the context). Before queuing the kernel
    /// this method therefore copies the [`BITCOMP_HEADER_BYTES`]-byte stream header to the host on
    /// `stream` and parses it with `bitcompGetCompressedInfo`. That copy is a **blocking** step:
    /// it waits for all work already queued on `stream` (pageable device-to-host copies return
    /// only after completion), so this call is not fully asynchronous.
    ///
    /// Fails with [`Error::Invalid`] if `input` is shorter than the header, shorter than the
    /// compressed size the header declares, or declares an uncompressed size other than the
    /// plan's; with [`Error::BitcompStatus`] if the header is not a Bitcomp header.
    pub fn uncompress_async(
        &mut self,
        stream: &Arc<Stream>,
        input: &DeviceBuffer<u8>,
        output: &mut DeviceBuffer<u8>,
    ) -> Result<()> {
        same_ctx(stream, &self.ctx)?;
        if !same_context(input.context(), &self.ctx) || !same_context(output.context(), &self.ctx) {
            return Err(Error::Invalid(
                "bitcomp input/output belongs to another context",
            ));
        }
        if !input.cu_deviceptr().is_multiple_of(8) {
            return Err(Error::Invalid(
                "bitcomp compressed input must be 64-bit aligned",
            ));
        }
        if output.num_bytes() < self.uncompressed_bytes {
            return Err(Error::Invalid(
                "bitcomp output buffer smaller than plan uncompressed_bytes",
            ));
        }
        if input.num_bytes() < BITCOMP_HEADER_BYTES {
            return Err(Error::Invalid(
                "bitcomp compressed input is shorter than the stream header",
            ));
        }
        self.ensure_stream(stream)?;
        self.check_stream_header(stream, input)?;
        // SAFETY: context and stream are bound, buffer extents/alignment were verified, and the
        // stream header declares a compressed size within `input` and the plan's uncompressed size.
        let status = unsafe {
            sys::bitcompUncompress(
                self.raw,
                input.cu_deviceptr() as *const c_void,
                output.cu_deviceptr() as *mut c_void,
            )
        };
        check_bitcomp("bitcompUncompress", status)
    }

    /// Reads the stream header of `input` on `stream` (blocking) and checks that its declared
    /// compressed size fits in `input` and its uncompressed size matches the plan.
    fn check_stream_header(&self, stream: &Stream, input: &DeviceBuffer<u8>) -> Result<()> {
        let mut header = [0u8; BITCOMP_HEADER_BYTES];
        // SAFETY: the context is bound (`ensure_stream`), `input` holds at least
        // `BITCOMP_HEADER_BYTES` bytes (checked by the caller) and `header` has that capacity. The
        // copy is ordered after earlier work on `stream`; a device-to-pageable-host copy returns
        // only after it completed, and the stream is synchronized before `header` is read.
        unsafe {
            cuda_core::memory::memcpy_dtoh_async(
                header.as_mut_ptr(),
                input.cu_deviceptr(),
                BITCOMP_HEADER_BYTES,
                stream.cu_stream(),
            )?;
        }
        stream.synchronize()?;
        let mut compressed = input.num_bytes();
        let mut uncompressed = 0usize;
        let mut dtype: sys::bitcompDataType_t = 0;
        let mut mode: sys::bitcompMode_t = 0;
        let mut algo: sys::bitcompAlgorithm_t = 0;
        // SAFETY: `header` is a host buffer holding the complete stream header (the query only
        // reads the header; the API accepts host pointers), and every out-pointer is valid.
        let status = unsafe {
            sys::bitcompGetCompressedInfo(
                header.as_ptr().cast(),
                &mut compressed,
                &mut uncompressed,
                &mut dtype,
                &mut mode,
                &mut algo,
            )
        };
        // `bitcompGetCompressedInfo` reports a buffer smaller than the declared compressed size
        // as BITCOMP_INVALID_PARAMETER (bitcomp.h:399-401).
        if status == sys::bitcompResult_t_BITCOMP_INVALID_PARAMETER
            || compressed > input.num_bytes()
        {
            return Err(Error::Invalid(
                "bitcomp compressed input is shorter than the size its stream header declares",
            ));
        }
        check_bitcomp("bitcompGetCompressedInfo", status)?;
        if uncompressed != self.uncompressed_bytes {
            return Err(Error::Invalid(
                "bitcomp stream header declares an uncompressed size other than the plan's",
            ));
        }
        Ok(())
    }
}

/// Size of the native Bitcomp stream header in bytes.
///
/// Not published in `bitcomp.h`; established on nvCOMP 6 by probing `bitcompGetCompressedInfo`:
/// a header whose fields after the `NVZ\0` magic are all zero reports a compressed size of 32
/// (an empty, header-only stream), and the reported sizes are stable once the first 24 bytes
/// are present. The smallest real stream (8 input bytes) is 56 bytes.
pub const BITCOMP_HEADER_BYTES: usize = 32;

impl Drop for BitcompPlan {
    fn drop(&mut self) {
        // A failed bind is put back on the context by `teardown::bind`; the plan then leaks.
        if !teardown::bind(&self.ctx) {
            return;
        }
        // SAFETY: `self.raw` was initialized by `bitcompCreatePlan` and is destroyed once.
        let status = unsafe { sys::bitcompDestroyPlan(self.raw) };
        teardown::check(
            Library::Nvcomp,
            "bitcompDestroyPlan",
            i64::from(status),
            i64::from(sys::bitcompResult_t_BITCOMP_SUCCESS),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_bitcomp_dtype_mapping() {
        assert_eq!(BitcompDataType::U32.elem_bytes(), 4);
        assert_eq!(BitcompDataType::F64.elem_bytes(), 8);
        assert_eq!(
            BitcompAlgo::Default.to_sys(),
            sys::bitcompAlgorithm_t_BITCOMP_DEFAULT_ALGO
        );
    }
}
