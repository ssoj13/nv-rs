//! Safe NVDEC hardware video decoder and parser wrapper over `nvcuvid-sys` and `cuda-core`.
//!
//! **What.**
//! - [`VideoParser`] — wraps `CUvideoparser` with a [`ParserHandler`] trait whose sequence,
//!   decode, and display callbacks execute inside [`std::panic::catch_unwind`]; any Rust error or
//!   caught panic is recorded and surfaced on the next safe call (`parse_packet` / `flush_eos`).
//! - [`VideoDecoder`] — wraps `CUvideodecoder`, capability queries ([`decoder_caps`]), picture
//!   decoding ([`VideoDecoder::decode_picture`]), and frame mapping ([`VideoDecoder::map_frame`]).
//! - [`MappedFrame`] — RAII guard over `cuvidMapVideoFrame64` / `cuvidUnmapVideoFrame64` with
//!   zero-allocation NV12 and P016 surface readback into caller-owned host slices.
//!
//! **Conventions.** Every handle owns its NVDEC object, holds an `Arc<Context>`, and destroys it
//! on `Drop`; teardown errors are recorded on `cuda_core::Context::record_err` instead of
//! panicking.
//! **Used by** media decoders and the NVENC -> NVDEC PSNR round-trip tests.

pub use nvcuvid_sys as sys;

use cuda_core::{Context, DriverError, IntoResult, Stream};
use std::any::Any;
use std::ffi::{c_int, c_ulong, c_void};
use std::fmt;
use std::mem::zeroed;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;

/// Raw `CUresult` status code wrapped as a `Copy` integer newtype.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StatusCode(pub sys::CUresult);

impl StatusCode {
    /// Returns the underlying `CUresult` value.
    pub const fn raw(self) -> sys::CUresult {
        self.0
    }
}

impl fmt::Debug for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StatusCode({})", self.0)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NVDEC status {}", self.0)
    }
}

/// Everything an NVDEC call can fail with.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `nvcuvid` could not be loaded or is missing an entry point.
    #[error("NVDEC unavailable: {0}")]
    Unavailable(String),
    /// An `nvcuvid` call returned a non-success `CUresult`.
    #[error("{call} failed with {code}")]
    Status {
        /// The `nvcuvid` entry point.
        call: &'static str,
        /// The raw `CUresult` code.
        code: StatusCode,
    },
    /// A CUDA driver call failed.
    #[error("CUDA: {0}")]
    Driver(#[from] DriverError),
    /// The hardware decoder does not support the requested codec, chroma, or dimensions.
    #[error("unsupported {what}: required {required}, found {found}")]
    Unsupported {
        /// What was requested.
        what: &'static str,
        /// Required value.
        required: u32,
        /// Reported value.
        found: u32,
    },
    /// Operation exceeds the caller's byte budget.
    #[error("needs {requested} bytes, budget admits {available}")]
    Budget {
        /// Bytes requested.
        requested: usize,
        /// Bytes admitted by budget.
        available: usize,
    },
    /// An argument was rejected before reaching NVDEC.
    #[error("invalid argument: {0}")]
    Invalid(&'static str),
    /// A parser callback panicked across the FFI boundary.
    #[error("parser callback panicked: {0}")]
    CallbackPanic(String),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

fn check(call: &'static str, code: sys::CUresult) -> Result<()> {
    if code == cuda_core::sys::cudaError_enum_CUDA_SUCCESS {
        return Ok(());
    }
    if code == cuda_core::sys::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED {
        return Err(Error::Unavailable(format!("{call}: nvcuvid library not loaded")));
    }
    if code == cuda_core::sys::cudaError_enum_CUDA_ERROR_NOT_FOUND {
        return Err(Error::Unavailable(format!("{call}: symbol missing from nvcuvid")));
    }
    Err(Error::Status { call, code: StatusCode(code) })
}

fn ensure_loaded() -> Result<()> {
    sys::load().map(|_| ()).map_err(|e| Error::Unavailable(e.to_string()))
}

/// Supported video codecs (`cudaVideoCodec`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Codec {
    /// MPEG-1.
    Mpeg1,
    /// MPEG-2.
    Mpeg2,
    /// MPEG-4.
    Mpeg4,
    /// VC-1.
    Vc1,
    /// H.264 / AVC.
    H264,
    /// Motion JPEG.
    Jpeg,
    /// HEVC / H.265.
    Hevc,
    /// VP8.
    Vp8,
    /// VP9.
    Vp9,
    /// AV1.
    Av1,
}

impl Codec {
    const fn to_sys(self) -> sys::cudaVideoCodec {
        match self {
            Self::Mpeg1 => sys::cudaVideoCodec_enum_cudaVideoCodec_MPEG1,
            Self::Mpeg2 => sys::cudaVideoCodec_enum_cudaVideoCodec_MPEG2,
            Self::Mpeg4 => sys::cudaVideoCodec_enum_cudaVideoCodec_MPEG4,
            Self::Vc1 => sys::cudaVideoCodec_enum_cudaVideoCodec_VC1,
            Self::H264 => sys::cudaVideoCodec_enum_cudaVideoCodec_H264,
            Self::Jpeg => sys::cudaVideoCodec_enum_cudaVideoCodec_JPEG,
            Self::Hevc => sys::cudaVideoCodec_enum_cudaVideoCodec_HEVC,
            Self::Vp8 => sys::cudaVideoCodec_enum_cudaVideoCodec_VP8,
            Self::Vp9 => sys::cudaVideoCodec_enum_cudaVideoCodec_VP9,
            Self::Av1 => sys::cudaVideoCodec_enum_cudaVideoCodec_AV1,
        }
    }

    fn from_sys(raw: sys::cudaVideoCodec) -> Option<Self> {
        match raw {
            x if x == sys::cudaVideoCodec_enum_cudaVideoCodec_MPEG1 => Some(Self::Mpeg1),
            x if x == sys::cudaVideoCodec_enum_cudaVideoCodec_MPEG2 => Some(Self::Mpeg2),
            x if x == sys::cudaVideoCodec_enum_cudaVideoCodec_MPEG4 => Some(Self::Mpeg4),
            x if x == sys::cudaVideoCodec_enum_cudaVideoCodec_VC1 => Some(Self::Vc1),
            x if x == sys::cudaVideoCodec_enum_cudaVideoCodec_H264 => Some(Self::H264),
            x if x == sys::cudaVideoCodec_enum_cudaVideoCodec_JPEG => Some(Self::Jpeg),
            x if x == sys::cudaVideoCodec_enum_cudaVideoCodec_HEVC => Some(Self::Hevc),
            x if x == sys::cudaVideoCodec_enum_cudaVideoCodec_VP8 => Some(Self::Vp8),
            x if x == sys::cudaVideoCodec_enum_cudaVideoCodec_VP9 => Some(Self::Vp9),
            x if x == sys::cudaVideoCodec_enum_cudaVideoCodec_AV1 => Some(Self::Av1),
            _ => None,
        }
    }
}

/// Chroma subsampling format (`cudaVideoChromaFormat`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChromaFormat {
    /// Monochrome (4:0:0).
    Monochrome,
    /// 4:2:0.
    Yuv420,
    /// 4:2:2.
    Yuv422,
    /// 4:4:4.
    Yuv444,
}

