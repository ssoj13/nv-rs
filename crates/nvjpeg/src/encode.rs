//! [`EncoderParams`] and [`EncoderState`]: JPEG encoding and bitstream retrieval.
//!
//! **What.** Configures quality (`1..=100`), chroma subsampling, SOF encoding, optimized Huffman,
//! and restart intervals, encodes planar or interleaved device images on a `Stream`, and
//! retrieves the compressed JPEG bitstream into a caller-owned `Vec<u8>` (zero-alloc on reuse).

use crate::{
    ChromaSubsampling, Error, Handle, ImagePlanes, InputFormat, JpegEncoding, Result, check, sys,
    validate_yuv_source,
};
use cuda_core::Stream;
use std::ffi::c_int;
use std::sync::Arc;

/// JPEG encoder parameters (`nvjpegEncoderParams_t`).
pub struct EncoderParams {
    raw: sys::nvjpegEncoderParams_t,
    handle: Arc<Handle>,
}

// SAFETY: `nvjpegEncoderParams_t` is owned exclusively; `&mut self` serializes mutation.
unsafe impl Send for EncoderParams {}

impl EncoderParams {
    /// Creates encoder parameters (`nvjpegEncoderParamsCreate`).
    pub fn new(handle: &Arc<Handle>, stream: &Stream) -> Result<Self> {
        handle.cuda().bind_to_thread()?;
        let mut raw: sys::nvjpegEncoderParams_t = std::ptr::null_mut();
        // SAFETY: `handle` and `stream` are valid on the bound context.
        check("nvjpegEncoderParamsCreate", unsafe {
            sys::nvjpegEncoderParamsCreate(handle.raw(), &mut raw, stream.cu_stream())
        })?;
        Ok(Self {
            raw,
            handle: handle.clone(),
        })
    }

    /// Sets JPEG quality in `1..=100` (`nvjpegEncoderParamsSetQuality`).
    pub fn set_quality(&mut self, quality: u8, stream: &Stream) -> Result<()> {
        if !(1..=100).contains(&quality) {
            return Err(Error::InvalidInput("JPEG quality must be in 1..=100"));
        }
        self.handle.cuda().bind_to_thread()?;
        // SAFETY: `self.raw` and `stream` are live.
        check("nvjpegEncoderParamsSetQuality", unsafe {
            sys::nvjpegEncoderParamsSetQuality(self.raw, c_int::from(quality), stream.cu_stream())
        })
    }

    /// Sets output chroma subsampling (`nvjpegEncoderParamsSetSamplingFactors`).
    pub fn set_sampling_factors(
        &mut self,
        subsampling: ChromaSubsampling,
        stream: &Stream,
    ) -> Result<()> {
        let css = subsampling.to_sys()?;
        self.handle.cuda().bind_to_thread()?;
        // SAFETY: `self.raw` and `stream` are live.
        check("nvjpegEncoderParamsSetSamplingFactors", unsafe {
            sys::nvjpegEncoderParamsSetSamplingFactors(self.raw, css, stream.cu_stream())
        })
    }

    /// Sets JPEG encoding mode (`nvjpegEncoderParamsSetEncoding`).
    pub fn set_encoding(&mut self, encoding: JpegEncoding, stream: &Stream) -> Result<()> {
        let enc = encoding.to_sys()?;
        self.handle.cuda().bind_to_thread()?;
        // SAFETY: `self.raw` and `stream` are live.
        check("nvjpegEncoderParamsSetEncoding", unsafe {
            sys::nvjpegEncoderParamsSetEncoding(self.raw, enc, stream.cu_stream())
        })
    }

    /// Enables or disables optimized Huffman tables (`nvjpegEncoderParamsSetOptimizedHuffman`).
    pub fn set_optimized_huffman(&mut self, optimized: bool, stream: &Stream) -> Result<()> {
        self.handle.cuda().bind_to_thread()?;
        // SAFETY: `self.raw` and `stream` are live.
        check("nvjpegEncoderParamsSetOptimizedHuffman", unsafe {
            sys::nvjpegEncoderParamsSetOptimizedHuffman(
                self.raw,
                c_int::from(optimized),
                stream.cu_stream(),
            )
        })
    }

