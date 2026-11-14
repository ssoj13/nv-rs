//! GPU integration tests (T2/T3) for `nv-wgpu` CUDA <-> `wgpu` 30 zero-copy interop.
//!
//! Run on a machine with an NVIDIA GPU (and Windows for DX12 tests):
//! `cargo test -p nv-wgpu --test gpu -- --include-ignored`

use std::future::Future;
use std::pin::pin;
use std::sync::{Arc, mpsc};
use std::task::{Context as TaskContext, Poll, Wake, Waker};
use std::thread::Thread;

use cuda_core::Context as CudaContext;
use cuda_core_ext::TextureDesc;
use nv_wgpu::{
    Error, InteropBackend, InteropContext, SharedBufferDescriptor, SharedTextureDescriptor,
    aligned_row_pitch, aligned_staging_bytes,
};

struct ThreadWake(Thread);

impl Wake for ThreadWake {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: Future>(fut: F) -> F::Output {
    let waker = Waker::from(Arc::new(ThreadWake(std::thread::current())));
    let mut cx = TaskContext::from_waker(&waker);
    let mut fut = pin!(fut);
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(out) => return out,
            Poll::Pending => std::thread::park(),
        }
    }
}

fn open_vulkan_interop() -> InteropContext {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    let adapters = block_on(instance.enumerate_adapters(wgpu::Backends::VULKAN));
    for adapter in adapters {
        if let Ok(ctx) = InteropContext::open_vulkan(
            &adapter,
            wgpu::Features::empty(),
            &wgpu::Limits::default(),
            Some("nv-wgpu-vulkan-test"),
        ) {
            return ctx;
        }
    }
    panic!("no Vulkan adapter matched an active NVIDIA CUDA device with external interop");
}

#[cfg(windows)]
fn open_dx12_interop() -> InteropContext {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::DX12,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    let adapters = block_on(instance.enumerate_adapters(wgpu::Backends::DX12));
    for adapter in adapters {
        let Ok((device, queue)) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("nv-wgpu-dx12-test"),
            ..Default::default()
        })) else {
            continue;
        };
        if let Ok(ctx) = InteropContext::from_wgpu(&adapter, device, queue) {
            return ctx;
        }
    }
    panic!("no DX12 adapter matched an active NVIDIA CUDA device");
}

fn readback_wgpu_buffer(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    src: &wgpu::Buffer,
    size: u64,
) -> Vec<u8> {
    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("nv-wgpu-test-readback"),
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("nv-wgpu-test-copy"),
    });
    enc.copy_buffer_to_buffer(src, 0, &staging, 0, size);
    queue.submit([enc.finish()]);
    map_read_staging(device, &staging, size as usize)
}

fn map_read_staging(device: &wgpu::Device, staging: &wgpu::Buffer, len: usize) -> Vec<u8> {
    let (tx, rx) = mpsc::sync_channel(1);
    staging
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |res| {
            let _ = tx.send(res);
        });
    device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("wgpu device poll");
    rx.recv()
        .expect("map_async callback dropped")
        .expect("staging buffer map failed");
    let view = staging
        .slice(..)
        .get_mapped_range()
        .expect("mapped staging range");
    let out = view[..len].to_vec();
    drop(view);
    staging.unmap();
    out
}

fn upload_texture_via_staging(
    ctx: &InteropContext,
    tex: &wgpu::Texture,
    format: wgpu::TextureFormat,
    width: u32,
    height: u32,
    tight_pixels: &[u8],
) {
    let bpp = nv_wgpu::format_mapping(format)
        .expect("supported format")
        .bytes_per_pixel;
    let unpadded_row = (width as usize) * bpp;
    assert_eq!(tight_pixels.len(), unpadded_row * (height as usize));

    let pitch = aligned_row_pitch(format, width).expect("aligned pitch");
    let extent = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let staging_bytes = aligned_staging_bytes(format, extent).expect("staging bytes");
    let mut padded = vec![0u8; staging_bytes as usize];
    for row in 0..(height as usize) {
        let src_start = row * unpadded_row;
        let dst_start = row * (pitch as usize);
        padded[dst_start..dst_start + unpadded_row]
            .copy_from_slice(&tight_pixels[src_start..src_start + unpadded_row]);
    }

    let staging = ctx.device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("nv-wgpu-tex-upload-staging"),
        size: staging_bytes,
        usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    ctx.queue().write_buffer(&staging, 0, &padded);

    let mut enc = ctx
        .device()
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("nv-wgpu-copy-buf-to-tex"),
        });
    enc.copy_buffer_to_texture(
        wgpu::TexelCopyBufferInfo {
            buffer: &staging,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pitch),
                rows_per_image: Some(height),
            },
        },
        wgpu::TexelCopyTextureInfo {
            texture: tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        extent,
    );
    ctx.queue().submit([enc.finish()]);
}

