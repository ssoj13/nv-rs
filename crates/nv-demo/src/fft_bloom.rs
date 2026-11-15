//! Real-time 2D FFT diffraction bloom pass powered by `cufft` and NVRTC CUDA kernels.
//!
//! **What.** [`FftBloomPass`] extracts bright pixels above a luminance threshold into a
//! power-of-two padded 3-channel planar buffer, transforms them to the frequency domain with a
//! batched 2D `R2C` cuFFT plan, multiplies by a cached procedural N-blade aperture diffraction
//! PSF spectrum, transforms back with a batched 2D `C2R` cuFFT plan, and additively composites
//! the result onto the `float4` RGBA device color buffer.
//!
//! **Why.** Simulates physically inspired polygonal aperture starbursts and wide-radius halation
//! in `O(N log N)` time independent of kernel radius, with zero per-frame host or device heap
//! allocations after initial sizing.
//!
//! **Used by** `nv-demo` post-processing pipeline.

use cuda_bindings::CUevent_flags_enum_CU_EVENT_DEFAULT;
use cuda_core::{
    Context as CudaContext, DeviceBuffer, Event, Function, Module, Stream, launch_kernel_on_stream,
};
use cufft::{Complex32, FftType, ManyDesc, Plan, Scratch};
use std::ffi::c_void;
use std::sync::Arc;

/// Maximum cuFFT workspace budget admitted by [`Scratch`] (512 MiB).
const FFT_SCRATCH_BUDGET_BYTES: usize = 512 * 1024 * 1024;

/// CUDA C++ kernels compiled at runtime via NVRTC for the FFT bloom pipeline.
const FFT_BLOOM_CU: &str = r#"
#include <math.h>

extern "C" __global__ void bright_pass_extract(
    const float4* __restrict__ color,
    float* __restrict__ real_planes,
    unsigned int w,
    unsigned int h,
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
    if (x < w && y < h) {
        float4 c = color[(size_t)y * (size_t)w + (size_t)x];
        float lum = 0.2126f * c.x + 0.7152f * c.y + 0.0722f * c.z;
        // Soft-knee threshold to avoid harsh temporal flickering near cutoff.
        float knee = fmaxf(threshold * 0.25f, 1e-4f);
        float soft = lum - threshold + knee;
        soft = fminf(fmaxf(soft, 0.0f), 2.0f * knee);
        soft = (soft * soft) / (4.0f * knee);
        float contrib = fmaxf(soft, lum - threshold);
        if (contrib > 0.0f && lum > 1e-6f) {
            float scale = contrib / lum;
            r = fmaxf(c.x, 0.0f) * scale;
            g = fmaxf(c.y, 0.0f) * scale;
            b = fmaxf(c.z, 0.0f) * scale;
        }
    }

    real_planes[dst_idx] = r;
    real_planes[plane_stride + dst_idx] = g;
    real_planes[2 * plane_stride + dst_idx] = b;
}

extern "C" __global__ void generate_aperture_psf(
    float* __restrict__ psf,
    unsigned int fft_w,
    unsigned int fft_h,
    unsigned int blades
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= fft_w || y >= fft_h) return;

    // Wrap coordinates around (0, 0) so the PSF is zero-phase in the FFT domain.
    float dx = (x <= fft_w / 2) ? (float)x : (float)((int)x - (int)fft_w);
    float dy = (y <= fft_h / 2) ? (float)y : (float)((int)y - (int)fft_h);
    float r = sqrtf(dx * dx + dy * dy);

    float min_dim = (float)((fft_w < fft_h) ? fft_w : fft_h);
    float core_sigma = fmaxf(min_dim * 0.004f, 1.5f);
    float halo_sigma = fmaxf(min_dim * 0.045f, 12.0f);

    // Core + wide halation glow.
    float val = expf(-r / core_sigma) + 0.08f * expf(-r / halo_sigma);

    // Procedural N-blade aperture diffraction starbursts.
    if (blades >= 3 && r > 0.5f) {
        const float PI = 3.14159265358979323846f;
        unsigned int num_rays = ((blades & 1u) == 0u) ? blades : (2u * blades);
        float sector = (2.0f * PI) / (float)num_rays;
        float phi = atan2f(dy, dx) + PI;
        float folded = fabsf(fmodf(phi + 0.5f * sector, sector) - 0.5f * sector);
        float d_perp = r * sinf(folded);
        float spike_width = 1.15f;
        float spike_len = fmaxf(min_dim * 0.14f, 32.0f);
        float spike = expf(-(d_perp * d_perp) / (2.0f * spike_width * spike_width))
                    * expf(-r / spike_len);
        val += 0.65f * spike;
    }

    psf[(size_t)y * (size_t)fft_w + (size_t)x] = val;
}

