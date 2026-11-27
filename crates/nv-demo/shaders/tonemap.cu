// `aov_mode` of the hidden-line wireframe pass (`wireframe::AOV_WIREFRAME`, raytracer.cu): its
// display colour is already linear [0, 1] like the albedo AOV.
#define AOV_WIREFRAME 11u

static __forceinline__ __device__ float3 world_pos_to_vis_rgb(float4 c) {
    // `c.w < 0.5f` marks sky miss -> crisp black; hits carry exact metric world coordinates in `c.xyz`.
    if (c.w < 0.5f) {
        return make_float3(0.0f, 0.0f, 0.0f);
    }
    auto map_axis = [](float v) -> float {
        // Signed-log macro position tint + subtle 20 m repeating spatial contour for 3D continuity.
        float macro = 0.5f + 0.5f * (v / (1.0f + fabsf(v) * 0.018f));
        float cell = v * 0.05f - floorf(v * 0.05f);
        float grid = 0.78f + 0.22f * cell;
        return fmaxf(0.0f, fminf(1.0f, macro * grid));
    };
    return make_float3(map_axis(c.x), map_axis(c.y), map_axis(c.z));
}

static __forceinline__ __device__ uchar4 eval_tonemap_pixel(float4 c, float exposure, unsigned int mode) {
    if (mode == 2u) {
        // Normal AOV: map [-1, 1] -> [0, 255]
        auto to_u8 = [](float v) -> unsigned char {
            float s = fmaxf(0.0f, fminf(1.0f, v * 0.5f + 0.5f));
            return (unsigned char)(s * 255.0f + 0.5f);
        };
        return make_uchar4(to_u8(c.x), to_u8(c.y), to_u8(c.z), 255);
    }
    if (mode == 1u || mode == AOV_WIREFRAME) {
        // Albedo AOV / wireframe display colour: linear -> sRGB
        auto to_srgb = [](float v) -> unsigned char {
            float s = powf(fmaxf(0.0f, fminf(1.0f, v)), 1.0f / 2.2f);
            return (unsigned char)(s * 255.0f + 0.5f);
        };
        return make_uchar4(to_srgb(c.x), to_srgb(c.y), to_srgb(c.z), 255);
    }
    if (mode == 7u) {
        // WorldPos AOV: raw metric 3D position -> readable signed-log + 20 m grid visualization
        float3 v = world_pos_to_vis_rgb(c);
        return make_uchar4(
            (unsigned char)(v.x * 255.0f + 0.5f),
            (unsigned char)(v.y * 255.0f + 0.5f),
            (unsigned char)(v.z * 255.0f + 0.5f),
            255
        );
    }
    if (mode >= 3u && mode <= 10u) {
        // Depth / Motion / SemanticId / InstanceId / Range / Material / Cryptomatte: normalized [0, 1]
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

static __forceinline__ __device__ ushort4 eval_scrgb_pixel(float4 c, float exposure, unsigned int mode) {
    // scRGB linear Rec.709: 1.0 = 80 nits, 2.5375 = 203 nits (paper white), 12.5 = 1000 nits peak.
    const float paper_white_scrgb = 203.0f / 80.0f;
    if (mode == 2u) {
        float r = sanitize_range(c.x * 0.5f + 0.5f, 0.0f, 1.0f);
        float g = sanitize_range(c.y * 0.5f + 0.5f, 0.0f, 1.0f);
        float b = sanitize_range(c.z * 0.5f + 0.5f, 0.0f, 1.0f);
        return pack_f16_4(
            powf(r, 2.2f) * paper_white_scrgb,
            powf(g, 2.2f) * paper_white_scrgb,
            powf(b, 2.2f) * paper_white_scrgb,
            1.0f
        );
    }
    if (mode == 1u || mode == AOV_WIREFRAME) {
        return pack_f16_4(
            sanitize_range(c.x, 0.0f, 1.0f) * paper_white_scrgb,
            sanitize_range(c.y, 0.0f, 1.0f) * paper_white_scrgb,
            sanitize_range(c.z, 0.0f, 1.0f) * paper_white_scrgb,
            1.0f
        );
    }
    if (mode == 7u) {
        float3 v = world_pos_to_vis_rgb(c);
        return pack_f16_4(
            powf(v.x, 2.2f) * paper_white_scrgb,
            powf(v.y, 2.2f) * paper_white_scrgb,
            powf(v.z, 2.2f) * paper_white_scrgb,
            1.0f
        );
    }
    if (mode >= 3u && mode <= 10u) {
        float r = sanitize_range(c.x, 0.0f, 1.0f);
        float g = sanitize_range(c.y, 0.0f, 1.0f);
        float b = sanitize_range(c.z, 0.0f, 1.0f);
        return pack_f16_4(
            powf(r, 2.2f) * paper_white_scrgb,
            powf(g, 2.2f) * paper_white_scrgb,
            powf(b, 2.2f) * paper_white_scrgb,
            1.0f
        );
    }
    // HDR10 grading lives here: 203-nit paper white + 750->1000-nit soft rolloff. The PQ10 video
    // encoder (`scrgb_f16_to_p010_pq` in gpu_export.rs) consumes this scRGB output as-is and applies
    // no rolloff of its own, so any other scRGB source would clip at 10,000 nits there.
    float rn = sanitize_range(c.x * exposure, 0.0f, 1000.0f) * 203.0f;
    float gn = sanitize_range(c.y * exposure, 0.0f, 1000.0f) * 203.0f;
    float bn = sanitize_range(c.z * exposure, 0.0f, 1000.0f) * 203.0f;
    float mx = fmaxf(rn, fmaxf(gn, bn));
    const float knee = 750.0f;
    const float peak = 1000.0f;
    if (mx > knee) {
        float excess = mx - knee;
        float span = peak - knee;
        float compressed = knee + span * (excess / (excess + span));
        float s = compressed / mx;
        rn *= s;
        gn *= s;
        bn *= s;
    }
    return pack_f16_4(rn * 0.0125f, gn * 0.0125f, bn * 0.0125f, 1.0f);
}

extern "C" __global__ void tonemap_f32_to_scrgb_f16(
    const float4* src,
    ushort4* dst,
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
            const float div = 203.0f / 80.0f;
            dst[idx] = pack_f16_4(div, div, div, 1.0f);
            return;
        }
        if (ix > split_px) {
            unsigned int r_mode = (right_is_aov > 0) ? (unsigned int)right_is_aov : 0u;
            dst[idx] = eval_scrgb_pixel(right_f32[idx], exposure, r_mode);
            return;
        }
    }

    dst[idx] = eval_scrgb_pixel(src[idx], exposure, mode);
}

// ---------------------------------------------------------------------------------------------
// Overlay raster (overlay.rs). One device-resident segment list (`overlay_segment.cuh`: CPU
// primitives uploaded first, GPU providers append with `nv_overlay_push`), one antialiased line
// pass, one composite with temporal stabilisation:
//   ovl_splat      one block per segment: world segments are projected with the pinhole camera
//                  (other lenses count an UNPROJECTABLE fault), the shared filter is applied
//                  (selected instance, radius), then every pixel near the segment gets the
//                  analytic coverage of a constant-width stroke (dashes, fade, optional depth
//                  test against the primary-hit depth) as `coverage << 24 | style` (max wins).
//                  Under DLSS the raster is the output resolution and the depth (render
//                  resolution, DLSS outputs none) is read at the nearest texel.
//   ovl_composite  temporal stabilisation (the previous frame's coverage reprojected with the
//                  motion vectors, clamped to the current 3x3 coverage range, exponentially
//                  blended) and a linear-light blend into the tonemapped outputs.
// ---------------------------------------------------------------------------------------------

#if !defined(NV_DEMO_OVERLAY_SEGMENT_CUH)
#include "overlay_segment.cuh"
#endif

struct OvlStyle {
    float r, g, b, a;
    float width;
    float dash_on;
    float dash_off;
    unsigned int depth_mode;
};

// Camera of the frame, for projecting world-space segments (pinhole only).
struct OvlCamera {
    float eye[3];
    float tan_half_fov;
    float fwd[3];
    float aspect;
    float right[3];
    unsigned int pinhole;
    float up[3];
    unsigned int sel_count;      // valid keys in the selection table (`sel`, OVL_MAX_SELECTED)
    float radius;                // WithinRadius scope (view depth / distance), <= 0 = off
    float min_px;                // line LOD: records whose proj_size_px is below this are not drawn
    unsigned int scope_selected; // 1 = draw only records whose inst_id is in the selection table
    float axes_min_px;           // part-level axes LOD: a part's triad needs this projected size (ovl_axes)
};

#define OVL_DEPTH_ALWAYS 0u
#define OVL_DEPTH_SKIP_HIDDEN 1u
#define OVL_DEPTH_FADE_HIDDEN 2u
#define OVL_MAX_SELECTED 64u

