//! Scene-agnostic multi-cassette cinematic camera rail powered by [`csg_scene::CompiledRail`]
//! plus an independent user `Custom` keyframe spline (periodic Centripetal Catmull-Rom +
//! Shoemake SQUAD + inline arc-length LUT), exponential gimbal damper, and 180-degree shutter
//! subframe timing helpers.
//!
//! What: dynamically loads collision-verified [`csg_scene::RailSpec`] cassettes from any active
//! [`csg_scene::SceneModule`] (including on-demand seeded random rails), preserves user-authored
//! `Custom` keyframes independently across scene and cassette switches, provides [`CameraDamper`]
//! for smooth exponential camera glides, and [`shutter_subframe_dt`] for zero-drift 180-degree
//! shutter motion-blur subframe pacing.
//! Why: decouples `nv-demo` from any hardcoded city coordinates while keeping steady-state frame
//! evaluation allocation-free and deterministic.
//! Used by: `crate::main`, `crate::ui`, `crate::persist`.

use std::sync::Arc;

use camera::{CameraPose, FOV_MAX, FOV_MIN};
use csg_scene::{CompiledRail, RailSpec};
use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

/// Clamps a vertical FOV (radians) to the camera controller's lens range
/// ([`camera::FOV_MIN`]..=[`camera::FOV_MAX`]) so rail keyframes keep every reachable framing.
#[inline]
fn clamp_fov(fov_y: f32) -> f32 {
    fov_y.clamp(FOV_MIN, FOV_MAX)
}

/// Fixed sample count for the inline zero-allocation `Custom` arc-length lookup table.
const ARC_LUT_SAMPLES: usize = 256;

/// Sentinel `selected_rail_id` value selecting the user's independent `Custom` keyframe deck.
pub const CUSTOM_RAIL_ID: &str = "custom";

/// Maximum number of runtime-generated random rails kept in the active cassette deck.
const MAX_GENERATED_RAILS: usize = 6;

/// Single camera pose keyframe along the user's `Custom` [`CameraRail`] deck.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CameraKeyframe {
    /// World-space camera position `[x, y, z]`.
    pub pos: [f32; 3],
    /// Unit orientation quaternion `[x, y, z, w]` (camera-to-world).
    pub orientation: [f32; 4],
    /// Vertical field of view in degrees.
    pub vfov_deg: f32,
}

impl CameraKeyframe {
    /// Captures a keyframe from the current [`CameraPose`].
    pub fn from_pose(pose: CameraPose) -> Self {
        let q = if pose.orientation.is_finite() && pose.orientation.length_squared() > 1e-6 {
            pose.orientation.normalize()
        } else {
            Quat::IDENTITY
        };
        Self {
            pos: pose.eye.to_array(),
            orientation: q.to_array(),
            vfov_deg: clamp_fov(pose.fov_y).to_degrees(),
        }
    }

    /// Constructs a keyframe at `eye` aiming at `target` with world `+Y` up.
    #[cfg(test)]
    pub fn from_look_at(eye: [f32; 3], target: [f32; 3], vfov_deg: f32) -> Self {
        let eye_v = Vec3::from_array(eye);
        let tgt_v = Vec3::from_array(target);
        let delta = tgt_v - eye_v;
        let fwd = if delta.length_squared() > 1e-8 {
            delta.normalize()
        } else {
            Vec3::NEG_Z
        };
        let pose = CameraPose::from_eye_forward_up(
            eye_v,
            fwd,
            Vec3::Y,
            clamp_fov(vfov_deg.to_radians()),
            0.05,
            2000.0,
        );
        Self::from_pose(pose)
    }

    /// Converts this keyframe back into a validated [`CameraPose`].
    pub fn to_pose(self, znear: f32, zfar: f32) -> CameraPose {
        let eye = Vec3::from_array(self.pos);
        let q_raw = Quat::from_array(self.orientation);
        let orientation = if q_raw.is_finite() && q_raw.length_squared() > 1e-6 {
            q_raw.normalize()
        } else {
            Quat::IDENTITY
        };
        CameraPose {
            eye: if eye.is_finite() { eye } else { Vec3::ZERO },
            orientation,
            fov_y: clamp_fov(self.vfov_deg.to_radians()),
            znear: znear.max(1e-3),
            zfar: zfar.max(znear + 1.0),
        }
    }

    #[inline]
    fn quat(self) -> Quat {
        let q = Quat::from_array(self.orientation);
        if q.is_finite() && q.length_squared() > 1e-6 {
            q.normalize()
        } else {
            Quat::IDENTITY
        }
    }
}

/// Smoothstep ease-in/out curve `3u^2 - 2u^3` on `[0, 1]`.
#[inline]
pub fn smoothstep01(u: f32) -> f32 {
    let t = if u.is_finite() {
        u.clamp(0.0, 1.0)
    } else {
        0.0
    };
    t * t * (3.0 - 2.0 * t)
}

