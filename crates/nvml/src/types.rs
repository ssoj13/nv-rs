//! Typed NVML domain structures, enums, and parsing helpers.
//!
//! **What.** Clean Rust representations for PCI bus IDs, memory info, utilization, clocks,
//! temperature thresholds, ECC counters, running processes, and NVLink queries.
//! **Why.** Avoids leaking raw C structs, sentinel values (`NVML_VALUE_NOT_AVAILABLE`), and
//! fixed-size C char buffers across the public API.
//! **How.** Stack-allocated buffer helpers convert C strings without intermediate allocations, and
//! [`PciBusId`] / [`validate_uuid`] validate caller inputs before hitting FFI.
//! **Used by** [`crate::Nvml`] and [`crate::Device`].

use std::ffi::c_char;
use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::sys;

/// Computes `NVML_STRUCT_VERSION(data, ver)` (`sizeof(T) | (ver << 24)`).
#[inline]
pub(crate) const fn struct_version<T>(ver: u32) -> u32 {
    (std::mem::size_of::<T>() as u32) | (ver << 24)
}

/// Decodes a NUL-terminated (or full-buffer) C `char` array into an owned `String`.
pub(crate) fn decode_c_buf(buf: &[c_char]) -> String {
    let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    // SAFETY: `c_char` and `u8` have identical size and alignment; slicing `..len` stays within `buf`.
    let bytes = unsafe { std::slice::from_raw_parts(buf.as_ptr().cast::<u8>(), len) };
    String::from_utf8_lossy(bytes).into_owned()
}

/// Validates a non-empty, interior-NUL-free string and writes it into a NUL-terminated stack
/// buffer `N` bytes wide.
pub(crate) fn encode_c_buf<const N: usize>(s: &str, label: &'static str) -> Result<[c_char; N]> {
    let bytes = s.as_bytes();
    if bytes.is_empty() || bytes.contains(&0) || bytes.len() >= N {
        return Err(Error::InvalidInput(label));
    }
    let mut buf = [0 as c_char; N];
    for (dst, &src) in buf.iter_mut().zip(bytes.iter()) {
        *dst = src as c_char;
    }
    Ok(buf)
}

/// Validates a GPU or MIG UUID string before passing it to `nvmlDeviceGetHandleByUUID`.
pub(crate) fn validate_uuid(uuid: &str) -> Result<()> {
    if uuid.is_empty()
        || uuid.as_bytes().contains(&0)
        || uuid.len() >= sys::NVML_DEVICE_UUID_V2_BUFFER_SIZE as usize
    {
        return Err(Error::InvalidInput("invalid UUID length or interior NUL"));
    }
    if !(uuid.starts_with("GPU-") || uuid.starts_with("MIG-")) {
        return Err(Error::InvalidInput("UUID must start with GPU- or MIG-"));
    }
    Ok(())
}

/// Parsed PCI bus identifier (`domain:bus:device.function`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PciBusId {
    /// PCI domain (`0..=0xffff_ffff`).
    pub domain: u32,
    /// PCI bus (`0..=0xff`).
    pub bus: u8,
    /// PCI device/slot (`0..=0x1f`).
    pub device: u8,
    /// PCI function (`0..=0x7`).
    pub function: u8,
}

impl fmt::Display for PciBusId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:08x}:{:02x}:{:02x}.{:x}",
            self.domain, self.bus, self.device, self.function
        )
    }
}

impl FromStr for PciBusId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (domain_s, rest) = s
            .split_once(':')
            .ok_or(Error::InvalidInput("PCI bus ID missing domain ':'"))?;
        let (bus_s, rest) = rest
            .split_once(':')
            .ok_or(Error::InvalidInput("PCI bus ID missing bus ':'"))?;
        let (dev_s, func_s) = rest
            .split_once('.')
            .ok_or(Error::InvalidInput("PCI bus ID missing function '.'"))?;
        if domain_s.is_empty()
            || domain_s.len() > 8
            || bus_s.is_empty()
            || bus_s.len() > 2
            || dev_s.is_empty()
            || dev_s.len() > 2
            || func_s.len() != 1
        {
            return Err(Error::InvalidInput("invalid PCI bus ID field width"));
        }
        let domain = u32::from_str_radix(domain_s, 16)
            .map_err(|_| Error::InvalidInput("invalid PCI domain hex"))?;
        let bus = u8::from_str_radix(bus_s, 16)
            .map_err(|_| Error::InvalidInput("invalid PCI bus hex"))?;
        let device = u8::from_str_radix(dev_s, 16)
            .map_err(|_| Error::InvalidInput("invalid PCI device hex"))?;
        let function = u8::from_str_radix(func_s, 16)
            .map_err(|_| Error::InvalidInput("invalid PCI function hex"))?;
        if device > 31 || function > 7 {
            return Err(Error::InvalidInput(
                "PCI device (>31) or function (>7) out of range",
            ));
        }
        Ok(Self {
            domain,
            bus,
            device,
            function,
        })
    }
}

