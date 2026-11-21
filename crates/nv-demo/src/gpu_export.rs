//! GPU-accelerated image (JPEG) and video (MP4 H.264 SDR / HEVC PQ10 HDR10) export from CUDA buffers.
//!
//! **What.**
//! - [`save_jpeg_from_rgba8`] — converts a device `RGBA8` image to interleaved `RGB8` via an
//!   NVRTC CUDA kernel, encodes it on the GPU with `nvjpeg`, and writes the `.jpg` file to disk.
//! - [`VideoRecorder`] — converts each rendered frame on the GPU (`RGBA8 -> NV12` BT.709 for
//!   [`VideoProfile::SdrH264`], or linear HDR `float4 -> P010` BT.2020 SMPTE ST 2084 PQ for
//!   [`VideoProfile::Hdr10Pq`]), encodes zero-copy with `nvenc`, and muxes into an
//!   iPhone/QuickTime-compatible `.mp4` container with per-frame `stts` timing.
//!
//! **Error handling.** All functions return `Result<_, String>` with descriptive messages (for
//! example when `nvjpeg` or `nvenc` is unavailable on the host system) and never panic.

use std::ffi::{CString, c_void};
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

pub use crate::mp4_muxer::VideoProfile;
use crate::mp4_muxer::{MP4_TIMESCALE, Mp4Muxer};

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

const HDR_F32_TO_P010_PQ_CU: &str = r#"
__device__ __forceinline__ unsigned short pack_p010(float v10) {
    float c = v10 < 0.0f ? 0.0f : (v10 > 1023.0f ? 1023.0f : v10);
    unsigned int q = static_cast<unsigned int>(c + 0.5f);
    return static_cast<unsigned short>((q & 1023u) << 6u);
}

__device__ __forceinline__ float pq_oetf(float nits) {
    float L = nits * 0.0001f; // normalized to 10,000 nits
    if (L <= 0.0f) return 0.0f;
    if (L > 1.0f) L = 1.0f;
    const float m1 = 0.1593017578125f;
    const float m2 = 78.84375f;
    const float c1 = 0.8359375f;
    const float c2 = 18.8515625f;
    const float c3 = 18.6875f;
    float lm1 = powf(L, m1);
    float num = c1 + c2 * lm1;
    float den = 1.0f + c3 * lm1;
    return powf(num / den, m2);
}

__device__ __forceinline__ float3 hdr_to_pq2020(float4 px, float exposure) {
    float r = fmaxf(0.0f, px.x * exposure);
    float g = fmaxf(0.0f, px.y * exposure);
    float b = fmaxf(0.0f, px.z * exposure);
    // Soft highlight rolloff toward 1000 nits peak (paper white = 203 nits, knee = 750 nits).
    const float paper_white = 203.0f;
    const float peak = 1000.0f;
    const float knee = 750.0f;
    float rn = r * paper_white;
    float gn = g * paper_white;
    float bn = b * paper_white;
    float mx = fmaxf(rn, fmaxf(gn, bn));
    if (mx > knee) {
        float excess = mx - knee;
        float span = peak - knee;
        float compressed = knee + span * (excess / (excess + span));
        float s = compressed / mx;
        rn *= s;
        gn *= s;
        bn *= s;
    }
    // Linear Rec.709 -> Linear Rec.2020 exact matrix.
    float r2020 = fmaxf(0.0f, 0.6274039f * rn + 0.3292830f * gn + 0.0433131f * bn);
    float g2020 = fmaxf(0.0f, 0.0690973f * rn + 0.9195404f * gn + 0.0113623f * bn);
    float b2020 = fmaxf(0.0f, 0.0163914f * rn + 0.0880133f * gn + 0.8955953f * bn);
    return make_float3(pq_oetf(r2020), pq_oetf(g2020), pq_oetf(b2020));
}