/// Evaluates a Centripetal Catmull-Rom spline (`alpha = 0.5`) between `p1` and `p2` for `s in [0, 1]`.
fn centripetal_catmull_rom(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, s: f32) -> Vec3 {
    let s = s.clamp(0.0, 1.0);
    let knot = |a: Vec3, b: Vec3| -> f32 { a.distance(b).sqrt().max(1.0e-3) };
    let t0 = 0.0f32;
    let t1 = t0 + knot(p0, p1);
    let t2 = t1 + knot(p1, p2);
    let t3 = t2 + knot(p2, p3);
    let t = t1 + (t2 - t1) * s;

    let lerp_t = |va: Vec3, vb: Vec3, ta: f32, tb: f32| -> Vec3 {
        let denom = tb - ta;
        if denom.abs() < 1.0e-7 {
            va
        } else {
            va * ((tb - t) / denom) + vb * ((t - ta) / denom)
        }
    };

    let a1 = lerp_t(p0, p1, t0, t1);
    let a2 = lerp_t(p1, p2, t1, t2);
    let a3 = lerp_t(p2, p3, t2, t3);
    let b1 = lerp_t(a1, a2, t0, t2);
    let b2 = lerp_t(a2, a3, t1, t3);
    lerp_t(b1, b2, t1, t2)
}

/// Shortest-path quaternion spherical linear interpolation (never takes the >180° long arc).
#[inline]
fn shortest_slerp(a: Quat, b: Quat, t: f32) -> Quat {
    let b_aligned = if a.dot(b) < 0.0 { -b } else { b };
    a.slerp(b_aligned, t.clamp(0.0, 1.0)).normalize()
}

/// Logarithm of a unit quaternion (`0.5 * angle * axis`) for Shoemake SQUAD tangent calculation.
#[inline]
fn quat_log_vec(q: Quat) -> Vec3 {
    let q = if q.w < 0.0 { -q } else { q };
    let v = Vec3::new(q.x, q.y, q.z);
    let sin_half = v.length();
    if sin_half < 1.0e-6 {
        Vec3::ZERO
    } else {
        let half_angle = sin_half.atan2(q.w.clamp(-1.0, 1.0));
        v * (half_angle / sin_half)
    }
}

/// Exponential of a pure-imaginary quaternion vector `v` (`exp(0, v)`).
#[inline]
fn quat_exp_vec(v: Vec3) -> Quat {
    let theta = v.length();
    if theta < 1.0e-6 {
        Quat::IDENTITY
    } else {
        let (s, c) = theta.sin_cos();
        let axis = v / theta;
        Quat::from_xyzw(axis.x * s, axis.y * s, axis.z * s, c).normalize()
    }
}

/// Shoemake inner quadrangle control point `s_i` for C1-continuous spherical cubic interpolation.
#[inline]
fn squad_tangent(q_prev: Quat, q_cur: Quat, q_next: Quat) -> Quat {
    let q_prev = if q_cur.dot(q_prev) < 0.0 { -q_prev } else { q_prev };
    let q_next = if q_cur.dot(q_next) < 0.0 { -q_next } else { q_next };
    let inv = q_cur.conjugate();
    let log_next = quat_log_vec(inv * q_next);
    let log_prev = quat_log_vec(inv * q_prev);
    (q_cur * quat_exp_vec(-0.25 * (log_next + log_prev))).normalize()
}

/// C1-continuous spherical cubic interpolation (SQUAD) across `(q0, q1, q2, q3)` for `s in [0, 1]`.
fn squad_interpolate(q0: Quat, q1: Quat, q2: Quat, q3: Quat, s: f32) -> Quat {
    let s = s.clamp(0.0, 1.0);
    let q1 = q1.normalize();
    let q0 = if q1.dot(q0) < 0.0 { -q0 } else { q0 };
    let q2 = if q1.dot(q2) < 0.0 { -q2 } else { q2 };
    let q3 = if q2.dot(q3) < 0.0 { -q3 } else { q3 };
    let s1 = squad_tangent(q0, q1, q2);
    let s2 = squad_tangent(q1, q2, q3);
    let slerp_12 = shortest_slerp(q1, q2, s);
    let slerp_s = shortest_slerp(s1, s2, s);
    shortest_slerp(slerp_12, slerp_s, 2.0 * s * (1.0 - s))
}

/// Resolved spline segment indices and local parameter `local_s in [0, 1]`.
struct SplineSeg {
    seg_idx: usize,
    seg_count: usize,
    i0: usize,
    i1: usize,
    i2: usize,
    i3: usize,
    local_s: f32,
}

#[inline]
fn spline_segment(n: usize, u: f32, looping: bool) -> SplineSeg {
    let seg_count = if looping { n } else { n - 1 };
    let u_wrapped = if looping {
        u.rem_euclid(1.0)
    } else {
        u.clamp(0.0, 1.0)
    };
    let scaled = u_wrapped * (seg_count as f32);
    let seg_idx = (scaled.floor() as usize).min(seg_count - 1);
    let local_s = (scaled - seg_idx as f32).clamp(0.0, 1.0);
    let (i0, i1, i2, i3) = if looping {
        (
            (seg_idx + n - 1) % n,
            seg_idx % n,
            (seg_idx + 1) % n,
            (seg_idx + 2) % n,
        )
    } else {
        (
            seg_idx.saturating_sub(1),
            seg_idx,
            (seg_idx + 1).min(n - 1),
            (seg_idx + 2).min(n - 1),
        )
    };
    SplineSeg {
        seg_idx,
        seg_count,
        i0,
        i1,
        i2,
        i3,
        local_s,
    }
}

