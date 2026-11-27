//! Centralized `appbus-rs` EventBus, lock-free latest-state snapshot slots, and worker wake
//! primitives decoupling the 60 Hz `egui` UI loop from the asynchronous OptiX render thread.
//!
//! What: defines [`AppBus`] combining three typed `appbus::EventBus` queues (`ui_bus`,
//! `render_bus`, `event_bus`), two coalescing `appbus::StateSlot`s (`state` for UI-to-worker
//! [`RenderStateSnapshot`] and `build_progress` for worker-to-UI `Option<SceneBuildProgress>`),
//! [`EnginePhase`], [`SceneBuildProgress`], and a lost-wakeup-immune [`RenderWake`] condvar.
//! Why: eliminates ad-hoc `mpsc` channels and scattered boolean UI flags, prevents slider/orbit
//! drags from queueing stale render frames, lets the render worker stream live compile/GAS/IAS
//! progress without flooding `event_bus`, and guarantees smooth 60 Hz UI responsiveness even when
//! heavy CSG builds or path-traced frames take seconds.
//! How: UI publishes coalesced snapshots via [`AppBus::publish_render_state`] (waking
//! [`RenderWake`]) and polls `build_progress` in O(1) via [`AppBus::load_build_progress`]
//! (`Arc<Option<SceneBuildProgress>>`, zero string clones); discrete commands/events flow through
//! reused scratch `Vec<BoxedEvent>` buffers.
//! Uses: `appbus`, `csg_scene`, `crate::csg_scene_host`, `crate::flight`, `crate::pick`,
//! `crate::renderer`, `crate::sdg_export`.
//! Used by: `crate::main`, `crate::ui`, `crate::renderer`.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use appbus::{BoxedEvent, EventBus, StateSlot};
use csg_scene::traffic::{
    IDM_ACCEL_MAX, MAX_BRAKE_DECEL, VehicleHazardStatePod, hazard_flags, hazard_state,
};
use csg_scene::{CameraMount, ProbeBackendKind, ViewScore};
use serde::{Deserialize, Serialize};

use crate::csg_scene_host::SceneLoadedMetadata;
use crate::renderer::{CameraFrame, DemoSettings, DlssRrBufferPtrs, FrameReady, OverlayOutPtrs};
use crate::sdg_export::{ActiveCrowdBackend, ActiveTrafficBackend, SdgOccluderBackend};

/// Duration after the last camera or setting change during which interactive fast-preview stays active.
pub const INTERACTION_SETTLE_WINDOW: Duration = Duration::from_millis(150);

/// Active possessed object metadata synced from the render worker to the UI thread.
#[derive(Clone, Debug, PartialEq)]
pub struct PossessInfo {
    /// Encoded static or dynamic instance ID (`DYNAMIC_INST_BIT` marks moving actors).
    pub inst_id: u32,
    /// Human-readable assembly label (e.g. `robotaxi`, `sf_house_of_ai_40_boardman`).
    pub assembly_name: String,
    /// Available camera mounts attached to the possessed object.
    pub mounts: Vec<CameraMount>,
    /// Currently active index inside `mounts`.
    pub mount_idx: usize,
    /// `true` when riding a moving dynamic traffic/marine actor.
    pub is_dynamic: bool,
}

/// Result of a GPU/CPU multi-candidate `Smart View` scoring pass.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SmartViewResult {
    /// Winning camera origin.
    pub eye: [f32; 3],
    /// Winning unit forward direction.
    pub forward: [f32; 3],
    /// Winning unit up vector.
    pub up: [f32; 3],
    /// Winning vertical FOV in degrees.
    pub vfov_deg: f32,
    /// SSOT composition quality breakdown and concrete backend that executed.
    pub score: ViewScore,
    /// Wall-clock scoring time in milliseconds.
    pub elapsed_ms: f32,
    /// Number of candidate viewpoints evaluated.
    pub candidates_evaluated: usize,
}

/// Still image capture format requested from the UI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StillFormat {
    /// Lossless PNG snapshot.
    Png,
    /// GPU-accelerated nvJPEG snapshot.
    Jpeg,
}

/// High-level render worker execution phase surfaced in the bottom-left Viewport status bar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EnginePhase {
    /// Compiling OptiX pipelines and NVRTC modules at startup or hot-reload.
    #[default]
    CompilingOptix,
    /// Building or rebuilding a procedural/OSM CSG world and per-compound GAS cache.
    BuildingCsgScene,
    /// Uploading instance transforms and building/updating the top-level OptiX IAS.
    UpdatingIas,
    /// Steady-state path tracing (UI resolves live `Accumulating (N/M)` vs `Ready`).
    Rendering,
}

impl EnginePhase {
    /// Human-readable phase label for transient build/compile states, or `None` when rendering.
    #[inline]
    pub const fn transient_label(self) -> Option<&'static str> {
        match self {
            Self::CompilingOptix => Some("Compiling OptiX..."),
            Self::BuildingCsgScene => Some("Building CSG Scene..."),
            Self::UpdatingIas => Some("Updating IAS..."),
            Self::Rendering => None,
        }
    }
}

/// Live ADAS / driving-state telemetry for a possessed dynamic traffic vehicle, sourced from a
/// 32-byte asynchronous pinned DMA of `VehicleHazardStatePod` (GPU) or `SceneTraffic::hazards()`
/// (CPU) with zero mid-frame stalls.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PossessAdas {
    /// Dominant driving state (`csg_scene::traffic::hazard_state::*`).
    pub state: u32,
    /// Status flags (`csg_scene::traffic::hazard_flags::*`).
    pub flags: u32,
    /// Triggering pedestrian actor / blocker index (`u32::MAX` when none).
    pub hazard_actor_idx: u32,
    /// Current forward speed in meters per second.
    pub speed_mps: f32,
    /// Commanded longitudinal acceleration in `m/s^2` (positive = throttle, negative = braking).
    pub accel_mps2: f32,
    /// Comfortable stopping distance `v^2 / (2 * IDM_DECEL_COMFORT)` in meters.
    pub stop_dist_m: f32,
    /// Bumper distance to the nearest active hazard in meters (`f32::INFINITY` when clear).
    pub hazard_dist_m: f32,
    /// Effective desired speed limit in meters per second.
    pub limit_mps: f32,
}

impl PossessAdas {
    /// Constructs a [`PossessAdas`] snapshot from the wire-exact 32-byte [`VehicleHazardStatePod`].
    #[inline]
    pub const fn from_pod(pod: VehicleHazardStatePod) -> Self {
        Self {
            state: pod.state,
            flags: pod.flags,
            hazard_actor_idx: pod.hazard_actor_idx,
            speed_mps: pod.speed_mps,
            accel_mps2: pod.accel_mps2,
            stop_dist_m: pod.stop_dist_m,
            hazard_dist_m: pod.hazard_dist_m,
            limit_mps: pod.limit_mps,
        }
    }

    /// Short uppercase ADAS pill badge label for the `PossessToolbar`.
    #[inline]
    pub const fn badge_label(self) -> &'static str {
        match self.state {
            hazard_state::CRUISE => "CRUISE",
            hazard_state::FOLLOW => "FOLLOW",
            hazard_state::RED_LIGHT => "RED LIGHT",
            hazard_state::STOP_SIGN => "STOP",
            hazard_state::YIELD_JUNCTION => "YIELD",
            hazard_state::PED_YIELD => "YIELD PED",
            hazard_state::PED_AEB => "AEB BRAKE",
            _ => "DRIVE",
        }
    }

    /// Severity bucket for color-coding (`0` = nominal cruise/follow, `1` = yield/signal/stop,
    /// `2` = emergency pedestrian AEB).
    #[inline]
    pub const fn severity(self) -> u8 {
        match self.state {
            hazard_state::PED_AEB => 2,
            hazard_state::RED_LIGHT
            | hazard_state::STOP_SIGN
            | hazard_state::YIELD_JUNCTION
            | hazard_state::PED_YIELD => 1,
            _ => 0,
        }
    }

    /// Current forward speed in km/h (sanitized to `>= 0.0`).
    #[inline]
    pub fn speed_kmh(self) -> f32 {
        if self.speed_mps.is_finite() && self.speed_mps > 0.0 {
            self.speed_mps * 3.6
        } else {
            0.0
        }
    }

    /// Effective speed limit in km/h (sanitized to `>= 0.0`).
    #[inline]
    pub fn limit_kmh(self) -> f32 {
        if self.limit_mps.is_finite() && self.limit_mps > 0.0 {
            self.limit_mps * 3.6
        } else {
            0.0
        }
    }

    /// `true` when vehicle brake lights are active (`hazard_flags::BRAKE_LIGHT`).
    #[inline]
    pub const fn brake_light_on(self) -> bool {
        (self.flags & hazard_flags::BRAKE_LIGHT) != 0
    }

    /// `true` when `hazard_actor_idx` identifies an active pedestrian conflict.
    #[inline]
    pub const fn has_ped_hazard(self) -> bool {
        (self.flags & hazard_flags::HAZARD_ACTOR_VALID) != 0
    }

    /// Signed throttle/brake command normalized to `[-1.0, +1.0]` for the compact `THR/BRK` bar
    /// (`+1.0` = full `IDM_ACCEL_MAX` throttle, `-1.0` = full `MAX_BRAKE_DECEL` braking).
    #[inline]
    pub fn throttle_brake_norm(self) -> f32 {
        if !self.accel_mps2.is_finite() {
            return 0.0;
        }
        if self.accel_mps2 >= 0.0 {
            (self.accel_mps2 / IDM_ACCEL_MAX.max(1e-3)).clamp(0.0, 1.0)
        } else {
            (self.accel_mps2 / MAX_BRAKE_DECEL.max(1e-3)).clamp(-1.0, 0.0)
        }
    }
}

