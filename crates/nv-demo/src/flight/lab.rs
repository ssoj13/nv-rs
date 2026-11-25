//! Flight Lab authoring state: manual keyframes, `camera-guide-rs` dynamics tuning, contact
//! sheet inspection, and honest gating of automatic planning pending `csg-rs` R0+R1.
//!
//! **What.** Holds [`FlightLabState`], [`ManualKeyframe`], [`DynamicsCurvePlot`], [`LabMode`],
//! [`PersistedLabPrefs`], and [`CSG_AUTO_PLAN_BLOCKED_REASON`].
//! **Why.** Separates authoring & dynamics inspection state from Winamp-style playback state
//! ([`crate::flight::player::FlightPlayer`]) while sharing one [`crate::flight::FlightWorkspace`]
//! so previewing a manual track or re-timing the active track updates playback immediately with
//! zero state duplication.
//! **How.** Manual keyframes capture viewport [`camera::CameraPose`] values (storing focal length
//! in mm on a 36x24 mm full-frame back), compile via [`crate::flight::convert::keyframes_to_track`],
//! analyze/re-time tracks via [`crate::flight::convert::analyze_and_retime_track`], and persist
//! verified `*.fnav.json` files atomically via [`flight_navigator::PlanTrackFile`].
//! **Uses.** `camera`, `flight_navigator::{camera_guide, fnav_core, fnav_plan}`, `glam`, `serde`.
//! **Used by.** `crate::flight`, `crate::flight::ui`, `crate::persist`.

use std::path::{Path, PathBuf};

use camera::{CameraPose, FOV_MAX, FOV_MIN};
use flight_navigator::camera_guide::{GuideLimits, LimitTerm, VerifyReport};
use flight_navigator::fnav_core::{
    FilmBack, Lens, SceneRef, TRACK_FILE_EXT, Track, default_flight_limits,
};
use flight_navigator::fnav_plan::{Mode, RailPlan};
use flight_navigator::PlanTrackFile;
use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::flight::convert::{analyze_and_retime_track, keyframes_to_track, safe_unit_quat};
use crate::flight::{sanitize_file_stem, utc_now_iso8601};

/// Explicit reason shown on the disabled `Auto Plan` button until `csg-rs` ships R0 + R1.
pub const CSG_AUTO_PLAN_BLOCKED_REASON: &str = "Blocked: csg-rs 697a203 has not yet shipped R0 (VoxelVolume) + R1 (WorldRayQuery). Manual keyframe authoring, live camera-guide dynamics, curated track re-timing, library & playlists are active.";

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
    /// Automatic cinematic fly-through planner (gated until `csg-rs` ships R0 + R1).
    AutoPlan,
    /// Interactive keyframe authoring and live `camera-guide-rs` dynamics tuning.
    #[default]
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
    /// Human-readable status or error message from the latest Lab action.
    pub status_msg: String,
}

impl Default for FlightLabState {
    fn default() -> Self {
        Self::new()
    }
}

impl FlightLabState {
    /// Creates a clean [`FlightLabState`] with default dynamics limits (`max_bank_deg = 35 deg`).
    #[must_use]
    pub fn new() -> Self {
        Self {
            mode: LabMode::ManualKeyframes,
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
            status_msg: String::new(),
        }
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
