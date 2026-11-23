//! GPU tests (T2/T3) for `npp`: color conversion roundtrip, resize, filter, threshold, and
//! 1D signal operations.

use cuda_core::{Context, DeviceBuffer};
use npp::image::{self, CmpOp, ImageView, ImageViewMut, Interpolation, MaskSize, Rect, Size};
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

/// Linear gradient `x * 7 + y * 3 + c * 10` (max 160 for a 16x16, 3-channel image).
fn gradient(w: u32, h: u32, ch: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity((w * h * ch) as usize);
    for y in 0..h {
        for x in 0..w {
            for c in 0..ch {
                out.push((x * 7 + y * 3 + c * 10) as u8);
            }
        }
    }
    out
}

/// NPP-4: box and Gaussian filters on an interior subview read the border pixels the
/// anchor/padding logic promises. On a linear gradient a centred 3x3 average (box) and any
/// symmetric kernel (Gauss) reproduce the centre pixel, so an anchor or border mismatch shows up
/// as an offset of 3 (row) or 7 (column) levels.
#[test]
#[ignore = "GPU"]
fn filters_on_interior_subview_match_host_reference() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let ctx = NppContext::new(&stream).expect("NppContext");
    let full = Size {
        width: 16,
        height: 16,
    };
    let inner = Rect {
        x: 1,
        y: 1,
        width: 14,
        height: 14,
    };
    let inner_size = Size {
        width: 14,
        height: 14,
    };

    for ch in [1u32, 3] {
        let src_host = gradient(16, 16, ch);
        let src_dev = DeviceBuffer::from_host(&stream, &src_host).expect("src");
        let src_full = ImageView::packed(&src_dev, full, ch).expect("src view");
        let src_inner = src_full.subview(inner).expect("subview");
        let n = (14 * 14 * ch) as usize;

        let mut box_dev = DeviceBuffer::<u8>::zeroed(&stream, n).expect("box dst");
        let mut box_m = ImageViewMut::packed(&mut box_dev, inner_size, ch).expect("box view");
        let mask = Size {
            width: 3,
            height: 3,
        };
        if ch == 1 {
            image::filter_box_8u_c1(&ctx, &src_inner, &mut box_m, mask, (1, 1)).expect("box c1");
        } else {
            image::filter_box_8u_c3(&ctx, &src_inner, &mut box_m, mask, (1, 1)).expect("box c3");
        }

        let mut gauss_dev = DeviceBuffer::<u8>::zeroed(&stream, n).expect("gauss dst");
        let mut gauss_m = ImageViewMut::packed(&mut gauss_dev, inner_size, ch).expect("gauss view");
        if ch == 1 {
            image::filter_gauss_8u_c1(&ctx, &src_inner, &mut gauss_m, MaskSize::X3x3)
                .expect("gauss c1");
        } else {
            image::filter_gauss_8u_c3(&ctx, &src_inner, &mut gauss_m, MaskSize::X3x3)
                .expect("gauss c3");
        }
        stream.synchronize().expect("sync");

        let box_host = box_dev.to_host_vec(&stream).expect("box readback");
        let gauss_host = gauss_dev.to_host_vec(&stream).expect("gauss readback");
        for y in 0..14u32 {
            for x in 0..14u32 {
                for c in 0..ch {
                    let i = ((y * 14 + x) * ch + c) as usize;
                    let expected = ((x + 1) * 7 + (y + 1) * 3 + c * 10) as u8;
                    assert!(
                        box_host[i].abs_diff(expected) <= 1,
                        "box ch={ch} ({x},{y},{c}): {} vs {expected}",
                        box_host[i]
                    );
                    assert!(
                        gauss_host[i].abs_diff(expected) <= 1,
                        "gauss ch={ch} ({x},{y},{c}): {} vs {expected}",
                        gauss_host[i]
                    );
                }
            }
        }
    }
}

/// NPP-4: `resize_8u_c1`, `resize_8u_c4`, `resize_32f_c3` and `threshold_val_32f_c1` run on
/// hardware; a constant image must stay constant under any interpolation.
#[test]
#[ignore = "GPU"]
fn resize_variants_and_float_threshold() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let ctx = NppContext::new(&stream).expect("NppContext");
    let src_size = Size {
        width: 16,
        height: 16,
    };
    let dst_size = Size {
        width: 8,
        height: 8,
    };

    for ch in [1u32, 4] {
        let src_dev =
            DeviceBuffer::from_host(&stream, &vec![77u8; (16 * 16 * ch) as usize]).expect("src");
        let mut dst_dev = DeviceBuffer::<u8>::zeroed(&stream, (8 * 8 * ch) as usize).expect("dst");
        let src_v = ImageView::packed(&src_dev, src_size, ch).expect("src view");
        let mut dst_m = ImageViewMut::packed(&mut dst_dev, dst_size, ch).expect("dst view");
        if ch == 1 {
            image::resize_8u_c1(
                &ctx,
                &src_v,
                Rect::full(src_size),
                &mut dst_m,
                Rect::full(dst_size),
                Interpolation::Linear,
            )
            .expect("resize c1");
        } else {
            image::resize_8u_c4(
                &ctx,
                &src_v,
                Rect::full(src_size),
                &mut dst_m,
                Rect::full(dst_size),
                Interpolation::Linear,
            )
            .expect("resize c4");
        }
        stream.synchronize().expect("sync");
        let out = dst_dev.to_host_vec(&stream).expect("readback");
        assert!(out.iter().all(|&v| v == 77), "resize ch={ch}: {out:?}");
    }

    let src_f = DeviceBuffer::from_host(&stream, &[0.25f32; 16 * 16 * 3]).expect("src f32");
    let mut dst_f = DeviceBuffer::<f32>::zeroed(&stream, 8 * 8 * 3).expect("dst f32");
    let src_fv = ImageView::packed(&src_f, src_size, 3).expect("src f32 view");
    let mut dst_fm = ImageViewMut::packed(&mut dst_f, dst_size, 3).expect("dst f32 view");
    image::resize_32f_c3(
        &ctx,
        &src_fv,
        Rect::full(src_size),
        &mut dst_fm,
        Rect::full(dst_size),
        Interpolation::Linear,
    )
    .expect("resize 32f c3");

    let thr_size = Size {
        width: 16,
        height: 1,
    };
    let thr_host: Vec<f32> = (0..16u8).map(f32::from).collect();
    let thr_src = DeviceBuffer::from_host(&stream, &thr_host).expect("thr src");
    let mut thr_dst = DeviceBuffer::<f32>::zeroed(&stream, 16).expect("thr dst");
    let thr_v = ImageView::packed(&thr_src, thr_size, 1).expect("thr view");
    let mut thr_m = ImageViewMut::packed(&mut thr_dst, thr_size, 1).expect("thr dst view");
    image::threshold_val_32f_c1(&ctx, &thr_v, &mut thr_m, 7.5, 100.0, CmpOp::Greater)
        .expect("threshold 32f");
    stream.synchronize().expect("sync");

    let resized = dst_f.to_host_vec(&stream).expect("f32 readback");
    assert!(
        resized.iter().all(|&v| (v - 0.25).abs() < 1e-5),
        "resize 32f: {resized:?}"
    );
    let thr = thr_dst.to_host_vec(&stream).expect("thr readback");
    for (i, &v) in thr.iter().enumerate() {
        let expected = if i > 7 { 100.0 } else { i as f32 };
        assert_eq!(v, expected, "threshold 32f at {i}");
    }
}
