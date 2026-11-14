//! Safe NVML session lifecycle, system queries, and device enumeration.
//!
//! **What.** [`Nvml`] is a cheaply cloneable `Arc`-backed handle to an initialized NVML session.
//! Dropping the last [`Nvml`] and [`Device`] calls `nvmlShutdown` exactly once.
//! **Why.** NVML uses process-global reference counting internally, so pairing each `nvmlInit_v2`
//! with one `nvmlShutdown` on the `Arc` drop boundary prevents mismatched init/shutdown calls.
//! **How.** [`NvmlSession`] calls `nvml_sys::load()` then `nvmlInit_v2` / `nvmlInitWithFlags`, and
//! `Drop` calls `nvmlShutdown`.
//! **Used by** [`crate::Device`] and downstream telemetry consumers.

use std::ffi::c_char;
use std::sync::Arc;

use crate::device::Device;
use crate::error::{Result, check};
use crate::sys;
use crate::types::{PciBusId, decode_c_buf, encode_c_buf, struct_version, validate_uuid};

/// Internal RAII owner of one `nvmlInit_v2` / `nvmlInitWithFlags` reference.
#[derive(Debug)]
pub(crate) struct NvmlSession;

impl Drop for NvmlSession {
    fn drop(&mut self) {
        // SAFETY: `NvmlSession` is only constructed after a successful `nvmlInit_v2` or
        // `nvmlInitWithFlags` call; `nvmlShutdown` balances that single initialization.
        let _ = unsafe { sys::nvmlShutdown() };
    }
}

/// Handle to an initialized NVML library session.
///
/// Cloning [`Nvml`] increments the `Arc` reference count without calling `nvmlInit_v2` again;
/// `nvmlShutdown` runs once when the last [`Nvml`] and all [`Device`]s derived from it are dropped.
#[derive(Clone, Debug)]
pub struct Nvml {
    session: Arc<NvmlSession>,
}

impl Nvml {
    /// Loads NVML and initializes a session (`nvmlInit_v2`).
    pub fn init() -> Result<Self> {
        sys::load()?;
        // SAFETY: `nvmlInit_v2` takes no pointers and initializes NVML's refcounted session.
        let code = unsafe { sys::nvmlInit_v2() };
        check("nvmlInit_v2", code)?;
        Ok(Self {
            session: Arc::new(NvmlSession),
        })
    }

    /// Loads NVML and initializes a session with explicit `NVML_INIT_FLAG_*` flags
    /// (`nvmlInitWithFlags`).
    pub fn init_with_flags(flags: u32) -> Result<Self> {
        sys::load()?;
        // SAFETY: `nvmlInitWithFlags` takes a bitmask of `NVML_INIT_FLAG_*` and initializes NVML.
        let code = unsafe { sys::nvmlInitWithFlags(flags) };
        check("nvmlInitWithFlags", code)?;
        Ok(Self {
            session: Arc::new(NvmlSession),
        })
    }

    /// Queries the display driver version string (`nvmlSystemGetDriverVersion`).
    pub fn driver_version(&self) -> Result<String> {
        let mut buf = [0 as c_char; sys::NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE as usize];
        // SAFETY: NVML is initialized by `self.session` and `buf` is `buf.len()` writable bytes.
        let code = unsafe { sys::nvmlSystemGetDriverVersion(buf.as_mut_ptr(), buf.len() as u32) };
        check("nvmlSystemGetDriverVersion", code)?;
        Ok(decode_c_buf(&buf))
    }

    /// Queries the NVML library version string (`nvmlSystemGetNVMLVersion`).
    pub fn nvml_version(&self) -> Result<String> {
        let mut buf = [0 as c_char; sys::NVML_SYSTEM_NVML_VERSION_BUFFER_SIZE as usize];
        // SAFETY: NVML is initialized by `self.session` and `buf` is `buf.len()` writable bytes.
        let code = unsafe { sys::nvmlSystemGetNVMLVersion(buf.as_mut_ptr(), buf.len() as u32) };
        check("nvmlSystemGetNVMLVersion", code)?;
        Ok(decode_c_buf(&buf))
    }

