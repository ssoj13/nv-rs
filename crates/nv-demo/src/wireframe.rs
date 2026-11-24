//! Viewport display modes (`DEMO:VIEWS`) and the hidden-line wireframe contour rules.
//!
//! - **What**: [`DisplayMode`] selects what the viewport shows: the shaded CSG path-traced render
//!   or a hidden-line wireframe of smooth, antialiased contour lines over flat light-grey faces
//!   with slight ambient occlusion.
//! - **Why**: a wireframe makes CSG structure (silhouettes, creases, part and instance borders)
//!   readable at a glance. Hidden lines are removed by construction: only the visible primary hit
//!   of each pixel is used, so geometry behind a surface never contributes a line.
//! - **How**: in wireframe mode `main.rs` renders at the native viewport resolution (no DLSS
//!   input scale, no render scale) and the raygen runs with `aov_mode == AOV_WIREFRAME`: one
//!   pixel-centre primary ray, `(packed octahedral normal, object diameter, depth, instance key)`
//!   into `normal_f32`, the frame's AO into `color_f32.w`. `shaders/tonemap.cu` then builds vector
//!   contours on the pixel-centre lattice: `wire_classify` marks every lattice edge whose two
//!   pixels differ (sky / depth jump = silhouette; other instance or crease angle = crease) with
//!   a crossing vertex (marching squares), `wire_smooth` runs `WIRE_SMOOTH_STEPS` Laplacian passes
//!   that pull each vertex towards its two contour neighbours unless the contour turns sharply
//!   there (corners, junctions and line ends stay), and `wire_faces` shades flat light-grey faces
//!   with slight AO. The lines themselves are not drawn here: `wire_emit` is the contour provider
//!   of the generic overlay renderer (`overlay.rs`), which strokes them after the tonemap with a
//!   constant screen width, line LOD by projected object size and temporal stabilisation.
//! - **Memory**: no per-frame allocation. The G-buffer and first vertex buffer reuse
//!   `normal_f32` / `albedo_f32`; `wire_cls_u32` and `wire_pos_f32` are allocated once on first
//!   use (and on resize) by `renderer::ensure_lazy_aovs`.
//! - **Used by**: `renderer.rs` (`launch_wireframe`, `run_overlay`), `ui.rs` (viewport selector and
//!   line width), `main.rs` (native resolution, DLSS off, capture decisions via
//!   `DemoSettings::launch_aov_mode`). The settings persist with the rest of `DemoSettings`.
//! - **Voxel view**: pending; csg-scene exposes no voxel volume / ESDF yet (`PLAN.md` `DEMO:VIEWS`).

use serde::{Deserialize, Serialize};

/// `aov_mode` value of the wireframe G-buffer pass (`AOV_WIREFRAME` in `raytracer.cu` and
/// `tonemap.cu`). One past the 11 SDG passes of `ui::AOV_PASSES`.
pub const AOV_WIREFRAME: u32 = 11;

/// Relative inverse-depth curvature above which a lattice edge is a depth jump
/// (`|1/d_a + 1/d_b - 2/d_c| * d_c` on both sides of the edge). Zero on any plane under a
/// pinhole camera, so slanted ground far away never draws.
pub const WIRE_DEPTH_THRESHOLD: f32 = 0.1;

/// Cosine of the crease angle: neighbouring normals more than ~35 degrees apart draw a crease.
pub const WIRE_CREASE_COS: f32 = 0.82;

/// Default silhouette stroke width in screen pixels.
pub const WIRE_LINE_PX_DEFAULT: f32 = 1.5;
/// Smallest selectable silhouette stroke width.
pub const WIRE_LINE_PX_MIN: f32 = 0.75;
/// Largest selectable silhouette stroke width (a drawing aid, not a thickness: wider strokes
/// merge neighbouring contours).
pub const WIRE_LINE_PX_MAX: f32 = 2.5;
/// Crease strokes are this fraction of the silhouette width.
pub const WIRE_CREASE_WIDTH_RATIO: f32 = 2.0 / 3.0;