/// Evaluates raw (pre-arc-length) eye position on `Custom` keyframes.
fn eval_custom_eye_raw(kfs: &[CameraKeyframe], u: f32, looping: bool) -> Vec3 {
    let n = kfs.len();
    if n == 0 {
        return Vec3::ZERO;
    }
    if n == 1 {
        return Vec3::from_array(kfs[0].pos);
    }
    let seg = spline_segment(n, u, looping);
    let p1 = Vec3::from_array(kfs[seg.i1].pos);
    let p2 = Vec3::from_array(kfs[seg.i2].pos);
    let p0 = if !looping && seg.seg_idx == 0 {
        p1 * 2.0 - p2
    } else {
        Vec3::from_array(kfs[seg.i0].pos)
    };
    let p3 = if !looping && seg.seg_idx + 1 == seg.seg_count {
        p2 * 2.0 - p1
    } else {
        Vec3::from_array(kfs[seg.i3].pos)
    };
    centripetal_catmull_rom(p0, p1, p2, p3, seg.local_s)
}

/// Compact inline fixed-size arc-length lookup table for `Custom` rails (zero heap allocations).
#[derive(Clone, Copy, Debug)]
struct CustomArcLut {
    signature: u64,
    cum_len: [f32; ARC_LUT_SAMPLES + 1],
    total_len: f32,
}

impl Default for CustomArcLut {
    fn default() -> Self {
        Self {
            signature: 0,
            cum_len: [0.0; ARC_LUT_SAMPLES + 1],
            total_len: 0.0,
        }
    }
}

impl CustomArcLut {
    fn build(signature: u64, kfs: &[CameraKeyframe], looping: bool) -> Self {
        let mut cum_len = [0.0f32; ARC_LUT_SAMPLES + 1];
        let mut prev = eval_custom_eye_raw(kfs, 0.0, looping);
        let mut acc = 0.0f32;
        let inv_n = 1.0 / (ARC_LUT_SAMPLES as f32);
        for (i, slot) in cum_len.iter_mut().enumerate().skip(1) {
            let u = (i as f32) * inv_n;
            let cur = eval_custom_eye_raw(kfs, u, looping);
            acc += prev.distance(cur);
            *slot = acc;
            prev = cur;
        }
        Self {
            signature,
            cum_len,
            total_len: acc,
        }
    }

    fn u_for_fraction(&self, frac: f32) -> f32 {
        let f = frac.clamp(0.0, 1.0);
        if self.total_len <= 1.0e-5 {
            return f;
        }
        if f <= 0.0 {
            return 0.0;
        }
        if f >= 1.0 {
            return 1.0;
        }
        let target_len = f * self.total_len;
        let idx = self
            .cum_len
            .partition_point(|&len| len < target_len)
            .clamp(1, ARC_LUT_SAMPLES);
        let l0 = self.cum_len[idx - 1];
        let l1 = self.cum_len[idx];
        let seg_t = if (l1 - l0).abs() > 1.0e-7 {
            ((target_len - l0) / (l1 - l0)).clamp(0.0, 1.0)
        } else {
            0.0
        };
        (((idx - 1) as f32) + seg_t) / (ARC_LUT_SAMPLES as f32)
    }
}

fn custom_lut_signature(looping: bool, keyframes: &[CameraKeyframe]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mix = |h: &mut u64, v: u64| {
        *h ^= v;
        *h = h.wrapping_mul(0x0100_0000_01b3);
    };
    mix(&mut h, u64::from(looping));
    mix(&mut h, keyframes.len() as u64);
    for kf in keyframes {
        for &c in &kf.pos {
            mix(&mut h, u64::from(c.to_bits()));
        }
    }
    h.max(1)
}

/// Runtime-only scene rail cache. `rails` is shared so the periodic `PersistedState` clone of
/// [`CameraRail`] copies a pointer, not every compiled cassette.
#[derive(Clone, Debug, Default)]
struct RuntimeSceneRails {
    rails: Arc<Vec<CompiledRail>>,
    base_count: usize,
    active_idx: usize,
}

/// Multi-cassette serializable camera rail supporting dynamic [`csg_scene::SceneModule`] cassettes,
/// on-demand exact-CSG random rails, and an independent user `Custom` keyframe spline.
///
/// Equality compares only the persisted fields (see the `PartialEq` impl), so `PersistedState`
/// dirty tracking ignores playback state and runtime caches.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct CameraRail {
    /// Selected rail identifier (e.g. `"road_cruise"`, `"grand_tour"`, or `"custom"`).
    /// Empty string auto-selects the active scene's first available cassette.
    pub selected_rail_id: String,
    /// User-authored `Custom` camera keyframes (never wiped when switching scenes or cassettes).
    pub keyframes: Vec<CameraKeyframe>,
    /// Total duration of one full rail pass in seconds.
    pub duration_s: f32,
    /// Loop continuously with C1 periodic wrap when reaching the end of the rail.
    pub looping: bool,
    /// Apply smoothstep ease-in / ease-out over the rail parameter.
    pub ease_io: bool,
    /// Runtime Auto-Drive ON/OFF state (not persisted as auto-playing across restarts).
    #[serde(skip)]
    pub playing: bool,
    /// Current playback cursor in `[0, duration_s]`.
    #[serde(skip)]
    pub time_s: f32,
    /// Runtime compiled cassettes loaded from the active `SceneModule` + generated random rails.
    #[serde(skip)]
    scene: RuntimeSceneRails,
    /// Inline fixed-size arc-length lookup table for the `Custom` keyframe deck.
    #[serde(skip)]
    custom_lut: CustomArcLut,
}

