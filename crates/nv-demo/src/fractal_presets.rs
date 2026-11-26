//! Fractal preset model, built-in presets, and atomic JSON persistence (`~/.nv-rs/fractal_presets.json`).
//!
//! **What.** Captures and restores all 40 Hero Fractal settings (formula selector, Mandelbulb /
//! Mandelbox / KIFS Octa parameters, Transform TRS, and Detail & Shading) plus optional sun-angle
//! hints for the four canonical built-in looks (`Gold Bulb`, `Chrome Box`, `Obsidian KIFS`,
//! `Julia Coral`) via [`FractalPreset`] and [`FractalPresetManager`].
//! **Why.** Gives users full save / load / overwrite / delete preset management without polluting
//! `nv-demo.json`, while keeping built-in presets read-only and always up to date from code.
//! **How.** Wraps `sl_controls::preset::PresetStore<FractalPreset>`, renders the preset bar
//! clamped through [`crate::ui::allocate_clamped_ui`] so narrow docks never overflow, and persists
//! user-created slots to `~/.nv-rs/fractal_presets.json` via sibling `.part` + `flush()` +
//! `sync_all()` + atomic rename (backing up unreadable files to `.json.corrupt` before first
//! overwrite). Every float range is clamped against [`crate::demo_anim::anim_range`] (SSOT) so
//! hand-edited or older JSON files cannot feed NaN/Inf or out-of-range values to the OptiX kernel.
//! **Uses.** `egui`, `sl_controls::preset`, `serde`, `serde_json`, `crate::demo_anim::anim_range`,
//! `crate::renderer::DemoSettings`, `crate::ui::allocate_clamped_ui`.
//! **Used by.** `crate::main` (`DemoApp`) and `crate::ui` (`UiContext::ui_fractal`).

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use sl_controls::preset::{Preset, PresetAction, PresetError, PresetStore, preset_bar, same_name};

use crate::demo_anim::anim_range;
use crate::renderer::DemoSettings;

/// Schema version written to `~/.nv-rs/fractal_presets.json`.
pub const PRESET_FILE_VERSION: u32 = 1;

/// Canonical name of the built-in Gold Bulb preset.
pub const BUILTIN_GOLD_BULB: &str = "Gold Bulb";
/// Canonical name of the built-in Chrome Box preset.
pub const BUILTIN_CHROME_BOX: &str = "Chrome Box";
/// Canonical name of the built-in Obsidian KIFS preset.
pub const BUILTIN_OBSIDIAN_KIFS: &str = "Obsidian KIFS";
/// Canonical name of the built-in Julia Coral preset.
pub const BUILTIN_JULIA_CORAL: &str = "Julia Coral";

/// Monotonic counter keeping sibling `.part` temp filenames unique within one process.
static ATOMIC_TMP_SEQ: AtomicU64 = AtomicU64::new(0);

/// Metadata and payload for one canonical built-in fractal preset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BuiltinPresetSpec {
    /// Display name (read-only in the store and UI).
    pub name: &'static str,
    /// Hover tooltip shown on the quick-load button.
    pub tooltip: &'static str,
    /// Full parameter payload applied when selected.
    pub preset: FractalPreset,
}

/// Complete snapshot of all Fractal panel parameters (`Formula`, `Transform`, `Detail & Shading`)
/// plus optional sun-angle hints used by the built-in showcase looks.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FractalPreset {
    /// Formula selector: `0` = Mandelbulb, `1` = Mandelbox, `2` = KIFS Octa.
    pub fractal_kind: u32,
    /// World-space translation `[x, y, z]` in meters.
    pub fractal_translate: [f32; 3],
    /// Uniform scale factor.
    pub obj_scale: f32,
    /// X-axis rotation in degrees.
    pub rot_x: f32,
    /// Y-axis rotation in degrees.
    pub rot_y: f32,
    /// Z-axis rotation in degrees.
    pub rot_z: f32,
    // Mandelbulb
    /// Mandelbulb exponent power.
    pub bulb_power: f32,
    /// Mandelbulb escape bailout radius.
    pub bulb_bailout: f32,
    /// Mandelbulb iteration count (`4..=32`).
    pub bulb_iter: u32,
    /// Whether Mandelbulb runs in Julia-set mode.
    pub bulb_julia: bool,
    /// Julia constant `c = [x, y, z]`.
    pub bulb_c: [f32; 3],
    /// Polar angle multiplier.
    pub bulb_theta_mult: f32,
    /// Azimuth angle multiplier.
    pub bulb_phi_mult: f32,
    /// Polar phase offset in degrees.
    pub bulb_theta_phase: f32,
    /// Azimuth phase offset in degrees.
    pub bulb_phi_phase: f32,
    // Mandelbox
    /// Mandelbox scale factor.
    pub box_scale: f32,
    /// Box-folding limit.
    pub box_fold: f32,
    /// Sphere-fold inner radius.
    pub box_min_r: f32,
    /// Sphere-fold fixed radius.
    pub box_fixed_r: f32,
    /// Mandelbox iteration count (`4..=32`).
    pub box_iter: u32,
    /// Mandelbox escape bailout radius.
    pub box_bailout: f32,
    /// Box-fold multiplier.
    pub box_fold_mult: f32,
    /// Per-iteration twist rotation in degrees.
    pub box_iter_rot: f32,
    // KIFS Octahedron
    /// KIFS scale factor.
    pub kifs_scale: f32,
    /// Primary KIFS fold rotation (Y) in degrees.
    pub kifs_rot1: f32,
    /// Secondary KIFS fold rotation (X) in degrees.
    pub kifs_rot2: f32,
    /// KIFS translation offset `[x, y, z]`.
    pub kifs_offset: [f32; 3],
    /// KIFS iteration count (`4..=32`).
    pub kifs_iter: u32,
    /// KIFS escape bailout radius.
    pub kifs_bailout: f32,
    /// KIFS strut thickness.
    pub kifs_thickness: f32,
    /// KIFS Z-axis folding bias.
    pub kifs_z_bias: f32,
    // Detail & Shading
    /// Distance-estimator surface hit epsilon.
    pub detail_eps: f32,
    /// Ray-marching step safety fudge factor.
    pub fudge: f32,
    /// Finite-difference normal epsilon.
    pub norm_eps: f32,
    /// Maximum ray-marching steps (`32..=512`).
    pub march_steps: u32,
    /// Maximum path-tracing bounces (`1..=16`).
    pub bounces: u32,
    /// Surface metallic factor (`0.0..=1.0`).
    pub metallic: f32,
    /// Surface roughness factor.
    pub roughness: f32,
    /// Orbit-trap color scale.
    pub trap_scale: f32,
    /// Palette hue shift (`0.0..=1.0`).
    pub palette_shift: f32,
    // Optional lighting hints (populated only on built-in showcase looks so user presets do not
    // unexpectedly overwrite CSG world lighting / time-of-day).
    /// Optional sun azimuth angle in degrees (`None` = leave current scene lighting untouched).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sun_azimuth: Option<f32>,
    /// Optional sun elevation angle in degrees (`None` = leave current scene lighting untouched).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sun_elevation: Option<f32>,
}

