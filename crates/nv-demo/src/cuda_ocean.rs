//! Real-time 3-cascade `256x256` spectral ocean simulator powered by `cufft` and NVRTC CUDA kernels.
//!
//! What: synthesizes a physically based JONSWAP + TMA finite-depth wave spectrum across three
//! wavenumber-partitioned spatial tiles (`L0 = 220 m` Pacific swell, `L1 = 48 m` mid wind waves,
//! `L2 = 9 m` capillary ripples), advances Hermitian-symmetric Fourier modes `h(k, t)` and their
//! slope/Jacobian derivatives on the GPU, executes a single batched 18-plane `256x256` `C2R`
//! inverse FFT via `crates/cufft` (`Plan::plan_many`), and packs the contiguous `3 * 256 * 256`
//! `float4` buffer `(height_m, slope_x, slope_z, foam)` consumed directly by OptiX `raytracer.cu`.
//!
//! Why: produces photorealistic, non-repeating ocean swells, crisp directional wind ripples, and
//! physically derived Jacobian crest whitecaps with 100% procedural geometry (zero triangle meshes),
//! showcasing `nv-rs`'s native `cufft` wrapper at `< 0.2 ms/frame` with zero steady-state allocations.
//!
//! How: `h0(k)` + `h0(-k)` are generated once on-device whenever `(wind_speed, wind_dir_deg, swell)`
//! changes (`ocean_init_h0`). Each active frame runs `ocean_evolve_spectrum` (writing 6 Hermitian
//! frequency planes per cascade: `h`, `dh/dx`, `dh/dz`, `dDx/dx`, `dDz/dz`, `dDx/dz`), runs one
//! batched `Plan::exec_c2r` (`batch = 18`), and packs the real planes into `packed_cascades` via
//! `ocean_pack_cascades` on the caller's CUDA stream with zero host synchronization.
//!
//! Uses: `crate::app_events::{OceanMode, OceanParams}`, `crate::gpu_export::load_ptx_cached`,
//! `cuda_core`, `cufft`.
//! Used by: `crate::renderer`.

use std::ffi::c_void;
use std::sync::Arc;

use cuda_core::{
    Context as CudaContext, DeviceBuffer, Function, Module, Stream, launch_kernel_on_stream,
};
use cufft::{Complex32, FftType, ManyDesc, Plan, Scratch};

use crate::app_events::{OceanMode, OceanParams};

/// Spatial resolution per cascade tile (`256 x 256` real texels).
pub const OCEAN_DIM: u32 = 256;
/// Complex row width for 2D `C2R` Hermitian frequency planes (`N / 2 + 1 = 129`).
pub const OCEAN_HALF_W: u32 = OCEAN_DIM / 2 + 1;
/// Number of tiled spectral cascades (`L0` swell, `L1` wind sea, `L2` capillary ripples).
pub const OCEAN_CASCADES: usize = 3;
/// Number of real spatial fields inverse-transformed per cascade (`h, sx, sz, jxx, jzz, jxz`).
const FIELDS_PER_CASCADE: usize = 6;
/// Total 2D `C2R` batch count (`3 * 6 = 18`).
const TOTAL_FFT_BATCH: usize = OCEAN_CASCADES * FIELDS_PER_CASCADE;
/// Spatial tile lengths in meters `[L0, L1, L2]` (shared verbatim with `raytracer.cu`).
#[cfg(test)]
const OCEAN_CASCADE_SCALES_M: [f32; OCEAN_CASCADES] = [220.0, 48.0, 9.0];
/// Maximum cuFFT workspace budget admitted by [`Scratch`] (64 MiB; actual workspace is < 2 MiB).
const OCEAN_FFT_SCRATCH_BUDGET_BYTES: usize = 64 * 1024 * 1024;

/// CUDA C++ kernels compiled once via NVRTC and cached on disk. Zero FP64 (`float` only).
const OCEAN_CU: &str = r#"
#define OCEAN_N 256u
#define OCEAN_HALF_W 129u
#define OCEAN_CASCADES 3u
#define OCEAN_FIELDS 6u
#define OCEAN_PI 3.14159265358979323846f
#define OCEAN_TWO_PI 6.28318530717958647692f
#define OCEAN_G 9.81f
#define OCEAN_DEPTH_M 28.0f

static __forceinline__ __device__ float ocean_cascade_len(unsigned int c) {
    if (c == 0u) return 220.0f;
    if (c == 1u) return 48.0f;
    return 9.0f;
}

static __forceinline__ __device__ float ocean_smoothstep(float e0, float e1, float x) {
    float t = fminf(fmaxf((x - e0) / fmaxf(e1 - e0, 1e-6f), 0.0f), 1.0f);
    return t * t * (3.0f - 2.0f * t);
}

// Deterministic PCG-style hash -> uniform (0, 1) for repeatable Gaussian Box-Muller modes.
static __forceinline__ __device__ unsigned int ocean_hash_u32(unsigned int x) {
    x ^= x >> 16u;
    x *= 0x7feb352du;
    x ^= x >> 15u;
    x *= 0x846ca68bu;
    x ^= x >> 16u;
    return x;
}

