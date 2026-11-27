//! Unified flight navigation workspace (`flight-navigator-rs` + `camera-guide-rs`), camera
//! gimbal damper, 180-degree shutter timing, and Flight Lab / Flight Player dock UI for `nv-demo`.
//!
//! **What.** Owns [`FlightWorkspace`] (one single source of truth pairing [`player::FlightPlayer`]
//! and [`lab::FlightLabState`]), typed [`FlightCommand`] actions routed through `AppBus`,
//! [`CameraDamper`], [`shutter_subframe_dt`], library directory resolvers
//! ([`default_repo_flights_dir`], [`default_user_flights_dir`]), and the six submodules
//! [`convert`], [`lab`], [`oracle`], [`player`], [`ui`], and [`voxel_gpu`].
//! **Why.** Replaces the legacy `camera_rail.rs` playback path with one scene-agnostic
//! [`flight_navigator::fnav_core::Track`] pipeline covering curated rails, live `csg-scene` R0+R1
//! automatic flight planning, manual Flight Lab keyframes, `camera-guide-rs` dynamics tuning,
//! Winamp-style playlists, and `--bench-*` hooks.
//! **How.** [`FlightWorkspace::apply_command`] executes every [`FlightCommand`] against the shared
//! player/lab state and returns [`FlightCommandOutcome`] so `main.rs` performs host side effects
//! (camera damper snap, scene switch, flight-stats reset, MP4 playlist recording) cleanly.
//! **Uses.** `camera`, `flight_navigator`, `glam`, `std`.
//! **Used by.** `crate::main`, `crate::ui`, `crate::app_events`, `crate::persist`, `crate::bench`.

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use camera::CameraPose;
use flight_navigator::fnav_core::{PlaylistTransition, RepeatMode};
use glam::Quat;

/// Conversion from curated [`csg_scene::RailSpec`] / manual keyframes to
/// [`flight_navigator::fnav_core::Track`] and `camera-guide-rs` dynamics analysis.
pub mod convert;
/// Flight Lab authoring state (live R0+R1 Auto Plan, manual keyframes, dynamics tuning, contact
/// sheet).
pub mod lab;
/// Live `csg-scene` R0 (`VoxelVolume`) + R1 (`WorldRayQuery`) [`flight_navigator::fnav_space::SceneOracle`]
/// bridge and asynchronous auto-plan worker.
pub mod oracle;
/// Winamp-style playback, playlist queue, library scanning, and C2 transitions.
pub mod player;
/// Native `egui` dock panels for Flight Lab and Flight Player.
pub mod ui;
/// CUDA GPU column-span baker accelerating `csg-scene` R0 `VoxelVolume` construction.
pub mod voxel_gpu;

/// Shortest-path quaternion slerp ensuring rotation always follows the `<= 180 deg` arc.
#[inline]
fn shortest_slerp(a: Quat, b: Quat, t: f32) -> Quat {
    let b_aligned = if a.dot(b) < 0.0 { -b } else { b };
    a.slerp(b_aligned, t.clamp(0.0, 1.0)).normalize()
}

/// Exponential gimbal camera damper smoothing position, orientation (`slerp`), and vertical FoV.
///
/// Snaps immediately on teleport / preset changes or when `tau <= 1e-4`, and settles cleanly onto
/// the exact target pose once within convergence epsilon so progressive path tracing can converge.
#[derive(Clone, Copy, Debug, Default)]
pub struct CameraDamper {
    smoothed: Option<CameraPose>,
}

impl CameraDamper {
    /// Immediately snaps the smoothed pose to `pose` without damping lag.
    #[inline]
    pub fn snap(&mut self, pose: CameraPose) {
        self.smoothed = Some(pose);
    }

    /// Advances the damped pose toward `target` over `dt` seconds with time constant `tau` seconds.
    pub fn step(&mut self, target: CameraPose, dt: f32, tau: f32) -> CameraPose {
        if !tau.is_finite() || tau <= 1.0e-4 || !dt.is_finite() || dt <= 0.0 {
            self.smoothed = Some(target);
            return target;
        }
        let Some(prev) = self.smoothed else {
            self.smoothed = Some(target);
            return target;
        };

        let alpha = (1.0 - (-dt / tau).exp()).clamp(0.0, 1.0);
        let eye = prev.eye.lerp(target.eye, alpha);
        let orientation = shortest_slerp(prev.orientation, target.orientation, alpha);
        let fov_y = prev.fov_y + (target.fov_y - prev.fov_y) * alpha;

        // Snap once converged so path-tracing accumulation stops resetting on micro-residuals.
        let pos_settled = eye == prev.eye || (eye - target.eye).length_squared() < 1.0e-10;
        let rot_settled = orientation == prev.orientation
            || (1.0 - orientation.dot(target.orientation).abs()) < 1.0e-10;
        let fov_settled = fov_y == prev.fov_y || (fov_y - target.fov_y).abs() < 1.0e-6;
        let next = if pos_settled && rot_settled && fov_settled {
            target
        } else {
            CameraPose {
                eye,
                orientation,
                fov_y,
                znear: target.znear,
                zfar: target.zfar,
            }
        };
        self.smoothed = Some(next);
        next
    }
}

/// Computes the exact simulation time delta for subframe `sub_idx` (`0..sub_count`) of a
/// 180-degree shutter window spanning `0.5 * frame_dt` per output video frame.
///
/// The sum of deltas across `sub_idx = 0..sub_count` equals `frame_dt` identically so long
/// recordings experience zero time drift regardless of `sub_count` (`1`, `2`, or `4`).
#[inline]
#[must_use]
pub fn shutter_subframe_dt(frame_dt: f32, sub_idx: u32, sub_count: u32) -> f32 {
    if !frame_dt.is_finite() || frame_dt <= 0.0 {
        return 0.0;
    }
    let n = sub_count.max(1);
    if n == 1 {
        return frame_dt;
    }
    let sub_dt = (0.5 * frame_dt) / (n as f32);
    if sub_idx == 0 {
        // Jump across the closed-shutter dark interval from the previous frame's last subframe.
        frame_dt - sub_dt * ((n - 1) as f32)
    } else {
        sub_dt
    }
}

