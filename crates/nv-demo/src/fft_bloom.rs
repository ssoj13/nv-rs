//! Real-time 2D FFT diffraction bloom pass powered by `cufft` and NVRTC CUDA kernels.
//!
//! **What.** [`FftBloomPass`] extracts bright pixels above a luminance threshold into a
//! half-resolution 7-smooth (`2^a 3^b 5^c 7^d`) padded 3-channel planar buffer with NaN/+Inf
//! guards, transforms them to the frequency domain with a batched 2D `R2C` cuFFT plan, multiplies
//! by a cached pre-normalized procedural N-blade aperture diffraction PSF spectrum, transforms
//! back with a batched 2D `C2R` cuFFT plan, and bilinearly composites the result onto the
//! `float4` RGBA device color buffer.
//!
//! **Why.** Simulates physically inspired polygonal aperture starbursts and wide-radius halation
//! in `O(N log N)` time independent of kernel radius, with zero circular wrap-around, 5x+ lower
//! FFT pixel count and VRAM vs full-res power-of-two, and zero per-frame host or device heap
//! allocations after initial sizing.
//!
//! **Used by** `nv-demo` post-processing pipeline.

use cuda_core::{
    Context as CudaContext, DeviceBuffer, Function, Module, Stream, launch_kernel_on_stream,
};
use cufft::{Complex32, FftType, ManyDesc, Plan, Scratch};
use std::ffi::c_void;
use std::sync::Arc;

/// Maximum cuFFT workspace budget admitted by [`Scratch`] (512 MiB).
const FFT_SCRATCH_BUDGET_BYTES: usize = 512 * 1024 * 1024;

/// CUDA C++ kernels compiled at runtime via NVRTC for the FFT bloom pipeline.
const FFT_BLOOM_CU: &str = r#"
#include <math.h>

static __forceinline__ __device__ float sanitize_radiance(float v) {
    unsigned int bits = __float_as_uint(v);
    if ((bits & 0x7F800000u) == 0x7F800000u || !isfinite(v)) {
        return 0.0f;
    }
    return fminf(fmaxf(v, 0.0f), 65504.0f);
}

extern "C" __global__ void bright_pass_extract(
    const float4* __restrict__ color,
    float* __restrict__ real_planes,
    unsigned int w,
    unsigned int h,
    unsigned int bw,
    unsigned int bh,
    unsigned int fft_w,
    unsigned int fft_h,
    float threshold
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= fft_w || y >= fft_h) return;

    size_t plane_stride = (size_t)fft_w * (size_t)fft_h;
    size_t dst_idx = (size_t)y * (size_t)fft_w + (size_t)x;

    float r = 0.0f;
    float g = 0.0f;
    float b = 0.0f;
    if (x < bw && y < bh) {
        float safe_thresh = (((__float_as_uint(threshold) & 0x7F800000u) == 0x7F800000u) || !isfinite(threshold))
            ? 1.0f
            : fmaxf(threshold, 0.0f);
        float knee = fmaxf(safe_thresh * 0.25f, 1e-4f);
        float inv_4knee = 0.25f / knee;

        // 2x2 box downsample with per-tap NaN/+Inf guard and soft-knee bright-pass extraction.
        float acc_r = 0.0f;
        float acc_g = 0.0f;
        float acc_b = 0.0f;
        float tap_count = 0.0f;
        unsigned int sx0 = x * 2u;
        unsigned int sy0 = y * 2u;
        for (unsigned int dy = 0u; dy < 2u; ++dy) {
            unsigned int sy = sy0 + dy;
            if (sy >= h) continue;
            size_t row_off = (size_t)sy * (size_t)w;
            for (unsigned int dx = 0u; dx < 2u; ++dx) {
                unsigned int sx = sx0 + dx;
                if (sx >= w) continue;
                float4 c = color[row_off + (size_t)sx];
                float cr = sanitize_radiance(c.x);
                float cg = sanitize_radiance(c.y);
                float cb = sanitize_radiance(c.z);
                float lum = 0.2126f * cr + 0.7152f * cg + 0.0722f * cb;
                float soft = lum - safe_thresh + knee;
                soft = fminf(fmaxf(soft, 0.0f), 2.0f * knee);
                soft = (soft * soft) * inv_4knee;
                float contrib = fmaxf(soft, lum - safe_thresh);
                if (contrib > 0.0f && lum > 1e-6f) {
                    float scale = contrib / lum;
                    acc_r += cr * scale;
                    acc_g += cg * scale;
                    acc_b += cb * scale;
                }
                tap_count += 1.0f;
            }
        }
        if (tap_count > 0.0f) {
            float inv_taps = 1.0f / tap_count;
            r = acc_r * inv_taps;
            g = acc_g * inv_taps;
            b = acc_b * inv_taps;
        }
    }

    real_planes[dst_idx] = r;
    real_planes[plane_stride + dst_idx] = g;
    real_planes[2 * plane_stride + dst_idx] = b;
}

