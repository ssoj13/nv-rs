//! Cross-platform NGX value types: [`ProjectId`], [`Guid`], [`DlssPerfQuality`], [`Feature`],
//! [`BufferFormat`], [`DlssFeatureFlags`], and DLSS Ray Reconstruction configuration enums
//! ([`DlssDenoiseMode`], [`DlssRoughnessMode`], [`DlssDepthType`], [`DlssRrPreset`]).
//!
//! **What.** Defines the pure-Rust configuration and enum types shared between Windows NGX FFI
//! calls and non-Windows stubs.
//! **Why.** Keeps `ProjectId` / GUID validation, DLSS quality presets, and feature/buffer-format
//! mappings testable on every platform without requiring a GPU or the NGX static library.
//! **How.** Stores `ProjectId` with pre-validated NUL-terminated `CString` buffers for zero-churn
//! FFI handoff and `engineType = NVSDK_NGX_ENGINE_TYPE_CUSTOM` (`0`) per PLAN D13.
//! **Used by** [`crate::params`] and [`crate::vulkan`].

use crate::error::{Error, Result};
use std::ffi::{CStr, CString};
use std::fmt;
use std::str::FromStr;

/// Canonical 128-bit GUID used to identify an NGX project (`8-4-4-4-12` hex format).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Guid([u8; 16]);

impl Guid {
    /// Constructs a [`Guid`] from 16 raw bytes in big-endian RFC 4122 order.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Returns the underlying 16 bytes.
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    /// Parses a hyphenated GUID string (`xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`).
    pub fn parse(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 36
            || bytes[8] != b'-'
            || bytes[13] != b'-'
            || bytes[18] != b'-'
            || bytes[23] != b'-'
        {
            return Err(Error::InvalidArgument(
                "GUID must be 36-char hyphenated hex (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)",
            ));
        }

        let mut out = [0u8; 16];
        let mut out_idx = 0usize;
        let mut i = 0usize;
        while i < 36 {
            if bytes[i] == b'-' {
                i += 1;
                continue;
            }
            let hi = hex_nibble(bytes[i]).ok_or(Error::InvalidArgument(
                "GUID contains non-hexadecimal character",
            ))?;
            let lo = hex_nibble(bytes[i + 1]).ok_or(Error::InvalidArgument(
                "GUID contains non-hexadecimal character",
            ))?;
            out[out_idx] = (hi << 4) | lo;
            out_idx += 1;
            i += 2;
        }
        Ok(Self(out))
    }
}

const fn hex_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

impl fmt::Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Guid({self})")
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = &self.0;
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0],
            b[1],
            b[2],
            b[3],
            b[4],
            b[5],
            b[6],
            b[7],
            b[8],
            b[9],
            b[10],
            b[11],
            b[12],
            b[13],
            b[14],
            b[15],
        )
    }
}

impl FromStr for Guid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::parse(s)
    }
}

/// NGX project identity passed to `NVSDK_NGX_VULKAN_Init_with_ProjectID` (PLAN D13).
///
/// Always uses `engineType = NVSDK_NGX_ENGINE_TYPE_CUSTOM` (`0`) so no NVIDIA-issued
/// application ID is required.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectId {
    guid: Guid,
    guid_cstr: CString,
    engine_name: Option<String>,
    engine_version: Option<CString>,
}

impl ProjectId {
    /// Creates a [`ProjectId`] from a [`Guid`] with `NVSDK_NGX_ENGINE_TYPE_CUSTOM`.
    pub fn new(guid: Guid) -> Self {
        let guid_str = guid.to_string();
        // SAFETY: `Guid::Display` emits only ASCII hex digits and hyphens (no interior NUL).
        let guid_cstr = CString::new(guid_str).expect("GUID formatting never contains NUL");
        Self {
            guid,
            guid_cstr,
            engine_name: None,
            engine_version: None,
        }
    }

