//! Live `csg-scene` R0 (`VoxelVolume`) + R1 (`WorldRayQuery`) bridge to `flight-navigator-rs`
//! (`Volume`, `SceneOracle`, `ObjectTable`) and asynchronous Auto-Plan background worker.
//!
//! **What.** Implements [`CsgVolumeAdapter`] (1-to-1 `fnav_space::Volume` over
//! [`csg_scene::VoxelVolume`]), [`CsgObjectTable`] (`fnav_interest::ObjectTable` over static and
//! dynamic `WorldScene` instances), [`CsgSceneOracle`] (`fnav_space::SceneOracle` pairing
//! `VoxelVolume` + `WorldRayQuery` + extracted `StreetGraph` and sanitized `Landmark`s), and
//! [`CsgAutoPlanner`] (a single non-blocking background worker thread driving `fnav_plan::plan`
//! with per-scene caching across repeated plans).
//! **Why.** Keeps `flight-navigator-rs` 100% decoupled from `csg-rs` while letting `nv-demo`
//! generate collision-verified cinematic tours directly from live analytical CSG worlds without
//! stalling the 60 Hz UI or OptiX render loop.
//! **How.** [`CsgVolumeAdapter`] pre-flattens every `1.0 m` XZ column's [`csg_scene::ColumnSpan`]s
//! into contiguous [`flight_navigator::fnav_space::Span`] storage so `column_spans(x, z)` returns
//! a zero-allocation borrowed slice matching [`csg_scene::VoxelVolume::column_spans`] identically.
//! [`CsgAutoPlanner`] lazily spawns one `"nv-flight-planner"` thread blocked on an `mpsc` channel
//! (zero busy-waiting), lazily initializes one reusable [`crate::flight::voxel_gpu::GpuVoxelBaker`]
//! to accelerate R0 `VoxelVolume` column baking on CUDA (falling back cleanly to exact CPU
//! `VoxelVolume::build` if CUDA is unavailable or overflows), caches the most recent
//! `(scene_idx, road_mode, inspector_variant)` oracle and
//! [`flight_navigator::fnav_plan::PlanCache`] (selectively invalidating `cache.catalog` on
//! `sun_dir` changes and both `cache.catalog` + `cache.handover_costs` on `req.limits` changes
//! while keeping the baked volume and `space_map` warm), and reports typed [`AutoPlanEvent`] stage
//! progress, ready [`flight_navigator::fnav_plan::RailPlan`]s, or errors back to `FlightWorkspace`.
//! **Uses.** `csg_core`, `csg_scene`, `csg_scenes`, `flight_navigator`, `glam`, `std::sync::mpsc`,
//! `crate::flight::voxel_gpu`.
//! **Used by.** `crate::flight`, `crate::flight::lab`, `crate::flight::ui`, `crate::main`.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Instant;

use csg_core::{Aabb as CsgAabb, SemanticId, Vec3 as CsgVec3};
use csg_scene::voxel_volume::{
    COARSE_VOXEL_M, VoxelClass, VoxelSem as CsgVoxelSem, VoxelVolume, VoxelVolumeConfig,
    voxel_class_from_semantic,
};
use csg_scene::world_ray::WorldRayQuery;
use csg_scene::{RoadMode, SceneBuildConfig, SceneBuilt, SceneHints, WorldScene, inst_flags};
use flight_navigator::camera_guide::GuideLimits;
use flight_navigator::fnav_interest::{ObjectInfo, ObjectTable, SimpleObjectTable};
use flight_navigator::fnav_plan::{self, CpuRayProber, PlanCache, PlanRequest, RailPlan};
use flight_navigator::fnav_space::{
    Aabb, Hit, Landmark, SceneOracle, Sem, Span, StreetEdge, StreetGraph, Volume,
    VoxelSem as FnavVoxelSem,
};
use glam::Vec3;

use crate::flight::voxel_gpu::GpuVoxelBaker;

/// Minimum ray parameter (metres) passed to [`WorldRayQuery::first_hit`] to prevent self-hits at
/// `t = 0` while preserving exact clearance checks down to 0.01 mm.
const RAY_TMIN_M: f32 = 1.0e-5;

/// High-bit tag applied to flat static instance indices when an instance carries `user_id == 0`.
///
/// Why: guarantees a non-zero, collision-free `inst_id` linking [`CsgSceneOracle::raycast`] hits
/// to [`CsgObjectTable`] metadata even for synthetic or anonymous static instances.
const FALLBACK_INST_ID_BIT: u32 = 1 << 31;

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
            Self::BakingVolume => "Baking voxel volume & ray BVH...",
            Self::Planning => "Planning cinematic flight...",
        }
    }

    /// Returns `true` while an asynchronous planning job is in flight.
    ///
    /// Why: lets callers gate new submissions and show progress controls with one predicate.
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
    /// Inspector variant index (`0` for non-inspector scenes).
    pub inspector_variant: u32,
    /// World-space unit direction pointing toward the scene sun (`[x, y, z]`).
    pub sun_dir: [f32; 3],
    /// Serializable flight-planning request (`duration_s`, `seed`, `mode`, `cut_density`, `limits`, `edits`).
    pub req: PlanRequest,
}

