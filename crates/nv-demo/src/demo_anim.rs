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

use serde::{Deserialize, Serialize};

use crate::renderer::DemoSettings;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

impl DemoAnimator {
    pub fn is_running(&self) -> bool {
        self.playing && self.enabled && !self.active.is_empty()
    }

    pub fn is_active(&self, key: &str) -> bool {
        self.active.contains(key)
    }

    pub fn toggle(&mut self, key: &str) {
        if !self.active.remove(key) {
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

fn apply_key(s: &mut DemoSettings, key: &str, phase: f32, amp: f32) {
    match key {
        // Mandelbulb
        "bulb_power" => anim_f32(&mut s.bulb_power, amp, phase, 2.0, 16.0),
        "bulb_bailout" => anim_f32(&mut s.bulb_bailout, amp, phase, 2.0, 32.0),
        "bulb_theta_mult" => anim_f32(&mut s.bulb_theta_mult, amp, phase, 0.25, 2.5),
        "bulb_phi_mult" => anim_f32(&mut s.bulb_phi_mult, amp, phase, 0.25, 2.5),
        "bulb_theta_phase" => anim_f32(&mut s.bulb_theta_phase, amp, phase, -90.0, 90.0),
        "bulb_phi_phase" => anim_f32(&mut s.bulb_phi_phase, amp, phase, -180.0, 180.0),
        "bulb_c" => anim_vec3(&mut s.bulb_c, amp, phase, -1.5, 1.5),
        // Mandelbox
        "box_scale" => anim_f32(&mut s.box_scale, amp, phase, -3.5, 3.5),
        "box_fold" => anim_f32(&mut s.box_fold, amp, phase, 0.2, 2.2),
        "box_fold_mult" => anim_f32(&mut s.box_fold_mult, amp, phase, 1.2, 3.0),
        "box_min_r" => anim_f32(&mut s.box_min_r, amp, phase, 0.05, 1.2),
        "box_fixed_r" => anim_f32(&mut s.box_fixed_r, amp, phase, 0.4, 2.5),
        "box_iter_rot" => anim_f32(&mut s.box_iter_rot, amp, phase, -45.0, 45.0),
        "box_bailout" => anim_f32(&mut s.box_bailout, amp, phase, 4.0, 128.0),
        // KIFS Octahedron
        "kifs_scale" => anim_f32(&mut s.kifs_scale, amp, phase, 1.3, 3.5),
        "kifs_rot1" => anim_f32(&mut s.kifs_rot1, amp, phase, -180.0, 180.0),
        "kifs_rot2" => anim_f32(&mut s.kifs_rot2, amp, phase, -180.0, 180.0),
        "kifs_offset" => anim_vec3(&mut s.kifs_offset, amp, phase, 0.1, 2.0),
        "kifs_thickness" => anim_f32(&mut s.kifs_thickness, amp, phase, 0.005, 0.25),
        "kifs_z_bias" => anim_f32(&mut s.kifs_z_bias, amp, phase, 0.0, 1.5),
        "kifs_bailout" => anim_f32(&mut s.kifs_bailout, amp, phase, 4.0, 32.0),
        // Transform
        "fractal_translate" => anim_vec3(&mut s.fractal_translate, amp, phase, -500.0, 500.0),
        "obj_scale" => anim_f32(&mut s.obj_scale, amp, phase, 0.05, 50.0),
        "rot_x" => anim_f32(&mut s.rot_x, amp, phase, -180.0, 180.0),
        "rot_y" => anim_f32(&mut s.rot_y, amp, phase, -180.0, 180.0),
        "rot_z" => anim_f32(&mut s.rot_z, amp, phase, -180.0, 180.0),
        // Detail & Shading
        "detail_eps" => anim_f32(&mut s.detail_eps, amp, phase, 0.00002, 0.005),
        "norm_eps" => anim_f32(&mut s.norm_eps, amp, phase, 0.00002, 0.005),
        "fudge" => anim_f32(&mut s.fudge, amp, phase, 0.3, 1.1),
        "metallic" => anim_f32(&mut s.metallic, amp, phase, 0.0, 1.0),
        "roughness" => anim_f32(&mut s.roughness, amp, phase, 0.02, 0.85),
        "trap_scale" => anim_f32(&mut s.trap_scale, amp, phase, 0.1, 3.0),
        "palette_shift" => anim_f32(&mut s.palette_shift, amp, phase, 0.0, 1.0),
        // Sky & Lighting
        "sun_intensity" => anim_f32(&mut s.sun_intensity, amp, phase, 0.0, 4.0),
        "sun_azimuth" => anim_f32(&mut s.sun_azimuth, amp, phase, -180.0, 180.0),
        "sun_elevation" => anim_f32(&mut s.sun_elevation, amp, phase, 2.0, 88.0),
        "sky_intensity" => anim_f32(&mut s.sky_intensity, amp, phase, 0.1, 2.0),
        "horizon_haze" => anim_f32(&mut s.horizon_haze, amp, phase, 0.4, 2.5),
        "exposure" => anim_f32(&mut s.exposure, amp, phase, 0.25, 2.0),
        // Environment (per-sky_mode)
        "env_cloud_cover" => anim_f32(&mut s.env.cloud_cover, amp, phase, 0.0, 1.0),
        "env_cloud_density" => anim_f32(&mut s.env.cloud_density, amp, phase, 0.0, 1.5),
        "env_softbox_power" => anim_f32(&mut s.env.softbox_power, amp, phase, 0.2, 3.0),
        "env_softbox_softness" => anim_f32(&mut s.env.softbox_softness, amp, phase, 0.02, 0.5),
        "env_nebula_density" => anim_f32(&mut s.env.nebula_density, amp, phase, 0.2, 2.5),
        "env_nebula_contrast" => anim_f32(&mut s.env.nebula_contrast, amp, phase, 0.5, 3.0),
        "env_nebula_dust" => anim_f32(&mut s.env.nebula_dust, amp, phase, 0.0, 3.0),
        "env_nebula_scale" => anim_f32(&mut s.env.nebula_scale, amp, phase, 0.5, 4.0),
        "env_star_density" => anim_f32(&mut s.env.star_density, amp, phase, 0.0, 2.0),
        "env_star_brightness" => anim_f32(&mut s.env.star_brightness, amp, phase, 0.0, 4.0),
        "env_grid_glow" => anim_f32(&mut s.env.grid_glow, amp, phase, 0.1, 3.0),
        "env_grid_density" => anim_f32(&mut s.env.grid_density, amp, phase, 4.0, 32.0),
        "env_elev_cloud_cover" => anim_f32(&mut s.env.elev_cloud_cover, amp, phase, 0.0, 1.0),
        "env_elev_fog" => anim_f32(&mut s.env.elev_fog, amp, phase, 0.2, 2.5),
        "env_city_fog" => anim_f32(&mut s.env.city_fog, amp, phase, 0.2, 2.5),
        "env_city_windows" => anim_f32(&mut s.env.city_windows, amp, phase, 0.0, 3.0),
        "env_colony_glow" => anim_f32(&mut s.env.colony_glow, amp, phase, 0.0, 3.0),
        "env_planet_limb" => anim_f32(&mut s.env.planet_limb, amp, phase, 0.0, 2.5),
        // Lens DoF
        "aperture" => anim_f32(&mut s.aperture, amp, phase, 0.0, 0.12),
        "focus_dist" => anim_f32(&mut s.focus_dist, amp, phase, 0.5, 12.0),
        // Floor
        "floor_roughness" => anim_f32(&mut s.floor_roughness, amp, phase, 0.03, 0.80),
        "floor_scratch" => anim_f32(&mut s.floor_scratch, amp, phase, 0.0, 1.5),
        "floor_scale" => anim_f32(&mut s.floor_scale, amp, phase, 0.3, 5.0),
        "floor_metallic" => anim_f32(&mut s.floor_metallic, amp, phase, 0.0, 1.0),
        // Scene / Spheres
        "sat_size_min" => anim_f32(&mut s.sat_size_min, amp, phase, 0.001, 0.80),
        "sat_size_max" => anim_f32(&mut s.sat_size_max, amp, phase, 0.001, 0.80),
        "sat_orbit_min" => anim_f32(&mut s.sat_orbit_min, amp, phase, 0.6, 4.5),
        "sat_orbit_max" => anim_f32(&mut s.sat_orbit_max, amp, phase, 0.6, 4.5),
        "sat_height_min" => anim_f32(&mut s.sat_height_min, amp, phase, -1.0, 2.0),
        "sat_height_max" => anim_f32(&mut s.sat_height_max, amp, phase, -1.0, 2.0),
        "sat_speed_min" => anim_f32(&mut s.sat_speed_min, amp, phase, -3.0, 3.0),
        "sat_speed_max" => anim_f32(&mut s.sat_speed_max, amp, phase, -3.0, 3.0),
        "sat_bob_amp_min" => anim_f32(&mut s.sat_bob_amp_min, amp, phase, 0.0, 0.80),
        "sat_bob_amp_max" => anim_f32(&mut s.sat_bob_amp_max, amp, phase, 0.0, 0.80),
        "sat_bob_freq_min" => anim_f32(&mut s.sat_bob_freq_min, amp, phase, 0.0, 5.0),
        "sat_bob_freq_max" => anim_f32(&mut s.sat_bob_freq_max, amp, phase, 0.0, 5.0),
        "sat_hue_spread" => anim_f32(&mut s.sat_hue_spread, amp, phase, 0.0, 1.0),
        "glass_ior" => anim_f32(&mut s.glass_ior, amp, phase, 1.05, 2.40),
        "glass_roughness" => anim_f32(&mut s.glass_roughness, amp, phase, 0.0, 0.35),
        "glass_absorb" => anim_f32(&mut s.glass_absorb, amp, phase, 0.0, 12.0),
        // Bloom & Split & Spin & DLSS
        "bloom_threshold" => anim_f32(&mut s.bloom_threshold, amp, phase, 0.2, 5.0),
        "bloom_intensity" => anim_f32(&mut s.bloom_intensity, amp, phase, 0.0, 2.0),
        "split_x" => anim_f32(&mut s.split_x, amp, phase, 0.02, 0.98),
        "spin_speed" => anim_f32(&mut s.spin_speed, amp, phase, 0.0, 2.0),
        "dlss_sharpness" => anim_f32(&mut s.dlss_sharpness, amp, phase, 0.0, 1.0),
        // CSG World
        "csg_wetness" => anim_f32(&mut s.csg_wetness, amp, phase, 0.0, 1.0),
        "csg_traffic_speed" => anim_f32(&mut s.csg_traffic_speed, amp, phase, 0.0, 3.0),
        "csg_lod_bias" => anim_f32(&mut s.csg_lod_bias, amp, phase, 0.25, 4.0),
        _ => {}
    }
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
}