impl Default for FractalPreset {
    fn default() -> Self {
        Self::from_settings(&DemoSettings::default())
    }
}

impl FractalPreset {
    /// Captures all 40 Fractal panel parameters from `s` (without capturing scene sun angles).
    pub fn from_settings(s: &DemoSettings) -> Self {
        Self {
            fractal_kind: s.fractal_kind,
            fractal_translate: s.fractal_translate,
            obj_scale: s.obj_scale,
            rot_x: s.rot_x,
            rot_y: s.rot_y,
            rot_z: s.rot_z,
            bulb_power: s.bulb_power,
            bulb_bailout: s.bulb_bailout,
            bulb_iter: s.bulb_iter,
            bulb_julia: s.bulb_julia,
            bulb_c: s.bulb_c,
            bulb_theta_mult: s.bulb_theta_mult,
            bulb_phi_mult: s.bulb_phi_mult,
            bulb_theta_phase: s.bulb_theta_phase,
            bulb_phi_phase: s.bulb_phi_phase,
            box_scale: s.box_scale,
            box_fold: s.box_fold,
            box_min_r: s.box_min_r,
            box_fixed_r: s.box_fixed_r,
            box_iter: s.box_iter,
            box_bailout: s.box_bailout,
            box_fold_mult: s.box_fold_mult,
            box_iter_rot: s.box_iter_rot,
            kifs_scale: s.kifs_scale,
            kifs_rot1: s.kifs_rot1,
            kifs_rot2: s.kifs_rot2,
            kifs_offset: s.kifs_offset,
            kifs_iter: s.kifs_iter,
            kifs_bailout: s.kifs_bailout,
            kifs_thickness: s.kifs_thickness,
            kifs_z_bias: s.kifs_z_bias,
            detail_eps: s.detail_eps,
            fudge: s.fudge,
            norm_eps: s.norm_eps,
            march_steps: s.march_steps,
            bounces: s.bounces,
            metallic: s.metallic,
            roughness: s.roughness,
            trap_scale: s.trap_scale,
            palette_shift: s.palette_shift,
            sun_azimuth: None,
            sun_elevation: None,
        }
    }

    /// Returns a copy with every field clamped to its authoritative range (`demo_anim::anim_range`
    /// for floats, UI slider bounds for integers) and any non-finite float replaced by its default.
    pub fn sanitized(self) -> Self {
        let d = Self::default();
        Self {
            fractal_kind: self.fractal_kind.min(2),
            fractal_translate: sanitize_vec3(
                self.fractal_translate,
                d.fractal_translate,
                "fractal_translate",
            ),
            obj_scale: sanitize_f32(self.obj_scale, d.obj_scale, "obj_scale"),
            rot_x: sanitize_f32(self.rot_x, d.rot_x, "rot_x"),
            rot_y: sanitize_f32(self.rot_y, d.rot_y, "rot_y"),
            rot_z: sanitize_f32(self.rot_z, d.rot_z, "rot_z"),
            bulb_power: sanitize_f32(self.bulb_power, d.bulb_power, "bulb_power"),
            bulb_bailout: sanitize_f32(self.bulb_bailout, d.bulb_bailout, "bulb_bailout"),
            bulb_iter: self.bulb_iter.clamp(4, 32),
            bulb_julia: self.bulb_julia,
            bulb_c: sanitize_vec3(self.bulb_c, d.bulb_c, "bulb_c"),
            bulb_theta_mult: sanitize_f32(
                self.bulb_theta_mult,
                d.bulb_theta_mult,
                "bulb_theta_mult",
            ),
            bulb_phi_mult: sanitize_f32(self.bulb_phi_mult, d.bulb_phi_mult, "bulb_phi_mult"),
            bulb_theta_phase: sanitize_f32(
                self.bulb_theta_phase,
                d.bulb_theta_phase,
                "bulb_theta_phase",
            ),
            bulb_phi_phase: sanitize_f32(self.bulb_phi_phase, d.bulb_phi_phase, "bulb_phi_phase"),
            box_scale: sanitize_f32(self.box_scale, d.box_scale, "box_scale"),
            box_fold: sanitize_f32(self.box_fold, d.box_fold, "box_fold"),
            box_min_r: sanitize_f32(self.box_min_r, d.box_min_r, "box_min_r"),
            box_fixed_r: sanitize_f32(self.box_fixed_r, d.box_fixed_r, "box_fixed_r"),
            box_iter: self.box_iter.clamp(4, 32),
            box_bailout: sanitize_f32(self.box_bailout, d.box_bailout, "box_bailout"),
            box_fold_mult: sanitize_f32(self.box_fold_mult, d.box_fold_mult, "box_fold_mult"),
            box_iter_rot: sanitize_f32(self.box_iter_rot, d.box_iter_rot, "box_iter_rot"),
            kifs_scale: sanitize_f32(self.kifs_scale, d.kifs_scale, "kifs_scale"),
            kifs_rot1: sanitize_f32(self.kifs_rot1, d.kifs_rot1, "kifs_rot1"),
            kifs_rot2: sanitize_f32(self.kifs_rot2, d.kifs_rot2, "kifs_rot2"),
            kifs_offset: sanitize_vec3(self.kifs_offset, d.kifs_offset, "kifs_offset"),
            kifs_iter: self.kifs_iter.clamp(4, 32),
            kifs_bailout: sanitize_f32(self.kifs_bailout, d.kifs_bailout, "kifs_bailout"),
            kifs_thickness: sanitize_f32(self.kifs_thickness, d.kifs_thickness, "kifs_thickness"),
            kifs_z_bias: sanitize_f32(self.kifs_z_bias, d.kifs_z_bias, "kifs_z_bias"),
            detail_eps: sanitize_f32(self.detail_eps, d.detail_eps, "detail_eps"),
            fudge: sanitize_f32(self.fudge, d.fudge, "fudge"),
            norm_eps: sanitize_f32(self.norm_eps, d.norm_eps, "norm_eps"),
            march_steps: self.march_steps.clamp(32, 512),
            bounces: self.bounces.clamp(1, 16),
            metallic: sanitize_f32(self.metallic, d.metallic, "metallic"),
            roughness: sanitize_f32(self.roughness, d.roughness, "roughness"),
            trap_scale: sanitize_f32(self.trap_scale, d.trap_scale, "trap_scale"),
            palette_shift: sanitize_f32(self.palette_shift, d.palette_shift, "palette_shift"),
            sun_azimuth: self
                .sun_azimuth
                .filter(|v| v.is_finite())
                .map(|v| sanitize_f32(v, 32.0, "sun_azimuth")),
            sun_elevation: self
                .sun_elevation
                .filter(|v| v.is_finite())
                .map(|v| sanitize_f32(v, 48.0, "sun_elevation")),
        }
    }

