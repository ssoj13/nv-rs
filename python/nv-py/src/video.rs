//! `nv_rs.nvenc`, `nv_rs.nvdec`, and `nv_rs.nvofa` — hardware video encode, decode, and optical flow.
//!
//! **What.** Python bindings over the `nvenc`, `nvdec` and `nvofa` wrapper crates:
//! - `NvencEncoder` encodes `DeviceBuffer` frames zero-copy (registered CUDA pointers). A frame
//!   NVENC buffers for B-frame reordering (`EncodeOutcome::NeedMoreInput`) stays registered,
//!   mapped and alive in a pending queue together with its output bitstream until a later
//!   `Ready` submission or `flush()` hands out its packet; only then is it unmapped
//!   (`nvEncodeAPI.h`: unmap after `NvEncLockBitstream` returned).
//! - `NvdecDecoder` parses and decodes packets and copies each displayed surface into an owned
//!   `DeviceBuffer` (NV12, P016, YUV444, YUV444 16-bit).
//! - `Nvofa` copies two frames into NVOFA buffers, executes optical flow and copies the flow
//!   field into an owned `DeviceBuffer`, all queued on the session stream without host waits.
//!
//! **Buffer contract.** Every Python buffer argument is validated by `buf_util::check_buf`
//! (dtype, dense layout, size, CUDA context) and every native call that queues work on it runs
//! through `buf_util::enqueue_leased`, so `del buf` right after a call never frees memory the
//! queued work still uses. Result buffers record the stream that produces them.
//! **Used by** the `nv_rs` extension module (`lib.rs`).

use std::collections::VecDeque;
use std::mem::zeroed;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Instant;

use cuda_core::{Context, DeviceBuffer as CudaDeviceBuffer, IntoResult, Stream};
use nv_loader::diag::{self, Level};
use nvdec::{
    ChromaFormat as DecChromaFormat, Codec as DecCodec, DecoderConfig, DisplayInfo, ParserConfig,
    ParserHandler, SurfaceFormat, VideoDecoder, VideoFormat, VideoParser, nv12_frame_bytes,
    p016_frame_u16s,
};
use nvenc::{
    BufferFormat as EncBufferFormat, Codec as EncCodec, EncodeFlags, EncodeInput, EncodeOutcome,
    EncodeSession, MappedResource, OutputBitstream, Preset, RegisteredResource, SessionConfig,
    Tuning,
};
use nvofa::{
    BufferDesc, BufferFormat as OfBufferFormat, BufferUsage, CudaBufferType, ExecuteParams,
    FlowVector, GridSize, InitConfig, OfBuffer, OpticalFlowSession, PerfLevel,
};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

use crate::buf_util::{Access, check_buf, enqueue_leased};
use crate::core::{PyDeviceBuffer, PyStream};
use crate::errors::{ErrorKind, record_and_err};

// ---------------------------------------------------------------------------
// Error mapping & parsing helpers
// ---------------------------------------------------------------------------

py_err_mapper!(map_nvenc_err, "nvenc", nvenc::Error, {
    nvenc::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    nvenc::Error::Invalid(_) => ErrorKind::InvalidArgument,
    nvenc::Error::Driver(_) | nvenc::Error::Status { .. } => ErrorKind::Cuda,
});

py_err_mapper!(map_nvdec_err, "nvdec", nvdec::Error, {
    nvdec::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    nvdec::Error::Invalid(_) => ErrorKind::InvalidArgument,
    nvdec::Error::Driver(_) | nvdec::Error::Status { .. } => ErrorKind::Cuda,
    // A panic inside a parser callback is a bug in this binding, not a CUDA failure.
    nvdec::Error::CallbackPanic(_) => ErrorKind::Runtime,
});

py_err_mapper!(map_nvofa_err, "nvofa", nvofa::Error, {
    nvofa::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    nvofa::Error::Invalid(_) | nvofa::Error::Unsupported { .. } => ErrorKind::InvalidArgument,
    nvofa::Error::Driver(_) | nvofa::Error::Status { .. } => ErrorKind::Cuda,
});

