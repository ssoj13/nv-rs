//! Docked egui UI with Phosphor icons and uniformly aligned property grids.

use std::ops::RangeInclusive;

use egui::{Align, Color32, FontDefinitions, Layout, RichText, Sense, Ui};
use egui_dock::{DockState, NodeIndex, Style, TabViewer};
use egui_phosphor::regular as ic;

use crate::app_events::{
    AppBus, EnginePhase, PossessInfo, SmartViewResult, StillFormat, UiCommand,
};
use crate::camera_core::input::{NavScheme, altitude_speed_multiplier};
use crate::camera_core::{CameraDriver, CameraIntent, CameraPose, NavFeel, NavMode, ViewportSize};
use crate::camera_rail::CameraRail;
use crate::csg_scene_host::CsgStats;
use crate::demo_anim::DemoAnimator;
use crate::profiler::{FrameTimings, GpuTelemetry, HISTORY_LEN, RingBuffer};
use crate::renderer::{CameraProjection, CaptureSource, DemoSettings};
use crate::sdg_export::{ActiveCrowdBackend, CrowdBackend, SdgOccluderBackend};
use csg_scene::{ProbeBackendKind, SceneDescriptor, SceneViewpoint, TimeOfDay};

const AOV_LABELS: [&str; 11] = [
    "Beauty",
    "Albedo",
    "Normal",
    "Depth",
    "Motion",
    "Semantic",
    "Instance",
    "WorldPos",
    "Range (LiDAR)",
    "Material",
    "Cryptomatte",
];

const AOV_HUD_TAGS: [&str; 11] = [
    "BEAUTY",
    "ALBEDO",
    "NORMAL",
    "DEPTH",
    "MOTION",
    "SEMANTIC",
    "INSTANCE",
    "WORLDPOS",
    "RANGE",
    "MATERIAL",
    "CRYPTO",
];

const LABEL_W: f32 = 84.0;
const VAL_W: f32 = 56.0;
const ANIM_BTN_W: f32 = 16.0;
const ROW_H: f32 = 14.0;

/// WGS84 semi-major equatorial radius in meters.
const WGS84_A: f64 = 6_378_137.0;
/// WGS84 flattening factor (`1 / 298.257223563`).
const WGS84_F: f64 = 1.0 / 298.257_223_563;
/// WGS84 first eccentricity squared (`f * (2 - f)`).
const WGS84_E2: f64 = WGS84_F * (2.0 - WGS84_F);

/// Returns the WGS84 geodetic origin `(lat_deg, lon_deg)` for real-world OSM scenes.
///
/// Matches the verified `osm_rs` origins (`GUNDO_EL_SEGUNDO_ORIGIN`, `MARINA_CITY_DR_ORIGIN`,
/// `SF_HOUSE_OF_AI_ORIGIN`). Returns `None` for non-georeferenced scenes (`classic`, procedural
/// `gundo`, `inspector`).
pub fn scene_wgs84_origin(scene_id: &str) -> Option<(f64, f64)> {
    match scene_id {
        "gundo_osm" => Some((33.9138, -118.4190)),
        "marina" => Some((33.9846, -118.4534)),
        "sf_house_of_ai" => Some((37.7750, -122.4032)),
        _ => None,
    }
}

/// Resolves the `Home` camera pose and optional orbit pivot distance for the active mode.
///
/// When a CSG scene is active (`csg_mode != 0`) and exports at least one curated viewpoint,
/// returns viewpoint `[0]` (guaranteed above DEM terrain) and its eye-to-target distance.
/// Otherwise (Classic Fractal mode or empty viewpoints) returns `classic_home`.
pub fn resolve_home_pose(
    csg_mode: u32,
    viewpoints: &[SceneViewpoint],
    classic_home: CameraPose,
) -> (CameraPose, Option<f32>) {
    if csg_mode != 0
        && let Some(vp) = viewpoints.first()
    {
        let eye = glam::Vec3::new(vp.eye.x, vp.eye.y, vp.eye.z);
        let target = glam::Vec3::new(vp.target.x, vp.target.y, vp.target.z);
        let delta = target - eye;
        let dist = delta.length().max(0.25);
        let fwd = if delta.length_squared() > 1e-6 {
            delta / dist
        } else {
            -glam::Vec3::Z
        };
        let pose = CameraPose::from_eye_forward_up(
            eye,
            fwd,
            glam::Vec3::Y,
            vp.vfov_deg.clamp(15.0, 110.0).to_radians(),
            classic_home.znear,
            classic_home.zfar.max(2000.0),
        );
        return (pose, Some(dist));
    }
    (classic_home, None)
}

/// Formats the human-readable engine phase prefix for the bottom-left Viewport status bar.
pub fn format_engine_phase_label(
    phase: EnginePhase,
    accum_count: u32,
    max_accum: u32,
    anim_tag: Option<&str>,
) -> String {
    if let Some(label) = phase.transient_label() {
        return label.to_owned();
    }
    if let Some(tag) = anim_tag {
        return format!("Live ({tag})");
    }
    if accum_count >= max_accum.max(1) {
        "Ready".to_owned()
    } else {
        format!("Accumulating ({accum_count}/{})", max_accum.max(1))
    }
}

/// Converts a right-handed world position `[x, y, z]` (`+X = East`, `+Y = Up` orthometric
/// height in meters above sea level, `-Z = North`) into Apple Maps–compatible WGS84 geodetic
/// coordinates `(lat_deg, lon_deg, alt_m)` relative to `(origin_lat, origin_lon)`.
///
/// Uses the exact WGS84 meridional (`R_M`) and prime-vertical (`R_N`) radii of curvature at
/// `origin_lat`. Non-finite coordinates are sanitized to `0.0`.
pub fn world_to_wgs84(origin_lat: f64, origin_lon: f64, pos: [f32; 3]) -> (f64, f64, f32) {
    let east_m = if pos[0].is_finite() { pos[0] as f64 } else { 0.0 };
    let alt_m = if pos[1].is_finite() { pos[1] } else { 0.0 };
    let north_m = if pos[2].is_finite() { -(pos[2] as f64) } else { 0.0 };

    let lat0_rad = origin_lat.clamp(-89.9999, 89.9999).to_radians();
    let sin_lat = lat0_rad.sin();
    let cos_lat = lat0_rad.cos().max(1e-6);
    let w2 = 1.0 - WGS84_E2 * sin_lat * sin_lat;
    let w = w2.sqrt();
    let r_n = WGS84_A / w;
    let r_m = WGS84_A * (1.0 - WGS84_E2) / (w2 * w);

    let lat_deg = origin_lat + (north_m / r_m).to_degrees();
    let lon_deg = origin_lon + (east_m / (r_n * cos_lat)).to_degrees();
    (lat_deg, lon_deg, alt_m)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DockTab {
    Viewport,
    Fractal,
    RenderFloor,
    CameraStats,
}

impl DockTab {
    pub fn title(self) -> String {
        match self {
            Self::Viewport => format!("{}  Viewport", ic::APERTURE),
            Self::Fractal => format!("{}  Fractal", ic::CUBE),
            Self::RenderFloor => format!("{}  Render & Floor", ic::SUN),
            Self::CameraStats => format!("{}  Camera & Stats", ic::GAUGE),
        }
    }
}

pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    // Ensure phosphor icons also resolve when a widget sets TextStyle::Monospace.
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("phosphor".to_owned());
    ctx.set_fonts(fonts);
}

pub fn default_dock_state() -> DockState<DockTab> {
    let mut state = DockState::new(vec![DockTab::Viewport]);
    let surface = state.main_surface_mut();
    let [vp, _left] = surface.split_left(NodeIndex::root(), 0.17, vec![DockTab::CameraStats]);
    let [_vp, right] = surface.split_right(vp, 0.70, vec![DockTab::Fractal]);
    let [_top, _bot] = surface.split_below(right, 0.41, vec![DockTab::RenderFloor]);
    state
}

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum UiTheme {
    #[default]
    Obsidian,
    Midnight,
    Slate,
    Catppuccin,
    Light,
}

impl UiTheme {
    pub const ALL: [Self; 5] = [
        Self::Obsidian,
        Self::Midnight,
        Self::Slate,
        Self::Catppuccin,
        Self::Light,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Obsidian => "Obsidian",
            Self::Midnight => "Midnight",
            Self::Slate => "Slate",
            Self::Catppuccin => "Catppuccin",
            Self::Light => "Light",
        }
    }

    pub const fn next(self) -> Self {
        match self {
            Self::Obsidian => Self::Midnight,
            Self::Midnight => Self::Slate,
            Self::Slate => Self::Catppuccin,
            Self::Catppuccin => Self::Light,
            Self::Light => Self::Obsidian,
        }
    }

    pub fn section_color(self) -> Color32 {
        match self {
            Self::Light => Color32::from_rgb(38, 125, 65),
            _ => Color32::from_rgb(166, 209, 137),
        }
    }

    /// Applies unified panel/widget visuals to `ctx` and returns a matching `egui_dock::Style`.
    pub fn apply(self, ctx: &egui::Context) -> Style {
        let (is_light, panel_bg, bar_bg, widget_bg, accent, text_col) = match self {
            Self::Obsidian => (
                false,
                Color32::from_rgb(22, 24, 29),
                Color32::from_rgb(17, 19, 23),
                Color32::from_rgb(32, 35, 42),
                Color32::from_rgb(82, 139, 255),
                Color32::from_rgb(218, 224, 235),
            ),
            Self::Midnight => (
                false,
                Color32::from_rgb(14, 16, 21),
                Color32::from_rgb(10, 12, 16),
                Color32::from_rgb(24, 27, 35),
                Color32::from_rgb(100, 160, 255),
                Color32::from_rgb(220, 226, 238),
            ),
            Self::Slate => (
                false,
                Color32::from_rgb(32, 35, 41),
                Color32::from_rgb(26, 29, 34),
                Color32::from_rgb(44, 48, 56),
                Color32::from_rgb(97, 175, 239),
                Color32::from_rgb(224, 228, 236),
            ),
            Self::Catppuccin => (
                false,
                Color32::from_rgb(30, 30, 46),
                Color32::from_rgb(24, 24, 37),
                Color32::from_rgb(49, 50, 68),
                Color32::from_rgb(137, 180, 250),
                Color32::from_rgb(205, 214, 244),
            ),
            Self::Light => (
                true,
                Color32::from_rgb(240, 242, 246),
                Color32::from_rgb(228, 231, 237),
                Color32::from_rgb(252, 253, 255),
                Color32::from_rgb(37, 99, 235),
                Color32::from_rgb(30, 34, 42),
            ),
        };

        let mut visuals = if is_light {
            egui::Visuals::light()
        } else {
            egui::Visuals::dark()
        };
        visuals.panel_fill = panel_bg;
        visuals.window_fill = panel_bg;
        visuals.extreme_bg_color = bar_bg;
        visuals.faint_bg_color = widget_bg;
        visuals.override_text_color = Some(text_col);
        visuals.selection.bg_fill = accent.linear_multiply(0.35);
        visuals.widgets.noninteractive.bg_fill = panel_bg;
        visuals.widgets.inactive.bg_fill = widget_bg;
        visuals.widgets.hovered.bg_fill = if is_light {
            Color32::from_rgb(218, 223, 232)
        } else {
            Color32::from_rgb(50, 55, 66)
        };
        visuals.widgets.active.bg_fill = accent.linear_multiply(0.55);
        let egui_theme = if is_light {
            egui::Theme::Light
        } else {
            egui::Theme::Dark
        };
        ctx.set_theme(egui_theme);
        ctx.set_visuals_of(egui_theme, visuals);
        ctx.style_mut_of(egui_theme, |s| {
            let mut scroll = egui::style::ScrollStyle::solid();
            scroll.bar_width = 4.0;
            scroll.floating_width = 4.0;
            scroll.bar_inner_margin = 2.0;
            scroll.bar_outer_margin = 2.0;
            scroll.handle_min_length = 16.0;
            scroll.dormant_background_opacity = 0.0;
            scroll.active_background_opacity = 0.14;
            scroll.dormant_handle_opacity = 0.25;
            scroll.active_handle_opacity = 0.55;
            scroll.interact_handle_opacity = 0.85;
            s.spacing.scroll = scroll;
        });

        let mut style = Style::from_egui(ctx.style_of(egui_theme).as_ref());
        style.tab_bar.height = 24.0;
        style.tab_bar.bg_fill = bar_bg;
        style.tab.tab_body.bg_fill = panel_bg;
        style.tab.active.bg_fill = panel_bg;
        style.tab.focused.bg_fill = panel_bg;
        style.tab.inactive.bg_fill = bar_bg;
        style.tab.hovered.bg_fill = widget_bg;
        style.tab.active.text_color = text_col;
        style.tab.focused.text_color = text_col;
        style.separator.width = 3.0;
        style.separator.color_idle = bar_bg;
        style
    }
}

pub struct UiContext<'a> {
    pub bus: &'a AppBus,
    pub settings: &'a mut DemoSettings,
    pub demo_anim: &'a mut DemoAnimator,
    pub display_tex_id: egui::TextureId,
    pub render_w: u32,
    pub render_h: u32,
    pub desired_vp_size: &'a mut (f32, f32),
    pub cam_driver: &'a mut CameraDriver,
    pub cam_home: CameraPose,
    pub cam_pose: &'a mut CameraPose,
    pub camera_rail: &'a mut CameraRail,
    pub cam_fixed_dt: Option<f32>,
    pub cam_mode: &'a mut NavMode,
    pub cam_fly_mode: &'a mut NavMode,
    pub cam_scheme: &'a mut NavScheme,
    pub cam_feel: &'a mut NavFeel,
    pub theme: &'a mut UiTheme,
    pub telemetry: &'a GpuTelemetry,
    pub timings: &'a FrameTimings,
    pub accum_count: u32,
    pub total_samples: u32,
    pub rt_ms: f32,
    pub denoise_ms: f32,
    pub bloom_ms: f32,
    pub tonemap_ms: f32,
    /// Hardware **Tensor Cores (Denoiser / DLSS)** execution time in milliseconds.
    pub tensor_cores_ms: f32,
    /// Hardware **CUDA Compute (IAS / cuFFT / Tonemap)** execution time in milliseconds.
    pub cuda_compute_ms: f32,
    /// Active engine execution phase (`CompilingOptix`, `BuildingCsgScene`, `UpdatingIas`, `Rendering`).
    pub engine_phase: EnginePhase,
    pub ui_fps: f32,
    pub rt_fps: f32,
    pub fast_preview: bool,
    pub is_recording: bool,
    pub record_frames: u64,
    pub ser_supported: bool,
    pub csg_stats: CsgStats,
    pub crowd_step_ms: f32,
    pub crowd_active_backend: ActiveCrowdBackend,
    pub crowd_actor_count: u32,
    pub scene_descriptors: &'a [SceneDescriptor],
    pub scene_viewpoints: &'a [SceneViewpoint],
    pub rail_random_seed: &'a mut u64,
    pub dlss_rr_available: bool,
    pub hdr_surface_supported: bool,
    pub viewport_focused: &'a mut bool,
    pub possess_info: Option<&'a PossessInfo>,
    pub possess_free_yaw: &'a mut f32,
    pub possess_free_pitch: &'a mut f32,
    pub possess_auto_level: &'a mut bool,
    pub possess_zoom_offset: &'a mut f32,
    pub viewport_toolbar: &'a mut crate::viewport_toolbar::ToolbarState,
    pub pick_possess_mode: &'a mut bool,
    pub smart_view_seed: &'a mut u64,
    pub last_smart_view: Option<SmartViewResult>,
    pub last_saved_msg: Option<&'a str>,
}

impl TabViewer for UiContext<'_> {
    type Tab = DockTab;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(*tab)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title().into()
    }

    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        false
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab {
            DockTab::Viewport => self.ui_viewport(ui),
            DockTab::Fractal => dock_scroll(ui, |ui| self.ui_fractal(ui)),
            DockTab::RenderFloor => dock_scroll(ui, |ui| self.ui_render_floor(ui)),
            DockTab::CameraStats => dock_scroll(ui, |ui| self.ui_camera_stats(ui)),
        }
    }
}

/// Vertical `ScrollArea` locked to the visible dock pane width so no wide button or combo
/// box can expand `ui.available_width()` and clip sibling sections off the right edge.
fn dock_scroll(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    let sc = ui.spacing().scroll;
    let bar_reserve = sc.bar_outer_margin + sc.bar_width + sc.bar_inner_margin + 2.0;
    let inner_w = (ui.available_width() - bar_reserve).max(120.0);
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_max_width(inner_w);
            ui.set_width(inner_w);
            add_contents(ui);
        });
}

