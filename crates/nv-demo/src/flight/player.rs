//! Winamp-style flight playback, playlist queue, library indexing, and C2 transition state
//! machine powered by `flight-navigator-rs`.
//!
//! **What.** Implements [`FlightPlayer`], [`PlaylistSlot`], [`PlayerState`], [`PlayerEval`],
//! [`TrackOrigin`], [`CuratedTrackEntry`], and [`PersistedPlayerPrefs`].
//! **Why.** Replaces the legacy single-cassette `CameraRail` playback path with one unified
//! [`fnav_core::Track`] player supporting scene-curated rails, repo/user `*.fnav.json` libraries,
//! `*.fnav-playlist.json` queues, deterministic SplitMix64 shuffle, repeat modes, `Cut`/`FadeBlack`/
//! `FlyThrough` transitions, exact interval `cut_before` detection, and headless `--bench-*` hooks.
//! **How.** Steady-state frame stepping (`tick`) evaluates the active immutable [`fnav_core::Track`]
//! via `eval_step(prev_t, t)` without allocating or touching the planner; same-scene `FlyThrough`
//! builds one C2 [`fnav_core::Transition`] between exit and entry kinematics; cross-scene switches
//! automatically force [`PlaylistTransition::FadeBlack`] and emit `request_scene_id` at the black
//! midpoint until [`FlightPlayer::notify_scene_loaded`] resumes the fade-in.
//! **Uses.** `camera`, `csg_scene`, `flight_navigator::{fnav_core, fnav_plan}`, `crate::flight::convert`.
//! **Used by.** `crate::flight`, `crate::flight::ui`, `crate::main`, `crate::persist`, `crate::bench`.

use std::path::{Path, PathBuf};

use camera::CameraPose;
use csg_scene::{
    BodyPose, BodyPoseSource, DroneTilt, LandmarkHint, LookMode, RailRig, RailSpec, RigInput,
    TrackResolver, TrackStatus, TrackTarget,
};
use flight_navigator::fnav_core::{
    Aim, CameraState, FlightLibrary, LibrarySource, PLAYLIST_FILE_EXT, PlaylistEntry, PlaylistFile,
    PlaylistTransition, RepeatMode, SceneRef, TRACK_FILE_EXT, Track, TrackRef, Transition,
};
use flight_navigator::fnav_plan::{RailPlan, SplitMix64};
use flight_navigator::{PlanTrackFile, load_plan_track};
use fnav_csg::{PlanBodyPoseSource, TrackBodyPoseSource};
use serde::{Deserialize, Serialize};

use crate::flight::convert::curated_rail_to_track;
use crate::flight::{
    default_repo_flights_dir, default_user_flights_dir, sanitize_file_stem, utc_now_iso8601,
};

/// Default fade-to-black duration (seconds) forced on scene changes when an entry requested Cut
/// or FlyThrough.
const DEFAULT_SCENE_FADE_S: f32 = 0.75;
/// Salt mixed into [`SplitMix64`] for deterministic playlist shuffle permutations.
const SHUFFLE_RNG_SALT: u64 = 0x540F_F1E5_9A31_C2B7;

/// Provenance badge showing where a [`PlaylistSlot`] originated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrackOrigin {
    /// Converted from an active scene module's curated [`csg_scene::RailSpec`].
    Curated,
    /// Loaded from the repository demo library (`assets/flights`).
    RepoLibrary,
    /// Loaded from the user-local library (`~/.nv-rs/flights`).
    UserLibrary,
    /// Built or re-timed live inside Flight Lab.
    LabPreview,
}

impl TrackOrigin {
    /// Short display badge for the Player UI.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Curated => "Curated",
            Self::RepoLibrary => "Repo",
            Self::UserLibrary => "User",
            Self::LabPreview => "Lab",
        }
    }
}

/// Converted curated scene rail cached for the active scene module.
#[derive(Clone, Debug)]
pub struct CuratedTrackEntry {
    /// Stable curated rail identifier (`RailSpec::id`).
    pub id: String,
    /// Human-readable UI label (`RailSpec::label`).
    pub label: String,
    /// Concise tooltip description (`RailSpec::description`).
    pub description: String,
    /// Scene identifier that owns this curated rail.
    pub scene_id: String,
    /// Converted immutable [`Track`].
    pub track: Track,
    /// Cached track duration in seconds (`f64`).
    pub duration_s: f64,
}

/// One queued entry in [`FlightPlayer::queue`].
#[derive(Clone, Debug)]
pub struct PlaylistSlot {
    /// Operator-facing display title.
    pub title: String,
    /// Optional tooltip description.
    pub description: String,
    /// Scene identifier this track targets (empty if scene-agnostic).
    pub scene_id: String,
    /// Content-addressed BLAKE3 hex id (or curated rail id).
    pub track_id: String,
    /// Source `*.fnav.json` path on disk, if loaded from or saved to a library.
    pub file_path: Option<PathBuf>,
    /// Immutable evaluated camera track.
    pub track: Track,
    /// Optional planner contact-sheet metadata loaded from `PlanTrackFile`.
    pub plan: Option<RailPlan>,
    /// Requested transition when entering this slot.
    pub transition_in: PlaylistTransition,
    /// Hold duration (seconds) on the final frame before advancing.
    pub hold_s: f32,
    /// True if the slot's recorded scene hash disagrees with the active scene hash.
    pub stale: bool,
    /// Origin badge (`Curated`, `RepoLibrary`, `UserLibrary`, `LabPreview`).
    pub origin: TrackOrigin,
}

impl PlaylistSlot {
    /// Constructs a [`PlaylistSlot`] from a converted [`CuratedTrackEntry`].
    #[must_use]
    fn from_curated(c: &CuratedTrackEntry) -> Self {
        Self {
            title: c.label.clone(),
            description: c.description.clone(),
            scene_id: c.scene_id.clone(),
            track_id: c.id.clone(),
            file_path: None,
            track: c.track.clone(),
            plan: None,
            transition_in: PlaylistTransition::Cut,
            hold_s: 0.0,
            stale: false,
            origin: TrackOrigin::Curated,
        }
    }

    /// Constructs a [`PlaylistSlot`] from a verified on-disk [`PlanTrackFile`].
    #[must_use]
    fn from_track_file(
        tf: PlanTrackFile,
        path: PathBuf,
        origin: TrackOrigin,
        active_scene_id: &str,
        active_scene_hash: &str,
    ) -> Self {
        let stale = !active_scene_id.is_empty()
            && !active_scene_hash.is_empty()
            && tf.scene.id == active_scene_id
            && tf.scene.content_hash != active_scene_hash;
        Self {
            title: tf.name,
            description: format!("Scene: {} ({})", tf.scene.id, path.display()),
            scene_id: tf.scene.id,
            track_id: tf.id,
            file_path: Some(path),
            track: tf.track,
            plan: tf.plan,
            transition_in: PlaylistTransition::Cut,
            hold_s: 0.0,
            stale,
            origin,
        }
    }
}

/// Playback finite-state machine state of [`FlightPlayer`].
#[derive(Clone, Debug, Default, PartialEq)]
pub enum PlayerState {
    /// Stopped; manual camera control is active.
    #[default]
    Idle,
    /// Holding on black while the host loads `scene_id` for the upcoming playlist entry.
    Loading {
        /// Target scene identifier requested from the host.
        scene_id: String,
    },
    /// Actively evaluating `queue[entry_idx]` at `time_s`.
    Playing {
        /// Active queue slot index.
        entry_idx: usize,
    },
    /// Running an inter-entry transition (`FadeBlack` or same-scene C2 `FlyThrough`).
    Transition {
        /// Departing queue slot index.
        from_idx: usize,
        /// Destination queue slot index.
        to_idx: usize,
        /// Active transition mode.
        kind: PlaylistTransition,
        /// Elapsed transition time in seconds (`0..=duration_s`).
        t: f64,
        /// Total transition duration in seconds.
        duration_s: f64,
    },
    /// Paused on `queue[entry_idx]` at `time_s` (scrubbable without advancing).
    Paused {
        /// Paused queue slot index.
        entry_idx: usize,
    },
}

/// Internal bundle describing an active transition step.
#[derive(Clone, Copy)]
struct TransitionStep {
    from_idx: usize,
    to_idx: usize,
    kind: PlaylistTransition,
    t: f64,
    duration_s: f64,
}

/// Single-frame evaluation output returned by [`FlightPlayer::tick`].
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerEval {
    /// Evaluated camera pose for this frame.
    pub pose: CameraPose,
    /// True when a hard cut or scene swap occurred before this frame (reset path-trace & motion history).
    pub cut_before: bool,
    /// Screen fade-to-black opacity in `0.0..=1.0` (`0.0` = clear, `1.0` = full black).
    pub fade_alpha: f32,
    /// When `Some(scene_id)`, the host must switch to `scene_id` and call [`FlightPlayer::notify_scene_loaded`].
    pub request_scene_id: Option<String>,
    /// Instantaneous camera speed (m/s) for HUD / telemetry.
    pub speed_mps: f32,
    /// Instantaneous coordinated bank angle (rad) for HUD / telemetry.
    pub bank_rad: f32,
    /// True when the evaluated timestamp lies inside an active chase window.
    pub active_chase: bool,
    /// True when a `MoveKind::Track` subject binding fell back to the stored world trajectory.
    pub subject_fallback: bool,
}