/// Laplacian smoothing passes over the contour vertices (even, so the result lands in the first
/// vertex buffer). Measured on a slope-0.37 staircase (`wireframe` tests): max deviation from the
/// true contour 0.455 px raw, 0.126 px after 6 passes; shrinkage of a 10 px radius curve stays
/// around 0.1 px.
pub const WIRE_SMOOTH_STEPS: u32 = 6;
const _: () = assert!(WIRE_SMOOTH_STEPS.is_multiple_of(2));

/// Step factor of each smoothing pass: a vertex moves this fraction towards the mean of its two
/// contour neighbours.
pub const WIRE_SMOOTH_LAMBDA: f32 = 0.5;

/// What the viewport displays.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisplayMode {
    /// The shaded path-traced render (the selected AOV applies).
    #[default]
    Csg,
    /// Hidden-line wireframe: contour lines over flat-shaded light faces.
    Wireframe,
}

impl DisplayMode {
    /// Every mode, in selector order.
    pub const ALL: [Self; 2] = [Self::Csg, Self::Wireframe];

    /// Selector label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Csg => "CSG",
            Self::Wireframe => "Wireframe",
        }
    }

    /// Short viewport HUD tag.
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Csg => "CSG",
            Self::Wireframe => "WIRE",
        }
    }
}

#[cfg(test)]
mod tests {
    //! CPU mirror of the `wire_classify` / `wire_smooth` rules and `ovl_coverage` in
    //! `shaders/tonemap.cu`; keep both identical.

    use super::*;

    const NONE: u32 = 0;
    const SIL: u32 = 1;
    const CREASE: u32 = 2;
    const CORNER_COS: f32 = 0.57;

    /// Primary-hit data of one pixel as the G-buffer pass stores it.
    #[derive(Clone, Copy)]
    struct Px {
        depth: f32,
        n: [f32; 3],
        id: u32,
        hit: bool,
    }

    const MISS: Px = Px {
        depth: 1e6,
        n: [0.0, 0.0, 1.0],
        id: u32::MAX,
        hit: false,
    };
    const UP: [f32; 3] = [0.0, 1.0, 0.0];

    fn px(depth: f32, n: [f32; 3], id: u32) -> Px {
        Px {
            depth,
            n,
            id,
            hit: true,
        }
    }

    fn inv_depth_curv(a: f32, c: f32, b: f32) -> f32 {
        (1.0 / a + 1.0 / b - 2.0 / c).abs() * c
    }

    /// `wire_pair_class`.
    fn pair_class(a: Px, p: Px, q: Px, b: Px) -> u32 {
        if !p.hit && !q.hit {
            return NONE;
        }
        if p.hit != q.hit {
            return SIL;
        }
        let ad = if a.hit { a.depth } else { p.depth };
        let bd = if b.hit { b.depth } else { q.depth };
        if inv_depth_curv(ad, p.depth, q.depth).min(inv_depth_curv(p.depth, q.depth, bd))
            > WIRE_DEPTH_THRESHOLD
        {
            return SIL;
        }
        if p.id != q.id {
            return CREASE;
        }
        let d = p.n[0] * q.n[0] + p.n[1] * q.n[1] + p.n[2] * q.n[2];
        if d < WIRE_CREASE_COS { CREASE } else { NONE }
    }