static __forceinline__ __device__ float2 ocean_gauss2(unsigned int c, int kx, int kz) {
    unsigned int sx = (unsigned int)(kx + 4096);
    unsigned int sz = (unsigned int)(kz + 4096);
    unsigned int h0 = ocean_hash_u32(sx * 1973u ^ sz * 9277u ^ (c + 1u) * 26699u ^ 0x9e3779b9u);
    unsigned int h1 = ocean_hash_u32(h0 ^ 0x85ebca6bu);
    float u1 = fmaxf(1.0e-6f, ((float)(h0 & 0x00FFFFFFu) + 0.5f) * (1.0f / 16777216.0f));
    float u2 = ((float)(h1 & 0x00FFFFFFu) + 0.5f) * (1.0f / 16777216.0f);
    float r = sqrtf(-2.0f * logf(u1));
    float phi = OCEAN_TWO_PI * u2;
    return make_float2(r * cosf(phi), r * sinf(phi));
}

// Finite-depth gravity-capillary dispersion omega(k) and group speed d(omega)/dk.
static __forceinline__ __device__ void ocean_dispersion(float k, float& omega, float& domega_dk) {
    float kd = fminf(k * OCEAN_DEPTH_M, 20.0f);
    float th = tanhf(kd);
    float sech = 1.0f / coshf(kd);
    float cap = 1.0f + (k * k) * (1.0f / (370.0f * 370.0f));
    float dcap_dk = (2.0f * k) * (1.0f / (370.0f * 370.0f));
    float w2 = fmaxf(OCEAN_G * k * cap * th, 1.0e-10f);
    omega = sqrtf(w2);
    float dw2_dk = OCEAN_G * ((cap + k * dcap_dk) * th + k * cap * OCEAN_DEPTH_M * sech * sech);
    domega_dk = 0.5f * dw2_dk / omega;
}

// Smooth wavenumber band weight partitioning spectrum across L0=220m, L1=48m, L2=9m with zero overlap duplication.
static __forceinline__ __device__ float ocean_band_weight(unsigned int c, float k) {
    const float k_cut_01 = OCEAN_TWO_PI / 24.0f; // ~0.262 rad/m (L0 owns >= 24 m swells)
    const float k_cut_12 = OCEAN_TWO_PI / 4.5f;  // ~1.396 rad/m (L1 owns 4.5..24 m wind sea, L2 owns < 4.5 m ripples)
    const float w01 = ocean_smoothstep(k_cut_01 * 0.78f, k_cut_01 * 1.28f, k);
    const float w12 = ocean_smoothstep(k_cut_12 * 0.78f, k_cut_12 * 1.28f, k);
    if (c == 0u) {
        return 1.0f - w01;
    }
    if (c == 1u) {
        return w01 * (1.0f - w12);
    }
    return w12;
}

