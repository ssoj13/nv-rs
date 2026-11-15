//! GPU-accelerated image (JPEG) and video (H.264 Annex-B) export from CUDA RGBA8 buffers.
//!
//! **What.**
//! - [`save_jpeg_from_rgba8`] — converts a device `RGBA8` image to interleaved `RGB8` via an
//!   NVRTC CUDA kernel, encodes it on the GPU with `nvjpeg`, and writes the `.jpg` file to disk.
//! - [`VideoRecorder`] — converts each rendered `RGBA8` frame to `NV12` on the GPU via an NVRTC
//!   CUDA kernel and encodes it zero-copy with `nvenc` into an Annex-B `.h264` elementary stream.
//!
//! **Error handling.** All functions return `Result<_, String>` with descriptive messages (for
//! example when `nvjpeg` or `nvenc` is unavailable on the host system) and never panic.

use std::ffi::{CString, c_void};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;

use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, DeviceBuffer, Stream};
use nvenc::{
    BufferFormat, Codec, EncodeFlags, EncodeInput, EncodeSession, OutputBitstream, Preset,
    RegisteredResource, SessionConfig, Tuning,
};
use nvjpeg::{
    ChromaSubsampling, EncoderParams, EncoderState, Handle as NvjpegHandle, ImagePlanes,
    InputFormat,
};

const RGBA8_TO_RGB8_CU: &str = r#"
extern "C" __global__ void rgba8_to_rgb8(
    const unsigned char* __restrict__ rgba,
    unsigned char* __restrict__ rgb,
    unsigned int w,
    unsigned int h
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= w || y >= h) return;
    unsigned int idx = y * w + x;
    unsigned int s = idx * 4u;
    unsigned int d = idx * 3u;
    rgb[d + 0u] = rgba[s + 0u];
    rgb[d + 1u] = rgba[s + 1u];
    rgb[d + 2u] = rgba[s + 2u];
}
"#;

const RGBA8_TO_NV12_CU: &str = r#"
__device__ __forceinline__ unsigned char clamp_u8(float v) {
    if (v < 0.0f) return 0u;
    if (v > 255.0f) return 255u;
    return static_cast<unsigned char>(v + 0.5f);
}

extern "C" __global__ void rgba8_to_nv12(
    const unsigned char* __restrict__ rgba,
    unsigned char* __restrict__ nv12,
    unsigned int w,
    unsigned int h,
    unsigned int nv12_pitch
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= w || y >= h) return;

    unsigned int s = (y * w + x) * 4u;
    float r = static_cast<float>(rgba[s + 0u]);
    float g = static_cast<float>(rgba[s + 1u]);
    float b = static_cast<float>(rgba[s + 2u]);

    // BT.709 limited-range luma (Y in [16, 235]).
    float y_val = 16.0f + (0.182586f * r + 0.614231f * g + 0.062007f * b);
    nv12[y * nv12_pitch + x] = clamp_u8(y_val);

    // Subsampled 4:2:0 chroma (2x2 box filter) on even coordinates.
    if ((x & 1u) == 0u && (y & 1u) == 0u) {
        unsigned int s01 = (y * w + (x + 1u)) * 4u;
        unsigned int s10 = ((y + 1u) * w + x) * 4u;
        unsigned int s11 = ((y + 1u) * w + (x + 1u)) * 4u;

        float r_avg = 0.25f * (r + static_cast<float>(rgba[s01 + 0u])
            + static_cast<float>(rgba[s10 + 0u]) + static_cast<float>(rgba[s11 + 0u]));
        float g_avg = 0.25f * (g + static_cast<float>(rgba[s01 + 1u])
            + static_cast<float>(rgba[s10 + 1u]) + static_cast<float>(rgba[s11 + 1u]));
        float b_avg = 0.25f * (b + static_cast<float>(rgba[s01 + 2u])
            + static_cast<float>(rgba[s10 + 2u]) + static_cast<float>(rgba[s11 + 2u]));

        float u_val = 128.0f + (-0.100644f * r_avg - 0.338572f * g_avg + 0.439216f * b_avg);
        float v_val = 128.0f + (0.439216f * r_avg - 0.398942f * g_avg - 0.040274f * b_avg);

        unsigned char* uv_plane = nv12 + static_cast<size_t>(h) * nv12_pitch;
        size_t uv_idx = static_cast<size_t>(y >> 1u) * nv12_pitch + x;
        uv_plane[uv_idx + 0u] = clamp_u8(u_val);
        uv_plane[uv_idx + 1u] = clamp_u8(v_val);
    }
}
"#;

