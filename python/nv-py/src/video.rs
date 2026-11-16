//! `nv_rs.nvenc`, `nv_rs.nvdec`, and `nv_rs.nvofa` — hardware video encode, decode, and optical flow.

use std::mem::zeroed;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use cuda_core::{Context, DeviceBuffer as CudaDeviceBuffer, IntoResult, Stream};
use nv_loader::diag::{self, Level};
use nvdec::{
    ChromaFormat as DecChromaFormat, Codec as DecCodec, DecoderConfig, DisplayInfo, ParserConfig,
    ParserHandler, SurfaceFormat, VideoDecoder, VideoFormat, VideoParser, nv12_frame_bytes,
    p016_frame_u16s,
};
use nvenc::{
    BufferFormat as EncBufferFormat, Codec as EncCodec, EncodeFlags, EncodeInput, EncodeSession,
    OutputBitstream, Preset, RegisteredResource, SessionConfig, Tuning,
};
use nvofa::{
    BufferDesc, BufferFormat as OfBufferFormat, BufferUsage, CudaBufferType, ExecuteParams,
    FlowVector, GridSize, InitConfig, OfBuffer, OpticalFlowSession, PerfLevel,
};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

use crate::buf_util::validate_dev_buf;
use crate::core::{PyDeviceBuffer, PyStream};
use crate::errors::{ErrorKind, record_and_err};

// ---------------------------------------------------------------------------
// Error mapping & parsing helpers
// ---------------------------------------------------------------------------

py_err_mapper!(map_nvenc_err, "nvenc", nvenc::Error, {
    nvenc::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    nvenc::Error::Invalid(_)
    | nvenc::Error::Unsupported { .. }
    | nvenc::Error::Budget { .. } => ErrorKind::InvalidArgument,
    nvenc::Error::Driver(_) | nvenc::Error::Status { .. } => ErrorKind::Cuda,
});

py_err_mapper!(map_nvdec_err, "nvdec", nvdec::Error, {
    nvdec::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    nvdec::Error::Invalid(_)
    | nvdec::Error::Unsupported { .. }
    | nvdec::Error::Budget { .. } => ErrorKind::InvalidArgument,
    nvdec::Error::Driver(_) | nvdec::Error::Status { .. } | nvdec::Error::CallbackPanic(_) => {
        ErrorKind::Cuda
    }
});

py_err_mapper!(map_nvofa_err, "nvofa", nvofa::Error, {
    nvofa::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    nvofa::Error::Invalid(_)
    | nvofa::Error::Unsupported { .. }
    | nvofa::Error::Budget { .. } => ErrorKind::InvalidArgument,
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

/// Executes a 2D device-to-device copy on `stream` and synchronizes.
#[expect(
    clippy::too_many_arguments,
    reason = "2D CUDA memcpy requires context, stream, src/dst pointers, pitches, and 2D extents"
)]
fn memcpy_2d_d2d_sync(
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
    // within their respective pitches, and `stream.synchronize()` completes before return.
    unsafe { cuda_core::sys::cuMemcpy2DAsync_v2(&copy, stream.cu_stream()) }.result()?;
    stream.synchronize()?;
    Ok(())
}

// ---------------------------------------------------------------------------
// nv_rs.nvenc
// ---------------------------------------------------------------------------

/// Queries whether `codec` (`"h264"`, `"hevc"`, `"av1"`) is supported on `stream`'s GPU.
#[pyfunction]
#[pyo3(signature = (stream, codec="h264"))]
fn is_codec_supported(py: Python<'_>, stream: &PyStream, codec: &str) -> PyResult<bool> {
    let (enc_codec, _) = parse_enc_codec(codec)?;
    let ctx = stream.inner.context().clone();
    let supported = py
        .detach(move || -> nvenc::Result<bool> {
            let session = EncodeSession::open(&ctx)?;
            match session.caps(enc_codec, nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_WIDTH_MAX) {
                Ok(w) => Ok(w > 0),
                Err(nvenc::Error::Status { .. }) => Ok(false),
                Err(e) => Err(e),
            }
        })
        .map_err(|e| map_nvenc_err("is_codec_supported", e))?;
    Ok(supported)
}

