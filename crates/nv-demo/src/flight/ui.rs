//! Native `egui` dock UI panels for **Flight Lab** and **Flight Player** (`nv-demo`).
//!
//! **What.** Renders [`ui_flight_lab`] (Flight Authoring with live R0+R1 Auto Plan, contact sheet
//! & actions, Manual Keyframes + live `camera-guide-rs` Camera Dynamics sliders & curve painter +
//! Contact Sheet + Flight Stats) and [`ui_flight_player`] (Playback Deck with Now Playing card,
//! phosphor transport controls, interactive shot/cut/chase scrubber, Cam Smooth & Time Scale +
//! Playlist Queue + Repo/User/Curated Flight Library Browser).
//! **Why.** Uses `crate::ui`'s canonical layout primitives (`collapsing_section`, `prop_row`,
//! `prop_slider_f32`, `prop_toggle`, `stat_row`, `equal_grid`, `active_accent_tint`,
//! `hdr_amber_tint`, `ROW_H`, `ANIM_BTN_W`) as the single source of truth so every label column,
//! slider track, numeric input, button grid, section accent header, and theme color matches
//! `Render & Floor`, `Camera & Stats`, and `Fractal` to the pixel across all 5 themes.
//! **How.** Both entry points take [`FlightUiArgs`] and emit typed [`FlightCommand`] actions via
//! `args.on_command`, keeping UI rendering decoupled from the OptiX render loop and `main.rs`.
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
use flight_navigator::fnav_plan::{HandoverKind, Mode};

