//! Vendored & adapted DCC Attribute Editor for `nv-demo`'s Parametric Asset Inspector.
//!
//! What: provides [`AttrEditorState`], [`AttrEditorResponse`], and [`render_attr_editor`] -- a
//! search/filterable, collapsible-group attribute grid over [`csg_scene::AssetParamSpec`] and
//! [`csg_scene::AssetParamMap`].
//! Why: brings the Maya/Houdini DCC attribute-editor ergonomics of `egui-widgets-rs`'s
//! `sl-panels::attr_grid` (search bar, collapsible groups, modified-from-default dot + 1-click and
//! right-click reset, scrubby numeric labels, typed copy/paste clipboard, right-aligned tabular
//! numeric widgets, seed dice reroll, and plain-English tooltips) directly into `nv-demo` with
//! zero new external crate dependencies.
//! How: groups incoming [`AssetParamSpec`]s in deterministic schema declaration order, filters by
//! case-insensitive search and modified-only state, and renders each row via
//! [`widgets::render_param_row`] inside `nv-demo`'s width-clamped SSOT primitives.
//! Uses: `egui`, `egui_phosphor`, `csg_scene::asset_params`, `crate::ui`.
//! Used by: `crate::ui` (`sec_asset_inspector` inside `Render & Floor`).

mod widgets;

use std::collections::HashSet;

use csg_scene::{AssetParamMap, AssetParamSpec};
use egui::{Align, Color32, Layout, RichText, Sense, Ui};
use egui_phosphor::regular as ic;

use crate::ui::{ROW_H, active_accent_tint, allocate_clamped_ui};

/// Persistent UI state for one Attribute Editor instance (search query, filters, collapsed groups,
/// and deterministic seed-reroll RNG state).
#[derive(Clone, Debug)]
pub struct AttrEditorState {
    /// Case-insensitive filter over parameter label, key, group, and tooltip.
    pub search: String,
    /// When `true`, displays only parameters whose current value differs from their default.
    pub modified_only: bool,
    /// Deterministic SplitMix64 state for the `Seed` dice-reroll button.
    pub seed_rng_state: u64,
    collapsed_groups: HashSet<String>,
}

impl Default for AttrEditorState {
    fn default() -> Self {
        Self {
            search: String::new(),
            modified_only: false,
            seed_rng_state: 0x517C_C1B7_2722_0A95,
            collapsed_groups: HashSet::new(),
        }
    }
}

impl AttrEditorState {
    /// Returns `true` when `group` is currently expanded.
    #[inline]
    pub fn is_group_open(&self, group: &str) -> bool {
        !self.collapsed_groups.contains(group)
    }

    /// Toggles the open/collapsed state of parameter group `group`.
    pub fn toggle_group(&mut self, group: &str) {
        if !self.collapsed_groups.remove(group) {
            self.collapsed_groups.insert(group.to_owned());
        }
    }
}

/// Summary returned from [`render_attr_editor`] for the current frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AttrEditorResponse {
    /// `true` when at least one parameter value was modified or reset this frame.
    pub changed: bool,
    /// Total number of schema parameters currently differing from their defaults.
    pub modified_count: usize,
}

fn spec_matches_query(spec: &AssetParamSpec, query_lc: &str) -> bool {
    if query_lc.is_empty() {
        return true;
    }
    spec.label.to_ascii_lowercase().contains(query_lc)
        || spec.key.to_ascii_lowercase().contains(query_lc)
        || spec.group.to_ascii_lowercase().contains(query_lc)
        || spec.tooltip.to_ascii_lowercase().contains(query_lc)
}

