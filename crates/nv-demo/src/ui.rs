//! Docked egui UI with Phosphor icons and uniformly aligned property grids.

use std::ops::RangeInclusive;

use egui::{Align, Color32, FontDefinitions, Layout, RichText, Sense, Ui};
use egui_dock::{DockState, NodeIndex, Style, TabViewer};
use egui_phosphor::regular as ic;

use crate::camera_core::input::NavScheme;
use crate::camera_core::{CameraDriver, CameraIntent, CameraPose, NavFeel, NavMode, ViewportSize};
use crate::profiler::{FrameTimings, GpuTelemetry, HISTORY_LEN, RingBuffer};
use crate::renderer::DemoSettings;

const LABEL_W: f32 = 86.0;
const VAL_W: f32 = 64.0;
const ROW_H: f32 = 20.0;

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
    ctx.set_fonts(fonts);
}

pub fn default_dock_state() -> DockState<DockTab> {
    let mut state = DockState::new(vec![DockTab::Viewport]);
    let surface = state.main_surface_mut();
    let [_vp, right] = surface.split_right(NodeIndex::root(), 0.74, vec![DockTab::Fractal]);
    let [_top, _bot] = surface.split_below(
        right,
        0.54,
        vec![DockTab::RenderFloor, DockTab::CameraStats],
    );
    state
}

pub fn dock_style(ctx: &egui::Context) -> Style {
    let mut style = Style::from_egui(ctx.style_of(egui::Theme::Dark).as_ref());
    style.tab_bar.height = 26.0;
    style.tab_bar.bg_fill = Color32::from_rgb(20, 22, 26);
    style.tab.tab_body.bg_fill = Color32::from_rgb(24, 26, 31);
    style.separator.width = 3.0;
    style
}

pub struct UiContext<'a> {
    pub settings: &'a mut DemoSettings,
    pub display_tex_id: egui::TextureId,
    pub render_w: u32,
    pub render_h: u32,
    pub desired_vp_size: &'a mut (f32, f32),
    pub cam_driver: &'a mut CameraDriver,
    pub cam_home: CameraPose,
    pub cam_pose: &'a mut CameraPose,
    pub cam_mode: &'a mut NavMode,
    pub cam_fly_mode: &'a mut NavMode,
    pub cam_scheme: &'a mut NavScheme,
    pub cam_feel: &'a mut NavFeel,
    pub telemetry: &'a GpuTelemetry,
    pub timings: &'a FrameTimings,
    pub accum_count: u32,
    pub total_samples: u32,
    pub rt_ms: f32,
    pub denoise_ms: f32,
    pub bloom_ms: f32,
    pub tonemap_ms: f32,
    pub fps: f32,
    pub is_recording: bool,
    pub record_frames: u64,
    pub viewport_focused: &'a mut bool,
    pub reset_accum_requested: &'a mut bool,
    pub save_png_requested: &'a mut bool,
    pub save_jpeg_requested: &'a mut bool,
    pub toggle_record_requested: &'a mut bool,
    pub reload_shader_requested: &'a mut bool,
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
            DockTab::Fractal => {
                egui::ScrollArea::vertical().show(ui, |ui| self.ui_fractal(ui));
            }
            DockTab::RenderFloor => {
                egui::ScrollArea::vertical().show(ui, |ui| self.ui_render_floor(ui));
            }
            DockTab::CameraStats => {
                egui::ScrollArea::vertical().show(ui, |ui| self.ui_camera_stats(ui));
            }
        }
    }
}

