//! Safe NVENC hardware video encoder wrapper over `nvenc-sys` and `cuda-core`.
//!
//! **What.** Wraps NVIDIA's Video Codec SDK encoder API:
//! - [`EncodeSession`] — opens an NVENC session bound to a `cuda_core::Context`, queries
//!   preset configs ([`EncodeSession::preset_config`]) and capabilities ([`EncodeSession::caps`]),
//!   initializes the encoder ([`EncodeSession::initialize`]), and encodes frames or flushes EOS
//!   with zero per-frame heap allocations. [`EncodeOutcome::NeedMoreInput`] reports a frame
//!   buffered for B-frame reordering; its output bitstream refuses to lock until a later
//!   [`EncodeOutcome::Ready`] or EOS releases it.
//! - [`InputBuffer`] / [`InputLock`] — host-accessible driver-allocated input buffers with RAII
//!   lock/unlock.
//! - [`RegisteredResource`] / [`MappedResource`] — zero-copy CUDA device-pointer or CUDA-array
//!   input registration and per-frame mapping RAII guards.
//! - [`OutputBitstream`] / [`BitstreamLock`] — driver-allocated output bitstream buffers with RAII
//!   lock/unlock yielding a `&[u8]` slice of the encoded packet.
//! - [`guid`] — authoritative H.264, HEVC, AV1, profile, and `P1`–`P7` preset GUIDs.
//!
//! **Conventions.** Every handle owns its NVENC object, holds an `Arc` of its parent
//! (`EncodeSession` / `cuda_core::Context`), and destroys it on `Drop`. A failing NVENC teardown
//! call is kept with its real `NVENCSTATUS` in `cuda_core_ext::teardown` (`Library::Nvenc`); a
//! failed context bind stays on the CUDA context (`Context::record_err`). `Drop` never panics.
//! **Used by** media pipelines and the `nvdec` round-trip PSNR tests.

pub mod guid;

pub use nvenc_sys as sys;

use cuda_core::{Context, DriverError, Stream};
use cuda_core_ext::media;
use cuda_core_ext::teardown::{self, Library};
use std::ffi::{CStr, c_void};
use std::fmt;
use std::marker::PhantomData;
use std::mem::zeroed;
use std::ptr::NonNull;
use std::slice;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

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
    #[error("{call} failed with {code}{}", media::detail_suffix(.detail))]
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
    /// An argument was rejected before reaching NVENC.
    #[error("invalid argument: {0}")]
    Invalid(&'static str),
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

/// Maps an `nvEncEncodePicture` status: `NV_ENC_ERR_NEED_MORE_INPUT` is not a failure
/// (`nvEncodeAPI.h`: "this is not a fatal error ... buffering the input frame for re-ordering").
fn encode_outcome(
    status: sys::NVENCSTATUS,
    encoder: Option<*mut c_void>,
    list: &sys::NV_ENCODE_API_FUNCTION_LIST,
) -> Result<EncodeOutcome> {
    if status == sys::_NVENCSTATUS_NV_ENC_ERR_NEED_MORE_INPUT {
        return Ok(EncodeOutcome::NeedMoreInput);
    }
    check("nvEncEncodePicture", status, encoder, list).map(|()| EncodeOutcome::Ready)
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
    /// 4:2:0 formats keep an odd height's last chroma row ([`media::yuv420_frame_bytes`]).
    pub const fn tight_frame_bytes(self, width: u32, height: u32) -> Option<usize> {
        let w = width as usize;
        let h = height as usize;
        match self {
            Self::Nv12 | Self::Yv12 | Self::Iyuv => media::yuv420_frame_bytes(w, height),
            Self::Yuv420_10Bit => match w.checked_mul(2) {
                Some(row_bytes) => media::yuv420_frame_bytes(row_bytes, height),
                None => None,
            },
            Self::Yuv444 => match w.checked_mul(h) {
                Some(p) => p.checked_mul(3),
                None => None,
            },
            Self::Yuv444_10Bit => match w.checked_mul(h) {
                Some(p) => p.checked_mul(6),
                None => None,
            },
            Self::Argb | Self::Abgr | Self::Argb10 | Self::Abgr10 => match w.checked_mul(h) {
                Some(p) => p.checked_mul(4),
                None => None,
            },
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
            Self::Nv12 | Self::Yv12 | Self::Iyuv | Self::Yuv420_10Bit => media::yuv420_rows(height),
            Self::Yuv444 | Self::Yuv444_10Bit => height.checked_mul(3),
            Self::Argb | Self::Abgr | Self::Argb10 | Self::Abgr10 => Some(height),
        }
    }
}

/// Typed input surface passed to [`EncodeSession::encode_picture`].
///
/// The surface must stay mapped (a live [`MappedResource`]) and unmodified (no new
/// [`InputBuffer::lock`]) until [`OutputBitstream::lock`] has returned for the frame it carries
/// (`nvEncodeAPI.h`, `NvEncUnmapInputResource`: "The client must unmap the buffer after
/// ::NvEncLockBitstream() API returns successfully"). After [`EncodeOutcome::NeedMoreInput`] that is
/// until a later submission returns [`EncodeOutcome::Ready`] (or EOS) and the frame's output is locked.
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

/// Successful result of [`EncodeSession::encode_picture`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncodeOutcome {
    /// Encoded data is ready: lock every output bitstream still pending from earlier
    /// [`Self::NeedMoreInput`] submissions, then this call's output, in submission order.
    Ready,
    /// `NV_ENC_ERR_NEED_MORE_INPUT`: the frame was accepted and buffered for B-frame reordering.
    /// Do not lock this call's output bitstream yet (it refuses with [`Error::Invalid`]); keep its
    /// input surface alive (see [`EncodeInput`]) and submit the next frame with another output
    /// bitstream, or call [`EncodeSession::flush_eos`].
    NeedMoreInput,
}

