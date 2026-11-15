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
    float cam_pos[4];     // xyz, pad
    float cam_forward[4]; // xyz, pad
    float cam_right[4];   // xyz, pad
    float cam_up[4];      // xyz, tan_half_fov in .w
    float sun_dir[4];     // xyz, roughness in .w
    float fractal_p0[4];  // [power/scale, bailout, fold_limit/julia_mode, min_radius]
    float fractal_p1[4];  // [fixed_radius/rot1, rot2, max_iter, 0]
    float fractal_p2[4];  // [c.x, c.y, c.z, 0] (julia_c or kifs offset)
    float fractal_r0[4];  // [detail_eps, fudge, normal_eps, march_steps]
    float fractal_r1[4];  // [metallic, trap_scale, palette_shift, sun_intensity]
    float time;
    unsigned int enable_shadows;
    unsigned int fractal_kind; // 0=Mandelbulb, 1=Mandelbox, 2=KIFS Octahedron
    unsigned int bounces;
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

static __forceinline__ __device__ float2 sdf_mandelbulb(float3 q) {
    float power = params.fractal_p0[0];
    float bailout = fmaxf(2.0f, params.fractal_p0[1]);
    bool julia = params.fractal_p0[2] > 0.5f;
    int max_iter = (int)fmaxf(2.0f, fminf(28.0f, params.fractal_p1[2]));
    float3 c_add = julia ? make_f3(params.fractal_p2) : q;

    float3 z = q;
    float dr = 1.0f;
    float r = f3_len(z);
    float trap = 1e6f;
    for (int i = 0; i < 28; ++i) {
        if (i >= max_iter || r > bailout) break;
        if (r < 1e-6f) return make_float2(0.0f, trap);
        float theta = acosf(fmaxf(-1.0f, fminf(1.0f, z.z / r)));
        float phi = atan2f(z.y, z.x);
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
    float min_r2 = min_r * min_r;
    float fixed_r2 = fixed_r * fixed_r;

    float3 z = q;
    float dr = 1.0f;
    float trap = 1e6f;
    for (int i = 0; i < 28; ++i) {
        if (i >= max_iter) break;
        // Box fold
        z.x = fmaxf(-fold, fminf(fold, z.x)) * 2.0f - z.x;
        z.y = fmaxf(-fold, fminf(fold, z.y)) * 2.0f - z.y;
        z.z = fmaxf(-fold, fminf(fold, z.z)) * 2.0f - z.z;
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
        if (z.z > 0.5f * off.z * (sc - 1.0f)) {
            z.z -= off.z * (sc - 1.0f);
        }
        s *= sc;
        float r2 = f3_dot(z, z);
        trap = fminf(trap, r2);
        if (r2 > bailout2) break;
    }
    return make_float2((f3_len(z) - 0.1f) / s, sqrtf(fmaxf(trap, 0.0f)));
}

static __forceinline__ __device__ float2 eval_fractal(float3 p) {
    if (params.fractal_kind == 1u) {
        float2 r = sdf_mandelbox(f3_mul(p, 2.2f));
        return make_float2(r.x * (1.0f / 2.2f), r.y);
    } else if (params.fractal_kind == 2u) {
        return sdf_kifs_octahedron(p);
    }
    return sdf_mandelbulb(p);
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

static __forceinline__ __device__ float3 sky_color(float3 dir, float3 sun_dir) {
    float t = fmaxf(0.0f, fminf(1.0f, 0.5f * (dir.y + 1.0f)));
    float3 horizon = make_float3(0.55f, 0.72f, 0.92f);
    float3 zenith  = make_float3(0.08f, 0.22f, 0.52f);
    float3 col = f3_add(f3_mul(horizon, 1.0f - t), f3_mul(zenith, t));
    float sun_dot = fmaxf(0.0f, f3_dot(dir, sun_dir));
    float sun_glow = powf(sun_dot, 32.0f) * 0.6f + powf(sun_dot, 512.0f) * 8.0f;
    return f3_add(col, make_float3(sun_glow, sun_glow * 0.92f, sun_glow * 0.75f));
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

    for (unsigned int s = 0; s < spp; ++s) {
        float jx = rand01(seed) - 0.5f;
        float jy = rand01(seed) - 0.5f;
        float u = ((float)idx.x + 0.5f + jx) / (float)params.width * 2.0f - 1.0f;
        float v = 1.0f - ((float)idx.y + 0.5f + jy) / (float)params.height * 2.0f;

        float3 dir = f3_norm(f3_add(
            cam_fwd,
            f3_add(f3_mul(cam_rgt, u * aspect * tan_fov), f3_mul(cam_up, v * tan_fov))
        ));
        float3 origin = cam_pos;

        float3 throughput = make_float3(1.0f, 1.0f, 1.0f);
        float3 sample_col = make_float3(0.0f, 0.0f, 0.0f);

        int max_bounces = (int)params.bounces;
        for (int bounce = 0; bounce < 8; ++bounce) {
            if (bounce >= max_bounces) break;
            unsigned int p0 = 0, p1 = 0, p2 = 0, p3 = 0, p4 = 0, p5 = 0, p6 = 0, p7 = 0;
            trace_primary(origin, dir, p0, p1, p2, p3, p4, p5, p6, p7);

            if (p0 == 0u) {
                float3 sky = sky_color(dir, sun_dir);
                sample_col = f3_add(sample_col, f3_mul3(throughput, sky));
                if (bounce == 0 && s == 0) {
                    first_albedo = sky;
                    first_normal = make_float3(-dir.x, -dir.y, -dir.z);
                }
                break;
            }

            float hit_t = __uint_as_float(p1);
            float3 n = make_float3(__uint_as_float(p2), __uint_as_float(p3), __uint_as_float(p4));
            if (f3_dot(n, dir) > 0.0f) n = f3_mul(n, -1.0f);
            float3 base_col = make_float3(
                (float)(p5 & 0xFFu) / 255.0f,
                (float)((p5 >> 8u) & 0xFFu) / 255.0f,
                (float)((p5 >> 16u) & 0xFFu) / 255.0f
            );
            unsigned int inst_id = p6;
            float3 hit_pos = f3_add(origin, f3_mul(dir, hit_t));

            float metallic = 0.0f;
            float rough = params.sun_dir[3];
            if (inst_id == 0u) {
                // Checkerboard floor
                int cx = (int)floorf(hit_pos.x * 0.75f);
                int cz = (int)floorf(hit_pos.z * 0.75f);
                bool even = ((cx + cz) & 1) == 0;
                base_col = even ? make_float3(0.82f, 0.84f, 0.88f) : make_float3(0.14f, 0.16f, 0.20f);
                metallic = 0.35f;
            } else {
                // Fractal custom primitive uses orbit-trap base_col from p5
                metallic = params.fractal_r1[0];
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
                    float3 sdir = f3_norm(f3_add(sun_dir, f3_mul(jitter, 0.06f)));
                    occluded = trace_shadow(f3_add(hit_pos, f3_mul(n, 0.003f)), sdir, 1000.0f);
                }
                if (!occluded) {
                    float sun_i = params.fractal_r1[3];
                    float3 sun_rad = make_float3(sun_i, sun_i * 0.93f, sun_i * 0.78f);
                    sample_col = f3_add(sample_col, f3_mul3(throughput, f3_mul(f3_mul3(base_col, sun_rad), ndotl * (1.0f - metallic * 0.4f))));
                }
            }

            float3 diff_dir = sample_cosine_hemisphere(n, seed);
            float3 refl_dir = f3_reflect(dir, n);
            float3 spec_dir = f3_norm(f3_add(refl_dir, f3_mul(diff_dir, rough)));
            bool do_spec = (rand01(seed) < (0.20f + 0.65f * metallic));
            dir = do_spec ? spec_dir : diff_dir;
            origin = f3_add(hit_pos, f3_mul(n, 0.003f));
            throughput = f3_mul3(throughput, base_col);
        }

        accum_col = f3_add(accum_col, sample_col);
    }

    float inv_spp = 1.0f / (float)spp;
    float3 final_col = f3_mul(accum_col, inv_spp);

    unsigned int out_idx = idx.y * params.width + idx.x;
    float4* color_buf  = (float4*)params.out_color_f32;
    float4* albedo_buf = (float4*)params.out_albedo_f32;
    float4* normal_buf = (float4*)params.out_normal_f32;
    color_buf[out_idx]  = make_float4(final_col.x, final_col.y, final_col.z, 1.0f);
    albedo_buf[out_idx] = make_float4(first_albedo.x, first_albedo.y, first_albedo.z, 1.0f);
    normal_buf[out_idx] = make_float4(first_normal.x, first_normal.y, first_normal.z, 1.0f);
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

