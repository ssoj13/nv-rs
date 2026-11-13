//! Spike S2 (nv-rs PLAN.md §8): one Vulkan allocation is a `wgpu::Buffer` and a CUDA device
//! pointer at once, and a Vulkan timeline semaphore orders the two APIs on the GPU.
//!
//! A: CUDA writes pattern A, signals the timeline to 1; wgpu waits for 1 (hal
//!    `Queue::add_wait_semaphore`), acquires the buffer from `VK_QUEUE_FAMILY_EXTERNAL`, copies it to
//!    a readback buffer. Pattern A must arrive intact — this also proves wgpu did not lazily clear
//!    a `from_hal` buffer over CUDA's data.
//! B: wgpu copies pattern B into the buffer, releases it, signals 2 (hal `add_signal_semaphore`);
//!    CUDA waits for 2 on its stream and reads B back.
//! The shared device and the exportable buffer come from gpu-info-rs (`shared_device`,
//! `external::SharedBuffer`). Throwaway code.

use ash::vk;
use cuda_core::{CudaContext, sys};
use gpu_info::external::{SharedBuffer, device_uuid};
use std::os::windows::io::AsRawHandle;

const N: usize = 4096;

fn cu(what: &str, r: sys::CUresult) -> Result<(), String> {
    if r as i32 == 0 { Ok(()) } else { Err(format!("{what}: CUresult {}", r as i32)) }
}

