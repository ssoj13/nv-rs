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

/// Pixel format of a CUDA device buffer passed into still image or video export pipelines.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourcePixelFormat {
    /// 8-bit UNORM / sRGB `R, G, B, A` (4 bytes/pixel).
    Rgba8,
    /// 8-bit UNORM / sRGB `B, G, R, A` (4 bytes/pixel, typical Windows Vulkan swapchain).
    Bgra8,
    /// 16-bit float linear scRGB Rec.709 (`1.0 = 80 nits`, 8 bytes/pixel).
    ScrgbF16,
}

impl SourcePixelFormat {
    /// Byte stride of a single pixel in this format.
    #[inline]
    pub const fn bytes_per_pixel(self) -> u32 {
        match self {
            Self::Rgba8 | Self::Bgra8 => 4,
            Self::ScrgbF16 => 8,
        }
    }

    /// Maps a `wgpu::TextureFormat` to its matching [`SourcePixelFormat`], if supported.
    #[inline]
    pub const fn from_wgpu(fmt: wgpu::TextureFormat) -> Option<Self> {
        match fmt {
            wgpu::TextureFormat::Rgba8Unorm | wgpu::TextureFormat::Rgba8UnormSrgb => {
                Some(Self::Rgba8)
            }
            wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Bgra8UnormSrgb => {
                Some(Self::Bgra8)
            }
            wgpu::TextureFormat::Rgba16Float => Some(Self::ScrgbF16),
            _ => None,
        }
    }
}

/// Rounds dimensions down to the largest even pair required by 4:2:0 video codecs (H.264/HEVC).
///
/// Returns `None` if either dimension is smaller than 2.
#[inline]
pub const fn align_even_dims(w: u32, h: u32) -> Option<(u32, u32)> {
    let ew = w & !1;
    let eh = h & !1;
    if ew == 0 || eh == 0 {
        None
    } else {
        Some((ew, eh))
    }
}

/// Computes the 256-byte-aligned row stride `(bytes_per_row, pitch_pixels)` required by
/// `wgpu::CommandEncoder::copy_texture_to_buffer` (`COPY_BYTES_PER_ROW_ALIGNMENT = 256`).
#[inline]
pub fn copy_pitch_for_width(w: u32, bpp: u32) -> Option<(u32, u32)> {
    if w == 0 || bpp == 0 {
        return None;
    }
    let raw_bpr = w.checked_mul(bpp)?;
    let aligned_bpr = raw_bpr.checked_next_multiple_of(256)?;
    if !aligned_bpr.is_multiple_of(bpp) {
        return None;
    }
    Some((aligned_bpr, aligned_bpr / bpp))
}

/// Converts an IEEE-754 binary16 bit pattern to `f32` without extra crate dependencies.
#[inline]
fn f16_bits_to_f32(bits: u16) -> f32 {
    let sign = u32::from(bits >> 15) << 31;
    let exp = u32::from((bits >> 10) & 0x1f);
    let frac = u32::from(bits & 0x03ff);
    let f32_bits = if exp == 0 {
        if frac == 0 {
            sign
        } else {
            // Subnormal half -> normalize into f32
            let mut m = frac;
            let mut e = 127i32 - 14;
            while (m & 0x0400) == 0 {
                m <<= 1;
                e -= 1;
            }
            m &= 0x03ff;
            sign | ((e as u32) << 23) | (m << 13)
        }
    } else if exp == 31 {
        sign | (0xff << 23) | (frac << 13)
    } else {
        sign | ((exp + (127 - 15)) << 23) | (frac << 13)
    };
    f32::from_bits(f32_bits)
}

/// Maps linear scRGB (`1.0 = 80 nits`, `203/80 = 2.5375` paper white) to 8-bit sRGB without
/// double-tonemapping.
#[inline]
fn scrgb_channel_to_srgb_u8(v: f32) -> u8 {
    if !v.is_finite() || v <= 0.0 {
        return 0;
    }
    const INV_PAPER_WHITE: f32 = 80.0 / 203.0;
    let lin = (v * INV_PAPER_WHITE).clamp(0.0, 1.0);
    let srgb = lin.powf(1.0 / 2.2);
    (srgb * 255.0 + 0.5).clamp(0.0, 255.0) as u8
}

