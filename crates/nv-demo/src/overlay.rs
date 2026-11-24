//! Generic overlay line renderer for nv-demo: one device-resident primitive list, one GPU
//! antialiased raster pass, one shared filter, many providers.
//!
//! - **What**: overlays (wireframe contours today; object axes, actor skeletons, rail path,
//!   bounds and camera frustum in later waves) are line segments and point markers with a
//!   constant screen-space width, colour, dash pattern and depth mode, drawn over the finished
//!   frame. Every frame the segment list is rebuilt on the device; nothing is read back.
//! - **Why**: one renderer and one filter instead of one-off drawing code per feature, and no
//!   per-frame host copies of scene data (operator rule: overlays run GPU-first and must not slow
//!   the frame).
//! - **How**:
//!   ```text
//!   providers ── emit(&OverlayCtx, &mut OverlayList) ──> OverlayList (host, reused)
//!      | CPU primitives (small sets: rail, gizmos)        | GPU sources (contours, ...)
//!      v flatten(): project + filter -> records             v device kernels append records
//!   one async upload (pinned staging) ──> ovl_segs[0..n_cpu] ── atomic counter ──> ovl_segs[n_cpu..]
//!                                         ovl_splat (one block per segment: analytic AA,
//!                                         dash, optional depth test vs. primary depth)
//!                                         -> ovl_cov (per pixel: max coverage | style)
//!                                         ovl_composite (linear-light blend into the tonemapped
//!                                         RGBA8 and scRGB outputs, so captures include it)
//!   ```
//!   Kernels live in `shaders/tonemap.cu` (overlay section); the host side of the pass is
//!   `renderer::run_overlay`. Device buffers are allocated once and grown on demand.
//! - **Filter**: [`OverlayFilter`] (persisted in `DemoSettings::overlay`, icon toggles in the
//!   viewport toolbar): master switch, kinds, scope (all / selected object / within a radius of
//!   the camera), screen-size cull, and whether captures include the overlay.
//! - **Projections**: world-space CPU primitives are projected with the pinhole camera; with
//!   another lens they are not drawn and [`OverlayList::status`] says so (no silently wrong
//!   lines). Screen-space primitives and GPU sources work with every lens.
//! - **Picking** (later wave): the mouse pixel goes up with the launch parameters and the
//!   primary launch writes a 32 B `{inst_id, part_id, t, world_pos}` record that is read back
//!   asynchronously; the overlay's selected scope will key on that id.

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::renderer::{CameraFrame, CameraProjection};

/// The CUDA side of the segment contract (`shaders/overlay_segment.cuh`): record layout, class
/// and flag values, counter words and the append helper. External GPU providers (csg-rs feature
/// curves) compile against this exact text.
#[cfg_attr(not(test), expect(dead_code, reason = "device record contract shared with external GPU producers (csg-rs feature curves) and the CUDA kernels; Rust code constructs only part of it, the contract test covers all of it"))]
pub const OVERLAY_SEGMENT_CUH: &str = include_str!("../shaders/overlay_segment.cuh");

/// What a segment means (`NV_OVERLAY_CLASS_*`): styling and filtering key on it, and external
/// providers state it so nv-demo can tell their curves apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
#[cfg_attr(not(test), expect(dead_code, reason = "device record contract shared with external GPU producers (csg-rs feature curves) and the CUDA kernels; Rust code constructs only part of it, the contract test covers all of it"))]
pub enum SegmentClass {
    /// Sharp edge between two faces of one object.
    Crease = 0,
    /// Open boundary of a surface.
    Boundary = 1,
    /// Silhouette generator (exact, view dependent, from geometry).
    SilhouetteGenerator = 2,
    /// Intersection curve of a CSG boolean.
    BooleanIntersection = 3,
    /// Screen-space contour from the wireframe G-buffer (silhouette / depth jump).
    Contour = 4,
    /// Object axis gizmo.
    Axis = 5,
    /// Actor skeleton bone.
    Bone = 6,
    /// Path (camera rail, trajectories).
    Path = 7,
}