    /// Parses a hyphenated GUID string into a [`ProjectId`].
    pub fn from_guid_str(guid: &str) -> Result<Self> {
        Ok(Self::new(Guid::parse(guid)?))
    }

    /// Attaches an optional engine name (for diagnostics/metadata).
    pub fn with_engine_name(mut self, name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        if name.as_bytes().contains(&0) {
            return Err(Error::InvalidArgument("engine_name contains interior NUL"));
        }
        self.engine_name = Some(name);
        Ok(self)
    }

    /// Attaches an optional engine version string passed to `NVSDK_NGX_*_Init_with_ProjectID`.
    pub fn with_engine_version(mut self, version: impl Into<String>) -> Result<Self> {
        let version = version.into();
        let cstr = CString::new(version)
            .map_err(|_| Error::InvalidArgument("engine_version contains interior NUL"))?;
        self.engine_version = Some(cstr);
        Ok(self)
    }

    /// Returns the project [`Guid`].
    pub const fn guid(&self) -> Guid {
        self.guid
    }

    /// Returns the NUL-terminated canonical GUID string for NGX FFI.
    pub fn guid_cstr(&self) -> &CStr {
        &self.guid_cstr
    }

    /// Returns the optional engine name.
    pub fn engine_name(&self) -> Option<&str> {
        self.engine_name.as_deref()
    }

    /// Returns the optional engine version string.
    pub fn engine_version(&self) -> Option<&str> {
        self.engine_version.as_ref().and_then(|c| c.to_str().ok())
    }

    /// Returns the NUL-terminated engine version string if set.
    pub fn engine_version_cstr(&self) -> Option<&CStr> {
        self.engine_version.as_deref()
    }

    /// Returns the NGX engine type (`NVSDK_NGX_ENGINE_TYPE_CUSTOM = 0`).
    pub const fn engine_type(&self) -> i32 {
        0
    }
}

/// DLSS performance/quality preset (`NVSDK_NGX_PerfQuality_Value`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DlssPerfQuality {
    /// Maximum performance (`NVSDK_NGX_PerfQuality_Value_MaxPerf = 0`).
    MaxPerf = 0,
    /// Balanced (`NVSDK_NGX_PerfQuality_Value_Balanced = 1`).
    Balanced = 1,
    /// Maximum quality (`NVSDK_NGX_PerfQuality_Value_MaxQuality = 2`).
    MaxQuality = 2,
    /// Ultra performance (`NVSDK_NGX_PerfQuality_Value_UltraPerformance = 3`).
    UltraPerformance = 3,
    /// Ultra quality (`NVSDK_NGX_PerfQuality_Value_UltraQuality = 4`).
    UltraQuality = 4,
    /// Native-resolution DLAA (`NVSDK_NGX_PerfQuality_Value_DLAA = 5`).
    Dlaa = 5,
}

/// Convenience alias for [`DlssPerfQuality`].
pub type DlssQuality = DlssPerfQuality;

impl DlssPerfQuality {
    /// Converts to the raw `NVSDK_NGX_PerfQuality_Value` integer.
    pub const fn to_raw(self) -> i32 {
        self as i32
    }

    /// Converts from a raw `NVSDK_NGX_PerfQuality_Value` integer.
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::MaxPerf),
            1 => Some(Self::Balanced),
            2 => Some(Self::MaxQuality),
            3 => Some(Self::UltraPerformance),
            4 => Some(Self::UltraQuality),
            5 => Some(Self::Dlaa),
            _ => None,
        }
    }
}

