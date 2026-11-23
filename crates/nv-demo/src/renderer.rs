//! Asynchronous OptiX path-tracing worker thread communicating with the 60+ FPS GUI via `appbus`.

use std::collections::{HashMap, VecDeque};
use std::ffi::{CString, c_void};
use std::path::PathBuf;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use appbus::downcast_event;
use bytemuck::{Pod, Zeroable};
use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, DeviceBuffer, Stream};
use optix::{
    Aabb, Accel, AovType, BuildOptions, CustomPrimitives, Denoiser, DenoiserModel, DenoiserOptions,
    DeviceContext, GeometryFlags, Guides, HitGroup, Image, Instance, InstanceBuffer, Layer,
    LinkOptions, MicromapCount, MicromapDesc, MicromapFormat, MicromapIndices, MicromapOptions,
    MicromapUse, Module, ModuleOptions, OpacityMicromapArray, Params as DenoiserParams, Pipeline,
    PipelineOptions, ProgramGroup, SbtRecord, Scratch, ShaderBindingTable, TriangleMesh,
    micro_to_base_bary,
};

use crate::app_events::{
    AppBus, EnginePhase, PossessInfo, RenderControl, RenderEvent, SmartViewResult,
    fast_preview_budget,
};
use crate::csg_scene_host::{
    CsgCameraParams, CsgGpuHost, CsgSceneKey, CsgStats, CsgSyncResult, csg_flags,
};
use crate::cuda_crowd::{CrowdLodParams, CudaCrowdBackend, GpuScatterTarget};
use crate::fft_bloom::FftBloomPass;
use crate::gpu_export::save_jpeg_from_rgba8;
use crate::profiler::GpuPassProfiler;
use crate::sdg_export::{
    ActiveCrowdBackend, ActiveSdgOccluder, CrowdBackend, SdgBurstFrameEntry, SdgBurstManifest,
    SdgExrFrame, SdgOccluderBackend, write_sdg_exr_atomic, write_sdg_manifest_atomic,
};
use csg_actors::{KeypointVis, RayOccluder, SdgCamera, Vec3 as ActorVec3};
use csg_scene::{
    CpuViewProber, PackedGpuScene, ProbeBackendKind, ProbeSample, SmartViewSelector,
    ViewCandidate, ViewProbeBackend, ViewScore, WgpuViewProber, WorldScene, score_probe_grid,
};

#[cfg(has_embedded_ptx)]
const EMBEDDED_RAYTRACER_PTX: &str = concat!(
    include_str!(concat!(env!("OUT_DIR"), "/raytracer.ptx")),
    "\0"
);
#[cfg(has_embedded_ptx)]
const EMBEDDED_TONEMAP_PTX: &str =
    concat!(include_str!(concat!(env!("OUT_DIR"), "/tonemap.ptx")), "\0");

#[cfg(not(has_embedded_ptx))]
const RAYTRACER_CU: &str = include_str!("../shaders/raytracer.cu");
#[cfg(not(has_embedded_ptx))]
const TONEMAP_CU: &str = include_str!("../shaders/tonemap.cu");

pub const DEFAULT_W: u32 = 896;
pub const DEFAULT_H: u32 = 512;

/// Aligns width to a multiple of 64 (256-byte `wgpu` row alignment) and height to even.
pub fn align_render_size(vp_w: f32, vp_h: f32, scale: f32) -> (u32, u32) {
    let w = ((vp_w * scale).round() as i32).clamp(256, 3840) as u32;
    let h = ((vp_h * scale).round() as i32).clamp(144, 2160) as u32;
    let w_aligned = (w & !63).max(256);
    let h_aligned = (h & !1).max(144);
    (w_aligned, h_aligned)
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct LaunchParams {
    out_color_f32: u64,
    out_albedo_f32: u64,
    out_normal_f32: u64,
    handle: u64,
    width: u32,
    height: u32,
    frame_index: u32,
    spp: u32,
    cam_pos: [f32; 4],
    cam_forward: [f32; 4],
    cam_right: [f32; 4],
    cam_up: [f32; 4],
    sun_dir: [f32; 4],
    fractal_p0: [f32; 4],
    fractal_p1: [f32; 4],
    fractal_p2: [f32; 4],
    fractal_p3: [f32; 4],
    fractal_xform: [f32; 4],
    fractal_r0: [f32; 4],
    fractal_r1: [f32; 4],
    time: f32,
    enable_shadows: u32,
    fractal_kind: u32,
    bounces: u32,
    floor_params: [f32; 4],
    sky_params: [f32; 4],
    accum_count: u32,
    enable_ser: u32,
    sky_mode_from: u32,
    sky_blend: f32,
    out_motion_f32: u64,
    out_depth_f32: u64,
    prev_cam_pos: [f32; 4],
    prev_cam_forward: [f32; 4],
    prev_cam_right: [f32; 4],
    prev_cam_up: [f32; 4],
    sat_pos: [f32; 4],
    glass_params: [f32; 4],
    aov_mode: u32,
    jitter_x: f32,
    jitter_y: f32,
    _pad1: u32,
    env_params: [f32; 8],
    out_diffuse_albedo_f32: u64,
    out_spec_albedo_f32: u64,
    csg_compounds: u64,
    csg_ops: u64,
    csg_prims: u64,
    csg_aux_planes: u64,
    csg_guard_aabbs: u64,
    csg_repeats: u64,
    csg_materials: u64,
    csg_instances: u64,
    csg_instance_count: u32,
    csg_material_count: u32,
    csg_compound_count: u32,
    csg_flags: u32,
    csg_wetness: f32,
    csg_traffic_time: f32,
    camera_projection: u32,
    probe_grid_wh: u32,
    probe_candidates: u64,
    probe_samples: u64,
    csg_prev_xforms: u64,
    out_semantic_u32: u64,
    out_instance_u32: u64,
    sdg_ray_queries: u64,
    sdg_ray_results: u64,
    sdg_ray_count: u32,
    _pad2: u32,
}

const _: () = assert!(std::mem::size_of::<LaunchParams>() == 640);
const _: () = assert!(std::mem::offset_of!(LaunchParams, csg_prev_xforms) == 592);
const _: () = assert!(std::mem::offset_of!(LaunchParams, out_semantic_u32) == 600);
const _: () = assert!(std::mem::offset_of!(LaunchParams, out_instance_u32) == 608);
const _: () = assert!(std::mem::offset_of!(LaunchParams, sdg_ray_queries) == 616);
const _: () = assert!(std::mem::offset_of!(LaunchParams, sdg_ray_results) == 624);
const _: () = assert!(std::mem::offset_of!(LaunchParams, sdg_ray_count) == 632);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
struct GpuSdgRayQuery {
    origin_tmin: [f32; 4],
    dir_tmax: [f32; 4],
}

const _: () = assert!(std::mem::size_of::<GpuSdgRayQuery>() == 32);

/// Optical sensor projection model for primary ray generation and `Ctrl+Click` Pick-to-Possess.
#[repr(u32)]
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum CameraProjection {
    /// Standard rectilinear pinhole perspective camera.
    #[default]
    Pinhole = 0,
    /// 180-degree equidistant automotive fisheye inscribed in the viewport.
    Fisheye180 = 1,
    /// 360 x 180 equirectangular panoramic camera (ideal for Robotaxi Roof LiDAR).
    Equirect360 = 2,
    /// 360-degree cylindrical slit-scan / pushbroom panorama (vertical lines remain straight).
    SlitCam = 3,
}

impl CameraProjection {
    /// All supported optical sensor projections in UI order.
    pub const ALL: [Self; 4] = [
        Self::Pinhole,
        Self::Fisheye180,
        Self::Equirect360,
        Self::SlitCam,
    ];

    /// Short UI label for toolbar and HUD badges.
    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pinhole => "Pinhole",
            Self::Fisheye180 => "Fisheye 180°",
            Self::Equirect360 => "360° Equirect",
            Self::SlitCam => "360° Slit",
        }
    }

    /// Returns `true` when this projection can observe geometry outside a narrow forward frustum.
    #[inline]
    pub const fn is_omnidirectional(self) -> bool {
        !matches!(self, Self::Pinhole)
    }
}

