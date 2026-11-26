//! Flight Lab authoring state: live `csg-scene` R0+R1 automatic planning, manual keyframes,
//! `camera-guide-rs` dynamics tuning, and contact sheet inspection.
//!
//! **What.** Holds [`FlightLabState`], [`ManualKeyframe`], [`DynamicsCurvePlot`], [`LabMode`],
//! and [`PersistedLabPrefs`].
//! **Why.** Separates authoring, auto-plan request/result state, and dynamics inspection from
//! Winamp-style playback state ([`crate::flight::player::FlightPlayer`]) while sharing one
//! [`crate::flight::FlightWorkspace`] so previewing an auto-planned or manual track or re-timing
//! the active track updates playback immediately with zero state duplication.
//! **How.** Auto-Plan builds sanitized [`flight_navigator::fnav_plan::PlanRequest`] values for
//! [`crate::flight::oracle::CsgAutoPlanner`] (backed by `csg-scene` R0 `VoxelVolume` + R1
//! `WorldRayQuery`) and installs validated [`flight_navigator::fnav_plan::RailPlan`] results into
//! preview state (naming the preview from `plan.request.seed` and surfacing any
//! `plan.report.gate_failures` in `status_msg`); manual keyframes capture viewport
//! [`camera::CameraPose`] values (storing focal length in mm on a 36x24 mm full-frame back),
//! compile via [`crate::flight::convert::keyframes_to_track`], analyze/re-time tracks via
//! [`crate::flight::convert::analyze_and_retime_track`], persist user lab preferences
//! ([`PersistedLabPrefs`], including `plan_mode`) in `nv-demo.json`, and persist verified
//! `*.fnav.json` files atomically via [`flight_navigator::PlanTrackFile`].
//! **Uses.** `camera`, `flight_navigator::{camera_guide, fnav_core, fnav_plan}`, `glam`, `serde`.
//! **Used by.** `crate::flight`, `crate::flight::ui`, `crate::persist`.

use std::path::{Path, PathBuf};

use camera::{CameraPose, FOV_MAX, FOV_MIN};
use flight_navigator::camera_guide::{GuideLimits, LimitTerm, VerifyReport};
use flight_navigator::fnav_core::{
    FilmBack, Lens, SceneRef, TRACK_FILE_EXT, Track, default_flight_limits,
};
use flight_navigator::fnav_plan::{Mode, PlanRequest, RailPlan};
use flight_navigator::PlanTrackFile;
use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::flight::convert::{analyze_and_retime_track, keyframes_to_track, safe_unit_quat};
use crate::flight::oracle::AutoPlanStage;
use crate::flight::{sanitize_file_stem, utc_now_iso8601};

/// Minimum allowed Flight Lab tour duration (seconds).
pub const LAB_DURATION_MIN_S: f32 = 15.0;
/// Maximum allowed Flight Lab tour duration (seconds).
pub const LAB_DURATION_MAX_S: f32 = 120.0;
/// Default Flight Lab tour duration (seconds).
pub const LAB_DURATION_DEFAULT_S: f32 = 45.0;
/// Default Flight Lab max bank limit (degrees), widened for expressive drone flights.
pub const LAB_DEFAULT_MAX_BANK_DEG: f32 = 35.0;

/// Active authoring tab inside the Flight Lab dock panel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LabMode {
    /// Live automatic cinematic fly-through planner backed by `csg-scene` R0 `VoxelVolume` + R1
    /// `WorldRayQuery`.
    #[default]
    AutoPlan,
    /// Interactive keyframe authoring and live `camera-guide-rs` dynamics tuning.
    ManualKeyframes,
}

/// One user-authored manual camera keyframe in Flight Lab.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualKeyframe {
    /// World-space camera eye position `[x, y, z]` (metres).
    pub pos: [f32; 3],
    /// Unit camera-to-world orientation quaternion `[x, y, z, w]`.
    pub orientation: [f32; 4],
    /// Equivalent 35 mm full-frame focal length (mm).
    pub focal_mm: f32,
}

impl ManualKeyframe {
    /// Captures a [`ManualKeyframe`] from the current viewport [`CameraPose`].
    #[must_use]
    pub fn from_pose(pose: CameraPose) -> Self {
        let q = safe_unit_quat(pose.orientation);
        let lens = Lens::from_fov_y(pose.fov_y.clamp(FOV_MIN, FOV_MAX), FilmBack::default());
        Self {
            pos: if pose.eye.is_finite() {
                pose.eye.to_array()
            } else {
                [0.0; 3]
            },
            orientation: q.to_array(),
            focal_mm: lens.focal_mm,
        }
    }

