//! Windows-only NVAPI FFI session and handle implementations.
//!
//! **What.** Owns `NvAPI_Initialize` / `NvAPI_Unload` inside an `Arc<NvApiSession>` and
//! implements all physical GPU, logical GPU, display, G-Sync, and DRS profile queries.
//! **Why.** Isolates Windows-only `nvapi-sys` calls behind `#[cfg(windows)]` so the rest of the
//! crate compiles and runs T0 tests cleanly on any OS.
//! **How.** Every versioned NVAPI struct is zero-initialized on the stack with its `version`
//! field set from the `*_VER*` constants below (mirrors of the `nvapi.h` macros, built with
//! [`make_version`]); enumeration loops terminate on `NVAPI_END_ENUMERATION`.
//! **Used by** [`crate::NvApi`] and its wrapper handles.

use std::ffi::c_char;
use std::mem::MaybeUninit;
use std::sync::Arc;

use crate::error::{Result, Status, check};
use crate::sys;
use crate::types::{
    ArchInfo, ClockFrequencies, ClockKind, DriverVersion, DrsProfileInfo, DynamicPstates,
    GSyncCapabilities, GSyncStatus, GpuMemoryInfo, HdrCapabilities, PciIdentifiers,
    PerfDecreaseReasons, ThermalSensor, VrrInfo, decode_short_string, decode_utf16_z,
    encode_utf16_z, make_version,
};

// Struct versions, mirroring the `nvapi.h` macros `MAKE_NVAPI_VERSION(<revision struct>, n)`.
// Each is built from the explicit revision struct, not the unversioned typedef.
/// `NV_GPU_UUID_VER1` (`nvapi.h:4639`).
const NV_GPU_UUID_VER1: u32 = make_version::<sys::NV_GPU_UUID_V1>(1);
/// `NV_GPU_ARCH_INFO_VER_2` (`nvapi.h:2917`).
const NV_GPU_ARCH_INFO_VER_2: u32 = make_version::<sys::NV_GPU_ARCH_INFO_V2>(2);
/// `NV_GPU_MEMORY_INFO_EX_VER_1` (`nvapi_lite_common.h:567`).
const NV_GPU_MEMORY_INFO_EX_VER_1: u32 = make_version::<sys::NV_GPU_MEMORY_INFO_EX_V1>(1);
/// `NV_GPU_THERMAL_SETTINGS_VER_2` (`nvapi.h:5410`).
const NV_GPU_THERMAL_SETTINGS_VER_2: u32 = make_version::<sys::NV_GPU_THERMAL_SETTINGS_V2>(2);
/// `NV_GPU_DYNAMIC_PSTATES_INFO_EX_VER` (`nvapi.h:5293`, unversioned struct).
const NV_GPU_DYNAMIC_PSTATES_INFO_EX_VER: u32 =
    make_version::<sys::NV_GPU_DYNAMIC_PSTATES_INFO_EX>(1);
/// `NV_GPU_CLOCK_FREQUENCIES_VER_3` (`nvapi.h:5504`): version 3 reuses the `_V2` struct.
const NV_GPU_CLOCK_FREQUENCIES_VER_3: u32 = make_version::<sys::NV_GPU_CLOCK_FREQUENCIES_V2>(3);
/// `NV_HDR_CAPABILITIES_VER3` (`nvapi.h:8111`).
const NV_HDR_CAPABILITIES_VER3: u32 = make_version::<sys::NV_HDR_CAPABILITIES_V3>(3);
/// `NV_GET_VRR_INFO_VER1` (`nvapi.h:9710`).
const NV_GET_VRR_INFO_VER1: u32 = make_version::<sys::NV_GET_VRR_INFO_V1>(1);
/// `NV_GSYNC_CAPABILITIES_VER3` (`nvapi.h:10838`).
const NV_GSYNC_CAPABILITIES_VER3: u32 = make_version::<sys::NV_GSYNC_CAPABILITIES_V3>(3);
/// `NV_GSYNC_STATUS_VER` (`nvapi.h:11181`, unversioned struct).
const NV_GSYNC_STATUS_VER: u32 = make_version::<sys::NV_GSYNC_STATUS>(1);
/// `NVDRS_PROFILE_VER1` (`nvapi.h:24718`).
const NVDRS_PROFILE_VER1: u32 = make_version::<sys::NVDRS_PROFILE_V1>(1);

// The call sites fill the unversioned typedefs. Pin each typedef to the revision its version
// constant was built from: an SDK that retypedefs one to a newer revision fails to compile here
// instead of returning `NVAPI_INCOMPATIBLE_STRUCT_VERSION` at run time.
const _: fn(sys::NV_GPU_UUID) -> sys::NV_GPU_UUID_V1 = std::convert::identity;
const _: fn(sys::NV_GPU_ARCH_INFO) -> sys::NV_GPU_ARCH_INFO_V2 = std::convert::identity;
const _: fn(sys::NV_GPU_MEMORY_INFO_EX) -> sys::NV_GPU_MEMORY_INFO_EX_V1 = std::convert::identity;
const _: fn(sys::NV_GPU_THERMAL_SETTINGS) -> sys::NV_GPU_THERMAL_SETTINGS_V2 =
    std::convert::identity;