impl UiContext<'_> {
    pub fn ui_toolbar(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            let (prev_m, prev_fly, prev_s, prev_f) = (
                *self.cam_mode,
                *self.cam_fly_mode,
                *self.cam_scheme,
                *self.cam_feel,
            );

            ui.label(RichText::new("Orbit").small().weak());
            egui::ComboBox::from_id_salt("toolbar_cam_mode")
                .width(96.0)
                .selected_text(self.cam_mode.label())
                .show_ui(ui, |ui| {
                    for m in NavMode::ALL.into_iter().filter(|m| !m.is_fly()) {
                        ui.selectable_value(self.cam_mode, m, m.label());
                    }
                });

            ui.label(RichText::new("Fly (RMB)").small().weak());
            egui::ComboBox::from_id_salt("toolbar_cam_fly")
                .width(104.0)
                .selected_text(self.cam_fly_mode.label())
                .show_ui(ui, |ui| {
                    for m in NavMode::ALL.into_iter().filter(|m| m.is_fly()) {
                        ui.selectable_value(self.cam_fly_mode, m, m.label());
                    }
                });

            ui.label(RichText::new("Scheme").small().weak());
            egui::ComboBox::from_id_salt("toolbar_cam_scheme")
                .width(74.0)
                .selected_text(format!("{:?}", *self.cam_scheme))
                .show_ui(ui, |ui| {
                    for s in NavScheme::ALL {
                        ui.selectable_value(self.cam_scheme, s, format!("{s:?}"));
                    }
                });

            ui.label(RichText::new("Feel").small().weak());
            egui::ComboBox::from_id_salt("toolbar_cam_feel")
                .width(74.0)
                .selected_text(format!("{:?}", *self.cam_feel))
                .show_ui(ui, |ui| {
                    for f in NavFeel::ALL {
                        ui.selectable_value(self.cam_feel, f, format!("{f:?}"));
                    }
                });

            if (
                *self.cam_mode,
                *self.cam_fly_mode,
                *self.cam_scheme,
                *self.cam_feel,
            ) != (prev_m, prev_fly, prev_s, prev_f)
            {
                self.cam_driver.controller_mut().set_mode(*self.cam_mode);
                let mut prefs = *self.cam_driver.prefs();
                prefs.hold_fly_mode = *self.cam_fly_mode;
                prefs.feel = *self.cam_feel;
                self.cam_driver.set_prefs(prefs);
                if *self.cam_scheme != prev_s {
                    self.cam_driver.set_scheme(*self.cam_scheme);
                    crate::enable_rmb_fly(self.cam_driver);
                }
            }

            if ui
                .button(format!("{} Home", ic::HOUSE))
                .on_hover_text("Reset the camera back to the active scene's default viewpoint (above terrain).")
                .clicked()
            {
                self.camera_rail.playing = false;
                let (home_pose, orbit_dist) =
                    resolve_home_pose(self.settings.csg_mode, self.scene_viewpoints, self.cam_home);
                self.cam_driver.controller_mut().set_pose(home_pose);
                if let Some(d) = orbit_dist {
                    self.cam_driver.controller_mut().set_orbit_distance(d);
                }
                *self.cam_pose = self.cam_driver.controller().pose();
                self.bus.emit_ui(UiCommand::SnapCamera);
            }
            if ui
                .button(format!("{} Frame", ic::CROSSHAIR))
                .on_hover_text("Automatically zoom and center the camera on the fractal (shortcut: A).")
                .clicked()
            {
                self.camera_rail.playing = false;
                let center = glam::Vec3::from_array(self.settings.fractal_translate);
                let ext = 1.35 * self.settings.obj_scale.clamp(0.05, 50.0);
                self.cam_driver.controller_mut().apply_intent(
                    CameraIntent::FrameBounds {
                        min: center - glam::Vec3::splat(ext),
                        max: center + glam::Vec3::splat(ext),
                        margin: 1.2,
                    },
                    1.0 / 60.0,
                    ViewportSize::new(self.render_w, self.render_h),
                );
                *self.cam_pose = self.cam_driver.controller().pose();
                self.bus.emit_ui(UiCommand::SnapCamera);
            }

            if ui
                .button(format!("{} Reset", ic::ARROWS_CLOCKWISE))
                .on_hover_text("Clear accumulated samples and restart path-tracing from frame 1.")
                .clicked()
            {
                self.bus.emit_ui(UiCommand::ResetAccum);
            }

            ui.separator();

            ui.label(RichText::new("AOV").small().weak());
            let cur_aov = (self.settings.aov_mode as usize).min(AOV_LABELS.len() - 1);
            egui::ComboBox::from_id_salt("toolbar_aov_mode")
                .width(102.0)
                .selected_text(AOV_LABELS[cur_aov])
                .show_ui(ui, |ui| {
                    for (idx, lbl) in AOV_LABELS.iter().enumerate() {
                        ui.selectable_value(&mut self.settings.aov_mode, idx as u32, *lbl);
                    }
                });

            ui.label(RichText::new("Lens").small().weak());
            egui::ComboBox::from_id_salt("toolbar_lens_proj")
                .width(96.0)
                .selected_text(self.settings.camera_projection.label())
                .show_ui(ui, |ui| {
                    for proj in CameraProjection::ALL {
                        ui.selectable_value(
                            &mut self.settings.camera_projection,
                            proj,
                            proj.label(),
                        );
                    }
                });

            let pick_on = *self.pick_possess_mode;
            let pick_btn = if pick_on {
                egui::Button::new(
                    RichText::new(format!("{} Pick", ic::CROSSHAIR_SIMPLE)).color(Color32::WHITE),
                )
                .fill(Color32::from_rgb(38, 108, 168))
            } else {
                egui::Button::new(format!("{} Pick", ic::CROSSHAIR_SIMPLE))
            };
            if ui
                .add_enabled(self.settings.csg_mode != 0, pick_btn)
                .on_hover_text(
                    "Click any vehicle, lamp, yacht, or building in the viewport to possess its camera mount (shortcut: Ctrl + Left Click).",
                )
                .clicked()
            {
                *self.pick_possess_mode = !*self.pick_possess_mode;
            }

            ui.separator();

            // Master Play / Pause + what is animated (Space shortcut outside fly mode)
            let in_fly = self.cam_driver.controller().is_fly() || self.cam_driver.is_hold_flying();
            if !ui.ctx().egui_wants_keyboard_input()
                && !in_fly
                && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Space))
            {
                self.demo_anim.toggle_play(self.settings);
            }
            let playing = self.demo_anim.playing;
            let play_label = if playing {
                format!("{} Pause", ic::PAUSE)
            } else {
                format!("{} Play", ic::PLAY)
            };
            let dark = ui.visuals().dark_mode;
            let play_btn = if playing {
                let fill = if dark {
                    Color32::from_rgb(38, 112, 68)
                } else {
                    Color32::from_rgb(44, 142, 82)
                };
                egui::Button::new(RichText::new(play_label).color(Color32::WHITE))
                    .fill(fill)
                    .stroke(egui::Stroke::new(1.0, Color32::from_rgb(82, 178, 116)))
            } else {
                egui::Button::new(RichText::new(play_label).color(ui.visuals().weak_text_color()))
                    .fill(ui.visuals().widgets.inactive.weak_bg_fill)
            };
            if ui
                .add_sized([68.0, 22.0], play_btn)
                .on_hover_text("Play / Pause selected animations (Space). Green = Playing, Gray = Paused.")
                .clicked()
            {
                self.demo_anim.toggle_play(self.settings);
            }

            ui.checkbox(&mut self.settings.auto_rotate, "Spin")
                .on_hover_text("Rotate fractal");
            ui.add_enabled(
                self.settings.sat_on,
                egui::Checkbox::new(&mut self.settings.sat_orbit, "Orbit"),
            )
            .on_hover_text("Orbit glass satellites");
            ui.checkbox(&mut self.demo_anim.enabled, "Params")
                .on_hover_text("Sinusoidal parameter animation");

            ui.separator();

            let prev_style = ui.style().override_text_style.clone();
            ui.label(RichText::new("Speed").small().weak());
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
            ui.add_sized(
                [46.0, 20.0],
                egui::DragValue::new(&mut self.settings.spin_speed)
                    .range(0.0..=2.0)
                    .speed(0.01)
                    .fixed_decimals(2),
            );
            ui.style_mut().override_text_style = prev_style.clone();

            ui.label(RichText::new("Period").small().weak());
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
            ui.add_sized(
                [48.0, 20.0],
                egui::DragValue::new(&mut self.demo_anim.period_s)
                    .range(0.5..=60.0)
                    .speed(0.1)
                    .fixed_decimals(1)
                    .suffix("s"),
            );
            ui.style_mut().override_text_style = prev_style.clone();

            ui.label(RichText::new("Amp").small().weak());
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
            ui.add_sized(
                [46.0, 20.0],
                egui::DragValue::new(&mut self.demo_anim.amp)
                    .range(0.05..=5.0)
                    .speed(0.02)
                    .fixed_decimals(2),
            );
            ui.style_mut().override_text_style = prev_style;

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let theme_btn = ui
                    .button(format!("{} {}", ic::PALETTE, self.theme.label()))
                    .on_hover_text("Left-click: cycle theme\nRight-click: choose theme");
                if theme_btn.clicked() {
                    *self.theme = self.theme.next();
                }
                theme_btn.context_menu(|ui| {
                    for t in UiTheme::ALL {
                        if ui.selectable_label(*self.theme == t, t.label()).clicked() {
                            *self.theme = t;
                            ui.close();
                        }
                    }
                });
            });
        });
    }

    fn ui_viewport(&mut self, ui: &mut Ui) {
        let avail = ui.available_size();
        let ppp = ui.ctx().pixels_per_point();
        *self.desired_vp_size = ((avail.x * ppp).max(256.0), (avail.y * ppp).max(144.0));
        let aspect = self.render_w as f32 / self.render_h as f32;
        let w = avail.x.min(avail.y * aspect).max(64.0);
        let h = (w / aspect).max(64.0);
        let (rect, _) = ui.allocate_exact_size(avail, Sense::hover());
        let img_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(w, h));
        ui.painter()
            .rect_filled(rect, 0.0, Color32::from_rgb(12, 13, 16));
        ui.painter().image(
            self.display_tex_id,
            img_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );

        let mut split_dragging = false;
        if self.settings.split_on && img_rect.width() > 8.0 {
            let sx = img_rect.left() + img_rect.width() * self.settings.split_x.clamp(0.02, 0.98);
            let handle_rect = egui::Rect::from_center_size(
                egui::pos2(sx, img_rect.center().y),
                egui::vec2(14.0, img_rect.height()),
            );
            let split_resp = ui.interact(handle_rect, ui.id().with("split-handle"), Sense::drag());
            if split_resp.hovered() || split_resp.dragged() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
            }
            if split_resp.dragged() {
                split_dragging = true;
                if let Some(pos) = split_resp.interact_pointer_pos() {
                    self.settings.split_x =
                        ((pos.x - img_rect.left()) / img_rect.width()).clamp(0.02, 0.98);
                }
            }
            let knob = egui::Rect::from_center_size(
                egui::pos2(sx, img_rect.center().y),
                egui::vec2(10.0, 26.0),
            );
            ui.painter().rect_filled(
                knob,
                4.0,
                Color32::from_rgba_unmultiplied(235, 240, 245, 220),
            );
        }

        let resp = ui.interact(rect, ui.id().with("optix-vp"), Sense::click_and_drag());
        *self.viewport_focused = (resp.hovered() || resp.dragged()) && !split_dragging;
        // Vendored floating ViewportToolbar (bottom edge above HUD strip) with input gating.
        let toolbar_resp = crate::viewport_toolbar::ViewportToolbar::new("nv_demo_viewport_toolbar")
            .bottom_margin(34.0)
            .show(ui.ctx(), rect, self.viewport_toolbar, |ui| {
                // 1. AOV quick selector
                let aov_cur = AOV_HUD_TAGS
                    .get(self.settings.aov_mode as usize)
                    .copied()
                    .unwrap_or("BEAUTY");
                egui::ComboBox::from_id_salt("vt_aov")
                    .selected_text(format!("{} {aov_cur}", ic::STACK))
                    .width(96.0)
                    .show_ui(ui, |ui| {
                        for (idx, tag) in AOV_HUD_TAGS.iter().enumerate() {
                            ui.selectable_value(
                                &mut self.settings.aov_mode,
                                idx as u32,
                                *tag,
                            );
                        }
                    });
                ui.separator();

                // 2. Optical Lens selector
                egui::ComboBox::from_id_salt("vt_lens")
                    .selected_text(format!(
                        "{} {}",
                        ic::APERTURE,
                        self.settings.camera_projection.label()
                    ))
                    .width(94.0)
                    .show_ui(ui, |ui| {
                        for proj in [
                            CameraProjection::Pinhole,
                            CameraProjection::Fisheye180,
                            CameraProjection::Equirect360,
                            CameraProjection::SlitCam,
                        ] {
                            ui.selectable_value(
                                &mut self.settings.camera_projection,
                                proj,
                                proj.label(),
                            );
                        }
                    });
                ui.separator();

                // 3. Day / Sunset / Night quick pills (routed through smooth transition)
                for (tod, icon, tip) in [
                    (0u32, ic::SUN, "Daylight"),
                    (1u32, ic::SUN_HORIZON, "Golden Sunset"),
                    (2u32, ic::MOON_STARS, "Coastal Night"),
                ] {
                    if ui
                        .add(egui::Button::selectable(self.settings.csg_time_of_day == tod, icon).small())
                        .on_hover_text(tip)
                        .clicked()
                    {
                        self.bus.emit_ui(UiCommand::SetTimeOfDay(tod));
                    }
                }
                ui.separator();

                // 4. Pick / Poke / Freeze quick actions
                if ui
                    .add(egui::Button::selectable(*self.pick_possess_mode, ic::CROSSHAIR).small())
                    .on_hover_text("Pick object to possess (or double-click / double-tap in viewport)")
                    .clicked()
                {
                    *self.pick_possess_mode = !*self.pick_possess_mode;
                }
                if self.settings.csg_mode > 0
                    && ui
                        .add(egui::Button::new(ic::HAND_POINTING).small())
                        .on_hover_text("Poke nearest articulated actor (P)")
                        .clicked()
                {
                    let is_inspector_109 = self
                        .scene_descriptors
                        .get((self.settings.csg_mode as usize).saturating_sub(1))
                        .is_some_and(|d| d.id == "inspector" && self.settings.csg_asset_idx == 109);
                    self.bus.emit_ui(UiCommand::PokeCrowd {
                        impulse: self.settings.poke_impulse,
                        all: is_inspector_109,
                    });
                }
                if ui
                    .add(egui::Button::selectable(self.settings.csg_freeze_pose, ic::PAUSE).small())
                    .on_hover_text("Freeze dynamic traffic & crowd poses for clean accumulation")
                    .clicked()
                {
                    self.bus.emit_ui(UiCommand::ToggleFreezePose);
                }
                ui.separator();

                // 5. Compact Exposure EV drag
                ui.add(
                    egui::DragValue::new(&mut self.settings.exposure)
                        .range(0.05..=8.0)
                        .speed(0.02)
                        .prefix("EV "),
                )
                .on_hover_text("Exposure multiplier");

                // 6. Save PNG & SDG EXR Burst
                if ui
                    .add(egui::Button::new(ic::CAMERA).small())
                    .on_hover_text("Save PNG snapshot")
                    .clicked()
                {
                    self.bus.emit_ui(UiCommand::SaveStill(StillFormat::Png));
                }
                if self.settings.csg_mode > 0
                    && ui
                        .add(egui::Button::new(ic::FILM_STRIP).small())
                        .on_hover_text("Export multi-frame 9-channel OpenEXR + SDG JSON burst")
                        .clicked()
                {
                    self.bus.emit_ui(UiCommand::StartSdgBurst {
                        frames: self.settings.sdg_burst_frames,
                    });
                }
            });

        let toolbar_blocking = toolbar_resp.contains_pointer;
        if *self.pick_possess_mode && resp.hovered() && !toolbar_blocking {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
        }
        let pick_active = self.settings.csg_mode != 0 && *self.pick_possess_mode && !toolbar_blocking;
        // Double-click / double-tap anywhere in the viewport (or click in Pick crosshair mode)
        // possesses the vehicle, pedestrian, or camera mount under the cursor.
        let pick_clicked = !split_dragging
            && !toolbar_blocking
            && self.settings.csg_mode != 0
            && (resp.double_clicked_by(egui::PointerButton::Primary)
                || (*self.pick_possess_mode && resp.clicked_by(egui::PointerButton::Primary)));
        if pick_clicked
            && let Some(pos) = resp.interact_pointer_pos()
            && img_rect.width() > 2.0
            && img_rect.height() > 2.0
            && img_rect.contains(pos)
        {
            let norm_x = ((pos.x - img_rect.left()) / img_rect.width()).clamp(0.0, 1.0);
            let norm_y = ((pos.y - img_rect.top()) / img_rect.height()).clamp(0.0, 1.0);
            self.bus
                .emit_ui(UiCommand::PickPossess { norm_x, norm_y });
            *self.pick_possess_mode = false;
        }

        let kb_free = !ui.ctx().egui_wants_keyboard_input();
        if kb_free
            && self.settings.csg_mode > 0
            && ui.input(|i| i.key_pressed(egui::Key::P))
        {
            let is_inspector_109 = self
                .scene_descriptors
                .get((self.settings.csg_mode as usize).saturating_sub(1))
                .is_some_and(|d| d.id == "inspector" && self.settings.csg_asset_idx == 109);
            self.bus.emit_ui(UiCommand::PokeCrowd {
                impulse: self.settings.poke_impulse,
                all: is_inspector_109,
            });
        }
        if let Some(pinfo) = self.possess_info {
            if kb_free {
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.bus.emit_ui(UiCommand::EjectPossess);
                } else if ui.input(|i| i.key_pressed(egui::Key::OpenBracket)) {
                    self.bus.emit_ui(UiCommand::CyclePossessMount { delta: -1 });
                } else if ui.input(|i| i.key_pressed(egui::Key::CloseBracket)) {
                    self.bus.emit_ui(UiCommand::CyclePossessMount { delta: 1 });
                }
            }
            let allow_look = pinfo
                .mounts
                .get(pinfo.mount_idx)
                .is_none_or(|m| m.allow_free_look);
            if !split_dragging && !toolbar_blocking && allow_look && resp.dragged() {
                let d = resp.drag_delta();
                if d.x.abs() > 0.0 || d.y.abs() > 0.0 {
                    *self.possess_free_yaw =
                        (*self.possess_free_yaw + d.x * 0.0045).rem_euclid(std::f32::consts::TAU);
                    *self.possess_free_pitch = (*self.possess_free_pitch - d.y * 0.0045)
                        .clamp(-1.35, 1.35);
                }
            }
            // Pinch or scroll-wheel zoom while riding a vehicle or actor.
            if !split_dragging && !toolbar_blocking && resp.hovered() {
                let (scroll_y, zoom_delta) =
                    ui.input(|i| (i.smooth_scroll_delta.y, i.zoom_delta()));
                let pinch_delta = if (zoom_delta - 1.0).abs() > 1e-4 {
                    (1.0 - zoom_delta) * 14.0
                } else {
                    0.0
                };
                let wheel_delta = if scroll_y.abs() > 1e-3 {
                    -scroll_y * 0.025
                } else {
                    0.0
                };
                if pinch_delta != 0.0 || wheel_delta != 0.0 {
                    *self.possess_zoom_offset =
                        (*self.possess_zoom_offset + pinch_delta + wheel_delta).clamp(-1.5, 40.0);
                }
            }
        } else if !split_dragging && !toolbar_blocking && !pick_clicked && !pick_active {
            if resp.dragged() && self.camera_rail.playing {
                self.camera_rail.playing = false;
            }
            let fly_scene_radius = 2.0 * altitude_speed_multiplier(self.cam_pose.eye.y);
            let df = match self.cam_fixed_dt {
                Some(fixed_dt) => crate::camera_egui::drive_with_dt(
                    ui,
                    &resp,
                    self.cam_driver,
                    fly_scene_radius,
                    fixed_dt,
                ),
                None => crate::camera_egui::drive(ui, &resp, self.cam_driver, fly_scene_radius),
            };
            let rmb_down = ui.input(|i| i.pointer.secondary_down());
            let f_pressed =
                kb_free && *self.viewport_focused && ui.input(|i| i.key_pressed(egui::Key::F));
            let a_pressed = kb_free
                && *self.viewport_focused
                && !rmb_down
                && ui.input(|i| i.key_pressed(egui::Key::A));

            if df.requests.frame_selected || f_pressed {
                self.camera_rail.playing = false;
                let cur = self.cam_driver.controller().pose();
                let dist = cur.eye.length().max(0.25);
                let fwd = if cur.eye.length_squared() > 1e-6 {
                    -cur.eye.normalize()
                } else {
                    -glam::Vec3::Z
                };
                let aimed = CameraPose::from_eye_forward_up(
                    cur.eye,
                    fwd,
                    glam::Vec3::Y,
                    cur.fov_y,
                    cur.znear,
                    cur.zfar,
                );
                self.cam_driver.controller_mut().set_pose(aimed);
                self.cam_driver.controller_mut().set_orbit_distance(dist);
                *self.cam_pose = self.cam_driver.controller().pose();
                self.bus.emit_ui(UiCommand::SnapCamera);
            } else if a_pressed {
                self.camera_rail.playing = false;
                let center = glam::Vec3::from_array(self.settings.fractal_translate);
                let ext = 1.35 * self.settings.obj_scale.clamp(0.05, 50.0);
                self.cam_driver.controller_mut().apply_intent(
                    CameraIntent::FrameBounds {
                        min: center - glam::Vec3::splat(ext),
                        max: center + glam::Vec3::splat(ext),
                        margin: 1.2,
                    },
                    1.0 / 60.0,
                    ViewportSize::new(self.render_w, self.render_h),
                );
                *self.cam_pose = self.cam_driver.controller().pose();
                self.bus.emit_ui(UiCommand::SnapCamera);
            } else if df.requests.home {
                self.camera_rail.playing = false;
                let (home_pose, orbit_dist) =
                    resolve_home_pose(self.settings.csg_mode, self.scene_viewpoints, self.cam_home);
                self.cam_driver.controller_mut().set_pose(home_pose);
                if let Some(d) = orbit_dist {
                    self.cam_driver.controller_mut().set_orbit_distance(d);
                }
                *self.cam_pose = self.cam_driver.controller().pose();
                self.bus.emit_ui(UiCommand::SnapCamera);
            } else if !self.camera_rail.is_active() {
                *self.cam_pose = df.pose;
            }
        }

        // Top-left floating Possess Body HUD overlay with mount pills and Eject button
        if let Some(pinfo) = self.possess_info {
            let overlay_rect = egui::Rect::from_min_size(
                rect.left_top() + egui::vec2(10.0, 10.0),
                egui::vec2((rect.width() - 20.0).max(160.0), 56.0),
            );
            ui.scope_builder(egui::UiBuilder::new().max_rect(overlay_rect), |ui| {
                egui::Frame::NONE
                    .fill(Color32::from_rgba_unmultiplied(12, 16, 22, 210))
                    .stroke(egui::Stroke::new(1.0, Color32::from_rgb(66, 152, 98)))
                    .corner_radius(6.0)
                    .inner_margin(egui::Margin::symmetric(8, 5))
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing.x = 5.0;
                            let tag = if pinfo.is_dynamic { "RIDING" } else { "POSSESS" };
                            ui.label(
                                RichText::new(format!("{} {tag}: {}", ic::CAR, pinfo.assembly_name))
                                    .size(11.5)
                                    .strong()
                                    .color(Color32::from_rgb(174, 236, 194)),
                            );
                            for (idx, mount) in pinfo.mounts.iter().enumerate() {
                                let selected = idx == pinfo.mount_idx;
                                if ui
                                    .add(egui::Button::selectable(selected, &mount.label).small())
                                    .clicked()
                                {
                                    self.bus.emit_ui(UiCommand::SelectPossessMount { mount_idx: idx });
                                }
                            }
                            ui.checkbox(self.possess_auto_level, "Level")
                                .on_hover_text("Keep horizon level while riding vehicles.");
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new("Eject (Esc)").color(Color32::WHITE),
                                    )
                                    .fill(Color32::from_rgb(168, 46, 54))
                                    .small(),
                                )
                                .clicked()
                            {
                                self.bus.emit_ui(UiCommand::EjectPossess);
                            }
                        });
                    });
            });
        }

        // Compact bottom-left status badge inside viewport
        let hud_pos = rect.left_bottom() + egui::vec2(10.0, -26.0);
        let aov_tag = AOV_HUD_TAGS
            .get(self.settings.aov_mode as usize)
            .copied()
            .unwrap_or("BEAUTY");
        let lens_tag = if self.settings.camera_projection != CameraProjection::Pinhole {
            format!("  |  {}", self.settings.camera_projection.label())
        } else {
            String::new()
        };
        let possess_tag = if self.possess_info.is_some() {
            "  |  POSSESS"
        } else {
            ""
        };
        let sat_orbiting =
            self.settings.sat_on && self.settings.sat_orbit && self.settings.sat_count > 0;
        let demo_running = self.demo_anim.is_running();
        let active_supports_traffic = self.settings.csg_mode > 0
            && self
                .scene_descriptors
                .get((self.settings.csg_mode as usize).saturating_sub(1))
                .is_some_and(|d| d.supports_traffic);
        let traffic_moving = active_supports_traffic
            && !self.settings.csg_freeze_pose
            && self.settings.csg_traffic_on
            && self.settings.csg_traffic_speed > 0.001;
        let freeze_tag = if self.settings.csg_mode > 0 && self.settings.csg_freeze_pose {
            "  |  FROZEN"
        } else {
            ""
        };
        let preview_tag = if self.fast_preview { "  |  PREVIEW" } else { "" };
        let nav = self.cam_driver.navigator();
        let fly_mult = nav.fly_speed();
        let shift_ramp = nav.shift_fly_ramp();
        let alt_mult = altitude_speed_multiplier(self.cam_pose.eye.y);
        let in_fly_hud = self.cam_driver.is_hold_flying() || self.cam_driver.controller().is_fly();
        let fly_tag = if in_fly_hud || (fly_mult - 1.0).abs() > 0.01 || shift_ramp > 1.05 {
            let eff = fly_mult * alt_mult * shift_ramp;
            if shift_ramp > 1.05 {
                format!("  |  FLY {eff:.1}x (SHIFT {shift_ramp:.1}x)")
            } else {
                format!("  |  FLY {eff:.1}x")
            }
        } else {
            String::new()
        };
        let anim_tag = if self.settings.auto_rotate || sat_orbiting || demo_running || traffic_moving {
            let mut tags = Vec::with_capacity(4);
            if demo_running {
                tags.push("DEMO");
            }
            if self.settings.auto_rotate {
                tags.push("SPIN");
            }
            if sat_orbiting {
                tags.push("ORBIT");
            }
            if traffic_moving {
                tags.push("TRAFFIC");
            }
            Some(tags.join("+"))
        } else {
            None
        };
        let phase_label = format_engine_phase_label(
            self.engine_phase,
            self.accum_count,
            self.settings.max_accum,
            anim_tag.as_deref(),
        );
        let phase_icon = if self.engine_phase.transient_label().is_some() {
            ic::SPINNER
        } else if anim_tag.is_some() {
            ic::ARROWS_CLOCKWISE
        } else if self.accum_count >= self.settings.max_accum.max(1) {
            ic::CHECK_CIRCLE
        } else {
            ic::STACK
        };
        let status = format!(
            "{} {}  |  {}x{}  |  {}{}{}{}{}{}  |  UI {} FPS  |  RT {:.1} FPS ({:.1} ms)",
            phase_icon,
            phase_label,
            self.render_w,
            self.render_h,
            aov_tag,
            lens_tag,
            possess_tag,
            freeze_tag,
            fly_tag,
            preview_tag,
            self.ui_fps.round() as u32,
            self.rt_fps,
            self.rt_ms
        );
        let galley = ui.painter().layout_no_wrap(
            status,
            egui::FontId::monospace(11.0),
            Color32::from_rgb(225, 232, 240),
        );
        let bg = egui::Rect::from_min_size(
            hud_pos - egui::vec2(6.0, 3.0),
            galley.size() + egui::vec2(12.0, 6.0),
        );
        ui.painter()
            .rect_filled(bg, 4.0, Color32::from_rgba_unmultiplied(12, 14, 18, 190));
        ui.painter().galley(hud_pos, galley, Color32::WHITE);

        // Clickable Apple Maps / WGS84 coordinate readout in bottom Viewport status bar
        let active_scene_id = self
            .settings
            .csg_mode
            .checked_sub(1)
            .and_then(|idx| self.scene_descriptors.get(idx as usize))
            .map(|d| d.id);
        if let Some((origin_lat, origin_lon)) = active_scene_id.and_then(scene_wgs84_origin) {
            let (lat, lon, alt_m) =
                world_to_wgs84(origin_lat, origin_lon, self.cam_pose.eye.to_array());
            let copy_id = ui.id().with("wgs84-hud-copied-at");
            let now_s = ui.input(|i| i.time);
            let copied_recently = ui
                .ctx()
                .data(|d| d.get_temp::<f64>(copy_id))
                .is_some_and(|t| (now_s - t) < 1.5);
            let pill_text = if copied_recently {
                format!("{} Copied {lat:.5}, {lon:.5}", ic::CHECK)
            } else {
                format!("{} {lat:.5}, {lon:.5} | {alt_m:.1} m", ic::MAP_PIN)
            };
            let text_col = if copied_recently {
                Color32::from_rgb(138, 232, 168)
            } else {
                Color32::from_rgb(212, 228, 245)
            };
            let wgs_galley = ui.painter().layout_no_wrap(
                pill_text,
                egui::FontId::monospace(11.0),
                text_col,
            );
            let pill_size = wgs_galley.size() + egui::vec2(12.0, 6.0);
            let pref_left = rect.right() - pill_size.x - 10.0;
            let pill_min = if pref_left >= bg.right() + 8.0 {
                egui::pos2(pref_left, bg.top())
            } else {
                egui::pos2(bg.left(), bg.top() - pill_size.y - 4.0)
            };
            let pill_rect = egui::Rect::from_min_size(pill_min, pill_size);
            let pill_resp = ui
                .interact(pill_rect, ui.id().with("wgs84-hud-copy"), Sense::click())
                .on_hover_text("Click to copy Apple Maps coordinates (lat, lon)");
            if pill_resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            if pill_resp.clicked() {
                ui.ctx().copy_text(format!("{lat:.5}, {lon:.5}"));
                ui.ctx().data_mut(|d| d.insert_temp(copy_id, now_s));
            }
            let pill_fill = if pill_resp.hovered() {
                Color32::from_rgba_unmultiplied(24, 34, 48, 225)
            } else {
                Color32::from_rgba_unmultiplied(12, 14, 18, 190)
            };
            ui.painter().rect_filled(pill_rect, 4.0, pill_fill);
            if pill_resp.hovered() || copied_recently {
                let stroke_col = if copied_recently {
                    Color32::from_rgb(76, 178, 112)
                } else {
                    Color32::from_rgb(78, 128, 184)
                };
                ui.painter().rect_stroke(
                    pill_rect,
                    4.0,
                    egui::Stroke::new(1.0, stroke_col),
                    egui::StrokeKind::Inside,
                );
            }
            ui.painter().galley(
                pill_rect.min + egui::vec2(6.0, 3.0),
                wgs_galley,
                Color32::WHITE,
            );
        }
    }

    fn ui_fractal(&mut self, ui: &mut Ui) {
        let s = &mut *self.settings;
        let anim = &mut *self.demo_anim;
        // Master Hero Fractal visibility toggle at top of Fractal panel (aligned with section columns).
        ui.add_space(2.0);
        ui.indent(ui.id().with("fractal_master_toggle"), |ui| {
            if s.csg_mode == 0 {
                let mut always_on = true;
                ui.add_enabled_ui(false, |ui| {
                    prop_toggle(ui, "Fractal", &mut always_on, "Enabled (Classic)");
                });
            } else {
                prop_toggle(ui, "Fractal", &mut s.csg_show_fractal, "Enabled");
            }
        });
        ui.add_space(2.0);
        collapsing_section(ui, "sec_presets", ic::SPARKLE, "Presets", true, |ui| {
            equal_grid(ui, 2, 4, |ui, idx, w| match idx {
                0 => {
                    if ui
                        .add_sized([w, ROW_H + 2.0], egui::Button::new("Gold Bulb"))
                        .on_hover_text("Load a classic 3D Mandelbulb preset with warm metallic gold shading.")
                        .clicked()
                    {
                        s.fractal_kind = 0;
                        s.bulb_power = 8.0;
                        s.bulb_julia = false;
                        s.metallic = 0.78;
                        s.roughness = 0.14;
                        s.palette_shift = 0.08;
                        s.sun_azimuth = 36.0;
                        s.sun_elevation = 42.0;
                    }
                }
                1 => {
                    if ui
                        .add_sized([w, ROW_H + 2.0], egui::Button::new("Chrome Box"))
                        .on_hover_text("Load an architectural Mandelbox preset with polished chrome reflections.")
                        .clicked()
                    {
                        s.fractal_kind = 1;
                        s.box_scale = -2.0;
                        s.box_fold = 1.0;
                        s.metallic = 0.92;
                        s.roughness = 0.10;
                        s.palette_shift = 0.45;
                        s.sun_azimuth = -45.0;
                        s.sun_elevation = 35.0;
                    }
                }
                2 => {
                    if ui
                        .add_sized([w, ROW_H + 2.0], egui::Button::new("Obsidian KIFS"))
                        .on_hover_text("Load a geometric KIFS/Menger folding fractal with dark glossy material.")
                        .clicked()
                    {
                        s.fractal_kind = 2;
                        s.kifs_scale = 2.0;
                        s.kifs_rot1 = 18.0;
                        s.kifs_rot2 = -12.0;
                        s.metallic = 0.65;
                        s.roughness = 0.22;
                        s.palette_shift = 0.72;
                        s.sun_azimuth = 55.0;
                        s.sun_elevation = 28.0;
                    }
                }
                _ => {
                    if ui
                        .add_sized([w, ROW_H + 2.0], egui::Button::new("Julia Coral"))
                        .on_hover_text("Load an organic Julia-set Mandelbulb preset with soft coral tones.")
                        .clicked()
                    {
                        s.fractal_kind = 0;
                        s.bulb_power = 7.0;
                        s.bulb_julia = true;
                        s.bulb_c = [-0.22, 0.62, 0.18];
                        s.metallic = 0.30;
                        s.roughness = 0.26;
                        s.palette_shift = 0.22;
                    }
                }
            });
        });

        collapsing_section(ui, "sec_formula", ic::CUBE, "Formula", true, |ui| {
            equal_grid(ui, 3, 3, |ui, idx, w| {
                let (kind, label) = match idx {
                    0 => (0u32, "Mandelbulb"),
                    1 => (1u32, "Mandelbox"),
                    _ => (2u32, "KIFS Octa"),
                };
                if ui
                    .add_sized(
                        [w, ROW_H + 2.0],
                        egui::Button::selectable(s.fractal_kind == kind, label),
                    )
                    .clicked()
                {
                    s.fractal_kind = kind;
                }
            });
            ui.add_space(1.0);

            match s.fractal_kind {
                0 => {
                    anim_slider_f32(
                        ui,
                        anim,
                        "bulb_power",
                        "Power",
                        &mut s.bulb_power,
                        2.0..=16.0,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "bulb_bailout",
                        "Bailout",
                        &mut s.bulb_bailout,
                        2.0..=32.0,
                        1,
                        false,
                    );
                    prop_slider_u32(ui, "Iterations", &mut s.bulb_iter, 4..=32, false);
                    anim_slider_f32(
                        ui,
                        anim,
                        "bulb_theta_mult",
                        "Theta Mult",
                        &mut s.bulb_theta_mult,
                        0.25..=2.5,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "bulb_phi_mult",
                        "Phi Mult",
                        &mut s.bulb_phi_mult,
                        0.25..=2.5,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "bulb_theta_phase",
                        "Theta Ph",
                        &mut s.bulb_theta_phase,
                        -90.0..=90.0,
                        1,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "bulb_phi_phase",
                        "Phi Ph",
                        &mut s.bulb_phi_phase,
                        -180.0..=180.0,
                        1,
                        false,
                    );
                    prop_toggle(ui, "Julia Mode", &mut s.bulb_julia, "Enable Julia C");
                    if s.bulb_julia {
                        anim_vec3(
                            ui,
                            anim,
                            "bulb_c",
                            "Julia C",
                            &mut s.bulb_c,
                            -1.5..=1.5,
                            0.01,
                        );
                    }
                }
                1 => {
                    anim_slider_f32(
                        ui,
                        anim,
                        "box_scale",
                        "Scale",
                        &mut s.box_scale,
                        -3.5..=3.5,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "box_fold",
                        "Fold",
                        &mut s.box_fold,
                        0.2..=2.2,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "box_fold_mult",
                        "Fold Mult",
                        &mut s.box_fold_mult,
                        1.2..=3.0,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "box_min_r",
                        "Min R",
                        &mut s.box_min_r,
                        0.05..=1.2,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "box_fixed_r",
                        "Fixed R",
                        &mut s.box_fixed_r,
                        0.4..=2.5,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "box_iter_rot",
                        "Iter Rot",
                        &mut s.box_iter_rot,
                        -45.0..=45.0,
                        1,
                        false,
                    );
                    prop_slider_u32(ui, "Iterations", &mut s.box_iter, 4..=32, false);
                    anim_slider_f32(
                        ui,
                        anim,
                        "box_bailout",
                        "Bailout",
                        &mut s.box_bailout,
                        4.0..=128.0,
                        1,
                        false,
                    );
                }
                _ => {
                    anim_slider_f32(
                        ui,
                        anim,
                        "kifs_scale",
                        "Scale",
                        &mut s.kifs_scale,
                        1.3..=3.5,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "kifs_rot1",
                        "Rot Y",
                        &mut s.kifs_rot1,
                        -180.0..=180.0,
                        1,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "kifs_rot2",
                        "Rot X",
                        &mut s.kifs_rot2,
                        -180.0..=180.0,
                        1,
                        false,
                    );
                    anim_vec3(
                        ui,
                        anim,
                        "kifs_offset",
                        "Offset",
                        &mut s.kifs_offset,
                        0.1..=2.0,
                        0.01,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "kifs_thickness",
                        "Thickness",
                        &mut s.kifs_thickness,
                        0.005..=0.25,
                        3,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "kifs_z_bias",
                        "Z Bias",
                        &mut s.kifs_z_bias,
                        0.0..=1.5,
                        2,
                        false,
                    );
                    prop_slider_u32(ui, "Iterations", &mut s.kifs_iter, 4..=32, false);
                    anim_slider_f32(
                        ui,
                        anim,
                        "kifs_bailout",
                        "Bailout",
                        &mut s.kifs_bailout,
                        4.0..=32.0,
                        1,
                        false,
                    );
                }
            }
        });

        collapsing_section(
            ui,
            "sec_transform",
            ic::ARROWS_OUT_CARDINAL,
            "Transform",
            true,
            |ui| {
                anim_vec3(
                    ui,
                    anim,
                    "fractal_translate",
                    "Translate",
                    &mut s.fractal_translate,
                    -500.0..=500.0,
                    0.05,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "rot_x",
                    "Rot X",
                    &mut s.rot_x,
                    -180.0..=180.0,
                    1,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "rot_y",
                    "Rot Y",
                    &mut s.rot_y,
                    -180.0..=180.0,
                    1,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "rot_z",
                    "Rot Z",
                    &mut s.rot_z,
                    -180.0..=180.0,
                    1,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "obj_scale",
                    "Scale",
                    &mut s.obj_scale,
                    0.05..=50.0,
                    2,
                    true,
                );
                ui.add_space(2.0);
                let btn_w = ui.available_width();
                if ui
                    .add_sized(
                        [btn_w, ROW_H + 4.0],
                        egui::Button::new(format!("{} Reset Transform", ic::ARROW_COUNTER_CLOCKWISE)),
                    )
                    .on_hover_text("Reset Hero Fractal Translate, Rotate, and Scale to identity.")
                    .clicked()
                {
                    s.fractal_translate = [0.0, 0.0, 0.0];
                    s.rot_x = 0.0;
                    s.rot_y = 0.0;
                    s.rot_z = 0.0;
                    s.obj_scale = 1.0;
                }
            },
        );

        collapsing_section(
            ui,
            "sec_detail",
            ic::SLIDERS_HORIZONTAL,
            "Detail & Shading",
            true,
            |ui| {
                anim_slider_f32(
                    ui,
                    anim,
                    "detail_eps",
                    "Detail Eps",
                    &mut s.detail_eps,
                    0.00002..=0.005,
                    5,
                    true,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "norm_eps",
                    "Normal Eps",
                    &mut s.norm_eps,
                    0.00002..=0.005,
                    5,
                    true,
                );
                prop_slider_u32(ui, "March Steps", &mut s.march_steps, 32..=512, false);
                anim_slider_f32(
                    ui,
                    anim,
                    "fudge",
                    "Fudge",
                    &mut s.fudge,
                    0.3..=1.1,
                    2,
                    false,
                );
                prop_slider_u32(ui, "Bounces", &mut s.bounces, 1..=16, false);
                anim_slider_f32(
                    ui,
                    anim,
                    "metallic",
                    "Metallic",
                    &mut s.metallic,
                    0.0..=1.0,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "roughness",
                    "Roughness",
                    &mut s.roughness,
                    0.02..=0.85,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "trap_scale",
                    "Trap Scale",
                    &mut s.trap_scale,
                    0.1..=3.0,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "palette_shift",
                    "Hue Shift",
                    &mut s.palette_shift,
                    0.0..=1.0,
                    2,
                    false,
                );
            },
        );
    }

    fn aim_camera(&mut self, eye: [f32; 3], target: [f32; 3]) {
        self.camera_rail.playing = false;
        let eye_v = glam::Vec3::from_array(eye);
        let tgt_v = glam::Vec3::from_array(target);
        let delta = tgt_v - eye_v;
        let dist = delta.length().max(0.25);
        let fwd = delta / dist;
        let cur = self.cam_driver.controller().pose();
        let pose = CameraPose::from_eye_forward_up(
            eye_v,
            fwd,
            glam::Vec3::Y,
            cur.fov_y,
            cur.znear,
            cur.zfar.max(2000.0),
        );
        self.cam_driver.controller_mut().set_pose(pose);
        self.cam_driver.controller_mut().set_orbit_distance(dist);
        *self.cam_pose = self.cam_driver.controller().pose();
        self.bus.emit_ui(UiCommand::SnapCamera);
    }

    fn ui_render_floor(&mut self, ui: &mut Ui) {
        let mut pending_cam: Option<([f32; 3], [f32; 3])> = None;
        let mut request_auto_frame = false;
        {
            let s = &mut *self.settings;
            let anim = &mut *self.demo_anim;
            let descs = self.scene_descriptors;
            let viewpoints = self.scene_viewpoints;

            collapsing_section(ui, "sec_csg", ic::BUILDINGS, "CSG World", true, |ui| {
                let prev_mode = s.csg_mode;
                let total_choices = descs.len() + 1;
                let active_name = if s.csg_mode == 0 {
                    "Off (Classic)"
                } else {
                    descs
                        .get((s.csg_mode as usize).saturating_sub(1))
                        .map_or("Off (Classic)", |d| d.name)
                };

                prop_row(ui, "Scene", |ui| {
                    let gap = 4.0;
                    let combo_w = (ui.available_width() - ANIM_BTN_W - gap).max(32.0);
                    ui.spacing_mut().item_spacing.x = gap;
                    egui::ComboBox::from_id_salt("csg_scene_combo")
                        .width(combo_w)
                        .truncate()
                        .selected_text(active_name)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut s.csg_mode, 0u32, "Off (Classic)")
                                .on_hover_text("Classic SDF fractal + scratched metal stage floor");
                            for (i, d) in descs.iter().enumerate() {
                                ui.selectable_value(&mut s.csg_mode, (i + 1) as u32, d.name)
                                    .on_hover_text(d.description);
                            }
                        });
                    anim_toggle_btn_disabled(ui);
                });

                // Quick-pick scene buttons driven strictly by `SceneRegistry::descriptors()`.
                let cols = total_choices.clamp(1, 3);
                equal_grid(ui, cols, total_choices, |ui, idx, w| {
                    let (mode, label, tip) = if idx == 0 {
                        (0u32, "Off", "Classic SDF fractal + stage floor")
                    } else if let Some(d) = descs.get(idx - 1) {
                        let short = match d.id {
                            "gundo" => "Gundo",
                            "gundo_osm" => "Gundo OSM",
                            "marina" => "Marina",
                            "sf_house_of_ai" => "SF AI",
                            "inspector" => "Inspector",
                            _ => d.name,
                        };
                        (idx as u32, short, d.description)
                    } else {
                        (0u32, "Off", "")
                    };
                    if ui
                        .add_sized(
                            [w, ROW_H + 2.0],
                            egui::Button::selectable(s.csg_mode == mode, label),
                        )
                        .on_hover_text(tip)
                        .clicked()
                    {
                        s.csg_mode = mode;
                    }
                });

                let active_desc = if s.csg_mode == 0 {
                    None
                } else {
                    descs.get((s.csg_mode as usize).saturating_sub(1))
                };

                if s.csg_mode != prev_mode
                    && let Some(desc) = active_desc
                {
                    let tod_u32 = match desc.default_tod {
                        TimeOfDay::Day => 0,
                        TimeOfDay::Sunset => 1,
                        TimeOfDay::Night => 2,
                    };
                    s.csg_wetness = desc.default_wetness.clamp(0.0, 1.0);
                    apply_csg_time_of_day_preset(s, tod_u32);
                    if !desc.variant_labels.is_empty() {
                        s.csg_asset_idx =
                            s.csg_asset_idx.min((desc.variant_labels.len() - 1) as u32);
                    }
                    request_auto_frame = true;
                }

                if let Some(desc) = active_desc {
                    ui.add_space(2.0);
                    if !desc.variant_labels.is_empty() {
                        let prev_var = s.csg_asset_idx;
                        let max_idx = (desc.variant_labels.len() - 1) as u32;
                        s.csg_asset_idx = s.csg_asset_idx.min(max_idx);
                        prop_row(ui, "Kit Asset", |ui| {
                            let gap = 4.0;
                            let combo_w = (ui.available_width() - ANIM_BTN_W - gap).max(32.0);
                            ui.spacing_mut().item_spacing.x = gap;
                            let cur_label = desc
                                .variant_labels
                                .get(s.csg_asset_idx as usize)
                                .copied()
                                .unwrap_or("Asset");
                            egui::ComboBox::from_id_salt("csg_variant_combo")
                                .width(combo_w)
                                .truncate()
                                .selected_text(cur_label)
                                .show_ui(ui, |ui| {
                                    for (i, label) in desc.variant_labels.iter().enumerate() {
                                        ui.selectable_value(&mut s.csg_asset_idx, i as u32, *label);
                                    }
                                });
                            anim_toggle_btn_disabled(ui);
                        });
                        if s.csg_asset_idx != prev_var {
                            request_auto_frame = true;
                        }
                    }

                    equal_grid(ui, 3, 3, |ui, idx, w| {
                        let (tod, label) = match idx {
                            0 => (0u32, "Day"),
                            1 => (1u32, "Sunset"),
                            _ => (2u32, "Night"),
                        };
                        if ui
                            .add_sized(
                                [w, ROW_H + 2.0],
                                egui::Button::selectable(s.csg_time_of_day == tod, label),
                            )
                            .clicked()
                        {
                            self.bus.emit_ui(UiCommand::SetTimeOfDay(tod));
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut s.tod_transition_enabled, "Transition")
                            .on_hover_text(
                                "Smoothly animate sky and sun lighting across Day / Sunset / Night.",
                            );
                        ui.add_enabled(
                            s.tod_transition_enabled,
                            egui::DragValue::new(&mut s.tod_transition_secs)
                                .range(0.1..=15.0)
                                .speed(0.1)
                                .suffix(" s"),
                        )
                        .on_hover_text("Transition duration in seconds.");
                    });
                    // 4-way Road Mode selector (`0=3D, 1=C1, 2=Hybrid, 3=F-Rep`)
                    let prev_road_mode = s.csg_road_mode.min(3);
                    ui.label(RichText::new("Road Mode").small().weak());
                    equal_grid(ui, 4, 4, |ui, idx, w| {
                        let (mode, label, tip) = match idx {
                            0 => (
                                0u8,
                                "3D",
                                "Watertight 3D triangular prism road ribbons and junction decks",
                            ),
                            1 => (
                                1u8,
                                "C1",
                                "C1 tangent-plane junction blending with continuous vertical slope",
                            ),
                            2 => (
                                2u8,
                                "Hybrid",
                                "Hybrid C1 + analytical SDF roadbed with grounded sidewalk aprons (default)",
                            ),
                            _ => (
                                3u8,
                                "F-Rep",
                                "Analytical RoadTerrainTile F-Rep (single-prim SBT 5 tiles with exact curved markings, ~55k -> <15k instances)",
                            ),
                        };
                        if ui
                            .add_sized(
                                [w, ROW_H + 2.0],
                                egui::Button::selectable(prev_road_mode == mode, label),
                            )
                            .on_hover_text(tip)
                            .clicked()
                            && prev_road_mode != mode
                        {
                            s.csg_road_mode = mode;
                            self.bus.emit_ui(UiCommand::SetRoadMode(mode));
                        }
                    });
                    anim_slider_f32(
                        ui,
                        anim,
                        "csg_wetness",
                        "Wetness",
                        &mut s.csg_wetness,
                        0.0..=1.0,
                        2,
                        false,
                    );
                    if desc.supports_traffic {
                        prop_toggle(ui, "Traffic", &mut s.csg_traffic_on, "Animated 3D Traffic");
                        if s.csg_traffic_on && !s.csg_freeze_pose {
                            anim_slider_f32(
                                ui,
                                anim,
                                "csg_traffic_speed",
                                "Traffic Spd",
                                &mut s.csg_traffic_speed,
                                0.0..=3.0,
                                2,
                                false,
                            );
                        }
                    }
                    let mut freeze_pose = s.csg_freeze_pose;
                    prop_toggle(
                        ui,
                        "Freeze Pose",
                        &mut freeze_pose,
                        "Pause Traffic & Crowd (SDG Accum)",
                    );
                    if freeze_pose != s.csg_freeze_pose {
                        self.bus.emit_ui(UiCommand::ToggleFreezePose);
                    }
                    ui.add_space(2.0);
                    let crowd_badge = if self.crowd_actor_count > 0 {
                        format!(
                            "Crowd Backend ({} {:.2} ms, {}a)",
                            self.crowd_active_backend.label(),
                            self.crowd_step_ms,
                            self.crowd_actor_count
                        )
                    } else {
                        "Crowd Backend".to_string()
                    };
                    ui.label(RichText::new(crowd_badge).small().weak());
                    equal_grid(ui, 3, 3, |ui, idx, w| {
                        let b = CrowdBackend::ALL[idx];
                        let tip = match b {
                            CrowdBackend::Auto => "Prefer GPU-direct CUDA crowd stepping when available; falls back to CPU and reports active backend.",
                            CrowdBackend::Cpu => "Run analytical CPU crowd stepping (deterministic reference).",
                            CrowdBackend::Cuda => "Require GPU-direct NVRTC CUDA crowd stepping (returns error if unavailable; never downgrades).",
                        };
                        if ui
                            .add_sized(
                                [w, ROW_H + 2.0],
                                egui::Button::selectable(s.crowd_backend == b, b.label()),
                            )
                            .on_hover_text(tip)
                            .clicked()
                        {
                            s.crowd_backend = b;
                        }
                    });
                    ui.label(RichText::new("SDG Occluder").small().weak());
                    equal_grid(ui, 3, 3, |ui, idx, w| {
                        let b = SdgOccluderBackend::ALL[idx];
                        let tip = match b {
                            SdgOccluderBackend::Auto => "Prefer hardware OptiX 9 batched keypoint occlusion; falls back to CPU and reports active backend.",
                            SdgOccluderBackend::Cpu => "Analytical CPU CSG ray-occlusion oracle (csg-core).",
                            SdgOccluderBackend::Optix => "Require hardware OptiX 9 __raygen__sdg_keypoints occlusion (never silently downgrades).",
                        };
                        if ui
                            .add_sized(
                                [w, ROW_H + 2.0],
                                egui::Button::selectable(s.sdg_occluder_backend == b, b.label()),
                            )
                            .on_hover_text(tip)
                            .clicked()
                        {
                            s.sdg_occluder_backend = b;
                        }
                    });
                    let is_inspector_109 = desc.id == "inspector" && s.csg_asset_idx == 109;
                    equal_grid(ui, 2, 2, |ui, idx, w| match idx {
                        0 => {
                            let lbl = if is_inspector_109 {
                                format!("{}  Poke All (P)", ic::HAND_POINTING)
                            } else {
                                format!("{}  Poke (P)", ic::HAND_POINTING)
                            };
                            if ui
                                .add_sized([w, ROW_H + 2.0], egui::Button::new(lbl).truncate())
                                .on_hover_text(
                                    "Apply a stumble/flinch impulse to the nearest actor along view (or all actors in Inspector 109). Hotkey: P.",
                                )
                                .clicked()
                            {
                                self.bus.emit_ui(UiCommand::PokeCrowd {
                                    impulse: s.poke_impulse,
                                    all: is_inspector_109,
                                });
                            }
                        }
                        _ => {
                            let lbl = format!("{}  Stress 2000", ic::USERS_THREE);
                            if ui
                                .add_sized([w, ROW_H + 2.0], egui::Button::new(lbl).truncate())
                                .on_hover_text(
                                    "Spawn a 2,000-actor walking crowd stress grid around the camera to benchmark GPU-direct CUDA vs CPU.",
                                )
                                .clicked()
                            {
                                self.bus
                                    .emit_ui(UiCommand::SpawnCrowdStress { target_count: 2000 });
                            }
                        }
                    });
                    prop_slider_f32(
                        ui,
                        "Impulse",
                        &mut s.poke_impulse,
                        0.5..=4.0,
                        2,
                        "m/s",
                    );
                    prop_slider_u32(ui, "World Seed", &mut s.csg_seed, 1..=9999, false);
                    anim_slider_f32(
                        ui,
                        anim,
                        "csg_lod_bias",
                        "LOD Bias",
                        &mut s.csg_lod_bias,
                        0.25..=4.0,
                        2,
                        false,
                    );
                    if !viewpoints.is_empty() {
                        ui.add_space(2.0);
                        let vp_cols = if viewpoints.len() == 1 { 1 } else { 2 };
                        equal_grid(ui, vp_cols, viewpoints.len(), |ui, idx, w| {
                            if let Some(vp) = viewpoints.get(idx)
                                && ui
                                    .add_sized(
                                        [w, ROW_H + 2.0],
                                        egui::Button::new(format!("{}  {}", ic::CAMERA, vp.label))
                                            .truncate(),
                                    )
                                    .on_hover_text(&vp.label)
                                    .clicked()
                            {
                                pending_cam = Some((
                                    [vp.eye.x, vp.eye.y, vp.eye.z],
                                    [vp.target.x, vp.target.y, vp.target.z],
                                ));
                            }
                        });
                    }
                }
            });
        }
        if request_auto_frame {
            self.bus.emit_ui(UiCommand::SelectScene {
                scene_idx: self.settings.csg_mode,
                variant_idx: self.settings.csg_asset_idx,
            });
        }
        if let Some((eye, tgt)) = pending_cam {
            self.aim_camera(eye, tgt);
        }

        let s = &mut *self.settings;
        let anim = &mut *self.demo_anim;

        collapsing_section(ui, "sec_sky", ic::SUN, "Sky & Lighting", true, |ui| {
            prop_row(ui, "Background", |ui| {
                let gap = 4.0;
                let combo_w = (ui.available_width() - ANIM_BTN_W - gap).max(32.0);
                ui.spacing_mut().item_spacing.x = gap;
                let modes = [
                    "Daylight",
                    "Sunset",
                    "Studio",
                    "Nebula",
                    "Cyber Grid",
                    "Elevated (4k)",
                    "Unprogress (4k)",
                    "Offscreen Colonies",
                    "Coastal Night",
                ];
                let idx = (s.sky_mode as usize).min(modes.len() - 1);
                egui::ComboBox::from_id_salt("sky_mode_combo")
                    .width(combo_w)
                    .truncate()
                    .selected_text(modes[idx])
                    .show_ui(ui, |ui| {
                        for (i, label) in modes.iter().enumerate() {
                            ui.selectable_value(&mut s.sky_mode, i as u32, *label);
                        }
                    });
                anim_toggle_btn_disabled(ui);
            });
            anim_slider_f32(
                ui,
                anim,
                "sun_intensity",
                "Sun Power",
                &mut s.sun_intensity,
                0.0..=4.0,
                2,
                false,
            );
            anim_slider_f32(
                ui,
                anim,
                "sun_azimuth",
                "Sun Azimuth",
                &mut s.sun_azimuth,
                -180.0..=180.0,
                1,
                false,
            );
            anim_slider_f32(
                ui,
                anim,
                "sun_elevation",
                "Sun Elev",
                &mut s.sun_elevation,
                2.0..=88.0,
                1,
                false,
            );
            anim_slider_f32(
                ui,
                anim,
                "sky_intensity",
                "Sky Power",
                &mut s.sky_intensity,
                0.1..=2.0,
                2,
                false,
            );
            anim_slider_f32(
                ui,
                anim,
                "horizon_haze",
                "Haze",
                &mut s.horizon_haze,
                0.4..=2.5,
                2,
                false,
            );
            anim_slider_f32(
                ui,
                anim,
                "exposure",
                "Exposure",
                &mut s.exposure,
                0.25..=2.0,
                2,
                false,
            );

            match s.sky_mode {
                0 | 1 => {
                    anim_slider_f32(ui, anim, "env_cloud_cover", "Cloud Cover", &mut s.env.cloud_cover, 0.0..=1.0, 2, false);
                    anim_slider_f32(ui, anim, "env_cloud_density", "Cloud Density", &mut s.env.cloud_density, 0.0..=1.5, 2, false);
                }
                2 => {
                    anim_slider_f32(ui, anim, "env_softbox_power", "Panel Power", &mut s.env.softbox_power, 0.2..=3.0, 2, false);
                    anim_slider_f32(ui, anim, "env_softbox_softness", "Edge Softness", &mut s.env.softbox_softness, 0.02..=0.5, 2, false);
                }
                3 => {
                    anim_slider_f32(ui, anim, "env_nebula_density", "Nebula Density", &mut s.env.nebula_density, 0.2..=2.5, 2, false);
                    anim_slider_f32(ui, anim, "env_nebula_contrast", "Contrast", &mut s.env.nebula_contrast, 0.5..=3.0, 2, false);
                    anim_slider_f32(ui, anim, "env_nebula_dust", "Dark Dust", &mut s.env.nebula_dust, 0.0..=3.0, 2, false);
                    anim_slider_f32(ui, anim, "env_nebula_scale", "Cloud Scale", &mut s.env.nebula_scale, 0.5..=4.0, 2, false);
                    anim_slider_f32(ui, anim, "env_star_density", "Star Density", &mut s.env.star_density, 0.0..=2.0, 2, false);
                    anim_slider_f32(ui, anim, "env_star_brightness", "Star Bright", &mut s.env.star_brightness, 0.0..=4.0, 2, false);
                }
                4 => {
                    anim_slider_f32(ui, anim, "env_grid_glow", "Grid Glow", &mut s.env.grid_glow, 0.1..=3.0, 2, false);
                    anim_slider_f32(ui, anim, "env_grid_density", "Grid Density", &mut s.env.grid_density, 4.0..=32.0, 1, false);
                }
                5 => {
                    anim_slider_f32(ui, anim, "env_elev_cloud_cover", "Cloud Cover", &mut s.env.elev_cloud_cover, 0.0..=1.0, 2, false);
                    anim_slider_f32(ui, anim, "env_elev_fog", "Valley Fog", &mut s.env.elev_fog, 0.2..=2.5, 2, false);
                }
                6 => {
                    anim_slider_f32(ui, anim, "env_city_fog", "City Fog", &mut s.env.city_fog, 0.2..=2.5, 2, false);
                    anim_slider_f32(ui, anim, "env_city_windows", "Window Glow", &mut s.env.city_windows, 0.0..=3.0, 2, false);
                }
                7 => {
                    anim_slider_f32(ui, anim, "env_star_density", "Star Density", &mut s.env.star_density, 0.0..=2.0, 2, false);
                    anim_slider_f32(ui, anim, "env_star_brightness", "Star Bright", &mut s.env.star_brightness, 0.0..=4.0, 2, false);
                    anim_slider_f32(ui, anim, "env_colony_glow", "Colony Glow", &mut s.env.colony_glow, 0.0..=3.0, 2, false);
                    anim_slider_f32(ui, anim, "env_planet_limb", "Planet Limb", &mut s.env.planet_limb, 0.0..=2.5, 2, false);
                }
                _ => {}
            }
        });

        collapsing_section(ui, "sec_dof", ic::APERTURE, "Lens DoF", true, |ui| {
            anim_slider_f32(
                ui,
                anim,
                "aperture",
                "Aperture",
                &mut s.aperture,
                0.0..=0.12,
                3,
                false,
            );
            anim_slider_f32(
                ui,
                anim,
                "focus_dist",
                "Focus Dist",
                &mut s.focus_dist,
                0.5..=12.0,
                2,
                false,
            );
        });

        collapsing_section(
            ui,
            "sec_floor",
            ic::SQUARES_FOUR,
            "Scratched Metal Floor",
            true,
            |ui| {
                anim_slider_f32(
                    ui,
                    anim,
                    "floor_roughness",
                    "Roughness",
                    &mut s.floor_roughness,
                    0.03..=0.80,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "floor_scratch",
                    "Scratches",
                    &mut s.floor_scratch,
                    0.0..=1.5,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "floor_scale",
                    "Tile Scale",
                    &mut s.floor_scale,
                    0.3..=5.0,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "floor_metallic",
                    "Metallic",
                    &mut s.floor_metallic,
                    0.0..=1.0,
                    2,
                    false,
                );
            },
        );

        collapsing_section(ui, "sec_scene", ic::GLOBE, "Scene", true, |ui| {
            prop_toggle(ui, "Glass Sat", &mut s.sat_on, "Dielectric Sphere");
            if s.sat_on {
                prop_toggle(
                    ui,
                    "Analytic",
                    &mut s.glass_analytic,
                    "Smooth Analytic Sphere",
                );
                prop_toggle(ui, "Orbit", &mut s.sat_orbit, "Animate Orbit");
                prop_slider_u32(ui, "Spheres Num", &mut s.sat_count, 0..=24, false);
                if !s.glass_analytic {
                    prop_slider_u32(ui, "U Segs", &mut s.sat_u_segs, 4..=96, false);
                    prop_slider_u32(ui, "V Segs", &mut s.sat_v_segs, 4..=96, false);
                }
                prop_slider_u32(ui, "Orbit Seed", &mut s.sat_seed, 0..=9999, false);
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_size_min",
                    "Min Size",
                    &mut s.sat_size_min,
                    0.001..=0.80,
                    3,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_size_max",
                    "Max Size",
                    &mut s.sat_size_max,
                    0.001..=0.80,
                    3,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_orbit_min",
                    "Min Orbit",
                    &mut s.sat_orbit_min,
                    0.6..=4.5,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_orbit_max",
                    "Max Orbit",
                    &mut s.sat_orbit_max,
                    0.6..=4.5,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_height_min",
                    "Min Height",
                    &mut s.sat_height_min,
                    -1.0..=2.0,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_height_max",
                    "Max Height",
                    &mut s.sat_height_max,
                    -1.0..=2.0,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_speed_min",
                    "Min Speed",
                    &mut s.sat_speed_min,
                    -3.0..=3.0,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_speed_max",
                    "Max Speed",
                    &mut s.sat_speed_max,
                    -3.0..=3.0,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_bob_amp_min",
                    "Bob Amp Min",
                    &mut s.sat_bob_amp_min,
                    0.0..=0.80,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_bob_amp_max",
                    "Bob Amp Max",
                    &mut s.sat_bob_amp_max,
                    0.0..=0.80,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_bob_freq_min",
                    "Bob Frq Min",
                    &mut s.sat_bob_freq_min,
                    0.0..=5.0,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_bob_freq_max",
                    "Bob Frq Max",
                    &mut s.sat_bob_freq_max,
                    0.0..=5.0,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_hue_spread",
                    "Hue Spread",
                    &mut s.sat_hue_spread,
                    0.0..=1.0,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "glass_ior",
                    "Glass IOR",
                    &mut s.glass_ior,
                    1.05..=2.40,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "glass_roughness",
                    "Glass Rough",
                    &mut s.glass_roughness,
                    0.0..=0.35,
                    3,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "glass_absorb",
                    "Absorb",
                    &mut s.glass_absorb,
                    0.0..=12.0,
                    2,
                    false,
                );
            }
            prop_toggle(ui, "Neon Ring", &mut s.neon_on, "Emissive Accent");
            prop_row(ui, "OMM Halo", |ui| {
                let gap = 4.0;
                let inner_w = (ui.available_width() - ANIM_BTN_W - gap).max(60.0);
                ui.spacing_mut().item_spacing.x = gap;
                let lbl = match s.omm_mode {
                    1 => "OMM (HW)",
                    2 => "Any-Hit (SW)",
                    _ => "Off",
                };
                let tint = (s.omm_mode != 0).then(|| active_accent_tint(ui));
                let step = cycle_toggle_btn(
                    ui,
                    inner_w,
                    "Halo",
                    lbl,
                    "Adds a perforated gold ring around the fractal to compare hardware Opacity Micro-Maps (OMM) vs software Any-Hit cutouts.",
                    true,
                    tint,
                );
                if step != 0 {
                    s.omm_mode = ((s.omm_mode as i32 + step).rem_euclid(3)) as u32;
                }
                anim_toggle_btn_disabled(ui);
            });
        });

        collapsing_section(
            ui,
            "sec_bloom",
            ic::SUN_DIM,
            "FFT Bloom (cuFFT)",
            true,
            |ui| {
                prop_toggle(ui, "Bloom", &mut s.bloom_on, "Enable Diffraction Bloom");
                if s.bloom_on {
                    anim_slider_f32(
                        ui,
                        anim,
                        "bloom_threshold",
                        "Threshold",
                        &mut s.bloom_threshold,
                        0.2..=5.0,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "bloom_intensity",
                        "Intensity",
                        &mut s.bloom_intensity,
                        0.0..=2.0,
                        2,
                        false,
                    );
                    prop_slider_u32(ui, "Blades", &mut s.bloom_blades, 3..=12, false);
                }
            },
        );

        collapsing_section(ui, "sec_split", ic::COLUMNS, "A/B Split", true, |ui| {
            prop_toggle(ui, "Split View", &mut s.split_on, "Drag Handle in Viewport");
            if s.split_on {
                anim_slider_f32(
                    ui,
                    anim,
                    "split_x",
                    "Split X",
                    &mut s.split_x,
                    0.02..=0.98,
                    2,
                    false,
                );
                prop_row(ui, "Right AOV", |ui| {
                    let lbl = match s.right_is_aov {
                        1 => "Albedo",
                        2 => "Normal",
                        _ => "Raw",
                    };
                    let step = cycle_toggle_btn(
                        ui,
                        ui.available_width(),
                        "Right",
                        lbl,
                        "Choose what appears on the right half of the split screen: raw noisy render, base surface color (Albedo), or surface direction (Normal).",
                        true,
                        None,
                    );
                    if step != 0 {
                        s.right_is_aov = ((s.right_is_aov as i32 + step).rem_euclid(3)) as u32;
                    }
                });
            }
        });

        collapsing_section(ui, "sec_pt", ic::LIGHTNING, "Path Tracer", true, |ui| {
            // Compact 3-toggle row replacing 13 separate buttons (Scale / AOV / AI mode)
            equal_grid(ui, 3, 3, |ui, idx, w| match idx {
                0 => {
                    const SCALES: [f32; 4] = [0.5, 1.0, 1.5, 2.0];
                    let cur = SCALES
                        .iter()
                        .position(|&v| (s.render_scale - v).abs() < 0.05)
                        .unwrap_or(1);
                    let lbl = format!("{:.1}x", SCALES[cur]);
                    let tip = if s.dlss_on || s.dlss_rr_on {
                        "Render resolution scale (controlled automatically while DLSS is active)."
                    } else {
                        "Internal render resolution multiplier. Lower = faster FPS, higher = sharper edges."
                    };
                    let step = cycle_toggle_btn(
                        ui,
                        w,
                        "Scale",
                        &lbl,
                        tip,
                        !s.dlss_on && !s.dlss_rr_on,
                        None,
                    );
                    if step != 0 {
                        let next = (cur as i32 + step).rem_euclid(SCALES.len() as i32) as usize;
                        s.render_scale = SCALES[next];
                    }
                }
                1 => {
                    let cur = (s.aov_mode as usize).min(AOV_LABELS.len() - 1);
                    let tint = (cur != 0).then(|| active_accent_tint(ui));
                    let step = cycle_toggle_btn(
                        ui,
                        w,
                        "AOV",
                        AOV_LABELS[cur],
                        "Inspect all 11 SDG/VFX passes: Beauty, Albedo, Normal, Depth, Motion, SemanticId, InstanceId, WorldPos, Range (LiDAR), Material (PBR), and Cryptomatte.",
                        true,
                        tint,
                    );
                    if step != 0 {
                        s.aov_mode =
                            ((cur as i32 + step).rem_euclid(AOV_LABELS.len() as i32)) as u32;
                    }
                }
                _ => {
                    let cur = if s.dlss_rr_on {
                        3i32
                    } else if s.dlss_on {
                        2
                    } else if s.denoiser_on {
                        1
                    } else {
                        0
                    };
                    let lbl = match cur {
                        1 => "OptiX",
                        2 => "DLSS SR",
                        3 => "DLSS RR",
                        _ => "Off",
                    };
                    let tip = if self.dlss_rr_available {
                        "AI noise removal & upscaling: Off (raw path tracing), OptiX (denoiser), DLSS SR (upscaler), or DLSS RR (AI denoiser + upscaler in one pass)."
                    } else {
                        "AI noise removal & upscaling: Off (raw path tracing), OptiX (denoiser), or DLSS SR (upscaler). DLSS RR is unavailable on this GPU/driver."
                    };
                    let tint = (cur != 0).then(|| active_accent_tint(ui));
                    let step = cycle_toggle_btn(ui, w, "AI", lbl, tip, true, tint);
                    if step != 0 {
                        let count = if self.dlss_rr_available { 4 } else { 3 };
                        let next = (cur + step).rem_euclid(count);
                        s.denoiser_on = next == 1;
                        s.dlss_on = next == 2;
                        s.dlss_rr_on = next == 3;
                    }
                }
            });
            ui.add_space(1.0);
            prop_slider_u32(ui, "SPP / Pass", &mut s.spp, 1..=64, true);
            prop_slider_u32(ui, "Max Accum", &mut s.max_accum, 1..=1024, true);
            prop_toggle(ui, "Shadows", &mut s.shadows_on, "Direct Sun & Soft Shadows");
            prop_toggle(
                ui,
                "OptiX SER",
                &mut s.ser_on,
                if self.ser_supported {
                    "HW Shader Execution Reordering"
                } else {
                    "Shader Execution Reordering (no HW)"
                },
            );

            if s.denoiser_on {
                prop_toggle(ui, "Temporal AI", &mut s.temporal_denoise, "Temporal Accumulation");
            }

            if s.dlss_on || s.dlss_rr_on {
                prop_row(ui, "DLSS Qual", |ui| {
                    let gap = 4.0;
                    let inner_w = (ui.available_width() - ANIM_BTN_W - gap).max(60.0);
                    ui.spacing_mut().item_spacing.x = gap;
                    const QUALS: [(u32, &str); 5] = [
                        (0, "Ultra Perf"),
                        (1, "Performance"),
                        (2, "Balanced"),
                        (3, "Quality"),
                        (5, "DLAA"),
                    ];
                    let cur = QUALS
                        .iter()
                        .position(|&(q, _)| q == s.dlss_quality)
                        .unwrap_or(3);
                    let step = cycle_toggle_btn(
                        ui,
                        inner_w,
                        "Quality",
                        QUALS[cur].1,
                        "Balance between frame rate and sharpness: Ultra Perf is fastest, Quality and DLAA give the sharpest image.",
                        true,
                        None,
                    );
                    if step != 0 {
                        let next = (cur as i32 + step).rem_euclid(QUALS.len() as i32) as usize;
                        s.dlss_quality = QUALS[next].0;
                    }
                    anim_toggle_btn_disabled(ui);
                });
                if s.dlss_on {
                    anim_slider_f32(
                        ui,
                        anim,
                        "dlss_sharpness",
                        "DLSS Sharp",
                        &mut s.dlss_sharpness,
                        0.0..=1.0,
                        2,
                        false,
                    );
                }
            }
            ui.add_space(1.0);
            let is_full_win = s.capture_source == CaptureSource::FullWindow;
            equal_grid(ui, 2, 4, |ui, idx, w| match idx {
                0 => {
                    if ui
                        .add_sized(
                            [w, ROW_H + 2.0],
                            egui::Button::new(format!("{}  Reset Accum", ic::ARROWS_CLOCKWISE)),
                        )
                        .on_hover_text("Clear accumulated samples and restart rendering cleanly from frame 1.")
                        .clicked()
                    {
                        self.bus.emit_ui(UiCommand::ResetAccum);
                    }
                }
                1 => {
                    let tip = if is_full_win {
                        "Save a lossless PNG screenshot of the full window (viewport + UI) into renders/."
                    } else {
                        "Save a lossless PNG screenshot of the 3D viewport into renders/."
                    };
                    if ui
                        .add_sized(
                            [w, ROW_H + 2.0],
                            egui::Button::new(format!("{}  Save PNG", ic::FLOPPY_DISK)),
                        )
                        .on_hover_text(tip)
                        .clicked()
                    {
                        self.bus.emit_ui(UiCommand::SaveStill(StillFormat::Png));
                    }
                }
                2 => {
                    let tip = if is_full_win {
                        "Encode a GPU-accelerated JPEG snapshot (nvJPEG) of the full window (viewport + UI) into renders/."
                    } else {
                        "Encode a fast GPU-accelerated JPEG snapshot (nvJPEG) of the 3D viewport into renders/."
                    };
                    if ui
                        .add_sized(
                            [w, ROW_H + 2.0],
                            egui::Button::new(format!("{}  Save JPEG", ic::IMAGE)),
                        )
                        .on_hover_text(tip)
                        .clicked()
                    {
                        self.bus.emit_ui(UiCommand::SaveStill(StillFormat::Jpeg));
                    }
                }
                _ => {
                    let hdr_active = s.hdr_display && self.hdr_surface_supported;
                    let use_pq10 = match s.record_video_mode {
                        1 => false,
                        2 => true,
                        _ => hdr_active,
                    };
                    let rec_lbl = if self.is_recording {
                        format!("{}  Stop ({}f)", ic::STOP, self.record_frames)
                    } else if use_pq10 {
                        format!("{}  Record PQ10", ic::VIDEO_CAMERA)
                    } else {
                        format!("{}  Record MP4", ic::VIDEO_CAMERA)
                    };
                    let target_desc = if is_full_win {
                        "full window (viewport + UI)"
                    } else {
                        "3D viewport"
                    };
                    let rec_tip = if self.is_recording {
                        "Stop recording and finalize the MP4 video file (Red = Recording).".to_string()
                    } else if use_pq10 {
                        format!(
                            "Start recording the {target_desc} as a 10-bit HDR10 PQ10 HEVC MP4 video (Green = Ready)."
                        )
                    } else {
                        format!(
                            "Start recording the {target_desc} as an iPhone-compatible H.264 MP4 video into renders/ (Green = Ready)."
                        )
                    };
                    let dark = ui.visuals().dark_mode;
                    let rec_btn = if self.is_recording {
                        let fill = if dark {
                            Color32::from_rgb(176, 42, 50)
                        } else {
                            Color32::from_rgb(202, 46, 54)
                        };
                        egui::Button::new(RichText::new(rec_lbl).color(Color32::WHITE))
                            .fill(fill)
                            .stroke(egui::Stroke::new(1.0, Color32::from_rgb(235, 96, 104)))
                    } else {
                        let fill = if dark {
                            Color32::from_rgb(32, 86, 56)
                        } else {
                            Color32::from_rgb(204, 238, 216)
                        };
                        let txt = if dark {
                            Color32::from_rgb(220, 248, 228)
                        } else {
                            Color32::from_rgb(22, 86, 46)
                        };
                        egui::Button::new(RichText::new(rec_lbl).color(txt))
                            .fill(fill)
                            .stroke(egui::Stroke::new(1.0, Color32::from_rgb(66, 152, 98)))
                    };
                    if ui
                        .add_sized([w, ROW_H + 2.0], rec_btn)
                        .on_hover_text(rec_tip)
                        .clicked()
                    {
                        self.bus.emit_ui(UiCommand::ToggleRecord);
                    }
                }
            });
            ui.add_space(2.0);
            equal_grid(ui, 2, 2, |ui, idx, w| match idx {
                0 => {
                    let frozen = s.csg_freeze_pose;
                    let lbl = if frozen {
                        format!("{}  Unfreeze Pose", ic::PLAY)
                    } else {
                        format!("{}  Freeze Pose", ic::PAUSE)
                    };
                    let btn = if frozen {
                        egui::Button::new(RichText::new(lbl).color(Color32::WHITE))
                            .fill(active_accent_tint(ui))
                    } else {
                        egui::Button::new(lbl)
                    };
                    if ui
                        .add_sized([w, ROW_H + 2.0], btn)
                        .on_hover_text(
                            "Pause dynamic traffic and crowd stepping so multi-spp path tracing accumulates cleanly for SDG captures.",
                        )
                        .clicked()
                    {
                        self.bus.emit_ui(UiCommand::ToggleFreezePose);
                    }
                }
                _ => {
                    let enabled = s.csg_mode > 0;
                    if ui
                        .add_enabled(
                            enabled,
                            egui::Button::new(format!("{}  Save SDG JSON", ic::STACK))
                                .min_size(egui::vec2(w, ROW_H + 2.0)),
                        )
                        .on_hover_text(
                            "Export exact 2D/3D actor keypoints, tight bone-OBB bounding boxes, and ray-occluded visibility labels into renders/ (also exported automatically alongside PNG/JPEG when Freeze Pose is active).",
                        )
                        .clicked()
                    {
                        self.bus.emit_ui(UiCommand::ExportSdgJson);
                    }
                }
            });
            ui.add_space(2.0);
            equal_grid(ui, 2, 2, |ui, idx, w| match idx {
                0 => {
                    const BURST_PRESETS: [u32; 3] = [16, 30, 60];
                    let cur = BURST_PRESETS
                        .iter()
                        .position(|&n| n == s.sdg_burst_frames)
                        .unwrap_or(0);
                    let lbl = format!("{}f", BURST_PRESETS[cur]);
                    let step = cycle_toggle_btn(
                        ui,
                        w,
                        "Burst",
                        &lbl,
                        "Number of frames captured in one SDG dataset burst (16, 30, or 60 frames).",
                        s.csg_mode > 0,
                        None,
                    );
                    if step != 0 {
                        let next =
                            (cur as i32 + step).rem_euclid(BURST_PRESETS.len() as i32) as usize;
                        s.sdg_burst_frames = BURST_PRESETS[next];
                    }
                }
                _ => {
                    let enabled = s.csg_mode > 0;
                    let lbl = format!(
                        "{}  SDG Burst ({}f)",
                        ic::FILM_STRIP,
                        s.sdg_burst_frames.clamp(1, 240)
                    );
                    if ui
                        .add_enabled(
                            enabled,
                            egui::Button::new(lbl)
                                .truncate()
                                .min_size(egui::vec2(w, ROW_H + 2.0)),
                        )
                        .on_hover_text(
                            "Capture a multi-frame SDG dataset burst (9-channel linear OpenEXR + .sdg.json keypoints/BBoxes + manifest.json) into renders/.",
                        )
                        .clicked()
                    {
                        self.bus.emit_ui(UiCommand::StartSdgBurst {
                            frames: s.sdg_burst_frames,
                        });
                    }
                }
            });
            ui.add_space(2.0);
            equal_grid(ui, 2, 4, |ui, idx, w| match idx {
                0 => {
                    let val = match s.capture_source {
                        CaptureSource::Viewport => "Viewport",
                        CaptureSource::FullWindow => "Full Window",
                    };
                    let tint = (s.capture_source == CaptureSource::FullWindow)
                        .then(|| active_accent_tint(ui));
                    if cycle_toggle_btn(
                        ui,
                        w,
                        "Capture",
                        val,
                        "Choose capture source for MP4 video and PNG/JPEG stills: 3D Viewport only or Full Window (viewport + docked egui UI).",
                        !self.is_recording,
                        tint,
                    ) != 0
                    {
                        s.capture_source = match s.capture_source {
                            CaptureSource::Viewport => CaptureSource::FullWindow,
                            CaptureSource::FullWindow => CaptureSource::Viewport,
                        };
                    }
                }
                1 => {
                    let hdr_active = s.hdr_display && self.hdr_surface_supported;
                    let val = if hdr_active { "HDR" } else { "SDR" };
                    let tip = if self.hdr_surface_supported {
                        "Switch the live window between standard SDR and true bright HDR (matches PQ10 video)."
                    } else {
                        "HDR display is currently off — enable HDR in Windows Display Settings to unlock."
                    };
                    let tint = hdr_active.then(|| hdr_amber_tint(ui));
                    if cycle_toggle_btn(ui, w, "View", val, tip, self.hdr_surface_supported, tint)
                        != 0
                    {
                        s.hdr_display = !s.hdr_display;
                    }
                }
                2 => {
                    let val = match s.record_video_mode {
                        1 => "SDR",
                        2 => "PQ10",
                        _ => "Auto",
                    };
                    let tint = match s.record_video_mode {
                        2 => Some(hdr_amber_tint(ui)),
                        1 => Some(active_accent_tint(ui)),
                        _ => None,
                    };
                    let step = cycle_toggle_btn(
                        ui,
                        w,
                        "Video",
                        val,
                        "Video format: Auto follows the current View, SDR saves standard H.264 MP4, PQ10 saves 10-bit HDR10 HEVC MP4 for iPhone Pro Max.",
                        !self.is_recording,
                        tint,
                    );
                    if step != 0 {
                        s.record_video_mode =
                            ((s.record_video_mode as i32 + step).rem_euclid(3)) as u32;
                    }
                }
                _ => {
                    let val = if s.record_fixed_fps {
                        "Smooth 60"
                    } else {
                        "Live RT"
                    };
                    let tint = s.record_fixed_fps.then(|| active_accent_tint(ui));
                    if cycle_toggle_btn(
                        ui,
                        w,
                        "Timing",
                        val,
                        "Live RT keeps real-time speed even when FPS drops; Smooth 60 renders animations at a locked cinematic 60 FPS.",
                        !self.is_recording,
                        tint,
                    ) != 0
                    {
                        s.record_fixed_fps = !s.record_fixed_fps;
                    }
                }
            });
            ui.add_space(2.0);
            {
                const SHUTTER_MODES: [(u32, &str); 3] = [(1, "Off (1x)"), (2, "180° 2x"), (4, "180° 4x")];
                let cur = SHUTTER_MODES
                    .iter()
                    .position(|&(n, _)| n == s.record_shutter_subframes)
                    .unwrap_or(0);
                let enabled = s.record_fixed_fps && !self.is_recording;
                let tint = (s.record_fixed_fps && s.record_shutter_subframes > 1)
                    .then(|| active_accent_tint(ui));
                let tip = if s.record_fixed_fps {
                    "180-degree shutter motion blur during Smooth 60 recording: averages 2 or 4 subframes across half a frame interval in linear HDR."
                } else {
                    "Enable Smooth 60 Timing to unlock 180-degree subframe shutter motion blur."
                };
                let step = cycle_toggle_btn(
                    ui,
                    ui.available_width(),
                    "Shutter",
                    SHUTTER_MODES[cur].1,
                    tip,
                    enabled,
                    tint,
                );
                if step != 0 {
                    let next = (cur as i32 + step).rem_euclid(SHUTTER_MODES.len() as i32) as usize;
                    s.record_shutter_subframes = SHUTTER_MODES[next].0;
                }
            }
            ui.add_space(2.0);
            if ui
                .add_sized(
                    [ui.available_width(), ROW_H + 2.0],
                    egui::Button::new(format!("{}  Hot-Reload Shader (NVRTC)", ic::LIGHTNING)),
                )
                .on_hover_text("Recompile raytracer.cu from disk on the fly without restarting the app.")
                .clicked()
            {
                self.bus.emit_ui(UiCommand::ReloadShader);
            }
            if let Some(msg) = self.last_saved_msg {
                ui.label(
                    RichText::new(msg)
                        .size(11.0)
                        .color(Color32::from_rgb(166, 209, 137)),
                );
            }
        });
    }

    fn ui_camera_stats(&mut self, ui: &mut Ui) {
        let mut snap_pose: Option<CameraPose> = None;
        let mut request_blend = false;
        let mut request_gen_rail = false;
        {
            let s = &mut *self.settings;
            let rail = &mut *self.camera_rail;
            let cur_pose = *self.cam_pose;
            let has_scenes = !self.scene_descriptors.is_empty();
            collapsing_section(
                ui,
                "sec_cinema",
                ic::VIDEO_CAMERA,
                "Cinema & Camera Rail",
                true,
                |ui| {
                    // 1. Toggleable Auto-Drive ON/OFF button (never erases custom keyframes).
                    {
                        let active = rail.is_active();
                        let can_play = rail.can_play() || (s.csg_mode == 0 && has_scenes);
                        let dark = ui.visuals().dark_mode;
                        let lbl = if active {
                            format!("{}  Auto-Drive: ON (Stop)", ic::PAUSE)
                        } else {
                            format!("{}  Auto-Drive: OFF (Start)", ic::PLAY)
                        };
                        let mut btn = egui::Button::new(if active {
                            RichText::new(lbl).color(if dark {
                                Color32::from_rgb(220, 248, 228)
                            } else {
                                Color32::from_rgb(22, 86, 46)
                            })
                        } else {
                            RichText::new(lbl)
                        });
                        if active {
                            btn = btn
                                .fill(if dark {
                                    Color32::from_rgb(32, 86, 56)
                                } else {
                                    Color32::from_rgb(204, 238, 216)
                                })
                                .stroke(egui::Stroke::new(1.0, Color32::from_rgb(66, 152, 98)));
                        }
                        let tip = if active {
                            "Turn Auto-Drive OFF to immediately regain manual camera control without wiping your custom keyframes."
                        } else if rail.can_play() {
                            "Turn Auto-Drive ON to fly the selected cassette smoothly along the active CSG scene."
                        } else if s.csg_mode == 0 && has_scenes {
                            "Activate the default CSG world and start Auto-Drive."
                        } else {
                            "Load a CSG scene or add at least 2 Custom keyframes (+ KF) to enable Auto-Drive."
                        };
                        if ui
                            .add_enabled(
                                active || can_play,
                                btn.min_size(egui::vec2(ui.available_width(), ROW_H + 4.0)),
                            )
                            .on_hover_text(tip)
                            .clicked()
                        {
                            if active {
                                rail.set_playing(false);
                            } else if rail.can_play() {
                                if rail.set_playing(true) {
                                    request_blend = true;
                                }
                            } else if s.csg_mode == 0 && has_scenes {
                                s.csg_mode = 1;
                                if let Some(d) = self.scene_descriptors.first() {
                                    let tod_u32 = match d.default_tod {
                                        TimeOfDay::Day => 0,
                                        TimeOfDay::Sunset => 1,
                                        TimeOfDay::Night => 2,
                                    };
                                    s.csg_wetness = d.default_wetness.clamp(0.0, 1.0);
                                    apply_csg_time_of_day_preset(s, tod_u32);
                                }
                                rail.playing = true;
                                request_blend = true;
                            }
                        }
                    }
                    ui.add_space(2.0);
                    // 2. Dynamic Scene Cassette Deck + Independent Custom Deck.
                    prop_row(ui, "Cassette", |ui| {
                        let sel_text = rail.active_label();
                        let active_scene_idx = rail.active_scene_rail_index();
                        let mut picked_scene: Option<usize> = None;
                        let mut picked_custom = false;
                        egui::ComboBox::from_id_salt("cinema_cassette_combo")
                            .width(ui.available_width().max(32.0))
                            .truncate()
                            .selected_text(sel_text)
                            .show_ui(ui, |ui| {
                                for (i, cr) in rail.scene_rails().iter().enumerate() {
                                    let selected = active_scene_idx == Some(i);
                                    if ui
                                        .selectable_label(selected, &cr.spec.label)
                                        .on_hover_text(&cr.spec.description)
                                        .clicked()
                                    {
                                        picked_scene = Some(i);
                                    }
                                }
                                let custom_lbl = format!("Custom ({} KF)", rail.keyframes.len());
                                if ui
                                    .selectable_label(rail.is_custom_selected(), custom_lbl)
                                    .on_hover_text("Play your own Custom keyframes (+ KF / Pop KF / Clear) independently of scene cassettes.")
                                    .clicked()
                                {
                                    picked_custom = true;
                                }
                            });
                        if let Some(idx) = picked_scene {
                            let changed = active_scene_idx != Some(idx);
                            rail.select_scene_rail(idx);
                            if changed && rail.is_active() {
                                request_blend = true;
                            }
                        } else if picked_custom {
                            let changed = !rail.is_custom_selected();
                            rail.select_custom();
                            if changed && rail.is_active() {
                                request_blend = true;
                            }
                        }
                    });
                    // 3. On-demand exact-CSG Random Rail, Possess Body & GPU Smart View controls.
                    {
                        let gen_enabled = s.csg_mode != 0;
                        equal_grid(ui, 2, 2, |ui, idx, w| match idx {
                            0 => {
                                let gen_lbl = format!(
                                    "{}  Random Rail",
                                    ic::SHUFFLE,
                                );
                                if ui
                                    .add_enabled(
                                        gen_enabled,
                                        egui::Button::new(gen_lbl)
                                            .min_size(egui::vec2(w, ROW_H + 2.0)),
                                    )
                                    .on_hover_text(
                                        "Synthesize a fresh, exact-CSG collision-verified random cinema rail for the active city scene.",
                                    )
                                    .clicked()
                                {
                                    *self.rail_random_seed = self
                                        .rail_random_seed
                                        .wrapping_add(1)
                                        .max(1);
                                    request_gen_rail = true;
                                }
                            }
                            _ => {
                                if ui
                                    .add_enabled(
                                        gen_enabled,
                                        egui::Button::new(format!("{}  Smart Rail", ic::SPARKLE))
                                            .min_size(egui::vec2(w, ROW_H + 2.0)),
                                    )
                                    .on_hover_text(
                                        "Generate 4 candidate cinema rails, score their vistas on the selected GPU/CPU backend, and fly the winner.",
                                    )
                                    .clicked()
                                {
                                    *self.rail_random_seed = self
                                        .rail_random_seed
                                        .wrapping_add(1)
                                        .max(1);
                                    self.bus.emit_ui(UiCommand::RunSmartRail {
                                        seed: *self.rail_random_seed,
                                        backend: s.probe_backend,
                                    });
                                }
                            }
                        });
                        ui.add_space(2.0);
                        equal_grid(ui, 2, 2, |ui, idx, w| match idx {
                            0 => {
                                if ui
                                    .add_enabled(
                                        gen_enabled,
                                        egui::Button::new(format!("{}  Possess Car", ic::CAR))
                                            .min_size(egui::vec2(w, ROW_H + 2.0)),
                                    )
                                    .on_hover_text(
                                        "Jump directly into a driving Robotaxi / vehicle or mounted landmark (cycles targets on each click; Ctrl+Click viewport to pick any object).",
                                    )
                                    .clicked()
                                {
                                    self.bus.emit_ui(UiCommand::PossessNextVehicle);
                                }
                            }
                            _ => {
                                if ui
                                    .add_enabled(
                                        gen_enabled,
                                        egui::Button::new(format!("{}  Smart View", ic::EYE))
                                            .min_size(egui::vec2(w, ROW_H + 2.0)),
                                    )
                                    .on_hover_text(
                                        "Score 64 city viewpoints on OptiX 9 / WGPU / CPU and smoothly glide to the best composition.",
                                    )
                                    .clicked()
                                {
                                    *self.smart_view_seed = self
                                        .smart_view_seed
                                        .wrapping_add(1)
                                        .max(1);
                                    self.bus.emit_ui(UiCommand::RunSmartView {
                                        seed: *self.smart_view_seed,
                                        backend: s.probe_backend,
                                    });
                                }
                            }
                        });
                        prop_row(ui, "Probe", |ui| {
                            egui::ComboBox::from_id_salt("cinema_probe_backend")
                                .width(ui.available_width().max(32.0))
                                .truncate()
                                .selected_text(s.probe_backend.label())
                                .show_ui(ui, |ui| {
                                    for kind in [
                                        ProbeBackendKind::Auto,
                                        ProbeBackendKind::Optix,
                                        ProbeBackendKind::Wgpu,
                                        ProbeBackendKind::Cpu,
                                    ] {
                                        ui.selectable_value(
                                            &mut s.probe_backend,
                                            kind,
                                            kind.label(),
                                        );
                                    }
                                });
                        });
                        if let Some(sv) = self.last_smart_view {
                            ui.label(
                                RichText::new(format!(
                                    "{}: {:.2} ({}v, ent {:.2}, depth {:.2}, {:.1} ms)",
                                    sv.score.backend.label(),
                                    sv.score.composite,
                                    sv.candidates_evaluated,
                                    sv.score.semantic_entropy,
                                    sv.score.depth_stratification,
                                    sv.elapsed_ms
                                ))
                                .size(10.5)
                                .weak(),
                            );
                        }
                    }
                    prop_slider_f32(
                        ui,
                        "Cam Smooth",
                        &mut s.cam_smooth_tau,
                        0.0..=1.0,
                        2,
                        "s",
                    );
                    prop_slider_f32(
                        ui,
                        "Time Scale",
                        &mut s.time_scale,
                        0.05..=2.0,
                        2,
                        "x",
                    );
                    ui.add_space(2.0);
                    equal_grid(ui, 2, 2, |ui, idx, w| match idx {
                        0 => {
                            let tint = rail.looping.then(|| active_accent_tint(ui));
                            if cycle_toggle_btn(
                                ui,
                                w,
                                "Loop",
                                if rail.looping { "On" } else { "Off" },
                                "Continuously wrap the camera rail with C1 periodic continuity.",
                                true,
                                tint,
                            ) != 0
                            {
                                rail.looping = !rail.looping;
                                rail.sync_lut();
                            }
                        }
                        _ => {
                            let tint = rail.ease_io.then(|| active_accent_tint(ui));
                            if cycle_toggle_btn(
                                ui,
                                w,
                                "Ease",
                                if rail.ease_io { "I/O" } else { "Lin" },
                                "Smoothstep ease-in / ease-out vs constant arc-length speed along the rail.",
                                true,
                                tint,
                            ) != 0
                            {
                                rail.ease_io = !rail.ease_io;
                            }
                        }
                    });
                    prop_slider_f32(
                        ui,
                        "Duration",
                        &mut rail.duration_s,
                        2.0..=120.0,
                        1,
                        "s",
                    );
                    if rail.can_play() {
                        let dur = rail.duration_s.max(0.5);
                        let prev_t = rail.time_s;
                        prop_slider_f32(ui, "Rail Time", &mut rail.time_s, 0.0..=dur, 2, "s");
                        if (rail.time_s - prev_t).abs() > 1e-5
                            && let Some(p) =
                                rail.evaluate_current(cur_pose.znear, cur_pose.zfar.max(2000.0))
                        {
                            snap_pose = Some(p);
                        }
                    }
                    ui.add_space(2.0);
                    // 4. Independent Custom Keyframe controls (preserved across scene & cassette switches).
                    equal_grid(ui, 3, 3, |ui, idx, w| match idx {
                        0 => {
                            let lbl = format!("+ KF ({})", rail.keyframes.len());
                            if ui
                                .add_sized([w, ROW_H + 2.0], egui::Button::new(lbl))
                                .on_hover_text("Append the current camera pose to your Custom keyframe deck.")
                                .clicked()
                            {
                                rail.push_pose(cur_pose);
                            }
                        }
                        1 => {
                            if ui
                                .add_enabled(
                                    !rail.keyframes.is_empty(),
                                    egui::Button::new("Pop KF").min_size(egui::vec2(w, ROW_H + 2.0)),
                                )
                                .on_hover_text("Remove the last Custom keyframe.")
                                .clicked()
                            {
                                rail.pop_keyframe();
                            }
                        }
                        _ => {
                            if ui
                                .add_enabled(
                                    !rail.keyframes.is_empty(),
                                    egui::Button::new("Clear").min_size(egui::vec2(w, ROW_H + 2.0)),
                                )
                                .on_hover_text("Clear all Custom keyframes without affecting scene cassettes.")
                                .clicked()
                            {
                                rail.clear();
                            }
                        }
                    });
                },
            );
        }
        if request_blend {
            self.bus.emit_ui(UiCommand::BlendRail);
        }
        if request_gen_rail {
            self.bus.emit_ui(UiCommand::GenerateRandomRail {
                seed: *self.rail_random_seed,
            });
        }
        if let Some(pose) = snap_pose {
            self.cam_driver.controller_mut().set_pose(pose);
            *self.cam_pose = pose;
            self.bus.emit_ui(UiCommand::SnapCamera);
        }

        collapsing_section(ui, "sec_telemetry", ic::GAUGE, "Telemetry", true, |ui| {
            stat_row(ui, "GPU", &self.telemetry.gpu_name);
            stat_row(
                ui,
                "Resolution",
                &format!("{} x {}", self.render_w, self.render_h),
            );
            stat_row(ui, "UI FPS", &format!("{:.0} FPS", self.ui_fps));
            stat_row(
                ui,
                "RT FPS",
                &format!(
                    "{:.1} FPS ({:.1} ms{})",
                    self.rt_fps,
                    self.rt_ms,
                    if self.fast_preview { " fast" } else { "" }
                ),
            );
            stat_row(
                ui,
                "Bloom / TM",
                &format!("{:.2} / {:.2} ms", self.bloom_ms, self.tonemap_ms),
            );
            stat_row(
                ui,
                "Accum",
                &format!(
                    "{} / {} ({} spp)",
                    self.accum_count, self.settings.max_accum, self.total_samples
                ),
            );
            stat_row(ui, "Denoiser", &format!("{:.1} ms", self.denoise_ms));
            stat_row(
                ui,
                "OptiX SER",
                match (self.ser_supported, self.settings.ser_on) {
                    (true, true) => "HW Active",
                    (true, false) => "HW Off",
                    (false, _) => "Unsupported",
                },
            );
            stat_row(
                ui,
                "VRAM",
                &format!(
                    "{} / {} MiB",
                    self.telemetry.vram_used_mib, self.telemetry.vram_total_mib
                ),
            );
            stat_row(
                ui,
                "Clocks",
                &format!(
                    "{} / {} MHz",
                    self.telemetry.gpu_clock_mhz, self.telemetry.mem_clock_mhz
                ),
            );
            stat_row(
                ui,
                "Temp / Load",
                &format!(
                    "{} C / {} %",
                    self.telemetry.temp_c, self.telemetry.util_gpu
                ),
            );
            stat_row(ui, "Power", &format!("{:.0} W", self.telemetry.power_w));
            stat_row(ui, "CUPTI", &self.telemetry.cupti_status);
            stat_row(ui, "NGX", &self.telemetry.ngx_status);
            stat_row(
                ui,
                "DLSS RR",
                match (self.dlss_rr_available, self.settings.dlss_rr_on) {
                    (true, true) => "Active",
                    (true, false) => "Available",
                    (false, _) => "Unavailable",
                },
            );
            if self.settings.csg_mode != 0 {
                stat_row(
                    ui,
                    "CSG Inst",
                    &format!(
                        "{} / {}",
                        self.csg_stats.visible_instances, self.csg_stats.total_instances
                    ),
                );
                stat_row(
                    ui,
                    "CSG DAG",
                    &format!(
                        "{} GAS / {} prm",
                        self.csg_stats.compounds, self.csg_stats.primitives
                    ),
                );
                stat_row(
                    ui,
                    "CSG Aux",
                    &format!(
                        "{:.1} KiB",
                        self.csg_stats.aux_planes_bytes as f64 / 1024.0
                    ),
                );
                stat_row(
                    ui,
                    "CSG VRAM",
                    &format!("{:.1} KiB", self.csg_stats.gpu_bytes as f64 / 1024.0),
                );
                if self.crowd_actor_count > 0 {
                    stat_row(
                        ui,
                        "Crowd Sim",
                        &format!(
                            "{} {:.2} ms ({})",
                            self.crowd_active_backend.label(),
                            self.crowd_step_ms,
                            self.crowd_actor_count
                        ),
                    );
                }
            }
        });

        collapsing_section(ui, "sec_perf", ic::CHART_LINE, "Performance", true, |ui| {
            stat_row(ui, "RT Cores", &format!("{:.2} ms (OptiX)", self.rt_ms));
            stat_row(
                ui,
                "Tensor Cores",
                &format!("{:.2} ms (Den/DLSS)", self.tensor_cores_ms),
            );
            stat_row(
                ui,
                "CUDA Compute",
                &format!("{:.2} ms (IAS/FFT/TM)", self.cuda_compute_ms),
            );
            ui.add_space(2.0);
            sparkline_row(
                ui,
                "RT Cores",
                &self.timings.rt_cores_ms,
                Color32::from_rgb(138, 173, 244),
                "ms",
            );
            sparkline_row(
                ui,
                "Tensor",
                &self.timings.tensor_cores_ms,
                Color32::from_rgb(166, 209, 137),
                "ms",
            );
            sparkline_row(
                ui,
                "CUDA SM",
                &self.timings.cuda_compute_ms,
                Color32::from_rgb(238, 212, 159),
                "ms",
            );
            sparkline_row(
                ui,
                "FPS",
                &self.timings.fps,
                Color32::from_rgb(198, 160, 246),
                "",
            );
        });
    }
}