    /// Converts this keyframe back into a validated [`CameraPose`].
    #[must_use]
    pub fn to_pose(self, znear: f32, zfar: f32) -> CameraPose {
        let eye = Vec3::from_array(self.pos);
        let orientation = safe_unit_quat(Quat::from_array(self.orientation));
        let fov_y = Lens::pinhole(self.focal_mm).fov_y().clamp(FOV_MIN, FOV_MAX);
        let zn = znear.max(1.0e-3);
        CameraPose {
            eye: if eye.is_finite() { eye } else { Vec3::ZERO },
            orientation,
            fov_y,
            znear: zn,
            zfar: zfar.max(zn + 1.0),
        }
    }
}

/// Sampled `camera-guide-rs` speed, cap, bank, and binding-limit curves for the Flight Lab graph.
#[derive(Clone, Debug, PartialEq)]
pub struct DynamicsCurvePlot {
    /// Arc-length sample positions along the path (metres).
    pub s: Vec<f32>,
    /// Static/bank-tightened speed cap curve `cap(s)` (m/s).
    pub cap: Vec<f32>,
    /// Achieved jerk-limited speed curve `v(s)` (m/s).
    pub v: Vec<f32>,
    /// Coordinated bank angle curve `bank(s)` (degrees).
    pub bank_deg: Vec<f32>,
    /// Binding limit term per sample.
    pub binding: Vec<LimitTerm>,
    /// Quantitative limit verification ratios (`worst() <= 1 + RATIO_TOL`).
    pub report: VerifyReport,
}

/// Default `GuideLimits` tuned for Flight Lab (`default_flight_limits()` with `max_bank_deg = 35`).
#[must_use]
pub fn default_lab_limits() -> GuideLimits {
    let mut lim = default_flight_limits();
    lim.max_bank_deg = LAB_DEFAULT_MAX_BANK_DEG;
    lim
}

/// Serializable Flight Lab user preferences persisted in `nv-demo.json`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PersistedLabPrefs {
    /// Requested tour duration (seconds, `15..=120`).
    pub duration_s: f32,
    /// Deterministic planner RNG seed.
    pub seed: u64,
    /// Planner mode (`Deterministic` or `Creative`).
    pub plan_mode: Mode,
    /// Global cut-density multiplier (`0.0..=2.0`).
    pub cut_density: f32,
    /// Active `camera-guide-rs` kinematic and bank limits.
    pub limits: GuideLimits,
    /// Saved manual keyframes authored in Flight Lab.
    pub manual_keyframes: Vec<ManualKeyframe>,
    /// Whether manual keyframe tracks close into a periodic loop.
    pub manual_looping: bool,
}

impl Default for PersistedLabPrefs {
    fn default() -> Self {
        Self {
            duration_s: LAB_DURATION_DEFAULT_S,
            seed: 42,
            plan_mode: Mode::Deterministic,
            cut_density: 1.0,
            limits: default_lab_limits(),
            manual_keyframes: Vec::new(),
            manual_looping: true,
        }
    }
}

/// Mutable state of the Flight Lab authoring & dynamics panel.
#[derive(Clone, Debug)]
pub struct FlightLabState {
    /// Currently selected Flight Lab authoring tab.
    pub mode: LabMode,
    /// Requested tour duration in seconds (`15.0..=120.0`, default `45.0`).
    pub duration_s: f32,
    /// Deterministic seed for automatic planning.
    pub seed: u64,
    /// Planner mode (`Deterministic` or `Creative`).
    pub plan_mode: Mode,
    /// Cut-density multiplier (`0.0..=2.0`).
    pub cut_density: f32,
    /// Live `camera-guide-rs` kinematic & coordinated-bank limits.
    pub limits: GuideLimits,
    /// User-authored manual keyframes.
    pub manual_keyframes: Vec<ManualKeyframe>,
    /// Whether manually built tracks loop seamlessly.
    pub manual_looping: bool,
    /// Operator-chosen name used when saving a preview track to disk.
    pub track_name: String,
    /// Most recently built or re-timed preview [`Track`].
    pub preview_track: Option<Track>,
    /// Optional full [`RailPlan`] contact sheet when available.
    pub preview_plan: Option<RailPlan>,
    /// Most recent `camera-guide-rs` speed/cap/bank curve plot.
    pub dynamics_plot: Option<DynamicsCurvePlot>,
    /// Current stage of any background automatic flight planning job (`Idle` when not planning).
    pub auto_plan_stage: AutoPlanStage,
    /// Human-readable status or error message from the latest Lab action.
    pub status_msg: String,
}