py_parse_enum!(
    parse_enc_codec,
    "nvenc",
    "parse_codec",
    (EncCodec, &'static str),
    "NVENC codec (expected h264, hevc, or av1)",
    {
        "h264" | "avc" => (EncCodec::H264, "h264"),
        "hevc" | "h265" => (EncCodec::Hevc, "hevc"),
        "av1" => (EncCodec::Av1, "av1"),
    }
);

py_parse_enum!(
    parse_preset,
    "nvenc",
    "parse_preset",
    Preset,
    "NVENC preset (expected p1..p7)",
    {
        "p1" => Preset::P1,
        "p2" => Preset::P2,
        "p3" => Preset::P3,
        "p4" => Preset::P4,
        "p5" => Preset::P5,
        "p6" => Preset::P6,
        "p7" => Preset::P7,
    }
);

py_parse_enum!(
    parse_tuning,
    "nvenc",
    "parse_tuning",
    Tuning,
    "NVENC tuning (expected high_quality, low_latency, ultra_low_latency, lossless, or ultra_high_quality)",
    {
        "hq" | "high_quality" => Tuning::HighQuality,
        "ll" | "low_latency" => Tuning::LowLatency,
        "ull" | "ultra_low_latency" => Tuning::UltraLowLatency,
        "lossless" => Tuning::Lossless,
        "uhq" | "ultra_high_quality" => Tuning::UltraHighQuality,
    }
);

py_parse_enum!(
    parse_enc_format,
    "nvenc",
    "parse_format",
    (EncBufferFormat, &'static str),
    "NVENC buffer format",
    {
        "nv12" => (EncBufferFormat::Nv12, "nv12"),
        "yv12" => (EncBufferFormat::Yv12, "yv12"),
        "iyuv" | "i420" => (EncBufferFormat::Iyuv, "iyuv"),
        "yuv444" => (EncBufferFormat::Yuv444, "yuv444"),
        "yuv420_10bit" | "p010" => (EncBufferFormat::Yuv420_10Bit, "yuv420_10bit"),
        "yuv444_10bit" => (EncBufferFormat::Yuv444_10Bit, "yuv444_10bit"),
        "argb" => (EncBufferFormat::Argb, "argb"),
        "abgr" => (EncBufferFormat::Abgr, "abgr"),
        "argb10" => (EncBufferFormat::Argb10, "argb10"),
        "abgr10" => (EncBufferFormat::Abgr10, "abgr10"),
    }
);

py_parse_enum!(
    parse_dec_codec,
    "nvdec",
    "parse_codec",
    (DecCodec, &'static str),
    "NVDEC codec",
    {
        "mpeg1" => (DecCodec::Mpeg1, "mpeg1"),
        "mpeg2" => (DecCodec::Mpeg2, "mpeg2"),
        "mpeg4" => (DecCodec::Mpeg4, "mpeg4"),
        "vc1" => (DecCodec::Vc1, "vc1"),
        "h264" | "avc" => (DecCodec::H264, "h264"),
        "jpeg" | "mjpeg" => (DecCodec::Jpeg, "jpeg"),
        "hevc" | "h265" => (DecCodec::Hevc, "hevc"),
        "vp8" => (DecCodec::Vp8, "vp8"),
        "vp9" => (DecCodec::Vp9, "vp9"),
        "av1" => (DecCodec::Av1, "av1"),
    }
);

py_parse_enum!(
    parse_dec_chroma,
    "nvdec",
    "parse_chroma",
    DecChromaFormat,
    "NVDEC chroma format (expected 420, 422, 444, or monochrome)",
    {
        "monochrome" | "400" => DecChromaFormat::Monochrome,
        "420" | "yuv420" => DecChromaFormat::Yuv420,
        "422" | "yuv422" => DecChromaFormat::Yuv422,
        "444" | "yuv444" => DecChromaFormat::Yuv444,
    }
);

py_parse_enum!(
    parse_dec_surface_format,
    "nvdec",
    "parse_surface_format",
    SurfaceFormat,
    "NVDEC output format (expected nv12, p016, yuv444, or yuv444_16bit)",
    {
        "nv12" => SurfaceFormat::Nv12,
        "p016" | "p010" => SurfaceFormat::P016,
        "yuv444" => SurfaceFormat::Yuv444,
        "yuv444_16bit" => SurfaceFormat::Yuv444_16Bit,
    }
);

fn parse_grid_size(grid: u32) -> PyResult<GridSize> {
    match grid {
        1 => Ok(GridSize::One),
        2 => Ok(GridSize::Two),
        4 => Ok(GridSize::Four),
        other => Err(record_and_err(
            "nvofa",
            "parse_grid_size",
            format!("unsupported NVOFA grid_size {other}; expected 1, 2, or 4"),
            ErrorKind::InvalidArgument,
        )),
    }
}

py_parse_enum!(
    parse_of_format,
    "nvofa",
    "parse_format",
    (OfBufferFormat, &'static str),
    "NVOFA input format (expected grayscale8, nv12, or abgr8)",
    {
        "grayscale8" | "gray8" | "u8" => (OfBufferFormat::Grayscale8, "grayscale8"),
        "nv12" => (OfBufferFormat::Nv12, "nv12"),
        "abgr8" | "rgba8" => (OfBufferFormat::Abgr8, "abgr8"),
    }
);

py_parse_enum!(
    parse_perf_level,
    "nvofa",
    "parse_perf_level",
    PerfLevel,
    "NVOFA perf_level (expected slow, medium, or fast)",
    {
        "slow" => PerfLevel::Slow,
        "medium" => PerfLevel::Medium,
        "fast" => PerfLevel::Fast,
    }
);

/// Queues a 2D device-to-device copy on `stream` (no host wait; callers order or wait for it).
#[expect(
    clippy::too_many_arguments,
    reason = "2D CUDA memcpy requires context, stream, src/dst pointers, pitches, and 2D extents"
)]
fn memcpy_2d_d2d(
    cuda: &Arc<Context>,
    stream: &Stream,
    src_ptr: u64,
    src_pitch: usize,
    dst_ptr: u64,
    dst_pitch: usize,
    width_bytes: usize,
    height: usize,
) -> std::result::Result<(), cuda_core::DriverError> {
    cuda.bind_to_thread()?;
    // SAFETY: all-zero bit pattern is valid for `CUDA_MEMCPY2D`.
    let mut copy: cuda_core::sys::CUDA_MEMCPY2D = unsafe { zeroed() };
    copy.srcMemoryType = cuda_core::sys::CUmemorytype_enum_CU_MEMORYTYPE_DEVICE;
    copy.srcDevice = src_ptr;
    copy.srcPitch = src_pitch;
    copy.dstMemoryType = cuda_core::sys::CUmemorytype_enum_CU_MEMORYTYPE_DEVICE;
    copy.dstDevice = dst_ptr;
    copy.dstPitch = dst_pitch;
    copy.WidthInBytes = width_bytes;
    copy.Height = height;
    // SAFETY: Caller guarantees `src_ptr` and `dst_ptr` cover `height` rows of `width_bytes`
    // within their respective pitches and stay allocated until the copy queued on `stream`
    // completes (leases, owned session buffers, or a host wait by the caller).
    unsafe { cuda_core::sys::cuMemcpy2DAsync_v2(&copy, stream.cu_stream()) }.result()
}

// ---------------------------------------------------------------------------
// nv_rs.nvenc
// ---------------------------------------------------------------------------

/// Whether an NVENC caps query failed because this GPU or driver does not know the codec or the
/// capability (`NV_ENC_ERR_INVALID_PARAM` / `NV_ENC_ERR_UNSUPPORTED_PARAM`); any other failure
/// (driver, memory, device) is a real error and propagates.
fn is_unsupported_caps_status(e: &nvenc::Error) -> bool {
    matches!(
        e,
        nvenc::Error::Status { code, .. }
            if code.raw() == nvenc::sys::_NVENCSTATUS_NV_ENC_ERR_INVALID_PARAM
                || code.raw() == nvenc::sys::_NVENCSTATUS_NV_ENC_ERR_UNSUPPORTED_PARAM
    )
}

/// Queries one capability; `None` when this GPU or driver does not know it.
fn optional_cap(
    session: &EncodeSession,
    codec: EncCodec,
    cap: nvenc::sys::NV_ENC_CAPS,
) -> nvenc::Result<Option<i32>> {
    match session.caps(codec, cap) {
        Ok(v) => Ok(Some(v)),
        Err(e) if is_unsupported_caps_status(&e) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Queries whether `codec` (`"h264"`, `"hevc"`, `"av1"`) is supported on `stream`'s GPU.
#[pyfunction]
#[pyo3(signature = (stream, codec="h264"))]
fn is_codec_supported(py: Python<'_>, stream: &PyStream, codec: &str) -> PyResult<bool> {
    let (enc_codec, _) = parse_enc_codec(codec)?;
    let ctx = stream.inner.context().clone();
    let supported = py
        .detach(move || -> nvenc::Result<bool> {
            let session = EncodeSession::open(&ctx)?;
            Ok(
                optional_cap(&session, enc_codec, nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_WIDTH_MAX)?
                    .is_some_and(|w| w > 0),
            )
        })
        .map_err(|e| map_nvenc_err("is_codec_supported", e))?;
    Ok(supported)
}

/// Queries hardware encoder capabilities for `codec` (`"h264"`, `"hevc"`, `"av1"`).
///
/// A capability this GPU or driver does not report is `None`.
/// @py-return: dict[str, Any]
#[pyfunction]
fn codec_caps(py: Python<'_>, stream: &PyStream, codec: &str) -> PyResult<Py<PyDict>> {
    let (enc_codec, codec_name) = parse_enc_codec(codec)?;
    let ctx = stream.inner.context().clone();
    let t0 = Instant::now();
    let caps = py
        .detach(move || -> nvenc::Result<Option<(i32, [Option<i32>; 8])>> {
            let session = EncodeSession::open(&ctx)?;
            let w_max = match optional_cap(
                &session,
                enc_codec,
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_WIDTH_MAX,
            )? {
                Some(w) if w > 0 => w,
                _ => return Ok(None),
            };
            let mut rest = [None; 8];
            for (slot, cap) in rest.iter_mut().zip([
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_HEIGHT_MAX,
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_WIDTH_MIN,
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_HEIGHT_MIN,
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_NUM_MAX_BFRAMES,
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_10BIT_ENCODE,
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_LOSSLESS_ENCODE,
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_YUV444_ENCODE,
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_NUM_ENCODER_ENGINES,
            ]) {
                *slot = optional_cap(&session, enc_codec, cap)?;
            }
            Ok(Some((w_max, rest)))
        })
        .map_err(|e| map_nvenc_err("codec_caps", e))?;

    diag::record_event(
        Level::Info,
        "nvenc",
        "codec_caps",
        format!("codec={codec_name} supported={}", caps.is_some()),
        t0.elapsed().as_nanos() as u64,
        true,
    );

    let flag = |v: Option<i32>| v.map(|v| v != 0);
    let d = PyDict::new(py);
    d.set_item("codec", codec_name)?;
    if let Some((w_max, [h_max, w_min, h_min, bframes, bit10, lossless, yuv444, engines])) = caps {
        d.set_item("supported", true)?;
        d.set_item("max_width", w_max)?;
        d.set_item("max_height", h_max)?;
        d.set_item("min_width", w_min)?;
        d.set_item("min_height", h_min)?;
        d.set_item("max_bframes", bframes)?;
        d.set_item("support_10bit", flag(bit10))?;
        d.set_item("support_lossless", flag(lossless))?;
        d.set_item("support_yuv444", flag(yuv444))?;
        d.set_item("num_encoder_engines", engines)?;
    } else {
        d.set_item("supported", false)?;
    }
    Ok(d.unbind())
}

/// Bit depth and chroma sampling `format` asks of the encoded stream: `(ten_bit, yuv444)`.
const fn enc_format_needs(format: EncBufferFormat) -> (bool, bool) {
    match format {
        EncBufferFormat::Nv12
        | EncBufferFormat::Yv12
        | EncBufferFormat::Iyuv
        | EncBufferFormat::Argb
        | EncBufferFormat::Abgr => (false, false),
        EncBufferFormat::Yuv444 => (false, true),
        EncBufferFormat::Yuv420_10Bit | EncBufferFormat::Argb10 | EncBufferFormat::Abgr10 => {
            (true, false)
        }
        EncBufferFormat::Yuv444_10Bit => (true, true),
    }
}

/// Element dtype a frame of `format` must carry and its size in bytes.
const fn enc_frame_dtype(format: EncBufferFormat) -> (&'static str, usize) {
    match format {
        EncBufferFormat::Nv12
        | EncBufferFormat::Yv12
        | EncBufferFormat::Iyuv
        | EncBufferFormat::Yuv444
        | EncBufferFormat::Argb
        | EncBufferFormat::Abgr => ("u8", 1),
        EncBufferFormat::Yuv420_10Bit | EncBufferFormat::Yuv444_10Bit => ("u16", 2),
        EncBufferFormat::Argb10 | EncBufferFormat::Abgr10 => ("u32", 4),
    }
}

/// Configures `cfg` so the stream carries the bit depth and chroma sampling of `format`, or fails
/// naming what this GPU / NVENC cannot honour: an explicit 10-bit or 4:4:4 request never
/// silently becomes an 8-bit 4:2:0 stream.
///
/// `cfg` must come from `session.preset_config(codec, ..)` (its codec-specific union arm is the
/// one for `codec`). 8-bit 4:2:0 formats leave the preset untouched.
fn configure_enc_format(
    session: &EncodeSession,
    codec: (EncCodec, &'static str),
    format: (EncBufferFormat, &'static str),
    cfg: &mut nvenc::EncodeConfig,
) -> PyResult<()> {
    const OP: &str = "NvencEncoder::new";
    let ((codec, codec_name), (format, format_name)) = (codec, format);
    let (ten_bit, yuv444) = enc_format_needs(format);
    if !ten_bit && !yuv444 {
        return Ok(());
    }
    let unsupported = |what: &str| {
        record_and_err(
            "nvenc",
            OP,
            format!("format `{format_name}` needs {what}, which `{codec_name}` encoding does not support on this GPU"),
            ErrorKind::InvalidArgument,
        )
    };
    let cap = |c: nvenc::sys::NV_ENC_CAPS| {
        session
            .caps(codec, c)
            .map_err(|e| map_nvenc_err(OP, e))
    };
    if ten_bit && cap(nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_10BIT_ENCODE)? == 0 {
        return Err(unsupported("10-bit encode (NV_ENC_CAPS_SUPPORT_10BIT_ENCODE = 0)"));
    }
    if yuv444 && cap(nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_YUV444_ENCODE)? == 0 {
        return Err(unsupported("4:4:4 encode (NV_ENC_CAPS_SUPPORT_YUV444_ENCODE = 0)"));
    }
    let depth = if ten_bit {
        nvenc::sys::_NV_ENC_BIT_DEPTH_NV_ENC_BIT_DEPTH_10
    } else {
        nvenc::sys::_NV_ENC_BIT_DEPTH_NV_ENC_BIT_DEPTH_8
    };
    // `chroma_format_idc`: 1 = 4:2:0, 3 = 4:4:4.
    let chroma_idc = if yuv444 { 3 } else { 1 };
    match codec {
        EncCodec::H264 => {
            cfg.set_profile(if yuv444 {
                nvenc::guid::H264_HIGH_444
            } else {
                nvenc::guid::H264_HIGH_10
            });
            // SAFETY: `cfg` was returned by `preset_config` for H.264 (function contract), which
            // fills the `h264Config` arm of the union.
            let h264 = unsafe { &mut cfg.raw.encodeCodecConfig.h264Config };
            h264.chromaFormatIDC = chroma_idc;
            h264.inputBitDepth = depth;
            h264.outputBitDepth = depth;
        }
        EncCodec::Hevc => {
            cfg.set_profile(if yuv444 {
                nvenc::guid::HEVC_FREXT
            } else {
                nvenc::guid::HEVC_MAIN10
            });
            // SAFETY: `cfg` was returned by `preset_config` for HEVC (function contract), which
            // fills the `hevcConfig` arm of the union.
            let hevc = unsafe { &mut cfg.raw.encodeCodecConfig.hevcConfig };
            hevc.set_chromaFormatIDC(chroma_idc);
            hevc.inputBitDepth = depth;
            hevc.outputBitDepth = depth;
        }
        EncCodec::Av1 => {
            if yuv444 {
                return Err(unsupported(
                    "4:4:4 encode, and NVENC exposes only the AV1 Main profile (4:2:0)",
                ));
            }
            // SAFETY: `cfg` was returned by `preset_config` for AV1 (function contract), which
            // fills the `av1Config` arm of the union. AV1 Main carries 10-bit 4:2:0.
            let av1 = unsafe { &mut cfg.raw.encodeCodecConfig.av1Config };
            av1.inputBitDepth = depth;
            av1.outputBitDepth = depth;
        }
    }
    Ok(())
}

/// A zero-copy input surface: the registration of a frame's device pointer, its mapping, and
/// the frame itself (which owns the memory NVENC reads).
///
/// `MappedResource` borrows its `RegisteredResource`; the registration therefore lives in a
/// leaked box owned by this struct, and `Drop` unmaps before it unregisters and frees it.
struct MappedInput {
    mapped: Option<MappedResource<'static>>,
    reg: NonNull<RegisteredResource>,
    frame: PyDeviceBuffer,
}

impl MappedInput {
    /// Registers and maps `ptr`, the dense span of `frame` that `check_buf` validated to hold a
    /// tight `width x height` frame of `format` (`>= pitch * format.locked_rows(height)` bytes).
    fn new(
        session: &Arc<EncodeSession>,
        frame: PyDeviceBuffer,
        ptr: u64,
        dims: (u32, u32, u32),
        format: EncBufferFormat,
    ) -> nvenc::Result<Self> {
        let (width, height, pitch) = dims;
        // SAFETY: `ptr` addresses at least `pitch * format.locked_rows(height)` bytes of `frame`
        // on the session's context (validated by `check_buf` against the encoder stream's
        // context); `frame` is stored next to the registration and dropped only after it.
        let reg = unsafe {
            RegisteredResource::from_cuda_ptr(session, ptr, width, height, pitch, format)?
        };
        let mut input = Self {
            mapped: None,
            reg: NonNull::from(Box::leak(Box::new(reg))),
            frame,
        };
        // SAFETY: `reg` comes from `Box::leak` above and is reached only through this mapping
        // until `Drop` drops the mapping and then frees the box; the heap allocation does not
        // move when `MappedInput` moves.
        let res: &'static mut RegisteredResource = unsafe { &mut *input.reg.as_ptr() };
        input.mapped = Some(res.map()?);
        Ok(input)
    }

    fn mapped(&self) -> nvenc::Result<&MappedResource<'static>> {
        self.mapped
            .as_ref()
            .ok_or(nvenc::Error::Invalid("encoder input surface is not mapped"))
    }
}

impl Drop for MappedInput {
    fn drop(&mut self) {
        // Unmap first: the mapping borrows the registration freed below.
        drop(self.mapped.take());
        // SAFETY: `reg` came from `Box::leak` in `MappedInput::new`, its only borrow (`mapped`)
        // is gone, and it is freed exactly once, here. `frame` drops after this body.
        drop(unsafe { Box::from_raw(self.reg.as_ptr()) });
    }
}

/// A submitted frame whose packet has not been retrieved yet.
struct PendingFrame {
    input: MappedInput,
    output: OutputBitstream,
}

struct EncoderInner {
    // Field order is drop order: pending frames (unmap, unregister, release each frame and its
    // output buffer), then spare output buffers, then the session they all hold an `Arc` of.
    /// Submitted frames in submission order; only `NeedMoreInput` frames stay here between calls.
    pending: VecDeque<PendingFrame>,
    /// Output bitstreams whose packets were retrieved, reused by later submissions.
    spare: Vec<OutputBitstream>,
    session: Arc<EncodeSession>,
    frame_idx: u32,
}

// SAFETY: NVENC session, registration, mapping and bitstream handles are only touched while the
// owning `Mutex` is held; every NVENC call is serialized by `EncodeSession` and binds its CUDA
// context. The raw registration pointer is owned by its `MappedInput`.
unsafe impl Send for EncoderInner {}

/// Retrieves the packet of every pending frame in submission order, after a `Ready` submission
/// or EOS. A frame is unmapped and released only after its bitstream lock returned
/// (`nvEncodeAPI.h`, `NvEncUnmapInputResource`). On error the frames not yet retrieved stay
/// pending.
fn drain_pending(st: &mut EncoderInner) -> nvenc::Result<Vec<Vec<u8>>> {
    let mut packets = Vec::with_capacity(st.pending.len());
    while let Some(front) = st.pending.front_mut() {
        // The lock guard is a temporary: the bitstream is unlocked at the end of this statement.
        let packet = front.output.lock()?.data().to_vec();
        let Some(PendingFrame { input, output }) = st.pending.pop_front() else {
            break;
        };
        drop(input);
        st.spare.push(output);
        packets.push(packet);
    }
    Ok(packets)
}

/// Hardware NVENC video encoder (`h264`, `hevc`, `av1`) operating on zero-copy `DeviceBuffer` frames.
#[pyclass(name = "NvencEncoder", module = "nv_rs.nvenc")]
pub struct PyNvencEncoder {
    state: Arc<Mutex<Option<EncoderInner>>>,
    /// Encoder I/O stream: frames are leased on it and must belong to its context.
    stream: Arc<Stream>,
    width: u32,
    height: u32,
    codec_name: &'static str,
    format: EncBufferFormat,
    format_name: &'static str,
}

#[pymethods]
impl PyNvencEncoder {
    /// Opens an encoder. 10-bit and 4:4:4 `format`s configure the matching profile, bit depth and
    /// chroma sampling, or raise `NvInvalidArgumentError` naming what the GPU does not support.
    /// 4:2:0 formats need an even `width` and `height`.
    #[new]
    #[pyo3(signature = (
        stream,
        width,
        height,
        codec="h264",
        preset="p4",
        tuning="high_quality",
        fps_num=30,
        fps_den=1,
        format="nv12",
        gop_length=None,
        bitrate=None
    ))]
    #[expect(
        clippy::too_many_arguments,
        reason = "PyO3 __new__ signature exposes all optional encoder parameters"
    )]
    fn new(
        py: Python<'_>,
        stream: &PyStream,
        width: u32,
        height: u32,
        codec: &str,
        preset: &str,
        tuning: &str,
        fps_num: u32,
        fps_den: u32,
        format: &str,
        gop_length: Option<u32>,
        bitrate: Option<u32>,
    ) -> PyResult<Self> {
        let (enc_codec, codec_name) = parse_enc_codec(codec)?;
        let enc_preset = parse_preset(preset)?;
        let enc_tuning = parse_tuning(tuning)?;
        let (enc_format, format_name) = parse_enc_format(format)?;
        let chroma_420 = matches!(
            enc_format,
            EncBufferFormat::Nv12
                | EncBufferFormat::Yv12
                | EncBufferFormat::Iyuv
                | EncBufferFormat::Yuv420_10Bit
        );
        if chroma_420 && (!width.is_multiple_of(2) || !height.is_multiple_of(2)) {
            return Err(record_and_err(
                "nvenc",
                "NvencEncoder::new",
                format!("4:2:0 format `{format_name}` needs an even width and height, got {width}x{height}"),
                ErrorKind::InvalidArgument,
            ));
        }
        let s_inner = stream.inner.clone();
        let t0 = Instant::now();

        let inner = py.detach(|| -> PyResult<EncoderInner> {
            let enc_err = |e: nvenc::Error| map_nvenc_err("NvencEncoder::new", e);
            let mut session = EncodeSession::open(s_inner.context()).map_err(enc_err)?;
            let mut enc_cfg = session
                .preset_config(enc_codec, enc_preset, enc_tuning)
                .map_err(enc_err)?;
            configure_enc_format(
                &session,
                (enc_codec, codec_name),
                (enc_format, format_name),
                &mut enc_cfg,
            )?;
            if let Some(gop) = gop_length {
                enc_cfg.set_gop_length(gop);
            }
            if let Some(bps) = bitrate {
                enc_cfg.set_average_bitrate(bps);
            }
            let mut cfg = SessionConfig::new(enc_codec, width, height, fps_num, fps_den);
            cfg.tuning = enc_tuning;
            cfg.preset = enc_preset;
            cfg.encode_config = Some(enc_cfg);
            session.initialize(&cfg).map_err(enc_err)?;
            // The session retains both streams for as long as NVENC may use their handles.
            session
                .set_io_streams(&s_inner, &s_inner)
                .map_err(enc_err)?;
            let output = OutputBitstream::new(&session).map_err(enc_err)?;
            Ok(EncoderInner {
                pending: VecDeque::new(),
                spare: vec![output],
                session,
                frame_idx: 0,
            })
        })?;

        diag::record_event(
            Level::Info,
            "nvenc",
            "encoder_new",
            format!("{codec_name} {width}x{height} fmt={format_name}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(Self {
            state: Arc::new(Mutex::new(Some(inner))),
            stream: stream.inner.clone(),
            width,
            height,
            codec_name,
            format: enc_format,
            format_name,
        })
    }

    /// Encodes one GPU `DeviceBuffer` frame and returns the packets it completed, concatenated
    /// in output order.
    ///
    /// Returns empty `bytes` when NVENC buffered the frame for B-frame reordering; its packet
    /// comes with a later `encode` or with `flush()`. The encoder keeps a buffered frame alive,
    /// but its contents must not change until its packet is returned. Frame dtype: `u8` for
    /// 8-bit formats, `u16` for `yuv420_10bit` / `yuv444_10bit`, `u32` for `argb10` / `abgr10`.
    #[pyo3(signature = (frame, force_idr=false, output_sps_pps=false))]
    fn encode<'py>(
        &self,
        py: Python<'py>,
        frame: &PyDeviceBuffer,
        force_idr: bool,
        output_sps_pps: bool,
    ) -> PyResult<Bound<'py, PyBytes>> {
        let pitch = self.format.tight_pitch(self.width).ok_or_else(|| {
            record_and_err(
                "nvenc",
                "encode",
                "input pitch overflow",
                ErrorKind::InvalidArgument,
            )
        })?;
        if !pitch.is_multiple_of(4) {
            return Err(record_and_err(
                "nvenc",
                "encode",
                format!(
                    "tight row pitch ({pitch} B) must be 4-byte aligned for zero-copy NVENC registration"
                ),
                ErrorKind::InvalidArgument,
            ));
        }
        let expected_bytes = self
            .format
            .tight_frame_bytes(self.width, self.height)
            .ok_or_else(|| {
                record_and_err(
                    "nvenc",
                    "encode",
                    "frame byte size overflow",
                    ErrorKind::InvalidArgument,
                )
            })?;
        let (dtype, itemsize) = enc_frame_dtype(self.format);
        let (dev_ptr, _) = check_buf(
            "nvenc",
            "encode",
            "frame",
            frame,
            &[dtype],
            expected_bytes.div_ceil(itemsize),
            self.stream.context(),
        )?;

        let state = &self.state;
        let stream = &self.stream;
        let dims = (self.width, self.height, pitch);
        let format = self.format;
        let t0 = Instant::now();

        let packets = py.detach(|| -> PyResult<Vec<Vec<u8>>> {
            let enc_err = |e: nvenc::Error| map_nvenc_err("encode", e);
            let mut guard = state
                .lock()
                .map_err(|_| enc_err(nvenc::Error::Invalid("encoder mutex poisoned")))?;
            let st = guard
                .as_mut()
                .ok_or_else(|| enc_err(nvenc::Error::Invalid("encoder is closed")))?;
            let input = MappedInput::new(&st.session, frame.clone(), dev_ptr, dims, format)
                .map_err(enc_err)?;
            let mut output = match st.spare.pop() {
                Some(output) => output,
                None => OutputBitstream::new(&st.session).map_err(enc_err)?,
            };
            let flags = EncodeFlags {
                force_intra: false,
                force_idr,
                output_sps_pps,
            };
            let idx = st.frame_idx;
            let session = &st.session;
            let mut outcome = None;
            let submitted = enqueue_leased(
                "nvenc",
                "encode",
                stream,
                [(&input.frame, Access::Read)],
                || {
                    let mapped = input.mapped().map_err(enc_err)?;
                    outcome = Some(
                        session
                            .encode_picture(
                                EncodeInput::Mapped(mapped),
                                &mut output,
                                idx,
                                u64::from(idx),
                                flags,
                            )
                            .map_err(enc_err)?,
                    );
                    Ok(())
                },
            );
            if let Err(e) = submitted {
                // The frame was not accepted: `input` unmaps and releases it on drop.
                st.spare.push(output);
                return Err(e);
            }
            st.frame_idx = st.frame_idx.wrapping_add(1);
            st.pending.push_back(PendingFrame { input, output });
            match outcome {
                Some(EncodeOutcome::Ready) => drain_pending(st).map_err(enc_err),
                Some(EncodeOutcome::NeedMoreInput) => Ok(Vec::new()),
                None => Err(enc_err(nvenc::Error::Invalid(
                    "nvEncEncodePicture reported no outcome",
                ))),
            }
        })?;

        let bytes = packets.concat();
        diag::record_event(
            Level::Trace,
            "nvenc",
            "encode",
            format!(
                "{} -> {} B in {} packet(s)",
                self.codec_name,
                bytes.len(),
                packets.len()
            ),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(PyBytes::new(py, &bytes))
    }

    /// Flushes End-of-Stream (EOS) and returns the packets of every frame still buffered, one
    /// `bytes` per frame in output order.
    fn flush<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyBytes>>> {
        let state = &self.state;
        let t0 = Instant::now();
        let packets = py
            .detach(|| -> nvenc::Result<Vec<Vec<u8>>> {
                let mut guard = state
                    .lock()
                    .map_err(|_| nvenc::Error::Invalid("encoder mutex poisoned"))?;
                let st = guard
                    .as_mut()
                    .ok_or(nvenc::Error::Invalid("encoder is closed"))?;
                st.session.flush_eos()?;
                drain_pending(st)
            })
            .map_err(|e| map_nvenc_err("flush", e))?;

        diag::record_event(
            Level::Info,
            "nvenc",
            "flush",
            format!("codec={} packets={}", self.codec_name, packets.len()),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(packets.iter().map(|p| PyBytes::new(py, p)).collect())
    }

    /// Explicitly releases the NVENC session and bitstream resources, discarding frames whose
    /// packets were not retrieved (call `flush()` first to keep them).
    fn close(&self, py: Python<'_>) {
        let state = &self.state;
        py.detach(|| {
            // A poisoned lock still holds the session; teardown proceeds without the GIL.
            let inner = state.lock().unwrap_or_else(PoisonError::into_inner).take();
            drop(inner);
        });
    }

    #[getter]
    fn width(&self) -> u32 {
        self.width
    }

    #[getter]
    fn height(&self) -> u32 {
        self.height
    }

    #[getter]
    fn codec(&self) -> &'static str {
        self.codec_name
    }

    #[getter]
    fn format(&self) -> &'static str {
        self.format_name
    }

    #[getter]
    fn is_closed(&self, py: Python<'_>) -> bool {
        let state = &self.state;
        py.detach(|| {
            state
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .is_none()
        })
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
    fn __exit__(
        &self,
        py: Python<'_>,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> bool {
        let _ = (exc_type, exc_val, exc_tb);
        self.close(py);
        false
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!(
            "NvencEncoder(codec='{}', {}x{}, format='{}', closed={})",
            self.codec_name,
            self.width,
            self.height,
            self.format_name,
            self.is_closed(py)
        )
    }
}

