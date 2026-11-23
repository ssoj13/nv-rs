//! GPU tests (T2/T3) for `nvofa`: synthetic shifted frame -> flow vector direction/magnitude check,
//! cross-stream readback ordering, concurrent `execute` on a shared session, and context binding
//! in `OfBuffer::drop`.
//!
//! Run with `cargo test -p nvofa -- --include-ignored`.

use cuda_core::{Context, Stream};
use nvofa::{
    BufferDesc, BufferFormat, BufferUsage, CudaBufferType, ExecuteParams, FlowVector, GridSize,
    InitConfig, OfBuffer, OpticalFlowSession, PerfLevel,
};
use std::sync::Arc;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const SHIFT_X: i32 = 8;
const SHIFT_Y: i32 = 4;

/// Current and reference frames where the feature at `(x, y)` moves by `(SHIFT_X, SHIFT_Y)`.
fn shifted_frames() -> (Vec<u8>, Vec<u8>) {
    let pattern = |x: i32, y: i32| -> u8 {
        // Rich, non-periodic multi-scale texture so hardware block matching has unique local descriptors.
        let ux = x.wrapping_mul(374_761_393) as u32;
        let uy = y.wrapping_mul(668_265_263) as u32;
        let mut h = ux ^ uy.rotate_left(13);
        h = h.wrapping_mul(1_274_126_177);
        h ^= h >> 16;
        let fine = (h & 0xff) as f32;
        let fx = (x as f32) * 0.13 + ((y as f32) * 0.03).sin();
        let fy = (y as f32) * 0.11 + ((x as f32) * 0.04).cos();
        let coarse = (fx.sin() * fy.cos() * 64.0) + 128.0;
        (0.55 * fine + 0.45 * coarse).clamp(0.0, 255.0) as u8
    };
    let mut img_cur = vec![0u8; (WIDTH * HEIGHT) as usize];
    let mut img_ref = vec![0u8; (WIDTH * HEIGHT) as usize];
    for y in 0..HEIGHT as i32 {
        for x in 0..WIDTH as i32 {
            img_cur[(y * WIDTH as i32 + x) as usize] = pattern(x, y);
            // Feature at (x, y) in input_frame moves to (x + shift_x, y + shift_y) in reference_frame.
            img_ref[(y * WIDTH as i32 + x) as usize] = pattern(x - SHIFT_X, y - SHIFT_Y);
        }
    }
    (img_cur, img_ref)
}

/// Opens and initializes a 4x4-grid grayscale session.
fn open_session(cuda: &Arc<Context>) -> Arc<OpticalFlowSession> {
    let mut session = OpticalFlowSession::open(cuda).expect("open NVOFA session");
    let mut cfg = InitConfig::new(WIDTH, HEIGHT, GridSize::Four, BufferFormat::Grayscale8);
    cfg.perf_level = PerfLevel::Slow;
    session.init(&cfg).expect("init NVOFA");
    session
}

/// Input, reference and output buffers for one flow estimation on `session`.
fn alloc_buffers(session: &Arc<OpticalFlowSession>) -> (OfBuffer, OfBuffer, OfBuffer) {
    let in_desc = BufferDesc {
        width: WIDTH,
        height: HEIGHT,
        usage: BufferUsage::Input,
        format: BufferFormat::Grayscale8,
    };
    let (grid_w, grid_h) = GridSize::Four.grid_dims(WIDTH, HEIGHT);
    let out_desc = BufferDesc {
        width: grid_w,
        height: grid_h,
        usage: BufferUsage::Output,
        format: BufferFormat::Short2,
    };
    (
        OfBuffer::new(session, in_desc, CudaBufferType::CuDevicePtr).expect("input buffer"),
        OfBuffer::new(session, in_desc, CudaBufferType::CuDevicePtr).expect("reference buffer"),
        OfBuffer::new(session, out_desc, CudaBufferType::CuDevicePtr).expect("output buffer"),
    )
}

/// Uploads the shifted frames, executes, and reads the flow back on `readback`.
fn run_flow(
    session: &OpticalFlowSession,
    buffers: &mut (OfBuffer, OfBuffer, OfBuffer),
    upload: &Stream,
    readback: &Stream,
) -> Vec<FlowVector> {
    let (img_cur, img_ref) = shifted_frames();
    let (buf_cur, buf_ref, buf_out) = (&buffers.0, &buffers.1, &mut buffers.2);
    buf_cur
        .upload_plane0(upload, WIDTH as usize, HEIGHT as usize, &img_cur)
        .expect("upload cur");
    buf_ref
        .upload_plane0(upload, WIDTH as usize, HEIGHT as usize, &img_ref)
        .expect("upload ref");

    let mut exec = ExecuteParams {
        input_frame: buf_cur,
        reference_frame: buf_ref,
        output_buffer: &mut *buf_out,
        external_hints: None,
        output_cost: None,
        disable_temporal_hints: true,
        rois: &[],
    };
    session.execute(&mut exec).expect("execute NVOFA");

    let (grid_w, grid_h) = GridSize::Four.grid_dims(WIDTH, HEIGHT);
    let mut flow = vec![FlowVector::default(); (grid_w * grid_h) as usize];
    buf_out
        .download_flow_vectors(readback, &mut flow)
        .expect("download flow");
    flow
}