impl ChromaFormat {
    const fn to_sys(self) -> sys::cudaVideoChromaFormat {
        match self {
            Self::Monochrome => sys::cudaVideoChromaFormat_enum_cudaVideoChromaFormat_Monochrome,
            Self::Yuv420 => sys::cudaVideoChromaFormat_enum_cudaVideoChromaFormat_420,
            Self::Yuv422 => sys::cudaVideoChromaFormat_enum_cudaVideoChromaFormat_422,
            Self::Yuv444 => sys::cudaVideoChromaFormat_enum_cudaVideoChromaFormat_444,
        }
    }

    fn from_sys(raw: sys::cudaVideoChromaFormat) -> Option<Self> {
        match raw {
            x if x == sys::cudaVideoChromaFormat_enum_cudaVideoChromaFormat_Monochrome => Some(Self::Monochrome),
            x if x == sys::cudaVideoChromaFormat_enum_cudaVideoChromaFormat_420 => Some(Self::Yuv420),
            x if x == sys::cudaVideoChromaFormat_enum_cudaVideoChromaFormat_422 => Some(Self::Yuv422),
            x if x == sys::cudaVideoChromaFormat_enum_cudaVideoChromaFormat_444 => Some(Self::Yuv444),
            _ => None,
        }
    }
}

/// Decoded output surface format (`cudaVideoSurfaceFormat`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceFormat {
    /// 8-bit semi-planar YUV 4:2:0 (Y plane + interleaved UV plane).
    Nv12,
    /// 16-bit semi-planar YUV 4:2:0 (10/12-bit MSBs in 16-bit words).
    P016,
    /// 8-bit planar YUV 4:4:4.
    Yuv444,
    /// 16-bit planar YUV 4:4:4.
    Yuv444_16Bit,
}

impl SurfaceFormat {
    const fn to_sys(self) -> sys::cudaVideoSurfaceFormat {
        match self {
            Self::Nv12 => sys::cudaVideoSurfaceFormat_enum_cudaVideoSurfaceFormat_NV12,
            Self::P016 => sys::cudaVideoSurfaceFormat_enum_cudaVideoSurfaceFormat_P016,
            Self::Yuv444 => sys::cudaVideoSurfaceFormat_enum_cudaVideoSurfaceFormat_YUV444,
            Self::Yuv444_16Bit => sys::cudaVideoSurfaceFormat_enum_cudaVideoSurfaceFormat_YUV444_16Bit,
        }
    }
}

/// Deinterlacing mode (`cudaVideoDeinterlaceMode`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DeinterlaceMode {
    /// Weave both fields (no deinterlacing; standard for progressive frames).
    #[default]
    Weave,
    /// Bob deinterlacing (drop one field).
    Bob,
    /// Hardware adaptive deinterlacing.
    Adaptive,
}

impl DeinterlaceMode {
    const fn to_sys(self) -> sys::cudaVideoDeinterlaceMode {
        match self {
            Self::Weave => sys::cudaVideoDeinterlaceMode_enum_cudaVideoDeinterlaceMode_Weave,
            Self::Bob => sys::cudaVideoDeinterlaceMode_enum_cudaVideoDeinterlaceMode_Bob,
            Self::Adaptive => sys::cudaVideoDeinterlaceMode_enum_cudaVideoDeinterlaceMode_Adaptive,
        }
    }
}