fn main() -> Result<(), String> {
    // SAFETY: single-threaded, before any wgpu instance exists.
    unsafe { std::env::set_var("WGPU_BACKEND", "vulkan") };
    let sg = gpu_info::shared_device().ok_or("no shared wgpu device")?;
    println!("adapter: {:?} ({:?})", sg.adapter.get_info().name, sg.adapter.get_info().backend);

    // --- CUDA on the same GPU ---
    let ctx = CudaContext::new(0).map_err(|e| format!("cuda ctx: {e:?}"))?;
    ctx.bind_to_thread().map_err(|e| format!("bind: {e:?}"))?;
    let stream = ctx.default_stream();
    let cs = stream.cu_stream();
    let vk_uuid = device_uuid(&sg.device).ok_or("no Vulkan device UUID")?;
    let mut cu_uuid: sys::CUuuid = unsafe { std::mem::zeroed() };
    let mut dev: sys::CUdevice = 0;
    unsafe {
        cu("cuCtxGetDevice", sys::cuCtxGetDevice(&mut dev))?;
        cu("cuDeviceGetUuid", sys::cuDeviceGetUuid_v2(&mut cu_uuid, dev))?;
    }
    let cu_bytes: [u8; 16] = cu_uuid.bytes.map(|b| b as u8);
    println!("device UUID match: {}", cu_bytes == vk_uuid);
    if cu_bytes != vk_uuid {
        return Err("CUDA and Vulkan are on different GPUs".into());
    }

    // --- shared buffer: Vulkan allocates and exports, CUDA imports ---
    let bytes = (N * 4) as u64;
    let (shared, export) = SharedBuffer::export(&sg.device, bytes).map_err(|e| e.to_string())?;
    let mut ext_mem: sys::CUexternalMemory = std::ptr::null_mut();
    let mut dptr: sys::CUdeviceptr = 0;
    unsafe {
        let mut desc: sys::CUDA_EXTERNAL_MEMORY_HANDLE_DESC = std::mem::zeroed();
        desc.type_ = sys::CUexternalMemoryHandleType_enum_CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32;
        desc.handle.win32.handle = export.handle.as_raw_handle();
        desc.size = export.allocation;
        desc.flags = sys::CUDA_EXTERNAL_MEMORY_DEDICATED;
        cu("cuImportExternalMemory", sys::cuImportExternalMemory(&mut ext_mem, &desc))?;
        let mut bdesc: sys::CUDA_EXTERNAL_MEMORY_BUFFER_DESC = std::mem::zeroed();
        bdesc.size = bytes;
        cu("cuExternalMemoryGetMappedBuffer", sys::cuExternalMemoryGetMappedBuffer(&mut dptr, ext_mem, &bdesc))?;
    }
    println!("imported {} B (allocation {} B) at {dptr:#x}", bytes, export.allocation);

    // --- timeline semaphore: Vulkan creates and exports, CUDA imports ---
    let (vk_dev, vk_inst) = {
        // SAFETY: handles are cloned and only used while `sg.device` lives (process lifetime).
        let hal = unsafe { sg.device.as_hal::<wgpu::hal::api::Vulkan>() }.ok_or("not Vulkan")?;
        (hal.raw_device().clone(), hal.shared_instance().raw_instance().clone())
    };
    let handle_ty = vk::ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32;
    let (sem, sem_handle) = unsafe {
        let mut ty = vk::SemaphoreTypeCreateInfo::default().semaphore_type(vk::SemaphoreType::TIMELINE).initial_value(0);
        let mut exp = vk::ExportSemaphoreCreateInfo::default().handle_types(handle_ty);
        let info = vk::SemaphoreCreateInfo::default().push_next(&mut ty).push_next(&mut exp);
        let sem = vk_dev.create_semaphore(&info, None).map_err(|e| format!("vkCreateSemaphore: {e}"))?;
        let win = ash::khr::external_semaphore_win32::Device::new(&vk_inst, &vk_dev);
        let h = win
            .get_semaphore_win32_handle(&vk::SemaphoreGetWin32HandleInfoKHR::default().semaphore(sem).handle_type(handle_ty))
            .map_err(|e| format!("vkGetSemaphoreWin32HandleKHR: {e}"))?;
        (sem, h)
    };
    let mut ext_sem: sys::CUexternalSemaphore = std::ptr::null_mut();
    unsafe {
        let mut d: sys::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC = std::mem::zeroed();
        d.type_ = sys::CUexternalSemaphoreHandleType_enum_CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_WIN32;
        d.handle.win32.handle = sem_handle as *mut _;
        cu("cuImportExternalSemaphore", sys::cuImportExternalSemaphore(&mut ext_sem, &d))?;
    }

    let readback = sg.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("s2 readback"),
        size: bytes,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // --- A: CUDA -> wgpu ---
    let a: Vec<u32> = (0..N as u32).map(|i| i.wrapping_mul(2654435761) ^ 0xA5A5_0000).collect();
    unsafe {
        cu("cuMemcpyHtoDAsync", sys::cuMemcpyHtoDAsync_v2(dptr, a.as_ptr().cast(), bytes as usize, cs))?;
        let mut sp: sys::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS = std::mem::zeroed();
        sp.params.fence.value = 1;
        cu("cuSignalExternalSemaphoresAsync", sys::cuSignalExternalSemaphoresAsync(&ext_sem, &sp, 1, cs))?;
    }
    {
        // SAFETY: the semaphore lives until the end of main; the wait is consumed by the next submit.
        let q = unsafe { sg.queue.as_hal::<wgpu::hal::api::Vulkan>() }.ok_or("queue not Vulkan")?;
        q.add_wait_semaphore(sem, Some(1), vk::PipelineStageFlags::ALL_COMMANDS);
    }
    let acquire = shared.acquire(&sg.device).map_err(|e| e.to_string())?;
    let mut enc = sg.device.create_command_encoder(&Default::default());
    enc.copy_buffer_to_buffer(shared.buffer(), 0, &readback, 0, bytes);
    sg.queue.submit([acquire, enc.finish()]);
    let got_a = read_u32(&sg.device, &readback)?;
    let bad_a = got_a.iter().zip(&a).filter(|(g, e)| g != e).count();
    println!("A (CUDA -> wgpu): {bad_a} wrong of {N}; first {:?}", &got_a[..4]);

    // --- B: wgpu -> CUDA ---
    let b: Vec<u32> = (0..N as u32).map(|i| i.wrapping_mul(40503) ^ 0x5A5A_FFFF).collect();
    let upload = sg.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("s2 upload"),
        size: bytes,
        usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::MAP_WRITE,
        mapped_at_creation: true,
    });
    upload.slice(..).get_mapped_range_mut().map_err(|e| format!("map upload: {e:?}"))?.copy_from_slice(as_bytes(&b));
    upload.unmap();
    let acquire = shared.acquire(&sg.device).map_err(|e| e.to_string())?;
    let mut enc = sg.device.create_command_encoder(&Default::default());
    enc.copy_buffer_to_buffer(&upload, 0, shared.buffer(), 0, bytes);
    let release = shared.release(&sg.device).map_err(|e| e.to_string())?;
    {
        let q = unsafe { sg.queue.as_hal::<wgpu::hal::api::Vulkan>() }.ok_or("queue not Vulkan")?;
        q.add_signal_semaphore(sem, Some(2));
    }
    sg.queue.submit([acquire, enc.finish(), release]);
    let mut got_b = vec![0u32; N];
    unsafe {
        let mut wp: sys::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS = std::mem::zeroed();
        wp.params.fence.value = 2;
        cu("cuWaitExternalSemaphoresAsync", sys::cuWaitExternalSemaphoresAsync(&ext_sem, &wp, 1, cs))?;
        cu("cuMemcpyDtoHAsync", sys::cuMemcpyDtoHAsync_v2(got_b.as_mut_ptr().cast(), dptr, bytes as usize, cs))?;
        cu("cuStreamSynchronize", sys::cuStreamSynchronize(cs))?;
    }
    let bad_b = got_b.iter().zip(&b).filter(|(g, e)| g != e).count();
    println!("B (wgpu -> CUDA): {bad_b} wrong of {N}; first {:?}", &got_b[..4]);

    unsafe {
        sys::cuDestroyExternalSemaphore(ext_sem);
        sys::cuDestroyExternalMemory(ext_mem);
        let _ = sg.device.poll(wgpu::PollType::wait_indefinitely());
        vk_dev.destroy_semaphore(sem, None);
    }
    if bad_a == 0 && bad_b == 0 {
        println!("S2 PASS: shared Vulkan buffer + timeline semaphore, both directions exact");
        Ok(())
    } else {
        Err(format!("S2 FAIL: A {bad_a} wrong, B {bad_b} wrong"))
    }
}

fn as_bytes(v: &[u32]) -> &[u8] {
    // SAFETY: u32 has no padding; the slice covers exactly len*4 initialised bytes.
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast(), v.len() * 4) }
}

fn read_u32(device: &wgpu::Device, buf: &wgpu::Buffer) -> Result<Vec<u32>, String> {
    let slice = buf.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| {
        let _ = tx.send(r);
    });
    device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| format!("poll: {e}"))?;
    rx.recv().map_err(|e| e.to_string())?.map_err(|e| format!("map: {e}"))?;
    let out = slice.get_mapped_range().map_err(|e| format!("map readback: {e:?}"))?.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
    buf.unmap();
    Ok(out)
}
