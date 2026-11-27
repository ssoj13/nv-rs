//! CUDA/NVRTC GPU-direct lane-graph vehicle traffic simulator driven by `csg-rs` [`FramePlan`].
//!
//! What: executes `csg_scene::traffic_gpu::traffic_frame_plan(csg_device::CSG_EVAL_CUH)`
//! (`PlanKind::Traffic` v1: IDM car-following, swept junction reservation arbitration, deadlock
//! relocation, VRAM-direct pedestrian AEB/yield over bound `CrowdStates`, per-wheel road surface
//! suspension, and Ackermann steering) through [`crate::frame_plan_runner::GpuFramePlan`], paired
//! with a compact host-bridge module (`TRAFFIC_BRIDGE_CUH`) that scatters multi-part vehicle body
//! and wheel transforms directly into `nv-demo`'s CSG/OptiX IAS buffers and tracks amortized IAS
//! rebuild displacement + fleet speed telemetry with zero mid-frame GPU stalls.
//!
//! Why: keeps vehicle simulation, pedestrian AEB/yield, crosswalk vehicle-yielding, possessed
//! camera composition, and ADAS overlays 100% VRAM-direct on `rig.stream` (`Traffic -> Crowd ->
//! Camera -> Overlay`). Only two tiny asynchronous page-locked DMA transfers come back per frame
//! (32 B `TrafficGlobalsPod` + 16 B bridge stats, plus 32 B `VehicleHazardStatePod` while
//! possessing a vehicle). Explicit `TrafficBackend::Cuda` stays strict and never silently
//! downgrades on failure.
//!
//! How: `ensure_scene_uploaded` flattens `scene.traffic.gpu_scene()`, bumps
//! [`TrafficTableGenerations`], uploads `BufClass::Table` buffers once per scene, seeds
//! `TrafficStates`, and zeroes `TrafficGlobals` / `TrafficHazards`. Each frame runs all catch-up
//! substeps (`EventId::TrafficSubstep`) + the final `plan.frame` (or `EventId::TrafficPoseOnly`
//! when `dt == 0`), binds live `CrowdStates` directly in VRAM, launches
//! `scatter_traffic_to_csg_and_optix` and `check_traffic_ias_and_stats`, and enqueues async
//! pinned D2H copies read on the following frame after the single end-of-frame fence.
//!
//! Uses: `crate::frame_plan_runner`, `crate::cuda_crowd::GrowCudaBuf`, `csg_scene::traffic_gpu`,
//! `csg_scene::frame_plan`, `csg_device::CSG_EVAL_CUH`, `cuda_bindings`, `cuda_core`.
//! Used by: `crate::renderer`, `crate::csg_scene_host`.

use std::ffi::c_void;
use std::sync::Arc;
use std::time::Instant;

use bytemuck::{Pod, Zeroable};
use csg_core::Mat3x4;
use csg_scene::InstanceId;
use csg_scene::frame_plan::{BufId, EventId, FrameSizes, PlanKind, ReadbackWhen};
use csg_scene::traffic::{TrafficPoseSink, VehicleHazardStatePod};
use csg_scene::traffic_gpu::{
    TRAFFIC_FRAME_PLAN_VERSION, TRAFFIC_SLOTS_PER_VEHICLE, TrafficBlockerPod, TrafficFrameData,
    TrafficGlobalsPod, TrafficGpuScene, TrafficInstanceOutPod, TrafficTableGenerations,
    encode_blockers, out_flags, traffic_frame_plan,
};
use csg_scene::SceneBuilt;
use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, Stream};

use crate::cuda_crowd::{CUDA_IAS_REBUILD_DISP_M, GrowCudaBuf};
use crate::frame_plan_runner::{GpuFramePlan, PinnedHostBuf, compile_nvrtc_ptx, load_cuda_fn};

/// Flag bit of [`TrafficScatterBindingPod::flags`]: `local_xform` is non-identity and must be
/// post-multiplied (`part_world = out.xform * local_xform`).
pub const TRAFFIC_BIND_NON_IDENTITY: u32 = 1 << 0;

/// One packed CSG/OptiX instance slot bound to a traffic output slot (`64 B`, `alignas(16)`).
///
/// Multi-part vehicle assemblies (chassis, roof, glass, headlights, LOD proxies) each carry their
/// own `local_xform` relative to the dynamic instance root (`body_id` or `wheel_ids[w]`). Storing
/// one binding per packed slot lets `scatter_traffic_to_csg_and_optix` update every part and wheel
/// in a single flat kernel pass without assuming 1-to-1 instance counts.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct TrafficScatterBindingPod {
    /// Index into `BufId::TrafficInstanceOuts` (`vehicle * 7 + slot`, `0` = body, `1..=6` = wheels).
    pub traffic_out_idx: u32,
    /// Destination slot index inside `csg_instances` / `csg_prev_xforms` / `d_optix_instances`
    /// (plus `optix_slot_offset`).
    pub packed_slot: u32,
    /// Bit 0 = [`TRAFFIC_BIND_NON_IDENTITY`].
    pub flags: u32,
    /// Padding to 16 B before `local_xform`.
    pub _pad: u32,
    /// Part/proxy local transform relative to its dynamic instance root.
    pub local_xform: [f32; 12],
}

const _: () = {
    assert!(std::mem::size_of::<TrafficScatterBindingPod>() == 64);
    assert!(std::mem::align_of::<TrafficScatterBindingPod>() <= 16);
    assert!(std::mem::size_of::<TrafficGlobalsPod>() == 32);
    assert!(std::mem::size_of::<VehicleHazardStatePod>() == 32);
    assert!(std::mem::size_of::<TrafficInstanceOutPod>() == 64);
};

/// 4-word device/pinned bridge stats array layout:
/// `[0] = disp_rebuild_flag`, `[1] = moving_count`, `[2] = sum_speed_mm_s`, `[3] = max_speed_mm_s`.
const BRIDGE_STAT_WORDS: usize = 4;
const STAT_DISP_FLAG_IDX: usize = 0;
const STAT_MOVING_COUNT_IDX: usize = 1;
const STAT_SUM_SPEED_MM_IDX: usize = 2;
const STAT_MAX_SPEED_MM_IDX: usize = 3;

/// Compact nv-demo host bridge CUDA source:
/// 1. `scatter_traffic_to_csg_and_optix` updates packed CSG instances, `csg_prev_xforms` (honoring
///    `SNAP_HISTORY` on vehicle relocation), and OptiX IAS instance transforms directly in VRAM.
/// 2. `check_traffic_ias_and_stats` measures cumulative body displacement for amortized full IAS
///    rebuilds (`CUDA_IAS_REBUILD_DISP_M`) AND reduces fleet speed metrics (`moving_count`,
///    `sum_speed_mm_s`, `max_speed_mm_s`) into 4 `u32` words in the same single pass.
const TRAFFIC_BRIDGE_CUH: &str = r#"
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;

