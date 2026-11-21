//! Asynchronous OptiX path-tracing worker thread communicating with the 60 FPS GUI via channels.

use std::collections::VecDeque;
use std::ffi::{CString, c_void};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Instant;

use bytemuck::{Pod, Zeroable};
use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, DeviceBuffer, Stream};
use optix::{
    Aabb, Accel, AovType, BuildOptions, CustomPrimitives, Denoiser, DenoiserModel, DenoiserOptions,
    DeviceContext, GeometryFlags, Guides, HitGroup, Image, Instance, Layer, LinkOptions,
    MicromapCount, MicromapDesc, MicromapFormat, MicromapIndices, MicromapOptions, MicromapUse,
    Module, ModuleOptions, OpacityMicromapArray, Params as DenoiserParams, Pipeline,
    PipelineOptions, ProgramGroup, SbtRecord, Scratch, ShaderBindingTable, TriangleMesh,
    micro_to_base_bary,
};

use crate::fft_bloom::FftBloomPass;
use crate::gpu_export::{VideoProfile, VideoRecorder, save_jpeg_from_rgba8};

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
    _pad0: [u32; 2],
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
}

const _: () = assert!(std::mem::size_of::<LaunchParams>() == 480);

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

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DemoSettings {
    pub fractal_kind: u32,
    // Fractal Transform
    pub obj_scale: f32,
    pub rot_x: f32,
    pub rot_y: f32,
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
    // Video Export (MP4 H.264 SDR / HEVC PQ10 HDR10 + Timing)
    pub record_hdr_pq: bool,
    pub record_fixed_fps: bool,
    // Per-environment procedural parameters (persisted in PersistedState)
    pub env: EnvSettings,
}

