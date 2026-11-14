//! Safe NVML device handle and GPU telemetry queries.
//!
//! **What.** [`Device`] wraps an `nvmlDevice_t` alongside an `Arc<NvmlSession>` so the NVML
//! library remains initialized for as long as any device handle exists.
//! **Why.** Prevents use-after-shutdown bugs while exposing zero-churn telemetry queries (memory,
//! utilization, thermal, fan, power, clocks, ECC, processes, PCIe, NVLink).
//! **How.** Scalar and string queries use stack buffers; process enumeration uses two-phase sizing
//! with a bounded retry loop and a caller-reusable `_into` variant for zero-allocation polling.
//! **Used by** [`crate::Nvml`] and downstream telemetry consumers (`gpu-info-rs`).

use std::ffi::c_char;
use std::mem::MaybeUninit;
use std::sync::Arc;

use crate::error::{Result, Status, check};
use crate::session::NvmlSession;
use crate::sys;
use crate::types::{
    Architecture, Bar1MemoryInfo, ClockDomain, CodecUtilization, EccCounterType, EccMode,
    MemoryErrorType, MemoryInfo, NvLinkCapability, NvLinkErrorCounter, PciInfo, PcieCounter,
    ProcessInfo, ProcessScratch, TemperatureThreshold, Utilization, decode_c_buf, struct_version,
};

/// Handle to a single NVIDIA GPU managed by NVML.
///
/// Holds an `Arc` reference to the owning [`NvmlSession`], ensuring `nvmlShutdown` is not called
/// while any `Device` is alive.
#[derive(Clone, Debug)]
pub struct Device {
    pub(crate) _session: Arc<NvmlSession>,
    pub(crate) raw: sys::nvmlDevice_t,
}

// SAFETY: NVML device handles are opaque driver identifiers safe to send and share across threads
// while NVML is initialized (kept alive by `_session`).
unsafe impl Send for Device {}
// SAFETY: NVML serializes internal driver state; concurrent read-only telemetry queries on a
// `Device` across threads are safe.
unsafe impl Sync for Device {}

impl Device {
    /// Returns the raw `nvmlDevice_t` handle.
    #[inline]
    pub const fn raw(&self) -> sys::nvmlDevice_t {
        self.raw
    }

    /// Queries the NVML device index (`nvmlDeviceGetIndex`).
    pub fn index(&self) -> Result<u32> {
        let mut idx = 0u32;
        // SAFETY: `self.raw` is a valid NVML device handle and `idx` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetIndex(self.raw, &mut idx) };
        check("nvmlDeviceGetIndex", code)?;
        Ok(idx)
    }

    /// Queries the product name (`nvmlDeviceGetName`).
    pub fn name(&self) -> Result<String> {
        let mut buf = [0 as c_char; sys::NVML_DEVICE_NAME_V2_BUFFER_SIZE as usize];
        // SAFETY: `self.raw` is valid and `buf` is a writable buffer of `buf.len()` bytes.
        let code = unsafe { sys::nvmlDeviceGetName(self.raw, buf.as_mut_ptr(), buf.len() as u32) };
        check("nvmlDeviceGetName", code)?;
        Ok(decode_c_buf(&buf))
    }

    /// Queries the globally unique immutable GPU UUID (`nvmlDeviceGetUUID`).
    pub fn uuid(&self) -> Result<String> {
        let mut buf = [0 as c_char; sys::NVML_DEVICE_UUID_V2_BUFFER_SIZE as usize];
        // SAFETY: `self.raw` is valid and `buf` is a writable buffer of `buf.len()` bytes.
        let code = unsafe { sys::nvmlDeviceGetUUID(self.raw, buf.as_mut_ptr(), buf.len() as u32) };
        check("nvmlDeviceGetUUID", code)?;
        Ok(decode_c_buf(&buf))
    }