impl Default for CameraRail {
    fn default() -> Self {
        Self {
            selected_rail_id: String::new(),
            keyframes: Vec::new(),
            duration_s: 18.0,
            looping: true,
            ease_io: false,
            playing: false,
            time_s: 0.0,
            scene: RuntimeSceneRails::default(),
            custom_lut: CustomArcLut::default(),
        }
    }
}

/// Compares the serialized fields only; `playing`, `time_s` and the runtime caches are skipped
/// by serde, so including them would mark an unchanged config dirty on every save tick.
impl PartialEq for CameraRail {
    fn eq(&self, other: &Self) -> bool {
        self.selected_rail_id == other.selected_rail_id
            && self.keyframes == other.keyframes
            && self.duration_s == other.duration_s
            && self.looping == other.looping
            && self.ease_io == other.ease_io
    }
}

impl CameraRail {
    /// Installs compiled rails exported by a newly loaded [`csg_scene::SceneModule`].
    ///
    /// Preserves the user's `Custom` keyframes and, if `selected_rail_id` matches a rail in the
    /// new scene (or is `"custom"`), keeps that selection active; otherwise selects `specs[0]`.
    pub fn set_scene_rails(&mut self, specs: Vec<RailSpec>) {
        let compiled: Vec<CompiledRail> = specs
            .into_iter()
            .filter(RailSpec::is_valid)
            .map(CompiledRail::compile)
            .collect();
        self.scene.base_count = compiled.len();
        self.scene.rails = Arc::new(compiled);
        self.scene.active_idx = 0;

        if self.selected_rail_id == CUSTOM_RAIL_ID {
            if self.keyframes.len() < 2 {
                self.playing = false;
            }
            return;
        }

        if let Some(found) = self
            .scene
            .rails
            .iter()
            .position(|r| r.spec.id == self.selected_rail_id)
        {
            // The same rail survives the rebuild: keep the user's duration like `looping` and
            // `ease_io`.
            self.scene.active_idx = found;
        } else if let Some(first) = self.scene.rails.first() {
            self.scene.active_idx = 0;
            self.selected_rail_id = first.spec.id.clone();
            self.duration_s = first.spec.duration_s.max(2.0);
            self.looping = first.spec.looping;
            self.time_s = 0.0;
        } else if self.keyframes.len() < 2 {
            self.playing = false;
        }
    }

    /// Appends a freshly generated exact-CSG random rail, selects it, and starts playback.
    pub fn push_generated_rail(&mut self, spec: RailSpec) {
        if !spec.is_valid() {
            return;
        }
        let max_total = self.scene.base_count + MAX_GENERATED_RAILS;
        let base_count = self.scene.base_count;
        // Copy-on-write: clones the cassette list only if a persisted snapshot still shares it.
        let rails = Arc::make_mut(&mut self.scene.rails);
        if rails.len() >= max_total && rails.len() > base_count {
            rails.remove(base_count);
        }
        let dur = spec.duration_s.max(2.0);
        let id = spec.id.clone();
        let looping = spec.looping;
        rails.push(CompiledRail::compile(spec));
        self.scene.active_idx = rails.len() - 1;
        self.selected_rail_id = id;
        self.duration_s = dur;
        self.looping = looping;
        self.ease_io = false;
        self.time_s = 0.0;
        self.playing = true;
    }

    /// Read-only slice of compiled cassettes currently available for the active scene.
    #[inline]
    pub fn scene_rails(&self) -> &[CompiledRail] {
        &self.scene.rails
    }

    /// Returns `true` when the user's independent `Custom` keyframe deck is currently selected.
    #[inline]
    pub fn is_custom_selected(&self) -> bool {
        self.selected_rail_id == CUSTOM_RAIL_ID || self.scene.rails.is_empty()
    }

    /// Index of the active scene cassette when not in `Custom` mode.
    #[inline]
    pub fn active_scene_rail_index(&self) -> Option<usize> {
        if self.is_custom_selected() {
            None
        } else {
            Some(self.scene.active_idx.min(self.scene.rails.len().saturating_sub(1)))
        }
    }

    /// Human-readable label for the currently selected cassette or `Custom (N KF)`.
    pub fn active_label(&self) -> String {
        if let Some(idx) = self.active_scene_rail_index()
            && let Some(r) = self.scene.rails.get(idx)
        {
            return r.spec.label.clone();
        }
        format!("Custom ({} KF)", self.keyframes.len())
    }

    /// Selects a scene cassette by index without disturbing `self.keyframes`.
    pub fn select_scene_rail(&mut self, idx: usize) {
        let Some(r) = self.scene.rails.get(idx) else {
            return;
        };
        self.scene.active_idx = idx;
        self.selected_rail_id = r.spec.id.clone();
        self.duration_s = r.spec.duration_s.max(2.0);
        self.looping = r.spec.looping;
        self.ease_io = false;
        self.time_s = 0.0;
    }

    /// Switches to the user's independent `Custom` keyframe deck.
    pub fn select_custom(&mut self) {
        self.selected_rail_id = CUSTOM_RAIL_ID.to_string();
        self.time_s = 0.0;
        if self.keyframes.len() < 2 {
            self.playing = false;
        }
        self.sync_lut();
    }

    /// Returns `true` if the currently selected cassette has enough waypoints (`>= 2`) to play.
    #[inline]
    pub fn can_play(&self) -> bool {
        if let Some(idx) = self.active_scene_rail_index() {
            self.scene.rails.get(idx).is_some_and(|r| r.spec.is_valid())
        } else {
            self.keyframes.len() >= 2
        }
    }

