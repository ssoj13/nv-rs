//! # viewport — viewport coordinate spaces and pick rays (single source of truth)
//!
//! | Space | Origin | Y axis | Typical use |
//! |-------|--------|--------|-------------|
//! | **Screen** (egui) | top-left of the window | down | `interact_pointer_pos` |
//! | **Viewport pixels** | top-left of the 3D widget | down | UI events, pick input |
//! | **NDC** (WebGPU clip) | center of the viewport | **up** | unproject, clip-space rays |
//!
//! Used by the controller (framing, projection) and by hosts for picking.
//!
//! # Pitfalls when unprojecting
//!
//! `inv_view_proj = (P * V).inverse()` is **not affine** — the bottom row of the
//! inverse of a perspective matrix has non-trivial entries, so the implicit
//! `w = 1` shortcut in [`glam::Mat4::transform_point3`] gives wrong results
//! (every pixel collapses to a ray along camera `forward`, dragging stops
//! tracking the cursor, picks become rank-deficient).
//!
//! When unprojecting an NDC point through `inv_view_proj`:
//! - **CPU (Rust):** use [`glam::Mat4::project_point3`] — it performs the
//!   perspective divide. [`ViewportRay::from_inv_view_proj`] is the canonical
//!   path; route every CPU-side viewport raycast through it instead of
//!   re-implementing the matrix math at the call-site.
//! - **GPU (WGSL):** apply `inv_view_proj` to `vec4f(ndc, depth, 1.0)` and
//!   then divide `xyz` by `w` manually; with this crate's reverse-Z, unproject
//!   depth 1 (near) and 0 (far) and take `far - near` as the ray direction,
//!   exactly as [`ViewportRay::from_inv_view_proj`] does on the CPU.
//!
//! Anything that needs world-space camera rays must take this route; any new
//! `transform_point3` call on a non-affine matrix is almost certainly a bug.

/// Viewport extent in pixels (minimum 1×1). The fields are public, so a literal can hold a
/// zero; the float accessors ([`ViewportSize::width_f`], [`ViewportSize::height_f`],
/// [`ViewportSize::aspect`]) read a zero as 1, so it never reaches a division.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewportSize {
    /// Width in pixels ([`ViewportSize::new`] keeps it at least 1).
    pub width: u32,
    /// Height in pixels ([`ViewportSize::new`] keeps it at least 1).
    pub height: u32,
}

impl ViewportSize {
    /// Size with both extents raised to at least 1.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
        }
    }

    /// Width as `f32` (at least 1).
    pub fn width_f(self) -> f32 {
        self.width.max(1) as f32
    }

    /// Height as `f32` (at least 1).
    pub fn height_f(self) -> f32 {
        self.height.max(1) as f32
    }

    /// Width / height (finite and positive, also for a zero literal).
    pub fn aspect(self) -> f32 {
        self.width_f() / self.height_f()
    }
}

/// Pointer position inside the viewport widget (top-left origin, Y down).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewportPoint {
    /// Horizontal pixel offset from the left edge.
    pub x: f32,
    /// Vertical pixel offset from the top edge.
    pub y: f32,
}

impl ViewportPoint {
    /// The top-left corner.
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    /// Point clamped into `[0, size]` on both axes (`f32::max`/`min` ignore a NaN input side).
    pub fn new_clamped(x: f32, y: f32, size: ViewportSize) -> Self {
        Self {
            x: x.max(0.0).min(size.width_f()),
            y: y.max(0.0).min(size.height_f()),
        }
    }

    /// Screen-space point relative to the widget rect origin, clamped into the viewport.
    pub fn from_screen(
        screen_x: f32,
        screen_y: f32,
        rect_min_x: f32,
        rect_min_y: f32,
        size: ViewportSize,
    ) -> Self {
        Self::new_clamped(screen_x - rect_min_x, screen_y - rect_min_y, size)
    }

    /// True when the raw position lies outside `[0, size]` (so clamping changed it).
    pub fn needed_clamp(raw_x: f32, raw_y: f32, size: ViewportSize) -> bool {
        raw_x < 0.0 || raw_y < 0.0 || raw_x > size.width_f() || raw_y > size.height_f()
    }

