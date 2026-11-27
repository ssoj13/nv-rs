//! CUDA/NVRTC GPU-direct articulated crowd simulation backend driven by `csg-rs` [`FramePlan`].
//!
//! What: executes `csg_scene::crowd_gpu::crowd_frame_plan(csg_device::CSG_EVAL_CUH)` (plan v2:
//! spatial grid, overlap resolution, walkway-network steering, two-pass foot probes & rendered-
//! support ground sampling, LIPM/PD state integration, and articulated FK/IK emission) through
//! [`crate::frame_plan_runner::GpuFramePlan`], paired with a tiny host-only CUDA bridge module
//! (`CROWD_BRIDGE_CUH`) that scatters bone outputs into `nv-demo`'s CSG/OptiX IAS buffers with
//! semantic visibility masking and monitors cumulative root displacement for amortized IAS
//! rebuilds.
//!
//! Why: delegating all steering, ground sampling, and articulated pose synthesis to `csg-rs`'s
//! declarative `FramePlan` eliminates host-side launch drift and duplicated GPU steering/DEM code
//! while keeping steady-state crowd stepping allocation-free and free of mid-frame host syncs.
//! Explicit `CrowdBackend::Cuda` stays strict and never silently downgrades on failure.
//!
//! How: `ensure_scene_uploaded` builds [`CrowdSteerScene::from_scene`], bumps
//! [`CrowdTableGenerations`], uploads `BufClass::Table` buffers only on generation change, seeds
//! `BufClass::DeviceState`, and queues [`EventId::CrowdNavRelink`] when needed. Each frame uploads
//! uniforms + traffic vehicles, clears `CrowdGroundMisses`, executes the plan with null OptiX
//! externals (so `CrowdBoneOuts` remains compactly bone-indexed for downstream zero-copy
//! `PlanKind::Camera` binding via [`CudaCrowdBackend::bone_outs_device_ptr`], and `CrowdWorldJoints`
//! plus a static parent-to-child bone-edge table remain device-resident for zero-copy skeleton
//! overlays via [`CudaCrowdBackend::world_joints_device_ptr`] and
//! [`CudaCrowdBackend::skeleton_edges_device_ptr`]), runs the two tiny
//! bridge kernels, and copies 12 bytes of status/displacement words into page-locked host memory.
//!
//! Uses: `crate::frame_plan_runner`, `csg_scene::crowd_gpu`, `csg_scene::frame_plan`,
//! `csg_device::CSG_EVAL_CUH`, `csg_actors`, `cuda_bindings`, `cuda_core`.
//! Used by: `crate::renderer`, `crate::csg_scene_host`.

use std::ffi::c_void;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Instant;

use bytemuck::Pod;
use csg_actors::{
    ActorStatePod, ArchetypeMetaPod, BONE_OUT_VISIBLE, BoneOutPod, OptixInstancePod, intent_flags,
};
use csg_scene::crowd_gpu::{
    CROWD_FAULT_NONE, CROWD_FRAME_PLAN_VERSION, CrowdFrameData, CrowdGpuPoses, CrowdGpuStatus,
    CrowdNavPod, CrowdSteerScene, CrowdTableGenerations, CrowdVehiclePod, crowd_frame_plan,
    crowd_nav_pods, crowd_sim_uniforms, crowd_vehicle_pods,
};
use csg_scene::frame_plan::{
    BufId, EventId, ExternalId, FrameSizes, ReadbackWhen,
};
use csg_scene::{SceneBuilt, inst_flags};
use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, DeviceBuffer, Stream};

use crate::csg_scene_host::CSG_SBT_PRIM_OFFSET;
use crate::frame_plan_runner::{GpuFramePlan, PinnedHostBuf, compile_nvrtc_ptx, load_cuda_fn};
use crate::semantic_filter::LINES_OCCLUDER_MASK;

/// Extra `CsgGpuInstance::flags` bits OR'd onto visible scattered crowd bone instances so the
/// GPU-written slot carries the same flags `WorldScene::pack_visible` gives a crowd bone: shadows,
/// `Motion` AOV optical flow, and the `CROWD_BONE` marker that slot mapping and repacks inspect.
const SCATTER_VISIBLE_EXTRA_FLAGS: u32 = inst_flags::DYNAMIC
    | inst_flags::CAST_SHADOW
    | inst_flags::MOTION_BLUR
    | inst_flags::CROWD_BONE;

/// OptiX instance flag disabling any-hit programs on opaque crowd bone instances.
const OPTIX_INST_DISABLE_ANYHIT: u32 = optix::INSTANCE_FLAG_DISABLE_ANYHIT;

/// Pinned status buffer indices (`[CrowdFault, CrowdGroundMisses, disp_flag]` = 12 B total).
const STATUS_FAULT_IDX: usize = 0;
const STATUS_GROUND_MISSES_IDX: usize = 1;
const STATUS_DISP_FLAG_IDX: usize = 2;
const STATUS_WORD_COUNT: usize = 3;

// Compile-time C-ABI locks between Rust PODs / constants and `CROWD_BRIDGE_CUH`.
const _: () = assert!(std::mem::size_of::<ActorStatePod>() == 112);
const _: () = assert!(std::mem::offset_of!(ActorStatePod, pos_yaw) == 0);
const _: () = assert!(std::mem::size_of::<BoneOutPod>() == 64);
const _: () = assert!(std::mem::offset_of!(BoneOutPod, xform) == 0);
const _: () = assert!(std::mem::offset_of!(BoneOutPod, compound_id) == 48);
const _: () = assert!(std::mem::offset_of!(BoneOutPod, flags) == 60);
const _: () = assert!(std::mem::size_of::<OptixInstancePod>() == 80);
const _: () = assert!(BONE_OUT_VISIBLE == 1);
const _: () = assert!(OPTIX_INST_DISABLE_ANYHIT == 4);
const _: () = assert!(CSG_SBT_PRIM_OFFSET == 10);
const _: () = assert!(LINES_OCCLUDER_MASK == 0x80);
const _: () = assert!(inst_flags::SEMANTIC_SHIFT == 16);
const _: () = assert!((SCATTER_VISIBLE_EXTRA_FLAGS & inst_flags::SEMANTIC_MASK) == 0);

/// Maximum actor root displacement (in meters) before forcing a full OptiX IAS rebuild instead of
/// an incremental `OPTIX_BUILD_OPERATION_UPDATE` refit (preserves tight BVH traversal quality).
pub const CUDA_IAS_REBUILD_DISP_M: f32 = 8.0;

/// Single source of truth scaling crowd LOD thresholds `(lod1_dist_m, lod2_dist_m, cull_dist_m)`
/// by `lod_bias` (`0.25..=4.0`), guarding against non-finite or non-positive inputs identically
/// across CPU (`CrowdSim`) and CUDA (`crowd_frame_plan`).
#[inline]
pub fn scale_crowd_lod_dists(
    lod1_dist_m: f32,
    lod2_dist_m: f32,
    cull_dist_m: f32,
    lod_bias: f32,
) -> (f32, f32, f32) {
    let bias = if lod_bias.is_finite() {
        lod_bias.clamp(0.25, 4.0)
    } else {
        1.0
    };
    let base_lod1 = if lod1_dist_m.is_finite() && lod1_dist_m > 0.0 {
        lod1_dist_m
    } else {
        22.0
    };
    let base_lod2 = if lod2_dist_m.is_finite() && lod2_dist_m > 0.0 {
        lod2_dist_m
    } else {
        75.0
    };
    let base_cull = if cull_dist_m.is_finite() && cull_dist_m > 0.0 {
        cull_dist_m
    } else {
        180.0
    };
    let l1 = base_lod1 * bias;
    let l2 = (base_lod2 * bias).max(l1);
    let cull = (base_cull * bias).max(l2);
    (l1, l2, cull)
}

/// Unified crowd LOD configuration shared across `DemoSettings`, `CsgGpuHost` (CPU crowd),
/// and `CudaCrowdBackend` (GPU `crowd_frame_plan`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CrowdLodParams {
    /// Master LOD/culling switch (`false` keeps all actors at full LOD0 with no culling).
    pub enabled: bool,
    /// Optional forced LOD tier (`None` = Auto, `Some(0)` = LOD0, `Some(1)` = LOD1, `Some(2)` = LOD2).
    pub force_lod: Option<u8>,
    /// Global LOD detail multiplier (`0.25..=4.0`).
    pub lod_bias: f32,
    /// Base LOD0 -> LOD1 transition distance in meters (`5..=120`, default `22.0`).
    pub lod1_dist_m: f32,
    /// Base LOD1 -> LOD2 transition distance in meters (`20..=300`, default `75.0`).
    pub lod2_dist_m: f32,
    /// Base crowd visibility cull distance in meters (`40..=600`, default `180.0`).
    pub cull_dist_m: f32,
}

impl Default for CrowdLodParams {
    fn default() -> Self {
        Self {
            enabled: true,
            force_lod: None,
            lod_bias: 1.0,
            lod1_dist_m: 22.0,
            lod2_dist_m: 75.0,
            cull_dist_m: 180.0,
        }
    }
}

impl CrowdLodParams {
    /// Maps UI `csg_force_lod` (`0 = Auto, 1 = LOD0, 2 = LOD1, 3 = LOD2`) to `Option<u8>`.
    #[inline]
    pub const fn decode_force_lod(v: u32) -> Option<u8> {
        match v {
            1 => Some(0),
            2 => Some(1),
            3.. => Some(2),
            _ => None,
        }
    }