// Evaluates JONSWAP + Toba omega^-4 equilibrium tail + TMA finite-depth directional spectral amplitude.
static __forceinline__ __device__ float ocean_spectrum_amp(
    unsigned int c,
    float kx_w,
    float kz_w,
    float wind_speed,
    float wind_dir_rad,
    float swell
) {
    float k = sqrtf(kx_w * kx_w + kz_w * kz_w);
    if (k < 1.0e-5f) return 0.0f;

    float L = ocean_cascade_len(c);
    // Preserve high-frequency FFT modes up to 84% Nyquist before rolling off checkerboard corner.
    float k_nyq = OCEAN_PI * ((float)OCEAN_N / L);
    float nyq_fade = 1.0f - ocean_smoothstep(k_nyq * 0.84f, k_nyq * 0.98f, k);
    float band = ocean_band_weight(c, k) * nyq_fade;
    if (band <= 1.0e-5f) return 0.0f;

    float omega, domega_dk;
    ocean_dispersion(k, omega, domega_dk);

    // JONSWAP parameters for 120 km Pacific coastal fetch.
    float U = fminf(fmaxf(wind_speed, 1.0f), 25.0f);
    float fetch = 120000.0f;
    float x_tilde = (OCEAN_G * fetch) / (U * U);
    float alpha = 0.076f * powf(x_tilde, -0.22f);
    float omega_p = 22.0f * (OCEAN_G / U) * powf(x_tilde, -0.33f);
    omega_p = fminf(fmaxf(omega_p, 0.42f), 2.8f);

    float ratio = omega_p / fmaxf(omega, 1.0e-4f);
    float ratio4 = ratio * ratio * ratio * ratio;
    float sigma = (omega <= omega_p) ? 0.07f : 0.09f;
    float delta = (omega - omega_p) / fmaxf(sigma * omega_p, 1.0e-4f);
    float gamma_exp = expf(-0.5f * delta * delta);
    float gamma_boost = powf(3.3f, gamma_exp);

    float pm = (alpha * OCEAN_G * OCEAN_G) / fmaxf(omega * omega * omega * omega * omega, 1.0e-8f);
    float s_jonswap = pm * expf(-1.25f * fminf(ratio4, 60.0f)) * gamma_boost;

    // Physical Toba omega^-4 equilibrium tail above 1.35 * omega_p: pure JONSWAP omega^-5 starves
    // medium wind chop (L1) and capillary wavelets (L2) of slope energy, making the sea look waxy.
    float tail_w = ocean_smoothstep(1.35f * omega_p, 2.8f * omega_p, omega);
    float u_star = 0.036f * U;
    float s_toba = (0.095f * OCEAN_G * u_star) / fmaxf(omega * omega * omega * omega, 1.0e-7f);
    float s_freq = s_jonswap * (1.0f - 0.55f * tail_w) + s_toba * tail_w;

    // TMA finite-depth attenuation Phi(omega_h).
    float omega_h = omega * sqrtf(OCEAN_DEPTH_M / OCEAN_G);
    float phi_tma = 1.0f;
    if (omega_h <= 1.0f) {
        phi_tma = 0.5f * omega_h * omega_h;
    } else if (omega_h < 2.0f) {
        float d = 2.0f - omega_h;
        phi_tma = 1.0f - 0.5f * d * d;
    }
    float s_omega = s_freq * phi_tma;

    // Directional spreading: coherent Pacific swell in L0, broader cross-chop in L1/L2 for lively facets.
    float wx = cosf(wind_dir_rad);
    float wz = sinf(wind_dir_rad);
    float cos_th = (kx_w * wx + kz_w * wz) / k;
    float half_cos = fmaxf(0.0f, 0.5f * (1.0f + cos_th));
    float s_wind = powf(half_cos, (c == 0u) ? 3.0f : 1.8f);
    float s_swell = powf(half_cos, 14.0f);
    float sw = fminf(fmaxf(swell, 0.0f), 1.0f) * ((c == 0u) ? 1.0f : ((c == 1u) ? 0.35f : 0.10f));
    float back_fill = (c == 0u) ? 0.12f : 0.24f;
    float dir_spread = ((1.0f - sw) * (back_fill + (1.0f - back_fill) * s_wind) + sw * (0.04f + 2.35f * s_swell));
    dir_spread *= (1.0f / OCEAN_PI);

    float dk = OCEAN_TWO_PI / L;
    float s_k = fmaxf(0.0f, s_omega * dir_spread * (domega_dk / k));
    // Elfouhaily-style short-wave / capillary energy lift in L1 & L2 keeps medium chop and fine wind glitter crisp.
    float detail_boost = (c == 0u)
        ? 0.85f
        : ((c == 1u)
            ? (1.55f + 0.65f * ocean_smoothstep(0.45f, 1.5f, k))
            : (2.40f + 1.80f * ocean_smoothstep(2.0f, 18.0f, k)));
    float variance = 2.0f * s_k * dk * dk * band * detail_boost;
    return 0.70710678f * sqrtf(fmaxf(variance, 0.0f));
}

// Generates h0(k) and h0(-k) for all 3 cascades across the (256 x 129) C2R half-plane.
// Each float4 stores (h0_k.re, h0_k.im, h0_neg_k.re, h0_neg_k.im).
extern "C" __global__ void ocean_init_h0(
    float4* __restrict__ h0_buf,
    float wind_speed,
    float wind_dir_rad,
    float swell
) {
    unsigned int kx_u = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int kz_u = blockIdx.y * blockDim.y + threadIdx.y;
    unsigned int c = blockIdx.z;
    if (kx_u >= OCEAN_HALF_W || kz_u >= OCEAN_N || c >= OCEAN_CASCADES) return;

    size_t plane_stride = (size_t)OCEAN_HALF_W * (size_t)OCEAN_N;
    size_t idx = (size_t)c * plane_stride + (size_t)kz_u * (size_t)OCEAN_HALF_W + (size_t)kx_u;

    int kx = (int)kx_u;
    int kz = (kz_u <= OCEAN_N / 2u) ? (int)kz_u : ((int)kz_u - (int)OCEAN_N);

    // Zero DC and Nyquist corner so mean sea level stays exactly 0 m and C2R is strictly real.
    if ((kx == 0 && kz == 0) || (kx_u == OCEAN_N / 2u && kz_u == OCEAN_N / 2u)) {
        h0_buf[idx] = make_float4(0.0f, 0.0f, 0.0f, 0.0f);
        return;
    }

    float dk = OCEAN_TWO_PI / ocean_cascade_len(c);
    float kx_w = (float)kx * dk;
    float kz_w = (float)kz * dk;

    float amp_pos = ocean_spectrum_amp(c, kx_w, kz_w, wind_speed, wind_dir_rad, swell);
    float amp_neg = ocean_spectrum_amp(c, -kx_w, -kz_w, wind_speed, wind_dir_rad, swell);
    float2 g_pos = ocean_gauss2(c, kx, kz);
    float2 g_neg = ocean_gauss2(c, -kx, -kz);

    h0_buf[idx] = make_float4(
        g_pos.x * amp_pos,
        g_pos.y * amp_pos,
        g_neg.x * amp_neg,
        g_neg.y * amp_neg
    );
}

