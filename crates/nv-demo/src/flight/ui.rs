//! Native `egui` dock UI panels for **Flight Lab** and **Flight Player** (`nv-demo`).
//!
//! **What.** Renders [`ui_flight_lab`] (Auto Plan gated banner + Manual Keyframe authoring + live
//! `camera-guide-rs` dynamics sliders & curve painter + Contact Sheet + Flight Stats) and
//! [`ui_flight_player`] (Winamp-style Now Playing card + phosphor transport controls + interactive
//! progress bar with shot/cut/chase overlays + Playlist Queue + Repo/User/Curated Library Browser).
//! **Why.** Gives operators clean, DCC-grade, columnar controls sharing one [`FlightWorkspace`]
//! while matching `nv-demo`'s existing visual tokens (`LABEL_W`, `ROW_H`, `VAL_W`, theme-aware
//! tints, right-aligned tabular numbers, and 100% `egui-phosphor` icons with zero Unicode glyphs).
//! **How.** Both entry points take [`FlightUiArgs`] and emit typed [`FlightCommand`] actions via
//! `args.on_command`, keeping UI rendering decoupled from the OptiX render loop and `main.rs`.
//! **Uses.** `camera`, `egui`, `egui_phosphor`, `flight_navigator`, `crate::flight`,
//! `crate::flight_stats`, `crate::csg_scene_host`, `crate::is_counters`, `crate::renderer`.
//! **Used by.** `crate::ui` (`DockTab::FlightLab` and `DockTab::FlightPlayer`).

use camera::CameraPose;
use egui::{Align, Color32, FontId, Layout, Pos2, Rect, RichText, Sense, Stroke, Ui};
use egui_phosphor::regular as ic;
use flight_navigator::camera_guide::RATIO_TOL;
use flight_navigator::fnav_core::{
    LibrarySource, Move, PlaylistTransition, RepeatMode, Segment, Track,
};
use flight_navigator::fnav_plan::Mode;

use crate::csg_scene_host::CsgStats;
use crate::flight::convert::limit_term_label;
use crate::flight::lab::{
    CSG_AUTO_PLAN_BLOCKED_REASON, DynamicsCurvePlot, LAB_DURATION_MAX_S, LAB_DURATION_MIN_S,
    LabMode,
};
use crate::flight::player::{PlayerState, TrackOrigin};
use crate::flight::{FlightCommand, FlightWorkspace};
use crate::flight_stats::{FlightStats, HIST_BINS, LoopPoint};
use crate::is_counters::{IsCounts, IsGroup};
use crate::renderer::{TIME_SCALE_MAX, TIME_SCALE_MIN};

/// Canonical property-row label column width matching `crate::ui`.
const LABEL_W: f32 = 84.0;
/// Canonical property-row numeric value width matching `crate::ui`.
const VAL_W: f32 = 56.0;
/// Canonical compact row height matching `crate::ui`.
const ROW_H: f32 = 16.0;
/// Height of the `camera-guide-rs` speed/cap/bank dynamics graph.
const DYNAMICS_GRAPH_H: f32 = 76.0;
/// Height of the Flight stats RT curve graph.
const STATS_GRAPH_H: f32 = 58.0;
/// Height of the interactive Winamp-style timeline progress bar.
const PROGRESS_BAR_H: f32 = 22.0;

/// Shared arguments passed by `crate::ui` into [`ui_flight_lab`] and [`ui_flight_player`].
pub struct FlightUiArgs<'a> {
    /// Single source of truth holding both [`crate::flight::player::FlightPlayer`] and
    /// [`crate::flight::lab::FlightLabState`].
    pub workspace: &'a mut FlightWorkspace,
    /// Accumulated per-frame flight performance statistics.
    pub flight_stats: &'a mut FlightStats,
    /// Latest CSG scene instance/LOD/path telemetry.
    pub csg_stats: &'a CsgStats,
    /// Active CSG LOD distance bias.
    pub csg_lod_bias: f32,
    /// Active crowd actor count in the scene.
    pub crowd_actor_count: u32,
    /// Latest OptiX ray-tracing frame time (ms).
    pub rt_ms: f32,
    /// Debug intersection-counter toggle.
    pub is_counters: &'a mut bool,
    /// Latest counted intersection totals (when `is_counters` is enabled).
    pub last_is_counts: Option<&'a IsCounts>,
    /// Human-readable label of the currently loaded scene.
    pub active_scene_label: &'a str,
    /// Current viewport camera pose (used for manual keyframe capture).
    pub cur_pose: CameraPose,
    /// Whether MP4 video recording is currently active.
    pub is_recording: bool,
    /// Global simulation time scale slider value.
    pub time_scale: &'a mut f32,
    /// Camera damper smoothing time constant (seconds).
    pub cam_smooth_tau: &'a mut f32,
    /// Reusable text formatting scratch buffer.
    pub label_buf: &'a mut String,
    /// Callback emitting [`FlightCommand`] actions to `AppBus`.
    pub on_command: &'a mut dyn FnMut(FlightCommand),
    /// Callback requesting Flight stats CSV export.
    pub on_export_flight_csv: &'a mut dyn FnMut(),
    /// Callback jumping the viewport camera directly to a pose.
    pub on_seek_pose: &'a mut dyn FnMut(CameraPose),
}

/// Renders the **Flight Lab** dock panel (Auto Plan gate, Manual Keyframes, live `camera-guide-rs`
/// dynamics tuning & curve plot, Contact Sheet, and Flight Stats).
pub fn ui_flight_lab(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    ui.spacing_mut().item_spacing.y = 4.0;

    let mode = args.workspace.lab.mode;
    equal_grid(ui, 2, 2, |ui, idx, w| match idx {
        0 => {
            let active = mode == LabMode::AutoPlan;
            let btn = mode_btn(ui, format!("{}  Auto Plan", ic::SPARKLE), active, w);
            if ui
                .add(btn)
                .on_hover_text("Automatic cinematic planner (requires csg-rs R0 + R1).")
                .clicked()
            {
                args.workspace.lab.mode = LabMode::AutoPlan;
            }
        }
        _ => {
            let active = mode == LabMode::ManualKeyframes;
            let btn = mode_btn(ui, format!("{}  Manual Keyframes", ic::PATH), active, w);
            if ui
                .add(btn)
                .on_hover_text("Author camera keyframes, tune dynamics, preview, and save tracks.")
                .clicked()
            {
                args.workspace.lab.mode = LabMode::ManualKeyframes;
            }
        }
    });

    ui.add_space(2.0);
    match args.workspace.lab.mode {
        LabMode::AutoPlan => ui_lab_auto_plan(ui, args),
        LabMode::ManualKeyframes => ui_lab_manual_keyframes(ui, args),
    }

    ui.add_space(2.0);
    ui_lab_dynamics_section(ui, args);
    ui.add_space(2.0);
    ui_lab_contact_sheet(ui, args);
    ui.add_space(2.0);
    ui_lab_flight_stats(ui, args);

    if !args.workspace.lab.status_msg.is_empty() {
        ui.add_space(2.0);
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(RichText::new(ic::INFO).size(11.0).weak());
            ui.add(
                egui::Label::new(
                    RichText::new(&args.workspace.lab.status_msg)
                        .size(10.5)
                        .weak(),
                )
                .truncate(),
            );
        });
    }
}