// True when `key` is one of the first `n` selection keys (the selection is small: a linear scan
// of at most OVL_MAX_SELECTED words, read through the read-only cache).
static __forceinline__ __device__ bool ovl_is_selected(const unsigned int* sel, unsigned int n, unsigned int key) {
    n = min(n, OVL_MAX_SELECTED);
    for (unsigned int i = 0; i < n; ++i) {
        if (__ldg(sel + i) == key) return true;
    }
    return false;
}

// Coverage of a stroke `width` px wide whose centre line is `dist` px from the pixel centre:
// smoothstep over one pixel of falloff; strokes thinner than a pixel fade in opacity instead.
static __forceinline__ __device__ float ovl_coverage(float dist, float width) {
    float t = fminf(1.0f, fmaxf(0.0f, 0.5f * width + 0.5f - dist));
    return t * t * (3.0f - 2.0f * t) * fminf(1.0f, width);
}

// World point -> (x px, y px, planar depth); false behind the camera.
static __forceinline__ __device__ bool ovl_project(
    const OvlCamera& c, const float* p, float w, float h, float3& out
) {
    float v0 = p[0] - c.eye[0], v1 = p[1] - c.eye[1], v2 = p[2] - c.eye[2];
    float z = v0 * c.fwd[0] + v1 * c.fwd[1] + v2 * c.fwd[2];
    if (z <= 1e-3f) return false;
    float xn = (v0 * c.right[0] + v1 * c.right[1] + v2 * c.right[2]) / (z * c.tan_half_fov * c.aspect);
    float yn = (v0 * c.up[0] + v1 * c.up[1] + v2 * c.up[2]) / (z * c.tan_half_fov);
    out = make_float3((xn * 0.5f + 0.5f) * w, (0.5f - 0.5f * yn) * h, z);
    return true;
}

extern "C" __global__ void ovl_splat(
    const NvOverlaySegment* segs,
    unsigned int* counter,
    unsigned int cap,
    const OvlStyle* styles,
    unsigned int style_count,
    const OvlCamera* cam,
    const unsigned int* sel,
    const float* depth,
    unsigned int depth_w,
    unsigned int depth_h,
    unsigned int* cov,
    unsigned int width,
    unsigned int height
) {
    const unsigned int n = min(counter[0], cap);
    const OvlCamera c = *cam;
    for (unsigned int si = blockIdx.x; si < n; si += gridDim.x) {
        const NvOverlaySegment s = segs[si];
        if (s.style >= style_count) continue;
        // Kind dispatch: straight kinds share the segment raster; analytic kinds (arc, ellipse,
        // conic) slot in here with their own exact distance, until then they are a fault.
        if (s.kind != NV_OVERLAY_KIND_SEGMENT && s.kind != NV_OVERLAY_KIND_POLYLINE_CHUNK) {
            if (threadIdx.x == 0) atomicAdd(&counter[3], 1u);
            continue;
        }
        if (c.scope_selected != 0u && !ovl_is_selected(sel, c.sel_count, s.inst_id)) continue;
        const OvlStyle st = styles[s.style];
        float3 pa, pb;
        if ((s.flags & NV_OVERLAY_FLAG_WORLD) != 0u) {
            bool ok = c.pinhole != 0u
                && ovl_project(c, s.a, (float)width, (float)height, pa)
                && ovl_project(c, s.b, (float)width, (float)height, pb);
            if (!ok) {
                if (c.pinhole == 0u && threadIdx.x == 0) atomicAdd(&counter[2], 1u);
                continue;
            }
        } else {
            pa = make_float3(s.a[0], s.a[1], s.a[2]);
            pb = make_float3(s.b[0], s.b[1], s.b[2]);
        }
        if (c.radius > 0.0f && fminf(pa.z, pb.z) > c.radius && pa.z > 0.0f) continue;
        // Line LOD (why: features smaller than a couple of pixels are undersampled and would
        // shimmer): fade from 0 at `min_px` to full at `3 * min_px` of projected feature size.
        float fade = 1.0f;
        if (c.min_px > 0.0f && s.proj_size_px > 0.0f) {
            const float lt = fminf(1.0f, fmaxf(0.0f, (s.proj_size_px - c.min_px) / (2.0f * c.min_px)));
            fade = lt * lt * (3.0f - 2.0f * lt);
        }
        const bool selected = (s.flags & NV_OVERLAY_FLAG_SELECTED) != 0u;
        if (selected) fade = 1.0f;
        if (fade <= 0.0f) continue;
        const bool flag_hidden = (s.flags & NV_OVERLAY_FLAG_HIDDEN) != 0u;
        const bool depth_test = (s.flags & NV_OVERLAY_FLAG_NO_DEPTH_TEST) == 0u;
        // Width in px; world-width styles are projected at the nearer end (constant along the
        // segment, which is short for every world provider).
        float width_px = st.width;
        if ((s.flags & NV_OVERLAY_FLAG_SCREEN_CONSTANT_WIDTH) == 0u) {
            const float zn = fmaxf(fminf(pa.z, pb.z), 1e-3f);
            width_px = st.width * (float)height / (2.0f * fmaxf(c.tan_half_fov, 1e-4f) * zn);
        }
        if (selected) width_px *= 1.5f;
        const float reach = 0.5f * width_px + 0.5f;
        const float dx = pb.x - pa.x, dy = pb.y - pa.y;
        const float len = sqrtf(dx * dx + dy * dy);
        // Walk the major axis; each column / row covers the band around the segment there.
        const bool xmajor = fabsf(dx) >= fabsf(dy);
        const float a0 = xmajor ? pa.x : pa.y, a1 = xmajor ? pb.x : pb.y;
        const float b0 = xmajor ? pa.y : pa.x, b1 = xmajor ? pb.y : pb.x;
        const float da = a1 - a0;
        const float slope = (fabsf(da) > 1e-6f) ? (b1 - b0) / da : 0.0f;
        const float band = (len > 1e-6f) ? reach * len / fmaxf(fabsf(da), 1e-6f) : reach;
        const int c0 = (int)floorf(fminf(a0, a1) - reach);
        const int c1 = (int)ceilf(fmaxf(a0, a1) + reach);
        if (c1 - c0 > 8192) continue;  // degenerate projection (point near the eye plane)
        const int rows = 2 * (int)ceilf(band + 1.0f) + 1;
        const long long total = (long long)(c1 - c0 + 1) * (long long)rows;
        for (long long k = threadIdx.x; k < total; k += blockDim.x) {
            const int ci = c0 + (int)(k / rows);
            const float cc = (float)ci + 0.5f;
            const float tc = (fabsf(da) > 1e-6f) ? fminf(1.0f, fmaxf(0.0f, (cc - a0) / da)) : 0.0f;
            const float bc = b0 + slope * (tc * da);
            const int ri = (int)floorf(bc) - rows / 2 + (int)(k % rows);
            const int px = xmajor ? ci : ri, py = xmajor ? ri : ci;
            if (px < 0 || py < 0 || px >= (int)width || py >= (int)height) continue;
            const float pcx = (float)px + 0.5f, pcy = (float)py + 0.5f;
            float t = 0.0f;
            if (len > 1e-6f) {
                t = fminf(1.0f, fmaxf(0.0f, ((pcx - pa.x) * dx + (pcy - pa.y) * dy) / (len * len)));
            }
            const float ex = pcx - (pa.x + t * dx), ey = pcy - (pa.y + t * dy);
            float cv = ovl_coverage(sqrtf(ex * ex + ey * ey), width_px) * st.a * fade;
            if (cv <= 0.0f) continue;
            float dash_on = st.dash_on, dash_off = st.dash_off;
            bool hidden = flag_hidden;
            if (!hidden && depth_test && st.depth_mode != OVL_DEPTH_ALWAYS && depth != nullptr
                && pa.z > 0.0f && pb.z > 0.0f) {
                // Perspective-correct view depth along the segment vs. the primary hit.
                const float z = 1.0f / ((1.0f - t) / pa.z + t / pb.z) - s.depth_bias;
                // Nearest depth texel when the overlay raster is finer than the depth (output
                // resolution after DLSS): a real surface's depth, never an edge blend.
                const unsigned int dx = (unsigned int)px * depth_w / width;
                const unsigned int dy = (unsigned int)py * depth_h / height;
                const float d = depth[dy * depth_w + dx];
                hidden = z > d * 1.002f + 1e-3f;
                if (hidden && st.depth_mode == OVL_DEPTH_SKIP_HIDDEN) continue;
            }
            if (hidden || (s.flags & NV_OVERLAY_FLAG_DASHED) != 0u) {
                if (hidden) cv *= 0.35f;
                if (dash_on <= 0.0f) {
                    dash_on = 4.0f;
                    dash_off = 3.0f;
                }
            }
            if (dash_on > 0.0f && dash_off > 0.0f) {
                const float sp = s.s0 + t * len;
                const float period = dash_on + dash_off;
                if (sp - floorf(sp / period) * period > dash_on) continue;
            }
            const unsigned int q = (unsigned int)(fminf(1.0f, cv) * 255.0f + 0.5f);
            if (q == 0u) continue;
            atomicMax(&cov[(unsigned int)py * width + (unsigned int)px], (q << 24u) | (s.style & 0xFFFFFFu));
        }
    }
}