    /// Applies this preset (sanitized) onto `s`. Non-fractal fields stay untouched except optional
    /// built-in sun hints when `Some`.
    pub fn apply_to(&self, s: &mut DemoSettings) {
        let p = self.sanitized();
        s.fractal_kind = p.fractal_kind;
        s.fractal_translate = p.fractal_translate;
        s.obj_scale = p.obj_scale;
        s.rot_x = p.rot_x;
        s.rot_y = p.rot_y;
        s.rot_z = p.rot_z;
        s.bulb_power = p.bulb_power;
        s.bulb_bailout = p.bulb_bailout;
        s.bulb_iter = p.bulb_iter;
        s.bulb_julia = p.bulb_julia;
        s.bulb_c = p.bulb_c;
        s.bulb_theta_mult = p.bulb_theta_mult;
        s.bulb_phi_mult = p.bulb_phi_mult;
        s.bulb_theta_phase = p.bulb_theta_phase;
        s.bulb_phi_phase = p.bulb_phi_phase;
        s.box_scale = p.box_scale;
        s.box_fold = p.box_fold;
        s.box_min_r = p.box_min_r;
        s.box_fixed_r = p.box_fixed_r;
        s.box_iter = p.box_iter;
        s.box_bailout = p.box_bailout;
        s.box_fold_mult = p.box_fold_mult;
        s.box_iter_rot = p.box_iter_rot;
        s.kifs_scale = p.kifs_scale;
        s.kifs_rot1 = p.kifs_rot1;
        s.kifs_rot2 = p.kifs_rot2;
        s.kifs_offset = p.kifs_offset;
        s.kifs_iter = p.kifs_iter;
        s.kifs_bailout = p.kifs_bailout;
        s.kifs_thickness = p.kifs_thickness;
        s.kifs_z_bias = p.kifs_z_bias;
        s.detail_eps = p.detail_eps;
        s.fudge = p.fudge;
        s.norm_eps = p.norm_eps;
        s.march_steps = p.march_steps;
        s.bounces = p.bounces;
        s.metallic = p.metallic;
        s.roughness = p.roughness;
        s.trap_scale = p.trap_scale;
        s.palette_shift = p.palette_shift;
        if let Some(az) = p.sun_azimuth {
            s.sun_azimuth = az;
        }
        if let Some(el) = p.sun_elevation {
            s.sun_elevation = el;
        }
    }

    /// Returns `true` when all 40 fractal settings in `s` match this preset within float tolerance.
    pub fn matches_settings(&self, s: &DemoSettings) -> bool {
        let a = self.sanitized();
        let b = Self::from_settings(s).sanitized();
        let feq = |x: f32, y: f32| (x - y).abs() <= 1e-5;
        let v3eq = |x: [f32; 3], y: [f32; 3]| feq(x[0], y[0]) && feq(x[1], y[1]) && feq(x[2], y[2]);
        a.fractal_kind == b.fractal_kind
            && v3eq(a.fractal_translate, b.fractal_translate)
            && feq(a.obj_scale, b.obj_scale)
            && feq(a.rot_x, b.rot_x)
            && feq(a.rot_y, b.rot_y)
            && feq(a.rot_z, b.rot_z)
            && feq(a.bulb_power, b.bulb_power)
            && feq(a.bulb_bailout, b.bulb_bailout)
            && a.bulb_iter == b.bulb_iter
            && a.bulb_julia == b.bulb_julia
            && v3eq(a.bulb_c, b.bulb_c)
            && feq(a.bulb_theta_mult, b.bulb_theta_mult)
            && feq(a.bulb_phi_mult, b.bulb_phi_mult)
            && feq(a.bulb_theta_phase, b.bulb_theta_phase)
            && feq(a.bulb_phi_phase, b.bulb_phi_phase)
            && feq(a.box_scale, b.box_scale)
            && feq(a.box_fold, b.box_fold)
            && feq(a.box_min_r, b.box_min_r)
            && feq(a.box_fixed_r, b.box_fixed_r)
            && a.box_iter == b.box_iter
            && feq(a.box_bailout, b.box_bailout)
            && feq(a.box_fold_mult, b.box_fold_mult)
            && feq(a.box_iter_rot, b.box_iter_rot)
            && feq(a.kifs_scale, b.kifs_scale)
            && feq(a.kifs_rot1, b.kifs_rot1)
            && feq(a.kifs_rot2, b.kifs_rot2)
            && v3eq(a.kifs_offset, b.kifs_offset)
            && a.kifs_iter == b.kifs_iter
            && feq(a.kifs_bailout, b.kifs_bailout)
            && feq(a.kifs_thickness, b.kifs_thickness)
            && feq(a.kifs_z_bias, b.kifs_z_bias)
            && feq(a.detail_eps, b.detail_eps)
            && feq(a.fudge, b.fudge)
            && feq(a.norm_eps, b.norm_eps)
            && a.march_steps == b.march_steps
            && a.bounces == b.bounces
            && feq(a.metallic, b.metallic)
            && feq(a.roughness, b.roughness)
            && feq(a.trap_scale, b.trap_scale)
            && feq(a.palette_shift, b.palette_shift)
    }
}