/// Renders the **Auto Plan** subsection with honest R0+R1 gating and disabled Generate button.
fn ui_lab_auto_plan(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let dark = ui.visuals().dark_mode;
    let warn_bg = if dark {
        Color32::from_rgb(72, 54, 22)
    } else {
        Color32::from_rgb(252, 236, 198)
    };
    let warn_stroke = if dark {
        Color32::from_rgb(186, 138, 48)
    } else {
        Color32::from_rgb(192, 136, 28)
    };
    let warn_fg = if dark {
        Color32::from_rgb(246, 212, 128)
    } else {
        Color32::from_rgb(112, 74, 12)
    };

    egui::Frame::NONE
        .fill(warn_bg)
        .stroke(Stroke::new(1.0, warn_stroke))
        .corner_radius(3.0)
        .inner_margin(egui::Margin::symmetric(6, 5))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(ic::WARNING).color(warn_fg).strong());
                ui.label(
                    RichText::new(CSG_AUTO_PLAN_BLOCKED_REASON)
                        .size(10.5)
                        .color(warn_fg),
                );
            });
        });

    let lab = &mut args.workspace.lab;
    slider_f32_row(
        ui,
        "Duration",
        &mut lab.duration_s,
        LAB_DURATION_MIN_S..=LAB_DURATION_MAX_S,
        1,
        " s",
    );
    prop_row(ui, "Seed", |ui| {
        ui.add_sized(
            [ui.available_width().max(48.0), ROW_H],
            egui::DragValue::new(&mut lab.seed).speed(1.0),
        );
    });
    prop_row(ui, "Plan Mode", |ui| {
        let cur_lbl = match lab.plan_mode {
            Mode::Deterministic => "Deterministic",
            Mode::Creative => "Creative",
        };
        egui::ComboBox::from_id_salt("flight_lab_plan_mode")
            .width(ui.available_width().max(48.0))
            .selected_text(cur_lbl)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut lab.plan_mode, Mode::Deterministic, "Deterministic");
                ui.selectable_value(&mut lab.plan_mode, Mode::Creative, "Creative");
            });
    });
    slider_f32_row(ui, "Cut Density", &mut lab.cut_density, 0.0..=2.0, 2, "x");

    ui.add_space(2.0);
    let gen_btn = egui::Button::new(format!("{}  Generate Flight", ic::LOCK))
        .min_size(egui::vec2(ui.available_width(), ROW_H + 4.0));
    ui.add_enabled(false, gen_btn)
        .on_disabled_hover_text(CSG_AUTO_PLAN_BLOCKED_REASON);
}

/// Renders the **Manual Keyframes** authoring subsection.
fn ui_lab_manual_keyframes(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let kf_count = args.workspace.lab.manual_keyframes.len();
    let has_preview = args.workspace.lab.preview_track.is_some();

    stat_row(
        ui,
        "Keyframes",
        fmt_into(
            args.label_buf,
            format_args!(
                "{kf_count} KF  ({:.1}, {:.1}, {:.1})",
                args.cur_pose.eye.x, args.cur_pose.eye.y, args.cur_pose.eye.z
            ),
        ),
    );

    equal_grid(ui, 3, 3, |ui, idx, w| match idx {
        0 => {
            if ui
                .add_sized(
                    [w, ROW_H + 2.0],
                    egui::Button::new(format!("{}  + KF", ic::PLUS)),
                )
                .on_hover_text("Capture the current viewport camera pose as a keyframe.")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabAddKeyframe);
            }
        }
        1 => {
            if ui
                .add_enabled(
                    kf_count > 0,
                    egui::Button::new(format!("{}  Pop KF", ic::MINUS))
                        .min_size(egui::vec2(w, ROW_H + 2.0)),
                )
                .on_hover_text("Remove the last captured keyframe.")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabPopKeyframe);
            }
        }
        _ => {
            if ui
                .add_enabled(
                    kf_count > 0,
                    egui::Button::new(format!("{}  Clear", ic::TRASH))
                        .min_size(egui::vec2(w, ROW_H + 2.0)),
                )
                .on_hover_text("Clear all manual keyframes.")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabClearKeyframes);
            }
        }
    });

    slider_f32_row(
        ui,
        "Duration",
        &mut args.workspace.lab.duration_s,
        2.0..=LAB_DURATION_MAX_S,
        1,
        " s",
    );
    prop_row(ui, "Loop Track", |ui| {
        ui.checkbox(&mut args.workspace.lab.manual_looping, "Closed C2 Loop");
    });
    prop_row(ui, "Track Name", |ui| {
        ui.add_sized(
            [ui.available_width().max(48.0), ROW_H],
            egui::TextEdit::singleline(&mut args.workspace.lab.track_name),
        );
    });

    equal_grid(ui, 2, 2, |ui, idx, w| match idx {
        0 => {
            if ui
                .add_enabled(
                    kf_count >= 2,
                    egui::Button::new(format!("{}  Build Track", ic::PATH))
                        .min_size(egui::vec2(w, ROW_H + 2.0)),
                )
                .on_hover_text("Compile manual keyframes through camera-guide-rs into a Track.")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabBuildManualTrack);
            }
        }
        _ => {
            if ui
                .add_enabled(
                    has_preview,
                    egui::Button::new(format!("{}  Preview", ic::PLAY))
                        .min_size(egui::vec2(w, ROW_H + 2.0)),
                )
                .on_hover_text("Load the built preview track into Flight Player and play it.")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabPreviewTrack);
            }
        }
    });

    let save_name = args.workspace.lab.track_name.clone();
    equal_grid(ui, 2, 2, |ui, idx, w| match idx {
        0 => {
            if ui
                .add_enabled(
                    has_preview,
                    egui::Button::new(format!("{}  Save User", ic::FLOPPY_DISK))
                        .min_size(egui::vec2(w, ROW_H + 2.0)),
                )
                .on_hover_text("Atomically save preview track to ~/.nv-rs/flights/")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabSaveTrack {
                    name: save_name.clone(),
                    to_repo: false,
                });
            }
        }
        _ => {
            if ui
                .add_enabled(
                    has_preview,
                    egui::Button::new(format!("{}  Save Repo", ic::FOLDER_OPEN))
                        .min_size(egui::vec2(w, ROW_H + 2.0)),
                )
                .on_hover_text("Atomically save preview track to assets/flights/")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabSaveTrack {
                    name: save_name.clone(),
                    to_repo: true,
                });
            }
        }
    });

    if kf_count > 0 {
        egui::ScrollArea::vertical()
            .id_salt("flight_lab_kf_list")
            .max_height(88.0)
            .show(ui, |ui| {
                for (i, kf) in args.workspace.lab.manual_keyframes.iter().enumerate() {
                    ui.horizontal(|ui| {
                        let txt = fmt_into(
                            args.label_buf,
                            format_args!(
                                "#{:02}  ({:.1}, {:.1}, {:.1})  {:.0} mm",
                                i + 1,
                                kf.pos[0],
                                kf.pos[1],
                                kf.pos[2],
                                kf.focal_mm
                            ),
                        );
                        ui.add(
                            egui::Label::new(RichText::new(txt).monospace().size(10.5)).truncate(),
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui
                                .add_sized([22.0, ROW_H], egui::Button::new(ic::EYE))
                                .on_hover_text("Jump viewport camera to this keyframe.")
                                .clicked()
                            {
                                (args.on_seek_pose)(kf.to_pose(0.05, 5000.0));
                            }
                        });
                    });
                }
            });
    }
}

/// Renders the **Camera Dynamics (`camera-guide-rs`)** section with live limit controls & curve plot.
fn ui_lab_dynamics_section(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let has_active = args.workspace.player.active_track().is_some()
        || args.workspace.lab.preview_track.is_some();

    section_box(
        ui,
        "flight_lab_dynamics",
        ic::SLIDERS_HORIZONTAL,
        "Camera Dynamics (camera-guide-rs)",
        true,
        |ui| {
            let lim = &mut args.workspace.lab.limits;
            slider_f32_row(ui, "Max Bank", &mut lim.max_bank_deg, 0.0..=60.0, 1, " deg");
            slider_f32_row(ui, "Bank Gain k", &mut lim.bank_k, 0.0..=1.0, 2, "");
            slider_f32_row(
                ui,
                "Bank Rate",
                &mut lim.bank_rate_deg_s,
                10.0..=180.0,
                0,
                " d/s",
            );
            slider_f32_row(
                ui,
                "Long Accel",
                &mut lim.long_accel,
                1.0..=12.0,
                1,
                " m/s2",
            );
            slider_f32_row(ui, "Jerk", &mut lim.jerk, 1.0..=20.0, 1, " m/s3");

            ui.add_space(2.0);
            if ui
                .add_enabled(
                    has_active,
                    egui::Button::new(format!("{}  Apply to Active Track", ic::LIGHTNING))
                        .min_size(egui::vec2(ui.available_width(), ROW_H + 3.0)),
                )
                .on_hover_text(
                    "Re-solve the active track through camera-guide-rs with these limits and plot cap/v/bank.",
                )
                .clicked()
            {
                (args.on_command)(FlightCommand::LabApplyLimitsToActive);
            }

            if let Some(plot) = args.workspace.lab.dynamics_plot.as_ref() {
                ui.add_space(3.0);
                paint_dynamics_plot(
                    ui,
                    args.label_buf,
                    plot,
                    args.workspace.lab.limits.max_bank_deg,
                );
            }
        },
    );
}

