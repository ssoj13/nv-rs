//! Persistent user preferences and window geometry stored in `~/.nv-rs/nv-demo.json`.
//!
//! What: serializes window position/size/maximized state, [`DemoSettings`], and camera
//! navigation settings/pose to JSON using atomic write-and-rename.
//! Why: preserves layout and scene tuning across sessions without losing defaults when new
//! fields are added (`#[serde(default)]`).
//! Used by: `crate::main` (`Runner` and `DemoApp`).

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::camera_core::input::NavScheme;
use crate::camera_core::{CameraPose, NavFeel, NavMode};
use crate::demo_anim::DemoAnimator;
use crate::renderer::DemoSettings;
use crate::ui::UiTheme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct WindowState {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            width: 1280,
            height: 720,
            maximized: false,
        }
    }
}

impl WindowState {
    pub fn sanitized(self) -> Self {
        Self {
            x: self.x.filter(|&v| (-16384..=16384).contains(&v)),
            y: self.y.filter(|&v| (-16384..=16384).contains(&v)),
            width: self.width.clamp(320, 16384),
            height: self.height.clamp(240, 16384),
            maximized: self.maximized,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedPose {
    pub eye: [f32; 3],
    pub orientation: [f32; 4],
    pub fov_y: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl SavedPose {
    pub fn from_pose(pose: CameraPose) -> Self {
        Self {
            eye: pose.eye.to_array(),
            orientation: pose.orientation.to_array(),
            fov_y: pose.fov_y,
            znear: pose.znear,
            zfar: pose.zfar,
        }
    }

    pub fn to_pose(self) -> Option<CameraPose> {
        let eye = Vec3::from_array(self.eye);
        let q = Quat::from_array(self.orientation);
        if !eye.is_finite()
            || !q.is_finite()
            || q.length_squared() < 1e-6
            || !self.fov_y.is_finite()
            || self.fov_y <= 0.0
            || !self.znear.is_finite()
            || self.znear <= 0.0
            || !self.zfar.is_finite()
            || self.zfar <= self.znear
        {
            return None;
        }
        Some(CameraPose {
            eye,
            orientation: q.normalize(),
            fov_y: self.fov_y,
            znear: self.znear,
            zfar: self.zfar,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CameraState {
    pub mode: String,
    pub fly_mode: String,
    pub scheme: String,
    pub feel: String,
    pub pose: Option<SavedPose>,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            mode: nav_mode_name(NavMode::Turntable).to_string(),
            fly_mode: nav_mode_name(NavMode::FpsFly).to_string(),
            scheme: nav_scheme_name(NavScheme::Default).to_string(),
            feel: nav_feel_name(NavFeel::Sharp).to_string(),
            pose: None,
        }
    }
}

impl CameraState {
    pub fn nav_mode(&self) -> NavMode {
        parse_nav_mode(&self.mode).unwrap_or(NavMode::Turntable)
    }

    pub fn nav_fly_mode(&self) -> NavMode {
        parse_nav_mode(&self.fly_mode).unwrap_or(NavMode::FpsFly)
    }

    pub fn nav_scheme(&self) -> NavScheme {
        parse_nav_scheme(&self.scheme).unwrap_or(NavScheme::Default)
    }

    pub fn nav_feel(&self) -> NavFeel {
        parse_nav_feel(&self.feel).unwrap_or(NavFeel::Sharp)
    }

    pub fn from_runtime(
        mode: NavMode,
        fly_mode: NavMode,
        scheme: NavScheme,
        feel: NavFeel,
        pose: CameraPose,
    ) -> Self {
        Self {
            mode: nav_mode_name(mode).to_string(),
            fly_mode: nav_mode_name(fly_mode).to_string(),
            scheme: nav_scheme_name(scheme).to_string(),
            feel: nav_feel_name(feel).to_string(),
            pose: Some(SavedPose::from_pose(pose)),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PersistedState {
    pub window: WindowState,
    pub theme: UiTheme,
    pub settings: DemoSettings,
    pub demo_anim: DemoAnimator,
    pub camera: CameraState,
    pub camera_rail: crate::camera_rail::CameraRail,
}

pub fn config_path() -> Option<PathBuf> {
    let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"))?;
    Some(PathBuf::from(home).join(".nv-rs").join("nv-demo.json"))
}

pub fn load() -> PersistedState {
    let Some(path) = config_path() else {
        return PersistedState::default();
    };
    load_from(&path)
}

pub fn load_from(path: &Path) -> PersistedState {
    let Ok(text) = std::fs::read_to_string(path) else {
        return PersistedState::default();
    };
    serde_json::from_str(&text).unwrap_or_default()
}

pub fn save(state: &PersistedState) -> std::io::Result<()> {
    let Some(path) = config_path() else {
        return Ok(());
    };
    save_to(&path, state)
}

pub fn save_to(path: &Path, state: &PersistedState) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_string_pretty(state).map_err(std::io::Error::other)?;
    let tmp = path.with_extension(format!("json.tmp.{}", std::process::id()));
    {
        let mut file = std::fs::File::create(&tmp)?;
        file.write_all(json.as_bytes())?;
        file.flush()?;
        file.sync_all()?;
    }
    if let Err(err) = std::fs::rename(&tmp, path) {
        if let Err(rm_err) = std::fs::remove_file(&tmp) {
            eprintln!(
                "nv-demo persist cleanup failed for '{}': {rm_err}",
                tmp.display()
            );
        }
        return Err(err);
    }
    Ok(())
}

#[derive(Default)]
struct PersistSlot {
    pending: Option<PersistedState>,
    shutdown: bool,
}

/// Asynchronous background persistence worker coalescing [`PersistedState`] saves off the UI thread.
///
/// Guarantees that `serde_json` formatting and `File::sync_all` (`FlushFileBuffers`) never stall
/// the 60 Hz UI loop, while flushing the latest snapshot cleanly on shutdown.
pub struct PersistWorker {
    slot: Arc<(Mutex<PersistSlot>, Condvar)>,
    handle: Option<JoinHandle<()>>,
}

impl PersistWorker {
    /// Spawns a background persistence thread targeting the default [`config_path`].
    pub fn spawn() -> Self {
        Self::spawn_for_path(config_path())
    }

    /// Spawns a background persistence thread writing to `target_path` (when `Some`).
    pub fn spawn_for_path(target_path: Option<PathBuf>) -> Self {
        let slot = Arc::new((Mutex::new(PersistSlot::default()), Condvar::new()));
        let worker_slot = Arc::clone(&slot);
        let handle = std::thread::Builder::new()
            .name("nv-demo-persist".to_string())
            .spawn(move || {
                let (lock, cvar) = &*worker_slot;
                loop {
                    let next = {
                        let mut guard = lock.lock().unwrap_or_else(|e| e.into_inner());
                        while guard.pending.is_none() && !guard.shutdown {
                            guard = cvar.wait(guard).unwrap_or_else(|e| e.into_inner());
                        }
                        match guard.pending.take() {
                            Some(st) => Some(st),
                            None if guard.shutdown => None,
                            None => continue,
                        }
                    };
                    let Some(state) = next else {
                        break;
                    };
                    if let Some(ref path) = target_path
                        && let Err(e) = save_to(path, &state)
                    {
                        eprintln!("nv-demo persist save failed ('{}'): {e}", path.display());
                    }
                }
            })
            .ok();
        Self { slot, handle }
    }

    /// Coalesces `state` as the latest pending save and wakes the background worker (non-blocking).
    pub fn enqueue(&self, state: PersistedState) {
        if self.handle.is_none() {
            if let Err(e) = save(&state) {
                eprintln!("nv-demo persist fallback save failed: {e}");
            }
            return;
        }
        let (lock, cvar) = &*self.slot;
        let mut guard = lock.lock().unwrap_or_else(|e| e.into_inner());
        guard.pending = Some(state);
        cvar.notify_one();
    }

    /// Enqueues an optional final state, signals shutdown, and waits for durable completion.
    pub fn flush_and_join(&mut self, final_state: Option<PersistedState>) {
        {
            let (lock, cvar) = &*self.slot;
            let mut guard = lock.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(st) = final_state {
                guard.pending = Some(st);
            }
            guard.shutdown = true;
            cvar.notify_one();
        }
        if let Some(handle) = self.handle.take()
            && let Err(e) = handle.join()
        {
            eprintln!("nv-demo persist worker join error: {e:?}");
        }
    }
}

impl Drop for PersistWorker {
    fn drop(&mut self) {
        self.flush_and_join(None);
    }
}

fn nav_mode_name(m: NavMode) -> &'static str {
    match m {
        NavMode::Turntable => "Turntable",
        NavMode::EulerTurntable => "EulerTurntable",
        NavMode::Trackball => "Trackball",
        NavMode::FpsFly => "FpsFly",
        NavMode::SpaceSim => "SpaceSim",
        NavMode::SpaceSimLocked => "SpaceSimLocked",
        NavMode::QuatFlight => "QuatFlight",
        NavMode::QuatFlightLocked => "QuatFlightLocked",
    }
}

fn parse_nav_mode(s: &str) -> Option<NavMode> {
    Some(match s {
        "Turntable" => NavMode::Turntable,
        "EulerTurntable" => NavMode::EulerTurntable,
        "Trackball" => NavMode::Trackball,
        "FpsFly" => NavMode::FpsFly,
        "SpaceSim" => NavMode::SpaceSim,
        "SpaceSimLocked" => NavMode::SpaceSimLocked,
        "QuatFlight" => NavMode::QuatFlight,
        "QuatFlightLocked" => NavMode::QuatFlightLocked,
        _ => return None,
    })
}

fn nav_scheme_name(s: NavScheme) -> &'static str {
    match s {
        NavScheme::Default => "Default",
        NavScheme::Maya => "Maya",
        NavScheme::Houdini => "Houdini",
        NavScheme::Blender => "Blender",
        NavScheme::Unreal => "Unreal",
    }
}

fn parse_nav_scheme(s: &str) -> Option<NavScheme> {
    Some(match s {
        "Default" => NavScheme::Default,
        "Maya" => NavScheme::Maya,
        "Houdini" => NavScheme::Houdini,
        "Blender" => NavScheme::Blender,
        "Unreal" => NavScheme::Unreal,
        _ => return None,
    })
}

fn nav_feel_name(f: NavFeel) -> &'static str {
    match f {
        NavFeel::Sharp => "Sharp",
        NavFeel::Smooth => "Smooth",
    }
}

fn parse_nav_feel(s: &str) -> Option<NavFeel> {
    Some(match s {
        "Sharp" => NavFeel::Sharp,
        "Smooth" => NavFeel::Smooth,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_preserves_window_settings_and_camera() {
        let mut state = PersistedState {
            window: WindowState {
                x: Some(120),
                y: Some(80),
                width: 1600,
                height: 900,
                maximized: true,
            },
            ..PersistedState::default()
        };
        state.settings.sat_count = 6;
        state.settings.sat_seed = 777;
        state.settings.glass_absorb = 5.5;
        state.settings.csg_mode = 2;
        state.settings.csg_asset_idx = 5;
        state.settings.csg_time_of_day = 2;
        state.settings.csg_wetness = 0.72;
        state.settings.csg_traffic_on = true;
        state.settings.csg_traffic_speed = 1.4;
        state.settings.capture_source = crate::renderer::CaptureSource::FullWindow;
        state.settings.cam_smooth_tau = 0.25;
        state.settings.time_scale = 0.5;
        state.settings.record_shutter_subframes = 4;
        state.camera_rail.selected_rail_id = "landmark_slalom".to_string();
        state.camera_rail.keyframes = vec![
            crate::camera_rail::CameraKeyframe::from_look_at(
                [-12.0, 4.0, -30.0],
                [10.0, 5.0, 0.0],
                50.0,
            ),
            crate::camera_rail::CameraKeyframe::from_look_at(
                [18.0, 8.0, 40.0],
                [40.0, 12.0, 90.0],
                54.0,
            ),
        ];
        state.camera_rail.duration_s = 22.0;
        state.camera_rail.looping = true;
        state.camera_rail.ease_io = false;
        state.camera = CameraState::from_runtime(
            NavMode::Trackball,
            NavMode::QuatFlightLocked,
            NavScheme::Blender,
            NavFeel::Smooth,
            CameraPose::from_eye_forward_up(
                Vec3::new(1.0, 2.0, 3.0),
                Vec3::NEG_Z,
                Vec3::Y,
                0.8,
                0.05,
                200.0,
            ),
        );

        let dir = std::env::temp_dir().join(format!("nv_demo_persist_{}", std::process::id()));
        let path = dir.join("nv-demo.json");
        save_to(&path, &state).expect("save_to");
        let loaded = load_from(&path);
        let _ = std::fs::remove_dir_all(&dir);

        assert_eq!(loaded, state);
        assert_eq!(loaded.camera_rail.selected_rail_id, "landmark_slalom");
        assert_eq!(loaded.camera_rail.keyframes.len(), 2);
        assert_eq!(loaded.camera.nav_mode(), NavMode::Trackball);
        assert_eq!(loaded.camera.nav_fly_mode(), NavMode::QuatFlightLocked);
        assert_eq!(loaded.camera.nav_scheme(), NavScheme::Blender);
        assert_eq!(loaded.camera.nav_feel(), NavFeel::Smooth);
        assert!(loaded.camera.pose.and_then(|p| p.to_pose()).is_some());
    }

    #[test]
    fn partial_json_falls_back_to_defaults() {
        let dir = std::env::temp_dir().join(format!("nv_demo_partial_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("nv-demo.json");
        std::fs::write(
            &path,
            r#"{"window":{"width":1440},"settings":{"sat_count":9}}"#,
        )
        .expect("write");

        let loaded = load_from(&path);
        let _ = std::fs::remove_dir_all(&dir);

        assert_eq!(loaded.window.width, 1440);
        assert_eq!(loaded.window.height, 720);
        assert_eq!(loaded.settings.sat_count, 9);
        assert_eq!(loaded.settings.sat_seed, 42);
        assert_eq!(
            loaded.settings.capture_source,
            crate::renderer::CaptureSource::Viewport
        );
        assert_eq!(loaded.settings.cam_smooth_tau, 0.0);
        assert_eq!(loaded.settings.time_scale, 1.0);
        assert_eq!(loaded.settings.record_shutter_subframes, 1);
        assert!(loaded.camera_rail.selected_rail_id.is_empty());
        assert!(loaded.camera_rail.keyframes.is_empty());
        assert_eq!(loaded.camera.nav_fly_mode(), NavMode::FpsFly);
    }

    #[test]
    fn persist_worker_coalesces_and_flushes_latest_state_on_shutdown() {
        let dir = std::env::temp_dir().join(format!(
            "nv_demo_persist_worker_{}",
            std::process::id()
        ));
        let path = dir.join("nv-demo.json");
        let mut worker = PersistWorker::spawn_for_path(Some(path.clone()));
        for i in 1..=5u32 {
            let mut st = PersistedState::default();
            st.settings.sat_count = i;
            worker.enqueue(st);
        }
        let mut final_st = PersistedState::default();
        final_st.settings.sat_count = 17;
        worker.flush_and_join(Some(final_st.clone()));

        let loaded = load_from(&path);
        if let Err(e) = std::fs::remove_dir_all(&dir) {
            eprintln!("test cleanup warning: {e}");
        }
        assert_eq!(loaded.settings.sat_count, 17);
    }
}
