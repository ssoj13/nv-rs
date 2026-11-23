//! GPU tests (T2/T3) for `nvjpeg`: interleaved and planar encode + decode PSNR check,
//! metadata/EXIF parsing, and reusable pinned/device buffer attachment.

use cuda_core::{Context, DeviceBuffer, Stream};
use nvjpeg::{
    Backend, ChromaSubsampling, DecodeParams, EncoderParams, EncoderState, ExifOrientation, Handle,
    ImagePlanes, ImagePlanesMut, InputFormat, JpegDecoder, JpegEncoding, JpegState, JpegStream,
    NvjpegDeviceBuffer, OutputFormat, PinnedBuffer,
};
use std::sync::Arc;

fn psnr_u8(a: &[u8], b: &[u8]) -> f64 {
    assert_eq!(a.len(), b.len());
    let mse: f64 = a
        .iter()
        .zip(b.iter())
        .map(|(&x, &y)| {
            let d = f64::from(x) - f64::from(y);
            d * d
        })
        .sum::<f64>()
        / a.len() as f64;
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
    params
        .set_sampling_factors(ChromaSubsampling::Yuv444, &stream)
        .expect("444");
    params
        .set_encoding(JpegEncoding::BaselineDct, &stream)
        .expect("baseline");
    assert!(params.max_bitstream_size(w, h).expect("max_bitstream_size") > 0);

    let mut enc = EncoderState::new(&handle, &stream).expect("encoder");
    enc.encode_image(&params, &src_planes, InputFormat::Rgbi, w, h, &stream)
        .expect("encode");
    let jpeg = enc.retrieve_bitstream(&stream).expect("bitstream");
    assert!(jpeg.starts_with(&[0xFF, 0xD8]), "expected JPEG SOI marker");

    // Metadata inspection via Handle::image_info and JpegStream.
    let info = handle.image_info(&jpeg).expect("image_info");
    assert_eq!(info.components, 3);
    assert_eq!(info.subsampling, ChromaSubsampling::Yuv444);
    assert_eq!(info.widths[0], w);
    assert_eq!(info.heights[0], h);

    let mut jstream = JpegStream::new(&handle).expect("JpegStream");
    jstream.parse(&jpeg, true, true).expect("parse");
    assert_eq!(jstream.dimensions().expect("dims"), (w, h));
    assert_eq!(jstream.components_num().expect("comps"), 3);
    assert_eq!(
        jstream.chroma_subsampling().expect("css"),
        ChromaSubsampling::Yuv444
    );
    assert_eq!(
        jstream.exif_orientation().expect("exif"),
        ExifOrientation::Unknown
    );
    assert_eq!(jstream.sample_precision().expect("prec"), 8);
    assert_eq!(
        jstream.jpeg_encoding().expect("enc"),
        JpegEncoding::BaselineDct
    );

    // Decode interleaved with decoupled v2 decoder and attached pinned/device buffers.
    let pinned = PinnedBuffer::new(&handle).expect("pinned");
    pinned.resize(64 * 1024, &stream).expect("resize pinned");
    let dev_scratch = NvjpegDeviceBuffer::new(&handle).expect("dev_scratch");
    dev_scratch.resize(64 * 1024, &stream).expect("resize dev");

    let decoder = JpegDecoder::new(&handle, Backend::Default).expect("JpegDecoder");
    let mut dec = JpegState::with_decoder(&decoder).expect("JpegState::with_decoder");
    dec.attach_pinned(&pinned).expect("attach pinned");
    dec.attach_device(&dev_scratch).expect("attach device");

    let mut dec_params = DecodeParams::new(&handle).expect("DecodeParams");
    dec_params
        .set_output_format(OutputFormat::Rgbi)
        .expect("set_output_format rgbi");

    let mut dst_rgbi =
        DeviceBuffer::from_host(&stream, &vec![0u8; rgb_host.len()]).expect("dst_rgbi");
    let mut dst_planes =
        ImagePlanesMut::interleaved(&mut dst_rgbi, w, h, 3, (w * 3) as usize).expect("dst_planes");
    dec.decode_jpeg(&jstream, &dec_params, &mut dst_planes, &stream)
        .expect("decode rgbi");
    stream.synchronize().expect("sync");

    let decoded_rgbi = dst_rgbi.to_host_vec(&stream).expect("readback rgbi");
    let psnr = psnr_u8(&rgb_host, &decoded_rgbi);
    assert!(psnr > 35.0, "interleaved decode PSNR {psnr:.2} dB too low");

    // Decode planar RGB.
    dec_params
        .set_output_format(OutputFormat::Rgb)
        .expect("set_output_format rgb");
    let plane_bytes = (w * h) as usize;
    let mut r_dev = DeviceBuffer::from_host(&stream, &vec![0u8; plane_bytes]).expect("r");
    let mut g_dev = DeviceBuffer::from_host(&stream, &vec![0u8; plane_bytes]).expect("g");
    let mut b_dev = DeviceBuffer::from_host(&stream, &vec![0u8; plane_bytes]).expect("b");
    let mut planar_dst = ImagePlanesMut::planar3(
        [
            (&mut r_dev, w as usize, w, h),
            (&mut g_dev, w as usize, w, h),
            (&mut b_dev, w as usize, w, h),
        ],
        w,
        h,
    )
    .expect("planar_dst");
    dec.decode_jpeg(&jstream, &dec_params, &mut planar_dst, &stream)
        .expect("decode planar");
    stream.synchronize().expect("sync planar");

    let r_host = r_dev.to_host_vec(&stream).expect("r readback");
    assert_eq!(r_host.len(), plane_bytes);
    assert!(r_host[0] < r_host[(w - 1) as usize]);
}

