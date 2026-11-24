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
use cuda_core_ext::media;
use std::any::Any;
use std::ffi::{c_int, c_ulong, c_void};
use std::fmt;
use std::mem::zeroed;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr::NonNull;
use std::sync::{Arc, Mutex, MutexGuard};

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
        return Err(Error::Unavailable(format!(
            "{call}: nvcuvid library not loaded"
        )));
    }
    if code == cuda_core::sys::cudaError_enum_CUDA_ERROR_NOT_FOUND {
        return Err(Error::Unavailable(format!(
            "{call}: symbol missing from nvcuvid"
        )));
    }
    Err(Error::Status {
        call,
        code: StatusCode(code),
    })
}

fn ensure_loaded() -> Result<()> {
    sys::load()
        .map(|_| ())
        .map_err(|e| Error::Unavailable(e.to_string()))
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
            x if x == sys::cudaVideoChromaFormat_enum_cudaVideoChromaFormat_Monochrome => {
                Some(Self::Monochrome)
            }
            x if x == sys::cudaVideoChromaFormat_enum_cudaVideoChromaFormat_420 => {
                Some(Self::Yuv420)
            }
            x if x == sys::cudaVideoChromaFormat_enum_cudaVideoChromaFormat_422 => {
                Some(Self::Yuv422)
            }
            x if x == sys::cudaVideoChromaFormat_enum_cudaVideoChromaFormat_444 => {
                Some(Self::Yuv444)
            }
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
            Self::Yuv444_16Bit => {
                sys::cudaVideoSurfaceFormat_enum_cudaVideoSurfaceFormat_YUV444_16Bit
            }
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
pub fn decoder_caps(
    cuda: &Arc<Context>,
    codec: Codec,
    chroma: ChromaFormat,
    bit_depth_minus8: u32,
) -> Result<DecoderCaps> {
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
    /// Return `Ok(surface_count)`: `> 1` overrides the parser's DPB size, `1` keeps
    /// [`ParserConfig::max_num_decode_surfaces`] (`nvcuvid.h`, `PFNVIDSEQUENCECALLBACK`). A handler
    /// that creates a [`VideoDecoder`] should return that decoder's
    /// [`DecoderConfig::num_decode_surfaces`], so the parser and the decoder use the same number
    /// of decode surfaces.
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

/// Keeps the first callback error of a parse call; later ones are usually its consequences.
fn record_first_error<H: ParserHandler>(inner: &mut ParserInner<H>, err: Error) {
    if inner.pending_error.is_none() {
        inner.pending_error = Some(err);
    }
}

/// Converts `on_sequence`'s surface count to the callback's `int` (`1` = keep parser default).
fn sequence_return(count: u32) -> Result<c_int> {
    c_int::try_from(count.max(1)).map_err(|_| {
        Error::Invalid("on_sequence surface count exceeds the parser's int range")
    })
}

/// Converts a packet length to `CUVIDSOURCEDATAPACKET::payload_size` (`c_ulong`, 32-bit on Windows).
fn payload_size(len: usize) -> Result<c_ulong> {
    c_ulong::try_from(len).map_err(|_| Error::Invalid("packet payload exceeds the c_ulong range"))
}

unsafe extern "C" fn on_sequence_cb<H: ParserHandler>(
    user_data: *mut c_void,
    fmt: *mut sys::CUVIDEOFORMAT,
) -> c_int {
    if user_data.is_null() || fmt.is_null() {
        return 0;
    }
    // SAFETY: `user_data` points to `ParserInner<H>` owned by `VideoParser` for the call duration;
    // `fmt` is a valid `CUVIDEOFORMAT` pointer provided by `nvcuvid`.
    let (inner, fmt_ref) = unsafe { (&mut *user_data.cast::<ParserInner<H>>(), &*fmt) };
    let vf = VideoFormat::from_sys(fmt_ref);
    match catch_unwind(AssertUnwindSafe(|| inner.handler.on_sequence(&vf))) {
        Ok(Ok(n)) => match sequence_return(n) {
            Ok(v) => v,
            Err(e) => {
                record_first_error(inner, e);
                0
            }
        },
        Ok(Err(e)) => {
            record_first_error(inner, e);
            0
        }
        Err(p) => {
            record_first_error(inner, Error::CallbackPanic(panic_payload_msg(p)));
            0
        }
    }
}

unsafe extern "C" fn on_decode_cb<H: ParserHandler>(
    user_data: *mut c_void,
    pic: *mut sys::CUVIDPICPARAMS,
) -> c_int {
    if user_data.is_null() || pic.is_null() {
        return 0;
    }
    // SAFETY: `user_data` points to `ParserInner<H>` and `pic` is provided by `nvcuvid`.
    let (inner, pic_ref) = unsafe { (&mut *user_data.cast::<ParserInner<H>>(), &*pic) };
    match catch_unwind(AssertUnwindSafe(|| inner.handler.on_decode(pic_ref))) {
        Ok(Ok(())) => 1,
        Ok(Err(e)) => {
            record_first_error(inner, e);
            0
        }
        Err(p) => {
            record_first_error(inner, Error::CallbackPanic(panic_payload_msg(p)));
            0
        }
    }
}

unsafe extern "C" fn on_display_cb<H: ParserHandler>(
    user_data: *mut c_void,
    disp: *mut sys::CUVIDPARSERDISPINFO,
) -> c_int {
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
            record_first_error(inner, e);
            0
        }
        Err(p) => {
            record_first_error(inner, Error::CallbackPanic(panic_payload_msg(p)));
            0
        }
    }
}