    /// Resolves effective `(lod1_dist_m, lod2_dist_m, cull_dist_m)` honored identically by CPU
    /// `CrowdSim` and GPU `crowd_frame_plan`.
    ///
    /// Uses strictly positive thresholds (`> 0.0`) so CUDA's `> 0.0f` default guard never
    /// overrides forced LOD tiers.
    #[inline]
    pub fn resolve_dists(self) -> (f32, f32, f32) {
        const FAR_M: f32 = 1.0e9;
        const TINY_M: f32 = 1.0e-5;
        if !self.enabled {
            return (FAR_M, FAR_M, FAR_M);
        }
        match self.force_lod {
            Some(0) => (FAR_M, FAR_M, FAR_M),
            Some(1) => (TINY_M, FAR_M, FAR_M),
            Some(2..) => (TINY_M, TINY_M, FAR_M),
            None => scale_crowd_lod_dists(
                self.lod1_dist_m,
                self.lod2_dist_m,
                self.cull_dist_m,
                self.lod_bias,
            ),
        }
    }

    /// Returns a NaN-normalized comparable key so any change to any crowd LOD setting triggers
    /// an immediate crowd LOD refresh without floating-point NaN thrashing.
    #[inline]
    pub fn signature_key(self) -> (bool, Option<u8>, u32, u32, u32, u32) {
        let (l1, l2, cull) = self.resolve_dists();
        let bias = if self.lod_bias.is_finite() {
            self.lod_bias.clamp(0.25, 4.0)
        } else {
            1.0
        };
        (
            self.enabled,
            self.force_lod,
            bias.to_bits(),
            l1.to_bits(),
            l2.to_bits(),
            cull.to_bits(),
        )
    }
}

/// Device pointer targets for zero-copy GPU-direct crowd scatter into CSG + OptiX IAS buffers.
#[derive(Clone, Copy, Debug)]
pub struct GpuScatterTarget {
    /// Device pointer to `CsgGpuInstance` (`instances_buf`).
    pub csg_instances_ptr: u64,
    /// Device pointer to `[f32; 12]` previous-frame transforms (`prev_xforms_buf`).
    pub csg_prev_xforms_ptr: u64,
    /// Valid element count of `csg_instances_ptr` (and `csg_prev_xforms_ptr`).
    pub max_csg_instances: u32,
    /// Device pointer to `OptixInstance` array (`d_optix_instances`).
    pub optix_instances_ptr: u64,
    /// Leading classic instance offset (`rig.ias_classic_count`) in `d_optix_instances`.
    pub optix_slot_offset: u32,
    /// Total valid element count of `d_optix_instances`.
    pub max_optix_instances: u32,
    /// Bitmask of hidden semantic classes (`bit s == 1` masks semantic id `s < 64` to
    /// `LINES_OCCLUDER_MASK = 0x80` on visible OptiX instances).
    pub hidden_semantics: u64,
    /// When `true` (OSM eye-filter `OsmFeatureClass::DynamicActor` toggled off), mask every
    /// visible crowd bone's OptiX instance to `LINES_OCCLUDER_MASK = 0x80` so render rays skip it.
    pub hide_dynamic_actors: bool,
}

/// Summary returned from a single GPU crowd step.
#[derive(Clone, Copy, Debug, Default)]
pub struct CudaCrowdStepReport {
    /// Host wall time for enqueueing the GPU crowd step (ms).
    pub step_ms: f32,
    /// True when cumulative actor displacement exceeded [`CUDA_IAS_REBUILD_DISP_M`], requesting an
    /// amortized full OptiX IAS rebuild instead of an incremental `OPTIX_BUILD_OPERATION_UPDATE`.
    pub needs_full_ias_rebuild: bool,
}

/// Minimal `nv-demo`-only CUDA host bridge: amortized IAS displacement check and bounds-checked
/// CSG/OptiX scatter with `hidden_semantics` and OSM `DynamicActor` visibility masking.
const CROWD_BRIDGE_CUH: &str = r#"
#if defined(__CUDACC_RTC__)
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;
typedef unsigned long long size_t;
#else
#include <stddef.h>
#include <stdint.h>
#endif

struct alignas(16) BridgeActorStatePod {
    float pos_yaw[4];
    float vel_yaw_rate[4];
    float gait_susp[4];
    float bal_angle_vel[4];
    float foot_lock[8];
    float gait_blend[4];
};
static_assert(sizeof(BridgeActorStatePod) == 112, "ActorStatePod must be 112 bytes");

struct alignas(16) BridgeBoneOutPod {
    float xform[12];
    uint32_t compound_id;
    uint32_t user_id;
    uint32_t mat_override;
    uint32_t flags;
};
static_assert(sizeof(BridgeBoneOutPod) == 64, "BoneOutPod must be 64 bytes");

struct alignas(16) BridgeOptixInstancePod {
    float transform[12];
    uint32_t instanceId;
    uint32_t sbtOffset;
    uint32_t visibilityMask;
    uint32_t flags;
    uint64_t traversableHandle;
    uint32_t pad[2];
};
static_assert(sizeof(BridgeOptixInstancePod) == 80, "OptixInstancePod must be 80 bytes");

extern "C" __global__ void check_crowd_ias_displacement(
    uint32_t actor_count,
    const BridgeActorStatePod* __restrict__ states,
    float4* __restrict__ rebuild_pos,
    float max_disp_sq,
    uint32_t reset_anchors,
    uint32_t* __restrict__ disp_flag) {
    const uint32_t i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= actor_count) {
        return;
    }
    const BridgeActorStatePod st = states[i];
    if (reset_anchors != 0u) {
        rebuild_pos[i] = make_float4(st.pos_yaw[0], st.pos_yaw[1], st.pos_yaw[2], 0.0f);
        return;
    }
    const float4 prev = rebuild_pos[i];
    const float dx = st.pos_yaw[0] - prev.x;
    const float dy = st.pos_yaw[1] - prev.y;
    const float dz = st.pos_yaw[2] - prev.z;
    const float d2 = dx * dx + dy * dy + dz * dz;
    if (isfinite(d2) && d2 > max_disp_sq) {
        atomicOr(disp_flag, 1u);
    }
}

extern "C" __global__ void scatter_crowd_to_csg_and_optix(
    const BridgeBoneOutPod* __restrict__ src_bones,
    const float* __restrict__ src_prev_xforms,
    const uint32_t* __restrict__ bone_slot_map,
    uint32_t total_bones,
    uint32_t max_csg_instances,
    uint32_t extra_csg_flags,
    BridgeBoneOutPod* __restrict__ dst_csg_instances,
    float* __restrict__ dst_csg_prev_xforms,
    BridgeOptixInstancePod* __restrict__ dst_optix_instances,
    uint32_t optix_slot_offset,
    uint32_t max_optix_instances,
    const uint64_t* __restrict__ compound_gas_handles,
    const uint32_t* __restrict__ compound_sbt_offsets,
    uint32_t compound_count,
    uint64_t hidden_semantics,
    uint32_t hide_dynamic_actors) {
    const uint32_t gidx = blockIdx.x * blockDim.x + threadIdx.x;
    if (gidx >= total_bones) {
        return;
    }
    const uint32_t out_idx = bone_slot_map[gidx];
    if (out_idx == 0xFFFFFFFFu || out_idx >= max_csg_instances) {
        return;
    }

    BridgeBoneOutPod pod = src_bones[gidx];
    const bool visible = (pod.flags & 1u) != 0u;
    if (visible) {
        pod.flags |= extra_csg_flags;
    }
    dst_csg_instances[out_idx] = pod;

    if (dst_csg_prev_xforms != nullptr && src_prev_xforms != nullptr) {
        const float* sp = src_prev_xforms + (size_t)gidx * 12u;
        float* dp = dst_csg_prev_xforms + (size_t)out_idx * 12u;
        for (int k = 0; k < 12; ++k) {
            dp[k] = sp[k];
        }
    }

    if (dst_optix_instances != nullptr) {
        const uint32_t oi_idx = optix_slot_offset + out_idx;
        if (oi_idx < max_optix_instances) {
            BridgeOptixInstancePod oi;
            for (int k = 0; k < 12; ++k) {
                oi.transform[k] = pod.xform[k];
            }
            oi.instanceId = out_idx;
            const uint32_t cid = pod.compound_id;
            oi.sbtOffset = (compound_sbt_offsets != nullptr && cid < compound_count)
                ? compound_sbt_offsets[cid]
                : 10u;
            const uint32_t over = (pod.flags >> 16u) & 0xFFu;
            // Honor both the semantic filter bitmask and the OSM `DynamicActor` eye-filter toggle:
            // hidden crowd bones keep only `LINES_OCCLUDER_MASK` (`0x80`) so render rays skip them.
            const bool sem_hidden = (hide_dynamic_actors != 0u)
                || ((over != 0u) && ((over - 1u) < 64u)
                    && (((hidden_semantics >> (over - 1u)) & 1ull) != 0ull));
            oi.visibilityMask = visible ? (sem_hidden ? 0x80u : 0xFFu) : 0u;
            oi.flags = 4u; // OPTIX_INSTANCE_FLAG_DISABLE_ANYHIT
            oi.traversableHandle = (compound_gas_handles != nullptr && cid < compound_count)
                ? compound_gas_handles[cid]
                : 0ull;
            oi.pad[0] = 0u;
            oi.pad[1] = 0u;
            dst_optix_instances[oi_idx] = oi;
        }
    }
}
"#;

/// Grow-only typed GPU byte buffer for the small host-bridge arrays (scatter slot map, GAS/SBT
/// tables, IAS rebuild anchors, and 4-byte displacement flag).
pub(crate) struct GrowCudaBuf<T: Pod> {
    buf: Option<DeviceBuffer<u8>>,
    cap_bytes: usize,
    len_elems: usize,
    _marker: PhantomData<T>,
}

impl<T: Pod> Default for GrowCudaBuf<T> {
    fn default() -> Self {
        Self {
            buf: None,
            cap_bytes: 0,
            len_elems: 0,
            _marker: PhantomData,
        }
    }
}

impl<T: Pod> GrowCudaBuf<T> {
    #[inline]
    pub(crate) fn device_ptr(&self) -> u64 {
        if self.len_elems == 0 {
            0
        } else {
            self.buf.as_ref().map_or(0, DeviceBuffer::cu_deviceptr)
        }
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.len_elems
    }