/// PCI bus and device/vendor identity returned by `nvmlDeviceGetPciInfo_v3`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PciInfo {
    /// Canonical `domain:bus:device.function` string.
    pub bus_id: String,
    /// PCI domain.
    pub domain: u32,
    /// PCI bus.
    pub bus: u32,
    /// PCI device.
    pub device: u32,
    /// Combined 16-bit device ID (upper) and 16-bit vendor ID (lower).
    pub pci_device_id: u32,
    /// 32-bit subsystem device ID.
    pub pci_subsystem_id: u32,
}

impl PciInfo {
    /// 16-bit PCI vendor ID (e.g. `0x10de` for NVIDIA).
    #[inline]
    pub const fn vendor_id(&self) -> u16 {
        (self.pci_device_id & 0xffff) as u16
    }

    /// 16-bit PCI device ID.
    #[inline]
    pub const fn device_id(&self) -> u16 {
        (self.pci_device_id >> 16) as u16
    }
}

/// Physical device memory usage in bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryInfo {
    /// Total physical framebuffer in bytes.
    pub total: u64,
    /// Unallocated device memory in bytes.
    pub free: u64,
    /// Allocated device memory in bytes.
    pub used: u64,
    /// System/firmware-reserved memory in bytes (`None` on drivers that only support v1).
    pub reserved: Option<u64>,
}

/// BAR1 memory usage in bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bar1MemoryInfo {
    /// Total BAR1 aperture in bytes.
    pub total: u64,
    /// Unallocated BAR1 aperture in bytes.
    pub free: u64,
    /// Allocated BAR1 aperture in bytes.
    pub used: u64,
}

/// Instantaneous GPU and memory controller utilization percentages (`0..=100`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Utilization {
    /// Percent of time kernels were executing on the GPU.
    pub gpu: u32,
    /// Percent of time global device memory was being read or written.
    pub memory: u32,
}

/// Encoder or decoder utilization percentage and sampling period.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodecUtilization {
    /// Utilization percentage (`0..=100`).
    pub utilization: u32,
    /// Sampling period in microseconds.
    pub sampling_period_us: u32,
}

/// Clock domains queryable via `nvmlDeviceGetClockInfo` / `nvmlDeviceGetMaxClockInfo`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClockDomain {
    /// Graphics clock (`NVML_CLOCK_GRAPHICS`).
    Graphics,
    /// Streaming multiprocessor clock (`NVML_CLOCK_SM`).
    Sm,
    /// Memory clock (`NVML_CLOCK_MEM`).
    Memory,
    /// Video encoder/decoder clock (`NVML_CLOCK_VIDEO`).
    Video,
}

impl ClockDomain {
    pub(crate) const fn to_raw(self) -> sys::nvmlClockType_t {
        match self {
            Self::Graphics => sys::nvmlClockType_enum_NVML_CLOCK_GRAPHICS,
            Self::Sm => sys::nvmlClockType_enum_NVML_CLOCK_SM,
            Self::Memory => sys::nvmlClockType_enum_NVML_CLOCK_MEM,
            Self::Video => sys::nvmlClockType_enum_NVML_CLOCK_VIDEO,
        }
    }
}

/// Temperature thresholds queryable via `nvmlDeviceGetTemperatureThreshold`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TemperatureThreshold {
    /// Hardware shutdown temperature.
    Shutdown,
    /// Thermal slowdown temperature.
    Slowdown,
    /// Memory max operating temperature.
    MemMax,
    /// GPU max operating temperature.
    GpuMax,
    /// Acoustic target temperature.
    AcousticCurr,
}