    /// Queries the board serial number (`nvmlDeviceGetSerial`).
    pub fn serial(&self) -> Result<String> {
        let mut buf = [0 as c_char; sys::NVML_DEVICE_SERIAL_BUFFER_SIZE as usize];
        // SAFETY: `self.raw` is valid and `buf` is a writable buffer of `buf.len()` bytes.
        let code = unsafe { sys::nvmlDeviceGetSerial(self.raw, buf.as_mut_ptr(), buf.len() as u32) };
        check("nvmlDeviceGetSerial", code)?;
        Ok(decode_c_buf(&buf))
    }

    /// Queries the VBIOS version string (`nvmlDeviceGetVbiosVersion`).
    pub fn vbios_version(&self) -> Result<String> {
        let mut buf = [0 as c_char; sys::NVML_DEVICE_VBIOS_VERSION_BUFFER_SIZE as usize];
        // SAFETY: `self.raw` is valid and `buf` is a writable buffer of `buf.len()` bytes.
        let code = unsafe { sys::nvmlDeviceGetVbiosVersion(self.raw, buf.as_mut_ptr(), buf.len() as u32) };
        check("nvmlDeviceGetVbiosVersion", code)?;
        Ok(decode_c_buf(&buf))
    }

    /// Queries the GPU hardware architecture (`nvmlDeviceGetArchitecture`).
    pub fn architecture(&self) -> Result<Architecture> {
        let mut arch: sys::nvmlDeviceArchitecture_t = 0;
        // SAFETY: `self.raw` is valid and `arch` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetArchitecture(self.raw, &mut arch) };
        check("nvmlDeviceGetArchitecture", code)?;
        Ok(Architecture(arch))
    }

    /// Queries `(major, minor)` CUDA compute capability (`nvmlDeviceGetCudaComputeCapability`).
    pub fn cuda_compute_capability(&self) -> Result<(i32, i32)> {
        let mut major = 0i32;
        let mut minor = 0i32;
        // SAFETY: `self.raw` is valid and `major`/`minor` are valid out-pointers.
        let code = unsafe { sys::nvmlDeviceGetCudaComputeCapability(self.raw, &mut major, &mut minor) };
        check("nvmlDeviceGetCudaComputeCapability", code)?;
        Ok((major, minor))
    }

    /// Queries PCI bus and device/vendor identifiers (`nvmlDeviceGetPciInfo_v3`).
    pub fn pci_info(&self) -> Result<PciInfo> {
        let mut raw = MaybeUninit::<sys::nvmlPciInfo_t>::zeroed();
        // SAFETY: `self.raw` is valid and `raw` points to zero-initialized `nvmlPciInfo_t`.
        let code = unsafe { sys::nvmlDeviceGetPciInfo_v3(self.raw, raw.as_mut_ptr()) };
        check("nvmlDeviceGetPciInfo_v3", code)?;
        // SAFETY: `nvmlDeviceGetPciInfo_v3` succeeded and initialized `raw`.
        let raw = unsafe { raw.assume_init() };
        Ok(PciInfo {
            bus_id: decode_c_buf(&raw.busId),
            domain: raw.domain,
            bus: raw.bus,
            device: raw.device,
            pci_device_id: raw.pciDeviceId,
            pci_subsystem_id: raw.pciSubSystemId,
        })
    }

    /// Queries framebuffer memory usage (`nvmlDeviceGetMemoryInfo_v2`, falling back to
    /// `nvmlDeviceGetMemoryInfo` on older drivers that lack the v2 entry point).
    pub fn memory_info(&self) -> Result<MemoryInfo> {
        let mut v2 = sys::nvmlMemory_v2_t {
            version: struct_version::<sys::nvmlMemory_v2_t>(2),
            total: 0,
            reserved: 0,
            free: 0,
            used: 0,
        };
        // SAFETY: `self.raw` is valid and `v2.version` is initialized to `nvmlMemory_v2`.
        let code = unsafe { sys::nvmlDeviceGetMemoryInfo_v2(self.raw, &mut v2) };
        if Status(code).is_ok() {
            return Ok(MemoryInfo {
                total: v2.total,
                free: v2.free,
                used: v2.used,
                reserved: Some(v2.reserved),
            });
        }
        if !Status(code).is_function_not_found() {
            return Err(crate::Error::Status {
                call: "nvmlDeviceGetMemoryInfo_v2",
                status: Status(code),
            });
        }

        let mut v1 = MaybeUninit::<sys::nvmlMemory_t>::zeroed();
        // SAFETY: `self.raw` is valid and `v1` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetMemoryInfo(self.raw, v1.as_mut_ptr()) };
        check("nvmlDeviceGetMemoryInfo", code)?;
        // SAFETY: `nvmlDeviceGetMemoryInfo` succeeded and initialized `v1`.
        let v1 = unsafe { v1.assume_init() };
        Ok(MemoryInfo { total: v1.total, free: v1.free, used: v1.used, reserved: None })
    }