const _: fn(sys::NV_GPU_CLOCK_FREQUENCIES) -> sys::NV_GPU_CLOCK_FREQUENCIES_V2 =
    std::convert::identity;
const _: fn(sys::NV_HDR_CAPABILITIES) -> sys::NV_HDR_CAPABILITIES_V3 = std::convert::identity;
const _: fn(sys::NV_GET_VRR_INFO) -> sys::NV_GET_VRR_INFO_V1 = std::convert::identity;
const _: fn(sys::NV_GSYNC_CAPABILITIES) -> sys::NV_GSYNC_CAPABILITIES_V3 = std::convert::identity;
const _: fn(sys::NVDRS_PROFILE) -> sys::NVDRS_PROFILE_V1 = std::convert::identity;

#[derive(Debug)]
pub(crate) struct NvApiSession;

impl Drop for NvApiSession {
    fn drop(&mut self) {
        // SAFETY: `NvApiSession` is only created after `NvAPI_Initialize` succeeds; `NvAPI_Unload`
        // balances that initialization.
        let _ = unsafe { sys::NvAPI_Unload() };
    }
}

pub(crate) fn init_session() -> Result<Arc<NvApiSession>> {
    sys::load()?;
    // SAFETY: `NvAPI_Initialize` takes no parameters and initializes NVAPI's refcounted session.
    let code = unsafe { sys::NvAPI_Initialize() };
    check("NvAPI_Initialize", code)?;
    Ok(Arc::new(NvApiSession))
}

pub(crate) fn interface_version(_session: &Arc<NvApiSession>) -> Result<String> {
    let mut buf: sys::NvAPI_ShortString = [0; 64];
    // SAFETY: `buf` is a 64-byte `NvAPI_ShortString` writable by NVAPI.
    let code = unsafe { sys::NvAPI_GetInterfaceVersionString(buf.as_mut_ptr()) };
    check("NvAPI_GetInterfaceVersionString", code)?;
    Ok(decode_short_string(&buf))
}

pub(crate) fn driver_and_branch_version(_session: &Arc<NvApiSession>) -> Result<DriverVersion> {
    let mut ver = 0u32;
    let mut branch: sys::NvAPI_ShortString = [0; 64];
    // SAFETY: `ver` is a valid out-pointer and `branch` is a 64-byte `NvAPI_ShortString`.
    let code = unsafe { sys::NvAPI_SYS_GetDriverAndBranchVersion(&mut ver, branch.as_mut_ptr()) };
    check("NvAPI_SYS_GetDriverAndBranchVersion", code)?;
    Ok(DriverVersion {
        version: ver,
        branch: decode_short_string(&branch),
    })
}

pub(crate) fn enum_physical_gpus(session: &Arc<NvApiSession>) -> Result<Vec<PhysicalGpu>> {
    let mut handles: [sys::NvPhysicalGpuHandle; sys::NVAPI_MAX_PHYSICAL_GPUS as usize] =
        [std::ptr::null_mut(); sys::NVAPI_MAX_PHYSICAL_GPUS as usize];
    let mut count = 0u32;
    // SAFETY: `handles` has `NVAPI_MAX_PHYSICAL_GPUS` elements and `count` is a valid out-pointer.
    let code = unsafe { sys::NvAPI_EnumPhysicalGPUs(handles.as_mut_ptr(), &mut count) };
    if Status(code).is_device_not_found() {
        return Ok(Vec::new());
    }
    check("NvAPI_EnumPhysicalGPUs", code)?;
    let n = (count as usize).min(handles.len());
    Ok(handles[..n]
        .iter()
        .map(|&handle| PhysicalGpu {
            _session: Arc::clone(session),
            handle,
        })
        .collect())
}

pub(crate) fn enum_tcc_physical_gpus(session: &Arc<NvApiSession>) -> Result<Vec<PhysicalGpu>> {
    let mut handles: [sys::NvPhysicalGpuHandle; sys::NVAPI_MAX_PHYSICAL_GPUS as usize] =
        [std::ptr::null_mut(); sys::NVAPI_MAX_PHYSICAL_GPUS as usize];
    let mut count = 0u32;
    // SAFETY: `handles` has `NVAPI_MAX_PHYSICAL_GPUS` elements and `count` is a valid out-pointer.
    let code = unsafe { sys::NvAPI_EnumTCCPhysicalGPUs(handles.as_mut_ptr(), &mut count) };
    if Status(code).is_device_not_found() {
        return Ok(Vec::new());
    }
    check("NvAPI_EnumTCCPhysicalGPUs", code)?;
    let n = (count as usize).min(handles.len());
    Ok(handles[..n]
        .iter()
        .map(|&handle| PhysicalGpu {
            _session: Arc::clone(session),
            handle,
        })
        .collect())
}