    /// `ovl_coverage`.
    fn coverage(dist: f32, width: f32) -> f32 {
        let t = (0.5 * width + 0.5 - dist).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t) * width.min(1.0)
    }

    /// Classified lattice of a `w x h` G-buffer: per pixel the right / down edge classes and
    /// their vertices `[right.x, right.y, down.x, down.y]`.
    struct Lattice {
        w: usize,
        h: usize,
        cls: Vec<[u32; 2]>,
        pos: Vec<[f32; 4]>,
    }

    /// `wire_classify`.
    fn classify(g: &[Px], w: usize, h: usize) -> Lattice {
        let at = |x: isize, y: isize| {
            g[(y.clamp(0, h as isize - 1) as usize) * w + x.clamp(0, w as isize - 1) as usize]
        };
        let mut cls = vec![[NONE; 2]; w * h];
        let mut pos = vec![[0.0; 4]; w * h];
        for y in 0..h as isize {
            for x in 0..w as isize {
                let i = y as usize * w + x as usize;
                let p = at(x, y);
                if x + 1 < w as isize {
                    cls[i][0] = pair_class(at(x - 1, y), p, at(x + 1, y), at(x + 2, y));
                }
                if y + 1 < h as isize {
                    cls[i][1] = pair_class(at(x, y - 1), p, at(x, y + 1), at(x, y + 2));
                }
                pos[i] = [x as f32 + 1.0, y as f32 + 0.5, x as f32 + 0.5, y as f32 + 1.0];
            }
        }
        Lattice { w, h, cls, pos }
    }

    /// One lattice cell: edges T, B, L, R (see `WireCell` in tonemap.cu).
    struct Cell {
        v: [[f32; 2]; 4],
        c: [u32; 4],
        n: u32,
    }

    fn cell(l: &Lattice, pos: &[[f32; 4]], cx: usize, cy: usize) -> Cell {
        let i = |x: usize, y: usize| y * l.w + x;
        let (t, b, r) = (i(cx, cy), i(cx, cy + 1), i(cx + 1, cy));
        let c = [l.cls[t][0], l.cls[b][0], l.cls[t][1], l.cls[r][1]];
        let v = [
            [pos[t][0], pos[t][1]],
            [pos[b][0], pos[b][1]],
            [pos[t][2], pos[t][3]],
            [pos[r][2], pos[r][3]],
        ];
        let n = c.iter().filter(|&&k| k != NONE).count() as u32;
        Cell { v, c, n }
    }

    fn dist(a: [f32; 2], b: [f32; 2]) -> f32 {
        (a[0] - b[0]).hypot(a[1] - b[1])
    }

    /// `wire_partner`.
    fn partner(k: &Cell, e: usize) -> Option<usize> {
        if k.c[e] == NONE {
            return None;
        }
        match k.n {
            2 => (0..4).find(|&o| o != e && k.c[o] != NONE),
            4 => {
                let tl = dist(k.v[0], k.v[2]) + dist(k.v[1], k.v[3])
                    <= dist(k.v[0], k.v[3]) + dist(k.v[1], k.v[2]);
                Some(if tl { [2, 3, 0, 1][e] } else { [3, 2, 1, 0][e] })
            }
            _ => None,
        }
    }

    /// A contour vertex `(x, y, axis)` (`WireVtx`).
    type Vtx = (isize, isize, usize);

    /// `wire_vtx_cell`: the cell of `v` on `side` and `v`'s edge index in it.
    fn vtx_cell(v: Vtx, side: usize) -> (isize, isize, usize) {
        let (x, y, axis) = v;
        match (axis, side) {
            (0, 0) => (x, y - 1, 1),
            (0, _) => (x, y, 0),
            (_, 0) => (x - 1, y, 3),
            _ => (x, y, 2),
        }
    }

    /// `wire_edge_vtx`.
    fn edge_vtx(cx: isize, cy: isize, e: usize) -> Vtx {
        (cx + isize::from(e == 3), cy + isize::from(e == 1), usize::from(e >= 2))
    }

    /// `wire_step`: neighbour of `v` through its cell on `side`, and that cell.
    fn step(l: &Lattice, pos: &[[f32; 4]], v: Vtx, side: usize) -> Option<(Vtx, (isize, isize))> {
        let (cx, cy, e) = vtx_cell(v, side);
        if cx < 0 || cy < 0 || cx + 1 >= l.w as isize || cy + 1 >= l.h as isize {
            return None;
        }
        let k = cell(l, pos, cx as usize, cy as usize);
        partner(&k, e).map(|p| (edge_vtx(cx, cy, p), (cx, cy)))
    }

    /// `wire_step_away`.
    fn step_away(l: &Lattice, pos: &[[f32; 4]], a: Vtx, from: (isize, isize)) -> Vtx {
        let (cx, cy, _) = vtx_cell(a, 0);
        let side = usize::from((cx, cy) == from);
        step(l, pos, a, side).map_or(a, |(o, _)| o)
    }

    fn vtx_pos(l: &Lattice, pos: &[[f32; 4]], v: Vtx) -> [f32; 2] {
        let p = pos[v.1 as usize * l.w + v.0 as usize];
        [p[2 * v.2], p[2 * v.2 + 1]]
    }

    /// One `wire_smooth` pass.
    fn smooth(l: &Lattice, src: &[[f32; 4]], lambda: f32) -> Vec<[f32; 4]> {
        let mut out = src.to_vec();
        for y in 0..l.h {
            for x in 0..l.w {
                for axis in 0..2 {
                    if l.cls[y * l.w + x][axis] == NONE {
                        continue;
                    }
                    let v = (x as isize, y as isize, axis);
                    let (Some((a, ac)), Some((b, bc))) = (step(l, src, v, 0), step(l, src, v, 1))
                    else {
                        continue;
                    };
                    let pv = vtx_pos(l, src, v);
                    let (pa, pb) = (vtx_pos(l, src, a), vtx_pos(l, src, b));
                    let pa2 = vtx_pos(l, src, step_away(l, src, a, ac));
                    let pb2 = vtx_pos(l, src, step_away(l, src, b, bc));
                    let d0 = [pv[0] - pa2[0], pv[1] - pa2[1]];
                    let d1 = [pb2[0] - pv[0], pb2[1] - pv[1]];
                    let (l0, l1) = (d0[0].hypot(d0[1]), d1[0].hypot(d1[1]));
                    if l0 < 1e-6
                        || l1 < 1e-6
                        || (d0[0] * d1[0] + d0[1] * d1[1]) / (l0 * l1) < CORNER_COS
                    {
                        continue;
                    }
                    let o = &mut out[y * l.w + x];
                    o[2 * axis] = pv[0] + lambda * (0.5 * (pa[0] + pb[0]) - pv[0]);
                    o[2 * axis + 1] = pv[1] + lambda * (0.5 * (pa[1] + pb[1]) - pv[1]);
                }
            }
        }
        out
    }

    fn smoothed(l: &Lattice) -> Vec<[f32; 4]> {
        (0..WIRE_SMOOTH_STEPS).fold(l.pos.clone(), |p, _| smooth(l, &p, WIRE_SMOOTH_LAMBDA))
    }

    /// Silhouette vertices of `l` away from the image border, as `(x, y)`.
    fn sil_vertices(l: &Lattice, pos: &[[f32; 4]]) -> Vec<[f32; 2]> {
        let mut v = Vec::new();
        for y in 2..l.h - 2 {
            for x in 2..l.w - 2 {
                let i = y * l.w + x;
                for axis in 0..2 {
                    if l.cls[i][axis] == SIL {
                        v.push([pos[i][2 * axis], pos[i][2 * axis + 1]]);
                    }
                }
            }
        }
        v
    }

    #[test]
    fn plane_far_away_has_no_lines_but_a_depth_jump_does() {
        // Ground plane under a pinhole camera: 1/depth is affine in screen space, so the depth
        // ratio between neighbours can be large without any line.
        let inv = |row: f32| 1.0 / (0.002 + 0.0005 * row);
        let col = |r: f32| px(inv(r), UP, 3);
        assert!(col(9.0).depth / col(11.0).depth > 1.15);
        assert_eq!(pair_class(col(9.0), col(10.0), col(11.0), col(12.0)), NONE);
        // Same instance folding behind itself: only the pair across the jump is a silhouette.
        let (near, far) = (px(10.0, UP, 4), px(40.0, UP, 4));
        assert_eq!(pair_class(near, near, far, far), SIL);
        assert_eq!(pair_class(near, near, near, far), NONE);
        assert_eq!(pair_class(near, far, far, far), NONE);
    }

    #[test]
    fn sky_instance_and_crease_classes() {
        let a = px(10.0, UP, 1);
        assert_eq!(pair_class(a, a, MISS, MISS), SIL);
        assert_eq!(pair_class(MISS, MISS, MISS, MISS), NONE);
        // Another instance in contact (no depth jump): a crease-class border line.
        let b = px(10.0, UP, 2);
        assert_eq!(pair_class(a, a, b, b), CREASE);
        let tilt = |deg: f32| {
            let r = deg.to_radians();
            px(10.0, [r.sin(), r.cos(), 0.0], 1)
        };
        assert_eq!(pair_class(a, a, tilt(20.0), tilt(20.0)), NONE);
        assert_eq!(pair_class(a, a, tilt(60.0), tilt(60.0)), CREASE);
    }

    #[test]
    fn stroke_coverage_is_smooth_and_width_constant() {
        let w = WIRE_LINE_PX_DEFAULT;
        assert_eq!(coverage(0.0, w), 1.0);
        assert_eq!(coverage(0.5 * w + 0.5, w), 0.0);
        let mut prev = 1.0;
        for i in 1..=20 {
            let c = coverage(i as f32 * 0.07, w);
            assert!(c <= prev, "coverage must fall off monotonically");
            prev = c;
        }
        let mid = coverage(0.5 * w, w);
        assert!((0.4..0.6).contains(&mid), "half coverage at the stroke edge, got {mid}");
        // A stroke thinner than a pixel fades in opacity instead of aliasing.
        let thin = coverage(0.0, 0.75);
        assert!(thin < 0.75 && thin > 0.6, "thin stroke peak {thin}");
    }

    #[test]
    fn smoothing_turns_the_pixel_staircase_into_the_true_contour() {
        // Object below the line y = 0.37 x + 6.2 (pixel-centre sampling), sky above.
        let (w, h) = (48, 32);
        let line = |x: f32| 0.37 * x + 6.2;
        let g: Vec<Px> = (0..w * h)
            .map(|i| {
                let (x, y) = ((i % w) as f32 + 0.5, (i / w) as f32 + 0.5);
                if y > line(x) { px(10.0, UP, 1) } else { MISS }
            })
            .collect();
        let l = classify(&g, w, h);
        let err = |v: &[[f32; 2]]| {
            v.iter()
                .filter(|p| p[0] > 6.0 && p[0] < w as f32 - 6.0)
                .map(|p| (p[1] - line(p[0])).abs() / (1.0f32 + 0.37 * 0.37).sqrt())
                .fold(0.0f32, f32::max)
        };
        let raw = err(&sil_vertices(&l, &l.pos));
        let smooth = err(&sil_vertices(&l, &smoothed(&l)));
        assert!(raw > 0.3, "the raw staircase deviates by {raw} px");
        assert!(smooth < 0.4 * raw && smooth < 0.15, "smoothed {smooth} px vs raw {raw} px");
    }

    #[test]
    fn smoothing_keeps_sharp_corners() {
        // Axis-aligned square: the vertices next to its corners must not be pulled inwards.
        let (w, h) = (24, 24);
        let g: Vec<Px> = (0..w * h)
            .map(|i| {
                let (x, y) = (i % w, i / w);
                if (6..18).contains(&x) && (6..18).contains(&y) { px(10.0, UP, 1) } else { MISS }
            })
            .collect();
        let l = classify(&g, w, h);
        let before = sil_vertices(&l, &l.pos);
        let after = sil_vertices(&l, &smoothed(&l));
        let moved = before
            .iter()
            .zip(&after)
            .map(|(a, b)| dist(*a, *b))
            .fold(0.0f32, f32::max);
        assert!(moved < 0.05, "straight square sides and corners must stay, moved {moved} px");
    }

    #[test]
    fn launch_aov_follows_the_display_mode() {
        let mut s = crate::renderer::DemoSettings {
            aov_mode: 3,
            ..Default::default()
        };
        assert_eq!(s.launch_aov_mode(), 3);
        s.display_mode = DisplayMode::Wireframe;
        assert_eq!(s.launch_aov_mode(), AOV_WIREFRAME);
        assert_eq!(DisplayMode::default(), DisplayMode::Csg);
        assert_eq!(s.wire_line_px, WIRE_LINE_PX_DEFAULT);
    }
}