extern "C" __global__ void freq_mul_psf(
    float2* __restrict__ freq_planes,
    const float2* __restrict__ psf_freq,
    unsigned int half_w,
    unsigned int fft_h,
    float inv_fft_elems
) {
    unsigned int kx = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int ky = blockIdx.y * blockDim.y + threadIdx.y;
    if (kx >= half_w || ky >= fft_h) return;

    // Normalize by PSF DC bin (psf_freq[0].x == sum of spatial PSF) and cuFFT C2R scale.
    float dc = fmaxf(psf_freq[0].x, 1e-12f);
    float norm = inv_fft_elems / dc;

    size_t idx = (size_t)ky * (size_t)half_w + (size_t)kx;
    size_t plane_stride = (size_t)half_w * (size_t)fft_h;
    float2 h = psf_freq[idx];
    float hr = h.x * norm;
    float hi = h.y * norm;

    for (int c = 0; c < 3; ++c) {
        size_t p_idx = (size_t)c * plane_stride + idx;
        float2 f = freq_planes[p_idx];
        freq_planes[p_idx] = make_float2(f.x * hr - f.y * hi, f.x * hi + f.y * hr);
    }
}

extern "C" __global__ void composite_bloom(
    float4* __restrict__ color,
    const float* __restrict__ real_planes,
    unsigned int w,
    unsigned int h,
    unsigned int fft_w,
    unsigned int fft_h,
    float intensity
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= w || y >= h) return;

    size_t plane_stride = (size_t)fft_w * (size_t)fft_h;
    size_t src_idx = (size_t)y * (size_t)fft_w + (size_t)x;
    size_t dst_idx = (size_t)y * (size_t)w + (size_t)x;

    float br = fmaxf(real_planes[src_idx], 0.0f);
    float bg = fmaxf(real_planes[plane_stride + src_idx], 0.0f);
    float bb = fmaxf(real_planes[2 * plane_stride + src_idx], 0.0f);

    float4 dst = color[dst_idx];
    dst.x += br * intensity;
    dst.y += bg * intensity;
    dst.z += bb * intensity;
    color[dst_idx] = dst;
}
"#;

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
    cached_blades: Option<u32>,
}

/// 2D FFT diffraction bloom post-process pass operating on `float4` RGBA device buffers.
pub struct FftBloomPass {
    cuda: Arc<CudaContext>,
    scratch: Scratch,
    _module: Arc<Module>,
    fn_bright_pass: Function,
    fn_gen_psf: Function,
    fn_freq_mul: Function,
    fn_composite: Function,
    ev_start: Event,
    ev_end: Event,
    state: Option<BloomState>,
}