static __forceinline__ __device__ float ovl_f16_to_f32(unsigned short h) {
    float f;
    asm("cvt.f32.f16 %0, %1;" : "=f"(f) : "h"(h));
    return f;
}

// Temporal stabilisation + composite. `hist_in` is the previous frame's stabilised coverage
// (same packing as `cov`), reprojected with `motion` (backward flow in pixels: previous =
// current + motion); its alpha is clamped to the current 3x3 coverage range (no ghosting) and
// blended with weight `blend` for the new frame (exponential history, like a TAA resolve).
// `hist_out` receives the result (ping-pong, never the same buffer as `hist_in`). RGBA8 is
// sRGB-encoded (the tonemap's 1/2.2 curve): decoded, blended in linear light, re-encoded;
// scRGB is linear (1.0 = 80 nits, lines at paper white). `motion` / `hist_in` may be null
// (no stabilisation). `motion` is `motion_w x motion_h` (the render resolution): when this raster
// is the DLSS output resolution, the nearest motion texel is scaled to output pixels.
// `dst_ovl` (output resolution under DLSS, else null) receives the overlay as premultiplied
// linear RGBA for every pixel (zero where nothing is drawn); the display passes (main.rs: the
// DLSS-RR wgpu tonemap, `DlssSrComposite`) blend it after DLSS, so the lines are not upscaled.
extern "C" __global__ void ovl_composite(
    const unsigned int* cov,
    const unsigned int* hist_in,
    unsigned int* hist_out,
    const float2* motion,
    unsigned int motion_w,
    unsigned int motion_h,
    float blend,
    const OvlStyle* styles,
    unsigned int style_count,
    uchar4* dst8,
    ushort4* dst16,
    ushort4* dst_ovl,
    unsigned int width,
    unsigned int height
) {
    const unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    const unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= width || y >= height) return;
    const unsigned int i = y * width + x;
    const unsigned int v = cov[i];
    float a = (float)(v >> 24u) / 255.0f;
    unsigned int si = v & 0xFFFFFFu;
    if (hist_in != nullptr && motion != nullptr && hist_out != nullptr) {
        float lo = 1.0f, hi = 0.0f;
        for (int dy = -1; dy <= 1; ++dy) {
            for (int dx = -1; dx <= 1; ++dx) {
                int nx = min(max((int)x + dx, 0), (int)width - 1);
                int ny = min(max((int)y + dy, 0), (int)height - 1);
                float na = (float)(cov[(unsigned int)ny * width + (unsigned int)nx] >> 24u) / 255.0f;
                lo = fminf(lo, na);
                hi = fmaxf(hi, na);
            }
        }
        // Motion of the nearest motion texel, scaled from its resolution to this raster's.
        const unsigned int mi = (y * motion_h / height) * motion_w + x * motion_w / width;
        const float2 m = motion[mi];
        const float sx = (float)width / (float)motion_w, sy = (float)height / (float)motion_h;
        const int hx = (int)floorf((float)x + 0.5f + m.x * sx);
        const int hy = (int)floorf((float)y + 0.5f + m.y * sy);
        if (hx >= 0 && hy >= 0 && hx < (int)width && hy < (int)height) {
            const unsigned int hv = hist_in[(unsigned int)hy * width + (unsigned int)hx];
            const float ha = fminf(hi, fmaxf(lo, (float)(hv >> 24u) / 255.0f));
            if (v == 0u && ha > 0.0f) si = hv & 0xFFFFFFu;
            a = ha + (a - ha) * blend;
        }
        hist_out[i] = ((unsigned int)(fminf(1.0f, a) * 255.0f + 0.5f) << 24u) | si;
    }
    if (a <= 0.0f || si >= style_count) {
        if (dst_ovl != nullptr) dst_ovl[i] = pack_f16_4(0.0f, 0.0f, 0.0f, 0.0f);
        return;
    }
    const OvlStyle st = styles[si];
    if (dst_ovl != nullptr) {
        // Output-resolution layer (DLSS): premultiplied linear light, blended by the display pass.
        dst_ovl[i] = pack_f16_4(st.r * a, st.g * a, st.b * a, a);
    }
    if (dst8 != nullptr) {
        uchar4 d = dst8[i];
        auto mix8 = [&](unsigned char c, float l) -> unsigned char {
            float lin = powf((float)c / 255.0f, 2.2f);
            float o = lin + (l - lin) * a;
            return (unsigned char)(powf(fmaxf(0.0f, fminf(1.0f, o)), 1.0f / 2.2f) * 255.0f + 0.5f);
        };
        dst8[i] = make_uchar4(mix8(d.x, st.r), mix8(d.y, st.g), mix8(d.z, st.b), d.w);
    }
    if (dst16 != nullptr) {
        const float pw = 203.0f / 80.0f;
        ushort4 d = dst16[i];
        auto mix16 = [&](unsigned short hh, float l) -> float {
            float c = ovl_f16_to_f32(hh);
            return c + (l * pw - c) * a;
        };
        dst16[i] = pack_f16_4(mix16(d.x, st.r), mix16(d.y, st.g), mix16(d.z, st.b), ovl_f16_to_f32(d.w));
    }
}

// Object axes provider (`overlay::AxesProvider`, CPU mirror `overlay::axes_emit`, unit-tested).
// Why on the device: the instance transforms live here (`csg_instances`, also written in place by
// the GPU crowd), so the axes cost one thread per instance and no host copy.
// Inputs: the frame's CSG instance records (csg-device `CsgGpuInstance`, 64 B: row-major 3x4
// xform, compound_id @48, user_id @52, flags @60), compound records (`CsgGpuCompound`, 64 B:
// flags @12, local aabb_min @16, aabb_max @32), `gas_count` = compounds that own an OptiX GAS
// (an instance of any other compound has visibilityMask 0 in the IAS and is skipped), and optional
// trailing P42 `inst_obj_keys` (`u32[inst_count]`, nullable: bit 31 = `is_primary`, low 31 bits =
// logical `object_id`, `0` = ungrouped).
// One thread per instance:
//   skip    not VISIBLE, mask-0 instances, unbounded compounds (ground / water planes have no
//           meaningful frame), ungrouped/legacy crowd bones when `inst_obj_keys` is null or
//           `obj_id == 0`, the Selected scope's non-members, unselected non-primary parts, the
//           WithinRadius scope's far origins, and non-pinhole lenses (UNPROJECTABLE fault);
//   cull    origin within OVL_AXES_NEAR of the eye plane (for triads / unselected parts),
//           bounding sphere outside the frustum, or smaller than max(`min_px`, `axes_min_px`) on
//           screen (the selection is never size-culled);
//   emit    when `is_primary` is true (and `zo > OVL_AXES_NEAR`), X / Y / Z from the origin along
//           the normalised transform columns, length `axis_px * 2 tan(fov/2) z / height` (constant
//           on screen), SCREEN_CONSTANT_WIDTH, and the SELECTED flag for selection members — giving
//           every multi-part building, vehicle+wheels, or 19-bone crowd actor exactly ONE triad at
//           its primary root; plus the 12 oriented-box selection edges for EVERY `selected` part so
//           the entire logical object highlights together.
// Selection key space: when `inst_obj_keys != nullptr`, matches `pick::csg_selection_key`
// (`(0x80000000 | obj_id)` when `obj_id != 0`, else `user_id & 0x7FFFFFFF`); when null, falls back
// to `user_id` with `is_primary = true`.
#define OVL_CSG_INST_VISIBLE 1u       // CSG_INST_FLAG_VISIBLE
#define OVL_CSG_INST_CROWD_BONE 16u   // CSG_INST_FLAG_CROWD_BONE
#define OVL_CSG_COMPOUND_UNBOUNDED 1u // CSG_COMPOUND_FLAG_UNBOUNDED
#define OVL_AXES_NEAR 0.05f           // overlay::AXES_NEAR_M
#define OVL_AXES_DEPTH_BIAS 0.05f     // metres: an origin on a surface must not z-fight it

static __forceinline__ __device__ void ovl_push_world(
    NvOverlaySegment* segs, unsigned int* counter, unsigned int cap,
    float3 a, float3 b, unsigned int flags, unsigned int cls, unsigned int key,
    unsigned int style, float proj_px
) {
    NvOverlaySegment s;
    s.a[0] = a.x; s.a[1] = a.y; s.a[2] = a.z;
    s.flags = flags;
    s.b[0] = b.x; s.b[1] = b.y; s.b[2] = b.z;
    s.kind = NV_OVERLAY_KIND_SEGMENT;
    s.cls = cls;
    s.inst_id = key;
    s.part_id = 0xFFFFFFFFu;
    s.style = style;
    s.s0 = 0.0f;
    s.depth_bias = OVL_AXES_DEPTH_BIAS;
    s.proj_size_px = isinf(proj_px) ? 0.0f : proj_px;
    s.reserved = 0u;
    nv_overlay_push(segs, counter, cap, s);
}