/// Paints `cap(s)` (dashed/muted), `v(s)` (accent green), and `bank(s)` (amber) plus `VerifyReport` badge.
fn paint_dynamics_plot(
    ui: &mut Ui,
    buf: &mut String,
    plot: &DynamicsCurvePlot,
    max_bank_deg: f32,
) {
    let worst = plot.report.worst();
    let ok = worst <= 1.0 + RATIO_TOL;
    let peak_v = plot.v.iter().copied().fold(0.0_f32, f32::max);
    let peak_bank = plot
        .bank_deg
        .iter()
        .copied()
        .map(f32::abs)
        .fold(0.0_f32, f32::max);
    let dominant_term = plot
        .binding
        .iter()
        .copied()
        .find(|t| !matches!(t, flight_navigator::camera_guide::LimitTerm::Target))
        .unwrap_or(flight_navigator::camera_guide::LimitTerm::Target);

    ui.horizontal(|ui| {
        let badge_col = if ok {
            Color32::from_rgb(76, 178, 92)
        } else {
            Color32::from_rgb(220, 148, 42)
        };
        let badge_txt = fmt_into(
            buf,
            format_args!(
                "{} worst {:.2}  |  v {:.1} m/s  |  bank {:.1} deg",
                if ok { ic::CHECK_CIRCLE } else { ic::WARNING },
                worst,
                peak_v,
                peak_bank
            ),
        );
        ui.label(
            RichText::new(badge_txt)
                .size(10.5)
                .color(badge_col)
                .monospace(),
        );
    });

    let w = ui.available_width().max(80.0);
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(w, DYNAMICS_GRAPH_H), Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 2.0, ui.visuals().extreme_bg_color);
    painter.rect_stroke(
        rect,
        2.0,
        Stroke::new(0.75, ui.visuals().widgets.noninteractive.bg_stroke.color),
        egui::StrokeKind::Inside,
    );

    let s_max = plot.s.last().copied().unwrap_or(1.0).max(1.0e-3);
    let v_max = plot
        .cap
        .iter()
        .chain(plot.v.iter())
        .copied()
        .filter(|v| v.is_finite())
        .fold(1.0_f32, f32::max)
        * 1.08;
    let b_max = max_bank_deg.max(peak_bank).max(5.0);

    let dark = ui.visuals().dark_mode;
    let cap_col = if dark {
        Color32::from_gray(128)
    } else {
        Color32::from_gray(142)
    };
    let v_col = Color32::from_rgb(108, 198, 64);
    let bank_col = Color32::from_rgb(232, 166, 54);

    paint_series(
        &painter,
        rect,
        &plot.s,
        &plot.cap,
        (s_max, v_max),
        SeriesStyle {
            color: cap_col,
            dashed: true,
            abs_y: false,
        },
    );
    paint_series(
        &painter,
        rect,
        &plot.s,
        &plot.v,
        (s_max, v_max),
        SeriesStyle {
            color: v_col,
            dashed: false,
            abs_y: false,
        },
    );
    paint_series(
        &painter,
        rect,
        &plot.s,
        &plot.bank_deg,
        (s_max, b_max),
        SeriesStyle {
            color: bank_col,
            dashed: false,
            abs_y: true,
        },
    );

    painter.text(
        rect.left_top() + egui::vec2(4.0, 2.0),
        egui::Align2::LEFT_TOP,
        fmt_into(buf, format_args!("{v_max:.1} m/s")),
        FontId::monospace(9.5),
        ui.visuals().weak_text_color(),
    );
    painter.text(
        rect.right_top() + egui::vec2(-4.0, 2.0),
        egui::Align2::RIGHT_TOP,
        fmt_into(buf, format_args!("{b_max:.0} deg")),
        FontId::monospace(9.5),
        bank_col,
    );
    painter.text(
        rect.left_bottom() + egui::vec2(4.0, -2.0),
        egui::Align2::LEFT_BOTTOM,
        fmt_into(
            buf,
            format_args!(
                "cap (dash) | v (green) | bank (amber) | limit: {}",
                limit_term_label(dominant_term)
            ),
        ),
        FontId::proportional(9.5),
        ui.visuals().weak_text_color(),
    );

    resp.on_hover_text(format!(
        "VerifyReport: lat={:.2}, yaw={:.2}, bank={:.2}, bank_rate={:.2}, accel={:.2}, decel={:.2}, jerk={:.2}",
        plot.report.lateral_accel,
        plot.report.yaw_rate,
        plot.report.bank,
        plot.report.bank_rate,
        plot.report.long_accel,
        plot.report.long_decel,
        plot.report.jerk
    ));
}

/// Renders the **Contact Sheet** listing segments/shots of the preview or active track.
fn ui_lab_contact_sheet(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let track_opt: Option<(Track, bool)> = args
        .workspace
        .player
        .active_track()
        .map(|t| (t.clone(), true))
        .or_else(|| {
            args.workspace
                .lab
                .preview_track
                .as_ref()
                .map(|t| (t.clone(), false))
        });

    section_box(
        ui,
        "flight_lab_contact_sheet",
        ic::FILM_STRIP,
        "Contact Sheet",
        false,
        |ui| {
            let Some((track, is_player_track)) = track_opt else {
                ui.label(RichText::new("No active or preview track.").size(11.0).weak());
                return;
            };
            let seg_count = track.segments.len();
            let cuts = track.cut_times().len();
            let chases = track.chase_spans().len();
            stat_row(
                ui,
                "Summary",
                fmt_into(
                    args.label_buf,
                    format_args!(
                        "{seg_count} seg  |  {cuts} cuts  |  {chases} chases  |  {:.1} s",
                        track.duration_s()
                    ),
                ),
            );

            egui::ScrollArea::vertical()
                .id_salt("flight_lab_contact_scroll")
                .max_height(132.0)
                .show(ui, |ui| {
                    let mut t_acc = 0.0_f64;
                    let mut shot_idx = 0usize;
                    let w = ui.available_width().max(80.0);
                    for seg in &track.segments {
                        match seg {
                            Segment::Cut(c) => {
                                let txt = fmt_into(
                                    args.label_buf,
                                    format_args!(
                                        "{}  {} @ {:.1} s",
                                        ic::SCISSORS,
                                        c.kind.label(),
                                        t_acc
                                    ),
                                );
                                ui.label(
                                    RichText::new(txt)
                                        .size(10.5)
                                        .color(Color32::from_rgb(230, 120, 72)),
                                );
                            }
                            Segment::Shot(s) => {
                                shot_idx += 1;
                                let dur = s.duration_s.max(0.0);
                                let focal = s.entry().state.lens.focal_mm;
                                let is_chase = matches!(
                                    s.speed_profile.kind,
                                    flight_navigator::fnav_core::ProfileKind::Chase
                                );
                                let chase_tag = if is_chase { ic::LIGHTNING } else { "" };
                                let txt = fmt_into(
                                    args.label_buf,
                                    format_args!(
                                        "S{:02}  {}  {:.1}s  {:.0}mm  {chase_tag}",
                                        shot_idx,
                                        s.kind().label(),
                                        dur,
                                        focal
                                    ),
                                );
                                if ui
                                    .add_sized(
                                        [w, ROW_H + 1.0],
                                        egui::Button::new(RichText::new(txt).monospace().size(10.5)),
                                    )
                                    .on_hover_text("Click to jump to this shot's start time.")
                                    .clicked()
                                {
                                    if is_player_track {
                                        (args.on_command)(FlightCommand::Seek(t_acc));
                                    } else {
                                        let pose =
                                            track.eval(t_acc).state.to_camera_pose(0.05, 5000.0);
                                        (args.on_seek_pose)(pose);
                                    }
                                }
                                t_acc += dur;
                            }
                            Segment::Transition(tr) => {
                                let dur = tr.duration_s.max(0.0);
                                if seg_count <= 24 || dur >= 1.0 {
                                    shot_idx += 1;
                                    let focal = tr.entry().state.lens.focal_mm;
                                    let txt = fmt_into(
                                        args.label_buf,
                                        format_args!(
                                            "T{:02}  Glide  {:.1}s  {:.0}mm",
                                            shot_idx, dur, focal
                                        ),
                                    );
                                    if ui
                                        .add_sized(
                                            [w, ROW_H + 1.0],
                                            egui::Button::new(
                                                RichText::new(txt).monospace().size(10.5),
                                            ),
                                        )
                                        .on_hover_text("Click to jump to this segment start.")
                                        .clicked()
                                    {
                                        if is_player_track {
                                            (args.on_command)(FlightCommand::Seek(t_acc));
                                        } else {
                                            let pose = track
                                                .eval(t_acc)
                                                .state
                                                .to_camera_pose(0.05, 5000.0);
                                            (args.on_seek_pose)(pose);
                                        }
                                    }
                                }
                                t_acc += dur;
                            }
                        }
                    }
                });
        },
    );
}

