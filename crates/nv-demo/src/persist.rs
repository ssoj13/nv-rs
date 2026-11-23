//! Persistent user preferences and window geometry stored in `~/.nv-rs/nv-demo.json`.
//!
//! What: serializes window position/size/maximized state, [`DemoSettings`], camera navigation
//! settings/pose, the selected scene (by descriptor id) and the viewport toolbar edge to JSON
//! using atomic write-and-rename. The `egui_dock` layout is not persisted (the crate is built
//! without its `serde` feature).
//! Why: preserves scene tuning across sessions without losing defaults when new fields are
//! added (`#[serde(default)]`).
//! How: [`load_from`] reads the file section by section (one top-level key at a time), so one
//! unreadable section (renamed enum variant, type change) only resets that section. When the file
//! exists but cannot be read in full, [`LoadReport::problem`] says why and the caller must not
//! overwrite it: `DemoApp` then runs a [`PersistWorker`] without a target path, which writes
//! nowhere for the whole session, so the user's file stays byte-for-byte intact until they fix or
//! delete it.
//! Used by: `crate::main` (`Runner` and `DemoApp`).

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;

use camera::input::NavScheme;
use camera::{CameraPose, NavFeel, NavMode};
use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::demo_anim::DemoAnimator;
use crate::renderer::DemoSettings;
use crate::ui::UiTheme;
use crate::viewport_toolbar::ToolbarEdge;

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

/// Selected CSG scene stored by stable descriptor id and variant label, so a registry reorder in
/// `csg-scenes` does not open a different scene than the one saved (`settings.csg_mode` is only
/// a 1-based registry index and is kept as the fallback for files written before this field).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SavedScene {
    /// `SceneDescriptor::id` (`""` = Classic fractal, no CSG scene).
    pub id: String,
    /// Selected entry of `SceneDescriptor::variant_labels` (`""` when the scene has none).
    pub variant: String,
}

/// Persisted part of `viewport_toolbar::ToolbarState` (the measured extents are runtime-only).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PersistedToolbar {
    pub edge: ToolbarEdge,
    pub expanded: bool,
}

impl Default for PersistedToolbar {
    fn default() -> Self {
        let d = crate::viewport_toolbar::ToolbarState::default();
        Self {
            edge: d.edge,
            expanded: d.expanded,
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
    pub scene: Option<SavedScene>,
    pub viewport_toolbar: PersistedToolbar,
}

/// Result of reading the config file.
#[derive(Debug)]
pub struct LoadReport {
    /// Everything that could be read; unreadable sections keep their defaults.
    pub state: PersistedState,
    /// `Some` when the file exists but could not be read in full. The caller must not overwrite
    /// the file in this case (it would replace the unreadable sections with defaults).
    pub problem: Option<String>,
}

pub fn config_path() -> Option<PathBuf> {
    let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"))?;
    Some(PathBuf::from(home).join(".nv-rs").join("nv-demo.json"))
}

pub fn load() -> LoadReport {
    let Some(path) = config_path() else {
        return LoadReport {
            state: PersistedState::default(),
            problem: None,
        };
    };
    load_from(&path)
}

pub fn load_from(path: &Path) -> LoadReport {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return LoadReport {
                state: PersistedState::default(),
                problem: None,
            };
        }
        Err(e) => {
            return LoadReport {
                state: PersistedState::default(),
                problem: Some(format!("cannot read '{}': {e}", path.display())),
            };
        }
    };
    let (state, errors) = parse_sections(&text);
    let problem = (!errors.is_empty()).then(|| {
        format!(
            "'{}' is not fully readable ({}); it is left untouched and settings are not saved this session",
            path.display(),
            errors.join("; ")
        )
    });
    LoadReport { state, problem }
}

