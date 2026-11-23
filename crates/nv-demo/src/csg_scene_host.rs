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

use std::ffi::c_void;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::Arc;

use bytemuck::Pod;
use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, DeviceBuffer, Stream};
use optix::{Aabb, Accel, BuildOptions, CustomPrimitives, DeviceContext, Instance, Scratch};

use csg_actors::{
    ArchetypeKind, CrowdSim, CsgRayOccluder, RayOccluder, SdgActorRecord, SdgCamera, SdgProjection,
    Vec3 as ActorVec3, extract_sdg_frame,
};
use csg_core::abi::compound_flags;
use csg_core::math::{Mat3x4, Vec3};
use csg_core::{CompoundId, GpuAabb, GpuCompound, GpuOp, GpuPrim, GpuRepeat, Ray};
use csg_scene::{
    CROWD_USER_ID_BASE, CameraPose as CsgCameraPose, CameraView, GpuInstance, GpuMaterial,
    PackedGpuScene, RailGenConfig, RailSpec, SceneBuildConfig, SceneBuilt, SceneDescriptor,
    SceneHints, SceneRegistry, SceneViewpoint, TimeOfDay, WorldScene, find_crowd_actor_for_inst,
    generate_random_rail, inst_flags, is_dynamic_inst_id, list_possess_targets_with_crowd,
    lookup_instance_and_assembly, pick_possessable_instance_with_crowd,
    resolve_possess_pose_with_crowd,
};

use crate::app_events::PossessInfo;
use crate::renderer::{CameraFrame, CameraProjection};

/// SBT hit-group offset for general multi-op CSG RPN (`__intersection__csg` + `__closesthit__csg`).
pub const CSG_SBT_OFFSET: u32 = 4;

/// SBT hit-group offset for ultra-lean single-primitive CSG (`__intersection__csg_prim` + `__closesthit__csg`).
pub const CSG_SBT_PRIM_OFFSET: u32 = 5;

/// Routes single-primitive compounds to the register-lean SBT 5 hit group and general RPN compounds to SBT 4.
#[inline]
pub const fn sbt_offset_for_compound(flags: u32) -> u32 {
    if (flags & compound_flags::SINGLE_PRIM) != 0 {
        CSG_SBT_PRIM_OFFSET
    } else {
        CSG_SBT_OFFSET
    }
}

