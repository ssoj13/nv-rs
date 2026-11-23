//! GPU tests (T2) for `nvenc`: run with `cargo test -p nvenc -- --include-ignored`.

use cuda_core::{Context, DriverError};
use nvenc::{
    BufferFormat, Codec as EncCodec, EncodeFlags, EncodeInput, EncodeOutcome, EncodeSession,
    InputBuffer, OutputBitstream, Preset, SessionConfig, Tuning,
};
use std::collections::VecDeque;
use std::sync::Arc;

/// Writes a moving NV12 luma gradient with flat chroma into `input` (pitch from the lock).
fn fill_nv12(input: &mut InputBuffer, width: u32, height: u32, frame_idx: u32) {
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

fn idr_flags(frame_idx: u32) -> EncodeFlags {
    EncodeFlags {
        force_idr: frame_idx == 0,
        output_sps_pps: frame_idx == 0,
        ..Default::default()
    }
}

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

        let mut input =
            InputBuffer::new(&session, width, height, BufferFormat::Nv12).expect("input buffer");
        let mut output = OutputBitstream::new(&session).expect("output bitstream");
        let mut total_bytes = 0usize;

        for frame_idx in 0..4u32 {
            fill_nv12(&mut input, width, height, frame_idx);
            let outcome = session
                .encode_picture(
                    EncodeInput::Buffer(&input),
                    &mut output,
                    frame_idx,
                    u64::from(frame_idx),
                    idr_flags(frame_idx),
                )
                .expect("encode_picture");
            assert_eq!(outcome, EncodeOutcome::Ready, "IPP GOP must not buffer frames");
            let pkt = output.lock().expect("lock bitstream");
            total_bytes += pkt.data().len();
        }
        session.flush_eos().expect("flush_eos");
        assert!(total_bytes > 0, "{codec:?} produced empty bitstream");
    }
}

/// ENC-1: with B-frames (`frameIntervalP = 2`) NVENC returns `NV_ENC_ERR_NEED_MORE_INPUT` for
/// buffered frames; that must surface as `EncodeOutcome::NeedMoreInput`, the buffered output must
/// refuse to lock, and every frame must yield a packet once released in submission order.
#[test]
#[ignore = "GPU: needs NVIDIA driver with NVENC; run with --ignored"]
fn encode_with_b_frames_reports_need_more_input() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let (width, height) = (256u32, 256u32);
    let num_frames = 8u32;
    const RING: usize = 4;

    let mut session = EncodeSession::open(&cuda).expect("open NVENC session");
    let max_b = session
        .caps(
            EncCodec::H264,
            nvenc::sys::_NV_ENC_CAPS_NV_ENC_CAPS_NUM_MAX_BFRAMES,
        )
        .expect("caps(NUM_MAX_BFRAMES)");
    if max_b < 1 {
        eprintln!("skipped: this GPU reports no H.264 B-frame support (NUM_MAX_BFRAMES = {max_b})");
        return;
    }
    let mut preset = session
        .preset_config(EncCodec::H264, Preset::P4, Tuning::HighQuality)
        .expect("preset_config");
    preset.set_gop_length(30);
    preset.set_frame_interval_p(2);
    let mut cfg = SessionConfig::new(EncCodec::H264, width, height, 30, 1);
    cfg.encode_config = Some(preset);
    session.initialize(&cfg).expect("initialize encoder");

    let mut inputs: Vec<InputBuffer> = (0..RING)
        .map(|_| InputBuffer::new(&session, width, height, BufferFormat::Nv12).expect("input"))
        .collect();
    let mut outputs: Vec<OutputBitstream> = (0..RING)
        .map(|_| OutputBitstream::new(&session).expect("output"))
        .collect();
    let mut pending: VecDeque<usize> = VecDeque::new();
    let mut packets = 0usize;
    let mut buffered = 0usize;

    for frame_idx in 0..num_frames {
        let slot = frame_idx as usize % RING;
        fill_nv12(&mut inputs[slot], width, height, frame_idx);
        let outcome = session
            .encode_picture(
                EncodeInput::Buffer(&inputs[slot]),
                &mut outputs[slot],
                frame_idx,
                u64::from(frame_idx),
                idr_flags(frame_idx),
            )
            .expect("encode_picture must not fail on NV_ENC_ERR_NEED_MORE_INPUT");
        pending.push_back(slot);
        match outcome {
            EncodeOutcome::NeedMoreInput => {
                buffered += 1;
                assert!(
                    outputs[slot].lock().is_err(),
                    "a buffered output must refuse to lock"
                );
            }
            EncodeOutcome::Ready => {
                while let Some(s) = pending.pop_front() {
                    let pkt = outputs[s].lock().expect("lock released bitstream");
                    assert!(!pkt.data().is_empty(), "released packet is empty");
                    packets += 1;
                }
            }
        }
        assert!(pending.len() < RING, "reorder depth exceeded the test ring");
    }
    session.flush_eos().expect("flush_eos");
    while let Some(s) = pending.pop_front() {
        let pkt = outputs[s].lock().expect("lock bitstream after EOS");
        assert!(!pkt.data().is_empty(), "packet after EOS is empty");
        packets += 1;
    }
    assert!(buffered > 0, "IBP GOP never returned NeedMoreInput");
    assert_eq!(packets, num_frames as usize, "every frame must yield one packet");
}