    /// Queries BAR1 memory usage (`nvmlDeviceGetBAR1MemoryInfo`).
    pub fn bar1_memory_info(&self) -> Result<Bar1MemoryInfo> {
        let mut raw = MaybeUninit::<sys::nvmlBAR1Memory_t>::zeroed();
        // SAFETY: `self.raw` is valid and `raw` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetBAR1MemoryInfo(self.raw, raw.as_mut_ptr()) };
        check("nvmlDeviceGetBAR1MemoryInfo", code)?;
        // SAFETY: `nvmlDeviceGetBAR1MemoryInfo` succeeded and initialized `raw`.
        let raw = unsafe { raw.assume_init() };
        Ok(Bar1MemoryInfo { total: raw.bar1Total, free: raw.bar1Free, used: raw.bar1Used })
    }

    /// Queries GPU and memory controller utilization (`nvmlDeviceGetUtilizationRates`).
    pub fn utilization_rates(&self) -> Result<Utilization> {
        let mut raw = MaybeUninit::<sys::nvmlUtilization_t>::zeroed();
        // SAFETY: `self.raw` is valid and `raw` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetUtilizationRates(self.raw, raw.as_mut_ptr()) };
        check("nvmlDeviceGetUtilizationRates", code)?;
        // SAFETY: `nvmlDeviceGetUtilizationRates` succeeded and initialized `raw`.
        let raw = unsafe { raw.assume_init() };
        Ok(Utilization { gpu: raw.gpu, memory: raw.memory })
    }

    /// Queries NVENC hardware encoder utilization (`nvmlDeviceGetEncoderUtilization`).
    pub fn encoder_utilization(&self) -> Result<CodecUtilization> {
        let mut utilization = 0u32;
        let mut sampling_period_us = 0u32;
        // SAFETY: `self.raw` is valid and both out-pointers are valid.
        let code = unsafe {
            sys::nvmlDeviceGetEncoderUtilization(self.raw, &mut utilization, &mut sampling_period_us)
        };
        check("nvmlDeviceGetEncoderUtilization", code)?;
        Ok(CodecUtilization { utilization, sampling_period_us })
    }

    /// Queries NVDEC hardware decoder utilization (`nvmlDeviceGetDecoderUtilization`).
    pub fn decoder_utilization(&self) -> Result<CodecUtilization> {
        let mut utilization = 0u32;
        let mut sampling_period_us = 0u32;
        // SAFETY: `self.raw` is valid and both out-pointers are valid.
        let code = unsafe {
            sys::nvmlDeviceGetDecoderUtilization(self.raw, &mut utilization, &mut sampling_period_us)
        };
        check("nvmlDeviceGetDecoderUtilization", code)?;
        Ok(CodecUtilization { utilization, sampling_period_us })
    }

    /// Queries the current GPU die temperature in degrees Celsius (`nvmlDeviceGetTemperature`).
    pub fn temperature_gpu(&self) -> Result<u32> {
        let mut temp = 0u32;
        // SAFETY: `self.raw` is valid and `temp` is a valid out-pointer.
        let code = unsafe {
            sys::nvmlDeviceGetTemperature(
                self.raw,
                sys::nvmlTemperatureSensors_enum_NVML_TEMPERATURE_GPU,
                &mut temp,
            )
        };
        check("nvmlDeviceGetTemperature", code)?;
        Ok(temp)
    }

