//! Native `egui` dock UI panels for **Flight Lab** and **Flight Player** (`nv-demo`).
//!
//! **What.** Renders [`ui_flight_lab`] (Flight Authoring with live R0+R1 Auto Plan summary &
//! actions, Manual Keyframes, live `camera-guide-rs` Camera Dynamics sliders & curve painter,
//! one unified Contact Sheet, and Flight Stats) and [`ui_flight_player`] (Playback Deck with Now
//! Playing card, phosphor transport controls, interactive shot/cut/chase scrubber, Cam Smooth &
//! Time Scale, Playlist Queue, and Repo/User/Curated Flight Library Browser).
//! **Why.** Uses `crate::ui`'s canonical layout primitives (`collapsing_section`, `prop_row`,
//! `prop_slider_f32`, `prop_toggle`, `stat_row`, `equal_grid`, `allocate_clamped_ui`,
//! `active_accent_tint`, `hdr_amber_tint`, `ROW_H`, `ANIM_BTN_W`) as the single source of truth so
//! every label column, slider track, numeric input, button grid, section accent header, and theme
//! color matches `Render & Floor`, `Camera & Stats`, and `Fractal` to the pixel across all 5
//! themes, while flowing all lists naturally inside the dock tab's outer `dock_scroll` without
//! nested scroll jails or text/action overlaps.
//! **How.** Both entry points take [`FlightUiArgs`] and emit typed [`FlightCommand`] actions via
//! `args.on_command`, keeping UI rendering decoupled from the OptiX render loop and `main.rs`.
//! `ui_lab_auto_plan` shows concise `Plan` & `Gates` summary rows (with a hover breakdown of
//! `plan.report.gate_failures` on warnings) while `ui_lab_contact_sheet` renders the single
//! authoritative shot/cut/segment list; Playlist Queue and Flight Library rows use font-measured
//! columns (`allocate_clamped_ui`) and strip redundant trailing duration tags (`" (24s)"`) so
//! titles never collide with right-aligned durations or action buttons.
//! **Uses.** `camera`, `egui`, `egui_phosphor`, `flight_navigator`, `crate::ui`, `crate::flight`,
//! `crate::flight_stats`, `crate::csg_scene_host`, `crate::is_counters`, `crate::renderer`.
//! **Used by.** `crate::ui` (`DockTab::FlightLab` and `DockTab::FlightPlayer`).

use camera::CameraPose;
use egui::{Align, Color32, FontId, Layout, Pos2, Rect, RichText, Sense, Stroke, Ui};
use egui_phosphor::regular as ic;
use flight_navigator::camera_guide::RATIO_TOL;
use flight_navigator::fnav_core::{
    LibrarySource, Move, PlaylistTransition, ProfileKind, RepeatMode, Segment, Track,
};
use flight_navigator::fnav_plan::{HandoverKind, Mode, RailPlan};

use crate::csg_scene_host::CsgStats;
use crate::flight::convert::limit_term_label;
use crate::flight::lab::{DynamicsCurvePlot, LAB_DURATION_MAX_S, LAB_DURATION_MIN_S, LabMode};
use crate::flight::player::{FlightRigCameraMode, PlayerState, TrackOrigin};
use crate::flight::{FlightCommand, FlightWorkspace};
use crate::flight_stats::{FlightStats, HIST_BINS, LoopPoint};
use crate::is_counters::{IsCounts, IsGroup};
use crate::renderer::{TIME_SCALE_MAX, TIME_SCALE_MIN};
use crate::ui::{
    ANIM_BTN_W, ROW_H, active_accent_tint, allocate_clamped_ui, anim_toggle_btn_disabled,
    collapsing_section, equal_grid, fmt_into, hdr_amber_tint, prop_row, prop_slider_f32,
    prop_toggle, stat_row,
};

/// Height of the `camera-guide-rs` speed/cap/bank dynamics graph.
const DYNAMICS_GRAPH_H: f32 = 76.0;
/// Height of the Flight stats RT curve graph.
const STATS_GRAPH_H: f32 = 58.0;
/// Height of the interactive Winamp-style timeline progress bar.
const PROGRESS_BAR_H: f32 = 22.0;
/// Standard horizontal spacing between controls inside Flight Lab / Player rows.
const GAP_SM: f32 = 4.0;
/// Standard button height matching `crate::ui::equal_grid`.
const BTN_H: f32 = ROW_H + 2.0;
/// Standard height of single-line list rows (keyframes, playlist queue, library browser).
const LIST_ROW_H: f32 = ROW_H + 4.0;
/// Font size for compact monospace metadata columns (durations, indices, playlist counts).
const MONO_META_FONT: f32 = 10.0;
/// Font size for Contact Sheet shot rows.
const SHOT_FONT: f32 = 10.5;
/// Font size for primary list item titles.
const TITLE_FONT: f32 = 11.0;
/// Font size for compact Phosphor action icons inside list rows.
const ICON_BTN_FONT: f32 = 11.0;

/// Shared arguments passed by `crate::ui` into [`ui_flight_lab`] and [`ui_flight_player`].
pub struct FlightUiArgs<'a> {
    /// Single source of truth holding both [`crate::flight::player::FlightPlayer`] and
    /// [`crate::flight::lab::FlightLabState`].
    pub workspace: &'a mut FlightWorkspace,
    /// Accumulated per-frame flight performance statistics.
    pub flight_stats: &'a mut FlightStats,
    /// Latest CSG scene instance/LOD/path telemetry.
    pub csg_stats: &'a CsgStats,
    /// True when a CSG world scene (`csg_mode != 0`) is active; Auto-Plan requires CSG geometry.
    pub is_csg_scene: bool,
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