    /// Queries the CUDA driver version as `(major, minor)` (`nvmlSystemGetCudaDriverVersion_v2`).
    pub fn cuda_driver_version(&self) -> Result<(i32, i32)> {
        let mut raw = 0i32;
        // SAFETY: NVML is initialized and `raw` is a valid out-pointer.
        let code = unsafe { sys::nvmlSystemGetCudaDriverVersion_v2(&mut raw) };
        check("nvmlSystemGetCudaDriverVersion_v2", code)?;
        Ok((raw / 1000, (raw % 1000) / 10))
    }

    /// Queries the NVIDIA driver branch string (`nvmlSystemGetDriverBranch`).
    pub fn driver_branch(&self) -> Result<String> {
        let mut info = sys::nvmlSystemDriverBranchInfo_t {
            version: struct_version::<sys::nvmlSystemDriverBranchInfo_t>(1),
            branch: [0; sys::NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE as usize],
        };
        // SAFETY: NVML is initialized and `info.version` is set to `nvmlSystemDriverBranchInfo_v1`.
        let code = unsafe {
            sys::nvmlSystemGetDriverBranch(&mut info, sys::NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE)
        };
        check("nvmlSystemGetDriverBranch", code)?;
        Ok(decode_c_buf(&info.branch))
    }

    /// Queries the number of accessible NVIDIA GPUs (`nvmlDeviceGetCount_v2`).
    pub fn device_count(&self) -> Result<u32> {
        let mut count = 0u32;
        // SAFETY: NVML is initialized and `count` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetCount_v2(&mut count) };
        check("nvmlDeviceGetCount_v2", code)?;
        Ok(count)
    }

    /// Acquires a [`Device`] handle by zero-based index (`nvmlDeviceGetHandleByIndex_v2`).
    pub fn device_by_index(&self, index: u32) -> Result<Device> {
        let mut raw: sys::nvmlDevice_t = std::ptr::null_mut();
        // SAFETY: NVML is initialized and `raw` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetHandleByIndex_v2(index, &mut raw) };
        check("nvmlDeviceGetHandleByIndex_v2", code)?;
        Ok(Device {
            _session: Arc::clone(&self.session),
            raw,
        })
    }

    /// Acquires a [`Device`] handle by GPU or MIG UUID (`nvmlDeviceGetHandleByUUID`).
    pub fn device_by_uuid(&self, uuid: &str) -> Result<Device> {
        validate_uuid(uuid)?;
        let buf = encode_c_buf::<{ sys::NVML_DEVICE_UUID_V2_BUFFER_SIZE as usize }>(
            uuid,
            "UUID exceeds buffer size or contains NUL",
        )?;
        let mut raw: sys::nvmlDevice_t = std::ptr::null_mut();
        // SAFETY: NVML is initialized, `buf` is NUL-terminated, and `raw` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetHandleByUUID(buf.as_ptr(), &mut raw) };
        check("nvmlDeviceGetHandleByUUID", code)?;
        Ok(Device {
            _session: Arc::clone(&self.session),
            raw,
        })
    }

    /// Acquires a [`Device`] handle by PCI bus ID (`domain:bus:device.function`)
    /// (`nvmlDeviceGetHandleByPciBusId_v2`).
    pub fn device_by_pci_bus_id(&self, bus_id: &PciBusId) -> Result<Device> {
        let formatted = bus_id.to_string();
        let buf = encode_c_buf::<{ sys::NVML_DEVICE_PCI_BUS_ID_BUFFER_SIZE as usize }>(
            &formatted,
            "PCI bus ID exceeds buffer size",
        )?;
        let mut raw: sys::nvmlDevice_t = std::ptr::null_mut();
        // SAFETY: NVML is initialized, `buf` is NUL-terminated, and `raw` is a valid out-pointer.
        let code = unsafe { sys::nvmlDeviceGetHandleByPciBusId_v2(buf.as_ptr(), &mut raw) };
        check("nvmlDeviceGetHandleByPciBusId_v2", code)?;
        Ok(Device {
            _session: Arc::clone(&self.session),
            raw,
        })
    }

    /// Enumerates all accessible [`Device`]s (`0..device_count()`).
    pub fn devices(&self) -> Result<Vec<Device>> {
        let count = self.device_count()?;
        let mut out = Vec::with_capacity(count as usize);
        for idx in 0..count {
            out.push(self.device_by_index(idx)?);
        }
        Ok(out)
    }
}