    /// Sets restart interval (`nvjpegEncoderParamsSetRestartInterval`).
    pub fn set_restart_interval(&mut self, interval: u32, stream: &Stream) -> Result<()> {
        self.handle.cuda().bind_to_thread()?;
        // SAFETY: `self.raw` and `stream` are live.
        check("nvjpegEncoderParamsSetRestartInterval", unsafe {
            sys::nvjpegEncoderParamsSetRestartInterval(self.raw, interval, stream.cu_stream())
        })
    }

    /// Returns the worst-case compressed bitstream size (`nvjpegEncodeGetBufferSize`).
    pub fn max_bitstream_size(&self, width: u32, height: u32) -> Result<usize> {
        let w = i32::try_from(width)
            .ok()
            .filter(|&v| v > 0)
            .ok_or(Error::InvalidInput("width must be 1..=i32::MAX"))?;
        let h = i32::try_from(height)
            .ok()
            .filter(|&v| v > 0)
            .ok_or(Error::InvalidInput("height must be 1..=i32::MAX"))?;
        let mut len = 0usize;
        // SAFETY: `self.handle`, `self.raw`, and `len` are valid.
        check("nvjpegEncodeGetBufferSize", unsafe {
            sys::nvjpegEncodeGetBufferSize(self.handle.raw(), self.raw, w, h, &mut len)
        })?;
        Ok(len)
    }

    pub(crate) fn raw(&self) -> sys::nvjpegEncoderParams_t {
        self.raw
    }
}

impl Drop for EncoderParams {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            self.handle.bind_for_drop();
            // SAFETY: `self.raw` is owned by `self` and `self.handle` is still alive.
            let code = unsafe { sys::nvjpegEncoderParamsDestroy(self.raw) };
            self.handle.record_drop_err(code);
        }
    }
}

/// JPEG encoder state (`nvjpegEncoderState_t`).
pub struct EncoderState {
    raw: sys::nvjpegEncoderState_t,
    handle: Arc<Handle>,
    /// Whether the last successful encode still has a bitstream to hand out. nvJPEG hands an
    /// encode's bitstream out once: a second `nvjpegEncodeRetrieveBitstream` succeeds with the
    /// same length but writes nothing, so a repeated retrieval is rejected instead.
    bitstream_ready: bool,
}

// SAFETY: `nvjpegEncoderState_t` is owned exclusively; `&mut self` serializes encode calls.
unsafe impl Send for EncoderState {}

impl EncoderState {
    /// Creates an encoder state (`nvjpegEncoderStateCreate`).
    pub fn new(handle: &Arc<Handle>, stream: &Stream) -> Result<Self> {
        handle.cuda().bind_to_thread()?;
        let mut raw: sys::nvjpegEncoderState_t = std::ptr::null_mut();
        // SAFETY: `handle` and `stream` are valid on the bound context.
        check("nvjpegEncoderStateCreate", unsafe {
            sys::nvjpegEncoderStateCreate(handle.raw(), &mut raw, stream.cu_stream())
        })?;
        Ok(Self {
            raw,
            handle: handle.clone(),
            bitstream_ready: false,
        })
    }

    /// Encodes an RGB/BGR/YUV image on `stream` (`nvjpegEncodeImage`).
    pub fn encode_image(
        &mut self,
        params: &EncoderParams,
        src: &ImagePlanes<'_>,
        format: InputFormat,
        width: u32,
        height: u32,
        stream: &Stream,
    ) -> Result<()> {
        let w = i32::try_from(width)
            .ok()
            .filter(|&v| v > 0)
            .ok_or(Error::InvalidInput("width must be 1..=i32::MAX"))?;
        let h = i32::try_from(height)
            .ok()
            .filter(|&v| v > 0)
            .ok_or(Error::InvalidInput("height must be 1..=i32::MAX"))?;
        format.validate_layout(src.layout(), width, height)?;
        self.handle.cuda().bind_to_thread()?;
        self.bitstream_ready = false;
        // SAFETY: `src` has validated device plane pointers matching `format`, and `params`/`stream` are live.
        check("nvjpegEncodeImage", unsafe {
            sys::nvjpegEncodeImage(
                self.handle.raw(),
                self.raw,
                params.raw(),
                src.as_raw(),
                format.to_sys(),
                w,
                h,
                stream.cu_stream(),
            )
        })?;
        self.bitstream_ready = true;
        Ok(())
    }