impl Default for FlightLabState {
    fn default() -> Self {
        Self::new()
    }
}

impl FlightLabState {
    /// Creates a clean [`FlightLabState`] defaulting to live [`LabMode::AutoPlan`] with default
    /// dynamics limits (`max_bank_deg = 35 deg`).
    #[must_use]
    pub fn new() -> Self {
        Self {
            mode: LabMode::AutoPlan,
            duration_s: LAB_DURATION_DEFAULT_S,
            seed: 42,
            plan_mode: Mode::Deterministic,
            cut_density: 1.0,
            limits: default_lab_limits(),
            manual_keyframes: Vec::new(),
            manual_looping: true,
            track_name: "manual_flight".to_string(),
            preview_track: None,
            preview_plan: None,
            dynamics_plot: None,
            auto_plan_stage: AutoPlanStage::Idle,
            status_msg: String::new(),
        }
    }

    /// Builds a sanitized [`PlanRequest`] from the current Flight Lab controls for submission
    /// to [`crate::flight::oracle::CsgAutoPlanner`].
    ///
    /// Why: clamping `duration_s`, `cut_density`, and `limits` here guarantees UI drag values or
    /// restored preferences cannot submit NaN or out-of-range parameters to `fnav-plan`.
    #[must_use]
    pub fn build_plan_request(&self) -> PlanRequest {
        let duration_s = if self.duration_s.is_finite() {
            self.duration_s.clamp(LAB_DURATION_MIN_S, LAB_DURATION_MAX_S)
        } else {
            LAB_DURATION_DEFAULT_S
        };
        let cut_density = if self.cut_density.is_finite() {
            self.cut_density.clamp(0.0, 2.0)
        } else {
            1.0
        };
        PlanRequest {
            duration_s,
            seed: self.seed,
            mode: self.plan_mode,
            cut_density,
            limits: sanitize_limits(self.limits),
            edits: Vec::new(),
        }
    }

    /// Installs a completed automatic [`RailPlan`] into Flight Lab preview state, derives
    /// `track_name` from `plan.request.seed`, surfaces any `plan.report.gate_failures` in
    /// `status_msg`, and refreshes [`Self::dynamics_plot`] without mutating the validated track.
    ///
    /// Why: `fnav-plan` already verified clearance, handovers, and section 7.4 metric gates on
    /// `plan.track`, so preserving `plan.track` verbatim as [`Self::preview_track`] keeps all
    /// validated flight invariants intact while still driving the dynamics plot.
    pub fn apply_auto_plan(&mut self, scene_id: String, plan: RailPlan, elapsed_ms: f32) {
        let shot_count = plan.shots.len();
        let dur_s = plan.track.duration_s();
        let plan_seed = plan.request.seed;
        let gate_warn = if !plan.report.all_gates_passed {
            plan.report.gate_failures.first().map(|first| {
                if plan.report.gate_failures.len() > 1 {
                    format!(
                        " [WARN {} gates: {}; ...]",
                        plan.report.gate_failures.len(),
                        first
                    )
                } else {
                    format!(" [WARN: {first}]")
                }
            })
        } else {
            None
        };
        self.dynamics_plot = analyze_and_retime_track(&plan.track, &self.limits)
            .ok()
            .map(|(_, plot)| plot);
        self.preview_track = Some(plan.track.clone());
        self.preview_plan = Some(plan);
        let stem = sanitize_file_stem(&scene_id);
        self.track_name = format!("{stem}_auto_{plan_seed}");
        self.auto_plan_stage = AutoPlanStage::Idle;
        self.status_msg = format!(
            "Planned {} shots ({:.1} s) in {:.0} ms{}",
            shot_count,
            dur_s,
            elapsed_ms.max(0.0),
            gate_warn.as_deref().unwrap_or("")
        );
    }

    /// Appends the current viewport camera pose as a new manual keyframe.
    pub fn push_pose(&mut self, pose: CameraPose) {
        self.manual_keyframes.push(ManualKeyframe::from_pose(pose));
        self.status_msg = format!("Captured keyframe #{}", self.manual_keyframes.len());
    }

    /// Removes the last authored manual keyframe.
    pub fn pop_keyframe(&mut self) {
        if self.manual_keyframes.pop().is_some() {
            self.status_msg = format!(
                "Removed last keyframe ({} remaining)",
                self.manual_keyframes.len()
            );
        }
    }

