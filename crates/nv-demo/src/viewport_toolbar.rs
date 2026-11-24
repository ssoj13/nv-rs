//! Vendored collapsible, edge-dockable floating viewport toolbar adapted from
//! `egui-widgets-rs` (`sl-viewport/src/toolbar/mod.rs`) with zero external `sl_*` dependencies.
//!
//! - **What**: Floating glass-pill [`ViewportToolbar`] anchored to any of the four viewport edges
//!   ([`ToolbarEdge`]) with smooth animated collapse/expand (`ctx.animate_value_with_time`),
//!   drag-to-edge re-anchoring on the toggle handle, and 100% `egui-phosphor` icons.
//! - **Why**: Keeps high-frequency viewport controls (AOV, Lens, Day/Sunset/Night, Poke, Freeze,
//!   Exposure, Save PNG / SDG EXR) one click away inside the viewport without cluttering or
//!   blocking camera navigation.
//! - **Layout**: `Top` / `Bottom` toolbars are left-aligned ([`toolbar_anchor`]). A second toolbar
//!   (the possession toolbar in `ui.rs`) stacks on the inner side of the first one
//!   ([`ViewportToolbar::stack`]): directly above a bottom-docked toolbar, below a top-docked one,
//!   beside a side-docked one, always in the same corner and with the same alignment. The
//!   bottom-left status overlays clear any bottom-docked bars ([`bottom_reserve`]).
//! - **Style**: both toolbars share one frame (drawn here) and one set of design tokens
//!   ([`TB_BUTTON_SIZE`], [`TB_COMBO_WIDTH`], [`tb_icon_button`], [`tb_separator`]): equal
//!   button sizes, centred `egui-phosphor` icons, tooltips carrying the text labels.
//! - **Possession toolbar state**: [`possess_bar_expanded`] / [`PossessBarPrefs::on_user_toggle`]
//!   (auto-expand on entering possession unless the user collapsed it, auto-collapse on leaving
//!   unless pinned; the user's choices persist).
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

/// Cross-axis thickness of a horizontal (`Top` / `Bottom`) toolbar in points.
pub const TOOLBAR_THICKNESS: f32 = 30.0;

/// Gap between stacked toolbars and between a bottom-docked toolbar and the overlays above it.
pub const TOOLBAR_STACK_GAP: f32 = 3.0;

/// Top-left corner and corner rounding of a toolbar of `size` docked on `edge` of `viewport`,
/// moved `stack` points inwards (away from the edge) when it sits on top of another toolbar.
///
/// Horizontal toolbars (`Top` / `Bottom`) are left-aligned, flush with the viewport's left edge,
/// so they grow to the right and their collapsed handle stays at a fixed spot. Vertical toolbars
/// are centred on their edge. Only the corner facing into the viewport is rounded.
pub fn toolbar_anchor(edge: ToolbarEdge, viewport: Rect, size: Vec2, stack: f32) -> (Pos2, CornerRadius) {
    let r = TB_FRAME_RADIUS;
    let round = |nw, ne, sw, se| CornerRadius { nw, ne, sw, se };
    match edge {
        ToolbarEdge::Top => (
            Pos2::new(viewport.left(), viewport.top() + stack),
            round(0, 0, 0, r),
        ),
        ToolbarEdge::Bottom => (
            Pos2::new(viewport.left(), viewport.bottom() - size.y - stack),
            round(0, r, 0, 0),
        ),
        ToolbarEdge::Left => (
            Pos2::new(viewport.left() + stack, viewport.center().y - 0.5 * size.y),
            round(0, r, 0, r),
        ),
        ToolbarEdge::Right => (
            Pos2::new(viewport.right() - size.x - stack, viewport.center().y - 0.5 * size.y),
            round(r, 0, r, 0),
        ),
    }
}

/// Inward offset for a toolbar stacked on `below`, the rectangle of the toolbar it sits on.
pub fn stack_offset(edge: ToolbarEdge, viewport: Rect, below: Rect) -> f32 {
    let inner = match edge {
        ToolbarEdge::Top => below.bottom() - viewport.top(),
        ToolbarEdge::Bottom => viewport.bottom() - below.top(),
        ToolbarEdge::Left => below.right() - viewport.left(),
        ToolbarEdge::Right => viewport.right() - below.left(),
    };
    inner.max(0.0) + TOOLBAR_STACK_GAP
}