/// RAII wrapper for a CUDA driver module and single entry-point function compiled via NVRTC.
struct CudaKernel {
    module: cu::CUmodule,
    func: cu::CUfunction,
    cuda: Arc<CudaContext>,
}

impl CudaKernel {
    fn compile(
        cuda: &Arc<CudaContext>,
        src: &str,
        unit_name: &str,
        fn_name: &str,
    ) -> Result<Self, String> {
        cuda.bind_to_thread()
            .map_err(|e| format!("CUDA bind failed: {e}"))?;

        let opts = nvrtc::CompileOptions::new()
            .arch("compute_75")
            .std("c++17")
            .fast_math();
        let mut prog = nvrtc::Program::new(src, Some(unit_name), &[])
            .map_err(|e| format!("NVRTC program creation failed for {unit_name}: {e}"))?;
        prog.compile(&opts)
            .map_err(|e| format!("NVRTC compile failed for {unit_name}: {e}"))?;
        let mut ptx = prog
            .ptx()
            .map_err(|e| format!("NVRTC PTX retrieval failed for {unit_name}: {e}"))?;
        if ptx.last() != Some(&0) {
            ptx.push(0);
        }

        let mut module: cu::CUmodule = std::ptr::null_mut();
        // SAFETY: `ptx` is a NUL-terminated PTX image and `cuda` is bound to the current thread.
        let load_res = unsafe { cu::cuModuleLoadData(&mut module, ptx.as_ptr().cast()) };
        if load_res != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!(
                "cuModuleLoadData failed for {unit_name}: {load_res:?}"
            ));
        }

        let c_fn_name = CString::new(fn_name)
            .map_err(|e| format!("invalid kernel function name {fn_name}: {e}"))?;
        let mut func: cu::CUfunction = std::ptr::null_mut();
        // SAFETY: `module` was just loaded on the bound context and `c_fn_name` is NUL-terminated.
        let fn_res = unsafe { cu::cuModuleGetFunction(&mut func, module, c_fn_name.as_ptr()) };
        if fn_res != cu::cudaError_enum_CUDA_SUCCESS {
            // SAFETY: `module` was loaded above and is not yet owned by `Self`.
            unsafe { cu::cuModuleUnload(module) };
            return Err(format!("cuModuleGetFunction({fn_name}) failed: {fn_res:?}"));
        }

        Ok(Self {
            module,
            func,
            cuda: cuda.clone(),
        })
    }
}

impl Drop for CudaKernel {
    fn drop(&mut self) {
        if !self.module.is_null() && self.cuda.bind_to_thread().is_ok() {
            // SAFETY: `self.module` was loaded via `cuModuleLoadData` on `self.cuda`.
            unsafe { cu::cuModuleUnload(self.module) };
        }
    }
}