/// Camera basis and viewport parameters driving live CSG frustum + small-detail culling.
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
    /// User LOD detail multiplier (`0.25..=4.0`).
    pub lod_bias: f32,
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
    if (comp.flags & 1) != 0 {
        return Aabb {
            min: [-1000.0, -1000.0, -1000.0],
            max: [1000.0, 1000.0, 1000.0],
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

/// Default pinned host staging capacity (`256 KiB` = 4,096 `GpuInstance` records) for true
/// async DMA updates of `dynamic_instances[0..D]`.
const CSG_PINNED_STAGING_BYTES: usize = 256 * 1024;

/// Reusable page-locked host staging buffer (`cuMemAllocHost_v2` / `cuMemFreeHost`).
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
}

impl Drop for PinnedStagingBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.ctx.bind_to_thread().is_ok() {
            // SAFETY: `self.ptr` was allocated via `cuMemAllocHost_v2` on `self.ctx`.
            unsafe {
                let _ = cu::cuMemFreeHost(self.ptr);
            }
            self.ptr = std::ptr::null_mut();
        }
    }
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
    fn upload(&mut self, stream: &Arc<Stream>, items: &[T]) -> Result<(), String> {
        self.len_elems = items.len();
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
        }
        Ok(())
    }

    /// Streams a leading prefix `items` into `self.buf[0..items.len()]` using `pinned` staging
    /// memory when available so `cuMemcpyHtoDAsync_v2` executes as true asynchronous DMA with
    /// zero VRAM allocations and zero pageable-memory stalls.
    fn upload_prefix_pinned(
        &self,
        stream: &Arc<Stream>,
        pinned: Option<&PinnedStagingBuffer>,
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
            && bytes.len() <= stage.bytes
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

/// Fast deterministic FNV-1a signature over the packed static tail so `TrafficOnly` never runs
/// if any static instance entered, exited, reordered, or switched LOD compound.
#[inline]
fn hash_static_topology(static_slice: &[GpuInstance]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for inst in static_slice {
        h ^= u64::from(inst.compound_id) | (u64::from(inst.user_id) << 32);
        h = h.wrapping_mul(0x0100_0000_01b3);
        h ^= u64::from(inst.mat_override);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
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
/// batched per-compound OptiX GAS cache, and flat GPU buffers.
pub struct CsgGpuHost {
    registry: SceneRegistry,
    scene_key: CsgSceneKey,
    active_reg_idx: Option<usize>,
    scene: Option<SceneBuilt>,
    crowd_frozen: bool,
    pending_metadata: Option<SceneLoadedMetadata>,
    packed: PackedGpuScene,
    /// Per-compound cached OptiX custom-primitive GASes (indexed by `compound_id`).
    compound_gases: Vec<Arc<Accel>>,
    /// Per-compound SBT hit-group offset (`CSG_SBT_PRIM_OFFSET` for single-prim, `CSG_SBT_OFFSET` for RPN).
    compound_sbt_offsets: Vec<u32>,
    compounds_buf: GrowDeviceBuffer<GpuCompound>,
    ops_buf: GrowDeviceBuffer<GpuOp>,
    prims_buf: GrowDeviceBuffer<GpuPrim>,
    aux_planes_buf: GrowDeviceBuffer<[f32; 4]>,
    guard_aabbs_buf: GrowDeviceBuffer<GpuAabb>,
    repeats_buf: GrowDeviceBuffer<GpuRepeat>,
    batched_aabbs_buf: Option<DeviceBuffer<Aabb>>,
    aabb_staging_buf: Option<DeviceBuffer<Aabb>>,
    materials_buf: GrowDeviceBuffer<GpuMaterial>,
    instances_buf: GrowDeviceBuffer<GpuInstance>,
    prev_xforms_buf: GrowDeviceBuffer<[f32; 12]>,
    pinned_staging: Option<PinnedStagingBuffer>,
    pinned_prev_staging: Option<PinnedStagingBuffer>,
    prev_packed_len: usize,
    prev_dyn_sig: Vec<(u32, u32)>,
    prev_static_sig: u64,
    static_gpu_bytes: usize,
    last_mat_state: (u32, u32),
    last_pack_cam: [f32; 3],
    last_pack_fwd: [f32; 3],
    last_pack_cam_params: Option<CsgCameraParams>,
    last_pack_lod_bits: u32,
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
            batched_aabbs_buf: None,
            aabb_staging_buf: None,
            materials_buf: GrowDeviceBuffer::default(),
            instances_buf: GrowDeviceBuffer::default(),
            prev_xforms_buf: GrowDeviceBuffer::default(),
            pinned_staging: None,
            pinned_prev_staging: None,
            prev_packed_len: 0,
            prev_dyn_sig: Vec::new(),
            prev_static_sig: 0,
            static_gpu_bytes: 0,
            last_mat_state: (u32::MAX, u32::MAX),
            last_pack_cam: [f32::NAN; 3],
            last_pack_fwd: [f32::NAN; 3],
            last_pack_cam_params: None,
            last_pack_lod_bits: 0,
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

    /// Returns the cached SBT hit-group offset (`4` or `5`) for every compiled compound.
    pub fn compound_sbt_offsets(&self) -> &[u32] {
        &self.compound_sbt_offsets
    }

    /// Computes the exact 1:1 mapping `bone_slot_map[gidx] -> packed_instance_slot` (or `u32::MAX`
    /// when a bone is frustum/LOD-culled from `self.packed`) in O(`dyn_count + total_bones`) time.
    ///
    /// Uses `(user_id, compound_id, xform_bits)` matching along the strictly monotonic ascending
    /// `crowd_inst_ids` order so partial frustum culling or identical adjacent bone materials can
    /// never cross-wire slots.
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
            if gpu_inst.user_id < CROWD_USER_ID_BASE {
                continue;
            }
            let target_bits = xform_to_bits(gpu_inst.xform);
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
                    break;
                }
                gidx += 1;
            }
        }
    }

    /// Returns `true` if calling [`Self::sync`] with `(key, cam)` will trigger a full frustum
    /// repack (`camera_repack_needed` or scene rebuild).
    pub fn will_camera_repack(&self, key: CsgSceneKey, cam: CsgCameraParams) -> bool {
        if key != self.scene_key || self.ias_dirty {
            return true;
        }
        if key.scene_idx == 0 {
            return false;
        }
        let cam_dist_sq = (cam.eye[0] - self.last_pack_cam[0]).powi(2)
            + (cam.eye[1] - self.last_pack_cam[1]).powi(2)
            + (cam.eye[2] - self.last_pack_cam[2]).powi(2);
        let fwd_dot = cam.fwd[0] * self.last_pack_fwd[0]
            + cam.fwd[1] * self.last_pack_fwd[1]
            + cam.fwd[2] * self.last_pack_fwd[2];
        let lod_bits = cam.lod_bias.clamp(0.25, 4.0).to_bits();
        let viewport_or_fov_changed = self.last_pack_cam_params.is_none_or(|prev| {
            prev.viewport_w != cam.viewport_w
                || prev.viewport_h != cam.viewport_h
                || prev.vfov_rad.to_bits() != cam.vfov_rad.to_bits()
        });
        viewport_or_fov_changed
            || lod_bits != self.last_pack_lod_bits
            || cam.omnidirectional != self.last_pack_omni
            || !cam_dist_sq.is_finite()
            || cam_dist_sq > 1.0
            || (!cam.omnidirectional && (!fwd_dot.is_finite() || fwd_dot < 0.9985))
    }

    /// Applies `freeze_pose` transitions to `scene.crowd` and dynamic traffic motion history.
    /// Returns `true` if crowd freeze intents changed and must be re-uploaded to CUDA.
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
        self.traffic_dirty = true;
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
        if new_compound_count > old_compound_count {
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

            let build_opts = BuildOptions {
                prefer_fast_trace: true,
                ..Default::default()
            };
            if self.aabb_staging_buf.is_none() {
                let staging = DeviceBuffer::<Aabb>::zeroed(stream, 1)
                    .map_err(|e| format!("CSG AABB staging alloc failed: {e}"))?;
                self.aabb_staging_buf = Some(staging);
            }
            let aabb_staging = self
                .aabb_staging_buf
                .as_ref()
                .ok_or_else(|| "CSG AABB staging buffer missing".to_string())?;
            let dst_ptr = aabb_staging.cu_deviceptr();
            let box_bytes = std::mem::size_of::<Aabb>();

            for c in &lib.compounds[old_compound_count..new_compound_count] {
                let aabb = sanitized_compound_aabb(c);
                // SAFETY: `aabb_staging` holds 1 `Aabb` (`24 B` plain `f32`) on `stream`;
                // `stream.synchronize()` completes before returning.
                let h2d = unsafe {
                    cu::cuMemcpyHtoDAsync_v2(
                        dst_ptr,
                        (&raw const aabb).cast(),
                        box_bytes,
                        stream.cu_stream(),
                    )
                };
                if h2d != cu::cudaError_enum_CUDA_SUCCESS {
                    return Err(format!("CSG stress AABB H2D copy failed ({h2d:?})"));
                }
                let gas = Accel::custom(
                    optix_ctx,
                    stream,
                    scratch,
                    &[CustomPrimitives::new(aabb_staging)],
                    build_opts,
                )
                .map_err(|e| format!("CSG stress GAS build failed: {e}"))?;
                self.compound_gases.push(gas);
                self.compound_sbt_offsets
                    .push(sbt_offset_for_compound(c.flags));
            }
            stream
                .synchronize()
                .map_err(|e| format!("CSG stress GAS sync failed: {e}"))?;
        }

        self.packed
            .reserve_instances(scene.world.total_instance_count().max(256));
        self.ias_dirty = true;
        self.traffic_dirty = true;
        Ok(added)
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

    /// Packs the full scene omnidirectionally into `self.packed` and uploads `self.instances_buf`
    /// for city-wide view scoring, marking `ias_dirty` so the next render frame restores frustum culling.
    pub fn prepare_full_scene_for_probe(&mut self, stream: &Arc<Stream>) -> Result<(), String> {
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
        self.instances_buf
            .upload(stream, self.packed.instances_slice())?;
        self.prev_xforms_buf
            .upload(stream, self.packed.motion_prev_xforms_slice())?;
        self.ias_dirty = true;
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
            instance_count: self.packed.len() as u32,
            material_count: world.materials.len() as u32,
            compound_count: world.library.compounds.len() as u32,
        }
    }

    /// Advances dynamic 3D Frenet traffic and articulated crowd actors in-place before resolving
    /// possessed vehicle mounts so cameras lock to exact current-frame transforms.
    ///
    /// Respects `freeze_pose` (pausing both traffic and crowd stepping so progressive path tracing
    /// can accumulate cleanly for Synthetic Data Generation captures).
    pub fn step_dynamic(
        &mut self,
        dt: f32,
        time_s: f32,
        cam_eye: [f32; 3],
        freeze_pose: bool,
        step_traffic: bool,
    ) -> bool {
        let Some(ref mut scene) = self.scene else {
            return false;
        };
        if freeze_pose != self.crowd_frozen {
            scene.set_crowd_frozen(freeze_pose);
            self.crowd_frozen = freeze_pose;
            if freeze_pose {
                scene.reset_crowd_motion_history();
                for inst in scene.world.dynamic_instances_mut() {
                    inst.prev_xform = Some(inst.xform);
                }
                self.traffic_dirty = true;
            }
        }
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
            scene.step_crowd(dt, time_s, Vec3::new(cam_eye[0], cam_eye[1], cam_eye[2]));
            self.traffic_dirty = true;
            moved = true;
        }

        moved
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
    /// atomically to `path` as pretty-printed JSON.
    #[expect(
        clippy::too_many_arguments,
        reason = "forwards camera basis, projection, viewport dimensions, and frame metadata cleanly"
    )]
    pub fn export_sdg_json(
        &self,
        path: &Path,
        cam: &CameraFrame,
        projection: CameraProjection,
        width: u32,
        height: u32,
        frame_index: u32,
        time_s: f32,
    ) -> Result<usize, String> {
        let Some(ref scene) = self.scene else {
            return Err("Load a CSG scene before exporting SDG JSON".to_string());
        };
        let sdg_cam = Self::build_sdg_camera(cam, projection, width, height);
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
    /// refreshes GPU materials in-place when `wetness` or `time_of_day` changes,
    /// steps 3D Frenet dynamic traffic (unless `freeze_pose` is enabled), and either streams only
    /// `dynamic_instances[0..D]` via the reusable `256 KiB` pinned staging buffers
    /// (`CsgSyncResult::TrafficOnly`) or repacks visible instances with live frustum + small-detail
    /// culling (`CsgSyncResult::FullRepack`).
    #[expect(
        clippy::too_many_arguments,
        reason = "coordinates scene key, material params, camera frustum, freeze flag, and OptiX resources cleanly"
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
        traffic_time_s: f32,
        freeze_pose: bool,
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

        if key.scene_idx == 0 {
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

        // Fallback animated 3D Frenet traffic step (zero heap allocations, skipped when frozen).
        let traffic_bits = traffic_time_s.to_bits();
        let traffic_moved_now =
            !freeze_pose && self.has_traffic() && traffic_bits != self.last_traffic_bits;
        if traffic_moved_now
            && let (Some(reg_idx), Some(scene)) = (self.active_reg_idx, self.scene.as_mut())
            && let Some(module) = self.registry.get_by_index(reg_idx)
        {
            module.step_dynamic(&mut scene.world, &scene.traffic, traffic_time_s);
            self.last_traffic_bits = traffic_bits;
            self.traffic_dirty = true;
        }

        // Distinguish camera/topology repacks from traffic-only frames.
        // Repack threshold (`1.0 m` translation or `~3.1°` rotation, plus viewport/FOV changes)
        // stays strictly inside the `1.35x` padded frustum margin so objects near viewport edges
        // never pop out prematurely.
        let cam_dist_sq = (cam.eye[0] - self.last_pack_cam[0]).powi(2)
            + (cam.eye[1] - self.last_pack_cam[1]).powi(2)
            + (cam.eye[2] - self.last_pack_cam[2]).powi(2);
        let fwd_dot = cam.fwd[0] * self.last_pack_fwd[0]
            + cam.fwd[1] * self.last_pack_fwd[1]
            + cam.fwd[2] * self.last_pack_fwd[2];
        let lod_bits = cam.lod_bias.clamp(0.25, 4.0).to_bits();
        let viewport_or_fov_changed = self.last_pack_cam_params.is_none_or(|prev| {
            prev.viewport_w != cam.viewport_w
                || prev.viewport_h != cam.viewport_h
                || prev.vfov_rad.to_bits() != cam.vfov_rad.to_bits()
        });
        let camera_repack_needed = rebuilt_scene
            || self.ias_dirty
            || viewport_or_fov_changed
            || lod_bits != self.last_pack_lod_bits
            || cam.omnidirectional != self.last_pack_omni
            || !cam_dist_sq.is_finite()
            || cam_dist_sq > 1.0
            || (!cam.omnidirectional && (!fwd_dot.is_finite() || fwd_dot < 0.9985));
        let traffic_dirty = self.traffic_dirty;

        if (camera_repack_needed || traffic_dirty)
            && let Some(ref mut scene) = self.scene
        {
            let world = &mut scene.world;
            // On traffic-only ticks (`!camera_repack_needed`), pack with the exact cached frustum
            // camera so static instance membership & LOD selection cannot drift out of sync with
            // the GPU IAS static tail.
            let pack_cam = if camera_repack_needed {
                cam
            } else {
                self.last_pack_cam_params.unwrap_or(cam)
            };
            let mut cam_view = if pack_cam.omnidirectional {
                CameraView::omnidirectional(
                    Vec3::new(pack_cam.eye[0], pack_cam.eye[1], pack_cam.eye[2]),
                    1500.0,
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
                    [0.05, 1500.0],
                    (pack_cam.viewport_h.max(144)) as f32,
                )
            };
            cam_view.lod_bias = pack_cam.lod_bias.clamp(0.25, 4.0);
            world.pack_visible(&cam_view, &mut self.packed);

            let slice = self.packed.instances_slice();
            let prev_slice = self.packed.motion_prev_xforms_slice();
            let dyn_count = slice
                .iter()
                .take_while(|inst| (inst.flags & inst_flags::DYNAMIC) != 0)
                .count();
            let static_sig = hash_static_topology(&slice[dyn_count..]);
            let same_topology = !camera_repack_needed
                && dyn_count > 0
                && slice.len() == self.prev_packed_len
                && prev_slice.len() == slice.len()
                && self.prev_xforms_buf.len_elems == slice.len()
                && dyn_count == self.prev_dyn_sig.len()
                && static_sig == self.prev_static_sig
                && slice[..dyn_count]
                    .iter()
                    .zip(&self.prev_dyn_sig)
                    .all(|(inst, &(cid, uid))| inst.compound_id == cid && inst.user_id == uid);

            let result = if same_topology {
                self.instances_buf.upload_prefix_pinned(
                    stream,
                    self.pinned_staging.as_ref(),
                    &slice[..dyn_count],
                )?;
                self.prev_xforms_buf.upload_prefix_pinned(
                    stream,
                    self.pinned_prev_staging.as_ref(),
                    &prev_slice[..dyn_count],
                )?;
                CsgSyncResult::TrafficOnly {
                    dynamic_count: dyn_count,
                }
            } else {
                self.instances_buf.upload(stream, slice)?;
                self.prev_xforms_buf.upload(stream, prev_slice)?;
                self.prev_packed_len = slice.len();
                self.prev_static_sig = static_sig;
                self.prev_dyn_sig.clear();
                self.prev_dyn_sig.reserve(dyn_count);
                for inst in &slice[..dyn_count] {
                    self.prev_dyn_sig.push((inst.compound_id, inst.user_id));
                }
                CsgSyncResult::FullRepack
            };

            let gpu_bytes = self.static_gpu_bytes
                + self.materials_buf.active_bytes()
                + self.instances_buf.active_bytes()
                + self.prev_xforms_buf.active_bytes();
            self.stats = CsgStats {
                visible_instances: self.packed.len() as u32,
                total_instances: world.total_instance_count() as u32,
                compounds: world.library.compounds.len() as u32,
                primitives: world.library.prims.len() as u32,
                gpu_bytes: gpu_bytes as u64,
                aux_planes_bytes: self.aux_planes_buf.active_bytes() as u64,
            };
            if camera_repack_needed {
                self.last_pack_cam = cam.eye;
                self.last_pack_fwd = cam.fwd;
                self.last_pack_cam_params = Some(cam);
                self.last_pack_lod_bits = lod_bits;
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
        let count = dynamic_count.min(slice.len()).min(1 << 24);
        out_instances.reserve(count);
        for (idx, gpu_inst) in slice[..count].iter().enumerate() {
            let cid = gpu_inst.compound_id as usize;
            if cid >= gas_count {
                continue;
            }
            let mut inst = Instance::new(&self.compound_gases[cid]);
            inst.id = idx as u32;
            inst.sbt_offset = self
                .compound_sbt_offsets
                .get(cid)
                .copied()
                .unwrap_or(CSG_SBT_OFFSET);
            inst.transform = gpu_inst.xform;
            out_instances.push(inst);
        }
    }

    /// Appends all visible CSG instances into `out_instances` using cached per-compound GASes.
    ///
    /// Each OptiX instance receives `id = csg_instance_index` (matching `params.csg_instances[id]`)
    /// and routes single-primitive compounds to `CSG_SBT_PRIM_OFFSET` (`5`) and general RPN
    /// compounds to `CSG_SBT_OFFSET` (`4`).
    pub fn append_optix_instances(&self, out_instances: &mut Vec<Instance>) {
        if !self.is_active() {
            return;
        }
        let gas_count = self.compound_gases.len();
        out_instances.reserve(self.packed.len());
        for (idx, gpu_inst) in self.packed.instances_slice().iter().enumerate() {
            // OptiX `instanceId` is 24 bits (max 16,777,215).
            if idx >= (1 << 24) {
                break;
            }
            let cid = gpu_inst.compound_id as usize;
            if cid >= gas_count {
                continue;
            }
            let mut inst = Instance::new(&self.compound_gases[cid]);
            inst.id = idx as u32;
            inst.sbt_offset = self
                .compound_sbt_offsets
                .get(cid)
                .copied()
                .unwrap_or(CSG_SBT_OFFSET);
            inst.transform = gpu_inst.xform;
            out_instances.push(inst);
        }
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

        self.scene_key = key;
        self.active_reg_idx = None;
        self.scene = None;
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
        self.prev_static_sig = 0;
        self.static_gpu_bytes = 0;
        self.stats = CsgStats::default();
        self.last_mat_state = (u32::MAX, u32::MAX);
        self.last_pack_cam = [f32::NAN; 3];
        self.last_pack_fwd = [f32::NAN; 3];
        self.last_pack_cam_params = None;
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

        let reg_count = self.registry.len();
        if reg_count == 0 {
            return Err("CSG SceneRegistry is empty".to_string());
        }
        let reg_idx = ((key.scene_idx as usize).saturating_sub(1)).min(reg_count - 1);
        let module = self
            .registry
            .get_by_index(reg_idx)
            .ok_or_else(|| format!("Scene index {reg_idx} not found in SceneRegistry"))?;

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
        let mut built = module.build(&build_cfg);
        module.refresh_materials(&mut built.world, wetness.clamp(0.0, 1.0), tod);
        self.last_mat_state = (wetness.clamp(0.0, 1.0).to_bits(), time_of_day);

        // Upload static compiled library buffers into persistent grow-only VRAM pools (zero
        // alloc/free churn across RoadMode toggles or scene rebuilds once warm).
        let lib = &built.world.library;
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

        // Upload ALL compound AABBs into a persistent grow-only `DeviceBuffer<Aabb>`, then build
        // one cached custom-primitive GAS per compound via fast GPU D2D copy into a persistent
        // 1-box staging slot.
        let build_opts = BuildOptions {
            prefer_fast_trace: true,
            ..Default::default()
        };
        let aabb_host: Vec<Aabb> = lib.compounds.iter().map(sanitized_compound_aabb).collect();
        let mut gases = Vec::with_capacity(aabb_host.len());
        if !aabb_host.is_empty() {
            let need_aabb_grow = self
                .batched_aabbs_buf
                .as_ref()
                .is_none_or(|b| b.len() < aabb_host.len());
            if need_aabb_grow {
                let new_cap = aabb_host.len().max(256);
                let buf = DeviceBuffer::<Aabb>::zeroed(stream, new_cap)
                    .map_err(|e| format!("CSG batched AABB pool alloc failed: {e}"))?;
                self.batched_aabbs_buf = Some(buf);
            }
            if self.aabb_staging_buf.is_none() {
                let staging = DeviceBuffer::<Aabb>::zeroed(stream, 1)
                    .map_err(|e| format!("CSG AABB staging alloc failed: {e}"))?;
                self.aabb_staging_buf = Some(staging);
            }
            let batched_aabbs = self
                .batched_aabbs_buf
                .as_ref()
                .ok_or_else(|| "CSG batched AABB buffer missing".to_string())?;
            let aabb_staging = self
                .aabb_staging_buf
                .as_ref()
                .ok_or_else(|| "CSG AABB staging buffer missing".to_string())?;
            let box_bytes = std::mem::size_of::<Aabb>();
            let total_aabb_bytes = aabb_host.len() * box_bytes;
            let base_ptr = batched_aabbs.cu_deviceptr();
            let dst_ptr = aabb_staging.cu_deviceptr();
            // SAFETY: `batched_aabbs` holds `>= aabb_host.len()` `Aabb`s on `stream`; `Aabb` is
            // 24-byte plain `f32` `DeviceCopy` with zero padding.
            let h2d = unsafe {
                cu::cuMemcpyHtoDAsync_v2(
                    base_ptr,
                    aabb_host.as_ptr().cast(),
                    total_aabb_bytes,
                    stream.cu_stream(),
                )
            };
            if h2d != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!("CSG batched AABB H2D copy failed ({h2d:?})"));
            }
            for i in 0..aabb_host.len() {
                let src_ptr = base_ptr + (i * box_bytes) as u64;
                // SAFETY: `batched_aabbs` owns `>= aabb_host.len()` contiguous `Aabb`s on `stream`
                // and `aabb_staging` owns 1 `Aabb`; both remain alive through `stream.synchronize()`.
                let cp = unsafe {
                    cu::cuMemcpyDtoDAsync_v2(dst_ptr, src_ptr, box_bytes, stream.cu_stream())
                };
                if cp != cu::cudaError_enum_CUDA_SUCCESS {
                    return Err(format!("CSG AABB D2D copy failed ({cp:?})"));
                }
                let gas = Accel::custom(
                    optix_ctx,
                    stream,
                    scratch,
                    &[CustomPrimitives::new(aabb_staging)],
                    build_opts,
                )
                .map_err(|e| format!("CSG compound GAS build failed: {e}"))?;
                gases.push(gas);
            }
            stream
                .synchronize()
                .map_err(|e| format!("CSG GAS sync failed: {e}"))?;
        }

        self.compound_gases = gases;
        self.compound_sbt_offsets = lib
            .compounds
            .iter()
            .map(|c| sbt_offset_for_compound(c.flags))
            .collect();
        self.packed
            .reserve_instances(built.world.total_instance_count().max(256));
        built.reset_crowd_motion_history();
        self.active_reg_idx = Some(reg_idx);
        self.pending_metadata = Some(SceneLoadedMetadata {
            scene_idx: key.scene_idx,
            variant_idx: key.variant_idx,
            viewpoints: std::mem::take(&mut built.viewpoints),
            rails: std::mem::take(&mut built.rails),
        });
        self.scene = Some(built);
        Ok(())
    }
}