/// How the draw pass interprets a record (`NV_OVERLAY_KIND_*`). Straight kinds today; analytic
/// curve kinds (arc, ellipse, conic, evaluated by exact per-pixel distance) are added here when
/// csg-scene publishes its curve record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
#[cfg_attr(not(test), expect(dead_code, reason = "device record contract shared with external GPU producers (csg-rs feature curves) and the CUDA kernels; Rust code constructs only part of it, the contract test covers all of it"))]
pub enum RecordKind {
    /// Independent straight segment `a`-`b`.
    Segment = 0,
    /// One segment of a polyline (dash phase continues through `s0`).
    PolylineChunk = 1,
}

/// Flag bits of [`OverlaySegmentPod::flags`] (`NV_OVERLAY_FLAG_*`).
#[cfg_attr(not(test), expect(dead_code, reason = "device record contract shared with external GPU producers (csg-rs feature curves) and the CUDA kernels; Rust code constructs only part of it, the contract test covers all of it"))]
pub mod seg_flags {
    /// `a` / `b` are world metres, projected on the device with the frame camera.
    pub const WORLD: u32 = 1;
    /// The producer occlusion-tested the record hidden: drawn faded and dashed.
    pub const HIDDEN: u32 = 2;
    /// Dashed regardless of the style.
    pub const DASHED: u32 = 4;
    /// Part of the selection: drawn wider and never LOD-faded.
    pub const SELECTED: u32 = 8;
    /// Never tested against the primary-hit depth (annotations, contours on the surface).
    pub const NO_DEPTH_TEST: u32 = 16;
    /// The style width is in screen pixels; otherwise in metres, projected with the depth.
    pub const SCREEN_CONSTANT_WIDTH: u32 = 32;
}

/// Words of the device counter: `[appended, dropped at capacity, unprojectable, unsupported kind]`.
pub const COUNTER_WORDS: usize = 4;

/// A typed overlay fault counted on the device (`counter[1..4]`) and reported in the status
/// line whenever it changes; never silently ignored.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayFault {
    /// Records beyond the buffer capacity were dropped.
    Overflow(u32),
    /// World-space records could not be projected with the active lens.
    Unprojectable(u32),
    /// Records of a kind this build cannot draw.
    UnsupportedKind(u32),
}

/// One device record, the stable 64-byte layout every provider writes (CPU providers through
/// the per-frame upload, GPU providers with `nv_overlay_push`). Layout = `NvOverlaySegment` in
/// `shaders/overlay_segment.cuh` (size and offsets asserted on both sides). Interim: replaced by
/// csg-scene's `OverlayCurvePod` once published.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct OverlaySegmentPod {
    /// Start point: world metres ([`seg_flags::WORLD`]) or screen `(x px, y px, planar view
    /// depth)`; a depth `<= 0` disables the depth test.
    pub a: [f32; 3],
    /// [`seg_flags`] bits.
    pub flags: u32,
    /// End point, same space as `a`.
    pub b: [f32; 3],
    /// [`RecordKind`] as `u32`: selects the distance function in the draw pass.
    pub kind: u32,
    /// [`SegmentClass`] as `u32`.
    pub cls: u32,
    /// Instance key (primary-hit instance id space), `u32::MAX` = none; the Selected scope
    /// filters on it.
    pub inst_id: u32,
    /// Part within the instance, `u32::MAX` = none.
    pub part_id: u32,
    /// Index into this frame's style table (colour, width, dash, depth mode).
    pub style: u32,
    /// Screen arc length at `a`, so dashes run continuously along a polyline.
    pub s0: f32,
    /// Metres subtracted from the record depth before the depth test (lines on surfaces).
    pub depth_bias: f32,
    /// Projected size in pixels of the feature this record belongs to, for the LOD fade
    /// (0 = unknown: never faded).
    pub proj_size_px: f32,
    /// Keeps the record at 64 bytes; write 0.
    pub reserved: u32,
}
const _: () = assert!(std::mem::size_of::<OverlaySegmentPod>() == 64);
const _: () = assert!(std::mem::offset_of!(OverlaySegmentPod, flags) == 12);
const _: () = assert!(std::mem::offset_of!(OverlaySegmentPod, b) == 16);
const _: () = assert!(std::mem::offset_of!(OverlaySegmentPod, kind) == 28);
const _: () = assert!(std::mem::offset_of!(OverlaySegmentPod, cls) == 32);
const _: () = assert!(std::mem::offset_of!(OverlaySegmentPod, inst_id) == 36);
const _: () = assert!(std::mem::offset_of!(OverlaySegmentPod, part_id) == 40);
const _: () = assert!(std::mem::offset_of!(OverlaySegmentPod, style) == 44);
const _: () = assert!(std::mem::offset_of!(OverlaySegmentPod, s0) == 48);
const _: () = assert!(std::mem::offset_of!(OverlaySegmentPod, depth_bias) == 52);
const _: () = assert!(std::mem::offset_of!(OverlaySegmentPod, proj_size_px) == 56);