/// Generates an exact world-space primary ray `(origin, unit_dir)` matching `__raygen__render`
/// for all 4 optical sensor projections (`norm_x`, `norm_y` in `[0, 1]`, top-left origin).
///
/// Returns `None` if `(norm_x, norm_y)` falls outside the circular mask of `Fisheye180`.
pub fn camera_projection_ray(
    cam: &CameraFrame,
    proj: CameraProjection,
    norm_x: f32,
    norm_y: f32,
    aspect: f32,
) -> Option<([f32; 3], [f32; 3])> {
    let fwd = glam::Vec3::from_array(cam.fwd);
    let rgt = glam::Vec3::from_array(cam.right);
    let up = glam::Vec3::from_array(cam.up);
    let safe_aspect = aspect.clamp(0.1, 10.0);
    let u01 = norm_x.clamp(0.0, 1.0);
    let v01 = norm_y.clamp(0.0, 1.0);

    let dir = match proj {
        CameraProjection::Pinhole => {
            let u = u01 * 2.0 - 1.0;
            let v = 1.0 - v01 * 2.0;
            (fwd + rgt * (u * safe_aspect * cam.tan_half_fov) + up * (v * cam.tan_half_fov))
                .normalize_or_zero()
        }
        CameraProjection::Fisheye180 => {
            let sx = safe_aspect.max(1.0);
            let sy = (1.0 / safe_aspect).max(1.0);
            let dx = (u01 - 0.5) * 2.0 * sx;
            let dy = (0.5 - v01) * 2.0 * sy;
            let r = (dx * dx + dy * dy).sqrt();
            if r > 1.0 {
                return None;
            }
            let theta = r * std::f32::consts::FRAC_PI_2;
            let phi = dy.atan2(dx);
            let radial = rgt * phi.cos() + up * phi.sin();
            (fwd * theta.cos() + radial * theta.sin()).normalize_or_zero()
        }
        CameraProjection::Equirect360 => {
            let yaw = (u01 - 0.5) * std::f32::consts::TAU;
            let pitch = (0.5 - v01) * std::f32::consts::PI;
            let hdir = fwd * yaw.cos() + rgt * yaw.sin();
            (hdir * pitch.cos() + up * pitch.sin()).normalize_or_zero()
        }
        CameraProjection::SlitCam => {
            let yaw = (u01 - 0.5) * std::f32::consts::TAU;
            let hdir = fwd * yaw.cos() + rgt * yaw.sin();
            let v_off = (0.5 - v01) * 2.0 * cam.tan_half_fov;
            (hdir + up * v_off).normalize_or_zero()
        }
    };
    if dir.length_squared() < 1e-8 {
        return None;
    }
    Some((cam.eye, dir.to_array()))
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
struct GpuProbeCandidate {
    eye: [f32; 4],
    forward: [f32; 4],
    right: [f32; 4],
    up: [f32; 4],
}

const _: () = assert!(std::mem::size_of::<GpuProbeCandidate>() == 64);

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EnvSettings {
    // Day Sky (0) & Sunset (1)
    pub cloud_cover: f32,
    pub cloud_density: f32,
    // Studio Softbox (2)
    pub softbox_power: f32,
    pub softbox_softness: f32,
    // Deep Nebula (3)
    pub nebula_density: f32,
    pub nebula_contrast: f32,
    pub nebula_dust: f32,
    pub nebula_scale: f32,
    // Starfield (Deep Nebula & Offscreen Colonies)
    pub star_density: f32,
    pub star_brightness: f32,
    // Cyber Grid (4)
    pub grid_glow: f32,
    pub grid_density: f32,
    // Elevated (5)
    pub elev_cloud_cover: f32,
    pub elev_fog: f32,
    // Unprogress (6)
    pub city_fog: f32,
    pub city_windows: f32,
    // Offscreen Colonies (7)
    pub colony_glow: f32,
    pub planet_limb: f32,
}

impl Default for EnvSettings {
    fn default() -> Self {
        Self {
            cloud_cover: 0.45,
            cloud_density: 1.0,
            softbox_power: 1.0,
            softbox_softness: 0.14,
            nebula_density: 1.15,
            nebula_contrast: 1.45,
            nebula_dust: 1.35,
            nebula_scale: 1.65,
            star_density: 1.0,
            star_brightness: 1.35,
            grid_glow: 1.0,
            grid_density: 12.0,
            elev_cloud_cover: 0.48,
            elev_fog: 1.0,
            city_fog: 1.0,
            city_windows: 1.0,
            colony_glow: 1.2,
            planet_limb: 1.0,
        }
    }
}

impl EnvSettings {
    pub fn pack(&self, sky_mode: u32) -> [f32; 8] {
        match sky_mode {
            0 | 1 => [
                self.cloud_cover,
                self.cloud_density,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ],
            2 => [
                self.softbox_power,
                self.softbox_softness,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ],
            3 => [
                self.nebula_density,
                self.nebula_contrast,
                self.nebula_dust,
                self.nebula_scale,
                self.star_density,
                self.star_brightness,
                0.0,
                0.0,
            ],
            4 => [
                self.grid_glow,
                self.grid_density,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ],
            5 => [
                self.elev_cloud_cover,
                self.elev_fog,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ],
            6 => [
                self.city_fog,
                self.city_windows,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ],
            7 => [
                self.star_density,
                self.star_brightness,
                self.colony_glow,
                self.planet_limb,
                0.0,
                0.0,
                0.0,
                0.0,
            ],
            _ => [0.0; 8],
        }
    }
}

/// Selects whether still captures (PNG/JPEG) and MP4 recordings capture only the 3D viewport
/// or the entire application window (3D viewport + docked egui UI).
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum CaptureSource {
    /// Capture only the 3D rendered viewport (zero-copy CUDA/DLSS fast path).
    #[default]
    Viewport,
    /// Capture the entire application window including docked egui UI panels and toolbar.
    FullWindow,
}

fn default_csg_road_mode() -> u8 {
    2
}

fn default_tod_transition_enabled() -> bool {
    true
}

fn default_tod_transition_secs() -> f32 {
    2.0
}

fn default_csg_lod_enabled() -> bool {
    true
}

fn default_csg_force_lod() -> u32 {
    0
}

fn default_csg_lod1_dist_m() -> f32 {
    80.0
}

fn default_csg_lod2_dist_m() -> f32 {
    220.0
}

fn default_csg_meso_dist_m() -> f32 {
    300.0
}

fn default_csg_micro_dist_m() -> f32 {
    35.0
}

fn default_csg_min_screen_px() -> f32 {
    1.5
}

fn default_csg_lod_hysteresis() -> f32 {
    0.15
}

fn default_csg_crowd_lod1_m() -> f32 {
    22.0
}

fn default_csg_crowd_lod2_m() -> f32 {
    75.0
}

fn default_csg_crowd_cull_m() -> f32 {
    180.0
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DemoSettings {
    pub fractal_kind: u32,
    // Fractal Transform
    /// World-space translation `[x, y, z]` of the Hero Fractal in meters.
    pub fractal_translate: [f32; 3],
    /// Uniform Hero Fractal scale factor (`0.05..=50.0`, default `1.0`).
    pub obj_scale: f32,
    /// Hero Fractal X-axis rotation in degrees (`-180.0..=180.0`).
    pub rot_x: f32,
    /// Hero Fractal Y-axis rotation in degrees (`-180.0..=180.0`).
    pub rot_y: f32,
    /// Hero Fractal Z-axis rotation in degrees (`-180.0..=180.0`).
    pub rot_z: f32,
    // Mandelbulb
    pub bulb_power: f32,
    pub bulb_bailout: f32,
    pub bulb_iter: u32,
    pub bulb_julia: bool,
    pub bulb_c: [f32; 3],
    pub bulb_theta_mult: f32,
    pub bulb_phi_mult: f32,
    pub bulb_theta_phase: f32,
    pub bulb_phi_phase: f32,
    // Mandelbox
    pub box_scale: f32,
    pub box_fold: f32,
    pub box_min_r: f32,
    pub box_fixed_r: f32,
    pub box_iter: u32,
    pub box_bailout: f32,
    pub box_fold_mult: f32,
    pub box_iter_rot: f32,
    // KIFS Octahedron
    pub kifs_scale: f32,
    pub kifs_rot1: f32,
    pub kifs_rot2: f32,
    pub kifs_offset: [f32; 3],
    pub kifs_iter: u32,
    pub kifs_bailout: f32,
    pub kifs_thickness: f32,
    pub kifs_z_bias: f32,
    // Detail & Shading
    pub detail_eps: f32,
    pub fudge: f32,
    pub norm_eps: f32,
    pub march_steps: u32,
    pub bounces: u32,
    pub metallic: f32,
    pub trap_scale: f32,
    pub palette_shift: f32,
    pub sun_intensity: f32,
    pub sun_azimuth: f32,
    pub sun_elevation: f32,
    // Sky & Floor
    pub sky_mode: u32,
    /// Transient source `sky_mode` during smooth `TodTransition` crossfade (not persisted).
    #[serde(skip)]
    pub sky_mode_from: u32,
    /// Transient `[0, 1]` crossfade factor from `sky_mode_from` to `sky_mode` (0 = no blend).
    #[serde(skip)]
    pub sky_blend: f32,
    pub sky_intensity: f32,
    pub horizon_haze: f32,
    pub floor_roughness: f32,
    pub floor_scratch: f32,
    pub floor_scale: f32,
    pub floor_metallic: f32,
    // Rendering & Lens
    pub roughness: f32,
    pub exposure: f32,
    pub aperture: f32,
    pub focus_dist: f32,
    pub spp: u32,
    pub max_accum: u32,
    pub aov_mode: u32,
    pub render_scale: f32,
    pub denoiser_on: bool,
    pub temporal_denoise: bool,
    pub ser_on: bool,
    pub omm_mode: u32,
    pub shadows_on: bool,
    pub auto_rotate: bool,
    pub spin_speed: f32,
    // Multi-instance Scene (Glass Satellite + Neon Accent)
    pub sat_on: bool,
    pub sat_orbit: bool,
    pub sat_count: u32,
    pub sat_seed: u32,
    pub sat_size_min: f32,
    pub sat_size_max: f32,
    pub sat_orbit_min: f32,
    pub sat_orbit_max: f32,
    pub sat_height_min: f32,
    pub sat_height_max: f32,
    pub sat_speed_min: f32,
    pub sat_speed_max: f32,
    pub sat_bob_amp_min: f32,
    pub sat_bob_amp_max: f32,
    pub sat_bob_freq_min: f32,
    pub sat_bob_freq_max: f32,
    pub sat_hue_spread: f32,
    pub glass_ior: f32,
    pub glass_absorb: f32,
    pub glass_roughness: f32,
    pub glass_analytic: bool,
    pub sat_u_segs: u32,
    pub sat_v_segs: u32,
    pub neon_on: bool,
    // FFT Bloom (cuFFT)
    pub bloom_on: bool,
    pub bloom_threshold: f32,
    pub bloom_intensity: f32,
    pub bloom_blades: u32,
    // A/B Split Comparison
    pub split_on: bool,
    pub split_x: f32,
    pub right_is_aov: u32,
    // DLSS Super Resolution & Ray Reconstruction (NGX)
    pub dlss_on: bool,
    pub dlss_rr_on: bool,
    pub dlss_quality: u32,
    pub dlss_sharpness: f32,
    // Display & Video Export (HDR scRGB viewport + MP4 Match/SDR/PQ10 + Timing)
    pub hdr_display: bool,
    /// `0 = Match View`, `1 = SDR H.264`, `2 = PQ10 HDR10 HEVC`.
    pub record_video_mode: u32,
    pub record_fixed_fps: bool,
    /// Capture target for PNG/JPEG stills and MP4 video recording.
    pub capture_source: CaptureSource,
    /// Cinematic gimbal camera smoothing time constant in seconds (`0.0` = instant/off).
    pub cam_smooth_tau: f32,
    /// Global simulation & animation time scale (`0.05..=2.0`, `1.0` = real-time).
    pub time_scale: f32,
    /// 180-degree shutter motion-blur subframe count in Smooth 60 recording (`1` = Off, `2`, `4`).
    pub record_shutter_subframes: u32,
    // Per-environment procedural parameters (persisted in PersistedState)
    pub env: EnvSettings,
    // Render-time analytical CSG world (`csg-scenes` SceneRegistry)
    /// `0 = Off (Classic)`, `1..=N` selects registered scene index `csg_mode - 1` (`Gundo`, `Marina del Rey`, `SF House of AI`, `Inspector`).
    pub csg_mode: u32,
    /// Selected variant index within the active scene (e.g. `Inspector` asset index).
    pub csg_asset_idx: u32,
    /// Deterministic procedural seed for CSG scenes.
    pub csg_seed: u32,
    /// `0 = Day`, `1 = Sunset`, `2 = Night`.
    pub csg_time_of_day: u32,
    /// Smoothly interpolate sky/sun parameters when switching Day / Sunset / Night.
    #[serde(default = "default_tod_transition_enabled")]
    pub tod_transition_enabled: bool,
    /// Duration in seconds for smooth Day / Sunset / Night transitions (`0.1..=15.0`).
    #[serde(default = "default_tod_transition_secs")]
    pub tod_transition_secs: f32,
    /// Coastal road & concrete wetness (`0.0..=1.0`).
    pub csg_wetness: f32,
    /// Animate highway traffic & spinning wheels in `Gundo`.
    pub csg_traffic_on: bool,
    /// Traffic simulation speed multiplier (`0.0..=3.0`).
    pub csg_traffic_speed: f32,
    /// Freeze dynamic traffic & crowd poses for clean multi-spp accumulation and SDG capture.
    pub csg_freeze_pose: bool,
    /// Keep the classic foreground fractal & stage floor visible inside CSG modes.
    pub csg_show_fractal: bool,
    /// Master distance & screen-space LOD / culling toggle (`false` renders full LOD0 everywhere).
    #[serde(default = "default_csg_lod_enabled")]
    pub csg_lod_enabled: bool,
    /// Optional forced LOD tier (`0 = Auto`, `1 = LOD0`, `2 = LOD1`, `3 = LOD2`).
    #[serde(default = "default_csg_force_lod")]
    pub csg_force_lod: u32,
    /// Distance/SSE LOD detail multiplier (`0.25..=4.0`).
    pub csg_lod_bias: f32,
    /// Base LOD0 -> LOD1 transition distance in meters (`10..=500`, default `80.0`).
    #[serde(default = "default_csg_lod1_dist_m")]
    pub csg_lod1_dist_m: f32,
    /// Base LOD1 -> LOD2 / proxy transition distance in meters (`30..=1200`, default `220.0`).
    #[serde(default = "default_csg_lod2_dist_m")]
    pub csg_lod2_dist_m: f32,
    /// Base `DetailTier::Meso` cull distance in meters (`30..=1200`, default `300.0`).
    #[serde(default = "default_csg_meso_dist_m")]
    pub csg_meso_dist_m: f32,
    /// Base `DetailTier::Micro` cull distance in meters (`5..=200`, default `35.0`).
    #[serde(default = "default_csg_micro_dist_m")]
    pub csg_micro_dist_m: f32,
    /// Minimum projected screen-space pixel diameter before culling (`0.0..=16.0`, default `1.5`).
    #[serde(default = "default_csg_min_screen_px")]
    pub csg_min_screen_px: f32,
    /// Fractional hysteresis deadband around LOD switching boundaries (`0.0..=0.45`, default `0.15`).
    #[serde(default = "default_csg_lod_hysteresis")]
    pub csg_lod_hysteresis: f32,
    /// Base crowd LOD0 -> LOD1 transition distance in meters (`5..=120`, default `22.0`).
    #[serde(default = "default_csg_crowd_lod1_m")]
    pub csg_crowd_lod1_m: f32,
    /// Base crowd LOD1 -> LOD2 transition distance in meters (`20..=300`, default `75.0`).
    #[serde(default = "default_csg_crowd_lod2_m")]
    pub csg_crowd_lod2_m: f32,
    /// Base crowd visibility cull distance in meters (`40..=600`, default `180.0`).
    #[serde(default = "default_csg_crowd_cull_m")]
    pub csg_crowd_cull_m: f32,
    /// Road geometry representation (`0 = Watertight 3D`, `1 = C1 Tangent`, `2 = Hybrid C1+SDF`, `3 = F-Rep Tile`).
    #[serde(default = "default_csg_road_mode")]
    pub csg_road_mode: u8,
    /// Active optical sensor projection (`Pinhole`, `Fisheye180`, `Equirect360`, `SlitCam`).
    pub camera_projection: CameraProjection,
    /// Active primary-ray view scoring backend (`Auto`, `Optix`, `Wgpu`, `Cpu`).
    pub probe_backend: ProbeBackendKind,
    /// Articulated crowd simulation compute backend (`Auto`, `Cpu`, `Cuda`).
    pub crowd_backend: CrowdBackend,
    /// SDG keypoint occlusion raycasting backend (`Auto`, `Cpu`, `Optix`).
    pub sdg_occluder_backend: SdgOccluderBackend,
    /// Interactive poke impulse strength (`0.5..=4.0`, default `1.8`).
    pub poke_impulse: f32,
    /// Number of frames captured by `Export SDG Burst` (`16`, `30`, or `60`).
    pub sdg_burst_frames: u32,
}

impl Default for DemoSettings {
    fn default() -> Self {
        Self {
            fractal_kind: 0,
            fractal_translate: [0.0, 0.0, 0.0],
            obj_scale: 1.0,
            rot_x: 0.0,
            rot_y: 0.0,
            rot_z: 0.0,
            bulb_power: 8.0,
            bulb_bailout: 8.0,
            bulb_iter: 16,
            bulb_julia: false,
            bulb_c: [-0.2, 0.6, 0.2],
            bulb_theta_mult: 1.0,
            bulb_phi_mult: 1.0,
            bulb_theta_phase: 0.0,
            bulb_phi_phase: 0.0,
            box_scale: -2.0,
            box_fold: 1.0,
            box_min_r: 0.5,
            box_fixed_r: 1.0,
            box_iter: 18,
            box_bailout: 64.0,
            box_fold_mult: 2.0,
            box_iter_rot: 0.0,
            kifs_scale: 3.0,
            kifs_rot1: 0.0,
            kifs_rot2: 0.0,
            kifs_offset: [1.0, 1.0, 1.0],
            kifs_iter: 14,
            kifs_bailout: 16.0,
            kifs_thickness: 0.05,
            kifs_z_bias: 0.5,
            detail_eps: 0.0002,
            fudge: 0.80,
            norm_eps: 0.0002,
            march_steps: 256,
            bounces: 3,
            metallic: 0.45,
            trap_scale: 0.85,
            palette_shift: 0.0,
            sun_intensity: 1.45,
            sun_azimuth: 32.0,
            sun_elevation: 48.0,
            sky_mode: 0,
            sky_mode_from: 0,
            sky_blend: 0.0,
            sky_intensity: 0.78,
            horizon_haze: 1.0,
            floor_roughness: 0.22,
            floor_scratch: 0.65,
            floor_scale: 1.5,
            floor_metallic: 0.85,
            roughness: 0.18,
            exposure: 0.92,
            aperture: 0.0,
            focus_dist: 3.2,
            spp: 1,
            max_accum: 256,
            aov_mode: 0,
            render_scale: 1.0,
            denoiser_on: false,
            temporal_denoise: false,
            ser_on: true,
            omm_mode: 0,
            shadows_on: true,
            auto_rotate: false,
            spin_speed: 0.45,
            sat_on: true,
            sat_orbit: false,
            sat_count: 1,
            sat_seed: 42,
            sat_size_min: 0.14,
            sat_size_max: 0.42,
            sat_orbit_min: 1.35,
            sat_orbit_max: 2.85,
            sat_height_min: -0.45,
            sat_height_max: 0.85,
            sat_speed_min: -1.20,
            sat_speed_max: 1.60,
            sat_bob_amp_min: 0.04,
            sat_bob_amp_max: 0.22,
            sat_bob_freq_min: 0.80,
            sat_bob_freq_max: 2.80,
            sat_hue_spread: 0.35,
            glass_ior: 1.52,
            glass_absorb: 0.45,
            glass_roughness: 0.02,
            glass_analytic: true,
            sat_u_segs: 24,
            sat_v_segs: 36,
            neon_on: true,
            bloom_on: false,
            bloom_threshold: 1.25,
            bloom_intensity: 0.35,
            bloom_blades: 6,
            split_on: false,
            split_x: 0.5,
            right_is_aov: 0,
            dlss_on: false,
            dlss_rr_on: false,
            dlss_quality: 3,
            dlss_sharpness: 0.35,
            hdr_display: false,
            record_video_mode: 0,
            record_fixed_fps: false,
            capture_source: CaptureSource::Viewport,
            cam_smooth_tau: 0.0,
            time_scale: 1.0,
            record_shutter_subframes: 1,
            env: EnvSettings::default(),
            csg_mode: 0,
            csg_asset_idx: 13,
            csg_seed: 1337,
            csg_time_of_day: 1,
            tod_transition_enabled: default_tod_transition_enabled(),
            tod_transition_secs: default_tod_transition_secs(),
            csg_wetness: 0.38,
            csg_traffic_on: true,
            csg_traffic_speed: 1.0,
            csg_freeze_pose: false,
            csg_show_fractal: false,
            csg_lod_enabled: default_csg_lod_enabled(),
            csg_force_lod: default_csg_force_lod(),
            csg_lod_bias: 1.0,
            csg_lod1_dist_m: default_csg_lod1_dist_m(),
            csg_lod2_dist_m: default_csg_lod2_dist_m(),
            csg_meso_dist_m: default_csg_meso_dist_m(),
            csg_micro_dist_m: default_csg_micro_dist_m(),
            csg_min_screen_px: default_csg_min_screen_px(),
            csg_lod_hysteresis: default_csg_lod_hysteresis(),
            csg_crowd_lod1_m: default_csg_crowd_lod1_m(),
            csg_crowd_lod2_m: default_csg_crowd_lod2_m(),
            csg_crowd_cull_m: default_csg_crowd_cull_m(),
            csg_road_mode: default_csg_road_mode(),
            camera_projection: CameraProjection::Pinhole,
            probe_backend: ProbeBackendKind::Auto,
            crowd_backend: CrowdBackend::Auto,
            sdg_occluder_backend: SdgOccluderBackend::Auto,
            poke_impulse: 1.8,
            sdg_burst_frames: 16,
        }
    }
}

impl DemoSettings {
    /// Builds the unified [`csg_scene::LodConfig`] driving static/dynamic CSG LODs and culling.
    #[inline]
    pub fn csg_lod_config(&self) -> csg_scene::LodConfig {
        csg_scene::LodConfig {
            enabled: self.csg_lod_enabled,
            lod_bias: self.csg_lod_bias,
            force_lod: CrowdLodParams::decode_force_lod(self.csg_force_lod),
            lod1_dist_m: self.csg_lod1_dist_m,
            lod2_dist_m: self.csg_lod2_dist_m,
            meso_dist_m: self.csg_meso_dist_m,
            micro_dist_m: self.csg_micro_dist_m,
            min_screen_px: self.csg_min_screen_px,
            hysteresis: self.csg_lod_hysteresis,
        }
    }

    /// Builds the unified [`CrowdLodParams`] driving both CPU (`CrowdSim`) and CUDA crowd LODs.
    #[inline]
    pub fn crowd_lod_params(&self) -> CrowdLodParams {
        CrowdLodParams {
            enabled: self.csg_lod_enabled,
            force_lod: CrowdLodParams::decode_force_lod(self.csg_force_lod),
            lod_bias: self.csg_lod_bias,
            lod1_dist_m: self.csg_crowd_lod1_m,
            lod2_dist_m: self.csg_crowd_lod2_m,
            cull_dist_m: self.csg_crowd_cull_m,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraFrame {
    pub eye: [f32; 3],
    pub fwd: [f32; 3],
    pub right: [f32; 3],
    pub up: [f32; 3],
    pub tan_half_fov: f32,
}

/// Double-buffered CUDA device pointers for optional DLSS Ray Reconstruction FP16 AOV buffers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DlssRrBufferPtrs {
    pub hdr_color: [u64; 2],
    pub diffuse_albedo: [u64; 2],
    pub spec_albedo: [u64; 2],
    pub normal_rough: [u64; 2],
}

#[derive(Clone, Debug)]
pub struct FrameReady {
    pub buf_idx: usize,
    pub gen_id: u64,
    pub accum_count: u32,
    pub total_samples: u32,
    pub jitter: [f32; 2],
    /// Hardware **RT Cores (OptiX)** pass duration in milliseconds.
    pub rt_ms: f32,
    /// Hardware **Tensor Cores (OptiX AI Denoiser)** pass duration in milliseconds.
    pub denoise_ms: f32,
    /// Hardware **cuFFT 2D Bloom** pass duration in milliseconds.
    pub bloom_ms: f32,
    /// Hardware **Tonemap / HDR / DLSS-RR Pack** pass duration in milliseconds.
    pub tonemap_ms: f32,
    /// Hardware **CUDA Compute** total (`IAS/Traffic + cuFFT + Tonemap`) in milliseconds.
    pub cuda_compute_ms: f32,
    pub ser_supported: bool,
    pub csg_stats: CsgStats,
    pub fast_preview: bool,
    /// Resolved world camera basis when actively possessing a body (synced back to UI for picking & Eject).
    pub possess_camera: Option<CameraFrame>,
    /// Exact world camera basis that rendered this frame (consumed by DLSS-RR `WorldToView` / `ViewToClip`).
    pub rendered_camera: CameraFrame,
    /// Articulated crowd step wall/GPU duration in milliseconds for the latest frame.
    pub crowd_step_ms: f32,
    /// Concrete backend that executed the latest crowd step (`None`, `Cpu`, `Cuda`).
    pub crowd_active_backend: ActiveCrowdBackend,
    /// Number of articulated crowd actors currently simulated.
    pub crowd_actor_count: u32,
}

struct TonemapKernel {
    cuda: Arc<CudaContext>,
    module: cu::CUmodule,
    func: cu::CUfunction,
    scrgb_func: cu::CUfunction,
    pack_rr_func: cu::CUfunction,
}

impl Drop for TonemapKernel {
    fn drop(&mut self) {
        if !self.module.is_null() && self.cuda.bind_to_thread().is_ok() {
            // SAFETY: `self.module` was loaded via `cuModuleLoadData` on `self.cuda`.
            let rc = unsafe { cu::cuModuleUnload(self.module) };
            if rc != cu::cudaError_enum_CUDA_SUCCESS {
                eprintln!("nv-demo: TonemapKernel cuModuleUnload failed ({rc:?})");
            }
            self.module = std::ptr::null_mut();
        }
    }
}

struct OptixRig {
    stream: Arc<Stream>,
    _optix_ctx: Arc<DeviceContext>,
    _scratch: Scratch,
    floor_gas: Arc<Accel>,
    fractal_gas: Arc<Accel>,
    glass_analytic_gas: Arc<Accel>,
    glass_mesh_gas: Arc<Accel>,
    glass_mesh_segs: (u32, u32),
    neon_gas: Arc<Accel>,
    omm_gas: Arc<Accel>,
    omm_anyhit_gas: Arc<Accel>,
    ias: Option<Arc<Accel>>,
    d_optix_instances: InstanceBuffer,
    ias_classic_count: usize,
    pipeline: Pipeline,
    sbt: ShaderBindingTable,
    probe_sbt: ShaderBindingTable,
    sdg_sbt: ShaderBindingTable,
    ser_supported: bool,
    denoiser: Option<Denoiser>,
    denoiser_is_temporal: bool,
    temporal_has_prev: bool,
    color_f32: DeviceBuffer<[f32; 4]>,
    albedo_f32: DeviceBuffer<[f32; 4]>,
    diffuse_albedo_f32: Option<DeviceBuffer<[f32; 4]>>,
    spec_albedo_f32: Option<DeviceBuffer<[f32; 4]>>,
    normal_f32: DeviceBuffer<[f32; 4]>,
    denoised_f32: Option<DeviceBuffer<[f32; 4]>>,
    prev_denoised_f32: Option<DeviceBuffer<[f32; 4]>>,
    motion_f32: Option<DeviceBuffer<[f32; 2]>>,
    depth_f32: Option<DeviceBuffer<f32>>,
    semantic_u32: Option<DeviceBuffer<u32>>,
    instance_u32: Option<DeviceBuffer<u32>>,
    params_buf: DeviceBuffer<u8>,
    tonemap: TonemapKernel,
    bloom: Option<FftBloomPass>,
    gpu_profiler: Option<GpuPassProfiler>,
    csg_host: CsgGpuHost,
    ias_instances_scratch: Vec<Instance>,
    ias_dyn_scratch: Vec<Instance>,
    probe_candidates_buf: Option<DeviceBuffer<u8>>,
    probe_samples_buf: Option<DeviceBuffer<u8>>,
    probe_candidates_host: Vec<GpuProbeCandidate>,
    probe_samples_host: Vec<ProbeSample>,
    sdg_queries_buf: Option<DeviceBuffer<u8>>,
    sdg_results_buf: Option<DeviceBuffer<u8>>,
    sdg_queries_host: Vec<GpuSdgRayQuery>,
    sdg_results_host: Vec<u32>,
    cpu_prober: CpuViewProber,
    wgpu_prober: Option<WgpuViewProber>,
    cuda_crowd: Option<Result<CudaCrowdBackend, String>>,
    crowd_slot_map_host: Vec<u32>,
    crowd_gas_handles_host: Vec<u64>,
}

fn find_optix_include() -> Option<String> {
    let mut candidates = Vec::new();
    if let Ok(root) = std::env::var("OPTIX_ROOT") {
        candidates.push(std::path::PathBuf::from(root).join("include"));
    }
    if let Ok(pd) = std::env::var("ProgramData") {
        let base = std::path::PathBuf::from(pd).join("NVIDIA Corporation");
        if let Ok(rd) = std::fs::read_dir(&base) {
            for entry in rd.flatten() {
                let name = entry.file_name();
                if name.to_string_lossy().starts_with("OptiX SDK") {
                    candidates.push(entry.path().join("include"));
                }
            }
        }
    }
    candidates
        .into_iter()
        .find(|p| p.join("optix.h").is_file())
        .map(|p| p.to_string_lossy().into_owned())
}

fn find_cuda_include() -> Option<String> {
    for var in ["CUDA_PATH", "CUDA_HOME", "CUDA_ROOT"] {
        if let Ok(root) = std::env::var(var) {
            let p = std::path::PathBuf::from(root).join("include");
            if p.is_dir() {
                return Some(p.to_string_lossy().into_owned());
            }
        }
    }
    let linux_inc = std::path::Path::new("/usr/local/cuda/include");
    if linux_inc.is_dir() {
        return Some(linux_inc.to_string_lossy().into_owned());
    }
    None
}

/// OptiX instance flag disabling any-hit programs on opaque classic instances.
const OPTIX_INST_DISABLE_ANYHIT: u32 =
    optix::sys::OptixInstanceFlags_OPTIX_INSTANCE_FLAG_DISABLE_ANYHIT as u32;

/// Pure helper clamping `accum_count` before a `SaveJpeg` capture without discarding converged
/// samples when `max_accum == 0` (infinite accumulation).
#[inline]
const fn clamp_accum_for_jpeg(accum_count: u32, max_accum: u32) -> u32 {
    if max_accum > 0 {
        let cap = max_accum.saturating_sub(1);
        if accum_count < cap { accum_count } else { cap }
    } else {
        accum_count
    }
}

/// Returns `true` when DLSS-RR owns the viewport display output and no consumer needs the CUDA
/// RGBA8 / HDR tonemap passes (`aov_mode == 0 && !split_on && pending_jpeg.is_none()`).
#[inline]
const fn should_skip_cuda_tonemap(
    dlss_rr_on: bool,
    has_rr_ptrs: bool,
    aov_mode: u32,
    split_on: bool,
    has_pending_jpeg: bool,
) -> bool {
    dlss_rr_on && has_rr_ptrs && aov_mode == 0 && !split_on && !has_pending_jpeg
}

fn try_compile_ptx(src: &str, name: &str, include_optix: bool) -> Result<Vec<u8>, String> {
    let mut include_dirs = Vec::with_capacity(3);
    let mut csg_eval_bytes: Vec<u8> = Vec::new();
    let mut optix_tag: Vec<u8> = Vec::new();

    if include_optix {
        if let Some(p) = find_optix_include() {
            optix_tag.extend_from_slice(p.as_bytes());
            include_dirs.push(p);
        }
        if let Some(p) = find_cuda_include() {
            include_dirs.push(p);
        }
        let csg_dir = csg_device::csg_eval_cuh_dir();
        if let Ok(bytes) = std::fs::read(std::path::Path::new(csg_dir).join("csg_eval.cuh")) {
            csg_eval_bytes = bytes;
        }
        include_dirs.push(csg_dir.to_string());
    }

    let mut extra_blobs: Vec<&[u8]> = Vec::with_capacity(2);
    if !optix_tag.is_empty() {
        extra_blobs.push(optix_tag.as_slice());
    }
    if !csg_eval_bytes.is_empty() {
        extra_blobs.push(csg_eval_bytes.as_slice());
    }
    crate::gpu_export::compile_ptx_cached(src, name, "compute_75", &include_dirs, &extra_blobs)
}

/// Compiles an OptiX `Module` in parallel via `Module::with_tasks` across a scoped worker pool,
/// falling back cleanly to `Module::new` if task-based compilation is unavailable.
fn compile_optix_module_parallel(
    optix_ctx: &Arc<DeviceContext>,
    mod_opts: &ModuleOptions<'_>,
    pipe_opts: &PipelineOptions,
    rt_ptx: &[u8],
) -> Result<Arc<Module>, String> {
    let (pending, first_task) = match Module::with_tasks(optix_ctx, mod_opts, pipe_opts, rt_ptx) {
        Ok(pair) => pair,
        Err(optix::Error::Unavailable(_)) => {
            let (module, _) = Module::new(optix_ctx, mod_opts, pipe_opts, rt_ptx)
                .map_err(|e| format!("Module::new: {e}"))?;
            return Ok(module);
        }
        Err(e) => return Err(format!("Module::with_tasks: {e}")),
    };

    let worker_count = thread::available_parallelism()
        .map(|n| n.get().clamp(2, 8))
        .unwrap_or(4);
    let queue_state = Mutex::new((VecDeque::from([first_task]), 0usize, false));
    let queue_cv = Condvar::new();

    thread::scope(|scope| {
        for _ in 0..worker_count {
            let state_ref = &queue_state;
            let cv_ref = &queue_cv;
            let pending_ref = &pending;
            scope.spawn(move || {
                loop {
                    let mut task = {
                        let mut guard = state_ref
                            .lock()
                            .unwrap_or_else(|poisoned| poisoned.into_inner());
                        loop {
                            if guard.2 {
                                return;
                            }
                            if let Some(t) = guard.0.pop_front() {
                                guard.1 += 1;
                                break t;
                            }
                            if guard.1 == 0 {
                                return;
                            }
                            guard = cv_ref
                                .wait(guard)
                                .unwrap_or_else(|poisoned| poisoned.into_inner());
                        }
                    };

                    let exec_res = task.execute(8);
                    let mut guard = state_ref
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner());
                    guard.1 = guard.1.saturating_sub(1);
                    match exec_res {
                        Ok(more) => {
                            for next in more {
                                guard.0.push_back(next);
                            }
                        }
                        Err(_) => {
                            guard.2 = true;
                            guard.0.clear();
                            if let Err(cancel_err) = pending_ref.cancel(false) {
                                eprintln!(
                                    "nv-demo: OptiX parallel compile cancel warning: {cancel_err}"
                                );
                            }
                        }
                    }
                    cv_ref.notify_all();
                }
            });
        }
    });

    let (module, _) = pending
        .finish()
        .map_err(|e| format!("PendingModule::finish: {e}"))?;
    Ok(module)
}

fn build_optix_pipeline(
    optix_ctx: &Arc<DeviceContext>,
    stream: &Stream,
    rt_ptx: &[u8],
) -> Result<
    (
        Pipeline,
        ShaderBindingTable,
        ShaderBindingTable,
        ShaderBindingTable,
    ),
    String,
> {
    let mut opts = PipelineOptions::new("params", std::mem::size_of::<LaunchParams>(), 8)
        .map_err(|e| format!("PipelineOptions: {e}"))?;
    opts.attribute_values = 5;
    opts.single_level_instancing = true;
    opts.primitive_types =
        (optix::sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_CUSTOM
            | optix::sys::OptixPrimitiveTypeFlags_OPTIX_PRIMITIVE_TYPE_FLAGS_TRIANGLE) as u32;
    opts.opacity_micromaps = true;
    let module = compile_optix_module_parallel(optix_ctx, &ModuleOptions::default(), &opts, rt_ptx)?;

    let raygen =
        ProgramGroup::raygen(&module, "__raygen__render").map_err(|e| format!("raygen: {e}"))?;
    let raygen_probe = ProgramGroup::raygen(&module, "__raygen__score_views")
        .map_err(|e| format!("raygen_probe: {e}"))?;
    let raygen_sdg = ProgramGroup::raygen(&module, "__raygen__sdg_keypoints")
        .map_err(|e| format!("raygen_sdg: {e}"))?;
    let miss = ProgramGroup::miss(&module, "__miss__miss").map_err(|e| format!("miss: {e}"))?;
    let miss_shadow =
        ProgramGroup::miss(&module, "__miss__shadow").map_err(|e| format!("miss_shadow: {e}"))?;

    // Paired `[primary, shadow]` hit groups (`SBT_STRIDE = 2`, 12 records total):
    // 0,1: Floor/Mesh/Neon | 2,3: Fractal | 4,5: Sphere | 6,7: OMM | 8,9: CSG RPN | 10,11: CSG Prim/Multi-AABB
    let hit_floor = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__hit")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_floor: {e}"))?;
    let hit_floor_shadow = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__shadow")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_floor_shadow: {e}"))?;
    let hit_fractal = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__fractal")),
            intersection: Some((&module, "__intersection__fractal")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_fractal: {e}"))?;
    let hit_fractal_shadow = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__shadow")),
            intersection: Some((&module, "__intersection__fractal_shadow")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_fractal_shadow: {e}"))?;
    let hit_sphere = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__fractal")),
            intersection: Some((&module, "__intersection__sphere")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_sphere: {e}"))?;
    let hit_sphere_shadow = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__shadow")),
            intersection: Some((&module, "__intersection__sphere_shadow")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_sphere_shadow: {e}"))?;
    let hit_omm = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__hit")),
            any_hit: Some((&module, "__anyhit__omm")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_omm: {e}"))?;
    let hit_omm_shadow = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__shadow")),
            any_hit: Some((&module, "__anyhit__omm")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_omm_shadow: {e}"))?;
    let hit_csg = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__csg")),
            intersection: Some((&module, "__intersection__csg")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_csg: {e}"))?;
    let hit_csg_shadow = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__shadow")),
            intersection: Some((&module, "__intersection__csg_shadow")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_csg_shadow: {e}"))?;
    let hit_csg_prim = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__csg")),
            intersection: Some((&module, "__intersection__csg_prim")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_csg_prim: {e}"))?;
    let hit_csg_prim_shadow = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__shadow")),
            intersection: Some((&module, "__intersection__csg_prim_shadow")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_csg_prim_shadow: {e}"))?;

    let (pipeline, _) = Pipeline::new(
        optix_ctx,
        &opts,
        LinkOptions::default(),
        &[
            raygen.clone(),
            raygen_probe.clone(),
            raygen_sdg.clone(),
            miss.clone(),
            miss_shadow.clone(),
            hit_floor.clone(),
            hit_floor_shadow.clone(),
            hit_fractal.clone(),
            hit_fractal_shadow.clone(),
            hit_sphere.clone(),
            hit_sphere_shadow.clone(),
            hit_omm.clone(),
            hit_omm_shadow.clone(),
            hit_csg.clone(),
            hit_csg_shadow.clone(),
            hit_csg_prim.clone(),
            hit_csg_prim_shadow.clone(),
        ],
    )
    .map_err(|e| format!("Pipeline::new: {e}"))?;

    let miss_recs = [
        SbtRecord::new(&miss, 0u32).map_err(|e| format!("sbt miss0: {e}"))?,
        SbtRecord::new(&miss_shadow, 0u32).map_err(|e| format!("sbt miss1: {e}"))?,
    ];
    let hit_recs = [
        SbtRecord::new(&hit_floor, 0u32).map_err(|e| format!("sbt hit0: {e}"))?,
        SbtRecord::new(&hit_floor_shadow, 0u32).map_err(|e| format!("sbt hit1: {e}"))?,
        SbtRecord::new(&hit_fractal, 0u32).map_err(|e| format!("sbt hit2: {e}"))?,
        SbtRecord::new(&hit_fractal_shadow, 0u32).map_err(|e| format!("sbt hit3: {e}"))?,
        SbtRecord::new(&hit_sphere, 0u32).map_err(|e| format!("sbt hit4: {e}"))?,
        SbtRecord::new(&hit_sphere_shadow, 0u32).map_err(|e| format!("sbt hit5: {e}"))?,
        SbtRecord::new(&hit_omm, 0u32).map_err(|e| format!("sbt hit6: {e}"))?,
        SbtRecord::new(&hit_omm_shadow, 0u32).map_err(|e| format!("sbt hit7: {e}"))?,
        SbtRecord::new(&hit_csg, 0u32).map_err(|e| format!("sbt hit8: {e}"))?,
        SbtRecord::new(&hit_csg_shadow, 0u32).map_err(|e| format!("sbt hit9: {e}"))?,
        SbtRecord::new(&hit_csg_prim, 0u32).map_err(|e| format!("sbt hit10: {e}"))?,
        SbtRecord::new(&hit_csg_prim_shadow, 0u32).map_err(|e| format!("sbt hit11: {e}"))?,
    ];
    let sbt = ShaderBindingTable::new(
        stream,
        &SbtRecord::new(&raygen, 0u32).map_err(|e| format!("sbt raygen: {e}"))?,
        &miss_recs,
        &hit_recs,
    )
    .map_err(|e| format!("SBT::new: {e}"))?;
    let probe_sbt = ShaderBindingTable::new(
        stream,
        &SbtRecord::new(&raygen_probe, 0u32).map_err(|e| format!("sbt probe raygen: {e}"))?,
        &miss_recs,
        &hit_recs,
    )
    .map_err(|e| format!("probe SBT::new: {e}"))?;
    let sdg_sbt = ShaderBindingTable::new(
        stream,
        &SbtRecord::new(&raygen_sdg, 0u32).map_err(|e| format!("sbt sdg raygen: {e}"))?,
        &miss_recs,
        &hit_recs,
    )
    .map_err(|e| format!("sdg SBT::new: {e}"))?;

    Ok((pipeline, sbt, probe_sbt, sdg_sbt))
}

fn create_denoiser(
    optix_ctx: &Arc<DeviceContext>,
    stream: &Arc<Stream>,
    width: u32,
    height: u32,
    temporal: bool,
) -> (Denoiser, bool) {
    if temporal
        && let Ok(d) = Denoiser::new(
            optix_ctx,
            stream,
            DenoiserOptions {
                model: DenoiserModel::TemporalAov,
                albedo: true,
                normal: true,
                ..Default::default()
            },
            width,
            height,
        )
    {
        return (d, true);
    }
    let d = Denoiser::new(
        optix_ctx,
        stream,
        DenoiserOptions {
            model: DenoiserModel::Aov,
            albedo: true,
            normal: true,
            ..Default::default()
        },
        width,
        height,
    )
    .expect("Denoiser");
    (d, false)
}

fn build_uv_sphere(stacks: u32, slices: u32) -> Vec<[f32; 3]> {
    use std::f32::consts::{PI, TAU};
    let mut tris = Vec::with_capacity((stacks * slices * 6) as usize);
    let pt = |i: u32, j: u32| -> [f32; 3] {
        let phi = PI * (i as f32 / stacks as f32);
        let theta = TAU * (j as f32 / slices as f32);
        let s = phi.sin();
        [s * theta.cos(), phi.cos(), s * theta.sin()]
    };
    for i in 0..stacks {
        for j in 0..slices {
            let p00 = pt(i, j);
            let p10 = pt(i + 1, j);
            let p11 = pt(i + 1, j + 1);
            let p01 = pt(i, j + 1);
            if i > 0 {
                tris.extend_from_slice(&[p00, p11, p01]);
            }
            if i + 1 < stacks {
                tris.extend_from_slice(&[p00, p10, p11]);
            }
        }
    }
    tris
}

fn build_sphere_mesh_gas(
    optix_ctx: &Arc<DeviceContext>,
    stream: &Arc<Stream>,
    scratch: &mut Scratch,
    u_segs: u32,
    v_segs: u32,
) -> Arc<Accel> {
    let sphere_tris = build_uv_sphere(v_segs.max(4), u_segs.max(4));
    let sphere_verts = DeviceBuffer::from_host(stream, &sphere_tris).expect("sphere verts");
    let tri_build_opts = BuildOptions {
        prefer_fast_trace: true,
        ..Default::default()
    };
    Accel::triangles(
        optix_ctx,
        stream,
        scratch,
        &[TriangleMesh {
            flags: &[GeometryFlags::DISABLE_ANYHIT],
            ..TriangleMesh::new(&sphere_verts)
        }],
        tri_build_opts,
    )
    .expect("glass mesh GAS")
}

fn build_torus_ring(major_r: f32, minor_r: f32, segs: u32, tube_segs: u32) -> Vec<[f32; 3]> {
    use std::f32::consts::TAU;
    let mut tris = Vec::with_capacity((segs * tube_segs * 6) as usize);
    let pt = |i: u32, j: u32| -> [f32; 3] {
        let u = TAU * (i as f32 / segs as f32);
        let v = TAU * (j as f32 / tube_segs as f32);
        let r = major_r + minor_r * v.cos();
        [r * u.cos(), minor_r * v.sin(), r * u.sin()]
    };
    for i in 0..segs {
        for j in 0..tube_segs {
            let p00 = pt(i, j);
            let p10 = pt(i + 1, j);
            let p11 = pt(i + 1, j + 1);
            let p01 = pt(i, j + 1);
            tris.extend_from_slice(&[p00, p10, p11, p00, p11, p01]);
        }
    }
    tris
}

/// Exact 1-Lipschitz 2D SDF for the perforated filigree halo band; matches `filigree_sdf` in
/// `shaders/raytracer.cu` bit-for-bit. `<= 0` is solid metal, `> 0` is transparent cutout.
fn filigree_sdf(u: f32, v: f32) -> f32 {
    let d_border = v.min(1.0 - v) - 0.06;
    let s1 = u * 2.0 + v * 2.0;
    let f1 = s1 - s1.floor();
    let d_diag1 = ((f1 - 0.5).abs() - 0.075) * 0.353_553_38;
    let s2 = u * 2.0 - v * 2.0 + 4.0;
    let f2 = s2 - s2.floor();
    let d_diag2 = ((f2 - 0.5).abs() - 0.075) * 0.353_553_38;
    let dx = u - 0.5;
    let dy = v - 0.5;
    let d_ring = ((dx * dx + dy * dy).sqrt() - 0.30).abs() - 0.042;
    let cu = u.min(1.0 - u);
    let cv = v.min(1.0 - v);
    let d_corner = ((cu * cu + cv * cv).sqrt() - 0.22).abs() - 0.036;
    d_border.min(d_diag1.min(d_diag2).min(d_ring.min(d_corner)))
}

/// Builds a cylindrical filigree halo band (`segs` quads = `2 * segs` triangles).
/// Even triangles `(p00, p10, p11)` have UVs `(0,0)-(1,0)-(1,1)`; odd triangles `(p00, p11, p01)`
/// have UVs `(0,0)-(1,1)-(0,1)` so all quads share just two opacity micromaps.
fn build_filigree_band_tris(radius: f32, half_h: f32, segs: u32) -> Vec<[f32; 3]> {
    use std::f32::consts::TAU;
    let mut tris = Vec::with_capacity((segs * 6) as usize);
    for i in 0..segs {
        let a0 = TAU * (i as f32 / segs as f32);
        let a1 = TAU * ((i + 1) as f32 / segs as f32);
        let p00 = [radius * a0.cos(), -half_h, radius * a0.sin()];
        let p10 = [radius * a1.cos(), -half_h, radius * a1.sin()];
        let p11 = [radius * a1.cos(), half_h, radius * a1.sin()];
        let p01 = [radius * a0.cos(), half_h, radius * a0.sin()];
        tris.extend_from_slice(&[p00, p10, p11, p00, p11, p01]);
    }
    tris
}

/// Bakes 2 shared 4-state opacity micromaps (`[even, odd]`) at subdivision `level` using
/// 1-Lipschitz SDF circumradius bounds so OMM and Any-Hit are pixel-identical.
fn bake_filigree_omm_bytes(level: u32) -> Vec<u8> {
    const STATE_TRANSPARENT: u8 = 0;
    const STATE_OPAQUE: u8 = 1;
    const STATE_UNKNOWN_TRANSPARENT: u8 = 2;
    const STATE_UNKNOWN_OPAQUE: u8 = 3;

    let micro_count = 1u32 << (2 * level);
    let bytes_per_map = ((micro_count as usize) * 2).div_ceil(8);
    let mut bytes = vec![0u8; bytes_per_map * 2];

    for (map_idx, odd) in [false, true].into_iter().enumerate() {
        let map_slice = &mut bytes[map_idx * bytes_per_map..(map_idx + 1) * bytes_per_map];
        for idx in 0..micro_count {
            let barys = micro_to_base_bary(idx, level).expect("micro_to_base_bary");
            let uvs: [[f32; 2]; 3] = barys.map(|[b1, b2]| {
                if odd {
                    [b1, b1 + b2]
                } else {
                    [b1 + b2, b2]
                }
            });
            let uc = (uvs[0][0] + uvs[1][0] + uvs[2][0]) * (1.0 / 3.0);
            let vc = (uvs[0][1] + uvs[1][1] + uvs[2][1]) * (1.0 / 3.0);
            let mut rad = 0.0f32;
            for [u, v] in uvs {
                let du = u - uc;
                let dv = v - vc;
                rad = rad.max((du * du + dv * dv).sqrt());
            }
            // Small safety margin covers f32 rounding so opaque/transparent claims are exact.
            let margin = rad * 1.02 + 1.0e-5;
            let d = filigree_sdf(uc, vc);
            let state = if d <= -margin {
                STATE_OPAQUE
            } else if d > margin {
                STATE_TRANSPARENT
            } else if d <= 0.0 {
                STATE_UNKNOWN_OPAQUE
            } else {
                STATE_UNKNOWN_TRANSPARENT
            };
            map_slice[(idx / 4) as usize] |= state << ((idx % 4) * 2);
        }
    }
    bytes
}

/// Builds both the hardware OMM GAS and the unaccelerated Any-Hit GAS over the same filigree halo
/// geometry so toggling `OMM` vs `Any-Hit` is a fair, zero-overhead benchmark comparison.
fn build_omm_gases(
    optix_ctx: &Arc<DeviceContext>,
    stream: &Arc<Stream>,
    scratch: &mut Scratch,
) -> (Arc<Accel>, Arc<Accel>) {
    const OMM_SEGS: u32 = 24;
    const OMM_LEVEL: u32 = 5;
    let tris = build_filigree_band_tris(2.15, 0.45, OMM_SEGS);
    let tri_count = (tris.len() / 3) as u32;
    let verts = DeviceBuffer::from_host(stream, &tris).expect("omm verts");

    let omm_bytes = bake_filigree_omm_bytes(OMM_LEVEL);
    let per_map_bytes = (omm_bytes.len() / 2) as u32;
    let omm_data = DeviceBuffer::from_host(stream, &omm_bytes).expect("omm data");
    let descs = [
        MicromapDesc::new(0, OMM_LEVEL as u16, MicromapFormat::FourState),
        MicromapDesc::new(per_map_bytes, OMM_LEVEL as u16, MicromapFormat::FourState),
    ];
    let omm_array = OpacityMicromapArray::build(
        optix_ctx,
        stream,
        scratch,
        &omm_data,
        &descs,
        MicromapOptions::default(),
    )
    .expect("OpacityMicromapArray");

    let indices_host: Vec<i16> = (0..tri_count).map(|i| (i & 1) as i16).collect();
    let indices_buf = DeviceBuffer::from_host(stream, &indices_host).expect("omm indices");
    let usage = [MicromapCount {
        count: tri_count,
        level: OMM_LEVEL,
        format: MicromapFormat::FourState,
    }];
    let build_opts = BuildOptions {
        prefer_fast_trace: true,
        random_vertex_access: true,
        ..Default::default()
    };

    // SAFETY: every index in `indices_buf` is `0` or `1` (both valid micromaps of `omm_array`
    // at `OMM_LEVEL` / `FourState`), and `usage` counts all `tri_count` references.
    let omm_use = unsafe {
        MicromapUse::indexed(&omm_array, MicromapIndices::I16(&indices_buf), 0, &usage)
    };
    let omm_gas = Accel::triangles(
        optix_ctx,
        stream,
        scratch,
        &[TriangleMesh {
            flags: &[GeometryFlags::NONE],
            micromap: Some(omm_use),
            ..TriangleMesh::new(&verts)
        }],
        build_opts,
    )
    .expect("omm GAS");

    let omm_anyhit_gas = Accel::triangles(
        optix_ctx,
        stream,
        scratch,
        &[TriangleMesh {
            flags: &[GeometryFlags::NONE],
            ..TriangleMesh::new(&verts)
        }],
        build_opts,
    )
    .expect("omm anyhit GAS");

    (omm_gas, omm_anyhit_gas)
}

struct SceneGas<'a> {
    floor: &'a Arc<Accel>,
    fractal: &'a Arc<Accel>,
    glass_analytic: &'a Arc<Accel>,
    glass_mesh: &'a Arc<Accel>,
    neon: &'a Arc<Accel>,
    omm: &'a Arc<Accel>,
    omm_anyhit: &'a Arc<Accel>,
}

/// Builds the row-major 3x4 OptiX TLAS instance transform `T * R_z * R_y * R_x * S`
/// for the Hero Fractal.
///
/// - `translate`: world-space position `[tx, ty, tz]` in meters (non-finite components default to `0.0`).
/// - `rotate_deg`: Euler angles `[rx, ry, rz]` in degrees applied in `R_z * R_y * R_x` order
///   (non-finite components default to `0.0`).
/// - `scale`: uniform object-to-world scale clamped to `0.05..=50.0` (non-finite defaults to `1.0`).
pub fn fractal_instance_transform(
    translate: [f32; 3],
    rotate_deg: [f32; 3],
    scale: f32,
) -> [f32; 12] {
    let tx = if translate[0].is_finite() { translate[0] } else { 0.0 };
    let ty = if translate[1].is_finite() { translate[1] } else { 0.0 };
    let tz = if translate[2].is_finite() { translate[2] } else { 0.0 };
    let rx = if rotate_deg[0].is_finite() {
        rotate_deg[0].to_radians()
    } else {
        0.0
    };
    let ry = if rotate_deg[1].is_finite() {
        rotate_deg[1].to_radians()
    } else {
        0.0
    };
    let rz = if rotate_deg[2].is_finite() {
        rotate_deg[2].to_radians()
    } else {
        0.0
    };
    let s = if scale.is_finite() {
        scale.clamp(0.05, 50.0)
    } else {
        1.0
    };
    let lin = glam::Mat3::from_rotation_z(rz)
        * glam::Mat3::from_rotation_y(ry)
        * glam::Mat3::from_rotation_x(rx)
        * s;
    [
        lin.x_axis.x,
        lin.y_axis.x,
        lin.z_axis.x,
        tx,
        lin.x_axis.y,
        lin.y_axis.y,
        lin.z_axis.y,
        ty,
        lin.x_axis.z,
        lin.y_axis.z,
        lin.z_axis.z,
        tz,
    ]
}

struct SceneState {
    sat_angle: f32,
    sat_count: u32,
    sat_seed: u32,
    sat_size_range: [f32; 2],
    sat_orbit_range: [f32; 2],
    sat_height_range: [f32; 2],
    sat_speed_range: [f32; 2],
    sat_bob_amp_range: [f32; 2],
    sat_bob_freq_range: [f32; 2],
    sat_hue_spread: f32,
    sat_on: bool,
    glass_analytic: bool,
    neon_on: bool,
    omm_mode: u32,
    show_classic: bool,
    fractal_transform: [f32; 12],
}

fn splitmix32(mut x: u32) -> f32 {
    x = x.wrapping_add(0x9E37_79B9);
    x = (x ^ (x >> 16)).wrapping_mul(0x85EB_CA6B);
    x = (x ^ (x >> 13)).wrapping_mul(0xC2B2_AE35);
    x ^= x >> 16;
    (x >> 8) as f32 * (1.0 / 16_777_216.0)
}

/// OptiX build options for the persistent top-level IAS (`ALLOW_UPDATE` enables zero-churn
/// in-place `OPTIX_BUILD_OPERATION_UPDATE` on traffic-only frames; `compact = false` is required
/// by `Accel::instances_into`).
fn ias_build_opts() -> BuildOptions {
    BuildOptions {
        allow_update: true,
        prefer_fast_trace: true,
        compact: false,
        ..Default::default()
    }
}

/// Comparable classic-stage IAS state that ignores satellite/fractal transforms completely when
/// `!show_classic` so CSG scenes incur zero spurious 50k-instance IAS rebuilds.
#[derive(Clone, Copy, Debug, PartialEq)]
enum IasClassicState {
    Hidden,
    Visible {
        sat_on: bool,
        glass_analytic: bool,
        sat_u_segs: u32,
        sat_v_segs: u32,
        sat_count: u32,
        sat_seed: u32,
        neon_on: bool,
        omm_mode: u32,
        sat_params: [f32; 14],
        fractal_transform: [f32; 12],
    },
}

/// Computes the classic-stage IAS comparison key and fractal transform, guarding `sat_angle` and
/// `fractal_transform` strictly behind `show_classic` (`csg_mode == 0 || csg_show_fractal`).
fn compute_ias_classic_state(
    settings: &DemoSettings,
    spin_angle: f32,
    sat_angle: f32,
) -> (IasClassicState, [f32; 12]) {
    let show_classic = settings.csg_mode == 0 || settings.csg_show_fractal;
    if !show_classic {
        return (IasClassicState::Hidden, Instance::IDENTITY);
    }
    let fractal_transform = fractal_instance_transform(
        settings.fractal_translate,
        [
            settings.rot_x,
            settings.rot_y + spin_angle.to_degrees(),
            settings.rot_z,
        ],
        settings.obj_scale,
    );
    let sat_angle_key = if settings.sat_on && settings.sat_count > 0 {
        sat_angle
    } else {
        0.0
    };
    let state = IasClassicState::Visible {
        sat_on: settings.sat_on,
        glass_analytic: settings.glass_analytic,
        sat_u_segs: settings.sat_u_segs,
        sat_v_segs: settings.sat_v_segs,
        sat_count: settings.sat_count,
        sat_seed: settings.sat_seed,
        neon_on: settings.neon_on,
        omm_mode: settings.omm_mode,
        sat_params: [
            settings.sat_size_min,
            settings.sat_size_max,
            settings.sat_orbit_min,
            settings.sat_orbit_max,
            settings.sat_height_min,
            settings.sat_height_max,
            settings.sat_speed_min,
            settings.sat_speed_max,
            settings.sat_bob_amp_min,
            settings.sat_bob_amp_max,
            settings.sat_bob_freq_min,
            settings.sat_bob_freq_max,
            settings.sat_hue_spread,
            sat_angle_key,
        ],
        fractal_transform,
    };
    (state, fractal_transform)
}

#[expect(
    clippy::too_many_arguments,
    reason = "coordinates persistent IAS buffer, scratch, GAS handles, and scene state with zero alloc"
)]
fn build_ias_into(
    optix_ctx: &Arc<DeviceContext>,
    stream: &Arc<Stream>,
    scratch: &mut Scratch,
    d_instances: &mut InstanceBuffer,
    target_ias: &mut Option<Arc<Accel>>,
    gas: SceneGas<'_>,
    csg_host: &CsgGpuHost,
    instances: &mut Vec<Instance>,
    state: SceneState,
) -> usize {
    instances.clear();
    if state.show_classic {
        let count = state.sat_count.min(24) as usize;
        let mut i0 = Instance::new(gas.floor);
        i0.id = 0;
        i0.sbt_offset = 0;
        i0.flags = OPTIX_INST_DISABLE_ANYHIT;
        instances.push(i0);

        let mut i1 = Instance::new(gas.fractal);
        i1.id = 1;
        i1.sbt_offset = 2;
        i1.flags = OPTIX_INST_DISABLE_ANYHIT;
        i1.transform = state.fractal_transform;
        instances.push(i1);

        if state.sat_on && count > 0 {
            let (glass_gas, sbt_off) = if state.glass_analytic {
                (gas.glass_analytic, 4)
            } else {
                (gas.glass_mesh, 0)
            };
            let s_min = state.sat_size_range[0]
                .min(state.sat_size_range[1])
                .max(0.001);
            let s_max = state.sat_size_range[0]
                .max(state.sat_size_range[1])
                .max(0.001);
            let o_min = state.sat_orbit_range[0].min(state.sat_orbit_range[1]);
            let o_max = state.sat_orbit_range[0].max(state.sat_orbit_range[1]);
            let h_min = state.sat_height_range[0].min(state.sat_height_range[1]);
            let h_max = state.sat_height_range[0].max(state.sat_height_range[1]);
            let v_min = state.sat_speed_range[0].min(state.sat_speed_range[1]);
            let v_max = state.sat_speed_range[0].max(state.sat_speed_range[1]);
            let a_min = state.sat_bob_amp_range[0].min(state.sat_bob_amp_range[1]);
            let a_max = state.sat_bob_amp_range[0].max(state.sat_bob_amp_range[1]);
            let f_min = state.sat_bob_freq_range[0].min(state.sat_bob_freq_range[1]);
            let f_max = state.sat_bob_freq_range[0].max(state.sat_bob_freq_range[1]);
            let hue_spread = state.sat_hue_spread.clamp(0.0, 1.0);

            for idx in 0..count {
                let base = state
                    .sat_seed
                    .wrapping_mul(747_796_405)
                    .wrapping_add((idx as u32).wrapping_mul(2_891_336_453));
                let u0 = splitmix32(base ^ 0x1111_1111);
                let u1 = splitmix32(base ^ 0x2222_2222);
                let u2 = splitmix32(base ^ 0x3333_3333);
                let u3 = splitmix32(base ^ 0x4444_4444);
                let u4 = splitmix32(base ^ 0x5555_5555);
                let u5 = splitmix32(base ^ 0x6666_6666);
                let u6 = splitmix32(base ^ 0x7777_7777);
                let u7 = splitmix32(base ^ 0x8888_8888);
                let u8 = splitmix32(base ^ 0x9999_9999);

                let r = s_min + (s_max - s_min) * u0;
                // Stratify orbital radii across the band with seeded jitter so spheres stay spaced.
                let t_orbit = (idx as f32 + 0.15 + 0.70 * u1) / (count as f32);
                let orbit_r = o_min + (o_max - o_min) * t_orbit;
                let height = h_min + (h_max - h_min) * u2;
                let phase0 = u3 * std::f32::consts::TAU;
                let speed = v_min + (v_max - v_min) * u4;
                let angle = phase0 + state.sat_angle * speed;
                let bob_amp = a_min + (a_max - a_min) * u5;
                let bob_freq = f_min + (f_max - f_min) * u6;
                let bob_phase = u7 * std::f32::consts::TAU;
                let bob = (state.sat_angle * bob_freq + bob_phase).sin() * 0.7
                    + (state.sat_angle * bob_freq * 0.63 + bob_phase * 1.7).cos() * 0.3;
                let px = orbit_r * angle.cos();
                let py = height + bob_amp * bob;
                let pz = orbit_r * angle.sin();
                let hue = (0.48 + (u8 - 0.5) * hue_spread).rem_euclid(1.0);
                let hue_byte = (hue * 255.0).round().clamp(0.0, 255.0) as u32;

                let mut inst = Instance::new(glass_gas);
                inst.id = 2 | (hue_byte << 8);
                inst.sbt_offset = sbt_off;
                inst.flags = OPTIX_INST_DISABLE_ANYHIT;
                inst.transform = [r, 0.0, 0.0, px, 0.0, r, 0.0, py, 0.0, 0.0, r, pz];
                instances.push(inst);
            }
        }

        if state.neon_on {
            let mut i3 = Instance::new(gas.neon);
            i3.id = 3;
            i3.sbt_offset = 0;
            i3.flags = OPTIX_INST_DISABLE_ANYHIT;
            i3.transform = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -1.24, 0.0, 0.0, 1.0, 0.0];
            instances.push(i3);
        }

        if state.omm_mode == 1 || state.omm_mode == 2 {
            let omm_accel = if state.omm_mode == 1 {
                gas.omm
            } else {
                gas.omm_anyhit
            };
            let mut i4 = Instance::new(omm_accel);
            i4.id = 4;
            i4.sbt_offset = 6;
            // OMM / filigree relies on `__anyhit__omm` for unknown micro-triangles; keep flags = 0.
            i4.flags = 0;
            instances.push(i4);
        }
    }

    let classic_count = instances.len();
    csg_host.append_optix_instances(instances);

    if instances.is_empty() {
        let mut dummy = Instance::new(gas.floor);
        dummy.mask = 0;
        dummy.flags = OPTIX_INST_DISABLE_ANYHIT;
        instances.push(dummy);
    }

    d_instances
        .upload_all(stream, instances)
        .expect("IAS instance upload");
    // SAFETY: all ray-tracing launches on `stream` are strictly ordered before this build.
    let _ = unsafe {
        Accel::instances_into(
            optix_ctx,
            stream,
            scratch,
            d_instances,
            ias_build_opts(),
            true,
            target_ias,
        )
    }
    .expect("IAS");
    classic_count
}

#[cfg(has_embedded_ptx)]
fn initial_ptx() -> (Vec<u8>, Vec<u8>) {
    (
        EMBEDDED_RAYTRACER_PTX.as_bytes().to_vec(),
        EMBEDDED_TONEMAP_PTX.as_bytes().to_vec(),
    )
}

#[cfg(not(has_embedded_ptx))]
fn initial_ptx() -> (Vec<u8>, Vec<u8>) {
    (
        try_compile_ptx(RAYTRACER_CU, "raytracer.cu", true).expect("raytracer.cu PTX"),
        try_compile_ptx(TONEMAP_CU, "tonemap.cu", false).expect("tonemap.cu PTX"),
    )
}

fn init_optix(cuda: &Arc<CudaContext>, initial_settings: &DemoSettings) -> OptixRig {
    cuda.bind_to_thread()
        .expect("cuda bind_to_thread before init_optix");
    let stream = cuda.default_stream();
    let log_level = std::env::var("NV_DEMO_OPTIX_LOG_LEVEL")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(3)
        .min(4);
    let captured_logs = Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
    let sink_logs = Arc::clone(&captured_logs);
    let log_sink: Box<optix::LogFn> = Box::new(move |level, tag, msg| {
        let line = format!("[OptiX L{level} {tag}] {msg}");
        if level <= 3 {
            eprintln!("{line}");
        }
        if let Ok(mut guard) = sink_logs.lock() {
            if guard.len() >= 64 {
                guard.remove(0);
            }
            guard.push(line);
        }
    });
    let optix_ctx = DeviceContext::new(cuda, Some(log_sink), log_level, false).unwrap_or_else(|e| {
        let details = captured_logs
            .lock()
            .map(|g| g.join("\n"))
            .unwrap_or_default();
        let cache_hint = matches!(e, optix::Error::Optix { code: 7999, .. })
            .then_some(
                "\nHint: OPTIX_ERROR_INTERNAL_ERROR (7999) is commonly caused by a locked or corrupt \
                 %LOCALAPPDATA%\\NVIDIA\\OptixCache database. Clear OptixCache or run with \
                 OPTIX_CACHE_MAXSIZE=0.",
            )
            .unwrap_or("");
        panic!("OptiX DeviceContext failed: {e}{cache_hint}\nOptiX log:\n{details}");
    });
    // Configure an isolated per-app OptiX disk cache directory unless the user overrode it via env.
    let env_cache_path_set = std::env::var_os("OPTIX_CACHE_PATH").is_some_and(|v| !v.is_empty());
    let env_cache_disabled = std::env::var("OPTIX_CACHE_MAXSIZE")
        .ok()
        .is_some_and(|v| v.trim() == "0");
    if !env_cache_path_set && !env_cache_disabled {
        let optix_cache_dir = crate::gpu_export::nv_demo_cache_dir("optix_cache");
        if std::fs::create_dir_all(&optix_cache_dir).is_ok()
            && let Err(e) = optix_ctx.set_cache_location(&optix_cache_dir)
        {
            eprintln!(
                "nv-demo: failed to set isolated OptiX cache '{}': {e}",
                optix_cache_dir.display()
            );
        }
    }
    let ser_supported = optix_ctx.shader_reordering().unwrap_or(false);
    let mut scratch = Scratch::new(&stream);

    let (rt_ptx, tm_ptx) = initial_ptx();
    let mut cu_mod: cu::CUmodule = std::ptr::null_mut();
    // SAFETY: `tm_ptx` is NUL-terminated PTX compiled by NVRTC on the current context.
    let res = unsafe { cu::cuModuleLoadData(&mut cu_mod, tm_ptx.as_ptr().cast()) };
    assert_eq!(
        res,
        cu::cudaError_enum_CUDA_SUCCESS,
        "cuModuleLoadData failed"
    );
    let mut cu_func: cu::CUfunction = std::ptr::null_mut();
    let fn_name = CString::new("tonemap_f32_to_rgba8").unwrap();
    // SAFETY: `cu_mod` is valid and `fn_name` is NUL-terminated.
    let res = unsafe { cu::cuModuleGetFunction(&mut cu_func, cu_mod, fn_name.as_ptr()) };
    assert_eq!(
        res,
        cu::cudaError_enum_CUDA_SUCCESS,
        "cuModuleGetFunction failed"
    );
    let mut scrgb_func: cu::CUfunction = std::ptr::null_mut();
    let scrgb_name = CString::new("tonemap_f32_to_scrgb_f16").unwrap();
    // SAFETY: `cu_mod` is valid and `scrgb_name` is NUL-terminated.
    let res = unsafe { cu::cuModuleGetFunction(&mut scrgb_func, cu_mod, scrgb_name.as_ptr()) };
    assert_eq!(
        res,
        cu::cudaError_enum_CUDA_SUCCESS,
        "cuModuleGetFunction(tonemap_f32_to_scrgb_f16) failed"
    );
    let mut pack_rr_func: cu::CUfunction = std::ptr::null_mut();
    let pack_name = CString::new("pack_dlss_rr_aovs_f16").unwrap();
    // SAFETY: `cu_mod` is valid and `pack_name` is NUL-terminated.
    let res = unsafe { cu::cuModuleGetFunction(&mut pack_rr_func, cu_mod, pack_name.as_ptr()) };
    assert_eq!(
        res,
        cu::cudaError_enum_CUDA_SUCCESS,
        "cuModuleGetFunction(pack_dlss_rr_aovs_f16) failed"
    );
    let tonemap = TonemapKernel {
        cuda: cuda.clone(),
        module: cu_mod,
        func: cu_func,
        scrgb_func,
        pack_rr_func,
    };

    let (pipeline, sbt, probe_sbt, sdg_sbt) =
        build_optix_pipeline(&optix_ctx, &stream, &rt_ptx).expect("OptiX pipeline");

    // Raytraced metal stage floor (~10x the [-1.5, 1.5] fractal, y = -1.35)
    let floor_verts = DeviceBuffer::from_host(
        &stream,
        &[
            [-15.0f32, -1.35, -15.0],
            [15.0, -1.35, -15.0],
            [15.0, -1.35, 15.0],
            [-15.0, -1.35, -15.0],
            [15.0, -1.35, 15.0],
            [-15.0, -1.35, 15.0],
        ],
    )
    .expect("floor verts");
    let tri_build_opts = BuildOptions {
        random_vertex_access: true,
        ..BuildOptions::default()
    };
    let floor_gas = Accel::triangles(
        &optix_ctx,
        &stream,
        &mut scratch,
        &[TriangleMesh {
            flags: &[GeometryFlags::DISABLE_ANYHIT],
            ..TriangleMesh::new(&floor_verts)
        }],
        tri_build_opts,
    )
    .expect("floor GAS");

    // Fractal AABB custom primitive [-1.85, 1.85]^3 (matches raytracer.cu; avoids rotated KIFS corner clipping)
    let aabb_buf = DeviceBuffer::from_host(
        &stream,
        &[Aabb {
            min: [-1.85, -1.85, -1.85],
            max: [1.85, 1.85, 1.85],
        }],
    )
    .expect("aabb");
    let fractal_gas = Accel::custom(
        &optix_ctx,
        &stream,
        &mut scratch,
        &[CustomPrimitives::new(&aabb_buf)],
        BuildOptions::default(),
    )
    .expect("fractal GAS");

    // Analytic glass satellite sphere (unit sphere AABB, transformed via IAS)
    let sphere_aabb_buf = DeviceBuffer::from_host(
        &stream,
        &[Aabb {
            min: [-1.0, -1.0, -1.0],
            max: [1.0, 1.0, 1.0],
        }],
    )
    .expect("sphere aabb");
    let glass_analytic_gas = Accel::custom(
        &optix_ctx,
        &stream,
        &mut scratch,
        &[CustomPrimitives::new(&sphere_aabb_buf)],
        BuildOptions::default(),
    )
    .expect("glass analytic GAS");
    let glass_mesh_segs = (initial_settings.sat_u_segs, initial_settings.sat_v_segs);
    let glass_mesh_gas = build_sphere_mesh_gas(
        &optix_ctx,
        &stream,
        &mut scratch,
        glass_mesh_segs.0,
        glass_mesh_segs.1,
    );

    // Emissive neon accent ring around the stage
    let ring_tris = build_torus_ring(1.95, 0.045, 72, 14);
    let ring_verts = DeviceBuffer::from_host(&stream, &ring_tris).expect("ring verts");
    let neon_gas = Accel::triangles(
        &optix_ctx,
        &stream,
        &mut scratch,
        &[TriangleMesh {
            flags: &[GeometryFlags::DISABLE_ANYHIT],
            ..TriangleMesh::new(&ring_verts)
        }],
        tri_build_opts,
    )
    .expect("neon GAS");

    // Perforated filigree halo GAS pair (hardware OMM vs pure Any-Hit benchmark)
    let (omm_gas, omm_anyhit_gas) = build_omm_gases(&optix_ctx, &stream, &mut scratch);

    let mut csg_host = CsgGpuHost::new();
    let initial_csg_key = CsgSceneKey {
        scene_idx: initial_settings.csg_mode,
        variant_idx: initial_settings.csg_asset_idx,
        seed: initial_settings.csg_seed,
        road_mode: initial_settings.csg_road_mode.min(3),
    };
    if let Err(err) = csg_host.sync(
        &optix_ctx,
        &stream,
        &mut scratch,
        initial_csg_key,
        initial_settings.csg_wetness,
        initial_settings.csg_time_of_day,
        CsgCameraParams {
            eye: [0.0, 2.0, -5.0],
            fwd: [0.0, 0.0, 1.0],
            up: [0.0, 1.0, 0.0],
            viewport_w: DEFAULT_W,
            viewport_h: DEFAULT_H,
            vfov_rad: std::f32::consts::FRAC_PI_3,
            lod_config: initial_settings.csg_lod_config(),
            omnidirectional: initial_settings.camera_projection.is_omnidirectional(),
        },
        0.0,
        initial_settings.csg_freeze_pose,
        false,
    ) {
        eprintln!("nv-demo: initial CSG scene sync failed: {err}");
    }
    let mut ias_instances_scratch = Vec::with_capacity(4096);
    let ias_dyn_scratch = Vec::with_capacity(1024);
    let mut d_optix_instances = InstanceBuffer::new(&optix_ctx, &stream);
    let mut ias: Option<Arc<Accel>> = None;
    let show_classic = initial_settings.csg_mode == 0 || initial_settings.csg_show_fractal;

    let ias_classic_count = build_ias_into(
        &optix_ctx,
        &stream,
        &mut scratch,
        &mut d_optix_instances,
        &mut ias,
        SceneGas {
            floor: &floor_gas,
            fractal: &fractal_gas,
            glass_analytic: &glass_analytic_gas,
            glass_mesh: &glass_mesh_gas,
            neon: &neon_gas,
            omm: &omm_gas,
            omm_anyhit: &omm_anyhit_gas,
        },
        &csg_host,
        &mut ias_instances_scratch,
        SceneState {
            sat_angle: 0.0,
            sat_count: initial_settings.sat_count,
            sat_seed: initial_settings.sat_seed,
            sat_size_range: [initial_settings.sat_size_min, initial_settings.sat_size_max],
            sat_orbit_range: [
                initial_settings.sat_orbit_min,
                initial_settings.sat_orbit_max,
            ],
            sat_height_range: [
                initial_settings.sat_height_min,
                initial_settings.sat_height_max,
            ],
            sat_speed_range: [
                initial_settings.sat_speed_min,
                initial_settings.sat_speed_max,
            ],
            sat_bob_amp_range: [
                initial_settings.sat_bob_amp_min,
                initial_settings.sat_bob_amp_max,
            ],
            sat_bob_freq_range: [
                initial_settings.sat_bob_freq_min,
                initial_settings.sat_bob_freq_max,
            ],
            sat_hue_spread: initial_settings.sat_hue_spread,
            sat_on: initial_settings.sat_on,
            glass_analytic: initial_settings.glass_analytic,
            neon_on: initial_settings.neon_on,
            omm_mode: initial_settings.omm_mode,
            show_classic,
            fractal_transform: fractal_instance_transform(
                initial_settings.fractal_translate,
                [
                    initial_settings.rot_x,
                    initial_settings.rot_y,
                    initial_settings.rot_z,
                ],
                initial_settings.obj_scale,
            ),
        },
    );

    let n = (DEFAULT_W * DEFAULT_H) as usize;
    let color_f32 = DeviceBuffer::zeroed(&stream, n).expect("color_f32");
    let albedo_f32 = DeviceBuffer::zeroed(&stream, n).expect("albedo_f32");
    let normal_f32 = DeviceBuffer::zeroed(&stream, n).expect("normal_f32");
    let params_buf =
        DeviceBuffer::zeroed(&stream, std::mem::size_of::<LaunchParams>()).expect("params");
    let bloom = FftBloomPass::new(cuda).ok();
    let gpu_profiler = GpuPassProfiler::new(cuda).ok();
    stream.synchronize().expect("init sync");

    OptixRig {
        stream,
        _optix_ctx: optix_ctx,
        _scratch: scratch,
        floor_gas,
        fractal_gas,
        glass_analytic_gas,
        glass_mesh_gas,
        glass_mesh_segs,
        neon_gas,
        omm_gas,
        omm_anyhit_gas,
        ias,
        d_optix_instances,
        ias_classic_count,
        pipeline,
        sbt,
        probe_sbt,
        sdg_sbt,
        ser_supported,
        denoiser: None,
        denoiser_is_temporal: initial_settings.temporal_denoise,
        temporal_has_prev: false,
        color_f32,
        albedo_f32,
        diffuse_albedo_f32: None,
        spec_albedo_f32: None,
        normal_f32,
        denoised_f32: None,
        prev_denoised_f32: None,
        motion_f32: None,
        depth_f32: None,
        semantic_u32: None,
        instance_u32: None,
        params_buf,
        tonemap,
        bloom,
        gpu_profiler,
        csg_host,
        ias_instances_scratch,
        ias_dyn_scratch,
        probe_candidates_buf: None,
        probe_samples_buf: None,
        probe_candidates_host: Vec::with_capacity(128),
        probe_samples_host: Vec::with_capacity(128 * 192),
        sdg_queries_buf: None,
        sdg_results_buf: None,
        sdg_queries_host: Vec::with_capacity(256),
        sdg_results_host: Vec::with_capacity(256),
        cpu_prober: CpuViewProber::new(),
        wgpu_prober: None,
        cuda_crowd: None,
        crowd_slot_map_host: Vec::with_capacity(1024),
        crowd_gas_handles_host: Vec::with_capacity(256),
    }
}

/// Lazily initializes `rig.cuda_crowd` once on first use, caching either the compiled NVRTC
/// backend or the diagnostic failure string so failed builds are not retried every frame.
fn ensure_cuda_crowd<'a>(
    rig: &'a mut OptixRig,
    cuda: &Arc<CudaContext>,
) -> Result<&'a mut CudaCrowdBackend, String> {
    if rig.cuda_crowd.is_none() {
        rig.cuda_crowd = Some(CudaCrowdBackend::new(cuda));
    }
    match rig.cuda_crowd.as_mut() {
        Some(Ok(cc)) => Ok(cc),
        Some(Err(e)) => Err(e.clone()),
        None => Err("CUDA crowd backend uninitialized".to_string()),
    }
}

/// Pulls GPU-stepped crowd poses (`states`, `bone_outs`, `prev_xforms`, `world_joints`) back into
/// `rig.csg_host`'s CPU `CrowdSim` (and optionally `WorldScene`) whenever CPU consumers need fresh
/// geometry (camera repack, `Ctrl+Click` picking, `SpawnCrowdStress`, or SDG JSON export).
fn sync_cuda_crowd_to_cpu(rig: &mut OptixRig, sync_to_world: bool) -> Result<(), String> {
    let Some(Ok(cc)) = rig.cuda_crowd.as_mut() else {
        return Ok(());
    };
    if !cc.cpu_bones_stale() {
        return Ok(());
    }
    let Some(scene) = rig.csg_host.active_scene_mut() else {
        return Ok(());
    };
    cc.sync_cpu_snapshot(&rig.stream, scene, sync_to_world)
}

/// Hardware OptiX 9 primary-ray [`ViewProbeBackend`] launching `__raygen__score_views`
/// across our existing split-SBT `hit_csg` / `hit_csg_prim` IAS with zero steady-state reallocations.
struct OptixViewProber<'a> {
    pipeline: &'a Pipeline,
    probe_sbt: &'a ShaderBindingTable,
    stream: &'a Arc<Stream>,
    params_buf: &'a mut DeviceBuffer<u8>,
    ias_handle: u64,
    csg_ptrs: crate::csg_scene_host::CsgDevicePointers,
    candidates_buf: &'a mut Option<DeviceBuffer<u8>>,
    samples_buf: &'a mut Option<DeviceBuffer<u8>>,
    host_candidates: &'a mut Vec<GpuProbeCandidate>,
    host_samples: &'a mut Vec<ProbeSample>,
}

impl ViewProbeBackend for OptixViewProber<'_> {
    fn kind(&self) -> ProbeBackendKind {
        ProbeBackendKind::Optix
    }

    fn probe_views(
        &mut self,
        _world: &WorldScene,
        _packed: &PackedGpuScene,
        candidates: &[ViewCandidate],
        grid_w: u32,
        grid_h: u32,
    ) -> Result<Vec<ViewScore>, String> {
        if grid_w == 0 || grid_h == 0 {
            return Err("OptiX probe grid dimensions must be non-zero".to_string());
        }
        if candidates.is_empty() {
            return Ok(Vec::new());
        }
        let rays_per_view = (grid_w as usize)
            .checked_mul(grid_h as usize)
            .ok_or_else(|| "OptiX probe ray count overflow".to_string())?;
        let total_rays = rays_per_view
            .checked_mul(candidates.len())
            .ok_or_else(|| "OptiX probe total ray count overflow".to_string())?;
        let aspect = grid_w as f32 / grid_h as f32;

        self.host_candidates.clear();
        self.host_candidates.reserve(candidates.len());
        for c in candidates {
            let fwd = if c.forward.len_sq() > 1e-10 {
                c.forward.norm()
            } else {
                csg_core::Vec3::Z
            };
            let up_ref = if c.up.len_sq() > 1e-10 {
                c.up.norm()
            } else {
                csg_core::Vec3::Y
            };
            let right_raw = fwd.cross(up_ref);
            let right = if right_raw.len_sq() > 1e-10 {
                right_raw.norm()
            } else {
                let alt = fwd.cross(csg_core::Vec3::X);
                if alt.len_sq() > 1e-10 {
                    alt.norm()
                } else {
                    fwd.cross(csg_core::Vec3::Z).norm()
                }
            };
            let up = right.cross(fwd).norm();
            let vfov_rad = if c.vfov_deg.is_finite() {
                c.vfov_deg.clamp(20.0, 110.0).to_radians()
            } else {
                55.0f32.to_radians()
            };
            let tan_half_fov = (0.5 * vfov_rad).tan();
            self.host_candidates.push(GpuProbeCandidate {
                eye: [c.eye.x, c.eye.y, c.eye.z, 0.0],
                forward: [fwd.x, fwd.y, fwd.z, aspect],
                right: [right.x, right.y, right.z, 0.0],
                up: [up.x, up.y, up.z, tan_half_fov],
            });
        }

        let cand_bytes_len = self.host_candidates.len() * std::mem::size_of::<GpuProbeCandidate>();
        if self
            .candidates_buf
            .as_ref()
            .is_none_or(|b| b.num_bytes() < cand_bytes_len)
        {
            let cap_bytes = cand_bytes_len
                .next_power_of_two()
                .max(64 * std::mem::size_of::<GpuProbeCandidate>());
            *self.candidates_buf = Some(
                DeviceBuffer::zeroed(self.stream, cap_bytes)
                    .map_err(|e| format!("OptiX probe candidates alloc failed: {e}"))?,
            );
        }
        let samp_bytes_len = total_rays * std::mem::size_of::<ProbeSample>();
        if self
            .samples_buf
            .as_ref()
            .is_none_or(|b| b.num_bytes() < samp_bytes_len)
        {
            let cap_bytes = samp_bytes_len
                .next_power_of_two()
                .max(4096 * std::mem::size_of::<ProbeSample>());
            *self.samples_buf = Some(
                DeviceBuffer::zeroed(self.stream, cap_bytes)
                    .map_err(|e| format!("OptiX probe samples alloc failed: {e}"))?,
            );
        }
        let cand_buf = self
            .candidates_buf
            .as_mut()
            .ok_or_else(|| "probe candidates buffer missing".to_string())?;
        let samp_buf = self
            .samples_buf
            .as_ref()
            .ok_or_else(|| "probe samples buffer missing".to_string())?;

        // Upload only the active candidates slice into the reusable device buffer.
        let cand_bytes: &[u8] = bytemuck::cast_slice(self.host_candidates.as_slice());
        // SAFETY: `cand_buf` holds `>= host_candidates.len()` `GpuProbeCandidate` elements on `self.stream`,
        // and `host_candidates` remains live through `self.stream.synchronize()`.
        let h2d = unsafe {
            cu::cuMemcpyHtoDAsync_v2(
                cand_buf.cu_deviceptr(),
                cand_bytes.as_ptr().cast(),
                cand_bytes.len(),
                self.stream.cu_stream(),
            )
        };
        if h2d != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("OptiX probe candidate upload failed ({h2d:?})"));
        }

        let lp = LaunchParams {
            handle: self.ias_handle,
            csg_compounds: self.csg_ptrs.compounds,
            csg_ops: self.csg_ptrs.ops,
            csg_prims: self.csg_ptrs.prims,
            csg_aux_planes: self.csg_ptrs.aux_planes,
            csg_guard_aabbs: self.csg_ptrs.guard_aabbs,
            csg_repeats: self.csg_ptrs.repeats,
            csg_materials: self.csg_ptrs.materials,
            csg_instances: self.csg_ptrs.instances,
            csg_instance_count: self.csg_ptrs.instance_count,
            csg_material_count: self.csg_ptrs.material_count,
            csg_compound_count: self.csg_ptrs.compound_count,
            csg_flags: csg_flags::ENABLED,
            probe_grid_wh: (grid_h << 16) | (grid_w & 0xFFFF),
            probe_candidates: cand_buf.cu_deviceptr(),
            probe_samples: samp_buf.cu_deviceptr(),
            ..LaunchParams::zeroed()
        };
        self.params_buf
            .copy_from_host(self.stream, bytemuck::bytes_of(&lp))
            .map_err(|e| format!("OptiX probe LaunchParams upload failed: {e}"))?;

        // SAFETY: `params_buf` matches `Params` layout; `cand_buf` and `samp_buf` are sized for
        // `candidates.len() * grid_w * grid_h` and stay alive through `stream.synchronize()`.
        unsafe {
            self.pipeline.launch(
                self.stream,
                self.params_buf,
                self.probe_sbt,
                [grid_w, grid_h, candidates.len() as u32],
            )
        }
        .map_err(|e| format!("OptiX __raygen__score_views launch failed: {e}"))?;
        self.stream
            .synchronize()
            .map_err(|e| format!("OptiX probe sync failed: {e}"))?;

        self.host_samples.resize(total_rays, ProbeSample::MISS);
        let samp_bytes: &mut [u8] = bytemuck::cast_slice_mut(&mut self.host_samples[..total_rays]);
        // SAFETY: `samp_buf` holds `>= total_rays` `ProbeSample` elements and GPU execution has synced.
        let d2h = unsafe {
            cu::cuMemcpyDtoH_v2(
                samp_bytes.as_mut_ptr().cast(),
                samp_buf.cu_deviceptr(),
                samp_bytes.len(),
            )
        };
        if d2h != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("OptiX probe readback failed ({d2h:?})"));
        }

        let mut scores = Vec::with_capacity(candidates.len());
        for (i, cand) in candidates.iter().enumerate() {
            let start = i * rays_per_view;
            let slice = &self.host_samples[start..start + rays_per_view];
            scores.push(score_probe_grid(
                slice,
                grid_w,
                grid_h,
                cand.target_sem,
                ProbeBackendKind::Optix,
            ));
        }
        Ok(scores)
    }
}

