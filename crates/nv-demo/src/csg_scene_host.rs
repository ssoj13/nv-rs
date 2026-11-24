//! 100% scene-agnostic `csg-rs` host lifecycle, batched per-compound OptiX GAS cache, and
//! zero-churn GPU uploads.
//!
//! What: loads any registered [`csg_scene::SceneModule`] from [`csg_scenes::default_scene_registry`]
//! (`Gundo`, `Marina del Rey`, `SF House of AI`, `Studio Asset Inspector`, or future scenes),
//! batches all compound `OptixAabb`s in a single H2D upload while caching one custom-primitive GAS
//! per unique `GpuCompound`, refreshes material palettes and 3D Frenet traffic in-place with zero
//! steady-state GPU buffer reallocations, and synthesizes exact-CSG random rails on demand.
//! Why: keeps `nv-demo` completely decoupled from scene-specific coordinates, presets, or rails
//! while delivering smooth frame times across 10k-100k+ CSG instances.
//! Used by: `crate::renderer`, `crate::ui`, `crate::main`.

use std::collections::{HashMap, VecDeque};
use std::ffi::c_void;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::Arc;

use bytemuck::Pod;
use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, DeviceBuffer, Stream};
use optix::{
    Aabb, Accel, BuildOptions, CustomPrimitives, DeviceContext, GeometryFlags, Instance, Keys,
    Scratch,
};

use csg_actors::{
    ArchetypeKind, CrowdSim, CsgRayOccluder, RayOccluder, SdgActorRecord, SdgCamera, SdgProjection,
    Vec3 as ActorVec3, extract_sdg_frame,
};
use csg_core::abi::{compound_flags, opcode};
use csg_core::math::{Aabb as CsgAabb, Mat3x4, Vec3};
use csg_core::{CompiledLibrary, CompoundId, GpuAabb, GpuCompound, GpuOp, GpuPrim, GpuRepeat, Ray};
use csg_scene::{
    CameraPose as CsgCameraPose, CameraView, DetailTier, GpuInstance,
    GpuMaterial, LodConfig, MAT_OVERRIDE_NONE, PackedGpuScene, RailGenConfig, RailSpec,
    SHADOW_GUARD_MIN_DIAMETER, SceneBuildConfig, SceneBuilt, SceneDescriptor, SceneHints,
    SceneRegistry, SceneViewpoint, TimeOfDay, WorldScene, find_crowd_actor_for_inst,
    generate_random_rail, inst_flags, is_dynamic_inst_id, list_possess_targets_with_crowd,
    lookup_instance_and_assembly, pick_possessable_instance_with_crowd,
    resolve_possess_pose_with_crowd,
};

use crate::app_events::PossessInfo;
use crate::cuda_crowd::CrowdLodParams;
use crate::renderer::{CameraFrame, CameraProjection};

/// SBT hit-group offset for general multi-op CSG RPN (`__intersection__csg` + `__closesthit__csg`,
/// paired with shadow hit group `CSG_SBT_OFFSET + 1` under `SBT_STRIDE = 2`).
pub const CSG_SBT_OFFSET: u32 = 8;

/// SBT hit-group offset for ultra-lean single-primitive & Multi-AABB `PURE_UNION` CSG
/// (`__intersection__csg_prim` + `__closesthit__csg`, paired with shadow hit group
/// `CSG_SBT_PRIM_OFFSET + 1` under `SBT_STRIDE = 2`).
pub const CSG_SBT_PRIM_OFFSET: u32 = 10;

/// OptiX instance flag disabling any-hit programs on opaque CSG instances.
const OPTIX_INST_DISABLE_ANYHIT: u32 = optix::INSTANCE_FLAG_DISABLE_ANYHIT;

/// Maximum cached `SceneBuilt` entries retained across scene/road-mode switches.
const SCENE_LRU_MAX_ENTRIES: usize = 4;

/// Maximum combined estimated CPU byte budget for the `SceneBuilt` LRU cache (`512 MiB`).
const SCENE_LRU_MAX_BYTES: usize = 512 * 1024 * 1024;

/// Half extent (m) of the local AABB given to an `UNBOUNDED` compound (ground, ocean). It must
/// exceed any camera-to-compound-origin distance plus the `CSG_VIEW_FAR_M` view range on the
/// largest maps; one oversized box costs nothing because every ray enters an unbounded
/// primitive anyway.
const UNBOUNDED_COMPOUND_HALF_EXTENT_M: f32 = 10_000.0;

/// Far plane / omnidirectional radius (m) of the CSG packing view.
const CSG_VIEW_FAR_M: f32 = 1500.0;

/// Maps a packed CSG compound id to `(gas_index, visible)`. Every packed slot must yield exactly
/// one OptiX instance, because the CUDA crowd scatter writes
/// `d_optix_instances[classic_count + packed_slot]`; a compound without a GAS (a partially failed
/// `spawn_crowd_stress`) becomes an invisible placeholder on GAS 0. `None` when no GAS exists.
#[inline]
const fn optix_slot_gas(cid: usize, gas_count: usize) -> Option<(usize, bool)> {
    if cid < gas_count {
        Some((cid, true))
    } else if gas_count > 0 {
        Some((0, false))
    } else {
        None
    }
}

/// Maps a 1-based `scene_idx` (`csg_mode`) to a registry index, rejecting out-of-range values
/// (for example a persisted index after the registry shrank) instead of loading another scene.
fn registry_index_for_scene(scene_idx: u32, reg_count: usize) -> Result<usize, String> {
    if reg_count == 0 {
        return Err("CSG SceneRegistry is empty".to_string());
    }
    let idx = (scene_idx as usize).checked_sub(1).ok_or_else(|| {
        "CSG scene index 0 is the classic stage, not a registry scene".to_string()
    })?;
    if idx >= reg_count {
        return Err(format!(
            "CSG scene index {scene_idx} is out of range (registry has {reg_count} scenes)"
        ));
    }
    Ok(idx)
}

/// OptiX GAS build strategy chosen for a compiled CSG compound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CompoundGasPlan {
    /// Single primitive (`op_count == 1`, `PUSH_PRIM`) -> 1-AABB GAS with `primitive_offset = prim_idx`, SBT 10.
    SinglePrim { prim_idx: u32 },
    /// Pure union of `count >= 2` contiguous finite primitives (no `FINITE_REPEAT`) -> `count`-AABB
    /// hardware BVH GAS with `primitive_offset = first_prim`, SBT 10.
    MultiAabbUnion { first_prim: u32, count: usize },
    /// General RPN / `SUBTRACT` / `INTERSECT` / `FINITE_REPEAT` -> 1-AABB GAS with `primitive_offset = 0`, SBT 8.
    GeneralRpn,
}

impl CompoundGasPlan {
    #[inline]
    const fn sbt_offset(self) -> u32 {
        match self {
            Self::SinglePrim { .. } | Self::MultiAabbUnion { .. } => CSG_SBT_PRIM_OFFSET,
            Self::GeneralRpn => CSG_SBT_OFFSET,
        }
    }
}

/// Inspects a compiled compound in `lib` and selects the fastest valid OptiX GAS strategy.
///
/// Eligible `PURE_UNION` compounds with `M >= 2` contiguous `PUSH_PRIM`s (no `FINITE_REPEAT`,
/// bounded finite primitive AABBs) build an `M`-AABB custom-primitive GAS with
/// `primitive_offset = first_prim` so hardware RT Cores traverse the compound's internal BVH
/// and invoke `__intersection__csg_prim[_shadow]` with `optixGetPrimitiveIndex() == prim_idx`.
/// Only `PURE_UNION` (pairwise separated parts) qualifies: a union whose parts overlap or touch
/// carries `FIRST_HIT_UNION` instead, and the per-primitive closest hit would report internal
/// faces for rays starting inside a part, so it stays on general RPN where `csg_eval.cuh` picks
/// the first-hit or exact span path per ray.
fn compound_gas_plan(lib: &CompiledLibrary, comp: &GpuCompound) -> CompoundGasPlan {
    let op_start = comp.op_offset as usize;
    let op_end = op_start.saturating_add(comp.op_count as usize);
    let Some(ops) = lib.ops.get(op_start..op_end) else {
        return CompoundGasPlan::GeneralRpn;
    };

    if (comp.flags & compound_flags::SINGLE_PRIM) != 0
        && ops.len() == 1
        && ops[0].opcode == opcode::PUSH_PRIM
        && (ops[0].arg0 as usize) < lib.prims.len()
    {
        return CompoundGasPlan::SinglePrim {
            prim_idx: ops[0].arg0,
        };
    }

    if (comp.flags & compound_flags::PURE_UNION) != 0
        && (comp.flags & compound_flags::UNBOUNDED) == 0
    {
        let mut first_prim: Option<u32> = None;
        let mut count: usize = 0;
        let mut eligible = true;
        for op in ops {
            match op.opcode {
                opcode::PUSH_PRIM => {
                    let pid = op.arg0;
                    let expected = match first_prim {
                        None => {
                            first_prim = Some(pid);
                            pid
                        }
                        Some(base) => base.wrapping_add(count as u32),
                    };
                    if pid != expected {
                        eligible = false;
                        break;
                    }
                    let Some(prim) = lib.prims.get(pid as usize) else {
                        eligible = false;
                        break;
                    };
                    let finite_aabb = (0..3).all(|j| {
                        prim.aabb_min[j].is_finite()
                            && prim.aabb_max[j].is_finite()
                            && prim.aabb_min[j] <= prim.aabb_max[j]
                    });
                    if !finite_aabb {
                        eligible = false;
                        break;
                    }
                    count += 1;
                }
                opcode::UNION | opcode::GUARD_AABB | opcode::PUSH_EMPTY => {}
                // `FINITE_REPEAT` or any non-union op stays on general RPN (`hit_csg`).
                _ => {
                    eligible = false;
                    break;
                }
            }
        }
        if eligible && let Some(base) = first_prim && count >= 2 {
            return CompoundGasPlan::MultiAabbUnion {
                first_prim: base,
                count,
            };
        }
    }

    CompoundGasPlan::GeneralRpn
}

/// OptiX `BuildOptions` for per-compound CSG custom-primitive GASes (`compact = false` eliminates
/// ~246 synchronous GPU compaction stalls on every scene or road-mode switch).
#[inline]
fn csg_gas_build_opts() -> BuildOptions {
    BuildOptions {
        prefer_fast_trace: true,
        compact: false,
        ..Default::default()
    }
}

/// Builds a non-degenerate, slightly padded OptiX custom-primitive AABB for a single `GpuPrim`.
#[inline]
fn sanitized_prim_aabb(prim: &GpuPrim) -> Aabb {
    let mut bmin = prim.aabb_min;
    let mut bmax = prim.aabb_max;
    for i in 0..3 {
        if !bmin[i].is_finite() || !bmax[i].is_finite() || bmin[i] > bmax[i] {
            bmin[i] = -0.01;
            bmax[i] = 0.01;
        } else {
            bmin[i] -= 1.0e-4;
            bmax[i] += 1.0e-4;
        }
    }
    Aabb {
        min: bmin,
        max: bmax,
    }
}

/// Sanitized, NaN-free equality key for [`LodConfig`] so any change to any distance, tier,
/// screen-pixel, force-LOD, hysteresis, or enabled setting triggers an immediate `FullRepack`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct LodConfigKey {
    enabled: bool,
    force_lod: Option<u8>,
    lod_bias_bits: u32,
    lod1_dist_bits: u32,
    lod2_dist_bits: u32,
    meso_dist_bits: u32,
    micro_dist_bits: u32,
    min_screen_px_bits: u32,
    hysteresis_bits: u32,
}

#[inline]
fn sanitize_lod_config(cfg: LodConfig) -> LodConfig {
    let lod_bias = if cfg.lod_bias.is_finite() {
        cfg.lod_bias.clamp(0.25, 4.0)
    } else {
        1.0
    };
    let lod1_dist_m = if cfg.lod1_dist_m.is_finite() {
        cfg.lod1_dist_m.clamp(1.0, 5000.0)
    } else {
        80.0
    };
    let lod2_dist_m = if cfg.lod2_dist_m.is_finite() {
        cfg.lod2_dist_m.clamp(lod1_dist_m, 10_000.0)
    } else {
        220.0_f32.max(lod1_dist_m)
    };
    let meso_dist_m = if cfg.meso_dist_m.is_finite() {
        cfg.meso_dist_m.clamp(1.0, 10_000.0)
    } else {
        300.0
    };
    let micro_dist_m = if cfg.micro_dist_m.is_finite() {
        cfg.micro_dist_m.clamp(1.0, meso_dist_m)
    } else {
        35.0_f32.min(meso_dist_m)
    };
    let min_screen_px = if cfg.min_screen_px.is_finite() {
        cfg.min_screen_px.clamp(0.0, 64.0)
    } else {
        1.5
    };
    let hysteresis = if cfg.hysteresis.is_finite() {
        cfg.hysteresis.clamp(0.0, 0.45)
    } else {
        0.15
    };
    LodConfig {
        enabled: cfg.enabled,
        lod_bias,
        force_lod: cfg.force_lod.map(|v| v.min(2)),
        lod1_dist_m,
        lod2_dist_m,
        meso_dist_m,
        micro_dist_m,
        min_screen_px,
        hysteresis,
    }
}

impl LodConfigKey {
    #[inline]
    fn from_config(cfg: LodConfig) -> Self {
        let s = sanitize_lod_config(cfg);
        Self {
            enabled: s.enabled,
            force_lod: s.force_lod,
            lod_bias_bits: s.lod_bias.to_bits(),
            lod1_dist_bits: s.lod1_dist_m.to_bits(),
            lod2_dist_bits: s.lod2_dist_m.to_bits(),
            meso_dist_bits: s.meso_dist_m.to_bits(),
            micro_dist_bits: s.micro_dist_m.to_bits(),
            min_screen_px_bits: s.min_screen_px.to_bits(),
            hysteresis_bits: s.hysteresis.to_bits(),
        }
    }
}

/// Camera basis and viewport parameters driving live CSG frustum + distance/screen-space LOD culling.
#[derive(Clone, Copy, Debug)]
pub struct CsgCameraParams {
    /// World-space camera origin.
    pub eye: [f32; 3],
    /// Unit camera forward vector.
    pub fwd: [f32; 3],
    /// Unit camera up vector.
    pub up: [f32; 3],
    /// Render target width in pixels.
    pub viewport_w: u32,
    /// Render target height in pixels.
    pub viewport_h: u32,
    /// Vertical field of view in radians.
    pub vfov_rad: f32,
    /// Full distance, screen-space, detail-tier, and force-LOD configuration.
    pub lod_config: LodConfig,
    /// Use 360-degree omnidirectional packing (for `Fisheye180`, `Equirect360`, and `SlitCam`).
    pub omnidirectional: bool,
}

/// Bitflags packed into `LaunchParams::csg_flags`.
pub mod csg_flags {
    /// CSG scene is active and device pointers are valid.
    pub const ENABLED: u32 = 1 << 0;
    /// Night lighting mode (boosts emissive streetlights, headlights, beacons).
    pub const NIGHT: u32 = 1 << 1;
    /// Wet asphalt puddles enabled.
    pub const WET_ASPHALT: u32 = 1 << 2;
    /// Golden sunset lighting mode.
    pub const SUNSET: u32 = 1 << 3;
}

/// Active CSG scene telemetry reported back to the UI each frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CsgStats {
    /// Visible CSG instances submitted into the OptiX IAS this frame.
    pub visible_instances: u32,
    /// Total static + dynamic instances registered in the world.
    pub total_instances: u32,
    /// Unique compiled CSG compounds (and cached OptiX GASes).
    pub compounds: u32,
    /// Total compiled analytical primitives across all compounds.
    pub primitives: u32,
    /// Combined GPU memory footprint of the 8 CSG device buffers in bytes.
    pub gpu_bytes: u64,
    /// Active GPU memory footprint of the `aux_planes` buffer (DEM / road tiles / convex planes) in bytes.
    pub aux_planes_bytes: u64,
    /// Visible parts emitted at LOD 0 (full detail).
    pub lod0_count: u32,
    /// Visible parts emitted at LOD 1 (mid detail).
    pub lod1_count: u32,
    /// Visible parts emitted at LOD 2+ (coarse / auto-proxy silhouette).
    pub lod2_count: u32,
    /// Total instances/parts culled this frame (frustum + detail tier + screen-px + budget).
    pub culled_count: u32,
}

/// Scene metadata delivered once to the UI thread whenever a new CSG scene finishes building.
#[derive(Clone, Debug, PartialEq)]
pub struct SceneLoadedMetadata {
    /// Active `csg_mode` (`0 = Off`, `1..=N` = registry index + 1).
    pub scene_idx: u32,
    /// Active variant index (e.g. `Inspector` asset index).
    pub variant_idx: u32,
    /// Curated static camera viewpoints exported by the scene module.
    pub viewpoints: Vec<SceneViewpoint>,
    /// Collision-verified cinematic camera rails exported by the scene module.
    pub rails: Vec<RailSpec>,
}

/// Device pointer bundle copied into `LaunchParams` each frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CsgDevicePointers {
    pub compounds: u64,
    pub ops: u64,
    pub prims: u64,
    pub aux_planes: u64,
    pub guard_aabbs: u64,
    pub repeats: u64,
    pub materials: u64,
    pub instances: u64,
    pub prev_xforms: u64,
    pub instance_count: u32,
    pub material_count: u32,
    pub compound_count: u32,
}

/// Structural configuration key that triggers a full `WorldScene` + GAS rebuild.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CsgSceneKey {
    /// `0 = Off (Classic)`, `1..=registry.len()` maps to `registry.get_by_index(scene_idx - 1)`.
    pub scene_idx: u32,
    /// Scene-specific variant index (e.g. asset index for `Inspector`).
    pub variant_idx: u32,
    /// Deterministic world / asset seed.
    pub seed: u32,
    /// Active road surface representation (`0=Watertight 3D, 1=C1 Tangent, 2=Hybrid C1+SDF, 3=F-Rep Tile`).
    pub road_mode: u8,
}

/// Returns descriptors for all registered pure-CSG scene modules in deterministic order.
pub fn default_scene_descriptors() -> Vec<SceneDescriptor> {
    csg_scenes::default_scene_registry().descriptors()
}

/// Maps UI `csg_time_of_day` (`0 = Day`, `1 = Sunset`, `2 = Night`) to [`TimeOfDay`].
pub const fn time_of_day_from_u32(v: u32) -> TimeOfDay {
    match v {
        0 => TimeOfDay::Day,
        2 => TimeOfDay::Night,
        _ => TimeOfDay::Sunset,
    }
}