/// Unpacks a pitched host buffer in [`SourcePixelFormat`] into tightly packed `RGBA8` (`w * h * 4`)
/// inside `dst`, reusing `dst`'s heap allocation across calls.
pub fn unpack_pitched_to_rgba8(
    src: &[u8],
    dst: &mut Vec<u8>,
    w: u32,
    h: u32,
    src_pitch_px: u32,
    fmt: SourcePixelFormat,
) -> Result<(), String> {
    if w == 0 || h == 0 {
        return Err("image dimensions must be non-zero".to_string());
    }
    if src_pitch_px < w {
        return Err(format!("source pitch ({src_pitch_px}) < width ({w})"));
    }
    let bpp = fmt.bytes_per_pixel() as usize;
    let row_stride = (src_pitch_px as usize)
        .checked_mul(bpp)
        .ok_or_else(|| "source row stride overflow".to_string())?;
    let required_src = row_stride
        .checked_mul(h as usize)
        .ok_or_else(|| "source byte length overflow".to_string())?;
    if src.len() < required_src {
        return Err(format!(
            "source buffer too small ({} B < required {required_src} B)",
            src.len()
        ));
    }
    let out_len = (w as usize)
        .checked_mul(h as usize)
        .and_then(|px| px.checked_mul(4))
        .ok_or_else(|| "RGBA8 output size overflow".to_string())?;
    dst.resize(out_len, 0);

    let w_usize = w as usize;
    match fmt {
        SourcePixelFormat::Rgba8 => {
            let row_bytes = w_usize * 4;
            for y in 0..(h as usize) {
                let s0 = y * row_stride;
                let d0 = y * row_bytes;
                dst[d0..d0 + row_bytes].copy_from_slice(&src[s0..s0 + row_bytes]);
                // Force opaque alpha so PNG viewers never show semi-transparent UI chrome.
                for px in dst[d0..d0 + row_bytes].as_chunks_mut::<4>().0 {
                    px[3] = 255;
                }
            }
        }
        SourcePixelFormat::Bgra8 => {
            let row_bytes = w_usize * 4;
            for y in 0..(h as usize) {
                let s_row = &src[y * row_stride..y * row_stride + row_bytes];
                let d_row = &mut dst[y * row_bytes..(y + 1) * row_bytes];
                for (s_px, d_px) in s_row
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .zip(d_row.as_chunks_mut::<4>().0)
                {
                    d_px[0] = s_px[2];
                    d_px[1] = s_px[1];
                    d_px[2] = s_px[0];
                    d_px[3] = 255;
                }
            }
        }
        SourcePixelFormat::ScrgbF16 => {
            let row_bytes_in = w_usize * 8;
            let row_bytes_out = w_usize * 4;
            for y in 0..(h as usize) {
                let s_row = &src[y * row_stride..y * row_stride + row_bytes_in];
                let d_row = &mut dst[y * row_bytes_out..(y + 1) * row_bytes_out];
                for (s_px, d_px) in s_row
                    .as_chunks::<8>()
                    .0
                    .iter()
                    .zip(d_row.as_chunks_mut::<4>().0)
                {
                    let r = f16_bits_to_f32(u16::from_le_bytes([s_px[0], s_px[1]]));
                    let g = f16_bits_to_f32(u16::from_le_bytes([s_px[2], s_px[3]]));
                    let b = f16_bits_to_f32(u16::from_le_bytes([s_px[4], s_px[5]]));
                    d_px[0] = scrgb_channel_to_srgb_u8(r);
                    d_px[1] = scrgb_channel_to_srgb_u8(g);
                    d_px[2] = scrgb_channel_to_srgb_u8(b);
                    d_px[3] = 255;
                }
            }
        }
    }
    Ok(())
}