    #[inline]
    pub(crate) fn cap_bytes(&self) -> usize {
        self.cap_bytes
    }

    pub(crate) fn ensure_len(&mut self, stream: &Stream, count: usize) -> Result<(), String> {
        let bytes = count.saturating_mul(std::mem::size_of::<T>());
        if bytes > self.cap_bytes {
            let alloc_bytes = bytes.max(256).next_power_of_two();
            let new_buf = DeviceBuffer::<u8>::zeroed(stream, alloc_bytes)
                .map_err(|e| format!("CUDA bridge alloc ({alloc_bytes} B) failed: {e}"))?;
            self.buf = Some(new_buf);
            self.cap_bytes = alloc_bytes;
        }
        self.len_elems = count;
        Ok(())
    }

    pub(crate) fn upload(&mut self, stream: &Stream, data: &[T]) -> Result<(), String> {
        self.ensure_len(stream, data.len())?;
        if data.is_empty() {
            return Ok(());
        }
        let raw: &[u8] = bytemuck::cast_slice(data);
        let ptr = self.device_ptr();
        // SAFETY: `self.buf` has capacity >= `raw.len()` bytes on `stream`; `T: Pod` guarantees
        // contiguous, initialized bytes and `cuMemcpyHtoDAsync_v2` reads `raw.len()` bytes.
        let rc = unsafe {
            cu::cuMemcpyHtoDAsync_v2(ptr, raw.as_ptr().cast(), raw.len(), stream.cu_stream())
        };
        if rc != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("CUDA bridge H2D copy failed ({rc:?})"));
        }
        Ok(())
    }
}

/// `true` while the device crowd buffers hold the crowd `scene.crowd` currently describes: no
/// pending scene re-upload and the same `(actors, bones)` shape as the last upload.
#[inline]
fn gpu_snapshot_matches_crowd(
    scene_dirty: bool,
    uploaded: (usize, usize),
    current: (usize, usize),
) -> bool {
    !scene_dirty && uploaded == current
}

/// Checks that a GPU scatter may run: the bone -> slot map must have been uploaded for the current
/// CSG/OptiX instance layout (not stale after a `FullRepack`) and cover every bone.
fn check_scatter_topology(
    topology_dirty: bool,
    slot_map_len: usize,
    n_bones: usize,
) -> Result<(), String> {
    if topology_dirty {
        return Err(
            "CUDA crowd scatter topology is stale (upload_scatter_topology has not succeeded since the last repack)"
                .to_string(),
        );
    }
    if slot_map_len < n_bones {
        return Err(format!(
            "CUDA crowd bone slot map covers {slot_map_len} of {n_bones} bones"
        ));
    }
    Ok(())
}

/// Builds the static per-scene skeleton bone-edge table `[parent_gidx, child_gidx, obj_key, 0]`
/// for every actor in `scene.crowd` so `ovl_skeletons` can draw articulated skeletons directly
/// from device-resident `BufId::CrowdWorldJoints` with zero per-frame host work.
fn build_skeleton_edges(scene: &SceneBuilt, out: &mut Vec<[u32; 4]>) {
    out.clear();
    let n_joints = scene.crowd.world_joints.len();
    if n_joints == 0 {
        return;
    }
    out.reserve(n_joints);

    for desc in &scene.crowd.descs {
        let Some(skel) = scene.crowd.archetype_skeleton(desc.archetype_id as usize) else {
            continue;
        };
        // Resolve the actor's P42 logical `object_id` from its root bone's dynamic provenance
        // so `Selected` overlay scope and highlight match viewport picking (`pick::csg_selection_key`).
        let obj_id = scene
            .crowd_inst_ids
            .get(desc.bone_offset as usize)
            .and_then(|&inst_id| scene.world.dynamic_index(inst_id))
            .and_then(|dyn_idx| scene.world.dynamic_provenance_at(dyn_idx))
            .map_or(csg_scene::provenance::OBJECT_ID_NONE, |prov| prov.object_id);
        let obj_key = crate::pick::csg_selection_key(obj_id, desc.user_id);

        for (k, joint) in skel.joints.iter().enumerate() {
            let Some(parent_u16) = joint.parent else {
                continue;
            };
            let parent = usize::from(parent_u16);
            if parent >= k {
                continue;
            }
            let (Some(&p_slot), Some(&c_slot)) = (
                skel.world_joint_index.get(parent),
                skel.world_joint_index.get(k),
            ) else {
                continue;
            };
            let p_gidx = desc.bone_offset.saturating_add(p_slot);
            let c_gidx = desc.bone_offset.saturating_add(c_slot);
            if (p_gidx as usize) < n_joints && (c_gidx as usize) < n_joints {
                out.push([p_gidx, c_gidx, obj_key, 0]);
            }
        }
    }
}

/// Uploads a `BufClass::Table` buffer only when its generation changed or its slot reallocated.
fn upload_table_if_needed<T: Pod>(
    plan: &mut GpuFramePlan,
    stream: &Stream,
    sizes: &dyn FrameSizes,
    id: BufId,
    data: &[T],
) -> Result<(), String> {
    let gen_id = sizes.table_generation(id);
    if plan.table_needs_upload(id, gen_id) {
        plan.upload(stream, id, data)?;
        plan.mark_table_uploaded(id, gen_id);
    }
    Ok(())
}

/// Persistent CUDA/NVRTC articulated crowd simulator backed by `csg-rs`'s `PlanKind::Crowd` v2
/// [`GpuFramePlan`] and a minimal CSG/OptiX scatter bridge.
pub struct CudaCrowdBackend {
    cuda: Arc<CudaContext>,
    plan: GpuFramePlan,
    bridge_module: cu::CUmodule,
    scatter_fn: cu::CUfunction,
    disp_fn: cu::CUfunction,
    steer_scene: Option<CrowdSteerScene>,
    generations: CrowdTableGenerations,
    pending_events: Vec<EventId>,
    // Host-bridge buffers (nv-demo OptiX scatter & amortized IAS rebuild check).
    d_bone_slot_map: GrowCudaBuf<u32>,
    d_gas_handles: GrowCudaBuf<u64>,
    d_sbt_offsets: GrowCudaBuf<u32>,
    d_rebuild_pos: GrowCudaBuf<[f32; 4]>,
    d_disp_flag: GrowCudaBuf<u32>,
    /// Static `(parent_gidx, child_gidx, obj_key, 0)` bone-edge table uploaded once per scene for
    /// zero-copy GPU skeleton overlay rendering (`ovl_skeletons`).
    d_skeleton_edges: GrowCudaBuf<[u32; 4]>,
    // 12 B page-locked host status: [CrowdFault, CrowdGroundMisses, disp_flag].
    pinned_status: PinnedHostBuf<u32>,
    has_pending_readback: bool,
    pending_ias_rebuild: bool,
    // Reusable host scratch vectors (zero steady-state per-frame heap allocations).
    host_archetypes: Vec<ArchetypeMetaPod>,
    host_vehicles: Vec<CrowdVehiclePod>,
    host_rebuild_pos: Vec<[f32; 4]>,
    host_skeleton_edges: Vec<[u32; 4]>,
    staging_states: Vec<ActorStatePod>,
    staging_bone_outs: Vec<BoneOutPod>,
    staging_prev_xforms: Vec<[f32; 12]>,
    staging_world_joints: Vec<[f32; 4]>,
    staging_intents: Vec<csg_actors::IntentPod>,
    /// Dedicated host buffer kept immutable after `cuMemcpyHtoDAsync_v2` until the next
    /// `intents_dirty` upload so clearing one-shot `MASK_IMPULSE` bits in `scene.crowd.intents`
    /// never races with an in-flight async PCIe transfer (>64 KiB on 2,000-actor crowds).
    upload_intents: Vec<csg_actors::IntentPod>,
    scene_dirty: bool,
    intents_dirty: bool,
    topology_dirty: bool,
    cpu_bones_stale: bool,
    uploaded_actor_count: usize,
    uploaded_bone_count: usize,
    last_fault: u32,
    ias_rebuild_count: u32,
}

impl Drop for CudaCrowdBackend {
    fn drop(&mut self) {
        if !self.bridge_module.is_null() && self.cuda.bind_to_thread().is_ok() {
            // SAFETY: `self.bridge_module` was loaded via `cuModuleLoadData` on `self.cuda` and
            // is unloaded exactly once when `CudaCrowdBackend` drops.
            let rc = unsafe { cu::cuModuleUnload(self.bridge_module) };
            if rc != cu::cudaError_enum_CUDA_SUCCESS {
                eprintln!("nv-demo: CudaCrowdBackend bridge cuModuleUnload failed ({rc:?})");
            }
            self.bridge_module = std::ptr::null_mut();
        }
    }
}