struct alignas(16) BridgeTrafficOutPod {
    float xform[12];
    uint32_t instance_id;
    uint32_t flags;
    uint32_t vehicle;
    uint32_t slot;
};

struct alignas(16) BridgeTrafficHazardPod {
    uint32_t state;
    uint32_t flags;
    uint32_t hazard_actor_idx;
    float speed_mps;
    float accel_mps2;
    float stop_dist_m;
    float hazard_dist_m;
    float limit_mps;
};

struct alignas(16) BridgeTrafficBindingPod {
    uint32_t traffic_out_idx;
    uint32_t packed_slot;
    uint32_t flags;
    uint32_t _pad;
    float local_xform[12];
};

struct alignas(16) BridgeCsgInstancePod {
    float xform[12];
    uint32_t compound_id;
    uint32_t user_id;
    uint32_t mat_override;
    uint32_t flags;
};

struct alignas(16) BridgeOptixInstancePod {
    float transform[12];
    uint32_t instanceId : 24;
    uint32_t sbtOffset : 24;
    uint32_t visibilityMask : 8;
    uint32_t flags : 8;
    uint64_t traversableHandle;
    uint32_t pad[2];
};

static_assert(sizeof(BridgeTrafficOutPod) == 64, "TrafficInstanceOutPod ABI mismatch");
static_assert(sizeof(BridgeTrafficHazardPod) == 32, "VehicleHazardStatePod ABI mismatch");
static_assert(sizeof(BridgeTrafficBindingPod) == 64, "TrafficScatterBindingPod ABI mismatch");
static_assert(sizeof(BridgeCsgInstancePod) == 64, "CsgInstance ABI mismatch");
static_assert(sizeof(BridgeOptixInstancePod) == 80, "OptixInstance ABI mismatch");

__device__ __forceinline__ void mul_3x4(const float a[12], const float b[12], float out[12]) {
    out[0]  = a[0] * b[0] + a[1] * b[4] + a[2] * b[8];
    out[1]  = a[0] * b[1] + a[1] * b[5] + a[2] * b[9];
    out[2]  = a[0] * b[2] + a[1] * b[6] + a[2] * b[10];
    out[3]  = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3];

    out[4]  = a[4] * b[0] + a[5] * b[4] + a[6] * b[8];
    out[5]  = a[4] * b[1] + a[5] * b[5] + a[6] * b[9];
    out[6]  = a[4] * b[2] + a[5] * b[6] + a[6] * b[10];
    out[7]  = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7];

    out[8]  = a[8] * b[0] + a[9] * b[4] + a[10] * b[8];
    out[9]  = a[8] * b[1] + a[9] * b[5] + a[10] * b[9];
    out[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10];
    out[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11];
}

extern "C" __global__ void scatter_traffic_to_csg_and_optix(
    uint32_t binding_count,
    uint32_t out_count,
    const BridgeTrafficOutPod* __restrict__ outs,
    const BridgeTrafficBindingPod* __restrict__ bindings,
    uint32_t max_csg_instances,
    BridgeCsgInstancePod* __restrict__ dst_csg_instances,
    float* __restrict__ dst_csg_prev_xforms,
    BridgeOptixInstancePod* __restrict__ dst_optix_instances,
    uint32_t optix_slot_offset,
    uint32_t max_optix_instances
) {
    const uint32_t i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= binding_count || outs == nullptr || bindings == nullptr) return;

    const BridgeTrafficBindingPod bind = bindings[i];
    if (bind.traffic_out_idx >= out_count) return;
    const BridgeTrafficOutPod out = outs[bind.traffic_out_idx];
    if ((out.flags & 1u) == 0u) return; // WRITTEN bit not set

    const uint32_t slot = bind.packed_slot;
    if (slot >= max_csg_instances) return;

    float world_xform[12];
    if ((bind.flags & 1u) != 0u) {
        mul_3x4(out.xform, bind.local_xform, world_xform);
    } else {
        for (int k = 0; k < 12; ++k) {
            world_xform[k] = out.xform[k];
        }
    }

    const bool snap_history = (out.flags & 2u) != 0u;
    if (dst_csg_instances != nullptr) {
        if (dst_csg_prev_xforms != nullptr) {
            float* dp = dst_csg_prev_xforms + (size_t)slot * 12u;
            if (snap_history) {
                for (int k = 0; k < 12; ++k) {
                    dp[k] = world_xform[k];
                }
            } else {
                const float* old_x = dst_csg_instances[slot].xform;
                for (int k = 0; k < 12; ++k) {
                    dp[k] = old_x[k];
                }
            }
        }
        float* dx = dst_csg_instances[slot].xform;
        for (int k = 0; k < 12; ++k) {
            dx[k] = world_xform[k];
        }
    }

    if (dst_optix_instances != nullptr) {
        const uint32_t oi_idx = optix_slot_offset + slot;
        if (oi_idx < max_optix_instances) {
            float* ox = dst_optix_instances[oi_idx].transform;
            for (int k = 0; k < 12; ++k) {
                ox[k] = world_xform[k];
            }
        }
    }
}

extern "C" __global__ void check_traffic_ias_and_stats(
    uint32_t vehicle_count,
    uint32_t slots_per_vehicle,
    const BridgeTrafficOutPod* __restrict__ outs,
    const BridgeTrafficHazardPod* __restrict__ hazards,
    float4* __restrict__ last_rebuild_pos,
    float max_disp_sq,
    uint32_t reset_anchors,
    uint32_t* __restrict__ stats_out
) {
    const uint32_t v = blockIdx.x * blockDim.x + threadIdx.x;
    if (v >= vehicle_count || outs == nullptr || stats_out == nullptr) return;

    const BridgeTrafficOutPod body = outs[(size_t)v * (size_t)slots_per_vehicle];
    if ((body.flags & 1u) != 0u && last_rebuild_pos != nullptr) {
        const float px = body.xform[3];
        const float py = body.xform[7];
        const float pz = body.xform[11];
        if (reset_anchors != 0u) {
            last_rebuild_pos[v] = make_float4(px, py, pz, 0.0f);
        } else {
            const float4 ref = last_rebuild_pos[v];
            const float dx = px - ref.x;
            const float dy = py - ref.y;
            const float dz = pz - ref.z;
            if (dx * dx + dy * dy + dz * dz > max_disp_sq) {
                atomicOr(&stats_out[0], 1u);
            }
        }
    }

    if (hazards != nullptr) {
        const float spd = hazards[v].speed_mps;
        if (isfinite(spd) && spd >= 0.0f) {
            if (spd > 0.25f) {
                atomicAdd(&stats_out[1], 1u);
            }
            const float clamped = fminf(spd, 200.0f);
            const uint32_t mm_s = (uint32_t)(clamped * 1000.0f + 0.5f);
            atomicAdd(&stats_out[2], mm_s);
            atomicMax(&stats_out[3], mm_s);
        }
    }
}
"#;

