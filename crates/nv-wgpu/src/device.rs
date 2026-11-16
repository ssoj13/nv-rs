//! Device matching, Vulkan extension negotiation, and `gpu-info-rs` composition.
//!
//! Why: External memory and timeline semaphore interop only works when both `wgpu`
//! and CUDA address the exact same physical GPU and the `wgpu` logical device has
//! enabled the required OS-specific Vulkan/DX12 sharing extensions. This module
//! enforces strict UUID/LUID verification (zero silent device mismatch), negotiates
//! Vulkan extensions via `wgpu_hal::vulkan::Adapter::open_with_callback`, and
//! composes cleanly with `gpu_info::SharedGpu`.
//!
//! Used by callers setting up [`InteropContext`](crate::InteropContext) and by
//! tests validating multi-GPU / multi-backend device selection.

use std::ffi::{CStr, CString};
use std::sync::{Arc, Mutex};

use ash::{khr, vk};
use cuda_core::Context;
#[cfg(windows)]
use cuda_core_ext::device_luid as cuda_device_luid;
use cuda_core_ext::device_uuid as cuda_device_uuid;
#[cfg(windows)]
use wgpu::hal::api::Dx12;
use wgpu::hal::api::Vulkan;

use crate::error::{Error, Result};

/// Active `wgpu` graphics backend used for CUDA zero-copy interop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteropBackend {
    /// Vulkan (`VK_KHR_external_memory_*` + `VK_KHR_external_semaphore_*` + timeline semaphores).
    Vulkan,
    /// Direct3D 12 (`D3D12_HEAP_FLAG_SHARED` + `D3D12_FENCE_FLAG_SHARED` NT handles).
    #[cfg(windows)]
    Dx12,
}

/// Physical hardware identity extracted from a `wgpu` adapter/device for CUDA matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuIdentity {
    /// 16-byte Vulkan `VkPhysicalDeviceIDProperties::deviceUUID`.
    Uuid([u8; 16]),
    /// 8-byte Windows DXGI `AdapterLuid` (little-endian `LowPart || HighPart`).
    #[cfg(windows)]
    Luid([u8; 8]),
}

/// Detects the interop backend of a `wgpu::Device`, failing explicitly on unsupported backends.
pub fn detect_backend(device: &wgpu::Device) -> Result<InteropBackend> {
    // SAFETY: `as_hal` only inspects the HAL backend wrapper without destroying it.
    if unsafe { device.as_hal::<Vulkan>() }.is_some() {
        return Ok(InteropBackend::Vulkan);
    }
    #[cfg(windows)]
    // SAFETY: `as_hal` only inspects the DX12 HAL backend wrapper without destroying it.
    if unsafe { device.as_hal::<Dx12>() }.is_some() {
        return Ok(InteropBackend::Dx12);
    }
    Err(Error::Unsupported {
        what: "wgpu backend",
        required: "Vulkan or DX12 (Windows)",
        found: format!("{:?}", device.adapter_info().backend),
    })
}

/// Returns the Vulkan device extensions required on top of `wgpu` defaults for
/// external memory and timeline semaphore interop on the current OS.
///
/// Pass the adapter's Vulkan `api_version` so promoted Vulkan 1.1 / 1.2 base
/// extensions are included when running on Vulkan 1.0/1.1 drivers.
pub fn required_vulkan_extensions(device_api_version: u32) -> Vec<&'static CStr> {
    let mut exts = Vec::with_capacity(5);
    if device_api_version < vk::API_VERSION_1_1 {
        exts.push(khr::external_memory::NAME);
        exts.push(khr::external_semaphore::NAME);
    }
    if device_api_version < vk::API_VERSION_1_2 {
        exts.push(khr::timeline_semaphore::NAME);
    }
    #[cfg(windows)]
    {
        exts.push(khr::external_memory_win32::NAME);
        exts.push(khr::external_semaphore_win32::NAME);
    }
    #[cfg(unix)]
    {
        exts.push(khr::external_memory_fd::NAME);
        exts.push(khr::external_semaphore_fd::NAME);
    }
    exts
}