pub(crate) fn enum_logical_gpus(session: &Arc<NvApiSession>) -> Result<Vec<LogicalGpu>> {
    let mut handles: [sys::NvLogicalGpuHandle; sys::NVAPI_MAX_LOGICAL_GPUS as usize] =
        [std::ptr::null_mut(); sys::NVAPI_MAX_LOGICAL_GPUS as usize];
    let mut count = 0u32;
    // SAFETY: `handles` has `NVAPI_MAX_LOGICAL_GPUS` elements and `count` is a valid out-pointer.
    let code = unsafe { sys::NvAPI_EnumLogicalGPUs(handles.as_mut_ptr(), &mut count) };
    if Status(code).is_device_not_found() {
        return Ok(Vec::new());
    }
    check("NvAPI_EnumLogicalGPUs", code)?;
    let n = (count as usize).min(handles.len());
    Ok(handles[..n]
        .iter()
        .map(|&handle| LogicalGpu {
            _session: Arc::clone(session),
            handle,
        })
        .collect())
}

pub(crate) fn enum_displays(session: &Arc<NvApiSession>) -> Result<Vec<Display>> {
    let mut out = Vec::new();
    for idx in 0..sys::NVAPI_MAX_DISPLAYS {
        let mut handle: sys::NvDisplayHandle = std::ptr::null_mut();
        // SAFETY: `handle` is a valid out-pointer and `idx` is bounded by `NVAPI_MAX_DISPLAYS`.
        let code = unsafe { sys::NvAPI_EnumNvidiaDisplayHandle(idx, &mut handle) };
        let status = Status(code);
        if status.is_end_enumeration() || status.is_device_not_found() {
            break;
        }
        check("NvAPI_EnumNvidiaDisplayHandle", code)?;
        out.push(Display {
            _session: Arc::clone(session),
            handle,
        });
    }
    Ok(out)
}

pub(crate) fn enum_gsync_devices(session: &Arc<NvApiSession>) -> Result<Vec<GSyncDevice>> {
    let mut handles: [sys::NvGSyncDeviceHandle; sys::NVAPI_MAX_GSYNC_DEVICES as usize] =
        [std::ptr::null_mut(); sys::NVAPI_MAX_GSYNC_DEVICES as usize];
    let mut count = 0u32;
    // SAFETY: `handles` has `NVAPI_MAX_GSYNC_DEVICES` elements and `count` is a valid out-pointer.
    let code = unsafe { sys::NvAPI_GSync_EnumSyncDevices(handles.as_mut_ptr(), &mut count) };
    if Status(code).is_device_not_found() {
        return Ok(Vec::new());
    }
    check("NvAPI_GSync_EnumSyncDevices", code)?;
    let n = (count as usize).min(handles.len());
    Ok(handles[..n]
        .iter()
        .map(|&handle| GSyncDevice {
            _session: Arc::clone(session),
            handle,
        })
        .collect())
}

#[derive(Clone, Debug)]
pub(crate) struct PhysicalGpu {
    pub(crate) _session: Arc<NvApiSession>,
    pub(crate) handle: sys::NvPhysicalGpuHandle,
}

// SAFETY: NVAPI physical GPU handles are opaque driver identifiers safe across threads while NVAPI
// is initialized (pinned by `_session`).
unsafe impl Send for PhysicalGpu {}
// SAFETY: Read-only NVAPI queries on a `PhysicalGpu` are safe to share across threads.
unsafe impl Sync for PhysicalGpu {}

impl PhysicalGpu {
    pub(crate) fn full_name(&self) -> Result<String> {
        let mut buf: sys::NvAPI_ShortString = [0; 64];
        // SAFETY: `self.handle` is valid and `buf` is a 64-byte `NvAPI_ShortString`.
        let code = unsafe { sys::NvAPI_GPU_GetFullName(self.handle, buf.as_mut_ptr()) };
        check("NvAPI_GPU_GetFullName", code)?;
        Ok(decode_short_string(&buf))
    }

    pub(crate) fn uuid(&self) -> Result<[u8; 16]> {
        let mut raw = MaybeUninit::<sys::NV_GPU_UUID>::zeroed();
        // SAFETY: `raw` is zeroed and its `version` field is set to `NV_GPU_UUID_VER1`.
        unsafe {
            (*raw.as_mut_ptr()).version = NV_GPU_UUID_VER1;
        }
        // SAFETY: `self.handle` is valid and `raw` points to a properly versioned `NV_GPU_UUID`.
        let code = unsafe { sys::NvAPI_GPU_GetUUID(self.handle, raw.as_mut_ptr()) };
        check("NvAPI_GPU_GetUUID", code)?;
        // SAFETY: `NvAPI_GPU_GetUUID` succeeded and initialized `raw`.
        let raw = unsafe { raw.assume_init() };
        Ok(raw.uuid)
    }

    pub(crate) fn pci_identifiers(&self) -> Result<PciIdentifiers> {
        let mut device_id = 0u32;
        let mut subsystem_id = 0u32;
        let mut revision_id = 0u32;
        let mut ext_device_id = 0u32;
        // SAFETY: `self.handle` is valid and all four out-pointers are valid.
        let code = unsafe {
            sys::NvAPI_GPU_GetPCIIdentifiers(
                self.handle,
                &mut device_id,
                &mut subsystem_id,
                &mut revision_id,
                &mut ext_device_id,
            )
        };
        check("NvAPI_GPU_GetPCIIdentifiers", code)?;
        Ok(PciIdentifiers {
            device_id,
            subsystem_id,
            revision_id,
            ext_device_id,
        })
    }