    /// Returns `true` when Auto-Drive playback is actively driving the camera.
    #[inline]
    pub fn is_active(&self) -> bool {
        self.playing && self.can_play()
    }

    /// Toggles Auto-Drive ON/OFF without altering cassettes or `self.keyframes`.
    pub fn set_playing(&mut self, on: bool) -> bool {
        if on && self.can_play() {
            if !self.looping && self.time_s >= self.duration_s.max(0.5) {
                self.time_s = 0.0;
            }
            self.playing = true;
            self.sync_lut();
        } else {
            self.playing = false;
        }
        self.playing
    }

    /// Appends the current camera pose to the `Custom` keyframe deck and selects `Custom`.
    pub fn push_pose(&mut self, pose: CameraPose) {
        self.keyframes.push(CameraKeyframe::from_pose(pose));
        self.selected_rail_id = CUSTOM_RAIL_ID.to_string();
        self.sync_lut();
    }

    /// Removes the last `Custom` keyframe without affecting scene cassettes.
    pub fn pop_keyframe(&mut self) {
        self.keyframes.pop();
        if self.is_custom_selected() && self.keyframes.len() < 2 {
            self.playing = false;
            self.time_s = 0.0;
        }
        self.sync_lut();
    }

    /// Clears all `Custom` keyframes without disturbing scene cassettes.
    pub fn clear(&mut self) {
        self.keyframes.clear();
        if self.is_custom_selected() {
            self.playing = false;
            self.time_s = 0.0;
        }
        self.sync_lut();
    }

    /// Rebuilds the inline `Custom` arc-length LUT only when the custom track signature changes.
    pub fn sync_lut(&mut self) {
        let sig = custom_lut_signature(self.looping, &self.keyframes);
        if self.custom_lut.signature == sig {
            return;
        }
        self.custom_lut = CustomArcLut::build(sig, &self.keyframes, self.looping);
    }

    /// Advances the rail clock by `dt` seconds with zero heap allocations.
    pub fn tick(&mut self, dt: f32) {
        if self.is_custom_selected() {
            self.sync_lut();
        }
        if !self.is_active() || !dt.is_finite() || dt <= 0.0 {
            return;
        }
        let dur = self.duration_s.max(0.5);
        self.time_s += dt;
        if self.looping {
            self.time_s = self.time_s.rem_euclid(dur);
        } else if self.time_s >= dur {
            self.time_s = dur;
            self.playing = false;
        }
    }

    /// Evaluates the active cassette or `Custom` spline at `self.time_s`.
    pub fn evaluate_current(&self, znear: f32, zfar: f32) -> Option<CameraPose> {
        self.evaluate_at(self.time_s, znear, zfar)
    }

    /// Evaluates the active cassette or `Custom` spline at `time_s` with zero heap allocations.
    pub fn evaluate_at(&self, time_s: f32, znear: f32, zfar: f32) -> Option<CameraPose> {
        let dur = self.duration_s.max(0.5);
        let raw_frac = if self.looping {
            (time_s / dur).rem_euclid(1.0)
        } else {
            (time_s / dur).clamp(0.0, 1.0)
        };
        let arc_frac = if self.ease_io {
            smoothstep01(raw_frac)
        } else {
            raw_frac
        };

        if let Some(idx) = self.active_scene_rail_index()
            && let Some(compiled) = self.scene.rails.get(idx)
        {
            let sample = compiled.eval_at_fraction(arc_frac);
            let eye = Vec3::new(sample.eye.x, sample.eye.y, sample.eye.z);
            let fwd = Vec3::new(sample.forward.x, sample.forward.y, sample.forward.z);
            let up = Vec3::new(sample.up.x, sample.up.y, sample.up.z);
            return Some(CameraPose::from_eye_forward_up(
                eye,
                fwd,
                up,
                clamp_fov(sample.vfov_deg.to_radians()),
                znear.max(1e-3),
                zfar.max(znear + 1.0),
            ));
        }

        // Evaluate the user's independent `Custom` keyframe spline.
        let n = self.keyframes.len();
        if n == 0 {
            return None;
        }
        if n == 1 {
            return Some(self.keyframes[0].to_pose(znear, zfar));
        }

        let sig = custom_lut_signature(self.looping, &self.keyframes);
        let local_lut;
        let lut = if self.custom_lut.signature == sig {
            &self.custom_lut
        } else {
            local_lut = CustomArcLut::build(sig, &self.keyframes, self.looping);
            &local_lut
        };
        let u = lut.u_for_fraction(arc_frac);

        let seg = spline_segment(n, u, self.looping);
        let k0 = self.keyframes[seg.i0];
        let k1 = self.keyframes[seg.i1];
        let k2 = self.keyframes[seg.i2];
        let k3 = self.keyframes[seg.i3];

        let p1 = Vec3::from_array(k1.pos);
        let p2 = Vec3::from_array(k2.pos);
        let p0 = if !self.looping && seg.seg_idx == 0 {
            p1 * 2.0 - p2
        } else {
            Vec3::from_array(k0.pos)
        };
        let p3 = if !self.looping && seg.seg_idx + 1 == seg.seg_count {
            p2 * 2.0 - p1
        } else {
            Vec3::from_array(k3.pos)
        };

        let eye = centripetal_catmull_rom(p0, p1, p2, p3, seg.local_s);
        let orientation = if n >= 3 {
            squad_interpolate(k0.quat(), k1.quat(), k2.quat(), k3.quat(), seg.local_s)
        } else {
            shortest_slerp(k1.quat(), k2.quat(), seg.local_s)
        };
        let vfov_deg = k1.vfov_deg + (k2.vfov_deg - k1.vfov_deg) * seg.local_s;

        Some(CameraPose {
            eye,
            orientation,
            fov_y: clamp_fov(vfov_deg.to_radians()),
            znear: znear.max(1e-3),
            zfar: zfar.max(znear + 1.0),
        })
    }
}