/// Hardware decoder capabilities returned by [`decoder_caps`].
#[derive(Clone, Copy, Debug)]
pub struct DecoderCaps {
    /// Whether the queried codec/chroma/bit-depth is supported.
    pub supported: bool,
    /// Number of hardware NVDEC engines supporting this configuration.
    pub num_nvdecs: u8,
    /// Bitmask of supported [`SurfaceFormat`] values.
    pub output_format_mask: u16,
    /// Maximum supported coded width in pixels.
    pub max_width: u32,
    /// Maximum supported coded height in pixels.
    pub max_height: u32,
    /// Maximum supported macroblock count (`width * height / 256`).
    pub max_mb_count: u32,
    /// Minimum supported coded width in pixels.
    pub min_width: u16,
    /// Minimum supported coded height in pixels.
    pub min_height: u16,
}

/// Queries NVDEC hardware capabilities for `codec`, `chroma`, and `bit_depth_minus8`.
pub fn decoder_caps(cuda: &Arc<Context>, codec: Codec, chroma: ChromaFormat, bit_depth_minus8: u32) -> Result<DecoderCaps> {
    ensure_loaded()?;
    cuda.bind_to_thread()?;
    // SAFETY: all-zero bit pattern is valid for `CUVIDDECODECAPS`.
    let mut caps: sys::CUVIDDECODECAPS = unsafe { zeroed() };
    caps.eCodecType = codec.to_sys();
    caps.eChromaFormat = chroma.to_sys();
    caps.nBitDepthMinus8 = bit_depth_minus8;
    // SAFETY: `cuda` is current and `caps` is a valid out-struct.
    let status = unsafe { sys::cuvidGetDecoderCaps(&mut caps) };
    check("cuvidGetDecoderCaps", status)?;
    Ok(DecoderCaps {
        supported: caps.bIsSupported != 0,
        num_nvdecs: caps.nNumNVDECs,
        output_format_mask: caps.nOutputFormatMask,
        max_width: caps.nMaxWidth,
        max_height: caps.nMaxHeight,
        max_mb_count: caps.nMaxMBCount,
        min_width: caps.nMinWidth,
        min_height: caps.nMinHeight,
    })
}

/// Display rectangle in pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayArea {
    /// Left edge (inclusive).
    pub left: i32,
    /// Top edge (inclusive).
    pub top: i32,
    /// Right edge (exclusive).
    pub right: i32,
    /// Bottom edge (exclusive).
    pub bottom: i32,
}

/// Video sequence format reported to [`ParserHandler::on_sequence`].
#[derive(Clone, Copy, Debug)]
pub struct VideoFormat {
    /// Detected codec (if known).
    pub codec: Option<Codec>,
    /// Coded frame width in pixels.
    pub coded_width: u32,
    /// Coded frame height in pixels.
    pub coded_height: u32,
    /// Active display rectangle.
    pub display_area: DisplayArea,
    /// Chroma subsampling format (if known).
    pub chroma_format: Option<ChromaFormat>,
    /// Luma bit depth minus 8.
    pub bit_depth_luma_minus8: u8,
    /// Chroma bit depth minus 8.
    pub bit_depth_chroma_minus8: u8,
    /// Minimum number of decode surfaces required by the parser/DPB.
    pub min_num_decode_surfaces: u8,
    /// True if sequence is progressive.
    pub progressive_sequence: bool,
    /// Frame rate numerator (0 if unspecified).
    pub fps_num: u32,
    /// Frame rate denominator (0 if unspecified).
    pub fps_den: u32,
}

impl VideoFormat {
    fn from_sys(raw: &sys::CUVIDEOFORMAT) -> Self {
        Self {
            codec: Codec::from_sys(raw.codec),
            coded_width: raw.coded_width,
            coded_height: raw.coded_height,
            display_area: DisplayArea {
                left: raw.display_area.left,
                top: raw.display_area.top,
                right: raw.display_area.right,
                bottom: raw.display_area.bottom,
            },
            chroma_format: ChromaFormat::from_sys(raw.chroma_format),
            bit_depth_luma_minus8: raw.bit_depth_luma_minus8,
            bit_depth_chroma_minus8: raw.bit_depth_chroma_minus8,
            min_num_decode_surfaces: raw.min_num_decode_surfaces,
            progressive_sequence: raw.progressive_sequence != 0,
            fps_num: raw.frame_rate.numerator,
            fps_den: raw.frame_rate.denominator,
        }
    }

    /// Display width derived from `display_area`, falling back to `coded_width`.
    pub fn display_width(&self) -> u32 {
        let w = self.display_area.right - self.display_area.left;
        if w > 0 { w as u32 } else { self.coded_width }
    }

    /// Display height derived from `display_area`, falling back to `coded_height`.
    pub fn display_height(&self) -> u32 {
        let h = self.display_area.bottom - self.display_area.top;
        if h > 0 { h as u32 } else { self.coded_height }
    }
}

/// Display metadata reported to [`ParserHandler::on_display`].
#[derive(Clone, Copy, Debug)]
pub struct DisplayInfo {
    /// Decoder surface index to map via [`VideoDecoder::map_frame`].
    pub picture_index: i32,
    /// True if the frame is progressive.
    pub progressive_frame: bool,
    /// True if top field is displayed first.
    pub top_field_first: bool,
    /// Repeat-first-field indicator.
    pub repeat_first_field: i32,
    /// Presentation timestamp (10 MHz clock by default).
    pub timestamp: i64,
}

