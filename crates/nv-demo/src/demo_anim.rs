//! Sinusoidal parameter animator ("Demo Mode") for float attributes in `DemoSettings`.
//!
//! What: maintains a set of animated parameter keys (`active`), global period `N` seconds,
//! amplitude multiplier `amp`, and master `enabled` toggle, evaluating
//! `clamp(base + 0.10 * (max - min) * amp * sin(2*pi*t / N), min, max)` onto a cloned `DemoSettings` each frame.
//! Why: keeps `DemoSettings` as the user's editable base values so enabling/disabling animation
//! or scrubbing sliders never overwrites or loses the center of oscillation.
//! Used by: `crate::main`, `crate::ui`, `crate::persist`.

use std::collections::BTreeSet;
use std::f32::consts::TAU;
use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

use crate::renderer::DemoSettings;

/// One animatable `DemoSettings` parameter: its persisted key and its `[min, max]` range, which
/// is both the slider range and the oscillation clamp.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnimParam {
    pub key: &'static str,
    pub min: f32,
    pub max: f32,
}

/// Range of the animatable parameter `key` (the single source for animator and UI sliders), or
/// `None` if `key` is not animatable.
pub fn anim_range(key: &str) -> Option<RangeInclusive<f32>> {
    ANIM_PARAMS
        .iter()
        .find(|p| p.key == key)
        .map(|p| p.min..=p.max)
}

/// Equality compares the persisted fields only: `time_s` is skipped by serde, so including it
/// would mark an unchanged config dirty on every save tick while the animator runs.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct DemoAnimator {
    pub playing: bool,
    pub enabled: bool,
    pub period_s: f32,
    pub amp: f32,
    pub active: BTreeSet<String>,
    #[serde(skip)]
    pub time_s: f64,
}

impl Default for DemoAnimator {
    fn default() -> Self {
        Self {
            playing: false,
            enabled: false,
            period_s: 8.0,
            amp: 1.0,
            active: BTreeSet::new(),
            time_s: 0.0,
        }
    }
}

impl PartialEq for DemoAnimator {
    fn eq(&self, other: &Self) -> bool {
        self.playing == other.playing
            && self.enabled == other.enabled
            && self.period_s == other.period_s
            && self.amp == other.amp
            && self.active == other.active
    }
}

impl DemoAnimator {
    pub fn is_running(&self) -> bool {
        self.playing && self.enabled && !self.active.is_empty()
    }

    pub fn is_active(&self, key: &str) -> bool {
        self.active.contains(key)
    }

    /// Arms or disarms `key`. Keys missing from [`ANIM_PARAMS`] are rejected (a debug assertion
    /// flags the caller's typo) so `active` never holds a key that silently does nothing.
    pub fn toggle(&mut self, key: &str) {
        if !self.active.remove(key) {
            let known = anim_range(key).is_some();
            debug_assert!(known, "unknown DemoAnimator key `{key}`");
            if !known {
                return;
            }
            self.active.insert(key.to_string());
            if !self.enabled {
                self.enabled = true;
                self.time_s = 0.0;
            }
            self.playing = true;
        }
    }

    /// Toggles master playback; if resuming with no active target, enables `Params` (when keys
    /// are armed) or `auto_rotate` so Play always produces visible motion.
    pub fn toggle_play(&mut self, settings: &mut DemoSettings) {
        let next = !self.playing;
        self.playing = next;
        if next
            && !settings.auto_rotate
            && !(settings.sat_on && settings.sat_orbit)
            && !self.enabled
        {
            if !self.active.is_empty() {
                self.enabled = true;
            } else {
                settings.auto_rotate = true;
            }
        }
    }

    pub fn tick(&mut self, dt: f32) {
        if self.is_running() && dt.is_finite() && dt > 0.0 {
            self.time_s += f64::from(dt);
        }
    }

    /// Evaluates sinusoidal offsets for all active parameters onto a copy of `base`.
    pub fn apply(&self, base: &DemoSettings) -> DemoSettings {
        if !self.is_running() {
            return *base;
        }
        let mut out = *base;
        let period = self.period_s.max(0.1);
        let phase = TAU * ((self.time_s as f32) / period);

        for key in &self.active {
            apply_key(&mut out, key, phase, self.amp);
        }
        out
    }
}

fn anim_f32(val: &mut f32, amp: f32, phase: f32, min: f32, max: f32) {
    let excursion = 0.10 * (max - min) * amp;
    *val = (*val + excursion * phase.sin()).clamp(min, max);
}

fn anim_vec3(v: &mut [f32; 3], amp: f32, phase: f32, min: f32, max: f32) {
    let excursion = 0.10 * (max - min) * amp;
    let offsets = [0.0, TAU / 3.0, 2.0 * TAU / 3.0];
    for (item, off) in v.iter_mut().zip(offsets) {
        *item = (*item + excursion * (phase + off).sin()).clamp(min, max);
    }
}

