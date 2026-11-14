//! External timeline semaphore and DX12 shared fence synchronization for CUDA <-> `wgpu`.
//!
//! Why: Sharing buffers or textures zero-copy between CUDA and `wgpu` requires
//! strict GPU-side execution ordering without stalling the CPU (`ctx.synchronize()`
//! or `device.poll(Wait)`). `InteropSemaphore` wraps a single timeline primitive:
//! - On Vulkan: a `VkSemaphore` (`VK_SEMAPHORE_TYPE_TIMELINE`) exported via
//!   `OPAQUE_WIN32` (Windows) or `OPAQUE_FD` (Unix) and imported into CUDA as a
//!   valued `ExternalSemaphore`.
//! - On DX12 (Windows): a shared `ID3D12Fence` (`D3D12_FENCE_FLAG_SHARED`) exported
//!   via `CreateSharedHandle` and imported into CUDA as `SemaphoreHandle::D3d12Fence`.
//!
//! All per-frame `signal_*`, `wait_*`, and `stage_*` operations perform zero heap allocations.
//!
//! Used by callers and integration tests to coordinate CUDA streams and `wgpu::Queue`.

use std::sync::Arc;

use ash::{khr, vk};
use cuda_core::{Context, Stream};
use cuda_core_ext::ExternalSemaphore;
#[cfg(windows)]
use wgpu::hal::api::Dx12;
use wgpu::hal::api::Vulkan;
#[cfg(windows)]
use windows::Win32::{
    Graphics::Direct3D12,
    System::Threading::{CreateEventW, INFINITE, WaitForSingleObject},
};

use crate::device::{InteropBackend, verify_device_interop_support};
use crate::error::{Error, Result};
use crate::platform::{
    ExternalSemaphoreKind, export_vk_semaphore, vk_external_semaphore_handle_type,
};

#[cfg(windows)]
use crate::platform::export_d3d12_shared_handle;

/// Cross-API GPU timeline semaphore shared between CUDA and `wgpu`.
///
/// Drop order invariant: `cuda` (`Arc<ExternalSemaphore>`) MUST drop before
/// `native`, which drops before `_device` (`wgpu::Device`), ensuring the CUDA
/// external handle is released before `vkDestroySemaphore` / `ID3D12Fence::Release`
/// and before the underlying `VkDevice` / `ID3D12Device` is destroyed.
pub struct InteropSemaphore {
    cuda: Arc<ExternalSemaphore>,
    native: NativeSemaphore,
    _device: wgpu::Device,
}

struct VkSemaphoreBacking {
    raw_device: ash::Device,
    instance: ash::Instance,
    semaphore: vk::Semaphore,
    api_version: u32,
}

enum NativeSemaphore {
    Vulkan(Box<VkSemaphoreBacking>),
    #[cfg(windows)]
    Dx12 {
        fence: Direct3D12::ID3D12Fence,
    },
}

// SAFETY: Vulkan `VkSemaphore` (timeline semaphore with external sync by user/driver)
// and COM `ID3D12Fence` + `ExternalSemaphore` + `wgpu::Device` are thread-safe to send.
unsafe impl Send for InteropSemaphore {}
// SAFETY: Shared methods take `&self` and delegate to driver-synchronized CUDA/Vulkan/DX12 calls.
unsafe impl Sync for InteropSemaphore {}

impl Drop for InteropSemaphore {
    fn drop(&mut self) {
        match &self.native {
            NativeSemaphore::Vulkan(vk) => {
                // SAFETY: `cuda` (`ExternalSemaphore`) lives in an earlier struct field and has
                // already dropped; `_device` lives in a later field so `raw_device` is still alive.
                unsafe {
                    vk.raw_device.destroy_semaphore(vk.semaphore, None);
                }
            }
            #[cfg(windows)]
            NativeSemaphore::Dx12 { .. } => {}
        }
    }
}