extern "C" __global__ void generate_aperture_psf(
    float* __restrict__ psf,
    unsigned int bw,
    unsigned int bh,
    unsigned int fft_w,
    unsigned int fft_h,
    unsigned int blades
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= fft_w || y >= fft_h) return;

    // Wrap coordinates around (0, 0) so the PSF is zero-phase in the FFT domain.
    float dx = (x <= fft_w / 2u) ? (float)x : (float)((int)x - (int)fft_w);
    float dy = (y <= fft_h / 2u) ? (float)y : (float)((int)y - (int)fft_h);
    float r = sqrtf(dx * dx + dy * dy);

    // Zero-wrap support radius strictly bounded by the actual zero-padding margin.
    unsigned int pad_x = (fft_w > bw) ? (fft_w - bw) : 1u;
    unsigned int pad_y = (fft_h > bh) ? (fft_h - bh) : 1u;
    float r_cut = (float)((pad_x < pad_y) ? pad_x : pad_y);
    if (r >= r_cut) {
        psf[(size_t)y * (size_t)fft_w + (size_t)x] = 0.0f;
        return;
    }

    float min_dim = (float)((bw < bh) ? bw : bh);
    float core_sigma = fmaxf(min_dim * 0.004f, 0.75f);
    float halo_sigma = fmaxf(min_dim * 0.045f, 6.0f);

    // Core + wide halation glow.
    float val = expf(-r / core_sigma) + 0.08f * expf(-r / halo_sigma);

    // Procedural N-blade aperture diffraction starbursts.
    if (blades >= 3u && r > 0.35f) {
        const float PI = 3.14159265358979323846f;
        unsigned int num_rays = ((blades & 1u) == 0u) ? blades : (2u * blades);
        float sector = (2.0f * PI) / (float)num_rays;
        float phi = atan2f(dy, dx) + PI;
        float folded = fabsf(fmodf(phi + 0.5f * sector, sector) - 0.5f * sector);
        float d_perp = r * sinf(folded);
        float spike_width = 0.75f;
        float spike_len = fmaxf(min_dim * 0.14f, 16.0f);
        float spike = expf(-(d_perp * d_perp) / (2.0f * spike_width * spike_width))
                    * expf(-r / spike_len);
        val += 0.65f * spike;
    }

    // Smooth radial window to zero at r_cut so linear convolution has exact zero wrap-around.
    float fade_start = r_cut * 0.75f;
    if (r > fade_start) {
        float t = (r - fade_start) / fmaxf(r_cut - fade_start, 1e-4f);
        t = fminf(fmaxf(t, 0.0f), 1.0f);
        float win = 1.0f - t * t * (3.0f - 2.0f * t);
        val *= win;
    }

    psf[(size_t)y * (size_t)fft_w + (size_t)x] = val;
}

extern "C" __global__ void compute_psf_norm(
    const float2* __restrict__ psf_freq,
    float* __restrict__ out_norm,
    float inv_fft_elems
) {
    if (blockIdx.x == 0u && threadIdx.x == 0u) {
        float dc = fmaxf(psf_freq[0].x, 1e-12f);
        out_norm[0] = inv_fft_elems / dc;
    }
}

extern "C" __global__ void normalize_psf_spectrum(
    float2* __restrict__ psf_freq,
    const float* __restrict__ norm_ptr,
    unsigned int half_w,
    unsigned int fft_h
) {
    unsigned int kx = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int ky = blockIdx.y * blockDim.y + threadIdx.y;
    if (kx >= half_w || ky >= fft_h) return;

    float norm = __ldg(norm_ptr);
    size_t idx = (size_t)ky * (size_t)half_w + (size_t)kx;
    float2 h = psf_freq[idx];
    psf_freq[idx] = make_float2(h.x * norm, h.y * norm);
}