/// Converts a CUDA `RGBA8` device buffer (`w x h`) to JPEG using `nvjpeg` and writes it to `path`.
///
/// `rgba_dev_ptr` must point to a valid CUDA device allocation of at least `w * h * 4` bytes on
/// `cuda`. `quality` must be in `1..=100`.
pub fn save_jpeg_from_rgba8(
    cuda: &Arc<CudaContext>,
    stream: &Stream,
    rgba_dev_ptr: u64,
    w: u32,
    h: u32,
    path: &Path,
    quality: u8,
) -> Result<(), String> {
    if rgba_dev_ptr == 0 {
        return Err("rgba_dev_ptr is null".to_string());
    }
    if w == 0 || h == 0 {
        return Err("image dimensions must be non-zero".to_string());
    }
    if !(1..=100).contains(&quality) {
        return Err(format!("JPEG quality {quality} must be in 1..=100"));
    }

    cuda.bind_to_thread()
        .map_err(|e| format!("CUDA bind failed: {e}"))?;

    let rgb_bytes = (w as usize)
        .checked_mul(h as usize)
        .and_then(|px| px.checked_mul(3))
        .ok_or_else(|| "RGB8 buffer size overflow".to_string())?;
    let alloc_stream = cuda
        .new_stream()
        .map_err(|e| format!("CUDA stream creation failed: {e}"))?;
    let rgb_buf = DeviceBuffer::<u8>::zeroed(&alloc_stream, rgb_bytes)
        .map_err(|e| format!("CUDA RGB8 allocation failed: {e}"))?;
    alloc_stream
        .synchronize()
        .map_err(|e| format!("CUDA stream sync failed: {e}"))?;

    let kernel = CudaKernel::compile(cuda, RGBA8_TO_RGB8_CU, "rgba8_to_rgb8.cu", "rgba8_to_rgb8")?;
    let mut src_ptr = rgba_dev_ptr;
    let mut dst_ptr = rgb_buf.cu_deviceptr();
    let mut width = w;
    let mut height = h;
    let mut args: [*mut c_void; 4] = [
        (&raw mut src_ptr).cast(),
        (&raw mut dst_ptr).cast(),
        (&raw mut width).cast(),
        (&raw mut height).cast(),
    ];
    let gx = w.div_ceil(16);
    let gy = h.div_ceil(16);
    // SAFETY: `kernel.func` expects `(const u8*, u8*, u32, u32)` with valid device pointers of
    // at least `w * h * 4` and `w * h * 3` bytes respectively; `rgb_buf` lives past stream sync.
    let launch_res = unsafe {
        cu::cuLaunchKernel(
            kernel.func,
            gx,
            gy,
            1,
            16,
            16,
            1,
            0,
            stream.cu_stream(),
            args.as_mut_ptr(),
            std::ptr::null_mut(),
        )
    };
    if launch_res != cu::cudaError_enum_CUDA_SUCCESS {
        return Err(format!(
            "rgba8_to_rgb8 cuLaunchKernel failed: {launch_res:?}"
        ));
    }

    let handle = NvjpegHandle::new(cuda).map_err(|e| format!("nvJPEG init failed: {e}"))?;
    let mut params = EncoderParams::new(&handle, stream)
        .map_err(|e| format!("nvJPEG encoder params failed: {e}"))?;
    params
        .set_quality(quality, stream)
        .map_err(|e| format!("nvJPEG set_quality failed: {e}"))?;
    params
        .set_sampling_factors(ChromaSubsampling::Yuv420, stream)
        .map_err(|e| format!("nvJPEG set_sampling_factors failed: {e}"))?;

    let mut state = EncoderState::new(&handle, stream)
        .map_err(|e| format!("nvJPEG encoder state failed: {e}"))?;
    let pitch = (w as usize)
        .checked_mul(3)
        .ok_or_else(|| "RGB8 pitch overflow".to_string())?;
    let planes = ImagePlanes::interleaved(&rgb_buf, w, h, 3, pitch)
        .map_err(|e| format!("nvJPEG ImagePlanes failed: {e}"))?;
    state
        .encode_image(&params, &planes, InputFormat::Rgbi, w, h, stream)
        .map_err(|e| format!("nvJPEG encode_image failed: {e}"))?;

    let mut bitstream = Vec::new();
    state
        .retrieve_bitstream_into(&mut bitstream, stream)
        .map_err(|e| format!("nvJPEG retrieve_bitstream failed: {e}"))?;

    stream
        .synchronize()
        .map_err(|e| format!("CUDA stream synchronize failed: {e}"))?;
    drop(rgb_buf);

    std::fs::write(path, &bitstream)
        .map_err(|e| format!("failed to write JPEG '{}': {e}", path.display()))?;
    Ok(())
}