    /// Clears all authored manual keyframes.
    pub fn clear_keyframes(&mut self) {
        self.manual_keyframes.clear();
        self.status_msg = "Cleared manual keyframes".to_string();
    }

    /// Compiles [`Self::manual_keyframes`] into [`Self::preview_track`] and updates
    /// [`Self::dynamics_plot`] via `camera-guide-rs`.
    ///
    /// # Errors
    /// Returns an error description if fewer than 2 keyframes exist or `camera-guide-rs` rejects
    /// the current limits.
    pub fn build_manual_track(&mut self) -> Result<&Track, String> {
        let dur = f64::from(self.duration_s.clamp(2.0, LAB_DURATION_MAX_S));
        let track = keyframes_to_track(
            &self.manual_keyframes,
            dur,
            self.manual_looping,
            &self.limits,
        )?;
        match analyze_and_retime_track(&track, &self.limits) {
            Ok((retimed, plot)) => {
                let final_dur = retimed.duration_s();
                let worst = plot.report.worst();
                self.preview_track = Some(retimed);
                self.preview_plan = None;
                self.dynamics_plot = Some(plot);
                self.status_msg = format!(
                    "Built track ({:.1} s, worst ratio {:.2})",
                    final_dur, worst
                );
            }
            Err(_) => {
                let final_dur = track.duration_s();
                self.preview_track = Some(track);
                self.preview_plan = None;
                self.dynamics_plot = None;
                self.status_msg = format!(
                    "Built track ({:.1} s; path < 2 m, dynamics skipped)",
                    final_dur
                );
            }
        }
        self.preview_track
            .as_ref()
            .ok_or_else(|| "preview track unavailable after build".to_string())
    }

    /// Re-times an arbitrary `track` under [`Self::limits`], stores the updated curve plot and
    /// preview track, and returns the re-timed track.
    ///
    /// # Errors
    /// Returns an error description if `track` is empty, too short, or limits are contradictory.
    pub fn retime_track(&mut self, track: &Track) -> Result<Track, String> {
        let (retimed, plot) = analyze_and_retime_track(track, &self.limits)?;
        let dur = retimed.duration_s();
        let worst = plot.report.worst();
        self.preview_track = Some(retimed.clone());
        self.dynamics_plot = Some(plot);
        self.status_msg = format!(
            "Applied dynamics ({:.1} s, worst ratio {:.2})",
            dur, worst
        );
        Ok(retimed)
    }

    /// Atomically saves [`Self::preview_track`] as a content-addressed `*.fnav.json` file in `dir`.
    ///
    /// # Errors
    /// Returns an error description if no preview track exists or atomic disk write fails.
    pub fn save_track_to_dir(
        &mut self,
        dir: &Path,
        name: &str,
        scene_id: &str,
        scene_hash: &str,
    ) -> Result<PathBuf, String> {
        let track = self
            .preview_track
            .clone()
            .ok_or_else(|| "no preview track to save; build or re-time a track first".to_string())?;
        let display_name = if name.trim().is_empty() {
            "manual_flight".to_string()
        } else {
            name.trim().to_string()
        };
        let stem = sanitize_file_stem(&display_name);
        let path = dir.join(format!("{stem}.{TRACK_FILE_EXT}"));
        let scene = SceneRef {
            id: scene_id.to_string(),
            content_hash: scene_hash.to_string(),
        };
        let file = PlanTrackFile::new(
            display_name,
            utc_now_iso8601(),
            scene,
            None,
            self.preview_plan.clone(),
            track,
        );
        file.save_atomic(&path).map_err(|e| e.to_string())?;
        self.status_msg = format!("Saved {}", path.display());
        Ok(path)
    }

    /// Captures the persisted subset of Flight Lab state for `nv-demo.json`.
    #[must_use]
    pub fn to_persisted(&self) -> PersistedLabPrefs {
        PersistedLabPrefs {
            duration_s: self.duration_s,
            seed: self.seed,
            plan_mode: self.plan_mode,
            cut_density: self.cut_density,
            limits: self.limits,
            manual_keyframes: self.manual_keyframes.clone(),
            manual_looping: self.manual_looping,
        }
    }

    /// Restores persisted preferences into this Flight Lab state (clamping ranges safely).
    pub fn apply_persisted(&mut self, prefs: &PersistedLabPrefs) {
        self.duration_s = if prefs.duration_s.is_finite() {
            prefs
                .duration_s
                .clamp(LAB_DURATION_MIN_S, LAB_DURATION_MAX_S)
        } else {
            LAB_DURATION_DEFAULT_S
        };
        self.seed = prefs.seed;
        self.plan_mode = prefs.plan_mode;
        self.cut_density = if prefs.cut_density.is_finite() {
            prefs.cut_density.clamp(0.0, 2.0)
        } else {
            1.0
        };
        self.limits = sanitize_limits(prefs.limits);
        self.manual_keyframes = prefs.manual_keyframes.clone();
        self.manual_looping = prefs.manual_looping;
    }
}

