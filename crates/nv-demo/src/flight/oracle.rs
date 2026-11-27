//! Asynchronous GPU-accelerated CSG flight-planning coordinator backed by `fnav-csg`.
//!
//! **What.** Re-exports the single-source-of-truth `csg-scene` R0 (`VoxelVolume`) + R1
//! (`WorldRayQuery`) adapters from [`fnav_csg`] ([`CsgVolumeAdapter`], [`CsgObjectTable`],
//! [`CsgSceneOracle`], [`CsgOracleError`], [`plan_world_track`]), provides
//! [`build_oracle_with_baker`] (plugging `nv-demo`'s CUDA [`GpuVoxelBaker`] into
//! [`CsgSceneOracle::from_parts`]), and implements [`CsgAutoPlanner`] (a single non-blocking
//! background worker thread driving `fnav_csg::plan_world_track` with selective `PlanCache`
//! reuse across repeated plans).
//! **Why.** Eliminates duplicated `VoxelVolume` / `WorldRayQuery` / street-graph / landmark-table
//! bridge code by delegating the scene-oracle contract directly to `flight-navigator-rs`'s
//! dedicated `fnav-csg` leaf crate while keeping `nv-demo`'s CUDA `voxel_bake.cuh` acceleration
//! and non-blocking 60 Hz UI worker intact.
//! **How.** [`CsgAutoPlanner`] lazily spawns one `"nv-flight-planner"` thread blocked on an `mpsc`
//! channel (zero busy-waiting), lazily initializes one reusable [`GpuVoxelBaker`] to bake the R0
//! `VoxelVolume` on CUDA (falling back cleanly to exact CPU `VoxelVolume::build` if CUDA is
//! unavailable or overflows), hands the baked volume + `WorldRayQuery` into
//! [`CsgSceneOracle::from_parts`], caches `(scene_idx, road_mode, ground_kerb_mode, smart_fill,
//! inspector_variant)` + `PlanCache` across iterations (selectively invalidating `cache.catalog`
//! on `sun_dir` changes and `cache.catalog` + `cache.handover_costs` on `req.limits` changes),
//! and emits typed [`AutoPlanEvent`] updates back to `FlightWorkspace`.
//! **Uses.** `csg_scene`, `csg_scenes`, `flight_navigator`, `fnav_csg`, `glam`, `std::sync::mpsc`,
//! `crate::flight::voxel_gpu`.
//! **Used by.** `crate::flight`, `crate::flight::lab`, `crate::flight::ui`, `crate::main`.

use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Instant;

use csg_scene::voxel_volume::{VoxelVolume, VoxelVolumeConfig};
use csg_scene::world_ray::WorldRayQuery;
use csg_scene::{GroundKerbMode, RoadMode, SceneBuildConfig, SceneBuilt, SmartFill};
use flight_navigator::camera_guide::GuideLimits;
use flight_navigator::fnav_plan::{PlanCache, PlanRequest, RailPlan};
use fnav_csg::{CsgObjectTable, CsgSceneOracle, PlanBodyPoseSource, plan_world_track};
use glam::Vec3;

use crate::flight::voxel_gpu::GpuVoxelBaker;

/// Progress phase reported by [`CsgAutoPlanner`] while an asynchronous Auto-Plan job runs.
///
/// Why: allows the Flight Lab UI to show honest per-phase feedback (`BuildingScene` ->
/// `BakingVolume` -> `Planning`) instead of an opaque spinner and prevents concurrent submissions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AutoPlanStage {
    /// No planning job is currently running.
    #[default]
    Idle,
    /// Constructing the CSG [`SceneBuilt`] geometry from `csg_scenes::default_scene_registry()`.
    BuildingScene,
    /// Baking the R1 BVH (`WorldRayQuery`) and R0 signed distance volume (`VoxelVolume`).
    BakingVolume,
    /// Running the `flight-navigator` `N1..N9` cinematic flight-planning pipeline.
    Planning,
}

impl AutoPlanStage {
    /// Human-readable status label rendered in Flight Lab while the planner is active.
    ///
    /// Why: keeps UI phase wording centralized in one authoritative table.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::BuildingScene => "Building CSG scene...",
            Self::BakingVolume => "Baking voxel volume & BVH...",
            Self::Planning => "Planning cinematic tour...",
        }
    }

    /// Returns `true` while a background job is executing any non-idle stage.
    #[must_use]
    pub const fn is_busy(self) -> bool {
        !matches!(self, Self::Idle)
    }
}

