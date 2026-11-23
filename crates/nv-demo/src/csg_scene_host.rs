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
use std::sync::Arc;

use bytemuck::Pod;
use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, DeviceBuffer, Stream};
use optix::{Aabb, Accel, BuildOptions, CustomPrimitives, DeviceContext, Instance, Scratch};

use csg_core::abi::compound_flags;
use csg_core::math::Vec3;
use csg_core::{GpuAabb, GpuCompound, GpuOp, GpuPrim, GpuRepeat, Ray};
use csg_scene::{
    CameraPose as CsgCameraPose, CameraView, GpuInstance, GpuMaterial, PackedGpuScene,
    RailGenConfig, RailSpec, SceneBuildConfig, SceneDescriptor, SceneHints, SceneRegistry,
    SceneTraffic, SceneViewpoint, TimeOfDay, WorldScene, generate_random_rail, inst_flags,
    is_dynamic_inst_id, list_possess_targets, lookup_instance_and_assembly,
    pick_possessable_instance, resolve_possess_pose,
};

use crate::app_events::PossessInfo;

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

/// Owns the active `SceneRegistry`, compiled `WorldScene`, batched per-compound OptiX GAS cache,
/// dynamic traffic controller, and flat GPU buffers.
pub struct CsgGpuHost {
    registry: SceneRegistry,
    scene_key: CsgSceneKey,
    active_reg_idx: Option<usize>,
    world: Option<WorldScene>,
    traffic: Option<SceneTraffic>,
    hints: Option<SceneHints>,
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
    pinned_staging: Option<PinnedStagingBuffer>,
    prev_packed_len: usize,
    prev_dyn_compounds: Vec<u32>,
    static_gpu_bytes: usize,
    last_mat_state: (u32, u32),
    last_pack_cam: [f32; 3],
    last_pack_fwd: [f32; 3],
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
            world: None,
            traffic: None,
            hints: None,
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
            pinned_staging: None,
            prev_packed_len: 0,
            prev_dyn_compounds: Vec::new(),
            static_gpu_bytes: 0,
            last_mat_state: (u32::MAX, u32::MAX),
            last_pack_cam: [f32::NAN; 3],
            last_pack_fwd: [f32::NAN; 3],
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
        self.scene_key.scene_idx != 0 && self.world.is_some()
    }

    /// Returns `true` when the active scene has dynamic moving traffic actors.
    pub fn has_traffic(&self) -> bool {
        self.traffic
            .as_ref()
            .is_some_and(|t| !t.actors.is_empty())
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
        let (Some(world), Some(hints)) = (&self.world, &self.hints) else {
            return Err("Load a CSG scene before generating a random rail".to_string());
        };
        let cfg = RailGenConfig {
            seed,
            ..RailGenConfig::default()
        };
        generate_random_rail(world, hints, seed, &cfg)
            .map_err(|e| format!("Random rail gen failed: {e}"))
    }

    /// Returns references to the active `WorldScene` and `SceneHints`, if a CSG scene is loaded.
    pub fn world_and_hints(&self) -> Option<(&WorldScene, &SceneHints)> {
        match (&self.world, &self.hints) {
            (Some(w), Some(h)) => Some((w, h)),
            _ => None,
        }
    }

    /// Returns the currently packed GPU instance staging buffer.
    pub fn packed(&self) -> &PackedGpuScene {
        &self.packed
    }

    /// Packs the full scene omnidirectionally into `self.packed` and uploads `self.instances_buf`
    /// for city-wide view scoring, marking `ias_dirty` so the next render frame restores frustum culling.
    pub fn prepare_full_scene_for_probe(&mut self, stream: &Arc<Stream>) -> Result<(), String> {
        let Some(ref mut world) = self.world else {
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
        world.pack_all(cam_pos, &mut self.packed);
        self.instances_buf
            .upload(stream, self.packed.instances_slice())?;
        self.ias_dirty = true;
        Ok(())
    }

    /// Casts an exact CSG ray against live dynamic + static instances and enters `Possess` mode.
    pub fn pick_possess(&mut self, origin: [f32; 3], dir: [f32; 3]) -> Result<PossessInfo, String> {
        let Some(ref world) = self.world else {
            return Err("Load a CSG scene before picking an object to possess".to_string());
        };
        let d = Vec3::new(dir[0], dir[1], dir[2]).norm();
        let ray = Ray::new(Vec3::new(origin[0], origin[1], origin[2]), d, 0.05, 1500.0);
        let Some((inst_id, _t, mounts)) = pick_possessable_instance(world, ray) else {
            return Err("No possessable object hit under cursor".to_string());
        };
        let assembly_name = lookup_instance_and_assembly(world, inst_id)
            .map(|(_, asm)| asm.name.clone())
            .unwrap_or_else(|| format!("Object #{inst_id}"));
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

    /// Possesses the next dynamic vehicle (prioritizing `Robotaxi`) or mounted scene landmark.
    pub fn possess_next_vehicle(&mut self) -> Result<PossessInfo, String> {
        let Some(ref world) = self.world else {
            return Err("Load a CSG scene before possessing a vehicle".to_string());
        };
        let mut targets = list_possess_targets(world);
        if targets.is_empty() {
            return Err("Active scene has no mounted vehicles or landmarks".to_string());
        }
        // Sort: dynamic Robotaxis first, then other dynamic actors, then static mounted landmarks.
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

    /// Returns `true` while actively possessing an object or moving vehicle.
    pub fn is_possessing(&self) -> bool {
        self.active_possess.is_some()
    }

    /// Resolves the live world-space camera pose of the possessed body (tracking moving traffic).
    pub fn resolve_active_possess(
        &self,
        free_yaw: f32,
        free_pitch: f32,
        auto_level: bool,
    ) -> Option<CsgCameraPose> {
        let world = self.world.as_ref()?;
        let info = self.active_possess.as_ref()?;
        resolve_possess_pose(
            world,
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
        let Some(ref world) = self.world else {
            return CsgDevicePointers::default();
        };
        CsgDevicePointers {
            compounds: self.compounds_buf.device_ptr(),
            ops: self.ops_buf.device_ptr(),
            prims: self.prims_buf.device_ptr(),
            aux_planes: self.aux_planes_buf.device_ptr(),
            guard_aabbs: self.guard_aabbs_buf.device_ptr(),
            repeats: self.repeats_buf.device_ptr(),
            materials: self.materials_buf.device_ptr(),
            instances: self.instances_buf.device_ptr(),
            instance_count: self.packed.len() as u32,
            material_count: world.materials.len() as u32,
            compound_count: world.library.compounds.len() as u32,
        }
    }

    /// Advances dynamic 3D Frenet traffic in-place to `traffic_time_s` before resolving possessed
    /// vehicle camera mounts so the camera is locked to the exact current-frame vehicle transform.
    pub fn step_traffic(&mut self, traffic_time_s: f32) -> bool {
        let traffic_bits = traffic_time_s.to_bits();
        let moved = self.has_traffic() && traffic_bits != self.last_traffic_bits;
        if moved
            && let (Some(reg_idx), Some(world), Some(traffic)) =
                (self.active_reg_idx, self.world.as_mut(), self.traffic.as_ref())
            && let Some(module) = self.registry.get_by_index(reg_idx)
        {
            module.step_dynamic(world, traffic, traffic_time_s);
            self.last_traffic_bits = traffic_bits;
            self.traffic_dirty = true;
            return true;
        }
        false
    }

    /// Rebuilds the CSG scene and per-compound GAS cache only when `key` changes,
    /// refreshes GPU materials in-place when `wetness` or `time_of_day` changes,
    /// steps 3D Frenet dynamic traffic, and either streams only `dynamic_instances[0..D]` via the
    /// reusable `256 KiB` pinned staging buffer (`CsgSyncResult::TrafficOnly`) or repacks visible
    /// instances with live frustum + small-detail culling (`CsgSyncResult::FullRepack`).
    #[expect(
        clippy::too_many_arguments,
        reason = "coordinates scene key, material params, camera frustum, and OptiX resources cleanly"
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
    ) -> Result<CsgSyncResult, String> {
        if self.pinned_staging.is_none() {
            self.pinned_staging =
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
            && let (Some(reg_idx), Some(world)) = (self.active_reg_idx, self.world.as_mut())
            && let Some(module) = self.registry.get_by_index(reg_idx)
        {
            let tod = time_of_day_from_u32(time_of_day);
            module.refresh_materials(world, wetness.clamp(0.0, 1.0), tod);
            self.materials_buf.upload(stream, world.materials.as_slice())?;
            self.last_mat_state = mat_state;
        }

        // Step animated 3D Frenet traffic in-place (zero heap allocations).
        let traffic_bits = traffic_time_s.to_bits();
        let traffic_moved_now = self.has_traffic() && traffic_bits != self.last_traffic_bits;
        if traffic_moved_now
            && let (Some(reg_idx), Some(world), Some(traffic)) =
                (self.active_reg_idx, self.world.as_mut(), self.traffic.as_ref())
            && let Some(module) = self.registry.get_by_index(reg_idx)
        {
            module.step_dynamic(world, traffic, traffic_time_s);
            self.last_traffic_bits = traffic_bits;
            self.traffic_dirty = true;
        }

        // Distinguish camera/topology repacks from traffic-only frames.
        let cam_dist_sq = (cam.eye[0] - self.last_pack_cam[0]).powi(2)
            + (cam.eye[1] - self.last_pack_cam[1]).powi(2)
            + (cam.eye[2] - self.last_pack_cam[2]).powi(2);
        let fwd_dot = cam.fwd[0] * self.last_pack_fwd[0]
            + cam.fwd[1] * self.last_pack_fwd[1]
            + cam.fwd[2] * self.last_pack_fwd[2];
        let lod_bits = cam.lod_bias.clamp(0.25, 4.0).to_bits();
        let camera_repack_needed = rebuilt_scene
            || self.ias_dirty
            || lod_bits != self.last_pack_lod_bits
            || cam.omnidirectional != self.last_pack_omni
            || !cam_dist_sq.is_finite()
            || cam_dist_sq > 4.0
            || (!cam.omnidirectional && (!fwd_dot.is_finite() || fwd_dot < 0.99));
        let traffic_dirty = self.traffic_dirty;

        if (camera_repack_needed || traffic_dirty)
            && let Some(ref mut world) = self.world
        {
            let mut cam_view = if cam.omnidirectional {
                CameraView::omnidirectional(
                    Vec3::new(cam.eye[0], cam.eye[1], cam.eye[2]),
                    1500.0,
                    cam.vfov_rad.clamp(0.15, 2.7),
                    (cam.viewport_h.max(144)) as f32,
                )
            } else {
                let aspect = (cam.viewport_w.max(1) as f32 / cam.viewport_h.max(1) as f32)
                    .clamp(0.25, 4.0);
                // Pad frustum FOV by 28% so fast turns (~8 deg threshold) and edge reflections never pop,
                // while culling off-screen behind-camera micro-instances (shadow_guard_radius = 120 m
                // retains large behind-camera buildings for accurate shadows).
                let padded_vfov = (cam.vfov_rad * 1.28).clamp(0.15, 2.7);
                CameraView::perspective(
                    Vec3::new(cam.eye[0], cam.eye[1], cam.eye[2]),
                    Vec3::new(cam.fwd[0], cam.fwd[1], cam.fwd[2]),
                    Vec3::new(cam.up[0], cam.up[1], cam.up[2]),
                    padded_vfov,
                    aspect,
                    [0.05, 1500.0],
                    (cam.viewport_h.max(144)) as f32,
                )
            };
            cam_view.lod_bias = cam.lod_bias.clamp(0.25, 4.0);
            world.pack_visible(&cam_view, &mut self.packed);

            let slice = self.packed.instances_slice();
            let dyn_count = slice
                .iter()
                .take_while(|inst| (inst.flags & inst_flags::DYNAMIC) != 0)
                .count();
            let same_topology = !camera_repack_needed
                && dyn_count > 0
                && slice.len() == self.prev_packed_len
                && dyn_count == self.prev_dyn_compounds.len()
                && slice[..dyn_count]
                    .iter()
                    .zip(&self.prev_dyn_compounds)
                    .all(|(inst, &cid)| inst.compound_id == cid);

            let result = if same_topology {
                self.instances_buf.upload_prefix_pinned(
                    stream,
                    self.pinned_staging.as_ref(),
                    &slice[..dyn_count],
                )?;
                CsgSyncResult::TrafficOnly {
                    dynamic_count: dyn_count,
                }
            } else {
                self.instances_buf.upload(stream, slice)?;
                self.prev_packed_len = slice.len();
                self.prev_dyn_compounds.clear();
                self.prev_dyn_compounds.reserve(dyn_count);
                for inst in &slice[..dyn_count] {
                    self.prev_dyn_compounds.push(inst.compound_id);
                }
                CsgSyncResult::FullRepack
            };

            let gpu_bytes = self.static_gpu_bytes
                + self.materials_buf.active_bytes()
                + self.instances_buf.active_bytes();
            self.stats = CsgStats {
                visible_instances: self.packed.len() as u32,
                total_instances: world.total_instance_count() as u32,
                compounds: world.library.compounds.len() as u32,
                primitives: world.library.prims.len() as u32,
                gpu_bytes: gpu_bytes as u64,
            };
            if camera_repack_needed {
                self.last_pack_cam = cam.eye;
                self.last_pack_fwd = cam.fwd;
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
        self.world = None;
        self.traffic = None;
        self.hints = None;
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
        self.prev_packed_len = 0;
        self.prev_dyn_compounds.clear();
        self.static_gpu_bytes = 0;
        self.stats = CsgStats::default();
        self.last_mat_state = (u32::MAX, u32::MAX);
        self.last_pack_cam = [f32::NAN; 3];
        self.last_pack_fwd = [f32::NAN; 3];
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
        let build_cfg = SceneBuildConfig {
            seed: u64::from(key.seed).max(1),
            wetness: wetness.clamp(0.0, 1.0),
            time_of_day: tod,
            variant_idx: key.variant_idx,
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
        self.active_reg_idx = Some(reg_idx);
        self.pending_metadata = Some(SceneLoadedMetadata {
            scene_idx: key.scene_idx,
            variant_idx: key.variant_idx,
            viewpoints: built.viewpoints,
            rails: built.rails,
        });
        self.world = Some(built.world);
        self.traffic = Some(built.traffic);
        self.hints = Some(built.hints);
        Ok(())
    }
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
}