/// Builds a non-degenerate, slightly padded OptiX custom-primitive AABB for a compiled compound.
fn sanitized_compound_aabb(comp: &GpuCompound) -> Aabb {
    if (comp.flags & compound_flags::UNBOUNDED) != 0 {
        let h = UNBOUNDED_COMPOUND_HALF_EXTENT_M;
        return Aabb {
            min: [-h, -h, -h],
            max: [h, h, h],
        };
    }
    let mut bmin = comp.aabb_min;
    let mut bmax = comp.aabb_max;
    for i in 0..3 {
        if !bmin[i].is_finite() || !bmax[i].is_finite() || bmin[i] > bmax[i] {
            bmin[i] = -0.01;
            bmax[i] = 0.01;
        } else {
            bmin[i] -= 1.0e-4;
            bmax[i] += 1.0e-4;
        }
    }
    Aabb {
        min: bmin,
        max: bmax,
    }
}

/// Result of [`CsgGpuHost::sync`], indicating how the OptiX IAS should be updated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsgSyncResult {
    /// Neither CSG scene topology nor dynamic traffic changed this frame.
    Unchanged,
    /// Only the leading `dynamic_count` moving traffic instances changed transforms with
    /// identical instance count and compound topology (eligible for pinned DMA prefix update
    /// + in-place `OPTIX_BUILD_OPERATION_UPDATE`).
    TrafficOnly {
        /// Number of leading dynamic CSG instances updated at `0..dynamic_count`.
        dynamic_count: usize,
    },
    /// Scene topology, camera frustum, LODs, or instance count changed (requires full IAS upload).
    FullRepack,
}

/// Default pinned host staging capacity of each of the two CSG staging buffers (`2 MiB`, not the
/// `256 KiB` of `optix::InstanceBuffer`) = 32,768 `GpuInstance` records or 43,690
/// `[f32; 12]` previous-frame transforms) for zero-stall async DMA updates of `dynamic_instances[0..D]`
/// across 5x urban crowds (600-900+ actors / ~10k-15k+ bones).
const CSG_PINNED_STAGING_BYTES: usize = 2 * 1024 * 1024;

/// Hard upper cap for grow-on-demand pinned host staging (`16 MiB` = 262,144 `GpuInstance`s).
const CSG_PINNED_STAGING_MAX_BYTES: usize = 16 * 1024 * 1024;

/// Reusable grow-on-demand page-locked host staging buffer (`cuMemAllocHost_v2` / `cuMemFreeHost`).
struct PinnedStagingBuffer {
    ptr: *mut c_void,
    bytes: usize,
    ctx: Arc<CudaContext>,
}

// SAFETY: page-locked CUDA host memory is context-owned and not bound to a single OS thread;
// mutable access is exclusively serialized through `&mut CsgGpuHost`.
unsafe impl Send for PinnedStagingBuffer {}

impl PinnedStagingBuffer {
    fn try_new(ctx: &Arc<CudaContext>, bytes: usize) -> Option<Self> {
        if bytes == 0 || ctx.bind_to_thread().is_err() {
            return None;
        }
        let mut ptr: *mut c_void = std::ptr::null_mut();
        // SAFETY: `ctx` is current on this thread and `ptr` is a valid out-pointer.
        let res = unsafe { cu::cuMemAllocHost_v2(&mut ptr, bytes) };
        if res == cu::cudaError_enum_CUDA_SUCCESS && !ptr.is_null() {
            Some(Self {
                ptr,
                bytes,
                ctx: ctx.clone(),
            })
        } else {
            None
        }
    }

    /// Ensures this page-locked staging buffer holds at least `needed_bytes`, growing power-of-two
    /// up to [`CSG_PINNED_STAGING_MAX_BYTES`] with zero cost once warm.
    fn ensure_capacity(&mut self, stream: &Arc<Stream>, needed_bytes: usize) -> bool {
        if needed_bytes <= self.bytes {
            return true;
        }
        if needed_bytes > CSG_PINNED_STAGING_MAX_BYTES {
            return false;
        }
        // Synchronize only on the rare cold growth edge so any in-flight async DMA reading the
        // previous host staging buffer finishes before `cuMemFreeHost`.
        if stream.synchronize().is_err() {
            return false;
        }
        let target = needed_bytes
            .next_power_of_two()
            .min(CSG_PINNED_STAGING_MAX_BYTES);
        if let Some(grown) = Self::try_new(&self.ctx, target) {
            *self = grown;
            true
        } else {
            false
        }
    }
}

impl Drop for PinnedStagingBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.ctx.bind_to_thread().is_ok() {
            // SAFETY: `self.ptr` was allocated via `cuMemAllocHost_v2` on `self.ctx`.
            let rc = unsafe { cu::cuMemFreeHost(self.ptr) };
            if rc != cu::cudaError_enum_CUDA_SUCCESS {
                eprintln!("nv-demo: PinnedStagingBuffer cuMemFreeHost failed ({rc:?})");
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}

/// Reusable staging for Multi-AABB compound GAS inputs: the host boxes of the compound being
/// built plus one persistent device buffer per primitive count.
#[derive(Default)]
struct MultiAabbStaging {
    host: Vec<Aabb>,
    device: HashMap<usize, DeviceBuffer<Aabb>>,
}

/// Grow-only typed GPU buffer backed by 256-byte-aligned `DeviceBuffer<u8>`, avoiding
/// steady-state CUDA allocations on per-frame updates and `RoadMode`/scene switches.
struct GrowDeviceBuffer<T: Pod> {
    buf: Option<DeviceBuffer<u8>>,
    cap_elems: usize,
    len_elems: usize,
    _marker: PhantomData<T>,
}

impl<T: Pod> Default for GrowDeviceBuffer<T> {
    fn default() -> Self {
        Self {
            buf: None,
            cap_elems: 0,
            len_elems: 0,
            _marker: PhantomData,
        }
    }
}

impl<T: Pod> GrowDeviceBuffer<T> {
    /// Uploads `items`. `len_elems` reaches `items.len()` only after the allocation and copy
    /// were enqueued; on any failure it stays `0`, so `device_ptr()` returns `0` and no consumer
    /// indexes past a smaller old allocation.
    fn upload(&mut self, stream: &Arc<Stream>, items: &[T]) -> Result<(), String> {
        self.len_elems = 0;
        if items.is_empty() {
            return Ok(());
        }
        if items.len() > self.cap_elems {
            let new_cap = items.len().max(self.cap_elems.saturating_mul(2)).max(64);
            let new_bytes = new_cap
                .checked_mul(std::mem::size_of::<T>())
                .ok_or_else(|| "CSG grow buffer byte size overflow".to_string())?;
            let new_buf = DeviceBuffer::zeroed(stream, new_bytes)
                .map_err(|e| format!("CSG grow buffer alloc ({new_bytes} B) failed: {e}"))?;
            self.buf = Some(new_buf);
            self.cap_elems = new_cap;
        }
        if let Some(ref buf) = self.buf {
            let bytes: &[u8] = bytemuck::cast_slice(items);
            // SAFETY: `buf` holds at least `self.cap_elems * size_of::<T>() >= bytes.len()`
            // 256-byte-aligned device bytes (`cuMemAlloc`), `bytes` points to valid `Pod` host
            // storage that remains immutable until CUDA stages the pageable copy.
            let res = unsafe {
                cu::cuMemcpyHtoDAsync_v2(
                    buf.cu_deviceptr(),
                    bytes.as_ptr().cast(),
                    bytes.len(),
                    stream.cu_stream(),
                )
            };
            if res != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!("CSG cuMemcpyHtoDAsync_v2 failed ({res:?})"));
            }
            self.len_elems = items.len();
        }
        Ok(())
    }

    /// Streams a leading prefix `items` into `self.buf[0..items.len()]` using `pinned` staging
    /// memory (growing on demand up to `16 MiB`) so `cuMemcpyHtoDAsync_v2` executes as true
    /// asynchronous DMA with zero VRAM allocations and zero pageable-memory stalls.
    fn upload_prefix_pinned(
        &self,
        stream: &Arc<Stream>,
        pinned: Option<&mut PinnedStagingBuffer>,
        items: &[T],
    ) -> Result<(), String> {
        if items.is_empty() {
            return Ok(());
        }
        if items.len() > self.len_elems {
            return Err(format!(
                "CSG prefix upload ({}) exceeds active buffer length ({})",
                items.len(),
                self.len_elems
            ));
        }
        let Some(ref buf) = self.buf else {
            return Err("CSG prefix upload target buffer unallocated".to_string());
        };
        let bytes: &[u8] = bytemuck::cast_slice(items);
        if let Some(stage) = pinned
            && stage.ensure_capacity(stream, bytes.len())
        {
            // SAFETY: `bytes` is valid initialized `Pod` memory, `stage.ptr` holds at least
            // `bytes.len()` page-locked host bytes, and `buf` holds at least `bytes.len()` device bytes.
            let res = unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), stage.ptr.cast::<u8>(), bytes.len());
                cu::cuMemcpyHtoDAsync_v2(
                    buf.cu_deviceptr(),
                    stage.ptr,
                    bytes.len(),
                    stream.cu_stream(),
                )
            };
            if res != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!("CSG pinned cuMemcpyHtoDAsync_v2 failed ({res:?})"));
            }
            return Ok(());
        }
        // SAFETY: `buf` holds at least `bytes.len()` device bytes; fallback pageable upload.
        let res = unsafe {
            cu::cuMemcpyHtoDAsync_v2(
                buf.cu_deviceptr(),
                bytes.as_ptr().cast(),
                bytes.len(),
                stream.cu_stream(),
            )
        };
        if res != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("CSG fallback prefix cuMemcpyHtoDAsync_v2 failed ({res:?})"));
        }
        Ok(())
    }

    fn device_ptr(&self) -> u64 {
        if self.len_elems == 0 {
            0
        } else {
            self.buf.as_ref().map_or(0, DeviceBuffer::cu_deviceptr)
        }
    }

    fn clear(&mut self) {
        self.len_elems = 0;
    }

    fn active_bytes(&self) -> usize {
        self.len_elems * std::mem::size_of::<T>()
    }
}

/// Serializable Synthetic Data Generation (SDG) frame ground-truth payload exported as JSON.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SdgExportFrame {
    /// Active registered scene identifier (e.g. `"inspector"`, `"gundo"`, `"marina"`).
    pub scene_id: String,
    /// Active 1-based scene index (`csg_mode`).
    pub scene_idx: u32,
    /// Active variant index (e.g. `109` for the 4-archetype actor showcase).
    pub variant_idx: u32,
    /// Render frame index at export time.
    pub frame_index: u32,
    /// Simulation clock in seconds at export time.
    pub time_s: f32,
    /// Exact optical sensor model and camera extrinsics/intrinsics.
    pub camera: SdgCamera,
    /// Extracted 2D/3D keypoints, tight bone-OBB bounding boxes, and ray-occluded visibility labels.
    pub actors: Vec<SdgActorRecord>,
}

/// Sentinel indicating a dynamic instance has no prior LOD history yet.
const PREV_LOD_NONE: u8 = u8::MAX;

#[inline]
fn csg_aabb_distance_sq(aabb: CsgAabb, p: Vec3) -> f32 {
    if aabb.is_empty() || !p.x.is_finite() || !p.y.is_finite() || !p.z.is_finite() {
        return f32::INFINITY;
    }
    if !aabb.is_finite() {
        return 0.0;
    }
    let dx = (aabb.min.x - p.x).max(0.0).max(p.x - aabb.max.x);
    let dy = (aabb.min.y - p.y).max(0.0).max(p.y - aabb.max.y);
    let dz = (aabb.min.z - p.z).max(0.0).max(p.z - aabb.max.z);
    dx * dx + dy * dy + dz * dz
}

#[inline]
fn xform_max_scale(m: Mat3x4) -> f32 {
    let a = &m.0;
    let c0 = a[0] * a[0] + a[4] * a[4] + a[8] * a[8];
    let c1 = a[1] * a[1] + a[5] * a[5] + a[9] * a[9];
    let c2 = a[2] * a[2] + a[6] * a[6] + a[10] * a[10];
    let max_sq = c0.max(c1).max(c2);
    if max_sq.is_finite() && max_sq > 0.0 {
        max_sq.sqrt()
    } else {
        1.0
    }
}

/// Packs ONLY `world.dynamic_instances()` against `view` into `out_insts` & `out_prev` with zero
/// iteration over static spatial chunks and zero static-tail topology hashing.
///
/// Mirrors `WorldScene::pack_visible` dynamic pass verbatim (honoring `LodConfig`, `min_screen_px`,
/// `DetailTier`, `select_lod_with_config`, and `AssemblyPart::active_proxy`) so `TrafficOnly` ticks
/// run in O(`dynamic_instances.len()`) — or O(`traffic_vehicles`) when `skip_crowd_bones` is
/// enabled for GPU-direct CUDA crowds.
fn pack_dynamic_prefix_only(
    world: &WorldScene,
    view: &CameraView,
    skip_crowd_bones: bool,
    dyn_prev_lods: &mut Vec<u8>,
    out_insts: &mut Vec<GpuInstance>,
    out_prev: &mut Vec<[f32; 12]>,
) {
    out_insts.clear();
    out_prev.clear();
    let dyn_slice = world.dynamic_instances();
    if dyn_prev_lods.len() != dyn_slice.len() {
        dyn_prev_lods.resize(dyn_slice.len(), PREV_LOD_NONE);
    }

    let fixed_bytes = world.fixed_gpu_bytes();
    let max_emit = world.budget.instance_budget(fixed_bytes);
    let lod_cfg = view.effective_lod_config();
    let max_radius = if view.max_radius.is_nan() {
        0.0
    } else {
        view.max_radius.max(0.0)
    };
    let max_radius_sq = if max_radius.is_finite() {
        max_radius * max_radius
    } else {
        f32::INFINITY
    };
    let shadow_radius = if view.shadow_guard_radius.is_finite() {
        view.shadow_guard_radius.max(0.0).min(max_radius)
    } else {
        0.0
    };
    let shadow_radius_sq = shadow_radius * shadow_radius;

    for (dyn_idx, (inst, prev_lod_slot)) in
        dyn_slice.iter().zip(dyn_prev_lods.iter_mut()).enumerate()
    {
        // The world's kind tag is the authority: `user_id` is an application id (some generators
        // derive it from OSM ids) and its numeric range does not identify crowd bones.
        let crowd_actor = world.crowd_actor_of_dynamic(dyn_idx);
        let is_crowd_bone = crowd_actor.is_some();
        if skip_crowd_bones && is_crowd_bone {
            continue;
        }
        if !inst.is_visible() {
            continue;
        }
        let Some(assembly) = world.assembly(inst.assembly_id) else {
            continue;
        };
        // csg-scene owns the per-actor visibility rule (frustum + draw radius on the actor's
        // bounds, honouring `set_crowd_culling`); a private copy here drifted once already.
        if crowd_actor.is_some_and(|actor| !world.crowd_actor_visible(actor, view)) {
            continue;
        }
        let world_aabb = inst.xform.xform_aabb(assembly.local_aabb);
        let dist_sq = csg_aabb_distance_sq(world_aabb, view.position);
        if !is_crowd_bone && max_radius_sq.is_finite() && dist_sq > max_radius_sq {
            continue;
        }
        let world_diam = world_aabb.diameter();
        let in_frustum = is_crowd_bone
            || view
                .frustum
                .is_none_or(|frustum| frustum.intersects_aabb(world_aabb));
        if !in_frustum {
            let keep_shadow = (inst.flags & inst_flags::CAST_SHADOW) != 0
                && shadow_radius_sq > 0.0
                && dist_sq <= shadow_radius_sq
                && (!world_diam.is_finite() || world_diam >= SHADOW_GUARD_MIN_DIAMETER);
            if !keep_shadow {
                continue;
            }
        }
        let center = if assembly.local_aabb.is_empty() || !assembly.local_aabb.is_finite() {
            inst.position()
        } else {
            inst.xform.xform_pt(assembly.local_aabb.center())
        };
        let dist = (center - view.position).len();
        let part_end = if is_crowd_bone {
            assembly.parts.len()
        } else {
            assembly.visible_part_end_with_config(dist, view.small_detail_cull_factor, &lod_cfg)
        };
        if part_end == 0 {
            continue;
        }
        // Same projection of the kind tag as `WorldScene::pack_visible` (`inst_flags::CROWD_BONE`).
        let packed_flags = inst_flags::with_crowd_bone(inst.flags, is_crowd_bone);

        let scale_max = xform_max_scale(inst.xform);
        let prev_lod = if *prev_lod_slot == PREV_LOD_NONE {
            None
        } else {
            Some(*prev_lod_slot as usize)
        };
        let min_px = if !is_crowd_bone
            && lod_cfg.enabled
            && view.small_detail_cull_factor.is_finite()
            && view.small_detail_cull_factor > 0.0
        {
            lod_cfg.sanitized_min_screen_px()
        } else {
            0.0
        };
        let mut recorded_lod: Option<usize> = None;
        let visible_parts = assembly.parts.get(..part_end).unwrap_or(&assembly.parts);
        for part in visible_parts {
            let part_diam = if part.local_aabb.is_finite() && !part.local_aabb.is_empty() {
                part.local_aabb.diameter() * scale_max * xform_max_scale(part.local_xform)
            } else {
                assembly.local_aabb.diameter() * scale_max
            };
            let proj_px = view.project_diameter_px(part_diam, dist);
            if min_px > 0.0 && part.tier != DetailTier::Macro && proj_px < min_px {
                continue;
            }
            let lod = part
                .lods
                .select_lod_with_config(dist, proj_px, prev_lod, &lod_cfg);
            if recorded_lod.is_none()
                && part.lods.num_levels() > 1
                && lod_cfg.enabled
                && lod_cfg.force_lod.is_none()
            {
                recorded_lod = Some(lod);
            }
            let active_proxy = part.active_proxy(lod);
            let compounds = part.lods.compounds_for_lod(lod);
            if active_proxy.is_none() && compounds.is_empty() {
                continue;
            }
            if let Some(proxy) = active_proxy {
                let part_world = inst.xform * proxy.box_local_xform;
                let part_prev = inst
                    .prev_xform
                    .map_or(part_world, |prev| prev * proxy.box_local_xform);
                let mat_ov = inst
                    .mat_override
                    .or(part.mat_override)
                    .or(proxy.dominant_mat)
                    .map_or(MAT_OVERRIDE_NONE, |m| m.0 as u32);
                if out_insts.len() < max_emit {
                    out_insts.push(GpuInstance {
                        xform: part_world.0,
                        compound_id: proxy.compound_id.0,
                        user_id: inst.user_id,
                        mat_override: mat_ov,
                        flags: packed_flags,
                    });
                    out_prev.push(part_prev.0);
                }
                continue;
            }
            let part_world = inst.xform * part.local_xform;
            let part_prev = inst
                .prev_xform
                .map_or(part_world, |prev| prev * part.local_xform);
            let mat_ov = inst
                .mat_override
                .or(part.mat_override)
                .map_or(MAT_OVERRIDE_NONE, |m| m.0 as u32);
            for &cid in compounds {
                if out_insts.len() >= max_emit {
                    continue;
                }
                out_insts.push(GpuInstance {
                    xform: part_world.0,
                    compound_id: cid.0,
                    user_id: inst.user_id,
                    mat_override: mat_ov,
                    flags: packed_flags,
                });
                out_prev.push(part_prev.0);
            }
        }
        if let Some(lod) = recorded_lod {
            *prev_lod_slot = u8::try_from(lod).unwrap_or(PREV_LOD_NONE - 1);
        } else if !lod_cfg.enabled || lod_cfg.force_lod.is_some() {
            // Same as upstream `emit_assembly_instance`: disabled or forced LODs drop the
            // hysteresis history so re-enabling auto LOD starts fresh.
            *prev_lod_slot = PREV_LOD_NONE;
        }
    }
}

