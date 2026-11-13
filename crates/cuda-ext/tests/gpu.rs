//! GPU tests (T2) for cuda-ext: run with `cargo test -p cuda-ext -- --include-ignored` on a machine
//! with an NVIDIA GPU. External memory and semaphores are exercised end to end by `nv-wgpu`'s
//! tests (M6), which have an exporter (Vulkan through gpu-info-rs).

use cuda_core::Context;
use cuda_ext::{Array, ArrayDesc, ArrayFormat, HostRegistration, ManagedBuffer, MipmappedArray, SurfaceObject, TextureDesc, TextureObject};
use std::sync::Arc;

fn ctx() -> Arc<Context> {
    Context::new(0).expect("CUDA device 0")
}

#[test]
#[ignore = "GPU"]
fn device_uuid_and_luid_are_reported() {
    let ctx = ctx();
    assert_ne!(cuda_ext::device_uuid(&ctx).expect("uuid"), [0; 16]);
    if cfg!(windows) {
        let luid = cuda_ext::device_luid(&ctx).expect("luid");
        assert_ne!(luid.bytes, [0; 8]);
        assert_ne!(luid.node_mask, 0);
    }
}

#[test]
#[ignore = "GPU"]
fn array_round_trips_host_data() {
    let ctx = ctx();
    let stream = ctx.default_stream();
    let desc = ArrayDesc::d2(16, 8, ArrayFormat::F32, 4);
    let array = Array::new(&ctx, desc).expect("array");
    let data: Vec<u8> = (0..desc.total_bytes()).map(|i| (i * 7 % 251) as u8).collect();
    array.upload(&stream, &data).expect("upload");
    let mut back = vec![0u8; desc.total_bytes()];
    array.download(&stream, &mut back).expect("download");
    assert_eq!(back, data);
    assert!(array.upload(&stream, &data[1..]).is_err(), "a short slice is rejected before the driver");
}

#[test]
#[ignore = "GPU"]
fn textures_and_surfaces_are_created_over_their_storage() {
    let ctx = ctx();
    let plain = Arc::new(Array::new(&ctx, ArrayDesc::d2(32, 32, ArrayFormat::U8, 4)).expect("array"));
    let tex = TextureObject::from_array(&plain, TextureDesc { srgb: true, ..TextureDesc::default() }).expect("texture");
    assert_ne!(tex.handle(), 0);
    assert!(SurfaceObject::new(&plain).is_err(), "surfaces need ArrayDesc::surface");

    let writable = Arc::new(Array::new(&ctx, ArrayDesc { surface: true, ..ArrayDesc::d2(32, 32, ArrayFormat::F16, 4) }).expect("array"));
    assert_ne!(SurfaceObject::new(&writable).expect("surface").handle(), 0);

    let mips = Arc::new(MipmappedArray::new(&ctx, ArrayDesc::d2(64, 16, ArrayFormat::F32, 1), 4).expect("mips"));
    let l3 = mips.level(3).expect("level 3");
    assert_eq!((l3.desc().width, l3.desc().height), (8, 2), "each level halves, floored at 1");
    assert!(mips.level(4).is_err());
    assert_ne!(TextureObject::from_mipmapped(&mips, TextureDesc::default()).expect("mip texture").handle(), 0);
}

#[test]
#[ignore = "GPU"]
fn host_registration_maps_borrowed_memory() {
    let ctx = ctx();
    let mut data = vec![1.0f32; 4096];
    let mut reg = HostRegistration::new(&ctx, &mut data, true).expect("register");
    assert!(reg.device_ptr().is_some_and(|p| p != 0));
    reg.as_mut_slice()[0] = 2.0;
    drop(reg);
    assert_eq!(data[0], 2.0, "the registration wrote through the original borrow");
}

#[test]
#[ignore = "GPU"]
fn managed_memory_is_host_visible() {
    let ctx = ctx();
    let mut buf = ManagedBuffer::<u32>::zeroed(&ctx, 1024).expect("managed");
    assert!(buf.as_slice().iter().all(|v| *v == 0));
    buf.as_mut_slice().iter_mut().enumerate().for_each(|(i, v)| *v = i as u32);
    assert_eq!(buf.as_slice()[1023], 1023);
    assert_ne!(buf.cu_deviceptr(), 0);
}

#[test]
#[ignore = "GPU"]
fn green_contexts_split_the_sms() {
    let ctx = ctx();
    let parts = cuda_ext::split_sms(&ctx, 8).expect("split");
    assert!(!parts.is_empty());
    for p in &parts {
        assert!(p.sm_count() >= 8, "{}", p.sm_count());
        let s = p.stream().expect("stream");
        s.synchronize().expect("sync");
    }
}