/// Zero-allocation, `Copy` snapshot of live Crowd, Locomotion, and Traffic simulation telemetry
/// delivered from the OptiX/CUDA worker to the `Telemetry` UI section each frame.
///
/// Every metric comes strictly from host-resident scene metadata, CPU step timers, or the
/// existing 12-byte async pinned `CrowdGpuStatus` DMA — never a blocking GPU readback.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SimTelemetry {
    /// Concrete backend that executed the latest crowd step (`None`, `Cpu`, `Cuda`).
    pub crowd_backend: ActiveCrowdBackend,
    /// Active `PlanKind::Crowd` version (`2` when CUDA plan is active, `0` otherwise).
    pub crowd_plan_version: u32,
    /// Total CUDA kernel dispatches per crowd step (`plan.frame.len() + 2` bridge kernels).
    pub crowd_kernel_count: u32,
    /// `true` when CUDA crowd PTX compiled with `nvrtc::MathMode::StrictF32` (else `Fast`).
    pub crowd_strict_f32: bool,
    /// Total articulated crowd actors currently simulated (`scene.crowd.actor_count()`).
    pub crowd_actors: u32,
    /// Actor counts by `csg_actors::ArchetypeKind`:
    /// `[0]=HumanHero, [1]=HumanCrowd, [2]=RoadSignStickman, [3]=Quadruped`.
    pub crowd_archetypes: [u32; 4],
    /// Total articulated bone slots across all actors (`scene.crowd.bone_count()`).
    pub crowd_bones: u32,
    /// Total world joints tracked for FK/IK and SDG keypoints (`scene.crowd.world_joints.len()`).
    pub crowd_joints: u32,
    /// Static parent-to-child skeleton edges uploaded for zero-copy GPU skeleton overlays.
    pub crowd_skeleton_edges: u32,
    /// Total generalized velocity DOFs (`sum(topology.nv)` across all actors).
    pub crowd_dofs: u32,
    /// Navigation walkways registered in the active scene (`scene.hints.walkways.len()`).
    pub crowd_walkways: u32,
    /// Steering obstacles uploaded in `CrowdSteerScene` (`0` when CUDA crowd is inactive).
    pub crowd_obstacles: u32,
    /// Latest articulated crowd step duration (CPU wall time or CUDA enqueue time) in ms.
    pub crowd_step_ms: f32,
    /// Total VRAM allocated by `CudaCrowdBackend` (`GpuFramePlan` slots + bridge buffers).
    pub crowd_gpu_bytes: u64,
    /// `true` when `Freeze Pose` holds dynamic crowd and traffic poses static.
    pub pose_frozen: bool,
    /// Effective dynamic simulation rate multiplier (`time_scale * csg_traffic_speed`, `0.0` if frozen).
    pub sim_rate: f32,
    /// Resolved effective crowd LOD distances `[lod1_m, lod2_m, cull_m]`.
    pub crowd_lod_dists_m: [f32; 3],
    /// Crowd LOD mode: `0 = Auto`, `1 = Forced LOD0`, `2 = Forced LOD1`, `3 = Forced LOD2`, `4 = Off`.
    pub crowd_lod_mode: u8,
    /// Cumulative ground-probe misses reported asynchronously via `CrowdGpuStatus` (12 B pinned DMA).
    pub crowd_ground_misses: u64,
    /// Latest `CrowdFault` code from `CrowdGpuStatus` (`0` = `CROWD_FAULT_NONE`).
    pub crowd_gpu_fault: u32,
    /// Cumulative amortized full OptiX IAS rebuilds triggered by `CUDA_IAS_REBUILD_DISP_M` (8 m).
    pub crowd_ias_rebuilds: u32,
    /// `true` when `PlanKind::Camera` v1 (`GpuCameraComposer`) composed the possessed camera in VRAM.
    pub gpu_camera_active: bool,
    /// Concrete backend that executed the latest traffic step (`None`, `Cpu`, `Cuda`).
    pub traffic_backend: ActiveTrafficBackend,
    /// Active `PlanKind::Traffic` version (`1` when CUDA traffic plan is active, `0` otherwise).
    pub traffic_plan_version: u32,
    /// Total CUDA kernel dispatches per traffic step (`plan.frame.len() + 2` bridge kernels).
    pub traffic_kernel_count: u32,
    /// Total VRAM allocated by `CudaTrafficBackend` (`GpuFramePlan` slots + bridge buffers).
    pub traffic_gpu_bytes: u64,
    /// `true` when the active scene has a `SceneTraffic` lane-graph driver.
    pub traffic_has_lane_graph: bool,
    /// `true` when traffic simulation is actively stepping this frame (not paused or frozen).
    pub traffic_stepping: bool,
    /// Total traffic vehicles (`scene.traffic.vehicle_count()`).
    pub traffic_vehicles: u32,
    /// Vehicles currently moving (`speed_mps > 0.25`).
    pub traffic_moving: u32,
    /// Mean vehicle speed in meters/second across all traffic vehicles.
    pub traffic_avg_speed_mps: f32,
    /// Peak vehicle speed in meters/second across all traffic vehicles.
    pub traffic_max_speed_mps: f32,
    /// Non-crowd dynamic CSG instances (vehicle bodies + rotating wheels) streamed via pinned DMA.
    pub traffic_dyn_instances: u32,
    /// Traffic step duration (CPU wall time or CUDA enqueue time) in milliseconds (`0.0` when paused).
    pub traffic_step_ms: f32,
    /// Cumulative vehicle network respawns (`TrafficRuntimeStats::respawns`).
    pub traffic_respawns: u64,
    /// Cumulative stale junction reservation releases (`TrafficRuntimeStats::stale_reservation_releases`).
    pub traffic_stale_releases: u64,
    /// Cumulative circular-wait deadlock relocations (`TrafficRuntimeStats::deadlock_relocations`).
    pub traffic_deadlock_relocations: u64,
    /// Vehicles currently in emergency pedestrian AEB (`TrafficGlobalsPod::aeb_active_count`).
    pub traffic_aeb_active: u32,
    /// Vehicles currently yielding to pedestrians (`TrafficGlobalsPod::yield_active_count`).
    pub traffic_yield_active: u32,
    /// Cumulative amortized full OptiX IAS rebuilds triggered by GPU traffic displacement.
    pub traffic_ias_rebuilds: u32,
}

impl SimTelemetry {
    /// Returns `true` when the active scene contains articulated crowd actors.
    #[inline]
    pub const fn has_crowd(self) -> bool {
        self.crowd_actors > 0
    }

    /// Returns `true` when the active scene contains dynamic traffic vehicles.
    #[inline]
    pub const fn has_traffic(self) -> bool {
        self.traffic_vehicles > 0
    }

    /// Returns `true` when either crowd or traffic simulation is present.
    #[inline]
    pub const fn has_any(self) -> bool {
        self.has_crowd() || self.has_traffic()
    }

    /// Vehicles currently stopped at signals or queued behind leaders (`speed <= 0.25 m/s`).
    #[inline]
    pub const fn traffic_stopped(self) -> u32 {
        self.traffic_vehicles.saturating_sub(self.traffic_moving)
    }

    /// Mean vehicle speed converted to km/h.
    #[inline]
    pub fn traffic_avg_kmh(self) -> f32 {
        if self.traffic_avg_speed_mps.is_finite() && self.traffic_avg_speed_mps > 0.0 {
            self.traffic_avg_speed_mps * 3.6
        } else {
            0.0
        }
    }

    /// Peak vehicle speed converted to km/h.
    #[inline]
    pub fn traffic_max_kmh(self) -> f32 {
        if self.traffic_max_speed_mps.is_finite() && self.traffic_max_speed_mps > 0.0 {
            self.traffic_max_speed_mps * 3.6
        } else {
            0.0
        }
    }

    /// Effective articulated bone transforms synthesized per second (`0.0` when frozen or idle).
    #[inline]
    pub fn bones_per_sec(self, rt_fps: f32) -> f32 {
        if self.pose_frozen
            || self.crowd_bones == 0
            || !self.sim_rate.is_finite()
            || self.sim_rate <= 0.0
            || !rt_fps.is_finite()
            || rt_fps <= 0.0
        {
            return 0.0;
        }
        self.crowd_bones as f32 * rt_fps
    }

    /// Formats a compact actor archetype summary into `buf` (e.g. `"184 (168 ped / 16 pet)"`).
    pub fn write_archetypes_compact(self, buf: &mut String) {
        buf.clear();
        let _ = write!(buf, "{}", self.crowd_actors);
        let labels = ["hero", "ped", "sign", "pet"];
        let mut wrote_any = false;
        for (count, tag) in self.crowd_archetypes.into_iter().zip(labels) {
            if count == 0 {
                continue;
            }
            if !wrote_any {
                buf.push_str(" (");
                wrote_any = true;
            } else {
                buf.push_str(" / ");
            }
            let _ = write!(buf, "{count} {tag}");
        }
        if wrote_any {
            buf.push(')');
        }
    }

    /// Appends the resolved crowd LOD policy onto `buf` (e.g. `"Auto 22/75/180m"`).
    pub fn append_lod_summary(self, buf: &mut String) {
        match self.crowd_lod_mode {
            1 => buf.push_str("Forced LOD0"),
            2 => buf.push_str("Forced LOD1"),
            3 => buf.push_str("Forced LOD2"),
            4 => buf.push_str("LOD Off"),
            _ => {
                let [l1, l2, cull] = self.crowd_lod_dists_m;
                let _ = write!(buf, "Auto {l1:.0}/{l2:.0}/{cull:.0}m");
            }
        }
    }