/// Evaluates `candidates` on the exact requested `ProbeBackendKind` (`Auto` selects `Optix`;
/// explicit `Optix`, `Wgpu`, and `Cpu` never silently downgrade).
fn score_views_on_rig(
    rig: &mut OptixRig,
    backend: ProbeBackendKind,
    candidates: &[ViewCandidate],
    grid_w: u32,
    grid_h: u32,
) -> Result<Vec<ViewScore>, String> {
    sync_cuda_crowd_to_cpu(rig, true)?;
    rig.csg_host.prepare_full_scene_for_probe(&rig.stream)?;
    match backend {
        ProbeBackendKind::Auto | ProbeBackendKind::Optix => {
            // Build an omnidirectional CSG-only IAS for accurate city-wide OptiX probing.
            rig.ias_instances_scratch.clear();
            rig.csg_host
                .append_optix_instances(&mut rig.ias_instances_scratch);
            if rig.ias_instances_scratch.is_empty() {
                return Err("Active CSG scene has zero instances to probe".to_string());
            }
            let probe_ias = Accel::instances(
                &rig._optix_ctx,
                &rig.stream,
                &mut rig._scratch,
                &rig.ias_instances_scratch,
                BuildOptions::default(),
            )
            .map_err(|e| format!("OptiX probe IAS build failed: {e}"))?;
            let csg_ptrs = rig.csg_host.device_pointers();
            let (world, _) = rig
                .csg_host
                .world_and_hints()
                .ok_or_else(|| "No active CSG scene".to_string())?;
            let packed = rig.csg_host.packed();
            let mut prober = OptixViewProber {
                pipeline: &rig.pipeline,
                probe_sbt: &rig.probe_sbt,
                stream: &rig.stream,
                params_buf: &mut rig.params_buf,
                ias_handle: probe_ias.handle(),
                csg_ptrs,
                candidates_buf: &mut rig.probe_candidates_buf,
                samples_buf: &mut rig.probe_samples_buf,
                host_candidates: &mut rig.probe_candidates_host,
                host_samples: &mut rig.probe_samples_host,
            };
            prober.probe_views(world, packed, candidates, grid_w, grid_h)
        }
        ProbeBackendKind::Wgpu => {
            if rig.wgpu_prober.is_none() {
                rig.wgpu_prober = Some(WgpuViewProber::try_headless()?);
            }
            let prober = rig
                .wgpu_prober
                .as_mut()
                .ok_or_else(|| "WGPU ViewProber unavailable".to_string())?;
            let (world, _) = rig
                .csg_host
                .world_and_hints()
                .ok_or_else(|| "No active CSG scene".to_string())?;
            let packed = rig.csg_host.packed();
            prober.probe_views(world, packed, candidates, grid_w, grid_h)
        }
        ProbeBackendKind::Cpu => {
            let (world, _) = rig
                .csg_host
                .world_and_hints()
                .ok_or_else(|| "No active CSG scene".to_string())?;
            let packed = rig.csg_host.packed();
            rig.cpu_prober
                .probe_views(world, packed, candidates, grid_w, grid_h)
        }
    }
}