/// Camera evaluation mode for [`FlightPlayer`].
///
/// Why: keeps direct `Track::eval_step` framing (`Cinematic`, default for exact `--bench-*` and
/// authored shot parity) alongside live `csg_scene::RailRig` drone-body / gimbal modes
/// (`AlongPath`, `Track`, `Free`) fed by [`fnav_csg::TrackBodyPoseSource`] /
/// [`fnav_csg::PlanBodyPoseSource`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlightRigCameraMode {
    /// Direct `fnav-core::Track` cinematic framing (preserves exact shot aims & `--bench-*`).
    #[default]
    Cinematic,
    /// `csg_scene::RailRig` with [`LookMode::AlongPath`] (acceleration tilt + forward gimbal).
    AlongPath,
    /// `csg_scene::RailRig` with [`LookMode::Track`] locked onto a scene landmark.
    Track,
    /// `csg_scene::RailRig` with [`LookMode::Free`] (viewport drag steers gimbal without pausing).
    Free,
}

impl FlightRigCameraMode {
    /// Short UI label for the Flight Player Drone Rig bar.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Cinematic => "Cinematic",
            Self::AlongPath => "Along",
            Self::Track => "Track",
            Self::Free => "Free",
        }
    }

    /// Returns `true` when `csg_scene::RailRig` owns the camera gimbal.
    #[must_use]
    pub const fn uses_rail_rig(self) -> bool {
        !matches!(self, Self::Cinematic)
    }
}

/// Live telemetry snapshot captured from `csg_scene::RailRig::step` for the Flight Player UI.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RigTelemetry {
    /// True when the latest evaluated frame came from `csg_scene::RailRig`.
    pub active: bool,
    /// Total drone body tilt angle off world `+Y` (degrees).
    pub tilt_deg: f32,
    /// Gimbal `[yaw, pitch, roll]` angles in degrees.
    pub gimbal_deg: [f32; 3],
    /// Tracking status from `RailRig::step` (`Locked`, `OutOfRange`, `TargetLost`).
    pub track_status: TrackStatus,
    /// Angular tracking error in degrees (`0.0` outside `LookMode::Track`).
    pub track_error_deg: f32,
    /// Drone rotor phase in radians (`[0, TAU)`).
    pub rotor_phase: f32,
}

/// [`csg_scene::BodyPoseSource`] unifying [`TrackBodyPoseSource`] and [`PlanBodyPoseSource`].
///
/// Why: lets [`FlightPlayer`] feed `csg_scene::RailRig` directly from `fnav-csg`'s
/// `PlanBodyPoseSource` when a slot carries a full [`RailPlan`] (contact sheet + report) and from
/// `TrackBodyPoseSource<Track>` for curated/manual tracks, with zero per-frame allocations.
#[derive(Clone, Debug)]
pub enum FlightBodySource {
    /// Owned [`Track`] wrapped in [`TrackBodyPoseSource`].
    Track(TrackBodyPoseSource<Track>),
    /// Owned [`RailPlan`] wrapped in [`PlanBodyPoseSource`] (boxed to keep `FlightBodySource` compact).
    Plan(Box<PlanBodyPoseSource>),
}

impl FlightBodySource {
    /// Constructs the best [`FlightBodySource`] for a [`PlaylistSlot`].
    #[must_use]
    pub fn from_slot(slot: &PlaylistSlot) -> Self {
        match slot.plan.clone() {
            Some(plan) => Self::Plan(Box::new(PlanBodyPoseSource::new(plan))),
            None => Self::Track(TrackBodyPoseSource::new(slot.track.clone())),
        }
    }
}

impl BodyPoseSource for FlightBodySource {
    #[inline]
    fn pose_at(&self, t: f32) -> BodyPose {
        match self {
            Self::Track(src) => src.pose_at(t),
            Self::Plan(src) => src.pose_at(t),
        }
    }

    #[inline]
    fn duration_s(&self) -> Option<f32> {
        match self {
            Self::Track(src) => src.duration_s(),
            Self::Plan(src) => src.duration_s(),
        }
    }
}

/// Zero-allocation [`TrackResolver`] over the loaded scene's [`LandmarkHint`] slice.
struct LandmarkTargets<'a>(&'a [LandmarkHint]);

impl TrackResolver for LandmarkTargets<'_> {
    fn resolve(&self, target: &TrackTarget) -> Option<csg_core::Vec3> {
        match target {
            TrackTarget::Point(p) => Some(*p),
            TrackTarget::Landmark(name) => self
                .0
                .iter()
                .find(|l| l.name == *name)
                .map(|l| l.center),
            TrackTarget::Instance(_) => None,
        }
    }
}

/// Converts a `csg_scene::CameraPose` (from [`RailRig::step`]) into a `camera::CameraPose`.
#[inline]
fn csg_cam_to_pose(cam: csg_scene::CameraPose, znear: f32, zfar: f32) -> CameraPose {
    let eye = glam::Vec3::new(cam.eye.x, cam.eye.y, cam.eye.z);
    let fwd = glam::Vec3::new(cam.forward.x, cam.forward.y, cam.forward.z);
    let up = glam::Vec3::new(cam.up.x, cam.up.y, cam.up.z);
    let fov_y = cam
        .vfov_deg
        .to_radians()
        .clamp(camera::FOV_MIN, camera::FOV_MAX);
    CameraPose::from_eye_forward_up(eye, fwd, up, fov_y, znear, zfar)
}

/// Serializable Flight Player user preferences persisted in `nv-demo.json`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PersistedPlayerPrefs {
    /// Playlist repeat mode (`Off`, `All`, `One`).
    pub repeat: RepeatMode,
    /// Deterministic shuffle toggle.
    pub shuffle: bool,
    /// SplitMix64 seed controlling shuffle order.
    pub shuffle_seed: u64,
    /// Preferred track identifier (curated rail id or BLAKE3 hex id) restored on scene load.
    pub selected_track_id: String,
}

impl Default for PersistedPlayerPrefs {
    fn default() -> Self {
        Self {
            repeat: RepeatMode::All,
            shuffle: false,
            shuffle_seed: 1,
            selected_track_id: String::new(),
        }
    }
}

/// Unified flight playback, playlist queue, and library manager.
#[derive(Clone, Debug)]
pub struct FlightPlayer {
    /// Current playback state machine variant.
    pub state: PlayerState,
    /// Ordered playlist queue slots.
    pub queue: Vec<PlaylistSlot>,
    /// Currently selected slot index inside [`Self::queue`].
    pub active_idx: usize,
    /// Current playback cursor inside the active slot's track (seconds, `f64`).
    pub time_s: f64,
    /// Accumulated hold time on the final frame of the current slot (seconds).
    pub hold_elapsed_s: f64,
    /// Repeat policy (`Off`, `All`, `One`).
    pub repeat: RepeatMode,
    /// Deterministic shuffle toggle.
    pub shuffle: bool,
    /// Seed for SplitMix64 shuffle permutation.
    pub shuffle_seed: u64,
    /// Editable name used when saving the current queue to a `*.fnav-playlist.json` file.
    pub playlist_name: String,
    /// Converted curated rails exported by the currently loaded scene module.
    pub curated: Vec<CuratedTrackEntry>,
    /// Directory-derived index of repo and user tracks & playlists.
    pub library: FlightLibrary,
    /// Repository flight library folder (`assets/flights`).
    pub repo_dir: PathBuf,
    /// User-local flight library folder (`~/.nv-rs/flights`).
    pub user_dir: PathBuf,
    /// Active scene identifier synced from the host.
    pub active_scene_id: String,
    /// Active scene geometry hash synced from the host.
    pub active_scene_hash: String,
    /// Persisted preferred track id restored when curated rails arrive.
    pub selected_track_id: String,
    /// True while recording an entire playlist pass to video (stops cleanly at playlist end).
    pub recording_playlist: bool,
    /// Latest live speed telemetry (m/s) for the Now Playing card.
    pub last_speed_mps: f32,
    /// Latest live coordinated bank telemetry (rad) for the Now Playing card.
    pub last_bank_rad: f32,
    /// Latest live chase-window flag for the Now Playing card.
    pub last_active_chase: bool,
    /// Camera mode (`Cinematic` track framing vs `csg_scene::RailRig` `AlongPath`/`Track`/`Free`).
    pub rig_mode: FlightRigCameraMode,
    /// `csg_scene::DroneTilt` filter parameters (`max_tilt_deg`, `tau_s`) applied to [`RailRig`].
    pub rig_tilt: DroneTilt,
    /// Selected landmark name for [`FlightRigCameraMode::Track`].
    pub rig_track_landmark: Option<String>,
    /// Scene landmarks synced from `SceneLoadedMetadata` for zero-allocation [`TrackResolver`].
    pub scene_landmarks: Vec<LandmarkHint>,
    /// Latest live `RailRig` drone-tilt / gimbal / tracking telemetry.
    pub last_rig: RigTelemetry,
    /// Human-readable status or error line surfaced in the Player footer.
    pub status_msg: String,

    prev_t: Option<f64>,
    pending_cut_before: bool,
    fly_transition: Option<Transition>,
    shuffle_order: Vec<usize>,
    pending_after_load: Option<usize>,
    pending_fade_in_s: Option<f64>,
    last_pose: Option<CameraPose>,
    active_rig: Option<(usize, u64, RailRig<FlightBodySource>)>,
    rig_generation: u64,
    pending_rig_input: RigInput,
}