/// Reads a pitched CUDA device buffer in [`SourcePixelFormat`], converts to tightly packed `RGBA8`,
/// and writes a lossless PNG file to `path`, reusing `readback_scratch` and `rgba_scratch`.
#[expect(
    clippy::too_many_arguments,
    reason = "explicit CUDA context, pitched source descriptor, output path, and reusable scratch buffers"
)]
pub fn save_png_from_pitched(
    cuda: &Arc<CudaContext>,
    src_dev_ptr: u64,
    w: u32,
    h: u32,
    src_pitch_px: u32,
    fmt: SourcePixelFormat,
    path: &Path,
    readback_scratch: &mut Vec<u8>,
    rgba_scratch: &mut Vec<u8>,
) -> Result<(), String> {
    if src_dev_ptr == 0 {
        return Err("PNG source device pointer is null".to_string());
    }
    if w == 0 || h == 0 {
        return Err("PNG image dimensions must be non-zero".to_string());
    }
    if src_pitch_px < w {
        return Err(format!("PNG source pitch ({src_pitch_px}) < width ({w})"));
    }
    let byte_len = (src_pitch_px as usize)
        .checked_mul(h as usize)
        .and_then(|px| px.checked_mul(fmt.bytes_per_pixel() as usize))
        .ok_or_else(|| "PNG readback buffer size overflow".to_string())?;
    readback_scratch.resize(byte_len, 0);

    cuda.bind_to_thread()
        .map_err(|e| format!("CUDA bind failed: {e}"))?;
    // SAFETY: `src_dev_ptr` points to a valid CUDA allocation of at least `byte_len` bytes on
    // `cuda`, and `readback_scratch` has `byte_len` writable host bytes.
    let res = unsafe { cu::cuMemcpyDtoH_v2(readback_scratch.as_mut_ptr().cast(), src_dev_ptr, byte_len) };
    if res != cu::cudaError_enum_CUDA_SUCCESS {
        return Err(format!("CUDA PNG readback failed ({res:?})"));
    }

    unpack_pitched_to_rgba8(readback_scratch, rgba_scratch, w, h, src_pitch_px, fmt)?;

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create '{}': {e}", parent.display()))?;
    }
    let file = std::fs::File::create(path)
        .map_err(|e| format!("failed to create '{}': {e}", path.display()))?;
    let writer = std::io::BufWriter::new(file);
    let mut enc = png::Encoder::new(writer, w, h);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header()
        .and_then(|mut wtr| wtr.write_image_data(rgba_scratch))
        .map_err(|e| format!("PNG encode failed: {e}"))
}

const RGBA8_TO_RGB8_CU: &str = r#"
extern "C" __global__ void rgba8_to_rgb8(
    const unsigned char* __restrict__ rgba,
    unsigned char* __restrict__ rgb,
    unsigned int w,
    unsigned int h,
    unsigned int src_pitch_px,
    unsigned int swap_rb
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= w || y >= h) return;
    unsigned int s = (y * src_pitch_px + x) * 4u;
    unsigned int d = (y * w + x) * 3u;
    unsigned int r_ch = (swap_rb != 0u) ? 2u : 0u;
    unsigned int b_ch = (swap_rb != 0u) ? 0u : 2u;
    rgb[d + 0u] = rgba[s + r_ch];
    rgb[d + 1u] = rgba[s + 1u];
    rgb[d + 2u] = rgba[s + b_ch];
}
"#;

const SCRGB_F16_TO_RGB8_CU: &str = r#"
__device__ __forceinline__ float f16_to_f32(unsigned short h) {
    float v;
    asm("cvt.f32.f16 %0, %1;" : "=f"(v) : "h"(h));
    return v;
}

__device__ __forceinline__ unsigned char scrgb_to_srgb_u8(float v) {
    if (!(v > 0.0f)) return 0u;
    float lin = v * (80.0f / 203.0f);
    if (lin > 1.0f) lin = 1.0f;
    float s = powf(lin, 1.0f / 2.2f) * 255.0f + 0.5f;
    if (s > 255.0f) s = 255.0f;
    return static_cast<unsigned char>(s);
}