/// Destination device pointers for GPU-direct traffic scatter into CSG + OptiX IAS buffers.
#[derive(Clone, Copy, Debug, Default)]
pub struct TrafficScatterTarget {
    /// Device pointer to packed `CsgGpuInstance` array (`csg_host.device_pointers().instances`).
    pub csg_instances_ptr: u64,
    /// Device pointer to packed `prev_xforms` array (`csg_host.device_pointers().prev_xforms`).
    pub csg_prev_xforms_ptr: u64,
    /// Element count of `csg_instances_ptr`.
    pub max_csg_instances: u32,
    /// Device pointer to OptiX IAS `InstanceBuffer` (`0` skips OptiX instance writes).
    pub optix_instances_ptr: u64,
    /// Offset of the first CSG instance inside `optix_instances_ptr` (`ias_classic_count`).
    pub optix_slot_offset: u32,
    /// Total capacity of `optix_instances_ptr`.
    pub max_optix_instances: u32,
}

/// Summary returned by [`CudaTrafficBackend::step_gpu_direct`] for IAS refit and telemetry.
#[derive(Clone, Copy, Debug, Default)]
pub struct CudaTrafficStepReport {
    /// Host enqueue wall time in milliseconds.
    pub step_ms: f32,
    /// `true` when any vehicle displaced beyond [`CUDA_IAS_REBUILD_DISP_M`] since the last full IAS
    /// rebuild (amortized from the previous frame's async pinned DMA).
    pub needs_full_ias_rebuild: bool,
    /// `true` when traffic scattered updated transforms into `d_optix_instances`.
    pub scattered_optix: bool,
}

/// Decoded zero-stall fleet speed snapshot from the 16 B pinned bridge stats DMA.
#[derive(Clone, Copy, Debug, Default)]
pub struct TrafficFleetStats {
    /// Moving vehicle count (`speed_mps > 0.25`).
    pub moving: u32,
    /// Mean vehicle speed in `m/s`.
    pub avg_speed_mps: f32,
    /// Peak vehicle speed in `m/s`.
    pub max_speed_mps: f32,
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

/// Persistent CUDA/NVRTC lane-graph vehicle traffic backend executing `csg-rs`'s
/// `PlanKind::Traffic` v1 [`GpuFramePlan`] (`StrictF32` IEEE-754 precision) and a compact
/// multi-part CSG/OptiX scatter bridge.
pub struct CudaTrafficBackend {
    cuda: Arc<CudaContext>,
    plan: GpuFramePlan,
    bridge_module: cu::CUmodule,
    scatter_fn: cu::CUfunction,
    stats_fn: cu::CUfunction,
    gpu_scene: Option<TrafficGpuScene>,
    generations: TrafficTableGenerations,
    // Host-bridge buffers.
    d_bindings: GrowCudaBuf<TrafficScatterBindingPod>,
    d_rebuild_pos: GrowCudaBuf<[f32; 4]>,
    d_bridge_stats: GrowCudaBuf<u32>,
    // Page-locked async readback buffers (zero mid-frame synchronization).
    pinned_globals: PinnedHostBuf<TrafficGlobalsPod>,
    pinned_bridge_stats: PinnedHostBuf<u32>,
    pinned_possess_hazard: PinnedHostBuf<VehicleHazardStatePod>,
    has_pending_readback: bool,
    pending_possess_vehicle: Option<u32>,
    pending_ias_rebuild: bool,
    // Latest zero-stall snapshots drained from the previous frame's pinned DMA.
    last_globals: TrafficGlobalsPod,
    last_fleet_stats: TrafficFleetStats,
    last_possess_hazard: Option<(u32, VehicleHazardStatePod)>,
    // Reusable host scratch buffers (zero steady-state heap allocations).
    host_blockers: Vec<TrafficBlockerPod>,
    host_rebuild_pos: Vec<[f32; 4]>,
    staging_outs: Vec<TrafficInstanceOutPod>,
    scene_dirty: bool,
    topology_dirty: bool,
    cpu_world_stale: bool,
    uploaded_vehicle_count: usize,
    ias_rebuild_count: u32,
}

impl Drop for CudaTrafficBackend {
    fn drop(&mut self) {
        if !self.bridge_module.is_null() && self.cuda.bind_to_thread().is_ok() {
            // SAFETY: `self.bridge_module` was loaded via `cuModuleLoadData` on `self.cuda` and
            // is unloaded exactly once when `CudaTrafficBackend` drops.
            let rc = unsafe { cu::cuModuleUnload(self.bridge_module) };
            if rc != cu::cudaError_enum_CUDA_SUCCESS {
                eprintln!("nv-demo: CudaTrafficBackend bridge cuModuleUnload failed ({rc:?})");
            }
            self.bridge_module = std::ptr::null_mut();
        }
    }
}

impl CudaTrafficBackend {
    /// Compiles and validates `csg_scene::traffic_gpu::traffic_frame_plan(csg_device::CSG_EVAL_CUH)`
    /// (version 1, `StrictF32` IEEE-754 math) plus `TRAFFIC_BRIDGE_CUH` on `cuda`.
    pub fn new(cuda: &Arc<CudaContext>) -> Result<Self, String> {
        cuda.bind_to_thread()
            .map_err(|e| format!("CUDA bind failed for CudaTrafficBackend: {e}"))?;

        let plan = GpuFramePlan::new_with_math(
            cuda,
            traffic_frame_plan(csg_device::CSG_EVAL_CUH),
            TRAFFIC_FRAME_PLAN_VERSION,
            "traffic_frame_v1.cu",
            nvrtc::MathMode::StrictF32,
        )?;

        let has_globals_readback = plan
            .plan()
            .readback_ids(ReadbackWhen::Always)
            .any(|id| id == BufId::TrafficGlobals);
        if !has_globals_readback {
            return Err(
                "PlanKind::Traffic missing BufId::TrafficGlobals in ReadbackWhen::Always"
                    .to_string(),
            );
        }

        let bridge_ptx = compile_nvrtc_ptx(TRAFFIC_BRIDGE_CUH, "traffic_host_bridge.cu")?;
        let mut bridge_module: cu::CUmodule = std::ptr::null_mut();
        // SAFETY: `bridge_ptx` is a NUL-terminated PTX buffer and `cuda` is current on this thread.
        let rc = unsafe { cu::cuModuleLoadData(&mut bridge_module, bridge_ptx.as_ptr().cast()) };
        if rc != cu::cudaError_enum_CUDA_SUCCESS || bridge_module.is_null() {
            return Err(format!(
                "cuModuleLoadData(traffic_host_bridge.cu) failed ({rc:?})"
            ));
        }

        struct BridgeInit {
            scatter_fn: cu::CUfunction,
            stats_fn: cu::CUfunction,
            pinned_globals: PinnedHostBuf<TrafficGlobalsPod>,
            pinned_bridge_stats: PinnedHostBuf<u32>,
            pinned_possess_hazard: PinnedHostBuf<VehicleHazardStatePod>,
        }
        let init_bridge = || -> Result<BridgeInit, String> {
            let scatter_fn = load_cuda_fn(bridge_module, "scatter_traffic_to_csg_and_optix")?;
            let stats_fn = load_cuda_fn(bridge_module, "check_traffic_ias_and_stats")?;
            let mut pinned_globals = PinnedHostBuf::<TrafficGlobalsPod>::new(cuda, 1)?;
            pinned_globals.write(0, TrafficGlobalsPod::default())?;
            let mut pinned_bridge_stats = PinnedHostBuf::<u32>::new(cuda, BRIDGE_STAT_WORDS)?;
            for idx in 0..BRIDGE_STAT_WORDS {
                pinned_bridge_stats.write(idx, 0)?;
            }
            let mut pinned_possess_hazard = PinnedHostBuf::<VehicleHazardStatePod>::new(cuda, 1)?;
            pinned_possess_hazard.write(0, VehicleHazardStatePod::default())?;
            Ok(BridgeInit {
                scatter_fn,
                stats_fn,
                pinned_globals,
                pinned_bridge_stats,
                pinned_possess_hazard,
            })
        };

        let BridgeInit {
            scatter_fn,
            stats_fn,
            pinned_globals,
            pinned_bridge_stats,
            pinned_possess_hazard,
        } = match init_bridge() {
            Ok(v) => v,
            Err(e) => {
                // SAFETY: `bridge_module` was just loaded on `cuda` and is being cleaned up.
                let _ = unsafe { cu::cuModuleUnload(bridge_module) };
                return Err(e);
            }
        };

        Ok(Self {
            cuda: Arc::clone(cuda),
            plan,
            bridge_module,
            scatter_fn,
            stats_fn,
            gpu_scene: None,
            generations: TrafficTableGenerations::default(),
            d_bindings: GrowCudaBuf::default(),
            d_rebuild_pos: GrowCudaBuf::default(),
            d_bridge_stats: GrowCudaBuf::default(),
            pinned_globals,
            pinned_bridge_stats,
            pinned_possess_hazard,
            has_pending_readback: false,
            pending_possess_vehicle: None,
            pending_ias_rebuild: false,
            last_globals: TrafficGlobalsPod::default(),
            last_fleet_stats: TrafficFleetStats::default(),
            last_possess_hazard: None,
            host_blockers: Vec::with_capacity(64),
            host_rebuild_pos: Vec::with_capacity(128),
            staging_outs: Vec::new(),
            scene_dirty: true,
            topology_dirty: true,
            cpu_world_stale: false,
            uploaded_vehicle_count: 0,
            ias_rebuild_count: 0,
        })
    }

