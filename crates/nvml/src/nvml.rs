//! Safe NVIDIA Management Library (NVML) wrapper over `nvml-sys`.
//!
//! **What.**
//! - [`Nvml`] — `Arc`-shared NVML session (`nvmlInit_v2` / `nvmlShutdown` once when the last
//!   session and device drop), system version queries, and device enumeration by index, UUID, or
//!   [`PciBusId`].
//! - [`Device`] — device identity, memory, utilization, thermal/fan, power/energy, clocks, ECC,
//!   running compute/graphics processes, PCIe link/throughput, and NVLink telemetry.
//! - [`Error`] / [`Status`] — `thiserror` error enum with a `Copy` integer newtype wrapping
//!   `nvmlReturn_t`.
//!
//! **Why.** Provides one safe, zero-churn NVML wrapper for nv-rs and downstream consumers such as
//! `gpu-info-rs` (PLAN D11).
//! **How.** Stack buffers for fixed-size queries, bounded two-phase sizing for process lists, and
//! `Arc<NvmlSession>` lifetime pinning on every [`Device`].
//! **Used by** `gpu-info-rs` and nv-rs GPU telemetry tests.

pub mod device;
pub mod error;
pub mod session;
pub mod types;

pub use device::Device;
pub use error::{Error, Result, Status};
pub use nvml_sys as sys;
pub use session::Nvml;
pub use types::{
    Architecture, Bar1MemoryInfo, ClockDomain, CodecUtilization, EccCounterType, EccMode,
    MemoryErrorType, MemoryInfo, NvLinkCapability, NvLinkErrorCounter, PciBusId, PciInfo,
    PcieCounter, ProcessInfo, ProcessScratch, TemperatureThreshold, Utilization,
};

#[cfg(test)]
mod tests {
    use std::ffi::c_char;
    use std::str::FromStr;

    use super::*;
    use crate::types::{decode_c_buf, encode_c_buf, struct_version, validate_uuid};

    #[test]
    fn status_copy_classification_and_formatting() {
        let ok = Status::SUCCESS;
        let ok_copy = ok;
        assert!(ok.is_ok());
        assert!(ok_copy.is_ok());
        assert!(!ok.is_not_supported());

        assert!(Status::NOT_SUPPORTED.is_not_supported());
        assert!(Status::NOT_FOUND.is_not_found());
        assert!(Status::INSUFFICIENT_SIZE.is_insufficient_size());
        assert!(Status::FUNCTION_NOT_FOUND.is_function_not_found());

        assert_eq!(Status::SUCCESS.known_name(), Some("NVML_SUCCESS"));
        assert_eq!(Status::NOT_SUPPORTED.known_name(), Some("NVML_ERROR_NOT_SUPPORTED"));

        let unknown = Status(9999);
        assert_eq!(unknown.known_name(), None);
        let disp = unknown.to_string();
        assert!(disp.contains("9999"), "unexpected display: {disp}");
    }

    #[test]
    fn pci_bus_id_parse_and_roundtrip() {
        let parsed = PciBusId::from_str("00000000:01:00.0").expect("valid 8-digit domain");
        assert_eq!(parsed.domain, 0);
        assert_eq!(parsed.bus, 1);
        assert_eq!(parsed.device, 0);
        assert_eq!(parsed.function, 0);
        assert_eq!(parsed.to_string(), "00000000:01:00.0");

        let short_domain = PciBusId::from_str("0000:af:1f.7").expect("valid 4-digit domain");
        assert_eq!(short_domain.domain, 0);
        assert_eq!(short_domain.bus, 0xaf);
        assert_eq!(short_domain.device, 0x1f);
        assert_eq!(short_domain.function, 7);
        assert_eq!(short_domain.to_string(), "00000000:af:1f.7");

        assert!(PciBusId::from_str("0000:01:00").is_err());
        assert!(PciBusId::from_str("0000:01:20.0").is_err()); // device 32 > 31
        assert!(PciBusId::from_str("0000:01:00.8").is_err()); // function 8 > 7
        assert!(PciBusId::from_str("zzzz:01:00.0").is_err());
    }