/// Lookup key matching one `(actor_user_id, target_xyz_bits)` keypoint visibility ray.
type SdgRayKey = (u32, u32, u32, u32);

/// Hardware OptiX 9 batched [`RayOccluder`] populated by a single 1D `__raygen__sdg_keypoints`
/// launch across all in-frame actor joints.
struct OptixBatchedOccluder {
    /// Maps `(actor_user_id, target_x_bits, target_y_bits, target_z_bits)` -> blocking `user_id`
    /// (`u32::MAX` = unobstructed line of sight).
    results: HashMap<SdgRayKey, u32>,
}

impl OptixBatchedOccluder {
    fn from_pairs(keys: &[SdgRayKey], blockers: &[u32]) -> Self {
        let mut results = HashMap::with_capacity(keys.len());
        for (&k, &uid) in keys.iter().zip(blockers.iter()) {
            results.insert(k, uid);
        }
        Self { results }
    }
}

impl RayOccluder for OptixBatchedOccluder {
    fn test_visibility(
        &self,
        _eye: ActorVec3,
        target: ActorVec3,
        _joint_radius: f32,
        actor_user_id: u32,
    ) -> KeypointVis {
        let key = (
            actor_user_id,
            target.x.to_bits(),
            target.y.to_bits(),
            target.z.to_bits(),
        );
        match self.results.get(&key).copied() {
            None | Some(u32::MAX) => KeypointVis::Visible,
            Some(uid) if uid == actor_user_id => KeypointVis::SelfOccluded,
            Some(_) => KeypointVis::OccludedWorld,
        }
    }
}

