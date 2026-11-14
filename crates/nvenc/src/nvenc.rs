//! Safe NVENC hardware video encoder wrapper over `nvenc-sys` and `cuda-core`.
//!
//! **What.** Wraps NVIDIA's Video Codec SDK encoder API:
//! - [`EncodeSession`] — opens an NVENC session bound to a `cuda_core::Context`, queries
//!   preset configs ([`EncodeSession::preset_config`]) and capabilities ([`EncodeSession::caps`]),
//!   initializes the encoder ([`EncodeSession::initialize`]), and encodes frames or flushes EOS
//!   with zero per-frame heap allocations.
//! - [`InputBuffer`] / [`InputLock`] — host-accessible driver-allocated input buffers with RAII
//!   lock/unlock.
//! - [`RegisteredResource`] / [`MappedResource`] — zero-copy CUDA device-pointer or CUDA-array
//!   input registration and per-frame mapping RAII guards.
//! - [`OutputBitstream`] / [`BitstreamLock`] — driver-allocated output bitstream buffers with RAII
//!   lock/unlock yielding a `&[u8]` slice of the encoded packet.
//! - [`guid`] — authoritative H.264, HEVC, AV1, profile, and `P1`–`P7` preset GUIDs.
//!
//! **Conventions.** Every handle owns its NVENC object, holds an `Arc` of its parent
//! (`EncodeSession` / `cuda_core::Context`), and destroys it on `Drop`; teardown failures are
//! recorded on the CUDA context (`Context::record_err`) rather than panicking.
//! **Used by** media pipelines and the `nvdec` round-trip PSNR tests.

pub mod guid;

pub use nvenc_sys as sys;

use cuda_core::{Context, DriverError, Stream};
use std::ffi::{CStr, c_void};
use std::fmt;
use std::marker::PhantomData;
use std::mem::zeroed;
use std::slice;
use std::sync::Arc;

/// Raw `NVENCSTATUS` code wrapped as a `Copy` integer newtype so newer driver codes stay valid.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StatusCode(pub sys::NVENCSTATUS);

impl StatusCode {
    /// Returns the underlying integer value.
    pub const fn raw(self) -> sys::NVENCSTATUS {
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
        write!(f, "NVENC status {}", self.0)
    }
}

/// Everything an NVENC call can fail with.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `nvEncodeAPI` could not be loaded or is missing an entry point.
    #[error("NVENC unavailable: {0}")]
    Unavailable(String),
    /// An NVENC entry point returned a non-success status.
    #[error("{call} failed with {code}{}", detail_suffix(.detail))]
    Status {
        /// The NVENC function-list entry point.
        call: &'static str,
        /// The raw `NVENCSTATUS` code.
        code: StatusCode,
        /// Optional message from `nvEncGetLastErrorString`.
        detail: String,
    },
    /// A CUDA driver call failed.
    #[error("CUDA: {0}")]
    Driver(#[from] DriverError),
    /// The device or driver does not support the requested codec, profile, or capability.
    #[error("unsupported {what}: required {required}, found {found}")]
    Unsupported {
        /// What was requested.
        what: &'static str,
        /// Minimum required value or identifier.
        required: u32,
        /// Reported capability value.
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
    /// An argument was rejected before reaching NVENC.
    #[error("invalid argument: {0}")]
    Invalid(&'static str),
}

fn detail_suffix(detail: &str) -> String {
    if detail.is_empty() {
        String::new()
    } else {
        format!(" ({detail})")
    }
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Struct version helper matching `NVENCAPI_STRUCT_VERSION(ver) | (1 << 31)` for high-bit-set structs.
const fn struct_version_hi(ver: u32) -> u32 {
    sys::struct_version(ver) | (1u32 << 31)
}

const VER_OPEN_SESSION_EX: u32 = sys::struct_version(1);
const VER_CREATE_INPUT_BUFFER: u32 = sys::struct_version(2);
const VER_CREATE_BITSTREAM_BUFFER: u32 = sys::struct_version(1);
const VER_REGISTER_RESOURCE: u32 = sys::struct_version(5);
const VER_MAP_INPUT_RESOURCE: u32 = sys::struct_version(4);
const VER_LOCK_INPUT_BUFFER: u32 = sys::struct_version(1);
const VER_CAPS_PARAM: u32 = sys::struct_version(1);
const VER_CONFIG: u32 = struct_version_hi(9);
const VER_INITIALIZE_PARAMS: u32 = struct_version_hi(7);
const VER_PRESET_CONFIG: u32 = struct_version_hi(5);
const VER_PIC_PARAMS: u32 = struct_version_hi(7);
const VER_LOCK_BITSTREAM: u32 = struct_version_hi(2);

fn fn_list() -> Result<&'static sys::NV_ENCODE_API_FUNCTION_LIST> {
    sys::api()
        .map(|a| &a.list)
        .map_err(|e| Error::Unavailable(e.to_string()))
}

macro_rules! entry {
    ($list:expr, $name:ident) => {
        $list.$name.ok_or_else(|| {
            Error::Unavailable(
                concat!(stringify!($name), " missing from NVENC function list").into(),
            )
        })?
    };
}

fn check(
    call: &'static str,
    status: sys::NVENCSTATUS,
    encoder: Option<*mut c_void>,
    list: &sys::NV_ENCODE_API_FUNCTION_LIST,
) -> Result<()> {
    if status == sys::_NVENCSTATUS_NV_ENC_SUCCESS {
        return Ok(());
    }
    let detail = match (encoder, list.nvEncGetLastErrorString) {
        (Some(enc), Some(get_err)) if !enc.is_null() => {
            // SAFETY: `enc` is a live encoder handle and `nvEncGetLastErrorString` returns a static or session-owned NUL-terminated string.
            let ptr = unsafe { get_err(enc) };
            if ptr.is_null() {
                String::new()
            } else {
                // SAFETY: `ptr` is non-null and NUL-terminated per NVENC spec.
                unsafe { CStr::from_ptr(ptr) }
                    .to_string_lossy()
                    .trim()
                    .to_owned()
            }
        }
        _ => String::new(),
    };
    Err(Error::Status {
        call,
        code: StatusCode(status),
        detail,
    })
}

/// Supported video codecs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Codec {
    /// H.264 / AVC.
    H264,
    /// HEVC / H.265.
    Hevc,
    /// AV1.
    Av1,
}

impl Codec {
    /// Returns the authoritative NVENC codec GUID.
    pub const fn guid(self) -> sys::GUID {
        match self {
            Self::H264 => guid::CODEC_H264,
            Self::Hevc => guid::CODEC_HEVC,
            Self::Av1 => guid::CODEC_AV1,
        }
    }
}

/// NVENC quality/speed presets (`P1` fastest .. `P7` highest quality).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preset {
    /// Fastest preset.
    P1,
    /// Fast preset.
    P2,
    /// Balanced-fast preset.
    P3,
    /// Medium preset.
    P4,
    /// Balanced-quality preset.
    P5,
    /// High-quality preset.
    P6,
    /// Highest-quality preset.
    P7,
}