/// Exact bit representation of a row-major 3x4 affine transform for deterministic bone-slot matching.
#[inline]
fn xform_to_bits(xf: [f32; 12]) -> [u32; 12] {
    let mut out = [0u32; 12];
    for (dst, &v) in out.iter_mut().zip(&xf) {
        *dst = v.to_bits();
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
        assert_eq!(
            ids,
            vec!["gundo", "gundo_osm", "marina", "sf_house_of_ai", "inspector"]
        );
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
            flags: 1,
            aabb_min: [f32::NEG_INFINITY; 3],
            aabb_max: [f32::INFINITY; 3],
            ..Default::default()
        };
        let u = sanitized_compound_aabb(&unbounded);
        assert_eq!(u.min, [-1000.0; 3]);
        assert_eq!(u.max, [1000.0; 3]);

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
            let mut built = module.build(&SceneBuildConfig::default());
            assert!(
                built.world.total_instance_count() >= 100,
                "{scene_id}: expected rich instance count"
            );
            assert!(
                built.world.library.compounds.len() <= 256,
                "{scene_id}: exceeded 256 GPU GAS budget ({})",
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
        assert_eq!(sbt_offset_for_compound(0), CSG_SBT_OFFSET);
        assert_eq!(
            sbt_offset_for_compound(compound_flags::UNBOUNDED),
            CSG_SBT_OFFSET
        );
        assert_eq!(
            sbt_offset_for_compound(compound_flags::SINGLE_PRIM),
            CSG_SBT_PRIM_OFFSET
        );
        assert_eq!(
            sbt_offset_for_compound(compound_flags::SINGLE_PRIM | compound_flags::BOX_ONLY),
            CSG_SBT_PRIM_OFFSET
        );
    }

    #[test]
    fn perspective_frustum_culling_prunes_offscreen_city_micro_instances() {
        let reg = csg_scenes::default_scene_registry();
        let module = reg.get("sf_house_of_ai").expect("sf_house_of_ai");
        let mut built = module.build(&SceneBuildConfig::default());
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
            "city scenes must route single-prim instances to the lean SBT 5 hit group"
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
        let mut built = module.build(&cfg);
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
        assert!(!host.step_dynamic(0.05, 0.05, cam_eye, true, true));
        assert!(host.crowd_frozen);

        // Unfreezing steps the crowd and populates motion_prev_xforms.
        assert!(host.step_dynamic(0.05, 0.10, cam_eye, false, true));
        assert!(!host.crowd_frozen);
        assert!(host.step_dynamic(0.05, 0.15, cam_eye, false, true));

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
        let count = host
            .export_sdg_json(
                &out_path,
                &cam_frame,
                CameraProjection::Pinhole,
                1280,
                720,
                7,
                0.15,
            )
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
        assert!(!host.step_dynamic(0.05, 0.20, cam_eye, true, true));
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
        let built = module.build(&SceneBuildConfig::default());
        assert!(
            built.traffic.actors.is_empty() && built.traffic.soa_vehicle_count() > 0,
            "gundo_osm must exercise TrafficStateSoA (actors empty, soa_vehicle_count > 0)"
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
            host.step_dynamic(0.0, 1.5, cam_eye, false, true),
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
        });
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
        assert!(host.step_dynamic(0.05, 1.5, [vp.eye.x, vp.eye.y, vp.eye.z], false, true));

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
        });
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
        assert!(host.step_dynamic(1.0 / 60.0, 0.1, [0.0, 2.0, -8.0], false, false));
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
}