/// Renders the DCC Attribute Editor over `schema` and `values`, returning whether any parameter
/// changed this frame along with the live modified-parameter count.
pub fn render_attr_editor(
    ui: &mut Ui,
    id_salt: &str,
    schema: &[AssetParamSpec],
    values: &mut AssetParamMap,
    state: &mut AttrEditorState,
) -> AttrEditorResponse {
    let mut changed = false;
    let total_modified = schema.iter().filter(|s| values.is_modified(s)).count();

    // 1. Compact search / filter / reset toolbar (strictly width-clamped).
    let bar_w = ui.available_width().max(1.0);
    let bar_h = ROW_H + 6.0;
    allocate_clamped_ui(
        ui,
        egui::vec2(bar_w, bar_h),
        Layout::left_to_right(Align::Center),
        |ui| {
            let gap = 3.0;
            ui.spacing_mut().item_spacing.x = gap;
            let clear_w = if state.search.is_empty() { 0.0 } else { 20.0 + gap };
            let mod_w = 24.0;
            let rst_w = 22.0;
            let edit_w = (bar_w - clear_w - mod_w - rst_w - gap * 2.0).max(48.0);

            ui.add_sized(
                [edit_w, bar_h - 2.0],
                egui::TextEdit::singleline(&mut state.search)
                    .hint_text(format!("{} Filter attributes...", ic::MAGNIFYING_GLASS)),
            )
            .on_hover_text("Filter attributes by name, group, or description.");

            if !state.search.is_empty()
                && ui
                    .add_sized(
                        [20.0, bar_h - 2.0],
                        egui::Button::new(RichText::new(ic::X).size(10.5))
                            .min_size(egui::vec2(20.0, bar_h - 2.0)),
                    )
                    .on_hover_text("Clear attribute filter")
                    .clicked()
            {
                state.search.clear();
            }

            let mut mod_btn = egui::Button::new(RichText::new(ic::FUNNEL).size(11.0))
                .min_size(egui::vec2(mod_w, bar_h - 2.0));
            if state.modified_only {
                mod_btn = mod_btn
                    .fill(active_accent_tint(ui))
                    .stroke(egui::Stroke::new(1.0, ui.visuals().selection.stroke.color));
            }
            if ui
                .add_sized([mod_w, bar_h - 2.0], mod_btn)
                .on_hover_text(format!(
                    "Show only modified parameters ({total_modified} modified)"
                ))
                .clicked()
            {
                state.modified_only = !state.modified_only;
            }

            if ui
                .add_enabled(
                    total_modified > 0,
                    egui::Button::new(RichText::new(ic::ARROW_COUNTER_CLOCKWISE).size(11.0))
                        .min_size(egui::vec2(rst_w, bar_h - 2.0)),
                )
                .on_hover_text("Reset all modified parameters back to defaults")
                .clicked()
            {
                for spec in schema {
                    if values.is_modified(spec) {
                        values.reset_key(spec);
                        changed = true;
                    }
                }
            }
        },
    );

    ui.add_space(2.0);

    // 2. Collect unique group names in deterministic schema declaration order.
    let query_lc = state.search.trim().to_ascii_lowercase();
    let mut groups: Vec<&'static str> = Vec::new();
    let mut matched_any = false;
    for spec in schema {
        if state.modified_only && !values.is_modified(spec) {
            continue;
        }
        if !spec_matches_query(spec, &query_lc) {
            continue;
        }
        matched_any = true;
        if !groups.contains(&spec.group) {
            groups.push(spec.group);
        }
    }

    if !matched_any {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(if state.modified_only && total_modified == 0 {
                    "All parameters are at default values."
                } else {
                    "No matching parameters."
                })
                .size(11.0)
                .weak(),
            );
            if (!state.search.is_empty() || state.modified_only)
                && ui.small_button("Show All").clicked()
            {
                state.search.clear();
                state.modified_only = false;
            }
        });
        let modified_count = schema.iter().filter(|s| values.is_modified(s)).count();
        return AttrEditorResponse {
            changed,
            modified_count,
        };
    }

    // 3. Render collapsible groups and rows.
    let force_expand = !query_lc.is_empty() || state.modified_only;
    for group in groups {
        let group_mod = schema
            .iter()
            .filter(|s| s.group == group && values.is_modified(s))
            .count();
        let is_open = force_expand || state.is_group_open(group);

        let hdr_w = ui.available_width().max(1.0);
        let hdr_h = 19.0;
        let (hdr_rect, hdr_resp) =
            ui.allocate_exact_size(egui::vec2(hdr_w, hdr_h), Sense::click());
        if hdr_resp.clicked() && !force_expand {
            state.toggle_group(group);
        }
        if ui.is_rect_visible(hdr_rect) {
            let p = ui.painter();
            let bg = if hdr_resp.hovered() {
                ui.visuals().widgets.hovered.weak_bg_fill
            } else {
                ui.visuals().faint_bg_color
            };
            p.rect_filled(hdr_rect, 2.5, bg);
            let caret = if is_open {
                ic::CARET_DOWN
            } else {
                ic::CARET_RIGHT
            };
            let title_col = ui.visuals().text_color();
            p.text(
                egui::pos2(hdr_rect.left() + 6.0, hdr_rect.center().y),
                egui::Align2::LEFT_CENTER,
                format!("{caret}  {group}"),
                egui::FontId::proportional(11.5),
                title_col,
            );
            if group_mod > 0 {
                let badge_col = if ui.visuals().dark_mode {
                    Color32::from_rgb(96, 178, 255)
                } else {
                    Color32::from_rgb(37, 99, 235)
                };
                p.text(
                    egui::pos2(hdr_rect.right() - 6.0, hdr_rect.center().y),
                    egui::Align2::RIGHT_CENTER,
                    format!("{group_mod} mod"),
                    egui::FontId::monospace(10.0),
                    badge_col,
                );
            }
        }

        if is_open {
            let mut row_idx = 0usize;
            for spec in schema {
                if spec.group != group {
                    continue;
                }
                if state.modified_only && !values.is_modified(spec) {
                    continue;
                }
                if !spec_matches_query(spec, &query_lc) {
                    continue;
                }
                if widgets::render_param_row(
                    ui,
                    id_salt,
                    spec,
                    values,
                    &mut state.seed_rng_state,
                    row_idx % 2 == 1,
                ) {
                    changed = true;
                }
                row_idx += 1;
            }
            ui.add_space(2.0);
        }
    }

    let modified_count = schema.iter().filter(|s| values.is_modified(s)).count();
    AttrEditorResponse {
        changed,
        modified_count,
    }
}