extern "C" __global__ void hdr_f32_to_p010_pq(
    const float4* __restrict__ hdr,
    unsigned short* __restrict__ p010,
    unsigned int w,
    unsigned int h,
    unsigned int p010_pitch_u16,
    float exposure
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= w || y >= h) return;

    float3 pq = hdr_to_pq2020(hdr[y * w + x], exposure);
    // BT.2020 NCL luma (10-bit limited range: Y in [64, 940]).
    float yn = 0.2627f * pq.x + 0.6780f * pq.y + 0.0593f * pq.z;
    p010[static_cast<size_t>(y) * p010_pitch_u16 + x] = pack_p010(64.0f + 876.0f * yn);

    if ((x & 1u) == 0u && (y & 1u) == 0u) {
        float3 p01 = hdr_to_pq2020(hdr[y * w + (x + 1u)], exposure);
        float3 p10 = hdr_to_pq2020(hdr[(y + 1u) * w + x], exposure);
        float3 p11 = hdr_to_pq2020(hdr[(y + 1u) * w + (x + 1u)], exposure);
        float r_avg = 0.25f * (pq.x + p01.x + p10.x + p11.x);
        float g_avg = 0.25f * (pq.y + p01.y + p10.y + p11.y);
        float b_avg = 0.25f * (pq.z + p01.z + p10.z + p11.z);
        float y_avg = 0.2627f * r_avg + 0.6780f * g_avg + 0.0593f * b_avg;
        float cb = 512.0f + 896.0f * ((b_avg - y_avg) / 1.8814f);
        float cr = 512.0f + 896.0f * ((r_avg - y_avg) / 1.4746f);

        unsigned short* uv_plane = p010 + static_cast<size_t>(h) * p010_pitch_u16;
        size_t uv_idx = static_cast<size_t>(y >> 1u) * p010_pitch_u16 + x;
        uv_plane[uv_idx + 0u] = pack_p010(cb);
        uv_plane[uv_idx + 1u] = pack_p010(cr);
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

/// Hardware MP4 video recorder using `nvenc` and zero-copy GPU color conversion kernels.
///
/// Supports both iPhone-compatible 8-bit H.264 SDR (`avc1`, BT.709) and 10-bit HEVC HDR10
/// (`hvc1`, BT.2020 SMPTE ST 2084 PQ) with per-frame `stts` timestamps.
pub struct VideoRecorder {
    cuda: Arc<CudaContext>,
    kernel: CudaKernel,
    profile: VideoProfile,
    // Drop order invariant: `yuv_res` and `bitstream` must drop before `yuv_buf` and `session`.
    yuv_res: RegisteredResource,
    bitstream: OutputBitstream,
    yuv_buf: DeviceBuffer<u8>,
    session: Arc<EncodeSession>,
    muxer: Mp4Muxer,
    width: u32,
    height: u32,
    /// Row stride in elements (`u8` for NV12, `u16` for P010).
    yuv_pitch_elems: u32,
    frame_idx: u32,
}

impl VideoRecorder {
    /// Opens an NVENC session (`H.264` SDR or `HEVC` Main10 PQ10) and creates an `.mp4` at `path`.
    ///
    /// Both `w` and `h` must be non-zero and even (required by 4:2:0 chroma subsampling).
    pub fn start(
        cuda: &Arc<CudaContext>,
        w: u32,
        h: u32,
        fps: u32,
        profile: VideoProfile,
        path: &Path,
    ) -> Result<Self, String> {
        if w == 0 || h == 0 || !w.is_multiple_of(2) || !h.is_multiple_of(2) {
            return Err(format!(
                "4:2:0 video dimensions ({w}x{h}) must be non-zero and even"
            ));
        }
        if fps == 0 {
            return Err("video FPS must be non-zero".to_string());
        }

        cuda.bind_to_thread()
            .map_err(|e| format!("CUDA bind failed: {e}"))?;

        let (kernel, codec, buf_fmt, bitrate) = match profile {
            VideoProfile::SdrH264 => (
                CudaKernel::compile(cuda, RGBA8_TO_NV12_CU, "rgba8_to_nv12.cu", "rgba8_to_nv12")?,
                Codec::H264,
                BufferFormat::Nv12,
                16_000_000u32,
            ),
            VideoProfile::Hdr10Pq => (
                CudaKernel::compile(
                    cuda,
                    HDR_F32_TO_P010_PQ_CU,
                    "hdr_f32_to_p010_pq.cu",
                    "hdr_f32_to_p010_pq",
                )?,
                Codec::Hevc,
                BufferFormat::Yuv420_10Bit,
                24_000_000u32,
            ),
        };

        let mut session = EncodeSession::open(cuda)
            .map_err(|e| format!("NVENC unavailable or session open failed: {e}"))?;

        let mut enc_cfg = session
            .preset_config(codec, Preset::P4, Tuning::HighQuality)
            .map_err(|e| format!("NVENC preset_config ({codec:?}) failed: {e}"))?;
        // IPP GOP with no B-frames so every `encode_picture` yields a packet immediately.
        enc_cfg.set_frame_interval_p(1);
        enc_cfg.set_gop_length(fps);
        enc_cfg.set_average_bitrate(bitrate);
        match profile {
            VideoProfile::SdrH264 => enc_cfg.set_h264_bt709_vui(),
            VideoProfile::Hdr10Pq => enc_cfg.set_hevc_10bit_pq(),
        }

        let mut session_cfg = SessionConfig::new(codec, w, h, fps, 1);
        session_cfg.preset = Preset::P4;
        session_cfg.tuning = Tuning::HighQuality;
        session_cfg.encode_config = Some(enc_cfg);

        session
            .initialize(&session_cfg)
            .map_err(|e| format!("NVENC initialize ({profile:?}) failed: {e}"))?;

        // `RegisteredResource::from_cuda_ptr` requires a 4-byte-aligned byte pitch.
        let (yuv_pitch_elems, yuv_pitch_bytes) = match profile {
            VideoProfile::SdrH264 => {
                let p = w
                    .checked_next_multiple_of(4)
                    .ok_or_else(|| "NV12 pitch overflow".to_string())?;
                (p, p)
            }
            VideoProfile::Hdr10Pq => {
                let p_elems = w
                    .checked_next_multiple_of(2)
                    .ok_or_else(|| "P010 pitch overflow".to_string())?;
                let p_bytes = p_elems
                    .checked_mul(2)
                    .ok_or_else(|| "P010 byte pitch overflow".to_string())?;
                (p_elems, p_bytes)
            }
        };
        let yuv_bytes = (yuv_pitch_bytes as usize)
            .checked_mul(h as usize)
            .and_then(|luma| luma.checked_mul(3))
            .map(|total| total / 2)
            .ok_or_else(|| "YUV buffer size overflow".to_string())?;

        let stream = cuda
            .new_stream()
            .map_err(|e| format!("CUDA stream creation failed: {e}"))?;
        let yuv_buf = DeviceBuffer::<u8>::zeroed(&stream, yuv_bytes)
            .map_err(|e| format!("CUDA YUV allocation failed: {e}"))?;
        stream
            .synchronize()
            .map_err(|e| format!("CUDA stream sync failed: {e}"))?;

        // SAFETY: `yuv_buf` is allocated on `cuda` with `yuv_pitch_bytes * h * 3 / 2` bytes and is
        // declared after `yuv_res` in `VideoRecorder` so it outlives `yuv_res`.
        let yuv_res = unsafe {
            RegisteredResource::from_cuda_ptr(
                &session,
                yuv_buf.cu_deviceptr(),
                w,
                h,
                yuv_pitch_bytes,
                buf_fmt,
            )
        }
        .map_err(|e| format!("NVENC register YUV resource failed: {e}"))?;

        let bitstream = OutputBitstream::new(&session)
            .map_err(|e| format!("NVENC create bitstream buffer failed: {e}"))?;

        let muxer = Mp4Muxer::create(path, w, h, profile)?;

        Ok(Self {
            cuda: cuda.clone(),
            kernel,
            profile,
            yuv_res,
            bitstream,
            yuv_buf,
            session,
            muxer,
            width: w,
            height: h,
            yuv_pitch_elems,
            frame_idx: 0,
        })
    }

    /// Encodes one frame into the MP4 stream with explicit `duration_secs` (real-time or fixed).
    ///
    /// Uses `rgba_dev_ptr` (`RGBA8`) for [`VideoProfile::SdrH264`] and `hdr_f32_dev_ptr` (linear
    /// HDR `float4`) with `exposure` for [`VideoProfile::Hdr10Pq`].
    pub fn push_frame(
        &mut self,
        stream: &Stream,
        rgba_dev_ptr: u64,
        hdr_f32_dev_ptr: u64,
        exposure: f32,
        duration_secs: f64,
    ) -> Result<(), String> {
        self.cuda
            .bind_to_thread()
            .map_err(|e| format!("CUDA bind failed: {e}"))?;

        let gx = self.width.div_ceil(16);
        let gy = self.height.div_ceil(16);
        let mut dst_ptr = self.yuv_buf.cu_deviceptr();
        let mut w = self.width;
        let mut h = self.height;
        let mut pitch = self.yuv_pitch_elems;

        let launch_res = match self.profile {
            VideoProfile::SdrH264 => {
                if rgba_dev_ptr == 0 {
                    return Err("rgba_dev_ptr is null".to_string());
                }
                let mut src_ptr = rgba_dev_ptr;
                let mut args: [*mut c_void; 5] = [
                    (&raw mut src_ptr).cast(),
                    (&raw mut dst_ptr).cast(),
                    (&raw mut w).cast(),
                    (&raw mut h).cast(),
                    (&raw mut pitch).cast(),
                ];
                // SAFETY: `rgba8_to_nv12` expects `(const u8*, u8*, u32, u32, u32)` with valid
                // device buffers; `stream.synchronize()` completes before NVENC maps `yuv_buf`.
                unsafe {
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
                }
            }
            VideoProfile::Hdr10Pq => {
                if hdr_f32_dev_ptr == 0 {
                    return Err("hdr_f32_dev_ptr is null".to_string());
                }
                let mut src_ptr = hdr_f32_dev_ptr;
                let mut exp = exposure;
                let mut args: [*mut c_void; 6] = [
                    (&raw mut src_ptr).cast(),
                    (&raw mut dst_ptr).cast(),
                    (&raw mut w).cast(),
                    (&raw mut h).cast(),
                    (&raw mut pitch).cast(),
                    (&raw mut exp).cast(),
                ];
                // SAFETY: `hdr_f32_to_p010_pq` expects `(const float4*, u16*, u32, u32, u32, f32)`
                // with valid device buffers; `stream.synchronize()` completes before NVENC maps.
                unsafe {
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
                }
            }
        };
        if launch_res != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("YUV conversion cuLaunchKernel failed: {launch_res:?}"));
        }

        stream
            .synchronize()
            .map_err(|e| format!("CUDA stream synchronize failed: {e}"))?;

        let mapped = self
            .yuv_res
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
        let duration_ticks = (duration_secs * f64::from(MP4_TIMESCALE))
            .round()
            .clamp(150.0, f64::from(MP4_TIMESCALE) * 2.0) as u32;
        self.muxer.write_annexb_frame(lock.data(), duration_ticks)?;

        self.frame_idx = self.frame_idx.saturating_add(1);
        Ok(())
    }

    /// Flushes the NVENC encoder and finalizes the MP4 container, returning total frames written.
    pub fn finish(self) -> Result<u64, String> {
        self.cuda
            .bind_to_thread()
            .map_err(|e| format!("CUDA bind failed: {e}"))?;
        self.session
            .flush_eos()
            .map_err(|e| format!("NVENC flush_eos failed: {e}"))?;
        self.muxer.finish()
    }

    /// Number of frames encoded so far.
    pub fn frame_count(&self) -> u64 {
        self.muxer.frame_count()
    }
}