impl FftBloomPass {
    /// Compiles the NVRTC bloom kernels and initializes reusable CUDA timing events.
    pub fn new(cuda: &Arc<CudaContext>) -> Result<Self, String> {
        let opts = nvrtc::CompileOptions::new()
            .arch("compute_75")
            .std("c++17")
            .fast_math();
        let mut prog = nvrtc::Program::new(FFT_BLOOM_CU, Some("fft_bloom.cu"), &[])
            .map_err(|e| format!("NVRTC program creation failed: {e}"))?;
        prog.compile(&opts)
            .map_err(|e| format!("NVRTC compile fft_bloom.cu failed: {e}"))?;
        let ptx = prog
            .ptx()
            .map_err(|e| format!("NVRTC PTX extraction failed: {e}"))?;

        let module = cuda
            .load_module_from_image(&ptx)
            .map_err(|e| format!("cuModuleLoadData failed: {e}"))?;
        let fn_bright_pass = module
            .load_function("bright_pass_extract")
            .map_err(|e| format!("load_function bright_pass_extract failed: {e}"))?;
        let fn_gen_psf = module
            .load_function("generate_aperture_psf")
            .map_err(|e| format!("load_function generate_aperture_psf failed: {e}"))?;
        let fn_freq_mul = module
            .load_function("freq_mul_psf")
            .map_err(|e| format!("load_function freq_mul_psf failed: {e}"))?;
        let fn_composite = module
            .load_function("composite_bloom")
            .map_err(|e| format!("load_function composite_bloom failed: {e}"))?;

        let ev_start = cuda
            .new_event_with_flags(Some(CUevent_flags_enum_CU_EVENT_DEFAULT))
            .map_err(|e| format!("create start event failed: {e}"))?;
        let ev_end = cuda
            .new_event_with_flags(Some(CUevent_flags_enum_CU_EVENT_DEFAULT))
            .map_err(|e| format!("create end event failed: {e}"))?;

        let scratch = Scratch::new(&cuda.default_stream());

        Ok(Self {
            cuda: cuda.clone(),
            scratch,
            _module: module,
            fn_bright_pass,
            fn_gen_psf,
            fn_freq_mul,
            fn_composite,
            ev_start,
            ev_end,
            state: None,
        })
    }

