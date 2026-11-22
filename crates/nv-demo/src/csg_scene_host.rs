//! Host-side `csg-rs` scene lifecycle, per-compound OptiX GAS cache, and zero-churn GPU uploads.
//!
//! What: builds `csg-urban` scenes (`Gundo` coastal refinery & highway or any of the 28
//! `InspectorAssetKind` assets), caches one OptiX custom-primitive GAS per compiled `GpuCompound`,
//! uploads flat `CsgSceneView` device buffers once per scene rebuild, and updates dynamic traffic
//! and materials in-place with zero steady-state GPU buffer reallocations.
//! Why: keeps `renderer.rs` focused on the OptiX frame loop while preserving zero-churn
//! performance for 10k-100k+ CSG instances.
//! Used by: `crate::renderer`, `crate::ui`, `crate::main`.

use std::marker::PhantomData;
use std::sync::Arc;

use bytemuck::Pod;
use cuda_bindings as cu;
use cuda_core::{DeviceBuffer, Stream};
use optix::{Aabb, Accel, BuildOptions, CustomPrimitives, DeviceContext, Instance, Scratch};

use csg_core::GpuCompound;
use csg_core::math::Vec3;
use csg_scene::{CameraView, GpuInstance, GpuMaterial, PackedGpuScene, WorldScene};
use csg_urban::{
    GundoParams, GundoTimeOfDay, GundoTraffic, InspectorAssetKind, MaterialPalette,
    build_gundo_scene_with_traffic, build_inspector_scene, step_gundo_traffic,
};

/// SBT hit-group offset for `hit_csg` (`__intersection__csg` + `__closesthit__csg`).
pub const CSG_SBT_OFFSET: u32 = 4;

/// Bitflags packed into `LaunchParams::csg_flags`.
pub mod csg_flags {
    /// CSG scene is active and device pointers are valid.
    pub const ENABLED: u32 = 1 << 0;
    /// Night lighting mode (boosts emissive streetlights, headlights, beacons).
    pub const NIGHT: u32 = 1 << 1;
    /// Wet coastal asphalt puddles enabled.
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

/// Scene structural configuration key that requires a full `WorldScene` + GAS rebuild.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CsgSceneKey {
    /// `0 = Off (Classic)`, `1 = Asset Inspector`, `2 = Gundo`.
    pub mode: u32,
    /// Asset index into `InspectorAssetKind::ALL` (used when `mode == 1`).
    pub asset_idx: u32,
    /// Deterministic world / asset seed.
    pub seed: u32,
}

/// Curated camera preset `(label, eye, look_target)` for `Gundo` coastal scenes.
pub const GUNDO_CAMERA_PRESETS: &[(&str, [f32; 3], [f32; 3])] = &[
    (
        "Highway",
        [-14.0, 5.2, -52.0],
        [10.0, 3.8, 22.0],
    ),
    (
        "Refinery",
        [-24.0, 18.0, -78.0],
        [56.0, 14.0, 4.0],
    ),
    (
        "Seawall",
        [-28.0, 4.4, -36.0],
        [4.0, 3.2, 24.0],
    ),
    (
        "Aerial",
        [-72.0, 46.0, -110.0],
        [36.0, 6.0, 12.0],
    ),
];

/// Returns the `InspectorAssetKind` for a UI index, clamping safely to the catalog length.
pub fn inspector_asset_by_index(idx: u32) -> InspectorAssetKind {
    let all = InspectorAssetKind::ALL;
    all[(idx as usize).min(all.len().saturating_sub(1))]
}

/// Returns `(eye, target)` recommended camera coordinates for an inspector asset index.
pub fn inspector_asset_camera(idx: u32) -> ([f32; 3], [f32; 3]) {
    let (eye, tgt) = inspector_asset_by_index(idx).recommended_camera();
    ([eye.x, eye.y, eye.z], [tgt.x, tgt.y, tgt.z])
}

