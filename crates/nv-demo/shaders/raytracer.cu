#include <optix.h>

struct Params {
    unsigned long long out_color_f32;
    unsigned long long out_albedo_f32;
    unsigned long long out_normal_f32;
    unsigned long long handle;
    unsigned int width;
    unsigned int height;
    unsigned int frame_index;
    unsigned int spp;
    float cam_pos[4];     // xyz, aperture in .w
    float cam_forward[4]; // xyz, pad
    float cam_right[4];   // xyz, focus_dist in .w
    float cam_up[4];      // xyz, tan_half_fov in .w
    float sun_dir[4];     // xyz, roughness in .w
    float fractal_p0[4];  // [power/scale, bailout, fold_limit/julia_mode, min_radius]
    float fractal_p1[4];  // [fixed_radius/rot1, rot2, max_iter, 0]
    float fractal_p2[4];  // [c.x, c.y, c.z, 0] (julia_c or kifs offset)
    float fractal_p3[4];  // formula-specific extra coefficients
    float fractal_xform[4]; // [obj_scale, rot_x_rad, rot_y_rad, rot_z_rad]
    float fractal_r0[4];  // [detail_eps, fudge, normal_eps, march_steps]
    float fractal_r1[4];  // [metallic, trap_scale, palette_shift, sun_intensity]
    float time;
    unsigned int enable_shadows;
    unsigned int fractal_kind; // 0=Mandelbulb, 1=Mandelbox, 2=KIFS Octahedron
    unsigned int bounces;
    float floor_params[4];     // [roughness, scratch, scale, metallic]
    float sky_params[4];       // [sky_intensity, sun_size, horizon_haze, pad]
    unsigned int accum_count;
    unsigned int _pad0[3];
    unsigned long long out_motion_f32;
    unsigned long long out_depth_f32;
    float prev_cam_pos[4];
    float prev_cam_forward[4];
    float prev_cam_right[4];
    float prev_cam_up[4];
    float sat_pos[4];      // [x, y, z, radius/enable]
    float glass_params[4]; // [ior, absorption, roughness, enable]
    unsigned int aov_mode; // 0=Beauty, 1=Albedo, 2=Normal, 3=Depth, 4=Motion
    float jitter_x;
    float jitter_y;
    unsigned int _pad1;
};

extern "C" {
__constant__ Params params;
}