impl Default for FlightPlayer {
    fn default() -> Self {
        Self::new(default_repo_flights_dir(), default_user_flights_dir())
    }
}

impl FlightPlayer {
    /// Creates a new [`FlightPlayer`] targeting `repo_dir` and `user_dir` and performs an initial
    /// library scan.
    #[must_use]
    pub fn new(repo_dir: PathBuf, user_dir: PathBuf) -> Self {
        let mut player = Self {
            state: PlayerState::Idle,
            queue: Vec::new(),
            active_idx: 0,
            time_s: 0.0,
            hold_elapsed_s: 0.0,
            repeat: RepeatMode::All,
            shuffle: false,
            shuffle_seed: 1,
            playlist_name: "demo_playlist".to_string(),
            curated: Vec::new(),
            library: FlightLibrary::default(),
            repo_dir,
            user_dir,
            active_scene_id: String::new(),
            active_scene_hash: String::new(),
            selected_track_id: String::new(),
            recording_playlist: false,
            last_speed_mps: 0.0,
            last_bank_rad: 0.0,
            last_active_chase: false,
            rig_mode: FlightRigCameraMode::Cinematic,
            rig_tilt: DroneTilt::default(),
            rig_track_landmark: None,
            scene_landmarks: Vec::new(),
            last_rig: RigTelemetry::default(),
            status_msg: String::new(),
            prev_t: None,
            pending_cut_before: false,
            fly_transition: None,
            shuffle_order: Vec::new(),
            pending_after_load: None,
            pending_fade_in_s: None,
            last_pose: None,
            active_rig: None,
            rig_generation: 0,
            pending_rig_input: RigInput::default(),
        };
        player.rescan_library();
        player
    }

    /// Rescans `repo_dir` and `user_dir` via [`FlightLibrary::scan`], flagging stale entries
    /// against the active scene hash.
    pub fn rescan_library(&mut self) {
        let sid = (!self.active_scene_id.is_empty()).then_some(self.active_scene_id.as_str());
        let sh = (!self.active_scene_hash.is_empty()).then_some(self.active_scene_hash.as_str());
        self.library = FlightLibrary::scan(Some(&self.repo_dir), Some(&self.user_dir), sid, sh);
        self.refresh_queue_stale_flags();
    }

    /// Installs curated rails exported by the active scene module, converting each once into an
    /// immutable [`Track`].
    pub fn set_curated_rails(&mut self, scene_id: &str, scene_hash: &str, rails: &[RailSpec]) {
        self.active_scene_id = scene_id.to_string();
        self.active_scene_hash = scene_hash.to_string();

        self.curated.clear();
        for spec in rails {
            if !spec.is_valid() {
                continue;
            }
            match curated_rail_to_track(spec) {
                Ok(track) => {
                    let duration_s = track.duration_s();
                    self.curated.push(CuratedTrackEntry {
                        id: spec.id.clone(),
                        label: spec.label.clone(),
                        description: spec.description.clone(),
                        scene_id: scene_id.to_string(),
                        track,
                        duration_s,
                    });
                }
                Err(err) => {
                    self.status_msg = format!("Skipped curated rail `{}`: {err}", spec.id);
                }
            }
        }

        let only_old_curated = !self.queue.is_empty()
            && self
                .queue
                .iter()
                .all(|s| s.origin == TrackOrigin::Curated && s.scene_id != scene_id);

        if (self.queue.is_empty() || only_old_curated) && !self.curated.is_empty() {
            let pick = self
                .curated
                .iter()
                .position(|c| !self.selected_track_id.is_empty() && c.id == self.selected_track_id)
                .unwrap_or(0);
            self.queue.clear();
            self.queue.push(PlaylistSlot::from_curated(&self.curated[pick]));
            self.active_idx = 0;
            self.selected_track_id = self.curated[pick].id.clone();
            self.time_s = 0.0;
            self.hold_elapsed_s = 0.0;
            self.prev_t = None;
            self.rebuild_shuffle_order();
            if matches!(self.state, PlayerState::Playing { .. } | PlayerState::Paused { .. }) {
                self.state = PlayerState::Playing { entry_idx: 0 };
            }
        }

        self.rescan_library();
        self.notify_scene_loaded(scene_id, scene_hash);
    }

    /// Notifies the player that the host finished loading `scene_id`, resuming a pending
    /// cross-scene `FadeBlack` transition if waiting in [`PlayerState::Loading`].
    pub fn notify_scene_loaded(&mut self, scene_id: &str, scene_hash: &str) {
        self.active_scene_id = scene_id.to_string();
        self.active_scene_hash = scene_hash.to_string();
        self.refresh_queue_stale_flags();

        let waiting = matches!(
            &self.state,
            PlayerState::Loading { scene_id: target } if target == scene_id
        );
        if waiting {
            if let Some(to_idx) = self.pending_after_load.take()
                && to_idx < self.queue.len()
            {
                let fade_half = self
                    .pending_fade_in_s
                    .take()
                    .unwrap_or(f64::from(DEFAULT_SCENE_FADE_S) * 0.5)
                    .max(0.05);
                let total = fade_half * 2.0;
                self.active_idx = to_idx;
                self.time_s = 0.0;
                self.hold_elapsed_s = 0.0;
                self.prev_t = None;
                self.pending_cut_before = true;
                self.state = PlayerState::Transition {
                    from_idx: to_idx,
                    to_idx,
                    kind: PlaylistTransition::FadeBlack { s: total as f32 },
                    t: fade_half,
                    duration_s: total,
                };
            } else {
                self.state = PlayerState::Idle;
            }
        }
    }

    /// Returns `true` when at least one track is queued or available as a curated rail.
    #[must_use]
    pub fn can_play(&self) -> bool {
        !self.queue.is_empty() || !self.curated.is_empty()
    }

    /// Returns `true` when the player is actively driving the camera (`Playing`, `Transition`, or `Loading`).
    #[must_use]
    pub fn is_active(&self) -> bool {
        matches!(
            self.state,
            PlayerState::Playing { .. }
                | PlayerState::Transition { .. }
                | PlayerState::Loading { .. }
        )
    }

    /// Starts or resumes playback from the active queue entry (seeding curated rail 0 if empty).
    pub fn play(&mut self) -> bool {
        if self.queue.is_empty() {
            if let Some(first) = self.curated.first() {
                self.queue.push(PlaylistSlot::from_curated(first));
                self.active_idx = 0;
                self.rebuild_shuffle_order();
            } else {
                self.status_msg = "No track queued or curated rail available".to_string();
                return false;
            }
        }
        let idx = self.active_idx.min(self.queue.len() - 1);
        self.active_idx = idx;
        if let Some(slot) = self.queue.get(idx) {
            let dur = slot.track.duration_s();
            if self.time_s >= dur {
                self.time_s = 0.0;
                self.hold_elapsed_s = 0.0;
                self.prev_t = None;
            }
            if !slot.scene_id.is_empty()
                && !self.active_scene_id.is_empty()
                && slot.scene_id != self.active_scene_id
            {
                let target_scene = slot.scene_id.clone();
                self.pending_after_load = Some(idx);
                self.pending_fade_in_s = Some(f64::from(DEFAULT_SCENE_FADE_S) * 0.5);
                self.state = PlayerState::Loading {
                    scene_id: target_scene,
                };
                return true;
            }
            self.selected_track_id = slot.track_id.clone();
        }
        self.fly_transition = None;
        self.state = PlayerState::Playing { entry_idx: idx };
        true
    }

    /// Pauses playback at the current timestamp without losing queue position.
    pub fn pause(&mut self) {
        match self.state {
            PlayerState::Playing { entry_idx } => {
                self.state = PlayerState::Paused { entry_idx };
            }
            PlayerState::Transition { to_idx, .. } => {
                self.fly_transition = None;
                self.active_idx = to_idx.min(self.queue.len().saturating_sub(1));
                self.time_s = 0.0;
                self.state = PlayerState::Paused {
                    entry_idx: self.active_idx,
                };
            }
            PlayerState::Loading { .. } => {
                self.pending_after_load = None;
                self.pending_fade_in_s = None;
                self.state = PlayerState::Idle;
            }
            PlayerState::Idle | PlayerState::Paused { .. } => {}
        }
    }

    /// Stops playback and rewinds the current entry to `t = 0.0`.
    pub fn stop(&mut self) {
        self.state = PlayerState::Idle;
        self.time_s = 0.0;
        self.hold_elapsed_s = 0.0;
        self.prev_t = None;
        self.pending_cut_before = false;
        self.fly_transition = None;
        self.pending_after_load = None;
        self.pending_fade_in_s = None;
        self.recording_playlist = false;
    }

    /// Jumps to the previous entry in playback order (or rewinds to `0.0` if past `2.0 s`).
    pub fn prev(&mut self) -> bool {
        if self.queue.is_empty() {
            return false;
        }
        if self.time_s > 2.0 {
            self.seek(0.0);
            return true;
        }
        let target = self
            .prev_entry_index(self.active_idx)
            .unwrap_or(self.active_idx);
        let was_playing = self.is_active();
        self.select_entry(target, was_playing)
    }

    /// Advances to the next entry in playback order.
    pub fn next(&mut self) -> bool {
        if self.queue.is_empty() {
            return false;
        }
        let target = self
            .next_entry_index_ignoring_repeat_one(self.active_idx)
            .unwrap_or(0);
        let was_playing = self.is_active();
        self.select_entry(target, was_playing)
    }