pub fn apply_csg_time_of_day_preset(s: &mut DemoSettings, tod: u32) {
    s.csg_time_of_day = tod.min(2);
    match s.csg_time_of_day {
        0 => {
            s.sky_mode = 0; // Daylight
            s.sun_azimuth = -52.0;
            s.sun_elevation = 46.0;
            s.sun_intensity = 2.20;
            s.sky_intensity = 0.95;
            s.exposure = 0.90;
        }
        2 => {
            s.sky_mode = 8; // Coastal Night
            s.sun_azimuth = -110.0;
            s.sun_elevation = 24.0;
            s.sun_intensity = 0.16;
            s.sky_intensity = 0.65;
            s.exposure = 1.05;
        }
        _ => {
            s.sky_mode = 1; // Golden Sunset
            s.sun_azimuth = -98.0;
            s.sun_elevation = 11.0;
            s.sun_intensity = 1.75;
            s.sky_intensity = 0.85;
            s.exposure = 0.95;
        }
    }
}

fn section_accent(id_salt: &str) -> Color32 {
    match id_salt {
        "sec_csg" => Color32::from_rgb(92, 184, 224),
        "sec_sky" => Color32::from_rgb(232, 178, 84),
        "sec_env" => Color32::from_rgb(98, 162, 208),
        "sec_dof" => Color32::from_rgb(198, 118, 162),
        "sec_floor" => Color32::from_rgb(154, 134, 212),
        "sec_scene" => Color32::from_rgb(122, 184, 156),
        "sec_bloom" => Color32::from_rgb(224, 132, 88),
        "sec_split" => Color32::from_rgb(186, 156, 96),
        "sec_pt" => Color32::from_rgb(196, 96, 96),
        "sec_presets" => Color32::from_rgb(216, 156, 78),
        "sec_formula" => Color32::from_rgb(92, 188, 168),
        "sec_transform" => Color32::from_rgb(138, 164, 212),
        "sec_detail" => Color32::from_rgb(166, 209, 137),
        "sec_cinema" => Color32::from_rgb(232, 148, 112),
        "sec_telemetry" => Color32::from_rgb(112, 172, 216),
        "sec_perf" => Color32::from_rgb(112, 184, 156),
        _ => Color32::from_rgb(142, 149, 163),
    }
}

