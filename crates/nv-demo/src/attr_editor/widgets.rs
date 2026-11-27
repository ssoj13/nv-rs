//! Per-row DCC attribute editor widgets, scrubby numeric labels, typed clipboard, and deterministic
//! seed stepping for [`crate::attr_editor`].
//!
//! What: renders one `[dot | scrubby label | tabular editor | reset gutter]` row for any
//! [`AssetParamSpec`] (`Float`, `Int`, `Bool`, `Choice`, `Seed`), including right-click context
//! menus (`Reset to Default`, `Copy Value`, `Paste Value`, `Reroll Seed`), Ctrl/Cmd+Click reset,
//! and alternating row striping.
//! Why: mirrors the DCC ergonomics of `egui-widgets-rs`'s `attr_grid` (`sl-panels`) while staying
//! strictly width-clamped inside `nv-demo`'s narrow dock panes (`260 / 320 px`) with zero extra
//! external dependencies.
//! How: lays out each row inside [`crate::ui::allocate_clamped_ui`] using shared `VAL_W` and
//! [`crate::ui::slider_drag_value`] primitives so numeric boxes share one columnar right edge and
//! never expand parent `available_width()`.
//! Uses: `egui`, `egui_phosphor`, `csg_scene::asset_params`, `crate::ui`.
//! Used by: `crate::attr_editor::render_attr_editor`.

use std::sync::Arc;

use csg_scene::{AssetParamKind, AssetParamMap, AssetParamSpec, AssetParamValue};
use egui::{Align, Color32, Context, Id, Layout, RichText, Sense, Ui};
use egui_phosphor::regular as ic;

use crate::ui::{ROW_H, VAL_W, allocate_clamped_ui, slider_drag_value};

/// Uniform row height for attribute editor rows (`18 px`).
pub(super) const ATTR_ROW_H: f32 = ROW_H + 4.0;

fn clipboard_id() -> Id {
    Id::new("nv_demo_attr_editor_clipboard_v1")
}

/// Stores a typed attribute value in the process-wide `egui` memory slot.
pub(super) fn clipboard_set(ctx: &Context, value: AssetParamValue) {
    ctx.data_mut(|d| d.insert_temp(clipboard_id(), Arc::new(value)));
}

/// Reads the last copied typed attribute value from `egui` memory.
pub(super) fn clipboard_get(ctx: &Context) -> Option<AssetParamValue> {
    ctx.data(|d| d.get_temp::<Arc<AssetParamValue>>(clipboard_id()))
        .map(|arc| *arc)
}

/// Returns `true` when `val` matches the variant expected by `kind`.
pub(super) fn value_matches_kind(val: AssetParamValue, kind: AssetParamKind) -> bool {
    matches!(
        (val, kind),
        (AssetParamValue::Float(_), AssetParamKind::Float { .. })
            | (AssetParamValue::Int(_), AssetParamKind::Int { .. })
            | (AssetParamValue::Bool(_), AssetParamKind::Bool)
            | (AssetParamValue::Choice(_), AssetParamKind::Choice { .. })
            | (AssetParamValue::Seed(_), AssetParamKind::Seed)
    )
}