use crate::csg_scene_host::CsgStats;
use crate::flight::convert::limit_term_label;
use crate::flight::lab::{DynamicsCurvePlot, LAB_DURATION_MAX_S, LAB_DURATION_MIN_S, LabMode};
use crate::flight::player::{PlayerState, TrackOrigin};
use crate::flight::{FlightCommand, FlightWorkspace};
use crate::flight_stats::{FlightStats, HIST_BINS, LoopPoint};
use crate::is_counters::{IsCounts, IsGroup};
use crate::renderer::{TIME_SCALE_MAX, TIME_SCALE_MIN};
use crate::ui::{
    ANIM_BTN_W, ROW_H, active_accent_tint, anim_toggle_btn_disabled, collapsing_section,
    equal_grid, fmt_into, hdr_amber_tint, prop_row, prop_slider_f32, prop_toggle, stat_row,
};

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
/// progress spinner, and compact contact sheet + Preview/Save controls once a plan is ready).
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
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
            ui.add_sized(
                [ctrl_w, ROW_H],
                egui::DragValue::new(&mut lab.seed).speed(1.0),
            );
            ui.style_mut().override_text_style = prev_style;
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
    let gen_btn = egui::Button::new(format!("{}  Generate Flight", ic::SPARKLE))
        .truncate()
        .min_size(egui::vec2(ui.available_width(), ROW_H + 2.0));
    let disabled_tip = if busy {
        stage.label()
    } else {
        "Switch to a CSG world scene (e.g. Gundo, Marina) to run Auto-Plan."
    };
    if ui
        .add_enabled(can_generate, gen_btn)
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
                            .size(11.0)
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
            if ui
                .add_enabled(
                    !busy,
                    egui::Button::new(format!("{}  Preview", ic::PLAY))
                        .truncate()
                        .min_size(egui::vec2(w, ROW_H + 2.0)),
                )
                .on_hover_text("Load this auto-planned flight into Flight Player and play it.")
                .clicked()
            {
                (args.on_command)(FlightCommand::LabPreviewTrack);
            }
        }
        1 => {
            if ui
                .add_enabled(
                    !busy,
                    egui::Button::new(format!("{}  Save User", ic::FLOPPY_DISK))
                        .truncate()
                        .min_size(egui::vec2(w, ROW_H + 2.0)),
                )
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
            if ui
                .add_enabled(
                    !busy,
                    egui::Button::new(format!("{}  Save Repo", ic::FOLDER_OPEN))
                        .truncate()
                        .min_size(egui::vec2(w, ROW_H + 2.0)),
                )
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
        let gate_icon = if plan.report.all_gates_passed {
            ic::CHECK_CIRCLE
        } else {
            ic::WARNING
        };
        let gate_lbl = if plan.report.all_gates_passed {
            "PASS"
        } else {
            "WARN"
        };
        stat_row(
            ui,
            "Gates",
            fmt_into(
                args.label_buf,
                format_args!(
                    "{gate_icon} {gate_lbl} | clr {:.1}m | ret {:.0}%",
                    plan.report.min_clearance_m,
                    plan.report.retrace_ratio * 100.0
                ),
            ),
        );

        let mut jump_pose: Option<CameraPose> = None;
        egui::ScrollArea::vertical()
            .id_salt("flight_lab_auto_plan_shots")
            .max_height(124.0)
            .show(ui, |ui| {
                let w = ui.available_width().max(80.0);
                for shot in &plan.shots {
                    let handover = plan.handovers.iter().find(|h| h.from_shot == shot.index);
                    let handover_tag = match handover.map(|h| &h.kind) {
                        Some(HandoverKind::Cut(_)) => "Cut",
                        Some(HandoverKind::Continuous { .. }) => "C2",
                        None => "End",
                    };
                    let chase_tag = if matches!(shot.profile_kind, ProfileKind::Chase) {
                        ic::LIGHTNING
                    } else {
                        ""
                    };
                    let txt = fmt_into(
                        args.label_buf,
                        format_args!(
                            "#{:02} {:<12} {:>4.1}s {:>4.2} {:>3} {chase_tag}",
                            shot.index + 1,
                            shot.move_kind.label(),
                            shot.duration_s,
                            shot.quality,
                            handover_tag
                        ),
                    );
                    let resp = ui.add_sized(
                        [w, ROW_H + 2.0],
                        egui::Button::new(RichText::new(txt).monospace().size(10.5)).truncate(),
                    );
                    if resp.clicked() {
                        jump_pose = Some(
                            plan.track
                                .eval(shot.start_s)
                                .state
                                .to_camera_pose(0.05, 5000.0),
                        );
                    }
                    let handover_kind = handover.map(|h| &h.kind);
                    resp.on_hover_ui(|ui| match handover_kind {
                        Some(HandoverKind::Cut(c)) => {
                            ui.label(format!(
                                "Shot #{}: {} ({:.0}mm, {:.1}s, score {:.2}) -> Cut ({})",
                                shot.index + 1,
                                shot.move_kind.label(),
                                shot.focal_mm,
                                shot.duration_s,
                                shot.quality,
                                c.kind.label()
                            ));
                        }
                        Some(HandoverKind::Continuous {
                            connector,
                            duration_s,
                        }) => {
                            ui.label(format!(
                                "Shot #{}: {} ({:.0}mm, {:.1}s, score {:.2}) -> C2 {} ({:.1}s)",
                                shot.index + 1,
                                shot.move_kind.label(),
                                shot.focal_mm,
                                shot.duration_s,
                                shot.quality,
                                connector,
                                duration_s
                            ));
                        }
                        None => {
                            ui.label(format!(
                                "Shot #{}: {} ({:.0}mm, {:.1}s, score {:.2}) [click to inspect]",
                                shot.index + 1,
                                shot.move_kind.label(),
                                shot.focal_mm,
                                shot.duration_s,
                                shot.quality
                            ));
                        }
                    });
                }
            });
        if let Some(pose) = jump_pose {
            (args.on_seek_pose)(pose);
        }
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
            if ui
                .add_sized(
                    [w, ROW_H + 2.0],
                    egui::Button::new(format!("{}  + KF", ic::PLUS)).truncate(),
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
                    egui::Button::new(format!("{}  Pop", ic::MINUS))
                        .truncate()
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
                        .truncate()
                        .min_size(egui::vec2(w, ROW_H + 2.0)),
                )
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
            if ui
                .add_enabled(
                    kf_count >= 2,
                    egui::Button::new(format!("{}  Build", ic::PATH))
                        .truncate()
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
                        .truncate()
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
                        .truncate()
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
                        .truncate()
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
        ui.add_space(2.0);
        egui::ScrollArea::vertical()
            .id_salt("flight_lab_kf_list")
            .max_height(88.0)
            .show(ui, |ui| {
                let row_h = ROW_H + 4.0;
                let btn_sz = ROW_H + 2.0;
                let gap = 4.0;
                for (i, kf) in args.workspace.lab.manual_keyframes.iter().enumerate() {
                    let row_w = ui.available_width().max(80.0);
                    let left_w = (row_w - btn_sz - gap).max(40.0);
                    ui.allocate_ui_with_layout(
                        egui::vec2(row_w, row_h),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.set_max_width(row_w);
                            ui.spacing_mut().item_spacing.x = gap;
                            ui.allocate_ui_with_layout(
                                egui::vec2(left_w, row_h),
                                Layout::left_to_right(Align::Center),
                                |ui| {
                                    ui.set_max_width(left_w);
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
                                        egui::Label::new(RichText::new(txt).monospace().size(10.5))
                                            .truncate(),
                                    );
                                },
                            );
                            if ui
                                .add_sized([btn_sz, btn_sz], egui::Button::new(ic::EYE))
                                .on_hover_text("Jump viewport camera to this keyframe.")
                                .clicked()
                            {
                                (args.on_seek_pose)(kf.to_pose(0.05, 5000.0));
                            }
                        },
                    );
                }
            });
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
            if ui
                .add_enabled(
                    has_active,
                    egui::Button::new(format!("{}  Apply to Active Track", ic::LIGHTNING))
                        .truncate()
                        .min_size(egui::vec2(ui.available_width(), ROW_H + 2.0)),
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
                "{} {:.2} | {:.1} m/s | {:.1} deg",
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

    collapsing_section(
        ui,
        "sec_flab_contact",
        ic::FILM_STRIP,
        "Contact Sheet",
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
                        "{seg_count} seg | {cuts} cuts | {chases} chase | {:.1}s",
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
                                        .color(ui.visuals().warn_fg_color),
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
                                        [w, ROW_H + 2.0],
                                        egui::Button::new(
                                            RichText::new(txt).monospace().size(10.5),
                                        )
                                        .truncate(),
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
                                            [w, ROW_H + 2.0],
                                            egui::Button::new(
                                                RichText::new(txt).monospace().size(10.5),
                                            )
                                            .truncate(),
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
                    "{} inst | {} GAS | {} crowd",
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
        if ui
            .add_enabled(
                stats.frames() > 0,
                egui::Button::new(format!("{}  Export CSV", ic::FLOPPY_DISK))
                    .truncate()
                    .min_size(egui::vec2(ui.available_width(), ROW_H + 2.0)),
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
        },
    );

    ui_player_playlist_queue(ui, args);
    ui_player_library_browser(ui, args);

    status_footer(ui, &args.workspace.player.status_msg);
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
            let badge_w = 88.0_f32.min(row_w * 0.48);
            let title_w = (row_w - badge_w - 4.0).max(40.0);
            ui.allocate_ui_with_layout(
                egui::vec2(row_w, ROW_H + 2.0),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.set_max_width(row_w);
                    ui.spacing_mut().item_spacing.x = 4.0;
                    ui.allocate_ui_with_layout(
                        egui::vec2(title_w, ROW_H + 2.0),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.set_max_width(title_w);
                            ui.add(
                                egui::Label::new(
                                    RichText::new(player.active_title()).size(12.0).strong(),
                                )
                                .truncate(),
                            );
                        },
                    );
                    ui.allocate_ui_with_layout(
                        egui::vec2(badge_w, ROW_H + 2.0),
                        Layout::right_to_left(Align::Center),
                        |ui| {
                            ui.set_max_width(badge_w);
                            ui.spacing_mut().item_spacing.x = 4.0;
                            ui.label(
                                RichText::new(state_lbl)
                                    .size(9.5)
                                    .strong()
                                    .color(state_col),
                            );
                            ui.label(
                                RichText::new(fmt_into(
                                    args.label_buf,
                                    format_args!("[{origin_lbl}]"),
                                ))
                                .size(9.5)
                                .weak(),
                            );
                        },
                    );
                },
            );

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                let time_txt = fmt_into(
                    args.label_buf,
                    format_args!("{} / {}", format_mmss(cur_t), format_mmss(dur)),
                );
                ui.label(RichText::new(time_txt).monospace().size(11.0).strong());
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
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
                            "{:.1} m/s | {:.1} deg",
                            player.last_speed_mps,
                            player.last_bank_rad.to_degrees(),
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
    let accent_stroke = Stroke::new(1.0, ui.visuals().selection.stroke.color);
    let rec_col = ui.visuals().error_fg_color;
    let rec_fill = rec_col.gamma_multiply(0.32);

    equal_grid(ui, 7, 7, |ui, idx, w| {
        let h = ROW_H + 2.0;
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
                    btn = btn.fill(accent).stroke(accent_stroke);
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
                    btn = btn.fill(accent).stroke(accent_stroke);
                }
                if ui.add_sized([w, h], btn).on_hover_text(tip).clicked() {
                    (args.on_command)(FlightCommand::SetRepeat(next_mode));
                }
            }
            5 => {
                let mut btn = egui::Button::new(ic::SHUFFLE).min_size(egui::vec2(w, h));
                if shuffle {
                    btn = btn.fill(accent).stroke(accent_stroke);
                }
                if ui
                    .add_sized([w, h], btn)
                    .on_hover_text("Toggle deterministic seeded playlist shuffle.")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::SetShuffle(!shuffle));
                }
            }
            _ => {
                let mut btn = egui::Button::new(ic::RECORD).min_size(egui::vec2(w, h));
                if rec_active {
                    btn = btn.fill(rec_fill).stroke(Stroke::new(1.0, rec_col));
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
                    .size(11.0)
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
            let row_h = ROW_H + 4.0;
            let btn_sz = ROW_H + 2.0;
            let gap = 4.0;

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
                            .inner_margin(egui::Margin::symmetric(4, 2))
                            .show(ui, |ui| {
                                let row_w = ui.available_width().max(80.0);
                                let actions_w = btn_sz * 3.0 + gap * 3.0 + 28.0;
                                let left_w = (row_w - actions_w - gap).max(36.0);

                                ui.allocate_ui_with_layout(
                                    egui::vec2(row_w, row_h),
                                    Layout::left_to_right(Align::Center),
                                    |ui| {
                                        ui.set_max_width(row_w);
                                        ui.spacing_mut().item_spacing.x = gap;
                                        ui.allocate_ui_with_layout(
                                            egui::vec2(left_w, row_h),
                                            Layout::left_to_right(Align::Center),
                                            |ui| {
                                                ui.set_max_width(left_w);
                                                ui.spacing_mut().item_spacing.x = gap;
                                                let idx_txt = fmt_into(
                                                    args.label_buf,
                                                    format_args!("{:02}.", idx + 1),
                                                );
                                                ui.label(
                                                    RichText::new(idx_txt)
                                                        .monospace()
                                                        .size(10.0)
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
                                                            RichText::new(&slot.title).size(11.0),
                                                        )
                                                        .truncate()
                                                        .sense(Sense::click()),
                                                    )
                                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                                    .on_hover_text(&slot.description);
                                                if title_resp.clicked() {
                                                    deferred_cmds
                                                        .push(FlightCommand::SelectPlaylistEntry(idx));
                                                }
                                            },
                                        );
                                        ui.allocate_ui_with_layout(
                                            egui::vec2(actions_w, row_h),
                                            Layout::right_to_left(Align::Center),
                                            |ui| {
                                                ui.set_max_width(actions_w);
                                                ui.spacing_mut().item_spacing.x = gap;
                                                if ui
                                                    .add_sized(
                                                        [btn_sz, btn_sz],
                                                        egui::Button::new(ic::TRASH),
                                                    )
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
                                                            .min_size(egui::vec2(btn_sz, btn_sz)),
                                                    )
                                                    .on_hover_text("Move down")
                                                    .clicked()
                                                {
                                                    deferred_cmds.push(
                                                        FlightCommand::ReorderPlaylist {
                                                            from: idx,
                                                            to: idx + 1,
                                                        },
                                                    );
                                                }
                                                if ui
                                                    .add_enabled(
                                                        idx > 0,
                                                        egui::Button::new(ic::ARROW_UP)
                                                            .min_size(egui::vec2(btn_sz, btn_sz)),
                                                    )
                                                    .on_hover_text("Move up")
                                                    .clicked()
                                                {
                                                    deferred_cmds.push(
                                                        FlightCommand::ReorderPlaylist {
                                                            from: idx,
                                                            to: idx - 1,
                                                        },
                                                    );
                                                }
                                                let dur_txt = fmt_into(
                                                    args.label_buf,
                                                    format_args!("{:.0}s", slot.track.duration_s()),
                                                );
                                                ui.label(
                                                    RichText::new(dur_txt)
                                                        .monospace()
                                                        .size(10.0)
                                                        .weak(),
                                                );
                                            },
                                        );
                                    },
                                );

                                let hold_w = 42.0;
                                let hold_lbl_w = 24.0;
                                let combo_w = (row_w - hold_w - hold_lbl_w - gap * 3.0).max(52.0);
                                ui.allocate_ui_with_layout(
                                    egui::vec2(row_w, row_h),
                                    Layout::left_to_right(Align::Center),
                                    |ui| {
                                        ui.set_max_width(row_w);
                                        ui.spacing_mut().item_spacing.x = gap;
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
                                            deferred_cmds.push(
                                                FlightCommand::SetEntryTransition {
                                                    idx,
                                                    transition: new_tr,
                                                },
                                            );
                                        }

                                        ui.label(RichText::new("Hold").size(10.0).weak());
                                        let mut hold_val = slot.hold_s;
                                        let prev_style = ui.style().override_text_style.clone();
                                        ui.style_mut().override_text_style =
                                            Some(egui::TextStyle::Monospace);
                                        if ui
                                            .add_sized(
                                                [hold_w, ROW_H],
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
                                        ui.style_mut().override_text_style = prev_style;
                                    },
                                );
                            });
                    }
                });

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
                if ui
                    .add_enabled(
                        q_len > 0,
                        egui::Button::new(format!("{}  Save User", ic::FLOPPY_DISK))
                            .truncate()
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
                            .truncate()
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
                if ui
                    .add_sized(
                        [ui.available_width(), ROW_H + 2.0],
                        egui::Button::new(format!("{}  Rescan Library", ic::ARROWS_CLOCKWISE))
                            .truncate(),
                    )
                    .on_hover_text("Rescan assets/flights and ~/.nv-rs/flights on disk.")
                    .clicked()
                {
                    (args.on_command)(FlightCommand::RescanLibrary);
                }
            } else {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    if ui
                        .add_sized(
                            [86.0, ROW_H + 2.0],
                            egui::Button::new(format!("{}  Rescan", ic::ARROWS_CLOCKWISE))
                                .truncate(),
                        )
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
                                    .size(10.0)
                                    .color(ui.visuals().text_color()),
                                )
                                .truncate(),
                            );
                        });
                });
            }

            ui.add_space(2.0);
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
                        let row_h = ROW_H + 4.0;
                        let btn_sz = ROW_H + 2.0;
                        let gap = 4.0;
                        for pl in &args.workspace.player.library.playlists {
                            let row_w = ui.available_width().max(80.0);
                            let actions_w = btn_sz + gap + 56.0;
                            let left_w = (row_w - actions_w - gap).max(36.0);
                            let src = match pl.source {
                                LibrarySource::Repo => "Repo",
                                LibrarySource::User => "User",
                            };
                            ui.allocate_ui_with_layout(
                                egui::vec2(row_w, row_h),
                                Layout::left_to_right(Align::Center),
                                |ui| {
                                    ui.set_max_width(row_w);
                                    ui.spacing_mut().item_spacing.x = gap;
                                    ui.allocate_ui_with_layout(
                                        egui::vec2(left_w, row_h),
                                        Layout::left_to_right(Align::Center),
                                        |ui| {
                                            ui.set_max_width(left_w);
                                            ui.add(
                                                egui::Label::new(
                                                    RichText::new(&pl.name).size(11.0),
                                                )
                                                .truncate(),
                                            )
                                            .on_hover_text(pl.path.display().to_string());
                                        },
                                    );
                                    ui.allocate_ui_with_layout(
                                        egui::vec2(actions_w, row_h),
                                        Layout::right_to_left(Align::Center),
                                        |ui| {
                                            ui.set_max_width(actions_w);
                                            ui.spacing_mut().item_spacing.x = gap;
                                            if ui
                                                .add_sized(
                                                    [btn_sz, btn_sz],
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
                                            ui.label(
                                                RichText::new(meta).monospace().size(10.0).weak(),
                                            );
                                        },
                                    );
                                },
                            );
                        }
                    }
                });

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
    let row_h = ROW_H + 4.0;
    let btn_sz = ROW_H + 2.0;
    let gap = 4.0;
    let row_w = ui.available_width().max(80.0);
    let actions_w = btn_sz * 2.0 + gap * 2.0 + 30.0;
    let left_w = (row_w - actions_w - gap).max(36.0);

    ui.allocate_ui_with_layout(
        egui::vec2(row_w, row_h),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.set_max_width(row_w);
            ui.spacing_mut().item_spacing.x = gap;
            ui.allocate_ui_with_layout(
                egui::vec2(left_w, row_h),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.set_max_width(left_w);
                    ui.spacing_mut().item_spacing.x = gap;
                    if stale {
                        stale_badge(
                            ui,
                            "Stale: scene content hash differs from when this track was created.",
                        );
                    }
                    ui.add(egui::Label::new(RichText::new(name).size(11.0)).truncate())
                        .on_hover_text(format!("[{badge}] {name}\n{tooltip}"));
                },
            );
            ui.allocate_ui_with_layout(
                egui::vec2(actions_w, row_h),
                Layout::right_to_left(Align::Center),
                |ui| {
                    ui.set_max_width(actions_w);
                    ui.spacing_mut().item_spacing.x = gap;
                    if ui
                        .add_sized([btn_sz, btn_sz], egui::Button::new(ic::PLUS))
                        .on_hover_text("Append to playlist queue")
                        .clicked()
                    {
                        action = Some(LibraryRowAction::Enqueue);
                    }
                    if ui
                        .add_sized([btn_sz, btn_sz], egui::Button::new(ic::PLAY))
                        .on_hover_text("Play now")
                        .clicked()
                    {
                        action = Some(LibraryRowAction::Play);
                    }
                    let meta = fmt_into(buf, format_args!("{:.0}s", duration_s));
                    ui.label(RichText::new(meta).monospace().size(10.0).weak());
                },
            );
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
        let gap = 4.0;
        let ctrl_w = (ui.available_width() - ANIM_BTN_W - gap).max(40.0);
        ui.spacing_mut().item_spacing.x = gap;
        add_ctrl(ui, ctrl_w);
        anim_toggle_btn_disabled(ui);
    });
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
                    .size(10.0)
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
        ui.spacing_mut().item_spacing.x = 4.0;
        ui.label(RichText::new(ic::INFO).size(11.0).weak());
        ui.add(egui::Label::new(RichText::new(msg).size(10.5).weak()).truncate())
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
