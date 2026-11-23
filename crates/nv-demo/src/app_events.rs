//! Centralized `appbus-rs` EventBus, lock-free latest-state snapshot slot, and worker wake
//! primitives decoupling the 60+ FPS `egui` UI loop from the asynchronous OptiX render thread.
//!
//! What: defines [`AppBus`] combining three typed `appbus::EventBus` queues (`ui_bus`,
//! `render_bus`, `event_bus`), a coalescing `appbus::StateSlot<RenderStateSnapshot>` for
//! `DemoSettings` + `CameraFrame` + interactive fast-preview deadlines, and a lost-wakeup-immune
//! [`RenderWake`] condvar.
//! Why: eliminates ad-hoc `mpsc` channels and scattered boolean UI flags, prevents slider/orbit
//! drags from queueing stale render frames, and guarantees smooth 60+ FPS UI responsiveness even
//! when heavy path-traced scenes take 100–500 ms per RT frame.
//! Used by: `crate::main`, `crate::ui`, `crate::renderer`.

use std::path::PathBuf;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use appbus::{BoxedEvent, EventBus, StateSlot};
use csg_scene::{CameraMount, ProbeBackendKind, RailSpec, ViewScore};

use crate::csg_scene_host::SceneLoadedMetadata;
use crate::renderer::{CameraFrame, DemoSettings, DlssRrBufferPtrs, FrameReady};
use crate::sdg_export::SdgOccluderBackend;

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

/// Typed UI commands emitted by `ui.rs` and consumed cleanly by the main application loop.
#[derive(Clone, Debug, PartialEq)]
pub enum UiCommand {
    /// Clear progressive path-tracing and DLSS temporal accumulation.
    ResetAccum,
    /// Save a still frame (PNG or JPEG) using the active `CaptureSource`.
    SaveStill(StillFormat),
    /// Export exact 2D/3D actor keypoints, bone-OBB bounding boxes, and occlusion labels as JSON.
    ExportSdgJson,
    /// Start or stop MP4 video recording.
    ToggleRecord,
    /// Recompile `raytracer.cu` via NVRTC on the render worker.
    ReloadShader,
    /// User selected a new CSG scene or variant (pauses rail and requests auto-framing on load).
    SelectScene { scene_idx: u32, variant_idx: u32 },
    /// User switched the live 4-way CSG road geometry mode (`0=3D, 1=C1, 2=Hybrid, 3=F-Rep`).
    SetRoadMode(u8),
    /// Toggle `Freeze Pose` (pauses dynamic traffic/crowd stepping for clean multi-spp SDG accumulation).
    ToggleFreezePose,
    /// Synthesize a fresh exact-CSG collision-verified random camera rail with `seed`.
    GenerateRandomRail { seed: u64 },
    /// Snap `CameraDamper` immediately onto the current camera pose (teleport / preset / scrub).
    SnapCamera,
    /// Smoothly glide `CameraDamper` into a newly started or switched camera rail cassette.
    BlendRail,
    /// Pick and possess the object under normalized viewport coordinates `(norm_x, norm_y)` (`[0, 1]`).
    PickPossess { norm_x: f32, norm_y: f32 },
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
    /// Generate multiple candidate rails, score their key vistas on `backend`, and activate the best rail.
    RunSmartRail { seed: u64, backend: ProbeBackendKind },
    /// Trigger an interactive poke impulse on the nearest actor (or all actors in `Inspector 109`).
    PokeCrowd { impulse: f32, all: bool },
    /// Spawn (or toggle) the 2,000-actor GPU/CPU crowd stress grid around the camera.
    SpawnCrowdStress { target_count: usize },
    /// Launch a multi-frame OpenEXR + SDG JSON + `manifest.json` dataset burst capture (`16 / 30 / 60` frames).
    StartSdgBurst { frames: u32 },
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
    },
    /// Synthesize a deterministic exact-CSG random rail for the active scene on the worker thread.
    GenerateRandomRail { seed: u64 },
    /// Cast an exact world-space CSG pick ray and possess the hit object.
    PickPossess { origin: [f32; 3], dir: [f32; 3] },
    /// Possess the next dynamic vehicle (prioritizing `Robotaxi`) or mounted scene object.
    PossessNextVehicle,
    /// Switch active camera mount index on the currently possessed object.
    SetPossessMount { mount_idx: usize },
    /// Eject from the currently possessed object back to free camera.
    EjectPossess,
    /// Evaluate 64 scenic viewpoints using the requested probe backend and return the winner.
    RunSmartView { seed: u64, backend: ProbeBackendKind },
    /// Score multiple candidate random rails using the requested probe backend and activate the winner.
    RunSmartRail { seed: u64, backend: ProbeBackendKind },
    /// Hot-reload `raytracer.cu` from disk via NVRTC.
    ReloadShader,
    /// Gracefully terminate the render worker thread.
    Shutdown,
}

/// Typed events emitted by the OptiX render worker back to the 60+ FPS UI thread.
#[derive(Clone, Debug)]
pub enum RenderEvent {
    /// A newly rendered viewport frame is ready in shared GPU memory.
    FrameReady(FrameReady),
    /// A newly built CSG scene exported its viewpoints and collision-verified camera rails.
    SceneMetaUpdated(SceneLoadedMetadata),
    /// An on-demand exact-CSG random rail finished generating.
    RailGenerated(RailSpec),
    /// Active possessed body or mount changed (`None` = ejected to free camera).
    PossessChanged(Option<PossessInfo>),
    /// A `Smart View` scoring run selected a top viewpoint.
    SmartViewReady(SmartViewResult),
    /// Render worker transitioned between compile/build/IAS/rendering phases.
    PhaseChanged(EnginePhase),
    /// Human-readable status or diagnostic message for the UI status bar.
    StatusMessage(String),
}

/// Lock-free coalesced latest render state published by the UI loop and read by the OptiX worker.
///
/// Storing `DemoSettings` and `CameraFrame` in an `appbus::StateSlot` guarantees that rapid
/// slider drags or mouse orbits never build up a backlog of stale frames — the render thread
/// always consumes only the latest snapshot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderStateSnapshot {
    /// Latest effective render settings (after `DemoAnimator` evaluation).
    pub settings: DemoSettings,
    /// Latest camera basis (after `CameraRail` and `CameraDamper` evaluation).
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
        let render_wake = RenderWake::default();

        let wake_for_bus = render_wake.clone();
        render_bus.set_waker(Arc::new(move || wake_for_bus.notify()));

        Self {
            ui_bus,
            render_bus,
            event_bus,
            state,
            render_wake,
        }
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
        };
        let bus = AppBus::new(initial);
        let mut scratch = Vec::with_capacity(16);

        bus.emit_ui(UiCommand::ResetAccum);
        bus.emit_ui(UiCommand::SaveStill(StillFormat::Png));
        bus.emit_ui(UiCommand::GenerateRandomRail { seed: 42 });
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
            Some(&UiCommand::GenerateRandomRail { seed: 42 })
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
}
