//! Docked studio `egui` UI with Phosphor icons, width-clamped property grids, fractal presets,
//! viewport HUD overlays, and theme SSOT.
//!
//! What: renders the top toolbar, `egui_dock` panels (`Viewport`, `Fractal`, `Render & Floor`,
//! `Camera & Stats`, `Flight Lab`, `Flight Player`), centered [`SceneBuildProgress`] compile/build
//! HUD overlay, clickable Apple Maps WGS84 coordinate pill, and shared property-row primitives
//! (`dock_scroll`, `collapsing_section`, `equal_grid`, `prop_row`, `allocate_clamped_ui`,
//! `prop_slider_f32`, `anim_slider_f32`, `stat_row`).
//! Why: provides one Single Source of Truth for control alignment (`LABEL_W`, `VAL_W = 68.0`,
//! `ANIM_BTN_W`, `allocate_clamped_ui`), hover-jitter-free button geometry across all five
//! [`UiTheme`] palettes (`bg_stroke.width = 1.0`, `expansion = 0.0`, `corner_radius = 2`), and
//! live non-blocking status feedback during heavy CSG builds or OptiX compiles.
//! How: draws the 60 Hz UI pass over borrowed [`UiContext`] state, emits typed [`UiCommand`]s
//! into [`AppBus`], delegates fractal preset persistence/bar UI to [`crate::fractal_presets`]
//! (`sl-controls::preset`) and Flight Lab/Player tabs to [`crate::flight::ui`], and formats HUD
//! lines into reusable [`UiScratch`] buffers.
//! Uses: `egui`, `egui_dock`, `egui_phosphor`, `camera`, `csg_scene`, `crate::app_events`,
//! `crate::demo_anim`, `crate::fractal_presets`, `crate::flight`, `crate::overlay`,
//! `crate::profiler`, `crate::renderer`, `crate::viewport_toolbar`, `crate::wireframe`.
//! Used by: `crate::main` (`DemoApp::render`), `crate::flight::ui`, `crate::fractal_presets`.

use std::fmt::Write as _;
use std::ops::RangeInclusive;
use std::sync::LazyLock;

use camera::input::NavScheme;
use camera::{CameraDriver, CameraIntent, CameraPose, NavFeel, NavMode, ViewportSize};
use egui::{Align, Color32, FontDefinitions, Layout, RichText, Sense, Ui};
use egui_dock::{DockState, NodeIndex, Style, TabViewer};
use egui_phosphor::regular as ic;

use crate::app_events::{
    AppBus, EnginePhase, OceanMode, OceanParams, PossessAdas, PossessInfo, SceneBuildProgress,
    SceneCacheStats, SceneLoadSource, SimTelemetry, SmartViewResult, StillFormat, UiCommand,
};
use crate::attr_editor::{AttrEditorState, render_attr_editor};
use crate::csg_scene_host::CsgStats;
use crate::demo_anim::{DemoAnimator, EXPOSURE_RANGE, anim_range};
use crate::profiler::{FrameTimings, GpuTelemetry, HISTORY_LEN, RingBuffer};
use crate::renderer::{
    CameraProjection, CaptureSource, DLSS_QUALITY_MODES, DemoSettings, TIME_SCALE_MAX,
    TIME_SCALE_MIN, dlss_quality_index,
};
use crate::sdg_export::{
    ActiveCrowdBackend, ActiveTrafficBackend, CrowdBackend, SdgOccluderBackend, TrafficBackend,
};
use crate::viewport_toolbar::{self as vt, TbTone, tb_icon_button, tb_separator};
use crate::overlay::{OverlayKind, OverlayScope};
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
use csg_scene::{
    AssetParamMap, AssetParamSpec, AssetPreset, ProbeBackendKind, SceneDescriptor, SceneViewpoint,
    TimeOfDay,
};
use csg_scenes::{AssetCategory, AssetDescriptor, KitAssetKind};

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
/// Width of numeric `DragValue` boxes in slider rows; sized for 9-character `11.0 pt` monospace
/// readouts with units (e.g. `"12.0 m/s2"`, `"20.0 m/s3"`) plus `2.0 px` horizontal padding.
pub(crate) const VAL_W: f32 = 68.0;
pub(crate) const ANIM_BTN_W: f32 = 16.0;
pub(crate) const ROW_H: f32 = 14.0;

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

/// Maps a registered CSG scene identifier to its Phosphor icon for the single-row scene strip.
fn scene_icon(scene_id: &str) -> &'static str {
    match scene_id {
        "gundo" | "gundo2" => ic::FACTORY,
        "gundo_osm" => ic::MAP_TRIFOLD,
        "marina" => ic::ANCHOR,
        "sf_house_of_ai" | "sf_house" => ic::BUILDINGS,
        "inspector" => ic::WRENCH,
        "proving_ground_marina" | "proving_ground" => ic::COMPASS,
        _ => ic::GLOBE,
    }
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
/// Cached descriptor, parameter schema, and presets for the currently selected Inspector asset
/// so 60 Hz UI frames never re-compose CSG blueprints or re-allocate schemas/presets.
#[derive(Clone, Debug)]
struct CachedAssetMeta {
    kind: KitAssetKind,
    descriptor: AssetDescriptor,
    schema: Vec<AssetParamSpec>,
    presets: Vec<AssetPreset>,
}

/// Persistent UI state for the Parametric Asset Inspector browser and vendored Attribute Editor.
#[derive(Clone, Debug, Default)]
pub struct AssetBrowserState {
    /// Active catalog category filter (`None` = All 111 kit assets + Crowd Showcase).
    pub category: Option<AssetCategory>,
    /// Case-insensitive asset search query across label, id, and category.
    pub search: String,
    /// Persistent filter/group/seed state for the vendored DCC Attribute Editor.
    pub attr_editor: AttrEditorState,
    cached_meta: Option<CachedAssetMeta>,
}

impl AssetBrowserState {
    fn ensure_cached(&mut self, kind: KitAssetKind) {
        let needs_rebuild = self.cached_meta.as_ref().is_none_or(|c| c.kind != kind);
        if needs_rebuild {
            self.cached_meta = Some(CachedAssetMeta {
                kind,
                descriptor: kind.descriptor(0),
                schema: kind.param_schema(),
                presets: kind.presets(),
            });
        }
    }
}

#[derive(Debug)]
pub struct UiScratch {
    /// Reusable buffer for the bottom-left Viewport status HUD line.
    pub status_buf: String,
    /// Reusable buffer for short telemetry/badge/toolbar formatted strings.
    pub label_buf: String,
    /// Reusable buffer for lazily formatted hover tooltips (formatted only while hovered).
    pub tip_buf: String,
}

impl Default for UiScratch {
    fn default() -> Self {
        Self {
            status_buf: String::with_capacity(192),
            label_buf: String::with_capacity(96),
            tip_buf: String::with_capacity(256),
        }
    }
}

/// Formats `args` into `buf` in place (clearing previous contents with zero reallocation once warmed).
#[inline]
pub(crate) fn fmt_into<'a>(buf: &'a mut String, args: std::fmt::Arguments<'_>) -> &'a str {
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

/// Draws a compact centered studio progress card inside `viewport_rect` while the render worker
/// compiles OptiX pipelines, generates a CSG world, builds per-compound GASes, or updates the IAS.
fn draw_scene_build_progress_overlay(
    ui: &mut Ui,
    viewport_rect: egui::Rect,
    theme: UiTheme,
    phase: EnginePhase,
    progress: Option<&SceneBuildProgress>,
    now: std::time::Instant,
) -> egui::Rect {
    if viewport_rect.width() < 40.0 || viewport_rect.height() < 40.0 {
        return viewport_rect;
    }
    ui.ctx()
        .request_repaint_after(std::time::Duration::from_millis(50));

    let max_w = (viewport_rect.width() - 24.0).max(1.0);
    let card_w = max_w.clamp(180.0_f32.min(viewport_rect.width()), 450.0);
    let card_h = 74.0_f32.min(viewport_rect.height());
    let card_rect =
        egui::Rect::from_center_size(viewport_rect.center(), egui::vec2(card_w, card_h));

    let is_light = matches!(theme, UiTheme::Light);
    let (card_bg, border_col, title_col, sub_col, accent_col, track_bg) = if is_light {
        (
            Color32::from_rgba_unmultiplied(246, 248, 252, 242),
            Color32::from_rgba_unmultiplied(37, 99, 235, 150),
            Color32::from_rgb(24, 29, 38),
            Color32::from_rgb(82, 94, 112),
            Color32::from_rgb(37, 99, 235),
            Color32::from_rgba_unmultiplied(15, 23, 42, 22),
        )
    } else {
        (
            Color32::from_rgba_unmultiplied(15, 18, 24, 232),
            Color32::from_rgba_unmultiplied(88, 142, 235, 165),
            Color32::from_rgb(232, 238, 248),
            Color32::from_rgb(168, 182, 204),
            Color32::from_rgb(92, 152, 255),
            Color32::from_rgba_unmultiplied(255, 255, 255, 24),
        )
    };

    let painter = ui.painter().with_clip_rect(viewport_rect);
    painter.rect_filled(card_rect, 6.0, card_bg);
    painter.rect_stroke(
        card_rect,
        6.0,
        egui::Stroke::new(1.0, border_col),
        egui::StrokeKind::Inside,
    );

    let phase_icon = match phase {
        EnginePhase::CompilingOptix => ic::CPU,
        EnginePhase::BuildingCsgScene => ic::BUILDINGS,
        EnginePhase::UpdatingIas => ic::STACK,
        EnginePhase::Rendering => ic::CHECK_CIRCLE,
    };
    let title = progress
        .filter(|bp| !bp.scene_name.is_empty())
        .map_or_else(
            || phase.transient_label().unwrap_or("Working..."),
            |bp| bp.scene_name.as_str(),
        );
    let stage_text = progress.map_or_else(
        || phase.transient_label().unwrap_or("Working...").to_owned(),
        SceneBuildProgress::format_stage_line,
    );
    let frac = progress.and_then(SceneBuildProgress::effective_fraction);
    let elapsed_text = progress
        .map(|bp| {
            let elapsed = SceneBuildProgress::format_elapsed(bp.elapsed_secs(now));
            if let Some(eta) = bp.eta_secs {
                format!("{elapsed} | ETA {}", SceneBuildProgress::format_eta(eta))
            } else if bp.step_count > 0 || frac.is_some() || phase == EnginePhase::BuildingCsgScene
            {
                format!("{elapsed} | ETA --:--")
            } else {
                elapsed
            }
        })
        .unwrap_or_default();
    let pct_text = match frac {
        Some(f) => format!("{}%", (f * 100.0).round() as u32),
        None => "Working".to_owned(),
    };

    let inner = card_rect.shrink2(egui::vec2(12.0, 9.0));
    if inner.width() <= 8.0 || inner.height() <= 16.0 {
        return card_rect;
    }

    let row1_rect = egui::Rect::from_min_max(
        inner.min,
        egui::pos2(inner.right(), (inner.top() + 18.0).min(inner.bottom())),
    );
    let mut row1 = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(row1_rect)
            .layout(Layout::right_to_left(Align::Center)),
    );
    row1.shrink_clip_rect(row1_rect);
    row1.spacing_mut().item_spacing.x = 6.0;
    if !elapsed_text.is_empty() {
        row1.label(
            RichText::new(&elapsed_text)
                .monospace()
                .size(11.0)
                .color(sub_col),
        );
    }
    row1.with_layout(Layout::left_to_right(Align::Center), |ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        ui.add(egui::Spinner::new().size(12.0).color(accent_col));
        ui.add(
            egui::Label::new(
                RichText::new(format!("{phase_icon}  {title}"))
                    .strong()
                    .size(12.0)
                    .color(title_col),
            )
            .truncate(),
        );
    });

    let row2_top = (inner.top() + 22.0).min(inner.bottom());
    let row2_rect = egui::Rect::from_min_max(
        egui::pos2(inner.left(), row2_top),
        egui::pos2(inner.right(), (row2_top + 16.0).min(inner.bottom())),
    );
    if row2_rect.height() >= 8.0 {
        let mut row2 = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(row2_rect)
                .layout(Layout::right_to_left(Align::Center)),
        );
        row2.shrink_clip_rect(row2_rect);
        row2.spacing_mut().item_spacing.x = 6.0;
        row2.label(
            RichText::new(&pct_text)
                .monospace()
                .size(10.5)
                .color(accent_col),
        );
        row2.with_layout(Layout::left_to_right(Align::Center), |ui| {
            ui.add(
                egui::Label::new(RichText::new(&stage_text).size(11.0).color(sub_col)).truncate(),
            );
        });
    }

    let bar_h = 6.0;
    let bar_rect = egui::Rect::from_min_max(
        egui::pos2(inner.left(), (inner.bottom() - bar_h).max(inner.top())),
        inner.max,
    );
    if bar_rect.height() >= 2.0 && bar_rect.width() >= 8.0 {
        let bar_painter = ui.painter().with_clip_rect(card_rect);
        bar_painter.rect_filled(bar_rect, 3.0, track_bg);
        if let Some(f) = frac {
            let fill_w = (bar_rect.width() * f).clamp(if f > 0.0 { 4.0 } else { 0.0 }, bar_rect.width());
            if fill_w > 0.0 {
                let fill_rect =
                    egui::Rect::from_min_size(bar_rect.min, egui::vec2(fill_w, bar_rect.height()));
                bar_painter.rect_filled(fill_rect, 3.0, accent_col);
            }
        } else {
            let t = ui.input(|i| i.time);
            let cycle = (t * 0.75).rem_euclid(1.0) as f32;
            let seg_min = 24.0_f32.min(bar_rect.width());
            let seg_w = (bar_rect.width() * 0.32).clamp(seg_min, bar_rect.width());
            let travel = bar_rect.width() + seg_w;
            let seg_left = (bar_rect.left() - seg_w + cycle * travel).max(bar_rect.left());
            let seg_right = (bar_rect.left() + cycle * travel).min(bar_rect.right());
            if seg_right > seg_left {
                let seg_rect = egui::Rect::from_min_max(
                    egui::pos2(seg_left, bar_rect.top()),
                    egui::pos2(seg_right, bar_rect.bottom()),
                );
                bar_painter.rect_filled(seg_rect, 3.0, accent_col);
            }
        }
    }

    card_rect
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
    FlightLab,
    FlightPlayer,
}

impl DockTab {
    pub const ALL: [Self; 6] = [
        Self::Viewport,
        Self::Fractal,
        Self::RenderFloor,
        Self::CameraStats,
        Self::FlightLab,
        Self::FlightPlayer,
    ];

    pub fn title(self) -> &'static str {
        static TITLES: LazyLock<[String; 6]> = LazyLock::new(|| {
            [
                format!("{}  Viewport", ic::APERTURE),
                format!("{}  Fractal", ic::CUBE),
                format!("{}  Render & Floor", ic::SUN),
                format!("{}  Camera & Stats", ic::GAUGE),
                format!("{}  Flight Lab", ic::FILM_SLATE),
                format!("{}  Player", ic::PLAYLIST),
            ]
        });
        let idx = match self {
            Self::Viewport => 0,
            Self::Fractal => 1,
            Self::RenderFloor => 2,
            Self::CameraStats => 3,
            Self::FlightLab => 4,
            Self::FlightPlayer => 5,
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
    let [vp, _left] = surface.split_left(
        NodeIndex::root(),
        0.17,
        vec![DockTab::CameraStats, DockTab::FlightPlayer, DockTab::FlightLab],
    );
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
        // Unify stroke width, expansion, and corner radius across all widget states so buttons
        // with explicit 1px `.stroke(...)` never change `inner_margin` or height on hover
        // (`Style::button_style` computes `button_padding + expansion - bg_stroke.width`).
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, Color32::TRANSPARENT);
        for w in [
            &mut visuals.widgets.noninteractive,
            &mut visuals.widgets.inactive,
            &mut visuals.widgets.hovered,
            &mut visuals.widgets.active,
            &mut visuals.widgets.open,
        ] {
            w.bg_stroke.width = 1.0;
            w.expansion = 0.0;
            w.corner_radius = egui::CornerRadius::same(2);
        }
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
    pub fractal_presets: &'a mut crate::fractal_presets::FractalPresetManager,
    pub display_tex_id: egui::TextureId,
    pub render_w: u32,
    pub render_h: u32,
    pub desired_vp_size: &'a mut (f32, f32),
    pub cam_driver: &'a mut CameraDriver,
    pub cam_home: CameraPose,
    pub cam_pose: &'a mut CameraPose,
    pub flight_workspace: &'a mut crate::flight::FlightWorkspace,
    pub active_scene_label: &'a str,
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
    /// Intersection counts of the latest counted launch (`is_counters.rs`).
    pub is_counts: Option<crate::is_counters::IsCounts>,
    pub denoise_ms: f32,
    pub bloom_ms: f32,
    pub tonemap_ms: f32,
    /// Hardware **Tensor Cores (Denoiser / DLSS)** execution time in milliseconds.
    pub tensor_cores_ms: f32,
    /// Hardware **CUDA Compute (IAS / cuFFT / Tonemap)** execution time in milliseconds.
    pub cuda_compute_ms: f32,
    /// Active engine execution phase (`CompilingOptix`, `BuildingCsgScene`, `UpdatingIas`, `Rendering`).
    pub engine_phase: EnginePhase,
    /// Live render-worker compile/build progress snapshot (`None` when steady-state rendering).
    pub build_progress: Option<&'a SceneBuildProgress>,
    pub ui_fps: f32,
    pub rt_fps: f32,
    pub fast_preview: bool,
    pub is_recording: bool,
    pub record_frames: u64,
    pub ser_supported: bool,
    pub csg_stats: CsgStats,
    pub sim_telemetry: SimTelemetry,
    pub scene_descriptors: &'a [SceneDescriptor],
    pub scene_viewpoints: &'a [SceneViewpoint],
    pub dlss_rr_available: bool,
    pub hdr_surface_supported: bool,
    pub viewport_focused: &'a mut bool,
    pub possess_info: Option<&'a PossessInfo>,
    pub possess_adas: Option<PossessAdas>,
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
    /// Live Ocean Water parameters (`cuFFT` 3-cascade spectral / analytical Gerstner + dielectric optics).
    pub ocean_params: &'a mut OceanParams,
    /// Live parametric overrides for the selected Inspector asset (`RenderControl::SetAssetParams`).
    pub asset_params: &'a mut AssetParamMap,
    /// Persistent filter/search/group UI state for the Parametric Asset Inspector.
    pub asset_browser: &'a mut AssetBrowserState,
    /// Live `SceneDiskCache` hit/size telemetry delivered via `RenderEvent::SceneCacheStats`.
    pub scene_cache_stats: SceneCacheStats,
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
            DockTab::FlightLab => dock_scroll(ui, |ui| self.ui_flight_lab_tab(ui)),
            DockTab::FlightPlayer => dock_scroll(ui, |ui| self.ui_flight_player_tab(ui)),
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
            allocate_clamped_ui(
                ui,
                egui::vec2(inner_w, 0.0),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_width(inner_w);
                    add_contents(ui);
                },
            );
        });
}