const SECTION_GROUPS: &[&[&str]] = &[
    // Fractal panel
    &["sec_presets", "sec_formula", "sec_transform", "sec_detail"],
    // Render & Floor panel
    &[
        "sec_csg",
        "sec_sky",
        "sec_dof",
        "sec_floor",
        "sec_scene",
        "sec_bloom",
        "sec_split",
        "sec_pt",
    ],
    // Camera & Stats panel
    &["sec_cinema", "sec_telemetry", "sec_perf"],
];

fn section_group_for(id_salt: &str) -> &'static [&'static str] {
    SECTION_GROUPS
        .iter()
        .copied()
        .find(|group| group.contains(&id_salt))
        .unwrap_or(&[])
}

fn section_default_open(id_salt: &str) -> bool {
    matches!(
        id_salt,
        "sec_cinema"
            | "sec_telemetry"
            | "sec_perf"
            | "sec_presets"
            | "sec_formula"
            | "sec_csg"
            | "sec_sky"
            | "sec_pt"
    )
}

fn collapsing_section(
    ui: &mut Ui,
    id_salt: &str,
    icon: &str,
    title: &str,
    _default_open: bool,
    add_body: impl FnOnce(&mut Ui),
) {
    let accent = section_accent(id_salt);
    let ctx = ui.ctx().clone();
    let id = egui::Id::new(id_salt);
    let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
        &ctx,
        id,
        section_default_open(id_salt),
    );
    let header_h = 22.0;
    let total_w = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(total_w, header_h), Sense::hover());

    // Icon button at the very start of the title bar: collapses/expands all sections in this panel.
    let icon_rect = egui::Rect::from_min_size(
        egui::pos2(rect.left() + 4.0, rect.top() + 1.0),
        egui::vec2(20.0, header_h - 2.0),
    );
    let title_rect = egui::Rect::from_min_max(
        egui::pos2(icon_rect.right(), rect.top()),
        rect.max,
    );

    let icon_resp = ui
        .interact(icon_rect, egui::Id::new((id_salt, "toggle_all")), Sense::click())
        .on_hover_text("Collapse / Expand all sections in this panel")
        .on_hover_cursor(egui::CursorIcon::PointingHand);
    let title_resp = ui
        .interact(title_rect, egui::Id::new((id_salt, "toggle_one")), Sense::click())
        .on_hover_cursor(egui::CursorIcon::PointingHand);

    if icon_resp.clicked() {
        let group = section_group_for(id_salt);
        let any_open = group.iter().any(|&sec| {
            egui::collapsing_header::CollapsingState::load_with_default_open(
                &ctx,
                egui::Id::new(sec),
                section_default_open(sec),
            )
            .is_open()
        });
        let target_open = !any_open;
        for &sec in group {
            let sec_id = egui::Id::new(sec);
            let mut st = egui::collapsing_header::CollapsingState::load_with_default_open(
                &ctx,
                sec_id,
                section_default_open(sec),
            );
            st.set_open(target_open);
            st.store(&ctx);
        }
        state.set_open(target_open);
    } else if title_resp.clicked() {
        state.toggle(ui);
    }

    let base_bg = ui.visuals().faint_bg_color;
    let tint_amt = if title_resp.hovered() || icon_resp.hovered() {
        0.22
    } else {
        0.14
    };
    let fill = base_bg.lerp_to_gamma(accent, tint_amt);
    ui.painter().rect_filled(rect, 3.0, fill);
    let bar_rect = egui::Rect::from_min_size(rect.min, egui::vec2(3.0, rect.height()));
    ui.painter().rect_filled(bar_rect, 1.5, accent);

    if icon_resp.hovered() {
        ui.painter()
            .rect_filled(icon_rect, 3.0, accent.gamma_multiply(0.32));
    }

    let caret = if state.is_open() {
        ic::CARET_DOWN
    } else {
        ic::CARET_RIGHT
    };
    let text_col = ui.visuals().text_color().lerp_to_gamma(accent, 0.45);
    let icon_col = if icon_resp.hovered() {
        ui.visuals().strong_text_color()
    } else {
        text_col
    };
    ui.painter().text(
        icon_rect.center(),
        egui::Align2::CENTER_CENTER,
        icon,
        egui::FontId::proportional(12.0),
        icon_col,
    );
    ui.painter().text(
        egui::pos2(icon_rect.right() + 4.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        format!("{caret}  {title}"),
        egui::FontId::proportional(12.0),
        text_col,
    );
    state.show_body_indented(&title_resp, ui, |ui| {
        ui.add_space(2.0);
        add_body(ui);
        ui.add_space(2.0);
    });
}