/// Wrapper around `NV_ENC_CONFIG` returned by [`EncodeSession::preset_config`].
#[derive(Clone)]
pub struct EncodeConfig {
    /// Raw `NV_ENC_CONFIG` struct; callers may tune codec-specific fields directly.
    pub raw: sys::NV_ENC_CONFIG,
    /// Codec the preset was queried for; selects the active `encodeCodecConfig` union arm.
    codec: Codec,
    /// First codec-specific setter called for the wrong codec; reported by `initialize`.
    misuse: Option<&'static str>,
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

    /// Configures H.264 High profile with BT.709 limited-range VUI color signaling.
    ///
    /// On an [`EncodeConfig`] created for another codec nothing is written and
    /// [`EncodeSession::initialize`] fails with [`Error::Invalid`].
    pub fn set_h264_bt709_vui(&mut self) {
        if self.codec != Codec::H264 {
            self.misuse = Some("set_h264_bt709_vui called on a non-H.264 EncodeConfig");
            return;
        }
        self.raw.profileGUID = guid::H264_HIGH;
        // SAFETY: `self.codec == Codec::H264` (checked above), so `preset_config` filled the `h264Config` arm.
        let h264 = unsafe { &mut self.raw.encodeCodecConfig.h264Config };
        h264.inputBitDepth = sys::_NV_ENC_BIT_DEPTH_NV_ENC_BIT_DEPTH_8;
        h264.outputBitDepth = sys::_NV_ENC_BIT_DEPTH_NV_ENC_BIT_DEPTH_8;
        let vui = &mut h264.h264VUIParameters;
        vui.videoSignalTypePresentFlag = 1;
        vui.videoFullRangeFlag = 0;
        vui.colourDescriptionPresentFlag = 1;
        vui.colourPrimaries =
            sys::_NV_ENC_VUI_COLOR_PRIMARIES_NV_ENC_VUI_COLOR_PRIMARIES_BT709;
        vui.transferCharacteristics =
            sys::_NV_ENC_VUI_TRANSFER_CHARACTERISTIC_NV_ENC_VUI_TRANSFER_CHARACTERISTIC_BT709;
        vui.colourMatrix = sys::_NV_ENC_VUI_MATRIX_COEFFS_NV_ENC_VUI_MATRIX_COEFFS_BT709;
    }