impl TemperatureThreshold {
    pub(crate) const fn to_raw(self) -> sys::nvmlTemperatureThresholds_t {
        match self {
            Self::Shutdown => {
                sys::nvmlTemperatureThresholds_enum_NVML_TEMPERATURE_THRESHOLD_SHUTDOWN
            }
            Self::Slowdown => {
                sys::nvmlTemperatureThresholds_enum_NVML_TEMPERATURE_THRESHOLD_SLOWDOWN
            }
            Self::MemMax => sys::nvmlTemperatureThresholds_enum_NVML_TEMPERATURE_THRESHOLD_MEM_MAX,
            Self::GpuMax => sys::nvmlTemperatureThresholds_enum_NVML_TEMPERATURE_THRESHOLD_GPU_MAX,
            Self::AcousticCurr => {
                sys::nvmlTemperatureThresholds_enum_NVML_TEMPERATURE_THRESHOLD_ACOUSTIC_CURR
            }
        }
    }
}

/// Current and pending ECC mode flags.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EccMode {
    /// Currently active ECC state.
    pub current: bool,
    /// Pending ECC state after next reset.
    pub pending: bool,
}

/// Memory error classification for ECC queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemoryErrorType {
    /// Single-bit / corrected errors.
    Corrected,
    /// Double-bit / uncorrected errors.
    Uncorrected,
}

impl MemoryErrorType {
    pub(crate) const fn to_raw(self) -> sys::nvmlMemoryErrorType_t {
        match self {
            Self::Corrected => sys::nvmlMemoryErrorType_enum_NVML_MEMORY_ERROR_TYPE_CORRECTED,
            Self::Uncorrected => sys::nvmlMemoryErrorType_enum_NVML_MEMORY_ERROR_TYPE_UNCORRECTED,
        }
    }
}

/// ECC counter lifetime scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EccCounterType {
    /// Volatile counters (reset on driver reload).
    Volatile,
    /// Aggregate counters (persisted across reboots).
    Aggregate,
}

impl EccCounterType {
    pub(crate) const fn to_raw(self) -> sys::nvmlEccCounterType_t {
        match self {
            Self::Volatile => sys::nvmlEccCounterType_enum_NVML_VOLATILE_ECC,
            Self::Aggregate => sys::nvmlEccCounterType_enum_NVML_AGGREGATE_ECC,
        }
    }
}

/// Running compute or graphics process on a GPU.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProcessInfo {
    /// OS process identifier.
    pub pid: u32,
    /// Used GPU memory in bytes (`None` when NVML reports `NVML_VALUE_NOT_AVAILABLE`, e.g. WDDM).
    pub used_gpu_memory: Option<u64>,
    /// MIG GPU instance ID if active, otherwise `None`.
    pub gpu_instance_id: Option<u32>,
    /// MIG compute instance ID if active, otherwise `None`.
    pub compute_instance_id: Option<u32>,
}

impl ProcessInfo {
    pub(crate) fn from_raw(raw: &sys::nvmlProcessInfo_t) -> Self {
        const NOT_AVAIL: u64 = u64::MAX;
        const NO_MIG: u32 = u32::MAX;
        Self {
            pid: raw.pid,
            used_gpu_memory: (raw.usedGpuMemory != NOT_AVAIL).then_some(raw.usedGpuMemory),
            gpu_instance_id: (raw.gpuInstanceId != NO_MIG).then_some(raw.gpuInstanceId),
            compute_instance_id: (raw.computeInstanceId != NO_MIG).then_some(raw.computeInstanceId),
        }
    }
}

/// Reusable scratch buffer for zero-allocation process enumeration via
/// [`crate::Device::compute_running_processes_into`] and
/// [`crate::Device::graphics_running_processes_into`].
#[derive(Debug, Default)]
pub struct ProcessScratch {
    pub(crate) raw: Vec<sys::nvmlProcessInfo_t>,
}

impl ProcessScratch {
    /// Creates an empty [`ProcessScratch`] buffer.
    #[inline]
    pub const fn new() -> Self {
        Self { raw: Vec::new() }
    }
}

/// PCIe throughput counter direction (reported in KB/s).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PcieCounter {
    /// Transmit bytes (1 KB granularity).
    TxBytes,
    /// Receive bytes (1 KB granularity).
    RxBytes,
}