fn active_accent_tint(ui: &Ui) -> Color32 {
    if ui.visuals().dark_mode {
        Color32::from_rgb(38, 70, 110)
    } else {
        Color32::from_rgb(208, 226, 250)
    }
}

fn hdr_amber_tint(ui: &Ui) -> Color32 {
    if ui.visuals().dark_mode {
        Color32::from_rgb(106, 78, 30)
    } else {
        Color32::from_rgb(252, 232, 184)
    }
}

/// Compact cyclic toggle button: LMB steps forward (`+1`), RMB steps backward (`-1`).
fn cycle_toggle_btn(
    ui: &mut Ui,
    width: f32,
    label: &str,
    value: &str,
    tooltip: &str,
    enabled: bool,
    tint: Option<Color32>,
) -> i32 {
    let mut step = 0;
    ui.add_enabled_ui(enabled, |ui| {
        let mut btn = egui::Button::new(format!("{label}: {value}"));
        if let Some(fill) = tint {
            btn = btn.fill(fill);
        }
        let resp = ui
            .add_sized([width, ROW_H + 2.0], btn)
            .on_hover_text(format!("{tooltip}\nLMB: next | RMB: prev"));
        if resp.clicked() {
            step = 1;
        } else if resp.secondary_clicked() {
            step = -1;
        }
    });
    step
}