    /// Formats a byte count compactly into `buf` (clearing `buf` first).
    pub fn write_bytes_compact(bytes: u64, buf: &mut String) {
        buf.clear();
        Self::append_bytes_compact(bytes, buf);
    }

    /// Appends a compact byte count onto `buf` (`"4.20 MiB"`, `"320.5 KiB"`, or `"0 B"`).
    pub fn append_bytes_compact(bytes: u64, buf: &mut String) {
        const MIB: f64 = 1024.0 * 1024.0;
        const KIB: f64 = 1024.0;
        if bytes >= 1024 * 1024 {
            let _ = write!(buf, "{:.2} MiB", bytes as f64 / MIB);
        } else if bytes >= 1024 {
            let _ = write!(buf, "{:.1} KiB", bytes as f64 / KIB);
        } else {
            let _ = write!(buf, "{bytes} B");
        }
    }
}

/// Ocean wave simulation backend selected for `CSG_PROC_WATER` ocean surfaces.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OceanMode {
    /// 3-cascade 256x256 JONSWAP + TMA spectral ocean synthesized via `cufft`.
    #[default]
    CuFft,
    /// Pure analytical multi-octave directional Gerstner / dispersion wave sum.
    Analytical,
}

impl OceanMode {
    /// Every ocean simulation mode in UI display order.
    pub const ALL: [Self; 2] = [Self::CuFft, Self::Analytical];

    /// Human-readable UI label for this ocean simulation mode.
    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::CuFft => "cuFFT Spectral",
            Self::Analytical => "Analytical",
        }
    }
}

/// Shared Single Source of Truth ocean wave and dielectric water optics parameters dispatched via
/// [`RenderControl::SetOceanParams`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OceanParams {
    /// Spectral `cuFFT` 3-cascade synthesis vs analytical Gerstner wave sum.
    pub mode: OceanMode,
    /// 10 m wind speed in m/s (`1.0..=25.0`; default `9.5`).
    pub wind_speed: f32,
    /// Dominant wind / wave propagation azimuth in degrees (`0.0..=360.0`; default `245.0`).
    pub wind_dir_deg: f32,
    /// Horizontal Gerstner / Jacobian choppiness factor (`0.0..=2.0`; default `1.15`).
    pub choppiness: f32,
    /// Long-fetch Pacific swell energy blend (`0.0..=1.0`; default `0.65`).
    pub swell: f32,
    /// Whitecap & shoreline contact foam intensity multiplier (`0.0..=2.0`; default `1.0`).
    pub foam: f32,
    /// Beer-Lambert optical water clarity multiplier (`0.25..=3.0`; default `1.0`).
    pub clarity: f32,
}

impl Default for OceanParams {
    fn default() -> Self {
        Self {
            mode: OceanMode::CuFft,
            wind_speed: 9.5,
            wind_dir_deg: 245.0,
            choppiness: 1.15,
            swell: 0.65,
            foam: 1.0,
            clarity: 1.0,
        }
    }
}

impl OceanParams {
    /// Clamps every field into its valid physical / UI range, replacing non-finite values with
    /// defaults.
    #[must_use]
    pub fn sanitized(self) -> Self {
        let d = Self::default();
        let clamp = |v: f32, lo: f32, hi: f32, fb: f32| {
            if v.is_finite() { v.clamp(lo, hi) } else { fb }
        };
        Self {
            mode: self.mode,
            wind_speed: clamp(self.wind_speed, 1.0, 25.0, d.wind_speed),
            wind_dir_deg: if self.wind_dir_deg.is_finite() {
                self.wind_dir_deg.rem_euclid(360.0)
            } else {
                d.wind_dir_deg
            },
            choppiness: clamp(self.choppiness, 0.0, 2.0, d.choppiness),
            swell: clamp(self.swell, 0.0, 1.0, d.swell),
            foam: clamp(self.foam, 0.0, 2.0, d.foam),
            clarity: clamp(self.clarity, 0.25, 3.0, d.clarity),
        }
    }
}

/// Where the currently loaded CSG scene came from (`Idle`, freshly built, in-memory LRU, or
/// BLAKE3-verified disk cache).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SceneLoadSource {
    /// No CSG scene has been loaded yet (or cache was just cleared).
    #[default]
    None,
    /// Generated fresh on the CPU via `SceneModule::build_with_progress`.
    Built,
    /// Restored from the in-memory `SceneBuilt` LRU cache.
    MemoryHit,
    /// Restored from the BLAKE3-verified on-disk `SceneDiskCache`.
    DiskHit,
}

impl SceneLoadSource {
    /// Human-readable label shown in tooltips and diagnostics.
    #[inline]
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "Idle",
            Self::Built => "Built",
            Self::MemoryHit => "Memory Hit",
            Self::DiskHit => "Disk Cache Hit",
        }
    }
}

/// Live on-disk and in-memory CSG scene cache telemetry emitted via [`RenderEvent::SceneCacheStats`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SceneCacheStats {
    /// Number of committed `.csgc` scene files currently stored on disk.
    pub entries: usize,
    /// Combined size of all committed `.csgc` scene files on disk in bytes.
    pub total_bytes: u64,
    /// `true` when the latest scene load hit either the memory LRU or on-disk cache.
    pub last_hit: bool,
    /// Exact provenance of the latest scene load.
    pub last_source: SceneLoadSource,
}

/// Window length for the moving-average build ETA estimator (`SMA-10`).
pub const ETA_SMA_WINDOW: usize = 10;

/// Zero-allocation 10-step moving average (`SMA-10`) ETA estimator over completed monotonic
/// progress intervals (`dt_secs / delta_units`).
///
/// Returns `None` until at least 2 intervals have completed so early single-tick startup spikes
/// never flash misleading ETAs; once warmed up, predicts remaining seconds smoothly from the
/// rolling mean over the last `<= 10` intervals.
#[derive(Clone, Copy, Debug)]
pub struct BuildEtaTracker {
    rates: [f32; ETA_SMA_WINDOW],
    head: usize,
    count: usize,
    last_progress: f32,
    last_at: Instant,
}

impl BuildEtaTracker {
    /// Creates a fresh tracker anchored at `started_at` with zero completed units.
    #[must_use]
    pub const fn new(started_at: Instant) -> Self {
        Self {
            rates: [0.0; ETA_SMA_WINDOW],
            head: 0,
            count: 0,
            last_progress: 0.0,
            last_at: started_at,
        }
    }

    /// Number of completed intervals currently stored in the rolling window (`0..=10`).
    #[cfg(test)]
    #[inline]
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        self.count
    }

    /// Records progress `completed_units` out of `total_units` at timestamp `now` and returns the
    /// estimated remaining seconds (`None` while `< 2` intervals have completed or `total_units <= 0`).
    pub fn record(&mut self, completed_units: f32, total_units: f32, now: Instant) -> Option<f32> {
        if !completed_units.is_finite() || !total_units.is_finite() || total_units <= 0.0 {
            return None;
        }
        let cur = completed_units.clamp(0.0, total_units);
        let delta = cur - self.last_progress;
        if delta > 1.0e-6 {
            let dt = now.saturating_duration_since(self.last_at).as_secs_f32();
            if dt.is_finite() && dt >= 0.0 {
                let secs_per_unit = dt / delta;
                if secs_per_unit.is_finite() {
                    self.rates[self.head] = secs_per_unit;
                    self.head = (self.head + 1) % ETA_SMA_WINDOW;
                    if self.count < ETA_SMA_WINDOW {
                        self.count += 1;
                    }
                }
            }
            self.last_progress = cur;
            self.last_at = now;
        }
        if self.count < 2 {
            return None;
        }
        let remaining = (total_units - cur).max(0.0);
        if remaining <= 1.0e-6 {
            return Some(0.0);
        }
        let sum: f32 = self.rates[..self.count].iter().copied().sum();
        let avg_secs_per_unit = sum / (self.count as f32);
        let eta = remaining * avg_secs_per_unit;
        if eta.is_finite() && eta >= 0.0 {
            Some(eta)
        } else {
            None
        }
    }
}

/// Live progress snapshot published by the OptiX render worker while compiling pipelines,
/// generating a CSG world, building per-compound OptiX GASes, or rebuilding the top-level IAS.
#[derive(Clone, Debug, PartialEq)]
pub struct SceneBuildProgress {
    /// Active transient engine phase (`CompilingOptix`, `BuildingCsgScene`, or `UpdatingIas`).
    pub phase: EnginePhase,
    /// Human-readable target scene or pipeline name (e.g. `"Gundo OSM"`, `"OptiX 9 Pipeline"`).
    pub scene_name: String,
    /// Current substage label (e.g. `"Generating CSG scene..."`, `"Building OptiX GAS"`).
    pub stage: String,
    /// Completed work items in the current substage (`0` when indeterminate).
    pub done: u32,
    /// Total work items in the current substage (`0` when indeterminate -> animated bar).
    pub total: u32,
    /// Monotonic timestamp when the current build/compile operation started.
    pub started_at: Instant,
    /// 1-based macro step index within the overall scene build (`0` when unspecified).
    pub step_idx: u32,
    /// Total macro step count across the overall scene build (`0` when unspecified).
    pub step_count: u32,
    /// Overall clamped completion fraction across all build steps in `[0.0, 1.0]`.
    pub overall_fraction: Option<f32>,
    /// Estimated remaining seconds from the 10-step moving average (`None` while warming up).
    pub eta_secs: Option<f32>,
}