/// Verifies that an already-open `wgpu::Device` supports CUDA interop without
/// silent downgrades (checks Vulkan external memory/semaphore + timeline extensions,
/// or confirms DX12 on Windows).
pub fn verify_device_interop_support(device: &wgpu::Device) -> Result<InteropBackend> {
    let backend = detect_backend(device)?;
    if backend == InteropBackend::Vulkan {
        // SAFETY: Read-only inspection of Vulkan HAL device extensions and capabilities.
        let hal_dev = unsafe { device.as_hal::<Vulkan>() }.ok_or(Error::BackendMismatch {
            expected: "Vulkan",
            found: device.adapter_info().backend,
        })?;
        let enabled = hal_dev.enabled_device_extensions();

        #[cfg(windows)]
        let platform_exts: [&CStr; 2] = [
            khr::external_memory_win32::NAME,
            khr::external_semaphore_win32::NAME,
        ];
        #[cfg(unix)]
        let platform_exts: [&CStr; 2] = [
            khr::external_memory_fd::NAME,
            khr::external_semaphore_fd::NAME,
        ];

        for req in platform_exts {
            if !enabled.contains(&req) {
                return Err(Error::Unsupported {
                    what: "vulkan device extension",
                    required: req.to_str().unwrap_or("external_interop"),
                    found: format!("{enabled:?}"),
                });
            }
        }

        // Timeline semaphores are promoted to Vulkan 1.2 core, or provided by VK_KHR_timeline_semaphore.
        let phys = hal_dev.raw_physical_device();
        let instance = hal_dev.shared_instance().raw_instance();
        // SAFETY: `phys` belongs to `instance`.
        let props = unsafe { instance.get_physical_device_properties(phys) };
        if props.api_version < vk::API_VERSION_1_2
            && !enabled.contains(&khr::timeline_semaphore::NAME)
        {
            return Err(Error::Unsupported {
                what: "vulkan timeline semaphore",
                required: "Vulkan 1.2+ or VK_KHR_timeline_semaphore",
                found: format!("api_version=0x{:x}", props.api_version),
            });
        }
    }
    Ok(backend)
}

/// Queries the hardware UUID (Vulkan) or LUID (DX12) of a `wgpu::Adapter` / `wgpu::Device`.
pub fn wgpu_gpu_identity(adapter: &wgpu::Adapter, device: &wgpu::Device) -> Result<GpuIdentity> {
    #[cfg(not(windows))]
    let _ = adapter;
    match detect_backend(device)? {
        InteropBackend::Vulkan => wgpu_vulkan_uuid(device).map(GpuIdentity::Uuid),
        #[cfg(windows)]
        InteropBackend::Dx12 => wgpu_dx12_luid(adapter).map(GpuIdentity::Luid),
    }
}

/// Queries `VkPhysicalDeviceIDProperties::deviceUUID` for a Vulkan `wgpu::Device`.
pub fn wgpu_vulkan_uuid(device: &wgpu::Device) -> Result<[u8; 16]> {
    // SAFETY: Inspects Vulkan physical device properties via `as_hal`.
    let hal_dev = unsafe { device.as_hal::<Vulkan>() }.ok_or(Error::BackendMismatch {
        expected: "Vulkan",
        found: device.adapter_info().backend,
    })?;
    let shared_inst = hal_dev.shared_instance();
    let raw_inst = shared_inst.raw_instance();
    let phys = hal_dev.raw_physical_device();

    let mut id_props = vk::PhysicalDeviceIDProperties::default();
    let mut props2 = vk::PhysicalDeviceProperties2::default().push_next(&mut id_props);

    if shared_inst.instance_api_version() >= vk::API_VERSION_1_1 {
        // SAFETY: `props2` has `id_props` chained with valid lifetime for this call.
        unsafe { raw_inst.get_physical_device_properties2(phys, &mut props2) };
    } else if shared_inst
        .extensions()
        .contains(&khr::get_physical_device_properties2::NAME)
    {
        let ext =
            khr::get_physical_device_properties2::Instance::new(shared_inst.entry(), raw_inst);
        // SAFETY: Extension was verified present on the Vulkan instance.
        unsafe { ext.get_physical_device_properties2(phys, &mut props2) };
    } else {
        return Err(Error::Unsupported {
            what: "vulkan physical device UUID query",
            required: "Vulkan 1.1+ or VK_KHR_get_physical_device_properties2",
            found: format!(
                "instance_api_version=0x{:x}",
                shared_inst.instance_api_version()
            ),
        });
    }

    Ok(id_props.device_uuid)
}