/// Height at the bottom of the viewport occupied by the toolbars in `bars` when they are docked
/// at the bottom, so the bottom overlays (status badge, WGS84 pill) stack above them.
pub fn bottom_reserve(edge: ToolbarEdge, viewport: Rect, bars: &[Rect]) -> f32 {
    if edge != ToolbarEdge::Bottom {
        return 0.0;
    }
    bars.iter()
        .map(|r| viewport.bottom() - r.top() + TOOLBAR_STACK_GAP)
        .fold(0.0, f32::max)
}

// ---- Shared toolbar design tokens (one source for both toolbars) ----------------------------

/// Corner radius of the toolbar frame's inner corner.
pub const TB_FRAME_RADIUS: u8 = 8;
/// Size of every toolbar button (icon centred).
pub const TB_BUTTON_SIZE: Vec2 = Vec2::new(24.0, 22.0);
/// Icon glyph size inside a toolbar button.
pub const TB_ICON_SIZE: f32 = 14.0;
/// Width of every toolbar combo box.
pub const TB_COMBO_WIDTH: f32 = 100.0;
/// Width of every toolbar slider track.
pub const TB_SLIDER_WIDTH: f32 = 64.0;
/// Spacing between toolbar items.
pub const TB_ITEM_SPACING: Vec2 = Vec2::new(4.0, 4.0);
const TB_FILL: Color32 = Color32::from_rgba_unmultiplied_const(15, 19, 26, 228);
const TB_BORDER: Color32 = Color32::from_rgba_unmultiplied_const(92, 108, 132, 135);
const TB_DANGER: Color32 = Color32::from_rgb(168, 46, 54);

/// Visual tone of a toolbar button.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TbTone {
    /// Regular button (selected state = egui selection colours).
    Normal,
    /// Destructive / exit action (Eject): red fill, white icon.
    Danger,
}

/// One toolbar icon button: fixed [`TB_BUTTON_SIZE`], centred `egui-phosphor` `icon`, selected
/// state, and `tooltip` carrying the text label the icon replaces.
pub fn tb_icon_button(ui: &mut Ui, icon: &str, selected: bool, tone: TbTone, tooltip: &str) -> egui::Response {
    let text = egui::RichText::new(icon).size(TB_ICON_SIZE);
    let button = match tone {
        TbTone::Normal => egui::Button::selectable(selected, text),
        TbTone::Danger => egui::Button::new(text.color(Color32::WHITE)).fill(TB_DANGER),
    };
    ui.add_sized(TB_BUTTON_SIZE, button).on_hover_text(tooltip)
}

/// Group separator with the toolbar spacing on both sides.
pub fn tb_separator(ui: &mut Ui) {
    ui.separator();
}

// ---- Possession toolbar expand / collapse rule ------------------------------------------------

/// The user's persisted choices for the possession toolbar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PossessBarPrefs {
    /// The user collapsed the bar by hand: entering possession no longer expands it.
    pub user_collapsed: bool,
    /// Pinned open: leaving possession no longer collapses it.
    pub pinned: bool,
}

impl PossessBarPrefs {
    /// Records a click on the bar's handle that left it `now_expanded`: collapsing by hand
    /// suppresses the auto-expand, expanding by hand restores it.
    pub fn on_user_toggle(&mut self, now_expanded: bool) {
        self.user_collapsed = !now_expanded;
    }
}

/// Whether the possession toolbar is expanded after a frame in which possession went from
/// `was_possessing` to `possessing`: entering expands it unless the user collapsed it, leaving
/// collapses it unless it is pinned, and anything else keeps `expanded`.
pub const fn possess_bar_expanded(
    expanded: bool,
    was_possessing: bool,
    possessing: bool,
    prefs: PossessBarPrefs,
) -> bool {
    match (was_possessing, possessing) {
        (false, true) => !prefs.user_collapsed,
        (true, false) => prefs.pinned,
        _ => expanded,
    }
}

/// Runtime state of the possession toolbar: its bar (edge follows the viewport toolbar), the
/// persisted user choices and last frame's possession flag.
#[derive(Clone, Debug, PartialEq)]
pub struct PossessToolbar {
    /// The bar's own state; its edge is copied from the viewport toolbar every frame so the two
    /// always share a corner.
    pub bar: ToolbarState,
    /// The user's persisted choices (manual collapse, pin).
    pub prefs: PossessBarPrefs,
    /// Last frame's possession state, to detect entering / leaving possession.
    pub was_possessing: bool,
}