impl InteropSemaphore {
    /// Creates an external timeline semaphore initialized to `initial_value` and
    /// imports it into `cuda_ctx`.
    ///
    /// Fails explicitly with [`Error::Unsupported`] if the `wgpu::Device` lacks
    /// the required Vulkan external semaphore / timeline extensions or runs on an
    /// unsupported backend.
    pub fn new(device: &wgpu::Device, cuda_ctx: &Arc<Context>, initial_value: u64) -> Result<Self> {
        match verify_device_interop_support(device)? {
            InteropBackend::Vulkan => Self::new_vulkan(device, cuda_ctx, initial_value),
            #[cfg(windows)]
            InteropBackend::Dx12 => Self::new_dx12(device, cuda_ctx, initial_value),
        }
    }

    fn new_vulkan(
        device: &wgpu::Device,
        cuda_ctx: &Arc<Context>,
        initial_value: u64,
    ) -> Result<Self> {
        // SAFETY: Reads Vulkan HAL device handles without destroying them.
        let hal_dev =
            unsafe { device.as_hal::<Vulkan>() }.ok_or_else(|| Error::BackendMismatch {
                expected: "Vulkan",
                found: device.adapter_info().backend,
            })?;
        let raw_device = hal_dev.raw_device().clone();
        let instance = hal_dev.shared_instance().raw_instance().clone();
        let phys = hal_dev.raw_physical_device();
        // SAFETY: `phys` belongs to `instance`.
        let api_version = unsafe { instance.get_physical_device_properties(phys) }.api_version;

        let handle_type = vk_external_semaphore_handle_type();
        let mut type_info = vk::SemaphoreTypeCreateInfo::default()
            .semaphore_type(vk::SemaphoreType::TIMELINE)
            .initial_value(initial_value);
        let mut export_info = vk::ExportSemaphoreCreateInfo::default().handle_types(handle_type);
        let create_info = vk::SemaphoreCreateInfo::default()
            .push_next(&mut type_info)
            .push_next(&mut export_info);

        // SAFETY: `raw_device` has external semaphore + timeline semaphore support verified.
        let semaphore = unsafe { raw_device.create_semaphore(&create_info, None) }
            .map_err(|e| Error::vulkan("vkCreateSemaphore(TIMELINE)", e))?;

        let cuda = match export_vk_semaphore(&instance, &raw_device, semaphore)
            .and_then(|h| h.import_cuda_semaphore(cuda_ctx, ExternalSemaphoreKind::VulkanTimeline))
        {
            Ok(sem) => sem,
            Err(err) => {
                // SAFETY: `semaphore` failed import and has not been exposed anywhere.
                unsafe { raw_device.destroy_semaphore(semaphore, None) };
                return Err(err);
            }
        };

        if !cuda.is_valued() {
            drop(cuda);
            // SAFETY: `cuda` has been dropped so CUDA holds no import reference, and
            // `semaphore` was not yet exposed outside `new_vulkan`.
            unsafe { raw_device.destroy_semaphore(semaphore, None) };
            return Err(Error::Unsupported {
                what: "CUDA external semaphore",
                required: "valued timeline semaphore",
                found: "binary-only external semaphore".to_owned(),
            });
        }

        Ok(Self {
            cuda,
            native: NativeSemaphore::Vulkan(Box::new(VkSemaphoreBacking {
                raw_device,
                instance,
                semaphore,
                api_version,
            })),
            _device: device.clone(),
        })
    }

    #[cfg(windows)]
    fn new_dx12(
        device: &wgpu::Device,
        cuda_ctx: &Arc<Context>,
        initial_value: u64,
    ) -> Result<Self> {
        // SAFETY: Reads DX12 HAL device handle without destroying it.
        let hal_dev = unsafe { device.as_hal::<Dx12>() }.ok_or_else(|| Error::BackendMismatch {
            expected: "Dx12",
            found: device.adapter_info().backend,
        })?;
        let raw_device = hal_dev.raw_device();

        // SAFETY: `raw_device` is a live `ID3D12Device`.
        let fence: Direct3D12::ID3D12Fence =
            unsafe { raw_device.CreateFence(initial_value, Direct3D12::D3D12_FENCE_FLAG_SHARED) }
                .map_err(|e| Error::dx12("ID3D12Device::CreateFence(SHARED)", e))?;

        let exported =
            export_d3d12_shared_handle(raw_device, &fence, "ID3D12Fence->ID3D12DeviceChild")?;
        let cuda = exported.import_cuda_semaphore(cuda_ctx, ExternalSemaphoreKind::D3d12Fence)?;

        Ok(Self {
            cuda,
            native: NativeSemaphore::Dx12 { fence },
            _device: device.clone(),
        })
    }