/// Renders the collapsible **Flight Stats** section inside Flight Lab.
fn ui_lab_flight_stats(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    section_box(
        ui,
        "flight_lab_stats",
        ic::CHART_LINE,
        "Flight Stats",
        false,
        |ui| {
            ui.checkbox(args.is_counters, "IS counters (debug)").on_hover_text(
                "Count OptiX intersection-program invocations per hit group (counted frames are slower).",
            );
            if *args.is_counters
                && let Some(c) = args.last_is_counts
            {
                stat_row(
                    ui,
                    "RT (counted)",
                    fmt_into(args.label_buf, format_args!("{:.2} ms", args.rt_ms)),
                );
                for g in IsGroup::ALL {
                    stat_row(
                        ui,
                        g.label(),
                        fmt_into(args.label_buf, format_args!("{}", c.get(g))),
                    );
                }
                stat_row(
                    ui,
                    "IS total",
                    fmt_into(args.label_buf, format_args!("{}", c.total_is())),
                );
            }

            let stats = &mut *args.flight_stats;
            let load = stats.load;
            let live_rt_ms = stats.latest().map_or(args.rt_ms, |s| s.rt_ms);
            stat_row(
                ui,
                "RT",
                fmt_into(args.label_buf, format_args!("{live_rt_ms:.1} ms")),
            );
            let summary = stats.summary();
            if let Some(s) = summary {
                stat_row(
                    ui,
                    "RT min/med/p95/max",
                    fmt_into(
                        args.label_buf,
                        format_args!(
                            "{:.1} / {:.1} / {:.1} / {:.1} ms ({}f)",
                            s.min, s.median, s.p95, s.max, s.frames
                        ),
                    ),
                );
            }
            stat_row(
                ui,
                "Visible",
                fmt_into(
                    args.label_buf,
                    format_args!(
                        "{} inst, {} GAS, {} crowd",
                        load.visible, args.csg_stats.compounds, args.crowd_actor_count
                    ),
                ),
            );
            stat_row(
                ui,
                "LOD 0/1/2",
                fmt_into(
                    args.label_buf,
                    format_args!(
                        "{} / {} / {}, culled {} (bias {:.2})",
                        load.lod[0], load.lod[1], load.lod[2], load.culled, args.csg_lod_bias
                    ),
                ),
            );
            stat_row(
                ui,
                "Camera",
                fmt_into(
                    args.label_buf,
                    format_args!("{:.1} m/s  {:.0} deg/s", stats.speed_mps, stats.turn_dps),
                ),
            );

            let dur = args.workspace.player.duration_s().max(1.0) as f32;
            let rt_max = stats
                .cur_loop()
                .iter()
                .chain(stats.prev_loop())
                .map(|p| p.rt_ms)
                .fold(1.0_f32, f32::max);
            let w = ui.available_width().max(80.0);
            let (rect, _) = ui.allocate_exact_size(egui::vec2(w, STATS_GRAPH_H), Sense::hover());
            let painter = ui.painter_at(rect);
            painter.rect_filled(rect, 2.0, ui.visuals().extreme_bg_color);
            paint_loop_curve(&painter, rect, stats.prev_loop(), dur, rt_max, Color32::from_gray(120));
            paint_loop_curve(
                &painter,
                rect,
                stats.cur_loop(),
                dur,
                rt_max,
                Color32::from_rgb(118, 185, 0),
            );
            painter.text(
                rect.left_top() + egui::vec2(3.0, 2.0),
                egui::Align2::LEFT_TOP,
                fmt_into(args.label_buf, format_args!("{rt_max:.0} ms")),
                FontId::monospace(9.5),
                ui.visuals().weak_text_color(),
            );

            if let Some(s) = summary {
                let mut bins = [0u32; HIST_BINS];
                stats.histogram(s.min, s.max, &mut bins);
                let max_bin = bins.iter().copied().max().unwrap_or(1).max(1) as f32;
                let (hrect, _) = ui.allocate_exact_size(egui::vec2(w, 26.0), Sense::hover());
                let hpainter = ui.painter_at(hrect);
                hpainter.rect_filled(hrect, 2.0, ui.visuals().extreme_bg_color);
                let bw = hrect.width() / HIST_BINS as f32;
                for (i, &count) in bins.iter().enumerate() {
                    if count == 0 {
                        continue;
                    }
                    let frac = (count as f32 / max_bin).clamp(0.0, 1.0);
                    let bh = (hrect.height() - 2.0) * frac;
                    let x0 = hrect.min.x + i as f32 * bw;
                    let bar = Rect::from_min_max(
                        Pos2::new(x0 + 0.5, hrect.max.y - bh),
                        Pos2::new((x0 + bw - 0.5).max(x0 + 1.0), hrect.max.y),
                    );
                    hpainter.rect_filled(bar, 1.0, Color32::from_rgb(118, 185, 0).gamma_multiply(0.8));
                }
            }

            let slow_spots: Vec<crate::flight_stats::SlowSpot> =
                stats.slow_spots().copied().collect();
            if !slow_spots.is_empty() {
                ui.add_space(2.0);
                ui.label(RichText::new("Slowest Rail Spots").size(10.0).weak());
                for spot in slow_spots {
                    let txt = fmt_into(
                        args.label_buf,
                        format_args!(
                            "t={:.1}s  {:.1} ms  ({:.0}, {:.0}, {:.0})",
                            spot.rail_t, spot.rt_ms, spot.eye[0], spot.eye[1], spot.eye[2]
                        ),
                    );
                    if ui
                        .add_sized(
                            [w, ROW_H],
                            egui::Button::new(RichText::new(txt).monospace().size(10.0)),
                        )
                        .on_hover_text("Click to seek player to this slow frame.")
                        .clicked()
                    {
                        (args.on_command)(FlightCommand::Seek(f64::from(spot.rail_t)));
                    }
                }
            }

            ui.add_space(2.0);
            if ui
                .add_enabled(
                    stats.frames() > 0,
                    egui::Button::new(format!("{}  Export CSV", ic::FLOPPY_DISK))
                        .min_size(egui::vec2(ui.available_width(), ROW_H + 2.0)),
                )
                .on_hover_text("Export accumulated flight telemetry frames to perf CSV.")
                .clicked()
            {
                (args.on_export_flight_csv)();
            }
        },
    );
}

/// Renders the **Flight Player** dock panel (Winamp-style Now Playing, phosphor transport controls,
/// interactive shot/cut/chase progress bar, Playlist Queue, and Repo/User/Curated Library Browser).
pub fn ui_flight_player(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    ui.spacing_mut().item_spacing.y = 4.0;

    ui_player_now_playing(ui, args);
    ui_player_transport_row(ui, args);
    ui_player_progress_bar(ui, args);

    slider_f32_row(ui, "Cam Smooth", args.cam_smooth_tau, 0.0..=1.0, 2, " s");
    slider_f32_row(
        ui,
        "Time Scale",
        args.time_scale,
        TIME_SCALE_MIN..=TIME_SCALE_MAX,
        2,
        "x",
    );

    ui.add_space(2.0);
    ui_player_playlist_queue(ui, args);
    ui.add_space(2.0);
    ui_player_library_browser(ui, args);

    if !args.workspace.player.status_msg.is_empty() {
        ui.add_space(2.0);
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(RichText::new(ic::INFO).size(11.0).weak());
            ui.add(
                egui::Label::new(
                    RichText::new(&args.workspace.player.status_msg)
                        .size(10.5)
                        .weak(),
                )
                .truncate(),
            );
        });
    }
}