    /// Configures HEVC Main 10 profile (10-bit input/output) with HDR10 BT.2020 PQ (SMPTE ST 2084)
    /// limited-range VUI color signaling.
    ///
    /// On an [`EncodeConfig`] created for another codec nothing is written and
    /// [`EncodeSession::initialize`] fails with [`Error::Invalid`].
    pub fn set_hevc_10bit_pq(&mut self) {
        if self.codec != Codec::Hevc {
            self.misuse = Some("set_hevc_10bit_pq called on a non-HEVC EncodeConfig");
            return;
        }
        self.raw.profileGUID = guid::HEVC_MAIN10;
        // SAFETY: `self.codec == Codec::Hevc` (checked above), so `preset_config` filled the `hevcConfig` arm.
        let hevc = unsafe { &mut self.raw.encodeCodecConfig.hevcConfig };
        hevc.inputBitDepth = sys::_NV_ENC_BIT_DEPTH_NV_ENC_BIT_DEPTH_10;
        hevc.outputBitDepth = sys::_NV_ENC_BIT_DEPTH_NV_ENC_BIT_DEPTH_10;
        let vui = &mut hevc.hevcVUIParameters;
        vui.videoSignalTypePresentFlag = 1;
        vui.videoFullRangeFlag = 0;
        vui.colourDescriptionPresentFlag = 1;
        vui.colourPrimaries =
            sys::_NV_ENC_VUI_COLOR_PRIMARIES_NV_ENC_VUI_COLOR_PRIMARIES_BT2020;
        vui.transferCharacteristics =
            sys::_NV_ENC_VUI_TRANSFER_CHARACTERISTIC_NV_ENC_VUI_TRANSFER_CHARACTERISTIC_SMPTE2084;
        vui.colourMatrix = sys::_NV_ENC_VUI_MATRIX_COEFFS_NV_ENC_VUI_MATRIX_COEFFS_BT2020_NCL;
    }

