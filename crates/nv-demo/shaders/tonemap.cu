
static __forceinline__ __device__ uchar4 eval_tonemap_pixel(float4 c, float exposure, unsigned int mode) {
    if (mode == 2u) {
        // Normal AOV: map [-1, 1] -> [0, 255]
        auto to_u8 = [](float v) -> unsigned char {
            float s = fmaxf(0.0f, fminf(1.0f, v * 0.5f + 0.5f));
            return (unsigned char)(s * 255.0f + 0.5f);
        };
        return make_uchar4(to_u8(c.x), to_u8(c.y), to_u8(c.z), 255);
    }
    if (mode == 1u) {
        // Albedo AOV: linear -> sRGB
        auto to_srgb = [](float v) -> unsigned char {
            float s = powf(fmaxf(0.0f, fminf(1.0f, v)), 1.0f / 2.2f);
            return (unsigned char)(s * 255.0f + 0.5f);
        };
        return make_uchar4(to_srgb(c.x), to_srgb(c.y), to_srgb(c.z), 255);
    }
    if (mode == 3u || mode == 4u) {
        // Depth / Motion AOV: already normalized [0, 1]
        auto to_u8 = [](float v) -> unsigned char {
            float s = fmaxf(0.0f, fminf(1.0f, v));
            return (unsigned char)(s * 255.0f + 0.5f);
        };
        return make_uchar4(to_u8(c.x), to_u8(c.y), to_u8(c.z), 255);
    }
    float3 x3 = make_float3(c.x * exposure, c.y * exposure, c.z * exposure);
    auto aces = [](float v) -> float {
        float num = v * (2.51f * v + 0.03f);
        float den = v * (2.43f * v + 0.59f) + 0.14f;
        float r = fmaxf(0.0f, fminf(1.0f, num / den));
        return powf(r, 1.0f / 2.2f);
    };
    return make_uchar4(
        (unsigned char)(aces(x3.x) * 255.0f + 0.5f),
        (unsigned char)(aces(x3.y) * 255.0f + 0.5f),
        (unsigned char)(aces(x3.z) * 255.0f + 0.5f),
        255
    );
}

extern "C" __global__ void tonemap_f32_to_rgba8(
    const float4* src,
    uchar4* dst,
    unsigned int width,
    unsigned int height,
    float exposure,
    unsigned int mode,
    const float4* right_f32,
    float split_x,
    int right_is_aov
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= width || y >= height) return;
    unsigned int idx = y * width + x;

    if (split_x >= 0.0f && right_f32 != nullptr) {
        int split_px = (int)(split_x * (float)width + 0.5f);
        int ix = (int)x;
        if (ix == split_px || ix + 1 == split_px) {
            // Crisp 2px vertical A/B divider
            dst[idx] = make_uchar4(235, 240, 245, 255);
            return;
        }
        if (ix > split_px) {
            unsigned int r_mode = (right_is_aov > 0) ? (unsigned int)right_is_aov : 0u;
            dst[idx] = eval_tonemap_pixel(right_f32[idx], exposure, r_mode);
            return;
        }
    }

    dst[idx] = eval_tonemap_pixel(src[idx], exposure, mode);
}

static __forceinline__ __device__ unsigned short f32_to_f16(float v) {
    unsigned short h;
    asm("cvt.rn.f16.f32 %0, %1;" : "=h"(h) : "f"(v));
    return h;
}

static __forceinline__ __device__ float sanitize_range(float v, float lo, float hi) {
    if (!(v == v)) return 0.0f;
    return fmaxf(lo, fminf(hi, v));
}

static __forceinline__ __device__ ushort4 pack_f16_4(float x, float y, float z, float w) {
    return make_ushort4(f32_to_f16(x), f32_to_f16(y), f32_to_f16(z), f32_to_f16(w));
}

extern "C" __global__ void pack_dlss_rr_aovs_f16(
    const float4* src_color,
    const float4* src_diffuse,
    const float4* src_specular,
    const float4* src_normal,
    ushort4* dst_color,
    ushort4* dst_diffuse,
    ushort4* dst_specular,
    ushort4* dst_normal,
    unsigned int width,
    unsigned int height
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= width || y >= height) return;
    unsigned int idx = y * width + x;

    float4 c = src_color[idx];
    dst_color[idx] = pack_f16_4(
        sanitize_range(c.x, 0.0f, 65000.0f),
        sanitize_range(c.y, 0.0f, 65000.0f),
        sanitize_range(c.z, 0.0f, 65000.0f),
        1.0f
    );

    float4 d = src_diffuse[idx];
    dst_diffuse[idx] = pack_f16_4(
        sanitize_range(d.x, 0.0f, 1.0f),
        sanitize_range(d.y, 0.0f, 1.0f),
        sanitize_range(d.z, 0.0f, 1.0f),
        1.0f
    );

    float4 s = src_specular[idx];
    dst_specular[idx] = pack_f16_4(
        sanitize_range(s.x, 0.0f, 1.0f),
        sanitize_range(s.y, 0.0f, 1.0f),
        sanitize_range(s.z, 0.0f, 1.0f),
        1.0f
    );

    float4 n = src_normal[idx];
    dst_normal[idx] = pack_f16_4(
        sanitize_range(n.x, -1.0f, 1.0f),
        sanitize_range(n.y, -1.0f, 1.0f),
        sanitize_range(n.z, -1.0f, 1.0f),
        sanitize_range(n.w, 0.0f, 1.0f)
    );
}
