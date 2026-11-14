//! Safe Windows NVAPI wrapper over `nvapi-sys`.
//!
//! **What.**
//! - [`NvApi`] — `Arc`-shared NVAPI session (`NvAPI_Initialize` / `NvAPI_Unload` once on last
//!   drop), driver/branch version, physical & logical GPU enumeration, display enumeration,
//!   Quadro Sync / G-Sync devices, and Driver Settings (DRS) sessions.
//! - [`PhysicalGpu`] — full name, UUID, PCI identifiers, architecture, core count, VBIOS,
//!   memory info, thermal sensors, tachometer, P-state, dynamic P-state utilization, clock
//!   frequencies, and throttling reasons.
//! - [`Display`] — display name/ID, HDR capabilities, and VRR (G-Sync Compatible) status.
//! - [`DrsSession`] — profile count, global profile, profile enumeration, and profile lookup.
//!
//! **Why.** Provides typed, RAII-safe access to Windows-only NVIDIA driver features (displays,
//! HDR, VRR, DRS profiles) while returning [`Error::UnsupportedPlatform`] cleanly on non-Windows.
//! **How.** Windows FFI calls live in `win.rs` behind `#[cfg(windows)]`; cross-platform types and
//! codecs live in [`types`] and are unit-tested on every platform.
//! **Used by** nv-rs system/display diagnostics and GPU tests.

pub mod error;
pub mod types;
#[cfg(windows)]
mod win;

pub use error::{Error, Result, Status};
pub use nvapi_sys as sys;
pub use types::{
    ArchInfo, ClockFrequencies, ClockKind, DriverVersion, DrsProfileInfo, DynamicPstates,
    GSyncCapabilities, GSyncStatus, GpuMemoryInfo, HdrCapabilities, PciIdentifiers,
    PerfDecreaseReasons, ThermalSensor, VrrInfo,
};

/// Handle to an initialized NVAPI session.
///
/// Cloning [`NvApi`] shares the underlying `Arc` session; `NvAPI_Unload` runs once when the last
/// [`NvApi`] and all handles derived from it are dropped. On non-Windows targets, [`NvApi::init`]
/// returns [`Error::UnsupportedPlatform`].
#[derive(Clone, Debug)]
pub struct NvApi {
    #[cfg(windows)]
    session: std::sync::Arc<win::NvApiSession>,
}