    /// Rejects a config whose union arm does not belong to `codec` or that saw a misused setter.
    fn check_codec(&self, codec: Codec) -> Result<()> {
        if let Some(msg) = self.misuse {
            return Err(Error::Invalid(msg));
        }
        if self.codec != codec {
            return Err(Error::Invalid(
                "encode_config was queried for a different codec than SessionConfig::codec",
            ));
        }
        Ok(())
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

/// Bit-reinterprets a `NV_ENC_PIC_FLAGS` constant as the `u32` `encodePicFlags` field.
///
/// bindgen types the C enum as `c_int` on MSVC and `c_uint` on GCC/Clang targets; the
/// byte round-trip is exact for both without a cast (which `clippy::unnecessary_cast` flags on
/// `c_uint` targets only).
const fn pic_flag(flag: sys::_NV_ENC_PIC_FLAGS) -> u32 {
    u32::from_ne_bytes(flag.to_ne_bytes())
}

impl EncodeFlags {
    const fn to_bits(self) -> u32 {
        let mut bits = 0u32;
        if self.force_intra {
            bits |= pic_flag(sys::_NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_FORCEINTRA);
        }
        if self.force_idr {
            bits |= pic_flag(sys::_NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_FORCEIDR);
        }
        if self.output_sps_pps {
            bits |= pic_flag(sys::_NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_OUTPUT_SPSPPS);
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
    lock: Mutex<()>,
    /// Incremented (under `lock`) by every `Ready` encode and every EOS flush; an output buffered at
    /// epoch `e` by `NeedMoreInput` becomes lockable once the epoch exceeds `e`.
    ready_epoch: AtomicU64,
    /// Streams passed to `nvEncSetIOCudaStreams`; dropped after the encoder is destroyed.
    io_streams: Mutex<Option<IoStreams>>,
}

// SAFETY: Transferring ownership of `EncodeSession` between threads is safe because NVENC
// session calls are serialized and the `Arc<Context>` / static function table are `Send + Sync`;
// `IoStreams` only owns `Arc<Stream>`s and a heap cell that is never read on the Rust side.
unsafe impl Send for EncodeSession {}
// SAFETY: Every NVENC API call using `self.raw` is serialized by `self.lock`; `io_streams` is behind
// its own `Mutex` and `ready_epoch` is atomic.
unsafe impl Sync for EncodeSession {}

/// CUDA streams handed to `nvEncSetIOCudaStreams`, kept alive until the session is destroyed or
/// a later successful call replaces them.
///
/// NVENC receives *pointers to* `CUstream` values (`NV_ENC_CUSTREAM_PTR`); the values live in an
/// owned heap cell with a stable address, so the call is sound whether the driver copies the
/// handles or keeps the pointers.
struct IoStreams {
    cells: NonNull<[*mut c_void; 2]>,
    _input: Arc<Stream>,
    _output: Arc<Stream>,
}

impl Drop for IoStreams {
    fn drop(&mut self) {
        // SAFETY: `cells` came from `Box::leak` in `set_io_streams` and is freed only here, after the
        // encoder stopped using it (session destroyed, call failed, or streams replaced).
        drop(unsafe { Box::from_raw(self.cells.as_ptr()) });
    }
}

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
                    teardown::record(Library::Nvenc, "nvEncDestroyEncoder", i64::from(d_status));
                }
            }
            return check("nvEncOpenEncodeSessionEx", status, None, list).map(|()| unreachable!());
        }
        Ok(Arc::new(Self {
            raw,
            cuda: cuda.clone(),
            list,
            initialized: false,
            lock: Mutex::new(()),
            ready_epoch: AtomicU64::new(0),
            io_streams: Mutex::new(None),
        }))
    }

    fn lock_session(&self) -> MutexGuard<'_, ()> {
        self.lock.lock().unwrap_or_else(|e| e.into_inner())
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
        let _guard = self.lock_session();
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
        let _guard = self.lock_session();
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
        Ok(EncodeConfig {
            raw: out.presetCfg,
            codec,
            misuse: None,
        })
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
        if let Some(enc_cfg) = &cfg.encode_config {
            enc_cfg.check_codec(cfg.codec)?;
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
        {
            let _guard = this.lock_session();
            // SAFETY: `this.raw` is a live session and `params` (plus optional `custom_cfg`) outlives the call.
            let status = unsafe { init_fn(this.raw, &mut params) };
            check("nvEncInitializeEncoder", status, Some(this.raw), this.list)?;
        }
        this.initialized = true;
        Ok(())
    }

    /// Binds input and output CUDA streams for subsequent encode calls (`nvEncSetIOCudaStreams`).
    ///
    /// The session keeps both streams alive until it is destroyed or this is called again, so the
    /// driver never uses a destroyed (or pooled and reused) `CUstream` handle.
    pub fn set_io_streams(&self, input: &Arc<Stream>, output: &Arc<Stream>) -> Result<()> {
        let set_streams = entry!(self.list, nvEncSetIOCudaStreams);
        let streams = IoStreams {
            cells: NonNull::from(Box::leak(Box::new([
                input.cu_stream().cast::<c_void>(),
                output.cu_stream().cast::<c_void>(),
            ]))),
            _input: input.clone(),
            _output: output.clone(),
        };
        let in_cell = streams.cells.as_ptr().cast::<*mut c_void>();
        let _guard = self.lock_session();
        // SAFETY: `self.raw` is a live session; `in_cell` and `in_cell.add(1)` point into the owned
        // two-element cell, which holds valid `CUstream` handles retained by `streams` and stays
        // allocated while NVENC may use it (stored in `self.io_streams` below on success).
        let status = unsafe { set_streams(self.raw, in_cell.cast(), in_cell.add(1).cast()) };
        check("nvEncSetIOCudaStreams", status, Some(self.raw), self.list)?;
        // Replacing drops the previous streams only after NVENC accepted the new ones.
        *self
            .io_streams
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(streams);
        Ok(())
    }

    /// Encodes one typed input surface into `output` with zero heap allocations.
    ///
    /// Returns [`EncodeOutcome::NeedMoreInput`] when NVENC buffered the frame for B-frame
    /// reordering (`frameIntervalP > 1`). The input surface must outlive the frame's bitstream
    /// lock; see [`EncodeInput`].
    pub fn encode_picture(
        &self,
        input: EncodeInput<'_>,
        output: &mut OutputBitstream,
        frame_idx: u32,
        timestamp: u64,
        flags: EncodeFlags,
    ) -> Result<EncodeOutcome> {
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
                // The driver chose the allocation pitch; pass what `nvEncLockInputBuffer` reported.
                let p = b.locked_pitch.ok_or(Error::Invalid(
                    "input buffer must be locked and filled before encoding",
                ))?;
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
        let _guard = self.lock_session();
        // SAFETY: `self.raw` is initialized and `input_ptr`/`output` belong to this session.
        let status = unsafe { encode_fn(self.raw, &mut pic) };
        let outcome = encode_outcome(status, Some(self.raw), self.list)?;
        match outcome {
            EncodeOutcome::Ready => {
                self.ready_epoch.fetch_add(1, Ordering::Relaxed);
                output.buffered_at = None;
            }
            EncodeOutcome::NeedMoreInput => {
                output.buffered_at = Some(self.ready_epoch.load(Ordering::Relaxed));
            }
        }
        Ok(outcome)
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
        pic.encodePicFlags = pic_flag(sys::_NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_EOS);
        let _guard = self.lock_session();
        // SAFETY: `self.raw` is initialized; EOS requires no input/output buffers.
        let status = unsafe { encode_fn(self.raw, &mut pic) };
        check("nvEncEncodePicture(EOS)", status, Some(self.raw), self.list)?;
        // EOS drains the reorder queue: every output still pending becomes lockable.
        self.ready_epoch.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Drop for EncodeSession {
    fn drop(&mut self) {
        if self.raw.is_null() {
            return;
        }
        // A failed bind (for example a sticky error recorded by a child drop) must not leak the
        // hardware session, a scarce per-GPU resource: record it and still attempt the destroy,
        // whose own status reports any real failure.
        if let Err(e) = self.cuda.bind_to_thread() {
            self.cuda.record_err(Err::<(), _>(e));
        }
        if let Some(destroy_fn) = self.list.nvEncDestroyEncoder {
            let _guard = self.lock_session();
            // SAFETY: `self.raw` is owned by this `EncodeSession` and no child handles remain (they hold `Arc<EncodeSession>`).
            let status = unsafe { destroy_fn(self.raw) };
            teardown::check(
                Library::Nvenc,
                "nvEncDestroyEncoder",
                i64::from(status),
                i64::from(sys::_NVENCSTATUS_NV_ENC_SUCCESS),
            );
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
    /// Row pitch reported by the last successful `nvEncLockInputBuffer` (`None` until locked).
    locked_pitch: Option<u32>,
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
        let _guard = session.lock_session();
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
            locked_pitch: None,
        })
    }

    /// Locks the buffer for CPU write access; unlocked automatically on `Drop`.
    pub fn lock(&mut self) -> Result<InputLock<'_>> {
        let lock_fn = entry!(self.session.list, nvEncLockInputBuffer);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_LOCK_INPUT_BUFFER`.
        let mut params: sys::NV_ENC_LOCK_INPUT_BUFFER = unsafe { zeroed() };
        params.version = VER_LOCK_INPUT_BUFFER;
        params.inputBuffer = self.raw;
        {
            let _guard = self.session.lock_session();
            // SAFETY: `self.raw` is a live input buffer owned by `self.session`.
            let status = unsafe { lock_fn(self.session.raw, &mut params) };
            check(
                "nvEncLockInputBuffer",
                status,
                Some(self.session.raw),
                self.session.list,
            )?;
        }
        self.locked_pitch = Some(params.pitch);
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
            let _guard = self.session.lock_session();
            // SAFETY: `self.raw` was created by `nvEncCreateInputBuffer` on `self.session`.
            let status = unsafe { destroy_fn(self.session.raw, self.raw) };
            teardown::check(
                Library::Nvenc,
                "nvEncDestroyInputBuffer",
                i64::from(status),
                i64::from(sys::_NVENCSTATUS_NV_ENC_SUCCESS),
            );
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
            let _guard = self.buf.session.lock_session();
            // SAFETY: `self.buf.raw` is currently locked by this guard.
            let status = unsafe { unlock_fn(self.buf.session.raw, self.buf.raw) };
            teardown::check(
                Library::Nvenc,
                "nvEncUnlockInputBuffer",
                i64::from(status),
                i64::from(sys::_NVENCSTATUS_NV_ENC_SUCCESS),
            );
        }
    }
}

/// Rejects a byte `pitch` shorter than one row of `format`: NVENC reads
/// `format.tight_pitch(width)` bytes per row, past the `pitch * rows` region the caller vouched for.
fn check_row_pitch(format: BufferFormat, width: u32, pitch: u32) -> Result<()> {
    let row_bytes = format
        .tight_pitch(width)
        .ok_or(Error::Invalid("input row size overflow"))?;
    if pitch < row_bytes {
        return Err(Error::Invalid(
            "pitch is smaller than one row of the buffer format",
        ));
    }
    Ok(())
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
    /// `pitch` is in bytes and must cover one row of `format` (`>= format.tight_pitch(width)`).
    ///
    /// # Safety
    /// `device_ptr` must remain valid and allocated on `session.cuda()` for the lifetime of this
    /// [`RegisteredResource`], with at least `pitch * format.locked_rows(height)` bytes
    /// (`pitch * height * 3 / 2` for 4:2:0 formats).
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
        check_row_pitch(format, width, pitch)?;
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
        let _guard = session.lock_session();
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
        let _guard = session.lock_session();
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
        {
            let _guard = self.session.lock_session();
            // SAFETY: `self.raw` is a live registered resource on `self.session`.
            let status = unsafe { map_fn(self.session.raw, &mut params) };
            check(
                "nvEncMapInputResource",
                status,
                Some(self.session.raw),
                self.session.list,
            )?;
        }
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
            let _guard = self.session.lock_session();
            // SAFETY: `self.raw` was registered on `self.session` and all `MappedResource` guards have dropped.
            let status = unsafe { unreg_fn(self.session.raw, self.raw) };
            teardown::check(
                Library::Nvenc,
                "nvEncUnregisterResource",
                i64::from(status),
                i64::from(sys::_NVENCSTATUS_NV_ENC_SUCCESS),
            );
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
            let _guard = self.res.session.lock_session();
            // SAFETY: `self.mapped` was returned by `nvEncMapInputResource`.
            let status = unsafe { unmap_fn(self.res.session.raw, self.mapped) };
            teardown::check(
                Library::Nvenc,
                "nvEncUnmapInputResource",
                i64::from(status),
                i64::from(sys::_NVENCSTATUS_NV_ENC_SUCCESS),
            );
        }
    }
}

/// Driver-allocated output bitstream buffer (`nvEncCreateBitstreamBuffer`).
pub struct OutputBitstream {
    raw: sys::NV_ENC_OUTPUT_PTR,
    session: Arc<EncodeSession>,
    /// Session `ready_epoch` at which the last submission returned `NeedMoreInput`.
    buffered_at: Option<u64>,
}

impl OutputBitstream {
    /// Allocates an output bitstream buffer on `session`.
    pub fn new(session: &Arc<EncodeSession>) -> Result<Self> {
        let create_fn = entry!(session.list, nvEncCreateBitstreamBuffer);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_CREATE_BITSTREAM_BUFFER`.
        let mut params: sys::NV_ENC_CREATE_BITSTREAM_BUFFER = unsafe { zeroed() };
        params.version = VER_CREATE_BITSTREAM_BUFFER;
        let _guard = session.lock_session();
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
            buffered_at: None,
        })
    }

    /// Locks the bitstream buffer, blocking until hardware encoding finishes, and returns an RAII guard.
    ///
    /// Fails with [`Error::Invalid`] while the buffer holds a frame reported as
    /// [`EncodeOutcome::NeedMoreInput`] that no later `Ready` submission or EOS has released
    /// (`nvEncodeAPI.h`: the client "cannot call ::NvEncLockBitstream()" on such a buffer).
    pub fn lock(&mut self) -> Result<BitstreamLock<'_>> {
        if let Some(at) = self.buffered_at
            && self.session.ready_epoch.load(Ordering::Relaxed) <= at
        {
            return Err(Error::Invalid(
                "output bitstream holds a frame buffered for B-frame reordering; submit more frames or flush_eos first",
            ));
        }
        let lock_fn = entry!(self.session.list, nvEncLockBitstream);
        let unlock_fn = entry!(self.session.list, nvEncUnlockBitstream);
        // SAFETY: all-zero bit pattern is valid for `NV_ENC_LOCK_BITSTREAM`.
        let mut params: sys::NV_ENC_LOCK_BITSTREAM = unsafe { zeroed() };
        params.version = VER_LOCK_BITSTREAM;
        params.outputBitstream = self.raw;
        let (size_bytes, ptr) = {
            let _guard = self.session.lock_session();
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
                // Unlock before returning error so the buffer doesn't stay locked. A failed unlock
                // is reported instead of the null pointer: the buffer is then still locked, which
                // is what the caller has to know first.
                // SAFETY: `self.raw` was just locked.
                let unlock_status = unsafe { unlock_fn(self.session.raw, self.raw) };
                check(
                    "nvEncUnlockBitstream",
                    unlock_status,
                    Some(self.session.raw),
                    self.session.list,
                )?;
                return Err(Error::Invalid(
                    "locked bitstream pointer is null for non-empty payload",
                ));
            }
            (size_bytes, ptr)
        };
        self.buffered_at = None;
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
            let _guard = self.session.lock_session();
            // SAFETY: `self.raw` was created by `nvEncCreateBitstreamBuffer` on `self.session`.
            let status = unsafe { destroy_fn(self.session.raw, self.raw) };
            teardown::check(
                Library::Nvenc,
                "nvEncDestroyBitstreamBuffer",
                i64::from(status),
                i64::from(sys::_NVENCSTATUS_NV_ENC_SUCCESS),
            );
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
            let _guard = self.buf.session.lock_session();
            // SAFETY: `self.buf.raw` is currently locked by this guard.
            let status = unsafe { unlock_fn(self.buf.session.raw, self.buf.raw) };
            teardown::check(
                Library::Nvenc,
                "nvEncUnlockBitstream",
                i64::from(status),
                i64::from(sys::_NVENCSTATUS_NV_ENC_SUCCESS),
            );
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

    /// Literals evaluated from the `*_VER` macros of Video Codec SDK 13.1 `nvEncodeAPI.h`
    /// (`NVENCAPI_STRUCT_VERSION(ver) = 13 | 1 << 24 | ver << 16 | 7 << 28`). A header bump fails
    /// the version assertion first: re-read every `*_VER` macro and update both sides.
    #[test]
    fn struct_versions_match_header_literals() {
        assert_eq!(
            (sys::NVENCAPI_MAJOR_VERSION, sys::NVENCAPI_MINOR_VERSION),
            (13, 1),
            "NVENC header changed: re-check every VER_* constant against nvEncodeAPI.h"
        );
        assert_eq!(VER_OPEN_SESSION_EX, 0x7101_000D); // NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER (1)
        assert_eq!(VER_CREATE_INPUT_BUFFER, 0x7102_000D); // NV_ENC_CREATE_INPUT_BUFFER_VER (2)
        assert_eq!(VER_CREATE_BITSTREAM_BUFFER, 0x7101_000D); // NV_ENC_CREATE_BITSTREAM_BUFFER_VER (1)
        assert_eq!(VER_REGISTER_RESOURCE, 0x7105_000D); // NV_ENC_REGISTER_RESOURCE_VER (5)
        assert_eq!(VER_MAP_INPUT_RESOURCE, 0x7104_000D); // NV_ENC_MAP_INPUT_RESOURCE_VER (4)
        assert_eq!(VER_LOCK_INPUT_BUFFER, 0x7101_000D); // NV_ENC_LOCK_INPUT_BUFFER_VER (1)
        assert_eq!(VER_CAPS_PARAM, 0x7101_000D); // NV_ENC_CAPS_PARAM_VER (1)
        assert_eq!(VER_CONFIG, 0xF109_000D); // NV_ENC_CONFIG_VER (9 | 1 << 31)
        assert_eq!(VER_INITIALIZE_PARAMS, 0xF107_000D); // NV_ENC_INITIALIZE_PARAMS_VER (7 | 1 << 31)
        assert_eq!(VER_PRESET_CONFIG, 0xF105_000D); // NV_ENC_PRESET_CONFIG_VER (5 | 1 << 31)
        assert_eq!(VER_PIC_PARAMS, 0xF107_000D); // NV_ENC_PIC_PARAMS_VER (7 | 1 << 31)
        assert_eq!(VER_LOCK_BITSTREAM, 0xF102_000D); // NV_ENC_LOCK_BITSTREAM_VER (2 | 1 << 31)
    }

    #[test]
    fn need_more_input_is_a_typed_outcome_not_an_error() {
        // SAFETY: all-zero is valid for the function list (every entry point `None`).
        let list: sys::NV_ENCODE_API_FUNCTION_LIST = unsafe { zeroed() };
        assert_eq!(
            encode_outcome(sys::_NVENCSTATUS_NV_ENC_ERR_NEED_MORE_INPUT, None, &list).ok(),
            Some(EncodeOutcome::NeedMoreInput)
        );
        assert_eq!(
            encode_outcome(sys::_NVENCSTATUS_NV_ENC_SUCCESS, None, &list).ok(),
            Some(EncodeOutcome::Ready)
        );
        match encode_outcome(sys::_NVENCSTATUS_NV_ENC_ERR_INVALID_PARAM, None, &list) {
            Err(Error::Status { code, .. }) => {
                assert_eq!(code.raw(), sys::_NVENCSTATUS_NV_ENC_ERR_INVALID_PARAM);
            }
            other => panic!("expected Error::Status, got {other:?}"),
        }
    }

    #[test]
    fn device_pitch_shorter_than_a_row_is_rejected() {
        assert!(matches!(
            check_row_pitch(BufferFormat::Argb, 1920, 256),
            Err(Error::Invalid(_))
        ));
        assert!(matches!(
            check_row_pitch(BufferFormat::Yuv420_10Bit, 1920, 1920),
            Err(Error::Invalid(_))
        ));
        assert!(check_row_pitch(BufferFormat::Argb, 1920, 1920 * 4).is_ok());
        assert!(check_row_pitch(BufferFormat::Nv12, 1920, 2048).is_ok());
        assert!(matches!(
            check_row_pitch(BufferFormat::Argb, u32::MAX, u32::MAX),
            Err(Error::Invalid(_))
        ));
    }

    #[test]
    fn codec_specific_setters_refuse_other_codecs() {
        let fresh = |codec| EncodeConfig {
            // SAFETY: all-zero is a valid `NV_ENC_CONFIG` (plain C data) for this host-only test.
            raw: unsafe { zeroed() },
            codec,
            misuse: None,
        };

        let mut hevc = fresh(Codec::Hevc);
        hevc.set_h264_bt709_vui();
        assert_eq!(hevc.raw.profileGUID.Data1, 0, "HEVC config must not be written");
        assert!(matches!(hevc.check_codec(Codec::Hevc), Err(Error::Invalid(_))));

        let mut hevc = fresh(Codec::Hevc);
        hevc.set_hevc_10bit_pq();
        assert_eq!(hevc.raw.profileGUID.Data1, guid::HEVC_MAIN10.Data1);
        assert!(hevc.check_codec(Codec::Hevc).is_ok());
        assert!(matches!(hevc.check_codec(Codec::H264), Err(Error::Invalid(_))));

        let mut h264 = fresh(Codec::H264);
        h264.set_hevc_10bit_pq();
        assert!(matches!(h264.check_codec(Codec::H264), Err(Error::Invalid(_))));
    }

    #[test]
    fn pic_flags_keep_header_bit_values() {
        assert_eq!(pic_flag(sys::_NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_FORCEINTRA), 0x1);
        assert_eq!(pic_flag(sys::_NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_FORCEIDR), 0x2);
        assert_eq!(pic_flag(sys::_NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_OUTPUT_SPSPPS), 0x4);
        assert_eq!(pic_flag(sys::_NV_ENC_PIC_FLAGS_NV_ENC_PIC_FLAG_EOS), 0x8);
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
        // An odd height keeps its last chroma row: 4-wide, 5 luma + 3 chroma rows.
        assert_eq!(BufferFormat::Nv12.tight_frame_bytes(4, 5), Some(4 * 8));
        assert_eq!(BufferFormat::Nv12.locked_rows(5), Some(8));
        assert_eq!(BufferFormat::Yuv420_10Bit.tight_frame_bytes(4, 5), Some(8 * 8));
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