impl Preset {
    /// Returns the authoritative NVENC preset GUID.
    pub const fn guid(self) -> sys::GUID {
        match self {
            Self::P1 => guid::PRESET_P1,
            Self::P2 => guid::PRESET_P2,
            Self::P3 => guid::PRESET_P3,
            Self::P4 => guid::PRESET_P4,
            Self::P5 => guid::PRESET_P5,
            Self::P6 => guid::PRESET_P6,
            Self::P7 => guid::PRESET_P7,
        }
    }
}

/// NVENC tuning mode (`NV_ENC_TUNING_INFO`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tuning {
    /// High quality, latency tolerant.
    HighQuality,
    /// Low latency streaming.
    LowLatency,
    /// Ultra-low latency streaming.
    UltraLowLatency,
    /// Lossless encoding.
    Lossless,
    /// Ultra-high quality (HEVC / AV1 on Turing+).
    UltraHighQuality,
}

impl Tuning {
    const fn to_sys(self) -> sys::NV_ENC_TUNING_INFO {
        match self {
            Self::HighQuality => sys::NV_ENC_TUNING_INFO_NV_ENC_TUNING_INFO_HIGH_QUALITY,
            Self::LowLatency => sys::NV_ENC_TUNING_INFO_NV_ENC_TUNING_INFO_LOW_LATENCY,
            Self::UltraLowLatency => sys::NV_ENC_TUNING_INFO_NV_ENC_TUNING_INFO_ULTRA_LOW_LATENCY,
            Self::Lossless => sys::NV_ENC_TUNING_INFO_NV_ENC_TUNING_INFO_LOSSLESS,
            Self::UltraHighQuality => sys::NV_ENC_TUNING_INFO_NV_ENC_TUNING_INFO_ULTRA_HIGH_QUALITY,
        }
    }
}

/// Surface pixel formats accepted by NVENC.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferFormat {
    /// 8-bit semi-planar YUV 4:2:0 (Y plane followed by interleaved UV).
    Nv12,
    /// 8-bit planar YUV 4:2:0 (Y, V, U).
    Yv12,
    /// 8-bit planar YUV 4:2:0 (Y, U, V).
    Iyuv,
    /// 8-bit planar YUV 4:4:4.
    Yuv444,
    /// 10-bit semi-planar YUV 4:2:0 (P010/P016 MSBs).
    Yuv420_10Bit,
    /// 10-bit planar YUV 4:4:4.
    Yuv444_10Bit,
    /// 8-bit packed ARGB (`A8R8G8B8`).
    Argb,
    /// 8-bit packed ABGR (`A8B8G8R8`).
    Abgr,
    /// 10-bit packed ARGB (`A2R10G10B10`).
    Argb10,
    /// 10-bit packed ABGR (`A2B10G10R10`).
    Abgr10,
}

impl BufferFormat {
    const fn to_sys(self) -> sys::NV_ENC_BUFFER_FORMAT {
        match self {
            Self::Nv12 => sys::_NV_ENC_BUFFER_FORMAT_NV_ENC_BUFFER_FORMAT_NV12,
            Self::Yv12 => sys::_NV_ENC_BUFFER_FORMAT_NV_ENC_BUFFER_FORMAT_YV12,
            Self::Iyuv => sys::_NV_ENC_BUFFER_FORMAT_NV_ENC_BUFFER_FORMAT_IYUV,
            Self::Yuv444 => sys::_NV_ENC_BUFFER_FORMAT_NV_ENC_BUFFER_FORMAT_YUV444,
            Self::Yuv420_10Bit => sys::_NV_ENC_BUFFER_FORMAT_NV_ENC_BUFFER_FORMAT_YUV420_10BIT,
            Self::Yuv444_10Bit => sys::_NV_ENC_BUFFER_FORMAT_NV_ENC_BUFFER_FORMAT_YUV444_10BIT,
            Self::Argb => sys::_NV_ENC_BUFFER_FORMAT_NV_ENC_BUFFER_FORMAT_ARGB,
            Self::Abgr => sys::_NV_ENC_BUFFER_FORMAT_NV_ENC_BUFFER_FORMAT_ABGR,
            Self::Argb10 => sys::_NV_ENC_BUFFER_FORMAT_NV_ENC_BUFFER_FORMAT_ARGB10,
            Self::Abgr10 => sys::_NV_ENC_BUFFER_FORMAT_NV_ENC_BUFFER_FORMAT_ABGR10,
        }
    }

    /// Minimum tight byte size for a frame of `width x height` pixels, or `None` on overflow.
    pub const fn tight_frame_bytes(self, width: u32, height: u32) -> Option<usize> {
        let w = width as usize;
        let h = height as usize;
        let pixels = match w.checked_mul(h) {
            Some(p) => p,
            None => return None,
        };
        match self {
            Self::Nv12 | Self::Yv12 | Self::Iyuv => match pixels.checked_mul(3) {
                Some(n) => Some(n / 2),
                None => None,
            },
            Self::Yuv420_10Bit => pixels.checked_mul(3),
            Self::Yuv444 => pixels.checked_mul(3),
            Self::Yuv444_10Bit => pixels.checked_mul(6),
            Self::Argb | Self::Abgr | Self::Argb10 | Self::Abgr10 => pixels.checked_mul(4),
        }
    }