extern "C" __global__ void freq_mul_psf(
    float2* __restrict__ freq_planes,
    const float2* __restrict__ psf_freq,
    unsigned int half_w,
    unsigned int fft_h
) {
    unsigned int kx = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int ky = blockIdx.y * blockDim.y + threadIdx.y;
    if (kx >= half_w || ky >= fft_h) return;

    size_t idx = (size_t)ky * (size_t)half_w + (size_t)kx;
    size_t plane_stride = (size_t)half_w * (size_t)fft_h;
    float2 h = __ldg(&psf_freq[idx]);

    for (int c = 0; c < 3; ++c) {
        size_t p_idx = (size_t)c * plane_stride + idx;
        float2 f = freq_planes[p_idx];
        freq_planes[p_idx] = make_float2(f.x * h.x - f.y * h.y, f.x * h.y + f.y * h.x);
    }
}

static __forceinline__ __device__ float bilerp_plane(
    const float* __restrict__ plane,
    unsigned int fft_w,
    unsigned int x0,
    unsigned int x1,
    unsigned int y0,
    unsigned int y1,
    float tx,
    float ty
) {
    size_t r0 = (size_t)y0 * (size_t)fft_w;
    size_t r1 = (size_t)y1 * (size_t)fft_w;
    float v00 = __ldg(&plane[r0 + (size_t)x0]);
    float v10 = __ldg(&plane[r0 + (size_t)x1]);
    float v01 = __ldg(&plane[r1 + (size_t)x0]);
    float v11 = __ldg(&plane[r1 + (size_t)x1]);
    float top = v00 + (v10 - v00) * tx;
    float bot = v01 + (v11 - v01) * tx;
    return sanitize_radiance(top + (bot - top) * ty);
}

extern "C" __global__ void composite_bloom(
    float4* __restrict__ color,
    const float* __restrict__ real_planes,
    unsigned int w,
    unsigned int h,
    unsigned int bw,
    unsigned int bh,
    unsigned int fft_w,
    unsigned int fft_h,
    float intensity
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= w || y >= h) return;

    float max_fx = (bw > 1u) ? (float)(bw - 1u) : 0.0f;
    float max_fy = (bh > 1u) ? (float)(bh - 1u) : 0.0f;
    float fx = fminf(fmaxf(((float)x + 0.5f) * 0.5f - 0.5f, 0.0f), max_fx);
    float fy = fminf(fmaxf(((float)y + 0.5f) * 0.5f - 0.5f, 0.0f), max_fy);

    unsigned int x0 = (unsigned int)fx;
    unsigned int y0 = (unsigned int)fy;
    unsigned int x1 = (x0 + 1u < bw) ? (x0 + 1u) : x0;
    unsigned int y1 = (y0 + 1u < bh) ? (y0 + 1u) : y0;
    float tx = fx - (float)x0;
    float ty = fy - (float)y0;

    size_t plane_stride = (size_t)fft_w * (size_t)fft_h;
    float br = bilerp_plane(real_planes, fft_w, x0, x1, y0, y1, tx, ty);
    float bg = bilerp_plane(real_planes + plane_stride, fft_w, x0, x1, y0, y1, tx, ty);
    float bb = bilerp_plane(real_planes + 2u * plane_stride, fft_w, x0, x1, y0, y1, tx, ty);

    size_t dst_idx = (size_t)y * (size_t)w + (size_t)x;
    float4 dst = color[dst_idx];
    dst.x += br * intensity;
    dst.y += bg * intensity;
    dst.z += bb * intensity;
    color[dst_idx] = dst;
}
"#;

/// Returns the smallest even 7-smooth integer (`2^a * 3^b * 5^c * 7^d`, `a >= 1`) `>= n.max(2)`.
pub(crate) fn next_7_smooth_even(n: u32) -> u32 {
    let target = u64::from(n.max(2));
    let mut best = u64::MAX;
    let mut p2 = 2u64;
    while p2 < best {
        let mut p23 = p2;
        while p23 < best {
            let mut p235 = p23;
            while p235 < best {
                let mut v = p235;
                while v < target {
                    v = v.saturating_mul(7);
                }
                if v < best {
                    best = v;
                }
                if p235 >= target {
                    break;
                }
                p235 = p235.saturating_mul(5);
            }
            if p23 >= target {
                break;
            }
            p23 = p23.saturating_mul(3);
        }
        if p2 >= target {
            break;
        }
        p2 = p2.saturating_mul(2);
    }
    u32::try_from(best).unwrap_or(u32::MAX - 1)
}

