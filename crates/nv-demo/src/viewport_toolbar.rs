//! Vendored collapsible, edge-dockable floating viewport toolbar adapted from
//! `egui-widgets-rs` (`sl-viewport/src/toolbar/mod.rs`) with zero external `sl_*` dependencies.
//!
//! - **What**: Floating glass-pill [`ViewportToolbar`] anchored to any of the four viewport edges
//!   ([`ToolbarEdge`]) with smooth animated collapse/expand (`ctx.animate_value_with_time`),
//!   drag-to-edge re-anchoring on the toggle handle, and 100% `egui-phosphor` icons.
//! - **Why**: Keeps high-frequency viewport controls (AOV, Lens, Day/Sunset/Night, Poke, Freeze,
//!   Exposure, Save PNG / SDG EXR) one click away inside the viewport without cluttering or
//!   blocking camera navigation.
//! - **How**: Renders inside `egui::Area` (`Order::Middle`), absorbs pointer/click/drag events on
//!   its rect first, and returns [`ToolbarResponse`] (`contains_pointer`) so the viewport camera
//!   driver gates mouse/wheel/double-click input cleanly on `!bar.contains_pointer`.

use egui::{
    Align2, Area, Color32, CornerRadius, FontId, Id, Order, Pos2, Rect, Sense, Stroke,
    StrokeKind, Ui, Vec2,
};
use egui_phosphor::regular as ic;
use serde::{Deserialize, Serialize};

/// Which edge of the viewport the floating [`ViewportToolbar`] is anchored to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolbarEdge {
    /// Bottom edge of the viewport (flush to the bottom frame).
    Bottom,
    /// Top edge of the viewport (flush to the top frame, default).
    #[default]
    Top,
    /// Left edge of the viewport (flush to the left frame).
    Left,
    /// Right edge of the viewport (flush to the right frame).
    Right,
}

impl ToolbarEdge {
    /// True when the toolbar lays out items horizontally (`Top` or `Bottom`).
    #[inline]
    pub const fn is_horizontal(self) -> bool {
        matches!(self, Self::Top | Self::Bottom)
    }

    /// Snap a pointer position to the nearest edge of `viewport`.
    pub fn nearest(viewport: Rect, pos: Pos2) -> Self {
        let dl = (pos.x - viewport.left()).abs();
        let dr = (viewport.right() - pos.x).abs();
        let dt = (pos.y - viewport.top()).abs();
        let db = (viewport.bottom() - pos.y).abs();
        let min_d = dl.min(dr).min(dt).min(db);
        if (min_d - db).abs() <= 1e-3 {
            Self::Bottom
        } else if (min_d - dt).abs() <= 1e-3 {
            Self::Top
        } else if (min_d - dl).abs() <= 1e-3 {
            Self::Left
        } else {
            Self::Right
        }
    }
}

/// Persistent state for [`ViewportToolbar`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolbarState {
    /// Currently docked viewport edge.
    pub edge: ToolbarEdge,
    /// Whether the toolbar is currently expanded (`true`) or collapsed to its toggle handle.
    pub expanded: bool,
    /// Last measured natural width when expanded horizontally (`Top`/`Bottom`).
    #[serde(skip)]
    pub measured_extent: f32,
    /// Last measured cross-axis width when expanded vertically (`Left`/`Right`).
    #[serde(skip)]
    pub measured_vertical_width: f32,
    /// Last measured main-axis height when expanded vertically (`Left`/`Right`).
    #[serde(skip)]
    pub measured_vertical_height: f32,
}

impl Default for ToolbarState {
    fn default() -> Self {
        Self {
            edge: ToolbarEdge::Top,
            expanded: true,
            measured_extent: 560.0,
            measured_vertical_width: 118.0,
            measured_vertical_height: 360.0,
        }
    }
}