/// Queries the DXGI `AdapterLuid` bytes for a DX12 `wgpu::Adapter`.
#[cfg(windows)]
pub fn wgpu_dx12_luid(adapter: &wgpu::Adapter) -> Result<[u8; 8]> {
    // SAFETY: Inspects DX12 HAL adapter's underlying `IDXGIAdapter3`.
    let hal_adapter = unsafe { adapter.as_hal::<Dx12>() }.ok_or(Error::BackendMismatch {
        expected: "Dx12",
        found: adapter.get_info().backend,
    })?;
    // SAFETY: `hal_adapter.as_raw()` returns a valid COM `IDXGIAdapter3`.
    let desc = unsafe { hal_adapter.as_raw().GetDesc2() }
        .map_err(|e| Error::dx12("IDXGIAdapter3::GetDesc2", e))?;
    Ok(luid_to_bytes(
        desc.AdapterLuid.LowPart,
        desc.AdapterLuid.HighPart,
    ))
}

/// Packs a Win32 `LUID` into the canonical 8-byte little-endian layout returned by CUDA.
#[cfg(windows)]
#[inline]
pub(crate) fn luid_to_bytes(low_part: u32, high_part: i32) -> [u8; 8] {
    let mut out = [0u8; 8];
    out[..4].copy_from_slice(&low_part.to_le_bytes());
    out[4..8].copy_from_slice(&high_part.to_le_bytes());
    out
}

/// Verifies that `cuda_ctx` refers to the exact same physical GPU as `(adapter, device)`.
pub fn verify_matching_cuda_device(
    adapter: &wgpu::Adapter,
    device: &wgpu::Device,
    cuda_ctx: &Context,
) -> Result<()> {
    match wgpu_gpu_identity(adapter, device)? {
        GpuIdentity::Uuid(wgpu_uuid) => {
            let cu_uuid = cuda_device_uuid(cuda_ctx)?;
            if wgpu_uuid != cu_uuid {
                return Err(Error::DeviceMismatch {
                    wgpu_id: format!("vk-uuid:{}", Error::fmt_uuid(&wgpu_uuid)),
                    cuda_id: format!("cuda-uuid:{}", Error::fmt_uuid(&cu_uuid)),
                });
            }
        }
        #[cfg(windows)]
        GpuIdentity::Luid(wgpu_luid) => {
            let cu_luid = cuda_device_luid(cuda_ctx)?;
            if wgpu_luid != cu_luid.bytes {
                return Err(Error::DeviceMismatch {
                    wgpu_id: format!("dx12-luid:{}", Error::fmt_luid(&wgpu_luid)),
                    cuda_id: format!("cuda-luid:{}", Error::fmt_luid(&cu_luid.bytes)),
                });
            }
        }
    }
    Ok(())
}

/// Scans visible CUDA devices (`0..Context::device_count()`) and returns the
/// [`Context`] whose hardware UUID (Vulkan) or LUID (DX12) matches `(adapter, device)`.
pub fn find_matching_cuda_device(
    adapter: &wgpu::Adapter,
    device: &wgpu::Device,
) -> Result<Arc<Context>> {
    let id = wgpu_gpu_identity(adapter, device)?;
    let count = Context::device_count()?;
    for ordinal in 0..count {
        let Ok(ctx) = Context::new(ordinal as usize) else {
            continue;
        };
        let matched = match id {
            GpuIdentity::Uuid(wgpu_uuid) => {
                cuda_device_uuid(&ctx).is_ok_and(|cu_uuid| cu_uuid == wgpu_uuid)
            }
            #[cfg(windows)]
            GpuIdentity::Luid(wgpu_luid) => {
                cuda_device_luid(&ctx).is_ok_and(|cu_luid| cu_luid.bytes == wgpu_luid)
            }
        };
        if matched {
            return Ok(ctx);
        }
    }

    let wgpu_id = match id {
        GpuIdentity::Uuid(u) => format!("vk-uuid:{}", Error::fmt_uuid(&u)),
        #[cfg(windows)]
        GpuIdentity::Luid(l) => format!("dx12-luid:{}", Error::fmt_luid(&l)),
    };
    Err(Error::NoMatchingCudaDevice {
        wgpu_id,
        device_count: count,
    })
}

