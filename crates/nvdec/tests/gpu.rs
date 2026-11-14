//! GPU tests (T2/T3) for `nvdec`: NVENC encode -> NVDEC decode -> PSNR check.
//!
//! Run with `cargo test -p nvdec -- --include-ignored`.

use cuda_core::Context;
use nvdec::{DecoderConfig, DisplayInfo, ParserConfig, ParserHandler, SurfaceFormat, VideoDecoder, VideoFormat, VideoParser, nv12_frame_bytes, psnr_u8};
use nvenc::{BufferFormat, Codec as EncCodec, EncodeFlags, EncodeInput, EncodeSession, InputBuffer, OutputBitstream, Preset, SessionConfig, Tuning};
use std::sync::Arc;

struct RoundTripCollector {
    cuda: Arc<Context>,
    stream: Arc<cuda_core::Stream>,
    decoder: Option<Arc<VideoDecoder>>,
    decoded_frames: Vec<Vec<u8>>,
    scratch_host: Vec<u8>,
}

impl ParserHandler for RoundTripCollector {
    fn on_sequence(&mut self, fmt: &VideoFormat) -> nvdec::Result<u32> {
        let cfg = DecoderConfig::from_video_format(fmt, SurfaceFormat::Nv12)?;
        self.decoder = Some(VideoDecoder::new(&self.cuda, &cfg)?);
        let frame_bytes = nv12_frame_bytes(cfg.target_width, cfg.target_height).expect("frame bytes");
        self.scratch_host.resize(frame_bytes, 0);
        Ok(cfg.num_decode_surfaces)
    }

    fn on_decode(&mut self, pic: &nvdec::sys::CUVIDPICPARAMS) -> nvdec::Result<()> {
        let dec = self.decoder.as_ref().ok_or(nvdec::Error::Invalid("decode before sequence"))?;
        // SAFETY: `pic` is provided by `VideoParser::parse_packet` and remains valid for this synchronous call.
        unsafe { dec.decode_picture(pic) }
    }

    fn on_display(&mut self, disp: Option<&DisplayInfo>) -> nvdec::Result<()> {
        let Some(disp) = disp else {
            return Ok(());
        };
        let dec = self.decoder.as_ref().ok_or(nvdec::Error::Invalid("display before sequence"))?;
        let mapped = dec.map_frame(disp, Some(&self.stream))?;
        mapped.copy_nv12_to_host(&self.stream, &mut self.scratch_host)?;
        self.decoded_frames.push(self.scratch_host.clone());
        Ok(())
    }
}

#[test]
#[ignore = "GPU: needs NVIDIA driver with NVENC + NVDEC; run with --ignored"]
fn nvenc_to_nvdec_h264_roundtrip_psnr() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let width = 256u32;
    let height = 256u32;
    let num_frames = 4u32;

    let mut enc = EncodeSession::open(&cuda).expect("open NVENC");
    let mut preset = enc.preset_config(EncCodec::H264, Preset::P4, Tuning::HighQuality).expect("preset");
    preset.set_gop_length(10);
    preset.set_frame_interval_p(1);
    let mut cfg = SessionConfig::new(EncCodec::H264, width, height, 30, 1);
    cfg.encode_config = Some(preset);
    enc.initialize(&cfg).expect("init NVENC");

    let mut input = InputBuffer::new(&enc, width, height, BufferFormat::Nv12).expect("input");
    let mut output = OutputBitstream::new(&enc).expect("output");

    let mut reference_frames = Vec::with_capacity(num_frames as usize);
    let mut packets = Vec::with_capacity(num_frames as usize);

    for frame_idx in 0..num_frames {
        let mut ref_nv12 = vec![128u8; nv12_frame_bytes(width, height).expect("size")];
        for y in 0..height as usize {
            for x in 0..width as usize {
                ref_nv12[y * width as usize + x] = ((x * 2 + y + frame_idx as usize * 12) & 0xff) as u8;
            }
        }
        {
            let mut lock = input.lock().expect("lock input");
            let pitch = lock.pitch() as usize;
            let rows = height + height / 2;
            let dst = lock.as_mut_slice(rows).expect("slice");
            for r in 0..rows as usize {
                let src_row = &ref_nv12[r * width as usize..(r + 1) * width as usize];
                dst[r * pitch..r * pitch + width as usize].copy_from_slice(src_row);
            }
        }
        let flags = EncodeFlags {
            force_idr: frame_idx == 0,
            output_sps_pps: frame_idx == 0,
            ..Default::default()
        };
        enc.encode_picture(EncodeInput::Buffer(&input), &mut output, frame_idx, u64::from(frame_idx), flags)
            .expect("encode_picture");
        let pkt = output.lock().expect("lock bitstream");
        packets.push(pkt.data().to_vec());
        reference_frames.push(ref_nv12);
    }
    enc.flush_eos().expect("flush_eos");

    let collector = RoundTripCollector {
        cuda: cuda.clone(),
        stream,
        decoder: None,
        decoded_frames: Vec::new(),
        scratch_host: Vec::new(),
    };
    let mut parser = VideoParser::new(&cuda, &ParserConfig::new(nvdec::Codec::H264), collector).expect("create parser");
    for (idx, pkt) in packets.iter().enumerate() {
        parser.parse_packet(pkt, Some(idx as i64)).expect("parse_packet");
    }
    parser.flush_eos().expect("flush_eos");

    let decoded = &parser.handler().decoded_frames;
    assert_eq!(decoded.len(), reference_frames.len(), "decoded frame count mismatch");
    for (i, (ref_f, dec_f)) in reference_frames.iter().zip(decoded.iter()).enumerate() {
        let psnr = psnr_u8(ref_f, dec_f).expect("psnr");
        assert!(psnr >= 30.0, "frame {i} PSNR {psnr:.2} dB below 30 dB threshold");
    }
}