/// Computes adaptive `(label_w, rem_w)` column widths for a property row so labels shrink
/// gracefully on narrow dock panes without clipping control columns.
fn prop_row_widths(total_w: f32) -> (f32, f32) {
    let row_gap = 6.0;
    let safe_w = if total_w.is_finite() {
        total_w.max(130.0)
    } else {
        240.0
    };
    let label_w = LABEL_W.min((safe_w * 0.34).max(46.0));
    let rem_w = (safe_w - label_w - row_gap).max(76.0);
    (label_w, rem_w)
}

/// Computes adaptive `(track_w, val_w)` widths inside a property row's control column so
/// numeric `DragValue` boxes and the animation toggle stay visible with zero horizontal overflow.
fn slider_control_widths(rem_w: f32) -> (f32, f32) {
    let ctrl_gap = 4.0;
    let avail = (rem_w - ANIM_BTN_W - ctrl_gap * 2.0).max(52.0);
    let val_w = VAL_W.min((avail - 16.0).max(36.0));
    let track_w = (avail - val_w).max(16.0);
    (track_w, val_w)
}

fn equal_grid(
    ui: &mut Ui,
    cols: usize,
    count: usize,
    mut item_ui: impl FnMut(&mut Ui, usize, f32),
) {
    let gap = 4.0;
    let total_w = ui.available_width().max(48.0);
    let cell_w = ((total_w - gap * (cols.saturating_sub(1) as f32)) / (cols as f32)).max(24.0);
    let mut idx = 0;
    while idx < count {
        ui.horizontal(|ui| {
            ui.set_max_width(total_w);
            ui.spacing_mut().item_spacing.x = gap;
            for _ in 0..cols {
                if idx < count {
                    ui.allocate_ui_with_layout(
                        egui::vec2(cell_w, ROW_H + 2.0),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.set_max_width(cell_w);
                            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                            item_ui(ui, idx, cell_w);
                        },
                    );
                    idx += 1;
                }
            }
        });
    }
}