#[cfg(test)]
mod tests {
    use csg_scene::{AssetParamKind, AssetParamMap, AssetParamSpec, AssetParamValue};

    use super::*;
    use crate::ui::{UiTheme, install_fonts};

    fn sample_schema() -> Vec<AssetParamSpec> {
        vec![
            AssetParamSpec::seed("seed", "Seed", "General", "Procedural variation seed.", 42),
            AssetParamSpec::float(
                "uniform_scale",
                "Uniform Scale",
                "General",
                "Overall asset scale multiplier.",
                1.0,
                0.25,
                3.0,
                0.05,
                "x",
            ),
            AssetParamSpec::float(
                "height_m",
                "Height",
                "Dimensions",
                "Total height in meters.",
                12.5,
                4.0,
                48.0,
                0.5,
                "m",
            ),
            AssetParamSpec::int(
                "floors",
                "Floors",
                "Dimensions",
                "Number of architectural stories.",
                4,
                1,
                16,
                "fl",
            ),
            AssetParamSpec::bool(
                "lit_night",
                "Night Lights",
                "Lighting",
                "Illuminate windows and emissive fixtures.",
                true,
            ),
            AssetParamSpec::choice(
                "paint",
                "Body Paint",
                "Appearance",
                "Clearcoat exterior finish.",
                1,
                &["Silver", "Pacific Blue", "Crimson Red", "Pearl White"],
            ),
        ]
    }

    #[test]
    fn attr_editor_fits_narrow_docks_without_staircase_or_overflow() {
        let schema = sample_schema();
        for theme in [UiTheme::Obsidian, UiTheme::Light] {
            for panel_w in [240.0_f32, 260.0, 320.0] {
                let ctx = egui::Context::default();
                install_fonts(&ctx);
                let _dock_style = theme.apply(&ctx);
                let mut values = AssetParamMap::from_schema(&schema);
                values.set("floors", AssetParamValue::Int(8));
                values.set("uniform_scale", AssetParamValue::Float(1.35));
                let mut state = AttrEditorState::default();
                let mut avail_before = 0.0_f32;
                let mut avail_after = 0.0_f32;
                let mut max_right = 0.0_f32;
                let mut mod_count = 0usize;

                for _pass in 0..2 {
                    let raw = egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(panel_w, 900.0),
                        )),
                        ..Default::default()
                    };
                    let out = ctx.run_ui(raw, |ui| {
                        crate::ui::dock_scroll_for_test(ui, |ui| {
                            avail_before = ui.available_width();
                            let resp = render_attr_editor(
                                ui,
                                "test_attr_ed",
                                &schema,
                                &mut values,
                                &mut state,
                            );
                            mod_count = resp.modified_count;
                            avail_after = ui.available_width();
                            max_right = ui.min_rect().right();
                        });
                    });
                    out.drop_without_applying_deltas();
                }

                assert_eq!(mod_count, 2);
                assert!(
                    (avail_after - avail_before).abs() <= 0.5,
                    "{theme:?} width {panel_w}: attr_editor drifted available_width ({avail_before} -> {avail_after})"
                );
                assert!(
                    max_right <= panel_w + 0.5,
                    "{theme:?} width {panel_w}: attr_editor overflowed right edge ({max_right} > {panel_w})"
                );
            }
        }
    }

    #[test]
    fn attr_editor_search_group_toggle_and_seed_step_behave_deterministically() {
        let schema = sample_schema();
        assert!(spec_matches_query(&schema[2], "height"));
        assert!(spec_matches_query(&schema[2], "dimensions"));
        assert!(!spec_matches_query(&schema[2], "paint"));

        let mut state = AttrEditorState::default();
        assert!(state.is_group_open("Dimensions"));
        state.toggle_group("Dimensions");
        assert!(!state.is_group_open("Dimensions"));
        state.toggle_group("Dimensions");
        assert!(state.is_group_open("Dimensions"));

        let s1 = widgets::next_seed(&mut state.seed_rng_state);
        let s2 = widgets::next_seed(&mut state.seed_rng_state);
        assert_ne!(s1, s2);
        assert_eq!(
            widgets::format_param_value(
                AssetParamValue::Float(12.5),
                AssetParamKind::Float {
                    min: 0.0,
                    max: 50.0,
                    step: 0.5,
                    unit: "m",
                },
            ),
            "12.5 m"
        );
    }
}