/// Computes half-resolution working dimensions `(bw, bh)` and zero-wrap 7-smooth FFT dimensions
/// `(fft_w, fft_h)` covering an input image of size `(w, h)`.
pub(crate) fn compute_bloom_dims(w: u32, h: u32) -> (u32, u32, u32, u32) {
    let bw = w.div_ceil(2).max(1);
    let bh = h.div_ceil(2).max(1);
    let min_dim = bw.min(bh);
    let pad = ((min_dim as f32 * 0.25).ceil() as u32).max(32);
    let fft_w = next_7_smooth_even(bw.saturating_add(pad).max(64));
    let fft_h = next_7_smooth_even(bh.saturating_add(pad).max(64));
    (bw, bh, fft_w, fft_h)
}

/// Returns `true` when bloom execution should be skipped (invalid dimensions, null pointer, or
/// non-finite / non-positive intensity).
#[inline]
pub(crate) fn should_skip_bloom(w: u32, h: u32, color_ptr: u64, intensity: f32) -> bool {
    w == 0 || h == 0 || color_ptr == 0 || !intensity.is_finite() || intensity <= 0.0
}

/// Sanitizes bright-pass threshold to a finite non-negative float.
#[inline]
pub(crate) fn sanitize_threshold(threshold: f32) -> f32 {
    if threshold.is_finite() {
        threshold.max(0.0)
    } else {
        1.0
    }
}

/// Cached resolution-dependent cuFFT plans and device buffers for [`FftBloomPass`].
struct BloomState {
    fft_w: u32,
    fft_h: u32,
    half_w: u32,
    r2c_plan: Plan,
    c2r_plan: Plan,
    psf_plan: Plan,
    real_buf: DeviceBuffer<f32>,
    freq_buf: DeviceBuffer<Complex32>,
    psf_spatial: DeviceBuffer<f32>,
    psf_freq: DeviceBuffer<Complex32>,
    cached_psf_key: Option<(u32, u32, u32)>,
}

/// 2D FFT diffraction bloom post-process pass operating on `float4` RGBA device buffers.
pub struct FftBloomPass {
    cuda: Arc<CudaContext>,
    scratch: Scratch,
    _module: Arc<Module>,
    fn_bright_pass: Function,
    fn_gen_psf: Function,
    fn_compute_psf_norm: Function,
    fn_normalize_psf: Function,
    fn_freq_mul: Function,
    fn_composite: Function,
    state: Option<BloomState>,
}

impl FftBloomPass {
    /// Compiles the NVRTC bloom kernels (through the shared on-disk PTX cache) and initializes the
    /// cuFFT workspace scratch pool.
    pub fn new(cuda: &Arc<CudaContext>) -> Result<Self, String> {
        let module = crate::gpu_export::load_ptx_cached(
            FFT_BLOOM_CU,
            "fft_bloom.cu",
            "compute_75",
            &[],
            &[],
            |ptx| {
                cuda.load_module_from_image(ptx)
                    .map_err(|e| format!("cuModuleLoadData(fft_bloom) failed: {e}"))
            },
        )?;
        let fn_bright_pass = module
            .load_function("bright_pass_extract")
            .map_err(|e| format!("load_function bright_pass_extract failed: {e}"))?;
        let fn_gen_psf = module
            .load_function("generate_aperture_psf")
            .map_err(|e| format!("load_function generate_aperture_psf failed: {e}"))?;
        let fn_compute_psf_norm = module
            .load_function("compute_psf_norm")
            .map_err(|e| format!("load_function compute_psf_norm failed: {e}"))?;
        let fn_normalize_psf = module
            .load_function("normalize_psf_spectrum")
            .map_err(|e| format!("load_function normalize_psf_spectrum failed: {e}"))?;
        let fn_freq_mul = module
            .load_function("freq_mul_psf")
            .map_err(|e| format!("load_function freq_mul_psf failed: {e}"))?;
        let fn_composite = module
            .load_function("composite_bloom")
            .map_err(|e| format!("load_function composite_bloom failed: {e}"))?;

        let scratch = Scratch::new(&cuda.default_stream());

        Ok(Self {
            cuda: cuda.clone(),
            scratch,
            _module: module,
            fn_bright_pass,
            fn_gen_psf,
            fn_compute_psf_norm,
            fn_normalize_psf,
            fn_freq_mul,
            fn_composite,
            state: None,
        })
    }