/// Builds a single cached OptiX custom-primitive GAS for `comp`, selecting Multi-AABB hardware BVH
/// (`M` tight primitive AABBs + `primitive_offset = first_prim`) for eligible `PURE_UNION`
/// compounds, `primitive_offset = prim_idx` for `SINGLE_PRIM` compounds, and a 1-AABB GAS for
/// general RPN compounds, with `compact: false` and `GeometryFlags::DISABLE_ANYHIT`.
///
/// Multi-AABB boxes go through `multi`, one persistent device buffer per primitive count (the
/// GAS input length is the buffer length), so scene and road-mode switches reuse them instead of
/// allocating, synchronizing and freeing one buffer per compound.
fn build_compound_gas(
    optix_ctx: &Arc<DeviceContext>,
    stream: &Arc<Stream>,
    scratch: &mut Scratch,
    aabb_staging: &DeviceBuffer<Aabb>,
    multi: &mut MultiAabbStaging,
    lib: &CompiledLibrary,
    comp: &GpuCompound,
) -> Result<(Arc<Accel>, u32), String> {
    let build_opts = csg_gas_build_opts();
    let plan = compound_gas_plan(lib, comp);
    let sbt_off = plan.sbt_offset();
    let gas = match plan {
        CompoundGasPlan::MultiAabbUnion { first_prim, count } => {
            let MultiAabbStaging {
                host: multi_aabb_host,
                device: multi_aabb_staging,
            } = multi;
            multi_aabb_host.clear();
            multi_aabb_host.reserve(count);
            let p0 = first_prim as usize;
            for prim in &lib.prims[p0..p0 + count] {
                multi_aabb_host.push(sanitized_prim_aabb(prim));
            }
            let multi_buf: &DeviceBuffer<Aabb> = match multi_aabb_staging.entry(count) {
                std::collections::hash_map::Entry::Occupied(slot) => slot.into_mut(),
                std::collections::hash_map::Entry::Vacant(slot) => {
                    slot.insert(DeviceBuffer::<Aabb>::zeroed(stream, count).map_err(|e| {
                        format!("CSG multi-AABB staging alloc ({count} prims) failed: {e}")
                    })?)
                }
            };
            let box_bytes = count * std::mem::size_of::<Aabb>();
            // SAFETY: `multi_buf` holds exactly `count` `Aabb`s (`box_bytes` bytes) on `stream` and
            // `multi_aabb_host` holds `count` initialized `Aabb`s; CUDA stages the pageable host
            // copy before returning, and the previous GAS build reading `multi_buf` is ordered
            // before this copy on the same stream.
            let h2d = unsafe {
                cu::cuMemcpyHtoDAsync_v2(
                    multi_buf.cu_deviceptr(),
                    multi_aabb_host.as_ptr().cast(),
                    box_bytes,
                    stream.cu_stream(),
                )
            };
            if h2d != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!(
                    "CSG multi-AABB H2D copy ({count} prims) failed ({h2d:?})"
                ));
            }
            let input = CustomPrimitives {
                aabbs: Keys::One(multi_buf),
                flags: &[GeometryFlags::DISABLE_ANYHIT],
                sbt_offsets: None,
                primitive_offset: first_prim,
            };
            Accel::custom(optix_ctx, stream, scratch, &[input], build_opts)
                .map_err(|e| format!("CSG multi-AABB GAS build failed: {e}"))?
        }
        CompoundGasPlan::SinglePrim { prim_idx } => {
            let aabb = sanitized_compound_aabb(comp);
            let box_bytes = std::mem::size_of::<Aabb>();
            // SAFETY: `aabb_staging` holds 1 `Aabb` (`24 B` plain `f32`) on `stream`; CUDA stages
            // the pageable host copy before returning.
            let h2d = unsafe {
                cu::cuMemcpyHtoDAsync_v2(
                    aabb_staging.cu_deviceptr(),
                    (&raw const aabb).cast(),
                    box_bytes,
                    stream.cu_stream(),
                )
            };
            if h2d != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!("CSG single-prim AABB H2D copy failed ({h2d:?})"));
            }
            let input = CustomPrimitives {
                aabbs: Keys::One(aabb_staging),
                flags: &[GeometryFlags::DISABLE_ANYHIT],
                sbt_offsets: None,
                primitive_offset: prim_idx,
            };
            Accel::custom(optix_ctx, stream, scratch, &[input], build_opts)
                .map_err(|e| format!("CSG single-prim GAS build failed: {e}"))?
        }
        CompoundGasPlan::GeneralRpn => {
            let aabb = sanitized_compound_aabb(comp);
            let box_bytes = std::mem::size_of::<Aabb>();
            // SAFETY: `aabb_staging` holds 1 `Aabb` (`24 B` plain `f32`) on `stream`; CUDA stages
            // the pageable host copy before returning.
            let h2d = unsafe {
                cu::cuMemcpyHtoDAsync_v2(
                    aabb_staging.cu_deviceptr(),
                    (&raw const aabb).cast(),
                    box_bytes,
                    stream.cu_stream(),
                )
            };
            if h2d != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!("CSG compound AABB H2D copy failed ({h2d:?})"));
            }
            let input = CustomPrimitives {
                aabbs: Keys::One(aabb_staging),
                flags: &[GeometryFlags::DISABLE_ANYHIT],
                sbt_offsets: None,
                primitive_offset: 0,
            };
            Accel::custom(optix_ctx, stream, scratch, &[input], build_opts)
                .map_err(|e| format!("CSG compound GAS build failed: {e}"))?
        }
    };
    Ok((gas, sbt_off))
}

/// Converts `nv-demo` [`CameraProjection`] to [`SdgProjection`].
#[inline]
pub const fn to_sdg_projection(proj: CameraProjection) -> SdgProjection {
    match proj {
        CameraProjection::Pinhole => SdgProjection::Pinhole,
        CameraProjection::Fisheye180 => SdgProjection::Fisheye180,
        CameraProjection::Equirect360 => SdgProjection::Equirect360,
        CameraProjection::SlitCam => SdgProjection::SlitCam,
    }
}

/// Owns the active `SceneRegistry`, compiled `SceneBuilt` (world, traffic, crowd, hints),
/// bounded `SceneBuilt` LRU cache, batched per-compound OptiX GAS cache, and flat GPU buffers.
pub struct CsgGpuHost {
    registry: SceneRegistry,
    scene_key: CsgSceneKey,
    active_reg_idx: Option<usize>,
    scene: Option<SceneBuilt>,
    /// Most-recently-used `SceneBuilt` cache for instant scene / road-mode switches.
    scene_cache: VecDeque<(CsgSceneKey, SceneBuilt)>,
    /// True if `spawn_crowd_stress` mutated `self.scene` (don't cache mutated stress grids).
    scene_mutated_stress: bool,
    crowd_frozen: bool,
    pending_metadata: Option<SceneLoadedMetadata>,
    packed: PackedGpuScene,
    /// Per-compound cached OptiX custom-primitive GASes (indexed by `compound_id`).
    compound_gases: Vec<Arc<Accel>>,
    /// Per-compound SBT hit-group offset (`CSG_SBT_PRIM_OFFSET` = 10 or `CSG_SBT_OFFSET` = 8).
    compound_sbt_offsets: Vec<u32>,
    compounds_buf: GrowDeviceBuffer<GpuCompound>,
    ops_buf: GrowDeviceBuffer<GpuOp>,
    prims_buf: GrowDeviceBuffer<GpuPrim>,
    aux_planes_buf: GrowDeviceBuffer<[f32; 4]>,
    guard_aabbs_buf: GrowDeviceBuffer<GpuAabb>,
    repeats_buf: GrowDeviceBuffer<GpuRepeat>,
    aabb_staging_buf: Option<DeviceBuffer<Aabb>>,
    multi_aabb: MultiAabbStaging,
    materials_buf: GrowDeviceBuffer<GpuMaterial>,
    instances_buf: GrowDeviceBuffer<GpuInstance>,
    prev_xforms_buf: GrowDeviceBuffer<[f32; 12]>,
    pinned_staging: Option<PinnedStagingBuffer>,
    pinned_prev_staging: Option<PinnedStagingBuffer>,
    prev_packed_len: usize,
    prev_dyn_sig: Vec<(u32, u32)>,
    prev_non_crowd_dyn_count: usize,
    crowd_at_dyn_tail: bool,
    dyn_packed_scratch: Vec<GpuInstance>,
    dyn_prev_scratch: Vec<[f32; 12]>,
    dyn_prev_lods: Vec<u8>,
    static_gpu_bytes: usize,
    last_mat_state: (u32, u32),
    last_pack_cam: [f32; 3],
    last_pack_fwd: [f32; 3],
    last_pack_cam_params: Option<CsgCameraParams>,
    last_pack_lod_key: Option<LodConfigKey>,
    last_pack_omni: bool,
    last_traffic_bits: u32,
    ias_dirty: bool,
    traffic_dirty: bool,
    stats: CsgStats,
    active_possess: Option<PossessInfo>,
    next_vehicle_cursor: usize,
}

impl Default for CsgGpuHost {
    fn default() -> Self {
        Self::new()
    }
}

impl CsgGpuHost {
    /// Creates an empty host in `Off (Classic)` mode backed by [`csg_scenes::default_scene_registry`].
    pub fn new() -> Self {
        Self {
            registry: csg_scenes::default_scene_registry(),
            scene_key: CsgSceneKey::default(),
            active_reg_idx: None,
            scene: None,
            scene_cache: VecDeque::with_capacity(SCENE_LRU_MAX_ENTRIES),
            scene_mutated_stress: false,
            crowd_frozen: false,
            pending_metadata: None,
            packed: PackedGpuScene::new(),
            compound_gases: Vec::new(),
            compound_sbt_offsets: Vec::new(),
            compounds_buf: GrowDeviceBuffer::default(),
            ops_buf: GrowDeviceBuffer::default(),
            prims_buf: GrowDeviceBuffer::default(),
            aux_planes_buf: GrowDeviceBuffer::default(),
            guard_aabbs_buf: GrowDeviceBuffer::default(),
            repeats_buf: GrowDeviceBuffer::default(),
            aabb_staging_buf: None,
            multi_aabb: MultiAabbStaging {
                host: Vec::with_capacity(64),
                device: HashMap::new(),
            },
            materials_buf: GrowDeviceBuffer::default(),
            instances_buf: GrowDeviceBuffer::default(),
            prev_xforms_buf: GrowDeviceBuffer::default(),
            pinned_staging: None,
            pinned_prev_staging: None,
            prev_packed_len: 0,
            prev_dyn_sig: Vec::new(),
            prev_non_crowd_dyn_count: 0,
            crowd_at_dyn_tail: false,
            dyn_packed_scratch: Vec::with_capacity(1024),
            dyn_prev_scratch: Vec::with_capacity(1024),
            dyn_prev_lods: Vec::with_capacity(1024),
            static_gpu_bytes: 0,
            last_mat_state: (u32::MAX, u32::MAX),
            last_pack_cam: [f32::NAN; 3],
            last_pack_fwd: [f32::NAN; 3],
            last_pack_cam_params: None,
            last_pack_lod_key: None,
            last_pack_omni: false,
            last_traffic_bits: u32::MAX,
            ias_dirty: false,
            traffic_dirty: false,
            stats: CsgStats::default(),
            active_possess: None,
            next_vehicle_cursor: 0,
        }
    }

    /// Returns `true` when any registered CSG scene is loaded and active.
    pub fn is_active(&self) -> bool {
        self.scene_key.scene_idx != 0 && self.scene.is_some()
    }

    /// Returns the currently loaded [`CsgSceneKey`].
    pub fn scene_key(&self) -> CsgSceneKey {
        self.scene_key
    }

    /// Returns `true` when the active scene has dynamic moving traffic actors (legacy or SoA).
    pub fn has_traffic(&self) -> bool {
        self.scene.as_ref().is_some_and(|s| !s.traffic.is_empty())
    }

    /// Returns `true` when the active scene has articulated crowd actors (pedestrians/animals).
    pub fn has_crowd(&self) -> bool {
        self.scene
            .as_ref()
            .is_some_and(|s| s.crowd.actor_count() > 0)
    }

    /// Returns the latest CSG telemetry for the UI.
    pub fn stats(&self) -> CsgStats {
        self.stats
    }

    /// Returns `true` if freshly built scene metadata is waiting to be sent to the UI thread.
    pub fn has_pending_metadata(&self) -> bool {
        self.pending_metadata.is_some()
    }

    /// Takes freshly built scene metadata (viewpoints + collision-verified rails) once per rebuild.
    pub fn take_loaded_metadata(&mut self) -> Option<SceneLoadedMetadata> {
        self.pending_metadata.take()
    }

    /// Synthesizes a fresh deterministic, exact-CSG collision-verified random rail for the active scene.
    pub fn generate_random_rail(&self, seed: u64) -> Result<RailSpec, String> {
        let Some(ref scene) = self.scene else {
            return Err("Load a CSG scene before generating a random rail".to_string());
        };
        let cfg = RailGenConfig {
            seed,
            ..RailGenConfig::default()
        };
        generate_random_rail(&scene.world, &scene.hints, seed, &cfg)
            .map_err(|e| format!("Random rail gen failed: {e}"))
    }

    /// Synthesizes up to 4 distinct collision-verified multi-landmark & street-network smart rail
    /// candidates using the active scene's `world`, `hints`, and curated `viewpoints`.
    pub fn build_smart_rail_candidates(&self, seed: u64) -> Result<Vec<RailSpec>, String> {
        let Some(ref scene) = self.scene else {
            return Err("Load a CSG scene before running Smart Rail".to_string());
        };
        let cfg = RailGenConfig {
            seed,
            ..RailGenConfig::default()
        };
        let rails = csg_scene::rail_gen::build_smart_rail_candidates(
            &scene.world,
            &scene.hints,
            &scene.viewpoints,
            seed,
            &cfg,
        );
        if rails.is_empty() {
            let single = csg_scene::rail_gen::build_smart_rail(
                &scene.world,
                &scene.hints,
                &scene.viewpoints,
                seed,
                &cfg,
            )
            .map_err(|e| format!("Smart rail gen failed: {e}"))?;
            Ok(vec![single])
        } else {
            Ok(rails)
        }
    }

    /// Returns references to the active `WorldScene` and `SceneHints`, if a CSG scene is loaded.
    pub fn world_and_hints(&self) -> Option<(&WorldScene, &SceneHints)> {
        self.scene.as_ref().map(|s| (&s.world, &s.hints))
    }

    /// Returns a reference to the active articulated crowd simulation, if a CSG scene is loaded.
    pub fn active_crowd(&self) -> Option<&CrowdSim> {
        self.scene.as_ref().map(|s| &s.crowd)
    }

    /// Returns a mutable reference to the active [`SceneBuilt`] for GPU crowd stepping/syncing.
    pub fn active_scene_mut(&mut self) -> Option<&mut SceneBuilt> {
        self.scene.as_mut()
    }

    /// Fills `out` with the OptiX `OptixTraversableHandle` (`u64`) for every cached compound GAS.
    pub fn fill_compound_gas_handles(&self, out: &mut Vec<u64>) {
        out.clear();
        out.reserve(self.compound_gases.len());
        for gas in &self.compound_gases {
            out.push(gas.handle());
        }
    }

    /// Returns the cached SBT hit-group offset (`CSG_SBT_PRIM_OFFSET` = 10 or `CSG_SBT_OFFSET`
    /// = 8) for every compound that has a GAS.
    pub fn compound_sbt_offsets(&self) -> &[u32] {
        &self.compound_sbt_offsets
    }

    /// Computes the exact 1:1 mapping `bone_slot_map[gidx] -> packed_instance_slot` (or `u32::MAX`
    /// when a bone is frustum/LOD-culled from `self.packed`) in O(`dyn_count + total_bones`) time.
    ///
    /// Uses `(user_id, compound_id, canonical_xform_bits)` matching along the strictly monotonic
    /// ascending `crowd_inst_ids` order so partial frustum culling, `-0.0` vs `+0.0` float signs,
    /// or identical adjacent bone materials can never cross-wire slots.
    pub fn fill_crowd_bone_slot_map(&self, out: &mut Vec<u32>) {
        out.clear();
        let Some(ref scene) = self.scene else {
            return;
        };
        let n_bones = scene.crowd.bone_count();
        if n_bones == 0 || scene.crowd_inst_ids.len() < n_bones {
            return;
        }
        out.resize(n_bones, u32::MAX);

        let slice = self.packed.instances_slice();
        let dyn_count = slice
            .iter()
            .take_while(|inst| (inst.flags & inst_flags::DYNAMIC) != 0)
            .count();

        let mut gidx = 0usize;
        for (slot, gpu_inst) in slice[..dyn_count].iter().enumerate() {
            // The packer derives this bit from the world's kind tag; `user_id` ranges are not
            // guaranteed unique across generators.
            if (gpu_inst.flags & inst_flags::CROWD_BONE) == 0 {
                continue;
            }
            let target_bits = xform_to_bits(gpu_inst.xform);
            let saved_gidx = gidx;
            let mut matched = false;
            while gidx < n_bones {
                let bone_inst_id = scene.crowd_inst_ids[gidx];
                let Some(world_inst) = scene.world.dynamic_instance(bone_inst_id) else {
                    gidx += 1;
                    continue;
                };
                if world_inst.user_id == gpu_inst.user_id
                    && scene.crowd.bones[gidx].compound_id == gpu_inst.compound_id
                    && xform_to_bits(world_inst.xform.0) == target_bits
                {
                    out[gidx] = slot as u32;
                    gidx += 1;
                    matched = true;
                    break;
                }
                gidx += 1;
            }
            if !matched {
                gidx = saved_gidx;
            }
        }
    }