/// Computes the target `(width, height)` of the floating [`ViewportToolbar`] for the given
/// `state` and `viewport` without clipping vertical (`Left`/`Right`) controls to `thickness`.
pub fn compute_toolbar_target_size(
    state: &ToolbarState,
    thickness: f32,
    toggle_size: f32,
    viewport: Rect,
) -> Vec2 {
    let collapsed_extent = toggle_size + 8.0;
    let max_horiz = (viewport.width() - 8.0).max(collapsed_extent);
    let max_vert = (viewport.height() - 48.0).max(collapsed_extent);
    if state.edge.is_horizontal() {
        let w = if state.expanded {
            let eff = if state.measured_extent > collapsed_extent {
                state.measured_extent
            } else {
                560.0
            };
            eff.clamp(collapsed_extent, max_horiz)
        } else {
            collapsed_extent
        };
        Vec2::new(w.min(max_horiz), thickness)
    } else {
        let max_vert_w = (viewport.width() * 0.45).max(thickness);
        let (w, h) = if state.expanded {
            let eff_w = if state.measured_vertical_width > thickness {
                state.measured_vertical_width
            } else {
                118.0
            };
            let eff_h = if state.measured_vertical_height > collapsed_extent {
                state.measured_vertical_height
            } else {
                360.0
            };
            (
                eff_w.clamp(thickness, max_vert_w),
                eff_h.clamp(collapsed_extent, max_vert),
            )
        } else {
            (thickness, collapsed_extent)
        };
        Vec2::new(w.min(max_vert_w), h.min(max_vert))
    }
}

/// Output returned by [`ViewportToolbar::show`] for input gating.
#[derive(Clone, Copy, Debug)]
pub struct ToolbarResponse {
    /// True if the pointer currently hovers or drags the toolbar (gate viewport camera on `!contains_pointer`).
    pub contains_pointer: bool,
}

/// Collapsible, edge-dockable glass-pill viewport toolbar anchored flush to any viewport edge.
pub struct ViewportToolbar {
    id: Id,
    thickness: f32,
    toggle_size: f32,
}

impl ViewportToolbar {
    /// Create a new edge-docked viewport toolbar identified by `id_source`.
    pub fn new(id_source: impl std::hash::Hash + std::fmt::Debug) -> Self {
        Self {
            id: Id::new(id_source),
            thickness: 30.0,
            toggle_size: 24.0,
        }
    }