/// Exponential gimbal camera damper smoothing position, orientation (`slerp`), and vertical FoV.
///
/// Snaps immediately on teleport / preset changes or when `tau <= 1e-4`, and settles cleanly onto
/// the exact target pose once within convergence epsilon so progressive path tracing can converge.
#[derive(Clone, Copy, Debug, Default)]
pub struct CameraDamper {
    smoothed: Option<CameraPose>,
}

impl CameraDamper {
    /// Immediately snaps the smoothed pose to `pose` without damping lag.
    #[inline]
    pub fn snap(&mut self, pose: CameraPose) {
        self.smoothed = Some(pose);
    }

    /// Advances the damped pose toward `target` over `dt` seconds with time constant `tau` seconds.
    pub fn step(&mut self, target: CameraPose, dt: f32, tau: f32) -> CameraPose {
        if !tau.is_finite() || tau <= 1.0e-4 || !dt.is_finite() || dt <= 0.0 {
            self.smoothed = Some(target);
            return target;
        }
        let Some(prev) = self.smoothed else {
            self.smoothed = Some(target);
            return target;
        };

        let alpha = (1.0 - (-dt / tau).exp()).clamp(0.0, 1.0);
        let eye = prev.eye.lerp(target.eye, alpha);
        let orientation = shortest_slerp(prev.orientation, target.orientation, alpha);
        let fov_y = prev.fov_y + (target.fov_y - prev.fov_y) * alpha;

        // Snap once converged so path-tracing accumulation stops resetting on micro-residuals.
        // A component that did not move this step is stuck at f32 resolution (far from the
        // origin `alpha * residual` rounds to zero) and would repeat the same value forever, so
        // it counts as settled too.
        let pos_settled = eye == prev.eye || (eye - target.eye).length_squared() < 1.0e-10;
        let rot_settled = orientation == prev.orientation
            || (1.0 - orientation.dot(target.orientation).abs()) < 1.0e-10;
        let fov_settled = fov_y == prev.fov_y || (fov_y - target.fov_y).abs() < 1.0e-6;
        let next = if pos_settled && rot_settled && fov_settled {
            target
        } else {
            CameraPose {
                eye,
                orientation,
                fov_y,
                znear: target.znear,
                zfar: target.zfar,
            }
        };
        self.smoothed = Some(next);
        next
    }
}