impl UiContext<'_> {
    fn ui_viewport(&mut self, ui: &mut Ui) {
        let avail = ui.available_size();
        let ppp = ui.ctx().pixels_per_point();
        *self.desired_vp_size = (
            (avail.x * ppp).max(256.0),
            (avail.y * ppp).max(144.0),
        );
        let aspect = self.render_w as f32 / self.render_h as f32;
        let w = avail.x.min(avail.y * aspect).max(64.0);
        let h = (w / aspect).max(64.0);
        let (rect, _) = ui.allocate_exact_size(avail, Sense::hover());
        let img_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(w, h));
        ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(12, 13, 16));
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
            ui.painter()
                .rect_filled(knob, 4.0, Color32::from_rgba_unmultiplied(235, 240, 245, 220));
        }

        let resp = ui.interact(rect, ui.id().with("optix-vp"), Sense::click_and_drag());
        *self.viewport_focused = (resp.hovered() || resp.dragged()) && !split_dragging;
        if !split_dragging {
            let df = crate::camera_egui::drive(ui, &resp, self.cam_driver, 2.0);
            let kb_free = !ui.ctx().egui_wants_keyboard_input();
            let rmb_down = ui.input(|i| i.pointer.secondary_down());
            let f_pressed =
                kb_free && *self.viewport_focused && ui.input(|i| i.key_pressed(egui::Key::F));
            let a_pressed = kb_free
                && *self.viewport_focused
                && !rmb_down
                && ui.input(|i| i.key_pressed(egui::Key::A));

            if df.requests.frame_selected || f_pressed {
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
            } else if a_pressed {
                let ext = 1.35 * self.settings.obj_scale.max(0.25);
                self.cam_driver.controller_mut().apply_intent(
                    CameraIntent::FrameBounds {
                        min: glam::Vec3::splat(-ext),
                        max: glam::Vec3::splat(ext),
                        margin: 1.2,
                    },
                    1.0 / 60.0,
                    ViewportSize::new(self.render_w, self.render_h),
                );
                *self.cam_pose = self.cam_driver.controller().pose();
            } else if df.requests.home {
                self.cam_driver.controller_mut().set_pose(self.cam_home);
                *self.cam_pose = self.cam_home;
            } else {
                *self.cam_pose = df.pose;
            }
        }

        // Compact bottom-left status badge inside viewport
        let hud_pos = rect.left_bottom() + egui::vec2(10.0, -26.0);
        let aov_tag = match self.settings.aov_mode {
            1 => "ALBEDO",
            2 => "NORMAL",
            3 => "DEPTH",
            4 => "MOTION",
            _ => "BEAUTY",
        };
        let status = if self.settings.auto_rotate {
            format!(
                "{} SPIN  |  {}x{}  |  {}  |  {} FPS  |  {:.1} ms RT",
                ic::ARROWS_CLOCKWISE,
                self.render_w,
                self.render_h,
                aov_tag,
                self.fps.round() as u32,
                self.rt_ms
            )
        } else {
            format!(
                "{} {}/{} spp  |  {}x{}  |  {}  |  {} FPS  |  {:.1} ms RT",
                ic::STACK,
                self.total_samples,
                self.settings.max_accum.saturating_mul(self.settings.spp),
                self.render_w,
                self.render_h,
                aov_tag,
                self.fps.round() as u32,
                self.rt_ms
            )
        };
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
    }

    fn ui_fractal(&mut self, ui: &mut Ui) {
        let s = &mut *self.settings;
        section_header(ui, ic::SPARKLE, "Presets");
        equal_grid(ui, 2, 4, |ui, idx, w| match idx {
            0 => {
                if ui.add_sized([w, ROW_H + 2.0], egui::Button::new("Gold Bulb")).clicked() {
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
                if ui.add_sized([w, ROW_H + 2.0], egui::Button::new("Chrome Box")).clicked() {
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
                if ui.add_sized([w, ROW_H + 2.0], egui::Button::new("Obsidian KIFS")).clicked() {
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
                if ui.add_sized([w, ROW_H + 2.0], egui::Button::new("Julia Coral")).clicked() {
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
        ui.add_space(6.0);

        section_header(ui, ic::CUBE, "Formula");
        equal_grid(ui, 3, 3, |ui, idx, w| {
            let (kind, label) = match idx {
                0 => (0u32, "Mandelbulb"),
                1 => (1u32, "Mandelbox"),
                _ => (2u32, "KIFS Octa"),
            };
            if ui
                .add_sized([w, ROW_H + 2.0], egui::Button::selectable(s.fractal_kind == kind, label))
                .clicked()
            {
                s.fractal_kind = kind;
            }
        });
        ui.add_space(4.0);

        match s.fractal_kind {
            0 => {
                prop_slider_f32(ui, "Power", &mut s.bulb_power, 2.0..=16.0, 2, false);
                prop_slider_f32(ui, "Bailout", &mut s.bulb_bailout, 2.0..=32.0, 1, false);
                prop_slider_u32(ui, "Iterations", &mut s.bulb_iter, 4..=24, false);
                prop_slider_f32(ui, "Theta Mult", &mut s.bulb_theta_mult, 0.25..=2.5, 2, false);
                prop_slider_f32(ui, "Phi Mult", &mut s.bulb_phi_mult, 0.25..=2.5, 2, false);
                prop_slider_f32(ui, "Theta Ph", &mut s.bulb_theta_phase, -90.0..=90.0, 1, false);
                prop_slider_f32(ui, "Phi Ph", &mut s.bulb_phi_phase, -180.0..=180.0, 1, false);
                prop_toggle(ui, "Julia Mode", &mut s.bulb_julia, "Enable Julia C");
                if s.bulb_julia {
                    prop_vec3(ui, "Julia C", &mut s.bulb_c, -1.5..=1.5, 0.01);
                }
            }
            1 => {
                prop_slider_f32(ui, "Scale", &mut s.box_scale, -3.5..=3.5, 2, false);
                prop_slider_f32(ui, "Fold", &mut s.box_fold, 0.2..=2.2, 2, false);
                prop_slider_f32(ui, "Fold Mult", &mut s.box_fold_mult, 1.2..=3.0, 2, false);
                prop_slider_f32(ui, "Min R", &mut s.box_min_r, 0.05..=1.2, 2, false);
                prop_slider_f32(ui, "Fixed R", &mut s.box_fixed_r, 0.4..=2.5, 2, false);
                prop_slider_f32(ui, "Iter Rot", &mut s.box_iter_rot, -45.0..=45.0, 1, false);
                prop_slider_u32(ui, "Iterations", &mut s.box_iter, 4..=24, false);
                prop_slider_f32(ui, "Bailout", &mut s.box_bailout, 4.0..=64.0, 1, false);
            }
            _ => {
                prop_slider_f32(ui, "Scale", &mut s.kifs_scale, 1.3..=2.8, 2, false);
                prop_slider_f32(ui, "Rot Y", &mut s.kifs_rot1, -180.0..=180.0, 1, false);
                prop_slider_f32(ui, "Rot X", &mut s.kifs_rot2, -180.0..=180.0, 1, false);
                prop_vec3(ui, "Offset", &mut s.kifs_offset, 0.1..=2.0, 0.01);
                prop_slider_f32(ui, "Thickness", &mut s.kifs_thickness, 0.005..=0.25, 3, false);
                prop_slider_f32(ui, "Z Bias", &mut s.kifs_z_bias, 0.0..=1.5, 2, false);
                prop_slider_u32(ui, "Iterations", &mut s.kifs_iter, 4..=24, false);
                prop_slider_f32(ui, "Bailout", &mut s.kifs_bailout, 4.0..=32.0, 1, false);
            }
        }

        ui.add_space(6.0);
        section_header(ui, ic::ARROWS_OUT_CARDINAL, "Transform");
        prop_slider_f32(ui, "Obj Scale", &mut s.obj_scale, 0.25..=2.5, 2, false);
        prop_slider_f32(ui, "Rot X", &mut s.rot_x, -180.0..=180.0, 1, false);
        prop_slider_f32(ui, "Rot Y", &mut s.rot_y, -180.0..=180.0, 1, false);
        prop_slider_f32(ui, "Rot Z", &mut s.rot_z, -180.0..=180.0, 1, false);

        ui.add_space(6.0);
        section_header(ui, ic::SLIDERS_HORIZONTAL, "Detail & Shading");
        prop_slider_f32(ui, "Detail Eps", &mut s.detail_eps, 0.00005..=0.005, 5, true);
        prop_slider_f32(ui, "Normal Eps", &mut s.norm_eps, 0.00005..=0.005, 5, true);
        prop_slider_u32(ui, "March Steps", &mut s.march_steps, 32..=320, false);
        prop_slider_f32(ui, "Fudge", &mut s.fudge, 0.3..=1.1, 2, false);
        prop_slider_u32(ui, "Bounces", &mut s.bounces, 1..=6, false);
        prop_slider_f32(ui, "Metallic", &mut s.metallic, 0.0..=1.0, 2, false);
        prop_slider_f32(ui, "Roughness", &mut s.roughness, 0.02..=0.85, 2, false);
        prop_slider_f32(ui, "Trap Scale", &mut s.trap_scale, 0.1..=3.0, 2, false);
        prop_slider_f32(ui, "Hue Shift", &mut s.palette_shift, 0.0..=1.0, 2, false);
    }

    fn ui_render_floor(&mut self, ui: &mut Ui) {
        let s = &mut *self.settings;

        section_header(ui, ic::SUN, "Sky & Lighting");
        prop_slider_f32(ui, "Sun Power", &mut s.sun_intensity, 0.0..=4.0, 2, false);
        prop_slider_f32(ui, "Sun Azimuth", &mut s.sun_azimuth, -180.0..=180.0, 1, false);
        prop_slider_f32(ui, "Sun Elev", &mut s.sun_elevation, 2.0..=88.0, 1, false);
        prop_slider_f32(ui, "Sky Power", &mut s.sky_intensity, 0.1..=2.0, 2, false);
        prop_slider_f32(ui, "Haze", &mut s.horizon_haze, 0.4..=2.5, 2, false);
        prop_slider_f32(ui, "Exposure", &mut s.exposure, 0.25..=2.0, 2, false);

        ui.add_space(6.0);
        section_header(ui, ic::APERTURE, "Lens DoF");
        prop_slider_f32(ui, "Aperture", &mut s.aperture, 0.0..=0.12, 3, false);
        prop_slider_f32(ui, "Focus Dist", &mut s.focus_dist, 0.5..=12.0, 2, false);

        ui.add_space(6.0);
        section_header(ui, ic::SQUARES_FOUR, "Scratched Metal Floor");
        prop_slider_f32(ui, "Roughness", &mut s.floor_roughness, 0.03..=0.80, 2, false);
        prop_slider_f32(ui, "Scratches", &mut s.floor_scratch, 0.0..=1.5, 2, false);
        prop_slider_f32(ui, "Tile Scale", &mut s.floor_scale, 0.3..=5.0, 2, false);
        prop_slider_f32(ui, "Metallic", &mut s.floor_metallic, 0.0..=1.0, 2, false);

        ui.add_space(6.0);
        section_header(ui, ic::GLOBE, "Scene");
        prop_toggle(ui, "Glass Sat", &mut s.sat_on, "Dielectric Sphere");
        if s.sat_on {
            prop_toggle(ui, "Orbit", &mut s.sat_orbit, "Animate Orbit");
            prop_slider_f32(ui, "Sat Radius", &mut s.sat_radius, 0.12..=0.80, 2, false);
            prop_slider_f32(ui, "Glass IOR", &mut s.glass_ior, 1.05..=2.40, 2, false);
            prop_slider_f32(ui, "Absorb", &mut s.glass_absorb, 0.0..=2.50, 2, false);
        }
        prop_toggle(ui, "Neon Ring", &mut s.neon_on, "Emissive Accent");

        ui.add_space(6.0);
        section_header(ui, ic::SUN_DIM, "FFT Bloom (cuFFT)");
        prop_toggle(ui, "Bloom", &mut s.bloom_on, "Enable Diffraction Bloom");
        if s.bloom_on {
            prop_slider_f32(ui, "Threshold", &mut s.bloom_threshold, 0.2..=5.0, 2, false);
            prop_slider_f32(ui, "Intensity", &mut s.bloom_intensity, 0.0..=2.0, 2, false);
            prop_slider_u32(ui, "Blades", &mut s.bloom_blades, 3..=12, false);
        }

        ui.add_space(6.0);
        section_header(ui, ic::COLUMNS, "A/B Split");
        prop_toggle(ui, "Split View", &mut s.split_on, "Drag Handle in Viewport");
        if s.split_on {
            prop_slider_f32(ui, "Split X", &mut s.split_x, 0.02..=0.98, 2, false);
            prop_row(ui, "Right AOV", |ui| {
                let w = ((ui.available_width() - 8.0) / 3.0).max(36.0);
                ui.spacing_mut().item_spacing.x = 4.0;
                for (mode, lbl) in [(0u32, "Raw"), (1u32, "Albedo"), (2u32, "Normal")] {
                    if ui
                        .add_sized(
                            [w, ROW_H],
                            egui::Button::selectable(s.right_is_aov == mode, lbl),
                        )
                        .clicked()
                    {
                        s.right_is_aov = mode;
                    }
                }
            });
        }

        ui.add_space(6.0);
        section_header(ui, ic::SPARKLE, "Path Tracer");
        prop_row(ui, "Scale", |ui| {
            let w = ((ui.available_width() - 12.0) / 4.0).max(28.0);
            ui.spacing_mut().item_spacing.x = 4.0;
            for (val, lbl) in [(0.5, "0.5x"), (1.0, "1.0x"), (1.5, "1.5x"), (2.0, "2.0x")] {
                let selected = (s.render_scale - val).abs() < 0.05;
                if ui
                    .add_sized([w, ROW_H], egui::Button::selectable(selected, lbl))
                    .clicked()
                {
                    s.render_scale = val;
                }
            }
        });
        prop_row(ui, "AOV", |ui| {
            let w = ((ui.available_width() - 16.0) / 5.0).max(28.0);
            ui.spacing_mut().item_spacing.x = 4.0;
            for (mode, lbl) in [
                (0u32, "Beauty"),
                (1u32, "Albedo"),
                (2u32, "Normal"),
                (3u32, "Depth"),
                (4u32, "Motion"),
            ] {
                if ui
                    .add_sized([w, ROW_H], egui::Button::selectable(s.aov_mode == mode, lbl))
                    .clicked()
                {
                    s.aov_mode = mode;
                }
            }
        });
        prop_slider_u32(ui, "SPP / Pass", &mut s.spp, 1..=64, true);
        prop_slider_u32(ui, "Max Accum", &mut s.max_accum, 1..=1024, true);
        prop_slider_f32(ui, "Spin Speed", &mut s.spin_speed, 0.0..=0.5, 3, false);

        ui.add_space(4.0);
        equal_grid(ui, 2, 4, |ui, idx, w| match idx {
            0 => {
                ui.add_sized([w, ROW_H], egui::Checkbox::new(&mut s.shadows_on, "Shadows"));
            }
            1 => {
                ui.add_sized([w, ROW_H], egui::Checkbox::new(&mut s.denoiser_on, "AI Denoise"));
            }
            2 => {
                ui.add_sized(
                    [w, ROW_H],
                    egui::Checkbox::new(&mut s.temporal_denoise, "Temporal AI"),
                );
            }
            _ => {
                ui.add_sized([w, ROW_H], egui::Checkbox::new(&mut s.auto_rotate, "Spin"));
            }
        });
        ui.add_space(4.0);
        equal_grid(ui, 2, 4, |ui, idx, w| match idx {
            0 => {
                if ui
                    .add_sized(
                        [w, ROW_H + 2.0],
                        egui::Button::new(format!("{}  Reset Accum", ic::ARROWS_CLOCKWISE)),
                    )
                    .clicked()
                {
                    *self.reset_accum_requested = true;
                }
            }
            1 => {
                if ui
                    .add_sized(
                        [w, ROW_H + 2.0],
                        egui::Button::new(format!("{}  Save PNG", ic::FLOPPY_DISK)),
                    )
                    .clicked()
                {
                    *self.save_png_requested = true;
                }
            }
            2 => {
                if ui
                    .add_sized(
                        [w, ROW_H + 2.0],
                        egui::Button::new(format!("{}  Save JPEG", ic::IMAGE)),
                    )
                    .clicked()
                {
                    *self.save_jpeg_requested = true;
                }
            }
            _ => {
                let rec_lbl = if self.is_recording {
                    format!("{}  Stop ({}f)", ic::STOP, self.record_frames)
                } else {
                    format!("{}  Record H.264", ic::VIDEO_CAMERA)
                };
                if ui
                    .add_sized([w, ROW_H + 2.0], egui::Button::new(rec_lbl))
                    .clicked()
                {
                    *self.toggle_record_requested = true;
                }
            }
        });
        ui.add_space(3.0);
        if ui
            .add_sized(
                [ui.available_width(), ROW_H + 2.0],
                egui::Button::new(format!("{}  Hot-Reload Shader (NVRTC)", ic::LIGHTNING)),
            )
            .clicked()
        {
            *self.reload_shader_requested = true;
        }
        if let Some(msg) = self.last_saved_msg {
            ui.label(
                RichText::new(msg)
                    .size(11.0)
                    .color(Color32::from_rgb(166, 209, 137)),
            );
        }
    }

    fn ui_camera_stats(&mut self, ui: &mut Ui) {
        section_header(ui, ic::CAMERA, "Navigation (RMB = Fly)");
        let (prev_m, prev_fly, prev_s, prev_f) = (
            *self.cam_mode,
            *self.cam_fly_mode,
            *self.cam_scheme,
            *self.cam_feel,
        );

        prop_row(ui, "Orbit Mode", |ui| {
            egui::ComboBox::from_id_salt("cam_mode")
                .width(ui.available_width())
                .selected_text(self.cam_mode.label())
                .show_ui(ui, |ui| {
                    for m in NavMode::ALL.into_iter().filter(|m| !m.is_fly()) {
                        ui.selectable_value(self.cam_mode, m, m.label());
                    }
                });
        });
        prop_row(ui, "Fly Mode", |ui| {
            egui::ComboBox::from_id_salt("cam_fly")
                .width(ui.available_width())
                .selected_text(self.cam_fly_mode.label())
                .show_ui(ui, |ui| {
                    for m in NavMode::ALL.into_iter().filter(|m| m.is_fly()) {
                        ui.selectable_value(self.cam_fly_mode, m, m.label());
                    }
                });
        });
        prop_row(ui, "Scheme", |ui| {
            egui::ComboBox::from_id_salt("cam_scheme")
                .width(ui.available_width())
                .selected_text(format!("{:?}", *self.cam_scheme))
                .show_ui(ui, |ui| {
                    for s in NavScheme::ALL {
                        ui.selectable_value(self.cam_scheme, s, format!("{s:?}"));
                    }
                });
        });
        prop_row(ui, "Feel", |ui| {
            egui::ComboBox::from_id_salt("cam_feel")
                .width(ui.available_width())
                .selected_text(format!("{:?}", *self.cam_feel))
                .show_ui(ui, |ui| {
                    for f in NavFeel::ALL {
                        ui.selectable_value(self.cam_feel, f, format!("{f:?}"));
                    }
                });
        });

        if (*self.cam_mode, *self.cam_fly_mode, *self.cam_scheme, *self.cam_feel)
            != (prev_m, prev_fly, prev_s, prev_f)
        {
            self.cam_driver.controller_mut().set_mode(*self.cam_mode);
            let mut prefs = *self.cam_driver.prefs();
            prefs.hold_fly_mode = *self.cam_fly_mode;
            prefs.feel = *self.cam_feel;
            self.cam_driver.set_prefs(prefs);
            self.cam_driver.set_scheme(*self.cam_scheme);
        }

        ui.add_space(4.0);
        equal_grid(ui, 2, 2, |ui, idx, w| match idx {
            0 => {
                if ui
                    .add_sized([w, ROW_H + 2.0], egui::Button::new(format!("{}  Home", ic::HOUSE)))
                    .clicked()
                {
                    self.cam_driver.controller_mut().set_pose(self.cam_home);
                    *self.cam_pose = self.cam_home;
                }
            }
            _ => {
                if ui
                    .add_sized(
                        [w, ROW_H + 2.0],
                        egui::Button::new(format!("{}  Frame", ic::CROSSHAIR)),
                    )
                    .clicked()
                {
                    self.cam_driver.controller_mut().apply_intent(
                        CameraIntent::FrameBounds {
                            min: glam::Vec3::splat(-1.25),
                            max: glam::Vec3::splat(1.25),
                            margin: 1.2,
                        },
                        1.0 / 60.0,
                        ViewportSize::new(self.render_w, self.render_h),
                    );
                    *self.cam_pose = self.cam_driver.controller().pose();
                }
            }
        });

        ui.add_space(6.0);
        section_header(ui, ic::GAUGE, "Telemetry");
        stat_row(ui, "GPU", &self.telemetry.gpu_name);
        stat_row(
            ui,
            "Resolution",
            &format!("{} x {}", self.render_w, self.render_h),
        );
        stat_row(
            ui,
            "GUI / RT",
            &format!("{:.0} FPS / {:.1} ms", self.fps, self.rt_ms),
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
            &format!("{} C / {} %", self.telemetry.temp_c, self.telemetry.util_gpu),
        );
        stat_row(ui, "Power", &format!("{:.0} W", self.telemetry.power_w));
        stat_row(ui, "CUPTI", &self.telemetry.cupti_status);
        stat_row(ui, "NGX", &self.telemetry.ngx_status);

        ui.add_space(6.0);
        section_header(ui, ic::CHART_LINE, "Performance");
        sparkline_row(
            ui,
            "RT ms",
            &self.timings.rt_ms,
            Color32::from_rgb(138, 173, 244),
            "ms",
        );
        sparkline_row(
            ui,
            "Denoise ms",
            &self.timings.denoise_ms,
            Color32::from_rgb(166, 209, 137),
            "ms",
        );
        sparkline_row(
            ui,
            "Bloom ms",
            &self.timings.bloom_ms,
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
    }
}

fn section_header(ui: &mut Ui, icon: &str, title: &str) {
    ui.label(
        RichText::new(format!("{icon}  {title}"))
            .strong()
            .color(Color32::from_rgb(166, 209, 137)),
    );
    ui.add_space(2.0);
}

fn equal_grid(ui: &mut Ui, cols: usize, count: usize, mut item_ui: impl FnMut(&mut Ui, usize, f32)) {
    let gap = 4.0;
    let total_w = ui.available_width();
    let cell_w = ((total_w - gap * (cols.saturating_sub(1) as f32)) / (cols as f32)).max(24.0);
    let mut idx = 0;
    while idx < count {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            for _ in 0..cols {
                if idx < count {
                    item_ui(ui, idx, cell_w);
                    idx += 1;
                }
            }
        });
    }
}

fn prop_row(ui: &mut Ui, label: &str, add_contents: impl FnOnce(&mut Ui)) {
    let total_w = ui.available_width().max(LABEL_W + 80.0);
    let gap = 6.0;
    ui.allocate_ui_with_layout(
        egui::vec2(total_w, ROW_H),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = gap;
            let (label_rect, _) =
                ui.allocate_exact_size(egui::vec2(LABEL_W, ROW_H), Sense::hover());
            ui.painter().text(
                egui::pos2(label_rect.left(), label_rect.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                egui::FontId::proportional(12.0),
                Color32::from_rgb(188, 194, 206),
            );
            let rem_w = (total_w - LABEL_W - gap).max(60.0);
            ui.allocate_ui_with_layout(
                egui::vec2(rem_w, ROW_H),
                Layout::left_to_right(Align::Center),
                add_contents,
            );
        },
    );
}

fn prop_slider_f32(
    ui: &mut Ui,
    label: &str,
    val: &mut f32,
    range: RangeInclusive<f32>,
    decimals: usize,
    log: bool,
) {
    prop_row(ui, label, |ui| {
        let gap = 6.0;
        let track_w = (ui.available_width() - VAL_W - gap).max(40.0);
        ui.spacing_mut().item_spacing.x = gap;
        ui.spacing_mut().slider_width = track_w;
        let slider = egui::Slider::new(val, range.clone())
            .show_value(false)
            .logarithmic(log);
        ui.add_sized([track_w, ROW_H], slider);
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
        ui.add_sized(
            [VAL_W, ROW_H],
            egui::DragValue::new(val)
                .range(range)
                .fixed_decimals(decimals)
                .speed(0.005),
        );
    });
}

fn prop_slider_u32(
    ui: &mut Ui,
    label: &str,
    val: &mut u32,
    range: RangeInclusive<u32>,
    log: bool,
) {
    prop_row(ui, label, |ui| {
        let gap = 6.0;
        let track_w = (ui.available_width() - VAL_W - gap).max(40.0);
        ui.spacing_mut().item_spacing.x = gap;
        ui.spacing_mut().slider_width = track_w;
        let slider = egui::Slider::new(val, range.clone())
            .show_value(false)
            .logarithmic(log);
        ui.add_sized([track_w, ROW_H], slider);
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
        ui.add_sized(
            [VAL_W, ROW_H],
            egui::DragValue::new(val).range(range).speed(1.0),
        );
    });
}

fn prop_vec3(
    ui: &mut Ui,
    label: &str,
    v: &mut [f32; 3],
    range: RangeInclusive<f32>,
    speed: f32,
) {
    prop_row(ui, label, |ui| {
        let w = ((ui.available_width() - 8.0) / 3.0).max(32.0);
        for item in v.iter_mut() {
            ui.add_sized(
                [w, ROW_H],
                egui::DragValue::new(item)
                    .range(range.clone())
                    .speed(speed)
                    .fixed_decimals(2),
            );
        }
    });
}

fn prop_toggle(ui: &mut Ui, label: &str, val: &mut bool, text: &str) {
    prop_row(ui, label, |ui| {
        ui.checkbox(val, text);
    });
}

fn stat_row(ui: &mut Ui, label: &str, value: &str) {
    prop_row(ui, label, |ui| {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(
                RichText::new(value)
                    .monospace()
                    .size(11.5)
                    .color(Color32::from_rgb(220, 226, 236)),
            );
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
        painter.rect_filled(rect, 2.0, Color32::from_rgb(16, 18, 22));

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
            ui.label(
                RichText::new(txt)
                    .monospace()
                    .size(11.0)
                    .color(color),
            );
        });
    });
}