impl SceneBuildProgress {
    /// Creates a new progress snapshot for `phase`.
    pub fn new(
        phase: EnginePhase,
        scene_name: impl Into<String>,
        stage: impl Into<String>,
        done: u32,
        total: u32,
        started_at: Instant,
    ) -> Self {
        Self {
            phase,
            scene_name: scene_name.into(),
            stage: stage.into(),
            done,
            total,
            started_at,
            step_idx: 0,
            step_count: 0,
            overall_fraction: None,
            eta_secs: None,
        }
    }

    /// Attaches a 1-based macro step counter (`step_idx / step_count`) to this snapshot.
    #[inline]
    #[must_use]
    pub fn with_step(mut self, step_idx: u32, step_count: u32) -> Self {
        self.step_idx = step_idx;
        self.step_count = step_count;
        self
    }

    /// Attaches an overall completion fraction across all steps (sanitized to `[0.0, 1.0]`).
    #[inline]
    #[must_use]
    pub fn with_overall(mut self, fraction: Option<f32>) -> Self {
        self.overall_fraction = fraction.and_then(|f| f.is_finite().then(|| f.clamp(0.0, 1.0)));
        self
    }

    /// Attaches an estimated remaining duration in seconds (sanitized to finite `>= 0.0`).
    #[inline]
    #[must_use]
    pub fn with_eta(mut self, eta_secs: Option<f32>) -> Self {
        self.eta_secs = eta_secs.and_then(|s| (s.is_finite() && s >= 0.0).then_some(s));
        self
    }

    /// Returns the clamped completion fraction in `[0.0, 1.0]` when `total > 0`, or `None`
    /// when the current substage is indeterminate (`total == 0`).
    #[inline]
    pub fn fraction(&self) -> Option<f32> {
        if self.total == 0 {
            None
        } else {
            let clamped = self.done.min(self.total);
            Some((clamped as f32 / self.total as f32).clamp(0.0, 1.0))
        }
    }

    /// Returns the best available determinate progress fraction (`overall_fraction`, falling back
    /// to substage [`Self::fraction`]).
    #[inline]
    pub fn effective_fraction(&self) -> Option<f32> {
        self.overall_fraction.or_else(|| self.fraction())
    }

    /// Elapsed wall-clock seconds since `started_at` (never negative).
    #[inline]
    pub fn elapsed_secs(&self, now: Instant) -> f32 {
        now.saturating_duration_since(self.started_at).as_secs_f32()
    }

    /// Appends `secs` formatted as `MM:SS.d` into `out` (sanitizing non-finite or negative values
    /// and avoiding `"00:60.0"` rounding rollovers).
    pub fn write_elapsed(out: &mut String, secs: f32) {
        let safe = if secs.is_finite() && secs > 0.0 {
            f64::from(secs)
        } else {
            0.0
        };
        let total_tenths = (safe * 10.0).floor() as u64;
        let mins = total_tenths / 600;
        let rem_secs = (total_tenths % 600) / 10;
        let tenths = total_tenths % 10;
        let Ok(()) = write!(out, "{mins:02}:{rem_secs:02}.{tenths}") else {
            unreachable!("writing to String cannot fail");
        };
    }

    /// Formats `secs` as `MM:SS.d`.
    pub fn format_elapsed(secs: f32) -> String {
        let mut out = String::with_capacity(8);
        Self::write_elapsed(&mut out, secs);
        out
    }

    /// Appends `secs` formatted as `~MM:SS.d` into `out`.
    pub fn write_eta(out: &mut String, secs: f32) {
        out.push('~');
        Self::write_elapsed(out, secs);
    }

    /// Formats `secs` as `~MM:SS.d`.
    pub fn format_eta(secs: f32) -> String {
        let mut out = String::with_capacity(9);
        Self::write_eta(&mut out, secs);
        out
    }

    /// Appends `"Step k/N"` into `out` when `step_count > 0` and returns `true`.
    pub fn write_step_prefix(&self, out: &mut String) -> bool {
        if self.step_count == 0 {
            return false;
        }
        let k = self.step_idx.clamp(1, self.step_count);
        let Ok(()) = write!(out, "Step {k}/{}", self.step_count) else {
            unreachable!("writing to String cannot fail");
        };
        true
    }

    /// Formats `"Step k/N"` when `step_count > 0`, or `None` otherwise.
    #[cfg(test)]
    pub fn format_step_prefix(&self) -> Option<String> {
        if self.step_count == 0 {
            return None;
        }
        let mut out = String::with_capacity(14);
        let _ = self.write_step_prefix(&mut out);
        Some(out)
    }

    /// Appends the current substage label (prefixed with `"Step k/N | "` when step metadata is
    /// present, plus `" (done/total)"` when determinate and not already embedded in `self.stage`)
    /// into `out`.
    pub fn write_stage_line(&self, out: &mut String) {
        if self.write_step_prefix(out) {
            out.push_str(" | ");
        }
        out.push_str(&self.stage);
        if self.total > 0 && !stage_has_inline_counter(&self.stage) {
            let clamped = self.done.min(self.total);
            let Ok(()) = write!(out, " ({clamped}/{})", self.total) else {
                unreachable!("writing to String cannot fail");
            };
        }
    }

    /// Formats the current substage label (plus step prefix and `(done/total)` when determinate).
    pub fn format_stage_line(&self) -> String {
        let mut out = String::with_capacity(self.stage.len() + 28);
        self.write_stage_line(&mut out);
        out
    }

    /// Appends a compact single-line summary (`"<scene>: [Step k/N | ]<stage> [(done/total)] [MM:SS.d[ | ETA ~MM:SS.d]]"`)
    /// into `out` for the viewport status bar.
    pub fn write_status_summary(&self, out: &mut String, elapsed_s: f32) {
        if !self.scene_name.is_empty() {
            out.push_str(&self.scene_name);
            out.push_str(": ");
        }
        self.write_stage_line(out);
        out.push_str(" [");
        Self::write_elapsed(out, elapsed_s);
        if let Some(eta) = self.eta_secs {
            out.push_str(" | ETA ");
            Self::write_eta(out, eta);
        }
        out.push(']');
    }

    /// Formats a compact single-line summary for the viewport status bar.
    #[cfg(test)]
    pub fn format_status_summary(&self, elapsed_s: f32) -> String {
        let mut out = String::with_capacity(self.scene_name.len() + self.stage.len() + 24);
        self.write_status_summary(&mut out, elapsed_s);
        out
    }
}

/// Returns `true` when `stage` already ends with a parenthesized `"(<digits>/<digits>)"`
/// sub-phase counter (e.g. `"Grounding objects 1/2 (1240/3480)"`), avoiding duplicate counter
/// suffixes when the outer weighted scale (`done/total`) is already shown in the bar and `%`.
fn stage_has_inline_counter(stage: &str) -> bool {
    let Some(inner) = stage
        .strip_suffix(')')
        .and_then(|s| s.rsplit_once('(').map(|(_, tail)| tail))
    else {
        return false;
    };
    let Some((lhs, rhs)) = inner.split_once('/') else {
        return false;
    };
    !lhs.is_empty()
        && !rhs.is_empty()
        && lhs.bytes().all(|b| b.is_ascii_digit())
        && rhs.bytes().all(|b| b.is_ascii_digit())
}

/// Typed UI commands emitted by `ui.rs` and consumed cleanly by the main application loop.
#[derive(Clone, Debug, PartialEq)]
pub enum UiCommand {
    /// Clear progressive path-tracing and DLSS temporal accumulation.
    ResetAccum,
    /// Save a still frame (PNG or JPEG) using the active `CaptureSource`.
    SaveStill(StillFormat),
    /// Export exact 2D/3D actor keypoints, bone-OBB bounding boxes, and occlusion labels as JSON.
    ExportSdgJson,
    /// Write the accumulated rail Flight stats frames as a perf CSV (`--bench-rail` schema).
    ExportFlightCsv,
    /// Start or stop MP4 video recording.
    ToggleRecord,
    /// Recompile `raytracer.cu` via NVRTC on the render worker.
    ReloadShader,
    /// User selected a new CSG scene or variant (pauses rail and requests auto-framing on load).
    SelectScene { scene_idx: u32, variant_idx: u32 },
    /// User switched the live 4-way CSG road geometry mode (`0=3D, 1=C1, 2=Hybrid, 3=F-Rep`).
    SetRoadMode(u8),
    /// User switched the live CSG ground kerb representation (`0=In-Tile F-Rep, 1=Sweep Tile`).
    SetGroundKerbMode(u8),
    /// Toggle `Freeze Pose` (pauses dynamic traffic/crowd stepping for clean multi-spp SDG accumulation).
    ToggleFreezePose,
    /// Typed Flight Lab / Flight Player command (`crate::flight::FlightCommand`).
    Flight(crate::flight::FlightCommand),
    /// Snap `CameraDamper` immediately onto the current camera pose (teleport / preset / scrub).
    SnapCamera,
    /// Pick the object under the click (`crate::pick`): select it or possess it.
    Pick(crate::pick::PickRequest),
    /// Jump straight into the next moving vehicle (prioritizing `Robotaxi`) with 1 click.
    PossessNextVehicle,
    /// Switch the active possessed object to a specific `CameraMount` index.
    SelectPossessMount { mount_idx: usize },
    /// Cycle the active possessed object's `CameraMount` by `delta` (`-1` for `[`, `+1` for `]`).
    CyclePossessMount { delta: i32 },
    /// Detach cleanly from the possessed object back to free camera control.
    EjectPossess,
    /// Score 64 city viewpoints on the selected `ProbeBackendKind` and glide to the highest-scoring view.
    RunSmartView { seed: u64, backend: ProbeBackendKind },
    /// Trigger an interactive poke impulse on the nearest actor (or all actors in the `Inspector` crowd showcase).
    PokeCrowd { impulse: f32, all: bool },
    /// Spawn (or toggle) the 2,000-actor GPU/CPU crowd stress grid around the camera.
    SpawnCrowdStress { target_count: usize },
    /// Launch a multi-frame OpenEXR + SDG JSON + `manifest.json` dataset burst capture (`16 / 30 / 60` frames).
    StartSdgBurst { frames: u32 },
    /// Abort a running SDG dataset burst (forwarded as [`RenderControl::CancelSdgBurst`]).
    CancelSdgBurst,
    /// Switch CSG time-of-day (`0 = Day`, `1 = Sunset`, `2 = Night`), smoothly transitioning when enabled.
    SetTimeOfDay(u32),
    /// Clear both the in-memory LRU and on-disk compiled CSG scene cache (`RenderControl::ClearSceneCache`).
    ClearSceneCache,
}