    /// Encodes a YUV image with explicit `subsampling` on `stream` (`nvjpegEncodeYUV`).
    pub fn encode_yuv(
        &mut self,
        params: &EncoderParams,
        src: &ImagePlanes<'_>,
        subsampling: ChromaSubsampling,
        width: u32,
        height: u32,
        stream: &Stream,
    ) -> Result<()> {
        let css = subsampling.to_sys()?;
        let w = i32::try_from(width)
            .ok()
            .filter(|&v| v > 0)
            .ok_or(Error::InvalidInput("width must be 1..=i32::MAX"))?;
        let h = i32::try_from(height)
            .ok()
            .filter(|&v| v > 0)
            .ok_or(Error::InvalidInput("height must be 1..=i32::MAX"))?;
        validate_yuv_source(src.layout(), subsampling, width, height)?;
        self.handle.cuda().bind_to_thread()?;
        self.bitstream_ready = false;
        // SAFETY: `src` planes cover the Y and chroma extents nvJPEG reads for `subsampling`
        // (checked above), and `params`/`stream` are live.
        check("nvjpegEncodeYUV", unsafe {
            sys::nvjpegEncodeYUV(
                self.handle.raw(),
                self.raw,
                params.raw(),
                src.as_raw(),
                css,
                w,
                h,
                stream.cu_stream(),
            )
        })?;
        self.bitstream_ready = true;
        Ok(())
    }

    /// Retrieves the compressed JPEG bitstream into `out`, reusing its capacity when sufficient.
    ///
    /// Each encode's bitstream can be retrieved once. Fails with [`Error::InvalidInput`] (leaving
    /// `out` untouched) when no encode succeeded since the last successful retrieval.
    pub fn retrieve_bitstream_into(&mut self, out: &mut Vec<u8>, stream: &Stream) -> Result<()> {
        if !self.bitstream_ready {
            return Err(Error::InvalidInput(
                "no encoded bitstream to retrieve: already retrieved, or no encode succeeded",
            ));
        }
        self.handle.cuda().bind_to_thread()?;
        let mut len = 0usize;
        // SAFETY: null `data` queries the exact encoded bitstream length.
        check("nvjpegEncodeRetrieveBitstream", unsafe {
            sys::nvjpegEncodeRetrieveBitstream(
                self.handle.raw(),
                self.raw,
                std::ptr::null_mut(),
                &mut len,
                stream.cu_stream(),
            )
        })?;
        // The bytes are initialized before the call, not left to nvJPEG: a repeated retrieval of
        // the same encode succeeds with the same length but writes nothing (measured on cuda 13,
        // sm_86), so `set_len` over nvJPEG's output would expose uninitialized memory.
        out.clear();
        if len == 0 {
            self.bitstream_ready = false;
            return Ok(());
        }
        out.resize(len, 0);
        let mut written = len;
        // SAFETY: `out` holds `len` initialized bytes and nvJPEG writes at most `written == len`
        // bytes into it.
        check("nvjpegEncodeRetrieveBitstream", unsafe {
            sys::nvjpegEncodeRetrieveBitstream(
                self.handle.raw(),
                self.raw,
                out.as_mut_ptr(),
                &mut written,
                stream.cu_stream(),
            )
        })?;
        if written > len {
            return Err(Error::InvalidInput(
                "nvjpegEncodeRetrieveBitstream reported more bytes than it was given",
            ));
        }
        out.truncate(written);
        self.bitstream_ready = false;
        Ok(())
    }

    /// Retrieves the compressed JPEG bitstream as a new `Vec<u8>`.
    ///
    /// Same once-per-encode rule as [`Self::retrieve_bitstream_into`].
    pub fn retrieve_bitstream(&mut self, stream: &Stream) -> Result<Vec<u8>> {
        let mut out = Vec::new();
        self.retrieve_bitstream_into(&mut out, stream)?;
        Ok(out)
    }
}

impl Drop for EncoderState {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            self.handle.bind_for_drop();
            // SAFETY: `self.raw` is owned by `self` and `self.handle` is still alive.
            let code = unsafe { sys::nvjpegEncoderStateDestroy(self.raw) };
            self.handle.record_drop_err(code);
        }
    }
}