    /// Seeks the active entry to timestamp `t_s` (clamped to `[0, duration_s]`).
    pub fn seek(&mut self, t_s: f64) {
        let dur = self.duration_s();
        if dur <= 0.0 {
            self.time_s = 0.0;
            return;
        }
        let clean = if t_s.is_finite() { t_s } else { 0.0 };
        self.time_s = clean.clamp(0.0, dur);
        self.hold_elapsed_s = 0.0;
        self.prev_t = None;
        self.pending_cut_before = true;
        if let PlayerState::Transition { to_idx, .. } = self.state {
            self.fly_transition = None;
            self.active_idx = to_idx.min(self.queue.len().saturating_sub(1));
            self.state = PlayerState::Playing {
                entry_idx: self.active_idx,
            };
        }
    }

    /// Selects queue entry `idx`, resetting cursor to `0.0` and optionally starting playback.
    pub fn select_entry(&mut self, idx: usize, autoplay: bool) -> bool {
        if idx >= self.queue.len() {
            return false;
        }
        self.active_idx = idx;
        self.time_s = 0.0;
        self.hold_elapsed_s = 0.0;
        self.prev_t = None;
        self.fly_transition = None;
        self.pending_cut_before = true;
        self.selected_track_id = self.queue[idx].track_id.clone();
        if autoplay {
            self.play()
        } else {
            self.state = PlayerState::Paused { entry_idx: idx };
            true
        }
    }

    /// Appends curated rail `curated_idx` to the playlist queue.
    pub fn enqueue_curated(&mut self, curated_idx: usize) -> bool {
        let Some(entry) = self.curated.get(curated_idx) else {
            return false;
        };
        let slot = PlaylistSlot::from_curated(entry);
        let msg = format!("Enqueued curated rail `{}`", entry.label);
        self.queue.push(slot);
        self.rebuild_shuffle_order();
        self.status_msg = msg;
        true
    }

    /// Plays curated rail `curated_idx` immediately (reusing its slot if already queued).
    pub fn play_curated(&mut self, curated_idx: usize) -> bool {
        let Some(entry) = self.curated.get(curated_idx) else {
            return false;
        };
        let slot_idx = if let Some(existing) = self
            .queue
            .iter()
            .position(|s| s.origin == TrackOrigin::Curated && s.track_id == entry.id)
        {
            existing
        } else {
            self.queue.push(PlaylistSlot::from_curated(entry));
            self.rebuild_shuffle_order();
            self.queue.len() - 1
        };
        self.select_entry(slot_idx, true)
    }

    /// Loads and BLAKE3-verifies a `*.fnav.json` file and appends it to the queue.
    ///
    /// # Errors
    /// Returns an error description if the file cannot be read, parsed, or fails BLAKE3 check.
    pub fn load_and_enqueue_track(&mut self, path: &Path) -> Result<usize, String> {
        let tf = load_plan_track(path).map_err(|e| e.to_string())?;
        let origin = self.classify_origin(path);
        let slot = PlaylistSlot::from_track_file(
            tf,
            path.to_path_buf(),
            origin,
            &self.active_scene_id,
            &self.active_scene_hash,
        );
        let title = slot.title.clone();
        self.queue.push(slot);
        self.rebuild_shuffle_order();
        let idx = self.queue.len() - 1;
        self.status_msg = format!("Enqueued `{title}`");
        Ok(idx)
    }

    /// Loads a `*.fnav.json` file, appends it (or selects if already loaded), and plays it.
    ///
    /// # Errors
    /// Propagates any error from [`Self::load_and_enqueue_track`].
    pub fn load_and_play_track(&mut self, path: &Path) -> Result<usize, String> {
        let idx = if let Some(existing) = self
            .queue
            .iter()
            .position(|s| s.file_path.as_deref() == Some(path))
        {
            existing
        } else {
            self.load_and_enqueue_track(path)?
        };
        self.select_entry(idx, true);
        Ok(idx)
    }

    /// Loads a `*.fnav-playlist.json` file, resolving every entry against the library, sibling
    /// directory, or active curated rails.
    ///
    /// # Errors
    /// Returns an error description if loading fails or zero entries can be resolved.
    pub fn load_playlist(&mut self, path: &Path) -> Result<usize, String> {
        let pl = PlaylistFile::load(path).map_err(|e| e.to_string())?;
        if self.library.tracks.is_empty() {
            self.rescan_library();
        }
        let base_dir = path.parent();
        let mut resolved = Vec::with_capacity(pl.entries.len());
        let mut missing = 0usize;

        for entry in &pl.entries {
            if let Some(mut slot) = self.resolve_track_ref(&entry.track, base_dir) {
                slot.transition_in = entry.transition_in;
                slot.hold_s = if entry.hold_s.is_finite() {
                    entry.hold_s.max(0.0)
                } else {
                    0.0
                };
                resolved.push(slot);
            } else {
                missing += 1;
            }
        }

        if resolved.is_empty() {
            return Err(format!(
                "playlist `{}` had no resolvable tracks ({missing} missing)",
                pl.name
            ));
        }

        self.queue = resolved;
        self.playlist_name = pl.name;
        self.repeat = pl.repeat;
        self.shuffle = pl.shuffle;
        self.shuffle_seed = pl.shuffle_seed;
        self.active_idx = 0;
        self.time_s = 0.0;
        self.hold_elapsed_s = 0.0;
        self.prev_t = None;
        self.fly_transition = None;
        self.pending_cut_before = true;
        self.rebuild_shuffle_order();
        self.status_msg = if missing > 0 {
            format!(
                "Loaded playlist `{}` ({} tracks, {} missing)",
                self.playlist_name,
                self.queue.len(),
                missing
            )
        } else {
            format!(
                "Loaded playlist `{}` ({} tracks)",
                self.playlist_name,
                self.queue.len()
            )
        };
        Ok(self.queue.len())
    }

    /// Atomically saves the current queue as a `*.fnav-playlist.json` file in `repo_dir`
    /// (`to_repo = true`) or `user_dir` (`to_repo = false`), also persisting any unsaved slots.
    ///
    /// # Errors
    /// Returns an error description if the queue is empty or atomic disk writes fail.
    pub fn save_playlist(&mut self, name: &str, to_repo: bool) -> Result<PathBuf, String> {
        if self.queue.is_empty() {
            return Err("cannot save an empty playlist".to_string());
        }
        let dir = if to_repo {
            self.repo_dir.clone()
        } else {
            self.user_dir.clone()
        };
        let display_name = if name.trim().is_empty() {
            "demo_playlist".to_string()
        } else {
            name.trim().to_string()
        };

        let mut entries = Vec::with_capacity(self.queue.len());
        for (i, slot) in self.queue.iter_mut().enumerate() {
            let (track_id, file_hint) = if let Some(ref existing_path) = slot.file_path {
                let hint = existing_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map_or_else(
                        || format!("{}.{TRACK_FILE_EXT}", sanitize_file_stem(&slot.title)),
                        ToOwned::to_owned,
                    );
                (slot.track_id.clone(), hint)
            } else {
                let stem = sanitize_file_stem(&format!("{}_{}", slot.title, i + 1));
                let file_name = format!("{stem}.{TRACK_FILE_EXT}");
                let track_path = dir.join(&file_name);
                let tf = PlanTrackFile::new(
                    slot.title.clone(),
                    utc_now_iso8601(),
                    SceneRef {
                        id: if slot.scene_id.is_empty() {
                            self.active_scene_id.clone()
                        } else {
                            slot.scene_id.clone()
                        },
                        content_hash: self.active_scene_hash.clone(),
                    },
                    None,
                    slot.plan.clone(),
                    slot.track.clone(),
                );
                tf.save_atomic(&track_path).map_err(|e| e.to_string())?;
                slot.track_id = tf.id.clone();
                slot.file_path = Some(track_path);
                slot.origin = if to_repo {
                    TrackOrigin::RepoLibrary
                } else {
                    TrackOrigin::UserLibrary
                };
                (tf.id, file_name)
            };

            entries.push(PlaylistEntry {
                track: TrackRef {
                    id: track_id,
                    file_hint,
                },
                transition_in: slot.transition_in,
                hold_s: slot.hold_s,
            });
        }

        let mut pl = PlaylistFile::new(display_name.clone(), entries);
        pl.repeat = self.repeat;
        pl.shuffle = self.shuffle;
        pl.shuffle_seed = self.shuffle_seed;

        let stem = sanitize_file_stem(&display_name);
        let pl_path = dir.join(format!("{stem}.{PLAYLIST_FILE_EXT}"));
        pl.save_atomic(&pl_path).map_err(|e| e.to_string())?;
        self.playlist_name = display_name;
        self.rescan_library();
        self.status_msg = format!("Saved playlist {}", pl_path.display());
        Ok(pl_path)
    }

    /// Installs the loaded scene's [`LandmarkHint`] list for `RailRig` [`LookMode::Track`].
    ///
    /// Automatically selects the highest-priority landmark when no valid landmark is currently
    /// selected so switching into `Track` mode locks immediately onto a hero subject.
    pub fn set_scene_landmarks(&mut self, landmarks: Vec<LandmarkHint>) {
        self.scene_landmarks = landmarks;
        let current_valid = self
            .rig_track_landmark
            .as_deref()
            .is_some_and(|name| self.scene_landmarks.iter().any(|l| l.name == name));
        if !current_valid {
            self.rig_track_landmark = self
                .scene_landmarks
                .iter()
                .max_by_key(|l| l.priority)
                .map(|l| l.name.clone());
        }
        self.sync_cached_rig_mode();
    }