    /// Queries a hardware temperature threshold in degrees Celsius (`nvmlDeviceGetTemperatureThreshold`).
    pub fn temperature_threshold(&self, kind: TemperatureThreshold) -> Result<u32> {
        let mut temp = 0u32;
        // SAFETY: `self.raw` is valid and `temp` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetTemperatureThreshold(self.raw, kind.to_raw(), &mut temp) };
        check("nvmlDeviceGetTemperatureThreshold", code)?;
        Ok(temp)
    }

    /// Queries the intended fan speed percentage (`0..=100`, `nvmlDeviceGetFanSpeed`).
    pub fn fan_speed(&self) -> Result<u32> {
        let mut speed = 0u32;
        // SAFETY: `self.raw` is valid and `speed` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetFanSpeed(self.raw, &mut speed) };
        check("nvmlDeviceGetFanSpeed", code)?;
        Ok(speed)
    }

    /// Queries the speed percentage of a specific fan index (`nvmlDeviceGetFanSpeed_v2`).
    pub fn fan_speed_of(&self, fan: u32) -> Result<u32> {
        let mut speed = 0u32;
        // SAFETY: `self.raw` is valid and `speed` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetFanSpeed_v2(self.raw, fan, &mut speed) };
        check("nvmlDeviceGetFanSpeed_v2", code)?;
        Ok(speed)
    }

    /// Queries the number of controllable fans on this device (`nvmlDeviceGetNumFans`).
    pub fn fan_count(&self) -> Result<u32> {
        let mut count = 0u32;
        // SAFETY: `self.raw` is valid and `count` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetNumFans(self.raw, &mut count) };
        check("nvmlDeviceGetNumFans", code)?;
        Ok(count)
    }

    /// Queries instantaneous board power draw in milliwatts (`nvmlDeviceGetPowerUsage`).
    pub fn power_usage_mw(&self) -> Result<u32> {
        let mut mw = 0u32;
        // SAFETY: `self.raw` is valid and `mw` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetPowerUsage(self.raw, &mut mw) };
        check("nvmlDeviceGetPowerUsage", code)?;
        Ok(mw)
    }

    /// Queries the active power management limit in milliwatts (`nvmlDeviceGetPowerManagementLimit`).
    pub fn power_limit_mw(&self) -> Result<u32> {
        let mut mw = 0u32;
        // SAFETY: `self.raw` is valid and `mw` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetPowerManagementLimit(self.raw, &mut mw) };
        check("nvmlDeviceGetPowerManagementLimit", code)?;
        Ok(mw)
    }

    /// Queries the default power management limit in milliwatts (`nvmlDeviceGetPowerManagementDefaultLimit`).
    pub fn power_default_limit_mw(&self) -> Result<u32> {
        let mut mw = 0u32;
        // SAFETY: `self.raw` is valid and `mw` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetPowerManagementDefaultLimit(self.raw, &mut mw) };
        check("nvmlDeviceGetPowerManagementDefaultLimit", code)?;
        Ok(mw)
    }

    /// Queries `(min_mw, max_mw)` power limit constraints (`nvmlDeviceGetPowerManagementLimitConstraints`).
    pub fn power_limit_constraints_mw(&self) -> Result<(u32, u32)> {
        let mut min_mw = 0u32;
        let mut max_mw = 0u32;
        // SAFETY: `self.raw` is valid and both out-pointers are valid.
        let code = unsafe {
            sys::nvmlDeviceGetPowerManagementLimitConstraints(self.raw, &mut min_mw, &mut max_mw)
        };
        check("nvmlDeviceGetPowerManagementLimitConstraints", code)?;
        Ok((min_mw, max_mw))
    }

    /// Queries the enforced power limit in milliwatts (`nvmlDeviceGetEnforcedPowerLimit`).
    pub fn enforced_power_limit_mw(&self) -> Result<u32> {
        let mut mw = 0u32;
        // SAFETY: `self.raw` is valid and `mw` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetEnforcedPowerLimit(self.raw, &mut mw) };
        check("nvmlDeviceGetEnforcedPowerLimit", code)?;
        Ok(mw)
    }