// ---------------------------------------------------------------------------
// nv_rs.nvdec
// ---------------------------------------------------------------------------

/// Queries NVDEC hardware capabilities for `codec`, `chroma`, and `bit_depth` (`>= 8`).
/// @py-return: dict[str, Any]
#[pyfunction]
#[pyo3(signature = (stream, codec="h264", chroma="420", bit_depth=8))]
fn decoder_caps(
    py: Python<'_>,
    stream: &PyStream,
    codec: &str,
    chroma: &str,
    bit_depth: u32,
) -> PyResult<Py<PyDict>> {
    let (dec_codec, codec_name) = parse_dec_codec(codec)?;
    let dec_chroma = parse_dec_chroma(chroma)?;
    let bit_depth_minus8 = bit_depth.checked_sub(8).ok_or_else(|| {
        record_and_err(
            "nvdec",
            "decoder_caps",
            format!("bit_depth must be at least 8, got {bit_depth}"),
            ErrorKind::InvalidArgument,
        )
    })?;
    let ctx = stream.inner.context().clone();
    let caps = py
        .detach(move || nvdec::decoder_caps(&ctx, dec_codec, dec_chroma, bit_depth_minus8))
        .map_err(|e| map_nvdec_err("decoder_caps", e))?;

    let d = PyDict::new(py);
    d.set_item("codec", codec_name)?;
    d.set_item("supported", caps.supported)?;
    d.set_item("num_nvdecs", caps.num_nvdecs)?;
    d.set_item("output_format_mask", caps.output_format_mask)?;
    d.set_item("max_width", caps.max_width)?;
    d.set_item("max_height", caps.max_height)?;
    d.set_item("max_mb_count", caps.max_mb_count)?;
    d.set_item("min_width", caps.min_width)?;
    d.set_item("min_height", caps.min_height)?;
    Ok(d.unbind())
}