/// Renders the **Flight Lab** dock panel (Flight Authoring, live `camera-guide-rs` Camera
/// Dynamics & curve plot, Contact Sheet, and Flight Stats).
pub fn ui_flight_lab(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    collapsing_section(
        ui,
        "sec_flab_author",
        ic::PAPER_PLANE_TILT,
        "Flight Authoring",
        |ui| {
            let mode = args.workspace.lab.mode;
            equal_grid(ui, 2, 2, |ui, idx, w| match idx {
                0 => {
                    let active = mode == LabMode::AutoPlan;
                    let mut btn =
                        egui::Button::new(format!("{}  Auto Plan", ic::SPARKLE)).truncate();
                    if active {
                        btn = btn
                            .fill(active_accent_tint(ui))
                            .stroke(Stroke::new(1.0, ui.visuals().selection.stroke.color));
                    }
                    if ui
                        .add_sized([w, ROW_H + 2.0], btn)
                        .on_hover_text(
                            "Automatic cinematic fly-through planner backed by CSG R0 voxel volume + R1 exact raycast.",
                        )
                        .clicked()
                    {
                        args.workspace.lab.mode = LabMode::AutoPlan;
                    }
                }
                _ => {
                    let active = mode == LabMode::ManualKeyframes;
                    let mut btn = egui::Button::new(format!("{}  Manual", ic::PATH)).truncate();
                    if active {
                        btn = btn
                            .fill(active_accent_tint(ui))
                            .stroke(Stroke::new(1.0, ui.visuals().selection.stroke.color));
                    }
                    if ui
                        .add_sized([w, ROW_H + 2.0], btn)
                        .on_hover_text(
                            "Author camera keyframes, tune dynamics, preview, and save tracks.",
                        )
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
        },
    );

    ui_lab_dynamics_section(ui, args);
    ui_lab_contact_sheet(ui, args);
    ui_lab_flight_stats(ui, args);

    status_footer(ui, &args.workspace.lab.status_msg);
}

/// Renders the live **Auto Plan** subsection (parameters, async generation trigger, live stage
/// progress spinner, Preview/Save actions, and compact `Plan` & `Gates` summary rows with a hover
/// breakdown of `plan.report.gate_failures` on warnings; full shot rows live once in
/// [`ui_lab_contact_sheet`]).
fn ui_lab_auto_plan(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let stage = args.workspace.lab.auto_plan_stage;
    let busy = stage.is_busy();
    let can_generate = args.is_csg_scene && !busy;

    {
        let lab = &mut args.workspace.lab;
        prop_slider_f32(
            ui,
            "Duration",
            &mut lab.duration_s,
            LAB_DURATION_MIN_S..=LAB_DURATION_MAX_S,
            1,
            " s",
        );
        aligned_prop_row(ui, "Seed", |ui, ctrl_w| {
            let prev_style = ui.style().override_text_style.clone();
            let prev_dv = ui.style().drag_value_text_style.clone();
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
            ui.style_mut().drag_value_text_style = egui::TextStyle::Monospace;
            ui.add_sized(
                [ctrl_w, ROW_H],
                egui::DragValue::new(&mut lab.seed).speed(1.0),
            );
            ui.style_mut().override_text_style = prev_style;
            ui.style_mut().drag_value_text_style = prev_dv;
        });
        aligned_prop_row(ui, "Plan Mode", |ui, ctrl_w| {
            let cur_lbl = match lab.plan_mode {
                Mode::Deterministic => "Deterministic",
                Mode::Creative => "Creative",
            };
            egui::ComboBox::from_id_salt("flight_lab_plan_mode")
                .width(ctrl_w)
                .truncate()
                .selected_text(cur_lbl)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut lab.plan_mode, Mode::Deterministic, "Deterministic");
                    ui.selectable_value(&mut lab.plan_mode, Mode::Creative, "Creative");
                });
        });
        prop_slider_f32(ui, "Cut Density", &mut lab.cut_density, 0.0..=2.0, 2, "x");
        aligned_prop_row(ui, "Track Name", |ui, ctrl_w| {
            ui.add_sized(
                [ctrl_w, ROW_H],
                egui::TextEdit::singleline(&mut lab.track_name),
            );
        });
    }

    ui.add_space(2.0);
    let gen_w = ui.available_width();
    let disabled_tip = if busy {
        stage.label()
    } else {
        "Switch to a CSG world scene (e.g. Gundo, Marina) to run Auto-Plan."
    };
    if sized_btn(
        ui,
        can_generate,
        gen_w,
        format!("{}  Generate Flight", ic::SPARKLE),
    )
    .on_hover_text("Plan a collision-free cinematic tour against the active CSG scene.")
    .on_disabled_hover_text(disabled_tip)
    .clicked()
    {
        (args.on_command)(FlightCommand::LabGenerateAutoPlan);
    }

    if busy {
        prop_row(ui, "Progress", |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                ui.spinner();
                ui.add(
                    egui::Label::new(
                        RichText::new(stage.label())
                            .size(TITLE_FONT)
                            .color(ui.visuals().selection.stroke.color),
                    )
                    .truncate(),
                );
            });
        });
    }

    let has_plan = args.workspace.lab.preview_plan.is_some();
    if !has_plan {
        return;
    }

    let save_name = args.workspace.lab.track_name.clone();
    ui.add_space(2.0);
    equal_grid(ui, 3, 3, |ui, idx, w| match idx {
        0 => {
            if sized_btn(ui, !busy, w, format!("{}  Preview", ic::PLAY))
                .on_hover_text("Load this auto-planned flight into Flight Player and play it.")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabPreviewTrack);
            }
        }
        1 => {
            if sized_btn(ui, !busy, w, format!("{}  Save User", ic::FLOPPY_DISK))
                .on_hover_text("Atomically save auto-planned track to ~/.nv-rs/flights/")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabSaveTrack {
                    name: save_name.clone(),
                    to_repo: false,
                });
            }
        }
        _ => {
            if sized_btn(ui, !busy, w, format!("{}  Save Repo", ic::FOLDER_OPEN))
                .on_hover_text("Atomically save auto-planned track to assets/flights/")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabSaveTrack {
                    name: save_name.clone(),
                    to_repo: true,
                });
            }
        }
    });

    if let Some(plan) = args.workspace.lab.preview_plan.as_ref() {
        ui.add_space(2.0);
        stat_row(
            ui,
            "Plan",
            fmt_into(
                args.label_buf,
                format_args!(
                    "{} shots | {} cuts | {:.1}s",
                    plan.shots.len(),
                    plan.report.cut_count,
                    plan.report.duration_s
                ),
            ),
        );
        let gates_ok = plan.report.all_gates_passed;
        let gate_icon = if gates_ok {
            ic::CHECK_CIRCLE
        } else {
            ic::WARNING
        };
        let gate_lbl = if gates_ok { "PASS" } else { "WARN" };
        let gate_txt = fmt_into(
            args.label_buf,
            format_args!(
                "{gate_icon} {gate_lbl} | clr {:.1}m | ret {:.0}%",
                plan.report.min_clearance_m,
                plan.report.retrace_ratio * 100.0
            ),
        );
        prop_row(ui, "Gates", |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let col = if gates_ok {
                    ui.visuals().text_color()
                } else {
                    ui.visuals().warn_fg_color
                };
                let resp = ui.add(
                    egui::Label::new(RichText::new(gate_txt).size(11.5).color(col)).truncate(),
                );
                if !plan.report.gate_failures.is_empty() {
                    resp.on_hover_ui(|ui| {
                        ui.label(
                            RichText::new(format!(
                                "{}  Gate warnings ({}):",
                                ic::WARNING,
                                plan.report.gate_failures.len()
                            ))
                            .strong()
                            .color(ui.visuals().warn_fg_color),
                        );
                        for reason in &plan.report.gate_failures {
                            ui.label(format!("- {reason}"));
                        }
                    });
                } else if resp.hovered() {
                    resp.on_hover_text(gate_txt);
                }
            });
        });
    }
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
                "{kf_count} KF ({:.1}, {:.1}, {:.1})",
                args.cur_pose.eye.x, args.cur_pose.eye.y, args.cur_pose.eye.z
            ),
        ),
    );

    equal_grid(ui, 3, 3, |ui, idx, w| match idx {
        0 => {
            if sized_btn(ui, true, w, format!("{}  + KF", ic::PLUS))
                .on_hover_text("Capture the current viewport camera pose as a keyframe.")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabAddKeyframe);
            }
        }
        1 => {
            if sized_btn(ui, kf_count > 0, w, format!("{}  Pop", ic::MINUS))
                .on_hover_text("Remove the last captured keyframe.")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabPopKeyframe);
            }
        }
        _ => {
            if sized_btn(ui, kf_count > 0, w, format!("{}  Clear", ic::TRASH))
                .on_hover_text("Clear all manual keyframes.")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabClearKeyframes);
            }
        }
    });

    prop_slider_f32(
        ui,
        "Duration",
        &mut args.workspace.lab.duration_s,
        2.0..=LAB_DURATION_MAX_S,
        1,
        " s",
    );
    prop_toggle(
        ui,
        "Loop",
        &mut args.workspace.lab.manual_looping,
        "Closed C2 Loop",
    );
    aligned_prop_row(ui, "Track Name", |ui, ctrl_w| {
        ui.add_sized(
            [ctrl_w, ROW_H],
            egui::TextEdit::singleline(&mut args.workspace.lab.track_name),
        );
    });

    ui.add_space(2.0);
    equal_grid(ui, 2, 2, |ui, idx, w| match idx {
        0 => {
            if sized_btn(ui, kf_count >= 2, w, format!("{}  Build", ic::PATH))
                .on_hover_text("Compile manual keyframes through camera-guide-rs into a Track.")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabBuildManualTrack);
            }
        }
        _ => {
            if sized_btn(ui, has_preview, w, format!("{}  Preview", ic::PLAY))
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
            if sized_btn(
                ui,
                has_preview,
                w,
                format!("{}  Save User", ic::FLOPPY_DISK),
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
            if sized_btn(
                ui,
                has_preview,
                w,
                format!("{}  Save Repo", ic::FOLDER_OPEN),
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
        ui.add_space(2.0);
        let btn_w = icon_btn_w(ui);
        for (i, kf) in args.workspace.lab.manual_keyframes.iter().enumerate() {
            let row_w = ui.available_width().max(80.0);
            let left_w = (row_w - btn_w - GAP_SM).max(40.0);
            allocate_clamped_ui(
                ui,
                egui::vec2(row_w, LIST_ROW_H),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = GAP_SM;
                    allocate_clamped_ui(
                        ui,
                        egui::vec2(left_w, LIST_ROW_H),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            let txt = fmt_into(
                                args.label_buf,
                                format_args!(
                                    "#{:02} ({:.1}, {:.1}, {:.1}) {:.0}mm",
                                    i + 1,
                                    kf.pos[0],
                                    kf.pos[1],
                                    kf.pos[2],
                                    kf.focal_mm
                                ),
                            );
                            ui.add(
                                egui::Label::new(RichText::new(txt).monospace().size(SHOT_FONT))
                                    .truncate(),
                            );
                        },
                    );
                    if compact_icon_btn(
                        ui,
                        true,
                        [btn_w, BTN_H],
                        ic::EYE,
                        "Jump viewport camera to this keyframe.",
                    )
                    .clicked()
                    {
                        (args.on_seek_pose)(kf.to_pose(0.05, 5000.0));
                    }
                },
            );
        }
    }
}

/// Renders the **Camera Dynamics** (`camera-guide-rs`) section with live limit controls & curve plot.
fn ui_lab_dynamics_section(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let has_active = args.workspace.player.active_track().is_some()
        || args.workspace.lab.preview_track.is_some();

    collapsing_section(
        ui,
        "sec_flab_dynamics",
        ic::SLIDERS_HORIZONTAL,
        "Camera Dynamics",
        |ui| {
            let lim = &mut args.workspace.lab.limits;
            prop_slider_f32(ui, "Max Bank", &mut lim.max_bank_deg, 0.0..=60.0, 1, " deg");
            prop_slider_f32(ui, "Bank Gain", &mut lim.bank_k, 0.0..=1.0, 2, "");
            prop_slider_f32(
                ui,
                "Bank Rate",
                &mut lim.bank_rate_deg_s,
                10.0..=180.0,
                0,
                " d/s",
            );
            prop_slider_f32(
                ui,
                "Long Accel",
                &mut lim.long_accel,
                1.0..=12.0,
                1,
                " m/s2",
            );
            prop_slider_f32(ui, "Jerk", &mut lim.jerk, 1.0..=20.0, 1, " m/s3");

            ui.add_space(2.0);
            let apply_w = ui.available_width();
            if sized_btn(
                ui,
                has_active,
                apply_w,
                format!("{}  Apply to Active Track", ic::LIGHTNING),
            )
            .on_hover_text(
                "Re-solve the active track through camera-guide-rs with these limits and plot cap/v/bank.",
            )
            .clicked()
            {
                (args.on_command)(FlightCommand::LabApplyLimitsToActive);
            }

            if let Some(plot) = args.workspace.lab.dynamics_plot.as_ref() {
                ui.add_space(2.0);
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

/// Paints `cap(s)` (dashed/muted), `v(s)` (accent green), and `bank(s)` (amber) plus `Ratio` readout.
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

    stat_row(
        ui,
        "Ratio",
        fmt_into(
            buf,
            format_args!(
                "{} {:.2} | {:.1} m/s | {:.0} deg",
                if ok { ic::CHECK_CIRCLE } else { ic::WARNING },
                worst,
                peak_v,
                peak_bank
            ),
        ),
    );
    stat_row(ui, "Limit", limit_term_label(dominant_term));

    let w = ui.available_width().max(80.0);
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(w, DYNAMICS_GRAPH_H), Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 2.0, ui.visuals().extreme_bg_color);
    painter.rect_stroke(
        rect,
        2.0,
        ui.visuals().widgets.noninteractive.bg_stroke,
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
    let cap_col = ui.visuals().weak_text_color();
    let v_col = if dark {
        Color32::from_rgb(108, 198, 64)
    } else {
        Color32::from_rgb(42, 138, 36)
    };
    let bank_col = if dark {
        Color32::from_rgb(232, 166, 54)
    } else {
        Color32::from_rgb(184, 114, 16)
    };

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
        "cap (dash) | v (green) | bank (amber)",
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

/// Authoritative track and optional matching [`RailPlan`] displayed in [`ui_lab_contact_sheet`].
struct ContactSheetSource<'a> {
    track: &'a Track,
    plan: Option<&'a RailPlan>,
    is_player_track: bool,
}

/// Resolves the single authoritative track (and optional matching [`RailPlan`] metadata) shown by
/// [`ui_lab_contact_sheet`] without cloning the track.
///
/// Priority:
/// 1. When the active Flight Player slot matches `lab.preview_track` by canonical hash, use the
///    active slot (`is_player_track = true`) with either the slot's plan or `lab.preview_plan`.
/// 2. When Flight Player is actively playing/transitioning a track and no un-previewed auto-plan
///    is waiting in Flight Lab, show the active player slot.
/// 3. When Flight Lab has a preview track (fresh auto-plan or manual build), show it immediately
///    with its matching `preview_plan` so clicking Generate/Build updates Contact Sheet right away.
/// 4. Otherwise fall back to the active Flight Player slot (e.g. default curated rail when Idle).
fn resolve_contact_sheet_source<'a>(
    workspace: &'a FlightWorkspace,
) -> Option<ContactSheetSource<'a>> {
    let matching_plan = |track: &Track, cand: Option<&'a RailPlan>| {
        cand.filter(|p| p.track.canonical_hash() == track.canonical_hash())
    };
    let active_slot = workspace.player.active_slot();
    let preview_track = workspace.lab.preview_track.as_ref();
    let preview_plan = workspace.lab.preview_plan.as_ref();

    if let (Some(slot), Some(preview)) = (active_slot, preview_track)
        && slot.track.canonical_hash() == preview.canonical_hash()
    {
        let plan = matching_plan(&slot.track, slot.plan.as_ref())
            .or_else(|| matching_plan(&slot.track, preview_plan));
        return Some(ContactSheetSource {
            track: &slot.track,
            plan,
            is_player_track: true,
        });
    }

    if workspace.player.is_active()
        && preview_plan.is_none()
        && let Some(slot) = active_slot
    {
        return Some(ContactSheetSource {
            track: &slot.track,
            plan: matching_plan(&slot.track, slot.plan.as_ref()),
            is_player_track: true,
        });
    }

    if let Some(preview) = preview_track {
        return Some(ContactSheetSource {
            track: preview,
            plan: matching_plan(preview, preview_plan),
            is_player_track: false,
        });
    }

    active_slot.map(|slot| ContactSheetSource {
        track: &slot.track,
        plan: matching_plan(&slot.track, slot.plan.as_ref()),
        is_player_track: true,
    })
}