impl DisplayInfo {
    fn from_sys(raw: &sys::CUVIDPARSERDISPINFO) -> Self {
        Self {
            picture_index: raw.picture_index,
            progressive_frame: raw.progressive_frame != 0,
            top_field_first: raw.top_field_first != 0,
            repeat_first_field: raw.repeat_first_field,
            timestamp: raw.timestamp,
        }
    }
}

/// Application callbacks driven synchronously by [`VideoParser::parse_packet`].
pub trait ParserHandler {
    /// Called on initial sequence header or format change.
    ///
    /// Return `Ok(surface_count)` (`> 1` overrides the parser's DPB size; `1` keeps default).
    fn on_sequence(&mut self, fmt: &VideoFormat) -> Result<u32>;
    /// Called when a picture is ready to be decoded (decode order).
    fn on_decode(&mut self, pic: &sys::CUVIDPICPARAMS) -> Result<()>;
    /// Called when a decoded picture is ready for display (`None` signals EOS).
    fn on_display(&mut self, disp: Option<&DisplayInfo>) -> Result<()>;
}

/// Configuration for [`VideoParser::new`].
#[derive(Clone, Copy, Debug)]
pub struct ParserConfig {
    /// Expected bitstream codec.
    pub codec: Codec,
    /// Maximum number of decode surfaces (DPB size).
    pub max_num_decode_surfaces: u32,
    /// Timestamp clock rate in Hz (`0` = default 10 MHz).
    pub clock_rate: u32,
    /// Corruption threshold `0..=100` (`100` decodes even corrupted pictures).
    pub error_threshold: u32,
    /// Maximum display queue delay (`0` = low latency, `2..=4` recommended for B-frames).
    pub max_display_delay: u32,
}

impl ParserConfig {
    /// Sensible defaults for `codec` with low-latency display (`max_display_delay = 0`).
    pub fn new(codec: Codec) -> Self {
        Self {
            codec,
            max_num_decode_surfaces: 8,
            clock_rate: 0,
            error_threshold: 100,
            max_display_delay: 0,
        }
    }
}

struct ParserInner<H: ParserHandler> {
    handler: H,
    pending_error: Option<Error>,
}

fn panic_payload_msg(payload: Box<dyn Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_owned()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown panic payload".to_owned()
    }
}

unsafe extern "C" fn on_sequence_cb<H: ParserHandler>(user_data: *mut c_void, fmt: *mut sys::CUVIDEOFORMAT) -> c_int {
    if user_data.is_null() || fmt.is_null() {
        return 0;
    }
    // SAFETY: `user_data` points to `ParserInner<H>` owned by `VideoParser` for the call duration;
    // `fmt` is a valid `CUVIDEOFORMAT` pointer provided by `nvcuvid`.
    let (inner, fmt_ref) = unsafe { (&mut *user_data.cast::<ParserInner<H>>(), &*fmt) };
    let vf = VideoFormat::from_sys(fmt_ref);
    match catch_unwind(AssertUnwindSafe(|| inner.handler.on_sequence(&vf))) {
        Ok(Ok(n)) => n.max(1) as c_int,
        Ok(Err(e)) => {
            inner.pending_error = Some(e);
            0
        }
        Err(p) => {
            inner.pending_error = Some(Error::CallbackPanic(panic_payload_msg(p)));
            0
        }
    }
}

unsafe extern "C" fn on_decode_cb<H: ParserHandler>(user_data: *mut c_void, pic: *mut sys::CUVIDPICPARAMS) -> c_int {
    if user_data.is_null() || pic.is_null() {
        return 0;
    }
    // SAFETY: `user_data` points to `ParserInner<H>` and `pic` is provided by `nvcuvid`.
    let (inner, pic_ref) = unsafe { (&mut *user_data.cast::<ParserInner<H>>(), &*pic) };
    match catch_unwind(AssertUnwindSafe(|| inner.handler.on_decode(pic_ref))) {
        Ok(Ok(())) => 1,
        Ok(Err(e)) => {
            inner.pending_error = Some(e);
            0
        }
        Err(p) => {
            inner.pending_error = Some(Error::CallbackPanic(panic_payload_msg(p)));
            0
        }
    }
}

unsafe extern "C" fn on_display_cb<H: ParserHandler>(user_data: *mut c_void, disp: *mut sys::CUVIDPARSERDISPINFO) -> c_int {
    if user_data.is_null() {
        return 0;
    }
    // SAFETY: `user_data` points to `ParserInner<H>`; `disp` is null on EOS notify or a valid `CUVIDPARSERDISPINFO`.
    let inner = unsafe { &mut *user_data.cast::<ParserInner<H>>() };
    // SAFETY: when non-null, `disp` is valid for the duration of this callback.
    let info = unsafe { disp.as_ref() }.map(DisplayInfo::from_sys);
    match catch_unwind(AssertUnwindSafe(|| inner.handler.on_display(info.as_ref()))) {
        Ok(Ok(())) => 1,
        Ok(Err(e)) => {
            inner.pending_error = Some(e);
            0
        }
        Err(p) => {
            inner.pending_error = Some(Error::CallbackPanic(panic_payload_msg(p)));
            0
        }
    }
}