/// Hardware bitstream parser (`CUvideoparser`) with `catch_unwind`-protected callbacks.
pub struct VideoParser<H: ParserHandler> {
    raw: sys::CUvideoparser,
    cuda: Arc<Context>,
    /// Owned `Box<ParserInner<H>>` held as a raw pointer: the parser's callbacks write through the
    /// same pointer (`pUserData`), so no `Box` (unique, retagged on every move) may own it while
    /// the parser exists. Created by `Box::leak` in `new`, freed by `Box::from_raw` in `Drop`.
    inner: NonNull<ParserInner<H>>,
}

impl<H: ParserHandler> VideoParser<H> {
    /// Creates a `CUvideoparser` driving `handler`.
    pub fn new(cuda: &Arc<Context>, cfg: &ParserConfig, handler: H) -> Result<Self> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let inner = NonNull::from(Box::leak(Box::new(ParserInner {
            handler,
            pending_error: None,
        })));
        // SAFETY: all-zero bit pattern is valid for `CUVIDPARSERPARAMS`.
        let mut params: sys::CUVIDPARSERPARAMS = unsafe { zeroed() };
        params.CodecType = cfg.codec.to_sys();
        params.ulMaxNumDecodeSurfaces = cfg.max_num_decode_surfaces.max(1);
        params.ulClockRate = cfg.clock_rate;
        params.ulErrorThreshold = cfg.error_threshold.min(100);
        params.ulMaxDisplayDelay = cfg.max_display_delay;
        params.pUserData = inner.as_ptr().cast::<c_void>();
        params.pfnSequenceCallback = Some(on_sequence_cb::<H>);
        params.pfnDecodePicture = Some(on_decode_cb::<H>);
        params.pfnDisplayPicture = Some(on_display_cb::<H>);