/// NGX SDK feature identifier (`NVSDK_NGX_Feature`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Feature {
    /// DLSS Super Resolution (`NVSDK_NGX_Feature_SuperSampling = 1`).
    SuperSampling = 1,
    /// In-painting (`NVSDK_NGX_Feature_InPainting = 2`).
    InPainting = 2,
    /// Image Super Resolution (`NVSDK_NGX_Feature_ImageSuperResolution = 3`).
    ImageSuperResolution = 3,
    /// Optical Flow / Slow Motion (`NVSDK_NGX_Feature_SlowMotion = 4`).
    SlowMotion = 4,
    /// Video Super Resolution (`NVSDK_NGX_Feature_VideoSuperResolution = 5`).
    VideoSuperResolution = 5,
    /// Image Signal Processing (`NVSDK_NGX_Feature_ImageSignalProcessing = 9`).
    ImageSignalProcessing = 9,
    /// Deep Resolve (`NVSDK_NGX_Feature_DeepResolve = 10`).
    DeepResolve = 10,
    /// DLSS Frame Generation (`NVSDK_NGX_Feature_FrameGeneration = 11`).
    FrameGeneration = 11,
    /// Deep DVC (`NVSDK_NGX_Feature_DeepDVC = 12`).
    DeepDvc = 12,
    /// DLSS Ray Reconstruction (`NVSDK_NGX_Feature_RayReconstruction = 13`).
    RayReconstruction = 13,
}

impl Feature {
    /// Converts to the raw `NVSDK_NGX_Feature` integer.
    pub const fn to_raw(self) -> i32 {
        self as i32
    }

    /// Converts from a raw `NVSDK_NGX_Feature` integer.
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            1 => Some(Self::SuperSampling),
            2 => Some(Self::InPainting),
            3 => Some(Self::ImageSuperResolution),
            4 => Some(Self::SlowMotion),
            5 => Some(Self::VideoSuperResolution),
            9 => Some(Self::ImageSignalProcessing),
            10 => Some(Self::DeepResolve),
            11 => Some(Self::FrameGeneration),
            12 => Some(Self::DeepDvc),
            13 => Some(Self::RayReconstruction),
            _ => None,
        }
    }
}

/// NGX buffer pixel format (`NVSDK_NGX_Buffer_Format`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BufferFormat {
    /// Unknown format (`0`).
    Unknown = 0,
    /// 8-bit unsigned RGB (`1`).
    Rgb8Ui = 1,
    /// 16-bit float RGB (`2`).
    Rgb16F = 2,
    /// 32-bit float RGB (`3`).
    Rgb32F = 3,
    /// 8-bit unsigned RGBA (`4`).
    Rgba8Ui = 4,
    /// 16-bit float RGBA (`5`).
    Rgba16F = 5,
    /// 32-bit float RGBA (`6`).
    Rgba32F = 6,
}

impl BufferFormat {
    /// Converts to the raw `NVSDK_NGX_Buffer_Format` integer.
    pub const fn to_raw(self) -> i32 {
        self as i32
    }

    /// Converts from a raw `NVSDK_NGX_Buffer_Format` integer.
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Unknown),
            1 => Some(Self::Rgb8Ui),
            2 => Some(Self::Rgb16F),
            3 => Some(Self::Rgb32F),
            4 => Some(Self::Rgba8Ui),
            5 => Some(Self::Rgba16F),
            6 => Some(Self::Rgba32F),
            _ => None,
        }
    }
}

/// Bitflags for DLSS feature creation (`NVSDK_NGX_DLSS_Feature_Flags`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DlssFeatureFlags(pub i32);

impl DlssFeatureFlags {
    /// No flags (`0`).
    pub const NONE: Self = Self(0);
    /// Input color is HDR (`1 << 0`).
    pub const IS_HDR: Self = Self(1);
    /// Motion vectors are at low (render) resolution (`1 << 1`).
    pub const MV_LOW_RES: Self = Self(2);
    /// Motion vectors include jitter (`1 << 2`).
    pub const MV_JITTERED: Self = Self(4);
    /// Reverse-Z depth buffer (`1 << 3`).
    pub const DEPTH_INVERTED: Self = Self(8);
    /// Enable sharpening pass (`1 << 5`).
    pub const DO_SHARPENING: Self = Self(32);
    /// Enable auto-exposure (`1 << 6`).
    pub const AUTO_EXPOSURE: Self = Self(64);
    /// Enable alpha upscaling (`1 << 7`).
    pub const ALPHA_UPSCALING: Self = Self(128);