    #[test]
    fn uuid_validation_accepts_gpu_and_mig_and_rejects_malformed() {
        assert!(validate_uuid("GPU-12345678-1234-1234-1234-1234567890ab").is_ok());
        assert!(validate_uuid("MIG-12345678-1234-1234-1234-1234567890ab").is_ok());
        assert!(validate_uuid("").is_err());
        assert!(validate_uuid("12345678-1234-1234-1234-1234567890ab").is_err());
        assert!(validate_uuid("GPU-abc\0def").is_err());
        let oversized = format!("GPU-{}", "a".repeat(sys::NVML_DEVICE_UUID_V2_BUFFER_SIZE as usize));
        assert!(validate_uuid(&oversized).is_err());
    }

    #[test]
    fn c_buffer_encode_and_decode_boundaries() {
        let encoded = encode_c_buf::<8>("RTX", "label").expect("fits with NUL");
        assert_eq!(decode_c_buf(&encoded), "RTX");

        // Exact boundary: 7 bytes + NUL fits in 8.
        let exact = encode_c_buf::<8>("1234567", "label").expect("7 chars + NUL fits in 8");
        assert_eq!(decode_c_buf(&exact), "1234567");

        // 8 bytes leaves no room for NUL.
        assert!(encode_c_buf::<8>("12345678", "label").is_err());
        assert!(encode_c_buf::<8>("", "label").is_err());
        assert!(encode_c_buf::<8>("ab\0c", "label").is_err());

        // Non-NUL-terminated C buffer decodes full slice without overreading.
        let full = [b'A' as c_char, b'B' as c_char, b'C' as c_char];
        assert_eq!(decode_c_buf(&full), "ABC");
    }

    #[test]
    fn struct_version_matches_nvml_macro_layout() {
        let expected_v2 = (std::mem::size_of::<sys::nvmlMemory_v2_t>() as u32) | (2 << 24);
        assert_eq!(struct_version::<sys::nvmlMemory_v2_t>(2), expected_v2);
    }

    #[test]
    fn process_info_maps_sentinels_to_none() {
        let raw = sys::nvmlProcessInfo_t {
            pid: 4242,
            usedGpuMemory: u64::MAX,
            gpuInstanceId: u32::MAX,
            computeInstanceId: u32::MAX,
        };
        let info = ProcessInfo::from_raw(&raw);
        assert_eq!(info.pid, 4242);
        assert_eq!(info.used_gpu_memory, None);
        assert_eq!(info.gpu_instance_id, None);
        assert_eq!(info.compute_instance_id, None);

        let raw_active = sys::nvmlProcessInfo_t {
            pid: 1337,
            usedGpuMemory: 1024 * 1024,
            gpuInstanceId: 1,
            computeInstanceId: 2,
        };
        let active = ProcessInfo::from_raw(&raw_active);
        assert_eq!(active.used_gpu_memory, Some(1024 * 1024));
        assert_eq!(active.gpu_instance_id, Some(1));
        assert_eq!(active.compute_instance_id, Some(2));
    }

    #[test]
    fn pci_info_splits_vendor_and_device_ids() {
        let pci = PciInfo {
            bus_id: "00000000:01:00.0".to_owned(),
            domain: 0,
            bus: 1,
            device: 0,
            pci_device_id: 0x2208_10de,
            pci_subsystem_id: 0,
        };
        assert_eq!(pci.vendor_id(), 0x10de);
        assert_eq!(pci.device_id(), 0x2208);
    }