/// Renders the framed Now Playing card at the top of Flight Player.
fn ui_player_now_playing(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let player = &args.workspace.player;
    let dark = ui.visuals().dark_mode;
    let card_bg = if dark {
        Color32::from_rgb(24, 28, 34)
    } else {
        Color32::from_rgb(236, 240, 246)
    };
    let card_stroke = if dark {
        Color32::from_rgb(56, 66, 80)
    } else {
        Color32::from_rgb(196, 206, 220)
    };

    let (state_lbl, state_col) = match &player.state {
        PlayerState::Playing { .. } => ("PLAYING", Color32::from_rgb(92, 198, 108)),
        PlayerState::Paused { .. } => ("PAUSED", Color32::from_rgb(228, 178, 66)),
        PlayerState::Transition { .. } => ("TRANSITION", Color32::from_rgb(86, 174, 236)),
        PlayerState::Loading { .. } => ("LOADING", Color32::from_rgb(228, 134, 64)),
        PlayerState::Idle => ("IDLE", ui.visuals().weak_text_color()),
    };

    let origin_lbl = player.active_slot().map_or("Empty", |s| s.origin.label());
    let cur_t = player.time_s;
    let dur = player.duration_s();

    egui::Frame::NONE
        .fill(card_bg)
        .stroke(Stroke::new(1.0, card_stroke))
        .corner_radius(4.0)
        .inner_margin(egui::Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    egui::Label::new(RichText::new(player.active_title()).size(12.5).strong())
                        .truncate(),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(state_lbl)
                            .size(10.0)
                            .strong()
                            .color(state_col),
                    );
                    ui.label(
                        RichText::new(fmt_into(args.label_buf, format_args!("[{origin_lbl}]")))
                            .size(10.0)
                            .weak(),
                    );
                });
            });

            ui.horizontal(|ui| {
                let time_txt = fmt_into(
                    args.label_buf,
                    format_args!("{} / {}", format_mmss(cur_t), format_mmss(dur)),
                );
                ui.label(RichText::new(time_txt).monospace().size(11.5).strong());
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if player.last_active_chase {
                        ui.label(
                            RichText::new(format!("{} CHASE", ic::LIGHTNING))
                                .size(10.0)
                                .strong()
                                .color(Color32::from_rgb(240, 152, 54)),
                        );
                    }
                    let kin_txt = fmt_into(
                        args.label_buf,
                        format_args!(
                            "{:.1} m/s  {:.1} deg  |  {}",
                            player.last_speed_mps,
                            player.last_bank_rad.to_degrees(),
                            args.active_scene_label
                        ),
                    );
                    ui.add(
                        egui::Label::new(RichText::new(kin_txt).monospace().size(10.0).weak())
                            .truncate(),
                    );
                });
            });
        });
}

/// Renders the 100% phosphor-icon transport bar (`Prev`, `Play/Pause`, `Stop`, `Next`, `Repeat`, `Shuffle`, `Record`).
fn ui_player_transport_row(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let player = &args.workspace.player;
    let is_playing = player.is_active();
    let is_paused = matches!(player.state, PlayerState::Paused { .. });
    let can_play = player.can_play();
    let has_queue = !player.queue.is_empty();
    let repeat = player.repeat;
    let shuffle = player.shuffle;
    let rec_active = args.is_recording || player.recording_playlist;

    let accent = active_accent_tint(ui);
    let rec_fill = if ui.visuals().dark_mode {
        Color32::from_rgb(122, 36, 36)
    } else {
        Color32::from_rgb(246, 194, 194)
    };

    equal_grid(ui, 7, 7, |ui, idx, w| {
        let h = ROW_H + 6.0;
        match idx {
            0 => {
                if ui
                    .add_enabled(
                        has_queue,
                        egui::Button::new(ic::SKIP_BACK).min_size(egui::vec2(w, h)),
                    )
                    .on_hover_text("Previous track (or rewind if > 2 s).")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::Prev);
                }
            }
            1 => {
                let icon = if is_playing { ic::PAUSE } else { ic::PLAY };
                let mut btn = egui::Button::new(icon).min_size(egui::vec2(w, h));
                if is_playing {
                    btn = btn.fill(accent);
                }
                if ui
                    .add_enabled(is_playing || can_play, btn)
                    .on_hover_text(if is_playing {
                        "Pause playback"
                    } else {
                        "Play active track"
                    })
                    .clicked()
                {
                    (args.on_command)(if is_playing {
                        FlightCommand::Pause
                    } else {
                        FlightCommand::Play
                    });
                }
            }
            2 => {
                if ui
                    .add_enabled(
                        is_playing || is_paused,
                        egui::Button::new(ic::STOP).min_size(egui::vec2(w, h)),
                    )
                    .on_hover_text("Stop playback and rewind to 0:00.")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::Stop);
                }
            }
            3 => {
                if ui
                    .add_enabled(
                        has_queue,
                        egui::Button::new(ic::SKIP_FORWARD).min_size(egui::vec2(w, h)),
                    )
                    .on_hover_text("Next track in playlist.")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::Next);
                }
            }
            4 => {
                let (icon, tip, next_mode) = match repeat {
                    RepeatMode::Off => (ic::REPEAT, "Repeat: Off (click -> All)", RepeatMode::All),
                    RepeatMode::All => (ic::REPEAT, "Repeat: All (click -> One)", RepeatMode::One),
                    RepeatMode::One => (
                        ic::REPEAT_ONCE,
                        "Repeat: One (click -> Off)",
                        RepeatMode::Off,
                    ),
                };
                let mut btn = egui::Button::new(icon).min_size(egui::vec2(w, h));
                if repeat != RepeatMode::Off {
                    btn = btn.fill(accent);
                }
                if ui.add(btn).on_hover_text(tip).clicked() {
                    (args.on_command)(FlightCommand::SetRepeat(next_mode));
                }
            }
            5 => {
                let mut btn = egui::Button::new(ic::SHUFFLE).min_size(egui::vec2(w, h));
                if shuffle {
                    btn = btn.fill(accent);
                }
                if ui
                    .add(btn)
                    .on_hover_text("Toggle deterministic seeded playlist shuffle.")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::SetShuffle(!shuffle));
                }
            }
            _ => {
                let mut btn = egui::Button::new(ic::RECORD).min_size(egui::vec2(w, h));
                if rec_active {
                    btn = btn.fill(rec_fill);
                }
                if ui
                    .add_enabled(can_play || rec_active, btn)
                    .on_hover_text(
                        "Record entire playlist pass to MP4 (auto-stops at playlist end).",
                    )
                    .clicked()
                {
                    (args.on_command)(FlightCommand::ToggleRecordPlaylist);
                }
            }
        }
    });
}