#[cfg(test)]
pub(crate) fn dock_scroll_for_test(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    dock_scroll(ui, add_contents);
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

            ui.label(RichText::new("Orbit").small().weak())
                .on_hover_text("Primary camera orbiting style used when dragging with the left mouse button.");
            egui::ComboBox::from_id_salt("toolbar_cam_mode")
                .width(96.0)
                .selected_text(self.cam_mode.label())
                .show_ui(ui, |ui| {
                    for m in NavMode::ALL.into_iter().filter(|m| !m.is_fly()) {
                        ui.selectable_value(self.cam_mode, m, m.label());
                    }
                })
                .response
                .on_hover_text("Primary camera orbiting style used when dragging with the left mouse button.");

            ui.label(RichText::new("Fly (RMB)").small().weak())
                .on_hover_text("Free-flight camera mode while holding the right mouse button and using WASD/QE.");
            egui::ComboBox::from_id_salt("toolbar_cam_fly")
                .width(104.0)
                .selected_text(self.cam_fly_mode.label())
                .show_ui(ui, |ui| {
                    for m in NavMode::ALL.into_iter().filter(|m| m.is_fly()) {
                        ui.selectable_value(self.cam_fly_mode, m, m.label());
                    }
                })
                .response
                .on_hover_text("Free-flight camera mode while holding the right mouse button and using WASD/QE.");

            ui.label(RichText::new("Scheme").small().weak())
                .on_hover_text("Mouse and keyboard shortcut layout matching familiar 3D apps (Maya, Houdini, Blender, Unreal).");
            egui::ComboBox::from_id_salt("toolbar_cam_scheme")
                .width(74.0)
                .selected_text(nav_scheme_label(*self.cam_scheme))
                .show_ui(ui, |ui| {
                    for s in NavScheme::ALL {
                        ui.selectable_value(self.cam_scheme, s, nav_scheme_label(s));
                    }
                })
                .response
                .on_hover_text("Mouse and keyboard shortcut layout matching familiar 3D apps (Maya, Houdini, Blender, Unreal).");

            ui.label(RichText::new("Feel").small().weak())
                .on_hover_text("Camera motion damping: Sharp stops immediately while Smooth glides softly to a halt.");
            egui::ComboBox::from_id_salt("toolbar_cam_feel")
                .width(74.0)
                .selected_text(nav_feel_label(*self.cam_feel))
                .show_ui(ui, |ui| {
                    for f in NavFeel::ALL {
                        ui.selectable_value(self.cam_feel, f, nav_feel_label(f));
                    }
                })
                .response
                .on_hover_text("Camera motion damping: Sharp stops immediately while Smooth glides softly to a halt.");

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
                self.flight_workspace.player.pause();
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
                self.flight_workspace.player.pause();
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

            ui.label(RichText::new("AOV").small().weak())
                .on_hover_text("Select which render output pass to display (final Beauty image, depth, normals, motion, semantic labels, etc.).");
            let aov_label_items: Vec<(u32, &str)> = AOV_PASSES
                .iter()
                .enumerate()
                .map(|(idx, pass)| (idx as u32, pass.label))
                .collect();
            vt::tb_stepper_combo(
                ui,
                "toolbar_aov_mode",
                "",
                96.0,
                "Select which render output pass to display (final Beauty image, depth, normals, motion, semantic labels, etc.).",
                &aov_label_items,
                &mut self.settings.aov_mode,
            );

            ui.label(RichText::new("Lens").small().weak())
                .on_hover_text("Camera optical lens projection: standard Pinhole, 180-degree Fisheye, 360 Equirectangular, or SlitCam.");
            let toolbar_lens_items: [(CameraProjection, &str); 4] =
                CameraProjection::ALL.map(|p| (p, p.label()));
            vt::tb_stepper_combo(
                ui,
                "toolbar_lens_proj",
                "",
                vt::TB_COMBO_WIDTH - 8.0,
                "Camera optical lens projection: standard Pinhole, 180-degree Fisheye, 360 Equirectangular, or SlitCam.",
                &toolbar_lens_items,
                &mut self.settings.camera_projection,
            );

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
                // 1. Display mode (DEMO:VIEWS), AOV / line width, lens — systemic stepped combos
                // with flush `<` / `>` quick-cycle arrows and hover-wheel stepping.
                let display_items: [(DisplayMode, &str); 2] =
                    DisplayMode::ALL.map(|m| (m, m.label()));
                vt::tb_stepper_combo(
                    ui,
                    "vt_display",
                    ic::CUBE,
                    vt::TB_COMBO_WIDTH - 16.0,
                    "Display mode: CSG = shaded path-traced render; Wireframe = smooth \
                     hidden-line contours (silhouettes, creases, instance borders).",
                    &display_items,
                    &mut self.settings.display_mode,
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
                    let aov_items: Vec<(u32, &str)> = AOV_PASSES
                        .iter()
                        .enumerate()
                        .map(|(idx, pass)| (idx as u32, pass.tag))
                        .collect();
                    vt::tb_stepper_combo(
                        ui,
                        "vt_aov",
                        ic::STACK,
                        vt::TB_COMBO_WIDTH - 8.0,
                        "Output pass (AOV)",
                        &aov_items,
                        &mut self.settings.aov_mode,
                    );
                }
                let lens_items: [(CameraProjection, &str); 4] =
                    CameraProjection::ALL.map(|p| (p, p.label()));
                vt::tb_stepper_combo(
                    ui,
                    "vt_lens",
                    ic::APERTURE,
                    vt::TB_COMBO_WIDTH + 4.0,
                    "Lens / sensor projection",
                    &lens_items,
                    &mut self.settings.camera_projection,
                );
                tb_separator(ui);

                // 1b. Overlay (overlay.rs): master switch, kinds, scope, screen-size cull, captures
                let ov = &mut self.settings.overlay;
                if tb_icon_button(ui, ic::LINE_SEGMENTS, ov.enabled, TbTone::Normal, "Overlay: draw overlay lines")
                    .clicked()
                {
                    ov.enabled = !ov.enabled;
                }
                ui.add_enabled_ui(ov.enabled, |ui| {
                    for kind in OverlayKind::ALL {
                        let icon = match kind {
                            OverlayKind::Contours => ic::POLYGON,
                            OverlayKind::Axes => ic::ARROWS_OUT_CARDINAL,
                            OverlayKind::Skeletons => ic::USERS_THREE,
                            OverlayKind::FlightPath => ic::PATH,
                            OverlayKind::Adas => ic::WARNING,
                        };
                        if tb_icon_button(ui, icon, ov.kinds & kind.bit() != 0, TbTone::Normal, kind.label())
                            .clicked()
                        {
                            ov.kinds ^= kind.bit();
                        }
                    }
                    for scope in OverlayScope::ALL {
                        let icon = match scope {
                            OverlayScope::All => ic::GLOBE,
                            OverlayScope::Selected => ic::CURSOR_CLICK,
                            OverlayScope::WithinRadius => ic::CIRCLE_DASHED,
                        };
                        if tb_icon_button(ui, icon, ov.scope == scope, TbTone::Normal, scope.label()).clicked() {
                            ov.scope = scope;
                        }
                    }
                    if ov.kinds & OverlayKind::Axes.bit() != 0 {
                        ui.add(
                            egui::DragValue::new(&mut ov.axis_px)
                                .range(crate::overlay::AXIS_PX_RANGE)
                                .speed(0.5)
                                .suffix(" px"),
                        )
                        .on_hover_text("Object axis length on screen (constant at any distance)");
                        ui.add(
                            egui::DragValue::new(&mut ov.axes_min_px)
                                .range(crate::overlay::AXES_MIN_PX_RANGE)
                                .speed(1.0)
                                .prefix("LOD ")
                                .suffix(" px"),
                        )
                        .on_hover_text(
                            "Axes LOD: a part's axes are drawn only when the part itself spans at least this \
                             many pixels on screen (they fade in up to 3x); the selection always shows",
                        );
                    }
                    if ov.scope == OverlayScope::WithinRadius {
                        ui.add(egui::DragValue::new(&mut ov.radius_m).range(1.0..=5000.0).speed(1.0).suffix(" m"))
                            .on_hover_text("Overlay radius around the camera");
                    }
                    ui.add_sized(vt::TB_BUTTON_SIZE, egui::Label::new(RichText::new(ic::FUNNEL).size(vt::TB_ICON_SIZE)))
                        .on_hover_text("Screen-size cull");
                    ui.add(egui::DragValue::new(&mut ov.min_screen_px).range(0.0..=64.0).speed(0.1).suffix(" px"))
                        .on_hover_text(
                            "Line LOD: shapes smaller than this on screen are not drawn; contour                              lines fade in between this and 3x this projected object size",
                        );
                    if tb_icon_button(
                        ui,
                        ic::WAVES,
                        ov.temporal,
                        TbTone::Normal,
                        "Temporal stabilisation: reproject and blend the line coverage so lines do not flicker",
                    )
                    .clicked()
                    {
                        ov.temporal = !ov.temporal;
                    }
                    if tb_icon_button(
                        ui,
                        ic::FILM_SLATE,
                        ov.in_captures,
                        TbTone::Normal,
                        "Include the overlay in video recordings (stills always show the viewport image)",
                    )
                    .clicked()
                    {
                        ov.in_captures = !ov.in_captures;
                    }
                });
                tb_separator(ui);

                // 1c. Viewport category filter (`semantic_filter.rs`): the eye dropdown.
                semantic_filter_menu(
                    ui,
                    &mut self.settings.hidden_semantics,
                    &mut self.settings.hidden_osm_classes,
                );
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
                    if let Some(adas) = self.possess_adas {
                        tb_separator(ui);
                        render_possess_adas_pill(ui, adas);
                    }
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
        // Viewport click model (`crate::pick`, shared with warpbro-rs / squarebob-rs through
        // `sl_config::sel_mods`, modifiers latched at the press): a click selects the object
        // under the cursor (Shift toggles it in the selection; empty space clears unless Shift
        // is held); Ctrl + click, a double click or a click in Pick crosshair mode possesses it.
        // Both are answered by the GPU pick record of the next launch.
        let mods = sl_config::list::sel_mods(ui.ctx());
        let clicked = !split_dragging && !toolbar_blocking && resp.clicked_by(egui::PointerButton::Primary);
        let pick_clicked = !split_dragging
            && !toolbar_blocking
            && self.settings.csg_mode != 0
            && (resp.double_clicked_by(egui::PointerButton::Primary)
                || (clicked && (*self.pick_possess_mode || mods.toggle)));
        let action = if pick_clicked {
            Some(crate::pick::PickAction::Possess)
        } else if clicked {
            Some(crate::pick::PickAction::Select { add: mods.range })
        } else {
            None
        };
        if let Some(action) = action
            && let Some(pos) = resp.interact_pointer_pos()
            && img_rect.width() > 2.0
            && img_rect.height() > 2.0
            && img_rect.contains(pos)
        {
            let norm_x = ((pos.x - img_rect.left()) / img_rect.width()).clamp(0.0, 1.0);
            let norm_y = ((pos.y - img_rect.top()) / img_rect.height()).clamp(0.0, 1.0);
            self.bus
                .emit_ui(UiCommand::Pick(crate::pick::PickRequest { norm_x, norm_y, action }));
            if pick_clicked {
                *self.pick_possess_mode = false;
            }
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
            if resp.dragged() && self.flight_workspace.player.is_active() {
                self.flight_workspace.player.pause();
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
                self.flight_workspace.player.pause();
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
                self.flight_workspace.player.pause();
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
                self.flight_workspace.player.pause();
                let (home_pose, orbit_dist) =
                    resolve_home_pose(self.settings.csg_mode, self.scene_viewpoints, self.cam_home);
                self.cam_driver.controller_mut().set_pose(home_pose);
                if let Some(d) = orbit_dist {
                    self.cam_driver.controller_mut().set_orbit_distance(d);
                }
                *self.cam_pose = self.cam_driver.controller().pose();
                self.bus.emit_ui(UiCommand::SnapCamera);
            } else if !self.flight_workspace.player.is_active() {
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

        let now = std::time::Instant::now();
        let status_buf = &mut self.scratch.status_buf;
        status_buf.clear();
        status_buf.push_str(phase_icon);
        status_buf.push(' ');
        if self.engine_phase != EnginePhase::Rendering
            && let Some(bp) = self.build_progress
        {
            bp.write_status_summary(status_buf, bp.elapsed_secs(now));
        } else {
            write_engine_phase_label(
                status_buf,
                self.engine_phase,
                self.accum_count,
                self.settings.max_accum,
                anim_slice,
            );
        }
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

        // Centered studio progress card while compiling pipelines or building a CSG scene / IAS.
        if self.engine_phase != EnginePhase::Rendering {
            draw_scene_build_progress_overlay(
                ui,
                rect,
                *self.theme,
                self.engine_phase,
                self.build_progress,
                now,
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
        let presets = &mut *self.fractal_presets;
        collapsing_section(ui, "sec_presets", ic::SPARKLE, "Presets", |ui| {
            presets.draw_preset_bar(ui, s);
            ui.add_space(4.0);

            let builtins = crate::fractal_presets::builtin_presets();
            let modified = presets.is_selected_modified(s);
            let mut load_builtin: Option<&'static str> = None;
            equal_grid(ui, 2, builtins.len(), |ui, idx, w| {
                let Some(spec) = builtins.get(idx) else {
                    return;
                };
                let selected = presets.is_selected(spec.name);
                let btn = if selected && modified {
                    egui::Button::selectable(true, format!("{} *", spec.name)).truncate()
                } else {
                    egui::Button::selectable(selected, spec.name).truncate()
                };
                if ui
                    .add_sized([w, ROW_H + 2.0], btn)
                    .on_hover_text(spec.tooltip)
                    .clicked()
                {
                    load_builtin = Some(spec.name);
                }
            });
            if let Some(name) = load_builtin {
                presets.apply_preset(name, s);
            }

            let user_count = presets.user_presets().count();
            if user_count > 0 {
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("User Presets")
                        .size(11.0)
                        .color(ui.visuals().weak_text_color()),
                );
                ui.add_space(2.0);
                let mut load_user: Option<String> = None;
                equal_grid(ui, 2, user_count, |ui, idx, w| {
                    let Some(preset) = presets.user_presets().nth(idx) else {
                        return;
                    };
                    let name = preset.name.as_str();
                    let selected = presets.is_selected(name);
                    let btn = if selected && modified {
                        egui::Button::selectable(true, format!("{name} *")).truncate()
                    } else {
                        egui::Button::selectable(selected, name).truncate()
                    };
                    if ui
                        .add_sized([w, ROW_H + 2.0], btn)
                        .on_hover_ui(|ui| {
                            ui.label(format!("Load user preset '{name}'"));
                        })
                        .clicked()
                    {
                        load_user = Some(name.to_string());
                    }
                });
                if let Some(name) = load_user {
                    presets.apply_preset(&name, s);
                }
            }

            if let Some(status) = &presets.status {
                ui.add_space(3.0);
                let (icon, msg, color) = match status {
                    crate::fractal_presets::PresetStatus::Info(m) => {
                        (ic::CHECK_CIRCLE, m.as_str(), ui.visuals().weak_text_color())
                    }
                    crate::fractal_presets::PresetStatus::Warning(m) => {
                        (ic::WARNING, m.as_str(), ui.visuals().warn_fg_color)
                    }
                };
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!("{icon}  {msg}"))
                            .size(11.0)
                            .color(color),
                    )
                    .wrap(),
                );
            }
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
        self.flight_workspace.player.pause();
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

                // Single-row Phosphor icon strip driven strictly by `SceneRegistry::descriptors()`.
                equal_grid(ui, total_choices, total_choices, |ui, idx, w| {
                    let (mode, icon, label, tip) = if idx == 0 {
                        (
                            0u32,
                            ic::CUBE,
                            "Off (Classic SDF Fractal)",
                            "Classic SDF fractal + stage floor",
                        )
                    } else if let Some(d) = descs.get(idx - 1) {
                        (idx as u32, scene_icon(d.id), d.name, d.description)
                    } else {
                        (0u32, ic::CUBE, "Off (Classic SDF Fractal)", "")
                    };
                    let mut btn = egui::Button::new(RichText::new(icon).size(13.0));
                    if s.csg_mode == mode {
                        btn = btn
                            .fill(active_accent_tint(ui))
                            .stroke(egui::Stroke::new(1.0, ui.visuals().selection.stroke.color));
                    }
                    let hover_tip = if tip.is_empty() {
                        label.to_owned()
                    } else {
                        format!("{label}\n{tip}")
                    };
                    if ui
                        .add_sized([w, ROW_H + 6.0], btn)
                        .on_hover_text(hover_tip)
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
                let active_label = active_desc.map_or("Off (Classic SDF Fractal)", |d| d.name);
                stat_row(ui, "Scene", active_label);

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
                            let btn_w = 22.0;
                            ui.spacing_mut().item_spacing.x = gap;
                            ui.spacing_mut().interact_size.x = btn_w;
                            ui.spacing_mut().button_padding.x = 2.0;
                            let avail = ui.available_width();
                            let combo_w = (avail - 2.0 * btn_w - 2.0 * gap).max(32.0);
                            if ui
                                .add_sized(
                                    [btn_w, ROW_H],
                                    egui::Button::new(ic::CARET_LEFT)
                                        .min_size(egui::vec2(btn_w, ROW_H)),
                                )
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
                                .add_sized(
                                    [btn_w, ROW_H],
                                    egui::Button::new(ic::CARET_RIGHT)
                                        .min_size(egui::vec2(btn_w, ROW_H)),
                                )
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

                    // Single-row Day / Sunset / Night + Smooth transition duration controls.
                    let row_w = ui.available_width().max(1.0);
                    allocate_clamped_ui(
                        ui,
                        egui::vec2(row_w, ROW_H + 2.0),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            let sp = 4.0_f32;
                            ui.spacing_mut().item_spacing.x = sp;
                            let avail = ui.available_width().max(120.0);
                            let dur_w = 46.0_f32;
                            let show_smooth_lbl = avail >= 285.0;
                            let chk_w = if show_smooth_lbl { 62.0 } else { 18.0 };
                            let right_w = chk_w + sp + dur_w + sp;
                            let seg_total = (avail - right_w).max(90.0);
                            let seg_w = ((seg_total - 2.0 * sp) / 3.0).max(26.0);
                            for (tod, label) in [(0u32, "Day"), (1u32, "Sunset"), (2u32, "Night")] {
                                if ui
                                    .add_sized(
                                        [seg_w, ROW_H + 2.0],
                                        egui::Button::selectable(s.csg_time_of_day == tod, label),
                                    )
                                    .clicked()
                                {
                                    self.bus.emit_ui(UiCommand::SetTimeOfDay(tod));
                                }
                            }
                            ui.spacing_mut().interact_size.x = 18.0;
                            let chk_lbl = if show_smooth_lbl { "Smooth" } else { "" };
                            ui.checkbox(&mut s.tod_transition_enabled, chk_lbl)
                                .on_hover_text(
                                    "Smoothly animate sky and sun lighting across Day / Sunset / Night.",
                                );
                            ui.add_enabled_ui(s.tod_transition_enabled, |ui| {
                                ui.add_sized(
                                    [dur_w, ROW_H],
                                    egui::DragValue::new(&mut s.tod_transition_secs)
                                        .range(0.1..=15.0)
                                        .speed(0.1)
                                        .suffix(" s"),
                                )
                                .on_hover_text("Transition duration in seconds.");
                            });
                        },
                    );
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
                    // 2-way Ground Kerb Mode selector (`0 = In-Tile F-Rep`, `1 = Sweep Tile`).
                    let prev_kerb_mode = s.csg_ground_kerb_mode.min(1);
                    ui.label(RichText::new("Ground Kerb").small().weak())
                        .on_hover_text(
                            "How ground-level (layer 0) road kerbs are represented in F-Rep/Hybrid road tiles (elevated bridges always use Sweep Tile).",
                        );
                    equal_grid(
                        ui,
                        csg_scene::GroundKerbMode::ALL.len(),
                        csg_scene::GroundKerbMode::ALL.len(),
                        |ui, idx, w| {
                            let kerb = csg_scene::GroundKerbMode::ALL[idx];
                            let mode = idx as u8;
                            let tip = match kerb {
                                csg_scene::GroundKerbMode::InTile => {
                                    "In-Tile F-Rep (default): ground-level kerbs stay baked analytically inside RoadTerrainTile with zero extra GAS/IAS instances."
                                }
                                csg_scene::GroundKerbMode::SweepTile => {
                                    "Sweep Tile: road tiles bake flush (zero kerb Z-fighting) and ground-level kerbs emit into analytical SWEEP_TILE (GasClass::Sweep)."
                                }
                            };
                            if ui
                                .add_sized(
                                    [w, ROW_H + 2.0],
                                    egui::Button::selectable(prev_kerb_mode == mode, kerb.label()),
                                )
                                .on_hover_text(tip)
                                .clicked()
                                && prev_kerb_mode != mode
                            {
                                s.csg_ground_kerb_mode = mode;
                                self.bus.emit_ui(UiCommand::SetGroundKerbMode(mode));
                            }
                        },
                    );
                    // Opt-in Smart Fill (OSM inference) controls (`SmartFill::OFF` = strictly source data).
                    let sf_bits = s.csg_smart_fill.to_bits();
                    let sf_on = sf_bits.count_ones();
                    let sf_summary = match sf_on {
                        0 => "Smart Fill (Off - strict OSM)".to_string(),
                        10 => "Smart Fill (All 10/10)".to_string(),
                        n => format!("Smart Fill ({n}/10)"),
                    };
                    egui::CollapsingHeader::new(RichText::new(sf_summary).small())
                        .id_salt("csg_smart_fill_hdr")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new("Opt-in procedural inference when OSM tags omit street details (rebuilds scene).")
                                    .small()
                                    .weak(),
                            );
                            equal_grid(ui, 2, 2, |ui, idx, w| {
                                if idx == 0 {
                                    if ui
                                        .add_sized([w, ROW_H], egui::Button::new("Strict OSM (Off)"))
                                        .on_hover_text("Disable all procedural inference; build strictly from source OSM data.")
                                        .clicked()
                                        && s.csg_smart_fill != csg_scene::SmartFill::OFF
                                    {
                                        s.csg_smart_fill = csg_scene::SmartFill::OFF;
                                    }
                                } else if ui
                                    .add_sized([w, ROW_H], egui::Button::new("Enable All"))
                                    .on_hover_text("Enable procedural inference for all 10 street-detail classes.")
                                    .clicked()
                                    && s.csg_smart_fill != csg_scene::SmartFill::ALL
                                {
                                    s.csg_smart_fill = csg_scene::SmartFill::ALL;
                                }
                            });
                            let sf = &mut s.csg_smart_fill;
                            let items: [(&str, &str, &mut bool); 10] = [
                                (
                                    "Lamps",
                                    "Infer street lamps along kerbs of lit road classes.",
                                    &mut sf.street_lamps,
                                ),
                                (
                                    "Trees",
                                    "Infer verge and median trees by road class and zone.",
                                    &mut sf.trees,
                                ),
                                (
                                    "Sidewalks",
                                    "Infer sidewalks on roads missing explicit sidewalk tags.",
                                    &mut sf.sidewalks,
                                ),
                                (
                                    "Crossings",
                                    "Infer crosswalks at junction approaches without crossing tags.",
                                    &mut sf.crossings,
                                ),
                                (
                                    "Signals",
                                    "Infer traffic signals at untagged junctions of classified roads.",
                                    &mut sf.traffic_signals,
                                ),
                                (
                                    "Stop Signs",
                                    "Infer stop/give-way signs on minor-into-major approaches.",
                                    &mut sf.stop_give_way,
                                ),
                                (
                                    "Hydrants",
                                    "Infer fire hydrants along the kerb-side furniture strip.",
                                    &mut sf.hydrants,
                                ),
                                (
                                    "Meters",
                                    "Infer parking meters along the kerb-side furniture strip.",
                                    &mut sf.parking_meters,
                                ),
                                (
                                    "Bollards",
                                    "Infer bollards along kerbs and pedestrian zones.",
                                    &mut sf.bollards,
                                ),
                                (
                                    "Work Zones",
                                    "Infer procedural road-work zones on industrial service roads.",
                                    &mut sf.work_zones,
                                ),
                            ];
                            egui::Grid::new("csg_smart_fill_grid")
                                .num_columns(2)
                                .spacing([8.0, 2.0])
                                .show(ui, |ui| {
                                    for (i, (label, tip, flag)) in items.into_iter().enumerate() {
                                        ui.checkbox(flag, label).on_hover_text(tip);
                                        if i % 2 == 1 {
                                            ui.end_row();
                                        }
                                    }
                                });
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
                    let crowd_badge = if self.sim_telemetry.crowd_actors > 0 {
                        fmt_into(
                            &mut self.scratch.label_buf,
                            format_args!(
                                "Crowd Backend ({} {:.2} ms, {}a)",
                                self.sim_telemetry.crowd_backend.label(),
                                self.sim_telemetry.crowd_step_ms,
                                self.sim_telemetry.crowd_actors
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
                    if desc.supports_traffic {
                        let traffic_badge = if self.sim_telemetry.traffic_vehicles > 0 {
                            fmt_into(
                                &mut self.scratch.label_buf,
                                format_args!(
                                    "Traffic Backend ({} {:.2} ms, {}v)",
                                    self.sim_telemetry.traffic_backend.label(),
                                    self.sim_telemetry.traffic_step_ms,
                                    self.sim_telemetry.traffic_vehicles
                                ),
                            )
                        } else {
                            "Traffic Backend"
                        };
                        ui.label(RichText::new(traffic_badge).small().weak());
                        equal_grid(ui, 3, 3, |ui, idx, w| {
                            let b = TrafficBackend::ALL[idx];
                            let tip = match b {
                                TrafficBackend::Auto => "Prefer GPU-direct CUDA PlanKind::Traffic v1 with VRAM pedestrian AEB; falls back to CPU SoA and reports active backend.",
                                TrafficBackend::Cpu => "Run analytical CPU SoA lane-graph traffic oracle.",
                                TrafficBackend::Cuda => "Require GPU-direct CUDA PlanKind::Traffic v1 (returns error if unavailable; never downgrades).",
                            };
                            if ui
                                .add_sized(
                                    [w, ROW_H + 2.0],
                                    egui::Button::selectable(s.traffic_backend == b, b.label()),
                                )
                                .on_hover_text(tip)
                                .clicked()
                            {
                                s.traffic_backend = b;
                            }
                        });
                    }
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
                    ui.add_space(3.0);
                    let UiScratch {
                        label_buf: buf,
                        tip_buf,
                        ..
                    } = &mut *self.scratch;
                    if render_scene_cache_row(ui, self.scene_cache_stats, buf, tip_buf) {
                        self.bus.emit_ui(UiCommand::ClearSceneCache);
                    }
                }
            });

            if s.csg_mode != 0
                && descs
                    .get((s.csg_mode as usize).saturating_sub(1))
                    .is_some_and(|d| d.id == "inspector")
            {
                collapsing_section(
                    ui,
                    "sec_asset_inspector",
                    ic::SLIDERS_HORIZONTAL,
                    "Asset Inspector",
                    |ui| {
                        let UiScratch {
                            label_buf: buf,
                            tip_buf,
                            ..
                        } = &mut *self.scratch;
                        if render_asset_inspector_body(
                            ui,
                            &mut s.csg_asset_idx,
                            self.asset_params,
                            self.asset_browser,
                            buf,
                            tip_buf,
                        ) {
                            request_auto_frame = true;
                        }
                    },
                );
            }
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

        collapsing_section(ui, "sec_ocean", ic::WAVES, "Ocean Water", |ui| {
            render_ocean_water_controls(ui, self.ocean_params);
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

    /// Shared dispatcher for [`DockTab::FlightLab`] and [`DockTab::FlightPlayer`].
    fn ui_flight_tab(
        &mut self,
        ui: &mut Ui,
        render: fn(&mut Ui, &mut crate::flight::ui::FlightUiArgs<'_>),
    ) {
        let mut seek_pose: Option<CameraPose> = None;
        {
            let bus = self.bus;
            let cur_pose = *self.cam_pose;
            let s = &mut *self.settings;
            let is_csg_scene = s.csg_mode != 0;
            let csg_lod_bias = s.csg_lod_bias;
            let mut on_cmd = |cmd: crate::flight::FlightCommand| {
                bus.emit_ui(UiCommand::Flight(cmd));
            };
            let mut on_export = || {
                bus.emit_ui(UiCommand::ExportFlightCsv);
            };
            let mut on_seek = |pose: CameraPose| {
                seek_pose = Some(pose);
            };
            let mut args = crate::flight::ui::FlightUiArgs {
                workspace: self.flight_workspace,
                flight_stats: self.flight,
                csg_stats: &self.csg_stats,
                is_csg_scene,
                csg_lod_bias,
                crowd_actor_count: self.sim_telemetry.crowd_actors,
                rt_ms: self.rt_ms,
                is_counters: &mut s.is_counters,
                last_is_counts: self.is_counts.as_ref(),
                active_scene_label: self.active_scene_label,
                cur_pose,
                is_recording: self.is_recording,
                time_scale: &mut s.time_scale,
                cam_smooth_tau: &mut s.cam_smooth_tau,
                label_buf: &mut self.scratch.label_buf,
                on_command: &mut on_cmd,
                on_export_flight_csv: &mut on_export,
                on_seek_pose: &mut on_seek,
            };
            render(ui, &mut args);
        }
        if let Some(pose) = seek_pose {
            self.flight_workspace.player.pause();
            self.cam_driver.controller_mut().set_pose(pose);
            *self.cam_pose = pose;
            self.bus.emit_ui(UiCommand::SnapCamera);
        }
    }

    fn ui_flight_lab_tab(&mut self, ui: &mut Ui) {
        self.ui_flight_tab(ui, crate::flight::ui::ui_flight_lab);
    }

    fn ui_flight_player_tab(&mut self, ui: &mut Ui) {
        self.ui_flight_tab(ui, crate::flight::ui::ui_flight_player);
    }

    fn ui_camera_stats(&mut self, ui: &mut Ui) {
        {
            let s = &mut *self.settings;
            collapsing_section(
                ui,
                "sec_view_possess",
                ic::VIDEO_CAMERA,
                "View & Possess",
                |ui| {
                    let csg_active = s.csg_mode != 0;
                    equal_grid(ui, 2, 2, |ui, idx, w| match idx {
                        0 => {
                            if ui
                                .add_enabled(
                                    csg_active,
                                    egui::Button::new(format!("{}  Possess Car", ic::CAR))
                                        .min_size(egui::vec2(w, ROW_H + 2.0)),
                                )
                                .on_hover_text(
                                    "Jump into a driving car or mounted landmark camera. Click again to cycle targets, or Ctrl+Click anything in the viewport.",
                                )
                                .clicked()
                            {
                                self.bus.emit_ui(UiCommand::PossessNextVehicle);
                            }
                        }
                        _ => {
                            if ui
                                .add_enabled(
                                    csg_active,
                                    egui::Button::new(format!("{}  Smart View", ic::EYE))
                                        .min_size(egui::vec2(w, ROW_H + 2.0)),
                                )
                                .on_hover_text(
                                    "Evaluate 64 candidate viewpoints across the scene and smoothly glide the camera to the best framing.",
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
                    let probe_tip = "Engine used by Smart View to score candidate camera views. Auto picks OptiX 9 GPU when available.";
                    prop_row_labeled(ui, "Probe", true, |ui, label_resp| {
                        label_resp.on_hover_text(probe_tip);
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
                            })
                            .response
                            .on_hover_text(probe_tip);
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
                        ui.label(RichText::new(sv_txt).size(10.5).weak())
                            .on_hover_text("Score summary from the most recent Smart View run: composite rating, views checked, scene variety, depth spread, and search time.");
                    }
                    prop_slider_f32_tip(
                        ui,
                        "Cam Smooth",
                        &mut s.cam_smooth_tau,
                        0.0..=1.0,
                        2,
                        "s",
                        "How gently the camera eases into movement and turns. Higher seconds feel more cinematic; 0 s responds instantly.",
                    );
                    prop_slider_f32_tip(
                        ui,
                        "Time Scale",
                        &mut s.time_scale,
                        TIME_SCALE_MIN..=TIME_SCALE_MAX,
                        2,
                        "x",
                        "Playback speed multiplier for moving traffic, crowds, animations, and flight rails (1.0x is real time).",
                    );
                },
            );
        }

        collapsing_section(ui, "sec_render_stats", ic::MONITOR_PLAY, "Render & Frame", |ui| {
            let UiScratch {
                label_buf: buf,
                tip_buf,
                ..
            } = &mut *self.scratch;
            stat_row_desc(
                ui,
                "Resolution",
                fmt_into(buf, format_args!("{} x {}", self.render_w, self.render_h)),
                "Internal ray-tracing render width and height in pixels (lower with DLSS upscaling for faster frames).",
                tip_buf,
            );
            stat_row_desc(
                ui,
                "UI FPS",
                fmt_into(buf, format_args!("{:.0} FPS", self.ui_fps)),
                "How smoothly the interface and dock panels redraw each second, decoupled from ray-tracing speed.",
                tip_buf,
            );
            stat_row_desc(
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
                "Ray-traced frames completed per second and render time per frame ('fast' indicates interactive preview mode).",
                tip_buf,
            );
            render_accum_bar(
                ui,
                self.accum_count,
                self.settings.max_accum,
                self.total_samples,
                buf,
                tip_buf,
            );
            stat_row_desc(
                ui,
                "Bloom / TM",
                fmt_into(buf, format_args!("{:.2} / {:.2} ms", self.bloom_ms, self.tonemap_ms)),
                "GPU time spent on diffraction glow (bloom) and tone mapping HDR light into display colors each frame.",
                tip_buf,
            );
            stat_row_desc(
                ui,
                "Denoiser",
                fmt_into(buf, format_args!("{:.1} ms", self.denoise_ms)),
                "Time spent by the AI denoiser cleaning up path-tracing noise each frame.",
                tip_buf,
            );
            stat_row_desc(
                ui,
                "DLSS RR",
                match (self.dlss_rr_available, self.settings.dlss_rr_on) {
                    (true, true) => "Active",
                    (true, false) => "Available",
                    (false, _) => "Unavailable",
                },
                "NVIDIA DLSS Ray Reconstruction status: AI-powered upscaling and denoising for sharper real-time ray tracing.",
                tip_buf,
            );
            stat_row_desc(
                ui,
                "OptiX SER",
                match (self.ser_supported, self.settings.ser_on) {
                    (true, true) => "HW Active",
                    (true, false) => "HW Off",
                    (false, _) => "Unsupported",
                },
                "Hardware Shader Execution Reordering status: groups similar rays on RTX GPUs to speed up complex scenes.",
                tip_buf,
            );
        });

        collapsing_section(ui, "sec_gpu_hw", ic::CPU, "GPU & Hardware", |ui| {
            let UiScratch {
                label_buf: buf,
                tip_buf,
                ..
            } = &mut *self.scratch;
            render_gpu_hw_body(ui, self.telemetry, buf, tip_buf);
        });

        if self.settings.csg_mode != 0 {
            collapsing_section(ui, "sec_csg_stats", ic::CUBE, "CSG Scene & LOD", |ui| {
                let UiScratch {
                    label_buf: buf,
                    tip_buf,
                    ..
                } = &mut *self.scratch;
                render_csg_stats_body(ui, &self.csg_stats, buf, tip_buf);
            });

            if self.sim_telemetry.has_any() {
                if self.sim_telemetry.has_crowd() {
                    collapsing_section(
                        ui,
                        "sec_crowd_stats",
                        ic::USERS_THREE,
                        "Crowd & Locomotion",
                        |ui| {
                            let UiScratch {
                                label_buf: buf,
                                tip_buf,
                                ..
                            } = &mut *self.scratch;
                            render_crowd_stats_body(
                                ui,
                                &self.sim_telemetry,
                                self.timings,
                                self.rt_fps,
                                self.cuda_compute_ms,
                                buf,
                                tip_buf,
                            );
                        },
                    );
                }

                if self.sim_telemetry.has_traffic() {
                    collapsing_section(ui, "sec_traffic_stats", ic::CAR, "Traffic & ADAS", |ui| {
                        let UiScratch {
                            label_buf: buf,
                            tip_buf,
                            ..
                        } = &mut *self.scratch;
                        render_traffic_stats_body(
                            ui,
                            &self.sim_telemetry,
                            self.timings,
                            buf,
                            tip_buf,
                        );
                    });
                }
            }
        }

        collapsing_section(ui, "sec_perf", ic::CHART_LINE, "Performance", |ui| {
            let UiScratch {
                label_buf: buf,
                tip_buf,
                ..
            } = &mut *self.scratch;
            stat_row_desc(
                ui,
                "RT Cores",
                fmt_into(buf, format_args!("{:.2} ms (OptiX)", self.rt_ms)),
                "Time spent on dedicated RTX ray-tracing cores tracing light paths through the scene each frame.",
                tip_buf,
            );
            stat_row_desc(
                ui,
                "Tensor Cores",
                fmt_into(buf, format_args!("{:.2} ms (Den/DLSS)", self.tensor_cores_ms)),
                "Time spent on AI Tensor Cores running the OptiX denoiser or DLSS upscaling/reconstruction.",
                tip_buf,
            );
            stat_row_desc(
                ui,
                "CUDA Compute",
                fmt_into(buf, format_args!("{:.2} ms (IAS/FFT/TM)", self.cuda_compute_ms)),
                "Time spent on general GPU compute shaders for scene updates, simulations, ocean FFT, bloom, and tone mapping.",
                tip_buf,
            );
            render_perf_frame_split_bar(
                ui,
                self.rt_ms,
                self.tensor_cores_ms,
                self.cuda_compute_ms,
                buf,
                tip_buf,
            );
            ui.add_space(2.0);
            sparkline_row(
                ui,
                buf,
                tip_buf,
                SparklineMeta {
                    label: "RT Cores",
                    desc: "Recent frame-by-frame RT Core ray-tracing duration in milliseconds (lower is faster).",
                    unit: "ms",
                },
                &self.timings.rt_cores_ms,
                Color32::from_rgb(138, 173, 244),
            );
            sparkline_row(
                ui,
                buf,
                tip_buf,
                SparklineMeta {
                    label: "Tensor",
                    desc: "Recent frame-by-frame AI denoiser and DLSS Tensor Core duration in milliseconds.",
                    unit: "ms",
                },
                &self.timings.tensor_cores_ms,
                Color32::from_rgb(166, 209, 137),
            );
            sparkline_row(
                ui,
                buf,
                tip_buf,
                SparklineMeta {
                    label: "CUDA SM",
                    desc: "Recent frame-by-frame general CUDA compute duration for simulations, ocean FFT, bloom, and tone mapping.",
                    unit: "ms",
                },
                &self.timings.cuda_compute_ms,
                Color32::from_rgb(238, 212, 159),
            );
            sparkline_row(
                ui,
                buf,
                tip_buf,
                SparklineMeta {
                    label: "FPS",
                    desc: "Recent history of rendered frames per second (higher and steadier means smoother motion).",
                    unit: "",
                },
                &self.timings.fps,
                Color32::from_rgb(198, 160, 246),
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
        "sec_asset_inspector" => Color32::from_rgb(138, 173, 244),
        "sec_lod" => Color32::from_rgb(104, 192, 186),
        "sec_sky" => Color32::from_rgb(232, 178, 84),
        "sec_ocean" => Color32::from_rgb(68, 178, 212),
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
        "sec_view_possess" | "sec_cinema" => Color32::from_rgb(232, 148, 112),
        "sec_render_stats" => Color32::from_rgb(112, 172, 216),
        "sec_gpu_hw" => Color32::from_rgb(154, 144, 216),
        "sec_csg_stats" => Color32::from_rgb(92, 184, 224),
        "sec_crowd_stats" => Color32::from_rgb(166, 209, 137),
        "sec_traffic_stats" => Color32::from_rgb(232, 178, 84),
        "sec_perf" => Color32::from_rgb(112, 184, 156),
        "sec_flab_author" => Color32::from_rgb(92, 184, 224),
        "sec_flab_dynamics" => Color32::from_rgb(104, 192, 186),
        "sec_flab_contact" => Color32::from_rgb(216, 156, 78),
        "sec_flab_stats" => Color32::from_rgb(112, 184, 156),
        "sec_fplay_deck" => Color32::from_rgb(232, 148, 112),
        "sec_fplay_queue" => Color32::from_rgb(138, 164, 212),
        "sec_fplay_library" => Color32::from_rgb(166, 209, 137),
        _ => Color32::from_rgb(142, 149, 163),
    }
}

const SECTION_GROUPS: &[&[&str]] = &[
    // Fractal panel
    &["sec_presets", "sec_formula", "sec_transform", "sec_detail"],
    // Render & Floor panel
    &[
        "sec_csg",
        "sec_asset_inspector",
        "sec_lod",
        "sec_sky",
        "sec_ocean",
        "sec_dof",
        "sec_floor",
        "sec_scene",
        "sec_bloom",
        "sec_split",
        "sec_pt",
    ],
    // Camera & Stats panel
    &[
        "sec_view_possess",
        "sec_render_stats",
        "sec_gpu_hw",
        "sec_csg_stats",
        "sec_crowd_stats",
        "sec_traffic_stats",
        "sec_perf",
    ],
    // Flight Lab panel
    &[
        "sec_flab_author",
        "sec_flab_dynamics",
        "sec_flab_contact",
        "sec_flab_stats",
    ],
    // Flight Player panel
    &["sec_fplay_deck", "sec_fplay_queue", "sec_fplay_library"],
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
        "sec_view_possess"
            | "sec_cinema"
            | "sec_render_stats"
            | "sec_csg_stats"
            | "sec_crowd_stats"
            | "sec_traffic_stats"
            | "sec_perf"
            | "sec_presets"
            | "sec_formula"
            | "sec_csg"
            | "sec_asset_inspector"
            | "sec_lod"
            | "sec_sky"
            | "sec_ocean"
            | "sec_pt"
            | "sec_flab_author"
            | "sec_flab_dynamics"
            | "sec_fplay_deck"
            | "sec_fplay_queue"
            | "sec_fplay_library"
    )
}

pub(crate) fn collapsing_section(
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
    let indent_w = (total_w - ui.spacing().indent).max(60.0);
    state.show_body_indented(&title_resp, ui, |ui| {
        let body_w = ui.available_width().min(indent_w).max(60.0);
        allocate_clamped_ui(
            ui,
            egui::vec2(body_w, 0.0),
            Layout::top_down(Align::Min),
            |ui| {
                ui.set_min_width(body_w);
                ui.add_space(2.0);
                add_body(ui);
                ui.add_space(2.0);
            },
        );
    });
}

pub(crate) fn active_accent_tint(ui: &Ui) -> Color32 {
    if ui.visuals().dark_mode {
        Color32::from_rgb(38, 70, 110)
    } else {
        Color32::from_rgb(208, 226, 250)
    }
}

pub(crate) fn hdr_amber_tint(ui: &Ui) -> Color32 {
    if ui.visuals().dark_mode {
        Color32::from_rgb(106, 78, 30)
    } else {
        Color32::from_rgb(252, 232, 184)
    }
}

/// Compact cyclic toggle button: LMB steps forward (`+1`), RMB steps backward (`-1`).
pub(crate) fn cycle_toggle_btn(
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

/// Allocates a child `Ui` of `desired_size` with `layout` whose reported horizontal extent is
/// hard-clamped to `desired_size.x` and clipped horizontally so no overflowing child widget
/// (such as a `DragValue` using `TextWrapMode::Extend`) can expand the parent `Ui`'s
/// `max_rect.max.x` (`available_width()`).
pub(crate) fn allocate_clamped_ui<R>(
    ui: &mut Ui,
    desired_size: egui::Vec2,
    layout: Layout,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let avail = ui.available_rect_before_wrap();
    let h = if ui.layout().is_horizontal() {
        avail.height().max(desired_size.y)
    } else {
        desired_size.y
    };
    let child_rect = egui::Rect::from_min_size(avail.min, egui::vec2(desired_size.x, h));
    let mut child_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(child_rect)
            .layout(layout),
    );
    child_ui.set_max_width(desired_size.x);
    let cur_clip = child_ui.clip_rect();
    let horiz_clip = egui::Rect::from_min_max(
        egui::pos2(child_rect.min.x, cur_clip.min.y),
        egui::pos2(child_rect.max.x, cur_clip.max.y),
    );
    child_ui.shrink_clip_rect(horiz_clip);
    let ret = add_contents(&mut child_ui);
    let mut used = child_ui.min_rect();
    used.min.x = child_rect.min.x;
    used.max.x = child_rect.max.x;
    used.min.y = used.min.y.min(child_rect.min.y);
    used.max.y = used.max.y.max(child_rect.min.y + desired_size.y);
    drop(child_ui);
    ui.advance_cursor_after_rect(used);
    ret
}

/// Renders a compact `11.0 pt` monospace `DragValue` inside a hard-clamped `val_w` column with
/// `2.0 px` horizontal padding so values with unit suffixes (`"35.0 deg"`, `"12.0 m/s2"`,
/// `"20.0 m/s3"`) fit cleanly and every slider value box shares one exact columnar width.
pub(crate) fn slider_drag_value(ui: &mut Ui, val_w: f32, drag: egui::DragValue<'_>) -> egui::Response {
    allocate_clamped_ui(
        ui,
        egui::vec2(val_w, ROW_H),
        Layout::centered_and_justified(egui::Direction::LeftToRight),
        |ui| {
            let font = egui::FontId::monospace(11.0);
            let style = ui.style_mut();
            style.drag_value_text_style = egui::TextStyle::Monospace;
            style.override_text_style = Some(egui::TextStyle::Monospace);
            style.override_font_id = Some(font.clone());
            style.text_styles.insert(egui::TextStyle::Monospace, font);
            style.spacing.button_padding.x = 2.0;
            style.spacing.interact_size.x = val_w;
            ui.add(drag)
        },
    )
}

/// Computes adaptive `(label_w, rem_w)` column widths for a property row so labels shrink
/// gracefully on narrow dock panes without clipping control columns.
pub(crate) fn prop_row_widths(total_w: f32) -> (f32, f32) {
    let row_gap = 6.0;
    let safe_w = if total_w.is_finite() {
        total_w.max(130.0)
    } else {
        240.0
    };
    let label_w = LABEL_W.min((safe_w * 0.34).max(46.0)).floor();
    let rem_w = (safe_w - label_w - row_gap).max(76.0);
    (label_w, rem_w)
}

/// Computes adaptive `(track_w, val_w)` widths inside a property row's control column so
/// numeric `DragValue` boxes and the animation toggle stay visible with zero horizontal overflow.
pub(crate) fn slider_control_widths(rem_w: f32) -> (f32, f32) {
    let ctrl_gap = 4.0;
    let avail = (rem_w - ANIM_BTN_W - ctrl_gap * 2.0).max(52.0);
    let val_w = VAL_W.min((avail - 16.0).max(36.0));
    let track_w = (avail - val_w).max(16.0);
    (track_w, val_w)
}

pub(crate) fn equal_grid(
    ui: &mut Ui,
    cols: usize,
    count: usize,
    mut item_ui: impl FnMut(&mut Ui, usize, f32),
) {
    let gap = 4.0;
    let cols = cols.max(1);
    let total_w = ui.available_width().max(1.0);
    let cell_w = ((total_w - gap * (cols.saturating_sub(1) as f32)) / (cols as f32)).max(1.0);
    let mut idx = 0;
    while idx < count {
        allocate_clamped_ui(
            ui,
            egui::vec2(total_w, ROW_H + 2.0),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = gap;
                for _ in 0..cols {
                    if idx < count {
                        allocate_clamped_ui(
                            ui,
                            egui::vec2(cell_w, ROW_H + 2.0),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                                item_ui(ui, idx, cell_w);
                            },
                        );
                        idx += 1;
                    }
                }
            },
        );
    }
}

pub(crate) fn prop_row(ui: &mut Ui, label: &str, add_contents: impl FnOnce(&mut Ui)) {
    prop_row_labeled(ui, label, false, |ui, _| add_contents(ui));
}

/// Core property row helper that passes the left label's [`egui::Response`] into `add_contents`
/// so callers can bind one shared hover tooltip across both label and right-hand control/value.
fn prop_row_labeled(
    ui: &mut Ui,
    label: &str,
    suppress_clip_tip: bool,
    add_contents: impl FnOnce(&mut Ui, egui::Response),
) {
    let row_w = ui.available_width().max(1.0);
    let gap = 6.0;
    let (label_w, rem_w) = prop_row_widths(row_w);
    let rem_w = rem_w.min((row_w - label_w - gap).max(1.0));
    allocate_clamped_ui(
        ui,
        egui::vec2(row_w, ROW_H),
        Layout::left_to_right(Align::Center),
        |ui| {
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
            let label_resp = if !suppress_clip_tip && clipped && label_resp.hovered() {
                label_resp.on_hover_text(label)
            } else {
                label_resp
            };
            let ctrl_w = ui.available_width().min(rem_w).max(1.0);
            allocate_clamped_ui(
                ui,
                egui::vec2(ctrl_w, ROW_H),
                Layout::left_to_right(Align::Center),
                |ui| {
                    add_contents(ui, label_resp);
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

pub(crate) fn anim_toggle_btn_disabled(ui: &mut Ui) {
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
        slider_drag_value(
            ui,
            val_w,
            egui::DragValue::new(val)
                .range(range)
                .fixed_decimals(decimals)
                .speed(0.005),
        );
        anim_toggle_btn(ui, anim, key);
    });
}

pub(crate) fn prop_slider_f32(
    ui: &mut Ui,
    label: &str,
    val: &mut f32,
    range: RangeInclusive<f32>,
    decimals: usize,
    suffix: &str,
) {
    prop_slider_f32_tip(ui, label, val, range, decimals, suffix, "");
}

/// Like [`prop_slider_f32`] with a plain-language hover tooltip bound to the label, slider track,
/// and numeric [`egui::DragValue`] box.
fn prop_slider_f32_tip(
    ui: &mut Ui,
    label: &str,
    val: &mut f32,
    range: RangeInclusive<f32>,
    decimals: usize,
    suffix: &str,
    tip: &str,
) {
    let has_tip = !tip.is_empty();
    prop_row_labeled(ui, label, has_tip, |ui, label_resp| {
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
        let slider_resp = ui.add_sized([track_w, ROW_H], slider);
        let mut drag = egui::DragValue::new(val)
            .range(range)
            .fixed_decimals(decimals)
            .speed(drag_speed);
        if !suffix.is_empty() {
            drag = drag.suffix(suffix);
        }
        let drag_resp = slider_drag_value(ui, val_w, drag);
        if has_tip {
            label_resp.on_hover_text(tip);
            slider_resp.on_hover_text(tip);
            drag_resp.on_hover_text(tip);
        }
        anim_toggle_btn_disabled(ui);
    });
}

pub(crate) fn prop_slider_u32(ui: &mut Ui, label: &str, val: &mut u32, range: RangeInclusive<u32>, log: bool) {
    prop_row(ui, label, |ui| {
        let gap = 4.0;
        let (track_w, val_w) = slider_control_widths(ui.available_width());
        ui.spacing_mut().item_spacing.x = gap;
        ui.spacing_mut().slider_width = track_w;
        let slider = egui::Slider::new(val, range.clone())
            .show_value(false)
            .logarithmic(log);
        ui.add_sized([track_w, ROW_H], slider);
        slider_drag_value(ui, val_w, egui::DragValue::new(val).range(range).speed(1.0));
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
        for item in v.iter_mut() {
            slider_drag_value(
                ui,
                w,
                egui::DragValue::new(item)
                    .range(range.clone())
                    .speed(speed)
                    .fixed_decimals(2),
            );
        }
        anim_toggle_btn(ui, anim, key);
    });
}

pub(crate) fn prop_toggle(ui: &mut Ui, label: &str, val: &mut bool, text: &str) {
    prop_row(ui, label, |ui| {
        let gap = 4.0;
        let check_w = (ui.available_width() - ANIM_BTN_W - gap).max(24.0);
        ui.spacing_mut().item_spacing.x = gap;
        allocate_clamped_ui(
            ui,
            egui::vec2(check_w, ROW_H),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                ui.checkbox(val, text);
            },
        );
        anim_toggle_btn_disabled(ui);
    });
}

pub(crate) fn stat_row(ui: &mut Ui, label: &str, value: &str) {
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

/// Like [`stat_row`] with a concise plain-language hover tooltip active on both the label and
/// value cells (formatting the full value alongside the description only when hovered).
fn stat_row_desc(ui: &mut Ui, label: &str, value: &str, desc: &str, tip_buf: &mut String) {
    stat_row_tip(ui, label, value, tip_buf, |t| {
        t.push_str(desc);
        if !value.is_empty() {
            let _ = write!(t, "\nCurrent: {value}");
        }
    });
}

/// Like [`stat_row`] with a lazily formatted hover tooltip active on both the left label and the
/// right value cell (runs `format_tip` only while hovered so 60 Hz frames do zero work).
fn stat_row_tip(
    ui: &mut Ui,
    label: &str,
    value: &str,
    tip_buf: &mut String,
    format_tip: impl FnOnce(&mut String),
) {
    prop_row_labeled(ui, label, true, |ui, label_resp| {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let val_resp = ui.add(
                egui::Label::new(
                    RichText::new(value)
                        .size(11.5)
                        .color(ui.visuals().text_color()),
                )
                .truncate(),
            );
            let target = if val_resp.hovered() {
                Some(val_resp)
            } else if label_resp.hovered() {
                Some(label_resp)
            } else {
                None
            };
            if let Some(resp) = target {
                tip_buf.clear();
                format_tip(tip_buf);
                resp.on_hover_text(tip_buf.as_str());
            }
        });
    });
}

/// One weighted segment inside a [`stacked_bar_row`] horizontal proportion bar.
#[derive(Clone, Copy)]
struct BarSegment {
    count: u64,
    color: Color32,
}

/// Visual tone for a compact backend / simulation status pill in [`stat_badge_row_tip`].
#[derive(Clone, Copy)]
enum PillTone {
    Green,
    Amber,
    Muted,
}

/// Compact color-coded status badge rendered at the left of a telemetry value row.
#[derive(Clone, Copy)]
struct StatusPill<'a> {
    text: &'a str,
    tone: PillTone,
}

fn pill_colors(ui: &Ui, tone: PillTone) -> (Color32, Color32) {
    let dark = ui.visuals().dark_mode;
    match (tone, dark) {
        (PillTone::Green, true) => (
            Color32::from_rgb(176, 242, 198),
            Color32::from_rgb(24, 76, 50),
        ),
        (PillTone::Green, false) => (
            Color32::from_rgb(18, 92, 52),
            Color32::from_rgb(208, 242, 220),
        ),
        (PillTone::Amber, true) => (
            Color32::from_rgb(252, 224, 162),
            Color32::from_rgb(96, 66, 20),
        ),
        (PillTone::Amber, false) => (
            Color32::from_rgb(126, 76, 8),
            Color32::from_rgb(252, 232, 186),
        ),
        (PillTone::Muted, true) => (
            Color32::from_rgb(186, 196, 212),
            Color32::from_rgb(54, 60, 72),
        ),
        (PillTone::Muted, false) => (
            Color32::from_rgb(66, 74, 88),
            Color32::from_rgb(220, 224, 232),
        ),
    }
}

/// Renders a [`prop_row`] with a compact color-coded status pill (e.g. `CUDA v2`, `CPU SoA`) and
/// an optional right-aligned trailing metric readout, with a lazily formatted hover tooltip.
fn stat_badge_row_tip(
    ui: &mut Ui,
    label: &str,
    pill: StatusPill<'_>,
    trailing_text: &str,
    tip_buf: &mut String,
    format_tip: impl FnOnce(&mut String),
) {
    let (pill_fg, pill_bg) = pill_colors(ui, pill.tone);
    prop_row_labeled(ui, label, true, |ui, label_resp| {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 5.0;
            let mut hovered_resp = None;
            if !trailing_text.is_empty() {
                let resp = ui.add(
                    egui::Label::new(
                        RichText::new(trailing_text)
                            .monospace()
                            .size(11.0)
                            .color(ui.visuals().text_color()),
                    )
                    .truncate(),
                );
                if resp.hovered() {
                    hovered_resp = Some(resp);
                }
            }

            let font = egui::FontId::proportional(10.0);
            let galley = ui
                .painter()
                .layout_no_wrap(pill.text.to_owned(), font, pill_fg);
            let max_pill_w = ui.available_width().max(20.0);
            let pill_w = (galley.size().x + 10.0).min(max_pill_w);
            let pill_h = 15.0;
            let (rect, pill_resp) =
                ui.allocate_exact_size(egui::vec2(pill_w, pill_h), Sense::hover());
            if ui.is_rect_visible(rect) {
                let p = ui.painter();
                p.rect_filled(rect, 3.0, pill_bg);
                p.rect_stroke(
                    rect,
                    3.0,
                    egui::Stroke::new(1.0, pill_fg.gamma_multiply(0.35)),
                    egui::StrokeKind::Inside,
                );
                let text_pos = egui::pos2(
                    (rect.center().x - galley.size().x * 0.5).max(rect.left() + 3.0),
                    rect.center().y - galley.size().y * 0.5,
                );
                p.with_clip_rect(rect.intersect(ui.clip_rect()))
                    .galley(text_pos, galley, Color32::WHITE);
            }
            if pill_resp.hovered() {
                hovered_resp = Some(pill_resp);
            } else if hovered_resp.is_none() && label_resp.hovered() {
                hovered_resp = Some(label_resp);
            }
            if let Some(resp) = hovered_resp {
                tip_buf.clear();
                format_tip(tip_buf);
                resp.on_hover_text(tip_buf.as_str());
            }
        });
    });
}

/// Renders a compact (`12 px` tall), width-clamped horizontal stacked proportion bar inside a
/// standard [`prop_row`] with an optional right-aligned compact readout and a lazy hover tooltip.
fn stacked_bar_row(
    ui: &mut Ui,
    label: &str,
    segments: &[BarSegment],
    right_text: Option<&str>,
    tip_buf: &mut String,
    format_tip: impl FnOnce(&mut String),
) {
    prop_row_labeled(ui, label, true, |ui, label_resp| {
        let avail_w = ui.available_width().max(1.0);
        let gap = 5.0;
        let (bar_w, val_w) = if let Some(txt) = right_text {
            let desired_val_w: f32 = if txt.len() <= 5 { 40.0 } else { 52.0 };
            let vw = desired_val_w.min((avail_w * 0.44).max(30.0));
            let bw = (avail_w - vw - gap).max(24.0);
            (bw, vw)
        } else {
            (avail_w, 0.0)
        };
        ui.spacing_mut().item_spacing.x = gap;

        let bar_h = 12.0;
        let (bar_rect, bar_resp) =
            ui.allocate_exact_size(egui::vec2(bar_w, bar_h), Sense::hover());
        if ui.is_rect_visible(bar_rect) {
            let p = ui.painter();
            let track_bg = ui.visuals().extreme_bg_color;
            let border_col = ui
                .visuals()
                .widgets
                .noninteractive
                .bg_stroke
                .color
                .gamma_multiply(0.55);
            p.rect_filled(bar_rect, 3.0, track_bg);

            let total: u64 = segments.iter().map(|s| s.count).sum();
            let inner = bar_rect.shrink(1.0);
            if total > 0 && inner.width() > 2.0 {
                let total_f = total as f32;
                let nonzero_total = segments.iter().filter(|s| s.count > 0).count();
                let min_seg = 2.5_f32.min(inner.width() / (nonzero_total.max(1) as f32));
                let mut cursor_x = inner.left();
                let mut seen_nonzero = 0usize;

                for seg in segments {
                    if seg.count == 0 {
                        continue;
                    }
                    seen_nonzero += 1;
                    let is_first = seen_nonzero == 1;
                    let is_last = seen_nonzero == nonzero_total;
                    let rem_after = nonzero_total.saturating_sub(seen_nonzero);
                    let remaining_w = (inner.right() - cursor_x).max(0.0);
                    let seg_w = if is_last {
                        remaining_w
                    } else {
                        let reserve = min_seg * (rem_after as f32);
                        let max_w = (remaining_w - reserve).max(0.0);
                        ((seg.count as f32 / total_f) * inner.width())
                            .max(min_seg)
                            .min(max_w)
                    };
                    if seg_w > 0.25 {
                        if seg.color.a() > 0 {
                            let seg_rect = egui::Rect::from_min_max(
                                egui::pos2(cursor_x, inner.top()),
                                egui::pos2((cursor_x + seg_w).min(inner.right()), inner.bottom()),
                            );
                            let cr = egui::CornerRadius {
                                nw: if is_first { 2 } else { 0 },
                                sw: if is_first { 2 } else { 0 },
                                ne: if is_last { 2 } else { 0 },
                                se: if is_last { 2 } else { 0 },
                            };
                            p.rect_filled(seg_rect, cr, seg.color);
                            if !is_first {
                                p.line_segment(
                                    [
                                        egui::pos2(cursor_x, inner.top()),
                                        egui::pos2(cursor_x, inner.bottom()),
                                    ],
                                    egui::Stroke::new(1.0, track_bg.gamma_multiply(0.75)),
                                );
                            }
                        }
                        cursor_x = (cursor_x + seg_w).min(inner.right());
                    }
                }
            }

            p.rect_stroke(
                bar_rect,
                3.0,
                egui::Stroke::new(0.75, border_col),
                egui::StrokeKind::Inside,
            );
        }

        let mut hovered_resp = if bar_resp.hovered() {
            Some(bar_resp)
        } else {
            None
        };
        if let Some(txt) = right_text {
            let txt_resp = allocate_clamped_ui(
                ui,
                egui::vec2(val_w, ROW_H),
                Layout::right_to_left(Align::Center),
                |ui| {
                    ui.add(
                        egui::Label::new(
                            RichText::new(txt)
                                .monospace()
                                .size(10.5)
                                .color(ui.visuals().text_color()),
                        )
                        .truncate(),
                    )
                },
            );
            if hovered_resp.is_none() && txt_resp.hovered() {
                hovered_resp = Some(txt_resp);
            }
        }
        if hovered_resp.is_none() && label_resp.hovered() {
            hovered_resp = Some(label_resp);
        }
        if let Some(resp) = hovered_resp {
            tip_buf.clear();
            format_tip(tip_buf);
            resp.on_hover_text(tip_buf.as_str());
        }
    });
}

/// Renders the compact `Disk Cache` status badge (`Disk Hit` / `Mem Hit` / `Built`) and
/// `Clear Cache` action inside `CSG World`. Returns `true` when `Clear Cache` is clicked.
fn render_scene_cache_row(
    ui: &mut Ui,
    stats: SceneCacheStats,
    label_buf: &mut String,
    tip_buf: &mut String,
) -> bool {
    let (pill_text, pill_tone) = match stats.last_source {
        SceneLoadSource::DiskHit => ("Disk Hit", PillTone::Green),
        SceneLoadSource::MemoryHit => ("Mem Hit", PillTone::Green),
        SceneLoadSource::Built => ("Built", PillTone::Amber),
        SceneLoadSource::None => ("Idle", PillTone::Muted),
    };
    let mb = stats.total_bytes as f64 / (1024.0 * 1024.0);
    let summary = fmt_into(label_buf, format_args!("{} ({mb:.1} MB)", stats.entries));
    stat_badge_row_tip(
        ui,
        "Disk Cache",
        StatusPill {
            text: pill_text,
            tone: pill_tone,
        },
        summary,
        tip_buf,
        |t| {
            let _ = write!(
                t,
                "Compiled CSG scenes cached on disk with BLAKE3 integrity checks for fast startup.\nCached Scenes: {} ({mb:.2} MB) | Last Load: {}",
                stats.entries,
                stats.last_source.label()
            );
        },
    );
    let btn_w = ui.available_width().max(40.0);
    ui.add_enabled(
        stats.entries > 0 || stats.total_bytes > 0,
        egui::Button::new(format!("{}  Clear Disk Cache", ic::TRASH))
            .min_size(egui::vec2(btn_w, ROW_H + 2.0)),
    )
    .on_hover_text(
        "Delete cached CSG scene files from disk and memory so scenes rebuild fresh next time.",
    )
    .clicked()
}

fn asset_category_icon(cat: AssetCategory) -> &'static str {
    match cat {
        AssetCategory::Vehicles => ic::CAR,
        AssetCategory::Architecture => ic::BUILDINGS,
        AssetCategory::Foliage => ic::TREE,
        AssetCategory::Industrial => ic::FACTORY,
        AssetCategory::Infra => ic::LIGHTNING,
        AssetCategory::Street => ic::SIGNPOST,
        AssetCategory::Marine => ic::ANCHOR,
        AssetCategory::MarinaFurniture => ic::ARMCHAIR,
    }
}

fn asset_category_short(cat: AssetCategory) -> &'static str {
    match cat {
        AssetCategory::Vehicles => "Veh",
        AssetCategory::Architecture => "Arch",
        AssetCategory::Foliage => "Fol",
        AssetCategory::Industrial => "Ind",
        AssetCategory::Infra => "Infra",
        AssetCategory::Street => "Street",
        AssetCategory::Marine => "Marine",
        AssetCategory::MarinaFurniture => "Club",
    }
}

/// Renders the categorized Parametric Asset Inspector (`All` + 8 category chips, search box,
/// scrollable asset browser, metadata summary, preset strip, and vendored DCC Attribute Editor).
/// Returns `true` when the user selected a different asset variant this frame.
fn render_asset_inspector_body(
    ui: &mut Ui,
    asset_idx: &mut u32,
    asset_params: &mut AssetParamMap,
    browser: &mut AssetBrowserState,
    label_buf: &mut String,
    tip_buf: &mut String,
) -> bool {
    let prev_idx = *asset_idx;

    // 1. Category filter chips (`All` + 8 catalog categories in a compact 3x3 grid).
    const CHIPS: [(Option<AssetCategory>, &str, &str, &str); 9] = [
        (None, ic::CUBE, "All", "Show all 111 kit assets and the crowd showcase"),
        (
            Some(AssetCategory::Vehicles),
            ic::CAR,
            "Veh",
            "Vehicles (cars, trucks, buses, wheels)",
        ),
        (
            Some(AssetCategory::Architecture),
            ic::BUILDINGS,
            "Arch",
            "Architecture & Interiors",
        ),
        (
            Some(AssetCategory::Foliage),
            ic::TREE,
            "Fol",
            "Foliage (palms, trees, planters)",
        ),
        (
            Some(AssetCategory::Industrial),
            ic::FACTORY,
            "Ind",
            "Industrial (refinery towers, tanks, pipes)",
        ),
        (
            Some(AssetCategory::Infra),
            ic::LIGHTNING,
            "Infra",
            "Infrastructure (power, rail, fences)",
        ),
        (
            Some(AssetCategory::Street),
            ic::SIGNPOST,
            "Street",
            "Street & Work Zone furniture",
        ),
        (
            Some(AssetCategory::Marine),
            ic::ANCHOR,
            "Marine",
            "Marine & Coastal (yachts, piers, towers)",
        ),
        (
            Some(AssetCategory::MarinaFurniture),
            ic::ARMCHAIR,
            "Club",
            "Marina City Club & Balcony fixtures",
        ),
    ];
    equal_grid(ui, 3, CHIPS.len(), |ui, idx, w| {
        let Some(&(cat, icon, short, tip)) = CHIPS.get(idx) else {
            return;
        };
        let active = browser.category == cat;
        let mut btn = egui::Button::new(format!("{icon} {short}"))
            .min_size(egui::vec2(w, ROW_H + 2.0))
            .truncate();
        if active {
            btn = btn
                .fill(active_accent_tint(ui))
                .stroke(egui::Stroke::new(1.0, ui.visuals().selection.stroke.color));
        }
        if ui
            .add_sized([w, ROW_H + 2.0], btn)
            .on_hover_text(tip)
            .clicked()
        {
            browser.category = cat;
        }
    });

    ui.add_space(2.0);

    // 2. Search input + match count badge (width-clamped).
    let query_lc = browser.search.trim().to_ascii_lowercase();
    let mut matched: Vec<(u32, &'static str, &'static str, &'static str)> =
        Vec::with_capacity(KitAssetKind::ALL.len() + 1);
    for (i, &kind) in KitAssetKind::ALL.iter().enumerate() {
        if let Some(cat) = browser.category
            && kind.category() != cat
        {
            continue;
        }
        if !query_lc.is_empty()
            && !kind.label().to_ascii_lowercase().contains(&query_lc)
            && !kind.id().to_ascii_lowercase().contains(&query_lc)
            && !kind.category().label().to_ascii_lowercase().contains(&query_lc)
            && !asset_category_short(kind.category())
                .to_ascii_lowercase()
                .contains(&query_lc)
        {
            continue;
        }
        matched.push((
            i as u32,
            asset_category_icon(kind.category()),
            kind.label(),
            kind.category().label(),
        ));
    }
    let show_crowd = browser.category.is_none()
        && (query_lc.is_empty()
            || "actors: 4-archetype crowd showcase".contains(&query_lc));
    if show_crowd {
        matched.push((
            csg_scenes::ACTOR_SHOWCASE_VARIANT_IDX as u32,
            ic::USERS_THREE,
            "Actors: 4-Archetype Crowd Showcase",
            "Articulated Crowds",
        ));
    }

    let search_bar_w = ui.available_width().max(1.0);
    let search_h = ROW_H + 6.0;
    allocate_clamped_ui(
        ui,
        egui::vec2(search_bar_w, search_h),
        Layout::left_to_right(Align::Center),
        |ui| {
            let gap = 3.0;
            ui.spacing_mut().item_spacing.x = gap;
            let clear_w = if browser.search.is_empty() {
                0.0
            } else {
                20.0 + gap
            };
            let count_w = 42.0;
            let edit_w = (search_bar_w - clear_w - count_w - gap).max(48.0);
            ui.add_sized(
                [edit_w, search_h - 2.0],
                egui::TextEdit::singleline(&mut browser.search)
                    .hint_text(format!("{} Search assets...", ic::MAGNIFYING_GLASS)),
            )
            .on_hover_text("Filter procedural kit assets by name, id, or category.");
            if !browser.search.is_empty()
                && ui
                    .add_sized(
                        [20.0, search_h - 2.0],
                        egui::Button::new(RichText::new(ic::X).size(10.5))
                            .min_size(egui::vec2(20.0, search_h - 2.0)),
                    )
                    .on_hover_text("Clear asset search")
                    .clicked()
            {
                browser.search.clear();
            }
            let count_txt = fmt_into(
                label_buf,
                format_args!("{}/{}", matched.len(), csg_scenes::INSPECTOR_VARIANT_COUNT),
            );
            allocate_clamped_ui(
                ui,
                egui::vec2(count_w, search_h),
                Layout::right_to_left(Align::Center),
                |ui| {
                    ui.label(RichText::new(count_txt).monospace().size(10.5).weak());
                },
            );
        },
    );

    // 3. Scrollable categorized asset list (width-clamped).
    ui.add_space(2.0);
    let list_w = ui.available_width().max(1.0);
    let list_inner_w = (list_w - 14.0).max(80.0);
    egui::Frame::new()
        .fill(ui.visuals().extreme_bg_color)
        .corner_radius(3.0)
        .inner_margin(egui::Margin::same(2))
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("inspector_asset_browser_scroll")
                .max_height(132.0)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    allocate_clamped_ui(
                        ui,
                        egui::vec2(list_inner_w, 0.0),
                        Layout::top_down(Align::Min),
                        |ui| {
                            if matched.is_empty() {
                                ui.label(
                                    RichText::new("No assets match this filter.")
                                        .size(11.0)
                                        .weak(),
                                );
                            } else {
                                ui.spacing_mut().item_spacing.y = 1.0;
                                for &(var_idx, icon, label, cat_label) in &matched {
                                    let selected = *asset_idx == var_idx;
                                    let mut btn = egui::Button::new(format!("{icon}  {label}"))
                                        .min_size(egui::vec2(list_inner_w, ROW_H + 4.0))
                                        .truncate();
                                    if selected {
                                        btn = btn.fill(active_accent_tint(ui)).stroke(
                                            egui::Stroke::new(
                                                1.0,
                                                ui.visuals().selection.stroke.color,
                                            ),
                                        );
                                    }
                                    let resp = ui
                                        .add_sized([list_inner_w, ROW_H + 4.0], btn)
                                        .on_hover_text(format!("{label} ({cat_label})"));
                                    if resp.clicked() {
                                        *asset_idx = var_idx;
                                    }
                                }
                            }
                        },
                    );
                });
        });

    let switched = *asset_idx != prev_idx;

    // 4. Selected asset metadata + presets + vendored DCC Attribute Editor (cached per kind so
    //    60 Hz UI frames spend zero CPU composing CSG blueprints).
    ui.add_space(4.0);
    if let Some(kind) = csg_scenes::inspector_asset_kind(*asset_idx) {
        if switched {
            *asset_params = kind.default_params(0);
        }
        browser.ensure_cached(kind);
        let Some(meta) = browser.cached_meta.as_ref() else {
            return switched;
        };
        let desc = &meta.descriptor;
        stat_row_desc(
            ui,
            "Category",
            desc.category.label(),
            "Procedural kit family owning this asset blueprint.",
            tip_buf,
        );
        stat_row_desc(
            ui,
            "GAS Class",
            fmt_into(label_buf, format_args!("{:?}", desc.gas_class)),
            "OptiX acceleration-structure budget bucket assigned to this asset.",
            tip_buf,
        );
        stat_row_desc(
            ui,
            "Ports",
            fmt_into(
                label_buf,
                format_args!(
                    "{} sockets | {} mounts",
                    desc.socket_count, desc.camera_mount_count
                ),
            ),
            "Typed connection sockets for snapping and Possess Body camera mounts on this asset.",
            tip_buf,
        );

        let presets = &meta.presets;
        if !presets.is_empty() {
            ui.add_space(2.0);
            ui.label(RichText::new("Presets").size(10.5).weak());
            let cols = if presets.len() <= 1 { 1 } else { 2 };
            equal_grid(ui, cols, presets.len(), |ui, idx, w| {
                let Some(preset) = presets.get(idx) else {
                    return;
                };
                let is_cur = *asset_params == preset.values;
                let mut btn = egui::Button::new(format!("{}  {}", ic::SPARKLE, preset.name))
                    .min_size(egui::vec2(w, ROW_H + 2.0))
                    .truncate();
                if is_cur {
                    btn = btn
                        .fill(active_accent_tint(ui))
                        .stroke(egui::Stroke::new(1.0, ui.visuals().selection.stroke.color));
                }
                if ui
                    .add_sized([w, ROW_H + 2.0], btn)
                    .on_hover_text(preset.description)
                    .clicked()
                {
                    *asset_params = preset.values.clone();
                }
            });
        }

        ui.add_space(3.0);
        let ed_resp = render_attr_editor(
            ui,
            "inspector_attr_ed",
            &meta.schema,
            asset_params,
            &mut browser.attr_editor,
        );
        if ed_resp.changed {
            ui.ctx().request_repaint();
        }

        ui.add_space(2.0);
        let reset_w = ui.available_width().max(40.0);
        let reset_lbl = if ed_resp.modified_count > 0 {
            fmt_into(
                label_buf,
                format_args!(
                    "{}  Reset Defaults ({} mod)",
                    ic::ARROW_COUNTER_CLOCKWISE,
                    ed_resp.modified_count
                ),
            )
        } else {
            fmt_into(
                label_buf,
                format_args!("{}  Reset Defaults", ic::ARROW_COUNTER_CLOCKWISE),
            )
        };
        if ui
            .add_enabled(
                ed_resp.modified_count > 0,
                egui::Button::new(reset_lbl).min_size(egui::vec2(reset_w, ROW_H + 2.0)),
            )
            .on_hover_text("Reset every parameter of this asset back to its catalog defaults.")
            .clicked()
        {
            *asset_params = kind.default_params(0);
        }
    } else {
        ui.label(
            RichText::new(
                "4-Archetype Crowd Showcase active (tune crowd settings in CSG World above).",
            )
            .size(11.0)
            .weak(),
        );
    }

    switched
}

/// Renders the `Ocean Water` section controls (`Mode`, `Wind Speed`, `Wind Dir`, `Choppiness`,
/// `Swell`, `Foam`, `Water Clarity`, plus Reset) using SSOT width-clamped helpers and plain-language
/// hover tooltips on every control.
fn render_ocean_water_controls(ui: &mut Ui, ocean: &mut OceanParams) {
    let mode_tip = "Wave solver: cuFFT Spectral runs a 3-cascade GPU FFT ocean; Analytical uses a fast procedural wave sum.";
    prop_row_labeled(ui, "Mode", true, |ui, label_resp| {
        label_resp.on_hover_text(mode_tip);
        let gap = 4.0;
        let combo_w = (ui.available_width() - ANIM_BTN_W - gap).max(32.0);
        ui.spacing_mut().item_spacing.x = gap;
        egui::ComboBox::from_id_salt("ocean_mode_combo")
            .width(combo_w)
            .truncate()
            .selected_text(ocean.mode.label())
            .show_ui(ui, |ui| {
                for m in OceanMode::ALL {
                    ui.selectable_value(&mut ocean.mode, m, m.label());
                }
            })
            .response
            .on_hover_text(mode_tip);
        anim_toggle_btn_disabled(ui);
    });
    prop_slider_f32_tip(
        ui,
        "Wind Speed",
        &mut ocean.wind_speed,
        1.0..=25.0,
        1,
        " m/s",
        "Surface wind speed driving wave height and energy (higher winds build larger, steeper swells).",
    );
    prop_slider_f32_tip(
        ui,
        "Wind Dir",
        &mut ocean.wind_dir_deg,
        0.0..=360.0,
        0,
        " deg",
        "Compass direction that wind and ocean waves travel toward in degrees.",
    );
    prop_slider_f32_tip(
        ui,
        "Choppiness",
        &mut ocean.choppiness,
        0.0..=2.0,
        2,
        "",
        "Sharpness of wave peaks from horizontal crest pinching (0 is gentle rolling swells, 2 is steep choppy peaks).",
    );
    prop_slider_f32_tip(
        ui,
        "Swell",
        &mut ocean.swell,
        0.0..=1.0,
        2,
        "",
        "Strength of long-distance Pacific swells blended with local wind ripples.",
    );
    prop_slider_f32_tip(
        ui,
        "Foam",
        &mut ocean.foam,
        0.0..=2.0,
        2,
        "",
        "Intensity of whitecaps on breaking wave crests and aerated shoreline contact foam.",
    );
    prop_slider_f32_tip(
        ui,
        "Water Clarity",
        &mut ocean.clarity,
        0.25..=3.0,
        2,
        "",
        "How far sunlight penetrates underwater before absorbing into deep navy (higher reveals more seabed and turquoise shallows).",
    );
    ui.add_space(2.0);
    let btn_w = ui.available_width().max(40.0);
    if ui
        .add_sized(
            [btn_w, ROW_H + 2.0],
            egui::Button::new(format!("{}  Reset Ocean Defaults", ic::ARROWS_CLOCKWISE)),
        )
        .on_hover_text("Restore default Pacific Ocean wind, swell, foam, and clarity settings.")
        .clicked()
    {
        *ocean = OceanParams::default();
    }
}

/// Renders the `Render & Frame -> Accum` progress bar (green when converged, blue while
/// accumulating, muted full-idle when `max_accum == 0` continuous) with a lazy hover tooltip.
fn render_accum_bar(
    ui: &mut Ui,
    accum_count: u32,
    max_accum: u32,
    total_samples: u32,
    label_buf: &mut String,
    tip_buf: &mut String,
) {
    if max_accum > 0 {
        let cur = accum_count.min(max_accum);
        let rem = max_accum.saturating_sub(cur);
        let converged = accum_count >= max_accum;
        let fill_col = if converged {
            Color32::from_rgb(122, 206, 138)
        } else {
            Color32::from_rgb(112, 172, 216)
        };
        let segs = [
            BarSegment {
                count: u64::from(cur),
                color: fill_col,
            },
            BarSegment {
                count: u64::from(rem),
                color: Color32::TRANSPARENT,
            },
        ];
        let pct = (cur as f32 / max_accum as f32) * 100.0;
        let txt = fmt_into(label_buf, format_args!("{accum_count}/{max_accum}"));
        stacked_bar_row(ui, "Accum", &segs, Some(txt), tip_buf, |t| {
            let status = if converged { "Converged" } else { "Accumulating" };
            let _ = write!(
                t,
                "Progressive path-tracing samples blended together to clear noise (green means converged).\nProgress: {accum_count} / {max_accum} ({pct:.0}%) | Total: {total_samples} spp | Status: {status}"
            );
        });
    } else {
        let segs = [BarSegment {
            count: 1,
            color: Color32::from_rgb(104, 114, 132),
        }];
        let txt = fmt_into(label_buf, format_args!("{total_samples} spp"));
        stacked_bar_row(ui, "Accum", &segs, Some(txt), tip_buf, |t| {
            let _ = write!(
                t,
                "Continuous path-tracing accumulation without a fixed sample cap (max_accum = 0).\nFrames: {accum_count} | Total Samples: {total_samples} spp"
            );
        });
    }
}

/// Renders the `Performance -> Frame Split` stacked bar (`RT Cores | Tensor | CUDA`) with total ms
/// readout and exact per-engine percentage + 60/30 FPS target budget tooltip.
fn render_perf_frame_split_bar(
    ui: &mut Ui,
    rt_ms: f32,
    tensor_cores_ms: f32,
    cuda_compute_ms: f32,
    label_buf: &mut String,
    tip_buf: &mut String,
) {
    let rt = if rt_ms.is_finite() && rt_ms > 0.0 {
        rt_ms
    } else {
        0.0
    };
    let tensor = if tensor_cores_ms.is_finite() && tensor_cores_ms > 0.0 {
        tensor_cores_ms
    } else {
        0.0
    };
    let cuda = if cuda_compute_ms.is_finite() && cuda_compute_ms > 0.0 {
        cuda_compute_ms
    } else {
        0.0
    };
    let tot = rt + tensor + cuda;
    let segs = [
        BarSegment {
            count: (rt * 1000.0).round() as u64,
            color: Color32::from_rgb(138, 173, 244),
        },
        BarSegment {
            count: (tensor * 1000.0).round() as u64,
            color: Color32::from_rgb(166, 209, 137),
        },
        BarSegment {
            count: (cuda * 1000.0).round() as u64,
            color: Color32::from_rgb(238, 212, 159),
        },
    ];
    let txt = fmt_into(label_buf, format_args!("{tot:.1} ms"));
    stacked_bar_row(ui, "Frame Split", &segs, Some(txt), tip_buf, |t| {
        let denom = tot.max(1e-4);
        let rt_pct = (rt / denom) * 100.0;
        let tensor_pct = (tensor / denom) * 100.0;
        let cuda_pct = (cuda / denom) * 100.0;
        let _ = write!(
            t,
            "Visual share of GPU frame time across RT Cores, Tensor Cores, and CUDA Compute to spot the bottleneck.\nTotal: {tot:.2} ms | RT: {rt:.2} ms ({rt_pct:.1}%) | Tensor: {tensor:.2} ms ({tensor_pct:.1}%) | CUDA: {cuda:.2} ms ({cuda_pct:.1}%)\nTargets: 60 FPS (16.7 ms) | 30 FPS (33.3 ms)"
        );
    });
}

/// Renders the `GPU & Hardware` section body (`GPU`, `VRAM` + gauge bar, `Clocks`, `GPU Load`,
/// `GPU Temp`, `Power` live gauge bars, `CUPTI`, `NGX`).
fn render_gpu_hw_body(
    ui: &mut Ui,
    telemetry: &GpuTelemetry,
    label_buf: &mut String,
    tip_buf: &mut String,
) {
    stat_row_desc(
        ui,
        "GPU",
        &telemetry.gpu_name,
        "Active NVIDIA graphics card rendering this scene (via NVML hardware monitoring).",
        tip_buf,
    );
    stat_row_desc(
        ui,
        "VRAM",
        fmt_into(
            label_buf,
            format_args!("{} / {} MiB", telemetry.vram_used_mib, telemetry.vram_total_mib),
        ),
        "Video memory currently used versus total capacity on your graphics card.",
        tip_buf,
    );
    if telemetry.vram_total_mib > 0 {
        let used = telemetry.vram_used_mib.min(telemetry.vram_total_mib);
        let free = telemetry.vram_total_mib.saturating_sub(used);
        let pct = (used as f32 / telemetry.vram_total_mib as f32) * 100.0;
        let gauge_col = if pct > 90.0 {
            Color32::from_rgb(236, 78, 86)
        } else if pct > 75.0 {
            Color32::from_rgb(242, 184, 72)
        } else {
            Color32::from_rgb(122, 206, 138)
        };
        let segs = [
            BarSegment {
                count: used,
                color: gauge_col,
            },
            BarSegment {
                count: free,
                color: Color32::TRANSPARENT,
            },
        ];
        let pct_txt = fmt_into(label_buf, format_args!("{pct:.0}%"));
        stacked_bar_row(ui, "VRAM Use", &segs, Some(pct_txt), tip_buf, |t| {
            let _ = write!(
                t,
                "Percentage of GPU memory in use (turns amber above 75% and red above 90% when nearing capacity).\nUsed: {used} / {} MiB ({pct:.1}%) | Free: {free} MiB",
                telemetry.vram_total_mib
            );
        });
    }
    stat_row_desc(
        ui,
        "Clocks",
        fmt_into(
            label_buf,
            format_args!("{} / {} MHz", telemetry.gpu_clock_mhz, telemetry.mem_clock_mhz),
        ),
        "Live GPU core and video memory clock speeds in MHz (boosts automatically under render load).",
        tip_buf,
    );

    let util = telemetry.util_gpu.min(100);
    let load_col = if util > 95 {
        Color32::from_rgb(236, 78, 86)
    } else if util > 80 {
        Color32::from_rgb(242, 184, 72)
    } else {
        Color32::from_rgb(122, 206, 138)
    };
    let load_segs = [
        BarSegment {
            count: u64::from(util),
            color: load_col,
        },
        BarSegment {
            count: u64::from(100 - util),
            color: Color32::TRANSPARENT,
        },
    ];
    let load_txt = fmt_into(label_buf, format_args!("{util}%"));
    stacked_bar_row(ui, "GPU Load", &load_segs, Some(load_txt), tip_buf, |t| {
        let _ = write!(
            t,
            "Percentage of time the GPU is actively executing work (turns amber above 80% and red above 95%).\nUtilization: {util}% (NVML)"
        );
    });

    let temp = telemetry.temp_c;
    let temp_filled = temp.saturating_sub(25).min(70);
    let temp_col = if temp >= 85 {
        Color32::from_rgb(236, 78, 86)
    } else if temp >= 75 {
        Color32::from_rgb(242, 184, 72)
    } else {
        Color32::from_rgb(112, 198, 168)
    };
    let temp_segs = [
        BarSegment {
            count: u64::from(temp_filled),
            color: temp_col,
        },
        BarSegment {
            count: u64::from(70 - temp_filled),
            color: Color32::TRANSPARENT,
        },
    ];
    let temp_txt = fmt_into(label_buf, format_args!("{temp} C"));
    stacked_bar_row(ui, "GPU Temp", &temp_segs, Some(temp_txt), tip_buf, |t| {
        let _ = write!(
            t,
            "Current graphics chip temperature (cool below 75 C, warm at 75-84 C, hot at 85 C and above).\nDie Temp: {temp} C (bar scale 25..95 C)"
        );
    });

    let power_w = if telemetry.power_w.is_finite() && telemetry.power_w > 0.0 {
        telemetry.power_w
    } else {
        0.0
    };
    let power_ceil = power_w.max(350.0);
    let power_pct = (power_w / power_ceil) * 100.0;
    let power_col = if power_pct > 90.0 {
        Color32::from_rgb(236, 78, 86)
    } else if power_pct > 75.0 {
        Color32::from_rgb(242, 184, 72)
    } else {
        Color32::from_rgb(112, 182, 224)
    };
    let p_filled = (power_w * 10.0).round() as u64;
    let p_total = (power_ceil * 10.0).round() as u64;
    let power_segs = [
        BarSegment {
            count: p_filled,
            color: power_col,
        },
        BarSegment {
            count: p_total.saturating_sub(p_filled),
            color: Color32::TRANSPARENT,
        },
    ];
    let power_txt = fmt_into(label_buf, format_args!("{power_w:.0} W"));
    stacked_bar_row(ui, "Power", &power_segs, Some(power_txt), tip_buf, |t| {
        let _ = write!(
            t,
            "Real-time electrical power drawn by the graphics card in Watts.\nDraw: {power_w:.1} W ({power_pct:.0}% of {power_ceil:.0} W reference scale)"
        );
    });

    stat_row_desc(
        ui,
        "CUPTI",
        &telemetry.cupti_status,
        "CUDA Profiling Tools Interface status used to measure low-level GPU kernel timing.",
        tip_buf,
    );
    stat_row_desc(
        ui,
        "NGX",
        &telemetry.ngx_status,
        "NVIDIA NGX runtime status required for DLSS Super Resolution and Ray Reconstruction.",
        tip_buf,
    );
}

/// Renders the `CSG Scene & LOD` section body (`CSG Inst`, `CSG LOD`, live LOD & SBT Path stacked
/// bars, `CSG DAG`, `CSG Aux`, `CSG VRAM`).
fn render_csg_stats_body(
    ui: &mut Ui,
    stats: &CsgStats,
    label_buf: &mut String,
    tip_buf: &mut String,
) {
    stat_row_desc(
        ui,
        "CSG Inst",
        fmt_into(
            label_buf,
            format_args!("{} / {}", stats.visible_instances, stats.total_instances),
        ),
        "Procedural scene objects currently visible to rays out of all objects in the world.",
        tip_buf,
    );
    stat_row_desc(
        ui,
        "CSG LOD",
        fmt_into(
            label_buf,
            format_args!(
                "L0:{} L1:{} L2:{} C:{}",
                stats.lod0_count, stats.lod1_count, stats.lod2_count, stats.culled_count
            ),
        ),
        "Breakdown of objects by Level of Detail: L0 full detail, L1 medium, L2 coarse far silhouette, C culled out of view.",
        tip_buf,
    );
    let lod_segs = [
        BarSegment {
            count: u64::from(stats.lod0_count),
            color: Color32::from_rgb(122, 206, 138),
        },
        BarSegment {
            count: u64::from(stats.lod1_count),
            color: Color32::from_rgb(104, 182, 238),
        },
        BarSegment {
            count: u64::from(stats.lod2_count),
            color: Color32::from_rgb(236, 180, 86),
        },
        BarSegment {
            count: u64::from(stats.culled_count),
            color: Color32::from_rgb(108, 114, 128),
        },
    ];
    let lod_total = u64::from(stats.lod0_count)
        + u64::from(stats.lod1_count)
        + u64::from(stats.lod2_count)
        + u64::from(stats.culled_count);
    let vis_pct = if lod_total > 0 {
        let vis = u64::from(stats.lod0_count)
            + u64::from(stats.lod1_count)
            + u64::from(stats.lod2_count);
        (vis as f32 / lod_total as f32) * 100.0
    } else {
        0.0
    };
    let vis_txt = fmt_into(label_buf, format_args!("{vis_pct:.0}%"));
    stacked_bar_row(ui, "LOD Mix", &lod_segs, Some(vis_txt), tip_buf, |t| {
        let _ = write!(
            t,
            "Visual share of full, medium, coarse, and culled objects; shifts dynamically as you move the camera.\nLOD0: {} | LOD1: {} | LOD2: {} | Culled: {} ({vis_pct:.1}% visible of {lod_total})",
            stats.lod0_count, stats.lod1_count, stats.lod2_count, stats.culled_count
        );
    });

    let [p_single, p_pure, p_first, p_rpn] = stats.path_instances;
    let path_total =
        u64::from(p_single) + u64::from(p_pure) + u64::from(p_first) + u64::from(p_rpn);
    if path_total > 0 {
        let fast_pct = ((u64::from(p_single) + u64::from(p_pure)) as f32 / path_total as f32) * 100.0;
        let path_segs = [
            BarSegment {
                count: u64::from(p_single),
                color: Color32::from_rgb(122, 206, 138),
            },
            BarSegment {
                count: u64::from(p_pure),
                color: Color32::from_rgb(104, 182, 238),
            },
            BarSegment {
                count: u64::from(p_first),
                color: Color32::from_rgb(236, 180, 86),
            },
            BarSegment {
                count: u64::from(p_rpn),
                color: Color32::from_rgb(224, 114, 126),
            },
        ];
        let fast_txt = fmt_into(label_buf, format_args!("{fast_pct:.0}%"));
        stacked_bar_row(ui, "SBT Paths", &path_segs, Some(fast_txt), tip_buf, |t| {
            let denom = path_total as f32;
            let _ = write!(
                t,
                "Share of visible objects using fast single-shape/union ray paths versus full CSG tree evaluation.\nFast Path: {fast_pct:.1}% of {path_total} | Single (SBT 10): {p_single} ({:.1}%) | Pure Union: {p_pure} ({:.1}%)\nExact Union (SBT 12): {p_first} ({:.1}%) | General RPN (SBT 8): {p_rpn} ({:.1}%)",
                (p_single as f32 / denom) * 100.0,
                (p_pure as f32 / denom) * 100.0,
                (p_first as f32 / denom) * 100.0,
                (p_rpn as f32 / denom) * 100.0
            );
        });
    }

    stat_row_desc(
        ui,
        "CSG DAG",
        fmt_into(
            label_buf,
            format_args!("{} GAS / {} prm", stats.compounds, stats.primitives),
        ),
        "Unique procedural shape templates (GAS) and analytical primitives making up the world with zero triangle meshes.",
        tip_buf,
    );
    stat_row_desc(
        ui,
        "CSG Aux",
        fmt_into(
            label_buf,
            format_args!("{:.1} KiB", stats.aux_planes_bytes as f64 / 1024.0),
        ),
        "GPU memory used by auxiliary terrain elevation, road tiles, and clipping planes.",
        tip_buf,
    );
    stat_row_desc(
        ui,
        "CSG VRAM",
        fmt_into(
            label_buf,
            format_args!("{:.1} KiB", stats.gpu_bytes as f64 / 1024.0),
        ),
        "Total GPU memory holding all procedural scene geometry buffers for OptiX ray tracing.",
        tip_buf,
    );
}

/// Renders the `Crowd & Locomotion` section body: backend status pill, actor counts, archetype
/// stacked bar, rig/locomotion/VRAM stats, `Sim Share` CUDA Compute bar, and live `Crowd ms` /
/// `kBones/s` sparklines.
fn render_crowd_stats_body(
    ui: &mut Ui,
    sim: &SimTelemetry,
    timings: &FrameTimings,
    rt_fps: f32,
    cuda_compute_ms: f32,
    label_buf: &mut String,
    tip_buf: &mut String,
) {
    let (pill_text, pill_tone, show_ms) = match sim.crowd_backend {
        ActiveCrowdBackend::Cuda => {
            let ver = sim.crowd_plan_version.max(2);
            let tag = if ver == 2 { "CUDA v2" } else { "CUDA" };
            (tag, PillTone::Green, true)
        }
        ActiveCrowdBackend::Cpu => ("CPU", PillTone::Amber, true),
        ActiveCrowdBackend::None => {
            if sim.pose_frozen {
                ("Frozen", PillTone::Muted, false)
            } else {
                ("Idle", PillTone::Muted, false)
            }
        }
    };
    let trailing = if show_ms {
        fmt_into(label_buf, format_args!("{:.2} ms", sim.crowd_step_ms))
    } else {
        ""
    };
    stat_badge_row_tip(
        ui,
        "Crowd Sim",
        StatusPill {
            text: pill_text,
            tone: pill_tone,
        },
        trailing,
        tip_buf,
        |t| {
            let math = if sim.crowd_strict_f32 {
                "StrictF32"
            } else {
                "Fast"
            };
            let _ = write!(
                t,
                "Where walking pedestrians and animals are simulated and how long each step takes.\nBackend: {} | Step: {:.2} ms",
                sim.crowd_backend.label(),
                sim.crowd_step_ms
            );
            if sim.crowd_backend == ActiveCrowdBackend::Cuda {
                let _ = write!(
                    t,
                    "\nPlanKind::Crowd v{} ({} kernels + scatter bridge, NVRTC {math})",
                    sim.crowd_plan_version.max(2),
                    sim.crowd_kernel_count
                );
                if sim.crowd_gpu_fault == 0 {
                    t.push_str("\nGPU Fault: None (OK)");
                } else {
                    let _ = write!(t, "\nGPU Fault: 0x{:08x}", sim.crowd_gpu_fault);
                }
            }
        },
    );

    sim.write_archetypes_compact(label_buf);
    stat_row_tip(ui, "Actors", label_buf.as_str(), tip_buf, |t| {
        let [hero, ped, sign, pet] = sim.crowd_archetypes;
        let _ = write!(
            t,
            "Total animated characters in the scene broken down by character type.\nTotal: {} | Hero: {hero} | Ped: {ped} | Sign: {sign} | Pet: {pet}\nWalkways: {} | Obstacles: {}",
            sim.crowd_actors, sim.crowd_walkways, sim.crowd_obstacles
        );
    });

    let [hero, ped, sign, pet] = sim.crowd_archetypes;
    let arch_segs = [
        BarSegment {
            count: u64::from(hero),
            color: Color32::from_rgb(198, 148, 246),
        },
        BarSegment {
            count: u64::from(ped),
            color: Color32::from_rgb(112, 186, 238),
        },
        BarSegment {
            count: u64::from(sign),
            color: Color32::from_rgb(238, 192, 92),
        },
        BarSegment {
            count: u64::from(pet),
            color: Color32::from_rgb(138, 214, 148),
        },
    ];
    let total_txt = fmt_into(label_buf, format_args!("{}", sim.crowd_actors));
    stacked_bar_row(ui, "Archetypes", &arch_segs, Some(total_txt), tip_buf, |t| {
        let _ = write!(
            t,
            "Visual mix of character types currently active in the crowd.\nHero: {hero} | Pedestrian: {ped} | Stickman: {sign} | Pet: {pet} (Total: {})",
            sim.crowd_actors
        );
    });

    let rig_val = fmt_into(
        label_buf,
        format_args!("{} b / {} DOF", sim.crowd_bones, sim.crowd_dofs),
    );
    stat_row_tip(ui, "Rig / Bones", rig_val, tip_buf, |t| {
        let bps = sim.bones_per_sec(rt_fps);
        let _ = write!(
            t,
            "Total articulated skeleton bones and motion joints updated each frame across all characters.\nBones: {} | World Joints: {} | Skeleton Edges: {} | DOFs: {}",
            sim.crowd_bones, sim.crowd_joints, sim.crowd_skeleton_edges, sim.crowd_dofs
        );
        if bps >= 1000.0 {
            let _ = write!(t, "\nThroughput: {:.1}k bones/s", bps / 1000.0);
        } else if bps > 0.0 {
            let _ = write!(t, "\nThroughput: {:.0} bones/s", bps);
        } else {
            t.push_str("\nThroughput: 0 bones/s (paused/frozen)");
        }
    });

    let loco_val = if sim.pose_frozen {
        fmt_into(
            label_buf,
            format_args!("Frozen | {} miss", sim.crowd_ground_misses),
        )
    } else if sim.gpu_camera_active {
        fmt_into(
            label_buf,
            format_args!("{:.2}x | GPU Cam", sim.sim_rate),
        )
    } else {
        fmt_into(
            label_buf,
            format_args!("{:.2}x | {} miss", sim.sim_rate, sim.crowd_ground_misses),
        )
    };
    stat_row_tip(ui, "Locomotion", loco_val, tip_buf, |t| {
        let _ = write!(
            t,
            "Animation playback speed, freeze state, terrain ground-check status, and GPU follow-camera state.\nState: {} (rate {:.2}x) | Crowd LOD: ",
            if sim.pose_frozen { "Frozen" } else { "Active" },
            sim.sim_rate
        );
        sim.append_lod_summary(t);
        let _ = write!(
            t,
            "\nGround Probe Misses (12 B async DMA): {} | GPU Camera Rig: {}",
            sim.crowd_ground_misses,
            if sim.gpu_camera_active {
                "Active (VRAM zero-copy)"
            } else {
                "Idle"
            }
        );
    });

    if sim.crowd_backend == ActiveCrowdBackend::Cuda && sim.crowd_gpu_bytes > 0 {
        SimTelemetry::write_bytes_compact(sim.crowd_gpu_bytes, label_buf);
        let _ = write!(label_buf, " ({} rbld)", sim.crowd_ias_rebuilds);
        stat_row_tip(ui, "Crowd VRAM", label_buf.as_str(), tip_buf, |t| {
            t.push_str("GPU memory reserved for crowd simulation buffers and full scene rebuilds after large movement.\nPlan + Scatter Bridge VRAM: ");
            SimTelemetry::append_bytes_compact(sim.crowd_gpu_bytes, t);
            let _ = write!(
                t,
                "\nAmortized Full IAS Rebuilds (>8m disp): {}",
                sim.crowd_ias_rebuilds
            );
        });
    }

    let crowd_ms = if sim.crowd_step_ms.is_finite() && sim.crowd_step_ms > 0.0 {
        sim.crowd_step_ms
    } else {
        0.0
    };
    let traffic_ms = if sim.traffic_step_ms.is_finite() && sim.traffic_step_ms > 0.0 {
        sim.traffic_step_ms
    } else {
        0.0
    };
    let cuda_ms = if cuda_compute_ms.is_finite() && cuda_compute_ms > 0.0 {
        cuda_compute_ms
    } else {
        0.0
    };
    let denom_ms = cuda_ms.max(crowd_ms + traffic_ms).max(0.01);
    let other_ms = (denom_ms - crowd_ms - traffic_ms).max(0.0);
    let sim_pct = (((crowd_ms + traffic_ms) / denom_ms) * 100.0).clamp(0.0, 100.0);
    let share_segs = [
        BarSegment {
            count: (crowd_ms * 1000.0).round() as u64,
            color: Color32::from_rgb(166, 209, 137),
        },
        BarSegment {
            count: (traffic_ms * 1000.0).round() as u64,
            color: Color32::from_rgb(238, 212, 159),
        },
        BarSegment {
            count: (other_ms * 1000.0).round() as u64,
            color: Color32::from_rgb(98, 122, 156),
        },
    ];
    let share_txt = fmt_into(label_buf, format_args!("{sim_pct:.0}%"));
    stacked_bar_row(ui, "Sim Share", &share_segs, Some(share_txt), tip_buf, |t| {
        let c_pct = (crowd_ms / denom_ms) * 100.0;
        let t_pct = (traffic_ms / denom_ms) * 100.0;
        let o_pct = (other_ms / denom_ms) * 100.0;
        let _ = write!(
            t,
            "Portion of total CUDA Compute frame time spent simulating crowds and traffic versus other GPU compute.\nTotal: {denom_ms:.2} ms | Crowd: {crowd_ms:.2} ms ({c_pct:.1}%) | Traffic: {traffic_ms:.2} ms ({t_pct:.1}%)\nOther CUDA (IAS/Ocean/FFT/TM): {other_ms:.2} ms ({o_pct:.1}%)"
        );
    });

    ui.add_space(2.0);
    sparkline_row(
        ui,
        label_buf,
        tip_buf,
        SparklineMeta {
            label: "Crowd ms",
            desc: "Recent history of crowd simulation time per frame in milliseconds (lower leaves more budget for ray tracing).",
            unit: "ms",
        },
        &timings.crowd_step_ms,
        Color32::from_rgb(166, 209, 137),
    );
    sparkline_row(
        ui,
        label_buf,
        tip_buf,
        SparklineMeta {
            label: "kBones/s",
            desc: "Thousands of animated character bones updated per second (higher reflects larger crowds or higher FPS).",
            unit: "k",
        },
        &timings.crowd_kbones_s,
        Color32::from_rgb(112, 186, 238),
    );
}

/// Renders the `Traffic & ADAS` section body: backend status pill, fleet counts, live Fleet State
/// stacked bar (`Moving | Ped Yield | Ped AEB | Stopped`), flow, pedestrian safety summary, VRAM,
/// and live `Traffic ms` / `Flow` / `AEB/Yld` sparklines.
fn render_traffic_stats_body(
    ui: &mut Ui,
    sim: &SimTelemetry,
    timings: &FrameTimings,
    label_buf: &mut String,
    tip_buf: &mut String,
) {
    let (pill_text, pill_tone, show_ms) = if sim.traffic_stepping {
        match sim.traffic_backend {
            ActiveTrafficBackend::Cuda => {
                let ver = sim.traffic_plan_version.max(1);
                let tag = if ver == 1 { "CUDA v1" } else { "CUDA" };
                (tag, PillTone::Green, true)
            }
            ActiveTrafficBackend::Cpu => ("CPU SoA", PillTone::Amber, true),
            ActiveTrafficBackend::None => ("Idle", PillTone::Muted, false),
        }
    } else if sim.pose_frozen {
        match sim.traffic_backend {
            ActiveTrafficBackend::Cuda => ("CUDA Frozen", PillTone::Muted, false),
            ActiveTrafficBackend::Cpu => ("CPU Frozen", PillTone::Muted, false),
            ActiveTrafficBackend::None => ("Frozen", PillTone::Muted, false),
        }
    } else {
        match sim.traffic_backend {
            ActiveTrafficBackend::Cuda => ("CUDA Paused", PillTone::Muted, false),
            ActiveTrafficBackend::Cpu => ("CPU Paused", PillTone::Muted, false),
            ActiveTrafficBackend::None => ("Idle", PillTone::Muted, false),
        }
    };
    let trailing = if show_ms {
        fmt_into(label_buf, format_args!("{:.2} ms", sim.traffic_step_ms))
    } else {
        ""
    };
    stat_badge_row_tip(
        ui,
        "Traffic Sim",
        StatusPill {
            text: pill_text,
            tone: pill_tone,
        },
        trailing,
        tip_buf,
        |t| {
            let _ = write!(
                t,
                "Where road vehicles are simulated (CUDA GPU or CPU) and how long each step takes.\nBackend: {} | Step: {:.2} ms | Lane Graph: {} | Dyn Inst: {}",
                sim.traffic_backend.label(),
                sim.traffic_step_ms,
                if sim.traffic_has_lane_graph {
                    "Active"
                } else {
                    "None"
                },
                sim.traffic_dyn_instances
            );
            if sim.traffic_backend == ActiveTrafficBackend::Cuda {
                let _ = write!(
                    t,
                    "\nPlanKind::Traffic v{} ({} kernels, StrictF32) | IAS Rebuilds: {}",
                    sim.traffic_plan_version.max(1),
                    sim.traffic_kernel_count,
                    sim.traffic_ias_rebuilds
                );
            }
        },
    );

    let stopped = sim.traffic_stopped();
    let veh_val = fmt_into(
        label_buf,
        format_args!(
            "{} ({} mov / {} stop)",
            sim.traffic_vehicles, sim.traffic_moving, stopped
        ),
    );
    stat_row_tip(ui, "Vehicles", veh_val, tip_buf, |t| {
        let _ = write!(
            t,
            "Total road vehicles in the city split into moving traffic and cars stopped at lights or queues.\nTotal: {} | Moving (>0.25 m/s): {} | Stopped: {} | CSG Inst: {}",
            sim.traffic_vehicles, sim.traffic_moving, stopped, sim.traffic_dyn_instances
        );
    });

    let total_v = sim.traffic_vehicles;
    let aeb_v = sim.traffic_aeb_active.min(total_v);
    let yld_v = sim.traffic_yield_active.min(total_v.saturating_sub(aeb_v));
    let non_ped = total_v.saturating_sub(aeb_v + yld_v);
    let moving_clear = sim.traffic_moving.min(non_ped);
    let stopped_other = non_ped.saturating_sub(moving_clear);
    let fleet_segs = [
        BarSegment {
            count: u64::from(moving_clear),
            color: Color32::from_rgb(122, 206, 138),
        },
        BarSegment {
            count: u64::from(yld_v),
            color: Color32::from_rgb(242, 184, 72),
        },
        BarSegment {
            count: u64::from(aeb_v),
            color: Color32::from_rgb(236, 78, 86),
        },
        BarSegment {
            count: u64::from(stopped_other),
            color: Color32::from_rgb(104, 114, 132),
        },
    ];
    let fleet_txt = fmt_into(label_buf, format_args!("{}/{total_v}", sim.traffic_moving));
    stacked_bar_row(ui, "Fleet State", &fleet_segs, Some(fleet_txt), tip_buf, |t| {
        let _ = write!(
            t,
            "Live visual split of cruising cars (green), yielding cars (amber), emergency braking (red), and stopped cars (grey).\nMoving: {moving_clear} | Ped Yield: {yld_v} | Ped AEB: {aeb_v} | Stopped: {stopped_other} (Total: {total_v})"
        );
    });

    let avg_kmh = sim.traffic_avg_kmh();
    let max_kmh = sim.traffic_max_kmh();
    let flow_val = fmt_into(
        label_buf,
        format_args!("{avg_kmh:.1} km/h (max {max_kmh:.0})"),
    );
    stat_row_tip(ui, "Traffic Flow", flow_val, tip_buf, |t| {
        let _ = write!(
            t,
            "Average and top driving speed across all vehicles in the road network.\nMean: {avg_kmh:.1} km/h ({:.2} m/s) | Peak: {max_kmh:.1} km/h ({:.2} m/s)\nRespawns: {} | Stale Releases: {} | Deadlock Reloc: {}",
            sim.traffic_avg_speed_mps,
            sim.traffic_max_speed_mps,
            sim.traffic_respawns,
            sim.traffic_stale_releases,
            sim.traffic_deadlock_relocations
        );
    });

    let ceil_kmh = max_kmh.max(50.0);
    let flow_col = if avg_kmh >= 20.0 {
        Color32::from_rgb(122, 206, 138)
    } else if avg_kmh >= 5.0 {
        Color32::from_rgb(242, 184, 72)
    } else {
        Color32::from_rgb(104, 114, 132)
    };
    let f_filled = (avg_kmh * 100.0).round() as u64;
    let f_total = (ceil_kmh * 100.0).round() as u64;
    let flow_segs = [
        BarSegment {
            count: f_filled,
            color: flow_col,
        },
        BarSegment {
            count: f_total.saturating_sub(f_filled),
            color: Color32::TRANSPARENT,
        },
    ];
    let flow_bar_txt = fmt_into(label_buf, format_args!("{avg_kmh:.0} km/h"));
    stacked_bar_row(ui, "Flow Speed", &flow_segs, Some(flow_bar_txt), tip_buf, |t| {
        let _ = write!(
            t,
            "Average fleet speed compared against the fastest vehicle so you can spot traffic jams at a glance.\nMean: {avg_kmh:.1} km/h ({:.2} m/s) | Peak: {max_kmh:.1} km/h ({:.2} m/s) | Scale: {ceil_kmh:.0} km/h",
            sim.traffic_avg_speed_mps, sim.traffic_max_speed_mps
        );
    });

    let safety_val = fmt_into(
        label_buf,
        format_args!(
            "AEB:{} Yld:{} Rel:{}",
            sim.traffic_aeb_active,
            sim.traffic_yield_active,
            sim.traffic_deadlock_relocations
        ),
    );
    stat_row_tip(ui, "Ped Safety", safety_val, tip_buf, |t| {
        let _ = write!(
            t,
            "Count of cars actively emergency-braking (AEB) or slowing (Yield) for crossing pedestrians, plus deadlock clears.\nPed AEB: {} | Ped Yield: {} | Deadlock Reloc: {}\nStale Junction Releases: {} | Network Respawns: {}",
            sim.traffic_aeb_active,
            sim.traffic_yield_active,
            sim.traffic_deadlock_relocations,
            sim.traffic_stale_releases,
            sim.traffic_respawns
        );
    });

    if sim.traffic_backend == ActiveTrafficBackend::Cuda && sim.traffic_gpu_bytes > 0 {
        SimTelemetry::write_bytes_compact(sim.traffic_gpu_bytes, label_buf);
        stat_row_tip(ui, "Traffic VRAM", label_buf.as_str(), tip_buf, |t| {
            t.push_str("GPU memory holding vehicle lane-graph simulation and wheel/body transform buffers.\nPlanKind::Traffic + Scatter Bridge VRAM: ");
            SimTelemetry::append_bytes_compact(sim.traffic_gpu_bytes, t);
            let _ = write!(
                t,
                "\nAmortized Full IAS Rebuilds (>8m disp): {}",
                sim.traffic_ias_rebuilds
            );
        });
    }

    ui.add_space(2.0);
    sparkline_row(
        ui,
        label_buf,
        tip_buf,
        SparklineMeta {
            label: "Traffic ms",
            desc: "Recent history of vehicle simulation step time per frame in milliseconds.",
            unit: "ms",
        },
        &timings.traffic_step_ms,
        Color32::from_rgb(238, 212, 159),
    );
    sparkline_row(
        ui,
        label_buf,
        tip_buf,
        SparklineMeta {
            label: "Flow",
            desc: "Recent trend of average vehicle speed in km/h (dips indicate red lights or congestion).",
            unit: "km/h",
        },
        &timings.traffic_avg_kmh,
        Color32::from_rgb(122, 206, 138),
    );
    sparkline_row(
        ui,
        label_buf,
        tip_buf,
        SparklineMeta {
            label: "AEB/Yld",
            desc: "Recent count of vehicles actively braking or yielding for pedestrians (spikes mark live crosswalk encounters).",
            unit: "",
        },
        &timings.traffic_ped_events,
        Color32::from_rgb(238, 112, 92),
    );
}

/// Renders the compact ADAS driving-state badge, speed readout, and bipolar `THR/BRK` bar inside
/// the floating `PossessToolbar` while riding a dynamic lane-graph traffic vehicle.
pub(crate) fn render_possess_adas_pill(ui: &mut Ui, adas: PossessAdas) {
    use csg_scene::traffic::hazard_state;

    let (badge_icon, fg, bg) = match adas.severity() {
        2 => (
            ic::WARNING,
            Color32::from_rgb(255, 224, 224),
            Color32::from_rgb(156, 28, 32),
        ),
        1 => {
            let icon = match adas.state {
                hazard_state::RED_LIGHT | hazard_state::STOP_SIGN => ic::STOP_CIRCLE,
                _ => ic::WARNING,
            };
            (
                icon,
                Color32::from_rgb(255, 236, 196),
                Color32::from_rgb(138, 86, 16),
            )
        }
        _ => (
            ic::CAR,
            Color32::from_rgb(196, 246, 222),
            Color32::from_rgb(24, 88, 66),
        ),
    };

    let pill_resp = egui::Frame::new()
        .fill(bg)
        .corner_radius(3.0)
        .inner_margin(egui::Margin::symmetric(5, 2))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 3.0;
                ui.label(RichText::new(badge_icon).size(12.0).color(fg));
                ui.label(RichText::new(adas.badge_label()).small().strong().color(fg));
            });
        })
        .response;

    let spd_resp = ui.label(
        RichText::new(format!("{:>3.0} km/h", adas.speed_kmh()))
            .small()
            .monospace()
            .color(ui.visuals().strong_text_color()),
    );

    // Compact bipolar THR/BRK bar (left = brake in red-orange, right = throttle in green).
    let bar_size = egui::vec2(48.0, 10.0);
    let (bar_rect, bar_resp) = ui.allocate_exact_size(bar_size, Sense::hover());
    if ui.is_rect_visible(bar_rect) {
        let p = ui.painter();
        p.rect_filled(bar_rect, 2.0, Color32::from_rgb(24, 27, 32));
        let mid_x = bar_rect.center().x;
        let norm = adas.throttle_brake_norm();
        if norm > 0.01 {
            let fill = egui::Rect::from_min_max(
                egui::pos2(mid_x, bar_rect.top() + 1.0),
                egui::pos2(mid_x + norm * (bar_rect.width() * 0.5 - 1.0), bar_rect.bottom() - 1.0),
            );
            p.rect_filled(fill, 1.0, Color32::from_rgb(56, 206, 128));
        } else if norm < -0.01 {
            let fill = egui::Rect::from_min_max(
                egui::pos2(
                    mid_x + norm * (bar_rect.width() * 0.5 - 1.0),
                    bar_rect.top() + 1.0,
                ),
                egui::pos2(mid_x, bar_rect.bottom() - 1.0),
            );
            p.rect_filled(fill, 1.0, Color32::from_rgb(232, 72, 62));
        }
        p.line_segment(
            [
                egui::pos2(mid_x, bar_rect.top()),
                egui::pos2(mid_x, bar_rect.bottom()),
            ],
            egui::Stroke::new(1.0, Color32::from_rgb(108, 116, 128)),
        );
        p.rect_stroke(
            bar_rect,
            2.0,
            egui::Stroke::new(1.0, Color32::from_rgb(64, 70, 80)),
            egui::StrokeKind::Inside,
        );
    }

    let hover_ui = |ui: &mut Ui| {
        ui.label(format!(
            "State: {} | Brake Lights: {}",
            adas.badge_label(),
            if adas.brake_light_on() { "ON" } else { "OFF" }
        ));
        ui.label(format!(
            "Speed: {:.1} / {:.0} km/h | Accel: {:+.2} m/s^2",
            adas.speed_kmh(),
            adas.limit_kmh(),
            adas.accel_mps2
        ));
        let hz_dist = if adas.hazard_dist_m.is_finite() {
            if adas.has_ped_hazard() {
                format!("{:.1} m (ped #{})", adas.hazard_dist_m, adas.hazard_actor_idx)
            } else {
                format!("{:.1} m", adas.hazard_dist_m)
            }
        } else {
            "Clear".to_string()
        };
        ui.label(format!(
            "Stop Dist: {:.1} m | Nearest Hazard: {hz_dist}",
            adas.stop_dist_m
        ));
    };
    pill_resp.on_hover_ui(hover_ui);
    spd_resp.on_hover_ui(hover_ui);
    bar_resp.on_hover_ui(hover_ui);
}