    /// Queries total energy consumption in millijoules since driver load (`nvmlDeviceGetTotalEnergyConsumption`).
    pub fn total_energy_consumption_mj(&self) -> Result<u64> {
        let mut mj = 0u64;
        // SAFETY: `self.raw` is valid and `mj` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetTotalEnergyConsumption(self.raw, &mut mj) };
        check("nvmlDeviceGetTotalEnergyConsumption", code)?;
        Ok(mj)
    }

    /// Queries the current P-state (`0..=15`, `nvmlDeviceGetPerformanceState`).
    pub fn performance_state(&self) -> Result<u32> {
        let mut pstate: sys::nvmlPstates_t = 0;
        // SAFETY: `self.raw` is valid and `pstate` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetPerformanceState(self.raw, &mut pstate) };
        check("nvmlDeviceGetPerformanceState", code)?;
        Ok(pstate as u32)
    }

    /// Queries the current clock frequency in MHz for `domain` (`nvmlDeviceGetClockInfo`).
    pub fn clock_mhz(&self, domain: ClockDomain) -> Result<u32> {
        let mut mhz = 0u32;
        // SAFETY: `self.raw` is valid and `mhz` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetClockInfo(self.raw, domain.to_raw(), &mut mhz) };
        check("nvmlDeviceGetClockInfo", code)?;
        Ok(mhz)
    }

    /// Queries the maximum clock frequency in MHz for `domain` (`nvmlDeviceGetMaxClockInfo`).
    pub fn max_clock_mhz(&self, domain: ClockDomain) -> Result<u32> {
        let mut mhz = 0u32;
        // SAFETY: `self.raw` is valid and `mhz` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetMaxClockInfo(self.raw, domain.to_raw(), &mut mhz) };
        check("nvmlDeviceGetMaxClockInfo", code)?;
        Ok(mhz)
    }

    /// Queries the current clock event/throttle reason bitmask (`nvmlDeviceGetCurrentClocksEventReasons`,
    /// falling back to `nvmlDeviceGetCurrentClocksThrottleReasons` on older drivers).
    pub fn clocks_event_reasons(&self) -> Result<u64> {
        let mut reasons = 0u64;
        // SAFETY: `self.raw` is valid and `reasons` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetCurrentClocksEventReasons(self.raw, &mut reasons) };
        if Status(code).is_ok() {
            return Ok(reasons);
        }
        if !Status(code).is_function_not_found() {
            return Err(crate::Error::Status {
                call: "nvmlDeviceGetCurrentClocksEventReasons",
                status: Status(code),
            });
        }
        // SAFETY: `self.raw` is valid and `reasons` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetCurrentClocksThrottleReasons(self.raw, &mut reasons) };
        check("nvmlDeviceGetCurrentClocksThrottleReasons", code)?;
        Ok(reasons)
    }

    /// Queries current and pending ECC mode (`nvmlDeviceGetEccMode`).
    pub fn ecc_mode(&self) -> Result<EccMode> {
        let mut current: sys::nvmlEnableState_t = 0;
        let mut pending: sys::nvmlEnableState_t = 0;
        // SAFETY: `self.raw` is valid and `current`/`pending` are valid out-pointers.
        let code = unsafe { sys::nvmlDeviceGetEccMode(self.raw, &mut current, &mut pending) };
        check("nvmlDeviceGetEccMode", code)?;
        let enabled = sys::nvmlEnableState_enum_NVML_FEATURE_ENABLED;
        Ok(EccMode { current: current == enabled, pending: pending == enabled })
    }

    /// Queries total ECC error counts (`nvmlDeviceGetTotalEccErrors`).
    pub fn total_ecc_errors(
        &self,
        error_type: MemoryErrorType,
        counter_type: EccCounterType,
    ) -> Result<u64> {
        let mut count = 0u64;
        // SAFETY: `self.raw` is valid and `count` is a valid out-pointer.
        let code = unsafe {
            sys::nvmlDeviceGetTotalEccErrors(
                self.raw,
                error_type.to_raw(),
                counter_type.to_raw(),
                &mut count,
            )
        };
        check("nvmlDeviceGetTotalEccErrors", code)?;
        Ok(count)
    }