fn readback_texture_via_staging(
    ctx: &InteropContext,
    tex: &wgpu::Texture,
    format: wgpu::TextureFormat,
    width: u32,
    height: u32,
) -> Vec<u8> {
    let bpp = nv_wgpu::format_mapping(format)
        .expect("supported format")
        .bytes_per_pixel;
    let unpadded_row = (width as usize) * bpp;
    let pitch = aligned_row_pitch(format, width).expect("aligned pitch");
    let extent = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let staging_bytes = aligned_staging_bytes(format, extent).expect("staging bytes");
    let staging = ctx.device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("nv-wgpu-tex-readback-staging"),
        size: staging_bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc = ctx
        .device()
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("nv-wgpu-copy-tex-to-buf"),
        });
    enc.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &staging,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pitch),
                rows_per_image: Some(height),
            },
        },
        extent,
    );
    ctx.queue().submit([enc.finish()]);
    let padded = map_read_staging(ctx.device(), &staging, staging_bytes as usize);
    let mut tight = vec![0u8; unpadded_row * (height as usize)];
    for row in 0..(height as usize) {
        let src_start = row * (pitch as usize);
        let dst_start = row * unpadded_row;
        tight[dst_start..dst_start + unpadded_row]
            .copy_from_slice(&padded[src_start..src_start + unpadded_row]);
    }
    tight
}

fn run_buffer_roundtrip(ctx: &InteropContext) {
    let len = 4096usize;
    let shared = ctx
        .create_buffer(&SharedBufferDescriptor {
            label: Some("nv-wgpu-shared-buf"),
            size: len as u64,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
        })
        .expect("create shared buffer");
    assert_eq!(shared.size(), len as u64);
    assert_ne!(shared.cu_deviceptr(), 0);

    let sem = ctx.create_semaphore(0).expect("create interop semaphore");
    let stream = ctx.cuda_context().new_stream().expect("create cuda stream");
    ctx.cuda_context()
        .bind_to_thread()
        .expect("bind cuda context");

    // 1. CUDA writes pattern -> signals sem=1 -> wgpu acquires & reads back (zero host wait before submit)
    let pattern_a: Vec<u8> = (0..len).map(|i| ((i * 17 + 9) & 0xff) as u8).collect();
    // SAFETY: `shared.cu_deviceptr()` addresses `len` bytes in `ctx.cuda_context()`,
    // `pattern_a` outlives the subsequent `sem.wait_on_host(1, ...)` completion.
    unsafe {
        cuda_core::memory::memcpy_htod_async(
            shared.cu_deviceptr(),
            pattern_a.as_ptr(),
            len,
            stream.cu_stream(),
        )
    }
    .expect("cuda memcpy_htod_async");
    sem.signal_from_cuda(&stream, 1)
        .expect("signal_from_cuda(1)");

    shared
        .acquire_from_cuda(ctx.queue(), Some((&sem, 1)), None)
        .expect("acquire_from_cuda");
    let got_a = readback_wgpu_buffer(ctx.device(), ctx.queue(), shared.wgpu_buffer(), len as u64);
    assert_eq!(got_a, pattern_a, "wgpu must see exact CUDA writes");

    // 2. wgpu writes pattern_b -> releases & signals sem=2 -> CUDA waits on sem=2 & reads back
    let pattern_b: Vec<u8> = (0..len).map(|i| ((i * 31 + 3) & 0xff) as u8).collect();
    ctx.queue()
        .write_buffer(shared.wgpu_buffer(), 0, &pattern_b);
    ctx.queue().submit(std::iter::empty());
    shared
        .release_to_cuda(ctx.queue(), None, Some((&sem, 2)))
        .expect("release_to_cuda");

    sem.wait_in_cuda(&stream, 2).expect("wait_in_cuda(2)");
    let mut got_b = vec![0u8; len];
    // SAFETY: `shared.cu_deviceptr()` and `got_b` cover `len` bytes, synchronized below
    // before reading `got_b`.
    unsafe {
        cuda_core::memory::memcpy_dtoh_async(
            got_b.as_mut_ptr(),
            shared.cu_deviceptr(),
            len,
            stream.cu_stream(),
        )
    }
    .expect("cuda memcpy_dtoh_async");
    stream.synchronize().expect("cuda stream sync");
    assert_eq!(got_b, pattern_b, "CUDA must see exact wgpu writes");

    ctx.cuda_context()
        .check_err()
        .expect("no deferred CUDA teardown errors");
}