/// Paints the interactive timeline scrubber with shot boundaries, cut ticks, and chase spans.
fn ui_player_progress_bar(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let player = &args.workspace.player;
    let dur = player.duration_s();
    let cur_t = player.time_s;
    let w = ui.available_width().max(80.0);
    let (rect, resp) =
        ui.allocate_exact_size(egui::vec2(w, PROGRESS_BAR_H), Sense::click_and_drag());
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, 3.0, ui.visuals().extreme_bg_color);

    if dur > 1.0e-4 {
        let frac = (cur_t / dur).clamp(0.0, 1.0) as f32;
        let fill_rect = Rect::from_min_max(
            rect.min,
            Pos2::new(rect.left() + rect.width() * frac, rect.bottom()),
        );
        let fill_col = if ui.visuals().dark_mode {
            Color32::from_rgba_unmultiplied(64, 132, 212, 110)
        } else {
            Color32::from_rgba_unmultiplied(80, 148, 226, 110)
        };
        painter.rect_filled(fill_rect, 3.0, fill_col);

        if let Some(track) = player.active_track() {
            for (ts, te) in track.chase_spans() {
                let x0 = rect.left() + rect.width() * (ts / dur).clamp(0.0, 1.0) as f32;
                let x1 = rect.left() + rect.width() * (te / dur).clamp(0.0, 1.0) as f32;
                let span_rect = Rect::from_min_max(
                    Pos2::new(x0, rect.top() + 2.0),
                    Pos2::new(x1.max(x0 + 2.0), rect.bottom() - 2.0),
                );
                painter.rect_filled(
                    span_rect,
                    1.5,
                    Color32::from_rgba_unmultiplied(240, 152, 54, 115),
                );
            }

            let shot_stroke =
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(180, 196, 216, 110));
            for (ts, _te, _kind, _prof) in track.shot_spans() {
                if ts > 1.0e-3 {
                    let x = rect.left() + rect.width() * (ts / dur).clamp(0.0, 1.0) as f32;
                    painter.line_segment(
                        [
                            Pos2::new(x, rect.top() + 3.0),
                            Pos2::new(x, rect.bottom() - 3.0),
                        ],
                        shot_stroke,
                    );
                }
            }

            let cut_stroke = Stroke::new(1.75, Color32::from_rgb(236, 94, 68));
            for (tc, _ck) in track.cut_times() {
                let x = rect.left() + rect.width() * (tc / dur).clamp(0.0, 1.0) as f32;
                painter.line_segment(
                    [
                        Pos2::new(x, rect.top() + 1.0),
                        Pos2::new(x, rect.bottom() - 1.0),
                    ],
                    cut_stroke,
                );
            }
        }

        let head_x = rect.left() + rect.width() * frac;
        painter.line_segment(
            [
                Pos2::new(head_x, rect.top()),
                Pos2::new(head_x, rect.bottom()),
            ],
            Stroke::new(2.0, Color32::from_rgb(235, 245, 255)),
        );

        if (resp.clicked() || resp.dragged())
            && let Some(pos) = resp.interact_pointer_pos()
        {
            let u = f64::from(((pos.x - rect.left()) / rect.width().max(1.0)).clamp(0.0, 1.0));
            (args.on_command)(FlightCommand::Seek(u * dur));
        }

        if resp.hovered()
            && let Some(pos) = ui.ctx().pointer_hover_pos()
        {
            let u = f64::from(((pos.x - rect.left()) / rect.width().max(1.0)).clamp(0.0, 1.0));
            let hover_t = u * dur;
            resp.on_hover_text(format!(
                "Seek to {} / {}",
                format_mmss(hover_t),
                format_mmss(dur)
            ));
        }
    }

    painter.rect_stroke(
        rect,
        3.0,
        Stroke::new(0.75, ui.visuals().widgets.noninteractive.bg_stroke.color),
        egui::StrokeKind::Inside,
    );
}

/// Renders the **Playlist Queue** with reorder arrows, transition selector, `hold_s`, delete, and save.
fn ui_player_playlist_queue(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let q_len = args.workspace.player.queue.len();
    let title = format!("Playlist Queue ({q_len})");

    section_box(ui, "flight_player_queue", ic::PLAYLIST, &title, true, |ui| {
        if q_len == 0 {
            ui.label(
                RichText::new("Queue is empty. Play or enqueue a track from the Library below.")
                    .size(11.0)
                    .weak(),
            );
        } else {
            let active_idx = args.workspace.player.active_idx;
            let accent = active_accent_tint(ui);
            let mut deferred_cmds: Vec<FlightCommand> = Vec::new();

            egui::ScrollArea::vertical()
                .id_salt("flight_player_queue_scroll")
                .max_height(168.0)
                .show(ui, |ui| {
                    for (idx, slot) in args.workspace.player.queue.iter().enumerate() {
                        let is_active = idx == active_idx;
                        let frame_fill = if is_active {
                            accent.gamma_multiply(0.55)
                        } else {
                            Color32::TRANSPARENT
                        };
                        egui::Frame::NONE
                            .fill(frame_fill)
                            .corner_radius(3.0)
                            .inner_margin(egui::Margin::symmetric(4, 3))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let idx_txt =
                                        fmt_into(args.label_buf, format_args!("{:02}.", idx + 1));
                                    ui.label(RichText::new(idx_txt).monospace().size(10.5).weak());
                                    if slot.stale {
                                        ui.label(
                                            RichText::new(ic::WARNING)
                                                .size(11.0)
                                                .color(Color32::from_rgb(232, 166, 54)),
                                        )
                                        .on_hover_text(
                                            "Scene geometry hash changed since this track was saved.",
                                        );
                                    }
                                    let title_resp = ui
                                        .add(
                                            egui::Label::new(RichText::new(&slot.title).size(11.0))
                                                .truncate()
                                                .sense(Sense::click()),
                                        )
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .on_hover_text(&slot.description);
                                    if title_resp.clicked() {
                                        deferred_cmds.push(FlightCommand::SelectPlaylistEntry(idx));
                                    }

                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        if ui
                                            .add_sized([20.0, ROW_H], egui::Button::new(ic::TRASH))
                                            .on_hover_text("Remove from playlist")
                                            .clicked()
                                        {
                                            deferred_cmds
                                                .push(FlightCommand::RemovePlaylistEntry(idx));
                                        }
                                        if ui
                                            .add_enabled(
                                                idx + 1 < q_len,
                                                egui::Button::new(ic::ARROW_DOWN)
                                                    .min_size(egui::vec2(20.0, ROW_H)),
                                            )
                                            .on_hover_text("Move down")
                                            .clicked()
                                        {
                                            deferred_cmds.push(FlightCommand::ReorderPlaylist {
                                                from: idx,
                                                to: idx + 1,
                                            });
                                        }
                                        if ui
                                            .add_enabled(
                                                idx > 0,
                                                egui::Button::new(ic::ARROW_UP)
                                                    .min_size(egui::vec2(20.0, ROW_H)),
                                            )
                                            .on_hover_text("Move up")
                                            .clicked()
                                        {
                                            deferred_cmds.push(FlightCommand::ReorderPlaylist {
                                                from: idx,
                                                to: idx - 1,
                                            });
                                        }
                                        let dur_txt = fmt_into(
                                            args.label_buf,
                                            format_args!("{:.0}s", slot.track.duration_s()),
                                        );
                                        ui.label(
                                            RichText::new(dur_txt).monospace().size(10.0).weak(),
                                        );
                                    });
                                });

                                ui.horizontal(|ui| {
                                    ui.add_space(18.0);
                                    let cur_tr_lbl = transition_label(slot.transition_in);
                                    let mut new_tr = slot.transition_in;
                                    egui::ComboBox::from_id_salt(("pl_tr", idx))
                                        .width(86.0)
                                        .selected_text(cur_tr_lbl)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut new_tr,
                                                PlaylistTransition::Cut,
                                                "Cut",
                                            );
                                            ui.selectable_value(
                                                &mut new_tr,
                                                PlaylistTransition::FadeBlack { s: 0.75 },
                                                "Fade 0.75s",
                                            );
                                            ui.selectable_value(
                                                &mut new_tr,
                                                PlaylistTransition::FlyThrough,
                                                "FlyThrough",
                                            );
                                        });
                                    if new_tr != slot.transition_in {
                                        deferred_cmds.push(FlightCommand::SetEntryTransition {
                                            idx,
                                            transition: new_tr,
                                        });
                                    }

                                    ui.label(RichText::new("Hold").size(10.0).weak());
                                    let mut hold_val = slot.hold_s;
                                    if ui
                                        .add_sized(
                                            [48.0, ROW_H],
                                            egui::DragValue::new(&mut hold_val)
                                                .range(0.0..=30.0)
                                                .speed(0.1)
                                                .suffix("s"),
                                        )
                                        .changed()
                                    {
                                        deferred_cmds.push(FlightCommand::SetEntryHold {
                                            idx,
                                            hold_s: hold_val,
                                        });
                                    }
                                });
                            });
                    }
                });

            for cmd in deferred_cmds {
                (args.on_command)(cmd);
            }
        }

        ui.add_space(2.0);
        prop_row(ui, "Playlist", |ui| {
            ui.add_sized(
                [ui.available_width().max(48.0), ROW_H],
                egui::TextEdit::singleline(&mut args.workspace.player.playlist_name),
            );
        });
        let pl_name = args.workspace.player.playlist_name.clone();
        equal_grid(ui, 2, 2, |ui, idx, w| match idx {
            0 => {
                if ui
                    .add_enabled(
                        q_len > 0,
                        egui::Button::new(format!("{}  Save User", ic::FLOPPY_DISK))
                            .min_size(egui::vec2(w, ROW_H + 2.0)),
                    )
                    .on_hover_text("Save playlist atomically to ~/.nv-rs/flights/")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::SaveCurrentPlaylist {
                        name: pl_name.clone(),
                        to_repo: false,
                    });
                }
            }
            _ => {
                if ui
                    .add_enabled(
                        q_len > 0,
                        egui::Button::new(format!("{}  Save Repo", ic::FOLDER_OPEN))
                            .min_size(egui::vec2(w, ROW_H + 2.0)),
                    )
                    .on_hover_text("Save playlist atomically to assets/flights/")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::SaveCurrentPlaylist {
                        name: pl_name.clone(),
                        to_repo: true,
                    });
                }
            }
        });
    });
}