extern "C" __global__ void ovl_axes(
    const float4* insts,
    unsigned int inst_count,
    const float4* compounds,
    unsigned int compound_count,
    unsigned int gas_count,
    const OvlCamera* cam,
    const unsigned int* sel,
    NvOverlaySegment* segs,
    unsigned int* counter,
    unsigned int cap,
    unsigned int height,
    unsigned int style_x,
    unsigned int style_y,
    unsigned int style_z,
    unsigned int style_box,
    float axis_px,
    const unsigned int* __restrict__ inst_obj_keys
) {
    const unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= inst_count) return;
    const OvlCamera c = *cam;
    if (c.pinhole == 0u) {
        if (i == 0u) atomicAdd(&counter[2], 1u);
        return;
    }
    const float4 r0 = __ldg(insts + 4u * i + 0u);
    const float4 r1 = __ldg(insts + 4u * i + 1u);
    const float4 r2 = __ldg(insts + 4u * i + 2u);
    const float4 meta = __ldg(insts + 4u * i + 3u);
    const unsigned int compound_id = __float_as_uint(meta.x);
    const unsigned int user_id = __float_as_uint(meta.y);
    const unsigned int flags = __float_as_uint(meta.w);
    if ((flags & OVL_CSG_INST_VISIBLE) == 0u) return;
    if (compound_id >= gas_count || compound_id >= compound_count) return;
    const float4 c0 = __ldg(compounds + 4u * compound_id + 0u);
    const float4 c1 = __ldg(compounds + 4u * compound_id + 1u);
    const float4 c2 = __ldg(compounds + 4u * compound_id + 2u);
    if ((__float_as_uint(c0.w) & OVL_CSG_COMPOUND_UNBOUNDED) != 0u) return;

    // Resolve P42 logical object key & primary-triad flag (nullable fallback when host did not
    // upload `inst_obj_keys`). Why: prevents 19 overlapping triads on crowd actors or 5 on
    // vehicles while letting every part of a selected object highlight its oriented bounds.
    bool is_primary = true;
    unsigned int key = user_id;
    if (inst_obj_keys != nullptr) {
        const unsigned int obj_word = __ldg(inst_obj_keys + i);
        const unsigned int obj_id = obj_word & 0x7FFFFFFFu;
        is_primary = (obj_word & 0x80000000u) != 0u;
        key = (obj_id != 0u) ? (0x80000000u | obj_id) : (user_id & 0x7FFFFFFFu);
        if ((flags & OVL_CSG_INST_CROWD_BONE) != 0u && obj_id == 0u) return;
    } else {
        if ((flags & OVL_CSG_INST_CROWD_BONE) != 0u) return;
    }
    const bool selected = ovl_is_selected(sel, c.sel_count, key);
    if (c.scope_selected != 0u && !selected) return;
    if (!is_primary && !selected) return;

    const float m[12] = {r0.x, r0.y, r0.z, r0.w, r1.x, r1.y, r1.z, r1.w, r2.x, r2.y, r2.z, r2.w};
    const float3 eye = make_float3(c.eye[0], c.eye[1], c.eye[2]);
    const float3 fwd = make_float3(c.fwd[0], c.fwd[1], c.fwd[2]);
    const float3 rgt = make_float3(c.right[0], c.right[1], c.right[2]);
    const float3 up = make_float3(c.up[0], c.up[1], c.up[2]);
    const float3 o = make_float3(m[3], m[7], m[11]);
    const float3 vo = make_float3(o.x - eye.x, o.y - eye.y, o.z - eye.z);
    const float zo = vo.x * fwd.x + vo.y * fwd.y + vo.z * fwd.z;
    if (!selected && zo <= OVL_AXES_NEAR) return;
    if (c.radius > 0.0f && sqrtf(vo.x * vo.x + vo.y * vo.y + vo.z * vo.z) > c.radius) return;

    // Bounding sphere of the transformed local AABB (centre M*c, half extents |M| h).
    const float lo[3] = {c1.x, c1.y, c1.z};
    const float hi[3] = {c2.x, c2.y, c2.z};
    const float cc[3] = {0.5f * (lo[0] + hi[0]), 0.5f * (lo[1] + hi[1]), 0.5f * (lo[2] + hi[2])};
    const float hh[3] = {0.5f * (hi[0] - lo[0]), 0.5f * (hi[1] - lo[1]), 0.5f * (hi[2] - lo[2])};
    float wc[3], ex[3];
    for (int r = 0; r < 3; ++r) {
        wc[r] = m[4 * r] * cc[0] + m[4 * r + 1] * cc[1] + m[4 * r + 2] * cc[2] + m[4 * r + 3];
        ex[r] = fabsf(m[4 * r]) * hh[0] + fabsf(m[4 * r + 1]) * hh[1] + fabsf(m[4 * r + 2]) * hh[2];
    }
    const float rad = sqrtf(ex[0] * ex[0] + ex[1] * ex[1] + ex[2] * ex[2]);
    const float3 p = make_float3(wc[0] - eye.x, wc[1] - eye.y, wc[2] - eye.z);
    const float px = p.x * rgt.x + p.y * rgt.y + p.z * rgt.z;
    const float py = p.x * up.x + p.y * up.y + p.z * up.z;
    const float pz = p.x * fwd.x + p.y * fwd.y + p.z * fwd.z;
    const float tx = c.tan_half_fov * c.aspect, ty = c.tan_half_fov;
    const float nx = sqrtf(1.0f + tx * tx), ny = sqrtf(1.0f + ty * ty);
    if (pz + rad <= 0.0f || (px - pz * tx) / nx > rad || (-px - pz * tx) / nx > rad
        || (py - pz * ty) / ny > rad || (-py - pz * ty) / ny > rad) {
        return;
    }
    const float dist = sqrtf(p.x * p.x + p.y * p.y + p.z * p.z);
    const float proj_px = (dist <= rad)
        ? INFINITY
        : 2.0f * rad * (float)height / (2.0f * fmaxf(c.tan_half_fov, 1e-4f) * fmaxf(pz, 1e-3f));
    // Object-level LOD (overlay::axes_lod): a primary root's triad needs projected size >=
    // axes_min_px so small distant props don't flood the viewport. The record size is rescaled so
    // the shared line-LOD fade of ovl_splat ([min_px, 3 min_px]) fades in over [axes_min, 3 axes_min].
    const float axes_min = fmaxf(c.min_px, c.axes_min_px);
    if (!selected && proj_px < axes_min) return;
    const float lod_px = (c.min_px > 0.0f && axes_min > c.min_px) ? proj_px * (c.min_px / axes_min) : proj_px;

    const unsigned int sflags = NV_OVERLAY_FLAG_WORLD | NV_OVERLAY_FLAG_SCREEN_CONSTANT_WIDTH
        | (selected ? NV_OVERLAY_FLAG_SELECTED : 0u);
    if (is_primary && zo > OVL_AXES_NEAR) {
        const float len = axis_px * 2.0f * c.tan_half_fov * zo / (float)max(height, 1u);
        const unsigned int styles[3] = {style_x, style_y, style_z};
        for (int k = 0; k < 3; ++k) {
            const float3 col = make_float3(m[k], m[4 + k], m[8 + k]);
            const float n = sqrtf(col.x * col.x + col.y * col.y + col.z * col.z);
            if (n <= 1e-8f) continue;
            const float s = len / n;
            ovl_push_world(segs, counter, cap, o, make_float3(o.x + col.x * s, o.y + col.y * s, o.z + col.z * s),
                           sflags, NV_OVERLAY_CLASS_AXIS, key, styles[k], lod_px);
        }
    }
    if (!selected) return;
    // Selection highlight: the 12 edges of every selected part's oriented local bounds.
    float3 corner[8];
    for (int k = 0; k < 8; ++k) {
        const float q[3] = {(k & 1) ? hi[0] : lo[0], (k & 2) ? hi[1] : lo[1], (k & 4) ? hi[2] : lo[2]};
        corner[k] = make_float3(
            m[0] * q[0] + m[1] * q[1] + m[2] * q[2] + m[3],
            m[4] * q[0] + m[5] * q[1] + m[6] * q[2] + m[7],
            m[8] * q[0] + m[9] * q[1] + m[10] * q[2] + m[11]);
    }
    for (int k = 0; k < 8; ++k) {
        for (int bit = 1; bit < 8; bit <<= 1) {
            if ((k & bit) == 0) {
                ovl_push_world(segs, counter, cap, corner[k], corner[k | bit], sflags,
                               NV_OVERLAY_CLASS_BOUNDS, key, style_box, proj_px);
            }
        }
    }
}