/// Renders the single authoritative **Contact Sheet** listing shots, cuts, and segments of the
/// preview or active track, flowing naturally inside the dock tab's outer `dock_scroll`.
fn ui_lab_contact_sheet(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let workspace = &*args.workspace;
    let source = resolve_contact_sheet_source(workspace);

    collapsing_section(
        ui,
        "sec_flab_contact",
        ic::FILM_STRIP,
        "Contact Sheet",
        |ui| {
            let Some(src) = source else {
                ui.label(
                    RichText::new("No active or preview track.")
                        .size(TITLE_FONT)
                        .weak(),
                );
                return;
            };
            let track = src.track;
            let plan = src.plan;
            let is_player_track = src.is_player_track;

            let seg_count = track.segments.len();
            let shot_count = track
                .segments
                .iter()
                .filter(|seg| matches!(seg, Segment::Shot(_)))
                .count();
            let cuts = track.cut_times().len();
            let chases = track.chase_spans().len();
            let dur_s = track.duration_s();
            let cut_unit = if cuts == 1 { "cut" } else { "cuts" };
            let (count, unit) = if shot_count > 0 {
                (shot_count, if chases > 0 { "sh" } else { "shots" })
            } else {
                (seg_count, "seg")
            };
            if chases > 0 {
                stat_row(
                    ui,
                    "Summary",
                    fmt_into(
                        args.label_buf,
                        format_args!(
                            "{count} {unit} | {cuts} {cut_unit} | {} {chases} | {dur_s:.1}s",
                            ic::LIGHTNING
                        ),
                    ),
                );
            } else {
                stat_row(
                    ui,
                    "Summary",
                    fmt_into(
                        args.label_buf,
                        format_args!("{count} {unit} | {cuts} {cut_unit} | {dur_s:.1}s"),
                    ),
                );
            }

            let mut t_acc = 0.0_f64;
            let mut shot_ordinal = 0usize;
            let mut glide_ordinal = 0usize;
            let w = ui.available_width().max(80.0);

            for seg in &track.segments {
                match seg {
                    Segment::Cut(c) => {
                        let txt = fmt_into(
                            args.label_buf,
                            format_args!("{}  {} @ {:.1}s", ic::SCISSORS, c.kind.label(), t_acc),
                        );
                        ui.add(
                            egui::Label::new(
                                RichText::new(txt)
                                    .size(SHOT_FONT)
                                    .color(ui.visuals().warn_fg_color),
                            )
                            .truncate(),
                        );
                    }
                    Segment::Shot(s) => {
                        let planned = plan.and_then(|p| p.shots.get(shot_ordinal));
                        let handover = plan.and_then(|p| {
                            planned.and_then(|ps| {
                                p.handovers.iter().find(|h| h.from_shot == ps.index)
                            })
                        });
                        shot_ordinal += 1;
                        let dur = s.duration_s.max(0.0);
                        let focal = s.entry().state.lens.focal_mm;
                        let is_chase = matches!(s.speed_profile.kind, ProfileKind::Chase)
                            || planned
                                .is_some_and(|ps| matches!(ps.profile_kind, ProfileKind::Chase));

                        let txt = if let Some(ps) = planned {
                            let handover_tag = match handover.map(|h| &h.kind) {
                                Some(HandoverKind::Cut(_)) => "Cut",
                                Some(HandoverKind::Continuous { .. }) => "C2",
                                None => "End",
                            };
                            if is_chase {
                                fmt_into(
                                    args.label_buf,
                                    format_args!(
                                        "#{:02} {:<10} {:>4.1}s {:>2.0}mm {:>4.2} {:>3} {}",
                                        shot_ordinal,
                                        ps.move_kind.label(),
                                        dur,
                                        focal,
                                        ps.quality,
                                        handover_tag,
                                        ic::LIGHTNING
                                    ),
                                )
                            } else {
                                fmt_into(
                                    args.label_buf,
                                    format_args!(
                                        "#{:02} {:<10} {:>4.1}s {:>2.0}mm {:>4.2} {:>3}",
                                        shot_ordinal,
                                        ps.move_kind.label(),
                                        dur,
                                        focal,
                                        ps.quality,
                                        handover_tag
                                    ),
                                )
                            }
                        } else if is_chase {
                            fmt_into(
                                args.label_buf,
                                format_args!(
                                    "S{:02} {:<10} {:>4.1}s {:>3.0}mm {}",
                                    shot_ordinal,
                                    s.kind().label(),
                                    dur,
                                    focal,
                                    ic::LIGHTNING
                                ),
                            )
                        } else {
                            fmt_into(
                                args.label_buf,
                                format_args!(
                                    "S{:02} {:<10} {:>4.1}s {:>3.0}mm",
                                    shot_ordinal,
                                    s.kind().label(),
                                    dur,
                                    focal
                                ),
                            )
                        };

                        let resp = ui.add_sized(
                            [w, BTN_H],
                            egui::Button::new(RichText::new(txt).monospace().size(SHOT_FONT))
                                .truncate(),
                        );
                        if resp.clicked() {
                            if is_player_track {
                                (args.on_command)(FlightCommand::Seek(t_acc));
                            } else {
                                let pose = track.eval(t_acc).state.to_camera_pose(0.05, 5000.0);
                                (args.on_seek_pose)(pose);
                            }
                        }
                        if let Some(ps) = planned {
                            let handover_kind = handover.map(|h| &h.kind);
                            resp.on_hover_ui(|ui| match handover_kind {
                                Some(HandoverKind::Cut(c)) => {
                                    ui.label(format!(
                                        "Shot #{}: {} ({:.0}mm, {:.1}s, score {:.2}) -> Cut ({}) [click to jump]",
                                        shot_ordinal,
                                        ps.move_kind.label(),
                                        ps.focal_mm,
                                        dur,
                                        ps.quality,
                                        c.kind.label()
                                    ));
                                }
                                Some(HandoverKind::Continuous {
                                    connector,
                                    duration_s,
                                }) => {
                                    ui.label(format!(
                                        "Shot #{}: {} ({:.0}mm, {:.1}s, score {:.2}) -> C2 {} ({:.1}s) [click to jump]",
                                        shot_ordinal,
                                        ps.move_kind.label(),
                                        ps.focal_mm,
                                        dur,
                                        ps.quality,
                                        connector,
                                        duration_s
                                    ));
                                }
                                None => {
                                    ui.label(format!(
                                        "Shot #{}: {} ({:.0}mm, {:.1}s, score {:.2}) [click to jump]",
                                        shot_ordinal,
                                        ps.move_kind.label(),
                                        ps.focal_mm,
                                        dur,
                                        ps.quality
                                    ));
                                }
                            });
                        } else {
                            resp.on_hover_text(format!(
                                "Shot #{}: {} ({:.0}mm, {:.1}s) [click to jump]",
                                shot_ordinal,
                                s.kind().label(),
                                focal,
                                dur
                            ));
                        }
                        t_acc += dur;
                    }
                    Segment::Transition(tr) => {
                        let dur = tr.duration_s.max(0.0);
                        let is_plan_connector = plan.is_some() && shot_count > 0;
                        if !is_plan_connector && (seg_count <= 24 || dur >= 1.0) {
                            glide_ordinal += 1;
                            let focal = tr.entry().state.lens.focal_mm;
                            let txt = fmt_into(
                                args.label_buf,
                                format_args!(
                                    "T{:02} {:<10} {:>4.1}s {:>3.0}mm",
                                    glide_ordinal, "Glide", dur, focal
                                ),
                            );
                            if ui
                                .add_sized(
                                    [w, BTN_H],
                                    egui::Button::new(
                                        RichText::new(txt).monospace().size(SHOT_FONT),
                                    )
                                    .truncate(),
                                )
                                .on_hover_text(format!(
                                    "Segment #{glide_ordinal}: Glide ({focal:.0}mm, {dur:.1}s) [click to jump]"
                                ))
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
                        }
                        t_acc += dur;
                    }
                }
            }
        },
    );
}

