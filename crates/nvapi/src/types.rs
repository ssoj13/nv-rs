//! Typed NVAPI domain structures, version helpers, and string codecs.
//!
//! **What.** Cross-platform data types for driver version, GPU identity, memory, thermal sensors,
//! P-states, clocks, performance decrease reasons, HDR/VRR/G-Sync capabilities, and DRS profiles.
//! **Why.** Isolates callers from raw C bitfields, `MAKE_NVAPI_VERSION` macros, and fixed-size
//! ASCII / UTF-16 buffers, while remaining testable on non-Windows hosts.
//! **How.** [`make_version`] implements `sizeof(T) | (ver << 16)`; stack-buffer codecs handle
//! `NvAPI_ShortString` and `NvAPI_UnicodeString` without intermediate allocations.
//! **Used by** [`crate::NvApi`] and its Windows implementation.

use std::ffi::c_char;

use crate::error::{Error, Result};

/// Implements `MAKE_NVAPI_VERSION(typeName, ver)` (`sizeof(T) | (ver << 16)`).
#[inline]
pub const fn make_version<T>(ver: u32) -> u32 {
    (std::mem::size_of::<T>() as u32) | (ver << 16)
}

/// Decodes a NUL-terminated (or full-buffer) `NvAPI_ShortString` (`[c_char; 64]`).
pub fn decode_short_string(buf: &[c_char]) -> String {
    let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    // SAFETY: `c_char` and `u8` have identical size and alignment; slicing `..len` stays within `buf`.
    let bytes = unsafe { std::slice::from_raw_parts(buf.as_ptr().cast::<u8>(), len) };
    String::from_utf8_lossy(bytes).into_owned()
}

/// Decodes a NUL-terminated UTF-16 `NvAPI_UnicodeString` buffer into an owned `String`.
pub fn decode_utf16_z(buf: &[u16]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len])
}

/// Encodes `s` as NUL-terminated UTF-16 into `out`, rejecting interior NULs or oversized strings.
pub fn encode_utf16_z(s: &str, out: &mut [u16]) -> Result<()> {
    if s.contains('\0') {
        return Err(Error::InvalidInput("string contains interior NUL"));
    }
    let mut written = 0usize;
    for unit in s.encode_utf16() {
        if written + 1 >= out.len() {
            return Err(Error::InvalidInput("UTF-16 string exceeds NVAPI buffer"));
        }
        out[written] = unit;
        written += 1;
    }
    if out.is_empty() {
        return Err(Error::InvalidInput("empty output buffer"));
    }
    out[written] = 0;
    Ok(())
}

/// NVIDIA display driver version and branch string (`NvAPI_SYS_GetDriverAndBranchVersion`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DriverVersion {
    /// Encoded driver version (e.g. `56094` for `560.94`).
    pub version: u32,
    /// Driver build branch string (e.g. `"r560_00"`).
    pub branch: String,
}

impl DriverVersion {
    /// Splits [`Self::version`] into `(major, minor)` (e.g. `56094` -> `(560, 94)`).
    #[inline]
    pub const fn major_minor(&self) -> (u32, u32) {
        (self.version / 100, self.version % 100)
    }
}

/// PCI hardware identifiers for a physical GPU (`NvAPI_GPU_GetPCIIdentifiers`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PciIdentifiers {
    /// Combined PCI device/vendor ID.
    pub device_id: u32,
    /// PCI subsystem ID.
    pub subsystem_id: u32,
    /// PCI revision ID.
    pub revision_id: u32,
    /// External device ID.
    pub ext_device_id: u32,
}

/// GPU architecture, implementation, and chip revision (`NvAPI_GPU_GetArchInfo`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArchInfo {
    /// Architecture ID (`NV_GPU_ARCHITECTURE_ID`).
    pub architecture: u32,
    /// Implementation ID (`NV_GPU_ARCH_IMPLEMENTATION_ID`).
    pub implementation: u32,
    /// Chip revision (`NV_GPU_CHIP_REVISION`).
    pub revision: u32,
}

/// Driver memory footprint in bytes (`NvAPI_GPU_GetMemoryInfoEx`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpuMemoryInfo {
    /// Total dedicated framebuffer in bytes.
    pub dedicated_bytes: u64,
    /// Available dedicated framebuffer for allocations in bytes.
    pub available_dedicated_bytes: u64,
    /// Currently available dedicated framebuffer in bytes.
    pub cur_available_dedicated_bytes: u64,
    /// System video memory allocated at load time (iGPUs) in bytes.
    pub system_bytes: u64,
    /// Shared system memory limit in bytes.
    pub shared_system_bytes: u64,
}

/// Single thermal sensor reading from `NvAPI_GPU_GetThermalSettings`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThermalSensor {
    /// Raw `NV_THERMAL_CONTROLLER` enum value.
    pub controller: i32,
    /// Raw `NV_THERMAL_TARGET` enum value (`1` = GPU core, `2` = memory, etc.).
    pub target: i32,
    /// Current temperature in degrees Celsius.
    pub current_temp_c: i32,
    /// Default minimum temperature in degrees Celsius.
    pub default_min_c: i32,
    /// Default maximum temperature in degrees Celsius.
    pub default_max_c: i32,
}