    /// Marks the lane graph and vehicle tables dirty (call on scene rebuild or CPU -> CUDA switch).
    #[inline]
    pub fn mark_scene_dirty(&mut self) {
        self.scene_dirty = true;
        self.topology_dirty = true;
        self.cpu_world_stale = false;
        self.has_pending_readback = false;
        self.pending_possess_vehicle = None;
        self.pending_ias_rebuild = false;
        self.last_globals = TrafficGlobalsPod::default();
        self.last_fleet_stats = TrafficFleetStats::default();
        self.last_possess_hazard = None;
    }

    /// Marks scatter bindings stale after a CSG `FullRepack`.
    #[inline]
    pub fn mark_topology_dirty(&mut self) {
        self.topology_dirty = true;
    }

    /// Returns `true` when scatter bindings must be re-uploaded before scattering.
    #[inline]
    pub fn topology_dirty(&self) -> bool {
        self.topology_dirty
    }

    /// Returns `true` when GPU traffic has stepped ahead of `scene.world` dynamic instances.
    #[inline]
    pub fn cpu_world_stale(&self) -> bool {
        self.cpu_world_stale
    }

    /// Returns the cached flattened [`TrafficGpuScene`] once uploaded for the active scene.
    #[inline]
    pub fn cached_gpu_scene(&self) -> Option<&TrafficGpuScene> {
        if self.scene_dirty {
            None
        } else {
            self.gpu_scene.as_ref()
        }
    }

    /// Declarative `PlanKind::Traffic` version executed by this backend (`1`).
    #[inline]
    pub fn plan_version(&self) -> u32 {
        self.plan.plan().version
    }

    /// Total CUDA kernel dispatches per nominal 1-substep traffic frame (`plan.frame` + 2 bridge).
    #[inline]
    pub fn kernel_count(&self) -> u32 {
        (self.plan.plan().frame.len() + 2) as u32
    }

    /// Total allocated VRAM bytes across `PlanKind::Traffic` buffers and host-bridge arrays.
    #[inline]
    pub fn gpu_bytes(&self) -> u64 {
        let bridge = self.d_bindings.cap_bytes()
            + self.d_rebuild_pos.cap_bytes()
            + self.d_bridge_stats.cap_bytes();
        self.plan.gpu_bytes().saturating_add(bridge as u64)
    }

    /// Cumulative amortized full OptiX IAS rebuilds triggered by traffic displacement.
    #[inline]
    pub fn ias_rebuild_count(&self) -> u32 {
        self.ias_rebuild_count
    }

    /// Latest zero-stall 32 B [`TrafficGlobalsPod`] snapshot from the async pinned DMA.
    #[inline]
    pub fn last_globals(&self) -> TrafficGlobalsPod {
        self.last_globals
    }

    /// Latest zero-stall fleet speed snapshot from the 16 B async pinned bridge stats DMA.
    #[inline]
    pub fn last_fleet_stats(&self) -> TrafficFleetStats {
        self.last_fleet_stats
    }

    /// Latest zero-stall 32 B [`VehicleHazardStatePod`] for the possessed vehicle (if matching).
    #[inline]
    pub fn last_possess_hazard(&self, vehicle_idx: u32) -> Option<VehicleHazardStatePod> {
        let (idx, pod) = self.last_possess_hazard?;
        (idx == vehicle_idx).then_some(pod)
    }

    /// Returns `(cu_deviceptr, vehicle_count)` of `BufId::TrafficInstanceOuts` (`64 B * 7 * N`)
    /// for zero-copy binding into `PlanKind::Camera` (`GpuCameraComposer`) and `ovl_adas`.
    #[inline]
    pub fn instance_outs_device_ptr(&self) -> (u64, u32) {
        if self.scene_dirty || self.uploaded_vehicle_count == 0 {
            return (0, 0);
        }
        (
            self.plan.device_ptr(BufId::TrafficInstanceOuts).unwrap_or(0),
            u32::try_from(self.uploaded_vehicle_count).unwrap_or(0),
        )
    }