/// Hardware H.264 video recorder using `nvenc` and a zero-copy GPU `RGBA8 -> NV12` kernel.
///
/// Writes an Annex-B `.h264` elementary bitstream directly from CUDA device memory.
pub struct VideoRecorder {
    cuda: Arc<CudaContext>,
    kernel: CudaKernel,
    // Drop order invariant: `nv12_res` and `bitstream` must drop before `nv12_buf` and `session`.
    nv12_res: RegisteredResource,
    bitstream: OutputBitstream,
    nv12_buf: DeviceBuffer<u8>,
    session: Arc<EncodeSession>,
    writer: BufWriter<File>,
    width: u32,
    height: u32,
    nv12_pitch: u32,
    frame_idx: u32,
}

impl VideoRecorder {
    /// Opens an NVENC H.264 encoding session and creates `path` for Annex-B `.h264` output.
    ///
    /// Both `w` and `h` must be non-zero and even (required by `NV12` 4:2:0 chroma subsampling).
    pub fn start(
        cuda: &Arc<CudaContext>,
        w: u32,
        h: u32,
        fps: u32,
        path: &Path,
    ) -> Result<Self, String> {
        if w == 0 || h == 0 || !w.is_multiple_of(2) || !h.is_multiple_of(2) {
            return Err(format!(
                "NV12 video dimensions ({w}x{h}) must be non-zero and even"
            ));
        }
        if fps == 0 {
            return Err("video FPS must be non-zero".to_string());
        }

        cuda.bind_to_thread()
            .map_err(|e| format!("CUDA bind failed: {e}"))?;

        let kernel =
            CudaKernel::compile(cuda, RGBA8_TO_NV12_CU, "rgba8_to_nv12.cu", "rgba8_to_nv12")?;

        let mut session = EncodeSession::open(cuda)
            .map_err(|e| format!("NVENC unavailable or session open failed: {e}"))?;

        let mut enc_cfg = session
            .preset_config(Codec::H264, Preset::P4, Tuning::HighQuality)
            .map_err(|e| format!("NVENC preset_config failed: {e}"))?;
        // IPP GOP with no B-frames so every `encode_picture` yields a packet immediately.
        enc_cfg.set_frame_interval_p(1);
        enc_cfg.set_gop_length(fps);
        enc_cfg.set_average_bitrate(12_000_000);

        let mut session_cfg = SessionConfig::new(Codec::H264, w, h, fps, 1);
        session_cfg.preset = Preset::P4;
        session_cfg.tuning = Tuning::HighQuality;
        session_cfg.encode_config = Some(enc_cfg);

        session
            .initialize(&session_cfg)
            .map_err(|e| format!("NVENC initialize failed: {e}"))?;

        // `RegisteredResource::from_cuda_ptr` requires a 4-byte-aligned pitch.
        let nv12_pitch = w
            .checked_next_multiple_of(4)
            .ok_or_else(|| "NV12 pitch overflow".to_string())?;
        let nv12_bytes = (nv12_pitch as usize)
            .checked_mul(h as usize)
            .and_then(|luma| luma.checked_mul(3))
            .map(|total| total / 2)
            .ok_or_else(|| "NV12 buffer size overflow".to_string())?;

        let stream = cuda
            .new_stream()
            .map_err(|e| format!("CUDA stream creation failed: {e}"))?;
        let nv12_buf = DeviceBuffer::<u8>::zeroed(&stream, nv12_bytes)
            .map_err(|e| format!("CUDA NV12 allocation failed: {e}"))?;
        stream
            .synchronize()
            .map_err(|e| format!("CUDA stream sync failed: {e}"))?;

        // SAFETY: `nv12_buf` is allocated on `cuda` with `nv12_pitch * h * 3 / 2` bytes and is
        // declared after `nv12_res` in `VideoRecorder` so it outlives `nv12_res`.
        let nv12_res = unsafe {
            RegisteredResource::from_cuda_ptr(
                &session,
                nv12_buf.cu_deviceptr(),
                w,
                h,
                nv12_pitch,
                BufferFormat::Nv12,
            )
        }
        .map_err(|e| format!("NVENC register NV12 resource failed: {e}"))?;

        let bitstream = OutputBitstream::new(&session)
            .map_err(|e| format!("NVENC create bitstream buffer failed: {e}"))?;

        let file = File::create(path)
            .map_err(|e| format!("failed to create H.264 file '{}': {e}", path.display()))?;

        Ok(Self {
            cuda: cuda.clone(),
            kernel,
            nv12_res,
            bitstream,
            nv12_buf,
            session,
            writer: BufWriter::new(file),
            width: w,
            height: h,
            nv12_pitch,
            frame_idx: 0,
        })
    }