/// Frame camera and device-side filter for `ovl_splat` (`OvlCamera` in tonemap.cu): world
/// segments are projected on the device, so GPU providers never need the camera themselves.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct GpuOverlayCamera {
    /// Camera position (world).
    pub eye: [f32; 3],
    /// `tan(fov_y / 2)`.
    pub tan_half_fov: f32,
    /// Forward axis.
    pub fwd: [f32; 3],
    /// Width / height.
    pub aspect: f32,
    /// Right axis.
    pub right: [f32; 3],
    /// 1 when the pinhole lens is active (world projection supported), else 0.
    pub pinhole: u32,
    /// Up axis.
    pub up: [f32; 3],
    /// Instance key of the Selected scope, `u32::MAX` = no filter.
    pub selected: u32,
    /// WithinRadius scope distance in metres (`<= 0` = off).
    pub radius: f32,
    /// Line LOD threshold ([`OverlayFilter::min_screen_px`]).
    pub min_px: f32,
    /// Keeps the record at 80 bytes (16-byte multiple).
    pub pad: [f32; 2],
}
const _: () = assert!(std::mem::size_of::<GpuOverlayCamera>() == 80);

/// One device style (`OvlStyle` in tonemap.cu).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct GpuStyle {
    /// Linear-light RGBA (alpha scales coverage); linear because the composite blends in
    /// linear light.
    pub color: [f32; 4],
    /// Stroke width in screen pixels, constant at any distance (an overlay reads as a drawing).
    pub width_px: f32,
    /// Dash length in pixels; 0 = solid.
    pub dash_on_px: f32,
    /// Gap length in pixels.
    pub dash_off_px: f32,
    /// [`DepthMode`] as `u32` (`OVL_DEPTH_*`).
    pub depth_mode: u32,
}
const _: () = assert!(std::mem::size_of::<GpuStyle>() == 32);

/// Largest style table (device buffer size, fixed).
pub const MAX_STYLES: usize = 64;
/// Device segment capacity per pixel of the frame (grown with the frame size; the counter
/// reports dropped segments beyond it).
pub const SEGS_PER_PIXEL: f32 = 0.5;

/// Overlay families, one bit each in [`OverlayFilter::kinds`]. Later providers add variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OverlayKind {
    /// Wireframe contour lines (silhouettes, creases, instance borders).
    Contours,
}

impl OverlayKind {
    /// Every kind, in toolbar order (one toggle each).
    pub const ALL: [Self; 1] = [Self::Contours];

    /// Bit of this kind in [`OverlayFilter::kinds`] (a mask persists compactly and new kinds
    /// default to off in old configs).
    pub const fn bit(self) -> u32 {
        match self {
            Self::Contours => 1,
        }
    }

    /// Toolbar tooltip.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Contours => "Contours",
        }
    }
}

/// Which objects' overlays are drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OverlayScope {
    /// Everything in view.
    #[default]
    All,
    /// Only the selected (today: possessed) object.
    Selected,
    /// Only primitives within [`OverlayFilter::radius_m`] of the camera.
    WithinRadius,
}

impl OverlayScope {
    /// Every scope, in toolbar order.
    pub const ALL: [Self; 3] = [Self::All, Self::Selected, Self::WithinRadius];

    /// Toolbar tooltip.
    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All objects",
            Self::Selected => "Selected object",
            Self::WithinRadius => "Within radius",
        }
    }
}