extern "C" __global__ void scrgb_f16_to_rgb8(
    const ushort4* __restrict__ scrgb,
    unsigned char* __restrict__ rgb,
    unsigned int w,
    unsigned int h,
    unsigned int src_pitch_px,
    unsigned int /*swap_rb*/
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= w || y >= h) return;
    ushort4 px = scrgb[static_cast<size_t>(y) * src_pitch_px + x];
    unsigned int d = (y * w + x) * 3u;
    rgb[d + 0u] = scrgb_to_srgb_u8(f16_to_f32(px.x));
    rgb[d + 1u] = scrgb_to_srgb_u8(f16_to_f32(px.y));
    rgb[d + 2u] = scrgb_to_srgb_u8(f16_to_f32(px.z));
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
    unsigned int src_pitch_px,
    unsigned int nv12_pitch,
    unsigned int swap_rb
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= w || y >= h) return;

    unsigned int r_ch = (swap_rb != 0u) ? 2u : 0u;
    unsigned int b_ch = (swap_rb != 0u) ? 0u : 2u;

    unsigned int s = (y * src_pitch_px + x) * 4u;
    float r = static_cast<float>(rgba[s + r_ch]);
    float g = static_cast<float>(rgba[s + 1u]);
    float b = static_cast<float>(rgba[s + b_ch]);

    // BT.709 limited-range luma (Y in [16, 235]).
    float y_val = 16.0f + (0.182586f * r + 0.614231f * g + 0.062007f * b);
    nv12[y * nv12_pitch + x] = clamp_u8(y_val);

    // Subsampled 4:2:0 chroma (2x2 box filter) on even coordinates.
    if ((x & 1u) == 0u && (y & 1u) == 0u) {
        unsigned int s01 = (y * src_pitch_px + (x + 1u)) * 4u;
        unsigned int s10 = ((y + 1u) * src_pitch_px + x) * 4u;
        unsigned int s11 = ((y + 1u) * src_pitch_px + (x + 1u)) * 4u;

        float r_avg = 0.25f * (r + static_cast<float>(rgba[s01 + r_ch])
            + static_cast<float>(rgba[s10 + r_ch]) + static_cast<float>(rgba[s11 + r_ch]));
        float g_avg = 0.25f * (g + static_cast<float>(rgba[s01 + 1u])
            + static_cast<float>(rgba[s10 + 1u]) + static_cast<float>(rgba[s11 + 1u]));
        float b_avg = 0.25f * (b + static_cast<float>(rgba[s01 + b_ch])
            + static_cast<float>(rgba[s10 + b_ch]) + static_cast<float>(rgba[s11 + b_ch]));

        float u_val = 128.0f + (-0.100644f * r_avg - 0.338572f * g_avg + 0.439216f * b_avg);
        float v_val = 128.0f + (0.439216f * r_avg - 0.398942f * g_avg - 0.040274f * b_avg);

        unsigned char* uv_plane = nv12 + static_cast<size_t>(h) * nv12_pitch;
        size_t uv_idx = static_cast<size_t>(y >> 1u) * nv12_pitch + x;
        uv_plane[uv_idx + 0u] = clamp_u8(u_val);
        uv_plane[uv_idx + 1u] = clamp_u8(v_val);
    }
}
"#;

const SCRGB_F16_TO_NV12_CU: &str = r#"
__device__ __forceinline__ float f16_to_f32(unsigned short h) {
    float v;
    asm("cvt.f32.f16 %0, %1;" : "=f"(v) : "h"(h));
    return v;
}

__device__ __forceinline__ unsigned char clamp_u8(float v) {
    if (v < 0.0f) return 0u;
    if (v > 255.0f) return 255u;
    return static_cast<unsigned char>(v + 0.5f);
}

__device__ __forceinline__ float scrgb_ch_to_srgb255(unsigned short h) {
    float v = f16_to_f32(h);
    if (!(v > 0.0f)) return 0.0f;
    float lin = v * (80.0f / 203.0f);
    if (lin > 1.0f) lin = 1.0f;
    return powf(lin, 1.0f / 2.2f) * 255.0f;
}

__device__ __forceinline__ float3 scrgb_to_srgb255(ushort4 px) {
    return make_float3(
        scrgb_ch_to_srgb255(px.x),
        scrgb_ch_to_srgb255(px.y),
        scrgb_ch_to_srgb255(px.z)
    );
}

