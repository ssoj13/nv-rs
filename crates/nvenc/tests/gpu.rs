//! GPU tests (T2) for `nvenc`: run with `cargo test -p nvenc -- --include-ignored`.

use cuda_core::Context;
use nvenc::{BufferFormat, Codec as EncCodec, EncodeFlags, EncodeInput, EncodeSession, InputBuffer, OutputBitstream, Preset, SessionConfig, Tuning};

#[test]
#[ignore = "GPU: needs NVIDIA driver with NVENC; run with --ignored"]
fn encode_synthetic_nv12_h264_hevc_av1() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let width = 256u32;
    let height = 256u32;

    for codec in [EncCodec::H264, EncCodec::Hevc, EncCodec::Av1] {
        let mut session = EncodeSession::open(&cuda).expect("open NVENC session");
        match session.caps(codec, nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_WIDTH_MAX) {
            Ok(w) if w > 0 => {}
            Ok(_) | Err(nvenc::Error::Status { .. }) if codec == EncCodec::Av1 => continue,
            Ok(w) => panic!("caps({codec:?}, WIDTH_MAX) returned non-positive {w}"),
            Err(e) => panic!("caps({codec:?}, WIDTH_MAX) failed: {e}"),
        }
        let mut preset = match session.preset_config(codec, Preset::P4, Tuning::HighQuality) {
            Ok(p) => p,
            Err(nvenc::Error::Status { .. }) if codec == EncCodec::Av1 => continue,
            Err(e) => panic!("preset_config({codec:?}) failed: {e}"),
        };
        preset.set_gop_length(10);
        preset.set_frame_interval_p(1);
        let mut cfg = SessionConfig::new(codec, width, height, 30, 1);
        cfg.encode_config = Some(preset);
        session.initialize(&cfg).expect("initialize encoder");

        let mut input = InputBuffer::new(&session, width, height, BufferFormat::Nv12).expect("input buffer");
        let mut output = OutputBitstream::new(&session).expect("output bitstream");
        let mut total_bytes = 0usize;

        for frame_idx in 0..4u32 {
            {
                let mut lock = input.lock().expect("lock input");
                let pitch = lock.pitch() as usize;
                let rows = height + height / 2;
                let slice = lock.as_mut_slice(rows).expect("mut slice");
                for y in 0..height as usize {
                    let row = &mut slice[y * pitch..y * pitch + width as usize];
                    for (x, px) in row.iter_mut().enumerate() {
                        *px = ((x + y + frame_idx as usize * 8) & 0xff) as u8;
                    }
                }
                let uv_offset = (height as usize) * pitch;
                for y in 0..(height as usize / 2) {
                    let row = &mut slice[uv_offset + y * pitch..uv_offset + y * pitch + width as usize];
                    row.fill(128);
                }
            }
            let flags = EncodeFlags { force_idr: frame_idx == 0, output_sps_pps: frame_idx == 0, ..Default::default() };
            session
                .encode_picture(EncodeInput::Buffer(&input), &mut output, frame_idx, u64::from(frame_idx), flags)
                .expect("encode_picture");
            let pkt = output.lock().expect("lock bitstream");
            total_bytes += pkt.data().len();
        }
        session.flush_eos().expect("flush_eos");
        assert!(total_bytes > 0, "{codec:?} produced empty bitstream");
    }
}