    pub(crate) fn bus_id(&self) -> Result<u32> {
        let mut bus = 0u32;
        // SAFETY: `self.handle` is valid and `bus` is a valid out-pointer.
        let code = unsafe { sys::NvAPI_GPU_GetBusId(self.handle, &mut bus) };
        check("NvAPI_GPU_GetBusId", code)?;
        Ok(bus)
    }

    pub(crate) fn bus_slot_id(&self) -> Result<u32> {
        let mut slot = 0u32;
        // SAFETY: `self.handle` is valid and `slot` is a valid out-pointer.
        let code = unsafe { sys::NvAPI_GPU_GetBusSlotId(self.handle, &mut slot) };
        check("NvAPI_GPU_GetBusSlotId", code)?;
        Ok(slot)
    }

    pub(crate) fn arch_info(&self) -> Result<ArchInfo> {
        let mut raw = MaybeUninit::<sys::NV_GPU_ARCH_INFO>::zeroed();
        // SAFETY: `raw` is zeroed and its `version` field is set to `NV_GPU_ARCH_INFO_VER_2`.
        unsafe {
            (*raw.as_mut_ptr()).version = NV_GPU_ARCH_INFO_VER_2;
        }
        // SAFETY: `self.handle` is valid and `raw` points to a versioned `NV_GPU_ARCH_INFO`.
        let code = unsafe { sys::NvAPI_GPU_GetArchInfo(self.handle, raw.as_mut_ptr()) };
        check("NvAPI_GPU_GetArchInfo", code)?;
        // SAFETY: `NvAPI_GPU_GetArchInfo` succeeded and populated the union fields.
        let raw = unsafe { raw.assume_init() };
        // SAFETY: `architecture`, `implementation`, and `revision` are the `NvU32` variants of
        // the anonymous unions in `NV_GPU_ARCH_INFO_V2`.
        unsafe {
            Ok(ArchInfo {
                architecture: raw.__bindgen_anon_1.architecture,
                implementation: raw.__bindgen_anon_2.implementation,
                revision: raw.__bindgen_anon_3.revision,
            })
        }
    }

    pub(crate) fn core_count(&self) -> Result<u32> {
        let mut count = 0u32;
        // SAFETY: `self.handle` is valid and `count` is a valid out-pointer.
        let code = unsafe { sys::NvAPI_GPU_GetGpuCoreCount(self.handle, &mut count) };
        check("NvAPI_GPU_GetGpuCoreCount", code)?;
        Ok(count)
    }

    pub(crate) fn vbios_version(&self) -> Result<String> {
        let mut buf: sys::NvAPI_ShortString = [0; 64];
        // SAFETY: `self.handle` is valid and `buf` is a 64-byte `NvAPI_ShortString`.
        let code = unsafe { sys::NvAPI_GPU_GetVbiosVersionString(self.handle, buf.as_mut_ptr()) };
        check("NvAPI_GPU_GetVbiosVersionString", code)?;
        Ok(decode_short_string(&buf))
    }

    pub(crate) fn memory_info(&self) -> Result<GpuMemoryInfo> {
        let mut raw = MaybeUninit::<sys::NV_GPU_MEMORY_INFO_EX>::zeroed();
        // SAFETY: `raw` is zeroed and `version` is set to `NV_GPU_MEMORY_INFO_EX_VER_1`.
        unsafe {
            (*raw.as_mut_ptr()).version = NV_GPU_MEMORY_INFO_EX_VER_1;
        }
        // SAFETY: `self.handle` is valid and `raw` points to a versioned `NV_GPU_MEMORY_INFO_EX`.
        let code = unsafe { sys::NvAPI_GPU_GetMemoryInfoEx(self.handle, raw.as_mut_ptr()) };
        check("NvAPI_GPU_GetMemoryInfoEx", code)?;
        // SAFETY: `NvAPI_GPU_GetMemoryInfoEx` succeeded and initialized `raw`.
        let raw = unsafe { raw.assume_init() };
        Ok(GpuMemoryInfo {
            dedicated_bytes: raw.dedicatedVideoMemory,
            available_dedicated_bytes: raw.availableDedicatedVideoMemory,
            cur_available_dedicated_bytes: raw.curAvailableDedicatedVideoMemory,
            system_bytes: raw.systemVideoMemory,
            shared_system_bytes: raw.sharedSystemMemory,
        })
    }