    /// This point in NDC.
    pub fn to_ndc(self, size: ViewportSize) -> NdcPoint {
        NdcPoint::from_viewport(self, size)
    }

    /// This point as a texture UV.
    pub fn to_texture_uv(self, size: ViewportSize) -> TextureUv {
        TextureUv::from_viewport(self, size)
    }
}

/// Normalized device coordinates for WebGPU (`clip_space`, Y up).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NdcPoint {
    /// NDC x in `[-1, 1]`, left to right.
    pub x: f32,
    /// NDC y in `[-1, 1]`, bottom to top.
    pub y: f32,
}

impl NdcPoint {
    /// Convert a viewport pixel to NDC (Y flipped).
    pub fn from_viewport(p: ViewportPoint, size: ViewportSize) -> Self {
        let w = size.width_f();
        let h = size.height_f();
        Self {
            x: (p.x / w) * 2.0 - 1.0,
            y: 1.0 - (p.y / h) * 2.0,
        }
    }

    /// `[x, y]`.
    pub fn as_array(self) -> [f32; 2] {
        [self.x, self.y]
    }

    /// Convert back to a viewport pixel (Y flipped).
    pub fn to_viewport(self, size: ViewportSize) -> ViewportPoint {
        let w = size.width_f();
        let h = size.height_f();
        ViewportPoint {
            x: ((self.x + 1.0) * 0.5) * w,
            y: (1.0 - self.y) * 0.5 * h,
        }
    }
}

/// Texture / framebuffer UV (0..1), top-left origin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextureUv {
    /// Horizontal coordinate, 0 at the left edge.
    pub u: f32,
    /// Vertical coordinate, 0 at the top edge.
    pub v: f32,
}

impl TextureUv {
    /// UV of a viewport pixel.
    pub fn from_viewport(p: ViewportPoint, size: ViewportSize) -> Self {
        Self {
            u: p.x / size.width_f(),
            v: p.y / size.height_f(),
        }
    }
}

/// Screen-space axis-aligned rectangle (egui `Rect` fields).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScreenRect {
    /// Left edge in screen space.
    pub min_x: f32,
    /// Top edge in screen space.
    pub min_y: f32,
    /// Width in pixels.
    pub width: f32,
    /// Height in pixels.
    pub height: f32,
}

impl ScreenRect {
    /// Integer viewport extent of this rect (at least 1×1).
    pub fn viewport_size(self) -> ViewportSize {
        ViewportSize::new(self.width.max(1.0) as u32, self.height.max(1.0) as u32)
    }

    /// True when the screen position lies inside (edges inclusive).
    pub fn contains_screen(self, screen_x: f32, screen_y: f32) -> bool {
        screen_x >= self.min_x
            && screen_y >= self.min_y
            && screen_x <= self.min_x + self.width
            && screen_y <= self.min_y + self.height
    }

    /// Screen position as a clamped viewport point.
    pub fn screen_to_viewport(self, screen_x: f32, screen_y: f32) -> ViewportPoint {
        ViewportPoint::from_screen(
            screen_x,
            screen_y,
            self.min_x,
            self.min_y,
            self.viewport_size(),
        )
    }
}

/// World-space ray from a viewport pixel and inverse view-projection (RH, WebGPU depth).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewportRay {
    /// Ray origin (the camera eye).
    pub origin: [f32; 3],
    /// Unit direction into the scene.
    pub dir: [f32; 3],
}