impl CudaCrowdBackend {
    /// Compiles and validates `csg_scene::crowd_gpu::crowd_frame_plan(csg_device::CSG_EVAL_CUH)`
    /// (version 2) plus the compact `CROWD_BRIDGE_CUH` scatter/displacement module on `cuda`.
    pub fn new(cuda: &Arc<CudaContext>) -> Result<Self, String> {
        cuda.bind_to_thread()
            .map_err(|e| format!("CUDA bind failed for CudaCrowdBackend: {e}"))?;

        let plan = GpuFramePlan::new(
            cuda,
            crowd_frame_plan(csg_device::CSG_EVAL_CUH),
            CROWD_FRAME_PLAN_VERSION,
            "crowd_frame_v2.cu",
        )?;

        let bridge_ptx = compile_nvrtc_ptx(CROWD_BRIDGE_CUH, "crowd_host_bridge.cu")?;
        let mut bridge_module: cu::CUmodule = std::ptr::null_mut();
        // SAFETY: `bridge_ptx` is a NUL-terminated PTX buffer and `cuda` is current on this thread.
        let rc = unsafe { cu::cuModuleLoadData(&mut bridge_module, bridge_ptx.as_ptr().cast()) };
        if rc != cu::cudaError_enum_CUDA_SUCCESS || bridge_module.is_null() {
            return Err(format!(
                "cuModuleLoadData(crowd_host_bridge.cu) failed ({rc:?})"
            ));
        }

        let init_bridge = || -> Result<(cu::CUfunction, cu::CUfunction, PinnedHostBuf<u32>), String> {
            let scatter_fn = load_cuda_fn(bridge_module, "scatter_crowd_to_csg_and_optix")?;
            let disp_fn = load_cuda_fn(bridge_module, "check_crowd_ias_displacement")?;
            let mut pinned = PinnedHostBuf::<u32>::new(cuda, STATUS_WORD_COUNT)?;
            pinned.write(STATUS_FAULT_IDX, CROWD_FAULT_NONE)?;
            pinned.write(STATUS_GROUND_MISSES_IDX, 0)?;
            pinned.write(STATUS_DISP_FLAG_IDX, 0)?;
            Ok((scatter_fn, disp_fn, pinned))
        };
        let (scatter_fn, disp_fn, pinned_status) = match init_bridge() {
            Ok(v) => v,
            Err(err) => {
                // SAFETY: unloading valid `bridge_module` on init error prevents leaking it.
                let urc = unsafe { cu::cuModuleUnload(bridge_module) };
                if urc != cu::cudaError_enum_CUDA_SUCCESS {
                    eprintln!(
                        "nv-demo: cuModuleUnload(crowd_host_bridge.cu) on error path failed ({urc:?})"
                    );
                }
                return Err(err);
            }
        };

        Ok(Self {
            cuda: cuda.clone(),
            plan,
            bridge_module,
            scatter_fn,
            disp_fn,
            steer_scene: None,
            generations: CrowdTableGenerations::default(),
            pending_events: Vec::with_capacity(2),
            d_bone_slot_map: GrowCudaBuf::default(),
            d_gas_handles: GrowCudaBuf::default(),
            d_sbt_offsets: GrowCudaBuf::default(),
            d_rebuild_pos: GrowCudaBuf::default(),
            d_disp_flag: GrowCudaBuf::default(),
            d_skeleton_edges: GrowCudaBuf::default(),
            pinned_status,
            has_pending_readback: false,
            pending_ias_rebuild: false,
            host_archetypes: Vec::with_capacity(8),
            host_vehicles: Vec::with_capacity(64),
            host_rebuild_pos: Vec::with_capacity(64),
            host_skeleton_edges: Vec::new(),
            staging_states: Vec::new(),
            staging_bone_outs: Vec::new(),
            staging_prev_xforms: Vec::new(),
            staging_world_joints: Vec::new(),
            staging_intents: Vec::new(),
            upload_intents: Vec::new(),
            scene_dirty: true,
            intents_dirty: true,
            topology_dirty: true,
            cpu_bones_stale: false,
            uploaded_actor_count: 0,
            uploaded_bone_count: 0,
            last_fault: CROWD_FAULT_NONE,
            ias_rebuild_count: 0,
        })
    }

    /// Marks all crowd tables and device state dirty (call after scene rebuild or respawn).
    #[inline]
    pub fn mark_scene_dirty(&mut self) {
        self.scene_dirty = true;
        self.intents_dirty = true;
        self.topology_dirty = true;
        self.cpu_bones_stale = false;
        self.has_pending_readback = false;
        self.pending_ias_rebuild = false;
        self.steer_scene = None;
        self.pending_events.clear();
        self.last_fault = CROWD_FAULT_NONE;
        self.ias_rebuild_count = 0;
    }

    /// Marks `IntentPod` buffer dirty (call after `poke_crowd` or `freeze_pose` toggle).
    #[inline]
    pub fn mark_intents_dirty(&mut self) {
        self.intents_dirty = true;
    }

    /// Marks GPU scatter mapping dirty (call after `CsgSyncResult::FullRepack`).
    #[inline]
    pub fn mark_topology_dirty(&mut self) {
        self.topology_dirty = true;
    }

    /// Returns `true` if GPU bone outputs have advanced ahead of CPU `CrowdSim::bone_outs`.
    #[inline]
    pub fn cpu_bones_stale(&self) -> bool {
        self.cpu_bones_stale
    }

    /// Returns `(cu_deviceptr, bone_count)` of the device-resident `BufId::CrowdBoneOuts`
    /// buffer for zero-copy binding into downstream `FramePlan`s (such as `PlanKind::Camera`).
    #[inline]
    pub fn bone_outs_device_ptr(&self) -> (u64, u32) {
        (
            self.plan.device_ptr(BufId::CrowdBoneOuts).unwrap_or(0),
            self.plan.elem_count(BufId::CrowdBoneOuts),
        )
    }

    /// Returns `(cu_deviceptr, actor_count)` of the device-resident `BufId::CrowdStates` buffer
    /// (`ActorStatePod[N]`, 112 B stride) for zero-copy VRAM binding into `PlanKind::Traffic` v1
    /// (pedestrian AEB/yield) and `ovl_adas` hazard tether rendering.
    ///
    /// Returns `(0, 0)` while `scene_dirty` is set so a scene switch never exposes stale actors.
    #[inline]
    pub fn states_device_ptr(&self) -> (u64, u32) {
        if self.scene_dirty {
            return (0, 0);
        }
        (
            self.plan.device_ptr(BufId::CrowdStates).unwrap_or(0),
            self.plan.elem_count(BufId::CrowdStates),
        )
    }

    /// Returns `(cu_deviceptr, bone_count)` of device-resident `BufId::CrowdWorldJoints`
    /// (`float4[N]`, `[x, y, z, radius]`) for zero-copy GPU skeleton overlay rendering.
    ///
    /// Why: returns `(0, 0)` while `scene_dirty` is set so a scene switch to a crowdless world
    /// never exposes stale world-joint buffers from the previous scene.
    #[inline]
    pub fn world_joints_device_ptr(&self) -> (u64, u32) {
        if self.scene_dirty {
            return (0, 0);
        }
        (
            self.plan.device_ptr(BufId::CrowdWorldJoints).unwrap_or(0),
            self.plan.elem_count(BufId::CrowdWorldJoints),
        )
    }

    /// Returns `(cu_deviceptr, edge_count)` of the static `(parent_gidx, child_gidx, obj_key, 0)`
    /// bone-edge table for `ovl_skeletons`.
    ///
    /// Why: returns `(0, 0)` while `scene_dirty` is set so `ovl_skeletons` never indexes an edge
    /// table built for a superseded scene.
    #[inline]
    pub fn skeleton_edges_device_ptr(&self) -> (u64, u32) {
        if self.scene_dirty {
            return (0, 0);
        }
        (
            self.d_skeleton_edges.device_ptr(),
            u32::try_from(self.d_skeleton_edges.len()).unwrap_or(0),
        )
    }

    /// Uploads updated `bone_slot_map`, per-compound GAS handles, and per-compound SBT offsets.
    pub fn upload_scatter_topology(
        &mut self,
        stream: &Stream,
        bone_slot_map: &[u32],
        gas_handles: &[u64],
        sbt_offsets: &[u32],
    ) -> Result<(), String> {
        self.d_bone_slot_map.upload(stream, bone_slot_map)?;
        self.d_gas_handles.upload(stream, gas_handles)?;
        self.d_sbt_offsets.upload(stream, sbt_offsets)?;
        self.topology_dirty = false;
        Ok(())
    }

    /// Returns `true` if scatter topology buffers need to be refreshed before scattering.
    #[inline]
    pub fn topology_dirty(&self) -> bool {
        self.topology_dirty
    }

    /// Total allocated VRAM bytes across `PlanKind::Crowd` buffers and the scatter/displacement
    /// bridge arrays.
    #[inline]
    pub fn gpu_bytes(&self) -> u64 {
        let bridge = self.d_bone_slot_map.cap_bytes()
            + self.d_gas_handles.cap_bytes()
            + self.d_sbt_offsets.cap_bytes()
            + self.d_rebuild_pos.cap_bytes()
            + self.d_disp_flag.cap_bytes()
            + self.d_skeleton_edges.cap_bytes();
        self.plan.gpu_bytes().saturating_add(bridge as u64)
    }

    /// Declarative `PlanKind::Crowd` version executed by this backend.
    #[inline]
    pub fn plan_version(&self) -> u32 {
        self.plan.plan().version
    }

    /// Total CUDA kernels launched per steady-state crowd step (`plan.frame` + 2 bridge kernels).
    #[inline]
    pub fn kernel_count(&self) -> u32 {
        (self.plan.plan().frame.len() + 2) as u32
    }

    /// NVRTC math compilation mode (`Fast` vs `StrictF32`) of the active crowd plan.
    #[inline]
    pub fn math_mode(&self) -> nvrtc::MathMode {
        self.plan.math_mode()
    }

    /// Number of static steering obstacles uploaded in `CrowdSteerScene`.
    #[inline]
    pub fn obstacle_count(&self) -> u32 {
        self.steer_scene
            .as_ref()
            .map_or(0, |s| s.obstacles.len() as u32)
    }

    /// Latest fault code observed from the 12-byte async pinned `CrowdGpuStatus` readback.
    #[inline]
    pub fn last_fault(&self) -> u32 {
        self.last_fault
    }

    /// Cumulative count of amortized full OptiX IAS rebuilds triggered by `CUDA_IAS_REBUILD_DISP_M`.
    #[inline]
    pub fn ias_rebuild_count(&self) -> u32 {
        self.ias_rebuild_count
    }