/// Renders the collapsible **Flight Stats** section inside Flight Lab.
fn ui_lab_flight_stats(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    collapsing_section(ui, "sec_flab_stats", ic::CHART_LINE, "Flight Stats", |ui| {
        prop_toggle(ui, "IS Counters", args.is_counters, "Count IS hits (debug)");
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
                "IS Rays",
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
                "Samples",
                fmt_into(
                    args.label_buf,
                    format_args!("{}f ({:.1}..{:.1} ms)", s.frames, s.min, s.max),
                ),
            );
            stat_row(
                ui,
                "Median",
                fmt_into(args.label_buf, format_args!("{:.1} ms", s.median)),
            );
            stat_row(
                ui,
                "p95",
                fmt_into(args.label_buf, format_args!("{:.1} ms", s.p95)),
            );
        }
        stat_row(
            ui,
            "Visible",
            fmt_into(
                args.label_buf,
                format_args!(
                    "{} | {} GAS | {} crowd",
                    load.visible, args.csg_stats.compounds, args.crowd_actor_count
                ),
            ),
        );
        stat_row(
            ui,
            "LOD 0/1/2",
            fmt_into(
                args.label_buf,
                format_args!("{} / {} / {}", load.lod[0], load.lod[1], load.lod[2]),
            ),
        );
        stat_row(
            ui,
            "Culled",
            fmt_into(
                args.label_buf,
                format_args!("{} (bias {:.2})", load.culled, args.csg_lod_bias),
            ),
        );
        stat_row(
            ui,
            "Camera",
            fmt_into(
                args.label_buf,
                format_args!("{:.1} m/s | {:.0} d/s", stats.speed_mps, stats.turn_dps),
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
        painter.rect_stroke(
            rect,
            2.0,
            ui.visuals().widgets.noninteractive.bg_stroke,
            egui::StrokeKind::Inside,
        );
        let rt_col = if ui.visuals().dark_mode {
            Color32::from_rgb(118, 185, 0)
        } else {
            Color32::from_rgb(56, 132, 24)
        };
        paint_loop_curve(
            &painter,
            rect,
            stats.prev_loop(),
            dur,
            rt_max,
            ui.visuals().weak_text_color(),
        );
        paint_loop_curve(&painter, rect, stats.cur_loop(), dur, rt_max, rt_col);
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
            hpainter.rect_stroke(
                hrect,
                2.0,
                ui.visuals().widgets.noninteractive.bg_stroke,
                egui::StrokeKind::Inside,
            );
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
                hpainter.rect_filled(bar, 1.0, rt_col.gamma_multiply(0.8));
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
                        [w, ROW_H + 2.0],
                        egui::Button::new(RichText::new(txt).monospace().size(10.0)).truncate(),
                    )
                    .on_hover_text("Click to seek player to this slow frame.")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::Seek(f64::from(spot.rail_t)));
                }
            }
        }

        ui.add_space(2.0);
        let export_w = ui.available_width();
        if sized_btn(
            ui,
            stats.frames() > 0,
            export_w,
            format!("{}  Export CSV", ic::FLOPPY_DISK),
        )
        .on_hover_text("Export accumulated flight telemetry frames to perf CSV.")
        .clicked()
        {
            (args.on_export_flight_csv)();
        }
    });
}

/// Renders the **Flight Player** dock panel (Playback Deck, Playlist Queue, and Flight Library).
pub fn ui_flight_player(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    collapsing_section(
        ui,
        "sec_fplay_deck",
        ic::CASSETTE_TAPE,
        "Playback Deck",
        |ui| {
            ui_player_now_playing(ui, args);
            ui.add_space(2.0);
            ui_player_transport_row(ui, args);
            ui.add_space(2.0);
            ui_player_progress_bar(ui, args);
            ui.add_space(2.0);
            ui_player_drone_rig(ui, args);
            ui.add_space(2.0);

            stat_row(ui, "Scene", args.active_scene_label);
            prop_slider_f32(ui, "Cam Smooth", args.cam_smooth_tau, 0.0..=1.0, 2, " s");
            prop_slider_f32(
                ui,
                "Time Scale",
                args.time_scale,
                TIME_SCALE_MIN..=TIME_SCALE_MAX,
                2,
                "x",
            );
            if args.workspace.player.rig_mode.uses_rail_rig() {
                let prev = args.workspace.player.rig_tilt;
                let mut tilt = prev;
                prop_slider_f32(ui, "Max Tilt", &mut tilt.max_tilt_deg, 0.0..=35.0, 1, " deg");
                prop_slider_f32(ui, "Tilt Tau", &mut tilt.tau_s, 0.05..=1.5, 2, " s");
                if tilt != prev {
                    (args.on_command)(FlightCommand::SetRigTilt(tilt));
                }
            }
        },
    );

    ui_player_playlist_queue(ui, args);
    ui_player_library_browser(ui, args);

    status_footer(ui, &args.workspace.player.status_msg);
}

/// Renders the **Drone Rig (`csg_scene::RailRig`)** look-mode selector (`Cinematic` / `Along` /
/// `Track` / `Free`), landmark target picker, live gimbal/tilt telemetry, and `Eject` action.
fn ui_player_drone_rig(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let rig_mode = args.workspace.player.rig_mode;
    let rig_active = args.workspace.player.is_rig_active();
    let accent = active_accent_tint(ui);
    let accent_stroke = Stroke::new(1.0, ui.visuals().selection.stroke.color);

    let modes = [
        (
            FlightRigCameraMode::Cinematic,
            ic::FILM_STRIP,
            "Direct fnav-core Track framing (preserves authored shot aims).",
        ),
        (
            FlightRigCameraMode::AlongPath,
            ic::AIRPLANE_TILT,
            "csg_scene::RailRig AlongPath: DroneTilt + rate-limited forward gimbal.",
        ),
        (
            FlightRigCameraMode::Track,
            ic::CROSSHAIR,
            "csg_scene::RailRig Track: lock rate-limited gimbal onto a scene landmark.",
        ),
        (
            FlightRigCameraMode::Free,
            ic::EYE,
            "csg_scene::RailRig Free: drag viewport to steer gimbal without pausing flight (Esc to eject).",
        ),
    ];

    equal_grid(ui, 5, 5, |ui, idx, w| {
        let prev_pad = ui.spacing().button_padding;
        ui.spacing_mut().button_padding = egui::vec2(2.0, 0.0);
        if let Some(&(mode, icon, tip)) = modes.get(idx) {
            let mut btn = egui::Button::new(format!("{icon} {}", mode.label())).truncate();
            if rig_mode == mode {
                btn = btn.fill(accent).stroke(accent_stroke);
            }
            if ui.add_sized([w, BTN_H], btn).on_hover_text(tip).clicked() {
                (args.on_command)(FlightCommand::SetRigMode(mode));
            }
        } else {
            let eject_col = ui.visuals().warn_fg_color;
            let mut btn = egui::Button::new(format!("{} Eject", ic::EJECT)).truncate();
            if rig_active {
                btn = btn
                    .fill(eject_col.gamma_multiply(0.28))
                    .stroke(Stroke::new(1.0, eject_col));
            }
            if ui
                .add_enabled_ui(args.workspace.player.is_active(), |ui| {
                    ui.add_sized([w, BTN_H], btn)
                })
                .inner
                .on_hover_text("Eject from RailRig into free camera at current gimbal pose (Esc).")
                .clicked()
            {
                (args.on_command)(FlightCommand::RigEject);
            }
        }
        ui.spacing_mut().button_padding = prev_pad;
    });

    if rig_mode == FlightRigCameraMode::Track {
        let cur_name = args
            .workspace
            .player
            .rig_track_landmark
            .as_deref()
            .unwrap_or("None")
            .to_owned();
        let landmarks: Vec<String> = args
            .workspace
            .player
            .scene_landmarks
            .iter()
            .map(|l| l.name.clone())
            .collect();
        let mut chosen: Option<String> = None;
        prop_row(ui, "Target", |ui| {
            let w = ui.available_width().max(80.0);
            egui::ComboBox::from_id_salt("fplay_rig_landmark")
                .width(w)
                .selected_text(&cur_name)
                .show_ui(ui, |ui| {
                    if landmarks.is_empty() {
                        ui.label(RichText::new("No landmarks in scene").weak());
                    } else {
                        for name in &landmarks {
                            if ui.selectable_label(cur_name == *name, name).clicked() {
                                chosen = Some(name.clone());
                            }
                        }
                    }
                });
        });
        if let Some(name) = chosen {
            (args.on_command)(FlightCommand::SetRigTrackLandmark(name));
        }
    }

    if rig_mode.uses_rail_rig() {
        let rt = args.workspace.player.last_rig;
        let status_txt = match rt.track_status {
            csg_scene::TrackStatus::Locked => "Locked",
            csg_scene::TrackStatus::OutOfRange { .. } => "Clamped",
            csg_scene::TrackStatus::TargetLost => "Lost",
        };
        let rig_line = if rig_mode == FlightRigCameraMode::Track {
            fmt_into(
                args.label_buf,
                format_args!(
                    "Tilt {:.1} deg | Y {:.0} P {:.0} | {} ({:.1} deg)",
                    rt.tilt_deg,
                    rt.gimbal_deg[0],
                    rt.gimbal_deg[1],
                    status_txt,
                    rt.track_error_deg
                ),
            )
        } else {
            fmt_into(
                args.label_buf,
                format_args!(
                    "Tilt {:.1} deg | Gimbal Y {:.0} P {:.0}",
                    rt.tilt_deg, rt.gimbal_deg[0], rt.gimbal_deg[1]
                ),
            )
        };
        stat_row(ui, "Drone Rig", rig_line);
    }
}