    /// Tight row pitch in bytes for the luma / primary plane.
    pub const fn tight_pitch(self, width: u32) -> Option<u32> {
        match self {
            Self::Nv12 | Self::Yv12 | Self::Iyuv | Self::Yuv444 => Some(width),
            Self::Yuv420_10Bit | Self::Yuv444_10Bit => width.checked_mul(2),
            Self::Argb | Self::Abgr | Self::Argb10 | Self::Abgr10 => width.checked_mul(4),
        }
    }

    /// Maximum number of `pitch`-byte rows in a locked [`InputBuffer`] of `height` pixels.
    pub const fn locked_rows(self, height: u32) -> Option<u32> {
        match self {
            Self::Nv12 | Self::Yv12 | Self::Iyuv | Self::Yuv420_10Bit => {
                height.checked_add(height / 2)
            }
            Self::Yuv444 | Self::Yuv444_10Bit => height.checked_mul(3),
            Self::Argb | Self::Abgr | Self::Argb10 | Self::Abgr10 => Some(height),
        }
    }
}

/// Typed input surface passed to [`EncodeSession::encode_picture`].
pub enum EncodeInput<'a> {
    /// Host-accessible driver input buffer.
    Buffer(&'a InputBuffer),
    /// Mapped zero-copy CUDA device pointer or CUDA array resource.
    Mapped(&'a MappedResource<'a>),
}

/// Picture type reported on an encoded packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PictureType {
    /// Forward predicted P-frame.
    P,
    /// Bi-directionally predicted B-frame.
    B,
    /// Intra I-frame.
    I,
    /// Instantaneous Decoder Refresh (IDR) keyframe.
    Idr,
    /// Intra-only B-frame.
    Bi,
    /// Skipped frame.
    Skipped,
    /// Intra-refresh start frame.
    IntraRefresh,
    /// Non-reference P-frame.
    NonRefP,
    /// AV1 switch frame.
    Switch,
    /// Unknown or driver-unspecified picture type.
    Unknown(sys::NV_ENC_PIC_TYPE),
}

impl PictureType {
    fn from_sys(raw: sys::NV_ENC_PIC_TYPE) -> Self {
        match raw {
            x if x == sys::_NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_P => Self::P,
            x if x == sys::_NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_B => Self::B,
            x if x == sys::_NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_I => Self::I,
            x if x == sys::_NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_IDR => Self::Idr,
            x if x == sys::_NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_BI => Self::Bi,
            x if x == sys::_NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_SKIPPED => Self::Skipped,
            x if x == sys::_NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_INTRA_REFRESH => Self::IntraRefresh,
            x if x == sys::_NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_NONREF_P => Self::NonRefP,
            x if x == sys::_NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_SWITCH => Self::Switch,
            other => Self::Unknown(other),
        }
    }
}

/// Wrapper around `NV_ENC_CONFIG` returned by [`EncodeSession::preset_config`].
#[derive(Clone)]
pub struct EncodeConfig {
    /// Raw `NV_ENC_CONFIG` struct; callers may tune codec-specific fields directly.
    pub raw: sys::NV_ENC_CONFIG,
}

impl EncodeConfig {
    /// Sets the GOP length (`NVENC_INFINITE_GOPLENGTH` = `0xFFFF_FFFF` for infinite GOP).
    pub fn set_gop_length(&mut self, gop_length: u32) {
        self.raw.gopLength = gop_length;
    }

    /// Sets `frameIntervalP` (0 = I-only, 1 = IPP, 2 = IBP, 3 = IBBP).
    pub fn set_frame_interval_p(&mut self, interval: i32) {
        self.raw.frameIntervalP = interval;
    }

    /// Sets the codec profile GUID (defaults to `PROFILE_AUTOSELECT` from preset).
    pub fn set_profile(&mut self, profile_guid: sys::GUID) {
        self.raw.profileGUID = profile_guid;
    }

    /// Sets average bitrate in bits per second (CBR/VBR).
    pub fn set_average_bitrate(&mut self, bps: u32) {
        self.raw.rcParams.averageBitRate = bps;
    }
}

/// High-level encoder initialization parameters.
#[derive(Clone)]
pub struct SessionConfig {
    /// Video codec (H.264, HEVC, AV1).
    pub codec: Codec,
    /// Quality/speed preset (`P1`–`P7`).
    pub preset: Preset,
    /// Tuning mode.
    pub tuning: Tuning,
    /// Encoded frame width in pixels.
    pub width: u32,
    /// Encoded frame height in pixels.
    pub height: u32,
    /// Frame rate numerator (e.g. 30 or 60).
    pub fps_num: u32,
    /// Frame rate denominator (e.g. 1).
    pub fps_den: u32,
    /// Enable NVENC Picture Type Decision (`enablePTD = 1`).
    pub enable_ptd: bool,
    /// Optional custom `EncodeConfig` (overrides preset defaults).
    pub encode_config: Option<EncodeConfig>,
}

impl SessionConfig {
    /// Minimal configuration for `codec` at `width x height` and `fps_num / fps_den`.
    pub fn new(codec: Codec, width: u32, height: u32, fps_num: u32, fps_den: u32) -> Self {
        Self {
            codec,
            preset: Preset::P4,
            tuning: Tuning::HighQuality,
            width,
            height,
            fps_num,
            fps_den,
            enable_ptd: true,
            encode_config: None,
        }
    }
}

/// Per-frame encode flags.
#[derive(Clone, Copy, Debug, Default)]
pub struct EncodeFlags {
    /// Force this frame to be encoded as an Intra frame.
    pub force_intra: bool,
    /// Force this frame to be encoded as an IDR keyframe.
    pub force_idr: bool,
    /// Emit SPS/PPS headers with this frame.
    pub output_sps_pps: bool,
}

impl EncodeFlags {
    const fn to_bits(self) -> u32 {
        let mut bits = 0u32;
        if self.force_intra {
            bits |= sys::_NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_FORCEINTRA as u32;
        }
        if self.force_idr {
            bits |= sys::_NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_FORCEIDR as u32;
        }
        if self.output_sps_pps {
            bits |= sys::_NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_OUTPUT_SPSPPS as u32;
        }
        bits
    }
}