impl Default for DemoSettings {
    fn default() -> Self {
        Self {
            fractal_kind: 0,
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
            record_hdr_pq: false,
            record_fixed_fps: false,
            env: EnvSettings::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
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

pub enum RenderCmd {
    Update {
        settings: Box<DemoSettings>,
        camera: CameraFrame,
        reset_accum: bool,
    },
    Resize {
        width: u32,
        height: u32,
        buf_ptrs: [u64; 2],
        depth_ptrs: [u64; 2],
        motion_ptrs: [u64; 2],
        rr_ptrs: Option<DlssRrBufferPtrs>,
        gen_id: u64,
    },
    ReturnBuffer {
        buf_idx: usize,
        gen_id: u64,
    },
    SaveJpeg {
        path: PathBuf,
        quality: u8,
    },
    ToggleRecord {
        path: PathBuf,
        fps: u32,
        profile: VideoProfile,
    },
    ReloadShader,
    Shutdown,
}

pub struct FrameReady {
    pub buf_idx: usize,
    pub gen_id: u64,
    pub accum_count: u32,
    pub total_samples: u32,
    pub jitter: [f32; 2],
    pub rt_ms: f32,
    pub denoise_ms: f32,
    pub bloom_ms: f32,
    pub tonemap_ms: f32,
    pub is_recording: bool,
    pub record_frames: u64,
    pub ser_supported: bool,
    pub status_msg: Option<String>,
}

struct TonemapKernel {
    module: cu::CUmodule,
    func: cu::CUfunction,
    pack_rr_func: cu::CUfunction,
}

impl Drop for TonemapKernel {
    fn drop(&mut self) {
        if !self.module.is_null() {
            // SAFETY: `self.module` was loaded via `cuModuleLoadData` on the active context.
            unsafe { cu::cuModuleUnload(self.module) };
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
    ias: Arc<Accel>,
    pipeline: Pipeline,
    sbt: ShaderBindingTable,
    ser_supported: bool,
    denoiser: Denoiser,
    denoiser_is_temporal: bool,
    temporal_has_prev: bool,
    color_f32: DeviceBuffer<[f32; 4]>,
    albedo_f32: DeviceBuffer<[f32; 4]>,
    diffuse_albedo_f32: DeviceBuffer<[f32; 4]>,
    spec_albedo_f32: DeviceBuffer<[f32; 4]>,
    normal_f32: DeviceBuffer<[f32; 4]>,
    denoised_f32: DeviceBuffer<[f32; 4]>,
    prev_denoised_f32: DeviceBuffer<[f32; 4]>,
    motion_f32: DeviceBuffer<[f32; 2]>,
    depth_f32: DeviceBuffer<f32>,
    params_buf: DeviceBuffer<u8>,
    tonemap: TonemapKernel,
    bloom: Option<FftBloomPass>,
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

fn try_compile_ptx(src: &str, name: &str, include_optix: bool) -> Result<Vec<u8>, String> {
    let mut opts = nvrtc::CompileOptions::new()
        .arch("compute_75")
        .std("c++17")
        .fast_math();

    if include_optix {
        if let Some(p) = find_optix_include() {
            opts = opts.include_dir(&p);
        }
        if let Some(p) = find_cuda_include() {
            opts = opts.include_dir(&p);
        }
    }

    let mut prog = nvrtc::Program::new(src, Some(name), &[])
        .map_err(|e| format!("NVRTC program {name} failed: {e}"))?;
    prog.compile(&opts)
        .map_err(|e| format!("NVRTC compile {name} failed: {e}"))?;
    let mut bytes = prog
        .ptx()
        .map_err(|e| format!("NVRTC ptx {name} failed: {e}"))?;
    if bytes.last() != Some(&0) {
        bytes.push(0);
    }
    Ok(bytes)
}

fn build_optix_pipeline(
    optix_ctx: &Arc<DeviceContext>,
    stream: &Stream,
    rt_ptx: &[u8],
) -> Result<(Pipeline, ShaderBindingTable), String> {
    let mut opts = PipelineOptions::new("params", std::mem::size_of::<LaunchParams>(), 8)
        .map_err(|e| format!("PipelineOptions: {e}"))?;
    opts.attribute_values = 4;
    opts.primitive_types = 0; // custom AABB + triangles
    opts.opacity_micromaps = true;
    let (module, _) = Module::new(optix_ctx, &ModuleOptions::default(), &opts, rt_ptx)
        .map_err(|e| format!("Module::new: {e}"))?;

    let raygen =
        ProgramGroup::raygen(&module, "__raygen__render").map_err(|e| format!("raygen: {e}"))?;
    let miss = ProgramGroup::miss(&module, "__miss__miss").map_err(|e| format!("miss: {e}"))?;
    let miss_shadow =
        ProgramGroup::miss(&module, "__miss__shadow").map_err(|e| format!("miss_shadow: {e}"))?;
    let hit_floor = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__hit")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_floor: {e}"))?;
    let hit_fractal = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__fractal")),
            intersection: Some((&module, "__intersection__fractal")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_fractal: {e}"))?;
    let hit_sphere = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__fractal")),
            intersection: Some((&module, "__intersection__sphere")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_sphere: {e}"))?;
    let hit_omm = ProgramGroup::hit_group(
        optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__hit")),
            any_hit: Some((&module, "__anyhit__omm")),
            ..Default::default()
        },
    )
    .map_err(|e| format!("hit_omm: {e}"))?;

    let (pipeline, _) = Pipeline::new(
        optix_ctx,
        &opts,
        LinkOptions::default(),
        &[
            raygen.clone(),
            miss.clone(),
            miss_shadow.clone(),
            hit_floor.clone(),
            hit_fractal.clone(),
            hit_sphere.clone(),
            hit_omm.clone(),
        ],
    )
    .map_err(|e| format!("Pipeline::new: {e}"))?;

    let sbt = ShaderBindingTable::new(
        stream,
        &SbtRecord::new(&raygen, 0u32).map_err(|e| format!("sbt raygen: {e}"))?,
        &[
            SbtRecord::new(&miss, 0u32).map_err(|e| format!("sbt miss0: {e}"))?,
            SbtRecord::new(&miss_shadow, 0u32).map_err(|e| format!("sbt miss1: {e}"))?,
        ],
        &[
            SbtRecord::new(&hit_floor, 0u32).map_err(|e| format!("sbt hit0: {e}"))?,
            SbtRecord::new(&hit_fractal, 0u32).map_err(|e| format!("sbt hit1: {e}"))?,
            SbtRecord::new(&hit_sphere, 0u32).map_err(|e| format!("sbt hit2: {e}"))?,
            SbtRecord::new(&hit_omm, 0u32).map_err(|e| format!("sbt hit3: {e}"))?,
        ],
    )
    .map_err(|e| format!("SBT::new: {e}"))?;

    Ok((pipeline, sbt))
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
}

fn splitmix32(mut x: u32) -> f32 {
    x = x.wrapping_add(0x9E37_79B9);
    x = (x ^ (x >> 16)).wrapping_mul(0x85EB_CA6B);
    x = (x ^ (x >> 13)).wrapping_mul(0xC2B2_AE35);
    x ^= x >> 16;
    (x >> 8) as f32 * (1.0 / 16_777_216.0)
}

fn build_ias(
    optix_ctx: &Arc<DeviceContext>,
    stream: &Arc<Stream>,
    scratch: &mut Scratch,
    gas: SceneGas<'_>,
    state: SceneState,
) -> Arc<Accel> {
    let count = state.sat_count.min(24) as usize;
    let mut instances = Vec::with_capacity(3 + count);
    let mut i0 = Instance::new(gas.floor);
    i0.id = 0;
    i0.sbt_offset = 0;
    instances.push(i0);

    let mut i1 = Instance::new(gas.fractal);
    i1.id = 1;
    i1.sbt_offset = 1;
    instances.push(i1);

    if state.sat_on && count > 0 {
        let (glass_gas, sbt_off) = if state.glass_analytic {
            (gas.glass_analytic, 2)
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
            inst.transform = [r, 0.0, 0.0, px, 0.0, r, 0.0, py, 0.0, 0.0, r, pz];
            instances.push(inst);
        }
    }

    if state.neon_on {
        let mut i3 = Instance::new(gas.neon);
        i3.id = 3;
        i3.sbt_offset = 0;
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
        i4.sbt_offset = 3;
        instances.push(i4);
    }

    Accel::instances(
        optix_ctx,
        stream,
        scratch,
        &instances,
        BuildOptions::default(),
    )
    .expect("IAS")
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
    let stream = cuda.default_stream();
    let optix_ctx = DeviceContext::new(cuda, None, 0, false).expect("OptiX DeviceContext");
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
        module: cu_mod,
        func: cu_func,
        pack_rr_func,
    };

    let (pipeline, sbt) =
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

    let ias = build_ias(
        &optix_ctx,
        &stream,
        &mut scratch,
        SceneGas {
            floor: &floor_gas,
            fractal: &fractal_gas,
            glass_analytic: &glass_analytic_gas,
            glass_mesh: &glass_mesh_gas,
            neon: &neon_gas,
            omm: &omm_gas,
            omm_anyhit: &omm_anyhit_gas,
        },
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
        },
    );

    let (denoiser, denoiser_is_temporal) = create_denoiser(
        &optix_ctx,
        &stream,
        DEFAULT_W,
        DEFAULT_H,
        initial_settings.temporal_denoise,
    );

    let n = (DEFAULT_W * DEFAULT_H) as usize;
    let color_f32 = DeviceBuffer::zeroed(&stream, n).expect("color_f32");
    let albedo_f32 = DeviceBuffer::zeroed(&stream, n).expect("albedo_f32");
    let diffuse_albedo_f32 = DeviceBuffer::zeroed(&stream, n).expect("diffuse_albedo_f32");
    let spec_albedo_f32 = DeviceBuffer::zeroed(&stream, n).expect("spec_albedo_f32");
    let normal_f32 = DeviceBuffer::zeroed(&stream, n).expect("normal_f32");
    let denoised_f32 = DeviceBuffer::zeroed(&stream, n).expect("denoised_f32");
    let prev_denoised_f32 = DeviceBuffer::zeroed(&stream, n).expect("prev_denoised_f32");
    let motion_f32 = DeviceBuffer::zeroed(&stream, n).expect("motion_f32");
    let depth_f32 = DeviceBuffer::zeroed(&stream, n).expect("depth_f32");
    let params_buf =
        DeviceBuffer::zeroed(&stream, std::mem::size_of::<LaunchParams>()).expect("params");
    let bloom = FftBloomPass::new(cuda).ok();
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
        pipeline,
        sbt,
        ser_supported,
        denoiser,
        denoiser_is_temporal,
        temporal_has_prev: false,
        color_f32,
        albedo_f32,
        diffuse_albedo_f32,
        spec_albedo_f32,
        normal_f32,
        denoised_f32,
        prev_denoised_f32,
        motion_f32,
        depth_f32,
        params_buf,
        tonemap,
        bloom,
    }
}

fn resize_rig(rig: &mut OptixRig, width: u32, height: u32, temporal: bool) {
    rig.stream.synchronize().expect("pre-resize sync");
    let n = (width * height) as usize;
    rig.color_f32 = DeviceBuffer::zeroed(&rig.stream, n).expect("color_f32 resize");
    rig.albedo_f32 = DeviceBuffer::zeroed(&rig.stream, n).expect("albedo_f32 resize");
    rig.diffuse_albedo_f32 =
        DeviceBuffer::zeroed(&rig.stream, n).expect("diffuse_albedo_f32 resize");
    rig.spec_albedo_f32 = DeviceBuffer::zeroed(&rig.stream, n).expect("spec_albedo_f32 resize");
    rig.normal_f32 = DeviceBuffer::zeroed(&rig.stream, n).expect("normal_f32 resize");
    rig.denoised_f32 = DeviceBuffer::zeroed(&rig.stream, n).expect("denoised_f32 resize");
    rig.prev_denoised_f32 = DeviceBuffer::zeroed(&rig.stream, n).expect("prev_denoised_f32 resize");
    rig.motion_f32 = DeviceBuffer::zeroed(&rig.stream, n).expect("motion_f32 resize");
    rig.depth_f32 = DeviceBuffer::zeroed(&rig.stream, n).expect("depth_f32 resize");
    let (denoiser, is_temporal) =
        create_denoiser(&rig._optix_ctx, &rig.stream, width, height, temporal);
    rig.denoiser = denoiser;
    rig.denoiser_is_temporal = is_temporal;
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

pub fn spawn_render_thread(
    cuda: Arc<CudaContext>,
    initial_buf_ptrs: [u64; 2],
    initial_depth_ptrs: [u64; 2],
    initial_motion_ptrs: [u64; 2],
    initial_settings: DemoSettings,
    initial_camera: CameraFrame,
) -> (Sender<RenderCmd>, Receiver<FrameReady>, JoinHandle<()>) {
    let (cmd_tx, cmd_rx) = mpsc::channel::<RenderCmd>();
    let (frame_tx, frame_rx) = mpsc::channel::<FrameReady>();

    let handle = thread::Builder::new()
        .name("optix-render".to_string())
        .spawn(move || {
            crate::nvtx::name_current_thread(c"optix-render");
            cuda.bind_to_thread()
                .expect("cuda bind_to_thread on render thread");
            let mut rig = init_optix(&cuda, &initial_settings);
            let mut cur_w = DEFAULT_W;
            let mut cur_h = DEFAULT_H;
            let mut cur_gen: u64 = 0;
            let mut buf_ptrs = initial_buf_ptrs;
            let mut depth_ptrs = initial_depth_ptrs;
            let mut motion_ptrs = initial_motion_ptrs;
            let mut rr_ptrs: Option<DlssRrBufferPtrs> = None;
            let mut free_bufs = VecDeque::from([0usize, 1usize]);
            let mut settings = initial_settings;
            let mut camera = initial_camera;
            let mut prev_camera = initial_camera;
            let mut accum_count: u32 = 0;
            let mut frame_index: u32 = 0;
            let mut spin_angle: f32 = 0.0;
            let mut sat_angle: f32 = 0.0;
            let mut last_ias_state = (
                settings.sat_on,
                settings.glass_analytic,
                settings.sat_u_segs,
                settings.sat_v_segs,
                settings.sat_count,
                settings.sat_seed,
                settings.neon_on,
                settings.omm_mode,
                [
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
                    0.0,
                ],
            );
            let mut last_tick = Instant::now();
            let mut last_record_at: Option<Instant> = None;
            let mut pending_jpeg: Option<(PathBuf, u8)> = None;
            let mut recorder: Option<(VideoRecorder, u32)> = None;
            let mut pending_status: Option<String> = None;

            loop {
                let mut should_exit = false;
                let apply_cmd = |cmd: RenderCmd,
                                 cuda: &Arc<CudaContext>,
                                 rig: &mut OptixRig,
                                 cur_w: &mut u32,
                                 cur_h: &mut u32,
                                 cur_gen: &mut u64,
                                 buf_ptrs: &mut [u64; 2],
                                 depth_ptrs: &mut [u64; 2],
                                 motion_ptrs: &mut [u64; 2],
                                 rr_ptrs: &mut Option<DlssRrBufferPtrs>,
                                 free_bufs: &mut VecDeque<usize>,
                                 settings: &mut DemoSettings,
                                 camera: &mut CameraFrame,
                                 accum_count: &mut u32,
                                 pending_jpeg: &mut Option<(PathBuf, u8)>,
                                 recorder: &mut Option<(VideoRecorder, u32)>,
                                 last_record_at: &mut Option<Instant>,
                                 pending_status: &mut Option<String>|
                 -> bool {
                    match cmd {
                        RenderCmd::Update {
                            settings: s,
                            camera: c,
                            reset_accum,
                        } => {
                            let mut tmp = *s;
                            tmp.exposure = settings.exposure;
                            tmp.split_on = settings.split_on;
                            tmp.split_x = settings.split_x;
                            tmp.right_is_aov = settings.right_is_aov;
                            tmp.bloom_on = settings.bloom_on;
                            tmp.bloom_threshold = settings.bloom_threshold;
                            tmp.bloom_intensity = settings.bloom_intensity;
                            tmp.bloom_blades = settings.bloom_blades;
                            tmp.dlss_sharpness = settings.dlss_sharpness;
                            tmp.record_hdr_pq = settings.record_hdr_pq;
                            tmp.record_fixed_fps = settings.record_fixed_fps;
                            if s.aov_mode < 3 && settings.aov_mode < 3 {
                                tmp.aov_mode = settings.aov_mode;
                            }
                            let only_non_accum = tmp == *settings && c == *camera;
                            if reset_accum || (!only_non_accum && (*s != *settings || c != *camera))
                            {
                                *accum_count = 0;
                            }
                            *settings = *s;
                            *camera = c;
                            false
                        }
                        RenderCmd::Resize {
                            width,
                            height,
                            buf_ptrs: new_ptrs,
                            depth_ptrs: new_depth_ptrs,
                            motion_ptrs: new_motion_ptrs,
                            rr_ptrs: new_rr_ptrs,
                            gen_id,
                        } => {
                            if let Some((rec, _)) = recorder.take() {
                                let _ = rec.finish();
                                *last_record_at = None;
                                *pending_status = Some("Recording stopped on resize".to_string());
                            }
                            if width != *cur_w || height != *cur_h {
                                resize_rig(rig, width, height, settings.temporal_denoise);
                                *cur_w = width;
                                *cur_h = height;
                            }
                            *buf_ptrs = new_ptrs;
                            *depth_ptrs = new_depth_ptrs;
                            *motion_ptrs = new_motion_ptrs;
                            *rr_ptrs = new_rr_ptrs;
                            *cur_gen = gen_id;
                            free_bufs.clear();
                            free_bufs.push_back(0);
                            free_bufs.push_back(1);
                            *accum_count = 0;
                            false
                        }
                        RenderCmd::ReturnBuffer { buf_idx, gen_id } => {
                            if gen_id == *cur_gen && !free_bufs.contains(&buf_idx) {
                                free_bufs.push_back(buf_idx);
                            }
                            false
                        }
                        RenderCmd::SaveJpeg { path, quality } => {
                            *pending_jpeg = Some((path, quality));
                            *accum_count = (*accum_count).min(settings.max_accum.saturating_sub(1));
                            false
                        }
                        RenderCmd::ToggleRecord { path, fps, profile } => {
                            if let Some((rec, _)) = recorder.take() {
                                *last_record_at = None;
                                let frames = rec.frame_count();
                                *pending_status = Some(match rec.finish() {
                                    Ok(n) => format!("Saved MP4 ({n} frames)"),
                                    Err(e) => format!("Video finish failed ({frames}f): {e}"),
                                });
                            } else {
                                match VideoRecorder::start(
                                    cuda, *cur_w, *cur_h, fps, profile, &path,
                                ) {
                                    Ok(rec) => {
                                        *recorder = Some((rec, fps));
                                        *last_record_at = None;
                                        *pending_status =
                                            Some(format!("Recording to {}", path.display()));
                                    }
                                    Err(e) => {
                                        *pending_status = Some(format!("Record start failed: {e}"));
                                    }
                                }
                            }
                            false
                        }
                        RenderCmd::ReloadShader => {
                            let disk_path = PathBuf::from("crates/nv-demo/shaders/raytracer.cu");
                            let res = std::fs::read_to_string(&disk_path)
                                .map_err(|e| format!("Read {}: {e}", disk_path.display()))
                                .and_then(|src| try_compile_ptx(&src, "raytracer.cu", true))
                                .and_then(|ptx| {
                                    build_optix_pipeline(&rig._optix_ctx, &rig.stream, &ptx)
                                });
                            match res {
                                Ok((pipeline, sbt)) => {
                                    let _ = rig.stream.synchronize();
                                    rig.pipeline = pipeline;
                                    rig.sbt = sbt;
                                    *accum_count = 0;
                                    *pending_status = Some("Shader hot-reloaded".to_string());
                                }
                                Err(e) => {
                                    *pending_status = Some(format!("Hot-reload failed: {e}"));
                                }
                            }
                            false
                        }
                        RenderCmd::Shutdown => {
                            if let Some((rec, _)) = recorder.take() {
                                let _ = rec.finish();
                            }
                            *last_record_at = None;
                            true
                        }
                    }
                };

                while let Ok(cmd) = cmd_rx.try_recv() {
                    if apply_cmd(
                        cmd,
                        &cuda,
                        &mut rig,
                        &mut cur_w,
                        &mut cur_h,
                        &mut cur_gen,
                        &mut buf_ptrs,
                        &mut depth_ptrs,
                        &mut motion_ptrs,
                        &mut rr_ptrs,
                        &mut free_bufs,
                        &mut settings,
                        &mut camera,
                        &mut accum_count,
                        &mut pending_jpeg,
                        &mut recorder,
                        &mut last_record_at,
                        &mut pending_status,
                    ) {
                        should_exit = true;
                        break;
                    }
                }
                if should_exit {
                    break;
                }

                let converged = !settings.auto_rotate
                    && !(settings.sat_on && settings.sat_orbit && settings.sat_count > 0)
                    && recorder.is_none()
                    && pending_jpeg.is_none()
                    && pending_status.is_none()
                    && settings.max_accum > 0
                    && accum_count >= settings.max_accum;

                if converged || free_bufs.is_empty() {
                    match cmd_rx.recv() {
                        Ok(cmd) => {
                            if apply_cmd(
                                cmd,
                                &cuda,
                                &mut rig,
                                &mut cur_w,
                                &mut cur_h,
                                &mut cur_gen,
                                &mut buf_ptrs,
                                &mut depth_ptrs,
                                &mut motion_ptrs,
                                &mut rr_ptrs,
                                &mut free_bufs,
                                &mut settings,
                                &mut camera,
                                &mut accum_count,
                                &mut pending_jpeg,
                                &mut recorder,
                                &mut last_record_at,
                                &mut pending_status,
                            ) {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                    continue;
                }

                let Some(buf_idx) = free_bufs.pop_front() else {
                    continue;
                };

                let now = Instant::now();
                let dt = if let Some((_, rec_fps)) = recorder.as_ref()
                    && settings.record_fixed_fps
                {
                    1.0 / (*rec_fps).max(1) as f32
                } else {
                    now.duration_since(last_tick).as_secs_f32().min(0.1)
                };
                last_tick = now;
                if settings.auto_rotate {
                    spin_angle += dt * settings.spin_speed;
                    accum_count = 0;
                }
                if settings.sat_on && settings.sat_orbit && settings.sat_count > 0 {
                    sat_angle += dt * settings.spin_speed;
                    accum_count = 0;
                }

                if !settings.glass_analytic
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
                let ias_state = (
                    settings.sat_on,
                    settings.glass_analytic,
                    settings.sat_u_segs,
                    settings.sat_v_segs,
                    settings.sat_count,
                    settings.sat_seed,
                    settings.neon_on,
                    settings.omm_mode,
                    [
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
                        sat_angle,
                    ],
                );
                if ias_state != last_ias_state {
                    rig.ias = build_ias(
                        &rig._optix_ctx,
                        &rig.stream,
                        &mut rig._scratch,
                        SceneGas {
                            floor: &rig.floor_gas,
                            fractal: &rig.fractal_gas,
                            glass_analytic: &rig.glass_analytic_gas,
                            glass_mesh: &rig.glass_mesh_gas,
                            neon: &rig.neon_gas,
                            omm: &rig.omm_gas,
                            omm_anyhit: &rig.omm_anyhit_gas,
                        },
                        SceneState {
                            sat_angle,
                            sat_count: settings.sat_count,
                            sat_seed: settings.sat_seed,
                            sat_size_range: [settings.sat_size_min, settings.sat_size_max],
                            sat_orbit_range: [settings.sat_orbit_min, settings.sat_orbit_max],
                            sat_height_range: [settings.sat_height_min, settings.sat_height_max],
                            sat_speed_range: [settings.sat_speed_min, settings.sat_speed_max],
                            sat_bob_amp_range: [settings.sat_bob_amp_min, settings.sat_bob_amp_max],
                            sat_bob_freq_range: [
                                settings.sat_bob_freq_min,
                                settings.sat_bob_freq_max,
                            ],
                            sat_hue_spread: settings.sat_hue_spread,
                            sat_on: settings.sat_on,
                            glass_analytic: settings.glass_analytic,
                            neon_on: settings.neon_on,
                            omm_mode: settings.omm_mode,
                        },
                    );
                    last_ias_state = ias_state;
                }

                if settings.temporal_denoise != rig.denoiser_is_temporal {
                    let (d, is_t) = create_denoiser(
                        &rig._optix_ctx,
                        &rig.stream,
                        cur_w,
                        cur_h,
                        settings.temporal_denoise,
                    );
                    rig.denoiser = d;
                    rig.denoiser_is_temporal = is_t;
                    rig.temporal_has_prev = false;
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

                let rr_export = settings.dlss_rr_on || rr_ptrs.is_some();
                let dlss_any = settings.dlss_on || rr_export;
                let jitter = if dlss_any {
                    accum_count = 0;
                    let period = if rr_export { 64 } else { 16 };
                    [
                        halton((frame_index % period) + 1, 2) - 0.5,
                        halton((frame_index % period) + 1, 3) - 0.5,
                    ]
                } else {
                    [0.0, 0.0]
                };

                let lp = LaunchParams {
                    out_color_f32: rig.color_f32.cu_deviceptr(),
                    out_albedo_f32: rig.albedo_f32.cu_deviceptr(),
                    out_normal_f32: rig.normal_f32.cu_deviceptr(),
                    handle: rig.ias.handle(),
                    width: cur_w,
                    height: cur_h,
                    frame_index,
                    spp: settings.spp,
                    cam_pos: [
                        camera.eye[0],
                        camera.eye[1],
                        camera.eye[2],
                        settings.aperture,
                    ],
                    cam_forward: [camera.fwd[0], camera.fwd[1], camera.fwd[2], 0.0],
                    cam_right: [
                        camera.right[0],
                        camera.right[1],
                        camera.right[2],
                        settings.focus_dist,
                    ],
                    cam_up: [
                        camera.up[0],
                        camera.up[1],
                        camera.up[2],
                        camera.tan_half_fov,
                    ],
                    sun_dir: [sun_x, sun_y, sun_z, settings.roughness],
                    fractal_p0,
                    fractal_p1,
                    fractal_p2,
                    fractal_p3,
                    fractal_xform: [
                        settings.obj_scale,
                        settings.rot_x.to_radians(),
                        settings.rot_y.to_radians() + spin_angle,
                        settings.rot_z.to_radians(),
                    ],
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
                    bounces: settings.bounces,
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
                    _pad0: [0; 2],
                    out_motion_f32: rig.motion_f32.cu_deviceptr(),
                    out_depth_f32: rig.depth_f32.cu_deviceptr(),
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
                        rig.diffuse_albedo_f32.cu_deviceptr()
                    } else {
                        0
                    },
                    out_spec_albedo_f32: if rr_export {
                        rig.spec_albedo_f32.cu_deviceptr()
                    } else {
                        0
                    },
                };
                prev_camera = camera;
                rig.params_buf
                    .copy_from_host(&rig.stream, bytemuck::bytes_of(&lp))
                    .expect("params upload");

                let rt_ms = {
                    let _nvtx = crate::nvtx::NvtxRange::new(c"OptiX RT", crate::nvtx::COLOR_RT);
                    let t_rt = Instant::now();
                    // SAFETY: `params_buf` matches `LaunchParams` layout; all output buffers outlive `synchronize`.
                    unsafe {
                        rig.pipeline.launch(
                            &rig.stream,
                            &rig.params_buf,
                            &rig.sbt,
                            [cur_w, cur_h, 1],
                        )
                    }
                    .expect("optix launch");
                    rig.stream.synchronize().expect("rt sync");
                    t_rt.elapsed().as_secs_f32() * 1000.0
                };

                let (mut final_f32_ptr, denoise_ms) = if settings.denoiser_on
                    && !dlss_any
                    && settings.aov_mode == 0
                {
                    let _nvtx =
                        crate::nvtx::NvtxRange::new(c"OptiX Denoise", crate::nvtx::COLOR_DENOISE);
                    let t_den = Instant::now();
                    let flow_img = if rig.denoiser_is_temporal {
                        Some(Image::new(&rig.motion_f32, cur_w, cur_h).expect("flow"))
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
                        let prev_buf = if rig.temporal_has_prev {
                            &rig.prev_denoised_f32
                        } else {
                            &rig.color_f32
                        };
                        Some(Image::new(prev_buf, cur_w, cur_h).expect("prev"))
                    } else {
                        None
                    };
                    let layer = Layer {
                        input: Image::new(&rig.color_f32, cur_w, cur_h).expect("in"),
                        output: Image::new(&rig.denoised_f32, cur_w, cur_h).expect("out"),
                        previous: prev_img,
                        aov: AovType::Beauty,
                    };
                    // SAFETY: all denoiser input/output/guide images outlive `synchronize`.
                    unsafe {
                        rig.denoiser.invoke(
                            &rig.stream,
                            &guides,
                            &[layer],
                            DenoiserParams::default(),
                        )
                    }
                    .expect("denoiser invoke");
                    if rig.denoiser_is_temporal {
                        // SAFETY: `denoised_f32` and `prev_denoised_f32` have identical byte size on `rig.stream`.
                        let cp_res = unsafe {
                            cu::cuMemcpyDtoDAsync_v2(
                                rig.prev_denoised_f32.cu_deviceptr(),
                                rig.denoised_f32.cu_deviceptr(),
                                rig.denoised_f32.num_bytes(),
                                rig.stream.cu_stream(),
                            )
                        };
                        if cp_res == cu::cudaError_enum_CUDA_SUCCESS {
                            rig.temporal_has_prev = true;
                        }
                    }
                    rig.stream.synchronize().expect("den sync");
                    (
                        rig.denoised_f32.cu_deviceptr(),
                        t_den.elapsed().as_secs_f32() * 1000.0,
                    )
                } else {
                    (rig.color_f32.cu_deviceptr(), 0.0)
                };

                let bloom_ms =
                    if settings.bloom_on && !rr_export && settings.aov_mode == 0 {
                        let _nvtx =
                            crate::nvtx::NvtxRange::new(c"cuFFT Bloom", crate::nvtx::COLOR_BLOOM);
                        if let Some(ref mut bloom) = rig.bloom {
                            let bloom_ready = if !settings.denoiser_on || dlss_any {
                            // SAFETY: `color_f32` and `denoised_f32` have identical byte size on `rig.stream`.
                            let cp_res = unsafe {
                                cu::cuMemcpyDtoDAsync_v2(
                                    rig.denoised_f32.cu_deviceptr(),
                                    rig.color_f32.cu_deviceptr(),
                                    rig.color_f32.num_bytes(),
                                    rig.stream.cu_stream(),
                                )
                            };
                            if cp_res == cu::cudaError_enum_CUDA_SUCCESS {
                                final_f32_ptr = rig.denoised_f32.cu_deviceptr();
                                true
                            } else {
                                false
                            }
                        } else {
                            true
                        };
                        if bloom_ready {
                            bloom
                                .apply(
                                    &rig.stream,
                                    final_f32_ptr,
                                    cur_w,
                                    cur_h,
                                    settings.bloom_threshold,
                                    settings.bloom_intensity,
                                    settings.bloom_blades,
                                )
                                .unwrap_or(0.0)
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };

                let tonemap_ms = {
                    let _nvtx =
                        crate::nvtx::NvtxRange::new(c"Tonemap", crate::nvtx::COLOR_TONEMAP);
                    let t_tm = Instant::now();
                    let mut src_ptr = match settings.aov_mode {
                        1 => rig.albedo_f32.cu_deviceptr(),
                        2 => rig.normal_f32.cu_deviceptr(),
                        _ => final_f32_ptr,
                    };
                    let mut dst_ptr = buf_ptrs[buf_idx];
                    let mut w = cur_w;
                    let mut h = cur_h;
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
                    let gx = cur_w.div_ceil(16);
                    let gy = cur_h.div_ceil(16);
                    // SAFETY: `rig.tonemap.func` takes `(float4*, uchar4*, u32, u32, f32, u32, float4*, f32, i32)`
                    // and all pointers/buffers remain valid through `stream.synchronize()`.
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
                    if dlss_any {
                        // SAFETY: `rig.depth_f32` and `depth_ptrs[buf_idx]` have `cur_w * cur_h * 4` bytes;
                        // `rig.motion_f32` and `motion_ptrs[buf_idx]` have `cur_w * cur_h * 8` bytes.
                        let (depth_res, motion_res) = unsafe {
                            let d = cu::cuMemcpyDtoDAsync_v2(
                                depth_ptrs[buf_idx],
                                rig.depth_f32.cu_deviceptr(),
                                rig.depth_f32.num_bytes(),
                                rig.stream.cu_stream(),
                            );
                            let m = cu::cuMemcpyDtoDAsync_v2(
                                motion_ptrs[buf_idx],
                                rig.motion_f32.cu_deviceptr(),
                                rig.motion_f32.num_bytes(),
                                rig.stream.cu_stream(),
                            );
                            (d, m)
                        };
                        if depth_res != cu::cudaError_enum_CUDA_SUCCESS
                            || motion_res != cu::cudaError_enum_CUDA_SUCCESS
                        {
                            pending_status = Some(format!(
                                "DLSS guide copy failed (depth={depth_res:?}, motion={motion_res:?})"
                            ));
                        }
                    }
                    if rr_export
                        && let Some(rr) = rr_ptrs
                    {
                        let mut c_src = rig.color_f32.cu_deviceptr();
                        let mut d_src = rig.diffuse_albedo_f32.cu_deviceptr();
                        let mut s_src = rig.spec_albedo_f32.cu_deviceptr();
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
                        // remain alive through `rig.stream.synchronize()`.
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
                            pending_status =
                                Some(format!("DLSS-RR AOV pack kernel failed ({pack_res:?})"));
                        }
                    }
                    rig.stream.synchronize().expect("tonemap sync");
                    t_tm.elapsed().as_secs_f32() * 1000.0
                };

                if let Some((jpeg_path, quality)) = pending_jpeg.take() {
                    pending_status = Some(
                        match save_jpeg_from_rgba8(
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
                        },
                    );
                }

                if let Some((ref mut rec, rec_fps)) = recorder {
                    let rec_now = Instant::now();
                    let frame_dur = if settings.record_fixed_fps {
                        1.0 / f64::from(rec_fps.max(1))
                    } else {
                        let d = last_record_at.map_or(1.0 / f64::from(rec_fps.max(1)), |prev| {
                            rec_now.duration_since(prev).as_secs_f64()
                        });
                        d.clamp(1.0 / 240.0, 2.0)
                    };
                    last_record_at = Some(rec_now);
                    if let Err(e) = rec.push_frame(
                        &rig.stream,
                        buf_ptrs[buf_idx],
                        final_f32_ptr,
                        settings.exposure,
                        frame_dur,
                    ) {
                        pending_status = Some(format!("Video encode error: {e}"));
                        if let Some((failed, _)) = recorder.take() {
                            let _ = failed.finish();
                        }
                        last_record_at = None;
                    }
                }

                accum_count = accum_count.saturating_add(1);
                frame_index = frame_index.wrapping_add(1);

                if frame_tx
                    .send(FrameReady {
                        buf_idx,
                        gen_id: cur_gen,
                        accum_count,
                        total_samples: accum_count.saturating_mul(settings.spp),
                        jitter,
                        rt_ms,
                        denoise_ms,
                        bloom_ms,
                        tonemap_ms,
                        is_recording: recorder.is_some(),
                        record_frames: recorder.as_ref().map_or(0, |(r, _)| r.frame_count()),
                        ser_supported: rig.ser_supported,
                        status_msg: pending_status.take(),
                    })
                    .is_err()
                {
                    break;
                }
            }
        })
        .expect("spawn optix-render thread");

    (cmd_tx, frame_rx, handle)
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
}