/// Discrete non-coalescable control commands dispatched from the UI thread to the OptiX worker.
#[derive(Clone, Debug)]
pub enum RenderControl {
    /// Reallocate OptiX render targets and switch shared CUDA/wgpu buffer pointers.
    Resize {
        width: u32,
        height: u32,
        buf_ptrs: [u64; 2],
        depth_ptrs: [u64; 2],
        motion_ptrs: [u64; 2],
        hdr_ptrs: Option<[u64; 2]>,
        rr_ptrs: Option<DlssRrBufferPtrs>,
        /// Output-resolution overlay layer while DLSS is on (`renderer::OverlayOutPtrs`).
        overlay_out: Option<OverlayOutPtrs>,
        gen_id: u64,
    },
    /// Return a previously presented shared buffer index back to the render thread's free pool.
    ReturnBuffer { buf_idx: usize, gen_id: u64 },
    /// Encode the current viewport frame as a GPU nvJPEG file.
    SaveJpeg { path: PathBuf, quality: u8 },
    /// Export exact SDG actor keypoints, tight bounding boxes, and ray-occluded visibility as JSON.
    ExportSdgJson {
        path: PathBuf,
        occluder: SdgOccluderBackend,
    },
    /// Capture a multi-frame SDG dataset burst (9-channel `.exr` + `.sdg.json` + `manifest.json`).
    StartSdgBurst {
        dir: PathBuf,
        frames: u32,
        occluder: SdgOccluderBackend,
    },
    /// Abort the running SDG dataset burst (no-op when none is running).
    CancelSdgBurst,
    /// Apply a poke stumble/flinch impulse to the nearest actor along camera ray or all actors.
    PokeCrowd {
        origin: [f32; 3],
        forward: [f32; 3],
        impulse: f32,
        all: bool,
    },
    /// Spawn a large crowd stress grid (`target_count`, e.g. `2000`) around `origin`.
    SpawnCrowdStress {
        target_count: usize,
        origin: [f32; 3],
    },
    /// Toggle Smooth 60 FPS / 180-degree shutter recording mode on the render worker.
    SetRecording {
        active: bool,
        fixed_fps: u32,
        shutter_subframes: u32,
    },
    /// Render one lockstep subframe of a 180-degree shutter motion-blur output frame.
    RenderShutterSubframe {
        settings: Box<DemoSettings>,
        camera: CameraFrame,
        sim_dt: f32,
        sub_idx: u32,
        sub_count: u32,
        reset_seq: u64,
        possess_free_yaw: f32,
        possess_free_pitch: f32,
        possess_auto_level: bool,
        possess_zoom_offset: f32,
    },
    /// Answer a viewport click with the GPU pick record of the next launch (`crate::pick`).
    Pick(crate::pick::PickRequest),
    /// Possess the next dynamic vehicle (prioritizing `Robotaxi`) or mounted scene object.
    PossessNextVehicle,
    /// Switch active camera mount index on the currently possessed object.
    SetPossessMount { mount_idx: usize },
    /// Eject from the currently possessed object back to free camera.
    EjectPossess,
    /// Evaluate 64 scenic viewpoints using the requested probe backend and return the winner.
    RunSmartView { seed: u64, backend: ProbeBackendKind },
    /// Hot-reload `raytracer.cu` from disk via NVRTC.
    ReloadShader,
    /// Update ocean wave simulation and dielectric water optics parameters.
    SetOceanParams(OceanParams),
    /// Update live parametric overrides for the active Inspector asset (or clear for city scenes).
    SetAssetParams(csg_scene::AssetParamMap),
    /// Clear both the in-memory LRU and on-disk compiled CSG scene cache.
    ClearSceneCache,
    /// Start (or restart) the per-frame performance log: skip `warmup` frames, keep `frames`,
    /// then write the CSV to `path` (and the last frame's linear colour as a PFM to `image`) and
    /// emit [`RenderEvent::PerfLogWritten`]. Restarts the sample-sequence frame index at 0 so two
    /// runs of the same camera sequence render identical images. Sent by the rail benchmark
    /// (`crate::bench`); `NV_DEMO_PERF_CSV` starts the same log from the environment.
    StartPerfLog {
        path: std::path::PathBuf,
        frames: u32,
        warmup: u32,
        image: Option<std::path::PathBuf>,
        /// Directory for every frame's displayed image as PNG (`bench::BenchRailArgs::png_dir`).
        png_dir: Option<std::path::PathBuf>,
    },
    /// Gracefully terminate the render worker thread.
    Shutdown,
}

/// Typed events emitted by the OptiX render worker back to the 60+ FPS UI thread.
#[derive(Clone, Debug)]
pub enum RenderEvent {
    /// A newly rendered viewport frame is ready in shared GPU memory (boxed to keep `RenderEvent` small).
    FrameReady(Box<FrameReady>),
    /// A newly built CSG scene exported its viewpoints and collision-verified camera rails.
    SceneMetaUpdated(SceneLoadedMetadata),
    /// Updated on-disk scene cache statistics and latest hit/miss provenance.
    SceneCacheStats(SceneCacheStats),
    /// Active possessed body or mount changed (`None` = ejected to free camera).
    PossessChanged(Option<PossessInfo>),
    /// Intersection counts of the latest counted launch (`crate::is_counters`), sent once when
    /// they arrive, immediately before the `FrameReady` of the frame that read them back.
    IsCounts(crate::is_counters::IsCounts),
    /// The viewport selection changed (GPU pick or scene switch); boxed, it carries strings.
    SelectionChanged(Box<crate::pick::SelectionInfo>),
    /// A `Smart View` scoring run selected a top viewpoint.
    SmartViewReady(SmartViewResult),
    /// Render worker transitioned between compile/build/IAS/rendering phases.
    PhaseChanged(EnginePhase),
    /// Human-readable status or diagnostic message for the UI status bar.
    StatusMessage(String),
    /// The performance log finished: the CSV path, or why it could not be written.
    PerfLogWritten(Result<std::path::PathBuf, String>),
    /// Unrecoverable render worker error, emitted once right before the worker leaves its loop.
    ///
    /// The UI shows it persistently, finishes any recording, leaves lockstep and stops sending
    /// render work. A worker that ends without this event (panic) is detected separately through
    /// `JoinHandle::is_finished` and treated the same way.
    Fatal(String),
}

/// Lock-free coalesced latest render state published by the UI loop and read by the OptiX worker.
///
/// Storing `DemoSettings` and `CameraFrame` in an `appbus::StateSlot` guarantees that rapid
/// slider drags or mouse orbits never build up a backlog of stale frames — the render thread
/// always consumes only the latest snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct RenderStateSnapshot {
    /// Latest effective render settings (after `DemoAnimator` evaluation).
    pub settings: DemoSettings,
    /// Latest camera basis (after `FlightPlayer` and `CameraDamper` evaluation).
    pub camera: CameraFrame,
    /// Deadline until which interactive fast-preview (`1 spp`, `bounces <= 2`) remains active.
    pub interacting_until: Option<Instant>,
    /// Monotonically incremented whenever an explicit `ResetAccum` is requested.
    pub reset_seq: u64,
    /// User free-look yaw offset (radians) while possessing a body with `allow_free_look`.
    pub possess_free_yaw: f32,
    /// User free-look pitch offset (radians) while possessing a body with `allow_free_look`.
    pub possess_free_pitch: f32,
    /// Keep possessed camera horizon level (`true`) or follow full body roll/pitch (`false`).
    pub possess_auto_level: bool,
    /// User scroll/pinch zoom offset (meters along `-forward`) while possessing a vehicle or actor.
    pub possess_zoom_offset: f32,
    /// Cached sampled flight path overlay data (`None` when the `FlightPath` overlay is off or no
    /// track is active), shared via `Arc` so 60 Hz UI snapshot publishes never resample or allocate.
    pub flight_path_overlay: Option<Arc<crate::overlay::FlightPathOverlayData>>,
}

/// Lost-wakeup-immune condvar waking the OptiX worker whenever a command or state snapshot arrives.
#[derive(Clone, Default)]
pub struct RenderWake {
    inner: Arc<(Mutex<bool>, Condvar)>,
}

impl RenderWake {
    /// Marks work pending and wakes the render worker immediately.
    pub fn notify(&self) {
        let (lock, cvar) = &*self.inner;
        let mut pending = lock.lock().unwrap_or_else(|e| e.into_inner());
        *pending = true;
        cvar.notify_one();
    }

    /// Sleeps until [`Self::notify`] is called (returns immediately if already notified).
    pub fn wait(&self) {
        let (lock, cvar) = &*self.inner;
        let mut pending = lock.lock().unwrap_or_else(|e| e.into_inner());
        while !*pending {
            pending = cvar.wait(pending).unwrap_or_else(|e| e.into_inner());
        }
        *pending = false;
    }