/// Renders the theme-derived Now Playing card inside the Playback Deck section.
fn ui_player_now_playing(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let player = &args.workspace.player;
    let dark = ui.visuals().dark_mode;
    let card_bg = ui.visuals().faint_bg_color;
    let card_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

    let (state_lbl, state_col) = match &player.state {
        PlayerState::Playing { .. } => (
            "PLAYING",
            if dark {
                Color32::from_rgb(92, 198, 108)
            } else {
                Color32::from_rgb(32, 138, 56)
            },
        ),
        PlayerState::Paused { .. } => ("PAUSED", ui.visuals().warn_fg_color),
        PlayerState::Transition { .. } => ("TRANSITION", ui.visuals().selection.stroke.color),
        PlayerState::Loading { .. } => ("LOADING", ui.visuals().warn_fg_color),
        PlayerState::Idle => ("IDLE", ui.visuals().weak_text_color()),
    };

    let origin_lbl = player.active_slot().map_or("Empty", |s| s.origin.label());
    let cur_t = player.time_s;
    let dur = player.duration_s();

    egui::Frame::NONE
        .fill(card_bg)
        .stroke(card_stroke)
        .corner_radius(3.0)
        .inner_margin(egui::Margin::symmetric(6, 5))
        .show(ui, |ui| {
            let row_w = ui.available_width().max(80.0);
            let origin_tag = fmt_into(args.label_buf, format_args!("[{origin_lbl}]")).to_owned();
            let badge_font = FontId::proportional(9.5);
            let origin_w = ui
                .painter()
                .layout_no_wrap(origin_tag.clone(), badge_font.clone(), Color32::WHITE)
                .size()
                .x
                .ceil();
            let state_w = ui
                .painter()
                .layout_no_wrap(state_lbl.to_owned(), badge_font, Color32::WHITE)
                .size()
                .x
                .ceil();
            let badge_w = (origin_w + GAP_SM + state_w + 4.0).min(row_w * 0.60);
            let title_w = (row_w - badge_w - GAP_SM).max(36.0);
            allocate_clamped_ui(
                ui,
                egui::vec2(row_w, BTN_H),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = GAP_SM;
                    allocate_clamped_ui(
                        ui,
                        egui::vec2(title_w, BTN_H),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.add(
                                egui::Label::new(
                                    RichText::new(player.active_title()).size(12.0).strong(),
                                )
                                .truncate(),
                            );
                        },
                    );
                    allocate_clamped_ui(
                        ui,
                        egui::vec2(badge_w, BTN_H),
                        Layout::right_to_left(Align::Center),
                        |ui| {
                            ui.spacing_mut().item_spacing.x = GAP_SM;
                            ui.label(
                                RichText::new(state_lbl)
                                    .size(9.5)
                                    .strong()
                                    .color(state_col),
                            );
                            ui.label(RichText::new(origin_tag).size(9.5).weak());
                        },
                    );
                },
            );

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = GAP_SM;
                let time_txt = fmt_into(
                    args.label_buf,
                    format_args!("{} / {}", format_mmss(cur_t), format_mmss(dur)),
                );
                ui.label(RichText::new(time_txt).monospace().size(TITLE_FONT).strong());
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = GAP_SM;
                    if player.last_active_chase {
                        ui.label(
                            RichText::new(format!("{} CHASE", ic::LIGHTNING))
                                .size(9.5)
                                .strong()
                                .color(ui.visuals().warn_fg_color),
                        );
                    }
                    let kin_txt = fmt_into(
                        args.label_buf,
                        format_args!(
                            "{:.1} m/s | {:.0} deg",
                            player.last_speed_mps,
                            player.last_bank_rad.to_degrees(),
                        ),
                    );
                    ui.add(
                        egui::Label::new(
                            RichText::new(kin_txt)
                                .monospace()
                                .size(MONO_META_FONT)
                                .weak(),
                        )
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
    let accent_stroke = Stroke::new(1.0, ui.visuals().selection.stroke.color);
    let rec_col = ui.visuals().error_fg_color;
    let rec_fill = rec_col.gamma_multiply(0.32);

    equal_grid(ui, 7, 7, |ui, idx, w| {
        let prev_pad = ui.spacing().button_padding;
        ui.spacing_mut().button_padding = egui::vec2(2.0, 0.0);
        match idx {
            0 => {
                if ui
                    .add_enabled_ui(has_queue, |ui| {
                        ui.add_sized([w, BTN_H], egui::Button::new(ic::SKIP_BACK))
                    })
                    .inner
                    .on_hover_text("Previous track (or rewind if > 2 s).")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::Prev);
                }
            }
            1 => {
                let icon = if is_playing { ic::PAUSE } else { ic::PLAY };
                let mut btn = egui::Button::new(icon);
                if is_playing {
                    btn = btn.fill(accent).stroke(accent_stroke);
                }
                if ui
                    .add_enabled_ui(is_playing || can_play, |ui| ui.add_sized([w, BTN_H], btn))
                    .inner
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
                    .add_enabled_ui(is_playing || is_paused, |ui| {
                        ui.add_sized([w, BTN_H], egui::Button::new(ic::STOP))
                    })
                    .inner
                    .on_hover_text("Stop playback and rewind to 0:00.")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::Stop);
                }
            }
            3 => {
                if ui
                    .add_enabled_ui(has_queue, |ui| {
                        ui.add_sized([w, BTN_H], egui::Button::new(ic::SKIP_FORWARD))
                    })
                    .inner
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
                let mut btn = egui::Button::new(icon);
                if repeat != RepeatMode::Off {
                    btn = btn.fill(accent).stroke(accent_stroke);
                }
                if ui.add_sized([w, BTN_H], btn).on_hover_text(tip).clicked() {
                    (args.on_command)(FlightCommand::SetRepeat(next_mode));
                }
            }
            5 => {
                let mut btn = egui::Button::new(ic::SHUFFLE);
                if shuffle {
                    btn = btn.fill(accent).stroke(accent_stroke);
                }
                if ui
                    .add_sized([w, BTN_H], btn)
                    .on_hover_text("Toggle deterministic seeded playlist shuffle.")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::SetShuffle(!shuffle));
                }
            }
            _ => {
                let mut btn = egui::Button::new(ic::RECORD);
                if rec_active {
                    btn = btn.fill(rec_fill).stroke(Stroke::new(1.0, rec_col));
                }
                if ui
                    .add_enabled_ui(can_play || rec_active, |ui| ui.add_sized([w, BTN_H], btn))
                    .inner
                    .on_hover_text(if rec_active {
                        "Stop MP4 recording without interrupting active playback."
                    } else {
                        "Record from the active track through playlist end to MP4 (auto-stops at end)."
                    })
                    .clicked()
                {
                    (args.on_command)(FlightCommand::ToggleRecordPlaylist);
                }
            }
        }
        ui.spacing_mut().button_padding = prev_pad;
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
        painter.rect_filled(fill_rect, 3.0, active_accent_tint(ui));

        if let Some(track) = player.active_track() {
            let chase_fill = ui.visuals().warn_fg_color.gamma_multiply(0.45);
            for (ts, te) in track.chase_spans() {
                let x0 = rect.left() + rect.width() * (ts / dur).clamp(0.0, 1.0) as f32;
                let x1 = rect.left() + rect.width() * (te / dur).clamp(0.0, 1.0) as f32;
                let span_rect = Rect::from_min_max(
                    Pos2::new(x0, rect.top() + 2.0),
                    Pos2::new(x1.max(x0 + 2.0), rect.bottom() - 2.0),
                );
                painter.rect_filled(span_rect, 1.5, chase_fill);
            }

            let shot_stroke = Stroke::new(1.0, ui.visuals().weak_text_color().gamma_multiply(0.6));
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

            let cut_stroke = Stroke::new(1.75, ui.visuals().error_fg_color);
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
            Stroke::new(2.0, ui.visuals().strong_text_color()),
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
        ui.visuals().widgets.noninteractive.bg_stroke,
        egui::StrokeKind::Inside,
    );
}