/// The shared overlay visibility state (persisted with the settings).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OverlayFilter {
    /// Master switch, so every overlay can be hidden at once.
    pub enabled: bool,
    /// Captures (PNG / JPEG / MP4) include the overlay.
    pub in_captures: bool,
    /// Enabled [`OverlayKind`] bits.
    pub kinds: u32,
    /// Which objects' overlays are drawn (declutters busy scenes).
    pub scope: OverlayScope,
    /// Radius for [`OverlayScope::WithinRadius`], metres.
    pub radius_m: f32,
    /// Screen-size cull and line LOD: CPU polylines smaller than this many pixels are culled,
    /// and contour lines fade in between this projected object size and three times it (far
    /// features too small to resolve would otherwise shimmer).
    pub min_screen_px: f32,
    /// Temporal stabilisation of the line coverage (motion-reprojected history, clamped and
    /// blended), so lines do not flicker while the camera moves.
    pub temporal: bool,
}

impl Default for OverlayFilter {
    fn default() -> Self {
        Self {
            enabled: true,
            in_captures: true,
            kinds: OverlayKind::Contours.bit(),
            scope: OverlayScope::All,
            radius_m: 150.0,
            min_screen_px: 2.0,
            temporal: true,
        }
    }
}

impl OverlayFilter {
    /// Whether providers of `kind` should emit at all this frame (they skip all work if not).
    pub const fn shows(&self, kind: OverlayKind) -> bool {
        self.enabled && (self.kinds & kind.bit()) != 0
    }
}

/// Depth test of a style against the primary-hit depth (`OVL_DEPTH_*` in tonemap.cu).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DepthMode {
    /// No depth test (screen-space annotations, contours).
    #[default]
    Always = 0,
    /// Hidden parts are not drawn.
    #[cfg_attr(not(test), expect(dead_code, reason = "CPU-provider API of the overlay renderer; its first CPU providers (rail path, gizmos) land in the next wave, the tests exercise it now"))]
    SkipHidden = 1,
    /// Hidden parts are drawn faded and dashed.
    #[cfg_attr(not(test), expect(dead_code, reason = "CPU-provider API of the overlay renderer; its first CPU providers (rail path, gizmos) land in the next wave, the tests exercise it now"))]
    FadeHidden = 2,
}

/// Index into [`OverlayList`]'s style table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StyleId(pub u32);

/// Coordinate space of CPU primitives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Space {
    /// Image pixels `(x, y)`; `z` is ignored.
    #[cfg_attr(not(test), expect(dead_code, reason = "CPU-provider API of the overlay renderer; its first CPU providers (rail path, gizmos) land in the next wave, the tests exercise it now"))]
    Screen,
    /// World metres, projected with the current camera.
    World,
}

#[derive(Clone, Copy, Debug)]
struct Polyline {
    class: SegmentClass,
    first: u32,
    count: u32,
    space: Space,
    style: StyleId,
    closed: bool,
    object: Option<u32>,
}

/// GPU-side overlay sources requested by providers (the renderer runs their kernels).
///
/// Why: large overlays (contours, later instance axes and skeletons) are generated where their
/// data lives, on the device, with no host copy of scene data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuSource {
    /// Wireframe contours from the wireframe G-buffer (`wire_emit`).
    Contours { silhouette: StyleId, crease: StyleId },
}

/// Per-frame camera, viewport and filter handed to providers.
#[derive(Clone, Copy, Debug)]
pub struct OverlayCtx {
    /// The camera that rendered this frame (world-space primitives project with it).
    pub camera: CameraFrame,
    /// Active lens; world-space projection supports [`CameraProjection::Pinhole`] only.
    pub projection: CameraProjection,
    /// Frame width in pixels (the overlay runs at the render resolution).
    pub width: u32,
    /// Frame height in pixels.
    pub height: u32,
    /// The shared visibility state every provider honours.
    pub filter: OverlayFilter,
    /// The selected object (today: the possessed instance), for [`OverlayScope::Selected`].
    pub selected: Option<u32>,
    /// A primary-hit depth buffer is available for [`DepthMode`] tests this frame.
    #[cfg_attr(not(test), expect(dead_code, reason = "CPU-provider API of the overlay renderer; its first CPU providers (rail path, gizmos) land in the next wave, the tests exercise it now"))]
    pub depth_available: bool,
}