    /// Ensures cuFFT plans and device buffers are allocated for padded dimensions covering `(w, h)`.
    fn ensure_size(
        &mut self,
        stream: &Arc<Stream>,
        w: u32,
        h: u32,
    ) -> Result<&mut BloomState, String> {
        // Add a small border margin before rounding up to power-of-two to prevent wrap-around.
        let fft_w = w.saturating_add(64).next_power_of_two().max(64);
        let fft_h = h.saturating_add(64).next_power_of_two().max(64);

        let needs_rebuild = match &self.state {
            Some(s) => s.fft_w != fft_w || s.fft_h != fft_h,
            None => true,
        };

        if needs_rebuild {
            // Drop old plans and buffers before allocating new ones so peak VRAM stays bounded.
            self.state = None;

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
                cached_blades: None,
            });
        }

        self.state
            .as_mut()
            .ok_or_else(|| "bloom state missing after ensure_size".to_owned())
    }

    /// Runs the 2D FFT diffraction bloom pass in-place on `color_ptr` (`float4` RGBA, `w * h`)
    /// and returns the elapsed GPU time in milliseconds.
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
    ) -> Result<f32, String> {
        if w == 0 || h == 0 || color_ptr == 0 || intensity <= 0.0 {
            return Ok(0.0);
        }

        self.ensure_size(stream, w, h)?;
        let state = self
            .state
            .as_mut()
            .ok_or_else(|| "bloom state missing".to_owned())?;

        self.ev_start
            .record(stream)
            .map_err(|e| format!("record start event failed: {e}"))?;

        let fft_w = state.fft_w;
        let fft_h = state.fft_h;
        let half_w = state.half_w;
        let block = (16u32, 16u32, 1u32);

        // 1. Rebuild procedural aperture PSF spectrum only when size or blade count changes.
        if state.cached_blades != Some(blades) {
            let mut psf_ptr = state.psf_spatial.cu_deviceptr();
            let mut fw = fft_w;
            let mut fh = fft_h;
            let mut b = blades;
            let mut args: [*mut c_void; 4] = [
                (&raw mut psf_ptr).cast(),
                (&raw mut fw).cast(),
                (&raw mut fh).cast(),
                (&raw mut b).cast(),
            ];
            let grid = (fft_w.div_ceil(16), fft_h.div_ceil(16), 1);
            // SAFETY: `fn_gen_psf` takes `(float*, u32, u32, u32)`; `psf_spatial` has `fft_w * fft_h`
            // elements and `args` remain valid for the duration of the launch call.
            unsafe {
                launch_kernel_on_stream(&self.fn_gen_psf, grid, block, 0, stream, &mut args)
                    .map_err(|e| format!("launch generate_aperture_psf failed: {e}"))?;
            }
            state
                .psf_plan
                .exec_r2c(stream, &state.psf_spatial, &mut state.psf_freq)
                .map_err(|e| format!("psf exec_r2c failed: {e}"))?;
            state.cached_blades = Some(blades);
        }

        // 2. Extract bright-pass RGB into padded 3-plane real buffer.
        {
            let mut c_ptr = color_ptr;
            let mut real_ptr = state.real_buf.cu_deviceptr();
            let mut img_w = w;
            let mut img_h = h;
            let mut fw = fft_w;
            let mut fh = fft_h;
            let mut thresh = threshold;
            let mut args: [*mut c_void; 7] = [
                (&raw mut c_ptr).cast(),
                (&raw mut real_ptr).cast(),
                (&raw mut img_w).cast(),
                (&raw mut img_h).cast(),
                (&raw mut fw).cast(),
                (&raw mut fh).cast(),
                (&raw mut thresh).cast(),
            ];
            let grid = (fft_w.div_ceil(16), fft_h.div_ceil(16), 1);
            // SAFETY: `fn_bright_pass` takes `(const float4*, float*, u32, u32, u32, u32, f32)`;
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

        // 4. Multiply all 3 RGB spectra by the normalized PSF spectrum.
        {
            let mut freq_ptr = state.freq_buf.cu_deviceptr();
            let mut psf_freq_ptr = state.psf_freq.cu_deviceptr();
            let mut hw = half_w;
            let mut fh = fft_h;
            let mut inv_n = 1.0f32 / ((fft_w as f32) * (fft_h as f32));
            let mut args: [*mut c_void; 5] = [
                (&raw mut freq_ptr).cast(),
                (&raw mut psf_freq_ptr).cast(),
                (&raw mut hw).cast(),
                (&raw mut fh).cast(),
                (&raw mut inv_n).cast(),
            ];
            let grid = (half_w.div_ceil(16), fft_h.div_ceil(16), 1);
            // SAFETY: `fn_freq_mul` takes `(float2*, const float2*, u32, u32, f32)` and both
            // buffers are sized for `half_w * fft_h` complex elements (3 planes for `freq_buf`).
            unsafe {
                launch_kernel_on_stream(&self.fn_freq_mul, grid, block, 0, stream, &mut args)
                    .map_err(|e| format!("launch freq_mul_psf failed: {e}"))?;
            }
        }

        // 5. Inverse batched 2D C2R FFT back to spatial RGB planes.
        state
            .c2r_plan
            .exec_c2r(stream, &state.freq_buf, &mut state.real_buf)
            .map_err(|e| format!("bloom exec_c2r failed: {e}"))?;

        // 6. Additively composite bloom back into `color_ptr`.
        {
            let mut c_ptr = color_ptr;
            let mut real_ptr = state.real_buf.cu_deviceptr();
            let mut img_w = w;
            let mut img_h = h;
            let mut fw = fft_w;
            let mut fh = fft_h;
            let mut inten = intensity;
            let mut args: [*mut c_void; 7] = [
                (&raw mut c_ptr).cast(),
                (&raw mut real_ptr).cast(),
                (&raw mut img_w).cast(),
                (&raw mut img_h).cast(),
                (&raw mut fw).cast(),
                (&raw mut fh).cast(),
                (&raw mut inten).cast(),
            ];
            let grid = (w.div_ceil(16), h.div_ceil(16), 1);
            // SAFETY: `fn_composite` takes `(float4*, const float*, u32, u32, u32, u32, f32)` and
            // operates strictly within `0..w` x `0..h`.
            unsafe {
                launch_kernel_on_stream(&self.fn_composite, grid, block, 0, stream, &mut args)
                    .map_err(|e| format!("launch composite_bloom failed: {e}"))?;
            }
        }

        self.ev_end
            .record(stream)
            .map_err(|e| format!("record end event failed: {e}"))?;

        self.ev_start
            .elapsed_ms(&self.ev_end)
            .map_err(|e| format!("elapsed_ms failed: {e}"))
    }
}