    pub(crate) fn thermal_settings(&self) -> Result<Vec<ThermalSensor>> {
        let mut raw = MaybeUninit::<sys::NV_GPU_THERMAL_SETTINGS>::zeroed();
        // SAFETY: `raw` is zeroed and `version` is set to `NV_GPU_THERMAL_SETTINGS_VER_2`.
        unsafe {
            (*raw.as_mut_ptr()).version = NV_GPU_THERMAL_SETTINGS_VER_2;
        }
        // SAFETY: `self.handle` is valid and `raw` points to a versioned `NV_GPU_THERMAL_SETTINGS`.
        let code = unsafe {
            sys::NvAPI_GPU_GetThermalSettings(
                self.handle,
                sys::NV_THERMAL_TARGET_NVAPI_THERMAL_TARGET_ALL as u32,
                raw.as_mut_ptr(),
            )
        };
        check("NvAPI_GPU_GetThermalSettings", code)?;
        // SAFETY: `NvAPI_GPU_GetThermalSettings` succeeded and initialized `raw`.
        let raw = unsafe { raw.assume_init() };
        let count = (raw.count as usize).min(raw.sensor.len());
        Ok(raw.sensor[..count]
            .iter()
            .map(|s| ThermalSensor {
                controller: s.controller,
                target: s.target,
                current_temp_c: s.currentTemp,
                default_min_c: s.defaultMinTemp,
                default_max_c: s.defaultMaxTemp,
            })
            .collect())
    }

    pub(crate) fn tach_reading(&self) -> Result<u32> {
        let mut rpm = 0u32;
        // SAFETY: `self.handle` is valid and `rpm` is a valid out-pointer.
        let code = unsafe { sys::NvAPI_GPU_GetTachReading(self.handle, &mut rpm) };
        check("NvAPI_GPU_GetTachReading", code)?;
        Ok(rpm)
    }

    pub(crate) fn current_pstate(&self) -> Result<u32> {
        let mut pstate: sys::NV_GPU_PERF_PSTATE_ID = 0;
        // SAFETY: `self.handle` is valid and `pstate` is a valid out-pointer.
        let code = unsafe { sys::NvAPI_GPU_GetCurrentPstate(self.handle, &mut pstate) };
        check("NvAPI_GPU_GetCurrentPstate", code)?;
        Ok(pstate as u32)
    }

    pub(crate) fn dynamic_pstates(&self) -> Result<DynamicPstates> {
        let mut raw = MaybeUninit::<sys::NV_GPU_DYNAMIC_PSTATES_INFO_EX>::zeroed();
        // SAFETY: `raw` is zeroed and `version` is set to `NV_GPU_DYNAMIC_PSTATES_INFO_EX_VER`.
        unsafe {
            (*raw.as_mut_ptr()).version = NV_GPU_DYNAMIC_PSTATES_INFO_EX_VER;
        }
        // SAFETY: `self.handle` is valid and `raw` points to a versioned `NV_GPU_DYNAMIC_PSTATES_INFO_EX`.
        let code = unsafe { sys::NvAPI_GPU_GetDynamicPstatesInfoEx(self.handle, raw.as_mut_ptr()) };
        check("NvAPI_GPU_GetDynamicPstatesInfoEx", code)?;
        // SAFETY: `NvAPI_GPU_GetDynamicPstatesInfoEx` succeeded and initialized `raw`.
        let raw = unsafe { raw.assume_init() };
        let read_domain = |idx: usize| -> Option<u32> {
            let entry = &raw.utilization[idx];
            (entry.bIsPresent() != 0).then_some(entry.percentage)
        };
        Ok(DynamicPstates {
            enabled: (raw.flags & 1) != 0,
            gpu: read_domain(0),
            fb: read_domain(1),
            vid: read_domain(2),
            bus: read_domain(3),
        })
    }

    pub(crate) fn clock_frequencies(&self, kind: ClockKind) -> Result<ClockFrequencies> {
        let mut raw = MaybeUninit::<sys::NV_GPU_CLOCK_FREQUENCIES>::zeroed();
        let clock_type = match kind {
            ClockKind::Current => {
                sys::NV_GPU_CLOCK_FREQUENCIES_CLOCK_TYPE_NV_GPU_CLOCK_FREQUENCIES_CURRENT_FREQ
            }
            ClockKind::Base => {
                sys::NV_GPU_CLOCK_FREQUENCIES_CLOCK_TYPE_NV_GPU_CLOCK_FREQUENCIES_BASE_CLOCK
            }
            ClockKind::Boost => {
                sys::NV_GPU_CLOCK_FREQUENCIES_CLOCK_TYPE_NV_GPU_CLOCK_FREQUENCIES_BOOST_CLOCK
            }
        };
        // SAFETY: `raw` is zeroed; we initialize `version` (V3) and `ClockType` before the call.
        unsafe {
            let ptr = raw.as_mut_ptr();
            (*ptr).version = NV_GPU_CLOCK_FREQUENCIES_VER_3;
            (*ptr).set_ClockType(clock_type as u32);
        }
        // SAFETY: `self.handle` is valid and `raw` points to a versioned `NV_GPU_CLOCK_FREQUENCIES`.
        let code = unsafe { sys::NvAPI_GPU_GetAllClockFrequencies(self.handle, raw.as_mut_ptr()) };
        check("NvAPI_GPU_GetAllClockFrequencies", code)?;
        // SAFETY: `NvAPI_GPU_GetAllClockFrequencies` succeeded and initialized `raw`.
        let raw = unsafe { raw.assume_init() };
        let read_clk = |id: i32| -> Option<u32> {
            let entry = &raw.domain[id as usize];
            (entry.bIsPresent() != 0).then_some(entry.frequency)
        };
        Ok(ClockFrequencies {
            graphics_khz: read_clk(sys::_NV_GPU_PUBLIC_CLOCK_ID_NVAPI_GPU_PUBLIC_CLOCK_GRAPHICS),
            memory_khz: read_clk(sys::_NV_GPU_PUBLIC_CLOCK_ID_NVAPI_GPU_PUBLIC_CLOCK_MEMORY),
            processor_khz: read_clk(sys::_NV_GPU_PUBLIC_CLOCK_ID_NVAPI_GPU_PUBLIC_CLOCK_PROCESSOR),
            video_khz: read_clk(sys::_NV_GPU_PUBLIC_CLOCK_ID_NVAPI_GPU_PUBLIC_CLOCK_VIDEO),
        })
    }