    #[test]
    #[ignore = "GPU: needs the NVIDIA driver; run via xtask gpu-test"]
    fn live_rtx_3080_ti_telemetry() {
        let nvml = Nvml::init().expect("NVML initializes");
        let _nvml_clone = nvml.clone();

        let driver = nvml.driver_version().expect("driver version");
        let nvml_ver = nvml.nvml_version().expect("NVML version");
        let (cuda_maj, cuda_min) = nvml.cuda_driver_version().expect("CUDA driver version");
        assert!(!driver.is_empty());
        assert!(!nvml_ver.is_empty());
        assert!(cuda_maj >= 11, "unexpected CUDA major {cuda_maj}.{cuda_min}");

        let devices = nvml.devices().expect("enumerate devices");
        assert!(!devices.is_empty(), "expected at least one NVIDIA GPU");

        let dev = &devices[0];
        let name = dev.name().expect("device name");
        let uuid = dev.uuid().expect("device UUID");
        let pci = dev.pci_info().expect("PCI info");
        assert!(!name.is_empty());
        assert!(uuid.starts_with("GPU-"));
        assert_eq!(pci.vendor_id(), 0x10de);

        // Lookup by UUID and PCI bus ID must resolve the same device index.
        let idx = dev.index().expect("device index");
        let by_uuid = nvml.device_by_uuid(&uuid).expect("lookup by UUID");
        assert_eq!(by_uuid.index().expect("uuid index"), idx);
        let parsed_bus = PciBusId::from_str(&pci.bus_id).expect("parse NVML PCI bus ID");
        let by_pci = nvml.device_by_pci_bus_id(&parsed_bus).expect("lookup by PCI bus ID");
        assert_eq!(by_pci.index().expect("pci index"), idx);

        let mem = dev.memory_info().expect("memory info");
        assert!(mem.total > 0);
        assert!(
            mem.free + mem.used <= mem.total,
            "free ({}) + used ({}) > total ({})",
            mem.free,
            mem.used,
            mem.total,
        );
        // On Linux/TCC NVML v2 `used` includes `reserved`; on Windows WDDM `reserved` is separate.
        if let Some(reserved) = mem.reserved {
            assert!(
                mem.free + mem.used + reserved == mem.total
                    || (mem.free + mem.used == mem.total && reserved <= mem.used),
                "unexpected memory accounting: free={}, used={}, reserved={}, total={}",
                mem.free,
                mem.used,
                reserved,
                mem.total,
            );
        }

        let util = dev.utilization_rates().expect("utilization rates");
        assert!(util.gpu <= 100 && util.memory <= 100);

        let temp = dev.temperature_gpu().expect("GPU temp");
        assert!(temp > 0 && temp < 120);

        let power = dev.power_usage_mw().expect("power usage");
        let limit = dev.power_limit_mw().expect("power limit");
        assert!(power > 0 && limit >= power / 2);

        let gfx_clk = dev.clock_mhz(ClockDomain::Graphics).expect("graphics clock");
        let mem_clk = dev.clock_mhz(ClockDomain::Memory).expect("memory clock");
        assert!(gfx_clk > 0 && mem_clk > 0);

        let pcie_gen = dev.pcie_generation().expect("PCIe gen");
        let width = dev.pcie_width().expect("PCIe width");
        assert!(pcie_gen >= 1 && width >= 1);

        // ECC may be unsupported on consumer GeForce GPUs; accept Ok or NOT_SUPPORTED.
        match dev.ecc_mode() {
            Ok(_) => {}
            Err(Error::Status { status, .. }) if status.is_not_supported() => {}
            Err(e) => panic!("unexpected ecc_mode error: {e}"),
        }

        // Process queries must succeed even if empty.
        let mut scratch = ProcessScratch::new();
        let mut procs = Vec::new();
        dev.compute_running_processes_into(&mut scratch, &mut procs).expect("compute processes");
        let _gfx_procs = dev.graphics_running_processes().expect("graphics processes");

        // NVLink is absent on GeForce RTX 3080 Ti; verify graceful NOT_SUPPORTED or Ok.
        match dev.nvlink_state(0) {
            Ok(_) => {}
            Err(Error::Status { status, .. }) if status.is_not_supported() => {}
            Err(e) => panic!("unexpected nvlink_state error: {e}"),
        }
    }
}