/// Complete scene and planner specification submitted to [`CsgAutoPlanner::submit`].
///
/// Why: carries every immutable input needed by the background worker to rebuild or reuse the
/// exact active CSG scene (including road synthesis mode, inspector variant, and sun vector)
/// without sharing non-`Send` GPU host state across threads.
#[derive(Clone, Debug)]
pub struct AutoPlanJob {
    /// 1-based CSG scene index matching `csg_scene_host` (`1..=registry.len()`).
    pub scene_idx: u32,
    /// Stable scene identifier (e.g. `"gundo"`, `"marina"`, `"sf_house_of_ai"`, `"inspector"`).
    pub scene_id: String,
    /// Active road surface synthesis mode forwarded into [`SceneBuildConfig::road_mode`].
    pub road_mode: RoadMode,
    /// Ground-level kerb representation forwarded into [`SceneBuildConfig::ground_kerb_mode`].
    pub ground_kerb_mode: GroundKerbMode,
    /// Opt-in per-class procedural inference forwarded into [`SceneBuildConfig::smart_fill`].
    pub smart_fill: SmartFill,
    /// Inspector variant index (`0` for non-inspector scenes).
    pub inspector_variant: u32,
    /// World-space unit direction pointing toward the scene sun (`[x, y, z]`).
    pub sun_dir: [f32; 3],
    /// Serializable flight-planning request (`duration_s`, `seed`, `mode`, `cut_density`, `limits`, `edits`).
    pub req: PlanRequest,
}

/// Asynchronous status or completion event emitted by [`CsgAutoPlanner::poll`].
///
/// Why: decouples long-running CPU/GPU baking and planning from the 60 Hz UI loop while delivering
/// explicit stage transitions, validated [`RailPlan`]s, or typed failure strings.
#[derive(Clone, Debug)]
pub enum AutoPlanEvent {
    /// Worker transitioned to a new pipeline stage.
    Stage(AutoPlanStage),
    /// Worker produced a clearance-verified [`RailPlan`] for `scene_id`.
    Ready {
        /// Scene identifier the plan targets.
        scene_id: String,
        /// Boxed validated flight plan (boxed to keep `AutoPlanEvent` compact).
        plan: Box<RailPlan>,
        /// Total wall-clock elapsed time spent handling the job, in milliseconds.
        elapsed_ms: f32,
    },
    /// Scene build, volume bake, or flight planning failed with an explicit diagnostic reason.
    Failed(String),
}

/// Builds a [`fnav_csg::CsgSceneOracle`] and [`fnav_csg::CsgObjectTable`] from a built CSG scene,
/// accelerating R0 [`VoxelVolume`] column-span baking via `gpu_baker` when available and falling
/// back transparently to exact CPU [`VoxelVolume::build`] on any GPU error or overflow.
///
/// Why: cuts full-city `VoxelVolume` baking (e.g. Gundo) from ~2 s on CPU to ~265 ms on GPU while
/// delegating oracle assembly, street-graph extraction, and landmark sanitization to
/// [`CsgSceneOracle::from_parts`] as the single source of truth.
///
/// # Errors
/// Returns an error if `built.world` contains no geometry or produces an invalid empty volume.
pub fn build_oracle_with_baker(
    built: SceneBuilt,
    sun_dir: Option<Vec3>,
    gpu_baker: Option<&mut GpuVoxelBaker>,
) -> Result<(CsgSceneOracle, CsgObjectTable), String> {
    if built.world.static_instance_count() == 0 && built.world.dynamic_instances().is_empty() {
        return Err("CSG scene contains no geometry to plan against".to_string());
    }

    let ray_query = WorldRayQuery::build(&built.world);
    let cfg = VoxelVolumeConfig::default();
    let raw_volume = match gpu_baker {
        Some(baker) => match baker.bake_voxel_volume(&built.world, &cfg) {
            Ok(vol) => vol,
            Err(err) => {
                eprintln!(
                    "nv-demo: GPU voxel bake unavailable ({err}); falling back to CPU VoxelVolume::build"
                );
                VoxelVolume::build(&built.world, None, cfg)
            }
        },
        None => VoxelVolume::build(&built.world, None, cfg),
    };

    CsgSceneOracle::from_parts(built.world, raw_volume, ray_query, &built.hints, sun_dir)
        .map_err(|e| e.to_string())
}