/// Metadata returned when locking an encoded bitstream packet.
#[derive(Clone, Copy, Debug)]
pub struct PacketInfo {
    /// Frame index.
    pub frame_idx: u32,
    /// Presentation timestamp echoed from `inputTimeStamp`.
    pub timestamp: u64,
    /// Duration echoed from `inputDuration`.
    pub duration: u64,
    /// Encoded picture type.
    pub picture_type: PictureType,
    /// Average QP used for the frame.
    pub frame_avg_qp: u32,
    /// Byte length of the encoded bitstream payload.
    pub size_bytes: usize,
}

/// An open NVENC hardware encoder session bound to a `cuda_core::Context`.
pub struct EncodeSession {
    raw: *mut c_void,
    cuda: Arc<Context>,
    list: &'static sys::NV_ENCODE_API_FUNCTION_LIST,
    initialized: bool,
}

// SAFETY: NVENC session calls are synchronized by the caller / driver per session, and the CUDA
// context handle is thread-safe.
unsafe impl Send for EncodeSession {}
// SAFETY: see `Send` above — shared references are safe when callers serialize encode submissions
// or rely on driver-level session locking.
unsafe impl Sync for EncodeSession {}

impl EncodeSession {
    /// Opens an NVENC encoder session on `cuda`.
    pub fn open(cuda: &Arc<Context>) -> Result<Arc<Self>> {
        let list = fn_list()?;
        cuda.bind_to_thread()?;
        let open_fn = entry!(list, nvEncOpenEncodeSessionEx);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS`;
        // `version`, `deviceType`, `device`, and `apiVersion` are populated below.
        let mut params: sys::NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS = unsafe { zeroed() };
        params.version = VER_OPEN_SESSION_EX;
        params.deviceType = sys::_NV_ENC_DEVICE_TYPE_NV_ENC_DEVICE_TYPE_CUDA;
        params.device = cuda.cu_ctx().cast::<c_void>();
        params.apiVersion = sys::API_VERSION;

        let mut raw: *mut c_void = std::ptr::null_mut();
        // SAFETY: `params` is properly versioned and `cuda.cu_ctx()` is a live CUDA context.
        let status = unsafe { open_fn(&mut params, &mut raw) };
        if status != sys::_NVENCSTATUS_NV_ENC_SUCCESS {
            // NVENC spec: if OpenEncodeSessionEx fails and returns a non-null handle, destroy it.
            if !raw.is_null()
                && let Some(destroy_fn) = list.nvEncDestroyEncoder
            {
                // SAFETY: `raw` was returned by `nvEncOpenEncodeSessionEx`.
                let d_status = unsafe { destroy_fn(raw) };
                if d_status != sys::_NVENCSTATUS_NV_ENC_SUCCESS {
                    cuda.record_err(Err::<(), _>(DriverError(
                        cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
                    )));
                }
            }
            return check("nvEncOpenEncodeSessionEx", status, None, list).map(|()| unreachable!());
        }
        Ok(Arc::new(Self {
            raw,
            cuda: cuda.clone(),
            list,
            initialized: false,
        }))
    }

    /// Returns the owning CUDA context.
    pub fn cuda(&self) -> &Arc<Context> {
        &self.cuda
    }

    /// Queries an integer capability (`NV_ENC_CAPS`) for `codec`.
    pub fn caps(&self, codec: Codec, cap: sys::NV_ENC_CAPS) -> Result<i32> {
        let get_caps = entry!(self.list, nvEncGetEncodeCaps);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_CAPS_PARAM`.
        let mut param: sys::NV_ENC_CAPS_PARAM = unsafe { zeroed() };
        param.version = VER_CAPS_PARAM;
        param.capsToQuery = cap;
        let mut val = 0i32;
        // SAFETY: `self.raw` is a live encoder session; `param` and `val` are valid pointers.
        let status = unsafe { get_caps(self.raw, codec.guid(), &mut param, &mut val) };
        check("nvEncGetEncodeCaps", status, Some(self.raw), self.list)?;
        Ok(val)
    }

    /// Queries the driver's default [`EncodeConfig`] for `codec`, `preset`, and `tuning`.
    pub fn preset_config(
        &self,
        codec: Codec,
        preset: Preset,
        tuning: Tuning,
    ) -> Result<EncodeConfig> {
        let get_preset = entry!(self.list, nvEncGetEncodePresetConfigEx);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_PRESET_CONFIG`.
        let mut out: sys::NV_ENC_PRESET_CONFIG = unsafe { zeroed() };
        out.version = VER_PRESET_CONFIG;
        out.presetCfg.version = VER_CONFIG;
        // SAFETY: `self.raw` is a live session and `out` is properly versioned.
        let status = unsafe {
            get_preset(
                self.raw,
                codec.guid(),
                preset.guid(),
                tuning.to_sys(),
                &mut out,
            )
        };
        check(
            "nvEncGetEncodePresetConfigEx",
            status,
            Some(self.raw),
            self.list,
        )?;
        Ok(EncodeConfig { raw: out.presetCfg })
    }

    /// Initializes the encoder with `cfg`. Must be called once before encoding frames.
    pub fn initialize(self: &mut Arc<Self>, cfg: &SessionConfig) -> Result<()> {
        if cfg.width == 0 || cfg.height == 0 {
            return Err(Error::Invalid("encode width and height must be non-zero"));
        }
        if cfg.fps_num == 0 || cfg.fps_den == 0 {
            return Err(Error::Invalid(
                "frame rate numerator and denominator must be non-zero",
            ));
        }
        let this = Arc::get_mut(self).ok_or(Error::Invalid(
            "cannot initialize session while child buffers exist",
        ))?;
        if this.initialized {
            return Err(Error::Invalid("encode session is already initialized"));
        }
        this.cuda.bind_to_thread()?;
        let init_fn = entry!(this.list, nvEncInitializeEncoder);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_INITIALIZE_PARAMS`.
        let mut params: sys::NV_ENC_INITIALIZE_PARAMS = unsafe { zeroed() };
        params.version = VER_INITIALIZE_PARAMS;
        params.encodeGUID = cfg.codec.guid();
        params.presetGUID = cfg.preset.guid();
        params.tuningInfo = cfg.tuning.to_sys();
        params.encodeWidth = cfg.width;
        params.encodeHeight = cfg.height;
        params.darWidth = cfg.width;
        params.darHeight = cfg.height;
        params.frameRateNum = cfg.fps_num;
        params.frameRateDen = cfg.fps_den;
        params.enableEncodeAsync = 0;
        params.enablePTD = u32::from(cfg.enable_ptd);

        let mut custom_cfg = cfg.encode_config.as_ref().map(|c| c.raw);
        if let Some(ref mut raw_cfg) = custom_cfg {
            raw_cfg.version = VER_CONFIG;
            params.encodeConfig = (&raw mut *raw_cfg).cast();
        }
        // SAFETY: `this.raw` is a live session and `params` (plus optional `custom_cfg`) outlives the call.
        let status = unsafe { init_fn(this.raw, &mut params) };
        check("nvEncInitializeEncoder", status, Some(this.raw), this.list)?;
        this.initialized = true;
        Ok(())
    }