extern "C" __global__ void scrgb_f16_to_nv12(
    const ushort4* __restrict__ scrgb,
    unsigned char* __restrict__ nv12,
    unsigned int w,
    unsigned int h,
    unsigned int src_pitch_px,
    unsigned int nv12_pitch,
    unsigned int /*swap_rb*/
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= w || y >= h) return;

    float3 c00 = scrgb_to_srgb255(scrgb[static_cast<size_t>(y) * src_pitch_px + x]);
    float y_val = 16.0f + (0.182586f * c00.x + 0.614231f * c00.y + 0.062007f * c00.z);
    nv12[static_cast<size_t>(y) * nv12_pitch + x] = clamp_u8(y_val);

    if ((x & 1u) == 0u && (y & 1u) == 0u) {
        float3 c01 = scrgb_to_srgb255(scrgb[static_cast<size_t>(y) * src_pitch_px + (x + 1u)]);
        float3 c10 = scrgb_to_srgb255(scrgb[static_cast<size_t>(y + 1u) * src_pitch_px + x]);
        float3 c11 = scrgb_to_srgb255(scrgb[static_cast<size_t>(y + 1u) * src_pitch_px + (x + 1u)]);
        float r_avg = 0.25f * (c00.x + c01.x + c10.x + c11.x);
        float g_avg = 0.25f * (c00.y + c01.y + c10.y + c11.y);
        float b_avg = 0.25f * (c00.z + c01.z + c10.z + c11.z);

        float u_val = 128.0f + (-0.100644f * r_avg - 0.338572f * g_avg + 0.439216f * b_avg);
        float v_val = 128.0f + (0.439216f * r_avg - 0.398942f * g_avg - 0.040274f * b_avg);

        unsigned char* uv_plane = nv12 + static_cast<size_t>(h) * nv12_pitch;
        size_t uv_idx = static_cast<size_t>(y >> 1u) * nv12_pitch + x;
        uv_plane[uv_idx + 0u] = clamp_u8(u_val);
        uv_plane[uv_idx + 1u] = clamp_u8(v_val);
    }
}
"#;

const SCRGB_F16_TO_P010_PQ_CU: &str = r#"
__device__ __forceinline__ float f16_to_f32(unsigned short h) {
    float v;
    asm("cvt.f32.f16 %0, %1;" : "=f"(v) : "h"(h));
    return v;
}

__device__ __forceinline__ unsigned short pack_p010(float v10) {
    float c = v10 < 0.0f ? 0.0f : (v10 > 1023.0f ? 1023.0f : v10);
    unsigned int q = static_cast<unsigned int>(c + 0.5f);
    return static_cast<unsigned short>((q & 1023u) << 6u);
}