        let mut raw: sys::CUvideoparser = std::ptr::null_mut();
        // SAFETY: `params.pUserData` is the leaked heap allocation `inner`, which outlives the parser;
        // `raw` is a valid out-pointer.
        let status = unsafe { sys::cuvidCreateVideoParser(&mut raw, &mut params) };
        if let Err(e) = check("cuvidCreateVideoParser", status) {
            // SAFETY: `inner` came from `Box::leak` above and no parser exists that could use it.
            drop(unsafe { Box::from_raw(inner.as_ptr()) });
            return Err(e);
        }
        Ok(Self {
            raw,
            cuda: cuda.clone(),
            inner,
        })
    }

    /// Takes the error recorded by a callback, if any.
    fn take_pending_error(&mut self) -> Option<Error> {
        // SAFETY: `self.inner` is live until `Drop`; callbacks run only inside `cuvidParseVideoData`,
        // which is never executing while this `&mut self` method runs, so this access is exclusive.
        unsafe { (*self.inner.as_ptr()).pending_error.take() }
    }

    /// Parses one compressed bitstream packet with zero heap allocations.
    pub fn parse_packet(&mut self, payload: &[u8], timestamp: Option<i64>) -> Result<()> {
        if let Some(err) = self.take_pending_error() {
            return Err(err);
        }
        self.cuda.bind_to_thread()?;
        // SAFETY: all-zero bit pattern is valid for `CUVIDSOURCEDATAPACKET`.
        let mut pkt: sys::CUVIDSOURCEDATAPACKET = unsafe { zeroed() };
        pkt.payload_size = payload_size(payload.len())?;
        pkt.payload = if payload.is_empty() {
            std::ptr::null()
        } else {
            payload.as_ptr()
        };
        if let Some(ts) = timestamp {
            pkt.flags |= sys::CUvideopacketflags_CUVID_PKT_TIMESTAMP as c_ulong;
            pkt.timestamp = ts;
        }
        // SAFETY: `self.raw` is a live parser and `pkt.payload` is valid for the synchronous call;
        // no Rust reference to `*self.inner` is live while the callbacks write through it.
        let status = unsafe { sys::cuvidParseVideoData(self.raw, &mut pkt) };
        if let Some(err) = self.take_pending_error() {
            return Err(err);
        }
        check("cuvidParseVideoData", status)
    }

    /// Signals End-of-Stream to flush all remaining display callbacks.
    pub fn flush_eos(&mut self) -> Result<()> {
        if let Some(err) = self.take_pending_error() {
            return Err(err);
        }
        self.cuda.bind_to_thread()?;
        // SAFETY: all-zero bit pattern is valid for `CUVIDSOURCEDATAPACKET`.
        let mut pkt: sys::CUVIDSOURCEDATAPACKET = unsafe { zeroed() };
        pkt.flags = (sys::CUvideopacketflags_CUVID_PKT_ENDOFSTREAM
            | sys::CUvideopacketflags_CUVID_PKT_NOTIFY_EOS) as c_ulong;
        // SAFETY: `self.raw` is a live parser; EOS packet carries null/0 payload; no Rust reference
        // to `*self.inner` is live while the callbacks write through it.
        let status = unsafe { sys::cuvidParseVideoData(self.raw, &mut pkt) };
        if let Some(err) = self.take_pending_error() {
            return Err(err);
        }
        check("cuvidParseVideoData(EOS)", status)
    }

    /// Shared reference to the inner [`ParserHandler`].
    pub fn handler(&self) -> &H {
        // SAFETY: `self.inner` is live until `Drop`; callbacks (the only writers besides `&mut self`
        // methods) run only inside `parse_packet` / `flush_eos`, which need `&mut self` and so
        // cannot overlap this shared borrow.
        unsafe { &self.inner.as_ref().handler }
    }

    /// Mutable reference to the inner [`ParserHandler`].
    pub fn handler_mut(&mut self) -> &mut H {
        // SAFETY: as in `handler`; `&mut self` makes this the only access for the borrow's lifetime.
        unsafe { &mut self.inner.as_mut().handler }
    }
}

impl<H: ParserHandler> Drop for VideoParser<H> {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            match self.cuda.bind_to_thread() {
                Err(e) => self.cuda.record_err(Err::<(), _>(e)),
                Ok(()) => {
                    // SAFETY: `self.raw` is owned by this `VideoParser`.
                    let status = unsafe { sys::cuvidDestroyVideoParser(self.raw) };
                    if status != cuda_core::sys::cudaError_enum_CUDA_SUCCESS {
                        self.cuda.record_err(Err::<(), _>(DriverError(status)));
                    }
                }
            }
        }
        // SAFETY: `self.inner` came from `Box::leak` in `new` and is freed only here. The parser can
        // no longer call back: it is destroyed, or (failed bind) its handle is unreachable, and
        // callbacks only run inside `cuvidParseVideoData` on that handle.
        drop(unsafe { Box::from_raw(self.inner.as_ptr()) });
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
    /// Crop rectangle of the coded frame that is scaled into the target size
    /// (`CUVIDDECODECREATEINFO::display_area`); all-zero selects the full coded frame.
    pub display_area: DisplayArea,
    /// Post-processed output width (aligned to 2).
    pub target_width: u32,
    /// Post-processed output height (aligned to 2).
    pub target_height: u32,
    /// Maximum number of output surfaces simultaneously mapped (`>= 1`).
    pub num_output_surfaces: u32,
}