/// Decoded GPU video frame and display metadata returned by `NvdecDecoder`.
#[pyclass(name = "DecodedFrame", module = "nv_rs.nvdec")]
pub struct PyDecodedFrame {
    buffer: PyDeviceBuffer,
    width: u32,
    height: u32,
    timestamp: i64,
    progressive: bool,
    top_field_first: bool,
    format_name: &'static str,
}

#[pymethods]
impl PyDecodedFrame {
    /// Zero-copy GPU `DeviceBuffer` containing the decoded surface (planes stacked as rows).
    #[getter]
    fn buffer(&self) -> PyDeviceBuffer {
        self.buffer.clone()
    }

    #[getter]
    fn width(&self) -> u32 {
        self.width
    }

    #[getter]
    fn height(&self) -> u32 {
        self.height
    }

    #[getter]
    fn timestamp(&self) -> i64 {
        self.timestamp
    }

    #[getter]
    fn progressive(&self) -> bool {
        self.progressive
    }

    #[getter]
    fn top_field_first(&self) -> bool {
        self.top_field_first
    }

    #[getter]
    fn format(&self) -> &'static str {
        self.format_name
    }

    fn __repr__(&self) -> String {
        format!(
            "DecodedFrame({}x{}, format='{}', ts={}, progressive={})",
            self.width, self.height, self.format_name, self.timestamp, self.progressive
        )
    }
}