/// Hardware bitstream parser (`CUvideoparser`) with `catch_unwind`-protected callbacks.
pub struct VideoParser<H: ParserHandler> {
    raw: sys::CUvideoparser,
    cuda: Arc<Context>,
    inner: Box<ParserInner<H>>,
}

impl<H: ParserHandler> VideoParser<H> {
    /// Creates a `CUvideoparser` driving `handler`.
    pub fn new(cuda: &Arc<Context>, cfg: &ParserConfig, handler: H) -> Result<Self> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut inner = Box::new(ParserInner { handler, pending_error: None });
        // SAFETY: all-zero bit pattern is valid for `CUVIDPARSERPARAMS`.
        let mut params: sys::CUVIDPARSERPARAMS = unsafe { zeroed() };
        params.CodecType = cfg.codec.to_sys();
        params.ulMaxNumDecodeSurfaces = cfg.max_num_decode_surfaces.max(1);
        params.ulClockRate = cfg.clock_rate;
        params.ulErrorThreshold = cfg.error_threshold.min(100);
        params.ulMaxDisplayDelay = cfg.max_display_delay;
        params.pUserData = (&raw mut *inner).cast::<c_void>();
        params.pfnSequenceCallback = Some(on_sequence_cb::<H>);
        params.pfnDecodePicture = Some(on_decode_cb::<H>);
        params.pfnDisplayPicture = Some(on_display_cb::<H>);

        let mut raw: sys::CUvideoparser = std::ptr::null_mut();
        // SAFETY: `params` points to `inner` at a stable heap address and `raw` is a valid out-pointer.
        let status = unsafe { sys::cuvidCreateVideoParser(&mut raw, &mut params) };
        check("cuvidCreateVideoParser", status)?;
        Ok(Self { raw, cuda: cuda.clone(), inner })
    }

    /// Parses one compressed bitstream packet with zero heap allocations.
    pub fn parse_packet(&mut self, payload: &[u8], timestamp: Option<i64>) -> Result<()> {
        if let Some(err) = self.inner.pending_error.take() {
            return Err(err);
        }
        self.cuda.bind_to_thread()?;
        // SAFETY: all-zero bit pattern is valid for `CUVIDSOURCEDATAPACKET`.
        let mut pkt: sys::CUVIDSOURCEDATAPACKET = unsafe { zeroed() };
        pkt.payload_size = payload.len() as c_ulong;
        pkt.payload = if payload.is_empty() { std::ptr::null() } else { payload.as_ptr() };
        if let Some(ts) = timestamp {
            pkt.flags |= sys::CUvideopacketflags_CUVID_PKT_TIMESTAMP as c_ulong;
            pkt.timestamp = ts;
        }
        // SAFETY: `self.raw` is a live parser and `pkt.payload` is valid for the synchronous call.
        let status = unsafe { sys::cuvidParseVideoData(self.raw, &mut pkt) };
        if let Some(err) = self.inner.pending_error.take() {
            return Err(err);
        }
        check("cuvidParseVideoData", status)
    }

    /// Signals End-of-Stream to flush all remaining display callbacks.
    pub fn flush_eos(&mut self) -> Result<()> {
        if let Some(err) = self.inner.pending_error.take() {
            return Err(err);
        }
        self.cuda.bind_to_thread()?;
        // SAFETY: all-zero bit pattern is valid for `CUVIDSOURCEDATAPACKET`.
        let mut pkt: sys::CUVIDSOURCEDATAPACKET = unsafe { zeroed() };
        pkt.flags = (sys::CUvideopacketflags_CUVID_PKT_ENDOFSTREAM | sys::CUvideopacketflags_CUVID_PKT_NOTIFY_EOS) as c_ulong;
        // SAFETY: `self.raw` is a live parser; EOS packet carries null/0 payload.
        let status = unsafe { sys::cuvidParseVideoData(self.raw, &mut pkt) };
        if let Some(err) = self.inner.pending_error.take() {
            return Err(err);
        }
        check("cuvidParseVideoData(EOS)", status)
    }

    /// Shared reference to the inner [`ParserHandler`].
    pub fn handler(&self) -> &H {
        &self.inner.handler
    }

    /// Mutable reference to the inner [`ParserHandler`].
    pub fn handler_mut(&mut self) -> &mut H {
        &mut self.inner.handler
    }
}

impl<H: ParserHandler> Drop for VideoParser<H> {
    fn drop(&mut self) {
        if self.raw.is_null() {
            return;
        }
        if let Err(e) = self.cuda.bind_to_thread() {
            self.cuda.record_err(Err::<(), _>(e));
            return;
        }
        // SAFETY: `self.raw` is owned by this `VideoParser`.
        let status = unsafe { sys::cuvidDestroyVideoParser(self.raw) };
        if status != cuda_core::sys::cudaError_enum_CUDA_SUCCESS {
            self.cuda.record_err(Err::<(), _>(DriverError(status)));
        }
    }
}