/// Encodes a `w x h` RGB pattern as a 4:4:4 JPEG.
fn encode_test_jpeg(handle: &Arc<Handle>, stream: &Arc<Stream>, w: u32, h: u32) -> Vec<u8> {
    let rgb_host: Vec<u8> = (0..w * h * 3).map(|i| (i % 251) as u8).collect();
    let src_dev = DeviceBuffer::from_host(stream, &rgb_host).expect("src_dev");
    let src = ImagePlanes::interleaved(&src_dev, w, h, 3, (w * 3) as usize).expect("planes");
    let mut params = EncoderParams::new(handle, stream).expect("params");
    params
        .set_sampling_factors(ChromaSubsampling::Yuv444, stream)
        .expect("444");
    let mut enc = EncoderState::new(handle, stream).expect("encoder");
    enc.encode_image(&params, &src, InputFormat::Rgbi, w, h, stream)
        .expect("encode");
    enc.retrieve_bitstream(stream).expect("bitstream")
}

/// JPG-1: planes sized for a smaller image than the bitstream are rejected before nvJPEG writes
/// out of bounds (previously only the caller's own width and height were checked).
#[test]
#[ignore = "GPU"]
fn decode_rejects_planes_smaller_than_bitstream() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("nvjpeg Handle");
    let jpeg = encode_test_jpeg(&handle, &stream, 64, 64);

    let mut small = DeviceBuffer::<u8>::zeroed(&stream, 16 * 16 * 3).expect("small");
    let mut planes = ImagePlanesMut::interleaved(&mut small, 16, 16, 3, 16 * 3).expect("planes");
    let mut state = JpegState::new(&handle).expect("JpegState");
    let err = state
        .decode(&jpeg, OutputFormat::Rgbi, &mut planes, &stream)
        .expect_err("undersized planes must be rejected");
    assert!(matches!(err, nvjpeg::Error::InvalidInput(_)), "{err}");

    let decoder = JpegDecoder::new(&handle, Backend::Default).expect("JpegDecoder");
    let mut dec = JpegState::with_decoder(&decoder).expect("with_decoder");
    let mut params = DecodeParams::new(&handle).expect("DecodeParams");
    params.set_output_format(OutputFormat::Rgbi).expect("rgbi");
    let mut jstream = JpegStream::new(&handle).expect("JpegStream");
    jstream.parse(&jpeg, false, true).expect("parse");
    let err = dec
        .decode_jpeg(&jstream, &params, &mut planes, &stream)
        .expect_err("undersized planes must be rejected");
    assert!(matches!(err, nvjpeg::Error::InvalidInput(_)), "{err}");
}