fn prop_row(ui: &mut Ui, label: &str, add_contents: impl FnOnce(&mut Ui)) {
    let total_w = ui.available_width().max(130.0);
    let gap = 6.0;
    let (label_w, rem_w) = prop_row_widths(total_w);
    ui.allocate_ui_with_layout(
        egui::vec2(total_w, ROW_H),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.set_max_width(total_w);
            ui.spacing_mut().item_spacing.x = gap;
            let (label_rect, label_resp) =
                ui.allocate_exact_size(egui::vec2(label_w, ROW_H), Sense::hover());
            let galley = ui.painter().layout_no_wrap(
                label.to_owned(),
                egui::FontId::proportional(11.5),
                ui.visuals().weak_text_color(),
            );
            let clipped = galley.size().x > label_w + 1.0;
            ui.painter()
                .with_clip_rect(label_rect.intersect(ui.clip_rect()))
                .galley(
                    egui::pos2(
                        label_rect.left(),
                        label_rect.center().y - galley.size().y * 0.5,
                    ),
                    galley,
                    Color32::WHITE,
                );
            if clipped && label_resp.hovered() {
                label_resp.on_hover_text(label);
            }
            ui.allocate_ui_with_layout(
                egui::vec2(rem_w, ROW_H),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.set_max_width(rem_w);
                    add_contents(ui);
                },
            );
        },
    );
}