impl PcieCounter {
    pub(crate) const fn to_raw(self) -> sys::nvmlPcieUtilCounter_t {
        match self {
            Self::TxBytes => sys::nvmlPcieUtilCounter_enum_NVML_PCIE_UTIL_TX_BYTES,
            Self::RxBytes => sys::nvmlPcieUtilCounter_enum_NVML_PCIE_UTIL_RX_BYTES,
        }
    }
}

/// NVLink capability query selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NvLinkCapability {
    /// P2P over NVLink supported.
    P2pSupported,
    /// System memory access over NVLink supported.
    SysmemAccess,
    /// P2P atomics supported.
    P2pAtomics,
    /// System memory atomics supported.
    SysmemAtomics,
    /// SLI bridge over NVLink supported.
    SliBridge,
    /// Link is valid/supported on this device.
    Valid,
}

impl NvLinkCapability {
    pub(crate) const fn to_raw(self) -> sys::nvmlNvLinkCapability_t {
        match self {
            Self::P2pSupported => sys::nvmlNvLinkCapability_enum_NVML_NVLINK_CAP_P2P_SUPPORTED,
            Self::SysmemAccess => sys::nvmlNvLinkCapability_enum_NVML_NVLINK_CAP_SYSMEM_ACCESS,
            Self::P2pAtomics => sys::nvmlNvLinkCapability_enum_NVML_NVLINK_CAP_P2P_ATOMICS,
            Self::SysmemAtomics => sys::nvmlNvLinkCapability_enum_NVML_NVLINK_CAP_SYSMEM_ATOMICS,
            Self::SliBridge => sys::nvmlNvLinkCapability_enum_NVML_NVLINK_CAP_SLI_BRIDGE,
            Self::Valid => sys::nvmlNvLinkCapability_enum_NVML_NVLINK_CAP_VALID,
        }
    }
}

/// NVLink error counter selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NvLinkErrorCounter {
    /// Data link transmit replay errors.
    DlReplay,
    /// Data link transmit recovery errors.
    DlRecovery,
    /// Data link receive flit CRC errors.
    DlCrcFlit,
    /// Data link receive data CRC errors.
    DlCrcData,
    /// Data link receive data ECC errors.
    DlEccData,
}

impl NvLinkErrorCounter {
    pub(crate) const fn to_raw(self) -> sys::nvmlNvLinkErrorCounter_t {
        match self {
            Self::DlReplay => sys::nvmlNvLinkErrorCounter_enum_NVML_NVLINK_ERROR_DL_REPLAY,
            Self::DlRecovery => sys::nvmlNvLinkErrorCounter_enum_NVML_NVLINK_ERROR_DL_RECOVERY,
            Self::DlCrcFlit => sys::nvmlNvLinkErrorCounter_enum_NVML_NVLINK_ERROR_DL_CRC_FLIT,
            Self::DlCrcData => sys::nvmlNvLinkErrorCounter_enum_NVML_NVLINK_ERROR_DL_CRC_DATA,
            Self::DlEccData => sys::nvmlNvLinkErrorCounter_enum_NVML_NVLINK_ERROR_DL_ECC_DATA,
        }
    }
}

/// GPU hardware architecture family (`NVML_DEVICE_ARCH_*`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Architecture(pub sys::nvmlDeviceArchitecture_t);

impl Architecture {
    /// Returns a human-readable name for known NVIDIA architectures.
    pub const fn name(self) -> Option<&'static str> {
        match self.0 {
            sys::NVML_DEVICE_ARCH_KEPLER => Some("Kepler"),
            sys::NVML_DEVICE_ARCH_MAXWELL => Some("Maxwell"),
            sys::NVML_DEVICE_ARCH_PASCAL => Some("Pascal"),
            sys::NVML_DEVICE_ARCH_VOLTA => Some("Volta"),
            sys::NVML_DEVICE_ARCH_TURING => Some("Turing"),
            sys::NVML_DEVICE_ARCH_AMPERE => Some("Ampere"),
            sys::NVML_DEVICE_ARCH_ADA => Some("Ada"),
            sys::NVML_DEVICE_ARCH_HOPPER => Some("Hopper"),
            sys::NVML_DEVICE_ARCH_BLACKWELL => Some("Blackwell"),
            sys::NVML_DEVICE_ARCH_RUBIN => Some("Rubin"),
            _ => None,
        }
    }
}