    /// Binds input and output CUDA streams for subsequent encode calls (`nvEncSetIOCudaStreams`).
    pub fn set_io_streams(&self, input: &Stream, output: &Stream) -> Result<()> {
        let set_streams = entry!(self.list, nvEncSetIOCudaStreams);
        let mut in_stream = input.cu_stream().cast::<c_void>();
        let mut out_stream = output.cu_stream().cast::<c_void>();
        // SAFETY: `self.raw` is a live session and both `CUstream` handles are valid.
        let status = unsafe {
            set_streams(
                self.raw,
                (&raw mut in_stream).cast(),
                (&raw mut out_stream).cast(),
            )
        };
        check("nvEncSetIOCudaStreams", status, Some(self.raw), self.list)
    }

    /// Encodes one typed input surface into `output` with zero heap allocations.
    pub fn encode_picture(
        &self,
        input: EncodeInput<'_>,
        output: &mut OutputBitstream,
        frame_idx: u32,
        timestamp: u64,
        flags: EncodeFlags,
    ) -> Result<()> {
        if !self.initialized {
            return Err(Error::Invalid("encode session is not initialized"));
        }
        if output.session.raw != self.raw {
            return Err(Error::Invalid(
                "output bitstream belongs to a different encode session",
            ));
        }
        let (input_ptr, format, width, height, pitch, input_session) = match input {
            EncodeInput::Buffer(b) => {
                let p = b
                    .format
                    .tight_pitch(b.width)
                    .ok_or(Error::Invalid("input pitch overflow"))?;
                (b.raw, b.format, b.width, b.height, p, b.session.raw)
            }
            EncodeInput::Mapped(m) => (
                m.mapped,
                m.res.format,
                m.res.width,
                m.res.height,
                m.res.pitch,
                m.res.session.raw,
            ),
        };
        if input_session != self.raw {
            return Err(Error::Invalid(
                "input surface belongs to a different encode session",
            ));
        }
        if width == 0 || height == 0 || pitch == 0 {
            return Err(Error::Invalid(
                "input dimensions and pitch must be non-zero",
            ));
        }
        let encode_fn = entry!(self.list, nvEncEncodePicture);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_PIC_PARAMS`.
        let mut pic: sys::NV_ENC_PIC_PARAMS = unsafe { zeroed() };
        pic.version = VER_PIC_PARAMS;
        pic.inputWidth = width;
        pic.inputHeight = height;
        pic.inputPitch = pitch;
        pic.encodePicFlags = flags.to_bits();
        pic.frameIdx = frame_idx;
        pic.inputTimeStamp = timestamp;
        pic.inputBuffer = input_ptr;
        pic.outputBitstream = output.raw;
        pic.bufferFmt = format.to_sys();
        pic.pictureStruct = sys::_NV_ENC_PIC_STRUCT_NV_ENC_PIC_STRUCT_FRAME;
        pic.pictureType = sys::_NV_ENC_PIC_TYPE_NV_ENC_PIC_TYPE_UNKNOWN;
        // SAFETY: `self.raw` is initialized and `input_ptr`/`output` belong to this session.
        let status = unsafe { encode_fn(self.raw, &mut pic) };
        check("nvEncEncodePicture", status, Some(self.raw), self.list)
    }

    /// Sends an End-of-Stream (EOS) notification to flush any buffered frames.
    pub fn flush_eos(&self) -> Result<()> {
        if !self.initialized {
            return Err(Error::Invalid("encode session is not initialized"));
        }
        let encode_fn = entry!(self.list, nvEncEncodePicture);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_PIC_PARAMS`.
        let mut pic: sys::NV_ENC_PIC_PARAMS = unsafe { zeroed() };
        pic.version = VER_PIC_PARAMS;
        pic.encodePicFlags = sys::_NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_EOS as u32;
        // SAFETY: `self.raw` is initialized; EOS requires no input/output buffers.
        let status = unsafe { encode_fn(self.raw, &mut pic) };
        check("nvEncEncodePicture(EOS)", status, Some(self.raw), self.list)
    }
}

impl Drop for EncodeSession {
    fn drop(&mut self) {
        if self.raw.is_null() {
            return;
        }
        if let Err(e) = self.cuda.bind_to_thread() {
            self.cuda.record_err(Err::<(), _>(e));
            return;
        }
        if let Some(destroy_fn) = self.list.nvEncDestroyEncoder {
            // SAFETY: `self.raw` is owned by this `EncodeSession` and no child handles remain (they hold `Arc<EncodeSession>`).
            let status = unsafe { destroy_fn(self.raw) };
            if status != sys::_NVENCSTATUS_NV_ENC_SUCCESS {
                self.cuda.record_err(Err::<(), _>(DriverError(
                    cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
                )));
            }
        }
    }
}

/// Host-accessible driver-allocated input buffer (`nvEncCreateInputBuffer`).
pub struct InputBuffer {
    raw: sys::NV_ENC_INPUT_PTR,
    session: Arc<EncodeSession>,
    width: u32,
    height: u32,
    format: BufferFormat,
}