/// Renders the grouped **Library Browser** (`Scene Curated`, `Repo`, `User`, and `Saved Playlists`).
fn ui_player_library_browser(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    section_box(
        ui,
        "flight_player_library",
        ic::QUEUE,
        "Flight Library",
        true,
        |ui| {
            let err_count = args.workspace.player.library.load_errors.len();
            ui.horizontal(|ui| {
                if ui
                    .add_sized(
                        [96.0, ROW_H + 2.0],
                        egui::Button::new(format!("{}  Rescan", ic::ARROWS_CLOCKWISE)),
                    )
                    .on_hover_text("Rescan assets/flights and ~/.nv-rs/flights on disk.")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::RescanLibrary);
                }
                if err_count > 0 {
                    ui.label(
                        RichText::new(fmt_into(
                            args.label_buf,
                            format_args!("{} {err_count} load errors", ic::WARNING),
                        ))
                        .size(10.5)
                        .color(Color32::from_rgb(232, 166, 54)),
                    );
                }
            });

            let mut deferred: Vec<FlightCommand> = Vec::new();
            egui::ScrollArea::vertical()
                .id_salt("flight_player_lib_scroll")
                .max_height(210.0)
                .show(ui, |ui| {
                    ui.label(RichText::new("Scene Curated").size(10.5).strong().weak());
                    if args.workspace.player.curated.is_empty() {
                        ui.label(
                            RichText::new("  No curated rails for active scene.")
                                .size(10.5)
                                .weak(),
                        );
                    } else {
                        for (i, c) in args.workspace.player.curated.iter().enumerate() {
                            match library_item_row(
                                ui,
                                args.label_buf,
                                &c.label,
                                &c.description,
                                TrackOrigin::Curated.label(),
                                c.duration_s,
                                false,
                            ) {
                                Some(LibraryRowAction::Play) => {
                                    deferred.push(FlightCommand::PlayCuratedRail(i));
                                }
                                Some(LibraryRowAction::Enqueue) => {
                                    deferred.push(FlightCommand::EnqueueCuratedRail(i));
                                }
                                None => {}
                            }
                        }
                    }

                    ui.add_space(3.0);
                    ui.label(
                        RichText::new("Repo (assets/flights)")
                            .size(10.5)
                            .strong()
                            .weak(),
                    );
                    let mut repo_any = false;
                    for t in &args.workspace.player.library.tracks {
                        if t.source == LibrarySource::Repo {
                            repo_any = true;
                            let tip = format!("Scene: {} ({})", t.scene_id, t.path.display());
                            match library_item_row(
                                ui,
                                args.label_buf,
                                &t.name,
                                &tip,
                                "Repo",
                                t.duration_s,
                                t.stale,
                            ) {
                                Some(LibraryRowAction::Play) => {
                                    deferred.push(FlightCommand::PlayLibraryTrack(t.path.clone()));
                                }
                                Some(LibraryRowAction::Enqueue) => {
                                    deferred
                                        .push(FlightCommand::EnqueueLibraryTrack(t.path.clone()));
                                }
                                None => {}
                            }
                        }
                    }
                    if !repo_any {
                        ui.label(RichText::new("  No repo tracks found.").size(10.5).weak());
                    }

                    ui.add_space(3.0);
                    ui.label(
                        RichText::new("User (~/.nv-rs/flights)")
                            .size(10.5)
                            .strong()
                            .weak(),
                    );
                    let mut user_any = false;
                    for t in &args.workspace.player.library.tracks {
                        if t.source == LibrarySource::User {
                            user_any = true;
                            let tip = format!("Scene: {} ({})", t.scene_id, t.path.display());
                            match library_item_row(
                                ui,
                                args.label_buf,
                                &t.name,
                                &tip,
                                "User",
                                t.duration_s,
                                t.stale,
                            ) {
                                Some(LibraryRowAction::Play) => {
                                    deferred.push(FlightCommand::PlayLibraryTrack(t.path.clone()));
                                }
                                Some(LibraryRowAction::Enqueue) => {
                                    deferred
                                        .push(FlightCommand::EnqueueLibraryTrack(t.path.clone()));
                                }
                                None => {}
                            }
                        }
                    }
                    if !user_any {
                        ui.label(
                            RichText::new("  No user tracks saved yet.")
                                .size(10.5)
                                .weak(),
                        );
                    }

                    if !args.workspace.player.library.playlists.is_empty() {
                        ui.add_space(3.0);
                        ui.label(RichText::new("Saved Playlists").size(10.5).strong().weak());
                        for pl in &args.workspace.player.library.playlists {
                            ui.horizontal(|ui| {
                                let src = match pl.source {
                                    LibrarySource::Repo => "Repo",
                                    LibrarySource::User => "User",
                                };
                                ui.add(
                                    egui::Label::new(RichText::new(&pl.name).size(11.0)).truncate(),
                                )
                                .on_hover_text(pl.path.display().to_string());
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui
                                        .add_sized(
                                            [24.0, ROW_H],
                                            egui::Button::new(ic::FOLDER_OPEN),
                                        )
                                        .on_hover_text("Load playlist into queue")
                                        .clicked()
                                    {
                                        deferred.push(FlightCommand::LoadPlaylistFile(
                                            pl.path.clone(),
                                        ));
                                    }
                                    let meta = fmt_into(
                                        args.label_buf,
                                        format_args!("[{src}] {}t", pl.entry_count),
                                    );
                                    ui.label(RichText::new(meta).monospace().size(10.0).weak());
                                });
                            });
                        }
                    }
                });

            for cmd in deferred {
                (args.on_command)(cmd);
            }
        },
    );
}

/// Action emitted by a single row in the Library Browser.
enum LibraryRowAction {
    Play,
    Enqueue,
}

/// Renders one row in the Library Browser with Play and Enqueue icon buttons.
fn library_item_row(
    ui: &mut Ui,
    buf: &mut String,
    name: &str,
    tooltip: &str,
    badge: &str,
    duration_s: f64,
    stale: bool,
) -> Option<LibraryRowAction> {
    let mut action = None;
    ui.horizontal(|ui| {
        if stale {
            ui.label(
                RichText::new(ic::WARNING)
                    .size(10.5)
                    .color(Color32::from_rgb(232, 166, 54)),
            )
            .on_hover_text("Stale: scene content hash differs from when this track was created.");
        }
        ui.add(egui::Label::new(RichText::new(name).size(11.0)).truncate())
            .on_hover_text(tooltip);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
                .add_sized([22.0, ROW_H], egui::Button::new(ic::PLUS))
                .on_hover_text("Append to playlist queue")
                .clicked()
            {
                action = Some(LibraryRowAction::Enqueue);
            }
            if ui
                .add_sized([22.0, ROW_H], egui::Button::new(ic::PLAY))
                .on_hover_text("Play now")
                .clicked()
            {
                action = Some(LibraryRowAction::Play);
            }
            let meta = fmt_into(buf, format_args!("[{badge}] {:.0}s", duration_s));
            ui.label(RichText::new(meta).monospace().size(10.0).weak());
        });
    });
    action
}

// ---------------------------------------------------------------------------
// Shared compact layout & painter helpers matching `crate::ui` visual language
// ---------------------------------------------------------------------------