/// Deserializes every top-level section on its own so one bad section only resets itself.
fn parse_sections(text: &str) -> (PersistedState, Vec<String>) {
    let mut st = PersistedState::default();
    let mut errors = Vec::new();
    let root: serde_json::Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => {
            errors.push(format!("invalid JSON: {e}"));
            return (st, errors);
        }
    };
    let Some(obj) = root.as_object() else {
        errors.push("top level is not a JSON object".to_string());
        return (st, errors);
    };
    fn section<T: serde::de::DeserializeOwned>(
        obj: &serde_json::Map<String, serde_json::Value>,
        key: &str,
        slot: &mut T,
        errors: &mut Vec<String>,
    ) {
        if let Some(v) = obj.get(key) {
            match serde_json::from_value::<T>(v.clone()) {
                Ok(parsed) => *slot = parsed,
                Err(e) => errors.push(format!("section '{key}': {e}")),
            }
        }
    }
    section(obj, "window", &mut st.window, &mut errors);
    section(obj, "theme", &mut st.theme, &mut errors);
    section(obj, "settings", &mut st.settings, &mut errors);
    section(obj, "demo_anim", &mut st.demo_anim, &mut errors);
    section(obj, "camera", &mut st.camera, &mut errors);
    section(obj, "camera_rail", &mut st.camera_rail, &mut errors);
    section(obj, "scene", &mut st.scene, &mut errors);
    section(obj, "viewport_toolbar", &mut st.viewport_toolbar, &mut errors);
    (st, errors)
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
    /// Same target as the thread's; used for synchronous saves when the thread failed to spawn.
    target_path: Option<PathBuf>,
}

impl PersistWorker {
    /// Spawns a background persistence thread targeting the default [`config_path`].
    pub fn spawn() -> Self {
        Self::spawn_for_path(config_path())
    }

    /// Spawns a background persistence thread writing to `target_path` (`None` writes nowhere).
    pub fn spawn_for_path(target_path: Option<PathBuf>) -> Self {
        let slot = Arc::new((Mutex::new(PersistSlot::default()), Condvar::new()));
        let worker_slot = Arc::clone(&slot);
        let thread_target = target_path.clone();
        let spawned = std::thread::Builder::new()
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
                    if let Some(ref path) = thread_target
                        && let Err(e) = save_to(path, &state)
                    {
                        eprintln!("nv-demo persist save failed ('{}'): {e}", path.display());
                    }
                }
            });
        let handle = match spawned {
            Ok(h) => Some(h),
            Err(e) => {
                eprintln!("nv-demo persist thread spawn failed ({e}); saving synchronously");
                None
            }
        };
        Self {
            slot,
            handle,
            target_path,
        }
    }

    /// Synchronous save to `target_path` (only used when the worker thread is unavailable).
    fn save_now(&self, state: &PersistedState) {
        if let Some(ref path) = self.target_path
            && let Err(e) = save_to(path, state)
        {
            eprintln!("nv-demo persist save failed ('{}'): {e}", path.display());
        }
    }

    /// Coalesces `state` as the latest pending save and wakes the background worker (non-blocking).
    pub fn enqueue(&self, state: PersistedState) {
        if self.handle.is_none() {
            self.save_now(&state);
            return;
        }
        let (lock, cvar) = &*self.slot;
        let mut guard = lock.lock().unwrap_or_else(|e| e.into_inner());
        guard.pending = Some(state);
        cvar.notify_one();
    }

    /// Enqueues an optional final state, signals shutdown, and waits for durable completion.
    pub fn flush_and_join(&mut self, final_state: Option<PersistedState>) {
        if self.handle.is_none() {
            if let Some(st) = final_state {
                self.save_now(&st);
            }
            return;
        }
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
        NavScheme::Studio => "Studio",
        NavScheme::Maya => "Maya",
        NavScheme::Houdini => "Houdini",
        NavScheme::Blender => "Blender",
        NavScheme::Unreal => "Unreal",
    }
}