/// Asynchronous status or completion event emitted by [`CsgAutoPlanner::poll`].
///
/// Why: decouples long-running CPU baking and planning from the 60 Hz UI loop while delivering
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

/// Zero-allocation [`Volume`] adapter wrapping a `csg-scene` R0 [`VoxelVolume`].
///
/// Why: `flight_navigator::fnav_space::Volume::column_spans` returns a borrowed `&[Span]`, whereas
/// [`VoxelVolume::column_spans`] returns `&[csg_scene::ColumnSpan]`. Pre-converting every `1.0 m`
/// XZ column once at construction guarantees exact 1-to-1 parity with zero per-query allocations.
pub struct CsgVolumeAdapter {
    volume: VoxelVolume,
    bounds: Aabb,
    nx: usize,
    nz: usize,
    spans: Vec<Span>,
    column_ranges: Vec<(usize, usize)>,
}

impl CsgVolumeAdapter {
    /// Wraps `volume`, pre-flattening all `1.0 m` column spans into `fnav_space::Span` storage.
    ///
    /// Why: pays the linear span-conversion cost once up front so `SpaceMap::build`, A*, CHOMP,
    /// and 60 Hz validation query `column_spans(x, z)` in O(1) with zero heap churn.
    #[must_use]
    pub fn new(volume: VoxelVolume) -> Self {
        let raw_bounds = volume.bounds();
        let bounds = csg_aabb_to_fnav(raw_bounds);
        let (nx, _ny, nz) = volume.grid_dims();
        let col_count = nx.saturating_mul(nz);

        let mut spans = Vec::with_capacity(col_count.saturating_mul(2));
        let mut column_ranges = Vec::with_capacity(col_count);

        for iz in 0..nz {
            let z = raw_bounds.min.z + (iz as f32 + 0.5) * COARSE_VOXEL_M;
            for ix in 0..nx {
                let x = raw_bounds.min.x + (ix as f32 + 0.5) * COARSE_VOXEL_M;
                let start = spans.len();
                for sp in volume.column_spans(x, z) {
                    spans.push(Span {
                        y0: sp.y0,
                        y1: sp.y1,
                        sem: voxel_class_to_sem(sp.class),
                    });
                }
                column_ranges.push((start, spans.len()));
            }
        }

        Self {
            volume,
            bounds,
            nx,
            nz,
            spans,
            column_ranges,
        }
    }

    /// Borrows the underlying `csg-scene` [`VoxelVolume`] for grid-dimension validation and parity tests.
    #[must_use]
    pub const fn inner(&self) -> &VoxelVolume {
        &self.volume
    }

    /// Computes the flat `iz * nx + ix` column index matching `VoxelVolume::column_index` 1-to-1.
    #[inline]
    fn column_index(&self, x: f32, z: f32) -> Option<usize> {
        if self.nx == 0 || self.nz == 0 || !x.is_finite() || !z.is_finite() {
            return None;
        }
        let fx = x - self.bounds.min.x;
        let fz = z - self.bounds.min.z;
        if fx < 0.0 || fz < 0.0 || fx > self.nx as f32 || fz > self.nz as f32 {
            return None;
        }
        let ix = (fx.floor() as isize).clamp(0, (self.nx - 1) as isize) as usize;
        let iz = (fz.floor() as isize).clamp(0, (self.nz - 1) as isize) as usize;
        Some(iz * self.nx + ix)
    }
}

impl Volume for CsgVolumeAdapter {
    #[inline]
    fn bounds(&self) -> Aabb {
        self.bounds
    }

    #[inline]
    fn hash(&self) -> [u8; 32] {
        self.volume.hash()
    }

    #[inline]
    fn column_spans(&self, x: f32, z: f32) -> &[Span] {
        let Some(col) = self.column_index(x, z) else {
            return &[];
        };
        let Some(&(start, end)) = self.column_ranges.get(col) else {
            return &[];
        };
        self.spans.get(start..end).unwrap_or(&[])
    }

    #[inline]
    fn semantics_at(&self, p: Vec3) -> FnavVoxelSem {
        voxel_sem_to_fnav(self.volume.semantics_at(glam_to_csg(p)))
    }

    #[inline]
    fn covered(&self, p: Vec3) -> bool {
        self.volume.covered(glam_to_csg(p))
    }

    #[inline]
    fn distance(&self, p: Vec3) -> f32 {
        self.volume.distance(glam_to_csg(p))
    }

    #[inline]
    fn gradient(&self, p: Vec3) -> Vec3 {
        csg_to_glam(self.volume.gradient(glam_to_csg(p)))
    }

    #[inline]
    fn ground_height(&self, x: f32, z: f32) -> Option<f32> {
        self.volume.ground_height(x, z)
    }

    #[inline]
    fn roof_height(&self, x: f32, z: f32) -> Option<f32> {
        self.volume.roof_height(x, z)
    }

    #[inline]
    fn voxel_size_at(&self, p: Vec3) -> f32 {
        self.volume.voxel_size_at(glam_to_csg(p))
    }

    #[inline]
    fn is_thin_sharp(&self, p: Vec3) -> bool {
        self.volume.is_thin_sharp(glam_to_csg(p))
    }
}