fn sanitize_f32(val: f32, fallback: f32, anim_key: &str) -> f32 {
    let v = if val.is_finite() { val } else { fallback };
    match anim_range(anim_key) {
        Some(range) => v.clamp(*range.start(), *range.end()),
        None => v,
    }
}

fn sanitize_vec3(val: [f32; 3], fallback: [f32; 3], anim_key: &str) -> [f32; 3] {
    [
        sanitize_f32(val[0], fallback[0], anim_key),
        sanitize_f32(val[1], fallback[1], anim_key),
        sanitize_f32(val[2], fallback[2], anim_key),
    ]
}

/// Returns the four canonical built-in presets in display order (`Gold Bulb`, `Chrome Box`,
/// `Obsidian KIFS`, `Julia Coral`).
pub fn builtin_presets() -> [BuiltinPresetSpec; 4] {
    let base = FractalPreset::default();
    [
        BuiltinPresetSpec {
            name: BUILTIN_GOLD_BULB,
            tooltip: "Load a classic 3D Mandelbulb preset with warm metallic gold shading.",
            preset: FractalPreset {
                fractal_kind: 0,
                bulb_power: 8.0,
                bulb_julia: false,
                metallic: 0.78,
                roughness: 0.14,
                palette_shift: 0.08,
                sun_azimuth: Some(36.0),
                sun_elevation: Some(42.0),
                ..base
            },
        },
        BuiltinPresetSpec {
            name: BUILTIN_CHROME_BOX,
            tooltip: "Load an architectural Mandelbox preset with polished chrome reflections.",
            preset: FractalPreset {
                fractal_kind: 1,
                box_scale: -2.0,
                box_fold: 1.0,
                metallic: 0.92,
                roughness: 0.10,
                palette_shift: 0.45,
                sun_azimuth: Some(-45.0),
                sun_elevation: Some(35.0),
                ..base
            },
        },
        BuiltinPresetSpec {
            name: BUILTIN_OBSIDIAN_KIFS,
            tooltip: "Load a geometric KIFS/Menger folding fractal with dark glossy material.",
            preset: FractalPreset {
                fractal_kind: 2,
                kifs_scale: 2.0,
                kifs_rot1: 18.0,
                kifs_rot2: -12.0,
                metallic: 0.65,
                roughness: 0.22,
                palette_shift: 0.72,
                sun_azimuth: Some(55.0),
                sun_elevation: Some(28.0),
                ..base
            },
        },
        BuiltinPresetSpec {
            name: BUILTIN_JULIA_CORAL,
            tooltip: "Load an organic Julia-set Mandelbulb preset with soft coral tones.",
            preset: FractalPreset {
                fractal_kind: 0,
                bulb_power: 7.0,
                bulb_julia: true,
                bulb_c: [-0.22, 0.62, 0.18],
                metallic: 0.30,
                roughness: 0.26,
                palette_shift: 0.22,
                sun_azimuth: None,
                sun_elevation: None,
                ..base
            },
        },
    ]
}

/// Regenerates all four built-in presets at the front of `store` in canonical display order.
pub fn ensure_builtins(store: &mut PresetStore<FractalPreset>) {
    // `ensure_builtin` inserts at index 0, so iterate in reverse to preserve display order.
    for spec in builtin_presets().into_iter().rev() {
        store.ensure_builtin(spec.name, spec.preset);
    }
}

/// Default on-disk location for user fractal presets (`~/.nv-rs/fractal_presets.json`).
pub fn presets_path() -> Option<PathBuf> {
    let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"))?;
    Some(
        PathBuf::from(home)
            .join(".nv-rs")
            .join("fractal_presets.json"),
    )
}

/// Serialized JSON file envelope (stores only user presets; built-ins regenerate from code).
#[derive(Serialize)]
struct FractalPresetsDisk<'a> {
    version: u32,
    selected: &'a str,
    presets: Vec<Preset<FractalPreset>>,
}

/// Result of reading the fractal presets file from disk.
#[derive(Debug)]
pub struct LoadReport {
    /// Loaded preset store (always contains the 4 canonical built-ins at the front).
    pub store: PresetStore<FractalPreset>,
    /// `Some` when the file exists but could not be read or parsed cleanly.
    pub problem: Option<String>,
}

/// Loads fractal presets from `path`, keeping valid entries even if one slot is malformed and
/// always regenerating the four canonical built-ins at the front.
pub fn load_from(path: &Path) -> LoadReport {
    let mut store = PresetStore::default();
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            ensure_builtins(&mut store);
            return LoadReport {
                store,
                problem: None,
            };
        }
        Err(e) => {
            ensure_builtins(&mut store);
            return LoadReport {
                store,
                problem: Some(format!("cannot read '{}': {e}", path.display())),
            };
        }
    };

    let root: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            ensure_builtins(&mut store);
            return LoadReport {
                store,
                problem: Some(format!(
                    "'{}' contains invalid JSON ({e}); built-in presets loaded",
                    path.display()
                )),
            };
        }
    };

    let Some(obj) = root.as_object() else {
        ensure_builtins(&mut store);
        return LoadReport {
            store,
            problem: Some(format!(
                "'{}' top-level JSON value is not an object; built-in presets loaded",
                path.display()
            )),
        };
    };

    let mut errors: Vec<String> = Vec::new();
    let mut selected = String::new();
    if let Some(sel_val) = obj.get("selected") {
        match sel_val.as_str() {
            Some(s) => selected = s.trim().to_string(),
            None => errors.push("field 'selected' is not a string".to_string()),
        }
    }

    ensure_builtins(&mut store);

    if let Some(presets_val) = obj.get("presets") {
        match presets_val.as_array() {
            Some(arr) => {
                for (idx, item) in arr.iter().enumerate() {
                    match serde_json::from_value::<Preset<FractalPreset>>(item.clone()) {
                        Ok(preset) => {
                            let name = preset.name.trim();
                            if name.is_empty() || preset.builtin || store.is_builtin(name) {
                                continue;
                            }
                            let payload = preset.payload.sanitized();
                            upsert_user_case_insensitive(&mut store, name, payload);
                        }
                        Err(e) => errors.push(format!("preset[{idx}]: {e}")),
                    }
                }
            }
            None => errors.push("field 'presets' is not a JSON array".to_string()),
        }
    }

    // Restore selection only when it resolves to an actual preset in the store.
    if let Some(canonical_name) = store
        .presets
        .iter()
        .find(|p| same_name(&p.name, &selected))
        .map(|p| p.name.clone())
    {
        store.selected = canonical_name.clone();
        store.name_buf = canonical_name;
    }

    let problem = (!errors.is_empty()).then(|| {
        format!(
            "'{}' partially readable ({})",
            path.display(),
            errors.join("; ")
        )
    });
    LoadReport { store, problem }
}