fn run_texture_roundtrip(ctx: &InteropContext) {
    let width = 37u32;
    let height = 19u32;
    let format = wgpu::TextureFormat::Rgba8Unorm;
    let desc = SharedTextureDescriptor {
        label: Some("nv-wgpu-shared-tex"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::STORAGE_BINDING,
        cuda_surface: true,
        cubemap: false,
        initial_state: wgpu::TextureUses::STORAGE_READ_WRITE,
    };
    let shared_tex = ctx.create_texture(&desc).expect("create shared 2D texture");
    let _tex_obj = shared_tex
        .cuda_texture_object(TextureDesc::default())
        .expect("cuda texture object");
    let surf_obj = shared_tex
        .cuda_surface_object(0)
        .expect("cuda surface object");
    assert_ne!(surf_obj.handle(), 0);

    let sem = ctx.create_semaphore(0).expect("create interop semaphore");
    let stream = ctx.cuda_context().new_stream().expect("create cuda stream");

    let byte_len = (width as usize) * (height as usize) * 4;
    let src_pixels: Vec<u8> = (0..byte_len).map(|i| ((i * 11 + 7) & 0xff) as u8).collect();
    shared_tex
        .cuda_level(0)
        .expect("level 0")
        .upload(&stream, &src_pixels)
        .expect("cuda array upload");
    sem.signal_from_cuda(&stream, 1)
        .expect("signal_from_cuda(1)");

    shared_tex
        .acquire_from_cuda(
            ctx.queue(),
            wgpu::TextureUses::STORAGE_READ_WRITE,
            Some((&sem, 1)),
            None,
        )
        .expect("acquire texture from cuda");
    let got_wgpu =
        readback_texture_via_staging(ctx, shared_tex.wgpu_texture(), format, width, height);
    assert_eq!(got_wgpu, src_pixels, "wgpu must observe CUDA array upload");

    let next_pixels: Vec<u8> = (0..byte_len)
        .map(|i| ((i * 23 + 13) & 0xff) as u8)
        .collect();
    upload_texture_via_staging(
        ctx,
        shared_tex.wgpu_texture(),
        format,
        width,
        height,
        &next_pixels,
    );
    shared_tex
        .release_to_cuda(
            ctx.queue(),
            wgpu::TextureUses::COPY_DST,
            None,
            Some((&sem, 2)),
        )
        .expect("release texture to cuda");

    sem.wait_in_cuda(&stream, 2).expect("wait_in_cuda(2)");
    let mut got_cuda = vec![0u8; byte_len];
    shared_tex
        .cuda_level(0)
        .expect("level 0")
        .download(&stream, &mut got_cuda)
        .expect("cuda array download");
    stream.synchronize().expect("cuda stream sync");
    assert_eq!(
        got_cuda, next_pixels,
        "CUDA must observe wgpu texture upload"
    );
}

#[test]
#[ignore = "GPU"]
fn vulkan_buffer_roundtrip_and_semaphore_sync() {
    let ctx = open_vulkan_interop();
    assert_eq!(ctx.backend(), InteropBackend::Vulkan);
    run_buffer_roundtrip(&ctx);
}

#[test]
#[ignore = "GPU"]
fn vulkan_texture_roundtrip_and_semaphore_sync() {
    let ctx = open_vulkan_interop();
    assert_eq!(ctx.backend(), InteropBackend::Vulkan);
    run_texture_roundtrip(&ctx);
}

#[cfg(windows)]
#[test]
#[ignore = "GPU"]
fn dx12_buffer_and_texture_roundtrip_with_shared_fence() {
    let ctx = open_dx12_interop();
    assert_eq!(ctx.backend(), InteropBackend::Dx12);
    run_buffer_roundtrip(&ctx);
    run_texture_roundtrip(&ctx);
}

#[test]
#[ignore = "GPU"]
fn gpu_info_shared_device_composition() {
    let shared = gpu_info::shared_device().expect("gpu_info::shared_device should find an adapter");
    match InteropContext::from_shared_gpu(shared) {
        Ok(ctx) => {
            assert!(ctx.verify().is_ok());
            run_buffer_roundtrip(&ctx);
        }
        Err(Error::Unsupported { .. }) => {
            // `gpu_info::shared_device()` opened Vulkan without external semaphore win32/fd
            // because e.g. video decode was unsupported; opening an interop device on its
            // adapter must still succeed without silent fallback.
            let ctx = InteropContext::open_vulkan(
                &shared.adapter,
                wgpu::Features::empty(),
                &wgpu::Limits::default(),
                Some("nv-wgpu-shared-adapter-interop"),
            )
            .expect("open_vulkan on SharedGpu.adapter");
            run_buffer_roundtrip(&ctx);
        }
        Err(other) => panic!("unexpected error pairing gpu_info::SharedGpu: {other}"),
    }
}

#[test]
#[ignore = "GPU"]
fn negative_validation_and_device_matching_on_gpu() {
    let ctx = open_vulkan_interop();
    assert!(ctx.verify().is_ok());

    let Err(map_err) = ctx.create_buffer(&SharedBufferDescriptor {
        label: None,
        size: 256,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::STORAGE,
    }) else {
        panic!("MAP_READ must be rejected");
    };
    assert!(matches!(map_err, Error::InvalidInput(_)));

    let Err(fmt_err) = ctx.create_texture(&SharedTextureDescriptor::d2(
        16,
        16,
        wgpu::TextureFormat::Depth32Float,
        wgpu::TextureUsages::TEXTURE_BINDING,
        false,
    )) else {
        panic!("Depth32Float must be rejected without downgrade");
    };
    assert!(matches!(
        fmt_err,
        Error::Unsupported {
            what: "texture format",
            ..
        }
    ));

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    let adapters = block_on(instance.enumerate_adapters(wgpu::Backends::VULKAN));
    let mut tested_pair = None;
    for adapter in adapters {
        if let Ok(ic) = InteropContext::open_vulkan(
            &adapter,
            wgpu::Features::empty(),
            &wgpu::Limits::default(),
            Some("nv-wgpu-mismatch-test"),
        ) {
            tested_pair = Some((adapter, ic));
            break;
        }
    }
    if let Some((adapter, ic)) = tested_pair {
        nv_wgpu::verify_matching_cuda_device(&adapter, ic.device(), ic.cuda_context())
            .expect("matching CUDA context must pass");
        let count = CudaContext::device_count().expect("device_count");
        let active_ord = ic.cuda_context().ordinal();
        for ord in 0..count {
            let ord_usize = ord as usize;
            if ord_usize != active_ord
                && let Ok(other_ctx) = CudaContext::new(ord_usize)
            {
                let err = nv_wgpu::verify_matching_cuda_device(&adapter, ic.device(), &other_ctx)
                    .expect_err("different CUDA ordinal must fail with DeviceMismatch");
                assert!(matches!(err, Error::DeviceMismatch { .. }));
            }
        }
    }
}