    /// Drains the previously queued 12-byte page-locked status/displacement DMA (whose stream
    /// already completed at the end-of-frame fence or an explicit sync) into `scene` and returns
    /// whether cumulative actor displacement requested a full IAS rebuild.
    fn drain_pending_status(&mut self, scene: &mut SceneBuilt) -> Result<bool, String> {
        let mut rebuild = std::mem::take(&mut self.pending_ias_rebuild);
        if self.has_pending_readback {
            let fault = self.pinned_status.read(STATUS_FAULT_IDX)?;
            let ground_misses = self.pinned_status.read(STATUS_GROUND_MISSES_IDX)?;
            let disp = self.pinned_status.read(STATUS_DISP_FLAG_IDX)?;
            self.last_fault = fault;
            if disp != 0 {
                self.ias_rebuild_count = self.ias_rebuild_count.saturating_add(1);
            }
            scene.apply_gpu_status(&CrowdGpuStatus {
                fault,
                ground_misses,
            });
            self.pinned_status.write(STATUS_FAULT_IDX, CROWD_FAULT_NONE)?;
            self.pinned_status.write(STATUS_GROUND_MISSES_IDX, 0)?;
            self.pinned_status.write(STATUS_DISP_FLAG_IDX, 0)?;
            self.has_pending_readback = false;
            rebuild |= disp != 0;
        }
        Ok(rebuild)
    }

    /// Builds `CrowdSteerScene` and uploads all cold `Table` + initial `DeviceState` buffers when
    /// the scene or crowd shape changes. Table uploads are gated on `CrowdTableGenerations`.
    fn ensure_scene_uploaded(
        &mut self,
        stream: &Stream,
        scene: &mut SceneBuilt,
    ) -> Result<(), String> {
        let n_actors = scene.crowd.actor_count();
        let n_bones = scene.crowd.bone_count();
        let size_changed =
            n_actors != self.uploaded_actor_count || n_bones != self.uploaded_bone_count;
        if !self.scene_dirty && !size_changed && self.steer_scene.is_some() {
            return Ok(());
        }

        let steer_scene = CrowdSteerScene::from_scene(scene)
            .map_err(|e| format!("CrowdSteerScene::from_scene failed: {e}"))?;
        self.generations.scene = self.generations.scene.wrapping_add(1).max(1);
        self.generations.library = self.generations.library.wrapping_add(1).max(1);
        self.generations.crowd = self.generations.crowd.wrapping_add(1).max(1);

        let frame_data = CrowdFrameData {
            scene: &steer_scene,
            library: &scene.world.library,
            crowd: &scene.crowd,
            vehicles: 0,
            optix_slot_offset: 0,
            generations: self.generations,
        };
        self.plan.sync_sizes(stream, &frame_data)?;

        // Upload `BufClass::Table` buffers only when their generation changed.
        scene
            .crowd
            .pack_archetype_metas_into(&mut self.host_archetypes);
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdArchetypes,
            &self.host_archetypes,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdDescs,
            &scene.crowd.descs,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdWalkways,
            &steer_scene.walkways,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdPoints,
            &steer_scene.points,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdLinks,
            &steer_scene.links,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdEndpoints,
            &steer_scene.endpoints,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdObstacles,
            &steer_scene.obstacles,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdObstacleStarts,
            &steer_scene.obstacle_starts,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdObstacleItems,
            &steer_scene.obstacle_items,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdSurface,
            &[steer_scene.ground.surface],
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdSurfaceData,
            &steer_scene.ground.data,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdSurfaceTiles,
            &steer_scene.ground.tiles,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdSupportGrid,
            &[steer_scene.support.grid],
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdSupportItems,
            &steer_scene.support.items,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdSupportTileStarts,
            &steer_scene.support.tile_starts,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::CrowdSupportTileItems,
            &steer_scene.support.tile_items,
        )?;