    /// Returns `true` if calling [`Self::sync`] with `(key, cam)` will trigger a full frustum
    /// repack (`camera_repack_needed` or scene rebuild).
    pub fn will_camera_repack(&self, key: CsgSceneKey, cam: CsgCameraParams) -> bool {
        if key != self.scene_key {
            return true;
        }
        if key.scene_idx == 0 || self.scene.is_none() {
            return self.ias_dirty;
        }
        self.camera_repack_needed(cam)
    }

    /// The repack predicate shared by [`Self::will_camera_repack`] and [`Self::sync`]: a pending
    /// `ias_dirty`, a viewport / FOV / LOD / projection change, `> 1 m` of camera travel or
    /// `> ~3.1°` of rotation (perspective only). It stays inside the `1.35x` padded frustum margin
    /// so objects near viewport edges never pop out prematurely.
    fn camera_repack_needed(&self, cam: CsgCameraParams) -> bool {
        if self.ias_dirty {
            return true;
        }
        let cam_dist_sq = (cam.eye[0] - self.last_pack_cam[0]).powi(2)
            + (cam.eye[1] - self.last_pack_cam[1]).powi(2)
            + (cam.eye[2] - self.last_pack_cam[2]).powi(2);
        let fwd_dot = cam.fwd[0] * self.last_pack_fwd[0]
            + cam.fwd[1] * self.last_pack_fwd[1]
            + cam.fwd[2] * self.last_pack_fwd[2];
        let lod_key = LodConfigKey::from_config(cam.lod_config);
        let viewport_or_fov_changed = self.last_pack_cam_params.is_none_or(|prev| {
            prev.viewport_w != cam.viewport_w
                || prev.viewport_h != cam.viewport_h
                || prev.vfov_rad.to_bits() != cam.vfov_rad.to_bits()
        });
        viewport_or_fov_changed
            || self.last_pack_lod_key != Some(lod_key)
            || cam.omnidirectional != self.last_pack_omni
            || !cam_dist_sq.is_finite()
            || cam_dist_sq > 1.0
            || (!cam.omnidirectional && (!fwd_dot.is_finite() || fwd_dot < 0.9985))
    }

    /// Returns `true` if calling [`Self::sync`] with `(key, cam)` will pack and upload instance
    /// buffers from CPU (`FullRepack` or `TrafficOnly`). Traffic moves only through
    /// [`Self::step_dynamic`], which sets `traffic_dirty`, so it must run first in the frame.
    pub fn will_upload_instances(&self, key: CsgSceneKey, cam: CsgCameraParams) -> bool {
        if self.will_camera_repack(key, cam) {
            return true;
        }
        key.scene_idx != 0 && self.scene.is_some() && self.traffic_dirty
    }

    /// Applies `freeze_pose` transitions to `scene.crowd` and dynamic traffic motion history.
    /// Returns `true` if crowd freeze intents changed and must be re-uploaded to CUDA.
    ///
    /// The single implementation of the freeze transition; [`Self::step_dynamic`] calls it too.
    pub fn sync_freeze_pose(&mut self, freeze_pose: bool) -> bool {
        let Some(ref mut scene) = self.scene else {
            return false;
        };
        if freeze_pose == self.crowd_frozen {
            return false;
        }
        scene.set_crowd_frozen(freeze_pose);
        self.crowd_frozen = freeze_pose;
        if freeze_pose {
            scene.reset_crowd_motion_history();
            for inst in scene.world.dynamic_instances_mut() {
                inst.prev_xform = Some(inst.xform);
            }
            self.traffic_dirty = true;
        }
        true
    }

    /// Injects a balance poke impulse into either the nearest crowd actor to `origin` or all
    /// crowd actors along `forward`, returning a concise human-readable status message.
    pub fn poke_crowd(
        &mut self,
        origin: [f32; 3],
        forward: [f32; 3],
        impulse_mag: f32,
        all: bool,
    ) -> Result<String, String> {
        let Some(ref mut scene) = self.scene else {
            return Err("Load a CSG scene before poking crowd actors".to_string());
        };
        let count = scene.crowd.actor_count();
        if count == 0 {
            return Err("Active scene has no crowd actors to poke".to_string());
        }
        let mag = if impulse_mag.is_finite() {
            impulse_mag.clamp(0.25, 8.0)
        } else {
            1.8
        };
        let mut dir = Vec3::new(forward[0], 0.0, forward[2]);
        if dir.len_sq() < 1e-6 {
            dir = Vec3::Z;
        } else {
            dir = dir.norm();
        }
        let impulse = Vec3::new(dir.x * mag, 0.15 * mag, dir.z * mag);
        if all {
            scene.poke_crowd_all(impulse);
            Ok(format!("Poked all {count} crowd actors ({mag:.1} m/s)"))
        } else {
            let pos = Vec3::new(origin[0], origin[1], origin[2]);
            match scene.poke_crowd_nearest(pos, impulse) {
                Some(idx) => Ok(format!("Poked crowd actor #{idx} ({mag:.1} m/s)")),
                None => Err("Failed to locate nearest crowd actor to poke".to_string()),
            }
        }
    }

    /// Grows the active scene's crowd up to `target_count` walking articulated actors centered at
    /// `origin`, compiling and uploading any newly-registered 11-GAS `ActorGasBank` compounds
    /// automatically (`0` extra GASes if the scene already had an actor bank).
    pub fn spawn_crowd_stress(
        &mut self,
        optix_ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        target_count: usize,
        origin: [f32; 3],
    ) -> Result<usize, String> {
        let Some(ref mut scene) = self.scene else {
            return Err("Load a CSG scene before spawning a crowd stress grid".to_string());
        };
        let old_compound_count = scene.world.library.compounds.len();
        let added = scene
            .spawn_crowd_stress(target_count, Vec3::new(origin[0], origin[1], origin[2]))
            .map_err(|e| format!("Crowd stress spawn failed (GAS budget): {e}"))?;

        let new_compound_count = scene.world.library.compounds.len();
        // The scene is mutated from here on, so the bookkeeping below runs even when this GPU
        // part fails. New GASes are committed only all together; compounds left without one
        // become invisible placeholders that keep packed and OptiX slots aligned.
        let gpu_res = (|| -> Result<(), String> {
            if new_compound_count <= old_compound_count {
                return Ok(());
            }
            // Re-upload compiled library tables and build GASes only for newly added actor compounds.
            let lib = &scene.world.library;
            self.compounds_buf.upload(stream, &lib.compounds)?;
            self.ops_buf.upload(stream, &lib.ops)?;
            self.prims_buf.upload(stream, &lib.prims)?;
            self.aux_planes_buf.upload(stream, &lib.aux_planes)?;
            self.guard_aabbs_buf.upload(stream, &lib.guard_aabbs)?;
            self.repeats_buf.upload(stream, &lib.repeats)?;
            self.materials_buf
                .upload(stream, scene.world.materials.as_slice())?;
            self.static_gpu_bytes = self.compounds_buf.active_bytes()
                + self.ops_buf.active_bytes()
                + self.prims_buf.active_bytes()
                + self.aux_planes_buf.active_bytes()
                + self.guard_aabbs_buf.active_bytes()
                + self.repeats_buf.active_bytes();

            if self.aabb_staging_buf.is_none() {
                let staging = DeviceBuffer::<Aabb>::zeroed(stream, 1)
                    .map_err(|e| format!("CSG AABB staging alloc failed: {e}"))?;
                self.aabb_staging_buf = Some(staging);
            }
            let aabb_staging = self
                .aabb_staging_buf
                .as_ref()
                .ok_or_else(|| "CSG AABB staging buffer missing".to_string())?;

            let new_count = new_compound_count - old_compound_count;
            let mut new_gases = Vec::with_capacity(new_count);
            let mut new_sbt_offsets = Vec::with_capacity(new_count);
            for c in &lib.compounds[old_compound_count..new_compound_count] {
                let (gas, sbt_off) = build_compound_gas(
                    optix_ctx,
                    stream,
                    scratch,
                    aabb_staging,
                    &mut self.multi_aabb,
                    lib,
                    c,
                )?;
                new_gases.push(gas);
                new_sbt_offsets.push(sbt_off);
            }
            stream
                .synchronize()
                .map_err(|e| format!("CSG stress GAS sync failed: {e}"))?;
            self.compound_gases.extend(new_gases);
            self.compound_sbt_offsets.extend(new_sbt_offsets);
            Ok(())
        })();

        if added > 0 {
            self.scene_mutated_stress = true;
        }
        self.packed
            .reserve_instances(scene.world.total_instance_count().max(256));
        self.ias_dirty = true;
        self.traffic_dirty = true;
        gpu_res.map(|()| added)
    }

    /// Returns the active registered scene identifier (`"gundo_osm"`, `"inspector"`, `"classic"`, etc.).
    pub fn active_scene_id(&self) -> String {
        self.active_reg_idx
            .and_then(|idx| self.registry.get_by_index(idx))
            .map(|m| m.descriptor().id.to_string())
            .unwrap_or_else(|| {
                if self.scene_key.scene_idx == 0 {
                    "classic".to_string()
                } else {
                    "unknown".to_string()
                }
            })
    }

    /// Returns the number of articulated crowd actors in the active CSG scene.
    pub fn crowd_actor_count(&self) -> usize {
        self.scene
            .as_ref()
            .map_or(0, |s| s.crowd.actor_count())
    }

    /// Returns the currently packed GPU instance staging buffer.
    pub fn packed(&self) -> &PackedGpuScene {
        &self.packed
    }

    /// Returns `true` while `self.packed` (and the instance buffers uploaded from it) describes
    /// the same instances as the last IAS the renderer built. A repack outside the frame loop
    /// ([`Self::prepare_full_scene_for_probe`], scene rebuild, crowd stress spawn) sets
    /// `ias_dirty` until the next [`Self::sync`] and IAS rebuild.
    pub fn instances_match_ias(&self) -> bool {
        !self.ias_dirty
    }

    /// Packs the full scene omnidirectionally into `self.packed` for city-wide view scoring and,
    /// when `upload_gpu` is set (OptiX probing), uploads `self.instances_buf` from it; the CPU
    /// and WGPU probers read only the CPU pack. Marks `ias_dirty` so the next render frame
    /// restores frustum culling.
    pub fn prepare_full_scene_for_probe(
        &mut self,
        stream: &Arc<Stream>,
        upload_gpu: bool,
    ) -> Result<(), String> {
        let Some(ref mut scene) = self.scene else {
            return Err("Load a CSG scene before running Smart View".to_string());
        };
        let cam_pos = if self.last_pack_cam.iter().all(|v| v.is_finite()) {
            Vec3::new(
                self.last_pack_cam[0],
                self.last_pack_cam[1],
                self.last_pack_cam[2],
            )
        } else {
            Vec3::ZERO
        };
        scene.world.pack_all(cam_pos, &mut self.packed);
        self.ias_dirty = true;
        if upload_gpu {
            self.instances_buf
                .upload(stream, self.packed.instances_slice())?;
            self.prev_xforms_buf
                .upload(stream, self.packed.motion_prev_xforms_slice())?;
        }
        Ok(())
    }

    /// Casts an exact CSG ray against live dynamic + static instances (including articulated crowd
    /// actors) and enters `Possess` mode.
    pub fn pick_possess(&mut self, origin: [f32; 3], dir: [f32; 3]) -> Result<PossessInfo, String> {
        let Some(ref scene) = self.scene else {
            return Err("Load a CSG scene before picking an object to possess".to_string());
        };
        let world = &scene.world;
        let d = Vec3::new(dir[0], dir[1], dir[2]).norm();
        let ray = Ray::new(Vec3::new(origin[0], origin[1], origin[2]), d, 0.05, 1500.0);
        let Some((inst_id, _t, mounts)) =
            pick_possessable_instance_with_crowd(world, &scene.crowd, ray)
        else {
            return Err("No possessable object hit under cursor".to_string());
        };
        let assembly_name = if let Some(actor_idx) =
            find_crowd_actor_for_inst(world, &scene.crowd, inst_id)
        {
            crowd_actor_display_name(&scene.crowd, actor_idx)
        } else {
            lookup_instance_and_assembly(world, inst_id)
                .map(|(_, asm)| asm.name.clone())
                .unwrap_or_else(|| format!("Object #{inst_id}"))
        };
        let info = PossessInfo {
            inst_id,
            assembly_name,
            mounts,
            mount_idx: 0,
            is_dynamic: is_dynamic_inst_id(inst_id),
        };
        self.active_possess = Some(info.clone());
        Ok(info)
    }

    /// Possesses the next dynamic vehicle (prioritizing `Robotaxi`), articulated crowd actor, or
    /// mounted scene landmark.
    pub fn possess_next_vehicle(&mut self) -> Result<PossessInfo, String> {
        let Some(ref scene) = self.scene else {
            return Err("Load a CSG scene before possessing a vehicle".to_string());
        };
        let world = &scene.world;
        let mut targets = list_possess_targets_with_crowd(world, &scene.crowd, &scene.crowd_inst_ids);
        if targets.is_empty() {
            return Err("Active scene has no mounted vehicles, actors, or landmarks".to_string());
        }
        // Sort: dynamic Robotaxis first, then other dynamic vehicles & crowd actors, then static landmarks.
        targets.sort_by_key(|t| {
            let dyn_rank = if is_dynamic_inst_id(t.inst_id) { 0u8 } else { 1u8 };
            let robo_rank = if t.assembly_name.to_ascii_lowercase().contains("robotaxi") {
                0u8
            } else {
                1u8
            };
            (dyn_rank, robo_rank, t.inst_id)
        });
        let idx = self.next_vehicle_cursor % targets.len();
        self.next_vehicle_cursor = self.next_vehicle_cursor.wrapping_add(1);
        let chosen = targets.swap_remove(idx);
        let info = PossessInfo {
            inst_id: chosen.inst_id,
            assembly_name: chosen.assembly_name,
            mounts: chosen.mounts,
            mount_idx: 0,
            is_dynamic: is_dynamic_inst_id(chosen.inst_id),
        };
        self.active_possess = Some(info.clone());
        Ok(info)
    }

    /// Switches the active mount index on the currently possessed body.
    pub fn set_possess_mount(&mut self, mount_idx: usize) -> Option<PossessInfo> {
        let info = self.active_possess.as_mut()?;
        if info.mounts.is_empty() {
            return None;
        }
        info.mount_idx = mount_idx % info.mounts.len();
        Some(info.clone())
    }

    /// Detaches from any currently possessed body back to free camera.
    pub fn eject_possess(&mut self) -> bool {
        self.active_possess.take().is_some()
    }

    /// Returns `true` while actively possessing an object, moving vehicle, or articulated crowd actor.
    pub fn is_possessing(&self) -> bool {
        self.active_possess.is_some()
    }

    /// Returns `true` when the currently possessed body is an articulated [`CrowdSim`] actor.
    pub fn is_possessing_crowd(&self) -> bool {
        let (Some(scene), Some(info)) = (self.scene.as_ref(), self.active_possess.as_ref()) else {
            return false;
        };
        find_crowd_actor_for_inst(&scene.world, &scene.crowd, info.inst_id).is_some()
    }

    /// Resolves the live world-space camera pose of the possessed body (tracking moving traffic
    /// and articulated crowd actors).
    pub fn resolve_active_possess(
        &self,
        free_yaw: f32,
        free_pitch: f32,
        auto_level: bool,
    ) -> Option<CsgCameraPose> {
        let scene = self.scene.as_ref()?;
        let info = self.active_possess.as_ref()?;
        resolve_possess_pose_with_crowd(
            &scene.world,
            &scene.crowd,
            info.inst_id,
            info.mount_idx,
            free_yaw,
            free_pitch,
            Some(auto_level),
        )
    }

    /// Returns the flat device pointers for `LaunchParams`.
    pub fn device_pointers(&self) -> CsgDevicePointers {
        if !self.is_active() {
            return CsgDevicePointers::default();
        }
        let Some(ref scene) = self.scene else {
            return CsgDevicePointers::default();
        };
        let world = &scene.world;
        CsgDevicePointers {
            compounds: self.compounds_buf.device_ptr(),
            ops: self.ops_buf.device_ptr(),
            prims: self.prims_buf.device_ptr(),
            aux_planes: self.aux_planes_buf.device_ptr(),
            guard_aabbs: self.guard_aabbs_buf.device_ptr(),
            repeats: self.repeats_buf.device_ptr(),
            materials: self.materials_buf.device_ptr(),
            instances: self.instances_buf.device_ptr(),
            prev_xforms: self.prev_xforms_buf.device_ptr(),
            // Never more than were uploaded: after a failed upload the buffer reports 0.
            instance_count: self.packed.len().min(self.instances_buf.len_elems) as u32,
            material_count: world.materials.len() as u32,
            compound_count: world.library.compounds.len() as u32,
        }
    }

    /// Advances dynamic 3D Frenet traffic and articulated crowd actors in-place before resolving
    /// possessed vehicle mounts so cameras lock to exact current-frame transforms.
    ///
    /// Applies [`CrowdLodParams`] (`enabled`, `force_lod`, `lod_bias`, `lod1/lod2/cull`) for the
    /// step while keeping the scene's nominal `1.0x` thresholds unmodified.
    /// Respects `freeze_pose` (pausing both traffic and crowd stepping so progressive path tracing
    /// can accumulate cleanly for Synthetic Data Generation captures).
    pub fn step_dynamic(
        &mut self,
        dt: f32,
        time_s: f32,
        cam_eye: [f32; 3],
        crowd_lod: CrowdLodParams,
        freeze_pose: bool,
        step_traffic: bool,
    ) -> bool {
        // Its "intents changed" result drives the CUDA re-upload; the renderer reads it from its
        // own `sync_freeze_pose` call earlier in the frame, which makes this one a no-op there.
        self.sync_freeze_pose(freeze_pose);
        let Some(ref mut scene) = self.scene else {
            return false;
        };
        if freeze_pose {
            return false;
        }

        let mut moved = false;
        let traffic_bits = time_s.to_bits();
        if step_traffic
            && !scene.traffic.is_empty()
            && traffic_bits != self.last_traffic_bits
            && let Some(reg_idx) = self.active_reg_idx
            && let Some(module) = self.registry.get_by_index(reg_idx)
        {
            module.step_dynamic(&mut scene.world, &scene.traffic, time_s);
            self.last_traffic_bits = traffic_bits;
            self.traffic_dirty = true;
            moved = true;
        }

        if scene.crowd.actor_count() > 0 && dt > 0.0 && dt.is_finite() {
            let orig = (
                scene.crowd.lod1_dist_m,
                scene.crowd.lod2_dist_m,
                scene.crowd.cull_dist_m,
            );
            let (l1, l2, cull) = crowd_lod.resolve_dists();
            scene.crowd.lod1_dist_m = l1;
            scene.crowd.lod2_dist_m = l2;
            scene.crowd.cull_dist_m = cull;
            scene.step_crowd(dt, time_s, Vec3::new(cam_eye[0], cam_eye[1], cam_eye[2]));
            scene.crowd.lod1_dist_m = orig.0;
            scene.crowd.lod2_dist_m = orig.1;
            scene.crowd.cull_dist_m = orig.2;
            self.traffic_dirty = true;
            moved = true;
        }

        moved
    }

