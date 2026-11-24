// nv-demo overlay record contract (`overlay.rs` `OverlaySegmentPod`, `OVERLAY_SEGMENT_CUH`).
//
// Why a standalone header: GPU providers outside nv-demo (csg-rs feature curves through a
// LinesFramePlan) write records straight into nv-demo's device overlay buffer. This header is
// the whole contract: record layout, kind / class / flag values, and the append helper with its
// capacity and fault rule. The Rust side asserts the same size and offsets. It is a local,
// interim contract: when csg-scene publishes its `OverlayCurvePod` (analytic curve kinds with
// visible / hidden parameter intervals) nv-demo switches to it and deletes this one; the draw
// pass already dispatches on `kind` and reads widths / colours from a style table.
//
// Buffer: `NvOverlaySegment segs[capacity]` plus `unsigned int counter[NV_OVERLAY_COUNTER_WORDS]`:
// counter[0] = records appended (may exceed capacity), counter[1] = dropped at capacity
// (typed fault OVERFLOW), counter[2] = world records the active lens cannot project
// (UNPROJECTABLE), counter[3] = records of a kind this build cannot draw (UNSUPPORTED_KIND).
// nv-demo reports every non-zero fault in its status line.
#ifndef NV_DEMO_OVERLAY_SEGMENT_CUH
#define NV_DEMO_OVERLAY_SEGMENT_CUH

// Record kinds (`overlay::RecordKind`): how `a` / `b` are interpreted by the draw pass.
#define NV_OVERLAY_KIND_SEGMENT 0u         // independent straight segment a-b
#define NV_OVERLAY_KIND_POLYLINE_CHUNK 1u  // one segment of a polyline; s0 continues the dashes

// Segment classes (`overlay::SegmentClass`): what the line means, for styling and filtering.
#define NV_OVERLAY_CLASS_CREASE 0u
#define NV_OVERLAY_CLASS_BOUNDARY 1u
#define NV_OVERLAY_CLASS_SILHOUETTE_GENERATOR 2u
#define NV_OVERLAY_CLASS_BOOLEAN_INTERSECTION 3u
#define NV_OVERLAY_CLASS_CONTOUR 4u
#define NV_OVERLAY_CLASS_AXIS 5u
#define NV_OVERLAY_CLASS_BONE 6u
#define NV_OVERLAY_CLASS_PATH 7u
#define NV_OVERLAY_CLASS_BOUNDS 8u  // oriented bounding-box edge (selection highlight)

// Flag bits (`overlay::seg_flags`).
#define NV_OVERLAY_FLAG_WORLD 1u                  // a / b in world metres (projected by nv-demo), else screen
#define NV_OVERLAY_FLAG_HIDDEN 2u                 // producer occlusion-tested it hidden: faded + dashed
#define NV_OVERLAY_FLAG_DASHED 4u                 // dashed regardless of the style
#define NV_OVERLAY_FLAG_SELECTED 8u               // part of the selection: drawn wider and opaque
#define NV_OVERLAY_FLAG_NO_DEPTH_TEST 16u         // never tested against the primary-hit depth
#define NV_OVERLAY_FLAG_SCREEN_CONSTANT_WIDTH 32u // style width in px; otherwise metres, projected

#define NV_OVERLAY_COUNTER_WORDS 4u

// 64-byte record.
//   a, b          world metres (FLAG_WORLD) or screen (x px, y px, planar view depth; depth <= 0
//                 = no depth test)
//   flags         NV_OVERLAY_FLAG_*
//   kind          NV_OVERLAY_KIND_*
//   cls           NV_OVERLAY_CLASS_*
//   inst_id       instance key (primary-hit instance id space), 0xFFFFFFFF = none
//   part_id       part within the instance, 0xFFFFFFFF = none
//   style         index into nv-demo's per-frame style table (colour, width, dash, depth mode)
//   s0            arc length at `a` in screen px (dash phase)
//   depth_bias    metres subtracted from the record depth before the depth test
//   proj_size_px  projected size of the feature it belongs to, for the LOD fade (0 = unknown,
//                 never faded)
struct NvOverlaySegment {
    float a[3];
    unsigned int flags;
    float b[3];
    unsigned int kind;
    unsigned int cls;
    unsigned int inst_id;
    unsigned int part_id;
    unsigned int style;
    float s0;
    float depth_bias;
    float proj_size_px;
    unsigned int reserved;
};
static_assert(sizeof(NvOverlaySegment) == 64, "NvOverlaySegment must stay 64 bytes");
static_assert(__builtin_offsetof(NvOverlaySegment, flags) == 12, "flags offset");
static_assert(__builtin_offsetof(NvOverlaySegment, b) == 16, "b offset");
static_assert(__builtin_offsetof(NvOverlaySegment, kind) == 28, "kind offset");
static_assert(__builtin_offsetof(NvOverlaySegment, cls) == 32, "cls offset");
static_assert(__builtin_offsetof(NvOverlaySegment, inst_id) == 36, "inst_id offset");
static_assert(__builtin_offsetof(NvOverlaySegment, part_id) == 40, "part_id offset");
static_assert(__builtin_offsetof(NvOverlaySegment, style) == 44, "style offset");
static_assert(__builtin_offsetof(NvOverlaySegment, s0) == 48, "s0 offset");
static_assert(__builtin_offsetof(NvOverlaySegment, depth_bias) == 52, "depth_bias offset");
static_assert(__builtin_offsetof(NvOverlaySegment, proj_size_px) == 56, "proj_size_px offset");

// Appends `s`; past `capacity` the record is dropped and counted as an OVERFLOW fault.
static __forceinline__ __device__ void nv_overlay_push(
    NvOverlaySegment* segs, unsigned int* counter, unsigned int capacity, const NvOverlaySegment& s
) {
    unsigned int i = atomicAdd(&counter[0], 1u);
    if (i < capacity) {
        segs[i] = s;
    } else {
        atomicAdd(&counter[1], 1u);
    }
}

#endif // NV_DEMO_OVERLAY_SEGMENT_CUH
