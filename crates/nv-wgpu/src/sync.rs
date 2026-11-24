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
//! All per-frame `signal_*`, `wait_*`, and `stage_*` operations perform zero heap allocations
//! (staging on a queue for the first time stores one `wgpu::Queue` clone).
//!
//! Timeline values signalled through this wrapper must strictly increase (Vulkan
//! VUID-VkSemaphoreSignalInfo-value-03258 and the queue-signal equivalent); a non-increasing
//! value is rejected before it reaches a driver. A failed signal still consumes its value.
//!
//! Teardown: dropping an [`InteropSemaphore`] removes its still-staged `wgpu` waits/signals,
//! waits for the `wgpu` submissions that used it, then lets the CUDA import synchronize its
//! context (cuda.h: outstanding CUDA signals/waits must complete before
//! `cuDestroyExternalSemaphore`), and only then destroys the `VkSemaphore`
//! (VUID-vkDestroySemaphore-semaphore-05149) while the `wgpu::Device` is still alive.
//!
//! Used by callers and integration tests to coordinate CUDA streams and `wgpu::Queue`.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use ash::{khr, vk};
use cuda_core::{Context, Stream};
use cuda_core_ext::ExternalSemaphore;
#[cfg(windows)]
use wgpu::hal::api::Dx12;
use wgpu::hal::api::Vulkan;
#[cfg(windows)]
use windows::Win32::{
    Foundation::WAIT_TIMEOUT,
    Graphics::Direct3D12,
    System::Threading::{CreateEventW, INFINITE, WaitForSingleObject},
};

use crate::device::{InteropBackend, effective_api_version, verify_device_interop_support};
use crate::error::{Error, Result};
use crate::platform::{
    ExternalSemaphoreKind, export_vk_semaphore, record_drop_failure, vk_external_semaphore_handle_type,
    wait_submission_bounded,
};

#[cfg(windows)]
use crate::platform::export_d3d12_shared_handle;

/// Cross-API GPU timeline semaphore shared between CUDA and `wgpu`.
///
/// Teardown order is structural: `Drop::drop` unstages it from every queue it was staged on
/// and waits (bounded by `VULKAN_SUBMIT_TIMEOUT`; a failure goes to `cuda_core_ext::teardown`) for
/// `wgpu`'s submissions; then the fields drop in declaration order:
/// `cuda` (synchronizes its CUDA context if CUDA work used it, then releases the import),
/// `native` (`vkDestroySemaphore` / `ID3D12Fence` release), and last `_device`, which keeps the
/// `VkDevice` / `ID3D12Device` alive for `native`. A CUDA wait whose signal is never submitted
/// blocks the drop forever.
pub struct InteropSemaphore {
    cuda: Arc<ExternalSemaphore>,
    native: NativeSemaphore,
    /// Queues this semaphore was staged on (directly or by a barrier submit).
    queues: Mutex<Vec<wgpu::Queue>>,
    /// Highest value signalled through this wrapper.
    last_signal: AtomicU64,
    _device: wgpu::Device,
}

struct VkSemaphoreBacking {
    raw_device: ash::Device,
    instance: ash::Instance,
    semaphore: vk::Semaphore,
    api_version: u32,
}