    /// Sleeps until [`Self::notify`] is called or `timeout` elapses.
    pub fn wait_timeout(&self, timeout: Duration) {
        let (lock, cvar) = &*self.inner;
        let mut pending = lock.lock().unwrap_or_else(|e| e.into_inner());
        if !*pending {
            let (guard, _) = cvar
                .wait_timeout(pending, timeout)
                .unwrap_or_else(|e| e.into_inner());
            pending = guard;
        }
        *pending = false;
    }
}

/// Unified SSOT event bus and coalesced state hub shared between `ui.rs`, `main.rs`, and `renderer.rs`.
#[derive(Clone)]
pub struct AppBus {
    /// UI-local typed command queue (`UiCommand`).
    pub ui_bus: EventBus,
    /// Discrete control commands sent from the UI thread to the OptiX worker (`RenderControl`).
    pub render_bus: EventBus,
    /// Asynchronous events sent from the OptiX worker back to the UI thread (`RenderEvent`).
    pub event_bus: EventBus,
    /// Latest-state coalescing slot for `DemoSettings`, `CameraFrame`, and fast-preview state.
    pub state: StateSlot<RenderStateSnapshot>,
    /// Latest-state coalescing slot for live compile / CSG scene build / IAS progress published
    /// by the OptiX render worker and polled by the 60 Hz UI loop.
    pub build_progress: StateSlot<Option<SceneBuildProgress>>,
    /// Condvar waker wired to `render_bus` and `state` updates.
    pub render_wake: RenderWake,
}

impl AppBus {
    /// Constructs a wired [`AppBus`] initialized with `initial_state`.
    pub fn new(initial_state: RenderStateSnapshot) -> Self {
        let ui_bus = EventBus::with_capacity(256);
        let render_bus = EventBus::with_capacity(256);
        let event_bus = EventBus::with_capacity(256);
        let state = StateSlot::new(initial_state);
        let build_progress = StateSlot::new(None);
        let render_wake = RenderWake::default();

        let wake_for_bus = render_wake.clone();
        render_bus.set_waker(Arc::new(move || wake_for_bus.notify()));

        Self {
            ui_bus,
            render_bus,
            event_bus,
            state,
            build_progress,
            render_wake,
        }
    }

    /// Publishes the latest [`SceneBuildProgress`] (`None` when steady-state rendering resumes).
    #[inline]
    pub fn publish_build_progress(&self, progress: Option<SceneBuildProgress>) -> u64 {
        self.build_progress.publish(progress)
    }

    /// Loads the latest [`SceneBuildProgress`] snapshot in O(1) without cloning strings.
    #[inline]
    pub fn load_build_progress(&self) -> Arc<Option<SceneBuildProgress>> {
        self.build_progress.load()
    }

    /// Emits a typed [`UiCommand`] from UI panels to the main application loop.
    #[inline]
    pub fn emit_ui(&self, cmd: UiCommand) {
        self.ui_bus.emit(cmd);
    }

    /// Emits a discrete [`RenderControl`] command and wakes the OptiX render worker.
    #[inline]
    pub fn send_render(&self, cmd: RenderControl) {
        self.render_bus.emit(cmd);
    }

    /// Emits a typed [`RenderEvent`] from the OptiX worker to the 60+ FPS UI thread.
    #[inline]
    pub fn emit_render_event(&self, event: RenderEvent) {
        self.event_bus.emit(event);
    }

    /// Publishes the latest coalesced [`RenderStateSnapshot`] and wakes the OptiX worker.
    #[inline]
    pub fn publish_render_state(&self, snapshot: RenderStateSnapshot) -> u64 {
        let rev = self.state.publish(snapshot);
        self.render_wake.notify();
        rev
    }

    /// Drains all queued UI commands into reusable `scratch` (zero steady-state allocations).
    #[inline]
    pub fn poll_ui_commands(&self, scratch: &mut Vec<BoxedEvent>) -> usize {
        self.ui_bus.poll_into(scratch)
    }

    /// Drains all queued render control commands into reusable `scratch`.
    #[inline]
    pub fn poll_render_controls(&self, scratch: &mut Vec<BoxedEvent>) -> usize {
        self.render_bus.poll_into(scratch)
    }

    /// Drains all queued render events into reusable `scratch` without blocking the UI thread.
    #[inline]
    pub fn poll_render_events(&self, scratch: &mut Vec<BoxedEvent>) -> usize {
        self.event_bus.poll_into(scratch)
    }
}