    pub(crate) fn perf_decrease_info(&self) -> Result<PerfDecreaseReasons> {
        let mut mask = 0u32;
        // SAFETY: `self.handle` is valid and `mask` is a valid out-pointer.
        let code = unsafe { sys::NvAPI_GPU_GetPerfDecreaseInfo(self.handle, &mut mask) };
        check("NvAPI_GPU_GetPerfDecreaseInfo", code)?;
        Ok(PerfDecreaseReasons(mask))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct LogicalGpu {
    pub(crate) _session: Arc<NvApiSession>,
    pub(crate) handle: sys::NvLogicalGpuHandle,
}

// SAFETY: NVAPI logical GPU handles are opaque driver identifiers safe across threads while NVAPI
// is initialized (pinned by `_session`).
unsafe impl Send for LogicalGpu {}
// SAFETY: Read-only NVAPI queries on a `LogicalGpu` are safe to share across threads.
unsafe impl Sync for LogicalGpu {}

impl LogicalGpu {
    pub(crate) fn physical_gpus(&self) -> Result<Vec<PhysicalGpu>> {
        let mut handles: [sys::NvPhysicalGpuHandle; sys::NVAPI_MAX_PHYSICAL_GPUS as usize] =
            [std::ptr::null_mut(); sys::NVAPI_MAX_PHYSICAL_GPUS as usize];
        let mut count = 0u32;
        // SAFETY: `self.handle` is valid and `handles`/`count` are valid out-buffers.
        let code = unsafe {
            sys::NvAPI_GetPhysicalGPUsFromLogicalGPU(self.handle, handles.as_mut_ptr(), &mut count)
        };
        check("NvAPI_GetPhysicalGPUsFromLogicalGPU", code)?;
        let n = (count as usize).min(handles.len());
        Ok(handles[..n]
            .iter()
            .map(|&handle| PhysicalGpu {
                _session: Arc::clone(&self._session),
                handle,
            })
            .collect())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Display {
    pub(crate) _session: Arc<NvApiSession>,
    pub(crate) handle: sys::NvDisplayHandle,
}

// SAFETY: NVAPI display handles are opaque driver identifiers safe across threads while NVAPI is
// initialized (pinned by `_session`).
unsafe impl Send for Display {}
// SAFETY: Read-only NVAPI queries on a `Display` are safe to share across threads.
unsafe impl Sync for Display {}

impl Display {
    pub(crate) fn name(&self) -> Result<String> {
        let mut buf: sys::NvAPI_ShortString = [0; 64];
        // SAFETY: `self.handle` is valid and `buf` is a 64-byte `NvAPI_ShortString`.
        let code =
            unsafe { sys::NvAPI_GetAssociatedNvidiaDisplayName(self.handle, buf.as_mut_ptr()) };
        check("NvAPI_GetAssociatedNvidiaDisplayName", code)?;
        Ok(decode_short_string(&buf))
    }

    pub(crate) fn display_id(&self) -> Result<u32> {
        let mut buf: sys::NvAPI_ShortString = [0; 64];
        // SAFETY: `self.handle` is valid and `buf` is a 64-byte `NvAPI_ShortString`.
        let code =
            unsafe { sys::NvAPI_GetAssociatedNvidiaDisplayName(self.handle, buf.as_mut_ptr()) };
        check("NvAPI_GetAssociatedNvidiaDisplayName", code)?;
        let mut id = 0u32;
        // SAFETY: `buf` is NUL-terminated by `NvAPI_GetAssociatedNvidiaDisplayName` and `id` is a
        // valid out-pointer.
        let code = unsafe {
            sys::NvAPI_DISP_GetDisplayIdByDisplayName(buf.as_ptr().cast::<c_char>(), &mut id)
        };
        check("NvAPI_DISP_GetDisplayIdByDisplayName", code)?;
        Ok(id)
    }

    pub(crate) fn hdr_capabilities(&self) -> Result<HdrCapabilities> {
        let display_id = self.display_id()?;
        let mut raw = MaybeUninit::<sys::NV_HDR_CAPABILITIES>::zeroed();
        // SAFETY: `raw` is zeroed and `version` is set to `NV_HDR_CAPABILITIES_VER3`.
        unsafe {
            (*raw.as_mut_ptr()).version = NV_HDR_CAPABILITIES_VER3;
        }
        // SAFETY: `display_id` is valid and `raw` points to a versioned `NV_HDR_CAPABILITIES`.
        let code = unsafe { sys::NvAPI_Disp_GetHdrCapabilities(display_id, raw.as_mut_ptr()) };
        check("NvAPI_Disp_GetHdrCapabilities", code)?;
        // SAFETY: `NvAPI_Disp_GetHdrCapabilities` succeeded and initialized `raw`.
        let raw = unsafe { raw.assume_init() };
        Ok(HdrCapabilities {
            st2084_eotf: raw.isST2084EotfSupported() != 0,
            traditional_hdr_gamma: raw.isTraditionalHdrGammaSupported() != 0,
            edr: raw.isEdrSupported() != 0,
            dolby_vision: raw.isDolbyVisionSupported() != 0,
            hdr10_plus: raw.isHdr10PlusSupported() != 0,
            hdr10_plus_gaming: raw.isHdr10PlusGamingSupported() != 0,
            nvidia_certified: raw.isNvidiaCertifiedDisplay() != 0,
            max_luminance_nits: raw.display_data.desired_content_max_luminance,
            min_luminance_ten_thousandths: raw.display_data.desired_content_min_luminance,
            max_fall_nits: raw.display_data.desired_content_max_frame_average_luminance,
        })
    }

    pub(crate) fn vrr_info(&self) -> Result<VrrInfo> {
        let display_id = self.display_id()?;
        let mut raw = MaybeUninit::<sys::NV_GET_VRR_INFO>::zeroed();
        // SAFETY: `raw` is zeroed and `version` is set to `NV_GET_VRR_INFO_VER1`.
        unsafe {
            (*raw.as_mut_ptr()).version = NV_GET_VRR_INFO_VER1;
        }
        // SAFETY: `display_id` is valid and `raw` points to a versioned `NV_GET_VRR_INFO`.
        let code = unsafe { sys::NvAPI_Disp_GetVRRInfo(display_id, raw.as_mut_ptr()) };
        check("NvAPI_Disp_GetVRRInfo", code)?;
        // SAFETY: `NvAPI_Disp_GetVRRInfo` succeeded and initialized `raw`.
        let raw = unsafe { raw.assume_init() };
        Ok(VrrInfo {
            enabled: raw.bIsVRREnabled() != 0,
            possible: raw.bIsVRRPossible() != 0,
            requested: raw.bIsVRRRequested() != 0,
            indicator_enabled: raw.bIsVRRIndicatorEnabled() != 0,
            in_vrr_mode: raw.bIsDisplayInVRRMode() != 0,
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct GSyncDevice {
    pub(crate) _session: Arc<NvApiSession>,
    pub(crate) handle: sys::NvGSyncDeviceHandle,
}

// SAFETY: NVAPI G-Sync device handles are opaque driver identifiers safe across threads while NVAPI
// is initialized (pinned by `_session`).
unsafe impl Send for GSyncDevice {}
// SAFETY: Read-only NVAPI queries on a `GSyncDevice` are safe to share across threads.
unsafe impl Sync for GSyncDevice {}

impl GSyncDevice {
    pub(crate) fn capabilities(&self) -> Result<GSyncCapabilities> {
        let mut raw = MaybeUninit::<sys::NV_GSYNC_CAPABILITIES>::zeroed();
        // SAFETY: `raw` is zeroed and `version` is set to `NV_GSYNC_CAPABILITIES_VER3`.
        unsafe {
            (*raw.as_mut_ptr()).version = NV_GSYNC_CAPABILITIES_VER3;
        }
        // SAFETY: `self.handle` is valid and `raw` points to a versioned `NV_GSYNC_CAPABILITIES`.
        let code = unsafe { sys::NvAPI_GSync_QueryCapabilities(self.handle, raw.as_mut_ptr()) };
        check("NvAPI_GSync_QueryCapabilities", code)?;
        // SAFETY: `NvAPI_GSync_QueryCapabilities` succeeded and initialized `raw`.
        let raw = unsafe { raw.assume_init() };
        Ok(GSyncCapabilities {
            board_id: raw.boardId,
            revision: raw.revision,
            cap_flags: raw.capFlags,
        })
    }

    pub(crate) fn sync_status(&self, gpu: &PhysicalGpu) -> Result<GSyncStatus> {
        let mut raw = MaybeUninit::<sys::NV_GSYNC_STATUS>::zeroed();
        // SAFETY: `raw` is zeroed and `version` is set to `NV_GSYNC_STATUS_VER`.
        unsafe {
            (*raw.as_mut_ptr()).version = NV_GSYNC_STATUS_VER;
        }
        // SAFETY: `self.handle` and `gpu.handle` are valid; `raw` is a versioned `NV_GSYNC_STATUS`.
        let code =
            unsafe { sys::NvAPI_GSync_GetSyncStatus(self.handle, gpu.handle, raw.as_mut_ptr()) };
        check("NvAPI_GSync_GetSyncStatus", code)?;
        // SAFETY: `NvAPI_GSync_GetSyncStatus` succeeded and initialized `raw`.
        let raw = unsafe { raw.assume_init() };
        Ok(GSyncStatus {
            is_synced: raw.bIsSynced != 0,
            is_stereo_synced: raw.bIsStereoSynced != 0,
            is_sync_signal_available: raw.bIsSyncSignalAvailable != 0,
        })
    }
}

#[derive(Debug)]
pub(crate) struct DrsSession {
    _session: Arc<NvApiSession>,
    handle: sys::NvDRSSessionHandle,
}

impl Drop for DrsSession {
    fn drop(&mut self) {
        // SAFETY: `self.handle` was created by `NvAPI_DRS_CreateSession` and has not been
        // destroyed yet.
        let _ = unsafe { sys::NvAPI_DRS_DestroySession(self.handle) };
    }
}

impl DrsSession {
    pub(crate) fn open(session: &Arc<NvApiSession>) -> Result<Self> {
        let mut handle: sys::NvDRSSessionHandle = std::ptr::null_mut();
        // SAFETY: `handle` is a valid out-pointer.
        let code = unsafe { sys::NvAPI_DRS_CreateSession(&mut handle) };
        check("NvAPI_DRS_CreateSession", code)?;
        let drs = Self {
            _session: Arc::clone(session),
            handle,
        };
        // SAFETY: `drs.handle` is a freshly created DRS session handle.
        let code = unsafe { sys::NvAPI_DRS_LoadSettings(drs.handle) };
        check("NvAPI_DRS_LoadSettings", code)?;
        Ok(drs)
    }

    pub(crate) fn num_profiles(&self) -> Result<u32> {
        let mut count = 0u32;
        // SAFETY: `self.handle` is valid and `count` is a valid out-pointer.
        let code = unsafe { sys::NvAPI_DRS_GetNumProfiles(self.handle, &mut count) };
        check("NvAPI_DRS_GetNumProfiles", code)?;
        Ok(count)
    }

    pub(crate) fn global_profile(&self) -> Result<DrsProfileInfo> {
        let mut profile: sys::NvDRSProfileHandle = std::ptr::null_mut();
        // SAFETY: `self.handle` is valid and `profile` is a valid out-pointer.
        let code = unsafe { sys::NvAPI_DRS_GetCurrentGlobalProfile(self.handle, &mut profile) };
        check("NvAPI_DRS_GetCurrentGlobalProfile", code)?;
        self.profile_info(profile)
    }

    pub(crate) fn profile_names(&self) -> Result<Vec<String>> {
        let total = self.num_profiles()?;
        let mut out = Vec::with_capacity(total as usize);
        for idx in 0..total {
            let mut profile: sys::NvDRSProfileHandle = std::ptr::null_mut();
            // SAFETY: `self.handle` is valid and `profile` is a valid out-pointer.
            let code = unsafe { sys::NvAPI_DRS_EnumProfiles(self.handle, idx, &mut profile) };
            if Status(code).is_end_enumeration() {
                break;
            }
            check("NvAPI_DRS_EnumProfiles", code)?;
            out.push(self.profile_info(profile)?.name);
        }
        Ok(out)
    }

    pub(crate) fn find_profile(&self, name: &str) -> Result<Option<DrsProfileInfo>> {
        let mut name_buf: sys::NvAPI_UnicodeString = [0; sys::NVAPI_UNICODE_STRING_MAX as usize];
        encode_utf16_z(name, &mut name_buf)?;
        let mut profile: sys::NvDRSProfileHandle = std::ptr::null_mut();
        // SAFETY: `self.handle` is valid, `name_buf` is NUL-terminated UTF-16, and `profile` is a
        // valid out-pointer.
        let code = unsafe {
            sys::NvAPI_DRS_FindProfileByName(self.handle, name_buf.as_mut_ptr(), &mut profile)
        };
        if Status(code).is_profile_not_found() {
            return Ok(None);
        }
        check("NvAPI_DRS_FindProfileByName", code)?;
        self.profile_info(profile).map(Some)
    }

    fn profile_info(&self, profile: sys::NvDRSProfileHandle) -> Result<DrsProfileInfo> {
        let mut raw = MaybeUninit::<sys::NVDRS_PROFILE>::zeroed();
        // SAFETY: `raw` is zeroed and `version` is set to `NVDRS_PROFILE_VER1`.
        unsafe {
            (*raw.as_mut_ptr()).version = NVDRS_PROFILE_VER1;
        }
        // SAFETY: `self.handle` and `profile` are valid; `raw` is a versioned `NVDRS_PROFILE`.
        let code = unsafe { sys::NvAPI_DRS_GetProfileInfo(self.handle, profile, raw.as_mut_ptr()) };
        check("NvAPI_DRS_GetProfileInfo", code)?;
        // SAFETY: `NvAPI_DRS_GetProfileInfo` succeeded and initialized `raw`.
        let raw = unsafe { raw.assume_init() };
        Ok(DrsProfileInfo {
            name: decode_utf16_z(&raw.profileName),
            is_predefined: raw.isPredefined != 0,
            num_apps: raw.numOfApps,
            num_settings: raw.numOfSettings,
        })
    }
}