/// Deterministic [`ObjectTable`] extracted from a CSG [`WorldScene`] for `fnav-interest` scoring.
///
/// Why: `fnav-interest` weights rare architectural landmarks (`idf`) and hero / dynamic subjects
/// across probe frames without coupling to `csg-scene` types; backing the catalog with
/// [`SimpleObjectTable`] (`BTreeMap`) guarantees bit-identical deterministic iteration.
#[derive(Clone, Debug, Default)]
pub struct CsgObjectTable {
    inner: SimpleObjectTable,
}

impl CsgObjectTable {
    /// Builds a deterministic [`CsgObjectTable`] from every visible static and dynamic instance in
    /// `world`, marking hero instances from `inst_flags::OVERRIDDEN` and curated `landmarks`.
    ///
    /// Why: precomputes [`csg_core::CompiledLibrary::semantic_masks`] once so per-instance semantic
    /// classification runs in O(parts) without re-evaluating RPN bytecode per instance.
    #[must_use]
    pub fn build(world: &WorldScene, landmarks: &[Landmark]) -> Self {
        let compound_masks = world.library.semantic_masks();
        let hero_ids: BTreeSet<u32> = landmarks
            .iter()
            .filter(|lm| lm.is_hero)
            .filter_map(|lm| lm.inst_user_id)
            .collect();
        let hero_bounds: Vec<Aabb> = landmarks
            .iter()
            .filter(|lm| lm.is_hero)
            .map(|lm| lm.bounds)
            .collect();

        let mut table = SimpleObjectTable::new();
        let mut static_idx = 0u32;
        for chunk in world.chunks() {
            for inst in chunk.instances() {
                let idx = static_idx;
                static_idx = static_idx.saturating_add(1);
                if !inst.is_visible() {
                    continue;
                }
                let Some(asm) = world.assembly(inst.assembly_id) else {
                    continue;
                };
                let inst_id = stable_inst_id(inst.user_id, idx);
                let sem = instance_semantic(inst.flags, asm, &compound_masks);
                let world_aabb = inst.xform.xform_aabb(asm.local_aabb);
                let size_m = finite_aabb_extent_m(world_aabb);
                let in_hero_bounds = matches!(sem, Sem::Building | Sem::Structure)
                    && world_aabb.is_finite()
                    && !world_aabb.is_empty()
                    && {
                        let c = csg_aabb_to_fnav(world_aabb).center();
                        hero_bounds.iter().any(|b| b.contains(c))
                    };
                let is_hero = (inst.flags & inst_flags::OVERRIDDEN) != 0
                    || hero_ids.contains(&inst_id)
                    || in_hero_bounds;
                table.insert(ObjectInfo {
                    inst_id,
                    assembly: inst.assembly_id.0,
                    sem,
                    is_hero,
                    dynamic: (inst.flags & inst_flags::DYNAMIC) != 0,
                    size_m,
                });
            }
        }

        for inst in world.dynamic_instances() {
            if !inst.is_visible() || inst.user_id == 0 {
                continue;
            }
            if table.object_info(inst.user_id).is_some() {
                continue;
            }
            let Some(asm) = world.assembly(inst.assembly_id) else {
                continue;
            };
            let sem = instance_semantic(inst.flags, asm, &compound_masks);
            let world_aabb = inst.xform.xform_aabb(asm.local_aabb);
            let size_m = finite_aabb_extent_m(world_aabb);
            table.insert(ObjectInfo {
                inst_id: inst.user_id,
                assembly: inst.assembly_id.0,
                sem,
                is_hero: (inst.flags & inst_flags::OVERRIDDEN) != 0,
                dynamic: true,
                size_m,
            });
        }

        Self { inner: table }
    }
}

impl ObjectTable for CsgObjectTable {
    #[inline]
    fn total_instances(&self) -> u32 {
        self.inner.total_instances()
    }

    #[inline]
    fn assembly_count(&self, assembly: u32) -> u32 {
        self.inner.assembly_count(assembly)
    }

    #[inline]
    fn object_info(&self, inst_id: u32) -> Option<ObjectInfo> {
        self.inner.object_info(inst_id)
    }
}

/// Complete [`SceneOracle`] backed by live `csg-scene` R0 [`VoxelVolume`] + R1 [`WorldRayQuery`].
///
/// Why: satisfies the planner invariant "the distance field proposes; exact rays decide" directly
/// against analytical CSG solids with zero mesh approximation and deterministic CPU execution.
pub struct CsgSceneOracle {
    volume: CsgVolumeAdapter,
    world: WorldScene,
    ray_query: WorldRayQuery,
    streets: StreetGraph,
    landmarks: Vec<Landmark>,
    sun_dir: Option<Vec3>,
}