/// Generates [`ANIM_PARAMS`] and `apply_key` from ONE list, so a key, its `DemoSettings`
/// field and its range cannot drift apart.
macro_rules! anim_params {
    ($($key:literal => $anim:ident($($field:ident).+), $min:expr, $max:expr;)*) => {
        /// Every animatable key with its range: the single source for the animator and the UI.
        pub const ANIM_PARAMS: &[AnimParam] = &[$(AnimParam { key: $key, min: $min, max: $max }),*];

        /// Applies the oscillation of `key`; returns `false` for a key not in [`ANIM_PARAMS`]
        /// (only reachable through a stale persisted `active` set).
        fn apply_key(s: &mut DemoSettings, key: &str, phase: f32, amp: f32) -> bool {
            match key {
                $($key => $anim(&mut s.$($field).+, amp, phase, $min, $max),)*
                _ => return false,
            }
            true
        }
    };
}

anim_params! {
    // Mandelbulb
    "bulb_power" => anim_f32(bulb_power), 2.0, 16.0;
    "bulb_bailout" => anim_f32(bulb_bailout), 2.0, 32.0;
    "bulb_theta_mult" => anim_f32(bulb_theta_mult), 0.25, 2.5;
    "bulb_phi_mult" => anim_f32(bulb_phi_mult), 0.25, 2.5;
    "bulb_theta_phase" => anim_f32(bulb_theta_phase), -90.0, 90.0;
    "bulb_phi_phase" => anim_f32(bulb_phi_phase), -180.0, 180.0;
    "bulb_c" => anim_vec3(bulb_c), -1.5, 1.5;
    // Mandelbox
    "box_scale" => anim_f32(box_scale), -3.5, 3.5;
    "box_fold" => anim_f32(box_fold), 0.2, 2.2;
    "box_fold_mult" => anim_f32(box_fold_mult), 1.2, 3.0;
    "box_min_r" => anim_f32(box_min_r), 0.05, 1.2;
    "box_fixed_r" => anim_f32(box_fixed_r), 0.4, 2.5;
    "box_iter_rot" => anim_f32(box_iter_rot), -45.0, 45.0;
    "box_bailout" => anim_f32(box_bailout), 4.0, 128.0;
    // KIFS Octahedron
    "kifs_scale" => anim_f32(kifs_scale), 1.3, 3.5;
    "kifs_rot1" => anim_f32(kifs_rot1), -180.0, 180.0;
    "kifs_rot2" => anim_f32(kifs_rot2), -180.0, 180.0;
    "kifs_offset" => anim_vec3(kifs_offset), 0.1, 2.0;
    "kifs_thickness" => anim_f32(kifs_thickness), 0.005, 0.25;
    "kifs_z_bias" => anim_f32(kifs_z_bias), 0.0, 1.5;
    "kifs_bailout" => anim_f32(kifs_bailout), 4.0, 32.0;
    // Transform
    "fractal_translate" => anim_vec3(fractal_translate), -500.0, 500.0;
    "obj_scale" => anim_f32(obj_scale), 0.05, 50.0;
    "rot_x" => anim_f32(rot_x), -180.0, 180.0;
    "rot_y" => anim_f32(rot_y), -180.0, 180.0;
    "rot_z" => anim_f32(rot_z), -180.0, 180.0;
    // Detail & Shading
    "detail_eps" => anim_f32(detail_eps), 0.00002, 0.005;
    "norm_eps" => anim_f32(norm_eps), 0.00002, 0.005;
    "fudge" => anim_f32(fudge), 0.3, 1.1;
    "metallic" => anim_f32(metallic), 0.0, 1.0;
    "roughness" => anim_f32(roughness), 0.02, 0.85;
    "trap_scale" => anim_f32(trap_scale), 0.1, 3.0;
    "palette_shift" => anim_f32(palette_shift), 0.0, 1.0;
    // Sky & Lighting
    "sun_intensity" => anim_f32(sun_intensity), 0.0, 4.0;
    "sun_azimuth" => anim_f32(sun_azimuth), -180.0, 180.0;
    "sun_elevation" => anim_f32(sun_elevation), 2.0, 88.0;
    "sky_intensity" => anim_f32(sky_intensity), 0.1, 2.0;
    "horizon_haze" => anim_f32(horizon_haze), 0.4, 2.5;
    "exposure" => anim_f32(exposure), 0.25, 2.0;
    // Environment (per-sky_mode)
    "env_cloud_cover" => anim_f32(env.cloud_cover), 0.0, 1.0;
    "env_cloud_density" => anim_f32(env.cloud_density), 0.0, 1.5;
    "env_softbox_power" => anim_f32(env.softbox_power), 0.2, 3.0;
    "env_softbox_softness" => anim_f32(env.softbox_softness), 0.02, 0.5;
    "env_nebula_density" => anim_f32(env.nebula_density), 0.2, 2.5;
    "env_nebula_contrast" => anim_f32(env.nebula_contrast), 0.5, 3.0;
    "env_nebula_dust" => anim_f32(env.nebula_dust), 0.0, 3.0;
    "env_nebula_scale" => anim_f32(env.nebula_scale), 0.5, 4.0;
    "env_star_density" => anim_f32(env.star_density), 0.0, 2.0;
    "env_star_brightness" => anim_f32(env.star_brightness), 0.0, 4.0;
    "env_grid_glow" => anim_f32(env.grid_glow), 0.1, 3.0;
    "env_grid_density" => anim_f32(env.grid_density), 4.0, 32.0;
    "env_elev_cloud_cover" => anim_f32(env.elev_cloud_cover), 0.0, 1.0;
    "env_elev_fog" => anim_f32(env.elev_fog), 0.2, 2.5;
    "env_city_fog" => anim_f32(env.city_fog), 0.2, 2.5;
    "env_city_windows" => anim_f32(env.city_windows), 0.0, 3.0;
    "env_colony_glow" => anim_f32(env.colony_glow), 0.0, 3.0;
    "env_planet_limb" => anim_f32(env.planet_limb), 0.0, 2.5;
    // Lens DoF
    "aperture" => anim_f32(aperture), 0.0, 0.12;
    "focus_dist" => anim_f32(focus_dist), 0.5, 12.0;
    // Floor
    "floor_roughness" => anim_f32(floor_roughness), 0.03, 0.80;
    "floor_scratch" => anim_f32(floor_scratch), 0.0, 1.5;
    "floor_scale" => anim_f32(floor_scale), 0.3, 5.0;
    "floor_metallic" => anim_f32(floor_metallic), 0.0, 1.0;
    // Scene / Spheres
    "sat_size_min" => anim_f32(sat_size_min), 0.001, 0.80;
    "sat_size_max" => anim_f32(sat_size_max), 0.001, 0.80;
    "sat_orbit_min" => anim_f32(sat_orbit_min), 0.6, 4.5;
    "sat_orbit_max" => anim_f32(sat_orbit_max), 0.6, 4.5;
    "sat_height_min" => anim_f32(sat_height_min), -1.0, 2.0;
    "sat_height_max" => anim_f32(sat_height_max), -1.0, 2.0;
    "sat_speed_min" => anim_f32(sat_speed_min), -3.0, 3.0;
    "sat_speed_max" => anim_f32(sat_speed_max), -3.0, 3.0;
    "sat_bob_amp_min" => anim_f32(sat_bob_amp_min), 0.0, 0.80;
    "sat_bob_amp_max" => anim_f32(sat_bob_amp_max), 0.0, 0.80;
    "sat_bob_freq_min" => anim_f32(sat_bob_freq_min), 0.0, 5.0;
    "sat_bob_freq_max" => anim_f32(sat_bob_freq_max), 0.0, 5.0;
    "sat_hue_spread" => anim_f32(sat_hue_spread), 0.0, 1.0;
    "glass_ior" => anim_f32(glass_ior), 1.05, 2.40;
    "glass_roughness" => anim_f32(glass_roughness), 0.0, 0.35;
    "glass_absorb" => anim_f32(glass_absorb), 0.0, 12.0;
    // Bloom & Split & Spin & DLSS
    "bloom_threshold" => anim_f32(bloom_threshold), 0.2, 5.0;
    "bloom_intensity" => anim_f32(bloom_intensity), 0.0, 2.0;
    "split_x" => anim_f32(split_x), 0.02, 0.98;
    "spin_speed" => anim_f32(spin_speed), 0.0, 2.0;
    "dlss_sharpness" => anim_f32(dlss_sharpness), 0.0, 1.0;
    // CSG World
    "csg_wetness" => anim_f32(csg_wetness), 0.0, 1.0;
    "csg_traffic_speed" => anim_f32(csg_traffic_speed), 0.0, 3.0;
    "csg_lod_bias" => anim_f32(csg_lod_bias), 0.25, 4.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sine_oscillation_hits_plus_minus_ten_percent_and_clamps() {
        let base = DemoSettings {
            palette_shift: 0.95,
            ..Default::default()
        };
        let mut anim = DemoAnimator {
            playing: true,
            enabled: true,
            period_s: 4.0,
            amp: 1.0,
            active: [
                "bulb_power".to_string(),
                "metallic".to_string(),
                "palette_shift".to_string(),
            ]
            .into_iter()
            .collect(),
            time_s: 1.0, // quarter period -> sin(pi/2) = +1.0
        };
        let s_pos = anim.apply(&base);
        // bulb_power range is 2.0..=16.0 -> 10% = 1.4
        assert!((s_pos.bulb_power - (base.bulb_power + 1.4)).abs() < 1e-5);
        // metallic range is 0.0..=1.0 -> 10% = 0.1
        assert!((s_pos.metallic - (base.metallic + 0.1)).abs() < 1e-5);
        // palette_shift 0.95 + 0.1 clamps to 1.0
        assert!((s_pos.palette_shift - 1.0).abs() < 1e-5);

        anim.time_s = 3.0; // three-quarter period -> sin(3pi/2) = -1.0
        let s_neg = anim.apply(&base);
        assert!((s_neg.bulb_power - (base.bulb_power - 1.4)).abs() < 1e-5);
        assert!((s_neg.metallic - (base.metallic - 0.1)).abs() < 1e-5);
        assert!((s_neg.palette_shift - 0.85).abs() < 1e-5);
    }

    #[test]
    fn disabled_animator_preserves_base_settings() {
        let base = DemoSettings::default();
        let anim = DemoAnimator {
            playing: true,
            enabled: false,
            period_s: 4.0,
            amp: 1.0,
            active: ["bulb_power".to_string()].into_iter().collect(),
            time_s: 1.0,
        };
        assert_eq!(anim.apply(&base), base);
    }

    #[test]
    fn toggle_play_activates_target_when_none_selected() {
        let mut settings = DemoSettings::default();
        let mut anim = DemoAnimator::default();
        assert!(!anim.playing);
        assert!(!settings.auto_rotate);

        anim.toggle_play(&mut settings);
        assert!(anim.playing);
        assert!(settings.auto_rotate);

        anim.toggle_play(&mut settings);
        assert!(!anim.playing);
    }

    #[test]
    fn anim_table_is_unique_ordered_and_drives_every_key() {
        assert_eq!(ANIM_PARAMS.len(), 87);
        let base = DemoSettings::default();
        let mut seen = BTreeSet::new();
        for p in ANIM_PARAMS {
            assert!(seen.insert(p.key), "duplicate key {}", p.key);
            assert!(p.min.is_finite() && p.max.is_finite() && p.min < p.max, "{}", p.key);
            assert_eq!(anim_range(p.key), Some(p.min..=p.max));
            let mut hi = base;
            let mut lo = base;
            assert!(apply_key(&mut hi, p.key, TAU / 4.0, 1.0), "{}", p.key);
            assert!(apply_key(&mut lo, p.key, -TAU / 4.0, 1.0), "{}", p.key);
            assert!(hi != base || lo != base, "{} does not move its field", p.key);
        }
        assert_eq!(anim_range("bulb_powr"), None);
        let mut s = base;
        assert!(!apply_key(&mut s, "bulb_powr", TAU / 4.0, 1.0));
        assert_eq!(s, base);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "unknown DemoAnimator key")]
    fn toggle_rejects_unknown_key_in_debug() {
        DemoAnimator::default().toggle("bulb_powr");
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn toggle_rejects_unknown_key_in_release() {
        let mut anim = DemoAnimator::default();
        anim.toggle("bulb_powr");
        assert!(anim.active.is_empty());
        assert!(!anim.playing);
    }

    #[test]
    fn equality_ignores_runtime_clock() {
        let saved = DemoAnimator {
            playing: true,
            enabled: true,
            active: ["metallic".to_string()].into_iter().collect(),
            ..Default::default()
        };
        let mut running = saved.clone();
        running.tick(1.5);
        assert!(running.time_s > 0.0);
        assert_eq!(running, saved, "time_s is not persisted, so it must not mark dirty");
        running.amp = 0.5;
        assert_ne!(running, saved);
    }

    #[test]
    fn csg_lod_bias_oscillates_and_clamps_within_bounds() {
        let base = DemoSettings {
            csg_lod_bias: 1.0,
            ..Default::default()
        };
        let mut anim = DemoAnimator {
            playing: true,
            enabled: true,
            period_s: 4.0,
            amp: 1.0,
            active: ["csg_lod_bias".to_string()].into_iter().collect(),
            time_s: 1.0, // sin(+pi/2) = +1.0 -> +0.10 * (4.0 - 0.25) = +0.375
        };
        let hi = anim.apply(&base);
        assert!((hi.csg_lod_bias - 1.375).abs() < 1e-5);

        anim.time_s = 3.0; // sin(3pi/2) = -1.0 -> -0.375
        let lo = anim.apply(&base);
        assert!((lo.csg_lod_bias - 0.625).abs() < 1e-5);

        // Extreme base clamps cleanly inside `0.25..=4.0`.
        let edge_base = DemoSettings {
            csg_lod_bias: 0.30,
            ..Default::default()
        };
        assert!((anim.apply(&edge_base).csg_lod_bias - 0.25).abs() < 1e-5);
    }
}
