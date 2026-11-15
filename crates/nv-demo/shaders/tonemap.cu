
extern "C" __global__ void tonemap_f32_to_rgba8(
    const float4* src,
    uchar4* dst,
    unsigned int width,
    unsigned int height,
    float exposure,
    unsigned int mode
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= width || y >= height) return;
    unsigned int idx = y * width + x;
    float4 c = src[idx];
    if (mode == 2u) {
        // Normal AOV: map [-1, 1] -> [0, 255]
        auto to_u8 = [](float v) -> unsigned char {
            float s = fmaxf(0.0f, fminf(1.0f, v * 0.5f + 0.5f));
            return (unsigned char)(s * 255.0f + 0.5f);
        };
        dst[idx] = make_uchar4(to_u8(c.x), to_u8(c.y), to_u8(c.z), 255);
        return;
    }
    if (mode == 1u) {
        // Albedo AOV: linear -> sRGB
        auto to_srgb = [](float v) -> unsigned char {
            float s = powf(fmaxf(0.0f, fminf(1.0f, v)), 1.0f / 2.2f);
            return (unsigned char)(s * 255.0f + 0.5f);
        };
        dst[idx] = make_uchar4(to_srgb(c.x), to_srgb(c.y), to_srgb(c.z), 255);
        return;
    }
    float3 x3 = make_float3(c.x * exposure, c.y * exposure, c.z * exposure);
    auto aces = [](float v) -> float {
        float num = v * (2.51f * v + 0.03f);
        float den = v * (2.43f * v + 0.59f) + 0.14f;
        float r = fmaxf(0.0f, fminf(1.0f, num / den));
        return powf(r, 1.0f / 2.2f);
    };
    dst[idx] = make_uchar4(
        (unsigned char)(aces(x3.x) * 255.0f + 0.5f),
        (unsigned char)(aces(x3.y) * 255.0f + 0.5f),
        (unsigned char)(aces(x3.z) * 255.0f + 0.5f),
        255
    );
}