/// Maps UI `csg_time_of_day` (`0 = Day`, `1 = Sunset`, `2 = Night`) to `GundoTimeOfDay`.
pub const fn time_of_day_from_u32(v: u32) -> GundoTimeOfDay {
    match v {
        0 => GundoTimeOfDay::Day,
        2 => GundoTimeOfDay::Night,
        _ => GundoTimeOfDay::Sunset,
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

fn upload_optional_slice<T: Pod>(
    stream: &Arc<Stream>,
    slice: &[T],
) -> Result<Option<DeviceBuffer<u8>>, String> {
    if slice.is_empty() {
        return Ok(None);
    }
    let bytes: &[u8] = bytemuck::cast_slice(slice);
    DeviceBuffer::from_host(stream, bytes)
        .map(Some)
        .map_err(|e| format!("CSG device buffer upload failed: {e}"))
}

/// Grow-only typed GPU buffer backed by 256-byte-aligned `DeviceBuffer<u8>`, avoiding
/// steady-state CUDA allocations on per-frame updates.
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
            // storage owned by `CsgGpuHost` (`self.packed.instances` or `self.world.materials`)
            // that remains immutable until after `stream.synchronize()`.
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

/// Owns the active `csg-rs` CPU world, per-compound OptiX GAS cache, and flat GPU buffers.
pub struct CsgGpuHost {
    scene_key: CsgSceneKey,
    world: Option<WorldScene>,
    traffic: Option<GundoTraffic>,
    packed: PackedGpuScene,
    /// Per-compound cached OptiX custom-primitive GASes (indexed by `compound_id`).
    compound_gases: Vec<Arc<Accel>>,
    compounds_buf: Option<DeviceBuffer<u8>>,
    ops_buf: Option<DeviceBuffer<u8>>,
    prims_buf: Option<DeviceBuffer<u8>>,
    aux_planes_buf: Option<DeviceBuffer<u8>>,
    guard_aabbs_buf: Option<DeviceBuffer<u8>>,
    repeats_buf: Option<DeviceBuffer<u8>>,
    materials_buf: GrowDeviceBuffer<GpuMaterial>,
    instances_buf: GrowDeviceBuffer<GpuInstance>,
    static_gpu_bytes: usize,
    last_mat_state: (u32, u32),
    last_pack_cam: [f32; 3],
    last_pack_lod_bits: u32,
    last_traffic_bits: u32,
    ias_dirty: bool,
    stats: CsgStats,
}

impl Default for CsgGpuHost {
    fn default() -> Self {
        Self::new()
    }
}

impl CsgGpuHost {
    /// Creates an empty host in `Off (Classic)` mode with zero GPU allocations.
    pub fn new() -> Self {
        Self {
            scene_key: CsgSceneKey::default(),
            world: None,
            traffic: None,
            packed: PackedGpuScene::new(),
            compound_gases: Vec::new(),
            compounds_buf: None,
            ops_buf: None,
            prims_buf: None,
            aux_planes_buf: None,
            guard_aabbs_buf: None,
            repeats_buf: None,
            materials_buf: GrowDeviceBuffer::default(),
            instances_buf: GrowDeviceBuffer::default(),
            static_gpu_bytes: 0,
            last_mat_state: (u32::MAX, u32::MAX),
            last_pack_cam: [f32::NAN; 3],
            last_pack_lod_bits: 0,
            last_traffic_bits: u32::MAX,
            ias_dirty: false,
            stats: CsgStats::default(),
        }
    }

    /// Returns `true` when a CSG scene (`Asset Inspector` or `Gundo`) is loaded and active.
    pub fn is_active(&self) -> bool {
        self.scene_key.mode != 0 && self.world.is_some()
    }

    /// Returns the latest CSG telemetry for the UI.
    pub fn stats(&self) -> CsgStats {
        self.stats
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
            compounds: self.compounds_buf.as_ref().map_or(0, DeviceBuffer::cu_deviceptr),
            ops: self.ops_buf.as_ref().map_or(0, DeviceBuffer::cu_deviceptr),
            prims: self.prims_buf.as_ref().map_or(0, DeviceBuffer::cu_deviceptr),
            aux_planes: self
                .aux_planes_buf
                .as_ref()
                .map_or(0, DeviceBuffer::cu_deviceptr),
            guard_aabbs: self
                .guard_aabbs_buf
                .as_ref()
                .map_or(0, DeviceBuffer::cu_deviceptr),
            repeats: self.repeats_buf.as_ref().map_or(0, DeviceBuffer::cu_deviceptr),
            materials: self.materials_buf.device_ptr(),
            instances: self.instances_buf.device_ptr(),
            instance_count: self.packed.len() as u32,
            material_count: world.materials.len() as u32,
            compound_count: world.library.compounds.len() as u32,
        }
    }

    /// Rebuilds the CSG scene and per-compound GAS cache only when `key` changes,
    /// refreshes GPU materials in-place when `wetness` or `time_of_day` changes,
    /// steps highway traffic, and repacks visible instances when needed.
    ///
    /// Returns `Ok(true)` if the OptiX IAS must be rebuilt this frame.
    #[expect(
        clippy::too_many_arguments,
        reason = "coordinates scene key, material params, camera state, and OptiX resources cleanly"
    )]
    pub fn sync(
        &mut self,
        optix_ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        scratch: &mut Scratch,
        key: CsgSceneKey,
        wetness: f32,
        time_of_day: u32,
        cam_eye: [f32; 3],
        viewport_h: u32,
        vfov_rad: f32,
        lod_bias: f32,
        traffic_time_s: f32,
    ) -> Result<bool, String> {
        let mut rebuilt_scene = false;
        if key != self.scene_key {
            self.rebuild_scene(optix_ctx, stream, scratch, key, wetness, time_of_day)?;
            rebuilt_scene = true;
        }

        if key.mode == 0 {
            let dirty = self.ias_dirty || rebuilt_scene;
            self.ias_dirty = false;
            return Ok(dirty);
        }

        // Fast in-place material palette refresh (zero GAS/scene rebuild!) on wetness/time change.
        let mat_state = (wetness.clamp(0.0, 1.0).to_bits(), time_of_day);
        if mat_state != self.last_mat_state
            && let Some(ref mut world) = self.world
        {
            let tod = time_of_day_from_u32(time_of_day);
            let palette = MaterialPalette::gundo(wetness.clamp(0.0, 1.0), tod.night_factor());
            world.materials = palette.to_material_table();
            self.materials_buf.upload(stream, world.materials.as_slice())?;
            self.last_mat_state = mat_state;
        }

        // Step animated highway traffic in-place (zero heap allocations).
        let traffic_bits = traffic_time_s.to_bits();
        let traffic_moved = self.traffic.is_some() && traffic_bits != self.last_traffic_bits;
        if traffic_moved
            && let (Some(world), Some(traffic)) = (&mut self.world, &self.traffic)
        {
            step_gundo_traffic(world, traffic, traffic_time_s);
            self.last_traffic_bits = traffic_bits;
        }

        // Repack visible instances & upload grow-only instance buffer when scene/traffic/LOD moved.
        let cam_dist_sq = (cam_eye[0] - self.last_pack_cam[0]).powi(2)
            + (cam_eye[1] - self.last_pack_cam[1]).powi(2)
            + (cam_eye[2] - self.last_pack_cam[2]).powi(2);
        let lod_bits = lod_bias.clamp(0.25, 4.0).to_bits();
        let need_pack = rebuilt_scene
            || self.ias_dirty
            || traffic_moved
            || lod_bits != self.last_pack_lod_bits
            || !cam_dist_sq.is_finite()
            || cam_dist_sq > 4.0;

        if need_pack && let Some(ref mut world) = self.world {
            let mut cam_view = CameraView::omnidirectional(
                Vec3::new(cam_eye[0], cam_eye[1], cam_eye[2]),
                1500.0,
                vfov_rad.clamp(0.1, 2.5),
                (viewport_h.max(144)) as f32,
            );
            cam_view.lod_bias = lod_bias.clamp(0.25, 4.0);
            world.pack_visible(&cam_view, &mut self.packed);
            self.instances_buf
                .upload(stream, self.packed.instances_slice())?;

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
            self.last_pack_cam = cam_eye;
            self.last_pack_lod_bits = lod_bits;
            self.ias_dirty = false;
            return Ok(true);
        }

        Ok(false)
    }

    /// Appends all visible CSG instances into `out_instances` using cached per-compound GASes.
    ///
    /// Each OptiX instance receives `id = csg_instance_index` (matching `params.csg_instances[id]`)
    /// and `sbt_offset = CSG_SBT_OFFSET` (`4`).
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
            inst.sbt_offset = CSG_SBT_OFFSET;
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
        self.world = None;
        self.traffic = None;
        self.compound_gases.clear();
        self.compounds_buf = None;
        self.ops_buf = None;
        self.prims_buf = None;
        self.aux_planes_buf = None;
        self.guard_aabbs_buf = None;
        self.repeats_buf = None;
        self.materials_buf.clear();
        self.instances_buf.clear();
        self.static_gpu_bytes = 0;
        self.stats = CsgStats::default();
        self.last_mat_state = (u32::MAX, u32::MAX);
        self.last_pack_cam = [f32::NAN; 3];
        self.last_traffic_bits = u32::MAX;
        self.ias_dirty = true;

        if key.mode == 0 {
            return Ok(());
        }

        let tod = time_of_day_from_u32(time_of_day);
        let (mut world, traffic) = match key.mode {
            1 => {
                let kind = inspector_asset_by_index(key.asset_idx);
                let w = build_inspector_scene(kind, u64::from(key.seed));
                (w, None)
            }
            _ => {
                let params = GundoParams {
                    seed: u64::from(key.seed).max(1),
                    wetness: wetness.clamp(0.0, 1.0),
                    time_of_day: tod,
                    ..GundoParams::default()
                };
                let (w, tr) = build_gundo_scene_with_traffic(&params);
                (w, Some(tr))
            }
        };

        // Ensure inspector scenes also reflect current wetness & time-of-day palette.
        let palette = MaterialPalette::gundo(wetness.clamp(0.0, 1.0), tod.night_factor());
        world.materials = palette.to_material_table();
        self.last_mat_state = (wetness.clamp(0.0, 1.0).to_bits(), time_of_day);

        // Upload static compiled library buffers once per scene rebuild.
        let lib = &world.library;
        self.compounds_buf = upload_optional_slice(stream, &lib.compounds)?;
        self.ops_buf = upload_optional_slice(stream, &lib.ops)?;
        self.prims_buf = upload_optional_slice(stream, &lib.prims)?;
        self.aux_planes_buf = upload_optional_slice(stream, &lib.aux_planes)?;
        self.guard_aabbs_buf = upload_optional_slice(stream, &lib.guard_aabbs)?;
        self.repeats_buf = upload_optional_slice(stream, &lib.repeats)?;
        self.materials_buf.upload(stream, world.materials.as_slice())?;

        self.static_gpu_bytes = std::mem::size_of_val(lib.compounds.as_slice())
            + std::mem::size_of_val(lib.ops.as_slice())
            + std::mem::size_of_val(lib.prims.as_slice())
            + std::mem::size_of_val(lib.aux_planes.as_slice())
            + std::mem::size_of_val(lib.guard_aabbs.as_slice())
            + std::mem::size_of_val(lib.repeats.as_slice());

        // Build & cache one custom-primitive GAS per unique compound so IAS updates never rebuild GASes.
        let build_opts = BuildOptions {
            prefer_fast_trace: true,
            ..Default::default()
        };
        let aabb_host: Vec<Aabb> = lib.compounds.iter().map(sanitized_compound_aabb).collect();
        let mut gases = Vec::with_capacity(aabb_host.len());
        let mut aabb_staging = DeviceBuffer::zeroed(stream, 1)
            .map_err(|e| format!("CSG AABB staging alloc failed: {e}"))?;
        for aabb in &aabb_host {
            aabb_staging
                .copy_from_host(stream, std::slice::from_ref(aabb))
                .map_err(|e| format!("CSG AABB upload failed: {e}"))?;
            let gas = Accel::custom(
                optix_ctx,
                stream,
                scratch,
                &[CustomPrimitives::new(&aabb_staging)],
                build_opts,
            )
            .map_err(|e| format!("CSG compound GAS build failed: {e}"))?;
            gases.push(gas);
        }
        stream
            .synchronize()
            .map_err(|e| format!("CSG GAS sync failed: {e}"))?;

        self.compound_gases = gases;
        self.packed
            .reserve_instances(world.total_instance_count().max(256));
        self.world = Some(world);
        self.traffic = traffic;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inspector_catalog_and_camera_helpers_are_valid() {
        assert!(!InspectorAssetKind::ALL.is_empty());
        for i in 0..InspectorAssetKind::ALL.len() {
            let (eye, tgt) = inspector_asset_camera(i as u32);
            assert!(eye.iter().all(|v| v.is_finite()));
            assert!(tgt.iter().all(|v| v.is_finite()));
            let dx = eye[0] - tgt[0];
            let dy = eye[1] - tgt[1];
            let dz = eye[2] - tgt[2];
            assert!(dx * dx + dy * dy + dz * dz > 0.1);
        }
        // Out-of-range index clamps cleanly without panicking.
        let clamped = inspector_asset_by_index(999_999);
        assert_eq!(clamped, *InspectorAssetKind::ALL.last().unwrap());
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
    fn gundo_scene_and_traffic_step_pack_without_heap_churn() {
        let params = GundoParams::default();
        let (mut world, traffic) = build_gundo_scene_with_traffic(&params);
        assert!(world.total_instance_count() >= 2000);
        assert!(!traffic.actors.is_empty());

        let mut packed = PackedGpuScene::with_capacity(
            world.total_instance_count() * 2,
            world.library.compounds.len(),
        );
        let cam = CameraView::omnidirectional(Vec3::new(-14.0, 5.2, -52.0), 1500.0, 1.0, 720.0);
        world.pack_visible(&cam, &mut packed);
        let cap_before = packed.instances.capacity();
        assert!(!packed.is_empty());

        for step in 1..=5 {
            step_gundo_traffic(&mut world, &traffic, step as f32 * 0.1);
            world.pack_visible(&cam, &mut packed);
            assert_eq!(packed.instances.capacity(), cap_before);
        }
    }
}