static __forceinline__ __device__ unsigned int pcg_hash(unsigned int v) {
    unsigned int state = v * 747796405u + 2891336453u;
    unsigned int word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

static __forceinline__ __device__ float rand01(unsigned int& seed) {
    seed = pcg_hash(seed);
    return (float)(seed & 0x00FFFFFFu) * (1.0f / 16777216.0f);
}

static __forceinline__ __device__ float3 make_f3(const float a[4]) {
    return make_float3(a[0], a[1], a[2]);
}
static __forceinline__ __device__ float3 f3_add(float3 a, float3 b) {
    return make_float3(a.x + b.x, a.y + b.y, a.z + b.z);
}
static __forceinline__ __device__ float3 f3_sub(float3 a, float3 b) {
    return make_float3(a.x - b.x, a.y - b.y, a.z - b.z);
}
static __forceinline__ __device__ float3 f3_mul(float3 a, float s) {
    return make_float3(a.x * s, a.y * s, a.z * s);
}
static __forceinline__ __device__ float3 f3_mul3(float3 a, float3 b) {
    return make_float3(a.x * b.x, a.y * b.y, a.z * b.z);
}
static __forceinline__ __device__ float f3_dot(float3 a, float3 b) {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}
static __forceinline__ __device__ float3 f3_cross(float3 a, float3 b) {
    return make_float3(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x);
}
static __forceinline__ __device__ float f3_len(float3 a) {
    return sqrtf(f3_dot(a, a));
}
static __forceinline__ __device__ float3 f3_norm(float3 a) {
    float len = f3_len(a);
    return (len > 1e-8f) ? f3_mul(a, 1.0f / len) : make_float3(0.0f, 1.0f, 0.0f);
}
static __forceinline__ __device__ float3 f3_reflect(float3 v, float3 n) {
    return f3_sub(v, f3_mul(n, 2.0f * f3_dot(v, n)));
}

// --- 3D Fractal SDFs (adapted from warpbro-rs formulas) ---

static __forceinline__ __device__ float3 rot_y(float3 p, float a) {
    float c = cosf(a);
    float s = sinf(a);
    return make_float3(c * p.x + s * p.z, p.y, -s * p.x + c * p.z);
}
static __forceinline__ __device__ float3 rot_x(float3 p, float a) {
    float c = cosf(a);
    float s = sinf(a);
    return make_float3(p.x, c * p.y - s * p.z, s * p.y + c * p.z);
}
static __forceinline__ __device__ float3 rot_z(float3 p, float a) {
    float c = cosf(a);
    float s = sinf(a);
    return make_float3(c * p.x - s * p.y, s * p.x + c * p.y, p.z);
}

static __forceinline__ __device__ float2 sdf_mandelbulb(float3 q) {
    float power = params.fractal_p0[0];
    float bailout = fmaxf(2.0f, params.fractal_p0[1]);
    bool julia = params.fractal_p0[2] > 0.5f;
    int max_iter = (int)fmaxf(2.0f, fminf(28.0f, params.fractal_p1[2]));
    float3 c_add = julia ? make_f3(params.fractal_p2) : q;
    float theta_mult = params.fractal_p3[0];
    float phi_mult = params.fractal_p3[1];
    float theta_phase = params.fractal_p3[2];
    float phi_phase = params.fractal_p3[3];

    float3 z = q;
    float dr = 1.0f;
    float r = f3_len(z);
    float trap = 1e6f;
    for (int i = 0; i < 28; ++i) {
        if (i >= max_iter || r > bailout) break;
        if (r < 1e-6f) return make_float2(0.0f, trap);
        float theta = acosf(fmaxf(-1.0f, fminf(1.0f, z.z / r))) * theta_mult + theta_phase;
        float phi = atan2f(z.y, z.x) * phi_mult + phi_phase;
        dr = powf(r, power - 1.0f) * power * dr + 1.0f;
        float zr = powf(r, power);
        theta *= power;
        phi *= power;
        float sin_t = sinf(theta);
        z = f3_add(make_float3(zr * sin_t * cosf(phi), zr * sin_t * sinf(phi), zr * cosf(theta)), c_add);
        r = f3_len(z);
        trap = fminf(trap, r);
    }
    return make_float2(0.5f * logf(fmaxf(r, 1e-6f)) * r / fmaxf(dr, 1e-6f), trap);
}

static __forceinline__ __device__ float2 sdf_mandelbox(float3 q) {
    float scale = params.fractal_p0[0];
    float bailout2 = params.fractal_p0[1] * params.fractal_p0[1];
    float fold = fmaxf(0.1f, params.fractal_p0[2]);
    float min_r = fmaxf(0.05f, params.fractal_p0[3]);
    float fixed_r = fmaxf(min_r + 0.05f, params.fractal_p1[0]);
    int max_iter = (int)fmaxf(2.0f, fminf(28.0f, params.fractal_p1[2]));
    float fold_mult = params.fractal_p3[0];
    float iter_rot = params.fractal_p3[1];
    float min_r2 = min_r * min_r;
    float fixed_r2 = fixed_r * fixed_r;

    float3 z = q;
    float dr = 1.0f;
    float trap = 1e6f;
    for (int i = 0; i < 28; ++i) {
        if (i >= max_iter) break;
        if (fabsf(iter_rot) > 1e-5f) {
            z = rot_y(z, iter_rot);
        }
        // Box fold
        z.x = fmaxf(-fold, fminf(fold, z.x)) * fold_mult - z.x;
        z.y = fmaxf(-fold, fminf(fold, z.y)) * fold_mult - z.y;
        z.z = fmaxf(-fold, fminf(fold, z.z)) * fold_mult - z.z;
        // Sphere fold
        float r2 = f3_dot(z, z);
        if (r2 < min_r2) {
            float k = fixed_r2 / min_r2;
            z = f3_mul(z, k);
            dr *= k;
        } else if (r2 < fixed_r2) {
            float k = fixed_r2 / r2;
            z = f3_mul(z, k);
            dr *= k;
        }
        z = f3_add(f3_mul(z, scale), q);
        dr = dr * fabsf(scale) + 1.0f;
        trap = fminf(trap, r2);
        if (f3_dot(z, z) > bailout2) break;
    }
    return make_float2((f3_len(z) - 1.0f) / fmaxf(fabsf(dr), 1e-6f), sqrtf(fmaxf(trap, 0.0f)));
}

static __forceinline__ __device__ float2 sdf_kifs_octahedron(float3 q) {
    float sc = fmaxf(1.2f, fabsf(params.fractal_p0[0]));
    float bailout2 = params.fractal_p0[1] * params.fractal_p0[1];
    float a1 = params.fractal_p1[0];
    float a2 = params.fractal_p1[1];
    int max_iter = (int)fmaxf(2.0f, fminf(28.0f, params.fractal_p1[2]));
    float3 off = make_f3(params.fractal_p2);
    float thickness = fmaxf(0.005f, params.fractal_p3[0]);
    float z_bias = params.fractal_p3[1];

    float3 z = q;
    float s = 1.0f;
    float trap = 1e6f;
    for (int i = 0; i < 28; ++i) {
        if (i >= max_iter) break;
        z = rot_y(z, a1);
        z.x = fabsf(z.x);
        z.y = fabsf(z.y);
        z.z = fabsf(z.z);
        if (z.x - z.y < 0.0f) { float t = z.x; z.x = z.y; z.y = t; }
        if (z.x - z.z < 0.0f) { float t = z.x; z.x = z.z; z.z = t; }
        if (z.y - z.z < 0.0f) { float t = z.y; z.y = z.z; z.z = t; }
        z = rot_x(z, a2);
        z.x = z.x * sc - off.x * (sc - 1.0f);
        z.y = z.y * sc - off.y * (sc - 1.0f);
        z.z = z.z * sc;
        if (z.z > z_bias * off.z * (sc - 1.0f)) {
            z.z -= off.z * (sc - 1.0f);
        }
        s *= sc;
        float r2 = f3_dot(z, z);
        trap = fminf(trap, r2);
        if (r2 > bailout2) break;
    }
    return make_float2((f3_len(z) - thickness) / s, sqrtf(fmaxf(trap, 0.0f)));
}

static __forceinline__ __device__ float2 eval_fractal(float3 p) {
    float obj_scale = fmaxf(0.15f, params.fractal_xform[0]);
    float3 q = rot_z(rot_y(rot_x(p, params.fractal_xform[1]), params.fractal_xform[2]), params.fractal_xform[3]);
    q = f3_mul(q, 1.0f / obj_scale);
    float2 res;
    if (params.fractal_kind == 1u) {
        float2 r = sdf_mandelbox(f3_mul(q, 2.2f));
        res = make_float2(r.x * (1.0f / 2.2f), r.y);
    } else if (params.fractal_kind == 2u) {
        res = sdf_kifs_octahedron(q);
    } else {
        res = sdf_mandelbulb(q);
    }
    return make_float2(res.x * obj_scale, res.y);
}

static __forceinline__ __device__ float3 fractal_normal(float3 p, float eps) {
    float e = fmaxf(eps, 0.00002f);
    float d0 = eval_fractal(make_float3(p.x + e, p.y, p.z)).x - eval_fractal(make_float3(p.x - e, p.y, p.z)).x;
    float d1 = eval_fractal(make_float3(p.x, p.y + e, p.z)).x - eval_fractal(make_float3(p.x, p.y - e, p.z)).x;
    float d2 = eval_fractal(make_float3(p.x, p.y, p.z + e)).x - eval_fractal(make_float3(p.x, p.y, p.z - e)).x;
    return f3_norm(make_float3(d0, d1, d2));
}

static __forceinline__ __device__ float3 trap_palette(float trap) {
    float t = fmaxf(0.0f, fminf(1.0f, trap * params.fractal_r1[1])) + params.fractal_r1[2];
    // Cosine palette (deep teal -> NVIDIA lime -> warm gold)
    float3 a = make_float3(0.45f, 0.52f, 0.42f);
    float3 b = make_float3(0.42f, 0.40f, 0.38f);
    float3 c = make_float3(1.0f, 1.0f, 1.0f);
    float3 d = make_float3(0.05f, 0.25f, 0.55f);
    return make_float3(
        fmaxf(0.05f, fminf(0.98f, a.x + b.x * cosf(6.28318f * (c.x * t + d.x)))),
        fmaxf(0.05f, fminf(0.98f, a.y + b.y * cosf(6.28318f * (c.y * t + d.y)))),
        fmaxf(0.05f, fminf(0.98f, a.z + b.z * cosf(6.28318f * (c.z * t + d.z))))
    );
}

static __forceinline__ __device__ float3 sample_cosine_hemisphere(float3 n, unsigned int& seed) {
    float r1 = rand01(seed);
    float r2 = rand01(seed);
    float phi = 6.28318530718f * r1;
    float cos_theta = sqrtf(r2);
    float sin_theta = sqrtf(fmaxf(0.0f, 1.0f - r2));

    float3 up = (fabsf(n.y) < 0.99f) ? make_float3(0.0f, 1.0f, 0.0f) : make_float3(1.0f, 0.0f, 0.0f);
    float3 tangent = f3_norm(f3_cross(up, n));
    float3 bitangent = f3_cross(n, tangent);

    return f3_norm(f3_add(
        f3_add(f3_mul(tangent, cosf(phi) * sin_theta), f3_mul(bitangent, sinf(phi) * sin_theta)),
        f3_mul(n, cos_theta)
    ));
}

static __forceinline__ __device__ float hash21(float x, float y) {
    float h = sinf(x * 127.1f + y * 311.7f) * 43758.5453123f;
    return h - floorf(h);
}

static __forceinline__ __device__ float value_noise2(float x, float y) {
    float ix = floorf(x);
    float iy = floorf(y);
    float fx = x - ix;
    float fy = y - iy;
    float ux = fx * fx * (3.0f - 2.0f * fx);
    float uy = fy * fy * (3.0f - 2.0f * fy);
    float a = hash21(ix, iy);
    float b = hash21(ix + 1.0f, iy);
    float c = hash21(ix, iy + 1.0f);
    float d = hash21(ix + 1.0f, iy + 1.0f);
    return (a + (b - a) * ux) + (c - a) * uy + (a - b - c + d) * ux * uy;
}

static __forceinline__ __device__ float fbm2(float x, float y) {
    float v = 0.0f;
    float amp = 0.5f;
    for (int i = 0; i < 4; ++i) {
        v += amp * value_noise2(x, y);
        float nx = 1.6f * x - 1.2f * y;
        float ny = 1.2f * x + 1.6f * y;
        x = nx;
        y = ny;
        amp *= 0.5f;
    }
    return v;
}

static __forceinline__ __device__ float scratch_layer(float x, float y, float ca, float sa, float freq, float warp) {
    float rx = x * ca - y * sa;
    float ry = x * sa + y * ca;
    float w = value_noise2(rx * 0.65f, ry * 0.65f) * warp;
    float cell = floorf((ry + w) * freq);
    float mask = hash21(cell, freq * 13.7f);
    if (mask < 0.55f) return 0.0f;
    float dist = fabsf(sinf((ry + w) * freq * 3.14159265f));
    float streak = powf(fmaxf(0.0f, 1.0f - dist * 14.0f), 3.0f);
    float seg = value_noise2(rx * 0.45f + cell * 1.7f, cell * 3.1f);
    return streak * fmaxf(0.0f, (seg - 0.35f) * 1.54f);
}

static __forceinline__ __device__ float floor_height(float x, float z, float scale, float scratch_amt) {
    float sx = x * scale;
    float sz = z * scale;
    float macro = fbm2(sx * 0.55f, sz * 0.55f) - 0.5f;
    float micro = value_noise2(sx * 6.5f, sz * 6.5f) - 0.5f;
    float s0 = scratch_layer(sx, sz, 0.9659f, 0.2588f, 3.8f, 0.45f);
    float s1 = scratch_layer(sx, sz, 0.5000f, -0.8660f, 5.4f, 0.35f);
    float s2 = scratch_layer(sx, sz, -0.7071f, 0.7071f, 7.2f, 0.28f);
    float scratches = fminf(1.0f, s0 * 0.9f + s1 * 0.75f + s2 * 0.6f);
    return macro * 0.55f + micro * 0.14f - scratches * 0.42f * scratch_amt;
}

static __forceinline__ __device__ float3 sky_color(float3 dir, float3 sun_dir, bool is_camera_ray) {
    float sky_i = fmaxf(0.05f, params.sky_params[0]);
    float haze_k = fmaxf(0.5f, params.sky_params[2]);
    float y = fmaxf(0.0f, dir.y);
    float horizon_band = expf(-fabsf(dir.y) * (6.0f / haze_k));

    // Balanced Rayleigh zenith-to-horizon gradient
    float3 zenith  = make_float3(0.08f, 0.21f, 0.52f);
    float3 horizon = make_float3(0.46f, 0.59f, 0.76f);
    float3 col = f3_add(f3_mul(horizon, 1.0f - powf(y, 0.55f)), f3_mul(zenith, powf(y, 0.55f)));

    // Warm horizon haze
    float3 haze_col = make_float3(0.72f, 0.68f, 0.62f);
    col = f3_add(col, f3_mul(haze_col, horizon_band * 0.22f));

    // Below-horizon ground bounce
    if (dir.y < 0.0f) {
        float g = fminf(1.0f, -dir.y * 4.0f);
        float3 ground_tint = make_float3(0.11f, 0.12f, 0.14f);
        col = f3_add(f3_mul(col, 1.0f - g), f3_mul(ground_tint, g));
    }

    // Mie forward scattering + bounded sun disc (prevents fireflies & bounce blowup)
    float sun_dot = fmaxf(0.0f, f3_dot(dir, sun_dir));
    float mie = powf(sun_dot, 8.0f) * 0.16f + powf(sun_dot, 64.0f) * 0.45f;
    float disc_t = fmaxf(0.0f, fminf(1.0f, (sun_dot - 0.9990f) / 0.0006f));
    float sun_disc = disc_t * disc_t * (3.0f - 2.0f * disc_t) * (is_camera_ray ? 3.5f : 1.1f);
    float3 sun_tint = make_float3(1.0f, 0.90f, 0.74f);
    col = f3_add(col, f3_mul(sun_tint, mie + sun_disc));
    return f3_mul(col, sky_i);
}

static __forceinline__ __device__ void trace_primary(
    float3 origin, float3 dir, unsigned int& p0, unsigned int& p1, unsigned int& p2, unsigned int& p3,
    unsigned int& p4, unsigned int& p5, unsigned int& p6, unsigned int& p7
) {
    optixTrace(
        (OptixTraversableHandle)params.handle,
        origin, dir,
        0.001f, 1e16f, 0.0f,
        OptixVisibilityMask(255),
        OPTIX_RAY_FLAG_NONE,
        0, 1, 0,
        p0, p1, p2, p3, p4, p5, p6, p7
    );
}

static __forceinline__ __device__ bool trace_shadow(float3 origin, float3 dir, float max_t) {
    unsigned int hit = 1u;
    optixTrace(
        (OptixTraversableHandle)params.handle,
        origin, dir,
        0.002f, max_t, 0.0f,
        OptixVisibilityMask(255),
        OPTIX_RAY_FLAG_TERMINATE_ON_FIRST_HIT | OPTIX_RAY_FLAG_DISABLE_CLOSESTHIT,
        0, 1, 1,
        hit
    );
    return hit != 0u;
}

extern "C" __global__ void __raygen__render() {
    const uint3 idx = optixGetLaunchIndex();
    if (idx.x >= params.width || idx.y >= params.height) return;

    unsigned int seed = pcg_hash(idx.x + idx.y * 8192u + params.frame_index * 16777619u);

    float3 cam_pos = make_f3(params.cam_pos);
    float3 cam_fwd = make_f3(params.cam_forward);
    float3 cam_rgt = make_f3(params.cam_right);
    float3 cam_up  = make_f3(params.cam_up);
    float3 sun_dir = f3_norm(make_f3(params.sun_dir));
    float aspect = (float)params.width / (float)params.height;
    float tan_fov = params.cam_up[3];
    unsigned int spp = (params.spp > 0u) ? params.spp : 1u;

    float3 accum_col = make_float3(0.0f, 0.0f, 0.0f);
    float3 first_albedo = make_float3(0.0f, 0.0f, 0.0f);
    float3 first_normal = make_float3(0.0f, 0.0f, 1.0f);
    float first_depth = 1e6f;
    float2 first_motion = make_float2(0.0f, 0.0f);

    float3 prev_pos = make_f3(params.prev_cam_pos);
    float3 prev_fwd = make_f3(params.prev_cam_forward);
    float3 prev_rgt = make_f3(params.prev_cam_right);
    float3 prev_up  = make_f3(params.prev_cam_up);
    float prev_tan_fov = params.prev_cam_up[3];
    bool has_prev_cam = (prev_tan_fov > 1e-5f);

    for (unsigned int s = 0; s < spp; ++s) {
        bool use_dlss_jitter = (s == 0u) && (params.jitter_x != 0.0f || params.jitter_y != 0.0f);
        float jx = use_dlss_jitter ? params.jitter_x : (rand01(seed) - 0.5f + params.jitter_x);
        float jy = use_dlss_jitter ? params.jitter_y : (rand01(seed) - 0.5f + params.jitter_y);
        float u = ((float)idx.x + 0.5f + jx) / (float)params.width * 2.0f - 1.0f;
        float v = 1.0f - ((float)idx.y + 0.5f + jy) / (float)params.height * 2.0f;

        float3 dir = f3_norm(f3_add(
            cam_fwd,
            f3_add(f3_mul(cam_rgt, u * aspect * tan_fov), f3_mul(cam_up, v * tan_fov))
        ));
        float3 origin = cam_pos;
        float aperture = params.cam_pos[3];
        float focus_dist = params.cam_right[3];
        if (aperture > 1e-5f && focus_dist > 0.05f) {
            float r = sqrtf(rand01(seed)) * aperture;
            float phi = 6.28318530718f * rand01(seed);
            float3 lens_off = f3_add(
                f3_mul(cam_rgt, r * cosf(phi)),
                f3_mul(cam_up, r * sinf(phi))
            );
            float3 focus_pt = f3_add(cam_pos, f3_mul(dir, focus_dist));
            origin = f3_add(cam_pos, lens_off);
            dir = f3_norm(f3_sub(focus_pt, origin));
        }

        float3 throughput = make_float3(1.0f, 1.0f, 1.0f);
        float3 sample_col = make_float3(0.0f, 0.0f, 0.0f);

        int max_bounces = (int)params.bounces;
        for (int bounce = 0; bounce < 8; ++bounce) {
            if (bounce >= max_bounces) break;
            unsigned int p0 = 0, p1 = 0, p2 = 0, p3 = 0, p4 = 0, p5 = 0, p6 = 0, p7 = 0;
            trace_primary(origin, dir, p0, p1, p2, p3, p4, p5, p6, p7);

            if (p0 == 0u) {
                float3 sky = sky_color(dir, sun_dir, bounce == 0);
                sample_col = f3_add(sample_col, f3_mul3(throughput, sky));
                if (bounce == 0 && s == 0) {
                    first_albedo = sky;
                    first_normal = make_float3(-dir.x, -dir.y, -dir.z);
                    first_depth = 1e6f;
                    if (has_prev_cam) {
                        float pz = f3_dot(dir, prev_fwd);
                        if (pz > 1e-4f) {
                            float px_ndc = f3_dot(dir, prev_rgt) / (pz * aspect * prev_tan_fov);
                            float py_ndc = f3_dot(dir, prev_up)  / (pz * prev_tan_fov);
                            float prev_x = (px_ndc * 0.5f + 0.5f) * (float)params.width;
                            float prev_y = (0.5f - py_ndc * 0.5f) * (float)params.height;
                            first_motion = make_float2(prev_x - ((float)idx.x + 0.5f), prev_y - ((float)idx.y + 0.5f));
                        }
                    }
                }
                break;
            }

            float hit_t = __uint_as_float(p1);
            float3 geom_n = make_float3(__uint_as_float(p2), __uint_as_float(p3), __uint_as_float(p4));
            bool entering = f3_dot(geom_n, dir) < 0.0f;
            float3 n = entering ? geom_n : f3_mul(geom_n, -1.0f);
            float3 base_col = make_float3(
                (float)(p5 & 0xFFu) / 255.0f,
                (float)((p5 >> 8u) & 0xFFu) / 255.0f,
                (float)((p5 >> 16u) & 0xFFu) / 255.0f
            );
            unsigned int inst_id = p6 & 0xFFu;
            float3 hit_pos = f3_add(origin, f3_mul(dir, hit_t));

            if (bounce == 0 && s == 0) {
                first_depth = hit_t;
                if (has_prev_cam) {
                    float3 rel = f3_sub(hit_pos, prev_pos);
                    float pz = f3_dot(rel, prev_fwd);
                    if (pz > 1e-4f) {
                        float px_ndc = f3_dot(rel, prev_rgt) / (pz * aspect * prev_tan_fov);
                        float py_ndc = f3_dot(rel, prev_up)  / (pz * prev_tan_fov);
                        float prev_x = (px_ndc * 0.5f + 0.5f) * (float)params.width;
                        float prev_y = (0.5f - py_ndc * 0.5f) * (float)params.height;
                        first_motion = make_float2(prev_x - ((float)idx.x + 0.5f), prev_y - ((float)idx.y + 0.5f));
                    }
                }
            }

            if (inst_id == 2u) {
                // Dielectric glass sphere: Beer-Lambert absorption + Schlick Fresnel + refraction/reflection
                float ior = fmaxf(1.05f, params.glass_params[0]);
                float absorb = fmaxf(0.0f, params.glass_params[1]);
                float g_rough = fmaxf(0.0f, fminf(0.5f, params.glass_params[2]));
                if (!entering && absorb > 0.0f) {
                    float hue = (float)((p6 >> 8u) & 0xFFu) * (1.0f / 255.0f);
                    float h6 = fmodf(hue * 6.0f, 6.0f);
                    float r = fminf(1.0f, fmaxf(0.0f, fabsf(h6 - 3.0f) - 1.0f));
                    float g = fminf(1.0f, fmaxf(0.0f, 2.0f - fabsf(h6 - 2.0f)));
                    float b = fminf(1.0f, fmaxf(0.0f, 2.0f - fabsf(h6 - 4.0f)));
                    float3 sigma = make_float3(
                        (1.05f - r) * absorb,
                        (1.05f - g) * absorb,
                        (1.05f - b) * absorb
                    );
                    throughput = f3_mul3(throughput, make_float3(
                        expf(-sigma.x * hit_t),
                        expf(-sigma.y * hit_t),
                        expf(-sigma.z * hit_t)
                    ));
                }
                if (bounce == 0 && s == 0) {
                    first_albedo = make_float3(0.92f, 0.97f, 1.0f);
                    first_normal = n;
                }
                float eta = entering ? (1.0f / ior) : ior;
                float cos_i = fminf(1.0f, fmaxf(0.0f, -f3_dot(dir, n)));
                float r0 = (1.0f - ior) / (1.0f + ior);
                r0 = r0 * r0;
                float fresnel = r0 + (1.0f - r0) * powf(1.0f - cos_i, 5.0f);
                float sin2_t = eta * eta * (1.0f - cos_i * cos_i);
                bool tir = sin2_t > 1.0f;

                if (tir || rand01(seed) < fresnel) {
                    float3 refl = f3_reflect(dir, n);
                    if (g_rough > 1e-4f) {
                        refl = f3_norm(f3_add(refl, f3_mul(sample_cosine_hemisphere(n, seed), g_rough)));
                    }
                    dir = refl;
                    origin = f3_add(hit_pos, f3_mul(n, 0.002f));
                } else {
                    float cos_t = sqrtf(fmaxf(0.0f, 1.0f - sin2_t));
                    float3 refr = f3_norm(f3_add(f3_mul(dir, eta), f3_mul(n, eta * cos_i - cos_t)));
                    if (g_rough > 1e-4f) {
                        refr = f3_norm(f3_add(refr, f3_mul(sample_cosine_hemisphere(refr, seed), g_rough)));
                    }
                    dir = refr;
                    origin = f3_sub(hit_pos, f3_mul(n, 0.002f));
                }
                continue;
            }

            if (inst_id == 3u) {
                // Emissive neon accent ring/sphere
                float pulse = 0.85f + 0.15f * sinf(params.time * 2.5f);
                float angle = atan2f(hit_pos.z, hit_pos.x);
                float mix_t = 0.5f + 0.5f * sinf(angle * 2.0f + params.time * 1.2f);
                float3 neon_a = make_float3(0.15f, 1.85f, 1.45f);
                float3 neon_b = make_float3(1.65f, 0.35f, 1.85f);
                float3 emission = f3_mul(f3_add(f3_mul(neon_a, 1.0f - mix_t), f3_mul(neon_b, mix_t)), pulse * 2.2f);
                sample_col = f3_add(sample_col, f3_mul3(throughput, emission));
                if (bounce == 0 && s == 0) {
                    first_albedo = make_float3(
                        fminf(1.0f, emission.x * 0.45f),
                        fminf(1.0f, emission.y * 0.45f),
                        fminf(1.0f, emission.z * 0.45f)
                    );
                    first_normal = n;
                }
                break;
            }

            float metallic = 0.0f;
            float rough = params.sun_dir[3];
            float horizon_fade = 0.0f;
            if (inst_id == 0u) {
                // Procedural scratched uneven metal floor (OptiX raytraced)
                float f_rough = params.floor_params[0];
                float f_scratch = params.floor_params[1];
                float f_scale = fmaxf(0.1f, params.floor_params[2]);
                float f_metal = params.floor_params[3];

                float h0 = floor_height(hit_pos.x, hit_pos.z, f_scale, f_scratch);
                float bump_eps = 0.006f;
                float hx = floor_height(hit_pos.x + bump_eps, hit_pos.z, f_scale, f_scratch);
                float hz = floor_height(hit_pos.x, hit_pos.z + bump_eps, f_scale, f_scratch);
                float bump_strength = (0.04f + 0.16f * f_scratch);
                float dhd_x = (hx - h0) / bump_eps * bump_strength;
                float dhd_z = (hz - h0) / bump_eps * bump_strength;
                n = f3_norm(make_float3(n.x - dhd_x, n.y, n.z - dhd_z));

                float sx = hit_pos.x * f_scale;
                float sz = hit_pos.z * f_scale;
                float patina = fbm2(sx * 0.45f + 11.3f, sz * 0.45f - 7.1f);
                float groove = fmaxf(0.0f, -h0 * 1.4f);

                // Brushed steel / titanium albedo (physically bounded, no overexposure)
                float3 steel_dark = make_float3(0.14f, 0.15f, 0.17f);
                float3 steel_bright = make_float3(0.36f, 0.38f, 0.42f);
                float3 warm_tint = make_float3(0.28f, 0.25f, 0.22f);
                base_col = f3_add(f3_mul(steel_dark, 1.0f - patina), f3_mul(steel_bright, patina));
                base_col = f3_add(f3_mul(base_col, 1.0f - groove * 0.45f), f3_mul(warm_tint, groove * 0.45f));

                rough = fmaxf(0.03f, fminf(0.88f, f_rough + 0.18f * (1.0f - patina) + 0.22f * groove));
                metallic = fmaxf(0.0f, fminf(1.0f, f_metal * (1.0f - groove * 0.25f)));
            } else {
                // Fractal custom primitive uses orbit-trap base_col from p5
                metallic = params.fractal_r1[0];
            }

            if (horizon_fade >= 0.995f) {
                float3 sky = sky_color(dir, sun_dir, bounce == 0);
                sample_col = f3_add(sample_col, f3_mul3(throughput, sky));
                break;
            }

            if (bounce == 0 && s == 0) {
                first_albedo = base_col;
                first_normal = n;
            }

            float ndotl = fmaxf(0.0f, f3_dot(n, sun_dir));
            if (ndotl > 0.0f) {
                bool occluded = false;
                if (params.enable_shadows != 0u) {
                    float3 jitter = make_float3(rand01(seed) - 0.5f, rand01(seed) - 0.5f, rand01(seed) - 0.5f);
                    float3 sdir = f3_norm(f3_add(sun_dir, f3_mul(jitter, 0.045f)));
                    occluded = trace_shadow(f3_add(hit_pos, f3_mul(n, 0.003f)), sdir, 1000.0f);
                }
                if (!occluded) {
                    float sun_i = params.fractal_r1[3];
                    float3 sun_rad = make_float3(sun_i, sun_i * 0.93f, sun_i * 0.80f);
                    // Energy-balanced diffuse (1 / PI) + specular lobe
                    float3 diff_term = f3_mul(f3_mul3(base_col, sun_rad), ndotl * 0.318309886f * (1.0f - metallic * 0.75f));
                    float3 half_v = f3_norm(f3_sub(sun_dir, dir));
                    float ndoth = fmaxf(0.0f, f3_dot(n, half_v));
                    float spec_exp = fmaxf(4.0f, 2.0f / (rough * rough + 0.002f) - 2.0f);
                    float spec_norm = (spec_exp + 2.0f) * 0.125f * 0.318309886f;
                    float spec_val = powf(ndoth, fminf(spec_exp, 512.0f)) * fminf(spec_norm, 6.0f);
                    float3 spec_tint = f3_add(make_float3(0.04f, 0.04f, 0.04f), f3_mul(base_col, metallic));
                    float3 spec_term = f3_mul(f3_mul3(spec_tint, sun_rad), spec_val * ndotl);
                    float3 direct = f3_add(diff_term, spec_term);
                    if (horizon_fade > 0.0f) {
                        float3 sky_h = sky_color(dir, sun_dir, bounce == 0);
                        direct = f3_add(f3_mul(direct, 1.0f - horizon_fade), f3_mul(sky_h, horizon_fade));
                    }
                    sample_col = f3_add(sample_col, f3_mul3(throughput, direct));
                }
            }
            if (horizon_fade > 0.0f) {
                float3 sky_h = sky_color(dir, sun_dir, bounce == 0);
                sample_col = f3_add(sample_col, f3_mul3(throughput, f3_mul(sky_h, horizon_fade * 0.5f)));
            }

            float3 diff_dir = sample_cosine_hemisphere(n, seed);
            float3 refl_dir = f3_reflect(dir, n);
            float3 spec_dir = f3_norm(f3_add(refl_dir, f3_mul(diff_dir, rough)));
            bool do_spec = (rand01(seed) < (0.15f + 0.75f * metallic));
            dir = do_spec ? spec_dir : diff_dir;
            origin = f3_add(hit_pos, f3_mul(n, 0.003f));
            throughput = f3_mul3(throughput, base_col);
        }

        accum_col = f3_add(accum_col, sample_col);
    }

    float inv_spp = 1.0f / (float)spp;
    float3 final_col = f3_mul(accum_col, inv_spp);
    if (params.aov_mode == 1u) {
        final_col = first_albedo;
    } else if (params.aov_mode == 2u) {
        final_col = first_normal;
    } else if (params.aov_mode == 3u) {
        float dv = (first_depth > 1e5f) ? 0.0f : (1.0f / (1.0f + first_depth * 0.15f));
        final_col = make_float3(dv, dv, dv);
    } else if (params.aov_mode == 4u) {
        float mag = sqrtf(first_motion.x * first_motion.x + first_motion.y * first_motion.y);
        final_col = make_float3(
            fmaxf(0.0f, fminf(1.0f, 0.5f + first_motion.x * 0.05f)),
            fmaxf(0.0f, fminf(1.0f, 0.5f + first_motion.y * 0.05f)),
            fminf(1.0f, mag * 0.05f)
        );
    }

    unsigned int out_idx = idx.y * params.width + idx.x;
    if (params.out_motion_f32 != 0ull) {
        ((float2*)params.out_motion_f32)[out_idx] = first_motion;
    }
    if (params.out_depth_f32 != 0ull) {
        ((float*)params.out_depth_f32)[out_idx] = first_depth;
    }
    float4* color_buf  = (float4*)params.out_color_f32;
    float4* albedo_buf = (float4*)params.out_albedo_f32;
    float4* normal_buf = (float4*)params.out_normal_f32;
    if (params.accum_count == 0u) {
        color_buf[out_idx]  = make_float4(final_col.x, final_col.y, final_col.z, 1.0f);
        albedo_buf[out_idx] = make_float4(first_albedo.x, first_albedo.y, first_albedo.z, 1.0f);
        normal_buf[out_idx] = make_float4(first_normal.x, first_normal.y, first_normal.z, 1.0f);
    } else {
        float w = 1.0f / (float)(params.accum_count + 1u);
        float4 pc = color_buf[out_idx];
        float4 pa = albedo_buf[out_idx];
        float4 pn = normal_buf[out_idx];
        color_buf[out_idx] = make_float4(
            pc.x + (final_col.x - pc.x) * w,
            pc.y + (final_col.y - pc.y) * w,
            pc.z + (final_col.z - pc.z) * w,
            1.0f
        );
        albedo_buf[out_idx] = make_float4(
            pa.x + (first_albedo.x - pa.x) * w,
            pa.y + (first_albedo.y - pa.y) * w,
            pa.z + (first_albedo.z - pa.z) * w,
            1.0f
        );
        normal_buf[out_idx] = make_float4(
            pn.x + (first_normal.x - pn.x) * w,
            pn.y + (first_normal.y - pn.y) * w,
            pn.z + (first_normal.z - pn.z) * w,
            1.0f
        );
    }
}

extern "C" __global__ void __miss__miss() {
    optixSetPayload_0(0u);
}

extern "C" __global__ void __miss__shadow() {
    optixSetPayload_0(0u);
}

extern "C" __global__ void __closesthit__hit() {
    OptixTraversableHandle gas = optixGetGASTraversableHandle();
    unsigned int prim_idx = optixGetPrimitiveIndex();
    unsigned int sbt_idx = optixGetSbtGASIndex();
    float3 tri[3];
    optixGetTriangleVertexData(gas, prim_idx, sbt_idx, 0.0f, tri);

    float3 e1 = f3_sub(tri[1], tri[0]);
    float3 e2 = f3_sub(tri[2], tri[0]);
    float3 obj_n = f3_norm(f3_cross(e1, e2));
    float3 world_n = f3_norm(optixTransformNormalFromObjectToWorldSpace(obj_n));

    optixSetPayload_0(1u);
    optixSetPayload_1(__float_as_uint(optixGetRayTmax()));
    optixSetPayload_2(__float_as_uint(world_n.x));
    optixSetPayload_3(__float_as_uint(world_n.y));
    optixSetPayload_4(__float_as_uint(world_n.z));
    optixSetPayload_5(0xFFE0E0E0u);
    optixSetPayload_6(optixGetInstanceId());
    optixSetPayload_7(prim_idx);
}

extern "C" __global__ void __intersection__fractal() {
    float3 orig = optixGetObjectRayOrigin();
    float3 dir  = f3_norm(optixGetObjectRayDirection());
    float tmin  = optixGetRayTmin();
    float tmax  = optixGetRayTmax();

    // Ray-AABB clip against [-1.5, 1.5]^3
    float t0 = tmin;
    float t1 = tmax;
    float bmin = -1.5f;
    float bmax = 1.5f;
    float* o = &orig.x;
    float* d = &dir.x;
    for (int i = 0; i < 3; ++i) {
        float invD = 1.0f / (fabsf(d[i]) > 1e-7f ? d[i] : 1e-7f);
        float ta = (bmin - o[i]) * invD;
        float tb = (bmax - o[i]) * invD;
        if (invD < 0.0f) { float tmp = ta; ta = tb; tb = tmp; }
        t0 = fmaxf(t0, ta);
        t1 = fminf(t1, tb);
        if (t0 > t1) return;
    }

    float detail_eps = fmaxf(0.00002f, params.fractal_r0[0]);
    float fudge = fmaxf(0.2f, fminf(1.2f, params.fractal_r0[1]));
    float norm_eps = fmaxf(0.00002f, params.fractal_r0[2]);
    int max_steps = (int)fmaxf(16.0f, fminf(384.0f, params.fractal_r0[3]));

    float t = t0;
    for (int step = 0; step < 384; ++step) {
        if (step >= max_steps || t > t1) return;
        float3 p = f3_add(orig, f3_mul(dir, t));
        float2 res = eval_fractal(p);
        float dist = res.x;
        float eps = detail_eps * (1.0f + t * 0.25f);
        if (dist < eps) {
            float3 obj_n = fractal_normal(p, fmaxf(norm_eps, eps));
            float3 col = trap_palette(res.y);
            unsigned int packed =
                ((unsigned int)(col.x * 255.0f) & 0xFFu) |
                (((unsigned int)(col.y * 255.0f) & 0xFFu) << 8u) |
                (((unsigned int)(col.z * 255.0f) & 0xFFu) << 16u);
            optixReportIntersection(
                t, 0,
                __float_as_uint(obj_n.x),
                __float_as_uint(obj_n.y),
                __float_as_uint(obj_n.z),
                packed
            );
            return;
        }
        t += fmaxf(dist * fudge, eps * 0.5f);
    }
}

extern "C" __global__ void __closesthit__fractal() {
    float3 obj_n = make_float3(
        __uint_as_float(optixGetAttribute_0()),
        __uint_as_float(optixGetAttribute_1()),
        __uint_as_float(optixGetAttribute_2())
    );
    float3 world_n = f3_norm(optixTransformNormalFromObjectToWorldSpace(obj_n));
    unsigned int packed = optixGetAttribute_3();

    optixSetPayload_0(1u);
    optixSetPayload_1(__float_as_uint(optixGetRayTmax()));
    optixSetPayload_2(__float_as_uint(world_n.x));
    optixSetPayload_3(__float_as_uint(world_n.y));
    optixSetPayload_4(__float_as_uint(world_n.z));
    optixSetPayload_5(packed);
    optixSetPayload_6(optixGetInstanceId());
    optixSetPayload_7(0u);
}

extern "C" __global__ void __intersection__sphere() {
    float3 orig = optixGetObjectRayOrigin();
    float3 dir  = optixGetObjectRayDirection();
    float tmin  = optixGetRayTmin();
    float tmax  = optixGetRayTmax();

    float a = f3_dot(dir, dir);
    if (a <= 1e-12f) return;
    float b = f3_dot(orig, dir);
    float c = f3_dot(orig, orig) - 1.0f;
    float disc = b * b - a * c;
    if (disc < 0.0f) return;

    float sq = sqrtf(disc);
    float inv_a = 1.0f / a;
    float t = (-b - sq) * inv_a;
    if (t < tmin || t > tmax) {
        t = (-b + sq) * inv_a;
        if (t < tmin || t > tmax) return;
    }

    float3 hit_obj = f3_add(orig, f3_mul(dir, t));
    float3 obj_n = f3_norm(hit_obj);
    optixReportIntersection(
        t, 0,
        __float_as_uint(obj_n.x),
        __float_as_uint(obj_n.y),
        __float_as_uint(obj_n.z),
        0xFFE0E0E0u
    );
}