/// Queries hardware encoder capabilities for `codec` (`"h264"`, `"hevc"`, `"av1"`).
/// @py-return: dict[str, Any]
#[pyfunction]
fn codec_caps(py: Python<'_>, stream: &PyStream, codec: &str) -> PyResult<Py<PyDict>> {
    let (enc_codec, codec_name) = parse_enc_codec(codec)?;
    let ctx = stream.inner.context().clone();
    let t0 = Instant::now();
    let caps = py
        .detach(move || -> nvenc::Result<Option<[i32; 9]>> {
            let session = EncodeSession::open(&ctx)?;
            let w_max =
                match session.caps(enc_codec, nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_WIDTH_MAX) {
                    Ok(w) if w > 0 => w,
                    Ok(_) | Err(nvenc::Error::Status { .. }) => return Ok(None),
                    Err(e) => return Err(e),
                };
            let h_max = session.caps(enc_codec, nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_HEIGHT_MAX)?;
            let w_min = session.caps(enc_codec, nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_WIDTH_MIN)?;
            let h_min = session.caps(enc_codec, nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_HEIGHT_MIN)?;
            let bframes = session.caps(
                enc_codec,
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_NUM_MAX_BFRAMES,
            )?;
            let bit10 = session.caps(
                enc_codec,
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_10BIT_ENCODE,
            )?;
            let lossless = session.caps(
                enc_codec,
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_LOSSLESS_ENCODE,
            )?;
            let yuv444 = session.caps(
                enc_codec,
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_YUV444_ENCODE,
            )?;
            let engines = session.caps(
                enc_codec,
                nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_NUM_ENCODER_ENGINES,
            )?;
            Ok(Some([
                w_max, h_max, w_min, h_min, bframes, bit10, lossless, yuv444, engines,
            ]))
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

    let d = PyDict::new(py);
    d.set_item("codec", codec_name)?;
    if let Some(
        [
            w_max,
            h_max,
            w_min,
            h_min,
            bframes,
            bit10,
            lossless,
            yuv444,
            engines,
        ],
    ) = caps
    {
        d.set_item("supported", true)?;
        d.set_item("max_width", w_max)?;
        d.set_item("max_height", h_max)?;
        d.set_item("min_width", w_min)?;
        d.set_item("min_height", h_min)?;
        d.set_item("max_bframes", bframes)?;
        d.set_item("support_10bit", bit10 != 0)?;
        d.set_item("support_lossless", lossless != 0)?;
        d.set_item("support_yuv444", yuv444 != 0)?;
        d.set_item("num_encoder_engines", engines)?;
    } else {
        d.set_item("supported", false)?;
    }
    Ok(d.unbind())
}

struct EncoderInner {
    // Drop `output` before `session` so bitstream buffers are destroyed first.
    output: OutputBitstream,
    session: Arc<EncodeSession>,
    frame_idx: u32,
}

// SAFETY: NVENC session and output bitstream handles are protected by a Mutex and
// bound to their owning CUDA context when accessed.
unsafe impl Send for EncoderInner {}

/// Hardware NVENC video encoder (`h264`, `hevc`, `av1`) operating on zero-copy `DeviceBuffer` frames.
#[pyclass(name = "NvencEncoder", module = "nv_rs.nvenc")]
pub struct PyNvencEncoder {
    state: Arc<Mutex<Option<EncoderInner>>>,
    width: u32,
    height: u32,
    codec_name: &'static str,
    format: EncBufferFormat,
    format_name: &'static str,
}

#[pymethods]
impl PyNvencEncoder {
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
        let s_inner = stream.inner.clone();
        let t0 = Instant::now();

        let inner = py
            .detach(move || -> nvenc::Result<EncoderInner> {
                let mut session = EncodeSession::open(s_inner.context())?;
                let mut enc_cfg = session.preset_config(enc_codec, enc_preset, enc_tuning)?;
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
                session.initialize(&cfg)?;
                session.set_io_streams(&s_inner, &s_inner)?;
                let output = OutputBitstream::new(&session)?;
                Ok(EncoderInner {
                    output,
                    session,
                    frame_idx: 0,
                })
            })
            .map_err(|e| map_nvenc_err("NvencEncoder::new", e))?;

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
            width,
            height,
            codec_name,
            format: enc_format,
            format_name,
        })
    }

    /// Encodes one GPU `DeviceBuffer` frame and returns the compressed packet `bytes`.
    #[pyo3(signature = (frame, force_idr=false, output_sps_pps=false))]
    fn encode<'py>(
        &self,
        py: Python<'py>,
        frame: &PyDeviceBuffer,
        force_idr: bool,
        output_sps_pps: bool,
    ) -> PyResult<Bound<'py, PyBytes>> {
        let dev_ptr = frame.raw_device_ptr();
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
        validate_dev_buf(
            "nvenc",
            "encode",
            "input",
            dev_ptr,
            frame.byte_len(),
            expected_bytes,
        )?;

        let state_arc = self.state.clone();
        let width = self.width;
        let height = self.height;
        let format = self.format;
        let t0 = Instant::now();

        let bytes = py
            .detach(move || -> nvenc::Result<Vec<u8>> {
                let mut guard = state_arc
                    .lock()
                    .map_err(|_| nvenc::Error::Invalid("encoder mutex poisoned"))?;
                let st = guard
                    .as_mut()
                    .ok_or(nvenc::Error::Invalid("encoder is closed"))?;
                // SAFETY: `dev_ptr` is a live CUDA device pointer owned or borrowed by `frame`,
                // which remains borrowed for the duration of this synchronous call and has at least
                // `expected_bytes` (`>= pitch * height`) bytes.
                let mut reg = unsafe {
                    RegisteredResource::from_cuda_ptr(
                        &st.session,
                        dev_ptr,
                        width,
                        height,
                        pitch,
                        format,
                    )?
                };
                let mapped = reg.map()?;
                let flags = EncodeFlags {
                    force_intra: false,
                    force_idr,
                    output_sps_pps,
                };
                let idx = st.frame_idx;
                st.session.encode_picture(
                    EncodeInput::Mapped(&mapped),
                    &mut st.output,
                    idx,
                    u64::from(idx),
                    flags,
                )?;
                st.frame_idx = st.frame_idx.wrapping_add(1);
                drop(mapped);
                let pkt = st.output.lock()?;
                Ok(pkt.data().to_vec())
            })
            .map_err(|e| map_nvenc_err("encode", e))?;

        diag::record_event(
            Level::Trace,
            "nvenc",
            "encode",
            format!("{} -> {} B", self.codec_name, bytes.len()),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(PyBytes::new(py, &bytes))
    }

    /// Flushes End-of-Stream (EOS) and returns any remaining encoded packets.
    fn flush<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyBytes>>> {
        let state_arc = self.state.clone();
        let t0 = Instant::now();
        py.detach(move || -> nvenc::Result<()> {
            let mut guard = state_arc
                .lock()
                .map_err(|_| nvenc::Error::Invalid("encoder mutex poisoned"))?;
            let st = guard
                .as_mut()
                .ok_or(nvenc::Error::Invalid("encoder is closed"))?;
            st.session.flush_eos()
        })
        .map_err(|e| map_nvenc_err("flush", e))?;

        diag::record_event(
            Level::Info,
            "nvenc",
            "flush",
            format!("codec={}", self.codec_name),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(Vec::new())
    }

    /// Explicitly releases the NVENC session and bitstream resources.
    fn close(&self) {
        if let Ok(mut guard) = self.state.lock() {
            *guard = None;
        }
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
    fn is_closed(&self) -> bool {
        self.state.lock().map(|g| g.is_none()).unwrap_or(true)
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
    fn __exit__(
        &self,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> bool {
        let _ = (exc_type, exc_val, exc_tb);
        self.close();
        false
    }

    fn __repr__(&self) -> String {
        format!(
            "NvencEncoder(codec='{}', {}x{}, format='{}', closed={})",
            self.codec_name,
            self.width,
            self.height,
            self.format_name,
            self.is_closed()
        )
    }
}

// ---------------------------------------------------------------------------
// nv_rs.nvdec
// ---------------------------------------------------------------------------

/// Queries NVDEC hardware capabilities for `codec`, `chroma`, and `bit_depth`.
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
    let bit_depth_minus8 = bit_depth.saturating_sub(8);
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
    /// Zero-copy GPU `DeviceBuffer` containing the decoded surface.
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
    device_id: i32,
    requested_format: Option<SurfaceFormat>,
    decoder: Option<Arc<VideoDecoder>>,
    width: u32,
    height: u32,
    frames: Vec<PyDecodedFrame>,
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
        let mapped = dec.map_frame(disp, Some(&self.stream))?;
        let w = mapped.width();
        let h = mapped.height();
        let rows = (h as usize) + (h as usize) / 2;
        let (row_bytes, total_bytes, dtype, format_name) = match dec.output_format() {
            SurfaceFormat::Nv12 => {
                let total = nv12_frame_bytes(w, h)
                    .ok_or(nvdec::Error::Invalid("NV12 frame size overflow"))?;
                (w as usize, total, "u8", "nv12")
            }
            SurfaceFormat::P016 => {
                let u16s = p016_frame_u16s(w, h)
                    .ok_or(nvdec::Error::Invalid("P016 frame size overflow"))?;
                let total = u16s
                    .checked_mul(2)
                    .ok_or(nvdec::Error::Invalid("P016 byte size overflow"))?;
                ((w as usize) * 2, total, "u16", "p016")
            }
            SurfaceFormat::Yuv444 | SurfaceFormat::Yuv444_16Bit => {
                return Err(nvdec::Error::Invalid(
                    "only nv12 and p016 decoded surface readback are supported",
                ));
            }
        };
        // SAFETY: Uninitialized GPU buffer is immediately populated via `memcpy_2d_d2d_sync`
        // before being wrapped in `PyDeviceBuffer`.
        let dst =
            unsafe { CudaDeviceBuffer::<u8>::uninitialized_async(&self.stream, total_bytes) }?;
        memcpy_2d_d2d_sync(
            &self.cuda,
            &self.stream,
            mapped.device_ptr(),
            mapped.pitch() as usize,
            dst.cu_deviceptr(),
            row_bytes,
            row_bytes,
            rows,
        )?;
        let buf = PyDeviceBuffer::from_owned_cuda(
            dst,
            self.device_id,
            dtype,
            vec![rows as i64, w as i64],
        );
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
        let device_id = stream.device_id;
        let t0 = Instant::now();

        let parser = py
            .detach(move || {
                let mut cfg = ParserConfig::new(dec_codec);
                cfg.max_display_delay = max_display_delay;
                let collector = FrameCollector {
                    cuda: cuda.clone(),
                    stream: s_inner,
                    device_id,
                    requested_format,
                    decoder: None,
                    width: 0,
                    height: 0,
                    frames: Vec::new(),
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
        let parser_arc = self.parser.clone();
        let payload = data.to_vec();
        let t0 = Instant::now();

        let frames = py
            .detach(move || -> nvdec::Result<Vec<PyDecodedFrame>> {
                let mut guard = parser_arc
                    .lock()
                    .map_err(|_| nvdec::Error::Invalid("decoder mutex poisoned"))?;
                let parser = guard
                    .as_mut()
                    .ok_or(nvdec::Error::Invalid("decoder is closed"))?;
                parser.parse_packet(&payload, timestamp)?;
                Ok(std::mem::take(&mut parser.handler_mut().frames))
            })
            .map_err(|e| map_nvdec_err("decode", e))?;

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
        let parser_arc = self.parser.clone();
        let t0 = Instant::now();

        let frames = py
            .detach(move || -> nvdec::Result<Vec<PyDecodedFrame>> {
                let mut guard = parser_arc
                    .lock()
                    .map_err(|_| nvdec::Error::Invalid("decoder mutex poisoned"))?;
                let parser = guard
                    .as_mut()
                    .ok_or(nvdec::Error::Invalid("decoder is closed"))?;
                parser.flush_eos()?;
                Ok(std::mem::take(&mut parser.handler_mut().frames))
            })
            .map_err(|e| map_nvdec_err("flush", e))?;

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
    fn close(&self) {
        if let Ok(mut guard) = self.parser.lock() {
            *guard = None;
        }
    }

    #[getter]
    fn codec(&self) -> &'static str {
        self.codec_name
    }

    #[getter]
    fn width(&self) -> u32 {
        self.parser
            .lock()
            .ok()
            .and_then(|g| g.as_ref().map(|p| p.handler().width))
            .unwrap_or(0)
    }

    #[getter]
    fn height(&self) -> u32 {
        self.parser
            .lock()
            .ok()
            .and_then(|g| g.as_ref().map(|p| p.handler().height))
            .unwrap_or(0)
    }

    #[getter]
    fn is_closed(&self) -> bool {
        self.parser.lock().map(|g| g.is_none()).unwrap_or(true)
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
    fn __exit__(
        &self,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> bool {
        let _ = (exc_type, exc_val, exc_tb);
        self.close();
        false
    }

    fn __repr__(&self) -> String {
        format!(
            "NvdecDecoder(codec='{}', {}x{}, closed={})",
            self.codec_name,
            self.width(),
            self.height(),
            self.is_closed()
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
    device_id: i32,
}

// SAFETY: NVOFA session and buffers are protected by a Mutex and bound to their
// owning CUDA context when accessed.
unsafe impl Send for NvofaInner {}

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
            memcpy_2d_d2d_sync(
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
            memcpy_2d_d2d_sync(
                cuda,
                stream,
                src_ptr,
                width_bytes,
                dst_ptr,
                dst_pitch,
                width_bytes,
                height as usize,
            )?;
            // Plane 1: interleaved UV (width x height/2)
            let uv_src_offset = (width as u64).saturating_mul(u64::from(height));
            let uv_dst_offset = u64::from(strides.planes[0].1);
            let uv_dst_pitch = strides.planes[1].0.max(strides.planes[0].0) as usize;
            memcpy_2d_d2d_sync(
                cuda,
                stream,
                src_ptr.saturating_add(uv_src_offset),
                width_bytes,
                dst_ptr.saturating_add(uv_dst_offset),
                uv_dst_pitch,
                width_bytes,
                (height as usize) / 2,
            )?;
        }
        _ => {
            return Err(nvofa::Error::Invalid("unsupported NVOFA input format"));
        }
    }
    Ok(())
}

/// Hardware NVIDIA Optical Flow (NVOFA) session producing S10.5 `(grid_h, grid_w, 2)` `i16` flow buffers.
#[pyclass(name = "Nvofa", module = "nv_rs.nvofa")]
pub struct PyNvofa {
    state: Arc<Mutex<Option<NvofaInner>>>,
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
        let (grid_w, grid_h) = grid.grid_dims(width, height);
        let s_inner = stream.inner.clone();
        let cuda = s_inner.context().clone();
        let device_id = stream.device_id;
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
                    device_id,
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
            width,
            height,
            grid_size,
            grid_w,
            grid_h,
            input_format: of_format,
            format_name,
        })
    }

    /// Computes optical flow from `prev_buf` to `curr_buf` and returns an `i16` `[grid_h, grid_w, 2]` `DeviceBuffer`.
    #[pyo3(signature = (prev_buf, curr_buf, disable_temporal_hints=false))]
    fn execute(
        &self,
        py: Python<'_>,
        prev_buf: &PyDeviceBuffer,
        curr_buf: &PyDeviceBuffer,
        disable_temporal_hints: bool,
    ) -> PyResult<PyDeviceBuffer> {
        let prev_ptr = prev_buf.raw_device_ptr();
        let curr_ptr = curr_buf.raw_device_ptr();
        let expected_in_bytes = match self.input_format {
            OfBufferFormat::Grayscale8 | OfBufferFormat::Abgr8 => (self.width as usize)
                .checked_mul(self.height as usize)
                .and_then(|p| p.checked_mul(self.input_format.plane0_elem_bytes())),
            OfBufferFormat::Nv12 => nv12_frame_bytes(self.width, self.height),
            _ => None,
        }
        .ok_or_else(|| {
            record_and_err(
                "nvofa",
                "execute",
                "input frame byte size overflow",
                ErrorKind::InvalidArgument,
            )
        })?;

        validate_dev_buf(
            "nvofa",
            "execute",
            "prev_buf",
            prev_ptr,
            prev_buf.byte_len(),
            expected_in_bytes,
        )?;
        validate_dev_buf(
            "nvofa",
            "execute",
            "curr_buf",
            curr_ptr,
            curr_buf.byte_len(),
            expected_in_bytes,
        )?;

        let state_arc = self.state.clone();
        let width = self.width;
        let height = self.height;
        let grid_w = self.grid_w;
        let grid_h = self.grid_h;
        let input_format = self.input_format;
        let t0 = Instant::now();

        let (out_buf, device_id) = py
            .detach(move || -> nvofa::Result<(CudaDeviceBuffer<u8>, i32)> {
                let mut guard = state_arc
                    .lock()
                    .map_err(|_| nvofa::Error::Invalid("nvofa mutex poisoned"))?;
                let st = guard
                    .as_mut()
                    .ok_or(nvofa::Error::Invalid("nvofa session is closed"))?;

                copy_input_to_of_buffer(
                    &st.cuda,
                    &st.stream,
                    prev_ptr,
                    width,
                    height,
                    input_format,
                    &st.buf_ref,
                )?;
                copy_input_to_of_buffer(
                    &st.cuda,
                    &st.stream,
                    curr_ptr,
                    width,
                    height,
                    input_format,
                    &st.buf_in,
                )?;

                let mut exec = ExecuteParams {
                    input_frame: &st.buf_in,
                    reference_frame: &st.buf_ref,
                    output_buffer: &mut st.buf_out,
                    external_hints: None,
                    output_cost: None,
                    disable_temporal_hints,
                    rois: &[],
                };
                st.session.execute(&mut exec)?;

                let out_ptr = st.buf_out.cu_deviceptr().ok_or(nvofa::Error::Invalid(
                    "NVOFA output buffer is not CuDevicePtr",
                ))?;
                let out_stride = st.buf_out.stride_info()?.planes[0].0 as usize;
                let row_bytes = (grid_w as usize)
                    .checked_mul(size_of::<FlowVector>())
                    .ok_or(nvofa::Error::Invalid("flow row byte size overflow"))?;
                let total_bytes = row_bytes
                    .checked_mul(grid_h as usize)
                    .ok_or(nvofa::Error::Invalid("flow buffer byte size overflow"))?;
                // SAFETY: Uninitialized GPU buffer is immediately populated via `memcpy_2d_d2d_sync`
                // before returning to Python.
                let dst = unsafe {
                    CudaDeviceBuffer::<u8>::uninitialized_async(&st.stream, total_bytes)
                }?;
                memcpy_2d_d2d_sync(
                    &st.cuda,
                    &st.stream,
                    out_ptr,
                    out_stride,
                    dst.cu_deviceptr(),
                    row_bytes,
                    row_bytes,
                    grid_h as usize,
                )?;
                Ok((dst, st.device_id))
            })
            .map_err(|e| map_nvofa_err("execute", e))?;

        diag::record_event(
            Level::Trace,
            "nvofa",
            "execute",
            format!("{width}x{height} -> {grid_w}x{grid_h}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(PyDeviceBuffer::from_owned_cuda(
            out_buf,
            device_id,
            "i16",
            vec![i64::from(grid_h), i64::from(grid_w), 2],
        ))
    }

    /// Explicitly releases the NVOFA session and internal GPU buffers.
    fn close(&self) {
        if let Ok(mut guard) = self.state.lock() {
            *guard = None;
        }
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
    fn is_closed(&self) -> bool {
        self.state.lock().map(|g| g.is_none()).unwrap_or(true)
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
    fn __exit__(
        &self,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> bool {
        let _ = (exc_type, exc_val, exc_tb);
        self.close();
        false
    }

    fn __repr__(&self) -> String {
        format!(
            "Nvofa({}x{}, grid_size={}, grid={}x{}, format='{}', closed={})",
            self.width,
            self.height,
            self.grid_size,
            self.grid_w,
            self.grid_h,
            self.format_name,
            self.is_closed()
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