    /// Switches the camera rig mode (`Cinematic`, `AlongPath`, `Track`, `Free`) without jumping
    /// gimbal angles on an active [`RailRig`].
    pub fn set_rig_mode(&mut self, mode: FlightRigCameraMode) {
        self.rig_mode = mode;
        if mode == FlightRigCameraMode::Cinematic {
            self.last_rig.active = false;
            self.pending_rig_input = RigInput::default();
        } else {
            self.sync_cached_rig_mode();
        }
    }

    /// Selects the named scene landmark tracked in [`FlightRigCameraMode::Track`].
    pub fn set_rig_track_landmark(&mut self, landmark_name: String) {
        self.rig_track_landmark = (!landmark_name.is_empty()).then_some(landmark_name);
        self.sync_cached_rig_mode();
    }

    /// Updates [`DroneTilt`] filter settings (`max_tilt_deg`, `tau_s`) on the cached [`RailRig`].
    pub fn set_rig_tilt(&mut self, tilt: DroneTilt) {
        self.rig_tilt = DroneTilt {
            max_tilt_deg: if tilt.max_tilt_deg.is_finite() {
                tilt.max_tilt_deg.clamp(0.0, 45.0)
            } else {
                DroneTilt::default().max_tilt_deg
            },
            tau_s: if tilt.tau_s.is_finite() {
                tilt.tau_s.clamp(0.02, 2.0)
            } else {
                DroneTilt::default().tau_s
            },
        };
        if let Some((_, _, rig)) = self.active_rig.as_mut() {
            rig.tilt = self.rig_tilt;
        }
    }

    /// Accumulates viewport drag deltas `[yaw_deg, pitch_deg, roll_deg]` for [`FlightRigCameraMode::Free`].
    #[inline]
    pub fn push_rig_input(&mut self, delta_deg: [f32; 3]) {
        for (dst, d) in self.pending_rig_input.free_delta_deg.iter_mut().zip(delta_deg) {
            if d.is_finite() {
                *dst += d;
            }
        }
    }

    /// Returns `true` when playback is active and driven by `csg_scene::RailRig`.
    #[inline]
    #[must_use]
    pub fn is_rig_active(&self) -> bool {
        self.is_active() && self.rig_mode.uses_rail_rig()
    }

    /// Returns `true` when playback is active in `RailRig` [`FlightRigCameraMode::Free`] mode
    /// (viewport drag steers the gimbal without pausing playback).
    #[inline]
    #[must_use]
    pub fn is_rig_free_look_active(&self) -> bool {
        self.is_active() && self.rig_mode == FlightRigCameraMode::Free
    }

    /// Ejects cleanly from the drone `RailRig` into free camera control at the rig's latest pose.
    ///
    /// Pauses active playback (leaving the cursor at the current timestamp for easy resume),
    /// clears queued gimbal deltas, and returns the latest camera pose so the host can snap
    /// `CameraDriver` + `CameraDamper` seamlessly without a camera jump.
    pub fn eject_rig(&mut self) -> Option<CameraPose> {
        self.pending_rig_input = RigInput::default();
        self.last_rig.active = false;
        self.pause();
        self.status_msg = "Ejected from RailRig to free camera".to_string();
        self.last_pose
    }

    /// Builds the `csg_scene::LookMode` corresponding to [`Self::rig_mode`] and [`Self::rig_track_landmark`].
    fn desired_look_mode(&self) -> LookMode {
        match self.rig_mode {
            FlightRigCameraMode::Cinematic | FlightRigCameraMode::AlongPath => LookMode::AlongPath,
            FlightRigCameraMode::Free => LookMode::Free,
            FlightRigCameraMode::Track => match self.rig_track_landmark.as_deref() {
                Some(name) if !name.is_empty() => {
                    LookMode::Track(TrackTarget::Landmark(name.to_string()))
                }
                _ => LookMode::AlongPath,
            },
        }
    }

    /// Syncs look mode and tilt parameters onto the cached [`RailRig`] (if instantiated).
    fn sync_cached_rig_mode(&mut self) {
        let mode = self.desired_look_mode();
        let tilt = self.rig_tilt;
        if let Some((_, _, rig)) = self.active_rig.as_mut() {
            rig.tilt = tilt;
            if rig.mode != mode {
                rig.set_mode(mode);
            }
        }
    }

    /// Marks any cached [`RailRig`] stale after queue mutation or track replacement.
    #[inline]
    fn invalidate_cached_rig(&mut self) {
        self.rig_generation = self.rig_generation.wrapping_add(1);
        self.active_rig = None;
    }

    /// Steps the cached `csg_scene::RailRig<FlightBodySource>` for `entry_idx` with zero per-tick
    /// allocations, updating [`Self::last_rig`] and returning the gimbal [`CameraPose`].
    fn step_slot_rail_rig(
        &mut self,
        entry_idx: usize,
        eval_t: f64,
        dt: f64,
        vfov_deg: f32,
        znear: f32,
        zfar: f32,
    ) -> Option<CameraPose> {
        let need_rebuild = match self.active_rig.as_ref() {
            Some((idx, gen_id, _)) => *idx != entry_idx || *gen_id != self.rig_generation,
            None => true,
        };
        if need_rebuild {
            let slot = self.queue.get(entry_idx)?;
            let src = FlightBodySource::from_slot(slot);
            let mut rig = RailRig::new(src, vfov_deg);
            rig.tilt = self.rig_tilt;
            rig.set_mode(self.desired_look_mode());
            self.active_rig = Some((entry_idx, self.rig_generation, rig));
        }

        let input = std::mem::take(&mut self.pending_rig_input);
        let desired_mode = self.desired_look_mode();
        let tilt_cfg = self.rig_tilt;
        let targets = LandmarkTargets(&self.scene_landmarks);
        let (_, _, rig) = self.active_rig.as_mut()?;
        rig.tilt = tilt_cfg;
        rig.vfov_deg = vfov_deg;
        if rig.mode != desired_mode {
            rig.set_mode(desired_mode);
        }
        let rig_pose = rig.step(eval_t as f32, dt as f32, &targets, &input);
        let up_y = (rig_pose.body_xform.xform_pt(csg_core::Vec3::Y)
            - rig_pose.body_xform.xform_pt(csg_core::Vec3::ZERO))
            .y
            .clamp(-1.0, 1.0);
        self.last_rig = RigTelemetry {
            active: true,
            tilt_deg: up_y.acos().to_degrees(),
            gimbal_deg: rig.gimbal.angle_deg,
            track_status: rig_pose.track,
            track_error_deg: rig_pose.track_error_deg,
            rotor_phase: rig_pose.rotor_phase,
        };
        Some(csg_cam_to_pose(rig_pose.camera, znear, zfar))
    }

    /// Reorders a playlist slot from `from` to `to`, keeping `active_idx` synced to the same slot.
    pub fn reorder(&mut self, from: usize, to: usize) {
        if from >= self.queue.len() || to >= self.queue.len() || from == to {
            return;
        }
        let item = self.queue.remove(from);
        self.queue.insert(to, item);
        self.active_idx = remap_index_after_move(self.active_idx, from, to);
        match &mut self.state {
            PlayerState::Playing { entry_idx } | PlayerState::Paused { entry_idx } => {
                *entry_idx = remap_index_after_move(*entry_idx, from, to);
            }
            PlayerState::Transition {
                from_idx, to_idx, ..
            } => {
                *from_idx = remap_index_after_move(*from_idx, from, to);
                *to_idx = remap_index_after_move(*to_idx, from, to);
            }
            PlayerState::Idle | PlayerState::Loading { .. } => {}
        }
        self.rebuild_shuffle_order();
        self.invalidate_cached_rig();
    }

    /// Removes slot `idx` from the playlist queue, stopping safely if the queue becomes empty.
    pub fn remove(&mut self, idx: usize) {
        if idx >= self.queue.len() {
            return;
        }
        self.queue.remove(idx);
        if self.queue.is_empty() {
            self.active_idx = 0;
            self.stop();
            self.rebuild_shuffle_order();
            return;
        }
        if self.active_idx == idx {
            self.active_idx = idx.min(self.queue.len() - 1);
            self.time_s = 0.0;
            self.hold_elapsed_s = 0.0;
            self.prev_t = None;
            self.fly_transition = None;
            if matches!(
                self.state,
                PlayerState::Playing { .. } | PlayerState::Transition { .. }
            ) {
                self.state = PlayerState::Playing {
                    entry_idx: self.active_idx,
                };
            } else if matches!(self.state, PlayerState::Paused { .. }) {
                self.state = PlayerState::Paused {
                    entry_idx: self.active_idx,
                };
            }
        } else if idx < self.active_idx {
            self.active_idx -= 1;
            match &mut self.state {
                PlayerState::Playing { entry_idx } | PlayerState::Paused { entry_idx } => {
                    *entry_idx = self.active_idx;
                }
                PlayerState::Transition { .. } => {
                    self.fly_transition = None;
                    self.state = PlayerState::Playing {
                        entry_idx: self.active_idx,
                    };
                }
                PlayerState::Idle | PlayerState::Loading { .. } => {}
            }
        }
        self.rebuild_shuffle_order();
        self.invalidate_cached_rig();
    }