    /// Encodes one `RGBA8` frame from `rgba_dev_ptr` (`width * height * 4` bytes) into the stream.
    pub fn push_frame(&mut self, stream: &Stream, rgba_dev_ptr: u64) -> Result<(), String> {
        if rgba_dev_ptr == 0 {
            return Err("rgba_dev_ptr is null".to_string());
        }

        self.cuda
            .bind_to_thread()
            .map_err(|e| format!("CUDA bind failed: {e}"))?;

        let mut src_ptr = rgba_dev_ptr;
        let mut dst_ptr = self.nv12_buf.cu_deviceptr();
        let mut w = self.width;
        let mut h = self.height;
        let mut pitch = self.nv12_pitch;
        let mut args: [*mut c_void; 5] = [
            (&raw mut src_ptr).cast(),
            (&raw mut dst_ptr).cast(),
            (&raw mut w).cast(),
            (&raw mut h).cast(),
            (&raw mut pitch).cast(),
        ];
        let gx = self.width.div_ceil(16);
        let gy = self.height.div_ceil(16);
        // SAFETY: `self.kernel.func` expects `(const u8*, u8*, u32, u32, u32)` with valid device
        // buffers; `stream.synchronize()` completes before NVENC maps `self.nv12_buf`.
        let launch_res = unsafe {
            cu::cuLaunchKernel(
                self.kernel.func,
                gx,
                gy,
                1,
                16,
                16,
                1,
                0,
                stream.cu_stream(),
                args.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        };
        if launch_res != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!(
                "rgba8_to_nv12 cuLaunchKernel failed: {launch_res:?}"
            ));
        }

        stream
            .synchronize()
            .map_err(|e| format!("CUDA stream synchronize failed: {e}"))?;

        let mapped = self
            .nv12_res
            .map()
            .map_err(|e| format!("NVENC map input resource failed: {e}"))?;
        let flags = EncodeFlags {
            force_idr: self.frame_idx == 0,
            output_sps_pps: self.frame_idx == 0,
            ..Default::default()
        };
        self.session
            .encode_picture(
                EncodeInput::Mapped(&mapped),
                &mut self.bitstream,
                self.frame_idx,
                u64::from(self.frame_idx),
                flags,
            )
            .map_err(|e| format!("NVENC encode_picture failed: {e}"))?;

        let lock = self
            .bitstream
            .lock()
            .map_err(|e| format!("NVENC lock bitstream failed: {e}"))?;
        self.writer
            .write_all(lock.data())
            .map_err(|e| format!("failed to write H.264 bitstream: {e}"))?;

        self.frame_idx = self.frame_idx.saturating_add(1);
        Ok(())
    }

    /// Flushes the NVENC encoder and output file, returning the total number of encoded frames.
    pub fn finish(mut self) -> Result<u64, String> {
        self.cuda
            .bind_to_thread()
            .map_err(|e| format!("CUDA bind failed: {e}"))?;
        self.session
            .flush_eos()
            .map_err(|e| format!("NVENC flush_eos failed: {e}"))?;
        self.writer
            .flush()
            .map_err(|e| format!("failed to flush H.264 file: {e}"))?;
        Ok(u64::from(self.frame_idx))
    }

    /// Number of frames encoded so far.
    pub const fn frame_count(&self) -> u64 {
        self.frame_idx as u64
    }
}
