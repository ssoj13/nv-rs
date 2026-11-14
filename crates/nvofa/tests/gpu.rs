//! GPU tests (T2/T3) for `nvofa`: synthetic shifted frame -> flow vector direction/magnitude check.
//!
//! Run with `cargo test -p nvofa -- --include-ignored`.

use cuda_core::Context;
use nvofa::{
    BufferDesc, BufferFormat, BufferUsage, CudaBufferType, ExecuteParams, FlowVector, GridSize,
    InitConfig, OfBuffer, OpticalFlowSession, PerfLevel,
};

#[test]
#[ignore = "GPU: needs Turing+ NVIDIA GPU with Optical Flow; run with --ignored"]
fn optical_flow_detects_synthetic_shift() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let width = 256u32;
    let height = 256u32;
    let shift_x = 8i32;
    let shift_y = 4i32;

    let mut session = OpticalFlowSession::open(&cuda).expect("open NVOFA session");
    let mut cfg = InitConfig::new(width, height, GridSize::Four, BufferFormat::Grayscale8);
    cfg.perf_level = PerfLevel::Slow;
    session.init(&cfg).expect("init NVOFA");
    session
        .set_io_streams(&stream, &stream)
        .expect("set_io_streams");

    let in_desc = BufferDesc {
        width,
        height,
        usage: BufferUsage::Input,
        format: BufferFormat::Grayscale8,
    };
    let (grid_w, grid_h) = GridSize::Four.grid_dims(width, height);
    let out_desc = BufferDesc {
        width: grid_w,
        height: grid_h,
        usage: BufferUsage::Output,
        format: BufferFormat::Short2,
    };

    let buf_cur =
        OfBuffer::new(&session, in_desc, CudaBufferType::CuDevicePtr).expect("input buffer");
    let buf_ref =
        OfBuffer::new(&session, in_desc, CudaBufferType::CuDevicePtr).expect("reference buffer");
    let mut buf_out =
        OfBuffer::new(&session, out_desc, CudaBufferType::CuDevicePtr).expect("output buffer");

    let mut img_cur = vec![0u8; (width * height) as usize];
    let mut img_ref = vec![0u8; (width * height) as usize];
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
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            img_cur[(y * width as i32 + x) as usize] = pattern(x, y);
            // Feature at (x, y) in input_frame moves to (x + shift_x, y + shift_y) in reference_frame.
            img_ref[(y * width as i32 + x) as usize] = pattern(x - shift_x, y - shift_y);
        }
    }

    buf_cur
        .upload_plane0(&stream, width as usize, height as usize, &img_cur)
        .expect("upload cur");
    buf_ref
        .upload_plane0(&stream, width as usize, height as usize, &img_ref)
        .expect("upload ref");

    let mut exec = ExecuteParams {
        input_frame: &buf_cur,
        reference_frame: &buf_ref,
        output_buffer: &mut buf_out,
        external_hints: None,
        output_cost: None,
        disable_temporal_hints: true,
        rois: &[],
    };
    session.execute(&mut exec).expect("execute NVOFA");

    let mut flow = vec![FlowVector::default(); (grid_w * grid_h) as usize];
    buf_out
        .download_flow_vectors(&stream, &mut flow)
        .expect("download flow");

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
        (mean_x - f64::from(shift_x)).abs() < 2.0,
        "expected dx ~ {shift_x}, got {mean_x:.2}"
    );
    assert!(
        (mean_y - f64::from(shift_y)).abs() < 2.0,
        "expected dy ~ {shift_y}, got {mean_y:.2}"
    );
}