impl OverlayCtx {
    /// Projects a world point to image pixels and planar view depth (pinhole only).
    pub fn project(&self, p: [f32; 3]) -> Option<[f32; 3]> {
        if self.projection != CameraProjection::Pinhole || self.width == 0 || self.height == 0 {
            return None;
        }
        let c = &self.camera;
        let v = [p[0] - c.eye[0], p[1] - c.eye[1], p[2] - c.eye[2]];
        let dot = |a: [f32; 3]| v[0] * a[0] + v[1] * a[1] + v[2] * a[2];
        let z = dot(c.fwd);
        if z <= 1e-3 || c.tan_half_fov <= 0.0 {
            return None;
        }
        let aspect = self.width as f32 / self.height as f32;
        let xn = dot(c.right) / (z * c.tan_half_fov * aspect);
        let yn = dot(c.up) / (z * c.tan_half_fov);
        Some([
            (xn * 0.5 + 0.5) * self.width as f32,
            (0.5 - 0.5 * yn) * self.height as f32,
            z,
        ])
    }

    fn distance_to_eye(&self, p: [f32; 3]) -> f32 {
        let e = self.camera.eye;
        ((p[0] - e[0]).powi(2) + (p[1] - e[1]).powi(2) + (p[2] - e[2]).powi(2)).sqrt()
    }
}

/// An overlay provider: appends its primitives (or GPU sources) for this frame.
///
/// Why one trait: every overlay feature plugs into the same list, filter and raster pass, so
/// none needs its own drawing code.
pub trait OverlayProvider {
    /// Appends this frame's primitives to `out`; must honour `ctx.filter` and allocate nothing
    /// in steady state (reuse `out`'s buffers).
    fn emit(&mut self, ctx: &OverlayCtx, out: &mut OverlayList);
}

/// The per-frame primitive list. Every buffer is cleared, never freed, so steady state does
/// not allocate.
#[derive(Debug, Default)]
pub struct OverlayList {
    styles: Vec<GpuStyle>,
    verts: Vec<[f32; 3]>,
    lines: Vec<Polyline>,
    gpu: Vec<GpuSource>,
    segs: Vec<OverlaySegmentPod>,
    /// Why some requested primitives are not drawn this frame (shown in the status line).
    pub status: Option<&'static str>,
}

impl OverlayList {
    /// Starts a new frame, keeping every allocation.
    pub fn clear(&mut self) {
        self.styles.clear();
        self.verts.clear();
        self.lines.clear();
        self.gpu.clear();
        self.segs.clear();
        self.status = None;
    }

    /// Registers a style; `None` when the table is full ([`MAX_STYLES`]).
    pub fn style(&mut self, color: [f32; 4], width_px: f32, dash_px: [f32; 2], depth: DepthMode) -> Option<StyleId> {
        if self.styles.len() >= MAX_STYLES {
            self.status = Some("overlay style table full");
            return None;
        }
        self.styles.push(GpuStyle {
            color,
            width_px,
            dash_on_px: dash_px[0],
            dash_off_px: dash_px[1],
            depth_mode: depth as u32,
        });
        Some(StyleId(self.styles.len() as u32 - 1))
    }

    /// Adds a polyline; `object` ties it to an object id for the Selected scope.
    #[cfg_attr(not(test), expect(dead_code, reason = "CPU-provider API of the overlay renderer; its first CPU providers (rail path, gizmos) land in the next wave, the tests exercise it now"))]
    pub fn polyline(
        &mut self,
        class: SegmentClass,
        space: Space,
        pts: &[[f32; 3]],
        closed: bool,
        style: StyleId,
        object: Option<u32>,
    ) {
        if pts.len() < 2 {
            return;
        }
        let first = self.verts.len() as u32;
        self.verts.extend_from_slice(pts);
        self.lines.push(Polyline {
            class,
            first,
            count: pts.len() as u32,
            space,
            style,
            closed,
            object,
        });
    }

    /// Adds a point marker (a round dot of the style's width).
    #[cfg_attr(not(test), expect(dead_code, reason = "CPU-provider API of the overlay renderer; its first CPU providers (rail path, gizmos) land in the next wave, the tests exercise it now"))]
    pub fn point(&mut self, class: SegmentClass, space: Space, p: [f32; 3], style: StyleId, object: Option<u32>) {
        let first = self.verts.len() as u32;
        self.verts.push(p);
        self.lines.push(Polyline {
            class,
            first,
            count: 1,
            space,
            style,
            closed: false,
            object,
        });
    }

    /// Requests a GPU source.
    pub fn gpu_source(&mut self, src: GpuSource) {
        self.gpu.push(src);
    }