/// Cached scene oracle, object catalog, and `PlanCache` held inside the worker thread.
///
/// Tracks `sun_dir` and `limits` alongside the scene geometry `key` so lighting or dynamics-limit
/// changes invalidate only the affected `PlanCache` tables without rebuilding the scene or volume.
struct CachedOracle {
    key: (u32, RoadMode, GroundKerbMode, SmartFill, u32),
    sun_dir: Option<Vec3>,
    limits: GuideLimits,
    oracle: CsgSceneOracle,
    objects: CsgObjectTable,
    cache: PlanCache,
}

/// Returns `true` when two optional unit sun directions differ enough to invalidate lighting scores.
#[inline]
fn sun_dir_changed(a: Option<Vec3>, b: Option<Vec3>) -> bool {
    match (a, b) {
        (Some(x), Some(y)) => (x - y).length_squared() > 1.0e-6,
        (None, None) => false,
        _ => true,
    }
}

/// Normalizes a raw `[x, y, z]` sun direction into a finite unit vector (or `None` if degenerate).
#[inline]
fn sanitize_sun_dir(raw: [f32; 3]) -> Option<Vec3> {
    let v = Vec3::from_array(raw);
    (v.is_finite() && v.length_squared() > 1.0e-6).then(|| v.normalize())
}

/// Asynchronous background flight-planning coordinator for `nv-demo`.
///
/// Why: building a CSG scene, baking its `VoxelVolume`, and running `fnav_csg::plan_world_track`
/// can take seconds; isolating that work on one dedicated `mpsc`-driven worker thread keeps the
/// 60 Hz UI and OptiX viewport completely responsive while reusing the baked scene and `PlanCache`
/// across repeated seed, plan-mode, duration, and cut-density iterations (and selectively
/// invalidating cached catalog/handover tables on `sun_dir` or `GuideLimits` changes).
pub struct CsgAutoPlanner {
    stage: AutoPlanStage,
    job_tx: Option<Sender<AutoPlanJob>>,
    ev_rx: Option<Receiver<AutoPlanEvent>>,
}

impl Default for CsgAutoPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl CsgAutoPlanner {
    /// Creates an idle [`CsgAutoPlanner`] (worker thread spawns lazily on the first submission).
    ///
    /// Why: avoids allocating a background thread in headless tests or sessions that never invoke
    /// Auto-Plan while keeping a single persistent worker once activated.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            stage: AutoPlanStage::Idle,
            job_tx: None,
            ev_rx: None,
        }
    }

    /// Submits a new [`AutoPlanJob`] to the background worker thread.
    ///
    /// Why: rejects overlapping jobs cleanly so the UI never queues stale plans or contends for
    /// CPU cores during an active bake/plan.
    ///
    /// # Errors
    /// Returns an error if another Auto-Plan job is already running or if the worker thread could
    /// not be spawned or reached.
    pub fn submit(&mut self, job: AutoPlanJob) -> Result<(), String> {
        if self.stage.is_busy() {
            return Err("Auto-plan is already running".to_string());
        }

        if self.job_tx.is_none() {
            self.spawn_worker()?;
        }

        let Some(tx) = self.job_tx.as_ref() else {
            return Err("Auto-plan worker channel is not initialized".to_string());
        };
        if tx.send(job).is_err() {
            self.job_tx = None;
            self.ev_rx = None;
            self.stage = AutoPlanStage::Idle;
            return Err("Auto-plan worker thread is no longer running".to_string());
        }

        self.stage = AutoPlanStage::BuildingScene;
        Ok(())
    }

    /// Drains all pending [`AutoPlanEvent`]s emitted by the worker thread without blocking.
    ///
    /// Why: called once per UI tick so stage labels, completed [`RailPlan`]s, and errors apply
    /// deterministically on the main thread with zero mutex locking.
    pub fn poll(&mut self) -> Vec<AutoPlanEvent> {
        let mut out = Vec::new();
        let mut disconnected = false;

        if let Some(rx) = self.ev_rx.as_ref() {
            loop {
                match rx.try_recv() {
                    Ok(ev) => {
                        match &ev {
                            AutoPlanEvent::Stage(s) => self.stage = *s,
                            AutoPlanEvent::Ready { .. } | AutoPlanEvent::Failed(_) => {
                                self.stage = AutoPlanStage::Idle;
                            }
                        }
                        out.push(ev);
                    }
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        disconnected = true;
                        break;
                    }
                }
            }
        }

        if disconnected {
            self.job_tx = None;
            self.ev_rx = None;
            if self.stage.is_busy() {
                self.stage = AutoPlanStage::Idle;
                out.push(AutoPlanEvent::Failed(
                    "Auto-plan worker thread terminated unexpectedly".to_string(),
                ));
            }
        }

        out
    }

    /// Returns `true` when an Auto-Plan job is currently in progress.
    #[must_use]
    pub const fn is_busy(&self) -> bool {
        self.stage().is_busy()
    }

    /// Returns the most recently observed [`AutoPlanStage`].
    #[must_use]
    pub const fn stage(&self) -> AutoPlanStage {
        self.stage
    }

    fn spawn_worker(&mut self) -> Result<(), String> {
        let (job_tx, job_rx) = mpsc::channel::<AutoPlanJob>();
        let (ev_tx, ev_rx) = mpsc::channel::<AutoPlanEvent>();
        thread::Builder::new()
            .name("nv-flight-planner".to_string())
            .spawn(move || planner_worker_loop(job_rx, ev_tx))
            .map_err(|err| format!("Failed to spawn nv-flight-planner thread: {err}"))?;
        self.job_tx = Some(job_tx);
        self.ev_rx = Some(ev_rx);
        Ok(())
    }
}