    /// Updates the entry transition mode of slot `idx`.
    pub fn set_entry_transition(&mut self, idx: usize, transition: PlaylistTransition) {
        if let Some(slot) = self.queue.get_mut(idx) {
            slot.transition_in = transition;
        }
    }

    /// Updates the end-of-slot hold duration (seconds) of slot `idx`.
    pub fn set_entry_hold(&mut self, idx: usize, hold_s: f32) {
        if let Some(slot) = self.queue.get_mut(idx) {
            slot.hold_s = if hold_s.is_finite() {
                hold_s.clamp(0.0, 30.0)
            } else {
                0.0
            };
        }
    }

    /// Sets the playlist [`RepeatMode`].
    pub fn set_repeat(&mut self, repeat: RepeatMode) {
        self.repeat = repeat;
    }

    /// Enables or disables deterministic playlist shuffle.
    pub fn set_shuffle(&mut self, shuffle: bool) {
        self.shuffle = shuffle;
        self.rebuild_shuffle_order();
    }

    /// Replaces the currently active slot's [`Track`] in-place (used when Flight Lab applies
    /// updated `camera-guide-rs` limits to the active track).
    ///
    /// # Errors
    /// Returns an error description if no active slot exists.
    pub fn replace_active_track(&mut self, track: Track) -> Result<(), String> {
        let Some(slot) = self.queue.get_mut(self.active_idx) else {
            return Err("no active track in player queue".to_string());
        };
        let new_dur = track.duration_s();
        slot.track = track;
        if self.time_s > new_dur {
            self.time_s = 0.0;
        }
        self.prev_t = None;
        self.status_msg = format!("Updated active track `{}` ({:.1} s)", slot.title, new_dur);
        self.invalidate_cached_rig();
        Ok(())
    }

    /// Installs a Flight Lab preview track into the queue (reusing any existing `LabPreview` slot)
    /// and starts playing it immediately.
    pub fn preview_lab_track(&mut self, title: String, track: Track, plan: Option<RailPlan>) {
        let h = track.canonical_hash();
        let slot = PlaylistSlot {
            title,
            description: "Live preview authored in Flight Lab".to_string(),
            scene_id: self.active_scene_id.clone(),
            track_id: format!("lab_{:02x}{:02x}{:02x}{:02x}", h[0], h[1], h[2], h[3]),
            file_path: None,
            track,
            plan,
            transition_in: PlaylistTransition::Cut,
            hold_s: 0.0,
            stale: false,
            origin: TrackOrigin::LabPreview,
        };
        let idx = if let Some(existing) = self
            .queue
            .iter()
            .position(|s| s.origin == TrackOrigin::LabPreview)
        {
            self.queue[existing] = slot;
            existing
        } else {
            self.queue.push(slot);
            self.rebuild_shuffle_order();
            self.queue.len() - 1
        };
        self.invalidate_cached_rig();
        self.select_entry(idx, true);
    }

    /// Computes the effective transition from `from_idx` to `to_idx`, forcing
    /// [`PlaylistTransition::FadeBlack`] whenever the destination targets a different scene.
    #[must_use]
    pub fn effective_transition_between(
        &self,
        from_idx: usize,
        to_idx: usize,
    ) -> PlaylistTransition {
        let Some(to_slot) = self.queue.get(to_idx) else {
            return PlaylistTransition::Cut;
        };
        let from_scene = self
            .queue
            .get(from_idx)
            .map(|s| s.scene_id.as_str())
            .filter(|s| !s.is_empty());
        let target_scene = (!to_slot.scene_id.is_empty()).then_some(to_slot.scene_id.as_str());
        let active_scene = (!self.active_scene_id.is_empty()).then_some(self.active_scene_id.as_str());

        let scene_changes = match (from_scene.or(active_scene), target_scene) {
            (Some(a), Some(b)) => a != b,
            _ => false,
        };
        if scene_changes {
            match to_slot.transition_in {
                PlaylistTransition::FadeBlack { s } if s.is_finite() && s > 0.0 => {
                    PlaylistTransition::FadeBlack { s }
                }
                _ => PlaylistTransition::FadeBlack {
                    s: DEFAULT_SCENE_FADE_S,
                },
            }
        } else {
            to_slot.transition_in
        }
    }

    /// Advances playback by `dt_s` seconds and evaluates the camera pose, `cut_before`, and
    /// transition/scene-switch signals. Returns `None` while [`PlayerState::Idle`] or
    /// [`PlayerState::Paused`].
    pub fn tick(&mut self, dt_s: f64, znear: f32, zfar: f32) -> Option<PlayerEval> {
        let dt = if dt_s.is_finite() && dt_s >= 0.0 {
            dt_s
        } else {
            0.0
        };

        match self.state.clone() {
            PlayerState::Idle | PlayerState::Paused { .. } => None,
            PlayerState::Loading { scene_id } => {
                let pose = self
                    .last_pose
                    .or_else(|| self.evaluate_current(znear, zfar))
                    .unwrap_or_else(|| CameraState::default().to_camera_pose(znear, zfar));
                Some(PlayerEval {
                    pose,
                    cut_before: false,
                    fade_alpha: 1.0,
                    request_scene_id: Some(scene_id),
                    speed_mps: 0.0,
                    bank_rad: 0.0,
                    active_chase: false,
                    subject_fallback: false,
                })
            }
            PlayerState::Transition {
                from_idx,
                to_idx,
                kind,
                t,
                duration_s,
            } => self.tick_transition(
                TransitionStep {
                    from_idx,
                    to_idx,
                    kind,
                    t,
                    duration_s,
                },
                dt,
                znear,
                zfar,
            ),
            PlayerState::Playing { entry_idx } => self.tick_playing(entry_idx, dt, znear, zfar),
        }
    }

    /// Evaluates the current slot at [`Self::time_s`] without advancing playback state.
    #[must_use]
    pub fn evaluate_current(&self, znear: f32, zfar: f32) -> Option<CameraPose> {
        let slot = self.queue.get(self.active_idx)?;
        if slot.track.duration_s() <= 0.0 {
            return None;
        }
        Some(slot.track.eval(self.time_s).state.to_camera_pose(znear, zfar))
    }

    /// Currently active [`Track`], if any slot is queued.
    #[must_use]
    pub fn active_track(&self) -> Option<&Track> {
        self.queue.get(self.active_idx).map(|s| &s.track)
    }

    /// Currently active [`PlaylistSlot`], if any slot is queued.
    #[must_use]
    pub fn active_slot(&self) -> Option<&PlaylistSlot> {
        self.queue.get(self.active_idx)
    }

    /// Display title of the currently active slot (or `"None"` when queue is empty).
    #[must_use]
    pub fn active_title(&self) -> &str {
        self.queue
            .get(self.active_idx)
            .map_or("None", |s| s.title.as_str())
    }

    /// Stable identifier of the currently active slot (empty string when queue is empty).
    #[must_use]
    pub fn active_id(&self) -> &str {
        self.queue
            .get(self.active_idx)
            .map_or("", |s| s.track_id.as_str())
    }

    /// Duration in seconds (`f64`) of the active slot's track (`0.0` when empty).
    #[must_use]
    pub fn duration_s(&self) -> f64 {
        self.queue
            .get(self.active_idx)
            .map_or(0.0, |s| s.track.duration_s())
    }

    /// Selects a curated rail by its `RailSpec::id` for `--bench-rail` CLI benchmarks.
    pub fn select_curated_by_id(&mut self, id: &str) -> Option<usize> {
        let idx = self.curated.iter().position(|c| c.id == id)?;
        self.queue.clear();
        self.queue.push(PlaylistSlot::from_curated(&self.curated[idx]));
        self.active_idx = 0;
        self.time_s = 0.0;
        self.hold_elapsed_s = 0.0;
        self.prev_t = None;
        self.pending_cut_before = true;
        self.selected_track_id = self.curated[idx].id.clone();
        self.rebuild_shuffle_order();
        self.state = PlayerState::Playing { entry_idx: 0 };
        Some(idx)
    }

    /// Loads one verified `*.fnav.json` track as the sole active benchmark entry (`--bench-track`).
    ///
    /// # Errors
    /// Returns an error description if the file fails to load or BLAKE3 verification fails.
    pub fn load_bench_track(&mut self, path: &Path) -> Result<(), String> {
        let tf = load_plan_track(path).map_err(|e| e.to_string())?;
        let origin = self.classify_origin(path);
        let slot = PlaylistSlot::from_track_file(
            tf,
            path.to_path_buf(),
            origin,
            &self.active_scene_id,
            &self.active_scene_hash,
        );
        self.queue.clear();
        self.queue.push(slot);
        self.active_idx = 0;
        self.time_s = 0.0;
        self.hold_elapsed_s = 0.0;
        self.prev_t = None;
        self.pending_cut_before = true;
        self.repeat = RepeatMode::Off;
        self.shuffle = false;
        self.rebuild_shuffle_order();
        self.state = PlayerState::Playing { entry_idx: 0 };
        Ok(())
    }

    /// Loads a `*.fnav-playlist.json` file and begins benchmark playback (`--bench-playlist`).
    ///
    /// # Errors
    /// Propagates any error from [`Self::load_playlist`].
    pub fn load_bench_playlist(&mut self, path: &Path) -> Result<(), String> {
        self.load_playlist(path)?;
        self.play();
        Ok(())
    }