#[inline]
fn fmt_into<'a>(buf: &'a mut String, args: std::fmt::Arguments<'_>) -> &'a str {
    buf.clear();
    let Ok(()) = std::fmt::write(buf, args) else {
        unreachable!("formatting into String cannot fail");
    };
    buf.as_str()
}

fn format_mmss(seconds: f64) -> String {
    let s = if seconds.is_finite() && seconds > 0.0 {
        seconds
    } else {
        0.0
    };
    let mins = (s / 60.0).floor() as u64;
    let rem = s - (mins as f64) * 60.0;
    format!("{mins:02}:{rem:04.1}")
}

const fn transition_label(tr: PlaylistTransition) -> &'static str {
    match tr {
        PlaylistTransition::Cut => "Cut",
        PlaylistTransition::FadeBlack { .. } => "Fade",
        PlaylistTransition::FlyThrough => "FlyThrough",
    }
}

fn active_accent_tint(ui: &Ui) -> Color32 {
    if ui.visuals().dark_mode {
        Color32::from_rgb(38, 70, 110)
    } else {
        Color32::from_rgb(208, 226, 250)
    }
}

fn mode_btn(ui: &Ui, text: String, active: bool, w: f32) -> egui::Button<'static> {
    let mut btn = egui::Button::new(text).min_size(egui::vec2(w, ROW_H + 4.0));
    if active {
        btn = btn.fill(active_accent_tint(ui));
    }
    btn
}

fn section_box(
    ui: &mut Ui,
    id_salt: &str,
    icon: &str,
    title: &str,
    default_open: bool,
    body: impl FnOnce(&mut Ui),
) {
    let id = egui::Id::new(id_salt);
    let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
        ui.ctx(),
        id,
        default_open,
    );
    let total_w = ui.available_width().max(80.0);
    let header_h = 20.0;
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(total_w, header_h), Sense::click());
    if resp.clicked() {
        state.toggle(ui);
    }
    let accent = Color32::from_rgb(86, 156, 214);
    let fill = ui
        .visuals()
        .faint_bg_color
        .lerp_to_gamma(accent, if resp.hovered() { 0.22 } else { 0.14 });
    ui.painter().rect_filled(rect, 3.0, fill);
    ui.painter().rect_filled(
        Rect::from_min_size(rect.min, egui::vec2(3.0, rect.height())),
        1.5,
        accent,
    );
    let caret = if state.is_open() {
        ic::CARET_DOWN
    } else {
        ic::CARET_RIGHT
    };
    let text_col = ui.visuals().text_color().lerp_to_gamma(accent, 0.40);
    ui.painter().text(
        Pos2::new(rect.left() + 8.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        format!("{icon}  {caret}  {title}"),
        FontId::proportional(11.5),
        text_col,
    );
    state.show_body_indented(&resp, ui, |ui| {
        ui.add_space(2.0);
        body(ui);
        ui.add_space(2.0);
    });
}

fn equal_grid(ui: &mut Ui, cols: usize, count: usize, mut item_ui: impl FnMut(&mut Ui, usize, f32)) {
    let gap = 4.0;
    let total_w = ui.available_width().max(48.0);
    let cell_w = ((total_w - gap * (cols.saturating_sub(1) as f32)) / (cols as f32)).max(20.0);
    let mut idx = 0;
    while idx < count {
        ui.horizontal(|ui| {
            ui.set_max_width(total_w);
            ui.spacing_mut().item_spacing.x = gap;
            for _ in 0..cols {
                if idx < count {
                    ui.allocate_ui_with_layout(
                        egui::vec2(cell_w, ROW_H + 4.0),
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
    let label_w = LABEL_W.min((total_w * 0.36).max(48.0));
    let rem_w = (total_w - label_w - gap).max(60.0);
    ui.allocate_ui_with_layout(
        egui::vec2(total_w, ROW_H + 2.0),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.set_max_width(total_w);
            ui.spacing_mut().item_spacing.x = gap;
            let (label_rect, _) =
                ui.allocate_exact_size(egui::vec2(label_w, ROW_H), Sense::hover());
            let galley = ui.painter().layout_no_wrap(
                label.to_owned(),
                FontId::proportional(11.0),
                ui.visuals().weak_text_color(),
            );
            ui.painter()
                .with_clip_rect(label_rect.intersect(ui.clip_rect()))
                .galley(
                    Pos2::new(
                        label_rect.left(),
                        label_rect.center().y - galley.size().y * 0.5,
                    ),
                    galley,
                    Color32::WHITE,
                );
            ui.allocate_ui_with_layout(
                egui::vec2(rem_w, ROW_H + 2.0),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.set_max_width(rem_w);
                    add_contents(ui);
                },
            );
        },
    );
}

fn stat_row(ui: &mut Ui, label: &str, value: &str) {
    prop_row(ui, label, |ui| {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let resp = ui.add(
                egui::Label::new(
                    RichText::new(value)
                        .monospace()
                        .size(11.0)
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

fn slider_f32_row(
    ui: &mut Ui,
    label: &str,
    val: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    decimals: usize,
    suffix: &str,
) {
    prop_row(ui, label, |ui| {
        let avail = ui.available_width().max(60.0);
        let val_w = VAL_W.min((avail - 24.0).max(38.0));
        let slider_w = (avail - val_w - 4.0).max(20.0);
        ui.spacing_mut().slider_width = slider_w;
        ui.add(egui::Slider::new(val, range.clone()).show_value(false));
        ui.add_sized(
            [val_w, ROW_H],
            egui::DragValue::new(val)
                .range(range)
                .fixed_decimals(decimals)
                .suffix(suffix),
        );
    });
}

#[derive(Clone, Copy)]
struct SeriesStyle {
    color: Color32,
    dashed: bool,
    abs_y: bool,
}

fn paint_series(
    painter: &egui::Painter,
    rect: Rect,
    xs: &[f32],
    ys: &[f32],
    bounds: (f32, f32),
    style: SeriesStyle,
) {
    let (x_max, y_max) = bounds;
    let n = xs.len().min(ys.len());
    if n < 2 || x_max <= 0.0 || y_max <= 0.0 {
        return;
    }
    let stride = (n as f32 / rect.width().max(1.0)).ceil().max(1.0) as usize;
    let map = |i: usize| {
        let yv = if style.abs_y { ys[i].abs() } else { ys[i] };
        Pos2::new(
            rect.left() + rect.width() * (xs[i] / x_max).clamp(0.0, 1.0),
            rect.bottom() - 2.0 - (rect.height() - 4.0) * (yv / y_max).clamp(0.0, 1.0),
        )
    };
    let stroke = Stroke::new(if style.dashed { 1.0 } else { 1.35 }, style.color);
    let mut prev = map(0);
    let mut seg_i = 0usize;
    let mut i = stride;
    while i < n {
        let cur = map(i);
        if !style.dashed || seg_i.is_multiple_of(2) {
            painter.line_segment([prev, cur], stroke);
        }
        prev = cur;
        seg_i += 1;
        i += stride;
    }
    let last = map(n - 1);
    if !style.dashed || seg_i.is_multiple_of(2) {
        painter.line_segment([prev, last], stroke);
    }
}

fn paint_loop_curve(
    painter: &egui::Painter,
    rect: Rect,
    pts: &[LoopPoint],
    x_max: f32,
    y_max: f32,
    color: Color32,
) {
    if pts.len() < 2 || x_max <= 0.0 || y_max <= 0.0 {
        return;
    }
    let stride = (pts.len() as f32 / rect.width().max(1.0)).ceil().max(1.0) as usize;
    let to_screen = |p: &LoopPoint| {
        Pos2::new(
            rect.left() + rect.width() * (p.rail_t / x_max).clamp(0.0, 1.0),
            rect.bottom() - 1.0 - (rect.height() - 2.0) * (p.rt_ms / y_max).clamp(0.0, 1.0),
        )
    };
    let stroke = Stroke::new(1.25, color);
    let mut prev = to_screen(&pts[0]);
    for p in pts.iter().step_by(stride).skip(1) {
        let cur = to_screen(p);
        painter.line_segment([prev, cur], stroke);
        prev = cur;
    }
}