// Articulated crowd skeleton overlay (`overlay::GpuSource::Skeletons`): reads device-resident
// `world_joints` (`float4[N]`, `[x, y, z, radius]`, updated in place by `PlanKind::Crowd`) and the
// static `(parent_gidx, child_gidx, obj_key, 0)` edge table uploaded once per scene. Why: emits
// near-clipped world-space `NvOverlaySegment`s directly on the device for `ovl_splat` to project
// and depth-test, with zero per-frame host readbacks.
#define OVL_SKEL_NEAR 0.05f
#define OVL_SKEL_JOINT_DOT_DIST 25.0f

extern "C" __global__ void ovl_skeletons(
    const float4* __restrict__ world_joints,
    unsigned int joint_count,
    const uint4* __restrict__ edges,
    unsigned int edge_count,
    const OvlCamera* __restrict__ cam,
    const unsigned int* __restrict__ sel,
    NvOverlaySegment* __restrict__ segs,
    unsigned int* __restrict__ counter,
    unsigned int cap,
    unsigned int style_bone,
    unsigned int style_joint
) {
    if (world_joints == nullptr || edges == nullptr || cam == nullptr
        || segs == nullptr || counter == nullptr) {
        return;
    }
    const unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= edge_count) return;
    const OvlCamera c = *cam;
    if (c.pinhole == 0u) {
        if (i == 0u) atomicAdd(&counter[2], 1u);
        return;
    }

    const uint4 e = __ldg(edges + i);
    if (e.x >= joint_count || e.y >= joint_count) return;

    const unsigned int sel_n = (sel != nullptr) ? c.sel_count : 0u;
    const bool selected = ovl_is_selected(sel, sel_n, e.z);
    if (c.scope_selected != 0u && !selected) return;

    const float4 j0 = __ldg(world_joints + e.x);
    const float4 j1 = __ldg(world_joints + e.y);
    // Culled or unposed actors carry non-positive radius (`w <= 0`); guard non-finite coordinates.
    if (!isfinite(j0.x) || !isfinite(j0.y) || !isfinite(j0.z) || !isfinite(j0.w)
        || !isfinite(j1.x) || !isfinite(j1.y) || !isfinite(j1.z) || !isfinite(j1.w)
        || j0.w <= 0.0f || j1.w <= 0.0f) {
        return;
    }

    const float3 orig0 = make_float3(j0.x, j0.y, j0.z);
    const float3 orig1 = make_float3(j1.x, j1.y, j1.z);
    const float bx = orig1.x - orig0.x;
    const float by = orig1.y - orig0.y;
    const float bz = orig1.z - orig0.z;
    if (bx * bx + by * by + bz * bz < 1e-8f) return; // length < 1e-4 m

    const float3 eye = make_float3(c.eye[0], c.eye[1], c.eye[2]);
    const float3 fwd = make_float3(c.fwd[0], c.fwd[1], c.fwd[2]);
    const float z0_raw = (orig0.x - eye.x) * fwd.x + (orig0.y - eye.y) * fwd.y + (orig0.z - eye.z) * fwd.z;
    const float z1_raw = (orig1.x - eye.x) * fwd.x + (orig1.y - eye.y) * fwd.y + (orig1.z - eye.z) * fwd.z;
    if (z0_raw <= OVL_SKEL_NEAR && z1_raw <= OVL_SKEL_NEAR) return;

    // Clip segments that straddle the near plane so `ovl_splat`'s `ovl_project` never drops a bone
    // whose parent or child sits just behind the camera.
    float3 p0 = orig0;
    float3 p1 = orig1;
    if (z0_raw <= OVL_SKEL_NEAR) {
        const float t = fminf(1.0f, fmaxf(0.0f, (OVL_SKEL_NEAR - z0_raw) / (z1_raw - z0_raw)));
        p0 = make_float3(orig0.x + t * bx, orig0.y + t * by, orig0.z + t * bz);
    } else if (z1_raw <= OVL_SKEL_NEAR) {
        const float t = fminf(1.0f, fmaxf(0.0f, (OVL_SKEL_NEAR - z0_raw) / (z1_raw - z0_raw)));
        p1 = make_float3(orig0.x + t * bx, orig0.y + t * by, orig0.z + t * bz);
    }

    const float3 v0 = make_float3(p0.x - eye.x, p0.y - eye.y, p0.z - eye.z);
    const float3 v1 = make_float3(p1.x - eye.x, p1.y - eye.y, p1.z - eye.z);
    if (c.radius > 0.0f) {
        const float d0sq = v0.x * v0.x + v0.y * v0.y + v0.z * v0.z;
        const float d1sq = v1.x * v1.x + v1.y * v1.y + v1.z * v1.z;
        if (fminf(d0sq, d1sq) > c.radius * c.radius) return;
    }

    // Frustum side-plane cull on the clipped segment's bounding sphere so offscreen crowds do not
    // consume overlay segment capacity.
    const float3 rgt = make_float3(c.right[0], c.right[1], c.right[2]);
    const float3 up = make_float3(c.up[0], c.up[1], c.up[2]);
    const float3 mid = make_float3(0.5f * (v0.x + v1.x), 0.5f * (v0.y + v1.y), 0.5f * (v0.z + v1.z));
    const float cdx = p1.x - p0.x, cdy = p1.y - p0.y, cdz = p1.z - p0.z;
    const float rad = 0.5f * sqrtf(cdx * cdx + cdy * cdy + cdz * cdz);
    const float mx = mid.x * rgt.x + mid.y * rgt.y + mid.z * rgt.z;
    const float my = mid.x * up.x + mid.y * up.y + mid.z * up.z;
    const float mz = mid.x * fwd.x + mid.y * fwd.y + mid.z * fwd.z;
    const float tx = c.tan_half_fov * c.aspect;
    const float ty = c.tan_half_fov;
    const float nx = sqrtf(1.0f + tx * tx);
    const float ny = sqrtf(1.0f + ty * ty);
    if ((mx - mz * tx) / nx > rad || (-mx - mz * tx) / nx > rad
        || (my - mz * ty) / ny > rad || (-my - mz * ty) / ny > rad) {
        return;
    }

    // Depth-tested world segment (omitting `NV_OVERLAY_FLAG_NO_DEPTH_TEST` enables `st.depth_mode`).
    const unsigned int sflags = NV_OVERLAY_FLAG_WORLD | NV_OVERLAY_FLAG_SCREEN_CONSTANT_WIDTH
        | (selected ? NV_OVERLAY_FLAG_SELECTED : 0u);
    ovl_push_world(
        segs, counter, cap, p0, p1, sflags, NV_OVERLAY_CLASS_BONE, e.z, style_bone, 0.0f
    );

    // Optional joint accent dot at child joint `j1` when selected or close enough to inspect joints.
    if (z1_raw > OVL_SKEL_NEAR && (selected || z1_raw < OVL_SKEL_JOINT_DOT_DIST)) {
        ovl_push_world(
            segs, counter, cap, orig1, orig1, sflags, NV_OVERLAY_CLASS_BONE, e.z, style_joint, 0.0f
        );
    }
}

// Vehicle ADAS stopping-distance corridors and pedestrian hazard tethers (`overlay::GpuSource::Adas`):
// reads device-resident `TrafficInstanceOuts`, `TrafficHazards`, and optional `CrowdStates` directly
// in VRAM with zero host readbacks.
struct alignas(16) OvlTrafficOut {
    float xform[12];
    unsigned int instance_id;
    unsigned int flags;
    unsigned int vehicle;
    unsigned int slot;
};

struct alignas(16) OvlVehicleHazard {
    unsigned int state;
    unsigned int flags;
    unsigned int hazard_actor_idx;
    float speed_mps;
    float accel_mps2;
    float stop_dist_m;
    float hazard_dist_m;
    float limit_mps;
};

struct alignas(16) OvlCrowdActorStateView {
    float4 pos_yaw;
    unsigned char rest[96];
};

static_assert(sizeof(OvlTrafficOut) == 64, "OvlTrafficOut ABI mismatch");
static_assert(sizeof(OvlVehicleHazard) == 32, "OvlVehicleHazard ABI mismatch");
static_assert(sizeof(OvlCrowdActorStateView) == 112, "OvlCrowdActorStateView ABI mismatch");

