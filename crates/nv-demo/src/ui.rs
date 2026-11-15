//! Docked egui UI with Phosphor icons and uniformly aligned property grids.

use std::ops::RangeInclusive;

use egui::{Align, Color32, FontDefinitions, Layout, RichText, Sense, Ui};
use egui_dock::{DockState, NodeIndex, Style, TabViewer};
use egui_phosphor::regular as ic;

use crate::camera_core::input::NavScheme;
use crate::camera_core::{CameraDriver, CameraIntent, CameraPose, NavFeel, NavMode, ViewportSize};
use crate::renderer::{DemoSettings, RENDER_H, RENDER_W};

const LABEL_W: f32 = 86.0;
const VAL_W: f32 = 64.0;
const ROW_H: f32 = 20.0;

pub struct GpuTelemetry {
    pub gpu_name: String,
    pub temp_c: u32,
    pub util_gpu: u32,
    pub vram_used_mib: u64,
    pub vram_total_mib: u64,
    pub power_w: f32,
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
    pub cam_driver: &'a mut CameraDriver,
    pub cam_home: CameraPose,
    pub cam_pose: &'a mut CameraPose,
    pub cam_mode: &'a mut NavMode,
    pub cam_fly_mode: &'a mut NavMode,
    pub cam_scheme: &'a mut NavScheme,
    pub cam_feel: &'a mut NavFeel,
    pub telemetry: &'a GpuTelemetry,
    pub accum_count: u32,
    pub total_samples: u32,
    pub rt_ms: f32,
    pub denoise_ms: f32,
    pub fps: f32,
    pub viewport_focused: &'a mut bool,
    pub reset_accum_requested: &'a mut bool,
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
        let aspect = RENDER_W as f32 / RENDER_H as f32;
        let w = avail.x.min(avail.y * aspect).max(64.0);
        let h = (w / aspect).max(64.0);
        let (rect, _) = ui.allocate_exact_size(avail, Sense::hover());
        let resp = ui.interact(rect, ui.id().with("optix-vp"), Sense::click_and_drag());
        let img_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(w, h));
        ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(12, 13, 16));
        ui.painter().image(
            self.display_tex_id,
            img_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );

        *self.viewport_focused = resp.hovered() || resp.dragged();
        let df = crate::camera_egui::drive(ui, &resp, self.cam_driver, 2.0);
        *self.cam_pose = df.pose;

        // Compact bottom-left status badge inside viewport
        let hud_pos = rect.left_bottom() + egui::vec2(10.0, -26.0);
        let status = if self.settings.auto_rotate {
            format!("{} SPIN  |  {} GUI FPS  |  {:.1} ms RT", ic::ARROWS_CLOCKWISE, self.fps.round() as u32, self.rt_ms)
        } else {
            format!(
                "{} {} / {} spp  |  {} FPS  |  {:.1} ms RT",
                ic::STACK,
                self.total_samples,
                self.settings.max_accum.saturating_mul(self.settings.spp),
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
        section_header(ui, ic::CUBE, "Formula");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut s.fractal_kind, 0, "Mandelbulb");
            ui.selectable_value(&mut s.fractal_kind, 1, "Mandelbox");
            ui.selectable_value(&mut s.fractal_kind, 2, "KIFS Octa");
        });
        ui.add_space(4.0);

        match s.fractal_kind {
            0 => {
                prop_slider_f32(ui, "Power", &mut s.bulb_power, 2.0..=16.0, 2, false);
                prop_slider_f32(ui, "Bailout", &mut s.bulb_bailout, 2.0..=32.0, 1, false);
                prop_slider_u32(ui, "Iterations", &mut s.bulb_iter, 4..=24, false);
                prop_toggle(ui, "Julia Mode", &mut s.bulb_julia, "Enable Julia C");
                if s.bulb_julia {
                    prop_vec3(ui, "Julia C", &mut s.bulb_c, -1.5..=1.5, 0.01);
                }
            }
            1 => {
                prop_slider_f32(ui, "Scale", &mut s.box_scale, -3.5..=3.5, 2, false);
                prop_slider_f32(ui, "Fold", &mut s.box_fold, 0.2..=2.2, 2, false);
                prop_slider_f32(ui, "Min R", &mut s.box_min_r, 0.05..=1.2, 2, false);
                prop_slider_f32(ui, "Fixed R", &mut s.box_fixed_r, 0.4..=2.5, 2, false);
                prop_slider_u32(ui, "Iterations", &mut s.box_iter, 4..=24, false);
                prop_slider_f32(ui, "Bailout", &mut s.box_bailout, 4.0..=64.0, 1, false);
            }
            _ => {
                prop_slider_f32(ui, "Scale", &mut s.kifs_scale, 1.3..=2.8, 2, false);
                prop_slider_f32(ui, "Rot Y", &mut s.kifs_rot1, -180.0..=180.0, 1, false);
                prop_slider_f32(ui, "Rot X", &mut s.kifs_rot2, -180.0..=180.0, 1, false);
                prop_vec3(ui, "Offset", &mut s.kifs_offset, 0.1..=2.0, 0.01);
                prop_slider_u32(ui, "Iterations", &mut s.kifs_iter, 4..=24, false);
                prop_slider_f32(ui, "Bailout", &mut s.kifs_bailout, 4.0..=32.0, 1, false);
            }
        }

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
        prop_slider_f32(ui, "Sky Power", &mut s.sky_intensity, 0.1..=2.0, 2, false);
        prop_slider_f32(ui, "Haze", &mut s.horizon_haze, 0.4..=2.5, 2, false);
        prop_slider_f32(ui, "Exposure", &mut s.exposure, 0.25..=2.0, 2, false);

        ui.add_space(6.0);
        section_header(ui, ic::SQUARES_FOUR, "Scratched Metal Floor");
        prop_slider_f32(ui, "Roughness", &mut s.floor_roughness, 0.03..=0.80, 2, false);
        prop_slider_f32(ui, "Scratches", &mut s.floor_scratch, 0.0..=1.5, 2, false);
        prop_slider_f32(ui, "Tile Scale", &mut s.floor_scale, 0.3..=5.0, 2, false);
        prop_slider_f32(ui, "Metallic", &mut s.floor_metallic, 0.0..=1.0, 2, false);

        ui.add_space(6.0);
        section_header(ui, ic::SPARKLE, "Path Tracer");
        prop_slider_u32(ui, "SPP / Pass", &mut s.spp, 1..=64, true);
        prop_slider_u32(ui, "Max Accum", &mut s.max_accum, 1..=1024, true);
        prop_slider_f32(ui, "Spin Speed", &mut s.spin_speed, 0.0..=0.5, 3, false);

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.checkbox(&mut s.shadows_on, "Shadows");
            ui.checkbox(&mut s.denoiser_on, "AI Denoise");
            ui.checkbox(&mut s.auto_rotate, "Spin");
        });
        ui.add_space(4.0);
        if ui
            .button(format!("{}  Reset Accumulation", ic::ARROWS_CLOCKWISE))
            .clicked()
        {
            *self.reset_accum_requested = true;
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
        ui.horizontal(|ui| {
            if ui.button(format!("{}  Home", ic::HOUSE)).clicked() {
                self.cam_driver.controller_mut().set_pose(self.cam_home);
                *self.cam_pose = self.cam_home;
            }
            if ui.button(format!("{}  Frame", ic::CROSSHAIR)).clicked() {
                self.cam_driver.controller_mut().apply_intent(
                    CameraIntent::FrameBounds {
                        min: glam::Vec3::splat(-1.25),
                        max: glam::Vec3::splat(1.25),
                        margin: 1.2,
                    },
                    1.0 / 60.0,
                    ViewportSize::new(RENDER_W, RENDER_H),
                );
                *self.cam_pose = self.cam_driver.controller().pose();
            }
        });

        ui.add_space(6.0);
        section_header(ui, ic::GAUGE, "Telemetry");
        stat_row(ui, "GPU", &self.telemetry.gpu_name);
        stat_row(
            ui,
            "GUI / RT",
            &format!("{:.0} FPS / {:.1} ms", self.fps, self.rt_ms),
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
            "Temp / Load",
            &format!("{} C / {} %", self.telemetry.temp_c, self.telemetry.util_gpu),
        );
        stat_row(ui, "Power", &format!("{:.0} W", self.telemetry.power_w));
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

fn prop_row(ui: &mut Ui, label: &str, add_contents: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(LABEL_W, ROW_H),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.label(
                    RichText::new(label)
                        .size(12.0)
                        .color(Color32::from_rgb(188, 194, 206)),
                );
            },
        );
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width().max(60.0), ROW_H),
            Layout::left_to_right(Align::Center),
            add_contents,
        );
    });
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
        let track_w = (ui.available_width() - VAL_W - 6.0).max(40.0);
        ui.spacing_mut().slider_width = track_w;
        let slider = egui::Slider::new(val, range.clone())
            .show_value(false)
            .logarithmic(log);
        ui.add_sized([track_w, ROW_H], slider);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add_sized(
                [VAL_W, ROW_H],
                egui::DragValue::new(val)
                    .range(range)
                    .fixed_decimals(decimals)
                    .speed(0.005),
            );
        });
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
        let track_w = (ui.available_width() - VAL_W - 6.0).max(40.0);
        ui.spacing_mut().slider_width = track_w;
        let slider = egui::Slider::new(val, range.clone())
            .show_value(false)
            .logarithmic(log);
        ui.add_sized([track_w, ROW_H], slider);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add_sized(
                [VAL_W, ROW_H],
                egui::DragValue::new(val).range(range).speed(1.0),
            );
        });
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