/// JPG-3: a parse that would leave nvJPEG pointing at the borrowed bitstream is refused, and a
/// header-only parse cannot be decoded.
#[test]
#[ignore = "GPU"]
fn jpeg_stream_refuses_borrowed_bitstream_and_header_only_decode() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("nvjpeg Handle");
    let jpeg = encode_test_jpeg(&handle, &stream, 64, 64);

    let mut jstream = JpegStream::new(&handle).expect("JpegStream");
    assert!(matches!(
        jstream.parse(&jpeg, false, false),
        Err(nvjpeg::Error::InvalidInput(_))
    ));

    jstream.parse_header(&jpeg).expect("parse_header");
    assert_eq!(jstream.dimensions().expect("dims"), (64, 64));
    let decoder = JpegDecoder::new(&handle, Backend::Default).expect("JpegDecoder");
    let mut dec = JpegState::with_decoder(&decoder).expect("with_decoder");
    let mut params = DecodeParams::new(&handle).expect("DecodeParams");
    params.set_output_format(OutputFormat::Rgbi).expect("rgbi");
    let mut dst = DeviceBuffer::<u8>::zeroed(&stream, 64 * 64 * 3).expect("dst");
    let mut planes = ImagePlanesMut::interleaved(&mut dst, 64, 64, 3, 64 * 3).expect("planes");
    assert!(matches!(
        dec.decode_jpeg(&jstream, &params, &mut planes, &stream),
        Err(nvjpeg::Error::InvalidInput(_))
    ));

    // A full parse copies the bitstream, so decoding after the source is dropped is valid.
    // A decoder-bound state needs attached pinned and device buffers before `nvjpegDecodeJpeg`
    // (without them nvJPEG returns NVJPEG_STATUS_INVALID_PARAMETER).
    let pinned = PinnedBuffer::new(&handle).expect("pinned");
    pinned.resize(64 * 1024, &stream).expect("resize pinned");
    let device = NvjpegDeviceBuffer::new(&handle).expect("device");
    device.resize(64 * 1024, &stream).expect("resize device");
    dec.attach_pinned(&pinned).expect("attach pinned");
    dec.attach_device(&device).expect("attach device");
    let copy = jpeg.clone();
    jstream.parse(&copy, false, true).expect("parse");
    drop(copy);
    dec.decode_jpeg(&jstream, &params, &mut planes, &stream)
        .expect("decode after source dropped");
    stream.synchronize().expect("sync");
}

/// JPG-4: a pinned or device scratch buffer backs one `JpegState` at a time, cannot be resized
/// while attached, and is released when its state drops.
#[test]
#[ignore = "GPU"]
fn scratch_buffers_attach_to_one_state_at_a_time() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("nvjpeg Handle");
    let decoder = JpegDecoder::new(&handle, Backend::Default).expect("JpegDecoder");

    let pinned = PinnedBuffer::new(&handle).expect("pinned");
    pinned.resize(64 * 1024, &stream).expect("resize pinned");
    let device = NvjpegDeviceBuffer::new(&handle).expect("device");
    device.resize(64 * 1024, &stream).expect("resize device");

    let mut a = JpegState::with_decoder(&decoder).expect("state a");
    let mut b = JpegState::with_decoder(&decoder).expect("state b");
    a.attach_pinned(&pinned).expect("attach pinned to a");
    a.attach_device(&device).expect("attach device to a");
    a.attach_pinned(&pinned)
        .expect("re-attaching to the same state is a no-op");

    assert!(matches!(
        b.attach_pinned(&pinned),
        Err(nvjpeg::Error::InvalidInput(_))
    ));
    assert!(matches!(
        b.attach_device(&device),
        Err(nvjpeg::Error::InvalidInput(_))
    ));
    assert!(matches!(
        pinned.resize(128 * 1024, &stream),
        Err(nvjpeg::Error::InvalidInput(_))
    ));
    assert!(matches!(
        device.resize(128 * 1024, &stream),
        Err(nvjpeg::Error::InvalidInput(_))
    ));

    drop(a);
    b.attach_pinned(&pinned)
        .expect("attach pinned to b after a dropped");
    b.attach_device(&device)
        .expect("attach device to b after a dropped");
}

/// JPG-5: every nvJPEG object binds its CUDA context in `Drop`, so tearing down on a thread with
/// no current context records no error.
#[test]
#[ignore = "GPU"]
fn drops_without_current_context_record_no_error() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("nvjpeg Handle");
    let decoder = JpegDecoder::new(&handle, Backend::Default).expect("JpegDecoder");
    let pinned = PinnedBuffer::new(&handle).expect("pinned");
    pinned.resize(64 * 1024, &stream).expect("resize pinned");
    let device = NvjpegDeviceBuffer::new(&handle).expect("device");
    device.resize(64 * 1024, &stream).expect("resize device");
    let mut state = JpegState::with_decoder(&decoder).expect("state");
    state.attach_pinned(&pinned).expect("attach pinned");
    state.attach_device(&device).expect("attach device");
    let legacy = JpegState::new(&handle).expect("legacy state");
    let params = DecodeParams::new(&handle).expect("DecodeParams");
    let jstream = JpegStream::new(&handle).expect("JpegStream");
    let enc_params = EncoderParams::new(&handle, &stream).expect("EncoderParams");
    let enc_state = EncoderState::new(&handle, &stream).expect("EncoderState");
    cuda.check_err().expect("no error before teardown");

    // SAFETY: clearing the calling thread's current context is always valid.
    let rc = unsafe { cuda_core::sys::cuCtxSetCurrent(std::ptr::null_mut()) };
    assert_eq!(rc, cuda_core::sys::cudaError_enum_CUDA_SUCCESS);
    drop((enc_state, enc_params, jstream, params, legacy, state));
    drop((pinned, device, decoder, handle));
    cuda.check_err()
        .expect("teardown without a current context recorded an error");
}