/// Computes effective `(spp, bounces)` for an OptiX launch, dropping to snappy `1 spp` and
/// `<= 2` bounces during interactive dragging/orbiting (unless recording is active).
#[inline]
pub fn fast_preview_budget(spp: u32, bounces: u32, fast_preview: bool) -> (u32, u32) {
    let base_spp = spp.max(1);
    let base_bounces = bounces.clamp(1, 16);
    if fast_preview {
        (1, base_bounces.min(2))
    } else {
        (base_spp, base_bounces)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use appbus::downcast_event;

    fn sample_camera(x: f32) -> CameraFrame {
        CameraFrame {
            eye: [x, 2.0, -5.0],
            fwd: [0.0, 0.0, 1.0],
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            tan_half_fov: 0.5,
        }
    }

    #[test]
    fn state_slot_coalesces_rapid_updates_to_latest_snapshot() {
        let initial = RenderStateSnapshot {
            settings: DemoSettings::default(),
            camera: sample_camera(0.0),
            interacting_until: None,
            reset_seq: 0,
            possess_free_yaw: 0.0,
            possess_free_pitch: 0.0,
            possess_auto_level: true,
            possess_zoom_offset: 0.0,
            flight_path_overlay: None,
        };
        let bus = AppBus::new(initial);

        for i in 1..=100u32 {
            let s = DemoSettings {
                exposure: 0.5 + i as f32 * 0.01,
                ..DemoSettings::default()
            };
            bus.publish_render_state(RenderStateSnapshot {
                settings: s,
                camera: sample_camera(i as f32),
                interacting_until: Some(Instant::now() + INTERACTION_SETTLE_WINDOW),
                reset_seq: u64::from(i),
                possess_free_yaw: 0.0,
                possess_free_pitch: 0.0,
                possess_auto_level: true,
                possess_zoom_offset: 0.0,
                flight_path_overlay: None,
            });
        }

        // Consumer reads only the latest coalesced state (revision 100) with zero stale backlog.
        assert_eq!(bus.state.revision(), 100);
        let latest = bus.state.load();
        assert!((latest.camera.eye[0] - 100.0).abs() < 1e-6);
        assert_eq!(latest.reset_seq, 100);
        assert!(latest.interacting_until.is_some());
    }

    #[test]
    fn event_bus_routes_ui_control_and_render_events_with_reused_scratch() {
        let initial = RenderStateSnapshot {
            settings: DemoSettings::default(),
            camera: sample_camera(0.0),
            interacting_until: None,
            reset_seq: 0,
            possess_free_yaw: 0.0,
            possess_free_pitch: 0.0,
            possess_auto_level: true,
            possess_zoom_offset: 0.0,
            flight_path_overlay: None,
        };
        let bus = AppBus::new(initial);
        let mut scratch = Vec::with_capacity(16);

        bus.emit_ui(UiCommand::ResetAccum);
        bus.emit_ui(UiCommand::SaveStill(StillFormat::Png));
        bus.emit_ui(UiCommand::Flight(crate::flight::FlightCommand::Play));
        bus.emit_ui(UiCommand::ToggleFreezePose);
        bus.emit_ui(UiCommand::ExportSdgJson);
        bus.emit_ui(UiCommand::PokeCrowd {
            impulse: 1.8,
            all: false,
        });
        bus.emit_ui(UiCommand::SpawnCrowdStress { target_count: 2000 });
        bus.emit_ui(UiCommand::StartSdgBurst { frames: 30 });
        assert_eq!(bus.poll_ui_commands(&mut scratch), 8);
        assert_eq!(
            downcast_event::<UiCommand>(&scratch[0]),
            Some(&UiCommand::ResetAccum)
        );
        assert_eq!(
            downcast_event::<UiCommand>(&scratch[1]),
            Some(&UiCommand::SaveStill(StillFormat::Png))
        );
        assert_eq!(
            downcast_event::<UiCommand>(&scratch[2]),
            Some(&UiCommand::Flight(crate::flight::FlightCommand::Play))
        );
        assert_eq!(
            downcast_event::<UiCommand>(&scratch[3]),
            Some(&UiCommand::ToggleFreezePose)
        );
        assert_eq!(
            downcast_event::<UiCommand>(&scratch[4]),
            Some(&UiCommand::ExportSdgJson)
        );
        assert_eq!(
            downcast_event::<UiCommand>(&scratch[5]),
            Some(&UiCommand::PokeCrowd {
                impulse: 1.8,
                all: false
            })
        );
        assert_eq!(
            downcast_event::<UiCommand>(&scratch[6]),
            Some(&UiCommand::SpawnCrowdStress { target_count: 2000 })
        );
        assert_eq!(
            downcast_event::<UiCommand>(&scratch[7]),
            Some(&UiCommand::StartSdgBurst { frames: 30 })
        );

        bus.send_render(RenderControl::ReloadShader);
        bus.send_render(RenderControl::Shutdown);
        assert_eq!(bus.poll_render_controls(&mut scratch), 2);
        assert!(matches!(
            downcast_event::<RenderControl>(&scratch[0]),
            Some(RenderControl::ReloadShader)
        ));
        assert!(matches!(
            downcast_event::<RenderControl>(&scratch[1]),
            Some(RenderControl::Shutdown)
        ));

        bus.emit_render_event(RenderEvent::StatusMessage("ok".to_string()));
        assert_eq!(bus.poll_render_events(&mut scratch), 1);
        assert!(matches!(
            downcast_event::<RenderEvent>(&scratch[0]),
            Some(RenderEvent::StatusMessage(msg)) if msg == "ok"
        ));
    }

    #[test]
    fn fatal_event_and_sdg_burst_cancel_route_through_the_buses() {
        let bus = AppBus::new(RenderStateSnapshot {
            settings: DemoSettings::default(),
            camera: sample_camera(0.0),
            interacting_until: None,
            reset_seq: 0,
            possess_free_yaw: 0.0,
            possess_free_pitch: 0.0,
            possess_auto_level: true,
            possess_zoom_offset: 0.0,
            flight_path_overlay: None,
        });
        let mut scratch = Vec::with_capacity(4);

        bus.emit_ui(UiCommand::CancelSdgBurst);
        assert_eq!(bus.poll_ui_commands(&mut scratch), 1);
        assert_eq!(
            downcast_event::<UiCommand>(&scratch[0]),
            Some(&UiCommand::CancelSdgBurst)
        );

        bus.send_render(RenderControl::CancelSdgBurst);
        assert_eq!(bus.poll_render_controls(&mut scratch), 1);
        assert!(matches!(
            downcast_event::<RenderControl>(&scratch[0]),
            Some(RenderControl::CancelSdgBurst)
        ));

        bus.emit_render_event(RenderEvent::Fatal("cuLaunchKernel failed".to_string()));
        assert_eq!(bus.poll_render_events(&mut scratch), 1);
        assert!(matches!(
            downcast_event::<RenderEvent>(&scratch[0]),
            Some(RenderEvent::Fatal(msg)) if msg == "cuLaunchKernel failed"
        ));
    }

    #[test]
    fn render_wake_is_immune_to_lost_wakeups() {
        let wake = RenderWake::default();
        wake.notify();
        // Must return immediately even though notify happened before wait.
        let t0 = Instant::now();
        wake.wait();
        assert!(t0.elapsed() < Duration::from_millis(50));
    }

    #[test]
    fn fast_preview_budget_caps_spp_and_bounces_during_interaction() {
        assert_eq!(fast_preview_budget(8, 6, true), (1, 2));
        assert_eq!(fast_preview_budget(8, 1, true), (1, 1));
        assert_eq!(fast_preview_budget(8, 6, false), (8, 6));
        assert_eq!(fast_preview_budget(0, 0, false), (1, 1));
    }

    #[test]
    fn scene_build_progress_fraction_and_formatting_clamp_cleanly() {
        let t0 = Instant::now();
        let indet = SceneBuildProgress::new(
            EnginePhase::BuildingCsgScene,
            "Gundo OSM",
            "Generating CSG scene...",
            0,
            0,
            t0,
        );
        assert_eq!(indet.fraction(), None);
        assert_eq!(indet.format_stage_line(), "Generating CSG scene...");
        assert_eq!(
            indet.format_status_summary(12.34),
            "Gundo OSM: Generating CSG scene... [00:12.3]"
        );

        let mid = SceneBuildProgress::new(
            EnginePhase::BuildingCsgScene,
            "Gundo OSM",
            "Building OptiX GAS",
            61,
            244,
            t0,
        );
        assert_eq!(mid.fraction(), Some(0.25));
        assert_eq!(mid.format_stage_line(), "Building OptiX GAS (61/244)");
        assert_eq!(
            mid.format_status_summary(65.89),
            "Gundo OSM: Building OptiX GAS (61/244) [01:05.8]"
        );

        // `done > total` clamps cleanly to `1.0` and `total/total`.
        let over = SceneBuildProgress::new(
            EnginePhase::BuildingCsgScene,
            "",
            "Building OptiX GAS",
            300,
            246,
            t0,
        );
        assert_eq!(over.fraction(), Some(1.0));
        assert_eq!(over.format_stage_line(), "Building OptiX GAS (246/246)");
        assert_eq!(
            over.format_status_summary(59.99),
            "Building OptiX GAS (246/246) [00:59.9]"
        );

        // Elapsed formatter sanitizes negative / NaN / inf and avoids `00:60.0` rollover.
        assert_eq!(SceneBuildProgress::format_elapsed(0.0), "00:00.0");
        assert_eq!(SceneBuildProgress::format_elapsed(-5.0), "00:00.0");
        assert_eq!(SceneBuildProgress::format_elapsed(f32::NAN), "00:00.0");
        assert_eq!(SceneBuildProgress::format_elapsed(f32::INFINITY), "00:00.0");
        assert_eq!(SceneBuildProgress::format_elapsed(59.99), "00:59.9");
        assert_eq!(SceneBuildProgress::format_elapsed(60.0), "01:00.0");
        assert_eq!(SceneBuildProgress::format_elapsed(125.34), "02:05.3");

        // `AppBus::build_progress` coalesces updates and clears cleanly to `None`.
        let bus = AppBus::new(RenderStateSnapshot {
            settings: DemoSettings::default(),
            camera: sample_camera(0.0),
            interacting_until: None,
            reset_seq: 0,
            possess_free_yaw: 0.0,
            possess_free_pitch: 0.0,
            possess_auto_level: true,
            possess_zoom_offset: 0.0,
            flight_path_overlay: None,
        });
        assert!(bus.load_build_progress().is_none());
        bus.publish_build_progress(Some(indet));
        bus.publish_build_progress(Some(mid.clone()));
        assert_eq!(bus.load_build_progress().as_ref(), &Some(mid));
        bus.publish_build_progress(None);
        assert!(bus.load_build_progress().is_none());
    }

    #[test]
    fn sim_telemetry_formatting_and_math_helpers() {
        let mut buf = String::new();
        let sim = SimTelemetry {
            crowd_backend: ActiveCrowdBackend::Cuda,
            crowd_plan_version: 2,
            crowd_kernel_count: 11,
            crowd_actors: 184,
            crowd_archetypes: [0, 168, 0, 16],
            crowd_bones: 2944,
            crowd_joints: 2944,
            crowd_skeleton_edges: 2760,
            crowd_dofs: 4416,
            sim_rate: 1.0,
            crowd_lod_dists_m: [22.0, 75.0, 180.0],
            crowd_lod_mode: 0,
            traffic_vehicles: 48,
            traffic_moving: 36,
            traffic_avg_speed_mps: 10.0,
            traffic_max_speed_mps: 15.0,
            ..SimTelemetry::default()
        };

        assert!(sim.has_crowd());
        assert!(sim.has_traffic());
        assert!(sim.has_any());
        assert_eq!(sim.traffic_stopped(), 12);
        assert!((sim.traffic_avg_kmh() - 36.0).abs() < 1e-4);
        assert!((sim.traffic_max_kmh() - 54.0).abs() < 1e-4);
        assert!((sim.bones_per_sec(60.0) - 176_640.0).abs() < 1e-2);

        sim.write_archetypes_compact(&mut buf);
        assert_eq!(buf, "184 (168 ped / 16 pet)");

        buf.clear();
        sim.append_lod_summary(&mut buf);
        assert_eq!(buf, "Auto 22/75/180m");

        let frozen = SimTelemetry {
            pose_frozen: true,
            crowd_lod_mode: 4,
            traffic_avg_speed_mps: f32::NAN,
            traffic_max_speed_mps: -2.0,
            ..sim
        };
        assert_eq!(frozen.bones_per_sec(60.0), 0.0);
        assert_eq!(frozen.traffic_avg_kmh(), 0.0);
        assert_eq!(frozen.traffic_max_kmh(), 0.0);
        buf.clear();
        frozen.append_lod_summary(&mut buf);
        assert_eq!(buf, "LOD Off");

        SimTelemetry::write_bytes_compact(512, &mut buf);
        assert_eq!(buf, "512 B");
        SimTelemetry::write_bytes_compact(2048, &mut buf);
        assert_eq!(buf, "2.0 KiB");
        SimTelemetry::write_bytes_compact(4 * 1024 * 1024 + 524_288, &mut buf);
        assert_eq!(buf, "4.50 MiB");
    }

    #[test]
    fn possess_adas_badge_severity_and_throttle_brake_normalization() {
        let aeb = PossessAdas::from_pod(VehicleHazardStatePod {
            state: hazard_state::PED_AEB,
            flags: hazard_flags::BRAKE_LIGHT | hazard_flags::HAZARD_ACTOR_VALID,
            hazard_actor_idx: 7,
            speed_mps: 10.0,
            accel_mps2: -MAX_BRAKE_DECEL,
            stop_dist_m: 16.0,
            hazard_dist_m: 4.5,
            limit_mps: 13.888_889,
        });
        assert_eq!(aeb.badge_label(), "AEB BRAKE");
        assert_eq!(aeb.severity(), 2);
        assert!(aeb.brake_light_on());
        assert!(aeb.has_ped_hazard());
        assert!((aeb.speed_kmh() - 36.0).abs() < 1e-4);
        assert!((aeb.limit_kmh() - 50.0).abs() < 1e-3);
        assert!((aeb.throttle_brake_norm() + 1.0).abs() < 1e-5);

        let cruise = PossessAdas::from_pod(VehicleHazardStatePod {
            state: hazard_state::CRUISE,
            flags: 0,
            hazard_actor_idx: u32::MAX,
            speed_mps: 12.0,
            accel_mps2: IDM_ACCEL_MAX * 0.5,
            stop_dist_m: 20.0,
            hazard_dist_m: f32::INFINITY,
            limit_mps: 15.0,
        });
        assert_eq!(cruise.badge_label(), "CRUISE");
        assert_eq!(cruise.severity(), 0);
        assert!(!cruise.brake_light_on());
        assert!(!cruise.has_ped_hazard());
        assert!((cruise.throttle_brake_norm() - 0.5).abs() < 1e-5);

        let yield_ped = PossessAdas {
            state: hazard_state::PED_YIELD,
            accel_mps2: f32::NAN,
            speed_mps: -1.0,
            ..PossessAdas::default()
        };
        assert_eq!(yield_ped.badge_label(), "YIELD PED");
        assert_eq!(yield_ped.severity(), 1);
        assert_eq!(yield_ped.speed_kmh(), 0.0);
        assert_eq!(yield_ped.throttle_brake_norm(), 0.0);
    }

    #[test]
    fn build_eta_tracker_sma10_warmup_rolling_window_and_completion() {
        let t0 = Instant::now();
        let mut tracker = BuildEtaTracker::new(t0);
        assert_eq!(tracker.sample_count(), 0);

        // First interval alone must stay in warmup (`None` until >= 2 completed intervals).
        assert_eq!(
            tracker.record(1.0, 20.0, t0 + Duration::from_secs(1)),
            None
        );
        assert_eq!(tracker.sample_count(), 1);

        // Non-advancing update must not add a sample or exit warmup.
        assert_eq!(
            tracker.record(1.0, 20.0, t0 + Duration::from_secs(2)),
            None
        );
        assert_eq!(tracker.sample_count(), 1);

        // Second interval (1 unit in 2 s -> mean rate = (1 + 2)/2 = 1.5 s/unit; 18 left -> 27.0 s).
        let eta2 = tracker
            .record(2.0, 20.0, t0 + Duration::from_secs(3))
            .expect("2nd interval must produce an ETA");
        assert!((eta2 - 27.0).abs() < 1e-3, "expected 27.0s, got {eta2}");
        assert_eq!(tracker.sample_count(), 2);

        // Push 10 more intervals of 0.5 s/unit each so the SMA-10 window rolls past the initial
        // slow intervals and converges strictly to 0.5 s/unit.
        let mut t = t0 + Duration::from_secs(3);
        let mut done = 2.0_f32;
        for _ in 0..ETA_SMA_WINDOW {
            t += Duration::from_millis(500);
            done += 1.0;
            let _ = tracker.record(done, 20.0, t);
        }
        assert_eq!(tracker.sample_count(), ETA_SMA_WINDOW);
        // 12 units done, 8 remaining at 0.5 s/unit => 4.0 s.
        let eta_steady = tracker
            .record(done, 20.0, t)
            .expect("steady SMA-10 must return ETA");
        assert!(
            (eta_steady - 4.0).abs() < 1e-3,
            "expected 4.0s after SMA-10 rollover, got {eta_steady}"
        );

        // Reaching total yields 0.0 s remaining; invalid inputs return None cleanly.
        let eta_done = tracker
            .record(20.0, 20.0, t + Duration::from_secs(4))
            .expect("completed build must return 0.0s ETA");
        assert_eq!(eta_done, 0.0);
        assert_eq!(tracker.record(f32::NAN, 10.0, t), None);
        assert_eq!(tracker.record(1.0, 0.0, t), None);
    }

    #[test]
    fn scene_build_progress_step_prefix_overall_and_eta_formatting() {
        let t0 = Instant::now();
        let p = SceneBuildProgress::new(
            EnginePhase::BuildingCsgScene,
            "Gundo",
            "Baking road tiles",
            12,
            24,
            t0,
        )
        .with_step(3, 8)
        .with_overall(Some(0.42))
        .with_eta(Some(8.64));

        assert_eq!(p.format_step_prefix().as_deref(), Some("Step 3/8"));
        assert_eq!(SceneBuildProgress::format_eta(8.64), "~00:08.6");
        assert_eq!(p.effective_fraction(), Some(0.42));
        assert_eq!(p.format_stage_line(), "Step 3/8 | Baking road tiles (12/24)");
        assert_eq!(
            p.format_status_summary(14.23),
            "Gundo: Step 3/8 | Baking road tiles (12/24) [00:14.2 | ETA ~00:08.6]"
        );

        // Non-finite / negative values sanitize cleanly.
        let bad = p
            .clone()
            .with_overall(Some(f32::NAN))
            .with_eta(Some(-1.0));
        assert_eq!(bad.overall_fraction, None);
        assert_eq!(bad.effective_fraction(), Some(0.5));
        assert_eq!(bad.eta_secs, None);
    }

    #[test]
    fn ocean_params_defaults_sanitization_and_control_routing() {
        let def = OceanParams::default();
        assert_eq!(def.mode, OceanMode::CuFft);
        assert_eq!(def.wind_speed, 9.5);
        assert_eq!(def.wind_dir_deg, 245.0);
        assert_eq!(def.choppiness, 1.15);
        assert_eq!(def.swell, 0.65);
        assert_eq!(def.foam, 1.0);
        assert_eq!(def.clarity, 1.0);

        let dirty = OceanParams {
            mode: OceanMode::Analytical,
            wind_speed: 99.0,
            wind_dir_deg: -45.0,
            choppiness: f32::NAN,
            swell: -3.0,
            foam: 5.0,
            clarity: 0.01,
        }
        .sanitized();
        assert_eq!(dirty.mode, OceanMode::Analytical);
        assert_eq!(dirty.wind_speed, 25.0);
        assert!((dirty.wind_dir_deg - 315.0).abs() < 1e-4);
        assert_eq!(dirty.choppiness, def.choppiness);
        assert_eq!(dirty.swell, 0.0);
        assert_eq!(dirty.foam, 2.0);
        assert_eq!(dirty.clarity, 0.25);

        let bus = AppBus::new(RenderStateSnapshot {
            settings: DemoSettings::default(),
            camera: sample_camera(0.0),
            interacting_until: None,
            reset_seq: 0,
            possess_free_yaw: 0.0,
            possess_free_pitch: 0.0,
            possess_auto_level: true,
            possess_zoom_offset: 0.0,
            flight_path_overlay: None,
        });
        let mut scratch = Vec::with_capacity(4);
        bus.send_render(RenderControl::SetOceanParams(dirty));
        assert_eq!(bus.poll_render_controls(&mut scratch), 1);
        assert!(matches!(
            downcast_event::<RenderControl>(&scratch[0]),
            Some(RenderControl::SetOceanParams(p)) if *p == dirty
        ));
    }

    #[test]
    fn scene_cache_stats_and_asset_param_controls_route_cleanly() {
        assert_eq!(SceneLoadSource::None.label(), "Idle");
        assert_eq!(SceneLoadSource::Built.label(), "Built");
        assert_eq!(SceneLoadSource::MemoryHit.label(), "Memory Hit");
        assert_eq!(SceneLoadSource::DiskHit.label(), "Disk Cache Hit");

        let stats = SceneCacheStats {
            entries: 3,
            total_bytes: 12_345_678,
            last_hit: true,
            last_source: SceneLoadSource::DiskHit,
        };
        let mut params = csg_scene::AssetParamMap::new();
        params.set("uniform_scale", csg_scene::AssetParamValue::Float(1.5));

        let bus = AppBus::new(RenderStateSnapshot {
            settings: DemoSettings::default(),
            camera: sample_camera(0.0),
            interacting_until: None,
            reset_seq: 0,
            possess_free_yaw: 0.0,
            possess_free_pitch: 0.0,
            possess_auto_level: true,
            possess_zoom_offset: 0.0,
            flight_path_overlay: None,
        });
        bus.emit_ui(UiCommand::ClearSceneCache);
        bus.send_render(RenderControl::SetAssetParams(params.clone()));
        bus.send_render(RenderControl::ClearSceneCache);
        bus.emit_render_event(RenderEvent::SceneCacheStats(stats));

        let mut scratch = Vec::with_capacity(4);
        assert_eq!(bus.poll_ui_commands(&mut scratch), 1);
        assert!(matches!(
            downcast_event::<UiCommand>(&scratch[0]),
            Some(UiCommand::ClearSceneCache)
        ));
        assert_eq!(bus.poll_render_controls(&mut scratch), 2);
        assert!(matches!(
            downcast_event::<RenderControl>(&scratch[0]),
            Some(RenderControl::SetAssetParams(p)) if *p == params
        ));
        assert!(matches!(
            downcast_event::<RenderControl>(&scratch[1]),
            Some(RenderControl::ClearSceneCache)
        ));
        assert_eq!(bus.poll_render_events(&mut scratch), 1);
        assert!(matches!(
            downcast_event::<RenderEvent>(&scratch[0]),
            Some(RenderEvent::SceneCacheStats(s)) if *s == stats
        ));
    }

    #[test]
    fn stage_line_skips_duplicate_counter_when_stage_has_inline_count() {
        let now = Instant::now();
        let inline = SceneBuildProgress::new(
            EnginePhase::BuildingCsgScene,
            "SF HoAI",
            "Grounding objects 1/2 (1240/3480)",
            6400,
            10_000,
            now,
        )
        .with_step(1, 3);
        assert_eq!(
            inline.format_stage_line(),
            "Step 1/3 | Grounding objects 1/2 (1240/3480)"
        );

        let plain = SceneBuildProgress::new(
            EnginePhase::BuildingCsgScene,
            "SF HoAI",
            "Building OptiX GAS",
            40,
            218,
            now,
        )
        .with_step(3, 3);
        assert_eq!(
            plain.format_stage_line(),
            "Step 3/3 | Building OptiX GAS (40/218)"
        );
    }
}