    /// Returns `(cu_deviceptr, vehicle_count)` of `BufId::TrafficCrowdVehicles` (`32 B * N`,
    /// ABI-identical to `CrowdVehiclePod`) for zero-copy D2D into `PlanKind::Crowd`.
    #[inline]
    pub fn crowd_vehicles_device_ptr(&self) -> (u64, u32) {
        if self.scene_dirty || self.uploaded_vehicle_count == 0 {
            return (0, 0);
        }
        (
            self.plan
                .device_ptr(BufId::TrafficCrowdVehicles)
                .unwrap_or(0),
            u32::try_from(self.uploaded_vehicle_count).unwrap_or(0),
        )
    }

    /// Returns `(cu_deviceptr, vehicle_count)` of `BufId::TrafficHazards` (`32 B * N`,
    /// `VehicleHazardStatePod`) for zero-copy GPU `ovl_adas` overlay rendering.
    #[inline]
    pub fn hazards_device_ptr(&self) -> (u64, u32) {
        if self.scene_dirty || self.uploaded_vehicle_count == 0 {
            return (0, 0);
        }
        (
            self.plan.device_ptr(BufId::TrafficHazards).unwrap_or(0),
            u32::try_from(self.uploaded_vehicle_count).unwrap_or(0),
        )
    }

    /// Uploads updated multi-part scatter bindings (`TrafficScatterBindingPod`) after a repack.
    pub fn upload_scatter_topology(
        &mut self,
        stream: &Stream,
        bindings: &[TrafficScatterBindingPod],
    ) -> Result<(), String> {
        self.d_bindings.upload(stream, bindings)?;
        self.topology_dirty = false;
        Ok(())
    }

    /// Ensures the active scene's `TrafficGpuScene` is built and its cold tables & initial state
    /// are uploaded to device memory. Returns `true` if a fresh upload occurred.
    pub fn ensure_scene_uploaded(
        &mut self,
        stream: &Stream,
        scene: &SceneBuilt,
    ) -> Result<bool, String> {
        let n_veh = scene.traffic.vehicle_count();
        if n_veh == 0 || !scene.traffic.has_lane_graph() {
            return Ok(false);
        }
        let shape_changed = self.uploaded_vehicle_count != n_veh || self.gpu_scene.is_none();
        if !self.scene_dirty && !shape_changed {
            return Ok(false);
        }

        let gpu_scene = scene
            .traffic
            .gpu_scene()
            .map_err(|e| format!("SceneTraffic::gpu_scene failed: {e}"))?;

        self.generations.scene = self.generations.scene.wrapping_add(1).max(1);
        let frame_data = TrafficFrameData {
            scene: &gpu_scene,
            blockers: 0,
            crowd_actors: 0,
            generations: self.generations,
        };
        self.plan.sync_sizes(stream, &frame_data)?;

        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::TrafficSegs,
            &gpu_scene.segments,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::TrafficStations,
            &gpu_scene.stations,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::TrafficOutgoing,
            &gpu_scene.outgoing,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::TrafficConflicts,
            &gpu_scene.conflicts,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::TrafficSourceLanes,
            &gpu_scene.source_lanes,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::TrafficDescs,
            &gpu_scene.descs,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::TrafficSurface,
            &[gpu_scene.surface],
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::TrafficSurfaceData,
            &gpu_scene.surface_data,
        )?;
        upload_table_if_needed(
            &mut self.plan,
            stream,
            &frame_data,
            BufId::TrafficSurfaceTiles,
            &gpu_scene.surface_tiles,
        )?;

        self.plan
            .upload(stream, BufId::TrafficStates, &gpu_scene.states)?;
        self.plan.zero_buffer(stream, BufId::TrafficGlobals, 1)?;
        self.plan
            .zero_buffer(stream, BufId::TrafficInstanceOuts, gpu_scene.slot_count() as u32)?;
        self.plan
            .zero_buffer(stream, BufId::TrafficCrowdVehicles, gpu_scene.vehicle_count())?;
        self.plan
            .zero_buffer(stream, BufId::TrafficHazards, gpu_scene.vehicle_count())?;

        // Seed amortized IAS rebuild anchor positions from current dynamic body positions.
        self.host_rebuild_pos.clear();
        self.host_rebuild_pos.reserve(n_veh);
        for desc in &gpu_scene.descs {
            let p = scene
                .world
                .dynamic_instance(InstanceId(desc.body_id))
                .map_or([0.0; 3], |inst| [inst.xform.0[3], inst.xform.0[7], inst.xform.0[11]]);
            self.host_rebuild_pos.push([p[0], p[1], p[2], 0.0]);
        }
        self.d_rebuild_pos.upload(stream, &self.host_rebuild_pos)?;
        self.d_bridge_stats.ensure_len(stream, BRIDGE_STAT_WORDS)?;

        self.pinned_globals.write(0, TrafficGlobalsPod::default())?;
        for idx in 0..BRIDGE_STAT_WORDS {
            self.pinned_bridge_stats.write(idx, 0)?;
        }
        self.pinned_possess_hazard
            .write(0, VehicleHazardStatePod::default())?;

        self.gpu_scene = Some(gpu_scene);
        self.uploaded_vehicle_count = n_veh;
        self.scene_dirty = false;
        self.topology_dirty = true;
        self.has_pending_readback = false;
        self.pending_possess_vehicle = None;
        self.pending_ias_rebuild = false;
        self.last_globals = TrafficGlobalsPod::default();
        self.last_fleet_stats = TrafficFleetStats::default();
        self.last_possess_hazard = None;
        Ok(true)
    }

    /// Drains the previous frame's asynchronous pinned DMA transfers (valid because `renderer.rs`
    /// waits on the single end-of-frame fence before launching the next frame).
    fn drain_pending_readbacks(&mut self) -> Result<bool, String> {
        let mut needs_rebuild = std::mem::take(&mut self.pending_ias_rebuild);
        if !self.has_pending_readback {
            return Ok(needs_rebuild);
        }
        self.has_pending_readback = false;

        self.last_globals = self.pinned_globals.read(0)?;
        let disp_flag = self.pinned_bridge_stats.read(STAT_DISP_FLAG_IDX)?;
        let moving = self.pinned_bridge_stats.read(STAT_MOVING_COUNT_IDX)?;
        let sum_mm = self.pinned_bridge_stats.read(STAT_SUM_SPEED_MM_IDX)?;
        let max_mm = self.pinned_bridge_stats.read(STAT_MAX_SPEED_MM_IDX)?;
        let n_veh = self.uploaded_vehicle_count as f32;
        self.last_fleet_stats = TrafficFleetStats {
            moving,
            avg_speed_mps: if n_veh > 0.0 {
                (sum_mm as f32 * 0.001) / n_veh
            } else {
                0.0
            },
            max_speed_mps: max_mm as f32 * 0.001,
        };

        if let Some(vidx) = self.pending_possess_vehicle.take() {
            let hz = self.pinned_possess_hazard.read(0)?;
            self.last_possess_hazard = Some((vidx, hz));
        } else {
            self.last_possess_hazard = None;
        }

        if disp_flag != 0 {
            self.ias_rebuild_count = self.ias_rebuild_count.saturating_add(1);
            needs_rebuild = true;
        }
        Ok(needs_rebuild)
    }