impl CsgSceneOracle {
    /// Builds a [`CsgSceneOracle`] and matching [`CsgObjectTable`] from a built CSG scene,
    /// accelerating R0 [`VoxelVolume`] column-span baking via `gpu_baker` when provided and
    /// falling back transparently to exact CPU [`VoxelVolume::build`] on any GPU error or
    /// overflow.
    ///
    /// Why: cuts full-city `VoxelVolume` baking (e.g. Gundo) from ~2 s on CPU to ~265 ms on GPU
    /// without ever accepting truncated GPU spans or failing on non-CUDA hosts.
    ///
    /// # Errors
    /// Returns an error if `built.world` contains no geometry or produces an invalid empty volume.
    pub fn build_with_baker(
        built: SceneBuilt,
        sun_dir: Option<Vec3>,
        gpu_baker: Option<&mut GpuVoxelBaker>,
    ) -> Result<(Self, CsgObjectTable), String> {
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
        let volume = CsgVolumeAdapter::new(raw_volume);
        let (nx, _ny, nz) = volume.inner().grid_dims();
        if nx == 0 || nz == 0 {
            return Err("Baked VoxelVolume has zero horizontal grid extent".to_string());
        }

        let streets = extract_street_graph(&built.hints);
        let landmarks = extract_landmarks(&built.world, &built.hints, &volume);
        let objects = CsgObjectTable::build(&built.world, &landmarks);
        let normalized_sun = sun_dir.and_then(|d| {
            (d.is_finite() && d.length_squared() > 1.0e-6).then(|| d.normalize())
        });

        let oracle = Self {
            volume,
            world: built.world,
            ray_query,
            streets,
            landmarks,
            sun_dir: normalized_sun,
        };
        Ok((oracle, objects))
    }

    /// Updates the sun direction used by subject-lighting probes on a cached oracle.
    ///
    /// Why: lets [`CsgAutoPlanner`] keep the baked `VoxelVolume` and `space_map` warm across
    /// time-of-day or lighting tweaks while clearing only `cache.catalog` so lighting scores are
    /// recomputed against the new sun vector.
    pub fn set_sun_dir(&mut self, sun_dir: Option<Vec3>) {
        self.sun_dir = sun_dir.and_then(|d| {
            (d.is_finite() && d.length_squared() > 1.0e-6).then(|| d.normalize())
        });
    }
}

impl SceneOracle for CsgSceneOracle {
    #[inline]
    fn volume(&self) -> &dyn Volume {
        &self.volume
    }

    #[inline]
    fn raycast(&self, origin: Vec3, dir: Vec3, t_max: f32) -> Option<Hit> {
        if !t_max.is_finite() || t_max <= RAY_TMIN_M {
            return None;
        }
        let hit = self.ray_query.first_hit(
            &self.world.library,
            glam_to_csg(origin),
            glam_to_csg(dir),
            RAY_TMIN_M,
            t_max,
        )?;
        Some(Hit {
            t: hit.t,
            normal: csg_to_glam(hit.normal),
            entering: hit.entering,
            sem: voxel_class_to_sem(voxel_class_from_semantic(hit.sem_id)),
            inst_user_id: stable_inst_id(hit.user_id, hit.instance_idx),
            assembly: hit.assembly_id.0,
        })
    }

    #[inline]
    fn streets(&self) -> &StreetGraph {
        &self.streets
    }

    #[inline]
    fn landmarks(&self) -> &[Landmark] {
        &self.landmarks
    }

    #[inline]
    fn sun_dir(&self) -> Option<Vec3> {
        self.sun_dir
    }
}