/// JPG-6: `retrieve_bitstream_into` reuses a stale, larger `Vec` and returns exactly the encoded
/// bytes. nvJPEG hands out an encode's bitstream once (a second retrieval writes nothing), so the
/// image is encoded twice and each encode is retrieved once.
#[test]
#[ignore = "GPU"]
fn retrieve_bitstream_into_reuses_stale_vec() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("nvjpeg Handle");
    let (w, h) = (64u32, 64u32);
    let rgb_host: Vec<u8> = (0..w * h * 3).map(|i| (i % 251) as u8).collect();
    let src_dev = DeviceBuffer::from_host(&stream, &rgb_host).expect("src_dev");
    let src = ImagePlanes::interleaved(&src_dev, w, h, 3, (w * 3) as usize).expect("planes");
    let mut params = EncoderParams::new(&handle, &stream).expect("params");
    // nvJPEG rejects `nvjpegEncodeImage` with default (unset) sampling factors.
    params
        .set_sampling_factors(ChromaSubsampling::Yuv444, &stream)
        .expect("444");
    let mut enc = EncoderState::new(&handle, &stream).expect("encoder");
    enc.encode_image(&params, &src, InputFormat::Rgbi, w, h, &stream)
        .expect("encode");

    let fresh = enc.retrieve_bitstream(&stream).expect("fresh");
    let mut reused = vec![0xAAu8; fresh.len() * 4];
    enc.encode_image(&params, &src, InputFormat::Rgbi, w, h, &stream)
        .expect("encode again");
    enc.retrieve_bitstream_into(&mut reused, &stream)
        .expect("reused");
    assert_eq!(reused, fresh);
}

/// nvJPEG hands out an encode's bitstream once (a second retrieval succeeds but writes nothing),
/// so the wrapper rejects a retrieval without a fresh encode instead of returning zeros.
#[test]
#[ignore = "GPU"]
fn second_retrieve_of_one_encode_is_rejected() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let handle = Handle::new(&cuda).expect("nvjpeg Handle");
    let (w, h) = (64u32, 64u32);
    let rgb_host: Vec<u8> = (0..w * h * 3).map(|i| (i % 251) as u8).collect();
    let src_dev = DeviceBuffer::from_host(&stream, &rgb_host).expect("src_dev");
    let src = ImagePlanes::interleaved(&src_dev, w, h, 3, (w * 3) as usize).expect("planes");
    let mut params = EncoderParams::new(&handle, &stream).expect("params");
    params
        .set_sampling_factors(ChromaSubsampling::Yuv444, &stream)
        .expect("444");
    let mut enc = EncoderState::new(&handle, &stream).expect("encoder");
    assert!(matches!(
        enc.retrieve_bitstream(&stream),
        Err(nvjpeg::Error::InvalidInput(_))
    ));

    enc.encode_image(&params, &src, InputFormat::Rgbi, w, h, &stream)
        .expect("encode");
    let first = enc.retrieve_bitstream(&stream).expect("first retrieve");
    assert!(first.len() > 4 && first[..2] == [0xFF, 0xD8], "not a JPEG");

    assert!(matches!(
        enc.retrieve_bitstream(&stream),
        Err(nvjpeg::Error::InvalidInput(_))
    ));
    let mut kept = vec![0xAAu8; 16];
    assert!(matches!(
        enc.retrieve_bitstream_into(&mut kept, &stream),
        Err(nvjpeg::Error::InvalidInput(_))
    ));
    assert_eq!(
        kept,
        vec![0xAAu8; 16],
        "a rejected retrieval must not touch `out`"
    );

    enc.encode_image(&params, &src, InputFormat::Rgbi, w, h, &stream)
        .expect("encode again");
    assert_eq!(
        enc.retrieve_bitstream(&stream).expect("after re-encode"),
        first
    );
}