    /// Signals `value` on the CUDA `stream` once prior CUDA work in `stream` finishes.
    ///
    /// Performs zero heap allocations.
    #[inline]
    pub fn signal_from_cuda(&self, stream: &Stream, value: u64) -> Result<()> {
        self.cuda.signal(stream, value)?;
        Ok(())
    }

    /// Makes subsequent CUDA work submitted on `stream` wait until the semaphore reaches `value`.
    ///
    /// Performs zero heap allocations.
    #[inline]
    pub fn wait_in_cuda(&self, stream: &Stream, value: u64) -> Result<()> {
        self.cuda.wait(stream, value)?;
        Ok(())
    }

    /// Stages a semaphore/fence wait (`>= value`) on `queue` for the next
    /// `wgpu::Queue::submit` call.
    ///
    /// Performs zero heap allocations.
    pub fn stage_wait_in_wgpu(&self, queue: &wgpu::Queue, value: u64) -> Result<()> {
        match &self.native {
            NativeSemaphore::Vulkan(vk) => {
                // SAFETY: Stages a timeline semaphore wait on `wgpu::hal::vulkan::Queue`.
                let hal_q =
                    unsafe { queue.as_hal::<Vulkan>() }.ok_or_else(|| Error::BackendMismatch {
                        expected: "Vulkan",
                        found: crate::platform::queue_backend_kind(queue),
                    })?;
                hal_q.add_wait_semaphore(
                    vk.semaphore,
                    Some(value),
                    vk::PipelineStageFlags::ALL_COMMANDS,
                );
                Ok(())
            }
            #[cfg(windows)]
            NativeSemaphore::Dx12 { fence } => {
                // SAFETY: Stages a fence wait on `wgpu::hal::dx12::Queue`.
                let hal_q =
                    unsafe { queue.as_hal::<Dx12>() }.ok_or_else(|| Error::BackendMismatch {
                        expected: "Dx12",
                        found: crate::platform::queue_backend_kind(queue),
                    })?;
                hal_q.add_wait_fence(fence.clone(), value);
                Ok(())
            }
        }
    }

    /// Stages a semaphore/fence signal (`value`) on `queue` for the next
    /// `wgpu::Queue::submit` call.
    ///
    /// Performs zero heap allocations.
    pub fn stage_signal_in_wgpu(&self, queue: &wgpu::Queue, value: u64) -> Result<()> {
        match &self.native {
            NativeSemaphore::Vulkan(vk) => {
                // SAFETY: Stages a timeline semaphore signal on `wgpu::hal::vulkan::Queue`.
                let hal_q =
                    unsafe { queue.as_hal::<Vulkan>() }.ok_or_else(|| Error::BackendMismatch {
                        expected: "Vulkan",
                        found: crate::platform::queue_backend_kind(queue),
                    })?;
                hal_q.add_signal_semaphore(vk.semaphore, Some(value));
                Ok(())
            }
            #[cfg(windows)]
            NativeSemaphore::Dx12 { fence } => {
                // SAFETY: Stages a fence signal on `wgpu::hal::dx12::Queue`.
                let hal_q =
                    unsafe { queue.as_hal::<Dx12>() }.ok_or_else(|| Error::BackendMismatch {
                        expected: "Dx12",
                        found: crate::platform::queue_backend_kind(queue),
                    })?;
                hal_q.add_signal_fence(fence.clone(), value);
                Ok(())
            }
        }
    }