/// Cached scene oracle, object catalog, and `PlanCache` held inside the worker thread.
///
/// Tracks `sun_dir` and `limits` alongside the scene geometry `key` so lighting or dynamics-limit
/// changes invalidate only the affected `PlanCache` tables without rebuilding the scene or volume.
struct CachedOracle {
    key: (u32, RoadMode, u32),
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

/// Asynchronous background flight-planning coordinator for `nv-demo`.
///
/// Why: building a CSG scene, baking its `VoxelVolume`, and running `fnav_plan::plan` can take
/// seconds; isolating that work on one dedicated `mpsc`-driven worker thread keeps the 60 Hz UI
/// and OptiX viewport completely responsive while reusing the baked scene and `PlanCache` across
/// repeated seed, plan-mode, duration, and cut-density iterations (and selectively invalidating
/// cached catalog/handover tables on `sun_dir` or `GuideLimits` changes).
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
/// whenever `(scene_idx, road_mode, variant)` match, while clearing `cache.catalog` on `sun_dir`
/// changes and both `cache.catalog` + `cache.handover_costs` on `req.limits` changes.
fn execute_auto_plan_job(
    registry: &csg_scene::SceneRegistry,
    cached: &mut Option<CachedOracle>,
    gpu_baker: &mut Option<Result<GpuVoxelBaker, String>>,
    job: &AutoPlanJob,
    ev_tx: &Sender<AutoPlanEvent>,
) -> Result<RailPlan, String> {
    let sun_dir = sanitize_sun_dir(job.sun_dir);
    let key = (job.scene_idx, job.road_mode, job.inspector_variant);

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
        let (oracle, objects) = CsgSceneOracle::build_with_baker(built, sun_dir, baker)?;
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
    let prober = CpuRayProber::new(&entry.oracle);
    fnav_plan::plan(
        &entry.oracle,
        &prober,
        &entry.objects,
        &mut entry.cache,
        &job.req,
    )
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

/// Converts a `csg_core::Vec3` to `glam::Vec3`.
#[inline]
const fn csg_to_glam(v: CsgVec3) -> Vec3 {
    Vec3::new(v.x, v.y, v.z)
}

/// Converts a `glam::Vec3` to `csg_core::Vec3`.
#[inline]
const fn glam_to_csg(v: Vec3) -> CsgVec3 {
    CsgVec3::new(v.x, v.y, v.z)
}

/// Converts a `csg_core::Aabb` into a normalized `fnav_space::Aabb`.
#[inline]
fn csg_aabb_to_fnav(a: CsgAabb) -> Aabb {
    Aabb::new(csg_to_glam(a.min), csg_to_glam(a.max))
}

/// Exhaustive 1-to-1 conversion from `csg_scene::VoxelClass` to `fnav_space::Sem`.
#[inline]
const fn voxel_class_to_sem(c: VoxelClass) -> Sem {
    match c {
        VoxelClass::Empty => Sem::Empty,
        VoxelClass::Ground => Sem::Ground,
        VoxelClass::Road => Sem::Road,
        VoxelClass::Sidewalk => Sem::Sidewalk,
        VoxelClass::Water => Sem::Water,
        VoxelClass::Building => Sem::Building,
        VoxelClass::Vegetation => Sem::Vegetation,
        VoxelClass::Bridge => Sem::Bridge,
        VoxelClass::Structure => Sem::Structure,
        VoxelClass::DynamicLane => Sem::DynamicLane,
    }
}

/// Converts a packed `csg_scene::VoxelSem` into `fnav_space::VoxelSem`, preserving `COVERED_BIT`.
#[inline]
const fn voxel_sem_to_fnav(s: CsgVoxelSem) -> FnavVoxelSem {
    FnavVoxelSem::new(voxel_class_to_sem(s.class()), s.is_covered())
}

/// Normalizes a raw `[f32; 3]` sun direction, discarding zero or non-finite vectors.
#[inline]
fn sanitize_sun_dir(dir: [f32; 3]) -> Option<Vec3> {
    let v = Vec3::from_array(dir);
    (v.is_finite() && v.length_squared() > 1.0e-6).then(|| v.normalize())
}

/// Returns a stable, non-zero instance ID shared identically by [`CsgSceneOracle::raycast`] and
/// [`CsgObjectTable`].
#[inline]
const fn stable_inst_id(user_id: u32, flat_idx: u32) -> u32 {
    if user_id != 0 {
        user_id
    } else {
        FALLBACK_INST_ID_BIT | (flat_idx & !FALLBACK_INST_ID_BIT)
    }
}

/// Physical precedence order for selecting one representative [`Sem`] from an assembly's bitmask.
#[inline]
const fn sem_priority(sem: Sem) -> u8 {
    match sem {
        Sem::Building => 9,
        Sem::Structure => 8,
        Sem::Bridge => 7,
        Sem::Vegetation => 6,
        Sem::Road => 5,
        Sem::Sidewalk => 4,
        Sem::Water => 3,
        Sem::Ground => 2,
        Sem::DynamicLane => 1,
        Sem::Empty => 0,
    }
}

/// Resolves the dominant [`Sem`] for an instance from its `inst_flags` semantic override or LOD0
/// compound semantic bitmasks.
fn instance_semantic(flags: u32, asm: &csg_scene::Assembly, compound_masks: &[u64]) -> Sem {
    if let Some(sem_id) = inst_flags::semantic(flags) {
        return voxel_class_to_sem(voxel_class_from_semantic(sem_id));
    }
    let mut combined_mask = 0u64;
    for part in &asm.parts {
        for &cid in part.lods.compounds_for_lod(0) {
            if let Some(&mask) = compound_masks.get(cid.0 as usize) {
                combined_mask |= mask;
            }
        }
    }
    if combined_mask == 0 {
        return Sem::Structure;
    }
    let mut best = Sem::Empty;
    let mut best_prio = 0u8;
    for bit in 0..64u16 {
        if (combined_mask & (1u64 << bit)) != 0 {
            let sem = voxel_class_to_sem(voxel_class_from_semantic(SemanticId(bit)));
            let prio = sem_priority(sem);
            if prio > best_prio {
                best = sem;
                best_prio = prio;
            }
        }
    }
    if best == Sem::Empty {
        Sem::Structure
    } else {
        best
    }
}

/// Returns a bounded finite linear extent (metres) for an instance's world AABB.
#[inline]
fn finite_aabb_extent_m(aabb: CsgAabb) -> f32 {
    if aabb.is_empty() || !aabb.is_finite() {
        return 1.0;
    }
    let size = csg_aabb_to_fnav(aabb).size();
    let extent = size.x.max(size.y).max(size.z);
    if extent.is_finite() {
        extent.clamp(0.1, 10_000.0)
    } else {
        1.0
    }
}

/// Quantizes an XZ world point onto a `0.5 m` grid for deterministic intersection node welding.
#[inline]
fn quantize_xz_node(p: Vec3) -> (i32, i32) {
    ((p.x * 2.0).round() as i32, (p.z * 2.0).round() as i32)
}

/// Computes total Euclidean polyline length in metres for finite `glam::Vec3` vertices.
fn polyline_length_m(pts: &[Vec3]) -> f32 {
    pts.windows(2)
        .map(|w| (w[1] - w[0]).length())
        .filter(|d| d.is_finite())
        .sum()
}

/// Builds a deterministic [`StreetGraph`] from [`SceneHints`] road spines, corridors, and (as a
/// fallback when no streets exist) pedestrian walkways.
fn extract_street_graph(hints: &SceneHints) -> StreetGraph {
    let mut nodes: Vec<Vec3> = Vec::new();
    let mut node_index: BTreeMap<(i32, i32), u32> = BTreeMap::new();
    let mut edges: Vec<StreetEdge> = Vec::new();
    let mut midpoints: Vec<Vec3> = Vec::new();

    let mut intern_node = |p: Vec3| -> u32 {
        let key = quantize_xz_node(p);
        if let Some(&id) = node_index.get(&key) {
            return id;
        }
        let id = u32::try_from(nodes.len()).unwrap_or(u32::MAX);
        nodes.push(p);
        node_index.insert(key, id);
        id
    };

    let mut push_edge = |edges: &mut Vec<StreetEdge>,
                         raw_pts: &[CsgVec3],
                         min_len_m: f32,
                         width_m: f32,
                         lanes: u8,
                         oneway: bool,
                         dedup_midpoints: bool| {
        let pts: Vec<Vec3> = raw_pts
            .iter()
            .map(|&p| csg_to_glam(p))
            .filter(|p| p.is_finite())
            .collect();
        if pts.len() < 2 || polyline_length_m(&pts) < min_len_m {
            return;
        }
        let mid = pts[pts.len() / 2];
        if dedup_midpoints
            && midpoints
                .iter()
                .any(|&m| Vec3::new(m.x - mid.x, 0.0, m.z - mid.z).length_squared() < 16.0)
        {
            return;
        }
        let from = intern_node(pts[0]);
        let to = intern_node(pts[pts.len() - 1]);
        let id = u32::try_from(edges.len()).unwrap_or(u32::MAX);
        let safe_width = if width_m.is_finite() {
            width_m.clamp(3.0, 60.0)
        } else {
            8.0
        };
        edges.push(StreetEdge {
            id,
            from,
            to,
            points: pts,
            width_m: safe_width,
            lanes: lanes.max(1),
            oneway,
        });
        midpoints.push(mid);
    };

    for spine in &hints.road_spines {
        push_edge(
            &mut edges,
            &spine.pts,
            1.0,
            spine.width_m,
            spine.lanes,
            spine.oneway,
            false,
        );
    }

    for corridor in &hints.corridors {
        push_edge(&mut edges, &corridor.pts, 6.0, corridor.width_m, 1, false, true);
    }

    if edges.is_empty() {
        for walk in &hints.walkways {
            if !walk.is_crosswalk {
                push_edge(
                    &mut edges,
                    &walk.pts,
                    8.0,
                    walk.width_m.max(3.0),
                    1,
                    false,
                    true,
                );
            }
        }
    }

    StreetGraph { nodes, edges }
}

/// Sanitizes a landmark display label so residential street addresses never leak into plans or UI.
///
/// Why: project privacy rules forbid residential street addresses in labels, names, and tests;
/// any blank label or label starting with a house number / matching a street-address pattern is
/// replaced with a neutral indexed label (`"Landmark N"`).
fn sanitize_landmark_name(raw: &str, fallback_idx: usize) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() || looks_like_street_address(trimmed) {
        return format!("Landmark {}", fallback_idx + 1);
    }
    trimmed.to_string()
}

/// Heuristic detector for residential/postal street-address strings (e.g. `"420 Maple Ave"`).
fn looks_like_street_address(label: &str) -> bool {
    const STREET_SUFFIXES: &[&str] = &[
        "st", "st.", "street", "ave", "ave.", "avenue", "blvd", "blvd.", "boulevard", "dr", "dr.",
        "drive", "rd", "rd.", "road", "ln", "ln.", "lane", "ct", "ct.", "court", "pl", "pl.",
        "place", "way", "cir", "circle", "ter", "terrace", "pkwy", "parkway", "apt", "unit",
    ];

    let tokens: Vec<&str> = label.split_whitespace().collect();
    if tokens.len() < 2 {
        return false;
    }

    // House-number prefix (e.g. "420", "420A", "12-34"): reject immediately.
    let first = tokens[0];
    let first_has_digit = first.bytes().any(|b| b.is_ascii_digit());
    let first_is_house_num = first_has_digit
        && first
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'#');
    if first_is_house_num {
        return true;
    }