__device__ __forceinline__ float pq_oetf(float nits) {
    float L = nits * 0.0001f; // normalized to 10,000 nits
    if (!(L > 0.0f)) return 0.0f;
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

__device__ __forceinline__ float3 scrgb_to_pq2020(ushort4 px) {
    // scRGB linear Rec.709 (1.0 = 80 nits) -> nits -> Linear Rec.2020 -> SMPTE ST 2084 PQ.
    float rn = fmaxf(0.0f, f16_to_f32(px.x) * 80.0f);
    float gn = fmaxf(0.0f, f16_to_f32(px.y) * 80.0f);
    float bn = fmaxf(0.0f, f16_to_f32(px.z) * 80.0f);
    float r2020 = fmaxf(0.0f, 0.6274039f * rn + 0.3292830f * gn + 0.0433131f * bn);
    float g2020 = fmaxf(0.0f, 0.0690973f * rn + 0.9195404f * gn + 0.0113623f * bn);
    float b2020 = fmaxf(0.0f, 0.0163914f * rn + 0.0880133f * gn + 0.8955953f * bn);
    return make_float3(pq_oetf(r2020), pq_oetf(g2020), pq_oetf(b2020));
}

extern "C" __global__ void scrgb_f16_to_p010_pq(
    const ushort4* __restrict__ scrgb,
    unsigned short* __restrict__ p010,
    unsigned int w,
    unsigned int h,
    unsigned int src_pitch_px,
    unsigned int p010_pitch_u16,
    unsigned int /*swap_rb*/
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= w || y >= h) return;

    float3 pq = scrgb_to_pq2020(scrgb[static_cast<size_t>(y) * src_pitch_px + x]);
    // BT.2020 NCL luma (10-bit limited range: Y in [64, 940]).
    float yn = 0.2627f * pq.x + 0.6780f * pq.y + 0.0593f * pq.z;
    p010[static_cast<size_t>(y) * p010_pitch_u16 + x] = pack_p010(64.0f + 876.0f * yn);

    if ((x & 1u) == 0u && (y & 1u) == 0u) {
        float3 p01 = scrgb_to_pq2020(scrgb[static_cast<size_t>(y) * src_pitch_px + (x + 1u)]);
        float3 p10 = scrgb_to_pq2020(scrgb[static_cast<size_t>(y + 1u) * src_pitch_px + x]);
        float3 p11 = scrgb_to_pq2020(scrgb[static_cast<size_t>(y + 1u) * src_pitch_px + (x + 1u)]);
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

/// Converts a tightly packed CUDA `RGBA8` device buffer (`w x h`) to JPEG using `nvjpeg`.
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
    save_jpeg_from_pitched(
        cuda,
        stream,
        rgba_dev_ptr,
        w,
        h,
        w,
        SourcePixelFormat::Rgba8,
        path,
        quality,
    )
}

/// Converts a pitched CUDA device buffer (`src_pitch_px >= w`) in [`SourcePixelFormat`] to JPEG
/// using `nvjpeg` and writes it to `path`.
#[expect(
    clippy::too_many_arguments,
    reason = "explicit CUDA context, stream, pitched source descriptor, format, output path, and quality"
)]
pub fn save_jpeg_from_pitched(
    cuda: &Arc<CudaContext>,
    stream: &Stream,
    src_dev_ptr: u64,
    w: u32,
    h: u32,
    src_pitch_px: u32,
    fmt: SourcePixelFormat,
    path: &Path,
    quality: u8,
) -> Result<(), String> {
    if src_dev_ptr == 0 {
        return Err("src_dev_ptr is null".to_string());
    }
    if w == 0 || h == 0 {
        return Err("image dimensions must be non-zero".to_string());
    }
    if src_pitch_px < w {
        return Err(format!("JPEG source pitch ({src_pitch_px}) < width ({w})"));
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

    let (kernel, mut swap_rb) = match fmt {
        SourcePixelFormat::Rgba8 => (
            CudaKernel::compile(cuda, RGBA8_TO_RGB8_CU, "rgba8_to_rgb8.cu", "rgba8_to_rgb8")?,
            0u32,
        ),
        SourcePixelFormat::Bgra8 => (
            CudaKernel::compile(cuda, RGBA8_TO_RGB8_CU, "rgba8_to_rgb8.cu", "rgba8_to_rgb8")?,
            1u32,
        ),
        SourcePixelFormat::ScrgbF16 => (
            CudaKernel::compile(
                cuda,
                SCRGB_F16_TO_RGB8_CU,
                "scrgb_f16_to_rgb8.cu",
                "scrgb_f16_to_rgb8",
            )?,
            0u32,
        ),
    };
    let mut src_ptr = src_dev_ptr;
    let mut dst_ptr = rgb_buf.cu_deviceptr();
    let mut width = w;
    let mut height = h;
    let mut pitch_px = src_pitch_px;
    let mut args: [*mut c_void; 6] = [
        (&raw mut src_ptr).cast(),
        (&raw mut dst_ptr).cast(),
        (&raw mut width).cast(),
        (&raw mut height).cast(),
        (&raw mut pitch_px).cast(),
        (&raw mut swap_rb).cast(),
    ];
    let gx = w.div_ceil(16);
    let gy = h.div_ceil(16);
    // SAFETY: Both `rgba8_to_rgb8` and `scrgb_f16_to_rgb8` take
    // `(const ptr*, u8*, u32 w, u32 h, u32 src_pitch_px, u32 swap_rb)` with valid device pointers
    // of at least `src_pitch_px * h * bpp` and `w * h * 3` bytes respectively; `rgb_buf` lives past
    // `stream.synchronize()`.
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
        return Err(format!("RGB8 conversion cuLaunchKernel failed: {launch_res:?}"));
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

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create '{}': {e}", parent.display()))?;
    }
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
    stream: Arc<Stream>,
    kernel: CudaKernel,
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
    swap_rb: u32,
    frame_idx: u32,
}