/// Main loop of the dedicated `"nv-flight-planner"` thread (blocks on `job_rx.recv()`).
fn planner_worker_loop(job_rx: Receiver<AutoPlanJob>, ev_tx: Sender<AutoPlanEvent>) {
    let registry = csg_scenes::default_scene_registry();
    let mut cached: Option<CachedOracle> = None;
    let mut gpu_baker: Option<Result<GpuVoxelBaker, String>> = None;

    while let Ok(job) = job_rx.recv() {
        let started = Instant::now();
        let outcome = execute_auto_plan_job(&registry, &mut cached, &mut gpu_baker, &job, &ev_tx);
        let event = match outcome {
            Ok(plan) => AutoPlanEvent::Ready {
                scene_id: job.scene_id,
                plan: Box::new(plan),
                elapsed_ms: started.elapsed().as_secs_f32() * 1000.0,
            },
            Err(err) => AutoPlanEvent::Failed(err),
        };
        if ev_tx.send(event).is_err() {
            break;
        }
    }
}

/// Executes one [`AutoPlanJob`], reusing the baked scene, volume, and `space_map` in `cached`
/// whenever `(scene_idx, road_mode, ground_kerb_mode, smart_fill, variant)` match, while clearing
/// `cache.catalog` on `sun_dir` changes and both `cache.catalog` + `cache.handover_costs` on
/// `req.limits` changes.
fn execute_auto_plan_job(
    registry: &csg_scene::SceneRegistry,
    cached: &mut Option<CachedOracle>,
    gpu_baker: &mut Option<Result<GpuVoxelBaker, String>>,
    job: &AutoPlanJob,
    ev_tx: &Sender<AutoPlanEvent>,
) -> Result<RailPlan, String> {
    let sun_dir = sanitize_sun_dir(job.sun_dir);
    let key = (
        job.scene_idx,
        job.road_mode,
        job.ground_kerb_mode,
        job.smart_fill,
        job.inspector_variant,
    );

    let cache_hit = cached.as_ref().is_some_and(|c| c.key == key);
    if !cache_hit {
        ev_tx
            .send(AutoPlanEvent::Stage(AutoPlanStage::BuildingScene))
            .map_err(|_| "Auto-plan caller disconnected".to_string())?;
        let reg_idx = registry_index_for_scene(job.scene_idx, registry.len())?;
        let module = registry
            .get_by_index(reg_idx)
            .ok_or_else(|| format!("Scene index {reg_idx} not found in SceneRegistry"))?;
        let build_cfg = SceneBuildConfig {
            variant_idx: job.inspector_variant,
            road_mode: job.road_mode,
            ground_kerb_mode: job.ground_kerb_mode,
            smart_fill: job.smart_fill,
            ..SceneBuildConfig::default()
        };
        let built = module.build(&build_cfg).map_err(|e| {
            format!(
                "CSG scene '{}' failed to build: {e}",
                module.descriptor().name
            )
        })?;

        ev_tx
            .send(AutoPlanEvent::Stage(AutoPlanStage::BakingVolume))
            .map_err(|_| "Auto-plan caller disconnected".to_string())?;
        if gpu_baker.is_none() {
            let init = GpuVoxelBaker::new();
            if let Err(ref err) = init {
                eprintln!(
                    "nv-demo: GPU voxel bake unavailable ({err}); falling back to CPU VoxelVolume::build"
                );
            }
            *gpu_baker = Some(init);
        }
        let baker = gpu_baker.as_mut().and_then(|res| res.as_mut().ok());
        let (oracle, objects) = build_oracle_with_baker(built, sun_dir, baker)?;
        *cached = Some(CachedOracle {
            key,
            sun_dir,
            limits: job.req.limits,
            oracle,
            objects,
            cache: PlanCache::default(),
        });
    } else if let Some(entry) = cached.as_mut() {
        let sun_changed = sun_dir_changed(entry.sun_dir, sun_dir);
        let limits_changed = entry.limits != job.req.limits;
        if sun_changed {
            entry.oracle.set_sun_dir(sun_dir);
            entry.sun_dir = sun_dir;
            entry.cache.catalog.clear();
        }
        if limits_changed {
            entry.limits = job.req.limits;
            entry.cache.catalog.clear();
            entry.cache.handover_costs.clear();
        }
    }

    ev_tx
        .send(AutoPlanEvent::Stage(AutoPlanStage::Planning))
        .map_err(|_| "Auto-plan caller disconnected".to_string())?;
    let entry = cached
        .as_mut()
        .ok_or_else(|| "Planner oracle cache unexpectedly empty".to_string())?;
    plan_world_track(&entry.oracle, &entry.objects, &mut entry.cache, &job.req)
        .map(PlanBodyPoseSource::into_plan)
        .map_err(|e| e.to_string())
}

