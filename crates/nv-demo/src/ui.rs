//! Docked egui UI with Phosphor icons and uniformly aligned property grids.

use std::fmt::Write as _;
use std::ops::RangeInclusive;
use std::sync::LazyLock;

use camera::input::NavScheme;
use camera::{CameraDriver, CameraIntent, CameraPose, NavFeel, NavMode, ViewportSize};
use egui::{Align, Color32, FontDefinitions, Layout, RichText, Sense, Ui};
use egui_dock::{DockState, NodeIndex, Style, TabViewer};
use egui_phosphor::regular as ic;

use crate::app_events::{
    AppBus, EnginePhase, PossessInfo, SmartViewResult, StillFormat, UiCommand,
};
use crate::camera_rail::CameraRail;
use crate::csg_scene_host::CsgStats;
use crate::demo_anim::{DemoAnimator, EXPOSURE_RANGE, anim_range};
use crate::profiler::{FrameTimings, GpuTelemetry, HISTORY_LEN, RingBuffer};
use crate::renderer::{
    CameraProjection, CaptureSource, DLSS_QUALITY_MODES, DemoSettings, TIME_SCALE_MAX,
    TIME_SCALE_MIN, dlss_quality_index,
};
use crate::sdg_export::{ActiveCrowdBackend, CrowdBackend, SdgOccluderBackend};
use crate::viewport_toolbar::{self as vt, TbTone, tb_icon_button, tb_separator};
use crate::wireframe::DisplayMode;

/// Possession-toolbar icons of camera mounts by index (the label is the tooltip).
const MOUNT_ICONS: [&str; 9] = [
    ic::NUMBER_CIRCLE_ONE,
    ic::NUMBER_CIRCLE_TWO,
    ic::NUMBER_CIRCLE_THREE,
    ic::NUMBER_CIRCLE_FOUR,
    ic::NUMBER_CIRCLE_FIVE,
    ic::NUMBER_CIRCLE_SIX,
    ic::NUMBER_CIRCLE_SEVEN,
    ic::NUMBER_CIRCLE_EIGHT,
    ic::NUMBER_CIRCLE_NINE,
];
use csg_scene::{ProbeBackendKind, SceneDescriptor, SceneViewpoint, TimeOfDay};

/// One selectable output pass: `label` for combo boxes, `tag` for the viewport HUD and its toolbar.
struct AovPass {
    label: &'static str,
    tag: &'static str,
}

/// The 11 SDG/VFX output passes, indexed by `DemoSettings::aov_mode` (the shader's `aov_mode`).
const AOV_PASSES: [AovPass; 11] = [
    AovPass { label: "Beauty", tag: "BEAUTY" },
    AovPass { label: "Albedo", tag: "ALBEDO" },
    AovPass { label: "Normal", tag: "NORMAL" },
    AovPass { label: "Depth", tag: "DEPTH" },
    AovPass { label: "Motion", tag: "MOTION" },
    AovPass { label: "Semantic", tag: "SEMANTIC" },
    AovPass { label: "Instance", tag: "INSTANCE" },
    AovPass { label: "WorldPos", tag: "WORLDPOS" },
    AovPass { label: "Range (LiDAR)", tag: "RANGE" },
    AovPass { label: "Material", tag: "MATERIAL" },
    AovPass { label: "Cryptomatte", tag: "CRYPTO" },
];

/// The pass for `aov_mode`; an out-of-range mode reads as `Beauty`.
fn aov_pass(aov_mode: u32) -> &'static AovPass {
    AOV_PASSES.get(aov_mode as usize).unwrap_or(&AOV_PASSES[0])
}

/// Background sky labels, indexed by `DemoSettings::sky_mode` (the shader's `sky_mode`).
const SKY_MODES: [&str; 9] = [
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
/// `sky_mode` of the "Daylight" entry of [`SKY_MODES`] (time-of-day preset 0).
const SKY_DAYLIGHT: u32 = 0;
/// `sky_mode` of the "Sunset" entry of [`SKY_MODES`] (time-of-day preset 1).
const SKY_SUNSET: u32 = 1;
/// `sky_mode` of the "Coastal Night" entry of [`SKY_MODES`] (time-of-day preset 2).
const SKY_COASTAL_NIGHT: u32 = 8;

const FORCE_LOD_LABELS: [&str; 4] = ["Auto", "LOD0", "LOD1", "LOD2"];

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

/// WGS84 anchor of the scene selected by `csg_mode` (`0` = classic, `n` = `descs[n - 1]`), read
/// from the scene's own [`SceneDescriptor::geo_origin`]. `None` for the classic stage and for
/// synthetic scenes, which then show no coordinates.
fn active_geo_origin(descs: &[SceneDescriptor], csg_mode: u32) -> Option<csg_scene::GeoAnchor> {
    csg_mode
        .checked_sub(1)
        .and_then(|i| descs.get(i as usize))
        .and_then(|d| d.geo_origin)
}

/// `Inspector` variant index of the 4-archetype crowd showcase.
const INSPECTOR_CROWD_SHOWCASE_VARIANT: u32 = csg_scenes::ACTOR_SHOWCASE_VARIANT_IDX as u32;

/// `true` for the `Inspector` crowd showcase variant, where a poke hits every actor instead of the
/// nearest one.
fn is_crowd_showcase(desc: &SceneDescriptor, asset_idx: u32) -> bool {
    desc.id == "inspector" && asset_idx == INSPECTOR_CROWD_SHOWCASE_VARIANT
}

/// [`is_crowd_showcase`] for the scene selected by `csg_mode` (`0` = classic, `n` = `descs[n - 1]`).
fn is_inspector_crowd_showcase(descs: &[SceneDescriptor], csg_mode: u32, asset_idx: u32) -> bool {
    csg_mode
        .checked_sub(1)
        .and_then(|i| descs.get(i as usize))
        .is_some_and(|d| is_crowd_showcase(d, asset_idx))
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

/// Reusable per-frame UI text scratch buffers for the 60 Hz HUD, telemetry and inspector text.
///
/// They remove the formatting allocations of the strings built here; egui itself still allocates
/// per widget text (`WidgetText` / galley jobs own a `String`), so the UI pass is not
/// allocation-free.
#[derive(Debug)]
pub struct UiScratch {
    /// Reusable buffer for the bottom-left Viewport status HUD line.
    pub status_buf: String,
    /// Reusable buffer for short telemetry/badge/toolbar formatted strings.
    pub label_buf: String,
}

impl Default for UiScratch {
    fn default() -> Self {
        Self {
            status_buf: String::with_capacity(192),
            label_buf: String::with_capacity(96),
        }
    }
}

/// Formats `args` into `buf` in place (clearing previous contents with zero reallocation once warmed).
#[inline]
fn fmt_into<'a>(buf: &'a mut String, args: std::fmt::Arguments<'_>) -> &'a str {
    buf.clear();
    // `std::fmt::Write` for `String` is infallible.
    let Ok(()) = std::fmt::write(buf, args) else {
        unreachable!("formatting into String cannot fail");
    };
    buf.as_str()
}

/// Static display label for [`NavScheme`] without per-frame `format!("{s:?}")` allocations.
#[inline]
const fn nav_scheme_label(s: NavScheme) -> &'static str {
    match s {
        NavScheme::Default => "Default",
        NavScheme::Studio => "Studio",
        NavScheme::Maya => "Maya",
        NavScheme::Houdini => "Houdini",
        NavScheme::Blender => "Blender",
        NavScheme::Unreal => "Unreal",
    }
}

/// Static display label for [`NavFeel`] without per-frame `format!("{f:?}")` allocations.
#[inline]
const fn nav_feel_label(f: NavFeel) -> &'static str {
    match f {
        NavFeel::Sharp => "Sharp",
        NavFeel::Smooth => "Smooth",
    }
}

/// Cesium/UE5-style altitude-aware flight speed multiplier (nv-demo host policy, fed into the
/// driver's `scene_radius` and shown on the HUD).
///
/// Keeps close-up ground/bolt inspection (`altitude <= 8 m`) at exact `1.0x`, scaling smoothly
/// up to `25.0x` at high aerial altitude (`>= 200 m`) so orbital/map sweeps feel effortless.
#[inline]
pub(crate) fn altitude_speed_multiplier(altitude_m: f32) -> f32 {
    if !altitude_m.is_finite() {
        return 1.0;
    }
    (altitude_m.max(2.0) / 8.0).clamp(1.0, 25.0)
}

/// Synchronizes camera toolbar dropdown selections into both `CameraController` and `NavPrefs`
/// so `CameraDriver::sync_prefs()` never reverts the selected orbit mode on the following frame.
pub(crate) fn apply_camera_toolbar_prefs(
    driver: &mut CameraDriver,
    mode: NavMode,
    fly_mode: NavMode,
    feel: NavFeel,
    scheme: NavScheme,
    scheme_changed: bool,
) {
    driver.controller_mut().set_mode(mode);
    let mut prefs = *driver.prefs();
    prefs.mode = mode;
    prefs.hold_fly_mode = fly_mode;
    prefs.feel = feel;
    driver.set_prefs(prefs);
    if scheme_changed {
        driver.set_scheme(scheme);
        crate::enable_rmb_fly(driver);
    }
}

/// Appends the human-readable engine phase prefix into `out` without intermediate allocations.
fn write_engine_phase_label(
    out: &mut String,
    phase: EnginePhase,
    accum_count: u32,
    max_accum: u32,
    anim_tags: &[&str],
) {
    if let Some(label) = phase.transient_label() {
        out.push_str(label);
        return;
    }
    if !anim_tags.is_empty() {
        out.push_str("Live (");
        for (i, tag) in anim_tags.iter().enumerate() {
            if i > 0 {
                out.push('+');
            }
            out.push_str(tag);
        }
        out.push(')');
        return;
    }
    let cap = max_accum.max(1);
    if accum_count >= cap {
        out.push_str("Ready");
    } else {
        let Ok(()) = write!(out, "Accumulating ({accum_count}/{cap})") else {
            unreachable!("writing to String cannot fail");
        };
    }
}