impl PossessToolbar {
    /// Not possessing at start: collapsed unless pinned.
    pub fn new(prefs: PossessBarPrefs) -> Self {
        Self {
            bar: ToolbarState {
                expanded: prefs.pinned,
                ..ToolbarState::default()
            },
            prefs,
            was_possessing: false,
        }
    }

    /// Applies [`possess_bar_expanded`] for this frame's possession state.
    pub fn sync(&mut self, possessing: bool) {
        self.bar.expanded =
            possess_bar_expanded(self.bar.expanded, self.was_possessing, possessing, self.prefs);
        self.was_possessing = possessing;
    }
}

/// Output returned by [`ViewportToolbar::show`] for input gating.
#[derive(Clone, Copy, Debug)]
pub struct ToolbarResponse {
    /// True if the pointer currently hovers or drags the toolbar (gate viewport camera on `!contains_pointer`).
    pub contains_pointer: bool,
    /// The toolbar's rectangle this frame (stack the next toolbar on it).
    pub rect: Rect,
    /// The user clicked the collapse / expand handle this frame.
    pub toggled: bool,
}

/// Collapsible, edge-dockable glass-pill viewport toolbar anchored flush to any viewport edge.
pub struct ViewportToolbar {
    id: Id,
    thickness: f32,
    toggle_size: f32,
    stack: f32,
    dockable: bool,
}

impl ViewportToolbar {
    /// Create a new edge-docked viewport toolbar identified by `id_source`.
    pub fn new(id_source: impl std::hash::Hash + std::fmt::Debug) -> Self {
        Self {
            id: Id::new(id_source),
            thickness: TOOLBAR_THICKNESS,
            toggle_size: 24.0,
            stack: 0.0,
            dockable: true,
        }
    }

    /// Stack this toolbar `offset` points inwards from its edge (see [`stack_offset`]). A stacked
    /// toolbar follows the edge of the one below it and cannot be dragged to another edge.
    pub fn stack(mut self, offset: f32) -> Self {
        self.stack = offset;
        self.dockable = false;
        self
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

        let (min, radius) = toolbar_anchor(state.edge, viewport, Vec2::new(w, h), self.stack);
        let rect = Rect::from_min_size(min, Vec2::new(w, h));

        let mut contains_pointer = false;
        let mut toggled = false;

        Area::new(self.id.with("area"))
            .order(Order::Middle)
            .fixed_pos(rect.min)
            .constrain_to(viewport)
            .show(ctx, |ui| {
                ui.set_min_size(rect.size());
                ui.set_max_size(rect.size());

                // Absorb pointer clicks/drags on the backdrop first so they never leak to the viewport.
                let bg_resp = ui.allocate_rect(rect, Sense::click_and_drag());
                ui.painter()
                    .rect(rect, radius, TB_FILL, Stroke::new(1.0, TB_BORDER), StrokeKind::Inside);

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
                    toggled = true;
                }
                if self.dockable
                    && toggle_resp.dragged()
                    && let Some(p) = ctx.pointer_interact_pos()
                {
                    let new_edge = ToolbarEdge::nearest(viewport, p);
                    if new_edge != state.edge {
                        state.edge = new_edge;
                    }
                }
                toggle_resp.on_hover_text(if self.dockable {
                    "Click to collapse/expand toolbar. Drag to dock to any viewport edge."
                } else {
                    "Click to collapse/expand toolbar."
                });

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
                    child_ui.spacing_mut().item_spacing = TB_ITEM_SPACING;
                    child_ui.spacing_mut().slider_width = TB_SLIDER_WIDTH;
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

        ToolbarResponse {
            contains_pointer,
            rect,
            toggled,
        }
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

    #[test]
    fn horizontal_toolbar_is_left_aligned_on_top_and_bottom() {
        let vp = Rect::from_min_size(Pos2::new(40.0, 20.0), Vec2::new(1000.0, 600.0));
        let size = Vec2::new(420.0, TOOLBAR_THICKNESS);
        let (top, top_r) = toolbar_anchor(ToolbarEdge::Top, vp, size, 0.0);
        assert_eq!(top, Pos2::new(40.0, 20.0));
        // Only the inner (bottom-right) corner is rounded.
        assert_eq!((top_r.nw, top_r.ne, top_r.sw, top_r.se), (0, 0, 0, TB_FRAME_RADIUS));
        let (bottom, bottom_r) = toolbar_anchor(ToolbarEdge::Bottom, vp, size, 0.0);
        assert_eq!(bottom, Pos2::new(40.0, 620.0 - TOOLBAR_THICKNESS));
        assert_eq!((bottom_r.nw, bottom_r.ne, bottom_r.sw, bottom_r.se), (0, TB_FRAME_RADIUS, 0, 0));
        // The narrow-dock clamp keeps a left-aligned toolbar inside the viewport.
        let narrow = Rect::from_min_size(Pos2::new(40.0, 20.0), Vec2::new(200.0, 600.0));
        let st = ToolbarState::default();
        let w = compute_toolbar_target_size(&st, TOOLBAR_THICKNESS, 24.0, narrow).x;
        let (min, _) = toolbar_anchor(st.edge, narrow, Vec2::new(w, TOOLBAR_THICKNESS), 0.0);
        assert!(min.x >= narrow.left() && min.x + w <= narrow.right(), "x={} w={w}", min.x);
    }

    #[test]
    fn vertical_toolbar_stays_centred_on_its_edge() {
        let vp = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 600.0));
        let size = Vec2::new(118.0, 360.0);
        assert_eq!(toolbar_anchor(ToolbarEdge::Left, vp, size, 0.0).0, Pos2::new(0.0, 120.0));
        assert_eq!(toolbar_anchor(ToolbarEdge::Right, vp, size, 0.0).0, Pos2::new(882.0, 120.0));
    }