/// Interns a Vulkan extension name as a `'static CStr`, validating against
/// interior NUL bytes and leaking each unique string at most once.
fn intern_cstr(name: &str) -> Result<&'static CStr> {
    static INTERNED: Mutex<Vec<&'static CStr>> = Mutex::new(Vec::new());
    let cstr = CString::new(name)
        .map_err(|_| Error::InvalidInput("vulkan extension name contains interior NUL"))?;
    let mut guard = INTERNED.lock().map_err(|_| Error::InvalidHandle {
        source_api: "INTERNED vulkan extension mutex poisoned",
    })?;
    if let Some(&existing) = guard.iter().find(|&&s| s.to_bytes() == cstr.as_bytes()) {
        return Ok(existing);
    }
    let leaked: &'static CStr = Box::leak(cstr.into_boxed_c_str());
    guard.push(leaked);
    Ok(leaked)
}

/// Creates a Vulkan-backed [`wgpu::Instance`] with `instance_exts` enabled on top
/// of `wgpu`'s default instance extensions via `wgpu::hal::vulkan::Instance::init_with_callback`.
pub fn create_vulkan_instance_with_extensions(instance_exts: &[&str]) -> Result<wgpu::Instance> {
    let mut extra_cstrs = Vec::with_capacity(instance_exts.len());
    for &ext in instance_exts {
        extra_cstrs.push(intern_cstr(ext)?);
    }

    let hal_desc = wgpu::hal::InstanceDescriptor {
        name: "nv-wgpu",
        flags: wgpu::InstanceFlags::from_build_config(),
        memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
        backend_options: wgpu::BackendOptions::default(),
        telemetry: None,
        display: None,
    };

    let callback: Option<Box<wgpu::hal::vulkan::CreateInstanceCallback>> = if extra_cstrs.is_empty()
    {
        None
    } else {
        Some(Box::new(move |args| {
            for &ext in &extra_cstrs {
                if !args.extensions.contains(&ext) {
                    args.extensions.push(ext);
                }
            }
        }))
    };

    // SAFETY: `hal_desc` is valid for the duration of `init_with_callback`, and
    // the callback only appends validated extension names without removing features.
    let hal_instance =
        unsafe { wgpu::hal::vulkan::Instance::init_with_callback(&hal_desc, callback) }.map_err(
            |e| {
                Error::WgpuRequest(format!(
                    "wgpu_hal::vulkan::Instance::init_with_callback: {e}"
                ))
            },
        )?;

    // SAFETY: `hal_instance` was just initialized by `wgpu::hal::vulkan::Instance::init_with_callback`.
    Ok(unsafe { wgpu::Instance::from_hal::<Vulkan>(hal_instance) })
}

/// Opens a Vulkan `wgpu::Device` + `wgpu::Queue` from `adapter` with all required
/// external memory and external timeline semaphore extensions enabled via
/// `wgpu_hal::vulkan::Adapter::open_with_callback`.
///
/// Fails with [`Error::BackendMismatch`] if `adapter` is not Vulkan, or
/// [`Error::Unsupported`] if the adapter lacks any required external-interop extension.
pub fn open_vulkan_device_with_interop(
    adapter: &wgpu::Adapter,
    features: wgpu::Features,
    limits: &wgpu::Limits,
    label: Option<&str>,
) -> Result<(wgpu::Device, wgpu::Queue)> {
    open_vulkan_device_with_interop_extensions(adapter, features, limits, label, &[])
}