// Evolves h(k, t) and fills 18 Hermitian frequency planes (3 cascades x 6 fields: h, sx, sz, jxx, jzz, jxz).
extern "C" __global__ void ocean_evolve_spectrum(
    const float4* __restrict__ h0_buf,
    float2* __restrict__ freq_planes,
    float time_s
) {
    unsigned int kx_u = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int kz_u = blockIdx.y * blockDim.y + threadIdx.y;
    unsigned int c = blockIdx.z;
    if (kx_u >= OCEAN_HALF_W || kz_u >= OCEAN_N || c >= OCEAN_CASCADES) return;

    size_t plane_stride = (size_t)OCEAN_HALF_W * (size_t)OCEAN_N;
    size_t cell = (size_t)kz_u * (size_t)OCEAN_HALF_W + (size_t)kx_u;
    float4 h0 = __ldg(&h0_buf[(size_t)c * plane_stride + cell]);

    int kx = (int)kx_u;
    int kz = (kz_u <= OCEAN_N / 2u) ? (int)kz_u : ((int)kz_u - (int)OCEAN_N);
    float dk = OCEAN_TWO_PI / ocean_cascade_len(c);
    float kx_w = (float)kx * dk;
    float kz_w = (float)kz * dk;
    float k = sqrtf(kx_w * kx_w + kz_w * kz_w);

    float2 h = make_float2(0.0f, 0.0f);
    float2 sx = make_float2(0.0f, 0.0f);
    float2 sz = make_float2(0.0f, 0.0f);
    float2 jxx = make_float2(0.0f, 0.0f);
    float2 jzz = make_float2(0.0f, 0.0f);
    float2 jxz = make_float2(0.0f, 0.0f);

    if (k > 1.0e-5f) {
        float omega, domega_dk;
        ocean_dispersion(k, omega, domega_dk);
        float phase = omega * time_s;
        float cp = cosf(phase);
        float sp = sinf(phase);

        // h(k, t) = h0(k) * e^{+i*w*t} + conj(h0(-k)) * e^{-i*w*t}
        // Guarantees exact Hermitian symmetry H(-k, t) = conj(H(k, t)) across the C2R planes.
        float h_re = (h0.x * cp - h0.y * sp) + (h0.z * cp - h0.w * sp);
        float h_im = (h0.x * sp + h0.y * cp) + (-h0.w * cp - h0.z * sp);
        // Self-conjugate real-axis modes (kx == 0 or kx == N/2 with kz == 0 or N/2) must be real.
        if ((kx_u == 0u || kx_u == OCEAN_N / 2u) && (kz_u == 0u || kz_u == OCEAN_N / 2u)) {
            h_im = 0.0f;
        }
        h = make_float2(h_re, h_im);

        // Slopes: i * k_axis * H
        sx = make_float2(-h_im * kx_w, h_re * kx_w);
        sz = make_float2(-h_im * kz_w, h_re * kz_w);

        // Horizontal displacement spatial derivatives: (k_a * k_b / |k|) * H
        // Positive at wave crests -> Jacobian (1 - chop*jxx)*(1 - chop*jzz) - (chop*jxz)^2 < 1 at crests.
        float inv_k = 1.0f / k;
        float fxx = (kx_w * kx_w) * inv_k;
        float fzz = (kz_w * kz_w) * inv_k;
        float fxz = (kx_w * kz_w) * inv_k;
        jxx = make_float2(h_re * fxx, h_im * fxx);
        jzz = make_float2(h_re * fzz, h_im * fzz);
        jxz = make_float2(h_re * fxz, h_im * fxz);
    }

    size_t base = (size_t)(c * OCEAN_FIELDS) * plane_stride + cell;
    freq_planes[base + 0u * plane_stride] = h;
    freq_planes[base + 1u * plane_stride] = sx;
    freq_planes[base + 2u * plane_stride] = sz;
    freq_planes[base + 3u * plane_stride] = jxx;
    freq_planes[base + 4u * plane_stride] = jzz;
    freq_planes[base + 5u * plane_stride] = jxz;
}