/// Clamps persisted `GuideLimits` into valid positive ranges so corrupt JSON cannot break planning.
fn sanitize_limits(mut l: GuideLimits) -> GuideLimits {
    let def = default_lab_limits();
    let fix = |v: f32, lo: f32, hi: f32, fb: f32| {
        if v.is_finite() { v.clamp(lo, hi) } else { fb }
    };
    l.lateral_accel = fix(l.lateral_accel, 0.5, 40.0, def.lateral_accel);
    l.yaw_rate_deg_s = fix(l.yaw_rate_deg_s, 5.0, 360.0, def.yaw_rate_deg_s);
    l.bank_k = fix(l.bank_k, 0.0, 1.0, def.bank_k);
    l.max_bank_deg = fix(l.max_bank_deg, 0.0, 60.0, def.max_bank_deg);
    l.bank_rate_deg_s = fix(l.bank_rate_deg_s, 5.0, 360.0, def.bank_rate_deg_s);
    l.bank_accel_deg_s2 = fix(l.bank_accel_deg_s2, 10.0, 1000.0, def.bank_accel_deg_s2);
    l.long_accel = fix(l.long_accel, 0.5, 30.0, def.long_accel);
    l.long_decel = fix(l.long_decel, 0.5, 30.0, def.long_decel);
    l.jerk = fix(l.jerk, 0.5, 50.0, def.jerk);
    l.v_min = fix(l.v_min, 0.1, 20.0, def.v_min);
    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lab_state_defaults_to_live_auto_plan_and_idle_stage() {
        let lab = FlightLabState::new();
        assert_eq!(lab.mode, LabMode::AutoPlan);
        assert_eq!(lab.auto_plan_stage, AutoPlanStage::Idle);
        assert_eq!(lab.duration_s, LAB_DURATION_DEFAULT_S);
        assert_eq!(lab.seed, 42);
    }

    #[test]
    fn build_plan_request_clamps_ranges_and_sanitizes_nans() {
        let mut lab = FlightLabState::new();
        lab.duration_s = 500.0;
        lab.seed = 9001;
        lab.plan_mode = Mode::Creative;
        lab.cut_density = -3.0;
        lab.limits.max_bank_deg = 120.0;
        lab.limits.jerk = f32::NAN;

        let req = lab.build_plan_request();
        assert_eq!(req.duration_s, LAB_DURATION_MAX_S);
        assert_eq!(req.seed, 9001);
        assert_eq!(req.mode, Mode::Creative);
        assert_eq!(req.cut_density, 0.0);
        assert_eq!(req.limits.max_bank_deg, 60.0);
        assert!(req.limits.jerk.is_finite());
        assert!(req.edits.is_empty());

        lab.duration_s = f32::NAN;
        lab.cut_density = f32::INFINITY;
        let req2 = lab.build_plan_request();
        assert_eq!(req2.duration_s, LAB_DURATION_DEFAULT_S);
        assert_eq!(req2.cut_density, 1.0);
    }

    #[test]
    fn persisted_lab_prefs_roundtrip_plan_mode_and_default_missing() {
        let mut lab = FlightLabState::new();
        lab.plan_mode = Mode::Creative;
        lab.seed = 10;
        lab.cut_density = 1.4;

        let prefs = lab.to_persisted();
        assert_eq!(prefs.plan_mode, Mode::Creative);

        let json = serde_json::to_string(&prefs).expect("serialize PersistedLabPrefs");
        let parsed: PersistedLabPrefs =
            serde_json::from_str(&json).expect("deserialize PersistedLabPrefs");
        assert_eq!(parsed.plan_mode, Mode::Creative);

        let mut restored = FlightLabState::new();
        restored.apply_persisted(&parsed);
        assert_eq!(restored.plan_mode, Mode::Creative);
        assert_eq!(restored.seed, 10);

        // Legacy JSON missing `plan_mode` must default cleanly to Mode::Deterministic.
        let legacy: PersistedLabPrefs =
            serde_json::from_str(r#"{"duration_s":30.0,"seed":7}"#).expect("legacy json");
        assert_eq!(legacy.plan_mode, Mode::Deterministic);
        assert_eq!(legacy.seed, 7);
    }
}