/// Asserts that the mean interior flow matches `(SHIFT_X, SHIFT_Y)` within 2 px.
fn assert_shift(flow: &[FlowVector]) {
    let (grid_w, grid_h) = GridSize::Four.grid_dims(WIDTH, HEIGHT);
    // Sample interior region away from borders to avoid boundary clamping.
    let mut sum_x = 0.0f64;
    let mut sum_y = 0.0f64;
    let mut count = 0usize;
    for gy in 8..(grid_h - 8) {
        for gx in 8..(grid_w - 8) {
            let (dx, dy) = flow[(gy * grid_w + gx) as usize].to_pixels();
            sum_x += f64::from(dx);
            sum_y += f64::from(dy);
            count += 1;
        }
    }
    let mean_x = sum_x / (count as f64);
    let mean_y = sum_y / (count as f64);
    assert!(
        (mean_x - f64::from(SHIFT_X)).abs() < 2.0,
        "expected dx ~ {SHIFT_X}, got {mean_x:.2}"
    );
    assert!(
        (mean_y - f64::from(SHIFT_Y)).abs() < 2.0,
        "expected dy ~ {SHIFT_Y}, got {mean_y:.2}"
    );
}

#[test]
#[ignore = "GPU: needs Turing+ NVIDIA GPU with Optical Flow; run with --ignored"]
fn optical_flow_detects_synthetic_shift() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let session = open_session(&cuda);
    session
        .set_io_streams(&stream, &stream)
        .expect("set_io_streams");
    let mut buffers = alloc_buffers(&session);
    let flow = run_flow(&session, &mut buffers, &stream, &stream);
    assert_shift(&flow);
}

/// OFA-2: `execute` runs on the session's I/O stream while the readback uses an unrelated
/// non-blocking stream; the download must still observe the finished flow vectors.
#[test]
#[ignore = "GPU: needs Turing+ NVIDIA GPU with Optical Flow; run with --ignored"]
fn flow_readback_on_other_stream_waits_for_execute() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let io = cuda.new_stream().expect("io stream");
    let readback = cuda.new_stream().expect("readback stream");
    let session = open_session(&cuda);
    session.set_io_streams(&io, &io).expect("set_io_streams");
    let mut buffers = alloc_buffers(&session);
    for _ in 0..8 {
        let flow = run_flow(&session, &mut buffers, &readback, &readback);
        assert_shift(&flow);
    }
}

/// OFA-1: two threads execute on one shared session at the same time, each with its own
/// buffers; every call must succeed and produce the expected flow.
#[test]
#[ignore = "GPU: needs Turing+ NVIDIA GPU with Optical Flow; run with --ignored"]
fn concurrent_execute_on_shared_session() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let session = open_session(&cuda);
    session
        .set_io_streams(&stream, &stream)
        .expect("set_io_streams");
    std::thread::scope(|scope| {
        for _ in 0..2 {
            let session = session.clone();
            let cuda = cuda.clone();
            scope.spawn(move || {
                let copy = cuda.new_stream().expect("copy stream");
                let mut buffers = alloc_buffers(&session);
                for _ in 0..16 {
                    let flow = run_flow(&session, &mut buffers, &copy, &copy);
                    assert_shift(&flow);
                }
            });
        }
    });
    cuda.check_err().expect("no recorded error");
}

/// OFA-4: `OfBuffer::drop` binds the CUDA context, so dropping with no current context records
/// no error.
#[test]
#[ignore = "GPU: needs Turing+ NVIDIA GPU with Optical Flow; run with --ignored"]
fn buffer_drop_without_current_context_records_no_error() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let session = open_session(&cuda);
    let buffers = alloc_buffers(&session);
    cuda.check_err().expect("no error before teardown");
    // SAFETY: clearing the calling thread's current context is always valid.
    let rc = unsafe { cuda_core::sys::cuCtxSetCurrent(std::ptr::null_mut()) };
    assert_eq!(rc, cuda_core::sys::cudaError_enum_CUDA_SUCCESS);
    drop(buffers);
    cuda.check_err()
        .expect("buffer teardown without a current context recorded an error");
}
