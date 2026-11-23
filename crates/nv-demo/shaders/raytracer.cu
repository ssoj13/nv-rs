#include <optix.h>
#include "csg_eval.cuh"

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
    float sky_params[4];       // [sky_intensity, sun_size, horizon_haze, sky_mode]
    unsigned int accum_count;
    unsigned int enable_ser;
    unsigned int _pad0[2];
    unsigned long long out_motion_f32;
    unsigned long long out_depth_f32;
    float prev_cam_pos[4];
    float prev_cam_forward[4];
    float prev_cam_right[4];
    float prev_cam_up[4];
    float sat_pos[4];      // [x, y, z, radius/enable]
    float glass_params[4]; // [ior, absorption, roughness, enable]
    unsigned int aov_mode; // 0=Beauty, 1=Albedo, 2=Normal, 3=Depth, 4=Motion, 5=SemanticId, 6=InstanceId
    float jitter_x;
    float jitter_y;
    unsigned int _pad1;
    float env_params[8];
    unsigned long long out_diffuse_albedo_f32;
    unsigned long long out_spec_albedo_f32;
    // Render-time analytical CSG scene device pointers & controls
    unsigned long long csg_compounds;
    unsigned long long csg_ops;
    unsigned long long csg_prims;
    unsigned long long csg_aux_planes;
    unsigned long long csg_guard_aabbs;
    unsigned long long csg_repeats;
    unsigned long long csg_materials;
    unsigned long long csg_instances;
    unsigned int csg_instance_count;
    unsigned int csg_material_count;
    unsigned int csg_compound_count;
    unsigned int csg_flags;      // bit0=enabled, bit1=night, bit2=wet_asphalt, bit3=sunset
    float csg_wetness;           // 0..1 smooth asphalt wetness
    float csg_traffic_time;      // simulation clock for water waves / aviation beacons
    unsigned int camera_projection; // 0=Pinhole, 1=Fisheye180, 2=Equirect360, 3=SlitCam
    unsigned int probe_grid_wh;     // (grid_h << 16) | grid_w for __raygen__score_views
    unsigned long long probe_candidates; // device ptr to GpuProbeCandidate[num_candidates]
    unsigned long long probe_samples;    // device ptr to GpuProbeSample[num_candidates * grid_w * grid_h]
    unsigned long long csg_prev_xforms;  // device ptr to float[12] prev-frame 3x4 per visible CSG instance
    unsigned long long out_semantic_u32; // optional uint32[w*h] raw semantic ID AOV for SDG EXR capture
    unsigned long long out_instance_u32; // optional uint32[w*h] raw instance user_id AOV for SDG EXR capture
    unsigned long long sdg_ray_queries;  // device ptr to GpuSdgRayQuery[sdg_ray_count]
    unsigned long long sdg_ray_results;  // device ptr to uint32[sdg_ray_count] (0xFFFFFFFFu on miss)
    unsigned int sdg_ray_count;          // number of active rays for __raygen__sdg_keypoints
    unsigned int _pad2;                  // 16-byte struct tail alignment
};

static_assert(sizeof(Params) == 640, "Params size must match LaunchParams (640 B)");
static_assert(__builtin_offsetof(Params, csg_prev_xforms) == 592, "Params::csg_prev_xforms offset must be 592");
static_assert(__builtin_offsetof(Params, out_semantic_u32) == 600, "Params::out_semantic_u32 offset must be 600");
static_assert(__builtin_offsetof(Params, out_instance_u32) == 608, "Params::out_instance_u32 offset must be 608");
static_assert(__builtin_offsetof(Params, sdg_ray_queries) == 616, "Params::sdg_ray_queries offset must be 616");
static_assert(__builtin_offsetof(Params, sdg_ray_results) == 624, "Params::sdg_ray_results offset must be 624");
static_assert(__builtin_offsetof(Params, sdg_ray_count) == 632, "Params::sdg_ray_count offset must be 632");

struct GpuSdgRayQuery {
    float origin_tmin[4]; // xyz = world origin, w = tmin
    float dir_tmax[4];    // xyz = unit world dir, w = tmax
};
static_assert(sizeof(GpuSdgRayQuery) == 32, "GpuSdgRayQuery must be 32 B");

struct GpuProbeCandidate {
    float eye[4];     // xyz = origin, w = unused
    float forward[4]; // xyz = unit forward, w = aspect
    float right[4];   // xyz = unit right, w = unused
    float up[4];      // xyz = unit up, w = tan_half_fov
};
static_assert(sizeof(GpuProbeCandidate) == 64, "GpuProbeCandidate must be 64 B");

struct GpuProbeSample {
    float t;                // <= 0.0f on sky miss
    unsigned short sem_id;  // SemanticId::0
    unsigned short _pad0;
    unsigned int inst_id;   // 0xFFFFFFFFu on miss
    unsigned int _pad1;
};
static_assert(sizeof(GpuProbeSample) == 16, "GpuProbeSample must match csg_scene::ProbeSample (16 B)");

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