/// Maps a 1-based `scene_idx` (`csg_mode`) to a `SceneRegistry` index, mirroring
/// `csg_scene_host.rs` so out-of-range or classic-stage (`0`) requests fail honestly.
fn registry_index_for_scene(scene_idx: u32, reg_count: usize) -> Result<usize, String> {
    if reg_count == 0 {
        return Err("CSG SceneRegistry is empty".to_string());
    }
    let idx = (scene_idx as usize).checked_sub(1).ok_or_else(|| {
        "Scene index 0 is the classic stage; switch to a CSG scene to auto-plan".to_string()
    })?;
    if idx >= reg_count {
        return Err(format!(
            "CSG scene index {scene_idx} is out of range (registry has {reg_count} scenes)"
        ));
    }
    Ok(idx)
}

#[cfg(test)]
mod tests {
    use csg_core::{CsgNode, Mat3x4, MaterialId, SemanticId, Vec3 as CsgVec3, semantic};
    use csg_scene::{Assembly, Instance, WorldScene};
    use flight_navigator::fnav_interest::ObjectTable;
    use flight_navigator::fnav_plan::Mode;
    use flight_navigator::fnav_space::{SceneOracle, Sem, Volume};
    use fnav_csg::CsgVolumeAdapter;

    use super::*;

    fn add_test_box(
        world: &mut WorldScene,
        name: &str,
        min: CsgVec3,
        max: CsgVec3,
        sem: SemanticId,
        user_id: u32,
    ) {
        let half = (max - min) * 0.5;
        let center = (min + max) * 0.5;
        let node = CsgNode::box3([half.x, half.y, half.z], MaterialId(1), sem);
        let lods = world.library.compile_lods(&node, &[0.0]);
        let asm = Assembly::from_compiled(name, lods, &world.library);
        let id = world.register_assembly(asm);
        world.add_static(Instance::new(id, Mat3x4::translate(center)).with_user_id(user_id));
    }