impl InputBuffer {
    /// Allocates a driver input buffer for `width x height` pixels in `format`.
    pub fn new(
        session: &Arc<EncodeSession>,
        width: u32,
        height: u32,
        format: BufferFormat,
    ) -> Result<Self> {
        if width == 0 || height == 0 {
            return Err(Error::Invalid("input buffer dimensions must be non-zero"));
        }
        let create_fn = entry!(session.list, nvEncCreateInputBuffer);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_CREATE_INPUT_BUFFER`.
        let mut params: sys::NV_ENC_CREATE_INPUT_BUFFER = unsafe { zeroed() };
        params.version = VER_CREATE_INPUT_BUFFER;
        params.width = width;
        params.height = height;
        params.bufferFmt = format.to_sys();
        // SAFETY: `session.raw` is a live encoder session.
        let status = unsafe { create_fn(session.raw, &mut params) };
        check(
            "nvEncCreateInputBuffer",
            status,
            Some(session.raw),
            session.list,
        )?;
        Ok(Self {
            raw: params.inputBuffer,
            session: session.clone(),
            width,
            height,
            format,
        })
    }

    /// Locks the buffer for CPU write access; unlocked automatically on `Drop`.
    pub fn lock(&mut self) -> Result<InputLock<'_>> {
        let lock_fn = entry!(self.session.list, nvEncLockInputBuffer);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_LOCK_INPUT_BUFFER`.
        let mut params: sys::NV_ENC_LOCK_INPUT_BUFFER = unsafe { zeroed() };
        params.version = VER_LOCK_INPUT_BUFFER;
        params.inputBuffer = self.raw;
        // SAFETY: `self.raw` is a live input buffer owned by `self.session`.
        let status = unsafe { lock_fn(self.session.raw, &mut params) };
        check(
            "nvEncLockInputBuffer",
            status,
            Some(self.session.raw),
            self.session.list,
        )?;
        Ok(InputLock {
            buf: self,
            ptr: params.bufferDataPtr.cast::<u8>(),
            pitch: params.pitch,
        })
    }

    /// Raw NVENC input pointer.
    pub fn raw(&self) -> sys::NV_ENC_INPUT_PTR {
        self.raw
    }

    /// Buffer width in pixels.
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Buffer height in pixels.
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Buffer pixel format.
    pub const fn format(&self) -> BufferFormat {
        self.format
    }
}

impl Drop for InputBuffer {
    fn drop(&mut self) {
        if let Some(destroy_fn) = self.session.list.nvEncDestroyInputBuffer {
            // SAFETY: `self.raw` was created by `nvEncCreateInputBuffer` on `self.session`.
            let status = unsafe { destroy_fn(self.session.raw, self.raw) };
            if status != sys::_NVENCSTATUS_NV_ENC_SUCCESS {
                self.session.cuda.record_err(Err::<(), _>(DriverError(
                    cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
                )));
            }
        }
    }
}

/// RAII guard for a locked [`InputBuffer`].
pub struct InputLock<'a> {
    buf: &'a mut InputBuffer,
    ptr: *mut u8,
    pitch: u32,
}

impl InputLock<'_> {
    /// Locked row pitch in bytes.
    pub const fn pitch(&self) -> u32 {
        self.pitch
    }

    /// Returns a mutable slice covering `rows` rows of `pitch` bytes each.
    ///
    /// For NV12, `rows` is at most `height + height / 2`; for packed RGB/RGBA, at most `height`.
    pub fn as_mut_slice(&mut self, rows: u32) -> Result<&mut [u8]> {
        let max_rows = self
            .buf
            .format
            .locked_rows(self.buf.height)
            .ok_or(Error::Invalid("input buffer height overflow"))?;
        if rows > max_rows {
            return Err(Error::Invalid(
                "requested rows exceed allocated input buffer height",
            ));
        }
        let len = (self.pitch as usize)
            .checked_mul(rows as usize)
            .ok_or(Error::Invalid("locked slice size overflow"))?;
        if self.ptr.is_null() {
            return Err(Error::Invalid("locked input buffer pointer is null"));
        }
        // SAFETY: NVENC guarantees `self.ptr` points to at least `pitch * max_rows` writable bytes until unlock.
        Ok(unsafe { slice::from_raw_parts_mut(self.ptr, len) })
    }
}

impl Drop for InputLock<'_> {
    fn drop(&mut self) {
        if let Some(unlock_fn) = self.buf.session.list.nvEncUnlockInputBuffer {
            // SAFETY: `self.buf.raw` is currently locked by this guard.
            let status = unsafe { unlock_fn(self.buf.session.raw, self.buf.raw) };
            if status != sys::_NVENCSTATUS_NV_ENC_SUCCESS {
                self.buf.session.cuda.record_err(Err::<(), _>(DriverError(
                    cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
                )));
            }
        }
    }
}

/// Registered external CUDA device pointer or CUDA array (`nvEncRegisterResource`).
pub struct RegisteredResource {
    raw: sys::NV_ENC_REGISTERED_PTR,
    session: Arc<EncodeSession>,
    width: u32,
    height: u32,
    pitch: u32,
    format: BufferFormat,
}