static __forceinline__ __device__ float2 sdf_mandelbulb(float3 q) {
    float power = params.fractal_p0[0];
    float bailout = fmaxf(2.0f, params.fractal_p0[1]);
    bool julia = params.fractal_p0[2] > 0.5f;
    int max_iter = (int)fmaxf(2.0f, fminf(32.0f, params.fractal_p1[2]));
    float3 c_add = julia ? make_f3(params.fractal_p2) : q;
    float theta_mult = params.fractal_p3[0];
    float phi_mult = params.fractal_p3[1];
    float theta_phase = params.fractal_p3[2];
    float phi_phase = params.fractal_p3[3];
    float ang_scale = fmaxf(1.0f, fmaxf(fabsf(theta_mult), fabsf(phi_mult)));
    float dr_add = julia ? 0.0f : 1.0f;

    float3 z = q;
    float dr = 1.0f;
    float r = f3_len(z);
    float trap = 1e6f;
    for (int i = 0; i < 32; ++i) {
        if (i >= max_iter || r > bailout || dr > 1e20f) break;
        if (r < 1e-6f) return make_float2(0.0f, trap);
        float theta = acosf(fmaxf(-1.0f, fminf(1.0f, z.z / r))) * theta_mult + theta_phase;
        float phi = atan2f(z.y, z.x) * phi_mult + phi_phase;
        float rp = powf(r, power - 1.0f);
        dr = rp * power * ang_scale * dr + dr_add;
        float zr = rp * r;
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
    int max_iter = (int)fmaxf(2.0f, fminf(32.0f, params.fractal_p1[2]));
    float fold_mult = params.fractal_p3[0];
    float iter_rot = params.fractal_p3[1];
    float min_r2 = min_r * min_r;
    float fixed_r2 = fixed_r * fixed_r;
    float abs_scale = fabsf(scale);

    float3 z = q;
    float dr = 1.0f;
    float trap = 1e6f;
    for (int i = 0; i < 32; ++i) {
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
        dr = dr * abs_scale + 1.0f;
        trap = fminf(trap, r2);
        if (f3_dot(z, z) > bailout2 || dr > 1e20f) break;
    }
    // Exact Lowe/Knighty Mandelbox distance estimator: (|z| - |scale - 1|) / |dr| - |scale|^(1 - max_iter)
    float c1 = fabsf(scale - 1.0f);
    float c2 = (abs_scale > 1.05f) ? powf(abs_scale, (float)(1 - max_iter)) : 0.0f;
    float de = (f3_len(z) - c1) / fmaxf(fabsf(dr), 1e-6f) - c2;
    return make_float2(de, sqrtf(fmaxf(trap, 0.0f)));
}

static __forceinline__ __device__ float2 sdf_kifs_octahedron(float3 q) {
    float sc = fmaxf(1.2f, fabsf(params.fractal_p0[0]));
    float bailout2 = params.fractal_p0[1] * params.fractal_p0[1];
    float a1 = params.fractal_p1[0];
    float a2 = params.fractal_p1[1];
    int max_iter = (int)fmaxf(2.0f, fminf(32.0f, params.fractal_p1[2]));
    float3 off = make_f3(params.fractal_p2);
    float thickness = fmaxf(0.005f, params.fractal_p3[0]);
    float z_bias = params.fractal_p3[1];

    float3 z = q;
    float s = 1.0f;
    float trap = 1e6f;
    for (int i = 0; i < 32; ++i) {
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
        if (r2 > bailout2 || s > 1e18f) break;
    }
    // Rounded box base primitive matching the [-off, +off] KIFS/Menger invariant domain
    float ox = fmaxf(0.1f, fabsf(off.x));
    float oy = fmaxf(0.1f, fabsf(off.y));
    float oz = fmaxf(0.1f, fabsf(off.z));
    float bevel = fminf(thickness, fminf(ox, fminf(oy, oz)) * 0.45f);
    float bx = fabsf(z.x) - (ox - bevel);
    float by = fabsf(z.y) - (oy - bevel);
    float bz = fabsf(z.z) - (oz - bevel);
    float d_box = fminf(fmaxf(bx, fmaxf(by, bz)), 0.0f)
        + f3_len(make_float3(fmaxf(bx, 0.0f), fmaxf(by, 0.0f), fmaxf(bz, 0.0f)))
        - bevel;
    return make_float2(d_box / s, sqrtf(fmaxf(trap, 0.0f)));
}

static __forceinline__ __device__ float2 eval_fractal(float3 q) {
    // Object-to-world TRS (Translate * Rz * Ry * Rx * Scale) lives in the OptiX TLAS
    // OptixInstance::transform; eval_fractal evaluates the canonical unscaled SDF directly.
    if (params.fractal_kind == 1u) {
        float2 r = sdf_mandelbox(f3_mul(q, 2.2f));
        return make_float2(r.x * (1.0f / 2.2f), r.y);
    } else if (params.fractal_kind == 2u) {
        return sdf_kifs_octahedron(q);
    } else {
        return sdf_mandelbulb(q);
    }
}

static __forceinline__ __device__ float fractal_hit_eps(float path_dist) {
    float detail_eps = fmaxf(0.00001f, params.fractal_r0[0]);
    float pixel_angle = (2.0f * fmaxf(params.cam_up[3], 0.05f)) / fmaxf((float)params.height, 64.0f);
    float cone = detail_eps * 400.0f * pixel_angle;
    return fmaxf(2.0e-6f, path_dist * cone);
}

static __forceinline__ __device__ float3 fractal_normal(float3 p, float eps) {
    // 4-tap tetrahedral gradient: 33% fewer SDF evaluations than 6-tap central differences,
    // with an ULP-safe floor to prevent float32 cancellation on extreme close-ups.
    float e = fmaxf(eps, 2.0e-6f * fmaxf(1.0f, f3_len(p)));
    float d0 = eval_fractal(make_float3(p.x + e, p.y - e, p.z - e)).x;
    float d1 = eval_fractal(make_float3(p.x - e, p.y - e, p.z + e)).x;
    float d2 = eval_fractal(make_float3(p.x - e, p.y + e, p.z - e)).x;
    float d3 = eval_fractal(make_float3(p.x + e, p.y + e, p.z + e)).x;
    return f3_norm(make_float3(
        d0 - d1 - d2 + d3,
       -d0 - d1 + d2 + d3,
       -d0 + d1 - d2 + d3
    ));
}

static __forceinline__ __device__ float fractal_ao(float3 p, float3 n, float base_eps) {
    // 3-tap multi-scale SDF cavity occlusion along surface normal (zoom-adaptive micro + macro).
    // Bounded to [0.45, 1.0] so it accentuates micro-crevices and denoiser albedo guides
    // without crushing physically-traced indirect shadows to black.
    float h0 = fmaxf(base_eps * 5.0f, 0.00005f);
    float h1 = h0 * 3.0f;
    float h2 = fmaxf(h1 * 3.0f, 0.015f);
    float occ = 0.0f;
    float d0 = eval_fractal(f3_add(p, f3_mul(n, h0))).x;
    occ += 0.45f * fmaxf(0.0f, fminf(1.0f, (h0 - d0) / h0));
    float d1 = eval_fractal(f3_add(p, f3_mul(n, h1))).x;
    occ += 0.35f * fmaxf(0.0f, fminf(1.0f, (h1 - d1) / h1));
    float d2 = eval_fractal(f3_add(p, f3_mul(n, h2))).x;
    occ += 0.20f * fmaxf(0.0f, fminf(1.0f, (h2 - d2) / h2));
    return fmaxf(0.45f, fminf(1.0f, 1.0f - occ * 0.55f));
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

static __noinline__ __device__ float floor_height(float x, float z, float scale, float scratch_amt) {
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

static __forceinline__ __device__ float sky_hash3(float3 p) {
    float3 q = make_float3(
        fmodf(fabsf(p.x) * 17.31f + fabsf(p.y) * 53.17f + fabsf(p.z) * 97.43f, 1.0f),
        fmodf(fabsf(p.y) * 31.71f + fabsf(p.z) * 19.83f + fabsf(p.x) * 67.19f, 1.0f),
        fmodf(fabsf(p.z) * 43.29f + fabsf(p.x) * 71.53f + fabsf(p.y) * 23.11f, 1.0f)
    );
    float d = q.x * 12.9898f + q.y * 78.233f + q.z * 37.719f;
    float s = sinf(d) * 43758.5453f;
    return s - floorf(s);
}

static __forceinline__ __device__ float sky_hash2(float x, float z) {
    float h = sinf(x * 127.1f + z * 311.7f) * 43758.5453123f;
    return h - floorf(h);
}

// Inigo Quilez "Elevated" 2D value noise with analytic derivatives: (val, dval/dx, dval/dz)
static __forceinline__ __device__ float3 elevated_noised(float x, float z) {
    float ix = floorf(x);
    float iz = floorf(z);
    float fx = x - ix;
    float fz = z - iz;

    float ux = fx * fx * fx * (fx * (fx * 6.0f - 15.0f) + 10.0f);
    float uz = fz * fz * fz * (fz * (fz * 6.0f - 15.0f) + 10.0f);
    float dux = 30.0f * fx * fx * (fx * (fx - 2.0f) + 1.0f);
    float duz = 30.0f * fz * fz * (fz * (fz - 2.0f) + 1.0f);

    float a = sky_hash2(ix + 0.0f, iz + 0.0f);
    float b = sky_hash2(ix + 1.0f, iz + 0.0f);
    float c = sky_hash2(ix + 0.0f, iz + 1.0f);
    float d = sky_hash2(ix + 1.0f, iz + 1.0f);

    float k0 = a;
    float k1 = b - a;
    float k2 = c - a;
    float k3 = a - b - c + d;

    float val = k0 + k1 * ux + k2 * uz + k3 * ux * uz;
    float dx = dux * (k1 + k3 * uz);
    float dz = duz * (k2 + k3 * ux);
    return make_float3(val, dx, dz);
}

// iq's gradient-damped FBM mountain heightfield (returns height in world units)
static __forceinline__ __device__ float elevated_terrain(float x, float z, int octaves) {
    float a = 0.0f;
    float b = 1.0f;
    float dx = 0.0f;
    float dz = 0.0f;
    float px = x * 0.0022f;
    float pz = z * 0.0022f;
    for (int i = 0; i < octaves; ++i) {
        float3 n = elevated_noised(px, pz);
        dx += n.y;
        dz += n.z;
        a += b * n.x / (1.0f + dx * dx + dz * dz);
        b *= 0.5f;
        // Rotate & scale octave (0.8, -0.6; 0.6, 0.8) * 2.0
        float nx = (0.8f * px - 0.6f * pz) * 2.0f;
        float nz = (0.6f * px + 0.8f * pz) * 2.0f;
        px = nx;
        pz = nz;
    }
    return 185.0f * a;
}

static __forceinline__ __device__ float3 elevated_normal(float x, float z, float t, int octaves) {
    float eps = fmaxf(0.15f, 0.0015f * t);
    float hx = elevated_terrain(x - eps, z, octaves) - elevated_terrain(x + eps, z, octaves);
    float hz = elevated_terrain(x, z - eps, octaves) - elevated_terrain(x, z + eps, octaves);
    return f3_norm(make_float3(hx, 2.0f * eps, hz));
}

static __forceinline__ __device__ float sky_noise3(float3 p) {
    float3 i = make_float3(floorf(p.x), floorf(p.y), floorf(p.z));
    float3 f = make_float3(p.x - i.x, p.y - i.y, p.z - i.z);
    float3 u = make_float3(
        f.x * f.x * f.x * (f.x * (f.x * 6.0f - 15.0f) + 10.0f),
        f.y * f.y * f.y * (f.y * (f.y * 6.0f - 15.0f) + 10.0f),
        f.z * f.z * f.z * (f.z * (f.z * 6.0f - 15.0f) + 10.0f)
    );
    float n000 = sky_hash3(make_float3(i.x + 0.0f, i.y + 0.0f, i.z + 0.0f));
    float n100 = sky_hash3(make_float3(i.x + 1.0f, i.y + 0.0f, i.z + 0.0f));
    float n010 = sky_hash3(make_float3(i.x + 0.0f, i.y + 1.0f, i.z + 0.0f));
    float n110 = sky_hash3(make_float3(i.x + 1.0f, i.y + 1.0f, i.z + 0.0f));
    float n001 = sky_hash3(make_float3(i.x + 0.0f, i.y + 0.0f, i.z + 1.0f));
    float n101 = sky_hash3(make_float3(i.x + 1.0f, i.y + 0.0f, i.z + 1.0f));
    float n011 = sky_hash3(make_float3(i.x + 0.0f, i.y + 1.0f, i.z + 1.0f));
    float n111 = sky_hash3(make_float3(i.x + 1.0f, i.y + 1.0f, i.z + 1.0f));
    float nx00 = n000 + (n100 - n000) * u.x;
    float nx10 = n010 + (n110 - n010) * u.x;
    float nx01 = n001 + (n101 - n001) * u.x;
    float nx11 = n011 + (n111 - n011) * u.x;
    float nxy0 = nx00 + (nx10 - nx00) * u.y;
    float nxy1 = nx01 + (nx11 - nx01) * u.y;
    return nxy0 + (nxy1 - nxy0) * u.z;
}

static __forceinline__ __device__ float sky_fbm3(float3 p, int octaves) {
    float val = 0.0f;
    float amp = 0.5f;
    float norm = 0.0f;
    for (int i = 0; i < octaves; ++i) {
        val += amp * sky_noise3(p);
        norm += amp;
        p = make_float3(
            1.62f * p.x - 1.18f * p.z + 1.37f,
            2.01f * p.y + 0.73f,
            1.18f * p.x + 1.62f * p.z - 0.91f
        );
        amp *= 0.5f;
    }
    return val / fmaxf(norm, 1e-4f);
}

static __forceinline__ __device__ float sky_smoothstep(float e0, float e1, float x) {
    float t = fmaxf(0.0f, fminf(1.0f, (x - e0) / fmaxf(e1 - e0, 1e-5f)));
    return t * t * (3.0f - 2.0f * t);
}

static __forceinline__ __device__ float3 sky_starfield(
    float3 dir,
    float density,
    float brightness,
    float dust_occ
) {
    if (density <= 0.001f || brightness <= 0.001f) {
        return make_float3(0.0f, 0.0f, 0.0f);
    }
    float ax = fabsf(dir.x), ay = fabsf(dir.y), az = fabsf(dir.z);
    float u, v, face;
    if (ax >= ay && ax >= az) {
        u = dir.z / ax; v = dir.y / ax; face = (dir.x >= 0.0f) ? 0.0f : 1.0f;
    } else if (ay >= ax && ay >= az) {
        u = dir.x / ay; v = dir.z / ay; face = (dir.y >= 0.0f) ? 2.0f : 3.0f;
    } else {
        u = dir.x / az; v = dir.y / az; face = (dir.z >= 0.0f) ? 4.0f : 5.0f;
    }

    float3 stars = make_float3(0.0f, 0.0f, 0.0f);
    const float scales[3] = { 105.0f, 210.0f, 410.0f };
    const float probs[3]  = { 0.018f, 0.030f, 0.045f };
    const float gains[3]  = { 3.2f, 1.45f, 0.65f };

    for (int l = 0; l < 3; ++l) {
        float sc = scales[l];
        float px = (u * 0.5f + 0.5f) * sc;
        float py = (v * 0.5f + 0.5f) * sc;
        float ix = floorf(px);
        float iy = floorf(py);
        float fx = px - ix;
        float fy = py - iy;

        float h0 = sky_hash3(make_float3(ix, iy, face * 17.0f + (float)l * 53.0f));
        float thresh = 1.0f - fminf(0.45f, probs[l] * density);
        if (h0 > thresh) {
            float h1 = sky_hash3(make_float3(ix + 11.3f, iy + 7.1f, face * 23.0f + (float)l * 31.0f));
            float h2 = sky_hash3(make_float3(ix + 29.7f, iy + 19.3f, face * 37.0f + (float)l * 13.0f));
            float h3 = sky_hash3(make_float3(ix + 43.1f, iy + 3.7f, face * 41.0f + (float)l * 7.0f));

            float cx = 0.22f + 0.56f * h1;
            float cy = 0.22f + 0.56f * h2;
            float dx = fx - cx;
            float dy = fy - cy;
            float d2 = dx * dx + dy * dy;
            float d = sqrtf(d2);

            float core = expf(-d2 * 52.0f);
            float halo = 0.14f * expf(-d * 9.5f);
            float mag = (h0 - thresh) / fmaxf(1.0f - thresh, 1e-4f);
            float lum = powf(mag, 2.2f) * gains[l] * (core + halo);

            float3 tint;
            if (h3 < 0.28f) {
                tint = make_float3(0.68f, 0.84f, 1.35f); // O/B blue-white
            } else if (h3 < 0.72f) {
                tint = make_float3(1.00f, 0.99f, 0.96f); // A/F white
            } else {
                tint = make_float3(1.32f, 0.84f, 0.50f); // G/K warm amber
            }
            stars = f3_add(stars, f3_mul(tint, lum));
        }
    }
    return f3_mul(stars, brightness * expf(-dust_occ));
}

static __noinline__ __device__ float3 sky_elevated(float3 dir, float3 sun_dir, bool is_camera_ray) {
    float sky_i = fmaxf(0.05f, params.sky_params[0]);
    float haze_k = fmaxf(0.5f, params.sky_params[2]);
    float y = fmaxf(0.0f, dir.y);

    // Base atmospheric sky + cirrus clouds
    float sun_dot = fmaxf(0.0f, f3_dot(dir, sun_dir));
    float3 sky_col = f3_add(
        f3_mul(make_float3(0.32f, 0.56f, 0.86f), 1.0f - 0.65f * powf(y, 0.5f)),
        f3_mul(make_float3(0.08f, 0.20f, 0.48f), 0.65f * powf(y, 0.5f))
    );
    if (dir.y > 0.02f) {
        float elev_cover = params.env_params[0];
        float ct = 500.0f / dir.y;
        float3 cl = elevated_noised(dir.x * ct * 0.0008f, dir.z * ct * 0.0008f);
        float cl2 = elevated_noised(dir.x * ct * 0.0017f + 3.1f, dir.z * ct * 0.0017f - 1.7f).x;
        float thresh = 0.78f - 0.52f * fmaxf(0.0f, fminf(1.0f, elev_cover));
        float cloud = fmaxf(0.0f, (cl.x * 0.65f + cl2 * 0.35f) - thresh) * 2.3f;
        cloud = fminf(1.0f, cloud) * fminf(1.0f, (dir.y - 0.02f) * 6.0f);
        float3 cloud_col = f3_add(make_float3(0.92f, 0.94f, 0.98f), f3_mul(make_float3(0.35f, 0.24f, 0.12f), powf(sun_dot, 6.0f)));
        sky_col = f3_add(f3_mul(sky_col, 1.0f - cloud * 0.75f), f3_mul(cloud_col, cloud * 0.75f));
    }
    sky_col = f3_add(sky_col, f3_mul(make_float3(1.0f, 0.86f, 0.68f), powf(sun_dot, 8.0f) * 0.25f));

    // Raymarch iq's FBM mountain range below horizon/low angles
    if (dir.y < 0.22f) {
        float ro_y = fmaxf(195.0f, elevated_terrain(0.0f, 0.0f, 4) + 35.0f);
        float3 ro = make_float3(0.0f, ro_y, 0.0f);
        float t = 12.0f;
        float tmax = is_camera_ray ? 2600.0f : 1100.0f;
        int steps = is_camera_ray ? 64 : 20;
        int oct = is_camera_ray ? 7 : 4;
        bool hit = false;
        for (int i = 0; i < steps; ++i) {
            float3 p = f3_add(ro, f3_mul(dir, t));
            float h = elevated_terrain(p.x, p.z, oct);
            float d = p.y - h;
            if (d < 0.003f * t) {
                hit = true;
                break;
            }
            t += fmaxf(1.5f, d * 0.45f + 0.010f * t);
            if (t > tmax) break;
        }
        if (hit) {
            float3 p = f3_add(ro, f3_mul(dir, t));
            float3 n = elevated_normal(p.x, p.z, t, oct);
            float dif = fmaxf(0.0f, f3_dot(n, sun_dir));
            float amb = 0.5f + 0.5f * n.y;

            // Snow on flat high peaks, dark granite on steep cliffs
            float snow = fmaxf(0.0f, fminf(1.0f, (n.y - 0.62f) * 3.5f))
                       * fmaxf(0.0f, fminf(1.0f, (p.y - 68.0f) * 0.03f));
            float3 rock = make_float3(0.14f, 0.13f, 0.12f);
            float3 grass = make_float3(0.11f, 0.16f, 0.09f);
            float3 base = f3_add(f3_mul(grass, fmaxf(0.0f, n.y - 0.5f)), f3_mul(rock, 1.0f - fmaxf(0.0f, n.y - 0.5f)));
            base = f3_add(f3_mul(base, 1.0f - snow), f3_mul(make_float3(0.88f, 0.90f, 0.94f), snow));

            float3 lit = f3_mul(base, dif * 1.35f + amb * 0.35f);
            // Distance & valley exponential fog
            float fog = 1.0f - expf(-t * (0.00085f * haze_k * fmaxf(0.1f, params.env_params[1])));
            float3 fog_col = f3_add(f3_mul(sky_col, 0.75f), make_float3(0.20f, 0.24f, 0.30f));
            float3 col = f3_add(f3_mul(lit, 1.0f - fog), f3_mul(fog_col, fog));
            return f3_mul(col, sky_i);
        }
    }

    float horizon_band = expf(-fabsf(dir.y) * (7.0f / haze_k));
    sky_col = f3_add(sky_col, f3_mul(make_float3(0.75f, 0.78f, 0.84f), horizon_band * 0.25f));
    return f3_mul(sky_col, sky_i);
}

// Mode 6: "Unprogress" style 4k procedural brutalist sci-fi cityscape & volumetric haze
static __forceinline__ __device__ float unprogress_map(float3 p, float* out_win) {
    float cell = 3.6f;
    float ix = floorf(p.x / cell);
    float iz = floorf(p.z / cell);
    float h_rnd = sky_hash2(ix, iz);
    float h_rnd2 = sky_hash2(ix + 13.7f, iz - 9.3f);

    float lx = p.x - (ix + 0.5f) * cell;
    float lz = p.z - (iz + 0.5f) * cell;

    // Safe cell-boundary step bound so varying neighbor heights never cause slicing artifacts
    float d_cell = fminf(0.5f * cell - fabsf(lx), 0.5f * cell - fabsf(lz)) + 0.04f;

    // Keep central avenue clear so towers frame the foreground fractal instead of blocking it
    float avenue = fabsf(ix + 0.5f);
    float bw = 0.78f + 0.52f * h_rnd2;
    float bh = (avenue < 1.2f && fabsf(iz + 0.5f) < 3.5f)
        ? 0.4f
        : (2.5f + 11.5f * h_rnd * h_rnd);

    // Brutalist stepped tower SDF
    float qx = fabsf(lx) - bw;
    float qy = p.y - bh;
    float qz = fabsf(lz) - bw;
    float d_tower = fminf(fmaxf(qx, fmaxf(qy, qz)), 0.0f)
        + sqrtf(fmaxf(qx, 0.0f) * fmaxf(qx, 0.0f) + fmaxf(qy, 0.0f) * fmaxf(qy, 0.0f) + fmaxf(qz, 0.0f) * fmaxf(qz, 0.0f));

    // Upper brutalist overhang collar
    float cx = fabsf(lx) - (bw + 0.16f);
    float cy = fabsf(p.y - bh * 0.68f) - 0.28f;
    float cz = fabsf(lz) - (bw + 0.16f);
    float d_collar = fminf(fmaxf(cx, fmaxf(cy, cz)), 0.0f)
        + sqrtf(fmaxf(cx, 0.0f) * fmaxf(cx, 0.0f) + fmaxf(cy, 0.0f) * fmaxf(cy, 0.0f) + fmaxf(cz, 0.0f) * fmaxf(cz, 0.0f));

    // Fine recessed architectural slit windows
    float wy = fabsf(fmodf(fabsf(p.y) * 3.2f, 1.0f) - 0.5f);
    float wx = fabsf(fmodf(fabsf(lx + lz) * 2.8f, 1.0f) - 0.5f);
    *out_win = (wy < 0.08f && wx < 0.16f && h_rnd > 0.35f && p.y > 0.6f && p.y < bh - 0.4f) ? 1.0f : 0.0f;

    float d_obj = fminf(p.y, fminf(d_tower, d_collar));
    return fminf(d_obj, d_cell);
}

static __noinline__ __device__ float3 sky_unprogress(float3 dir, float3 sun_dir, bool is_camera_ray) {
    float sky_i = fmaxf(0.05f, params.sky_params[0]);
    float haze_k = fmaxf(0.5f, params.sky_params[2]);
    float y = fmaxf(0.0f, dir.y);
    float sun_dot = fmaxf(0.0f, f3_dot(dir, sun_dir));

    float3 overcast = f3_add(
        f3_mul(make_float3(0.05f, 0.07f, 0.10f), powf(y, 0.5f)),
        f3_mul(make_float3(0.16f, 0.22f, 0.28f), 1.0f - powf(y, 0.5f))
    );
    overcast = f3_add(overcast, f3_mul(make_float3(0.75f, 0.82f, 0.90f), powf(sun_dot, 12.0f) * 0.35f));

    if (dir.y < 0.38f) {
        float3 ro = make_float3(0.0f, 8.5f, 0.0f);
        float t = 1.5f;
        float tmax = is_camera_ray ? 58.0f : 26.0f;
        int steps = is_camera_ray ? 64 : 22;
        float win = 0.0f;
        bool hit = false;
        float glow = 0.0f;
        for (int i = 0; i < steps; ++i) {
            float3 p = f3_add(ro, f3_mul(dir, t));
            float d = unprogress_map(p, &win);
            glow += expf(-fabsf(d) * 3.5f) * 0.010f;
            if (d < 0.015f) {
                hit = true;
                break;
            }
            t += fmaxf(0.04f, d * 0.85f);
            if (t > tmax) break;
        }
        if (hit) {
            float3 p = f3_add(ro, f3_mul(dir, t));
            float w_dummy = 0.0f;
            float e = 0.02f;
            float3 n = f3_norm(make_float3(
                unprogress_map(make_float3(p.x + e, p.y, p.z), &w_dummy) - unprogress_map(make_float3(p.x - e, p.y, p.z), &w_dummy),
                unprogress_map(make_float3(p.x, p.y + e, p.z), &w_dummy) - unprogress_map(make_float3(p.x, p.y - e, p.z), &w_dummy),
                unprogress_map(make_float3(p.x, p.y, p.z + e), &w_dummy) - unprogress_map(make_float3(p.x, p.y, p.z - e), &w_dummy)
            ));
            float dif = fmaxf(0.0f, f3_dot(n, sun_dir));
            float3 concrete = make_float3(0.065f, 0.075f, 0.09f);
            float3 lit = f3_mul(concrete, dif * 1.4f + 0.16f + 0.12f * fmaxf(0.0f, n.y));
            if (win > 0.5f && n.y < 0.5f) {
                float tint = sky_hash2(floorf(p.x * 2.0f), floorf(p.y * 2.0f));
                float3 wc = (tint > 0.65f) ? make_float3(0.95f, 0.52f, 0.18f) : make_float3(0.24f, 0.72f, 0.95f);
                lit = f3_add(lit, f3_mul(wc, 0.85f * params.env_params[1]));
            }
            float fog = 1.0f - expf(-t * (0.042f * haze_k * fmaxf(0.1f, params.env_params[0])));
            float3 fog_col = f3_add(overcast, f3_mul(make_float3(0.18f, 0.28f, 0.36f), glow));
            return f3_mul(f3_add(f3_mul(lit, 1.0f - fog), f3_mul(fog_col, fog)), sky_i);
        }
        overcast = f3_add(overcast, f3_mul(make_float3(0.22f, 0.34f, 0.44f), glow));
    }
    return f3_mul(overcast, sky_i);
}

// Mode 7: "Offscreen Colonies" style orbital megastructure ring + modular trusses in deep space
static __forceinline__ __device__ float colonies_map(float3 p, float* out_emissive) {
    // Tilted orbital ring centered at (0, -2.0, 26.0)
    float3 q = make_float3(p.x, p.y + 2.0f, p.z - 26.0f);
    // Tilt around X by ~0.42 rad
    float cy = 0.913f * q.y - 0.408f * q.z;
    float cz = 0.408f * q.y + 0.913f * q.z;
    q.y = cy;
    q.z = cz;

    float r_major = 22.0f;
    float r_xy = sqrtf(q.x * q.x + q.y * q.y);
    float ang = atan2f(q.y, q.x);
    float sector = fmodf(fabsf(ang) * 24.0f / 3.14159265f, 1.0f) - 0.5f;

    // Main outer structural ring (torus-box cross section)
    float dx = fabsf(r_xy - r_major) - 1.6f;
    float dz = fabsf(q.z) - 2.6f;
    float d_ring = fminf(fmaxf(dx, dz), 0.0f) + sqrtf(fmaxf(dx, 0.0f) * fmaxf(dx, 0.0f) + fmaxf(dz, 0.0f) * fmaxf(dz, 0.0f));

    // Inner ribbed truss struts
    float rx = fabsf(r_xy - (r_major - 2.4f)) - 1.1f;
    float rz = fabsf(q.z) - 0.55f;
    float rs = fabsf(sector) - 0.18f;
    float d_rib = fmaxf(fmaxf(rx, rz), rs * 2.2f);

    // Glowing colony habitat strip on inner rim
    *out_emissive = (fabsf(r_xy - (r_major - 1.6f)) < 0.25f && fabsf(q.z) < 0.85f && fabsf(sector) < 0.32f) ? 1.0f : 0.0f;
    return fminf(d_ring, d_rib);
}

static __noinline__ __device__ float3 sky_colonies(float3 dir, float3 sun_dir, bool is_camera_ray) {
    float sky_i = fmaxf(0.05f, params.sky_params[0]);
    float sun_dot = fmaxf(0.0f, f3_dot(dir, sun_dir));

    // Deep space background + subtle atmospheric planet limb + pinpoint stars
    float3 col = make_float3(0.004f, 0.006f, 0.014f);
    float limb = expf(-fabsf(dir.y + 0.35f) * 8.0f) * fmaxf(0.0f, 0.6f - fabsf(dir.x));
    col = f3_add(col, f3_mul(make_float3(0.08f, 0.24f, 0.48f), limb * 0.8f * params.env_params[3]));
    col = f3_add(col, f3_mul(make_float3(0.85f, 0.90f, 1.0f), powf(sun_dot, 32.0f) * 0.45f));

    if (is_camera_ray) {
        col = f3_add(col, sky_starfield(dir, params.env_params[0], params.env_params[1], 0.0f));
    }

    // Raymarch the orbital colony megastructure
    float t = 4.0f;
    float tmax = 58.0f;
    int steps = is_camera_ray ? 48 : 20;
    float em = 0.0f;
    bool hit = false;
    for (int i = 0; i < steps; ++i) {
        float3 p = f3_mul(dir, t);
        float d = colonies_map(p, &em);
        if (d < 0.03f) {
            hit = true;
            break;
        }
        t += fmaxf(0.08f, d * 0.7f);
        if (t > tmax) break;
    }
    if (hit) {
        float3 p = f3_mul(dir, t);
        float e = 0.04f;
        float ed = 0.0f;
        float3 n = f3_norm(make_float3(
            colonies_map(make_float3(p.x + e, p.y, p.z), &ed) - colonies_map(make_float3(p.x - e, p.y, p.z), &ed),
            colonies_map(make_float3(p.x, p.y + e, p.z), &ed) - colonies_map(make_float3(p.x, p.y - e, p.z), &ed),
            colonies_map(make_float3(p.x, p.y, p.z + e), &ed) - colonies_map(make_float3(p.x, p.y, p.z - e), &ed)
        ));
        float dif = fmaxf(0.0f, f3_dot(n, sun_dir));
        float rim = powf(1.0f - fmaxf(0.0f, -f3_dot(n, dir)), 3.0f);
        float3 metal = make_float3(0.14f, 0.16f, 0.20f);
        col = f3_add(f3_mul(metal, dif * 1.8f + 0.10f), f3_mul(make_float3(0.22f, 0.42f, 0.68f), rim * 0.35f));
        if (em > 0.5f) {
            col = f3_add(col, f3_mul(make_float3(0.35f, 0.85f, 1.15f), params.env_params[2]));
        }
    }
    return f3_mul(col, sky_i);
}

static __noinline__ __device__ float3 sky_color(float3 dir, float3 sun_dir, bool is_camera_ray) {
    float sky_i = fmaxf(0.05f, params.sky_params[0]);
    float haze_k = fmaxf(0.5f, params.sky_params[2]);
    int mode = (int)(params.sky_params[3] + 0.5f);
    if (mode == 5) return sky_elevated(dir, sun_dir, is_camera_ray);
    if (mode == 6) return sky_unprogress(dir, sun_dir, is_camera_ray);
    if (mode == 7) return sky_colonies(dir, sun_dir, is_camera_ray);
    float y = fmaxf(0.0f, dir.y);
    float horizon_band = expf(-fabsf(dir.y) * (6.0f / haze_k));

    if (mode == 8) {
        // Coastal Night: deep Pacific navy-indigo twilight + warm sodium-vapor city horizon + stars
        float3 zenith  = make_float3(0.004f, 0.008f, 0.024f);
        float3 horizon = make_float3(0.032f, 0.042f, 0.075f);
        float t = powf(y, 0.50f);
        float3 col = f3_add(f3_mul(horizon, 1.0f - t), f3_mul(zenith, t));
        // Subtle warm coastal urban sodium glow along the inland horizon
        float inland = 0.55f + 0.45f * fmaxf(0.0f, dir.x);
        col = f3_add(col, f3_mul(make_float3(0.16f, 0.095f, 0.045f), horizon_band * 0.48f * inland));
        if (dir.y < 0.0f) {
            float g = fminf(1.0f, -dir.y * 5.0f);
            col = f3_add(f3_mul(col, 1.0f - g), f3_mul(make_float3(0.006f, 0.009f, 0.015f), g));
        }
        float moon_dot = fmaxf(0.0f, f3_dot(dir, sun_dir));
        float moon_halo = powf(moon_dot, 24.0f) * 0.14f;
        float moon_t = fmaxf(0.0f, fminf(1.0f, (moon_dot - 0.9992f) / 0.0005f));
        float moon_disc = moon_t * moon_t * (3.0f - 2.0f * moon_t) * (is_camera_ray ? 1.8f : 0.45f);
        col = f3_add(col, f3_mul(make_float3(0.76f, 0.84f, 1.0f), moon_halo + moon_disc));
        if (is_camera_ray && dir.y > 0.01f) {
            float star_fade = sky_smoothstep(0.02f, 0.22f, dir.y);
            col = f3_add(col, f3_mul(sky_starfield(dir, 1.15f, 1.25f, 0.0f), star_fade));
        }
        return f3_mul(col, sky_i);
    }

    if (mode == 1) {
        // Golden Sunset: deep indigo zenith -> warm coral/amber horizon + layered sunset clouds
        float3 zenith  = make_float3(0.05f, 0.07f, 0.22f);
        float3 horizon = make_float3(0.85f, 0.36f, 0.18f);
        float t = powf(y, 0.45f);
        float3 col = f3_add(f3_mul(horizon, 1.0f - t), f3_mul(zenith, t));
        float3 haze_col = make_float3(1.0f, 0.62f, 0.28f);
        col = f3_add(col, f3_mul(haze_col, horizon_band * 0.42f));
        float sun_dot = fmaxf(0.0f, f3_dot(dir, sun_dir));
        if (dir.y > 0.015f && params.env_params[0] > 0.01f) {
            float ct = 620.0f / (dir.y + 0.04f);
            float n1 = elevated_noised(dir.x * ct * 0.0007f, dir.z * ct * 0.0007f).x;
            float n2 = elevated_noised(dir.x * ct * 0.0019f + 4.3f, dir.z * ct * 0.0019f - 2.1f).x;
            float thresh = 0.78f - 0.50f * fmaxf(0.0f, fminf(1.0f, params.env_params[0]));
            float cloud = sky_smoothstep(thresh, thresh + 0.26f, n1 * 0.65f + n2 * 0.35f)
                        * sky_smoothstep(0.015f, 0.12f, dir.y) * fminf(1.5f, params.env_params[1]);
            float3 dark_base = make_float3(0.18f, 0.10f, 0.16f);
            float3 lit_rim = make_float3(1.25f, 0.56f, 0.24f);
            float rim = powf(sun_dot, 4.0f) * 0.85f + (1.0f - n2) * 0.35f;
            float3 ccol = f3_add(dark_base, f3_mul(lit_rim, rim));
            col = f3_add(f3_mul(col, 1.0f - cloud * 0.80f), f3_mul(ccol, cloud * 0.80f));
        }
        if (dir.y < 0.0f) {
            float g = fminf(1.0f, -dir.y * 4.0f);
            col = f3_add(f3_mul(col, 1.0f - g), f3_mul(make_float3(0.08f, 0.05f, 0.07f), g));
        }
        float mie = powf(sun_dot, 5.0f) * 0.35f + powf(sun_dot, 48.0f) * 0.65f;
        float disc_t = fmaxf(0.0f, fminf(1.0f, (sun_dot - 0.9985f) / 0.0008f));
        float sun_disc = disc_t * disc_t * (3.0f - 2.0f * disc_t) * (is_camera_ray ? 4.0f : 1.3f);
        col = f3_add(col, f3_mul(make_float3(1.0f, 0.55f, 0.25f), mie + sun_disc));
        return f3_mul(col, sky_i);
    }

    if (mode == 2) {
        // Studio Softbox: clean neutral cyclorama with smooth-edged overhead & rim softboxes
        float3 top = make_float3(0.14f, 0.15f, 0.17f);
        float3 mid = make_float3(0.06f, 0.065f, 0.075f);
        float3 bot = make_float3(0.03f, 0.032f, 0.036f);
        float3 col = (dir.y >= 0.0f)
            ? f3_add(f3_mul(mid, 1.0f - y), f3_mul(top, y))
            : f3_add(f3_mul(mid, 1.0f + dir.y), f3_mul(bot, -dir.y));
        float soft = fmaxf(0.02f, params.env_params[1]);
        float box_top = sky_smoothstep(0.68f - soft, 0.68f + soft, dir.y)
                      * (1.0f - sky_smoothstep(0.45f - soft, 0.45f + soft, fabsf(dir.x)))
                      * (1.0f - sky_smoothstep(0.45f - soft, 0.45f + soft, fabsf(dir.z)));
        float box_l = sky_smoothstep(0.62f - soft, 0.62f + soft, -dir.x)
                    * (1.0f - sky_smoothstep(0.32f - soft, 0.32f + soft, fabsf(dir.y - 0.18f)))
                    * (1.0f - sky_smoothstep(0.48f - soft, 0.48f + soft, fabsf(dir.z)));
        float box_r = sky_smoothstep(0.62f - soft, 0.62f + soft, dir.x)
                    * (1.0f - sky_smoothstep(0.28f - soft, 0.28f + soft, fabsf(dir.y - 0.12f)))
                    * (1.0f - sky_smoothstep(0.44f - soft, 0.44f + soft, fabsf(dir.z)));
        float panel_gain = (is_camera_ray ? 1.15f : 2.2f) * params.env_params[0];
        col = f3_add(col, f3_mul(make_float3(0.95f, 0.97f, 1.0f), box_top * panel_gain));
        col = f3_add(col, f3_mul(make_float3(0.65f, 0.82f, 1.0f), box_l * panel_gain * 0.85f));
        col = f3_add(col, f3_mul(make_float3(1.0f, 0.86f, 0.70f), box_r * panel_gain * 0.75f));
        return f3_mul(col, sky_i);
    }

    if (mode == 3) {
        // Deep Nebula: domain-warped 3D FBM cosmic clouds + dark dust lanes + cube-projected PSF starfield
        float neb_dens = params.env_params[0];
        float neb_cont = params.env_params[1];
        float neb_dust = params.env_params[2];
        float neb_scale = fmaxf(0.4f, params.env_params[3]);
        float star_dens = params.env_params[4];
        float star_bright = params.env_params[5];

        float3 p = f3_mul(dir, neb_scale);
        float band = expf(-fabsf(dir.y * 0.82f + dir.x * 0.48f - 0.08f) * 2.1f);
        float warp_x = sky_fbm3(f3_add(p, make_float3(1.7f, 9.2f, 3.1f)), 4);
        float warp_y = sky_fbm3(f3_add(p, make_float3(8.3f, 2.8f, 6.4f)), 4);
        float warp_z = sky_fbm3(f3_add(p, make_float3(4.5f, 6.1f, 0.9f)), 4);
        float3 q = f3_add(p, f3_mul(make_float3(warp_x - 0.5f, warp_y - 0.5f, warp_z - 0.5f), 2.4f));

        float f_base = sky_fbm3(q, 5);
        float f_ridge = 1.0f - fabsf(sky_fbm3(f3_mul(q, 1.65f), 4) * 2.0f - 1.0f);
        f_ridge = powf(fmaxf(0.0f, f_ridge), 2.2f);

        float cloud = fmaxf(0.0f, (f_base * 0.72f + f_ridge * 0.38f) * (0.45f + 0.75f * band) - 0.24f);
        cloud = powf(fminf(1.8f, cloud * neb_dens), neb_cont);

        float dust_raw = sky_fbm3(f3_add(f3_mul(q, 1.85f), make_float3(12.4f, -3.7f, 5.2f)), 4);
        float dust = sky_smoothstep(0.44f, 0.74f, dust_raw) * neb_dust * (0.35f + 0.65f * band);

        float3 space = make_float3(0.002f, 0.003f, 0.008f);
        float3 c_violet = make_float3(0.22f, 0.06f, 0.42f);
        float3 c_magenta = make_float3(0.68f, 0.12f, 0.36f);
        float3 c_cyan = make_float3(0.06f, 0.56f, 0.74f);
        float3 c_core = make_float3(1.10f, 0.72f, 0.42f);

        float3 gas = f3_mul(c_violet, cloud * 0.85f);
        gas = f3_add(gas, f3_mul(c_magenta, powf(fmaxf(0.0f, f_base - 0.32f) * 2.1f, 1.5f) * neb_dens * 0.65f));
        gas = f3_add(gas, f3_mul(c_cyan, f_ridge * cloud * 0.95f));
        gas = f3_add(gas, f3_mul(c_core, powf(f_ridge, 2.4f) * band * neb_dens * 0.55f));

        float atten = expf(-dust * 2.4f);
        float3 col = f3_add(space, f3_mul(gas, atten));
        if (is_camera_ray) {
            col = f3_add(col, sky_starfield(dir, star_dens, star_bright, dust * 2.6f));
        }
        return f3_mul(col, sky_i);
    }

    if (mode == 4) {
        // Cyber Grid: dark slate/indigo gradient + anti-aliased spherical lat/lon wireframe grid
        float3 zenith = make_float3(0.015f, 0.025f, 0.07f);
        float3 horiz  = make_float3(0.045f, 0.09f, 0.18f);
        float3 col = f3_add(f3_mul(horiz, 1.0f - powf(y, 0.6f)), f3_mul(zenith, powf(y, 0.6f)));
        float freq = fmaxf(4.0f, params.env_params[1]);
        float lon = atan2f(dir.z, dir.x) * (freq / 3.14159265f);
        float lat = asinf(fmaxf(-1.0f, fminf(1.0f, dir.y))) * (freq / 3.14159265f);
        float gx = fabsf(lon - floorf(lon + 0.5f));
        float gy = fabsf(lat - floorf(lat + 0.5f));
        float cos_lat = fmaxf(0.15f, sqrtf(fmaxf(0.0f, 1.0f - dir.y * dir.y)));
        float line = fmaxf(
            1.0f - sky_smoothstep(0.015f / cos_lat, 0.055f / cos_lat, gx),
            1.0f - sky_smoothstep(0.015f, 0.055f, gy)
        );
        float3 grid_col = make_float3(0.12f, 0.68f, 0.95f);
        col = f3_add(col, f3_mul(grid_col, line * (is_camera_ray ? 0.55f : 0.30f) * params.env_params[0]));
        col = f3_add(col, f3_mul(make_float3(0.10f, 0.45f, 0.75f), horizon_band * 0.25f));
        return f3_mul(col, sky_i);
    }

    // Mode 0: Balanced Rayleigh zenith-to-horizon gradient + cirrus/cumulus clouds
    float3 zenith  = make_float3(0.08f, 0.21f, 0.52f);
    float3 horizon = make_float3(0.46f, 0.59f, 0.76f);
    float3 col = f3_add(f3_mul(horizon, 1.0f - powf(y, 0.55f)), f3_mul(zenith, powf(y, 0.55f)));

    // Warm horizon haze
    float3 haze_col = make_float3(0.72f, 0.68f, 0.62f);
    col = f3_add(col, f3_mul(haze_col, horizon_band * 0.22f));

    float sun_dot = fmaxf(0.0f, f3_dot(dir, sun_dir));
    if (dir.y > 0.015f && params.env_params[0] > 0.01f) {
        float ct = 560.0f / (dir.y + 0.04f);
        float n1 = elevated_noised(dir.x * ct * 0.0008f, dir.z * ct * 0.0008f).x;
        float n2 = elevated_noised(dir.x * ct * 0.0021f + 2.7f, dir.z * ct * 0.0021f - 1.3f).x;
        float thresh = 0.78f - 0.50f * fmaxf(0.0f, fminf(1.0f, params.env_params[0]));
        float cloud = sky_smoothstep(thresh, thresh + 0.25f, n1 * 0.65f + n2 * 0.35f)
                    * sky_smoothstep(0.015f, 0.14f, dir.y) * fminf(1.5f, params.env_params[1]);
        float3 cloud_col = f3_add(make_float3(0.88f, 0.91f, 0.96f), f3_mul(make_float3(0.30f, 0.24f, 0.16f), powf(sun_dot, 8.0f)));
        col = f3_add(f3_mul(col, 1.0f - cloud * 0.72f), f3_mul(cloud_col, cloud * 0.72f));
    }

    // Below-horizon ground bounce
    if (dir.y < 0.0f) {
        float g = fminf(1.0f, -dir.y * 4.0f);
        float3 ground_tint = make_float3(0.11f, 0.12f, 0.14f);
        col = f3_add(f3_mul(col, 1.0f - g), f3_mul(ground_tint, g));
    }

    // Mie forward scattering + bounded sun disc (prevents fireflies & bounce blowup)
    float mie = powf(sun_dot, 8.0f) * 0.16f + powf(sun_dot, 64.0f) * 0.45f;
    float disc_t = fmaxf(0.0f, fminf(1.0f, (sun_dot - 0.9990f) / 0.0006f));
    float sun_disc = disc_t * disc_t * (3.0f - 2.0f * disc_t) * (is_camera_ray ? 3.5f : 1.1f);
    float3 sun_tint = make_float3(1.0f, 0.90f, 0.74f);
    col = f3_add(col, f3_mul(sun_tint, mie + sun_disc));
    return f3_mul(col, sky_i);
}

// --- Render-Time Analytical CSG Helpers & Procedural Urban Shading ---

static __forceinline__ __device__ CsgSceneView make_csg_view() {
    return csg_make_scene_view(
        params.csg_compounds,
        params.csg_ops,
        params.csg_prims,
        params.csg_aux_planes,
        params.csg_guard_aabbs,
        params.csg_repeats,
        params.csg_materials,
        params.csg_instances,
        params.csg_compound_count,
        params.csg_material_count,
        params.csg_instance_count
    );
}

// Perceptually distinct Cityscapes-inspired palette matching `csg_core::ast::semantic` (0..19).
static __forceinline__ __device__ float3 semantic_aov_color(unsigned int sem_id) {
    switch (sem_id) {
        case 1u:  return make_float3(0.50f, 0.25f, 0.50f); // ROAD
        case 2u:  return make_float3(0.96f, 0.14f, 0.91f); // SIDEWALK
        case 3u:  return make_float3(0.76f, 0.60f, 0.76f); // CURB
        case 4u:  return make_float3(0.28f, 0.28f, 0.34f); // BUILDING
        case 5u:  return make_float3(0.32f, 0.76f, 0.96f); // WINDOW
        case 6u:  return make_float3(0.60f, 0.60f, 0.60f); // POLE
        case 7u:  return make_float3(0.98f, 0.67f, 0.12f); // TRAFFIC_LIGHT
        case 8u:  return make_float3(0.86f, 0.86f, 0.00f); // TRAFFIC_SIGN
        case 9u:  return make_float3(0.42f, 0.56f, 0.14f); // FURNITURE
        case 10u: return make_float3(0.00f, 0.32f, 0.88f); // VEHICLE
        case 11u: return make_float3(0.12f, 0.16f, 0.45f); // WHEEL
        case 12u: return make_float3(0.90f, 0.42f, 0.14f); // INDUSTRIAL
        case 13u: return make_float3(0.95f, 0.58f, 0.22f); // PIPE
        case 14u: return make_float3(0.86f, 0.24f, 0.28f); // TANK
        case 15u: return make_float3(0.06f, 0.46f, 0.78f); // WATER
        case 16u: return make_float3(0.78f, 0.68f, 0.46f); // GROUND / SAND
        case 17u: return make_float3(0.64f, 0.48f, 0.40f); // BARRIER
        case 18u: return make_float3(0.86f, 0.08f, 0.24f); // PEDESTRIAN
        case 19u: return make_float3(0.96f, 0.48f, 0.12f); // ANIMAL
        default:  return make_float3(0.08f, 0.08f, 0.10f); // UNKNOWN / SKY
    }
}

static_assert(__builtin_offsetof(CsgGpuInstance, xform) == 0, "CsgGpuInstance::xform offset");
static_assert(__builtin_offsetof(CsgGpuInstance, compound_id) == 48, "CsgGpuInstance::compound_id offset");
static_assert(__builtin_offsetof(CsgGpuInstance, user_id) == 52, "CsgGpuInstance::user_id offset");
static_assert(__builtin_offsetof(CsgGpuInstance, mat_override) == 56, "CsgGpuInstance::mat_override offset");
static_assert(__builtin_offsetof(CsgGpuInstance, flags) == 60, "CsgGpuInstance::flags offset");
static_assert(__builtin_offsetof(CsgGpuMaterial, roughness) == 12, "CsgGpuMaterial::roughness offset");
static_assert(__builtin_offsetof(CsgGpuMaterial, metallic) == 28, "CsgGpuMaterial::metallic offset");
static_assert(__builtin_offsetof(CsgGpuMaterial, ior) == 44, "CsgGpuMaterial::ior offset");
static_assert(__builtin_offsetof(CsgGpuMaterial, procedural_kind) == 56, "CsgGpuMaterial::procedural_kind offset");
static_assert(__builtin_offsetof(CsgGpuMaterial, flags) == 60, "CsgGpuMaterial::flags offset");

// Loads (compound_id, user_id, mat_override, flags) in a single 16-byte read-only cache load.
static __forceinline__ __device__ uint4 load_csg_instance_meta(unsigned int inst_idx) {
    const uint4* base = reinterpret_cast<const uint4*>(params.csg_instances);
    return __ldg(base + ((size_t)inst_idx * 4u + 3u));
}

// Exact inverse of a general row-major 3x4 affine object-to-world transform loaded via __ldg
// (handles non-uniform bone/instance scale + rotation + translation without assuming orthonormality).
static __forceinline__ __device__ bool csg_inv_xform_pt_ldg(const float* __restrict__ m, float3 p_world, float3& p_obj) {
    const float4* rows = reinterpret_cast<const float4*>(m);
    float4 r0 = __ldg(rows + 0);
    float4 r1 = __ldg(rows + 1);
    float4 r2 = __ldg(rows + 2);

    float c00 = r1.y * r2.z - r1.z * r2.y;
    float c01 = r1.z * r2.x - r1.x * r2.z;
    float c02 = r1.x * r2.y - r1.y * r2.x;
    float det = r0.x * c00 + r0.y * c01 + r0.z * c02;
    if (fabsf(det) <= 1e-12f) {
        return false;
    }
    float inv_det = 1.0f / det;

    float c10 = r0.z * r2.y - r0.y * r2.z;
    float c11 = r0.x * r2.z - r0.z * r2.x;
    float c12 = r0.y * r2.x - r0.x * r2.y;

    float c20 = r0.y * r1.z - r0.z * r1.y;
    float c21 = r0.z * r1.x - r0.x * r1.z;
    float c22 = r0.x * r1.y - r0.y * r1.x;

    float qx = p_world.x - r0.w;
    float qy = p_world.y - r1.w;
    float qz = p_world.z - r2.w;

    p_obj = make_float3(
        (c00 * qx + c10 * qy + c20 * qz) * inv_det,
        (c01 * qx + c11 * qy + c21 * qz) * inv_det,
        (c02 * qx + c12 * qy + c22 * qz) * inv_det
    );
    return true;
}

static __forceinline__ __device__ float3 csg_xform_pt_ldg(const float* __restrict__ m, float3 p) {
    const float4* rows = reinterpret_cast<const float4*>(m);
    float4 r0 = __ldg(rows + 0);
    float4 r1 = __ldg(rows + 1);
    float4 r2 = __ldg(rows + 2);
    return make_float3(
        r0.x * p.x + r0.y * p.y + r0.z * p.z + r0.w,
        r1.x * p.x + r1.y * p.y + r1.z * p.z + r1.w,
        r2.x * p.x + r2.y * p.y + r2.z * p.z + r2.w
    );
}

static __forceinline__ __device__ float3 instance_aov_color(unsigned int inst_id) {
    if (inst_id == 0u) return make_float3(0.02f, 0.02f, 0.04f);
    unsigned int h = pcg_hash(inst_id * 2654435761u ^ 0x9E3779B9u);
    float hue = (float)(h & 0xFFFFu) * (1.0f / 65536.0f);
    float sat = 0.68f + 0.28f * ((float)((h >> 16u) & 0xFFu) * (1.0f / 255.0f));
    float val = 0.65f + 0.32f * ((float)((h >> 24u) & 0xFFu) * (1.0f / 255.0f));
    float h6 = fmodf(hue * 6.0f, 6.0f);
    float r = fminf(1.0f, fmaxf(0.0f, fabsf(h6 - 3.0f) - 1.0f));
    float g = fminf(1.0f, fmaxf(0.0f, 2.0f - fabsf(h6 - 2.0f)));
    float b = fminf(1.0f, fmaxf(0.0f, 2.0f - fabsf(h6 - 4.0f)));
    return make_float3(
        val * (1.0f - sat + sat * r),
        val * (1.0f - sat + sat * g),
        val * (1.0f - sat + sat * b)
    );
}

static __forceinline__ __device__ CsgGpuMaterial load_csg_material(unsigned int mat_id) {
    CsgGpuMaterial m;
    if (params.csg_materials != 0ull && mat_id < params.csg_material_count) {
        const float4* v = reinterpret_cast<const float4*>(params.csg_materials) + ((size_t)mat_id * 4u);
        float4 v0 = __ldg(v + 0);
        float4 v1 = __ldg(v + 1);
        float4 v2 = __ldg(v + 2);
        float4 v3 = __ldg(v + 3);
        m.albedo[0] = v0.x; m.albedo[1] = v0.y; m.albedo[2] = v0.z;
        m.roughness = v0.w;
        m.emission[0] = v1.x; m.emission[1] = v1.y; m.emission[2] = v1.z;
        m.metallic = v1.w;
        m.absorption[0] = v2.x; m.absorption[1] = v2.y; m.absorption[2] = v2.z;
        m.ior = v2.w;
        m.clearcoat = v3.x;
        m.clearcoat_roughness = v3.y;
        m.procedural_kind = __float_as_uint(v3.z);
        m.flags = __float_as_uint(v3.w);
        return m;
    }
    m.albedo[0] = 0.45f; m.albedo[1] = 0.45f; m.albedo[2] = 0.45f;
    m.roughness = 0.55f;
    m.emission[0] = 0.0f; m.emission[1] = 0.0f; m.emission[2] = 0.0f;
    m.metallic = 0.0f;
    m.absorption[0] = 0.0f; m.absorption[1] = 0.0f; m.absorption[2] = 0.0f;
    m.ior = 1.5f;
    m.clearcoat = 0.0f;
    m.clearcoat_roughness = 0.08f;
    m.procedural_kind = CSG_PROC_NONE;
    m.flags = 0u;
    return m;
}

struct CsgProcResult {
    float3 n;
    float3 base_col;
    float3 emission;
    float3 absorption;
    float rough;
    float metallic;
    float clearcoat;
    float cc_rough;
    float ior;
    unsigned int mat_flags;
    unsigned int proc_kind;
};

static __noinline__ __device__ CsgProcResult eval_csg_procedural(
    unsigned int mat_id,
    unsigned int sem_id,
    unsigned int part_id,
    float3 hit_pos,
    float3 n
) {
    CsgGpuMaterial mat = load_csg_material(mat_id);
    float3 base_col = make_float3(mat.albedo[0], mat.albedo[1], mat.albedo[2]);
    float rough = fmaxf(0.02f, fminf(0.98f, mat.roughness));
    float metallic = fmaxf(0.0f, fminf(1.0f, mat.metallic));
    float clearcoat = fmaxf(0.0f, fminf(1.0f, mat.clearcoat));
    float cc_rough = fmaxf(0.02f, fminf(0.60f, mat.clearcoat_roughness));
    float3 emission = make_float3(mat.emission[0], mat.emission[1], mat.emission[2]);

    float wet = fmaxf(0.0f, fminf(1.0f, params.csg_wetness));
    if ((mat.flags & CSG_MAT_FLAG_WET) != 0u) {
        wet = fmaxf(wet, 0.35f);
    }

    // RoadTerrainTile packs analytical curve-aligned (s, lat) into part_id:
    //   s   = (part_id & 0xFFFFu) / 1024.0f              (meters along road centerline)
    //   lat = ((part_id >> 16u) & 0xFFFFu) / 1024.0f - 32.0f  (signed lateral offset in meters)
    bool has_road_uv = (part_id != 0u) && (sem_id == 1u || sem_id == 2u || sem_id == 3u);
    float road_s = (float)(part_id & 0xFFFFu) * (1.0f / 1024.0f);
    float road_lat = (float)((part_id >> 16u) & 0xFFFFu) * (1.0f / 1024.0f) - 32.0f;

    switch (mat.procedural_kind) {
        case CSG_PROC_ASPHALT: {
            float grain = value_noise2(hit_pos.x * 22.0f, hit_pos.z * 22.0f);
            float fine_chip = value_noise2(hit_pos.x * 68.0f + 5.7f, hit_pos.z * 68.0f - 2.3f);
            float coarse = fbm2(hit_pos.x * 0.45f, hit_pos.z * 0.45f);
            float tire_wear = 0.0f;
            float gutter_dark = 0.0f;
            if (has_road_uv) {
                // Curve-aligned dual wheel-track darkening (+/-0.78 m from 3.6 m lane centers)
                // and subtle gutter grime/moisture darkening along curb edges.
                float abs_lat = fabsf(road_lat);
                float lane_off = fabsf(fmodf(abs_lat, 3.6f) - 1.8f);
                float track_d = fabsf(lane_off - 0.78f);
                float track_band = 1.0f - sky_smoothstep(0.14f, 0.48f, track_d);
                float s_var = 0.85f + 0.15f * sinf(road_s * 0.35f + coarse * 3.0f);
                tire_wear = track_band * s_var;
                float edge_band = sky_smoothstep(0.0f, 0.55f, fmodf(abs_lat, 3.6f));
                float curb_gutter = (1.0f - edge_band) * sky_smoothstep(2.2f, 3.6f, abs_lat);
                gutter_dark = 0.22f * curb_gutter;
            } else {
                // Fallback world-aligned dual-wheel tire-track wear darkening for mesh/prism roads.
                float wheel_pair = 0.5f + 0.5f * cosf(hit_pos.x * 3.4f);
                float lane_band = 0.5f + 0.5f * cosf(hit_pos.x * 1.70f);
                tire_wear = lane_band * (0.45f + 0.55f * wheel_pair);
            }
            float tar_crack = sky_smoothstep(0.76f, 0.84f, fbm2(hit_pos.x * 0.25f + 7.1f, hit_pos.z * 0.25f - 3.4f));
            float agg = 0.78f + 0.22f * grain + 0.14f * fine_chip;
            float shade = agg * (0.86f + 0.24f * coarse) * (1.0f - 0.22f * tire_wear) * (1.0f - gutter_dark);
            base_col = f3_mul(base_col, shade * (1.0f - 0.38f * tar_crack));
            rough = fmaxf(0.04f, fminf(0.95f, rough + (grain - 0.5f) * 0.16f + (fine_chip - 0.5f) * 0.08f - 0.12f * tire_wear - 0.08f * gutter_dark));
            float puddle = 0.0f;
            if (wet > 0.01f) {
                float p_noise = fbm2(hit_pos.x * 0.14f + 2.3f, hit_pos.z * 0.14f - 5.1f);
                puddle = sky_smoothstep(0.68f - 0.52f * wet, 0.84f - 0.44f * wet, p_noise) * wet;
                float damp = wet * (0.45f + 0.55f * puddle);
                base_col = f3_mul(base_col, 1.0f - 0.52f * damp);
                rough = rough * (1.0f - 0.85f * puddle) + 0.035f * puddle;
                clearcoat = fmaxf(clearcoat, 0.25f * wet + 0.70f * puddle);
                cc_rough = 0.03f + 0.08f * (1.0f - puddle);
            }
            // Tactile asphalt aggregate micro-normal relief (suppressed inside flat mirror puddles).
            float bump_k = 0.034f * (1.0f - puddle);
            if (bump_k > 0.002f) {
                float gx = value_noise2((hit_pos.x + 0.018f) * 24.0f, hit_pos.z * 24.0f) - grain;
                float gz = value_noise2(hit_pos.x * 24.0f, (hit_pos.z + 0.018f) * 24.0f) - grain;
                n = f3_norm(make_float3(n.x - gx * bump_k, n.y, n.z - gz * bump_k));
            }
            break;
        }
        case CSG_PROC_LANE_PAINT: {
            if ((mat.flags & CSG_MAT_FLAG_LANE_DASHED) != 0u) {
                float dash = fmodf(fabsf(hit_pos.z), 9.0f);
                if (dash > 3.2f) {
                    base_col = make_float3(0.07f, 0.072f, 0.078f);
                    rough = fmaxf(0.06f, 0.55f - 0.35f * wet);
                    break;
                }
            }
            float wear = value_noise2(hit_pos.x * 16.0f, hit_pos.z * 16.0f);
            float chip = sky_smoothstep(0.18f, 0.32f, wear);
            float3 asphalt_under = make_float3(0.07f, 0.072f, 0.078f);
            base_col = f3_add(f3_mul(asphalt_under, 1.0f - chip), f3_mul(base_col, chip));
            rough = fmaxf(0.10f, rough - 0.14f * wet);
            break;
        }
        case CSG_PROC_CONCRETE: {
            float lum = base_col.x * 0.3f + base_col.y * 0.59f + base_col.z * 0.11f;
            // Crushed gravel / rail ballast / tank dike aggregate (`COASTAL_ROCK = 38` or dark berms).
            if (mat_id == 38u || (sem_id == 17u && lum < 0.32f)) {
                float u = (fabsf(n.y) > 0.45f) ? hit_pos.x : (hit_pos.x + hit_pos.z);
                float v = (fabsf(n.y) > 0.45f) ? hit_pos.z : hit_pos.y;
                float clump = fbm2(u * 1.1f + 3.1f, v * 1.1f - 2.4f);
                float stone_m = value_noise2(u * 16.0f, v * 16.0f);
                float stone_f = value_noise2(u * 44.0f + 11.3f, v * 44.0f - 7.9f);
                float speck = value_noise2(u * 96.0f - 4.2f, v * 96.0f + 8.6f);
                // Multi-octave angular stone contrast (basalt darks + crushed granite flecks).
                float rock_tone = (0.68f + 0.34f * clump) * (0.76f + 0.34f * stone_m) * (0.86f + 0.26f * stone_f);
                float granite_fleck = sky_smoothstep(0.74f, 0.90f, speck);
                float3 stone_col = f3_mul(base_col, rock_tone);
                base_col = f3_add(stone_col, make_float3(0.10f * granite_fleck, 0.095f * granite_fleck, 0.09f * granite_fleck));
                rough = fmaxf(0.80f, fminf(0.96f, 0.88f + (0.5f - stone_m) * 0.14f));
                // Micro-normal perturbation for faceted gravel / ballast relief.
                float eps_g = 0.022f;
                float gx = value_noise2((u + eps_g) * 28.0f, v * 28.0f) - value_noise2((u - eps_g) * 28.0f, v * 28.0f);
                float gz = value_noise2(u * 28.0f, (v + eps_g) * 28.0f) - value_noise2(u * 28.0f, (v - eps_g) * 28.0f);
                if (fabsf(n.y) > 0.45f) {
                    n = f3_norm(make_float3(n.x - gx * 0.22f, n.y, n.z - gz * 0.22f));
                } else {
                    float horiz = sqrtf(n.x * n.x + n.z * n.z);
                    float tx = (horiz > 1e-4f) ? (-n.z / horiz) : 1.0f;
                    float tz = (horiz > 1e-4f) ? ( n.x / horiz) : 0.0f;
                    n = f3_norm(make_float3(n.x - tx * gx * 0.18f, n.y - gz * 0.18f, n.z - tz * gx * 0.18f));
                }
                if (wet > 0.05f) {
                    base_col = f3_mul(base_col, 1.0f - 0.28f * wet);
                    rough = fmaxf(0.42f, rough - 0.24f * wet);
                }
                break;
            }

            float horiz = sqrtf(n.x * n.x + n.z * n.z);
            float tx = (horiz > 1e-4f) ? (-n.z / horiz) : 1.0f;
            float tz = (horiz > 1e-4f) ? ( n.x / horiz) : 0.0f;
            bool is_horiz = fabsf(n.y) > 0.5f;
            bool use_curved_uv = has_road_uv && is_horiz;
            float sx = use_curved_uv ? road_s : (is_horiz ? hit_pos.x : (hit_pos.x * tx + hit_pos.z * tz));
            float sy = use_curved_uv ? road_lat : (is_horiz ? hit_pos.z : hit_pos.y);
            float grime = fbm2(sx * 0.35f + 4.1f, sy * 0.25f - 1.9f);
            float micro = value_noise2(sx * 14.0f, sy * 14.0f);
            float fine = value_noise2(sx * 46.0f + 2.1f, sy * 46.0f - 3.7f);
            float vein = fbm2(sx * 1.4f + sy * 0.4f, sy * 2.8f - sx * 0.3f);
            // Expansion-joint slab grid: 1.5 m curved sidewalk/curb slabs on RoadTerrainTile,
            // 2.5 m horizontal pavement slabs, or 3.0x2.4 m architectural formwork.
            float pitch_x = use_curved_uv ? 1.5f : (is_horiz ? 2.5f : 3.0f);
            float pitch_y = use_curved_uv ? 1.5f : (is_horiz ? 2.5f : 2.4f);
            float cell_x = floorf(sx / pitch_x);
            float cell_y = floorf(sy / pitch_y);
            float fx = (sx / pitch_x) - cell_x;
            float fy = (sy / pitch_y) - cell_y;
            float slab_hash = hash21(cell_x, cell_y);
            float joint_w = use_curved_uv ? 0.008f : 0.014f;
            bool in_joint = (fx < joint_w || fx > (1.0f - joint_w) || fy < joint_w || fy > (1.0f - joint_w));
            float curb_darken = (has_road_uv && sem_id == 3u) ? 0.88f : 1.0f;
            float seam = (in_joint ? 0.64f : (0.95f + 0.10f * slab_hash)) * curb_darken;
            // Light travertine/Marina stucco keeps crisp veining; dark concrete gains weathering.
            float grime_amp = (lum > 0.55f) ? 0.14f : 0.28f;
            base_col = f3_mul(
                base_col,
                ((1.0f - 0.5f * grime_amp) + grime_amp * grime)
                    * (0.90f + 0.12f * micro + 0.06f * fine)
                    * (0.95f + 0.10f * vein)
                    * seam
            );
            rough = fmaxf(0.18f, fminf(0.96f, rough + (0.5f - micro) * 0.10f + (in_joint ? 0.12f : 0.0f)));
            // Subtle expansion-joint bevel normal on horizontal concrete slabs.
            if (is_horiz && !in_joint) {
                float bx = (fx < 0.032f) ? -1.0f : ((fx > 0.968f) ? 1.0f : 0.0f);
                float bz = (fy < 0.032f) ? -1.0f : ((fy > 0.968f) ? 1.0f : 0.0f);
                if (bx != 0.0f || bz != 0.0f) {
                    n = f3_norm(make_float3(n.x + bx * 0.05f, n.y, n.z + bz * 0.05f));
                }
            }
            if (wet > 0.05f && is_horiz) {
                float wmask = sky_smoothstep(0.42f, 0.72f, grime) * wet;
                base_col = f3_mul(base_col, 1.0f - 0.30f * wmask);
                rough = fmaxf(0.12f, rough * (1.0f - 0.55f * wmask));
                clearcoat = fmaxf(clearcoat, 0.65f * wmask);
                cc_rough = fminf(cc_rough, 0.06f);
            }
            break;
        }
        case CSG_PROC_BRICK: {
            if (n.y > 0.7f) {
                // Horizontal roof faces of brick buildings shade as a weathered tar/gravel/EPDM
                // flat roof membrane instead of world-axis-aligned brick stripes.
                float gravel = value_noise2(hit_pos.x * 26.0f, hit_pos.z * 26.0f);
                float patch_n = fbm2(hit_pos.x * 0.24f + 2.3f, hit_pos.z * 0.24f - 1.7f);
                float seam_x = fabsf(fmodf(fabsf(hit_pos.x + hit_pos.z * 0.25f), 2.4f) - 1.2f);
                float seam_z = fabsf(fmodf(fabsf(hit_pos.z - hit_pos.x * 0.25f), 4.8f) - 2.4f);
                float tar_seam = (seam_x > 1.17f || seam_z > 2.37f) ? 0.72f : 1.0f;
                float tone = (0.78f + 0.30f * patch_n) * (0.90f + 0.20f * gravel) * tar_seam;
                base_col = make_float3(0.11f * tone, 0.115f * tone, 0.125f * tone);
                rough = fmaxf(0.76f, fminf(0.92f, 0.84f + (0.5f - gravel) * 0.12f));
                if (wet > 0.05f) {
                    float wmask = sky_smoothstep(0.45f, 0.75f, patch_n) * wet;
                    base_col = f3_mul(base_col, 1.0f - 0.32f * wmask);
                    rough = fmaxf(0.18f, rough * (1.0f - 0.52f * wmask));
                    clearcoat = fmaxf(clearcoat, 0.55f * wmask);
                    cc_rough = fminf(cc_rough, 0.08f);
                }
                break;
            }
            // Exact horizontal wall-tangent projection preserves undistorted brick proportions
            // on arbitrary-angle OSM streets (e.g. 45-degree SF SoMa Boardman Pl & curved arcs).
            float horiz = sqrtf(n.x * n.x + n.z * n.z);
            float tx = (horiz > 1e-4f) ? (-n.z / horiz) : 1.0f;
            float tz = (horiz > 1e-4f) ? ( n.x / horiz) : 0.0f;
            float u = hit_pos.x * tx + hit_pos.z * tz;
            float v = hit_pos.y;
            float row = floorf(v * 11.5f);
            float col_u = u * 4.4f + 0.5f * fmodf(fabsf(row), 2.0f);
            float fx = col_u - floorf(col_u);
            float fy = (v * 11.5f) - row;
            bool mortar = (fx < 0.065f || fy < 0.11f);
            float weather = fbm2(u * 0.28f + 1.7f, v * 0.22f);
            float street_soot = 0.84f + 0.16f * sky_smoothstep(0.0f, 3.5f, v);
            if (mortar) {
                base_col = f3_mul(make_float3(0.44f, 0.42f, 0.39f), (0.85f + 0.24f * weather) * street_soot);
                rough = 0.92f;
            } else {
                float b_hash = hash21(floorf(col_u), row);
                float warm_shift = hash21(floorf(col_u) + 37.0f, row + 19.0f);
                float grain = value_noise2(u * 28.0f, v * 28.0f);
                float3 b_tint = make_float3(
                    0.82f + 0.30f * b_hash + 0.08f * warm_shift,
                    0.79f + 0.28f * b_hash,
                    0.76f + 0.24f * b_hash - 0.04f * warm_shift
                );
                base_col = f3_mul(f3_mul3(base_col, b_tint), (0.88f + 0.20f * weather) * (0.93f + 0.14f * grain) * street_soot);
                rough = fmaxf(0.42f, fminf(0.95f, rough + (b_hash - 0.5f) * 0.16f));
                // Subtle mortar-edge normal bevel for grazing sunlight across SoMa brick facades.
                float edge_u = (fx < 0.12f) ? -1.0f : ((fx > 0.94f) ? 1.0f : 0.0f);
                float edge_v = (fy < 0.17f) ? -1.0f : ((fy > 0.93f) ? 1.0f : 0.0f);
                if (edge_u != 0.0f || edge_v != 0.0f) {
                    n = f3_norm(make_float3(
                        n.x + tx * edge_u * 0.06f,
                        n.y + edge_v * 0.06f,
                        n.z + tz * edge_u * 0.06f
                    ));
                }
            }
            break;
        }
        case CSG_PROC_WATER: {
            float wt = params.csg_traffic_time;
            // Distinguish bright turquoise swimming pools from deep Pacific ocean / marina basins.
            bool is_pool = (base_col.y > 0.25f && base_col.z > 0.30f);
            float scale = is_pool ? 1.15f : 0.35f;
            float wx = hit_pos.x * scale;
            float wz = hit_pos.z * scale;
            float eps = 0.08f;
            float h0 = sinf(wx * 2.4f + wt * 1.4f) * 0.35f
                     + cosf(wz * 1.9f - wt * 1.1f) * 0.25f
                     + fbm2(wx * 1.6f + wt * 0.25f, wz * 1.6f - wt * 0.18f) * 0.45f;
            float hx = sinf((wx + eps) * 2.4f + wt * 1.4f) * 0.35f
                     + cosf(wz * 1.9f - wt * 1.1f) * 0.25f
                     + fbm2((wx + eps) * 1.6f + wt * 0.25f, wz * 1.6f - wt * 0.18f) * 0.45f;
            float hz = sinf(wx * 2.4f + wt * 1.4f) * 0.35f
                     + cosf((wz + eps) * 1.9f - wt * 1.1f) * 0.25f
                     + fbm2(wx * 1.6f + wt * 0.25f, (wz + eps) * 1.6f - wt * 0.18f) * 0.45f;
            float strength = is_pool ? 0.06f : 0.10f;
            n = f3_norm(make_float3(n.x - (hx - h0) / eps * strength, n.y, n.z - (hz - h0) / eps * strength));
            if (is_pool) {
                float caustic = fmaxf(0.0f, sinf(wx * 4.5f + wt * 1.8f) * cosf(wz * 4.2f - wt * 1.5f));
                base_col = f3_mul(base_col, 0.90f + 0.28f * caustic * caustic);
            }
            rough = is_pool ? 0.02f : 0.035f;
            clearcoat = 0.95f;
            cc_rough = 0.025f;
            break;
        }
        case CSG_PROC_METAL: {
            float horiz = sqrtf(n.x * n.x + n.z * n.z);
            float tx = (horiz > 1e-4f) ? (-n.z / horiz) : 1.0f;
            float tz = (horiz > 1e-4f) ? ( n.x / horiz) : 0.0f;
            float u = (horiz > 1e-4f) ? (hit_pos.x * tx + hit_pos.z * tz) : hit_pos.x;
            float patina = fbm2(u * 0.65f, hit_pos.y * 0.45f + hit_pos.z * 0.15f);
            float brush = value_noise2(hit_pos.y * 16.0f, u * 3.0f);

            if (sem_id == 14u) {
                // Storage tank welded steel plate courses (2.4 m height x 6.0 m staggered plates) + vertical weathering.
                if (fabsf(n.y) < 0.65f) {
                    float course = hit_pos.y * (1.0f / 2.4f);
                    float row_idx = floorf(course);
                    float fy = course - row_idx;
                    float stagger = 0.5f * fmodf(fabsf(row_idx), 2.0f);
                    float col_u = u * (1.0f / 6.0f) + stagger;
                    float col_idx = floorf(col_u);
                    float fx = col_u - col_idx;
                    bool h_weld = (fy < 0.022f || fy > 0.978f);
                    bool v_weld = (fx < 0.010f || fx > 0.990f);
                    float plate_var = 0.94f + 0.11f * hash21(col_idx, row_idx);
                    float streak = fbm2(u * 0.72f + 1.9f, hit_pos.y * 0.09f);
                    float lower_dust = 0.90f + 0.10f * sky_smoothstep(0.0f, 4.5f, hit_pos.y);
                    float weld_k = (h_weld || v_weld) ? 0.76f : 1.0f;
                    base_col = f3_mul(base_col, plate_var * (0.89f + 0.20f * streak) * lower_dust * weld_k);
                    rough = fmaxf(0.12f, fminf(0.92f, rough + (0.5f - streak) * 0.14f + (h_weld ? 0.12f : 0.0f)));
                    if (h_weld) {
                        float wy = (fy < 0.022f) ? -0.09f : 0.09f;
                        n = f3_norm(make_float3(n.x, n.y + wy, n.z));
                    }
                } else {
                    float rx = fabsf(fmodf(fabsf(hit_pos.x), 3.0f) - 1.5f);
                    float rz = fabsf(fmodf(fabsf(hit_pos.z), 6.0f) - 3.0f);
                    float roof_seam = (rx > 1.46f || rz > 2.95f) ? 0.80f : 1.0f;
                    base_col = f3_mul(base_col, (0.86f + 0.22f * patina) * roof_seam);
                    rough = fmaxf(0.22f, fminf(0.90f, rough + (0.5f - brush) * 0.12f));
                }
            } else if ((sem_id == 4u || sem_id == 12u) && fabsf(n.y) < 0.45f) {
                // Corrugated industrial steel cladding (vertical ribs + metallic specular variation).
                float phase = u * 25.132741f; // 4 ribs/m
                float rib = sinf(phase);
                float rib_d = cosf(phase);
                float panel_u = fabsf(fmodf(fabsf(u), 1.2f) - 0.6f);
                float panel_seam = (panel_u > 0.585f) ? 0.76f : 1.0f;
                float girt_y = fabsf(fmodf(fabsf(hit_pos.y), 3.2f) - 1.6f);
                float girt_seam = (girt_y > 1.56f) ? 0.86f : 1.0f;
                float rib_shade = 0.88f + 0.16f * rib;
                base_col = f3_mul(base_col, (0.86f + 0.22f * patina) * rib_shade * panel_seam * girt_seam);
                rough = fmaxf(0.10f, fminf(0.88f, rough + (0.5f - rib * 0.5f) * 0.10f + (0.5f - brush) * 0.08f));
                n = f3_norm(make_float3(n.x + tx * rib_d * 0.14f, n.y, n.z + tz * rib_d * 0.14f));
            } else {
                base_col = f3_mul(base_col, 0.85f + 0.24f * patina);
                rough = fmaxf(0.06f, fminf(0.88f, rough + (0.5f - brush) * 0.14f));
            }
            break;
        }
        case CSG_PROC_GLASS: {
            // Subtle architectural glass flatness micro-warp for realistic sky/building reflections.
            float pane_u = floorf((hit_pos.x + hit_pos.z) * 0.55f);
            float pane_v = floorf(hit_pos.y * 0.35f);
            float h = hash21(pane_u, pane_v) - 0.5f;
            n = f3_norm(make_float3(n.x + h * 0.008f, n.y + h * 0.004f, n.z - h * 0.008f));
            break;
        }
        case CSG_PROC_CAR_PAINT: {
            if ((mat.flags & CSG_MAT_FLAG_CAR_PAINT_FLAKE) != 0u) {
                float flake = value_noise2(hit_pos.x * 95.0f + hit_pos.y * 41.0f, hit_pos.z * 95.0f);
                base_col = f3_mul(base_col, 0.92f + 0.18f * flake);
                rough = fmaxf(0.08f, rough - 0.05f * flake);
            }
            clearcoat = fmaxf(clearcoat, 0.85f);
            cc_rough = fminf(cc_rough, 0.05f);
            break;
        }
        case CSG_PROC_WOOD: {
            float horiz = sqrtf(n.x * n.x + n.z * n.z);
            float u = (fabsf(n.y) > 0.5f) ? hit_pos.x : ((horiz > 1e-4f) ? ((-hit_pos.x * n.z + hit_pos.z * n.x) / horiz) : hit_pos.x);
            float v = (fabsf(n.y) > 0.5f) ? hit_pos.z : hit_pos.y;
            float plank_idx = floorf(u * 6.5f);
            float plank_f = (u * 6.5f) - plank_idx;
            float seam = (plank_f < 0.045f || plank_f > 0.955f) ? 0.68f : 1.0f;
            float p_hash = hash21(plank_idx, floorf(v * 0.4f));
            float ring = sinf((u * 22.0f + v * 2.5f + p_hash * 5.0f) + fbm2(u * 4.5f, v * 2.2f) * 3.8f);
            base_col = f3_mul(base_col, (0.82f + 0.18f * ring) * (0.90f + 0.20f * p_hash) * seam);
            rough = fmaxf(0.18f, fminf(0.85f, rough + (0.5f - p_hash) * 0.10f));
            if (rough < 0.42f) {
                clearcoat = fmaxf(clearcoat, 0.35f);
                cc_rough = 0.16f;
            }
            break;
        }
        case CSG_PROC_SAND: {
            // Wind-aligned coastal dune sand micro-ripples (faded cleanly by camera distance and
            // beach elevation so distant inland bluffs/hillsides never show corduroy striping).
            const float wx_dir = 0.88f;
            const float wz_dir = 0.475f;
            float3 to_cam = f3_sub(hit_pos, make_f3(params.cam_pos));
            float cam_dist = f3_len(to_cam);
            float dist_fade = 1.0f - sky_smoothstep(8.0f, 36.0f, cam_dist);
            float beach_fade = 1.0f - sky_smoothstep(2.5f, 9.5f, hit_pos.y);
            float ripple_w = dist_fade * beach_fade;

            float wind_u = hit_pos.x * wx_dir + hit_pos.z * wz_dir;
            float wind_v = -hit_pos.x * wz_dir + hit_pos.z * wx_dir;
            float warp = fbm2(wind_u * 0.28f + 1.4f, wind_v * 0.20f - 2.1f)
                + 0.45f * fbm2(hit_pos.x * 0.11f - 5.3f, hit_pos.z * 0.11f + 2.9f);
            float phase = wind_u * 4.4f + warp * 3.8f;
            float ripple = sinf(phase);
            float ripple_d = cosf(phase);
            float macro_dune = fbm2(hit_pos.x * 0.032f - 3.2f, hit_pos.z * 0.032f + 5.7f);
            float meso_earth = fbm2(hit_pos.x * 0.14f + 7.1f, hit_pos.z * 0.14f - 4.3f);
            float grit = value_noise2(hit_pos.x * 32.0f, hit_pos.z * 32.0f) * dist_fade + 0.5f * (1.0f - dist_fade);
            float sparkle = sky_smoothstep(0.86f, 0.96f, value_noise2(hit_pos.x * 78.0f + 4.1f, hit_pos.z * 78.0f - 1.7f)) * dist_fade;
            // Subtle darker damp foreshore near sea level (`y < 0.85 m`) vs warm dry upper dunes.
            float foreshore = 1.0f - sky_smoothstep(0.0f, 0.85f, hit_pos.y);
            float3 dune_tint = make_float3(
                0.95f + 0.08f * macro_dune + 0.03f * meso_earth,
                0.93f + 0.06f * macro_dune + 0.03f * meso_earth,
                0.89f + 0.04f * macro_dune + 0.02f * meso_earth
            );
            float shade = (1.0f + 0.038f * ripple * ripple_w)
                * (0.93f + 0.12f * grit)
                * (1.0f - 0.28f * foreshore);
            base_col = f3_mul(f3_mul3(base_col, dune_tint), shade);
            rough = fmaxf(0.32f, fminf(0.95f, rough - 0.22f * foreshore - 0.14f * sparkle));
            if (fabsf(n.y) > 0.35f && ripple_w > 0.001f) {
                float slope = ripple_d * 0.048f * ripple_w * (1.0f - 0.5f * foreshore);
                n = f3_norm(make_float3(n.x - wx_dir * slope, n.y, n.z - wz_dir * slope));
            }
            break;
        }
        default:
            break;
    }

    // Room-to-room warmth variation on illuminated architectural interiors (Marina towers & SoMa offices).
    if ((mat.flags & CSG_MAT_FLAG_EMISSIVE) != 0u
        && (mat.procedural_kind == CSG_PROC_NONE || mat.procedural_kind == CSG_PROC_GLASS)
        && emission.x > 0.15f
        && emission.x < 4.0f) {
        float cell_u = floorf((hit_pos.x + hit_pos.z) * 0.28f);
        float cell_v = floorf(hit_pos.y * 0.30f);
        float room = hash21(cell_u, cell_v);
        float dim = 0.55f + 0.70f * room;
        emission = make_float3(
            emission.x * dim,
            emission.y * dim * (0.92f + 0.14f * room),
            emission.z * dim * (0.85f + 0.26f * room)
        );
    }

    // Pulse red aviation warning beacons atop tall refinery flare stacks & distillation towers.
    if ((mat.flags & CSG_MAT_FLAG_EMISSIVE) != 0u
        && hit_pos.y > 16.0f
        && emission.x > 2.0f * fmaxf(emission.y, 0.05f)
        && emission.x > 2.0f * fmaxf(emission.z, 0.05f))
    {
        float ph = 0.5f + 0.5f * sinf(params.csg_traffic_time * 3.2f + hit_pos.x * 0.13f + hit_pos.z * 0.07f);
        float pulse = 0.20f + 0.80f * ph * ph * ph;
        emission = f3_mul(emission, pulse);
    }

    CsgProcResult res;
    res.n = n;
    res.base_col = base_col;
    res.emission = emission;
    res.absorption = make_float3(mat.absorption[0], mat.absorption[1], mat.absorption[2]);
    res.rough = rough;
    res.metallic = metallic;
    res.clearcoat = clearcoat;
    res.cc_rough = cc_rough;
    res.ior = mat.ior;
    res.mat_flags = mat.flags;
    res.proc_kind = mat.procedural_kind;
    return res;
}

static __forceinline__ __device__ void trace_primary(
    float3 origin, float3 dir, float tmin,
    unsigned int& p0, unsigned int& p1, unsigned int& p2, unsigned int& p3,
    unsigned int& p4, unsigned int& p5, unsigned int& p6, unsigned int& p7
) {
    if (params.enable_ser != 0u) {
        optixTraverse(
            (OptixTraversableHandle)params.handle,
            origin, dir,
            tmin, 1e16f, 0.0f,
            OptixVisibilityMask(255),
            OPTIX_RAY_FLAG_NONE,
            0, 2, 0,
            p0, p1, p2, p3, p4, p5, p6, p7
        );
        // 6-bit SER coherence hint: sky miss = 0, CSG material buckets = 1..63 (never collides
        // with sky miss and never overflows 6 bits), classic geometry = 60..63.
        unsigned int hint = 0u;
        if (optixHitObjectIsHit()) {
            if (optixHitObjectGetHitKind() == 1u) {
                unsigned int mat_bucket = optixHitObjectGetAttribute_3() & 0x3Fu;
                hint = (mat_bucket == 0u) ? 63u : mat_bucket;
            } else {
                hint = 60u + (optixHitObjectGetInstanceId() & 0x3u);
            }
        }
        optixReorder(hint, 6u);
        optixInvoke(p0, p1, p2, p3, p4, p5, p6, p7);
    } else {
        optixTrace(
            (OptixTraversableHandle)params.handle,
            origin, dir,
            tmin, 1e16f, 0.0f,
            OptixVisibilityMask(255),
            OPTIX_RAY_FLAG_NONE,
            0, 2, 0,
            p0, p1, p2, p3, p4, p5, p6, p7
        );
    }
}

static __forceinline__ __device__ bool trace_shadow(float3 origin, float3 dir, float max_t) {
    unsigned int hit = 1u;
    optixTrace(
        (OptixTraversableHandle)params.handle,
        origin, dir,
        1.0e-5f, max_t, 0.0f,
        OptixVisibilityMask(255),
        OPTIX_RAY_FLAG_TERMINATE_ON_FIRST_HIT | OPTIX_RAY_FLAG_DISABLE_CLOSESTHIT,
        1, 2, 1,
        hit
    );
    return hit != 0u;
}

static __forceinline__ __device__ float3 turbo_colormap(float t) {
    // Google AI Turbo polynomial approximation (perceptually smooth LiDAR range visualization).
    float x = fmaxf(0.0f, fminf(1.0f, t));
    float x2 = x * x;
    float x3 = x2 * x;
    float x4 = x2 * x2;
    float x5 = x4 * x;
    float r = 0.13572138f + 4.61539260f * x - 42.66032258f * x2 + 132.13108234f * x3
        - 152.94239396f * x4 + 59.28637943f * x5;
    float g = 0.09140261f + 2.19418839f * x + 4.84296658f * x2 - 14.18503333f * x3
        + 4.27729857f * x4 + 2.82956604f * x5;
    float b = 0.10667330f + 12.64194608f * x - 60.58204836f * x2 + 110.36276771f * x3
        - 89.90310912f * x4 + 27.34824973f * x5;
    return make_float3(fmaxf(0.0f, fminf(1.0f, r)), fmaxf(0.0f, fminf(1.0f, g)), fmaxf(0.0f, fminf(1.0f, b)));
}

static __forceinline__ __device__ float3 range_lidar_aov_color(float range_m) {
    if (range_m > 1e5f) return make_float3(0.0f, 0.0f, 0.0f);
    // Log-compressed Euclidean LiDAR range across 0..250 m (near = warm bright, far = deep indigo).
    float norm = log2f(1.0f + fmaxf(0.0f, range_m) * 0.15f) / log2f(1.0f + 250.0f * 0.15f);
    return turbo_colormap(1.0f - fminf(1.0f, norm));
}

static __forceinline__ __device__ unsigned int murmur3_rotl32(unsigned int x, unsigned int r) {
    return (x << r) | (x >> (32u - r));
}

static __forceinline__ __device__ unsigned int murmur3_32_3u(
    unsigned int k0,
    unsigned int k1,
    unsigned int k2
) {
    // Canonical MurmurHash3-32 over 12 bytes (inst_id, part_id, sem_id), seed = 0.
    const unsigned int c1 = 0xcc9e2d51u;
    const unsigned int c2 = 0x1b873593u;
    unsigned int h = 0u;
    unsigned int ks[3] = { k0, k1, k2 };
    for (int i = 0; i < 3; ++i) {
        unsigned int k = ks[i];
        k *= c1;
        k = murmur3_rotl32(k, 15u);
        k *= c2;
        h ^= k;
        h = murmur3_rotl32(h, 13u);
        h = h * 5u + 0xe6546b64u;
    }
    h ^= 12u;
    h ^= h >> 16u;
    h *= 0x85ebca6bu;
    h ^= h >> 13u;
    h *= 0xc2b2ae35u;
    h ^= h >> 16u;
    return h;
}

static __forceinline__ __device__ unsigned int cryptomatte_hash_to_f32_bits(unsigned int m3hash) {
    // Psyop / Academy Cryptomatte specification `hash_to_float`:
    // mantissa = low 23 bits, exponent = bits 23..30 (remapped away from 0/255 so zero NaN/Inf/denormals).
    unsigned int exp = (m3hash >> 23u) & 0xFFu;
    if (exp == 0u || exp == 255u) {
        exp ^= 1u;
    }
    return (exp << 23u) | (m3hash & 0x007FFFFFu);
}

static __forceinline__ __device__ float3 cryptomatte_aov_color(
    unsigned int inst_id,
    unsigned int part_id,
    unsigned int sem_id,
    bool has_hit
) {
    if (!has_hit) return make_float3(0.0f, 0.0f, 0.0f);
    unsigned int m3hash = murmur3_32_3u(inst_id, part_id, sem_id);
    unsigned int id_bits = cryptomatte_hash_to_f32_bits(m3hash);
    // Official Psyop / Nuke / Blender / Houdini Cryptomatte false-color preview mapping from float32 ID bits.
    float r = 0.25f * ((float)(id_bits << 8u) * (1.0f / 4294967296.0f));
    float g = (float)(id_bits << 16u) * (1.0f / 4294967296.0f);
    float b = (float)(id_bits << 24u) * (1.0f / 4294967296.0f);
    return make_float3(r, g, b);
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
    unsigned int proj = params.camera_projection;

    float3 accum_col = make_float3(0.0f, 0.0f, 0.0f);
    float3 first_albedo = make_float3(0.0f, 0.0f, 0.0f);
    float3 first_diffuse = make_float3(0.0f, 0.0f, 0.0f);
    float3 first_specular = make_float3(0.0f, 0.0f, 0.0f);
    float3 first_normal = make_float3(0.0f, 0.0f, 1.0f);
    float first_roughness = 1.0f;
    float first_depth = 1e6f;
    float first_range = 1e6f;
    float3 first_world_pos = make_float3(0.0f, 0.0f, 0.0f);
    float3 first_mat_pbr = make_float3(0.0f, 0.0f, 0.0f);
    bool first_has_hit = false;
    float2 first_motion = make_float2(0.0f, 0.0f);
    unsigned int first_sem_id = 0u;
    unsigned int first_inst_id = 0u;
    unsigned int first_part_id = 0u;

    float3 prev_pos = make_f3(params.prev_cam_pos);
    float3 prev_fwd = make_f3(params.prev_cam_forward);
    float3 prev_rgt = make_f3(params.prev_cam_right);
    float3 prev_up  = make_f3(params.prev_cam_up);
    float prev_tan_fov = params.prev_cam_up[3];
    bool has_prev_cam = (prev_tan_fov > 1e-5f) && (proj == 0u);

    const bool aov_aa = (params.aov_mode != 0u);
    const bool aov_geom_only = aov_aa
        && (params.aov_mode != 1u && params.aov_mode != 2u && params.aov_mode != 9u);
    const unsigned int loop_spp = aov_aa ? 4u : spp;
    float3 aov_accum = make_float3(0.0f, 0.0f, 0.0f);
    float aov_w_accum = 0.0f;
    float2 s0_motion = make_float2(0.0f, 0.0f);
    float s0_depth = 1e6f;
    unsigned int s0_sem_id = 0u;
    unsigned int s0_inst_id = 0u;
    bool s0_has_hit = false;
    float3 s0_albedo = make_float3(0.0f, 0.0f, 0.0f);
    float3 s0_normal = make_float3(0.0f, 1.0f, 0.0f);
    float s0_roughness = 0.5f;
    float3 s0_diffuse = make_float3(0.0f, 0.0f, 0.0f);
    float3 s0_specular = make_float3(0.0f, 0.0f, 0.0f);

    for (unsigned int s = 0; s < loop_spp; ++s) {
        if (aov_aa) {
            first_has_hit = false;
            first_sem_id = 0u;
            first_inst_id = 0u;
            first_part_id = 0u;
            first_depth = 1e6f;
            first_range = 1e6f;
            first_motion = make_float2(0.0f, 0.0f);
            first_albedo = make_float3(0.0f, 0.0f, 0.0f);
            first_normal = make_float3(0.0f, 1.0f, 0.0f);
            first_world_pos = make_float3(0.0f, 0.0f, 0.0f);
            first_mat_pbr = make_float3(0.0f, 0.0f, 0.0f);
        }
        float jx, jy;
        if (aov_aa) {
            // Deterministic 4x Rotated-Grid Supersampling (RGSS): zero frame-to-frame subpixel
            // jitter and clean 4x anti-aliased edges on every non-Beauty AOV pass.
            const float rgss_x[4] = {-0.375f,  0.125f, -0.125f, 0.375f};
            const float rgss_y[4] = {-0.125f, -0.375f,  0.375f, 0.125f};
            jx = rgss_x[s & 3u];
            jy = rgss_y[s & 3u];
        } else {
            bool use_dlss_jitter = (s == 0u) && (params.jitter_x != 0.0f || params.jitter_y != 0.0f);
            jx = use_dlss_jitter ? params.jitter_x : (rand01(seed) - 0.5f + params.jitter_x);
            jy = use_dlss_jitter ? params.jitter_y : (rand01(seed) - 0.5f + params.jitter_y);
        }
        float px = (float)idx.x + 0.5f + jx;
        float py = (float)idx.y + 0.5f + jy;
        float u01 = px / (float)params.width;
        float v01 = py / (float)params.height;

        float3 dir;
        bool masked = false;
        if (proj == 1u) {
            // 180-degree equidistant automotive fisheye inscribed in centered viewport circle.
            float half_min = 0.5f * fminf((float)params.width, (float)params.height);
            float dx = (px - 0.5f * (float)params.width) / fmaxf(1.0f, half_min);
            float dy = (0.5f * (float)params.height - py) / fmaxf(1.0f, half_min);
            float r = sqrtf(dx * dx + dy * dy);
            if (r > 1.0f) {
                masked = true;
                dir = cam_fwd;
            } else {
                float theta = r * 1.5707963268f;
                float phi = atan2f(dy, dx);
                float st = sinf(theta);
                float ct = cosf(theta);
                float3 radial = f3_add(f3_mul(cam_rgt, cosf(phi)), f3_mul(cam_up, sinf(phi)));
                dir = f3_norm(f3_add(f3_mul(cam_fwd, ct), f3_mul(radial, st)));
            }
        } else if (proj == 2u) {
            // 360 x 180 equirectangular panorama (ideal for Robotaxi 360 Roof LiDAR).
            float yaw = (u01 - 0.5f) * 6.28318530718f;
            float pitch = (0.5f - v01) * 3.14159265359f;
            float cp = cosf(pitch);
            float sp = sinf(pitch);
            float3 hdir = f3_add(f3_mul(cam_fwd, cosf(yaw)), f3_mul(cam_rgt, sinf(yaw)));
            dir = f3_norm(f3_add(f3_mul(hdir, cp), f3_mul(cam_up, sp)));
        } else if (proj == 3u) {
            // 360-degree cylindrical slit-scan / pushbroom panorama (vertical lines remain straight).
            float yaw = (u01 - 0.5f) * 6.28318530718f;
            float3 hdir = f3_add(f3_mul(cam_fwd, cosf(yaw)), f3_mul(cam_rgt, sinf(yaw)));
            float v_off = (0.5f - v01) * 2.0f * tan_fov;
            dir = f3_norm(f3_add(hdir, f3_mul(cam_up, v_off)));
        } else {
            // Standard rectilinear pinhole camera.
            float u = u01 * 2.0f - 1.0f;
            float v = 1.0f - v01 * 2.0f;
            dir = f3_norm(f3_add(
                cam_fwd,
                f3_add(f3_mul(cam_rgt, u * aspect * tan_fov), f3_mul(cam_up, v * tan_fov))
            ));
        }

        if (masked) {
            if (s == 0u || aov_aa) {
                first_albedo = make_float3(0.0f, 0.0f, 0.0f);
                first_diffuse = make_float3(0.0f, 0.0f, 0.0f);
                first_specular = make_float3(0.0f, 0.0f, 0.0f);
                first_normal = make_float3(0.0f, 0.0f, 0.0f);
                first_roughness = 0.0f;
                first_depth = 1e6f;
                first_range = 1e6f;
                first_has_hit = false;
            }
            continue;
        }
        float3 origin = cam_pos;
        float aperture = params.cam_pos[3];
        float focus_dist = params.cam_right[3];
        if (proj == 0u && aperture > 1e-5f && focus_dist > 0.05f) {
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

        int max_bounces = aov_aa ? 1 : (int)params.bounces;
        for (int bounce = 0; bounce < 16; ++bounce) {
            if (bounce >= max_bounces) break;
            if (bounce >= 2) {
                float p_survive = fmaxf(
                    0.05f,
                    fminf(0.95f, fmaxf(throughput.x, fmaxf(throughput.y, throughput.z)))
                );
                if (rand01(seed) > p_survive) break;
                throughput = f3_mul(throughput, 1.0f / p_survive);
            }
            unsigned int p0 = 0, p1 = 0, p2 = 0, p3 = 0, p4 = 0, p5 = 0, p6 = 0, p7 = 0;
            trace_primary(origin, dir, (bounce == 0) ? 1.0e-4f : 1.0e-5f, p0, p1, p2, p3, p4, p5, p6, p7);

            if (p0 == 0u) {
                if (!aov_geom_only) {
                    float3 sky = sky_color(dir, sun_dir, bounce == 0);
                    sample_col = f3_add(sample_col, f3_mul3(throughput, sky));
                    if (bounce == 0 && (s == 0u || aov_aa)) {
                        first_albedo = make_float3(
                            fmaxf(0.0f, fminf(1.0f, sky.x)),
                            fmaxf(0.0f, fminf(1.0f, sky.y)),
                            fmaxf(0.0f, fminf(1.0f, sky.z))
                        );
                    }
                }
                if (bounce == 0 && (s == 0u || aov_aa)) {
                    if (params.out_diffuse_albedo_f32 != 0ull && params.aov_mode == 0u) {
                        // DLSS-RR Integration Guide §§3.4.1/3.4.3/3.4.4 sky defaults
                        first_diffuse = make_float3(0.5f, 0.5f, 0.5f);
                        first_specular = make_float3(0.0f, 0.0f, 0.0f);
                        first_normal = make_float3(0.0f, 0.0f, 0.0f);
                        first_roughness = 0.0f;
                    } else {
                        first_diffuse = make_float3(0.0f, 0.0f, 0.0f);
                        first_specular = make_float3(0.0f, 0.0f, 0.0f);
                        first_normal = make_float3(-dir.x, -dir.y, -dir.z);
                        first_roughness = 1.0f;
                    }
                    first_depth = 1e6f;
                    if (has_prev_cam) {
                        float pz = f3_dot(dir, prev_fwd);
                        if (pz > 1e-4f) {
                            float px_ndc = f3_dot(dir, prev_rgt) / (pz * aspect * prev_tan_fov);
                            float py_ndc = f3_dot(dir, prev_up)  / (pz * prev_tan_fov);
                            float prev_x = (px_ndc * 0.5f + 0.5f) * (float)params.width;
                            float prev_y = (0.5f - py_ndc * 0.5f) * (float)params.height;
                            // Subtract the subpixel jitter used to cast `dir` so static cameras produce 0 motion.
                            first_motion = make_float2(
                                prev_x - ((float)idx.x + 0.5f + jx),
                                prev_y - ((float)idx.y + 0.5f + jy)
                            );
                        }
                    }
                }
                break;
            }

            float hit_t = __uint_as_float(p1);
            float3 geom_n = make_float3(__uint_as_float(p2), __uint_as_float(p3), __uint_as_float(p4));
            bool entering = f3_dot(geom_n, dir) < 0.0f;
            float3 n = entering ? geom_n : f3_mul(geom_n, -1.0f);
            bool is_csg = (p0 == 2u);
            float3 base_col = make_float3(
                (float)(p5 & 0xFFu) / 255.0f,
                (float)((p5 >> 8u) & 0xFFu) / 255.0f,
                (float)((p5 >> 16u) & 0xFFu) / 255.0f
            );
            unsigned int inst_id = is_csg ? 0xFFFFu : (p6 & 0xFFu);
            float3 hit_pos = f3_add(origin, f3_mul(dir, hit_t));

            if (bounce == 0 && (s == 0u || aov_aa)) {
                first_has_hit = true;
                first_world_pos = hit_pos;
                first_range = f3_len(f3_sub(hit_pos, cam_pos));
                first_depth = fmaxf(0.0f, f3_dot(f3_sub(hit_pos, cam_pos), cam_fwd));
                float3 prev_world_pos = hit_pos;
                if (is_csg) {
                    first_sem_id = (p5 >> 16u) & 0xFFFFu;
                    first_part_id = p7;
                    unsigned int uid = p6 + 1u;
                    if (params.csg_instances != 0ull && p6 < params.csg_instance_count) {
                        uint4 meta = load_csg_instance_meta(p6);
                        if (meta.y != 0u) uid = meta.y;
                        if (has_prev_cam
                            && params.csg_prev_xforms != 0ull
                            && (meta.w & CSG_INST_FLAG_MOTION_BLUR) != 0u)
                        {
                            const float* cur_m = ((const CsgGpuInstance*)params.csg_instances)[p6].xform;
                            float3 p_obj;
                            if (csg_inv_xform_pt_ldg(cur_m, hit_pos, p_obj)) {
                                const float* prev_m = ((const float*)params.csg_prev_xforms)
                                    + ((unsigned long long)p6 * 12ull);
                                prev_world_pos = csg_xform_pt_ldg(prev_m, p_obj);
                            }
                        }
                    }
                    first_inst_id = uid;
                } else {
                    first_sem_id = (inst_id == 0u) ? 16u : (inst_id + 1u);
                    first_inst_id = p6 + 1u;
                    first_part_id = 0u;
                }
                if (has_prev_cam) {
                    float3 rel = f3_sub(prev_world_pos, prev_pos);
                    float pz = f3_dot(rel, prev_fwd);
                    if (pz > 1e-4f) {
                        float px_ndc = f3_dot(rel, prev_rgt) / (pz * aspect * prev_tan_fov);
                        float py_ndc = f3_dot(rel, prev_up)  / (pz * prev_tan_fov);
                        float prev_x = (px_ndc * 0.5f + 0.5f) * (float)params.width;
                        float prev_y = (0.5f - py_ndc * 0.5f) * (float)params.height;
                        // Subtract the subpixel jitter used to cast `dir` so static cameras produce 0 motion.
                        first_motion = make_float2(
                            prev_x - ((float)idx.x + 0.5f + jx),
                            prev_y - ((float)idx.y + 0.5f + jy)
                        );
                    }
                }
                if (aov_geom_only) {
                    // Geometry-only AOVs (Depth/Motion/Semantic/Instance/WorldPos/Range/Cryptomatte)
                    // skip all procedural micro-detail shading, shadow rays, Blinn-Phong, and bounces.
                    first_normal = n;
                    break;
                }
            }

            if (is_csg) {
                unsigned int mat_id = p5 & 0xFFFFu;
                unsigned int sem_id = (p5 >> 16u) & 0xFFFFu;
                CsgProcResult pr = eval_csg_procedural(mat_id, sem_id, p7, hit_pos, n);
                n = pr.n;
                base_col = pr.base_col;
                float rough = pr.rough;
                float metallic = pr.metallic;
                float clearcoat = pr.clearcoat;
                float cc_rough = pr.cc_rough;
                float3 emission = pr.emission;

                // Emissive contribution (streetlights, vehicle headlights/taillights, signals, aviation beacons, lit windows)
                float max_em = fmaxf(emission.x, fmaxf(emission.y, emission.z));
                if (max_em > 1e-4f) {
                    sample_col = f3_add(sample_col, f3_mul3(throughput, emission));
                    if (max_em > 1.5f) {
                        if (bounce == 0 && (s == 0u || aov_aa)) {
                            first_albedo = make_float3(
                                fminf(1.0f, emission.x * 0.35f),
                                fminf(1.0f, emission.y * 0.35f),
                                fminf(1.0f, emission.z * 0.35f)
                            );
                            first_diffuse = make_float3(0.0f, 0.0f, 0.0f);
                            first_specular = make_float3(0.0f, 0.0f, 0.0f);
                            first_normal = n;
                            first_roughness = 1.0f;
                            first_mat_pbr = make_float3(1.0f, 0.0f, 1.0f);
                        }
                        break;
                    }
                }

                // Dielectric transmissive CSG glass (windshields, architectural glazing, bus shelters)
                if ((pr.mat_flags & CSG_MAT_FLAG_TRANSMISSIVE) != 0u && pr.proc_kind == CSG_PROC_GLASS) {
                    float ior = fmaxf(1.05f, pr.ior);
                    float g_rough = fmaxf(0.0f, fminf(0.45f, rough));
                    float r0 = (1.0f - ior) / (1.0f + ior);
                    r0 = r0 * r0;
                    if (bounce == 0 && (s == 0u || aov_aa)) {
                        first_albedo = base_col;
                        first_diffuse = make_float3(0.0f, 0.0f, 0.0f);
                        first_specular = make_float3(r0, r0, r0);
                        first_normal = n;
                        first_roughness = g_rough;
                        first_mat_pbr = make_float3(g_rough, 0.0f, 1.0f);
                    }
                    if (aov_aa) {
                        break;
                    }
                    if (!entering) {
                        throughput = f3_mul3(throughput, make_float3(
                            expf(-fmaxf(0.0f, pr.absorption.x) * hit_t),
                            expf(-fmaxf(0.0f, pr.absorption.y) * hit_t),
                            expf(-fmaxf(0.0f, pr.absorption.z) * hit_t)
                        ));
                    }
                    float eta = entering ? (1.0f / ior) : ior;
                    float cos_i = fminf(1.0f, fmaxf(0.0f, -f3_dot(dir, n)));
                    float fresnel = r0 + (1.0f - r0) * powf(1.0f - cos_i, 5.0f);
                    float sin2_t = eta * eta * (1.0f - cos_i * cos_i);
                    bool tir = sin2_t > 1.0f;
                    float g_bias = fmaxf(0.001f, hit_t * 0.0004f);
                    if (tir || rand01(seed) < fresnel) {
                        float3 refl = f3_reflect(dir, n);
                        if (g_rough > 1e-4f) {
                            refl = f3_norm(f3_add(refl, f3_mul(sample_cosine_hemisphere(n, seed), g_rough)));
                        }
                        dir = refl;
                        origin = f3_add(hit_pos, f3_mul(n, g_bias));
                    } else {
                        float cos_t = sqrtf(fmaxf(0.0f, 1.0f - sin2_t));
                        float3 refr = f3_norm(f3_add(f3_mul(dir, eta), f3_mul(n, eta * cos_i - cos_t)));
                        if (g_rough > 1e-4f) {
                            refr = f3_norm(f3_add(refr, f3_mul(sample_cosine_hemisphere(refr, seed), g_rough)));
                        }
                        dir = refr;
                        origin = f3_sub(hit_pos, f3_mul(n, g_bias));
                        throughput = f3_mul3(throughput, make_float3(
                            0.65f + 0.35f * base_col.x,
                            0.65f + 0.35f * base_col.y,
                            0.65f + 0.35f * base_col.z
                        ));
                    }
                    continue;
                }

                if (bounce == 0 && (s == 0u || aov_aa)) {
                    float inv_m = 1.0f - metallic;
                    first_albedo = base_col;
                    first_diffuse = f3_mul(base_col, inv_m);
                    first_specular = f3_add(
                        make_float3(0.04f * inv_m + 0.04f * clearcoat, 0.04f * inv_m + 0.04f * clearcoat, 0.04f * inv_m + 0.04f * clearcoat),
                        f3_mul(base_col, metallic)
                    );
                    first_normal = n;
                    first_roughness = (clearcoat > 0.5f) ? fminf(rough, cc_rough + 0.08f) : rough;
                    first_mat_pbr = make_float3(
                        fmaxf(0.0f, fminf(1.0f, rough)),
                        fmaxf(0.0f, fminf(1.0f, metallic)),
                        (max_em > 0.05f) ? fminf(1.0f, max_em) : 0.0f
                    );
                }
                if (aov_aa) {
                    // Shading AOVs (Albedo/Normal/Material) skip shadow rays, Blinn-Phong, and bounces.
                    break;
                }

                float ray_bias = fmaxf(0.001f, hit_t * 0.0004f);
                float3 biased_origin = f3_add(hit_pos, f3_mul(n, ray_bias));
                float ndotl = fmaxf(0.0f, f3_dot(n, sun_dir));
                if (ndotl > 0.0f) {
                    bool occluded = false;
                    if (params.enable_shadows != 0u) {
                        float3 jitter = make_float3(rand01(seed) - 0.5f, rand01(seed) - 0.5f, rand01(seed) - 0.5f);
                        float3 sdir = f3_norm(f3_add(sun_dir, f3_mul(jitter, 0.024f)));
                        occluded = trace_shadow(biased_origin, sdir, 1500.0f);
                    }
                    if (!occluded) {
                        float sun_i = params.fractal_r1[3];
                        float3 sun_rad = ((params.csg_flags & 2u) != 0u)
                            ? make_float3(sun_i * 0.55f, sun_i * 0.65f, sun_i * 0.90f)
                            : (((params.csg_flags & 8u) != 0u)
                                ? make_float3(sun_i * 1.15f, sun_i * 0.68f, sun_i * 0.36f)
                                : make_float3(sun_i, sun_i * 0.94f, sun_i * 0.84f));
                        float3 diff_term = f3_mul(f3_mul3(base_col, sun_rad), ndotl * 0.318309886f * (1.0f - metallic * 0.85f));
                        float3 half_v = f3_norm(f3_sub(sun_dir, dir));
                        float ndoth = fmaxf(0.0f, f3_dot(n, half_v));
                        float spec_exp = fmaxf(4.0f, 2.0f / (rough * rough + 0.002f) - 2.0f);
                        float spec_norm = (spec_exp + 2.0f) * 0.125f * 0.318309886f;
                        float spec_val = powf(ndoth, fminf(spec_exp, 512.0f)) * fminf(spec_norm, 6.0f);
                        float3 spec_tint = f3_add(make_float3(0.04f, 0.04f, 0.04f), f3_mul(base_col, metallic));
                        float3 direct = f3_add(diff_term, f3_mul(f3_mul3(spec_tint, sun_rad), spec_val * ndotl));
                        if (clearcoat > 0.01f) {
                            float cc_exp = fmaxf(12.0f, 2.0f / (cc_rough * cc_rough + 0.001f) - 2.0f);
                            float cc_norm = (cc_exp + 2.0f) * 0.125f * 0.318309886f;
                            float cc_val = powf(ndoth, fminf(cc_exp, 1024.0f)) * fminf(cc_norm, 10.0f);
                            direct = f3_add(direct, f3_mul(sun_rad, 0.08f * clearcoat * cc_val * ndotl));
                        }
                        sample_col = f3_add(sample_col, f3_mul3(throughput, direct));
                    }
                }

                float cos_v = fminf(1.0f, fmaxf(0.0f, -f3_dot(dir, n)));
                float schlick = 0.04f + 0.96f * powf(1.0f - cos_v, 5.0f);
                float p_coat = clearcoat * (0.12f + 0.78f * schlick);
                float p_spec = (1.0f - p_coat) * (0.12f + 0.78f * metallic + (1.0f - rough) * 0.25f * schlick);
                float u_lobe = rand01(seed);
                float3 diff_dir = sample_cosine_hemisphere(n, seed);
                float3 refl_dir = f3_reflect(dir, n);
                if (u_lobe < p_coat) {
                    dir = f3_norm(f3_add(refl_dir, f3_mul(diff_dir, cc_rough)));
                    origin = biased_origin;
                    // Clearcoat is neutral white Fresnel reflection (preserves crisp sky/lamp reflections)
                    throughput = f3_mul(throughput, 0.88f);
                } else if (u_lobe < p_coat + p_spec) {
                    dir = f3_norm(f3_add(refl_dir, f3_mul(diff_dir, rough)));
                    origin = biased_origin;
                    float3 spec_col = f3_add(
                        make_float3(0.04f + 0.92f * (1.0f - metallic) * schlick,
                                    0.04f + 0.92f * (1.0f - metallic) * schlick,
                                    0.04f + 0.92f * (1.0f - metallic) * schlick),
                        f3_mul(base_col, metallic)
                    );
                    throughput = f3_mul3(throughput, spec_col);
                } else {
                    dir = diff_dir;
                    origin = biased_origin;
                    throughput = f3_mul3(throughput, base_col);
                }
                continue;
            }

            if (inst_id == 2u) {
                // Dielectric glass sphere: Beer-Lambert absorption + Schlick Fresnel + refraction/reflection
                float ior = fmaxf(1.05f, params.glass_params[0]);
                float absorb = fmaxf(0.0f, params.glass_params[1]);
                float g_rough = fmaxf(0.0f, fminf(0.5f, params.glass_params[2]));
                float r0 = (1.0f - ior) / (1.0f + ior);
                r0 = r0 * r0;
                if (bounce == 0 && (s == 0u || aov_aa)) {
                    first_albedo = make_float3(0.92f, 0.97f, 1.0f);
                    first_diffuse = make_float3(0.0f, 0.0f, 0.0f);
                    first_specular = make_float3(r0, r0, r0);
                    first_normal = n;
                    first_roughness = g_rough;
                    first_mat_pbr = make_float3(g_rough, 0.0f, 1.0f);
                }
                if (aov_aa) {
                    break;
                }
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
                float eta = entering ? (1.0f / ior) : ior;
                float cos_i = fminf(1.0f, fmaxf(0.0f, -f3_dot(dir, n)));
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
                if (bounce == 0 && (s == 0u || aov_aa)) {
                    first_albedo = make_float3(
                        fminf(1.0f, emission.x * 0.45f),
                        fminf(1.0f, emission.y * 0.45f),
                        fminf(1.0f, emission.z * 0.45f)
                    );
                    first_diffuse = make_float3(0.0f, 0.0f, 0.0f);
                    first_specular = make_float3(0.0f, 0.0f, 0.0f);
                    first_normal = n;
                    first_roughness = 1.0f;
                    first_mat_pbr = make_float3(1.0f, 0.0f, 1.0f);
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
            } else if (inst_id == 4u) {
                // Brushed satin-gold filigree halo (OptiX OMM / Any-Hit benchmark mesh)
                float brush = fbm2(hit_pos.y * 28.0f, atan2f(hit_pos.z, hit_pos.x) * 14.0f);
                base_col = make_float3(
                    0.74f + 0.14f * brush,
                    0.56f + 0.12f * brush,
                    0.26f + 0.08f * brush
                );
                metallic = 0.88f;
                rough = 0.16f + 0.10f * (1.0f - brush);
            } else {
                // Fractal custom primitive uses orbit-trap base_col from p5
                metallic = params.fractal_r1[0];
            }

            if (horizon_fade >= 0.995f) {
                float3 sky = sky_color(dir, sun_dir, bounce == 0);
                sample_col = f3_add(sample_col, f3_mul3(throughput, sky));
                break;
            }

            if (bounce == 0 && (s == 0u || aov_aa)) {
                float inv_m = 1.0f - metallic;
                first_albedo = base_col;
                first_diffuse = f3_mul(base_col, inv_m);
                first_specular = f3_add(
                    make_float3(0.04f * inv_m, 0.04f * inv_m, 0.04f * inv_m),
                    f3_mul(base_col, metallic)
                );
                first_normal = n;
                first_roughness = fmaxf(0.0f, fminf(1.0f, rough));
                first_mat_pbr = make_float3(first_roughness, fmaxf(0.0f, fminf(1.0f, metallic)), 0.0f);
            }
            if (aov_aa) {
                break;
            }

            // Adaptive surface offset: scales down to micro-detail epsilon on fractals so
            // fine buds cast crisp self-shadows and tight crevices self-occlude naturally.
            float ray_bias;
            if (inst_id == 1u) {
                float hit_eps = fractal_hit_eps(f3_len(f3_sub(hit_pos, cam_pos)));
                ray_bias = fmaxf(2.5e-5f, hit_eps * 5.0f);
            } else {
                ray_bias = fmaxf(0.0005f, hit_t * 0.0003f);
            }
            float3 biased_origin = f3_add(hit_pos, f3_mul(n, ray_bias));

            float ndotl = fmaxf(0.0f, f3_dot(n, sun_dir));
            if (ndotl > 0.0f) {
                bool occluded = false;
                if (params.enable_shadows != 0u) {
                    float3 jitter = make_float3(rand01(seed) - 0.5f, rand01(seed) - 0.5f, rand01(seed) - 0.5f);
                    float3 sdir = f3_norm(f3_add(sun_dir, f3_mul(jitter, 0.028f)));
                    occluded = trace_shadow(biased_origin, sdir, 1000.0f);
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
            origin = biased_origin;
            throughput = f3_mul3(throughput, base_col);
        }

        if (aov_aa) {
            if (s == 0u) {
                s0_motion = first_motion;
                s0_depth = first_depth;
                s0_sem_id = first_sem_id;
                s0_inst_id = first_inst_id;
                s0_has_hit = first_has_hit;
                s0_albedo = first_albedo;
                s0_normal = first_normal;
                s0_roughness = first_roughness;
                s0_diffuse = first_diffuse;
                s0_specular = first_specular;
            }
            float3 sc = make_float3(0.0f, 0.0f, 0.0f);
            float sw = 1.0f;
            if (params.aov_mode == 1u) {
                sc = first_albedo;
            } else if (params.aov_mode == 2u) {
                sc = first_normal;
            } else if (params.aov_mode == 3u) {
                float dv = (first_depth > 1e5f) ? 0.0f : (1.0f / (1.0f + first_depth * 0.15f));
                sc = make_float3(dv, dv, dv);
            } else if (params.aov_mode == 4u) {
                float mag = sqrtf(first_motion.x * first_motion.x + first_motion.y * first_motion.y);
                sc = make_float3(
                    fmaxf(0.0f, fminf(1.0f, 0.5f + first_motion.x * 0.05f)),
                    fmaxf(0.0f, fminf(1.0f, 0.5f + first_motion.y * 0.05f)),
                    fminf(1.0f, mag * 0.05f)
                );
            } else if (params.aov_mode == 5u) {
                sc = semantic_aov_color(first_sem_id);
            } else if (params.aov_mode == 6u) {
                sc = instance_aov_color(first_inst_id);
            } else if (params.aov_mode == 7u) {
                sc = first_has_hit ? first_world_pos : make_float3(0.0f, 0.0f, 0.0f);
                sw = first_has_hit ? 1.0f : 0.0f;
            } else if (params.aov_mode == 8u) {
                sc = range_lidar_aov_color(first_range);
            } else if (params.aov_mode == 9u) {
                sc = first_has_hit ? first_mat_pbr : make_float3(0.0f, 0.0f, 0.0f);
            } else if (params.aov_mode == 10u) {
                sc = cryptomatte_aov_color(first_inst_id, first_part_id, first_sem_id, first_has_hit);
            }
            aov_accum = f3_add(aov_accum, sc);
            aov_w_accum += sw;
        } else {
            accum_col = f3_add(accum_col, sample_col);
        }
    }

    float inv_spp = 1.0f / (float)spp;
    float3 final_col = aov_aa ? f3_mul(aov_accum, 0.25f) : f3_mul(accum_col, inv_spp);
    float final_w = aov_aa ? (aov_w_accum * 0.25f) : 1.0f;
    if (aov_aa) {
        first_motion = s0_motion;
        first_depth = s0_depth;
        first_sem_id = s0_sem_id;
        first_inst_id = s0_inst_id;
        first_has_hit = s0_has_hit;
        first_albedo = s0_albedo;
        first_normal = s0_normal;
        first_roughness = s0_roughness;
        first_diffuse = s0_diffuse;
        first_specular = s0_specular;
    }

    unsigned int out_idx = idx.y * params.width + idx.x;
    if (params.out_motion_f32 != 0ull) {
        ((float2*)params.out_motion_f32)[out_idx] = first_motion;
    }
    if (params.out_depth_f32 != 0ull) {
        ((float*)params.out_depth_f32)[out_idx] = first_depth;
    }
    if (params.out_semantic_u32 != 0ull) {
        ((unsigned int*)params.out_semantic_u32)[out_idx] = first_sem_id;
    }
    if (params.out_instance_u32 != 0ull) {
        ((unsigned int*)params.out_instance_u32)[out_idx] = first_has_hit ? first_inst_id : 0u;
    }
    if (params.out_diffuse_albedo_f32 != 0ull) {
        ((float4*)params.out_diffuse_albedo_f32)[out_idx] =
            make_float4(first_diffuse.x, first_diffuse.y, first_diffuse.z, 1.0f);
    }
    if (params.out_spec_albedo_f32 != 0ull) {
        ((float4*)params.out_spec_albedo_f32)[out_idx] =
            make_float4(first_specular.x, first_specular.y, first_specular.z, 1.0f);
    }
    float4* color_buf  = (float4*)params.out_color_f32;
    float4* albedo_buf = (float4*)params.out_albedo_f32;
    float4* normal_buf = (float4*)params.out_normal_f32;
    if (params.accum_count == 0u) {
        color_buf[out_idx]  = make_float4(final_col.x, final_col.y, final_col.z, final_w);
        albedo_buf[out_idx] = make_float4(first_albedo.x, first_albedo.y, first_albedo.z, 1.0f);
        normal_buf[out_idx] = make_float4(first_normal.x, first_normal.y, first_normal.z, first_roughness);
    } else {
        float w = 1.0f / (float)(params.accum_count + 1u);
        float4 pc = color_buf[out_idx];
        float4 pa = albedo_buf[out_idx];
        float4 pn = normal_buf[out_idx];
        color_buf[out_idx] = make_float4(
            pc.x + (final_col.x - pc.x) * w,
            pc.y + (final_col.y - pc.y) * w,
            pc.z + (final_col.z - pc.z) * w,
            pc.w + (final_w - pc.w) * w
        );
        albedo_buf[out_idx] = make_float4(
            pa.x + (first_albedo.x - pa.x) * w,
            pa.y + (first_albedo.y - pa.y) * w,
            pa.z + (first_albedo.z - pa.z) * w,
            1.0f
        );
        float3 n_acc = make_float3(
            pn.x + (first_normal.x - pn.x) * w,
            pn.y + (first_normal.y - pn.y) * w,
            pn.z + (first_normal.z - pn.z) * w
        );
        float n_len = f3_len(n_acc);
        if (n_len > 1e-4f) n_acc = f3_mul(n_acc, 1.0f / n_len);
        float r_acc = pn.w + (first_roughness - pn.w) * w;
        normal_buf[out_idx] = make_float4(n_acc.x, n_acc.y, n_acc.z, r_acc);
    }
}

extern "C" __global__ void __raygen__score_views() {
    if (params.probe_candidates == 0ull || params.probe_samples == 0ull) return;
    unsigned int grid_w = params.probe_grid_wh & 0xFFFFu;
    unsigned int grid_h = (params.probe_grid_wh >> 16u) & 0xFFFFu;
    if (grid_w == 0u || grid_h == 0u) return;

    const uint3 idx = optixGetLaunchIndex();
    if (idx.x >= grid_w || idx.y >= grid_h) return;

    const GpuProbeCandidate cand = ((const GpuProbeCandidate*)params.probe_candidates)[idx.z];
    float3 eye   = make_f3(cand.eye);
    float3 fwd   = make_f3(cand.forward);
    float3 right = make_f3(cand.right);
    float3 up    = make_f3(cand.up);
    float aspect = cand.forward[3];
    float tan_half_fov = cand.up[3];

    float ndc_x = 2.0f * (((float)idx.x + 0.5f) / (float)grid_w) - 1.0f;
    float ndc_y = 1.0f - 2.0f * (((float)idx.y + 0.5f) / (float)grid_h);
    float3 dir = f3_norm(f3_add(
        fwd,
        f3_add(f3_mul(right, ndc_x * aspect * tan_half_fov), f3_mul(up, ndc_y * tan_half_fov))
    ));

    unsigned int p0 = 0, p1 = 0, p2 = 0, p3 = 0, p4 = 0, p5 = 0, p6 = 0, p7 = 0;
    optixTrace(
        (OptixTraversableHandle)params.handle,
        eye, dir,
        0.05f, 600.0f, 0.0f,
        OptixVisibilityMask(255),
        OPTIX_RAY_FLAG_NONE,
        0, 2, 0,
        p0, p1, p2, p3, p4, p5, p6, p7
    );

    GpuProbeSample out_s;
    if (p0 == 2u) {
        out_s.t = __uint_as_float(p1);
        out_s.sem_id = (unsigned short)((p5 >> 16u) & 0xFFFFu);
        out_s._pad0 = 0u;
        unsigned int uid = p6;
        if (params.csg_instances != 0ull && p6 < params.csg_instance_count) {
            uid = load_csg_instance_meta(p6).y;
        }
        out_s.inst_id = uid;
        out_s._pad1 = 0u;
    } else if (p0 != 0u) {
        out_s.t = __uint_as_float(p1);
        unsigned int inst_id = p6 & 0xFFu;
        out_s.sem_id = (unsigned short)((inst_id == 0u) ? 16u : (inst_id + 1u));
        out_s._pad0 = 0u;
        out_s.inst_id = p6;
        out_s._pad1 = 0u;
    } else {
        out_s.t = -1.0f;
        out_s.sem_id = 0u;
        out_s._pad0 = 0u;
        out_s.inst_id = 0xFFFFFFFFu;
        out_s._pad1 = 0u;
    }

    unsigned int out_idx = (idx.z * grid_h + idx.y) * grid_w + idx.x;
    ((GpuProbeSample*)params.probe_samples)[out_idx] = out_s;
}

extern "C" __global__ void __raygen__sdg_keypoints() {
    if (params.sdg_ray_queries == 0ull || params.sdg_ray_results == 0ull) return;
    const uint3 idx = optixGetLaunchIndex();
    if (idx.x >= params.sdg_ray_count) return;

    const GpuSdgRayQuery q = ((const GpuSdgRayQuery*)params.sdg_ray_queries)[idx.x];
    float3 orig = make_float3(q.origin_tmin[0], q.origin_tmin[1], q.origin_tmin[2]);
    float  tmin = q.origin_tmin[3];
    float3 dir  = make_float3(q.dir_tmax[0], q.dir_tmax[1], q.dir_tmax[2]);
    float  tmax = q.dir_tmax[3];
    if (tmax <= tmin) {
        ((unsigned int*)params.sdg_ray_results)[idx.x] = 0xFFFFFFFFu;
        return;
    }

    unsigned int p0 = 0, p1 = 0, p2 = 0, p3 = 0, p4 = 0, p5 = 0, p6 = 0, p7 = 0;
    optixTrace(
        (OptixTraversableHandle)params.handle,
        orig, dir,
        tmin, tmax, 0.0f,
        OptixVisibilityMask(255),
        OPTIX_RAY_FLAG_NONE,
        0, 2, 0,
        p0, p1, p2, p3, p4, p5, p6, p7
    );

    unsigned int blocker_uid = 0xFFFFFFFFu;
    if (p0 == 2u) {
        if (params.csg_instances != 0ull && p6 < params.csg_instance_count) {
            blocker_uid = load_csg_instance_meta(p6).y;
        } else {
            blocker_uid = p6;
        }
    } else if (p0 != 0u) {
        blocker_uid = p6;
    }
    ((unsigned int*)params.sdg_ray_results)[idx.x] = blocker_uid;
}

extern "C" __global__ void __miss__miss() {
    optixSetPayload_0(0u);
}

extern "C" __global__ void __miss__shadow() {
    optixSetPayload_0(0u);
}

extern "C" __global__ void __closesthit__shadow() {
    optixSetPayload_0(1u);
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

// Exact 1-Lipschitz 2D SDF for the perforated filigree halo band; matches `filigree_sdf` in renderer.rs.
static __forceinline__ __device__ float filigree_sdf(float u, float v) {
    float d_border = fminf(v, 1.0f - v) - 0.06f;
    float s1 = u * 2.0f + v * 2.0f;
    float f1 = s1 - floorf(s1);
    float d_diag1 = (fabsf(f1 - 0.5f) - 0.075f) * 0.35355339f;
    float s2 = u * 2.0f - v * 2.0f + 4.0f;
    float f2 = s2 - floorf(s2);
    float d_diag2 = (fabsf(f2 - 0.5f) - 0.075f) * 0.35355339f;
    float dx = u - 0.5f;
    float dy = v - 0.5f;
    float d_ring = fabsf(sqrtf(dx * dx + dy * dy) - 0.30f) - 0.042f;
    float cu = fminf(u, 1.0f - u);
    float cv = fminf(v, 1.0f - v);
    float d_corner = fabsf(sqrtf(cu * cu + cv * cv) - 0.22f) - 0.036f;
    return fminf(d_border, fminf(fminf(d_diag1, d_diag2), fminf(d_ring, d_corner)));
}

extern "C" __global__ void __anyhit__omm() {
    float2 bary = optixGetTriangleBarycentrics();
    unsigned int odd = optixGetPrimitiveIndex() & 1u;
    float u = odd ? bary.x : (bary.x + bary.y);
    float v = odd ? (bary.x + bary.y) : bary.y;
    if (filigree_sdf(u, v) > 0.0f) {
        optixIgnoreIntersection();
    }
}

extern "C" __global__ void __intersection__fractal() {
    float3 orig    = optixGetObjectRayOrigin();
    float3 raw_dir = optixGetObjectRayDirection();
    float dir_len  = f3_len(raw_dir);
    if (dir_len <= 1e-8f) return;
    float inv_dir_len = 1.0f / dir_len;
    float3 dir        = f3_mul(raw_dir, inv_dir_len);
    // March in unit object-space arc length s = t_world * dir_len so SDF/DE Lipschitz bounds
    // remain exact under arbitrary uniform TLAS scale (0.05..=50.0), then report s * inv_dir_len.
    float t0 = optixGetRayTmin() * dir_len;
    float t1 = optixGetRayTmax() * dir_len;

    // Ray-AABB clip against [-1.85, 1.85]^3 (matches fractal GAS AABB; accommodates rotated KIFS corners)
    float bmin = -1.85f;
    float bmax = 1.85f;
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

    float fudge = fmaxf(0.2f, fminf(1.2f, params.fractal_r0[1]));
    float formula_safety = (params.fractal_kind == 0u) ? 0.78f : ((params.fractal_kind == 1u) ? 0.85f : 0.90f);
    float step_fudge = fudge * formula_safety;
    float norm_eps = fmaxf(0.00001f, params.fractal_r0[2]);
    int max_steps = (int)fmaxf(16.0f, fminf(512.0f, params.fractal_r0[3]));

    // Distance from camera to ray origin keeps secondary ray epsilon matched to screen-space scale.
    float base_cam_dist = f3_len(f3_sub(optixGetWorldRayOrigin(), make_f3(params.cam_pos)));
    float pixel_angle = (2.0f * fmaxf(params.cam_up[3], 0.05f)) / fmaxf((float)params.height, 64.0f);

    float t = t0;
    float t_prev = t0;
    float trap_prev = 0.0f;
    for (int step = 0; step < 512; ++step) {
        if (step >= max_steps || t > t1) return;
        float3 p = f3_add(orig, f3_mul(dir, t));
        float2 res = eval_fractal(p);
        float dist = res.x;
        float path_dist = base_cam_dist + t * inv_dir_len;
        float eps = fractal_hit_eps(path_dist) * dir_len;
        if (dist < eps) {
            // Refine overstepped interior hits back to the exterior surface boundary via 4-step bisection
            float t_hit = t;
            float trap_hit = res.y;
            if (dist < 0.0f && step > 0) {
                float lo = t_prev;
                float hi = t;
                trap_hit = trap_prev;
                for (int b = 0; b < 4; ++b) {
                    float mid = 0.5f * (lo + hi);
                    float2 mres = eval_fractal(f3_add(orig, f3_mul(dir, mid)));
                    if (mres.x < 0.0f) {
                        hi = mid;
                    } else {
                        lo = mid;
                        trap_hit = mres.y;
                    }
                }
                t_hit = lo;
            }
            float3 p_hit = f3_add(orig, f3_mul(dir, t_hit));
            float hit_path_dist = base_cam_dist + t_hit * inv_dir_len;
            float hit_eps = fractal_hit_eps(hit_path_dist) * dir_len;
            float norm_cone = norm_eps * 250.0f * pixel_angle;
            float n_eps = fmaxf(hit_eps * 0.75f, hit_path_dist * norm_cone * dir_len);
            float3 obj_n = fractal_normal(p_hit, n_eps);
            float ao = fractal_ao(p_hit, obj_n, hit_eps);
            float3 col = f3_mul(trap_palette(trap_hit), ao);
            unsigned int packed =
                ((unsigned int)(fminf(1.0f, col.x) * 255.0f) & 0xFFu) |
                (((unsigned int)(fminf(1.0f, col.y) * 255.0f) & 0xFFu) << 8u) |
                (((unsigned int)(fminf(1.0f, col.z) * 255.0f) & 0xFFu) << 16u);
            optixReportIntersection(
                t_hit * inv_dir_len, 0u,
                __float_as_uint(obj_n.x),
                __float_as_uint(obj_n.y),
                __float_as_uint(obj_n.z),
                packed
            );
            return;
        }
        t_prev = t;
        trap_prev = res.y;
        t += fmaxf(dist * step_fudge, eps * 0.25f);
    }
}

extern "C" __global__ void __intersection__fractal_shadow() {
    float3 orig    = optixGetObjectRayOrigin();
    float3 raw_dir = optixGetObjectRayDirection();
    float dir_len  = f3_len(raw_dir);
    if (dir_len <= 1e-8f) return;
    float inv_dir_len = 1.0f / dir_len;
    float3 dir        = f3_mul(raw_dir, inv_dir_len);
    float t0 = optixGetRayTmin() * dir_len;
    float t1 = optixGetRayTmax() * dir_len;

    float bmin = -1.85f;
    float bmax = 1.85f;
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

    float fudge = fmaxf(0.2f, fminf(1.2f, params.fractal_r0[1]));
    float formula_safety = (params.fractal_kind == 0u) ? 0.78f : ((params.fractal_kind == 1u) ? 0.85f : 0.90f);
    float step_fudge = fudge * formula_safety;
    int max_steps = (int)fmaxf(16.0f, fminf(512.0f, params.fractal_r0[3]));
    float base_cam_dist = f3_len(f3_sub(optixGetWorldRayOrigin(), make_f3(params.cam_pos)));

    float t = t0;
    for (int step = 0; step < 512; ++step) {
        if (step >= max_steps || t > t1) return;
        float dist = eval_fractal(f3_add(orig, f3_mul(dir, t))).x;
        float path_dist = base_cam_dist + t * inv_dir_len;
        float eps = fractal_hit_eps(path_dist) * dir_len * 1.5f;
        if (dist < eps) {
            optixReportIntersection(t * inv_dir_len, 0u);
            return;
        }
        t += fmaxf(dist * step_fudge, eps * 0.25f);
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
        t, 0u,
        __float_as_uint(obj_n.x),
        __float_as_uint(obj_n.y),
        __float_as_uint(obj_n.z),
        0xFFE0E0E0u
    );
}

extern "C" __global__ void __intersection__sphere_shadow() {
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
    optixReportIntersection(t, 0u);
}

extern "C" __global__ void __intersection__csg() {
    if (params.csg_instances == 0ull || params.csg_compounds == 0ull) return;
    unsigned int inst_idx = optixGetInstanceId();
    if (inst_idx >= params.csg_instance_count) return;

    uint4 meta = load_csg_instance_meta(inst_idx);
    unsigned int compound_id  = meta.x;
    unsigned int mat_override = meta.z;
    unsigned int flags        = meta.w;
    if ((flags & CSG_INST_FLAG_VISIBLE) == 0u) return;
    if (compound_id >= params.csg_compound_count) return;

    float3 orig = optixGetObjectRayOrigin();
    float3 dir  = optixGetObjectRayDirection();
    float tmin  = optixGetRayTmin();
    float tmax  = optixGetRayTmax();

    // Pass CsgSceneView by value and return CsgHit by value (zero &view/&hit .local spills),
    // skipping redundant compound AABB test already performed by OptiX BVH hardware.
    CsgHit hit = csg_trace_compound_val(make_csg_view(), compound_id, orig, dir, tmin, tmax, true);
    if (hit.t >= 0.0f) {
        unsigned int mat_id = (mat_override != CSG_MAT_OVERRIDE_NONE)
            ? (mat_override & 0xFFFFu)
            : (hit.mat_id & 0xFFFFu);
        unsigned int mat_sem = (mat_id & 0xFFFFu) | ((hit.sem_id & 0xFFFFu) << 16u);
        optixReportIntersection(
            hit.t,
            1u,
            __float_as_uint(hit.normal.x),
            __float_as_uint(hit.normal.y),
            __float_as_uint(hit.normal.z),
            mat_sem,
            hit.part_id
        );
    }
}

extern "C" __global__ void __intersection__csg_shadow() {
    if (params.csg_instances == 0ull || params.csg_compounds == 0ull) return;
    unsigned int inst_idx = optixGetInstanceId();
    if (inst_idx >= params.csg_instance_count) return;

    uint4 meta = load_csg_instance_meta(inst_idx);
    unsigned int compound_id = meta.x;
    unsigned int flags       = meta.w;
    if ((flags & (CSG_INST_FLAG_VISIBLE | CSG_INST_FLAG_CAST_SHADOW))
        != (CSG_INST_FLAG_VISIBLE | CSG_INST_FLAG_CAST_SHADOW)) {
        return;
    }
    if (compound_id >= params.csg_compound_count) return;

    float3 orig = optixGetObjectRayOrigin();
    float3 dir  = optixGetObjectRayDirection();
    float tmin  = optixGetRayTmin();
    float tmax  = optixGetRayTmax();

    if (csg_trace_shadow(make_csg_view(), compound_id, orig, dir, tmin, tmax, true)) {
        optixReportIntersection(tmin, 0u);
    }
}

extern "C" __global__ void __intersection__csg_prim() {
    if (params.csg_instances == 0ull || params.csg_prims == 0ull) return;
    unsigned int inst_idx = optixGetInstanceId();
    if (inst_idx >= params.csg_instance_count) return;

    uint4 meta = load_csg_instance_meta(inst_idx);
    unsigned int mat_override = meta.z;
    unsigned int flags        = meta.w;
    if ((flags & CSG_INST_FLAG_VISIBLE) == 0u) return;

    // Host sets CustomPrimitives::primitive_offset = first_prim_idx for both SINGLE_PRIM and
    // Multi-AABB PURE_UNION GASes, so optixGetPrimitiveIndex() is the exact global prim_idx.
    unsigned int prim_idx = optixGetPrimitiveIndex();
    float3 orig = optixGetObjectRayOrigin();
    float3 dir  = optixGetObjectRayDirection();
    float tmin  = optixGetRayTmin();
    float tmax  = optixGetRayTmax();

    CsgHit hit = csg_intersect_prim_idx(
        reinterpret_cast<const CsgGpuPrim*>(params.csg_prims),
        reinterpret_cast<const float4*>(params.csg_aux_planes),
        prim_idx,
        orig, dir, tmin, tmax
    );
    if (hit.t >= 0.0f) {
        unsigned int mat_id = (mat_override != CSG_MAT_OVERRIDE_NONE)
            ? (mat_override & 0xFFFFu)
            : (hit.mat_id & 0xFFFFu);
        unsigned int mat_sem = (mat_id & 0xFFFFu) | ((hit.sem_id & 0xFFFFu) << 16u);
        optixReportIntersection(
            hit.t,
            1u,
            __float_as_uint(hit.normal.x),
            __float_as_uint(hit.normal.y),
            __float_as_uint(hit.normal.z),
            mat_sem,
            hit.part_id
        );
    }
}

extern "C" __global__ void __intersection__csg_prim_shadow() {
    if (params.csg_instances == 0ull || params.csg_prims == 0ull) return;
    unsigned int inst_idx = optixGetInstanceId();
    if (inst_idx >= params.csg_instance_count) return;

    uint4 meta = load_csg_instance_meta(inst_idx);
    unsigned int flags = meta.w;
    if ((flags & (CSG_INST_FLAG_VISIBLE | CSG_INST_FLAG_CAST_SHADOW))
        != (CSG_INST_FLAG_VISIBLE | CSG_INST_FLAG_CAST_SHADOW)) {
        return;
    }

    unsigned int prim_idx = optixGetPrimitiveIndex();
    float3 orig = optixGetObjectRayOrigin();
    float3 dir  = optixGetObjectRayDirection();
    float tmin  = optixGetRayTmin();
    float tmax  = optixGetRayTmax();

    if (csg_intersect_prim_idx_shadow(
            reinterpret_cast<const CsgGpuPrim*>(params.csg_prims),
            reinterpret_cast<const float4*>(params.csg_aux_planes),
            prim_idx,
            orig, dir, tmin, tmax)) {
        optixReportIntersection(tmin, 0u);
    }
}

extern "C" __global__ void __closesthit__csg() {
    float3 obj_n = make_float3(
        __uint_as_float(optixGetAttribute_0()),
        __uint_as_float(optixGetAttribute_1()),
        __uint_as_float(optixGetAttribute_2())
    );
    float3 world_n = f3_norm(optixTransformNormalFromObjectToWorldSpace(obj_n));
    unsigned int mat_sem = optixGetAttribute_3();
    unsigned int part_id = optixGetAttribute_4();

    optixSetPayload_0(2u);
    optixSetPayload_1(__float_as_uint(optixGetRayTmax()));
    optixSetPayload_2(__float_as_uint(world_n.x));
    optixSetPayload_3(__float_as_uint(world_n.y));
    optixSetPayload_4(__float_as_uint(world_n.z));
    optixSetPayload_5(mat_sem);
    optixSetPayload_6(optixGetInstanceId());
    optixSetPayload_7(part_id);
}