/// Typed flight commands emitted by Flight Lab and Flight Player UI panels and routed via
/// `UiCommand::Flight(FlightCommand)`.
#[derive(Clone, Debug, PartialEq)]
pub enum FlightCommand {
    /// Start or resume playback of the active playlist slot.
    Play,
    /// Pause playback at the current timestamp.
    Pause,
    /// Stop playback and rewind to `0.0 s`.
    Stop,
    /// Jump to previous playlist entry (or rewind current entry if past `2.0 s`).
    Prev,
    /// Advance to next playlist entry.
    Next,
    /// Seek active track to `f64` timestamp in seconds.
    Seek(f64),
    /// Select playlist entry `usize` and begin playing it.
    SelectPlaylistEntry(usize),
    /// Remove playlist entry `usize` from the queue.
    RemovePlaylistEntry(usize),
    /// Move playlist entry from index `from` to index `to`.
    ReorderPlaylist {
        /// Original queue slot index.
        from: usize,
        /// Target queue slot index.
        to: usize,
    },
    /// Set entry transition style for queue slot `idx`.
    SetEntryTransition {
        /// Queue slot index.
        idx: usize,
        /// Desired transition into this entry.
        transition: PlaylistTransition,
    },
    /// Set final-frame hold duration (seconds) for queue slot `idx`.
    SetEntryHold {
        /// Queue slot index.
        idx: usize,
        /// Hold time in seconds (`>= 0.0`).
        hold_s: f32,
    },
    /// Set playlist repetition mode (`Off`, `All`, `One`).
    SetRepeat(RepeatMode),
    /// Enable or disable deterministic playlist shuffle.
    SetShuffle(bool),
    /// Play scene-curated rail `usize` immediately.
    PlayCuratedRail(usize),
    /// Append scene-curated rail `usize` to the playlist queue.
    EnqueueCuratedRail(usize),
    /// Load and play a `*.fnav.json` track file from disk.
    PlayLibraryTrack(PathBuf),
    /// Load and append a `*.fnav.json` track file to the playlist queue.
    EnqueueLibraryTrack(PathBuf),
    /// Load a `*.fnav-playlist.json` file into the queue.
    LoadPlaylistFile(PathBuf),
    /// Save the current playlist queue to either repo (`to_repo = true`) or user library.
    SaveCurrentPlaylist {
        /// Operator-facing playlist name.
        name: String,
        /// Write to `assets/flights` if `true`, else `~/.nv-rs/flights`.
        to_repo: bool,
    },
    /// Rescan repo and user flight libraries from disk.
    RescanLibrary,
    /// Toggle full-playlist MP4 video recording.
    ToggleRecordPlaylist,
    /// Launch asynchronous automatic flight planning against the active CSG scene (R0+R1).
    LabGenerateAutoPlan,
    /// Capture current viewport camera pose into Flight Lab manual keyframes.
    LabAddKeyframe,
    /// Remove the last manual keyframe from Flight Lab.
    LabPopKeyframe,
    /// Clear all manual keyframes in Flight Lab.
    LabClearKeyframes,
    /// Compile manual keyframes through `camera-guide-rs` into a preview track.
    LabBuildManualTrack,
    /// Load the Flight Lab preview track into Flight Player and start playing it.
    LabPreviewTrack,
    /// Re-time the active Flight Player track (or preview track) with Flight Lab's current limits.
    LabApplyLimitsToActive,
    /// Atomically save the Flight Lab preview track to repo (`to_repo = true`) or user library.
    LabSaveTrack {
        /// Operator-facing track name.
        name: String,
        /// Write to `assets/flights` if `true`, else `~/.nv-rs/flights`.
        to_repo: bool,
    },
    /// Switch Flight Player camera rig mode (`Cinematic` / `AlongPath` / `Track` / `Free`).
    SetRigMode(player::FlightRigCameraMode),
    /// Select the named scene landmark tracked by `RailRig` in `LookMode::Track`.
    SetRigTrackLandmark(String),
    /// Update `csg_scene::DroneTilt` filter settings (`max_tilt_deg`, `tau_s`).
    SetRigTilt(csg_scene::DroneTilt),
    /// Eject cleanly from `RailRig` into free camera control at the rig's current pose.
    RigEject,
}

/// Host side effects requested after [`FlightWorkspace::apply_command`] updates state.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlightCommandOutcome {
    /// Snap `CameraDamper` immediately onto the evaluated pose (seek, cut, track switch).
    pub snap_cam: bool,
    /// Explicit camera pose to snap the host camera to when ejecting from `RailRig`.
    pub ejected_pose: Option<CameraPose>,
    /// Reset `FlightStats` telemetry ring buffer (new track / playback start).
    pub reset_stats: bool,
    /// Toggle host MP4 video recording for a full-playlist capture pass.
    pub toggle_record_playlist: bool,
    /// Request that the host switch to the named scene id before playback resumes.
    pub request_scene_id: Option<String>,
    /// Request that the host submit an asynchronous [`oracle::AutoPlanJob`] for the active CSG scene.
    pub request_auto_plan: bool,
}

/// Single source of truth shared between the **Flight Player** and **Flight Lab** dock panels.
#[derive(Clone, Debug, Default)]
pub struct FlightWorkspace {
    /// Winamp-style playlist player and library index.
    pub player: player::FlightPlayer,
    /// Flight Lab manual keyframe authoring and live `camera-guide-rs` dynamics state.
    pub lab: lab::FlightLabState,
}

impl FlightWorkspace {
    /// Creates a new [`FlightWorkspace`] backed by `repo_dir` and `user_dir`.
    #[must_use]
    pub fn new(repo_dir: PathBuf, user_dir: PathBuf) -> Self {
        Self {
            player: player::FlightPlayer::new(repo_dir, user_dir),
            lab: lab::FlightLabState::new(),
        }
    }