    /// This frame's style table (uploaded once per frame).
    pub fn styles(&self) -> &[GpuStyle] {
        &self.styles
    }

    /// GPU sources the renderer runs after uploading the CPU segments.
    pub fn gpu_sources(&self) -> &[GpuSource] {
        &self.gpu
    }

    /// Projects and filters the CPU primitives into device segments (`ovl_segs[0..n]`).
    pub fn flatten(&mut self, ctx: &OverlayCtx) -> &[OverlaySegmentPod] {
        self.segs.clear();
        let f = ctx.filter;
        for li in 0..self.lines.len() {
            let l = self.lines[li];
            if f.scope == OverlayScope::Selected && (l.object.is_none() || l.object != ctx.selected) {
                continue;
            }
            let start = self.segs.len();
            let (mut lo, mut hi) = ([f32::MAX; 2], [f32::MIN; 2]);
            let mut s = 0.0f32;
            let n = l.count as usize + usize::from(l.closed && l.count > 2);
            let mut prev: Option<[f32; 3]> = None;
            let mut dropped = false;
            for k in 0..n {
                let p = self.verts[l.first as usize + k % l.count as usize];
                if l.space == Space::World
                    && f.scope == OverlayScope::WithinRadius
                    && ctx.distance_to_eye(p) > f.radius_m
                {
                    prev = None;
                    continue;
                }
                let q = match l.space {
                    Space::Screen => Some([p[0], p[1], 0.0]),
                    Space::World => ctx.project(p),
                };
                let Some(q) = q else {
                    dropped = true;
                    prev = None;
                    continue;
                };
                lo = [lo[0].min(q[0]), lo[1].min(q[1])];
                hi = [hi[0].max(q[0]), hi[1].max(q[1])];
                if l.count == 1 {
                    self.segs.push(screen_pod(q, q, 0.0, &l));
                } else if let Some(a) = prev {
                    self.segs.push(screen_pod(a, q, s, &l));
                    s += (q[0] - a[0]).hypot(q[1] - a[1]);
                }
                prev = Some(q);
            }
            if dropped && l.space == Space::World && ctx.projection != CameraProjection::Pinhole {
                self.status = Some("world-space overlays need the Pinhole lens; not drawn");
            }
            // Screen-size cull: a polyline smaller than `min_screen_px` is noise (points keep
            // their marker size).
            let diag = (hi[0] - lo[0]).hypot(hi[1] - lo[1]);
            if l.count > 1 && self.segs.len() > start && diag < f.min_screen_px {
                self.segs.truncate(start);
            }
        }
        &self.segs
    }
}

/// A projected CPU segment (screen space, no fade).
fn screen_pod(a: [f32; 3], b: [f32; 3], s0: f32, l: &Polyline) -> OverlaySegmentPod {
    OverlaySegmentPod {
        a,
        flags: seg_flags::SCREEN_CONSTANT_WIDTH,
        b,
        kind: RecordKind::PolylineChunk as u32,
        cls: l.class as u32,
        inst_id: l.object.unwrap_or(u32::MAX),
        part_id: u32::MAX,
        style: l.style.0,
        s0,
        depth_bias: 0.0,
        proj_size_px: 0.0,
        reserved: 0,
    }
}

/// The wireframe contour provider: in Wireframe display mode it requests the GPU contour
/// source (`wire_emit` appends the segments on the device) with a dark silhouette and a lighter,
/// thinner crease style.
#[derive(Debug, Default)]
pub struct ContourProvider {
    /// `DisplayMode::Wireframe` is active this frame (the contour G-buffer exists only then).
    pub wireframe: bool,
    /// Silhouette width in px (`DemoSettings::wire_line_px`).
    pub line_px: f32,
}