    /// Render the edge-docked toolbar flush against `viewport` and populate its interior with `add_contents`.
    pub fn show(
        self,
        ctx: &egui::Context,
        viewport: Rect,
        state: &mut ToolbarState,
        add_contents: impl FnOnce(&mut Ui),
    ) -> ToolbarResponse {
        let collapsed_extent = self.toggle_size + 8.0;
        let max_horiz = (viewport.width() - 8.0).max(collapsed_extent);
        let max_vert = (viewport.height() - 48.0).max(collapsed_extent);
        let max_vert_w = (viewport.width() * 0.45).max(self.thickness);
        let target = compute_toolbar_target_size(state, self.thickness, self.toggle_size, viewport);

        let (target_main, target_cross) = if state.edge.is_horizontal() {
            (target.x, target.y)
        } else {
            (target.y, target.x)
        };
        let anim_extent = ctx.animate_value_with_time(self.id.with("extent"), target_main, 0.16);
        let anim_cross = ctx.animate_value_with_time(self.id.with("cross"), target_cross, 0.16);
        let (w, h) = if state.edge.is_horizontal() {
            (anim_extent.min(max_horiz), self.thickness)
        } else {
            (
                anim_cross.clamp(self.thickness, max_vert_w),
                anim_extent.min(max_vert),
            )
        };

        // Anchor flush against the chosen viewport edge (and aligned to the top-right / edge
        // so it never floats mid-viewport or collides with the top-left RIDING overlay).
        let (min, radius) = match state.edge {
            ToolbarEdge::Top => (
                Pos2::new(viewport.right() - w, viewport.top()),
                CornerRadius {
                    nw: 0,
                    ne: 0,
                    sw: 8,
                    se: 0,
                },
            ),
            ToolbarEdge::Bottom => (
                Pos2::new(viewport.right() - w, viewport.bottom() - h),
                CornerRadius {
                    nw: 8,
                    ne: 0,
                    sw: 0,
                    se: 0,
                },
            ),
            ToolbarEdge::Left => (
                Pos2::new(viewport.left(), viewport.center().y - 0.5 * h),
                CornerRadius {
                    nw: 0,
                    ne: 8,
                    sw: 0,
                    se: 8,
                },
            ),
            ToolbarEdge::Right => (
                Pos2::new(viewport.right() - w, viewport.center().y - 0.5 * h),
                CornerRadius {
                    nw: 8,
                    ne: 0,
                    sw: 8,
                    se: 0,
                },
            ),
        };
        let rect = Rect::from_min_size(min, Vec2::new(w, h));

        let mut contains_pointer = false;

        Area::new(self.id.with("area"))
            .order(Order::Middle)
            .fixed_pos(rect.min)
            .constrain_to(viewport)
            .show(ctx, |ui| {
                ui.set_min_size(rect.size());
                ui.set_max_size(rect.size());

                // Absorb pointer clicks/drags on the backdrop first so they never leak to the viewport.
                let bg_resp = ui.allocate_rect(rect, Sense::click_and_drag());
                let fill = Color32::from_rgba_unmultiplied(15, 19, 26, 228);
                let border = Stroke::new(1.0, Color32::from_rgba_unmultiplied(92, 108, 132, 135));
                ui.painter()
                    .rect(rect, radius, fill, border, StrokeKind::Inside);

                let inner = rect.shrink2(Vec2::new(4.0, 3.0));
                let toggle_sz = self.toggle_size.min(inner.width()).min(inner.height());
                let toggle_min = if state.edge.is_horizontal() {
                    inner.min
                } else {
                    Pos2::new(inner.center().x - 0.5 * toggle_sz, inner.top())
                };
                let toggle_rect = Rect::from_min_size(toggle_min, Vec2::splat(toggle_sz));

                // Toggle button doubles as a drag-to-edge handle.
                let toggle_resp = ui.interact(
                    toggle_rect,
                    self.id.with("toggle"),
                    Sense::click_and_drag(),
                );
                let toggle_fill = if toggle_resp.hovered() || toggle_resp.dragged() {
                    Color32::from_rgba_unmultiplied(64, 82, 108, 160)
                } else {
                    Color32::from_rgba_unmultiplied(36, 44, 58, 120)
                };
                ui.painter().rect_filled(toggle_rect, CornerRadius::same(6), toggle_fill);
                let glyph = match (state.edge, state.expanded) {
                    (ToolbarEdge::Bottom | ToolbarEdge::Top, true) => ic::CARET_LEFT,
                    (ToolbarEdge::Bottom | ToolbarEdge::Top, false) => ic::CARET_RIGHT,
                    (ToolbarEdge::Left | ToolbarEdge::Right, true) => ic::CARET_UP,
                    (ToolbarEdge::Left | ToolbarEdge::Right, false) => ic::CARET_DOWN,
                };
                ui.painter().text(
                    toggle_rect.center(),
                    Align2::CENTER_CENTER,
                    glyph,
                    FontId::proportional(13.0),
                    Color32::from_rgb(206, 218, 236),
                );

                if toggle_resp.clicked() {
                    state.expanded = !state.expanded;
                }
                if toggle_resp.dragged()
                    && let Some(p) = ctx.pointer_interact_pos()
                {
                    let new_edge = ToolbarEdge::nearest(viewport, p);
                    if new_edge != state.edge {
                        state.edge = new_edge;
                    }
                }
                toggle_resp.on_hover_text(
                    "Click to collapse/expand toolbar. Drag to dock to any viewport edge.",
                );

                // Content area (visible whenever expanded enough to fit controls cleanly).
                if state.expanded && anim_extent > collapsed_extent + 12.0 {
                    let content_rect = if state.edge.is_horizontal() {
                        Rect::from_min_max(
                            Pos2::new(toggle_rect.right() + 4.0, inner.top()),
                            inner.max,
                        )
                    } else {
                        Rect::from_min_max(
                            Pos2::new(inner.left(), toggle_rect.bottom() + 4.0),
                            inner.max,
                        )
                    };
                    let mut child_ui = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(content_rect)
                            .layout(if state.edge.is_horizontal() {
                                egui::Layout::left_to_right(egui::Align::Center)
                            } else {
                                egui::Layout::top_down(egui::Align::Center)
                            }),
                    );
                    child_ui.set_clip_rect(content_rect);
                    child_ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
                    child_ui.spacing_mut().button_padding = Vec2::new(5.0, 2.0);
                    add_contents(&mut child_ui);
                    let used = child_ui.min_size();
                    if state.edge.is_horizontal() {
                        let desired =
                            (self.toggle_size + 16.0 + used.x).max(collapsed_extent + 24.0);
                        if desired.is_finite() && (desired - state.measured_extent).abs() > 1.0 {
                            state.measured_extent = desired;
                        }
                    } else {
                        let desired_h =
                            (self.toggle_size + 16.0 + used.y).max(collapsed_extent + 24.0);
                        let desired_w = (used.x + 12.0).max(self.thickness);
                        if desired_h.is_finite()
                            && (desired_h - state.measured_vertical_height).abs() > 1.0
                        {
                            state.measured_vertical_height = desired_h;
                        }
                        if desired_w.is_finite()
                            && (desired_w - state.measured_vertical_width).abs() > 1.0
                        {
                            state.measured_vertical_width = desired_w;
                        }
                    }
                }

                let ptr_in = ctx
                    .pointer_hover_pos()
                    .is_some_and(|p| rect.expand(2.0).contains(p));
                contains_pointer = ptr_in || bg_resp.hovered() || bg_resp.dragged();
            });