impl Drop for VkSemaphoreBacking {
    fn drop(&mut self) {
        // SAFETY: this backing is `InteropSemaphore::native`; by the time it drops, the semaphore
        // is unstaged, the `wgpu` submissions that used it completed (`InteropSemaphore::drop`),
        // the CUDA import was released after its context synchronized (the earlier `cuda`
        // field), and the `VkDevice` is kept alive by the later `_device` field.
        unsafe { self.raw_device.destroy_semaphore(self.semaphore, None) };
    }
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
// SAFETY: Shared methods take `&self` and delegate to driver-synchronized CUDA/Vulkan/DX12 calls;
// the queue list and the last signalled value are behind a `Mutex` / atomic.
unsafe impl Sync for InteropSemaphore {}

impl Drop for InteropSemaphore {
    fn drop(&mut self) {
        // Poisoning only means a panic elsewhere while pushing a queue clone; the list is intact.
        let queues = std::mem::take(
            self.queues
                .get_mut()
                .unwrap_or_else(PoisonError::into_inner),
        );
        if queues.is_empty() {
            return;
        }
        for queue in &queues {
            // A staged entry left behind would make the next `wgpu::Queue::submit` reference a
            // destroyed semaphore.
            if let Err(err) = self
                .unstage_wait_in_wgpu(queue)
                .and_then(|_| self.unstage_signal_in_wgpu(queue))
            {
                record_drop_failure("InteropSemaphore::drop (unstage from wgpu queue)", &err);
            }
        }
        // Every submission that used this semaphore went through `wgpu::Queue::submit`, so one
        // wait for the most recent submission covers all of them.
        if let Err(err) = wait_submission_bounded(
            &self._device,
            None,
            "InteropSemaphore::drop (wait for wgpu submissions)",
        ) {
            record_drop_failure("InteropSemaphore::drop (wait for wgpu submissions)", &err);
        }
    }
}

/// Moves the last signalled value to `value`, rejecting a value that does not exceed it.
fn advance_timeline(last: &AtomicU64, value: u64) -> Result<()> {
    last.try_update(Ordering::AcqRel, Ordering::Acquire, |current| {
        (value > current).then_some(value)
    })
    .map_err(|_| {
        Error::InvalidInput("timeline semaphore signal value must exceed every earlier signal")
    })?;
    Ok(())
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
        let device_api = unsafe { instance.get_physical_device_properties(phys) }.api_version;
        // Core 1.2 entry points exist only if both the instance and the device are 1.2+.
        let api_version =
            effective_api_version(hal_dev.shared_instance().instance_api_version(), device_api);

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
            queues: Mutex::new(Vec::new()),
            last_signal: AtomicU64::new(initial_value),
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
            queues: Mutex::new(Vec::new()),
            last_signal: AtomicU64::new(initial_value),
            _device: device.clone(),
        })
    }

    /// Remembers `queue` so `Drop` can unstage this semaphore from it and wait for its work.
    fn note_queue(&self, queue: &wgpu::Queue) {
        // Poisoning only means a panic elsewhere while pushing; the list is intact.
        let mut queues = self.queues.lock().unwrap_or_else(PoisonError::into_inner);
        if !queues.contains(queue) {
            queues.push(queue.clone());
        }
    }

    /// Signals `value` on the CUDA `stream` once prior CUDA work in `stream` finishes.
    /// `value` must exceed every value signalled before through this semaphore.
    ///
    /// Performs zero heap allocations.
    #[inline]
    pub fn signal_from_cuda(&self, stream: &Stream, value: u64) -> Result<()> {
        advance_timeline(&self.last_signal, value)?;
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
            }
        }
        self.note_queue(queue);
        Ok(())
    }

    /// Stages a semaphore/fence signal (`value`) on `queue` for the next
    /// `wgpu::Queue::submit` call. `value` must exceed every value signalled before through
    /// this semaphore.
    ///
    /// Performs zero heap allocations.
    pub fn stage_signal_in_wgpu(&self, queue: &wgpu::Queue, value: u64) -> Result<()> {
        advance_timeline(&self.last_signal, value)?;
        match &self.native {
            NativeSemaphore::Vulkan(vk) => {
                // SAFETY: Stages a timeline semaphore signal on `wgpu::hal::vulkan::Queue`.
                let hal_q =
                    unsafe { queue.as_hal::<Vulkan>() }.ok_or_else(|| Error::BackendMismatch {
                        expected: "Vulkan",
                        found: crate::platform::queue_backend_kind(queue),
                    })?;
                hal_q.add_signal_semaphore(vk.semaphore, Some(value));
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
            }
        }
        self.note_queue(queue);
        Ok(())
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

    /// Signals the timeline semaphore/fence from the host CPU. `value` must exceed every value
    /// signalled before through this semaphore.
    pub fn signal_from_host(&self, value: u64) -> Result<()> {
        advance_timeline(&self.last_signal, value)?;
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

    /// Blocks the calling host thread until the semaphore reaches at least `value`, or fails
    /// with [`Error::Timeout`] after `timeout_ns` (`u64::MAX` waits forever; DX12 rounds up to
    /// whole milliseconds).
    pub fn wait_on_host(&self, value: u64, timeout_ns: u64) -> Result<()> {
        match &self.native {
            NativeSemaphore::Vulkan(vk) => {
                let sems = [vk.semaphore];
                let vals = [value];
                let info = vk::SemaphoreWaitInfo::default()
                    .semaphores(&sems)
                    .values(&vals);
                let (call, res) = if vk.api_version >= vk::API_VERSION_1_2 {
                    // SAFETY: `sems` and `vals` have matching length 1 and valid timeline semaphore.
                    let res = unsafe { vk.raw_device.wait_semaphores(&info, timeout_ns) };
                    ("vkWaitSemaphores", res)
                } else {
                    let ext = khr::timeline_semaphore::Device::new(&vk.instance, &vk.raw_device);
                    // SAFETY: `VK_KHR_timeline_semaphore` enabled on `< 1.2` device.
                    let res = unsafe { ext.wait_semaphores(&info, timeout_ns) };
                    ("vkWaitSemaphoresKHR", res)
                };
                res.map_err(|e| match e {
                    vk::Result::TIMEOUT => Error::Timeout { what: call },
                    other => Error::vulkan(call, other),
                })
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
                if wait_res == WAIT_TIMEOUT {
                    return Err(Error::Timeout {
                        what: "ID3D12Fence::SetEventOnCompletion + WaitForSingleObject",
                    });
                }
                if wait_res.0 != 0 {
                    return Err(Error::InvalidHandle {
                        source_api: "WaitForSingleObject (wait failed)",
                    });
                }
                Ok(())
            }
        }
    }

    /// Returns the raw Vulkan semaphore handle if this semaphore uses Vulkan.
    pub fn vulkan_handle(&self) -> Option<vk::Semaphore> {
        match &self.native {
            NativeSemaphore::Vulkan(vk) => Some(vk.semaphore),
            #[cfg(windows)]
            NativeSemaphore::Dx12 { .. } => None,
        }
    }

    /// Returns the underlying `cuda_core_ext::ExternalSemaphore`, borrowed: the import must not
    /// outlive the native semaphore it refers to.
    pub fn cuda_semaphore(&self) -> &ExternalSemaphore {
        &self.cuda
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeline_values_must_strictly_increase() {
        let last = AtomicU64::new(0);
        assert!(
            advance_timeline(&last, 0).is_err(),
            "the initial value is already reached"
        );
        advance_timeline(&last, 2).unwrap();
        assert!(advance_timeline(&last, 2).is_err(), "re-signalling a value");
        assert!(advance_timeline(&last, 1).is_err(), "going backwards");
        advance_timeline(&last, 3).unwrap();
        assert_eq!(last.load(Ordering::Acquire), 3);
    }
}