// Packs the 6 real spatial planes per cascade into contiguous float4(height_m, slope_x, slope_z, foam).
extern "C" __global__ void ocean_pack_cascades(
    const float* __restrict__ real_planes,
    float4* __restrict__ packed_out,
    float choppiness
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int z = blockIdx.y * blockDim.y + threadIdx.y;
    unsigned int c = blockIdx.z;
    if (x >= OCEAN_N || z >= OCEAN_N || c >= OCEAN_CASCADES) return;

    size_t plane_stride = (size_t)OCEAN_N * (size_t)OCEAN_N;
    size_t cell = (size_t)z * (size_t)OCEAN_N + (size_t)x;
    size_t base = (size_t)(c * OCEAN_FIELDS) * plane_stride + cell;

    float h   = __ldg(&real_planes[base + 0u * plane_stride]);
    float sx  = __ldg(&real_planes[base + 1u * plane_stride]);
    float sz  = __ldg(&real_planes[base + 2u * plane_stride]);
    float jxx = __ldg(&real_planes[base + 3u * plane_stride]);
    float jzz = __ldg(&real_planes[base + 4u * plane_stride]);
    float jxz = __ldg(&real_planes[base + 5u * plane_stride]);

    if (!isfinite(h)) h = 0.0f;
    if (!isfinite(sx)) sx = 0.0f;
    if (!isfinite(sz)) sz = 0.0f;
    if (!isfinite(jxx)) jxx = 0.0f;
    if (!isfinite(jzz)) jzz = 0.0f;
    if (!isfinite(jxz)) jxz = 0.0f;

    float chop = fminf(fmaxf(choppiness, 0.0f), 2.0f);
    // Cascade-weighted choppiness keeps capillary ripples from over-folding.
    float c_chop = (c == 0u) ? chop : ((c == 1u) ? (chop * 0.85f) : (chop * 0.55f));
    float dxx = 1.0f - c_chop * jxx;
    float dzz = 1.0f - c_chop * jzz;
    float dxz = -c_chop * jxz;
    float jacobian = dxx * dzz - dxz * dxz;
    // Whitecap foam onset as Jacobian pinches below 0.82 on sharp wave crests.
    float foam_raw = fminf(fmaxf((0.82f - jacobian) * 1.55f, 0.0f), 1.0f);

    // Clamp slopes softly to keep perturbed normals physically upward-facing.
    sx = fminf(fmaxf(sx, -2.5f), 2.5f);
    sz = fminf(fmaxf(sz, -2.5f), 2.5f);

    packed_out[(size_t)c * plane_stride + cell] = make_float4(h, sx, sz, foam_raw);
}
"#;

/// Sanitized spectrum cache key (quantized `wind_speed`, `wind_dir_deg`, `swell`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SpectrumKey {
    wind_speed_q: u32,
    wind_dir_q: u32,
    swell_q: u32,
}

impl SpectrumKey {
    fn from_params(p: OceanParams) -> Self {
        let ws = sanitize_ocean_f32(p.wind_speed, 9.5, 1.0, 25.0);
        let wd = sanitize_ocean_angle_deg(p.wind_dir_deg, 245.0);
        let sw = sanitize_ocean_f32(p.swell, 0.65, 0.0, 1.0);
        Self {
            wind_speed_q: (ws * 1000.0).round() as u32,
            wind_dir_q: (wd * 100.0).round() as u32,
            swell_q: (sw * 1000.0).round() as u32,
        }
    }
}

/// Sanitizes an ocean scalar to `[min_v, max_v]`, returning `fallback` for non-finite values.
#[inline]
pub(crate) fn sanitize_ocean_f32(v: f32, fallback: f32, min_v: f32, max_v: f32) -> f32 {
    if v.is_finite() {
        v.clamp(min_v, max_v)
    } else {
        fallback.clamp(min_v, max_v)
    }
}

/// Sanitizes wind direction degrees into `[0.0, 360.0)`.
#[inline]
pub(crate) fn sanitize_ocean_angle_deg(v: f32, fallback: f32) -> f32 {
    if v.is_finite() {
        v.rem_euclid(360.0)
    } else {
        fallback.rem_euclid(360.0)
    }
}

/// Packs [`OceanParams`] into the two `float4` launch uniforms `(ocean_params_a, ocean_params_b)`
/// shared by `LaunchParams` (`renderer.rs`) and `Params` (`raytracer.cu`):
/// - `ocean_params_a = [wind_speed, wind_dir_rad, choppiness, swell]`
/// - `ocean_params_b = [foam, clarity, cascade_dim_f, mode_flags]`
///   where `mode_flags = 1.0` when `CuFft` is active (`cascades_ptr != 0`), else `0.0`.
#[inline]
pub(crate) fn pack_ocean_uniforms(params: OceanParams, cascades_ptr: u64) -> ([f32; 4], [f32; 4]) {
    let wind_speed = sanitize_ocean_f32(params.wind_speed, 9.5, 1.0, 25.0);
    let wind_dir_deg = sanitize_ocean_angle_deg(params.wind_dir_deg, 245.0);
    let wind_dir_rad = wind_dir_deg.to_radians();
    let choppiness = sanitize_ocean_f32(params.choppiness, 1.15, 0.0, 2.0);
    let swell = sanitize_ocean_f32(params.swell, 0.65, 0.0, 1.0);
    let foam = sanitize_ocean_f32(params.foam, 1.0, 0.0, 2.0);
    let clarity = sanitize_ocean_f32(params.clarity, 1.0, 0.25, 3.0);
    let use_cufft = params.mode == OceanMode::CuFft && cascades_ptr != 0;
    (
        [wind_speed, wind_dir_rad, choppiness, swell],
        [
            foam,
            clarity,
            OCEAN_DIM as f32,
            if use_cufft { 1.0 } else { 0.0 },
        ],
    )
}