fn parse_nav_scheme(s: &str) -> Option<NavScheme> {
    Some(match s {
        "Default" => NavScheme::Default,
        "Studio" => NavScheme::Studio,
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
        state.scene = Some(SavedScene {
            id: "inspector".to_string(),
            variant: "Robotaxi".to_string(),
        });
        state.viewport_toolbar = PersistedToolbar {
            edge: ToolbarEdge::Left,
            expanded: false,
        };
        save_to(&path, &state).expect("save_to");
        let report = load_from(&path);
        let _ = std::fs::remove_dir_all(&dir);

        assert_eq!(report.problem, None);
        let loaded = report.state;
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

        let report = load_from(&path);
        let _ = std::fs::remove_dir_all(&dir);

        assert_eq!(report.problem, None);
        let loaded = report.state;
        assert_eq!(loaded.scene, None);
        assert_eq!(loaded.viewport_toolbar, PersistedToolbar::default());
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

        let loaded = load_from(&path).state;
        if let Err(e) = std::fs::remove_dir_all(&dir) {
            eprintln!("test cleanup warning: {e}");
        }
        assert_eq!(loaded.settings.sat_count, 17);
    }

    /// F07: a corrupt file is reported, never overwritten, and a session without a target path
    /// writes nowhere.
    #[test]
    fn corrupt_config_is_reported_and_left_untouched() {
        let dir = std::env::temp_dir().join(format!("nv_demo_corrupt_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("nv-demo.json");
        let corrupt = r#"{"window":{"width":1440},"settings":{"sat_count":9"#;
        std::fs::write(&path, corrupt).expect("write");

        let report = load_from(&path);
        assert!(report.problem.is_some());
        assert_eq!(report.state, PersistedState::default());

        // What `DemoApp` does when `problem` is `Some`: a worker without a target path.
        let mut worker = PersistWorker::spawn_for_path(None);
        worker.enqueue(PersistedState::default());
        worker.flush_and_join(Some(PersistedState::default()));
        let after = std::fs::read_to_string(&path).expect("read back");
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(after, corrupt);
    }

    /// F07: one unreadable section (unknown enum variant) only resets itself; the rest (here the
    /// Custom rail keyframes and the window) still loads, and the problem is reported.
    #[test]
    fn unreadable_section_keeps_the_other_sections() {
        let dir = std::env::temp_dir().join(format!("nv_demo_section_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("nv-demo.json");
        let mut good = PersistedState::default();
        good.window.width = 1500;
        good.camera_rail.keyframes = vec![
            crate::camera_rail::CameraKeyframe::from_look_at([0.0, 2.0, -9.0], [0.0, 1.0, 0.0], 50.0),
            crate::camera_rail::CameraKeyframe::from_look_at([5.0, 3.0, 4.0], [0.0, 1.0, 0.0], 50.0),
        ];
        let mut json: serde_json::Value = serde_json::to_value(&good).expect("to_value");
        json["theme"] = serde_json::Value::String("NoSuchThemeVariant".to_string());
        std::fs::write(&path, serde_json::to_string(&json).expect("to_string")).expect("write");

        let report = load_from(&path);
        let _ = std::fs::remove_dir_all(&dir);
        let problem = report.problem.expect("problem reported");
        assert!(problem.contains("theme"), "{problem}");
        assert_eq!(report.state.window.width, 1500);
        assert_eq!(report.state.camera_rail.keyframes.len(), 2);
        assert_eq!(report.state.theme, UiTheme::default());
    }

    /// F07: an existing path that cannot be read (here: a directory) is a problem, not a default.
    #[test]
    fn unreadable_path_is_a_problem_but_missing_file_is_not() {
        let dir = std::env::temp_dir().join(format!("nv_demo_unreadable_{}", std::process::id()));
        std::fs::create_dir_all(dir.join("nv-demo.json")).expect("mkdir");
        let report = load_from(&dir.join("nv-demo.json"));
        let missing = load_from(&dir.join("absent.json"));
        let _ = std::fs::remove_dir_all(&dir);
        assert!(report.problem.is_some());
        assert_eq!(missing.problem, None);
    }

    /// F20: without a worker thread the final state is saved synchronously, not dropped.
    #[test]
    fn flush_without_worker_thread_saves_synchronously() {
        let dir = std::env::temp_dir().join(format!("nv_demo_nothread_{}", std::process::id()));
        let path = dir.join("nv-demo.json");
        let mut worker = PersistWorker {
            slot: Arc::new((Mutex::new(PersistSlot::default()), Condvar::new())),
            handle: None,
            target_path: Some(path.clone()),
        };
        let mut st = PersistedState::default();
        st.settings.sat_count = 23;
        worker.flush_and_join(Some(st));
        let loaded = load_from(&path).state;
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(loaded.settings.sat_count, 23);
    }

    /// F21: the on-disk names are PascalCase (`"FpsFly"`), while `camera`'s serde derives use
    /// `rename_all = "snake_case"` (`"fps_fly"`), so the hand-written tables stay. This pins that
    /// every variant round-trips through them.
    #[test]
    fn nav_name_tables_round_trip_every_variant() {
        for m in NavMode::ALL {
            assert_eq!(parse_nav_mode(nav_mode_name(m)), Some(m));
        }
        for s in NavScheme::ALL {
            assert_eq!(parse_nav_scheme(nav_scheme_name(s)), Some(s));
        }
        for f in NavFeel::ALL {
            assert_eq!(parse_nav_feel(nav_feel_name(f)), Some(f));
        }
        assert_eq!(nav_mode_name(NavMode::FpsFly), "FpsFly");
    }
}