__device__ __forceinline__ void ovl_emit_adas_seg(
    const OvlCamera& c,
    float3 orig0,
    float3 orig1,
    unsigned int sflags,
    unsigned int key,
    unsigned int style,
    NvOverlaySegment* segs,
    unsigned int* counter,
    unsigned int cap
) {
    const float3 eye = make_float3(c.eye[0], c.eye[1], c.eye[2]);
    const float3 fwd = make_float3(c.fwd[0], c.fwd[1], c.fwd[2]);
    const float z0 = (orig0.x - eye.x) * fwd.x + (orig0.y - eye.y) * fwd.y + (orig0.z - eye.z) * fwd.z;
    const float z1 = (orig1.x - eye.x) * fwd.x + (orig1.y - eye.y) * fwd.y + (orig1.z - eye.z) * fwd.z;
    if (z0 <= OVL_SKEL_NEAR && z1 <= OVL_SKEL_NEAR) return;

    float3 p0 = orig0;
    float3 p1 = orig1;
    const float dx = orig1.x - orig0.x;
    const float dy = orig1.y - orig0.y;
    const float dz = orig1.z - orig0.z;
    if (dx * dx + dy * dy + dz * dz < 1e-8f) return;
    if (z0 <= OVL_SKEL_NEAR) {
        const float t = fminf(1.0f, fmaxf(0.0f, (OVL_SKEL_NEAR - z0) / (z1 - z0)));
        p0 = make_float3(orig0.x + t * dx, orig0.y + t * dy, orig0.z + t * dz);
    } else if (z1 <= OVL_SKEL_NEAR) {
        const float t = fminf(1.0f, fmaxf(0.0f, (OVL_SKEL_NEAR - z0) / (z1 - z0)));
        p1 = make_float3(orig0.x + t * dx, orig0.y + t * dy, orig0.z + t * dz);
    }

    const float3 v0 = make_float3(p0.x - eye.x, p0.y - eye.y, p0.z - eye.z);
    const float3 v1 = make_float3(p1.x - eye.x, p1.y - eye.y, p1.z - eye.z);
    if (c.radius > 0.0f) {
        const float d0sq = v0.x * v0.x + v0.y * v0.y + v0.z * v0.z;
        const float d1sq = v1.x * v1.x + v1.y * v1.y + v1.z * v1.z;
        if (fminf(d0sq, d1sq) > c.radius * c.radius) return;
    }

    ovl_push_world(
        segs, counter, cap, p0, p1, sflags, NV_OVERLAY_CLASS_PATH, key, style, 0.0f
    );
}