/// Renders the **Playlist Queue** with reorder arrows, transition selector, `hold_s`, delete, and save.
fn ui_player_playlist_queue(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    let q_len = args.workspace.player.queue.len();
    let title = format!("Playlist Queue ({q_len})");

    collapsing_section(ui, "sec_fplay_queue", ic::PLAYLIST, &title, |ui| {
        if q_len == 0 {
            ui.label(
                RichText::new("Queue is empty. Play or enqueue a track from the Library below.")
                    .size(TITLE_FONT)
                    .weak(),
            );
        } else {
            let active_idx = args.workspace.player.active_idx;
            stat_row(
                ui,
                "Active",
                fmt_into(
                    args.label_buf,
                    format_args!("{:02} / {:02}", (active_idx + 1).min(q_len), q_len),
                ),
            );
            let accent = active_accent_tint(ui);
            let mut deferred_cmds: Vec<FlightCommand> = Vec::new();
            let btn_w = icon_btn_w(ui);
            let hold_lbl_w = ui
                .painter()
                .layout_no_wrap("Hold".to_owned(), FontId::proportional(MONO_META_FONT), Color32::WHITE)
                .size()
                .x
                .ceil()
                + 2.0;
            let hold_w = (mono_chars_w(ui, SHOT_FONT, 5) + 12.0).max(48.0);

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
                    .inner_margin(egui::Margin::symmetric(4, 2))
                    .show(ui, |ui| {
                        let row_w = ui.available_width().max(80.0);
                        let dur_s = slot.track.duration_s();
                        let dur_w = mono_chars_w(ui, MONO_META_FONT, duration_char_count(dur_s)) + 4.0;
                        let right_w = dur_w + GAP_SM + btn_w * 3.0 + GAP_SM * 2.0;
                        let left_w = (row_w - right_w - GAP_SM).max(36.0);
                        let display_title = strip_trailing_duration_tag(&slot.title, dur_s);

                        allocate_clamped_ui(
                            ui,
                            egui::vec2(row_w, LIST_ROW_H),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                ui.spacing_mut().item_spacing.x = GAP_SM;
                                allocate_clamped_ui(
                                    ui,
                                    egui::vec2(left_w, LIST_ROW_H),
                                    Layout::left_to_right(Align::Center),
                                    |ui| {
                                        ui.spacing_mut().item_spacing.x = GAP_SM;
                                        let idx_txt = fmt_into(
                                            args.label_buf,
                                            format_args!("{:02}.", idx + 1),
                                        );
                                        ui.label(
                                            RichText::new(idx_txt)
                                                .monospace()
                                                .size(MONO_META_FONT)
                                                .weak(),
                                        );
                                        if slot.stale {
                                            stale_badge(
                                                ui,
                                                "Scene geometry hash changed since this track was saved.",
                                            );
                                        }
                                        let title_resp = ui
                                            .add(
                                                egui::Label::new(
                                                    RichText::new(display_title).size(TITLE_FONT),
                                                )
                                                .truncate()
                                                .sense(Sense::click()),
                                            )
                                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                                            .on_hover_text(if slot.description.is_empty() {
                                                slot.title.clone()
                                            } else {
                                                format!("{}\n{}", slot.title, slot.description)
                                            });
                                        if title_resp.clicked() {
                                            deferred_cmds
                                                .push(FlightCommand::SelectPlaylistEntry(idx));
                                        }
                                    },
                                );
                                allocate_clamped_ui(
                                    ui,
                                    egui::vec2(dur_w, LIST_ROW_H),
                                    Layout::right_to_left(Align::Center),
                                    |ui| {
                                        let dur_txt = fmt_into(
                                            args.label_buf,
                                            format_args!("{dur_s:.0}s"),
                                        );
                                        ui.label(
                                            RichText::new(dur_txt)
                                                .monospace()
                                                .size(MONO_META_FONT)
                                                .weak(),
                                        );
                                    },
                                );
                                if compact_icon_btn(
                                    ui,
                                    idx > 0,
                                    [btn_w, BTN_H],
                                    ic::ARROW_UP,
                                    "Move up",
                                )
                                .clicked()
                                {
                                    deferred_cmds.push(FlightCommand::ReorderPlaylist {
                                        from: idx,
                                        to: idx - 1,
                                    });
                                }
                                if compact_icon_btn(
                                    ui,
                                    idx + 1 < q_len,
                                    [btn_w, BTN_H],
                                    ic::ARROW_DOWN,
                                    "Move down",
                                )
                                .clicked()
                                {
                                    deferred_cmds.push(FlightCommand::ReorderPlaylist {
                                        from: idx,
                                        to: idx + 1,
                                    });
                                }
                                if compact_icon_btn(
                                    ui,
                                    true,
                                    [btn_w, BTN_H],
                                    ic::TRASH,
                                    "Remove from playlist",
                                )
                                .clicked()
                                {
                                    deferred_cmds.push(FlightCommand::RemovePlaylistEntry(idx));
                                }
                            },
                        );

                        let combo_w = (row_w - hold_w - hold_lbl_w - GAP_SM * 2.0).max(52.0);
                        allocate_clamped_ui(
                            ui,
                            egui::vec2(row_w, LIST_ROW_H),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                ui.spacing_mut().item_spacing.x = GAP_SM;
                                let cur_tr_lbl = transition_label(slot.transition_in);
                                let mut new_tr = slot.transition_in;
                                egui::ComboBox::from_id_salt(("pl_tr", idx))
                                    .width(combo_w)
                                    .truncate()
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

                                allocate_clamped_ui(
                                    ui,
                                    egui::vec2(hold_lbl_w, LIST_ROW_H),
                                    Layout::right_to_left(Align::Center),
                                    |ui| {
                                        ui.label(
                                            RichText::new("Hold").size(MONO_META_FONT).weak(),
                                        );
                                    },
                                );
                                let mut hold_val = slot.hold_s;
                                let changed = allocate_clamped_ui(
                                    ui,
                                    egui::vec2(hold_w, ROW_H),
                                    Layout::centered_and_justified(egui::Direction::LeftToRight),
                                    |ui| {
                                        let font = FontId::monospace(SHOT_FONT);
                                        let style = ui.style_mut();
                                        style.drag_value_text_style = egui::TextStyle::Monospace;
                                        style.override_text_style = Some(egui::TextStyle::Monospace);
                                        style.override_font_id = Some(font.clone());
                                        style.text_styles.insert(egui::TextStyle::Monospace, font);
                                        style.spacing.button_padding.x = 2.0;
                                        style.spacing.interact_size.x = hold_w;
                                        ui.add(
                                            egui::DragValue::new(&mut hold_val)
                                                .range(0.0..=30.0)
                                                .speed(0.1)
                                                .suffix("s"),
                                        )
                                        .changed()
                                    },
                                );
                                if changed {
                                    deferred_cmds.push(FlightCommand::SetEntryHold {
                                        idx,
                                        hold_s: hold_val,
                                    });
                                }
                            },
                        );
                    });
            }

            for cmd in deferred_cmds {
                (args.on_command)(cmd);
            }
        }

        ui.add_space(2.0);
        aligned_prop_row(ui, "Playlist", |ui, ctrl_w| {
            ui.add_sized(
                [ctrl_w, ROW_H],
                egui::TextEdit::singleline(&mut args.workspace.player.playlist_name),
            );
        });
        ui.add_space(2.0);
        let pl_name = args.workspace.player.playlist_name.clone();
        equal_grid(ui, 2, 2, |ui, idx, w| match idx {
            0 => {
                if sized_btn(ui, q_len > 0, w, format!("{}  Save User", ic::FLOPPY_DISK))
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
                if sized_btn(ui, q_len > 0, w, format!("{}  Save Repo", ic::FOLDER_OPEN))
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

/// Renders the grouped **Flight Library** (`Scene Curated`, `Repo`, `User`, and `Saved Playlists`).
fn ui_player_library_browser(ui: &mut Ui, args: &mut FlightUiArgs<'_>) {
    collapsing_section(
        ui,
        "sec_fplay_library",
        ic::QUEUE,
        "Flight Library",
        |ui| {
            let err_count = args.workspace.player.library.load_errors.len();
            if err_count == 0 {
                let w = ui.available_width();
                if sized_btn(ui, true, w, format!("{}  Rescan Library", ic::ARROWS_CLOCKWISE))
                    .on_hover_text("Rescan assets/flights and ~/.nv-rs/flights on disk.")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::RescanLibrary);
                }
            } else {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = GAP_SM;
                    if sized_btn(ui, true, 86.0, format!("{}  Rescan", ic::ARROWS_CLOCKWISE))
                        .on_hover_text("Rescan assets/flights and ~/.nv-rs/flights on disk.")
                        .clicked()
                    {
                        (args.on_command)(FlightCommand::RescanLibrary);
                    }
                    egui::Frame::NONE
                        .fill(hdr_amber_tint(ui))
                        .corner_radius(3.0)
                        .inner_margin(egui::Margin::symmetric(5, 1))
                        .show(ui, |ui| {
                            ui.add(
                                egui::Label::new(
                                    RichText::new(fmt_into(
                                        args.label_buf,
                                        format_args!("{} {err_count} errors", ic::WARNING),
                                    ))
                                    .size(MONO_META_FONT)
                                    .color(ui.visuals().text_color()),
                                )
                                .truncate(),
                            );
                        });
                });
            }

            ui.add_space(2.0);
            let mut deferred: Vec<FlightCommand> = Vec::new();
            let btn_w = icon_btn_w(ui);

            ui.label(RichText::new("Scene Curated").size(SHOT_FONT).strong().weak());
            if args.workspace.player.curated.is_empty() {
                ui.label(
                    RichText::new("  No curated rails for active scene.")
                        .size(SHOT_FONT)
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
                        btn_w,
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
                    .size(SHOT_FONT)
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
                        btn_w,
                    ) {
                        Some(LibraryRowAction::Play) => {
                            deferred.push(FlightCommand::PlayLibraryTrack(t.path.clone()));
                        }
                        Some(LibraryRowAction::Enqueue) => {
                            deferred.push(FlightCommand::EnqueueLibraryTrack(t.path.clone()));
                        }
                        None => {}
                    }
                }
            }
            if !repo_any {
                ui.label(RichText::new("  No repo tracks found.").size(SHOT_FONT).weak());
            }

            ui.add_space(3.0);
            ui.label(
                RichText::new("User (~/.nv-rs/flights)")
                    .size(SHOT_FONT)
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
                        btn_w,
                    ) {
                        Some(LibraryRowAction::Play) => {
                            deferred.push(FlightCommand::PlayLibraryTrack(t.path.clone()));
                        }
                        Some(LibraryRowAction::Enqueue) => {
                            deferred.push(FlightCommand::EnqueueLibraryTrack(t.path.clone()));
                        }
                        None => {}
                    }
                }
            }
            if !user_any {
                ui.label(
                    RichText::new("  No user tracks saved yet.")
                        .size(SHOT_FONT)
                        .weak(),
                );
            }

            if !args.workspace.player.library.playlists.is_empty() {
                ui.add_space(3.0);
                ui.label(RichText::new("Saved Playlists").size(SHOT_FONT).strong().weak());
                for pl in &args.workspace.player.library.playlists {
                    let row_w = ui.available_width().max(80.0);
                    let src = match pl.source {
                        LibrarySource::Repo => "Repo",
                        LibrarySource::User => "User",
                    };
                    let meta = fmt_into(
                        args.label_buf,
                        format_args!("[{src}] {}t", pl.entry_count),
                    );
                    let meta_w = mono_chars_w(ui, MONO_META_FONT, meta.len().max(9)) + 4.0;
                    let right_w = meta_w + GAP_SM + btn_w;
                    let left_w = (row_w - right_w - GAP_SM).max(36.0);
                    allocate_clamped_ui(
                        ui,
                        egui::vec2(row_w, LIST_ROW_H),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.spacing_mut().item_spacing.x = GAP_SM;
                            allocate_clamped_ui(
                                ui,
                                egui::vec2(left_w, LIST_ROW_H),
                                Layout::left_to_right(Align::Center),
                                |ui| {
                                    ui.add(
                                        egui::Label::new(
                                            RichText::new(&pl.name).size(TITLE_FONT),
                                        )
                                        .truncate(),
                                    )
                                    .on_hover_text(pl.path.display().to_string());
                                },
                            );
                            allocate_clamped_ui(
                                ui,
                                egui::vec2(meta_w, LIST_ROW_H),
                                Layout::right_to_left(Align::Center),
                                |ui| {
                                    ui.label(
                                        RichText::new(meta)
                                            .monospace()
                                            .size(MONO_META_FONT)
                                            .weak(),
                                    );
                                },
                            );
                            if compact_icon_btn(
                                ui,
                                true,
                                [btn_w, BTN_H],
                                ic::FOLDER_OPEN,
                                "Load playlist into queue",
                            )
                            .clicked()
                            {
                                deferred.push(FlightCommand::LoadPlaylistFile(pl.path.clone()));
                            }
                        },
                    );
                }
            }

            for cmd in deferred {
                (args.on_command)(cmd);
            }
        },
    );
}