/// Configuration for [`VideoDecoder::new`].
#[derive(Clone, Copy, Debug)]
pub struct DecoderConfig {
    /// Coded width in pixels.
    pub width: u32,
    /// Coded height in pixels.
    pub height: u32,
    /// Number of internal decode surfaces (`>= min_num_decode_surfaces`).
    pub num_decode_surfaces: u32,
    /// Video codec.
    pub codec: Codec,
    /// Chroma subsampling format.
    pub chroma_format: ChromaFormat,
    /// Bit depth minus 8 (`0` for 8-bit, `2` for 10-bit).
    pub bit_depth_minus8: u32,
    /// Output surface format (`Nv12` or `P016`).
    pub output_format: SurfaceFormat,
    /// Deinterlacing mode.
    pub deinterlace_mode: DeinterlaceMode,
    /// Post-processed output width (aligned to 2).
    pub target_width: u32,
    /// Post-processed output height (aligned to 2).
    pub target_height: u32,
    /// Maximum number of output surfaces simultaneously mapped (`>= 1`).
    pub num_output_surfaces: u32,
}

impl DecoderConfig {
    /// Builds a [`DecoderConfig`] from a [`VideoFormat`] reported by [`ParserHandler::on_sequence`].
    pub fn from_video_format(fmt: &VideoFormat, output_format: SurfaceFormat) -> Result<Self> {
        let codec = fmt.codec.ok_or(Error::Invalid("unsupported or unknown codec in sequence header"))?;
        let chroma_format = fmt.chroma_format.ok_or(Error::Invalid("unsupported or unknown chroma format"))?;
        let w = fmt.display_width();
        let h = fmt.display_height();
        Ok(Self {
            width: fmt.coded_width,
            height: fmt.coded_height,
            num_decode_surfaces: u32::from(fmt.min_num_decode_surfaces).max(4),
            codec,
            chroma_format,
            bit_depth_minus8: u32::from(fmt.bit_depth_luma_minus8),
            output_format,
            deinterlace_mode: DeinterlaceMode::Weave,
            target_width: w + (w & 1),
            target_height: h + (h & 1),
            num_output_surfaces: 2,
        })
    }
}

/// Hardware video decoder (`CUvideodecoder`).
pub struct VideoDecoder {
    raw: sys::CUvideodecoder,
    cuda: Arc<Context>,
    target_width: u32,
    target_height: u32,
    output_format: SurfaceFormat,
}

// SAFETY: `CUvideodecoder` is thread-safe when bound to a `cuda_core::Context` (driver-synchronized).
unsafe impl Send for VideoDecoder {}
// SAFETY: see `Send` above.
unsafe impl Sync for VideoDecoder {}

impl VideoDecoder {
    /// Creates a hardware video decoder on `cuda`.
    pub fn new(cuda: &Arc<Context>, cfg: &DecoderConfig) -> Result<Arc<Self>> {
        ensure_loaded()?;
        if cfg.width == 0 || cfg.height == 0 || cfg.target_width == 0 || cfg.target_height == 0 {
            return Err(Error::Invalid("decoder dimensions must be non-zero"));
        }
        if !cfg.target_width.is_multiple_of(2) || !cfg.target_height.is_multiple_of(2) {
            return Err(Error::Invalid("decoder target_width and target_height must be multiples of 2"));
        }
        cuda.bind_to_thread()?;
        // SAFETY: all-zero bit pattern is valid for `CUVIDDECODECREATEINFO`.
        let mut info: sys::CUVIDDECODECREATEINFO = unsafe { zeroed() };
        info.ulWidth = cfg.width as c_ulong;
        info.ulHeight = cfg.height as c_ulong;
        info.ulNumDecodeSurfaces = cfg.num_decode_surfaces.max(1) as c_ulong;
        info.CodecType = cfg.codec.to_sys();
        info.ChromaFormat = cfg.chroma_format.to_sys();
        info.bitDepthMinus8 = cfg.bit_depth_minus8 as c_ulong;
        info.ulMaxWidth = cfg.width as c_ulong;
        info.ulMaxHeight = cfg.height as c_ulong;
        info.OutputFormat = cfg.output_format.to_sys();
        info.DeinterlaceMode = cfg.deinterlace_mode.to_sys();
        info.ulTargetWidth = cfg.target_width as c_ulong;
        info.ulTargetHeight = cfg.target_height as c_ulong;
        info.ulNumOutputSurfaces = cfg.num_output_surfaces.max(1) as c_ulong;

        let mut raw: sys::CUvideodecoder = std::ptr::null_mut();
        // SAFETY: `cuda` is current and `info` is populated with valid dimensions and formats.
        let status = unsafe { sys::cuvidCreateDecoder(&mut raw, &mut info) };
        check("cuvidCreateDecoder", status)?;
        Ok(Arc::new(Self {
            raw,
            cuda: cuda.clone(),
            target_width: cfg.target_width,
            target_height: cfg.target_height,
            output_format: cfg.output_format,
        }))
    }

    /// Submits one parsed picture (`CUVIDPICPARAMS`) for hardware decoding.
    ///
    /// # Safety
    /// `pic` and all internal bitstream/slice pointers it contains (`pBitstreamData`,
    /// `pSliceDataOffsets`) must be valid for the duration of this call (guaranteed when `pic` is
    /// received from [`ParserHandler::on_decode`]).
    pub unsafe fn decode_picture(&self, pic: &sys::CUVIDPICPARAMS) -> Result<()> {
        self.cuda.bind_to_thread()?;
        // SAFETY: `self.raw` is a live decoder and `pic` (plus its internal pointers) is valid per caller contract.
        let status = unsafe { sys::cuvidDecodePicture(self.raw, std::ptr::from_ref(pic).cast_mut()) };
        check("cuvidDecodePicture", status)
    }