    // Embedded street number followed later by a thoroughfare suffix.
    let has_number_token = tokens.iter().any(|t| {
        !t.is_empty()
            && t.bytes().all(|b| b.is_ascii_digit() || b == b'-')
            && t.bytes().any(|b| b.is_ascii_digit())
    });
    if has_number_token {
        for tok in &tokens {
            let lower = tok
                .trim_matches(|c: char| c == ',' || c == ';')
                .to_ascii_lowercase();
            if STREET_SUFFIXES.contains(&lower.as_str()) {
                return true;
            }
        }
    }

    false
}

/// One static building/structure candidate eligible to back a curated or auto-detected [`Landmark`].
struct StaticLandmarkCandidate {
    inst_id: u32,
    bounds: Aabb,
    center: Vec3,
    volume_m3: f32,
    is_overridden: bool,
}

/// Extracts curated landmarks from [`SceneHints::landmarks`] (with fallback promotion of notable
/// static structures when hints are empty) and resolves matching static instance IDs.
fn extract_landmarks(
    world: &WorldScene,
    hints: &SceneHints,
    volume: &CsgVolumeAdapter,
) -> Vec<Landmark> {
    let compound_masks = world.library.semantic_masks();

    // Pre-collect finite static building/structure/overridden instances for deterministic matching.
    let mut static_candidates: Vec<StaticLandmarkCandidate> = Vec::new();
    let mut static_idx = 0u32;
    for chunk in world.chunks() {
        for inst in chunk.instances() {
            let idx = static_idx;
            static_idx = static_idx.saturating_add(1);
            if !inst.is_visible() {
                continue;
            }
            let Some(asm) = world.assembly(inst.assembly_id) else {
                continue;
            };
            let sem = instance_semantic(inst.flags, asm, &compound_masks);
            let is_overridden = (inst.flags & inst_flags::OVERRIDDEN) != 0;
            if !is_overridden && !matches!(sem, Sem::Building | Sem::Structure) {
                continue;
            }
            let raw_aabb = inst.xform.xform_aabb(asm.local_aabb);
            if raw_aabb.is_empty() || !raw_aabb.is_finite() {
                continue;
            }
            let bounds = csg_aabb_to_fnav(raw_aabb);
            let sz = bounds.size();
            let volume_m3 = sz.x * sz.y * sz.z;
            if !volume_m3.is_finite() || volume_m3 < 1.0 {
                continue;
            }
            static_candidates.push(StaticLandmarkCandidate {
                inst_id: stable_inst_id(inst.user_id, idx),
                bounds,
                center: bounds.center(),
                volume_m3,
                is_overridden,
            });
        }
    }

    let mut landmarks = Vec::with_capacity(hints.landmarks.len());
    for (idx, hint) in hints.landmarks.iter().enumerate() {
        let center = csg_to_glam(hint.center);
        if !center.is_finite() {
            continue;
        }
        let bounds = if !hint.bounds.is_empty() && hint.bounds.is_finite() {
            csg_aabb_to_fnav(hint.bounds)
        } else {
            let r = if hint.hero_radius.is_finite() {
                hint.hero_radius.clamp(4.0, 40.0)
            } else {
                10.0
            };
            Aabb::new(
                center - Vec3::new(r, r * 0.5, r),
                center + Vec3::new(r, r, r),
            )
        };
        let ground_y = volume.ground_height(center.x, center.z).unwrap_or(bounds.min.y);
        let height_m = (bounds.max.y - ground_y).max(bounds.size().y).max(2.0);
        let pvd = csg_to_glam(hint.preferred_view_dir);
        let preferred_view_dir =
            (pvd.is_finite() && pvd.length_squared() > 1.0e-4).then(|| pvd.normalize());

        let inst_user_id = static_candidates
            .iter()
            .filter(|c| c.bounds.contains(center))
            .min_by(|a, b| {
                b.is_overridden
                    .cmp(&a.is_overridden)
                    .then_with(|| {
                        let da = (a.center - center).length_squared();
                        let db = (b.center - center).length_squared();
                        da.total_cmp(&db)
                    })
                    .then_with(|| a.inst_id.cmp(&b.inst_id))
            })
            .map(|c| c.inst_id);

        let id = u32::try_from(landmarks.len()).unwrap_or(u32::MAX);
        landmarks.push(Landmark {
            id,
            inst_user_id,
            name: sanitize_landmark_name(&hint.name, idx),
            center,
            bounds,
            preferred_view_dir,
            height_m,
            is_hero: hint.priority >= 128 || idx == 0,
        });
    }

    // Fallback for hint-less scenes (e.g. Studio Asset Inspector or synthetic worlds):
    // promote up to 4 largest static structures deterministically so hero framing still works.
    if landmarks.is_empty() && !static_candidates.is_empty() {
        static_candidates.sort_by(|a, b| {
            b.is_overridden
                .cmp(&a.is_overridden)
                .then_with(|| b.volume_m3.total_cmp(&a.volume_m3))
                .then_with(|| a.inst_id.cmp(&b.inst_id))
        });
        for (idx, cand) in static_candidates.into_iter().take(4).enumerate() {
            let ground_y = volume
                .ground_height(cand.center.x, cand.center.z)
                .unwrap_or(cand.bounds.min.y);
            let height_m = (cand.bounds.max.y - ground_y)
                .max(cand.bounds.size().y)
                .max(2.0);
            landmarks.push(Landmark {
                id: u32::try_from(idx).unwrap_or(u32::MAX),
                inst_user_id: Some(cand.inst_id),
                name: format!("Landmark {}", idx + 1),
                center: cand.center,
                bounds: cand.bounds,
                preferred_view_dir: None,
                height_m,
                is_hero: cand.is_overridden || idx == 0,
            });
        }
    }

    landmarks
}