    #[test]
    fn second_toolbar_stacks_on_the_inner_side_in_the_same_corner() {
        let vp = Rect::from_min_size(Pos2::new(10.0, 10.0), Vec2::new(1000.0, 600.0));
        let size = Vec2::new(300.0, TOOLBAR_THICKNESS);
        for edge in [ToolbarEdge::Top, ToolbarEdge::Bottom] {
            let (m0, _) = toolbar_anchor(edge, vp, size, 0.0);
            let first = Rect::from_min_size(m0, size);
            let (m1, _) = toolbar_anchor(edge, vp, size, stack_offset(edge, vp, first));
            let second = Rect::from_min_size(m1, size);
            assert_eq!(m1.x, m0.x, "{edge:?}: same left alignment");
            assert!(!first.intersects(second), "{edge:?}: bars must not overlap");
            let gap = if edge == ToolbarEdge::Bottom {
                first.top() - second.bottom()
            } else {
                second.top() - first.bottom()
            };
            assert!((gap - TOOLBAR_STACK_GAP).abs() < 1e-4, "{edge:?}: gap {gap}");
        }
        // Bottom: the second bar sits directly above the first.
        let (m0, _) = toolbar_anchor(ToolbarEdge::Bottom, vp, size, 0.0);
        let first = Rect::from_min_size(m0, size);
        let (m1, _) = toolbar_anchor(ToolbarEdge::Bottom, vp, size, stack_offset(ToolbarEdge::Bottom, vp, first));
        assert!(m1.y < m0.y);
        // Bottom overlays clear both bars; other edges reserve nothing.
        let second = Rect::from_min_size(m1, size);
        let reserve = bottom_reserve(ToolbarEdge::Bottom, vp, &[first, second]);
        assert!((vp.bottom() - reserve - second.top() + TOOLBAR_STACK_GAP).abs() < 1e-4);
        assert_eq!(bottom_reserve(ToolbarEdge::Top, vp, &[first, second]), 0.0);
    }

    #[test]
    fn possession_bar_auto_expands_unless_the_user_collapsed_it() {
        let mut prefs = PossessBarPrefs::default();
        // Starts collapsed; entering possession expands it.
        assert!(possess_bar_expanded(false, false, true, prefs));
        // While possessing nothing changes on its own.
        assert!(possess_bar_expanded(true, true, true, prefs));
        assert!(!possess_bar_expanded(false, true, true, prefs));
        // Leaving collapses it.
        assert!(!possess_bar_expanded(true, true, false, prefs));

        // The user collapses it during a possession: the next possession stays collapsed.
        prefs.on_user_toggle(false);
        assert!(prefs.user_collapsed);
        assert!(!possess_bar_expanded(false, false, true, prefs));
        // Expanding by hand restores the auto-expand.
        prefs.on_user_toggle(true);
        assert!(possess_bar_expanded(false, false, true, prefs));

        // Pinned open: leaving possession keeps it expanded.
        prefs.pinned = true;
        assert!(possess_bar_expanded(true, true, false, prefs));
        // Not possessing and not entering: the current state is kept either way.
        assert!(possess_bar_expanded(true, false, false, prefs));
        assert!(!possess_bar_expanded(false, false, false, prefs));
    }
}