impl ViewportRay {
    /// Build a world-space ray from a viewport pixel and the camera's inverse
    /// view-projection.
    ///
    /// **Why `project_point3` and not `transform_point3`:** `inv_view_proj` is
    /// *not* affine — the inverse of a perspective matrix has a non-trivial
    /// bottom row, so the implicit `w = 1` shortcut in `transform_point3`
    /// produces a constant-direction ray (all NDCs collapse to camera
    /// `forward`). `project_point3` performs the perspective divide that the
    /// inverse projection needs, so off-centre pixels tilt the ray correctly.
    /// See glam docs for `Mat4::transform_point3` (“assumes affine”) vs
    /// `Mat4::project_point3` (“applies perspective correction”).
    ///
    /// A degenerate or non-finite matrix yields the fallback direction `-Z` instead of NaN.
    pub fn from_inv_view_proj(
        point: ViewportPoint,
        size: ViewportSize,
        inv_view_proj_cols: [[f32; 4]; 4],
        eye: [f32; 3],
    ) -> Self {
        use glam::{Mat4, Vec3};
        let ndc = point.to_ndc(size);
        let inv = Mat4::from_cols_array_2d(&inv_view_proj_cols);
        // Reverse-Z: the near plane is at NDC depth 1, the far plane at 0 (the
        // projection swaps near/far). Unproject both so `dir` still points INTO
        // the scene (far − near); getting these backwards reverses the pick ray.
        let near = inv.project_point3(Vec3::new(ndc.x, ndc.y, 1.0));
        let far = inv.project_point3(Vec3::new(ndc.x, ndc.y, 0.0));
        let dir = (far - near).try_normalize().unwrap_or(Vec3::NEG_Z);
        Self {
            origin: eye,
            dir: dir.to_array(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_corners_to_ndc() {
        let size = ViewportSize::new(800, 600);
        let tl = ViewportPoint { x: 0.0, y: 0.0 }.to_ndc(size);
        assert!((tl.x + 1.0).abs() < 1e-5);
        assert!((tl.y - 1.0).abs() < 1e-5);

        let br = ViewportPoint { x: 800.0, y: 600.0 }.to_ndc(size);
        assert!((br.x - 1.0).abs() < 1e-5);
        assert!((br.y + 1.0).abs() < 1e-5);

        let c = ViewportPoint { x: 400.0, y: 300.0 }.to_ndc(size);
        assert!(c.x.abs() < 1e-5);
        assert!(c.y.abs() < 1e-5);
    }

    #[test]
    fn ndc_round_trip() {
        let size = ViewportSize::new(756, 680);
        let p = ViewportPoint { x: 320.5, y: 210.0 };
        let back = p.to_ndc(size).to_viewport(size);
        assert!((back.x - p.x).abs() < 0.01);
        assert!((back.y - p.y).abs() < 0.01);
    }

    #[test]
    fn clamp_rejects_oob_screen_style() {
        let size = ViewportSize::new(756, 680);
        let p = ViewportPoint::new_clamped(840.0, 318.0, size);
        assert!((p.x - 756.0).abs() < 1e-3);
        assert!((p.y - 318.0).abs() < 1e-3);
        assert!(ViewportPoint::needed_clamp(840.0, 318.0, size));
    }

    #[test]
    fn degenerate_matrix_gives_fallback_ray() {
        let size = ViewportSize::new(100, 100);
        let ray = ViewportRay::from_inv_view_proj(
            ViewportPoint { x: 50.0, y: 50.0 },
            size,
            [[0.0; 4]; 4],
            [1.0, 2.0, 3.0],
        );
        assert_eq!(ray.dir, [0.0, 0.0, -1.0]);
        assert_eq!(ray.origin, [1.0, 2.0, 3.0]);
    }

    /// A zero-size literal (bypassing `new`) never divides by zero.
    #[test]
    fn zero_size_literal_stays_finite() {
        let size = ViewportSize {
            width: 0,
            height: 0,
        };
        assert_eq!((size.width_f(), size.height_f()), (1.0, 1.0));
        assert_eq!(size.aspect(), 1.0);
        let p = ViewportPoint { x: 0.5, y: 0.5 };
        let ndc = p.to_ndc(size);
        let uv = p.to_texture_uv(size);
        assert!(ndc.x.is_finite() && ndc.y.is_finite() && uv.u.is_finite() && uv.v.is_finite());
    }

    #[test]
    fn texture_uv_of_centre() {
        let size = ViewportSize::new(200, 100);
        let uv = ViewportPoint { x: 100.0, y: 50.0 }.to_texture_uv(size);
        assert!((uv.u - 0.5).abs() < 1e-6 && (uv.v - 0.5).abs() < 1e-6);
    }
}
