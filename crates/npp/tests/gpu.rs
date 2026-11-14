//! GPU tests (T2/T3) for `npp`: color conversion roundtrip, resize, filter, threshold, and
//! 1D signal operations.

use cuda_core::{Context, DeviceBuffer};
use npp::image::{self, CmpOp, ImageView, ImageViewMut, Interpolation, Rect, Size};
use npp::{NppContext, signal};

#[test]
#[ignore = "GPU"]
fn color_convert_and_resize_roundtrip() {
    let ver = npp::lib_version().expect("nppGetLibVersion");
    assert!(ver.major >= 12);

    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let ctx = NppContext::new(&stream).expect("NppContext");

    let w = 64u32;
    let h = 64u32;
    let size = Size {
        width: w,
        height: h,
    };
    let mut rgb_host = vec![0u8; (w * h * 3) as usize];
    for y in 0..h {
        for x in 0..w {
            let i = ((y * w + x) * 3) as usize;
            rgb_host[i] = (x * 4) as u8;
            rgb_host[i + 1] = (y * 4) as u8;
            rgb_host[i + 2] = 128;
        }
    }

    let rgb_dev = DeviceBuffer::from_host(&stream, &rgb_host).expect("rgb_dev");
    let mut ycbcr_dev =
        DeviceBuffer::from_host(&stream, &vec![0u8; rgb_host.len()]).expect("ycbcr_dev");
    let mut back_dev =
        DeviceBuffer::from_host(&stream, &vec![0u8; rgb_host.len()]).expect("back_dev");

    let rgb_v = ImageView::packed(&rgb_dev, size, 3).expect("view");
    let mut ycbcr_m = ImageViewMut::packed(&mut ycbcr_dev, size, 3).expect("view");
    image::rgb_to_ycbcr_8u(&ctx, &rgb_v, &mut ycbcr_m).expect("rgb_to_ycbcr");

    let ycbcr_v = ImageView::packed(&ycbcr_dev, size, 3).expect("view");
    let mut back_m = ImageViewMut::packed(&mut back_dev, size, 3).expect("view");
    image::ycbcr_to_rgb_8u(&ctx, &ycbcr_v, &mut back_m).expect("ycbcr_to_rgb");

    let mut gray_dev =
        DeviceBuffer::from_host(&stream, &vec![0u8; (w * h) as usize]).expect("gray_dev");
    let mut gray_m = ImageViewMut::packed(&mut gray_dev, size, 1).expect("view");
    image::rgb_to_gray_8u(&ctx, &rgb_v, &mut gray_m).expect("rgb_to_gray");

    let half_sz = Size {
        width: 32,
        height: 32,
    };
    let mut half_dev =
        DeviceBuffer::from_host(&stream, &vec![0u8; (32 * 32 * 3) as usize]).expect("half_dev");
    let mut half_m = ImageViewMut::packed(&mut half_dev, half_sz, 3).expect("view");
    image::resize_8u_c3(
        &ctx,
        &rgb_v,
        Rect::full(size),
        &mut half_m,
        Rect::full(half_sz),
        Interpolation::Linear,
    )
    .expect("resize");

    stream.synchronize().expect("sync");

    let back_host = back_dev.to_host_vec(&stream).expect("readback");
    let max_diff = rgb_host
        .iter()
        .zip(back_host.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .max()
        .unwrap_or(0);
    // Studio-range 8-bit YCbCr (Y in [16..235], Cb/Cr in [16..240]) + integer rounding yields up to 4-5 LSB roundtrip error.
    assert!(
        max_diff <= 5,
        "RGB <-> YCbCr roundtrip max diff {max_diff} > 5"
    );

    let gray_host = gray_dev.to_host_vec(&stream).expect("gray readback");
    assert!(
        gray_host[0] < gray_host[gray_host.len() - 1],
        "gradient luminance should increase"
    );

    let half_host = half_dev.to_host_vec(&stream).expect("half readback");
    assert_eq!(half_host.len(), 32 * 32 * 3);
}

#[test]
#[ignore = "GPU"]
fn filter_threshold_and_signal_ops() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let ctx = NppContext::new(&stream).expect("NppContext");

    let size = Size {
        width: 16,
        height: 16,
    };
    let src_host: Vec<u8> = (0..256).map(|i| i as u8).collect();
    let src_dev = DeviceBuffer::from_host(&stream, &src_host).expect("src");
    let mut dst_dev = DeviceBuffer::from_host(&stream, &vec![0u8; 256]).expect("dst");

    let src_v = ImageView::packed(&src_dev, size, 1).expect("src_v");
    let mut dst_m = ImageViewMut::packed(&mut dst_dev, size, 1).expect("dst_m");
    image::threshold_val_8u_c1(&ctx, &src_v, &mut dst_m, 127, 255, CmpOp::Greater)
        .expect("threshold");
    stream.synchronize().expect("sync");

    let thresh_host = dst_dev.to_host_vec(&stream).expect("readback");
    assert_eq!(thresh_host[127], 127);
    assert_eq!(thresh_host[128], 255);
    assert_eq!(thresh_host[255], 255);

    // Signal ops: set 2.0, add 3.0 -> 5.0, sum 64 elements -> 320.0
    let n = 64usize;
    let mut a = DeviceBuffer::from_host(&stream, &vec![0.0f32; n]).expect("a");
    let b = DeviceBuffer::from_host(&stream, &vec![3.0f32; n]).expect("b");
    let mut c = DeviceBuffer::from_host(&stream, &vec![0.0f32; n]).expect("c");
    let mut sum_out = DeviceBuffer::from_host(&stream, &[0.0f32]).expect("sum_out");

    signal::set_32f(&ctx, 2.0, &mut a).expect("set");
    signal::add_32f(&ctx, &a, &b, &mut c).expect("add");
    let needed = signal::sum_32f_scratch_bytes(&ctx, n).expect("scratch size");
    let mut scratch = DeviceBuffer::from_host(&stream, &vec![0u8; needed.max(1)]).expect("scratch");
    signal::sum_32f(&ctx, &c, &mut sum_out, &mut scratch).expect("sum");
    stream.synchronize().expect("sync");

    let sum_val = sum_out.to_host_vec(&stream).expect("sum readback")[0];
    assert!(
        (sum_val - 320.0).abs() < 1e-3,
        "expected 320.0, got {sum_val}"
    );
}