/// Partition predicate for the eye-dropdown OSM provenance section: returns `true` for OSM /
/// terrain feature classes (`Building..=Poi`, `Terrain`) and `false` for generated / dynamic /
/// unknown classes (`Procedural..=DynamicRig`, `Unknown`).
fn is_osm_feature_class(cls: csg_scene::provenance::OsmFeatureClass) -> bool {
    use csg_scene::provenance::OsmFeatureClass as C;
    matches!(
        cls,
        C::Building
            | C::Road
            | C::Water
            | C::LandUse
            | C::Vegetation
            | C::Barrier
            | C::Industrial
            | C::Power
            | C::Railway
            | C::Marine
            | C::StreetFurniture
            | C::Poi
            | C::Terrain
    )
}

/// Compact two-column checkbox grid over a group of [`csg_scene::provenance::OsmFeatureClass`]
/// entries, mutating `osm_mask` via [`crate::semantic_filter::toggle_osm_class`].
fn osm_class_grid(
    ui: &mut Ui,
    id_salt: &str,
    classes: impl Iterator<Item = csg_scene::provenance::OsmFeatureClass>,
    osm_mask: &mut u32,
) {
    use crate::semantic_filter as sf;
    egui::Grid::new(id_salt)
        .num_columns(2)
        .spacing([12.0, 2.0])
        .show(ui, |ui| {
            let mut col = 0usize;
            for cls in classes {
                let mut visible = (*osm_mask & sf::osm_class_bit(cls)) == 0;
                if ui.checkbox(&mut visible, cls.label()).changed() {
                    *osm_mask = sf::toggle_osm_class(*osm_mask, cls, visible);
                }
                col += 1;
                if col.is_multiple_of(2) {
                    ui.end_row();
                }
            }
            if !col.is_multiple_of(2) {
                ui.end_row();
            }
        });
}