    /// Maps a decoded display surface for CUDA access; unmapped automatically on `Drop`.
    pub fn map_frame(&self, disp: &DisplayInfo, stream: Option<&Stream>) -> Result<MappedFrame<'_>> {
        self.cuda.bind_to_thread()?;
        // SAFETY: all-zero bit pattern is valid for `CUVIDPROCPARAMS`.
        let mut proc_params: sys::CUVIDPROCPARAMS = unsafe { zeroed() };
        proc_params.progressive_frame = c_int::from(disp.progressive_frame);
        proc_params.top_field_first = c_int::from(disp.top_field_first);
        proc_params.unpaired_field = c_int::from(disp.repeat_first_field < 0);
        if let Some(s) = stream {
            proc_params.output_stream = s.cu_stream();
        }
        let mut dev_ptr: u64 = 0;
        let mut pitch: u32 = 0;
        // SAFETY: `self.raw` is live and `dev_ptr`/`pitch` are valid out-pointers.
        let status = unsafe { sys::cuvidMapVideoFrame64(self.raw, disp.picture_index, &mut dev_ptr, &mut pitch, &mut proc_params) };
        check("cuvidMapVideoFrame64", status)?;
        Ok(MappedFrame {
            decoder: self,
            dev_ptr,
            pitch,
        })
    }

    /// Target output width in pixels.
    pub const fn target_width(&self) -> u32 {
        self.target_width
    }

    /// Target output height in pixels.
    pub const fn target_height(&self) -> u32 {
        self.target_height
    }

    /// Configured output surface format.
    pub const fn output_format(&self) -> SurfaceFormat {
        self.output_format
    }
}

impl Drop for VideoDecoder {
    fn drop(&mut self) {
        if self.raw.is_null() {
            return;
        }
        if let Err(e) = self.cuda.bind_to_thread() {
            self.cuda.record_err(Err::<(), _>(e));
            return;
        }
        // SAFETY: `self.raw` is owned by this `VideoDecoder` and no `MappedFrame` outlives it.
        let status = unsafe { sys::cuvidDestroyDecoder(self.raw) };
        if status != cuda_core::sys::cudaError_enum_CUDA_SUCCESS {
            self.cuda.record_err(Err::<(), _>(DriverError(status)));
        }
    }
}

/// RAII guard for a mapped NVDEC output surface (`cuvidMapVideoFrame64` / `cuvidUnmapVideoFrame64`).
pub struct MappedFrame<'a> {
    decoder: &'a VideoDecoder,
    dev_ptr: u64,
    pitch: u32,
}

impl MappedFrame<'_> {
    /// Device pointer (`CUdeviceptr`) to the mapped surface.
    pub const fn device_ptr(&self) -> cuda_core::sys::CUdeviceptr {
        self.dev_ptr
    }

    /// Row pitch in bytes.
    pub const fn pitch(&self) -> u32 {
        self.pitch
    }

    /// Frame width in pixels.
    pub const fn width(&self) -> u32 {
        self.decoder.target_width
    }

    /// Frame height in pixels.
    pub const fn height(&self) -> u32 {
        self.decoder.target_height
    }

    /// Copies a tight NV12 surface (`Y` + interleaved `UV`) to `dst` on `stream` with zero allocations.
    pub fn copy_nv12_to_host(&self, stream: &Stream, dst: &mut [u8]) -> Result<()> {
        if self.decoder.output_format != SurfaceFormat::Nv12 {
            return Err(Error::Invalid("surface format is not NV12"));
        }
        let expected = nv12_frame_bytes(self.width(), self.height()).ok_or(Error::Invalid("NV12 frame size overflow"))?;
        if dst.len() != expected {
            return Err(Error::Invalid("destination slice length does not match NV12 frame size"));
        }
        let rows = (self.height() as usize) + (self.height() as usize) / 2;
        self.copy_2d_to_host(stream, self.width() as usize, rows, dst.as_mut_ptr().cast())
    }

    /// Copies a tight P016 surface (`Y` + interleaved `UV` in `u16`s) to `dst` on `stream` with zero allocations.
    pub fn copy_p016_to_host(&self, stream: &Stream, dst: &mut [u16]) -> Result<()> {
        if self.decoder.output_format != SurfaceFormat::P016 {
            return Err(Error::Invalid("surface format is not P016"));
        }
        let expected = p016_frame_u16s(self.width(), self.height()).ok_or(Error::Invalid("P016 frame size overflow"))?;
        if dst.len() != expected {
            return Err(Error::Invalid("destination slice length does not match P016 frame size"));
        }
        let rows = (self.height() as usize) + (self.height() as usize) / 2;
        self.copy_2d_to_host(stream, (self.width() as usize) * 2, rows, dst.as_mut_ptr().cast())
    }

    fn copy_2d_to_host(&self, stream: &Stream, width_bytes: usize, rows: usize, dst_host: *mut c_void) -> Result<()> {
        self.decoder.cuda.bind_to_thread()?;
        // SAFETY: all-zero bit pattern is valid for `CUDA_MEMCPY2D`.
        let mut copy: cuda_core::sys::CUDA_MEMCPY2D = unsafe { zeroed() };
        copy.srcMemoryType = cuda_core::sys::CUmemorytype_enum_CU_MEMORYTYPE_DEVICE;
        copy.srcDevice = self.dev_ptr;
        copy.srcPitch = self.pitch as usize;
        copy.dstMemoryType = cuda_core::sys::CUmemorytype_enum_CU_MEMORYTYPE_HOST;
        copy.dstHost = dst_host;
        copy.dstPitch = width_bytes;
        copy.WidthInBytes = width_bytes;
        copy.Height = rows;
        // SAFETY: `self.dev_ptr` is mapped for `rows` of `self.pitch` bytes, `dst_host` holds `width_bytes * rows`
        // bytes, and `stream.synchronize()` completes before returning.
        unsafe { cuda_core::sys::cuMemcpy2DAsync_v2(&copy, stream.cu_stream()) }.result()?;
        stream.synchronize()?;
        Ok(())
    }
}