    /// Executes one [`FlightCommand`] against the shared workspace and returns host side-effect flags.
    pub fn apply_command(
        &mut self,
        cmd: FlightCommand,
        cur_pose: CameraPose,
    ) -> FlightCommandOutcome {
        let mut out = FlightCommandOutcome::default();
        match cmd {
            FlightCommand::Play => {
                if self.player.play() {
                    out.reset_stats = true;
                    if let player::PlayerState::Loading { ref scene_id } = self.player.state {
                        out.request_scene_id = Some(scene_id.clone());
                    }
                }
            }
            FlightCommand::Pause => self.player.pause(),
            FlightCommand::Stop => self.player.stop(),
            FlightCommand::Prev => {
                if self.player.prev() {
                    out.snap_cam = true;
                    out.reset_stats = true;
                }
            }
            FlightCommand::Next => {
                if self.player.next() {
                    out.snap_cam = true;
                    out.reset_stats = true;
                }
            }
            FlightCommand::Seek(t_s) => {
                self.player.seek(t_s);
                out.snap_cam = true;
            }
            FlightCommand::SelectPlaylistEntry(idx) => {
                if self.player.select_entry(idx, true) {
                    out.snap_cam = true;
                    out.reset_stats = true;
                    if let player::PlayerState::Loading { ref scene_id } = self.player.state {
                        out.request_scene_id = Some(scene_id.clone());
                    }
                }
            }
            FlightCommand::RemovePlaylistEntry(idx) => self.player.remove(idx),
            FlightCommand::ReorderPlaylist { from, to } => self.player.reorder(from, to),
            FlightCommand::SetEntryTransition { idx, transition } => {
                self.player.set_entry_transition(idx, transition);
            }
            FlightCommand::SetEntryHold { idx, hold_s } => {
                self.player.set_entry_hold(idx, hold_s);
            }
            FlightCommand::SetRepeat(mode) => self.player.set_repeat(mode),
            FlightCommand::SetShuffle(on) => self.player.set_shuffle(on),
            FlightCommand::PlayCuratedRail(idx) => {
                if self.player.play_curated(idx) {
                    out.snap_cam = true;
                    out.reset_stats = true;
                }
            }
            FlightCommand::EnqueueCuratedRail(idx) => {
                let _ = self.player.enqueue_curated(idx);
            }
            FlightCommand::PlayLibraryTrack(path) => match self.player.load_and_play_track(&path) {
                Ok(_) => {
                    out.snap_cam = true;
                    out.reset_stats = true;
                    if let player::PlayerState::Loading { ref scene_id } = self.player.state {
                        out.request_scene_id = Some(scene_id.clone());
                    }
                }
                Err(err) => {
                    self.player.status_msg = format!("Failed to play track: {err}");
                }
            },
            FlightCommand::EnqueueLibraryTrack(path) => {
                if let Err(err) = self.player.load_and_enqueue_track(&path) {
                    self.player.status_msg = format!("Failed to enqueue track: {err}");
                }
            }
            FlightCommand::LoadPlaylistFile(path) => match self.player.load_playlist(&path) {
                Ok(_) => {
                    self.player.play();
                    out.snap_cam = true;
                    out.reset_stats = true;
                    if let player::PlayerState::Loading { ref scene_id } = self.player.state {
                        out.request_scene_id = Some(scene_id.clone());
                    }
                }
                Err(err) => {
                    self.player.status_msg = format!("Failed to load playlist: {err}");
                }
            },
            FlightCommand::SaveCurrentPlaylist { name, to_repo } => {
                if let Err(err) = self.player.save_playlist(&name, to_repo) {
                    self.player.status_msg = format!("Failed to save playlist: {err}");
                }
            }
            FlightCommand::RescanLibrary => {
                self.player.rescan_library();
                self.player.status_msg = format!(
                    "Scanned {} tracks, {} playlists",
                    self.player.library.tracks.len(),
                    self.player.library.playlists.len()
                );
            }
            FlightCommand::ToggleRecordPlaylist => {
                return self.toggle_record_playlist(false);
            }
            FlightCommand::LabGenerateAutoPlan => {
                out.request_auto_plan = true;
            }
            FlightCommand::LabAddKeyframe => self.lab.push_pose(cur_pose),
            FlightCommand::LabPopKeyframe => self.lab.pop_keyframe(),
            FlightCommand::LabClearKeyframes => self.lab.clear_keyframes(),
            FlightCommand::LabBuildManualTrack => {
                if let Err(err) = self.lab.build_manual_track() {
                    self.lab.status_msg = format!("Build failed: {err}");
                }
            }
            FlightCommand::LabPreviewTrack => {
                if let Some(track) = self.lab.preview_track.clone() {
                    let title = if self.lab.track_name.trim().is_empty() {
                        "Lab Preview".to_string()
                    } else {
                        self.lab.track_name.trim().to_string()
                    };
                    self.player
                        .preview_lab_track(title, track, self.lab.preview_plan.clone());
                    out.snap_cam = true;
                    out.reset_stats = true;
                } else {
                    self.lab.status_msg = "Build a track before previewing".to_string();
                }
            }
            FlightCommand::LabApplyLimitsToActive => {
                let source = self
                    .player
                    .active_track()
                    .cloned()
                    .or_else(|| self.lab.preview_track.clone());
                match source {
                    Some(track) => match self.lab.retime_track(&track) {
                        Ok(retimed) => {
                            if self.player.active_track().is_some()
                                && let Err(err) = self.player.replace_active_track(retimed)
                            {
                                self.lab.status_msg = format!("Apply failed: {err}");
                            }
                        }
                        Err(err) => {
                            self.lab.status_msg = format!("Dynamics solve failed: {err}");
                        }
                    },
                    None => {
                        self.lab.status_msg = "No active or preview track to re-time".to_string();
                    }
                }
            }
            FlightCommand::LabSaveTrack { name, to_repo } => {
                let dir = if to_repo {
                    self.player.repo_dir.clone()
                } else {
                    self.player.user_dir.clone()
                };
                match self.lab.save_track_to_dir(
                    &dir,
                    &name,
                    &self.player.active_scene_id,
                    &self.player.active_scene_hash,
                ) {
                    Ok(_) => self.player.rescan_library(),
                    Err(err) => {
                        self.lab.status_msg = format!("Save failed: {err}");
                    }
                }
            }
            FlightCommand::SetRigMode(mode) => {
                self.player.set_rig_mode(mode);
            }
            FlightCommand::SetRigTrackLandmark(name) => {
                self.player.set_rig_track_landmark(name);
            }
            FlightCommand::SetRigTilt(tilt) => {
                self.player.set_rig_tilt(tilt);
            }
            FlightCommand::RigEject => {
                out.ejected_pose = self.player.eject_rig();
                out.snap_cam = true;
            }
        }
        out
    }

    /// Toggles playlist MP4 recording while respecting both active host recording (`host_recording`,
    /// e.g. started from `Render & Floor -> Record MP4`) and the currently active/previewed track.
    ///
    /// Why: when recording is already active, clicking Flight Player's red Record button must stop
    /// recording without interrupting or resetting the active rail; when starting a pass, it must
    /// record the currently active/previewed track (such as a `LabPreview` rail appended after a
    /// curated rail) rather than jumping back to slot `0`, seed curated rail `0` when the queue is
    /// empty, and forward cross-scene `PlayerState::Loading` requests to the host.
    pub fn toggle_record_playlist(&mut self, host_recording: bool) -> FlightCommandOutcome {
        let mut out = FlightCommandOutcome::default();
        if host_recording || self.player.recording_playlist {
            self.player.recording_playlist = false;
            out.toggle_record_playlist = true;
            return out;
        }

        if self.player.queue.is_empty() {
            if !self.player.play() {
                return out;
            }
            self.player.recording_playlist = true;
            out.snap_cam = true;
            out.reset_stats = true;
            out.toggle_record_playlist = true;
            if let player::PlayerState::Loading { ref scene_id } = self.player.state {
                out.request_scene_id = Some(scene_id.clone());
            }
            return out;
        }

        let start_idx = match self.player.state {
            player::PlayerState::Playing { entry_idx }
            | player::PlayerState::Paused { entry_idx } => {
                entry_idx.min(self.player.queue.len() - 1)
            }
            player::PlayerState::Transition { to_idx, .. } => {
                to_idx.min(self.player.queue.len() - 1)
            }
            player::PlayerState::Loading { .. } => {
                self.player.active_idx.min(self.player.queue.len() - 1)
            }
            player::PlayerState::Idle => {
                let dur = self.player.duration_s();
                let finished_pass = dur > 0.0 && self.player.time_s >= dur - 1.0e-5;
                if finished_pass {
                    0
                } else {
                    self.player.active_idx.min(self.player.queue.len() - 1)
                }
            }
        };

        if self.player.select_entry(start_idx, true) {
            self.player.recording_playlist = true;
            out.snap_cam = true;
            out.reset_stats = true;
            out.toggle_record_playlist = true;
            if let player::PlayerState::Loading { ref scene_id } = self.player.state {
                out.request_scene_id = Some(scene_id.clone());
            }
        }
        out
    }