    /// Re-evaluates CPU crowd LOD visibility against [`CrowdLodParams`] with zero actor motion
    /// (used when adjusting LOD settings while crowd simulation is paused or frozen).
    pub fn refresh_crowd_lod_cpu(
        &mut self,
        time_s: f32,
        cam_eye: [f32; 3],
        crowd_lod: CrowdLodParams,
    ) -> bool {
        let Some(ref mut scene) = self.scene else {
            return false;
        };
        if scene.crowd.actor_count() == 0 {
            return false;
        }
        let was_frozen = self.crowd_frozen;
        if !was_frozen {
            scene.set_crowd_frozen(true);
        }
        let orig = (
            scene.crowd.lod1_dist_m,
            scene.crowd.lod2_dist_m,
            scene.crowd.cull_dist_m,
        );
        let (l1, l2, cull) = crowd_lod.resolve_dists();
        scene.crowd.lod1_dist_m = l1;
        scene.crowd.lod2_dist_m = l2;
        scene.crowd.cull_dist_m = cull;
        scene.step_crowd(1.0 / 60.0, time_s, Vec3::new(cam_eye[0], cam_eye[1], cam_eye[2]));
        scene.crowd.lod1_dist_m = orig.0;
        scene.crowd.lod2_dist_m = orig.1;
        scene.crowd.cull_dist_m = orig.2;
        if !was_frozen {
            scene.set_crowd_frozen(false);
        }
        scene.reset_crowd_motion_history();
        self.traffic_dirty = true;
        true
    }

    /// Builds the SSOT [`SdgCamera`] matching `cam`, `projection`, and viewport `(width, height)`.
    pub fn build_sdg_camera(
        cam: &CameraFrame,
        projection: CameraProjection,
        width: u32,
        height: u32,
    ) -> SdgCamera {
        let vfov_rad = 2.0 * cam.tan_half_fov.max(0.05).atan();
        SdgCamera {
            projection: to_sdg_projection(projection),
            eye: ActorVec3::from_array(cam.eye),
            forward: ActorVec3::from_array(cam.fwd),
            up: ActorVec3::from_array(cam.up),
            vfov_rad,
            width: width.max(1),
            height: height.max(1),
        }
    }

    /// Extracts exact 2D/3D actor keypoints, tight bone-OBB bounding boxes, and occlusion labels
    /// using an explicit [`RayOccluder`] (CPU `CsgRayOccluder` or OptiX 9 hardware keypoint batch)
    /// and writes them atomically to `path` as pretty-printed JSON.
    pub fn export_sdg_json_with_occluder(
        &self,
        path: &Path,
        sdg_cam: SdgCamera,
        frame_index: u32,
        time_s: f32,
        occluder: &dyn RayOccluder,
    ) -> Result<usize, String> {
        let Some(ref scene) = self.scene else {
            return Err("Load a CSG scene before exporting SDG JSON".to_string());
        };
        let scene_id = self.active_scene_id();
        let actors = extract_sdg_frame(&scene.crowd, &sdg_cam, occluder);
        let actor_count = actors.len();
        let payload = SdgExportFrame {
            scene_id,
            scene_idx: self.scene_key.scene_idx,
            variant_idx: self.scene_key.variant_idx,
            frame_index,
            time_s,
            camera: sdg_cam,
            actors,
        };
        let json = serde_json::to_string_pretty(&payload)
            .map_err(|e| format!("SDG JSON serialize failed: {e}"))?;
        crate::sdg_export::write_bytes_atomic(path, json.as_bytes(), "json.tmp")?;
        Ok(actor_count)
    }

    /// Extracts exact 2D/3D actor keypoints, tight bone-OBB bounding boxes, and analytical CPU
    /// CSG ray-occluded visibility labels via [`csg_actors::extract_sdg_frame`] and writes them
    /// atomically to `path` as pretty-printed JSON. `sdg_cam` comes from
    /// [`Self::build_sdg_camera`], the single source of the export camera.
    pub fn export_sdg_json(
        &self,
        path: &Path,
        sdg_cam: SdgCamera,
        frame_index: u32,
        time_s: f32,
    ) -> Result<usize, String> {
        let Some(ref scene) = self.scene else {
            return Err("Load a CSG scene before exporting SDG JSON".to_string());
        };
        let visible_slice = self.packed.instances_slice();
        if visible_slice.is_empty() {
            let occluder = CsgRayOccluder::new(&scene.world.library, &scene.crowd.bone_outs);
            self.export_sdg_json_with_occluder(path, sdg_cam, frame_index, time_s, &occluder)
        } else {
            let world_insts: Vec<(CompoundId, Mat3x4, u32)> = visible_slice
                .iter()
                .map(|inst| (CompoundId(inst.compound_id), Mat3x4(inst.xform), inst.user_id))
                .collect();
            let occluder = CsgRayOccluder::from_instances(&scene.world.library, &world_insts);
            self.export_sdg_json_with_occluder(path, sdg_cam, frame_index, time_s, &occluder)
        }
    }

    /// Rebuilds the CSG scene and per-compound GAS cache only when `key` changes,
    /// refreshes GPU materials in-place when `wetness` or `time_of_day` changes, and either
    /// streams only `dynamic_instances[0..D]` (or just non-crowd traffic vehicles when
    /// `gpu_crowd_active` is true) via the growable pinned staging buffers
    /// (`CsgSyncResult::TrafficOnly`) or repacks visible instances with live frustum +
    /// small-detail culling (`CsgSyncResult::FullRepack`).
    ///
    /// Traffic is not stepped here: [`Self::step_dynamic`] is the only traffic step, so the
    /// Traffic toggle holds even while the crowd keeps the shared simulation clock moving.
    #[expect(
        clippy::too_many_arguments,
        reason = "coordinates scene key, material params, camera frustum, GPU-crowd flag, and OptiX resources cleanly"
    )]
    pub fn sync(
        &mut self,
        optix_ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        key: CsgSceneKey,
        wetness: f32,
        time_of_day: u32,
        cam: CsgCameraParams,
        gpu_crowd_active: bool,
    ) -> Result<CsgSyncResult, String> {
        if self.pinned_staging.is_none() {
            self.pinned_staging =
                PinnedStagingBuffer::try_new(optix_ctx.cuda(), CSG_PINNED_STAGING_BYTES);
        }
        if self.pinned_prev_staging.is_none() {
            self.pinned_prev_staging =
                PinnedStagingBuffer::try_new(optix_ctx.cuda(), CSG_PINNED_STAGING_BYTES);
        }

        let mut rebuilt_scene = false;
        if key != self.scene_key {
            self.rebuild_scene(optix_ctx, stream, scratch, key, wetness, time_of_day)?;
            rebuilt_scene = true;
        }

        // Classic stage, or a scene whose rebuild failed: no CSG instances. One `FullRepack`
        // makes the renderer drop the old scene's instances from the IAS.
        if self.scene.is_none() {
            let dirty = self.ias_dirty || rebuilt_scene;
            self.ias_dirty = false;
            self.traffic_dirty = false;
            return Ok(if dirty {
                CsgSyncResult::FullRepack
            } else {
                CsgSyncResult::Unchanged
            });
        }

        // Fast in-place material palette refresh (zero GAS/scene rebuild!) via SceneModule.
        let mat_state = (wetness.clamp(0.0, 1.0).to_bits(), time_of_day);
        if mat_state != self.last_mat_state
            && let (Some(reg_idx), Some(scene)) = (self.active_reg_idx, self.scene.as_mut())
            && let Some(module) = self.registry.get_by_index(reg_idx)
        {
            let tod = time_of_day_from_u32(time_of_day);
            module.refresh_materials(&mut scene.world, wetness.clamp(0.0, 1.0), tod);
            self.materials_buf
                .upload(stream, scene.world.materials.as_slice())?;
            self.last_mat_state = mat_state;
        }

        // Distinguish camera/topology repacks from traffic-only frames.
        let lod_key = LodConfigKey::from_config(cam.lod_config);
        let camera_repack_needed = rebuilt_scene || self.camera_repack_needed(cam);
        let traffic_dirty = self.traffic_dirty;

        if (camera_repack_needed || traffic_dirty)
            && let Some(ref mut scene) = self.scene
        {
            // On traffic-only ticks (`!camera_repack_needed`), pack with the exact cached frustum
            // camera so static instance membership & LOD selection cannot drift out of sync with
            // the GPU IAS static tail.
            let pack_cam = if camera_repack_needed {
                cam
            } else {
                self.last_pack_cam_params.unwrap_or(cam)
            };
            let safe_lod_cfg = sanitize_lod_config(pack_cam.lod_config);
            let cam_view = if pack_cam.omnidirectional {
                CameraView::omnidirectional(
                    Vec3::new(pack_cam.eye[0], pack_cam.eye[1], pack_cam.eye[2]),
                    CSG_VIEW_FAR_M,
                    pack_cam.vfov_rad.clamp(0.15, 2.7),
                    (pack_cam.viewport_h.max(144)) as f32,
                )
            } else {
                let aspect = (pack_cam.viewport_w.max(1) as f32
                    / pack_cam.viewport_h.max(1) as f32)
                    .clamp(0.25, 4.0);
                // Pad frustum FOV by 35% so fast turns and wide architectural assemblies near
                // viewport edges never clip prematurely, while culling off-screen micro-instances.
                let padded_vfov = (pack_cam.vfov_rad * 1.35).clamp(0.15, 2.7);
                CameraView::perspective(
                    Vec3::new(pack_cam.eye[0], pack_cam.eye[1], pack_cam.eye[2]),
                    Vec3::new(pack_cam.fwd[0], pack_cam.fwd[1], pack_cam.fwd[2]),
                    Vec3::new(pack_cam.up[0], pack_cam.up[1], pack_cam.up[2]),
                    padded_vfov,
                    aspect,
                    [0.05, CSG_VIEW_FAR_M],
                    (pack_cam.viewport_h.max(144)) as f32,
                )
            }
            .with_lod_config(safe_lod_cfg);

            // W3.1: Static-tail vs dynamic-prefix split!
            // On `!camera_repack_needed` dynamic ticks, pack ONLY `world.dynamic_instances()`
            // (or ONLY non-crowd traffic vehicles when `gpu_crowd_active` scatters crowd bones
            // directly in VRAM) against the cached frustum camera, leaving the 10k-100k static
            // tail untouched in `self.packed.instances[D..]`.
            let can_try_dyn_only = !camera_repack_needed
                && !self.prev_dyn_sig.is_empty()
                && self.prev_packed_len == self.packed.instances.len()
                && self.packed.motion_prev_xforms.len() == self.packed.instances.len()
                && self.prev_xforms_buf.len_elems == self.packed.instances.len()
                && self.prev_dyn_sig.len() <= self.packed.instances.len();

            let mut handled_traffic_only = None;
            if can_try_dyn_only {
                let skip_crowd = gpu_crowd_active && self.crowd_at_dyn_tail;
                pack_dynamic_prefix_only(
                    &scene.world,
                    &cam_view,
                    skip_crowd,
                    &mut self.dyn_prev_lods,
                    &mut self.dyn_packed_scratch,
                    &mut self.dyn_prev_scratch,
                );
                let expected_sig: &[(u32, u32)] = if skip_crowd {
                    &self.prev_dyn_sig[..self.prev_non_crowd_dyn_count]
                } else {
                    &self.prev_dyn_sig
                };
                let dyn_count = self.dyn_packed_scratch.len();
                let same_dyn_topology = dyn_count == expected_sig.len()
                    && self.dyn_prev_scratch.len() == dyn_count
                    && self
                        .dyn_packed_scratch
                        .iter()
                        .zip(expected_sig)
                        .all(|(inst, &(cid, uid))| inst.compound_id == cid && inst.user_id == uid);
                if same_dyn_topology {
                    if dyn_count > 0 {
                        self.packed.instances[..dyn_count]
                            .copy_from_slice(&self.dyn_packed_scratch);
                        self.packed.motion_prev_xforms[..dyn_count]
                            .copy_from_slice(&self.dyn_prev_scratch);
                        self.instances_buf.upload_prefix_pinned(
                            stream,
                            self.pinned_staging.as_mut(),
                            &self.dyn_packed_scratch,
                        )?;
                        self.prev_xforms_buf.upload_prefix_pinned(
                            stream,
                            self.pinned_prev_staging.as_mut(),
                            &self.dyn_prev_scratch,
                        )?;
                    }
                    handled_traffic_only = Some(CsgSyncResult::TrafficOnly {
                        dynamic_count: dyn_count,
                    });
                }
            }

            let result = if let Some(fast_res) = handled_traffic_only {
                fast_res
            } else {
                if gpu_crowd_active {
                    mark_crowd_bones_visible(scene);
                }
                // GPU-direct scatter needs a packed slot for every bone wherever the camera
                // looks, and the CPU-side actor bounds that per-actor culling tests are not
                // advanced while the GPU steps the crowd. The CPU crowd path keeps culling.
                scene.world.set_crowd_culling(!gpu_crowd_active);
                scene.world.pack_visible(&cam_view, &mut self.packed);
                let slice = self.packed.instances_slice();
                let prev_slice = self.packed.motion_prev_xforms_slice();
                let dyn_count = slice
                    .iter()
                    .take_while(|inst| (inst.flags & inst_flags::DYNAMIC) != 0)
                    .count();
                self.instances_buf.upload(stream, slice)?;
                self.prev_xforms_buf.upload(stream, prev_slice)?;
                self.prev_packed_len = slice.len();
                self.prev_dyn_sig.clear();
                self.prev_dyn_sig.reserve(dyn_count);
                let mut first_crowd = dyn_count;
                let mut saw_crowd = false;
                let mut crowd_tail_contiguous = true;
                for (i, inst) in slice[..dyn_count].iter().enumerate() {
                    self.prev_dyn_sig.push((inst.compound_id, inst.user_id));
                    if (inst.flags & inst_flags::CROWD_BONE) != 0 {
                        if !saw_crowd {
                            first_crowd = i;
                            saw_crowd = true;
                        }
                    } else if saw_crowd {
                        crowd_tail_contiguous = false;
                    }
                }
                self.prev_non_crowd_dyn_count = first_crowd;
                self.crowd_at_dyn_tail = saw_crowd && crowd_tail_contiguous;
                self.dyn_prev_lods
                    .resize(scene.world.dynamic_instance_count(), PREV_LOD_NONE);
                self.dyn_prev_lods.fill(PREV_LOD_NONE);
                CsgSyncResult::FullRepack
            };

            let gpu_bytes = self.static_gpu_bytes
                + self.materials_buf.active_bytes()
                + self.instances_buf.active_bytes()
                + self.prev_xforms_buf.active_bytes();
            let lod_stats = self.packed.lod_stats;
            let culled_count = lod_stats
                .total_culled()
                .saturating_add(u32::try_from(self.packed.stats.instances_culled_budget).unwrap_or(u32::MAX));
            self.stats = CsgStats {
                visible_instances: self.packed.len() as u32,
                total_instances: scene.world.total_instance_count() as u32,
                compounds: scene.world.library.compounds.len() as u32,
                primitives: scene.world.library.prims.len() as u32,
                gpu_bytes: gpu_bytes as u64,
                aux_planes_bytes: self.aux_planes_buf.active_bytes() as u64,
                lod0_count: lod_stats.lod0,
                lod1_count: lod_stats.lod1,
                lod2_count: lod_stats.lod2,
                culled_count,
            };
            if camera_repack_needed {
                self.last_pack_cam = cam.eye;
                self.last_pack_fwd = cam.fwd;
                self.last_pack_cam_params = Some(cam);
                self.last_pack_lod_key = Some(lod_key);
                self.last_pack_omni = cam.omnidirectional;
            }
            self.ias_dirty = false;
            self.traffic_dirty = false;
            return Ok(result);
        }

        Ok(CsgSyncResult::Unchanged)
    }

    /// Fills `out_instances` with ONLY the leading `dynamic_count` moving CSG instances (`0..D`),
    /// avoiding a full 50k+ static instance rebuild on `CsgSyncResult::TrafficOnly` frames.
    pub fn fill_dynamic_optix_instances(
        &self,
        dynamic_count: usize,
        out_instances: &mut Vec<Instance>,
    ) {
        out_instances.clear();
        if !self.is_active() {
            return;
        }
        let gas_count = self.compound_gases.len();
        let slice = self.packed.instances_slice();
        let count = dynamic_count
            .min(slice.len())
            .min(self.instances_buf.len_elems)
            .min(1 << 24);
        out_instances.reserve(count);
        for (idx, gpu_inst) in slice[..count].iter().enumerate() {
            let cid = gpu_inst.compound_id as usize;
            let Some((gas_idx, visible)) = optix_slot_gas(cid, gas_count) else {
                break;
            };
            let mut inst = Instance::new(&self.compound_gases[gas_idx]);
            if !visible {
                inst.mask = 0;
            }
            inst.id = idx as u32;
            inst.sbt_offset = self
                .compound_sbt_offsets
                .get(cid)
                .copied()
                .unwrap_or(CSG_SBT_OFFSET);
            inst.flags = OPTIX_INST_DISABLE_ANYHIT;
            inst.transform = gpu_inst.xform;
            out_instances.push(inst);
        }
    }

    /// Appends all visible CSG instances into `out_instances` using cached per-compound GASes.
    ///
    /// Each OptiX instance receives `id = csg_instance_index` (matching `params.csg_instances[id]`),
    /// `OPTIX_INSTANCE_FLAG_DISABLE_ANYHIT` for opaque RT traversal, and routes single-primitive &
    /// Multi-AABB `PURE_UNION` compounds to `CSG_SBT_PRIM_OFFSET` (`10`) and general RPN compounds
    /// to `CSG_SBT_OFFSET` (`8`). Only instances present in `instances_buf` are appended, so a
    /// failed upload never leaves IAS instances indexing missing `csg_instances` entries.
    pub fn append_optix_instances(&self, out_instances: &mut Vec<Instance>) {
        if !self.is_active() {
            return;
        }
        let gas_count = self.compound_gases.len();
        let uploaded = self.packed.len().min(self.instances_buf.len_elems);
        out_instances.reserve(uploaded);
        for (idx, gpu_inst) in self.packed.instances_slice()[..uploaded].iter().enumerate() {
            // OptiX `instanceId` is 24 bits (max 16,777,215).
            if idx >= (1 << 24) {
                break;
            }
            let cid = gpu_inst.compound_id as usize;
            let Some((gas_idx, visible)) = optix_slot_gas(cid, gas_count) else {
                break;
            };
            let mut inst = Instance::new(&self.compound_gases[gas_idx]);
            if !visible {
                inst.mask = 0;
            }
            inst.id = idx as u32;
            inst.sbt_offset = self
                .compound_sbt_offsets
                .get(cid)
                .copied()
                .unwrap_or(CSG_SBT_OFFSET);
            inst.flags = OPTIX_INST_DISABLE_ANYHIT;
            inst.transform = gpu_inst.xform;
            out_instances.push(inst);
        }
    }

    /// Inserts `scene` into the bounded LRU cache under `key`, evicting oldest entries when either
    /// entry count or estimated CPU byte budget is exceeded.
    fn cache_insert_scene(&mut self, key: CsgSceneKey, scene: SceneBuilt) {
        if key.scene_idx == 0 {
            return;
        }
        self.scene_cache.retain(|(k, _)| *k != key);
        self.scene_cache.push_front((key, scene));
        while self.scene_cache.len() > SCENE_LRU_MAX_ENTRIES {
            let _evicted = self.scene_cache.pop_back();
        }
        while self.scene_cache.len() > 1 {
            let total_bytes: usize = self
                .scene_cache
                .iter()
                .map(|(_, s)| s.world.estimated_cpu_bytes())
                .sum();
            if total_bytes <= SCENE_LRU_MAX_BYTES {
                break;
            }
            let _evicted = self.scene_cache.pop_back();
        }
    }

    /// Pops a cached `SceneBuilt` matching `key` from the LRU cache if present.
    fn cache_take_scene(&mut self, key: CsgSceneKey) -> Option<SceneBuilt> {
        let pos = self.scene_cache.iter().position(|(k, _)| *k == key)?;
        self.scene_cache.remove(pos).map(|(_, s)| s)
    }

    fn rebuild_scene(
        &mut self,
        optix_ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        key: CsgSceneKey,
        wetness: f32,
        time_of_day: u32,
    ) -> Result<(), String> {
        // Synchronize before dropping old GASes / GPU buffers so no in-flight launch reads them.
        stream
            .synchronize()
            .map_err(|e| format!("CSG pre-rebuild sync failed: {e}"))?;

        let old_key = self.scene_key;
        if let Some(old_scene) = self.scene.take()
            && old_key.scene_idx != 0
            && !self.scene_mutated_stress
        {
            self.cache_insert_scene(old_key, old_scene);
        }

        self.scene_key = key;
        self.active_reg_idx = None;
        self.scene_mutated_stress = false;
        self.crowd_frozen = false;
        self.compound_gases.clear();
        self.compound_sbt_offsets.clear();
        self.compounds_buf.clear();
        self.ops_buf.clear();
        self.prims_buf.clear();
        self.aux_planes_buf.clear();
        self.guard_aabbs_buf.clear();
        self.repeats_buf.clear();
        self.materials_buf.clear();
        self.instances_buf.clear();
        self.prev_xforms_buf.clear();
        self.prev_packed_len = 0;
        self.prev_dyn_sig.clear();
        self.prev_non_crowd_dyn_count = 0;
        self.crowd_at_dyn_tail = false;
        self.dyn_prev_lods.clear();
        self.static_gpu_bytes = 0;
        self.stats = CsgStats::default();
        self.last_mat_state = (u32::MAX, u32::MAX);
        self.last_pack_cam = [f32::NAN; 3];
        self.last_pack_fwd = [f32::NAN; 3];
        self.last_pack_cam_params = None;
        self.last_pack_lod_key = None;
        self.last_pack_omni = false;
        self.active_possess = None;
        self.next_vehicle_cursor = 0;
        self.last_traffic_bits = u32::MAX;
        self.ias_dirty = true;
        self.traffic_dirty = false;

        if key.scene_idx == 0 {
            self.pending_metadata = Some(SceneLoadedMetadata {
                scene_idx: 0,
                variant_idx: 0,
                viewpoints: Vec::new(),
                rails: Vec::new(),
            });
            return Ok(());
        }

        // From here on a failure leaves `scene = None` with `scene_key = key` committed: `sync`
        // then reports one `FullRepack` (dropping the old CSG instances from the IAS) and does
        // not retry every frame; selecting another scene and back retries, from the LRU cache
        // when the CPU build itself succeeded.
        let reg_idx = registry_index_for_scene(key.scene_idx, self.registry.len())?;
        let tod = time_of_day_from_u32(time_of_day);
        let road_mode = match key.road_mode {
            0 => csg_scene::RoadMode::Watertight3D,
            1 => csg_scene::RoadMode::TangentPlaneC1,
            3 => csg_scene::RoadMode::FRepTile,
            _ => csg_scene::RoadMode::HybridC1Sdf,
        };
        let build_cfg = SceneBuildConfig {
            seed: u64::from(key.seed).max(1),
            wetness: wetness.clamp(0.0, 1.0),
            time_of_day: tod,
            variant_idx: key.variant_idx,
            road_mode,
            ..SceneBuildConfig::default()
        };

        let cached_built = self.cache_take_scene(key);
        let module = self
            .registry
            .get_by_index(reg_idx)
            .ok_or_else(|| format!("Scene index {reg_idx} not found in SceneRegistry"))?;
        let mut built = match cached_built {
            Some(mut cached) => {
                cached.set_crowd_frozen(false);
                cached
            }
            None => module.build(&build_cfg).map_err(|e| {
                format!("CSG scene '{}' failed to build: {e}", module.descriptor().name)
            })?,
        };
        module.refresh_materials(&mut built.world, wetness.clamp(0.0, 1.0), tod);
        self.last_mat_state = (wetness.clamp(0.0, 1.0).to_bits(), time_of_day);

        if let Err(e) = self.upload_scene_gpu(optix_ctx, stream, scratch, &built) {
            // Keep the CPU build for a cheap retry; `compound_gases` is still empty from above.
            self.cache_insert_scene(key, built);
            return Err(e);
        }

        self.packed
            .reserve_instances(built.world.total_instance_count().max(256));
        built.reset_crowd_motion_history();
        self.active_reg_idx = Some(reg_idx);
        self.pending_metadata = Some(SceneLoadedMetadata {
            scene_idx: key.scene_idx,
            variant_idx: key.variant_idx,
            viewpoints: built.viewpoints.clone(),
            rails: built.rails.clone(),
        });
        self.scene = Some(built);
        Ok(())
    }

    /// Uploads `built`'s compiled library and materials and builds its per-compound GASes,
    /// committing `compound_gases` / `compound_sbt_offsets` only when every build succeeded.
    fn upload_scene_gpu(
        &mut self,
        optix_ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        built: &SceneBuilt,
    ) -> Result<(), String> {
        // Upload static compiled library buffers into persistent grow-only VRAM pools (zero
        // alloc/free churn across RoadMode toggles or scene rebuilds once warm).
        let lib = &built.world.library;
        // A compound the compiler refused is absent from the library, so its object would
        // otherwise vanish from the render without a trace.
        for issue in &lib.issues {
            eprintln!(
                "nv-demo: CSG compiler rejected geometry: {:?} (required {}, limit {})",
                issue.kind, issue.required, issue.limit
            );
        }
        self.compounds_buf.upload(stream, &lib.compounds)?;
        self.ops_buf.upload(stream, &lib.ops)?;
        self.prims_buf.upload(stream, &lib.prims)?;
        self.aux_planes_buf.upload(stream, &lib.aux_planes)?;
        self.guard_aabbs_buf.upload(stream, &lib.guard_aabbs)?;
        self.repeats_buf.upload(stream, &lib.repeats)?;
        self.materials_buf
            .upload(stream, built.world.materials.as_slice())?;

        self.static_gpu_bytes = self.compounds_buf.active_bytes()
            + self.ops_buf.active_bytes()
            + self.prims_buf.active_bytes()
            + self.aux_planes_buf.active_bytes()
            + self.guard_aabbs_buf.active_bytes()
            + self.repeats_buf.active_bytes();

        // Build one non-compacted (`compact: false`) OptiX custom-primitive GAS per compound:
        // - `MultiAabbUnion` -> M tight primitive AABBs with `primitive_offset = first_prim` (SBT 10)
        // - `SinglePrim` -> 1 compound AABB with `primitive_offset = prim_idx` (SBT 10)
        // - `GeneralRpn` -> 1 compound AABB with `primitive_offset = 0` (SBT 8)
        let mut gases = Vec::with_capacity(lib.compounds.len());
        let mut sbt_offsets = Vec::with_capacity(lib.compounds.len());
        if !lib.compounds.is_empty() {
            if self.aabb_staging_buf.is_none() {
                let staging = DeviceBuffer::<Aabb>::zeroed(stream, 1)
                    .map_err(|e| format!("CSG AABB staging alloc failed: {e}"))?;
                self.aabb_staging_buf = Some(staging);
            }
            let aabb_staging = self
                .aabb_staging_buf
                .as_ref()
                .ok_or_else(|| "CSG AABB staging buffer missing".to_string())?;
            for c in &lib.compounds {
                let (gas, sbt_off) = build_compound_gas(
                    optix_ctx,
                    stream,
                    scratch,
                    aabb_staging,
                    &mut self.multi_aabb,
                    lib,
                    c,
                )?;
                gases.push(gas);
                sbt_offsets.push(sbt_off);
            }
            stream
                .synchronize()
                .map_err(|e| format!("CSG GAS sync failed: {e}"))?;
        }

        self.compound_gases = gases;
        self.compound_sbt_offsets = sbt_offsets;
        Ok(())
    }
}