struct FrameCollector {
    cuda: Arc<Context>,
    stream: Arc<Stream>,
    requested_format: Option<SurfaceFormat>,
    decoder: Option<Arc<VideoDecoder>>,
    width: u32,
    height: u32,
    frames: Vec<PyDecodedFrame>,
    /// Exception raised while wrapping a displayed frame; the callback itself can only return an
    /// `nvdec::Error`, so the full error is kept here and reported by `take_frames`.
    wrap_err: Option<PyErr>,
}

impl ParserHandler for FrameCollector {
    fn on_sequence(&mut self, fmt: &VideoFormat) -> nvdec::Result<u32> {
        let out_fmt = self
            .requested_format
            .unwrap_or(if fmt.bit_depth_luma_minus8 > 0 {
                SurfaceFormat::P016
            } else {
                SurfaceFormat::Nv12
            });
        let cfg = DecoderConfig::from_video_format(fmt, out_fmt)?;
        self.width = cfg.target_width;
        self.height = cfg.target_height;
        self.decoder = Some(VideoDecoder::new(&self.cuda, &cfg)?);
        Ok(cfg.num_decode_surfaces)
    }

    fn on_decode(&mut self, pic: &nvdec::sys::CUVIDPICPARAMS) -> nvdec::Result<()> {
        let dec = self
            .decoder
            .as_ref()
            .ok_or(nvdec::Error::Invalid("decode before sequence header"))?;
        // SAFETY: `pic` is provided synchronously by `VideoParser::parse_packet`.
        unsafe { dec.decode_picture(pic) }
    }