extern "C" __global__ void ovl_adas(
    const OvlTrafficOut* __restrict__ outs,
    const OvlVehicleHazard* __restrict__ hazards,
    const OvlCrowdActorStateView* __restrict__ crowd_states,
    unsigned int vehicle_count,
    unsigned int slots_per_vehicle,
    unsigned int crowd_count,
    const OvlCamera* __restrict__ cam,
    const unsigned int* __restrict__ sel,
    NvOverlaySegment* __restrict__ segs,
    unsigned int* __restrict__ counter,
    unsigned int cap,
    unsigned int style_cruise,
    unsigned int style_yield,
    unsigned int style_aeb,
    unsigned int style_tether,
    const unsigned int* __restrict__ vehicle_sel_keys
) {
    if (outs == nullptr || hazards == nullptr || cam == nullptr
        || segs == nullptr || counter == nullptr) {
        return;
    }
    const unsigned int v = blockIdx.x * blockDim.x + threadIdx.x;
    if (v >= vehicle_count) return;
    const OvlCamera c = *cam;
    if (c.pinhole == 0u) {
        if (v == 0u) atomicAdd(&counter[2], 1u);
        return;
    }

    const OvlTrafficOut body = outs[(size_t)v * (size_t)slots_per_vehicle];
    if ((body.flags & 1u) == 0u) return;

    const OvlVehicleHazard hz = hazards[v];
    const float ox = body.xform[3];
    const float oy = body.xform[7];
    const float oz = body.xform[11];
    if (!isfinite(ox) || !isfinite(oy) || !isfinite(oz)) return;

    // Distance gate (`250 m`) keeps distant vehicles from saturating overlay segment capacity.
    const float ex = ox - c.eye[0];
    const float ey = oy - c.eye[1];
    const float ez = oz - c.eye[2];
    const float max_dist = (c.radius > 0.0f) ? fminf(c.radius, 250.0f) : 250.0f;
    if (ex * ex + ey * ey + ez * ez > max_dist * max_dist) return;

    const unsigned int key = (vehicle_sel_keys != nullptr) ? vehicle_sel_keys[v] : body.instance_id;
    const unsigned int sel_n = (sel != nullptr) ? c.sel_count : 0u;
    const bool selected = ovl_is_selected(sel, sel_n, key);
    if (c.scope_selected != 0u && !selected) return;

    const float3 r = make_float3(body.xform[0], body.xform[4], body.xform[8]);
    const float3 u = make_float3(body.xform[1], body.xform[5], body.xform[9]);
    const float3 f = make_float3(body.xform[2], body.xform[6], body.xform[10]);

    const float stop_len = isfinite(hz.stop_dist_m)
        ? fminf(40.0f, fmaxf(3.0f, hz.stop_dist_m))
        : 3.0f;
    const float bumper_fwd = 2.25f;
    const float half_w = 1.05f;
    const float lift_y = 0.22f;

    const float3 base = make_float3(
        ox + f.x * bumper_fwd + u.x * lift_y,
        oy + f.y * bumper_fwd + u.y * lift_y,
        oz + f.z * bumper_fwd + u.z * lift_y
    );
    const float3 tip = make_float3(
        base.x + f.x * stop_len,
        base.y + f.y * stop_len,
        base.z + f.z * stop_len
    );
    const float3 bl = make_float3(base.x - r.x * half_w, base.y - r.y * half_w, base.z - r.z * half_w);
    const float3 br = make_float3(base.x + r.x * half_w, base.y + r.y * half_w, base.z + r.z * half_w);
    const float3 tl = make_float3(tip.x - r.x * half_w, tip.y - r.y * half_w, tip.z - r.z * half_w);
    const float3 tr = make_float3(tip.x + r.x * half_w, tip.y + r.y * half_w, tip.z + r.z * half_w);

    unsigned int style = style_cruise;
    if (hz.state == 6u) {
        style = style_aeb;
    } else if (hz.state >= 2u && hz.state <= 5u) {
        style = style_yield;
    }

    const unsigned int sflags = NV_OVERLAY_FLAG_WORLD | NV_OVERLAY_FLAG_SCREEN_CONSTANT_WIDTH
        | (selected ? NV_OVERLAY_FLAG_SELECTED : 0u);
    ovl_emit_adas_seg(c, bl, tl, sflags, key, style, segs, counter, cap);
    ovl_emit_adas_seg(c, br, tr, sflags, key, style, segs, counter, cap);
    ovl_emit_adas_seg(c, tl, tr, sflags, key, style, segs, counter, cap);

    if ((hz.flags & 2u) != 0u && crowd_states != nullptr && hz.hazard_actor_idx < crowd_count) {
        const float4 ped = crowd_states[hz.hazard_actor_idx].pos_yaw;
        if (isfinite(ped.x) && isfinite(ped.y) && isfinite(ped.z)) {
            const float3 ped_pt = make_float3(ped.x, ped.y + 1.0f, ped.z);
            ovl_emit_adas_seg(
                c,
                base,
                ped_pt,
                sflags | NV_OVERLAY_FLAG_DASHED,
                key,
                style_tether,
                segs,
                counter,
                cap
            );
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Hidden-line wireframe (`DisplayMode::Wireframe`, wireframe.rs): vector contour lines at native
// resolution. Input is this frame's visible-hit G-buffer (raytracer.cu, `aov_mode` 11):
// `gbuf` = (octahedral normal, depth, instance key bits; WIRE_MISS_KEY = sky), `accum.w` = AO.
//
//   wire_classify  every pixel pair (right / down neighbour) gets a class (none / silhouette /
//                  crease) and a crossing vertex at the pair midpoint: the marching-squares
//                  vertices of the edge isolines on the pixel-centre lattice.
//   wire_smooth    Laplacian passes move each vertex towards the mean of its two contour
//                  neighbours (the partner crossings in the two lattice cells that share its
//                  edge); vertices whose contour turns sharply over two vertices on each side,
//                  junctions and line ends stay put, so corners survive while the pixel
//                  staircase becomes the smooth contour.
//   wire_emit      the contour provider: each cell's segments (marching-squares pairing of its
//                  crossings) are appended to the overlay segment list, which `ovl_splat` draws
//                  as antialiased strokes of constant screen width after the tonemap.
//   wire_faces     flat light-grey faces with slight AO, accumulated through `accum_count`.
//
// Hidden lines never appear: only the visible primary hit of each pixel exists in the G-buffer.
// Keep the pair rule, cell pairing, smoothing and coverage identical to the CPU mirror in
// `wireframe.rs` tests.
// ---------------------------------------------------------------------------------------------

#define WIRE_MISS_KEY 0xFFFFFFFFu
#define WIRE_NONE 0u
#define WIRE_SILHOUETTE 1u
#define WIRE_CREASE 2u
// Smoothing never moves a vertex whose two-ring contour turns by more than ~55 degrees.
#define WIRE_CORNER_COS 0.57f

struct WirePx {
    float3 n;
    float diam;
    float depth;
    unsigned int id;
    bool hit;
};

static __forceinline__ __device__ float3 wire_oct_decode(float ex, float ey) {
    float3 n = make_float3(ex, ey, 1.0f - fabsf(ex) - fabsf(ey));
    if (n.z < 0.0f) {
        float ox = (1.0f - fabsf(ey)) * (ex >= 0.0f ? 1.0f : -1.0f);
        float oy = (1.0f - fabsf(ex)) * (ey >= 0.0f ? 1.0f : -1.0f);
        n.x = ox;
        n.y = oy;
    }
    float inv = rsqrtf(fmaxf(1e-12f, n.x * n.x + n.y * n.y + n.z * n.z));
    return make_float3(n.x * inv, n.y * inv, n.z * inv);
}

// G-buffer texel: x = octahedral normal as two 16-bit snorm halves (`wire_oct_pack` in
// raytracer.cu), y = world diameter of the hit object (line LOD), z = depth, w = instance key.
static __forceinline__ __device__ WirePx wire_load(const float4* g, int x, int y, int w, int h) {
    x = max(0, min(w - 1, x));
    y = max(0, min(h - 1, y));
    float4 v = g[(unsigned int)y * (unsigned int)w + (unsigned int)x];
    WirePx p;
    const unsigned int on = __float_as_uint(v.x);
    p.n = wire_oct_decode((float)(short)(on & 0xFFFFu) / 32767.0f, (float)(short)(on >> 16u) / 32767.0f);
    p.diam = v.y;
    p.depth = fmaxf(v.z, 1e-4f);
    p.id = __float_as_uint(v.w);
    p.hit = (p.id != WIRE_MISS_KEY);
    return p;
}

// Relative curvature of 1/depth at `c` between `a` and `b`: zero on any plane under a pinhole
// camera, so slanted ground far away never reads as a depth jump.
static __forceinline__ __device__ float wire_inv_depth_curv(float a, float c, float b) {
    return fabsf(1.0f / a + 1.0f / b - 2.0f / c) * c;
}

// Class of the lattice edge between neighbours `p` and `q` (`a` before `p`, `b` after `q` on
// the same axis). Silhouette: sky on one side, or a depth jump between p and q (the curvature is
// high on both sides of the boundary, so the pairs next to a jump stay clean). Crease: another
// instance without a depth jump (contact / border line) or normals beyond the crease angle.
static __forceinline__ __device__ unsigned int wire_pair_class(
    WirePx a, WirePx p, WirePx q, WirePx b, float depth_thr, float crease_cos
) {
    if (!p.hit && !q.hit) return WIRE_NONE;
    if (p.hit != q.hit) return WIRE_SILHOUETTE;
    float ad = a.hit ? a.depth : p.depth;
    float bd = b.hit ? b.depth : q.depth;
    if (fminf(wire_inv_depth_curv(ad, p.depth, q.depth), wire_inv_depth_curv(p.depth, q.depth, bd))
        > depth_thr) {
        return WIRE_SILHOUETTE;
    }
    if (p.id != q.id) return WIRE_CREASE;
    float d = p.n.x * q.n.x + p.n.y * q.n.y + p.n.z * q.n.z;
    return (d < crease_cos) ? WIRE_CREASE : WIRE_NONE;
}

extern "C" __global__ void wire_classify(
    const float4* gbuf,
    unsigned int* cls,
    float4* pos,
    unsigned int width,
    unsigned int height,
    float depth_thr,
    float crease_cos
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= width || y >= height) return;
    const int ix = (int)x, iy = (int)y, w = (int)width, h = (int)height;
    WirePx p = wire_load(gbuf, ix, iy, w, h);
    unsigned int cr = WIRE_NONE, cd = WIRE_NONE;
    if (ix + 1 < w) {
        cr = wire_pair_class(wire_load(gbuf, ix - 1, iy, w, h), p, wire_load(gbuf, ix + 1, iy, w, h),
                             wire_load(gbuf, ix + 2, iy, w, h), depth_thr, crease_cos);
    }
    if (iy + 1 < h) {
        cd = wire_pair_class(wire_load(gbuf, ix, iy - 1, w, h), p, wire_load(gbuf, ix, iy + 1, w, h),
                             wire_load(gbuf, ix, iy + 2, w, h), depth_thr, crease_cos);
    }
    const unsigned int i = y * width + x;
    cls[i] = cr | (cd << 2u);
    // Pair midpoints in image pixels (pixel centres sit at +0.5).
    pos[i] = make_float4((float)ix + 1.0f, (float)iy + 0.5f, (float)ix + 0.5f, (float)iy + 1.0f);
}

// One lattice cell: its corners are the pixel centres (cx, cy) .. (cx + 1, cy + 1). Edge order
// T (right pair of (cx, cy)), B (right pair of (cx, cy + 1)), L (down pair of (cx, cy)),
// R (down pair of (cx + 1, cy)).
struct WireCell {
    float2 v[4];
    unsigned int c[4];
    unsigned int n;
};

// `pairing` of a cell with four crossings: true = (T, L) + (B, R), false = (T, R) + (B, L);
// the pairing with the shorter total length wins (ties pick the first).
static __forceinline__ __device__ bool wire_saddle_tl(const WireCell& k) {
    float a = hypotf(k.v[0].x - k.v[2].x, k.v[0].y - k.v[2].y)
            + hypotf(k.v[1].x - k.v[3].x, k.v[1].y - k.v[3].y);
    float b = hypotf(k.v[0].x - k.v[3].x, k.v[0].y - k.v[3].y)
            + hypotf(k.v[1].x - k.v[2].x, k.v[1].y - k.v[2].y);
    return a <= b;
}

// Contour partner of edge `e` inside cell `k`, or -1 (no crossing, junction or line end).
static __forceinline__ __device__ int wire_partner(const WireCell& k, int e) {
    if (k.c[e] == WIRE_NONE) return -1;
    if (k.n == 2u) {
        for (int o = 0; o < 4; ++o) {
            if (o != e && k.c[o] != WIRE_NONE) return o;
        }
        return -1;
    }
    if (k.n == 4u) {
        const int tl[4] = {2, 3, 0, 1};
        const int tr[4] = {3, 2, 1, 0};
        return wire_saddle_tl(k) ? tl[e] : tr[e];
    }
    return -1;
}

// Gathers cell (cx, cy) from a class / position source addressed by `fetch`.
template <typename Fetch>
static __forceinline__ __device__ WireCell wire_cell(int cx, int cy, Fetch fetch) {
    WireCell k;
    unsigned int c0, c1, c2, c3;
    float4 p0, p1, p2, p3;
    fetch(cx, cy, c0, p0);
    fetch(cx, cy + 1, c1, p1);
    fetch(cx + 1, cy, c3, p3);
    c2 = c0;
    p2 = p0;
    k.c[0] = c0 & 3u;
    k.v[0] = make_float2(p0.x, p0.y);
    k.c[1] = c1 & 3u;
    k.v[1] = make_float2(p1.x, p1.y);
    k.c[2] = (c2 >> 2u) & 3u;
    k.v[2] = make_float2(p2.z, p2.w);
    k.c[3] = (c3 >> 2u) & 3u;
    k.v[3] = make_float2(p3.z, p3.w);
    k.n = (k.c[0] != 0u) + (k.c[1] != 0u) + (k.c[2] != 0u) + (k.c[3] != 0u);
    return k;
}

// A contour vertex: the crossing on the right (`axis` 0) or down (`axis` 1) lattice edge of
// pixel (x, y). It belongs to two cells: side 0 = (x, y - 1) as B / (x - 1, y) as R, side 1 =
// (x, y) as T / L.
struct WireVtx {
    int x, y, axis;
};

static __forceinline__ __device__ void wire_vtx_cell(WireVtx v, int side, int& cx, int& cy, int& e) {
    if (v.axis == 0) {
        cx = v.x;
        cy = side == 0 ? v.y - 1 : v.y;
        e = side == 0 ? 1 : 0;
    } else {
        cx = side == 0 ? v.x - 1 : v.x;
        cy = v.y;
        e = side == 0 ? 3 : 2;
    }
}

static __forceinline__ __device__ WireVtx wire_edge_vtx(int cx, int cy, int e) {
    WireVtx v;
    v.x = (e == 3) ? cx + 1 : cx;
    v.y = (e == 1) ? cy + 1 : cy;
    v.axis = (e >= 2) ? 1 : 0;
    return v;
}

// The contour neighbour of `v` through its cell on `side`, if that cell pairs it.
template <typename Fetch>
static __forceinline__ __device__ bool wire_step(
    WireVtx v, int side, int w, int h, Fetch fetch, WireVtx& out, int& out_cx, int& out_cy
) {
    int cx, cy, e;
    wire_vtx_cell(v, side, cx, cy, e);
    if (cx < 0 || cy < 0 || cx + 1 >= w || cy + 1 >= h) return false;
    WireCell k = wire_cell(cx, cy, fetch);
    int p = wire_partner(k, e);
    if (p < 0) return false;
    out = wire_edge_vtx(cx, cy, p);
    out_cx = cx;
    out_cy = cy;
    return true;
}

// The neighbour of `a` on the far side from cell (from_cx, from_cy); `a` itself at a line end.
template <typename Fetch>
static __forceinline__ __device__ WireVtx wire_step_away(
    WireVtx a, int from_cx, int from_cy, int w, int h, Fetch fetch
) {
    int cx, cy, e;
    wire_vtx_cell(a, 0, cx, cy, e);
    int side = (cx == from_cx && cy == from_cy) ? 1 : 0;
    WireVtx out;
    int ocx, ocy;
    return wire_step(a, side, w, h, fetch, out, ocx, ocy) ? out : a;
}

template <typename Fetch>
static __forceinline__ __device__ float2 wire_vtx_pos(WireVtx v, Fetch fetch) {
    unsigned int c;
    float4 p;
    fetch(v.x, v.y, c, p);
    return v.axis == 0 ? make_float2(p.x, p.y) : make_float2(p.z, p.w);
}

// One Laplacian pass: every vertex with a contour neighbour on both sides moves by `lambda`
// towards their mean, unless the contour turns by more than ~55 degrees over two vertices on
// each side (a real corner; the one-ring alone cannot tell it from the pixel staircase, whose
// local turns are 45 degrees too).
extern "C" __global__ void wire_smooth(
    const unsigned int* cls,
    const float4* pos_in,
    float4* pos_out,
    unsigned int width,
    unsigned int height,
    float lambda
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= width || y >= height) return;
    const int w = (int)width, h = (int)height;
    auto fetch = [&](int px, int py, unsigned int& c, float4& p) {
        if (px < 0 || py < 0 || px >= w || py >= h) {
            c = 0u;
            p = make_float4(0.0f, 0.0f, 0.0f, 0.0f);
            return;
        }
        unsigned int i = (unsigned int)py * width + (unsigned int)px;
        c = cls[i];
        p = pos_in[i];
    };
    const unsigned int i = y * width + x;
    float4 out = pos_in[i];
    for (int axis = 0; axis < 2; ++axis) {
        if (((cls[i] >> (2u * (unsigned int)axis)) & 3u) == WIRE_NONE) continue;
        WireVtx v;
        v.x = (int)x;
        v.y = (int)y;
        v.axis = axis;
        WireVtx a, b;
        int acx, acy, bcx, bcy;
        if (!wire_step(v, 0, w, h, fetch, a, acx, acy) || !wire_step(v, 1, w, h, fetch, b, bcx, bcy)) {
            continue;
        }
        float2 pv = wire_vtx_pos(v, fetch);
        float2 pa = wire_vtx_pos(a, fetch);
        float2 pb = wire_vtx_pos(b, fetch);
        float2 pa2 = wire_vtx_pos(wire_step_away(a, acx, acy, w, h, fetch), fetch);
        float2 pb2 = wire_vtx_pos(wire_step_away(b, bcx, bcy, w, h, fetch), fetch);
        float2 d0 = make_float2(pv.x - pa2.x, pv.y - pa2.y);
        float2 d1 = make_float2(pb2.x - pv.x, pb2.y - pv.y);
        float l0 = hypotf(d0.x, d0.y), l1 = hypotf(d1.x, d1.y);
        if (l0 < 1e-6f || l1 < 1e-6f) continue;
        if ((d0.x * d1.x + d0.y * d1.y) / (l0 * l1) < WIRE_CORNER_COS) continue;
        float nx = pv.x + lambda * (0.5f * (pa.x + pb.x) - pv.x);
        float ny = pv.y + lambda * (0.5f * (pa.y + pb.y) - pv.y);
        if (axis == 0) {
            out.x = nx;
            out.y = ny;
        } else {
            out.z = nx;
            out.w = ny;
        }
    }
    pos_out[i] = out;
}

// Contour provider (`overlay::ContourProvider`): one thread per lattice cell appends the cell's
// smoothed segments (marching-squares pairing of its crossings; junctions and line ends join the
// crossing centroid / cell centre) to the overlay list as screen-space `NvOverlaySegment`s.
// Silhouette style + CONTOUR class when both ends are silhouette crossings, crease style +
// CREASE class otherwise.
// Line LOD (why: a 1-spp G-buffer undersamples far features, whose pixel pattern changes every
// frame; no edge filter can make that stable): each record carries the projected diameter of
// its foreground object (`diam * height / (2 tan(fov/2) depth)`) as `proj_size_px`, and
// `ovl_splat` fades it between `min_px` and `3 * min_px`.
extern "C" __global__ void wire_emit(
    const float4* gbuf,
    const unsigned int* cls,
    const float4* pos,
    NvOverlaySegment* segs,
    unsigned int* counter,
    unsigned int cap,
    unsigned int width,
    unsigned int height,
    unsigned int style_sil,
    unsigned int style_crease,
    float tan_half_fov
) {
    const unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    const unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    const int w = (int)width, h = (int)height;
    if ((int)x + 1 >= w || (int)y + 1 >= h) return;
    auto fetch = [&](int px, int py, unsigned int& c, float4& p) {
        unsigned int i = (unsigned int)py * width + (unsigned int)px;
        c = cls[i];
        p = pos[i];
    };
    const WireCell k = wire_cell((int)x, (int)y, fetch);
    if (k.n == 0u) return;
    // The nearest hit corner is the foreground object of the feature.
    float z = 1e30f, diam = 1e6f;
    unsigned int inst = 0xFFFFFFFFu;
    for (int dy = 0; dy < 2; ++dy) {
        for (int dx = 0; dx < 2; ++dx) {
            WirePx p = wire_load(gbuf, (int)x + dx, (int)y + dy, w, h);
            if (p.hit && p.depth < z) {
                z = p.depth;
                diam = p.diam;
                inst = p.id;
            }
        }
    }
    if (z > 1e29f) z = 1e6f;
    const float proj_px = diam * (float)height / (2.0f * fmaxf(tan_half_fov, 1e-4f) * z);
    auto emit = [&](float2 a, float2 b, unsigned int ca, unsigned int cb) {
        const bool sil = ca == WIRE_SILHOUETTE && cb == WIRE_SILHOUETTE;
        NvOverlaySegment s;
        s.a[0] = a.x;
        s.a[1] = a.y;
        s.a[2] = z;
        s.flags = NV_OVERLAY_FLAG_SCREEN_CONSTANT_WIDTH | NV_OVERLAY_FLAG_NO_DEPTH_TEST;
        s.b[0] = b.x;
        s.b[1] = b.y;
        s.b[2] = z;
        s.kind = NV_OVERLAY_KIND_SEGMENT;
        s.cls = sil ? NV_OVERLAY_CLASS_CONTOUR : NV_OVERLAY_CLASS_CREASE;
        s.inst_id = inst;
        s.part_id = 0xFFFFFFFFu;
        s.style = sil ? style_sil : style_crease;
        s.s0 = 0.0f;
        s.depth_bias = 0.0f;
        s.proj_size_px = proj_px;
        s.reserved = 0u;
        nv_overlay_push(segs, counter, cap, s);
    };
    if (k.n == 2u || k.n == 4u) {
        for (int e = 0; e < 4; ++e) {
            int o = wire_partner(k, e);
            if (o > e) emit(k.v[e], k.v[o], k.c[e], k.c[o]);
        }
        return;
    }
    float2 m = make_float2((float)x + 1.0f, (float)y + 1.0f);
    if (k.n == 3u) {
        m = make_float2(0.0f, 0.0f);
        for (int e = 0; e < 4; ++e) {
            if (k.c[e] != WIRE_NONE) {
                m.x += k.v[e].x / 3.0f;
                m.y += k.v[e].y / 3.0f;
            }
        }
    }
    for (int e = 0; e < 4; ++e) {
        if (k.c[e] != WIRE_NONE) emit(k.v[e], m, k.c[e], k.c[e]);
    }
}

// Wireframe faces: flat light-grey shading from a fixed view-independent key light with slight
// AO (this frame's AO in `accum.w`), sky as light paper, accumulated through `accum_count`. The
// lines are overlay segments drawn after the tonemap.
extern "C" __global__ void wire_faces(
    const float4* gbuf,
    float4* accum,
    unsigned int width,
    unsigned int height,
    unsigned int accum_count
) {
    const unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    const unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= width || y >= height) return;
    const unsigned int idx = y * width + x;
    const float4 prev = accum[idx];
    const WirePx c = wire_load(gbuf, (int)x, (int)y, (int)width, (int)height);
    float v;
    if (!c.hit) {
        v = 0.92f;
    } else {
        const float3 key = make_float3(0.37f, 0.84f, 0.40f);
        const float ndl = fabsf(c.n.x * key.x + c.n.y * key.y + c.n.z * key.z);
        v = (0.50f + 0.30f * ndl) * (1.0f - 0.30f * prev.w);
    }
    float3 o;
    if (accum_count == 0u) {
        o = make_float3(v, v, v);
    } else {
        const float kf = 1.0f / (float)(accum_count + 1u);
        o = make_float3(prev.x + (v - prev.x) * kf, prev.y + (v - prev.y) * kf, prev.z + (v - prev.z) * kf);
    }
    accum[idx] = make_float4(o.x, o.y, o.z, 1.0f);
}