    /// Removes a previously staged wait if it has not yet been consumed by `Queue::submit`.
    pub fn unstage_wait_in_wgpu(&self, queue: &wgpu::Queue) -> Result<bool> {
        match &self.native {
            NativeSemaphore::Vulkan(vk) => {
                // SAFETY: Inspects Vulkan queue pending wait list.
                let hal_q =
                    unsafe { queue.as_hal::<Vulkan>() }.ok_or_else(|| Error::BackendMismatch {
                        expected: "Vulkan",
                        found: crate::platform::queue_backend_kind(queue),
                    })?;
                Ok(hal_q.remove_wait_semaphore(vk.semaphore))
            }
            #[cfg(windows)]
            NativeSemaphore::Dx12 { fence } => {
                // SAFETY: Inspects DX12 queue pending wait list.
                let hal_q =
                    unsafe { queue.as_hal::<Dx12>() }.ok_or_else(|| Error::BackendMismatch {
                        expected: "Dx12",
                        found: crate::platform::queue_backend_kind(queue),
                    })?;
                Ok(hal_q.remove_wait_fence(fence))
            }
        }
    }

    /// Removes a previously staged signal if it has not yet been consumed by `Queue::submit`.
    pub fn unstage_signal_in_wgpu(&self, queue: &wgpu::Queue) -> Result<bool> {
        match &self.native {
            NativeSemaphore::Vulkan(vk) => {
                // SAFETY: Inspects Vulkan queue pending signal list.
                let hal_q =
                    unsafe { queue.as_hal::<Vulkan>() }.ok_or_else(|| Error::BackendMismatch {
                        expected: "Vulkan",
                        found: crate::platform::queue_backend_kind(queue),
                    })?;
                Ok(hal_q.remove_signal_semaphore(vk.semaphore))
            }
            #[cfg(windows)]
            NativeSemaphore::Dx12 { fence } => {
                // SAFETY: Inspects DX12 queue pending signal list.
                let hal_q =
                    unsafe { queue.as_hal::<Dx12>() }.ok_or_else(|| Error::BackendMismatch {
                        expected: "Dx12",
                        found: crate::platform::queue_backend_kind(queue),
                    })?;
                Ok(hal_q.remove_signal_fence(fence))
            }
        }
    }

    /// Immediately flushes a signal (`value`) on `queue` after all previously
    /// staged waits and `wgpu::Queue` writes (`write_buffer` / `write_texture`).
    pub fn signal_from_wgpu(&self, queue: &wgpu::Queue, value: u64) -> Result<()> {
        self.stage_signal_in_wgpu(queue, value)?;
        queue.submit(core::iter::empty());
        Ok(())
    }

    /// Queries the current completed timeline counter value on the GPU.
    pub fn completed_value(&self) -> Result<u64> {
        match &self.native {
            NativeSemaphore::Vulkan(vk) => {
                if vk.api_version >= vk::API_VERSION_1_2 {
                    // SAFETY: Vulkan 1.2 core `vkGetSemaphoreCounterValue` on live timeline semaphore.
                    unsafe { vk.raw_device.get_semaphore_counter_value(vk.semaphore) }
                        .map_err(|e| Error::vulkan("vkGetSemaphoreCounterValue", e))
                } else {
                    let ext = khr::timeline_semaphore::Device::new(&vk.instance, &vk.raw_device);
                    // SAFETY: `VK_KHR_timeline_semaphore` enabled on `< 1.2` device.
                    unsafe { ext.get_semaphore_counter_value(vk.semaphore) }
                        .map_err(|e| Error::vulkan("vkGetSemaphoreCounterValueKHR", e))
                }
            }
            #[cfg(windows)]
            NativeSemaphore::Dx12 { fence } => {
                // SAFETY: `fence` is a valid COM `ID3D12Fence`.
                let val = unsafe { fence.GetCompletedValue() };
                if val == u64::MAX {
                    return Err(Error::InvalidHandle {
                        source_api: "ID3D12Fence::GetCompletedValue (device removed)",
                    });
                }
                Ok(val)
            }
        }
    }

    /// Signals the timeline semaphore/fence from the host CPU.
    pub fn signal_from_host(&self, value: u64) -> Result<()> {
        match &self.native {
            NativeSemaphore::Vulkan(vk) => {
                let info = vk::SemaphoreSignalInfo::default()
                    .semaphore(vk.semaphore)
                    .value(value);
                if vk.api_version >= vk::API_VERSION_1_2 {
                    // SAFETY: `semaphore` is a live Vulkan 1.2 timeline semaphore.
                    unsafe { vk.raw_device.signal_semaphore(&info) }
                        .map_err(|e| Error::vulkan("vkSignalSemaphore", e))
                } else {
                    let ext = khr::timeline_semaphore::Device::new(&vk.instance, &vk.raw_device);
                    // SAFETY: `VK_KHR_timeline_semaphore` enabled on `< 1.2` device.
                    unsafe { ext.signal_semaphore(&info) }
                        .map_err(|e| Error::vulkan("vkSignalSemaphoreKHR", e))
                }
            }
            #[cfg(windows)]
            NativeSemaphore::Dx12 { fence } => {
                // SAFETY: `fence` is a valid COM `ID3D12Fence`.
                unsafe { fence.Signal(value) }.map_err(|e| Error::dx12("ID3D12Fence::Signal", e))
            }
        }
    }