        let lib = &scene.world.library;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::LibCompounds,
            &lib.compounds,
        )?;
        upload_table_if_needed(&mut self.plan, stream, &frame_data, BufId::LibOps, &lib.ops)?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::LibPrims,
            &lib.prims,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::LibAux,
            &lib.aux_planes,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::LibGuards,
            &lib.guard_aabbs,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::LibRepeats,
            &lib.repeats,
        )?;

        // Seed `BufClass::DeviceState` buffers (`CrowdIntents` uploads in step 2 with impulse clearing).
        let navs = crowd_nav_pods(&scene.world.crowd_nav, n_actors);
        self.plan.upload(stream, BufId::CrowdStates, &scene.crowd.states)?;
        self.plan
            .upload(stream, BufId::CrowdGrounds, &scene.crowd.grounds)?;
        self.plan.upload(stream, BufId::CrowdNavs, &navs)?;
        self.plan.upload(stream, BufId::CrowdBones, &scene.crowd.bones)?;
        self.plan
            .upload(stream, BufId::CrowdBoneOuts, &scene.crowd.bone_outs)?;
        self.plan
            .upload(stream, BufId::CrowdPrevXforms, &scene.crowd.prev_xforms)?;
        self.plan
            .upload(stream, BufId::CrowdWorldJoints, &scene.crowd.world_joints)?;

        // Initialize per-frame status scratch slots.
        self.plan
            .upload(stream, BufId::CrowdFault, &[CROWD_FAULT_NONE])?;
        self.plan.zero_buffer(stream, BufId::CrowdGroundMisses, 1)?;

        // Queue nav relink event when `CrowdSteerScene` rebuilt its walkway network.
        self.pending_events.clear();
        if steer_scene.nav_reset_required {
            self.pending_events.push(EventId::CrowdNavRelink);
        }

        // Seed amortized IAS rebuild anchor positions, static skeleton bone edges, and status words.
        self.host_rebuild_pos.clear();
        self.host_rebuild_pos.reserve(n_actors);
        for st in &scene.crowd.states {
            self.host_rebuild_pos
                .push([st.pos_yaw[0], st.pos_yaw[1], st.pos_yaw[2], 0.0]);
        }
        self.d_rebuild_pos.upload(stream, &self.host_rebuild_pos)?;
        build_skeleton_edges(scene, &mut self.host_skeleton_edges);
        self.d_skeleton_edges
            .upload(stream, &self.host_skeleton_edges)?;
        self.d_disp_flag.ensure_len(stream, 1)?;
        self.pinned_status.write(STATUS_FAULT_IDX, CROWD_FAULT_NONE)?;
        self.pinned_status.write(STATUS_GROUND_MISSES_IDX, 0)?;
        self.pinned_status.write(STATUS_DISP_FLAG_IDX, 0)?;
        self.has_pending_readback = false;
        self.pending_ias_rebuild = false;

        self.steer_scene = Some(steer_scene);
        self.uploaded_actor_count = n_actors;
        self.uploaded_bone_count = n_bones;
        self.scene_dirty = false;
        self.intents_dirty = true;
        Ok(())
    }

    /// Synchronizes GPU `ReadbackWhen::OnRequest` buffers (`CrowdStates`, `CrowdBoneOuts`,
    /// `CrowdPrevXforms`, `CrowdWorldJoints`, `CrowdNavs`, `CrowdIntents`) back into `scene` via
    /// [`SceneBuilt::apply_gpu_poses`] (which atomically validates all six lengths, restores
    /// `scene.crowd` and `scene.world.crowd_nav`, and mirrors bones onto `scene.world` instances).
    ///
    /// Called on demand right before camera repacks, `PokeCrowd`, `Ctrl+Click` picking, Freeze
    /// Pose, or SDG JSON export. `_sync_to_world` is retained for signature compatibility with
    /// `renderer.rs`; `apply_gpu_poses` always mirrors to `scene.world` so `crowd_nav` and world
    /// instances stay consistent.
    pub fn sync_cpu_snapshot(
        &mut self,
        stream: &Stream,
        scene: &mut SceneBuilt,
        _sync_to_world: bool,
    ) -> Result<(), String> {
        if !self.cpu_bones_stale || scene.crowd.actor_count() == 0 {
            return Ok(());
        }
        let n_actors = scene.crowd.actor_count();
        let n_bones = scene.crowd.bone_count();
        // Device state matches `scene.crowd` only while the scene was not dirtied or reshaped.
        if gpu_snapshot_matches_crowd(
            self.scene_dirty,
            (self.uploaded_actor_count, self.uploaded_bone_count),
            (n_actors, n_bones),
        ) {
            self.plan
                .download_into_async(stream, BufId::CrowdStates, &mut self.staging_states)?;
            self.plan
                .download_into_async(stream, BufId::CrowdBoneOuts, &mut self.staging_bone_outs)?;
            self.plan.download_into_async(
                stream,
                BufId::CrowdPrevXforms,
                &mut self.staging_prev_xforms,
            )?;
            self.plan.download_into_async(
                stream,
                BufId::CrowdWorldJoints,
                &mut self.staging_world_joints,
            )?;
            self.plan
                .download_into_async(stream, BufId::CrowdIntents, &mut self.staging_intents)?;
            // One synchronous download completes all five preceding async D2H copies and any
            // pending status readback on `stream` with a single `stream.synchronize()`.
            let navs: Vec<CrowdNavPod> = self.plan.download_sync(stream, BufId::CrowdNavs)?;

            if self.has_pending_readback {
                let disp = self.drain_pending_status(scene)?;
                self.pending_ias_rebuild |= disp;
            }

            if self.intents_dirty {
                merge_pending_cpu_intents(&mut self.staging_intents, &scene.crowd.intents);
            }

            let poses = CrowdGpuPoses {
                states: &self.staging_states,
                bone_outs: &self.staging_bone_outs,
                prev_xforms: &self.staging_prev_xforms,
                world_joints: &self.staging_world_joints,
                navs: &navs,
                intents: &self.staging_intents,
            };
            scene
                .apply_gpu_poses(&poses)
                .map_err(|e| format!("SceneBuilt::apply_gpu_poses failed: {e:?}"))?;
        }
        self.cpu_bones_stale = false;
        Ok(())
    }

    /// Steps `scene.crowd` entirely on GPU through `PlanKind::Crowd` v2 (`GpuFramePlan`), scatters
    /// resolved bone transforms into `CsgGpuInstance`/`prev_xforms`/`OptixInstance` with semantic
    /// visibility masking, and schedules a 12-byte async status/displacement readback with **zero
    /// mid-frame host synchronization** when `readback_poses_for_cpu` is `false`.
    #[expect(
        clippy::too_many_arguments,
        reason = "coordinates CUDA stream, scene state, timestep, camera eye, crowd LOD params, optional GPU traffic vehicle pointer, scatter target, and CPU readback policy"
    )]
    pub fn step_gpu_direct(
        &mut self,
        stream: &Stream,
        scene: &mut SceneBuilt,
        dt: f32,
        time_s: f32,
        cam_eye: [f32; 3],
        lod: CrowdLodParams,
        traffic_crowd_vehicles: Option<(u64, u32)>,
        scatter: Option<GpuScatterTarget>,
        readback_poses_for_cpu: bool,
    ) -> Result<CudaCrowdStepReport, String> {
        let n_actors = scene.crowd.actor_count();
        let n_bones = scene.crowd.bone_count();
        if n_actors == 0 || n_bones == 0 {
            return Ok(CudaCrowdStepReport::default());
        }

        // Reject stale scatter topology before queueing anything on the stream so a failed step
        // leaves every device buffer untouched.
        let scatter = scatter.filter(|t| t.csg_instances_ptr != 0);
        if scatter.is_some() {
            check_scatter_topology(self.topology_dirty, self.d_bone_slot_map.len(), n_bones)?;
        }

        let t0 = Instant::now();

        // Apply resolved LOD distances onto `scene.crowd` so `CrowdFrameData` and CPU state agree.
        let (lod1, lod2, cull) = lod.resolve_dists();
        scene.crowd.lod1_dist_m = lod1;
        scene.crowd.lod2_dist_m = lod2;
        scene.crowd.cull_dist_m = cull;

        // 1. Drain previous frame's 12-byte pinned status/displacement DMA (zero mid-frame sync!).
        let needs_full_ias_rebuild = self.drain_pending_status(scene)?;

        // 2. Upload cold scene/library/crowd tables & seed state if scene or crowd shape changed.
        self.ensure_scene_uploaded(stream, scene)?;

        // Resolve frame dt before uploading intents so CPU impulse consumption mirrors
        // `step_actors_cuda` (`substeps > 0 && !frozen`, via `policy::mark_impulse_consumed`).
        let step_dt = if dt.is_finite() {
            dt.clamp(0.0, 1.0 / 30.0)
        } else {
            0.0
        };

        // 3. Upload `CrowdIntents` only when externally modified (init, PokeCrowd, FreezePose).
        // Copy into `self.upload_intents` first so clearing one-shot `MASK_IMPULSE` bits on CPU
        // never mutates the source slice while `cuMemcpyHtoDAsync_v2` is still reading it.
        if self.intents_dirty {
            self.upload_intents.clear();
            self.upload_intents
                .extend_from_slice(&scene.crowd.intents);
            self.plan
                .upload(stream, BufId::CrowdIntents, &self.upload_intents)?;
            for intent in &mut scene.crowd.intents {
                csg_actors::policy::mark_impulse_consumed(intent, step_dt);
            }
            self.intents_dirty = false;
        }

        // 4. Collect per-frame traffic vehicles (VRAM-direct D2D when GPU traffic ran, else host
        // extraction) and uniforms (`dt == 0` allowed for paused LOD refresh).
        let (vehicle_count, traffic_active) = match traffic_crowd_vehicles {
            Some((ptr, count)) if ptr != 0 && count > 0 => {
                self.plan
                    .copy_from_device_ptr(stream, BufId::CrowdVehicles, ptr, count)?;
                (count as usize, true)
            }
            _ => {
                let active = crowd_vehicle_pods(&scene.traffic, &mut self.host_vehicles);
                self.plan
                    .upload(stream, BufId::CrowdVehicles, &self.host_vehicles)?;
                (self.host_vehicles.len(), active)
            }
        };
        let steer_u = {
            let steer_scene = self
                .steer_scene
                .as_ref()
                .ok_or_else(|| "CrowdSteerScene missing after ensure_scene_uploaded".to_string())?;
            steer_scene
                .uniforms(step_dt, n_actors, vehicle_count, traffic_active)
                .map_err(|e| format!("CrowdSteerScene::uniforms failed: {e}"))?
        };
        let sim_u = crowd_sim_uniforms(&scene.crowd, step_dt, time_s, cam_eye)
            .map_err(|e| format!("crowd_sim_uniforms failed: {e}"))?;

        self.plan
            .upload(stream, BufId::CrowdSteerUniforms, &[steer_u])?;
        self.plan
            .upload(stream, BufId::CrowdSimUniforms, &[sim_u])?;
        // Reset per-frame ground-miss atomic counter on device (`crowd_grid_clear` resets fault).
        self.plan.zero_buffer(stream, BufId::CrowdGroundMisses, 1)?;

        // 5. Execute `PlanKind::Crowd` v2 on `stream` with null OptiX externals so `emit_actors_cuda`
        // keeps `CrowdBoneOuts` / `CrowdPrevXforms` compactly bone-indexed (`0..n_bones`) for both
        // downstream `PlanKind::Camera` binding and our semantic-masked scatter bridge below.
        {
            let steer_scene = self
                .steer_scene
                .as_ref()
                .ok_or_else(|| "CrowdSteerScene missing before plan execute".to_string())?;
            let frame_data = CrowdFrameData {
                scene: steer_scene,
                library: &scene.world.library,
                crowd: &scene.crowd,
                vehicles: vehicle_count,
                optix_slot_offset: 0,
                generations: self.generations,
            };
            let externals = [
                (ExternalId::OptixInstances, 0u64),
                (ExternalId::BoneSlotMap, 0u64),
                (ExternalId::CompoundGasHandles, 0u64),
                (ExternalId::CompoundSbtOffsets, 0u64),
            ];
            let events = std::mem::take(&mut self.pending_events);
            self.plan
                .execute(stream, &frame_data, &frame_data, &events, &externals, &[])?;
        }

        // 6. Scatter GPU-directly into `csg_instances`, `csg_prev_xforms`, and `d_optix_instances`
        // with `hidden_semantics` visibility masking in one pass.
        if let Some(target) = scatter {
            let mut p_src_bones = self
                .plan
                .device_ptr(BufId::CrowdBoneOuts)
                .ok_or_else(|| "CrowdBoneOuts device ptr missing".to_string())?;
            let mut p_src_prev = self
                .plan
                .device_ptr(BufId::CrowdPrevXforms)
                .ok_or_else(|| "CrowdPrevXforms device ptr missing".to_string())?;
            let mut p_slot_map = self.d_bone_slot_map.device_ptr();
            let mut total_bones_u32 = u32::try_from(n_bones)
                .map_err(|_| format!("crowd bone count {n_bones} overflows u32"))?;
            let mut max_csg = target.max_csg_instances;
            let mut extra_flags = SCATTER_VISIBLE_EXTRA_FLAGS;
            let mut p_dst_csg = target.csg_instances_ptr;
            let mut p_dst_prev = target.csg_prev_xforms_ptr;
            let mut p_dst_optix = target.optix_instances_ptr;
            let mut optix_off = target.optix_slot_offset;
            let mut max_optix = target.max_optix_instances;
            let mut p_gas = self.d_gas_handles.device_ptr();
            let mut p_sbt = self.d_sbt_offsets.device_ptr();
            let mut compound_count =
                self.d_gas_handles.len().min(self.d_sbt_offsets.len()) as u32;
            let mut hidden_sem = target.hidden_semantics;
            let mut hide_actors = u32::from(target.hide_dynamic_actors);

            let mut scat_args: [*mut c_void; 16] = [
                (&raw mut p_src_bones).cast(),
                (&raw mut p_src_prev).cast(),
                (&raw mut p_slot_map).cast(),
                (&raw mut total_bones_u32).cast(),
                (&raw mut max_csg).cast(),
                (&raw mut extra_flags).cast(),
                (&raw mut p_dst_csg).cast(),
                (&raw mut p_dst_prev).cast(),
                (&raw mut p_dst_optix).cast(),
                (&raw mut optix_off).cast(),
                (&raw mut max_optix).cast(),
                (&raw mut p_gas).cast(),
                (&raw mut p_sbt).cast(),
                (&raw mut compound_count).cast(),
                (&raw mut hidden_sem).cast(),
                (&raw mut hide_actors).cast(),
            ];
            let s_block = 256u32;
            let s_grid = total_bones_u32.div_ceil(s_block).max(1);
            // SAFETY: `self.scatter_fn` matches the 16-parameter ABI of
            // `scatter_crowd_to_csg_and_optix`; all source slots hold `n_bones` items and
            // destination writes are bounds-checked against `max_csg_instances`/`max_optix_instances`.
            let rc = unsafe {
                cu::cuLaunchKernel(
                    self.scatter_fn,
                    s_grid,
                    1,
                    1,
                    s_block,
                    1,
                    1,
                    0,
                    stream.cu_stream(),
                    scat_args.as_mut_ptr(),
                    std::ptr::null_mut(),
                )
            };
            if rc != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!(
                    "cuLaunchKernel(scatter_crowd_to_csg_and_optix) failed ({rc:?})"
                ));
            }
        }

        // 7. Zero `d_disp_flag`, run amortized displacement check, and enqueue 12-byte pinned D2H
        // (`ReadbackWhen::Always` status words + displacement flag; zero mid-frame sync!).
        {
            let disp_ptr = self.d_disp_flag.device_ptr();
            // SAFETY: `d_disp_flag` owns at least 4 bytes on `stream`.
            let zrc = unsafe { cu::cuMemsetD32Async(disp_ptr, 0, 1, stream.cu_stream()) };
            if zrc != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!("cuMemsetD32Async(d_disp_flag) failed ({zrc:?})"));
            }

            let mut actor_count_u32 = u32::try_from(n_actors)
                .map_err(|_| format!("crowd actor count {n_actors} overflows u32"))?;
            let mut p_states = self
                .plan
                .device_ptr(BufId::CrowdStates)
                .ok_or_else(|| "CrowdStates device ptr missing".to_string())?;
            let mut p_rebuild = self.d_rebuild_pos.device_ptr();
            let mut max_disp_sq = CUDA_IAS_REBUILD_DISP_M * CUDA_IAS_REBUILD_DISP_M;
            let mut reset_anchors = u32::from(needs_full_ias_rebuild);
            let mut p_flag = disp_ptr;
            let mut disp_args: [*mut c_void; 6] = [
                (&raw mut actor_count_u32).cast(),
                (&raw mut p_states).cast(),
                (&raw mut p_rebuild).cast(),
                (&raw mut max_disp_sq).cast(),
                (&raw mut reset_anchors).cast(),
                (&raw mut p_flag).cast(),
            ];
            let block_x = 128u32;
            let grid_x = actor_count_u32.div_ceil(block_x).max(1);
            // SAFETY: `self.disp_fn` matches the 6-parameter ABI of `check_crowd_ias_displacement`;
            // `CrowdStates` and `d_rebuild_pos` hold `n_actors` items and `d_disp_flag` holds 1 `u32`.
            let drc = unsafe {
                cu::cuLaunchKernel(
                    self.disp_fn,
                    grid_x,
                    1,
                    1,
                    block_x,
                    1,
                    1,
                    0,
                    stream.cu_stream(),
                    disp_args.as_mut_ptr(),
                    std::ptr::null_mut(),
                )
            };
            if drc != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!(
                    "cuLaunchKernel(check_crowd_ias_displacement) failed ({drc:?})"
                ));
            }

            // Enqueue every `ReadbackWhen::Always` buffer listed by the plan into `pinned_status`.
            for id in self.plan.plan().readback_ids(ReadbackWhen::Always) {
                let dst_idx = match id {
                    BufId::CrowdFault => STATUS_FAULT_IDX,
                    BufId::CrowdGroundMisses => STATUS_GROUND_MISSES_IDX,
                    other => {
                        return Err(format!(
                            "PlanKind::Crowd has unexpected ReadbackWhen::Always buffer {other:?}"
                        ));
                    }
                };
                self.plan.copy_to_pinned_buf_async(
                    stream,
                    id,
                    &mut self.pinned_status,
                    dst_idx,
                    1,
                )?;
            }
            // SAFETY: `pinned_status` owns `STATUS_WORD_COUNT` page-locked `u32` words; index
            // `STATUS_DISP_FLAG_IDX` is writable and remains live until `stream` completes.
            let flag_rc = unsafe {
                cu::cuMemcpyDtoHAsync_v2(
                    self.pinned_status
                        .as_mut_ptr()
                        .add(STATUS_DISP_FLAG_IDX)
                        .cast(),
                    disp_ptr,
                    std::mem::size_of::<u32>(),
                    stream.cu_stream(),
                )
            };
            if flag_rc != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!(
                    "CUDA crowd async displacement flag D2H failed ({flag_rc:?})"
                ));
            }
            self.has_pending_readback = true;
        }

        self.cpu_bones_stale = true;
        if readback_poses_for_cpu {
            self.sync_cpu_snapshot(stream, scene, true)?;
        }

        Ok(CudaCrowdStepReport {
            step_ms: t0.elapsed().as_secs_f32() * 1000.0,
            needs_full_ias_rebuild,
        })
    }
}