/// Inserts or updates a user preset case-insensitively in-place, preserving display order and
/// avoiding duplicate case variants.
fn upsert_user_case_insensitive(
    store: &mut PresetStore<FractalPreset>,
    name: &str,
    payload: FractalPreset,
) {
    if let Some(slot) = store
        .presets
        .iter_mut()
        .find(|p| !p.builtin && same_name(&p.name, name))
    {
        slot.name = name.to_string();
        slot.payload = payload;
    } else {
        store.upsert(name.to_string(), payload);
    }
}

/// Atomically writes all user presets in `store` to `path` via a sibling `.part` file +
/// `flush()` + `sync_all()` + `std::fs::rename`.
pub fn save_to(path: &Path, store: &PresetStore<FractalPreset>) -> std::io::Result<()> {
    if let Some(dir) = path.parent()
        && !dir.as_os_str().is_empty()
    {
        std::fs::create_dir_all(dir)?;
    }

    let user_presets: Vec<Preset<FractalPreset>> = store
        .presets
        .iter()
        .filter(|p| !p.builtin)
        .cloned()
        .map(|mut p| {
            p.payload = p.payload.sanitized();
            p
        })
        .collect();

    let envelope = FractalPresetsDisk {
        version: PRESET_FILE_VERSION,
        selected: &store.selected,
        presets: user_presets,
    };

    let json = serde_json::to_string_pretty(&envelope).map_err(std::io::Error::other)?;
    let seq = ATOMIC_TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let tmp = sibling_part_path(path, seq);

    let write_res = (|| -> std::io::Result<()> {
        let mut file = std::fs::File::create(&tmp)?;
        file.write_all(json.as_bytes())?;
        file.flush()?;
        file.sync_all()?;
        Ok(())
    })();

    if let Err(err) = write_res {
        if let Err(rm_err) = std::fs::remove_file(&tmp) {
            eprintln!(
                "nv-demo fractal_presets cleanup failed for '{}': {rm_err}",
                tmp.display()
            );
        }
        return Err(err);
    }

    if let Err(err) = std::fs::rename(&tmp, path) {
        if let Err(rm_err) = std::fs::remove_file(&tmp) {
            eprintln!(
                "nv-demo fractal_presets cleanup failed for '{}': {rm_err}",
                tmp.display()
            );
        }
        return Err(err);
    }
    Ok(())
}

fn sibling_part_path(path: &Path, seq: u64) -> PathBuf {
    let mut name = path
        .file_name()
        .map(std::ffi::OsStr::to_os_string)
        .unwrap_or_else(|| std::ffi::OsString::from("fractal_presets.json"));
    name.push(format!(".part.{}.{seq}", std::process::id()));
    path.with_file_name(name)
}

/// Short inline status feedback displayed in the `Fractal -> Presets` section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PresetStatus {
    /// Informational feedback (e.g. saved / deleted).
    Info(String),
    /// Non-fatal warning or I/O error.
    Warning(String),
}

/// Host state for Fractal preset management, owned by `DemoApp` and borrowed by `UiContext`.
#[derive(Debug)]
pub struct FractalPresetManager {
    /// Underlying `sl_controls::preset` store (4 built-ins + persisted user slots).
    pub store: PresetStore<FractalPreset>,
    /// Target JSON path (`None` disables disk writes, e.g. in headless tests).
    path: Option<PathBuf>,
    /// Latest user-visible status or error line.
    pub status: Option<PresetStatus>,
    /// True when startup found an unreadable/corrupt file on disk; backed up before first save.
    corrupt_on_load: bool,
}

impl Default for FractalPresetManager {
    fn default() -> Self {
        Self::load_from(None)
    }
}

impl FractalPresetManager {
    /// Loads presets from the default `~/.nv-rs/fractal_presets.json` path.
    pub fn load_default() -> Self {
        Self::load_from(presets_path())
    }

    /// Loads presets from `path` (or initializes an in-memory store with built-ins when `None`).
    pub fn load_from(path: Option<PathBuf>) -> Self {
        let report = match &path {
            Some(p) => load_from(p),
            None => {
                let mut store = PresetStore::default();
                ensure_builtins(&mut store);
                LoadReport {
                    store,
                    problem: None,
                }
            }
        };
        let corrupt_on_load = report.problem.is_some();
        let status = report.problem.map(PresetStatus::Warning);
        Self {
            store: report.store,
            path,
            status,
            corrupt_on_load,
        }
    }

    /// Applies the named preset (built-in or user) onto `settings`, updating selection state.
    /// Returns `true` if a preset with `name` existed and was applied.
    pub fn apply_preset(&mut self, name: &str, settings: &mut DemoSettings) -> bool {
        let Some((canonical_name, payload)) = self
            .store
            .presets
            .iter()
            .find(|p| same_name(&p.name, name))
            .map(|p| (p.name.clone(), p.payload))
        else {
            return false;
        };
        payload.apply_to(settings);
        self.store.selected = canonical_name.clone();
        self.store.name_buf = canonical_name;
        self.status = None;
        true
    }

