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
    /// Bottom edge of the viewport (above the HUD status line).
    #[default]
    Bottom,
    /// Top edge of the viewport.
    Top,
    /// Left edge of the viewport.
    Left,
    /// Right edge of the viewport.
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
    /// Last measured natural width when expanded (used for smooth collapse/expand animation).
    #[serde(skip)]
    pub measured_extent: f32,
}

impl Default for ToolbarState {
    fn default() -> Self {
        Self {
            edge: ToolbarEdge::Bottom,
            expanded: true,
            measured_extent: 440.0,
        }
    }
}

/// Output returned by [`ViewportToolbar::show`] for input gating.
#[derive(Clone, Copy, Debug)]
pub struct ToolbarResponse {
    /// True if the pointer currently hovers or drags the toolbar (gate viewport camera on `!contains_pointer`).
    pub contains_pointer: bool,
}

/// Collapsible, edge-dockable floating glass-pill viewport toolbar.
pub struct ViewportToolbar {
    id: Id,
    margin_bottom: f32,
    margin_other: f32,
    thickness: f32,
    toggle_size: f32,
}

impl ViewportToolbar {
    /// Create a new floating toolbar identified by `id_source`.
    pub fn new(id_source: impl std::hash::Hash + std::fmt::Debug) -> Self {
        Self {
            id: Id::new(id_source),
            margin_bottom: 34.0,
            margin_other: 10.0,
            thickness: 30.0,
            toggle_size: 24.0,
        }
    }

    /// Override the bottom clearance above the viewport's bottom HUD strip.
    pub const fn bottom_margin(mut self, margin: f32) -> Self {
        self.margin_bottom = margin;
        self
    }

    /// Render the floating toolbar over `viewport` and populate its interior with `add_contents`.
    pub fn show(
        self,
        ctx: &egui::Context,
        viewport: Rect,
        state: &mut ToolbarState,
        add_contents: impl FnOnce(&mut Ui),
    ) -> ToolbarResponse {
        let collapsed_extent = self.toggle_size + 8.0;
        let max_horiz = (viewport.width() - 24.0).max(collapsed_extent);
        let max_vert = (viewport.height() - 64.0).max(collapsed_extent);
        let target_extent = if state.expanded {
            if state.edge.is_horizontal() {
                state.measured_extent.clamp(collapsed_extent, max_horiz)
            } else {
                state.measured_extent.clamp(collapsed_extent, max_vert)
            }
        } else {
            collapsed_extent
        };

        let anim_extent = ctx.animate_value_with_time(self.id.with("extent"), target_extent, 0.16);
        let (w, h) = if state.edge.is_horizontal() {
            (anim_extent.min(max_horiz), self.thickness)
        } else {
            (self.thickness, anim_extent.min(max_vert))
        };

        let min = match state.edge {
            ToolbarEdge::Bottom => Pos2::new(
                viewport.center().x - 0.5 * w,
                viewport.bottom() - self.margin_bottom - h,
            ),
            ToolbarEdge::Top => Pos2::new(
                viewport.center().x - 0.5 * w,
                viewport.top() + self.margin_other,
            ),
            ToolbarEdge::Left => Pos2::new(
                viewport.left() + self.margin_other,
                viewport.center().y - 0.5 * h,
            ),
            ToolbarEdge::Right => Pos2::new(
                viewport.right() - self.margin_other - w,
                viewport.center().y - 0.5 * h,
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
                let fill = Color32::from_rgba_unmultiplied(15, 19, 26, 224);
                let border = Stroke::new(1.0, Color32::from_rgba_unmultiplied(92, 108, 132, 125));
                let radius = CornerRadius::same(8);
                ui.painter()
                    .rect(rect, radius, fill, border, StrokeKind::Inside);

                let inner = rect.shrink2(Vec2::new(4.0, 3.0));
                let toggle_rect = Rect::from_min_size(
                    inner.min,
                    Vec2::splat(self.toggle_size.min(inner.width()).min(inner.height())),
                );

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
                    let desired = if state.edge.is_horizontal() {
                        (self.toggle_size + 16.0 + used.x).max(collapsed_extent + 24.0)
                    } else {
                        (self.toggle_size + 16.0 + used.y).max(collapsed_extent + 24.0)
                    };
                    if desired.is_finite() && (desired - state.measured_extent).abs() > 1.0 {
                        state.measured_extent = desired;
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
}