/// Persistent GPU 3-cascade `256x256` spectral ocean simulator (`cuFFT` `C2R` + NVRTC kernels).
pub struct CudaOcean {
    /// `3 * 256 * 129` `float4` initial spectrum pairs `(h0(k), h0(-k))`.
    h0_buf: DeviceBuffer<[f32; 4]>,
    /// `18 * 256 * 129` complex frequency planes fed into batched `C2R`.
    freq_buf: DeviceBuffer<Complex32>,
    /// `18 * 256 * 256` real spatial planes output by batched `C2R`.
    real_buf: DeviceBuffer<f32>,
    /// Contiguous `3 * 256 * 256` `float4(height_m, slope_x, slope_z, foam)` read by `raytracer.cu`.
    packed_cascades: DeviceBuffer<[f32; 4]>,
    c2r_plan: Plan,
    fn_init_h0: Function,
    fn_evolve: Function,
    fn_pack: Function,
    _module: Arc<Module>,
    _scratch: Scratch,
    _cuda: Arc<CudaContext>,
    cached_spec_key: Option<SpectrumKey>,
    last_step_sig: Option<(u32, u32)>,
}

impl CudaOcean {
    /// Compiles NVRTC ocean kernels (cached on disk), creates the batched 18-plane `256x256`
    /// `C2R` `cufft::Plan`, and pre-allocates all persistent VRAM buffers on `stream`.
    pub fn new(cuda: &Arc<CudaContext>, stream: &Arc<Stream>) -> Result<Self, String> {
        let module = crate::gpu_export::load_ptx_cached(
            OCEAN_CU,
            "cuda_ocean.cu",
            "compute_75",
            &[],
            &[],
            |ptx| {
                cuda.load_module_from_image(ptx)
                    .map_err(|e| format!("cuModuleLoadData(cuda_ocean) failed: {e}"))
            },
        )?;
        let fn_init_h0 = module
            .load_function("ocean_init_h0")
            .map_err(|e| format!("load_function ocean_init_h0 failed: {e}"))?;
        let fn_evolve = module
            .load_function("ocean_evolve_spectrum")
            .map_err(|e| format!("load_function ocean_evolve_spectrum failed: {e}"))?;
        let fn_pack = module
            .load_function("ocean_pack_cascades")
            .map_err(|e| format!("load_function ocean_pack_cascades failed: {e}"))?;

        let mut scratch = Scratch::new(stream);
        let dim = OCEAN_DIM as i32;
        let plane_freq = (OCEAN_HALF_W as usize) * (OCEAN_DIM as usize);
        let plane_real = (OCEAN_DIM as usize) * (OCEAN_DIM as usize);

        let c2r_desc = ManyDesc {
            rank: 2,
            n: [dim, dim, 0],
            inembed: None,
            istride: 1,
            idist: i32::try_from(plane_freq).map_err(|_| "ocean plane_freq exceeds i32")?,
            onembed: None,
            ostride: 1,
            odist: i32::try_from(plane_real).map_err(|_| "ocean plane_real exceeds i32")?,
            fft_type: FftType::C2R,
            batch: i32::try_from(TOTAL_FFT_BATCH).map_err(|_| "ocean batch exceeds i32")?,
        };
        let c2r_plan = Plan::plan_many(
            cuda,
            &c2r_desc,
            OCEAN_FFT_SCRATCH_BUDGET_BYTES,
            &mut scratch,
        )
        .map_err(|e| format!("cufft ocean C2R plan_many failed: {e}"))?;

        let h0_elems = OCEAN_CASCADES * plane_freq;
        let freq_elems = TOTAL_FFT_BATCH * plane_freq;
        let real_elems = TOTAL_FFT_BATCH * plane_real;
        let packed_elems = OCEAN_CASCADES * plane_real;

        // SAFETY: `h0_buf`, `freq_buf`, `real_buf`, and `packed_cascades` are fully overwritten
        // by `ocean_init_h0` / `ocean_evolve_spectrum` / `exec_c2r` / `ocean_pack_cascades` on
        // `stream` before `raytracer.cu` reads `packed_cascades`.
        let (h0_buf, freq_buf, real_buf, packed_cascades) = unsafe {
            (
                DeviceBuffer::<[f32; 4]>::uninitialized_async(stream, h0_elems)
                    .map_err(|e| format!("alloc ocean h0_buf failed: {e}"))?,
                DeviceBuffer::<Complex32>::uninitialized_async(stream, freq_elems)
                    .map_err(|e| format!("alloc ocean freq_buf failed: {e}"))?,
                DeviceBuffer::<f32>::uninitialized_async(stream, real_elems)
                    .map_err(|e| format!("alloc ocean real_buf failed: {e}"))?,
                DeviceBuffer::<[f32; 4]>::uninitialized_async(stream, packed_elems)
                    .map_err(|e| format!("alloc ocean packed_cascades failed: {e}"))?,
            )
        };

        Ok(Self {
            h0_buf,
            freq_buf,
            real_buf,
            packed_cascades,
            c2r_plan,
            fn_init_h0,
            fn_evolve,
            fn_pack,
            _module: module,
            _scratch: scratch,
            _cuda: cuda.clone(),
            cached_spec_key: None,
            last_step_sig: None,
        })
    }