    /// Queries running compute processes into `out`, reusing `scratch` and `out` capacity across
    /// calls for zero-allocation steady-state polling.
    pub fn compute_running_processes_into(
        &self,
        scratch: &mut ProcessScratch,
        out: &mut Vec<ProcessInfo>,
    ) -> Result<()> {
        self.query_processes_into(
            "nvmlDeviceGetComputeRunningProcesses_v3",
            |dev, count, ptr| {
                // SAFETY: caller ensures `ptr` is null when `*count == 0` or points to `*count` elements.
                unsafe { sys::nvmlDeviceGetComputeRunningProcesses_v3(dev, count, ptr) }
            },
            scratch,
            out,
        )
    }

    /// Queries running compute processes (`nvmlDeviceGetComputeRunningProcesses_v3`).
    pub fn compute_running_processes(&self) -> Result<Vec<ProcessInfo>> {
        let mut scratch = ProcessScratch::new();
        let mut out = Vec::new();
        self.compute_running_processes_into(&mut scratch, &mut out)?;
        Ok(out)
    }

    /// Queries running graphics processes into `out`, reusing `scratch` and `out` capacity across
    /// calls for zero-allocation steady-state polling.
    pub fn graphics_running_processes_into(
        &self,
        scratch: &mut ProcessScratch,
        out: &mut Vec<ProcessInfo>,
    ) -> Result<()> {
        self.query_processes_into(
            "nvmlDeviceGetGraphicsRunningProcesses_v3",
            |dev, count, ptr| {
                // SAFETY: caller ensures `ptr` is null when `*count == 0` or points to `*count` elements.
                unsafe { sys::nvmlDeviceGetGraphicsRunningProcesses_v3(dev, count, ptr) }
            },
            scratch,
            out,
        )
    }

    /// Queries running graphics processes (`nvmlDeviceGetGraphicsRunningProcesses_v3`).
    pub fn graphics_running_processes(&self) -> Result<Vec<ProcessInfo>> {
        let mut scratch = ProcessScratch::new();
        let mut out = Vec::new();
        self.graphics_running_processes_into(&mut scratch, &mut out)?;
        Ok(out)
    }

    fn query_processes_into<F>(
        &self,
        call: &'static str,
        mut f: F,
        scratch: &mut ProcessScratch,
        out: &mut Vec<ProcessInfo>,
    ) -> Result<()>
    where
        F: FnMut(sys::nvmlDevice_t, *mut u32, *mut sys::nvmlProcessInfo_t) -> sys::nvmlReturn_t,
    {
        out.clear();
        let mut count = 0u32;

        // Bounded two-phase sizing: if a new process starts between the size query and the read,
        // NVML returns `NVML_ERROR_INSUFFICIENT_SIZE` with the updated count.
        for _ in 0..4 {
            let ptr = if count == 0 {
                std::ptr::null_mut()
            } else {
                // SAFETY: `nvmlProcessInfo_t` is a plain C POD struct of integers.
                scratch.raw.resize(count as usize, unsafe { std::mem::zeroed() });
                scratch.raw.as_mut_ptr()
            };
            let code = f(self.raw, &mut count, ptr);
            let status = Status(code);
            if status.is_ok() {
                let actual = (count as usize).min(scratch.raw.len());
                out.extend(scratch.raw[..actual].iter().map(ProcessInfo::from_raw));
                return Ok(());
            }
            if status.is_insufficient_size() && count > 0 {
                continue;
            }
            return Err(crate::Error::Status { call, status });
        }

        Err(crate::Error::Status { call, status: Status::INSUFFICIENT_SIZE })
    }

    /// Queries current PCIe link generation (`nvmlDeviceGetCurrPcieLinkGeneration`).
    pub fn pcie_generation(&self) -> Result<u32> {
        let mut pcie_gen = 0u32;
        // SAFETY: `self.raw` is valid and `pcie_gen` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetCurrPcieLinkGeneration(self.raw, &mut pcie_gen) };
        check("nvmlDeviceGetCurrPcieLinkGeneration", code)?;
        Ok(pcie_gen)
    }