/// Ensures every dynamic crowd bone instance in `scene.world` carries `inst_flags::VISIBLE` so
/// GPU-direct scatter (`scatter_crowd_to_csg_and_optix`) always receives a 1:1 packed slot.
#[inline]
fn mark_crowd_bones_visible(scene: &mut SceneBuilt) {
    for &inst_id in &scene.crowd_inst_ids {
        if let Some(inst) = scene.world.dynamic_instance_mut(inst_id) {
            inst.flags |= inst_flags::VISIBLE;
        }
    }
}

/// Canonical bit representation of a row-major 3x4 affine transform for deterministic bone-slot
/// matching (canonicalizes `-0.0` to `+0.0` so `xform * Mat3x4::IDENTITY` never mismatches).
#[inline]
fn xform_to_bits(xf: [f32; 12]) -> [u32; 12] {
    let mut out = [0u32; 12];
    for (dst, &v) in out.iter_mut().zip(&xf) {
        *dst = if v == 0.0 { 0.0f32 } else { v }.to_bits();
    }
    out
}

/// Human-readable display label matching `csg_scene::list_possess_targets_with_crowd`.
fn crowd_actor_display_name(crowd: &CrowdSim, actor_idx: usize) -> String {
    let prefix = match crowd.descs.get(actor_idx).map(|d| d.archetype_id) {
        Some(x) if x == ArchetypeKind::HumanHero as u32 => "sdg_mannequin",
        Some(x) if x == ArchetypeKind::HumanCrowd as u32 => "pedestrian",
        Some(x) if x == ArchetypeKind::RoadSignStickman as u32 => "road_sign_stickman",
        Some(x) if x == ArchetypeKind::Quadruped as u32 => "quadruped_dog",
        _ => "crowd_actor",
    };
    format!("{prefix}_{actor_idx}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_registry_exposes_all_scenes_and_variants() {
        let descs = default_scene_descriptors();
        let ids: Vec<&str> = descs.iter().map(|d| d.id).collect();
        // The scenes nv-demo relies on must be present; csg-rs may register more.
        for id in ["gundo", "gundo_osm", "marina", "sf_house_of_ai", "inspector"] {
            assert!(ids.contains(&id), "{id} missing from {ids:?}");
        }
        let inspector = descs.iter().find(|d| d.id == "inspector").expect("inspector");
        assert!(inspector.variant_labels.len() >= 45);
    }

    #[test]
    fn sanitized_compound_aabb_handles_thin_unbounded_and_nan() {
        let thin = GpuCompound {
            aabb_min: [0.0, 0.0, 0.0],
            aabb_max: [10.0, 0.0, 10.0],
            ..Default::default()
        };
        let a = sanitized_compound_aabb(&thin);
        assert!(a.min[1] < 0.0 && a.max[1] > 0.0);

        let unbounded = GpuCompound {
            flags: compound_flags::UNBOUNDED,
            aabb_min: [f32::NEG_INFINITY; 3],
            aabb_max: [f32::INFINITY; 3],
            ..Default::default()
        };
        let u = sanitized_compound_aabb(&unbounded);
        assert_eq!(u.min, [-UNBOUNDED_COMPOUND_HALF_EXTENT_M; 3]);
        assert_eq!(u.max, [UNBOUNDED_COMPOUND_HALF_EXTENT_M; 3]);
        // An unbounded ground must still cover a camera at the far end of the packing view from
        // a compound origin 1 km away (the old box ended at 1000 m).
        assert!(u.max[0] > 1000.0 + CSG_VIEW_FAR_M);

        let broken = GpuCompound {
            aabb_min: [f32::NAN, 5.0, 0.0],
            aabb_max: [1.0, 2.0, 3.0],
            ..Default::default()
        };
        let b = sanitized_compound_aabb(&broken);
        assert!(b.min.iter().all(|v| v.is_finite()));
        assert!(b.max.iter().all(|v| v.is_finite()));
        for i in 0..3 {
            assert!(b.min[i] < b.max[i]);
        }
    }

    #[test]
    fn all_city_scenes_pack_and_step_traffic_without_heap_churn() {
        let reg = csg_scenes::default_scene_registry();
        for scene_id in ["gundo", "gundo_osm", "marina", "sf_house_of_ai"] {
            let module = reg.get(scene_id).expect("scene registered");
            let mut built = module.build(&SceneBuildConfig::default()).expect("scene builds");
            assert!(
                built.world.total_instance_count() >= 100,
                "{scene_id}: expected rich instance count"
            );
            assert!(
                built.world.library.compounds.len() <= csg_scene::gas_ledger::MAX_GPU_COMPOUNDS,
                "{scene_id}: exceeded the {} GPU compound limit ({})",
                csg_scene::gas_ledger::MAX_GPU_COMPOUNDS,
                built.world.library.compounds.len()
            );
            assert!(!built.viewpoints.is_empty(), "{scene_id}: missing viewpoints");
            assert!(!built.rails.is_empty(), "{scene_id}: missing rails");

            let mut packed = PackedGpuScene::with_capacity(
                built.world.total_instance_count() * 2,
                built.world.library.compounds.len(),
            );
            let vp = &built.viewpoints[0];
            let cam = CameraView::omnidirectional(vp.eye, 1500.0, 1.0, 720.0);
            built.world.pack_visible(&cam, &mut packed);
            let cap_before = packed.instances.capacity();
            assert!(!packed.is_empty(), "{scene_id}: packed empty");

            for step in 1..=5 {
                module.step_dynamic(&mut built.world, &built.traffic, step as f32 * 0.1);
                built.world.pack_visible(&cam, &mut packed);
                assert_eq!(
                    packed.instances.capacity(),
                    cap_before,
                    "{scene_id}: steady-state heap realloc"
                );
            }
        }
    }

    #[test]
    fn sbt_routing_splits_single_prim_and_rpn_compounds() {
        // Production routing (`CompoundGasPlan::sbt_offset`), not a test-only copy of it.
        assert_eq!(
            CompoundGasPlan::SinglePrim { prim_idx: 3 }.sbt_offset(),
            CSG_SBT_PRIM_OFFSET
        );
        assert_eq!(
            CompoundGasPlan::MultiAabbUnion {
                first_prim: 0,
                count: 2
            }
            .sbt_offset(),
            CSG_SBT_PRIM_OFFSET
        );
        assert_eq!(CompoundGasPlan::GeneralRpn.sbt_offset(), CSG_SBT_OFFSET);
    }

    #[test]
    fn optix_slots_stay_aligned_when_a_compound_has_no_gas() {
        assert_eq!(optix_slot_gas(3, 5), Some((3, true)));
        // Missing GAS: invisible placeholder keeps `packed slot == OptiX slot` for the scatter.
        assert_eq!(optix_slot_gas(7, 5), Some((0, false)));
        assert_eq!(optix_slot_gas(0, 0), None);
    }

    #[test]
    fn out_of_range_scene_index_is_rejected_not_clamped() {
        assert_eq!(registry_index_for_scene(1, 5), Ok(0));
        assert_eq!(registry_index_for_scene(5, 5), Ok(4));
        assert!(registry_index_for_scene(6, 5).is_err());
        assert!(registry_index_for_scene(9, 5).is_err());
        assert!(registry_index_for_scene(0, 5).is_err());
        assert!(registry_index_for_scene(1, 0).is_err());
    }

    #[test]
    fn dynamic_prefix_packer_drops_lod_history_when_lod_is_disabled() {
        let reg = csg_scenes::default_scene_registry();
        let module = reg.get("gundo_osm").expect("gundo_osm registered");
        let built = module.build(&SceneBuildConfig::default()).expect("scene builds");
        let vp = built.viewpoints[0].clone();
        let fwd = (vp.target - vp.eye).norm();
        let view = CameraView::perspective(
            vp.eye,
            fwd,
            Vec3::Y,
            (vp.vfov_deg.to_radians() * 1.35).clamp(0.15, 2.7),
            16.0 / 9.0,
            [0.05, CSG_VIEW_FAR_M],
            720.0,
        )
        .with_lod_config(LodConfig {
            enabled: false,
            ..LodConfig::default()
        });
        let mut lods = vec![0u8; built.world.dynamic_instance_count()];
        let mut insts = Vec::new();
        let mut prev = Vec::new();
        pack_dynamic_prefix_only(&built.world, &view, false, &mut lods, &mut insts, &mut prev);
        assert!(!insts.is_empty(), "the viewpoint must see dynamic instances");
        assert!(
            lods.contains(&PREV_LOD_NONE),
            "emitted instances must reset their LOD history like upstream emit_assembly_instance"
        );
    }

    #[test]
    fn traffic_clock_alone_does_not_schedule_an_instance_upload() {
        let mut host = CsgGpuHost::new();
        let gundo_idx = host
            .registry
            .descriptors()
            .iter()
            .position(|d| d.id == "gundo")
            .expect("gundo registered");
        let module = host
            .registry
            .get_by_index(gundo_idx)
            .expect("gundo module");
        let built = module.build(&SceneBuildConfig::default()).expect("scene builds");
        let vp = built.viewpoints[0].clone();
        let fwd = (vp.target - vp.eye).norm();
        let key = CsgSceneKey {
            scene_idx: (gundo_idx + 1) as u32,
            variant_idx: 0,
            seed: 2026,
            road_mode: 2,
        };
        host.scene_key = key;
        host.active_reg_idx = Some(gundo_idx);
        host.scene = Some(built);
        assert!(host.has_traffic());
        let cam = CsgCameraParams {
            eye: [vp.eye.x, vp.eye.y, vp.eye.z],
            fwd: [fwd.x, fwd.y, fwd.z],
            up: [0.0, 1.0, 0.0],
            viewport_w: 1280,
            viewport_h: 720,
            vfov_rad: vp.vfov_deg.to_radians(),
            lod_config: LodConfig::default(),
            omnidirectional: false,
        };
        host.last_pack_cam = cam.eye;
        host.last_pack_fwd = cam.fwd;
        host.last_pack_cam_params = Some(cam);
        host.last_pack_lod_key = Some(LodConfigKey::from_config(cam.lod_config));
        host.ias_dirty = false;
        host.traffic_dirty = false;

        // Traffic toggled off while the crowd advances the shared clock: `step_dynamic` skips
        // traffic, so nothing schedules an upload (the old predicate compared the clock bits and
        // `sync` then stepped the vehicles anyway).
        assert!(!host.step_dynamic(0.0, 4.25, cam.eye, CrowdLodParams::default(), false, false));
        assert!(!host.will_upload_instances(key, cam));
        // A real traffic step does.
        assert!(host.step_dynamic(0.0, 4.5, cam.eye, CrowdLodParams::default(), false, true));
        assert!(host.will_upload_instances(key, cam));
    }

    #[test]
    fn perspective_frustum_culling_prunes_offscreen_city_micro_instances() {
        let reg = csg_scenes::default_scene_registry();
        let module = reg.get("sf_house_of_ai").expect("sf_house_of_ai");
        let mut built = module.build(&SceneBuildConfig::default()).expect("scene builds");
        let vp = &built.viewpoints[0];
        let fwd = (vp.target - vp.eye).norm();
        let cam_persp = CameraView::perspective(
            vp.eye,
            fwd,
            Vec3::Y,
            (vp.vfov_deg.to_radians() * 1.28).clamp(0.15, 2.7),
            16.0 / 9.0,
            [0.05, 1500.0],
            720.0,
        );
        let mut packed = PackedGpuScene::with_capacity(
            built.world.total_instance_count() * 2,
            built.world.library.compounds.len(),
        );
        built.world.pack_visible(&cam_persp, &mut packed);
        assert!(!packed.is_empty(), "visible frustum must contain geometry");
        assert!(
            packed.stats.culled_instances > 0,
            "perspective frustum + small-detail culling must prune off-screen/far micro-instances"
        );
        assert!(
            packed.stats.single_prim_instances > 0,
            "city scenes must route single-prim instances to the lean SBT 10 hit group"
        );
    }

    #[test]
    fn crowd_step_dynamic_respects_freeze_pose_and_exports_sdg_json() {
        let mut host = CsgGpuHost::new();
        let inspector_idx = host
            .registry
            .descriptors()
            .iter()
            .position(|d| d.id == "inspector")
            .expect("inspector registered");
        let module = host
            .registry
            .get_by_index(inspector_idx)
            .expect("inspector module");
        let cfg = SceneBuildConfig {
            variant_idx: 109, // 4-archetype actor crowd showcase
            ..SceneBuildConfig::default()
        };
        let mut built = module.build(&cfg).expect("scene builds");
        assert_eq!(built.crowd.actor_count(), 4, "4-archetype showcase has 4 actors");
        built.reset_crowd_motion_history();
        let vp = built.viewpoints[0].clone();
        let cam_eye = [vp.eye.x, vp.eye.y, vp.eye.z];

        host.scene_key = CsgSceneKey {
            scene_idx: (inspector_idx + 1) as u32,
            variant_idx: 109,
            seed: 1,
            road_mode: 2,
        };
        host.active_reg_idx = Some(inspector_idx);
        host.scene = Some(built);

        assert!(host.has_crowd());
        // Freeze Pose must prevent crowd stepping and set FREEZE_POSE flag.
        assert!(!host.step_dynamic(
            0.05,
            0.05,
            cam_eye,
            CrowdLodParams::default(),
            true,
            true
        ));
        assert!(host.crowd_frozen);

        // Unfreezing steps the crowd and populates motion_prev_xforms.
        assert!(host.step_dynamic(
            0.05,
            0.10,
            cam_eye,
            CrowdLodParams::default(),
            false,
            true
        ));
        assert!(!host.crowd_frozen);
        assert!(host.step_dynamic(
            0.05,
            0.15,
            cam_eye,
            CrowdLodParams::default(),
            false,
            true
        ));

        let fwd = (vp.target - vp.eye).norm();
        let right = fwd.cross(Vec3::Y).norm();
        let up = right.cross(fwd).norm();
        let cam_frame = CameraFrame {
            eye: cam_eye,
            fwd: [fwd.x, fwd.y, fwd.z],
            right: [right.x, right.y, right.z],
            up: [up.x, up.y, up.z],
            tan_half_fov: (0.5 * vp.vfov_deg.to_radians()).tan(),
        };
        let cam_view = CameraView::perspective(
            vp.eye,
            fwd,
            up,
            vp.vfov_deg.to_radians(),
            16.0 / 9.0,
            [0.05, 500.0],
            720.0,
        );
        host.scene
            .as_mut()
            .expect("scene")
            .world
            .pack_visible(&cam_view, &mut host.packed);
        assert_eq!(
            host.packed.instances_slice().len(),
            host.packed.motion_prev_xforms_slice().len(),
            "motion_prev_xforms must align 1:1 with visible instances"
        );
        assert!(
            host.packed
                .instances_slice()
                .iter()
                .any(|inst| (inst.flags & inst_flags::MOTION_BLUR) != 0),
            "stepped crowd bones must emit MOTION_BLUR flags for object optical flow"
        );

        let out_path = std::env::temp_dir().join("nv_demo_sdg_export_test.json");
        let sdg_cam =
            CsgGpuHost::build_sdg_camera(&cam_frame, CameraProjection::Pinhole, 1280, 720);
        let count = host
            .export_sdg_json(&out_path, sdg_cam, 7, 0.15)
            .expect("SDG JSON export");
        assert_eq!(count, 4, "all 4 showcase actors must be exported");
        let raw = std::fs::read_to_string(&out_path).expect("read exported SDG JSON");
        let parsed: serde_json::Value = serde_json::from_str(&raw).expect("valid JSON");
        assert_eq!(parsed["scene_id"], "inspector");
        assert_eq!(parsed["actors"].as_array().map(Vec::len), Some(4));
        let _ = std::fs::remove_file(&out_path);

        // Transitioning freeze_pose false -> true must snap prev_xform = xform for all dynamic
        // instances and mark traffic_dirty so Motion AOV emits zero stale object optical flow.
        host.traffic_dirty = false;
        assert!(!host.step_dynamic(
            0.05,
            0.20,
            cam_eye,
            CrowdLodParams::default(),
            true,
            true
        ));
        assert!(host.crowd_frozen);
        assert!(
            host.traffic_dirty,
            "freeze_pose false -> true transition must schedule zero-motion upload"
        );
        host.scene
            .as_mut()
            .expect("scene")
            .world
            .pack_visible(&cam_view, &mut host.packed);
        for (inst, prev) in host
            .packed
            .instances_slice()
            .iter()
            .zip(host.packed.motion_prev_xforms_slice())
        {
            if (inst.flags & inst_flags::DYNAMIC) != 0 {
                assert_eq!(
                    inst.xform, *prev,
                    "frozen dynamic instance must have zero object optical flow (prev_xform == xform)"
                );
            }
        }
    }

    #[test]
    fn soa_city_traffic_detected_and_stepped_by_host() {
        let mut host = CsgGpuHost::new();
        let osm_idx = host
            .registry
            .descriptors()
            .iter()
            .position(|d| d.id == "gundo_osm")
            .expect("gundo_osm registered");
        let module = host
            .registry
            .get_by_index(osm_idx)
            .expect("gundo_osm module");
        let built = module.build(&SceneBuildConfig::default()).expect("scene builds");
        assert!(
            built.traffic.vehicle_count() > 0,
            "gundo_osm must exercise the SoA city traffic (vehicle_count > 0)"
        );
        let vp = built.viewpoints[0].clone();
        let cam_eye = [vp.eye.x, vp.eye.y, vp.eye.z];

        host.scene_key = CsgSceneKey {
            scene_idx: (osm_idx + 1) as u32,
            variant_idx: 0,
            seed: 1,
            road_mode: 2,
        };
        host.active_reg_idx = Some(osm_idx);
        host.scene = Some(built);

        assert!(
            host.has_traffic(),
            "has_traffic() must detect SoA traffic even when legacy actors vec is empty"
        );
        let before: Vec<[f32; 12]> = host
            .scene
            .as_ref()
            .expect("scene")
            .world
            .dynamic_instances()
            .iter()
            .map(|i| i.xform.0)
            .collect();
        assert!(
            host.step_dynamic(0.0, 1.5, cam_eye, CrowdLodParams::default(), false, true),
            "step_dynamic must advance SoA city traffic"
        );
        let moved_any = host
            .scene
            .as_ref()
            .expect("scene")
            .world
            .dynamic_instances()
            .iter()
            .zip(&before)
            .any(|(inst, prev)| inst.xform.0 != *prev);
        assert!(moved_any, "SoA city traffic vehicles must move after step_dynamic");
    }

    #[test]
    fn ctrl_click_pick_possess_on_gundo_osm_moving_car_never_goes_underground() {
        let mut host = CsgGpuHost::new();
        let osm_idx = host
            .registry
            .descriptors()
            .iter()
            .position(|d| d.id == "gundo_osm")
            .expect("gundo_osm registered");
        let module = host
            .registry
            .get_by_index(osm_idx)
            .expect("gundo_osm module");
        let built = module.build(&SceneBuildConfig {
            road_mode: csg_scene::RoadMode::FRepTile,
            ..SceneBuildConfig::default()
        })
        .expect("scene builds");
        let vp = built.viewpoints[0].clone();
        host.scene_key = CsgSceneKey {
            scene_idx: (osm_idx + 1) as u32,
            variant_idx: 0,
            seed: 1,
            road_mode: 3,
        };
        host.active_reg_idx = Some(osm_idx);
        host.scene = Some(built);

        // Advance SoA traffic so vehicles are actively moving at bluff elevation (~30-45 m).
        assert!(host.step_dynamic(
            0.05,
            1.5,
            [vp.eye.x, vp.eye.y, vp.eye.z],
            CrowdLodParams::default(),
            false,
            true
        ));

        let car_sample = {
            let scene = host.scene.as_ref().expect("scene");
            scene
                .world
                .dynamic_instances()
                .iter()
                .find_map(|inst| {
                    if !inst.is_visible() {
                        return None;
                    }
                    let asm = scene.world.assembly(inst.assembly_id)?;
                    if !asm.camera_mounts.is_empty() && inst.xform.0[7] > 20.0 {
                        Some((
                            asm.name.clone(),
                            inst.xform,
                            asm.local_aabb,
                            scene.hints.elevation.sample(inst.xform.0[3], inst.xform.0[11]),
                        ))
                    } else {
                        None
                    }
                })
                .expect("expected moving car at bluff elevation y > 20 m in gundo_osm")
        };
        let (car_name, car_xform, car_aabb, _dem_y) = car_sample;
        let car_pos = [car_xform.0[3], car_xform.0[7], car_xform.0[11]];
        let car_y = car_pos[1];
        let target = car_xform.xform_pt(car_aabb.center());
        let cam_eye = Vec3::new(target.x, target.y + 8.0, target.z - 4.0);
        let fwd = (target - cam_eye).norm();
        let right = fwd.cross(Vec3::Y).norm();
        let up = right.cross(fwd).norm();
        let cam_frame = CameraFrame {
            eye: [cam_eye.x, cam_eye.y, cam_eye.z],
            fwd: [fwd.x, fwd.y, fwd.z],
            right: [right.x, right.y, right.z],
            up: [up.x, up.y, up.z],
            tan_half_fov: (0.5 * 55.0_f32.to_radians()).tan(),
        };
        let (origin, dir) = crate::renderer::camera_projection_ray(
            &cam_frame,
            CameraProjection::Pinhole,
            0.5,
            0.5,
            16.0 / 9.0,
        )
        .expect("center click ray");
        let info = host
            .pick_possess(origin, dir)
            .expect("Ctrl+Click on moving car must pick the car");
        let pose = host
            .resolve_active_possess(0.0, 0.0, true)
            .expect("resolve possessed car pose");
        eprintln!(
            "target_car={car_name} car_y={car_y:.2} picked={} is_dyn={} mount0={} pose_eye=({:.2}, {:.2}, {:.2})",
            info.assembly_name,
            info.is_dynamic,
            info.mounts.first().map_or("none", |m| m.label.as_str()),
            pose.eye.x,
            pose.eye.y,
            pose.eye.z
        );
        assert!(info.is_dynamic, "picked instance must be dynamic, got {}", info.assembly_name);
        assert!(
            pose.eye.y >= car_y + 0.5 && pose.eye.y <= car_y + 6.0,
            "possessed camera eye.y={:.2} must be within [{:.2}, {:.2}] for {car_name}, never underground!",
            pose.eye.y,
            car_y + 0.5,
            car_y + 6.0
        );
    }

    #[test]
    fn crowd_possess_poke_and_stress_work_on_inspector_109() {
        let mut host = CsgGpuHost::new();
        let insp_idx = host
            .registry
            .descriptors()
            .iter()
            .position(|d| d.id == "inspector")
            .expect("inspector registered");
        let module = host.registry.get_by_index(insp_idx).expect("inspector module");
        let built = module.build(&SceneBuildConfig {
            variant_idx: 109,
            ..SceneBuildConfig::default()
        })
        .expect("scene builds");
        assert_eq!(built.crowd.actor_count(), 4);
        let initial_compounds = built.world.library.compounds.len();

        host.scene_key = CsgSceneKey {
            scene_idx: (insp_idx + 1) as u32,
            variant_idx: 109,
            seed: 2026,
            road_mode: 2,
        };
        host.active_reg_idx = Some(insp_idx);
        host.scene = Some(built);

        // Step crowd once and verify bone_slot_map matches every visible bone exactly.
        assert!(host.step_dynamic(
            1.0 / 60.0,
            0.1,
            [0.0, 2.0, -8.0],
            CrowdLodParams::default(),
            false,
            false
        ));
        {
            let scene = host.scene.as_mut().expect("scene");
            scene.world.pack_all(Vec3::new(0.0, 2.0, -8.0), &mut host.packed);
        }
        let mut slot_map = Vec::new();
        host.fill_crowd_bone_slot_map(&mut slot_map);
        let crowd = host.active_crowd().expect("crowd");
        assert_eq!(slot_map.len(), crowd.bone_count());
        let mapped_visible = slot_map.iter().filter(|&&s| s != u32::MAX).count();
        assert!(
            mapped_visible >= 40,
            "expected >=40 visible bones mapped in Inspector 109, got {mapped_visible}"
        );

        // Possess cycling finds all 4 crowd actors with eye_pov + over_shoulder mounts.
        let first = host
            .possess_next_vehicle()
            .expect("possess_next_vehicle must include crowd actors");
        assert!(host.is_possessing_crowd());
        assert_eq!(first.mounts.len(), 2);
        let pov = host
            .resolve_active_possess(0.0, 0.0, true)
            .expect("eye_pov pose");
        assert!(pov.eye.y > 0.2 && pov.eye.y < 2.5);
        let _ = host.set_possess_mount(1).expect("over_shoulder mount");
        let os = host
            .resolve_active_possess(0.0, 0.0, true)
            .expect("over_shoulder pose");
        assert!((os.eye - pov.eye).len() > 0.5);

        // Poke nearest actor sets MASK_IMPULSE on at least one actor.
        let msg = host
            .poke_crowd([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], 2.0, false)
            .expect("poke_crowd");
        assert!(msg.contains("Poked crowd actor"));
        assert!(
            host.active_crowd()
                .expect("crowd")
                .intents
                .iter()
                .any(|it| (it.flags & csg_actors::intent_flags::MASK_IMPULSE) != 0)
        );

        // Stress spawn grows cleanly reusing the existing 11-GAS bank (0 extra compounds).
        let scene = host.active_scene_mut().expect("scene");
        let added = scene
            .spawn_crowd_stress(64, Vec3::ZERO)
            .expect("spawn_crowd_stress 64");
        assert_eq!(added, 60);
        assert_eq!(scene.crowd.actor_count(), 64);
        assert_eq!(
            scene.world.library.compounds.len(),
            initial_compounds,
            "reusing existing 11-GAS ActorGasBank must add 0 compounds"
        );
    }

    #[test]
    fn csg_gas_build_opts_disables_compaction_and_multi_aabb_plans_pure_union() {
        let opts = csg_gas_build_opts();
        assert!(!opts.compact, "CSG compound GAS builds must disable synchronous compaction");
        assert!(opts.prefer_fast_trace);

        let mut lib = CompiledLibrary::new();
        let single_node = csg_core::CsgNode::box3(
            [1.0, 1.0, 1.0],
            csg_core::MaterialId(1),
            csg_core::semantic::BUILDING,
        );
        let single_id = lib
            .compile_single(&single_node, csg_core::OptimizeConfig::default(), 0)
            .expect("single");
        assert!(matches!(
            compound_gas_plan(&lib, &lib.compounds[single_id.0 as usize]),
            CompoundGasPlan::SinglePrim { .. }
        ));

        // Box + sphere + cylinder at lateral offset `dx`: separated parts for `dx = 5`
        // (`PURE_UNION`), overlapping parts for `dx = 0.5` (`FIRST_HIT_UNION`).
        let three_parts = |dx: f32| {
            csg_core::CsgNode::box3(
                [1.0, 1.0, 1.0],
                csg_core::MaterialId(1),
                csg_core::semantic::BUILDING,
            )
            .union(
                csg_core::CsgNode::sphere(
                    0.75,
                    csg_core::MaterialId(2),
                    csg_core::semantic::BUILDING,
                )
                .translated(dx, 0.0, 0.0),
            )
            .union(
                csg_core::CsgNode::cylinder(
                    0.4,
                    1.2,
                    csg_core::MaterialId(3),
                    csg_core::semantic::BUILDING,
                )
                .translated(-dx, 0.2, 0.0),
            )
        };
        let union_id = lib
            .compile_single(&three_parts(5.0), csg_core::OptimizeConfig::default(), 0)
            .expect("pure union");
        let union_flags = lib.compounds[union_id.0 as usize].flags;
        assert_ne!(union_flags & compound_flags::PURE_UNION, 0, "flags {union_flags:#x}");
        let plan = compound_gas_plan(&lib, &lib.compounds[union_id.0 as usize]);
        assert!(
            matches!(plan, CompoundGasPlan::MultiAabbUnion { count: 3, .. }),
            "expected MultiAabbUnion with 3 prims, got {plan:?}"
        );
        assert_eq!(plan.sbt_offset(), CSG_SBT_PRIM_OFFSET);

        // Overlapping parts: first hit over parts is wrong for rays starting inside a part, so
        // the compound must stay on the exact RPN path, never on the multi-AABB GAS.
        let overlap_id = lib
            .compile_single(&three_parts(0.5), csg_core::OptimizeConfig::default(), 0)
            .expect("overlapping union");
        let overlap_flags = lib.compounds[overlap_id.0 as usize].flags;
        assert_ne!(
            overlap_flags & compound_flags::FIRST_HIT_UNION,
            0,
            "flags {overlap_flags:#x}"
        );
        assert_eq!(overlap_flags & compound_flags::PURE_UNION, 0, "flags {overlap_flags:#x}");
        let overlap_plan = compound_gas_plan(&lib, &lib.compounds[overlap_id.0 as usize]);
        assert_eq!(overlap_plan, CompoundGasPlan::GeneralRpn);
        assert_eq!(overlap_plan.sbt_offset(), CSG_SBT_OFFSET);

        let sub_node = single_node.subtract(
            csg_core::CsgNode::sphere(0.5, csg_core::MaterialId(2), csg_core::semantic::WINDOW)
                .translated(0.0, 0.0, 0.8),
        );
        let sub_id = lib
            .compile_single(&sub_node, csg_core::OptimizeConfig::default(), 0)
            .expect("subtract");
        let sub_plan = compound_gas_plan(&lib, &lib.compounds[sub_id.0 as usize]);
        assert_eq!(sub_plan, CompoundGasPlan::GeneralRpn);
        assert_eq!(sub_plan.sbt_offset(), CSG_SBT_OFFSET);
    }

    #[test]
    fn dynamic_prefix_packer_matches_full_pack_and_lru_cache_evicts_bounded() {
        let mut host = CsgGpuHost::new();
        let osm_idx = host
            .registry
            .descriptors()
            .iter()
            .position(|d| d.id == "gundo_osm")
            .expect("gundo_osm registered");
        let module = host
            .registry
            .get_by_index(osm_idx)
            .expect("gundo_osm module");
        let mut built = module.build(&SceneBuildConfig::default()).expect("scene builds");
        let vp = built.viewpoints[0].clone();
        let fwd = (vp.target - vp.eye).norm();
        let view = CameraView::perspective(
            vp.eye,
            fwd,
            Vec3::Y,
            (vp.vfov_deg.to_radians() * 1.35).clamp(0.15, 2.7),
            16.0 / 9.0,
            [0.05, 1500.0],
            720.0,
        );

        let mut full_packed = PackedGpuScene::new();
        built.world.pack_visible(&view, &mut full_packed);
        let full_dyn_count = full_packed
            .instances_slice()
            .iter()
            .take_while(|inst| (inst.flags & inst_flags::DYNAMIC) != 0)
            .count();

        let mut dyn_lods = Vec::new();
        let mut dyn_insts = Vec::new();
        let mut dyn_prev = Vec::new();
        pack_dynamic_prefix_only(
            &built.world,
            &view,
            false,
            &mut dyn_lods,
            &mut dyn_insts,
            &mut dyn_prev,
        );
        assert_eq!(dyn_insts.len(), full_dyn_count);
        assert_eq!(dyn_insts, full_packed.instances_slice()[..full_dyn_count]);
        assert_eq!(
            dyn_prev,
            full_packed.motion_prev_xforms_slice()[..full_dyn_count]
        );

        // Verify `skip_crowd_bones = true` emits strictly the non-crowd traffic prefix.
        let expected_non_crowd = full_packed.instances_slice()[..full_dyn_count]
            .iter()
            .filter(|inst| (inst.flags & inst_flags::CROWD_BONE) == 0)
            .count();
        let mut traffic_lods = Vec::new();
        let mut traffic_insts = Vec::new();
        let mut traffic_prev = Vec::new();
        pack_dynamic_prefix_only(
            &built.world,
            &view,
            true,
            &mut traffic_lods,
            &mut traffic_insts,
            &mut traffic_prev,
        );
        assert_eq!(traffic_insts.len(), expected_non_crowd);
        assert!(
            traffic_insts
                .iter()
                .all(|inst| (inst.flags & inst_flags::CROWD_BONE) == 0),
            "skip_crowd_bones must omit all GPU-scattered crowd bones"
        );

        // Verify LRU cache hit, metadata retention, and eviction bound.
        let key_a = CsgSceneKey {
            scene_idx: 1,
            variant_idx: 0,
            seed: 1,
            road_mode: 2,
        };
        let vp_count = built.viewpoints.len();
        host.cache_insert_scene(key_a, built);
        let restored = host.cache_take_scene(key_a).expect("LRU cache hit");
        assert_eq!(restored.viewpoints.len(), vp_count);
        assert!(host.cache_take_scene(key_a).is_none());
    }

    #[test]
    fn lod_bias_change_triggers_repack_and_changes_gundo_visible_instances() {
        let mut host = CsgGpuHost::new();
        let gundo_idx = host
            .registry
            .descriptors()
            .iter()
            .position(|d| d.id == "gundo")
            .expect("gundo registered");
        let module = host
            .registry
            .get_by_index(gundo_idx)
            .expect("gundo module");
        let built = module.build(&SceneBuildConfig::default()).expect("scene builds");
        let vp = built.viewpoints[0].clone();
        let fwd = (vp.target - vp.eye).norm();

        let key = CsgSceneKey {
            scene_idx: (gundo_idx + 1) as u32,
            variant_idx: 0,
            seed: 2026,
            road_mode: 2,
        };
        host.scene_key = key;
        host.active_reg_idx = Some(gundo_idx);
        host.scene = Some(built);

        let base_cam = CsgCameraParams {
            eye: [vp.eye.x, vp.eye.y, vp.eye.z],
            fwd: [fwd.x, fwd.y, fwd.z],
            up: [0.0, 1.0, 0.0],
            viewport_w: 1280,
            viewport_h: 720,
            vfov_rad: vp.vfov_deg.to_radians(),
            lod_config: LodConfig {
                lod_bias: 0.25,
                ..LodConfig::default()
            },
            omnidirectional: false,
        };

        // Prime cached pack state at `lod_bias = 0.25`.
        host.last_pack_cam = base_cam.eye;
        host.last_pack_fwd = base_cam.fwd;
        host.last_pack_cam_params = Some(base_cam);
        host.last_pack_lod_key = Some(LodConfigKey::from_config(base_cam.lod_config));
        host.last_pack_omni = false;
        host.ias_dirty = false;

        assert!(
            !host.will_camera_repack(key, base_cam),
            "unchanged camera and lod_config must not spuriously force FullRepack"
        );
        let high_cam = CsgCameraParams {
            lod_config: LodConfig {
                lod_bias: 4.0,
                ..LodConfig::default()
            },
            ..base_cam
        };
        assert!(
            host.will_camera_repack(key, high_cam),
            "changing lod_bias from 0.25 to 4.0 must always trigger FullRepack even when camera is stationary"
        );
        // Non-finite lod_bias must safely normalize to 1.0 and trigger repack from 0.25.
        let nan_cam = CsgCameraParams {
            lod_config: LodConfig {
                lod_bias: f32::NAN,
                ..LodConfig::default()
            },
            ..base_cam
        };
        assert!(host.will_camera_repack(key, nan_cam));

        // Pack Gundo at lod_bias = 0.25, 1.0, and 4.0 and verify visible instance count increases.
        let make_view = |bias: f32| {
            CameraView::perspective(
                vp.eye,
                fwd,
                Vec3::Y,
                (vp.vfov_deg.to_radians() * 1.35).clamp(0.15, 2.7),
                16.0 / 9.0,
                [0.05, 1500.0],
                720.0,
            )
            .with_lod_bias(bias)
        };
        let scene = host.scene.as_mut().expect("gundo scene");
        let mut packed_low = PackedGpuScene::new();
        let mut packed_mid = PackedGpuScene::new();
        let mut packed_high = PackedGpuScene::new();
        scene.world.pack_visible(&make_view(0.25), &mut packed_low);
        scene.world.pack_visible(&make_view(1.0), &mut packed_mid);
        scene.world.pack_visible(&make_view(4.0), &mut packed_high);

        assert!(
            !packed_low.is_empty(),
            "Gundo must retain primary terrain/buildings/roads even at lod_bias=0.25"
        );
        assert!(
            packed_low.len() < packed_high.len(),
            "Gundo visible packed instances must strictly increase from lod_bias=0.25 ({}) to 4.0 ({}) (mid 1.0 = {})",
            packed_low.len(),
            packed_high.len(),
            packed_mid.len()
        );
        assert!(
            packed_low.len() <= packed_mid.len() && packed_mid.len() <= packed_high.len(),
            "Gundo visible packed instances must be monotonic across 0.25 ({}) <= 1.0 ({}) <= 4.0 ({})",
            packed_low.len(),
            packed_mid.len(),
            packed_high.len()
        );
    }

    #[test]
    fn crowd_bone_slot_map_handles_neg_zero_and_all_slotted_bones() {
        assert_eq!(
            xform_to_bits([-0.0, 1.0, 0.0, -0.0, 0.0, 1.0, -0.0, 0.0, -0.0, 0.0, 1.0, -0.0]),
            xform_to_bits([0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
            "xform_to_bits must canonicalize -0.0 to +0.0 for deterministic bone slot mapping"
        );

        let mut host = CsgGpuHost::new();
        let insp_idx = host
            .registry
            .descriptors()
            .iter()
            .position(|d| d.id == "inspector")
            .expect("inspector registered");
        let module = host
            .registry
            .get_by_index(insp_idx)
            .expect("inspector module");
        let built = module.build(&SceneBuildConfig {
            variant_idx: 109,
            ..SceneBuildConfig::default()
        })
        .expect("scene builds");
        host.scene_key = CsgSceneKey {
            scene_idx: (insp_idx + 1) as u32,
            variant_idx: 109,
            seed: 1,
            road_mode: 2,
        };
        host.active_reg_idx = Some(insp_idx);
        host.scene = Some(built);

        // Simulate low lod_bias culling all actors far from camera, then reviving for GPU scatter.
        assert!(host.refresh_crowd_lod_cpu(
            0.0,
            [0.0, 2.0, 500.0],
            CrowdLodParams {
                lod_bias: 0.25,
                ..CrowdLodParams::default()
            }
        ));
        {
            let scene = host.scene.as_mut().expect("scene");
            mark_crowd_bones_visible(scene);
            scene
                .world
                .pack_all(Vec3::new(0.0, 2.0, 500.0), &mut host.packed);
        }
        let mut slot_map = Vec::new();
        host.fill_crowd_bone_slot_map(&mut slot_map);
        let n_bones = host.active_crowd().expect("crowd").bone_count();
        assert_eq!(slot_map.len(), n_bones);
        let unmapped = slot_map.iter().filter(|&&s| s == u32::MAX).count();
        assert_eq!(
            unmapped, 0,
            "every crowd bone must stay mapped after mark_crowd_bones_visible so GPU scatter can revive culled actors"
        );
    }

    #[test]
    fn smart_rail_candidates_builds_verified_multi_landmark_rails_for_gundo() {
        let mut host = CsgGpuHost::new();
        let gundo_idx = host
            .registry
            .descriptors()
            .iter()
            .position(|d| d.id == "gundo")
            .expect("gundo registered");
        let module = host
            .registry
            .get_by_index(gundo_idx)
            .expect("gundo module");
        let built = module.build(&SceneBuildConfig::default()).expect("scene builds");
        host.scene_key = CsgSceneKey {
            scene_idx: (gundo_idx + 1) as u32,
            variant_idx: 0,
            seed: 2026,
            road_mode: 2,
        };
        host.active_reg_idx = Some(gundo_idx);
        host.scene = Some(built);

        let rails = host
            .build_smart_rail_candidates(42)
            .expect("build_smart_rail_candidates on Gundo");
        assert!(
            !rails.is_empty(),
            "build_smart_rail_candidates must return at least one verified rail"
        );
        for rail in &rails {
            assert!(
                rail.waypoints.len() >= 4,
                "smart rail '{}' should visit multiple waypoints, got {}",
                rail.label,
                rail.waypoints.len()
            );
        }
    }

    #[test]
    fn lod_config_force_lod_disable_and_distance_sliders_trigger_repack_and_update_stats() {
        let mut host = CsgGpuHost::new();
        let gundo_idx = host
            .registry
            .descriptors()
            .iter()
            .position(|d| d.id == "gundo")
            .expect("gundo registered");
        let module = host
            .registry
            .get_by_index(gundo_idx)
            .expect("gundo module");
        let built = module.build(&SceneBuildConfig::default()).expect("scene builds");
        let vp = built.viewpoints[0].clone();
        let fwd = (vp.target - vp.eye).norm();

        let key = CsgSceneKey {
            scene_idx: (gundo_idx + 1) as u32,
            variant_idx: 0,
            seed: 2026,
            road_mode: 2,
        };
        host.scene_key = key;
        host.active_reg_idx = Some(gundo_idx);
        host.scene = Some(built);

        let default_cfg = LodConfig::default();
        let base_cam = CsgCameraParams {
            eye: [vp.eye.x, vp.eye.y, vp.eye.z],
            fwd: [fwd.x, fwd.y, fwd.z],
            up: [0.0, 1.0, 0.0],
            viewport_w: 1280,
            viewport_h: 720,
            vfov_rad: vp.vfov_deg.to_radians(),
            lod_config: default_cfg,
            omnidirectional: false,
        };

        host.last_pack_cam = base_cam.eye;
        host.last_pack_fwd = base_cam.fwd;
        host.last_pack_cam_params = Some(base_cam);
        host.last_pack_lod_key = Some(LodConfigKey::from_config(default_cfg));
        host.last_pack_omni = false;
        host.ias_dirty = false;
        assert!(!host.will_camera_repack(key, base_cam));

        // Every individual LodConfig slider/toggle must trigger immediate FullRepack.
        let variants = [
            LodConfig { enabled: false, ..default_cfg },
            LodConfig { force_lod: Some(0), ..default_cfg },
            LodConfig { force_lod: Some(2), ..default_cfg },
            LodConfig { lod1_dist_m: 35.0, ..default_cfg },
            LodConfig { lod2_dist_m: 90.0, ..default_cfg },
            LodConfig { meso_dist_m: 80.0, ..default_cfg },
            LodConfig { micro_dist_m: 10.0, ..default_cfg },
            LodConfig { min_screen_px: 8.0, ..default_cfg },
            LodConfig { hysteresis: 0.30, ..default_cfg },
        ];
        for (idx, cfg) in variants.into_iter().enumerate() {
            let cam = CsgCameraParams {
                lod_config: cfg,
                ..base_cam
            };
            assert!(
                host.will_camera_repack(key, cam),
                "LodConfig variant #{idx} ({cfg:?}) must trigger immediate FullRepack"
            );
        }

        // Pack Gundo under Force LOD0, Force LOD2, LOD Disabled, and strict distance/tier cull
        // and verify `CsgStats` (`lod0_count`, `lod1_count`, `lod2_count`, `culled_count`).
        let pack_stats_for = |world: &mut WorldScene, cfg: LodConfig| -> CsgStats {
            let view = CameraView::perspective(
                vp.eye,
                fwd,
                Vec3::Y,
                (vp.vfov_deg.to_radians() * 1.35).clamp(0.15, 2.7),
                16.0 / 9.0,
                [0.05, 1500.0],
                720.0,
            )
            .with_lod_config(sanitize_lod_config(cfg));
            let mut packed = PackedGpuScene::new();
            world.pack_visible(&view, &mut packed);
            let ls = packed.lod_stats;
            CsgStats {
                visible_instances: packed.len() as u32,
                total_instances: world.total_instance_count() as u32,
                compounds: world.library.compounds.len() as u32,
                primitives: world.library.prims.len() as u32,
                gpu_bytes: 0,
                aux_planes_bytes: 0,
                lod0_count: ls.lod0,
                lod1_count: ls.lod1,
                lod2_count: ls.lod2,
                culled_count: ls
                    .total_culled()
                    .saturating_add(u32::try_from(packed.stats.instances_culled_budget).unwrap_or(u32::MAX)),
            }
        };

        let world = &mut host.scene.as_mut().expect("gundo").world;
        let stats_force_lod0 = pack_stats_for(
            world,
            LodConfig {
                force_lod: Some(0),
                ..default_cfg
            },
        );
        let stats_force_lod2 = pack_stats_for(
            world,
            LodConfig {
                force_lod: Some(2),
                ..default_cfg
            },
        );
        let stats_disabled = pack_stats_for(
            world,
            LodConfig {
                enabled: false,
                meso_dist_m: 15.0,
                micro_dist_m: 5.0,
                min_screen_px: 12.0,
                ..default_cfg
            },
        );
        let stats_tight_cull = pack_stats_for(
            world,
            LodConfig {
                enabled: true,
                lod1_dist_m: 20.0,
                lod2_dist_m: 45.0,
                meso_dist_m: 40.0,
                micro_dist_m: 8.0,
                min_screen_px: 10.0,
                ..default_cfg
            },
        );

        assert!(
            stats_force_lod0.lod0_count > 0 && stats_force_lod0.lod2_count == 0,
            "Force LOD0 must emit only LOD0 parts, got {stats_force_lod0:?}"
        );
        assert!(
            stats_force_lod2.lod2_count > 0 && stats_force_lod2.lod0_count < stats_force_lod0.lod0_count,
            "Force LOD2 must push multi-LOD/auto-proxy parts into lod2_count: lod0={stats_force_lod0:?}, lod2={stats_force_lod2:?}"
        );
        assert!(
            stats_disabled.lod1_count == 0
                && stats_disabled.lod2_count == 0
                && stats_disabled.visible_instances > stats_tight_cull.visible_instances,
            "LOD disabled must bypass LOD reduction and tier/screen-px culling: disabled={stats_disabled:?}, tight={stats_tight_cull:?}"
        );
        assert!(
            stats_tight_cull.culled_count > stats_disabled.culled_count
                && (stats_tight_cull.lod1_count + stats_tight_cull.lod2_count) > 0,
            "tight distance/tier/screen-px sliders must increase culled_count and activate LOD1/LOD2: tight={stats_tight_cull:?}, disabled={stats_disabled:?}"
        );
    }
}