    /// Ensures cuFFT plans and device buffers are allocated for padded 7-smooth dimensions
    /// `(fft_w, fft_h)`.
    fn ensure_size(
        &mut self,
        stream: &Arc<Stream>,
        fft_w: u32,
        fft_h: u32,
    ) -> Result<&mut BloomState, String> {
        let needs_rebuild = match &self.state {
            Some(s) => s.fft_w != fft_w || s.fft_h != fft_h,
            None => true,
        };

        if needs_rebuild {
            // Drop old plans and buffers before allocating new ones so peak VRAM stays bounded.
            self.state = None;
            // Rebind scratch to the execution stream so plan_many / exec_* share the same stream.
            self.scratch = Scratch::new(stream);

            let half_w = fft_w / 2 + 1;
            let plane_real = (fft_w as usize) * (fft_h as usize);
            let plane_freq = (half_w as usize) * (fft_h as usize);

            let nx = i32::try_from(fft_w).map_err(|_| "fft_w exceeds i32".to_owned())?;
            let ny = i32::try_from(fft_h).map_err(|_| "fft_h exceeds i32".to_owned())?;
            let idist_real =
                i32::try_from(plane_real).map_err(|_| "plane_real exceeds i32".to_owned())?;
            let idist_freq =
                i32::try_from(plane_freq).map_err(|_| "plane_freq exceeds i32".to_owned())?;

            let r2c_desc = ManyDesc {
                rank: 2,
                n: [ny, nx, 0],
                inembed: None,
                istride: 1,
                idist: idist_real,
                onembed: None,
                ostride: 1,
                odist: idist_freq,
                fft_type: FftType::R2C,
                batch: 3,
            };
            let c2r_desc = ManyDesc {
                rank: 2,
                n: [ny, nx, 0],
                inembed: None,
                istride: 1,
                idist: idist_freq,
                onembed: None,
                ostride: 1,
                odist: idist_real,
                fft_type: FftType::C2R,
                batch: 3,
            };

            let r2c_plan = Plan::plan_many(
                &self.cuda,
                &r2c_desc,
                FFT_SCRATCH_BUDGET_BYTES,
                &mut self.scratch,
            )
            .map_err(|e| format!("cufft R2C plan_many failed: {e}"))?;
            let c2r_plan = Plan::plan_many(
                &self.cuda,
                &c2r_desc,
                FFT_SCRATCH_BUDGET_BYTES,
                &mut self.scratch,
            )
            .map_err(|e| format!("cufft C2R plan_many failed: {e}"))?;
            let psf_plan = Plan::plan_2d(
                &self.cuda,
                ny,
                nx,
                FftType::R2C,
                FFT_SCRATCH_BUDGET_BYTES,
                &mut self.scratch,
            )
            .map_err(|e| format!("cufft PSF plan_2d failed: {e}"))?;

            // SAFETY: every element of `real_buf`, `freq_buf`, `psf_spatial`, and `psf_freq` is
            // overwritten by CUDA kernels / cuFFT before being read.
            let (real_buf, freq_buf, psf_spatial, psf_freq) = unsafe {
                (
                    DeviceBuffer::<f32>::uninitialized_async(stream, 3 * plane_real)
                        .map_err(|e| format!("alloc real_buf failed: {e}"))?,
                    DeviceBuffer::<Complex32>::uninitialized_async(stream, 3 * plane_freq)
                        .map_err(|e| format!("alloc freq_buf failed: {e}"))?,
                    DeviceBuffer::<f32>::uninitialized_async(stream, plane_real)
                        .map_err(|e| format!("alloc psf_spatial failed: {e}"))?,
                    DeviceBuffer::<Complex32>::uninitialized_async(stream, plane_freq)
                        .map_err(|e| format!("alloc psf_freq failed: {e}"))?,
                )
            };

            self.state = Some(BloomState {
                fft_w,
                fft_h,
                half_w,
                r2c_plan,
                c2r_plan,
                psf_plan,
                real_buf,
                freq_buf,
                psf_spatial,
                psf_freq,
                cached_psf_key: None,
            });
        }

        self.state
            .as_mut()
            .ok_or_else(|| "bloom state missing after ensure_size".to_owned())
    }