/// Advances a deterministic SplitMix64 generator to produce a fresh non-repeating seed.
#[inline]
pub(super) fn next_seed(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Computes an ergonomic decimal precision for a float parameter's declared `step`.
pub(super) fn decimals_for_step(step: f32) -> usize {
    if !step.is_finite() || step <= 0.0 {
        return 2;
    }
    if step >= 1.0 {
        0
    } else if step >= 0.1 {
        1
    } else if step >= 0.01 {
        2
    } else {
        3
    }
}

/// Formats a parameter value concisely for tooltips and default-value readouts.
pub(super) fn format_param_value(val: AssetParamValue, kind: AssetParamKind) -> String {
    match (val, kind) {
        (AssetParamValue::Float(v), AssetParamKind::Float { step, unit, .. }) => {
            let dec = decimals_for_step(step);
            if unit.is_empty() {
                format!("{v:.dec$}")
            } else {
                format!("{v:.dec$} {unit}")
            }
        }
        (AssetParamValue::Int(v), AssetParamKind::Int { unit, .. }) => {
            if unit.is_empty() {
                format!("{v}")
            } else {
                format!("{v} {unit}")
            }
        }
        (AssetParamValue::Bool(b), AssetParamKind::Bool) => {
            if b {
                "On".to_owned()
            } else {
                "Off".to_owned()
            }
        }
        (AssetParamValue::Choice(idx), AssetParamKind::Choice { options }) => options
            .get(idx)
            .copied()
            .unwrap_or("Option")
            .to_owned(),
        (AssetParamValue::Seed(s), AssetParamKind::Seed) => format!("{s}"),
        (AssetParamValue::Float(v), _) => format!("{v:.2}"),
        (AssetParamValue::Int(v), _) => format!("{v}"),
        (AssetParamValue::Bool(b), _) => format!("{b}"),
        (AssetParamValue::Choice(i), _) => format!("{i}"),
        (AssetParamValue::Seed(s), _) => format!("{s}"),
    }
}

/// Renders one width-clamped attribute row (`[dot | scrubby label | editor | reset]`) and returns
/// `true` if `values` changed this frame.
pub(super) fn render_param_row(
    ui: &mut Ui,
    id_salt: &str,
    spec: &AssetParamSpec,
    values: &mut AssetParamMap,
    seed_rng: &mut u64,
    zebra: bool,
) -> bool {
    let row_w = ui.available_width().max(1.0);
    let current = values
        .get(spec.key)
        .unwrap_or(spec.default)
        .clamped_to(spec.kind, spec.default);
    let modified = values.is_modified(spec);
    let mut next_val: Option<AssetParamValue> = None;

    let dot_w = 10.0;
    let action_w = 18.0;
    let gap = 3.0;
    let label_w = (row_w * 0.34).clamp(48.0, 92.0).floor();
    let editor_w = (row_w - dot_w - label_w - action_w - gap * 3.0).max(52.0);
    let ctrl_click_reset = ui.input(|i| i.modifiers.command);

    allocate_clamped_ui(
        ui,
        egui::vec2(row_w, ATTR_ROW_H),
        Layout::left_to_right(Align::Center),
        |ui| {
            let full_row_rect = ui.max_rect();
            if zebra && ui.is_rect_visible(full_row_rect) {
                let zebra_col = if ui.visuals().dark_mode {
                    Color32::from_rgba_unmultiplied(255, 255, 255, 7)
                } else {
                    Color32::from_rgba_unmultiplied(15, 23, 42, 8)
                };
                ui.painter().rect_filled(full_row_rect, 2.0, zebra_col);
            }

            ui.spacing_mut().item_spacing.x = gap;

            // 1. Modified-from-default indicator dot (click to reset).
            let (dot_rect, dot_resp) = ui.allocate_exact_size(
                egui::vec2(dot_w, ATTR_ROW_H),
                if modified {
                    Sense::click()
                } else {
                    Sense::hover()
                },
            );
            if modified && ui.is_rect_visible(dot_rect) {
                let accent = if ui.visuals().dark_mode {
                    Color32::from_rgb(96, 178, 255)
                } else {
                    Color32::from_rgb(37, 99, 235)
                };
                let r = if dot_resp.hovered() { 3.5 } else { 2.7 };
                ui.painter().circle_filled(dot_rect.center(), r, accent);
            }
            if modified {
                let dot_resp = dot_resp
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .on_hover_text(format!(
                        "Modified from default ({}) - click to reset",
                        format_param_value(spec.default, spec.kind)
                    ));
                if dot_resp.clicked() {
                    next_val = Some(spec.default);
                }
            }

            // 2. Scrubby label column with right-click context menu.
            let is_numeric = matches!(
                spec.kind,
                AssetParamKind::Float { .. } | AssetParamKind::Int { .. }
            );
            let label_sense = if is_numeric {
                Sense::click_and_drag()
            } else {
                Sense::click()
            };
            let (label_rect, label_resp) =
                ui.allocate_exact_size(egui::vec2(label_w, ATTR_ROW_H), label_sense);
            if ui.is_rect_visible(label_rect) {
                let text_col = if modified {
                    ui.visuals().strong_text_color()
                } else {
                    ui.visuals().weak_text_color()
                };
                let galley = ui.painter().layout_no_wrap(
                    spec.label.to_owned(),
                    egui::FontId::proportional(11.5),
                    text_col,
                );
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
            }

            if is_numeric && label_resp.dragged() {
                let dx = label_resp.drag_delta().x;
                if dx.abs() > 0.0 {
                    match (current, spec.kind) {
                        (
                            AssetParamValue::Float(v),
                            AssetParamKind::Float { min, max, step, .. },
                        ) => {
                            let lo = min.min(max);
                            let hi = min.max(max);
                            let span = (hi - lo).max(1e-4);
                            let speed = if step.is_finite() && step > 0.0 {
                                step * 0.35
                            } else {
                                span / 220.0
                            };
                            let updated = (v + dx * speed).clamp(lo, hi);
                            if (updated - v).abs() > 1e-6 {
                                next_val = Some(AssetParamValue::Float(updated));
                            }
                        }
                        (AssetParamValue::Int(v), AssetParamKind::Int { min, max, .. }) => {
                            let lo = min.min(max);
                            let hi = min.max(max);
                            let delta = (dx * 0.2).round() as i32;
                            if delta != 0 {
                                let updated = v.saturating_add(delta).clamp(lo, hi);
                                if updated != v {
                                    next_val = Some(AssetParamValue::Int(updated));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            if is_numeric && label_resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
            }
            if label_resp.clicked() && ctrl_click_reset && modified {
                next_val = Some(spec.default);
            }
            let label_resp = label_resp.on_hover_ui(|ui| {
                if !spec.tooltip.is_empty() {
                    ui.label(spec.tooltip);
                } else {
                    ui.label(spec.label);
                }
                ui.label(
                    RichText::new(format!(
                        "Default: {} | Right-click or Ctrl+Click to reset",
                        format_param_value(spec.default, spec.kind)
                    ))
                    .small()
                    .weak(),
                );
            });
            label_resp.context_menu(|ui| {
                if ui
                    .add_enabled(
                        modified,
                        egui::Button::new(format!(
                            "{}  Reset to Default ({})",
                            ic::ARROW_COUNTER_CLOCKWISE,
                            format_param_value(spec.default, spec.kind)
                        )),
                    )
                    .clicked()
                {
                    next_val = Some(spec.default);
                    ui.close();
                }
                if ui
                    .button(format!("{}  Copy Value", ic::COPY))
                    .clicked()
                {
                    clipboard_set(ui.ctx(), current);
                    ui.close();
                }
                let clip = clipboard_get(ui.ctx())
                    .filter(|v| value_matches_kind(*v, spec.kind))
                    .map(|v| v.clamped_to(spec.kind, spec.default));
                if ui
                    .add_enabled(
                        clip.is_some(),
                        egui::Button::new(format!("{}  Paste Value", ic::CLIPBOARD)),
                    )
                    .clicked()
                {
                    if let Some(v) = clip {
                        next_val = Some(v);
                    }
                    ui.close();
                }
                if matches!(spec.kind, AssetParamKind::Seed)
                    && ui
                        .button(format!("{}  Reroll Seed", ic::DICE_FIVE))
                        .clicked()
                {
                    next_val = Some(AssetParamValue::Seed(next_seed(seed_rng)));
                    ui.close();
                }
            });

            // 3. Width-clamped typed value editor column.
            allocate_clamped_ui(
                ui,
                egui::vec2(editor_w, ATTR_ROW_H),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    match (current, spec.kind) {
                        (
                            AssetParamValue::Float(mut v),
                            AssetParamKind::Float {
                                min,
                                max,
                                step,
                                unit,
                            },
                        ) => {
                            let lo = min.min(max);
                            let hi = min.max(max);
                            let dec = decimals_for_step(step);
                            let drag_speed = if step.is_finite() && step > 0.0 {
                                (step * 0.5).max(1e-3)
                            } else {
                                ((hi - lo) / 250.0).clamp(0.005, 1.0)
                            };
                            let val_w = VAL_W.min(editor_w);
                            let track_w = editor_w - val_w - 4.0;
                            let before = v;
                            let mut reset_req = false;
                            if track_w >= 22.0 {
                                ui.spacing_mut().slider_width = track_w;
                                let sr = ui.add_sized(
                                    [track_w, ROW_H],
                                    egui::Slider::new(&mut v, lo..=hi).show_value(false),
                                );
                                if sr.clicked() && ctrl_click_reset {
                                    reset_req = true;
                                }
                                if !spec.tooltip.is_empty() {
                                    sr.on_hover_text(spec.tooltip);
                                }
                            }
                            let mut drag = egui::DragValue::new(&mut v)
                                .range(lo..=hi)
                                .fixed_decimals(dec)
                                .speed(drag_speed);
                            if !unit.is_empty() {
                                drag = drag.suffix(format!(" {unit}"));
                            }
                            let dr = slider_drag_value(ui, val_w.min(ui.available_width().max(24.0)), drag);
                            if dr.clicked() && ctrl_click_reset {
                                reset_req = true;
                            }
                            if !spec.tooltip.is_empty() {
                                dr.on_hover_text(spec.tooltip);
                            }
                            if reset_req {
                                next_val = Some(spec.default);
                            } else if v.to_bits() != before.to_bits() {
                                next_val = Some(AssetParamValue::Float(v.clamp(lo, hi)));
                            }
                        }
                        (AssetParamValue::Int(mut v), AssetParamKind::Int { min, max, unit }) => {
                            let lo = min.min(max);
                            let hi = min.max(max);
                            let val_w = VAL_W.min(editor_w);
                            let track_w = editor_w - val_w - 4.0;
                            let before = v;
                            let mut reset_req = false;
                            if track_w >= 22.0 && hi.saturating_sub(lo) <= 256 {
                                ui.spacing_mut().slider_width = track_w;
                                let sr = ui.add_sized(
                                    [track_w, ROW_H],
                                    egui::Slider::new(&mut v, lo..=hi).show_value(false),
                                );
                                if sr.clicked() && ctrl_click_reset {
                                    reset_req = true;
                                }
                                if !spec.tooltip.is_empty() {
                                    sr.on_hover_text(spec.tooltip);
                                }
                            }
                            let mut drag = egui::DragValue::new(&mut v).range(lo..=hi).speed(0.25);
                            if !unit.is_empty() {
                                drag = drag.suffix(format!(" {unit}"));
                            }
                            let dr = slider_drag_value(ui, val_w.min(ui.available_width().max(24.0)), drag);
                            if dr.clicked() && ctrl_click_reset {
                                reset_req = true;
                            }
                            if !spec.tooltip.is_empty() {
                                dr.on_hover_text(spec.tooltip);
                            }
                            if reset_req {
                                next_val = Some(spec.default);
                            } else if v != before {
                                next_val = Some(AssetParamValue::Int(v.clamp(lo, hi)));
                            }
                        }
                        (AssetParamValue::Bool(mut b), AssetParamKind::Bool) => {
                            let before = b;
                            let lbl = if b { "Enabled" } else { "Disabled" };
                            let resp = ui.checkbox(&mut b, lbl);
                            if !spec.tooltip.is_empty() {
                                resp.on_hover_text(spec.tooltip);
                            }
                            if b != before {
                                next_val = Some(AssetParamValue::Bool(b));
                            }
                        }
                        (
                            AssetParamValue::Choice(mut idx),
                            AssetParamKind::Choice { options },
                        ) => {
                            let before = idx;
                            let selected = options.get(idx).copied().unwrap_or("Option");
                            let combo = egui::ComboBox::from_id_salt((id_salt, spec.key))
                                .width(editor_w.max(32.0))
                                .truncate()
                                .selected_text(selected)
                                .show_ui(ui, |ui| {
                                    for (i, opt) in options.iter().enumerate() {
                                        ui.selectable_value(&mut idx, i, *opt);
                                    }
                                });
                            if !spec.tooltip.is_empty() {
                                combo.response.on_hover_text(spec.tooltip);
                            }
                            if idx != before {
                                next_val = Some(AssetParamValue::Choice(idx));
                            }
                        }
                        (AssetParamValue::Seed(mut seed), AssetParamKind::Seed) => {
                            let dice_w = 20.0;
                            let drag_w = (editor_w - dice_w - 4.0).max(32.0);
                            let before = seed;
                            let dr = slider_drag_value(
                                ui,
                                drag_w,
                                egui::DragValue::new(&mut seed).speed(1.0),
                            );
                            if !spec.tooltip.is_empty() {
                                dr.on_hover_text(spec.tooltip);
                            }
                            if seed != before {
                                next_val = Some(AssetParamValue::Seed(seed));
                            }
                            if ui
                                .add_sized(
                                    [dice_w, ROW_H],
                                    egui::Button::new(RichText::new(ic::DICE_FIVE).size(11.0))
                                        .min_size(egui::vec2(dice_w, ROW_H)),
                                )
                                .on_hover_text("Roll a new deterministic random seed")
                                .clicked()
                            {
                                next_val = Some(AssetParamValue::Seed(next_seed(seed_rng)));
                            }
                        }
                        _ => {
                            ui.label(RichText::new(format_param_value(current, spec.kind)).weak());
                        }
                    }
                },
            );

            // 4. Compact right-gutter Reset action button.
            if modified {
                let reset_resp = ui
                    .add_sized(
                        [action_w, ROW_H],
                        egui::Button::new(
                            RichText::new(ic::ARROW_COUNTER_CLOCKWISE).size(10.5),
                        )
                        .min_size(egui::vec2(action_w, ROW_H)),
                    )
                    .on_hover_text("Reset parameter to default");
                if reset_resp.clicked() {
                    next_val = Some(spec.default);
                }
            } else {
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(action_w, ROW_H), Sense::hover());
                if ui.is_rect_visible(rect) {
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        ic::ARROW_COUNTER_CLOCKWISE,
                        egui::FontId::proportional(10.0),
                        ui.visuals().weak_text_color().gamma_multiply(0.25),
                    );
                }
            }
        },
    );

    if let Some(val) = next_val {
        let clamped = val.clamped_to(spec.kind, spec.default);
        if clamped != current || (clamped == spec.default && modified) {
            if clamped == spec.default {
                values.reset_key(spec);
            } else {
                values.set(spec.key, clamped);
            }
            return true;
        }
    }
    false
}