    fn on_display(&mut self, disp: Option<&DisplayInfo>) -> nvdec::Result<()> {
        let Some(disp) = disp else {
            return Ok(());
        };
        let dec = self
            .decoder
            .as_ref()
            .ok_or(nvdec::Error::Invalid("display before sequence header"))?;
        let mapped = dec.map_frame(disp, Some(self.stream.as_ref()))?;
        let w = mapped.width();
        let h = mapped.height();
        let (w_us, h_us) = (w as usize, h as usize);
        // Planes follow each other at `pitch * height` (the target height is even), so every
        // surface is `rows` pitched rows: Y + interleaved UV for 4:2:0, Y + U + V for 4:4:4.
        let (row_bytes, rows, dtype, format_name) = match dec.output_format() {
            SurfaceFormat::Nv12 => (w_us, h_us + h_us / 2, "u8", "nv12"),
            SurfaceFormat::P016 => (w_us * 2, h_us + h_us / 2, "u16", "p016"),
            SurfaceFormat::Yuv444 => (w_us, h_us * 3, "u8", "yuv444"),
            SurfaceFormat::Yuv444_16Bit => (w_us * 2, h_us * 3, "u16", "yuv444_16bit"),
        };
        let total_bytes = match dec.output_format() {
            SurfaceFormat::Nv12 => nv12_frame_bytes(w, h),
            SurfaceFormat::P016 => p016_frame_u16s(w, h).and_then(|n| n.checked_mul(2)),
            SurfaceFormat::Yuv444 | SurfaceFormat::Yuv444_16Bit => row_bytes.checked_mul(rows),
        }
        .ok_or(nvdec::Error::Invalid("decoded frame byte size overflow"))?;
        // SAFETY: Uninitialized GPU buffer is fully written by the copy below, which completes
        // (host wait) before the buffer is wrapped in a `PyDeviceBuffer`.
        let dst =
            unsafe { CudaDeviceBuffer::<u8>::uninitialized_async(&self.stream, total_bytes) }?;
        memcpy_2d_d2d(
            &self.cuda,
            &self.stream,
            mapped.device_ptr(),
            mapped.pitch() as usize,
            dst.cu_deviceptr(),
            row_bytes,
            row_bytes,
            rows,
        )?;
        // The surface is unmapped when `mapped` drops; the copy must have read it by then.
        self.stream.synchronize()?;
        drop(mapped);
        let buf = match PyDeviceBuffer::from_owned_cuda(
            dst,
            dtype,
            vec![rows as i64, w as i64],
            &self.stream,
        ) {
            Ok(buf) => buf,
            Err(e) => {
                self.wrap_err = Some(e);
                return Err(nvdec::Error::Invalid(
                    "decoded frame could not be wrapped as a DeviceBuffer",
                ));
            }
        };
        self.frames.push(PyDecodedFrame {
            buffer: buf,
            width: w,
            height: h,
            timestamp: disp.timestamp,
            progressive: disp.progressive_frame,
            top_field_first: disp.top_field_first,
            format_name,
        });
        Ok(())
    }
}

struct DecoderParser(VideoParser<FrameCollector>);

// SAFETY: NVDEC parser and decoder handles are protected by a Mutex and bound to
// their owning CUDA context when accessed.
unsafe impl Send for DecoderParser {}

impl std::ops::Deref for DecoderParser {
    type Target = VideoParser<FrameCollector>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for DecoderParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Takes the frames a parse call displayed, so frames of a failed packet never leak into the
/// next call's result, then reports the call's error (a frame-wrapping error first, since the
/// parser's own error for it is only a placeholder).
fn take_frames(
    parser: &mut DecoderParser,
    parsed: nvdec::Result<()>,
    op: &str,
) -> PyResult<Vec<PyDecodedFrame>> {
    let handler = parser.handler_mut();
    let frames = std::mem::take(&mut handler.frames);
    if let Some(e) = handler.wrap_err.take() {
        return Err(e);
    }
    parsed.map_err(|e| map_nvdec_err(op, e))?;
    Ok(frames)
}

/// Hardware NVDEC video decoder producing zero-copy `DecodedFrame` GPU buffers.
#[pyclass(name = "NvdecDecoder", module = "nv_rs.nvdec")]
pub struct PyNvdecDecoder {
    parser: Arc<Mutex<Option<DecoderParser>>>,
    codec_name: &'static str,
}

#[pymethods]
impl PyNvdecDecoder {
    #[new]
    #[pyo3(signature = (stream, codec="h264", output_format=None, max_display_delay=0))]
    fn new(
        py: Python<'_>,
        stream: &PyStream,
        codec: &str,
        output_format: Option<&str>,
        max_display_delay: u32,
    ) -> PyResult<Self> {
        let (dec_codec, codec_name) = parse_dec_codec(codec)?;
        let requested_format = output_format.map(parse_dec_surface_format).transpose()?;
        let s_inner = stream.inner.clone();
        let cuda = s_inner.context().clone();
        let t0 = Instant::now();

        let parser = py
            .detach(move || {
                let mut cfg = ParserConfig::new(dec_codec);
                cfg.max_display_delay = max_display_delay;
                let collector = FrameCollector {
                    cuda: cuda.clone(),
                    stream: s_inner,
                    requested_format,
                    decoder: None,
                    width: 0,
                    height: 0,
                    frames: Vec::new(),
                    wrap_err: None,
                };
                VideoParser::new(&cuda, &cfg, collector).map(DecoderParser)
            })
            .map_err(|e| map_nvdec_err("NvdecDecoder::new", e))?;

        diag::record_event(
            Level::Info,
            "nvdec",
            "decoder_new",
            format!("codec={codec_name}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(Self {
            parser: Arc::new(Mutex::new(Some(parser))),
            codec_name,
        })
    }