    /// Combines two flag sets.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns `true` if `other` bits are all set.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for DlssFeatureFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl std::ops::BitOrAssign for DlssFeatureFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

/// Vulkan instance and device extension names required by NGX.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RequiredExtensions {
    /// Required Vulkan instance extensions.
    pub instance_extensions: Vec<String>,
    /// Required Vulkan device extensions.
    pub device_extensions: Vec<String>,
}

/// DLSS Ray Reconstruction denoise mode (`NVSDK_NGX_DLSS_Denoise_Mode`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DlssDenoiseMode {
    /// Denoising disabled (`NVSDK_NGX_DLSS_Denoise_Mode_Off = 0`).
    Off = 0,
    /// Deep-learning unified denoiser and upscaler (`NVSDK_NGX_DLSS_Denoise_Mode_DLUnified = 1`).
    #[default]
    DlUnified = 1,
}

impl DlssDenoiseMode {
    /// Converts to the raw `NVSDK_NGX_DLSS_Denoise_Mode` integer.
    pub const fn to_raw(self) -> i32 {
        self as i32
    }

    /// Converts from a raw `NVSDK_NGX_DLSS_Denoise_Mode` integer.
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Off),
            1 => Some(Self::DlUnified),
            _ => None,
        }
    }
}

/// DLSS Ray Reconstruction roughness input layout (`NVSDK_NGX_DLSS_Roughness_Mode`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DlssRoughnessMode {
    /// Read roughness from its own `GBuffer.Roughness` texture (`0`).
    #[default]
    Unpacked = 0,
    /// Read roughness packed in the `.w` channel of `GBuffer.Normals` (`1`).
    Packed = 1,
}

impl DlssRoughnessMode {
    /// Converts to the raw `NVSDK_NGX_DLSS_Roughness_Mode` unsigned integer.
    pub const fn to_raw(self) -> u32 {
        self as u32
    }

    /// Converts from a raw `NVSDK_NGX_DLSS_Roughness_Mode` unsigned integer.
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Unpacked),
            1 => Some(Self::Packed),
            _ => None,
        }
    }
}

/// DLSS Ray Reconstruction depth buffer encoding (`NVSDK_NGX_DLSS_Depth_Type`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DlssDepthType {
    /// Linear view-space depth (`NVSDK_NGX_DLSS_Depth_Type_Linear = 0`).
    #[default]
    Linear = 0,
    /// Hardware projected depth (`NVSDK_NGX_DLSS_Depth_Type_HW = 1`).
    Hw = 1,
}

impl DlssDepthType {
    /// Converts to the raw `NVSDK_NGX_DLSS_Depth_Type` unsigned integer.
    pub const fn to_raw(self) -> u32 {
        self as u32
    }

    /// Converts from a raw `NVSDK_NGX_DLSS_Depth_Type` unsigned integer.
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Linear),
            1 => Some(Self::Hw),
            _ => None,
        }
    }
}

/// DLSS Ray Reconstruction render preset hint (`NVSDK_NGX_RayReconstruction_Hint_Render_Preset`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DlssRrPreset {
    /// Runtime default preset (`0`).
    #[default]
    Default = 0,
    /// Transformer model (`4`).
    D = 4,
    /// Latest transformer model (`5`; required when using the DoF guide).
    E = 5,
    /// Default RR2 model (`6`).
    F = 6,
}

impl DlssRrPreset {
    /// Converts to the raw `NVSDK_NGX_RayReconstruction_Hint_Render_Preset` unsigned integer.
    pub const fn to_raw(self) -> u32 {
        self as u32
    }

    /// Converts from a raw `NVSDK_NGX_RayReconstruction_Hint_Render_Preset` unsigned integer.
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Default),
            4 => Some(Self::D),
            5 => Some(Self::E),
            6 => Some(Self::F),
            _ => None,
        }
    }
}
