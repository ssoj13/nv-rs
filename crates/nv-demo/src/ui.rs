//! Docked egui UI with Phosphor icons and uniformly aligned property grids.

use std::ops::RangeInclusive;

use egui::{Align, Color32, FontDefinitions, Layout, RichText, Sense, Ui};
use egui_dock::{DockState, NodeIndex, Style, TabViewer};
use egui_phosphor::regular as ic;

use crate::camera_core::input::NavScheme;
use crate::camera_core::{CameraDriver, CameraIntent, CameraPose, NavFeel, NavMode, ViewportSize};
use crate::demo_anim::DemoAnimator;
use crate::profiler::{FrameTimings, GpuTelemetry, HISTORY_LEN, RingBuffer};
use crate::renderer::DemoSettings;

const LABEL_W: f32 = 84.0;
const VAL_W: f32 = 56.0;
const ANIM_BTN_W: f32 = 16.0;
const ROW_H: f32 = 14.0;

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
    pub settings: &'a mut DemoSettings,
    pub demo_anim: &'a mut DemoAnimator,
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
    pub theme: &'a mut UiTheme,
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
    pub ser_supported: bool,
    pub dlss_rr_available: bool,
    pub hdr_surface_supported: bool,
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

            if ui.button(format!("{} Home", ic::HOUSE)).clicked() {
                self.cam_driver.controller_mut().set_pose(self.cam_home);
                *self.cam_pose = self.cam_home;
            }
            if ui.button(format!("{} Frame", ic::CROSSHAIR)).clicked() {
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
            }

            if ui
                .button(format!("{} Reset", ic::ARROWS_CLOCKWISE))
                .on_hover_text("Reset path-tracer accumulation")
                .clicked()
            {
                *self.reset_accum_requested = true;
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
            let play_btn = egui::Button::new(play_label).selected(playing);
            if ui
                .add_sized([68.0, 22.0], play_btn)
                .on_hover_text("Play / Pause selected animations (Space)")
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
        let sat_orbiting =
            self.settings.sat_on && self.settings.sat_orbit && self.settings.sat_count > 0;
        let demo_running = self.demo_anim.is_running();
        let status = if self.settings.auto_rotate || sat_orbiting || demo_running {
            let mut tags = Vec::with_capacity(3);
            if demo_running {
                tags.push("DEMO");
            }
            if self.settings.auto_rotate {
                tags.push("SPIN");
            }
            if sat_orbiting {
                tags.push("ORBIT");
            }
            let anim_tag = tags.join("+");
            format!(
                "{} {}  |  {}x{}  |  {}  |  {} FPS  |  {:.1} ms RT",
                ic::ARROWS_CLOCKWISE,
                anim_tag,
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
        let anim = &mut *self.demo_anim;
        collapsing_section(ui, "sec_presets", ic::SPARKLE, "Presets", true, |ui| {
            equal_grid(ui, 2, 4, |ui, idx, w| match idx {
                0 => {
                    if ui
                        .add_sized([w, ROW_H + 2.0], egui::Button::new("Gold Bulb"))
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
                anim_slider_f32(
                    ui,
                    anim,
                    "obj_scale",
                    "Obj Scale",
                    &mut s.obj_scale,
                    0.25..=2.5,
                    2,
                    false,
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

    fn ui_render_floor(&mut self, ui: &mut Ui) {
        let s = &mut *self.settings;
        let anim = &mut *self.demo_anim;

        collapsing_section(ui, "sec_sky", ic::SUN, "Sky & Lighting", true, |ui| {
            prop_row(ui, "Background", |ui| {
                let gap = 4.0;
                let combo_w = (ui.available_width() - ANIM_BTN_W - gap).max(60.0);
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
                ];
                let idx = (s.sky_mode as usize).min(modes.len() - 1);
                egui::ComboBox::from_id_salt("sky_mode_combo")
                    .width(combo_w)
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
                let w = ((inner_w - gap * 2.0) / 3.0).max(28.0);
                ui.spacing_mut().item_spacing.x = gap;
                for (m, lbl) in [(0u32, "Off"), (1, "OMM"), (2, "Any-Hit")] {
                    if ui
                        .add_sized(
                            [w, ROW_H],
                            egui::Button::selectable(s.omm_mode == m, lbl),
                        )
                        .clicked()
                    {
                        s.omm_mode = m;
                    }
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
        });

        collapsing_section(ui, "sec_pt", ic::LIGHTNING, "Path Tracer", true, |ui| {
            prop_row(ui, "Scale", |ui| {
                let gap = 4.0;
                let inner_w = (ui.available_width() - ANIM_BTN_W - gap).max(60.0);
                ui.spacing_mut().item_spacing.x = gap;
                ui.add_enabled_ui(!s.dlss_on && !s.dlss_rr_on, |ui| {
                    let w = ((inner_w - gap * 3.0) / 4.0).max(24.0);
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
                anim_toggle_btn_disabled(ui);
            });
            prop_row(ui, "AOV", |ui| {
                let gap = 4.0;
                let inner_w = (ui.available_width() - ANIM_BTN_W - gap).max(60.0);
                let w = ((inner_w - gap * 4.0) / 5.0).max(24.0);
                ui.spacing_mut().item_spacing.x = gap;
                for (mode, lbl) in [
                    (0u32, "Beauty"),
                    (1u32, "Albedo"),
                    (2u32, "Normal"),
                    (3u32, "Depth"),
                    (4u32, "Motion"),
                ] {
                    if ui
                        .add_sized(
                            [w, ROW_H],
                            egui::Button::selectable(s.aov_mode == mode, lbl),
                        )
                        .clicked()
                    {
                        s.aov_mode = mode;
                    }
                }
                anim_toggle_btn_disabled(ui);
            });
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

            // Unified 4-way Upscale / AI Denoise selector (mutually exclusive SSOT)
            prop_row(ui, "Upscale / AI", |ui| {
                let gap = 4.0;
                let inner_w = (ui.available_width() - ANIM_BTN_W - gap).max(60.0);
                let w = ((inner_w - gap * 3.0) / 4.0).max(24.0);
                ui.spacing_mut().item_spacing.x = gap;
                let mode = if s.dlss_rr_on {
                    3u32
                } else if s.dlss_on {
                    2
                } else if s.denoiser_on {
                    1
                } else {
                    0
                };
                for (m, lbl) in [
                    (0u32, "Off"),
                    (1, "OptiX AI"),
                    (2, "DLSS SR"),
                    (3, "DLSS RR"),
                ] {
                    let resp = ui.add_sized([w, ROW_H], egui::Button::selectable(mode == m, lbl));
                    let resp = if m == 3 {
                        resp.on_hover_text(if self.dlss_rr_available {
                            "DLSS Ray Reconstruction (available)"
                        } else {
                            "DLSS Ray Reconstruction (unavailable on this host/runtime)"
                        })
                    } else {
                        resp
                    };
                    if resp.clicked() {
                        s.denoiser_on = m == 1;
                        s.dlss_on = m == 2;
                        s.dlss_rr_on = m == 3;
                    }
                }
                anim_toggle_btn_disabled(ui);
            });

            if s.denoiser_on {
                prop_toggle(ui, "Temporal AI", &mut s.temporal_denoise, "Temporal Accumulation");
            }

            if s.dlss_on || s.dlss_rr_on {
                prop_row(ui, "DLSS Qual", |ui| {
                    let gap = 4.0;
                    let inner_w = (ui.available_width() - ANIM_BTN_W - gap).max(60.0);
                    let w = ((inner_w - gap * 4.0) / 5.0).max(24.0);
                    ui.spacing_mut().item_spacing.x = gap;
                    for (q, lbl) in [
                        (0u32, "UltraP"),
                        (1u32, "Perf"),
                        (2u32, "Bal"),
                        (3u32, "Qual"),
                        (5u32, "DLAA"),
                    ] {
                        if ui
                            .add_sized(
                                [w, ROW_H],
                                egui::Button::selectable(s.dlss_quality == q, lbl),
                            )
                            .clicked()
                        {
                            s.dlss_quality = q;
                        }
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
                    if ui
                        .add_sized([w, ROW_H + 2.0], egui::Button::new(rec_lbl))
                        .clicked()
                    {
                        *self.toggle_record_requested = true;
                    }
                }
            });
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("View")
                        .size(11.0)
                        .color(Color32::from_rgb(165, 173, 203)),
                );
                if ui
                    .selectable_label(!s.hdr_display, "SDR")
                    .on_hover_text("8-bit sRGB/BT.709 ACES viewport")
                    .clicked()
                {
                    s.hdr_display = false;
                }
                ui.add_enabled_ui(self.hdr_surface_supported, |ui| {
                    let hdr_tip = if self.hdr_surface_supported {
                        "True HDR (Rgba16Float scRGB, 203-nit white / 1000-nit peak, matches PQ10 video)"
                    } else {
                        "HDR unavailable — enable HDR in Windows Display Settings"
                    };
                    if ui
                        .selectable_label(s.hdr_display && self.hdr_surface_supported, "HDR")
                        .on_hover_text(hdr_tip)
                        .clicked()
                    {
                        s.hdr_display = true;
                    }
                });
                ui.separator();
                ui.add_enabled_ui(!self.is_recording, |ui| {
                    ui.label(
                        RichText::new("Timing")
                            .size(11.0)
                            .color(Color32::from_rgb(165, 173, 203)),
                    );
                    if ui
                        .selectable_label(!s.record_fixed_fps, "Live RT")
                        .on_hover_text("Real-time MP4 timestamps (no speedup when FPS dips)")
                        .clicked()
                    {
                        s.record_fixed_fps = false;
                    }
                    if ui
                        .selectable_label(s.record_fixed_fps, "Smooth 60")
                        .on_hover_text("Fixed 1/60s per rendered frame for smooth offline animations")
                        .clicked()
                    {
                        s.record_fixed_fps = true;
                    }
                });
            });
            ui.add_enabled_ui(!self.is_recording, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Video")
                            .size(11.0)
                            .color(Color32::from_rgb(165, 173, 203)),
                    );
                    if ui
                        .selectable_label(s.record_video_mode == 0, "Match")
                        .on_hover_text("Match View: SDR view -> H.264 MP4, HDR view -> PQ10 HEVC MP4")
                        .clicked()
                    {
                        s.record_video_mode = 0;
                    }
                    if ui
                        .selectable_label(s.record_video_mode == 1, "SDR")
                        .on_hover_text("Force H.264 8-bit BT.709 MP4 (avc1)")
                        .clicked()
                    {
                        s.record_video_mode = 1;
                    }
                    if ui
                        .selectable_label(s.record_video_mode == 2, "PQ10")
                        .on_hover_text("Force iPhone Pro Max HDR10 HEVC Main10 BT.2020 PQ MP4 (hvc1)")
                        .clicked()
                    {
                        s.record_video_mode = 2;
                    }
                });
            });
            ui.add_space(2.0);
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
        });
    }

    fn ui_camera_stats(&mut self, ui: &mut Ui) {
        collapsing_section(ui, "sec_telemetry", ic::GAUGE, "Telemetry", true, |ui| {
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
        });

        collapsing_section(ui, "sec_perf", ic::CHART_LINE, "Performance", true, |ui| {
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
        });
    }
}

fn section_accent(id_salt: &str) -> Color32 {
    match id_salt {
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
        "sec_sky",
        "sec_dof",
        "sec_floor",
        "sec_scene",
        "sec_bloom",
        "sec_split",
        "sec_pt",
    ],
    // Camera & Stats panel
    &["sec_telemetry", "sec_perf"],
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
        "sec_telemetry"
            | "sec_perf"
            | "sec_presets"
            | "sec_formula"
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

fn equal_grid(
    ui: &mut Ui,
    cols: usize,
    count: usize,
    mut item_ui: impl FnMut(&mut Ui, usize, f32),
) {
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
                egui::FontId::proportional(11.5),
                ui.visuals().weak_text_color(),
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
        let track_w = (ui.available_width() - VAL_W - ANIM_BTN_W - gap * 2.0).max(32.0);
        ui.spacing_mut().item_spacing.x = gap;
        ui.spacing_mut().slider_width = track_w;
        let slider = egui::Slider::new(val, range.clone())
            .show_value(false)
            .logarithmic(log);
        ui.add_sized([track_w, ROW_H], slider);
        let prev_style = ui.style().override_text_style.clone();
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
        ui.add_sized(
            [VAL_W, ROW_H],
            egui::DragValue::new(val)
                .range(range)
                .fixed_decimals(decimals)
                .speed(0.005),
        );
        ui.style_mut().override_text_style = prev_style;
        anim_toggle_btn(ui, anim, key);
    });
}

fn prop_slider_u32(ui: &mut Ui, label: &str, val: &mut u32, range: RangeInclusive<u32>, log: bool) {
    prop_row(ui, label, |ui| {
        let gap = 4.0;
        let track_w = (ui.available_width() - VAL_W - ANIM_BTN_W - gap * 2.0).max(32.0);
        ui.spacing_mut().item_spacing.x = gap;
        ui.spacing_mut().slider_width = track_w;
        let slider = egui::Slider::new(val, range.clone())
            .show_value(false)
            .logarithmic(log);
        ui.add_sized([track_w, ROW_H], slider);
        let prev_style = ui.style().override_text_style.clone();
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
        ui.add_sized(
            [VAL_W, ROW_H],
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
        let w = ((ui.available_width() - ANIM_BTN_W - gap * 3.0) / 3.0).max(28.0);
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
        let check_w = (ui.available_width() - ANIM_BTN_W - gap).max(40.0);
        ui.spacing_mut().item_spacing.x = gap;
        ui.allocate_ui_with_layout(
            egui::vec2(check_w, ROW_H),
            Layout::left_to_right(Align::Center),
            |ui| {
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