/// The viewport toolbar "eye" dropdown (`crate::semantic_filter`): which semantic categories and
/// P42 OSM / source feature classes are rendered. Same pattern as the sl-studio viewport
/// visibility popover used by warpbro-rs (an eye `MenuButton` over a grid of
/// `sl_config::icons::tool_toggle` eye toggles, closing on a click outside); nv-demo adds Show all
/// / None, per-id sub-rows (Buildings > Windows), per-class P42 OSM / source checkboxes, and a
/// combined badge with the total number of hidden classes on the icon. Both masks are persisted
/// and reset accumulation on change.
fn semantic_filter_menu(ui: &mut Ui, mask: &mut u64, osm_mask: &mut u32) {
    use crate::semantic_filter::{self as sf, Hidden};
    use csg_scene::provenance::OsmFeatureClass;
    let sem_hidden = sf::hidden_count(*mask);
    let osm_hidden = sf::osm_hidden_count(*osm_mask);
    let hidden = sf::combined_hidden_count(*mask, *osm_mask);
    let label = if hidden > 0 {
        RichText::new(format!("{} {hidden}", ic::EYE_SLASH))
            .size(vt::TB_ICON_SIZE)
            .color(Color32::from_rgb(255, 196, 64))
    } else {
        RichText::new(ic::EYE).size(vt::TB_ICON_SIZE)
    };
    let tip = match (sem_hidden, osm_hidden) {
        (0, 0) => "Visible categories: everything is rendered".to_string(),
        (s, 0) => format!(
            "Visible categories: {s} semantic classes hidden (also from shadows and reflections)"
        ),
        (0, o) => format!(
            "Visible categories: {o} OSM / source classes hidden (also from shadows and reflections)"
        ),
        (s, o) => format!(
            "Visible categories: {s} semantic and {o} OSM / source classes hidden (also from shadows and reflections)"
        ),
    };
    let config = egui::containers::menu::MenuConfig::new()
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
    let (resp, _) = egui::containers::menu::MenuButton::new(label).config(config).ui(ui, |ui| {
        ui.horizontal(|ui| {
            if ui
                .button("Show all")
                .on_hover_text("Render every semantic and OSM / source class")
                .clicked()
            {
                *mask = 0;
                *osm_mask = 0;
            }
            if ui
                .button("None")
                .on_hover_text("Hide every semantic category")
                .clicked()
            {
                *mask = sf::all_mask();
            }
        });
        ui.separator();
        egui::ScrollArea::vertical()
            .id_salt("semantic_filter_scroll")
            .max_height(560.0)
            .show(ui, |ui| {
                ui.label(RichText::new("Semantic groups").strong());
                egui::Grid::new("semantic_filter_grid")
                    .num_columns(2)
                    .spacing([8.0, 2.0])
                    .show(ui, |ui| {
                        for g in sf::GROUPS {
                            if let Some(reason) = g.pending {
                                ui.add_enabled(false, egui::Label::new(g.label));
                                ui.add_enabled(
                                    false,
                                    egui::Label::new(RichText::new(reason).italics()),
                                );
                                ui.end_row();
                                continue;
                            }
                            let state = sf::group_hidden(*mask, g);
                            let icon = if state == Hidden::All {
                                ic::EYE_SLASH
                            } else {
                                ic::EYE
                            };
                            let tip = match state {
                                Hidden::None => "Shown: click to hide the whole group",
                                Hidden::Some => "Partly hidden: click to show the whole group",
                                Hidden::All => "Hidden: click to show the whole group",
                            };
                            if sl_config::icons::tool_toggle(ui, state == Hidden::None, icon, tip)
                                .clicked()
                            {
                                *mask = sf::set_group(*mask, g, state != Hidden::None);
                            }
                            ui.label(g.label);
                            ui.end_row();
                            if g.ids.len() > 1 {
                                for &id in g.ids {
                                    let on = *mask & sf::bit(id) == 0;
                                    ui.horizontal(|ui| {
                                        ui.add_space(12.0);
                                        let icon = if on { ic::EYE } else { ic::EYE_SLASH };
                                        if sl_config::icons::tool_toggle(
                                            ui,
                                            on,
                                            icon,
                                            "Show / hide this class",
                                        )
                                        .clicked()
                                        {
                                            *mask ^= sf::bit(id);
                                        }
                                    });
                                    ui.label(RichText::new(sf::semantic_name(id)).weak());
                                    ui.end_row();
                                }
                            }
                        }
                    });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(RichText::new("OSM / Source Classes").strong());
                    if ui
                        .small_button("Show all")
                        .on_hover_text("Show all OSM and generated / dynamic classes")
                        .clicked()
                    {
                        *osm_mask = 0;
                    }
                    if ui
                        .small_button("Hide all")
                        .on_hover_text("Hide all OSM and generated / dynamic classes")
                        .clicked()
                    {
                        *osm_mask = sf::osm_all_mask();
                    }
                });
                ui.label(RichText::new("OSM Features").weak().small());
                osm_class_grid(
                    ui,
                    "osm_features_filter_grid",
                    OsmFeatureClass::ALL
                        .iter()
                        .copied()
                        .filter(|&c| is_osm_feature_class(c)),
                    osm_mask,
                );
                ui.add_space(4.0);
                ui.label(RichText::new("Generated / Dynamic").weak().small());
                osm_class_grid(
                    ui,
                    "osm_generated_filter_grid",
                    OsmFeatureClass::ALL
                        .iter()
                        .copied()
                        .filter(|&c| !is_osm_feature_class(c) && c != OsmFeatureClass::Unknown)
                        .chain(std::iter::once(OsmFeatureClass::Unknown)),
                    osm_mask,
                );
            });
    });
    resp.on_hover_text(tip);
}