    #[test]
    fn fnav_csg_volume_adapter_matches_voxel_volume_bridge_coverage() {
        let mut world = WorldScene::new(32.0);
        add_test_box(
            &mut world,
            "ground",
            CsgVec3::new(0.0, -4.0, 0.0),
            CsgVec3::new(24.0, 0.0, 24.0),
            semantic::GROUND,
            10,
        );
        add_test_box(
            &mut world,
            "basin",
            CsgVec3::new(24.0, -2.0, 0.0),
            CsgVec3::new(36.0, 0.0, 24.0),
            semantic::WATER,
            14,
        );
        add_test_box(
            &mut world,
            "bridge_deck",
            CsgVec3::new(24.0, 6.0, 8.0),
            CsgVec3::new(36.0, 7.0, 16.0),
            semantic::ROAD,
            15,
        );

        let raw = VoxelVolume::build(&world, None, VoxelVolumeConfig::default());
        let expected_hash = raw.hash();
        let adapter = CsgVolumeAdapter::new(raw);
        assert_eq!(adapter.hash(), expected_hash);

        let under_bridge = Vec3::new(30.0, 3.0, 12.0);
        assert!(adapter.covered(under_bridge));
        assert!(adapter.semantics_at(under_bridge).is_covered());
        assert_eq!(adapter.semantics_at(under_bridge).sem(), Sem::Empty);
    }

    #[test]
    fn auto_planner_stage_and_registry_index_contracts() {
        assert!(!AutoPlanStage::Idle.is_busy());
        assert!(AutoPlanStage::BuildingScene.is_busy());
        assert!(AutoPlanStage::BakingVolume.is_busy());
        assert!(AutoPlanStage::Planning.is_busy());
        assert!(!AutoPlanStage::Planning.label().is_empty());

        assert!(registry_index_for_scene(0, 5).is_err());
        assert_eq!(registry_index_for_scene(1, 5), Ok(0));
        assert_eq!(registry_index_for_scene(5, 5), Ok(4));
        assert!(registry_index_for_scene(6, 5).is_err());
        assert!(registry_index_for_scene(1, 0).is_err());

        let mut planner = CsgAutoPlanner::new();
        assert!(!planner.is_busy());
        assert_eq!(planner.stage(), AutoPlanStage::Idle);
        assert!(planner.poll().is_empty());

        let sun_a = Some(Vec3::new(0.4, 0.8, 0.4).normalize());
        let sun_b = Some(Vec3::new(-0.4, 0.8, 0.4).normalize());
        assert!(!sun_dir_changed(None, None));
        assert!(!sun_dir_changed(sun_a, sun_a));
        assert!(sun_dir_changed(sun_a, sun_b));
        assert!(sun_dir_changed(sun_a, None));
    }

    #[test]
    #[ignore = "Builds the full Gundo CSG scene, bakes R0 VoxelVolume, and runs deterministic 30 s flight planning"]
    fn csg_oracle_auto_plans_gundo_deterministically() {
        let registry = csg_scenes::default_scene_registry();
        let module = registry
            .get("gundo")
            .expect("gundo scene must be registered in default_scene_registry");
        let built = module
            .build(&SceneBuildConfig::default())
            .expect("gundo scene must build cleanly");

        let sun_dir = Vec3::new(0.45, 0.72, 0.52).normalize();
        let (oracle, objects) = build_oracle_with_baker(built, Some(sun_dir), None)
            .expect("build_oracle_with_baker must succeed on gundo");

        assert!(
            objects.total_instances() > 0,
            "gundo object table must be populated"
        );
        assert!(
            !oracle.streets().edges.is_empty(),
            "gundo street graph must be populated"
        );

        let req = PlanRequest {
            duration_s: 30.0,
            seed: 42,
            mode: Mode::Deterministic,
            ..PlanRequest::default()
        };
        let mut cache = PlanCache::default();

        let src_a = plan_world_track(&oracle, &objects, &mut cache, &req)
            .expect("deterministic 30 s plan on gundo must succeed");
        let src_b = plan_world_track(&oracle, &objects, &mut cache, &req)
            .expect("cached deterministic 30 s plan on gundo must succeed");
        let plan_a = src_a.plan();
        let plan_b = src_b.plan();

        assert!(!plan_a.shots.is_empty(), "planned tour must contain shots");
        assert!(
            !plan_a.track.segments.is_empty(),
            "planned track must contain segments"
        );
        assert_eq!(
            plan_a.report.open_water_samples, 0,
            "no samples over disallowed open water"
        );
        assert!(
            plan_a.report.min_clearance_m > 0.0,
            "track must maintain positive clearance (got {:.3} m)",
            plan_a.report.min_clearance_m
        );
        assert_eq!(
            plan_a.report.plan_id, plan_b.report.plan_id,
            "deterministic planning must yield identical plan_id"
        );
        assert_eq!(
            plan_a.track, plan_b.track,
            "deterministic planning must yield bit-identical tracks"
        );
    }
}