/// ENC-6: a sticky CUDA error on the context made `EncodeSession::drop` return before
/// `nvEncDestroyEncoder`, leaking the hardware session. GeForce boards cap concurrent sessions
/// (8 on current drivers), so leaked sessions make a later `open` fail inside this loop.
#[test]
#[ignore = "GPU: needs NVIDIA driver with NVENC; run with --ignored"]
fn drop_with_sticky_context_error_still_destroys_session() {
    let cuda = Context::new(0).expect("CUDA device 0");
    for i in 0..32 {
        let session = EncodeSession::open(&cuda)
            .unwrap_or_else(|e| panic!("open #{i} failed (sessions leaked by drop?): {e}"));
        cuda.record_err(Err::<(), _>(DriverError(
            cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
        )));
        drop(session);
        assert!(
            cuda.check_err().is_err(),
            "the sticky error must still be reported after the drop"
        );
    }
}

/// ENC-2: NVENC keeps the `CUstream` handles from `nvEncSetIOCudaStreams`; the session must hold
/// the streams so a caller dropping its `Arc<Stream>` cannot destroy (or recycle) them.
#[test]
#[ignore = "GPU: needs NVIDIA driver with NVENC; run with --ignored"]
fn set_io_streams_retains_streams() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let (width, height) = (256u32, 256u32);
    let mut session = EncodeSession::open(&cuda).expect("open NVENC session");
    let mut preset = session
        .preset_config(EncCodec::H264, Preset::P4, Tuning::HighQuality)
        .expect("preset_config");
    preset.set_frame_interval_p(1);
    let mut cfg = SessionConfig::new(EncCodec::H264, width, height, 30, 1);
    cfg.encode_config = Some(preset);
    session.initialize(&cfg).expect("initialize encoder");

    let stream = cuda.new_stream().expect("stream");
    session
        .set_io_streams(&stream, &stream)
        .expect("set_io_streams");
    assert_eq!(
        Arc::strong_count(&stream),
        3,
        "session must retain the input and output streams"
    );
    drop(stream);

    let mut input = InputBuffer::new(&session, width, height, BufferFormat::Nv12).expect("input");
    let mut output = OutputBitstream::new(&session).expect("output");
    fill_nv12(&mut input, width, height, 0);
    let outcome = session
        .encode_picture(EncodeInput::Buffer(&input), &mut output, 0, 0, idr_flags(0))
        .expect("encode_picture with retained IO streams");
    assert_eq!(outcome, EncodeOutcome::Ready);
    let pkt = output.lock().expect("lock bitstream");
    assert!(!pkt.data().is_empty(), "empty packet");
}