impl OverlayProvider for ContourProvider {
    fn emit(&mut self, ctx: &OverlayCtx, out: &mut OverlayList) {
        if !self.wireframe || !ctx.filter.shows(OverlayKind::Contours) {
            return;
        }
        if ctx.filter.scope == OverlayScope::Selected {
            out.status = Some("contours have no per-object selection yet (picking wave); showing all");
        }
        let w = self
            .line_px
            .clamp(crate::wireframe::WIRE_LINE_PX_MIN, crate::wireframe::WIRE_LINE_PX_MAX);
        let crease_w = w * crate::wireframe::WIRE_CREASE_WIDTH_RATIO;
        // Linear-light colours: near-black silhouettes, mid-grey creases.
        let sil = out.style([0.010, 0.010, 0.012, 1.0], w, [0.0, 0.0], DepthMode::Always);
        let crease = out.style([0.06, 0.06, 0.065, 0.85], crease_w, [0.0, 0.0], DepthMode::Always);
        if let (Some(silhouette), Some(crease)) = (sil, crease) {
            out.gpu_source(GpuSource::Contours { silhouette, crease });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(projection: CameraProjection) -> OverlayCtx {
        OverlayCtx {
            camera: CameraFrame {
                eye: [0.0, 0.0, 0.0],
                fwd: [0.0, 0.0, 1.0],
                right: [1.0, 0.0, 0.0],
                up: [0.0, 1.0, 0.0],
                tan_half_fov: 1.0,
            },
            projection,
            width: 200,
            height: 100,
            filter: OverlayFilter::default(),
            selected: None,
            depth_available: true,
        }
    }

    #[test]
    fn pinhole_projection_matches_the_raygen_convention() {
        let c = ctx(CameraProjection::Pinhole);
        // Straight ahead: image centre, depth = distance along fwd.
        assert_eq!(c.project([0.0, 0.0, 5.0]), Some([100.0, 50.0, 5.0]));
        // u = dot(right) / (z tan aspect) = 1 -> right edge; up -> top edge (y = 0).
        assert_eq!(c.project([10.0, 0.0, 5.0]), Some([200.0, 50.0, 5.0]));
        assert_eq!(c.project([0.0, 5.0, 5.0]), Some([100.0, 0.0, 5.0]));
        assert_eq!(c.project([0.0, 0.0, -1.0]), None, "behind the camera");
    }

    #[test]
    fn flatten_splits_polylines_with_arc_length_and_culls() {
        let c = ctx(CameraProjection::Pinhole);
        let mut l = OverlayList::default();
        let st = l.style([1.0; 4], 1.5, [4.0, 2.0], DepthMode::SkipHidden).expect("style");
        l.polyline(SegmentClass::Path, Space::Screen, &[[0.0, 0.0, 0.0], [3.0, 4.0, 0.0], [3.0, 10.0, 0.0]], false, st, None);
        // A 1 px squiggle is culled by `min_screen_px`.
        l.polyline(SegmentClass::Path, Space::Screen, &[[50.0, 50.0, 0.0], [50.5, 50.5, 0.0]], false, st, None);
        let segs = l.flatten(&c).to_vec();
        assert_eq!(segs.len(), 2);
        assert_eq!(segs[0].s0, 0.0);
        assert_eq!(segs[1].s0, 5.0, "arc length continues across segments");
        assert_eq!(segs[1].b, [3.0, 10.0, 0.0]);
    }

    #[test]
    fn world_primitives_need_pinhole_and_respect_radius_and_selection() {
        let mut l = OverlayList::default();
        let st = l.style([1.0; 4], 1.0, [0.0, 0.0], DepthMode::Always).expect("style");
        l.polyline(SegmentClass::Path, Space::World, &[[0.0, 0.0, 5.0], [1.0, 0.0, 5.0]], false, st, Some(7));
        l.polyline(SegmentClass::Path, Space::World, &[[0.0, 0.0, 500.0], [200.0, 0.0, 500.0]], false, st, Some(8));

        let mut fish = ctx(CameraProjection::Fisheye180);
        assert!(l.flatten(&fish).is_empty());
        assert!(l.status.is_some(), "unsupported lens must be reported, not drawn wrong");

        let mut c = ctx(CameraProjection::Pinhole);
        l.status = None;
        assert_eq!(l.flatten(&c).len(), 2);
        c.filter.scope = OverlayScope::WithinRadius;
        c.filter.radius_m = 50.0;
        assert_eq!(l.flatten(&c).len(), 1);
        c.filter.scope = OverlayScope::Selected;
        c.selected = Some(8);
        let segs = l.flatten(&c).to_vec();
        assert_eq!(segs.len(), 1);
        assert!(segs[0].a[2] > 400.0);
        fish.filter.enabled = false;
        assert!(!fish.filter.shows(OverlayKind::Contours));
    }

    #[test]
    fn clearing_keeps_capacity_so_steady_state_does_not_allocate() {
        let c = ctx(CameraProjection::Pinhole);
        let mut l = OverlayList::default();
        let mut caps = None;
        for _ in 0..3 {
            l.clear();
            let st = l.style([1.0; 4], 1.0, [0.0, 0.0], DepthMode::Always).expect("style");
            l.polyline(SegmentClass::Path, Space::Screen, &[[0.0, 0.0, 0.0], [9.0, 0.0, 0.0], [9.0, 9.0, 0.0]], true, st, None);
            l.point(SegmentClass::Path, Space::Screen, [4.0, 4.0, 0.0], st, None);
            assert_eq!(l.flatten(&c).len(), 4, "closed triangle = 3 segments + 1 point");
            let now = (l.styles.capacity(), l.verts.capacity(), l.lines.capacity(), l.segs.capacity());
            if let Some(prev) = caps {
                assert_eq!(prev, now);
            }
            caps = Some(now);
        }
    }

    #[test]
    fn segment_contract_matches_the_cuda_header() {
        let h = OVERLAY_SEGMENT_CUH;
        assert!(h.contains("sizeof(NvOverlaySegment) == 64"));
        for (name, v) in [
            ("NV_OVERLAY_CLASS_CREASE", SegmentClass::Crease as u32),
            ("NV_OVERLAY_CLASS_BOUNDARY", SegmentClass::Boundary as u32),
            ("NV_OVERLAY_CLASS_SILHOUETTE_GENERATOR", SegmentClass::SilhouetteGenerator as u32),
            ("NV_OVERLAY_CLASS_BOOLEAN_INTERSECTION", SegmentClass::BooleanIntersection as u32),
            ("NV_OVERLAY_CLASS_CONTOUR", SegmentClass::Contour as u32),
            ("NV_OVERLAY_CLASS_AXIS", SegmentClass::Axis as u32),
            ("NV_OVERLAY_CLASS_BONE", SegmentClass::Bone as u32),
            ("NV_OVERLAY_CLASS_PATH", SegmentClass::Path as u32),
            ("NV_OVERLAY_FLAG_WORLD", seg_flags::WORLD),
            ("NV_OVERLAY_FLAG_HIDDEN", seg_flags::HIDDEN),
            ("NV_OVERLAY_FLAG_DASHED", seg_flags::DASHED),
            ("NV_OVERLAY_FLAG_SELECTED", seg_flags::SELECTED),
            ("NV_OVERLAY_FLAG_NO_DEPTH_TEST", seg_flags::NO_DEPTH_TEST),
            ("NV_OVERLAY_FLAG_SCREEN_CONSTANT_WIDTH", seg_flags::SCREEN_CONSTANT_WIDTH),
            ("NV_OVERLAY_KIND_SEGMENT", RecordKind::Segment as u32),
            ("NV_OVERLAY_KIND_POLYLINE_CHUNK", RecordKind::PolylineChunk as u32),
            ("NV_OVERLAY_COUNTER_WORDS", COUNTER_WORDS as u32),
        ] {
            assert!(h.contains(&format!("#define {name} {v}u")), "{name} != {v}");
        }
    }

    #[test]
    fn depth_modes_match_the_kernel_constants() {
        // `OVL_DEPTH_ALWAYS` / `_SKIP_HIDDEN` / `_FADE_HIDDEN` in tonemap.cu.
        assert_eq!(
            [DepthMode::Always as u32, DepthMode::SkipHidden as u32, DepthMode::FadeHidden as u32],
            [0, 1, 2]
        );
        assert!(ctx(CameraProjection::Pinhole).depth_available);
    }

    #[test]
    fn contour_provider_requests_the_gpu_source_only_in_wireframe() {
        let c = ctx(CameraProjection::Pinhole);
        let mut l = OverlayList::default();
        let mut p = ContourProvider { wireframe: false, line_px: 1.5 };
        p.emit(&c, &mut l);
        assert!(l.gpu_sources().is_empty());
        p.wireframe = true;
        p.emit(&c, &mut l);
        assert_eq!(l.gpu_sources().len(), 1);
        assert_eq!(l.styles().len(), 2);
        assert!(l.styles()[0].width_px > l.styles()[1].width_px, "silhouettes are wider");
    }
}