/// Action emitted by a single row in the Flight Library Browser.
enum LibraryRowAction {
    Play,
    Enqueue,
}

/// Renders one fixed-height single-line row in the Flight Library with Play and Enqueue icon buttons.
#[expect(clippy::too_many_arguments, reason = "internal stateless row helper keeps call sites concise")]
fn library_item_row(
    ui: &mut Ui,
    buf: &mut String,
    name: &str,
    tooltip: &str,
    badge: &str,
    duration_s: f64,
    stale: bool,
    btn_w: f32,
) -> Option<LibraryRowAction> {
    let mut action = None;
    let row_w = ui.available_width().max(80.0);
    let dur_w = mono_chars_w(ui, MONO_META_FONT, duration_char_count(duration_s)) + 4.0;
    let right_w = dur_w + GAP_SM + btn_w * 2.0 + GAP_SM;
    let left_w = (row_w - right_w - GAP_SM).max(36.0);
    let display_name = strip_trailing_duration_tag(name, duration_s);

    allocate_clamped_ui(
        ui,
        egui::vec2(row_w, LIST_ROW_H),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = GAP_SM;
            allocate_clamped_ui(
                ui,
                egui::vec2(left_w, LIST_ROW_H),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = GAP_SM;
                    if stale {
                        stale_badge(
                            ui,
                            "Stale: scene content hash differs from when this track was created.",
                        );
                    }
                    ui.add(egui::Label::new(RichText::new(display_name).size(TITLE_FONT)).truncate())
                        .on_hover_text(format!("[{badge}] {name}\n{tooltip}"));
                },
            );
            allocate_clamped_ui(
                ui,
                egui::vec2(dur_w, LIST_ROW_H),
                Layout::right_to_left(Align::Center),
                |ui| {
                    let meta = fmt_into(buf, format_args!("{duration_s:.0}s"));
                    ui.label(RichText::new(meta).monospace().size(MONO_META_FONT).weak());
                },
            );
            if compact_icon_btn(ui, true, [btn_w, BTN_H], ic::PLAY, "Play now").clicked() {
                action = Some(LibraryRowAction::Play);
            }
            if compact_icon_btn(
                ui,
                true,
                [btn_w, BTN_H],
                ic::PLUS,
                "Append to playlist queue",
            )
            .clicked()
            {
                action = Some(LibraryRowAction::Enqueue);
            }
        },
    );
    action
}

// ---------------------------------------------------------------------------
// Flight-specific micro-helpers & canvas painters (layout primitives live in `crate::ui`)
// ---------------------------------------------------------------------------

/// Renders a single non-slider control inside a canonical [`prop_row`] using the exact same
/// control width as [`prop_slider_f32`] / [`prop_toggle`] plus the trailing disabled animation
/// slot so right edges align to the pixel across every row.
fn aligned_prop_row(ui: &mut Ui, label: &str, add_ctrl: impl FnOnce(&mut Ui, f32)) {
    prop_row(ui, label, |ui| {
        let ctrl_w = (ui.available_width() - ANIM_BTN_W - GAP_SM).max(40.0);
        ui.spacing_mut().item_spacing.x = GAP_SM;
        allocate_clamped_ui(
            ui,
            egui::vec2(ctrl_w, ROW_H),
            Layout::left_to_right(Align::Center),
            |ui| {
                add_ctrl(ui, ctrl_w);
            },
        );
        anim_toggle_btn_disabled(ui);
    });
}

/// Renders an enabled/disabled text button locked to exact `[w, BTN_H]` dimensions.
fn sized_btn(
    ui: &mut Ui,
    enabled: bool,
    w: f32,
    text: impl Into<egui::WidgetText>,
) -> egui::Response {
    ui.add_enabled_ui(enabled, |ui| {
        ui.add_sized([w, BTN_H], egui::Button::new(text).truncate())
    })
    .inner
}

/// Computes a uniform icon-button width wide enough for an [`ICON_BTN_FONT`] Phosphor glyph plus
/// compact horizontal padding and 1 px stroke at the active UI scale so icon columns never
/// overflow into adjacent titles or durations.
fn icon_btn_w(ui: &Ui) -> f32 {
    let glyph = ui
        .ctx()
        .fonts_mut(|f| f.glyph_width(&FontId::proportional(ICON_BTN_FONT), 'W'));
    (glyph + 8.0).ceil().max(20.0)
}

/// Renders a fixed-size Phosphor icon action button with tight padding so its outer rect matches
/// `size` exactly without expanding into sibling columns.
fn compact_icon_btn(
    ui: &mut Ui,
    enabled: bool,
    size: [f32; 2],
    icon: &str,
    tooltip: &str,
) -> egui::Response {
    let prev_pad = ui.spacing().button_padding;
    ui.spacing_mut().button_padding = egui::vec2(2.0, 0.0);
    let resp = ui
        .add_enabled_ui(enabled, |ui| {
            ui.add_sized(
                size,
                egui::Button::new(RichText::new(icon).size(ICON_BTN_FONT)),
            )
        })
        .inner;
    ui.spacing_mut().button_padding = prev_pad;
    resp.on_hover_text(tooltip)
}

/// Measures the exact width of `chars` monospace digits at font `size` (zero heap allocation).
fn mono_chars_w(ui: &Ui, size: f32, chars: usize) -> f32 {
    let font = FontId::monospace(size);
    let cw = ui.ctx().fonts_mut(|f| f.glyph_width(&font, '0'));
    (cw * chars as f32).ceil()
}

/// Character count needed for a rounded `{dur:.0}s` duration label (at least 4 chars for `"999s"`
/// columnar alignment across rows).
fn duration_char_count(duration_s: f64) -> usize {
    let secs = if duration_s.is_finite() && duration_s > 0.0 {
        duration_s.round() as u64
    } else {
        0
    };
    let digits = if secs < 10 {
        1
    } else if secs < 100 {
        2
    } else if secs < 1000 {
        3
    } else if secs < 10_000 {
        4
    } else {
        5
    };
    (digits + 1).max(4)
}

/// Strips a redundant trailing `" (24s)"` / `" (45.0 s)"` duration suffix from `title` when it
/// matches `duration_s` within `1.0 s`, preventing rows with a dedicated right-aligned duration
/// column from displaying `"Street Navigation (24s) 24s"`.
fn strip_trailing_duration_tag(title: &str, duration_s: f64) -> &str {
    let trimmed = title.trim_end();
    if !duration_s.is_finite() || duration_s <= 0.0 {
        return trimmed;
    }
    let Some(no_close) = trimmed.strip_suffix(')') else {
        return trimmed;
    };
    let Some(open_idx) = no_close.rfind('(') else {
        return trimmed;
    };
    let inner = no_close[open_idx + 1..].trim();
    let Some(num_str) = inner
        .strip_suffix('s')
        .or_else(|| inner.strip_suffix('S'))
        .map(str::trim)
    else {
        return trimmed;
    };
    let Ok(parsed) = num_str.parse::<f64>() else {
        return trimmed;
    };
    if (parsed - duration_s).abs() <= 1.0 {
        let prefix = trimmed[..open_idx].trim_end();
        if !prefix.is_empty() {
            return prefix;
        }
    }
    trimmed
}

/// Compact theme-aware amber stale-track warning pill using [`hdr_amber_tint`].
fn stale_badge(ui: &mut Ui, tooltip: &str) {
    let resp = egui::Frame::NONE
        .fill(hdr_amber_tint(ui))
        .corner_radius(2.0)
        .inner_margin(egui::Margin::symmetric(2, 0))
        .show(ui, |ui| {
            ui.label(
                RichText::new(ic::WARNING)
                    .size(MONO_META_FONT)
                    .color(ui.visuals().warn_fg_color),
            )
        })
        .inner;
    resp.on_hover_text(tooltip);
}