/// Runs a single batched OptiX 9 `__raygen__sdg_keypoints` launch across all in-frame crowd
/// keypoints against the live scene IAS with zero steady-state GPU reallocations.
fn trace_sdg_occluder_optix(
    rig: &mut OptixRig,
    sdg_cam: &SdgCamera,
) -> Result<OptixBatchedOccluder, String> {
    let ias_handle = rig
        .ias
        .as_ref()
        .map(|a| a.handle())
        .filter(|&h| h != 0)
        .ok_or_else(|| "OptiX IAS unavailable for hardware SDG keypoint occlusion".to_string())?;
    let crowd = rig
        .csg_host
        .active_crowd()
        .ok_or_else(|| "Load a CSG scene before running OptiX SDG keypoint occlusion".to_string())?;

    rig.sdg_queries_host.clear();
    let mut keys: Vec<SdgRayKey> = Vec::with_capacity(crowd.world_joints.len());
    let w = sdg_cam.width as f32;
    let h = sdg_cam.height as f32;
    let eye = sdg_cam.eye;

    for desc in &crowd.descs {
        let topo = &crowd.topologies[desc.archetype_id as usize];
        let b0 = desc.bone_offset as usize;
        for li in 0..topo.joints.len() {
            let jw = crowd.world_joints[b0 + li];
            let p3 = ActorVec3::new(jw[0], jw[1], jw[2]);
            let Some(p2) = sdg_cam.project_point(p3) else {
                continue;
            };
            if p2[0] < 0.0 || p2[0] >= w || p2[1] < 0.0 || p2[1] >= h {
                continue;
            }
            let delta = p3 - eye;
            let dist = delta.len();
            let tmin = 1e-3_f32;
            let tmax = dist - jw[3].max(0.01) - 1e-3;
            if !dist.is_finite() || !tmax.is_finite() || tmax <= tmin {
                continue;
            }
            let inv_dist = 1.0 / dist;
            rig.sdg_queries_host.push(GpuSdgRayQuery {
                origin_tmin: [eye.x, eye.y, eye.z, tmin],
                dir_tmax: [
                    delta.x * inv_dist,
                    delta.y * inv_dist,
                    delta.z * inv_dist,
                    tmax,
                ],
            });
            keys.push((
                desc.user_id,
                p3.x.to_bits(),
                p3.y.to_bits(),
                p3.z.to_bits(),
            ));
        }
    }

    let query_count = rig.sdg_queries_host.len();
    if query_count == 0 {
        return Ok(OptixBatchedOccluder {
            results: HashMap::new(),
        });
    }

    let q_bytes_len = query_count * std::mem::size_of::<GpuSdgRayQuery>();
    if rig
        .sdg_queries_buf
        .as_ref()
        .is_none_or(|b| b.num_bytes() < q_bytes_len)
    {
        let cap_bytes = q_bytes_len
            .next_power_of_two()
            .max(256 * std::mem::size_of::<GpuSdgRayQuery>());
        rig.sdg_queries_buf = Some(
            DeviceBuffer::zeroed(&rig.stream, cap_bytes)
                .map_err(|e| format!("OptiX SDG ray query buffer alloc failed: {e}"))?,
        );
    }
    let r_bytes_len = query_count * std::mem::size_of::<u32>();
    if rig
        .sdg_results_buf
        .as_ref()
        .is_none_or(|b| b.num_bytes() < r_bytes_len)
    {
        let cap_bytes = r_bytes_len
            .next_power_of_two()
            .max(256 * std::mem::size_of::<u32>());
        rig.sdg_results_buf = Some(
            DeviceBuffer::zeroed(&rig.stream, cap_bytes)
                .map_err(|e| format!("OptiX SDG ray result buffer alloc failed: {e}"))?,
        );
    }

    let q_ptr = rig
        .sdg_queries_buf
        .as_ref()
        .ok_or_else(|| "SDG query buffer missing".to_string())?
        .cu_deviceptr();
    let r_ptr = rig
        .sdg_results_buf
        .as_ref()
        .ok_or_else(|| "SDG result buffer missing".to_string())?
        .cu_deviceptr();

    let q_bytes: &[u8] = bytemuck::cast_slice(&rig.sdg_queries_host);
    // SAFETY: `sdg_queries_buf` holds `>= q_bytes.len()` bytes on `rig.stream`, and
    // `sdg_queries_host` stays live through `rig.stream.synchronize()`.
    let h2d = unsafe {
        cu::cuMemcpyHtoDAsync_v2(
            q_ptr,
            q_bytes.as_ptr().cast(),
            q_bytes.len(),
            rig.stream.cu_stream(),
        )
    };
    if h2d != cu::cudaError_enum_CUDA_SUCCESS {
        return Err(format!("OptiX SDG query upload failed ({h2d:?})"));
    }

    let csg_ptrs = rig.csg_host.device_pointers();
    let lp = LaunchParams {
        handle: ias_handle,
        csg_compounds: csg_ptrs.compounds,
        csg_ops: csg_ptrs.ops,
        csg_prims: csg_ptrs.prims,
        csg_aux_planes: csg_ptrs.aux_planes,
        csg_guard_aabbs: csg_ptrs.guard_aabbs,
        csg_repeats: csg_ptrs.repeats,
        csg_materials: csg_ptrs.materials,
        csg_instances: csg_ptrs.instances,
        csg_instance_count: csg_ptrs.instance_count,
        csg_material_count: csg_ptrs.material_count,
        csg_compound_count: csg_ptrs.compound_count,
        csg_flags: csg_flags::ENABLED,
        sdg_ray_queries: q_ptr,
        sdg_ray_results: r_ptr,
        sdg_ray_count: query_count as u32,
        ..LaunchParams::zeroed()
    };
    rig.params_buf
        .copy_from_host(&rig.stream, bytemuck::bytes_of(&lp))
        .map_err(|e| format!("OptiX SDG LaunchParams upload failed: {e}"))?;

    // SAFETY: `params_buf` matches `Params` layout; `sdg_queries_buf` and `sdg_results_buf` are
    // sized for `query_count` entries and remain live through `rig.stream.synchronize()`.
    unsafe {
        rig.pipeline.launch(
            &rig.stream,
            &rig.params_buf,
            &rig.sdg_sbt,
            [query_count as u32, 1, 1],
        )
    }
    .map_err(|e| format!("OptiX __raygen__sdg_keypoints launch failed: {e}"))?;
    rig.stream
        .synchronize()
        .map_err(|e| format!("OptiX SDG occluder sync failed: {e}"))?;

    rig.sdg_results_host.resize(query_count, u32::MAX);
    let r_bytes: &mut [u8] = bytemuck::cast_slice_mut(&mut rig.sdg_results_host[..query_count]);
    // SAFETY: `sdg_results_buf` holds `>= query_count` `u32` elements and GPU execution synced.
    let d2h = unsafe { cu::cuMemcpyDtoH_v2(r_bytes.as_mut_ptr().cast(), r_ptr, r_bytes.len()) };
    if d2h != cu::cudaError_enum_CUDA_SUCCESS {
        return Err(format!("OptiX SDG result readback failed ({d2h:?})"));
    }

    Ok(OptixBatchedOccluder::from_pairs(
        &keys,
        &rig.sdg_results_host[..query_count],
    ))
}

/// Exports exact SDG keypoints & tight BBoxes honoring strict backend semantics:
/// explicit [`SdgOccluderBackend::Optix`] never silently downgrades to CPU on failure, while
/// [`SdgOccluderBackend::Auto`] tries OptiX first and reports which concrete backend ran.
#[expect(
    clippy::too_many_arguments,
    reason = "forwards output path, camera, projection, viewport dims, frame metadata, and backend selector cleanly"
)]
fn export_sdg_json_with_backend(
    rig: &mut OptixRig,
    path: &std::path::Path,
    cam: &CameraFrame,
    projection: CameraProjection,
    width: u32,
    height: u32,
    frame_index: u32,
    time_s: f32,
    backend: SdgOccluderBackend,
) -> Result<(usize, ActiveSdgOccluder), String> {
    sync_cuda_crowd_to_cpu(rig, true)?;
    match backend {
        SdgOccluderBackend::Cpu => {
            let count = rig
                .csg_host
                .export_sdg_json(path, cam, projection, width, height, frame_index, time_s)?;
            Ok((count, ActiveSdgOccluder::Cpu))
        }
        SdgOccluderBackend::Optix => {
            let sdg_cam = CsgGpuHost::build_sdg_camera(cam, projection, width, height);
            let occluder = trace_sdg_occluder_optix(rig, &sdg_cam)?;
            let count = rig.csg_host.export_sdg_json_with_occluder(
                path,
                sdg_cam,
                frame_index,
                time_s,
                &occluder,
            )?;
            Ok((count, ActiveSdgOccluder::Optix))
        }
        SdgOccluderBackend::Auto => {
            let sdg_cam = CsgGpuHost::build_sdg_camera(cam, projection, width, height);
            if let Ok(occluder) = trace_sdg_occluder_optix(rig, &sdg_cam) {
                let count = rig.csg_host.export_sdg_json_with_occluder(
                    path,
                    sdg_cam,
                    frame_index,
                    time_s,
                    &occluder,
                )?;
                Ok((count, ActiveSdgOccluder::Optix))
            } else {
                let count = rig.csg_host.export_sdg_json(
                    path,
                    cam,
                    projection,
                    width,
                    height,
                    frame_index,
                    time_s,
                )?;
                Ok((count, ActiveSdgOccluder::Cpu))
            }
        }
    }
}

/// Reusable host-side staging scratch for multi-channel OpenEXR SDG burst readbacks.
#[derive(Default)]
struct SdgExrReadbackScratch {
    color_rgba: Vec<[f32; 4]>,
    depth_f32: Vec<f32>,
    motion_xy: Vec<[f32; 2]>,
    semantic_u32: Vec<u32>,
    instance_u32: Vec<u32>,
}

/// Active multi-frame OpenEXR + SDG JSON + `manifest.json` dataset burst capture state.
struct SdgBurstState {
    dir: PathBuf,
    total_frames: u32,
    current_frame: u32,
    target_accum: u32,
    occluder: SdgOccluderBackend,
    last_active_occluder: ActiveSdgOccluder,
    stepped_current_frame: bool,
    entries: Vec<SdgBurstFrameEntry>,
}

/// Reads back the 9 SDG AOV channels (`R/G/B/A`, `depth.Z`, `motion.X/Y`, `semantic.id`,
/// `instance.id`) from `rig` and writes one atomic ZIP-compressed OpenEXR frame to `path`.
fn readback_and_write_sdg_exr(
    rig: &OptixRig,
    width: u32,
    height: u32,
    path: &std::path::Path,
    scratch: &mut SdgExrReadbackScratch,
) -> Result<(), String> {
    let n = (width as usize)
        .checked_mul(height as usize)
        .ok_or_else(|| "SDG EXR pixel count overflow".to_string())?;
    let depth_buf = rig
        .depth_f32
        .as_ref()
        .ok_or_else(|| "SDG depth_f32 device buffer not initialized".to_string())?;
    let motion_buf = rig
        .motion_f32
        .as_ref()
        .ok_or_else(|| "SDG motion_f32 device buffer not initialized".to_string())?;
    let sem_buf = rig
        .semantic_u32
        .as_ref()
        .ok_or_else(|| "SDG semantic_u32 device buffer not initialized".to_string())?;
    let inst_buf = rig
        .instance_u32
        .as_ref()
        .ok_or_else(|| "SDG instance_u32 device buffer not initialized".to_string())?;

    scratch.color_rgba.resize(n, [0.0; 4]);
    scratch.depth_f32.resize(n, 0.0);
    scratch.motion_xy.resize(n, [0.0; 2]);
    scratch.semantic_u32.resize(n, 0);
    scratch.instance_u32.resize(n, 0);

    let copy_d2h = |dst: &mut [u8], src_ptr: u64, label: &str| -> Result<(), String> {
        // SAFETY: GPU stream synced before readback; `src_ptr` holds `>= dst.len()` bytes.
        let rc = unsafe { cu::cuMemcpyDtoH_v2(dst.as_mut_ptr().cast(), src_ptr, dst.len()) };
        if rc != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("SDG EXR {label} readback failed ({rc:?})"));
        }
        Ok(())
    };
    copy_d2h(
        bytemuck::cast_slice_mut(&mut scratch.color_rgba[..n]),
        rig.color_f32.cu_deviceptr(),
        "color_f32",
    )?;
    copy_d2h(
        bytemuck::cast_slice_mut(&mut scratch.depth_f32[..n]),
        depth_buf.cu_deviceptr(),
        "depth_f32",
    )?;
    copy_d2h(
        bytemuck::cast_slice_mut(&mut scratch.motion_xy[..n]),
        motion_buf.cu_deviceptr(),
        "motion_f32",
    )?;
    copy_d2h(
        bytemuck::cast_slice_mut(&mut scratch.semantic_u32[..n]),
        sem_buf.cu_deviceptr(),
        "semantic_u32",
    )?;
    copy_d2h(
        bytemuck::cast_slice_mut(&mut scratch.instance_u32[..n]),
        inst_buf.cu_deviceptr(),
        "instance_u32",
    )?;

    let frame = SdgExrFrame {
        width,
        height,
        color_rgba: &scratch.color_rgba[..n],
        depth: &scratch.depth_f32[..n],
        motion: &scratch.motion_xy[..n],
        semantic_id: &scratch.semantic_u32[..n],
        instance_id: &scratch.instance_u32[..n],
    };
    write_sdg_exr_atomic(path, &frame)
}

fn csg_pose_to_camera_frame(pose: csg_scene::CameraPose) -> CameraFrame {
    let fwd = glam::Vec3::new(pose.forward.x, pose.forward.y, pose.forward.z).normalize_or_zero();
    let fwd = if fwd.length_squared() > 1e-8 {
        fwd
    } else {
        glam::Vec3::Z
    };
    let up_ref = glam::Vec3::new(pose.up.x, pose.up.y, pose.up.z).normalize_or_zero();
    let up_ref = if up_ref.length_squared() > 1e-8 {
        up_ref
    } else {
        glam::Vec3::Y
    };
    let right_raw = fwd.cross(up_ref);
    let right = if right_raw.length_squared() > 1e-8 {
        right_raw.normalize()
    } else {
        fwd.cross(glam::Vec3::X).normalize_or_zero()
    };
    let up = right.cross(fwd).normalize();
    let tan_half_fov = (0.5 * pose.vfov_deg.clamp(15.0, 115.0).to_radians()).tan();
    CameraFrame {
        eye: [pose.eye.x, pose.eye.y, pose.eye.z],
        fwd: fwd.to_array(),
        right: right.to_array(),
        up: up.to_array(),
        tan_half_fov,
    }
}

fn camera_frame_to_core_pose(cam: CameraFrame) -> crate::camera_core::CameraPose {
    let fov_y = (2.0 * cam.tan_half_fov.max(0.05).atan()).clamp(0.2, 2.2);
    crate::camera_core::CameraPose::from_eye_forward_up(
        glam::Vec3::from_array(cam.eye),
        glam::Vec3::from_array(cam.fwd),
        glam::Vec3::from_array(cam.up),
        fov_y,
        0.05,
        2000.0,
    )
}

fn core_pose_to_camera_frame(pose: crate::camera_core::CameraPose) -> CameraFrame {
    CameraFrame {
        eye: pose.eye.to_array(),
        fwd: pose.forward().to_array(),
        right: pose.right().to_array(),
        up: pose.up().to_array(),
        tan_half_fov: (0.5 * pose.fov_y).tan(),
    }
}

fn emit_possess_status(bus: &AppBus, info: &PossessInfo) {
    let mount_lbl = info
        .mounts
        .get(info.mount_idx)
        .map_or("Mount", |m| m.label.as_str());
    bus.emit_render_event(RenderEvent::StatusMessage(format!(
        "Possessing {} — {}",
        info.assembly_name, mount_lbl
    )));
}

fn resize_rig(rig: &mut OptixRig, width: u32, height: u32, temporal: bool) {
    rig.stream.synchronize().expect("pre-resize sync");
    let n = (width * height) as usize;
    rig.color_f32 = DeviceBuffer::zeroed(&rig.stream, n).expect("color_f32 resize");
    rig.albedo_f32 = DeviceBuffer::zeroed(&rig.stream, n).expect("albedo_f32 resize");
    rig.normal_f32 = DeviceBuffer::zeroed(&rig.stream, n).expect("normal_f32 resize");
    // Drop lazy auxiliary buffers and denoiser; they reallocate on-demand only when enabled.
    rig.diffuse_albedo_f32 = None;
    rig.spec_albedo_f32 = None;
    rig.denoised_f32 = None;
    rig.prev_denoised_f32 = None;
    rig.motion_f32 = None;
    rig.depth_f32 = None;
    rig.semantic_u32 = None;
    rig.instance_u32 = None;
    rig.denoiser = None;
    rig.denoiser_is_temporal = temporal;
    rig.temporal_has_prev = false;
    rig.stream.synchronize().expect("resize sync");
}

fn halton(mut index: u32, base: u32) -> f32 {
    let mut f = 1.0f32;
    let mut r = 0.0f32;
    let base_f = base as f32;
    while index > 0 {
        f /= base_f;
        r += f * (index % base) as f32;
        index /= base;
    }
    r
}

struct QueuedShutterSubframe {
    settings: Box<DemoSettings>,
    camera: CameraFrame,
    sim_dt: f32,
    sub_idx: u32,
    sub_count: u32,
    reset_seq: u64,
    possess_free_yaw: f32,
    possess_free_pitch: f32,
    possess_auto_level: bool,
    possess_zoom_offset: f32,
}

