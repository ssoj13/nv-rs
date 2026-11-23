//! External memory and semaphores end to end (T2, Windows): Vulkan (gpu-info-rs's shared device)
//! exports a buffer and a timeline semaphore, CUDA imports both through cuda-core-ext, and data crosses
//! in both directions ordered only by the semaphore — the nv-rs spike S2 rebuilt on the crate API.

#![cfg(windows)]

use ash::vk;
use cuda_core::{Context, IntoResult, sys};
use cuda_core_ext::{ExternalMemory, ExternalSemaphore, MemoryHandle, SemaphoreHandle};
use gpu_info::external::{SharedBuffer, device_uuid};
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};

const N: usize = 4096;

#[test]
#[ignore = "GPU: Vulkan + CUDA"]
fn vulkan_buffer_and_timeline_semaphore_round_trip() {
    // SAFETY: set before any wgpu instance exists in this test process.
    unsafe { std::env::set_var("WGPU_BACKEND", "vulkan") };
    let sg = gpu_info::shared_device().expect("shared wgpu device");
    let ctx = Context::new(0).expect("CUDA device 0");
    let stream = ctx.new_stream().expect("stream");
    assert_eq!(
        cuda_core_ext::device_uuid(&ctx).expect("uuid"),
        device_uuid(&sg.device).expect("vk uuid"),
        "same GPU"
    );

    // Vulkan allocates and exports; CUDA imports.
    let bytes = (N * 4) as u64;
    let (shared, export) = SharedBuffer::export(&sg.device, bytes).expect("export buffer");
    // SAFETY: a live dedicated OPAQUE_WIN32 export of `export.allocation` bytes on this GPU.
    let memory = unsafe {
        ExternalMemory::import(
            &ctx,
            MemoryHandle::OpaqueWin32(export.handle.as_raw_handle()),
            export.allocation,
            true,
        )
    }
    .expect("import memory");
    let buffer = memory.mapped_buffer(0, bytes).expect("map buffer");
    assert!(
        memory.mapped_buffer(0, export.allocation + 1).is_err(),
        "ranges past the import are rejected"
    );

    // Timeline semaphore: Vulkan creates and exports; CUDA imports.
    let (vk_dev, vk_inst) = {
        // SAFETY: handles are cloned and used while `sg.device` lives (process lifetime).
        let hal = unsafe { sg.device.as_hal::<wgpu::hal::api::Vulkan>() }.expect("vulkan");
        (
            hal.raw_device().clone(),
            hal.shared_instance().raw_instance().clone(),
        )
    };
    let ty = vk::ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32;
    // SAFETY: standard timeline-semaphore creation and Win32 export on a live device; the
    // exported NT handle is fresh and owned by this test, so `OwnedHandle` may adopt it (CUDA does
    // not consume Win32 handles, so it is closed when `sem_handle` drops).
    let (sem, sem_handle) = unsafe {
        let mut kind =
            vk::SemaphoreTypeCreateInfo::default().semaphore_type(vk::SemaphoreType::TIMELINE);
        let mut exp = vk::ExportSemaphoreCreateInfo::default().handle_types(ty);
        let sem = vk_dev
            .create_semaphore(
                &vk::SemaphoreCreateInfo::default()
                    .push_next(&mut kind)
                    .push_next(&mut exp),
                None,
            )
            .expect("semaphore");
        let win = ash::khr::external_semaphore_win32::Device::new(&vk_inst, &vk_dev);
        let h = win
            .get_semaphore_win32_handle(
                &vk::SemaphoreGetWin32HandleInfoKHR::default()
                    .semaphore(sem)
                    .handle_type(ty),
            )
            .expect("export");
        (
            sem,
            OwnedHandle::from_raw_handle(h as *mut std::ffi::c_void),
        )
    };
    // SAFETY: a live timeline semaphore exported as an NT handle on this GPU.
    let timeline = unsafe {
        ExternalSemaphore::import(
            &ctx,
            SemaphoreHandle::TimelineWin32(sem_handle.as_raw_handle()),
        )
    }
    .expect("import semaphore");
    assert!(timeline.is_valued());

    // A: CUDA writes, signals 1; wgpu waits for 1 and reads.
    let a: Vec<u32> = (0..N as u32).map(|i| i.wrapping_mul(2654435761)).collect();
    // SAFETY: `bytes` bytes into the mapped buffer from a live host slice, synchronized below by
    // the semaphore wait on the Vulkan side and the stream sync.
    unsafe {
        sys::cuMemcpyHtoDAsync_v2(
            buffer.cu_deviceptr(),
            a.as_ptr().cast(),
            bytes as usize,
            stream.cu_stream(),
        )
    }
    .result()
    .expect("copy");
    timeline.signal(&stream, 1).expect("signal 1");
    {
        // SAFETY: the semaphore outlives the submit that consumes this wait.
        let q = unsafe { sg.queue.as_hal::<wgpu::hal::api::Vulkan>() }.expect("vulkan queue");
        q.add_wait_semaphore(sem, Some(1), vk::PipelineStageFlags::ALL_COMMANDS);
    }
    let readback = sg.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("cuda-core-ext interop readback"),
        size: bytes,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut enc = sg.device.create_command_encoder(&Default::default());
    enc.copy_buffer_to_buffer(shared.buffer(), 0, &readback, 0, bytes);
    sg.queue
        .submit([shared.acquire(&sg.device).expect("acquire"), enc.finish()]);
    let slice = readback.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| tx.send(r).expect("send"));
    sg.device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("poll");
    rx.recv().expect("recv").expect("map");
    let got: Vec<u32> = slice
        .get_mapped_range()
        .expect("range")
        .as_chunks::<4>()
        .0
        .iter()
        .map(|c| u32::from_le_bytes(*c))
        .collect();
    readback.unmap();
    assert_eq!(got, a, "CUDA -> wgpu");

    // B: wgpu writes and signals 2; CUDA waits for 2 and reads.
    let b: Vec<u32> = (0..N as u32).map(|i| i ^ 0x5A5A_FFFF).collect();
    let upload = sg.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("cuda-core-ext interop upload"),
        size: bytes,
        usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::MAP_WRITE,
        mapped_at_creation: true,
    });
    upload
        .slice(..)
        .get_mapped_range_mut()
        .expect("upload range")
        .copy_from_slice(&b.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
    upload.unmap();
    let mut enc = sg.device.create_command_encoder(&Default::default());
    enc.copy_buffer_to_buffer(&upload, 0, shared.buffer(), 0, bytes);
    {
        // SAFETY: as above; the signal is consumed by the next submit.
        let q = unsafe { sg.queue.as_hal::<wgpu::hal::api::Vulkan>() }.expect("vulkan queue");
        q.add_signal_semaphore(sem, Some(2));
    }
    sg.queue.submit([
        shared.acquire(&sg.device).expect("acquire"),
        enc.finish(),
        shared.release(&sg.device).expect("release"),
    ]);
    timeline.wait(&stream, 2).expect("wait 2");
    let mut back = vec![0u32; N];
    // SAFETY: device-to-host into a live slice; the stream is synchronized right after.
    unsafe {
        sys::cuMemcpyDtoHAsync_v2(
            back.as_mut_ptr().cast(),
            buffer.cu_deviceptr(),
            bytes as usize,
            stream.cu_stream(),
        )
    }
    .result()
    .expect("copy back");
    stream.synchronize().expect("sync");
    assert_eq!(back, b, "wgpu -> CUDA");

    drop((buffer, timeline, memory));
    sg.device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("idle");
    // SAFETY: no submitted work references the semaphore any more (device idle above).
    unsafe { vk_dev.destroy_semaphore(sem, None) };
}