impl VideoRecorder {
    /// Opens an NVENC session (`H.264` SDR or `HEVC` Main10 PQ10) and creates an `.mp4` at `path`,
    /// using the default source format (`Rgba8` for SDR H.264, `ScrgbF16` for HDR10 PQ).
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
        let src_fmt = match profile {
            VideoProfile::SdrH264 => SourcePixelFormat::Rgba8,
            VideoProfile::Hdr10Pq => SourcePixelFormat::ScrgbF16,
        };
        Self::start_with_format(cuda, w, h, fps, profile, src_fmt, path)
    }

    /// Opens an NVENC session with an explicit [`SourcePixelFormat`] (`Rgba8`, `Bgra8`, or
    /// `ScrgbF16`) for zero-copy GPU color conversion directly from viewport or full-window targets.
    pub fn start_with_format(
        cuda: &Arc<CudaContext>,
        w: u32,
        h: u32,
        fps: u32,
        profile: VideoProfile,
        src_fmt: SourcePixelFormat,
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

        let (kernel, codec, buf_fmt, bitrate, swap_rb) = match (profile, src_fmt) {
            (VideoProfile::SdrH264, SourcePixelFormat::Rgba8) => (
                CudaKernel::compile(cuda, RGBA8_TO_NV12_CU, "rgba8_to_nv12.cu", "rgba8_to_nv12")?,
                Codec::H264,
                BufferFormat::Nv12,
                16_000_000u32,
                0u32,
            ),
            (VideoProfile::SdrH264, SourcePixelFormat::Bgra8) => (
                CudaKernel::compile(cuda, RGBA8_TO_NV12_CU, "rgba8_to_nv12.cu", "rgba8_to_nv12")?,
                Codec::H264,
                BufferFormat::Nv12,
                16_000_000u32,
                1u32,
            ),
            (VideoProfile::SdrH264, SourcePixelFormat::ScrgbF16) => (
                CudaKernel::compile(
                    cuda,
                    SCRGB_F16_TO_NV12_CU,
                    "scrgb_f16_to_nv12.cu",
                    "scrgb_f16_to_nv12",
                )?,
                Codec::H264,
                BufferFormat::Nv12,
                16_000_000u32,
                0u32,
            ),
            (VideoProfile::Hdr10Pq, SourcePixelFormat::ScrgbF16) => (
                CudaKernel::compile(
                    cuda,
                    SCRGB_F16_TO_P010_PQ_CU,
                    "scrgb_f16_to_p010_pq.cu",
                    "scrgb_f16_to_p010_pq",
                )?,
                Codec::Hevc,
                BufferFormat::Yuv420_10Bit,
                24_000_000u32,
                0u32,
            ),
            (VideoProfile::Hdr10Pq, SourcePixelFormat::Rgba8 | SourcePixelFormat::Bgra8) => {
                return Err(
                    "PQ10 HDR10 recording requires an HDR scRGB FP16 source (switch View to HDR or Video to SDR/Auto)"
                        .to_string(),
                );
            }
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
            stream,
            kernel,
            yuv_res,
            bitstream,
            yuv_buf,
            session,
            muxer,
            width: w,
            height: h,
            yuv_pitch_elems,
            swap_rb,
            frame_idx: 0,
        })
    }

    /// Encodes one frame into the MP4 stream with explicit `duration_secs` (real-time or fixed).
    ///
    /// Expects `src_pitch_px * height * bpp` device bytes matching the [`SourcePixelFormat`] passed
    /// to [`Self::start_with_format`] (or [`Self::start`]).
    pub fn push_frame(
        &mut self,
        src_dev_ptr: u64,
        src_pitch_px: u32,
        duration_secs: f64,
    ) -> Result<(), String> {
        if src_dev_ptr == 0 {
            return Err("video source device pointer is null".to_string());
        }
        if src_pitch_px < self.width {
            return Err(format!(
                "video source pitch ({src_pitch_px}) < width ({})",
                self.width
            ));
        }

        self.cuda
            .bind_to_thread()
            .map_err(|e| format!("CUDA bind failed: {e}"))?;

        let gx = self.width.div_ceil(16);
        let gy = self.height.div_ceil(16);
        let mut src_ptr = src_dev_ptr;
        let mut dst_ptr = self.yuv_buf.cu_deviceptr();
        let mut w = self.width;
        let mut h = self.height;
        let mut src_pitch = src_pitch_px;
        let mut dst_pitch = self.yuv_pitch_elems;
        let mut swap_rb = self.swap_rb;
        let mut args: [*mut c_void; 7] = [
            (&raw mut src_ptr).cast(),
            (&raw mut dst_ptr).cast(),
            (&raw mut w).cast(),
            (&raw mut h).cast(),
            (&raw mut src_pitch).cast(),
            (&raw mut dst_pitch).cast(),
            (&raw mut swap_rb).cast(),
        ];
        // SAFETY: All three conversion kernels (`rgba8_to_nv12`, `scrgb_f16_to_nv12`,
        // `scrgb_f16_to_p010_pq`) share the exact 7-argument ABI
        // `(const ptr*, dst ptr*, u32 w, u32 h, u32 src_pitch_px, u32 dst_pitch_elems, u32 swap_rb)`
        // with valid device buffers; `self.stream.synchronize()` completes before NVENC maps `yuv_buf`.
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
                self.stream.cu_stream(),
                args.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        };
        if launch_res != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("YUV conversion cuLaunchKernel failed: {launch_res:?}"));
        }

        self.stream
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn align_even_dims_truncates_odd_and_rejects_tiny() {
        assert_eq!(align_even_dims(1920, 1080), Some((1920, 1080)));
        assert_eq!(align_even_dims(1281, 721), Some((1280, 720)));
        assert_eq!(align_even_dims(3, 2), Some((2, 2)));
        assert_eq!(align_even_dims(1, 720), None);
        assert_eq!(align_even_dims(1280, 0), None);
    }

    #[test]
    fn copy_pitch_enforces_256_byte_wgpu_alignment() {
        // 1281 px * 4 B = 5124 B -> next multiple of 256 is 5376 B (1344 px).
        assert_eq!(copy_pitch_for_width(1281, 4), Some((5376, 1344)));
        // 1280 px * 4 B = 5120 B (already multiple of 256).
        assert_eq!(copy_pitch_for_width(1280, 4), Some((5120, 1280)));
        // 1281 px * 8 B = 10248 B -> next multiple of 256 is 10496 B (1312 px).
        assert_eq!(copy_pitch_for_width(1281, 8), Some((10496, 1312)));
        assert_eq!(copy_pitch_for_width(0, 4), None);
        assert_eq!(copy_pitch_for_width(100, 0), None);
    }

    #[test]
    fn unpack_pitched_bgra8_swizzles_channels_and_strips_padding() {
        // 2x2 image with pitch = 4 px (16 B/row)
        let mut src = vec![0u8; 4 * 2 * 4];
        // Row 0: pixel (0,0) = BGRA(10, 20, 30, 128), pixel (1,0) = BGRA(40, 50, 60, 0)
        src[0..4].copy_from_slice(&[10, 20, 30, 128]);
        src[4..8].copy_from_slice(&[40, 50, 60, 0]);
        // Row 1: pixel (0,1) = BGRA(70, 80, 90, 255), pixel (1,1) = BGRA(1, 2, 3, 4)
        src[16..20].copy_from_slice(&[70, 80, 90, 255]);
        src[20..24].copy_from_slice(&[1, 2, 3, 4]);

        let mut dst = Vec::new();
        unpack_pitched_to_rgba8(&src, &mut dst, 2, 2, 4, SourcePixelFormat::Bgra8).expect("bgra");
        assert_eq!(
            dst,
            vec![
                30, 20, 10, 255, 60, 50, 40, 255, // row 0
                90, 80, 70, 255, 3, 2, 1, 255, // row 1
            ]
        );
    }

    #[test]
    fn unpack_pitched_scrgb_f16_maps_paper_white_and_zero_accurately() {
        // IEEE-754 binary16 constants: 0.0 = 0x0000, 2.5375 (~2.5371094) = 0x4113, 0.5 = 0x3800
        let f16_zero: u16 = 0x0000;
        let f16_paper_white: u16 = 0x4113; // 2.5371094 (~203 / 80 nits)
        let f16_half_pw: u16 = 0x3d13; // ~1.26855 (0.5 linear)
        assert!((f16_bits_to_f32(f16_paper_white) - 2.537_109_4).abs() < 1e-3);

        let mut src = Vec::with_capacity(8);
        for bits in [f16_paper_white, f16_half_pw, f16_zero, 0x3c00u16] {
            src.extend_from_slice(&bits.to_le_bytes());
        }
        let mut dst = Vec::new();
        unpack_pitched_to_rgba8(&src, &mut dst, 1, 1, 1, SourcePixelFormat::ScrgbF16)
            .expect("scrgb");
        assert_eq!(dst[0], 255); // paper white -> 255 (no double tonemapping)
        assert!((184..=188).contains(&dst[1])); // 0.5^(1/2.2) * 255 ~= 186
        assert_eq!(dst[2], 0);
        assert_eq!(dst[3], 255);
    }
}