/// Preserves pending CPU-side `MASK_IMPULSE` (with `impulse_stance[0..3]`) and `FREEZE_POSE`
/// intents when a GPU pose readback (`sync_cpu_snapshot`) runs before `intents_dirty` was uploaded,
/// so an intervening sync never silently drops a queued `PokeCrowd` or `FreezePose` command.
fn merge_pending_cpu_intents(
    gpu_intents: &mut [csg_actors::IntentPod],
    cpu_intents: &[csg_actors::IntentPod],
) {
    for (gpu, cpu) in gpu_intents.iter_mut().zip(cpu_intents.iter()) {
        if (cpu.flags & intent_flags::MASK_IMPULSE) != 0 {
            gpu.impulse_stance[0] = cpu.impulse_stance[0];
            gpu.impulse_stance[1] = cpu.impulse_stance[1];
            gpu.impulse_stance[2] = cpu.impulse_stance[2];
            gpu.flags |= intent_flags::MASK_IMPULSE;
        }
        if (cpu.flags & intent_flags::FREEZE_POSE) != 0 {
            gpu.flags |= intent_flags::FREEZE_POSE;
        } else {
            gpu.flags &= !intent_flags::FREEZE_POSE;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crowd_lod_params_scale_and_signature() {
        assert_eq!(CrowdLodParams::decode_force_lod(0), None);
        assert_eq!(CrowdLodParams::decode_force_lod(1), Some(0));
        assert_eq!(CrowdLodParams::decode_force_lod(2), Some(1));
        assert_eq!(CrowdLodParams::decode_force_lod(3), Some(2));

        let (l1, l2, cull) = scale_crowd_lod_dists(20.0, 60.0, 150.0, 2.0);
        assert!((l1 - 40.0).abs() < 1e-5);
        assert!((l2 - 120.0).abs() < 1e-5);
        assert!((cull - 300.0).abs() < 1e-5);

        // Non-finite inputs fall back deterministically and remain monotonic.
        let (nl1, nl2, ncull) = scale_crowd_lod_dists(f32::NAN, -5.0, 0.0, f32::INFINITY);
        assert!(nl1 > 0.0 && nl2 >= nl1 && ncull >= nl2);

        let disabled = CrowdLodParams {
            enabled: false,
            ..CrowdLodParams::default()
        };
        let (dl1, dl2, dcull) = disabled.resolve_dists();
        assert!(dl1 >= 1.0e8 && dl2 >= 1.0e8 && dcull >= 1.0e8);

        let f1 = CrowdLodParams {
            force_lod: Some(1),
            ..CrowdLodParams::default()
        };
        let (f1_l1, f1_l2, _) = f1.resolve_dists();
        assert!(f1_l1 > 0.0 && f1_l1 < 1e-3 && f1_l2 >= 1.0e8);

        let a = CrowdLodParams {
            lod_bias: f32::NAN,
            ..CrowdLodParams::default()
        };
        let b = CrowdLodParams {
            lod_bias: 1.0,
            ..CrowdLodParams::default()
        };
        assert_eq!(a.signature_key(), b.signature_key());
    }

    #[test]
    fn merge_pending_cpu_intents_preserves_unuploaded_poke_and_freeze() {
        let mut gpu = [
            csg_actors::IntentPod {
                move_cmd: [1.0, 0.5, 0.25, 1.2],
                impulse_stance: [0.0, 0.0, 0.0, 1.0],
                flags: intent_flags::MASK_VEL | intent_flags::FREEZE_POSE,
                ..csg_actors::IntentPod::default()
            },
            csg_actors::IntentPod {
                move_cmd: [0.0, 1.0, 0.0, 1.1],
                impulse_stance: [0.0, 0.0, 0.0, 0.9],
                flags: intent_flags::MASK_VEL,
                ..csg_actors::IntentPod::default()
            },
        ];
        let cpu = [
            // Actor 0: CPU cleared FREEZE_POSE and queued a poke impulse.
            csg_actors::IntentPod {
                impulse_stance: [2.4, 0.36, -0.8, 1.0],
                flags: intent_flags::MASK_IMPULSE,
                ..csg_actors::IntentPod::default()
            },
            // Actor 1: CPU set FREEZE_POSE without poke.
            csg_actors::IntentPod {
                flags: intent_flags::FREEZE_POSE,
                ..csg_actors::IntentPod::default()
            },
        ];

        merge_pending_cpu_intents(&mut gpu, &cpu);

        // GPU steering fields (`move_cmd`, `stance_scale`) stay intact while pending CPU poke
        // and freeze state win.
        assert_eq!(gpu[0].move_cmd, [1.0, 0.5, 0.25, 1.2]);
        assert_eq!(gpu[0].impulse_stance, [2.4, 0.36, -0.8, 1.0]);
        assert_ne!(gpu[0].flags & intent_flags::MASK_IMPULSE, 0);
        assert_eq!(gpu[0].flags & intent_flags::FREEZE_POSE, 0);
        assert_eq!(gpu[1].impulse_stance[3], 0.9);
        assert_ne!(gpu[1].flags & intent_flags::FREEZE_POSE, 0);
        assert_eq!(gpu[1].flags & intent_flags::MASK_IMPULSE, 0);

        // `mark_impulse_consumed` keeps impulse when dt == 0 (paused LOD refresh) and clears when dt > 0.
        let mut paused = gpu[0];
        csg_actors::policy::mark_impulse_consumed(&mut paused, 0.0);
        assert_ne!(paused.flags & intent_flags::MASK_IMPULSE, 0);
        assert_eq!(paused.impulse_stance[0], 2.4);

        csg_actors::policy::mark_impulse_consumed(&mut paused, 1.0 / 60.0);
        assert_eq!(paused.flags & intent_flags::MASK_IMPULSE, 0);
        assert_eq!(paused.impulse_stance[0], 0.0);
    }

    #[test]
    fn build_skeleton_edges_emits_valid_parent_child_pairs_with_selection_keys() {
        let reg = csg_scenes::default_scene_registry();
        let module = reg.get("marina").expect("marina registered");
        let scene = module
            .build(&csg_scene::SceneBuildConfig::default())
            .expect("build marina scene");
        assert!(scene.crowd.actor_count() > 0 && scene.crowd.bone_count() > 0);

        let mut edges = Vec::new();
        build_skeleton_edges(&scene, &mut edges);
        let expected_edges: usize = scene
            .crowd
            .descs
            .iter()
            .filter_map(|d| scene.crowd.archetype_skeleton(d.archetype_id as usize))
            .map(|skel| skel.joints.iter().filter(|j| j.parent.is_some()).count())
            .sum();
        assert_eq!(
            edges.len(),
            expected_edges,
            "every non-root crowd joint must produce one skeleton edge"
        );
        let n_joints = scene.crowd.world_joints.len() as u32;
        for &[p_gidx, c_gidx, _obj_key, pad] in &edges {
            assert!(p_gidx < c_gidx, "parent joint must precede child joint");
            assert!(c_gidx < n_joints, "child joint index out of bounds");
            assert_eq!(pad, 0);
        }
        // Re-running into the same buffer must clear and reproduce identical edges.
        let snapshot = edges.clone();
        build_skeleton_edges(&scene, &mut edges);
        assert_eq!(edges, snapshot);
    }

    #[test]
    #[ignore = "requires local NVIDIA GPU + CUDA driver + NVRTC"]
    fn cuda_crowd_frame_plan_v2_steps_and_applies_poses() {
        let ctx = CudaContext::new(0).expect("CUDA context on device 0");
        ctx.bind_to_thread().expect("bind CUDA context");
        let stream = ctx.default_stream();
        let mut backend = CudaCrowdBackend::new(&ctx).expect("compile PlanKind::Crowd v2");

        let reg = csg_scenes::default_scene_registry();
        let module = reg.get("marina").expect("marina registered");
        let mut scene = module
            .build(&csg_scene::SceneBuildConfig::default())
            .expect("build marina scene");
        assert!(scene.crowd.actor_count() > 0 && scene.crowd.bone_count() > 0);

        let initial_pos: Vec<[f32; 3]> = scene
            .crowd
            .states
            .iter()
            .map(|s| [s.pos_yaw[0], s.pos_yaw[1], s.pos_yaw[2]])
            .collect();
        let lod = CrowdLodParams::default();
        let dt = 1.0 / 60.0;
        let eye = [0.0, 2.0, -5.0];

        for step in 0..20 {
            let t = (step + 1) as f32 * dt;
            backend
                .step_gpu_direct(&stream, &mut scene, dt, t, eye, lod, None, None, false)
                .expect("step_gpu_direct");
        }
        let (bone_ptr, bone_cnt) = backend.bone_outs_device_ptr();
        assert_ne!(bone_ptr, 0, "CrowdBoneOuts must be device-resident");
        assert_eq!(bone_cnt as usize, scene.crowd.bone_count());

        backend
            .sync_cpu_snapshot(&stream, &mut scene, true)
            .expect("sync_cpu_snapshot");
        assert!(
            scene.crowd_fault().is_none(),
            "unexpected crowd fault: {:?}",
            scene.crowd_fault()
        );

        let moved = scene
            .crowd
            .states
            .iter()
            .zip(initial_pos.iter())
            .filter(|(s, p0)| {
                let dx = s.pos_yaw[0] - p0[0];
                let dz = s.pos_yaw[2] - p0[2];
                (dx * dx + dz * dz).sqrt() > 0.05
            })
            .count();
        assert!(moved > 0, "expected patrolling Marina actors to advance on GPU");
    }

    #[test]
    #[ignore = "requires local NVIDIA GPU + CUDA driver + NVRTC + OptiX"]
    fn cuda_crowd_lod2_actors_move_in_vram_after_camera_repack() {
        use crate::csg_scene_host::{
            CsgCameraParams, CsgGpuHost, CsgSceneKey, CsgSyncResult, default_scene_descriptors,
        };
        use csg_scene::{GpuInstance, LodConfig};
        use optix::{DeviceContext, Scratch};

        let ctx = CudaContext::new(0).expect("CUDA context on device 0");
        ctx.bind_to_thread().expect("bind CUDA context");
        let stream = ctx.default_stream();
        let optix_ctx = DeviceContext::new(&ctx, None, 0, false).expect("OptiX context");
        let mut scratch = Scratch::new(&stream);
        let mut backend = CudaCrowdBackend::new(&ctx).expect("compile PlanKind::Crowd v2");
        let mut host = CsgGpuHost::new();

        let marina_idx = default_scene_descriptors()
            .iter()
            .position(|d| d.id == "marina")
            .expect("marina registered");
        let key = CsgSceneKey {
            scene_idx: (marina_idx + 1) as u32,
            variant_idx: 0,
            seed: 2026,
            road_mode: 2,
            ground_kerb_mode: 0,
            smart_fill: csg_scene::SmartFill::OFF,
            asset_params_hash: 0,
        };

        let init_cam = CsgCameraParams {
            eye: [0.0, 2.0, -5.0],
            fwd: [0.0, 0.0, 1.0],
            up: [0.0, 1.0, 0.0],
            viewport_w: 1280,
            viewport_h: 720,
            vfov_rad: std::f32::consts::FRAC_PI_3,
            lod_config: LodConfig::default(),
            omnidirectional: false,
        };
        let sync0 = host
            .sync(
                &optix_ctx,
                &stream,
                &mut scratch,
                key,
                0.0,
                0,
                init_cam,
                true,
            )
            .expect("initial Marina sync");
        assert_eq!(sync0, CsgSyncResult::FullRepack);

        let vp = host
            .take_loaded_metadata()
            .and_then(|m| m.viewpoints.into_iter().next())
            .expect("marina vp0");
        let fwd = (vp.target - vp.eye).norm();
        let vp0_cam = CsgCameraParams {
            eye: [vp.eye.x, vp.eye.y, vp.eye.z],
            fwd: [fwd.x, fwd.y, fwd.z],
            up: [0.0, 1.0, 0.0],
            viewport_w: 1280,
            viewport_h: 720,
            vfov_rad: vp.vfov_deg.to_radians(),
            lod_config: LodConfig::default(),
            omnidirectional: false,
        };
        let lod = CrowdLodParams::default();
        let dt = 1.0 / 60.0;

        {
            let scene = host.active_scene_mut().expect("marina scene");
            backend
                .step_gpu_direct(&stream, scene, dt, dt, vp0_cam.eye, lod, None, None, false)
                .expect("warmup GPU step at vp0");
            stream.synchronize().expect("warmup fence");
            backend
                .sync_cpu_snapshot(&stream, scene, true)
                .expect("sync_cpu_snapshot before repack");
        }

        let sync1 = host
            .sync(
                &optix_ctx,
                &stream,
                &mut scratch,
                key,
                0.0,
                0,
                vp0_cam,
                true,
            )
            .expect("vp0 FullRepack");
        assert_eq!(sync1, CsgSyncResult::FullRepack);

        let mut slot_map = Vec::new();
        let mut gas_handles = Vec::new();
        host.fill_crowd_bone_slot_map(&mut slot_map);
        host.fill_compound_gas_handles(&mut gas_handles);
        assert_eq!(
            slot_map.iter().filter(|&&s| s == u32::MAX).count(),
            0,
            "all crowd bones (including LOD2 pawn roots) must map after camera repack"
        );
        backend
            .upload_scatter_topology(&stream, &slot_map, &gas_handles, host.compound_sbt_offsets())
            .expect("upload_scatter_topology");

        let ptrs = host.device_pointers();
        let inst_count = ptrs.instance_count as usize;
        assert!(ptrs.instances != 0 && inst_count > 0);
        let scatter = GpuScatterTarget {
            csg_instances_ptr: ptrs.instances,
            csg_prev_xforms_ptr: ptrs.prev_xforms,
            max_csg_instances: ptrs.instance_count,
            optix_instances_ptr: 0,
            optix_slot_offset: 0,
            max_optix_instances: 0,
            hidden_semantics: 0,
            hide_dynamic_actors: false,
        };

        let download_instances = |stream: &Stream| -> Vec<GpuInstance> {
            stream.synchronize().expect("fence before D2H");
            let mut buf = vec![GpuInstance::default(); inst_count];
            let bytes = inst_count * std::mem::size_of::<GpuInstance>();
            // SAFETY: `ptrs.instances` holds `inst_count` `GpuInstance` pods on `ctx` and
            // `stream.synchronize()` completed all prior scatter writes.
            let rc = unsafe { cu::cuMemcpyDtoH_v2(buf.as_mut_ptr().cast(), ptrs.instances, bytes) };
            assert_eq!(rc, cu::cudaError_enum_CUDA_SUCCESS, "cuMemcpyDtoH_v2 failed");
            buf
        };

        {
            let scene = host.active_scene_mut().expect("marina scene");
            backend
                .step_gpu_direct(
                    &stream,
                    scene,
                    dt,
                    2.0 * dt,
                    vp0_cam.eye,
                    lod,
                    None,
                    Some(scatter),
                    false,
                )
                .expect("baseline GPU scatter step");
        }
        let before = download_instances(&stream);
        for step in 0..30 {
            let t = (step + 3) as f32 * dt;
            let scene = host.active_scene_mut().expect("marina scene");
            backend
                .step_gpu_direct(&stream, scene, dt, t, vp0_cam.eye, lod, None, Some(scatter), false)
                .expect("GPU scatter step");
        }
        let after = download_instances(&stream);

        let crowd = host.active_crowd().expect("marina crowd");
        let pawn_cid = crowd
            .lod2_pawn_compound
            .expect("marina must define lod2_pawn_compound");
        let mut lod2_moved = 0usize;
        for desc in &crowd.descs {
            let root_gidx = desc.bone_offset as usize;
            let slot = slot_map[root_gidx] as usize;
            let b0 = &before[slot];
            let b1 = &after[slot];
            let vis = (b0.flags & BONE_OUT_VISIBLE) != 0 && (b1.flags & BONE_OUT_VISIBLE) != 0;
            if vis && b0.compound_id == pawn_cid {
                let dx = b1.xform[3] - b0.xform[3];
                let dy = b1.xform[7] - b0.xform[7];
                let dz = b1.xform[11] - b0.xform[11];
                if (dx * dx + dy * dy + dz * dz).sqrt() > 0.05 {
                    lod2_moved += 1;
                }
            }
        }
        assert!(
            lod2_moved > 0,
            "expected Marina LOD2 pawn actors to move in VRAM after camera repack"
        );
    }
}