impl RegisteredResource {
    /// Registers a CUDA device pointer (`CUdeviceptr`) with NVENC for zero-copy encoding.
    ///
    /// # Safety
    /// `device_ptr` must remain valid and allocated on `session.cuda()` for the lifetime of this
    /// [`RegisteredResource`], with at least `pitch * height` (or `pitch * height * 3 / 2` for NV12) bytes.
    pub unsafe fn from_cuda_ptr(
        session: &Arc<EncodeSession>,
        device_ptr: cuda_core::sys::CUdeviceptr,
        width: u32,
        height: u32,
        pitch: u32,
        format: BufferFormat,
    ) -> Result<Self> {
        if device_ptr == 0 || width == 0 || height == 0 || pitch == 0 || !pitch.is_multiple_of(4) {
            return Err(Error::Invalid(
                "CUDA pointer, dimensions, and 4-byte-aligned pitch must be valid",
            ));
        }
        let reg_fn = entry!(session.list, nvEncRegisterResource);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_REGISTER_RESOURCE`.
        let mut params: sys::NV_ENC_REGISTER_RESOURCE = unsafe { zeroed() };
        params.version = VER_REGISTER_RESOURCE;
        params.resourceType =
            sys::_NV_ENC_INPUT_RESOURCE_TYPE_NV_ENC_INPUT_RESOURCE_TYPE_CUDADEVICEPTR;
        params.width = width;
        params.height = height;
        params.pitch = pitch;
        params.resourceToRegister = device_ptr as *mut c_void;
        params.bufferFormat = format.to_sys();
        params.bufferUsage = sys::_NV_ENC_BUFFER_USAGE_NV_ENC_INPUT_IMAGE;
        // SAFETY: `session.raw` is live and `device_ptr` is valid per caller contract.
        let status = unsafe { reg_fn(session.raw, &mut params) };
        check(
            "nvEncRegisterResource",
            status,
            Some(session.raw),
            session.list,
        )?;
        Ok(Self {
            raw: params.registeredResource,
            session: session.clone(),
            width,
            height,
            pitch,
            format,
        })
    }

    /// Registers a 2D CUDA array (`CUarray`) created with `CUDA_ARRAY3D_SURFACE_LDST`.
    ///
    /// # Safety
    /// `array` must be a valid 2D `CUarray` on `session.cuda()` and outlive this [`RegisteredResource`].
    pub unsafe fn from_cuda_array(
        session: &Arc<EncodeSession>,
        array: cuda_core::sys::CUarray,
        width: u32,
        height: u32,
        pitch_bytes: u32,
        format: BufferFormat,
    ) -> Result<Self> {
        if array.is_null() || width == 0 || height == 0 || pitch_bytes == 0 {
            return Err(Error::Invalid(
                "CUDA array handle and dimensions must be non-zero",
            ));
        }
        let reg_fn = entry!(session.list, nvEncRegisterResource);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_REGISTER_RESOURCE`.
        let mut params: sys::NV_ENC_REGISTER_RESOURCE = unsafe { zeroed() };
        params.version = VER_REGISTER_RESOURCE;
        params.resourceType = sys::_NV_ENC_INPUT_RESOURCE_TYPE_NV_ENC_INPUT_RESOURCE_TYPE_CUDAARRAY;
        params.width = width;
        params.height = height;
        params.pitch = pitch_bytes;
        params.resourceToRegister = array.cast::<c_void>();
        params.bufferFormat = format.to_sys();
        params.bufferUsage = sys::_NV_ENC_BUFFER_USAGE_NV_ENC_INPUT_IMAGE;
        // SAFETY: `session.raw` is live and `array` is valid per caller contract.
        let status = unsafe { reg_fn(session.raw, &mut params) };
        check(
            "nvEncRegisterResource",
            status,
            Some(session.raw),
            session.list,
        )?;
        Ok(Self {
            raw: params.registeredResource,
            session: session.clone(),
            width,
            height,
            pitch: pitch_bytes,
            format,
        })
    }