    /// Steps lane-graph traffic directly on GPU via `PlanKind::Traffic` v1, binding live
    /// `crowd_states` (`(ptr, actor_count)`) in VRAM for pedestrian AEB/yield, scattering vehicle
    /// bodies + wheels into CSG/OptiX buffers, and enqueuing zero-stall async pinned telemetry DMA.
    pub fn step_gpu_direct(
        &mut self,
        stream: &Stream,
        scene: &SceneBuilt,
        time_s: f32,
        crowd_states: Option<(u64, u32)>,
        scatter: Option<TrafficScatterTarget>,
        possess_vehicle_idx: Option<u32>,
    ) -> Result<CudaTrafficStepReport, String> {
        if scene.traffic.vehicle_count() == 0 || !scene.traffic.has_lane_graph() {
            return Ok(CudaTrafficStepReport::default());
        }
        let scatter = scatter.filter(|t| t.csg_instances_ptr != 0);
        if scatter.is_some() && self.topology_dirty {
            return Err(
                "CUDA traffic scatter topology is stale (upload_scatter_topology has not succeeded since repack)"
                    .to_string(),
            );
        }

        let t0 = Instant::now();

        // 1. Drain previous frame's async pinned DMA (zero mid-frame sync).
        let needs_full_ias_rebuild = self.drain_pending_readbacks()?;

        // 2. Upload cold scene tables & seed device state if needed.
        self.ensure_scene_uploaded(stream, scene)?;

        // 3. Collect explicit standing blockers and compute substeps matching CPU oracle.
        let raw_blockers = scene.traffic.pedestrian_blockers();
        encode_blockers(&raw_blockers, &mut self.host_blockers);
        let blocker_count = u32::try_from(self.host_blockers.len())
            .map_err(|_| format!("traffic blocker count {} overflows u32", self.host_blockers.len()))?;
        self.plan
            .upload(stream, BufId::TrafficBlockers, &self.host_blockers)?;

        let (crowd_ptr, crowd_count) = match crowd_states {
            Some((ptr, count)) if ptr != 0 && count > 0 => (ptr, count as usize),
            _ => (0u64, 0usize),
        };
        let bounds = [((PlanKind::Crowd, BufId::CrowdStates), crowd_ptr)];

        let substeps = {
            let gpu_scene = self
                .gpu_scene
                .as_mut()
                .ok_or_else(|| "TrafficGpuScene missing after ensure_scene_uploaded".to_string())?;
            gpu_scene.frame_substeps(time_s)
        };

        let Some(substeps) = substeps else {
            return Ok(CudaTrafficStepReport {
                step_ms: t0.elapsed().as_secs_f32() * 1000.0,
                needs_full_ias_rebuild,
                scattered_optix: false,
            });
        };

        // 4. Execute `PlanKind::Traffic` v1: catch-up substeps via `EventId::TrafficSubstep`,
        // final substep + poses via `plan.frame`, or pose-only via `EventId::TrafficPoseOnly`.
        let pose_u = {
            let gpu_scene = self
                .gpu_scene
                .as_ref()
                .ok_or_else(|| "TrafficGpuScene missing before pose_uniforms".to_string())?;
            gpu_scene.pose_uniforms(&substeps)
        };
        if substeps.count == 0 {
            self.plan
                .upload(stream, BufId::TrafficPoseUniforms, &[pose_u])?;
            let gpu_scene = self
                .gpu_scene
                .as_ref()
                .ok_or_else(|| "TrafficGpuScene missing before pose-only execute".to_string())?;
            let frame_data = TrafficFrameData {
                scene: gpu_scene,
                blockers: self.host_blockers.len(),
                crowd_actors: crowd_count,
                generations: self.generations,
            };
            self.plan.execute_event_only(
                stream,
                EventId::TrafficPoseOnly,
                &frame_data,
                &frame_data,
                &[],
                &bounds,
            )?;
        } else {
            for k in 0..substeps.count {
                let step_u = {
                    let gpu_scene = self
                        .gpu_scene
                        .as_mut()
                        .ok_or_else(|| "TrafficGpuScene missing before step_uniforms".to_string())?;
                    gpu_scene.step_uniforms(&substeps, k, blocker_count)
                };
                self.plan
                    .upload(stream, BufId::TrafficStepUniforms, &[step_u])?;
                let is_last = k + 1 == substeps.count;
                if is_last {
                    self.plan
                        .upload(stream, BufId::TrafficPoseUniforms, &[pose_u])?;
                }
                let gpu_scene = self
                    .gpu_scene
                    .as_ref()
                    .ok_or_else(|| "TrafficGpuScene missing before execute".to_string())?;
                let frame_data = TrafficFrameData {
                    scene: gpu_scene,
                    blockers: self.host_blockers.len(),
                    crowd_actors: crowd_count,
                    generations: self.generations,
                };
                if is_last {
                    self.plan
                        .execute(stream, &frame_data, &frame_data, &[], &[], &bounds)?;
                } else {
                    self.plan.execute_event_only(
                        stream,
                        EventId::TrafficSubstep,
                        &frame_data,
                        &frame_data,
                        &[],
                        &bounds,
                    )?;
                }
            }
        }

        // 5. Scatter multi-part vehicle body + wheel poses into CSG + OptiX IAS buffers.
        let mut scattered_optix = false;
        let out_ptr = self
            .plan
            .device_ptr(BufId::TrafficInstanceOuts)
            .ok_or_else(|| "TrafficInstanceOuts device ptr missing".to_string())?;
        let out_count = self.plan.elem_count(BufId::TrafficInstanceOuts);
        if let Some(target) = scatter
            && self.d_bindings.len() > 0
        {
            let mut binding_count = u32::try_from(self.d_bindings.len())
                .map_err(|_| "traffic scatter binding count overflows u32".to_string())?;
            let mut out_count_arg = out_count;
            let mut p_outs = out_ptr;
            let mut p_binds = self.d_bindings.device_ptr();
            let mut max_csg = target.max_csg_instances;
            let mut p_dst_csg = target.csg_instances_ptr;
            let mut p_dst_prev = target.csg_prev_xforms_ptr;
            let mut p_dst_optix = target.optix_instances_ptr;
            let mut optix_off = target.optix_slot_offset;
            let mut max_optix = target.max_optix_instances;
            let mut scat_args: [*mut c_void; 10] = [
                (&raw mut binding_count).cast(),
                (&raw mut out_count_arg).cast(),
                (&raw mut p_outs).cast(),
                (&raw mut p_binds).cast(),
                (&raw mut max_csg).cast(),
                (&raw mut p_dst_csg).cast(),
                (&raw mut p_dst_prev).cast(),
                (&raw mut p_dst_optix).cast(),
                (&raw mut optix_off).cast(),
                (&raw mut max_optix).cast(),
            ];
            let s_block = 256u32;
            let s_grid = binding_count.div_ceil(s_block).max(1);
            // SAFETY: `self.scatter_fn` matches the 10-parameter ABI of
            // `scatter_traffic_to_csg_and_optix`; source/binding indices and destination writes
            // are bounds-checked in the kernel, and all buffers live on `stream` until the fence.
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
                    "cuLaunchKernel(scatter_traffic_to_csg_and_optix) failed ({rc:?})"
                ));
            }
            scattered_optix = target.optix_instances_ptr != 0;
        }

        // 6. Zero 16 B bridge stats, run amortized IAS displacement + fleet speed reduction, and
        // enqueue async pinned D2H readbacks (32 B globals + 16 B stats + optional 32 B hazard).
        {
            let stats_ptr = self.d_bridge_stats.device_ptr();
            // SAFETY: `d_bridge_stats` owns `BRIDGE_STAT_WORDS` `u32` words on `stream`.
            let zrc = unsafe {
                cu::cuMemsetD32Async(stats_ptr, 0, BRIDGE_STAT_WORDS, stream.cu_stream())
            };
            if zrc != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!(
                    "cuMemsetD32Async(d_bridge_stats) failed ({zrc:?})"
                ));
            }

            let mut veh_count = u32::try_from(self.uploaded_vehicle_count)
                .map_err(|_| "traffic vehicle count overflows u32".to_string())?;
            let mut slots_per_veh = TRAFFIC_SLOTS_PER_VEHICLE as u32;
            let mut p_outs = out_ptr;
            let mut p_hazards = self.plan.device_ptr(BufId::TrafficHazards).unwrap_or(0);
            let mut p_rebuild = self.d_rebuild_pos.device_ptr();
            let mut max_disp_sq = CUDA_IAS_REBUILD_DISP_M * CUDA_IAS_REBUILD_DISP_M;
            let mut reset_anchors = u32::from(needs_full_ias_rebuild);
            let mut p_stats = stats_ptr;
            let mut stat_args: [*mut c_void; 8] = [
                (&raw mut veh_count).cast(),
                (&raw mut slots_per_veh).cast(),
                (&raw mut p_outs).cast(),
                (&raw mut p_hazards).cast(),
                (&raw mut p_rebuild).cast(),
                (&raw mut max_disp_sq).cast(),
                (&raw mut reset_anchors).cast(),
                (&raw mut p_stats).cast(),
            ];
            let block_x = 128u32;
            let grid_x = veh_count.div_ceil(block_x).max(1);
            // SAFETY: `self.stats_fn` matches the 8-parameter ABI of
            // `check_traffic_ias_and_stats`; all device arrays are sized for `veh_count` and
            // `d_bridge_stats` holds `BRIDGE_STAT_WORDS` `u32` words.
            let drc = unsafe {
                cu::cuLaunchKernel(
                    self.stats_fn,
                    grid_x,
                    1,
                    1,
                    block_x,
                    1,
                    1,
                    0,
                    stream.cu_stream(),
                    stat_args.as_mut_ptr(),
                    std::ptr::null_mut(),
                )
            };
            if drc != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!(
                    "cuLaunchKernel(check_traffic_ias_and_stats) failed ({drc:?})"
                ));
            }

            // Enqueue `ReadbackWhen::Always` (`TrafficGlobals`, 32 B) into `pinned_globals`.
            for id in self.plan.plan().readback_ids(ReadbackWhen::Always) {
                match id {
                    BufId::TrafficGlobals => {
                        self.plan.copy_to_pinned_buf_async(
                            stream,
                            id,
                            &mut self.pinned_globals,
                            0,
                            1,
                        )?;
                    }
                    other => {
                        return Err(format!(
                            "PlanKind::Traffic has unexpected ReadbackWhen::Always buffer {other:?}"
                        ));
                    }
                }
            }

            // Enqueue 16 B bridge stats into `pinned_bridge_stats`.
            let stats_bytes = BRIDGE_STAT_WORDS * std::mem::size_of::<u32>();
            // SAFETY: `stats_ptr` holds `stats_bytes` bytes on `stream` and `pinned_bridge_stats`
            // holds `BRIDGE_STAT_WORDS` page-locked `u32` words valid until the end-of-frame fence.
            let crc = unsafe {
                cu::cuMemcpyDtoHAsync_v2(
                    self.pinned_bridge_stats.as_mut_ptr().cast(),
                    stats_ptr,
                    stats_bytes,
                    stream.cu_stream(),
                )
            };
            if crc != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!(
                    "traffic bridge stats pinned cuMemcpyDtoHAsync_v2 failed ({crc:?})"
                ));
            }

            // Optional 32 B async pinned DMA for the currently possessed traffic vehicle's ADAS pill.
            self.pending_possess_vehicle = None;
            if let Some(vidx) = possess_vehicle_idx
                && (vidx as usize) < self.uploaded_vehicle_count
            {
                self.plan.copy_elem_to_pinned_buf_async(
                    stream,
                    BufId::TrafficHazards,
                    vidx as usize,
                    &mut self.pinned_possess_hazard,
                    0,
                )?;
                self.pending_possess_vehicle = Some(vidx);
            }
            self.has_pending_readback = true;
        }

        self.cpu_world_stale = true;
        Ok(CudaTrafficStepReport {
            step_ms: t0.elapsed().as_secs_f32() * 1000.0,
            needs_full_ias_rebuild,
            scattered_optix,
        })
    }

    /// On-demand readback of `BufId::TrafficInstanceOuts` into `scene.world` dynamic instances
    /// (called right before `FullRepack`, `Freeze Pose`, `ExportSdgJson`, or CPU pick/possess
    /// fallback so host frustum culling and picking see exact GPU traffic transforms).
    pub fn sync_cpu_world_snapshot(
        &mut self,
        stream: &Stream,
        scene: &mut SceneBuilt,
    ) -> Result<(), String> {
        if !self.cpu_world_stale || self.scene_dirty || self.uploaded_vehicle_count == 0 {
            self.cpu_world_stale = false;
            return Ok(());
        }
        self.plan.download_into_async(
            stream,
            BufId::TrafficInstanceOuts,
            &mut self.staging_outs,
        )?;
        stream
            .synchronize()
            .map_err(|e| format!("CUDA traffic CPU world sync failed: {e}"))?;

        if self.has_pending_readback {
            let disp = self.drain_pending_readbacks()?;
            self.pending_ias_rebuild |= disp;
        }

        for out in &self.staging_outs {
            if (out.flags & out_flags::WRITTEN) == 0 || out.instance_id == u32::MAX {
                continue;
            }
            let id = InstanceId(out.instance_id);
            TrafficPoseSink::update_dynamic_xform(&mut scene.world, id, Mat3x4(out.xform));
            if (out.flags & out_flags::SNAP_HISTORY) != 0 {
                TrafficPoseSink::snap_motion_history(&mut scene.world, id);
            }
        }
        self.cpu_world_stale = false;
        Ok(())
    }
}