    /// Queries maximum PCIe link generation (`nvmlDeviceGetMaxPcieLinkGeneration`).
    pub fn max_pcie_generation(&self) -> Result<u32> {
        let mut pcie_gen = 0u32;
        // SAFETY: `self.raw` is valid and `pcie_gen` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetMaxPcieLinkGeneration(self.raw, &mut pcie_gen) };
        check("nvmlDeviceGetMaxPcieLinkGeneration", code)?;
        Ok(pcie_gen)
    }

    /// Queries current PCIe link width (`nvmlDeviceGetCurrPcieLinkWidth`).
    pub fn pcie_width(&self) -> Result<u32> {
        let mut width = 0u32;
        // SAFETY: `self.raw` is valid and `width` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetCurrPcieLinkWidth(self.raw, &mut width) };
        check("nvmlDeviceGetCurrPcieLinkWidth", code)?;
        Ok(width)
    }

    /// Queries maximum PCIe link width (`nvmlDeviceGetMaxPcieLinkWidth`).
    pub fn max_pcie_width(&self) -> Result<u32> {
        let mut width = 0u32;
        // SAFETY: `self.raw` is valid and `width` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetMaxPcieLinkWidth(self.raw, &mut width) };
        check("nvmlDeviceGetMaxPcieLinkWidth", code)?;
        Ok(width)
    }

    /// Queries PCIe throughput in KB/s over a ~20ms interval (`nvmlDeviceGetPcieThroughput`).
    pub fn pcie_throughput_kbps(&self, counter: PcieCounter) -> Result<u32> {
        let mut kbps = 0u32;
        // SAFETY: `self.raw` is valid and `kbps` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetPcieThroughput(self.raw, counter.to_raw(), &mut kbps) };
        check("nvmlDeviceGetPcieThroughput", code)?;
        Ok(kbps)
    }

    /// Queries the PCIe replay counter (`nvmlDeviceGetPcieReplayCounter`).
    pub fn pcie_replay_counter(&self) -> Result<u32> {
        let mut count = 0u32;
        // SAFETY: `self.raw` is valid and `count` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetPcieReplayCounter(self.raw, &mut count) };
        check("nvmlDeviceGetPcieReplayCounter", code)?;
        Ok(count)
    }

    /// Queries whether NVLink `link` is active (`nvmlDeviceGetNvLinkState`).
    pub fn nvlink_state(&self, link: u32) -> Result<bool> {
        let mut state: sys::nvmlEnableState_t = 0;
        // SAFETY: `self.raw` is valid and `state` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetNvLinkState(self.raw, link, &mut state) };
        check("nvmlDeviceGetNvLinkState", code)?;
        Ok(state == sys::nvmlEnableState_enum_NVML_FEATURE_ENABLED)
    }

    /// Queries NVLink protocol version for `link` (`nvmlDeviceGetNvLinkVersion`).
    pub fn nvlink_version(&self, link: u32) -> Result<u32> {
        let mut ver = 0u32;
        // SAFETY: `self.raw` is valid and `ver` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetNvLinkVersion(self.raw, link, &mut ver) };
        check("nvmlDeviceGetNvLinkVersion", code)?;
        Ok(ver)
    }

    /// Queries an NVLink capability bit for `link` (`nvmlDeviceGetNvLinkCapability`).
    pub fn nvlink_capability(&self, link: u32, cap: NvLinkCapability) -> Result<bool> {
        let mut res = 0u32;
        // SAFETY: `self.raw` is valid and `res` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetNvLinkCapability(self.raw, link, cap.to_raw(), &mut res) };
        check("nvmlDeviceGetNvLinkCapability", code)?;
        Ok(res != 0)
    }

    /// Queries an NVLink error counter for `link` (`nvmlDeviceGetNvLinkErrorCounter`).
    pub fn nvlink_error_counter(&self, link: u32, counter: NvLinkErrorCounter) -> Result<u64> {
        let mut count = 0u64;
        // SAFETY: `self.raw` is valid and `count` is a valid out-pointer.
        let code = unsafe {
            sys::nvmlDeviceGetNvLinkErrorCounter(self.raw, link, counter.to_raw(), &mut count)
        };
        check("nvmlDeviceGetNvLinkErrorCounter", code)?;
        Ok(count)
    }
}