impl DecoderConfig {
    /// Builds a [`DecoderConfig`] from a [`VideoFormat`] reported by [`ParserHandler::on_sequence`].
    ///
    /// The output is the display rectangle cropped from the coded frame (for example 1920x1080 out
    /// of a 1920x1088 H.264 frame), not the coded frame rescaled. Return the resulting
    /// `num_decode_surfaces` from `on_sequence` (see [`ParserHandler::on_sequence`]).
    pub fn from_video_format(fmt: &VideoFormat, output_format: SurfaceFormat) -> Result<Self> {
        let codec = fmt.codec.ok_or(Error::Invalid(
            "unsupported or unknown codec in sequence header",
        ))?;
        let chroma_format = fmt
            .chroma_format
            .ok_or(Error::Invalid("unsupported or unknown chroma format"))?;
        let w = fmt.display_width();
        let h = fmt.display_height();
        // Same fallback as `display_width` / `display_height`: a degenerate axis uses the coded extent.
        let area = fmt.display_area;
        let (left, right) = if area.right > area.left {
            (area.left, area.right)
        } else {
            let coded = i32::try_from(fmt.coded_width)
                .map_err(|_| Error::Invalid("coded width exceeds the i32 range"))?;
            (0, coded)
        };
        let (top, bottom) = if area.bottom > area.top {
            (area.top, area.bottom)
        } else {
            let coded = i32::try_from(fmt.coded_height)
                .map_err(|_| Error::Invalid("coded height exceeds the i32 range"))?;
            (0, coded)
        };
        Ok(Self {
            width: fmt.coded_width,
            height: fmt.coded_height,
            num_decode_surfaces: u32::from(fmt.min_num_decode_surfaces).max(4),
            codec,
            chroma_format,
            bit_depth_minus8: u32::from(fmt.bit_depth_luma_minus8),
            output_format,
            deinterlace_mode: DeinterlaceMode::Weave,
            display_area: DisplayArea {
                left,
                top,
                right,
                bottom,
            },
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
    /// Serializes `cuvidDecodePicture` / `cuvidMapVideoFrame64` / `cuvidUnmapVideoFrame64` on `raw`:
    /// `nvcuvid.h` documents no thread safety for concurrent calls on one decoder handle.
    calls: Mutex<()>,
}

// SAFETY: the decoder handle is not tied to the creating thread; every call binds `cuda` first.
unsafe impl Send for VideoDecoder {}
// SAFETY: every nvcuvid call on `raw` through `&self` is serialized by `calls`; destruction needs
// exclusive ownership (`Drop`).
unsafe impl Sync for VideoDecoder {}

impl VideoDecoder {
    /// Creates a hardware video decoder on `cuda`.
    pub fn new(cuda: &Arc<Context>, cfg: &DecoderConfig) -> Result<Arc<Self>> {
        ensure_loaded()?;
        if cfg.width == 0 || cfg.height == 0 || cfg.target_width == 0 || cfg.target_height == 0 {
            return Err(Error::Invalid("decoder dimensions must be non-zero"));
        }
        if !cfg.target_width.is_multiple_of(2) || !cfg.target_height.is_multiple_of(2) {
            return Err(Error::Invalid(
                "decoder target_width and target_height must be multiples of 2",
            ));
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
        let short = |v: i32| {
            i16::try_from(v).map_err(|_| Error::Invalid("display_area exceeds the 16-bit range"))
        };
        info.display_area.left = short(cfg.display_area.left)?;
        info.display_area.top = short(cfg.display_area.top)?;
        info.display_area.right = short(cfg.display_area.right)?;
        info.display_area.bottom = short(cfg.display_area.bottom)?;
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
            calls: Mutex::new(()),
        }))
    }

    fn lock_calls(&self) -> MutexGuard<'_, ()> {
        self.calls.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Submits one parsed picture (`CUVIDPICPARAMS`) for hardware decoding.
    ///
    /// # Safety
    /// `pic` and all internal bitstream/slice pointers it contains (`pBitstreamData`,
    /// `pSliceDataOffsets`) must be valid for the duration of this call (guaranteed when `pic` is
    /// received from [`ParserHandler::on_decode`]).
    pub unsafe fn decode_picture(&self, pic: &sys::CUVIDPICPARAMS) -> Result<()> {
        self.cuda.bind_to_thread()?;
        let _guard = self.lock_calls();
        // SAFETY: `self.raw` is a live decoder and `pic` (plus its internal pointers) is valid per caller contract.
        let status =
            unsafe { sys::cuvidDecodePicture(self.raw, std::ptr::from_ref(pic).cast_mut()) };
        check("cuvidDecodePicture", status)
    }

    /// Maps a decoded display surface for CUDA access; unmapped automatically on `Drop`.
    ///
    /// Post-processing is enqueued on `stream` (`None` = the legacy NULL stream). The frame keeps
    /// that stream so the `copy_*_to_host` methods wait for it when they copy on another stream.
    pub fn map_frame<'a>(
        &'a self,
        disp: &DisplayInfo,
        stream: Option<&'a Stream>,
    ) -> Result<MappedFrame<'a>> {
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
        let status = {
            let _guard = self.lock_calls();
            // SAFETY: `self.raw` is live and `dev_ptr`/`pitch` are valid out-pointers.
            unsafe {
                sys::cuvidMapVideoFrame64(
                    self.raw,
                    disp.picture_index,
                    &mut dev_ptr,
                    &mut pitch,
                    &mut proc_params,
                )
            }
        };
        check("cuvidMapVideoFrame64", status)?;
        Ok(MappedFrame {
            decoder: self,
            map_stream: stream,
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
    /// Stream the post-processing was enqueued on (`None` = legacy NULL stream).
    map_stream: Option<&'a Stream>,
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
        let expected = media::yuv420_frame_bytes(self.width() as usize, self.height())
            .ok_or(Error::Invalid("NV12 frame size overflow"))?;
        if dst.len() != expected {
            return Err(Error::Invalid(
                "destination slice length does not match NV12 frame size",
            ));
        }
        let rows = media::yuv420_rows(self.height())
            .ok_or(Error::Invalid("NV12 frame size overflow"))? as usize;
        self.copy_2d_to_host(stream, self.width() as usize, rows, dst.as_mut_ptr().cast())
    }

    /// Copies a tight P016 surface (`Y` + interleaved `UV` in `u16`s) to `dst` on `stream` with zero allocations.
    pub fn copy_p016_to_host(&self, stream: &Stream, dst: &mut [u16]) -> Result<()> {
        if self.decoder.output_format != SurfaceFormat::P016 {
            return Err(Error::Invalid("surface format is not P016"));
        }
        let expected = (self.width() as usize)
            .checked_mul(2)
            .and_then(|row_bytes| media::yuv420_frame_bytes(row_bytes, self.height()))
            .ok_or(Error::Invalid("P016 frame size overflow"))?
            / 2;
        if dst.len() != expected {
            return Err(Error::Invalid(
                "destination slice length does not match P016 frame size",
            ));
        }
        let rows = media::yuv420_rows(self.height())
            .ok_or(Error::Invalid("P016 frame size overflow"))? as usize;
        self.copy_2d_to_host(
            stream,
            (self.width() as usize) * 2,
            rows,
            dst.as_mut_ptr().cast(),
        )
    }

    fn copy_2d_to_host(
        &self,
        stream: &Stream,
        width_bytes: usize,
        rows: usize,
        dst_host: *mut c_void,
    ) -> Result<()> {
        self.decoder.cuda.bind_to_thread()?;
        // `cuvidMapVideoFrame64` post-processes on the map stream; a copy on any other stream
        // (cuda-core streams are non-blocking, so not even the legacy NULL stream orders them)
        // must wait for it. The copy below is synchronous anyway, so a host wait costs nothing extra.
        let map_handle = self
            .map_stream
            .map_or(std::ptr::null_mut(), |s| s.cu_stream());
        if map_handle != stream.cu_stream() {
            // SAFETY: `map_handle` is the legacy NULL stream or a stream borrowed for `'a`, so it is
            // live, and it belongs to the decoder's context, which is current.
            unsafe { cuda_core::sys::cuStreamSynchronize(map_handle) }.result()?;
        }
        // SAFETY: `self.dev_ptr` is mapped for `rows` of `self.pitch` bytes, `dst_host` holds
        // `width_bytes * rows` bytes, and the copy waits for `stream` before returning.
        unsafe {
            media::copy_2d_device_to_host(
                stream,
                self.dev_ptr,
                self.pitch as usize,
                dst_host,
                width_bytes,
                rows,
            )
        }?;
        Ok(())
    }
}

impl Drop for MappedFrame<'_> {
    fn drop(&mut self) {
        if let Err(e) = self.decoder.cuda.bind_to_thread() {
            self.decoder.cuda.record_err(Err::<(), _>(e));
            return;
        }
        let _guard = self.decoder.lock_calls();
        // SAFETY: `self.dev_ptr` was mapped by `cuvidMapVideoFrame64` on `self.decoder.raw`.
        let status = unsafe { sys::cuvidUnmapVideoFrame64(self.decoder.raw, self.dev_ptr) };
        if status != cuda_core::sys::cudaError_enum_CUDA_SUCCESS {
            self.decoder
                .cuda
                .record_err(Err::<(), _>(DriverError(status)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn first_callback_error_is_kept() {
        let mut inner = Box::new(ParserInner {
            handler: PanicHandler,
            pending_error: None,
        });
        let user_data = (&raw mut *inner).cast::<c_void>();
        // SAFETY: all-zero bit patterns are valid for these plain C structs in this unit test.
        let mut pic: sys::CUVIDPICPARAMS = unsafe { zeroed() };
        // SAFETY: see above.
        let mut disp: sys::CUVIDPARSERDISPINFO = unsafe { zeroed() };
        // SAFETY: `user_data` points to `inner`; `pic` and `disp` are valid structs.
        let (rc_decode, rc_display) = unsafe {
            (
                on_decode_cb::<PanicHandler>(user_data, &mut pic),
                on_display_cb::<PanicHandler>(user_data, &mut disp),
            )
        };
        assert_eq!((rc_decode, rc_display), (0, 0));
        match inner.pending_error.take() {
            Some(Error::CallbackPanic(msg)) => assert!(msg.contains("boom in decode"), "{msg}"),
            other => panic!("expected the decode panic, got {other:?}"),
        }
    }

    struct HugeSurfaceCount;
    impl ParserHandler for HugeSurfaceCount {
        fn on_sequence(&mut self, _fmt: &VideoFormat) -> Result<u32> {
            Ok(u32::MAX)
        }
        fn on_decode(&mut self, _pic: &sys::CUVIDPICPARAMS) -> Result<()> {
            Ok(())
        }
        fn on_display(&mut self, _disp: Option<&DisplayInfo>) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn out_of_range_surface_count_fails_the_sequence_callback() {
        let mut inner = Box::new(ParserInner {
            handler: HugeSurfaceCount,
            pending_error: None,
        });
        let user_data = (&raw mut *inner).cast::<c_void>();
        // SAFETY: all-zero bit pattern is valid for `CUVIDEOFORMAT` in this unit test.
        let mut fmt: sys::CUVIDEOFORMAT = unsafe { zeroed() };
        // SAFETY: `user_data` points to `inner` and `fmt` is a valid struct.
        let rc = unsafe { on_sequence_cb::<HugeSurfaceCount>(user_data, &mut fmt) };
        assert_eq!(rc, 0, "u32::MAX must not wrap to a negative int");
        assert!(matches!(inner.pending_error.take(), Some(Error::Invalid(_))));
        assert_eq!(sequence_return(0).ok(), Some(1));
        assert_eq!(sequence_return(8).ok(), Some(8));
    }

    #[test]
    fn payload_size_rejects_lengths_beyond_c_ulong() {
        assert_eq!(payload_size(4096).ok(), Some(4096));
        let fits = std::mem::size_of::<c_ulong>() >= std::mem::size_of::<usize>();
        assert_eq!(payload_size(usize::MAX).is_ok(), fits);
    }

    fn format(coded: (u32, u32), area: DisplayArea) -> VideoFormat {
        VideoFormat {
            codec: Some(Codec::H264),
            coded_width: coded.0,
            coded_height: coded.1,
            display_area: area,
            chroma_format: Some(ChromaFormat::Yuv420),
            bit_depth_luma_minus8: 0,
            bit_depth_chroma_minus8: 0,
            min_num_decode_surfaces: 5,
            progressive_sequence: true,
            fps_num: 30,
            fps_den: 1,
        }
    }

    #[test]
    fn decoder_config_crops_to_the_display_area() {
        let crop = DisplayArea {
            left: 0,
            top: 0,
            right: 1920,
            bottom: 1080,
        };
        let cfg = DecoderConfig::from_video_format(&format((1920, 1088), crop), SurfaceFormat::Nv12)
            .expect("config");
        assert_eq!((cfg.width, cfg.height), (1920, 1088));
        assert_eq!(cfg.display_area, crop);
        assert_eq!((cfg.target_width, cfg.target_height), (1920, 1080));

        let empty = DisplayArea {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        let cfg = DecoderConfig::from_video_format(&format((256, 144), empty), SurfaceFormat::Nv12)
            .expect("config");
        assert_eq!(
            cfg.display_area,
            DisplayArea {
                left: 0,
                top: 0,
                right: 256,
                bottom: 144
            }
        );
        assert_eq!((cfg.target_width, cfg.target_height), (256, 144));
    }
}