/// Static metadata for a [`sparkline_row`] (label, plain-English description, unit suffix).
#[derive(Clone, Copy)]
struct SparklineMeta<'a> {
    label: &'a str,
    desc: &'a str,
    unit: &'a str,
}

/// Renders a compact, width-clamped live telemetry sparkline inside a [`prop_row_labeled`] with a
/// subtle mid-height guide line, translucent vertical-gradient area fill, bright latest-sample
/// dot, monospace readout, and lazy plain-English + min/max hover tooltip on both label and plot.
fn sparkline_row(
    ui: &mut Ui,
    label_buf: &mut String,
    tip_buf: &mut String,
    meta: SparklineMeta<'_>,
    ring: &RingBuffer<HISTORY_LEN>,
    color: Color32,
) {
    let unit = meta.unit;
    prop_row_labeled(ui, meta.label, true, |ui, label_resp| {
        let avail_w = ui.available_width().max(1.0);
        let gap = 5.0;
        let desired_val_w: f32 = if unit.len() >= 3 {
            58.0
        } else if !unit.is_empty() {
            48.0
        } else {
            34.0
        };
        let val_w = desired_val_w.min((avail_w * 0.48).max(30.0));
        let plot_w = (avail_w - val_w - gap).max(24.0);
        ui.spacing_mut().item_spacing.x = gap;

        let (rect, plot_resp) =
            ui.allocate_exact_size(egui::vec2(plot_w, ROW_H - 2.0), Sense::hover());
        let n = ring.len();
        let (min_v, max_v) = ring.min_max();

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let border_col = ui
                .visuals()
                .widgets
                .noninteractive
                .bg_stroke
                .color
                .gamma_multiply(0.5);
            painter.rect_filled(rect, 2.5, ui.visuals().extreme_bg_color);

            let guide_col = ui.visuals().weak_text_color().gamma_multiply(0.22);
            painter.line_segment(
                [
                    egui::pos2(rect.left() + 2.0, rect.center().y),
                    egui::pos2(rect.right() - 2.0, rect.center().y),
                ],
                egui::Stroke::new(0.75, guide_col),
            );

            if n >= 2 && rect.width() > 6.0 {
                let flat_nonzero = (max_v - min_v) < 1e-3 && max_v > 0.0;
                let span = (max_v - min_v).max(1e-3);
                let top_y = rect.top() + 2.0;
                let bot_y = rect.bottom() - 2.0;
                let base_y = rect.bottom() - 1.25;
                let inner_h = (bot_y - top_y).max(1.0);
                let inner_w = (rect.width() - 5.0).max(1.0);
                let left_x = rect.left() + 2.0;
                let denom = (n - 1) as f32;

                let mut pts = Vec::with_capacity(n);
                let mut fill = egui::epaint::Mesh::default();
                fill.vertices.reserve(n * 2);
                fill.indices.reserve((n - 1) * 6);
                let top_fill = color.gamma_multiply(0.26);
                let bot_fill = color.gamma_multiply(0.03);

                for (i, raw_v) in ring.iter().enumerate() {
                    let v = if raw_v.is_finite() { raw_v } else { min_v };
                    let x = left_x + inner_w * (i as f32 / denom);
                    let norm = if flat_nonzero {
                        0.5
                    } else {
                        ((v - min_v) / span).clamp(0.0, 1.0)
                    };
                    let y = bot_y - inner_h * norm;
                    let pt = egui::pos2(x, y);
                    pts.push(pt);
                    fill.vertices.push(egui::epaint::Vertex::untextured(pt, top_fill));
                    fill.vertices.push(egui::epaint::Vertex::untextured(
                        egui::pos2(x, base_y),
                        bot_fill,
                    ));
                }
                for i in 0..(n - 1) {
                    let a = (2 * i) as u32;
                    fill.add_triangle(a, a + 1, a + 2);
                    fill.add_triangle(a + 1, a + 3, a + 2);
                }

                let last_pt = pts.last().copied();
                painter.add(egui::Shape::mesh(fill));
                painter.add(egui::Shape::line(pts, egui::Stroke::new(1.5, color)));
                if let Some(dot) = last_pt {
                    painter.circle_filled(dot, 3.0, color.gamma_multiply(0.32));
                    painter.circle_filled(dot, 1.7, color.lerp_to_gamma(Color32::WHITE, 0.35));
                }
            } else if n == 1 && rect.width() > 6.0 {
                let dot = egui::pos2(rect.right() - 3.5, rect.center().y);
                painter.circle_filled(dot, 3.0, color.gamma_multiply(0.32));
                painter.circle_filled(dot, 1.7, color.lerp_to_gamma(Color32::WHITE, 0.35));
            }

            painter.rect_stroke(
                rect,
                2.5,
                egui::Stroke::new(0.75, border_col),
                egui::StrokeKind::Inside,
            );
        }

        let latest_raw = ring.latest();
        let latest = if latest_raw.is_finite() {
            latest_raw
        } else {
            0.0
        };
        let txt = if unit.is_empty() {
            fmt_into(label_buf, format_args!("{latest:.0}"))
        } else if unit == "ms" && latest < 10.0 {
            fmt_into(label_buf, format_args!("{latest:.2} {unit}"))
        } else {
            fmt_into(label_buf, format_args!("{latest:.1} {unit}"))
        };
        let val_resp = allocate_clamped_ui(
            ui,
            egui::vec2(val_w, ROW_H),
            Layout::right_to_left(Align::Center),
            |ui| {
                ui.add(
                    egui::Label::new(
                        RichText::new(txt).monospace().size(10.5).color(color),
                    )
                    .truncate(),
                )
            },
        );

        let hovered = if plot_resp.hovered() {
            Some(plot_resp)
        } else if val_resp.hovered() {
            Some(val_resp)
        } else if label_resp.hovered() {
            Some(label_resp)
        } else {
            None
        };
        if let Some(resp) = hovered {
            tip_buf.clear();
            if !meta.desc.is_empty() {
                tip_buf.push_str(meta.desc);
                tip_buf.push('\n');
            }
            let suffix = if unit.is_empty() { "" } else { unit };
            let sep = if unit.is_empty() { "" } else { " " };
            if n > 0 {
                let _ = write!(
                    tip_buf,
                    "Latest: {latest:.2}{sep}{suffix} | Range: {min_v:.2} .. {max_v:.2}{sep}{suffix} ({n} frames)"
                );
            } else {
                tip_buf.push_str("No samples collected yet.");
            }
            resp.on_hover_text(tip_buf.as_str());
        }
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
        let expected_track_w = 320.0 - LABEL_W - 6.0 - ANIM_BTN_W - 8.0 - VAL_W;
        assert!((track_w - expected_track_w).abs() < 1e-4);
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
    fn ground_kerb_mode_serde_defaults_to_in_tile_and_roundtrips() {
        // Legacy JSON missing `csg_ground_kerb_mode` deserializes to In-Tile F-Rep (`0`).
        let legacy: DemoSettings = serde_json::from_str("{}").expect("legacy json");
        assert_eq!(legacy.csg_ground_kerb_mode, 0);
        assert_eq!(legacy.csg_smart_fill, csg_scene::SmartFill::OFF);

        // Explicit Sweep Tile (`1`) and SmartFill flags round-trip cleanly.
        let sf = csg_scene::SmartFill {
            street_lamps: true,
            trees: true,
            crossings: true,
            ..csg_scene::SmartFill::OFF
        };
        let s = DemoSettings {
            csg_ground_kerb_mode: 1,
            csg_smart_fill: sf,
            ..DemoSettings::default()
        };
        let json = serde_json::to_string(&s).expect("serialize");
        let back: DemoSettings = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.csg_ground_kerb_mode, 1);
        assert_eq!(back.csg_smart_fill, sf);
        assert_eq!(csg_scene::GroundKerbMode::ALL.len(), 2);
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

    #[test]
    fn osm_feature_class_partition_covers_all_variants_once() {
        use csg_scene::provenance::OsmFeatureClass;
        let osm: Vec<_> = OsmFeatureClass::ALL
            .iter()
            .copied()
            .filter(|&c| is_osm_feature_class(c))
            .collect();
        let generated: Vec<_> = OsmFeatureClass::ALL
            .iter()
            .copied()
            .filter(|&c| !is_osm_feature_class(c) && c != OsmFeatureClass::Unknown)
            .chain(std::iter::once(OsmFeatureClass::Unknown))
            .collect();
        assert_eq!(osm.len() + generated.len(), OsmFeatureClass::ALL.len());
        assert_eq!(generated.last(), Some(&OsmFeatureClass::Unknown));
        for &cls in OsmFeatureClass::ALL {
            let count = osm.iter().filter(|&&c| c == cls).count()
                + generated.iter().filter(|&&c| c == cls).count();
            assert_eq!(count, 1, "OsmFeatureClass::{cls:?} must appear exactly once");
        }
    }

    #[test]
    fn stroked_and_unstroked_buttons_have_zero_hover_height_jitter_all_themes() {
        for theme in UiTheme::ALL {
            let ctx = egui::Context::default();
            install_fonts(&ctx);
            let _dock_style = theme.apply(&ctx);

            let run_pass = |pointer: Option<egui::Pos2>| -> (egui::Rect, egui::Rect, bool, bool) {
                let mut raw = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(320.0, 240.0),
                    )),
                    ..Default::default()
                };
                if let Some(pos) = pointer {
                    raw.events.push(egui::Event::PointerMoved(pos));
                }
                let mut auto_rect = egui::Rect::NOTHING;
                let mut manual_rect = egui::Rect::NOTHING;
                let mut auto_hov = false;
                let mut manual_hov = false;
                let out = ctx.run_ui(raw, |ui| {
                    dock_scroll(ui, |ui| {
                        collapsing_section(
                            ui,
                            "sec_flab_author",
                            ic::PAPER_PLANE_TILT,
                            "Flight Authoring",
                            |ui| {
                                equal_grid(ui, 2, 2, |ui, idx, w| match idx {
                                    0 => {
                                        let btn = egui::Button::new(format!(
                                            "{}  Auto Plan",
                                            ic::SPARKLE
                                        ))
                                        .truncate()
                                        .fill(active_accent_tint(ui))
                                        .stroke(egui::Stroke::new(
                                            1.0,
                                            ui.visuals().selection.stroke.color,
                                        ));
                                        let resp = ui.add_sized([w, ROW_H + 2.0], btn);
                                        auto_rect = resp.rect;
                                        auto_hov = resp.hovered();
                                    }
                                    _ => {
                                        let btn =
                                            egui::Button::new(format!("{}  Manual", ic::PATH))
                                                .truncate();
                                        let resp = ui.add_sized([w, ROW_H + 2.0], btn);
                                        manual_rect = resp.rect;
                                        manual_hov = resp.hovered();
                                    }
                                });
                            },
                        );
                    });
                });
                out.drop_without_applying_deltas();
                (auto_rect, manual_rect, auto_hov, manual_hov)
            };

            // Warm up layout, then capture unhovered baseline (`WidgetState::Inactive`).
            let _ = run_pass(None);
            let (auto_idle, manual_idle, _, _) = run_pass(None);
            assert!(auto_idle.height() > 0.0 && manual_idle.height() > 0.0);
            assert_eq!(
                (auto_idle.height() - manual_idle.height()).abs(),
                0.0,
                "{theme:?}: active .stroke(1.0) vs inactive button height mismatch when unhovered ({} vs {})",
                auto_idle.height(),
                manual_idle.height()
            );

            // Move pointer over `Auto Plan`: two passes so `Button::atom_ui` reads the hovered
            // response from the previous pass and evaluates `WidgetState::Hovered`.
            let _ = run_pass(Some(auto_idle.center()));
            let (auto_hov_rect, manual_while_auto_hov, auto_hov, _) =
                run_pass(Some(auto_idle.center()));
            assert!(auto_hov, "{theme:?}: Auto Plan button must register hover");
            assert_eq!(
                (auto_hov_rect.height() - auto_idle.height()).abs(),
                0.0,
                "{theme:?}: Auto Plan height jittered on hover ({} -> {})",
                auto_idle.height(),
                auto_hov_rect.height()
            );
            assert_eq!(
                (auto_hov_rect.height() - manual_while_auto_hov.height()).abs(),
                0.0,
                "{theme:?}: hovered Auto Plan vs unhovered Manual height mismatch"
            );

            // Move pointer over `Manual`: verify zero height jitter on unstroked buttons too.
            let _ = run_pass(Some(manual_idle.center()));
            let (auto_while_manual_hov, manual_hov_rect, _, manual_hov) =
                run_pass(Some(manual_idle.center()));
            assert!(manual_hov, "{theme:?}: Manual button must register hover");
            assert_eq!(
                (manual_hov_rect.height() - manual_idle.height()).abs(),
                0.0,
                "{theme:?}: Manual height jittered on hover ({} -> {})",
                manual_idle.height(),
                manual_hov_rect.height()
            );
            assert_eq!(
                (auto_while_manual_hov.height() - manual_hov_rect.height()).abs(),
                0.0,
                "{theme:?}: unhovered Auto Plan vs hovered Manual height mismatch"
            );
        }
    }

    #[test]
    fn consecutive_sliders_with_unit_suffixes_never_staircase_or_clip() {
        for panel_w in [240.0_f32, 300.0, 360.0] {
            let ctx = egui::Context::default();
            install_fonts(&ctx);
            let _dock_style = UiTheme::Obsidian.apply(&ctx);

            let sc = ctx.global_style().spacing.scroll;
            let bar_reserve = sc.bar_outer_margin + sc.bar_width + sc.bar_inner_margin + 2.0;
            let inner_w = (panel_w - bar_reserve).max(120.0);

            let mut max_bank = 35.0_f32;
            let mut bank_gain = 0.55_f32;
            let mut bank_rate = 120.0_f32;
            let mut long_accel = 12.0_f32;
            let mut jerk = 20.0_f32;

            let mut avail_widths = Vec::new();
            let mut row_rights = Vec::new();
            let mut sibling_avail_w = 0.0_f32;
            let mut worst_text_w = 0.0_f32;

            for _pass in 0..2 {
                avail_widths.clear();
                row_rights.clear();
                let raw = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(panel_w, 800.0),
                    )),
                    ..Default::default()
                };
                let out = ctx.run_ui(raw, |ui| {
                    let galley = ui.painter().layout_no_wrap(
                        "20.0 m/s3".to_owned(),
                        egui::FontId::monospace(11.0),
                        Color32::WHITE,
                    );
                    worst_text_w = galley.size().x;

                    dock_scroll(ui, |ui| {
                        collapsing_section(
                            ui,
                            "sec_flab_dynamics",
                            ic::SLIDERS_HORIZONTAL,
                            "Camera Dynamics",
                            |ui| {
                                avail_widths.push(ui.available_width());
                                prop_slider_f32(
                                    ui,
                                    "Max Bank",
                                    &mut max_bank,
                                    0.0..=60.0,
                                    1,
                                    " deg",
                                );
                                row_rights.push(ui.min_rect().right());

                                avail_widths.push(ui.available_width());
                                prop_slider_f32(ui, "Bank Gain", &mut bank_gain, 0.0..=1.0, 2, "");
                                row_rights.push(ui.min_rect().right());

                                avail_widths.push(ui.available_width());
                                prop_slider_f32(
                                    ui,
                                    "Bank Rate",
                                    &mut bank_rate,
                                    10.0..=180.0,
                                    0,
                                    " d/s",
                                );
                                row_rights.push(ui.min_rect().right());

                                avail_widths.push(ui.available_width());
                                prop_slider_f32(
                                    ui,
                                    "Long Accel",
                                    &mut long_accel,
                                    1.0..=12.0,
                                    1,
                                    " m/s2",
                                );
                                row_rights.push(ui.min_rect().right());

                                avail_widths.push(ui.available_width());
                                prop_slider_f32(ui, "Jerk", &mut jerk, 1.0..=20.0, 1, " m/s3");
                                row_rights.push(ui.min_rect().right());

                                avail_widths.push(ui.available_width());
                                stat_row(ui, "Ratio", "0.94 | 14.2 m/s | 35.0 deg");
                                row_rights.push(ui.min_rect().right());

                                avail_widths.push(ui.available_width());
                                stat_row(ui, "Limit", "Lateral Accel");
                                row_rights.push(ui.min_rect().right());

                                avail_widths.push(ui.available_width());
                            },
                        );
                        sibling_avail_w = ui.available_width();
                    });
                });
                out.drop_without_applying_deltas();
            }

            assert!(
                worst_text_w + 4.0 <= VAL_W,
                "9-char monospace unit readout ({worst_text_w:.1} + 4.0 px pad) must fit inside VAL_W={VAL_W}"
            );
            assert_eq!(avail_widths.len(), 8);
            assert_eq!(row_rights.len(), 7);
            let first_w = avail_widths[0];
            for (idx, &w) in avail_widths.iter().enumerate() {
                assert!(
                    (w - first_w).abs() <= 1e-3,
                    "panel_w={panel_w}: available_width() drifted at row {idx}: {first_w} -> {w}"
                );
            }
            for (idx, &right) in row_rights.iter().enumerate() {
                assert!(
                    right <= inner_w + 0.5,
                    "panel_w={panel_w}: row {idx} rect.right()={right} exceeded inner_w={inner_w}"
                );
            }
            assert!(
                sibling_avail_w <= inner_w + 0.5,
                "panel_w={panel_w}: sibling section available_width()={sibling_avail_w} exceeded inner_w={inner_w}"
            );
        }
    }

    #[test]
    fn scene_build_progress_overlay_renders_within_viewport_all_themes() {
        let now = std::time::Instant::now();
        let indeterminate = SceneBuildProgress::new(
            EnginePhase::BuildingCsgScene,
            "Gundo OSM",
            "Generating CSG scene...",
            0,
            0,
            now,
        );
        let determinate = SceneBuildProgress::new(
            EnginePhase::BuildingCsgScene,
            "Gundo OSM",
            "Building OptiX GAS",
            112,
            246,
            now,
        )
        .with_step(4, 12)
        .with_overall(Some(0.62))
        .with_eta(Some(8.6));
        let warming_up = SceneBuildProgress::new(
            EnginePhase::BuildingCsgScene,
            "Gundo",
            "Baking road tiles",
            3,
            32,
            now,
        )
        .with_step(2, 10)
        .with_overall(Some(0.14));

        for theme in UiTheme::ALL {
            let ctx = egui::Context::default();
            install_fonts(&ctx);
            let _dock_style = theme.apply(&ctx);

            for (vp_w, vp_h) in [(48.0_f32, 48.0_f32), (220.0, 160.0), (1280.0, 720.0)] {
                let vp_rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(vp_w, vp_h));
                for (phase, progress, time) in [
                    (EnginePhase::CompilingOptix, None, 0.25_f64),
                    (EnginePhase::BuildingCsgScene, Some(&indeterminate), 0.75_f64),
                    (EnginePhase::BuildingCsgScene, Some(&warming_up), 1.0_f64),
                    (EnginePhase::BuildingCsgScene, Some(&determinate), 1.25_f64),
                ] {
                    let mut card_rect = egui::Rect::NOTHING;
                    let raw = egui::RawInput {
                        screen_rect: Some(vp_rect),
                        time: Some(time),
                        ..Default::default()
                    };
                    let out = ctx.run_ui(raw, |ui| {
                        card_rect = draw_scene_build_progress_overlay(
                            ui,
                            vp_rect,
                            theme,
                            phase,
                            progress,
                            now,
                        );
                    });
                    out.drop_without_applying_deltas();
                    assert!(
                        vp_rect.contains_rect(card_rect),
                        "{theme:?} {vp_w}x{vp_h} {phase:?}: card {card_rect:?} must stay within {vp_rect:?}"
                    );
                    if vp_w >= 220.0 && vp_h >= 160.0 {
                        assert!(card_rect.width() >= 180.0 && card_rect.height() >= 70.0);
                    }
                }
            }
        }
    }

    #[test]
    fn camera_stats_section_registry_and_defaults_cover_all_seven_sections() {
        let expected = [
            "sec_view_possess",
            "sec_render_stats",
            "sec_gpu_hw",
            "sec_csg_stats",
            "sec_crowd_stats",
            "sec_traffic_stats",
            "sec_perf",
        ];
        let fallback_accent = section_accent("sec_unknown_fallback_sentinel");
        for id in expected {
            let group = section_group_for(id);
            assert_eq!(
                group,
                &expected[..],
                "section `{id}` must share the 7-section Camera & Stats collapse/expand group"
            );
            assert_ne!(
                section_accent(id),
                fallback_accent,
                "section `{id}` must register a dedicated accent color"
            );
            let want_open = id != "sec_gpu_hw";
            assert_eq!(
                section_default_open(id),
                want_open,
                "section `{id}` default open state mismatch"
            );
        }
    }

    #[test]
    fn sectioned_camera_stats_bars_and_sparklines_fit_narrow_docks() {
        let sim = SimTelemetry {
            crowd_backend: ActiveCrowdBackend::Cuda,
            crowd_plan_version: 2,
            crowd_kernel_count: 11,
            crowd_actors: 184,
            crowd_archetypes: [2, 166, 4, 12],
            crowd_bones: 2944,
            crowd_joints: 2944,
            crowd_skeleton_edges: 2760,
            crowd_dofs: 4416,
            crowd_walkways: 32,
            crowd_obstacles: 140,
            crowd_step_ms: 0.24,
            crowd_gpu_bytes: 4 * 1024 * 1024 + 262_144,
            pose_frozen: false,
            sim_rate: 1.0,
            crowd_lod_dists_m: [22.0, 75.0, 180.0],
            crowd_lod_mode: 0,
            crowd_ground_misses: 0,
            crowd_gpu_fault: 0,
            crowd_ias_rebuilds: 2,
            gpu_camera_active: true,
            traffic_backend: ActiveTrafficBackend::Cuda,
            traffic_plan_version: 1,
            traffic_kernel_count: 14,
            traffic_gpu_bytes: 1_572_864,
            traffic_has_lane_graph: true,
            traffic_stepping: true,
            traffic_vehicles: 48,
            traffic_moving: 36,
            traffic_avg_speed_mps: 9.2,
            traffic_max_speed_mps: 15.8,
            traffic_dyn_instances: 240,
            traffic_step_ms: 0.14,
            traffic_respawns: 12,
            traffic_stale_releases: 0,
            traffic_deadlock_relocations: 1,
            traffic_aeb_active: 1,
            traffic_yield_active: 3,
            traffic_ias_rebuilds: 4,
            crowd_strict_f32: false,
        };
        let csg = CsgStats {
            visible_instances: 1620,
            total_instances: 2400,
            compounds: 246,
            primitives: 14_820,
            gpu_bytes: 18_432_000,
            aux_planes_bytes: 2_621_440,
            lod0_count: 410,
            lod1_count: 680,
            lod2_count: 530,
            culled_count: 780,
            path_instances: [820, 460, 210, 130],
        };
        let csg_empty_paths = CsgStats {
            path_instances: [0; 4],
            ..csg
        };
        let gpu_hw_cases = [
            GpuTelemetry {
                gpu_name: "NVIDIA GeForce RTX 3080 Ti".to_owned(),
                temp_c: 64,
                util_gpu: 62,
                vram_used_mib: 5_200,
                vram_total_mib: 12_288,
                power_w: 240.0,
                gpu_clock_mhz: 1920,
                mem_clock_mhz: 9501,
                cupti_status: "Active".to_owned(),
                ngx_status: "DLSS 3.7 RR".to_owned(),
                ..GpuTelemetry::default()
            },
            GpuTelemetry {
                gpu_name: "RTX Amber".to_owned(),
                temp_c: 78,
                util_gpu: 86,
                vram_used_mib: 10_000,
                vram_total_mib: 12_288,
                power_w: 300.0,
                ..GpuTelemetry::default()
            },
            GpuTelemetry {
                gpu_name: "RTX Red".to_owned(),
                temp_c: 88,
                util_gpu: 98,
                vram_used_mib: 11_500,
                vram_total_mib: 12_288,
                power_w: 420.0,
                ..GpuTelemetry::default()
            },
        ];

        let mut timings = FrameTimings::new();
        for i in 0..40 {
            let t = i as f32;
            timings.record_with_engines(
                8.5 + (t * 0.25).sin() * 1.2,
                1.4,
                0.45,
                0.32,
                8.5 + (t * 0.25).sin() * 1.2,
                1.4,
                0.9,
                60.0 + (t * 0.2).cos() * 4.0,
            );
            let mut s = sim;
            s.crowd_step_ms = 0.20 + (t * 0.3).sin().abs() * 0.15;
            s.traffic_step_ms = 0.11 + (t * 0.2).cos().abs() * 0.08;
            s.traffic_avg_speed_mps = 8.0 + (t * 0.15).sin() * 3.0;
            s.traffic_aeb_active = u32::from(i % 7 == 0);
            s.traffic_yield_active = (i % 4) as u32;
            timings.record_sim(&s, 60.0);
        }

        let empty_ring: RingBuffer<HISTORY_LEN> = RingBuffer::new();
        let mut single_ring: RingBuffer<HISTORY_LEN> = RingBuffer::new();
        single_ring.push(12.5);
        let mut flat_zero_ring: RingBuffer<HISTORY_LEN> = RingBuffer::new();
        let mut flat_const_ring: RingBuffer<HISTORY_LEN> = RingBuffer::new();
        for _ in 0..8 {
            flat_zero_ring.push(0.0);
            flat_const_ring.push(60.0);
        }

        for theme in [UiTheme::Obsidian, UiTheme::Light] {
            for panel_w in [240.0_f32, 280.0, 340.0] {
                let ctx = egui::Context::default();
                install_fonts(&ctx);
                let _dock_style = theme.apply(&ctx);
                let mut label_buf = String::with_capacity(96);
                let mut tip_buf = String::with_capacity(256);
                let mut avail_samples = Vec::new();
                let mut max_right = 0.0_f32;

                for _pass in 0..2 {
                    avail_samples.clear();
                    max_right = 0.0;
                    let raw = egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(panel_w, 1600.0),
                        )),
                        ..Default::default()
                    };
                    let out = ctx.run_ui(raw, |ui| {
                        dock_scroll(ui, |ui| {
                            collapsing_section(
                                ui,
                                "sec_gpu_hw_open_test",
                                ic::CPU,
                                "GPU & Hardware",
                                |ui| {
                                    // Force open in test so VRAM gauge bar runs across all thresholds.
                                    let _ = ui.available_width();
                                },
                            );
                            let start_w = ui.available_width();
                            avail_samples.push(start_w);
                            collapsing_section(
                                ui,
                                "sec_render_stats",
                                ic::MONITOR_PLAY,
                                "Render & Frame",
                                |ui| {
                                    let w0 = ui.available_width();
                                    render_accum_bar(ui, 12, 64, 48, &mut label_buf, &mut tip_buf);
                                    render_accum_bar(ui, 64, 64, 256, &mut label_buf, &mut tip_buf);
                                    render_accum_bar(ui, 18, 0, 72, &mut label_buf, &mut tip_buf);
                                    avail_samples.push(ui.available_width() - w0 + start_w);
                                    max_right = max_right.max(ui.min_rect().right());
                                },
                            );
                            for hw in &gpu_hw_cases {
                                render_gpu_hw_body(ui, hw, &mut label_buf, &mut tip_buf);
                                avail_samples.push(ui.available_width());
                                max_right = max_right.max(ui.min_rect().right());
                            }
                            collapsing_section(
                                ui,
                                "sec_csg_stats",
                                ic::CUBE,
                                "CSG Scene & LOD",
                                |ui| {
                                    let w0 = ui.available_width();
                                    render_csg_stats_body(ui, &csg, &mut label_buf, &mut tip_buf);
                                    render_csg_stats_body(
                                        ui,
                                        &csg_empty_paths,
                                        &mut label_buf,
                                        &mut tip_buf,
                                    );
                                    avail_samples.push(ui.available_width() - w0 + start_w);
                                    max_right = max_right.max(ui.min_rect().right());
                                },
                            );
                            collapsing_section(
                                ui,
                                "sec_crowd_stats",
                                ic::USERS_THREE,
                                "Crowd & Locomotion",
                                |ui| {
                                    let w0 = ui.available_width();
                                    render_crowd_stats_body(
                                        ui,
                                        &sim,
                                        &timings,
                                        60.0,
                                        0.95,
                                        &mut label_buf,
                                        &mut tip_buf,
                                    );
                                    avail_samples.push(ui.available_width() - w0 + start_w);
                                    max_right = max_right.max(ui.min_rect().right());
                                },
                            );
                            collapsing_section(
                                ui,
                                "sec_traffic_stats",
                                ic::CAR,
                                "Traffic & ADAS",
                                |ui| {
                                    let w0 = ui.available_width();
                                    render_traffic_stats_body(
                                        ui,
                                        &sim,
                                        &timings,
                                        &mut label_buf,
                                        &mut tip_buf,
                                    );
                                    avail_samples.push(ui.available_width() - w0 + start_w);
                                    max_right = max_right.max(ui.min_rect().right());
                                },
                            );
                            collapsing_section(
                                ui,
                                "sec_perf",
                                ic::CHART_LINE,
                                "Performance",
                                |ui| {
                                    let w0 = ui.available_width();
                                    render_perf_frame_split_bar(
                                        ui,
                                        8.5,
                                        1.4,
                                        0.9,
                                        &mut label_buf,
                                        &mut tip_buf,
                                    );
                                    render_perf_frame_split_bar(
                                        ui,
                                        0.0,
                                        0.0,
                                        0.0,
                                        &mut label_buf,
                                        &mut tip_buf,
                                    );
                                    sparkline_row(
                                        ui,
                                        &mut label_buf,
                                        &mut tip_buf,
                                        SparklineMeta {
                                            label: "Empty",
                                            desc: "Empty sparkline edge case.",
                                            unit: "ms",
                                        },
                                        &empty_ring,
                                        Color32::from_rgb(138, 173, 244),
                                    );
                                    sparkline_row(
                                        ui,
                                        &mut label_buf,
                                        &mut tip_buf,
                                        SparklineMeta {
                                            label: "Single",
                                            desc: "Single-sample sparkline edge case.",
                                            unit: "k",
                                        },
                                        &single_ring,
                                        Color32::from_rgb(166, 209, 137),
                                    );
                                    sparkline_row(
                                        ui,
                                        &mut label_buf,
                                        &mut tip_buf,
                                        SparklineMeta {
                                            label: "Zero",
                                            desc: "All-zero flat sparkline edge case.",
                                            unit: "",
                                        },
                                        &flat_zero_ring,
                                        Color32::from_rgb(238, 112, 92),
                                    );
                                    sparkline_row(
                                        ui,
                                        &mut label_buf,
                                        &mut tip_buf,
                                        SparklineMeta {
                                            label: "Const",
                                            desc: "Constant non-zero sparkline edge case.",
                                            unit: "km/h",
                                        },
                                        &flat_const_ring,
                                        Color32::from_rgb(122, 206, 138),
                                    );
                                    stacked_bar_row(
                                        ui,
                                        "Tiny Pulse",
                                        &[
                                            BarSegment {
                                                count: 999,
                                                color: Color32::from_rgb(122, 206, 138),
                                            },
                                            BarSegment {
                                                count: 1,
                                                color: Color32::from_rgb(236, 78, 86),
                                            },
                                        ],
                                        Some("1/1000"),
                                        &mut tip_buf,
                                        |t| t.push_str("1 AEB event out of 1000"),
                                    );
                                    stacked_bar_row(
                                        ui,
                                        "Zero Bar",
                                        &[BarSegment {
                                            count: 0,
                                            color: Color32::from_rgb(122, 206, 138),
                                        }],
                                        None,
                                        &mut tip_buf,
                                        |t| t.push_str("empty"),
                                    );
                                    avail_samples.push(ui.available_width() - w0 + start_w);
                                    max_right = max_right.max(ui.min_rect().right());
                                },
                            );
                        });
                    });
                    out.drop_without_applying_deltas();
                }

                let baseline_w = avail_samples[0];
                for (idx, &w) in avail_samples.iter().enumerate() {
                    assert!(
                        (w - baseline_w).abs() <= 0.5,
                        "{theme:?} width {panel_w}: available_width drifted at stage {idx} ({baseline_w} -> {w})"
                    );
                }
                assert!(
                    max_right <= panel_w + 0.5,
                    "{theme:?} width {panel_w}: sectioned Camera & Stats overflowed right edge ({max_right} > {panel_w})"
                );
            }
        }
    }

    #[test]
    fn ocean_water_controls_fit_narrow_docks_and_register_section() {
        let fallback_accent = section_accent("sec_unknown_fallback_sentinel");
        assert_ne!(section_accent("sec_ocean"), fallback_accent);
        assert!(section_group_for("sec_ocean").contains(&"sec_ocean"));
        assert!(section_group_for("sec_ocean").contains(&"sec_sky"));
        assert!(section_default_open("sec_ocean"));

        for theme in [UiTheme::Obsidian, UiTheme::Light] {
            for panel_w in [240.0_f32, 280.0, 340.0] {
                let ctx = egui::Context::default();
                install_fonts(&ctx);
                let _dock_style = theme.apply(&ctx);
                let mut ocean = OceanParams::default();
                let mut avail_before = 0.0_f32;
                let mut avail_after = 0.0_f32;
                let mut max_right = 0.0_f32;

                for _pass in 0..2 {
                    let raw = egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(panel_w, 600.0),
                        )),
                        ..Default::default()
                    };
                    let out = ctx.run_ui(raw, |ui| {
                        dock_scroll(ui, |ui| {
                            collapsing_section(ui, "sec_ocean", ic::WAVES, "Ocean Water", |ui| {
                                avail_before = ui.available_width();
                                render_ocean_water_controls(ui, &mut ocean);
                                avail_after = ui.available_width();
                                max_right = ui.min_rect().right();
                            });
                        });
                    });
                    out.drop_without_applying_deltas();
                }

                assert!(
                    (avail_after - avail_before).abs() <= 0.5,
                    "{theme:?} width {panel_w}: ocean controls drifted available_width ({avail_before} -> {avail_after})"
                );
                assert!(
                    max_right <= panel_w + 0.5,
                    "{theme:?} width {panel_w}: ocean controls overflowed right edge ({max_right} > {panel_w})"
                );
            }
        }
    }

    #[test]
    fn parametric_asset_inspector_and_disk_cache_fit_narrow_docks() {
        let fallback_accent = section_accent("sec_unknown_fallback_sentinel");
        assert_ne!(section_accent("sec_asset_inspector"), fallback_accent);
        assert!(section_group_for("sec_asset_inspector").contains(&"sec_asset_inspector"));
        assert!(section_default_open("sec_asset_inspector"));

        let cache_cases = [
            SceneCacheStats {
                entries: 4,
                total_bytes: 48 * 1024 * 1024,
                last_hit: true,
                last_source: SceneLoadSource::DiskHit,
            },
            SceneCacheStats {
                entries: 2,
                total_bytes: 16 * 1024 * 1024,
                last_hit: true,
                last_source: SceneLoadSource::MemoryHit,
            },
            SceneCacheStats {
                entries: 1,
                total_bytes: 8 * 1024 * 1024,
                last_hit: false,
                last_source: SceneLoadSource::Built,
            },
        ];

        for theme in [UiTheme::Obsidian, UiTheme::Light] {
            for panel_w in [260.0_f32, 320.0] {
                let ctx = egui::Context::default();
                install_fonts(&ctx);
                let _dock_style = theme.apply(&ctx);
                let mut asset_idx = 0u32;
                let mut asset_params = csg_scenes::inspector_asset_kind(asset_idx)
                    .map_or_else(AssetParamMap::new, |k| k.default_params(0));
                let mut browser = AssetBrowserState::default();
                let mut label_buf = String::with_capacity(96);
                let mut tip_buf = String::with_capacity(256);
                let mut avail_before = 0.0_f32;
                let mut avail_after = 0.0_f32;
                let mut max_right = 0.0_f32;

                for _pass in 0..2 {
                    let raw = egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(panel_w, 1200.0),
                        )),
                        ..Default::default()
                    };
                    let out = ctx.run_ui(raw, |ui| {
                        dock_scroll(ui, |ui| {
                            collapsing_section(
                                ui,
                                "sec_asset_inspector",
                                ic::SLIDERS_HORIZONTAL,
                                "Asset Inspector",
                                |ui| {
                                    avail_before = ui.available_width();
                                    for &stats in &cache_cases {
                                        let _ = render_scene_cache_row(
                                            ui,
                                            stats,
                                            &mut label_buf,
                                            &mut tip_buf,
                                        );
                                    }
                                    let _ = render_asset_inspector_body(
                                        ui,
                                        &mut asset_idx,
                                        &mut asset_params,
                                        &mut browser,
                                        &mut label_buf,
                                        &mut tip_buf,
                                    );
                                    avail_after = ui.available_width();
                                    max_right = ui.min_rect().right();
                                },
                            );
                        });
                    });
                    out.drop_without_applying_deltas();
                }

                assert!(
                    (avail_after - avail_before).abs() <= 0.5,
                    "{theme:?} width {panel_w}: asset inspector drifted available_width ({avail_before} -> {avail_after})"
                );
                assert!(
                    max_right <= panel_w + 0.5,
                    "{theme:?} width {panel_w}: asset inspector overflowed right edge ({max_right} > {panel_w})"
                );
            }
        }
    }

    #[test]
    fn possess_adas_pill_renders_cleanly_for_all_severities() {
        use csg_scene::traffic::{hazard_flags, hazard_state};
        let cases = [
            PossessAdas {
                state: hazard_state::CRUISE,
                speed_mps: 11.5,
                accel_mps2: 1.2,
                stop_dist_m: 18.0,
                hazard_dist_m: f32::INFINITY,
                limit_mps: 13.9,
                ..PossessAdas::default()
            },
            PossessAdas {
                state: hazard_state::PED_YIELD,
                flags: hazard_flags::BRAKE_LIGHT | hazard_flags::HAZARD_ACTOR_VALID,
                hazard_actor_idx: 4,
                speed_mps: 4.0,
                accel_mps2: -2.4,
                stop_dist_m: 6.0,
                hazard_dist_m: 8.2,
                limit_mps: 11.1,
            },
            PossessAdas {
                state: hazard_state::PED_AEB,
                flags: hazard_flags::BRAKE_LIGHT | hazard_flags::HAZARD_ACTOR_VALID,
                hazard_actor_idx: 9,
                speed_mps: 6.0,
                accel_mps2: -7.5,
                stop_dist_m: 9.0,
                hazard_dist_m: 3.1,
                limit_mps: 11.1,
            },
        ];
        let ctx = egui::Context::default();
        install_fonts(&ctx);
        let _dock_style = UiTheme::Obsidian.apply(&ctx);
        for adas in cases {
            let raw = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(360.0, 64.0),
                )),
                ..Default::default()
            };
            let mut pill_w = 0.0_f32;
            let out = ctx.run_ui(raw, |ui| {
                ui.horizontal(|ui| {
                    render_possess_adas_pill(ui, adas);
                    pill_w = ui.min_rect().width();
                });
            });
            out.drop_without_applying_deltas();
            assert!(
                pill_w > 60.0 && pill_w <= 340.0,
                "unexpected PossessAdas pill width {pill_w} for {}",
                adas.badge_label()
            );
        }
    }
}