/// Compact single-line status footer shown below sections when a panel has a status message.
fn status_footer(ui: &mut Ui, msg: &str) {
    if msg.is_empty() {
        return;
    }
    ui.add_space(2.0);
    ui.separator();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = GAP_SM;
        ui.label(RichText::new(ic::INFO).size(TITLE_FONT).weak());
        ui.add(egui::Label::new(RichText::new(msg).size(SHOT_FONT).weak()).truncate())
            .on_hover_text(msg);
    });
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
    if n < 2 || !x_max.is_finite() || !y_max.is_finite() || x_max <= 0.0 || y_max <= 0.0 {
        return;
    }
    let stride = (n as f32 / rect.width().max(1.0)).ceil().max(1.0) as usize;
    let map = |i: usize| {
        let xv = if xs[i].is_finite() { xs[i] } else { 0.0 };
        let y_raw = if ys[i].is_finite() { ys[i] } else { 0.0 };
        let yv = if style.abs_y { y_raw.abs() } else { y_raw };
        Pos2::new(
            rect.left() + rect.width() * (xv / x_max).clamp(0.0, 1.0),
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
    if pts.len() < 2 || !x_max.is_finite() || !y_max.is_finite() || x_max <= 0.0 || y_max <= 0.0 {
        return;
    }
    let stride = (pts.len() as f32 / rect.width().max(1.0)).ceil().max(1.0) as usize;
    let to_screen = |p: &LoopPoint| {
        let t = if p.rail_t.is_finite() { p.rail_t } else { 0.0 };
        let ms = if p.rt_ms.is_finite() { p.rt_ms } else { 0.0 };
        Pos2::new(
            rect.left() + rect.width() * (t / x_max).clamp(0.0, 1.0),
            rect.bottom() - 1.0 - (rect.height() - 2.0) * (ms / y_max).clamp(0.0, 1.0),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flight::player::{CuratedTrackEntry, PlaylistSlot};
    use crate::ui::{UiTheme, install_fonts};
    use flight_navigator::fnav_core::{
        Aim, CameraState, Cut, CutEvidence, CutKind, Kinematic, Lens, Transition,
    };
    use glam::Vec3;

    fn sample_track(segment_count: usize, seg_dur_s: f64) -> Track {
        let mut segments = Vec::with_capacity(segment_count);
        for i in 0..segment_count {
            let x0 = (i as f32) * 10.0;
            let k0 = Kinematic {
                state: CameraState::from_look_at(
                    Vec3::new(x0, 5.0, 20.0),
                    Vec3::new(x0 + 5.0, 2.0, 0.0),
                    Vec3::Y,
                    Lens::pinhole(35.0),
                ),
                ..Kinematic::default()
            };
            let k1 = Kinematic {
                state: CameraState::from_look_at(
                    Vec3::new(x0 + 10.0, 6.0, 20.0),
                    Vec3::new(x0 + 15.0, 2.0, 0.0),
                    Vec3::Y,
                    Lens::pinhole(50.0),
                ),
                ..Kinematic::default()
            };
            if i > 0 && i.is_multiple_of(4) {
                segments.push(Segment::Cut(Cut {
                    kind: CutKind::Contrast,
                    score: 0.9,
                    evidence: CutEvidence {
                        metric_a: 45.0,
                        metric_b: 2.0,
                        note: "test cut".to_string(),
                    },
                }));
            }
            segments.push(Segment::Transition(Transition::direct(
                k0,
                k1,
                seg_dur_s,
                Aim::Squad,
            )));
        }
        Track {
            segments,
            looping: false,
        }
    }

    #[test]
    fn strip_trailing_duration_tag_removes_matching_suffix_only() {
        assert_eq!(
            strip_trailing_duration_tag("Street Navigation (24s)", 24.0),
            "Street Navigation"
        );
        assert_eq!(
            strip_trailing_duration_tag("Street Navigation (24s)", 24.4),
            "Street Navigation"
        );
        assert_eq!(
            strip_trailing_duration_tag("El Segundo Orbit (45.0 s)", 45.0),
            "El Segundo Orbit"
        );
        assert_eq!(
            strip_trailing_duration_tag("Street Navigation (24s)", 60.0),
            "Street Navigation (24s)"
        );
        assert_eq!(
            strip_trailing_duration_tag("gundo_auto_42", 45.0),
            "gundo_auto_42"
        );
        assert_eq!(strip_trailing_duration_tag("(24s)", 24.0), "(24s)");
        assert_eq!(
            strip_trailing_duration_tag("Track (v2)", 24.0),
            "Track (v2)"
        );
    }

    #[test]
    fn contact_sheet_expands_naturally_without_scroll_jail_or_overflow() {
        let mut ws = FlightWorkspace::default();
        ws.lab.preview_track = Some(sample_track(14, 3.0));
        let mut stats = FlightStats::new();
        let csg_stats = CsgStats::default();
        let mut is_counters = false;
        let mut time_scale = 1.0_f32;
        let mut cam_smooth_tau = 0.08_f32;
        let mut label_buf = String::with_capacity(96);

        for panel_w in [260.0_f32, 300.0, 340.0] {
            let ctx = egui::Context::default();
            install_fonts(&ctx);
            let _dock = UiTheme::Obsidian.apply(&ctx);
            let mut st = egui::collapsing_header::CollapsingState::load_with_default_open(
                &ctx,
                egui::Id::new("sec_flab_contact"),
                true,
            );
            st.set_open(true);
            st.store(&ctx);
            let mut contact_height = 0.0_f32;
            let mut content_right = 0.0_f32;

            for _ in 0..2 {
                let raw = egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(panel_w, 2000.0))),
                    ..Default::default()
                };
                let out = ctx.run_ui(raw, |ui| {
                    ui.set_max_width(panel_w);
                    ui.set_width(panel_w);
                    let mut on_cmd = |_cmd: FlightCommand| {};
                    let mut on_csv = || {};
                    let mut on_seek = |_pose: CameraPose| {};
                    let mut args = FlightUiArgs {
                        workspace: &mut ws,
                        flight_stats: &mut stats,
                        csg_stats: &csg_stats,
                        is_csg_scene: true,
                        csg_lod_bias: 1.0,
                        crowd_actor_count: 12,
                        rt_ms: 8.5,
                        is_counters: &mut is_counters,
                        last_is_counts: None,
                        active_scene_label: "Gundo",
                        cur_pose: CameraPose::default(),
                        is_recording: false,
                        time_scale: &mut time_scale,
                        cam_smooth_tau: &mut cam_smooth_tau,
                        label_buf: &mut label_buf,
                        on_command: &mut on_cmd,
                        on_export_flight_csv: &mut on_csv,
                        on_seek_pose: &mut on_seek,
                    };
                    let y0 = ui.cursor().top();
                    ui_lab_contact_sheet(ui, &mut args);
                    contact_height = ui.cursor().top() - y0;
                    content_right = ui.min_rect().right();
                });
                out.drop_without_applying_deltas();
            }

            // 14 segments + 3 cut markers + summary header must flow well beyond the old 132 px cap.
            assert!(
                contact_height > 240.0,
                "panel_w={panel_w}: Contact Sheet stayed cramped at {contact_height:.1} px (expected > 240 px)"
            );
            assert!(
                content_right <= panel_w + 1.0,
                "panel_w={panel_w}: Contact Sheet overflowed right edge ({content_right:.1} > {panel_w:.1})"
            );
        }
    }

    #[test]
    fn flight_player_queue_and_library_never_overflow_or_overlap_at_standard_widths() {
        let mut ws = FlightWorkspace::default();
        ws.player.curated.clear();
        ws.player.curated.push(CuratedTrackEntry {
            id: "curated_street".to_string(),
            label: "Street Navigation (24s)".to_string(),
            description: "Street-level tour through Gundo".to_string(),
            scene_id: "gundo".to_string(),
            track: sample_track(4, 6.0),
            duration_s: 24.0,
        });
        ws.player.queue.clear();
        ws.player.queue.push(PlaylistSlot {
            title: "Street Navigation (24s)".to_string(),
            description: "Street-level tour through Gundo".to_string(),
            scene_id: "gundo".to_string(),
            track_id: "curated_street".to_string(),
            file_path: None,
            track: sample_track(4, 6.0),
            plan: None,
            transition_in: PlaylistTransition::Cut,
            hold_s: 0.0,
            stale: false,
            origin: TrackOrigin::Curated,
        });
        ws.player.queue.push(PlaylistSlot {
            title: "Very Long Cinematic Aerial Flythrough Over El Segundo Refinery (90s)".to_string(),
            description: "Extended aerial overview".to_string(),
            scene_id: "gundo".to_string(),
            track_id: "curated_aerial".to_string(),
            file_path: None,
            track: sample_track(5, 18.0),
            plan: None,
            transition_in: PlaylistTransition::FlyThrough,
            hold_s: 1.5,
            stale: true,
            origin: TrackOrigin::RepoLibrary,
        });

        let mut stats = FlightStats::new();
        let csg_stats = CsgStats::default();
        let mut is_counters = false;
        let mut time_scale = 1.0_f32;
        let mut cam_smooth_tau = 0.08_f32;
        let mut label_buf = String::with_capacity(96);

        for panel_w in [240.0_f32, 280.0, 340.0] {
            let ctx = egui::Context::default();
            install_fonts(&ctx);
            let _dock = UiTheme::Obsidian.apply(&ctx);
            let mut content_right = 0.0_f32;

            for _ in 0..2 {
                let raw = egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(panel_w, 2000.0))),
                    ..Default::default()
                };
                let out = ctx.run_ui(raw, |ui| {
                    ui.set_max_width(panel_w);
                    ui.set_width(panel_w);
                    let mut on_cmd = |_cmd: FlightCommand| {};
                    let mut on_csv = || {};
                    let mut on_seek = |_pose: CameraPose| {};
                    let mut args = FlightUiArgs {
                        workspace: &mut ws,
                        flight_stats: &mut stats,
                        csg_stats: &csg_stats,
                        is_csg_scene: true,
                        csg_lod_bias: 1.0,
                        crowd_actor_count: 12,
                        rt_ms: 8.5,
                        is_counters: &mut is_counters,
                        last_is_counts: None,
                        active_scene_label: "Gundo",
                        cur_pose: CameraPose::default(),
                        is_recording: false,
                        time_scale: &mut time_scale,
                        cam_smooth_tau: &mut cam_smooth_tau,
                        label_buf: &mut label_buf,
                        on_command: &mut on_cmd,
                        on_export_flight_csv: &mut on_csv,
                        on_seek_pose: &mut on_seek,
                    };
                    ui_flight_player(ui, &mut args);
                    content_right = ui.min_rect().right();
                });
                out.drop_without_applying_deltas();
            }

            assert!(
                content_right <= panel_w + 1.0,
                "panel_w={panel_w}: Flight Player overflowed right edge ({content_right:.1} > {panel_w:.1})"
            );
        }
    }
}