impl Drop for MappedFrame<'_> {
    fn drop(&mut self) {
        if let Err(e) = self.decoder.cuda.bind_to_thread() {
            self.decoder.cuda.record_err(Err::<(), _>(e));
            return;
        }
        // SAFETY: `self.dev_ptr` was mapped by `cuvidMapVideoFrame64` on `self.decoder.raw`.
        let status = unsafe { sys::cuvidUnmapVideoFrame64(self.decoder.raw, self.dev_ptr) };
        if status != cuda_core::sys::cudaError_enum_CUDA_SUCCESS {
            self.decoder.cuda.record_err(Err::<(), _>(DriverError(status)));
        }
    }
}

/// Tight NV12 byte count for `width x height` pixels (`width * height * 3 / 2`).
pub const fn nv12_frame_bytes(width: u32, height: u32) -> Option<usize> {
    let w = width as usize;
    let h = height as usize;
    match w.checked_mul(h) {
        Some(p) => match p.checked_mul(3) {
            Some(n) => Some(n / 2),
            None => None,
        },
        None => None,
    }
}

/// Tight P016 `u16` element count for `width x height` pixels (`width * height * 3 / 2`).
pub const fn p016_frame_u16s(width: u32, height: u32) -> Option<usize> {
    nv12_frame_bytes(width, height)
}

/// Computes Peak Signal-to-Noise Ratio (dB) between two equal-length 8-bit buffers.
///
/// Returns `f64::INFINITY` when identical and `None` when empty or mismatched in length.
pub fn psnr_u8(a: &[u8], b: &[u8]) -> Option<f64> {
    if a.is_empty() || a.len() != b.len() {
        return None;
    }
    let mse: f64 = a
        .iter()
        .zip(b.iter())
        .map(|(&x, &y)| {
            let d = f64::from(x) - f64::from(y);
            d * d
        })
        .sum::<f64>()
        / (a.len() as f64);
    if mse == 0.0 {
        Some(f64::INFINITY)
    } else {
        Some(10.0 * (255.0_f64 * 255.0_f64 / mse).log10())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_size_math_and_overflow() {
        assert_eq!(nv12_frame_bytes(1920, 1080), Some(1920 * 1080 * 3 / 2));
        assert_eq!(p016_frame_u16s(1920, 1080), Some(1920 * 1080 * 3 / 2));
        assert_eq!(nv12_frame_bytes(u32::MAX, u32::MAX), None);
    }

    #[test]
    fn psnr_u8_behaves_correctly() {
        let a = [100u8; 16];
        assert_eq!(psnr_u8(&a, &a), Some(f64::INFINITY));
        assert_eq!(psnr_u8(&[], &[]), None);
        assert_eq!(psnr_u8(&a, &[100u8; 8]), None);
        let mut b = a;
        b[0] = 101;
        let p = psnr_u8(&a, &b).expect("valid psnr");
        assert!(p > 50.0 && p < 70.0, "unexpected psnr {p}");
    }

    #[test]
    fn status_code_is_copy_and_displays() {
        let code = StatusCode(cuda_core::sys::cudaError_enum_CUDA_ERROR_INVALID_VALUE);
        let copy = code;
        assert_eq!(code, copy);
        assert!(code.to_string().contains("NVDEC status"));
    }

    struct PanicHandler;
    impl ParserHandler for PanicHandler {
        fn on_sequence(&mut self, _fmt: &VideoFormat) -> Result<u32> {
            panic!("boom in sequence");
        }
        fn on_decode(&mut self, _pic: &sys::CUVIDPICPARAMS) -> Result<()> {
            panic!("boom in decode");
        }
        fn on_display(&mut self, _disp: Option<&DisplayInfo>) -> Result<()> {
            panic!("boom in display");
        }
    }

    #[test]
    fn callback_catch_unwind_records_panic_instead_of_aborting() {
        let mut inner = Box::new(ParserInner {
            handler: PanicHandler,
            pending_error: None,
        });
        let user_data = (&raw mut *inner).cast::<c_void>();
        // SAFETY: all-zero bit pattern is valid for `CUVIDPARSERDISPINFO` in this unit test.
        let mut disp: sys::CUVIDPARSERDISPINFO = unsafe { zeroed() };
        // SAFETY: `user_data` points to `inner` and `disp` is a valid struct.
        let rc = unsafe { on_display_cb::<PanicHandler>(user_data, &mut disp) };
        assert_eq!(rc, 0);
        match inner.pending_error.take() {
            Some(Error::CallbackPanic(msg)) => assert!(msg.contains("boom in display")),
            other => panic!("expected CallbackPanic, got {other:?}"),
        }
    }
}