    /// Captures persisted player preferences for `nv-demo.json`.
    #[must_use]
    pub fn to_persisted(&self) -> PersistedPlayerPrefs {
        PersistedPlayerPrefs {
            repeat: self.repeat,
            shuffle: self.shuffle,
            shuffle_seed: self.shuffle_seed,
            selected_track_id: if !self.active_id().is_empty() {
                self.active_id().to_string()
            } else {
                self.selected_track_id.clone()
            },
        }
    }

    /// Applies persisted preferences into this [`FlightPlayer`].
    pub fn apply_persisted(&mut self, prefs: &PersistedPlayerPrefs) {
        self.repeat = prefs.repeat;
        self.shuffle = prefs.shuffle;
        self.shuffle_seed = prefs.shuffle_seed;
        self.selected_track_id = prefs.selected_track_id.clone();
        self.rebuild_shuffle_order();
    }

    /// Advances active track playback and initiates end-of-track holds, loops, or transitions.
    fn tick_playing(
        &mut self,
        entry_idx: usize,
        dt: f64,
        znear: f32,
        zfar: f32,
    ) -> Option<PlayerEval> {
        if entry_idx >= self.queue.len() {
            self.stop();
            return None;
        }
        self.active_idx = entry_idx;
        let dur = self.queue[entry_idx].track.duration_s();
        if dur <= 1.0e-5 {
            self.stop();
            return None;
        }

        let looping = self.queue[entry_idx].track.looping;
        let hold_s = f64::from(self.queue[entry_idx].hold_s.max(0.0));
        let next_t = self.time_s + dt;

        if next_t < dur {
            let sample = self.queue[entry_idx].track.eval_step(self.prev_t, next_t);
            self.prev_t = Some(next_t);
            self.time_s = next_t;
            let cut_before = sample.cut_before || std::mem::take(&mut self.pending_cut_before);
            let base_pose = sample.state.to_camera_pose(znear, zfar);
            let pose = if self.rig_mode.uses_rail_rig() {
                self.step_slot_rail_rig(
                    entry_idx,
                    next_t,
                    dt,
                    base_pose.fov_y.to_degrees(),
                    znear,
                    zfar,
                )
                .unwrap_or(base_pose)
            } else {
                self.last_rig.active = false;
                base_pose
            };
            return Some(self.finish_eval(
                pose,
                cut_before,
                0.0,
                None,
                sample.speed_mps,
                sample.bank_rad,
                sample.active_chase,
                sample.subject.is_some(),
            ));
        }

        let overflow = next_t - dur;
        if self.hold_elapsed_s < hold_s {
            self.time_s = dur;
            self.hold_elapsed_s += overflow.max(dt);
            let sample = self.queue[entry_idx].track.eval_step(self.prev_t, dur);
            self.prev_t = Some(dur);
            let cut_before = sample.cut_before || std::mem::take(&mut self.pending_cut_before);
            return Some(self.finish_eval(
                sample.state.to_camera_pose(znear, zfar),
                cut_before,
                0.0,
                None,
                0.0,
                sample.bank_rad,
                false,
                sample.subject.is_some(),
            ));
        }

        self.hold_elapsed_s = 0.0;
        let single_loop = !self.recording_playlist
            && (self.repeat == RepeatMode::One
                || (self.repeat == RepeatMode::All && self.queue.len() == 1));
        if single_loop {
            if looping && hold_s <= 0.0 {
                let sample = self.queue[entry_idx].track.eval_step(self.prev_t, next_t);
                let wrapped = next_t.rem_euclid(dur);
                self.time_s = wrapped;
                self.prev_t = Some(wrapped);
                let cut_before = sample.cut_before || std::mem::take(&mut self.pending_cut_before);
                let base_pose = sample.state.to_camera_pose(znear, zfar);
                let pose = if self.rig_mode.uses_rail_rig() {
                    self.step_slot_rail_rig(
                        entry_idx,
                        wrapped,
                        dt,
                        base_pose.fov_y.to_degrees(),
                        znear,
                        zfar,
                    )
                    .unwrap_or(base_pose)
                } else {
                    self.last_rig.active = false;
                    base_pose
                };
                return Some(self.finish_eval(
                    pose,
                    cut_before,
                    0.0,
                    None,
                    sample.speed_mps,
                    sample.bank_rad,
                    sample.active_chase,
                    sample.subject.is_some(),
                ));
            }
            self.time_s = 0.0;
            self.prev_t = Some(0.0);
            let sample = self.queue[entry_idx].track.eval(0.0);
            return Some(self.finish_eval(
                sample.state.to_camera_pose(znear, zfar),
                true,
                0.0,
                None,
                sample.speed_mps,
                sample.bank_rad,
                sample.active_chase,
                sample.subject.is_some(),
            ));
        }

        let next_idx_opt = if self.recording_playlist {
            self.next_entry_index_no_wrap(entry_idx)
        } else {
            self.next_entry_index(entry_idx)
        };

        let Some(next_idx) = next_idx_opt else {
            let sample = self.queue[entry_idx].track.eval(dur);
            self.time_s = dur;
            self.state = PlayerState::Idle;
            self.recording_playlist = false;
            return Some(self.finish_eval(
                sample.state.to_camera_pose(znear, zfar),
                false,
                0.0,
                None,
                0.0,
                sample.bank_rad,
                false,
                sample.subject.is_some(),
            ));
        };

        self.begin_transition(entry_idx, next_idx, znear, zfar)
    }

    /// Initiates a `Cut`, `FadeBlack`, or same-scene C2 `FlyThrough` transition.
    fn begin_transition(
        &mut self,
        from_idx: usize,
        to_idx: usize,
        znear: f32,
        zfar: f32,
    ) -> Option<PlayerEval> {
        let eff = self.effective_transition_between(from_idx, to_idx);
        match eff {
            PlaylistTransition::Cut => {
                self.enter_slot_playing(to_idx);
                let sample = self.queue[to_idx].track.eval(0.0);
                Some(self.finish_eval(
                    sample.state.to_camera_pose(znear, zfar),
                    true,
                    0.0,
                    None,
                    sample.speed_mps,
                    sample.bank_rad,
                    sample.active_chase,
                    sample.subject.is_some(),
                ))
            }
            PlaylistTransition::FadeBlack { s } => {
                let dur = f64::from(if s.is_finite() && s > 0.05 {
                    s
                } else {
                    DEFAULT_SCENE_FADE_S
                });
                self.fly_transition = None;
                self.state = PlayerState::Transition {
                    from_idx,
                    to_idx,
                    kind: PlaylistTransition::FadeBlack { s: dur as f32 },
                    t: 0.0,
                    duration_s: dur,
                };
                let from_end = self.queue[from_idx].track.duration_s();
                let sample = self.queue[from_idx].track.eval(from_end);
                Some(self.finish_eval(
                    sample.state.to_camera_pose(znear, zfar),
                    false,
                    0.0,
                    None,
                    sample.speed_mps,
                    sample.bank_rad,
                    false,
                    false,
                ))
            }
            PlaylistTransition::FlyThrough => {
                let from_end = self.queue[from_idx].track.duration_s();
                let from_kin = self.queue[from_idx].track.eval(from_end).kinematic;
                let to_kin = self.queue[to_idx].track.eval(0.0).kinematic;
                let dist = from_kin.state.eye.distance(to_kin.state.eye);
                let speed = from_kin
                    .vel
                    .length()
                    .max(to_kin.vel.length())
                    .clamp(4.0, 20.0);
                let dur = f64::from((dist / speed).clamp(1.5, 6.0));
                let tr = Transition::direct(from_kin, to_kin, dur, Aim::Squad);
                let pose = tr.at(0.0).state.to_camera_pose(znear, zfar);
                self.fly_transition = Some(tr);
                self.state = PlayerState::Transition {
                    from_idx,
                    to_idx,
                    kind: PlaylistTransition::FlyThrough,
                    t: 0.0,
                    duration_s: dur,
                };
                Some(self.finish_eval(
                    pose,
                    false,
                    0.0,
                    None,
                    from_kin.vel.length(),
                    0.0,
                    false,
                    false,
                ))
            }
        }
    }