/// Computes `inv(parent) * child` for two row-major 3x4 affine matrices in `f64` precision so
/// multi-part vehicle assembly children (`part.local_xform` or `proxy.box_local_xform`) are
/// recovered exactly from `WorldScene` during topology binding.
///
/// Returns `(local_xform, is_non_identity)`. Falls back to identity if `parent` is singular.
pub(crate) fn compute_local_xform_from_world(
    parent: [f32; 12],
    child: [f32; 12],
) -> ([f32; 12], bool) {
    let a00 = f64::from(parent[0]);
    let a01 = f64::from(parent[1]);
    let a02 = f64::from(parent[2]);
    let a03 = f64::from(parent[3]);
    let a10 = f64::from(parent[4]);
    let a11 = f64::from(parent[5]);
    let a12 = f64::from(parent[6]);
    let a13 = f64::from(parent[7]);
    let a20 = f64::from(parent[8]);
    let a21 = f64::from(parent[9]);
    let a22 = f64::from(parent[10]);
    let a23 = f64::from(parent[11]);

    let c00 = a11 * a22 - a12 * a21;
    let c01 = a12 * a20 - a10 * a22;
    let c02 = a10 * a21 - a11 * a20;
    let det = a00 * c00 + a01 * c01 + a02 * c02;
    if !det.is_finite() || det.abs() < 1e-12 {
        return ([1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0], false);
    }
    let inv_det = 1.0 / det;
    let i00 = c00 * inv_det;
    let i01 = (a02 * a21 - a01 * a22) * inv_det;
    let i02 = (a01 * a12 - a02 * a11) * inv_det;
    let i10 = c01 * inv_det;
    let i11 = (a00 * a22 - a02 * a20) * inv_det;
    let i12 = (a02 * a10 - a00 * a12) * inv_det;
    let i20 = c02 * inv_det;
    let i21 = (a01 * a20 - a00 * a21) * inv_det;
    let i22 = (a00 * a11 - a01 * a10) * inv_det;

    let i03 = -(i00 * a03 + i01 * a13 + i02 * a23);
    let i13 = -(i10 * a03 + i11 * a13 + i12 * a23);
    let i23 = -(i20 * a03 + i21 * a13 + i22 * a23);

    let b = child.map(f64::from);
    let local = [
        (i00 * b[0] + i01 * b[4] + i02 * b[8]) as f32,
        (i00 * b[1] + i01 * b[5] + i02 * b[9]) as f32,
        (i00 * b[2] + i01 * b[6] + i02 * b[10]) as f32,
        (i00 * b[3] + i01 * b[7] + i02 * b[11] + i03) as f32,
        (i10 * b[0] + i11 * b[4] + i12 * b[8]) as f32,
        (i10 * b[1] + i11 * b[5] + i12 * b[9]) as f32,
        (i10 * b[2] + i11 * b[6] + i12 * b[10]) as f32,
        (i10 * b[3] + i11 * b[7] + i12 * b[11] + i13) as f32,
        (i20 * b[0] + i21 * b[4] + i22 * b[8]) as f32,
        (i20 * b[1] + i21 * b[5] + i22 * b[9]) as f32,
        (i20 * b[2] + i21 * b[6] + i22 * b[10]) as f32,
        (i20 * b[3] + i21 * b[7] + i22 * b[11] + i23) as f32,
    ];
    let ident = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    let non_ident = local
        .iter()
        .zip(ident.iter())
        .any(|(&v, &id)| !v.is_finite() || (v - id).abs() > 1e-5);
    (local, non_ident)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sdg_export::{ActiveTrafficBackend, TrafficBackend};

    #[test]
    fn traffic_backend_labels_and_defaults() {
        assert_eq!(TrafficBackend::default(), TrafficBackend::Auto);
        assert_eq!(TrafficBackend::ALL, [TrafficBackend::Auto, TrafficBackend::Cpu, TrafficBackend::Cuda]);
        assert_eq!(TrafficBackend::Auto.label(), "Auto");
        assert_eq!(TrafficBackend::Cpu.label(), "CPU");
        assert_eq!(TrafficBackend::Cuda.label(), "CUDA");
        assert_eq!(ActiveTrafficBackend::default(), ActiveTrafficBackend::None);
        assert_eq!(ActiveTrafficBackend::None.label(), "Idle");
        assert_eq!(ActiveTrafficBackend::Cpu.label(), "CPU");
        assert_eq!(ActiveTrafficBackend::Cuda.label(), "CUDA");
    }

    #[test]
    fn compute_local_xform_recovers_exact_child_offset_and_detects_identity() {
        let parent = [
            0.0, 0.0, 1.0, 10.0, // 90 deg yaw + translate (10, 2, -5)
            0.0, 1.0, 0.0, 2.0, //
            -1.0, 0.0, 0.0, -5.0,
        ];
        let (id_local, non_id) = compute_local_xform_from_world(parent, parent);
        assert!(!non_id, "identical parent/child must be detected as identity");
        assert!((id_local[0] - 1.0).abs() < 1e-5 && id_local[3].abs() < 1e-5);

        let local_want = [
            1.0, 0.0, 0.0, 0.25, //
            0.0, 1.0, 0.0, 1.15, //
            0.0, 0.0, 1.0, -0.8,
        ];
        let child = (Mat3x4(parent) * Mat3x4(local_want)).0;
        let (recovered, is_non_id) = compute_local_xform_from_world(parent, child);
        assert!(is_non_id);
        for (k, (&got, &want)) in recovered.iter().zip(local_want.iter()).enumerate() {
            assert!(
                (got - want).abs() < 1e-5,
                "local_xform[{k}]: got {got} != want {want}"
            );
        }
    }

    #[test]
    fn traffic_frame_plan_validates_for_nv_demo() {
        let plan = traffic_frame_plan(csg_device::CSG_EVAL_CUH);
        plan.validate().expect("traffic_frame_plan validates");
        assert_eq!(plan.version, TRAFFIC_FRAME_PLAN_VERSION);
        assert_eq!(plan.kind, PlanKind::Traffic);
    }
}
