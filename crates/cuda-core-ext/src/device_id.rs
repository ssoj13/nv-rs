//! Device identity: the UUID every API reports for the same GPU, and the Windows LUID.
//!
//! **Why.** Interop needs both sides on the same physical GPU. Vulkan reports
//! `VkPhysicalDeviceIDProperties::deviceUUID` (and `deviceLUID` on Windows), DXGI reports the
//! adapter LUID; CUDA's `cuDeviceGetUuid` / `cuDeviceGetLuid` give the same values, so comparing
//! them pairs a CUDA context with a wgpu adapter on a multi-GPU machine.

use cuda_core::{Context, DriverError, IntoResult};

/// The device's 16-byte UUID (`cuDeviceGetUuid`), equal to Vulkan's `deviceUUID`.
pub fn device_uuid(ctx: &Context) -> Result<[u8; 16], DriverError> {
    // SAFETY: zeroed CUuuid is a valid out-parameter; the device handle comes from a live context.
    let mut uuid: cuda_core::sys::CUuuid = unsafe { std::mem::zeroed() };
    // SAFETY: valid out-pointer and device.
    unsafe { cuda_core::sys::cuDeviceGetUuid_v2(&mut uuid, ctx.cu_device()) }.result()?;
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

/// The device's LUID. Windows only: the driver reports `CUDA_ERROR_NOT_SUPPORTED` elsewhere.
pub fn device_luid(ctx: &Context) -> Result<Luid, DriverError> {
    let mut bytes = [0 as std::ffi::c_char; 8];
    let mut node_mask = 0u32;
    // SAFETY: 8-byte LUID buffer and a valid node-mask out-pointer, as the API requires.
    unsafe { cuda_core::sys::cuDeviceGetLuid(bytes.as_mut_ptr(), &mut node_mask, ctx.cu_device()) }
        .result()?;
    Ok(Luid {
        bytes: bytes.map(|b| b as u8),
        node_mask,
    })
}