pub fn spawn_render_thread(
    cuda: Arc<CudaContext>,
    initial_buf_ptrs: [u64; 2],
    initial_depth_ptrs: [u64; 2],
    initial_motion_ptrs: [u64; 2],
    bus: AppBus,
) -> JoinHandle<()> {
    thread::Builder::new()
        .name("optix-render".to_string())
        .spawn(move || {
            crate::nvtx::name_current_thread(c"optix-render");
            cuda.bind_to_thread()
                .expect("cuda bind_to_thread on render thread");
            let init_snap = bus.state.load();
            let mut last_state_rev = bus.state.revision();
            let mut last_reset_seq = init_snap.reset_seq;
            let mut interacting_until = init_snap.interacting_until;
            let mut settings = init_snap.settings;
            let mut camera = init_snap.camera;
            let mut prev_camera = camera;
            let mut possess_free_yaw = init_snap.possess_free_yaw;
            let mut possess_free_pitch = init_snap.possess_free_pitch;
            let mut possess_auto_level = init_snap.possess_auto_level;
            let mut possess_zoom_offset = init_snap.possess_zoom_offset;
            let mut possess_damper = crate::camera_rail::CameraDamper::default();
            possess_damper.snap(camera_frame_to_core_pose(camera));
            let mut possess_blend_rem: f32 = 0.0;
            let mut last_rendered_camera = camera;
            let mut was_possessing = false;
            bus.emit_render_event(RenderEvent::PhaseChanged(EnginePhase::CompilingOptix));
            let mut rig = init_optix(&cuda, &settings);
            bus.emit_render_event(RenderEvent::PhaseChanged(EnginePhase::Rendering));
            if let Some(meta) = rig.csg_host.take_loaded_metadata() {
                bus.emit_render_event(RenderEvent::SceneMetaUpdated(meta));
            }
            let mut cur_w = DEFAULT_W;
            let mut cur_h = DEFAULT_H;
            let mut cur_gen: u64 = 0;
            let mut buf_ptrs = initial_buf_ptrs;
            let mut depth_ptrs = initial_depth_ptrs;
            let mut motion_ptrs = initial_motion_ptrs;
            let mut hdr_ptrs: Option<[u64; 2]> = None;
            let mut rr_ptrs: Option<DlssRrBufferPtrs> = None;
            let mut free_bufs = VecDeque::from([0usize, 1usize]);
            let mut accum_count: u32 = 0;
            let mut frame_index: u32 = 0;
            let mut spin_angle: f32 = 0.0;
            let mut sat_angle: f32 = 0.0;
            let mut traffic_time: f32 = 0.0;
            let mut fast_preview_reduced = false;
            let (mut last_ias_state, _) =
                compute_ias_classic_state(&settings, spin_angle, sat_angle);
            let mut last_tick = Instant::now();
            let mut pending_jpeg: Option<(PathBuf, u8)> = None;
            let mut active_burst: Option<SdgBurstState> = None;
            let mut burst_exr_scratch = SdgExrReadbackScratch::default();
            let mut last_crowd_step_ms: f32 = 0.0;
            let mut last_crowd_backend = ActiveCrowdBackend::None;
            let mut last_cuda_crowd_err: Option<String> = None;
            let mut last_crowd_lod_sig = settings.crowd_lod_params().signature_key();
            let mut rec_active = false;
            let mut rec_fixed_fps: u32 = 0;
            let mut rec_shutter_subframes: u32 = 1;
            let mut shutter_queue: VecDeque<QueuedShutterSubframe> = VecDeque::with_capacity(8);
            let mut cmd_scratch: Vec<appbus::BoxedEvent> = Vec::with_capacity(32);

            loop {
                let mut should_exit = false;
                bus.poll_render_controls(&mut cmd_scratch);
                for boxed in &cmd_scratch {
                    let Some(cmd) = downcast_event::<RenderControl>(boxed) else {
                        continue;
                    };
                    match cmd {
                        RenderControl::Resize {
                            width,
                            height,
                            buf_ptrs: new_ptrs,
                            depth_ptrs: new_depth_ptrs,
                            motion_ptrs: new_motion_ptrs,
                            hdr_ptrs: new_hdr_ptrs,
                            rr_ptrs: new_rr_ptrs,
                            gen_id,
                        } => {
                            if *width != cur_w || *height != cur_h {
                                resize_rig(&mut rig, *width, *height, settings.temporal_denoise);
                                cur_w = *width;
                                cur_h = *height;
                                if active_burst.take().is_some() {
                                    bus.emit_render_event(RenderEvent::StatusMessage(
                                        "SDG Burst cancelled (viewport resized)".to_string(),
                                    ));
                                }
                            }
                            buf_ptrs = *new_ptrs;
                            depth_ptrs = *new_depth_ptrs;
                            motion_ptrs = *new_motion_ptrs;
                            hdr_ptrs = *new_hdr_ptrs;
                            rr_ptrs = *new_rr_ptrs;
                            cur_gen = *gen_id;
                            free_bufs.clear();
                            free_bufs.push_back(0);
                            free_bufs.push_back(1);
                            shutter_queue.clear();
                            accum_count = 0;
                            fast_preview_reduced = false;
                        }
                        RenderControl::ReturnBuffer { buf_idx, gen_id } => {
                            if *gen_id == cur_gen && !free_bufs.contains(buf_idx) {
                                free_bufs.push_back(*buf_idx);
                            }
                        }
                        RenderControl::SaveJpeg { path, quality } => {
                            pending_jpeg = Some((path.clone(), *quality));
                            accum_count = clamp_accum_for_jpeg(accum_count, settings.max_accum);
                        }
                        RenderControl::ExportSdgJson { path, occluder } => {
                            match export_sdg_json_with_backend(
                                &mut rig,
                                path,
                                &last_rendered_camera,
                                settings.camera_projection,
                                cur_w,
                                cur_h,
                                frame_index,
                                traffic_time,
                                *occluder,
                            ) {
                                Ok((actor_count, active_occ)) => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                        "Saved SDG JSON ({actor_count} actors, {}) -> {}",
                                        active_occ.label(),
                                        path.display()
                                    )));
                                }
                                Err(e) => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                        "SDG JSON export failed: {e}"
                                    )));
                                }
                            }
                        }
                        RenderControl::StartSdgBurst {
                            dir,
                            frames,
                            occluder,
                        } => {
                            if settings.csg_mode == 0 {
                                bus.emit_render_event(RenderEvent::StatusMessage(
                                    "Load a CSG scene before exporting an SDG Burst".to_string(),
                                ));
                            } else {
                                let total_frames = (*frames).clamp(1, 240);
                                let target_accum = settings.max_accum.clamp(4, 16);
                                active_burst = Some(SdgBurstState {
                                    dir: dir.clone(),
                                    total_frames,
                                    current_frame: 0,
                                    target_accum,
                                    occluder: *occluder,
                                    last_active_occluder: ActiveSdgOccluder::Cpu,
                                    stepped_current_frame: false,
                                    entries: Vec::with_capacity(total_frames as usize),
                                });
                                accum_count = 0;
                                fast_preview_reduced = false;
                                bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                    "Starting SDG Burst ({total_frames}f @ {target_accum}x{} spp) -> {}",
                                    settings.spp.max(1),
                                    dir.display()
                                )));
                            }
                        }
                        RenderControl::PokeCrowd {
                            origin,
                            forward,
                            impulse,
                            all,
                        } => {
                            if let Err(e) = sync_cuda_crowd_to_cpu(&mut rig, true) {
                                bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                    "CUDA crowd sync before poke failed: {e}"
                                )));
                            }
                            match rig.csg_host.poke_crowd(*origin, *forward, *impulse, *all) {
                                Ok(msg) => {
                                    if let Some(Ok(cc)) = rig.cuda_crowd.as_mut() {
                                        cc.mark_intents_dirty();
                                    }
                                    accum_count = 0;
                                    bus.emit_render_event(RenderEvent::StatusMessage(msg));
                                }
                                Err(e) => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(e));
                                }
                            }
                        }
                        RenderControl::SpawnCrowdStress {
                            target_count,
                            origin,
                        } => {
                            if let Err(e) = sync_cuda_crowd_to_cpu(&mut rig, true) {
                                bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                    "CUDA crowd sync before stress spawn failed: {e}"
                                )));
                            }
                            match rig.csg_host.spawn_crowd_stress(
                                &rig._optix_ctx,
                                &rig.stream,
                                &mut rig._scratch,
                                *target_count,
                                *origin,
                            ) {
                                Ok(added) => {
                                    if let Some(Ok(cc)) = rig.cuda_crowd.as_mut() {
                                        cc.mark_scene_dirty();
                                    }
                                    accum_count = 0;
                                    bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                        "Crowd stress grid: +{added} actors (total {})",
                                        rig.csg_host.crowd_actor_count()
                                    )));
                                }
                                Err(e) => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(e));
                                }
                            }
                        }
                        RenderControl::SetRecording {
                            active,
                            fixed_fps,
                            shutter_subframes,
                        } => {
                            rec_active = *active;
                            rec_fixed_fps = *fixed_fps;
                            rec_shutter_subframes = if *active && *fixed_fps > 0 {
                                (*shutter_subframes).max(1)
                            } else {
                                1
                            };
                            if !*active || rec_shutter_subframes <= 1 {
                                shutter_queue.clear();
                            }
                        }
                        RenderControl::RenderShutterSubframe {
                            settings: s,
                            camera: c,
                            sim_dt,
                            sub_idx,
                            sub_count,
                            reset_seq,
                            possess_free_yaw: p_yaw,
                            possess_free_pitch: p_pitch,
                            possess_auto_level: p_auto,
                            possess_zoom_offset: p_zoom,
                        } => {
                            shutter_queue.push_back(QueuedShutterSubframe {
                                settings: s.clone(),
                                camera: *c,
                                sim_dt: *sim_dt,
                                sub_idx: *sub_idx,
                                sub_count: (*sub_count).max(1),
                                reset_seq: *reset_seq,
                                possess_free_yaw: *p_yaw,
                                possess_free_pitch: *p_pitch,
                                possess_auto_level: *p_auto,
                                possess_zoom_offset: *p_zoom,
                            });
                        }
                        RenderControl::GenerateRandomRail { seed } => {
                            match rig.csg_host.generate_random_rail(*seed) {
                                Ok(rail) => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                        "Generated rail: {}",
                                        rail.label
                                    )));
                                    bus.emit_render_event(RenderEvent::RailGenerated(rail));
                                }
                                Err(e) => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(e));
                                }
                            }
                        }
                        RenderControl::PickPossess { origin, dir } => {
                            if let Err(e) = sync_cuda_crowd_to_cpu(&mut rig, true) {
                                bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                    "CUDA crowd sync before pick failed: {e}"
                                )));
                            }
                            match rig.csg_host.pick_possess(*origin, *dir) {
                                Ok(info) => {
                                    possess_damper.snap(camera_frame_to_core_pose(last_rendered_camera));
                                    possess_blend_rem = 0.9;
                                    accum_count = 0;
                                    emit_possess_status(&bus, &info);
                                    bus.emit_render_event(RenderEvent::PossessChanged(Some(info)));
                                }
                                Err(e) => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(e));
                                }
                            }
                        }
                        RenderControl::PossessNextVehicle => {
                            if let Err(e) = sync_cuda_crowd_to_cpu(&mut rig, true) {
                                bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                    "CUDA crowd sync before possess failed: {e}"
                                )));
                            }
                            match rig.csg_host.possess_next_vehicle() {
                                Ok(info) => {
                                    possess_damper.snap(camera_frame_to_core_pose(last_rendered_camera));
                                    possess_blend_rem = 0.9;
                                    accum_count = 0;
                                    emit_possess_status(&bus, &info);
                                    bus.emit_render_event(RenderEvent::PossessChanged(Some(info)));
                                }
                                Err(e) => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(e));
                                }
                            }
                        }
                        RenderControl::SetPossessMount { mount_idx } => {
                            if let Some(info) = rig.csg_host.set_possess_mount(*mount_idx) {
                                possess_damper.snap(camera_frame_to_core_pose(last_rendered_camera));
                                possess_blend_rem = 0.6;
                                accum_count = 0;
                                emit_possess_status(&bus, &info);
                                bus.emit_render_event(RenderEvent::PossessChanged(Some(info)));
                            }
                        }
                        RenderControl::EjectPossess => {
                            if rig.csg_host.eject_possess() {
                                possess_blend_rem = 0.0;
                                accum_count = 0;
                                bus.emit_render_event(RenderEvent::PossessChanged(None));
                                bus.emit_render_event(RenderEvent::StatusMessage(
                                    "Ejected to free camera".to_string(),
                                ));
                            }
                        }
                        RenderControl::RunSmartView { seed, backend } => {
                            let t0 = Instant::now();
                            let candidates_opt = rig
                                .csg_host
                                .world_and_hints()
                                .map(|(_, hints)| SmartViewSelector::generate_candidates(hints, *seed, 64));
                            match candidates_opt {
                                None => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(
                                        "Load a CSG scene before running Smart View".to_string(),
                                    ));
                                }
                                Some(candidates) if candidates.is_empty() => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(
                                        "Smart View generated zero candidates".to_string(),
                                    ));
                                }
                                Some(candidates) => {
                                    match score_views_on_rig(&mut rig, *backend, &candidates, 16, 12) {
                                        Ok(scores) => {
                                            if let Some((best_idx, best_score)) = scores
                                                .iter()
                                                .enumerate()
                                                .max_by(|(_, a), (_, b)| {
                                                    a.composite.total_cmp(&b.composite)
                                                })
                                            {
                                                let winner = candidates[best_idx];
                                                let elapsed_ms =
                                                    t0.elapsed().as_secs_f32() * 1000.0;
                                                if rig.csg_host.eject_possess() {
                                                    bus.emit_render_event(
                                                        RenderEvent::PossessChanged(None),
                                                    );
                                                }
                                                let res = SmartViewResult {
                                                    eye: [winner.eye.x, winner.eye.y, winner.eye.z],
                                                    forward: [
                                                        winner.forward.x,
                                                        winner.forward.y,
                                                        winner.forward.z,
                                                    ],
                                                    up: [winner.up.x, winner.up.y, winner.up.z],
                                                    vfov_deg: winner.vfov_deg,
                                                    score: *best_score,
                                                    elapsed_ms,
                                                    candidates_evaluated: candidates.len(),
                                                };
                                                bus.emit_render_event(RenderEvent::StatusMessage(
                                                    format!(
                                                        "Smart View ({}): {:.2} score ({} views in {:.1} ms)",
                                                        best_score.backend.label(),
                                                        best_score.composite,
                                                        candidates.len(),
                                                        elapsed_ms
                                                    ),
                                                ));
                                                bus.emit_render_event(RenderEvent::SmartViewReady(
                                                    res,
                                                ));
                                                accum_count = 0;
                                            }
                                        }
                                        Err(e) => {
                                            bus.emit_render_event(RenderEvent::StatusMessage(
                                                format!("Smart View failed: {e}"),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        RenderControl::RunSmartRail { seed, backend } => {
                            let t0 = Instant::now();
                            match rig.csg_host.build_smart_rail_candidates(*seed) {
                                Err(e) => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(e));
                                }
                                Ok(mut rails) if !rails.is_empty() => {
                                    const SAMPLES_PER_RAIL: usize = 8;
                                    let mut cands =
                                        Vec::with_capacity(rails.len() * SAMPLES_PER_RAIL);
                                    for rail in &rails {
                                        let compiled = rail.compile();
                                        for s_idx in 0..SAMPLES_PER_RAIL {
                                            let u = (s_idx as f32 + 0.5) / SAMPLES_PER_RAIL as f32;
                                            let sample = compiled.eval_at_fraction(u);
                                            cands.push(ViewCandidate {
                                                eye: sample.eye,
                                                forward: sample.forward,
                                                up: sample.up,
                                                vfov_deg: sample.vfov_deg,
                                                target_sem: None,
                                            });
                                        }
                                    }
                                    match score_views_on_rig(&mut rig, *backend, &cands, 16, 12) {
                                        Ok(scores) => {
                                            let mut best_rail_idx = 0usize;
                                            let mut best_avg = -1.0f32;
                                            let mut used_backend = ProbeBackendKind::Cpu;
                                            for (ri, chunk) in
                                                scores.chunks(SAMPLES_PER_RAIL).enumerate()
                                            {
                                                if ri >= rails.len() || chunk.is_empty() {
                                                    break;
                                                }
                                                used_backend = chunk[0].backend;
                                                let avg =
                                                    chunk.iter().map(|s| s.composite).sum::<f32>()
                                                        / chunk.len() as f32;
                                                if avg > best_avg {
                                                    best_avg = avg;
                                                    best_rail_idx = ri;
                                                }
                                            }
                                            if rig.csg_host.eject_possess() {
                                                bus.emit_render_event(
                                                    RenderEvent::PossessChanged(None),
                                                );
                                            }
                                            let mut winner = rails.swap_remove(best_rail_idx);
                                            winner.label = format!(
                                                "{} ({:.2})",
                                                winner.label,
                                                best_avg.max(0.0)
                                            );
                                            let elapsed_ms = t0.elapsed().as_secs_f32() * 1000.0;
                                            bus.emit_render_event(RenderEvent::StatusMessage(
                                                format!(
                                                    "Smart Rail ({}): avg {:.2} across {} rails in {:.1} ms",
                                                    used_backend.label(),
                                                    best_avg.max(0.0),
                                                    scores.len() / SAMPLES_PER_RAIL,
                                                    elapsed_ms
                                                ),
                                            ));
                                            bus.emit_render_event(RenderEvent::RailGenerated(
                                                winner,
                                            ));
                                            accum_count = 0;
                                        }
                                        Err(e) => {
                                            bus.emit_render_event(RenderEvent::StatusMessage(
                                                format!("Smart Rail scoring failed: {e}"),
                                            ));
                                        }
                                    }
                                }
                                Ok(_) => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(
                                        "Smart Rail generated zero verified candidates".to_string(),
                                    ));
                                }
                            }
                        }
                        RenderControl::ReloadShader => {
                            let disk_path = PathBuf::from("crates/nv-demo/shaders/raytracer.cu");
                            let res = std::fs::read_to_string(&disk_path)
                                .map_err(|e| format!("Read {}: {e}", disk_path.display()))
                                .and_then(|src| try_compile_ptx(&src, "raytracer.cu", true))
                                .and_then(|ptx| {
                                    build_optix_pipeline(&rig._optix_ctx, &rig.stream, &ptx)
                                });
                            match res {
                                Ok((pipeline, sbt, probe_sbt, sdg_sbt)) => {
                                    if let Err(sync_err) = rig.stream.synchronize() {
                                        bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                            "Hot-reload stream sync failed: {sync_err}"
                                        )));
                                    }
                                    rig.pipeline = pipeline;
                                    rig.sbt = sbt;
                                    rig.probe_sbt = probe_sbt;
                                    rig.sdg_sbt = sdg_sbt;
                                    accum_count = 0;
                                    fast_preview_reduced = false;
                                    bus.emit_render_event(RenderEvent::StatusMessage(
                                        "Shader hot-reloaded".to_string(),
                                    ));
                                }
                                Err(e) => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                        "Hot-reload failed: {e}"
                                    )));
                                }
                            }
                        }
                        RenderControl::Shutdown => {
                            should_exit = true;
                            break;
                        }
                    }
                }
                if should_exit {
                    break;
                }

                // Consume latest coalesced UI state snapshot (zero stale-frame queue buildup).
                let cur_state_rev = bus.state.revision();
                if cur_state_rev != last_state_rev {
                    let snap = bus.state.load();
                    last_state_rev = cur_state_rev;
                    let s = snap.settings;
                    let c = snap.camera;
                    let reset_accum = snap.reset_seq != last_reset_seq;
                    last_reset_seq = snap.reset_seq;

                    let mut tmp = s;
                    tmp.exposure = settings.exposure;
                    tmp.split_on = settings.split_on;
                    tmp.split_x = settings.split_x;
                    tmp.right_is_aov = settings.right_is_aov;
                    tmp.bloom_on = settings.bloom_on;
                    tmp.bloom_threshold = settings.bloom_threshold;
                    tmp.bloom_intensity = settings.bloom_intensity;
                    tmp.bloom_blades = settings.bloom_blades;
                    tmp.dlss_sharpness = settings.dlss_sharpness;
                    tmp.hdr_display = settings.hdr_display;
                    tmp.record_video_mode = settings.record_video_mode;
                    tmp.record_fixed_fps = settings.record_fixed_fps;
                    tmp.capture_source = settings.capture_source;
                    tmp.cam_smooth_tau = settings.cam_smooth_tau;
                    tmp.time_scale = settings.time_scale;
                    tmp.record_shutter_subframes = settings.record_shutter_subframes;
                    tmp.probe_backend = settings.probe_backend;
                    tmp.crowd_backend = settings.crowd_backend;
                    tmp.sdg_occluder_backend = settings.sdg_occluder_backend;
                    tmp.poke_impulse = settings.poke_impulse;
                    tmp.sdg_burst_frames = settings.sdg_burst_frames;
                    if s.aov_mode < 3 && settings.aov_mode < 3 {
                        tmp.aov_mode = settings.aov_mode;
                    }
                    let only_non_accum = tmp == settings && c == camera;
                    let possess_look_changed = rig.csg_host.is_possessing()
                        && (snap.possess_free_yaw.to_bits() != possess_free_yaw.to_bits()
                            || snap.possess_free_pitch.to_bits() != possess_free_pitch.to_bits()
                            || snap.possess_zoom_offset.to_bits() != possess_zoom_offset.to_bits()
                            || snap.possess_auto_level != possess_auto_level);
                    if reset_accum
                        || possess_look_changed
                        || (!only_non_accum && (s != settings || c != camera))
                    {
                        accum_count = 0;
                        interacting_until = snap.interacting_until;
                    }
                    settings = s;
                    camera = c;
                    possess_free_yaw = snap.possess_free_yaw;
                    possess_free_pitch = snap.possess_free_pitch;
                    possess_auto_level = snap.possess_auto_level;
                    possess_zoom_offset = snap.possess_zoom_offset;
                }

                // Strict 1-to-1 deterministic lockstep for ANY Smooth 60 FPS recording
                // (`rec_shutter_subframes >= 1`): render thread advances traffic, crowd, and
                // encodes exactly one frame per `RenderShutterSubframe` batch from UI.
                let in_shutter_mode = rec_active && rec_fixed_fps > 0 && rec_shutter_subframes >= 1;
                let burst_active = active_burst.is_some();
                let now_preview = Instant::now();
                let fast_preview = !rec_active
                    && !in_shutter_mode
                    && !burst_active
                    && interacting_until.is_some_and(|until| now_preview < until);
                if !fast_preview && fast_preview_reduced {
                    // Interaction settle window expired — restart clean full-quality accumulation.
                    accum_count = 0;
                    fast_preview_reduced = false;
                }

                let shutter_waiting = in_shutter_mode
                    && (shutter_queue.is_empty()
                        || (shutter_queue
                            .front()
                            .is_some_and(|q| q.sub_idx + 1 >= q.sub_count)
                            && free_bufs.is_empty()));

                let show_classic_now = settings.csg_mode == 0 || settings.csg_show_fractal;
                let csg_traffic_moving = !settings.csg_freeze_pose
                    && rig.csg_host.has_traffic()
                    && settings.csg_traffic_on
                    && settings.csg_traffic_speed > 0.001;
                let csg_crowd_moving = !settings.csg_freeze_pose
                    && rig.csg_host.has_crowd()
                    && settings.csg_traffic_speed > 0.001;
                let csg_dynamic_moving = csg_traffic_moving || csg_crowd_moving;
                let converged = !in_shutter_mode
                    && !burst_active
                    && !(show_classic_now && settings.auto_rotate)
                    && !(show_classic_now
                        && settings.sat_on
                        && settings.sat_orbit
                        && settings.sat_count > 0)
                    && !csg_dynamic_moving
                    && possess_blend_rem <= 0.0
                    && !rec_active
                    && pending_jpeg.is_none()
                    && !rig.csg_host.has_pending_metadata()
                    && settings.max_accum > 0
                    && accum_count >= settings.max_accum;

                if shutter_waiting || converged || (!in_shutter_mode && free_bufs.is_empty()) {
                    if let Some(until) = interacting_until.filter(|_| fast_preview && !shutter_waiting)
                    {
                        let rem = until
                            .saturating_duration_since(now_preview)
                            .max(Duration::from_millis(1));
                        bus.render_wake.wait_timeout(rem);
                    } else {
                        bus.render_wake.wait();
                    }
                    continue;
                }

                let (shutter_sub_idx, publish_frame, dt) = if in_shutter_mode {
                    let Some(q) = shutter_queue.pop_front() else {
                        continue;
                    };
                    settings = *q.settings;
                    camera = q.camera;
                    possess_free_yaw = q.possess_free_yaw;
                    possess_free_pitch = q.possess_free_pitch;
                    possess_auto_level = q.possess_auto_level;
                    possess_zoom_offset = q.possess_zoom_offset;
                    let reset_now = q.reset_seq != last_reset_seq;
                    last_reset_seq = q.reset_seq;
                    if q.sub_idx == 0 || reset_now {
                        accum_count = 0;
                    }
                    last_tick = Instant::now();
                    (
                        Some(q.sub_idx),
                        q.sub_idx + 1 >= q.sub_count,
                        q.sim_dt.max(0.0),
                    )
                } else {
                    let now = Instant::now();
                    let base_dt = if rec_active && rec_fixed_fps > 0 {
                        1.0 / rec_fixed_fps as f32
                    } else {
                        now.duration_since(last_tick).as_secs_f32().min(0.1)
                    };
                    last_tick = now;
                    let scale = settings.time_scale.clamp(0.05, 2.0);
                    (None, true, base_dt * scale)
                };

                let buf_idx = if publish_frame {
                    let Some(idx) = free_bufs.pop_front() else {
                        continue;
                    };
                    idx
                } else {
                    0
                };

                let preserve_shutter_accum = shutter_sub_idx.is_some_and(|idx| idx > 0);
                let show_classic = settings.csg_mode == 0 || settings.csg_show_fractal;
                let csg_traffic_moving = !settings.csg_freeze_pose
                    && rig.csg_host.has_traffic()
                    && settings.csg_traffic_on
                    && settings.csg_traffic_speed > 0.001;
                let csg_crowd_moving = !settings.csg_freeze_pose
                    && rig.csg_host.has_crowd()
                    && settings.csg_traffic_speed > 0.001;
                if show_classic && settings.auto_rotate {
                    spin_angle += dt * settings.spin_speed;
                    if !preserve_shutter_accum {
                        accum_count = 0;
                    }
                }
                if show_classic && settings.sat_on && settings.sat_orbit && settings.sat_count > 0 {
                    sat_angle += dt * settings.spin_speed;
                    if !preserve_shutter_accum {
                        accum_count = 0;
                    }
                }
                let (step_traffic_now, crowd_step_dt, freeze_now, burst_step_frame) =
                    if let Some(ref mut burst) = active_burst {
                        if !burst.stepped_current_frame {
                            let burst_dt = (1.0 / 30.0) * settings.csg_traffic_speed.max(0.1);
                            traffic_time += burst_dt;
                            accum_count = 0;
                            burst.stepped_current_frame = true;
                            (
                                rig.csg_host.has_traffic() && settings.csg_traffic_on,
                                if rig.csg_host.has_crowd() { burst_dt } else { 0.0 },
                                false,
                                true,
                            )
                        } else {
                            (false, 0.0, false, false)
                        }
                    } else if csg_traffic_moving || csg_crowd_moving {
                        let dyn_dt = dt * settings.csg_traffic_speed;
                        traffic_time += dyn_dt;
                        (
                            csg_traffic_moving,
                            if csg_crowd_moving { dyn_dt } else { 0.0 },
                            false,
                            false,
                        )
                    } else {
                        (false, 0.0, settings.csg_freeze_pose, false)
                    };

                if freeze_now
                    && let Err(e) = sync_cuda_crowd_to_cpu(&mut rig, true)
                {
                    bus.emit_render_event(RenderEvent::StatusMessage(format!(
                        "CUDA crowd freeze sync failed: {e}"
                    )));
                }
                if rig.csg_host.sync_freeze_pose(freeze_now)
                    && let Some(Ok(cc)) = rig.cuda_crowd.as_mut()
                {
                    cc.mark_intents_dirty();
                }

                let crowd_lod = settings.crowd_lod_params();
                let cur_crowd_lod_sig = crowd_lod.signature_key();
                let crowd_lod_changed = cur_crowd_lod_sig != last_crowd_lod_sig;
                last_crowd_lod_sig = cur_crowd_lod_sig;

                let mut run_cuda_crowd_after_sync = false;
                if rig.csg_host.has_crowd() && crowd_step_dt > 0.0 {
                    match settings.crowd_backend {
                        CrowdBackend::Cpu => {
                            if let Err(e) = sync_cuda_crowd_to_cpu(&mut rig, true) {
                                bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                    "CUDA crowd CPU-switch sync failed: {e}"
                                )));
                            }
                            let t_crowd = Instant::now();
                            let moved = rig.csg_host.step_dynamic(
                                crowd_step_dt,
                                traffic_time,
                                last_rendered_camera.eye,
                                crowd_lod,
                                false,
                                step_traffic_now,
                            );
                            last_crowd_step_ms = t_crowd.elapsed().as_secs_f32() * 1000.0;
                            last_crowd_backend = ActiveCrowdBackend::Cpu;
                            last_cuda_crowd_err = None;
                            if moved && !preserve_shutter_accum && !burst_step_frame {
                                accum_count = 0;
                            }
                        }
                        CrowdBackend::Cuda | CrowdBackend::Auto => {
                            let moved_traffic = rig.csg_host.step_dynamic(
                                0.0,
                                traffic_time,
                                last_rendered_camera.eye,
                                crowd_lod,
                                false,
                                step_traffic_now,
                            );
                            if moved_traffic && !preserve_shutter_accum && !burst_step_frame {
                                accum_count = 0;
                            }
                            match ensure_cuda_crowd(&mut rig, &cuda) {
                                Ok(_) => {
                                    run_cuda_crowd_after_sync = true;
                                    last_cuda_crowd_err = None;
                                }
                                Err(err) => {
                                    if settings.crowd_backend == CrowdBackend::Cuda {
                                        // Strict backend semantics: explicit CUDA never silently downgrades.
                                        if last_cuda_crowd_err.as_deref() != Some(err.as_str()) {
                                            bus.emit_render_event(RenderEvent::StatusMessage(
                                                format!("CUDA crowd error: {err}"),
                                            ));
                                            last_cuda_crowd_err = Some(err);
                                        }
                                        last_crowd_step_ms = 0.0;
                                        last_crowd_backend = ActiveCrowdBackend::None;
                                    } else {
                                        if let Err(e) = sync_cuda_crowd_to_cpu(&mut rig, true) {
                                            bus.emit_render_event(RenderEvent::StatusMessage(
                                                format!("CUDA crowd fallback sync failed: {e}"),
                                            ));
                                        }
                                        let t_crowd = Instant::now();
                                        let moved = rig.csg_host.step_dynamic(
                                            crowd_step_dt,
                                            traffic_time,
                                            last_rendered_camera.eye,
                                            crowd_lod,
                                            false,
                                            false,
                                        );
                                        last_crowd_step_ms =
                                            t_crowd.elapsed().as_secs_f32() * 1000.0;
                                        last_crowd_backend = ActiveCrowdBackend::Cpu;
                                        if moved && !preserve_shutter_accum && !burst_step_frame {
                                            accum_count = 0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    let moved = rig.csg_host.step_dynamic(
                        0.0,
                        traffic_time,
                        last_rendered_camera.eye,
                        crowd_lod,
                        freeze_now,
                        step_traffic_now,
                    );
                    if moved && !preserve_shutter_accum && !burst_step_frame {
                        accum_count = 0;
                    }
                    if !rig.csg_host.has_crowd() {
                        last_crowd_step_ms = 0.0;
                        last_crowd_backend = ActiveCrowdBackend::None;
                    } else if crowd_lod_changed {
                        match settings.crowd_backend {
                            CrowdBackend::Cuda => {
                                if freeze_now {
                                    match ensure_cuda_crowd(&mut rig, &cuda) {
                                        Ok(_) => {
                                            run_cuda_crowd_after_sync = true;
                                            last_cuda_crowd_err = None;
                                        }
                                        Err(err) => {
                                            if last_cuda_crowd_err.as_deref() != Some(err.as_str())
                                            {
                                                bus.emit_render_event(RenderEvent::StatusMessage(
                                                    format!("CUDA crowd error: {err}"),
                                                ));
                                                last_cuda_crowd_err = Some(err);
                                            }
                                            last_crowd_step_ms = 0.0;
                                            last_crowd_backend = ActiveCrowdBackend::None;
                                        }
                                    }
                                }
                            }
                            CrowdBackend::Auto => {
                                if freeze_now && ensure_cuda_crowd(&mut rig, &cuda).is_ok() {
                                    run_cuda_crowd_after_sync = true;
                                    last_cuda_crowd_err = None;
                                } else {
                                    if let Err(e) = sync_cuda_crowd_to_cpu(&mut rig, true) {
                                        bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                            "CUDA crowd LOD sync failed: {e}"
                                        )));
                                    }
                                    if rig.csg_host.refresh_crowd_lod_cpu(
                                        traffic_time,
                                        last_rendered_camera.eye,
                                        crowd_lod,
                                    ) && let Some(Ok(cc)) = rig.cuda_crowd.as_mut()
                                    {
                                        cc.mark_scene_dirty();
                                    }
                                }
                            }
                            CrowdBackend::Cpu => {
                                if let Err(e) = sync_cuda_crowd_to_cpu(&mut rig, true) {
                                    bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                        "CUDA crowd LOD sync failed: {e}"
                                    )));
                                }
                                if rig.csg_host.refresh_crowd_lod_cpu(
                                    traffic_time,
                                    last_rendered_camera.eye,
                                    crowd_lod,
                                ) && let Some(Ok(cc)) = rig.cuda_crowd.as_mut()
                                {
                                    cc.mark_scene_dirty();
                                }
                            }
                        }
                    }
                }

                // Resolve live possessed camera mount AFTER dynamic traffic advances so the
                // camera stays mathematically locked to moving vehicles with zero 1-frame lag.
                let mut possess_cam_out: Option<CameraFrame> = None;
                let mut active_camera = if let Some(csg_pose) = rig.csg_host.resolve_active_possess(
                    possess_free_yaw,
                    possess_free_pitch,
                    possess_auto_level,
                ) {
                    let mut target_frame = csg_pose_to_camera_frame(csg_pose);
                    if possess_zoom_offset.abs() > 1e-4 {
                        target_frame.eye[0] -= target_frame.fwd[0] * possess_zoom_offset;
                        target_frame.eye[1] -= target_frame.fwd[1] * possess_zoom_offset;
                        target_frame.eye[2] -= target_frame.fwd[2] * possess_zoom_offset;
                    }
                    let final_frame = if possess_blend_rem > 0.0 {
                        let frac = (possess_blend_rem / 0.9).clamp(0.0, 1.0);
                        possess_blend_rem = (possess_blend_rem - dt.max(1.0 / 120.0)).max(0.0);
                        let tau = 0.24 * frac;
                        let target_core = camera_frame_to_core_pose(target_frame);
                        let blended = possess_damper.step(target_core, dt.max(1.0 / 120.0), tau);
                        if !preserve_shutter_accum {
                            accum_count = 0;
                        }
                        core_pose_to_camera_frame(blended)
                    } else {
                        possess_damper.snap(camera_frame_to_core_pose(target_frame));
                        target_frame
                    };
                    possess_cam_out = Some(final_frame);
                    final_frame
                } else {
                    possess_blend_rem = 0.0;
                    camera
                };
                last_rendered_camera = active_camera;

                let prof_slot = rig
                    .gpu_profiler
                    .as_mut()
                    .map(|p| p.begin_frame(&rig.stream));

                let csg_key = CsgSceneKey {
                    scene_idx: settings.csg_mode,
                    variant_idx: settings.csg_asset_idx,
                    seed: settings.csg_seed,
                    road_mode: settings.csg_road_mode.min(3),
                };
                let building_scene = csg_key != rig.csg_host.scene_key();
                if building_scene {
                    if let Some(Ok(cc)) = rig.cuda_crowd.as_mut() {
                        cc.mark_scene_dirty();
                    }
                    bus.emit_render_event(RenderEvent::PhaseChanged(EnginePhase::BuildingCsgScene));
                }
                let vfov_rad = 2.0 * active_camera.tan_half_fov.max(0.05).atan();
                let csg_cam_params = CsgCameraParams {
                    eye: active_camera.eye,
                    fwd: active_camera.fwd,
                    up: active_camera.up,
                    viewport_w: cur_w,
                    viewport_h: cur_h,
                    vfov_rad,
                    lod_config: settings.csg_lod_config(),
                    omnidirectional: settings.camera_projection.is_omnidirectional(),
                };
                let freeze_for_sync = settings.csg_freeze_pose || active_burst.is_some();
                let need_crowd_sync_before_csg = !building_scene
                    && (rig.csg_host.will_camera_repack(csg_key, csg_cam_params)
                        || (!run_cuda_crowd_after_sync
                            && rig.csg_host.will_upload_instances(
                                csg_key,
                                csg_cam_params,
                                traffic_time,
                                freeze_for_sync,
                            )));
                if need_crowd_sync_before_csg
                    && let Err(e) = sync_cuda_crowd_to_cpu(&mut rig, true)
                {
                    bus.emit_render_event(RenderEvent::StatusMessage(format!(
                        "CUDA crowd sync before CSG upload failed: {e}"
                    )));
                }
                let csg_sync = match rig.csg_host.sync(
                    &rig._optix_ctx,
                    &rig.stream,
                    &mut rig._scratch,
                    csg_key,
                    settings.csg_wetness,
                    settings.csg_time_of_day,
                    csg_cam_params,
                    traffic_time,
                    freeze_for_sync,
                    run_cuda_crowd_after_sync,
                ) {
                    Ok(sync_res) => sync_res,
                    Err(e) => {
                        bus.emit_render_event(RenderEvent::StatusMessage(format!(
                            "CSG sync error: {e}"
                        )));
                        CsgSyncResult::Unchanged
                    }
                };
                if (building_scene || matches!(csg_sync, CsgSyncResult::FullRepack))
                    && let Some(Ok(cc)) = rig.cuda_crowd.as_mut()
                {
                    cc.mark_topology_dirty();
                }
                let is_possessing_now = rig.csg_host.is_possessing();
                if was_possessing && !is_possessing_now {
                    bus.emit_render_event(RenderEvent::PossessChanged(None));
                }
                was_possessing = is_possessing_now;
                if let Some(meta) = rig.csg_host.take_loaded_metadata() {
                    bus.emit_render_event(RenderEvent::SceneMetaUpdated(meta));
                }

                if show_classic
                    && !settings.glass_analytic
                    && (settings.sat_u_segs, settings.sat_v_segs) != rig.glass_mesh_segs
                {
                    rig.glass_mesh_gas = build_sphere_mesh_gas(
                        &rig._optix_ctx,
                        &rig.stream,
                        &mut rig._scratch,
                        settings.sat_u_segs,
                        settings.sat_v_segs,
                    );
                    rig.glass_mesh_segs = (settings.sat_u_segs, settings.sat_v_segs);
                }
                let (ias_state, fractal_transform) =
                    compute_ias_classic_state(&settings, spin_angle, sat_angle);
                let classic_changed = ias_state != last_ias_state;
                match (classic_changed, csg_sync) {
                    (false, CsgSyncResult::Unchanged) => {}
                    (false, CsgSyncResult::TrafficOnly { dynamic_count })
                        if rig.ias.is_some()
                            && rig.ias_classic_count + dynamic_count
                                <= rig.d_optix_instances.len() =>
                    {
                        // Zero-churn fast path: stream ONLY `dynamic_instances[0..D]` (or only
                        // non-crowd vehicles when CUDA crowd scatters directly in VRAM) via pinned
                        // staging + `OPTIX_BUILD_OPERATION_UPDATE`.
                        rig.csg_host
                            .fill_dynamic_optix_instances(dynamic_count, &mut rig.ias_dyn_scratch);
                        if !rig.ias_dyn_scratch.is_empty() {
                            match rig.d_optix_instances.update_prefix(
                                &rig.stream,
                                rig.ias_classic_count,
                                &rig.ias_dyn_scratch,
                            ) {
                                Ok(()) => {
                                    if !run_cuda_crowd_after_sync {
                                        // SAFETY: all launches on `rig.stream` are strictly stream-ordered.
                                        let _ = unsafe {
                                            Accel::instances_into(
                                                &rig._optix_ctx,
                                                &rig.stream,
                                                &mut rig._scratch,
                                                &rig.d_optix_instances,
                                                ias_build_opts(),
                                                false,
                                                &mut rig.ias,
                                            )
                                        }
                                        .expect("IAS traffic update");
                                    }
                                }
                                Err(e) => {
                                    bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                        "IAS dynamic prefix update failed: {e}"
                                    )));
                                }
                            }
                        }
                    }
                    _ => {
                        if building_scene {
                            bus.emit_render_event(RenderEvent::PhaseChanged(
                                EnginePhase::UpdatingIas,
                            ));
                        }
                        rig.ias_classic_count = build_ias_into(
                            &rig._optix_ctx,
                            &rig.stream,
                            &mut rig._scratch,
                            &mut rig.d_optix_instances,
                            &mut rig.ias,
                            SceneGas {
                                floor: &rig.floor_gas,
                                fractal: &rig.fractal_gas,
                                glass_analytic: &rig.glass_analytic_gas,
                                glass_mesh: &rig.glass_mesh_gas,
                                neon: &rig.neon_gas,
                                omm: &rig.omm_gas,
                                omm_anyhit: &rig.omm_anyhit_gas,
                            },
                            &rig.csg_host,
                            &mut rig.ias_instances_scratch,
                            SceneState {
                                sat_angle,
                                sat_count: settings.sat_count,
                                sat_seed: settings.sat_seed,
                                sat_size_range: [settings.sat_size_min, settings.sat_size_max],
                                sat_orbit_range: [settings.sat_orbit_min, settings.sat_orbit_max],
                                sat_height_range: [
                                    settings.sat_height_min,
                                    settings.sat_height_max,
                                ],
                                sat_speed_range: [settings.sat_speed_min, settings.sat_speed_max],
                                sat_bob_amp_range: [
                                    settings.sat_bob_amp_min,
                                    settings.sat_bob_amp_max,
                                ],
                                sat_bob_freq_range: [
                                    settings.sat_bob_freq_min,
                                    settings.sat_bob_freq_max,
                                ],
                                sat_hue_spread: settings.sat_hue_spread,
                                sat_on: settings.sat_on,
                                glass_analytic: settings.glass_analytic,
                                neon_on: settings.neon_on,
                                omm_mode: settings.omm_mode,
                                show_classic,
                                fractal_transform,
                            },
                        );
                        last_ias_state = ias_state;
                    }
                }

                // GPU-direct CUDA articulated crowd step -> scatter into VRAM (`csg_instances`,
                // `csg_prev_xforms`, `d_optix_instances`) -> single OptiX IAS refit/rebuild.
                if run_cuda_crowd_after_sync && rig.csg_host.has_crowd() && rig.ias.is_some() {
                    let csg_ptrs_scatter = rig.csg_host.device_pointers();
                    let possessing_crowd = rig.csg_host.is_possessing_crowd();
                    let optix_ptr = if rig.d_optix_instances.len()
                        >= rig.ias_classic_count + csg_ptrs_scatter.instance_count as usize
                    {
                        rig.d_optix_instances.device_ptr()
                    } else {
                        0
                    };
                    let scatter = GpuScatterTarget {
                        csg_instances_ptr: csg_ptrs_scatter.instances,
                        csg_prev_xforms_ptr: csg_ptrs_scatter.prev_xforms,
                        max_csg_instances: csg_ptrs_scatter.instance_count,
                        optix_instances_ptr: optix_ptr,
                        optix_slot_offset: rig.ias_classic_count as u32,
                        max_optix_instances: rig.d_optix_instances.len() as u32,
                    };

                    let need_topo = rig
                        .cuda_crowd
                        .as_ref()
                        .and_then(|r| r.as_ref().ok())
                        .is_some_and(CudaCrowdBackend::topology_dirty);
                    if need_topo {
                        rig.csg_host
                            .fill_crowd_bone_slot_map(&mut rig.crowd_slot_map_host);
                        rig.csg_host
                            .fill_compound_gas_handles(&mut rig.crowd_gas_handles_host);
                        if let Some(Ok(cc)) = rig.cuda_crowd.as_mut()
                            && let Err(e) = cc.upload_scatter_topology(
                                &rig.stream,
                                &rig.crowd_slot_map_host,
                                &rig.crowd_gas_handles_host,
                                rig.csg_host.compound_sbt_offsets(),
                            )
                        {
                            bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                "CUDA crowd topology upload failed: {e}"
                            )));
                        }
                    }

                    let cuda_step_res = match (
                        rig.cuda_crowd.as_mut(),
                        rig.csg_host.active_scene_mut(),
                    ) {
                        (Some(Ok(cc)), Some(scene)) => cc.step_gpu_direct(
                            &rig.stream,
                            scene,
                            crowd_step_dt,
                            traffic_time,
                            active_camera.eye,
                            crowd_lod,
                            Some(scatter),
                            possessing_crowd,
                        ),
                        _ => Err("CUDA crowd backend unavailable".to_string()),
                    };
                    match cuda_step_res {
                        Ok(report) => {
                            last_crowd_step_ms = report.step_ms;
                            last_crowd_backend = ActiveCrowdBackend::Cuda;
                            last_cuda_crowd_err = None;
                            if !preserve_shutter_accum && !burst_step_frame {
                                accum_count = 0;
                            }
                            if optix_ptr != 0 {
                                // SAFETY: all launches on `rig.stream` are strictly stream-ordered
                                // and `scatter_crowd_to_csg_and_optix` finished before this call.
                                let _ = unsafe {
                                    Accel::instances_into(
                                        &rig._optix_ctx,
                                        &rig.stream,
                                        &mut rig._scratch,
                                        &rig.d_optix_instances,
                                        ias_build_opts(),
                                        report.needs_full_ias_rebuild,
                                        &mut rig.ias,
                                    )
                                }
                                .expect("IAS CUDA crowd refit");
                            }
                            if possessing_crowd
                                && let Some(csg_pose) = rig.csg_host.resolve_active_possess(
                                    possess_free_yaw,
                                    possess_free_pitch,
                                    possess_auto_level,
                                )
                            {
                                let mut updated = csg_pose_to_camera_frame(csg_pose);
                                if possess_zoom_offset.abs() > 1e-4 {
                                    updated.eye[0] -= updated.fwd[0] * possess_zoom_offset;
                                    updated.eye[1] -= updated.fwd[1] * possess_zoom_offset;
                                    updated.eye[2] -= updated.fwd[2] * possess_zoom_offset;
                                }
                                if possess_blend_rem <= 0.0 {
                                    possess_damper.snap(camera_frame_to_core_pose(updated));
                                    active_camera = updated;
                                    last_rendered_camera = updated;
                                    possess_cam_out = Some(updated);
                                }
                            }
                        }
                        Err(err) => {
                            if last_cuda_crowd_err.as_deref() != Some(err.as_str()) {
                                bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                    "CUDA crowd step failed: {err}"
                                )));
                                last_cuda_crowd_err = Some(err);
                            }
                            last_crowd_step_ms = 0.0;
                            last_crowd_backend = ActiveCrowdBackend::None;
                        }
                    }
                }
                if building_scene {
                    bus.emit_render_event(RenderEvent::PhaseChanged(EnginePhase::Rendering));
                }

                let rr_export = settings.dlss_rr_on || rr_ptrs.is_some();
                let dlss_any = settings.dlss_on || rr_export;
                let will_run_denoiser = settings.denoiser_on && !dlss_any && settings.aov_mode == 0;
                let will_run_bloom = settings.bloom_on && !rr_export && settings.aov_mode == 0;
                let n_px = (cur_w * cur_h) as usize;

                // Lazily allocate / recreate OptiX AI denoiser only when enabled.
                if will_run_denoiser
                    && (rig.denoiser.is_none()
                        || settings.temporal_denoise != rig.denoiser_is_temporal)
                {
                    let (d, is_t) = create_denoiser(
                        &rig._optix_ctx,
                        &rig.stream,
                        cur_w,
                        cur_h,
                        settings.temporal_denoise,
                    );
                    rig.denoiser = Some(d);
                    rig.denoiser_is_temporal = is_t;
                    rig.temporal_has_prev = false;
                }

                let need_motion_buf =
                    dlss_any || (will_run_denoiser && rig.denoiser_is_temporal) || active_burst.is_some();
                let need_depth_buf = dlss_any || active_burst.is_some();
                if need_motion_buf && rig.motion_f32.is_none() {
                    rig.motion_f32 = DeviceBuffer::zeroed(&rig.stream, n_px).ok();
                }
                if need_depth_buf && rig.depth_f32.is_none() {
                    rig.depth_f32 = DeviceBuffer::zeroed(&rig.stream, n_px).ok();
                }
                if rr_export {
                    if rig.diffuse_albedo_f32.is_none() {
                        rig.diffuse_albedo_f32 = DeviceBuffer::zeroed(&rig.stream, n_px).ok();
                    }
                    if rig.spec_albedo_f32.is_none() {
                        rig.spec_albedo_f32 = DeviceBuffer::zeroed(&rig.stream, n_px).ok();
                    }
                }
                if (will_run_denoiser || will_run_bloom) && rig.denoised_f32.is_none() {
                    rig.denoised_f32 = DeviceBuffer::zeroed(&rig.stream, n_px).ok();
                }
                if will_run_denoiser && rig.denoiser_is_temporal && rig.prev_denoised_f32.is_none() {
                    rig.prev_denoised_f32 = DeviceBuffer::zeroed(&rig.stream, n_px).ok();
                }

                let (fractal_p0, fractal_p1, fractal_p2, fractal_p3) = match settings.fractal_kind {
                    1 => (
                        [
                            settings.box_scale,
                            settings.box_bailout,
                            settings.box_fold,
                            settings.box_min_r,
                        ],
                        [settings.box_fixed_r, 0.0, settings.box_iter as f32, 0.0],
                        [0.0; 4],
                        [
                            settings.box_fold_mult,
                            settings.box_iter_rot.to_radians(),
                            0.0,
                            0.0,
                        ],
                    ),
                    2 => (
                        [settings.kifs_scale, settings.kifs_bailout, 0.0, 0.0],
                        [
                            settings.kifs_rot1.to_radians(),
                            settings.kifs_rot2.to_radians(),
                            settings.kifs_iter as f32,
                            0.0,
                        ],
                        [
                            settings.kifs_offset[0],
                            settings.kifs_offset[1],
                            settings.kifs_offset[2],
                            0.0,
                        ],
                        [settings.kifs_thickness, settings.kifs_z_bias, 0.0, 0.0],
                    ),
                    _ => (
                        [
                            settings.bulb_power,
                            settings.bulb_bailout,
                            if settings.bulb_julia { 1.0 } else { 0.0 },
                            0.0,
                        ],
                        [0.0, 0.0, settings.bulb_iter as f32, 0.0],
                        [
                            settings.bulb_c[0],
                            settings.bulb_c[1],
                            settings.bulb_c[2],
                            0.0,
                        ],
                        [
                            settings.bulb_theta_mult,
                            settings.bulb_phi_mult,
                            settings.bulb_theta_phase.to_radians(),
                            settings.bulb_phi_phase.to_radians(),
                        ],
                    ),
                };

                let az = settings.sun_azimuth.to_radians();
                let el = settings.sun_elevation.to_radians();
                let sun_x = el.cos() * az.sin();
                let sun_y = el.sin();
                let sun_z = el.cos() * az.cos();

                let jitter = if dlss_any && settings.aov_mode == 0 {
                    if !preserve_shutter_accum {
                        accum_count = 0;
                    }
                    let period = if rr_export { 64 } else { 16 };
                    let jitter_idx = frame_index.saturating_sub(shutter_sub_idx.unwrap_or(0));
                    [
                        halton((jitter_idx % period) + 1, 2) - 0.5,
                        halton((jitter_idx % period) + 1, 3) - 0.5,
                    ]
                } else {
                    [0.0, 0.0]
                };

                let csg_ptrs = rig.csg_host.device_pointers();
                let mut csg_flag_bits = 0u32;
                if rig.csg_host.is_active() {
                    csg_flag_bits |= csg_flags::ENABLED;
                    if settings.csg_time_of_day == 2 {
                        csg_flag_bits |= csg_flags::NIGHT;
                    } else if settings.csg_time_of_day == 1 {
                        csg_flag_bits |= csg_flags::SUNSET;
                    }
                    if settings.csg_wetness > 0.05 {
                        csg_flag_bits |= csg_flags::WET_ASPHALT;
                    }
                }

                let (launch_spp, launch_bounces) =
                    fast_preview_budget(settings.spp, settings.bounces, fast_preview);
                let frame_is_fast_preview = fast_preview
                    && (launch_spp < settings.spp.max(1)
                        || launch_bounces < settings.bounces.clamp(1, 16));
                if frame_is_fast_preview {
                    fast_preview_reduced = true;
                }

                let (out_semantic_u32_ptr, out_instance_u32_ptr) = if active_burst.is_some() {
                    let n_px = (cur_w * cur_h) as usize;
                    if rig.semantic_u32.is_none() {
                        rig.semantic_u32 = DeviceBuffer::zeroed(&rig.stream, n_px).ok();
                    }
                    if rig.instance_u32.is_none() {
                        rig.instance_u32 = DeviceBuffer::zeroed(&rig.stream, n_px).ok();
                    }
                    (
                        rig.semantic_u32.as_ref().map_or(0, DeviceBuffer::cu_deviceptr),
                        rig.instance_u32.as_ref().map_or(0, DeviceBuffer::cu_deviceptr),
                    )
                } else {
                    (0, 0)
                };

                let lp = LaunchParams {
                    out_color_f32: rig.color_f32.cu_deviceptr(),
                    out_albedo_f32: rig.albedo_f32.cu_deviceptr(),
                    out_normal_f32: rig.normal_f32.cu_deviceptr(),
                    handle: rig.ias.as_ref().map_or(0, |a| a.handle()),
                    width: cur_w,
                    height: cur_h,
                    frame_index,
                    spp: launch_spp,
                    cam_pos: [
                        active_camera.eye[0],
                        active_camera.eye[1],
                        active_camera.eye[2],
                        settings.aperture,
                    ],
                    cam_forward: [
                        active_camera.fwd[0],
                        active_camera.fwd[1],
                        active_camera.fwd[2],
                        0.0,
                    ],
                    cam_right: [
                        active_camera.right[0],
                        active_camera.right[1],
                        active_camera.right[2],
                        settings.focus_dist,
                    ],
                    cam_up: [
                        active_camera.up[0],
                        active_camera.up[1],
                        active_camera.up[2],
                        active_camera.tan_half_fov,
                    ],
                    sun_dir: [sun_x, sun_y, sun_z, settings.roughness],
                    fractal_p0,
                    fractal_p1,
                    fractal_p2,
                    fractal_p3,
                    // Fractal TRS is applied in the OptiX TLAS instance transform; keep ABI slot identity.
                    fractal_xform: [1.0, 0.0, 0.0, 0.0],
                    fractal_r0: [
                        settings.detail_eps,
                        settings.fudge,
                        settings.norm_eps,
                        settings.march_steps as f32,
                    ],
                    fractal_r1: [
                        settings.metallic,
                        settings.trap_scale,
                        settings.palette_shift,
                        settings.sun_intensity,
                    ],
                    time: spin_angle,
                    enable_shadows: u32::from(settings.shadows_on),
                    fractal_kind: settings.fractal_kind,
                    bounces: launch_bounces,
                    floor_params: [
                        settings.floor_roughness,
                        settings.floor_scratch,
                        settings.floor_scale,
                        settings.floor_metallic,
                    ],
                    sky_params: [
                        settings.sky_intensity,
                        1.0,
                        settings.horizon_haze,
                        settings.sky_mode as f32,
                    ],
                    accum_count,
                    enable_ser: u32::from(settings.ser_on),
                    sky_mode_from: settings.sky_mode_from,
                    sky_blend: settings.sky_blend,
                    out_motion_f32: rig.motion_f32.as_ref().map_or(0, DeviceBuffer::cu_deviceptr),
                    out_depth_f32: rig.depth_f32.as_ref().map_or(0, DeviceBuffer::cu_deviceptr),
                    prev_cam_pos: [
                        prev_camera.eye[0],
                        prev_camera.eye[1],
                        prev_camera.eye[2],
                        0.0,
                    ],
                    prev_cam_forward: [
                        prev_camera.fwd[0],
                        prev_camera.fwd[1],
                        prev_camera.fwd[2],
                        0.0,
                    ],
                    prev_cam_right: [
                        prev_camera.right[0],
                        prev_camera.right[1],
                        prev_camera.right[2],
                        0.0,
                    ],
                    prev_cam_up: [
                        prev_camera.up[0],
                        prev_camera.up[1],
                        prev_camera.up[2],
                        prev_camera.tan_half_fov,
                    ],
                    sat_pos: [0.0; 4],
                    glass_params: [
                        settings.glass_ior,
                        settings.glass_absorb,
                        settings.glass_roughness,
                        if settings.sat_on { 1.0 } else { 0.0 },
                    ],
                    aov_mode: settings.aov_mode,
                    jitter_x: jitter[0],
                    jitter_y: jitter[1],
                    _pad1: 0,
                    env_params: settings.env.pack(settings.sky_mode),
                    out_diffuse_albedo_f32: if rr_export {
                        rig.diffuse_albedo_f32
                            .as_ref()
                            .map_or(0, DeviceBuffer::cu_deviceptr)
                    } else {
                        0
                    },
                    out_spec_albedo_f32: if rr_export {
                        rig.spec_albedo_f32
                            .as_ref()
                            .map_or(0, DeviceBuffer::cu_deviceptr)
                    } else {
                        0
                    },
                    csg_compounds: csg_ptrs.compounds,
                    csg_ops: csg_ptrs.ops,
                    csg_prims: csg_ptrs.prims,
                    csg_aux_planes: csg_ptrs.aux_planes,
                    csg_guard_aabbs: csg_ptrs.guard_aabbs,
                    csg_repeats: csg_ptrs.repeats,
                    csg_materials: csg_ptrs.materials,
                    csg_instances: csg_ptrs.instances,
                    csg_instance_count: csg_ptrs.instance_count,
                    csg_material_count: csg_ptrs.material_count,
                    csg_compound_count: csg_ptrs.compound_count,
                    csg_flags: csg_flag_bits,
                    csg_wetness: settings.csg_wetness.clamp(0.0, 1.0),
                    csg_traffic_time: traffic_time,
                    camera_projection: settings.camera_projection as u32,
                    probe_grid_wh: 0,
                    probe_candidates: 0,
                    probe_samples: 0,
                    csg_prev_xforms: csg_ptrs.prev_xforms,
                    out_semantic_u32: out_semantic_u32_ptr,
                    out_instance_u32: out_instance_u32_ptr,
                    sdg_ray_queries: 0,
                    sdg_ray_results: 0,
                    sdg_ray_count: 0,
                    _pad2: 0,
                };
                if publish_frame && active_burst.is_none() {
                    prev_camera = active_camera;
                }
                rig.params_buf
                    .copy_from_host(&rig.stream, bytemuck::bytes_of(&lp))
                    .expect("params upload");
                if let (Some(prof), Some(slot)) = (rig.gpu_profiler.as_ref(), prof_slot) {
                    prof.mark_accel_end(slot, &rig.stream);
                }

                // Single uninterrupted GPU stream burst: [IAS/Traffic] -> [RT Cores] ->
                // [Tensor Denoiser] -> [cuFFT Bloom] -> [Tonemap/Pack], with ONE end-of-frame fence.
                let t_burst = Instant::now();
                {
                    let _nvtx = crate::nvtx::NvtxRange::new(c"OptiX RT", crate::nvtx::COLOR_RT);
                    // SAFETY: `params_buf` matches `LaunchParams` layout; all output buffers outlive the end-of-frame fence.
                    unsafe {
                        rig.pipeline.launch(
                            &rig.stream,
                            &rig.params_buf,
                            &rig.sbt,
                            [cur_w, cur_h, 1],
                        )
                    }
                    .expect("optix launch");
                }
                if let (Some(prof), Some(slot)) = (rig.gpu_profiler.as_ref(), prof_slot) {
                    prof.mark_rt_end(slot, &rig.stream);
                }

                if !publish_frame {
                    if let (Some(prof), Some(slot)) = (rig.gpu_profiler.as_mut(), prof_slot) {
                        prof.mark_denoise_end(slot, &rig.stream);
                        prof.mark_bloom_end(slot, &rig.stream);
                        prof.mark_frame_end(slot, &rig.stream);
                    }
                    // Synchronize before looping to the next 180-degree shutter subframe so the
                    // next subframe's pinned H2D staging upload cannot race with this DMA.
                    rig.stream.synchronize().expect("shutter subframe sync");
                    accum_count = accum_count.saturating_add(1);
                    frame_index = frame_index.wrapping_add(1);
                    continue;
                }

                let mut final_f32_ptr = if will_run_denoiser
                    && let (Some(denoiser), Some(denoised_buf)) =
                        (rig.denoiser.as_mut(), rig.denoised_f32.as_ref())
                {
                    let _nvtx =
                        crate::nvtx::NvtxRange::new(c"OptiX Denoise", crate::nvtx::COLOR_DENOISE);
                    let flow_img = if rig.denoiser_is_temporal
                        && let Some(ref motion_buf) = rig.motion_f32
                    {
                        Some(Image::new(motion_buf, cur_w, cur_h).expect("flow"))
                    } else {
                        None
                    };
                    let guides = Guides {
                        albedo: Some(Image::new(&rig.albedo_f32, cur_w, cur_h).expect("albedo")),
                        normal: Some(Image::new(&rig.normal_f32, cur_w, cur_h).expect("normal")),
                        flow: flow_img,
                        ..Default::default()
                    };
                    let prev_img = if rig.denoiser_is_temporal {
                        let prev_buf = if rig.temporal_has_prev
                            && let Some(ref prev_d) = rig.prev_denoised_f32
                        {
                            prev_d
                        } else {
                            &rig.color_f32
                        };
                        Some(Image::new(prev_buf, cur_w, cur_h).expect("prev"))
                    } else {
                        None
                    };
                    let layer = Layer {
                        input: Image::new(&rig.color_f32, cur_w, cur_h).expect("in"),
                        output: Image::new(denoised_buf, cur_w, cur_h).expect("out"),
                        previous: prev_img,
                        aov: AovType::Beauty,
                    };
                    // SAFETY: all denoiser input/output/guide images outlive the end-of-frame fence.
                    unsafe {
                        denoiser.invoke(
                            &rig.stream,
                            &guides,
                            &[layer],
                            DenoiserParams::default(),
                        )
                    }
                    .expect("denoiser invoke");
                    if rig.denoiser_is_temporal
                        && let Some(ref prev_d) = rig.prev_denoised_f32
                    {
                        // SAFETY: `denoised_buf` and `prev_d` have identical byte size on `rig.stream`.
                        let cp_res = unsafe {
                            cu::cuMemcpyDtoDAsync_v2(
                                prev_d.cu_deviceptr(),
                                denoised_buf.cu_deviceptr(),
                                denoised_buf.num_bytes(),
                                rig.stream.cu_stream(),
                            )
                        };
                        if cp_res == cu::cudaError_enum_CUDA_SUCCESS {
                            rig.temporal_has_prev = true;
                        }
                    }
                    denoised_buf.cu_deviceptr()
                } else {
                    rig.color_f32.cu_deviceptr()
                };
                if let (Some(prof), Some(slot)) = (rig.gpu_profiler.as_ref(), prof_slot) {
                    prof.mark_denoise_end(slot, &rig.stream);
                }

                if will_run_bloom {
                    let _nvtx =
                        crate::nvtx::NvtxRange::new(c"cuFFT Bloom", crate::nvtx::COLOR_BLOOM);
                    if let Some(ref mut bloom) = rig.bloom {
                        let bloom_ready = if will_run_denoiser {
                            true
                        } else if let Some(ref denoised_buf) = rig.denoised_f32 {
                            // SAFETY: `color_f32` and `denoised_buf` have identical byte size on `rig.stream`.
                            let cp_res = unsafe {
                                cu::cuMemcpyDtoDAsync_v2(
                                    denoised_buf.cu_deviceptr(),
                                    rig.color_f32.cu_deviceptr(),
                                    rig.color_f32.num_bytes(),
                                    rig.stream.cu_stream(),
                                )
                            };
                            if cp_res == cu::cudaError_enum_CUDA_SUCCESS {
                                final_f32_ptr = denoised_buf.cu_deviceptr();
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        if bloom_ready
                            && let Err(err) = bloom.apply(
                                &rig.stream,
                                final_f32_ptr,
                                cur_w,
                                cur_h,
                                settings.bloom_threshold,
                                settings.bloom_intensity,
                                settings.bloom_blades,
                            )
                        {
                            bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                "cuFFT bloom error: {err}"
                            )));
                        }
                    }
                }
                if let (Some(prof), Some(slot)) = (rig.gpu_profiler.as_ref(), prof_slot) {
                    prof.mark_bloom_end(slot, &rig.stream);
                }

                {
                    let _nvtx =
                        crate::nvtx::NvtxRange::new(c"Tonemap", crate::nvtx::COLOR_TONEMAP);
                    let mut w = cur_w;
                    let mut h = cur_h;
                    let gx = cur_w.div_ceil(16);
                    let gy = cur_h.div_ceil(16);

                    // Skip redundant CUDA RGBA8/HDR tonemap when DLSS-RR owns the display output.
                    let skip_tonemap = should_skip_cuda_tonemap(
                        settings.dlss_rr_on,
                        rr_ptrs.is_some(),
                        settings.aov_mode,
                        settings.split_on,
                        pending_jpeg.is_some(),
                    );
                    if !skip_tonemap {
                        let mut src_ptr = match settings.aov_mode {
                            1 => rig.albedo_f32.cu_deviceptr(),
                            2 => rig.normal_f32.cu_deviceptr(),
                            _ => final_f32_ptr,
                        };
                        let mut dst_ptr = buf_ptrs[buf_idx];
                        let mut exposure = settings.exposure;
                        let mut aov_mode = settings.aov_mode;
                        let mut right_ptr: u64 = if settings.split_on {
                            match settings.right_is_aov {
                                1 => rig.albedo_f32.cu_deviceptr(),
                                2 => rig.normal_f32.cu_deviceptr(),
                                _ => rig.color_f32.cu_deviceptr(),
                            }
                        } else {
                            0
                        };
                        let mut split_x: f32 = if settings.split_on {
                            settings.split_x.clamp(0.0, 1.0)
                        } else {
                            -1.0
                        };
                        let mut right_is_aov: i32 = settings.right_is_aov as i32;
                        let mut kernel_args: [*mut c_void; 9] = [
                            (&raw mut src_ptr).cast(),
                            (&raw mut dst_ptr).cast(),
                            (&raw mut w).cast(),
                            (&raw mut h).cast(),
                            (&raw mut exposure).cast(),
                            (&raw mut aov_mode).cast(),
                            (&raw mut right_ptr).cast(),
                            (&raw mut split_x).cast(),
                            (&raw mut right_is_aov).cast(),
                        ];
                        // SAFETY: `rig.tonemap.func` takes `(float4*, uchar4*, u32, u32, f32, u32, float4*, f32, i32)`
                        // and all pointers/buffers remain valid through the single end-of-frame fence.
                        let res = unsafe {
                            cu::cuLaunchKernel(
                                rig.tonemap.func,
                                gx,
                                gy,
                                1,
                                16,
                                16,
                                1,
                                0,
                                rig.stream.cu_stream(),
                                kernel_args.as_mut_ptr(),
                                std::ptr::null_mut(),
                            )
                        };
                        assert_eq!(
                            res,
                            cu::cudaError_enum_CUDA_SUCCESS,
                            "tonemap cuLaunchKernel failed"
                        );
                        if let Some(hdr_dsts) = hdr_ptrs {
                            let mut hdr_dst_ptr = hdr_dsts[buf_idx];
                            let mut hdr_args: [*mut c_void; 9] = [
                                (&raw mut src_ptr).cast(),
                                (&raw mut hdr_dst_ptr).cast(),
                                (&raw mut w).cast(),
                                (&raw mut h).cast(),
                                (&raw mut exposure).cast(),
                                (&raw mut aov_mode).cast(),
                                (&raw mut right_ptr).cast(),
                                (&raw mut split_x).cast(),
                                (&raw mut right_is_aov).cast(),
                            ];
                            // SAFETY: `tonemap_f32_to_scrgb_f16` writes `cur_w * cur_h * 8` bytes of
                            // scRGB FP16 into `hdr_dsts[buf_idx]`, valid through the end-of-frame fence.
                            let hdr_res = unsafe {
                                cu::cuLaunchKernel(
                                    rig.tonemap.scrgb_func,
                                    gx,
                                    gy,
                                    1,
                                    16,
                                    16,
                                    1,
                                    0,
                                    rig.stream.cu_stream(),
                                    hdr_args.as_mut_ptr(),
                                    std::ptr::null_mut(),
                                )
                            };
                            assert_eq!(
                                hdr_res,
                                cu::cudaError_enum_CUDA_SUCCESS,
                                "tonemap_f32_to_scrgb_f16 cuLaunchKernel failed"
                            );
                        }
                    }
                    if dlss_any
                        && let (Some(depth_buf), Some(motion_buf)) =
                            (&rig.depth_f32, &rig.motion_f32)
                    {
                        // SAFETY: `depth_buf` and `depth_ptrs[buf_idx]` have `cur_w * cur_h * 4` bytes;
                        // `motion_buf` and `motion_ptrs[buf_idx]` have `cur_w * cur_h * 8` bytes.
                        let (depth_res, motion_res) = unsafe {
                            let d = cu::cuMemcpyDtoDAsync_v2(
                                depth_ptrs[buf_idx],
                                depth_buf.cu_deviceptr(),
                                depth_buf.num_bytes(),
                                rig.stream.cu_stream(),
                            );
                            let m = cu::cuMemcpyDtoDAsync_v2(
                                motion_ptrs[buf_idx],
                                motion_buf.cu_deviceptr(),
                                motion_buf.num_bytes(),
                                rig.stream.cu_stream(),
                            );
                            (d, m)
                        };
                        if depth_res != cu::cudaError_enum_CUDA_SUCCESS
                            || motion_res != cu::cudaError_enum_CUDA_SUCCESS
                        {
                            bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                "DLSS guide copy failed (depth={depth_res:?}, motion={motion_res:?})"
                            )));
                        }
                    }
                    if rr_export
                        && let Some(rr) = rr_ptrs
                        && let (Some(diff_buf), Some(spec_buf)) =
                            (&rig.diffuse_albedo_f32, &rig.spec_albedo_f32)
                    {
                        let mut c_src = rig.color_f32.cu_deviceptr();
                        let mut d_src = diff_buf.cu_deviceptr();
                        let mut s_src = spec_buf.cu_deviceptr();
                        let mut n_src = rig.normal_f32.cu_deviceptr();
                        let mut c_dst = rr.hdr_color[buf_idx];
                        let mut d_dst = rr.diffuse_albedo[buf_idx];
                        let mut s_dst = rr.spec_albedo[buf_idx];
                        let mut n_dst = rr.normal_rough[buf_idx];
                        let mut pack_args: [*mut c_void; 10] = [
                            (&raw mut c_src).cast(),
                            (&raw mut d_src).cast(),
                            (&raw mut s_src).cast(),
                            (&raw mut n_src).cast(),
                            (&raw mut c_dst).cast(),
                            (&raw mut d_dst).cast(),
                            (&raw mut s_dst).cast(),
                            (&raw mut n_dst).cast(),
                            (&raw mut w).cast(),
                            (&raw mut h).cast(),
                        ];
                        // SAFETY: `pack_dlss_rr_aovs_f16` takes 4 `float4*` inputs, 4 `ushort4*`
                        // outputs (`cur_w * cur_h * 8` bytes each), and `(u32, u32)`; all buffers
                        // remain alive through the single end-of-frame fence.
                        let pack_res = unsafe {
                            cu::cuLaunchKernel(
                                rig.tonemap.pack_rr_func,
                                gx,
                                gy,
                                1,
                                16,
                                16,
                                1,
                                0,
                                rig.stream.cu_stream(),
                                pack_args.as_mut_ptr(),
                                std::ptr::null_mut(),
                            )
                        };
                        if pack_res != cu::cudaError_enum_CUDA_SUCCESS {
                            bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                "DLSS-RR AOV pack kernel failed ({pack_res:?})"
                            )));
                        }
                    }
                }
                if let (Some(prof), Some(slot)) = (rig.gpu_profiler.as_mut(), prof_slot) {
                    prof.mark_frame_end(slot, &rig.stream);
                }

                // Single end-of-frame fence ensuring shared RGBA8 / HDR / DLSS buffers are complete
                // before handing `buf_idx` to the Vulkan/wgpu UI thread.
                rig.stream.synchronize().expect("frame fence sync");

                let (rt_ms, denoise_ms, bloom_ms, tonemap_ms, cuda_compute_ms) =
                    if let Some(prof) = rig.gpu_profiler.as_mut() {
                        let s = prof.poll_completed();
                        (
                            s.rt_cores_ms,
                            s.tensor_cores_ms,
                            s.bloom_ms,
                            s.tonemap_ms,
                            s.cuda_compute_ms(),
                        )
                    } else {
                        (t_burst.elapsed().as_secs_f32() * 1000.0, 0.0, 0.0, 0.0, 0.0)
                    };

                if let Some((jpeg_path, quality)) = pending_jpeg.take() {
                    let msg = match save_jpeg_from_rgba8(
                        &cuda,
                        &rig.stream,
                        buf_ptrs[buf_idx],
                        cur_w,
                        cur_h,
                        &jpeg_path,
                        quality,
                    ) {
                        Ok(()) => format!("Saved {}", jpeg_path.display()),
                        Err(e) => format!("JPEG failed: {e}"),
                    };
                    bus.emit_render_event(RenderEvent::StatusMessage(msg));
                }

                accum_count = accum_count.saturating_add(1);
                frame_index = frame_index.wrapping_add(1);

                if let Some(mut burst) = active_burst.take() {
                    if accum_count >= burst.target_accum {
                        let fidx = burst.current_frame;
                        let exr_name = format!("frame_{fidx:04}.exr");
                        let json_name = format!("frame_{fidx:04}.sdg.json");
                        let exr_path = burst.dir.join(&exr_name);
                        let json_path = burst.dir.join(&json_name);

                        let exr_res = readback_and_write_sdg_exr(
                            &rig,
                            cur_w,
                            cur_h,
                            &exr_path,
                            &mut burst_exr_scratch,
                        );
                        let json_res = exr_res.and_then(|()| {
                            export_sdg_json_with_backend(
                                &mut rig,
                                &json_path,
                                &active_camera,
                                settings.camera_projection,
                                cur_w,
                                cur_h,
                                fidx,
                                traffic_time,
                                burst.occluder,
                            )
                        });
                        match json_res {
                            Ok((actor_count, used_occ)) => {
                                burst.last_active_occluder = used_occ;
                                burst.entries.push(SdgBurstFrameEntry {
                                    burst_index: fidx,
                                    render_frame_index: frame_index,
                                    time_s: traffic_time,
                                    exr_file: exr_name,
                                    json_file: json_name,
                                    actor_count,
                                    occluder_used: used_occ.label().to_string(),
                                });
                                burst.current_frame += 1;
                                burst.stepped_current_frame = false;
                                prev_camera = active_camera;
                                if burst.current_frame >= burst.total_frames {
                                    let manifest = SdgBurstManifest {
                                        schema_version: "1.0".to_string(),
                                        scene_id: rig.csg_host.active_scene_id(),
                                        scene_idx: settings.csg_mode,
                                        variant_idx: settings.csg_asset_idx,
                                        width: cur_w,
                                        height: cur_h,
                                        spp_per_frame: burst
                                            .target_accum
                                            .saturating_mul(launch_spp),
                                        projection: settings.camera_projection.label().to_string(),
                                        requested_occluder: burst.occluder.label().to_string(),
                                        crowd_backend: last_crowd_backend.label().to_string(),
                                        exr_channels: crate::sdg_export::SDG_EXR_CHANNELS
                                            .iter()
                                            .map(|&s| s.to_string())
                                            .collect(),
                                        frames: burst.entries,
                                    };
                                    let manifest_path = burst.dir.join("manifest.json");
                                    match write_sdg_manifest_atomic(&manifest_path, &manifest) {
                                        Ok(()) => {
                                            bus.emit_render_event(RenderEvent::StatusMessage(
                                                format!(
                                                    "Saved SDG Burst ({}f EXR+JSON, {}) -> {}",
                                                    burst.total_frames,
                                                    burst.last_active_occluder.label(),
                                                    burst.dir.display()
                                                ),
                                            ));
                                        }
                                        Err(e) => {
                                            bus.emit_render_event(RenderEvent::StatusMessage(
                                                format!("SDG Burst manifest failed: {e}"),
                                            ));
                                        }
                                    }
                                } else {
                                    bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                        "SDG Burst: {}/{} frames ({})",
                                        burst.current_frame,
                                        burst.total_frames,
                                        burst.last_active_occluder.label()
                                    )));
                                    active_burst = Some(burst);
                                }
                            }
                            Err(e) => {
                                bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                    "SDG Burst aborted at frame {fidx}: {e}"
                                )));
                            }
                        }
                    } else {
                        active_burst = Some(burst);
                    }
                }

                bus.emit_render_event(RenderEvent::FrameReady(FrameReady {
                    buf_idx,
                    gen_id: cur_gen,
                    accum_count,
                    total_samples: accum_count.saturating_mul(launch_spp),
                    jitter,
                    rt_ms,
                    denoise_ms,
                    bloom_ms,
                    tonemap_ms,
                    cuda_compute_ms,
                    ser_supported: rig.ser_supported,
                    csg_stats: rig.csg_host.stats(),
                    fast_preview: frame_is_fast_preview,
                    possess_camera: possess_cam_out,
                    rendered_camera: active_camera,
                    crowd_step_ms: last_crowd_step_ms,
                    crowd_active_backend: last_crowd_backend,
                    crowd_actor_count: rig.csg_host.crowd_actor_count() as u32,
                }));
            }
        })
        .expect("spawn optix-render thread")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filigree_omm_bake_produces_valid_four_state_coverage() {
        let level = 5u32;
        let bytes = bake_filigree_omm_bytes(level);
        // 2 shared micromaps * (4^5 = 1024 micro-tris) * 2 bits / 8 = 512 bytes
        assert_eq!(bytes.len(), 512);
        let mut counts = [0u32; 4];
        for b in &bytes {
            for shift in [0, 2, 4, 6] {
                counts[((b >> shift) & 0x3) as usize] += 1;
            }
        }
        assert_eq!(counts.iter().sum::<u32>(), 2048);
        // Both definitive states must dominate so hardware OMM skips the vast majority of any-hit calls.
        assert!(counts[0] > 300, "expected substantial transparent cutouts: {counts:?}");
        assert!(counts[1] > 100, "expected substantial opaque filigree struts: {counts:?}");
        assert!(
            counts[2] + counts[3] > 0,
            "expected unknown boundary micro-tris for pixel-exact edges: {counts:?}"
        );
    }

    #[test]
    fn filigree_band_geometry_and_sdf_borders() {
        let tris = build_filigree_band_tris(2.15, 0.45, 24);
        assert_eq!(tris.len(), 24 * 6);
        // Top and bottom borders are solid (<= 0), empty pocket inside center ring is transparent (> 0).
        assert!(filigree_sdf(0.5, 0.02) < 0.0);
        assert!(filigree_sdf(0.5, 0.98) < 0.0);
        assert!(filigree_sdf(0.5, 0.5) > 0.0);
    }

    #[test]
    fn ias_build_opts_enables_update_and_disables_compaction() {
        let opts = ias_build_opts();
        assert!(opts.allow_update, "persistent IAS requires allow_update = true");
        assert!(!opts.compact, "Accel::instances_into requires compact = false");
        assert!(opts.prefer_fast_trace);
    }

    #[test]
    fn launch_params_abi_size_and_offsets_match_cuda() {
        assert_eq!(std::mem::size_of::<LaunchParams>(), 640);
        assert_eq!(std::mem::align_of::<LaunchParams>(), 8);
        assert_eq!(std::mem::offset_of!(LaunchParams, csg_compounds), 480);
        assert_eq!(std::mem::offset_of!(LaunchParams, csg_instances), 536);
        assert_eq!(std::mem::offset_of!(LaunchParams, csg_instance_count), 544);
        assert_eq!(std::mem::offset_of!(LaunchParams, csg_flags), 556);
        assert_eq!(std::mem::offset_of!(LaunchParams, csg_wetness), 560);
        assert_eq!(std::mem::offset_of!(LaunchParams, camera_projection), 568);
        assert_eq!(std::mem::offset_of!(LaunchParams, probe_grid_wh), 572);
        assert_eq!(std::mem::offset_of!(LaunchParams, probe_candidates), 576);
        assert_eq!(std::mem::offset_of!(LaunchParams, probe_samples), 584);
        assert_eq!(std::mem::offset_of!(LaunchParams, csg_prev_xforms), 592);
        assert_eq!(std::mem::offset_of!(LaunchParams, out_semantic_u32), 600);
        assert_eq!(std::mem::offset_of!(LaunchParams, out_instance_u32), 608);
        assert_eq!(std::mem::offset_of!(LaunchParams, sdg_ray_queries), 616);
        assert_eq!(std::mem::offset_of!(LaunchParams, sdg_ray_results), 624);
        assert_eq!(std::mem::offset_of!(LaunchParams, sdg_ray_count), 632);
        assert_eq!(std::mem::offset_of!(LaunchParams, _pad2), 636);
        assert_eq!(std::mem::size_of::<GpuProbeCandidate>(), 64);
        assert_eq!(std::mem::size_of::<GpuSdgRayQuery>(), 32);
    }

    #[test]
    fn optix_batched_occluder_classifies_visible_self_and_world() {
        let p_vis = ActorVec3::new(1.0, 2.0, 3.0);
        let p_self = ActorVec3::new(4.0, 5.0, 6.0);
        let p_world = ActorVec3::new(7.0, 8.0, 9.0);
        let uid = 700_042u32;
        let keys = [
            (uid, p_vis.x.to_bits(), p_vis.y.to_bits(), p_vis.z.to_bits()),
            (uid, p_self.x.to_bits(), p_self.y.to_bits(), p_self.z.to_bits()),
            (
                uid,
                p_world.x.to_bits(),
                p_world.y.to_bits(),
                p_world.z.to_bits(),
            ),
        ];
        let blockers = [u32::MAX, uid, 12345u32];
        let occ = OptixBatchedOccluder::from_pairs(&keys, &blockers);
        let eye = ActorVec3::new(0.0, 1.6, 0.0);
        assert_eq!(occ.test_visibility(eye, p_vis, 0.08, uid), KeypointVis::Visible);
        assert_eq!(
            occ.test_visibility(eye, p_self, 0.08, uid),
            KeypointVis::SelfOccluded
        );
        assert_eq!(
            occ.test_visibility(eye, p_world, 0.08, uid),
            KeypointVis::OccludedWorld
        );
    }

    #[test]
    fn camera_projection_rays_match_cardinal_axes_and_fisheye_mask() {
        let cam = CameraFrame {
            eye: [1.0, 2.0, 3.0],
            fwd: [0.0, 0.0, 1.0],
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            tan_half_fov: 0.5,
        };
        for proj in CameraProjection::ALL {
            let (orig, dir) =
                camera_projection_ray(&cam, proj, 0.5, 0.5, 16.0 / 9.0).expect("center ray");
            assert_eq!(orig, cam.eye);
            assert!(
                (dir[2] - 1.0).abs() < 1e-5 && dir[0].abs() < 1e-5 && dir[1].abs() < 1e-5,
                "{proj:?} center must point straight along +fwd: {dir:?}"
            );
        }
        // Corner of 16:9 viewport lies outside the inscribed 180-degree fisheye circle -> masked None.
        assert!(camera_projection_ray(&cam, CameraProjection::Fisheye180, 0.0, 0.0, 16.0 / 9.0).is_none());
        // Equirect360 & SlitCam edges (norm_x = 0.0 / 1.0) wrap 180 degrees to -fwd.
        for proj in [CameraProjection::Equirect360, CameraProjection::SlitCam] {
            let (_, back) = camera_projection_ray(&cam, proj, 0.0, 0.5, 16.0 / 9.0).expect("edge");
            assert!(
                (back[2] + 1.0).abs() < 1e-5,
                "{proj:?} left seam must look behind (-fwd): {back:?}"
            );
        }
        // Equirect360 poles look straight up / down.
        let (_, zenith) =
            camera_projection_ray(&cam, CameraProjection::Equirect360, 0.5, 0.0, 2.0).expect("top");
        assert!((zenith[1] - 1.0).abs() < 1e-5, "equirect top = +up: {zenith:?}");
    }

    #[test]
    fn cryptomatte_murmur3_produces_valid_non_nan_f32_ids() {
        fn murmur3_32_3u(k0: u32, k1: u32, k2: u32) -> u32 {
            let (c1, c2) = (0xcc9e_2d51u32, 0x1b87_3593u32);
            let mut h = 0u32;
            for mut k in [k0, k1, k2] {
                k = k.wrapping_mul(c1).rotate_left(15).wrapping_mul(c2);
                h ^= k;
                h = h.rotate_left(13).wrapping_mul(5).wrapping_add(0xe654_6b64);
            }
            h ^= 12;
            h ^= h >> 16;
            h = h.wrapping_mul(0x85eb_ca6b);
            h ^= h >> 13;
            h = h.wrapping_mul(0xc2b2_ae35);
            h ^ (h >> 16)
        }
        for i in 0..2048u32 {
            let h = murmur3_32_3u(i, i.wrapping_mul(7), i & 31);
            let mut exp = (h >> 23) & 0xFF;
            if exp == 0 || exp == 255 {
                exp ^= 1;
            }
            let id_f32 = f32::from_bits((exp << 23) | (h & 0x007F_FFFF));
            assert!(id_f32.is_normal(), "Cryptomatte ID must never be NaN/Inf/denormal/zero");
        }
    }

    #[test]
    fn fractal_instance_transform_builds_exact_trs_matrix() {
        // Default identity TRS
        let id = fractal_instance_transform([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0);
        assert_eq!(id, [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]);

        // Pure translation
        let tr = fractal_instance_transform([12.5, -3.25, 44.0], [0.0, 0.0, 0.0], 1.0);
        assert_eq!([tr[3], tr[7], tr[11]], [12.5, -3.25, 44.0]);

        // Uniform scale clamps to [0.05, 50.0] and determinant equals scale^3
        for (raw_s, expect_s) in [(0.001_f32, 0.05_f32), (0.05, 0.05), (3.5, 3.5), (50.0, 50.0), (200.0, 50.0)] {
            let m = fractal_instance_transform([1.0, 2.0, 3.0], [25.0, -40.0, 65.0], raw_s);
            let mat3 = glam::Mat3::from_cols(
                glam::Vec3::new(m[0], m[4], m[8]),
                glam::Vec3::new(m[1], m[5], m[9]),
                glam::Vec3::new(m[2], m[6], m[10]),
            );
            let det = mat3.determinant();
            let expect_det = expect_s * expect_s * expect_s;
            assert!(
                (det - expect_det).abs() <= expect_det * 1e-4 + 1e-5,
                "det={det}, expected={expect_det} for scale={raw_s}"
            );
        }

        // Combined T * R_z * R_y * R_x * S exact point mapping
        let t = [10.0_f32, -4.0, 22.5];
        let r = [30.0_f32, 45.0, -60.0];
        let s = 2.5_f32;
        let m = fractal_instance_transform(t, r, s);
        let p_obj = glam::Vec3::new(0.7, -1.1, 0.4);
        let got = glam::Vec3::new(
            m[0] * p_obj.x + m[1] * p_obj.y + m[2] * p_obj.z + m[3],
            m[4] * p_obj.x + m[5] * p_obj.y + m[6] * p_obj.z + m[7],
            m[8] * p_obj.x + m[9] * p_obj.y + m[10] * p_obj.z + m[11],
        );
        let expected = glam::Vec3::from_array(t)
            + glam::Mat3::from_rotation_z(r[2].to_radians())
                * (glam::Mat3::from_rotation_y(r[1].to_radians())
                    * (glam::Mat3::from_rotation_x(r[0].to_radians()) * (p_obj * s)));
        assert!(
            (got - expected).length() < 1e-4,
            "got={got:?}, expected={expected:?}"
        );
    }

    #[test]
    fn save_jpeg_preserves_accum_count_when_max_accum_is_zero() {
        assert_eq!(clamp_accum_for_jpeg(128, 0), 128);
        assert_eq!(clamp_accum_for_jpeg(0, 0), 0);
        assert_eq!(clamp_accum_for_jpeg(256, 256), 255);
        assert_eq!(clamp_accum_for_jpeg(300, 256), 255);
        assert_eq!(clamp_accum_for_jpeg(42, 256), 42);
        assert_eq!(clamp_accum_for_jpeg(5, 1), 0);
    }

    #[test]
    fn ias_classic_state_ignores_sat_angle_and_fractal_when_classic_hidden() {
        let mut s = DemoSettings {
            csg_mode: 1,
            csg_show_fractal: false,
            sat_on: true,
            sat_orbit: true,
            sat_count: 8,
            auto_rotate: true,
            ..Default::default()
        };

        let (st_a, _) = compute_ias_classic_state(&s, 0.0, 0.0);
        s.fractal_translate = [10.0, 5.0, -3.0];
        s.rot_y = 90.0;
        let (st_b, _) = compute_ias_classic_state(&s, 2.5, 7.25);
        assert_eq!(st_a, IasClassicState::Hidden);
        assert_eq!(
            st_a, st_b,
            "changing sat_angle or fractal_transform while !show_classic must cause 0 IAS rebuilds"
        );

        s.csg_show_fractal = true;
        let (st_c, _) = compute_ias_classic_state(&s, 0.0, 0.0);
        let (st_d, _) = compute_ias_classic_state(&s, 0.0, 1.5);
        assert_ne!(st_c, IasClassicState::Hidden);
        assert_ne!(st_c, st_d);
    }

    #[test]
    fn dlss_rr_skips_redundant_cuda_tonemap_only_when_rr_owns_display() {
        assert!(should_skip_cuda_tonemap(true, true, 0, false, false));
        assert!(!should_skip_cuda_tonemap(false, true, 0, false, false));
        assert!(!should_skip_cuda_tonemap(true, false, 0, false, false));
        assert!(!should_skip_cuda_tonemap(true, true, 1, false, false));
        assert!(!should_skip_cuda_tonemap(true, true, 0, true, false));
        assert!(!should_skip_cuda_tonemap(true, true, 0, false, true));
    }
}