/// Opens a Vulkan `wgpu::Device` + `wgpu::Queue` from `adapter` with all required
/// external interop extensions plus `extra_device_exts` enabled via
/// `wgpu_hal::vulkan::Adapter::open_with_callback`.
///
/// Fails with [`Error::BackendMismatch`] if `adapter` is not Vulkan,
/// [`Error::InvalidInput`] if any extension name contains an interior NUL byte,
/// or [`Error::Unsupported`] if the adapter lacks any required or extra extension.
pub fn open_vulkan_device_with_interop_extensions(
    adapter: &wgpu::Adapter,
    features: wgpu::Features,
    limits: &wgpu::Limits,
    label: Option<&str>,
    extra_device_exts: &[&str],
) -> Result<(wgpu::Device, wgpu::Queue)> {
    let mut extra_cstrs = Vec::with_capacity(extra_device_exts.len());
    for &ext in extra_device_exts {
        extra_cstrs.push(intern_cstr(ext)?);
    }

    // SAFETY: Reads Vulkan HAL adapter capabilities without destroying the adapter.
    let hal_adapter = unsafe { adapter.as_hal::<Vulkan>() }.ok_or(Error::BackendMismatch {
        expected: "Vulkan",
        found: adapter.get_info().backend,
    })?;

    let caps = hal_adapter.physical_device_capabilities();
    let raw_inst = hal_adapter.shared_instance().raw_instance();
    let phys = hal_adapter.raw_physical_device();
    // SAFETY: `phys` is a valid physical device of `raw_inst`.
    let api_version = unsafe { raw_inst.get_physical_device_properties(phys) }.api_version;
    let mut required = required_vulkan_extensions(api_version);
    for ext in extra_cstrs {
        if !required.contains(&ext) {
            required.push(ext);
        }
    }
    for &ext in &required {
        if !caps.supports_extension(ext) {
            return Err(Error::Unsupported {
                what: "vulkan physical device extension",
                required: ext.to_str().unwrap_or("external_interop"),
                found: format!("adapter={}", adapter.get_info().name),
            });
        }
    }

    // SAFETY: All `required` extensions were verified supported by `caps`, and the
    // callback preserves all extensions and features requested by `wgpu`.
    let hal_open = unsafe {
        hal_adapter.open_with_callback(
            features,
            limits,
            &wgpu::MemoryHints::Performance,
            Some(Box::new(move |args| {
                for &ext in &required {
                    if !args.extensions.contains(&ext) {
                        args.extensions.push(ext);
                    }
                }
            })),
        )
    }
    .map_err(|e| {
        Error::WgpuRequest(format!(
            "wgpu_hal::vulkan::Adapter::open_with_callback: {e}"
        ))
    })?;

    let desc = wgpu::DeviceDescriptor {
        label,
        required_features: features,
        required_limits: limits.clone(),
        memory_hints: wgpu::MemoryHints::Performance,
        ..Default::default()
    };

    // SAFETY: `hal_open` was opened directly from `hal_adapter` with `features` and `limits`.
    unsafe { adapter.create_device_from_hal::<Vulkan>(hal_open, &desc) }
        .map_err(|e| Error::WgpuRequest(format!("Adapter::create_device_from_hal: {e}")))
}

/// Validates and pairs `gpu_info::SharedGpu` with its matching CUDA [`Context`].
///
/// This verifies `SharedGpu.device` already has the required interop extensions
/// (or DX12 backend on Windows) and refuses silent fallbacks if e.g.
/// `gpu_info::shared_device()` omitted `VK_KHR_external_semaphore_win32` because
/// video decode extensions were absent on the adapter. In that case, callers
/// can open an interop-enabled device from `&shared.adapter` via
/// [`open_vulkan_device_with_interop`].
pub fn pair_shared_gpu(shared: &gpu_info::SharedGpu) -> Result<(InteropBackend, Arc<Context>)> {
    let backend = verify_device_interop_support(&shared.device)?;
    let ctx = find_matching_cuda_device(&shared.adapter, &shared.device)?;
    Ok((backend, ctx))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_vulkan_extensions_include_platform_memory_and_semaphore() {
        let vk13 = required_vulkan_extensions(vk::API_VERSION_1_3);
        #[cfg(windows)]
        {
            assert!(vk13.contains(&khr::external_memory_win32::NAME));
            assert!(vk13.contains(&khr::external_semaphore_win32::NAME));
            assert!(!vk13.contains(&khr::timeline_semaphore::NAME));
        }
        #[cfg(unix)]
        {
            assert!(vk13.contains(&khr::external_memory_fd::NAME));
            assert!(vk13.contains(&khr::external_semaphore_fd::NAME));
            assert!(!vk13.contains(&khr::timeline_semaphore::NAME));
        }

        let vk10 = required_vulkan_extensions(vk::API_VERSION_1_0);
        assert!(vk10.contains(&khr::external_memory::NAME));
        assert!(vk10.contains(&khr::external_semaphore::NAME));
        assert!(vk10.contains(&khr::timeline_semaphore::NAME));
    }

    #[cfg(windows)]
    #[test]
    fn luid_packing_matches_little_endian_order() {
        let bytes = luid_to_bytes(0x0403_0201, 0x0807_0605);
        assert_eq!(bytes, [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    }

    #[test]
    fn intern_cstr_deduplicates_and_rejects_interior_nul() {
        let a = intern_cstr("VK_KHR_test_ext").unwrap();
        let b = intern_cstr("VK_KHR_test_ext").unwrap();
        assert!(std::ptr::eq(a.as_ptr(), b.as_ptr()));
        assert!(matches!(
            create_vulkan_instance_with_extensions(&["VK_KHR_bad\0ext"]),
            Err(Error::InvalidInput(_))
        ));
    }
}