/// Dynamic P-state utilization percentages (`0..=100`, `None` if domain absent).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DynamicPstates {
    /// Whether dynamic P-state management is enabled.
    pub enabled: bool,
    /// Graphics engine utilization percentage.
    pub gpu: Option<u32>,
    /// Framebuffer (memory controller) utilization percentage.
    pub fb: Option<u32>,
    /// Video engine utilization percentage.
    pub vid: Option<u32>,
    /// Bus interface utilization percentage.
    pub bus: Option<u32>,
}

/// Clock type selector for `NvAPI_GPU_GetAllClockFrequencies`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClockKind {
    /// Current clock frequencies.
    Current,
    /// Base clock frequencies.
    Base,
    /// Boost clock frequencies.
    Boost,
}

/// Clock frequencies in kHz (`None` when the domain is not present).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClockFrequencies {
    /// Graphics clock in kHz.
    pub graphics_khz: Option<u32>,
    /// Memory clock in kHz.
    pub memory_khz: Option<u32>,
    /// Processor/shader clock in kHz.
    pub processor_khz: Option<u32>,
    /// Video clock in kHz.
    pub video_khz: Option<u32>,
}

/// Bitmask of active performance-decrease / throttling reasons (`NvAPI_GPU_GetPerfDecreaseInfo`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PerfDecreaseReasons(pub u32);

impl PerfDecreaseReasons {
    /// Thermal slowdown / shutdown / POR thermal protection (`0x01`).
    pub const THERMAL_PROTECTION: u32 = 0x0000_0001;
    /// Power capping / P-state cap (`0x02`).
    pub const POWER_CONTROL: u32 = 0x0000_0002;
    /// AC -> battery transition (`0x04`).
    pub const AC_BATT: u32 = 0x0000_0004;
    /// API-triggered slowdown (`0x08`).
    pub const API_TRIGGERED: u32 = 0x0000_0008;
    /// External power connector missing / insufficient power (`0x10`).
    pub const INSUFFICIENT_POWER: u32 = 0x0000_0010;
    /// Unknown slowdown reason (`0x8000_0000`).
    pub const UNKNOWN: u32 = 0x8000_0000;

    /// Returns `true` when no slowdown is active.
    #[inline]
    pub const fn is_none(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` when thermal protection throttling is active.
    #[inline]
    pub const fn thermal_protection(self) -> bool {
        (self.0 & Self::THERMAL_PROTECTION) != 0
    }

    /// Returns `true` when power-control capping is active.
    #[inline]
    pub const fn power_control(self) -> bool {
        (self.0 & Self::POWER_CONTROL) != 0
    }

    /// Returns `true` when insufficient external power is reported.
    #[inline]
    pub const fn insufficient_power(self) -> bool {
        (self.0 & Self::INSUFFICIENT_POWER) != 0
    }
}

/// Display HDR capabilities (`NvAPI_Disp_GetHdrCapabilities`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HdrCapabilities {
    /// HDMI 2.0a UHDA HDR with ST2084 EOTF.
    pub st2084_eotf: bool,
    /// Traditional HDR gamma.
    pub traditional_hdr_gamma: bool,
    /// Extended Dynamic Range on SDR displays.
    pub edr: bool,
    /// Dolby Vision supported.
    pub dolby_vision: bool,
    /// HDR10+ sink-side tonemapping supported.
    pub hdr10_plus: bool,
    /// HDR10+ Gaming supported.
    pub hdr10_plus_gaming: bool,
    /// NVIDIA-certified HDR display.
    pub nvidia_certified: bool,
    /// Desired max content luminance in cd/m^2 (nits).
    pub max_luminance_nits: u16,
    /// Desired min content luminance in 0.0001 cd/m^2 units.
    pub min_luminance_ten_thousandths: u16,
    /// Desired max frame-average luminance (MaxFALL) in cd/m^2.
    pub max_fall_nits: u16,
}

/// Variable Refresh Rate (VRR / G-Sync Compatible) status (`NvAPI_Disp_GetVRRInfo`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VrrInfo {
    /// VRR mode is enabled.
    pub enabled: bool,
    /// VRR mode is possible on this display.
    pub possible: bool,
    /// VRR mode is requested.
    pub requested: bool,
    /// On-screen VRR indicator is enabled.
    pub indicator_enabled: bool,
    /// Display is currently in VRR mode.
    pub in_vrr_mode: bool,
}

/// Quadro Sync / G-Sync board capabilities (`NvAPI_GSync_QueryCapabilities`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GSyncCapabilities {
    /// Sync board ID.
    pub board_id: u32,
    /// FPGA major revision.
    pub revision: u32,
    /// Capability flags.
    pub cap_flags: u32,
}

/// Quadro Sync / G-Sync GPU sync status (`NvAPI_GSync_GetSyncStatus`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GSyncStatus {
    /// Timing is in sync.
    pub is_synced: bool,
    /// Stereo timing phase matches master sync signal.
    pub is_stereo_synced: bool,
    /// Sync signal is available.
    pub is_sync_signal_available: bool,
}

/// Driver Settings (DRS) profile metadata (`NvAPI_DRS_GetProfileInfo`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrsProfileInfo {
    /// Profile name.
    pub name: String,
    /// Whether this is a predefined NVIDIA profile (`false` = user-defined).
    pub is_predefined: bool,
    /// Number of applications bound to this profile.
    pub num_apps: u32,
    /// Number of settings configured on this profile.
    pub num_settings: u32,
}