    /// Enqueues the half-resolution 2D FFT diffraction bloom pass in-place on `color_ptr`
    /// (`float4` RGBA, `w * h`) strictly asynchronously on `stream` (zero mid-frame host
    /// synchronization; hardware timing is recorded by [`crate::profiler::GpuPassProfiler`]).
    #[expect(
        clippy::too_many_arguments,
        reason = "public API signature matches renderer pass parameters"
    )]
    pub fn apply(
        &mut self,
        stream: &Arc<Stream>,
        color_ptr: u64,
        w: u32,
        h: u32,
        threshold: f32,
        intensity: f32,
        blades: u32,
    ) -> Result<(), String> {
        if should_skip_bloom(w, h, color_ptr, intensity) {
            return Ok(());
        }
        let threshold = sanitize_threshold(threshold);
        let (bw, bh, fft_w, fft_h) = compute_bloom_dims(w, h);

        self.ensure_size(stream, fft_w, fft_h)?;
        let state = self
            .state
            .as_mut()
            .ok_or_else(|| "bloom state missing".to_owned())?;

        let half_w = state.half_w;
        let block = (16u32, 16u32, 1u32);

        // 1. Rebuild and pre-normalize procedural aperture PSF spectrum only when size or blades change.
        let psf_key = (bw, bh, blades);
        if state.cached_psf_key != Some(psf_key) {
            let mut psf_ptr = state.psf_spatial.cu_deviceptr();
            let mut half_img_w = bw;
            let mut half_img_h = bh;
            let mut fw = fft_w;
            let mut fh = fft_h;
            let mut b = blades;
            let mut args: [*mut c_void; 6] = [
                (&raw mut psf_ptr).cast(),
                (&raw mut half_img_w).cast(),
                (&raw mut half_img_h).cast(),
                (&raw mut fw).cast(),
                (&raw mut fh).cast(),
                (&raw mut b).cast(),
            ];
            let grid = (fft_w.div_ceil(16), fft_h.div_ceil(16), 1);
            // SAFETY: `fn_gen_psf` takes `(float*, u32, u32, u32, u32, u32)`; `psf_spatial` has
            // `fft_w * fft_h` elements and `args` remain valid for the duration of the launch call.
            unsafe {
                launch_kernel_on_stream(&self.fn_gen_psf, grid, block, 0, stream, &mut args)
                    .map_err(|e| format!("launch generate_aperture_psf failed: {e}"))?;
            }
            state
                .psf_plan
                .exec_r2c(stream, &state.psf_spatial, &mut state.psf_freq)
                .map_err(|e| format!("psf exec_r2c failed: {e}"))?;

            // Pre-compute `inv_fft_elems / psf_freq[0].x` into `psf_spatial[0]` (dead after R2C)
            // and scale `psf_freq` once so `freq_mul_psf` performs zero divisions per frame.
            {
                let mut psf_freq_ptr = state.psf_freq.cu_deviceptr();
                let mut norm_out_ptr = state.psf_spatial.cu_deviceptr();
                let mut inv_n = 1.0f32 / ((fft_w as f32) * (fft_h as f32));
                let mut norm_args: [*mut c_void; 3] = [
                    (&raw mut psf_freq_ptr).cast(),
                    (&raw mut norm_out_ptr).cast(),
                    (&raw mut inv_n).cast(),
                ];
                // SAFETY: `psf_freq` holds `half_w * fft_h >= 1` complex elements and `psf_spatial`
                // holds `fft_w * fft_h >= 1` floats; strictly ordered after `psf_plan.exec_r2c`.
                unsafe {
                    launch_kernel_on_stream(
                        &self.fn_compute_psf_norm,
                        (1, 1, 1),
                        (1, 1, 1),
                        0,
                        stream,
                        &mut norm_args,
                    )
                    .map_err(|e| format!("launch compute_psf_norm failed: {e}"))?;
                }
            }
            {
                let mut psf_freq_ptr = state.psf_freq.cu_deviceptr();
                let mut norm_in_ptr = state.psf_spatial.cu_deviceptr();
                let mut hw = half_w;
                let mut fh = fft_h;
                let mut scale_args: [*mut c_void; 4] = [
                    (&raw mut psf_freq_ptr).cast(),
                    (&raw mut norm_in_ptr).cast(),
                    (&raw mut hw).cast(),
                    (&raw mut fh).cast(),
                ];
                let norm_grid = (half_w.div_ceil(16), fft_h.div_ceil(16), 1);
                // SAFETY: `psf_freq` holds `half_w * fft_h` complex elements and `norm_in_ptr`
                // points to the scalar written by `compute_psf_norm` earlier on the same stream.
                unsafe {
                    launch_kernel_on_stream(
                        &self.fn_normalize_psf,
                        norm_grid,
                        block,
                        0,
                        stream,
                        &mut scale_args,
                    )
                    .map_err(|e| format!("launch normalize_psf_spectrum failed: {e}"))?;
                }
            }
            state.cached_psf_key = Some(psf_key);
        }

        // 2. Extract 2x2 downsampled bright-pass RGB into padded 3-plane real buffer.
        {
            let mut c_ptr = color_ptr;
            let mut real_ptr = state.real_buf.cu_deviceptr();
            let mut img_w = w;
            let mut img_h = h;
            let mut half_img_w = bw;
            let mut half_img_h = bh;
            let mut fw = fft_w;
            let mut fh = fft_h;
            let mut thresh = threshold;
            let mut args: [*mut c_void; 9] = [
                (&raw mut c_ptr).cast(),
                (&raw mut real_ptr).cast(),
                (&raw mut img_w).cast(),
                (&raw mut img_h).cast(),
                (&raw mut half_img_w).cast(),
                (&raw mut half_img_h).cast(),
                (&raw mut fw).cast(),
                (&raw mut fh).cast(),
                (&raw mut thresh).cast(),
            ];
            let grid = (fft_w.div_ceil(16), fft_h.div_ceil(16), 1);
            // SAFETY: `fn_bright_pass` takes `(const float4*, float*, u32, u32, u32, u32, u32, u32, f32)`;
            // `color_ptr` holds `w * h` `float4`s and `real_buf` holds `3 * fft_w * fft_h` `f32`s.
            unsafe {
                launch_kernel_on_stream(&self.fn_bright_pass, grid, block, 0, stream, &mut args)
                    .map_err(|e| format!("launch bright_pass_extract failed: {e}"))?;
            }
        }

        // 3. Forward batched 2D R2C FFT across all 3 RGB planes.
        state
            .r2c_plan
            .exec_r2c(stream, &state.real_buf, &mut state.freq_buf)
            .map_err(|e| format!("bloom exec_r2c failed: {e}"))?;

        // 4. Multiply all 3 RGB spectra by the pre-normalized PSF spectrum.
        {
            let mut freq_ptr = state.freq_buf.cu_deviceptr();
            let mut psf_freq_ptr = state.psf_freq.cu_deviceptr();
            let mut hw = half_w;
            let mut fh = fft_h;
            let mut args: [*mut c_void; 4] = [
                (&raw mut freq_ptr).cast(),
                (&raw mut psf_freq_ptr).cast(),
                (&raw mut hw).cast(),
                (&raw mut fh).cast(),
            ];
            let grid = (half_w.div_ceil(16), fft_h.div_ceil(16), 1);
            // SAFETY: `fn_freq_mul` takes `(float2*, const float2*, u32, u32)` and both
            // buffers are sized for `half_w * fft_h` complex elements (3 planes for `freq_buf`).
            unsafe {
                launch_kernel_on_stream(&self.fn_freq_mul, grid, block, 0, stream, &mut args)
                    .map_err(|e| format!("launch freq_mul_psf failed: {e}"))?;
            }
        }

        // 5. Inverse batched 2D C2R FFT back to spatial RGB planes.
        state
            .c2r_plan
            .exec_c2r(stream, &mut state.freq_buf, &mut state.real_buf)
            .map_err(|e| format!("bloom exec_c2r failed: {e}"))?;

        // 6. Bilinearly upsample and additively composite bloom back into `color_ptr`.
        {
            let mut c_ptr = color_ptr;
            let mut real_ptr = state.real_buf.cu_deviceptr();
            let mut img_w = w;
            let mut img_h = h;
            let mut half_img_w = bw;
            let mut half_img_h = bh;
            let mut fw = fft_w;
            let mut fh = fft_h;
            let mut inten = intensity;
            let mut args: [*mut c_void; 9] = [
                (&raw mut c_ptr).cast(),
                (&raw mut real_ptr).cast(),
                (&raw mut img_w).cast(),
                (&raw mut img_h).cast(),
                (&raw mut half_img_w).cast(),
                (&raw mut half_img_h).cast(),
                (&raw mut fw).cast(),
                (&raw mut fh).cast(),
                (&raw mut inten).cast(),
            ];
            let grid = (w.div_ceil(16), h.div_ceil(16), 1);
            // SAFETY: `fn_composite` takes `(float4*, const float*, u32, u32, u32, u32, u32, u32, f32)`
            // and operates strictly within `0..w` x `0..h`, clamping bilinear taps to `0..bw` x `0..bh`.
            unsafe {
                launch_kernel_on_stream(&self.fn_composite, grid, block, 0, stream, &mut args)
                    .map_err(|e| format!("launch composite_bloom failed: {e}"))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_7_smooth(mut v: u32) -> bool {
        if v == 0 {
            return false;
        }
        for p in [2u32, 3, 5, 7] {
            while v.is_multiple_of(p) {
                v /= p;
            }
        }
        v == 1
    }

    #[test]
    fn next_7_smooth_even_produces_valid_even_7_smooth_dimensions() {
        let samples = [
            0u32, 1, 2, 3, 5, 11, 13, 63, 64, 65, 129, 540, 675, 960, 1080, 1095, 1350, 1920,
            2160, 2190, 3840,
        ];
        for &n in &samples {
            let s = next_7_smooth_even(n);
            assert!(s >= n.max(2), "next_7_smooth_even({n}) = {s} < target");
            assert!(s.is_multiple_of(2), "next_7_smooth_even({n}) = {s} must be even");
            assert!(is_7_smooth(s), "next_7_smooth_even({n}) = {s} is not 7-smooth");
            // Verify minimality: no even 7-smooth number exists strictly between n.max(2) and s.
            for cand in (n.max(2)..s).filter(|c| c.is_multiple_of(2)) {
                assert!(
                    !is_7_smooth(cand),
                    "next_7_smooth_even({n}) = {s} skipped smaller even 7-smooth {cand}"
                );
            }
        }
    }

    #[test]
    fn compute_bloom_dims_half_res_padding_and_vram_savings() {
        let (bw_1080, bh_1080, fw_1080, fh_1080) = compute_bloom_dims(1920, 1080);
        assert_eq!((bw_1080, bh_1080), (960, 540));
        assert!(is_7_smooth(fw_1080) && is_7_smooth(fh_1080));
        assert!(fw_1080 - bw_1080 >= 135, "1080p x wrap padding too small");
        assert!(fh_1080 - bh_1080 >= 135, "1080p y wrap padding too small");
        assert!(
            (fw_1080 as usize) * (fh_1080 as usize) * 4 < 2048 * 2048,
            "1080p half-res 7-smooth must use >4x fewer FFT elements than 2048^2"
        );

        let (bw_4k, bh_4k, fw_4k, fh_4k) = compute_bloom_dims(3840, 2160);
        assert_eq!((bw_4k, bh_4k), (1920, 1080));
        assert!(is_7_smooth(fw_4k) && is_7_smooth(fh_4k));
        assert!(fw_4k - bw_4k >= 270, "4K x wrap padding too small");
        assert!(fh_4k - bh_4k >= 270, "4K y wrap padding too small");
        assert!(
            (fw_4k as usize) * (fh_4k as usize) * 5 < 4096 * 4096,
            "4K half-res 7-smooth must use >5x fewer FFT elements than 4096^2"
        );

        // Odd dimensions round up cleanly without dropping border pixels.
        let (bw_odd, bh_odd, fw_odd, fh_odd) = compute_bloom_dims(127, 65);
        assert_eq!((bw_odd, bh_odd), (64, 33));
        assert!(fw_odd >= bw_odd + 32 && fh_odd >= bh_odd + 32);
    }

    #[test]
    fn should_skip_bloom_guards_nan_inf_and_non_positive() {
        assert!(should_skip_bloom(1920, 1080, 0x1000, f32::NAN));
        assert!(should_skip_bloom(1920, 1080, 0x1000, f32::INFINITY));
        assert!(should_skip_bloom(1920, 1080, 0x1000, f32::NEG_INFINITY));
        assert!(should_skip_bloom(1920, 1080, 0x1000, 0.0));
        assert!(should_skip_bloom(1920, 1080, 0x1000, -0.25));
        assert!(should_skip_bloom(0, 1080, 0x1000, 0.5));
        assert!(should_skip_bloom(1920, 0, 0x1000, 0.5));
        assert!(should_skip_bloom(1920, 1080, 0, 0.5));
        assert!(!should_skip_bloom(1920, 1080, 0x1000, 0.25));
    }

    #[test]
    fn sanitize_threshold_handles_non_finite_and_negative() {
        assert_eq!(sanitize_threshold(f32::NAN), 1.0);
        assert_eq!(sanitize_threshold(f32::INFINITY), 1.0);
        assert_eq!(sanitize_threshold(f32::NEG_INFINITY), 1.0);
        assert_eq!(sanitize_threshold(-2.5), 0.0);
        assert_eq!(sanitize_threshold(1.25), 1.25);
    }
}
