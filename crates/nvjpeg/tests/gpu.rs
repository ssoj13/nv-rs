//! GPU tests (T2/T3) for `nvjpeg`: interleaved and planar encode + decode PSNR check,
//! metadata/EXIF parsing, and reusable pinned/device buffer attachment.

use cuda_core::{Context, DeviceBuffer};
use nvjpeg::{
    ChromaSubsampling, EncoderParams, EncoderState, ExifOrientation, Handle, ImagePlanes, ImagePlanesMut, InputFormat, JpegEncoding, JpegState, JpegStream,
    NvjpegDeviceBuffer, OutputFormat, PinnedBuffer,
};

fn psnr_u8(a: &[u8], b: &[u8]) -> f64 {
    assert_eq!(a.len(), b.len());
    let mse: f64 = a.iter().zip(b.iter()).map(|(&x, &y)| {
        let d = f64::from(x) - f64::from(y);
        d * d
    }).sum::<f64>() / a.len() as f64;
    if mse == 0.0 {
        return f64::INFINITY;
    }
    10.0 * (255.0 * 255.0 / mse).log10()
}

#[test]
#[ignore = "GPU"]
fn encode_decode_psnr_interleaved_and_planar() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("nvjpeg Handle");

    let w = 64u32;
    let h = 64u32;
    let mut rgb_host = vec![0u8; (w * h * 3) as usize];
    for y in 0..h {
        for x in 0..w {
            let idx = ((y * w + x) * 3) as usize;
            rgb_host[idx] = (x * 4) as u8;
            rgb_host[idx + 1] = (y * 4) as u8;
            rgb_host[idx + 2] = ((x + y) * 2) as u8;
        }
    }

    let src_dev = DeviceBuffer::from_host(&stream, &rgb_host).expect("src_dev");
    let src_planes = ImagePlanes::interleaved(&src_dev, w, h, 3, (w * 3) as usize).expect("planes");

    let mut params = EncoderParams::new(&handle, &stream).expect("params");
    params.set_quality(95, &stream).expect("quality");
    params.set_sampling_factors(ChromaSubsampling::Yuv444, &stream).expect("444");
    params.set_encoding(JpegEncoding::BaselineDct, &stream).expect("baseline");
    assert!(params.max_bitstream_size(w, h).expect("max_bitstream_size") > 0);

    let mut enc = EncoderState::new(&handle, &stream).expect("encoder");
    enc.encode_image(&params, &src_planes, InputFormat::Rgbi, w, h, &stream).expect("encode");
    let jpeg = enc.retrieve_bitstream(&stream).expect("bitstream");
    assert!(jpeg.starts_with(&[0xFF, 0xD8]), "expected JPEG SOI marker");

    // Metadata inspection via Handle::image_info and JpegStream.
    let info = handle.image_info(&jpeg).expect("image_info");
    assert_eq!(info.components, 3);
    assert_eq!(info.subsampling, ChromaSubsampling::Yuv444);
    assert_eq!(info.widths[0], w);
    assert_eq!(info.heights[0], h);

    let mut jstream = JpegStream::new(&handle).expect("JpegStream");
    jstream.parse(&jpeg, true, false).expect("parse");
    assert_eq!(jstream.dimensions().expect("dims"), (w, h));
    assert_eq!(jstream.components_num().expect("comps"), 3);
    assert_eq!(jstream.chroma_subsampling().expect("css"), ChromaSubsampling::Yuv444);
    assert_eq!(jstream.exif_orientation().expect("exif"), ExifOrientation::Unknown);
    assert_eq!(jstream.sample_precision().expect("prec"), 8);
    assert_eq!(jstream.jpeg_encoding().expect("enc"), JpegEncoding::BaselineDct);

    // Decode interleaved with attached pinned/device buffers.
    let pinned = PinnedBuffer::new(&handle).expect("pinned");
    pinned.resize(64 * 1024, &stream).expect("resize pinned");
    let dev_scratch = NvjpegDeviceBuffer::new(&handle).expect("dev_scratch");
    dev_scratch.resize(64 * 1024, &stream).expect("resize dev");

    let mut dec = JpegState::new(&handle).expect("JpegState");
    dec.attach_pinned(&pinned).expect("attach pinned");
    dec.attach_device(&dev_scratch).expect("attach device");

    let mut dst_rgbi = DeviceBuffer::from_host(&stream, &vec![0u8; rgb_host.len()]).expect("dst_rgbi");
    let mut dst_planes = ImagePlanesMut::interleaved(&mut dst_rgbi, w, h, 3, (w * 3) as usize).expect("dst_planes");
    dec.decode(&jpeg, OutputFormat::Rgbi, &mut dst_planes, &stream).expect("decode rgbi");
    stream.synchronize().expect("sync");

    let decoded_rgbi = dst_rgbi.to_host_vec(&stream).expect("readback rgbi");
    let psnr = psnr_u8(&rgb_host, &decoded_rgbi);
    assert!(psnr > 35.0, "interleaved decode PSNR {psnr:.2} dB too low");

    // Decode planar RGB.
    let plane_bytes = (w * h) as usize;
    let mut r_dev = DeviceBuffer::from_host(&stream, &vec![0u8; plane_bytes]).expect("r");
    let mut g_dev = DeviceBuffer::from_host(&stream, &vec![0u8; plane_bytes]).expect("g");
    let mut b_dev = DeviceBuffer::from_host(&stream, &vec![0u8; plane_bytes]).expect("b");
    let mut planar_dst = ImagePlanesMut::planar3(
        [(&mut r_dev, w as usize, w, h), (&mut g_dev, w as usize, w, h), (&mut b_dev, w as usize, w, h)],
        w,
        h,
    )
    .expect("planar_dst");
    dec.decode(&jpeg, OutputFormat::Rgb, &mut planar_dst, &stream).expect("decode planar");
    stream.synchronize().expect("sync planar");

    let r_host = r_dev.to_host_vec(&stream).expect("r readback");
    assert_eq!(r_host.len(), plane_bytes);
    assert!(r_host[0] < r_host[(w - 1) as usize]);
}