    /// Captures the current fractal `settings` under `name`, refusing empty or built-in names,
    /// and atomically persists the store to disk.
    pub fn save_preset(
        &mut self,
        name: &str,
        settings: &DemoSettings,
    ) -> Result<(), PresetError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(PresetError::Empty);
        }
        if self.store.is_builtin(trimmed) {
            self.status = Some(PresetStatus::Warning(
                "Built-in presets are read-only; enter a new name.".to_string(),
            ));
            return Err(PresetError::Builtin);
        }
        let existed = self.store.taken(trimmed);
        let payload = FractalPreset::from_settings(settings).sanitized();
        upsert_user_case_insensitive(&mut self.store, trimmed, payload);
        self.store.selected = trimmed.to_string();
        self.store.name_buf = trimmed.to_string();
        self.persist_with_status(if existed {
            format!("Updated preset '{trimmed}'")
        } else {
            format!("Saved preset '{trimmed}'")
        });
        Ok(())
    }

    /// Deletes a user preset by `name`, refusing empty, missing, or built-in names, and
    /// atomically persists the store to disk.
    pub fn delete_preset(&mut self, name: &str) -> Result<(), PresetError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(PresetError::Empty);
        }
        if self.store.is_builtin(trimmed) {
            self.status = Some(PresetStatus::Warning(
                "Built-in presets cannot be deleted.".to_string(),
            ));
            return Err(PresetError::Builtin);
        }
        let Some(canonical) = self
            .store
            .presets
            .iter()
            .find(|p| same_name(&p.name, trimmed))
            .map(|p| p.name.clone())
        else {
            return Err(PresetError::Missing);
        };
        self.store.remove(&canonical);
        if same_name(&self.store.name_buf, &canonical) {
            self.store.name_buf.clear();
        }
        self.persist_with_status(format!("Deleted preset '{canonical}'"));
        Ok(())
    }

    /// Dispatches a [`PresetAction`] emitted by [`sl_controls::preset::preset_bar`].
    /// Returns `true` if `settings` were modified (i.e. a preset was loaded).
    pub fn handle_bar_action(&mut self, action: PresetAction, settings: &mut DemoSettings) -> bool {
        match action {
            PresetAction::Load(name) => self.apply_preset(&name, settings),
            PresetAction::Save(name) => {
                if let Err(err) = self.save_preset(&name, settings)
                    && self.status.is_none()
                {
                    self.status = Some(PresetStatus::Warning(err.to_string()));
                }
                false
            }
            PresetAction::Delete(name) => {
                if let Err(err) = self.delete_preset(&name)
                    && self.status.is_none()
                {
                    self.status = Some(PresetStatus::Warning(err.to_string()));
                }
                false
            }
        }
    }

    /// Draws the compact `sl_controls::preset::preset_bar` strictly clamped to `ui.available_width()`
    /// so it can never expand parent panel width or trigger layout staircase in narrow docks.
    /// Returns `true` when `settings` were changed by loading a preset.
    pub fn draw_preset_bar(&mut self, ui: &mut egui::Ui, settings: &mut DemoSettings) -> bool {
        let avail_w = ui.available_width().max(1.0);
        let bar_h = ui.spacing().interact_size.y + 4.0;
        let action = crate::ui::allocate_clamped_ui(
            ui,
            egui::vec2(avail_w, bar_h),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                preset_bar(ui, &mut self.store, "nv_demo_fractal_presets")
            },
        );
        match action {
            Some(act) => self.handle_bar_action(act, settings),
            None => false,
        }
    }

    /// Iterates over all user-created (non-builtin) presets in display order.
    pub fn user_presets(&self) -> impl Iterator<Item = &Preset<FractalPreset>> {
        self.store.presets.iter().filter(|p| !p.builtin)
    }

    /// Returns `true` when `name` is the currently selected preset.
    pub fn is_selected(&self, name: &str) -> bool {
        !self.store.selected.is_empty() && same_name(&self.store.selected, name)
    }

    /// Returns `true` when a preset is selected and the live fractal `settings` differ from it.
    pub fn is_selected_modified(&self, settings: &DemoSettings) -> bool {
        if self.store.selected.is_empty() {
            return false;
        }
        self.store
            .get(&self.store.selected)
            .is_some_and(|p| !p.matches_settings(settings))
    }

    fn persist_with_status(&mut self, ok_msg: String) {
        let Some(ref path) = self.path else {
            self.status = Some(PresetStatus::Info(ok_msg));
            return;
        };
        if self.corrupt_on_load {
            let backup = path.with_extension("json.corrupt");
            if let Err(e) = std::fs::copy(path, &backup) {
                eprintln!(
                    "nv-demo fractal_presets corrupt backup failed ('{}'): {e}",
                    backup.display()
                );
            }
            self.corrupt_on_load = false;
        }
        match save_to(path, &self.store) {
            Ok(()) => self.status = Some(PresetStatus::Info(ok_msg)),
            Err(e) => {
                self.status = Some(PresetStatus::Warning(format!(
                    "Preset updated in memory; disk save failed: {e}"
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(tag: &str) -> PathBuf {
        let seq = ATOMIC_TMP_SEQ.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "nv_demo_fractal_presets_{tag}_{}_{seq}",
            std::process::id()
        ))
    }

    #[test]
    fn builtins_present_in_canonical_order_and_match_original_values() {
        let mut mgr = FractalPresetManager::load_from(None);
        let names: Vec<String> = mgr.store.presets.iter().map(|p| p.name.clone()).collect();
        assert_eq!(
            names,
            vec![
                BUILTIN_GOLD_BULB,
                BUILTIN_CHROME_BOX,
                BUILTIN_OBSIDIAN_KIFS,
                BUILTIN_JULIA_CORAL,
            ]
        );
        for name in &names {
            assert!(mgr.store.is_builtin(name), "{name} must be builtin");
        }

        let mut s = DemoSettings::default();
        assert!(mgr.apply_preset(BUILTIN_GOLD_BULB, &mut s));
        assert_eq!(s.fractal_kind, 0);
        assert_eq!(s.bulb_power, 8.0);
        assert!(!s.bulb_julia);
        assert_eq!(s.metallic, 0.78);
        assert_eq!(s.roughness, 0.14);
        assert_eq!(s.palette_shift, 0.08);
        assert_eq!(s.sun_azimuth, 36.0);
        assert_eq!(s.sun_elevation, 42.0);

        assert!(mgr.apply_preset(BUILTIN_CHROME_BOX, &mut s));
        assert_eq!(s.fractal_kind, 1);
        assert_eq!(s.box_scale, -2.0);
        assert_eq!(s.box_fold, 1.0);
        assert_eq!(s.metallic, 0.92);
        assert_eq!(s.roughness, 0.10);
        assert_eq!(s.palette_shift, 0.45);
        assert_eq!(s.sun_azimuth, -45.0);
        assert_eq!(s.sun_elevation, 35.0);

        assert!(mgr.apply_preset(BUILTIN_OBSIDIAN_KIFS, &mut s));
        assert_eq!(s.fractal_kind, 2);
        assert_eq!(s.kifs_scale, 2.0);
        assert_eq!(s.kifs_rot1, 18.0);
        assert_eq!(s.kifs_rot2, -12.0);
        assert_eq!(s.metallic, 0.65);
        assert_eq!(s.roughness, 0.22);
        assert_eq!(s.palette_shift, 0.72);
        assert_eq!(s.sun_azimuth, 55.0);
        assert_eq!(s.sun_elevation, 28.0);

        // Julia Coral preserves existing sun angles when applied.
        s.sun_azimuth = 11.0;
        s.sun_elevation = 66.0;
        assert!(mgr.apply_preset(BUILTIN_JULIA_CORAL, &mut s));
        assert_eq!(s.fractal_kind, 0);
        assert_eq!(s.bulb_power, 7.0);
        assert!(s.bulb_julia);
        assert_eq!(s.bulb_c, [-0.22, 0.62, 0.18]);
        assert_eq!(s.metallic, 0.30);
        assert_eq!(s.roughness, 0.26);
        assert_eq!(s.palette_shift, 0.22);
        assert_eq!(s.sun_azimuth, 11.0);
        assert_eq!(s.sun_elevation, 66.0);
    }

    #[test]
    fn save_load_roundtrip_preserves_all_40_fractal_fields() {
        let dir = temp_dir("roundtrip");
        let path = dir.join("fractal_presets.json");

        let custom = DemoSettings {
            fractal_kind: 2,
            fractal_translate: [12.5, -3.25, 44.0],
            obj_scale: 3.75,
            rot_x: 15.0,
            rot_y: -72.5,
            rot_z: 130.0,
            bulb_power: 11.25,
            bulb_bailout: 19.5,
            bulb_iter: 24,
            bulb_julia: true,
            bulb_c: [0.45, -0.35, 0.75],
            bulb_theta_mult: 1.45,
            bulb_phi_mult: 0.85,
            bulb_theta_phase: 22.5,
            bulb_phi_phase: -64.0,
            box_scale: -2.45,
            box_fold: 1.35,
            box_min_r: 0.32,
            box_fixed_r: 1.45,
            box_iter: 26,
            box_bailout: 88.0,
            box_fold_mult: 2.35,
            box_iter_rot: 14.5,
            kifs_scale: 2.65,
            kifs_rot1: 42.0,
            kifs_rot2: -33.5,
            kifs_offset: [0.85, 1.25, 0.65],
            kifs_iter: 20,
            kifs_bailout: 24.0,
            kifs_thickness: 0.085,
            kifs_z_bias: 0.72,
            detail_eps: 0.00045,
            fudge: 0.72,
            norm_eps: 0.00035,
            march_steps: 384,
            bounces: 7,
            metallic: 0.84,
            roughness: 0.19,
            trap_scale: 1.65,
            palette_shift: 0.61,
            // Non-fractal fields should NOT be overwritten on load.
            sun_azimuth: 120.0,
            sun_elevation: 19.0,
            ..DemoSettings::default()
        };

        let mut mgr = FractalPresetManager::load_from(Some(path.clone()));
        mgr.save_preset("My Custom Look", &custom)
            .expect("save user preset");

        // Ensure no leftover `.part` temp files remain in the directory.
        let leftover_parts: Vec<_> = std::fs::read_dir(&dir)
            .expect("read_dir")
            .filter_map(Result::ok)
            .filter(|e| e.file_name().to_string_lossy().contains(".part."))
            .collect();
        assert!(leftover_parts.is_empty(), "leftover .part files found");

        // Reload into a fresh manager from disk.
        let mut reloaded = FractalPresetManager::load_from(Some(path.clone()));
        let _ = std::fs::remove_dir_all(&dir);

        assert_eq!(reloaded.status, None);
        assert_eq!(reloaded.store.selected, "My Custom Look");
        assert_eq!(reloaded.user_presets().count(), 1);

        let mut applied = DemoSettings {
            sun_azimuth: -10.0,
            sun_elevation: 55.0,
            ..DemoSettings::default()
        };
        assert!(reloaded.apply_preset("My Custom Look", &mut applied));

        assert_eq!(
            FractalPreset::from_settings(&applied),
            FractalPreset::from_settings(&custom)
        );
        // User preset must not clobber existing sun angles.
        assert_eq!(applied.sun_azimuth, -10.0);
        assert_eq!(applied.sun_elevation, 55.0);
        assert!(
            reloaded
                .store
                .get("My Custom Look")
                .expect("stored preset")
                .matches_settings(&applied)
        );
    }

    #[test]
    fn overwrite_and_delete_user_presets_handle_case_and_persist() {
        let dir = temp_dir("overwrite_delete");
        let path = dir.join("fractal_presets.json");

        let mut mgr = FractalPresetManager::load_from(Some(path.clone()));
        let s1 = DemoSettings {
            bulb_power: 9.0,
            ..DemoSettings::default()
        };
        mgr.save_preset("Warm Bulb", &s1).expect("save first");

        let s2 = DemoSettings {
            bulb_power: 12.0,
            ..DemoSettings::default()
        };
        mgr.save_preset("warm bulb", &s2)
            .expect("case-insensitive overwrite");

        assert_eq!(mgr.user_presets().count(), 1, "no duplicate case variants");
        assert_eq!(
            mgr.store.get("WARM BULB").expect("present").bulb_power,
            12.0
        );

        mgr.delete_preset("WARM BULB").expect("delete user preset");
        assert_eq!(mgr.user_presets().count(), 0);
        assert!(mgr.store.selected.is_empty());
        assert!(mgr.store.name_buf.is_empty());

        let reloaded = FractalPresetManager::load_from(Some(path));
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(reloaded.user_presets().count(), 0);
        assert_eq!(reloaded.store.presets.len(), 4);
    }

    #[test]
    fn builtin_presets_cannot_be_overwritten_or_deleted() {
        let mut mgr = FractalPresetManager::load_from(None);
        let s = DemoSettings {
            bulb_power: 15.0,
            ..DemoSettings::default()
        };

        assert_eq!(
            mgr.save_preset("Gold Bulb", &s),
            Err(PresetError::Builtin)
        );
        assert_eq!(
            mgr.save_preset("  gold bulb ", &s),
            Err(PresetError::Builtin)
        );
        assert_eq!(
            mgr.delete_preset("Chrome Box"),
            Err(PresetError::Builtin)
        );
        assert_eq!(
            mgr.delete_preset("OBSIDIAN KIFS"),
            Err(PresetError::Builtin)
        );
        assert_eq!(mgr.save_preset("   ", &s), Err(PresetError::Empty));
        assert_eq!(mgr.delete_preset("NoSuchPreset"), Err(PresetError::Missing));

        // Built-in payload remained untouched.
        assert_eq!(
            mgr.store.get(BUILTIN_GOLD_BULB).expect("gold bulb").bulb_power,
            8.0
        );
        assert_eq!(mgr.store.presets.len(), 4);
    }

    #[test]
    fn missing_and_corrupt_files_recover_cleanly_and_backup_corrupt_before_write() {
        let dir = temp_dir("corrupt");
        let missing_path = dir.join("does_not_exist.json");
        let missing = load_from(&missing_path);
        assert_eq!(missing.problem, None);
        assert_eq!(missing.store.presets.len(), 4);

        std::fs::create_dir_all(&dir).expect("mkdir");
        let corrupt_path = dir.join("fractal_presets.json");
        let garbage = r#"{"version":1,"presets":[{"name":"Broken"#;
        std::fs::write(&corrupt_path, garbage).expect("write corrupt");

        let mut mgr = FractalPresetManager::load_from(Some(corrupt_path.clone()));
        assert!(matches!(mgr.status, Some(PresetStatus::Warning(_))));
        assert_eq!(mgr.store.presets.len(), 4);
        // Corrupt file must stay untouched on load.
        assert_eq!(
            std::fs::read_to_string(&corrupt_path).expect("read corrupt"),
            garbage
        );

        // Explicit user save backs up the corrupt file before writing the valid JSON.
        mgr.save_preset("Fresh", &DemoSettings::default())
            .expect("save fresh");
        let backup = corrupt_path.with_extension("json.corrupt");
        assert_eq!(
            std::fs::read_to_string(&backup).expect("read backup"),
            garbage
        );
        let reloaded = load_from(&corrupt_path);
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(reloaded.problem, None);
        assert!(reloaded.store.get("Fresh").is_some());
    }

    #[test]
    fn partial_corrupt_entry_keeps_valid_user_presets() {
        let dir = temp_dir("partial");
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("fractal_presets.json");
        let json = r#"{
            "version": 1,
            "selected": "Good One",
            "presets": [
                {"name": "Good One", "payload": {"fractal_kind": 1, "box_scale": -2.5}},
                {"name": 12345, "payload": "bad"}
            ]
        }"#;
        std::fs::write(&path, json).expect("write");

        let report = load_from(&path);
        let _ = std::fs::remove_dir_all(&dir);
        assert!(report.problem.is_some());
        assert_eq!(report.store.selected, "Good One");
        assert_eq!(
            report.store.get("Good One").expect("good preset").box_scale,
            -2.5
        );
        assert_eq!(report.store.presets.len(), 5);
    }

    #[test]
    fn sanitized_clamps_out_of_range_and_non_finite_values() {
        let bad = FractalPreset {
            fractal_kind: 99,
            fractal_translate: [f32::NAN, 9999.0, -9999.0],
            obj_scale: f32::INFINITY,
            rot_x: 720.0,
            bulb_power: -5.0,
            bulb_iter: 0,
            box_iter: 999,
            kifs_iter: 1,
            detail_eps: f32::NEG_INFINITY,
            march_steps: 2,
            bounces: 100,
            metallic: 5.0,
            roughness: -2.0,
            sun_azimuth: Some(f32::NAN),
            sun_elevation: Some(250.0),
            ..FractalPreset::default()
        };
        let s = bad.sanitized();
        let d = FractalPreset::default();
        assert_eq!(s.fractal_kind, 2);
        assert_eq!(s.fractal_translate, [d.fractal_translate[0], 500.0, -500.0]);
        assert_eq!(s.obj_scale, d.obj_scale);
        assert_eq!(s.rot_x, 180.0);
        assert_eq!(s.bulb_power, 2.0);
        assert_eq!(s.bulb_iter, 4);
        assert_eq!(s.box_iter, 32);
        assert_eq!(s.kifs_iter, 4);
        assert_eq!(s.detail_eps, d.detail_eps);
        assert_eq!(s.march_steps, 32);
        assert_eq!(s.bounces, 16);
        assert_eq!(s.metallic, 1.0);
        assert_eq!(s.roughness, 0.02);
        assert_eq!(s.sun_azimuth, None);
        assert_eq!(s.sun_elevation, Some(88.0));
    }

    #[test]
    fn preset_bar_never_overflows_narrow_dock_widths() {
        let ctx = egui::Context::default();
        let mut mgr = FractalPresetManager::load_from(None);
        let mut settings = DemoSettings::default();
        mgr.save_preset("Very Long Custom Fractal Preset Name", &settings)
            .expect("save long preset");

        for panel_w in [240.0_f32, 300.0, 360.0] {
            let mut avail_before = 0.0_f32;
            let mut avail_after = 0.0_f32;
            let mut bar_right = 0.0_f32;
            let mut origin_x = 0.0_f32;

            let raw = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(panel_w, 400.0),
                )),
                ..Default::default()
            };
            let out = ctx.run_ui(raw, |ui| {
                ui.set_min_width(panel_w);
                ui.set_max_width(panel_w);
                origin_x = ui.max_rect().left();
                avail_before = ui.available_width();
                let resp = ui.scope(|ui| {
                    mgr.draw_preset_bar(ui, &mut settings);
                });
                bar_right = resp.response.rect.right();
                avail_after = ui.available_width();
            });
            out.drop_without_applying_deltas();

            assert!(
                avail_after <= avail_before + 0.5,
                "width {panel_w}: preset_bar expanded available_width ({avail_before} -> {avail_after})"
            );
            assert!(
                bar_right <= origin_x + panel_w + 0.5,
                "width {panel_w}: preset_bar overflowed right edge ({bar_right} > {})",
                origin_x + panel_w
            );
        }
    }
}