// Compile-time thread-safety contract: planner traits require Send + Sync across worker threads.
const _: fn() = || {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CsgVolumeAdapter>();
    assert_send_sync::<CsgObjectTable>();
    assert_send_sync::<CsgSceneOracle>();
};

#[cfg(test)]
mod tests {
    use csg_core::{CsgNode, Mat3x4, MaterialId, semantic};
    use csg_scene::{Assembly, Instance};
    use flight_navigator::fnav_plan::Mode;

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
    fn volume_adapter_matches_voxel_volume_parity() {
        let mut world = WorldScene::new(32.0);
        // Synthetic canyon + water basin + bridge deck for full R0 coverage.
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
            "road",
            CsgVec3::new(8.0, -0.2, 0.0),
            CsgVec3::new(16.0, 0.1, 24.0),
            semantic::ROAD,
            11,
        );
        add_test_box(
            &mut world,
            "tower_west",
            CsgVec3::new(1.0, 0.0, 4.0),
            CsgVec3::new(7.0, 12.0, 20.0),
            semantic::BUILDING,
            12,
        );
        add_test_box(
            &mut world,
            "tower_east",
            CsgVec3::new(17.0, 0.0, 4.0),
            CsgVec3::new(23.0, 10.0, 20.0),
            semantic::BUILDING,
            13,
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
        let expected_bounds = csg_aabb_to_fnav(raw.bounds());
        let expected_hash = raw.hash();

        let adapter = CsgVolumeAdapter::new(raw);
        let raw_ref = adapter.inner();

        assert_eq!(adapter.bounds(), expected_bounds);
        assert_eq!(adapter.hash(), expected_hash);

        let probe_xz = [
            (0.0, 0.0),
            (4.2, 10.5),
            (12.0, 12.0),
            (20.1, 14.7),
            (30.0, 12.0),
            (30.0, 2.0),
            (expected_bounds.max.x, expected_bounds.max.z),
            (-5.0, -5.0),
            (100.0, 100.0),
            (f32::NAN, 5.0),
            (5.0, f32::INFINITY),
        ];

        for (x, z) in probe_xz {
            let raw_spans = raw_ref.column_spans(x, z);
            let ad_spans = adapter.column_spans(x, z);
            assert_eq!(
                ad_spans.len(),
                raw_spans.len(),
                "span count mismatch at ({x}, {z})"
            );
            for (a, r) in ad_spans.iter().zip(raw_spans.iter()) {
                assert_eq!(a.y0, r.y0);
                assert_eq!(a.y1, r.y1);
                assert_eq!(a.sem, voxel_class_to_sem(r.class));
            }
            assert_eq!(adapter.ground_height(x, z), raw_ref.ground_height(x, z));
            assert_eq!(adapter.roof_height(x, z), raw_ref.roof_height(x, z));
        }

        let probe_pts = [
            Vec3::new(4.0, 5.0, 12.0),
            Vec3::new(12.0, 3.5, 12.0),
            Vec3::new(12.0, 0.05, 12.0),
            Vec3::new(30.0, 3.0, 12.0),
            Vec3::new(30.0, 6.5, 12.0),
            Vec3::new(30.0, 2.0, 2.0),
            Vec3::new(-10.0, 5.0, -10.0),
            Vec3::new(f32::NAN, 1.0, 2.0),
        ];

        for p in probe_pts {
            let cp = glam_to_csg(p);
            assert_eq!(adapter.distance(p), raw_ref.distance(cp));
            assert_eq!(adapter.gradient(p), csg_to_glam(raw_ref.gradient(cp)));
            assert_eq!(adapter.covered(p), raw_ref.covered(cp));
            assert_eq!(
                adapter.semantics_at(p),
                voxel_sem_to_fnav(raw_ref.semantics_at(cp))
            );
            assert_eq!(adapter.voxel_size_at(p), raw_ref.voxel_size_at(cp));
            assert_eq!(adapter.is_thin_sharp(p), raw_ref.is_thin_sharp(cp));
        }

        // Verify bridge-covered water point reports covered = true and preserves COVERED_BIT.
        let under_bridge = Vec3::new(30.0, 3.0, 12.0);
        assert!(adapter.covered(under_bridge));
        assert!(adapter.semantics_at(under_bridge).is_covered());
        assert_eq!(adapter.semantics_at(under_bridge).sem(), Sem::Empty);
    }