/// Formats the human-readable engine phase prefix for the bottom-left Viewport status bar.
#[cfg(test)]
pub fn format_engine_phase_label(
    phase: EnginePhase,
    accum_count: u32,
    max_accum: u32,
    anim_tag: Option<&str>,
) -> String {
    let mut out = String::with_capacity(32);
    match anim_tag {
        Some(tag) => write_engine_phase_label(&mut out, phase, accum_count, max_accum, &[tag]),
        None => write_engine_phase_label(&mut out, phase, accum_count, max_accum, &[]),
    }
    out
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DockTab {
    Viewport,
    Fractal,
    RenderFloor,
    CameraStats,
}

impl DockTab {
    pub const ALL: [Self; 4] = [
        Self::Viewport,
        Self::Fractal,
        Self::RenderFloor,
        Self::CameraStats,
    ];

    pub fn title(self) -> &'static str {
        static TITLES: LazyLock<[String; 4]> = LazyLock::new(|| {
            [
                format!("{}  Viewport", ic::APERTURE),
                format!("{}  Fractal", ic::CUBE),
                format!("{}  Render & Floor", ic::SUN),
                format!("{}  Camera & Stats", ic::GAUGE),
            ]
        });
        let idx = match self {
            Self::Viewport => 0,
            Self::Fractal => 1,
            Self::RenderFloor => 2,
            Self::CameraStats => 3,
        };
        TITLES[idx].as_str()
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
    pub last_cam_raw_input: &'a mut Option<camera::input::RawInput>,
    pub scratch: &'a mut UiScratch,
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
    pub possess_toolbar: &'a mut crate::viewport_toolbar::PossessToolbar,
    pub pick_possess_mode: &'a mut bool,
    pub smart_view_seed: &'a mut u64,
    pub last_smart_view: Option<SmartViewResult>,
    pub last_saved_msg: Option<&'a str>,
    /// Rail fly-through statistics (Flight stats subsection of the rail panel).
    pub flight: &'a mut crate::flight_stats::FlightStats,
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
                .selected_text(nav_scheme_label(*self.cam_scheme))
                .show_ui(ui, |ui| {
                    for s in NavScheme::ALL {
                        ui.selectable_value(self.cam_scheme, s, nav_scheme_label(s));
                    }
                });

            ui.label(RichText::new("Feel").small().weak());
            egui::ComboBox::from_id_salt("toolbar_cam_feel")
                .width(74.0)
                .selected_text(nav_feel_label(*self.cam_feel))
                .show_ui(ui, |ui| {
                    for f in NavFeel::ALL {
                        ui.selectable_value(self.cam_feel, f, nav_feel_label(f));
                    }
                });

            if (
                *self.cam_mode,
                *self.cam_fly_mode,
                *self.cam_scheme,
                *self.cam_feel,
            ) != (prev_m, prev_fly, prev_s, prev_f)
            {
                apply_camera_toolbar_prefs(
                    self.cam_driver,
                    *self.cam_mode,
                    *self.cam_fly_mode,
                    *self.cam_feel,
                    *self.cam_scheme,
                    *self.cam_scheme != prev_s,
                );
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
            egui::ComboBox::from_id_salt("toolbar_aov_mode")
                .width(102.0)
                .selected_text(aov_pass(self.settings.aov_mode).label)
                .show_ui(ui, |ui| {
                    for (idx, pass) in AOV_PASSES.iter().enumerate() {
                        ui.selectable_value(&mut self.settings.aov_mode, idx as u32, pass.label);
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
        // Viewport toolbar, plus the possession toolbar stacked on its inner side in the same
        // corner (viewport_toolbar.rs); both share one frame and one set of design tokens.
        let wire_mode = self.settings.display_mode == DisplayMode::Wireframe;
        let toolbar_resp = vt::ViewportToolbar::new("nv_demo_viewport_toolbar")
            .show(ui.ctx(), rect, self.viewport_toolbar, |ui| {
                // 1. Display mode (DEMO:VIEWS), AOV / line width, lens
                egui::ComboBox::from_id_salt("vt_display")
                    .selected_text(format!("{} {}", ic::CUBE, self.settings.display_mode.label()))
                    .width(vt::TB_COMBO_WIDTH)
                    .show_ui(ui, |ui| {
                        for mode in DisplayMode::ALL {
                            ui.selectable_value(&mut self.settings.display_mode, mode, mode.label());
                        }
                    })
                    .response
                    .on_hover_text(
                        "Display mode: CSG = shaded path-traced render; Wireframe = smooth \
                         hidden-line contours (silhouettes, creases, instance borders).",
                    );
                if wire_mode {
                    ui.add_sized(
                        vt::TB_BUTTON_SIZE,
                        egui::Label::new(RichText::new(ic::PEN_NIB).size(vt::TB_ICON_SIZE)),
                    )
                    .on_hover_text("Wireframe line width");
                    ui.add(
                        egui::Slider::new(
                            &mut self.settings.wire_line_px,
                            crate::wireframe::WIRE_LINE_PX_MIN..=crate::wireframe::WIRE_LINE_PX_MAX,
                        )
                        .fixed_decimals(2)
                        .suffix(" px"),
                    )
                    .on_hover_text("Silhouette stroke width in screen pixels (creases are 2/3 of it)");
                } else {
                    let aov_cur = aov_pass(self.settings.aov_mode).tag;
                    egui::ComboBox::from_id_salt("vt_aov")
                        .selected_text(format!("{} {aov_cur}", ic::STACK))
                        .width(vt::TB_COMBO_WIDTH)
                        .show_ui(ui, |ui| {
                            for (idx, pass) in AOV_PASSES.iter().enumerate() {
                                ui.selectable_value(&mut self.settings.aov_mode, idx as u32, pass.tag);
                            }
                        })
                        .response
                        .on_hover_text("Output pass (AOV)");
                }
                egui::ComboBox::from_id_salt("vt_lens")
                    .selected_text(format!(
                        "{} {}",
                        ic::APERTURE,
                        self.settings.camera_projection.label()
                    ))
                    .width(vt::TB_COMBO_WIDTH)
                    .show_ui(ui, |ui| {
                        for proj in CameraProjection::ALL {
                            ui.selectable_value(
                                &mut self.settings.camera_projection,
                                proj,
                                proj.label(),
                            );
                        }
                    })
                    .response
                    .on_hover_text("Lens / sensor projection");
                tb_separator(ui);

                // 2. Day / Sunset / Night (routed through the smooth transition)
                for (tod, icon, tip) in [
                    (0u32, ic::SUN, "Daylight"),
                    (1u32, ic::SUN_HORIZON, "Golden Sunset"),
                    (2u32, ic::MOON_STARS, "Coastal Night"),
                ] {
                    if tb_icon_button(ui, icon, self.settings.csg_time_of_day == tod, TbTone::Normal, tip)
                        .clicked()
                    {
                        self.bus.emit_ui(UiCommand::SetTimeOfDay(tod));
                    }
                }
                tb_separator(ui);

                // 3. Poke / Freeze
                if self.settings.csg_mode > 0
                    && tb_icon_button(
                        ui,
                        ic::HAND_POINTING,
                        false,
                        TbTone::Normal,
                        "Poke nearest articulated actor (P)",
                    )
                    .clicked()
                {
                    let is_inspector_109 = is_inspector_crowd_showcase(
                        self.scene_descriptors,
                        self.settings.csg_mode,
                        self.settings.csg_asset_idx,
                    );
                    self.bus.emit_ui(UiCommand::PokeCrowd {
                        impulse: self.settings.poke_impulse,
                        all: is_inspector_109,
                    });
                }
                if tb_icon_button(
                    ui,
                    ic::PAUSE,
                    self.settings.csg_freeze_pose,
                    TbTone::Normal,
                    "Freeze dynamic traffic & crowd poses for clean accumulation",
                )
                .clicked()
                {
                    self.bus.emit_ui(UiCommand::ToggleFreezePose);
                }
                tb_separator(ui);

                // 4. Exposure EV & LOD bias
                ui.add(
                    egui::DragValue::new(&mut self.settings.exposure)
                        .range(EXPOSURE_RANGE)
                        .speed(0.02)
                        .prefix("EV "),
                )
                .on_hover_text("Exposure multiplier");
                if self.settings.csg_mode > 0 {
                    ui.add_sized(
                        vt::TB_BUTTON_SIZE,
                        egui::Label::new(RichText::new(ic::GAUGE).size(vt::TB_ICON_SIZE)),
                    )
                    .on_hover_text("CSG LOD bias");
                    ui.add_enabled(
                        self.settings.csg_lod_enabled,
                        egui::Slider::new(&mut self.settings.csg_lod_bias, 0.25..=4.0)
                            .logarithmic(true)
                            .fixed_decimals(2)
                            .suffix("x"),
                    )
                    .on_hover_text("CSG geometric LOD detail bias (higher = more detail at distance)");
                }
                tb_separator(ui);

                // 5. Save PNG & SDG EXR burst
                if tb_icon_button(ui, ic::CAMERA, false, TbTone::Normal, "Save PNG snapshot").clicked() {
                    self.bus.emit_ui(UiCommand::SaveStill(StillFormat::Png));
                }
                if self.settings.csg_mode > 0 {
                    if tb_icon_button(
                        ui,
                        ic::FILM_STRIP,
                        false,
                        TbTone::Normal,
                        "Export multi-frame 9-channel OpenEXR + SDG JSON burst",
                    )
                    .clicked()
                    {
                        self.bus.emit_ui(UiCommand::StartSdgBurst {
                            frames: self.settings.sdg_burst_frames,
                        });
                    }
                    if tb_icon_button(
                        ui,
                        ic::STOP_CIRCLE,
                        false,
                        TbTone::Normal,
                        "Cancel the running SDG burst",
                    )
                    .clicked()
                    {
                        self.bus.emit_ui(UiCommand::CancelSdgBurst);
                    }
                }
            });

        // Possession toolbar: collapsed to its handle while not possessing, auto-expanded on
        // entering possession unless the user collapsed it, collapsed on leaving unless pinned.
        let possessing = self.possess_info.is_some();
        let possess_toolbar = &mut *self.possess_toolbar;
        possess_toolbar.sync(possessing);
        possess_toolbar.bar.edge = self.viewport_toolbar.edge;
        let stack = vt::stack_offset(self.viewport_toolbar.edge, rect, toolbar_resp.rect);
        let prefs = &mut possess_toolbar.prefs;
        let possess_resp = vt::ViewportToolbar::new("nv_demo_possess_toolbar")
            .stack(stack)
            .show(ui.ctx(), rect, &mut possess_toolbar.bar, |ui| {
                if let Some(pinfo) = self.possess_info {
                    let (icon, tag) = if pinfo.is_dynamic {
                        (ic::STEERING_WHEEL, "Riding")
                    } else {
                        (ic::CAR, "Possessing")
                    };
                    ui.add_sized(
                        vt::TB_BUTTON_SIZE,
                        egui::Label::new(
                            RichText::new(icon)
                                .size(vt::TB_ICON_SIZE)
                                .color(Color32::from_rgb(174, 236, 194)),
                        ),
                    )
                    .on_hover_ui(|ui| {
                        ui.label(format!("{tag}: {}", pinfo.assembly_name));
                    });
                    ui.add(
                        egui::Label::new(
                            RichText::new(&pinfo.assembly_name)
                                .small()
                                .color(Color32::from_rgb(174, 236, 194)),
                        )
                        .truncate(),
                    );
                    tb_separator(ui);
                    for (idx, mount) in pinfo.mounts.iter().enumerate() {
                        let icon = MOUNT_ICONS.get(idx).copied().unwrap_or(ic::DOTS_THREE);
                        if tb_icon_button(ui, icon, idx == pinfo.mount_idx, TbTone::Normal, &mount.label)
                            .clicked()
                        {
                            self.bus.emit_ui(UiCommand::SelectPossessMount { mount_idx: idx });
                        }
                    }
                    tb_separator(ui);
                    if tb_icon_button(
                        ui,
                        ic::ALIGN_CENTER_HORIZONTAL,
                        *self.possess_auto_level,
                        TbTone::Normal,
                        "Level: keep the horizon level while riding vehicles",
                    )
                    .clicked()
                    {
                        *self.possess_auto_level = !*self.possess_auto_level;
                    }
                    if tb_icon_button(ui, ic::EJECT, false, TbTone::Danger, "Eject (Esc)").clicked() {
                        self.bus.emit_ui(UiCommand::EjectPossess);
                    }
                } else {
                    let csg = self.settings.csg_mode != 0;
                    ui.add_enabled_ui(csg, |ui| {
                        if tb_icon_button(
                            ui,
                            ic::CROSSHAIR_SIMPLE,
                            *self.pick_possess_mode,
                            TbTone::Normal,
                            "Pick: click any vehicle, lamp, yacht or building to possess its camera \
                             mount (also Ctrl + Left Click or double-click in the viewport)",
                        )
                        .clicked()
                        {
                            *self.pick_possess_mode = !*self.pick_possess_mode;
                        }
                        if tb_icon_button(
                            ui,
                            ic::CAR,
                            false,
                            TbTone::Normal,
                            "Possess Car: jump into a driving vehicle or mounted landmark (cycles targets)",
                        )
                        .clicked()
                        {
                            self.bus.emit_ui(UiCommand::PossessNextVehicle);
                        }
                    });
                }
                tb_separator(ui);
                let pin_tip = if prefs.pinned {
                    "Pinned open: stays expanded after leaving possession (click to unpin)"
                } else {
                    "Pin open: keep this toolbar expanded after leaving possession"
                };
                if tb_icon_button(ui, ic::PUSH_PIN, prefs.pinned, TbTone::Normal, pin_tip).clicked() {
                    prefs.pinned = !prefs.pinned;
                }
            });
        if possess_resp.toggled {
            possess_toolbar.prefs.on_user_toggle(possess_toolbar.bar.expanded);
        }
        let toolbar_rects = [toolbar_resp.rect, possess_resp.rect];

        let toolbar_blocking = toolbar_resp.contains_pointer || possess_resp.contains_pointer;
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
        // Viewport hotkey: gated on `viewport_focused` like `F` and `A` (F29).
        if kb_free
            && *self.viewport_focused
            && self.settings.csg_mode > 0
            && ui.input(|i| i.key_pressed(egui::Key::P))
        {
            let is_inspector_109 = is_inspector_crowd_showcase(
                self.scene_descriptors,
                self.settings.csg_mode,
                self.settings.csg_asset_idx,
            );
            self.bus.emit_ui(UiCommand::PokeCrowd {
                impulse: self.settings.poke_impulse,
                all: is_inspector_109,
            });
        }
        // Lockstep subframes replay this input; the camera branch below refreshes it. When that
        // branch is skipped (possessed, split drag, toolbar, pick) a stale copy would keep held
        // keys alive in the recording (F08).
        *self.last_cam_raw_input = None;
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
                Some(fixed_dt) => {
                    let raw = camera::egui::read_input(ui, &resp, fixed_dt);
                    *self.last_cam_raw_input = Some(raw);
                    if raw.hovered {
                        camera::egui::consume_wheel(ui.ctx());
                    }
                    let frame = self.cam_driver.update(&raw, fly_scene_radius);
                    camera::egui::apply_cursor(ui.ctx(), resp.id, frame.cursor);
                    if frame.wants_repaint {
                        ui.ctx().request_repaint();
                    }
                    frame
                }
                None => camera::egui::drive(ui, &resp, self.cam_driver, fly_scene_radius),
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

        // Compact bottom-left status badge inside viewport (formatted into reusable scratch buffer).
        // Bottom overlays stack upwards from the bottom edge, above a bottom-docked toolbar.
        let bottom_reserve = vt::bottom_reserve(self.viewport_toolbar.edge, rect, &toolbar_rects);
        let hud_pos = rect.left_bottom() + egui::vec2(10.0, -26.0 - bottom_reserve);
        let aov_tag = match self.settings.display_mode {
            DisplayMode::Csg => aov_pass(self.settings.aov_mode).tag,
            DisplayMode::Wireframe => DisplayMode::Wireframe.tag(),
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
        let nav = self.cam_driver.navigator();
        let fly_mult = nav.fly_speed();
        let shift_ramp = nav.shift_fly_ramp();
        let alt_mult = altitude_speed_multiplier(self.cam_pose.eye.y);
        let in_fly_hud = self.cam_driver.is_hold_flying() || self.cam_driver.controller().is_fly();

        let mut anim_tags: [&str; 4] = [""; 4];
        let mut anim_tag_len = 0usize;
        if demo_running {
            anim_tags[anim_tag_len] = "DEMO";
            anim_tag_len += 1;
        }
        if self.settings.auto_rotate {
            anim_tags[anim_tag_len] = "SPIN";
            anim_tag_len += 1;
        }
        if sat_orbiting {
            anim_tags[anim_tag_len] = "ORBIT";
            anim_tag_len += 1;
        }
        if traffic_moving {
            anim_tags[anim_tag_len] = "TRAFFIC";
            anim_tag_len += 1;
        }
        let anim_slice = &anim_tags[..anim_tag_len];
        let phase_icon = if self.engine_phase.transient_label().is_some() {
            ic::SPINNER
        } else if !anim_slice.is_empty() {
            ic::ARROWS_CLOCKWISE
        } else if self.accum_count >= self.settings.max_accum.max(1) {
            ic::CHECK_CIRCLE
        } else {
            ic::STACK
        };

        let status_buf = &mut self.scratch.status_buf;
        status_buf.clear();
        status_buf.push_str(phase_icon);
        status_buf.push(' ');
        write_engine_phase_label(
            status_buf,
            self.engine_phase,
            self.accum_count,
            self.settings.max_accum,
            anim_slice,
        );
        let Ok(()) = write!(
            status_buf,
            "  |  {}x{}  |  {}",
            self.render_w, self.render_h, aov_tag
        ) else {
            unreachable!("writing to String cannot fail");
        };
        if self.settings.camera_projection != CameraProjection::Pinhole {
            status_buf.push_str("  |  ");
            status_buf.push_str(self.settings.camera_projection.label());
        }
        if self.possess_info.is_some() {
            status_buf.push_str("  |  POSSESS");
        }
        if self.settings.csg_mode > 0 && self.settings.csg_freeze_pose {
            status_buf.push_str("  |  FROZEN");
        }
        if in_fly_hud || (fly_mult - 1.0).abs() > 0.01 || shift_ramp > 1.05 {
            let eff = fly_mult * alt_mult * shift_ramp;
            let res = if shift_ramp > 1.05 {
                write!(status_buf, "  |  FLY {eff:.1}x (SHIFT {shift_ramp:.1}x)")
            } else {
                write!(status_buf, "  |  FLY {eff:.1}x")
            };
            let Ok(()) = res else {
                unreachable!("writing to String cannot fail");
            };
        }
        if self.fast_preview {
            status_buf.push_str("  |  PREVIEW");
        }
        let Ok(()) = write!(
            status_buf,
            "  |  UI {} FPS  |  RT {:.1} FPS ({:.1} ms)",
            self.ui_fps.round() as u32,
            self.rt_fps,
            self.rt_ms
        ) else {
            unreachable!("writing to String cannot fail");
        };
        let galley = ui.painter().layout_no_wrap(
            status_buf.clone(),
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
        if let Some(origin) = active_geo_origin(self.scene_descriptors, self.settings.csg_mode) {
            let (lat, lon, alt_m) =
                world_to_wgs84(origin.lat_deg, origin.lon_deg, self.cam_pose.eye.to_array());
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
        collapsing_section(ui, "sec_presets", ic::SPARKLE, "Presets", |ui| {
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

        collapsing_section(ui, "sec_formula", ic::CUBE, "Formula", |ui| {
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
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "bulb_bailout",
                        "Bailout",
                        &mut s.bulb_bailout,
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
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "bulb_phi_mult",
                        "Phi Mult",
                        &mut s.bulb_phi_mult,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "bulb_theta_phase",
                        "Theta Ph",
                        &mut s.bulb_theta_phase,
                        1,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "bulb_phi_phase",
                        "Phi Ph",
                        &mut s.bulb_phi_phase,
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
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "box_fold",
                        "Fold",
                        &mut s.box_fold,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "box_fold_mult",
                        "Fold Mult",
                        &mut s.box_fold_mult,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "box_min_r",
                        "Min R",
                        &mut s.box_min_r,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "box_fixed_r",
                        "Fixed R",
                        &mut s.box_fixed_r,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "box_iter_rot",
                        "Iter Rot",
                        &mut s.box_iter_rot,
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
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "kifs_rot1",
                        "Rot Y",
                        &mut s.kifs_rot1,
                        1,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "kifs_rot2",
                        "Rot X",
                        &mut s.kifs_rot2,
                        1,
                        false,
                    );
                    anim_vec3(
                        ui,
                        anim,
                        "kifs_offset",
                        "Offset",
                        &mut s.kifs_offset,
                        0.01,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "kifs_thickness",
                        "Thickness",
                        &mut s.kifs_thickness,
                        3,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "kifs_z_bias",
                        "Z Bias",
                        &mut s.kifs_z_bias,
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
            |ui| {
                anim_vec3(
                    ui,
                    anim,
                    "fractal_translate",
                    "Translate",
                    &mut s.fractal_translate,
                    0.05,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "rot_x",
                    "Rot X",
                    &mut s.rot_x,
                    1,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "rot_y",
                    "Rot Y",
                    &mut s.rot_y,
                    1,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "rot_z",
                    "Rot Z",
                    &mut s.rot_z,
                    1,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "obj_scale",
                    "Scale",
                    &mut s.obj_scale,
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
            |ui| {
                anim_slider_f32(
                    ui,
                    anim,
                    "detail_eps",
                    "Detail Eps",
                    &mut s.detail_eps,
                    5,
                    true,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "norm_eps",
                    "Normal Eps",
                    &mut s.norm_eps,
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
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "roughness",
                    "Roughness",
                    &mut s.roughness,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "trap_scale",
                    "Trap Scale",
                    &mut s.trap_scale,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "palette_shift",
                    "Hue Shift",
                    &mut s.palette_shift,
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

            collapsing_section(ui, "sec_csg", ic::BUILDINGS, "CSG World", |ui| {
                let prev_mode = s.csg_mode;
                let total_choices = descs.len() + 1;

                // Scene selector buttons driven strictly by `SceneRegistry::descriptors()`.
                let cols = total_choices.clamp(1, 3);
                equal_grid(ui, cols, total_choices, |ui, idx, w| {
                    let (mode, label, tip) = if idx == 0 {
                        (0u32, "Off", "Classic SDF fractal + stage floor")
                    } else if let Some(d) = descs.get(idx - 1) {
                        (idx as u32, d.name, d.description)
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

                if s.csg_mode != prev_mode {
                    if let Some(desc) = active_desc {
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
                    }
                    // Every scene change, "Off" included, goes through `SelectScene` so main
                    // pauses the rail and ejects a possessed body (F25).
                    request_auto_frame = true;
                }

                if let Some(desc) = active_desc {
                    ui.add_space(2.0);
                    if !desc.variant_labels.is_empty() {
                        let prev_var = s.csg_asset_idx;
                        let max_idx = (desc.variant_labels.len() - 1) as u32;
                        s.csg_asset_idx = s.csg_asset_idx.min(max_idx);
                        prop_row(ui, "Kit Asset", |ui| {
                            let gap = 3.0;
                            let btn_w = ROW_H;
                            let combo_w =
                                (ui.available_width() - 2.0 * btn_w - 2.0 * gap).max(32.0);
                            ui.spacing_mut().item_spacing.x = gap;
                            if ui
                                .add_sized([btn_w, ROW_H], egui::Button::new(ic::CARET_LEFT))
                                .on_hover_text("Previous asset")
                                .clicked()
                            {
                                s.csg_asset_idx = if s.csg_asset_idx == 0 {
                                    max_idx
                                } else {
                                    s.csg_asset_idx - 1
                                };
                            }
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
                            if ui
                                .add_sized([btn_w, ROW_H], egui::Button::new(ic::CARET_RIGHT))
                                .on_hover_text("Next asset")
                                .clicked()
                            {
                                s.csg_asset_idx = if s.csg_asset_idx >= max_idx {
                                    0
                                } else {
                                    s.csg_asset_idx + 1
                                };
                            }
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
                        fmt_into(
                            &mut self.scratch.label_buf,
                            format_args!(
                                "Crowd Backend ({} {:.2} ms, {}a)",
                                self.crowd_active_backend.label(),
                                self.crowd_step_ms,
                                self.crowd_actor_count
                            ),
                        )
                    } else {
                        "Crowd Backend"
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
                    let is_inspector_109 = is_crowd_showcase(desc, s.csg_asset_idx);
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
                                    "Apply a stumble/flinch impulse to the nearest actor along view (or all actors in the Inspector crowd showcase). Hotkey: P.",
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
        let csg_stats = self.csg_stats;
        let label_buf = &mut self.scratch.label_buf;

        collapsing_section(ui, "sec_lod", ic::STACK, "LOD & Culling", |ui| {
            lod_telemetry_strip(ui, label_buf, csg_stats, s.csg_mode != 0);
            ui.add_space(2.0);
            prop_toggle(ui, "LOD Enabled", &mut s.csg_lod_enabled, "Distance & SSE LOD");
            ui.add_enabled_ui(s.csg_lod_enabled, |ui| {
                prop_row(ui, "Force LOD", |ui| {
                    let gap = 4.0;
                    let combo_w = (ui.available_width() - ANIM_BTN_W - gap).max(32.0);
                    ui.spacing_mut().item_spacing.x = gap;
                    let cur = (s.csg_force_lod as usize).min(FORCE_LOD_LABELS.len() - 1);
                    egui::ComboBox::from_id_salt("csg_force_lod_combo")
                        .width(combo_w)
                        .truncate()
                        .selected_text(FORCE_LOD_LABELS[cur])
                        .show_ui(ui, |ui| {
                            const TIPS: [&str; 4] = [
                                "Select LOD0 / LOD1 / LOD2 dynamically by camera distance and screen-space size.",
                                "Force full-detail LOD0 on all CSG assemblies and crowd actors.",
                                "Force mid-detail LOD1 on all CSG assemblies and crowd actors.",
                                "Force coarse proxy LOD2 on all CSG assemblies and crowd actors.",
                            ];
                            for (idx, lbl) in FORCE_LOD_LABELS.iter().enumerate() {
                                ui.selectable_value(&mut s.csg_force_lod, idx as u32, *lbl)
                                    .on_hover_text(TIPS[idx]);
                            }
                        });
                    anim_toggle_btn_disabled(ui);
                });
                anim_slider_f32(
                    ui,
                    anim,
                    "csg_lod_bias",
                    "LOD Bias",
                    &mut s.csg_lod_bias,
                    2,
                    true,
                );
                let prev_lod = *s;
                ui.add_space(2.0);
                ui.label(RichText::new("Scene Distances & SSE").small().weak());
                prop_slider_f32(ui, "LOD1 Dist", &mut s.csg_lod1_dist_m, 10.0..=500.0, 0, "m");
                prop_slider_f32(ui, "LOD2 Dist", &mut s.csg_lod2_dist_m, 30.0..=1200.0, 0, "m");
                prop_slider_f32(ui, "Meso Cull", &mut s.csg_meso_dist_m, 30.0..=1200.0, 0, "m");
                prop_slider_f32(ui, "Micro Cull", &mut s.csg_micro_dist_m, 5.0..=200.0, 0, "m");
                prop_slider_f32(ui, "Min Screen Px", &mut s.csg_min_screen_px, 0.0..=16.0, 1, "px");
                prop_slider_f32(ui, "Hysteresis", &mut s.csg_lod_hysteresis, 0.0..=0.45, 2, "");
                ui.add_space(2.0);
                ui.label(RichText::new("Crowd LOD").small().weak());
                prop_slider_f32(ui, "Crowd LOD1", &mut s.csg_crowd_lod1_m, 5.0..=120.0, 0, "m");
                prop_slider_f32(ui, "Crowd LOD2", &mut s.csg_crowd_lod2_m, 20.0..=300.0, 0, "m");
                prop_slider_f32(ui, "Crowd Cull", &mut s.csg_crowd_cull_m, 40.0..=600.0, 0, "m");
                sync_lod_distance_order(s, &prev_lod);
            });
            ui.add_space(2.0);
            let reset_lbl = fmt_into(
                label_buf,
                format_args!("{}  Reset LOD Defaults", ic::ARROW_COUNTER_CLOCKWISE),
            );
            if ui
                .add_sized(
                    [ui.available_width(), ROW_H + 4.0],
                    egui::Button::new(reset_lbl).truncate(),
                )
                .on_hover_text(
                    "Restore all CSG distance, screen-space, and crowd LOD settings to defaults.",
                )
                .clicked()
            {
                reset_csg_lod_defaults(s);
            }
        });

        collapsing_section(ui, "sec_sky", ic::SUN, "Sky & Lighting", |ui| {
            prop_row(ui, "Background", |ui| {
                let gap = 4.0;
                let combo_w = (ui.available_width() - ANIM_BTN_W - gap).max(32.0);
                ui.spacing_mut().item_spacing.x = gap;
                let idx = (s.sky_mode as usize).min(SKY_MODES.len() - 1);
                egui::ComboBox::from_id_salt("sky_mode_combo")
                    .width(combo_w)
                    .truncate()
                    .selected_text(SKY_MODES[idx])
                    .show_ui(ui, |ui| {
                        for (i, label) in SKY_MODES.iter().enumerate() {
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
                2,
                false,
            );
            anim_slider_f32(
                ui,
                anim,
                "sun_azimuth",
                "Sun Azimuth",
                &mut s.sun_azimuth,
                1,
                false,
            );
            anim_slider_f32(
                ui,
                anim,
                "sun_elevation",
                "Sun Elev",
                &mut s.sun_elevation,
                1,
                false,
            );
            anim_slider_f32(
                ui,
                anim,
                "sky_intensity",
                "Sky Power",
                &mut s.sky_intensity,
                2,
                false,
            );
            anim_slider_f32(
                ui,
                anim,
                "horizon_haze",
                "Haze",
                &mut s.horizon_haze,
                2,
                false,
            );
            anim_slider_f32(
                ui,
                anim,
                "exposure",
                "Exposure",
                &mut s.exposure,
                2,
                true,
            );

            match s.sky_mode {
                0 | 1 => {
                    anim_slider_f32(ui, anim, "env_cloud_cover", "Cloud Cover", &mut s.env.cloud_cover, 2, false);
                    anim_slider_f32(ui, anim, "env_cloud_density", "Cloud Density", &mut s.env.cloud_density, 2, false);
                }
                2 => {
                    anim_slider_f32(ui, anim, "env_softbox_power", "Panel Power", &mut s.env.softbox_power, 2, false);
                    anim_slider_f32(ui, anim, "env_softbox_softness", "Edge Softness", &mut s.env.softbox_softness, 2, false);
                }
                3 => {
                    anim_slider_f32(ui, anim, "env_nebula_density", "Nebula Density", &mut s.env.nebula_density, 2, false);
                    anim_slider_f32(ui, anim, "env_nebula_contrast", "Contrast", &mut s.env.nebula_contrast, 2, false);
                    anim_slider_f32(ui, anim, "env_nebula_dust", "Dark Dust", &mut s.env.nebula_dust, 2, false);
                    anim_slider_f32(ui, anim, "env_nebula_scale", "Cloud Scale", &mut s.env.nebula_scale, 2, false);
                    anim_slider_f32(ui, anim, "env_star_density", "Star Density", &mut s.env.star_density, 2, false);
                    anim_slider_f32(ui, anim, "env_star_brightness", "Star Bright", &mut s.env.star_brightness, 2, false);
                }
                4 => {
                    anim_slider_f32(ui, anim, "env_grid_glow", "Grid Glow", &mut s.env.grid_glow, 2, false);
                    anim_slider_f32(ui, anim, "env_grid_density", "Grid Density", &mut s.env.grid_density, 1, false);
                }
                5 => {
                    anim_slider_f32(ui, anim, "env_elev_cloud_cover", "Cloud Cover", &mut s.env.elev_cloud_cover, 2, false);
                    anim_slider_f32(ui, anim, "env_elev_fog", "Valley Fog", &mut s.env.elev_fog, 2, false);
                }
                6 => {
                    anim_slider_f32(ui, anim, "env_city_fog", "City Fog", &mut s.env.city_fog, 2, false);
                    anim_slider_f32(ui, anim, "env_city_windows", "Window Glow", &mut s.env.city_windows, 2, false);
                }
                7 => {
                    anim_slider_f32(ui, anim, "env_star_density", "Star Density", &mut s.env.star_density, 2, false);
                    anim_slider_f32(ui, anim, "env_star_brightness", "Star Bright", &mut s.env.star_brightness, 2, false);
                    anim_slider_f32(ui, anim, "env_colony_glow", "Colony Glow", &mut s.env.colony_glow, 2, false);
                    anim_slider_f32(ui, anim, "env_planet_limb", "Planet Limb", &mut s.env.planet_limb, 2, false);
                }
                _ => {}
            }
        });

        collapsing_section(ui, "sec_dof", ic::APERTURE, "Lens DoF", |ui| {
            anim_slider_f32(
                ui,
                anim,
                "aperture",
                "Aperture",
                &mut s.aperture,
                3,
                false,
            );
            anim_slider_f32(
                ui,
                anim,
                "focus_dist",
                "Focus Dist",
                &mut s.focus_dist,
                2,
                false,
            );
        });

        collapsing_section(
            ui,
            "sec_floor",
            ic::SQUARES_FOUR,
            "Scratched Metal Floor",
            |ui| {
                anim_slider_f32(
                    ui,
                    anim,
                    "floor_roughness",
                    "Roughness",
                    &mut s.floor_roughness,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "floor_scratch",
                    "Scratches",
                    &mut s.floor_scratch,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "floor_scale",
                    "Tile Scale",
                    &mut s.floor_scale,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "floor_metallic",
                    "Metallic",
                    &mut s.floor_metallic,
                    2,
                    false,
                );
            },
        );

        collapsing_section(ui, "sec_scene", ic::GLOBE, "Scene", |ui| {
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
                    3,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_size_max",
                    "Max Size",
                    &mut s.sat_size_max,
                    3,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_orbit_min",
                    "Min Orbit",
                    &mut s.sat_orbit_min,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_orbit_max",
                    "Max Orbit",
                    &mut s.sat_orbit_max,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_height_min",
                    "Min Height",
                    &mut s.sat_height_min,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_height_max",
                    "Max Height",
                    &mut s.sat_height_max,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_speed_min",
                    "Min Speed",
                    &mut s.sat_speed_min,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_speed_max",
                    "Max Speed",
                    &mut s.sat_speed_max,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_bob_amp_min",
                    "Bob Amp Min",
                    &mut s.sat_bob_amp_min,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_bob_amp_max",
                    "Bob Amp Max",
                    &mut s.sat_bob_amp_max,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_bob_freq_min",
                    "Bob Frq Min",
                    &mut s.sat_bob_freq_min,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_bob_freq_max",
                    "Bob Frq Max",
                    &mut s.sat_bob_freq_max,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "sat_hue_spread",
                    "Hue Spread",
                    &mut s.sat_hue_spread,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "glass_ior",
                    "Glass IOR",
                    &mut s.glass_ior,
                    2,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "glass_roughness",
                    "Glass Rough",
                    &mut s.glass_roughness,
                    3,
                    false,
                );
                anim_slider_f32(
                    ui,
                    anim,
                    "glass_absorb",
                    "Absorb",
                    &mut s.glass_absorb,
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
            |ui| {
                prop_toggle(ui, "Bloom", &mut s.bloom_on, "Enable Diffraction Bloom");
                if s.bloom_on {
                    anim_slider_f32(
                        ui,
                        anim,
                        "bloom_threshold",
                        "Threshold",
                        &mut s.bloom_threshold,
                        2,
                        false,
                    );
                    anim_slider_f32(
                        ui,
                        anim,
                        "bloom_intensity",
                        "Intensity",
                        &mut s.bloom_intensity,
                        2,
                        false,
                    );
                    prop_slider_u32(ui, "Blades", &mut s.bloom_blades, 3..=12, false);
                }
            },
        );

        collapsing_section(ui, "sec_split", ic::COLUMNS, "A/B Split", |ui| {
            prop_toggle(ui, "Split View", &mut s.split_on, "Drag Handle in Viewport");
            if s.split_on {
                anim_slider_f32(
                    ui,
                    anim,
                    "split_x",
                    "Split X",
                    &mut s.split_x,
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

        collapsing_section(ui, "sec_pt", ic::LIGHTNING, "Path Tracer", |ui| {
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
                    let cur = (s.aov_mode as usize).min(AOV_PASSES.len() - 1);
                    let tint = (cur != 0).then(|| active_accent_tint(ui));
                    let step = cycle_toggle_btn(
                        ui,
                        w,
                        "AOV",
                        AOV_PASSES[cur].label,
                        "Inspect all 11 SDG/VFX passes: Beauty, Albedo, Normal, Depth, Motion, SemanticId, InstanceId, WorldPos, Range (LiDAR), Material (PBR), and Cryptomatte.",
                        true,
                        tint,
                    );
                    if step != 0 {
                        s.aov_mode =
                            ((cur as i32 + step).rem_euclid(AOV_PASSES.len() as i32)) as u32;
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
                    let cur = dlss_quality_index(s.dlss_quality);
                    let step = cycle_toggle_btn(
                        ui,
                        inner_w,
                        "Quality",
                        DLSS_QUALITY_MODES[cur].label,
                        "Balance between frame rate and sharpness: Ultra Perf is fastest, Quality and DLAA give the sharpest image.",
                        true,
                        None,
                    );
                    if step != 0 {
                        let next = (cur as i32 + step)
                            .rem_euclid(DLSS_QUALITY_MODES.len() as i32) as usize;
                        s.dlss_quality = DLSS_QUALITY_MODES[next].id;
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
                    let rec_tip = match (self.is_recording, use_pq10, is_full_win) {
                        (true, _, _) => {
                            "Stop recording and finalize the MP4 video file (Red = Recording)."
                        }
                        (false, true, true) => {
                            "Start recording the full window (viewport + UI) as a 10-bit HDR10 PQ10 HEVC MP4 video (Green = Ready)."
                        }
                        (false, true, false) => {
                            "Start recording the 3D viewport as a 10-bit HDR10 PQ10 HEVC MP4 video (Green = Ready)."
                        }
                        (false, false, true) => {
                            "Start recording the full window (viewport + UI) as an iPhone-compatible H.264 MP4 video into renders/ (Green = Ready)."
                        }
                        (false, false, false) => {
                            "Start recording the 3D viewport as an iPhone-compatible H.264 MP4 video into renders/ (Green = Ready)."
                        }
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
            equal_grid(ui, 3, 3, |ui, idx, w| match idx {
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
                2 => {
                    static CANCEL_BURST_LABEL: LazyLock<String> =
                        LazyLock::new(|| format!("{}  Cancel Burst", ic::STOP_CIRCLE));
                    if ui
                        .add_enabled(
                            s.csg_mode > 0,
                            egui::Button::new(CANCEL_BURST_LABEL.as_str())
                                .truncate()
                                .min_size(egui::vec2(w, ROW_H + 2.0)),
                        )
                        .on_hover_text("Stop the running SDG dataset burst.")
                        .clicked()
                    {
                        self.bus.emit_ui(UiCommand::CancelSdgBurst);
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
                                // Not routed through `SelectScene` (that pauses the rail this
                                // starts), so apply its other effects here (F25).
                                s.csg_mode = 1;
                                if let Some(d) = self.scene_descriptors.first() {
                                    let tod_u32 = match d.default_tod {
                                        TimeOfDay::Day => 0,
                                        TimeOfDay::Sunset => 1,
                                        TimeOfDay::Night => 2,
                                    };
                                    s.csg_wetness = d.default_wetness.clamp(0.0, 1.0);
                                    apply_csg_time_of_day_preset(s, tod_u32);
                                    s.csg_asset_idx = s.csg_asset_idx.min(
                                        u32::try_from(d.variant_labels.len().saturating_sub(1))
                                            .unwrap_or(0),
                                    );
                                }
                                if self.possess_info.is_some() {
                                    self.bus.emit_ui(UiCommand::EjectPossess);
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
                            let sv_txt = fmt_into(
                                &mut self.scratch.label_buf,
                                format_args!(
                                    "{}: {:.2} ({}v, ent {:.2}, depth {:.2}, {:.1} ms)",
                                    sv.score.backend.label(),
                                    sv.score.composite,
                                    sv.candidates_evaluated,
                                    sv.score.semantic_entropy,
                                    sv.score.depth_stratification,
                                    sv.elapsed_ms
                                ),
                            );
                            ui.label(RichText::new(sv_txt).size(10.5).weak());
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
                        TIME_SCALE_MIN..=TIME_SCALE_MAX,
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
                    // 5. Flight stats of the running rail (accumulated while Auto-Drive flies).
                    ui.add_space(4.0);
                    egui::CollapsingHeader::new(
                        RichText::new(format!("{}  Flight stats", ic::CHART_LINE)).strong(),
                    )
                    .id_salt("flight_stats")
                    .default_open(false)
                    .show(ui, |ui| {
                        if let Some(t) = flight_stats_ui(
                            ui,
                            &mut self.scratch.label_buf,
                            self.flight,
                            rail,
                            s.csg_lod_bias,
                            &self.csg_stats,
                            self.crowd_actor_count,
                            self.rt_ms,
                            self.bus,
                        ) {
                            rail.time_s = t;
                            if let Some(p) =
                                rail.evaluate_at(t, cur_pose.znear, cur_pose.zfar.max(2000.0))
                            {
                                snap_pose = Some(p);
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

        collapsing_section(ui, "sec_telemetry", ic::GAUGE, "Telemetry", |ui| {
            let buf = &mut self.scratch.label_buf;
            stat_row(ui, "GPU", &self.telemetry.gpu_name);
            stat_row(
                ui,
                "Resolution",
                fmt_into(buf, format_args!("{} x {}", self.render_w, self.render_h)),
            );
            stat_row(ui, "UI FPS", fmt_into(buf, format_args!("{:.0} FPS", self.ui_fps)));
            stat_row(
                ui,
                "RT FPS",
                fmt_into(
                    buf,
                    format_args!(
                        "{:.1} FPS ({:.1} ms{})",
                        self.rt_fps,
                        self.rt_ms,
                        if self.fast_preview { " fast" } else { "" }
                    ),
                ),
            );
            stat_row(
                ui,
                "Bloom / TM",
                fmt_into(buf, format_args!("{:.2} / {:.2} ms", self.bloom_ms, self.tonemap_ms)),
            );
            stat_row(
                ui,
                "Accum",
                fmt_into(
                    buf,
                    format_args!(
                        "{} / {} ({} spp)",
                        self.accum_count, self.settings.max_accum, self.total_samples
                    ),
                ),
            );
            stat_row(ui, "Denoiser", fmt_into(buf, format_args!("{:.1} ms", self.denoise_ms)));
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
                fmt_into(
                    buf,
                    format_args!(
                        "{} / {} MiB",
                        self.telemetry.vram_used_mib, self.telemetry.vram_total_mib
                    ),
                ),
            );
            stat_row(
                ui,
                "Clocks",
                fmt_into(
                    buf,
                    format_args!(
                        "{} / {} MHz",
                        self.telemetry.gpu_clock_mhz, self.telemetry.mem_clock_mhz
                    ),
                ),
            );
            stat_row(
                ui,
                "Temp / Load",
                fmt_into(
                    buf,
                    format_args!("{} C / {} %", self.telemetry.temp_c, self.telemetry.util_gpu),
                ),
            );
            stat_row(
                ui,
                "Power",
                fmt_into(buf, format_args!("{:.0} W", self.telemetry.power_w)),
            );
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
                    fmt_into(
                        buf,
                        format_args!(
                            "{} / {}",
                            self.csg_stats.visible_instances, self.csg_stats.total_instances
                        ),
                    ),
                );
                stat_row(
                    ui,
                    "CSG LOD",
                    fmt_into(
                        buf,
                        format_args!(
                            "L0:{} L1:{} L2:{} C:{}",
                            self.csg_stats.lod0_count,
                            self.csg_stats.lod1_count,
                            self.csg_stats.lod2_count,
                            self.csg_stats.culled_count
                        ),
                    ),
                );
                stat_row(
                    ui,
                    "CSG DAG",
                    fmt_into(
                        buf,
                        format_args!(
                            "{} GAS / {} prm",
                            self.csg_stats.compounds, self.csg_stats.primitives
                        ),
                    ),
                );
                stat_row(
                    ui,
                    "CSG Aux",
                    fmt_into(
                        buf,
                        format_args!("{:.1} KiB", self.csg_stats.aux_planes_bytes as f64 / 1024.0),
                    ),
                );
                stat_row(
                    ui,
                    "CSG VRAM",
                    fmt_into(
                        buf,
                        format_args!("{:.1} KiB", self.csg_stats.gpu_bytes as f64 / 1024.0),
                    ),
                );
                if self.crowd_actor_count > 0 {
                    stat_row(
                        ui,
                        "Crowd Sim",
                        fmt_into(
                            buf,
                            format_args!(
                                "{} {:.2} ms ({})",
                                self.crowd_active_backend.label(),
                                self.crowd_step_ms,
                                self.crowd_actor_count
                            ),
                        ),
                    );
                }
            }
        });

        collapsing_section(ui, "sec_perf", ic::CHART_LINE, "Performance", |ui| {
            let buf = &mut self.scratch.label_buf;
            stat_row(
                ui,
                "RT Cores",
                fmt_into(buf, format_args!("{:.2} ms (OptiX)", self.rt_ms)),
            );
            stat_row(
                ui,
                "Tensor Cores",
                fmt_into(buf, format_args!("{:.2} ms (Den/DLSS)", self.tensor_cores_ms)),
            );
            stat_row(
                ui,
                "CUDA Compute",
                fmt_into(buf, format_args!("{:.2} ms (IAS/FFT/TM)", self.cuda_compute_ms)),
            );
            ui.add_space(2.0);
            sparkline_row(
                ui,
                buf,
                "RT Cores",
                &self.timings.rt_cores_ms,
                Color32::from_rgb(138, 173, 244),
                "ms",
            );
            sparkline_row(
                ui,
                buf,
                "Tensor",
                &self.timings.tensor_cores_ms,
                Color32::from_rgb(166, 209, 137),
                "ms",
            );
            sparkline_row(
                ui,
                buf,
                "CUDA SM",
                &self.timings.cuda_compute_ms,
                Color32::from_rgb(238, 212, 159),
                "ms",
            );
            sparkline_row(
                ui,
                buf,
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
            s.sky_mode = SKY_DAYLIGHT;
            s.sun_azimuth = -52.0;
            s.sun_elevation = 46.0;
            s.sun_intensity = 2.20;
            s.sky_intensity = 0.95;
            s.exposure = 0.90;
        }
        2 => {
            s.sky_mode = SKY_COASTAL_NIGHT;
            s.sun_azimuth = -110.0;
            s.sun_elevation = 24.0;
            s.sun_intensity = 0.16;
            s.sky_intensity = 0.65;
            s.exposure = 1.05;
        }
        _ => {
            s.sky_mode = SKY_SUNSET;
            s.sun_azimuth = -98.0;
            s.sun_elevation = 11.0;
            s.sun_intensity = 1.75;
            s.sky_intensity = 0.85;
            s.exposure = 0.95;
        }
    }
}

/// Restores all CSG distance, screen-space, and crowd LOD/culling settings to
/// [`DemoSettings::default`] without modifying unrelated render or scene fields.
pub fn reset_csg_lod_defaults(s: &mut DemoSettings) {
    let d = DemoSettings::default();
    s.csg_lod_enabled = d.csg_lod_enabled;
    s.csg_force_lod = d.csg_force_lod;
    s.csg_lod_bias = d.csg_lod_bias;
    s.csg_lod1_dist_m = d.csg_lod1_dist_m;
    s.csg_lod2_dist_m = d.csg_lod2_dist_m;
    s.csg_meso_dist_m = d.csg_meso_dist_m;
    s.csg_micro_dist_m = d.csg_micro_dist_m;
    s.csg_min_screen_px = d.csg_min_screen_px;
    s.csg_lod_hysteresis = d.csg_lod_hysteresis;
    s.csg_crowd_lod1_m = d.csg_crowd_lod1_m;
    s.csg_crowd_lod2_m = d.csg_crowd_lod2_m;
    s.csg_crowd_cull_m = d.csg_crowd_cull_m;
}

/// Keeps ordered LOD and culling distance thresholds monotonic (`lod1 <= lod2`,
/// `micro <= meso`, `crowd_lod1 <= crowd_lod2 <= crowd_cull`) based on which slider moved.
fn sync_lod_distance_order(s: &mut DemoSettings, prev: &DemoSettings) {
    if (s.csg_lod1_dist_m - prev.csg_lod1_dist_m).abs() > 1e-4 {
        s.csg_lod2_dist_m = s.csg_lod2_dist_m.max(s.csg_lod1_dist_m).clamp(30.0, 1200.0);
    } else if (s.csg_lod2_dist_m - prev.csg_lod2_dist_m).abs() > 1e-4 {
        s.csg_lod1_dist_m = s.csg_lod1_dist_m.min(s.csg_lod2_dist_m).clamp(10.0, 500.0);
    }

    if (s.csg_micro_dist_m - prev.csg_micro_dist_m).abs() > 1e-4 {
        s.csg_meso_dist_m = s.csg_meso_dist_m.max(s.csg_micro_dist_m).clamp(30.0, 1200.0);
    } else if (s.csg_meso_dist_m - prev.csg_meso_dist_m).abs() > 1e-4 {
        s.csg_micro_dist_m = s.csg_micro_dist_m.min(s.csg_meso_dist_m).clamp(5.0, 200.0);
    }

    if (s.csg_crowd_lod1_m - prev.csg_crowd_lod1_m).abs() > 1e-4 {
        s.csg_crowd_lod2_m = s.csg_crowd_lod2_m.max(s.csg_crowd_lod1_m).clamp(20.0, 300.0);
        s.csg_crowd_cull_m = s.csg_crowd_cull_m.max(s.csg_crowd_lod2_m).clamp(40.0, 600.0);
    } else if (s.csg_crowd_lod2_m - prev.csg_crowd_lod2_m).abs() > 1e-4 {
        s.csg_crowd_lod1_m = s.csg_crowd_lod1_m.min(s.csg_crowd_lod2_m).clamp(5.0, 120.0);
        s.csg_crowd_cull_m = s.csg_crowd_cull_m.max(s.csg_crowd_lod2_m).clamp(40.0, 600.0);
    } else if (s.csg_crowd_cull_m - prev.csg_crowd_cull_m).abs() > 1e-4 {
        s.csg_crowd_lod2_m = s.csg_crowd_lod2_m.min(s.csg_crowd_cull_m).clamp(20.0, 300.0);
        s.csg_crowd_lod1_m = s.csg_crowd_lod1_m.min(s.csg_crowd_lod2_m).clamp(5.0, 120.0);
    }
}

/// Computes `(inner_w, cell_w)` for the 4-column LOD telemetry strip so columns never overflow
/// narrow dock panes (`total_w >= 120.0`).
fn lod_strip_widths(total_w: f32) -> (f32, f32) {
    let safe_w = if total_w.is_finite() {
        total_w.max(120.0)
    } else {
        220.0
    };
    let inner_w = (safe_w - 8.0).max(112.0);
    let gap = 4.0;
    let cell_w = ((inner_w - gap * 3.0) / 4.0).max(24.0);
    (inner_w, cell_w)
}

/// Renders a compact, zero-churn 4-column tabular telemetry strip (`LOD0 / LOD1 / LOD2 / Culled`)
/// using reusable scratch buffer `buf`.
fn lod_telemetry_strip(ui: &mut Ui, buf: &mut String, stats: CsgStats, active: bool) {
    let (inner_w, cell_w) = lod_strip_widths(ui.available_width());
    let dark = ui.visuals().dark_mode;
    let colors = if !active {
        let muted = ui.visuals().weak_text_color();
        [muted, muted, muted, muted]
    } else if dark {
        [
            Color32::from_rgb(142, 214, 158),
            Color32::from_rgb(138, 178, 244),
            Color32::from_rgb(236, 198, 126),
            Color32::from_rgb(168, 174, 186),
        ]
    } else {
        [
            Color32::from_rgb(30, 122, 62),
            Color32::from_rgb(38, 96, 192),
            Color32::from_rgb(164, 108, 24),
            Color32::from_rgb(96, 102, 114),
        ]
    };
    let items: [(&str, u32, Color32); 4] = [
        ("LOD0", stats.lod0_count, colors[0]),
        ("LOD1", stats.lod1_count, colors[1]),
        ("LOD2", stats.lod2_count, colors[2]),
        ("Culled", stats.culled_count, colors[3]),
    ];

    egui::Frame::NONE
        .fill(ui.visuals().extreme_bg_color)
        .corner_radius(3.0)
        .inner_margin(egui::Margin::symmetric(4, 3))
        .show(ui, |ui| {
            ui.set_width(inner_w);
            let gap = 4.0;
            let resp = ui
                .horizontal(|ui| {
                    ui.set_max_width(inner_w);
                    ui.spacing_mut().item_spacing.x = gap;
                    for (lbl, count, col) in items {
                        ui.allocate_ui_with_layout(
                            egui::vec2(cell_w, 26.0),
                            Layout::top_down(Align::Center),
                            |ui| {
                                ui.set_max_width(cell_w);
                                ui.spacing_mut().item_spacing.y = 0.0;
                                ui.add(
                                    egui::Label::new(
                                        RichText::new(lbl)
                                            .size(9.5)
                                            .color(ui.visuals().weak_text_color()),
                                    )
                                    .truncate(),
                                );
                                let val_str = if active {
                                    fmt_into(buf, format_args!("{count}"))
                                } else {
                                    "-"
                                };
                                ui.add(
                                    egui::Label::new(
                                        RichText::new(val_str).monospace().size(11.0).color(col),
                                    )
                                    .truncate(),
                                );
                            },
                        );
                    }
                })
                .response;
            let hover_resp = ui.interact(
                resp.rect,
                ui.id().with("lod_strip_hover"),
                Sense::hover(),
            );
            if hover_resp.hovered() {
                let tip = if active {
                    fmt_into(
                        buf,
                        format_args!(
                            "LOD0: {}  |  LOD1: {}  |  LOD2: {}  |  Culled: {}",
                            stats.lod0_count,
                            stats.lod1_count,
                            stats.lod2_count,
                            stats.culled_count
                        ),
                    )
                } else {
                    "Switch to a CSG World scene to inspect live LOD & culling counts."
                };
                hover_resp.on_hover_text(tip);
            }
        });
}

fn section_accent(id_salt: &str) -> Color32 {
    match id_salt {
        "sec_csg" => Color32::from_rgb(92, 184, 224),
        "sec_lod" => Color32::from_rgb(104, 192, 186),
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
        "sec_lod",
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
            | "sec_lod"
            | "sec_sky"
            | "sec_pt"
    )
}

fn collapsing_section(
    ui: &mut Ui,
    id_salt: &str,
    icon: &str,
    title: &str,
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
        let resp = ui.add_sized([width, ROW_H + 2.0], btn).on_hover_ui(|ui| {
            ui.label(tooltip);
            ui.label(RichText::new("LMB: next | RMB: prev").small().weak());
        });
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

/// Row for a key that is missing from `demo_anim::ANIM_PARAMS` (a caller typo; a debug assertion
/// and `anim_slider_keys_are_all_animatable` catch it). Shows the problem instead of a slider
/// with an invented range.
fn unknown_anim_key_row(ui: &mut Ui, key: &str, label: &str) {
    debug_assert!(false, "`{key}` is not in demo_anim::ANIM_PARAMS");
    prop_row(ui, label, |ui| {
        ui.colored_label(ui.visuals().error_fg_color, format!("unknown parameter `{key}`"));
    });
}

/// Slider row for the animatable `key`; its range is `demo_anim::anim_range(key)`, the same range
/// the animator clamps the oscillation to.
fn anim_slider_f32(
    ui: &mut Ui,
    anim: &mut DemoAnimator,
    key: &str,
    label: &str,
    val: &mut f32,
    decimals: usize,
    log: bool,
) {
    let Some(range) = anim_range(key) else {
        unknown_anim_key_row(ui, key, label);
        return;
    };
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
        let span = (*range.end() - *range.start()).abs();
        let drag_speed = if span.is_finite() && span > 0.0 {
            (span / 250.0).clamp(0.005, 2.0)
        } else {
            0.01
        };
        let slider = egui::Slider::new(val, range.clone()).show_value(false);
        ui.add_sized([track_w, ROW_H], slider);
        let prev_style = ui.style().override_text_style.clone();
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
        let mut drag = egui::DragValue::new(val)
            .range(range)
            .fixed_decimals(decimals)
            .speed(drag_speed);
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
    speed: f32,
) {
    let Some(range) = anim_range(key) else {
        unknown_anim_key_row(ui, key, label);
        return;
    };
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

/// Height of one Flight stats graph.
const FLIGHT_GRAPH_H: f32 = 64.0;

/// Flight stats subsection of the rail panel (`crate::flight_stats`). Returns a rail time to jump
/// to when a slow spot was clicked.
#[expect(
    clippy::too_many_arguments,
    reason = "one immediate-mode panel reading disjoint UiContext fields next to the borrowed rail"
)]
fn flight_stats_ui(
    ui: &mut Ui,
    buf: &mut String,
    flight: &mut crate::flight_stats::FlightStats,
    rail: &CameraRail,
    lod_bias: f32,
    csg: &CsgStats,
    crowd_actors: u32,
    rt_ms: f32,
    bus: &AppBus,
) -> Option<f32> {
    let mut jump = None;
    let dur = rail.duration_s.max(0.5);
    let label = rail.active_label();
    stat_row(ui, "Cassette", &label);
    stat_row(
        ui,
        "Rail Time",
        fmt_into(
            buf,
            format_args!(
                "{:.1} / {:.1} s ({:.0} %), loop {}",
                rail.time_s,
                dur,
                100.0 * rail.time_s / dur,
                flight.loops + 1
            ),
        ),
    );
    stat_row(
        ui,
        "Camera",
        fmt_into(
            buf,
            format_args!(
                "{:.1} m/s  {:.0} deg/s",
                flight.speed_mps, flight.turn_dps
            ),
        ),
    );
    if let Some(last) = flight.latest() {
        stat_row(
            ui,
            "Altitude",
            fmt_into(buf, format_args!("y {:.1} m (world)", last.state.eye[1])),
        );
        stat_row(
            ui,
            "Launch",
            fmt_into(
                buf,
                format_args!(
                    "{}x{}  {} spp  {} bounces",
                    last.state.width, last.state.height, last.state.spp, last.state.bounces
                ),
            ),
        );
    }
    stat_row(ui, "LOD Bias", fmt_into(buf, format_args!("{lod_bias:.2}")));
    let load = flight.load;
    stat_row(
        ui,
        "Visible",
        fmt_into(
            buf,
            format_args!(
                "{} inst, {} GAS, {} crowd actors",
                load.visible, csg.compounds, crowd_actors
            ),
        ),
    );
    stat_row(
        ui,
        "Paths",
        fmt_into(
            buf,
            format_args!(
                "prim {}  union {}  exact union {}  RPN {}",
                load.paths[0], load.paths[1], load.paths[2], load.paths[3]
            ),
        ),
    );
    stat_row(
        ui,
        "LOD 0/1/2",
        fmt_into(
            buf,
            format_args!(
                "{} / {} / {}, culled {}",
                load.lod[0], load.lod[1], load.lod[2], load.culled
            ),
        ),
    );
    stat_row(ui, "RT", fmt_into(buf, format_args!("{rt_ms:.1} ms")));
    let summary = flight.summary();
    if let Some(sm) = summary {
        stat_row(
            ui,
            "RT min/med",
            fmt_into(buf, format_args!("{:.1} / {:.1} ms", sm.min, sm.median)),
        );
        stat_row(
            ui,
            "RT p95/max",
            fmt_into(
                buf,
                format_args!("{:.1} / {:.1} ms ({} frames)", sm.p95, sm.max, sm.frames),
            ),
        );
    } else {
        ui.label(RichText::new("Start Auto-Drive to collect flight stats.").weak());
    }

    let w = ui.available_width().max(60.0);
    let dark = ui.visuals().dark_mode;
    let cur_col = Color32::from_rgb(118, 185, 0);
    let prev_col = if dark {
        Color32::from_gray(110)
    } else {
        Color32::from_gray(150)
    };
    let load_col = Color32::from_rgb(90, 160, 230);
    let lod_col = Color32::from_rgb(230, 170, 60);

    // Graph 1: RT ms vs rail time, previous loop under the current one.
    ui.label(RichText::new("RT ms vs rail time (grey: previous loop)").size(10.5).weak());
    let rt_max = flight
        .cur_loop()
        .iter()
        .chain(flight.prev_loop())
        .map(|p| p.rt_ms)
        .fold(1.0_f32, f32::max);
    let (rect, _) = ui.allocate_exact_size(egui::vec2(w, FLIGHT_GRAPH_H), Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 2.0, ui.visuals().extreme_bg_color);
    paint_flight_curve(&painter, rect, flight.prev_loop(), dur, rt_max, |p| p.rt_ms, prev_col);
    paint_flight_curve(&painter, rect, flight.cur_loop(), dur, rt_max, |p| p.rt_ms, cur_col);
    painter.text(
        rect.left_top() + egui::vec2(3.0, 2.0),
        egui::Align2::LEFT_TOP,
        fmt_into(buf, format_args!("{rt_max:.0} ms")),
        egui::FontId::proportional(10.0),
        ui.visuals().weak_text_color(),
    );

    // Graph 2: scene load vs rail time.
    ui.label(
        RichText::new("Visible instances (blue) and LOD 0 share (orange) vs rail time")
            .size(10.5)
            .weak(),
    );
    let vis_max = flight
        .cur_loop()
        .iter()
        .map(|p| p.visible as f32)
        .fold(1.0_f32, f32::max);
    let (rect, _) = ui.allocate_exact_size(egui::vec2(w, FLIGHT_GRAPH_H), Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 2.0, ui.visuals().extreme_bg_color);
    paint_flight_curve(&painter, rect, flight.cur_loop(), dur, vis_max, |p| p.visible as f32, load_col);
    paint_flight_curve(&painter, rect, flight.cur_loop(), dur, 1.0, |p| p.lod0_share, lod_col);
    painter.text(
        rect.left_top() + egui::vec2(3.0, 2.0),
        egui::Align2::LEFT_TOP,
        fmt_into(buf, format_args!("{vis_max:.0} inst")),
        egui::FontId::proportional(10.0),
        ui.visuals().weak_text_color(),
    );

    // RT histogram over the accumulated frames.
    if let Some(sm) = summary {
        ui.label(
            RichText::new(fmt_into(
                buf,
                format_args!("RT histogram {:.0}..{:.0} ms", sm.min, sm.max),
            ))
            .size(10.5)
            .weak(),
        );
        let mut bins = [0u32; crate::flight_stats::HIST_BINS];
        flight.histogram(sm.min, sm.max, &mut bins);
        let peak = bins.iter().copied().max().unwrap_or(1).max(1) as f32;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(w, FLIGHT_GRAPH_H * 0.6), Sense::hover());
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 2.0, ui.visuals().extreme_bg_color);
        let bw = rect.width() / bins.len() as f32;
        for (i, &c) in bins.iter().enumerate() {
            let h = (rect.height() - 2.0) * c as f32 / peak;
            let x0 = rect.left() + i as f32 * bw;
            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(x0 + 0.5, rect.bottom() - h),
                    egui::pos2(x0 + bw - 0.5, rect.bottom()),
                ),
                0.0,
                cur_col,
            );
        }
    }

    // Slowest rail positions (click: jump the rail there).
    let mut any_slow = false;
    for spot in flight.slow_spots() {
        if !any_slow {
            ui.label(RichText::new("Slowest positions (click to jump)").size(10.5).weak());
            any_slow = true;
        }
        let txt = fmt_into(
            buf,
            format_args!(
                "{:.1} ms at {:.2} s  ({:.0}, {:.0}, {:.0})",
                spot.rt_ms, spot.rail_t, spot.eye[0], spot.eye[1], spot.eye[2]
            ),
        );
        if ui
            .add_sized([w, ROW_H], egui::Button::new(RichText::new(txt).size(11.0)))
            .clicked()
        {
            jump = Some(spot.rail_t);
        }
    }

    equal_grid(ui, 2, 2, |ui, idx, bw| match idx {
        0 => {
            if ui
                .add_sized(
                    [bw, ROW_H + 2.0],
                    egui::Button::new(format!("{}  Reset", ic::ARROW_COUNTER_CLOCKWISE)),
                )
                .on_hover_text("Clear the accumulated flight stats (also done on a new cassette or Auto-Drive start).")
                .clicked()
            {
                flight.reset();
            }
        }
        _ => {
            if ui
                .add_enabled(
                    flight.frames() > 0,
                    egui::Button::new(format!("{}  Export CSV", ic::FLOPPY_DISK))
                        .min_size(egui::vec2(bw, ROW_H + 2.0)),
                )
                .on_hover_text("Write the accumulated frames as a perf CSV (same columns as --bench-rail).")
                .clicked()
            {
                bus.emit_ui(UiCommand::ExportFlightCsv);
            }
        }
    });
    jump
}

/// Draws `pts` (x = rail time over `[0, x_max]`, y = `y(p)` over `[0, y_max]`) into `rect` as line
/// segments, at most one segment per horizontal pixel; allocates nothing.
fn paint_flight_curve(
    painter: &egui::Painter,
    rect: egui::Rect,
    pts: &[crate::flight_stats::LoopPoint],
    x_max: f32,
    y_max: f32,
    y: impl Fn(&crate::flight_stats::LoopPoint) -> f32,
    color: Color32,
) {
    if pts.len() < 2 || x_max <= 0.0 || y_max <= 0.0 {
        return;
    }
    let stride = (pts.len() as f32 / rect.width().max(1.0)).ceil().max(1.0) as usize;
    let to_screen = |p: &crate::flight_stats::LoopPoint| {
        egui::pos2(
            rect.left() + rect.width() * (p.rail_t / x_max).clamp(0.0, 1.0),
            rect.bottom() - 1.0 - (rect.height() - 2.0) * (y(p) / y_max).clamp(0.0, 1.0),
        )
    };
    let stroke = egui::Stroke::new(1.25, color);
    let mut prev = to_screen(&pts[0]);
    for p in pts.iter().step_by(stride).skip(1) {
        let cur = to_screen(p);
        // A backwards jump in rail time (scrub) starts a new segment instead of a long chord.
        if cur.x >= prev.x {
            painter.line_segment([prev, cur], stroke);
        }
        prev = cur;
    }
}

fn sparkline_row(
    ui: &mut Ui,
    label_buf: &mut String,
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
            fmt_into(label_buf, format_args!("{:.0}", ring.latest()))
        } else {
            fmt_into(label_buf, format_args!("{:.1} {unit}", ring.latest()))
        };
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add(egui::Label::new(RichText::new(txt).size(11.5).color(color)).truncate());
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every `anim_slider_f32` / `anim_vec3` row takes its range from `demo_anim::anim_range`, so a
    /// key missing from `ANIM_PARAMS` would render an error row: scan this file's call sites (the
    /// key is the first string literal of each call) and require every key to be animatable.
    #[test]
    fn anim_slider_keys_are_all_animatable() {
        let src = include_str!("ui.rs");
        let mut calls = 0;
        // Split literals, so the needles do not match their own spelling in this test.
        for name in [concat!("anim_slider", "_f32("), concat!("anim_", "vec3(")] {
            for (at, _) in src.match_indices(name) {
                if src[..at].ends_with("fn ") {
                    continue;
                }
                let rest = &src[at + name.len()..];
                let Some(open) = rest.find('"') else {
                    panic!("call at byte {at} has no key literal");
                };
                let Some(len) = rest[open + 1..].find('"') else {
                    panic!("unterminated key literal at byte {at}");
                };
                let key = &rest[open + 1..open + 1 + len];
                assert!(anim_range(key).is_some(), "`{key}` is not in ANIM_PARAMS");
                calls += 1;
            }
        }
        assert!(calls > 80, "found only {calls} call sites");
    }

    #[test]
    fn time_of_day_presets_select_the_named_sky_entries() {
        assert_eq!(SKY_MODES[SKY_DAYLIGHT as usize], "Daylight");
        assert_eq!(SKY_MODES[SKY_SUNSET as usize], "Sunset");
        assert_eq!(SKY_MODES[SKY_COASTAL_NIGHT as usize], "Coastal Night");
        let mut s = DemoSettings::default();
        for (tod, sky) in [(0, SKY_DAYLIGHT), (1, SKY_SUNSET), (2, SKY_COASTAL_NIGHT)] {
            apply_csg_time_of_day_preset(&mut s, tod);
            assert_eq!(s.sky_mode, sky);
        }
    }

    #[test]
    fn geo_origin_comes_from_the_selected_scene_descriptor_only() {
        let reg = csg_scenes::default_scene_registry();
        let descs: Vec<SceneDescriptor> = (0..reg.len())
            .map(|i| reg.get_by_index(i).expect("registered scene").descriptor())
            .collect();
        assert_eq!(active_geo_origin(&descs, 0), None, "classic stage");
        assert_eq!(
            active_geo_origin(&descs, descs.len() as u32 + 1),
            None,
            "out of range"
        );
        for (i, d) in descs.iter().enumerate() {
            assert_eq!(active_geo_origin(&descs, i as u32 + 1), d.geo_origin, "{}", d.id);
        }
        let by_id = |id: &str| {
            let i = descs.iter().position(|d| d.id == id).expect("scene id");
            active_geo_origin(&descs, i as u32 + 1)
        };
        for id in ["gundo_osm", "marina", "sf_house_of_ai"] {
            assert!(by_id(id).is_some(), "{id} is a real-world scene");
        }
        for id in ["gundo", "inspector"] {
            assert_eq!(by_id(id), None, "{id} is synthetic: no invented coordinates");
        }
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

    #[test]
    fn orbit_mode_dropdown_updates_prefs_mode_and_survives_driver_update() {
        let mut driver = CameraDriver::default();
        assert_eq!(driver.prefs().mode, NavMode::Turntable);

        apply_camera_toolbar_prefs(
            &mut driver,
            NavMode::Trackball,
            NavMode::FpsFly,
            NavFeel::Sharp,
            NavScheme::Default,
            false,
        );
        assert_eq!(driver.prefs().mode, NavMode::Trackball);
        assert_eq!(driver.controller().mode(), Some(NavMode::Trackball));

        // Next frame's `CameraDriver::update` calls `sync_prefs()` and must NOT revert Trackball!
        let raw = camera::input::RawInput {
            dt: 1.0 / 60.0,
            viewport: glam::Vec2::new(1280.0, 720.0),
            ..Default::default()
        };
        let df = driver.update(&raw, 2.0);
        assert!(df.pose.eye.is_finite());
        assert_eq!(driver.prefs().mode, NavMode::Trackball);
        assert_eq!(driver.controller().mode(), Some(NavMode::Trackball));
    }

    #[test]
    fn altitude_speed_multiplier_scales_smoothly_from_ground_to_aerial() {
        assert_eq!(altitude_speed_multiplier(0.0), 1.0);
        assert_eq!(altitude_speed_multiplier(2.0), 1.0);
        assert_eq!(altitude_speed_multiplier(8.0), 1.0);
        assert!((altitude_speed_multiplier(16.0) - 2.0).abs() < 1e-5);
        assert!((altitude_speed_multiplier(80.0) - 10.0).abs() < 1e-5);
        assert_eq!(altitude_speed_multiplier(200.0), 25.0);
        assert_eq!(altitude_speed_multiplier(500.0), 25.0);
        assert_eq!(altitude_speed_multiplier(f32::NAN), 1.0);
    }

    #[test]
    fn reset_csg_lod_defaults_restores_all_lod_fields_and_preserves_other_settings() {
        let def = DemoSettings::default();
        let mut s = DemoSettings {
            csg_mode: 3,
            csg_wetness: 0.72,
            exposure: 1.65,
            csg_lod_enabled: false,
            csg_force_lod: 3,
            csg_lod_bias: 2.75,
            csg_lod1_dist_m: 210.0,
            csg_lod2_dist_m: 640.0,
            csg_meso_dist_m: 720.0,
            csg_micro_dist_m: 95.0,
            csg_min_screen_px: 8.5,
            csg_lod_hysteresis: 0.35,
            csg_crowd_lod1_m: 60.0,
            csg_crowd_lod2_m: 150.0,
            csg_crowd_cull_m: 410.0,
            ..def
        };

        reset_csg_lod_defaults(&mut s);

        assert_eq!(s.csg_lod_enabled, def.csg_lod_enabled);
        assert_eq!(s.csg_force_lod, def.csg_force_lod);
        assert_eq!(s.csg_lod_bias, def.csg_lod_bias);
        assert_eq!(s.csg_lod1_dist_m, def.csg_lod1_dist_m);
        assert_eq!(s.csg_lod2_dist_m, def.csg_lod2_dist_m);
        assert_eq!(s.csg_meso_dist_m, def.csg_meso_dist_m);
        assert_eq!(s.csg_micro_dist_m, def.csg_micro_dist_m);
        assert_eq!(s.csg_min_screen_px, def.csg_min_screen_px);
        assert_eq!(s.csg_lod_hysteresis, def.csg_lod_hysteresis);
        assert_eq!(s.csg_crowd_lod1_m, def.csg_crowd_lod1_m);
        assert_eq!(s.csg_crowd_lod2_m, def.csg_crowd_lod2_m);
        assert_eq!(s.csg_crowd_cull_m, def.csg_crowd_cull_m);

        // Unrelated settings stay untouched.
        assert_eq!(s.csg_mode, 3);
        assert!((s.csg_wetness - 0.72).abs() < 1e-6);
        assert!((s.exposure - 1.65).abs() < 1e-6);
    }

    #[test]
    fn sync_lod_distance_order_keeps_tiers_monotonic_in_both_directions() {
        let mut s = DemoSettings::default();

        // Pushing LOD1 above LOD2 lifts LOD2.
        let prev = s;
        s.csg_lod1_dist_m = 340.0;
        sync_lod_distance_order(&mut s, &prev);
        assert!(s.csg_lod2_dist_m >= s.csg_lod1_dist_m);

        // Pulling LOD2 below LOD1 lowers LOD1.
        let prev = s;
        s.csg_lod2_dist_m = 60.0;
        sync_lod_distance_order(&mut s, &prev);
        assert!(s.csg_lod1_dist_m <= s.csg_lod2_dist_m);

        // Pushing Micro Cull above Meso Cull lifts Meso Cull.
        s.csg_meso_dist_m = 100.0;
        s.csg_micro_dist_m = 100.0;
        let prev_m = s;
        s.csg_micro_dist_m = 180.0;
        sync_lod_distance_order(&mut s, &prev_m);
        assert!(s.csg_meso_dist_m >= s.csg_micro_dist_m);

        // Pulling Meso Cull below Micro Cull lowers Micro Cull.
        let prev_m2 = s;
        s.csg_meso_dist_m = 60.0;
        sync_lod_distance_order(&mut s, &prev_m2);
        assert!(s.csg_micro_dist_m <= s.csg_meso_dist_m);

        // Pulling Crowd Cull down pulls Crowd LOD2 and Crowd LOD1 down cleanly.
        let prev = s;
        s.csg_crowd_lod1_m = 110.0;
        sync_lod_distance_order(&mut s, &prev);
        assert!(s.csg_crowd_lod1_m <= s.csg_crowd_lod2_m);
        assert!(s.csg_crowd_lod2_m <= s.csg_crowd_cull_m);

        let prev = s;
        s.csg_crowd_cull_m = 45.0;
        sync_lod_distance_order(&mut s, &prev);
        assert!(s.csg_crowd_lod2_m <= s.csg_crowd_cull_m);
        assert!(s.csg_crowd_lod1_m <= s.csg_crowd_lod2_m);
    }

    #[test]
    fn lod_strip_widths_never_overflow_narrow_or_wide_panels() {
        for total_w in [120.0_f32, 140.0, 180.0, 220.0, 320.0, 480.0] {
            let (inner_w, cell_w) = lod_strip_widths(total_w);
            assert!(inner_w + 8.0 <= total_w + 1e-3);
            let used = 4.0 * cell_w + 3.0 * 4.0;
            assert!(
                used <= inner_w + 1e-3,
                "LOD strip overflow at total_w={total_w}: used={used} > inner_w={inner_w}"
            );
            assert!(cell_w >= 24.0);
        }
        let (nan_inner, nan_cell) = lod_strip_widths(f32::NAN);
        assert!(nan_inner.is_finite() && nan_cell.is_finite() && nan_cell >= 24.0);
    }
}