    /// Blocks the calling host thread until the semaphore reaches at least `value`
    /// (up to `timeout_ns` on Vulkan).
    pub fn wait_on_host(&self, value: u64, timeout_ns: u64) -> Result<()> {
        match &self.native {
            NativeSemaphore::Vulkan(vk) => {
                let sems = [vk.semaphore];
                let vals = [value];
                let info = vk::SemaphoreWaitInfo::default()
                    .semaphores(&sems)
                    .values(&vals);
                if vk.api_version >= vk::API_VERSION_1_2 {
                    // SAFETY: `sems` and `vals` have matching length 1 and valid timeline semaphore.
                    unsafe { vk.raw_device.wait_semaphores(&info, timeout_ns) }
                        .map_err(|e| Error::vulkan("vkWaitSemaphores", e))
                } else {
                    let ext = khr::timeline_semaphore::Device::new(&vk.instance, &vk.raw_device);
                    // SAFETY: `VK_KHR_timeline_semaphore` enabled on `< 1.2` device.
                    unsafe { ext.wait_semaphores(&info, timeout_ns) }
                        .map_err(|e| Error::vulkan("vkWaitSemaphoresKHR", e))
                }
            }
            #[cfg(windows)]
            NativeSemaphore::Dx12 { fence } => {
                // SAFETY: `fence` is a valid COM `ID3D12Fence`.
                if unsafe { fence.GetCompletedValue() } >= value {
                    return Ok(());
                }
                // SAFETY: Creates an unnamed auto-reset Win32 event closed immediately after waiting.
                let event = unsafe { CreateEventW(None, false, false, None) }
                    .map_err(|e| Error::dx12("CreateEventW", e))?;
                // Wrap in ExportedHandle / OwnedHandle immediately so early return closes `event`.
                let _guard =
                    crate::platform::ExportedHandle::from_raw_win32(event.0, "CreateEventW")?;
                // SAFETY: `fence` and `event` are both live handles for the duration of the wait.
                unsafe { fence.SetEventOnCompletion(value, event) }
                    .map_err(|e| Error::dx12("ID3D12Fence::SetEventOnCompletion", e))?;
                let timeout_ms = if timeout_ns == u64::MAX {
                    INFINITE
                } else {
                    let rounded = timeout_ns.saturating_add(999_999) / 1_000_000;
                    u32::try_from(rounded).unwrap_or(u32::MAX - 1)
                };
                // SAFETY: `event` is kept alive by `_guard`.
                let wait_res = unsafe { WaitForSingleObject(event, timeout_ms) };
                if wait_res.0 != 0 {
                    return Err(Error::InvalidHandle {
                        source_api: "WaitForSingleObject (fence timeout or failure)",
                    });
                }
                Ok(())
            }
        }
    }

    /// Returns the underlying `wgpu::Backend` of the device that created this semaphore.
    pub(crate) fn wgpu_backend(&self) -> wgpu::Backend {
        self._device.adapter_info().backend
    }

    /// Returns the raw Vulkan semaphore handle if this semaphore uses Vulkan.
    pub fn vulkan_handle(&self) -> Option<vk::Semaphore> {
        match &self.native {
            NativeSemaphore::Vulkan(vk) => Some(vk.semaphore),
            #[cfg(windows)]
            NativeSemaphore::Dx12 { .. } => None,
        }
    }

    /// Returns the underlying `cuda_core_ext::ExternalSemaphore`.
    pub fn cuda_semaphore(&self) -> &Arc<ExternalSemaphore> {
        &self.cuda
    }
}