impl NvApi {
    /// Loads `nvapi64.dll` and initializes an NVAPI session (`NvAPI_Initialize`).
    pub fn init() -> Result<Self> {
        #[cfg(windows)]
        {
            Ok(Self { session: win::init_session()? })
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries the NVAPI interface version string (`NvAPI_GetInterfaceVersionString`).
    pub fn interface_version(&self) -> Result<String> {
        #[cfg(windows)]
        {
            win::interface_version(&self.session)
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries the NVIDIA display driver version and branch (`NvAPI_SYS_GetDriverAndBranchVersion`).
    pub fn driver_and_branch_version(&self) -> Result<DriverVersion> {
        #[cfg(windows)]
        {
            win::driver_and_branch_version(&self.session)
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Enumerates physical WDDM GPUs (`NvAPI_EnumPhysicalGPUs`).
    pub fn physical_gpus(&self) -> Result<Vec<PhysicalGpu>> {
        #[cfg(windows)]
        {
            win::enum_physical_gpus(&self.session)
                .map(|v| v.into_iter().map(|inner| PhysicalGpu { inner }).collect())
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Enumerates physical TCC-mode GPUs (`NvAPI_EnumTCCPhysicalGPUs`).
    pub fn tcc_physical_gpus(&self) -> Result<Vec<PhysicalGpu>> {
        #[cfg(windows)]
        {
            win::enum_tcc_physical_gpus(&self.session)
                .map(|v| v.into_iter().map(|inner| PhysicalGpu { inner }).collect())
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Enumerates logical GPUs (each representing one or more physical GPUs in SLI/mGPU)
    /// (`NvAPI_EnumLogicalGPUs`).
    pub fn logical_gpus(&self) -> Result<Vec<LogicalGpu>> {
        #[cfg(windows)]
        {
            win::enum_logical_gpus(&self.session)
                .map(|v| v.into_iter().map(|inner| LogicalGpu { inner }).collect())
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Enumerates attached NVIDIA displays (`NvAPI_EnumNvidiaDisplayHandle`).
    pub fn displays(&self) -> Result<Vec<Display>> {
        #[cfg(windows)]
        {
            win::enum_displays(&self.session)
                .map(|v| v.into_iter().map(|inner| Display { inner }).collect())
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Enumerates Quadro Sync / G-Sync hardware devices (`NvAPI_GSync_EnumSyncDevices`).
    pub fn gsync_devices(&self) -> Result<Vec<GSyncDevice>> {
        #[cfg(windows)]
        {
            win::enum_gsync_devices(&self.session)
                .map(|v| v.into_iter().map(|inner| GSyncDevice { inner }).collect())
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Opens a Driver Settings (DRS) session and loads current system settings.
    pub fn drs_session(&self) -> Result<DrsSession> {
        #[cfg(windows)]
        {
            win::DrsSession::open(&self.session).map(|inner| DrsSession { inner })
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }
}

/// Handle to a physical NVIDIA GPU in NVAPI.
#[derive(Clone, Debug)]
pub struct PhysicalGpu {
    #[cfg(windows)]
    inner: win::PhysicalGpu,
}

impl PhysicalGpu {
    /// Queries the full marketing name (`NvAPI_GPU_GetFullName`).
    pub fn full_name(&self) -> Result<String> {
        #[cfg(windows)]
        {
            self.inner.full_name()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries the 16-byte GPU UUID (`NvAPI_GPU_GetUUID`).
    pub fn uuid(&self) -> Result<[u8; 16]> {
        #[cfg(windows)]
        {
            self.inner.uuid()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries PCI device, subsystem, revision, and external IDs (`NvAPI_GPU_GetPCIIdentifiers`).
    pub fn pci_identifiers(&self) -> Result<PciIdentifiers> {
        #[cfg(windows)]
        {
            self.inner.pci_identifiers()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries the PCI bus ID (`NvAPI_GPU_GetBusId`).
    pub fn bus_id(&self) -> Result<u32> {
        #[cfg(windows)]
        {
            self.inner.bus_id()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries the PCI bus slot ID (`NvAPI_GPU_GetBusSlotId`).
    pub fn bus_slot_id(&self) -> Result<u32> {
        #[cfg(windows)]
        {
            self.inner.bus_slot_id()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries GPU architecture, implementation, and revision (`NvAPI_GPU_GetArchInfo`).
    pub fn arch_info(&self) -> Result<ArchInfo> {
        #[cfg(windows)]
        {
            self.inner.arch_info()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries CUDA core count (`NvAPI_GPU_GetGpuCoreCount`).
    pub fn core_count(&self) -> Result<u32> {
        #[cfg(windows)]
        {
            self.inner.core_count()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries VBIOS version string (`NvAPI_GPU_GetVbiosVersionString`).
    pub fn vbios_version(&self) -> Result<String> {
        #[cfg(windows)]
        {
            self.inner.vbios_version()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries framebuffer and shared memory footprint (`NvAPI_GPU_GetMemoryInfoEx`).
    pub fn memory_info(&self) -> Result<GpuMemoryInfo> {
        #[cfg(windows)]
        {
            self.inner.memory_info()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries all thermal sensors on this GPU (`NvAPI_GPU_GetThermalSettings`).
    pub fn thermal_settings(&self) -> Result<Vec<ThermalSensor>> {
        #[cfg(windows)]
        {
            self.inner.thermal_settings()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries fan tachometer RPM (`NvAPI_GPU_GetTachReading`).
    pub fn tach_reading(&self) -> Result<u32> {
        #[cfg(windows)]
        {
            self.inner.tach_reading()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries current P-state ID (`NvAPI_GPU_GetCurrentPstate`).
    pub fn current_pstate(&self) -> Result<u32> {
        #[cfg(windows)]
        {
            self.inner.current_pstate()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries dynamic P-state engine utilization (`NvAPI_GPU_GetDynamicPstatesInfoEx`).
    pub fn dynamic_pstates(&self) -> Result<DynamicPstates> {
        #[cfg(windows)]
        {
            self.inner.dynamic_pstates()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries clock frequencies in kHz for `kind` (`NvAPI_GPU_GetAllClockFrequencies`).
    pub fn clock_frequencies(&self, kind: ClockKind) -> Result<ClockFrequencies> {
        #[cfg(windows)]
        {
            self.inner.clock_frequencies(kind)
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries active throttling / performance-decrease reasons (`NvAPI_GPU_GetPerfDecreaseInfo`).
    pub fn perf_decrease_info(&self) -> Result<PerfDecreaseReasons> {
        #[cfg(windows)]
        {
            self.inner.perf_decrease_info()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }
}

/// Handle to a logical GPU (one or more physical GPUs grouped for SLI/mGPU).
#[derive(Clone, Debug)]
pub struct LogicalGpu {
    #[cfg(windows)]
    inner: win::LogicalGpu,
}

impl LogicalGpu {
    /// Returns the physical GPUs backing this logical GPU (`NvAPI_GetPhysicalGPUsFromLogicalGPU`).
    pub fn physical_gpus(&self) -> Result<Vec<PhysicalGpu>> {
        #[cfg(windows)]
        {
            self.inner
                .physical_gpus()
                .map(|v| v.into_iter().map(|inner| PhysicalGpu { inner }).collect())
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }
}

/// Handle to an attached NVIDIA display output.
#[derive(Clone, Debug)]
pub struct Display {
    #[cfg(windows)]
    inner: win::Display,
}

impl Display {
    /// Queries the associated GDI display name (`NvAPI_GetAssociatedNvidiaDisplayName`).
    pub fn name(&self) -> Result<String> {
        #[cfg(windows)]
        {
            self.inner.name()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Resolves the numeric NVAPI display ID (`NvAPI_DISP_GetDisplayIdByDisplayName`).
    pub fn display_id(&self) -> Result<u32> {
        #[cfg(windows)]
        {
            self.inner.display_id()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries HDR capabilities (`NvAPI_Disp_GetHdrCapabilities`).
    pub fn hdr_capabilities(&self) -> Result<HdrCapabilities> {
        #[cfg(windows)]
        {
            self.inner.hdr_capabilities()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries Variable Refresh Rate (VRR / G-Sync Compatible) status (`NvAPI_Disp_GetVRRInfo`).
    pub fn vrr_info(&self) -> Result<VrrInfo> {
        #[cfg(windows)]
        {
            self.inner.vrr_info()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }
}

/// Handle to a Quadro Sync / G-Sync hardware device.
#[derive(Clone, Debug)]
pub struct GSyncDevice {
    #[cfg(windows)]
    inner: win::GSyncDevice,
}

impl GSyncDevice {
    /// Queries G-Sync board capabilities (`NvAPI_GSync_QueryCapabilities`).
    pub fn capabilities(&self) -> Result<GSyncCapabilities> {
        #[cfg(windows)]
        {
            self.inner.capabilities()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries sync status for `gpu` (`NvAPI_GSync_GetSyncStatus`).
    pub fn sync_status(&self, gpu: &PhysicalGpu) -> Result<GSyncStatus> {
        #[cfg(windows)]
        {
            self.inner.sync_status(&gpu.inner)
        }
        #[cfg(not(windows))]
        {
            let _ = gpu;
            Err(Error::UnsupportedPlatform)
        }
    }
}

/// RAII handle to an NVAPI Driver Settings (DRS) session (`NvAPI_DRS_DestroySession` on drop).
#[derive(Debug)]
pub struct DrsSession {
    #[cfg(windows)]
    inner: win::DrsSession,
}

impl DrsSession {
    /// Queries the number of profiles in the loaded DRS database (`NvAPI_DRS_GetNumProfiles`).
    pub fn num_profiles(&self) -> Result<u32> {
        #[cfg(windows)]
        {
            self.inner.num_profiles()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Queries the current global DRS profile (`NvAPI_DRS_GetCurrentGlobalProfile`).
    pub fn global_profile(&self) -> Result<DrsProfileInfo> {
        #[cfg(windows)]
        {
            self.inner.global_profile()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Enumerates all profile names in the loaded DRS database (`NvAPI_DRS_EnumProfiles`).
    pub fn profile_names(&self) -> Result<Vec<String>> {
        #[cfg(windows)]
        {
            self.inner.profile_names()
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    /// Looks up a DRS profile by name (`NvAPI_DRS_FindProfileByName`). Returns `Ok(None)` when
    /// `NVAPI_PROFILE_NOT_FOUND` is reported.
    pub fn find_profile(&self, name: &str) -> Result<Option<DrsProfileInfo>> {
        #[cfg(windows)]
        {
            self.inner.find_profile(name)
        }
        #[cfg(not(windows))]
        {
            let _ = name;
            Err(Error::UnsupportedPlatform)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;
    use crate::types::{decode_short_string, decode_utf16_z, encode_utf16_z, make_version};

    #[test]
    fn status_copy_classification_and_formatting() {
        let ok = Status::OK;
        let ok_copy = ok;
        assert!(ok.is_ok());
        assert!(ok_copy.is_ok());
        assert!(!ok.is_not_supported());

        assert!(Status::END_ENUMERATION.is_end_enumeration());
        assert!(Status::NOT_SUPPORTED.is_not_supported());
        assert!(Status::NO_IMPLEMENTATION.is_not_supported());
        assert!(Status::NVIDIA_DEVICE_NOT_FOUND.is_device_not_found());
        assert!(Status::PROFILE_NOT_FOUND.is_profile_not_found());

        assert_eq!(Status::OK.known_name(), Some("NVAPI_OK"));
        assert_eq!(Status::PROFILE_NOT_FOUND.known_name(), Some("NVAPI_PROFILE_NOT_FOUND"));

        let unknown = Status(-4242);
        assert_eq!(unknown.known_name(), None);
        let disp = unknown.to_string();
        assert!(disp.contains("-4242"), "unexpected display: {disp}");
    }

    #[test]
    fn make_version_encodes_size_and_revision() {
        #[repr(C)]
        struct Dummy {
            _a: u32,
            _b: u64,
        }
        let v = make_version::<Dummy>(3);
        assert_eq!(v & 0xffff, std::mem::size_of::<Dummy>() as u32);
        assert_eq!(v >> 16, 3);
    }

    #[test]
    fn short_and_utf16_string_codecs_handle_boundaries() {
        let mut short = [0 as c_char; 8];
        short[0] = b'R' as c_char;
        short[1] = b'T' as c_char;
        short[2] = b'X' as c_char;
        assert_eq!(decode_short_string(&short), "RTX");

        let full = [b'N' as c_char, b'V' as c_char];
        assert_eq!(decode_short_string(&full), "NV");

        let mut u16_buf = [0u16; 8];
        encode_utf16_z("Global", &mut u16_buf).expect("fits in 8 units");
        assert_eq!(decode_utf16_z(&u16_buf), "Global");

        // Exact fit: 7 UTF-16 units + NUL.
        encode_utf16_z("1234567", &mut u16_buf).expect("7 units + NUL fits in 8");
        assert_eq!(decode_utf16_z(&u16_buf), "1234567");

        // Overflow and interior NUL rejections.
        assert!(encode_utf16_z("12345678", &mut u16_buf).is_err());
        assert!(encode_utf16_z("ab\0cd", &mut u16_buf).is_err());
        assert!(encode_utf16_z("", &mut []).is_err());
    }

    #[test]
    fn driver_version_splits_major_and_minor() {
        let ver = DriverVersion { version: 62124, branch: "r620_00".to_owned() };
        assert_eq!(ver.major_minor(), (621, 24));
    }

    #[test]
    fn perf_decrease_reasons_bitflags() {
        let none = PerfDecreaseReasons(0);
        assert!(none.is_none());
        assert!(!none.thermal_protection());
        assert!(!none.power_control());

        let throttled = PerfDecreaseReasons(
            PerfDecreaseReasons::THERMAL_PROTECTION | PerfDecreaseReasons::POWER_CONTROL,
        );
        assert!(!throttled.is_none());
        assert!(throttled.thermal_protection());
        assert!(throttled.power_control());
        assert!(!throttled.insufficient_power());
    }

    #[cfg(not(windows))]
    #[test]
    fn non_windows_init_returns_unsupported_platform() {
        assert!(matches!(NvApi::init(), Err(Error::UnsupportedPlatform)));
    }

    #[test]
    #[ignore = "GPU: needs the NVIDIA Windows driver; run via xtask gpu-test"]
    fn live_rtx_3080_ti_nvapi_queries() {
        let nvapi = NvApi::init().expect("NVAPI initializes");
        let _nvapi_clone = nvapi.clone();

        let iface = nvapi.interface_version().expect("interface version");
        assert!(!iface.is_empty());

        let drv = nvapi.driver_and_branch_version().expect("driver and branch version");
        assert!(drv.version > 0);
        assert!(!drv.branch.is_empty());

        let gpus = nvapi.physical_gpus().expect("enumerate physical GPUs");
        assert!(!gpus.is_empty(), "expected at least one physical GPU");
        let gpu = &gpus[0];

        let name = gpu.full_name().expect("GPU full name");
        let pci = gpu.pci_identifiers().expect("PCI identifiers");
        let arch = gpu.arch_info().expect("arch info");
        let cores = gpu.core_count().expect("CUDA core count");
        let vbios = gpu.vbios_version().expect("VBIOS version");
        assert!(!name.is_empty());
        assert_ne!(pci.device_id, 0);
        assert_ne!(arch.architecture, 0);
        assert!(cores > 0);
        assert!(!vbios.is_empty());

        let mem = gpu.memory_info().expect("memory info ex");
        assert!(mem.dedicated_bytes > 0);

        let sensors = gpu.thermal_settings().expect("thermal settings");
        assert!(!sensors.is_empty());
        assert!(sensors[0].current_temp_c > 0 && sensors[0].current_temp_c < 120);

        let pstate = gpu.current_pstate().expect("current P-state");
        assert!(pstate <= 16);
        let _dyn_pstates = gpu.dynamic_pstates().expect("dynamic P-states");
        let clocks = gpu.clock_frequencies(ClockKind::Current).expect("current clocks");
        assert!(clocks.graphics_khz.is_some());
        let _perf_decr = gpu.perf_decrease_info().expect("perf decrease info");

        let logical = nvapi.logical_gpus().expect("logical GPUs");
        assert!(!logical.is_empty());
        assert!(!logical[0].physical_gpus().expect("logical -> physical").is_empty());

        // Displays: query HDR & VRR if any display is attached.
        let displays = nvapi.displays().expect("enumerate displays");
        if let Some(disp) = displays.first() {
            assert!(!disp.name().expect("display name").is_empty());
            match disp.hdr_capabilities() {
                Ok(_) => {}
                Err(Error::Status { status, .. }) if status.is_not_supported() => {}
                Err(e) => panic!("unexpected hdr_capabilities error: {e}"),
            }
            match disp.vrr_info() {
                Ok(_) => {}
                Err(Error::Status { status, .. }) if status.is_not_supported() => {}
                Err(e) => panic!("unexpected vrr_info error: {e}"),
            }
        }

        // G-Sync hardware devices may be absent on consumer desktops.
        let _gsync = nvapi.gsync_devices().expect("enum gsync devices");

        // DRS profile database.
        let drs = nvapi.drs_session().expect("open DRS session");
        assert!(drs.num_profiles().expect("num profiles") > 0);
        let global = drs.global_profile().expect("global profile");
        assert!(!global.name.is_empty());
        assert!(drs.find_profile("__nonexistent_nv_rs_profile__").expect("find").is_none());
    }
}