    #[test]
    fn landmark_name_sanitization_strips_address_patterns() {
        assert_eq!(
            sanitize_landmark_name("Coastal Refinery Towers", 0),
            "Coastal Refinery Towers"
        );
        assert_eq!(
            sanitize_landmark_name("Harbor Promenade", 1),
            "Harbor Promenade"
        );
        assert_eq!(sanitize_landmark_name("   ", 2), "Landmark 3");
        assert_eq!(sanitize_landmark_name("420 Maple Ave", 3), "Landmark 4");
        assert_eq!(sanitize_landmark_name("12-B Cedar Street", 4), "Landmark 5");
        assert_eq!(sanitize_landmark_name("Unit 700 Pine Rd", 5), "Landmark 6");
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
        let (oracle, objects) = CsgSceneOracle::build_with_baker(built, Some(sun_dir), None)
            .expect("CsgSceneOracle::build_with_baker must succeed on gundo");

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
        let prober = CpuRayProber::new(&oracle);
        let mut cache = PlanCache::default();

        let plan_a = fnav_plan::plan(&oracle, &prober, &objects, &mut cache, &req)
            .expect("deterministic 30 s plan on gundo must succeed");
        let plan_b = fnav_plan::plan(&oracle, &prober, &objects, &mut cache, &req)
            .expect("cached deterministic 30 s plan on gundo must succeed");

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