    /// Advances the 3-cascade spectral ocean to `time_s` on `stream` with zero per-frame
    /// allocations and zero mid-frame host synchronization.
    ///
    /// Returns the device pointer to `packed_cascades` (`3 * 256 * 256` `float4`s), or `0` when
    /// `params.mode != OceanMode::CuFft`. Skips redundant FFT work when `(spec_key, time_s,
    /// choppiness)` is unchanged (e.g. while `Freeze Pose` accumulates multi-spp frames).
    pub fn step(
        &mut self,
        stream: &Arc<Stream>,
        time_s: f32,
        params: OceanParams,
    ) -> Result<u64, String> {
        if params.mode != OceanMode::CuFft {
            return Ok(0);
        }

        let wind_speed = sanitize_ocean_f32(params.wind_speed, 9.5, 1.0, 25.0);
        let wind_dir_rad = sanitize_ocean_angle_deg(params.wind_dir_deg, 245.0).to_radians();
        let choppiness = sanitize_ocean_f32(params.choppiness, 1.15, 0.0, 2.0);
        let swell = sanitize_ocean_f32(params.swell, 0.65, 0.0, 1.0);
        let safe_time = if time_s.is_finite() { time_s } else { 0.0 };

        let spec_key = SpectrumKey::from_params(params);
        let mut rebuilt_h0 = false;
        if self.cached_spec_key != Some(spec_key) {
            let mut h0_ptr = self.h0_buf.cu_deviceptr();
            let mut ws = wind_speed;
            let mut wd = wind_dir_rad;
            let mut sw = swell;
            let mut args: [*mut c_void; 4] = [
                (&raw mut h0_ptr).cast(),
                (&raw mut ws).cast(),
                (&raw mut wd).cast(),
                (&raw mut sw).cast(),
            ];
            let grid = (
                OCEAN_HALF_W.div_ceil(16),
                OCEAN_DIM.div_ceil(16),
                OCEAN_CASCADES as u32,
            );
            // SAFETY: `fn_init_h0` takes `(float4*, float, float, float)`; `h0_buf` holds
            // `3 * 256 * 129` `float4`s on `stream` and `args` live through the launch call.
            unsafe {
                launch_kernel_on_stream(
                    &self.fn_init_h0,
                    grid,
                    (16, 16, 1),
                    0,
                    stream,
                    &mut args,
                )
                .map_err(|e| format!("launch ocean_init_h0 failed: {e}"))?;
            }
            self.cached_spec_key = Some(spec_key);
            rebuilt_h0 = true;
        }

        let step_sig = (safe_time.to_bits(), choppiness.to_bits());
        if !rebuilt_h0 && self.last_step_sig == Some(step_sig) {
            return Ok(self.packed_cascades.cu_deviceptr());
        }

        // 1. Evolve 18 Hermitian frequency planes (3 cascades x 6 fields).
        {
            let mut h0_ptr = self.h0_buf.cu_deviceptr();
            let mut freq_ptr = self.freq_buf.cu_deviceptr();
            let mut t_arg = safe_time;
            let mut args: [*mut c_void; 3] = [
                (&raw mut h0_ptr).cast(),
                (&raw mut freq_ptr).cast(),
                (&raw mut t_arg).cast(),
            ];
            let grid = (
                OCEAN_HALF_W.div_ceil(16),
                OCEAN_DIM.div_ceil(16),
                OCEAN_CASCADES as u32,
            );
            // SAFETY: `fn_evolve` takes `(const float4*, float2*, float)`; `h0_buf` (`3*256*129`)
            // and `freq_buf` (`18*256*129`) remain live on `stream` through the frame fence.
            unsafe {
                launch_kernel_on_stream(
                    &self.fn_evolve,
                    grid,
                    (16, 16, 1),
                    0,
                    stream,
                    &mut args,
                )
                .map_err(|e| format!("launch ocean_evolve_spectrum failed: {e}"))?;
            }
        }

        // 2. Single batched 18-plane 256x256 inverse C2R cuFFT directly on `stream`.
        self.c2r_plan
            .exec_c2r(stream, &mut self.freq_buf, &mut self.real_buf)
            .map_err(|e| format!("cufft ocean exec_c2r failed: {e}"))?;

        // 3. Pack real planes into contiguous `3 * 256 * 256` `float4(height, sx, sz, foam)`.
        {
            let mut real_ptr = self.real_buf.cu_deviceptr();
            let mut out_ptr = self.packed_cascades.cu_deviceptr();
            let mut chop_arg = choppiness;
            let mut args: [*mut c_void; 3] = [
                (&raw mut real_ptr).cast(),
                (&raw mut out_ptr).cast(),
                (&raw mut chop_arg).cast(),
            ];
            let grid = (
                OCEAN_DIM.div_ceil(16),
                OCEAN_DIM.div_ceil(16),
                OCEAN_CASCADES as u32,
            );
            // SAFETY: `fn_pack` takes `(const float*, float4*, float)`; `real_buf` (`18*256*256`)
            // and `packed_cascades` (`3*256*256`) remain live on `stream` through the frame fence.
            unsafe {
                launch_kernel_on_stream(
                    &self.fn_pack,
                    grid,
                    (16, 16, 1),
                    0,
                    stream,
                    &mut args,
                )
                .map_err(|e| format!("launch ocean_pack_cascades failed: {e}"))?;
            }
        }

        self.last_step_sig = Some(step_sig);
        Ok(self.packed_cascades.cu_deviceptr())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_ocean_uniforms_sanitizes_and_encodes_mode_flag() {
        let p = OceanParams::default();
        let (a, b) = pack_ocean_uniforms(p, 0x1000);
        assert!((a[0] - 9.5).abs() < 1e-5);
        assert!((a[1] - 245.0_f32.to_radians()).abs() < 1e-5);
        assert!((a[2] - 1.15).abs() < 1e-5);
        assert!((a[3] - 0.65).abs() < 1e-5);
        assert!((b[0] - 1.0).abs() < 1e-5);
        assert!((b[1] - 1.0).abs() < 1e-5);
        assert!((b[2] - 256.0).abs() < 1e-5);
        assert_eq!(b[3], 1.0);

        // Null pointer forces analytical mode flag even when CuFft is requested.
        let (_, b_null) = pack_ocean_uniforms(p, 0);
        assert_eq!(b_null[3], 0.0);

        // Explicit Analytical mode disables CuFft flag even with non-null pointer.
        let p_ana = OceanParams {
            mode: OceanMode::Analytical,
            wind_speed: f32::NAN,
            wind_dir_deg: -45.0,
            choppiness: 9.0,
            swell: -2.0,
            foam: f32::INFINITY,
            clarity: 0.01,
        };
        let (aa, ba) = pack_ocean_uniforms(p_ana, 0x1000);
        assert!((aa[0] - 9.5).abs() < 1e-5);
        assert!((aa[1] - 315.0_f32.to_radians()).abs() < 1e-5);
        assert!((aa[2] - 2.0).abs() < 1e-5);
        assert!((aa[3] - 0.0).abs() < 1e-5);
        assert!((ba[0] - 1.0).abs() < 1e-5);
        assert!((ba[1] - 0.25).abs() < 1e-5);
        assert_eq!(ba[3], 0.0);
    }

    #[test]
    fn ocean_cascade_constants_and_spectrum_key_are_consistent() {
        assert_eq!(OCEAN_DIM, 256);
        assert_eq!(OCEAN_HALF_W, 129);
        assert_eq!(OCEAN_CASCADES, 3);
        assert_eq!(OCEAN_CASCADE_SCALES_M, [220.0, 48.0, 9.0]);

        let p0 = OceanParams::default();
        let p1 = OceanParams {
            choppiness: 1.9,
            foam: 1.8,
            clarity: 2.2,
            ..p0
        };
        assert_eq!(
            SpectrumKey::from_params(p0),
            SpectrumKey::from_params(p1),
            "choppiness/foam/clarity must not force h0 spectrum rebuild"
        );
        let p2 = OceanParams {
            wind_speed: 14.0,
            ..p0
        };
        assert_ne!(SpectrumKey::from_params(p0), SpectrumKey::from_params(p2));
    }

    #[test]
    #[ignore = "requires local NVIDIA GPU + CUDA driver + NVRTC + cuFFT"]
    fn cuda_ocean_steps_3_cascades_finite_and_nonzero() {
        let ctx = CudaContext::new(0).expect("CUDA context on device 0");
        let stream = ctx.new_stream().expect("create CUDA stream");
        let mut ocean = CudaOcean::new(&ctx, &stream).expect("init CudaOcean");
        let ptr = ocean
            .step(&stream, 1.25, OceanParams::default())
            .expect("step CudaOcean");
        assert_ne!(ptr, 0);
        stream.synchronize().expect("sync stream");

        let host = ocean
            .packed_cascades
            .to_host_vec(&stream)
            .expect("download ocean cascades");
        assert_eq!(host.len(), OCEAN_CASCADES * (OCEAN_DIM as usize) * (OCEAN_DIM as usize));
        let mut max_abs_h = 0.0_f32;
        let mut max_abs_slope = 0.0_f32;
        for v in &host {
            assert!(
                v[0].is_finite() && v[1].is_finite() && v[2].is_finite() && v[3].is_finite(),
                "non-finite cascade texel: {v:?}"
            );
            assert!(v[3] >= 0.0 && v[3] <= 1.0, "foam out of [0, 1]: {}", v[3]);
            max_abs_h = max_abs_h.max(v[0].abs());
            max_abs_slope = max_abs_slope.max(v[1].abs()).max(v[2].abs());
        }
        assert!(max_abs_h > 0.05, "expected non-trivial wave height, got {max_abs_h}");
        assert!(max_abs_slope > 0.02, "expected non-trivial wave slope, got {max_abs_slope}");
    }
}