        ToolbarResponse { contains_pointer }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_edge_selects_closest_viewport_side() {
        let vp = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 600.0));
        assert_eq!(
            ToolbarEdge::nearest(vp, Pos2::new(500.0, 580.0)),
            ToolbarEdge::Bottom
        );
        assert_eq!(
            ToolbarEdge::nearest(vp, Pos2::new(500.0, 20.0)),
            ToolbarEdge::Top
        );
        assert_eq!(
            ToolbarEdge::nearest(vp, Pos2::new(20.0, 300.0)),
            ToolbarEdge::Left
        );
        assert_eq!(
            ToolbarEdge::nearest(vp, Pos2::new(980.0, 300.0)),
            ToolbarEdge::Right
        );
    }

    #[test]
    fn vertical_left_right_toolbar_expands_cross_width_and_collapses_cleanly() {
        let vp = Rect::from_min_size(Pos2::ZERO, Vec2::new(1280.0, 720.0));
        for edge in [ToolbarEdge::Left, ToolbarEdge::Right] {
            let mut st = ToolbarState {
                edge,
                expanded: true,
                measured_extent: 560.0,
                measured_vertical_width: 118.0,
                measured_vertical_height: 360.0,
            };
            let expanded = compute_toolbar_target_size(&st, 30.0, 24.0, vp);
            assert!(
                expanded.x >= 110.0,
                "{edge:?} expanded width must not clip to 30px, got {}",
                expanded.x
            );
            assert!(expanded.y >= 300.0);

            // Deserialized ToolbarState (`#[serde(skip)]` zeros) also gets a safe non-clipping width.
            st.measured_vertical_width = 0.0;
            st.measured_vertical_height = 0.0;
            let fallback = compute_toolbar_target_size(&st, 30.0, 24.0, vp);
            assert!(fallback.x >= 110.0, "fallback width={}", fallback.x);

            // Collapsed vertical toolbar shrinks cleanly back to the 30px pill.
            st.expanded = false;
            let collapsed = compute_toolbar_target_size(&st, 30.0, 24.0, vp);
            assert_eq!(collapsed.x, 30.0);
            assert_eq!(collapsed.y, 32.0);
        }
    }
}
