//! Device identity: the UUID every API reports for the same GPU, and the Windows LUID.
//!
//! **Why.** Interop needs both sides on the same physical GPU. Vulkan reports
//! `VkPhysicalDeviceIDProperties::deviceUUID` (and `deviceLUID` on Windows), DXGI reports the
//! adapter LUID; CUDA's `cuDeviceGetUuid` / `cuDeviceGetLuid` give the same values, so comparing
//! them pairs a CUDA context with a wgpu adapter on a multi-GPU machine.
//! **How.** Both queries need only a `CUdevice`: pass a context-free [`cuda_core::Device`] to
//! identify a GPU without retaining its primary context, or a [`Context`] already in hand.

use cuda_core::{Context, Device, DriverError, IntoResult, sys};
use std::sync::Arc;

/// Something that names one physical CUDA device: a [`Context`], an `Arc` of one, or a
/// context-free [`Device`].
pub trait DeviceRef {
    /// The raw `CUdevice` handle.
    fn raw_cu_device(&self) -> sys::CUdevice;
}

impl DeviceRef for Context {
    fn raw_cu_device(&self) -> sys::CUdevice {
        self.cu_device()
    }
}

impl DeviceRef for Device {
    fn raw_cu_device(&self) -> sys::CUdevice {
        self.cu_device()
    }
}

impl<T: DeviceRef + ?Sized> DeviceRef for Arc<T> {
    fn raw_cu_device(&self) -> sys::CUdevice {
        (**self).raw_cu_device()
    }
}

/// The device's 16-byte UUID (`cuDeviceGetUuid`), equal to Vulkan's `deviceUUID`.
pub fn device_uuid(device: &(impl DeviceRef + ?Sized)) -> Result<[u8; 16], DriverError> {
    // SAFETY: zeroed CUuuid is a valid out-parameter.
    let mut uuid: sys::CUuuid = unsafe { std::mem::zeroed() };
    // SAFETY: valid out-pointer; the device handle comes from a live `Device` or `Context`.
    unsafe { sys::cuDeviceGetUuid_v2(&mut uuid, device.raw_cu_device()) }.result()?;
    Ok(uuid.bytes.map(|b| b as u8))
}

/// A Windows adapter LUID plus the device's node mask (`cuDeviceGetLuid`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Luid {
    /// The 8-byte LUID, equal to DXGI's adapter LUID and Vulkan's `deviceLUID`.
    pub bytes: [u8; 8],
    /// Which node of a linked adapter this device is.
    pub node_mask: u32,
}

/// The device's LUID. Windows only: the driver reports `CUDA_ERROR_NOT_SUPPORTED` elsewhere and
/// for devices outside the WDDM driver model (TCC).
pub fn device_luid(device: &(impl DeviceRef + ?Sized)) -> Result<Luid, DriverError> {
    let mut bytes = [0 as std::ffi::c_char; 8];
    let mut node_mask = 0u32;
    // SAFETY: 8-byte LUID buffer and a valid node-mask out-pointer, as the API requires.
    unsafe { sys::cuDeviceGetLuid(bytes.as_mut_ptr(), &mut node_mask, device.raw_cu_device()) }
        .result()?;
    Ok(Luid {
        bytes: bytes.map(|b| b as u8),
        node_mask,
    })
}