    /// Maps the registered resource for one encode submission; unmapped automatically on `Drop`.
    pub fn map(&mut self) -> Result<MappedResource<'_>> {
        let map_fn = entry!(self.session.list, nvEncMapInputResource);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_MAP_INPUT_RESOURCE`.
        let mut params: sys::NV_ENC_MAP_INPUT_RESOURCE = unsafe { zeroed() };
        params.version = VER_MAP_INPUT_RESOURCE;
        params.registeredResource = self.raw;
        // SAFETY: `self.raw` is a live registered resource on `self.session`.
        let status = unsafe { map_fn(self.session.raw, &mut params) };
        check(
            "nvEncMapInputResource",
            status,
            Some(self.session.raw),
            self.session.list,
        )?;
        Ok(MappedResource {
            res: self,
            mapped: params.mappedResource,
        })
    }

    /// Width in pixels.
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Height in pixels.
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Pitch in bytes.
    pub const fn pitch(&self) -> u32 {
        self.pitch
    }

    /// Pixel format.
    pub const fn format(&self) -> BufferFormat {
        self.format
    }
}

impl Drop for RegisteredResource {
    fn drop(&mut self) {
        if let Some(unreg_fn) = self.session.list.nvEncUnregisterResource {
            // SAFETY: `self.raw` was registered on `self.session` and all `MappedResource` guards have dropped.
            let status = unsafe { unreg_fn(self.session.raw, self.raw) };
            if status != sys::_NVENCSTATUS_NV_ENC_SUCCESS {
                self.session.cuda.record_err(Err::<(), _>(DriverError(
                    cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
                )));
            }
        }
    }
}

/// RAII guard for a mapped [`RegisteredResource`].
pub struct MappedResource<'a> {
    res: &'a mut RegisteredResource,
    mapped: sys::NV_ENC_INPUT_PTR,
}

impl MappedResource<'_> {
    /// Mapped NVENC input pointer to pass to [`EncodeSession::encode_picture`].
    pub fn raw(&self) -> sys::NV_ENC_INPUT_PTR {
        self.mapped
    }
}

impl Drop for MappedResource<'_> {
    fn drop(&mut self) {
        if let Some(unmap_fn) = self.res.session.list.nvEncUnmapInputResource {
            // SAFETY: `self.mapped` was returned by `nvEncMapInputResource`.
            let status = unsafe { unmap_fn(self.res.session.raw, self.mapped) };
            if status != sys::_NVENCSTATUS_NV_ENC_SUCCESS {
                self.res.session.cuda.record_err(Err::<(), _>(DriverError(
                    cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
                )));
            }
        }
    }
}

/// Driver-allocated output bitstream buffer (`nvEncCreateBitstreamBuffer`).
pub struct OutputBitstream {
    raw: sys::NV_ENC_OUTPUT_PTR,
    session: Arc<EncodeSession>,
}

impl OutputBitstream {
    /// Allocates an output bitstream buffer on `session`.
    pub fn new(session: &Arc<EncodeSession>) -> Result<Self> {
        let create_fn = entry!(session.list, nvEncCreateBitstreamBuffer);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_CREATE_BITSTREAM_BUFFER`.
        let mut params: sys::NV_ENC_CREATE_BITSTREAM_BUFFER = unsafe { zeroed() };
        params.version = VER_CREATE_BITSTREAM_BUFFER;
        // SAFETY: `session.raw` is a live encoder session.
        let status = unsafe { create_fn(session.raw, &mut params) };
        check(
            "nvEncCreateBitstreamBuffer",
            status,
            Some(session.raw),
            session.list,
        )?;
        Ok(Self {
            raw: params.bitstreamBuffer,
            session: session.clone(),
        })
    }

    /// Locks the bitstream buffer, blocking until hardware encoding finishes, and returns an RAII guard.
    pub fn lock(&mut self) -> Result<BitstreamLock<'_>> {
        let lock_fn = entry!(self.session.list, nvEncLockBitstream);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_LOCK_BITSTREAM`.
        let mut params: sys::NV_ENC_LOCK_BITSTREAM = unsafe { zeroed() };
        params.version = VER_LOCK_BITSTREAM;
        params.outputBitstream = self.raw;
        // SAFETY: `self.raw` is a live bitstream buffer owned by `self.session`.
        let status = unsafe { lock_fn(self.session.raw, &mut params) };
        check(
            "nvEncLockBitstream",
            status,
            Some(self.session.raw),
            self.session.list,
        )?;
        let size_bytes = params.bitstreamSizeInBytes as usize;
        let ptr = params.bitstreamBufferPtr.cast::<u8>();
        if size_bytes > 0 && ptr.is_null() {
            // Unlock before returning error so the buffer doesn't stay locked.
            if let Some(unlock_fn) = self.session.list.nvEncUnlockBitstream {
                // SAFETY: `self.raw` was just locked.
                let _ = unsafe { unlock_fn(self.session.raw, self.raw) };
            }
            return Err(Error::Invalid(
                "locked bitstream pointer is null for non-empty payload",
            ));
        }
        let info = PacketInfo {
            frame_idx: params.frameIdx,
            timestamp: params.outputTimeStamp,
            duration: params.outputDuration,
            picture_type: PictureType::from_sys(params.pictureType),
            frame_avg_qp: params.frameAvgQP,
            size_bytes,
        };
        Ok(BitstreamLock {
            buf: self,
            ptr,
            size_bytes,
            info,
            _phantom: PhantomData,
        })
    }
}

impl Drop for OutputBitstream {
    fn drop(&mut self) {
        if let Some(destroy_fn) = self.session.list.nvEncDestroyBitstreamBuffer {
            // SAFETY: `self.raw` was created by `nvEncCreateBitstreamBuffer` on `self.session`.
            let status = unsafe { destroy_fn(self.session.raw, self.raw) };
            if status != sys::_NVENCSTATUS_NV_ENC_SUCCESS {
                self.session.cuda.record_err(Err::<(), _>(DriverError(
                    cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
                )));
            }
        }
    }
}

/// RAII guard for a locked [`OutputBitstream`], yielding a zero-copy `&[u8]` of the encoded packet.
pub struct BitstreamLock<'a> {
    buf: &'a mut OutputBitstream,
    ptr: *const u8,
    size_bytes: usize,
    info: PacketInfo,
    _phantom: PhantomData<&'a [u8]>,
}

impl BitstreamLock<'_> {
    /// Encoded bitstream payload bytes.
    pub fn data(&self) -> &[u8] {
        if self.size_bytes == 0 {
            &[]
        } else {
            // SAFETY: NVENC guarantees `self.ptr` points to `self.size_bytes` readable bytes until unlock.
            unsafe { slice::from_raw_parts(self.ptr, self.size_bytes) }
        }
    }

    /// Packet metadata reported by NVENC.
    pub const fn info(&self) -> PacketInfo {
        self.info
    }
}

impl Drop for BitstreamLock<'_> {
    fn drop(&mut self) {
        if let Some(unlock_fn) = self.buf.session.list.nvEncUnlockBitstream {
            // SAFETY: `self.buf.raw` is currently locked by this guard.
            let status = unsafe { unlock_fn(self.buf.session.raw, self.buf.raw) };
            if status != sys::_NVENCSTATUS_NV_ENC_SUCCESS {
                self.buf.session.cuda.record_err(Err::<(), _>(DriverError(
                    cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guids_match_nvencodeapi_header() {
        assert_eq!(guid::CODEC_H264.Data1, 0x6bc82762);
        assert_eq!(guid::CODEC_HEVC.Data1, 0x790cdc88);
        assert_eq!(guid::CODEC_AV1.Data1, 0x0a352289);
        assert_eq!(guid::PRESET_P1.Data1, 0xfc0a8d3e);
        assert_eq!(guid::PRESET_P4.Data1, 0x90a7b826);
        assert_eq!(guid::PRESET_P7.Data1, 0x84848c12);
        assert_eq!(guid::H264_HIGH.Data1, 0xe7cbc309);
        assert_eq!(guid::HEVC_MAIN.Data1, 0xb514c39a);
        assert_eq!(guid::AV1_MAIN.Data1, 0x5f2a39f5);
    }

    #[test]
    fn struct_versions_match_header_macros() {
        assert_eq!(VER_OPEN_SESSION_EX, sys::struct_version(1));
        assert_eq!(VER_CONFIG, sys::struct_version(9) | (1u32 << 31));
        assert_eq!(VER_INITIALIZE_PARAMS, sys::struct_version(7) | (1u32 << 31));
        assert_eq!(VER_PRESET_CONFIG, sys::struct_version(5) | (1u32 << 31));
        assert_eq!(VER_PIC_PARAMS, sys::struct_version(7) | (1u32 << 31));
        assert_eq!(VER_LOCK_BITSTREAM, sys::struct_version(2) | (1u32 << 31));
    }

    #[test]
    fn buffer_format_sizes_and_overflow_checks() {
        assert_eq!(
            BufferFormat::Nv12.tight_frame_bytes(1920, 1080),
            Some(1920 * 1080 * 3 / 2)
        );
        assert_eq!(
            BufferFormat::Yuv420_10Bit.tight_frame_bytes(1920, 1080),
            Some(1920 * 1080 * 3)
        );
        assert_eq!(
            BufferFormat::Abgr.tight_frame_bytes(1920, 1080),
            Some(1920 * 1080 * 4)
        );
        assert_eq!(
            BufferFormat::Nv12.tight_frame_bytes(u32::MAX, u32::MAX),
            None
        );
        assert_eq!(BufferFormat::Abgr.tight_pitch(64), Some(256));
        assert_eq!(BufferFormat::Abgr.tight_pitch(u32::MAX), None);
    }

    #[test]
    fn status_code_is_copy_and_displays() {
        let code = StatusCode(sys::_NVENCSTATUS_NV_ENC_ERR_INVALID_PARAM);
        let copy = code;
        assert_eq!(code, copy);
        assert!(code.to_string().contains("NVENC status"));
    }
}