    /// Advances an active `FadeBlack` or `FlyThrough` inter-entry transition.
    fn tick_transition(
        &mut self,
        step: TransitionStep,
        dt: f64,
        znear: f32,
        zfar: f32,
    ) -> Option<PlayerEval> {
        if step.to_idx >= self.queue.len() {
            self.stop();
            return None;
        }
        let dur = step.duration_s.max(0.05);
        let new_t = step.t + dt;

        match step.kind {
            PlaylistTransition::FadeBlack { .. } => {
                let half = dur * 0.5;
                if new_t < half {
                    let alpha = (new_t / half).clamp(0.0, 1.0) as f32;
                    let from_slot = self.queue.get(step.from_idx).unwrap_or(&self.queue[step.to_idx]);
                    let sample = from_slot.track.eval(from_slot.track.duration_s());
                    self.state = PlayerState::Transition {
                        from_idx: step.from_idx,
                        to_idx: step.to_idx,
                        kind: step.kind,
                        t: new_t,
                        duration_s: dur,
                    };
                    Some(self.finish_eval(
                        sample.state.to_camera_pose(znear, zfar),
                        false,
                        alpha,
                        None,
                        0.0,
                        sample.bank_rad,
                        false,
                        false,
                    ))
                } else if step.t < half
                    && !self.queue[step.to_idx].scene_id.is_empty()
                    && !self.active_scene_id.is_empty()
                    && self.queue[step.to_idx].scene_id != self.active_scene_id
                {
                    let req_scene = self.queue[step.to_idx].scene_id.clone();
                    self.pending_after_load = Some(step.to_idx);
                    self.pending_fade_in_s = Some(half);
                    self.state = PlayerState::Loading {
                        scene_id: req_scene.clone(),
                    };
                    let pose = self.last_pose.unwrap_or_else(|| {
                        self.queue[step.to_idx]
                            .track
                            .eval(0.0)
                            .state
                            .to_camera_pose(znear, zfar)
                    });
                    Some(self.finish_eval(
                        pose,
                        true,
                        1.0,
                        Some(req_scene),
                        0.0,
                        0.0,
                        false,
                        false,
                    ))
                } else if new_t < dur {
                    let alpha = (1.0 - (new_t - half) / half).clamp(0.0, 1.0) as f32;
                    let crossed_mid = step.t < half || std::mem::take(&mut self.pending_cut_before);
                    self.active_idx = step.to_idx;
                    self.time_s = 0.0;
                    self.selected_track_id = self.queue[step.to_idx].track_id.clone();
                    let sample = self.queue[step.to_idx].track.eval(0.0);
                    self.state = PlayerState::Transition {
                        from_idx: step.from_idx,
                        to_idx: step.to_idx,
                        kind: step.kind,
                        t: new_t,
                        duration_s: dur,
                    };
                    Some(self.finish_eval(
                        sample.state.to_camera_pose(znear, zfar),
                        crossed_mid,
                        alpha,
                        None,
                        0.0,
                        sample.bank_rad,
                        false,
                        false,
                    ))
                } else {
                    self.enter_slot_playing(step.to_idx);
                    let sample = self.queue[step.to_idx].track.eval(0.0);
                    Some(self.finish_eval(
                        sample.state.to_camera_pose(znear, zfar),
                        false,
                        0.0,
                        None,
                        sample.speed_mps,
                        sample.bank_rad,
                        sample.active_chase,
                        sample.subject.is_some(),
                    ))
                }
            }
            PlaylistTransition::FlyThrough => {
                if new_t < dur
                    && let Some(ref tr) = self.fly_transition
                {
                    let kin = tr.at(new_t);
                    let pose = kin.state.to_camera_pose(znear, zfar);
                    let speed = kin.vel.length();
                    self.state = PlayerState::Transition {
                        from_idx: step.from_idx,
                        to_idx: step.to_idx,
                        kind: step.kind,
                        t: new_t,
                        duration_s: dur,
                    };
                    return Some(self.finish_eval(pose, false, 0.0, None, speed, 0.0, false, false));
                }
                self.enter_slot_playing(step.to_idx);
                let sample = self.queue[step.to_idx].track.eval(0.0);
                Some(self.finish_eval(
                    sample.state.to_camera_pose(znear, zfar),
                    false,
                    0.0,
                    None,
                    sample.speed_mps,
                    sample.bank_rad,
                    sample.active_chase,
                    sample.subject.is_some(),
                ))
            }
            PlaylistTransition::Cut => {
                self.enter_slot_playing(step.to_idx);
                let sample = self.queue[step.to_idx].track.eval(0.0);
                Some(self.finish_eval(
                    sample.state.to_camera_pose(znear, zfar),
                    true,
                    0.0,
                    None,
                    sample.speed_mps,
                    sample.bank_rad,
                    sample.active_chase,
                    sample.subject.is_some(),
                ))
            }
        }
    }

    /// Switches state cleanly into [`PlayerState::Playing`] at `t = 0.0` for `to_idx`.
    fn enter_slot_playing(&mut self, to_idx: usize) {
        self.fly_transition = None;
        self.active_idx = to_idx;
        self.time_s = 0.0;
        self.hold_elapsed_s = 0.0;
        self.prev_t = Some(0.0);
        self.selected_track_id = self.queue[to_idx].track_id.clone();
        self.state = PlayerState::Playing { entry_idx: to_idx };
    }

    /// Records live telemetry and builds a [`PlayerEval`] without code duplication.
    #[expect(
        clippy::too_many_arguments,
        reason = "private builder mirroring PlayerEval fields 1-to-1"
    )]
    fn finish_eval(
        &mut self,
        pose: CameraPose,
        cut_before: bool,
        fade_alpha: f32,
        request_scene_id: Option<String>,
        speed_mps: f32,
        bank_rad: f32,
        active_chase: bool,
        subject_fallback: bool,
    ) -> PlayerEval {
        self.last_pose = Some(pose);
        self.last_speed_mps = speed_mps;
        self.last_bank_rad = bank_rad;
        self.last_active_chase = active_chase;
        PlayerEval {
            pose,
            cut_before,
            fade_alpha,
            request_scene_id,
            speed_mps,
            bank_rad,
            active_chase,
            subject_fallback,
        }
    }

    /// Rebuilds the deterministic SplitMix64 shuffle permutation for `0..queue.len()`.
    fn rebuild_shuffle_order(&mut self) {
        let n = self.queue.len();
        self.shuffle_order.clear();
        self.shuffle_order.extend(0..n);
        if self.shuffle && n > 1 {
            let mut rng = SplitMix64::new(self.shuffle_seed, SHUFFLE_RNG_SALT);
            for i in (1..n).rev() {
                let j = rng.next_usize(i + 1);
                self.shuffle_order.swap(i, j);
            }
        }
    }

    fn next_entry_index(&self, cur_idx: usize) -> Option<usize> {
        if self.queue.is_empty() {
            return None;
        }
        if self.repeat == RepeatMode::One {
            return Some(cur_idx);
        }
        self.next_entry_index_ignoring_repeat_one(cur_idx)
    }

    fn next_entry_index_ignoring_repeat_one(&self, cur_idx: usize) -> Option<usize> {
        if self.queue.is_empty() {
            return None;
        }
        let pos = self
            .shuffle_order
            .iter()
            .position(|&idx| idx == cur_idx)
            .unwrap_or(0);
        if pos + 1 < self.shuffle_order.len() {
            Some(self.shuffle_order[pos + 1])
        } else if self.repeat == RepeatMode::All {
            self.shuffle_order.first().copied()
        } else {
            None
        }
    }

    fn next_entry_index_no_wrap(&self, cur_idx: usize) -> Option<usize> {
        let pos = self.shuffle_order.iter().position(|&idx| idx == cur_idx)?;
        self.shuffle_order.get(pos + 1).copied()
    }

    fn prev_entry_index(&self, cur_idx: usize) -> Option<usize> {
        if self.queue.is_empty() {
            return None;
        }
        let pos = self
            .shuffle_order
            .iter()
            .position(|&idx| idx == cur_idx)
            .unwrap_or(0);
        if pos > 0 {
            self.shuffle_order.get(pos - 1).copied()
        } else if self.repeat == RepeatMode::All {
            self.shuffle_order.last().copied()
        } else {
            self.shuffle_order.first().copied()
        }
    }

    fn resolve_track_ref(
        &self,
        tref: &TrackRef,
        playlist_dir: Option<&Path>,
    ) -> Option<PlaylistSlot> {
        if let Some(lt) = self.library.tracks.iter().find(|t| t.id == tref.id)
            && let Ok(tf) = load_plan_track(&lt.path)
        {
            let origin = match lt.source {
                LibrarySource::Repo => TrackOrigin::RepoLibrary,
                LibrarySource::User => TrackOrigin::UserLibrary,
            };
            return Some(PlaylistSlot::from_track_file(
                tf,
                lt.path.clone(),
                origin,
                &self.active_scene_id,
                &self.active_scene_hash,
            ));
        }

        if !tref.file_hint.is_empty() {
            let candidates = [
                playlist_dir.map(|d| d.join(&tref.file_hint)),
                Some(self.repo_dir.join(&tref.file_hint)),
                Some(self.user_dir.join(&tref.file_hint)),
            ];
            for cand in candidates.into_iter().flatten() {
                if cand.is_file()
                    && let Ok(tf) = load_plan_track(&cand)
                {
                    let origin = self.classify_origin(&cand);
                    return Some(PlaylistSlot::from_track_file(
                        tf,
                        cand,
                        origin,
                        &self.active_scene_id,
                        &self.active_scene_hash,
                    ));
                }
            }
        }

        self.curated
            .iter()
            .find(|c| c.id == tref.id)
            .map(PlaylistSlot::from_curated)
    }

    fn classify_origin(&self, path: &Path) -> TrackOrigin {
        if path.starts_with(&self.repo_dir) {
            TrackOrigin::RepoLibrary
        } else {
            TrackOrigin::UserLibrary
        }
    }

    fn refresh_queue_stale_flags(&mut self) {
        for slot in &mut self.queue {
            if let Some(lt) = self.library.tracks.iter().find(|t| t.id == slot.track_id) {
                slot.stale = lt.stale;
            }
        }
    }
}

/// Remaps an index `cur` after removing element `from` and re-inserting it at `to`.
fn remap_index_after_move(cur: usize, from: usize, to: usize) -> usize {
    if cur == from {
        to
    } else if from < to && cur > from && cur <= to {
        cur - 1
    } else if to < from && cur >= to && cur < from {
        cur + 1
    } else {
        cur
    }
}