fn anim_toggle_btn(ui: &mut Ui, anim: &mut DemoAnimator, key: &str) {
    let active = anim.is_active(key);
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(ANIM_BTN_W, ROW_H), Sense::click());
    if ui.is_rect_visible(rect) {
        if active {
            let bg = ui.visuals().selection.bg_fill;
            ui.painter().rect_filled(rect, 3.0, bg);
        } else if resp.hovered() {
            let bg = ui.visuals().widgets.hovered.weak_bg_fill;
            ui.painter().rect_filled(rect, 3.0, bg);
        }
        let col = if active {
            if ui.visuals().dark_mode {
                UiTheme::Obsidian.section_color()
            } else {
                UiTheme::Light.section_color()
            }
        } else if resp.hovered() {
            ui.visuals().text_color()
        } else {
            ui.visuals().weak_text_color()
        };
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            ic::WAVE_SINE,
            egui::FontId::proportional(11.0),
            col,
        );
    }
    if resp
        .on_hover_text("Animate +/-10% * Amp * range * sin(2*pi*t / Period)")
        .clicked()
    {
        anim.toggle(key);
    }
}

fn anim_toggle_btn_disabled(ui: &mut Ui) {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(ANIM_BTN_W, ROW_H), Sense::hover());
    if ui.is_rect_visible(rect) {
        let col = ui.visuals().weak_text_color().gamma_multiply(0.45);
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            ic::WAVE_SINE,
            egui::FontId::proportional(11.0),
            col,
        );
    }
    resp.on_hover_text("This parameter cannot be animated");
}

#[allow(clippy::too_many_arguments)]
fn anim_slider_f32(
    ui: &mut Ui,
    anim: &mut DemoAnimator,
    key: &str,
    label: &str,
    val: &mut f32,
    range: RangeInclusive<f32>,
    decimals: usize,
    log: bool,
) {
    prop_row(ui, label, |ui| {
        let gap = 4.0;
        let (track_w, val_w) = slider_control_widths(ui.available_width());
        ui.spacing_mut().item_spacing.x = gap;
        ui.spacing_mut().slider_width = track_w;
        let slider = egui::Slider::new(val, range.clone())
            .show_value(false)
            .logarithmic(log);
        ui.add_sized([track_w, ROW_H], slider);
        let prev_style = ui.style().override_text_style.clone();
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
        ui.add_sized(
            [val_w, ROW_H],
            egui::DragValue::new(val)
                .range(range)
                .fixed_decimals(decimals)
                .speed(0.005),
        );
        ui.style_mut().override_text_style = prev_style;
        anim_toggle_btn(ui, anim, key);
    });
}

fn prop_slider_f32(
    ui: &mut Ui,
    label: &str,
    val: &mut f32,
    range: RangeInclusive<f32>,
    decimals: usize,
    suffix: &str,
) {
    prop_row(ui, label, |ui| {
        let gap = 4.0;
        let (track_w, val_w) = slider_control_widths(ui.available_width());
        ui.spacing_mut().item_spacing.x = gap;
        ui.spacing_mut().slider_width = track_w;
        let slider = egui::Slider::new(val, range.clone()).show_value(false);
        ui.add_sized([track_w, ROW_H], slider);
        let prev_style = ui.style().override_text_style.clone();
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
        let mut drag = egui::DragValue::new(val)
            .range(range)
            .fixed_decimals(decimals)
            .speed(0.01);
        if !suffix.is_empty() {
            drag = drag.suffix(suffix);
        }
        ui.add_sized([val_w, ROW_H], drag);
        ui.style_mut().override_text_style = prev_style;
        anim_toggle_btn_disabled(ui);
    });
}

fn prop_slider_u32(ui: &mut Ui, label: &str, val: &mut u32, range: RangeInclusive<u32>, log: bool) {
    prop_row(ui, label, |ui| {
        let gap = 4.0;
        let (track_w, val_w) = slider_control_widths(ui.available_width());
        ui.spacing_mut().item_spacing.x = gap;
        ui.spacing_mut().slider_width = track_w;
        let slider = egui::Slider::new(val, range.clone())
            .show_value(false)
            .logarithmic(log);
        ui.add_sized([track_w, ROW_H], slider);
        let prev_style = ui.style().override_text_style.clone();
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
        ui.add_sized(
            [val_w, ROW_H],
            egui::DragValue::new(val).range(range).speed(1.0),
        );
        ui.style_mut().override_text_style = prev_style;
        anim_toggle_btn_disabled(ui);
    });
}

fn anim_vec3(
    ui: &mut Ui,
    anim: &mut DemoAnimator,
    key: &str,
    label: &str,
    v: &mut [f32; 3],
    range: RangeInclusive<f32>,
    speed: f32,
) {
    prop_row(ui, label, |ui| {
        let gap = 4.0;
        let w = ((ui.available_width() - ANIM_BTN_W - gap * 3.0) / 3.0).max(18.0);
        ui.spacing_mut().item_spacing.x = gap;
        let prev_style = ui.style().override_text_style.clone();
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
        for item in v.iter_mut() {
            ui.add_sized(
                [w, ROW_H],
                egui::DragValue::new(item)
                    .range(range.clone())
                    .speed(speed)
                    .fixed_decimals(2),
            );
        }
        ui.style_mut().override_text_style = prev_style;
        anim_toggle_btn(ui, anim, key);
    });
}

fn prop_toggle(ui: &mut Ui, label: &str, val: &mut bool, text: &str) {
    prop_row(ui, label, |ui| {
        let gap = 4.0;
        let check_w = (ui.available_width() - ANIM_BTN_W - gap).max(24.0);
        ui.spacing_mut().item_spacing.x = gap;
        ui.allocate_ui_with_layout(
            egui::vec2(check_w, ROW_H),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.set_max_width(check_w);
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                ui.checkbox(val, text);
            },
        );
        anim_toggle_btn_disabled(ui);
    });
}

fn stat_row(ui: &mut Ui, label: &str, value: &str) {
    prop_row(ui, label, |ui| {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let resp = ui.add(
                egui::Label::new(
                    RichText::new(value)
                        .size(11.5)
                        .color(ui.visuals().text_color()),
                )
                .truncate(),
            );
            if resp.hovered() {
                resp.on_hover_text(value);
            }
        });
    });
}

fn sparkline_row(
    ui: &mut Ui,
    label: &str,
    ring: &RingBuffer<HISTORY_LEN>,
    color: Color32,
    unit: &str,
) {
    prop_row(ui, label, |ui| {
        let val_w = 52.0;
        let gap = 6.0;
        let plot_w = (ui.available_width() - val_w - gap).max(40.0);
        let (rect, _) = ui.allocate_exact_size(egui::vec2(plot_w, ROW_H - 2.0), Sense::hover());
        let painter = ui.painter();
        painter.rect_filled(rect, 2.0, ui.visuals().extreme_bg_color);

        let n = ring.len();
        if n >= 2 {
            let (min_v, max_v) = ring.min_max();
            let span = (max_v - min_v).max(1e-3);
            let mut pts = Vec::with_capacity(n);
            let denom = (n - 1) as f32;
            for (i, v) in ring.iter().enumerate() {
                let x = rect.left() + 2.0 + (rect.width() - 4.0) * (i as f32 / denom);
                let norm = ((v - min_v) / span).clamp(0.0, 1.0);
                let y = rect.bottom() - 2.0 - (rect.height() - 4.0) * norm;
                pts.push(egui::pos2(x, y));
            }
            painter.add(egui::Shape::line(pts, egui::Stroke::new(1.5, color)));
        }

        let txt = if unit.is_empty() {
            format!("{:.0}", ring.latest())
        } else {
            format!("{:.1} {unit}", ring.latest())
        };
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add(egui::Label::new(RichText::new(txt).size(11.5).color(color)).truncate());
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_wgs84_origin_maps_osm_scenes_and_skips_non_geo() {
        assert_eq!(scene_wgs84_origin("gundo_osm"), Some((33.9138, -118.4190)));
        assert_eq!(scene_wgs84_origin("marina"), Some((33.9846, -118.4534)));
        assert_eq!(
            scene_wgs84_origin("sf_house_of_ai"),
            Some((37.7750, -122.4032))
        );
        assert_eq!(scene_wgs84_origin("gundo"), None);
        assert_eq!(scene_wgs84_origin("inspector"), None);
    }

    #[test]
    fn world_to_wgs84_exact_at_origin_and_1km_east_north() {
        let (lat0, lon0) = (33.9138, -118.4190);
        // Origin `(0, 41.1, 0)` reproduces exact anchor lat/lon and orthometric altitude.
        let (lat, lon, alt) = world_to_wgs84(lat0, lon0, [0.0, 41.1, 0.0]);
        assert!((lat - lat0).abs() < 1e-12, "origin lat={lat}");
        assert!((lon - lon0).abs() < 1e-12, "origin lon={lon}");
        assert!((alt - 41.1).abs() < 1e-5, "origin alt={alt}");

        // `+1000 m East (+X)` and `+1000 m North (-Z)` match WGS84 curvature radii.
        let (lat_ne, lon_ne, alt_ne) = world_to_wgs84(lat0, lon0, [1000.0, 125.5, -1000.0]);
        let dlat = lat_ne - lat0;
        let dlon = lon_ne - lon0;
        // At 33.9138° N: 1 km North ~= +0.0090148°, 1 km East ~= +0.0108144°.
        assert!(
            (dlat - 0.009_014_8).abs() < 2e-6,
            "1 km North dlat={dlat:.7} deg"
        );
        assert!(
            (dlon - 0.010_814_4).abs() < 2e-6,
            "1 km East dlon={dlon:.7} deg"
        );
        assert!((alt_ne - 125.5).abs() < 1e-4, "alt_ne={alt_ne}");

        // Non-finite inputs sanitize safely without NaN.
        let (lat_nan, lon_nan, alt_nan) =
            world_to_wgs84(lat0, lon0, [f32::NAN, f32::INFINITY, f32::NEG_INFINITY]);
        assert!((lat_nan - lat0).abs() < 1e-12);
        assert!((lon_nan - lon0).abs() < 1e-12);
        assert_eq!(alt_nan, 0.0);
    }

    #[test]
    fn slider_row_widths_never_overflow_narrow_or_wide_panels() {
        for total_w in [130.0_f32, 160.0, 190.0, 220.0, 260.0, 320.0, 480.0] {
            let (label_w, rem_w) = prop_row_widths(total_w);
            let (track_w, val_w) = slider_control_widths(rem_w);
            let used = label_w + 6.0 + track_w + 4.0 + val_w + 4.0 + ANIM_BTN_W;
            assert!(
                used <= total_w + 1e-3,
                "row overflow at total_w={total_w}: used={used} (label={label_w}, track={track_w}, val={val_w})"
            );
            assert!(track_w >= 16.0 && val_w >= 36.0 && label_w >= 46.0);
        }
        // Wide panels preserve canonical column widths.
        let (label_w, rem_w) = prop_row_widths(320.0);
        let (track_w, val_w) = slider_control_widths(rem_w);
        assert_eq!(label_w, LABEL_W);
        assert_eq!(val_w, VAL_W);
        assert!((track_w - 150.0).abs() < 1e-4);
    }

    #[test]
    fn home_pose_selects_csg_viewpoint_0_above_dem_when_csg_active() {
        let classic_home = CameraPose::from_eye_forward_up(
            glam::Vec3::new(0.0, 4.6, 5.0),
            -glam::Vec3::Z,
            glam::Vec3::Y,
            std::f32::consts::FRAC_PI_3,
            0.05,
            500.0,
        );
        let vp0 = SceneViewpoint {
            id: "el_segundo_bluff".to_string(),
            label: "El Segundo Bluff Overview".to_string(),
            eye: csg_core::math::Vec3::new(10.0, 65.0, -40.0),
            target: csg_core::math::Vec3::new(0.0, 41.1, 0.0),
            vfov_deg: 55.0,
        };
        let viewpoints = [vp0];

        // Classic mode (`csg_mode == 0`) preserves classic fractal home.
        let (pose_classic, dist_classic) = resolve_home_pose(0, &viewpoints, classic_home);
        assert_eq!(pose_classic.eye, classic_home.eye);
        assert!(dist_classic.is_none());

        // Active CSG scene (`csg_mode != 0`) resets to viewpoint `[0]` above DEM terrain!
        let (pose_csg, dist_csg) = resolve_home_pose(2, &viewpoints, classic_home);
        assert!(
            (pose_csg.eye.y - 65.0).abs() < 1e-4,
            "CSG Home must use viewpoint[0] above DEM (y=65.0), got {}",
            pose_csg.eye.y
        );
        assert!(pose_csg.zfar >= 2000.0);
        assert!(dist_csg.is_some_and(|d| d > 40.0));

        // Phase label reflects transient compile/build/IAS and live accumulation/ready states.
        assert_eq!(
            format_engine_phase_label(EnginePhase::CompilingOptix, 0, 256, None),
            "Compiling OptiX..."
        );
        assert_eq!(
            format_engine_phase_label(EnginePhase::BuildingCsgScene, 0, 256, None),
            "Building CSG Scene..."
        );
        assert_eq!(
            format_engine_phase_label(EnginePhase::UpdatingIas, 0, 256, None),
            "Updating IAS..."
        );
        assert_eq!(
            format_engine_phase_label(EnginePhase::Rendering, 12, 256, None),
            "Accumulating (12/256)"
        );
        assert_eq!(
            format_engine_phase_label(EnginePhase::Rendering, 256, 256, None),
            "Ready"
        );
    }

    #[test]
    fn road_mode_serde_defaults_to_hybrid_and_roundtrips() {
        // Legacy JSON missing `csg_road_mode` deserializes to Hybrid (`2`).
        let legacy: DemoSettings = serde_json::from_str("{}").expect("legacy json");
        assert_eq!(legacy.csg_road_mode, 2);

        // Explicit F-Rep Tile (`3`) round-trips cleanly.
        let s = DemoSettings {
            csg_road_mode: 3,
            ..DemoSettings::default()
        };
        let json = serde_json::to_string(&s).expect("serialize");
        let back: DemoSettings = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.csg_road_mode, 3);
    }
}
