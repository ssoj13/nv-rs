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
//   wire_draw      each cell's segments (marching-squares pairing of its crossings) are drawn as
//                  antialiased strokes of constant screen width (analytic distance to the segment,
//                  smoothstep falloff), composited in linear light over flat light-grey faces with
//                  slight AO, and accumulated through `accum_count` (only the AO varies).
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
#define WIRE_TILE 16
#define WIRE_HALO 4
#define WIRE_SPAN (WIRE_TILE + 2 * WIRE_HALO + 1)

struct WirePx {
    float3 n;
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

static __forceinline__ __device__ WirePx wire_load(const float4* g, int x, int y, int w, int h) {
    x = max(0, min(w - 1, x));
    y = max(0, min(h - 1, y));
    float4 v = g[(unsigned int)y * (unsigned int)w + (unsigned int)x];
    WirePx p;
    p.n = wire_oct_decode(v.x, v.y);
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

// Distance from `p` to the segment `a`-`b`.
static __forceinline__ __device__ float wire_seg_dist(float2 p, float2 a, float2 b) {
    float2 ab = make_float2(b.x - a.x, b.y - a.y);
    float2 ap = make_float2(p.x - a.x, p.y - a.y);
    float l2 = ab.x * ab.x + ab.y * ab.y;
    float t = (l2 > 1e-12f) ? fminf(1.0f, fmaxf(0.0f, (ap.x * ab.x + ap.y * ab.y) / l2)) : 0.0f;
    float dx = ap.x - t * ab.x, dy = ap.y - t * ab.y;
    return sqrtf(dx * dx + dy * dy);
}

// Coverage of a stroke `width` px wide whose centre line is `dist` px from the pixel centre:
// smoothstep over one pixel of falloff; strokes thinner than a pixel fade in opacity instead.
static __forceinline__ __device__ float wire_coverage(float dist, float width) {
    float t = fminf(1.0f, fmaxf(0.0f, 0.5f * width + 0.5f - dist));
    return t * t * (3.0f - 2.0f * t) * fminf(1.0f, width);
}

static __forceinline__ __device__ void wire_stroke(
    float2 p, float2 a, float2 b, unsigned int ca, unsigned int cb, float sil_px, float crease_px,
    float& a_sil, float& a_cr
) {
    float d = wire_seg_dist(p, a, b);
    if (ca == WIRE_SILHOUETTE && cb == WIRE_SILHOUETTE) {
        a_sil = fmaxf(a_sil, wire_coverage(d, sil_px));
    } else {
        a_cr = fmaxf(a_cr, wire_coverage(d, crease_px));
    }
}

extern "C" __global__ void wire_draw(
    const float4* gbuf,
    const unsigned int* cls,
    const float4* pos,
    float4* accum,
    unsigned int width,
    unsigned int height,
    unsigned int accum_count,
    float sil_px,
    float crease_px
) {
    // Tile of crossings covering every cell whose strokes can reach this block's pixels.
    __shared__ unsigned int s_cls[WIRE_SPAN][WIRE_SPAN];
    __shared__ float4 s_pos[WIRE_SPAN][WIRE_SPAN];
    const int w = (int)width, h = (int)height;
    const int ox = (int)(blockIdx.x * WIRE_TILE) - WIRE_HALO;
    const int oy = (int)(blockIdx.y * WIRE_TILE) - WIRE_HALO;
    for (int t = (int)(threadIdx.y * WIRE_TILE + threadIdx.x); t < WIRE_SPAN * WIRE_SPAN;
         t += WIRE_TILE * WIRE_TILE) {
        int sx = t % WIRE_SPAN, sy = t / WIRE_SPAN;
        int gx = ox + sx, gy = oy + sy;
        bool in = gx >= 0 && gy >= 0 && gx < w && gy < h;
        unsigned int gi = in ? (unsigned int)gy * width + (unsigned int)gx : 0u;
        s_cls[sy][sx] = in ? cls[gi] : 0u;
        s_pos[sy][sx] = in ? pos[gi] : make_float4(0.0f, 0.0f, 0.0f, 0.0f);
    }
    __syncthreads();

    const unsigned int x = blockIdx.x * WIRE_TILE + threadIdx.x;
    const unsigned int y = blockIdx.y * WIRE_TILE + threadIdx.y;
    if (x >= width || y >= height) return;
    auto fetch = [&](int px, int py, unsigned int& c, float4& p) {
        int sx = px - ox, sy = py - oy;
        if (sx < 0 || sy < 0 || sx >= WIRE_SPAN || sy >= WIRE_SPAN || px >= w || py >= h) {
            c = 0u;
            p = make_float4(0.0f, 0.0f, 0.0f, 0.0f);
            return;
        }
        c = s_cls[sy][sx];
        p = s_pos[sy][sx];
    };

    const float2 pc = make_float2((float)x + 0.5f, (float)y + 0.5f);
    float a_sil = 0.0f, a_cr = 0.0f;
    for (int cy = (int)y - WIRE_HALO; cy < (int)y + WIRE_HALO; ++cy) {
        for (int cx = (int)x - WIRE_HALO; cx < (int)x + WIRE_HALO; ++cx) {
            if (cx < 0 || cy < 0 || cx + 1 >= w || cy + 1 >= h) continue;
            WireCell k = wire_cell(cx, cy, fetch);
            if (k.n == 0u) continue;
            if (k.n == 2u || k.n == 4u) {
                for (int e = 0; e < 4; ++e) {
                    int o = wire_partner(k, e);
                    if (o > e) wire_stroke(pc, k.v[e], k.v[o], k.c[e], k.c[o], sil_px, crease_px, a_sil, a_cr);
                }
            } else {
                // Junction (3 crossings) or line end (1): every crossing joins the cell centroid
                // of its crossings (the cell centre for a line end).
                float2 m = make_float2((float)cx + 1.0f, (float)cy + 1.0f);
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
                    if (k.c[e] != WIRE_NONE) wire_stroke(pc, k.v[e], m, k.c[e], k.c[e], sil_px, crease_px, a_sil, a_cr);
                }
            }
        }
    }

    // Linear light: light paper sky, flat light-grey faces with slight AO, crease lines mid grey,
    // silhouettes near black. The tonemap applies the sRGB transfer.
    const unsigned int idx = y * width + x;
    const float4 prev = accum[idx];
    WirePx c = wire_load(gbuf, (int)x, (int)y, w, h);
    float v;
    if (!c.hit) {
        v = 0.92f;
    } else {
        const float3 key = make_float3(0.37f, 0.84f, 0.40f);
        float ndl = fabsf(c.n.x * key.x + c.n.y * key.y + c.n.z * key.z);
        v = (0.50f + 0.30f * ndl) * (1.0f - 0.30f * prev.w);
    }
    v = v + (0.06f - v) * (0.85f * a_cr);
    v = v + (0.010f - v) * a_sil;
    float3 o;
    if (accum_count == 0u) {
        o = make_float3(v, v, v);
    } else {
        float kf = 1.0f / (float)(accum_count + 1u);
        o = make_float3(prev.x + (v - prev.x) * kf, prev.y + (v - prev.y) * kf, prev.z + (v - prev.z) * kf);
    }
    accum[idx] = make_float4(o.x, o.y, o.z, 1.0f);
}