    /// Applies one asynchronous [`oracle::AutoPlanEvent`] emitted by [`oracle::CsgAutoPlanner`]
    /// into Flight Lab state.
    ///
    /// Why: keeping event reduction inside [`FlightWorkspace`] keeps `main.rs` free of Lab state
    /// mutation details and makes stage/ready/error transitions directly unit-testable.
    ///
    /// Returns `true` when a newly generated [`flight_navigator::fnav_plan::RailPlan`] became
    /// ready and was installed into [`Self::lab`], `false` for stage progress or error updates.
    pub fn apply_auto_plan_event(&mut self, ev: oracle::AutoPlanEvent) -> bool {
        match ev {
            oracle::AutoPlanEvent::Stage(stage) => {
                self.lab.auto_plan_stage = stage;
                self.lab.status_msg = stage.label().to_string();
                false
            }
            oracle::AutoPlanEvent::Ready {
                scene_id,
                plan,
                elapsed_ms,
            } => {
                self.lab.apply_auto_plan(scene_id, *plan, elapsed_ms);
                true
            }
            oracle::AutoPlanEvent::Failed(err) => {
                self.lab.auto_plan_stage = oracle::AutoPlanStage::Idle;
                self.lab.status_msg = format!("Auto-plan failed: {err}");
                false
            }
        }
    }

    /// Returns the active [`flight_navigator::fnav_core::Track`] and optional
    /// [`flight_navigator::fnav_plan::RailPlan`] currently loaded in Flight Player or previewed in
    /// Flight Lab, for rendering the 3D viewport `FlightPath` overlay.
    ///
    /// Why: provides one authoritative overlay source so actively playing, paused, or
    /// transitioning tracks in Flight Player take precedence without leaking an unrelated Lab
    /// plan, while idle sessions still visualize a freshly planned or authored Flight Lab track
    /// before pressing Play.
    #[must_use]
    pub fn active_overlay_source(
        &self,
    ) -> Option<(
        &flight_navigator::fnav_core::Track,
        Option<&flight_navigator::fnav_plan::RailPlan>,
    )> {
        let active_slot = match self.player.state {
            player::PlayerState::Playing { entry_idx }
            | player::PlayerState::Paused { entry_idx } => self.player.queue.get(entry_idx),
            player::PlayerState::Transition { to_idx, .. } => self.player.queue.get(to_idx),
            player::PlayerState::Idle | player::PlayerState::Loading { .. } => None,
        };
        if let Some(slot) = active_slot {
            let plan = slot.plan.as_ref().or_else(|| {
                let lab_plan = self.lab.preview_plan.as_ref()?;
                (slot.origin == player::TrackOrigin::LabPreview
                    && lab_plan.track.canonical_hash() == slot.track.canonical_hash())
                .then_some(lab_plan)
            });
            return Some((&slot.track, plan));
        }
        self.lab
            .preview_track
            .as_ref()
            .map(|track| (track, self.lab.preview_plan.as_ref()))
    }
}