    /// Parses and decodes a compressed bitstream packet, returning any newly displayed frames.
    #[pyo3(signature = (data, timestamp=None))]
    fn decode(
        &self,
        py: Python<'_>,
        data: &[u8],
        timestamp: Option<i64>,
    ) -> PyResult<Vec<PyDecodedFrame>> {
        let parser_mx = &self.parser;
        let t0 = Instant::now();

        // `data` borrows an immutable `bytes` object that this call keeps alive.
        let frames = py.detach(|| -> PyResult<Vec<PyDecodedFrame>> {
            let dec_err = |e: nvdec::Error| map_nvdec_err("decode", e);
            let mut guard = parser_mx
                .lock()
                .map_err(|_| dec_err(nvdec::Error::Invalid("decoder mutex poisoned")))?;
            let parser = guard
                .as_mut()
                .ok_or_else(|| dec_err(nvdec::Error::Invalid("decoder is closed")))?;
            let parsed = parser.parse_packet(data, timestamp);
            take_frames(parser, parsed, "decode")
        })?;

        diag::record_event(
            Level::Trace,
            "nvdec",
            "decode",
            format!("codec={} frames={}", self.codec_name, frames.len()),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(frames)
    }

    /// Flushes End-of-Stream (EOS) and returns all remaining buffered frames.
    fn flush(&self, py: Python<'_>) -> PyResult<Vec<PyDecodedFrame>> {
        let parser_mx = &self.parser;
        let t0 = Instant::now();

        let frames = py.detach(|| -> PyResult<Vec<PyDecodedFrame>> {
            let dec_err = |e: nvdec::Error| map_nvdec_err("flush", e);
            let mut guard = parser_mx
                .lock()
                .map_err(|_| dec_err(nvdec::Error::Invalid("decoder mutex poisoned")))?;
            let parser = guard
                .as_mut()
                .ok_or_else(|| dec_err(nvdec::Error::Invalid("decoder is closed")))?;
            let parsed = parser.flush_eos();
            take_frames(parser, parsed, "flush")
        })?;

        diag::record_event(
            Level::Info,
            "nvdec",
            "flush",
            format!("codec={} frames={}", self.codec_name, frames.len()),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(frames)
    }

    /// Explicitly releases the NVDEC parser and decoder resources.
    fn close(&self, py: Python<'_>) {
        let parser_mx = &self.parser;
        py.detach(|| {
            // A poisoned lock still holds the parser; teardown proceeds without the GIL.
            let parser = parser_mx
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .take();
            drop(parser);
        });
    }

    #[getter]
    fn codec(&self) -> &'static str {
        self.codec_name
    }

    #[getter]
    fn width(&self, py: Python<'_>) -> u32 {
        let parser_mx = &self.parser;
        py.detach(|| {
            parser_mx
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .as_ref()
                .map_or(0, |p| p.handler().width)
        })
    }

    #[getter]
    fn height(&self, py: Python<'_>) -> u32 {
        let parser_mx = &self.parser;
        py.detach(|| {
            parser_mx
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .as_ref()
                .map_or(0, |p| p.handler().height)
        })
    }

    #[getter]
    fn is_closed(&self, py: Python<'_>) -> bool {
        let parser_mx = &self.parser;
        py.detach(|| {
            parser_mx
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .is_none()
        })
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
    fn __exit__(
        &self,
        py: Python<'_>,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> bool {
        let _ = (exc_type, exc_val, exc_tb);
        self.close(py);
        false
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!(
            "NvdecDecoder(codec='{}', {}x{}, closed={})",
            self.codec_name,
            self.width(py),
            self.height(py),
            self.is_closed(py)
        )
    }
}

// ---------------------------------------------------------------------------
// nv_rs.nvofa
// ---------------------------------------------------------------------------

struct NvofaInner {
    // Drop `OfBuffer` children before `session`.
    buf_in: OfBuffer,
    buf_ref: OfBuffer,
    buf_out: OfBuffer,
    session: Arc<OpticalFlowSession>,
    stream: Arc<Stream>,
    cuda: Arc<Context>,
}

// SAFETY: NVOFA session and buffers are protected by a Mutex and bound to their
// owning CUDA context when accessed.
unsafe impl Send for NvofaInner {}

/// Queues the copy of a tight `width x height` frame at `src_ptr` into the NVOFA input buffer
/// `dst` on `stream` (the session's I/O stream, so `nvOFExecute` is ordered after it).
fn copy_input_to_of_buffer(
    cuda: &Arc<Context>,
    stream: &Stream,
    src_ptr: u64,
    width: u32,
    height: u32,
    format: OfBufferFormat,
    dst: &OfBuffer,
) -> nvofa::Result<()> {
    let dst_ptr = dst.cu_deviceptr().ok_or(nvofa::Error::Invalid(
        "NVOFA input buffer is not CuDevicePtr",
    ))?;
    let strides = dst.stride_info()?;
    let dst_pitch = strides.planes[0].0 as usize;
    let elem_bytes = format.plane0_elem_bytes();
    let width_bytes = (width as usize)
        .checked_mul(elem_bytes)
        .ok_or(nvofa::Error::Invalid("NVOFA input width_bytes overflow"))?;
    match format {
        OfBufferFormat::Grayscale8 | OfBufferFormat::Abgr8 => {
            memcpy_2d_d2d(
                cuda,
                stream,
                src_ptr,
                width_bytes,
                dst_ptr,
                dst_pitch,
                width_bytes,
                height as usize,
            )?;
        }
        OfBufferFormat::Nv12 => {
            // Plane 0: Y (width x height)
            memcpy_2d_d2d(
                cuda,
                stream,
                src_ptr,
                width_bytes,
                dst_ptr,
                dst_pitch,
                width_bytes,
                height as usize,
            )?;
            // Plane 1: interleaved UV (width x height/2). It starts `strideYInBytes` pitched rows
            // after plane 0: despite its name the field is a row count (NVOF SDK sample
            // `NvOFBufferCudaDevicePtr::UploadData` offsets by `strideXInBytes * strideYInBytes`).
            let (y_pitch, y_rows) = strides.planes[0];
            if y_rows < height || u64::from(y_rows) >= u64::from(y_pitch) * u64::from(height) {
                // A byte count would be `>= pitch * height`; refusing it keeps a misread stride
                // from turning into an out-of-bounds write.
                return Err(nvofa::Error::Invalid(
                    "NVOFA reported an NV12 plane-0 vertical stride that is not a row count of at least the frame height",
                ));
            }
            let uv_src_offset = u64::from(width)
                .checked_mul(u64::from(height))
                .ok_or(nvofa::Error::Invalid("NV12 UV source offset overflow"))?;
            let uv_dst_offset = u64::from(y_pitch)
                .checked_mul(u64::from(y_rows))
                .ok_or(nvofa::Error::Invalid("NV12 UV destination offset overflow"))?;
            let uv_dst_pitch = strides.planes[1].0.max(y_pitch) as usize;
            memcpy_2d_d2d(
                cuda,
                stream,
                src_ptr
                    .checked_add(uv_src_offset)
                    .ok_or(nvofa::Error::Invalid("NV12 UV source pointer overflow"))?,
                width_bytes,
                dst_ptr
                    .checked_add(uv_dst_offset)
                    .ok_or(nvofa::Error::Invalid("NV12 UV destination pointer overflow"))?,
                uv_dst_pitch,
                width_bytes,
                (height as usize) / 2,
            )?;
        }
        _ => {
            return Err(nvofa::Error::Invalid(
                "NVOFA input format must be grayscale8, nv12 or abgr8",
            ));
        }
    }
    Ok(())
}

/// Hardware NVIDIA Optical Flow (NVOFA) session producing S10.5 `(grid_h, grid_w, 2)` `i16` flow buffers.
#[pyclass(name = "Nvofa", module = "nv_rs.nvofa")]
pub struct PyNvofa {
    state: Arc<Mutex<Option<NvofaInner>>>,
    /// Session I/O stream: inputs are leased on it and must belong to its context.
    stream: Arc<Stream>,
    width: u32,
    height: u32,
    grid_size: u32,
    grid_w: u32,
    grid_h: u32,
    input_format: OfBufferFormat,
    format_name: &'static str,
}

#[pymethods]
impl PyNvofa {
    /// Opens an optical-flow session; `nv12` input needs an even `width` and `height`.
    #[new]
    #[pyo3(signature = (stream, width, height, grid_size=4, input_format="grayscale8", perf_level="slow"))]
    fn new(
        py: Python<'_>,
        stream: &PyStream,
        width: u32,
        height: u32,
        grid_size: u32,
        input_format: &str,
        perf_level: &str,
    ) -> PyResult<Self> {
        let grid = parse_grid_size(grid_size)?;
        let (of_format, format_name) = parse_of_format(input_format)?;
        let perf = parse_perf_level(perf_level)?;
        if of_format == OfBufferFormat::Nv12
            && (!width.is_multiple_of(2) || !height.is_multiple_of(2))
        {
            return Err(record_and_err(
                "nvofa",
                "Nvofa::new",
                format!("nv12 input needs an even width and height, got {width}x{height}"),
                ErrorKind::InvalidArgument,
            ));
        }
        let (grid_w, grid_h) = grid.grid_dims(width, height);
        let s_inner = stream.inner.clone();
        let cuda = s_inner.context().clone();
        let t0 = Instant::now();

        let inner = py
            .detach(move || -> nvofa::Result<NvofaInner> {
                let mut session = OpticalFlowSession::open(&cuda)?;
                let mut cfg = InitConfig::new(width, height, grid, of_format);
                cfg.perf_level = perf;
                session.init(&cfg)?;
                session.set_io_streams(&s_inner, &s_inner)?;

                let in_desc = BufferDesc {
                    width,
                    height,
                    usage: BufferUsage::Input,
                    format: of_format,
                };
                let out_desc = BufferDesc {
                    width: grid_w,
                    height: grid_h,
                    usage: BufferUsage::Output,
                    format: OfBufferFormat::Short2,
                };
                let buf_in = OfBuffer::new(&session, in_desc, CudaBufferType::CuDevicePtr)?;
                let buf_ref = OfBuffer::new(&session, in_desc, CudaBufferType::CuDevicePtr)?;
                let buf_out = OfBuffer::new(&session, out_desc, CudaBufferType::CuDevicePtr)?;

                Ok(NvofaInner {
                    buf_in,
                    buf_ref,
                    buf_out,
                    session,
                    stream: s_inner,
                    cuda,
                })
            })
            .map_err(|e| map_nvofa_err("Nvofa::new", e))?;

        diag::record_event(
            Level::Info,
            "nvofa",
            "nvofa_new",
            format!("{width}x{height} grid={grid_size} ({grid_w}x{grid_h}) fmt={format_name}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(Self {
            state: Arc::new(Mutex::new(Some(inner))),
            stream: stream.inner.clone(),
            width,
            height,
            grid_size,
            grid_w,
            grid_h,
            input_format: of_format,
            format_name,
        })
    }

    /// Computes optical flow from `prev_buf` to `curr_buf` (tight `u8` frames) and returns an
    /// `i16` `[grid_h, grid_w, 2]` `DeviceBuffer`.
    ///
    /// All work is queued on the session stream without a host wait; the result records that
    /// stream as its producer.
    #[pyo3(signature = (prev_buf, curr_buf, disable_temporal_hints=false))]
    fn execute(
        &self,
        py: Python<'_>,
        prev_buf: &PyDeviceBuffer,
        curr_buf: &PyDeviceBuffer,
        disable_temporal_hints: bool,
    ) -> PyResult<PyDeviceBuffer> {
        let expected_in_bytes = match self.input_format {
            OfBufferFormat::Grayscale8 | OfBufferFormat::Abgr8 => (self.width as usize)
                .checked_mul(self.height as usize)
                .and_then(|p| p.checked_mul(self.input_format.plane0_elem_bytes())),
            OfBufferFormat::Nv12 => nv12_frame_bytes(self.width, self.height),
            _ => {
                return Err(record_and_err(
                    "nvofa",
                    "execute",
                    "NVOFA input format must be grayscale8, nv12 or abgr8",
                    ErrorKind::InvalidArgument,
                ));
            }
        }
        .ok_or_else(|| {
            record_and_err(
                "nvofa",
                "execute",
                "input frame byte size overflow",
                ErrorKind::InvalidArgument,
            )
        })?;

        let ctx = self.stream.context();
        let (prev_ptr, _) = check_buf(
            "nvofa",
            "execute",
            "prev_buf",
            prev_buf,
            &["u8"],
            expected_in_bytes,
            ctx,
        )?;
        let (curr_ptr, _) = check_buf(
            "nvofa",
            "execute",
            "curr_buf",
            curr_buf,
            &["u8"],
            expected_in_bytes,
            ctx,
        )?;

        let state = &self.state;
        let width = self.width;
        let height = self.height;
        let grid_w = self.grid_w;
        let grid_h = self.grid_h;
        let input_format = self.input_format;
        let t0 = Instant::now();

        let flow = py.detach(|| -> PyResult<PyDeviceBuffer> {
            let of_err = |e: nvofa::Error| map_nvofa_err("execute", e);
            let mut guard = state
                .lock()
                .map_err(|_| of_err(nvofa::Error::Invalid("nvofa mutex poisoned")))?;
            let st = guard
                .as_mut()
                .ok_or_else(|| of_err(nvofa::Error::Invalid("nvofa session is closed")))?;
            let NvofaInner {
                buf_in,
                buf_ref,
                buf_out,
                session,
                stream,
                cuda,
            } = st;
            let (buf_in, buf_ref, session, stream, cuda) =
                (&*buf_in, &*buf_ref, &*session, &*stream, &*cuda);

            enqueue_leased(
                "nvofa",
                "execute",
                stream,
                [(prev_buf, Access::Read), (curr_buf, Access::Read)],
                || {
                    copy_input_to_of_buffer(
                        cuda,
                        stream,
                        prev_ptr,
                        width,
                        height,
                        input_format,
                        buf_ref,
                    )
                    .map_err(of_err)?;
                    copy_input_to_of_buffer(
                        cuda,
                        stream,
                        curr_ptr,
                        width,
                        height,
                        input_format,
                        buf_in,
                    )
                    .map_err(of_err)?;
                    let mut exec = ExecuteParams {
                        input_frame: buf_in,
                        reference_frame: buf_ref,
                        output_buffer: &mut *buf_out,
                        external_hints: None,
                        output_cost: None,
                        disable_temporal_hints,
                        rois: &[],
                    };
                    session.execute(&mut exec).map_err(of_err)
                },
            )?;

            let out_ptr = buf_out.cu_deviceptr().ok_or_else(|| {
                of_err(nvofa::Error::Invalid(
                    "NVOFA output buffer is not CuDevicePtr",
                ))
            })?;
            let out_stride = buf_out.stride_info().map_err(of_err)?.planes[0].0 as usize;
            let row_bytes = (grid_w as usize)
                .checked_mul(size_of::<FlowVector>())
                .ok_or_else(|| of_err(nvofa::Error::Invalid("flow row byte size overflow")))?;
            let total_bytes = row_bytes.checked_mul(grid_h as usize).ok_or_else(|| {
                of_err(nvofa::Error::Invalid("flow buffer byte size overflow"))
            })?;
            // SAFETY: every byte is written by the copy queued below on `stream`, the producer
            // stream recorded on the result, before any reader: Python exports wait for that
            // stream and native calls see the leased write.
            let dst = unsafe { CudaDeviceBuffer::<u8>::uninitialized_async(stream, total_bytes) }
                .map_err(|e| of_err(nvofa::Error::from(e)))?;
            let flow = PyDeviceBuffer::from_owned_cuda(
                dst,
                "i16",
                vec![i64::from(grid_h), i64::from(grid_w), 2],
                stream,
            )?;
            let dst_ptr = flow.raw_device_ptr();
            enqueue_leased(
                "nvofa",
                "execute",
                stream,
                [(&flow, Access::Write)],
                || {
                    memcpy_2d_d2d(
                        cuda,
                        stream,
                        out_ptr,
                        out_stride,
                        dst_ptr,
                        row_bytes,
                        row_bytes,
                        grid_h as usize,
                    )
                    .map_err(|e| of_err(nvofa::Error::from(e)))
                },
            )?;
            Ok(flow)
        })?;

        diag::record_event(
            Level::Trace,
            "nvofa",
            "execute",
            format!("{width}x{height} -> {grid_w}x{grid_h}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(flow)
    }

    /// Explicitly releases the NVOFA session and internal GPU buffers.
    fn close(&self, py: Python<'_>) {
        let state = &self.state;
        py.detach(|| {
            // A poisoned lock still holds the session; teardown proceeds without the GIL.
            let inner = state.lock().unwrap_or_else(PoisonError::into_inner).take();
            drop(inner);
        });
    }

    #[getter]
    fn width(&self) -> u32 {
        self.width
    }

    #[getter]
    fn height(&self) -> u32 {
        self.height
    }

    #[getter]
    fn grid_size(&self) -> u32 {
        self.grid_size
    }

    #[getter]
    fn grid_dims(&self) -> (u32, u32) {
        (self.grid_w, self.grid_h)
    }

    #[getter]
    fn input_format(&self) -> &'static str {
        self.format_name
    }

    #[getter]
    fn is_closed(&self, py: Python<'_>) -> bool {
        let state = &self.state;
        py.detach(|| {
            state
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .is_none()
        })
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
    fn __exit__(
        &self,
        py: Python<'_>,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> bool {
        let _ = (exc_type, exc_val, exc_tb);
        self.close(py);
        false
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!(
            "Nvofa({}x{}, grid_size={}, grid={}x{}, format='{}', closed={})",
            self.width,
            self.height,
            self.grid_size,
            self.grid_w,
            self.grid_h,
            self.format_name,
            self.is_closed(py)
        )
    }
}

// ---------------------------------------------------------------------------
// Module registration
// ---------------------------------------------------------------------------

/// Registers `nv_rs.nvenc`, `nv_rs.nvdec`, and `nv_rs.nvofa` submodules.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    py_register_submodule!(py, parent, "nvenc", |m| {
        m.add_function(wrap_pyfunction!(is_codec_supported, &m)?)?;
        m.add_function(wrap_pyfunction!(codec_caps, &m)?)?;
        m.add_class::<PyNvencEncoder>()?;
    })?;

    py_register_submodule!(py, parent, "nvdec", |m| {
        m.add_function(wrap_pyfunction!(decoder_caps, &m)?)?;
        m.add_class::<PyDecodedFrame>()?;
        m.add_class::<PyNvdecDecoder>()?;
    })?;

    py_register_submodule!(py, parent, "nvofa", |m| {
        m.add_class::<PyNvofa>()?;
    })?;

    Ok(())
}