/// Computes the exact simulation time delta for subframe `sub_idx` (`0..sub_count`) of a
/// 180-degree shutter window spanning `0.5 * frame_dt` per output video frame.
///
/// The sum of deltas across `sub_idx = 0..sub_count` equals `frame_dt` identically so long
/// recordings experience zero time drift regardless of `sub_count` (`1`, `2`, or `4`).
#[inline]
pub fn shutter_subframe_dt(frame_dt: f32, sub_idx: u32, sub_count: u32) -> f32 {
    if !frame_dt.is_finite() || frame_dt <= 0.0 {
        return 0.0;
    }
    let n = sub_count.max(1);
    if n == 1 {
        return frame_dt;
    }
    let sub_dt = (0.5 * frame_dt) / (n as f32);
    if sub_idx == 0 {
        // Jump across the closed-shutter dark interval from the previous frame's last subframe.
        frame_dt - sub_dt * ((n - 1) as f32)
    } else {
        sub_dt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csg_scene::SceneBuildConfig;

    #[test]
    fn smoothstep_is_monotonic_and_clamped() {
        assert_eq!(smoothstep01(-0.5), 0.0);
        assert_eq!(smoothstep01(0.0), 0.0);
        assert_eq!(smoothstep01(1.0), 1.0);
        assert_eq!(smoothstep01(1.5), 1.0);
        assert_eq!(smoothstep01(f32::NAN), 0.0);

        let mut prev = 0.0f32;
        for i in 1..=100 {
            let v = smoothstep01(i as f32 / 100.0);
            assert!(v >= prev, "smoothstep must be monotonic: {v} < {prev}");
            prev = v;
        }
    }

    #[test]
    fn open_custom_rail_interpolates_exact_endpoints_and_shortest_slerp() {
        let mut rail = CameraRail {
            selected_rail_id: CUSTOM_RAIL_ID.to_string(),
            duration_s: 10.0,
            looping: false,
            ease_io: true,
            ..Default::default()
        };
        let k0 = CameraKeyframe::from_look_at([0.0, 2.0, -10.0], [0.0, 1.0, 0.0], 45.0);
        let mut k1 = CameraKeyframe::from_look_at([10.0, 4.0, 0.0], [0.0, 1.0, 0.0], 60.0);
        k1.orientation = k1.orientation.map(|v| -v);
        let k2 = CameraKeyframe::from_look_at([0.0, 3.0, 10.0], [0.0, 1.0, 0.0], 50.0);
        rail.keyframes = vec![k0, k1, k2];
        rail.sync_lut();

        let p_start = rail.evaluate_at(0.0, 0.05, 500.0).expect("start");
        assert!((p_start.eye - Vec3::from_array(k0.pos)).length() < 1e-4);
        assert!((p_start.fov_y.to_degrees() - 45.0).abs() < 1e-4);

        let p_end = rail.evaluate_at(10.0, 0.05, 500.0).expect("end");
        assert!((p_end.eye - Vec3::from_array(k2.pos)).length() < 1e-4);
        assert!((p_end.fov_y.to_degrees() - 50.0).abs() < 1e-4);

        let mut prev_fwd = p_start.forward();
        for i in 1..=40 {
            let p = rail
                .evaluate_at(10.0 * (i as f32 / 40.0), 0.05, 500.0)
                .expect("sample");
            assert!(p.eye.is_finite());
            assert!(p.orientation.is_finite());
            assert!(
                p.forward().dot(prev_fwd) > 0.85,
                "orientation discontinuity at step {i}"
            );
            prev_fwd = p.forward();
        }
    }

    #[test]
    fn custom_loop_c1_seam_and_arc_length_uniform_speed() {
        // Non-uniform spacing (3m next to 60m) must travel at constant arc-length speed.
        let mut uneven = CameraRail {
            selected_rail_id: CUSTOM_RAIL_ID.to_string(),
            duration_s: 10.0,
            looping: false,
            ease_io: false,
            keyframes: vec![
                CameraKeyframe::from_look_at([0.0, 4.0, 0.0], [0.0, 4.0, 100.0], 50.0),
                CameraKeyframe::from_look_at([0.0, 4.0, 3.0], [0.0, 4.0, 100.0], 50.0),
                CameraKeyframe::from_look_at([0.0, 4.0, 63.0], [0.0, 4.0, 100.0], 50.0),
                CameraKeyframe::from_look_at([0.0, 4.0, 68.0], [0.0, 4.0, 100.0], 50.0),
            ],
            ..Default::default()
        };
        uneven.sync_lut();
        let mut prev = uneven.evaluate_at(0.0, 0.05, 500.0).expect("s0").eye;
        let mut min_step = f32::INFINITY;
        let mut max_step = 0.0f32;
        for i in 1..=64 {
            let cur = uneven
                .evaluate_at(10.0 * (i as f32 / 64.0), 0.05, 500.0)
                .expect("si")
                .eye;
            let d = cur.distance(prev);
            min_step = min_step.min(d);
            max_step = max_step.max(d);
            prev = cur;
        }
        assert!(
            max_step / min_step.max(1e-5) < 1.12,
            "expected uniform speed, min={min_step}, max={max_step}"
        );

        // Closed-loop SQUAD orientation and C1 tangent continuity across `1.0 -> 0.0`.
        let mut custom = CameraRail {
            selected_rail_id: CUSTOM_RAIL_ID.to_string(),
            duration_s: 12.0,
            looping: true,
            ease_io: false,
            keyframes: vec![
                CameraKeyframe::from_look_at([-20.0, 5.0, -20.0], [0.0, 2.0, 0.0], 50.0),
                CameraKeyframe::from_look_at([20.0, 8.0, -20.0], [0.0, 2.0, 0.0], 52.0),
                CameraKeyframe::from_look_at([20.0, 6.0, 20.0], [0.0, 2.0, 0.0], 50.0),
                CameraKeyframe::from_look_at([-20.0, 9.0, 20.0], [0.0, 2.0, 0.0], 48.0),
            ],
            ..Default::default()
        };
        custom.sync_lut();
        let eps = 0.012;
        let p_b = custom.evaluate_at(12.0 - eps, 0.05, 500.0).expect("b");
        let p_0 = custom.evaluate_at(0.0, 0.05, 500.0).expect("0");
        let p_a = custom.evaluate_at(eps, 0.05, 500.0).expect("a");
        let v_in = (p_0.eye - p_b.eye).normalize();
        let v_out = (p_a.eye - p_0.eye).normalize();
        assert!(v_in.dot(v_out) > 0.99, "custom loop velocity kink");
        assert!(p_b.forward().dot(p_0.forward()) > 0.995, "custom loop orientation pop");
    }

    #[test]
    fn scene_registry_rails_evaluate_cleanly_and_preserve_custom_keyframes() {
        let reg = csg_scenes::default_scene_registry();
        let mut rail = CameraRail::default();
        let k0 = CameraKeyframe::from_look_at([-10.0, 3.0, -10.0], [0.0, 2.0, 0.0], 48.0)
            .to_pose(0.05, 500.0);
        let k1 = CameraKeyframe::from_look_at([12.0, 5.0, 14.0], [0.0, 2.0, 0.0], 55.0)
            .to_pose(0.05, 500.0);
        rail.push_pose(k0);
        rail.push_pose(k1);
        assert_eq!(rail.keyframes.len(), 2);

        for scene_id in ["gundo", "marina", "sf_house_of_ai", "inspector"] {
            let module = reg.get(scene_id).expect("scene registered");
            let built = module.build(&SceneBuildConfig::default()).expect("scene builds");
            rail.selected_rail_id.clear();
            rail.set_scene_rails(built.rails);
            assert!(
                !rail.scene_rails().is_empty(),
                "{scene_id}: expected >= 1 rail cassette"
            );
            assert_eq!(rail.keyframes.len(), 2, "custom keyframes must be preserved");

            for idx in 0..rail.scene_rails().len() {
                rail.select_scene_rail(idx);
                assert!(rail.can_play());
                for s in 0..16 {
                    let t = rail.duration_s * (s as f32 / 16.0);
                    let pose = rail.evaluate_at(t, 0.05, 2000.0).expect("pose");
                    assert!(pose.eye.is_finite() && pose.orientation.is_finite());
                }
            }
        }

        // Verify serde round-trip preserves `selected_rail_id` and `Custom` keyframes cleanly.
        let json = serde_json::to_string(&rail).expect("serialize rail");
        let decoded: CameraRail = serde_json::from_str(&json).expect("deserialize rail");
        assert_eq!(decoded.keyframes.len(), 2);
        assert!(!decoded.playing);
        assert_eq!(decoded.time_s, 0.0);
        assert_eq!(decoded, rail);
    }

    #[test]
    fn gimbal_damper_snaps_on_zero_and_converges_smoothly() {
        let p0 = CameraKeyframe::from_look_at([0.0, 2.0, -5.0], [0.0, 0.0, 0.0], 45.0)
            .to_pose(0.05, 500.0);
        let p1 = CameraKeyframe::from_look_at([6.0, 5.0, 4.0], [0.0, 1.0, 0.0], 60.0)
            .to_pose(0.05, 500.0);

        let mut damper = CameraDamper::default();
        damper.snap(p0);
        let snapped = damper.step(p1, 1.0 / 60.0, 0.0);
        assert_eq!(snapped.eye, p1.eye);
        assert_eq!(snapped.fov_y, p1.fov_y);

        damper.snap(p0);
        let mut prev_dist = p0.eye.distance(p1.eye);
        for _ in 0..600 {
            let cur = damper.step(p1, 1.0 / 60.0, 0.25);
            let d = cur.eye.distance(p1.eye);
            assert!(d <= prev_dist + 1e-6);
            prev_dist = d;
        }
        assert!(prev_dist < 1e-4, "expected convergence, dist={prev_dist}");
    }

    #[test]
    fn keyframes_keep_the_full_lens_fov_range() {
        // Dolly-zoom reaches 5 degrees; the keyframe must replay that framing, not 10 degrees.
        let pose = CameraPose {
            eye: Vec3::new(1.0, 2.0, 3.0),
            orientation: Quat::IDENTITY,
            fov_y: 5.0f32.to_radians(),
            znear: 0.05,
            zfar: 500.0,
        };
        let kf = CameraKeyframe::from_pose(pose);
        assert!((kf.vfov_deg - 5.0).abs() < 1e-4, "stored {}", kf.vfov_deg);
        assert!((kf.to_pose(0.05, 500.0).fov_y - pose.fov_y).abs() < 1e-6);

        let narrow = CameraKeyframe::from_pose(CameraPose {
            fov_y: 0.1f32.to_radians(),
            ..pose
        });
        assert!((narrow.to_pose(0.05, 500.0).fov_y - FOV_MIN).abs() < 1e-6);
        let wide = CameraKeyframe {
            vfov_deg: 170.0,
            ..kf
        };
        assert!((wide.to_pose(0.05, 500.0).fov_y - FOV_MAX).abs() < 1e-6);
    }

    #[test]
    fn equality_ignores_runtime_playback_state_and_clone_shares_rails() {
        let reg = csg_scenes::default_scene_registry();
        let module = reg.get("gundo").expect("scene registered");
        let mut rail = CameraRail::default();
        rail.set_scene_rails(
            module
                .build(&SceneBuildConfig::default())
                .expect("scene builds")
                .rails,
        );
        assert!(!rail.scene_rails().is_empty());

        let saved = rail.clone();
        assert!(Arc::ptr_eq(&saved.scene.rails, &rail.scene.rails));
        rail.set_playing(true);
        rail.tick(0.5);
        assert!(rail.time_s > 0.0);
        assert_eq!(rail, saved, "playback state is not persisted, so it must not mark dirty");

        rail.looping = !rail.looping;
        assert_ne!(rail, saved);
    }

    #[test]
    fn damper_settles_exactly_far_from_the_origin() {
        let pose = |eye: Vec3| CameraPose {
            eye,
            orientation: Quat::IDENTITY,
            fov_y: 1.0,
            znear: 0.05,
            zfar: 5000.0,
        };
        let target = pose(Vec3::new(1000.5, 50.0, 1000.25));
        let mut damper = CameraDamper::default();
        damper.snap(pose(Vec3::new(1000.0, 50.0, 1000.0)));
        let mut cur = target;
        for _ in 0..2000 {
            cur = damper.step(target, 1.0 / 60.0, 0.25);
        }
        assert_eq!(cur.eye, target.eye, "damper must snap onto the exact target");
    }

    #[test]
    fn shutter_subframe_deltas_sum_exactly_to_frame_dt() {
        let frame_dt = 1.0 / 60.0;
        for n in [1u32, 2, 4] {
            let sum: f32 = (0..n).map(|k| shutter_subframe_dt(frame_dt, k, n)).sum();
            assert!(
                (sum - frame_dt).abs() < 1e-6,
                "n={n}: sum={sum} != frame_dt={frame_dt}"
            );
            for k in 0..n {
                assert!(shutter_subframe_dt(frame_dt, k, n) > 0.0);
            }
        }
    }
}