/// Resolves the repository flight asset directory (`assets/flights`), checking the current
/// working directory first and falling back to `<CARGO_MANIFEST_DIR>/../../assets/flights`.
#[must_use]
pub fn default_repo_flights_dir() -> PathBuf {
    let cwd_assets = PathBuf::from("assets").join("flights");
    if cwd_assets.exists() || Path::new("assets").exists() {
        return cwd_assets;
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("assets")
        .join("flights")
}

/// Resolves the user-local flight asset directory (`~/.nv-rs/flights`).
#[must_use]
pub fn default_user_flights_dir() -> PathBuf {
    let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"));
    match home {
        Some(dir) if !dir.is_empty() => PathBuf::from(dir).join(".nv-rs").join("flights"),
        _ => PathBuf::from(".nv-rs").join("flights"),
    }
}

/// Produces a clean ASCII filename stem safe on Windows and POSIX filesystems.
#[must_use]
pub(crate) fn sanitize_file_stem(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
            out.push(ch.to_ascii_lowercase());
        } else if ch.is_whitespace() && !out.ends_with('_') {
            out.push('_');
        }
    }
    let trimmed = out.trim_matches('_');
    if trimmed.is_empty() {
        "untitled".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Formats the current wall-clock UTC time as `YYYY-MM-DDTHH:MM:SSZ` using only `std`.
#[must_use]
pub(crate) fn utc_now_iso8601() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    let hour = rem / 3600;
    let min = (rem % 3600) / 60;
    let sec = rem % 60;
    // Howard Hinnant's civil-from-days algorithm (proleptic Gregorian calendar).
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = y + i64::from(m <= 2);
    format!("{year:04}-{m:02}-{d:02}T{hour:02}:{min:02}:{sec:02}Z")
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    use csg_scene::{CompiledRail, RailSpec, RailWaypoint};
    use flight_navigator::fnav_core::{
        Aim, CameraState, Cut, CutEvidence, CutKind, FilmBack, Kinematic, Lens, PlaylistTransition,
        RepeatMode, Segment, Track, Transition,
    };
    use glam::Vec3;

    use super::*;
    use crate::flight::convert::{analyze_and_retime_track, curated_rail_to_track};
    use crate::flight::player::{FlightPlayer, PlayerState, PlaylistSlot, TrackOrigin};

    static TEST_SEQ: AtomicU64 = AtomicU64::new(0);

    fn temp_dir(tag: &str) -> PathBuf {
        let seq = TEST_SEQ.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "nv_demo_flight_{tag}_{}_{seq}",
            std::process::id()
        ));
        let _ = fs::create_dir_all(&dir);
        dir
    }

    fn sample_pose(x: f32, y: f32, z: f32) -> CameraPose {
        CameraPose::from_eye_forward_up(
            Vec3::new(x, y, z),
            Vec3::NEG_Z,
            Vec3::Y,
            45.0_f32.to_radians(),
            0.05,
            2000.0,
        )
    }

    fn sample_linear_track(x0: f32, x1: f32, duration_s: f64, looping: bool) -> Track {
        let poses = vec![
            (
                Vec3::new(x0, 5.0, 0.0),
                Quat::IDENTITY,
                45.0_f32.to_radians(),
            ),
            (
                Vec3::new(0.5 * (x0 + x1), 5.0, 8.0),
                Quat::IDENTITY,
                45.0_f32.to_radians(),
            ),
            (
                Vec3::new(x1, 5.0, 0.0),
                Quat::IDENTITY,
                45.0_f32.to_radians(),
            ),
        ];
        Track::from_keyframe_poses(&poses, duration_s, looping, FilmBack::default())
    }

    #[test]
    fn camera_damper_snaps_converges_and_bypasses_when_tau_zero() {
        let mut damper = CameraDamper::default();
        let a = sample_pose(0.0, 2.0, 10.0);
        let b = sample_pose(20.0, 8.0, -5.0);

        // First step snaps immediately.
        assert_eq!(damper.step(a, 1.0 / 60.0, 0.2), a);
        // Zero tau bypasses damping.
        assert_eq!(damper.step(b, 1.0 / 60.0, 0.0), b);

        damper.snap(a);
        let mid = damper.step(b, 0.1, 0.25);
        assert!(mid.eye.x > a.eye.x && mid.eye.x < b.eye.x);

        let mut cur = mid;
        for _ in 0..400 {
            cur = damper.step(b, 1.0 / 60.0, 0.12);
        }
        assert_eq!(cur.eye, b.eye, "damper must settle identically onto target");
    }

    #[test]
    fn shutter_subframe_dt_sums_identically_to_frame_dt() {
        let frame_dt = 1.0_f32 / 60.0;
        for sub_count in [1u32, 2, 4, 8] {
            let sum: f32 = (0..sub_count)
                .map(|i| shutter_subframe_dt(frame_dt, i, sub_count))
                .sum();
            assert!(
                (sum - frame_dt).abs() < 1.0e-7,
                "drift for sub_count={sub_count}: {sum} vs {frame_dt}"
            );
        }
        assert_eq!(shutter_subframe_dt(0.0, 0, 4), 0.0);
        assert_eq!(shutter_subframe_dt(f32::NAN, 0, 4), 0.0);
    }

    #[test]
    fn curated_rail_to_track_matches_compiled_rail_within_1cm_and_01deg() {
        let mut wps = Vec::new();
        let count = 12usize;
        let radius = 60.0_f32;
        for i in 0..count {
            let a = std::f32::consts::TAU * (i as f32) / (count as f32);
            let eye = csg_core::Vec3::new(
                radius * a.cos(),
                12.0 + 3.0 * (2.0 * a).sin(),
                radius * a.sin(),
            );
            let tgt = csg_core::Vec3::new(0.0, 6.0, 0.0);
            let vfov = 48.0 + 6.0 * a.cos();
            wps.push(RailWaypoint::new(eye, tgt, vfov));
        }

        for looping in [true, false] {
            let spec = RailSpec::new(
                "test_orbit",
                "Test Orbit",
                "Curated parity test",
                wps.clone(),
                24.0,
            )
            .with_looping(looping);

            let compiled = CompiledRail::compile(spec.clone());
            let track = curated_rail_to_track(&spec).expect("curated_rail_to_track should succeed");
            let dur = track.duration_s();
            assert!((dur - 24.0).abs() < 1.0e-6);

            let mut max_pos_err = 0.0_f32;
            let mut max_interior_pos_err = 0.0_f32;
            let mut max_pos_i = 0usize;
            let mut max_pos_got = Vec3::ZERO;
            let mut max_pos_exp = Vec3::ZERO;
            let mut max_interior_ang_err_deg = 0.0_f32;
            let mut max_fwd_err_deg = 0.0_f32;
            let mut max_fov_err_deg = 0.0_f32;
            let steps = 120usize;
            for i in 0..steps {
                let frac = (i as f64) / (steps as f64);
                let t = frac * dur;
                let got = track.eval(t).state.to_camera_pose(0.05, 5000.0);
                let exp_smp = compiled.eval_at_fraction(frac as f32);
                let exp_eye = Vec3::new(exp_smp.eye.x, exp_smp.eye.y, exp_smp.eye.z);
                let exp_fwd = Vec3::new(exp_smp.forward.x, exp_smp.forward.y, exp_smp.forward.z);
                let exp_up = Vec3::new(exp_smp.up.x, exp_smp.up.y, exp_smp.up.z);
                let exp_pose = CameraPose::from_eye_forward_up(
                    exp_eye,
                    exp_fwd,
                    exp_up,
                    exp_smp.vfov_deg.to_radians(),
                    0.05,
                    5000.0,
                );

                let pos_err = got.eye.distance(exp_pose.eye);
                let dot = got.orientation.dot(exp_pose.orientation).abs().clamp(0.0, 1.0);
                let ang_err_deg = (2.0 * dot.acos()).to_degrees();
                let fwd_dot = got
                    .forward()
                    .dot(exp_pose.forward())
                    .clamp(-1.0, 1.0);
                let fwd_err_deg = fwd_dot.acos().to_degrees();
                let fov_err_deg = (got.fov_y - exp_pose.fov_y).abs().to_degrees();

                if pos_err > max_pos_err {
                    max_pos_err = pos_err;
                    max_pos_i = i;
                    max_pos_got = got.eye;
                    max_pos_exp = exp_pose.eye;
                }
                if looping || (0.15..=0.85).contains(&frac) {
                    max_interior_pos_err = max_interior_pos_err.max(pos_err);
                    max_interior_ang_err_deg = max_interior_ang_err_deg.max(ang_err_deg);
                }
                max_fwd_err_deg = max_fwd_err_deg.max(fwd_err_deg);
                max_fov_err_deg = max_fov_err_deg.max(fov_err_deg);
            }

            assert!(
                max_interior_pos_err < 0.01,
                "looping={looping}: interior position error {max_interior_pos_err:.5} m exceeded 1 cm"
            );
            // Note: on open (`looping=false`) rails in pinned `csg-rs#697a203e`,
            // `centripetal_catmull_rom` clamps `p0==p1` and `p2==p3` with `d = 1e-4`, introducing
            // ~0.047 m `f32` cancellation jitter and legacy `compute_bank_rad_single` roll spikes
            // inside the clamped first/last segments (`frac < 0.15` or `frac > 0.85`).
            let pos_tol = if looping { 0.01 } else { 0.06 };
            assert!(
                max_pos_err < pos_tol,
                "looping={looping}: position error {max_pos_err:.5} m exceeded {pos_tol} m at step {max_pos_i}/{steps} (got={max_pos_got:?}, exp={max_pos_exp:?})"
            );
            assert!(
                max_interior_ang_err_deg < 0.1,
                "looping={looping}: interior angle error {max_interior_ang_err_deg:.4} deg exceeded 0.1 deg"
            );
            let fwd_tol = if looping { 0.1 } else { 1.0 };
            assert!(
                max_fwd_err_deg < fwd_tol,
                "looping={looping}: forward-direction error {max_fwd_err_deg:.4} deg exceeded {fwd_tol} deg"
            );
            assert!(
                max_fov_err_deg < 0.1,
                "looping={looping}: fov error {max_fov_err_deg:.4} deg exceeded 0.1 deg"
            );
        }
    }

    #[test]
    fn flight_player_transport_cut_detection_scene_fade_and_playlist_roundtrip() {
        let dir = temp_dir("player_roundtrip");
        let repo_dir = dir.join("repo");
        let user_dir = dir.join("user");
        let _ = fs::create_dir_all(&repo_dir);
        let _ = fs::create_dir_all(&user_dir);

        // Build a track with an explicit internal `Segment::Cut` at t = 2.0 s.
        let k0 = Kinematic {
            state: CameraState::from_look_at(
                Vec3::new(0.0, 5.0, 20.0),
                Vec3::ZERO,
                Vec3::Y,
                Lens::pinhole(35.0),
            ),
            ..Kinematic::default()
        };
        let k1 = Kinematic {
            state: CameraState::from_look_at(
                Vec3::new(15.0, 5.0, 20.0),
                Vec3::ZERO,
                Vec3::Y,
                Lens::pinhole(35.0),
            ),
            ..Kinematic::default()
        };
        let k2 = Kinematic {
            state: CameraState::from_look_at(
                Vec3::new(-25.0, 12.0, -10.0),
                Vec3::ZERO,
                Vec3::Y,
                Lens::pinhole(85.0),
            ),
            ..Kinematic::default()
        };
        let k3 = Kinematic {
            state: CameraState::from_look_at(
                Vec3::new(-10.0, 12.0, -10.0),
                Vec3::ZERO,
                Vec3::Y,
                Lens::pinhole(85.0),
            ),
            ..Kinematic::default()
        };
        let cut_track = Track {
            segments: vec![
                Segment::Transition(Transition::direct(k0, k1, 2.0, Aim::Squad)),
                Segment::Cut(Cut {
                    kind: CutKind::Contrast,
                    score: 0.9,
                    evidence: CutEvidence {
                        metric_a: 45.0,
                        metric_b: 2.4,
                        note: "test contrast cut".to_string(),
                    },
                }),
                Segment::Transition(Transition::direct(k2, k3, 2.0, Aim::Squad)),
            ],
            looping: false,
        };

        let mut player = FlightPlayer::new(repo_dir.clone(), user_dir.clone());
        player.active_scene_id = "scene_a".to_string();
        player.active_scene_hash = "hash_a".to_string();
        player.queue.push(PlaylistSlot {
            title: "Cut Track".to_string(),
            description: "Track with cut".to_string(),
            scene_id: "scene_a".to_string(),
            track_id: "cut_track_1".to_string(),
            file_path: None,
            track: cut_track,
            plan: None,
            transition_in: PlaylistTransition::Cut,
            hold_s: 0.0,
            stale: false,
            origin: TrackOrigin::LabPreview,
        });
        player.queue.push(PlaylistSlot {
            title: "Other Scene Track".to_string(),
            description: "Scene B track".to_string(),
            scene_id: "scene_b".to_string(),
            track_id: "scene_b_track".to_string(),
            file_path: None,
            track: sample_linear_track(0.0, 40.0, 4.0, false),
            plan: None,
            // Request FlyThrough across scenes: player MUST force FadeBlack!
            transition_in: PlaylistTransition::FlyThrough,
            hold_s: 0.0,
            stale: false,
            origin: TrackOrigin::LabPreview,
        });
        player.set_repeat(RepeatMode::Off);
        player.set_shuffle(false);

        // Verify cross-scene FadeBlack forcing.
        assert!(matches!(
            player.effective_transition_between(0, 1),
            PlaylistTransition::FadeBlack { .. }
        ));

        // Play through the cut at t = 2.0 s and verify `cut_before` fires once.
        assert!(player.play());
        let mut cut_hits = 0usize;
        for _ in 0..38 {
            let ev = player.tick(0.1, 0.05, 5000.0).expect("playing");
            if ev.cut_before {
                cut_hits += 1;
            }
        }
        assert_eq!(cut_hits, 1, "internal cut at t=2.0s must trigger exactly once");

        // Advance past end of slot 0 (4.0 s) -> enters FadeBlack transition -> hits midpoint -> Loading scene_b.
        let mut saw_loading = false;
        for _ in 0..30 {
            if let Some(ev) = player.tick(0.1, 0.05, 5000.0)
                && ev.request_scene_id.as_deref() == Some("scene_b")
            {
                saw_loading = true;
                assert!((ev.fade_alpha - 1.0).abs() < 1e-5);
                break;
            }
        }
        assert!(saw_loading, "cross-scene switch must emit request_scene_id at black midpoint");
        assert!(matches!(player.state, PlayerState::Loading { .. }));

        // Notify host loaded `scene_b` -> fades back in and enters Playing slot 1.
        player.notify_scene_loaded("scene_b", "hash_b");
        for _ in 0..10 {
            let _ = player.tick(0.1, 0.05, 5000.0);
        }
        assert_eq!(player.state, PlayerState::Playing { entry_idx: 1 });

        // Pause / Stop / Save Playlist + Reload roundtrip.
        player.pause();
        assert!(matches!(player.state, PlayerState::Paused { entry_idx: 1 }));
        player.stop();
        assert_eq!(player.state, PlayerState::Idle);

        let pl_path = player
            .save_playlist("Roundtrip Demo", false)
            .expect("save_playlist");
        assert!(pl_path.is_file());

        let mut player2 = FlightPlayer::new(repo_dir, user_dir);
        let loaded_n = player2.load_playlist(&pl_path).expect("load_playlist");
        assert_eq!(loaded_n, 2);
        assert_eq!(player2.queue[0].title, "Cut Track");
        assert_eq!(player2.queue[1].scene_id, "scene_b");

        // Verify deterministic shuffle reproducibility.
        player2.shuffle_seed = 777;
        player2.set_shuffle(true);
        let first_next = player2.next();
        assert!(first_next);

        // Verify `analyze_and_retime_track` produces valid dynamics plot.
        let (_, plot) = analyze_and_retime_track(
            &sample_linear_track(0.0, 80.0, 12.0, false),
            &lab::default_lab_limits(),
        )
        .expect("analyze_and_retime_track");
        assert!(!plot.s.is_empty());
        assert!(plot.report.worst() <= 1.0 + flight_navigator::camera_guide::RATIO_TOL);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn committed_repo_flight_assets_load_and_verify() {
        use flight_navigator::fnav_core::{FlightLibrary, PlaylistFile, TrackFile};

        let repo_dir = default_repo_flights_dir();
        let track_path = repo_dir.join("gundo_demo.fnav.json");
        let playlist_path = repo_dir.join("demo.fnav-playlist.json");

        assert!(
            track_path.is_file(),
            "committed demo track missing at {}",
            track_path.display()
        );
        assert!(
            playlist_path.is_file(),
            "committed demo playlist missing at {}",
            playlist_path.display()
        );

        let lib = FlightLibrary::scan(Some(&repo_dir), None, None, None);
        assert!(
            lib.load_errors.is_empty(),
            "repo flight library must have 0 load_errors, got: {:?}",
            lib.load_errors
        );
        assert!(
            !lib.tracks.is_empty(),
            "repo flight library must discover at least 1 track in {}",
            repo_dir.display()
        );
        assert!(
            !lib.playlists.is_empty(),
            "repo flight library must discover at least 1 playlist in {}",
            repo_dir.display()
        );

        let tf = TrackFile::<serde_json::Value>::load(&track_path)
            .expect("gundo_demo.fnav.json must load and pass BLAKE3 verification");
        assert_eq!(tf.format_version, 1);
        assert_eq!(tf.scene.id, "gundo");
        assert!(tf.track.duration_s() > 10.0);

        let pl = PlaylistFile::load(&playlist_path)
            .expect("demo.fnav-playlist.json must load and verify schema version");
        assert_eq!(pl.format_version, 1);
        assert_eq!(pl.entries.len(), 1);
        assert_eq!(pl.entries[0].track.id, tf.id);
        assert_eq!(pl.entries[0].track.file_hint, "gundo_demo.fnav.json");

        let user_dir = temp_dir("committed_assets_player");
        let mut player = FlightPlayer::new(repo_dir, user_dir.clone());
        let loaded = player
            .load_playlist(&playlist_path)
            .expect("FlightPlayer must load committed demo playlist");
        assert_eq!(loaded, 1);
        assert_eq!(player.queue[0].scene_id, "gundo");
        assert_eq!(player.queue[0].track_id, tf.id);
        let _ = fs::remove_dir_all(&user_dir);
    }

    #[test]
    fn workspace_auto_plan_command_and_stage_event_transitions() {
        let dir = temp_dir("auto_plan_events");
        let mut ws = FlightWorkspace::new(dir.join("repo"), dir.join("user"));
        let outcome = ws.apply_command(FlightCommand::LabGenerateAutoPlan, sample_pose(0.0, 5.0, 0.0));
        assert!(outcome.request_auto_plan);
        assert!(!outcome.snap_cam);

        assert!(!ws.apply_auto_plan_event(oracle::AutoPlanEvent::Stage(
            oracle::AutoPlanStage::BakingVolume
        )));
        assert_eq!(ws.lab.auto_plan_stage, oracle::AutoPlanStage::BakingVolume);
        assert_eq!(ws.lab.status_msg, oracle::AutoPlanStage::BakingVolume.label());

        assert!(!ws.apply_auto_plan_event(oracle::AutoPlanEvent::Failed(
            "no valid land".to_string()
        )));
        assert_eq!(ws.lab.auto_plan_stage, oracle::AutoPlanStage::Idle);
        assert_eq!(ws.lab.status_msg, "Auto-plan failed: no valid land");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn active_overlay_source_prefers_active_player_then_lab_preview() {
        let dir = temp_dir("overlay_source");
        let mut ws = FlightWorkspace::new(dir.join("repo"), dir.join("user"));
        assert!(
            ws.active_overlay_source().is_none(),
            "empty workspace must report no active overlay source"
        );

        let lab_track = sample_linear_track(0.0, 20.0, 6.0, false);
        let player_track = sample_linear_track(-30.0, -5.0, 8.0, false);
        ws.lab.preview_track = Some(lab_track.clone());

        let (t_idle, plan_idle) = ws
            .active_overlay_source()
            .expect("idle workspace must fall back to lab preview track");
        assert_eq!(t_idle.canonical_hash(), lab_track.canonical_hash());
        assert!(plan_idle.is_none());

        ws.player.queue.push(PlaylistSlot {
            title: "Curated".to_string(),
            description: String::new(),
            scene_id: String::new(),
            track_id: "curated_0".to_string(),
            file_path: None,
            track: player_track.clone(),
            plan: None,
            transition_in: PlaylistTransition::Cut,
            hold_s: 0.0,
            stale: false,
            origin: TrackOrigin::Curated,
        });
        assert!(ws.player.play());
        let (t_play, plan_play) = ws
            .active_overlay_source()
            .expect("playing player track must take precedence");
        assert_eq!(t_play.canonical_hash(), player_track.canonical_hash());
        assert!(plan_play.is_none());

        ws.player.pause();
        let (t_pause, _) = ws
            .active_overlay_source()
            .expect("paused player track must remain active overlay source");
        assert_eq!(t_pause.canonical_hash(), player_track.canonical_hash());

        ws.player.stop();
        let (t_stopped, _) = ws
            .active_overlay_source()
            .expect("stopped player must fall back to lab preview track");
        assert_eq!(t_stopped.canonical_hash(), lab_track.canonical_hash());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn toggle_record_playlist_preserves_active_lab_preview_rail() {
        let dir = temp_dir("record_lab_preview");
        let mut ws = FlightWorkspace::new(dir.join("repo"), dir.join("user"));
        ws.player.active_scene_id = "gundo".to_string();

        // Seed curated rail at slot 0, then install & preview a Lab track (appended at slot 1).
        let curated = sample_linear_track(0.0, 20.0, 6.0, false);
        let lab_track = sample_linear_track(-30.0, -5.0, 8.0, false);
        ws.player.queue.push(PlaylistSlot {
            title: "Curated".to_string(),
            description: String::new(),
            scene_id: "gundo".to_string(),
            track_id: "curated_0".to_string(),
            file_path: None,
            track: curated,
            plan: None,
            transition_in: PlaylistTransition::Cut,
            hold_s: 0.0,
            stale: false,
            origin: TrackOrigin::Curated,
        });
        ws.player
            .preview_lab_track("gundo_auto_10".to_string(), lab_track.clone(), None);
        assert_eq!(ws.player.active_idx, 1);
        assert_eq!(ws.player.state, PlayerState::Playing { entry_idx: 1 });

        // Advance mid-track, then stop an already-active Render & Floor recording via Flight Player:
        // playback must remain on slot 1 at the current timestamp without jumping back to slot 0.
        let _ = ws.player.tick(2.5, 0.05, 5000.0);
        let t_before = ws.player.time_s;
        let stop_out = ws.toggle_record_playlist(true);
        assert!(stop_out.toggle_record_playlist);
        assert!(!stop_out.snap_cam);
        assert!(!ws.player.recording_playlist);
        assert_eq!(ws.player.active_idx, 1);
        assert_eq!(ws.player.state, PlayerState::Playing { entry_idx: 1 });
        assert!((ws.player.time_s - t_before).abs() < 1e-6);

        // Starting playlist recording while previewing slot 1 must record slot 1 (`LabPreview`),
        // not jump away to curated slot 0.
        let start_out = ws.toggle_record_playlist(false);
        assert!(start_out.toggle_record_playlist);
        assert!(start_out.snap_cam);
        assert!(start_out.reset_stats);
        assert!(ws.player.recording_playlist);
        assert_eq!(ws.player.active_idx, 1);
        assert_eq!(ws.player.state, PlayerState::Playing { entry_idx: 1 });
        assert_eq!(
            ws.player.active_track().expect("active track").canonical_hash(),
            lab_track.canonical_hash()
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn toggle_record_playlist_handles_empty_queue_wrap_and_cross_scene_loading() {
        let dir = temp_dir("record_edge_cases");
        let mut ws = FlightWorkspace::new(dir.join("repo"), dir.join("user"));
        ws.player.active_scene_id = "gundo".to_string();

        // Empty queue and no curated rails: refuse without starting an empty recording.
        let empty_out = ws.toggle_record_playlist(false);
        assert!(!empty_out.toggle_record_playlist);
        assert!(!ws.player.recording_playlist);
        assert_eq!(ws.player.state, PlayerState::Idle);

        // Finished Idle pass at end of slot 1 wraps back to slot 0 on next playlist record.
        let t0 = sample_linear_track(0.0, 10.0, 4.0, false);
        let t1 = sample_linear_track(10.0, 20.0, 4.0, false);
        ws.player.queue.push(PlaylistSlot {
            title: "First".to_string(),
            description: String::new(),
            scene_id: "gundo".to_string(),
            track_id: "t0".to_string(),
            file_path: None,
            track: t0,
            plan: None,
            transition_in: PlaylistTransition::Cut,
            hold_s: 0.0,
            stale: false,
            origin: TrackOrigin::Curated,
        });
        ws.player.queue.push(PlaylistSlot {
            title: "Second".to_string(),
            description: String::new(),
            scene_id: "marina".to_string(),
            track_id: "t1".to_string(),
            file_path: None,
            track: t1,
            plan: None,
            transition_in: PlaylistTransition::Cut,
            hold_s: 0.0,
            stale: false,
            origin: TrackOrigin::UserLibrary,
        });
        ws.player.active_idx = 1;
        ws.player.time_s = ws.player.duration_s();
        ws.player.state = PlayerState::Idle;
        let wrap_out = ws.toggle_record_playlist(false);
        assert!(wrap_out.toggle_record_playlist);
        assert!(ws.player.recording_playlist);
        assert_eq!(ws.player.active_idx, 0);
        assert_eq!(ws.player.state, PlayerState::Playing { entry_idx: 0 });
        ws.player.recording_playlist = false;

        // Cross-scene active slot must emit `request_scene_id` so host loads the scene.
        ws.player.active_idx = 1;
        ws.player.time_s = 0.0;
        ws.player.state = PlayerState::Paused { entry_idx: 1 };
        let cross_out = ws.toggle_record_playlist(false);
        assert!(cross_out.toggle_record_playlist);
        assert!(ws.player.recording_playlist);
        assert_eq!(cross_out.request_scene_id.as_deref(), Some("marina"));
        assert!(matches!(
            ws.player.state,
            PlayerState::Loading { ref scene_id } if scene_id == "marina"
        ));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn rail_rig_drives_along_path_track_free_and_eject_via_fnav_csg() {
        use csg_core::{Aabb as CsgAabb, Vec3 as CsgVec3};
        use csg_scene::{BodyPoseSource, DroneTilt, LandmarkHint, TrackStatus};
        use flight_navigator::fnav_core::{Lens, Segment, ShotMove, ShotMoveParams};
        use player::{FlightBodySource, FlightRigCameraMode};

        let dir = temp_dir("rail_rig_modes");
        let mut ws = FlightWorkspace::new(dir.join("repo"), dir.join("user"));

        let orbit = ShotMove::new(
            ShotMoveParams::Orbit {
                center: Vec3::new(0.0, 14.0, 0.0),
                target: Vec3::new(0.0, 14.0, 0.0),
                radius_m: 28.0,
                height_offset_m: 2.0,
                start_az_rad: 0.0,
                sweep_rad: std::f32::consts::TAU,
                lens: Lens::pinhole(35.0),
            },
            12.0,
        );
        let loop_track = Track {
            segments: vec![Segment::Shot(orbit.clone())],
            looping: true,
        };
        let oneshot_track = Track {
            segments: vec![Segment::Shot(orbit)],
            looping: false,
        };

        // Looping vs one-shot duration contract via fnav_csg::TrackBodyPoseSource.
        let loop_slot = PlaylistSlot {
            title: "Orbit Loop".to_string(),
            description: String::new(),
            scene_id: "gundo".to_string(),
            track_id: "orbit_loop".to_string(),
            file_path: None,
            track: loop_track,
            plan: None,
            transition_in: PlaylistTransition::Cut,
            hold_s: 0.0,
            stale: false,
            origin: TrackOrigin::LabPreview,
        };
        assert_eq!(FlightBodySource::from_slot(&loop_slot).duration_s(), None);

        let oneshot_slot = PlaylistSlot {
            track: oneshot_track,
            ..loop_slot.clone()
        };
        assert_eq!(
            FlightBodySource::from_slot(&oneshot_slot).duration_s(),
            Some(12.0)
        );

        ws.player.queue.push(loop_slot);
        ws.player.set_scene_landmarks(vec![LandmarkHint {
            name: "Civic Spire".to_string(),
            center: CsgVec3::new(0.0, 14.0, 0.0),
            bounds: CsgAabb::new(
                CsgVec3::new(-5.0, 0.0, -5.0),
                CsgVec3::new(5.0, 28.0, 5.0),
            ),
            preferred_view_dir: CsgVec3::new(0.0, 0.0, -1.0),
            hero_radius: 10.0,
            priority: 200,
        }]);
        assert_eq!(
            ws.player.rig_track_landmark.as_deref(),
            Some("Civic Spire")
        );
        assert!(ws.player.play());

        let dt = 1.0 / 60.0;
        let cur_pose = CameraPose::default();

        // 1. AlongPath: centripetal acceleration must produce measurable DroneTilt.
        ws.apply_command(FlightCommand::SetRigMode(FlightRigCameraMode::AlongPath), cur_pose);
        ws.apply_command(
            FlightCommand::SetRigTilt(DroneTilt {
                max_tilt_deg: 25.0,
                tau_s: 0.25,
            }),
            cur_pose,
        );
        let mut max_tilt_deg = 0.0_f32;
        for _ in 0..45 {
            let ev = ws
                .player
                .tick(dt, 0.05, 5000.0)
                .expect("RailRig AlongPath tick");
            assert!(ev.pose.eye.is_finite());
            assert!(ws.player.last_rig.active);
            max_tilt_deg = max_tilt_deg.max(ws.player.last_rig.tilt_deg);
        }
        assert!(
            max_tilt_deg > 0.1,
            "expected measurable DroneTilt along circular orbit, got {max_tilt_deg}"
        );

        // 2. Track: gimbal locks onto Civic Spire landmark.
        ws.apply_command(FlightCommand::SetRigMode(FlightRigCameraMode::Track), cur_pose);
        for _ in 0..90 {
            let ev = ws
                .player
                .tick(dt, 0.05, 5000.0)
                .expect("RailRig Track tick");
            assert!(ev.pose.eye.is_finite());
        }
        assert_ne!(ws.player.last_rig.track_status, TrackStatus::TargetLost);

        // 3. Free: viewport drag steers gimbal without pausing playback.
        ws.apply_command(FlightCommand::SetRigMode(FlightRigCameraMode::Free), cur_pose);
        assert!(ws.player.is_rig_free_look_active());
        let yaw_before = ws.player.last_rig.gimbal_deg[0];
        ws.player.push_rig_input([25.0, -10.0, 0.0]);
        let mut last_free_pose = None;
        for _ in 0..30 {
            let ev = ws
                .player
                .tick(dt, 0.05, 5000.0)
                .expect("RailRig Free tick");
            last_free_pose = Some(ev.pose);
        }
        assert!(ws.player.is_active(), "Free look must not pause playback");
        assert!(
            (ws.player.last_rig.gimbal_deg[0] - yaw_before).abs() > 1.0,
            "expected Free mode gimbal yaw to respond to RigInput"
        );

        // 4. Eject: pauses playback and hands back the exact latest gimbal pose.
        let eject_out = ws.apply_command(FlightCommand::RigEject, cur_pose);
        assert!(eject_out.snap_cam);
        assert_eq!(eject_out.ejected_pose, last_free_pose);
        assert!(!ws.player.is_active());
        assert!(!ws.player.last_rig.active);

        let _ = fs::remove_dir_all(&dir);
    }
}
